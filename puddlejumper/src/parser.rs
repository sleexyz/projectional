use std::ops::Range;

use super::diff;
use indexmap::IndexSet;
use serde_json;

#[cfg(all(feature = "native", feature = "wasm"))]
compile_error!("feature \"native\" and feature \"wasm\" cannot be enabled at the same time");

#[cfg(feature = "native")]
use tree_sitter;

#[cfg(feature = "wasm")]
use tree_sitter_c2rust as tree_sitter;

pub struct Parser {
    pub parser: tree_sitter::Parser,
    pub text: String,
    pub tree: tree_sitter::Tree,
}

pub struct Updates {
    pub tree_old: tree_sitter::Tree,
    pub tree_new: tree_sitter::Tree,
}

impl Parser {
    pub fn new(text: String, language: tree_sitter::Language) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .expect("Error loading puddlejumper grammar");
        let tree: tree_sitter::Tree = parser.parse(&text, None).unwrap();
        Self { parser, text, tree }
    }

    pub fn update(&mut self, text_new: String) {
        let text_old = self.text.clone();
        let diff = diff::compute_diff(text_old.as_str(), text_new.as_str());
        for change in diff.changes {
            let text_intermediate = format!(
                "{}{}",
                &text_new[0..change.after_bytes.end],
                &text_old[change.before_bytes.end..]
            );

            &self.tree.edit(&change.input_edit());

            let new_tree = self
                .parser
                .parse(text_intermediate, Some(&self.tree))
                .unwrap();
            self.tree = new_tree;
        }
        self.text = text_new;
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out);
    }

    pub fn get_text(&self, n: tree_sitter::Node) -> &str {
        return &self.text[n.start_byte()..n.end_byte()];
    }
}

fn debug_print(
    node: &tree_sitter::Node,
    code: &str,
    out: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();

        let content = &code[n.start_byte()..n.end_byte()];
        write_indent(out, indent_level)?;
        write!(
            out,
            "{} [{}..{}] [({}, {}) - ({}, {})]       {}\n",
            n.kind(),
            n.start_byte(),
            n.end_byte(),
            n.start_position().row,
            n.start_position().column,
            n.end_position().row,
            n.end_position().column,
            serde_json::to_string(content).unwrap()
        )?;

        // Move to the next node
        if cursor.goto_first_child() {
            indent_level += 1;
            continue;
        }

        // No child nodes, move to the next sibling or parent's next sibling
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return Ok(());
            }
            indent_level -= 1;
        }
    }
}

fn write_indent(out: &mut dyn std::io::Write, indent_level: usize) -> Result<(), std::io::Error> {
    let indent = "    ";
    for _ in 0..indent_level {
        write!(out, "{}", indent)?;
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_noop() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_content() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nwarld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_append() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_change_kind() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_change_kind_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\n  @foo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_delete() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_delete_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_change_multiple() {
        let code1 = String::from(
            r#"hello
  world
foo
  bar"#,
        );
        let code2 = String::from(
            r#"hello
  world
  x
foo
  bar
  y"#,
        );
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }
}

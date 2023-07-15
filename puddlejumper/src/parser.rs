use super::text_diff;
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

pub struct Update {
    pub old_text: String,
    pub old_tree: tree_sitter::Tree,

    pub new_text: String,
    pub new_tree: tree_sitter::Tree,

    pub diff: text_diff::Diff,
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

    pub fn update(&mut self, text_new: String) -> Update {
        let text_old = self.text.clone();
        let tree_old = self.tree.clone();

        let mut tree_new = self.tree.clone();
        let diff = text_diff::compute_diff(text_old.as_str(), text_new.as_str());

        for change in &diff.changes {
            let text_intermediate = format!(
                "{}{}",
                &text_new[0..change.after_bytes.end],
                &text_old[change.before_bytes.end..]
            );
            tree_new.edit(&change.input_edit());
            tree_new = self
                .parser
                .parse(text_intermediate, Some(&tree_new))
                .unwrap();
        }
        Update {
            old_text: text_old,
            old_tree: tree_old,

            new_text: text_new,
            new_tree: tree_new,

            diff,
        }
    }

    // NOTE: the top level node's id will change after the tree is cloned.
    pub fn apply_update(&mut self, update: &Update) {
        self.text = update.new_text.clone();
        self.tree = update.new_tree.clone();
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
            "{} [{}..{}] [({}, {}) - ({}, {})]  ({})       {}\n",
            n.kind(),
            n.start_byte(),
            n.end_byte(),
            n.start_position().row,
            n.start_position().column,
            n.end_position().row,
            n.end_position().column,
            n.id(),
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
        let update = parser.update(code2.clone());
        parser.apply_update(&update);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_edit() {
        let code1 = String::from("halloo\nwooorld");
        let code2 = String::from("hello\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        parser.apply_update(&update);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_append_end() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        parser.apply_update(&update);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }

    #[test]
    fn test_update_append_start() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("foo\nhello\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        parser.apply_update(&update);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }
}

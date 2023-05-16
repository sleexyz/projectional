use tree_sitter::{self, Tree};
use serde_json;

pub struct Parser {
    pub parser: tree_sitter::Parser,
    pub text: String,
    pub tree: tree_sitter::Tree,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_puddlejumper::language()).expect("Error loading puddlejumper grammar");
        let tree: Tree = parser.parse(&text, None).unwrap();
        Self {
            parser,
            text,
            tree,
        }
    }
    pub fn lossless_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return lossless_print(self.tree.root_node(), &self.text, out);
    }

    pub fn pretty_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return pretty_print(self.tree.root_node(), &self.text, out);
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(self.tree.root_node(), &self.text, out);
    }
}

fn debug_print(node: tree_sitter::Node, input: &str, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();

        let content = &input[n.start_byte()..n.end_byte()];
        let start = n.start_position();
        let end = n.end_position();

        write_indent(out, indent_level)?;
        write!(
            out,
            "{} [{}, {}] - [{}, {}]       {}\n",
            n.kind(),
            start.row,
            start.column,
            end.row,
            end.column,
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

fn pretty_print(node: tree_sitter::Node, input: &str, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();
        if n.kind() == "content" {
            write_indent(out, indent_level)?;
            write!(out, "{}\n", &input[n.start_byte()..n.end_byte()])?;
        }

        // Add newline if necessary
        if n.kind() == "children" {
            indent_level += 1;
        }

        // Move to the next node
        if cursor.goto_first_child() {
            continue;
        }

        // No child nodes, move to the next sibling or parent's next sibling
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return Ok(());
            }
            if cursor.node().kind() == "children" {
                indent_level -= 1;
            }
        }
    }
}

fn write_indent(out: &mut dyn std::io::Write,indent_level: usize) -> Result<(), std::io::Error> {
    let indent = "    ";
    for _ in 0..indent_level {
        write!(out, "{}", indent)?;
    }
    return Ok(());
}


fn lossless_print(node: tree_sitter::Node, input: &str, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n.kind() == "newline"
            || n.kind() == "mega_newline"
            || n.kind() == "indent"
            || n.kind() == "dedent"
            || n.kind() == "content"
            || n.kind() == "binding"
            || n.kind() == "ref"
            {
            write!(out, "{}", &input[n.start_byte()..n.end_byte()])?;
        }

        // Move to the next node
        if cursor.goto_first_child() {
            continue;
        }

        // No child nodes, move to the next sibling or parent's next sibling
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lossless_print() {
        let code = String::from("hello\n\n  world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.lossless_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "hello\n\n  world")

    }
    #[test]
    fn test_pretty_print() {
        let code = String::from("hello\n\n  world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "hello\n    world")
    }
}
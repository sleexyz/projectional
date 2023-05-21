use serde_json;
use tree_sitter;

#[derive(Debug)]
pub enum Content {
    Content(String),
    Ref(String),
}

#[derive(Debug)]
pub enum Node<'a> {
    Document {
        t_node: tree_sitter::Node<'a>,
        children: Vec<Node<'a>>,
    },
    Node {
        t_node: tree_sitter::Node<'a>,
        binding: Option<String>,
        content: Option<Content>,
        children: Vec<Node<'a>>,
    },
    Block {
        t_node: tree_sitter::Node<'a>,
        binding: Option<String>,
        header: Box<Node<'a>>,
        children: Vec<Node<'a>>,
    },
}

pub struct Parser {
    pub parser: tree_sitter::Parser,
    pub text: String,
    pub tree: tree_sitter::Tree,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_puddlejumper::language())
            .expect("Error loading puddlejumper grammar");
        let tree: tree_sitter::Tree = parser.parse(&text, None).unwrap();
        Self { parser, text, tree }
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

    pub fn get_text(&self, n: tree_sitter::Node) -> String {
        return self.text[n.start_byte()..n.end_byte()].to_string();
    }

    pub fn load_document(&self) -> Option<Node> {
        return self.load(self.tree.root_node());
    }

    pub fn load<'tree>(&self, t_node: tree_sitter::Node<'tree>) -> Option<Node<'tree>> {
        if t_node.kind() == "document" {
            let mut children: Vec<Node> = Vec::new();
            for child in t_node.children_by_field_name("children", &mut t_node.walk()) {
                self.load(child).map(|node| {
                    children.push(node);
                });
            }
            return Some(Node::Document { t_node, children });
        }
        if t_node.kind() == "node" {
            let binding: Option<String> = t_node
                .child_by_field_name("binding")
                .and_then(|binding: tree_sitter::Node| {
                    return binding.child_by_field_name("identifier");
                })
                .map(|identifier: tree_sitter::Node| {
                    return self.get_text(identifier);
                });
            let content: Option<Content> =
                t_node
                    .child_by_field_name("content")
                    .and_then(|n: tree_sitter::Node| {
                        if n.kind() == "content" {
                            return Some(Content::Content(self.get_text(n)));
                        }
                        if n.kind() == "ref" {
                            return Some(Content::Ref(self.get_text(n)));
                        }
                        return None;
                    });
            let mut children: Vec<Node> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(n).map(|node| {
                            children.push(node);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return Some(Node::Node {
                t_node,
                binding,
                content,
                children,
            });
        }
        if t_node.kind() == "block" {
            let binding: Option<String> = t_node
                .child_by_field_name("binding")
                .and_then(|binding: tree_sitter::Node| {
                    return binding.child_by_field_name("identifier");
                })
                .map(|identifier: tree_sitter::Node| {
                    return self.get_text(identifier);
                });
            let header: Option<Node> =
                t_node
                    .child_by_field_name("header")
                    .and_then(|child: tree_sitter::Node| {
                        return self.load(child);
                    });
            let mut children: Vec<Node> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(n).map(|node| {
                            children.push(node);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return header.map(|header| {
                return Node::Block {
                    t_node,
                    binding,
                    header: Box::new(header),
                    children,
                };
            });
        }
        return None;
    }
}

fn debug_print(
    node: tree_sitter::Node,
    input: &str,
    out: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
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
            start.row + 1, // 1-indexed
            start.column,
            end.row + 1, // 1-indexed
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

fn pretty_print(
    node: tree_sitter::Node,
    input: &str,
    out: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();
        if n.kind() == "content" || n.kind() == "binding" || n.kind() == "ref" {
            write_indent(out, indent_level)?;
            write!(out, "{}\n", &input[n.start_byte()..n.end_byte()])?;
        }
        if n.kind() == "block_header" {
            write_indent(out, indent_level)?;
            write!(out, "# ")?;
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

fn write_indent(out: &mut dyn std::io::Write, indent_level: usize) -> Result<(), std::io::Error> {
    let indent = "    ";
    for _ in 0..indent_level {
        write!(out, "{}", indent)?;
    }
    return Ok(());
}

fn lossless_print(
    node: tree_sitter::Node,
    input: &str,
    out: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n.kind() == "newline"
            || n.kind() == "indent"
            || n.kind() == "dedent"
            || n.kind() == "content"
            || n.kind() == "binding"
            || n.kind() == "ref"
            || n.kind() == "block_header"
        {
            write!(out, "{}", &input[n.start_byte()..n.end_byte()])?;
        } else if cursor.goto_first_child() {
            // Move to the next node
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
    fn test_lossless_print_bindings() {
        let code = String::from("@hello:\n\n\n\n@world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.lossless_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:\n\n\n\n@world")
    }

    #[test]
    fn test_lossless_print_block() {
        let code = String::from("@hello:\n\n#  @world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.lossless_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:\n\n#  @world")
    }

    #[test]
    fn test_pretty_print() {
        let code = String::from("hello\n\n  world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "hello\n    world\n")
    }

    #[test]
    fn test_pretty_print_bindings() {
        let code = String::from("@hello:\n@world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:\n@world\n")
    }

    #[test]
    fn test_pretty_print_block() {
        let code = String::from("@hello:\n\n#  @world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:\n# @world\n")
    }
}

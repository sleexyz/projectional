use super::node::*;
use id_arena::Arena;
use serde_json;
use tree_sitter;

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

    pub fn pretty_print(
        &self,
        out: &mut dyn std::io::Write,
        level: usize,
    ) -> Result<(), std::io::Error> {
        return pretty_print(&self.tree.root_node(), &self.text, out, level);
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out);
    }

    pub fn get_text(&self, n: tree_sitter::Node) -> String {
        return self.text[n.start_byte()..n.end_byte()].to_string();
    }

    pub fn load_document(&self) -> Option<(Context, NodeId)> {
        let mut context = Context {
            arena: Arena::new(),
        };
        let id = self.load(&self.tree.root_node(), &mut context)?;
        return Some((context, id));
    }

    pub fn load(&self, t_node: &tree_sitter::Node, context: &mut Context) -> Option<NodeId> {
        if t_node.kind() == "document" {
            let mut children: Vec<NodeId> = Vec::new();
            for child in t_node.children_by_field_name("children", &mut t_node.walk()) {
                self.load(&child, context).map(|node_id| {
                    children.push(node_id);
                });
            }
            return Some(context.arena.alloc(Node::Document { children }));
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
            let mut children: Vec<NodeId> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(&n, context).map(|node| {
                            children.push(node);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return Some(context.arena.alloc(Node::Node {
                binding,
                content,
                children,
            }));
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
            let header: Option<NodeId> =
                t_node
                    .child_by_field_name("header")
                    .and_then(|child: tree_sitter::Node| {
                        return self.load(&child, context);
                    });
            let mut children: Vec<NodeId> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(&n, context).map(|node_id| {
                            children.push(node_id);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return header.map(|header| {
                return context.arena.alloc(Node::Block {
                    binding,
                    header,
                    children,
                });
            });
        }
        return None;
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
    node: &tree_sitter::Node,
    code: &str,
    out: &mut dyn std::io::Write,
    level: usize,
) -> Result<(), std::io::Error> {
    let mut indent_level = level;
    let mut cursor = node.walk();
    let mut should_indent = false;

    loop {
        let n = cursor.node();
        if n.kind() == "binding" {
            write_indent(out, indent_level)?;
            should_indent = false;
            write!(out, "{}", &code[n.start_byte()..n.end_byte()])?;
        }
        if n.kind() == "content" || n.kind() == "ref" {
            if should_indent {
                write_indent(out, indent_level)?;
            }
            write!(out, "{}\n", &code[n.start_byte()..n.end_byte()])?;
            should_indent = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let code = String::from("hello\n\n  world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output, 0);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "hello\n    world\n")
    }

    #[test]
    fn test_pretty_print_bindings() {
        let code = String::from("@hello:\n@world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output, 0);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:@world\n")
    }

    #[test]
    fn test_pretty_print_block() {
        let code = String::from("@hello:\n\n#  @world");
        let p = Parser::new(code);
        let mut output = Vec::new();
        let result = p.pretty_print(&mut output, 0);
        assert!(result.is_ok());
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output, "@hello:\n# @world\n")
    }
}

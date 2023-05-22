use serde_json;
use tree_sitter;

#[derive(Debug)]
pub enum Content {
    Content(String),
    Ref(String),
}

#[derive(Debug)]
pub enum AST<'a> {
    Document {
        // TODO: rename t_node to data and make it a generic.
        t_node: Option<tree_sitter::Node<'a>>,
        children: Vec<AST<'a>>,
    },
    Node {
        t_node: Option<tree_sitter::Node<'a>>,
        binding: Option<String>,
        content: Option<Content>,
        children: Vec<AST<'a>>,
    },
    Block {
        t_node: Option<tree_sitter::Node<'a>>,
        binding: Option<String>,
        header: Box<AST<'a>>,
        children: Vec<AST<'a>>,
    },
}

pub struct PrintContext<'str, 'out> {
    pub level: usize,
    pub out: &'out mut dyn std::io::Write,
    pub input: &'str str,
    pub needs_indent: bool,
}

impl AST<'_> {
    pub fn t_node(&self) -> &Option<tree_sitter::Node> {
        match self {
            AST::Document { t_node, .. } => t_node,
            AST::Node { t_node, .. } => t_node,
            AST::Block { t_node, .. } => t_node,
        }
    }
    pub fn children(&self) -> &Vec<AST> {
        match self {
            AST::Document { children, .. } => children,
            AST::Node { children, .. } => children,
            AST::Block { children, .. } => children,
        }
    }
    pub fn make_prioritized_list(
        & self,
    ) -> AST {
        let mut priority_nodes: Vec<AST> = Vec::from(["P1", "P2", "P3", "P4", "P5"].map(|p| AST::Node {
            t_node: None,
            binding: None,
            content: Some(Content::Content(p.to_string())),
            children: vec![],
        }));
        let priorities: Vec<(&Self, i32)> = self.extract_priorities();
        for (node, priority) in priorities {
            // self.children().push(*node);
            // Even if we had a refcell, a node can only have a single parent.
            // We could do something like an either type, but perhaps it's better to just
            // use an arena allocator and have the nodes be owned by the arena.
            priority_nodes[priority as usize - 1].children().push(*node);
        }
        AST::Document {
            t_node: None,
            children: priority_nodes
        }
    }

    pub fn pretty_print<'a>(
        &'a self,
        ctx: &mut PrintContext,
    ) -> Result<(), std::io::Error> {
        match self {
            AST::Document { children, .. } => {
                for child in children {
                    child.pretty_print(ctx)?;
                }
            }
            AST::Node {
                content,
                binding,
                children,
                ..
            } => {
                let mut indent = String::new();
                for _ in 0..ctx.level {
                    indent.push_str("  ");
                }
                write!(ctx.out, "{}", indent)?;
                match binding {
                    Some(binding) => {
                        write!(ctx.out, "{}:", binding)?;
                    }
                    None => (),
                }
                match content {
                    Some(Content::Content(content)) => {
                        writeln!(ctx.out, "{}", content)?;
                    }
                    Some(Content::Ref(content)) => {
                        writeln!(ctx.out, "{}", content)?;
                    }
                    None => (),
                }
                for child in children {
                    child.pretty_print(&mut PrintContext {
                        level: ctx.level + 1,
                        out: ctx.out,
                        input: ctx.input,
                        needs_indent: true,
                    })?;
                }
            }
            AST::Block {
                header,
                binding,
                children,
                ..
            } => {
                let mut indent = String::new();
                for _ in 0..ctx.level {
                    indent.push_str("  ");
                }
                match binding {
                    Some(binding) => {
                        writeln!(ctx.out, "{}{}:", indent, binding)?;
                    }
                    None => (),
                }
                header.pretty_print(&mut PrintContext {
                    level: ctx.level,
                    out: ctx.out,
                    input: ctx.input,
                    needs_indent: false,
                })?;
                for child in children {
                    child.pretty_print(&mut PrintContext {
                        level: ctx.level + 1,
                        out: ctx.out,
                        input: ctx.input,
                        needs_indent: true,
                    })?;
                }
            }
        }
        Ok(())
    }
    pub fn extract_priorities(&self) -> Vec<(&Self, i32)> {
        let mut priorities: Vec<(&Self, i32)> = Vec::new();
        self.extract_priorities_rec(&mut priorities);
        return priorities;
    }
    fn extract_priorities_rec<'a>(&'a self, priorities: &mut Vec<(&'a Self, i32)>) {
        match self {
            AST::Document { children, .. } => {
                for child in children {
                    child.extract_priorities_rec(priorities);
                }
            }
            AST::Node { children, .. } => {
                for child in children {
                    match child {
                        AST::Node {
                            content: Some(Content::Content(content)),
                            children,
                            ..
                        } => {
                            let priority: Option<i32> = match content.as_str() {
                                "P0" => Some(0),
                                "P1" => Some(1),
                                "P2" => Some(2),
                                "P3" => Some(3),
                                "P4" => Some(4),
                                _ => None,
                            };
                            match priority {
                                Some(p) => {
                                    // Determine if priority should be applied to parent or child
                                    if children.len() > 0 {
                                        for c in children {
                                            priorities.push((c, p));
                                        }
                                    } else {
                                        priorities.push((&self, p));
                                    }
                                    continue;
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                    child.extract_priorities_rec(priorities);
                }
            }
            _ => return,
        }
    }
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

    pub fn pretty_print(&self, out: &mut dyn std::io::Write, level: usize) -> Result<(), std::io::Error> {
        return pretty_print(&self.tree.root_node(), &self.text, out, level);
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out);
    }

    pub fn get_text(&self, n: tree_sitter::Node) -> String {
        return self.text[n.start_byte()..n.end_byte()].to_string();
    }

    pub fn load_document(&self) -> Option<AST> {
        return self.load(self.tree.root_node());
    }

    pub fn load<'tree>(&self, t_node: tree_sitter::Node<'tree>) -> Option<AST<'tree>> {
        if t_node.kind() == "document" {
            let mut children: Vec<AST> = Vec::new();
            for child in t_node.children_by_field_name("children", &mut t_node.walk()) {
                self.load(child).map(|node| {
                    children.push(node);
                });
            }
            return Some(AST::Document { t_node: Some(t_node), children });
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
            let mut children: Vec<AST> = Vec::new();
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
            return Some(AST::Node {
                t_node: Some(t_node),
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
            let header: Option<AST> =
                t_node
                    .child_by_field_name("header")
                    .and_then(|child: tree_sitter::Node| {
                        return self.load(child);
                    });
            let mut children: Vec<AST> = Vec::new();
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
                return AST::Block {
                    t_node: Some(t_node),
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
    node: &tree_sitter::Node,
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
    node: &tree_sitter::Node,
    input: &str,
    out: &mut dyn std::io::Write,
    level: usize
) -> Result<(), std::io::Error> {
    let mut indent_level = level;
    let mut cursor = node.walk();
    let mut should_indent = false;

    loop {
        let n = cursor.node();
        if n.kind() == "binding" {
            write_indent(out, indent_level)?;
            should_indent = false;
            write!(out, "{}", &input[n.start_byte()..n.end_byte()])?;
        }
        if n.kind() == "content" || n.kind() == "ref" {
            if should_indent {
                write_indent(out, indent_level)?;
            }
            write!(out, "{}\n", &input[n.start_byte()..n.end_byte()])?;
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

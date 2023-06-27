use std::cmp;
use std::{collections::HashMap, time::SystemTime};

use super::diff;
use super::node::*;
use id_arena::Arena;
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

impl Parser {
    pub fn new(text: String, language: tree_sitter::Language) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .expect("Error loading puddlejumper grammar");
        let tree: tree_sitter::Tree = parser.parse(&text, None).unwrap();
        Self { parser, text, tree }
    }

    pub fn perform_edit(&mut self, diff: &diff::Change) -> tree_sitter::InputEdit {
        let start_byte = diff.after_bytes.start as usize;
        let old_end_byte =
            (diff.after_bytes.start + diff.before_bytes.end - diff.before_bytes.start) as usize;
        let new_end_byte = diff.after_bytes.end as usize;

        let start_position = diff.start_position;
        let old_end_position = diff.old_end_position;
        let new_end_position = diff.new_end_position;

        let edit = tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position,
            old_end_position,
            new_end_position,
        };
        self.tree.edit(&edit);
        edit
    }

    pub fn update(&mut self, text_new: String) -> Vec<tree_sitter::Range> {
        let text_old = self.text.clone();
        let diff = diff::compute_diff(text_old.as_str(), text_new.as_str());
        let mut changed_ranges = Vec::new();
        for change in &diff.changes {
            let text_intermediate = format!(
                "{}{}",
                &text_new[0..change.after_bytes.end],
                &text_old[change.before_bytes.end..]
            );

            // Prepare old tree
            self.perform_edit(&change);

            // Parse new tree
            let new_tree = self
                .parser
                .parse(text_intermediate.clone(), Some(&self.tree))
                .unwrap();

            // Compare old and new tree
            let mut new_changed_ranges = self.tree.changed_ranges(&new_tree).collect::<Vec<_>>();
            // let mut new_changed_ranges = get_changed_nodes(&self.tree, false);

            changed_ranges.append(&mut new_changed_ranges.as_mut());

            self.tree = new_tree;
        }
        self.text = text_new;
        return changed_ranges;
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out);
    }

    fn get_text(&self, n: tree_sitter::Node) -> &str {
        return &self.text[n.start_byte()..n.end_byte()];
    }

    pub fn load_document(&self) -> Option<(Context, NodeId)> {
        let mut context = Context {
            arena: Arena::new(),
            metadata: HashMap::new(),
        };
        let id = self.load(&self.tree.root_node(), &mut context)?;
        let now = SystemTime::now();

        for (id, _node) in context.arena.iter() {
            let metadata = NodeMetadata { created_at: now };
            context.metadata.insert(id, metadata);
        }

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
                    return self.get_text(identifier).to_string();
                });
            let content: Option<Content> =
                t_node
                    .child_by_field_name("content")
                    .and_then(|n: tree_sitter::Node| {
                        if n.kind() == "content" {
                            return Some(Content::Content(self.get_text(n).to_string()));
                        }
                        if n.kind() == "ref" {
                            return Some(Content::Ref(self.get_text(n).to_string()));
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
                    return self.get_text(identifier).to_string();
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

    pub fn debug_changes(&self, changes: &Vec<tree_sitter::Range>) -> Vec<&str> {
        let mut result = Vec::new();
        for change in changes {
            result.push(&self.text[change.start_byte..change.end_byte]);
        }
        result
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

// pub fn get_changed_nodes(tree: &tree_sitter::Tree, bubble_up: bool) -> Vec<tree_sitter::Range> {
//     let mut changed_nodes = Vec::new();
//     get_changed_nodes_rec(&tree.root_node(), bubble_up, &mut changed_nodes);
//     return changed_nodes
// }

// pub fn get_changed_nodes_rec(node: &tree_sitter::Node, bubble_up: bool, changed_nodes: &mut Vec<tree_sitter::Range>) -> bool  {
//     if node.child_count() == 0 {
//         if node.has_changes() {
//             changed_nodes.push(node.range());
//             return true;
//         } else {
//             return false;
//         }
//     }
//     let mut cursor = node.walk();
//     let mut children_changed = false;
//     for child in node.named_children(&mut cursor) {
//         children_changed |= get_changed_nodes_rec(&child, bubble_up, changed_nodes);
//     }
//     if bubble_up && children_changed {
//         changed_nodes.push(node.range());
//     }
//     return children_changed;
// }

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
        let changes = parser.update(code2.clone());
        assert_eq!(changes.len(), 0);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    }


    #[test]
    fn test_update_append() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.debug_changes(&changes), vec!["\nfoo"]);
    }

    #[test]
    fn test_update_change_kind() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_changes(&changes), vec!["\n  world"]);
    }

    #[test]
    fn test_update_change_kind_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\n  @foo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_changes(&changes), vec!["\n  world", "\n  @foo"]);
    }

    #[test]
    fn test_update_delete() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(changes.len(), 0);
    }

    #[test]
    fn test_update_delete_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(changes.len(), 0);
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
        let changes = parser.update(code2.clone());
        assert_eq!(parser.debug_changes(&changes), vec!["\n  x", "\n  y"]);
    }
}

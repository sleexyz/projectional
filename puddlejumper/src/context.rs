use super::node::*;
use super::parser::*;
use id_arena::Arena;
use std::{collections::HashMap, time::SystemTime};

impl Context {
    pub fn new() -> Context {
        Context {
            arena: Arena::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn load_document(&mut self, parser: &Parser) -> Option<NodeId> {
        let id = self.load(&parser.tree.root_node(), parser)?;
        let now = SystemTime::now();

        for (id, _node) in self.arena.iter() {
            let metadata = NodeMetadata { created_at: now };
            self.metadata.insert(id, metadata);
        }

        return Some(id);
    }

    pub fn load(&mut self, t_node: &tree_sitter::Node, parser: &Parser) -> Option<NodeId> {
        if t_node.kind() == "document" {
            let mut children: Vec<NodeId> = Vec::new();
            for child in t_node.children_by_field_name("children", &mut t_node.walk()) {
                self.load(&child, parser).map(|node_id| {
                    children.push(node_id);
                });
            }
            return Some(self.arena.alloc(Node::Document { children }));
        }
        if t_node.kind() == "node" {
            let binding: Option<String> = t_node
                .child_by_field_name("binding")
                .and_then(|binding: tree_sitter::Node| {
                    return binding.child_by_field_name("identifier");
                })
                .map(|identifier: tree_sitter::Node| {
                    return parser.get_text(identifier).to_string();
                });
            let content: Option<Content> =
                t_node
                    .child_by_field_name("content")
                    .and_then(|n: tree_sitter::Node| {
                        if n.kind() == "content" {
                            return Some(Content::Content(parser.get_text(n).to_string()));
                        }
                        if n.kind() == "ref" {
                            return Some(Content::Ref(parser.get_text(n).to_string()));
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
                        self.load(&n, parser).map(|node| {
                            children.push(node);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return Some(self.arena.alloc(Node::Node {
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
                    return parser.get_text(identifier).to_string();
                });
            let header: Option<NodeId> =
                t_node
                    .child_by_field_name("header")
                    .and_then(|child: tree_sitter::Node| {
                        return self.load(&child, parser);
                    });
            let mut children: Vec<NodeId> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(&n, parser).map(|node_id| {
                            children.push(node_id);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return header.map(|header| {
                return self.arena.alloc(Node::Block {
                    binding,
                    header,
                    children,
                });
            });
        }
        return None;
    }
}

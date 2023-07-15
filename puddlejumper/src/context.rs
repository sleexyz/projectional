use super::node::*;
use id_arena::Arena;
use std::{collections::HashMap, time::SystemTime};

impl Context {
    pub fn new() -> Context {
        Context {
            now: SystemTime::now(),
            arena: Arena::new(),
            metadata: HashMap::new(),
            lookup_by_ts_node_id: HashMap::new(),
        }
    }

    pub fn get_node_metadata(&self, id: NodeId) -> Option<&NodeMetadata> {
        return self.metadata.get(&id);
    }

    pub fn get_ts_node_id(&self, id: NodeId) -> Option<usize> {
        return self.get_node_metadata(id).and_then(|metadata| {
            return metadata.ts_node_id;
        });
    }

    pub fn load_document<'a>(
        &mut self,
        tree: &'a tree_sitter::Tree,
        text: &'a String,
    ) -> Option<NodeId> {
        let id = self.load(&tree.root_node(), text)?;
        return Some(id);
    }

    pub fn lookup(&self, ts_node_id: usize) -> Option<NodeId> {
        return self.lookup_by_ts_node_id.get(&ts_node_id).map(|id| {
            return *id;
        });
    }

    pub fn make_node(&mut self, node: Node, ts_node: &tree_sitter::Node) -> NodeId {
        let id = self.arena.alloc(node);

        // Insert metadata
        self.metadata.insert(
            id,
            NodeMetadata {
                created_at: self.now,
                updated_at: self.now,
                ts_node_id: Some(ts_node.id()),
            },
        );

        // Insert lookup entry
        self.lookup_by_ts_node_id.insert(ts_node.id(), id);

        return id;
    }

    pub fn load(&mut self, t_node: &tree_sitter::Node, text: &String) -> Option<NodeId> {
        if t_node.kind() == "document" {
            let mut children: Vec<NodeId> = Vec::new();
            for child in t_node.children_by_field_name("children", &mut t_node.walk()) {
                self.load(&child, text).map(|node_id| {
                    children.push(node_id);
                });
            }
            return Some(self.make_node(Node::Document { children }, t_node));
        }
        if t_node.kind() == "node" {
            let binding: Option<String> = t_node
                .child_by_field_name("binding")
                .and_then(|binding: tree_sitter::Node| {
                    return binding.child_by_field_name("identifier");
                })
                .map(|identifier: tree_sitter::Node| {
                    return text[identifier.byte_range()].to_string();
                });
            let content: Option<Content> =
                t_node
                    .child_by_field_name("content")
                    .and_then(|n: tree_sitter::Node| {
                        if n.kind() == "content" {
                            return Some(Content::Content(text[n.byte_range()].to_string()));
                        }
                        if n.kind() == "ref" {
                            return Some(Content::Ref(text[n.byte_range()].to_string()));
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
                        self.load(&n, text).map(|node| {
                            children.push(node);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return Some(self.make_node(
                Node::Node {
                    binding,
                    content,
                    children,
                },
                t_node,
            ));
        }
        if t_node.kind() == "block" {
            let binding: Option<String> = t_node
                .child_by_field_name("binding")
                .and_then(|binding: tree_sitter::Node| {
                    return binding.child_by_field_name("identifier");
                })
                .map(|identifier: tree_sitter::Node| {
                    return text[identifier.byte_range()].to_string();
                });
            let header: Option<NodeId> =
                t_node
                    .child_by_field_name("header")
                    .and_then(|child: tree_sitter::Node| {
                        return self.load(&child, text);
                    });
            let mut children: Vec<NodeId> = Vec::new();
            t_node
                .child_by_field_name("children")
                .map(|child: tree_sitter::Node| {
                    let cursor = &mut child.walk();
                    cursor.goto_first_child();
                    loop {
                        let n = cursor.node();
                        self.load(&n, text).map(|node_id| {
                            children.push(node_id);
                        });
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                });
            return header.map(|header| {
                return self.make_node(
                    Node::Block {
                        binding,
                        header,
                        children,
                    },
                    t_node,
                );
            });
        }
        return None;
    }
}

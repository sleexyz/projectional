use super::*;

impl Context {
    pub fn extract_priorities(&self, node: NodeId) -> Vec<(NodeId, i32)> {
        let mut priorities: Vec<(NodeId, i32)> = Vec::new();
        self.extract_priorities_rec(node, &mut priorities);
        return priorities;
    }

    fn extract_priorities_rec(&self, node: NodeId, priorities: &mut Vec<(NodeId, i32)>) {
        match &self.arena[node] {
            Node::Document { children, .. } => {
                for child in children {
                    self.extract_priorities_rec(*child, priorities);
                }
            }
            Node::Node { children, .. } => {
                for child in children {
                    match &self.arena[*child] {
                        Node::Node {
                            content: Some(Content::Content(content)),
                            children,
                            ..
                        } => {
                            match content.as_str() {
                                "P0" => Some(0),
                                "P1" => Some(1),
                                "P2" => Some(2),
                                "P3" => Some(3),
                                "P4" => Some(4),
                                _ => None,
                            }
                            .map(|p| {
                                // Determine if priority should be applied to parent or child
                                if children.len() > 0 {
                                    for c in children {
                                        priorities.push((*c, p));
                                    }
                                } else {
                                    priorities.push((node, p));
                                }
                            });
                        }
                        _ => {}
                    }
                    self.extract_priorities_rec(*child, priorities);
                }
            }
            _ => return,
        }
    }
    pub fn make_prioritized_list(&mut self, node: NodeId) -> NodeId {
        let mut priority_nodes: [Vec<NodeId>; 5] = [(); 5].map(|_| vec![]);
        let priorities: Vec<(NodeId, i32)> = self.extract_priorities(node);
        for (node, priority) in priorities {
            priority_nodes[priority as usize].push(node);
        }
        let list = Node::Document {
            children: priority_nodes
                .iter()
                .enumerate()
                .map(|(index, nodes)| {
                    if nodes.len() == 0 {
                        return None;
                    }
                    Some(self.arena.alloc(Node::Node {
                        binding: None,
                        content: Some(Content::Content(format!("P{}", index))),
                        children: nodes.to_vec(),
                    }))
                })
                .filter_map(|x| x)
                .collect(),
        };
        self.arena.alloc(list)
    }
}

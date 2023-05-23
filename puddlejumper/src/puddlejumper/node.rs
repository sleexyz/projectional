use id_arena::{Arena, Id};

#[derive(Debug)]
pub enum Content {
    Content(String),
    Ref(String),
}

type ASTIdWith<T> = Id<ASTWith<T>>;

pub type AST = ASTWith<()>;
pub type ASTId = ASTIdWith<()>;

#[derive(Debug)]
pub enum ASTWith<T> {
    Document {
        data: T,
        children: Vec<ASTIdWith<T>>,
    },
    Node {
        data: T,
        binding: Option<String>,
        content: Option<Content>,
        children: Vec<ASTIdWith<T>>,
    },
    Block {
        data: T,
        binding: Option<String>,
        header: ASTIdWith<T>,
        children: Vec<ASTIdWith<T>>,
    },
}

#[derive(Debug)]
pub struct Context {
    pub arena: Arena<AST>,
}

pub struct PrintContext<'out> {
    pub level: usize,
    pub out: &'out mut dyn std::io::Write,
    pub needs_indent: bool,
}

impl Context {
    pub fn make_prioritized_list(&mut self, node: ASTId) -> ASTId {
        let mut priority_nodes: [Vec<ASTId>; 5] = [(); 5].map(|_| vec![]);
        let priorities: Vec<(ASTId, i32)> = self.extract_priorities(node);
        for (node, priority) in priorities {
            priority_nodes[priority as usize].push(node);
        }
        let list = AST::Document {
            data: (),
            children: priority_nodes
                .iter()
                .enumerate()
                .map(|(index, nodes)| {
                    self.arena.alloc(AST::Node {
                        data: (),
                        binding: None,
                        content: Some(Content::Content(format!("P{}", index))),
                        children: nodes.to_vec(),
                    })
                })
                .collect(),
        };
        self.arena.alloc(list)
    }

    pub fn pretty_print<'a>(
        &'a self,
        node: ASTId,
        ctx: &mut PrintContext,
    ) -> Result<(), std::io::Error> {
        match &self.arena[node] {
            AST::Document { children, .. } => {
                for child in children {
                    self.pretty_print(*child, ctx)?;
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
                    indent.push_str("    ");
                }
                write!(ctx.out, "{}", indent)?;
                match binding {
                    Some(binding) => {
                        write!(ctx.out, "@{}:", binding)?;
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
                    self.pretty_print(
                        *child,
                        &mut PrintContext {
                            level: ctx.level + 1,
                            out: ctx.out,
                            needs_indent: true,
                        },
                    )?;
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
                self.pretty_print(
                    *header,
                    &mut PrintContext {
                        level: ctx.level,
                        out: ctx.out,
                        needs_indent: false,
                    },
                )?;
                for child in children {
                    self.pretty_print(
                        *child,
                        &mut PrintContext {
                            level: ctx.level + 1,
                            out: ctx.out,
                            needs_indent: true,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }
    pub fn extract_priorities(&self, node: ASTId) -> Vec<(ASTId, i32)> {
        let mut priorities: Vec<(ASTId, i32)> = Vec::new();
        self.extract_priorities_rec(node, &mut priorities);
        return priorities;
    }
    fn extract_priorities_rec<'a>(&'a self, node: ASTId, priorities: &mut Vec<(ASTId, i32)>) {
        match &self.arena[node] {
            AST::Document { children, .. } => {
                for child in children {
                    self.extract_priorities_rec(*child, priorities);
                }
            }
            AST::Node { children, .. } => {
                for child in children {
                    match &self.arena[*child] {
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
                                            priorities.push((*c, p));
                                        }
                                    } else {
                                        priorities.push((node, p));
                                    }
                                    continue;
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                    self.extract_priorities_rec(*child, priorities);
                }
            }
            _ => return,
        }
    }
}


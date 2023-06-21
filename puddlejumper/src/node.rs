pub mod printer;
pub mod extract_priorities;

use id_arena::{Arena, Id};

#[derive(Debug)]
pub enum Content {
    Content(String),
    Ref(String),
}

pub type NodeId = Id<Node>;

#[derive(Debug)]
pub enum Node {
    Document {
        children: Vec<NodeId>,
    },
    Node {
        binding: Option<String>,
        content: Option<Content>,
        children: Vec<NodeId>,
    },
    Block {
        binding: Option<String>,
        header: NodeId,
        children: Vec<NodeId>,
    },
}

#[derive(Debug)]
pub struct Context {
    pub arena: Arena<Node>,
}
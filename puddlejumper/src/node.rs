pub mod printer;
pub mod extract_priorities;

use std::{collections::HashMap, time::SystemTime, cell::*, rc::Rc};
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
    pub now: SystemTime,
    pub arena: Arena<Node>,
    pub metadata: HashMap<NodeId, NodeMetadata>,
    pub lookup_by_ts_node_id: HashMap<usize, NodeId>,
}

#[derive(Debug, Clone)]
pub struct NodeMetadata {
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub ts_node_id: Option<usize>,
}

impl Node {
    pub fn children(&self) -> &Vec<NodeId> {
        match self {
            Node::Document { children } => children,
            Node::Node { children, .. } => children,
            Node::Block { children, .. } => children,
        }
    }
    pub fn kind(&self) -> &str {
        match self {
            Node::Document { .. } => "document",
            Node::Node { .. } => "node",
            Node::Block { .. } => "block",
        }
    }
}

// TODO: consolidate with NodeCursor 
pub struct NodeCursorImmutable<'a> {
    pub root_id: NodeId,
    pub stack: Vec<(NodeId, usize)>,
    pub context: &'a Context,
}

impl NodeCursorImmutable<'_> {
    pub fn new(context: &Context, root_id: NodeId) -> NodeCursorImmutable {
        NodeCursorImmutable {
            root_id,
            stack: vec![],
            context,
        }
    }
}

impl std::iter::Iterator for NodeCursorImmutable<'_> {
    type Item = NodeId;

    // Pre-order traversal
    fn next(&mut self) -> Option<Self::Item> {
        let (node_id, i) = match self.stack.last() {
            Some(&(node_id, i)) => (node_id, i),
            None => {
                self.stack.push((self.root_id, 0));
                return Some(self.root_id);
            }
        };
        let node = &self.context.arena[node_id];

        // Get first child
        if let Some(&child_id) = node.children().get(0) {
            self.stack.push((child_id, 0));
            return Some(child_id);
        }

        loop {
            self.stack.pop();

            // Get next sibling
            let (node_id, _) = *self.stack.last()?;
            let node = &self.context.arena[node_id];

            if let Some(&next_sibling_id) = node.children().get(i + 1) {
                self.stack.pop();
                self.stack.push((next_sibling_id, i + 1));
                return Some(next_sibling_id);
            }
        }
    }
}

pub struct NodeCursor {
    pub root_id: NodeId,
    pub stack: Vec<(NodeId, usize)>,
    pub context: Rc<RefCell<Context>>,
}

impl NodeCursor {
    pub fn new(context: Rc<RefCell<Context>>, root_id: NodeId) -> NodeCursor {
        NodeCursor {
            root_id,
            stack: vec![],
            context,
        }
    }
}

impl std::iter::Iterator for NodeCursor {
    type Item = NodeId;

    // Pre-order traversal
    fn next(&mut self) -> Option<Self::Item> {
        let (node_id, i) = match self.stack.last() {
            Some(&(node_id, i)) => (node_id, i),
            None => {
                self.stack.push((self.root_id, 0));
                return Some(self.root_id);
            }
        };
        let node = &self.context.borrow().arena[node_id];

        // Get first child
        if let Some(&child_id) = node.children().get(0) {
            self.stack.push((child_id, 0));
            return Some(child_id);
        }

        loop {
            self.stack.pop();

            // Get next sibling
            let (node_id, _) = *self.stack.last()?;
            let node = &self.context.borrow().arena[node_id];

            if let Some(&next_sibling_id) = node.children().get(i + 1) {
                self.stack.pop();
                self.stack.push((next_sibling_id, i + 1));
                return Some(next_sibling_id);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::{parser::Parser, node::printer::PrintContext};

    #[test]
    fn test_node_iterator_basic() {
        let code = String::from("hello\n  x\nworld\n  y");
        let parser = Parser::new(code.clone(), tree_sitter_puddlejumper::language());
        let (context, id) =  {
            let mut context = Context::new();
            let id = context.load_document(&parser.tree, &parser.text).unwrap();
            (context, id)
        };
        let context = Rc::new(RefCell::new(context));
        let cursor = NodeCursor::new(context.clone(), id);

        let mut out = vec![];
        let mut pc = PrintContext::new(&mut out);
        for node_id in cursor {
            pc.pretty_print(node_id, &context.borrow()).unwrap();
        }
        assert_eq!(std::str::from_utf8(&out).unwrap(), r#"hello
    x
world
    y
hello
    x
x
world
    y
y
"#);
    }
}
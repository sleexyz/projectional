use core::fmt;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use super::levenshtein::*;
use super::parser::*;
use tree_sitter::Node;
use tree_sitter::Tree;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum OpF<T> {
    // Content is identical.
    Exact { old: T, new: T },

    // Descendant content is different.
    Update { old: T, new: T },

    // Content is different.
    UpdateScalar { old: T, new: T },

    // Node was deleted. Descendants might still be present.
    Delete { old: T },

    //  Node was inserted
    DeleteSubtree { old: T },

    // Node was inserted.
    Insert { new: T },

    // Node was inserted.
    InsertSubtree { new: T },
}

impl<T> OpF<T> {
    pub fn map<U>(&self, fun: &impl Fn(&T) -> U) -> OpF<U> {
        match &self {
            OpF::Exact { old, new } => OpF::Exact {
                old: fun(old),
                new: fun(new),
            },
            OpF::Update { old, new } => OpF::Update {
                old: fun(old),
                new: fun(new),
            },
            OpF::UpdateScalar { old, new } => OpF::UpdateScalar {
                old: fun(old),
                new: fun(new),
            },
            OpF::Delete { old } => OpF::Delete { old: fun(old) },
            OpF::DeleteSubtree { old } => OpF::DeleteSubtree { old: fun(old) },
            OpF::Insert { new } => OpF::Insert { new: fun(new) },
            OpF::InsertSubtree { new } => OpF::InsertSubtree { new: fun(new) },
        }
    }
}

impl<'a> OpF<Node<'a>> {
    pub fn cost(&self, context: &UpdateContext) -> usize {
        match &self {
            OpF::Exact { .. } => 0,
            OpF::UpdateScalar { old, new } => levenshtein(
                &context.update.old_text[old.byte_range()],
                &context.update.new_text[new.byte_range()],
            ),
            OpF::DeleteSubtree { old } => context.old_data[old].base_cost_subtree * 4,
            OpF::InsertSubtree { new, .. } => context.new_data[new].base_cost_subtree * 2,
            OpF::Update { .. } => 0,
            OpF::Delete { old } => base_cost(old) * 4,
            OpF::Insert { new } => base_cost(new) * 2,
        }
    }

    pub fn reconciliation(&self) -> Option<(usize, usize)> {
        match &self {
            OpF::Exact { old, new } => Some((old.id(), new.id())),
            OpF::Update{ old, new } => Some((old.id(), new.id())),
            OpF::UpdateScalar{ old, new } => Some((old.id(), new.id())),
            _ => None,
        }
    }

    pub fn source_node_id(&self) -> usize {
        match &self {
            OpF::Exact { old, .. } => old.id(),
            OpF::Update { old, .. } => old.id(),
            OpF::UpdateScalar { old, .. } => old.id(),
            OpF::Delete { old } => old.parent().unwrap().id(),
            OpF::DeleteSubtree { old } => old.parent().unwrap().id(),
            OpF::Insert { new } => new.parent().unwrap().id(),
            OpF::InsertSubtree { new } => new.parent().unwrap().id(),
        }
    }

    pub fn map_with_type<T>(&self, fun: &impl Fn(&Node<'a>, bool) -> T) -> OpF<T> {
        match &self {
            OpF::Exact { old, new } => OpF::Exact {
                old: fun(old, false),
                new: fun(new, true),
            },
            OpF::Update { old, new } => OpF::Update {
                old: fun(old, false),
                new: fun(new, true),
            },
            OpF::UpdateScalar { old, new } => OpF::UpdateScalar {
                old: fun(old, false),
                new: fun(new, true),
            },
            OpF::Delete { old } => OpF::Delete {
                old: fun(old, false),
            },
            OpF::DeleteSubtree { old } => OpF::DeleteSubtree {
                old: fun(old, false),
            },
            OpF::Insert { new } => OpF::Insert {
                new: fun(new, true),
            },
            OpF::InsertSubtree { new } => OpF::InsertSubtree {
                new: fun(new, true),
            },
        }
    }

    pub fn to_hunks_with_kind(&self, context: &'a UpdateContext) -> OpF<(&'a str, &'static str)> {
        self.map_with_type(&|node, is_new| {
            if is_new {
                (&context.update.new_text[node.byte_range()], node.kind())
            } else {
                (&context.update.old_text[node.byte_range()], node.kind())
            }
        })
    }

    pub fn to_hunks(&self, context: &'a UpdateContext) -> OpF<&'a str> {
        self.to_hunks_with_kind(context).map(&|x| x.0)
    }
}

pub type Op<'a> = OpF<Node<'a>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NodeData {
    pub hash: u32,
    pub size_bytes: usize,
    pub base_cost_subtree: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cursor<T> {
    Before(T),
    After(T),
}

impl<'a> Cursor<Node<'a>> {
    pub fn node(&self) -> Node<'a> {
        match self {
            Cursor::Before(node) => *node,
            Cursor::After(node) => *node,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Cursor::Before(node) => {
                if let Some(node) = node.child(0) {
                    return Cursor::Before(node);
                }
                return Cursor::After(node);
            }
            Cursor::After(node) => {
                if let Some(node) = node.next_sibling() {
                    return Cursor::Before(node);
                }
                if let Some(node) = node.parent() {
                    return Cursor::After(node);
                }
                assert_eq!(node.kind(), "document");
                return Cursor::After(node);
            }
        }
    }
    // TODO: make this idempotent
    pub fn normalize(self) -> Self {
        match self {
            Cursor::After(node) => {
                if let Some(node) = node.next_sibling() {
                    return Cursor::Before(node);
                }
                if let Some(node) = node.parent() {
                    return Cursor::After(node);
                }
                assert_eq!(node.kind(), "document");
                return Cursor::After(node);

                // if node.parent().is_none() {
                //     assert_eq!(node.kind(), "document");
                //     return Cursor::After(node);
                // }
                // node = node.parent().unwrap();
            }
            x => x,
        }
    }
}

pub struct UpdateContext<'a> {
    // TODO: deprecate
    pub update: &'a Update,
    pub old_tree: &'a Tree,
    pub old_data: HashMap<Node<'a>, NodeData>,
    pub new_data: HashMap<Node<'a>, NodeData>,
    pub debug_info: HashMap<OpPtr<'a>, &'static str>,
    pub search_cache: SearchCache<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OpPtr<'a>(Rc<Op<'a>>);

impl Hash for OpPtr<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(&**self, state);
    }
}

impl<'a> Deref for OpPtr<'a> {
    type Target = Op<'a>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl OpPtr<'_> {
    pub fn raw_ptr(&self) -> *const Op<'_> {
        &*self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SearchData<'a> {
    pub old: Cursor<Node<'a>>,
    pub new: Cursor<Node<'a>>,
    pub cost: usize,
    pub path: Vec<OpPtr<'a>>,
}

impl PartialOrd for SearchData<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // BinaryHeap is a max heap, so we want to reverse the ordering
        Some(Reverse(self.cost).cmp(&Reverse(other.cost)))
    }
}

impl Ord for SearchData<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max heap, so we want to reverse the ordering
        Reverse(self.cost).cmp(&Reverse(other.cost))
    }
}

pub struct SearchDataDebug<'a> {
    pub data: &'a SearchData<'a>,
    pub context: &'a UpdateContext<'a>,
}

impl Debug for SearchDataDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchData")
            .field(
                "old",
                &(
                    self.data.old,
                    &self.context.update.old_text[self.data.old.node().byte_range()],
                ),
            )
            .field(
                "new",
                &(
                    self.data.new,
                    &self.context.update.new_text[self.data.new.node().byte_range()],
                ),
            )
            .field("cost", &self.data.cost)
            .field(
                "path",
                &self
                    .data
                    .path
                    .iter()
                    // .map(|x| x.to_hunks_with_kind(self.context))
                    .map(|x| {
                        (
                            x.to_hunks_with_kind(self.context),
                            self.context.debug_info[x],
                            x.raw_ptr(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

pub struct SearchCache<'a> {
    pub frontier: BinaryHeap<Rc<SearchData<'a>>>,
    pub map: HashMap<(Cursor<Node<'a>>, Cursor<Node<'a>>), Rc<SearchData<'a>>>,
}

impl<'a> UpdateContext<'a> {
    pub fn new(update: &'a Update, old_tree: &'a Tree) -> Self {
        let old_data = get_content_hashes(old_tree, &update.old_text);
        let new_data = get_content_hashes(&update.new_tree, &update.new_text);
        let mut context = UpdateContext {
            update: &update,
            old_tree,
            old_data,
            new_data,
            search_cache: SearchCache {
                frontier: BinaryHeap::new(),
                map: HashMap::new(),
            },
            debug_info: HashMap::new(),
        };
        context.compute_diff();
        context
    }

    pub fn get_root_diff_debug(&self) -> Vec<OpF<&str>> {
        self.get_root_diff()
            .unwrap()
            .iter()
            .map(|c| c.to_hunks(&self))
            .collect::<Vec<_>>()
    }

    pub fn get_root_diff_debug_verbose(
        &self,
    ) -> Vec<(OpF<(&str, &str)>, &'static str, *const Op)> {
        self.get_root_diff()
            .unwrap()
            .iter()
            .map(|c| (c.to_hunks_with_kind(&self), self.debug_info[c], c.raw_ptr()))
            .collect::<Vec<_>>()
    }

    pub fn get_root_diff(&self) -> Option<&Vec<OpPtr<'a>>> {
        self.search_cache
            .map
            .get(&(
                Cursor::After(self.old_tree.root_node()),
                Cursor::After(self.update.new_tree.root_node()),
            ))
            .map(|data| &data.path)
    }

    /* Returns a map from target node (new) to source node (old) */
    pub fn get_root_diff_reconciliations_by_target_node(&self) -> Option<HashMap<usize, OpPtr<'a>>> {
        let mut result = HashMap::new();
        for op in self.get_root_diff()? {
            if let Some((_, new)) = op.reconciliation() {
                result.insert(new, op.clone());
            }
        }
        Some(result)
    }

    pub fn get_root_diff_by_source_node(&self) -> Option<HashMap<usize, Vec<OpPtr<'a>>>> {
        let mut result = HashMap::new();
        for op in self.get_root_diff()? {
            let ts_node_id = op.source_node_id();
            result.entry(ts_node_id).or_insert_with(Vec::new).push(op.clone());
        }
        Some(result)
    }

    pub fn make_op(&mut self, op: OpF<Node<'a>>, reason: &'static str) -> OpPtr<'a> {
        let op_ptr = OpPtr(Rc::new(op));
        self.debug_info.insert(op_ptr.clone(), reason);
        op_ptr
    }

    pub fn compute_diff(&mut self) {
        let old_root = self.old_tree.root_node();
        let new_root = self.update.new_tree.root_node();
        let data = SearchData {
            old: Cursor::Before(old_root),
            new: Cursor::Before(new_root),
            cost: 0,
            path: vec![],
        };
        self.search_cache.frontier.push(data.into());

        while let Some(vertex) = self.search_cache.frontier.pop() {
            self.find_change_step(vertex);
            if self
                .search_cache
                .map
                .contains_key(&(Cursor::After(old_root), Cursor::After(new_root)))
            {
                break;
            }
        }
    }

    pub fn search_data_debug(&'a self, data: &'a SearchData<'a>) -> SearchDataDebug<'a> {
        SearchDataDebug {
            data,
            context: self,
        }
    }

    // Adds a vertex to the frontier.
    pub fn add_to_frontier(&mut self, data: Rc<SearchData<'a>>) -> bool {
        if let Some(existing_data) = self.search_cache.map.get(&(data.old, data.new)) {
            if existing_data.cost <= data.cost {
                println!(
                    ">> Not adding to frontier (cost: {} >= existing_cost {}): {:#?}",
                    data.cost,
                    existing_data.cost,
                    self.search_data_debug(&data)
                );
                return false;
            }
        }
        println!(
            ">> Adding to frontier: {:#?}",
            self.search_data_debug(&data)
        );
        self.search_cache
            .map
            .insert((data.old, data.new), data.clone());
        self.search_cache.frontier.push(data.clone());
        return true;
    }

    pub fn find_change_step(&mut self, data: Rc<SearchData<'a>>) {
        // println!("find_change_step: {:#?}", self.search_data_debug(&data));
        match *data {
            SearchData {
                old: Cursor::Before(old_node),
                new: Cursor::Before(new_node),
                ..
            } => {
                let NodeData { hash: old_hash, .. } = self.old_data[&old_node];
                let NodeData { hash: new_hash, .. } = self.new_data[&new_node];
                // No change
                if old_hash == new_hash {
                    let op = self.make_op(
                        Op::Exact {
                            old: old_node,
                            new: new_node,
                        },
                        "Exact",
                    );

                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node).normalize(),
                            new: Cursor::After(new_node).normalize(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                    return;
                }

                // Scalar update
                if old_node.child_count() == 0
                    && new_node.child_count() == 0
                    && is_compatible_for_scalar_update(old_node, new_node)
                {
                    let op = self.make_op(
                        Op::UpdateScalar {
                            old: old_node,
                            new: new_node,
                        },
                        "Scalar update",
                    );
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node).normalize(),
                            new: Cursor::After(new_node).normalize(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                    return;
                }

                // Descend both:
                // (x y); (x z)
                // ^^   ; ^^
                match (old_node.child(0), new_node.child(0)) {
                    (Some(old_child), Some(new_child)) if old_child.kind() == new_child.kind() => {
                        let op = self.make_op(
                            Op::Update {
                                old: old_node,
                                new: new_node,
                            },
                            "Descend both",
                        );
                        let cost = data.cost + op.cost(&self);
                        self.add_to_frontier(
                            SearchData {
                                old: Cursor::Before(old_child).normalize(),
                                new: Cursor::Before(new_child).normalize(),
                                cost,
                                path: [data.path.clone(), vec![op]].concat(),
                            }
                            .into(),
                        );

                        if old_node.kind() == new_node.kind() && old_node.kind() == "document" {
                            return;
                        }
                    }
                    _ => {}
                }

                // Move left. Left novel, so delete whole subtree.
                // (x z) ; (z)
                //  ^ ^  ;  ^
                {
                    let op = self.make_op(
                        Op::DeleteSubtree { old: old_node },
                        "Delete left, move left",
                    );
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node).normalize(),
                            new: Cursor::Before(new_node),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }

                // Move right. Right novel, so insert whole subtree.
                // (z) ; (x z)
                //  ^  ;  ^ ^
                {
                    let op = self.make_op(
                        Op::InsertSubtree { new: new_node },
                        "Insert right, move right",
                    );
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::Before(old_node),
                            new: Cursor::After(new_node).normalize(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }

                // Descend left. Left novel, so delete parent.
                // (a (x z)); (x z)
                // ^^       ; ^
                if let Some(old_child) = old_node.child(0) {
                    let op = self.make_op(Op::Delete { old: old_node }, "Descend left");
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::Before(old_child).normalize(),
                            new: Cursor::Before(new_node).normalize(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }

                // Descend right. Right novel, so insert parent.
                // (x z); (a (x z))
                // ^    ; ^^
                if let Some(new_child) = new_node.child(0) {
                    let op = self.make_op(Op::Insert { new: new_node }, "Descend right");
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::Before(old_node),
                            new: Cursor::Before(new_child),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }
            }
            SearchData {
                old: Cursor::After(old_node),
                new: Cursor::Before(new_node),
                ..
            } => {
                // Delete new subtree
                {
                    let op = self.make_op(
                        Op::InsertSubtree { new: new_node },
                        "Left traversed, insert left subtree",
                    );
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node),
                            new: Cursor::After(new_node).next(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }
                // Insert new node
                {
                    let op =
                        self.make_op(Op::Insert { new: new_node }, "Left traversed, insert right");
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node).normalize(),
                            new: Cursor::Before(new_node).next(), // Go to next node
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }
                // Pop left
                self.add_to_frontier(
                    SearchData {
                        old: Cursor::After(old_node).next(),
                        new: Cursor::Before(new_node),
                        cost: data.cost,
                        path: data.path.clone(),
                    }
                    .into(),
                );
            }
            SearchData {
                old: Cursor::Before(old_node),
                new: Cursor::After(new_node),
                ..
            } => {
                {
                    // Delete old subtree
                    let op = self.make_op(
                        Op::DeleteSubtree { old: old_node },
                        "Right traversed, delete left subtree",
                    );
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::After(old_node).next(),
                            new: Cursor::After(new_node),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }
                {
                    // Delete old node
                    let op =
                        self.make_op(Op::Delete { old: old_node }, "Right traversed, delete left");
                    self.add_to_frontier(
                        SearchData {
                            old: Cursor::Before(old_node).next(),
                            new: Cursor::After(new_node).normalize(),
                            cost: data.cost + op.cost(&self),
                            path: [data.path.clone(), vec![op]].concat(),
                        }
                        .into(),
                    );
                }
                self.add_to_frontier(
                    SearchData {
                        old: Cursor::Before(old_node),
                        new: Cursor::After(new_node).next(),
                        cost: data.cost,
                        path: data.path.clone(),
                    }
                    .into(),
                );
            }
            SearchData {
                old: Cursor::After(old_node),
                new: Cursor::After(new_node),
                ..
            } => {
                self.add_to_frontier(
                    SearchData {
                        old: Cursor::After(old_node).normalize(),
                        new: Cursor::After(new_node).normalize(),
                        cost: data.cost,
                        path: data.path.clone(),
                    }
                    .into(),
                );
            }
        }
    }
}

pub fn is_compatible_for_scalar_update(old: Node, new: Node) -> bool {
    match (old.kind(), new.kind()) {
        (a, b) if a == b => true,
        _ => false,
    }
}

pub fn get_content_hashes<'a>(tree: &'a Tree, text: &str) -> HashMap<Node<'a>, NodeData> {
    let mut map = HashMap::new();
    get_content_hashes_rec(tree.root_node(), text, &mut map);
    map
}

pub fn base_cost<'a>(node: &Node<'a>) -> usize {
    match node.kind() {
        "node" => 1,
        "binding" => 1,
        "ref" => 1,
        "block" => 1,
        _ => 0,
    }
}
pub fn get_content_hashes_rec<'a>(
    node: Node<'a>,
    text: &str,
    map: &mut HashMap<Node<'a>, NodeData>,
) -> NodeData {
    let mut size = 0;
    let mut size_nodes = base_cost(&node);
    let mut hasher = DefaultHasher::new();

    node.kind().hash(&mut hasher);

    if node.child_count() == 0 {
        text[node.byte_range()].hash(&mut hasher);
        size += node.byte_range().len();
    } else {
        let mut cursor = &mut node.walk();
        for child in node.children(&mut cursor) {
            let data = get_content_hashes_rec(child, text, map);
            hasher.write_u32(data.hash);
            size += data.size_bytes;
            size_nodes += data.base_cost_subtree;
        }
    }

    let hash = hasher.finish() as u32;
    let data = NodeData {
        hash,
        size_bytes: size,
        base_cost_subtree: size_nodes,
    };
    map.insert(node, data);
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_noop() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![OpF::Exact {
                old: "hello\nworld",
                new: "hello\nworld",
            },]
        );
    }

    #[test]
    fn test_diff_content() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nwarld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld",
                    new: "hello\nwarld"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Update {
                    old: "world",
                    new: "warld"
                },
                OpF::UpdateScalar {
                    old: "world",
                    new: "warld"
                },
            ]
        );
    }

    #[test]
    fn test_diff_append() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld",
                    new: "hello\nworld\nfoo"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
                OpF::InsertSubtree { new: "foo" },
            ]
        );
    }

    #[test]
    fn test_diff_delete_start() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld",
                    new: "world",
                },
                OpF::DeleteSubtree { old: "hello" },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
            ]
        );
    }

    #[test]
    fn test_diff_delete_end() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld",
                    new: "hello",
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::DeleteSubtree { old: "world" },
            ]
        );
    }

    #[test]
    fn test_diff_delete_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld\nfoo",
                    new: "world",
                },
                OpF::DeleteSubtree { old: "hello" },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
                OpF::DeleteSubtree { old: "foo" },
            ]
        );
    }

    #[test]
    fn test_diff_change_kind_single() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\n  world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld",
                    new: "hello\n  world",
                },
                OpF::Update {
                    old: "hello",
                    new: "hello\n  world"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Insert { new: "\n  world" },
                OpF::Exact {
                    old: "world",
                    new: "world"
                }
            ]
        );
    }

    #[test]
    fn test_diff_change_kind_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\n  @foo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\nworld\nfoo",
                    new: "hello\n  world\n  @foo",
                },
                OpF::Update {
                    old: "hello",
                    new: "hello\n  world\n  @foo"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Insert {
                    new: "\n  world\n  @foo"
                },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
                OpF::InsertSubtree { new: "@foo" },
                OpF::DeleteSubtree { old: "foo" },
            ]
        );
    }

    #[test]
    fn test_diff_insert() {
        let code1 = String::from(
            r#"hello
    world"#,
        );
        let code2 = String::from(
            r#"hello
    world
    x"#,
        );
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        println!("{:#?}", update_context.get_root_diff_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\n    world",
                    new: "hello\n    world\n    x"
                },
                OpF::Update {
                    old: "hello\n    world",
                    new: "hello\n    world\n    x"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Update {
                    old: "\n    world",
                    new: "\n    world\n    x"
                },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
                OpF::InsertSubtree { new: "x" },
            ]
        );
    }

    #[test]
    fn test_diff_insert2() {
        let code1 = String::from(
            r#"hello
    world
foo"#,
        );
        let code2 = String::from(
            r#"hello
    world
    x
foo"#,
        );
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);
        // println!("{:#?}", update_context.get_root_change_path_debug_verbose());
        assert_eq!(
            update_context.get_root_diff_debug(),
            vec![
                OpF::Update {
                    old: "hello\n    world\nfoo",
                    new: "hello\n    world\n    x\nfoo"
                },
                OpF::Update {
                    old: "hello\n    world",
                    new: "hello\n    world\n    x"
                },
                OpF::Exact {
                    old: "hello",
                    new: "hello"
                },
                OpF::Update {
                    old: "\n    world",
                    new: "\n    world\n    x"
                },
                OpF::Exact {
                    old: "world",
                    new: "world"
                },
                OpF::InsertSubtree { new: "x" },
                OpF::Exact {
                    old: "foo",
                    new: "foo"
                },
            ]
        );
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use super::node::*;
use super::tree_diff::*;

impl Context {
    pub fn reconcile_changes(
        context: Rc<RefCell<Context>>,
        update_context: &UpdateContext,
        old_context: &Context,
        new_root_id: NodeId,
    ) {
        let reconciliations_by_target = update_context
            .get_root_diff_reconciliations_by_target_node()
            .unwrap();

        let cursor = NodeCursor::new(context.clone(), new_root_id);
        for node_id in cursor {
            let new = context.borrow().get_ts_node_id(node_id).unwrap();
            if let Some(op) = reconciliations_by_target.get(&new) {
                dbg!(op);
                Context::reconcile_node_op(context.clone(), old_context, op);
            }
        }
    }

    pub fn reconcile_node_op(context: Rc<RefCell<Context>>, old_context: &Context, op: &OpPtr) {
        let (old, new) = op.reconciliation().unwrap();

        let old_node_id = old_context.lookup(old).unwrap();
        let new_node_id = context.borrow().lookup(new).unwrap();

        match **op {
            OpF::Exact { .. } => {
                Context::reconcile_tree(
                    context,
                    old_context,
                    old_node_id,
                    new_node_id,
                    /* should_update_timestamp */ false,
                )
            }
            _ => {
                Context::reconcile_node(
                    context,
                    old_context,
                    old_node_id,
                    new_node_id,
                    /* should_update_timestamp */ true,
                )
            }
        }
    }

    pub fn reconcile_tree(
        context: Rc<RefCell<Context>>,
        old_context: &Context,
        old_node_id: NodeId,
        new_node_id: NodeId,
        should_update_timestamp: bool,
    ) {
        // Walk subtrees and gather pairs
        let mut ids = Vec::new();
        {
            let old_cursor = NodeCursorImmutable::new(old_context, old_node_id);
            let new_cursor = NodeCursor::new(context.clone(), new_node_id);

            for (old, new) in old_cursor.zip(new_cursor) {
                ids.push((old, new));
            }
        }
        for (old_node_id, new_node_id) in ids {
            Context::reconcile_node(
                context.clone(),
                old_context,
                old_node_id,
                new_node_id,
                should_update_timestamp,
            );
        }
    }

    pub fn reconcile_node(
        context: Rc<RefCell<Context>>,
        old_context: &Context,
        old_node_id: NodeId,
        new_node_id: NodeId,
        should_update_timestamp: bool,
    ) {
        // Update metadata
        let old_metadata = old_context.get_node_metadata(old_node_id).unwrap();
        let mut new_metadata = context
            .borrow()
            .get_node_metadata(new_node_id)
            .unwrap()
            .clone();
        new_metadata.created_at = old_metadata.created_at;
        if !should_update_timestamp {
            new_metadata.updated_at = old_metadata.updated_at;
        }
        dbg!((&old_metadata, &new_metadata));
        context
            .borrow_mut()
            .metadata
            .insert(new_node_id, new_metadata);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ops::Add,
        time::{Duration, SystemTime},
        vec,
    };

    use super::*;
    use crate::{node::printer::PrintContext, parser::Parser};

    fn split_by_updated_at(context: &Context, root_node: NodeId) -> (Vec<NodeId>, Vec<NodeId>) {
        let mut updated = Vec::new();
        let mut not_updated = Vec::new();
        let cursor = NodeCursorImmutable::new(context, root_node);
        for node_id in cursor {
            let metadata = context.get_node_metadata(node_id).unwrap();
            if metadata.updated_at == metadata.created_at {
                not_updated.push(node_id);
            } else {
                updated.push(node_id);
            }
        }
        return (updated, not_updated);
    }

    fn reconcile_changes(code1: String, code2: String) -> (Context, NodeId) {
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());

        let (context1, _) = {
            let mut context = Context::new();
            context.now = SystemTime::UNIX_EPOCH;
            let id = context.load_document(&parser.tree, &code1).unwrap();
            (context, id)
        };

        let update = parser.update(code2.clone());
        let update_context = UpdateContext::new(&update, &parser.tree);

        let (context2, id2) = {
            let mut context = Context::new();
            context.now = SystemTime::UNIX_EPOCH.add(Duration::from_secs(1));
            let id = context
                .load_document(
                    &update_context.update.new_tree,
                    &update_context.update.new_text,
                )
                .unwrap();
            (context, id)
        };

        let context = Rc::new(RefCell::new(context2));
        {
            Context::reconcile_changes(context.clone(), &update_context, &context1, id2);
        }
        (Rc::try_unwrap(context).unwrap().into_inner(), id2)
    }

    #[test]
    fn test_patch_noop() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld");
        let (context2, id2) = reconcile_changes(code1, code2);
        let (updated, not_updated) = split_by_updated_at(&context2, id2);
        assert_eq!(updated.len(), 0);
        assert_eq!(
            not_updated
                .iter()
                .map(|&id| PrintContext::fmt(id, &context2))
                .collect::<Vec<_>>(),
            vec!["hello\nworld\n", "hello\n", "world\n"]
        );
    }

    #[test]
    fn test_patch_content() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nwarld");
        let (context2, id2) = reconcile_changes(code1, code2);
        let (updated, not_updated) = split_by_updated_at(&context2, id2);
        assert_eq!(
            updated
                .iter()
                .map(|&id| PrintContext::fmt(id, &context2))
                .collect::<Vec<_>>(),
            vec!["hello\nwarld\n", "warld\n"]
        );
        assert_eq!(
            not_updated
                .iter()
                .map(|&id| PrintContext::fmt(id, &context2))
                .collect::<Vec<_>>(),
            vec!["hello\n"]
        );
    }
}

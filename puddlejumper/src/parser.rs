use core::panic;
use std::{ops::Range, collections::HashMap};

use super::diff;
use indexmap:: IndexSet;
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

#[derive(Debug)]
pub struct Updates {
    pub text_old: String,
    pub text_new: String,

    // in new coordinates:
    pub tree_old_edited: tree_sitter::Tree,
    pub tree_new: tree_sitter::Tree,
    pub updates: Vec<Update>,

    // in old coordinates:
    pub tree_old: tree_sitter::Tree,
    pub tree_new_edited: tree_sitter::Tree,
    pub reverse_updates: Vec<Update>,
}

#[derive(Debug)]
pub struct Update {
    pub tree_old: tree_sitter::Tree,
    pub tree_new: tree_sitter::Tree,
    pub change: diff::Change,
    pub _ts_changed_ranges: Vec<tree_sitter::Range>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeChange<'a> {
    pub old: Option<tree_sitter::Node<'a>>,
    pub new: Option<tree_sitter::Node<'a>>,
}


impl Updates {

    pub fn get_changed_nodes<'a>(&'a self) -> IndexSet<NodeChange> {
        std::iter::zip(&self.updates, &self.reverse_updates)
            .map(|(update, reverse_update)| {
                self.get_changed_nodes_for_update_simple(update, reverse_update)
            })
            // .flat_map(|(update, reverse_update)| {
            //     self.get_changed_nodes_for_update2(update, reverse_update)
            // })
            .collect()
    }

    fn get_changed_nodes_for_update_simple<'a>(&'a self, update: &'a Update, reverse_update: &'a Update) -> NodeChange {
        let range_before = trim_byte_range(&update.change.before_bytes, &self.text_old.as_str());
        println!("range_before: {:#?}", range_before);
        let old = self.tree_old
            .root_node()
            .descendant_for_byte_range(range_before.start, range_before.end)
            .and_then(|n| {
                println!("n (before): {:#?}", n);
                if n.kind() == "document" {
                    None
                } else {
                    Some(n)
                }
            });



        let range_after = trim_byte_range(&update.change.after_bytes, &self.text_new.as_str());
        println!("range_after : {:#?}", range_after);
        let new = self.tree_new
                .root_node()
                .descendant_for_byte_range(range_after.start, range_after.end)
                .and_then(|n| {
                    // if range_after.start == range_after.end {
                    //     return None;
                    // }
                    println!("n  (after): {:#?}", n);
                    if n.kind() == "document" {
                        None
                    } else {
                        Some(n)
                    }
                });

        NodeChange {
            old,
            new,
        }
    }

    // TODO: Represent deleted nodes
    fn get_changed_nodes_for_update<'a>(&'a self, update: &'a Update, reverse_update: &'a Update) -> NodeChange {
        let old = if update.change.before_bytes.start == update.change.before_bytes.end {
            None
        } else {
            let range_before = trim_byte_range(&update.change.before_bytes, &self.text_old.as_str());
            self.tree_old
                .root_node()
                .descendant_for_byte_range(range_before.start, range_before.end)
        };

        let new = if update.change.after_bytes.start == update.change.after_bytes.end {
            None
        } else {
            let range_after = trim_byte_range(&update.change.after_bytes, &self.text_new.as_str());
            self.tree_new
                .root_node()
                .descendant_for_byte_range(range_after.start, range_after.end)
        };

        NodeChange {
            old,
            new,
        }
    }

    pub fn get_hunks<'a>(&'a self) -> Vec<(&'a str, &'a str)> {
        self
            .get_changed_nodes()
            .iter()
            .map(|node_change| { (
                node_change.old.map_or("", |old| &self.text_old[old.byte_range()]),
                node_change.new.map_or("", |new| &self.text_new[new.byte_range()])
            ) })
            .collect()
    }
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

    pub fn update(&mut self, text_new: String) -> Updates {
        return self.update_with_diff(text_new);
    }

    pub fn update_with_diff(&mut self, text_new: String) -> Updates {
        let text_old = self.text.clone();

        let diff = diff::compute_diff(text_old.as_str(), text_new.as_str());

        let mut updates = Vec::new();
        let mut reverse_updates = Vec::new();

        let mut tree_old = self.tree.clone();
        let mut tree_old_edited = self.tree.clone();
        let mut tree_new = self.tree.clone();
        for change in diff.changes {
            let text_intermediate_new = format!(
                "{}{}",
                &text_new[0..change.after_bytes.end],
                &text_old[change.before_bytes.end..]
            );

            // Prepare old tree
            let edit = change.input_edit();
            tree_old_edited.edit(&edit);
            // println!("before: {:#?}", tree_old);
            edit_nodes(tree_old.root_node(), &edit);
            // println!("after: {:#?}", tree_old);

            // Parse new tree
            tree_new = self
                .parser
                .parse(text_intermediate_new, Some(&tree_old_edited))
                .unwrap();

            // println!("change: {:#?}", change);

            let update = Update {
                change,
                tree_old: tree_old_edited.clone(),
                tree_new: tree_new.clone(),
                _ts_changed_ranges: tree_old_edited.changed_ranges(&tree_new).collect(),
            };

            updates.push(update);
        }

        let mut tree_old_reverse = tree_new.clone();
        let mut tree_new_reverse = tree_new.clone();

        for change in diff.reverse_changes.into_iter().rev() {
            let text_intermediate = format!(
                "{}{}",
                &text_new[0..change.before_bytes.end],
                &text_old[change.after_bytes.end..]
            );

            // Prepare old tree
            tree_old_reverse.edit(&change.input_edit());

            // Parse new tree
            tree_new_reverse = self
                .parser
                .parse(text_intermediate, Some(&tree_old_reverse))
                .unwrap();

            // println!("reverse_change: {:#?}", change);

            let update = Update {
                change,
                tree_old: tree_old_reverse.clone(),
                tree_new: tree_new_reverse.clone(),
                _ts_changed_ranges: tree_old_reverse.changed_ranges(&tree_new_reverse).collect(),
            };

            reverse_updates.push(update);
        }
        reverse_updates.reverse();
        assert_eq!(updates.len(), reverse_updates.len());

        return Updates { 
            updates, 
            reverse_updates,
            tree_old,
            tree_old_edited, 
            tree_new,
            // tree_old: tree_new_reverse.clone(),
            tree_new_edited: tree_old_reverse,
            text_old, 
            text_new,
        };
    }

    pub fn apply_updates(&mut self, updates: &Updates) {
        self.tree = updates.tree_new.clone();
        self.text = updates.text_new.clone();
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out, |_n| {
            return true;
        });
    }

    pub fn get_text(&self, n: tree_sitter::Node) -> &str {
        return &self.text[n.start_byte()..n.end_byte()];
    }
}

// Walk identical trees and generate a bidirectional mapping from old to new nodes
pub fn create_mapping<'a>(
    old: &tree_sitter::Node<'a>,
    new: &tree_sitter::Node<'a>,
) -> HashMap<tree_sitter::Node<'a>, tree_sitter::Node<'a>> {
    let mut mapping = HashMap::new();
    let mut cursor_old = old.walk();
    let mut cursor_new = new.walk();
    loop {
        let n_old = cursor_old.node();
        let n_new = cursor_new.node();
        mapping.insert(n_old, n_new);
        mapping.insert(n_new, n_old);
        if both(&mut cursor_old, &mut cursor_new, &|c| c.goto_first_child() ) {
            continue;
        }
        while !both(&mut cursor_old, &mut cursor_new, &|c| c.goto_next_sibling() ) {
            if !both(&mut cursor_old, &mut cursor_new, &|c| c.goto_parent() ) {
                return mapping;
            }
        }
    }
}

pub fn both<I, O: Eq + std::fmt::Debug>(
    old: I,
    new: I,
    fun: &dyn Fn(I) -> O,
) -> O {
    let old_val = fun(old);
    let new_val = fun(new);
    debug_assert_eq!(old_val, new_val);
    return old_val;
}



fn trim_byte_range(range: &Range<usize>, code: &str) -> Range<usize> {
    let mut start = range.start;
    let mut end = range.end;
    while start < end && code[start..start + 1].trim().is_empty() {
        start += 1;
    }
    while start < end && code[end - 1..end].trim().is_empty() {
        end -= 1;
    }
    return Range { start, end };
}

pub fn debug_print(
    node: &tree_sitter::Node,
    code: &str,
    out: &mut dyn std::io::Write,
    filter: fn(&tree_sitter::Node) -> bool,
) -> Result<(), std::io::Error> {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();

        let content = &code[n.start_byte()..n.end_byte()];
        if filter(&n) {
            write_indent(out, indent_level)?;
            write!(
                out,
                "{}@{} [{}..{}] [({}, {}) - ({}, {})]       {}\n",
                n.kind(),
                n.id(),
                n.start_byte(),
                n.end_byte(),
                n.start_position().row,
                n.start_position().column,
                n.end_position().row,
                n.end_position().column,
                serde_json::to_string(content).unwrap()
            )?;
        }

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

pub fn edit_nodes(mut node: tree_sitter::Node, edit: &tree_sitter::InputEdit) {
    node.edit(edit);
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        edit_nodes(child, edit);
    }
}

pub fn get_has_changed_rec<'a>(node: &tree_sitter::Node<'a>, bubble_up: bool, changed_nodes: &mut Vec<tree_sitter::Node<'a>>) -> bool  {
    if node.child_count() == 0 {
        if node.has_changes() {
            changed_nodes.push(*node);
            return true;
        } else {
            return false;
        }
    }
    let mut cursor = node.walk();
    let mut children_changed = false;
    for child in node.named_children(&mut cursor) {
        children_changed |= get_has_changed_rec(&child, bubble_up, changed_nodes);
    }
    if bubble_up && (children_changed || node.has_changes()) {
        changed_nodes.push(*node);
    }
    return children_changed;
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
    fn test_update_noop() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(updates.updates.len(), 0);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![] as Vec<(&str, &str)>);
    }

    #[test]
    fn test_update_content() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hallo\nworld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(updates.updates.len(), 1);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("hello", "hallo")]);
    }

    #[test]
    fn test_update_content_line_change() {
        let code1 = String::from("blah\nhello\nworld\nfoo");
        let code2 = String::from("blah\nhello\n\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        // parser.debug_print(&mut std::io::stdout()).unwrap();

        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        // parser.debug_print(&mut std::io::stdout()).unwrap();

        assert_eq!(updates.updates.len(), 1);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("world", "world")]);
    }

    #[test]
    fn test_update_append() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(updates.get_hunks(), vec![("", "foo")]);
    }

    #[test]
    fn test_update_change_kind() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("world", "world")]);
    }

    #[test]
    fn test_update_change_kind_reverse() {
        let code1 = String::from("hello\n  world\nfoo");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("world", "world")]);
    }

    // #[test]
    // fn test_update_change_kind_multiple() {
    //     let code1 = String::from("hello\nworld\nfoo");
    //     let code2 = String::from("hello\n  world\n  @foo");
    //     let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
    //     let updates = parser.update(code2.clone());
    //     assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
    //     assert_eq!(updates.get_hunks(), vec![("world", "world"), ("foo", "@")]);
    // }

    #[test]
    fn test_update_delete() {
        let code1 = String::from("hello\nworld\ngah\nblah");
        let code2 = String::from("world\ngah\nblah");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("hello", "")]);
    }

    #[test]
    fn test_update_delete_end() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("world", "")]);
    }

    #[test]
    fn test_update_delete_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(updates.get_hunks(), vec![("hello", ""), ("foo", "")] as Vec<(&str, &str)>);
    }

    #[test]
    fn test_update_insert_multiple() {
        let code1 = String::from(
            r#"hello
  world
foo
  bar"#,
        );
        let code2 = String::from(
            r#"hello
  x
  world
foo
  y
  bar"#,
        );
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let updates = parser.update(code2.clone());
        parser.apply_updates(&updates);
        assert_eq!(updates.get_hunks(), vec![("", "x"), ("", "y")]);
    }
}

use std::ops::Range;

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

pub struct Updates {
    pub updates: Vec<Update>,
}

impl Updates {
    fn get_changed_nodes<'a>(&'a self, parser: &'a Parser) -> IndexSet<tree_sitter::Node> {
        self
            .updates
            .iter()
            .flat_map(|update| update.get_changed_nodes(parser))
            .collect()
    }
}

pub struct Update {
    // tree: tree_sitter::Tree,
    change: diff::Change,
    // _ts_changed_ranges: Vec<tree_sitter::Range>,
}

impl Update {
    fn get_changed_nodes<'a>(&'a self, parser: &'a Parser) -> IndexSet<tree_sitter::Node> {
        let nodes = self.get_changed_nodes_from_diff(parser);
        // let mut nodes = IndexSet::new();
        // nodes.extend(self.get_changed_nodes_from_ts_changed_ranges(code));
        return nodes;
    }

    // fn get_changed_nodes_from_ts_changed_ranges(&self, code: &str) -> IndexSet<tree_sitter::Node> {
    //     return self
    //         ._ts_changed_ranges
    //         .iter()
    //         .flat_map(|ts_range| {
    //             let range = trim_byte_range(&(ts_range.start_byte as usize..ts_range.end_byte as usize), code);
    //             println!("start: {}, end: {}", range.start, range.end);
    //             self.tree
    //                 .root_node()
    //                 .descendant_for_byte_range(range.start, range.end)
    //         })
    //         .collect::<IndexSet<_>>();
    // }

    // TODO: Represent deleted nodes
    fn get_changed_nodes_from_diff<'a>(&'a self, parser: &'a Parser) -> IndexSet<tree_sitter::Node> {
        if self.change.after_bytes.start == self.change.after_bytes.end {
            return IndexSet::new();
        }
        let range = trim_byte_range(&self.change.after_bytes, &parser.text);
        parser.tree
            .root_node()
            .descendant_for_byte_range(range.start, range.end)
            .into_iter()
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

    pub fn update(&mut self, text_new: String) -> Updates {
        let text_old = self.text.clone();
        let diff = diff::compute_diff(text_old.as_str(), text_new.as_str());
        let mut updates = Vec::new();
        for change in diff.changes {
            let text_intermediate = format!(
                "{}{}",
                &text_new[0..change.after_bytes.end],
                &text_old[change.before_bytes.end..]
            );

            // println!("text_intermediate:\n{}", text_intermediate);

            // Prepare old tree
            self.perform_edit(&change);

            // Parse new tree
            let new_tree = self
                .parser
                .parse(text_intermediate, Some(&self.tree))
                .unwrap();

            // Compare old and new tree
            // let changed_ranges = self.tree.changed_ranges(&new_tree).collect::<Vec<_>>();

            let update = Update {
                // tree: new_tree.clone(),
                change,
                // _ts_changed_ranges: changed_ranges,
            };

            updates.push(update);
            // changed_ranges.append(&mut new_changed_ranges.as_mut());

            self.tree = new_tree;
        }
        self.text = text_new;
        return Updates { updates };
    }

    pub fn debug_print(&self, out: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        return debug_print(&self.tree.root_node(), &self.text, out);
    }

    pub fn get_text(&self, n: tree_sitter::Node) -> &str {
        return &self.text[n.start_byte()..n.end_byte()];
    }


    // pub fn debug_updates2(&self, updates: &Vec<Update>) -> Vec<&str> {
    //     let mut result = Vec::new();
    //     for update in updates {
    //         for range in &update.reverse_changed_ranges {
    //             result.push(&self.text[range.start_byte..range.end_byte]);
    //         }
    //     }
    //     result
    // }

    pub fn debug_updates<'a>(&'a self, updates: &Updates, parser: &'a Parser) -> Vec<&str> {
        updates
            .get_changed_nodes(parser)
            .iter()
            .map(|node| { &parser.text[node.byte_range()] })
            .collect()
    }

    pub fn debug_after_bytes(&self, updates: &Updates) -> Vec<&str> {
        let mut result = Vec::new();
        for update in &updates.updates {
            result.push(&self.text[update.change.after_bytes.clone()]);
        }
        result
    }
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
        write_indent(out, indent_level)?;
        write!(
            out,
            "{} [{}..{}] [({}, {}) - ({}, {})]       {}\n",
            n.kind(),
            n.start_byte(),
            n.end_byte(),
            n.start_position().row,
            n.start_position().column,
            n.end_position().row,
            n.end_position().column,
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

// pub fn get_changed_nodes(tree: &tree_sitter::Tree, bubble_up: bool) -> Vec<tree_sitter::Node> {
//     let mut changed_nodes = Vec::new();
//     get_changed_nodes_rec(&tree.root_node(), bubble_up, &mut changed_nodes);
//     return changed_nodes
// }

// pub fn get_changed_nodes_rec<'a>(node: &tree_sitter::Node<'a>, bubble_up: bool, changed_nodes: &mut Vec<tree_sitter::Node<'a>>) -> bool  {
//     if node.child_count() == 0 {
//         if node.has_changes() {
//             changed_nodes.push(*node);
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
//         changed_nodes.push(*node);
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
        assert_eq!(changes.updates.len(), 0);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_updates(&changes, &parser), vec![] as Vec<&str>);
    }

    #[test]
    fn test_update_content() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nwarld");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(changes.updates.len(), 1);
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_updates(&changes, &parser), vec!["warld"]);
    }

    #[test]
    fn test_update_append() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("hello\nworld\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.debug_updates(&changes, &parser), vec!["foo"]);
    }

    #[test]
    fn test_update_change_kind() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\nfoo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_updates(&changes, &parser), vec!["world"]);
    }

    #[test]
    fn test_update_change_kind_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("hello\n  world\n  @foo");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(
            changes
                .get_changed_nodes(&parser)
                .iter()
                .map(|node| { parser.get_text(*node) })
                .collect::<Vec<_>>(),
            vec!["world", "@"] as Vec<&str>
        );
    }

    #[test]
    fn test_update_delete() {
        let code1 = String::from("hello\nworld");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(
            changes
                .get_changed_nodes(&parser)
                .iter()
                .map(|node| { parser.get_text(*node) })
                .collect::<Vec<_>>(),
            vec![] as Vec<&str>
        );
    }

    #[test]
    fn test_update_delete_multiple() {
        let code1 = String::from("hello\nworld\nfoo");
        let code2 = String::from("world");
        let mut parser = Parser::new(code1.clone(), tree_sitter_puddlejumper::language());
        let changes = parser.update(code2.clone());
        assert_eq!(parser.get_text(parser.tree.root_node()), code2.clone());
        assert_eq!(parser.debug_updates(&changes, &parser), vec![] as Vec<&str>);
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
        assert_eq!(parser.debug_updates(&changes, &parser), vec!["x", "y"]);
    }
}

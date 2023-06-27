use std::ops::Range;

use imara_diff::intern::{InternedInput, TokenSource};
use imara_diff::{diff, Algorithm};

#[derive(Debug, PartialEq)]
pub struct Diff<'a> {
    pub changes: Vec<Change<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Change<'a> {
    pub hunk_before: &'a str,
    pub hunk_after: &'a str,
    pub before_bytes: Range<usize>,
    pub after_bytes: Range<usize>,
    pub start_position: tree_sitter::Point,
    pub old_end_position: tree_sitter::Point,
    pub new_end_position: tree_sitter::Point,
}

struct BytesWrapper<'a> {
    bytes: std::str::Bytes<'a>,
}

impl<'a> BytesWrapper<'a> {
    pub fn new(string: &'a str) -> Self {
        BytesWrapper {
            bytes: string.bytes(),
        }
    }
}

impl<'a> TokenSource for BytesWrapper<'a> {
    type Token = u8;
    type Tokenizer = std::str::Bytes<'a>;

    fn tokenize(&self) -> Self::Tokenizer {
        self.bytes.clone()
    }
    fn estimate_tokens(&self) -> u32 {
        self.bytes.len() as u32
    }
}

pub fn compute_diff<'a>(before: &'a str, after: &'a str) -> Diff<'a> {
    let mut changes = Vec::new();

    let before_tokens = BytesWrapper::new(before);
    let after_tokens = BytesWrapper::new(after);

    let input = InternedInput::new(before_tokens, after_tokens);

    let before_token_points: Vec<tree_sitter::Point> =
        std::iter::once(tree_sitter::Point { row: 0, column: 0 })
            .chain(input.before.iter().scan(
                tree_sitter::Point { row: 0, column: 0 },
                |point, &index| {
                    if input.interner[index] == b'\n' {
                        *point = tree_sitter::Point {
                            row: point.row + 1,
                            column: 0,
                        };
                    } else {
                        *point = tree_sitter::Point {
                            row: point.row,
                            column: point.column + 1,
                        };
                    }
                    let ret = Some(point.clone());
                    return ret;
                },
            ))
            .collect();

    let after_token_points: Vec<tree_sitter::Point> =
        std::iter::once(tree_sitter::Point { row: 0, column: 0 })
            .chain(input.after.iter().scan(
                tree_sitter::Point { row: 0, column: 0 },
                |point, &index| {
                    if input.interner[index] == b'\n' {
                        *point = tree_sitter::Point {
                            row: point.row + 1,
                            column: 0,
                        };
                    } else {
                        *point = tree_sitter::Point {
                            row: point.row,
                            column: point.column + 1,
                        };
                    }
                    let ret = Some(point.clone());
                    return ret;
                },
            ))
            .collect();

    let sink = |before_bytes: Range<u32>, after_bytes: Range<u32>| {
        let before_bytes = before_bytes.start as usize..before_bytes.end as usize;
        let after_bytes = after_bytes.start as usize..after_bytes.end as usize;

        let hunk_before = &before[before_bytes.clone()];
        let hunk_after = &after[after_bytes.clone()];

        let start_position = after_token_points[after_bytes.start];

        let old_row_diff =
            before_token_points[before_bytes.end].row - before_token_points[before_bytes.start].row;
        let old_col_diff = before_token_points[before_bytes.end].column
            - before_token_points[before_bytes.end].column;
        let old_end_position = tree_sitter::Point {
            row: start_position.row + old_row_diff,
            column: if old_row_diff > 0 {
                before_token_points[before_bytes.end].column
            } else {
                start_position.column + old_col_diff
            },
        };
        let new_end_position = after_token_points[after_bytes.end];

        changes.push(Change {
            hunk_before,
            hunk_after,
            before_bytes,
            after_bytes,
            start_position,
            old_end_position,
            new_end_position,
        })
    };

    let _diff = diff(Algorithm::Histogram, &input, sink);
    return Diff { changes };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_simple_addition() {
        let before = r#"h"#;

        let after = r#"he"#;

        let diff = compute_diff(before, after);
        assert_eq!(
            &diff.changes,
            &vec![Change {
                hunk_before: "",
                hunk_after: "e",
                before_bytes: 1..1,
                after_bytes: 1..2,
                start_position: tree_sitter::Point { row: 0, column: 1 },
                old_end_position: tree_sitter::Point { row: 0, column: 1 },
                new_end_position: tree_sitter::Point { row: 0, column: 2 },
            },]
        );
    }

    #[test]
    fn test_diff_addition() {
        let before = r#"hello"#;

        let after = r#"hello
world"#;
        let diff = compute_diff(before, after);
        assert_eq!(
            &diff.changes,
            &vec![Change {
                hunk_before: "",
                hunk_after: "\nworld",
                before_bytes: 5..5,
                after_bytes: 5..11,
                start_position: tree_sitter::Point { row: 0, column: 5 },
                old_end_position: tree_sitter::Point { row: 0, column: 5 },
                new_end_position: tree_sitter::Point { row: 1, column: 5 },
            },]
        );
    }

    #[test]
    fn test_diff_deletion() {
        let before = r#"hello
world"#;

        let after = r#"hello"#;

        let diff = compute_diff(before, after);
        println!("{:#?}", diff);
        assert_eq!(
            &diff.changes,
            &vec![Change {
                hunk_before: "\nworld",
                hunk_after: "",
                before_bytes: 5..11,
                after_bytes: 5..5,
                start_position: tree_sitter::Point { row: 0, column: 5 },
                old_end_position: tree_sitter::Point { row: 1, column: 5 },
                new_end_position: tree_sitter::Point { row: 0, column: 5 },
            },]
        );
    }

    #[test]
    fn test_diff() {
        let before = r#"fn foo() -> Bar {
    let mut foo = 2;
    foo *= 50;
    println!("hello world")
}"#;

        let after = r#"// lorem ipsum
fn foo() -> Bar {
    let mut foo = 2;
    foo *= 50;
    println!("hello world");
    println!("{foo}");
}
// foo
"#;

        let diff = compute_diff(before, after);
        println!("{:#?}", diff);
        assert_eq!(
            &diff.changes,
            &vec![
                Change {
                    hunk_before: "",
                    hunk_after: "// lorem ipsum\n",
                    before_bytes: 0..0,
                    after_bytes: 0..15,
                    start_position: tree_sitter::Point { row: 0, column: 0 },
                    old_end_position: tree_sitter::Point { row: 0, column: 0 },
                    new_end_position: tree_sitter::Point { row: 1, column: 0 },
                },
                Change {
                    hunk_before: "",
                    hunk_after: ";\n    println!(\"{foo}\");",
                    before_bytes: 81..81,
                    after_bytes: 96..120,
                    start_position: tree_sitter::Point { row: 4, column: 27 },
                    old_end_position: tree_sitter::Point { row: 4, column: 27 },
                    new_end_position: tree_sitter::Point { row: 5, column: 22 },
                },
                Change {
                    hunk_before: "",
                    hunk_after: "\n// foo\n",
                    before_bytes: 83..83,
                    after_bytes: 122..130,
                    start_position: tree_sitter::Point { row: 6, column: 1 },
                    old_end_position: tree_sitter::Point { row: 6, column: 1 },
                    new_end_position: tree_sitter::Point { row: 8, column: 0 },
                },
            ]
        );
    }
}

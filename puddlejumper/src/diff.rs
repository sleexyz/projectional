use std::ops::Range;

use imara_diff::intern::{InternedInput, TokenSource};
use imara_diff::{diff, Algorithm};

#[derive(Debug, PartialEq)]
pub struct Diff {
    pub changes: Vec<Change>,
}

#[derive(Debug, PartialEq)]
pub struct Change {
    pub before_bytes: Range<usize>,
    pub after_bytes: Range<usize>,
    pub after_bytes_trimmed: Range<usize>,
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

impl Change {
    fn get_hunk<'a>(&'a self, code_before: &'a str, code_after: &'a str) -> (&'a str, &'a str) {
        (
            &code_before[self.before_bytes.clone()],
            &code_after[self.after_bytes.clone()],
        )
    }
}

impl Diff {
    fn get_hunks<'a>(
        &'a self,
        code_before: &'a str,
        code_after: &'a str,
    ) -> Vec<(&'a str, &'a str)> {
        self.changes
            .iter()
            .map(|change| change.get_hunk(code_before, code_after))
            .collect()
    }
}

pub fn compute_diff(before: &str, after: &str) -> Diff {
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

        let mut after_bytes_start_trimmed_offset: usize = 0;
        for i in 0..after_bytes.len() {
            if after.as_bytes()[after_bytes.start + i] == b'\n' {
                after_bytes_start_trimmed_offset += 1;
                continue;
            }
            break;
        }

        let after_bytes_trimmed =
            after_bytes.start + after_bytes_start_trimmed_offset..after_bytes.end as usize;

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
            before_bytes,
            after_bytes,
            after_bytes_trimmed,
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
                before_bytes: 1..1,
                after_bytes: 1..2,
                after_bytes_trimmed: 1..2,
                start_position: tree_sitter::Point { row: 0, column: 1 },
                old_end_position: tree_sitter::Point { row: 0, column: 1 },
                new_end_position: tree_sitter::Point { row: 0, column: 2 },
            },]
        );
        assert_eq!(&diff.get_hunks(before, after), &vec![("", "e")]);
    }

    #[test]
    fn test_diff_simple_addition_line() {
        let before = r#"h"#;
        let after = r#"h
e"#;
        let diff = compute_diff(before, after);
        assert_eq!(
            &diff.changes,
            &vec![Change {
                before_bytes: 1..1,
                after_bytes: 1..3,
                after_bytes_trimmed: 2..3,
                start_position: tree_sitter::Point { row: 0, column: 1 },
                old_end_position: tree_sitter::Point { row: 0, column: 1 },
                new_end_position: tree_sitter::Point { row: 1, column: 1 },
            },]
        );
        assert_eq!(&diff.get_hunks(before, after), &vec![("", "\ne")]);
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
                before_bytes: 5..5,
                after_bytes: 5..11,
                after_bytes_trimmed: 6..11,
                start_position: tree_sitter::Point { row: 0, column: 5 },
                old_end_position: tree_sitter::Point { row: 0, column: 5 },
                new_end_position: tree_sitter::Point { row: 1, column: 5 },
            },]
        );
        assert_eq!(&diff.get_hunks(before, after), &vec![("", "\nworld")]);
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
                before_bytes: 5..11,
                after_bytes: 5..5,
                after_bytes_trimmed: 5..5,
                start_position: tree_sitter::Point { row: 0, column: 5 },
                old_end_position: tree_sitter::Point { row: 1, column: 5 },
                new_end_position: tree_sitter::Point { row: 0, column: 5 },
            },]
        );
        assert_eq!(&diff.get_hunks(before, after), &vec![("\nworld", "")]);
    }

    #[test]
    fn test_diff_reference() {
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
                    before_bytes: 0..0,
                    after_bytes: 0..15,
                    after_bytes_trimmed: 0..15,
                    start_position: tree_sitter::Point { row: 0, column: 0 },
                    old_end_position: tree_sitter::Point { row: 0, column: 0 },
                    new_end_position: tree_sitter::Point { row: 1, column: 0 },
                },
                Change {
                    before_bytes: 81..81,
                    after_bytes: 96..120,
                    after_bytes_trimmed: 96..120,
                    start_position: tree_sitter::Point { row: 4, column: 27 },
                    old_end_position: tree_sitter::Point { row: 4, column: 27 },
                    new_end_position: tree_sitter::Point { row: 5, column: 22 },
                },
                Change {
                    before_bytes: 83..83,
                    after_bytes: 122..130,
                    after_bytes_trimmed: 123..130,
                    start_position: tree_sitter::Point { row: 6, column: 1 },
                    old_end_position: tree_sitter::Point { row: 6, column: 1 },
                    new_end_position: tree_sitter::Point { row: 8, column: 0 },
                },
            ]
        );
        assert_eq!(
            &diff.get_hunks(before, after),
            &vec![
                ("", "// lorem ipsum\n"),
                ("", ";\n    println!(\"{foo}\");"),
                ("", "\n// foo\n"),
            ]
        );
    }
}

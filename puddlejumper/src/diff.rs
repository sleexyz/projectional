use std::ops::Range;

use imara_diff::intern::{InternedInput, TokenSource};
// use imara_diff::sources::lines;
use imara_diff::{diff, Algorithm};


#[derive(Debug, PartialEq)]
pub struct Diff<'a> {
    pub changes: Vec<Change<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Change<'a> {
    pub hunk_before: &'a str,
    pub hunk_after: &'a str,
    // pub before_lines: Range<u32>, // TODO: use usize
    // pub after_lines: Range<u32>, // TODO: use usize
    pub before_bytes: Range<usize>,
    pub after_bytes: Range<usize>,
}

struct BytesWrapper<'a> {
    bytes: std::str::Bytes<'a>
}


impl<'a> BytesWrapper<'a> {
    pub fn new(string: &'a str) -> Self {
        BytesWrapper {
            bytes: string.bytes()
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

    // let before_tokens = lines(before);
    // let after_tokens = lines(after);
    let before_tokens = BytesWrapper::new(before);
    let after_tokens = BytesWrapper::new(after);

    let input = InternedInput::new(before_tokens, after_tokens);


    let before_token_offsets: Vec<_> = std::iter::once(0).chain(input.before.iter().scan(0, |offset, &line| {
        *offset += 1;
        // *offset += input.interner[line].len();
        // if *offset < before.len() {
        //     *offset += "\n".len();
        // }
        let end = offset.clone();
        assert!(before.len() >= offset.clone(), "offset: {}, before.len(): {}", offset, before.len());
        return Some(end);
    })).collect();

    let after_token_offsets: Vec<_> = std::iter::once(0).chain(input.after.iter().scan(0, |offset, &line| {
        *offset += 1;
        // *offset += input.interner[line].len();
        // if *offset < after.len() {
        //     *offset += "\n".len();
        // }
        let end = offset.clone();
        assert!(after.len() >= offset.clone(), "offset: {}, after.len(): {}", offset, after.len());
        return Some(end);
    })).collect();

    let mut last_byte = 0;

    let sink = |before_bytes: Range<u32>, after_bytes: Range<u32>| {
        let before_bytes = before_bytes.start as usize..before_bytes.end as usize;
        let after_bytes = after_bytes.start as usize..after_bytes.end as usize;

        // let before_bytes = before_token_offsets[before_lines.start as usize]..before_token_offsets[before_lines.end as usize];
        // let hunk_before: Vec<_> = input.before[before_lines.start as usize..before_lines.end as usize]
        //     .iter()
        //     .map(|&line| input.interner[line])
        //     .collect();

        let hunk_before = &before[before_bytes.clone()];

        // let after_bytes = after_token_offsets[after_lines.start as usize]..after_token_offsets[after_lines.end as usize];
        // let hunk_after: Vec<_> = input.after[after_lines.start as usize..after_lines.end as usize]
        //     .iter()
        //     .map(|&line| input.interner[line])
        //     .collect();
        
        let hunk_after = &after[after_bytes.clone()];

        changes.push(Change {
            hunk_before,
            hunk_after,
            // before_lines,
            // after_lines,
            before_bytes,
            after_bytes,
        })
    };

    let _diff = diff(Algorithm::Histogram, &input, sink);
    return Diff {
        changes
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_simple() {
        let before = r#"hello"#;
        
        let after = r#"hello
world"#;

        let diff = compute_diff(before, after);
        println!("{:#?}", diff);
        assert_eq!(
            &diff.changes,
            &vec![
                Change {
                    hunk_before: "",
                    hunk_after: "world",
                    // before_lines: 1..1,
                    // after_lines: 1..2,
                    before_bytes: 5..5,
                    after_bytes: 6..11,
                },
            ]
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
                    // before_lines: 0..0,
                    // after_lines: 0..1,
                    before_bytes: 0..0,
                    after_bytes: 0..15,
                },
                Change {
                    hunk_before: "    println!(\"hello world\")\n",
                    hunk_after:  "    println!(\"hello world\");\n    println!(\"{foo}\");\n",
                    // before_lines: 3..4,
                    // after_lines: 4..6,
                    before_bytes: 54..82,
                    after_bytes: 69..121,
                },
                Change {
                    hunk_before: "",
                    hunk_after: "// foo\n",
                    // before_lines: 5..5,
                    // after_lines: 7..8,
                    before_bytes: 83..83,
                    after_bytes: 123..130,
                },
            ]
        );
    }
}
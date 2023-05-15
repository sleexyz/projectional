use std::env;
use std::fs;
use tree_sitter::{self, Tree };

fn main() {
    // Retrieve the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <file_path>");
        return;
    }
    let file_path = &args[1];

    // Read the contents of the file
    let code = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_puddlejumper::language()).expect("Error loading puddlejumper grammar");
    let tree: Tree = parser.parse(&code, None).unwrap();
    // println!("{}", tree.root_node().to_sexp());
    pretty_print(tree.root_node(), &code);
}

fn pretty_print(node: tree_sitter::Node, input: &str) {
    let mut indent_level = 0;
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();

        // Print node's content
        if n.kind() == "content" {
            print_indent(indent_level);
            println!("{}", &input[n.start_byte()..n.end_byte()]);
        }

        // Add newline if necessary
        if n.kind() == "children" {
            indent_level += 1;
        }

        // Move to the next node
        if cursor.goto_first_child() {
            continue;
        }

        // No child nodes, move to the next sibling or parent's next sibling
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return;
            }
            if cursor.node().kind() == "children" {
                indent_level -= 1;
            }
        }
    }
}

fn print_indent(_indent_level: usize) {
    // let indent = "  ";
    // for _ in 0..indent_level {
    //     // print!("{}", indent);
    // }
}


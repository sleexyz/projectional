mod puddlejumper;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

fn print_usage() {
    println!("Usage: cargo run -- [print | debug_print | parse | print_prioritized] <file_path>");
}

fn main() {
    // Retrieve the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        return;
    }

    let command: &str = args[1].as_str();
    let file_path: &str = &args[2].as_str();

    if file_path == "" {
        print_usage();
        return;
    }

    // Read the contents of the file
    let code = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };

    // Parse and print the code
    let p = puddlejumper::Parser::new(code);
    match command {
        "print" => {
            let result = p.pretty_print(&mut std::io::stdout(), 0);
            match result {
                Ok(_) => (),
                Err(error) => {
                    println!("Error pretty printing file: {}", error);
                    return;
                }
            }
        }
        "debug_print" => {
            let result = p.debug_print(&mut std::io::stdout());
            match result {
                Ok(_) => (),
                Err(error) => {
                    println!("Error debug printing file: {}", error);
                    return;
                }
            }
        }
        "parse" => {
            let result = p.load_document();
            match result {
                Some(node) => {
                    println!("{:#?}", node);
                    println!("File parsed successfully");
                    return;
                }
                None => {
                    println!("Error parsing file");
                    return;
                }
            }
        }
        "print_prioritized" => {
            let result = p
                .load_document()
                .ok_or(Error::new(ErrorKind::Other, "Error parsing file"))
                .and_then(|node| {
                    let list = node.make_prioritized_list();
                    return list.pretty_print(&mut puddlejumper::PrintContext {
                        level: 0,
                        needs_indent: true,
                        out: &mut std::io::stdout(),
                        input: &p.text,
                    })
                });
            match result {
                Ok(_) => (),
                Err(error) => {
                    println!("Error printing file: {}", error);
                    return;
                }
            }
        }
        _ => {
            print_usage();
            return;
        }
    }
}

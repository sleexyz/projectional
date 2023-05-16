mod puddlejumper;
use std::env;
use std::fs;
fn print_usage() {
    println!("Usage: cargo run -- [pretty_print | print | debug_print] <file_path>");
}

fn main() {
    // Retrieve the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        return;
    }
    let mut file_path = "";
    let mut pretty_print = false;
    let mut lossless_print = false;
    let mut debug_print = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "pretty_print" => pretty_print = true,
            "debug_print" => debug_print = true,
            "print" => lossless_print = true,
            _ => {
                file_path = &args[i];
                break;
            }
        }
        i += 1;
    }

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
    if pretty_print {
        let result = p.pretty_print(&mut std::io::stdout());
        match result {
            Ok(_) => (),
            Err(error) => {
                println!("Error pretty printing file: {}", error);
                return;
            }
        }
    } else if lossless_print {
        let result = p.lossless_print(&mut std::io::stdout());
        match result {
            Ok(_) => (),
            Err(error) => {
                println!("Error lossless printing file: {}", error);
                return;
            }
        }
    } else if debug_print {
        let result = p.debug_print(&mut std::io::stdout());
        match result {
            Ok(_) => (),
            Err(error) => {
                println!("Error debug printing file: {}", error);
                return;
            }
        }
    } else {
        print_usage()
    }
}
mod lexer;
mod parser;
mod token;

use std::env;
use std::fs;
use std::io;

fn main() -> () {
    // main read loop
    let mut args = env::args();

    match args.len() {
        1 => eprintln!("Usage: [script]"),
        2 => {
            if let Some(file_name) = args.nth(1) {
                let contents = fs::read_to_string(file_name).unwrap();
                let lexer = lexer::Lexer::new(contents.to_string().as_str());
            };
        }
        3.. => {}
        _ => (),
    }

    if args.len() <= 1 {
        ()
    }

    if args.len() == 2 {}

    let contents = todo!(); // fs::read_to_string();

    ()
}

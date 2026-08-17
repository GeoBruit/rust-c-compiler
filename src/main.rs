mod lexer;
mod parser;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.last()
        .expect("No source file provided");

    let source = fs::read_to_string(filename)
        .expect("Failed to read source file");

    let tokens = lexer::lex(&source);
    parser::parse(&tokens);
    println!("{:#?}", tokens);
}
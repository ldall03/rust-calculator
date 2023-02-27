use std::env;

// local imports
mod tokenizing;
mod parser;
use crate::tokenizing::Token;
use crate::tokenizing::tokenize;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_concat: String = args[1..].join("");
    println!("{}", args_concat);
    let tokens: Vec<Token> = tokenize(args_concat);

    let mut parser = Parser::new(tokens);
    let result: f64 = parser.parse();
    println!("{}", result);
}


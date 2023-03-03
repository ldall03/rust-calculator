use std::env;

// local imports
mod tokenizing;
mod parser;
use crate::tokenizing::Token;
use crate::tokenizing::tokenize;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide a math expression.");
        return;
    }

    let args_concat: String = args[1..].join("");
    let tokens: Vec<Token> = tokenize(args_concat);

    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    match result {
        Ok(r) => println!("{}", r),
        _ => println!("There was an error with your input.")
    }
}


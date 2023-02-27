//      GRAMMAR:
//
//      <expr>     ::= <term> <exprcl>
//
//      <exprcl>   ::= e
//                  | + <term> <exprcl>
//                  | - <term> <exprcl>
//
//      <term>     ::= <unary> <termcl>
//
//      <termcl>   ::= e
//                  | * <unary> <termcl>
//                  | / <unary> <termcl>
//
//      <unary>    ::= ! <unary>
//                  | - <unary>
//                  | <factor>
//
//      <factor>   ::= ( <expr> )
//                  | NUM

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_concat: String = args[1..].join("");
    println!("{}", args_concat);
    let tokens: Vec<Token> = tokenize(args_concat);
    for t in tokens {
        println!("{:?}, {:?}, {:?}", t.t, t.op, t.value);
    }
}

fn is_valid_op(op: &char) -> bool {
    vec!['+', '-', '*', '/', '(', '[', ']', ')'].contains(op)
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut t: Token;
    let mut current: String = String::new();
    let mut is_decimal: bool = false;
    for c in input.chars() {
        if c == ' ' {
            continue;
        } else if c.is_ascii_digit() {
            current.push(c);
        } else if (c == '.') && (!is_decimal) {
            current.push(c);
            is_decimal = true;
        } else if c == '.' {
            panic!("Invalid token {}", c);
        } else if is_valid_op(&c) && current.len() > 0 {
            t = Token {
                t: Type::Num,
                op: None,
                value: Some(current),
            };
            current = String::new();
            tokens.push(t);
            let op = match c {
                '+' => Op::Plus,
                '-' => Op::Minus,
                '*' => Op::Mult,
                '/' => Op::Div,
                '(' | '[' => Op::OpParen,
                ')' | ']' => Op::ClParen,
                _ => panic!("Invalid token {}", c)

            };
            t = Token {
                t: Type::Op,
                op: Some(op),
                value: None,
            };
            tokens.push(t);
        } else if is_valid_op(&c) {
            let op = match c {
                '+' => Op::Plus,
                '-' => Op::Minus,
                '*' => Op::Mult,
                '/' => Op::Div,
                '(' | '[' => Op::OpParen,
                ')' | ']' => Op::ClParen,
                _ => panic!("Invalid token {}", c)
            };
            t = Token {
                t: Type::Op,
                op: Some(op),
                value: None,
            };
            tokens.push(t);
        } else {
            print!("{}", c);
            panic!("Invalid Token {}", c);
        }
    }
    if current.len() > 0 {
        t = Token {
            t: Type::Num,
            op: None,
            value: Some(current),
        };
        tokens.push(t);
    }

    tokens
}

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    OpParen,
    ClParen,
}

#[derive(Debug)]
enum Type {
    Op,
    Num,
}

struct Token {
    t: Type,
    op: Option<Op>,
    value: Option<String>,
}

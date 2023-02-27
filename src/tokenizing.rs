#[derive(Debug, PartialEq)]
pub enum Vocab {
    Plus,
    Minus,
    Mult,
    Div,
    OpParen,
    ClParen,
    OpBracket,
    ClBracket,
    Num,
}

pub struct Token {
    pub vocab: Vocab,
    pub value: String,
}

fn is_valid_op(op: &char) -> bool {
    vec!['+', '-', '*', '/', '(', '[', ']', ')'].contains(op)
}

pub fn tokenize(input: String) -> Vec<Token> {
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
                vocab: Vocab::Num,
                value: current,
            };
            current = String::new();
            is_decimal = false;
            tokens.push(t);
            let op = match c {
                '+' => Vocab::Plus,
                '-' => Vocab::Minus,
                '*' => Vocab::Mult,
                '/' => Vocab::Div,
                '(' => Vocab::OpParen,
                ')' => Vocab::ClParen,
                '[' => Vocab::OpBracket,
                ']' => Vocab::ClBracket,
                _ => panic!("Invalid token {}", c),
            };
            t = Token {
                vocab: op,
                value: String::from("op"),
            };
            tokens.push(t);
        } else if is_valid_op(&c) {
            let op = match c {
                '+' => Vocab::Plus,
                '-' => Vocab::Minus,
                '*' => Vocab::Mult,
                '/' => Vocab::Div,
                '(' => Vocab::OpParen,
                ')' => Vocab::ClParen,
                '[' => Vocab::OpBracket,
                ']' => Vocab::ClBracket,
                _ => panic!("Invalid token {}", c),
            };
            t = Token {
                vocab: op,
                value: String::from("op"),
            };
            tokens.push(t);
        } else {
            print!("{}", c);
            panic!("Invalid Token {}", c);
        }
    }
    // If we have one last number to push
    if current.len() > 0 {
        t = Token {
            vocab: Vocab::Num,
            value: current,
        };
        tokens.push(t);
    }
    tokens
}

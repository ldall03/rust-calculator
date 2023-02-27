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
//      <unary>    ::= - <unary>
//                  | <factor>
//
//      <factor>   ::= ( <expr> )
//                  | NUM

use crate::tokenizing::Token;
use crate::tokenizing::Vocab;

pub struct Parser {
    tokens: Vec<Token>,
    current: Token,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        if let Some(current) = tokens.pop() {
            Self { tokens, current }
        } else {
            panic!("Empty expr");
        }
    }

    fn get_token(&mut self) {
        if let Some(token) = self.tokens.pop() {
            self.current = token;
        }
    }

    fn must_be(&mut self, op: Vocab) {
        if self.current.vocab != op {
            panic!("Expected {:?}, but got {:?}", op, self.current.vocab);
        }
        self.get_token();
    }

    pub fn parse(&mut self) -> f64 {
        let res: f64 = self.parse_expr();
        if self.tokens.len() > 0 {
            panic!("There was an error parsing at: {:?}", self.current.vocab);
        } else {
            res
        }
    }

    //      <factor>   ::= ( <expr> )
    //                   | [ <expr> ]
    //                   | NUM
    fn parse_factor(&mut self) -> f64 {
        if matches!(self.current.vocab, Vocab::OpParen) {
            self.get_token();
            let ret: f64 = self.parse_expr();
            self.must_be(Vocab::ClParen);
            ret
        } else if matches!(self.current.vocab, Vocab::OpBracket) {
            self.get_token();
            let ret: f64 = self.parse_expr();
            self.must_be(Vocab::ClBracket);
            ret
        } else if matches!(self.current.vocab, Vocab::Num) {
            let ret: f64 = self.current.value.parse::<f64>().unwrap();
            self.get_token();
            ret
        } else {
            panic!("Bad number");
        }
    }

    //      <unary>    ::= - <unary>
    //                   | <factor>
    fn parse_unary(&mut self) -> f64 {
        if matches!(self.current.vocab, Vocab::Minus) {
            self.get_token();
            -self.parse_unary()
        } else {
            self.parse_factor()
        }
    }

    //      <termcl>   ::= e
    //                   | * <unary> <termcl>
    //                   | / <unary> <termcl>
    fn parse_termcl(&mut self, passed: f64) -> f64 {
        if matches!(self.current.vocab, Vocab::Mult) {
            self.get_token();
            let unary: f64 = self.parse_unary();
            let val: f64 = passed * unary;
            self.parse_termcl(val)
        } else if matches!(self.current.vocab, Vocab::Div) {
            self.get_token();
            let unary: f64 = self.parse_unary();
            let val: f64 = passed / unary;
            self.parse_termcl(val)
        } else {
            passed
        }
    }

    //      <term>     ::= <unary> <termcl>
    fn parse_term(&mut self) -> f64 {
        let unary: f64 = self.parse_unary();
        self.parse_termcl(unary)
    }

    //      <exprcl>   ::= e
    //                  | + <term> <exprcl>
    //                  | - <term> <exprcl>
    fn parse_exprcl(&mut self, passed: f64) -> f64 {
        if matches!(self.current.vocab, Vocab::Plus) {
            self.get_token();
            let term: f64 = self.parse_term();
            let val: f64 = passed + term;
            self.parse_exprcl(val)
        } else if matches!(self.current.vocab, Vocab::Minus) {
            self.get_token();
            let term: f64 = self.parse_term();
            let val: f64 = passed - term;
            self.parse_exprcl(val)
        } else {
            passed
        }
    }

    //      <expr>     ::= <term> <exprcl>
    fn parse_expr(&mut self) -> f64 {
        let term: f64 = self.parse_term();
        self.parse_exprcl(term)
    }
}

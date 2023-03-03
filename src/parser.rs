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
        let current = tokens.pop().unwrap();
        Self {tokens, current }
    }

    fn get_token(&mut self) {
        if let Some(token) = self.tokens.pop() {
            self.current = token;
        }
    }

    fn must_be(&mut self, op: Vocab) -> Result<(), i32> {
        if self.current.vocab != op {
            return Err(-3);
        }
        self.get_token();
        Ok(())
    }

    pub fn parse(&mut self) -> Result<f64, i32> {
        let res = self.parse_expr();
        if self.tokens.len() > 0 {
            Err(-2)
        } else {
            res
        }
    }

    //      <factor>   ::= ( <expr> )
    //                   | [ <expr> ]
    //                   | NUM
    fn parse_factor(&mut self) -> Result<f64, i32> {
        if self.current.vocab == Vocab::OpParen {
            self.get_token();
            let ret = self.parse_expr();
            self.must_be(Vocab::ClParen)?;
            ret
        } else if self.current.vocab == Vocab::OpBracket {
            self.get_token();
            let ret = self.parse_expr();
            self.must_be(Vocab::ClBracket)?;
            ret
        } else {
            let ret = self.current.value.parse::<f64>();
            self.get_token();
            match ret {
                Ok(ret) => Ok(ret),
                Err(_) => Err(-1)
            }
        } 
    }

    //      <unary>    ::= - <unary>
    //                   | <factor>
    fn parse_unary(&mut self) -> Result<f64, i32> {
        if self.current.vocab == Vocab::Minus {
            self.get_token();
            Ok(-self.parse_unary()?)
        } else {
            self.parse_factor()
        }
    }

    //      <termcl>   ::= e
    //                   | * <unary> <termcl>
    //                   | / <unary> <termcl>
    fn parse_termcl(&mut self, passed: Result<f64, i32>) -> Result<f64, i32> {
        if self.current.vocab == Vocab::Mult {
            self.get_token();
            let unary = self.parse_unary()?;
            self.parse_termcl(Ok(passed? * unary))
        } else if self.current.vocab == Vocab::Div {
            self.get_token();
            let unary = self.parse_unary()?;
            self.parse_termcl(Ok(passed? / unary))
        } else {
            passed // Base case
        }
    }

    //      <term>     ::= <unary> <termcl>
    fn parse_term(&mut self) -> Result<f64, i32> {
        let unary = self.parse_unary();
        self.parse_termcl(unary)
    }

    //      <exprcl>   ::= e
    //                  | + <term> <exprcl>
    //                  | - <term> <exprcl>
    fn parse_exprcl(&mut self, passed: Result<f64, i32>) -> Result<f64, i32> {
        if self.current.vocab == Vocab::Plus {
            self.get_token();
            let term = self.parse_term()?;
            self.parse_termcl(Ok(passed? + term))
        } else if self.current.vocab == Vocab::Minus {
            self.get_token();
            let term = self.parse_term()?;
            self.parse_termcl(Ok(passed? - term))
        } else {
            passed
        }
    }

    //      <expr>     ::= <term> <exprcl>
    fn parse_expr(&mut self) -> Result<f64, i32> {
        let term = self.parse_term();
        self.parse_exprcl(term)
    }
}

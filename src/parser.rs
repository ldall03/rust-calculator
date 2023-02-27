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
//      <unary>    ::= - <unary>
//                  | <factor>
//
//      <factor>   ::= ( <expr> )
//                  | NUM

struct Parser {
    tokens: Vec<Token>,
    current: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> self {
        Self { tokens }
    }

    fn get_token() {
        current = tokens.pop();
    }

    fn must_be(op: Op) {
        if current.op != op {
            panic!("Expecated {}, but got {}", op, current.op);
        }
        get_token();
    } 

    pub fn parse() -> f64 {
        get_token();
        expr()
    }

    //      <factor>   ::= ( <expr> )
    //                  | NUM
    fn parse_factor() -> f64 {
        if current.op == Op.OpParen {
            let ret: f64 = parse_expr();
            must_be(Op.ClParen);
            ret
        } else if current.t == Type::Num {
            let ret: f64 = current.value.parse();
            get_token();
            ret
        } else {
            panic!("Unexpected token");
        }
    }

    //      <unary>    ::= - <unary>
    //                   | <factor>
    fn parse_unary() -> f64 {
        if current.op == Op.Minus {
            get_token();
            -parse_unary()
        } else {
            parse_factor()
        }
    }

    //      <termcl>   ::= e
    //                   | * <unary> <termcl>
    //                   | / <unary> <termcl>
    fn parse_termcl(passed: f64) -> f64 {
        if current.op == Op.Mult {
            get_token();
            let unary: f64 = parse_unary();
            let val: f64 = passed * unary;
            parse_termcl(val)
        } else if current.op == Op.Div {
            get_token();
            let unary: f64 = parse_unary();
            let val: f64 = passed / unary;
            parse_termcl(val)
        } else {
            passed
        }
    }

    
    //      <term>     ::= <unary> <termcl>
    fn parse_term() -> f64 {
        let unary: f64 = parse_unary();
        parse_termcl(unary)
    }

    //      <exprcl>   ::= e
    //                  | + <term> <exprcl>
    //                  | - <term> <exprcl>
    fn parse_exprcl(passed: f64) -> f64 {
        if current.op == Op.Plus {
            get_token();
            let term: f64 = parse_term();
            let val: f64 = passed + term;
            parse_exprcl(val)
        } else if current.op == Op.minus {
            get_token();
            let term: f64 = parse_term();
            let val: f64 = passed - term;
            parse_exprcl(val)
        } else {
            passed
        }
    }

    //      <expr>     ::= <term> <exprcl>
    fn parse_expr() -> f64 {
        let term: f64 = parse_term();
        parse_exprcl(term)
    }

}












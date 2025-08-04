use super::{ast::bp::get_infix, bp, Assign, Ast, BinOperation, BinOperator, Expr, Ident, Literal, BP};
use crate::tokenizer::{Keyword, Operator, Token};
use std::process::exit;

pub struct Parser {
    tokens: Vec<Token>,
    current: Vec<Token>,
    ast: Ast,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: vec![],
            ast: Ast::default(),
        }
    }


    fn next(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(0).unwrap()
    }


    fn pratt_parse(&mut self, min_bp: BP) -> Expr {
        let mut lhs = match self.next() {
            Token::Number(number) => Expr::Literal(Literal::Number(number)), 
            token => {
                eprint!("Error: Parser did not get a number");
                exit(1);
            },
        };

        loop {
            let op = match self.peek() {
                Token::Newline => break,
                Token::Operator(operator) => operator.into(),
                token => {
                    eprint!("Error: Parser did not get a newline or an operator");
                    exit(1);
                },
            };

            let (l_bp, r_bp) = get_infix(op);
            let op = op.clone().into();

            if min_bp > l_bp {
                break;
            };
            self.next();

            let rhs = self.pratt_parse(r_bp);
            lhs = Expr::BinOp(Box::new(BinOperation::new(op,lhs,rhs)));

        };
        lhs    
    }
    


    pub fn parse(mut self) -> Expr {
        self.pratt_parse(0.0)
    }


    



}

use std::{process::exit, usize};

use crate::{tokenizer::Lexer, units::{bp, BinOpn, Expr, Literal, Token, BP}};
use bp::get_infix;

pub struct Parser {
    lexer: Lexer,
    current: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Lexer) -> Self {
        Parser {
            lexer: tokens,
            current: vec![],
        }
    }

    fn next(&mut self) -> Token {
        self.lexer.next()
    }

    fn peek(&mut self) -> &Token {
        self.lexer.peek()
    }

    fn peek_at(&mut self, index: usize) -> &Token {
        self.lexer.peek_at(index)
    }

    fn pratt_parse(&mut self, min_bp: BP) -> Expr {
        let mut lhs = match self.next() {
            Token::Number(number) => Expr::Literal(Literal::Number(number)),
            token => {
                eprint!("Error: Parser did not get a number");
                exit(1);
            }
        };

        loop {
            let op = match self.peek() {
                Token::Newline => break,
                Token::Operator(operator) => operator.into(),
                token => {
                    eprint!("Error: Parser did not get a newline or an operator");
                    exit(1);
                }
            };

            let (l_bp, r_bp) = get_infix(op);
            let op = op.clone().into();

            if min_bp > l_bp {
                break;
            };
            self.next();

            let rhs = self.pratt_parse(r_bp);
            lhs = Expr::BinOp(Box::new(BinOpn::new(op, lhs, rhs)));
        }
        lhs
    }

    pub fn parse(mut self) -> Expr {
        self.pratt_parse(0.0)
    }
}

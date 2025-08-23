use super::Diagnostics;
use super::ast::{BP, BinOpn, BinOpr, Expr, LiteralKind};

use crate::{
    lex::{Token, TokenKind},
    source::Source,
};
use std::{collections::VecDeque, ops::Range};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    ExpectedExpression,
    ExpectedOperator,
    ExpectedCloseParen,
}

#[derive(Debug)]
pub struct ParserError {
    pub error: ErrorKind,
    pub token: TokenKind,
    pub range: Range<usize>,
}

struct RangedToken {
    range: Range<usize>,
    kind: TokenKind,
}


pub struct Parser<'src> {
    src: &'src Source,
    pub tokens: VecDeque<Token>,
    index: usize,
}

impl<'src> Parser<'src> {
    #[inline(always)]
    fn error(&self, error: ErrorKind, token: RangedToken) -> ParserError {
        ParserError {
            error,
            token: token.kind,
            range: token.range,
        }
    }

    #[inline(always)]
    fn recover(&mut self) {
        loop {
            let token = self.next();
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                return;
            };
        }
    }

    pub fn new<T: Into<(&'src Source, VecDeque<Token>)>>(input: T) -> Self {
        let (src, tokens) = input.into();
        assert!(!tokens.is_empty());

        Self {
            src,
            tokens,
            index: 0,
        }
    }

    #[inline(always)]
    fn next(&mut self) -> RangedToken {
        let token = loop {
            let token = self
                .tokens
                .pop_front()
                .expect("popped nothing from tokens queue");

            if token.kind != TokenKind::WhiteSpace {
                break token;
            };
            self.index += token.size;
        };

        let end = self.index + token.size;

        let range = Range {
            start: self.index,
            end,
        };

        self.index = end;

        RangedToken {
            range,
            kind: token.kind,
        }
    }

    #[inline(always)]
    fn flush_newlines(&mut self) {
        while self.peek().kind == TokenKind::Newline {
            self.next();
        }
    }

    #[inline(always)]
    fn peek_at(&self, at: usize) -> RangedToken {
        let mut iter = self.tokens.iter();
        let mut index = 0;


        let mut start = self.index;
        let token = loop {
            let token = iter.next().expect("got none on unwrap for peek");

            if token.kind == TokenKind::WhiteSpace {
                start += token.size;
                continue;
            };

            if index == at {
                break token;
            };

            start += token.size;
            index += 1;
        };

        let range = Range {
            start: start,
            end: start + token.size,
        };

        RangedToken {
            range,
            kind: token.kind,
        }
    }

    #[inline(always)]
    fn peek(&self) -> RangedToken {
        self.peek_at(0)
    }

    #[inline(always)]
    fn peek_second(&self) -> RangedToken {
        self.peek_at(1)
    }


    fn none(&self) -> RangedToken {
        let range = Range {
            start: self.index - 1,
            end: self.index,
        };

        RangedToken {
            kind: TokenKind::None,
            range,
        }
    }




    fn match_operator(&self, token_kind: TokenKind) -> Option<(BinOpr, bool)> {
        match token_kind {
            TokenKind::Plus => Some((BinOpr::Add, false)),
            TokenKind::Minus => Some((BinOpr::Sub, false)),
            TokenKind::Percent => Some((BinOpr::Mod, false)),
            TokenKind::Star => match self.peek_second().kind {
                TokenKind::Star => Some((BinOpr::Exp, true)),
                _ => Some((BinOpr::Mul, false)),
            },
            TokenKind::Slash => match self.peek_second().kind {
                TokenKind::Slash => Some((BinOpr::Flo, true)),
                _ => Some((BinOpr::Div, false)),
            },
            _ => None,
        }
    }

    fn pratt_parse(&mut self, min_bp: BP) -> Option<Result<Expr, ParserError>> {
        let token = self.next();
        let mut lhs = match token.kind {
            TokenKind::Eof | TokenKind::Newline => return None,
            TokenKind::Number => Expr::literal(LiteralKind::Number, token.range),
            TokenKind::Ident => Expr::Ident(token.range),
            TokenKind::OpenParen => {
                // first check expr for Err and None,
                let expr_opt = self.pratt_parse(min_bp);
                let expr = match expr_opt {
                    Some(Ok(expr)) => expr,
                    some_error @ Some(Err(_)) => return some_error,
                    None => {
                        let err = self.error(ErrorKind::ExpectedExpression, self.none());
                        self.recover();
                        return Some(Err(err));
                    }
                };

                // check next token exists and is a close paren
                let peeked = self.peek();
                match peeked.kind {
                    TokenKind::CloseParen => self.next(),
                    _ => {
                        let err = self.error(ErrorKind::ExpectedCloseParen, peeked);
                        self.recover();
                        return Some(Err(err));
                    },
                };
                expr
            }

            _ => {
                let error = self.error(ErrorKind::ExpectedExpression, token);
                self.recover();
                return Some(Err(error));
            }
        };


        loop {
            let op_token = self.peek();
            let (op, is_compound) = match op_token.kind {
                TokenKind::Eof | TokenKind::CloseParen | TokenKind::Newline => break,
                _ => {
                    let op = match self.match_operator(op_token.kind) {
                        Some(op) => op,
                        None => {
                            let error = self.error(ErrorKind::ExpectedOperator, op_token);
                            self.recover();
                            return Some(Err(error));
                        }
                    };

                    op
                }
            };



            let (l_bp, r_bp): (BP, BP) = op.get_bp();
            if min_bp > l_bp {
                break;
            };

            if is_compound {
                self.next();
                self.next();
            } else {
                self.next();
            };


            let rhs = self.pratt_parse(r_bp);
            let rhs = match rhs {
                Some(Ok(exp)) => {
                    exp
                }
                err @ Some(Err(_)) => return err,
                None => {
                    let error = self.error(ErrorKind::ExpectedExpression, self.none());
                    return Some(Err(error));
                }
            };

            lhs = Expr::bin_op(BinOpn::new(op, lhs, rhs));
        }

        Some(Ok(lhs))
    }

    pub fn parse(&mut self) {
        let mut errors = VecDeque::<ParserError>::new();

        self.flush_newlines();
        while let Some(result) = self.pratt_parse(0) {
            match result {
                Ok(_) => (),
                Err(e) => errors.push_back(e),
            };
            self.flush_newlines();
            print!("\n\n");
        }

        let diagnostics = Diagnostics::new(errors, self.src);


        diagnostics.report_all();
    }
}

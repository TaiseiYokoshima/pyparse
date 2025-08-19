use std::{collections::VecDeque, ops::Range};
use super::{ast::{BinOpn, BinOpr, Expr, Literal, LiteralKind, BP}, Diagnostics};
use crate::lex::{Lexer, Token, TokenKind, TokenStream};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    ExpectedExpression,
    ExpectedOperator,
    ExpectedCloseParen,
}

#[derive(Debug)]
pub struct ParserError {
    pub kind: ErrorKind,
    pub index: usize,
    pub token: Option<Token>,
}

pub struct Parser<'src> {
    src: &'src str,
    pub tokens: VecDeque<Token>,
    temp: Option<Token>,
    index: usize,
    
    current_line_number: usize,
    current_line_start_index: usize,

    line_indices: VecDeque<Range<usize>>,
}

impl<'src> Parser<'src> {

    #[inline(always)]
    fn error(&self, kind: ErrorKind, token_op: Option<Token>) -> ParserError {
        ParserError {
            kind,
            index: self.index,
            token: token_op,
        }
    }


    #[inline(always)]
    fn recover(&mut self) {
        while let Some(token) = self.next() {
            if token.kind == TokenKind::Newline {
                self.newline();
                return;
            };
        };

    }

    #[inline(always)]
    fn error_and_recover(&mut self, kind: ErrorKind, token_op: Option<Token>) -> ParserError {
        let err = self.error(kind, token_op);
        self.recover();
        err
    }

    pub fn new<T: Into<(&'src str, VecDeque<Token>)>>(input: T) -> Self {
        let (src, tokens) = input.into();
        assert!(!tokens.is_empty());

        Self {
            src,
            tokens,
            index: 0,
            temp: None,
            current_line_number: 0,
            current_line_start_index: 0,
            line_indices: VecDeque::new(),
        }
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.temp {
            self.temp = None;
            return Some(token);
        };

        self.consume_next()
    }

    #[inline(always)]
    fn consume_next(&mut self) -> Option<Token> {
        let token = loop {
            let token = self.tokens.pop_front()?;
            self.index += token.size;

            if token.kind != TokenKind::WhiteSpace {
                break token;
            };
        };

        Some(token)
    }



    #[inline(always)]
    fn flush_newlines(&mut self) {
        while let Some(token) = self.consume_next() {
            if token.kind != TokenKind::Newline {
                self.set_temp(token);
                return;
            };
            self.newline();
        }
    }

    #[inline(always)]
    fn set_temp(&mut self, token: Token) {
        self.temp = Some(token);
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<&Token> {
        if let Some(ref token) = self.temp {
            return Some(token);
        };

        let token = self.consume_next()?;
        self.set_temp(token);
        self.temp.as_ref()
    }

    fn match_operator(&mut self, token_kind: TokenKind) -> Option<BinOpr> {
        match token_kind {
            TokenKind::Plus => Some(BinOpr::Add),
            TokenKind::Minus => Some(BinOpr::Sub),
            TokenKind::Percent => Some(BinOpr::Mod),
            TokenKind::Star => {
                if let Some(next_token) = self.next() {
                    if token_kind == next_token.kind {
                        Some(BinOpr::Exp)
                    } else {
                        self.set_temp(next_token);
                        Some(BinOpr::Mul)
                    }
                } else {
                    None
                }
            }
            TokenKind::Slash => {
                if let Some(next_token) = self.next() {
                    if token_kind == next_token.kind {
                        Some(BinOpr::Flo)
                    } else {
                        self.set_temp(next_token);
                        Some(BinOpr::Div)
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }



    fn get_range(&self, token: &Token) -> Range<usize> {
        let end = self.index;
        let start = end - token.size;

        Range {
            start,
            end
        }
    }



    #[inline(always)]
    fn newline(&mut self) {
        self.line_indices.push_back(Range {start: self.current_line_start_index, end: self.index - 1 });
        self.current_line_number += 1;
        self.current_line_start_index = self.index;
    }

    fn pratt_parse(&mut self, min_bp: BP) -> Result<Option<Expr>, ParserError> {
        let token = match self.next() {
            Some(token) => token,
            None => return Ok(None),
        };

        let mut lhs = match token.kind {
            TokenKind::Newline => {
                self.newline();
                return Ok(None);
            },
            TokenKind::Number => Expr::Literal(Literal::new(LiteralKind::Number,self.get_range(&token))),
            TokenKind::Ident => Expr::Ident(self.get_range(&token)),
            TokenKind::OpenParen => {
                // first check expr for Err and None,
                let expr_opt = self.pratt_parse(min_bp);
                let expr = match expr_opt {
                    Ok(Some(expr)) => expr,
                    Ok(None) => {
                        let err = self.error(ErrorKind::ExpectedExpression, None);
                        self.recover();
                        return Err(err);
                    }
                    err @ Err(_) => return err,
                };

                // check next token exists and is a close paren
                match self.next() {
                    Some(token) => {
                        // error origin: got some token but wasn't close paren
                        match token.kind {
                            TokenKind::CloseParen => (),
                            TokenKind::Newline => return Err(self.error(ErrorKind::ExpectedCloseParen, None)),
                            _ => {
                                let err = self.error(ErrorKind::ExpectedCloseParen, Some(token));
                                self.recover();
                                return Err(err);
                            },
                        }
                    }
                    None => {
                        let err = self.error(ErrorKind::ExpectedCloseParen, None);
                        return Err(err);
                    }
                };

                expr
            }

            _ => {
                let error = self.error(ErrorKind::ExpectedExpression, Some(token));
                self.recover();
                return Err(error);
            }
        };

        loop {
            let op_token = match self.next() {
                Some(op_token) => op_token,
                None => break,
            };

            let op = match op_token.kind {
                TokenKind::Newline => break,
                TokenKind::CloseParen => {
                    self.set_temp(op_token);
                    break;
                }
                _ => {
                    let op = match self.match_operator(op_token.kind) {
                        Some(op) => op,
                        None => {
                            let error = self.error(ErrorKind::ExpectedOperator,Some(op_token));
                            self.recover();
                            return Err(error);
                        }
                    };

                    op
                }
            };

            let (l_bp, r_bp): (BP, BP) = op.get_bp();

            if min_bp > l_bp {
                self.set_temp(op_token);
                break;
            };

            let rhs = match self.pratt_parse(r_bp)? {
                Some(exp) => exp,
                None => {
                    let error = self.error(ErrorKind::ExpectedExpression,None);
                    return Err(error);
                }
            };

            lhs = Expr::BinOp(Box::new(BinOpn::new(op, lhs, rhs)));
        }
        Ok(Some(lhs))
    }

    pub fn parse(&mut self) {
        let mut errors = VecDeque::<ParserError>::new();

        loop {
            self.flush_newlines();
            match self.pratt_parse(0) {
                Ok(Some(expr)) => println!("expr: {:?}", expr),
                Ok(None) => break,
                Err(e) => errors.push_back(e),
            }
        };

       let diagnostics = Diagnostics::new(errors, self.src, &self.line_indices);
       diagnostics.report_all();

    }
}

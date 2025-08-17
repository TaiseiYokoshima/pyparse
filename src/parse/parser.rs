use std::collections::VecDeque;

use super::ast::{BP, BinOpn, BinOpr, Expr, Literal};
use crate::lex::{Lexer, Token, TokenKind, TokenStream};



#[derive(Debug)]
pub enum NumberError {
    MultipleDots(usize),
    TooManyDigits,
    NotDigit(usize),
}


#[derive(Debug)]
pub enum IdentError {
    InvalidStart,
    InvalidChar(usize)

}

#[derive(Debug)]
pub enum ErrorKind {
    ExpectedExpression,
    ExpectedOperator,
    ExpectedCloseParen,

    NumberError(NumberError),
    IdentError(IdentError)
}

#[derive(Debug)]
pub struct ParserError {
    error_kind: ErrorKind,
    start: usize,
    token_set: Option<(TokenKind, usize)>,
}

pub struct Parser<'src> {
    src: &'src str,
    pub tokens: VecDeque<Token>,
    temp: Option<Token>,
    index: usize,
    // errors: VecDeque<ParserError>,
}

impl<'src> Parser<'src> {
    pub fn new<T: Into<(&'src str, VecDeque<Token>)>>(input: T) -> Self {
        let (src, tokens) = input.into();
        assert!(!tokens.is_empty());

        Self {
            src,
            tokens,
            index: 0,
            temp: None,
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
            self.index += token.len;

            if token.kind != TokenKind::WhiteSpace {
                break token;
            };

        };

        Some(token)
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

    #[inline(always)]
    fn str(&self, token: &Token) -> &'src str {
        let src = self.src;
        let index = self.index;
        &src[index - token.len..index]
    }

    // #[inline(always)]
    // fn push_get_err(&mut self, error: ParserError) -> &ParserError {
    //     self.errors.push_back(error);
    //     self.errors.get(self.errors.len() - 1).expect("I pushed an error just now but got none")
    // }

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

    fn parse_number(&self, token: &Token) -> Result<Literal, ParserError> {
        let string = self.str(token);
        let mut found_dot = false;
        let mut count = 0_usize;
        for char in string.chars() {
            match char {
                '.' => {
                    if found_dot {
                        return Err(ParserError {
                            start: self.index - token.len,
                            error_kind: ErrorKind::NumberError(NumberError::MultipleDots(count)),
                            token_set: Some((token.kind, self.index)),
                        });
                    };

                    found_dot = true;
                }
                '0'..='9' => (),
                _ => {
                    return Err(ParserError {
                        error_kind: ErrorKind::NumberError(NumberError::NotDigit(count)),
                        start: self.index - token.len,
                        token_set: Some((token.kind, self.index)),
                    });
                }
            };
            count += 1;
        };


        let float= string.parse::<f32>().expect("Should not have errored here, should have not been empty, nor wrong literal");

        let output = if float.is_infinite() {
            Err(ParserError {
                error_kind: ErrorKind::NumberError(NumberError::TooManyDigits),
                start: self.index - token.len,
                token_set: Some((token.kind, self.index)),
            })

        } else {
            Ok(Literal::Number(float))
        };

        output
    }

    fn parse_ident(&self, token: &Token) -> Result<Box<str>, ParserError> {
        let string = self.str(token);
        let mut iter = string.chars();

        let first = iter.next().expect("Not supposed to be empty");

        if first.is_numeric() || (first != '_' && !first.is_alphabetic()) {
            return Err(ParserError {
                error_kind: ErrorKind::IdentError(IdentError::InvalidStart),
                start: self.index - token.len,
                token_set: Some((token.kind, self.index))
            });
        };

        let mut count: usize = 1;

        for char in iter {
            if !char.is_ascii_alphanumeric() && char != '_' {
                return Err(ParserError {
                    error_kind: ErrorKind::IdentError(IdentError::InvalidStart),
                    start: self.index - token.len,
                    token_set: Some((token.kind, self.index))
                });
            };

            count += 1;
        };


        Ok(string.to_string().into_boxed_str())
    }




    fn pratt_parse(&mut self, min_bp: BP) -> Result<Option<Expr>, ParserError> {
        let token = match self.next() {
            Some(token) => token,
            None => return Ok(None),
        };

        let mut lhs = match token.kind {
            TokenKind::Newline => return Ok(None),
            TokenKind::Number => Expr::Literal(self.parse_number(&token)?),
            TokenKind::Ident => Expr::Ident(self.parse_ident(&token)?),
            TokenKind::OpenParen => {
                // first check expr for Err and None,
                let expr_opt = self.pratt_parse(min_bp);
                let expr = match expr_opt {
                    Ok(Some(expr)) => expr,
                    Ok(None) => {
                        let error = ParserError {
                            error_kind: ErrorKind::ExpectedExpression,
                            start: self.index,
                            token_set: None,
                        };
                        return Err(error);
                    }
                    err @ Err(_) => return err,
                };

                // check next token exists and is a close paren
                match self.next() {
                    Some(token) => {
                        // error origin: got some token but wasn't close paren
                        if token.kind != TokenKind::CloseParen {
                            let error = ParserError {
                                error_kind: ErrorKind::ExpectedCloseParen,
                                start: self.index - token.len,
                                token_set: Some((token.kind, self.index)),
                            };
                            return Err(error);
                        };
                    }
                    None => {
                        let error = ParserError {
                            error_kind: ErrorKind::ExpectedCloseParen,
                            start: self.index,
                            token_set: None,
                        };
                        return Err(error);
                    }
                };

                expr
            }

            _ => {
                let error = ParserError {
                    error_kind: ErrorKind::ExpectedExpression,
                    start: self.index - token.len,
                    token_set: Some((token.kind, self.index)),
                };

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
                            let error = ParserError {
                                error_kind: ErrorKind::ExpectedOperator,
                                start: self.index - op_token.len,
                                token_set: Some((op_token.kind, self.index)),
                            };

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
                    let error = ParserError {
                        error_kind: ErrorKind::ExpectedExpression,
                        start: self.index,
                        token_set: None,
                    };

                    return Err(error);
                }
            };

            lhs = Expr::BinOp(Box::new(BinOpn::new(op, lhs, rhs)));
        }
        Ok(Some(lhs))
    }

    pub fn parse(&mut self) -> Result<Option<Expr>, ParserError> {
        self.pratt_parse(0)
    }
}

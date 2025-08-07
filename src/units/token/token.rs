use core::fmt;

use crate::SyntaxError;
use crate::units::{Keyword, Operator};

use crate::errors::syntax::{ErrorKind, IdentError, NumberError};

use crate::SrcSpan;

pub struct Token<'src> {
    span: SrcSpan<'src>,
    kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Space,
    Newline,

    LParen,
    RParen,

    Operator(Operator),
    Number(f32),
    Keyword(Keyword),
    Identifier,
}

impl<'src> Token<'src> {
    fn parse_number_literal(span: SrcSpan<'src>) -> Result<Token, SyntaxError<'src>> {
        let mut found_dot = false;

        for char in src.chars() {
            if char == '.' {
                if found_dot {
                    return Err(NumberError::TooManyDigits);
                };
                found_dot = true;
                continue;
            };
        }
    }

    fn parse_identifier(span: SrcSpan<'src>) -> Result<Token, SyntaxError<'src>> {
        for char in span.chars() {
            if !char.is_alphanumeric() && char != '_' {
                return Err(SyntaxError::new(span, ErrorKind::IdentifierError);
            };
        }
    }
    

    pub fn new(span: SrcSpan<'src>) -> Result<Self, SyntaxError<'src>> {
        let src = span.as_ref();

        if let Some(operator) = Operator::new(src) {
            return Ok(Self {
                kind: TokenKind::Operator(operator),
                span,
            });
        };

        let mut iter = src.chars();
        let first = iter.next().unwrap();

        let output = if first.is_numeric() || first == '.' {
            Self::parse_number_literal(span)
        } else if first.is_alphabetic() || first == '_' {
            Self::parse_identifier(span)
        } else {
            Err(SyntaxError::new(span, ErrorKind::UnexpectedToken))
        };

        output
    }
}

impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::Operator(operator) => write!(f, "Opr({})", operator),
            TokenKind::Number(number) => write!(f, "Num({})", number),
            TokenKind::Space => write!(f, "Space"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::Identifier => write!(f, "Idn({})", self.span),
            TokenKind::Keyword(keyword) => write!(f, "Key({})", keyword),
        }
    }
}

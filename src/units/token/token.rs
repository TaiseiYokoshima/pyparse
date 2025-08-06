use core::fmt;

use crate::errors::SyntaxError;
use crate::units::{Keyword, Operator};


#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Space(&'src str),
    Newline(&'src str),

    LParen(&'src str),
    RParen(&'src str),

    Operator(&'src str, Operator),

    Number(&'src str, f32),
    Keyword(&'src str, Keyword),
    Identifier(&'src str),
}

impl<'src> Token<'src> {
    pub fn new(src: &'src str) -> Result<Self, SyntaxError<'src>> {
        if let Some(operator) = Operator::new(src) {
            return Ok(Self::Operator(src, operator));
        };

        let mut iter = src.chars();
        let first = iter.next().unwrap();

        if first.is_numeric() {
            match src.parse::<f32>() {
                Ok(value) => return Ok(Token::Number(src, value)),
                Err(f) => return Err(SyntaxError::NumberError(src, f.to_string())),
            };
        };

        if first.is_alphabetic() || first == '_' {
            for char in src.chars() {
                if !char.is_alphanumeric() && char != '_' {
                    return Err(SyntaxError::IdentifierError(src));
                }
            }
        };

        return Ok(Token::Identifier(src));
    }
}


impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LParen(_) => write!(f, "LParen"),
            Self::RParen(_) => write!(f, "RParen"),
            Self::Operator(_, operator) => write!(f, "Opr({})", operator),
            Self::Number(_, number) => write!(f, "Num({})", number),
            Self::Space(_) => write!(f, "Space"),
            Self::Newline(_) => write!(f, "Newline"),
            Self::Identifier(str) => write!(f, "Idn({})", str),
            Self::Keyword(_, keyword) => write!(f, "Key({})", keyword),
        }
    }
}

use core::fmt;

use crate::SyntaxError;
use crate::units::{Keyword, Operator};

use crate::errors::syntax as syn_errors;

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
    fn parse_number_literal(src: &str) -> Result<Token, SyntaxError<'src>> {
        let mut found_dot = false;

        for char in src.chars() {
            if char == '.' {
                if found_dot {
                    return Err(syn_errors::NumberError::TooManyDigits);
                };
                found_dot = true;
                continue;
            };

        };
        



    }


    pub fn new(src: &'src str) -> Result<Self, SyntaxError<'src>> {
        if let Some(operator) = Operator::new(src) {
            return Ok(Self::Operator(src, operator));
        };

        let mut iter = src.chars();
        let first = iter.next().unwrap();

        let output = if first.is_numeric() || first == '.' {
            Self::parse_number_literal(src)
        } else if first.is_alphabetic() || first == '_' {
            for char in src.chars() {
                if !char.is_alphanumeric() && char != '_' {
                    return Err(SyntaxError::IdentifierError(src));
                };
            };
            Ok(Token::Identifier(src))
        } else {
            Err(SyntaxError::UnexpectedToken(src))
        };

        output
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

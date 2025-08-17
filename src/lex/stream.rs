use std::collections::VecDeque;
use std::fmt::{self, Write};

use crate::lex::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub struct Tokens(VecDeque<Token>);
impl From<Lexer<'_>> for Tokens {
    fn from(value: Lexer<'_>) -> Self {
        Self(value.tokens)
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tokens [\n")?;
        for token in &self.0 {
            write!(f, "{}\n", token)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
pub struct TokenStream<'src> {
    src: &'src str,
    pub stream: VecDeque<Token>,
}

impl<'src> From<Lexer<'src>> for TokenStream<'src> {
    fn from(value: Lexer<'src>) -> Self {
        Self {
            src: value.src,
            stream: value.tokens,
        }
    }
}

impl<'src> fmt::Display for TokenStream<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let src = self.src;
        let mut s = String::new();
        write!(s, "Tokens: [")?;

        let mut index = 0;
        for token in &self.stream {
            let len = token.len;

            write!(s, " ")?;
            match token.kind {
                TokenKind::Dot => write!(s, "`.`")?,
                TokenKind::Plus => write!(s, "`+`")?,
                TokenKind::Minus => write!(s, "`-`")?,
                TokenKind::Star => write!(s, "`*`")?,
                TokenKind::Slash => write!(s, "`/`")?,
                TokenKind::Percent => write!(s, "`%`")?,
                TokenKind::OpenParen => write!(s, "`(`")?,
                TokenKind::CloseParen => write!(s, "`)`")?,
                TokenKind::Newline => write!(s, "Newline")?,
                TokenKind::Semi => write!(s, "`;`")?,
                TokenKind::Colon => write!(s, "`:`")?,
                TokenKind::Comma => write!(s, "`,`")?,

                TokenKind::Number => write!(s, "Number({})", &src[index..index + len])?,
                TokenKind::WhiteSpace => write!(s, "WhiteSpace({})", &src[index..index + len])?,
                TokenKind::Ident => write!(s, "Ident({})", &src[index..index + len])?,
                TokenKind::InvalidChar => write!(s, "Invalid_Char({})", &src[index..index + len])?,
            };

            write!(s, ",")?;
            index += len;
        }

        s.pop();
        write!(s, " ]")?;
        writeln!(f, "{}", s)
    }
}

impl<'src> Into<(&'src str, VecDeque<Token>)> for TokenStream<'src> {
    fn into(self) -> (&'src str, VecDeque<Token>) {
        (self.src, self.stream)
    }
}

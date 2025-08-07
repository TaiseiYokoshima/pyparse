use std::fmt;
use std::fmt::Write;


use std::collections::VecDeque;

use crate::{Source, SyntaxError};
use crate::units::TokenKind;



pub struct Lexer<'src> {
    src: &'src Source,
    start: usize,
    end: usize,
    tokens: VecDeque<TokenKind<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src Source) -> Self {
        Self {
            src,
            start: 0,
            end: 0,
            tokens: VecDeque::default(),
        }
    }

    pub fn tokenize(&mut self) -> Result<(), SyntaxError<'src>> {
        while let Some(char) = self.peek_char() {
            self.parse_char(char)?;
        }

        self.parse_token()
    }
    fn peek_char(&self) -> Option<char> {
        self.src.src[self.end..].chars().next()
    }

    fn parse_char(&mut self, peeked: char) -> Result<(), SyntaxError<'src>> {
        match peeked {
            ' ' => {
                self.parse_token()?;
                self.advance(peeked);
                self.reset_builder();
                Ok(())
            }
            '+' | '-' | '%' | '\n' => {
                self.parse_token()?;
                self.advance(peeked);
                self.parse_token()?;
                Ok(())
            }
            '*' | '/' => {
                self.parse_token()?;
                self.advance(peeked);

                if let Some(second_peeked) = self.peek_second() {
                    if peeked == second_peeked {
                        self.advance(peeked);
                    }
                };
                self.parse_token()?;
                Ok(())
            }
            '_' | '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' => {
                self.advance(peeked);
                Ok(())
            }
            _ => Err(SyntaxError::IdentifierError(self.current_peek(peeked))),
        }
    }

    fn parse_token(&mut self) -> Result<(), SyntaxError<'src>> {
        if self.start == self.end {
            return Ok(());
        }

        let str = self.current_str();
        let token = TokenKind::new(str)?;
        self.tokens.push_back(token);
        self.reset_builder();
        Ok(())
    }

    fn peek_second(&self) -> Option<char> {
        let mut it = self.src[self.end..].chars();
        it.next()?;
        it.next()
    }

    fn advance(&mut self, char: char) {
        self.end += char.len_utf8();
    }

    fn reset_builder(&mut self) {
        self.start = self.end;
    }

    fn current_str(&self) -> &'src str {
        &self.src[self.start..self.end]
    }

    fn current_peek(&self, peeked: char) -> &'src str {
        &self.src[self.end..peeked.len_utf8()]
    }
}


impl<'src> fmt::Display for Lexer<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut s = String::new();
        write!(s, "Tokens [")?;
        for token in &self.tokens {
            write!(s, " {},", token)?
        };
        s.pop();
        write!(f, "{} ]", s)
    }

}


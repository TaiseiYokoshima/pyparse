use std::collections::VecDeque;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result};

use crate::tokenizer::Lexer;

use crate::units::TokenKind;


struct Span<'src> {
    src: &'src String,
    start: usize,
    end: usize,
}


struct Source<'src>{
    src: &'src String,
    start: usize,
    end: usize,
}


impl<'src> Source<'src> {
    fn new(src: &'src String) -> Self {
        Self {
            src,
            start: 0,
            end: 0,
        }
    }

    pub fn parsed_token(&mut self) {
        self.start = self.end;
    }

    pub fn move_one(&mut self, length: usize) {
        self.start+=length;
        self.end+=length;
    }

    pub fn get_current(&self) -> &str {
        &self.src[self.start..self.end]
    }

    pub fn peek(&self) -> Option<char> {
        self.src[self.end..].chars().next()
    }

    pub fn advance(&mut self, length: usize) {
        self.end += length;
    }
}





#[derive(Debug)]
pub struct Tokenizer {
    pub tokens: VecDeque<TokenKind>,
    builder: Box<String>,
    src: String,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Tokenizer {
            src,
            tokens: VecDeque::default(),
            builder: Box::default(),
        }
    }

    pub fn tokenize(mut self) -> Lexer {
        while let Some(char) = self.advance() {
            self.parse_char(char);
        }

        self.parse_token();
        return self.into();
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        match self.peek() {
            Some(char) => {
                self.src.drain(..char.len_utf8());
                Some(char)
            }
            None => None,
        }
    }

    fn parse_char(&mut self, char: char) {
        if let Some(deliminiter) = TokenKind::match_deliminiter(&char, self.peek()) {
            if !self.builder.is_empty() {
                self.parse_token();
            };

            if deliminiter != TokenKind::Space {
                self.add_token(deliminiter);
            }

            return;
        };

        self.builder.push(char);
    }

    fn add_token(&mut self, token: TokenKind) {
        // println!("parsed: {:?}", token);
        self.tokens.push_back(token);
    }

    fn parse_token(&mut self) {
        if self.builder.is_empty() {
            return;
        }

        let token = TokenKind::match_token(&mut self.builder);
        self.add_token(token);
    }
}

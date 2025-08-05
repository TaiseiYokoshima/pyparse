use std::collections::VecDeque;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result};

use crate::units::Token;

#[derive(Debug)]
pub struct Lexer {
    tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn next(&mut self) -> Token {
        self.tokens
            .pop_front()
            .expect("None unwrapped in lexer next")
    }

    pub fn peek(&mut self) -> &Token {
        self.tokens.get(0).expect("None unwrapped in lexer peek")
    }

    pub fn peek_at(&mut self, index: usize) -> &Token {
        self.tokens
            .get(index)
            .expect("None unwrapped in lexer peek")
    }
}

impl Display for Lexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut temp = String::new();
        write!(temp, "Tokens: [")?;
        self.tokens
            .iter()
            .try_for_each(|token| write!(temp, " {},", token))?;

        temp.pop();
        write!(temp, " ]")?;
        write!(f, "{}", temp)
    }
}

impl From<Tokenizer> for Lexer {
    fn from(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokens: tokenizer.tokens,
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: VecDeque<Token>,
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
        while let Some(char) = self.src.chars().next() {
            self.src.drain(..char.len_utf8());
            self.parse_char(char);
        }

        self.parse_token();
        return self.into();
    }

    fn parse_char(&mut self, char: char) {
        if let Some(deliminiter) = Token::match_deliminiter(&char) {
            if !self.builder.is_empty() {
                self.parse_token();
            };

            if deliminiter != Token::Space {
                self.add_token(deliminiter);
            }

            return;
        };

        self.builder.push(char);
    }

    fn add_token(&mut self, token: Token) {
        println!("parsed: {:?}", token);
        self.tokens.push_back(token);
    }

    fn parse_token(&mut self) {
        if self.builder.is_empty() {
            return;
        }

        let token = Token::match_token(&mut self.builder);
        self.add_token(token);
    }
}

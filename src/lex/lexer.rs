use std::fmt;
use std::fmt::Write;

use std::collections::VecDeque;

use crate::errors::syntax::ErrorKind;
use crate::units::{Token, TokenKind};
use crate::{Span, SyntaxError};

use crate::units::{ColumnRange, LineByteRange, TokenSpan};

pub struct Lexer<'src> {
    pub src: &'src str,
    pub span: TokenSpan,
    pub line_num: usize,
    pub line_range: LineByteRange,
    line_map: Vec<LineByteRange>,
    tokens: VecDeque<Token>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            span: TokenSpan::default(),
            line_num: 1,
            line_range: LineByteRange::default(),
            tokens: VecDeque::default(),
            line_map: vec![],
        }
    }

    pub fn tokenize(&mut self) -> Result<(), SyntaxError> {
        while let Some(char) = self.peek_char() {
            self.parse_char(char)?;
        }

        self.parse_token()
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.span.end..].chars().next()
    }


    fn parse_num(&mut self, peeked: char) -> Result<(), SyntaxError> {


    }


    fn parse_char(&mut self, peeked: char) -> Result<(), SyntaxError> {
        let length = peeked.len_utf8();
        match peeked {
            ' ' => {
                self.parse_token()?;
                self.skip(length);
                Ok(())
            }

            '\n' => {
                self.parse_token()?;
                self.consume_char(length);
                self.parse_token()?;
                self.matched_newline();
                Ok(())
            }

            '+' | '-' | '%' => {
                self.parse_token()?;
                self.consume_char(length);
                self.parse_token()?;
                Ok(())
            }

            '*' | '/' => {
                self.parse_token()?;
                self.consume_char(length);

                if let Some(second_peeked) = self.peek_second() {
                    if peeked == second_peeked {
                        self.consume_char(length);
                    }
                };
                self.parse_token()?;
                Ok(())
            }

            '.' => {
                
            }

            '_' | '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                self.consume_char(length);
                Ok(())
            }

            _ => Err(todo!()),
        }
    }

    fn parse_token(&mut self) -> Result<(), SyntaxError> {
        let span = &mut self.span;
        let start = span.start;
        let end = span.end;

        if start == end {
            return Ok(());
        };

        let src = self.src;

        let kind = TokenKind::new(src, span)?;

        let span = span.parsed_token();
        let token = Token::new(span, kind);

        self.tokens.push_back(token);
        Ok(())
    }

    fn skip(&mut self, offset: usize) {
        self.span.skip(offset);
    }

    fn consume_char(&mut self, offset: usize) {
        self.span.consume_char(offset);
    }

    fn matched_newline(&mut self) {
        let offset = self.span.end;
        let line_range = self.line_range.parsed_newline(offset);
        self.line_map.push(line_range);
        self.line_num+=1;
    }

    fn peek_first


    fn current_peek(&'src self, peeked: char) -> &'src str {
        &self.src[self.end..self.end + peeked.len_utf8()]
    }
}

use crate::{Lexer, Span};
use std::ops::Range;

use crate::errors::syntax::{ErrorKind, NumberError};

use crate::units::TokenKind;

use crate::units::TokenSpan;

pub struct DotIterator<'src, 'span> {
    src: &'src str,
    span: &'span mut TokenSpan,
    char: char,
    current: usize,
}

impl<'src, 'span> DotIterator<'src, 'span> {
    pub fn parse(src: &'src str, span: &'span mut TokenSpan) -> Result<TokenKind, ErrorKind> {
        let mut it = Self {
            span,
            src,
            char: '.',
            current: span.start,
        };

        // let first_char =
    }

    pub fn get_current_index(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end,
        }
    }

    fn parse_number(&mut self) -> Result<TokenKind, ErrorKind> {
        let src = self.lex.src;

        let byte_count = 0;
        for char in src[self.lex.start..self.lex.end].chars() {
            if char == '.' {
                let err = NumberError::MultipleDots(Range {
                    start: self.lex.start,
                });
            };
        }
    }
}

impl Iterator for DotIterator<'_, '_> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let span = &mut self.span;
        let src = self.src;

        let char = self.src[self.current..].chars().next()?;
        let length = char.len_utf8();

        if span.end == self.current {
            span.consume_char(length);
            self.current = span.end;
        };

        Some(char)
    }
}

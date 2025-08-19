use crate::lex::TokenKind;
use std::fmt::Write;

use super::{ErrorKind, ParserError};
use std::{collections::VecDeque, ops::Range};

pub struct Diagnostics<'src, 'lines> {
    errors: VecDeque<ParserError>,
    src: &'src str,
    lines: &'lines VecDeque<Range<usize>>,
}

impl<'src, 'lines> Diagnostics<'src, 'lines> {
    pub fn new(
        errors: VecDeque<ParserError>,
        src: &'src str,
        lines: &'lines VecDeque<Range<usize>>,
    ) -> Self {
        Self { errors, src, lines }
    }

    pub fn find_line_none(&self, start: usize) -> &Range<usize> {
        for line in self.lines {
            if start < line.end {
                return line;
            };
        }

        panic!("Could not find line")
    }

    pub fn find_line_some(&self, start: usize, end: usize) -> (usize, Range<usize>) {
        for (number, range) in self.lines.iter().enumerate() {
            if start < range.end {
                if end > range.end {
                    panic!("Error token crossed line boundaries")
                };

                return (number, range.clone());
            };
        }

        panic!("Could not find line")
    }

    fn formatted_error_some(
        &self,
        number: usize,
        line: Range<usize>,
        error: ErrorKind,
        token: TokenKind,
        start: usize,
        end: usize,
    ) {
        let line_str = &self.src[line.clone()];
        let (start, end) = (start - line.start, end - line.start);


        let mut second_line = Vec::new();
        let mut index = 0;

        let mut column_start = 0;
        let mut column_count = 0;

        for (column, char) in line_str.chars().enumerate() {
            if index >= start && index < end {
                column_count += 1;
                match second_line.get(second_line.len() - 1) {
                    Some('-') => second_line.push('-'),
                    Some('^') => {
                        second_line.pop();
                        second_line.push('-');
                        second_line.push('-');
                    }
                    _ => {
                        column_start = column + 1;
                        second_line.push('^');
                    }
                };
            } else {
                second_line.push(' ');
            };

            index += char.len_utf8();
        }


        let msg = match error {
            ErrorKind::ExpectedOperator => "expected one of `+`, `-`, `*`, `/`, `%`, `.`,",
            ErrorKind::ExpectedExpression => "expected an expression",
            ErrorKind::ExpectedCloseParen => "could not find the closing parenthesis",
        };

        println!("Error: {}", msg);
        println!("Filename:{}:{}", number + 1, column_start,);



        // println!("error: {:?}", kind, );
        println!("{}", line_str);
        println!("{}", second_line.into_iter().collect::<String>());


        let mut msg = String::new();
        match error {
            ErrorKind::ExpectedCloseParen => write!(msg, "this parenthesis is not closed"),
            _ => write!(msg, "but found {:?} instead", token),
        }.expect("unwrapped fail when writting error");
        println!("{}", msg);
    }

    pub fn report(&self, error: &ParserError) {
        let line = match error.token {
            Some(token) => {

                let end = error.index;
                let start = error.index - token.size;
                let (number, line) = Self::find_line_some(&self, start, end);

                self.formatted_error_some(number, line, error.kind, token.kind, start, end);
            }
            None => {
                let line = Self::find_line_none(&self, error.index);
            }
        };
    }

    pub fn report_all(&self) {
        for error in &self.errors {
            self.report(&error);
        }
    }
}

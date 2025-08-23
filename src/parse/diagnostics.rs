use crate::{lex::TokenKind, source::Source};
use std::fmt::Write;

use super::{ErrorKind, ParserError};
use std::{collections::VecDeque, ops::Range};

pub struct Diagnostics<'src> {
    pub errors: VecDeque<ParserError>,
    src: &'src Source,
}

impl<'src> Diagnostics<'src> {
    pub fn new(errors: VecDeque<ParserError>, src: &'src Source) -> Self {
        Self { errors, src, }
    }

    fn formatted_error_some(&self, number: usize, line: Range<usize>, error: &ParserError) {
        let start = error.range.start - line.start;
        let end = error.range.end - line.start;
        let line_str = self.src.line_str(&line);

        println!("start: {}", start);
        println!("end: {}", end);
        return;





        let mut second_line = Vec::new();

        let mut index = 0;
        let mut column_start = 0;
        let mut column_count = 0;

        for (column, char) in line_str.chars().enumerate() {
            if index >= start && index <= end {
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
                    },
                };

            } else {
                second_line.push(' ');
            };

            index += char.len_utf8();
        };




        let msg = match error.error {
            ErrorKind::ExpectedOperator => "expected one of `+`, `-`, `*`, `/`, `%`, `.`,",
            ErrorKind::ExpectedExpression => "expected an expression",
            ErrorKind::ExpectedCloseParen => "could not find the closing parenthesis",
        };

        println!("Error: {}", msg);
        println!("Filename:{}:{}", number, column_start);



        // println!("error: {:?}", kind, );
        println!("{:?} ", &line_str);
        println!("{:?}", second_line.into_iter().collect::<String>());


        let mut msg = String::new();
        match error.error {
            ErrorKind::ExpectedCloseParen => write!(msg, "this parenthesis is not closed"),
            _ => write!(msg, "but found {:?} instead", error.token),
        }.expect("unwrapped fail when writting error");
        println!("{}", msg);
    }

    pub fn report(&self, error: &ParserError) {
        // println!("{:?}", error);

        let (number, line) = self.src.line_range(&error.range);
        self.formatted_error_some(number, line, error);
    }

    pub fn report_all(&self) {
        let mut index = 0;
        for error in &self.errors {
            print!("{}", index);
            self.report(&error);
            index += 1;
        }
    }
}

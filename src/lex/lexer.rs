use std::collections::VecDeque;
use std::str::Chars;

use crate::lex::{Token, TokenKind};

pub struct Lexer<'src> {
    pub src: &'src str,
    pub tokens: VecDeque<Token>,
    it: Chars<'src>,
    temp_char: Option<char>,
    debug: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Lexer<'src> {
        let it = src.chars();
        let tokens = VecDeque::default();
        let temp_char = None;

        Self {
            src,
            it,
            tokens,
            temp_char,
            debug: false,
        }
    }

    #[inline]
    fn push(&mut self, kind: TokenKind, size: usize, count: usize) {
        let token = Token::new(kind, size, count);
        if self.debug {
            println!("Token: {}", &token)
        };
        self.tokens.push_back(token);
    }


    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(temp_char) = self.temp_char {
            if self.debug { println!("got temp: {}", temp_char) };;
            self.temp_char = None;
            return Some(temp_char);
        };
        
        if self.debug { println!("going to consuem next char") };
        self.it.next()
    }

    #[inline]
    fn set_temp(&mut self, temp_char: char) {
        self.temp_char = Some(temp_char);
        if self.debug {
            println!("set temp: {:?}", temp_char)
        }
    }

    fn parse_ident(&mut self) {
        let mut size = 1;
        let mut count = 1;
        while let Some(char) = self.next() {
            match char {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => { 
                    size += 1; 
                    count += 1;
                },
                '.' => {
                    self.push(TokenKind::Ident, size, count);
                    self.push(TokenKind::Dot, 1, 1);
                    return;
                },

                '\n' | ' ' | '(' | ')' | '+' | '-' | '*' | '/' | '%' => {
                    self.push(TokenKind::Ident, size, count);
                    self.set_temp(char);
                    return;
                },
                _ => {
                    size += char.len_utf8();
                    count += 1;
                },
            };
        }
    }

    fn parse_number(&mut self, found_dot: bool) {
        let (mut size, mut count) = if found_dot { (2, 2) } else { (1, 1) };
        while let Some(char) = self.next() {
            match char {
                '0'..='9' | '.' | '_' | 'a'..='z' | 'A'..='Z' => {
                    size += 1;
                    count += 1;
                },
                '\n' | ' ' | '(' | ')' | '+' | '-' | '*' | '/' | '%' => {
                    self.push(TokenKind::Number, size, count);
                    self.set_temp(char);
                    return;
                },
                _ => {
                    size += char.len_utf8();
                    count += 1;
                }
            };
        };

        self.push(TokenKind::Number, size, count);
    }

    fn parse_whitespace(&mut self) {
        let mut size = 1;
        let mut count = 1;

        while let Some(char) = self.next() {
            if char != ' ' {
                self.push(TokenKind::WhiteSpace, size, count);
                self.set_temp(char);
                return;
            };
            size += 1;
            count += 1;
        }
    }

    fn parse_char(&mut self, first: char) {
        match first {
            ' ' => self.parse_whitespace(),
            '\n' => self.push(TokenKind::Newline, 1, 1),
            '+' => self.push(TokenKind::Plus, 1, 1),
            '-' => self.push(TokenKind::Minus, 1, 1),
            '*' => self.push(TokenKind::Star, 1, 1),
            '/' => self.push(TokenKind::Slash, 1, 1),
            '%' => self.push(TokenKind::Percent, 1, 1),
            '(' => self.push(TokenKind::OpenParen, 1, 1),
            ')' => self.push(TokenKind::CloseParen, 1, 1),
            '.' => 'arm: {
                if let Some(char) = self.next() {
                    if char.is_numeric() {
                        self.parse_number(true);
                        break 'arm;
                    } else {
                        self.push(TokenKind::Dot, 1, 1);
                        self.set_temp(char);
                    };
                }
            }
            '0'..='9' => self.parse_number(false),
            'a'..='z' | 'A'..='Z' | '_' => self.parse_ident(),
            _ => self.push(TokenKind::InvalidChar, first.len_utf8(), 1),
        };
    }

    pub fn tokenize<Output: From<Lexer<'src>>>(mut self, debug: bool) -> Output {
        self.debug = debug;

        while let Some(char) = self.next() {
            self.parse_char(char);
        };

        Output::from(self)
    }
}

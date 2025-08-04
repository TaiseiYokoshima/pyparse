use std::str::FromStr;

use super::{Keyword, Operator};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Space,
    Newline,

    LParen,
    RParen,

    Operator(Operator),

    Number(Box<str>),
    Keyword(Keyword),
    Identifier(Box<str>),


    // String(Box<str>),
    // Quote,
}

fn take_builder(builder: &mut Box<String>) -> Box<str> {
    use std::mem;
    mem::take(&mut *builder).into_boxed_str()
}

impl Token {
    fn parse_number(builder: &mut Box<String>) -> Self {
        use std::process::exit;
        if builder.chars().any(|c| !c.is_numeric()) {
            eprintln!("Error: Tokenizer got an unsupported token: {}", builder);
            eprintln!("Number literal cannot contain non numeric characters");
            exit(1);
        };

        let number_str = take_builder(builder);
        Token::Number(number_str)
    }

    fn parse_keyword(builder: &mut Box<String>) -> Option<Self> {
        if let Ok(keyword) = Keyword::from_str(builder.as_str()) {
            builder.clear();
            return Some(Token::Keyword(keyword));
        };

        None
    }

    fn parse_identifier(builder: &mut Box<String>) -> Self {
        use std::process::exit;
        if builder.chars().any(|c| !c.is_alphanumeric() && c != '_') {
            eprintln!("Error: Tokenizer got an unsupported token: {}", builder);
            eprintln!(
                "Identifier must start with a letter and can only contain alphanumeric and underscore characters"
            );
            exit(1);
        };

        let identifier_str = take_builder(builder);
        Token::Identifier(identifier_str)
    }

    pub fn match_deliminiter(char: &char) -> Option<Self> {
        use std::str::FromStr;
        match char {
            ' ' => return Some(Self::Space),
            '(' => return Some(Self::LParen),
            ')' => return Some(Self::RParen),
            '\n' => return Some(Self::Newline),
            _ => (),
        };

        if let Ok(operator) = Operator::from_str(char.to_string().as_str()) {
            return Some(Self::Operator(operator));
        };

        None
    }

    pub fn match_token(builder: &mut Box<String>) -> Self {
        if builder.chars().next().unwrap().is_numeric() {
            return Self::parse_number(builder);
        };

        if let Some(token) = Self::parse_keyword(builder) {
            return token;
        };

        Self::parse_identifier(builder)
    }
}

use super::{tokens::Keyword, Token};
use std::process::exit;


fn take_builder(builder: &mut Box<String>) -> Box<str> {
    use std::mem;
    mem::take(&mut *builder).into_boxed_str()
}


fn parse_delimiter(char: char, tokens: &mut Vec<Token>) {
    let token = match char {
        '+' => Token::Plus,
        '-' => Token::Minus,
        '/' => Token::Slash,
        '*' => Token::Star,
        '(' => Token::LParen,
        ')' => Token::RParen,
        '\n' => Token::Newline,
        _ => {
            eprintln!("Error: Tokenizer got an unsupported character: {}", char);
            exit(1);
        }
    };

    tokens.push(token);
    return;
}



fn keyword_consumed(tokens: &mut Vec<Token>, builder: &mut Box<String>) -> bool {
    let keyword = match builder.as_ref().as_str() {
        "Def" => Keyword::Def,
        "And" => Keyword::And,
        "Not" => Keyword::Not,
        _ => { return false; }
    };

    let token = Token::Keyword(keyword);
    tokens.push(token);
    true
}

fn parse_non_delimiter(tokens: &mut Vec<Token>, builder: &mut Box<String>) {
    println!("came here with: {}", builder);

    if builder.chars().next().unwrap().is_numeric() {
        if builder.chars().any(|c| !c.is_numeric()) {
            eprintln!("Error: Tokenizer got an unsupported token: {} | number literal contains non numeric character", builder);
            exit(1);
        };

        let token = Token::Number(take_builder(builder));
        tokens.push(token);
        return;
    };


    if keyword_consumed(tokens, builder) {
        builder.as_mut().clear();
        return;
    }


    let token = Token::Identifier(take_builder(builder));
    // tokens.push();




}



fn parse_char(char: char, tokens: &mut Vec<Token>, builder: &mut Box<String>) {
    match char {
        ' ' => {
            if builder.is_empty() {
                return;
            };
            builder.push(char);
            parse_non_delimiter(tokens, builder);
            return;
        },
        '+' | '-' | '/' | '*' | '(' | ')' | '\n' => {
            if !builder.is_empty() {
                parse_non_delimiter(tokens, builder);
            };

            parse_delimiter(char, tokens);
            return;
        },

        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
            builder.push(char);
            return;
        },
        _ => {
            eprintln!("Error: Tokenizer got an unsupported character: {}", char);
            exit(1);
        }
    };
} 





pub fn tokenize(mut src: String) -> Vec<Token> {
    // println!("printing file contents:\n-{}-", src);

    if src.len() == 0 {
        eprintln!("Error: The python file is empty");
        exit(1);
    };

    let mut tokens = vec![];
    let mut builder = Box::default();


    while let Some(char) = src.chars().next() {
        println!("|{:?}", char);
        src.drain(..char.len_utf8());
        parse_char(char, &mut tokens, &mut builder);
    };



    println!("size of string: {}", src.len());


    println!("tokens:\n{:?}", tokens);


    return vec![];
}

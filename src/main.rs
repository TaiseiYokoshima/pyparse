use parse::Parser;

pub mod lex;
pub mod parse;
mod start;

fn main() {
    use lex::{Lexer, TokenStream};

    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    let debug = false;
    // let debug = true;
    let cursor = Lexer::new(&src);
    let tokens: TokenStream = cursor.tokenize(debug);

    println!("{}", tokens);


    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.parse());


}

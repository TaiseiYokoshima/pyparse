use parse::Parser;

mod lex;
mod parse;
mod start;
mod source;

fn main() {
    use lex::{Lexer, TokenStream};

    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    let debug = false;
    let cursor = Lexer::new(&src);
    let tokens: TokenStream = cursor.tokenize(debug);

    // for token in &tokens.stream {
    //     println!("{}", token)
    // };

    let mut parser = Parser::new(tokens);
    parser.parse();

}

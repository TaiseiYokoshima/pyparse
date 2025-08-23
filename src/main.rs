use parse::Parser;

mod lex;
mod parse;
mod start;
mod source;


use std::mem::size_of;



fn main() {
    use lex::{Lexer, TokenStream, Token};
    use source::Source;

    let src = Source::new({
        let path = start::parse_path();
        start::load_src(&path)
    });


    // println!("size of token: {}", size_of::<Token>());


   // src.print_lines();
   // println!("\n\n{:?}", src.line_ranges);
   // return;



    let debug = false;
    let cursor = Lexer::new(&src);
    


    let tokens: TokenStream = cursor.tokenize(debug);

    // for token in &tokens.stream {
    //     println!("{}", token)
    // };

    let mut parser = Parser::new(tokens);
    parser.parse();

}

mod units;
mod errors;

mod start;
mod lex;
mod parser;


pub use units::{Source, SrcSpan};

pub use errors::syntax::SyntaxError;
pub use lex::Lexer;
pub use parser::Parser;


fn main() {
    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    let mut lexer = Lexer::new(&src);
    let result = lexer.tokenize();

    match result {
        Ok(_) => println!("\n\n\n{}", lexer),
        Err(e) => println!("{:?}", e),

    };

    // println!("\n\n\n{:?}", result);

    // let parser = Parser::new(lexer);
    // let expr = parser.parse();
    // println!("\n\n\nexpr:\n{}", expr);
}

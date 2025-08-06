#[derive(Debug)]
pub enum SyntaxError<'err> {
    NumberError(&'err str, String),
    IdentifierError(&'err str),
}

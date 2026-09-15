use crate::lex;
use crate::ast;
use crate::lex::Token;

pub struct Lexer<'src> {
   stream: lex::TokenStream<'src>,
   pos: usize,
}

impl<'src> Lexer<'src> {
   pub fn new(stream: lex::TokenStream<'src>) -> Self {
      Self { stream, pos: 0 }
   }

   
   pub fn next(&mut self) -> ast::Token {
      let token = self.stream.stream.pop_front();
      let token = token.unwrap_or(lex::Token { kind: lex::TokenKind::Eof, size: 0 });

      let span = ast::Span { pos: self.pos, len: token.size };

      self.pos += token.size;

      ast::Token {
         kind: token.kind,
         span
      }
   }

   pub fn peek(&self) -> ast::Token {
      let token = self.stream.stream.get(0).copied();
      let token = token.unwrap_or(Token { kind: ast::TokenKind::Eof, size: 0 });
      let span = ast::Span { pos: self.pos, len: token.size };

      ast::Token { kind: token.kind, span }
   }
}

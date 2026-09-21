mod lexer;
mod stream;
mod token;

use std::iter::Peekable;
use std::str::Chars;

use crate::source::Source;
pub use lexer::Lexer;
pub use token::{Span, Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub enum LexErrorKind {
   TrailingBackslash,
}

#[derive(Debug)]
pub struct LexError {
   kind: LexErrorKind,
   span: Span,
}

pub struct ShellCursor<'src> {
   src: &'src str,
   chars: Peekable<Chars<'src>>,
   pos: usize,
   len: usize,
}

impl<'src> ShellCursor<'src> {
   pub fn new(src: &'src str) -> Self {
      let chars = src.chars().peekable();
      Self {
         src,
         chars,
         pos: 0,
         len: 0,
      }
   }

   fn reset(&mut self) {
      self.pos += self.len;
      self.len = 0;
   }

   fn next(&mut self) -> Option<char> {
      self.chars.next()
   }

   fn peek(&mut self) -> Option<char> {
      self.chars.peek().map(|char| *char)
   }

   fn single_byte_token(&mut self, kind: TokenKind) -> Token {
      let token = Token {
         kind,
         span: Span::one(self.pos),
      };
      self.pos += 1;
      token
   }

   fn shell_word(&mut self) -> Result<Token, LexError> {
      let kind = TokenKind::ShellLiteral;

      while let Some(char) = self.peek() {
         match char {
            '\\' => {
               self.next();
               let pos = self.pos + self.len;
               self.len += 1;

               let next_char = self.chars.next().ok_or(LexError {
                  kind: LexErrorKind::TrailingBackslash,
                  span: Span::zero(pos),
               })?;

               self.len += next_char.len_utf8();
               continue;
            }

            '$' | '"' | '\'' | ' ' | '\t' | '\n' | '(' | ')' | '>' | '<' | '|' | '&' | ':'
            | ';' => break,
            _ => {
               self.next();
               self.len += char.len_utf8();
            }
         };
      }

      let span = Span::new(self.pos, self.len);
      let token = Token::new(kind, span);
      self.pos += self.len;
      self.len = 0;
      Ok(token)
   }

   fn redirect(&mut self) -> Token {
      todo!()
   }

   fn expansion(&mut self) -> Token {
      todo!()
   }

   pub fn token(&mut self) -> Result<Token, LexError> {
      let Some(char) = self.next() else {
         return Ok(Token {
            kind: TokenKind::Eof,
            span: Span::zero(self.pos),
         });
      };

      let token = match char {
         '(' => self.single_byte_token(TokenKind::OpenParen),
         ')' => self.single_byte_token(TokenKind::CloseParen),
         '|' => self.single_byte_token(TokenKind::Pipe),
         '&' => self.single_byte_token(TokenKind::Ampersand),
         ';' => self.single_byte_token(TokenKind::Semi),
         '"' => self.single_byte_token(TokenKind::ShellDoubleQuote),
         '\'' => self.single_byte_token(TokenKind::ShellSingleQuote),
         ':' => self.expansion(),
         '>' | '<' => self.redirect(),
         '\\' => {
            let Some(next_char) = self.next() else {
               self.pos += 1;
               return Err(LexError {
                  kind: LexErrorKind::TrailingBackslash,
                  span: Span::zero(self.pos),
               });
            };

            if '\n' == next_char {
               let start = self.pos;
               let len = 2;
               let span = Span::new(start, len);
               let kind = TokenKind::WhiteSpace;
               self.pos += len;

               Token { kind, span }
            } else {
               self.pos += 1;
               self.len += next_char.len_utf8();
               self.shell_word()?
            }
         }

         char => {
            self.len += char.len_utf8();
            self.shell_word()?
         }
      };

      Ok(token)
   }
}

#[test]
fn shell_word_reports_trailing_backslash() {
   let source = "foo\\";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.shell_word();
   let error = result.expect_err("expected a trailing backslash error");

   assert_eq!(error.kind, LexErrorKind::TrailingBackslash);
   assert_eq!(error.span, Span::zero(3));
}

#[test]
fn start_with_newline() {
   let source = "\n";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.shell_word();
   assert!(result.is_ok());
   assert_eq!(result.unwrap().kind, TokenKind::ShellLiteral);
}

#[test]
fn line_continuation() {
   let source = "\\\n";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.token();
   assert!(result.is_ok());
   assert_eq!(result.unwrap().kind, TokenKind::WhiteSpace);
}


#[test]
fn shell_word_ascii() {
    let source = "hello";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_space() {
    let source = "hello world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_shell_operator() {
    let source = "hello|world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_expansion() {
    let source = "hello$world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_quotes() {
    let source = "hello\"world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_escaped_space() {
    let source = r"hello\ world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_operator() {
    let source = r"hello\|world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_dollar() {
    let source = r"hello\$world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_quote() {
    let source = r#"hello\"world"#;

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_unicode() {
    let source = "hello世界";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 11));
}

#[test]
fn shell_word_unicode_before_delimiter() {
    let source = "hello世界 world";

    let mut cursor = ShellCursor::new(source);
    let token = cursor.shell_word().unwrap();

    assert_eq!(token.kind, TokenKind::ShellLiteral);
    assert_eq!(token.span, Span::new(0, 11));
}

#[test]
fn shell_word_trailing_backslash() {
    let source = "hello\\";

    let mut cursor = ShellCursor::new(source);
    let result = cursor.shell_word();

    let error = result.expect_err("expected trailing backslash");

    assert_eq!(error.kind, LexErrorKind::TrailingBackslash);
    assert_eq!(error.span, Span::zero(5));
}

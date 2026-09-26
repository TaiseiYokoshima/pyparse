mod lexer;
mod stream;
mod token;

use std::{collections::VecDeque, iter::Peekable};
use std::str::Chars;

use crate::source::Source;
pub use lexer::Lexer;
pub use token::{Span, Token, TokenKind};

#[derive(Debug, PartialEq, Eq)]
pub enum LexErrorKind {
   TrailingBackslash,
   TrailingLang,
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

macro_rules! delimiters_arm {
    () => {
        '$' | '"' | '\'' | ' ' | '\t' | '\n' | '(' | ')' | '>' | '<' | '|' | '&' | ':' | ';'
    };
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

   fn revert(&mut self) {
      self.chars = self.src[self.pos + self.len..].chars().peekable();
   }

   fn pop(&mut self) -> Option<char> {
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

   fn str(&self) -> &str {
      &self.src[self.pos..self.pos + self.len]
   }

   fn shell_word(&mut self, char: char) -> Result<Token, LexError> {
      assert!(self.len == 0);

      let mut char = char;
      loop {
         match char {
            '\\' => match self.pop() {
               None => {
                  let pos = self.pos + self.len;
                  self.reset();

                  return Err(LexError {
                     kind: LexErrorKind::TrailingBackslash,
                     span: Span::zero(pos),
                  });
               }
               Some(escaped) => {
                  self.len += 1 + escaped.len_utf8();
               }
            },
            delimiters_arm!() => break self.revert(),
            _ => self.len += char.len_utf8(),
         };

         if let Some(next) = self.pop() {
            char = next;
         } else {
            break;
         };
      }

      let token = if self.str() == "fd" {
         let kind = TokenKind::Fd;
         let span = Span::new(self.pos, self.len);
         Token::new(kind, span)
      } else {
         let kind = TokenKind::Word;
         let span = Span::new(self.pos, self.len);
         Token::new(kind, span)
      };

      self.pos += self.len;
      self.len = 0;
      Ok(token)
   }

   fn redirect(&mut self) -> Result<Token, LexError> {
      self.len += 1;

      while let Some(char) = self.pop() {
         match char {
            '>' | '<' => self.len += 1,
            _ => break,
         };
      }

      self.revert();

      let str = &self.src[self.pos..self.pos + self.len];

      let kind = match str {
         ">" => TokenKind::RedirectOut,
         ">>" => TokenKind::RedirectOutAppend,
         "<" => TokenKind::RedirectIn,
         "<<<" => TokenKind::RedirectInStr,
         "<<" => TokenKind::RedirectInDoc,
         _ => todo!(),
      };

      let span = Span::new(self.pos, self.len);
      let token = Token { span, kind };
      self.reset();
      Ok(token)
   }

   fn lang_expansion(&mut self) -> Result<Token, LexError> {
      self.len += 1;

      while let Some(char) = self.pop() {
         match char {
            '{' => todo!("lang block not implemented"),
            '(' => todo!("lang expr not implemented"),
            delimiters_arm!() => {
               self.revert();
               break;
            }
            _ => self.len += char.len_utf8(),
         };
      };

      if self.src == ":" {
         let kind = LexErrorKind::TrailingLang;
         let span = Span::one(self.pos);
         return Err(LexError { kind, span });
      };


      println!("str: {:?}", self.str());
      let kind = TokenKind::LangVar;
      let span = Span::new(self.pos, self.len);
      self.reset();

      return Ok(Token::new(kind, span));
   }

   pub fn token(&mut self) -> Result<Token, LexError> {
      let Some(char) = self.pop() else {
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
         '$' => self.single_byte_token(TokenKind::Dollar),
         ';' => self.single_byte_token(TokenKind::Semi),
         '"' => self.single_byte_token(TokenKind::DoubleQuote),
         '\'' => self.single_byte_token(TokenKind::SingleQuote),
         '\n' => self.single_byte_token(TokenKind::Newline),
         ' ' => self.single_byte_token(TokenKind::WhiteSpace),
         ':' => self.lang_expansion()?,
         '>' | '<' => self.redirect()?,
         _ => self.shell_word(char)?,
      };

      Ok(token)
   }
}

#[test]
fn shell_word_reports_trailing_backslash() {
   let source = "foo\\";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.token();
   let error = result.expect_err("expected a trailing backslash error");

   assert_eq!(error.kind, LexErrorKind::TrailingBackslash);
   assert_eq!(error.span, Span::zero(3));
}

#[test]
fn start_with_newline() {
   let source = "\n";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.token();
   assert!(result.is_ok());
   assert_eq!(result.unwrap().kind, TokenKind::Newline);
}

#[test]
fn line_continuation() {
   let source = "\\\n";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.token();
   assert!(result.is_ok());
   assert_eq!(result.unwrap().kind, TokenKind::Word);
}

#[test]
fn shell_word_ascii() {
   let source = "hello";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_space() {
   let source = "hello world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_shell_operator() {
   let source = "hello|world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_expansion() {
   let source = "hello$world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_stops_at_quotes() {
   let source = "hello\"world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 5));
}

#[test]
fn shell_word_escaped_space() {
   let source = r"hello\ world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_operator() {
   let source = r"hello\|world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_dollar() {
   let source = r"hello\$world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_escaped_quote() {
   let source = r#"hello\"world"#;

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 12));
}

#[test]
fn shell_word_unicode() {
   let source = "hello世界";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 11));
}

#[test]
fn shell_word_unicode_before_delimiter() {
   let source = "hello世界 world";

   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().unwrap();

   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span, Span::new(0, 11));
}

#[test]
fn shell_word_trailing_backslash() {
   let source = "hello\\";

   let mut cursor = ShellCursor::new(source);
   let result = cursor.token();

   let error = result.expect_err("expected trailing backslash");

   assert_eq!(error.kind, LexErrorKind::TrailingBackslash);
   assert_eq!(error.span, Span::zero(5));
}

#[test]
fn shell_word_delimiter() {
   let source = "hello$test\nhaha\nfd 1";
   let mut cursor = ShellCursor::new(source);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Dollar);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 4);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Newline);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 4);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Newline);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Fd);
   assert_eq!(token.span.len(), 2);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::WhiteSpace);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 1);
}

#[test]
fn redirect() {
   let source = "fd 2 > test";
   let mut cursor = ShellCursor::new(source);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Fd);
   assert_eq!(token.span.len(), 2);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::WhiteSpace);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::WhiteSpace);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectOut);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::WhiteSpace);
   assert_eq!(token.span.len(), 1);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 4);
}

#[test]
fn all_redirect() {
   let source = "> >> < << <<<";
   let mut cursor = ShellCursor::new(source);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectOut);
   assert_eq!(token.span.len(), 1);
   cursor.token().unwrap();

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectOutAppend);
   assert_eq!(token.span.len(), 2);
   cursor.token().unwrap();

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectIn);
   assert_eq!(token.span.len(), 1);
   cursor.token().unwrap();

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectInDoc);
   assert_eq!(token.span.len(), 2);
   cursor.token().unwrap();

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::RedirectInStr);
   assert_eq!(token.span.len(), 3);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Eof);
   assert_eq!(token.span.len(), 0);

   let result = cursor.token();
   let token = result.expect("expected token instead of error");
   assert_eq!(token.kind, TokenKind::Eof);
   assert_eq!(token.span.len(), 0);
}


#[test]
fn lang_simpel() {
   let source = ":test";
   let mut cursor = ShellCursor::new(source);
   let token = cursor.token().expect("expected token instead of error");
   assert_eq!(token.span.len(), 5);
}

#[test]
fn lang_var() {
   let source = "cd :test || echo \"failed\"";
   let mut cursor = ShellCursor::new(source);

   let mut tokens: VecDeque<_> = (0..100)
      .map(|_| cursor.token().expect("expected token instead of error"))
      .filter(|t| t.kind != TokenKind::Eof)
      .collect();

   let whitespace_count = tokens
      .iter()
      .filter(|t| t.kind == TokenKind::WhiteSpace)
      .count();

   tokens.retain(|x| x.kind != TokenKind::WhiteSpace);

   assert_eq!(whitespace_count, 4);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 2);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::LangVar);
   assert_eq!(token.span.len(), 5);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::Pipe);
   assert_eq!(token.span.len(), 1);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::Pipe);
   assert_eq!(token.span.len(), 1);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 4);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::DoubleQuote);
   assert_eq!(token.span.len(), 1);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::Word);
   assert_eq!(token.span.len(), 6);

   let token = tokens.pop_front().unwrap();
   assert_eq!(token.kind, TokenKind::DoubleQuote);
   assert_eq!(token.span.len(), 1);
}


#[test]
#[should_panic(expected = "lang block not implemented")]
fn lang_block() {
   let src = ":{hahaha}";
   let mut cursor = ShellCursor::new(src);
   cursor.token().unwrap();
}

#[test]
#[should_panic(expected = "lang expr not implemented")]
fn lang_expr() {
   let src = ":(hahaha)";
   let mut cursor = ShellCursor::new(src);
   cursor.token().unwrap();
}


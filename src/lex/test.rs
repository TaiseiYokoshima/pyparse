use super::{ShellCursor, TokenKind, Span};

macro_rules! span {
   ($pos:expr, $len:expr) => {
      Span::new($pos, $len)
   };
}

struct Tester<'src> {
   cursor: ShellCursor<'src>,
}

impl<'src> Tester<'src> {
   pub fn init() -> Self {
      Self { cursor: ShellCursor::new("") }
   }


   pub fn new(src: &'src str) -> Self {
      Self { cursor: ShellCursor::new(src) }
   }

   pub fn test_token(&mut self, kind: TokenKind, span: Span) {
      let token = self.cursor.token();
      assert_eq!(token.kind, kind);
      assert_eq!(token.span, span);
   }
}


#[thread_local]
static mut TESTER: std::sync::LazyLock<Tester> = std::sync::LazyLock::new(|| Tester::init());

macro_rules! test {
   ($pos:expr, $len:expr) => {
      unsafe { (*TESTER).test_token($pos, $len) }
   };
}

macro_rules! src {
   ($src:expr) => {
      unsafe { *TESTER = Tester::new($src) }
   };
}

#[test]
fn words_around_all_delimiters() {
   src!(r#"a$b"c'd e f(g)h>i>>j<k<<l<<<m|n||o&p&&q:r;s"#);

   test!(TokenKind::Word, span!(0, 1));
   test!(TokenKind::Dollar, span!(1, 1));
   test!(TokenKind::Word, span!(2, 1));
   test!(TokenKind::DoubleQuote, span!(3, 1));
   test!(TokenKind::Word, span!(4, 1));
   test!(TokenKind::SingleQuote, span!(5, 1));
   test!(TokenKind::Word, span!(6, 1));
   test!(TokenKind::WhiteSpace, span!(7, 1));
   test!(TokenKind::Word, span!(8, 1));
   test!(TokenKind::WhiteSpace, span!(9, 1));

   test!(TokenKind::Word, span!(10, 1));
   test!(TokenKind::OpenParen, span!(11, 1));
   test!(TokenKind::Word, span!(12, 1));
   test!(TokenKind::CloseParen, span!(13, 1));
   test!(TokenKind::Word, span!(14, 1));

   test!(TokenKind::RedirectOut, span!(15, 1));
   test!(TokenKind::Word, span!(16, 1));
   test!(TokenKind::RedirectOutAppend, span!(17, 2));
   test!(TokenKind::Word, span!(19, 1));
   test!(TokenKind::RedirectIn, span!(20, 1));
   test!(TokenKind::Word, span!(21, 1));
   test!(TokenKind::RedirectInDoc, span!(22, 2));
   test!(TokenKind::Word, span!(24, 1));
   test!(TokenKind::RedirectInStr, span!(25, 3));
   test!(TokenKind::Word, span!(28, 1));

   test!(TokenKind::Pipe, span!(29, 1));
   test!(TokenKind::Word, span!(30, 1));
   test!(TokenKind::Or, span!(31, 2));
   test!(TokenKind::Word, span!(33, 1));
   test!(TokenKind::Ampersand, span!(34, 1));
   test!(TokenKind::Word, span!(35, 1));
   test!(TokenKind::And, span!(36, 2));
   test!(TokenKind::Word, span!(38, 1));

   test!(TokenKind::Colon, span!(39, 1));
   test!(TokenKind::Word, span!(40, 1));
   test!(TokenKind::Semi, span!(41, 1));
   test!(TokenKind::Word, span!(42, 1));
   test!(TokenKind::Eof, span!(43, 0));
}

#[test]
fn escaped_delimiters_are_part_of_word() {
   let s = r#"a\$b\"c\'d\ e\ f\(g\)h\>i\>\>j\<k\<\<l\<\<\<m\|n\|\|o\&p\&\&q\:r\;s"#;
   src!(s);
   test!(TokenKind::Word, span!(0, s.len() as u32));
   test!(TokenKind::Eof, span!(s.len(), 0));
}

#[test]
fn shell_word_unicode_before_delimiter() {
	src!("hello世界 world");
   test!(TokenKind::Word, span!(0, 11));
   test!(TokenKind::WhiteSpace, span!(11, 1));
   test!(TokenKind::Word, span!(12, 5));
}

#[test]
fn all_redirect() {
	src!("> >> < << <<<");

   test!(TokenKind::RedirectOut, span!(0, 1));
   test!(TokenKind::WhiteSpace, span!(1, 1));

   test!(TokenKind::RedirectOutAppend, span!(2, 2));
   test!(TokenKind::WhiteSpace, span!(4, 1));

   test!(TokenKind::RedirectIn, span!(5, 1));
   test!(TokenKind::WhiteSpace, span!(6, 1));

   test!(TokenKind::RedirectInDoc, span!(7, 2));
   test!(TokenKind::WhiteSpace, span!(9, 1));

   test!(TokenKind::RedirectInStr, span!(10, 3));
   test!(TokenKind::Eof, span!(13, 0));
}

#[test]
fn lang() {
	src!(":test");
   test!(TokenKind::Colon, span!(0, 1));
   test!(TokenKind::Word, span!(1, 4));
}

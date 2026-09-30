use crate::lex::Span;

pub enum WordPartKind {
   Id,
   Lit,
   StrLit,
   Cmd(Box<Cmd>),
   LangVar,
}

pub struct WordPart {
   kind: WordPartKind,
   span: Span,
}


impl WordPart {
   pub fn new(kind: WordPartKind, span: Span) -> Self {
      Self { kind, span }
   }
}

pub struct Word(Vec<WordPart>, Span);

pub enum RedirectKind {
   WriteDup,
   WritePath,
   WriteAppendPath,

   ReadDup,
   ReadPath,
   ReadMultiStr,
   ReadStr,
}

pub struct Redirect(Vec<Word>, RedirectKind, Vec<Word>);

pub enum CmdPart {
   Word(Word),
   Redirect(Redirect),
}

pub enum CmdOperator {
   And,
   Or,
   Pipe,
}

pub struct Cmd {
   vars: Vec<(Span, Span)>,
   words: Vec<CmdPart>,
   span: Span,
   bin: Option<(CmdOperator, Box<Cmd>)>,
   bg: bool,
}

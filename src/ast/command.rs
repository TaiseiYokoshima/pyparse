use crate::ast::Span;

pub enum WordPartKind {
   Id,
   Lit,
   StrLit,
   Cmd(Box<Cmd>),
}

struct WordPart {
   kind: WordPartKind,
   span: Span,
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

pub struct Redirect(Word, RedirectKind, Word);

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
   bin: Option<(CmdOperator, Box<Cmd>)>
}

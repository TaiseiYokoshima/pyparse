// use super::ast::{BP, BinOpn, BinOpr, Expr, LiteralKind};

use super::lexer::Lexer;

use crate::ast::{Token, TokenKind, command::{Cmd, Redirect}};
use std::{collections::VecDeque, ops::Range};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
   ExpectedExpression,
   ExpectedOperator,
   ExpectedCloseParen,
}

pub struct Parser<'src> {
   lexer: Lexer<'src>,
   token: Token,
}

impl<'src> Parser<'src> {
   pub fn new(mut lexer: Lexer<'src>) -> Self {
      let token = lexer.next();
      Self {
         token,
         lexer
      }
   }

   // #[inline(always)]
   // fn peek_at(&self, at: usize) -> Token {
   //    unimplemented!()
   // }

   #[inline(always)]
   fn peek(&self) -> Token {
      self.lexer.peek()
   }

   // #[inline(always)]
   // fn peek_second(&self) -> RangedToken {
   //    self.peek_at(1)
   // }

   // fn match_operator(&self, token_kind: TokenKind) -> Option<(BinOpr, bool)> {
   //    match token_kind {
   //       TokenKind::Plus => Some((BinOpr::Add, false)),
   //       TokenKind::Minus => Some((BinOpr::Sub, false)),
   //       TokenKind::Percent => Some((BinOpr::Mod, false)),
   //       TokenKind::Star => match self.peek_second().kind {
   //          TokenKind::Star => Some((BinOpr::Exp, true)),
   //          _ => Some((BinOpr::Mul, false)),
   //       },
   //       TokenKind::Slash => match self.peek_second().kind {
   //          TokenKind::Slash => Some((BinOpr::Flo, true)),
   //          _ => Some((BinOpr::Div, false)),
   //       },
   //       _ => None,
   //    }
   // }





   
   fn parse_words_and_redirect(&self) -> (Vec<Token>, Vec<Redirect>) {
      type K = TokenKind;

      let mut words = vec![];
      let redirects = vec![];

      // loop { 
      //    match self.token.kind {
      //       K::Semi | K::Ampersand | K::Newline | K::Pipe => break,
      //       // K::Dollar |
      //    }
      //
      // }


      (words, redirects)
   }




   // fn parse_cmd(&self) -> Option<Cmd> {
   //    use crate::ast::command::{
   //       Word, WordPartKind, Redirect, CmdPart, CmdOperator
   //    };
   //
   //    let words = vec![];
   //
   //
   //    while self.next()
   //
   //
   //
   //
   //    todo!()
   // }

   // fn pratt_parse(&mut self, min_bp: BP) -> Option<Result<Expr, ParserError>> {
   //    let token = self.next();
   //    let mut lhs = match token.kind {
   //       TokenKind::Eof | TokenKind::Newline => return None,
   //       TokenKind::Number => Expr::literal(LiteralKind::Number, token.range),
   //       TokenKind::Ident => Expr::Ident(token.range),
   //       TokenKind::OpenParen => {
   //          // first check expr for Err and None,
   //          let expr_opt = self.pratt_parse(min_bp);
   //          let expr = match expr_opt {
   //             Some(Ok(expr)) => expr,
   //             some_error @ Some(Err(_)) => return some_error,
   //             None => {
   //                let err = self.error(ErrorKind::ExpectedExpression, self.none());
   //                self.recover();
   //                return Some(Err(err));
   //             }
   //          };
   //
   //          // check next token exists and is a close paren
   //          let peeked = self.peek();
   //          match peeked.kind {
   //             TokenKind::CloseParen => self.next(),
   //             _ => {
   //                let err = self.error(ErrorKind::ExpectedCloseParen, peeked);
   //                self.recover();
   //                return Some(Err(err));
   //             }
   //          };
   //          expr
   //       }
   //
   //       _ => {
   //          let error = self.error(ErrorKind::ExpectedExpression, token);
   //          self.recover();
   //          return Some(Err(error));
   //       }
   //    };
   //
   //    loop {
   //       let op_token = self.peek();
   //       let (op, is_compound) = match op_token.kind {
   //          TokenKind::Eof | TokenKind::CloseParen | TokenKind::Newline => break,
   //          _ => {
   //             let op = match self.match_operator(op_token.kind) {
   //                Some(op) => op,
   //                None => {
   //                   let error = self.error(ErrorKind::ExpectedOperator, op_token);
   //                   self.recover();
   //                   return Some(Err(error));
   //                }
   //             };
   //
   //             op
   //          }
   //       };
   //
   //       let (l_bp, r_bp): (BP, BP) = op.get_bp();
   //       if min_bp > l_bp {
   //          break;
   //       };
   //
   //       if is_compound {
   //          self.next();
   //          self.next();
   //       } else {
   //          self.next();
   //       };
   //
   //       let rhs = self.pratt_parse(r_bp);
   //       let rhs = match rhs {
   //          Some(Ok(exp)) => exp,
   //          err @ Some(Err(_)) => return err,
   //          None => {
   //             let error = self.error(ErrorKind::ExpectedExpression, self.none());
   //             return Some(Err(error));
   //          }
   //       };
   //
   //       lhs = Expr::bin_op(BinOpn::new(op, lhs, rhs));
   //    }
   //
   //    Some(Ok(lhs))
   // }
   //
   // pub fn parse(&mut self) {
   //    let mut errors = VecDeque::<ParserError>::new();
   //
   //    self.flush_newlines();
   //    while let Some(result) = self.pratt_parse(0) {
   //       match result {
   //          Ok(_) => (),
   //          Err(e) => errors.push_back(e),
   //       };
   //       self.flush_newlines();
   //       print!("\n\n");
   //    }
   //
   //    let diagnostics = Diagnostics::new(errors, self.src);
   //
   //    diagnostics.report_all();
   // }
}

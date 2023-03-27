use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Token {
  // special

  EOF,
  Error,
  Num,
  Space,
  Sym,

  // dots

  Dot,
  DotDot,
  DotDotDot,

  // punctuation

  Colon,
  Comma,
  LBrace,
  LBracket,
  LParen,
  RBrace,
  RBracket,
  RParen,
  Semi,

  // operators

  Assign,
  EQ,
  NE,
  GT,
  GE,
  LT,
  LE,
  Ampersand,
  At,
  Bang,
  Caret,
  Dollar,
  Minus,
  Percent,
  Pipe,
  Plus,
  Query,
  Slash,
  Star,
  Tilde,

  // keywords

  And,
  Break,
  Elif,
  Else,
  End,
  For,
  Fun,
  If,
  Let,
  Loop,
  Or,
  Return,
  While,
}

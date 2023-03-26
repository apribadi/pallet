use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Token {
  // special

  EOF,
  Error,
  Number,
  Space,
  Symbol,

  // dots

  Dot,
  DotDot,
  DotDotDot,

  // punctuation

  Colon,
  Comma,
  LBrace,
  LBracket,
  LParenthesis,
  RBrace,
  RBracket,
  RParenthesis,
  Semicolon,

  // operators

  Assignment,
  Equal,
  NotEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
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

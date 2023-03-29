use crate::prelude::*;

pub struct Lexer<'a> {
  buf: &'a [u8],
  start: usize,
  stop: usize,
  kinds: [Kind; 256],
  jumps: [[State; Kind::VARIANT_COUNT]; 8],
}

impl<'a> Lexer<'a> {
  pub fn new(buf: &'a [u8]) -> Self {
    Self {
      buf,
      start: 0,
      stop: 0,
      kinds: array::from_fn(|c| Kind::classify(c as u8)),
      jumps: [
        [
          // Start =>
          State::Number,
          State::Dot,
          State::Comment,
          State::Space,
          State::Symbol,
          State::Operator,
          State::TerminalPunctuation,
          State::Sign,
          State::Space,
          State::Symbol,
          State::TerminalUnknownCharacter,
        ],
        [
          // Comment =>
          State::Comment,
          State::Comment,
          State::Comment,
          State::Space,
          State::Comment,
          State::Comment,
          State::Comment,
          State::Comment,
          State::Comment,
          State::Comment,
          State::Comment,
        ],
        [
          // Dot =>
          State::TerminalDot,
          State::Dot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
          State::TerminalDot,
        ],
        [
          // Number =>
          State::Number,
          State::Number,
          State::TerminalNumber,
          State::TerminalNumber,
          State::TerminalNumber,
          State::TerminalNumber,
          State::TerminalNumber,
          State::TerminalNumber,
          State::TerminalNumber,
          State::Number,
          State::TerminalNumber,
        ],
        [
          // Operator =>
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::Operator,
          State::TerminalOperator,
          State::Operator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
        ],
        [
          // Sign =>
          State::Number,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::Operator,
          State::TerminalOperator,
          State::Operator,
          State::TerminalOperator,
          State::TerminalOperator,
          State::TerminalOperator,
        ],
        [
          // Space =>
          State::TerminalSpace,
          State::TerminalSpace,
          State::Comment,
          State::Space,
          State::TerminalSpace,
          State::TerminalSpace,
          State::TerminalSpace,
          State::TerminalSpace,
          State::Space,
          State::TerminalSpace,
          State::TerminalSpace,
        ],
        [
          // Symbol =>
          State::Symbol,
          State::TerminalSymbol,
          State::TerminalSymbol,
          State::TerminalSymbol,
          State::Symbol,
          State::TerminalSymbol,
          State::TerminalSymbol,
          State::TerminalSymbol,
          State::TerminalSymbol,
          State::Symbol,
          State::TerminalSymbol,
        ],
      ],
    }
  }

  pub fn next(&mut self) -> Token {
    let buf = self.buf;
    let kinds = &self.kinds;
    let jumps = &self.jumps;

    let i = self.stop;
    let mut j = i;
    let mut c = b'\0';
    let mut s = State::Start;

    while j != buf.len() {
      c = *unsafe { buf.get_unchecked(j) };

      let k = *unsafe { kinds.get_unchecked(c as usize) };

      s = *unsafe { jumps.get_unchecked(s as usize).get_unchecked(k as usize ) };

      if s.is_terminal() { break; }

      j += 1;
    }

    let token =
      match s {
        State::Start => {
          Token::EOF
        }
        State::Comment | State::Space | State::TerminalSpace => {
          Token::Space
        }
        State::Dot | State::TerminalDot => {
          match j.wrapping_sub(i) {
            1 => Token::Dot,
            2 => Token::DotDot,
            3 => Token::DotDotDot,
            _ => Token::Error,
          }
        }
        State::Number | State::TerminalNumber => {
          Token::Number
        }
        State::Operator | State::Sign | State::TerminalOperator => {
          match unsafe { buf.get_unchecked(i .. j) } {
            b"=" => Token::Assign,
            b"==" => Token::EQ,
            b"!=" => Token::NE,
            b">" => Token::GT,
            b">=" => Token::GE,
            b"<" => Token::LT,
            b"<=" => Token::LE,
            b"&" => Token::Ampersand,
            b"@" => Token::At,
            b"!" => Token::Bang,
            b"^" => Token::Caret,
            b"$" => Token::Dollar,
            b"-" => Token::Minus,
            b"%" => Token::Percent,
            b"|" => Token::Pipe,
            b"+" => Token::Plus,
            b"?" => Token::Query,
            b"/" => Token::Slash,
            b"*" => Token::Star,
            b"~" => Token::Tilde,
            _ => Token::Error,
          }
        }
        State::Symbol | State::TerminalSymbol => {
          match unsafe { buf.get_unchecked(i .. j) } {
            b"and" => Token::And,
            b"break" => Token::Break,
            b"do" => Token::Do,
            b"elif" => Token::Elif,
            b"else" => Token::Else,
            b"end" => Token::End,
            b"for" => Token::For,
            b"fun" => Token::Fun,
            b"if" => Token::If,
            b"let" => Token::Let,
            b"loop" => Token::Loop,
            b"or" => Token::Or,
            b"return" => Token::Return,
            b"then" => Token::Then,
            b"while" => Token::While,
            _ => Token::Symbol,
          }
        }
        State::TerminalPunctuation => {
          j += 1;

          match c {
            b':' => Token::Colon,
            b',' => Token::Comma,
            b'{' => Token::LBrace,
            b'[' => Token::LBracket,
            b'(' => Token::LParen,
            b'}' => Token::RBrace,
            b']' => Token::RBracket,
            b')' => Token::RParen,
            b';' => Token::Semi,
            _ => panic!()
          }
        }
        State::TerminalUnknownCharacter => {
          j += 1;

          Token::Error
        }
      };

    self.start = i;
    self.stop = j;

    token
  }

  pub fn span(&self) -> &'a [u8] {
    unsafe { self.buf.get_unchecked(self.start .. self.stop) }
  }
}

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
enum Kind {
  Digit,
  Dot,
  Hash,
  LF,
  Letter,
  Operator,
  Punctuation,
  Sign,
  Space,
  Underscore,
  Unknown,
}

impl Kind {
  fn classify(c: u8) -> Self {
    match c {
      b'\t' => Self::Space,
      b'\n' => Self::LF,
      b' ' => Self::Space,
      b'!' => Self::Operator,
      b'#' => Self::Hash,
      b'$' => Self::Operator,
      b'%' => Self::Operator,
      b'&' => Self::Operator,
      b'(' => Self::Punctuation,
      b')' => Self::Punctuation,
      b'*' => Self::Operator,
      b'+' => Self::Sign,
      b',' => Self::Punctuation,
      b'-' => Self::Sign,
      b'.' => Self::Dot,
      b'/' => Self::Operator,
      b'0' ..= b'9' => Self::Digit,
      b':' => Self::Punctuation,
      b';' => Self::Punctuation,
      b'<' => Self::Operator,
      b'=' => Self::Operator,
      b'>' => Self::Operator,
      b'?' => Self::Operator,
      b'@' => Self::Operator,
      b'A' ..= b'Z' => Self::Letter,
      b'[' => Self::Punctuation,
      b']' => Self::Punctuation,
      b'^' => Self::Operator,
      b'_' => Self::Underscore,
      b'a' ..= b'z' => Self::Letter,
      b'{' => Self::Punctuation,
      b'|' => Self::Operator,
      b'}' => Self::Punctuation,
      b'~' => Self::Operator,
      _ => Self::Unknown,
    }
  }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, VariantCount)]
#[repr(u8)]
enum State {
  Start,
  Comment,
  Dot,
  Number,
  Operator,
  Sign,
  Space,
  Symbol,
  TerminalDot,
  TerminalNumber,
  TerminalOperator,
  TerminalPunctuation,
  TerminalSpace,
  TerminalSymbol,
  TerminalUnknownCharacter,
}

impl State {
  #[inline(always)]
  fn is_terminal(self) -> bool {
    self >= Self::TerminalDot
  }
}

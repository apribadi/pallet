use crate::prelude::*;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  token: Token,
}

#[derive(Debug)]
pub struct ParseError;

impl<'a> Parser<'a> {
  pub fn new(buf: &'a [u8]) -> Self {
    let mut lexer = Lexer::new(buf);
    let token = lexer.next();
    Self { lexer, token, }
  }

  fn advance(&mut self) {
    self.token = self.lexer.next()
  }

  fn advance_if_space(&mut self) {
    if self.token == Token::Space {
      self.advance()
    }
  }

  fn expect(&mut self, token: Token) -> Result<(), ParseError> {
    if self.token == token {
      self.advance();
      Ok(())
    } else {
      Err(ParseError)
    }
  }

  fn span(&self) -> &'a [u8] {
    self.lexer.span()
  }

  pub fn parse_expr<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    self.advance_if_space();
    self.parse_expr_p(alloc)
  }

  // "f"uncall

  pub fn parse_expr_f<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_t(alloc)?;

    loop {
      // NB: Don't call `advance_if_space` here.

      match self.token {
        Token::LParenthesis => {
          self.advance();
          // TODO: parse expression sequence
          let x = self.parse_expr(alloc)?;
          self.expect(Token::RParenthesis)?;
          let x = &*alloc.alloc_slice(1).init_slice(|_| x);
          let a = &*alloc.alloc().init((e, x));
          e = AstExpr::FunCall(a);
        }
        _ => {
          break;
        }
      }
    }

    Ok(e)
  }

  // "p"refix

  pub fn parse_expr_p<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    self.advance_if_space();

    match self.token {
      Token::Minus => {
        self.advance();
        let x = self.parse_expr_p(alloc)?;
        let a = &*alloc.alloc().init(("-", [x]));
        Ok(AstExpr::Op1(a))
      }
      _ => {
        self.parse_expr_f(alloc)
      }
    }
  }

  // "t"erminal

  pub fn parse_expr_t<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    match self.token {
      Token::Number => {
        let s = str::from_utf8(alloc.copy_slice(self.span())).unwrap();
        self.advance();
        Ok(AstExpr::Number(s))
      }
      Token::Symbol => {
        let s = str::from_utf8(alloc.copy_slice(self.span())).unwrap();
        self.advance();
        Ok(AstExpr::Symbol(s))
      }
      Token::LParenthesis => {
        self.advance();
        let expr = self.parse_expr(alloc)?;
        self.expect(Token::RParenthesis)?;
        Ok(expr)
      }
      _ => Err(ParseError)
    }
  }

}

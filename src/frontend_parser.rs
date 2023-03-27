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
    self.parse_expr_e(alloc)
  }

  // "e"quality

  pub fn parse_expr_e<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_c(alloc)?;

    loop {
      self.advance_if_space();

      if let token @ (
          Token::Equal |
          Token::NotEqual
        ) = self.token
      {
        let op =
          match token {
            Token::Equal => "==",
            Token::NotEqual => "!=",
            _ => panic!(),
          };
        self.advance();
        let x = self.parse_expr_c(alloc)?;
        let a = &*alloc.alloc().init((op, [e, x]));
        e = AstExpr::Op2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "c"omparison

  pub fn parse_expr_c<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_a(alloc)?;

    loop {
      self.advance_if_space();

      if let token @ (
          Token::GreaterThan |
          Token::GreaterThanOrEqual |
          Token::LessThan |
          Token::LessThanOrEqual
        ) = self.token
      {
        let op =
          match token {
            Token::GreaterThan => ">",
            Token::GreaterThanOrEqual => ">=",
            Token::LessThan => "<",
            Token::LessThanOrEqual => "<=",
            _ => panic!(),
          };
        self.advance();
        let x = self.parse_expr_a(alloc)?;
        let a = &*alloc.alloc().init((op, [e, x]));
        e = AstExpr::Op2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "a"ddition

  pub fn parse_expr_a<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_m(alloc)?;

    loop {
      self.advance_if_space();

      if let token @ (Token::Minus | Token::Plus) = self.token {
        let op =
          match token {
            Token::Minus => "-",
            Token::Plus => "+",
            _ => panic!(),
          };
        self.advance();
        let x = self.parse_expr_m(alloc)?;
        let a = &*alloc.alloc().init((op, [e, x]));
        e = AstExpr::Op2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "m"ultiplication

  pub fn parse_expr_m<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_p(alloc)?;

    loop {
      self.advance_if_space();

      if let Token::Star = self.token {
        self.advance();
        let x = self.parse_expr_p(alloc)?;
        let a = &*alloc.alloc().init(("*", [e, x]));
        e = AstExpr::Op2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "p"refix

  pub fn parse_expr_p<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    self.advance_if_space();

    if let Token::Minus = self.token {
      self.advance();
      let x = self.parse_expr_p(alloc)?;
      let a = &*alloc.alloc().init(("-", [x]));
      Ok(AstExpr::Op1(a))
    } else {
      self.parse_expr_f(alloc)
    }
  }

  // "f"uncall

  pub fn parse_expr_f<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_t(alloc)?;

    loop {
      // NB: Don't call `advance_if_space` here.

      if let Token::LParenthesis = self.token {
        self.advance();
        // TODO: parse expression sequence
        let x = self.parse_expr(alloc)?;
        self.expect(Token::RParenthesis)?;
        let x = &*alloc.alloc_slice(1).init_slice(|_| x);
        let a = &*alloc.alloc().init((e, x));
        e = AstExpr::FunCall(a);
      } else {
        break;
      }
    }

    Ok(e)
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

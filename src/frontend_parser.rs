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

  fn fail<T>(&mut self) -> Result<T, ParseError> {
    let _ = self;
    Err(ParseError)
  }

  fn expect(&mut self, token: Token) -> Result<(), ParseError> {
    if self.token == token {
      Ok(())
    } else {
      self.fail()
    }
  }

  fn span(&self) -> &'a [u8] {
    self.lexer.span()
  }

  pub fn parse_expr<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    self.parse_expr_e(alloc)
  }

  // "e"quality

  pub fn parse_expr_e<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_c(alloc)?;

    loop {
      if let token @ (Token::EQ | Token::NE) = self.token {
        let op =
          match token {
            Token::EQ => "==",
            Token::NE => "!=",
            _ => panic!(),
          };
        self.advance();
        self.advance_if_space();
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
      if let token @ (
          Token::GT |
          Token::GE |
          Token::LT |
          Token::LE
        ) = self.token
      {
        let op =
          match token {
            Token::GT => ">",
            Token::GE => ">=",
            Token::LT => "<",
            Token::LE => "<=",
            _ => panic!(),
          };
        self.advance();
        self.advance_if_space();

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
      if let token @ (Token::Minus | Token::Plus) = self.token {
        let op =
          match token {
            Token::Minus => "-",
            Token::Plus => "+",
            _ => panic!(),
          };
        self.advance();
        self.advance_if_space();
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
      if let Token::Star = self.token {
        self.advance();
        self.advance_if_space();
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
    if let Token::Minus = self.token {
      self.advance();
      self.advance_if_space();
      let x = self.parse_expr_p(alloc)?;
      let a = &*alloc.alloc().init(("-", [x]));
      Ok(AstExpr::Op1(a))
    } else {
      self.parse_expr_t(alloc)
    }
  }

  // "t"erminal (and funcalls)

  pub fn parse_expr_t<'b>(&mut self, alloc: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e =
      match self.token {
        Token::Num => {
          let s = str::from_utf8(alloc.copy_slice(self.span())).unwrap();
          self.advance();
          AstExpr::Num(s)
        }
        Token::Sym => {
          let s = str::from_utf8(alloc.copy_slice(self.span())).unwrap();
          self.advance();
          AstExpr::Sym(s)
        }
        Token::LParen => {
          self.advance();
          self.advance_if_space();
          let expr = self.parse_expr(alloc)?;
          self.expect(Token::RParen)?;
          self.advance();
          expr
        }
        _ => {
          self.fail()?
        }
      };

    // Leave unconsumed space, because there can't be space between a function
    // and its arguments.

    loop {
      if self.token == Token::LParen {
        self.advance();
        self.advance_if_space();

        let mut x = Vec::new();

        if self.token != Token::RParen {
          let y = self.parse_expr(alloc)?;
          x.push(y);

          loop {
            if self.token == Token::RParen { break; }
            self.expect(Token::Comma)?;
            self.advance();
            self.advance_if_space();
            let y = self.parse_expr(alloc)?;
            x.push(y);
          }
        }

        self.advance();

        let x = &*alloc.copy_slice(x.as_slice());
        let a = &*alloc.alloc().init((e, x));
        e = AstExpr::App(a);
      } else {
        break;
      }
    }

    self.advance_if_space();

    Ok(e)
  }
}

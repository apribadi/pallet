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
      Err(ParseError)
    }
  }

  fn span(&self) -> &'a [u8] {
    self.lexer.span()
  }

  fn is_end_of_block(&self) -> bool {
    match self.token {
      Token::End | Token::Elif | Token::Else => true,
      _ => false
    }
  }

  pub fn parse_binding<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<&'b str, ParseError> {
    self.expect(Token::Sym)?;
    let s = aa.copy_str(str::from_utf8(self.span()).unwrap());
    self.advance();
    self.advance_if_space();
    Ok(s)
  }

  pub fn parse_stmt_seq<'b>(
      &mut self,
      aa: &mut Allocator<'b>
    ) -> Result<&'b [AstStmt<'b>], ParseError>
  {
    let mut x = Vec::new();
    while ! self.is_end_of_block() {
      let s = self.parse_stmt(aa)?;
      x.push(s);
    }
    Ok(aa.copy_slice(x.as_slice()))
  }

  pub fn parse_stmt<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstStmt<'b>, ParseError> {
    match self.token {
      Token::Break => {
        self.advance();
        self.advance_if_space();
        Ok(AstStmt::Break)
      }
      Token::Return => {
        self.advance();
        self.advance_if_space();
        Ok(AstStmt::Return)
      }
      Token::Let => {
        self.advance();
        self.advance_if_space();

        let mut x = Vec::new();
        let s = self.parse_binding(aa)?;
        x.push(s);
        while self.token == Token::Comma {
          self.advance();
          self.advance_if_space();
          let s = self.parse_binding(aa)?;
          x.push(s);
        }
        let x = aa.copy_slice(x.as_slice());

        self.expect(Token::Assign)?;
        self.advance();
        self.advance_if_space();

        let mut y = Vec::new();
        let e = self.parse_expr(aa)?;
        y.push(e);
        while self.token == Token::Comma {
          self.advance();
          self.advance_if_space();
          let e = self.parse_expr(aa)?;
          y.push(e);
        }
        let y = aa.copy_slice(y.as_slice());

        let a = aa.alloc().init(AstLet(x, y));
        Ok(AstStmt::Let(a))
      }
      _ => {
        let mut x = Vec::new();
        let e = self.parse_expr(aa)?;
        x.push(e);
        while self.token == Token::Comma {
          self.advance();
          self.advance_if_space();
          let e = self.parse_expr(aa)?;
          x.push(e);
        }
        let x = aa.copy_slice(x.as_slice());
        let a = aa.alloc().init(AstValues(x));
        Ok(AstStmt::Values(a))
      }
    }
  }

  pub fn parse_expr<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    self.parse_expr_c(aa)
  }

  // "c"omparison

  pub fn parse_expr_c<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_a(aa)?;

    loop {
      if let token @ (
          Token::EQ |
          Token::NE |
          Token::GT |
          Token::GE |
          Token::LT |
          Token::LE
        ) = self.token
      {
        let op =
          match token {
            Token::EQ => AstOp::EQ,
            Token::NE => AstOp::NE,
            Token::GT => AstOp::NE,
            Token::GE => AstOp::NE,
            Token::LT => AstOp::NE,
            Token::LE => AstOp::NE,
            _ => panic!(),
          };
        self.advance();
        self.advance_if_space();

        let x = self.parse_expr_a(aa)?;
        let a = aa.alloc().init(AstApp(op, [e, x]));
        e = AstExpr::App2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "a"ddition

  pub fn parse_expr_a<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_m(aa)?;

    loop {
      if let token @ (Token::Minus | Token::Plus) = self.token {
        let op =
          match token {
            Token::Minus => AstOp::Sub,
            Token::Plus => AstOp::Add,
            _ => panic!(),
          };
        self.advance();
        self.advance_if_space();
        let x = self.parse_expr_m(aa)?;
        let a = aa.alloc().init(AstApp(op, [e, x]));
        e = AstExpr::App2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "m"ultiplication

  pub fn parse_expr_m<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_p(aa)?;

    loop {
      if let Token::Star = self.token {
        self.advance();
        self.advance_if_space();
        let x = self.parse_expr_p(aa)?;
        let a = aa.alloc().init(AstApp(AstOp::Mul, [e, x]));
        e = AstExpr::App2(a);
      } else {
        break;
      }
    }

    Ok(e)
  }

  // "p"refix

  pub fn parse_expr_p<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    if let Token::Minus = self.token {
      self.advance();
      self.advance_if_space();
      let x = self.parse_expr_p(aa)?;
      let a = aa.alloc().init(AstApp(AstOp::Neg, [x]));
      Ok(AstExpr::App1(a))
    } else {
      self.parse_expr_t(aa)
    }
  }

  // "t"erminal (and funcalls)

  pub fn parse_expr_t<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e =
      match self.token {
        Token::Num => {
          let s = aa.copy_str(str::from_utf8(self.span()).unwrap());
          self.advance();
          AstExpr::Num(s)
        }
        Token::Sym => {
          let s = aa.copy_str(str::from_utf8(self.span()).unwrap());
          self.advance();
          AstExpr::Sym(s)
        }
        Token::LParen => {
          self.advance();
          self.advance_if_space();
          let expr = self.parse_expr(aa)?;
          self.expect(Token::RParen)?;
          self.advance();
          expr
        }
        Token::Loop => {
          // TODO: disallow a function call where the function is a control
          // expression

          self.advance();
          self.advance_if_space();
          let body = self.parse_stmt_seq(aa)?;
          match self.token {
            Token::End => {
            }
            _ => {
              self.fail()?;
            }
          }
          self.advance();
          let a = aa.alloc().init(AstLoop(body));
          AstExpr::Loop(a)
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
          let y = self.parse_expr(aa)?;
          x.push(y);

          loop {
            if self.token == Token::RParen { break; }
            self.expect(Token::Comma)?;
            self.advance();
            self.advance_if_space();
            let y = self.parse_expr(aa)?;
            x.push(y);
          }
        }

        self.advance();

        let x = aa.copy_slice(x.as_slice());
        let a = aa.alloc().init(AstCall(e, x));
        e = AstExpr::Call(a);
      } else {
        break;
      }
    }

    self.advance_if_space();

    Ok(e)
  }
}

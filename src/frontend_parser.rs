use crate::prelude::*;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  token: Token,
}

#[derive(Debug)]
pub struct ParseError;

fn is_block_terminator(token: Token) -> bool {
  match token {
    Token::End | Token::Elif | Token::Else => true,
    _ => false
  }
}

impl<'a> Parser<'a> {
  pub fn new(buf: &'a [u8]) -> Self {
    let mut lexer = Lexer::new(buf);
    let token = lexer.next();
    Self { lexer, token, }
  }

  fn advance(&mut self) {
    self.token = self.lexer.next()
  }

  fn advance_over_space(&mut self) {
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

  pub fn parse_symbol<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstSymbol<'b>, ParseError> {
    self.expect(Token::Symbol)?;
    let x = AstSymbol(aa.copy_str(str::from_utf8(self.span()).unwrap()));
    self.advance();
    self.advance_over_space();
    Ok(x)
  }

  pub fn parse_item<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstItem<'b>, ParseError> {
    match self.token {
      Token::Fun => {
        let x = self.parse_fundef(aa)?;
        Ok(AstItem::FunDef(aa.alloc().init(x)))
      }
      _ => {
        self.fail()
      }
    }
  }

  pub fn parse_fundef<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstFunDef<'b>, ParseError> {
    self.expect(Token::Fun)?;
    self.advance();
    self.advance_over_space();
    let name = self.parse_symbol(aa)?;
    self.expect(Token::LParen)?;
    self.advance();
    self.advance_over_space();
    let mut params = Vec::new();
    if self.token != Token::RParen {
      let param = self.parse_symbol(aa)?;
      params.push(param);
      while self.token != Token::RParen {
        self.expect(Token::Comma)?;
        self.advance();
        self.advance_over_space();
        let param = self.parse_symbol(aa)?;
        params.push(param);
      }
    }
    let params = aa.copy_slice(params.as_slice());
    self.advance();
    self.advance_over_space();
    let body = self.parse_stmt_seq(aa)?;
    self.expect(Token::End)?;
    self.advance();
    self.advance_over_space();
    Ok(AstFunDef { name, params, body })
  }

  pub fn parse_stmt_seq<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<&'b [AstStmt<'b>], ParseError> {
    let mut a = Vec::new();
    while ! is_block_terminator(self.token) {
      let x = self.parse_stmt(aa)?;
      a.push(x);
    }
    Ok(aa.copy_slice(a.as_slice()))
  }

  pub fn parse_expr_nonempty_seq<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<&'b [AstExpr<'b>], ParseError> {
    let mut a = Vec::new();
    let x = self.parse_expr(aa)?;
    a.push(x);
    while self.token == Token::Comma {
      self.advance();
      self.advance_over_space();
      let x = self.parse_expr(aa)?;
      a.push(x);
    }
    Ok(aa.copy_slice(a.as_slice()))
  }

  pub fn parse_symbol_nonempty_seq<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<&'b [AstSymbol<'b>], ParseError> {
    let mut a = Vec::new();
    let x = self.parse_symbol(aa)?;
    a.push(x);
    while self.token == Token::Comma {
      self.advance();
      self.advance_over_space();
      let x = self.parse_symbol(aa)?;
      a.push(x);
    }
    Ok(aa.copy_slice(a.as_slice()))
  }

  pub fn parse_stmt<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstStmt<'b>, ParseError> {
    match self.token {
      Token::Break => {
        self.advance();
        self.advance_over_space();
        let x =
          if is_block_terminator(self.token) {
            &[]
          } else {
            self.parse_expr_nonempty_seq(aa)?
          };
        Ok(AstStmt::Break(aa.alloc().init(AstBreak(x))))
      }
      Token::Let => {
        self.advance();
        self.advance_over_space();
        let x = self.parse_symbol_nonempty_seq(aa)?;
        self.expect(Token::Assign)?;
        self.advance();
        self.advance_over_space();
        let y = self.parse_expr_nonempty_seq(aa)?;
        Ok(AstStmt::Let(aa.alloc().init(AstLet(x, y))))
      }
      Token::Return => {
        self.advance();
        self.advance_over_space();
        let x =
          if is_block_terminator(self.token) {
            &[]
          } else {
            self.parse_expr_nonempty_seq(aa)?
          };
        Ok(AstStmt::Return(aa.alloc().init(AstReturn(x))))
      }
      _ => {
        let x = self.parse_expr_nonempty_seq(aa)?;
        Ok(AstStmt::ExprSeq(aa.alloc().init(AstExprSeq(x))))
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
      let op =
        match self.token {
          Token::EQ => AstOp::EQ,
          Token::NE => AstOp::NE,
          Token::GT => AstOp::GT,
          Token::GE => AstOp::GE,
          Token::LT => AstOp::LT,
          Token::LE => AstOp::LE,
          _ => { break; }
        };
      self.advance();
      self.advance_over_space();
      let x = self.parse_expr_a(aa)?;
      e = AstExpr::OpCall2(aa.alloc().init(AstOpCall(op, [e, x])));
    }

    Ok(e)
  }

  // "a"ddition

  pub fn parse_expr_a<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_m(aa)?;

    loop {
      let op =
        match self.token {
          Token::Minus => AstOp::Sub,
          Token::Plus => AstOp::Add,
          _ => { break; }
        };
      self.advance();
      self.advance_over_space();
      let x = self.parse_expr_m(aa)?;
      e = AstExpr::OpCall2(aa.alloc().init(AstOpCall(op, [e, x])));
    }

    Ok(e)
  }

  // "m"ultiplication

  pub fn parse_expr_m<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e = self.parse_expr_p(aa)?;

    loop {
      let op =
        match self.token {
          Token::Slash => AstOp::Div,
          Token::Star => AstOp::Mul,
          _ => { break; }
        };
      self.advance();
      self.advance_over_space();
      let x = self.parse_expr_p(aa)?;
      e = AstExpr::OpCall2(aa.alloc().init(AstOpCall(op, [e, x])));
    }

    Ok(e)
  }

  // "p"refix

  pub fn parse_expr_p<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let op =
      match self.token {
        Token::Bang => AstOp::Not,
        Token::Minus => AstOp::Neg,
        _ => { return self.parse_expr_t(aa); }
      };
    self.advance();
    self.advance_over_space();
    let x = self.parse_expr_p(aa)?;
    Ok(AstExpr::OpCall1(aa.alloc().init(AstOpCall(op, [x]))))
  }

  // "t"erminal (and funcalls)

  pub fn parse_expr_t<'b>(&mut self, aa: &mut Allocator<'b>) -> Result<AstExpr<'b>, ParseError> {
    let mut e =
      match self.token {
        Token::LParen => {
          self.advance();
          self.advance_over_space();
          let x = self.parse_expr(aa)?;
          self.expect(Token::RParen)?;
          self.advance();
          x
        }
        Token::Number => {
          let x = AstNumber(aa.copy_str(str::from_utf8(self.span()).unwrap()));
          self.advance();
          AstExpr::Number(aa.alloc().init(x))
        }
        Token::Symbol => {
          let x = AstSymbol(aa.copy_str(str::from_utf8(self.span()).unwrap()));
          self.advance();
          AstExpr::Symbol(aa.alloc().init(x))
        }
        Token::If => {
          self.advance();
          self.advance_over_space();
          let x = self.parse_expr(aa)?;
          self.expect(Token::Then)?;
          self.advance();
          self.advance_over_space();
          let y = self.parse_stmt_seq(aa)?;
          let z =
            match self.token {
              Token::Else => {
                self.advance();
                self.advance_over_space();
                self.parse_stmt_seq(aa)?
              }
              _ => {
                &[]
              }
            };
          self.expect(Token::End)?;
          self.advance();
          AstExpr::If(aa.alloc().init(AstIf(x, y, z)))
        }
        Token::Loop => {
          self.advance();
          self.advance_over_space();
          let x = self.parse_stmt_seq(aa)?;
          self.expect(Token::End)?;
          self.advance();
          AstExpr::Loop(aa.alloc().init(AstLoop(x)))
        }
        _ => {
          return self.fail();
        }
      };

    // Leave unconsumed space, because there can't be space between a function
    // and its arguments.

    while self.token == Token::LParen {
      self.advance();
      self.advance_over_space();

      let mut a = Vec::new();

      if self.token != Token::RParen {
        let x = self.parse_expr(aa)?;
        a.push(x);

        while self.token != Token::RParen {
          self.expect(Token::Comma)?;
          self.advance();
          self.advance_over_space();
          let x = self.parse_expr(aa)?;
          a.push(x);
        }
      }

      self.advance();

      e = AstExpr::FunCall(aa.alloc().init(AstFunCall(e, aa.copy_slice(a.as_slice()))))
    }

    self.advance_over_space();

    Ok(e)
  }
}

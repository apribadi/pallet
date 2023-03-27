use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum AstStmt<'a> {
  Break,
  Let(&'a AstLet<'a>),
  Return,
  Values(&'a AstValues<'a>),
}

#[derive(Clone, Copy)]
pub enum AstExpr<'a> {
  App1(&'a AstApp<'a, 1>),
  App2(&'a AstApp<'a, 2>),
  Call(&'a AstCall<'a>),
  Loop(&'a AstLoop<'a>),
  Num(&'a str),
  Sym(&'a str),
}

#[derive(Clone, Copy)]
pub enum AstOp {
  Add,
  And,
  Div,
  Mul,
  Neg,
  Not,
  Or,
  Sub,
  Xor,
  EQ,
  NE,
  GT,
  GE,
  LT,
  LE,
}

#[derive(Clone, Copy)]
pub struct AstLet<'a>(pub &'a [&'a str], pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstValues<'a>(pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstApp<'a, const N: usize>(pub AstOp, pub [AstExpr<'a>; N]);

#[derive(Clone, Copy)]
pub struct AstCall<'a>(pub AstExpr<'a>, pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstLoop<'a>(pub &'a [AstStmt<'a>]);

impl<'a> AstStmt<'a> {
  pub fn to_sexp(&self) -> Sexp {
    match self {
      Self::Break => Sexp::sym("break"),
      Self::Let(a) => a.to_sexp(),
      Self::Return => Sexp::sym("return"),
      Self::Values(a) => a.to_sexp(),
    }
  }
}

impl<'a> AstExpr<'a> {
  pub fn to_sexp(&self) -> Sexp {
    match self {
      Self::App1(a) => a.to_sexp(),
      Self::App2(a) => a.to_sexp(),
      Self::Call(a) => a.to_sexp(),
      Self::Loop(a) => a.to_sexp(),
      Self::Num(a) => Sexp::sym(a),
      Self::Sym(a) => Sexp::sym(a),
    }
  }
}

impl<'a> AstLet<'a> {
  pub fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::sym("let"));
    for x in self.0.iter() { a.push(Sexp::sym(x)) }
    a.push(Sexp::sym("="));
    for x in self.1.iter() { a.push(x.to_sexp()) }
    Sexp::Seq(a.into_boxed_slice())
  }
}

impl<'a> AstValues<'a> {
  pub fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    for x in self.0.iter() { a.push(x.to_sexp()) }
    Sexp::Seq(a.into_boxed_slice())
  }
}

impl<'a> AstCall<'a> {
  pub fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(self.0.to_sexp());
    for x in self.1.iter() { a.push(x.to_sexp()) }
    Sexp::Seq(a.into_boxed_slice())
  }
}

impl<'a> AstLoop<'a> {
  pub fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::sym("loop"));
    for x in self.0.iter() { a.push(x.to_sexp()) }
    Sexp::Seq(a.into_boxed_slice())
  }
}

impl<'a, const N: usize> AstApp<'a, N> {
  pub fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::sym(self.0.to_str()));
    for x in self.1.iter() { a.push(x.to_sexp()) }
    Sexp::Seq(a.into_boxed_slice())
  }
}

impl AstOp {
  fn to_str(&self) -> &'static str {
    match self {
      Self::Add => "+",
      Self::And => "&",
      Self::Div => "/",
      Self::Mul => "*",
      Self::Neg => "-/neg",
      Self::Not => "!",
      Self::Or => "|",
      Self::Sub => "-",
      Self::Xor => "^",
      Self::EQ => "==",
      Self::NE => "!=",
      Self::GT => ">",
      Self::GE => ">=",
      Self::LT => "<",
      Self::LE => "<=",
    }
  }
}

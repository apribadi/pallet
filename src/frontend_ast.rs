use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum AstItem<'a> {
  FunDef(&'a AstFunDef<'a>),
}

#[derive(Clone, Copy)]
pub enum AstStmt<'a> {
  Break(&'a AstBreak<'a>),
  ExprSeq(&'a AstExprSeq<'a>),
  Let(&'a AstLet<'a>),
  Return(&'a AstReturn<'a>),
}

#[derive(Clone, Copy)]
pub enum AstExpr<'a> {
  FunCall(&'a AstFunCall<'a>),
  If(&'a AstIf<'a>),
  Loop(&'a AstLoop<'a>),
  Number(&'a AstNumber<'a>),
  OpCall1(&'a AstOpCall<'a, 1>),
  OpCall2(&'a AstOpCall<'a, 2>),
  Symbol(&'a AstSymbol<'a>),
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
pub struct AstFunDef<'a> {
  pub name: AstSymbol<'a>,
  pub params: &'a [AstSymbol<'a>],
  pub body: &'a [AstStmt<'a>],
}

#[derive(Clone, Copy)]
pub struct AstBreak<'a>(pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstExprSeq<'a>(pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstLet<'a>(pub &'a [AstSymbol<'a>], pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstReturn<'a>(pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstFunCall<'a>(pub AstExpr<'a>, pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstIf<'a>(pub AstExpr<'a>, pub &'a [AstStmt<'a>], pub &'a [AstStmt<'a>]);

#[derive(Clone, Copy)]
pub struct AstLoop<'a>(pub &'a [AstStmt<'a>]);

#[derive(Clone, Copy)]
pub struct AstNumber<'a>(pub &'a str);

#[derive(Clone, Copy)]
pub struct AstOpCall<'a, const N: usize>(pub AstOp, pub [AstExpr<'a>; N]);

#[derive(Clone, Copy)]
pub struct AstSymbol<'a>(pub &'a str);

fn sexp_list<T>(list: &[T]) -> Sexp
where
  T: ToSexp
{
  let mut a = Vec::new();
  for x in list.iter() {
    a.push(x.to_sexp())
  }
  Sexp::List(a.into_boxed_slice())
}

fn sexp_head_and_body<T>(head: Sexp, body: &[T]) -> Sexp
where
  T: ToSexp
{
  let mut a = Vec::new();
  a.push(head);
  for x in body.iter() {
    a.push(x.to_sexp())
  }
  Sexp::List(a.into_boxed_slice())
}

impl<'a> ToSexp for AstItem<'a> {
  fn to_sexp(&self) -> Sexp {
    match self {
      Self::FunDef(x) => x.to_sexp(),
    }
  }
}

impl<'a> ToSexp for AstStmt<'a> {
  fn to_sexp(&self) -> Sexp {
    match self {
      Self::Break(x) => x.to_sexp(),
      Self::ExprSeq(x) => x.to_sexp(),
      Self::Let(x) => x.to_sexp(),
      Self::Return(x) => x.to_sexp(),
    }
  }
}

impl<'a> ToSexp for AstExpr<'a> {
  fn to_sexp(&self) -> Sexp {
    match self {
      Self::FunCall(x) => x.to_sexp(),
      Self::If(x) => x.to_sexp(),
      Self::Loop(x) => x.to_sexp(),
      Self::Number(x) => x.to_sexp(),
      Self::OpCall1(x) => x.to_sexp(),
      Self::OpCall2(x) => x.to_sexp(),
      Self::Symbol(x) => x.to_sexp(),
    }
  }
}

impl<'a> ToSexp for AstFunDef<'a> {
  fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::from_atom("fundef"));
    a.push(self.name.to_sexp());
    a.push(sexp_list(self.params));
    for stmt in self.body.iter() {
      a.push(stmt.to_sexp())
    }
    Sexp::List(a.into_boxed_slice())
  }
}

impl<'a> ToSexp for AstBreak<'a> {
  fn to_sexp(&self) -> Sexp {
    sexp_head_and_body(Sexp::from_atom("break"), self.0)
  }
}

impl<'a> ToSexp for AstLet<'a> {
  fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::from_atom("let"));
    for x in self.0.iter() { a.push(x.to_sexp()) }
    a.push(Sexp::from_atom("="));
    for x in self.1.iter() { a.push(x.to_sexp()) }
    Sexp::List(a.into_boxed_slice())
  }
}

impl<'a> ToSexp for AstReturn<'a> {
  fn to_sexp(&self) -> Sexp {
    sexp_head_and_body(Sexp::from_atom("return"), self.0)
  }
}

impl<'a> ToSexp for AstExprSeq<'a> {
  fn to_sexp(&self) -> Sexp {
    sexp_head_and_body(Sexp::from_atom("exprseq"), self.0)
  }
}

impl<'a> ToSexp for AstFunCall<'a> {
  fn to_sexp(&self) -> Sexp {
    sexp_head_and_body(self.0.to_sexp(), self.1)
  }
}

impl<'a> ToSexp for AstIf<'a> {
  fn to_sexp(&self) -> Sexp {
    Sexp::List(
      Box::new([
        Sexp::from_atom("if"),
        self.0.to_sexp(),
        sexp_head_and_body(Sexp::from_atom("then"), self.1),
        sexp_head_and_body(Sexp::from_atom("else"), self.2),
      ])
    )
  }
}

impl<'a> ToSexp for AstLoop<'a> {
  fn to_sexp(&self) -> Sexp {
    let mut a = Vec::new();
    a.push(Sexp::from_atom("loop"));
    for x in self.0.iter() { a.push(x.to_sexp()) }
    Sexp::List(a.into_boxed_slice())
  }
}

impl<'a> ToSexp for AstNumber<'a> {
  fn to_sexp(&self) -> Sexp {
    Sexp::from_atom(self.0)
  }
}

impl<'a, const N: usize> ToSexp for AstOpCall<'a, N> {
  fn to_sexp(&self) -> Sexp {
    sexp_head_and_body(Sexp::from_atom(self.0.to_name()), &self.1)
  }
}

impl<'a> ToSexp for AstSymbol<'a> {
  fn to_sexp(&self) -> Sexp {
    Sexp::from_atom(self.0)
  }
}

impl AstOp {
  fn to_name(self) -> &'static str {
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

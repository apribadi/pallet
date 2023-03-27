use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum AstExpr<'a> {
  App(&'a AstApp<'a>),
  Num(&'a str),
  Op1(&'a AstOp<'a, 1>),
  Op2(&'a AstOp<'a, 2>),
  Sym(&'a str),
}

#[derive(Clone, Copy)]
pub struct AstApp<'a>(pub AstExpr<'a>, pub &'a [AstExpr<'a>]);

#[derive(Clone, Copy)]
pub struct AstOp<'a, const N: usize>(pub &'a str, pub [AstExpr<'a>; N]);

impl<'a> AstExpr<'a> {
  pub fn sexp(self, alloc: &mut Allocator<'a>) -> Sexp<'a> {
    match self {
      Self::App(s) => s.sexp(alloc),
      Self::Num(s) => Sexp::Sym(s),
      Self::Op1(s) => s.sexp(alloc),
      Self::Op2(s) => s.sexp(alloc),
      Self::Sym(s) => Sexp::Sym(s),
    }
  }
}

impl<'a> AstApp<'a> {
  pub fn sexp(self, alloc: &mut Allocator<'a>) -> Sexp<'a> {
    Sexp::Seq(
      alloc.alloc_slice(1 + self.1.len()).init_slice(|i|
        if i == 0 {
          self.0.sexp(alloc)
        } else {
          self.1[i - 1].sexp(alloc)
        }
      )
    )
  }
}

impl<'a, const N: usize> AstOp<'a, N> {
  pub fn sexp(self, alloc: &mut Allocator<'a>) -> Sexp<'a> {
    Sexp::Seq(
      alloc.alloc_slice(1 + N).init_slice(|i|
        if i == 0 {
          Sexp::Sym(self.0)
        } else {
          self.1[i - 1].sexp(alloc)
        }
      )
    )
  }
}

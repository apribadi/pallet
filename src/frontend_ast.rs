use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum AstExpr<'a> {
  App(&'a (AstExpr<'a>, &'a [AstExpr<'a>])),
  Num(&'a str),
  Op1(&'a (&'static str, [AstExpr<'a>; 1])),
  Op2(&'a (&'static str, [AstExpr<'a>; 2])),
  Sym(&'a str),
}

impl<'a> AstExpr<'a> {
  pub fn sexp(self, alloc: &mut Allocator<'a>) -> Sexp<'a> {
    match self {
      Self::App(&(fun, args)) => {
        let s = alloc.alloc_slice(1 + args.len());
        let s =
          s.init_slice(|i|
            if i == 0 {
              fun.sexp(alloc)
            } else {
              args[i - 1].sexp(alloc)
            }
          );
        Sexp::Seq(s)
      }
      Self::Num(s) =>
        Sexp::Sym(s),
      Self::Op1(&(op, [x])) => {
        let x = x.sexp(alloc);
        Sexp::Seq(alloc.copy_slice(&[Sexp::Sym(op), x]))
      }
      Self::Op2(&(op, [x, y])) => {
        let x = x.sexp(alloc);
        let y = y.sexp(alloc);
        Sexp::Seq(alloc.copy_slice(&[Sexp::Sym(op), x, y]))
      }
      Self::Sym(s) =>
        Sexp::Sym(s),
    }
  }
}

// use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum AstExpr<'a> {
  FunCall(&'a (AstExpr<'a>, &'a [AstExpr<'a>])),
  Op1(&'a (&'static str, [AstExpr<'a>; 1])),
  Op2(&'a (&'static str, [AstExpr<'a>; 2])),
  Number(&'a str),
  Symbol(&'a str),
}

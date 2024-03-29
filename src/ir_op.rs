use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Op11 {
  BoolNot,
  I128HiI64,
  I128ToI64,
  I64Abs,
  I64BitNot,
  I64Clz,
  I64Ctz,
  I64IsNonZero,
  I64Neg,
  I64Popcount,
  I64RevBits,
  I64RevBytes,
  I64ToI6,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Op21 {
  BoolAnd,
  BoolEq,
  BoolNeq,
  BoolOr,
  I64Add,
  I64BitAnd,
  I64BitOr,
  I64BitXor,
  I64IsEq,
  I64IsGeS,
  I64IsGeU,
  I64IsGtS,
  I64IsGtU,
  I64IsLeS,
  I64IsLeU,
  I64IsLtS,
  I64IsLtU,
  I64IsNeq,
  I64MaxS,
  I64MaxU,
  I64MinS,
  I64MinU,
  I64Mul,
  I64MulFullS,
  I64MulFullU,
  I64MulHiS,
  I64MulHiU,
  I64Rol,
  I64Ror,
  I64Shl,
  I64ShrS,
  I64ShrU,
  I64Sub,
}

impl fmt::Display for Op11 {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(out, "{:?}", self)
  }
}

impl fmt::Display for Op21 {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(out, "{:?}", self)
  }
}

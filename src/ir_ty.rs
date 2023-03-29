use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Ty {
  Bool,
  I128,
  I6,
  I64,
}

impl fmt::Display for Ty {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(out, "{:?}", self)
  }
}

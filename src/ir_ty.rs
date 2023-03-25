use crate::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum Ty {
  Bool,
  I128,
  I6,
  I64,
}

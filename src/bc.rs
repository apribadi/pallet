/*
use crate::prelude::*;
use core::mem::transmute;

#[repr(transparent)]
pub struct Prog([u8]);

#[repr(transparent)]
pub struct Fun([u8]);

#[repr(transparent)]
pub struct FunList([u8]);

#[repr(transparent)]
pub struct FunType([u8]);

#[repr(transparent)]
pub struct ValTypeList([u8]);

#[repr(transparent)]
pub struct InstList([u8]);

pub struct IterFunList<'a> {
  list: &'a FunList,
  offset: usize,
}

pub struct IterValTypeList<'a> {
  list: &'a ValTypeList,
  offset: usize,
}

pub struct IterInstList<'a> {
  list: &'a InstList,
  offset: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct VarId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ValType {
  Bool,
  I6,
  I64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp11 {
  BoolNot,
  I64BitNot,
  I64Clz,
  I64Ctz,
  I64IsNonZero,
  I64Neg,
  I64Popcount,
  I64Swap,
  I64ToI6,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp21 {
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
  I64MulHiS,
  I64MulHiU,
  I64Rol,
  I64Ror,
  I64Shl,
  I64ShrS,
  I64ShrU,
  I64Sub,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp22 {
  I64MulFullS,
  I64MulFullU,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp31 {
  I64Sel,
}

impl Fun {
  // ofs0
  // ofs1
  // name     | [8   , ofs0)
  // funtype  | [ofs0, ofs1)
  // instlist | [ofs1, len )

  /*
  pub fn name(&self) -> &str {

  }
  pub fn funtype(&self) -> &FunType {
  }
  */
}

impl ValTypeList {
  pub fn iter(&self) -> IterValTypeList<'_> {
    IterValTypeList { list: self, offset: 0, }
  }
}

impl<'a> Iterator for IterValTypeList<'a> {
  type Item = ValType;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    let a = &self.list.0;
    let i = self.offset;

    if i == a.len() { return None; }

    let x = a[i];
    let x = ValType::decode(x);

    self.offset = i + 1;

    Some(x)
  }
}

impl ValType {
  const MAX_VALUE: Self = Self::I64;

  #[inline(always)]
  pub fn is_valid(x: u8) -> bool {
    x <= Self::MAX_VALUE.encode()
  }

  #[inline(always)]
  pub fn encode(self) -> u8 {
    self as u8
  }

  #[inline(always)]
  pub unsafe fn decode_unchecked(x: u8) -> Self {
    unsafe { transmute::<u8, Self>(x) }
  }

  #[inline(always)]
  pub fn decode(x: u8) -> Self {
    assert!(Self::is_valid(x));
    unsafe { Self::decode_unchecked(x) }
  }
}

#[inline(always)]
fn get_u8s<const N: usize>(data: &[u8], index: usize) -> [u8; N] {
  let len = data.len();
  assert!(index <= len && N <= len - index);
  array::from_fn(|i| unsafe { *data.get_unchecked(index + i) })
}

#[inline(always)]
fn get_u16(data: &[u8], index: usize) -> u16 {
  u16::from_le_bytes(get_u8s(data, index))
}

#[inline(always)]
fn get_u32(data: &[u8], index: usize) -> u32 {
  u32::from_le_bytes(get_u8s(data, index))
}
*/

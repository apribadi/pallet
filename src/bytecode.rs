use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Program<'a> {
  pub functions: &'a [Function<'a>],
}

#[derive(Clone, Copy)]
pub struct Signature<'a> {
  pub inputs: &'a [ValType],
  pub outputs: &'a [ValType],
}

#[derive(Clone, Copy)]
pub struct Function<'a> {
  pub name: &'a str,
  pub signature: Signature<'a>,
  pub code: &'a [Inst<'a>],
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct VarId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ValType {
  Bool,
  FunRef,
  I6,
  I64,
  Ref,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp11 {
  BoolNot,
  I64BitNot,
  I64Clz,
  I64Ctz,
  I64IsZero,
  I64Neg,
  I64Popcnt,
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
  I64MulFull
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagOp31 {
  I64Sel,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagIf1 {
  I64IfZero,
  If,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagIf2 {
  I64IfEq,
  I64IfGeS,
  I64IfGeU,
  I64IfGtS,
  I64IfGtU,
  I64IfLeS,
  I64IfLeU,
  I64IfLtS,
  I64IfLtU,
  I64IfNeq,
}

#[derive(Clone, Copy)]
pub enum Imm {
  Bool(bool),
  I6(u6),
  I64(u64),
}

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(&'a [ValType]),
  FunCall,
  FunCallIndirect,
  FunTailCall,
  FunTailCallIndirect,
  If1(TagIf1, VarId, BlockId, &'a [VarId], BlockId, &'a [VarId]),
  If2(TagIf2, VarId, VarId, BlockId, &'a [VarId], BlockId, &'a [VarId]),
  Jump(BlockId, &'a [VarId]),
  Op01(Imm),
  Op11(TagOp11, VarId),
  Op21(TagOp21, VarId, VarId),
  Op22(TagOp22, VarId, VarId),
  Op31(TagOp21, VarId, VarId, VarId),
  Return(&'a [VarId]),
}

impl From<VarId> for usize {
  #[inline(always)]
  fn from(x: VarId) -> usize {
    x.0 as usize
  }
}

impl TagOp11 {
  pub fn types(self) -> ([ValType; 1], [ValType; 1]) {
    TYPE_OP_11[self as u8 as usize]
  }
}

impl TagOp21 {
  pub fn types(self) -> ([ValType; 2], [ValType; 1]) {
    TYPE_OP_21[self as usize]
  }
}

impl TagOp22 {
  pub fn types(self) -> ([ValType; 2], [ValType; 2]) {
    TYPE_OP_22[self as u8 as usize]
  }
}

impl TagOp31 {
  pub fn types(self) -> ([ValType; 3], [ValType; 1]) {
    TYPE_OP_31[self as usize]
  }
}

////////////////////////////////////////////////////////////////////////////////
//
// TYPING
//
////////////////////////////////////////////////////////////////////////////////

use ValType::*;

pub(crate) static TYPE_OP_11: [([ValType; 1], [ValType; 1]); 9] = [
  /* BoolNot     */ ([Bool], [Bool]),
  /* I64BitNot   */ ([I64], [I64]),
  /* I64Clz      */ ([I64], [I64]),
  /* I64Ctz      */ ([I64], [I64]),
  /* I64IsZero   */ ([I64], [Bool]),
  /* I64Neg      */ ([I64], [I64]),
  /* I64Popcnt   */ ([I64], [I64]),
  /* I64Swap     */ ([I64], [I64]),
  /* I64ToI6     */ ([I64], [I6]),
];

pub(crate) static TYPE_OP_21: [([ValType; 2], [ValType; 1]); 31] = [
  /* BoolAnd     */ ([Bool, Bool], [Bool]),
  /* BoolEq      */ ([Bool, Bool], [Bool]),
  /* BoolNeq     */ ([Bool, Bool], [Bool]),
  /* BoolOr      */ ([Bool, Bool], [Bool]),
  /* I64Add      */ ([I64, I64], [I64]),
  /* I64BitAnd   */ ([I64, I64], [I64]),
  /* I64BitOr    */ ([I64, I64], [I64]),
  /* I64BitXor   */ ([I64, I64], [I64]),
  /* I64IsEq     */ ([I64, I64], [Bool]),
  /* I64IsGeS    */ ([I64, I64], [Bool]),
  /* I64IsGeU    */ ([I64, I64], [Bool]),
  /* I64IsGtS    */ ([I64, I64], [Bool]),
  /* I64IsGtU    */ ([I64, I64], [Bool]),
  /* I64IsLeS    */ ([I64, I64], [Bool]),
  /* I64IsLeU    */ ([I64, I64], [Bool]),
  /* I64IsLtS    */ ([I64, I64], [Bool]),
  /* I64IsLtU    */ ([I64, I64], [Bool]),
  /* I64IsNeq    */ ([I64, I64], [Bool]),
  /* I64MaxS     */ ([I64, I64], [I64]),
  /* I64MaxU     */ ([I64, I64], [I64]),
  /* I64MinS     */ ([I64, I64], [I64]),
  /* I64MinU     */ ([I64, I64], [I64]),
  /* I64Mul      */ ([I64, I64], [I64]),
  /* I64MulHiS   */ ([I64, I64], [I64]),
  /* I64MulHiU   */ ([I64, I64], [I64]),
  /* I64Rol      */ ([I64, I6], [I64]),
  /* I64Ror      */ ([I64, I6], [I64]),
  /* I64Shl      */ ([I64, I6], [I64]),
  /* I64ShrS     */ ([I64, I6], [I64]),
  /* I64ShrU     */ ([I64, I6], [I64]),
  /* I64Sub      */ ([I64, I64], [I64]),
];

pub(crate) static TYPE_OP_22: [([ValType; 2], [ValType; 2]); 2] = [
  /* I64MulFullS */ ([I64, I64], [I64, I64]),
  /* I64MulFullU */ ([I64, I64], [I64, I64]),
];

pub(crate) static TYPE_OP_31: [([ValType; 3], [ValType; 1]); 1] = [
  /* I64Sel      */ ([Bool, I64, I64], [I64]),
];

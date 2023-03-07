use crate::prelude::*;
use core::mem::transmute;

#[repr(transparent)]
pub struct Prog([u8]);

#[repr(transparent)]
pub struct FunList([u8]);

pub struct FunListIter<'a>(&'a [u8]);

#[repr(transparent)]
pub struct Fun([u8]);

#[repr(transparent)]
pub struct FunType([u8]);

#[repr(transparent)]
pub struct ValTypeList([u8]);

pub struct ValTypeListIter<'a>(&'a [u8]);

#[repr(transparent)]
pub struct InstList([u8]);

pub struct InstListIter<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(&'a ValTypeList),
  // Const(_),
  If100(&'a If100),
  If200(&'a If200),
  Jump(&'a Jump),
  Op11(&'a Op11),
  Op21(&'a Op21),
  Op22(&'a Op22),
  Op31(&'a Op31),
  Return(&'a Return),
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagInst {
  Block,
  Const,
  If100,
  If200,
  Jump,
  Op11,
  Op21,
  Op22,
  Op31,
  Return,
}

#[repr(transparent)]
pub struct If100([u8]);

#[repr(transparent)]
pub struct If200([u8]);

#[repr(transparent)]
pub struct Jump([u8]);

#[repr(transparent)]
pub struct Op11([u8]);

#[repr(transparent)]
pub struct Op21([u8]);

#[repr(transparent)]
pub struct Op22([u8]);

#[repr(transparent)]
pub struct Op31([u8]);

#[repr(transparent)]
pub struct Return([u8]);

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
pub enum TagIf100 {
  BoolIsTrue,
  I64IsNonZero,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TagIf200 {
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

#[derive(Clone, Copy)]
pub enum Imm {
  Bool(bool),
  I6(u6),
  I64(u64),
}

/*

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(&'a [ValType]),
  Const(Imm),
  If100(TagIf100, VarId, BlockId, BlockId),
  If200(TagIf200, VarId, VarId, BlockId, BlockId),
  Jump(BlockId, &'a [VarId]),
  Op11(TagOp11, VarId),
  Op21(TagOp21, VarId, VarId),
  Op22(TagOp22, VarId, VarId),
  Op31(TagOp31, VarId, VarId, VarId),
  Return(&'a [VarId]),
}

*/

impl Prog {
  pub fn fun_list(&self) -> &FunList {
    let x = &self.0 as *const [u8] as *const FunList;
    unsafe { &*x }
  }
}

impl FunList {
  pub fn iter(&self) -> FunListIter<'_> {
    FunListIter(&self.0)
  }
}

impl<'a> Iterator for FunListIter<'a> {
  type Item = &'a Fun;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    let a = self.0;

    if a.is_empty() { return None; }

    let n = a.get_u32(0) as usize;
    let x = &a[4 .. 4 + n] as *const [u8] as *const Fun;
    let y = &a[4 + n ..];

    self.0 = y;

    Some(unsafe { &*x })
  }
}

impl Fun {
  // ofs1
  // ofs2
  // name      | [ofs0, ofs1)
  // fun_type  | [ofs1, ofs2)
  // inst_list | [ofs2, ofs3)
  //
  // ofs0 = 8
  // ofs3 = len

  #[inline(always)]
  fn ofs0(&self) -> usize { let _ = self; 8 }

  #[inline(always)]
  fn ofs1(&self) -> usize { self.0.get_u32(0) as usize }

  #[inline(always)]
  fn ofs2(&self) -> usize { self.0.get_u32(4) as usize }

  #[inline(always)]
  fn ofs3(&self) -> usize { self.0.len() }

  pub fn name(&self) -> &str {
    let i = self.ofs0();
    let j = self.ofs1();
    let a = &self.0;
    let x = &a[i .. j];
    unsafe { core::str::from_utf8_unchecked(x) }
  }

  pub fn fun_type(&self) -> &FunType {
    let i = self.ofs1();
    let j = self.ofs2();
    let a = &self.0;
    let x = &a[i .. j] as *const [u8] as *const FunType;
    unsafe { &*x }
  }

  pub fn inst_list(&self) -> &InstList {
    let i = self.ofs2();
    let j = self.ofs3();
    let a = &self.0;
    let x = &a[i .. j] as *const [u8] as *const InstList;
    unsafe { &*x }
  }
}

impl FunType {
  // ofs1
  // val_type_list | [ofs0, ofs1)
  // val_type_list | [ofs1, ofs2)
  //
  // ofs0 = 4
  // ofs2 = len

  #[inline(always)]
  fn ofs0(&self) -> usize { let _ = self; 4 }

  #[inline(always)]
  fn ofs1(&self) -> usize { self.0.get_u32(0) as usize }

  #[inline(always)]
  fn ofs2(&self) -> usize { self.0.len() }

  pub fn input(&self) -> &ValTypeList {
    let i = self.ofs0();
    let j = self.ofs1();
    let a = &self.0;
    let x = &a[i .. j] as *const [u8] as *const ValTypeList;
    unsafe { &*x }
  }

  pub fn output(&self) -> &ValTypeList {
    let i = self.ofs1();
    let j = self.ofs2();
    let a = &self.0;
    let x = &a[i .. j] as *const [u8] as *const ValTypeList;
    unsafe { &*x }
  }
}

impl ValTypeList {
  pub fn iter(&self) -> ValTypeListIter<'_> {
    ValTypeListIter(&self.0)
  }
}

impl<'a> Iterator for ValTypeListIter<'a> {
  type Item = ValType;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    let a = self.0;

    if a.is_empty() { return None; }

    let x = a[0];
    let y = &a[1 ..];

    self.0 = y;

    Some(ValType::decode(x))
  }
}

impl InstList {
  pub fn iter(&self) -> InstListIter<'_> {
    InstListIter(&self.0)
  }
}

impl<'a> Iterator for InstListIter<'a> {
  type Item = Inst<'a>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    let a = self.0;

    if a.is_empty() { return None; }

    let (x, y) =
      match TagInst::decode(a[0]) {
        TagInst::Block => {
          // tag_inst u8
          // count    u16
          // types    u8[]

          let n = a.get_u16(1) as usize;
          let x = &a[3 .. 3 + n] as *const [u8] as *const ValTypeList;
          let y = &a[3 + n .. ];

          (Inst::Block(unsafe { &*x }), y)
        }
        TagInst::If100 => {
          // tag_inst  u8
          // tag_if100 u8
          // var_id    u16
          // block_id  u16
          // block_id  u16

          let x = &a[1 .. 8] as *const [u8] as *const If100;
          let y = &a[8 ..];

          (Inst::If100(unsafe { &*x }), y)
        }
        TagInst::If200 => {
          // tag_inst  u8
          // tag_if100 u8
          // var_id    u16
          // var_id    u16
          // block_id  u16
          // block_id  u16

          let x = &a[1 .. 10] as *const [u8] as *const If200;
          let y = &a[10 ..];

          (Inst::If200(unsafe { &*x }), y)
        }
        TagInst::Jump => {
          // tag_inst
          // count
          // ...

          let n = a[1] as usize;
          let x = &a[2 .. 2 + n] as *const [u8] as *const ValTypeList;
          let y = &a[2 + n .. ];

          let x = &a[1 .. 10] as *const [u8] as *const If200;
          let y = &a[10 ..];

          (Inst::If200(unsafe { &*x }), y)
        }
        _ => {
          unimplemented!()
        }
      };

    self.0 = y;

    Some(x)
  }
}

impl TagInst {
  const MAX_VALUE: Self = Self::Return;

  #[inline(always)]
  pub fn is_valid(x: u8) -> bool {
    x <= Self::MAX_VALUE.encode()
  }

  #[inline(always)]
  pub fn encode(self) -> u8 {
    self as u8
  }

  #[inline(always)]
  pub fn decode(x: u8) -> Self {
    assert!(Self::is_valid(x));
    unsafe { transmute::<u8, Self>(x) }
  }
}

impl TagIf100 {
  const MAX_VALUE: Self = Self::I64IsNonZero;

  #[inline(always)]
  pub fn is_valid(x: u8) -> bool {
    x <= Self::MAX_VALUE.encode()
  }

  #[inline(always)]
  pub fn encode(self) -> u8 {
    self as u8
  }

  #[inline(always)]
  pub fn decode(x: u8) -> Self {
    assert!(Self::is_valid(x));
    unsafe { transmute::<u8, Self>(x) }
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
  pub fn decode(x: u8) -> Self {
    assert!(Self::is_valid(x));
    unsafe { transmute::<u8, Self>(x) }
  }
}

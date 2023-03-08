use crate::prelude::*;
use core::mem::transmute;

#[repr(transparent)]
pub struct Prog([u8]);

#[repr(transparent)]
pub struct FunList([u8]);

pub struct FunIter<'a>(ReadBuf<'a, u8>);

#[repr(transparent)]
pub struct Fun([u8]);

#[repr(transparent)]
pub struct FunType([u8]);

#[repr(transparent)]
pub struct ValTypeList([u8]);

pub struct ValTypeIter<'a>(ReadBuf<'a, u8>);

#[repr(transparent)]
pub struct VarIdList([u8]);

pub struct VarIdIter<'a>(ReadBuf<'a, u8>);

#[repr(transparent)]
pub struct InstList([u8]);

pub struct InstIter<'a>(ReadBuf<'a, u8>);

#[repr(transparent)]
pub struct If100([u8; 7]);

#[repr(transparent)]
pub struct If200([u8; 9]);

#[repr(transparent)]
pub struct Jump([u8]);

#[repr(transparent)]
pub struct Op11([u8; 3]);

#[repr(transparent)]
pub struct Op21([u8; 5]);

#[repr(transparent)]
pub struct Op22([u8; 5]);

#[repr(transparent)]
pub struct Op31([u8; 7]);

#[repr(transparent)]
pub struct Return([u8]);

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(&'a ValTypeList),
  ConstBool(bool),
  ConstI6(u6),
  ConstI64(u64),
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
pub struct VarId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy)]
pub enum Imm {
  Bool(bool),
  I6(u6),
  I64(u64),
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(VariantCount)]
#[repr(u8)]
pub enum ValType {
  Bool,
  I6,
  I64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(VariantCount)]
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

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum TagIf100 {
  BoolIsTrue,
  I64IsNonZero,
}

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
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

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
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

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
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

impl Prog {
  pub fn fun_list(&self) -> &FunList {
    let x = &self.0 as *const [u8] as *const FunList;
    unsafe { &*x }
  }
}

impl FunList {
  pub fn iter(&self) -> FunIter<'_> {
    FunIter(ReadBuf::new(&self.0))
  }
}

impl<'a> Iterator for FunIter<'a> {
  type Item = &'a Fun;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    let n = self.0.pop_u32();
    let x = self.0.pop_slice(n as usize) as *const [u8] as *const Fun;

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
    let a = &self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    unsafe { core::str::from_utf8_unchecked(x) }
  }

  pub fn fun_type(&self) -> &FunType {
    let a = &self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j] as *const [u8] as *const FunType;
    unsafe { &*x }
  }

  pub fn inst_list(&self) -> &InstList {
    let a = &self.0;
    let i = self.ofs2();
    let j = self.ofs3();
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
    let a = &self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j] as *const [u8] as *const ValTypeList;
    unsafe { &*x }
  }

  pub fn output(&self) -> &ValTypeList {
    let a = &self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j] as *const [u8] as *const ValTypeList;
    unsafe { &*x }
  }
}

impl ValTypeList {
  pub fn iter(&self) -> ValTypeIter<'_> {
    ValTypeIter(ReadBuf::new(&self.0))
  }
}

impl<'a> Iterator for ValTypeIter<'a> {
  type Item = ValType;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    Some(ValType::decode(self.0.pop_u8()).unwrap())
  }
}

impl VarIdList {
  pub fn iter(&self) -> VarIdIter<'_> {
    VarIdIter(ReadBuf::new(&self.0))
  }
}

impl<'a> Iterator for VarIdIter<'a> {
  type Item = VarId;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    Some(VarId(self.0.pop_u16()))
  }
}

impl InstList {
  pub fn iter(&self) -> InstIter<'_> {
    InstIter(ReadBuf::new(&self.0))
  }
}

impl<'a> Iterator for InstIter<'a> {
  type Item = Inst<'a>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    match TagInst::decode(self.0.pop_u8()).unwrap() {
      TagInst::Block => {
        let n = self.0.pop_u16() as usize;
        let x = self.0.pop_slice(n) as *const [u8] as *const ValTypeList;

        Some(Inst::Block(unsafe { &*x }))
      }
      TagInst::If100 => {
        let x = self.0.pop_array() as *const [u8; 6] as *const If100;

        Some(Inst::If100(unsafe { &*x }))
      }
      /*
      TagInst::If200 => {
        // tag_if100 u8
        // var_id    u16
        // var_id    u16
        // block_id  u16
        // block_id  u16

        let i = 1;
        let j = 10;
        let x = &a[i .. j] as *const [u8] as *const If200;
        let y = &a[j ..];

        (Inst::If200(unsafe { &*x }), y)
      }
      TagInst::Jump => {
        // count     u16
        // block_id  u16
        // var_id    u16[]

        let n = a.get_u16(1) as usize;
        let i = 3;
        let j = 3 + 2 + 2 * n;
        let x = &a[i .. j] as *const [u8] as *const Jump;
        let y = &a[j ..];

        (Inst::Jump(unsafe { &*x }), y)
      }
      */
      _ => {
        unimplemented!()
      }
    }
  }
}

impl If100 {
  const OFS0: usize = 0;
  const OFS1: usize = 1;
  const OFS2: usize = 3;
  const OFS3: usize = 5;

  #[inline(always)]
  pub fn tag(&self) -> TagIf100 { TagIf100::decode(self.0[Self::OFS0]).unwrap() }

  #[inline(always)]
  pub fn var_id(&self) -> VarId { VarId(self.0.get_u16(Self::OFS1)) }

  #[inline(always)]
  pub fn block_id_0(&self) -> BlockId { BlockId(self.0.get_u16(Self::OFS2)) }

  #[inline(always)]
  pub fn block_id_1(&self) -> BlockId { BlockId(self.0.get_u16(Self::OFS3)) }
}

impl If200 {
  const OFS0: usize = 0;
  const OFS1: usize = 1;
  const OFS2: usize = 3;
  const OFS3: usize = 5;
  const OFS4: usize = 7;

  #[inline(always)]
  pub fn tag(&self) -> TagIf200 { TagIf200::decode(self.0[Self::OFS0]).unwrap() }

  #[inline(always)]
  pub fn var_id_0(&self) -> VarId { VarId(self.0.get_u16(Self::OFS1)) }

  #[inline(always)]
  pub fn var_id_1(&self) -> VarId { VarId(self.0.get_u16(Self::OFS2)) }

  #[inline(always)]
  pub fn block_id_0(&self) -> BlockId { BlockId(self.0.get_u16(Self::OFS3)) }

  #[inline(always)]
  pub fn block_id_1(&self) -> BlockId { BlockId(self.0.get_u16(Self::OFS4)) }
}

impl ValType {
  #[inline(always)]
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagInst {
  #[inline(always)]
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagIf100 {
  #[inline(always)]
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagIf200 {
  #[inline(always)]
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

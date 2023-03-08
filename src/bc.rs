use crate::prelude::*;
use core::mem::transmute;

#[derive(Clone, Copy)]
pub struct Prog<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct FunList<'a>(&'a [u8]);

pub struct FunIter<'a>(ReadBuf<'a, u8>);

#[derive(Clone, Copy)]
pub struct Fun<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct FunType<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct ValTypeList<'a>(&'a [u8]);

pub struct ValTypeIter<'a>(ReadBuf<'a, u8>);

#[derive(Clone, Copy)]
pub struct VarIdList<'a>(&'a [u8]);

pub struct VarIdIter<'a>(ReadBuf<'a, u8>);

#[derive(Clone, Copy)]
pub struct InstList<'a>(&'a [u8]);

pub struct InstIter<'a>(ReadBuf<'a, u8>);

#[derive(Clone, Copy)]
pub struct If100<'a>(&'a [u8; 7]);

#[derive(Clone, Copy)]
pub struct If200<'a>(&'a [u8; 9]);

#[derive(Clone, Copy)]
pub struct Jump<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct Op11<'a>(&'a [u8; 3]);

#[derive(Clone, Copy)]
pub struct Op21<'a>(&'a [u8; 5]);

#[derive(Clone, Copy)]
pub struct Op22<'a>(&'a [u8; 5]);

#[derive(Clone, Copy)]
pub struct Op31<'a>(&'a [u8; 7]);

#[derive(Clone, Copy)]
pub struct Return<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(ValTypeList<'a>),
  ConstBool(bool),
  ConstI6(u6),
  ConstI64(u64),
  If100(If100<'a>),
  If200(If200<'a>),
  Jump(Jump<'a>),
  Op11(Op11<'a>),
  Op21(Op21<'a>),
  Op22(Op22<'a>),
  Op31(Op31<'a>),
  Return(Return<'a>),
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

impl<'a> Prog<'a> {
  pub fn fun_list(self) -> FunList<'a> {
    FunList(self.0)
  }
}

impl<'a> FunList<'a> {
  pub fn iter(self) -> FunIter<'a> {
    FunIter(ReadBuf::new(self.0))
  }
}

impl<'a> Iterator for FunIter<'a> {
  type Item = Fun<'a>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    let n = self.0.pop_u32();
    let x = self.0.pop_slice(n as usize);

    Some(Fun(x))
  }
}

impl<'a> Fun<'a> {
  // ofs1
  // ofs2
  // name      | [ofs0, ofs1)
  // fun_type  | [ofs1, ofs2)
  // inst_list | [ofs2, ofs3)
  //
  // ofs0 = 8
  // ofs3 = len

  #[inline(always)]
  fn ofs0(self) -> usize { let _ = self; 8 }

  #[inline(always)]
  fn ofs1(self) -> usize { self.0.get_u32(0) as usize }

  #[inline(always)]
  fn ofs2(self) -> usize { self.0.get_u32(4) as usize }

  #[inline(always)]
  fn ofs3(self) -> usize { self.0.len() }

  pub fn name(self) -> &'a str {
    let a = self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    unsafe { core::str::from_utf8_unchecked(x) }
  }

  pub fn fun_type(self) -> FunType<'a> {
    let a = self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j];
    FunType(x)
  }

  pub fn inst_list(self) -> InstList<'a> {
    let a = self.0;
    let i = self.ofs2();
    let j = self.ofs3();
    let x = &a[i .. j];
    InstList(x)
  }
}

impl<'a> FunType<'a> {
  // ofs1
  // val_type_list | [ofs0, ofs1)
  // val_type_list | [ofs1, ofs2)
  //
  // ofs0 = 4
  // ofs2 = len

  #[inline(always)]
  fn ofs0(self) -> usize { let _ = self; 4 }

  #[inline(always)]
  fn ofs1(self) -> usize { self.0.get_u32(0) as usize }

  #[inline(always)]
  fn ofs2(self) -> usize { self.0.len() }

  pub fn input(self) -> ValTypeList<'a> {
    let a = self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    ValTypeList(x)
  }

  pub fn output(self) -> ValTypeList<'a> {
    let a = self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j];
    ValTypeList(x)
  }
}

impl<'a> ValTypeList<'a> {
  pub fn iter(self) -> ValTypeIter<'a> {
    ValTypeIter(ReadBuf::new(self.0))
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

impl<'a> VarIdList<'a> {
  pub fn iter(self) -> VarIdIter<'a> {
    VarIdIter(ReadBuf::new(self.0))
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

impl<'a> InstList<'a> {
  pub fn iter(&self) -> InstIter<'a> {
    InstIter(ReadBuf::new(self.0))
  }
}

impl<'a> Iterator for InstIter<'a> {
  type Item = Inst<'a>;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    let t = self.0.pop_u8();

    match TagInst::decode(t).unwrap() {
      TagInst::Block => {
        let n = self.0.pop_u16() as usize;

        Some(Inst::Block(ValTypeList(self.0.pop_slice(n))))
      }
      TagInst::If100 => {
        Some(Inst::If100(If100(self.0.pop_array())))
      }
      TagInst::If200 => {
        Some(Inst::If200(If200(self.0.pop_array())))
      }
      TagInst::Jump => {
        let n = self.0.pop_u16() as usize;
        let n = 2 + 2 * n;

        Some(Inst::Jump(Jump(self.0.pop_slice(n))))
      }
      _ => {
        unimplemented!()
      }
    }
  }
}
/*

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
*/

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

use crate::prelude::*;
use core::mem::transmute;

#[derive(Clone, Copy)]
pub struct Prog<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct FunList<'a>(&'a [u8]);

pub struct FunIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct Fun<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct FunType<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct ValTypeList<'a>(&'a [u8]);

pub struct ValTypeIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct VarIdList<'a>(&'a [u8]);

pub struct VarIdIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct InstList<'a>(&'a [u8]);

pub struct InstIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct Block<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct ConstBool<'a>(&'a [u8; 1]);

#[derive(Clone, Copy)]
pub struct ConstI6<'a>(&'a [u8; 1]);

#[derive(Clone, Copy)]
pub struct ConstI64<'a>(&'a [u8; 8]);

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
  Block(Block<'a>),
  ConstBool(ConstBool<'a>),
  ConstI6(ConstI6<'a>),
  ConstI64(ConstI64<'a>),
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

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum ValType {
  Bool,
  I6,
  I64,
}

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum TagInst {
  Block,
  ConstBool,
  ConstI6,
  ConstI64,
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

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum TagOp22 {
  I64MulFullS,
  I64MulFullU,
}

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
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
    FunIter(ByteCursor::new(self.0))
  }
}

impl<'a> Iterator for FunIter<'a> {
  type Item = Fun<'a>;

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

  fn ofs0(self) -> usize { let _ = self; 8 }
  fn ofs1(self) -> usize { self.0.get_u32(0) as usize }
  fn ofs2(self) -> usize { self.0.get_u32(4) as usize }
  fn ofs3(self) -> usize { self.0.len() }

  pub fn name(self) -> &'a str {
    let a = self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    core::str::from_utf8(x).unwrap()
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

  fn ofs0(self) -> usize { let _ = self; 4 }
  fn ofs1(self) -> usize { self.0.get_u32(0) as usize }
  fn ofs2(self) -> usize { self.0.len() }

  pub fn inputs(self) -> ValTypeList<'a> {
    let a = self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    ValTypeList(x)
  }

  pub fn outputs(self) -> ValTypeList<'a> {
    let a = self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j];
    ValTypeList(x)
  }
}

impl<'a> ValTypeList<'a> {
  pub fn iter(self) -> ValTypeIter<'a> {
    ValTypeIter(ByteCursor::new(self.0))
  }
}

impl<'a> Iterator for ValTypeIter<'a> {
  type Item = ValType;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }
    Some(ValType::decode(self.0.pop_u8()).unwrap())
  }
}

impl<'a> VarIdList<'a> {
  pub fn iter(self) -> VarIdIter<'a> {
    VarIdIter(ByteCursor::new(self.0))
  }
}

impl<'a> Iterator for VarIdIter<'a> {
  type Item = VarId;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }
    Some(VarId(self.0.pop_u16()))
  }
}

impl<'a> InstList<'a> {
  pub fn iter(&self) -> InstIter<'a> {
    InstIter(ByteCursor::new(self.0))
  }
}

impl<'a> Iterator for InstIter<'a> {
  type Item = Inst<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    let t = self.0.pop_u8();

    match TagInst::decode(t).unwrap() {
      TagInst::Block => {
        let n = self.0.pop_u16() as usize;
        Some(Inst::Block(Block(self.0.pop_slice(n))))
      }
      TagInst::ConstBool =>
        Some(Inst::ConstBool(ConstBool(self.0.pop_array()))),
      TagInst::ConstI6 =>
        Some(Inst::ConstI6(ConstI6(self.0.pop_array()))),
      TagInst::ConstI64 =>
        Some(Inst::ConstI64(ConstI64(self.0.pop_array()))),
      TagInst::If100 =>
        Some(Inst::If100(If100(self.0.pop_array()))),
      TagInst::If200 =>
        Some(Inst::If200(If200(self.0.pop_array()))),
      TagInst::Jump => {
        let n = self.0.pop_u16() as usize;
        let n = 2 + 2 * n;
        Some(Inst::Jump(Jump(self.0.pop_slice(n))))
      }
      TagInst::Op11 =>
        Some(Inst::Op11(Op11(self.0.pop_array()))),
      TagInst::Op21 =>
        Some(Inst::Op21(Op21(self.0.pop_array()))),
      TagInst::Op22 =>
        Some(Inst::Op22(Op22(self.0.pop_array()))),
      TagInst::Op31 =>
        Some(Inst::Op31(Op31(self.0.pop_array()))),
      TagInst::Return => {
        let n = self.0.pop_u16() as usize;
        let n = 2 * n;
        Some(Inst::Return(Return(self.0.pop_slice(n))))
      }
    }
  }
}

impl<'a> Block<'a> {
  pub fn params(self) -> ValTypeList<'a> {
    ValTypeList(self.0)
  }
}

impl<'a> ConstBool<'a> {
  pub fn imm(self) -> bool {
    u8::from_le_bytes(*self.0) != 0
  }
}

impl<'a> ConstI6<'a> {
  pub fn imm(self) -> u6 {
    u6::from(u8::from_le_bytes(*self.0))
  }
}

impl<'a> ConstI64<'a> {
  pub fn imm(self) -> u64 {
    u64::from_le_bytes(*self.0)
  }
}

impl<'a> If100<'a> {
  pub fn get(self) -> (TagIf100, VarId, BlockId, BlockId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);
    let a = self.0.get_u16(3);
    let b = self.0.get_u16(5);

    (TagIf100::decode(t).unwrap(), VarId(x), BlockId(a), BlockId(b))
  }
}

impl<'a> If200<'a> {
  pub fn get(self) -> (TagIf200, VarId, VarId, BlockId, BlockId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);
    let y = self.0.get_u16(3);
    let a = self.0.get_u16(5);
    let b = self.0.get_u16(7);

    (TagIf200::decode(t).unwrap(), VarId(x), VarId(y), BlockId(a), BlockId(b))
  }
}

impl<'a> Jump<'a> {
  pub fn target(self) -> BlockId {
    let x = self.0.get_u16(0);
    BlockId(x)
  }

  pub fn args(self) -> VarIdList<'a> {
    let x = self.0.get(2 ..).unwrap();
    VarIdList(x)
  }
}

impl<'a> Op11<'a> {
  pub fn get(self) -> (TagOp11, VarId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);

    (TagOp11::decode(t).unwrap(), VarId(x))
  }
}

impl<'a> Op21<'a> {
  pub fn get(self) -> (TagOp21, VarId, VarId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);
    let y = self.0.get_u16(3);

    (TagOp21::decode(t).unwrap(), VarId(x), VarId(y))
  }
}

impl<'a> Op22<'a> {
  pub fn get(self) -> (TagOp22, VarId, VarId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);
    let y = self.0.get_u16(3);

    (TagOp22::decode(t).unwrap(), VarId(x), VarId(y))
  }
}

impl<'a> Op31<'a> {
  pub fn get(self) -> (TagOp31, VarId, VarId, VarId) {
    let t = self.0.get_u8(0);
    let x = self.0.get_u16(1);
    let y = self.0.get_u16(3);
    let z = self.0.get_u16(5);

    (TagOp31::decode(t).unwrap(), VarId(x), VarId(y), VarId(z))
  }
}

impl<'a> Return<'a> {
  pub fn args(self) -> VarIdList<'a> {
    VarIdList(self.0)
  }
}

impl ValType {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagInst {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagIf100 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagIf200 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagOp11 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagOp21 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagOp22 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl TagOp31 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

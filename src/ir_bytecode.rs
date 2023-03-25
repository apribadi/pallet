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
pub struct FunTy<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct TyList<'a>(&'a [u8]);

pub struct TyIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct VarIdList<'a>(&'a [u8]);

pub struct VarIdIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct InstList<'a>(&'a [u8]);

pub struct InstIter<'a>(ByteCursor<'a>);

#[derive(Clone, Copy)]
pub struct InstBlock<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct InstGoto<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct InstIf<'a>(&'a [u8; 6]);

#[derive(Clone, Copy)]
pub struct InstImmBool<'a>(&'a [u8; 1]);

#[derive(Clone, Copy)]
pub struct InstImmI6<'a>(&'a [u8; 1]);

#[derive(Clone, Copy)]
pub struct InstImmI64<'a>(&'a [u8; 8]);

#[derive(Clone, Copy)]
pub struct InstOp11<'a>(&'a [u8; 3]);

#[derive(Clone, Copy)]
pub struct InstOp21<'a>(&'a [u8; 5]);

#[derive(Clone, Copy)]
pub struct InstOp31<'a>(&'a [u8; 7]);

#[derive(Clone, Copy)]
pub struct InstRet<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub enum Inst<'a> {
  Block(InstBlock<'a>),
  Goto(InstGoto<'a>),
  If(InstIf<'a>),
  ImmBool(InstImmBool<'a>),
  ImmI6(InstImmI6<'a>),
  ImmI64(InstImmI64<'a>),
  Op11(InstOp11<'a>),
  Op21(InstOp21<'a>),
  Op31(InstOp31<'a>),
  Ret(InstRet<'a>),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct VarId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BlockId(pub u16);

#[derive(Clone, Copy, Eq, PartialEq, VariantCount)]
#[repr(u8)]
pub enum InstTag {
  Block,
  Goto,
  If,
  ImmBool,
  ImmI6,
  ImmI64,
  Op11,
  Op21,
  Op31,
  Ret,
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

  pub fn fun_type(self) -> FunTy<'a> {
    let a = self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j];
    FunTy(x)
  }

  pub fn inst_list(self) -> InstList<'a> {
    let a = self.0;
    let i = self.ofs2();
    let j = self.ofs3();
    let x = &a[i .. j];
    InstList(x)
  }
}

impl<'a> FunTy<'a> {
  // ofs1
  // val_type_list | [ofs0, ofs1)
  // val_type_list | [ofs1, ofs2)
  //
  // ofs0 = 4
  // ofs2 = len

  fn ofs0(self) -> usize { let _ = self; 4 }
  fn ofs1(self) -> usize { self.0.get_u32(0) as usize }
  fn ofs2(self) -> usize { self.0.len() }

  pub fn inputs(self) -> TyList<'a> {
    let a = self.0;
    let i = self.ofs0();
    let j = self.ofs1();
    let x = &a[i .. j];
    TyList(x)
  }

  pub fn outputs(self) -> TyList<'a> {
    let a = self.0;
    let i = self.ofs1();
    let j = self.ofs2();
    let x = &a[i .. j];
    TyList(x)
  }
}

impl<'a> TyList<'a> {
  pub fn iter(self) -> TyIter<'a> {
    TyIter(ByteCursor::new(self.0))
  }
}

impl<'a> Iterator for TyIter<'a> {
  type Item = Ty;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }
    Some(Ty::decode(self.0.pop_u8()).unwrap())
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

    match InstTag::decode(t).unwrap() {
      InstTag::Block => {
        let n = self.0.pop_u16() as usize;
        Some(Inst::Block(InstBlock(self.0.pop_slice(n))))
      }
      InstTag::ImmBool =>
        Some(Inst::ImmBool(InstImmBool(self.0.pop_array()))),
      InstTag::ImmI6 =>
        Some(Inst::ImmI6(InstImmI6(self.0.pop_array()))),
      InstTag::ImmI64 =>
        Some(Inst::ImmI64(InstImmI64(self.0.pop_array()))),
      InstTag::Goto => {
        let n = self.0.pop_u16() as usize;
        let n = 2 + 2 * n;
        Some(Inst::Goto(InstGoto(self.0.pop_slice(n))))
      }
      InstTag::If=>
        Some(Inst::If(InstIf(self.0.pop_array()))),
      InstTag::Op11 =>
        Some(Inst::Op11(InstOp11(self.0.pop_array()))),
      InstTag::Op21 =>
        Some(Inst::Op21(InstOp21(self.0.pop_array()))),
      InstTag::Op31 =>
        Some(Inst::Op31(InstOp31(self.0.pop_array()))),
      InstTag::Ret => {
        let n = self.0.pop_u16() as usize;
        let n = 2 * n;
        Some(Inst::Ret(InstRet(self.0.pop_slice(n))))
      }
    }
  }
}

impl<'a> InstBlock<'a> {
  pub fn params(self) -> TyList<'a> {
    TyList(self.0)
  }
}

impl<'a> InstGoto<'a> {
  pub fn target(self) -> BlockId {
    let x = self.0.get_u16(0);
    BlockId(x)
  }

  pub fn args(self) -> VarIdList<'a> {
    let x = self.0.get(2 ..).unwrap();
    VarIdList(x)
  }
}

impl<'a> InstIf<'a> {
  pub fn vars(self) -> [VarId; 1] {
    [ VarId(self.0.get_u16(0)) ]
  }

  pub fn dsts(self) -> [BlockId; 2] {
    [ BlockId(self.0.get_u16(2)),
      BlockId(self.0.get_u16(4)),
    ]
  }
}

impl<'a> InstImmBool<'a> {
  pub fn imm(self) -> bool {
    u8::from_le_bytes(*self.0) != 0
  }
}

impl<'a> InstImmI6<'a> {
  pub fn imm(self) -> u6 {
    u6::from(u8::from_le_bytes(*self.0))
  }
}

impl<'a> InstImmI64<'a> {
  pub fn imm(self) -> u64 {
    u64::from_le_bytes(*self.0)
  }
}

impl<'a> InstOp11<'a> {
  pub fn op(self) -> Op11 {
    Op11::decode(self.0.get_u8(0)).unwrap()
  }

  pub fn vars(self) -> [VarId; 1] {
    [ VarId(self.0.get_u16(1)) ]
  }
}

impl<'a> InstOp21<'a> {
  pub fn op(self) -> Op21 {
    Op21::decode(self.0.get_u8(0)).unwrap()
  }

  pub fn vars(self) -> [VarId; 2] {
    [ VarId(self.0.get_u16(1)),
      VarId(self.0.get_u16(3)),
    ]
  }
}

impl<'a> InstOp31<'a> {
  pub fn op(self) -> Op31 {
    Op31::decode(self.0.get_u8(0)).unwrap()
  }

  pub fn vars(self) -> [VarId; 3] {
    [ VarId(self.0.get_u16(1)),
      VarId(self.0.get_u16(3)),
      VarId(self.0.get_u16(5)),
    ]
  }
}

impl<'a> InstRet<'a> {
  pub fn args(self) -> VarIdList<'a> {
    VarIdList(self.0)
  }
}

impl Ty {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl InstTag {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl Op11 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl Op21 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

impl Op31 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { transmute::<u8, Self>(t) })
  }
}

use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Prog<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct FunList<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct Fun<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct TyList<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct IdList<'a>(&'a [u8]);

#[derive(Clone, Copy)]
pub struct InstList<'a>(&'a [u8]);

pub struct InstIter<'a>(&'a [u8]);

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
pub struct InstReturn<'a>(&'a [u8]);

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
  Return(InstReturn<'a>),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Id(pub u16);

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
  Return,
}

impl<'a> Prog<'a> {
  pub fn fun_list(self) -> FunList<'a> {
    FunList(bc::field::<1, 0>(self.0))
  }
}

impl<'a> FunList<'a> {
  pub fn iter(self) -> impl Iterator<Item = Fun<'a>> {
    bc::iter_variable_sized(self.0).map(|x| Fun(x))
  }
}

impl<'a> Fun<'a> {
  pub fn name(self) -> &'a str {
    core::str::from_utf8(bc::field::<4, 0>(self.0)).unwrap()
  }

  pub fn input_ty_list(self) -> TyList<'a> {
    TyList(bc::field::<4, 1>(self.0))
  }

  pub fn output_ty_list(self) -> TyList<'a> {
    TyList(bc::field::<4, 2>(self.0))
  }

  pub fn inst_list(self) -> InstList<'a> {
    InstList(bc::field::<4, 3>(self.0))
  }
}

impl<'a> TyList<'a> {
  pub fn iter(self) -> impl 'a + Iterator<Item = Ty> {
    bc::iter_constant_sized(self.0).map(|x| Ty::decode(u8::from_le_bytes(*x)).unwrap())
  }
}

impl<'a> IdList<'a> {
  pub fn iter(self) -> impl 'a + Iterator<Item = Id> {
    bc::iter_constant_sized(self.0).map(|x| Id(u16::from_le_bytes(*x)))
  }
}

impl<'a> InstList<'a> {
  pub fn iter(&self) -> InstIter<'a> {
    InstIter(self.0)
  }
}

impl<'a> Iterator for InstIter<'a> {
  type Item = Inst<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() { return None; }

    let t = InstTag::decode(self.0.pop_u8()).unwrap();

    match t {
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
      InstTag::Return => {
        let n = self.0.pop_u16() as usize;
        let n = 2 * n;
        Some(Inst::Return(InstReturn(self.0.pop_slice(n))))
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

  pub fn args(self) -> IdList<'a> {
    let x = self.0.get(2 ..).unwrap();
    IdList(x)
  }
}

impl<'a> InstIf<'a> {
  pub fn args(self) -> [Id; 1] {
    [ Id(self.0.get_u16(0)) ]
  }

  pub fn targets(self) -> [BlockId; 2] {
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

  pub fn args(self) -> [Id; 1] {
    [ Id(self.0.get_u16(1)) ]
  }
}

impl<'a> InstOp21<'a> {
  pub fn op(self) -> Op21 {
    Op21::decode(self.0.get_u8(0)).unwrap()
  }

  pub fn args(self) -> [Id; 2] {
    [ Id(self.0.get_u16(1)),
      Id(self.0.get_u16(3)),
    ]
  }
}

impl<'a> InstReturn<'a> {
  pub fn args(self) -> IdList<'a> {
    IdList(self.0)
  }
}

impl Ty {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl InstTag {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl Op11 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl Op21 {
  pub fn decode(t: u8) -> Option<Self> {
    if (t as usize) >= Self::VARIANT_COUNT { return None; }
    Some(unsafe { core::mem::transmute::<u8, Self>(t) })
  }
}

impl fmt::Display for BlockId {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(out, "{}", self.0)
  }
}

impl fmt::Display for Id {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(out, "{}", self.0)
  }
}

impl<'a> fmt::Display for Prog<'a> {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    for f in self.fun_list().iter() {
      let mut i = 0; // varid
      let mut j = 0; // blockid
      write!(out, "function {} (", f.name())?;
      for ty in f.input_ty_list().iter() {
        write!(out, " %{}:{}", i, ty)?;
        i += 1;
      }
      write!(out, " ) -> (")?;
      for ty in f.output_ty_list().iter() {
        write!(out, " {}", ty)?;
      }
      write!(out, " ):\n")?;

      /*
      for inst in f.inst_list().iter() {
        match inst {
          Inst::Block(x) => {
            write!(out, "block @{}", j)?;
            j += 1;
            for ty in x.params().iter() {
              write!(out, " %{}:{}", i, ty)?;
              i += 1;
            }
            write!(out, ":\n")?;
          }
          Inst::Goto(x) => {
            write!(out, "  goto @{}", x.target())?;
            for arg in x.args().iter() {
              write!(out, " %{}", arg)?;
            }
            write!(out, "\n")?;
          }
          Inst::If(x) => {
            write!(out, "  if {} {} {}\n", x.args()[0], x.targets()[0], x.targets()[1])?;
          }
          Inst::ImmBool(x) => {
            write!(out, "  %{} = imm bool {}\n", i, x.imm())?;
            i += 1;
          }
          Inst::ImmI6(x) => {
            write!(out, "  %{} = imm i6 {}\n", i, u8::from(x.imm()))?;
            i += 1;
          }
          Inst::ImmI64(x) => {
            write!(out, "  %{} = imm i64 {}\n", i, x.imm())?;
            i += 1;
          }
          Inst::Op11(x) => {
            write!(out, "  %{} = {} {}\n", i, x.op(), x.args()[0])?;
          }
          Inst::Op21(x) => {
            write!(out, "  %{} = {} {} {}\n", i, x.op(), x.args()[0], x.args()[1])?;
          }
          Inst::Return(x) => {
            write!(out, "  return")?;
            for arg in x.args().iter() {
              write!(out, " %{}", arg)?;
            }
            write!(out, "\n")?;
          }
        }
      }
      */
      write!(out, "\n")?;
    }

    write!(out, "\n")?;

    Ok(())
  }
}

/*

pub struct BcBuilder {
  buf: ByteBuf,
  fun_size_backpatch: Option<usize>,
  fun_start: Option<usize>,
  fun_ofs1_backpatch: Option<usize>,
  fun_ofs2_backpatch: Option<usize>,
}

impl BcBuilder {
  pub fn new() -> Self {
    Self {
      buf: ByteBuf::new(),
      fun_start: None,
      fun_size_backpatch: None,
      fun_ofs1_backpatch: None,
      fun_ofs2_backpatch: None,
    }
  }

  pub fn emit_fun_start(&mut self) {
    let i = self.buf.len();
    self.fun_size_backpatch = Some(i);
    self.buf.put_u32(0);
    let i = self.buf.len();
    self.fun_start = Some(i);
    self.fun_ofs1_backpatch = Some(i);
    self.buf.put_u32(0);
    let i = self.buf.len();
    self.fun_ofs2_backpatch = Some(i);
    self.buf.put_u32(0);
  }

  pub fn emit_fun_stop(&mut self) {
    let here = self.buf.len();
    let start = self.fun_start.unwrap();
    let size = u32::try_from(here - start).unwrap();
    let backpatch = self.fun_size_backpatch.unwrap();
    self.buf.set_u32(backpatch, size);
    self.fun_size_backpatch = None;
    self.fun_start = None;
  }
}
*/

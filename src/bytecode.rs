#[repr(u8)]
enum ValType {
  Bool,
  FunRef,
  I64,
  Ref,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[repr(transparent)]
struct VarId(u16);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[repr(transparent)]
struct BlockId(u16);

#[repr(transparent)]
struct Fun<'a>(&'a [Inst<'a>]);

#[repr(u8)]
enum TagOp11 {
  BoolNot,
  I64Clz,
  I64Ctz,
  I64IsZero,
  I64Neg,
  I64Not,
  I64Popcnt,
  I64Swap,
}

#[repr(u8)]
enum TagOp21 {
  BoolAnd,
  BoolEq,
  BoolNe,
  BoolOr,
  I64Add,
  I64And,
  I64Asr,
  I64IsEq,
  I64IsGeS,
  I64IsGeU,
  I64IsGtS,
  I64IsGtU,
  I64IsLeS,
  I64IsLeU,
  I64IsLsr,
  I64IsLtS,
  I64IsLtU,
  I64IsNe,
  I64MaxS,
  I64MaxU,
  I64MinS,
  I64MinU,
  I64Mul,
  I64MulHi,
  I64Or,
  I64Rol,
  I64Ror,
  I64Shl,
  I64Sub,
  I64Xor,
}

#[repr(u8)]
enum TagOp22 {
  I64MulFull
}

#[repr(u8)]
enum TagOp31 {
  I64Sel,
}

#[repr(u8)]
enum TagIf1 {
  I64IfZero,
  If,
}

#[repr(u8)]
enum TagIf2 {
  I64IfEq,
  I64IfGeS,
  I64IfGeU,
  I64IfGtS,
  I64IfGtU,
  I64IfLeS,
  I64IfLeU,
  I64IfLtS,
  I64IfLtU,
  I64IfNe,
}

enum Const {
  ConstBool(bool),
  ConstI64(u64),
}

enum Inst<'a> {
  Block(&'a [ValType]),
  Entry(&'a [ValType]),
  FunCall,
  FunCallIndirect,
  FunTailCall,
  FunTailCallIndirect,
  If1(TagIf1, VarId, BlockId, &'a [VarId], BlockId, &'a [VarId]),
  If2(TagIf2, VarId, VarId, BlockId, &'a [VarId], BlockId, &'a [VarId]),
  Jump(BlockId, &'a [VarId]),
  Op01(Const),
  Op11(TagOp11, VarId),
  Op21(TagOp21, VarId, VarId),
  Op22(TagOp22, VarId, VarId),
  Op31(TagOp21, VarId, VarId, VarId),
  Return(&'a [VarId]),
}

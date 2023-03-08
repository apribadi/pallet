pub(crate) use crate::bytecode;
pub(crate) use std::fs::File;
pub(crate) use std::io::Write;
pub(crate) use std::sync::Arc;
pub(crate) use target_lexicon;
pub(crate) use variant_count::VariantCount;

pub(crate) mod cranelift {
  // use cranelift_codegen::ir::Signature;
  // use cranelift_module::FuncId;
  // pub(crate) use cranelift_codegen::ir::Signature;
  // pub(crate) use cranelift_codegen::ir::Value;
  // pub(crate) use cranelift_codegen::ir::Inst;
  pub(crate) use cranelift_codegen as codegen;
  pub(crate) use cranelift_codegen::Context;
  pub(crate) use cranelift_codegen::ir::AbiParam;
  pub(crate) use cranelift_codegen::ir::InstBuilder;
  pub(crate) use cranelift_codegen::ir::Type;
  pub(crate) use cranelift_codegen::ir::condcodes::IntCC;
  pub(crate) use cranelift_codegen::ir::types::I64;
  pub(crate) use cranelift_codegen::ir::types::I8;
  pub(crate) use cranelift_codegen::isa::CallConv;
  pub(crate) use cranelift_codegen::isa::aarch64::AArch64Backend;
  pub(crate) use cranelift_codegen::settings::Configurable;
  pub(crate) use cranelift_frontend::FunctionBuilder;
  pub(crate) use cranelift_frontend::FunctionBuilderContext;
  pub(crate) use cranelift_module as module;
  pub(crate) use cranelift_module::Linkage;
  pub(crate) use cranelift_module::Module;
  pub(crate) use cranelift_module::ModuleCompiledFunction;
  pub(crate) use cranelift_object::ObjectBuilder;
  pub(crate) use cranelift_object::ObjectModule;
}

pub(crate) use cranelift::Configurable;
pub(crate) use cranelift::InstBuilder;
pub(crate) use cranelift::Module;

pub(crate) fn map_slice<F, T, U>(src: &[T], f: F) -> Box<[U]>
where
  F: FnMut(&T) -> U
{
  let mut f = f;
  let mut dst = Vec::new();

  for x in src.iter() {
    dst.push(f(x));
  }

  dst.into_boxed_slice()
}

#[inline(always)]
pub(crate) const fn max(x: usize, y: usize) -> usize {
  if x >= y { x } else { y }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct u6(u8);

impl From<u8> for u6 {
  #[inline(always)]
  fn from(x: u8) -> Self { Self(x & 0x3f) }
}

impl From<u6> for u8 {
  #[inline(always)]
  fn from(x: u6) -> Self { x.0 }
}

pub(crate) trait SliceExt<T> {
  fn get_array<const N: usize>(&self, offset: usize) -> &[T; N];
  fn get_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [T; N];
  fn split_array<const N: usize>(&self) -> (&[T; N], &[T]);
}

pub(crate) trait SliceRefExt<T> {
  fn chomp_array<const N: usize>(&mut self) -> &[T; N];
  fn chomp_slice(&mut self, n: usize) -> &[T];
}

pub(crate) trait BytesExt {
  fn get_u16(&self, offset: usize) -> u16;
  fn get_u32(&self, offset: usize) -> u32;
  fn get_u64(&self, offset: usize) -> u64;
  fn set_u16(&mut self, offset: usize, value: u16);
  fn set_u32(&mut self, offset: usize, value: u32);
  fn set_u64(&mut self, offset: usize, value: u64);
}

impl<T> SliceExt<T> for [T] {
  #[inline(always)]
  fn get_array<const N: usize>(&self, offset: usize) -> &[T; N] {
    let len = self.len();
    assert!(offset <= len && N <= len - offset);
    let p = self.as_ptr();
    let p = unsafe { p.add(offset) };
    let p = p as *const [T; N];
    unsafe { &*p }
  }

  #[inline(always)]
  fn get_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [T; N] {
    let len = self.len();
    assert!(offset <= len && N <= len - offset);
    let p = self.as_mut_ptr();
    let p = unsafe { p.add(offset) };
    let p = p as *mut [T; N];
    unsafe { &mut *p }
  }

  #[inline(always)]
  fn split_array<const N: usize>(&self) -> (&[T; N], &[T]) {
    let len = self.len();
    assert!(N <= len);
    let x = self.as_ptr() as *const [T; N];
    let x = unsafe { &*x };
    let y = unsafe { self.get_unchecked(N ..) };
    (x, y)
  }
}

impl BytesExt for [u8] {
  #[inline(always)]
  fn get_u16(&self, offset: usize) -> u16 {
    u16::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn get_u32(&self, offset: usize) -> u32 {
    u32::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn get_u64(&self, offset: usize) -> u64 {
    u64::from_le_bytes(*self.get_array(offset))
  }

  #[inline(always)]
  fn set_u16(&mut self, offset: usize, value: u16) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }

  #[inline(always)]
  fn set_u32(&mut self, offset: usize, value: u32) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }

  #[inline(always)]
  fn set_u64(&mut self, offset: usize, value: u64) {
    *self.get_array_mut(offset) = value.to_le_bytes();
  }
}

pub(crate) struct ReadBuf<'a, T>(&'a [T]);

impl<'a, T> ReadBuf<'a, T> {
  #[inline(always)]
  pub(crate) fn new(x: &'a [T]) -> Self {
    Self(x)
  }

  #[inline(always)]
  pub(crate) fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline(always)]
  pub(crate) fn pop_array<const K: usize>(&mut self) -> &'a [T; K] {
    let (x, y) = self.0.split_array();
    self.0 = y;
    x
  }

  #[inline(always)]
  pub(crate) fn pop_slice(&mut self, k: usize) -> &'a [T] {
    let (x, y) = self.0.split_at(k);
    self.0 = y;
    x
  }
}

impl<'a> ReadBuf<'a, u8> {
  #[inline(always)]
  pub(crate) fn pop_u8(&mut self) -> u8 {
    self.pop_array::<1>()[0]
  }

  #[inline(always)]
  pub(crate) fn pop_u16(&mut self) -> u16 {
    u16::from_le_bytes(*self.pop_array())
  }

  #[inline(always)]
  pub(crate) fn pop_u32(&mut self) -> u32 {
    u32::from_le_bytes(*self.pop_array())
  }
}

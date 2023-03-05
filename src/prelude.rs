pub(crate) use crate::bytecode;
pub(crate) use std::fs::File;
pub(crate) use std::io::Write;
pub(crate) use std::sync::Arc;

pub(crate) use target_lexicon;
pub(crate) use cranelift_codegen;
pub(crate) use cranelift_module;
// use cranelift_frontend;
// use cranelift_object;

pub(crate) mod cranelift {
  // use cranelift_codegen::ir::Signature;
  // use cranelift_module::FuncId;
  pub(crate) use cranelift_codegen::Context;
  pub(crate) use cranelift_codegen::ir::AbiParam;
  pub(crate) use cranelift_codegen::ir::Inst;
  pub(crate) use cranelift_codegen::ir::InstBuilder;
  // pub(crate) use cranelift_codegen::ir::Signature;
  pub(crate) use cranelift_codegen::ir::Type;
  // pub(crate) use cranelift_codegen::ir::Value;
  pub(crate) use cranelift_codegen::ir::types::I64;
  pub(crate) use cranelift_codegen::ir::types::I8;
  pub(crate) use cranelift_codegen::isa::CallConv;
  pub(crate) use cranelift_codegen::isa::aarch64::AArch64Backend;
  pub(crate) use cranelift_codegen::settings::Configurable;
  pub(crate) use cranelift_frontend::FunctionBuilder;
  pub(crate) use cranelift_frontend::FunctionBuilderContext;
  pub(crate) use cranelift_module::Linkage;
  pub(crate) use cranelift_module::Module;
  pub(crate) use cranelift_module::ModuleCompiledFunction;
  pub(crate) use cranelift_object::ObjectBuilder;
  pub(crate) use cranelift_object::ObjectModule;
}

pub(crate) use cranelift::Configurable;
pub(crate) use cranelift::InstBuilder;
pub(crate) use cranelift::Module;

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


pub(crate) use crate::bc;
pub(crate) use crate::buf::*;
pub(crate) use crate::bytecode;
pub(crate) use crate::frontend_ast::*;
pub(crate) use crate::frontend_lexer::*;
pub(crate) use crate::frontend_parser::*;
pub(crate) use crate::frontend_token::*;
pub(crate) use crate::ir_op::*;
pub(crate) use crate::ir_ty::*;
pub(crate) use crate::phantom::*;
pub(crate) use crate::sexp::*;
pub(crate) use crate::slice_ext::*;
pub(crate) use crate::u6::*;

pub(crate) use oxcart::Allocator;
pub(crate) use oxcart::Arena;
pub(crate) use std::array;
pub(crate) use std::fmt;
pub(crate) use std::fs::File;
pub(crate) use std::io::Write;
pub(crate) use std::str;
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
  pub(crate) use cranelift_codegen::ir::types::I128;
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
  let mut dst = Vec::with_capacity(src.len());

  for x in src.iter() {
    dst.push(f(x));
  }

  dst.into_boxed_slice()
}

#[inline(always)]
pub(crate) const fn max(x: usize, y: usize) -> usize {
  if x >= y { x } else { y }
}

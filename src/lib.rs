#![deny(unsafe_op_in_unsafe_fn)]
#![warn(elided_lifetimes_in_paths)]
// #![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

mod prelude;
pub mod bytecode;

pub use prelude::u6;

use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use target_lexicon;

use cranelift_codegen;
use cranelift_module;
// use cranelift_frontend;
// use cranelift_object;

mod cranelift {
  // use cranelift_codegen::ir::Signature;
  // use cranelift_module::FuncId;
  pub(crate) use cranelift_codegen::Context;
  pub(crate) use cranelift_codegen::ir::AbiParam;
  pub(crate) use cranelift_codegen::ir::Inst;
  pub(crate) use cranelift_codegen::ir::InstBuilder;
  pub(crate) use cranelift_codegen::ir::types::I64;
  pub(crate) use cranelift_codegen::isa::CallConv;
  pub(crate) use cranelift_codegen::isa::aarch64::AArch64Backend;
  pub(crate) use cranelift_frontend::FunctionBuilder;
  pub(crate) use cranelift_frontend::FunctionBuilderContext;
  pub(crate) use cranelift_module::Linkage;
  pub(crate) use cranelift_module::Module;
  pub(crate) use cranelift_module::ModuleCompiledFunction;
  pub(crate) use cranelift_object::ObjectBuilder;
  pub(crate) use cranelift_object::ObjectModule;
}

use cranelift::InstBuilder;
use cranelift::Module;

pub fn foo() {
  println!("hello!");
}

pub fn go() {
  let shared_flags =
    cranelift_codegen::settings::Flags::new(
      cranelift_codegen::settings::builder()
    );

  let aarch64_flags =
    cranelift_codegen::isa::aarch64::settings::Flags::new(
      &shared_flags,
      cranelift_codegen::isa::aarch64::settings::builder()
    );

  let isa =
    cranelift::AArch64Backend::new_with_flags(
      target_lexicon::HOST,
      shared_flags,
      aarch64_flags
    );

  let object_builder =
    cranelift::ObjectBuilder::new(
      Arc::new(isa),
      "???",
      cranelift_module::default_libcall_names()
    ).unwrap();

  let mut object_module = cranelift::ObjectModule::new(object_builder);

  /////////////////

  let mut ctx = cranelift::Context::new();
  let mut func_builder_ctx = cranelift::FunctionBuilderContext::new();

  /////////////////

  ctx.func.signature.call_conv = cranelift::CallConv::AppleAarch64;
  ctx.func.signature.params.push(cranelift::AbiParam::new(cranelift::I64));
  ctx.func.signature.returns.push(cranelift::AbiParam::new(cranelift::I64));
  ctx.func.signature.returns.push(cranelift::AbiParam::new(cranelift::I64));
  ctx.func.signature.returns.push(cranelift::AbiParam::new(cranelift::I64));
  ctx.func.signature.returns.push(cranelift::AbiParam::new(cranelift::I64));
  ctx.func.signature.returns.push(cranelift::AbiParam::new(cranelift::I64));

  let mut fb = cranelift::FunctionBuilder::new(&mut ctx.func, &mut func_builder_ctx);

  let entry_block = fb.create_block();

  fb.append_block_params_for_function_params(entry_block);
  fb.switch_to_block(entry_block);
  let x = fb.ins().iconst(cranelift::I64, 13);
  let _: cranelift::Inst = fb.ins().return_(&[x, x, x, x, x]);

  fb.seal_all_blocks();
  fb.finalize();

  if true {
    let mut s = String::new();
    cranelift_codegen::write::write_function(&mut s, &ctx.func).unwrap();
    std::io::stdout().write_all(s.as_bytes()).unwrap();
  }

  /////////////////

  let func_id =
    object_module.declare_function(
      "foo",
      cranelift::Linkage::Export,
      &ctx.func.signature
    ).unwrap();

  let _: cranelift::ModuleCompiledFunction =
    object_module.define_function(
      func_id,
      &mut ctx
    ).unwrap();

  ctx.clear();

  /////////////////

  let object_product = object_module.finish();
  let object_bytes = &object_product.emit().unwrap();

  let mut out = File::create("out.o").unwrap();

  out.write_all(object_bytes).unwrap()
}

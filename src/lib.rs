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
pub mod backend;
pub mod bytecode;

use crate::prelude::*;


pub fn go() {
  use bytecode::*;

  let program =
    Program {
      functions: &[
        Function {
          name: "foo",
          signature: Signature {
            inputs: &[
              ValType::I64,
              ValType::I64,
            ],
            outputs: &[
              ValType::I64,
              ValType::I64,
              ValType::I64,
              ValType::Bool,
              ValType::I64,
            ]
          },
          code: &[
            Inst::Op01(Imm::I64(13)),
            Inst::Op11(TagOp11::I64Neg, VarId(0)),
            Inst::Op21(TagOp21::I64Add, VarId(0), VarId(1)),
            Inst::Op11(TagOp11::I64IsZero, VarId(0)),
            Inst::Op11(TagOp11::I64ToI6, VarId(1)),
            Inst::Op21(TagOp21::I64Ror, VarId(0), VarId(6)),
            Inst::Return(
              &[
                VarId(2),
                VarId(3),
                VarId(4),
                VarId(5),
                VarId(7),
              ]
            ),
          ]
        }
      ]
    };

  let object_bytes = backend::compile(program);

  let mut out = File::create("out.o").unwrap();

  out.write_all(&object_bytes).unwrap()
}

/*
pub fn go() {
  let mut shared_flags = cranelift_codegen::settings::builder();
  shared_flags.set("opt_level", "speed").unwrap();
  let shared_flags = cranelift_codegen::settings::Flags::new(shared_flags);

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

  let mut fb = cranelift::FunctionBuilder::new(&mut ctx.func, &mut func_builder_ctx);

  let mut blocks = Vec::new();

  blocks.push(fb.create_block());
  // blocks.push(fb.create_block());
  // blocks.push(fb.create_block());

  let block = blocks[0];

  fb.switch_to_block(block);
  fb.append_block_params_for_function_params(block);
  let x = fb.block_params(block)[0];
  let y = fb.ins().iconst(cranelift::I64, 13);
  let z = fb.ins().iadd(x, y);
  let _: cranelift::Inst = fb.ins().return_(&[z, y, x]);

  fb.seal_all_blocks();
  fb.finalize();

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
*/

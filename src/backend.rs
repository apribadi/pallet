use crate::prelude::*;

fn compile_valtype(ty: bytecode::ValType) -> cranelift::Type {
  match ty {
    bytecode::ValType::I64 => cranelift::I64,
    bytecode::ValType::Bool => cranelift::I8,
    _ => unimplemented!(),
  }
}

pub fn compile<'a>(program: bytecode::Program<'a>) -> Box<[u8]> {
  let mut shared_flags = cranelift_codegen::settings::builder();
  shared_flags.set("opt_level", "speed").unwrap();
  let shared_flags =
    cranelift_codegen::settings::Flags::new(
      shared_flags
    );

  let aarch64_flags = cranelift_codegen::isa::aarch64::settings::builder();
  let aarch64_flags =
    cranelift_codegen::isa::aarch64::settings::Flags::new(
      &shared_flags,
      aarch64_flags
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

  let mut ctx = cranelift::Context::new();
  let mut fbc = cranelift::FunctionBuilderContext::new();

  let mut func_ids = Vec::new();

  // declare functions

  for &func in program.functions.iter() {
    ctx.func.signature.call_conv = cranelift::CallConv::AppleAarch64;

    for &ty in func.signature.inputs.iter() {
      let ty = compile_valtype(ty);
      ctx.func.signature.params.push(cranelift::AbiParam::new(ty));
    }

    for &ty in func.signature.outputs.iter() {
      let ty = compile_valtype(ty);
      ctx.func.signature.returns.push(cranelift::AbiParam::new(ty));
    }

    let func_id =
      object_module.declare_function(
        func.name,
        cranelift::Linkage::Export,
        &ctx.func.signature
      ).unwrap();

    func_ids.push(func_id);

    ctx.clear()
  }

  // define functions

  for (func_idx, &func) in program.functions.iter().enumerate() {
    ctx.func.signature.call_conv = cranelift::CallConv::AppleAarch64;

    for &ty in func.signature.inputs.iter() {
      let ty = compile_valtype(ty);
      ctx.func.signature.params.push(cranelift::AbiParam::new(ty));
    }

    for &ty in func.signature.outputs.iter() {
      let ty = compile_valtype(ty);
      ctx.func.signature.returns.push(cranelift::AbiParam::new(ty));
    }

    let mut fb = cranelift::FunctionBuilder::new(&mut ctx.func, &mut fbc);

    let mut vars = Vec::new();

    let entry = fb.create_block();
    fb.append_block_params_for_function_params(entry);
    fb.switch_to_block(entry);

    for (i, &_) in func.signature.inputs.iter().enumerate() {
      vars.push(fb.block_params(entry)[i]);
    }

    for &inst in func.code.iter() {
      match inst {
        bytecode::Inst::Op01(imm) => {
          match imm {
            bytecode::Imm::Bool(x) => {
              let u = fb.ins().iconst(cranelift::I8, u8::from(x) as i64);
              vars.push(u);
            }
            bytecode::Imm::I6(x) => {
              let u = fb.ins().iconst(cranelift::I8, u8::from(x) as i64);
              vars.push(u);
            }
            bytecode::Imm::I64(x) => {
              let u = fb.ins().iconst(cranelift::I64, x as i64);
              vars.push(u);
            }
          }
        }
        bytecode::Inst::Op11(tag, x) => {
          match tag {
            bytecode::TagOp11::BoolNot => {
              let u = fb.ins().bxor_imm(vars[usize::from(x)], 1);
              vars.push(u);
            }
            bytecode::TagOp11::I64Neg => {
              let u = fb.ins().ineg(vars[usize::from(x)]);
              vars.push(u);
            }
            _ => {
              unimplemented!()
            }
          }
        }
        bytecode::Inst::Op21(tag, x, y) => {
          match tag {
            bytecode::TagOp21::I64Add => {
              let u = fb.ins().iadd(vars[usize::from(x)], vars[usize::from(y)]);
              vars.push(u);
            }
            _ => {
              unimplemented!()
            }
          }
        }
        bytecode::Inst::Return(xs) => {
          let _: cranelift::Inst =
            fb.ins().return_(&map_slice(xs, |&x| vars[usize::from(x)]));
        }
        _ => unimplemented!()
      }
    }

    fb.seal_all_blocks();
    fb.finalize();

    let func_id = func_ids[func_idx];

    let _: cranelift::ModuleCompiledFunction =
      object_module.define_function(
        func_id,
        &mut ctx
      ).unwrap();

    let mut s = String::new();
    cranelift_codegen::write::write_function(&mut s, &ctx.func).unwrap();
    std::io::stdout().write_all(s.as_bytes()).unwrap();

    ctx.clear()
  }

  let object_product = object_module.finish();

  object_product.emit().unwrap().into_boxed_slice()
}

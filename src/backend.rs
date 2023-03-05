use crate::prelude::*;

// REPRESENTATION AS CRANELIFT TYPES
//
// bool -> i8, either 0x00 or 0x01
// i6   -> i8, with the two MSBs *unspecified*

fn compile_valtype(ty: bytecode::ValType) -> cranelift::Type {
  match ty {
    bytecode::ValType::Bool => cranelift::I8,
    bytecode::ValType::I6 => cranelift::I8,
    bytecode::ValType::I64 => cranelift::I64,
    _ => unimplemented!(),
  }
}

pub fn compile<'a>(program: bytecode::Program<'a>) -> Box<[u8]> {
  const CALL_CONV: cranelift::CallConv = cranelift::CallConv::AppleAarch64;

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
    ctx.func.signature.call_conv = CALL_CONV;

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
    ctx.func.signature.call_conv = CALL_CONV;

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
              let x = u8::from(x);
              let u = fb.ins().iconst(cranelift::I8, x as i64);
              vars.push(u);
            }
            bytecode::Imm::I6(x) => {
              let x = u8::from(x);
              let u = fb.ins().iconst(cranelift::I8, x as i64);
              vars.push(u);
            }
            bytecode::Imm::I64(x) => {
              let u = fb.ins().iconst(cranelift::I64, x as i64);
              vars.push(u);
            }
          }
        }
        bytecode::Inst::Op11(tag, x) => {
          let x = vars[usize::from(x)];
          match tag {
            bytecode::TagOp11::BoolNot => {
              let u = fb.ins().bxor_imm(x, 1);
              vars.push(u);
            }
            bytecode::TagOp11::I64BitNot => {
              let u = fb.ins().bnot(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64Clz => {
              let u = fb.ins().clz(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64Ctz => {
              let u = fb.ins().ctz(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64IsZero => {
              let u = fb.ins().icmp_imm(cranelift::IntCC::Equal, x, 0);
              vars.push(u);
            }
            bytecode::TagOp11::I64Neg => {
              let u = fb.ins().ineg(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64Popcnt => {
              let u = fb.ins().popcnt(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64Swap => {
              let u = fb.ins().bswap(x);
              vars.push(u);
            }
            bytecode::TagOp11::I64ToI6 => {
              let u = fb.ins().ireduce(cranelift::I8, x);
              vars.push(u);
            }
          }
        }
        bytecode::Inst::Op21(tag, x, y) => {
          let x = vars[usize::from(x)];
          let y = vars[usize::from(y)];
          match tag {
            bytecode::TagOp21::BoolAnd => {
              let u = fb.ins().band(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::BoolEq => {
              let u = fb.ins().icmp(cranelift::IntCC::Equal, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::BoolNeq => {
              let u = fb.ins().icmp(cranelift::IntCC::NotEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::BoolOr => {
              let u = fb.ins().bor(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64Add => {
              let u = fb.ins().iadd(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64BitAnd => {
              let u = fb.ins().band(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64BitOr => {
              let u = fb.ins().bor(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64BitXor => {
              let u = fb.ins().bxor(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsEq => {
              let u = fb.ins().icmp(cranelift::IntCC::Equal, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsGeS => {
              let u = fb.ins().icmp(cranelift::IntCC::SignedGreaterThanOrEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsGeU => {
              let u = fb.ins().icmp(cranelift::IntCC::UnsignedGreaterThanOrEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsGtS => {
              let u = fb.ins().icmp(cranelift::IntCC::SignedGreaterThan, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsGtU => {
              let u = fb.ins().icmp(cranelift::IntCC::UnsignedGreaterThan, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsLeS => {
              let u = fb.ins().icmp(cranelift::IntCC::SignedLessThanOrEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsLeU => {
              let u = fb.ins().icmp(cranelift::IntCC::UnsignedLessThanOrEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsLtS => {
              let u = fb.ins().icmp(cranelift::IntCC::SignedLessThan, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsLtU => {
              let u = fb.ins().icmp(cranelift::IntCC::UnsignedLessThan, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64IsNeq => {
              let u = fb.ins().icmp(cranelift::IntCC::NotEqual, x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MaxS => {
              let u = fb.ins().smax(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MaxU => {
              let u = fb.ins().umax(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MinS => {
              let u = fb.ins().smin(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MinU => {
              let u = fb.ins().umin(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64Mul => {
              let u = fb.ins().imul(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MulHiS => {
              let u = fb.ins().smulhi(x, y);
              vars.push(u);
            }
            bytecode::TagOp21::I64MulHiU => {
              let u = fb.ins().umulhi(x, y);
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

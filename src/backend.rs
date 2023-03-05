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
    let mut blocks = Vec::new();
    let mut num_blocks = 0;

    let entry = fb.create_block();
    fb.append_block_params_for_function_params(entry);
    fb.switch_to_block(entry);

    for (i, &_) in func.signature.inputs.iter().enumerate() {
      vars.push(fb.block_params(entry)[i]);
    }

    for &inst in func.code.iter() {
      match inst {
        bytecode::Inst::Block(tys) => {
          num_blocks += 1;
          while blocks.len() < num_blocks { blocks.push(fb.create_block()); }
          let block = blocks[num_blocks - 1];
          fb.switch_to_block(block);

          for &ty in tys.iter() {
            vars.push(fb.append_block_param(block, compile_valtype(ty)));
          }
        }
        bytecode::Inst::If1(tag, p, a, xs, b, ys) => {
          let a = a.0 as usize;
          let b = b.0 as usize;
          let n = max(a, b) + 1;
          while blocks.len() < n { blocks.push(fb.create_block()); }
          let a = blocks[a];
          let b = blocks[b];
          let p = vars[usize::from(p)];

          match tag {
            bytecode::TagIf1::I64IfNonZero => {
              let _: _ =
                fb.ins().brif(
                  p,
                  a,
                  &map_slice(xs, |&x| vars[usize::from(x)]),
                  b,
                  &map_slice(ys, |&y| vars[usize::from(y)])
                );
            }
            bytecode::TagIf1::If => {
              let _: _ =
                fb.ins().brif(
                  p,
                  a,
                  &map_slice(xs, |&x| vars[usize::from(x)]),
                  b,
                  &map_slice(ys, |&y| vars[usize::from(y)])
                );
            }
          }

          let _: _ = fb.ins().jump(a, &map_slice(xs, |&x| vars[usize::from(x)]));
        }
        bytecode::Inst::Jump(a, xs) => {
          let a = a.0 as usize;
          while blocks.len() < a + 1 { blocks.push(fb.create_block()); }
          let a = blocks[a];

          let _: _ = fb.ins().jump(a, &map_slice(xs, |&x| vars[usize::from(x)]));
        }
        bytecode::Inst::Op01(imm) => {
          let u =
            match imm {
              bytecode::Imm::Bool(x) =>
                fb.ins().iconst(cranelift::I8, u8::from(x) as i64),
              bytecode::Imm::I6(x) =>
                fb.ins().iconst(cranelift::I8, u8::from(x) as i64),
              bytecode::Imm::I64(x) =>
                fb.ins().iconst(cranelift::I64, x as i64),
            };
          vars.push(u);
        }
        bytecode::Inst::Op11(tag, x) => {
          let x = vars[usize::from(x)];
          let u =
            match tag {
              bytecode::TagOp11::BoolNot =>
                fb.ins().bxor_imm(x, 1),
              bytecode::TagOp11::I64BitNot =>
                fb.ins().bnot(x),
              bytecode::TagOp11::I64Clz =>
                fb.ins().clz(x),
              bytecode::TagOp11::I64Ctz =>
                fb.ins().ctz(x),
              bytecode::TagOp11::I64IsNonZero =>
                fb.ins().icmp_imm(cranelift::IntCC::NotEqual, x, 0),
              bytecode::TagOp11::I64Neg =>
                fb.ins().ineg(x),
              bytecode::TagOp11::I64Popcount =>
                fb.ins().popcnt(x),
              bytecode::TagOp11::I64Swap =>
                fb.ins().bswap(x),
              bytecode::TagOp11::I64ToI6 =>
                fb.ins().ireduce(cranelift::I8, x),
            };
          vars.push(u);
        }
        bytecode::Inst::Op21(tag, x, y) => {
          let x = vars[usize::from(x)];
          let y = vars[usize::from(y)];
          let u =
            match tag {
              bytecode::TagOp21::BoolAnd =>
                fb.ins().band(x, y),
              bytecode::TagOp21::BoolEq =>
                fb.ins().icmp(cranelift::IntCC::Equal, x, y),
              bytecode::TagOp21::BoolNeq =>
                fb.ins().icmp(cranelift::IntCC::NotEqual, x, y),
              bytecode::TagOp21::BoolOr =>
                fb.ins().bor(x, y),
              bytecode::TagOp21::I64Add =>
                fb.ins().iadd(x, y),
              bytecode::TagOp21::I64BitAnd =>
                fb.ins().band(x, y),
              bytecode::TagOp21::I64BitOr =>
                fb.ins().bor(x, y),
              bytecode::TagOp21::I64BitXor =>
                fb.ins().bxor(x, y),
              bytecode::TagOp21::I64IsEq =>
                fb.ins().icmp(cranelift::IntCC::Equal, x, y),
              bytecode::TagOp21::I64IsGeS =>
                fb.ins().icmp(cranelift::IntCC::SignedGreaterThanOrEqual, x, y),
              bytecode::TagOp21::I64IsGeU =>
                fb.ins().icmp(cranelift::IntCC::UnsignedGreaterThanOrEqual, x, y),
              bytecode::TagOp21::I64IsGtS =>
                fb.ins().icmp(cranelift::IntCC::SignedGreaterThan, x, y),
              bytecode::TagOp21::I64IsGtU =>
                fb.ins().icmp(cranelift::IntCC::UnsignedGreaterThan, x, y),
              bytecode::TagOp21::I64IsLeS =>
                fb.ins().icmp(cranelift::IntCC::SignedLessThanOrEqual, x, y),
              bytecode::TagOp21::I64IsLeU =>
                fb.ins().icmp(cranelift::IntCC::UnsignedLessThanOrEqual, x, y),
              bytecode::TagOp21::I64IsLtS =>
                fb.ins().icmp(cranelift::IntCC::SignedLessThan, x, y),
              bytecode::TagOp21::I64IsLtU =>
                fb.ins().icmp(cranelift::IntCC::UnsignedLessThan, x, y),
              bytecode::TagOp21::I64IsNeq =>
                fb.ins().icmp(cranelift::IntCC::NotEqual, x, y),
              bytecode::TagOp21::I64MaxS =>
                fb.ins().smax(x, y),
              bytecode::TagOp21::I64MaxU =>
                fb.ins().umax(x, y),
              bytecode::TagOp21::I64MinS =>
                fb.ins().smin(x, y),
              bytecode::TagOp21::I64MinU =>
                fb.ins().umin(x, y),
              bytecode::TagOp21::I64Mul =>
                fb.ins().imul(x, y),
              bytecode::TagOp21::I64MulHiS =>
                fb.ins().smulhi(x, y),
              bytecode::TagOp21::I64MulHiU =>
                fb.ins().umulhi(x, y),
              bytecode::TagOp21::I64Rol =>
                fb.ins().rotl(x, y),
              bytecode::TagOp21::I64Ror =>
                fb.ins().rotr(x, y),
              bytecode::TagOp21::I64Shl =>
                fb.ins().ishl(x, y),
              bytecode::TagOp21::I64ShrS =>
                fb.ins().sshr(x, y),
              bytecode::TagOp21::I64ShrU =>
                fb.ins().ushr(x, y),
              bytecode::TagOp21::I64Sub =>
                fb.ins().isub(x, y),
            };
          vars.push(u);
        }
        bytecode::Inst::Op22(tag, x, y) => {
          let x = vars[usize::from(x)];
          let y = vars[usize::from(y)];
          let [u, v] =
            match tag {
              bytecode::TagOp22::I64MulFullS => [
                fb.ins().imul(x, y),
                fb.ins().smulhi(x, y),
              ],
              bytecode::TagOp22::I64MulFullU => [
                fb.ins().imul(x, y),
                fb.ins().umulhi(x, y),
              ]
            };
          vars.push(u);
          vars.push(v);
        }
        bytecode::Inst::Op31(tag, x, y, z) => {
          let x = vars[usize::from(x)];
          let y = vars[usize::from(y)];
          let z = vars[usize::from(z)];
          let u =
            match tag {
              bytecode::TagOp31::I64Sel =>
                fb.ins().select(x, y, z),
            };
          vars.push(u);
        }
        bytecode::Inst::Return(xs) => {
          let _: _ = fb.ins().return_(&map_slice(xs, |&x| vars[usize::from(x)]));
        }
        _ => unimplemented!()
      }
    }

    fb.seal_all_blocks();
    fb.finalize();

    let mut s = String::new();
    cranelift_codegen::write::write_function(&mut s, &ctx.func).unwrap();
    std::io::stdout().write_all(s.as_bytes()).unwrap();

    let func_id = func_ids[func_idx];

    let _: cranelift::ModuleCompiledFunction =
      object_module.define_function(
        func_id,
        &mut ctx
      ).unwrap();

    ctx.clear()
  }

  let object_product = object_module.finish();

  object_product.emit().unwrap().into_boxed_slice()
}

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(elided_lifetimes_in_paths)]
#![warn(non_ascii_idents)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

mod prelude;
pub mod backend;
pub mod buf;
pub mod bytecode;
pub mod c;
pub mod op;
pub mod slice_ext;

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
            Inst::Const(Imm::I64(13)),
            Inst::Jump(BlockId(0), &[VarId(1), VarId(0)]),
            Inst::Block(&[ValType::I64, ValType::I64]),
            Inst::Op11(Op11::I64Neg, VarId(3)),
            Inst::Op21(Op21::I64Add, VarId(3), VarId(4)),
            Inst::Op11(Op11::I64IsNonZero, VarId(0)),
            Inst::Op11(Op11::I64ToI6, VarId(1)),
            Inst::Op21(Op21::I64Ror, VarId(0), VarId(8)),
            Inst::Return(
              &[
                VarId(4),
                VarId(5),
                VarId(6),
                VarId(7),
                VarId(9),
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

#![allow(non_camel_case_types)]
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
pub mod bc;
pub mod buf;
pub mod bytecode;
pub mod frontend_ast;
pub mod frontend_lexer;
pub mod frontend_parser;
pub mod frontend_token;
pub mod ir_bytecode;
pub mod ir_op;
pub mod ir_ty;
pub mod phantom;
pub mod sexp;
pub mod slice_ext;
pub mod u6;

use crate::prelude::*;

pub fn go() {
  let source =
      b"\
fun foo(n)
  n
end";

  let mut arena = Arena::new();
  let allocator = arena.allocator_mut();
  let mut parser = Parser::new(source);


  if let Ok(x) = parser.parse_item(allocator) {
    println!("{}", x.to_sexp());
  }

  use bytecode::*;

  let program =
    Program {
      functions: &[
        Function {
          name: "foo",
          signature: Signature {
            inputs: &[
              Ty::I64,
              Ty::I64,
            ],
            outputs: &[
              Ty::I64,
              Ty::I64,
              Ty::I64,
              Ty::Bool,
              Ty::I64,
            ]
          },
          code: &[
            Inst::ImmI64(13),
            Inst::Goto(BlockId(0), &[VarId(1), VarId(0)]),
            Inst::Block(&[Ty::I64, Ty::I64]),
            Inst::Op11(Op11::I64Neg, VarId(3)),
            Inst::Op21(Op21::I64Add, VarId(3), VarId(4)),
            Inst::Op11(Op11::I64IsNonZero, VarId(0)),
            Inst::Op11(Op11::I64ToI6, VarId(1)),
            Inst::Op21(Op21::I64Ror, VarId(0), VarId(8)),
            Inst::Ret(
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

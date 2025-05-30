mod ast2ir;
mod ir2cranelift;

pub use ast2ir::compile_ast_to_ir;
pub use ir2cranelift::*;

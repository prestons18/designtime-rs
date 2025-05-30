//! DesignTime Language Core Library
//!
//! This is the central crate for lexing, parsing, IR generation,
//! bytecode generation, and VM execution. It also supports multi-target codegen (e.g. JSX).

pub mod ast;
pub mod bytecode;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod targets;
pub mod vm;
pub mod workspace;

pub use bytecode::{Bytecode, generate_bytecode};
pub use ir::ir_node;
pub use lexer::tokenize;
pub use parser::parse;
pub use targets::{TargetCodegen, jsx};
pub use vm::{VMResult, VirtualMachine};
pub use workspace::{ProjectConfig, Workspace};

// /// Compile source to IR
// pub fn compile_to_ir(source: &str) -> Result<ir::, String> {
//     let tokens = lexer::tokenize(source)?;
//     let ast = parser::parse(tokens)?;
//     let ir = ir::generate_ir(ast)?;
//     Ok(ir)
// }

/// Compile source to bytecode
pub fn compile_to_bytecode(source: &str) -> Result<bytecode::Bytecode, String> {
    let ir = compile_to_ir(source)?;
    let bc = bytecode::generate_bytecode(&ir)?;
    Ok(bc)
}

/// Run code directly via the VM
pub fn run_in_vm(source: &str) -> Result<vm::VMResult, String> {
    let bc = compile_to_bytecode(source)?;
    let mut vm = vm::VirtualMachine::new();
    vm.execute(&bc)
}

/// Compile source to JSX code
pub fn compile_to_jsx(source: &str) -> Result<String, String> {
    let ir = compile_to_ir(source)?;
    targets::jsx::generate_jsx(&ir)
}

//! DesignTime Language Core Library
//!
//! This is the central crate for lexing, parsing, IR generation,
//! bytecode generation, and VM execution. It will also support multi-target codegen (e.g. JSX).

pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod util;
pub mod workspace;

//pub mod targets;
// pub mod vm;
// pub mod bytecode;

pub use ir::ir_node;
pub use lexer::token;
pub use parser::Parser;
pub use workspace::{WorkspaceConfig, validate_and_load_workspace};

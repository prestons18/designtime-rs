pub mod ast;
pub mod lexer;
pub mod parser;
pub mod ir;

// Re-export commonly used items
pub use ast::{ASTNode, Node, ImportDecl, PageDecl, Function, Attribute};
pub use lexer::{Lexer, Token};
pub use parser::Parser;
pub use ir::*; // todo
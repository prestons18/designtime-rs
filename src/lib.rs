pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod workspace;

// Re-export commonly used items
pub use ast::{ASTNode, Attribute, Function, ImportDecl, Node, PageDecl};
pub use ir::*;
pub use lexer::{Lexer, Token};
pub use parser::Parser;
pub use workspace::*;

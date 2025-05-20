use crate::ast::Expr;

#[derive(Debug)]
pub enum Instruction {
    Print(String),
}

pub fn compile_ast(ast: &[Expr]) -> Vec<Instruction> {
    ast.iter().map(|expr| match expr {
        Expr::Print(msg) => Instruction::Print(msg.clone()),
    }).collect()
}

use crate::lexer::{Token};
use crate::ast::Expr;

pub fn parse(tokens: &[Token]) -> Vec<Expr> {
    let mut i = 0;
    let mut ast = Vec::new();

    while i < tokens.len() {
        match tokens[i] {
            Token::Print => {
                i += 1;
                if tokens[i] != Token::LParen { panic!("Expected ("); }
                i += 1;
                if let Token::String(ref s) = tokens[i] {
                    i += 1;
                    if tokens[i] != Token::RParen { panic!("Expected )"); }
                    ast.push(Expr::Print(s.clone()));
                    i += 1;
                } else {
                    panic!("Expected string inside print()");
                }
            }
            Token::EOF => break,
            _ => panic!("Unexpected token: {:?}", tokens[i]),
        }
    }

    ast
}

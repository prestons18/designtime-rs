use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Lt, // <
    Gt, // >

    Slash, // /
    Eq,    // =

    Name(String),          // tag names like div, span
    Text(String),          // text
    InnerText(String),     // inner text
    StringLiteral(String), // string literal
    Unknown(char),         // unknown char

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

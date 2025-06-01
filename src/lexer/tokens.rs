#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Lt,           // <
    Gt,           // >
    Slash,        // /
    Name(String), // tag names like div, span
    Text(String), // inner text
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

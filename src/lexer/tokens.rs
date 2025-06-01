#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Lt,           // <
    Gt,           // >
    Slash,        // /
    Name(String), // tag names like div, span
    Text(String), // text
    InnerText(String), // inner text
    Unknown(char), // unknown char
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

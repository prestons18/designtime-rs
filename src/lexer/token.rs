use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Import,
    From,
    Page,
    Layout,
    Render,
    Functions,

    // Identifiers and literals
    Ident(String),
    StringLiteral(String),
    Number(f64),
    Text(String), // For JSX text content

    // Punctuation
    LBrace,    // '{'
    RBrace,    // '}'
    Colon,     // ':'
    Comma,     // ','
    LParen,    // '('
    RParen,    // ')'
    LT,        // '<'
    GT,        // '>'
    SlashGT,   // '/>'
    Arrow,     // '=>'
    Slash,     // '/'
    SemiColon, // ';'
    Plus,      // '+'
    Minus,     // '-'
    Star,      // '*'
    EQ,        // '='

    // Special
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Import => write!(f, "import"),
            Token::From => write!(f, "from"),
            Token::Page => write!(f, "page"),
            Token::Layout => write!(f, "layout"),
            Token::Render => write!(f, "render"),
            Token::Functions => write!(f, "functions"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Text(s) => write!(f, "{}", s),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::SlashGT => write!(f, "/>"),
            Token::Arrow => write!(f, "=>"),
            Token::Slash => write!(f, "/"),
            Token::SemiColon => write!(f, ";"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::EQ => write!(f, "="),
            Token::EOF => write!(f, ""),
        }
    }
}

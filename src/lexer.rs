#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    LParen,
    RParen,
    String(String),
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '"' => {
                chars.next(); // skip opening quote
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' { break; }
                    string.push(c);
                }
                tokens.push(Token::String(string));
            }
            _ => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() { ident.push(c); chars.next(); } else { break; }
                }
                match ident.as_str() {
                    "print" => tokens.push(Token::Print),
                    _ => panic!("Unknown identifier: {}", ident),
                }
            }
        }
    }

    tokens.push(Token::EOF);
    tokens
}

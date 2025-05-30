use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

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

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            input: src.chars().peekable(),
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.input.next()
    }
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    fn peek_ahead(&self) -> Option<char> {
        let mut iter = self.input.clone();
        iter.next(); // Skip current character
        iter.next()
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut jsx_depth = 0; // Track JSX nesting depth

        while let Some(&ch) = self.peek() {
            match ch {
                // Skip whitespace
                c if c.is_whitespace() => {
                    self.bump();
                }

                // Handle arrow function syntax
                '=' if self.peek_ahead() == Some('>') => {
                    self.bump(); // consume '='
                    self.bump(); // consume '>'
                    tokens.push(Token::Arrow);
                }

                // Single character tokens
                '=' => {
                    tokens.push(Token::EQ);
                    self.bump();
                }
                '{' => {
                    // In JSX context, '{' might be an expression
                    if jsx_depth > 0 {
                        // This could be a JSX expression - let the JSX handler deal with it
                        self.tokenize_jsx_expression_or_brace(&mut tokens, &mut jsx_depth);
                    } else {
                        tokens.push(Token::LBrace);
                        self.bump();
                    }
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    self.bump();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.bump();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.bump();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.bump();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.bump();
                }
                ';' => {
                    tokens.push(Token::SemiColon);
                    self.bump();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.bump();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.bump();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.bump();
                }

                // Handle JSX tags
                '<' => {
                    self.tokenize_jsx_element(&mut tokens, &mut jsx_depth);
                }

                // Handle string literals
                '"' => {
                    self.bump(); // consume opening quote
                    let mut lit = String::new();
                    while let Some(&c) = self.peek() {
                        if c == '"' {
                            self.bump(); // consume closing quote
                            break;
                        }
                        lit.push(c);
                        self.bump();
                    }
                    tokens.push(Token::StringLiteral(lit));
                }

                // Handle numbers
                c if c.is_ascii_digit() => {
                    let mut num_str = String::new();
                    let mut has_dot = false;

                    while let Some(&c2) = self.peek() {
                        if c2.is_ascii_digit() {
                            num_str.push(c2);
                            self.bump();
                        } else if c2 == '.' && !has_dot {
                            // Look ahead to see if there's a digit after the dot
                            if let Some(next_char) = self.peek_ahead() {
                                if next_char.is_ascii_digit() {
                                    has_dot = true;
                                    num_str.push('.');
                                    self.bump();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if let Ok(num) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    }
                }

                // Handle identifiers and keywords
                c if is_ident_start(c) => {
                    let mut ident = String::new();
                    while let Some(&c2) = self.peek() {
                        if is_ident_cont(c2) {
                            ident.push(c2);
                            self.bump();
                        } else {
                            break;
                        }
                    }

                    // Check for keywords
                    let tok = match ident.as_str() {
                        "import" => Token::Import,
                        "from" => Token::From,
                        "page" => Token::Page,
                        "layout" => Token::Layout,
                        "render" => Token::Render,
                        "functions" => Token::Functions,
                        _ => Token::Ident(ident),
                    };
                    tokens.push(tok);
                }

                // Skip unknown characters
                _ => {
                    self.bump();
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn tokenize_jsx_element(&mut self, tokens: &mut Vec<Token>, jsx_depth: &mut i32) {
        // Consume '<'
        self.bump();
        tokens.push(Token::LT);

        // Check for closing tag
        let is_closing_tag = if self.peek() == Some(&'/') {
            self.bump();
            tokens.push(Token::Slash);
            true
        } else {
            false
        };

        // Get tag name
        self.skip_whitespace();
        let tag_name = self.consume_ident();
        if !tag_name.is_empty() {
            tokens.push(Token::Ident(tag_name));
        }

        if is_closing_tag {
            // For closing tags, just consume '>' and decrease depth
            self.skip_whitespace();
            if self.peek() == Some(&'>') {
                self.bump();
                tokens.push(Token::GT);
                *jsx_depth -= 1;
            }
            return;
        }

        // Parse attributes for opening tags
        let mut is_self_closing = false;
        loop {
            self.skip_whitespace();

            // Check for end of tag
            if self.peek() == Some(&'>') {
                self.bump();
                tokens.push(Token::GT);
                *jsx_depth += 1; // Increase depth for opening tag
                break;
            } else if self.peek() == Some(&'/') && self.peek_ahead() == Some('>') {
                self.bump(); // consume '/'
                self.bump(); // consume '>'
                tokens.push(Token::SlashGT);
                is_self_closing = true;
                break;
            }

            // Parse attribute name
            let attr_name = self.consume_ident();
            if attr_name.is_empty() {
                break;
            }
            tokens.push(Token::Ident(attr_name));

            self.skip_whitespace();

            // Check for attribute value
            if self.peek() == Some(&'=') {
                self.bump();
                tokens.push(Token::EQ);
                self.skip_whitespace();

                match self.peek() {
                    Some('"') => {
                        // String attribute value
                        self.bump(); // consume opening quote
                        let mut val = String::new();
                        while let Some(&c) = self.peek() {
                            if c == '"' {
                                self.bump(); // consume closing quote
                                break;
                            }
                            val.push(c);
                            self.bump();
                        }
                        tokens.push(Token::StringLiteral(val));
                    }
                    Some('{') => {
                        // Expression attribute value
                        self.bump();
                        tokens.push(Token::LBrace);
                        let expr = self.consume_jsx_expression();
                        tokens.push(Token::Ident(expr));
                        tokens.push(Token::RBrace);
                    }
                    _ => {}
                }
            }
        }

        // Only collect text content if we're in JSX and it's not self-closing
        if !is_self_closing && *jsx_depth > 0 {
            self.collect_jsx_text_content(tokens);
        }
    }

    fn tokenize_jsx_expression_or_brace(&mut self, tokens: &mut Vec<Token>, jsx_depth: &mut i32) {
        // If we're in JSX context, this might be a JSX expression
        if *jsx_depth > 0 {
            // Look ahead to see if this looks like a JSX expression
            let mut temp_iter = self.input.clone();
            temp_iter.next(); // skip '{'

            // Skip whitespace
            while let Some(&c) = temp_iter.peek() {
                if c.is_whitespace() {
                    temp_iter.next();
                } else {
                    break;
                }
            }

            // Check if this looks like JavaScript code vs JSX expression
            if let Some(&next_char) = temp_iter.peek() {
                if next_char.is_alphabetic() || next_char == '_' {
                    // Might be a JSX expression, handle specially
                    self.bump(); // consume '{'
                    tokens.push(Token::LBrace);
                    let expr = self.consume_jsx_expression();
                    tokens.push(Token::Ident(expr));
                    tokens.push(Token::RBrace);
                    return;
                }
            }
        }

        // Default: just treat as regular brace
        tokens.push(Token::LBrace);
        self.bump();
    }

    fn collect_jsx_text_content(&mut self, tokens: &mut Vec<Token>) {
        let mut text_content = String::new();

        while let Some(&c) = self.peek() {
            if c == '<' {
                // End of text content, start of new tag
                break;
            } else if c == '{' {
                // JSX expression within text
                if !text_content.trim().is_empty() {
                    tokens.push(Token::Text(text_content.trim().to_string()));
                    text_content.clear();
                }
                // The main loop will handle the '{'
                break;
            }
            text_content.push(c);
            self.bump();
        }

        let trimmed = text_content.trim();
        if !trimmed.is_empty() {
            tokens.push(Token::Text(trimmed.to_string()));
        }
    }

    fn consume_jsx_expression(&mut self) -> String {
        let mut expr = String::new();
        let mut depth = 1;
        let mut in_string = false;
        let mut string_delim = None;

        while let Some(&c) = self.peek() {
            match c {
                '"' | '\'' => {
                    if in_string && Some(c) == string_delim {
                        in_string = false;
                        string_delim = None;
                    } else if !in_string {
                        in_string = true;
                        string_delim = Some(c);
                    }
                    expr.push(c);
                    self.bump();
                }
                '{' if !in_string => {
                    depth += 1;
                    expr.push(c);
                    self.bump();
                }
                '}' if !in_string => {
                    depth -= 1;
                    if depth == 0 {
                        self.bump(); // consume closing '}'
                        break;
                    }
                    expr.push(c);
                    self.bump();
                }
                _ => {
                    expr.push(c);
                    self.bump();
                }
            }
        }

        expr.trim().to_string()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn consume_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(&c) = self.peek() {
            if is_ident_cont(c) {
                ident.push(c);
                self.bump();
            } else {
                break;
            }
        }
        ident
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_cont(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dt_syntax_tokenization() {
        let input = r#"
import { Checkbox } from "@designtime.core.ui.MUI"

page Home {
    layout: Glassmorphism
    render: { 
        <div class="container">
            <h1>Welcome to DesignTime</h1>
            <Checkbox checked={true}>Do you see this?</Checkbox>
        </div>
    }
    functions: {
        onSelect: () => {
            let x = 40;
            let y = 2;
            let result = x + y;
            return result;
        }
    }
}
        "#;

        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        // Print tokens for debugging
        for (i, token) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, token);
        }

        // Basic assertions
        assert!(tokens.contains(&Token::Import));
        assert!(tokens.contains(&Token::Page));
        assert!(tokens.contains(&Token::Ident("Home".to_string())));
    }
}

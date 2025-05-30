use std::iter::Peekable;
use std::str::Chars;

use crate::lexer::token::Token;

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
        iter.next(); // Skip current char
        iter.next()
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut jsx_depth = 0;

        while let Some(&ch) = self.peek() {
            match ch {
                c if c.is_whitespace() => {
                    self.bump();
                }

                '=' if self.peek_ahead() == Some('>') => {
                    self.bump();
                    self.bump();
                    tokens.push(Token::Arrow);
                }

                '=' => {
                    tokens.push(Token::EQ);
                    self.bump();
                }
                '{' => {
                    if jsx_depth > 0 {
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

                '<' => self.tokenize_jsx_element(&mut tokens, &mut jsx_depth),

                '"' => {
                    self.bump(); // consume opening quote
                    let mut lit = String::new();
                    while let Some(&c) = self.peek() {
                        if c == '"' {
                            self.bump();
                            break;
                        }
                        lit.push(c);
                        self.bump();
                    }
                    tokens.push(Token::StringLiteral(lit));
                }

                c if c.is_ascii_digit() => {
                    let mut num_str = String::new();
                    let mut has_dot = false;
                    while let Some(&c2) = self.peek() {
                        if c2.is_ascii_digit() {
                            num_str.push(c2);
                            self.bump();
                        } else if c2 == '.' && !has_dot {
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

                _ => {
                    self.bump();
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn tokenize_jsx_element(&mut self, tokens: &mut Vec<Token>, jsx_depth: &mut i32) {
        self.bump();
        tokens.push(Token::LT);

        let is_closing_tag = if self.peek() == Some(&'/') {
            self.bump();
            tokens.push(Token::Slash);
            true
        } else {
            false
        };

        self.skip_whitespace();
        let tag_name = self.consume_ident();
        if !tag_name.is_empty() {
            tokens.push(Token::Ident(tag_name));
        }

        if is_closing_tag {
            self.skip_whitespace();
            if self.peek() == Some(&'>') {
                self.bump();
                tokens.push(Token::GT);
                *jsx_depth -= 1;
            }
            return;
        }

        let mut is_self_closing = false;
        loop {
            self.skip_whitespace();
            if self.peek() == Some(&'>') {
                self.bump();
                tokens.push(Token::GT);
                *jsx_depth += 1;
                break;
            } else if self.peek() == Some(&'/') && self.peek_ahead() == Some('>') {
                self.bump();
                self.bump();
                tokens.push(Token::SlashGT);
                is_self_closing = true;
                break;
            }

            let attr_name = self.consume_ident();
            if attr_name.is_empty() {
                break;
            }
            tokens.push(Token::Ident(attr_name));

            self.skip_whitespace();

            if self.peek() == Some(&'=') {
                self.bump();
                tokens.push(Token::EQ);
                self.skip_whitespace();

                match self.peek() {
                    Some('"') => {
                        self.bump();
                        let mut val = String::new();
                        while let Some(&c) = self.peek() {
                            if c == '"' {
                                self.bump();
                                break;
                            }
                            val.push(c);
                            self.bump();
                        }
                        tokens.push(Token::StringLiteral(val));
                    }
                    Some('{') => {
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

        if !is_self_closing && *jsx_depth > 0 {
            self.collect_jsx_text_content(tokens);
        }
    }

    fn tokenize_jsx_expression_or_brace(&mut self, tokens: &mut Vec<Token>, jsx_depth: &mut i32) {
        if *jsx_depth > 0 {
            let mut temp_iter = self.input.clone();
            temp_iter.next();

            while let Some(&c) = temp_iter.peek() {
                if c.is_whitespace() {
                    temp_iter.next();
                } else {
                    break;
                }
            }

            if let Some(&next_char) = temp_iter.peek() {
                if next_char.is_alphabetic() || next_char == '_' {
                    self.bump();
                    tokens.push(Token::LBrace);
                    let expr = self.consume_jsx_expression();
                    tokens.push(Token::Ident(expr));
                    tokens.push(Token::RBrace);
                    return;
                }
            }
        }

        tokens.push(Token::LBrace);
        self.bump();
    }

    fn collect_jsx_text_content(&mut self, tokens: &mut Vec<Token>) {
        let mut text = String::new();
        while let Some(&c) = self.peek() {
            if c == '<' || c == '{' {
                break;
            }
            text.push(c);
            self.bump();
        }
        if !text.trim().is_empty() {
            tokens.push(Token::Text(text));
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

    fn consume_jsx_expression(&mut self) -> String {
        let mut expr = String::new();
        let mut brace_count = 1;

        while let Some(c) = self.bump() {
            if c == '{' {
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    break;
                }
            }
            expr.push(c);
        }
        expr.trim().to_string()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_cont(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

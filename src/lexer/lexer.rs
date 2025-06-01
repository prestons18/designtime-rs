// Simple lexer

use crate::lexer::tokens::{Token, TokenKind};
use crate::lexer::line_tracker::LineTracker;

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
    line_tracker: LineTracker,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            peeked: None,
            line_tracker: LineTracker::new(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let next = if let Some(c) = self.peeked.take() {
            Some(c)
        } else {
            self.chars.next()
        };
        if let Some(c) = next {
            self.line_tracker.advance(c);
        }
        next
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let (line, column) = self.line_tracker.position();

        match self.next_char() {
            Some('<') => Token { kind: TokenKind::Lt, line, column },
            Some('>') => Token { kind: TokenKind::Gt, line, column },
            Some('/') => Token { kind: TokenKind::Slash, line, column },
            Some(c) if is_name_start_char(c) => {
                let mut name = String::new();
                name.push(c);
                while let Some(&next_c) = self.peek_char().as_ref() {
                    if is_name_char(next_c) {
                        name.push(next_c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                Token { kind: TokenKind::Name(name), line, column }
            }
            Some(c) => {
                // read as text until '<'
                let mut text = String::new();
                text.push(c);
                while let Some(&next_c) = self.peek_char().as_ref() {
                    if next_c == '<' {
                        break;
                    } else {
                        text.push(next_c);
                        self.next_char();
                    }
                }
                Token { kind: TokenKind::Text(text), line, column }
            }
            None => Token { kind: TokenKind::EOF, line, column },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char().as_ref() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
}

fn is_name_start_char(c: char) -> bool {
    c.is_alphabetic()
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

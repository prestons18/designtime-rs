// Simple lexer

use crate::Span;
use crate::lexer::line_tracker::LineTracker;
use crate::lexer::tokens::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
    line_tracker: LineTracker,
    in_tag: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            peeked: None,
            line_tracker: LineTracker::new(),
            in_tag: false,
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

        if self.in_tag {
            // We are inside a tag - read tag tokens
            match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    self.in_tag = false;
                    Token {
                        kind: TokenKind::Gt,
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some('=') => {
                    self.next_char();
                    Token {
                        kind: TokenKind::Eq,
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some('"') => {
                    self.next_char(); // consume opening quote
                    let mut value = String::new();
                    while let Some(ch) = self.peek_char() {
                        if ch == '"' {
                            self.next_char(); // consume closing quote
                            break;
                        }
                        value.push(ch);
                        self.next_char();
                    }
                    Token {
                        kind: TokenKind::StringLiteral(value),
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some('\'') => {
                    self.next_char(); // consume opening quote
                    let mut value = String::new();
                    while let Some(ch) = self.peek_char() {
                        if ch == '\'' {
                            self.next_char(); // consume closing quote
                            break;
                        }
                        value.push(ch);
                        self.next_char();
                    }
                    Token {
                        kind: TokenKind::StringLiteral(value),
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some('/') => {
                    self.next_char();
                    Token {
                        kind: TokenKind::Slash,
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some(c) if is_name_start_char(c) => {
                    let mut name = String::new();
                    while let Some(next_c) = self.peek_char() {
                        if is_name_char(next_c) {
                            name.push(next_c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    Token {
                        kind: TokenKind::Name(name),
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some(c) => {
                    // Unexpected char inside tag - consume it anyway
                    self.next_char();
                    Token {
                        kind: TokenKind::Unknown(c),
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                None => Token {
                    kind: TokenKind::EOF,
                    span: Span {
                        start_line: line,
                        start_column: column,
                        end_line: line,
                        end_column: column,
                    },
                },
            }
        } else {
            // Outside tag - should start with '<' or text
            match self.peek_char() {
                Some('<') => {
                    self.next_char();
                    self.in_tag = true;
                    Token {
                        kind: TokenKind::Lt,
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                Some(_) => {
                    // Read all text until next '<'
                    let mut text = String::new();
                    while let Some(next_c) = self.peek_char() {
                        if next_c == '<' {
                            break;
                        }
                        text.push(next_c);
                        self.next_char();
                    }
                    Token {
                        kind: TokenKind::InnerText(text),
                        span: Span {
                            start_line: line,
                            start_column: column,
                            end_line: line,
                            end_column: column,
                        },
                    }
                }
                None => Token {
                    kind: TokenKind::EOF,
                    span: Span {
                        start_line: line,
                        start_column: column,
                        end_line: line,
                        end_column: column,
                    },
                },
            }
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

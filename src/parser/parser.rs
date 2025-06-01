use crate::{lexer::{Token, TokenKind}, Lexer, ast::Node};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn bump(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        self.parse_element()
    }

    fn parse_element(&mut self) -> Result<Node, String> {
        if self.current.kind != TokenKind::Lt {
            return Err(format!("Expected <, got {:?}", self.current.kind));
        }
        self.bump();

        let tag_name = match &self.current.kind {
            TokenKind::Name(n) => n.clone(),
            _ => return Err(format!("Expected tag name, got {:?}", self.current.kind)),
        };
        self.bump();

        // Parse attributes before expecting '>'
        let class_names = self.parse_attributes()?;

        if self.current.kind != TokenKind::Gt {
            return Err(format!("Expected >, got {:?}", self.current.kind));
        }
        self.bump();

        let mut children = Vec::new();

        while self.current.kind != TokenKind::Lt {
            match &self.current.kind {
                TokenKind::InnerText(text) => {
                    children.push(Node::Text(text.clone()));
                    self.bump();
                }
                TokenKind::EOF => return Err("Unexpected EOF".into()),
                other => return Err(format!("Unexpected token: {:?}", other)),
            }
        }

        // Now at '<', check for closing tag
        self.bump();
        if self.current.kind != TokenKind::Slash {
            return Err(format!("Expected closing slash, got {:?}", self.current.kind));
        }
        self.bump();

        match &self.current.kind {
            TokenKind::Name(close_name) if *close_name == tag_name => (),
            TokenKind::Name(close_name) => {
                return Err(format!("Mismatched closing tag: expected </{}>, got </{}>", tag_name, close_name))
            }
            other => return Err(format!("Expected closing tag name, got {:?}", other)),
        }
        self.bump();

        if self.current.kind != TokenKind::Gt {
            return Err(format!("Expected > to close tag, got {:?}", self.current.kind));
        }
        self.bump();

        Ok(Node::Element { tag_name, class_names, children })
    }

    fn parse_attributes(&mut self) -> Result<Vec<String>, String> {
        let mut class_names = Vec::new();

        while self.current.kind != TokenKind::Gt {
            // Expect attribute name
            let attr_name = match &self.current.kind {
                TokenKind::Name(name) => name.clone(),
                _ => return Err(format!("Expected attribute name, got {:?}", self.current.kind)),
            };
            self.bump();

            // Expect '='
            if self.current.kind != TokenKind::Eq {
                return Err(format!("Expected '=', got {:?}", self.current.kind));
            }
            self.bump();

            // Expect attribute value as string literal (Text token here)
            let attr_value = match &self.current.kind {
                TokenKind::StringLiteral(value) => value.clone(),
                _ => return Err(format!("Expected string literal, got {:?}", self.current.kind)),
            };
            self.bump();

            if attr_name == "class" {
                // Split class string by whitespace into Vec<String>
                class_names = attr_value
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        Ok(class_names)
    }
}

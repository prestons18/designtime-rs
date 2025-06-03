use crate::{
    lexer::{Token, TokenKind}, 
    Lexer, 
    error::DesignTimeError
};
use designtime_ast::Node;
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

    pub fn parse(&mut self) -> Result<Node, DesignTimeError> {
        self.parse_element()
    }

    fn parse_element(&mut self) -> Result<Node, DesignTimeError> {
        if self.current.kind != TokenKind::Lt {
            return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected '<', got {:?}", self.current.kind),
                suggestion: Some("Make sure to start your element with '<'".to_string()),
            });
        }
        self.bump();

        let tag_name = match &self.current.kind {
            TokenKind::Name(n) => n.clone(),
            _ => return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected tag name, got {:?}", self.current.kind),
                suggestion: Some("Tag names should be valid identifiers (e.g., div, span, p)".to_string()),
            }),
        };
        self.bump();

        // Parse attributes before expecting '>'
        let (attributes, class_names) = self.parse_attributes()?;

        if self.current.kind != TokenKind::Gt {
            return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected '>', got {:?}", self.current.kind),
                suggestion: Some("Close the opening tag with '>'".to_string()),
            });
        }
        self.bump();

        let mut children = Vec::new();
        while self.current.kind != TokenKind::Lt {
            match &self.current.kind {
                TokenKind::InnerText(text) => {
                    children.push(Node::Text(text.clone()));
                    self.bump();
                }
                TokenKind::EOF => return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: "Unexpected end of file".to_string(),
                    suggestion: Some(format!("Expected closing tag </{}>", tag_name)),
                }),
                other => return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: format!("Unexpected token: {:?}", other),
                    suggestion: Some("Expected text content or closing tag".to_string()),
                }),
            }
        }

        // Now at '<', check for closing tag
        self.bump();
        if self.current.kind != TokenKind::Slash {
            return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected closing slash '/', got {:?}", self.current.kind),
                suggestion: Some("Closing tags should start with '</'".to_string()),
            });
        }
        self.bump();

        match &self.current.kind {
            TokenKind::Name(close_name) if *close_name == tag_name => (),
            TokenKind::Name(close_name) => {
                return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: format!("Mismatched closing tag: expected </{}>, got </{}>", tag_name, close_name),
                    suggestion: Some(format!("Change the closing tag to match the opening tag: </{}>", tag_name)),
                })
            }
            other => return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected closing tag name, got {:?}", other),
                suggestion: Some(format!("Expected the tag name '{}' after '</'", tag_name)),
            }),
        }
        self.bump();

        if self.current.kind != TokenKind::Gt {
            return Err(DesignTimeError::ParserError {
                span: self.current.span,
                message: format!("Expected '>' to close tag, got {:?}", self.current.kind),
                suggestion: Some("Close the closing tag with '>'".to_string()),
            });
        }
        self.bump();

        Ok(Node::Element { tag_name, attributes, class_names, children })
    }

    fn parse_attributes(&mut self) -> Result<(Vec<(String, String)>, Vec<String>), DesignTimeError> {
        let mut attributes = Vec::new();
        let mut class_names = Vec::new();

        while self.current.kind != TokenKind::Gt {
            // Expect attribute name
            let attr_name = match &self.current.kind {
                TokenKind::Name(name) => name.clone(),
                _ => return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: format!("Expected attribute name, got {:?}", self.current.kind),
                    suggestion: Some("Attribute names should be valid identifiers (e.g., class, id, style)".to_string()),
                }),
            };
            self.bump();

            // Expect '='
            if self.current.kind != TokenKind::Eq {
                return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: format!("Expected '=', got {:?}", self.current.kind),
                    suggestion: Some("Attributes should be in the format: name=\"value\"".to_string()),
                });
            }
            self.bump();

            // Expect attribute value as string literal
            let attr_value = match &self.current.kind {
                TokenKind::StringLiteral(value) => value.clone(),
                _ => return Err(DesignTimeError::ParserError {
                    span: self.current.span,
                    message: format!("Expected string literal, got {:?}", self.current.kind),
                    suggestion: Some("Attribute values should be quoted strings (e.g., \"value\")".to_string()),
                }),
            };
            self.bump();

            // Store all attributes in the attributes vector
            attributes.push((attr_name.clone(), attr_value.clone()));

            // Special handling for class attribute
            if attr_name == "class" {
                // Split class string by whitespace into Vec<String>
                class_names = attr_value
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        Ok((attributes, class_names))
    }
}
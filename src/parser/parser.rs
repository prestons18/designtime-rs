use crate::ast::{ASTNode, Function, ImportDecl, Node, PageDecl};
use crate::lexer::Token;
use designtime_jsx::{RenderNode, parse_render_block};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken {
        found: Token,
        expected: Vec<Token>,
        pos: usize,
    },
    MissingToken {
        expected: Token,
        pos: usize,
    },
    // TODO: Add more detailed errors & messages
}

impl SyntaxError {
    pub fn message(&self) -> String {
        match self {
            SyntaxError::UnexpectedToken {
                found, expected, ..
            } => {
                format!(
                    "Unexpected token {:?}, expected one of {:?}",
                    found, expected
                )
            }
            SyntaxError::MissingToken { expected, .. } => {
                format!("Expected token {:?}", expected)
            }
        }
    }

    pub fn span(&self) -> (usize, usize) {
        match self {
            SyntaxError::UnexpectedToken { pos, .. } => (*pos, *pos + 1),
            SyntaxError::MissingToken { pos, .. } => (*pos, *pos + 1),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF).clone()
    }

    fn bump(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn unexpected_token_err(&self, expected: Vec<Token>) -> SyntaxError {
        SyntaxError::UnexpectedToken {
            found: self.peek(),
            expected,
            pos: self.pos,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<ASTNode>, SyntaxError> {
        let mut items = Vec::new();
        while self.peek() != Token::EOF {
            match self.peek() {
                Token::Import => items.push(ASTNode::Import(self.parse_import()?)),
                Token::Page => items.push(ASTNode::Page(self.parse_page()?)),
                _ => return Err(self.unexpected_token_err(vec![Token::Import, Token::Page])),
            }
        }
        Ok(items)
    }

    fn parse_import(&mut self) -> Result<ImportDecl, SyntaxError> {
        if self.bump() != Token::Import {
            return Err(self.unexpected_token_err(vec![Token::Import]));
        }
        let mut names = Vec::new();
        if self.peek() == Token::LBrace {
            self.bump(); // consume '{'
            while self.peek() != Token::RBrace {
                match self.bump() {
                    Token::Ident(n) => names.push(n.clone()),
                    other => {
                        return Err(SyntaxError::UnexpectedToken {
                            found: other,
                            expected: vec![Token::Ident(String::new())],
                            pos: self.pos,
                        });
                    }
                }
                if self.peek() == Token::Comma {
                    self.bump();
                }
            }
            self.bump(); // consume '}'
        }
        if self.peek() != Token::From {
            return Err(SyntaxError::MissingToken {
                expected: Token::From,
                pos: self.pos,
            });
        }
        self.bump(); // consume 'from'
        match self.bump() {
            Token::StringLiteral(module) => Ok(ImportDecl {
                names,
                module: module.clone(),
            }),
            other => Err(SyntaxError::UnexpectedToken {
                found: other,
                expected: vec![Token::StringLiteral(String::new())],
                pos: self.pos,
            }),
        }
    }

    fn parse_page(&mut self) -> Result<PageDecl, SyntaxError> {
        if self.bump() != Token::Page {
            return Err(self.unexpected_token_err(vec![Token::Page]));
        }

        let name = match self.bump() {
            Token::Ident(n) => n.clone(),
            other => {
                return Err(SyntaxError::UnexpectedToken {
                    found: other,
                    expected: vec![Token::Ident(String::new())],
                    pos: self.pos,
                });
            }
        };

        self.expect_token(Token::LBrace, "Expected '{' after page name")?;

        let mut layout = None;
        let mut render_nodes = Vec::new();
        let mut functions = Vec::new();

        while self.peek() != Token::RBrace {
            match self.peek() {
                Token::Layout => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'layout'")?;
                    match self.bump() {
                        Token::Ident(l) => layout = Some(l.clone()),
                        other => {
                            return Err(SyntaxError::UnexpectedToken {
                                found: other,
                                expected: vec![Token::Ident(String::new())],
                                pos: self.pos,
                            });
                        }
                    }
                }
                Token::Render => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'render'")?;
                    self.expect_token(Token::LBrace, "Expected '{' to start render block")?;

                    let jsx_source = self.collect_until_closing_brace()?;

                    let root_node = parse_render_block(&jsx_source).map_err(|_| {
                        SyntaxError::UnexpectedToken {
                            found: Token::Text("Invalid JSX".to_string()),
                            expected: vec![],
                            pos: self.pos,
                        }
                    })?;

                    if let RenderNode::Element { children, .. } = root_node {
                        render_nodes = children.into_iter().map(Node::from).collect();
                    } else {
                        return Err(SyntaxError::UnexpectedToken {
                            found: Token::Text("JSX root not an element".to_string()),
                            expected: vec![],
                            pos: self.pos,
                        });
                    }
                }
                Token::Functions => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'functions'")?;
                    functions = self.parse_functions()?;
                }
                other => {
                    return Err(SyntaxError::UnexpectedToken {
                        found: other,
                        expected: vec![Token::Layout, Token::Render, Token::Functions],
                        pos: self.pos,
                    });
                }
            }
        }

        self.expect_token(Token::RBrace, "Expected '}' to close page")?;

        Ok(PageDecl {
            name,
            layout,
            render: render_nodes,
            functions,
        })
    }

    fn collect_until_closing_brace(&mut self) -> Result<String, SyntaxError> {
        let mut depth = 1;
        let mut jsx_tokens = Vec::new();
        let mut expression_depth = 0;

        self.bump(); // skip opening '{'

        while depth > 0 {
            let token = self.bump();
            match &token {
                Token::LBrace => {
                    expression_depth += 1;
                    jsx_tokens.push(token);
                }
                Token::RBrace => {
                    if expression_depth > 0 {
                        expression_depth -= 1;
                        jsx_tokens.push(token);
                    } else {
                        depth -= 1;
                        if depth > 0 {
                            jsx_tokens.push(token);
                        }
                    }
                }
                Token::EOF => {
                    return Err(SyntaxError::MissingToken {
                        expected: Token::RBrace,
                        pos: self.pos,
                    });
                }
                _ => jsx_tokens.push(token),
            }
        }

        Ok(Self::tokens_to_jsx_string(&jsx_tokens))
    }

    fn tokens_to_jsx_string(tokens: &[Token]) -> String {
        let mut result = String::new();
        let mut prev_token = None;

        for token in tokens {
            if let Some(prev) = prev_token {
                match (prev, token) {
                    (Token::LT, _)
                    | (Token::Slash, _)
                    | (Token::EQ, _)
                    | (_, Token::GT)
                    | (_, Token::SlashGT) => {}
                    (Token::Text(_), Token::LT) => result.push(' '),
                    (Token::StringLiteral(_), Token::Ident(_))
                    | (Token::Ident(_), Token::Ident(_)) => result.push(' '),
                    _ => result.push(' '),
                }
            }
            result.push_str(&token.to_string());
            prev_token = Some(token.clone());
        }

        result
    }

    fn expect_token(&mut self, expected: Token, msg: &str) -> Result<(), SyntaxError> {
        let token = self.bump();
        if token != expected {
            return Err(SyntaxError::UnexpectedToken {
                found: token,
                expected: vec![expected],
                pos: self.pos,
            });
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Token::Text(text) if text.trim().is_empty()) {
            self.bump();
        }
    }

    fn parse_functions(&mut self) -> Result<Vec<Function>, SyntaxError> {
        self.expect_token(Token::LBrace, "Expected '{' to start functions")?;
        let mut funcs = Vec::new();

        while self.peek() != Token::RBrace {
            self.skip_whitespace();

            let name = match self.bump() {
                Token::Ident(n) => n.clone(),
                other => {
                    return Err(SyntaxError::UnexpectedToken {
                        found: other,
                        expected: vec![Token::Ident(String::new())],
                        pos: self.pos,
                    });
                }
            };

            self.expect_token(Token::Colon, "Expected ':' after function name")?;
            self.skip_whitespace();

            let mut params = Vec::new();
            if self.peek() == Token::LParen {
                self.bump(); // consume '('
                while self.peek() != Token::RParen {
                    match self.bump() {
                        Token::Ident(param) => params.push(param.clone()),
                        other => {
                            return Err(SyntaxError::UnexpectedToken {
                                found: other,
                                expected: vec![Token::Ident(String::new())],
                                pos: self.pos,
                            });
                        }
                    }
                    if self.peek() == Token::Comma {
                        self.bump();
                    }
                }
                self.bump(); // consume ')'
            }

            self.skip_whitespace();
            self.expect_token(Token::Arrow, "Expected '=>' after parameters")?;
            self.skip_whitespace();

            let body = if self.peek() == Token::LBrace {
                self.collect_block_body()?
            } else {
                let expr = self.bump();
                expr.to_string()
            };

            funcs.push(Function {
                name,
                params,
                body: vec![body],
            });
            self.skip_whitespace();
        }

        self.expect_token(Token::RBrace, "Expected '}' to end functions")?;
        Ok(funcs)
    }

    fn collect_block_body(&mut self) -> Result<String, SyntaxError> {
        let mut depth = 1;
        let mut body_tokens = Vec::new();

        self.bump(); // consume '{'

        while depth > 0 {
            let token = self.bump();
            match &token {
                Token::LBrace => {
                    depth += 1;
                    body_tokens.push(token);
                }
                Token::RBrace => {
                    depth -= 1;
                    if depth > 0 {
                        body_tokens.push(token);
                    }
                }
                Token::EOF => {
                    return Err(SyntaxError::UnexpectedToken {
                        found: Token::EOF,
                        expected: vec![Token::RBrace],
                        pos: self.pos,
                    });
                }
                _ => body_tokens.push(token),
            }
        }

        Ok(Self::tokens_to_jsx_string(&body_tokens))
    }
}

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
        message: String,
    },
    MissingToken {
        expected: Token,
        pos: usize,
        message: String,
    },
}

impl SyntaxError {
    pub fn message(&self) -> String {
        match self {
            SyntaxError::UnexpectedToken { message, .. }
            | SyntaxError::MissingToken { message, .. } => message.clone(),
        }
    }

    pub fn span(&self) -> (usize, usize) {
        match self {
            SyntaxError::UnexpectedToken { pos, .. } | SyntaxError::MissingToken { pos, .. } => {
                (*pos, *pos + 1)
            }
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

    fn unexpected(&self, expected: Vec<Token>) -> SyntaxError {
        self.unexpected_with_msg(expected, "Unexpected token")
    }

    fn unexpected_with_msg(&self, expected: Vec<Token>, msg: &str) -> SyntaxError {
        let found = self.peek();
        let expected_str = expected
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<_>>()
            .join(", ");

        SyntaxError::UnexpectedToken {
            found: found.clone(),
            expected,
            pos: self.pos,
            message: format!("{msg}: `{found:?}`, expected one of: {expected_str}"),
        }
    }

    fn missing(&self, expected: Token, msg: &str) -> SyntaxError {
        SyntaxError::MissingToken {
            expected,
            pos: self.pos,
            message: msg.to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<ASTNode>, SyntaxError> {
        let mut items = Vec::new();
        while self.peek() != Token::EOF {
            match self.peek() {
                Token::Import => items.push(ASTNode::Import(self.parse_import()?)),
                Token::Page => items.push(ASTNode::Page(self.parse_page()?)),
                _ => return Err(self.unexpected(vec![Token::Import, Token::Page])),
            }
        }
        Ok(items)
    }

    fn parse_import(&mut self) -> Result<ImportDecl, SyntaxError> {
        self.expect_token(
            Token::Import,
            "Expected 'import' keyword at start of import declaration",
        )?;

        let mut names = Vec::new();
        if self.peek() == Token::LBrace {
            self.bump();
            while self.peek() != Token::RBrace {
                match self.bump() {
                    Token::Ident(n) => names.push(n),
                    _ => {
                        return Err(self.unexpected_with_msg(
                            vec![Token::Ident(String::new())],
                            "Expected identifier inside import braces",
                        ));
                    }
                }
                if self.peek() == Token::Comma {
                    self.bump();
                }
            }
            self.expect_token(Token::RBrace, "Expected '}' to close import specifiers")?;
        }

        self.expect_token(Token::From, "Expected 'from' after import specifiers")?;

        match self.bump() {
            Token::StringLiteral(module) => Ok(ImportDecl { names, module }),
            _ => Err(self.unexpected_with_msg(
                vec![Token::StringLiteral(String::new())],
                "Expected module string after 'from'",
            )),
        }
    }

    fn parse_page(&mut self) -> Result<PageDecl, SyntaxError> {
        self.expect_token(Token::Page, "Expected 'page' keyword")?;

        let name = match self.bump() {
            Token::Ident(n) => n,
            _ => {
                return Err(self
                    .unexpected_with_msg(vec![Token::Ident(String::new())], "Expected page name"));
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
                        Token::Ident(l) => layout = Some(l),
                        _ => {
                            return Err(self.unexpected_with_msg(
                                vec![Token::Ident(String::new())],
                                "Expected layout name",
                            ));
                        }
                    }
                }
                Token::Render => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'render'")?;
                    self.expect_token(Token::LBrace, "Expected '{' to start render block")?;

                    let jsx_source = self.collect_until_closing_brace()?;

                    let root_node = parse_render_block(&jsx_source).map_err(|_| {
                        self.unexpected_with_msg(vec![], "Failed to parse JSX in render block")
                    })?;

                    if let RenderNode::Element { children, .. } = root_node {
                        render_nodes = children.into_iter().map(Node::from).collect();
                    } else {
                        return Err(self.unexpected_with_msg(vec![], "JSX root must be an element"));
                    }
                }
                Token::Functions => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'functions'")?;
                    functions = self.parse_functions()?;
                }
                _ => {
                    return Err(self.unexpected(vec![
                        Token::Layout,
                        Token::Render,
                        Token::Functions,
                    ]));
                }
            }
        }

        self.expect_token(Token::RBrace, "Expected '}' to close page block")?;

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

        self.bump(); // consume initial {

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
                    return Err(self.missing(Token::RBrace, "Unterminated JSX block"));
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
            return Err(self.unexpected_with_msg(vec![expected], msg));
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Token::Text(text) if text.trim().is_empty()) {
            self.bump();
        }
    }

    fn parse_functions(&mut self) -> Result<Vec<Function>, SyntaxError> {
        self.expect_token(Token::LBrace, "Expected '{' to start functions block")?;
        let mut funcs = Vec::new();

        while self.peek() != Token::RBrace {
            self.skip_whitespace();

            let name = match self.bump() {
                Token::Ident(n) => n,
                _ => {
                    return Err(self.unexpected_with_msg(
                        vec![Token::Ident(String::new())],
                        "Expected function name",
                    ));
                }
            };

            self.expect_token(Token::Colon, "Expected ':' after function name")?;
            self.skip_whitespace();

            let mut params = Vec::new();
            if self.peek() == Token::LParen {
                self.bump(); // consume '('
                while self.peek() != Token::RParen {
                    match self.bump() {
                        Token::Ident(param) => params.push(param),
                        _ => {
                            return Err(self.unexpected_with_msg(
                                vec![Token::Ident(String::new())],
                                "Expected parameter name",
                            ));
                        }
                    }
                    if self.peek() == Token::Comma {
                        self.bump();
                    }
                }
                self.expect_token(Token::RParen, "Expected ')' to close parameter list")?;
            }

            self.skip_whitespace();
            self.expect_token(Token::Arrow, "Expected '=>' after function parameters")?;
            self.skip_whitespace();

            let body = if self.peek() == Token::LBrace {
                self.collect_block_body()?
            } else {
                self.bump().to_string()
            };

            funcs.push(Function {
                name,
                params,
                body: vec![body],
            });

            self.skip_whitespace();
        }

        self.expect_token(Token::RBrace, "Expected '}' to end functions block")?;
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
                    return Err(self.missing(Token::RBrace, "Unterminated function body"));
                }
                _ => body_tokens.push(token),
            }
        }

        Ok(Self::tokens_to_jsx_string(&body_tokens))
    }
}

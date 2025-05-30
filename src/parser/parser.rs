use crate::ast::{ASTNode, Function, ImportDecl, Node, PageDecl};
use crate::lexer::Token;
use designtime_jsx::{RenderNode, parse_render_block};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
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

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut items = Vec::new();
        while self.peek() != Token::EOF {
            match self.peek() {
                Token::Import => items.push(ASTNode::Import(self.parse_import())),
                Token::Page => items.push(ASTNode::Page(self.parse_page())),
                tok => panic!("Unexpected Token at top-level: {:?}", tok),
            }
        }
        items
    }

    fn parse_import(&mut self) -> ImportDecl {
        self.bump(); // Consume 'import'
        let mut names = Vec::new();
        if let Token::LBrace = self.peek() {
            self.bump(); // Consume '{'
            while self.peek() != Token::RBrace {
                if let Token::Ident(n) = self.bump() {
                    names.push(n.clone());
                    if self.peek() == Token::Comma {
                        self.bump(); // Consume ','
                    }
                } else {
                    panic!("Expected identifier in import list");
                }
            }
            self.bump(); // Consume '}'
        }
        if let Token::From = self.peek() {
            self.bump(); // Consume 'from'
        } else {
            panic!("Expected 'from' in import statement");
        }
        if let Token::StringLiteral(module) = self.bump() {
            ImportDecl {
                names,
                module: module.clone(),
            }
        } else {
            panic!("Expected module string after 'from'");
        }
    }

    fn parse_page(&mut self) -> PageDecl {
        self.bump(); // consume 'page'
        let name = if let Token::Ident(n) = self.bump() {
            n.clone()
        } else {
            panic!("Expected page name");
        };
        self.expect(Token::LBrace, "Expected '{' after page name");

        let mut layout = None;
        let mut render_nodes = Vec::new();
        let mut functions = Vec::new();

        while self.peek() != Token::RBrace {
            match self.peek() {
                Token::Layout => {
                    self.bump();
                    self.expect(Token::Colon, "Expected ':' after 'layout'");
                    if let Token::Ident(l) = self.bump() {
                        layout = Some(l.clone());
                    } else {
                        panic!("Expected layout name");
                    }
                }
                Token::Render => {
                    self.bump();
                    self.expect(Token::Colon, "Expected ':' after 'render'");
                    self.expect(Token::LBrace, "Expected '{' to start render block");

                    let jsx_source = self.collect_until_closing_brace();

                    let root_node =
                        parse_render_block(&jsx_source).expect("Failed to parse JSX block");

                    if let RenderNode::Element { children, .. } = root_node {
                        render_nodes = children.into_iter().map(Node::from).collect();
                    } else {
                        panic!("JSX root should be an element");
                    }
                }
                Token::Functions => {
                    self.bump();
                    self.expect(Token::Colon, "Expected ':' after 'functions'");
                    functions = self.parse_functions();
                }
                other => panic!("Unexpected token in page body: {:?}", other),
            }
        }

        self.expect(Token::RBrace, "Expected '}' to close page");

        PageDecl {
            name,
            layout,
            render: render_nodes,
            functions,
        }
    }

    fn collect_until_closing_brace(&mut self) -> String {
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
                Token::EOF => panic!("Unexpected EOF while parsing render JSX block"),
                _ => jsx_tokens.push(token),
            }
        }

        Self::tokens_to_jsx_string(&jsx_tokens)
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

    fn expect(&mut self, expected: Token, msg: &str) {
        let token = self.bump();
        if token != expected {
            panic!("{}: found {:?}", msg, token);
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Token::Text(text) if text.trim().is_empty()) {
            self.bump();
        }
    }

    fn parse_functions(&mut self) -> Vec<Function> {
        self.expect(Token::LBrace, "Expected '{' to start functions");
        let mut funcs = Vec::new();

        while self.peek() != Token::RBrace {
            self.skip_whitespace();

            let name = if let Token::Ident(n) = self.bump() {
                n.clone()
            } else {
                panic!("Expected function name");
            };

            self.expect(Token::Colon, "Expected ':' after function name");
            self.skip_whitespace();

            let mut params = Vec::new();
            if self.peek() == Token::LParen {
                self.bump();
                while self.peek() != Token::RParen {
                    if let Token::Ident(param) = self.bump() {
                        params.push(param.clone());
                        if self.peek() == Token::Comma {
                            self.bump();
                        }
                    } else {
                        panic!("Expected parameter name");
                    }
                }
                self.bump();
            }

            self.skip_whitespace();
            self.expect(Token::Arrow, "Expected '=>' after parameters");
            self.skip_whitespace();

            let body = if self.peek() == Token::LBrace {
                self.bump();
                let mut body_tokens = Vec::new();
                let mut brace_count = 1;

                while brace_count > 0 {
                    match self.bump() {
                        Token::LBrace => {
                            brace_count += 1;
                            body_tokens.push(Token::LBrace);
                        }
                        Token::RBrace => {
                            brace_count -= 1;
                            if brace_count > 0 {
                                body_tokens.push(Token::RBrace);
                            }
                        }
                        token => body_tokens.push(token),
                    }
                }

                vec![Self::tokens_to_js_string(&body_tokens)]
            } else {
                let mut expr_tokens = Vec::new();
                while !matches!(self.peek(), Token::Comma | Token::RBrace) {
                    expr_tokens.push(self.bump());
                }
                vec![format!(
                    "return {}",
                    Self::tokens_to_js_string(&expr_tokens)
                )]
            };

            funcs.push(Function { name, params, body });

            if self.peek() == Token::Comma {
                self.bump();
            }
            self.skip_whitespace();
        }

        self.expect(Token::RBrace, "Expected '}' to close functions block");
        funcs
    }

    fn tokens_to_js_string(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

use crate::lexer::Token;
use crate::ast::{ASTNode, ImportDecl, PageDecl, Node, Function, Attribute};

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
        if self.bump() != Token::LBrace {
            panic!("Expected '{{' after page name");
        }
    
        let mut layout = None;
        let mut render = Vec::new();
        let mut functions = Vec::new();
    
        while self.peek() != Token::RBrace {
            match self.peek() {
                Token::Layout => {
                    self.bump(); // Consume 'layout'
                    if self.bump() != Token::Colon {
                        panic!("Expected ':' after 'layout'");
                    }
                    if let Token::Ident(l) = self.bump() {
                        layout = Some(l.clone());
                    } else {
                        panic!("Expected layout name");
                    }
                }
                Token::Render => {
                    self.bump(); // Consume 'render'
                    if self.bump() != Token::Colon {
                        panic!("Expected ':' after 'render'");
                    }
                    if self.bump() != Token::LBrace {
                        panic!("Expected '{{' to start render block");
                    }
                    render = self.parse_nodes_until_closing_brace();
                }
                Token::Functions => {
                    self.bump(); // Consume 'functions'
                    if self.bump() != Token::Colon {
                        panic!("Expected ':' after 'functions'");
                    }
                    functions = self.parse_functions();
                }
                tok => panic!("Unexpected Token in page body: {:?}", tok),
            }
        }
        self.bump(); // Consume closing '}'
    
        PageDecl {
            name,
            layout,
            render,
            functions,
        }
    }
    
    // This parses multiple nodes until it sees the matching closing '}' for render block
    fn parse_nodes_until_closing_brace(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
    
        while self.peek() != Token::RBrace && self.peek() != Token::EOF {
            nodes.push(self.parse_single_node());
        }
    
        if self.bump() != Token::RBrace {
            panic!("Expected '}}' to close render block");
        }
        nodes
    }
    
    fn parse_single_node(&mut self) -> Node {
        match self.peek() {
            Token::LT => {
                self.bump(); // consume '<'
    
                if self.peek() == Token::Slash {
                    // closing tag found without matching opening tag, error
                    panic!("Unexpected closing tag");
                }
    
                let name = if let Token::Ident(n) = self.bump() {
                    n.clone()
                } else {
                    panic!("Expected tag name after '<'");
                };
    
                let attrs = self.parse_attributes();
    
                if self.peek() == Token::Slash {
                    // self-closing tag like <Checkbox checked={true} />
                    self.bump(); // consume '/'
                    self.expect(Token::GT, "Expected '>' after '/' in self-closing tag");
                    Node::Element {
                        name,
                        attrs,
                        children: vec![],
                    }
                } else {
                    // Normal tag with children: consume '>'
                    self.expect(Token::GT, "Expected '>' after tag name");
    
                    let mut children = Vec::new();
    
                    // parse children nodes until matching closing tag
                    loop {
                        match self.peek() {
                            Token::LT => {
                                // Check if next tokens are closing tag
                                if self.pos + 1 < self.tokens.len() {
                                    if let Token::Slash = self.tokens[self.pos + 1] {
                                        // consume closing tag
                                        self.bump(); // consume '<'
                                        self.bump(); // consume '/'
                                        let close_name = if let Token::Ident(n) = self.bump() {
                                            n.clone()
                                        } else {
                                            panic!("Expected closing tag name");
                                        };
                                        if close_name != name {
                                            panic!(
                                                "Mismatched closing tag: expected </{}>, found </{}>",
                                                name, close_name
                                            );
                                        }
                                        self.expect(Token::GT, "Expected '>' after closing tag");
                                        break; // closing tag matched, exit loop
                                    }
                                }
                                // Otherwise parse child node recursively
                                children.push(self.parse_single_node());
                            }
    
                            Token::Text(text) => {
                                let trimmed = text.trim();
                                if !trimmed.is_empty() {
                                    children.push(Node::Text(trimmed.to_string()));
                                }
                                self.bump();
                            }
    
                            Token::LBrace => {
                                // Expression block inside JSX
                                self.bump(); // consume '{'
                                let mut expr_tokens = vec![];
                                while self.peek() != Token::RBrace && self.peek() != Token::EOF {
                                    expr_tokens.push(format!("{:?}", self.bump()));
                                }
                                self.expect(Token::RBrace, "Expected '}' to close expression block");
                                children.push(Node::Text(expr_tokens.join(" ")));
                            }
    
                            other => {
                                println!("Skipping unexpected token in children: {:?}", other);
                                self.bump();
                            }
                        }
                    }
    
                    Node::Element {
                        name,
                        attrs,
                        children,
                    }
                }
            }
    
            Token::Text(text) => {
                let trimmed = text.trim();
                self.bump();
                if !trimmed.is_empty() {
                    Node::Text(trimmed.to_string())
                } else {
                    // Empty text node, parse next node instead
                    self.parse_single_node()
                }
            }
    
            Token::LBrace => {
                // Expression block alone as a node
                self.bump(); // consume '{'
                let mut expr_tokens = vec![];
                while self.peek() != Token::RBrace && self.peek() != Token::EOF {
                    expr_tokens.push(format!("{:?}", self.bump()));
                }
                self.expect(Token::RBrace, "Expected '}' to close expression block");
                Node::Text(expr_tokens.join(" "))
            }
    
            other => {
                panic!("Unexpected token while parsing node: {:?}", other);
            }
        }
    }
    
    fn parse_attributes(&mut self) -> Vec<Attribute> {
        let mut attrs = Vec::new();
        loop {
            match self.peek() {
                Token::GT | Token::Slash => break,
                Token::Ident(attr_name) => {
                    let name = attr_name.clone();
                    self.bump(); // Consume attribute name
                    let value = if self.peek() == Token::EQ {
                        self.bump(); // Consume '='
                        match self.bump() {
                            Token::StringLiteral(val) => val.clone(),
                            Token::LBrace => {
                                // Handle {expression} values
                                let val = if let Token::Ident(val) = self.bump() {
                                    val.clone()
                                } else {
                                    panic!("Expected expression inside '{{' '}}'");
                                };
                                if self.bump() != Token::RBrace {
                                    panic!("Expected '}}' after expression");
                                }
                                val
                            }
                            Token::Ident(val) => val.clone(),
                            _ => panic!("Expected value after '=' in attribute"),
                        }
                    } else if self.peek() == Token::Colon {
                        self.bump(); // Consume ':'
                        match self.bump() {
                            Token::StringLiteral(val) => val.clone(),
                            Token::Ident(val) => val.clone(),
                            _ => panic!("Expected value after ':' in attribute"),
                        }
                    } else if self.peek() == Token::LBrace {
                        self.bump(); // Consume '{'
                        let val = if let Token::Ident(val) = self.bump() {
                            val.clone()
                        } else {
                            panic!("Expected expression inside '{{' '}}'");
                        };
                        if self.bump() != Token::RBrace {
                            panic!("Expected '}}' after expression");
                        }
                        val
                    } else {
                        "true".to_string()
                    };
                    attrs.push(Attribute::new(name, value));
                }
                _ => {
                    self.bump(); // Skip unexpected Tokens
                }
            }
        }
        attrs
    }
    
    fn expect(&mut self, expected: Token, msg: &str) {
        let token = self.bump();
        if token != expected {
            panic!("{}: found {:?}", msg, token);
        }
    }
    

    fn skip_whitespace(&mut self) {
        while let Token::Text(text) = self.peek() {
            if text.trim().is_empty() {
                self.bump();
            } else {
                break;
            }
        }
    }


    fn parse_functions(&mut self) -> Vec<Function> {
        if self.bump() != Token::LBrace {
            panic!("Expected '{{' to start functions");
        }

        let mut funcs = Vec::new();
        self.skip_whitespace();

        while self.peek() != Token::RBrace {
            let name = if let Token::Ident(n) = self.bump() {
                n.clone()
            } else {
                panic!("Expected function name");
            };
            if self.bump() != Token::Colon {
                panic!("Expected ':' after function name");
            }
            self.skip_whitespace();

            let mut params = Vec::new();
            if self.peek() == Token::LParen {
                self.bump(); // Consume '('
                while self.peek() != Token::RParen {
                    if let Token::Ident(param) = self.bump() {
                        params.push(param.clone());
                        if self.peek() == Token::Comma {
                            self.bump(); // Consume ','
                        }
                    } else {
                        panic!("Expected parameter name");
                    }
                }
                self.bump(); // Consume ')'
            }

            self.skip_whitespace();

            if self.bump() != Token::Arrow {
                panic!("Expected '=>' after parameters");
            }

            self.skip_whitespace();

            let body = if self.peek() == Token::LBrace {
                self.bump(); // Consume '{'
                let mut body_tokens = Vec::new();
                let mut brace_count = 1;
                while brace_count > 0 {
                    match self.bump() {
                        Token::LBrace => {
                            brace_count += 1;
                            body_tokens.push("{".to_string());
                        }
                        Token::RBrace => {
                            brace_count -= 1;
                            if brace_count > 0 {
                                body_tokens.push("}".to_string());
                            }
                        }
                        Token::SemiColon => {
                            body_tokens.push(";".to_string());
                        }
                        token => {
                            body_tokens.push(token.to_string());
                        }
                    }
                }
                vec![body_tokens.join(" ").trim().to_string()]
            } else {
                let mut expr = String::new();
                while self.peek() != Token::Comma && self.peek() != Token::RBrace {
                    expr.push_str(&self.bump().to_string());
                    expr.push(' ');
                }
                vec![format!("return {}", expr.trim())]
            };

            funcs.push(Function {
                name,
                params,
                body,
            });

            if self.peek() == Token::Comma {
                self.bump(); // Consume ','
            }
            self.skip_whitespace();
        }
        self.bump(); // Consume '}'
        funcs
    }
}
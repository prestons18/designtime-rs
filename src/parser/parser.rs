use crate::ast::{ASTNode, Attribute, Function, ImportDecl, Node, PageDecl};
use crate::lexer::Token;
use crate::error::LangError;
// SourceMap is used indirectly through swc_ecma_parser
use miette::{NamedSource, SourceSpan};
use swc_common::SourceMap;
use std::sync::Arc;
use swc_ecma_ast::{JSXElement, JSXElementChild, JSXAttrName, JSXAttrValue, JSXExpr, Expr};
use swc_ecma_parser::EsSyntax;
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    source: Arc<NamedSource<String>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: Arc<NamedSource<String>>) -> Self {
        Self { 
            tokens, 
            pos: 0,
            source,
        }
    }

    fn peek(&self) -> Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF).clone();
        token
    }

    fn bump(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn current_span(&self) -> SourceSpan {
            SourceSpan::new(self.pos.try_into().expect("Position cannot be negative"), 1)
    }

    fn unexpected_token_error(&self, token: String) -> LangError {
        LangError::UnexpectedToken {
            src: Arc::clone(&self.source),
            span: self.current_span(),
            token,
        }
    }

    fn runtime_error(&self, message: String, help: Option<String>) -> LangError {
        LangError::RuntimeError {
            src: Arc::clone(&self.source),
            span: self.current_span(),
            message,
            help,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<ASTNode>, LangError> {
        let mut items = Vec::new();
        while self.peek() != Token::EOF {
            match self.peek() {
                Token::Import => {
                    items.push(ASTNode::Import(self.parse_import()?));
                }
                Token::Page => {
                    items.push(ASTNode::Page(self.parse_page()?));
                }
                _ => {
                    return Err(self.unexpected_token_error(format!("{:?}", self.peek())));
                }
            }
        }
        Ok(items)
    }

    fn parse_import(&mut self) -> Result<ImportDecl, LangError> {
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
                        return Err(self.runtime_error(
                            "Expected identifier inside import braces".to_string(),
                            Some("Check your import syntax".to_string()),
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
            _ => Err(self.runtime_error(
                "Expected module string after 'from'".to_string(),
                Some("Import statements should end with a quoted string".to_string()),
            )),
        }
    }

    fn parse_page(&mut self) -> Result<PageDecl, LangError> {
        self.expect_token(Token::Page, "Expected 'page' keyword")?;

        let name = match self.bump() {
            Token::Ident(n) => n,
            _ => {
                return Err(self.runtime_error(
                    "Expected page name".to_string(),
                    Some("Page declarations need a name identifier".to_string()),
                ));
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
                            return Err(self.runtime_error(
                                "Expected layout name".to_string(),
                                Some("Layout should be followed by an identifier".to_string()),
                            ));
                        }
                    }
                }
                Token::Render => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'render'")?;
                    self.expect_token(Token::LBrace, "Expected '{' to start render block")?;
                
                    let jsx_source = self.collect_jsx_block()?;
                    render_nodes = self.parse_jsx_with_swc(&jsx_source)?;
                }                
                Token::Functions => {
                    self.bump();
                    self.expect_token(Token::Colon, "Expected ':' after 'functions'")?;
                    functions = self.parse_functions()?;
                }
                _ => {
                    return Err(self.unexpected_token_error(format!("{:?}", self.peek())));
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

    fn parse_jsx_with_swc(&self, jsx_source: &str) -> Result<Vec<Node>, LangError> {
        // First check if the source is empty or just whitespace
        if jsx_source.trim().is_empty() {
            return Ok(Vec::new());
        }

        // Check if the source needs to be wrapped in a fragment
        let wrapped_source = if jsx_source.trim_start().starts_with('<') {
            jsx_source.to_string()
        } else {
            // If it's not a valid JSX element, wrap it in a fragment
            format!("<>{}</>", jsx_source)
        };

        let cm = Arc::new(SourceMap::default());
        let fm = cm.new_source_file(
            swc_common::FileName::Anon.into(),
            wrapped_source.clone(),
        );

        let lexer = Lexer::new(
            Syntax::Es(EsSyntax {
                jsx: true,
                ..Default::default()
            }),
            swc_ecma_ast::EsVersion::Es2022,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = SwcParser::new_from(lexer);

        match parser.parse_expr() {
            Ok(expr) => match &*expr {
                Expr::JSXElement(jsx_elem) => self.convert_jsx_element_to_nodes(jsx_elem),
                Expr::JSXFragment(frag) => {
                    // Handle JSX fragments
                    let mut nodes = Vec::new();
                    for child in &frag.children {
                        match child {
                            swc_ecma_ast::JSXElementChild::JSXElement(elem) => {
                                nodes.extend(self.convert_jsx_element_to_nodes(elem)?);
                            }
                            swc_ecma_ast::JSXElementChild::JSXText(text) => {
                                let content = text.value.to_string();
                                if !content.trim().is_empty() {
                                    nodes.push(Node::Text(content));
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(nodes)
                }
                _ => Err(self.runtime_error(
                    "Expected JSX element or fragment".to_string(),
                    Some("JSX parsing failed - check your JSX syntax".to_string()),
                )),
            },
            Err(e) => Err(self.runtime_error(
                format!("Failed to parse JSX: {:?}", e),
                Some("Check your JSX syntax for errors".to_string()),
            )),
        }
    }

    fn convert_jsx_element_to_nodes(&self, jsx_elem: &JSXElement) -> Result<Vec<Node>, LangError> {
        let mut nodes = Vec::new();

        // If this is a fragment (empty tag name), just process children
        let tag_name = self.jsx_element_name_to_string(&jsx_elem.opening.name);
        
        if tag_name.is_empty() {
            // Fragment - process children directly
            for child in &jsx_elem.children {
                nodes.extend(self.convert_jsx_child_to_nodes(child)?);
            }
        } else {
            // Regular element
            let attrs = jsx_elem.opening.attrs.iter()
                .filter_map(|attr| self.convert_jsx_attr(attr))
                .collect();

            let mut children = Vec::new();
            for child in &jsx_elem.children {
                children.extend(self.convert_jsx_child_to_nodes(child)?);
            }

            nodes.push(Node::Element {
                name: tag_name,
                attrs,
                children,
            });
        }

        Ok(nodes)
    }

    fn convert_jsx_child_to_nodes(&self, child: &JSXElementChild) -> Result<Vec<Node>, LangError> {
        match child {
            JSXElementChild::JSXText(text) => {
                let content = text.value.to_string();
                if content.trim().is_empty() {
                    Ok(vec![])
                } else {
                    Ok(vec![Node::Text(content)])
                }
            }
            JSXElementChild::JSXElement(elem) => {
                self.convert_jsx_element_to_nodes(elem)
            }
            JSXElementChild::JSXExprContainer(expr_container) => {
                if let JSXExpr::Expr(expr) = &expr_container.expr {
                    // Convert the expression back to string for now
                    // You might want to create a proper expression AST node
                    Ok(vec![Node::Expr(format!("{:?}", expr))])
                } else {
                    Ok(vec![])
                }
            }
            JSXElementChild::JSXFragment(fragment) => {
                let mut nodes = Vec::new();
                for child in &fragment.children {
                    nodes.extend(self.convert_jsx_child_to_nodes(child)?);
                }
                Ok(nodes)
            }
            JSXElementChild::JSXSpreadChild(_) => {
                // Handle spread children if needed
                Ok(vec![])
            }
        }
    }

    fn jsx_element_name_to_string(&self, name: &swc_ecma_ast::JSXElementName) -> String {
        match name {
            swc_ecma_ast::JSXElementName::Ident(ident) => ident.sym.to_string(),
            swc_ecma_ast::JSXElementName::JSXMemberExpr(member) => {
                format!("{}.{}", 
                    self.jsx_object_to_string(&member.obj),
                    member.prop.sym
                )
            }
            swc_ecma_ast::JSXElementName::JSXNamespacedName(ns) => {
                format!("{}:{}", ns.ns.sym, ns.name.sym)
            }
        }
    }

    fn jsx_object_to_string(&self, obj: &swc_ecma_ast::JSXObject) -> String {
        match obj {
            swc_ecma_ast::JSXObject::Ident(ident) => ident.sym.to_string(),
            swc_ecma_ast::JSXObject::JSXMemberExpr(member) => {
                format!("{}.{}", 
                    self.jsx_object_to_string(&member.obj),
                    member.prop.sym
                )
            }
        }
    }

    fn convert_jsx_attr(&self, attr: &swc_ecma_ast::JSXAttrOrSpread) -> Option<Attribute> {
        match attr {
            swc_ecma_ast::JSXAttrOrSpread::JSXAttr(jsx_attr) => {
                let name = match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => ident.sym.to_string(),
                    JSXAttrName::JSXNamespacedName(ns) => {
                        format!("{}:{}", ns.ns.sym, ns.name.sym)
                    }
                };

                let value = jsx_attr.value.as_ref().map(|v| match v {
                    JSXAttrValue::Lit(lit) => match lit {
                        swc_ecma_ast::Lit::Str(s) => s.value.to_string(),
                        swc_ecma_ast::Lit::Num(n) => n.value.to_string(),
                        swc_ecma_ast::Lit::Bool(b) => b.value.to_string(),
                        _ => format!("{:?}", lit),
                    },
                    JSXAttrValue::JSXExprContainer(expr) => {
                        format!("{{{:?}}}", expr.expr)
                    }
                    JSXAttrValue::JSXElement(elem) => {
                        format!("<{}>", self.jsx_element_name_to_string(&elem.opening.name))
                    }
                    JSXAttrValue::JSXFragment(_) => "<>".to_string(),
                }).unwrap_or_default();

                Some(Attribute { name, value })
            }
            swc_ecma_ast::JSXAttrOrSpread::SpreadElement(_) => None,
        }
    }

    fn collect_jsx_block(&mut self) -> Result<String, LangError> {
        let mut depth = 1;
        let mut jsx_tokens = Vec::new();

        while depth > 0 && self.peek() != Token::EOF {
            let token = self.bump();
            match &token {
                Token::LBrace => {
                    depth += 1;
                    jsx_tokens.push(token);
                }
                Token::RBrace => {
                    depth -= 1;
                    if depth > 0 {
                        jsx_tokens.push(token);
                    }
                }
                Token::EOF => {
                    return Err(self.runtime_error(
                        "Unterminated JSX block".to_string(),
                        Some("JSX blocks must be properly closed with '}'".to_string()),
                    ));
                }
                _ => jsx_tokens.push(token),
            }
        }

        Ok(self.reconstruct_jsx(&jsx_tokens))
    }

    fn reconstruct_jsx(&self, tokens: &[Token]) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Token::LT => {
                    result.push('<');
                    i += 1;

                    // Handle possible closing tag
                    if i < tokens.len() && tokens[i] == Token::Slash {
                        result.push('/');
                        i += 1;
                    }

                    // Element name
                    if i < tokens.len() {
                        if let Token::Ident(name) = &tokens[i] {
                            result.push_str(name);
                            i += 1;
                        }
                    }

                    // Attributes
                    while i < tokens.len() && !matches!(tokens[i], Token::GT | Token::SlashGT) {
                        match &tokens[i] {
                            Token::Ident(attr) => {
                                result.push(' ');
                                result.push_str(attr);
                                i += 1;

                                // Attribute value
                                if i < tokens.len() && tokens[i] == Token::EQ {
                                    result.push('=');
                                    i += 1;

                                    if i < tokens.len() {
                                        match &tokens[i] {
                                            Token::StringLiteral(s) => {
                                                result.push('"');
                                                result.push_str(s);
                                                result.push('"');
                                                i += 1;
                                            }
                                            Token::LBrace => {
                                                result.push('{');
                                                i += 1;
                                                let (expr, new_i) =
                                                    self.reconstruct_expression(tokens, i);
                                                result.push_str(&expr);
                                                result.push('}');
                                                i = new_i;
                                            }
                                            _ => {
                                                result.push_str(&format!("{:?}", tokens[i]));
                                                i += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }

                    // Closing bracket
                    if i < tokens.len() {
                        match tokens[i] {
                            Token::GT => result.push('>'),
                            Token::SlashGT => result.push_str("/>"),
                            _ => {}
                        }
                        i += 1;
                    }
                }
                Token::Text(text) => {
                    result.push_str(text);
                    i += 1;
                }
                Token::LBrace => {
                    result.push('{');
                    i += 1;
                    let (expr, new_i) = self.reconstruct_expression(tokens, i);
                    result.push_str(&expr);
                    result.push('}');
                    i = new_i;
                }
                _ => {
                    i += 1;
                }
            }
        }

        result
    }

    fn reconstruct_expression(&self, tokens: &[Token], start: usize) -> (String, usize) {
        let mut result = String::new();
        let mut i = start;
        let mut brace_depth = 1;

        while i < tokens.len() && brace_depth > 0 {
            match &tokens[i] {
                Token::LBrace => {
                    result.push('{');
                    brace_depth += 1;
                }
                Token::RBrace => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        result.push('}');
                    }
                }
                Token::Ident(id) => result.push_str(id),
                Token::Number(n) => result.push_str(&n.to_string()),
                Token::Plus => result.push('+'),
                Token::Minus => result.push('-'),
                Token::Star => result.push('*'),
                Token::Slash => result.push('/'),
                Token::Text(t) => result.push_str(t),
                Token::StringLiteral(s) => {
                    result.push('"');
                    result.push_str(s);
                    result.push('"');
                }
                _ => result.push_str(&format!("{:?}", tokens[i])),
            }
            i += 1;
        }

        (result, i)
    }

    fn expect_token(&mut self, expected: Token, msg: &str) -> Result<(), LangError> {
        let token = self.bump();
        if std::mem::discriminant(&token) != std::mem::discriminant(&expected) {
            return Err(self.runtime_error(
                format!("{}: expected {:?}, found {:?}", msg, expected, token),
                Some("Check your syntax".to_string()),
            ));
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Token::Text(text) if text.trim().is_empty()) {
            self.bump();
        }
    }

    fn parse_functions(&mut self) -> Result<Vec<Function>, LangError> {
        self.expect_token(Token::LBrace, "Expected '{' to start functions block")?;
        let mut funcs = Vec::new();

        while self.peek() != Token::RBrace {
            self.skip_whitespace();

            let name = match self.bump() {
                Token::Ident(n) => n,
                _ => {
                    return Err(self.runtime_error(
                        "Expected function name".to_string(),
                        Some("Function definitions need a name identifier".to_string()),
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
                            return Err(self.runtime_error(
                                "Expected parameter name".to_string(),
                                Some("Function parameters must be identifiers".to_string()),
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
                self.collect_function_body()?
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

    fn collect_function_body(&mut self) -> Result<String, LangError> {
        let mut depth = 1;
        let mut body_tokens = Vec::new();

        self.bump(); // consume '{'

        while depth > 0 && self.peek() != Token::EOF {
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
                    return Err(self.runtime_error(
                        "Unterminated function body".to_string(),
                        Some("Function bodies must be properly closed with '}'".to_string()),
                    ));
                }
                _ => body_tokens.push(token),
            }
        }

        // Convert function body tokens to proper JavaScript string
        Ok(self.tokens_to_js_string(&body_tokens))
    }

    fn tokens_to_js_string(&self, tokens: &[Token]) -> String {
        let mut result = String::new();

        for (i, token) in tokens.iter().enumerate() {
            if i > 0 {
                // Add spacing between tokens as needed
                match (&tokens[i - 1], token) {
                    (Token::Ident(_), Token::Ident(_)) => result.push(' '),
                    (Token::Ident(_), Token::EQ) => result.push(' '),
                    (Token::EQ, _) => result.push(' '),
                    (Token::Plus, _) => result.push(' '),
                    (_, Token::Plus) => result.push(' '),
                    _ => {}
                }
            }

            match token {
                Token::Ident(s) => result.push_str(s),
                Token::Number(n) => result.push_str(&n.to_string()),
                Token::StringLiteral(s) => {
                    result.push('"');
                    result.push_str(s);
                    result.push('"');
                }
                Token::Plus => result.push('+'),
                Token::EQ => result.push('='),
                Token::SemiColon => result.push(';'),
                Token::LBrace => result.push('{'),
                Token::RBrace => result.push('}'),
                Token::LParen => result.push('('),
                Token::RParen => result.push(')'),
                _ => result.push_str(&format!("{:?}", token)),
            }
        }

        result
    }
}
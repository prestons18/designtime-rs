// Todo: improve this AST

use designtime_jsx::RenderNode;
use serde::{Deserialize, Serialize};

/// Represents an import declaration in the source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDecl {
    pub names: Vec<String>,
    pub module: String,
}

impl ImportDecl {
    /// Creates a new import declaration
    pub fn new<S: Into<String>>(names: Vec<S>, module: S) -> Self {
        Self {
            names: names.into_iter().map(|s| s.into()).collect(),
            module: module.into(),
        }
    }
}

/// Represents a page declaration with its render content and functions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageDecl {
    pub name: String,
    pub layout: Option<String>,
    pub render: Vec<Node>,
    pub functions: Vec<Function>,
}

impl PageDecl {
    /// Creates a new page declaration
    pub fn new<S: Into<String>>(
        name: S,
        layout: Option<S>,
        render: Vec<Node>,
        functions: Vec<Function>,
    ) -> Self {
        Self {
            name: name.into(),
            layout: layout.map(Into::into),
            render,
            functions,
        }
    }
}

/// Represents an HTML/JSX attribute
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    /// Creates a new attribute
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Represents a node in the AST that can be rendered
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Text(String),
    Element {
        name: String,
        attrs: Vec<Attribute>,
        children: Vec<Node>,
    },
    Fragment(Vec<Node>),
    Expr(String),
}

impl Node {
    /// Creates a new text node
    pub fn text<S: Into<String>>(content: S) -> Self {
        Node::Text(content.into())
    }

    /// Creates a new element node
    pub fn element<S: Into<String>>(name: S, attrs: Vec<Attribute>, children: Vec<Node>) -> Self {
        Node::Element {
            name: name.into(),
            attrs,
            children,
        }
    }

    /// Creates a new fragment node
    pub fn fragment(nodes: Vec<Node>) -> Self {
        Node::Fragment(nodes)
    }

    /// Visits this node and all its children with the given visitor
    pub fn visit<V: NodeVisitor>(&self, visitor: &mut V) {
        self.visit_with_parent(visitor, None);
    }

    fn visit_with_parent<V: NodeVisitor>(&self, visitor: &mut V, parent: Option<&Node>) {
        visitor.visit_node(self, parent);

        if let Node::Element { children, .. } | Node::Fragment(children) = self {
            for child in children {
                child.visit_with_parent(visitor, Some(self));
            }
        }
    }
}

/// Represents a function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<String>,
}

impl From<RenderNode> for Node {
    fn from(rn: RenderNode) -> Self {
        match rn {
            RenderNode::Element {
                tag_name,
                attrs,
                children,
            } => Node::Element {
                name: tag_name,
                attrs: attrs
                    .into_iter()
                    .map(|(n, v)| Attribute { name: n, value: v })
                    .collect(),
                children: children.into_iter().map(Node::from).collect(),
            },
            RenderNode::Text(t) => Node::Text(t),
            RenderNode::Expr(e) => Node::Expr(e),
        }
    }
}

impl Function {
    /// Creates a new function
    pub fn new<S: Into<String>>(name: S, params: Vec<S>, body: Vec<S>) -> Self {
        Self {
            name: name.into(),
            params: params.into_iter().map(|s| s.into()).collect(),
            body: body.into_iter().map(|s| s.into()).collect(),
        }
    }
}

/// Represents a node in the abstract syntax tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "node_type")]
pub enum ASTNode {
    Import(ImportDecl),
    Page(PageDecl),
}

impl ASTNode {
    /// Creates a new import node
    pub fn import<S: Into<String>>(names: Vec<S>, module: S) -> Self {
        ASTNode::Import(ImportDecl::new(names, module))
    }

    /// Creates a new page node
    pub fn page<S: Into<String>>(
        name: S,
        layout: Option<S>,
        render: Vec<Node>,
        functions: Vec<Function>,
    ) -> Self {
        ASTNode::Page(PageDecl::new(name, layout, render, functions))
    }
}

/// Trait for visiting AST nodes
pub trait NodeVisitor {
    /// Called for each node in the AST during traversal
    fn visit_node(&mut self, node: &Node, parent: Option<&Node>);
}

/// A simple visitor that prints the AST structure
pub struct PrintVisitor {
    indent: usize,
}

impl PrintVisitor {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn print_indent(&self) {
        print!("{:indent$}", "", indent = self.indent * 2);
    }
}

impl NodeVisitor for PrintVisitor {
    fn visit_node(&mut self, node: &Node, _parent: Option<&Node>) {
        self.print_indent();
        match node {
            Node::Text(text) => println!("Text: {:?}", text),
            Node::Expr(expr) => println!("Expr: {{{}}}", expr),
            Node::Element { name, attrs, .. } => {
                println!("Element: {} ({} attrs)", name, attrs.len());
            }
            Node::Fragment(nodes) => {
                println!("Fragment ({} nodes)", nodes.len());
            }
        }
        self.indent += 1;
    }
}

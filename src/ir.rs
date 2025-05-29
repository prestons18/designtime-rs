// most of this file is defining types, this is just a working example
// i'll use this to make a real IR

use crate::ast::{ASTNode, Attribute, Function, ImportDecl, Node, PageDecl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the entire program in IR form
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub imports: Vec<ImportDef>,
    pub pages: Vec<PageDef>,
    pub metadata: ProgramMetadata,
}

/// Metadata about the program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub version: String,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
}

/// Normalized import definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportDef {
    pub module: String,
    pub imports: Vec<ImportItem>,
    pub import_type: ImportType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportType {
    Named,     // import { Component } from "module"
    Default,   // import Component from "module"
    Namespace, // import * as Module from "module"
}

/// Normalized page definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageDef {
    pub name: String,
    pub layout: Option<String>,
    pub component_tree: ComponentTree,
    pub functions: Vec<FunctionDef>,
    pub state: Vec<StateDef>,
    pub hooks: Vec<HookDef>,
    pub metadata: PageMetadata,
}

/// Page-specific metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMetadata {
    pub route: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub requires_auth: bool,
}

/// Represents the component hierarchy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentTree {
    pub root: ComponentNode,
    pub component_registry: HashMap<String, ComponentDef>,
}

/// A single component in the tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentNode {
    pub id: String,
    pub component_type: ComponentType,
    pub props: Vec<PropDef>,
    pub children: Vec<ComponentNode>,
    pub event_handlers: Vec<EventHandler>,
    pub conditional: Option<ConditionalRender>,
    pub loop_info: Option<LoopRender>,
}

/// Type of component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    Html(String),           // div, span, etc.
    Expr(String),           // JSX expression
    Custom(String),         // Checkbox, Button, etc.
    Text(String),           // Text content
    Expression(Expression), // {variable} or {expression}
    Fragment,               // React.Fragment equivalent
}

/// Component definition for reusable components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentDef {
    pub name: String,
    pub props: Vec<PropDef>,
    pub body: ComponentNode,
    pub is_external: bool,
}

/// Property/attribute definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropDef {
    pub name: String,
    pub value: PropValue,
    pub is_spread: bool,
}

/// Value of a property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Expression(Expression),
    Null,
}

/// Represents expressions in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    pub expr_type: ExpressionType,
    pub raw: String,
    pub dependencies: Vec<String>, // Variables this expression depends on
}

/// Types of expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionType {
    Variable,        // {variable}
    FunctionCall,    // {myFunction()}
    BinaryOperation, // {x + y}
    MemberAccess,    // {obj.prop}
    Conditional,     // {condition ? a : b}
    Literal,         // {42}, {"string"}, {true}
    Complex,         // Complex multi-line expressions
}

/// Event handler definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventHandler {
    pub event: String, // onClick, onChange, etc.
    pub handler_type: HandlerType,
}

/// Types of event handlers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandlerType {
    FunctionRef(String),         // onClick={handleClick}
    InlineFunction(FunctionDef), // onClick={() => {}}
    Expression(Expression),      // onClick={e => setValue(e.target.value)}
}

/// Conditional rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalRender {
    pub condition: Expression,
    pub then_branch: Option<Box<ComponentNode>>,
    pub else_branch: Option<Box<ComponentNode>>,
}

/// Loop/iteration rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopRender {
    pub iterable: Expression,
    pub item_name: String,
    pub index_name: Option<String>,
    pub key_expr: Option<Expression>,
}

/// Function definition in IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeDef>,
    pub body: FunctionBody,
    pub function_type: FunctionType,
    pub is_async: bool,
    pub is_pure: bool,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<TypeDef>,
    pub default_value: Option<Expression>,
    pub is_rest: bool,
}

/// Function body representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionBody {
    pub statements: Vec<Statement>,
    pub locals: Vec<Variable>,
}

/// Types of functions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionType {
    EventHandler, // Functions that handle events
    Utility,      // Helper functions
    Hook,         // React-like hooks
    Computed,     // Computed values
    Effect,       // Side effects
}

/// Statement in function body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Assignment {
        target: String,
        value: Expression,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Loop {
        init: Option<Expression>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    Break,
    Continue,
    Block(Vec<Statement>),
}

/// State definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDef {
    pub name: String,
    pub initial_value: Expression,
    pub state_type: Option<TypeDef>,
    pub is_reactive: bool,
}

/// Hook definition (React-like)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookDef {
    pub hook_type: HookType,
    pub name: String,
    pub dependencies: Vec<String>,
    pub config: HashMap<String, String>,
}

/// Types of hooks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HookType {
    State,          // useState equivalent
    Effect,         // useEffect equivalent
    Memo,           // useMemo equivalent
    Callback,       // useCallback equivalent
    Custom(String), // Custom hooks
}

/// Variable definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: Option<TypeDef>,
    pub is_mutable: bool,
    pub scope: VariableScope,
}

/// Variable scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariableScope {
    Global,
    Page,
    Function,
    Block,
}

/// Type definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeDef {
    String,
    Number,
    Boolean,
    Array(Box<TypeDef>),
    Object(HashMap<String, TypeDef>),
    Function {
        params: Vec<TypeDef>,
        return_type: Box<TypeDef>,
    },
    Union(Vec<TypeDef>),
    Custom(String),
    Any,
}

impl Program {
    /// Creates a Program from AST nodes
    pub fn from_ast(ast_nodes: Vec<ASTNode>) -> Self {
        let mut program = Program {
            imports: Vec::new(),
            pages: Vec::new(),
            metadata: ProgramMetadata {
                version: "1.0.0".to_string(),
                dependencies: Vec::new(),
                exports: Vec::new(),
            },
        };

        for node in ast_nodes {
            match node {
                ASTNode::Import(import_decl) => {
                    program.imports.push(ImportDef::from_ast(import_decl));
                }
                ASTNode::Page(page_decl) => {
                    program.pages.push(PageDef::from_ast(page_decl));
                }
            }
        }

        // Extract dependencies from imports
        program.metadata.dependencies = program
            .imports
            .iter()
            .map(|imp| imp.module.clone())
            .collect();

        // Extract exports from pages
        program.metadata.exports = program.pages.iter().map(|page| page.name.clone()).collect();

        program
    }

    /// Validates the IR program
    pub fn validate(&self) -> Result<(), String> {
        // Check for duplicate page names
        let mut page_names = std::collections::HashSet::new();
        for page in &self.pages {
            if !page_names.insert(&page.name) {
                return Err(format!("Duplicate page name: {}", page.name));
            }
        }

        // Validate each page
        for page in &self.pages {
            page.validate()?;
        }

        Ok(())
    }

    /// Optimizes the IR program
    pub fn optimize(&mut self) {
        for page in &mut self.pages {
            page.optimize();
        }
    }
}

impl ImportDef {
    fn from_ast(import_decl: ImportDecl) -> Self {
        let imports = import_decl
            .names
            .into_iter()
            .map(|name| ImportItem { name, alias: None })
            .collect();

        ImportDef {
            module: import_decl.module,
            imports,
            import_type: ImportType::Named,
        }
    }
}

impl PageDef {
    fn from_ast(page_decl: PageDecl) -> Self {
        let component_tree = ComponentTree::from_ast_nodes(page_decl.render);
        let functions = page_decl
            .functions
            .into_iter()
            .map(FunctionDef::from_ast)
            .collect();

        PageDef {
            name: page_decl.name,
            layout: page_decl.layout,
            component_tree,
            functions,
            state: Vec::new(),
            hooks: Vec::new(),
            metadata: PageMetadata {
                route: None,
                title: None,
                description: None,
                requires_auth: false,
            },
        }
    }

    fn validate(&self) -> Result<(), String> {
        // Validate function names are unique
        let mut func_names = std::collections::HashSet::new();
        for func in &self.functions {
            if !func_names.insert(&func.name) {
                return Err(format!(
                    "Duplicate function name in page {}: {}",
                    self.name, func.name
                ));
            }
        }

        // Validate component tree
        self.component_tree.validate()
    }

    fn optimize(&mut self) {
        // Remove unused functions
        // Inline simple expressions
        // Optimize component tree
        self.component_tree.optimize();
    }
}

impl ComponentTree {
    fn from_ast_nodes(nodes: Vec<Node>) -> Self {
        let mut id_counter = 0;
        let root = if nodes.len() == 1 {
            ComponentNode::from_ast_node(&nodes[0], &mut id_counter)
        } else {
            // Multiple root nodes, wrap in fragment
            ComponentNode {
                id: format!("node_{}", id_counter),
                component_type: ComponentType::Fragment,
                props: Vec::new(),
                children: nodes
                    .into_iter()
                    .map(|node| {
                        id_counter += 1;
                        ComponentNode::from_ast_node(&node, &mut id_counter)
                    })
                    .collect(),
                event_handlers: Vec::new(),
                conditional: None,
                loop_info: None,
            }
        };

        ComponentTree {
            root,
            component_registry: HashMap::new(),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.root.validate()
    }

    fn optimize(&mut self) {
        self.root.optimize();
    }
}

impl ComponentNode {
    fn from_ast_node(node: &Node, id_counter: &mut i32) -> Self {
        *id_counter += 1;
        let id = format!("node_{}", id_counter);

        match node {
            Node::Text(text) => ComponentNode {
                id,
                component_type: ComponentType::Text(text.clone()),
                props: Vec::new(),
                children: Vec::new(),
                event_handlers: Vec::new(),
                conditional: None,
                loop_info: None,
            },

            Node::Expr(expr) => ComponentNode {
                id,
                component_type: ComponentType::Expr(expr.clone()),
                props: Vec::new(),
                children: Vec::new(),
                event_handlers: Vec::new(),
                conditional: None,
                loop_info: None,
            },

            Node::Element {
                name,
                attrs,
                children,
            } => {
                let props = attrs.iter().map(PropDef::from_ast_attribute).collect();

                let child_nodes = children
                    .iter()
                    .map(|child| ComponentNode::from_ast_node(child, id_counter))
                    .collect();

                let component_type = if is_html_element(name) {
                    ComponentType::Html(name.clone())
                } else {
                    ComponentType::Custom(name.clone())
                };

                ComponentNode {
                    id,
                    component_type,
                    props,
                    children: child_nodes,
                    event_handlers: Vec::new(),
                    conditional: None,
                    loop_info: None,
                }
            }

            Node::Fragment(children) => {
                let child_nodes = children
                    .iter()
                    .map(|child| ComponentNode::from_ast_node(child, id_counter))
                    .collect();

                ComponentNode {
                    id,
                    component_type: ComponentType::Fragment,
                    props: Vec::new(),
                    children: child_nodes,
                    event_handlers: Vec::new(),
                    conditional: None,
                    loop_info: None,
                }
            }
        }
    }

    fn validate(&self) -> Result<(), String> {
        // Validate children recursively
        for child in &self.children {
            child.validate()?;
        }
        Ok(())
    }

    fn optimize(&mut self) {
        // Remove empty text nodes
        self.children.retain(|child| {
            !matches!(child.component_type, ComponentType::Text(ref text) if text.trim().is_empty())
        });

        // Optimize children recursively
        for child in &mut self.children {
            child.optimize();
        }
    }
}

impl PropDef {
    fn from_ast_attribute(attr: &Attribute) -> Self {
        let value = if attr.value == "true" {
            PropValue::Boolean(true)
        } else if attr.value == "false" {
            PropValue::Boolean(false)
        } else if let Ok(num) = attr.value.parse::<f64>() {
            PropValue::Number(num)
        } else if attr.value.starts_with('{') && attr.value.ends_with('}') {
            // Expression
            let expr_content = attr.value[1..attr.value.len() - 1].to_string();
            PropValue::Expression(Expression {
                expr_type: ExpressionType::Variable, // Simplified
                raw: expr_content,
                dependencies: Vec::new(),
            })
        } else {
            PropValue::String(attr.value.clone())
        };

        PropDef {
            name: attr.name.clone(),
            value,
            is_spread: false,
        }
    }
}

impl FunctionDef {
    fn from_ast(func: Function) -> Self {
        let parameters = func
            .params
            .into_iter()
            .map(|param| Parameter {
                name: param,
                param_type: None,
                default_value: None,
                is_rest: false,
            })
            .collect();

        let statements = func
            .body
            .into_iter()
            .map(|body_line| {
                // Simple parsing of function body
                if body_line.trim().starts_with("return") {
                    let expr_part = body_line.trim().strip_prefix("return").unwrap_or("").trim();
                    if expr_part.is_empty() {
                        Statement::Return(None)
                    } else {
                        Statement::Return(Some(Expression {
                            expr_type: ExpressionType::Complex,
                            raw: expr_part.to_string(),
                            dependencies: Vec::new(),
                        }))
                    }
                } else {
                    Statement::Expression(Expression {
                        expr_type: ExpressionType::Complex,
                        raw: body_line,
                        dependencies: Vec::new(),
                    })
                }
            })
            .collect();

        FunctionDef {
            name: func.name,
            parameters,
            return_type: None,
            body: FunctionBody {
                statements,
                locals: Vec::new(),
            },
            function_type: FunctionType::Utility,
            is_async: false,
            is_pure: false,
        }
    }
}

// I am going to make a JSX-like crate that handles default HTML elements & attributes.
// It will also handle custom DesignTime components.
// Also will be useful for the LSP.
fn is_html_element(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "div"
            | "span"
            | "p"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "a"
            | "img"
            | "button"
            | "input"
            | "form"
            | "label"
            | "select"
            | "option"
            | "textarea"
            | "ul"
            | "ol"
            | "li"
            | "table"
            | "tr"
            | "td"
            | "th"
            | "thead"
            | "tbody"
            | "tfoot"
            | "section"
            | "article"
            | "header"
            | "footer"
            | "nav"
            | "main"
            | "aside"
            | "figure"
            | "figcaption"
    )
}

use serde::{Deserialize, Serialize};

/// Represents an expression in the IR.
/// Could be a variable, literal, binary op, call, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// A literal value like number, string, boolean
    Literal(Literal),

    /// A variable reference by name
    Variable(String),

    /// A binary operation like `a + b`
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOpKind,
        right: Box<Expr>,
    },

    /// A function call, with function name and args
    Call { function: String, args: Vec<Expr> },

    /// A property access, e.g. `obj.prop`
    PropertyAccess { object: Box<Expr>, property: String },
}

/// Literal values for expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// Kinds of binary operators supported
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    // Add more as needed
}

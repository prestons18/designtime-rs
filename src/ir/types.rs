/// Core types for DesignTime IR

/// A simple enum representing the primitive types in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// No value / void
    Void,
    /// Boolean type
    Bool,
    /// Integer type (signed 32-bit)
    Int,
    /// Floating point type (64-bit)
    Float,
    /// String type
    String,
    /// Custom user-defined type by name
    Custom(String),
    /// Function type with parameter types and return type
    Function { params: Vec<Type>, ret: Box<Type> },
    /// Reference type to another type (like pointers or refs)
    Ref(Box<Type>),
}

/// A parameter for a function or method
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

impl Param {
    pub fn new<S: Into<String>>(name: S, ty: Type) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

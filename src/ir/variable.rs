use super::types::Type;

/// Represents a variable in the IR
#[derive(Debug, Clone)]
pub struct Variable {
    /// The variable name
    pub name: String,
    /// The variable type
    pub ty: Type,
    /// Whether the variable is mutable
    pub mutable: bool,
}

impl Variable {
    pub fn new<S: Into<String>>(name: S, ty: Type, mutable: bool) -> Self {
        Self {
            name: name.into(),
            ty,
            mutable,
        }
    }
}

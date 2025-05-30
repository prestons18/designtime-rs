use super::types::Type;

/// Represents a single property (prop) with a name, type, and optional default value.
#[derive(Debug, Clone)]
pub struct Prop {
    pub name: String,
    pub ty: Type,
    pub default_value: Option<String>,
}

impl Prop {
    pub fn new<S: Into<String>>(name: S, ty: Type, default_value: Option<String>) -> Self {
        Self {
            name: name.into(),
            ty,
            default_value,
        }
    }
}

use serde::{Deserialize, Serialize};

/// Represents a hook call in the IR, with its name and arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    /// The name of the hook, e.g. "useState", "useEffect"
    pub name: String,
    /// Arguments passed to the hook as strings (could later be an expression AST)
    pub args: Vec<String>,
}

impl Hook {
    pub fn new<S: Into<String>>(name: S, args: Vec<S>) -> Self {
        Self {
            name: name.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

use super::function::IrFunction;
use super::ir_node::IrNode;

/// Represents an IR Page with render nodes and functions.
#[derive(Debug, Clone)]
pub struct IrPage {
    pub name: String,
    pub layout: Option<String>,
    pub render: Vec<IrNode>,
    pub functions: Vec<IrFunction>,
}

impl IrPage {
    pub fn new<S: Into<String>>(
        name: S,
        layout: Option<S>,
        render: Vec<IrNode>,
        functions: Vec<IrFunction>,
    ) -> Self {
        Self {
            name: name.into(),
            layout: layout.map(Into::into),
            render,
            functions,
        }
    }
}

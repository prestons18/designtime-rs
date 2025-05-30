use super::function::IrFunction;
use super::ir_node::IrNode;

/// Represents a component declaration in the IR
#[derive(Debug, Clone)]
pub struct ComponentDecl {
    pub name: String,
    pub props: Vec<String>,
    pub render: Vec<IrNode>,
    pub functions: Vec<IrFunction>,
}

impl ComponentDecl {
    pub fn new<S: Into<String>>(
        name: S,
        props: Vec<S>,
        render: Vec<IrNode>,
        functions: Vec<IrFunction>,
    ) -> Self {
        Self {
            name: name.into(),
            props: props.into_iter().map(Into::into).collect(),
            render,
            functions,
        }
    }
}

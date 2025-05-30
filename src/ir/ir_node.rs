#[derive(Debug, Clone)]
pub enum IrNode {
    Text(String),
    Element {
        name: String,
        attrs: Vec<(String, String)>,
        children: Vec<IrNode>,
    },
    Fragment(Vec<IrNode>),
    Expr(String),
}

impl IrNode {
    pub fn text<S: Into<String>>(content: S) -> Self {
        IrNode::Text(content.into())
    }

    pub fn element<S: Into<String>>(
        name: S,
        attrs: Vec<(String, String)>,
        children: Vec<IrNode>,
    ) -> Self {
        IrNode::Element {
            name: name.into(),
            attrs,
            children,
        }
    }

    pub fn fragment(children: Vec<IrNode>) -> Self {
        IrNode::Fragment(children)
    }
}

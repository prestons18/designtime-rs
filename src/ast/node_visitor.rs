use super::Node;

/// Trait for visiting AST nodes
pub trait NodeVisitor {
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

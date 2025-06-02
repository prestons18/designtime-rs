use crate::{
    engine::Runtime,
    parser::Parser,
    lexer::Lexer,
    dominate::prelude::DomNode, 
    RuntimeError
};

pub struct RenderLib {
    runtime: Runtime,
}

impl RenderLib {
    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }

    /// Process source code and return DOM nodes or error
    pub fn process_source(&mut self, source: &str) -> Result<Vec<DomNode>, RuntimeError> {
        // Parse the source into AST nodes
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let nodes = parser.parse().map_err(|e| RuntimeError {
            message: e.to_string(),
            source: Some(Box::new(e)),
            span: None,
        })?;

        // Process the nodes through the runtime
        // Wrap the single node in a Vec since process_nodes expects Vec<Node>
        let (dom_nodes, _css) = self.runtime.process_nodes(vec![nodes])?;
        Ok(dom_nodes)
    }
}

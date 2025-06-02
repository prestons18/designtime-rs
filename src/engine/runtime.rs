use crate::{ast::Node, workspace::WorkspaceConfig};
use crate::dominate::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
    pub span: Option<crate::error::Span>,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(
                f,
                "{} (at line {}, column {})",
                self.message, span.start_line, span.start_column
            )
        } else {
            write!(f, "Runtime error: {}", self.message)
        }
    }
}

impl std::error::Error for RuntimeError {}

impl Clone for RuntimeError {
    fn clone(&self) -> Self {
        RuntimeError {
            message: self.message.clone(),
            source: self.source.as_ref().map(|e| {
                let msg = e.to_string();
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)) as Box<dyn std::error::Error + Send + Sync>
            }),
            span: self.span,
        }
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

pub struct Runtime {
    pub workspace: WorkspaceConfig,
    pub last_error: Option<RuntimeError>,
}

impl Runtime {
    pub fn new(workspace: WorkspaceConfig) -> Self {
        Self {
            workspace,
            last_error: None,
        }
    }

    /// Accepts parsed AST nodes, transforms them into DomNodes, and retrieves CSS.
    pub fn process_nodes(&mut self, nodes: Vec<Node>) -> Result<(Vec<DomNode>, String), RuntimeError> {
        // Call dominate's transform
        let dom_nodes: Vec<DomNode> = transform(nodes);

        // Get generated CSS from StyleMan
        let css = get_css();

        Ok((dom_nodes, css))
    }

    pub fn run(&mut self, nodes: Vec<Node>) {
        match self.process_nodes(nodes) {
            Ok((dom_nodes, css)) => {
                println!("DomNodes:\n{:#?}", dom_nodes);
                println!("CSS:\n{}", css);
            }
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                if let Some(source) = &e.source {
                    eprintln!("Caused by: {}", source);
                }
            }
        }
    }
}

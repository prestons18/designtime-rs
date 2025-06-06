use designtime_ast::Node;
use crate::workspace::WorkspaceConfig;
// use dominate::prelude::*;
use std::fmt;
use watchman::{generate_html_file, start_server};

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

    /// Accepts parsed AST nodes and generates an HTML file with a live preview
    pub fn process_nodes(&mut self, nodes: Vec<Node>) -> Result<(), RuntimeError> {
        // Generate HTML file with live preview
        generate_html_file(&nodes).map_err(|e| RuntimeError {
            message: "Failed to generate HTML file".to_string(),
            source: Some(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error + Send + Sync>),
            span: None,
        })
    }

    pub async fn run(&mut self, nodes: Vec<Node>) {
        match self.process_nodes(nodes) {
            Ok(()) => {
                println!("✅ HTML file generated successfully");
                println!("📡 Starting preview server...");
                start_server().await;
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

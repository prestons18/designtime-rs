//! A Rust library for rendering design-time ASTs to various output formats.
//! This crate provides a high-level API for transforming and rendering AST nodes.

use std::fmt;
use thiserror::Error;
use designtime_ast::Node;
use render_core::transform as core_transform;

/// Errors that can occur during rendering
#[derive(Error, Debug)]
pub enum RenderError {
    /// Error parsing input JSON
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    
    /// Other rendering errors
    #[error("Rendering error: {0}")]
    RenderingError(String),
}

/// Renders a JSON string containing AST nodes into the target format
///
/// # Arguments
/// * `json` - A JSON string containing an array of AST nodes
///
/// # Returns
/// Returns a `Result` containing the rendered output as a string, or a `RenderError`
/// if rendering fails.
pub fn render_json(json: &str) -> Result<String, RenderError> {
    let nodes: Vec<Node> = serde_json::from_str(json)?;
    let dom_nodes = core_transform(nodes);
    
    // For now, we'll just return the debug representation of the DOM nodes
    // In a real implementation, you might want to serialize this to a specific format
    Ok(format!("{:?}", dom_nodes))
}

/// Renders a slice of AST nodes into the target format
///
/// # Arguments
/// * `nodes` - A slice of AST nodes to render
///
/// # Returns
/// Returns the rendered output as a string
pub fn render_nodes(nodes: &[Node]) -> String {
    let dom_nodes = core_transform(nodes.to_vec());
    format!("{:?}", dom_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use designtime_ast::Node;
    use serde_json::json;

    #[test]
    fn test_render_json() {
        let json = json!([
            {
                "Element": {
                    "tag_name": "div",
                    "attributes": [["class", "container"]],
                    "class_names": ["container"],
                    "children": [
                        {
                            "Text": "Hello, world!"
                        }
                    ]
                }
            }
        ]).to_string();

        let result = render_json(&json);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("div"));
        assert!(output.contains("container"));
        assert!(output.contains("Hello, world!"));
    }
}

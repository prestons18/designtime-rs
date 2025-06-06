use dominate::dom::{DomNode};
use designtime_ast::Node;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("DOM operation failed")]
    DomError,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Core rendering function that transforms a list of AST nodes into DOM nodes
pub fn render_nodes(nodes: Vec<Node>) -> Vec<DomNode> {
    nodes.into_iter().map(transform_node).collect::<Result<Vec<DomNode>, RenderError>>().unwrap()
}

/// Transform a single AST node into a DOM node
pub fn transform_node(node: Node) -> Result<DomNode, RenderError> {
    match node {
        Node::Element { tag_name, attributes, children, .. } => {
            let mut builder = DomNode::element(&tag_name)
                .attributes(attributes);
            
            for child in children {
                builder = builder.child(transform_node(child)?);
            }
            Ok(builder.build())
        }
        Node::Text(content) => Ok(DomNode::text(&content)),
        _ => Err(RenderError::DomError),
    }
}

/// Transform a list of AST nodes into a list of DOM nodes
pub fn transform(nodes: Vec<Node>) -> Vec<DomNode> {
    nodes.into_iter().map(transform_node).collect::<Result<Vec<DomNode>, RenderError>>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use designtime_ast::Node;

    #[test]
    fn test_transform_text_node() {
        let node = Node::Text("Hello".to_string());
        let dom_node = transform_node(node).unwrap();
        
        if let DomNode::Text(text) = dom_node {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected text node");
        }
    }
    
    #[test]
    fn test_transform_element_node() {
        let node = Node::Element {
            tag_name: "div".to_string(),
            attributes: vec![("class".to_string(), "container".to_string())],
            children: vec![Node::Text("Hello".to_string())],
            class_names: vec!["container".to_string()],
        };
        
        let dom_node = transform_node(node).unwrap();
        
        if let DomNode::Element { tag, attributes, children, .. } = dom_node {
            
        let class_attr = attributes.iter()
        .find(|(k, _)| k == "class")
        .map(|(_, v)| v);
    
            assert_eq!(tag, "div");
            assert_eq!(class_attr, Some(&"container".to_string()));
            assert_eq!(children.len(), 1);
            
            if let DomNode::Text(text) = &children[0] {
                assert_eq!(text, "Hello");
            } else {
                panic!("Expected text child");
            }
        } else {
            panic!("Expected element node");
        }
    }
}

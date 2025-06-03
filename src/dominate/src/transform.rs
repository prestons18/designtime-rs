use crate::dom::DomNode;
use designtime_ast::Node;
use styleman::StyleMan;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref STYLEMAN: Mutex<StyleMan> = Mutex::new(StyleMan::new());
}

/// Transform AST nodes into DomNodes while accumulating CSS classes.
pub fn transform(nodes: Vec<Node>) -> Vec<DomNode> {
    // Reset StyleMan before transform
    {
        let mut styleman = STYLEMAN.lock().expect("StyleMan poisoned");
        *styleman = StyleMan::new();
    }

    nodes.into_iter()
        .enumerate()
        .map(|(idx, node)| transform_node(node, format!("node{}", idx)))
        .collect()
}

fn transform_node(node: Node, key: String) -> DomNode {
    match node {
        Node::Text(text) => {
            if let Some(expr) = extract_expression(&text) {
                DomNode::expression(&expr)
            } else {
                DomNode::text(&text)
            }
        }
        Node::Element { tag_name, attributes, class_names, children } => {
            // Remove "class" attr from attributes vector
            let mut filtered_attrs = attributes.clone();
            filtered_attrs.retain(|(name, _)| name != "class");

            if !class_names.is_empty() {
                let mut styleman = STYLEMAN.lock().expect("StyleMan poisoned");
                styleman.add_classes(class_names.clone());
            }

            DomNode::element(&tag_name)
                .key(&key)
                .attributes(filtered_attrs)
                .class_names(class_names)
                .children(children.into_iter()
                    .enumerate()
                    .map(|(i, c)| transform_node(c, format!("{}-{}", key, i)))
                    .collect()
                )
                .build()
        }
    }
}

fn extract_expression(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
        Some(trimmed[2..trimmed.len() - 2].trim().to_string())
    } else {
        None
    }
}

pub fn get_css() -> String {
    let styleman = STYLEMAN.lock().expect("StyleMan poisoned");
    styleman.generate_css()
}

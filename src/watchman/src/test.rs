

#[cfg(test)]
mod tests {
    use dominate::dom::DomNode;
    use designtime_ast::Node;
    use render_core::transform;
    
    /// Creates a simple "Hello world!" div element as an AST node and transforms it to DOM
    /// 
    /// Returns a vector containing the transformed DOM nodes
    pub fn create_hello_world() -> Vec<DomNode> {
        let hello_world_node = Node::Element {
            tag_name: "div".to_string(),
            attributes: vec![
                ("class".to_string(), "greeting".to_string()),
                ("id".to_string(), "hello".to_string()),
            ],
            children: vec![
                Node::Text("Hello world!".to_string())
            ],
            class_names: vec!["greeting".to_string()],
        };
        
        transform(vec![hello_world_node])
    }
    
    /// Creates a more complex nested structure for testing
    pub fn create_complex_greeting() -> Vec<DomNode> {
        let complex_node = Node::Element {
            tag_name: "div".to_string(),
            attributes: vec![
                ("class".to_string(), "container".to_string()),
            ],
            children: vec![
                Node::Element {
                    tag_name: "h1".to_string(),
                    attributes: vec![
                        ("class".to_string(), "title".to_string()),
                    ],
                    children: vec![
                        Node::Text("Welcome to Watchman!".to_string())
                    ],
                    class_names: vec!["title".to_string()],
                },
                Node::Element {
                    tag_name: "p".to_string(),
                    attributes: vec![
                        ("class".to_string(), "description".to_string()),
                    ],
                    children: vec![
                        Node::Text("This is a demonstration of render-core in action.".to_string())
                    ],
                    class_names: vec!["description".to_string()],
                }
            ],
            class_names: vec!["container".to_string()],
        };
        
        transform(vec![complex_node])
    }
    
    /// Creates AST nodes suitable for web rendering
    pub fn create_web_demo_ast() -> Vec<Node> {
        vec![
            Node::Element {
                tag_name: "div".to_string(),
                attributes: vec![
                    ("class".to_string(), "app".to_string()),
                ],
                children: vec![
                    Node::Element {
                        tag_name: "header".to_string(),
                        attributes: vec![
                            ("class".to_string(), "header".to_string()),
                        ],
                        children: vec![
                            Node::Element {
                                tag_name: "h1".to_string(),
                                attributes: vec![],
                                children: vec![
                                    Node::Text("🔍 Watchman Live Demo".to_string())
                                ],
                                class_names: vec![],
                            }
                        ],
                        class_names: vec!["header".to_string()],
                    },
                    Node::Element {
                        tag_name: "main".to_string(),
                        attributes: vec![
                            ("class".to_string(), "main-content".to_string()),
                        ],
                        children: vec![
                            Node::Element {
                                tag_name: "div".to_string(),
                                attributes: vec![
                                    ("class".to_string(), "greeting".to_string()),
                                    ("id".to_string(), "hello".to_string()),
                                ],
                                children: vec![
                                    Node::Text("Hello world!".to_string())
                                ],
                                class_names: vec!["greeting".to_string()],
                            },
                            Node::Element {
                                tag_name: "p".to_string(),
                                attributes: vec![
                                    ("class".to_string(), "description".to_string()),
                                ],
                                children: vec![
                                    Node::Text("This content was generated from AST nodes using render-core and WebAssembly.".to_string())
                                ],
                                class_names: vec!["description".to_string()],
                            }
                        ],
                        class_names: vec!["main-content".to_string()],
                    }
                ],
                class_names: vec!["app".to_string()],
            }
        ]
    }
    
    #[test]
    fn test_create_hello_world() {
        let dom_nodes = create_hello_world();
        assert_eq!(dom_nodes.len(), 1);
        
        // Check if we got a div element
        if let DomNode::Element { tag, attributes, children, .. } = &dom_nodes[0] {
            assert_eq!(tag, "div");
            assert!(attributes.contains(&("class".to_string(), "greeting".to_string())));
            assert!(attributes.contains(&("id".to_string(), "hello".to_string())));
            assert_eq!(children.len(), 1);
            
            // Check the text content
            if let DomNode::Text(text) = &children[0] {
                assert_eq!(text, "Hello world!");
            } else {
                panic!("Expected text node as child");
            }
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_create_complex_greeting() {
        let dom_nodes = create_complex_greeting();
        assert_eq!(dom_nodes.len(), 1);
        
        if let DomNode::Element { tag, children, .. } = &dom_nodes[0] {
            assert_eq!(tag, "div");
            assert_eq!(children.len(), 2); // h1 and p elements
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_create_web_demo_ast() {
        let ast_nodes = create_web_demo_ast();
        assert_eq!(ast_nodes.len(), 1);
        
        if let Node::Element { tag_name, children, .. } = &ast_nodes[0] {
            assert_eq!(tag_name, "div");
            assert_eq!(children.len(), 2); // header and main
        } else {
            panic!("Expected element node");
        }
    }
}
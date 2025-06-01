use crate::{Lexer, Node, Parser, styleman::StyleMan, workspace::WorkspaceConfig};

pub struct Runtime {
    pub workspace: WorkspaceConfig,
    pub style_manager: Option<StyleMan>,
}

impl Runtime {
    pub fn new(workspace: WorkspaceConfig) -> Self {
        Self {
            workspace,
            style_manager: Some(StyleMan::new()),
        }
    }

    pub fn process_node_for_styles(&mut self, node: &Node) {
        // Only handle Element nodes with classes
        if let Node::Element {
            class_names,
            children,
            ..
        } = node
        {
            if let Some(style_manager) = &mut self.style_manager {
                style_manager.add_classes(class_names.clone());
            }
            // Recursively process children
            for child in children {
                self.process_node_for_styles(child);
            }
        }
    }

    pub fn process_source(&mut self, source: &str) -> Result<String, String> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let node = parser.parse()?;

        self.style_manager = Some(StyleMan::new()); // Reset styles each time

        self.process_node_for_styles(&node);

        if let Some(style_manager) = &self.style_manager {
            Ok(style_manager.generate_css())
        } else {
            Ok(String::new())
        }
    }

    pub fn run(&mut self, source: &str) {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(node) => {
                println!("Parsed AST: {:#?}", node);

                // First process all nodes to collect styles
                self.process_node_for_styles(&node);

                // Then generate CSS if we have a style manager
                if let Some(style_manager) = &mut self.style_manager {
                    let css = style_manager.generate_css();
                    println!("Generated CSS:\n{}", css);
                }
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}

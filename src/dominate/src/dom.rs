use std::fmt;

/// DesignTime DOM node with metadata and styling info.
#[derive(Debug, Clone)]
pub enum DomNode {
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        class_names: Vec<String>,
        children: Vec<DomNode>,
        key: String,
        inline_style: Option<String>,
    },
    Text(String),
    Expression(String),
}

impl DomNode {
    /// Start building a new element node with the given tag.
    pub fn element(tag: &str) -> DomNodeBuilder {
        DomNodeBuilder {
            tag: tag.to_string(),
            attributes: Vec::new(),
            class_names: Vec::new(),
            children: Vec::new(),
            key: None,
            inline_style: None,
        }
    }

    pub fn text(text: &str) -> DomNode {
        DomNode::Text(text.to_string())
    }

    pub fn expression(expr: &str) -> DomNode {
        DomNode::Expression(expr.to_string())
    }
}

pub struct DomNodeBuilder {
    tag: String,
    attributes: Vec<(String, String)>,
    class_names: Vec<String>,
    children: Vec<DomNode>,
    key: Option<String>,
    inline_style: Option<String>,
}

impl DomNodeBuilder {
    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attributes.push((name.to_string(), value.to_string()));
        self
    }

    pub fn class(mut self, class_name: &str) -> Self {
        self.class_names.push(class_name.to_string());
        self
    }

    pub fn child(mut self, node: DomNode) -> Self {
        self.children.push(node);
        self
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());
        self
    }

    pub fn inline_style(mut self, style: &str) -> Self {
        self.inline_style = Some(style.to_string());
        self
    }

    pub fn attributes(mut self, attrs: Vec<(String, String)>) -> Self {
        self.attributes.extend(attrs);
        self
    }

    pub fn class_names(mut self, classes: Vec<String>) -> Self {
        self.class_names.extend(classes);
        self
    }

    pub fn children(mut self, children: Vec<DomNode>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn build(self) -> DomNode {
        DomNode::Element {
            tag: self.tag,
            attributes: self.attributes,
            class_names: self.class_names,
            children: self.children,
            key: self.key.unwrap_or_else(|| "auto_key".to_string()),
            inline_style: self.inline_style,
        }
    }
}

impl fmt::Display for DomNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_node(node: &DomNode, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            let indent_str = "  ".repeat(indent);
            match node {
                DomNode::Text(text) => writeln!(f, "{}Text: {}", indent_str, text),
                DomNode::Expression(expr) => writeln!(f, "{}Expression: {{ {} }}", indent_str, expr),
                DomNode::Element { tag, attributes, class_names, children, key, inline_style } => {
                    writeln!(f, "{}Element: <{}> (key: {})", indent_str, tag, key)?;
                    if !attributes.is_empty() {
                        writeln!(f, "{}  Attributes:", indent_str)?;
                        for (name, value) in attributes {
                            writeln!(f, "{}    {} = \"{}\"", indent_str, name, value)?;
                        }
                    }
                    if !class_names.is_empty() {
                        writeln!(f, "{}  Class Names:", indent_str)?;
                        for class in class_names {
                            writeln!(f, "{}    {}", indent_str, class)?;
                        }
                    }
                    if let Some(style) = inline_style {
                        writeln!(f, "{}  Inline Style: {}", indent_str, style)?;
                    }
                    if children.is_empty() {
                        writeln!(f, "{}  Children: None", indent_str)
                    } else {
                        writeln!(f, "{}  Children:", indent_str)?;
                        for child in children {
                            fmt_node(child, f, indent + 2)?;
                        }
                        Ok(())
                    }
                }
            }
        }
        fmt_node(self, f, 0)
    }
}

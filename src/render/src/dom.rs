use dominate::dom::DomNode;
use web_sys::{Document, console, Node as WebSysNode};
use designtime_ast::Node;
// todo: use class_names
pub fn render_node(doc: &Document, parent_dom: &WebSysNode, node: &DomNode) {
    match node {
        DomNode::Element { tag, attributes, class_names, children, key, inline_style } => { 
            let el = doc.create_element(&tag).unwrap();
            for (k, v) in attributes {
                el.set_attribute(k, v).unwrap();
            }
            for child_node in children {
                render_node(doc, &el, child_node);
            }
            parent_dom.append_child(&el).unwrap();
        }
        DomNode::Text(t) => {
            let text_node = doc.create_text_node(&t);
            parent_dom.append_child(&text_node).unwrap();
        }
        DomNode::Expression(t) => {
            let text_node = doc.create_text_node(&t);
            parent_dom.append_child(&text_node).unwrap();
        }
        _ => {
            console::log_1(&"Unsupported node".into());
        }
    }
}

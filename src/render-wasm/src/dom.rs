use wasm_bindgen::JsValue;
use web_sys::{Document, Node as WebSysNode};
use dominate::dom::DomNode;

pub fn render_node(
    doc: &Document,
    parent_dom: &WebSysNode,
    node: &DomNode,
) -> Result<(), JsValue> {
    match node {
        DomNode::Element {
            tag,
            attributes,
            children,
            ..
        } => {
            let el = doc.create_element(tag).map_err(|e| {
                JsValue::from_str(&format!("Failed to create element {}: {:?}", tag, e))
            })?;
            
            for (k, v) in attributes {
                el.set_attribute(k, v)
                    .map_err(|e| JsValue::from_str(&format!("Failed to set attribute: {:?}", e)))?;
            }

            for child_node in children {
                render_node(doc, &el, child_node)?;
            }

            parent_dom
                .append_child(&el)
                .map_err(|e| JsValue::from_str(&format!("Failed to append child: {:?}", e)))?;
        }
        DomNode::Text(t) => {
            let text_node = doc.create_text_node(t);
            parent_dom
                .append_child(&text_node)
                .map_err(|e| JsValue::from_str(&format!("Failed to append text node: {:?}", e)))?;
        }
        DomNode::Expression(t) => {
            let text_node = doc.create_text_node(t);
            parent_dom
                .append_child(&text_node)
                .map_err(|e| JsValue::from_str(&format!("Failed to append expression node: {:?}", e)))?;
        }
        _ => {
            web_sys::console::warn_1(&"Unsupported node type".into());
        }
    }

    Ok(())
}

//! WebAssembly bindings for the render library

mod dom;

use wasm_bindgen::prelude::*;
use designtime_ast::Node;
use render_core::transform;

/// Initialize the WebAssembly module
///
/// This sets up console error handling for better debugging.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Render nodes from a JSON string into the DOM
///
/// # Arguments
/// * `json` - A JSON string containing an array of AST nodes
///
/// # Returns
/// Returns `Ok(())` on success, or a `JsValue` containing an error message on failure.
/// 
/// For now, this function processes nodes one by one. For better performance, I should use a batch renderer.
#[wasm_bindgen(js_name = renderFromJson)]
pub fn render_from_json(json: &str) -> Result<(), JsValue> {
    // Parse the JSON string into AST nodes
    let nodes: Vec<Node> = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

    // Get the DOM document and body
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document found"))?;
    let body = document.body().ok_or_else(|| JsValue::from_str("No body found"))?;

    // Process and render each node
    for node in nodes {
        let transformed = transform(vec![node]);
        for n in transformed {
            dom::render_node(&document, &body, &n)
                .map_err(|e| JsValue::from_str(&format!("Rendering error: {:?}", e)))?;
        }
    }
    
    Ok(())
}

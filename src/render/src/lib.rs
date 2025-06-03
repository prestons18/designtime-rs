mod dom;
mod render;

use wasm_bindgen::prelude::*;
use dominate::prelude::{transform};
use designtime_ast::Node;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_from_json(json: &str) {
    let nodes: Vec<Node> = serde_json::from_str(json).unwrap();
    // let document = window().unwrap().document().unwrap();
    // let body = document.body().unwrap();

    for node in nodes {
        transform(vec![node]);
    }
}

use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = Springy)]
extern "C" {
    pub type Graph;

    #[wasm_bindgen(constructor, js_namespace = Springy)]
    pub fn new() -> Graph;

    #[wasm_bindgen(method, js_name = "newNode", js_class = Graph)]
    pub fn new_node(this: &Graph, node: JsValue) -> Object;

    #[wasm_bindgen(method, js_name = "newEdge", js_class = Graph)]
    pub fn new_edge(this: &Graph, lhs: Object, rhs: Object);
}

#[wasm_bindgen]
pub struct Node {
    label: String,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

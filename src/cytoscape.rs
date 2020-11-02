use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn init() -> Cytoscape {
    cytoscape(Options {
        elements: vec![Node {
            group: NodeGroup::Nodes,
            data: NodeData {
                id: "1".to_string(),
                parent: None,
            },
        }],
    })
}

#[wasm_bindgen]
extern "C" {
    pub type Cytoscape;

    pub fn cytoscape(options: Options) -> Cytoscape;

    #[wasm_bindgen(catch, method)]
    pub async fn mount(this: &Cytoscape, element: Element) -> Result<JsValue, JsValue>;
}

enum NodeGroup {
    Nodes,
    Edges,
}

#[wasm_bindgen]
pub struct Options {
    container: web_sys::Element,
    // container: Option<web_sys::Element>,

    // For convenience, this option can alternatively be specified as a promise that resolves to the elements JSON.
    elements: Vec<Node>,
}

#[wasm_bindgen]
pub struct Node {
    group: NodeGroup,
    data: NodeData,
}

#[wasm_bindgen]
pub struct NodeData {
    id: String,
    parent: Option<String>,
}

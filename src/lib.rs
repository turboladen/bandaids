use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    cy: Object,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            cy: build_graph(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        // html! {
        //     <div id="counter">
        //         <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
        //         <p>{ self.value }</p>
        //     </div>
        // }
        html! {
            <div id="cy">
            </div>
        }
    }
}

fn build_graph() -> Object {
    let data = {
        let d = Object::create(&JsValue::NULL.into());
        Reflect::set(&d, &JsValue::from_str("id"), &JsValue::from_str("a")).unwrap();

        d
    };

    let node = {
        let n = Object::create(&JsValue::NULL.into());
        Reflect::set(&n, &JsValue::from_str("data"), &data).unwrap();
        n
    };

    let nodes = Array::new();
    nodes.push(node.as_ref());

    let elements = {
        let e = Object::create(&JsValue::NULL.into());
        Reflect::set(&e, &JsValue::from_str("nodes"), &nodes).unwrap();
        e
    };

    let options = {
        let o = Object::create(&JsValue::NULL.into());
        Reflect::set(&o, &JsValue::from_str("elements"), elements.as_ref()).unwrap();
        o
    };

    cytoscape(options)
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

// #[wasm_bindgen(raw_module = "/static/cytoscape.min.js")]
#[wasm_bindgen]
extern "C" {
    fn cytoscape(options: Object) -> Object;
}

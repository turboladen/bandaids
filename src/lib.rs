mod cytoscape;

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    graph: cytoscape::Cytoscape,
}

impl Model {
    async fn mount(&self) -> Result<JsValue, JsValue> {
        let cy = web_sys::window()
            .unwrap()
            .document()
            .expect_throw("no document")
            .get_element_by_id("cy")
            .expect_throw("no 'cy' id");

        self.graph.mount(cy).await
    }
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
            graph: cytoscape::init(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
            }
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
        html! {
            <div id="main">
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
                <div id="cy">
                </div>
            </div>
        }
    }

    // fn rendered(&mut self, first_render: bool) {
    //     if first_render {
    //         self.mount().expect_throw("uh oh");
    //     }
    // }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

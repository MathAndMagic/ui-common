use yew::prelude::*;

use ui_common::Icon;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"It's just some "} {Icon::MATH_OPERATIONS}{" and "} {Icon::MAGIC_WAND}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

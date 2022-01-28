use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let title = "Hello World";
    html! {
        <h1>{ title }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}

use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let title = use_state(|| String::from("Hello Yew"));
    let counter = use_state(|| 0);

    let increase = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let decrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };
    {
        let title = title.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = reqwest::get("http://localhost:8000?lang=rust")
                        .await
                        .expect("request fail");
                    let body = res.text().await.expect("parse fail");
                    title.set(body);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <div>
            <h1>{ (*title).as_str() }</h1>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
            <button onclick={increase}> { "+" } </button>
            <button onclick={decrease}> { "-" } </button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}

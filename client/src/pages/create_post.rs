use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::Route;

#[function_component(CreatePost)]
pub fn create_post() -> Html {
    let input_ref = use_node_ref();
    let textarea_ref = use_node_ref();

    let history = use_history().unwrap();

    let publish = {
        let input_ref = input_ref.clone();
        let textarea_ref = textarea_ref.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let input_ref = input_ref.clone();
            let textarea_ref = textarea_ref.clone();
            let history = history.clone();
            spawn_local(async move {
                let origin = gloo::utils::window().location().origin().unwrap();
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let textarea = textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
                let mut map = HashMap::new();
                map.insert("title", input.value());
                map.insert("body", textarea.value());
                reqwest::Client::new()
                    .post(format!("{origin}/api/create_post"))
                    .json(&map)
                    .send()
                    .await
                    .expect("create fail");
                history.push(Route::Home);
            })
        })
    };

    html! {
        <div class="create-post">
            <input ref={input_ref} class="post-title" placeholder="输入标题"/>
            <textarea ref={textarea_ref} class="post-content" placeholder="输入正文"/>
            <button class="post-confirm" onclick={publish}>{"提交"}</button>
        </div>
    }
}

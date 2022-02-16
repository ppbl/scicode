use crate::components::button::Button;
use crate::utils::get_origin::get_origin;
use crate::utils::request::get_client;
use crate::Route;
use gloo::dialogs::alert;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

#[derive(Serialize)]
struct NewPost {
    title: String,
    body: String,
    topics: Vec<i32>,
}

#[function_component(CreatePost)]
pub fn create_post() -> Html {
    let input_ref = use_node_ref();
    let textarea_ref = use_node_ref();
    let topics_ref = use_node_ref();

    let history = use_history().unwrap();

    let publish = {
        let input_ref = input_ref.clone();
        let textarea_ref = textarea_ref.clone();
        let topics_ref = topics_ref.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let textarea = textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
            let topics = topics_ref.cast::<HtmlInputElement>().unwrap();
            let new_post = NewPost {
                title: input.value(),
                body: textarea.value(),
                topics: topics
                    .value()
                    .split(",")
                    .map(|topic_id| topic_id.parse::<i32>().unwrap())
                    .collect(),
            };
            let history = history.clone();
            spawn_local(async move {
                let res = get_client()
                    .post(format!("{}/api/create_post", get_origin()))
                    .json(&new_post)
                    .send()
                    .await
                    .expect("create fail")
                    .text()
                    .await
                    .unwrap();
                if res == "success" {
                    history.push(Route::Home);
                } else {
                    alert(&res);
                }
            })
        })
    };

    html! {
        <div class="create-post">
            <input ref={input_ref} class="px-2 py-4 border border-slate-200 rounded" placeholder="输入标题"/>
            <textarea ref={textarea_ref} class="min-h-[200px] px-2 py-4 border border-slate-200 rounded" placeholder="输入正文"/>
            <input ref={topics_ref} class="px-2 py-4 border border-slate-200 rounded" placeholder="输入话题id（多个id用，隔开）"/>
            <Button onclick={publish}>{ "提交" }</Button>
        </div>
    }
}

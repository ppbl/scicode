use crate::components::button::Button;
use crate::pages::Topic;
use crate::utils::get_client;
use crate::utils::get_origin;
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
                topics: if topics.value() == "" {
                    [-1].to_vec()
                } else {
                    topics
                        .value()
                        .split(",")
                        .map(|topic_id| topic_id.parse::<i32>().unwrap_or(-1))
                        .collect()
                },
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

    let topics = use_state(|| Vec::new());
    {
        let topics = topics.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body: Vec<Topic> = reqwest::get(format!("{}/api/topics", get_origin()))
                        .await
                        .expect("request fail")
                        .json()
                        .await
                        .expect("parse fail");
                    topics.set(body);
                });
                || ()
            },
            (),
        );
    };

    html! {
        <div class="create-post">
            <input ref={input_ref} class="px-2 py-4 border border-slate-200 rounded" placeholder="输入标题"/>
            <textarea ref={textarea_ref} class="min-h-[200px] px-2 py-4 border border-slate-200 rounded" placeholder="输入正文"/>
            <select ref={topics_ref} class="px-1 border border-slate-200 rounded text-gray-400" >
                <option value="fffsdfsd" >{"请选择一个话题"}</option>
                {
                    (*topics).iter().map(|Topic { id, name }| {
                        html!(<option key={id.to_string()} value={id.to_string()} >{name}</option>)
                    }).collect::<Html>()
                }
            </select>
            <Button onclick={publish}>{ "提交" }</Button>
        </div>
    }
}

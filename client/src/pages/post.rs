use crate::{
    components::markdown::Markdown,
    utils::{get_origin::*, request::get_client},
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::components::button::Button;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: i32,
}
#[derive(Clone, PartialEq, Deserialize)]
pub struct PostBody {
    id: i32,
    title: String,
    body: String,
    username: String,
    create_at: NaiveDateTime,
}

#[derive(Serialize)]
struct NewComment {
    post_id: i32,
    body: String,
}

#[derive(Deserialize)]
struct Comment {
    post: i32,
    body: String,
    author: i32,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let refresh = use_state(|| false);
    let post = use_state(|| PostBody {
        id: 0,
        title: "".to_string(),
        body: "".to_string(),
        username: "".to_string(),
        create_at: Local::now().naive_local(),
    });

    {
        let post = post.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body: PostBody = reqwest::get(format!("{}/api/post?id={id}", get_origin()))
                        .await
                        .expect("request fail")
                        .json()
                        .await
                        .expect("parse fail");
                    post.set(body);
                });
                || ()
            },
            (),
        );
    };
    let comments = use_state(|| Vec::new());
    {
        let comments = comments.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body: Vec<Comment> =
                        reqwest::get(format!("{}/api/comments?id={id}", get_origin()))
                            .await
                            .expect("request fail")
                            .json()
                            .await
                            .expect("parse fail");
                    comments.set(body);
                });
                || ()
            },
            *refresh,
        );
    };
    let textarea_ref = use_node_ref();
    let comment = {
        let textarea_ref = textarea_ref.clone();
        let id = props.id;
        Callback::from(move |_| {
            let textarea = textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
            let comment = NewComment {
                post_id: id,
                body: textarea.value(),
            };
            let refresh = refresh.clone();
            spawn_local(async move {
                get_client()
                    .post(format!("{}/api/post_comment", get_origin()))
                    .json(&comment)
                    .send()
                    .await
                    .expect("create fail");
                textarea.set_value("");
                refresh.set(!(*refresh));
            })
        })
    };
    html! {
        <div class="post">
            <section class="mt-4 p-4 bg-white rounded shadow shadow-gray-300">
                <h1 class="text-xl">{&((*post)).title}</h1>
                <div>{&*post.username}{"发布于"}{(*post).create_at.format("%Y-%m-%d %H:%M:%S")}</div>
                <Markdown source={(*post).body.to_string()} />
            </section>
            <section class="mt-4 p-4 bg-white rounded shadow shadow-gray-300">
                <textarea ref={textarea_ref} class="w-full p-2 border rounded border-gray-200 block" placeholder="输入评论"/>
                <div class="mt-2 flex justify-end"><Button onclick={comment}>{"评论"}</Button></div>
                <div class="mt-4 py-2 font-semibold">{(*comments).len()}{"条评论"}</div>
                {
                    (*comments).iter().map(|item| {
                        html! {
                            <div class="py-2 border-t border-gray-100">{ &item.body }</div>
                        }
                    }).collect::<Html>()
                }
            </section>
        </div>
    }
}

use gloo::dialogs::alert;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    utils::{get_origin::get_origin, request::get_client},
    Route,
};

#[derive(Clone, PartialEq, Deserialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let posts = use_state(|| Vec::new());
    {
        let posts = posts.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body: Vec<Post> = reqwest::get(format!("{}/api/posts", get_origin()))
                        .await
                        .expect("request fail")
                        .json()
                        .await
                        .expect("parse fail");
                    posts.set(body);
                });
                || ()
            },
            (),
        );
    };

    fn delete(id: i32, posts: &UseStateHandle<Vec<Post>>) {
        let posts = posts.clone();
        spawn_local(async move {
            let res = get_client()
                .delete(format!("{}/api/delete_post?id={id}", get_origin()))
                .send()
                .await
                .expect("request fail")
                .text()
                .await
                .unwrap();
            if res == "success" {
                posts.set(posts.iter().filter(|item| item.id != id).cloned().collect())
            } else {
                alert(&res)
            }
        })
    }

    html! {
        <ul class="posts">
        {
            (*posts).iter().map(|item| html!{
                <li class="posts-item">
                    <div class="posts-item-title" onclick={
                        let history = history.clone();
                        let id = item.id;
                        Callback::from(move |_| {
                            history.push(Route::Post {
                                id,
                            })
                        })
                }>{ &item.title }</div>
                <span class="posts-item-delete" onclick={
                    let posts = posts.clone();
                    let id = item.id;
                    Callback::from( move |_| {
                        delete(id, &posts);
                    })
                }
                    >{"删除"}</span>
                </li>
            }).collect::<Html>()
        }
        </ul>
    }
}

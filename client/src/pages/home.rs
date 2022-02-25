use chrono::NaiveDateTime;
use gloo::dialogs::alert;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    utils::{get_origin::get_origin, request::get_client},
    Route,
};

use super::post::User;

#[derive(Clone, PartialEq, Deserialize)]
pub struct PostBody {
    id: i32,
    title: String,
    body: String,
    topics: Vec<i32>,
    author: User,
    create_at: NaiveDateTime,
    ups: i32,
    downs: i32,
    voting: Option<bool>,
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
                    let body: Vec<PostBody> = reqwest::get(format!("{}/api/posts", get_origin()))
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

    fn delete(id: i32, posts: &UseStateHandle<Vec<PostBody>>) {
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
            (*posts).iter().map(|PostBody { id, title, author, .. }| html!{
                <li class="posts-item">
                    <div class="posts-item-title" onclick={
                        let history = history.clone();
                        let id = *id;
                        Callback::from(move |_| {
                            history.push(Route::Post {
                                id,
                            })
                        })}
                    >
                        <div class="mb-2 flex items-center">
                            {
                                match &author.avatar_url{
                                    Some(avatar_url) => html!(<img class="w-6 h-6 rounded-full mr-2" src={avatar_url.clone()} alt="" />),
                                    None => html!()
                                }
                            }
                            <span>{&author.username}</span>
                        </div>
                        <div>{ title }</div>
                    </div>
                    <span class="posts-item-delete" onclick={
                        let posts = posts.clone();
                        let id = *id;
                        Callback::from( move |_| {
                            delete(id, &posts);
                        })}
                    >
                        {"删除"}
                    </span>
                </li>
            }).collect::<Html>()
        }
        </ul>
    }
}

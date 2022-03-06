use crate::{utils::get_origin, Route};
use chrono::NaiveDateTime;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use super::SomeUser;

#[derive(Clone, PartialEq, Deserialize)]
pub struct PostBody {
    id: i32,
    title: String,
    body: String,
    topics: Vec<i32>,
    author: SomeUser,
    create_at: NaiveDateTime,
    ups: i32,
    downs: i32,
    voting: Option<bool>,
}

#[derive(Properties, PartialEq)]
pub struct TopicPageProps {
    pub topic: String,
}

#[function_component(TopicPage)]
pub fn topic_page(props: &TopicPageProps) -> Html {
    let history = use_history().unwrap();

    let posts = use_state(|| Some(Vec::new()));
    {
        let posts = posts.clone();
        let topic = props.topic.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body = reqwest::get(format!("{}/api/posts?topic={}", get_origin(), topic))
                        .await
                        .expect("request fail")
                        .json()
                        .await;
                    if let Ok(body) = body {
                        posts.set(body);
                    } else {
                        // log!("Ewqqrq");
                        posts.set(None);
                    }
                });
                || ()
            },
            (),
        );
    };

    match (*posts).clone() {
        Some(posts) => {
            html! {
                <div class="mx-auto max-w-[710px]">
                    <h1 class="mt-2">{&props.topic}</h1>
                    <ul class="posts">
                    {
                        (posts).iter().map(|PostBody { id, title, author, .. }| html!{
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

                            </li>
                        }).collect::<Html>()
                    }
                    </ul>
                </div>
            }
        }
        None => {
            html! { <h1>{ "404" }</h1> }
        }
    }
}

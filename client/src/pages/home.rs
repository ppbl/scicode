use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

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
                    let origin = gloo::utils::window().location().origin().unwrap();
                    let body: Vec<Post> = reqwest::get(format!("{origin}/api/posts"))
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

    fn delete(id: i32) {
        spawn_local(async move {
            let origin = gloo::utils::window().location().origin().unwrap();
            reqwest::Client::new()
                .delete(format!("{origin}/api/delete_post?id={id}"))
                .send()
                .await
                .expect("request fail");
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
                    Callback::from(move |_| {
                        delete(id);
                        posts.set(posts.iter().filter(|item| item.id != id).cloned().collect())
                    })
                }
                    >{"删除"}</span>
                </li>
            }).collect::<Html>()
        }
        </ul>
    }
}

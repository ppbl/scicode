use crate::components::button::Button;
use crate::pages::Topic;
use crate::utils::{get_origin, get_userid};
use crate::Route;
use crate::{components::markdown::Markdown, utils::get_client};
use chrono::prelude::*;
use gloo::dialogs::{alert, confirm};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History};
use yew_router::hooks::use_history;
use yew_router::prelude::*;

use super::SomeUser;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: i32,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct PostBody {
    id: i32,
    title: String,
    body: String,
    topics: Vec<Topic>,
    author: SomeUser,
    create_at: NaiveDateTime,
    ups: i32,
    downs: i32,
    voting: Option<bool>,
}

#[derive(Serialize)]
struct NewComment {
    post_id: i32,
    body: String,
}

#[derive(Deserialize)]
struct ThumbsRes {
    ups: i32,
    downs: i32,
    voting: Option<bool>,
}

#[derive(Deserialize)]
struct Comment {
    id: i32,
    body: String,
    create_at: NaiveDateTime,
    author: SomeUser,
}
#[derive(Serialize)]
struct VoteBody {
    post: i32,
    vote_type: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let refresh = use_state(|| false);

    let post = use_state(|| PostBody {
        id: 0,
        title: "".to_string(),
        body: "".to_string(),
        topics: vec![Topic {
            id: 0,
            name: "".to_string(),
        }],
        author: SomeUser {
            id: 0,
            username: "".to_string(),
            avatar_url: None,
        },
        create_at: Local::now().naive_local(),
        ups: 0,
        downs: 0,
        voting: None,
    });
    {
        let post = post.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body = get_client()
                        .get(format!("{}/api/post?id={id}", get_origin()))
                        .send()
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
                let res = get_client()
                    .post(format!("{}/api/post_comment", get_origin()))
                    .json(&comment)
                    .send()
                    .await
                    .expect("create fail")
                    .text()
                    .await
                    .unwrap();
                if res == "success" {
                    textarea.set_value("");
                    refresh.set(!(*refresh));
                } else {
                    alert(&res);
                }
            })
        })
    };

    fn vote(post_id: i32, vote_type: &'static str, post: &UseStateHandle<PostBody>) {
        let post = post.clone();
        let body = VoteBody {
            post: post_id,
            vote_type: vote_type.to_string(),
        };
        spawn_local(async move {
            let res = get_client()
                .post(format!("{}/api/vote", get_origin()))
                .json(&body)
                .send()
                .await
                .expect("vote fail")
                .json::<ThumbsRes>()
                .await
                .expect("failed ---------------");
            let mut next_post = (*post).clone();
            next_post.ups = res.ups;
            next_post.downs = res.downs;
            next_post.voting = res.voting;
            post.set(next_post);
        });
    }

    let history = use_history().unwrap();
    fn delete(id: i32, history: &AnyHistory) {
        let history = history.clone();
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
                history.replace(Route::Home)
            } else {
                alert(&res)
            }
        })
    }

    let PostBody {
        id,
        title,
        body,
        topics,
        author,
        create_at,
        ups,
        downs,
        voting,
        ..
    } = &*post;

    html! {
        <div class="post">
            <section class="mt-4 p-4 bg-white rounded shadow shadow-gray-300">
                <div class="mb-2">
                {topics.iter().map(|item| {
                    html! {
                        <Link<Route> to={Route::TopicPage{topic: item.name.clone()}} classes="mr-2 py-1 px-4 rounded-full bg-blue-100 text-blue-600 cursor-pointer">{ &item.name }</Link<Route>>
                    }
                }).collect::<Html>()}
                </div>
                <h1 class="pb-2 text-xl">{title}</h1>
                <div class="flex items-center">
                    {
                        if let Some(avatar) = author.avatar_url.clone() {
                            html!(
                                <a href={format!("/user/{}", author.id)} target="_blank"><img  class="mr-2 w-8 h-8 rounded-full" src={avatar} alt="" /></a>
                            )
                        }else {
                            html!()
                        }
                    }
                    <div class="text-sm text-slate-500">{author.username.clone()}{"发布于"}{Local.from_utc_datetime(create_at).format("%Y-%m-%d %H:%M:%S")}</div>
                </div>
                <Markdown class="pt-2" source={body.to_string()} />
                <div class="flex justify-between mt-2">
                    <div>
                        {
                            html!(
                                if voting.is_none() || !voting.unwrap()  {
                                        <button onclick={{
                                            let post = post.clone();
                                            let post_id = props.id;
                                            Callback::from(move |_| {
                                                vote(post_id, "up", &post);
                                            })
                                        }}>{"👍"}</button>
                                }else {
                                    <button onclick={{
                                        let post = post.clone();
                                        let post_id = props.id;
                                        Callback::from(move |_| {
                                            vote(post_id, "neutral", &post);
                                        })
                                    }}>{"✊"}</button>
                                }
                            )
                        }
                        <span class="mx-1">{ups - downs}</span>
                        {
                            html!(
                                if voting.is_none() || voting.unwrap() {
                                    <button onclick={{
                                        let post = post.clone();
                                        let post_id = props.id;
                                        Callback::from(move |_| {
                                            vote(post_id, "down", &post);
                                        })
                                    }}>{"👎"}</button>
                                }else {
                                    <button onclick={{
                                        let post = post.clone();
                                        let post_id = props.id;
                                        Callback::from(move |_| {
                                            vote(post_id, "neutral", &post);
                                        })
                                    }}>{"✊"}</button>
                                }
                            )
                        }
                    </div>
                    <div>
                        {
                            if let Some(userid) = get_userid() {
                                if author.id.to_string() == userid {
                                    html!(
                                        <span class="posts-item-delete" onclick={
                                            let history = history.clone();
                                            let id = *id;
                                            Callback::from( move |_| {
                                                if confirm("确定删除吗") {
                                                    delete(id, &history);
                                                }
                                            })}
                                        >
                                            {"删除"}
                                        </span>
                                    )
                                } else {
                                    html!()
                                }
                            }else {
                                html!()
                            }
                        }
                    </div>
                </div>
            </section>
            <section class="mt-4 p-4 bg-white rounded shadow shadow-gray-300">
                <textarea ref={textarea_ref} class="w-full p-2 border rounded border-gray-200 block" placeholder="输入评论"/>
                <div class="mt-2 flex justify-end"><Button onclick={comment}>{"评论"}</Button></div>
                <div class="mt-4 py-2 font-semibold">{(*comments).len()}{"条评论"}</div>
                {
                    (*comments).iter().map(|Comment{id, body, create_at, author}| {
                        html! {
                            <div key={id.to_string().clone()} class="flex py-2 border-t border-gray-100">
                                <div></div>
                                <div class="flex-auto">
                                    <div class="mb-1 flex justify-between">
                                        <div class="flex items-center">
                                        {
                                            match &author.avatar_url{
                                                Some(avatar_url) => {
                                                    html!( <a href={format!("/user/{}", author.id)} target="_blank"><img class="w-6 h-6 mr-2 rounded-full" src={avatar_url.clone()} alt="" /></a> )
                                                }
                                                None => html!()
                                            }
                                        }
                                            <span>{ &author.username }</span>
                                        </div>
                                        <span class="text-slate-400">{ Local.from_utc_datetime(create_at).format("%m-%d %H:%M:%S") }</span>
                                    </div>
                                    <div class="">{ body }</div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </section>
        </div>
    }
}

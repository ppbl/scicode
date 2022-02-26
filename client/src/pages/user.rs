use crate::utils::get_client;
use crate::utils::get_origin;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserProps {
    pub id: i32,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
    pub github_url: Option<String>,
}

#[function_component(User)]
pub fn user(props: &UserProps) -> Html {
    let user = use_state(|| UserInfo {
        id: 0,
        username: "".to_string(),
        avatar_url: None,
        github_url: None,
    });
    {
        let user = user.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let body = get_client()
                        .get(format!("{}/api/user?id={id}", get_origin()))
                        .send()
                        .await
                        .expect("request fail")
                        .json()
                        .await
                        .expect("parse fail");
                    user.set(body);
                });
                || ()
            },
            (),
        );
    };

    let UserInfo {
        id,
        username,
        avatar_url,
        github_url,
    } = &*user;

    html! {
        <div class="user">
            <div class="flex items-center">
                {
                    match avatar_url{
                        Some(avatar_url) => {
                            html!(<img class="w-24 h-24 mr-6 rounded-full" src={avatar_url.clone()} alt="" />)
                        }
                        None => html!()
                    }
                }
                <div>
                    <div class="mb-2 text-3xl">{username}</div>
                    {
                        match github_url{
                            Some(github_url) => {
                                html!(<div>{"github主页："}<a class="text-blue-600" target="_blank" href={github_url.clone()}>{github_url}</a></div>)
                            }
                            None => html!()
                        }
                    }
                </div>
            </div>
        </div>
    }
}

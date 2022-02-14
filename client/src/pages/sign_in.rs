use gloo::dialogs::alert;
use gloo::utils::window;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::components::button::Button;
use crate::utils::get_origin::get_origin;
use crate::Route;

#[derive(Deserialize)]
pub struct ResBody {
    status: String,
    data: String,
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let history = use_history().unwrap();

    let publish = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |_| {
            let username = username_ref.cast::<HtmlInputElement>().unwrap();
            let password = password_ref.cast::<HtmlInputElement>().unwrap();
            let mut map = HashMap::new();
            map.insert("username", username.value());
            map.insert("password", password.value());

            let history = history.clone();
            spawn_local(async move {
                let res: ResBody = reqwest::Client::new()
                    .post(format!("{}/api/sign_in", get_origin()))
                    .json(&map)
                    .send()
                    .await
                    .expect("sign in fail")
                    .json()
                    .await
                    .expect("");
                if res.status == "success" {
                    window()
                        .local_storage()
                        .unwrap()
                        .unwrap()
                        .set_item("token", &res.data)
                        .unwrap();
                    history.push(Route::Home);
                } else {
                    alert("登录失败");
                }
            })
        })
    };

    html! {
        <div class="create-post">
            <input ref={username_ref} class="post-title" placeholder="输入账号"/>
            <input ref={password_ref} class="post-title" placeholder="输入密码"/>
            <Button onclick={publish}>{ "登录" }</Button>
        </div>
    }
}

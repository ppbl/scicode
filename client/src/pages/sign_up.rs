use crate::components::button::Button;
use crate::utils::get_client;
use crate::utils::get_origin;
use crate::Route;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let history = use_history().unwrap();

    let submit_sign_up = {
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
                get_client()
                    .post(format!("{}/api/sign_up", get_origin()))
                    .json(&map)
                    .send()
                    .await
                    .expect("sign up fail");
                history.push(Route::Home);
            })
        })
    };

    html! {
        <div class="create-post">
            <input ref={username_ref} class="post-title" placeholder="输入账号"/>
            <input ref={password_ref} class="post-title" placeholder="输入密码"/>
            <Button onclick={submit_sign_up}>{ "注册" }</Button>
        </div>
    }
}

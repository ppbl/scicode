use gloo::dialogs::alert;
use gloo::utils::window;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::components::icon::github::Github;
use crate::utils::get_origin::get_origin;
use crate::utils::request::get_client;
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

    // let publish = {
    //     let username_ref = username_ref.clone();
    //     let password_ref = password_ref.clone();
    //     Callback::from(move |_| {
    //         let username = username_ref.cast::<HtmlInputElement>().unwrap();
    //         let password = password_ref.cast::<HtmlInputElement>().unwrap();
    //         let mut map = HashMap::new();
    //         map.insert("username", username.value());
    //         map.insert("password", password.value());

    //         let history = history.clone();
    //         spawn_local(async move {
    //             let res: ResBody = get_client()
    //                 .post(format!("{}/api/sign_in", get_origin()))
    //                 .json(&map)
    //                 .send()
    //                 .await
    //                 .expect("sign in fail")
    //                 .json()
    //                 .await
    //                 .expect("");
    //             if res.status == "success" {
    //                 window()
    //                     .local_storage()
    //                     .unwrap()
    //                     .unwrap()
    //                     .set_item("token", &res.data)
    //                     .unwrap();
    //                 history.push(Route::Home);
    //                 window().location().reload().unwrap();
    //             } else {
    //                 alert("登录失败");
    //             }
    //         })
    //     })
    // };

    let open_authorize_window = {
        Callback::from(move |_| {
            window().open_with_url_and_target_and_features("https://github.com/login/oauth/authorize?client_id=346b0d0b5427b64bb33c&redirect_uri=http://localhost:8080/api/login/oauth", "github", "left=0,top=0,width=800,height=600").expect("open fail");
        })
    };

    html! {
        <div class="mx-auto max-w-[710px] py-4">
            <Github class="mx-auto cursor-pointer" onclick={open_authorize_window}/>
        </div>
    }
}

use crate::components::button::Button;
use crate::utils::get_client;
use crate::utils::get_origin;
use gloo::dialogs::alert;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(CreateTopic)]
pub fn create_topic() -> Html {
    let topic_ref = use_node_ref();

    let create = {
        let topic_ref = topic_ref.clone();
        Callback::from(move |_| {
            let topic = topic_ref.cast::<HtmlInputElement>().unwrap();
            let mut map = HashMap::new();
            map.insert("name", topic.value());
            spawn_local(async move {
                let res = get_client()
                    .post(format!("{}/api/create_topic", get_origin()))
                    .json(&map)
                    .send()
                    .await
                    .expect("create in fail")
                    .text()
                    .await
                    .expect("");
                if res == "success" {
                    alert("创建成功");
                    topic.set_value("");
                } else {
                    alert("登录失败");
                }
            })
        })
    };

    html! {
        <div class="create-post">
            <input ref={topic_ref} class="post-title px-2" placeholder="输入主题名称"/>
            <Button onclick={create}>{ "创建" }</Button>
        </div>
    }
}

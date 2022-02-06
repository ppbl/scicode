use crate::components::markdown::Markdown;
use gloo::console::log;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: i32,
}
#[derive(Clone, PartialEq, Deserialize)]
pub struct PostBody {
    id: i32,
    title: String,
    body: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    log!(&props.id.to_string());

    let post = use_state(|| PostBody {
        id: 0,
        title: "".to_string(),
        body: "".to_string(),
    });
    {
        let post = post.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let origin = gloo::utils::window().location().origin().unwrap();
                    let body: PostBody = reqwest::get(format!("{origin}/api/post?id={id}"))
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
    html! {
        <div class="post">
            <h1>{&(*post).title}</h1>
            <Markdown source={(*post).body.to_string()} />
        </div>
    }
}

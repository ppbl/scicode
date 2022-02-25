use gloo::utils::window;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod utils;

use components::button::Button;
use pages::{
    create_post::CreatePost, create_topic::CreateTopic, home::Home, post::Post, sign_in::SignIn,
    sign_up::SignUp,
};

use crate::utils::stroage::{get_token, sign_out};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sign_in")]
    SignIn,
    #[at("/sign_up")]
    SignUp,
    #[at("/post/:id")]
    Post { id: i32 },
    #[at("/create_post")]
    CreatePost,
    #[at("/create_topic")]
    CreateTopic,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::SignIn => html! { <SignIn /> },
        Route::SignUp => html! { "Registration is temporarily closed" },
        Route::Post { id } => html! {
            <Post id={id.clone()}/>
        },
        Route::CreatePost => html! { <CreatePost /> },
        Route::CreateTopic => html! { <CreateTopic /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let sign_out = Callback::from(move |_| {
        sign_out();
        window().location().reload().unwrap();
    });

    let token = get_token();
    html! {
        <BrowserRouter>
            <header class="header">
                <Link<Route> classes="header-logo" to={Route::Home}>
                    {"Scicode"}
                </Link<Route>>
                <div class="flex gap-2">
                    <Link<Route> to={Route::CreatePost}>
                        <Button>{ "发布" }</Button>
                    </Link<Route>>
                    {
                        if let Some(_token) = token {
                            html!(
                                <Button onclick={sign_out}>{ "退出登录" }</Button>
                            )
                        }else {
                            html!(
                                <Link<Route> to={Route::SignIn}>
                                    <Button>{ "登录" }</Button>
                                </Link<Route>>
                            )
                        }
                    }
                </div>
            </header>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}

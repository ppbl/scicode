use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod utils;

use components::button::Button;
use pages::{create_post::CreatePost, home::Home, post::Post};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: i32 },
    #[at("/create_post")]
    CreatePost,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Post { id } => html! {
            <Post id={id.clone()}/>
        },
        Route::CreatePost => html! { <CreatePost /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <header class="header">
                <Link<Route> classes="header-logo" to={Route::Home}>
                    {"scicode"}
                </Link<Route>>
                <Link<Route> to={Route::CreatePost}>
                    <Button>{ "发布" }</Button>
                </Link<Route>>
            </header>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}

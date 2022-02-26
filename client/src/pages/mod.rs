mod create_post;
mod create_topic;
mod home;
mod post;
mod sign_in;
mod sign_up;
mod user;

pub use create_post::*;
pub use create_topic::*;
pub use home::*;
pub use post::*;
pub use sign_in::*;
pub use sign_up::*;
pub use user::*;

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}
#[derive(Clone, PartialEq, Deserialize)]
pub struct SomeUser {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}

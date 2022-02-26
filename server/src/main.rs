#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod auth;
mod db;
mod models;
mod schema;
mod services;

use services::{
    comments, create_post, create_topic, delete_post, login_oauth, post, post_comment, posts,
    sign_in, sign_up, topics, user, vote,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(post)
            .service(create_post)
            .service(delete_post)
            .service(post_comment)
            .service(comments)
            // .service(sign_up)
            // .service(sign_in)
            .service(login_oauth)
            .service(create_topic)
            .service(topics)
            .service(user)
            .service(vote)
    })
    .bind("localhost:8000")?
    .run()
    .await
}

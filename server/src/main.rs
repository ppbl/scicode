#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod db;
mod models;
mod schema;
mod services;

use services::{
    comments::comments, create_post::create_post, delete_post::delete_post, post::post,
    post_comment::post_comment, posts::posts,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(post)
            .service(create_post)
            .service(delete_post)
            .service(post_comment)
            .service(comments)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

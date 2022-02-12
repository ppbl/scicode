use crate::{
    db,
    models::{NewPost, Post},
    schema::posts,
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    title: String,
    body: String,
}

#[post("/create_post")]
async fn create_post(req_body: web::Json<Body>) -> impl Responder {
    if db::can_connect() {
        let connection = db::get_connection();
        let post = NewPost {
            title: &req_body.title,
            body: &req_body.body,
            published: &true,
        };
        diesel::insert_into(posts::table)
            .values(&post)
            .get_result::<Post>(&connection)
            .expect("Error saving new post");
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

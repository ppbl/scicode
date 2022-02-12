use crate::{
    db,
    models::{Comment, NewComment},
    schema::comments,
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    post_id: i32,
    body: String,
}

#[post("/post_comment")]
async fn post_comment(req_body: web::Json<Body>) -> impl Responder {
    if db::can_connect() {
        let connection = db::get_connection();
        let comment = NewComment {
            post_id: &req_body.post_id,
            body: &req_body.body,
        };
        diesel::insert_into(comments::table)
            .values(&comment)
            .get_result::<Comment>(&connection)
            .expect("Error saving new comment");
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

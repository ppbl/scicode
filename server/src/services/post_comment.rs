use crate::{
    auth::Claims,
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
async fn post_comment(req_body: web::Json<Body>, claims: Claims) -> impl Responder {
    if req_body.body.trim() == "" {
        return HttpResponse::BadRequest().body("please input body");
    }
    let conn = db::get_connection();
    let comment = NewComment {
        author: &claims.userid,
        post: &req_body.post_id,
        body: &req_body.body,
    };
    diesel::insert_into(comments::table)
        .values(&comment)
        .load::<Comment>(&conn)
        .expect("Error saving new comment");
    HttpResponse::Ok().body("success")
}

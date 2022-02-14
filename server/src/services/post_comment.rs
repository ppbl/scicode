use crate::{
    db,
    models::{Comment, NewComment},
    schema::comments,
    services::sign_in::{Claims, SECRET},
};
use actix_web::{http::header::AUTHORIZATION, post, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    post_id: i32,
    body: String,
}

#[post("/post_comment")]
async fn post_comment(req_body: web::Json<Body>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let token = token.to_str().unwrap().split(" ").collect::<Vec<&str>>();
        let claims = decode::<Claims>(
            token[1],
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .unwrap()
        .claims;
        if db::can_connect() {
            let connection = db::get_connection();
            let comment = NewComment {
                author: &claims.userid,
                post: &req_body.post_id,
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
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}

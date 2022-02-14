use crate::{
    db,
    models::{NewPost, Post},
    schema::posts,
    services::sign_in::{Claims, SECRET},
};
use actix_web::{
    http::{header::AUTHORIZATION, HeaderValue},
    post, web, HttpRequest, HttpResponse, Responder,
};
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    title: String,
    body: String,
}

#[post("/create_post")]
async fn create_post(req_body: web::Json<Body>, req: HttpRequest) -> impl Responder {
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
            let post = NewPost {
                title: &req_body.title,
                body: &req_body.body,
                published: &true,
                author: &claims.userid,
            };
            diesel::insert_into(posts::table)
                .values(&post)
                .get_result::<Post>(&connection)
                .expect("Error saving new post");
            HttpResponse::Ok().body("success")
        } else {
            HttpResponse::Ok().body("cannot connect to db")
        }
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}

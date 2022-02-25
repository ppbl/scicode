use crate::{
    db,
    models::NewTopic,
    schema::topics,
    auth::{Claims, SECRET},
};
use actix_web::{http::header::AUTHORIZATION, post, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    name: String,
}

#[post("/create_topic")]
async fn create_topic(req_body: web::Json<Body>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let token = token.to_str().unwrap().split(" ").collect::<Vec<&str>>();
        decode::<Claims>(
            token[1],
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .unwrap()
        .claims;
        if req_body.name.trim() == "" {
            return HttpResponse::Ok().body("please input topic name");
        }
        let conn = db::get_connection();
        let topic = NewTopic {
            name: &req_body.name,
        };
        diesel::insert_into(topics::table)
            .values(&topic)
            .execute(&conn)
            .expect("Error creating new topic");
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}

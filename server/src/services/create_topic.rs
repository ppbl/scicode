use crate::{db, models::NewTopic, schema::topics};
use actix_web::{http::header::AUTHORIZATION, post, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    name: String,
}

#[post("/create_topic")]
async fn create_topic(req_body: web::Json<Body>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
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

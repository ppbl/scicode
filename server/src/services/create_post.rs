use crate::{
    auth::Claims,
    db,
    models::{NewPost, Post, Topics},
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    title: String,
    body: String,
    topics: Vec<i32>,
}

#[post("/create_post")]
async fn create_post(req_body: web::Json<Body>, claims: Claims) -> impl Responder {
    use crate::schema::posts::dsl::*;
    use crate::schema::topics::dsl::{id as topics_id, topics};
    if req_body.title.trim() == "" {
        return HttpResponse::BadRequest().body("please input title");
    }
    if req_body.body.trim() == "" {
        return HttpResponse::BadRequest().body("please input body");
    }
    let conn = db::get_connection();
    let verify_topic = topics
        .filter(topics_id.eq_any(&req_body.topics))
        .load::<Topics>(&conn);
    let mut invalid_topic = true;
    if let Ok(verify_topic) = verify_topic {
        if verify_topic.len() > 0 {
            invalid_topic = false
        }
    }
    if invalid_topic {
        return HttpResponse::BadRequest().body("Please enter the correct topic");
    }

    let post = NewPost {
        title: &req_body.title,
        body: &req_body.body,
        topics: &req_body.topics,
        published: &true,
        author: &claims.userid,
    };
    diesel::insert_into(posts)
        .values(&post)
        .get_result::<Post>(&conn)
        .expect("Error saving new post");
    HttpResponse::Ok().body("Success")
}

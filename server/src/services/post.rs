use crate::{db, models::Post};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostQuery {
    pub id: i32,
}

#[get("/post")]
pub async fn post(query: web::Query<PostQuery>) -> impl Responder {
    use crate::schema::posts::dsl::*;
    if db::can_connect() {
        let connection = db::get_connection();
        let results = posts
            .find(query.id)
            .get_result::<Post>(&connection)
            .expect("Error loading posts");
        HttpResponse::Ok().json(results)
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

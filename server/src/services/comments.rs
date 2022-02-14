use crate::{db, models::Comment};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommentQuery {
    pub id: i32,
}

#[get("/comments")]
async fn comments(query: web::Query<CommentQuery>) -> impl Responder {
    use crate::schema::comments::dsl::*;
    if db::can_connect() {
        let connection = db::get_connection();
        let results = comments
            .filter(post.eq(query.id))
            .order(id.desc())
            .limit(20)
            .load::<Comment>(&connection)
            .expect("Error loading comments");
        HttpResponse::Ok().json(results)
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

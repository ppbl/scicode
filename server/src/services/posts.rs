use crate::{db, models::Post};
use actix_web::{get, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/posts")]
async fn posts() -> impl Responder {
    use crate::schema::posts::dsl::*;
    let connection = db::get_connection();
    let results = posts
        .filter(published.eq(true))
        .order(id.desc())
        .limit(20)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    HttpResponse::Ok().json(results)
}

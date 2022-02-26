use crate::{db, models::Topics};
use actix_web::{get, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/topics")]
async fn topics() -> impl Responder {
    use crate::schema::topics::dsl::*;
    let conn = db::get_connection();
    let results = topics
        .limit(20)
        .load::<Topics>(&conn)
        .expect("Error loading topics");
    HttpResponse::Ok().json(results)
}

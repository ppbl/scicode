use crate::{db, schema::posts, services::post::PostQuery};
use actix_web::{delete, web, HttpResponse, Responder};
use diesel::prelude::*;

#[delete("/delete_post")]
async fn delete_post(query: web::Query<PostQuery>) -> impl Responder {
    if db::can_connect() {
        let connection = db::get_connection();
        let rows = diesel::delete(posts::table)
            .filter(posts::id.eq(query.id))
            .execute(&connection)
            .unwrap();
        HttpResponse::Ok().body(if rows == 1 {
            format!("post {} has been deleted", query.id)
        } else {
            "nothing need to do".to_string()
        })
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

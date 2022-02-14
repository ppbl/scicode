use crate::{db, models::PostAndUser};
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
    use crate::schema::users::dsl::{username, users};
    if db::can_connect() {
        let connection = db::get_connection();
        let results = users
            .inner_join(posts)
            .filter(id.eq(query.id))
            .select((id, title, body, author, create_at, username))
            .load::<PostAndUser>(&connection)
            .expect("Error loading posts");
        HttpResponse::Ok().json(&results[0])
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

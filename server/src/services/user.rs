use crate::{db, models::UserInfo};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserQuery {
    pub id: i32,
}

#[get("/user")]
pub async fn user(query: web::Query<UserQuery>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = db::get_connection();
    let user = users
        .find(query.id)
        .select((id, username, avatar_url, github_url))
        .first::<UserInfo>(&conn)
        .expect("Failed to query user");
    HttpResponse::Ok().json(&user)
}

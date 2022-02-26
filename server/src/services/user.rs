use crate::{auth::get_claims, db, models::UserInfo};
use actix_web::{get, http::header::AUTHORIZATION, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;

#[get("/user")]
pub async fn user(req: HttpRequest) -> impl Responder {
    use crate::schema::users::dsl::*;

    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let claims = get_claims(token.to_str().unwrap());
        let conn = db::get_connection();
        let user = users
            .find(claims.userid)
            .select((id, username, avatar_url, github_url))
            .first::<UserInfo>(&conn)
            .expect("Failed to query user");
        HttpResponse::Ok().json(&user)
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}

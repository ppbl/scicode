use crate::{db, models::CommentAndUser, schema::users::avatar_url};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommentQuery {
    pub id: i32,
}

#[get("/comments")]
async fn comments(query: web::Query<CommentQuery>) -> impl Responder {
    use crate::schema::{
        comments::dsl::*,
        users::dsl::{username, users},
    };
    let conn = db::get_connection();
    let results = comments
        .inner_join(users)
        .filter(post.eq(query.id))
        .order(id.desc())
        .limit(20)
        .select((id, body, create_at, (id, username, avatar_url)))
        .load::<CommentAndUser>(&conn)
        .expect("Error loading comments");
    HttpResponse::Ok().json(results)
}

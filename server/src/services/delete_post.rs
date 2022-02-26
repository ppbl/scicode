use crate::{auth::get_claims, db, models::Post, services::post::PostQuery};
use actix_web::{delete, http::header::AUTHORIZATION, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;

#[delete("/delete_post")]
async fn delete_post(query: web::Query<PostQuery>, req: HttpRequest) -> impl Responder {
    use crate::schema::comments;
    use crate::schema::posts::dsl::*;
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let claims = get_claims(token.to_str().unwrap());
        let conn = db::get_connection();
        let post = posts
            .find(query.id)
            .get_result::<Post>(&conn)
            .expect("Not found");
        if post.author == claims.userid {
            diesel::update(comments::table.filter(comments::post.eq(query.id)))
                .set(comments::post.eq(None::<i32>))
                .execute(&conn)
                .expect("Failed to delete comments's post id");
            diesel::delete(posts.filter(id.eq(query.id)))
                .execute(&conn)
                .expect("Failed to delete post");
            HttpResponse::Ok().body(format!("success"))
        } else {
            HttpResponse::Ok().body("No permission to delete")
        }
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}

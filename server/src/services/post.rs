use crate::{
    db,
    models::{PostAndUser, PostAndUserAndTopics, PostThumbs, Topics},
    services::sign_in::{Claims, SECRET},
};
use actix_web::{get, http::header::AUTHORIZATION, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostQuery {
    pub id: i32,
}

#[get("/post")]
pub async fn post(query: web::Query<PostQuery>, req: HttpRequest) -> impl Responder {
    use crate::schema::posts::dsl::*;
    use crate::schema::posts_thumbs;
    use crate::schema::topics::dsl::{id as topics_id, topics as topics_table};
    use crate::schema::users::dsl::{username, users};

    let conn = db::get_connection();
    let results = posts
        .inner_join(users)
        .filter(id.eq(query.id))
        .select((
            id,
            title,
            body,
            topics,
            (author, username),
            create_at,
            ups,
            downs,
        ))
        .load::<PostAndUser>(&conn)
        .expect("Error loading posts");
    let topics_ids = &results[0].topics;
    let topic_list = topics_table
        .filter(topics_id.eq_any(topics_ids))
        .load::<Topics>(&conn)
        .expect("Error loading posts");
    let mut res = PostAndUserAndTopics {
        topics: topic_list,
        id: results[0].id,
        title: results[0].title.clone(),
        body: results[0].body.clone(),
        author: results[0].author.clone(),
        create_at: results[0].create_at,
        ups: results[0].ups,
        downs: results[0].downs,
        voting: None,
    };
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let token = token.to_str().unwrap().split(" ").collect::<Vec<&str>>();
        let claims = decode::<Claims>(
            token[1],
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .unwrap()
        .claims;
        let thumbs = posts_thumbs::table
            .filter(posts_thumbs::post.eq(query.id))
            .filter(posts_thumbs::author.eq(claims.userid))
            .get_result::<PostThumbs>(&conn);
        if let Ok(thumbs) = thumbs {
            res.voting = thumbs.voting
        }
    }
    HttpResponse::Ok().json(&res)
}

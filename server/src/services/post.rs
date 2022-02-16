use crate::{
    db,
    models::{PostAndUser, PostAndUserAndTopics, Topics},
};
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
    use crate::schema::topics::dsl::{id as topics_id, topics as topics_table};
    use crate::schema::users::dsl::{username, users};
    let connection = db::get_connection();
    let results = posts
        .inner_join(users)
        .filter(id.eq(query.id))
        .select((id, title, body, topics, (author, username), create_at))
        .load::<PostAndUser>(&connection)
        .expect("Error loading posts");
    let topics_ids = &results[0].topics;
    let topic_list = topics_table
        .filter(topics_id.eq_any(topics_ids))
        .load::<Topics>(&connection)
        .expect("Error loading posts");
    let res = PostAndUserAndTopics {
        topics: topic_list,
        id: results[0].id,
        title: results[0].title.clone(),
        body: results[0].body.clone(),
        author: results[0].author.clone(),
        create_at: results[0].create_at,
    };
    HttpResponse::Ok().json(&res)
}

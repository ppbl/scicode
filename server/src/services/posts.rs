use crate::{
    db,
    models::{PostAndUser, Topics},
};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::Local;
use diesel::prelude::*;
use rust_decimal::prelude::*;
use serde::Deserialize;
use std::ops::Mul;

// reddit hot post algorithm
fn get_hot_value(ups: i64, downs: i64, publish_date: chrono::NaiveDateTime) -> i64 {
    let s = ups - downs;
    let sign = if s > 0 {
        1
    } else if s < 0 {
        -1
    } else {
        0
    };
    let order = Decimal::new(s, 0).abs().max(Decimal::new(1, 0)).log10();
    let seconds = (Local::now().naive_local() - publish_date).num_seconds();
    order.mul(Decimal::new(sign, 0)).to_i64().unwrap() + seconds / 45000
}

#[derive(Deserialize)]
struct PostsQuery {
    topic: Option<String>,
}

#[get("/posts")]
async fn posts(query: web::Query<PostsQuery>) -> impl Responder {
    use crate::schema::posts::dsl::*;
    use crate::schema::topics;
    use crate::schema::users::dsl::{avatar_url, id as user_id, username, users};
    let conn = db::get_connection();

    let mut topic_id = -1;
    if let Some(topic) = &query.topic {
        let result = topics::table
            .filter(topics::name.eq(topic))
            .get_result::<Topics>(&conn);
        match result {
            Ok(res) => topic_id = res.id,
            Err(_) => return HttpResponse::NotFound().body("404"),
        }
    }

    let mut results;
    if topic_id != -1 {
        results = posts
            .inner_join(users)
            .filter(topics.contains(vec![topic_id]))
            .filter(published.eq(true))
            .order(create_at.desc())
            .select((
                id,
                title,
                body,
                topics,
                (user_id, username, avatar_url),
                create_at,
                ups,
                downs,
            ))
            .load::<PostAndUser>(&conn)
            .expect("Error loading posts");
    } else {
        results = posts
            .inner_join(users)
            .filter(published.eq(true))
            .order(create_at.desc())
            .select((
                id,
                title,
                body,
                topics,
                (user_id, username, avatar_url),
                create_at,
                ups,
                downs,
            ))
            .load::<PostAndUser>(&conn)
            .expect("Error loading posts");
    }
    results.sort_by(|prev, curr| {
        get_hot_value(prev.ups as i64, prev.downs as i64, prev.create_at).cmp(&get_hot_value(
            curr.ups as i64,
            curr.downs as i64,
            curr.create_at,
        ))
    });
    HttpResponse::Ok().json(results)
}

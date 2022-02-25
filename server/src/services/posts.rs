use std::{cmp::Ordering, ops::Mul};

use crate::{
    db,
    models::{Post, PostAndUser},
    schema::users::{avatar_url, username},
};
use actix_web::{get, HttpResponse, Responder};
use chrono::Local;
use diesel::prelude::*;
use rust_decimal::prelude::*;

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

#[get("/posts")]
async fn posts() -> impl Responder {
    use crate::schema::posts::dsl::*;
    use crate::schema::users::dsl::{avatar_url, id as user_id, username, users};
    let conn = db::get_connection();

    let mut results = posts
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
    results.sort_by(|prev, curr| {
        let order = get_hot_value(curr.ups as i64, curr.downs as i64, curr.create_at)
            - get_hot_value(prev.ups as i64, prev.downs as i64, prev.create_at);
        if order > 0 {
            Ordering::Greater
        } else if order < 0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    HttpResponse::Ok().json(results)
}

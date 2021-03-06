use crate::{
    auth::{generate_token, Claims},
    db,
    models::User,
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    usize,
};

#[derive(Deserialize)]
struct Body {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Response<T> {
    status: &'static str,
    data: T,
}
type SignInResponse<'a> = Response<&'a str>;

fn get_after_days(n: u64) -> usize {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + n * 24 * 3600 * 1000) as usize
}

#[post("/sign_in")]
async fn sign_in(req_body: web::Json<Body>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = db::get_connection();
    let results = users
        .filter(username.eq(&req_body.username))
        .get_result::<User>(&conn);
    if let Ok(results) = results {
        let passwd = req_body.password.as_bytes();
        let matches = argon2::verify_encoded(&results.password.unwrap(), passwd).unwrap();
        if matches {
            let claims = Claims {
                userid: results.id,
                username: results.username,
                exp: get_after_days(7),
            };
            let token = generate_token(claims);
            HttpResponse::Ok().json(SignInResponse {
                status: "success",
                data: &token,
            })
        } else {
            HttpResponse::Ok().json(SignInResponse {
                status: "error",
                data: "账号或密码不匹配",
            })
        }
    } else {
        HttpResponse::Ok().json(SignInResponse {
            status: "error",
            data: "账号或密码不匹配",
        })
    }
}

use crate::{
    db,
    models::{NewUser, User},
    schema::users,
};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::Config;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    username: String,
    password: String,
}

#[post("/sign_up")]
async fn sign_up(req_body: web::Json<Body>) -> impl Responder {
    if db::can_connect() {
        let salt = nanoid!();
        let salt = salt.as_bytes();
        let password = req_body.password.as_bytes();
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
        let connection = db::get_connection();
        let user = NewUser {
            username: &req_body.username,
            password: &hash,
        };
        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&connection)
            .expect("Error saving new user");
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::InternalServerError().body("cannot connect to db")
    }
}

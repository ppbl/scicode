#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use serde::Deserialize;

mod db;
mod models;
mod schema;

use models::*;

#[get("/posts")]
async fn posts() -> impl Responder {
    use schema::posts::dsl::*;
    let connection = db::establish_connection();
    let results = posts
        .filter(published.eq(true))
        .order(id.desc())
        .limit(20)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    HttpResponse::Ok().json(results)
}

#[derive(Deserialize)]
struct PostQuery {
    id: i32,
}
#[get("/post")]
async fn post(query: web::Query<PostQuery>) -> impl Responder {
    use schema::posts::dsl::*;
    let connection = db::establish_connection();
    let results = posts
        .find(query.id)
        .get_result::<Post>(&connection)
        .expect("Error loading posts");
    HttpResponse::Ok().json(results)
}

#[derive(Deserialize)]
struct Body {
    title: String,
    body: String,
}

#[post("/create_post")]
async fn create_post(req_body: web::Json<Body>) -> impl Responder {
    let connection = db::establish_connection();
    println!("{}", req_body.title);
    db::create_post(&connection, &req_body.title, &req_body.body);
    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(posts).service(post).service(create_post))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

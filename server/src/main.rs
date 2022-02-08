#[macro_use]
extern crate diesel;

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use serde::Deserialize;

mod db;
mod models;
mod schema;

use models::*;

#[get("/posts")]
async fn posts() -> impl Responder {
    use schema::posts::dsl::*;
    if db::can_connect() {
        let connection = db::get_connection();
        let results = posts
            .filter(published.eq(true))
            .order(id.desc())
            .limit(20)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        HttpResponse::Ok().json(results)
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

#[derive(Deserialize)]
struct PostQuery {
    id: i32,
}
#[get("/post")]
async fn post(query: web::Query<PostQuery>) -> impl Responder {
    use schema::posts::dsl::*;
    if db::can_connect() {
        let connection = db::get_connection();
        let results = posts
            .find(query.id)
            .get_result::<Post>(&connection)
            .expect("Error loading posts");
        HttpResponse::Ok().json(results)
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

#[derive(Deserialize)]
struct Body {
    title: String,
    body: String,
}

#[post("/create_post")]
async fn create_post(req_body: web::Json<Body>) -> impl Responder {
    if db::can_connect() {
        let connection = db::get_connection();
        println!("{}", req_body.title);
        db::create_post(&connection, &req_body.title, &req_body.body);
        HttpResponse::Ok().body("success")
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

#[delete("/delete_post")]
async fn delete_post(query: web::Query<PostQuery>) -> impl Responder {
    if db::can_connect() {
        let connection = db::get_connection();
        let rows = db::delete_post(&connection, query.id);
        HttpResponse::Ok().body(format!("{} row(s) affected", rows))
    } else {
        HttpResponse::Ok().body("cannot connect to db")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(posts)
            .service(post)
            .service(create_post)
            .service(delete_post)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

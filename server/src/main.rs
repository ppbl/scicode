use actix_cors::Cors;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allowed_origin("http://localhost:8080").allowed_origin("http://127.0.0.1:8080");
        App::new().wrap(cors).service(hello).service(echo)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

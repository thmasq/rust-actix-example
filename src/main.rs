use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, Result};
use std::time::Instant;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html")))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let start_time = Instant::now();
    let response = HttpResponse::Ok().body(req_body.clone());
    let end_time = Instant::now();

    println!("Time taken for 'echo': {:?}", end_time - start_time);

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

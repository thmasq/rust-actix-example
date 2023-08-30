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
    let response_body = req_body.clone().to_lowercase();
    let end_time = Instant::now();
    let execution_time = end_time - start_time;

    let response_message = format!(
        "{}\nExecution Time: {:?}",
        response_body, execution_time
    );

    println!("Time taken for 'echo': {:?}", execution_time);

    HttpResponse::Ok()
        .content_type("text/plain") // Set content type to plain text
        .body(response_message)
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

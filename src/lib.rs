use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicitly given that we have
// become more familiar with `actix-web`.
// There is no performance difference! Just a stylistic choice :)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// pub fn run() -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind("127.0.0.1:8000")?
//         .run();
//     // No .await here!
//     Ok(server)
// }

// pub fn run(address: &str) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind(address)?
//         .run();
//     Ok(server)
// }

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

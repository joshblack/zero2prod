use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

// pub async fn run() -> std::io::Result<()> {
// HttpServer::new(|| {
// App::new()
// .route("/health_check", web::get().to(health_check))
// .route("/", web::get().to(greet))
// .route("/{name}", web::get().to(greet))
// })
// .bind("127.0.0.1:8000")?
// .run()
// .await
// }

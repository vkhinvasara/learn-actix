use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

pub mod login;
pub mod register;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(register::register).service(login::login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

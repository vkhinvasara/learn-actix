use actix_web::{middleware::Logger,  App, HttpServer};
pub mod login;
pub mod register;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register::register)
            .service(login::login)
            .wrap(Logger::default())
            .wrap(login::session_middleware())

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::*;

#[post("/send_email")]
async fn send_email(email: String) -> HttpResponse {
    HttpResponse::Ok().body(email)
}

#[get("/")]
async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(send_email)
        .service(say_hello)
    }).bind(("127.0.0.1",8080))?
    .run()
    .await  
}
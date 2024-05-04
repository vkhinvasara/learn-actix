use actix_web::*;

#[get("/")]
async fn say_hello() -> impl Responder{
	HttpResponse::Ok().body("Hello world!")
}

async fn say_juvinile_stuff() -> impl Responder{
	HttpResponse::Ok().body("I am a child")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	HttpServer::new(||{
		App::new()
			.service(say_hello)
			.route("/child", web::get().to(say_juvinile_stuff))
	}).bind(("127.0.0.1", 8080))? 
	.run()
	.await
}
mod ws;
mod lobby;
use lobby::Lobby;
mod message;
mod start_connection;
use start_connection::start_connection as start_connection_route;
use actix::Actor;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route) //register our route. rename with "as" import or naming conflict
            .app_data(actix_web::web::Data::new(chat_server.clone())) //register the lobby
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
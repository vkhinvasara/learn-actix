use crate::ws::WsConn;
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;


pub async fn start_connection(req: HttpRequest, stream: web::Payload,`` data: web::Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
	let room = req.match_info().get("room").unwrap();
	let room = Uuid::parse_str(room).unwrap();
	let resp = ws::start(WsConn::new(room, data.get_ref().clone()), &req, stream);
	resp
}
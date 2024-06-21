use crate::ws::WsConn;
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::{get, web, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;
use actix_web::Responder;


#[get("/{group_id}")]
pub async fn start_connection(req: HttpRequest, stream: web::Payload, group_id: web::Path<Uuid>,data: web::Data<Addr<Lobby>>) -> impl Responder{
	let ws = WsConn::new(group_id.into_inner(), data.get_ref().clone());
	let resp = ws::start(ws, &req, stream);
	resp.unwrap()
}
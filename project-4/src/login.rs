use actix_web::{web, post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
	pub username: String,
	pub password: String,
}
#[post("/login"), data = "<login>"]
pub async fn login(login: web::Json<Login>){
	let login = login.into_inner();
	
}
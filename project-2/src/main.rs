use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize, Serialize)]
struct EmailData {
    name: String,
    email: String,
}

struct Store {
    items: Mutex<HashMap<String, EmailData>>,
}

#[get("/get_email/{name}")]
async fn get_email(name: web::Path<String>, data: web::Data<Store>) -> impl Responder {
    let items = data.items.lock().unwrap();
    match items.get(&name.to_string()) {
        Some(email) => HttpResponse::Ok().json(email),
        None => HttpResponse::NotFound().body("Email not found"),
    }
}

#[post("/send_email")]
async fn send_email(item: web::Json<EmailData>, data: web::Data<Store>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let email_data = item.into_inner();
    if email_data.email.is_empty() && email_data.name.is_empty() {
        HttpResponse::BadRequest().body("Email is empty")
    } else {
        items.insert(email_data.name.clone(), email_data);
        HttpResponse::Ok().body("Email sent successfully")
    }
}

#[put("/update_email/{name}")]
async fn update_email(name: web::Path<String>, item: web::Json<EmailData>, data: web::Data<Store>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let email_data = item.into_inner();
    if email_data.email.is_empty() && email_data.name.is_empty() {
        HttpResponse::BadRequest().body("Email is empty")
    } else {
        items.insert(name.to_string(), email_data);
        HttpResponse::Ok().body("Email updated successfully")
    }
}

#[delete("/delete_email/{name}")]
async fn delete_email(name: web::Path<String>, data: web::Data<Store>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    match items.remove(&name.to_string()) {
        Some(_) => HttpResponse::Ok().body("Email deleted successfully"),
        None => HttpResponse::NotFound().body("Email not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store = web::Data::new(Store{
        items: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .service(send_email)
            .service(get_email)
            .service(update_email)
            .service(delete_email)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

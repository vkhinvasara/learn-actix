use std::collections::HashMap;

use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
#[post("/login")]
pub async fn login(login: web::Json<Login>) -> impl Responder {
    let login = login.into_inner();
    let username = login.username;
    let password = hash(&login.password, DEFAULT_COST).unwrap();

    let client = DynamoDbClient::new(Region::ApSouth1);
    let mut key = HashMap::new();
    key.insert(
        "username".to_string(),
        AttributeValue {
            s: Some(username.clone()),
            ..Default::default()
        },
    );
    key.insert(
        "password".to_string(),
        AttributeValue {
            s: Some(password.clone()),
            ..Default::default()
        },
    );
    let input = GetItemInput {
        table_name: "customer_login_details".to_string(),
        key,
        ..Default::default()
    };
    match client.get_item(input).await {
        Ok(output) => match output.item {
            Some(item) => {
                let password = item.get("password").unwrap().s.as_ref().unwrap();
                if verify(&login.password, password).unwrap() {
                    HttpResponse::Ok().body("Successfully logged in")
                } else {
                    HttpResponse::Unauthorized().body("Invalid password")
                }
            }
            None => HttpResponse::Unauthorized().body("Invalid username"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in logging in"),
    };
    "login"
}

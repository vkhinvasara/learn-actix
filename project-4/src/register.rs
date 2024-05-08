use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use dotenv::dotenv;

#[derive(Deserialize)]
struct CustomerDetails {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/register")]
pub async fn register(register: web::Json<CustomerDetails>) -> impl Responder {
    let register = register.into_inner();
    let username = register.username;
    let password = hash(&register.password, DEFAULT_COST).unwrap();
    let client = DynamoDbClient::new(Region::ApSouth1);

    let mut item = HashMap::new();
    item.insert(
        "username".to_string(),
        AttributeValue {
            s: Some(username.clone()),
            ..Default::default()
        },
    );
    item.insert(
        "password".to_string(),
        AttributeValue {
            s: Some(password.clone()),
            ..Default::default()
        },
    );

    if check_if_registerd(username.clone()).await {
        return HttpResponse::Conflict().body("User already exists");
    }

    let input = PutItemInput {
        table_name: "customer_login_details".to_string(),
        item,
        ..Default::default()
    };
    match client.put_item(input).await {
        Ok(_) => {
            let token = get_token(username.clone()).await;
            HttpResponse::Ok().body(token)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error in registering"),
    }
}

async fn get_token(username: String) -> String {
    dotenv().ok();
    let claims = Claims {
        sub: username,
        exp: 10000000000,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("SECRET").unwrap().as_ref()),
    ).unwrap();
    token
}
async fn check_if_registerd(username: String) -> bool {
    let client = DynamoDbClient::new(Region::ApSouth1);
    let mut key = HashMap::new();
    key.insert(
        "username".to_string(),
        AttributeValue {
            s: Some(username),
            ..Default::default()
        },
    );
    let input = GetItemInput {
        table_name: "customer_login_details".to_string(),
        key,
        ..Default::default()
    };
    match client.get_item(input).await {
        Ok(output) => {
            if output.item.is_some() {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

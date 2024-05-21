use actix_web::{cookie::{Cookie, Key, SameSite}, post, web, HttpResponse, HttpResponseBuilder, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use actix_session::{config::{BrowserSession, CookieContentSecurity}, storage::CookieSessionStore, SessionMiddleware};

#[derive(Deserialize)]
struct CustomerDetails {
     username: String,
     password: String,
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
            let mut response = HttpResponse::Ok();
            set_session(&mut response, token.clone()).await;
            response.body(token)
        }
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
        &EncodingKey::from_secret(env::var("JWT_SECRET_KEY").unwrap().as_ref()),
    )
    .unwrap();
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

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore>{
    dotenv().ok();
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(env::var("COOKIE_SESSION_KEY").unwrap().as_ref()),
    ).cookie_name(String::from("JWT token"))
    .cookie_secure(true)
    .session_lifecycle(BrowserSession::default())
    .cookie_same_site(SameSite::Strict)
    .cookie_content_security(CookieContentSecurity::Private)
    .cookie_http_only(true)
    .build()
}

pub async fn set_session(response: &mut HttpResponseBuilder, token: String){
    let cookie = Cookie::build("JWT token", token)
    .secure(true) // Only send the cookie over HTTPS
    .http_only(true) // Don't allow JavaScript to access the cookie
    .finish();
    response.cookie(cookie);
}
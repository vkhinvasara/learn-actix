use crate::register::CustomerDetails;
use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    Session, SessionMiddleware,
};
use actix_web::{
    cookie::{Cookie, Key, SameSite},
    post, web, HttpResponse, HttpResponseBuilder, Responder,
};
use bcrypt::verify;
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[post("/login")]
pub async fn login(customer: web::Json<CustomerDetails>, session: Session) -> impl Responder {
    let customer = customer.into_inner();
    let username = &customer.username;
    let password = &customer.password;
    let client = DynamoDbClient::new(Region::ApSouth1);

    let mut key: HashMap<String, rusoto_dynamodb::AttributeValue> = HashMap::new();
    key.insert(
        "username".to_string(),
        rusoto_dynamodb::AttributeValue {
            s: Some(username.clone()),
            ..Default::default()
        },
    );
    let input = rusoto_dynamodb::GetItemInput {
        table_name: "customer_login_details".to_string(),
        key,
        ..Default::default()
    };

    match client.get_item(input).await {
        Ok(output) => match output.item {
            Some(item) => {
                let stored_password = item.get("password").unwrap().s.as_ref().unwrap();
                if verify(password, stored_password).unwrap() {
                    let token = get_token(username.clone());
                    let mut response = HttpResponse::Ok();
                    set_session(&mut response, token.clone());
                    session.insert("JWT_TOKEN", token.clone()).unwrap();
                    return HttpResponse::Ok().body("Login successful");
                } else {
                    return HttpResponse::Unauthorized().body("Invalid password");
                }
            }
            None => return HttpResponse::Unauthorized().body("Invalid username"),
        },
        Err(_) => return HttpResponse::InternalServerError().body("Error in login"),
    }
}

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    dotenv().ok();
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::from(env::var("COOKIE_SESSION_KEY").unwrap().as_ref()),
    )
    .cookie_name(String::from("JWT token"))
    .cookie_secure(true)
    .session_lifecycle(BrowserSession::default())
    .cookie_same_site(SameSite::Strict)
    .cookie_content_security(CookieContentSecurity::Private)
    .cookie_http_only(true)
    .build()
}

pub fn set_session(response: &mut HttpResponseBuilder, token: String) {
    let cookie = Cookie::build("JWT_TOKEN", token)
        .secure(true)
        .http_only(true)
        .finish();
    response.cookie(cookie);
}
pub fn get_token(username: String) -> String {
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

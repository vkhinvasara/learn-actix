use crate::login::Claims;
use crate::register::UserRole;
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput};
use std::collections::HashMap;

fn check_user_role(token: &str) -> Result<UserRole, &str> {
    let decoding_key = DecodingKey::from_secret("JWT_SECRET_KEY".as_ref());
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(token_data) => {
            let user_role = &token_data.claims.role;
            match user_role.as_str() {
                "Admin" => Ok(UserRole::Admin),
                "User" => Ok(UserRole::User),
                _ => Err("Invalid role"),
            }
        }
        Err(_) => Err("Error in decoding token"),
    }
}

fn get_sesion_token(session: &Session) -> String{
    let token = session.get("JWT_TOKEN").unwrap();
    print!("{:?}", &token);
    match token {
        Some(token) => token as String,
        None => "".to_string().to_string(),
    }
}

#[post("/getuserdetails/{user_id}")]
pub async fn get_user_details(session: Session, user_id: web::Path<String>) -> impl Responder {
    let token = get_sesion_token(&session);
    print!("{:?}", &token);
    let role = check_user_role(&token).expect("Error in decoding token");
    match role {
        UserRole::Admin => {
            let client = DynamoDbClient::new(Region::ApSouth1);
            let mut key: HashMap<String, rusoto_dynamodb::AttributeValue> = HashMap::new();
            key.insert(
                "id".to_string(),
                rusoto_dynamodb::AttributeValue {
                    n: Some(user_id.clone()),
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
                        let username = item.get("username").unwrap().s.as_ref().unwrap();
                        let role = item.get("role").unwrap().s.as_ref().unwrap();
                        HttpResponse::Ok().body(format!("Username: {}, Role: {}", username, role))
                    }
                    None => HttpResponse::NotFound().body("User not found"),
                },
                Err(_) => {
                    HttpResponse::InternalServerError().body("Error in fetching user details")
                }
            }
        }
        UserRole::User => {
            HttpResponse::Forbidden().body("You are not authorized to view this page")
        }
    }
}

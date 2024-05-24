use std::collections::HashMap;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::register::{CustomerDetails,get_token, set_session};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};


#[post("/login")]
pub async fn login(customer: web::Json<CustomerDetails>) -> impl Responder {
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
        Ok(output) => {
            match output.item {
                Some(item) => {
                    let stored_password = item.get("password").unwrap().s.as_ref().unwrap();
                    if verify(password, stored_password).unwrap() {
                        let token = get_token(username.clone()).await;
                        let mut response = HttpResponse::Ok();
                        set_session(&mut response, token.clone()).await;
                        response.body(token);
                        return HttpResponse::Ok().body("Login successful");
                    }else{
                        return HttpResponse::Unauthorized().body("Invalid password");
                    }
                }
                None => return HttpResponse::Unauthorized().body("Invalid username"),
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("Error in login"),
    }
}
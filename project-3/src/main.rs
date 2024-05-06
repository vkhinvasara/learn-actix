use std::collections::HashMap;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput, PutItemInput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Address{
    street: String,
    city: String,
    state: String,
    zip: String,
}
#[derive(Serialize, Deserialize)]  
struct Customer{
    id: String,
    name: String,
    age: u8,
    active: bool,
    address: Address,
}

#[post("/register_customer")]
async fn register_customer(customer: web::Json<Customer>)-> impl Responder{
    let client = DynamoDbClient::new(Region::ApSouth1);
    let customer = customer.into_inner();
    if customer.id.is_empty() 
    && customer.name.is_empty() 
    && customer.age == 0
    && customer.active == false{
       return  HttpResponse::BadRequest().body("Id is required");
    }

    let mut item = HashMap::new();
    item.insert("customer_key".to_string(), AttributeValue{
        s: Some(customer.id.to_string()),
        ..Default::default()
    });
    item.insert("name".to_string(), AttributeValue{
        s: Some(customer.name),
        ..Default::default()
    });
    item.insert("age".to_string(), AttributeValue{
        n: Some(customer.age.to_string()),
        ..Default::default()
    });
    item.insert("active".to_string(), AttributeValue{
        bool: Some(customer.active),
        ..Default::default()
    });
    item.insert("address".to_string(),AttributeValue{
        m: Some({
            let mut address = HashMap::new();
            address.insert("street".to_string(), AttributeValue{
                s: Some(customer.address.street),
                ..Default::default()
            });
            address.insert("city".to_string(), AttributeValue{
                s: Some(customer.address.city),
                ..Default::default()
            });
            address.insert("state".to_string(), AttributeValue{
                s: Some(customer.address.state),
                ..Default::default()
            });
            address.insert("zip".to_string(), AttributeValue{
                s: Some(customer.address.zip),
                ..Default::default()
            });
            address
        }),
        ..Default::default()
    });
    let put_item_input = PutItemInput{
        item: item,
        table_name: "Customer".to_string(),
        ..Default::default()
    };
    match client.put_item(put_item_input).await{
        Ok(_) => HttpResponse::Ok().body("Customer registered successfully"),
        Err(error) => {
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
    
    
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(register_customer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
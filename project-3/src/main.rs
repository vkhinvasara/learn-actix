use std::collections::HashMap;

use actix_web::{dev::Path, get, post, delete, put, web::{self, get}, App, HttpResponse, HttpServer, Responder};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, DeleteItemInput, PutItemInput};
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

#[get("/get_customer/{id}")]
async fn get_customer(id: web::Path<String>)-> impl Responder{
    let client = DynamoDbClient::new(Region::ApSouth1);
    let get_item_input = GetItemInput{ 
        key: {
            let mut key = HashMap::new();
            key.insert("customer_key".to_string(), AttributeValue{
                s: Some(id.to_string()),
                ..Default::default()
            });
            key
        },
        table_name: "Customer".to_string(),
        ..Default::default()
    };
    match client.get_item(get_item_input).await{
        Ok(output) => {
            match output.item{
                Some(item) => {
                    let name = item.get("name").unwrap().s.as_ref().unwrap();
                    let age = item.get("age").unwrap().n.as_ref().unwrap();
                    let active = item.get("active").unwrap().bool.as_ref().unwrap();
                    let address = item.get("address").unwrap().m.as_ref().unwrap();
                    let street = address.get("street").unwrap().s.as_ref().unwrap();
                    let city = address.get("city").unwrap().s.as_ref().unwrap();
                    let state = address.get("state").unwrap().s.as_ref().unwrap();
                    let zip = address.get("zip").unwrap().s.as_ref().unwrap();
                    let customer = Customer{
                        id: id.to_string(),
                        name: name.to_string(),
                        age: age.parse().unwrap(),
                        active: *active,
                        address: Address{
                            street: street.to_string(),
                            city: city.to_string(),
                            state: state.to_string(),
                            zip: zip.to_string(),
                        }
                    };
                    HttpResponse::Ok().json(customer)
                },
                None => HttpResponse::NotFound().body("Customer not found")
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[delete("/delete_customer/{id}")]
async fn delete_customer(id: web::Path<String>)-> impl Responder{
    let client = DynamoDbClient::new(Region::ApSouth1);
    let delete_item_input = DeleteItemInput{
        key: {
            let mut key = HashMap::new();
            key.insert("customer_key".to_string(), AttributeValue{
                s: Some(id.to_string()),
                ..Default::default()
            });
            key
        },
        table_name: "Customer".to_string(),
        ..Default::default()
    };
    match client.delete_item(delete_item_input).await{
        Ok(_) => HttpResponse::Ok().body("Customer deleted successfully"),
        Err(error) => {
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}

#[put("/update_customer")]

async fn update_customer(customer: web::Json<Customer>)-> impl Responder{
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
        Ok(_) => HttpResponse::Ok().body("Customer updated successfully"),
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
            .service(get_customer)
            .service(delete_customer)
            .service(update_customer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
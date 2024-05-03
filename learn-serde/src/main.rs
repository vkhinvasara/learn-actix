use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};


#[derive(Serialize, Deserialize, Debug)]

struct Book{
    title: String,
    author: String,
    year: i32,
    pages: i32,
}

#[derive(Serialize, Deserialize, Debug)]

struct Library{
    books: Vec<Book>,
}

fn main(){
    let book1 = Book{
        title: String::from("The Kitchen Confidential"),
        author: String::from("Anthony Bourdain"),
        year: 2019,
        pages: 300,
    };
    let library = Library{
        books: vec![book1],
    };

    let library_json = to_string_pretty(&library);
    if library_json.is_ok(){
        println!("{}",library_json.unwrap());
    }else if library_json.is_err(){
        println!("{:?}",library_json.err());
    }
    let library_json = to_string(&library);
    if library_json.is_ok(){
        println!("{}",library_json.unwrap());
    }else if library_json.is_err(){
        println!("{:?}",library_json.err());
    }
    let input = r#"
          {
            "title": "The Kitchen Confidential",
            "author": "Anthony Bourdain",
            "year": 2019,
            "pages": 300
          }
        "#;
      deserialize_library(input);

}

fn deserialize_library(input: &str){
    let book1 = from_str::<Book>(input);
    if book1.is_ok(){
        println!("{:?}",book1.unwrap());
    }else if book1.is_err(){
        println!("{:?}", book1.err());
    }
}   
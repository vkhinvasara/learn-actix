use serde::{Serialize, Deserialize};
use serde_json::{to_string, to_string_pretty};


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
}
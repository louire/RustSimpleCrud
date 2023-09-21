use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    name: String,
}


#[tokio::main]
async fn main(){
    let user = User{
        name: "Loui Recio".to_string(),
        age: 23,
        email: "lreciopersonal@gmail.com".to_string()
    };

    let firebase = Firebase::new("https://rustcrudbor-default-rtdb.firebaseio.com/").unwrap();
}
    

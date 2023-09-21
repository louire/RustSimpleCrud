use std::collections::HashMap;
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted!");

}


use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = User::new(
        "Loui Recio".to_string(),
        23,
        "lreciopersonal@gmail.com".to_string(),
    );
    let firebase = Firebase::new("https://fbrustcrud-default-rtdb.firebaseio.com/").unwrap();

    let response = user.set_user(&firebase).await;

    let mut user = response.get_user(&firebase).await;
    println!("{:?}", user);

    let all_users = get_users(&firebase).await;
    println!("{:?}", all_users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = user.update_user(&firebase, &response.name).await;
    println!("{:?}", updated_user);

    response.delete_user(&firebase).await;
    println!("User deleted!");
}
impl User {
    fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }

    async fn set_user(&self, firebase_client: &Firebase) -> Response {
        let firebase = firebase_client.at("users");
        let users = firebase.set::<User>(self).await;
        return string_to_response(&users.unwrap().data);
    }

    async fn update_user(&self, firebase_client: &Firebase, id: &String) -> User {
        let firebase = firebase_client.at("users").at(&id);
        let _user = firebase.update::<User>(self).await;
        return string_to_user(&_user.unwrap().data);
    }
}
impl Response {
    async fn delete_user(&self, firebase_client: &Firebase) {
        let firebase = firebase_client.at("users").at(&self.name);
        let _result = firebase.delete().await;
    }
    async fn get_user(&self, firebase_client: &Firebase) -> User {
        let firebase = firebase_client.at("users").at(&self.name);
        let user = firebase.get::<User>().await;
        return user.unwrap();
    }
}
async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}
// convert a string to a response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

//convert a string to a user
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}

use crate::samples::query_users::query_users;
use crate::user::structs::User;
use serde_json;
use serde_json::{to_value, Value};

pub fn print_all_users() {
    println!("\n#### print_all_users() ####");

    let users = query_users();
    for user in users {
        // convert to json
        let user_string = serde_json::to_string(&serde_json::json!(&user)).unwrap();
        println!("\nuser_string: {:?}", user_string);
        print_user(&user_string);
    }
}

pub fn print_user(user_string: &str) {
    println!("\n#### print_user(user: User) ####");
    // deserialize from json
    let user: User = serde_json::from_str(user_string).unwrap();

    println!("User: id: {}", user.id);
    println!("----: name: {}", user.name);
    println!("----: email: {}", user.email);
    println!("----: email_verified: {:?}", user.email_verified);
    println!("----: image: {:?}", user.image);
}

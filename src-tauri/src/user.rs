use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};
use std::fs;
pub struct User {
   pub username: String,
    pub password: String,
    pub logged_in: bool
}

impl User {
   pub fn new_user(username: String, password: String) -> User {
        User {
            username, 
            password, 
            logged_in: true
        }
    }
 
    pub fn login(&self) -> Result<String, String> {
        let mut user_file = File::open("user.json")
            .map_err(|err| format!("Error opening user.json: {}", err))?;
    
        let mut user_data = String::new();
        user_file
            .read_to_string(&mut user_data)
            .map_err(|err| format!("Error reading user data: {}", err))?;
    
        let user_data_json: Value = serde_json::from_str(&user_data)
            .map_err(|err| format!("Error parsing user data as JSON: {}", err))?;
        

        if user_data_json["username"] == self.username && user_data_json["password"] == self.password {
            Ok(format!("Logged in... Welcome {}", self.username))
        } else {
            Err("Invalid credentials!".to_string())
        }
    }

    // pub fn user_details(&self) -> String {


    //         format!("Username {} \n Password {}", data["username"], data["password"])

    // }
   
}
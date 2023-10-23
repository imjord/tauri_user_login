use std::fs::File;
use std::io::prelude::*;
use serde_json::json;


pub fn create_user_file(username: &String, password: &String) -> std::io::Result<()> {
    
    let mut user_file = File::create("user.json")?;
   
    let data = json!({
        "username": username.trim(),
        "password": password.trim()
    });

    let json_str = serde_json::to_string(&data)?;

    user_file.write_all(json_str.as_bytes())?;
    
    Ok(())
}
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod user;
mod create_user_file;
use create_user_file::create_user_file;
use user::User;
use window_shadows::set_shadow;
use tauri::Manager;
use std::path::Path;


#[tauri::command]
fn check_if_logged_in_before() -> bool {
    let user_path = Path::new("user.json").is_file();

        if user_path {
            true
        } else {
            false
        }

}


#[tauri::command]
fn sign_user_in(username: &str, password: &str) -> String {
    let new_user = User::new_user(username.to_string(), password.to_string());
        // check if theres a user.json 
        let user_path = Path::new("user.json").is_file();
        // new_member.print_user();
           if  user_path { 
              match new_user.login() { // match on the login function to either get valid creds or not
                Ok(message) => message,
                Err(error) => error
              }
          } else {
            create_user_file(&username.to_string(), &password.to_string());
            format!("{} is now logged in!", username)
          }

}



fn main() {
    tauri::Builder::default()
        .setup(|app| { // this creates rounded corners hehe
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        }).invoke_handler(tauri::generate_handler![sign_user_in, check_if_logged_in_before])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

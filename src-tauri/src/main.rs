#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod emailsender;
mod save_auth;
mod load_auth;

use save_auth::save_auth;
use load_auth::load_auth;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_auth,
            load_auth,
            emailsender::send_bulk_email
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

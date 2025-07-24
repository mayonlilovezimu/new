use serde::{Deserialize, Serialize};
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;
use tauri::command;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[command]
pub fn load_auth() -> Result<serde_json::Value, String> {
    let base_dirs = BaseDirs::new().ok_or("Не удалось получить папку пользователя")?;
    let mut path = PathBuf::from(base_dirs.data_dir());
    path.push("emailsender");
    path.push("auth.json");

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Ошибка чтения файла: {}", e))?;
    let auth_data: AuthData = serde_json::from_str(&content)
        .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    Ok(json!(auth_data))
}

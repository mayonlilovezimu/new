use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use tauri::command;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct AuthData {
    email: String,
    password: String,
}

#[command]
pub fn save_auth(email: String, password: String) -> Result<String, String> {
    let project_dirs = ProjectDirs::from("com", "menu", "emailsender")
        .ok_or("Не удалось определить директорию приложения")?;
    let mut path = PathBuf::from(project_dirs.data_local_dir());

    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Ошибка создания папки: {}", e))?;
    }

    path.push("auth.json");

    let auth_data = AuthData { email, password };
    let data = serde_json::to_string_pretty(&auth_data).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| format!("Ошибка записи файла: {}", e))?;

    Ok(format!("Данные успешно сохранены в {:?}", path))
}

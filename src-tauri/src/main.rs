#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod emailsender;
mod save_auth;
mod load_auth;

use save_auth::save_auth;
use load_auth::load_auth;

use axum::{
    response::IntoResponse,
    routing::get,
    Router,
    body::Bytes,
    http::{StatusCode, header, HeaderMap},
};
use std::net::SocketAddr;
use std::thread;
use axum::Server;

use tauri_plugin_fs::init;

#[tokio::main]
async fn main() {
    // Запуск сервера трекинг-пикселя в отдельном потоке
    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let app = Router::new().route("/pixel", get(pixel_handler));
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            println!("Сервер трекинга запущен на http://{}", addr);
            Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
    });

    // Запуск Tauri с подключением плагина fs без аргументов
    tauri::Builder::default()
        .plugin(init())  // здесь без параметров
        .invoke_handler(tauri::generate_handler![
            save_auth,
            load_auth,
            emailsender::send_bulk_email
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка при запуске Tauri");
}

async fn pixel_handler() -> impl IntoResponse {
    const PIXEL: &[u8; 43] = &[
        71, 73, 70, 56, 57, 97, 1, 0, 1, 0, 128, 0, 0, 0, 0, 0,
        255, 255, 255, 33, 249, 4, 1, 0, 0, 1, 0, 44, 0, 0, 0,
        0, 1, 0, 1, 0, 0, 2, 2, 68, 1, 0, 59,
    ];

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/gif".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());

    (StatusCode::OK, headers, Bytes::from_static(PIXEL))
}

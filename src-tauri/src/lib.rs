use tauri::{generate_handler, Builder, Runtime};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::address::AddressError;
use serde::Deserialize;

#[derive(Deserialize)]
struct EmailConfig {
    email: String,
    password: String,
    smtp_server: String,
    port: u16,
}

#[tauri::command]
fn send_test_email(config: EmailConfig) -> Result<String, String> {
    let creds = Credentials::new(config.email.clone(), config.password.clone());

    let mail = Message::builder()
        .from(config.email.parse().map_err(|e: AddressError| e.to_string())?)
        .to(config.email.parse().map_err(|e: AddressError| e.to_string())?)
        .subject("Тестовое письмо от emailsender")
        .body(String::from("<b>Это тестовое письмо</b> — отправлено через lettre!"))
        .map_err(|e| e.to_string())?;

    let mailer = SmtpTransport::relay(&config.smtp_server)
        .map_err(|e| e.to_string())?
        .port(config.port)
        .credentials(creds)
        .build();

    mailer.send(&mail).map_err(|e| e.to_string())?;

    Ok("Письмо отправлено успешно!".to_string())
}

pub fn run<R: Runtime>() {
    Builder::default()
        .invoke_handler(generate_handler![send_test_email])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска приложения");
}

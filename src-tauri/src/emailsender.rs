use tauri::command;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailConfig {
    pub email: String,
    pub password: String,
    pub smtp_server: String,
    pub port: u16,
    pub recipients: Vec<String>,
    pub subject: String,
    pub mail_type: String,
    pub body: Option<String>,
    pub html_body: Option<String>,
    pub add_text_to_html: Option<bool>,
    pub text_for_html: Option<String>,
}

#[command]
pub fn send_bulk_email(config: EmailConfig) -> Result<String, String> {
    let creds = Credentials::new(config.email.clone(), config.password.clone());

    let mailer = SmtpTransport::relay(&config.smtp_server)
        .map_err(|e| format!("Ошибка SMTP: {}", e))?
        .port(config.port)
        .credentials(creds)
        .build();

    let mut success = 0;
    let mut failed = 0;

    for to in &config.recipients {
        // Парсим адреса, возвращаем ошибку, если невалидные
        let from_addr = config.email.parse().map_err(|e| format!("Неверный email отправителя: {}", e))?;
        let to_addr = to.parse().map_err(|e| format!("Неверный email получателя: {}", e))?;

        let email_result = match config.mail_type.as_str() {
            "html" => {
                // Формируем тело html с добавлением скрытого пикселя
                let email_html = if let Some(mut html) = config.html_body.clone() {
                    let pixel_img = format!(
                        r#"<img src="http://localhost:3000/pixel?email={}" width="1" height="1" style="display:none;" alt="" />"#,
                        to
                    );
                    html.push_str(&pixel_img);
                    html
                } else {
                    "".to_string()
                };

                let html_part = SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(email_html);

                let email_builder = Message::builder()
                    .from(from_addr)
                    .to(to_addr)
                    .subject(&config.subject);

                if config.add_text_to_html.unwrap_or(false) {
                    let text_part = SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(config.text_for_html.clone().unwrap_or_default());

                    email_builder.multipart(
                        MultiPart::alternative()
                            .singlepart(text_part)
                            .singlepart(html_part)
                    )
                } else {
                    email_builder.multipart(
                        MultiPart::alternative()
                            .singlepart(html_part)
                    )
                }
            },
            _ => {
                Message::builder()
                    .from(from_addr)
                    .to(to_addr)
                    .subject(&config.subject)
                    .body(config.body.clone().unwrap_or_default())
            }
        };

        let email = email_result.map_err(|e| format!("Ошибка создания письма: {}", e))?;

        match mailer.send(&email) {
            Ok(_) => success += 1,
            Err(_) => failed += 1,
        }
    }

    Ok(format!("Успешно отправлено: {}, ошибок: {}", success, failed))
}

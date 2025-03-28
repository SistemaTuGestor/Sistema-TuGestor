use lettre::message::{header, Mailbox, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpTransport, Transport};
use oauth2::{ClientId, ClientSecret, Scope, TokenResponse}; // Añadido TokenResponse aquí
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct EmailData {
    destinatarios: Vec<String>,
    asunto: String,
    mensaje: String,
    archivo_adjunto: Option<String>,
}

fn log_error(context: &str, error: impl std::fmt::Display) -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let error_msg = format!("[{}] {}: {}", timestamp, context, error);
    println!("{}", error_msg);
    error_msg
}

async fn obtener_token_oauth() -> Result<String, String> {
    let client = BasicClient::new(
        ClientId::new("".to_string()),
        Some(ClientSecret::new("".to_string())),
        oauth2::AuthUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string())
            .map_err(|e| log_error("Error creando AuthUrl", e))?,
        Some(oauth2::TokenUrl::new("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string())
            .map_err(|e| log_error("Error creando TokenUrl", e))?),
    );

    let token = client
        .exchange_client_credentials()
        .add_scope(Scope::new("https://outlook.office365.com/.default".to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|e| log_error("Error obteniendo token OAuth", e))?;

    Ok(token.access_token().secret().to_string())
}

#[tauri::command]
pub async fn enviar_correo(data: EmailData) -> Result<(), String> {

    // Destinatarios hardcodeados (opcional)
    let destinatarios_finales = if data.destinatarios.is_empty() {
        vec![
            "correo1@ejemplo.com".to_string(),
            "correo2@ejemplo.com".to_string(),
            "correo3@ejemplo.com".to_string()
        ]
    } else {
        data.destinatarios
    };

    // 1. Obtener token OAuth
    let access_token = obtener_token_oauth().await?;

    // 2. Parseo de direcciones
    let from_mailbox: Mailbox = "jhosephs_lizarazom@javeriana.edu.co"
        .parse()
        .map_err(|e| log_error("Error parseando remitente", e))?;

    let to_mailboxes: Vec<Mailbox> = destinatarios_finales
        .iter()
        .map(|dest| dest.parse().map_err(|e| log_error("Error parseando destinatario", e)))
        .collect::<Result<Vec<_>, _>>()?;

    // 3. Construcción base del mensaje
    let mut email_builder = Message::builder()
        .from(from_mailbox)
        .subject(&data.asunto);

    for mailbox in &to_mailboxes {
        email_builder = email_builder.to(mailbox.clone());
    }

    // 4. Manejo del adjunto si existe
    let multipart = if let Some(archivo_path) = &data.archivo_adjunto {
        let file_body = std::fs::read(archivo_path)
            .map_err(|e| log_error("Error leyendo archivo adjunto", e))?;
        
        let file_name = Path::new(archivo_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("archivo.pdf");

        let attachment = SinglePart::builder()
            .header(header::ContentType::parse("application/pdf").map_err(|e| log_error("Error parseando content-type", e))?)
            .header(header::ContentDisposition::attachment(file_name))
            .body(file_body);

        MultiPart::mixed()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(data.mensaje),
            )
            .singlepart(attachment)
    } else {
        MultiPart::mixed().singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(data.mensaje),
        )
    };

    let email = email_builder.multipart(multipart)
        .map_err(|e| log_error("Error construyendo email", e))?;

    // 5. Configuración SMTP con OAuth2
    let creds = Credentials::new(
        "jhosephs_lizarazom@javeriana.edu.co".to_string(),
        access_token,
    );

    let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
        .map_err(|e| log_error("Error configurando relay SMTP", e))?
        .credentials(creds)
        .authentication(vec![Mechanism::Xoauth2])
        .port(587)
        .build();

    // 6. Envío con manejo detallado de errores
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(log_error("Error enviando email", e)),
    }
}
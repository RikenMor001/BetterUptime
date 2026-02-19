use lettre::{Message, SmtpTransport, Transport};
use letter::transport::smtp::authentication::Credentials;

pub fn send_downtime_alert(to_email: &str, website_url: &str, status: &str) -> Result<(), String>{
    let smtp_user = std::env::var("SMTP_USER").map_err(|_| "SMTP user not found")?;
    let smtp_password = std::env::var("SMTP_PASSWORD").map_err(|_| "SMTP password not found")?;

    let email = Message::builder()
        .from("BetterUptime <no-reply@gmail.com>".parse().unwrap())
        .to(to_email.parse().map_err(|e| e.to_string())?)
        .subject("Downtime Alert")
        .body(format!("The website {} is now {}", website_url, status))
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| e.to_string())?
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap_or_else(|_| panic!("Failed to send email"))?;
    Ok(())
}

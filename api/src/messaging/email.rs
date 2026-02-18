use lettre::{Message, SmtpTransport, Transport};
use letter::transport::smtp::authentication::Credentials;

pub fn send_downtime_alert(to_email: &str, website_url: &str, status: &str) -> Result<(), String>{
    let smtp_user = std::env::var("SMTP_USER").map_err(|_| "SMTP user not found")?;
    let smtp_password = std::env::var("SMTP_PASSWORD").map_err(|_| "SMTP password not found")?;

    let build_email = Message::builder()
    .from("BetterUptime <no-reply@betteruptime.com>".parse().unwrap())

    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .map_err(|_| "Failed to connect to the SMTP server")?
    .credentials(creds)
    .build();
}

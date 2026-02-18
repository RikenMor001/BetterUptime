use lettre::{Message, SmtpTransport, Transport};
use letter::transport::smtp::authentication::Credentials;

pub fn send_downtime_alert(to_email: &str, website_url: &str, status: &str) -> Result<(), String>{
    let smtp_user = std::env::var("SMTP_USER").map_err(|_| "SMTP user not found")?;
    let smtp_password = std::env::var("SMTP_PASSWORD").map_err(|_| "SMTP password not found")?;

}

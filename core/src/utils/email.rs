use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use anyhow::Result;

pub async fn send_email(
    smtp_server: &str,
    smtp_port: u16,
    smtp_user: &str,
    smtp_password: &str,
    from_email: &str,
    to_email: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    let creds = Credentials::new(smtp_user.to_string(), smtp_password.to_string());

    // Open a remote connection to the SMTP server
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Could not send email: {}", e)),
    }
} 
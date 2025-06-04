use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use anyhow::Result;
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::client::{SmtpTransport, Tls};
use lettre::transport::smtp::Error as SmtpError;
use crate::core::settings::Config;

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

    // 打开与 SMTP 服务器的远程连接
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    // 发送电子邮件
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Could not send email: {}", e)),
    }
}

/// 发送邮件验证码
pub async fn send_verification_email(
    config: &Config,
    to_email: &str,
    code: &str,
) -> Result<(), SmtpError> {
    let smtp_config = &config.email;

    let server = SmtpTransport::builder_relay(&smtp_config.server)?
        .port(smtp_config.port)
        .credentials(lettre::transport::smtp::authentication::Credentials::new(
            smtp_config.username.clone(),
            smtp_config.password.clone(),
        ))
        .tls(Tls::Opportunistic)
        .build();

    let email = Message::builder()
        .from(smtp_config.from_address.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("您的验证码")
        .body(format!("您的验证码是：{}", code))
        .unwrap();

    server.send(email).await.map(|_| ()) // 忽略 SendmailResponse
}

// TODO: 完善邮件内容模板，添加更详细的错误处理

pub async fn send_email_with_template(
    smtp_server: &str,
    smtp_port: u16,
    smtp_user: &str,
    smtp_password: &str,
    from_email: &str,
    to_email: &str,
    subject: &str,
    template: &str,
    data: &str,
) -> Result<()> {
    let email = Message::builder()
        .from(from_email.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(template.to_string())?;

    let creds = Credentials::new(smtp_user.to_string(), smtp_password.to_string());

    // 打开与 SMTP 服务器的远程连接
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    // 发送电子邮件
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Could not send email: {}", e)),
    }
} 
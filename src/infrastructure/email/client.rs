use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use tracing::{error, info};

use crate::config::SmtpSettings;

#[derive(Clone)]
pub struct EmailClient {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Failed to build email: {0}")]
    BuildError(String),

    #[error("Failed to send email: {0}")]
    SendError(String),

    #[error("Invalid email address: {0}")]
    InvalidAddress(String),
}

impl EmailClient {
    pub fn new(settings: &SmtpSettings) -> Result<Self, EmailError> {
        let from: Mailbox = format!("{} <{}>", settings.from_name, settings.from_email)
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("{}", e)))?;

        let creds = Credentials::new(settings.username.clone(), settings.password.clone());

        let mailer = if settings.tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.host)
                .map_err(|e| EmailError::BuildError(e.to_string()))?
                .credentials(creds)
                .port(settings.port)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&settings.host)
                .credentials(creds)
                .port(settings.port)
                .build()
        };

        Ok(Self { mailer, from })
    }

    pub async fn send_text(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        let to_mailbox: Mailbox = to
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("{}", e)))?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to_mailbox)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| EmailError::BuildError(e.to_string()))?;

        self.send_message(email).await
    }

    pub async fn send_html(
        &self,
        to: &str,
        subject: &str,
        text_body: &str,
        html_body: &str,
    ) -> Result<(), EmailError> {
        let to_mailbox: Mailbox = to
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("{}", e)))?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to_mailbox)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text_body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .map_err(|e| EmailError::BuildError(e.to_string()))?;

        self.send_message(email).await
    }

    pub async fn send_welcome_email(&self, to: &str, name: &str) -> Result<(), EmailError> {
        let subject = "Welcome!";

        let text_body = format!(
            "Hello {}!\n\nWelcome to our platform. We're glad to have you!\n\nBest regards,\nThe Team",
            name
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Welcome!</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
        <h1 style="color: #2563eb;">Hello {}!</h1>
        <p>Welcome to our platform. We're glad to have you!</p>
        <p>Best regards,<br>The Team</p>
    </div>
</body>
</html>"#,
            name
        );

        self.send_html(to, subject, &text_body, &html_body).await
    }

    pub async fn send_password_reset_email(
        &self,
        to: &str,
        name: &str,
        reset_link: &str,
    ) -> Result<(), EmailError> {
        let subject = "Password Reset Request";

        let text_body = format!(
            "Hello {}!\n\nYou requested a password reset. Click the link below to reset your password:\n\n{}\n\nIf you didn't request this, please ignore this email.\n\nBest regards,\nThe Team",
            name, reset_link
        );

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Password Reset</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
        <h1 style="color: #2563eb;">Password Reset</h1>
        <p>Hello {}!</p>
        <p>You requested a password reset. Click the button below to reset your password:</p>
        <p style="text-align: center; margin: 30px 0;">
            <a href="{}" style="background-color: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px;">Reset Password</a>
        </p>
        <p>If you didn't request this, please ignore this email.</p>
        <p>Best regards,<br>The Team</p>
    </div>
</body>
</html>"#,
            name, reset_link
        );

        self.send_html(to, subject, &text_body, &html_body).await
    }

    async fn send_message(&self, message: Message) -> Result<(), EmailError> {
        match self.mailer.send(message).await {
            Ok(response) => {
                info!(
                    "Email sent successfully: {:?}",
                    response.message().collect::<Vec<_>>()
                );
                Ok(())
            }
            Err(e) => {
                error!("Failed to send email: {}", e);
                Err(EmailError::SendError(e.to_string()))
            }
        }
    }
}

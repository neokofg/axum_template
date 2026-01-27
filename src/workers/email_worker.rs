use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::infrastructure::email::EmailClient;
use crate::infrastructure::queue::QueueClient;

pub const EMAIL_JOB_TYPE: &str = "send_email";
pub const WELCOME_EMAIL_JOB_TYPE: &str = "send_welcome_email";
pub const PASSWORD_RESET_JOB_TYPE: &str = "send_password_reset";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailArgs {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeEmailArgs {
    pub user_id: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetArgs {
    pub email: String,
    pub name: String,
    pub reset_link: String,
}

pub struct EmailWorker {
    queue: QueueClient,
}

impl EmailWorker {
    pub fn new(queue: QueueClient) -> Self {
        Self {
            queue: queue.with_queue("emails"),
        }
    }

    pub async fn enqueue_email(&self, args: SendEmailArgs) -> Result<String, redis::RedisError> {
        self.queue.enqueue(EMAIL_JOB_TYPE, args).await
    }

    pub async fn enqueue_welcome_email(
        &self,
        args: WelcomeEmailArgs,
    ) -> Result<String, redis::RedisError> {
        self.queue.enqueue(WELCOME_EMAIL_JOB_TYPE, args).await
    }

    pub async fn enqueue_password_reset(
        &self,
        args: PasswordResetArgs,
    ) -> Result<String, redis::RedisError> {
        self.queue.enqueue(PASSWORD_RESET_JOB_TYPE, args).await
    }
}

pub async fn process_email_job(
    job: serde_json::Value,
    email_client: &EmailClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let job_type = job["type"].as_str().unwrap_or("unknown");

    match job_type {
        EMAIL_JOB_TYPE => {
            let args: SendEmailArgs = serde_json::from_value(job["args"].clone())?;
            info!(
                to = %args.to,
                subject = %args.subject,
                "Processing email job"
            );

            if let Some(html_body) = &args.html_body {
                email_client
                    .send_html(&args.to, &args.subject, &args.body, html_body)
                    .await?;
            } else {
                email_client
                    .send_text(&args.to, &args.subject, &args.body)
                    .await?;
            }

            info!(to = %args.to, "Email sent successfully");
        }
        WELCOME_EMAIL_JOB_TYPE => {
            let args: WelcomeEmailArgs = serde_json::from_value(job["args"].clone())?;
            info!(
                user_id = %args.user_id,
                email = %args.email,
                "Processing welcome email job"
            );

            email_client
                .send_welcome_email(&args.email, &args.name)
                .await?;

            info!(email = %args.email, "Welcome email sent successfully");
        }
        PASSWORD_RESET_JOB_TYPE => {
            let args: PasswordResetArgs = serde_json::from_value(job["args"].clone())?;
            info!(
                email = %args.email,
                "Processing password reset email job"
            );

            email_client
                .send_password_reset_email(&args.email, &args.name, &args.reset_link)
                .await?;

            info!(email = %args.email, "Password reset email sent successfully");
        }
        _ => {
            error!(job_type = %job_type, "Unknown email job type");
        }
    }

    Ok(())
}

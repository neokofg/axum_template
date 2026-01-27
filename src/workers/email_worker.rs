use serde::{Deserialize, Serialize};
use tracing::info;

use crate::infrastructure::queue::QueueClient;

pub const EMAIL_JOB_TYPE: &str = "send_email";
pub const WELCOME_EMAIL_JOB_TYPE: &str = "send_welcome_email";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailArgs {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeEmailArgs {
    pub user_id: String,
    pub email: String,
    pub name: String,
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
}

pub async fn process_email_job(job: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let job_type = job["type"].as_str().unwrap_or("unknown");

    match job_type {
        EMAIL_JOB_TYPE => {
            let args: SendEmailArgs = serde_json::from_value(job["args"].clone())?;
            info!(
                to = %args.to,
                subject = %args.subject,
                "Processing email job"
            );
            // TODO: Implement actual email sending
            info!(to = %args.to, "Email sent successfully");
        }
        WELCOME_EMAIL_JOB_TYPE => {
            let args: WelcomeEmailArgs = serde_json::from_value(job["args"].clone())?;
            info!(
                user_id = %args.user_id,
                email = %args.email,
                "Processing welcome email job"
            );
            // TODO: Implement welcome email template
            info!(email = %args.email, "Welcome email sent successfully");
        }
        _ => {
            tracing::warn!(job_type = %job_type, "Unknown email job type");
        }
    }

    Ok(())
}

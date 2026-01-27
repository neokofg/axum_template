use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Deserialization failed: {0}")]
    DeserializationError(String),

    #[error("Request timeout")]
    Timeout,
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, HttpClientError> {
        Self::with_timeout(Duration::from_secs(30))
    }

    pub fn with_timeout(timeout: Duration) -> Result<Self, HttpClientError> {
        let client = Client::builder().timeout(timeout).build()?;

        Ok(Self { client })
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, HttpClientError> {
        let response = self.client.get(url).send().await?;
        self.handle_response(response).await
    }

    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, HttpClientError> {
        let response = self.client.post(url).json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T, HttpClientError> {
        let response = self.client.put(url).json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T, HttpClientError> {
        let response = self.client.delete(url).send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, HttpClientError> {
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(HttpClientError::DeserializationError(format!(
                "HTTP {} - {}",
                status, body
            )));
        }

        serde_json::from_str(&body)
            .map_err(|e| HttpClientError::DeserializationError(e.to_string()))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

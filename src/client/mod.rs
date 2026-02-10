mod messages;
mod tickets;

pub use messages::MessagesClient;
pub use tickets::TicketsClient;

use crate::config::GleapConfig;
use crate::error::AppError;

/// Core Gleap API client. Holds shared HTTP client and auth configuration.
///
/// Access resource-specific clients via accessor methods:
/// ```no_run
/// let client = GleapClient::new(config)?;
/// client.tickets().list(filters).await?;
/// client.messages().create_note(ticket_id, text).await?;
/// ```
pub struct GleapClient {
    http: reqwest::Client,
    config: GleapConfig,
}

impl GleapClient {
    pub fn new(config: GleapConfig) -> Result<Self, AppError> {
        let http = reqwest::Client::builder()
            .user_agent("gleap-cli/0.1.0")
            .build()
            .map_err(|e| AppError::Http(e))?;

        Ok(Self { http, config })
    }

    pub fn from_env() -> Result<Self, AppError> {
        let config = GleapConfig::from_env()?;
        Self::new(config)
    }

    pub fn tickets(&self) -> TicketsClient<'_> {
        TicketsClient::new(self)
    }

    pub fn messages(&self) -> MessagesClient<'_> {
        MessagesClient::new(self)
    }

    /// Build a GET request with auth headers pre-applied.
    pub(crate) fn get(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        self.http
            .get(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Build a POST request with auth headers pre-applied.
    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        self.http
            .post(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Build a PATCH request with auth headers pre-applied.
    pub(crate) fn patch(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        self.http
            .patch(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Send a request and handle common error responses.
    pub(crate) async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, AppError> {
        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            return Ok(response);
        }

        let status_code = status.as_u16();
        let body = response.text().await.unwrap_or_default();

        match status_code {
            401 | 403 => Err(AppError::Auth(body)),
            404 => Err(AppError::NotFound(body)),
            429 => Err(AppError::RateLimited {
                retry_after_secs: 60,
            }),
            _ => Err(AppError::ApiStatus {
                status: status_code,
                message: body,
            }),
        }
    }
}

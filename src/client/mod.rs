mod messages;
mod tickets;

pub use messages::MessagesClient;
pub use tickets::TicketsClient;

use crate::config::GleapConfig;
use crate::error::AppError;

/// Core Gleap API client. Holds shared HTTP client and auth configuration.
///
/// Access resource-specific clients via accessor methods:
/// ```ignore
/// let client = GleapClient::new(config)?;
/// client.tickets().list(filters).await?;
/// client.messages().create_note(ticket_id, text).await?;
/// ```
pub struct GleapClient {
    http: reqwest::Client,
    config: GleapConfig,
    verbose: u8,
}

impl GleapClient {
    pub fn new(config: GleapConfig) -> Result<Self, AppError> {
        let http = reqwest::Client::builder()
            .user_agent("gleap-cli/0.1.0")
            .build()
            .map_err(AppError::Http)?;

        Ok(Self {
            http,
            config,
            verbose: 0,
        })
    }

    /// Set verbosity level (0=quiet, 1=requests, 2=+responses, 3=+full body).
    pub fn with_verbose(mut self, level: u8) -> Self {
        self.verbose = level;
        self
    }

    /// Create a client using env vars only.
    pub fn from_env() -> Result<Self, AppError> {
        let config = GleapConfig::from_env()?;
        Self::new(config)
    }

    /// Create a client using the full credential resolution chain (env vars → keychain).
    pub fn resolve() -> Result<Self, AppError> {
        let config = GleapConfig::resolve()?;
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
        if self.verbose >= 1 {
            eprintln!("> GET {url}");
        }
        self.http
            .get(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Build a POST request with auth headers pre-applied.
    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        if self.verbose >= 1 {
            eprintln!("> POST {url}");
        }
        self.http
            .post(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Build a PUT request with auth headers pre-applied.
    pub(crate) fn put(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        if self.verbose >= 1 {
            eprintln!("> PUT {url}");
        }
        self.http
            .put(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Build a DELETE request with auth headers pre-applied.
    pub(crate) fn delete(&self, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.base_url, path);
        if self.verbose >= 1 {
            eprintln!("> DELETE {url}");
        }
        self.http
            .delete(&url)
            .bearer_auth(&self.config.api_key)
            .header("project", &self.config.project_id)
    }

    /// Send a request and handle common error responses.
    pub(crate) async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, AppError> {
        let start = std::time::Instant::now();
        let response = request.send().await?;
        let elapsed = start.elapsed();
        let status = response.status();

        if self.verbose >= 1 {
            eprintln!(
                "< {} {} ({:.0?})",
                status.as_u16(),
                status.canonical_reason().unwrap_or(""),
                elapsed
            );
        }

        if self.verbose >= 2 {
            for (name, value) in response.headers() {
                eprintln!("< {}: {}", name, value.to_str().unwrap_or("<binary>"));
            }
        }

        if status.is_success() {
            return Ok(response);
        }

        let status_code = status.as_u16();
        let body = response.text().await.unwrap_or_default();

        if self.verbose >= 2 {
            eprintln!("< Body: {body}");
        }

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

    /// Read response body as text, log at verbose levels, then deserialize.
    /// This preserves the raw body for debugging when deserialization fails.
    pub(crate) async fn send_and_parse<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, AppError> {
        let response = self.send(request).await?;
        let text = response.text().await?;

        if self.verbose >= 3 {
            eprintln!("< Body: {text}");
        }

        serde_json::from_str(&text).map_err(|e| {
            if self.verbose >= 2 {
                eprintln!("Deserialization error: {e}");
                eprintln!("Raw response: {text}");
            } else {
                eprintln!("Deserialization error: {e}");
                eprintln!("Hint: use -vv to see the raw API response");
            }
            AppError::Serialization(e)
        })
    }
}

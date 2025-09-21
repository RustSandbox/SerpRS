use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use tracing::{debug, info, warn};

use crate::{
    error::{SerpError, SerpResult},
    query::{SearchQuery, SearchQueryBuilder},
    response::SearchResults,
    retry::RetryPolicy,
};

/// Main client for SerpAPI interactions.
/// 
/// This is the primary interface for making search requests to the SerpAPI service.
/// It handles authentication, request construction, response parsing, and retry logic.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use serp_sdk::{SerpClient, SearchQuery};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SerpClient::builder()
///         .api_key("your-api-key")
///         .build()?;
/// 
///     let results = client.search(
///         SearchQuery::new("rust programming")
///             .language("en")
///             .limit(10)?
///     ).await?;
/// 
///     println!("Found {} results", 
///         results.organic_results.as_ref().map_or(0, |r| r.len()));
///     Ok(())
/// }
/// ```
pub struct SerpClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
    retry_policy: RetryPolicy,
}

/// Builder for constructing [`SerpClient`] with ergonomic API.
/// 
/// This builder provides a fluent interface for configuring the SerpAPI client
/// with various options like timeouts, retry policies, and custom headers.
/// 
/// # Examples
/// 
/// ```rust
/// use serp_sdk::{SerpClient, RetryPolicy};
/// use std::time::Duration;
/// 
/// let client = SerpClient::builder()
///     .api_key("your-api-key")
///     .timeout(Duration::from_secs(30))
///     .retry_policy(RetryPolicy::new(5))
///     .user_agent("my-app/1.0")
///     .build()?;
/// # Ok::<(), serp_sdk::SerpError>(())
/// ```
pub struct SerpClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Duration,
    retry_policy: RetryPolicy,
    user_agent: String,
    default_headers: HeaderMap,
}

impl Default for SerpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SerpClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: Some("https://serpapi.com".to_string()),
            timeout: Duration::from_secs(30),
            retry_policy: RetryPolicy::default(),
            user_agent: format!("serp-sdk-rust/{}", env!("CARGO_PKG_VERSION")),
            default_headers: HeaderMap::new(),
        }
    }

    /// Set the API key for authentication
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set a custom base URL (useful for testing or proxies)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set request timeout duration
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set retry policy for handling transient failures
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Set custom User-Agent header
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = agent.into();
        self
    }

    /// Add a default header to all requests
    pub fn default_header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> SerpResult<Self> {
        let header_name: reqwest::header::HeaderName = name.as_ref().parse()
            .map_err(|_| SerpError::InvalidParameter(format!("Invalid header name: {}", name.as_ref())))?;
        let header_value = HeaderValue::from_str(value.as_ref())
            .map_err(|_| SerpError::InvalidParameter(format!("Invalid header value: {}", value.as_ref())))?;
        
        self.default_headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Build the SerpClient
    pub fn build(self) -> SerpResult<SerpClient> {
        // Try to get API key from builder, then environment
        let api_key = self.api_key
            .or_else(|| std::env::var("SERP_API_KEY").ok())
            .ok_or(SerpError::MissingApiKey)?;

        // Validate API key format (basic check)
        if api_key.trim().is_empty() {
            return Err(SerpError::InvalidParameter("API key cannot be empty".to_string()));
        }

        // Build HTTP client with configured settings
        let mut client_builder = reqwest::Client::builder()
            .timeout(self.timeout)
            .default_headers(self.default_headers);

        // Set User-Agent
        if let Ok(user_agent) = HeaderValue::from_str(&self.user_agent) {
            client_builder = client_builder.default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(USER_AGENT, user_agent);
                headers
            });
        }

        let client = client_builder
            .build()
            .map_err(|e| SerpError::ClientBuilder(e.to_string()))?;

        Ok(SerpClient {
            api_key,
            base_url: self.base_url.unwrap_or_else(|| "https://serpapi.com".to_string()),
            client,
            retry_policy: self.retry_policy,
        })
    }
}

impl SerpClient {
    /// Create a new client builder
    pub fn builder() -> SerpClientBuilder {
        SerpClientBuilder::new()
    }

    /// Create a client with just an API key (using defaults)
    pub fn new(api_key: impl Into<String>) -> SerpResult<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Execute a search query asynchronously
    pub async fn search(&self, query: SearchQueryBuilder) -> SerpResult<SearchResults> {
        let query = query.build(self.api_key.clone());
        self.search_with_retry(query).await
    }

    /// Execute a search with the configured retry policy
    async fn search_with_retry(&self, query: SearchQuery) -> SerpResult<SearchResults> {
        let mut retries = 0;
        let max_retries = self.retry_policy.max_retries;

        loop {
            debug!("Executing search request (attempt {})", retries + 1);
            
            match self.execute_request(&query).await {
                Ok(results) => {
                    info!("Search completed successfully");
                    return Ok(results);
                }
                Err(SerpError::RateLimited { retry_after }) if retries < max_retries => {
                    warn!("Rate limited, retrying after {} seconds", retry_after);
                    tokio::time::sleep(Duration::from_secs(retry_after)).await;
                    retries += 1;
                }
                Err(e) if retries < max_retries && self.should_retry(&e) => {
                    let delay = self.retry_policy.backoff_duration(retries);
                    warn!("Request failed, retrying after {:?}: {}", delay, e);
                    tokio::time::sleep(delay).await;
                    retries += 1;
                }
                Err(e) => {
                    warn!("Request failed permanently: {}", e);
                    return Err(e);
                }
            }
        }
    }

    /// Execute a single HTTP request to SerpAPI
    async fn execute_request(&self, query: &SearchQuery) -> SerpResult<SearchResults> {
        let query_string = query.to_query_string()?;
        let url = format!("{}/search?{}", self.base_url, query_string);
        
        debug!("Making request to: {}", url.replace(&self.api_key, "***"));

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(SerpError::RequestFailed)?;

        let status = response.status();
        debug!("Response status: {}", status);

        // Handle rate limiting
        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return Err(SerpError::RateLimited { retry_after });
        }

        // Handle other HTTP errors
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(SerpError::ApiError {
                code: status.as_u16(),
                message: error_body,
            });
        }

        // Parse successful response
        let response_text = response.text().await
            .map_err(SerpError::RequestFailed)?;

        serde_json::from_str::<SearchResults>(&response_text)
            .map_err(|e| SerpError::InvalidResponse(format!("JSON parse error: {}", e)))
    }

    /// Determine if an error should trigger a retry
    fn should_retry(&self, error: &SerpError) -> bool {
        match error {
            SerpError::RequestFailed(reqwest_err) => {
                // Retry on network errors, timeouts, etc.
                reqwest_err.is_timeout() || 
                reqwest_err.is_connect() ||
                reqwest_err.is_request()
            }
            SerpError::ApiError { code, .. } => {
                // Retry on server errors (5xx)
                *code >= 500 && *code < 600
            }
            SerpError::Timeout | SerpError::Network(_) => true,
            _ => false,
        }
    }

    /// Get the configured API key (masked for logging)
    pub fn api_key_masked(&self) -> String {
        if self.api_key.len() > 8 {
            format!("{}***{}", 
                &self.api_key[..4], 
                &self.api_key[self.api_key.len()-4..]
            )
        } else {
            "***".to_string()
        }
    }

    /// Check if the client is properly configured
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let builder = SerpClient::builder()
            .api_key("test-key-longer")
            .timeout(Duration::from_secs(10))
            .user_agent("test-agent");

        let client = builder.build().unwrap();
        assert!(client.is_configured());
        assert_eq!(client.api_key_masked(), "test***nger");
    }

    #[test]
    fn test_missing_api_key() {
        // Clear any environment variable
        std::env::remove_var("SERP_API_KEY");
        
        let result = SerpClient::builder().build();
        assert!(matches!(result, Err(SerpError::MissingApiKey)));
    }

    #[test]
    fn test_invalid_api_key() {
        let result = SerpClient::builder().api_key("").build();
        assert!(matches!(result, Err(SerpError::InvalidParameter(_))));
    }
}
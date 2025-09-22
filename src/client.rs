//! # SerpAPI HTTP Client Module
//!
//! This module provides the core HTTP client implementation for interacting with the SerpAPI service.
//! It handles all aspects of API communication including authentication, request construction,
//! response parsing, error handling, and retry logic with exponential backoff.
//!
//! ## Architecture
//!
//! The client module follows a builder pattern for configuration and uses async/await for
//! non-blocking I/O operations. It's built on top of `reqwest` for HTTP communication and
//! integrates with `tracing` for observability.
//!
//! ## Key Components
//!
//! - [`SerpClient`]: The main client struct that manages API interactions
//! - [`SerpClientBuilder`]: A fluent builder for configuring client instances
//! - Retry logic with configurable policies for handling transient failures
//! - Automatic rate limiting detection and handling
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, SearchQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with an API key
//!     let client = SerpClient::new("your-api-key")?;
//!
//!     // Execute a search
//!     let results = client.search(
//!         SearchQuery::new("rust programming")
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Configuration
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, RetryPolicy};
//! use std::time::Duration;
//!
//! let client = SerpClient::builder()
//!     .api_key("your-api-key")
//!     .timeout(Duration::from_secs(60))
//!     .retry_policy(
//!         RetryPolicy::new(5)
//!             .with_base_delay(Duration::from_millis(100))
//!     )
//!     .user_agent("my-app/1.0")
//!     .base_url("https://custom-proxy.example.com")
//!     .build()?;
//! # Ok::<(), serp_sdk::SerpError>(())
//! ```

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::{
    error::{SerpError, SerpResult},
    query::{SearchQuery, SearchQueryBuilder},
    response::SearchResults,
    retry::RetryPolicy,
};

/// The main SerpAPI client for executing search requests.
///
/// `SerpClient` is the primary interface for interacting with the SerpAPI service.
/// It manages authentication, HTTP connections, retry logic, and response parsing.
/// The client is thread-safe and can be shared across multiple async tasks.
///
/// ## Design Philosophy
///
/// The client is designed with the following principles:
/// - **Ergonomic API**: Intuitive method chaining and builder patterns
/// - **Robust error handling**: Comprehensive error types with actionable information
/// - **Production-ready**: Built-in retry logic, rate limiting, and observability
/// - **Performance**: Connection pooling, async I/O, and efficient serialization
///
/// ## Connection Management
///
/// The client maintains an internal connection pool for efficient HTTP communication.
/// Connections are reused across requests to minimize latency and resource usage.
///
/// ## Authentication
///
/// Authentication is handled via API key, which can be provided through:
/// 1. Direct configuration via the builder
/// 2. Environment variable `SERP_API_KEY`
///
/// ## Error Handling
///
/// The client automatically handles common error scenarios:
/// - **Network errors**: Retried with exponential backoff
/// - **Rate limiting**: Respects `Retry-After` headers
/// - **Server errors**: 5xx responses trigger automatic retries
/// - **Client errors**: 4xx responses return immediately with detailed error info
///
/// ## Examples
///
/// ### Simple Search
///
/// ```rust,no_run
/// # use serp_sdk::{SerpClient, SearchQuery};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = SerpClient::new("your-api-key")?;
///
/// let results = client.search(
///     SearchQuery::new("rust async programming")
///         .limit(20)?
/// ).await?;
///
/// for result in results.organic_results.unwrap_or_default() {
///     println!("{}: {}", result.title, result.link);
/// }
/// # Ok(())
/// # }
/// ```
///
/// ### With Error Handling
///
/// ```rust,no_run
/// # use serp_sdk::{SerpClient, SearchQuery, SerpError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = SerpClient::new("test")?;
/// match client.search(SearchQuery::new("query")).await {
///     Ok(results) => {
///         // Process results
///     }
///     Err(SerpError::RateLimited { retry_after }) => {
///         println!("Rate limited, retry after {} seconds", retry_after);
///     }
///     Err(e) => {
///         eprintln!("Search failed: {}", e);
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct SerpClient {
    /// API key for authentication
    api_key: String,
    /// Base URL for the SerpAPI service
    base_url: String,
    /// HTTP client instance with connection pooling
    client: reqwest::Client,
    /// Retry policy for handling transient failures
    retry_policy: RetryPolicy,
}

/// A builder for constructing [`SerpClient`] instances with custom configuration.
///
/// The builder pattern provides a flexible and type-safe way to configure the client
/// with various options. All configuration methods return `self` for method chaining.
///
/// ## Default Configuration
///
/// - **Timeout**: 30 seconds
/// - **Retry Policy**: Default policy with exponential backoff
/// - **Base URL**: `https://serpapi.com`
/// - **User Agent**: `serp-sdk-rust/{version}`
///
/// ## Configuration Options
///
/// - [`api_key`](Self::api_key): Set the SerpAPI authentication key
/// - [`timeout`](Self::timeout): Configure request timeout duration
/// - [`retry_policy`](Self::retry_policy): Set custom retry behavior
/// - [`user_agent`](Self::user_agent): Override the User-Agent header
/// - [`base_url`](Self::base_url): Use a custom API endpoint
/// - [`default_header`](Self::default_header): Add custom headers to all requests
///
/// ## Examples
///
/// ### Basic Configuration
///
/// ```rust
/// # use serp_sdk::SerpClient;
/// let client = SerpClient::builder()
///     .api_key("your-api-key")
///     .build()?;
/// # Ok::<(), serp_sdk::SerpError>(())
/// ```
///
/// ### Full Configuration
///
/// ```rust
/// # use serp_sdk::{SerpClient, RetryPolicy};
/// # use std::time::Duration;
/// let client = SerpClient::builder()
///     .api_key("your-api-key")
///     .timeout(Duration::from_secs(60))
///     .retry_policy(
///         RetryPolicy::new(3)
///             .with_base_delay(Duration::from_millis(500))
///             .with_max_delay(Duration::from_secs(30))
///     )
///     .user_agent("my-search-app/2.0")
///     .default_header("X-Custom-Header", "value")?
///     .build()?;
/// # Ok::<(), serp_sdk::SerpError>(())
/// ```
///
/// ### Using Environment Variables
///
/// ```rust,no_run
/// # use serp_sdk::SerpClient;
/// // API key will be read from SERP_API_KEY environment variable
/// std::env::set_var("SERP_API_KEY", "your-api-key");
/// let client = SerpClient::builder().build()?;
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
    /// Creates a new client builder with default settings.
    ///
    /// The default configuration is suitable for most use cases:
    /// - 30-second timeout
    /// - Standard retry policy with exponential backoff
    /// - Official SerpAPI endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serp_sdk::SerpClient;
    ///
    /// let builder = SerpClient::builder();
    /// ```
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

    /// Sets the API key for authentication.
    ///
    /// The API key is required for all SerpAPI requests. You can obtain one by
    /// signing up at [https://serpapi.com](https://serpapi.com).
    ///
    /// # Arguments
    ///
    /// * `key` - The SerpAPI authentication key
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::builder()
    ///     .api_key("your-secret-api-key")
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Sets a custom base URL for the API.
    ///
    /// This is useful for:
    /// - Using a proxy server
    /// - Connecting to a mock server for testing
    /// - Using a regional endpoint
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL (without trailing slash)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::builder()
    ///     .api_key("test-key")
    ///     .base_url("https://proxy.example.com")
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the request timeout duration.
    ///
    /// This timeout applies to the entire request/response cycle, including:
    /// - DNS resolution
    /// - TCP connection
    /// - TLS handshake
    /// - Request transmission
    /// - Response reception
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum duration for a single request
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// # use std::time::Duration;
    /// let client = SerpClient::builder()
    ///     .api_key("key")
    ///     .timeout(Duration::from_secs(60))  // 1 minute timeout
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the retry policy for handling transient failures.
    ///
    /// The retry policy determines:
    /// - Maximum number of retry attempts
    /// - Delay between retries (with exponential backoff)
    /// - Jitter for avoiding thundering herd
    ///
    /// # Arguments
    ///
    /// * `policy` - Custom retry policy configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::{SerpClient, RetryPolicy};
    /// # use std::time::Duration;
    /// let policy = RetryPolicy::new(5)
    ///     .with_base_delay(Duration::from_millis(100))
    ///     .with_max_delay(Duration::from_secs(60))
    ///     .with_jitter(true);
    ///
    /// let client = SerpClient::builder()
    ///     .api_key("key")
    ///     .retry_policy(policy)
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Sets a custom User-Agent header.
    ///
    /// The User-Agent identifies your application to the API server.
    /// This can be useful for debugging and analytics.
    ///
    /// # Arguments
    ///
    /// * `agent` - User-Agent string
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::builder()
    ///     .api_key("key")
    ///     .user_agent("MySearchApp/1.0 (contact@example.com)")
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = agent.into();
        self
    }

    /// Adds a default header to all requests.
    ///
    /// Custom headers are useful for:
    /// - Adding authentication tokens
    /// - Setting correlation IDs for tracing
    /// - Adding custom metadata
    ///
    /// # Arguments
    ///
    /// * `name` - Header name
    /// * `value` - Header value
    ///
    /// # Returns
    ///
    /// Returns `Result<Self>` as header parsing can fail for invalid names/values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::builder()
    ///     .api_key("key")
    ///     .default_header("X-Request-ID", "abc123")?
    ///     .default_header("X-Client-Version", "2.0")?
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn default_header(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> SerpResult<Self> {
        let header_name: reqwest::header::HeaderName = name.as_ref().parse().map_err(|_| {
            SerpError::InvalidParameter(format!("Invalid header name: {}", name.as_ref()))
        })?;
        let header_value = HeaderValue::from_str(value.as_ref()).map_err(|_| {
            SerpError::InvalidParameter(format!("Invalid header value: {}", value.as_ref()))
        })?;

        self.default_headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Builds the configured [`SerpClient`] instance.
    ///
    /// This method validates the configuration and creates the client.
    /// It will fail if:
    /// - No API key is provided (via builder or environment)
    /// - The API key is empty or whitespace-only
    /// - HTTP client construction fails
    ///
    /// # Returns
    ///
    /// Returns `Result<SerpClient>` which will be:
    /// - `Ok(client)` if configuration is valid
    /// - `Err(SerpError)` if configuration is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// assert!(client.is_configured());
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - [`SerpError::MissingApiKey`]: No API key provided
    /// - [`SerpError::InvalidParameter`]: API key is empty
    /// - [`SerpError::ClientBuilder`]: HTTP client construction failed
    pub fn build(self) -> SerpResult<SerpClient> {
        // Try to get API key from builder, then environment
        let api_key = self
            .api_key
            .or_else(|| std::env::var("SERP_API_KEY").ok())
            .ok_or(SerpError::MissingApiKey)?;

        // Validate API key format (basic check)
        if api_key.trim().is_empty() {
            return Err(SerpError::InvalidParameter(
                "API key cannot be empty".to_string(),
            ));
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
            base_url: self
                .base_url
                .unwrap_or_else(|| "https://serpapi.com".to_string()),
            client,
            retry_policy: self.retry_policy,
        })
    }
}

impl SerpClient {
    /// Creates a new client builder for configuration.
    ///
    /// This is the recommended way to create a client instance, as it provides
    /// full control over configuration options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serp_sdk::SerpClient;
    ///
    /// let client = SerpClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn builder() -> SerpClientBuilder {
        SerpClientBuilder::new()
    }

    /// Creates a new client with just an API key using default settings.
    ///
    /// This is a convenience method for simple use cases. For more control
    /// over configuration, use [`SerpClient::builder()`].
    ///
    /// # Arguments
    ///
    /// * `api_key` - The SerpAPI authentication key
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::new("your-api-key")?;
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`SerpError::InvalidParameter`] if the API key is empty.
    pub fn new(api_key: impl Into<String>) -> SerpResult<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Executes a search query asynchronously.
    ///
    /// This is the main method for performing searches. It handles:
    /// - Query parameter validation
    /// - HTTP request construction
    /// - Automatic retry on transient failures
    /// - Rate limiting detection and respect
    /// - Response parsing and validation
    ///
    /// # Arguments
    ///
    /// * `query` - A configured search query builder
    ///
    /// # Returns
    ///
    /// Returns `Result<SearchResults>` containing:
    /// - Organic search results
    /// - Knowledge graph data
    /// - Answer boxes
    /// - Related searches
    /// - And more specialized result types
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serp_sdk::{SerpClient, SearchQuery};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SerpClient::new("test")?;
    /// let results = client.search(
    ///     SearchQuery::new("rust programming")
    ///         .language("en")
    ///         .limit(20)?
    /// ).await?;
    ///
    /// println!("Found {} organic results",
    ///     results.organic_results.as_ref().map_or(0, |r| r.len()));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`SerpError::InvalidParameter`]: Query parameters are invalid
    /// - [`SerpError::RateLimited`]: API rate limit exceeded
    /// - [`SerpError::ApiError`]: API returned an error response
    /// - [`SerpError::Network`]: Network communication failed
    /// - [`SerpError::InvalidResponse`]: Response parsing failed
    pub async fn search(&self, query: SearchQueryBuilder) -> SerpResult<SearchResults> {
        let query = query.build(self.api_key.clone());
        self.search_with_retry(query).await
    }

    /// Executes a search with automatic retry logic.
    ///
    /// This internal method implements the retry loop with exponential backoff.
    /// It will retry on:
    /// - Network errors (timeout, connection failures)
    /// - Server errors (5xx status codes)
    /// - Rate limiting (with respect to Retry-After header)
    ///
    /// The retry behavior is controlled by the configured [`RetryPolicy`].
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

    /// Executes a single HTTP request to the SerpAPI service.
    ///
    /// This method constructs the full request URL, sends the HTTP GET request,
    /// and parses the response. It handles various HTTP status codes and
    /// converts them to appropriate error types.
    async fn execute_request(&self, query: &SearchQuery) -> SerpResult<SearchResults> {
        let query_string = query.to_query_string()?;
        let url = format!("{}/search?{}", self.base_url, query_string);

        debug!("Making request to: {}", url.replace(&self.api_key, "***"));

        let response = self
            .client
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
        let response_text = response.text().await.map_err(SerpError::RequestFailed)?;

        serde_json::from_str::<SearchResults>(&response_text)
            .map_err(|e| SerpError::InvalidResponse(format!("JSON parse error: {}", e)))
    }

    /// Determines if an error should trigger a retry attempt.
    ///
    /// This method implements the retry decision logic based on error type:
    /// - Network errors are always retried
    /// - Server errors (5xx) are retried
    /// - Client errors (4xx) are not retried
    /// - Parsing errors are not retried
    fn should_retry(&self, error: &SerpError) -> bool {
        match error {
            SerpError::RequestFailed(reqwest_err) => {
                // Retry on network errors, timeouts, etc.
                reqwest_err.is_timeout() || reqwest_err.is_connect() || reqwest_err.is_request()
            }
            SerpError::ApiError { code, .. } => {
                // Retry on server errors (5xx)
                *code >= 500 && *code < 600
            }
            SerpError::Timeout | SerpError::Network(_) => true,
            _ => false,
        }
    }

    /// Returns a masked version of the API key for logging.
    ///
    /// This method is useful for debugging and logging without exposing
    /// the full API key. It shows the first 4 and last 4 characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::new("abcd1234efgh5678")?;
    /// assert_eq!(client.api_key_masked(), "abcd***5678");
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
    pub fn api_key_masked(&self) -> String {
        if self.api_key.len() > 8 {
            format!(
                "{}***{}",
                &self.api_key[..4],
                &self.api_key[self.api_key.len() - 4..]
            )
        } else {
            "***".to_string()
        }
    }

    /// Checks if the client is properly configured.
    ///
    /// A client is considered configured if it has a non-empty API key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serp_sdk::SerpClient;
    /// let client = SerpClient::new("api-key")?;
    /// assert!(client.is_configured());
    /// # Ok::<(), serp_sdk::SerpError>(())
    /// ```
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

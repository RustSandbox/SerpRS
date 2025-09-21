use thiserror::Error;

/// Comprehensive error types for SerpAPI SDK operations.
/// 
/// This enum covers all possible error conditions that can occur when using the SDK,
/// from configuration issues to network problems and API-specific errors.
/// 
/// # Examples
/// 
/// ```rust
/// use serp_sdk::{SerpClient, SerpError};
/// 
/// // Handle specific error types
/// match SerpClient::builder().build() {
///     Ok(client) => println!("Client created successfully"),
///     Err(SerpError::MissingApiKey) => {
///         println!("Please set SERP_API_KEY environment variable");
///     }
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SerpError {
    /// API key not provided via builder or environment variable.
    /// 
    /// Set the API key using [`SerpClientBuilder::api_key`] or the `SERP_API_KEY` environment variable.
    /// 
    /// [`SerpClientBuilder::api_key`]: crate::client::SerpClientBuilder::api_key
    #[error("API key not provided")]
    MissingApiKey,

    /// Error during HTTP client construction.
    /// 
    /// This typically indicates an issue with the underlying HTTP client configuration,
    /// such as invalid TLS settings or proxy configuration.
    #[error("Client builder error: {0}")]
    ClientBuilder(String),

    /// HTTP request failed.
    /// 
    /// This wraps underlying [`reqwest::Error`] and can indicate network connectivity issues,
    /// DNS resolution failures, or connection timeouts.
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// Response could not be parsed or has invalid format.
    /// 
    /// This occurs when the API returns a response that doesn't match the expected JSON structure,
    /// which might indicate API changes or corrupted responses.
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Rate limit exceeded, with retry-after duration.
    /// 
    /// SerpAPI has rate limits. When exceeded, this error includes the number of seconds
    /// to wait before retrying. The SDK's retry logic handles this automatically.
    #[error("Rate limit exceeded: retry after {retry_after} seconds")]
    RateLimited { 
        /// Number of seconds to wait before retrying
        retry_after: u64 
    },

    /// API returned an error response.
    /// 
    /// This represents HTTP error status codes (4xx, 5xx) returned by the SerpAPI service,
    /// such as authentication failures or server errors.
    #[error("API error: {code} - {message}")]
    ApiError { 
        /// HTTP status code
        code: u16, 
        /// Error message from the API
        message: String 
    },

    /// JSON serialization/deserialization error.
    /// 
    /// This wraps [`serde_json::Error`] and occurs when request parameters cannot be
    /// serialized or response data cannot be deserialized.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// URL encoding error.
    /// 
    /// This occurs when query parameters cannot be properly URL-encoded,
    /// typically due to invalid characters in parameter values.
    #[error("URL encoding error: {0}")]
    UrlEncoding(#[from] serde_urlencoded::ser::Error),

    /// Invalid query parameter.
    /// 
    /// This is thrown when query parameters fail validation, such as
    /// setting a limit outside the valid range (1-100).
    #[error("Invalid query parameter: {0}")]
    InvalidParameter(String),

    /// Timeout during request execution.
    /// 
    /// The request exceeded the configured timeout duration.
    /// Adjust the timeout using [`SerpClientBuilder::timeout`].
    /// 
    /// [`SerpClientBuilder::timeout`]: crate::client::SerpClientBuilder::timeout
    #[error("Request timeout")]
    Timeout,

    /// Network connectivity issues.
    /// 
    /// This represents lower-level network problems that don't fit into
    /// other categories, such as proxy failures or DNS issues.
    #[error("Network error: {0}")]
    Network(String),
}

/// Result type alias for SerpAPI operations.
/// 
/// This is a convenience type alias that uses [`SerpError`] as the error type.
/// Most functions in this crate return this type.
/// 
/// # Examples
/// 
/// ```rust
/// use serp_sdk::{SerpResult, SerpClient};
/// 
/// fn create_client() -> SerpResult<SerpClient> {
///     SerpClient::builder()
///         .api_key("test-key")
///         .build()
/// }
/// ```
pub type SerpResult<T> = Result<T, SerpError>;
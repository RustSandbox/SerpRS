//! # SerpAPI Rust SDK
//!
//! [![Crates.io](https://img.shields.io/crates/v/serp-sdk.svg)](https://crates.io/crates/serp-sdk)
//! [![Documentation](https://docs.rs/serp-sdk/badge.svg)](https://docs.rs/serp-sdk)
//! [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
//! [![Build Status](https://github.com/your-org/serp-sdk/workflows/CI/badge.svg)](https://github.com/your-org/serp-sdk/actions)
//!
//! A comprehensive, production-ready Rust SDK for the [SerpAPI](https://serpapi.com) service
//! that provides real-time search engine results through a unified, type-safe interface.
//!
//! > 🏆 **Developed during the [Realtime Search AI Hackathon (Hybrid)](https://www.eventbrite.com/e/realtime-search-ai-hackathon-hybrid-powered-by-serpapi-tickets)
//! > powered by SerpAPI and organized by [AI Tinkerers Paris](https://paris.aitinkerers.org/)**
//!
//! ## Overview
//!
//! The SerpAPI Rust SDK is designed with developer experience at its core. It provides a fluent,
//! intuitive API that makes complex search operations simple while maintaining the flexibility
//! needed for advanced use cases. Whether you're building a search aggregator, market research
//! tool, or AI-powered application, this SDK handles the complexity of search API interactions
//! so you can focus on your business logic.
//!
//! ## Key Features
//!
//! - 🦀 **Type-safe by Design**: Leverage Rust's type system to catch errors at compile-time
//! - ⚡ **Async-First Architecture**: Built on Tokio for high-performance concurrent operations
//! - 🎯 **Intuitive Builder Pattern**: Chain methods naturally to construct complex queries
//! - 🔄 **Intelligent Retry Logic**: Automatic retries with exponential backoff for resilience
//! - 🌊 **Streaming Support**: Efficiently handle large result sets with async streams
//! - 🏭 **Production-Ready**: Battle-tested error handling, comprehensive logging, and metrics
//! - 🔍 **Multi-Engine Support**: Google, Bing, Yahoo, Yandex, and 40+ search engines
//! - 📊 **Specialized Searches**: Images, news, videos, shopping, maps, and local results
//!
//! ## Installation
//!
//! Add the SDK to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serp-sdk = "0.2"
//! tokio = { version = "1.0", features = ["full"] }
//!
//! # Optional: For streaming support
//! futures = "0.3"
//!
//! # Optional: For enhanced logging
//! tracing = "0.1"
//! tracing-subscriber = "0.3"
//! ```
//!
//! ## Quick Start
//!
//! ### Basic Search Example
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, SearchQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client with API key from environment or explicit configuration
//!     let client = SerpClient::builder()
//!         .api_key("your-serp-api-key")
//!         .build()?;
//!
//!     // Perform a simple search
//!     let results = client.search(
//!         SearchQuery::new("Rust programming language")
//!             .language("en")
//!             .country("us")
//!             .limit(10)?
//!     ).await?;
//!
//!     // Process organic search results
//!     if let Some(organic) = results.organic_results {
//!         for result in organic {
//!             println!("Title: {}", result.title);
//!             println!("Link: {}", result.link);
//!             if let Some(snippet) = result.snippet {
//!                 println!("Snippet: {}", snippet);
//!             }
//!             println!("---");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Environment Configuration
//!
//! The SDK supports configuration through environment variables for production deployments:
//!
//! ```bash
//! export SERP_API_KEY="your-api-key"
//! export SERP_TIMEOUT="30"  # Timeout in seconds
//! export SERP_MAX_RETRIES="5"  # Maximum retry attempts
//! ```
//!
//! ## Advanced Usage
//!
//! ### Streaming Large Result Sets
//!
//! For queries that return large numbers of results, use streaming to process them efficiently:
//!
//! ```rust,no_run
//! use futures::StreamExt;
//! use serp_sdk::{SerpClient, SearchQuery, StreamConfig};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! // Configure streaming parameters
//! let stream_config = StreamConfig::new()
//!     .page_size(20)?    // Results per page
//!     .max_pages(5)      // Maximum pages to fetch
//!     .delay(std::time::Duration::from_millis(500)); // Rate limiting
//!
//! // Create a search stream
//! let mut stream = client.search_stream(
//!     SearchQuery::new("rust async programming"),
//!     stream_config
//! );
//!
//! // Process results as they arrive
//! while let Some(page_result) = stream.next().await {
//!     match page_result {
//!         Ok(results) => {
//!             println!("Processing page with {} results",
//!                 results.organic_results.as_ref().map_or(0, |r| r.len()));
//!
//!             // Process each page's results
//!             if let Some(organic) = results.organic_results {
//!                 for result in organic {
//!                     // Your processing logic here
//!                 }
//!             }
//!         }
//!         Err(e) => eprintln!("Error fetching page: {}", e),
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Specialized Search Types
//!
//! The SDK provides built-in support for various search types with tailored result structures:
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! // Image search with visual results
//! let image_results = client.search(
//!     SearchQuery::new("rust logo")
//!         .images()  // Automatically sets tbm=isch parameter
//!         .limit(50)?
//! ).await?;
//!
//! if let Some(images) = image_results.inline_images {
//!     for image in images {
//!         println!("Image: {:?}", image.original);
//!         println!("Thumbnail: {:?}", image.thumbnail);
//!     }
//! }
//!
//! // News search with recent articles
//! let news_results = client.search(
//!     SearchQuery::new("rust programming language")
//!         .news()    // Automatically sets tbm=nws parameter
//!         .language("en")
//!         .time_filter("d")  // Last 24 hours
//! ).await?;
//!
//! // Video search results
//! let video_results = client.search(
//!     SearchQuery::new("rust tutorial")
//!         .videos()  // Automatically sets tbm=vid parameter
//! ).await?;
//!
//! // Shopping/product search
//! let shopping_results = client.search(
//!     SearchQuery::new("rust programming book")
//!         .shopping()  // Automatically sets tbm=shop parameter
//!         .country("us")
//! ).await?;
//!
//! // Local search with location
//! let local_results = client.search(
//!     SearchQuery::new("coffee shops")
//!         .location("Austin, Texas")
//!         .limit(20)?
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Query Building
//!
//! Leverage the fluent builder pattern for complex queries:
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! let complex_query = SearchQuery::new("site:github.com rust async")
//!     .language("en")
//!     .country("us")
//!     .device("desktop")      // Desktop, tablet, or mobile
//!     .safe_search("off")     // off, active, or medium
//!     .time_filter("m")       // Past month
//!     .filter("0")           // Include similar results
//!     .offset(10)            // Start from result 10
//!     .limit(50)?            // Get 50 results
//!     .custom_param("gl", "us")  // Add any SerpAPI parameter
//!     .custom_param("lr", "lang_en");
//!
//! let results = client.search(complex_query).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Handling
//!
//! The SDK provides granular error types for precise error handling:
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, SearchQuery, SerpError};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! # let query = SearchQuery::new("test");
//! match client.search(query).await {
//!     Ok(results) => {
//!         // Process successful results
//!     }
//!     Err(SerpError::RateLimited { retry_after }) => {
//!         // Handle rate limiting
//!         println!("Rate limited. Retry after {} seconds", retry_after);
//!         tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
//!         // Retry the request
//!     }
//!     Err(SerpError::ApiError { code, message }) => {
//!         // Handle API errors
//!         match code {
//!             401 => println!("Invalid API key: {}", message),
//!             403 => println!("Access forbidden: {}", message),
//!             404 => println!("Resource not found: {}", message),
//!             _ => println!("API error {}: {}", code, message),
//!         }
//!     }
//!     Err(SerpError::InvalidQuery(msg)) => {
//!         // Handle query validation errors
//!         println!("Invalid query parameters: {}", msg);
//!     }
//!     Err(SerpError::NetworkError(e)) => {
//!         // Handle network-level errors
//!         println!("Network error: {}", e);
//!     }
//!     Err(e) => {
//!         // Handle other errors
//!         println!("Unexpected error: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Custom Retry Policies
//!
//! Configure retry behavior for resilient applications:
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, RetryPolicy};
//! use std::time::Duration;
//!
//! let retry_policy = RetryPolicy::new(5)              // Max 5 retries
//!     .with_base_delay(Duration::from_millis(200))   // Start with 200ms delay
//!     .with_max_delay(Duration::from_secs(30))       // Cap at 30 seconds
//!     .with_jitter(true);                            // Add randomization
//!
//! let client = SerpClient::builder()
//!     .api_key("your-key")
//!     .timeout(Duration::from_secs(60))
//!     .retry_policy(retry_policy)
//!     .build()?;
//! # Ok::<(), serp_sdk::SerpError>(())
//! ```
//!
//! ### Working with Response Data
//!
//! The SDK provides strongly-typed response structures for all result types:
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! let results = client.search(SearchQuery::new("rust")).await?;
//!
//! // Access search metadata
//! println!("Search ID: {}", results.search_metadata.id);
//! if let Some(time) = results.search_metadata.total_time_taken {
//!     println!("Time taken: {:.2}s", time);
//! }
//!
//! // Access knowledge graph
//! if let Some(kg) = results.knowledge_graph {
//!     println!("Knowledge Graph: {}", kg.title);
//!     if let Some(desc) = kg.description {
//!         println!("Description: {}", desc);
//!     }
//! }
//!
//! // Access answer box
//! if let Some(answer) = results.answer_box {
//!     println!("Answer: {:?}", answer.answer);
//! }
//!
//! // Access related searches
//! if let Some(related) = results.related_searches {
//!     println!("Related searches:");
//!     for search in related {
//!         // Handle both simple and block formats
//!         match search {
//!             serp_sdk::response::RelatedSearch::Simple { query, .. } => {
//!                 println!("  - {}", query);
//!             }
//!             serp_sdk::response::RelatedSearch::Block { items, .. } => {
//!                 for item in items {
//!                     if let Some(name) = item.name {
//!                         println!("  - {}", name);
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! // Access pagination information
//! if let Some(pagination) = results.pagination {
//!     println!("Current page: {:?}", pagination.current);
//!     if let Some(next) = pagination.next {
//!         println!("Next page: {}", next);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! ### Connection Pooling
//!
//! The SDK automatically manages connection pooling for optimal performance:
//!
//! ```rust,no_run
//! # use serp_sdk::SerpClient;
//! // The client reuses connections efficiently
//! let client = SerpClient::builder()
//!     .api_key("your-key")
//!     .max_connections(100)     // Maximum concurrent connections
//!     .connection_timeout(std::time::Duration::from_secs(10))
//!     .build()?;
//! # Ok::<(), serp_sdk::SerpError>(())
//! ```
//!
//! ### Batch Processing
//!
//! Process multiple queries efficiently:
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! use futures::future::join_all;
//!
//! let queries = vec![
//!     SearchQuery::new("rust async"),
//!     SearchQuery::new("rust web framework"),
//!     SearchQuery::new("rust database"),
//! ];
//!
//! // Execute queries concurrently
//! let futures = queries.into_iter()
//!     .map(|q| client.search(q))
//!     .collect::<Vec<_>>();
//!
//! let results = join_all(futures).await;
//!
//! for result in results {
//!     match result {
//!         Ok(data) => println!("Got {} results",
//!             data.organic_results.as_ref().map_or(0, |r| r.len())),
//!         Err(e) => eprintln!("Query failed: {}", e),
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Integration Examples
//!
//! ### MCP (Model Context Protocol) Integration
//!
//! The SDK is designed to work seamlessly with AI agents and LLM tools:
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! // Convert search results to MCP-compatible format
//! async fn search_for_mcp(
//!     client: &SerpClient,
//!     query: String,
//! ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
//!     let results = client.search(SearchQuery::new(&query)).await?;
//!
//!     Ok(json!({
//!         "results": results.organic_results.unwrap_or_default()
//!             .iter()
//!             .map(|r| json!({
//!                 "title": r.title,
//!                 "url": r.link,
//!                 "snippet": r.snippet
//!             }))
//!             .collect::<Vec<_>>(),
//!         "metadata": {
//!             "total_time": results.search_metadata.total_time_taken,
//!             "query": query
//!         }
//!     }))
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Troubleshooting
//!
//! ### Common Issues
//!
//! 1. **Rate Limiting**: Implement exponential backoff or use the built-in retry policy
//! 2. **Timeout Errors**: Increase the timeout duration for slow queries
//! 3. **Invalid API Key**: Ensure your API key is correctly set and has sufficient quota
//! 4. **Deserialization Errors**: Update to the latest SDK version for API compatibility
//!
//! ### Debug Logging
//!
//! Enable detailed logging for troubleshooting:
//!
//! ```rust,no_run
//! use tracing_subscriber;
//!
//! // Enable debug logging
//! tracing_subscriber::fmt()
//!     .with_max_level(tracing::Level::DEBUG)
//!     .init();
//! ```
//!
//! ## Contributing
//!
//! We welcome contributions! Please see our [GitHub repository](https://github.com/RustSandbox/SerpRS)
//! for contribution guidelines and development setup.
//!
//! ## License
//!
//! This project is dual-licensed under MIT and Apache-2.0 licenses.
//!
//! ## See Also
//!
//! - [`client`]: HTTP client implementation and configuration
//! - [`query`]: Query builder and search parameters
//! - [`response`]: Response structures and deserialization
//! - [`streaming`]: Async streaming support for pagination
//! - [`error`]: Error types and handling
//! - [`retry`]: Retry policies and backoff strategies

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

/// HTTP client module providing the main SerpAPI client implementation.
///
/// This module contains the [`SerpClient`](client::SerpClient) struct which is the primary
/// interface for interacting with the SerpAPI service. It provides methods for executing
/// searches, handling retries, and managing HTTP connections efficiently.
///
/// # Examples
///
/// ```rust,no_run
/// use serp_sdk::client::SerpClient;
///
/// let client = SerpClient::builder()
///     .api_key("your-api-key")
///     .build()
///     .expect("Failed to create client");
/// ```
pub mod client;

/// Comprehensive error types for all SDK operations.
///
/// This module defines the [`SerpError`] enum and related types that
/// represent all possible error conditions in the SDK. It provides detailed error information
/// with actionable messages for debugging and error recovery.
pub mod error;

/// Fluent query builder for constructing search requests.
///
/// The [`SearchQuery`](query::SearchQuery) builder provides a type-safe, ergonomic API
/// for constructing complex search queries with compile-time validation where possible.
pub mod query;

/// Strongly-typed response structures for SerpAPI results.
///
/// This module contains all the response types returned by SerpAPI, including organic results,
/// knowledge graphs, answer boxes, and specialized result types for images, videos, news, etc.
pub mod response;

/// Retry policies with configurable backoff strategies.
///
/// The [`RetryPolicy`](retry::RetryPolicy) struct allows fine-grained control over retry
/// behavior, including exponential backoff, jitter, and maximum delay configuration.
pub mod retry;

/// Streaming support for paginated search results.
///
/// This module provides async stream implementations for efficiently processing large result
/// sets through pagination, with built-in rate limiting and error handling.
pub mod streaming;

// Re-export main types for convenience
pub use client::{SerpClient, SerpClientBuilder};
pub use error::{SerpError, SerpResult};
pub use query::{SearchQuery, SearchQueryBuilder};
pub use response::SearchResults;
pub use retry::RetryPolicy;
pub use streaming::StreamConfig;

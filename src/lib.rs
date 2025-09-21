//! # SerpAPI Rust SDK
//!
//! [![Crates.io](https://img.shields.io/crates/v/serp-sdk.svg)](https://crates.io/crates/serp-sdk)
//! [![Documentation](https://docs.rs/serp-sdk/badge.svg)](https://docs.rs/serp-sdk)
//! [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
//! [![Build Status](https://github.com/your-org/serp-sdk/workflows/CI/badge.svg)](https://github.com/your-org/serp-sdk/actions)
//!
//! A comprehensive, production-ready Rust SDK for the [SerpAPI](https://serpapi.com) service
//! that provides ergonomic APIs, type safety, and async-first design.
//!
//! > 🏆 **Developed during the [Realtime Search AI Hackathon (Hybrid)](https://www.eventbrite.com/e/realtime-search-ai-hackathon-hybrid-powered-by-serpapi-tickets)
//! > powered by SerpAPI and organized by [AI Tinkerers Paris](https://paris.aitinkerers.org/)**
//!
//! ## Features
//!
//! - 🦀 **Type-safe**: Strongly typed request builders and response structures
//! - ⚡ **Async/await**: Built on tokio with efficient async I/O
//! - 🎯 **Ergonomic**: Fluent builder APIs for constructing queries
//! - 🔄 **Resilient**: Automatic retry logic with exponential backoff
//! - 🌊 **Streaming**: Support for paginated result streaming
//! - 🏭 **Production-ready**: Comprehensive error handling and logging
//! - 🔍 **Specialized**: Built-in support for images, news, videos, shopping, and local search
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serp-sdk = "0.1"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! Basic usage:
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, SearchQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client (API key from env var SERP_API_KEY or builder)
//!     let client = SerpClient::builder()
//!         .api_key("your-serp-api-key")
//!         .build()?;
//!
//!     // Build and execute search
//!     let results = client.search(
//!         SearchQuery::new("Rust programming language")
//!             .language("en")
//!             .country("us")
//!             .limit(10)?
//!     ).await?;
//!
//!     // Process results
//!     if let Some(organic) = results.organic_results {
//!         for result in organic {
//!             println!("{}: {}", result.title, result.link);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! ### Streaming Results
//!
//! Stream paginated results for large queries:
//!
//! ```rust,no_run
//! use futures::StreamExt;
//! use serp_sdk::{SerpClient, SearchQuery, StreamConfig};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! let mut stream = client.search_stream(
//!     SearchQuery::new("rust tutorials"),
//!     StreamConfig::new().page_size(20)?.max_pages(5)
//! );
//!
//! while let Some(page) = stream.next().await {
//!     match page {
//!         Ok(results) => println!("Got page with {} results",
//!             results.organic_results.as_ref().map_or(0, |r| r.len())),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Specialized Search Types
//!
//! ```rust,no_run
//! # use serp_sdk::{SerpClient, SearchQuery};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SerpClient::builder().api_key("test").build()?;
//! // Image search
//! let images = client.search(
//!     SearchQuery::new("rust logo").images()
//! ).await?;
//!
//! // News search
//! let news = client.search(
//!     SearchQuery::new("rust programming").news()
//! ).await?;
//!
//! // Shopping search
//! let products = client.search(
//!     SearchQuery::new("rust book").shopping()
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Handling
//!
//! The SDK provides comprehensive error handling:
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
//!         // Process results
//!     }
//!     Err(SerpError::RateLimited { retry_after }) => {
//!         println!("Rate limited, retry after {} seconds", retry_after);
//!     }
//!     Err(SerpError::ApiError { code, message }) => {
//!         println!("API error {}: {}", code, message);
//!     }
//!     Err(e) => {
//!         println!("Other error: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration
//!
//! ### Environment Variables
//!
//! - `SERP_API_KEY`: Your SerpAPI key (if not provided via builder)
//!
//! ### Client Configuration
//!
//! ```rust,no_run
//! use serp_sdk::{SerpClient, RetryPolicy};
//! use std::time::Duration;
//!
//! let client = SerpClient::builder()
//!     .api_key("your-key")
//!     .timeout(Duration::from_secs(30))
//!     .retry_policy(
//!         RetryPolicy::new(5)
//!             .with_base_delay(Duration::from_millis(200))
//!             .with_max_delay(Duration::from_secs(30))
//!     )
//!     .build()?;
//! # Ok::<(), serp_sdk::SerpError>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

/// HTTP client for SerpAPI with builder pattern and retry logic.
pub mod client;

/// Comprehensive error types for all SDK operations.
pub mod error;

/// Fluent query builder for constructing search requests.
pub mod query;

/// Strongly-typed response structures for SerpAPI results.
pub mod response;

/// Retry policies with configurable backoff strategies.
pub mod retry;

/// Streaming support for paginated search results.
pub mod streaming;

// Re-export main types for convenience
pub use client::{SerpClient, SerpClientBuilder};
pub use error::{SerpError, SerpResult};
pub use query::{SearchQuery, SearchQueryBuilder};
pub use response::SearchResults;
pub use retry::RetryPolicy;

pub use streaming::StreamConfig;

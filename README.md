# SerpAPI Rust SDK

<div align="center">

  <p><strong>A comprehensive, production-ready Rust SDK for SerpAPI</strong></p>

[![Crates.io](https://img.shields.io/crates/v/serp-sdk.svg)](https://crates.io/crates/serp-sdk)
[![Documentation](https://docs.rs/serp-sdk/badge.svg)](https://docs.rs/serp-sdk)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Build Status](https://github.com/your-org/serp-sdk/workflows/CI/badge.svg)](https://github.com/your-org/serp-sdk/actions)
[![codecov](https://codecov.io/gh/your-org/serp-sdk/branch/main/graph/badge.svg)](https://codecov.io/gh/your-org/serp-sdk)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

</div>

---

A comprehensive, production-ready Rust SDK for the [SerpAPI](https://serpapi.com) service that provides ergonomic APIs, type safety, and async-first design.

> 🏆 **Developed during the [Realtime Search AI Hackathon (Hybrid)](https://www.eventbrite.com/e/realtime-search-ai-hackathon-hybrid-powered-by-serpapi-tickets) powered by SerpAPI and organized by [AI Paris Thinker](https://www.meetup.com/ai-paris-thinker/)**

## Features

- 🦀 **Type-safe**: Strongly typed request builders and response structures
- ⚡ **Async/await**: Built on tokio with efficient async I/O
- 🎯 **Ergonomic**: Fluent builder APIs for constructing queries
- 🔄 **Resilient**: Automatic retry logic with exponential backoff
- 🌊 **Streaming**: Support for paginated result streaming
- 🏭 **Production-ready**: Comprehensive error handling and logging
- 🔍 **Specialized**: Built-in support for images, news, videos, shopping, and local search

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
serp-sdk = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use serp_sdk::{SerpClient, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (API key from env var SERP_API_KEY or builder)
    let client = SerpClient::builder()
        .api_key("your-serp-api-key")
        .build()?;

    // Build and execute search
    let results = client.search(
        SearchQuery::new("Rust programming language")
            .language("en")
            .country("us")
            .limit(10)?
    ).await?;

    // Process results
    if let Some(organic) = results.organic_results {
        for result in organic {
            println!("{}: {}", result.title, result.link);
        }
    }

    Ok(())
}
```

## Advanced Features

### Streaming Results

Stream paginated results for large queries:

```rust
use futures::StreamExt;
use serp_sdk::{SerpClient, SearchQuery, StreamConfig};

let mut stream = client.search_stream(
    SearchQuery::new("rust tutorials"),
    StreamConfig::new()
        .page_size(20)?
        .max_pages(5)
        .delay(std::time::Duration::from_millis(500))
);

while let Some(page) = stream.next().await {
    match page {
        Ok(results) => println!("Got page with {} results",
            results.organic_results.as_ref().map_or(0, |r| r.len())),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Specialized Search Types

#### Image Search

```rust
let images = client.search(
    SearchQuery::new("rust logo").images()
).await?;
```

#### News Search

```rust
let news = client.search(
    SearchQuery::new("rust programming").news()
).await?;
```

#### Video Search

```rust
let videos = client.search(
    SearchQuery::new("rust tutorial").videos()
).await?;
```

#### Shopping Search

```rust
let products = client.search(
    SearchQuery::new("rust book").shopping()
).await?;
```

#### Local Search

```rust
let local = client.search(
    SearchQuery::new("rust meetup")
        .location("San Francisco, CA")
).await?;
```

### Advanced Query Building

```rust
let results = client.search(
    SearchQuery::new("site:github.com rust web framework")
        .language("en")
        .country("us")
        .device("desktop")
        .safe_search("off")
        .domain("google.com")
        .limit(50)?
        .offset(10)
).await?;
```

### Error Handling

The SDK provides comprehensive error handling with the `SerpError` enum:

```rust
match client.search(query).await {
    Ok(results) => {
        // Process results
    }
    Err(SerpError::RateLimited { retry_after }) => {
        println!("Rate limited, retry after {} seconds", retry_after);
    }
    Err(SerpError::ApiError { code, message }) => {
        println!("API error {}: {}", code, message);
    }
    Err(SerpError::MissingApiKey) => {
        println!("Please set SERP_API_KEY environment variable");
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

### Retry Policy Configuration

```rust
use serp_sdk::RetryPolicy;
use std::time::Duration;

let client = SerpClient::builder()
    .api_key("your-key")
    .retry_policy(
        RetryPolicy::new(5) // Max 5 retries
            .with_base_delay(Duration::from_millis(200))
            .with_max_delay(Duration::from_secs(30))
            .with_backoff_multiplier(2.0)
    )
    .build()?;
```

## Configuration

### Environment Variables

- `SERP_API_KEY`: Your SerpAPI key (if not provided via builder)

### Client Configuration

```rust
let client = SerpClient::builder()
    .api_key("your-key")
    .base_url("https://serpapi.com") // Custom base URL
    .timeout(Duration::from_secs(30)) // Request timeout
    .user_agent("my-app/1.0") // Custom User-Agent
    .default_header("X-Custom", "value")? // Add custom headers
    .build()?;
```

## Response Types

The SDK provides strongly-typed response structures:

- `SearchResults`: Complete search response
- `OrganicResult`: Individual organic search result
- `AnswerBox`: Featured snippet/answer box
- `KnowledgeGraph`: Knowledge panel information
- `NewsResult`: News article result
- `VideoResult`: Video search result
- `ShoppingResult`: Shopping/product result
- `LocalPlace`: Local business result

## Examples

The repository includes comprehensive examples:

- [`basic_search.rs`](examples/basic_search.rs): Basic search functionality
- [`streaming.rs`](examples/streaming.rs): Streaming and pagination
- [`specialized_search.rs`](examples/specialized_search.rs): Different search types

Run examples with:

```bash
# Set your API key
export SERP_API_KEY="your-serp-api-key"

# Run basic search example
cargo run --example basic_search

# Run streaming example
cargo run --example streaming

# Run specialized search example
cargo run --example specialized_search
```

## Testing

Run the test suite:

```bash
cargo test
```

Run with logging:

```bash
RUST_LOG=debug cargo test
```

## Features

- `streaming`: Enable streaming support (enabled by default)
- `mcp`: Enable MCP (Model Context Protocol) integration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Performance

The SDK is designed for high performance with minimal overhead:

- **Query Building**: ~54ns for simple queries, ~113ns for complex queries
- **HTTP Client**: Built on reqwest with connection pooling and keepalive
- **Memory Efficient**: Streaming support prevents large result sets from consuming excessive memory
- **Zero-Cost Abstractions**: Leverages Rust's type system for compile-time guarantees

## Roadmap

## Sponsors

This project is supported by our generous sponsors:

<a href="https://serpapi.com">
  <img src="https://serpapi.com/logo.png" alt="SerpAPI" width="200"/>
</a>

## Acknowledgments

- **[Realtime Search AI Hackathon](https://www.eventbrite.com/e/realtime-search-ai-hackathon-hybrid-powered-by-serpapi-tickets)** - This SDK was developed during this innovative hackathon event
- **[AI Paris Thinker](https://www.meetup.com/ai-paris-thinker/)** - For organizing the hackathon and fostering AI innovation
- **[SerpAPI](https://serpapi.com)** - For providing the excellent search API service and sponsoring the hackathon
- The **Rust community** for exceptional async and HTTP libraries
- All **[contributors](https://github.com/your-org/serp-sdk/graphs/contributors)** who help improve this project

## Team

This SDK was developed by:

- **[Hamze Ghalebi](https://www.linkedin.com/in/hamze/)** - Lead Developer
- **[Reetika Gautam](https://www.linkedin.com/in/reetika-gautam/)** - Developer
- **[Leon Carlo](https://www.linkedin.com/in/leoncarlo/)** - Developer

## Roadmap

> 📍 **See [ROADMAP.md](ROADMAP.md) for detailed implementation plans**

### Summary

This SDK is evolving into a comprehensive AI-powered search infrastructure through three strategic phases:

1. **🎯 Rig Integration (Q1 2026)**: Transform the SDK into an intelligent search layer for LLM applications, enabling RAG pipelines, semantic search, and AI agent tools.

2. **🗄️ PostgreSQL Integration (Q2 2026)**: Add persistent caching, search analytics, and query optimization with database-backed storage for enterprise-scale deployments.

3. **🌐 MCP Server (Q3 2026)**: Expose search capabilities to AI assistants like Claude and ChatGPT through the Model Context Protocol, enabling multi-assistant collaboration.

### Phase 1: Rig Rust Integration 🎯
Integration with [Rig](https://github.com/0xPlaygrounds/rig), the Rust library for building LLM-powered applications:
- **LLM-Powered Search**: Combine SerpAPI search results with Rig's LLM capabilities for intelligent search summarization
- **RAG Pipeline**: Use SerpAPI as a real-time data source for Retrieval-Augmented Generation
- **Agent Tools**: Expose SerpAPI search as a tool for Rig agents to use in autonomous workflows
- **Semantic Search**: Enhance search queries with embeddings and semantic understanding

### Phase 2: PostgreSQL Integration 🗄️
Database persistence and caching layer:
- **Search Result Caching**: Store search results in PostgreSQL with configurable TTL
- **Query History**: Track and analyze search patterns over time
- **Result Deduplication**: Intelligent deduplication of search results across queries
- **Full-Text Search**: Combine SerpAPI results with PostgreSQL's full-text search capabilities
- **Analytics Dashboard**: Query performance metrics and usage analytics

### Phase 3: MCP Server Implementation 🌐
Model Context Protocol server for AI assistants:
- **MCP Tools**: Expose SerpAPI search as MCP tools for AI assistants
- **Resource Providers**: Stream search results as MCP resources
- **Context Management**: Intelligent context window management for search results
- **Multi-Assistant Support**: Allow multiple AI assistants to share search context
- **Rate Limiting**: Built-in rate limiting and quota management per assistant

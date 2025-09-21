# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-12-21

### Changed
- Updated documentation attribution for hackathon organizers
- Cleaned up README formatting
- Minor documentation improvements

## [0.1.0] - 2025-12-21

### Added
- Initial release of the SerpAPI Rust SDK
- Comprehensive async client with builder pattern
- Type-safe query construction with fluent API
- Support for all major search types (web, images, videos, news, shopping, local)
- Streaming support for paginated results
- Automatic retry logic with exponential backoff
- Comprehensive error handling with detailed error types
- Production-ready logging and observability
- Full documentation with examples
- Integration tests and benchmarks

### Features
- 🦀 **Type-safe**: Strongly typed request builders and response structures
- ⚡ **Async/await**: Built on tokio with efficient async I/O
- 🎯 **Ergonomic**: Fluent builder APIs for constructing queries
- 🔄 **Resilient**: Automatic retry logic with exponential backoff
- 🌊 **Streaming**: Support for paginated result streaming
- 🏭 **Production-ready**: Comprehensive error handling and logging
- 🔍 **Specialized**: Built-in support for images, news, videos, shopping, and local search

## [0.1.0] - 2024-01-XX

### Added
- Initial implementation of SerpAPI Rust SDK
- Basic client functionality with authentication
- Search query builder with parameter validation
- Response type definitions for all SerpAPI result types
- Error handling with comprehensive error types
- Retry policy configuration
- Streaming support for paginated results
- Documentation and examples
- Unit tests and integration tests
- Benchmarks for performance testing

### Development Context
- 🏆 **Developed during the [Realtime Search AI Hackathon (Hybrid)](https://www.eventbrite.com/e/realtime-search-ai-hackathon-hybrid-powered-by-serpapi-tickets)**
- 🏢 **Organized by [AI Tinkerers Paris](https://paris.aitinkerers.org/)**
- 🚀 **Powered by [SerpAPI](https://serpapi.com)**

### Development Team
- [Hamze Ghalebi](https://www.linkedin.com/in/hamze/) - Lead Developer
- [Reetika Gautam](https://www.linkedin.com/in/reetika-gautam/) - Developer
- [Leon Carlo](https://www.linkedin.com/in/leoncarlo/) - Developer

### Technical Details
- Minimum Rust version: 1.70
- Dependencies: tokio, reqwest, serde, futures, thiserror
- License: MIT OR Apache-2.0
- Repository: https://github.com/your-org/serp-sdk
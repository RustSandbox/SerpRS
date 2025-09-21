# Contributing to SerpAPI Rust SDK

We love your input! We want to make contributing to the SerpAPI Rust SDK as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

### Pull Requests

Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code follows the existing style.
6. Issue that pull request!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/serp-sdk.git
   cd serp-sdk
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run examples (requires API key):
   ```bash
   export SERP_API_KEY="your-api-key"
   cargo run --example basic_search
   ```

## Code Style

We use the standard Rust formatting and linting tools:

- **Formatting**: Use `cargo fmt` to format your code
- **Linting**: Use `cargo clippy` to catch common mistakes
- **Testing**: Use `cargo test` to run all tests
- **Documentation**: Use `cargo doc` to build documentation

### Code Guidelines

- Follow Rust naming conventions
- Write comprehensive documentation for public APIs
- Include examples in documentation where helpful
- Add tests for new functionality
- Use descriptive variable and function names
- Prefer explicit error handling over panics

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration

# Run with output
cargo test -- --nocapture
```

### Writing Tests

- Unit tests go in the same file as the code they test
- Integration tests go in the `tests/` directory
- Use descriptive test names that explain what is being tested
- Test both success and error cases
- Mock external dependencies when possible

## Documentation

We use rustdoc for documentation. All public APIs should be documented with:

- A brief description of what the item does
- Examples showing how to use it
- Information about panics, errors, or safety concerns
- Links to related items where helpful

```rust
/// Creates a new search query for the given search terms.
/// 
/// # Examples
/// 
/// ```rust
/// use serp_sdk::SearchQuery;
/// 
/// let query = SearchQuery::new("rust programming")
///     .language("en")
///     .limit(10)?;
/// ```
/// 
/// # Errors
/// 
/// Returns an error if the limit is outside the valid range (1-100).
pub fn new(query: impl Into<String>) -> SearchQueryBuilder {
    // implementation
}
```

## Benchmarks

We use Criterion for benchmarking. To run benchmarks:

```bash
cargo bench
```

When adding new functionality, consider adding benchmarks if performance is important.

## Submitting Changes

### Commit Messages

Use clear and descriptive commit messages:

- Use the imperative mood ("Add feature" not "Added feature")
- Limit the first line to 72 characters or less
- Reference issues and pull requests where applicable

Example:
```
Add support for video search queries

Implements video search functionality by adding tbm=vid parameter
support to the query builder.

Fixes #123
```

### Pull Request Process

1. **Update Documentation**: Ensure all public APIs are documented
2. **Add Tests**: Include tests for new functionality
3. **Update CHANGELOG**: Add an entry describing your changes
4. **Follow Code Style**: Run `cargo fmt` and `cargo clippy`
5. **Ensure CI Passes**: All tests and checks must pass

## Issue Reporting

### Bug Reports

When filing an issue, make sure to answer these questions:

1. What version of Rust are you using?
2. What version of the SDK are you using?
3. What did you do?
4. What did you expect to see?
5. What did you see instead?

### Feature Requests

We welcome feature requests! Please provide:

1. A clear description of the feature
2. Why you think it would be useful
3. Any relevant examples or use cases

## License

By contributing, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0.

## Questions?

Don't hesitate to ask questions by opening an issue or starting a discussion. We're here to help!

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

## Recognition

Contributors will be recognized in the project's documentation and release notes. Thank you for helping make this project better!
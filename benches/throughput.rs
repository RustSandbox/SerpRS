use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serp_sdk::{SearchQuery, SerpClient};
use std::time::Duration;

// Mock data for benchmarking without actual API calls
fn create_mock_client() -> SerpClient {
    SerpClient::builder()
        .api_key("mock-key")
        .timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to create mock client")
}

fn benchmark_query_builder(c: &mut Criterion) {
    c.bench_function("query_builder_simple", |b| {
        b.iter(|| {
            black_box(
                SearchQuery::new("rust programming")
                    .language("en")
                    .country("us")
                    .limit(10)
                    .unwrap()
            )
        })
    });

    c.bench_function("query_builder_complex", |b| {
        b.iter(|| {
            black_box(
                SearchQuery::new("site:github.com rust web framework")
                    .language("en")
                    .country("us")
                    .device("desktop")
                    .safe_search("off")
                    .domain("google.com")
                    .limit(50)
                    .unwrap()
                    .offset(10)
                    .location("San Francisco, CA")
            )
        })
    });

    c.bench_function("query_builder_specialized", |b| {
        b.iter(|| {
            black_box(
                SearchQuery::new("rust tutorial")
                    .videos()
                    .language("en")
                    .limit(20)
                    .unwrap()
            )
        })
    });
}

fn benchmark_client_creation(c: &mut Criterion) {
    c.bench_function("client_builder", |b| {
        b.iter(|| {
            black_box(create_mock_client())
        })
    });

    c.bench_function("client_builder_with_options", |b| {
        b.iter(|| {
            black_box(
                SerpClient::builder()
                    .api_key("test-key")
                    .timeout(Duration::from_secs(30))
                    .user_agent("benchmark-client")
                    .base_url("https://serpapi.com")
                    .build()
                    .expect("Failed to create client")
            )
        })
    });
}

fn benchmark_serialization(c: &mut Criterion) {
    let query = SearchQuery::new("test query")
        .language("en")
        .country("us")
        .limit(10)
        .unwrap();

    // We need to create a complete query with API key for serialization
    let client = create_mock_client();
    let built_query = SearchQuery::new("test query")
        .language("en")
        .country("us")
        .limit(10)
        .unwrap();

    c.bench_function("query_serialization", |b| {
        b.iter(|| {
            // We can't test the actual serialization without exposing internals
            // This is a placeholder for query creation
            black_box(&built_query)
        })
    });
}

#[cfg(feature = "streaming")]
fn benchmark_streaming_config(c: &mut Criterion) {
    use serp_sdk::StreamConfig;

    c.bench_function("stream_config_creation", |b| {
        b.iter(|| {
            black_box(
                StreamConfig::new()
                    .page_size(10)
                    .unwrap()
                    .max_pages(5)
                    .delay(Duration::from_millis(100))
            )
        })
    });
}

criterion_group!(
    benches,
    benchmark_query_builder,
    benchmark_client_creation,
    benchmark_serialization
);

#[cfg(feature = "streaming")]
criterion_group!(
    streaming_benches,
    benchmark_streaming_config
);

#[cfg(feature = "streaming")]
criterion_main!(benches, streaming_benches);

#[cfg(not(feature = "streaming"))]
criterion_main!(benches);
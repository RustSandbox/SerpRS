use serp_sdk::{SerpClient, SearchQuery, SerpError};
use std::time::Duration;

#[tokio::test]
async fn test_client_builder() {
    let client = SerpClient::builder()
        .api_key("test-key-longer")
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    assert!(client.is_configured());
    assert_eq!(client.api_key_masked(), "test***nger");
}

#[tokio::test]
async fn test_missing_api_key() {
    // Ensure environment variable is not set
    std::env::remove_var("SERP_API_KEY");
    
    let result = SerpClient::builder().build();
    assert!(matches!(result, Err(SerpError::MissingApiKey)));
}

#[tokio::test]
async fn test_empty_api_key() {
    let result = SerpClient::builder().api_key("").build();
    assert!(matches!(result, Err(SerpError::InvalidParameter(_))));
}

#[tokio::test]
async fn test_query_builder() {
    let query_builder = SearchQuery::new("test query")
        .language("en")
        .country("us")
        .limit(10)
        .unwrap();

    // We can't test the build method directly as it requires an API key
    // But we can test the builder pattern compiles correctly
    let _ = query_builder;
}

#[tokio::test]
async fn test_query_limit_validation() {
    let result = SearchQuery::new("test").limit(0);
    assert!(result.is_err());

    let result = SearchQuery::new("test").limit(101);
    assert!(result.is_err());

    let result = SearchQuery::new("test").limit(50);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_specialized_query_builders() {
    let _image_query = SearchQuery::new("test").images();
    let _video_query = SearchQuery::new("test").videos();
    let _news_query = SearchQuery::new("test").news();
    let _shopping_query = SearchQuery::new("test").shopping();
}

#[cfg(feature = "streaming")]
#[tokio::test]
async fn test_stream_config() {
    use serp_sdk::StreamConfig;

    let config = StreamConfig::new()
        .page_size(20)
        .unwrap()
        .max_pages(5)
        .delay(Duration::from_millis(500));

    // We can't test the actual functionality without API access
    // But we can test the configuration
    assert_eq!(config.page_size, 20);
    assert_eq!(config.max_pages, 5);
    assert_eq!(config.delay_between_requests, Duration::from_millis(500));
}

#[cfg(feature = "streaming")]
#[tokio::test]
async fn test_stream_config_validation() {
    use serp_sdk::StreamConfig;

    let result = StreamConfig::new().page_size(0);
    assert!(result.is_err());

    let result = StreamConfig::new().page_size(101);
    assert!(result.is_err());
}

// Mock tests would go here if we were using wiremock
// For now, we focus on testing the builder patterns and validation
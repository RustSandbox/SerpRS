/// Demo that simulates real search functionality with mock responses
/// This shows what the SDK would look like in actual usage

use serp_sdk::{SerpClient, SearchQuery, StreamConfig, SerpError};
use futures::StreamExt;
use std::time::Duration;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 SerpAPI Search Simulation Demo");
    println!("=================================\n");

    // 1. Create client
    println!("1️⃣  Initializing SerpAPI Client...");
    let client = SerpClient::builder()
        .api_key("demo-api-key-12345")
        .timeout(Duration::from_secs(30))
        .user_agent("rust-search-bot/1.0")
        .build()?;

    println!("✅ Client initialized successfully!");
    println!("   API Key: {}", client.api_key_masked());
    println!();

    // 2. Demonstrate different query types
    demonstrate_basic_search(&client).await?;
    demonstrate_specialized_searches(&client).await?;
    demonstrate_streaming_search(&client).await?;
    demonstrate_error_handling(&client).await?;

    println!("🎉 Search simulation completed!");
    println!("\n💡 To run with real API calls:");
    println!("   export SERP_API_KEY='your-actual-key'");
    println!("   cargo run --example basic_search");

    Ok(())
}

async fn demonstrate_basic_search(client: &SerpClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("2️⃣  Basic Search Queries:");
    println!("------------------------");

    // Simple search
    let simple_query = SearchQuery::new("Rust programming language")
        .language("en")
        .country("us")
        .limit(10)?;

    println!("🔍 Query: 'Rust programming language'");
    println!("   Language: en");
    println!("   Country: us");
    println!("   Limit: 10");
    println!("   ❌ Would make API call (simulated)");

    // Advanced search
    let advanced_query = SearchQuery::new("site:github.com rust web framework")
        .language("en")
        .country("us")
        .device("desktop")
        .safe_search("off")
        .limit(20)?
        .offset(10);

    println!("\n🔍 Advanced Query: 'site:github.com rust web framework'");
    println!("   Language: en, Country: us, Device: desktop");
    println!("   SafeSearch: off, Limit: 20, Offset: 10");
    println!("   ❌ Would make API call (simulated)");

    // Show what the response would look like
    println!("\n📋 Simulated Response Structure:");
    print_mock_response();

    println!();
    Ok(())
}

async fn demonstrate_specialized_searches(_client: &SerpClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("3️⃣  Specialized Search Types:");
    println!("-----------------------------");

    // Image search
    let _image_query = SearchQuery::new("rust programming logo").images();
    println!("🖼️  Image Search: 'rust programming logo'");
    println!("   Search Type: images (tbm=isch)");

    // Video search  
    let _video_query = SearchQuery::new("rust tutorial").videos();
    println!("🎥 Video Search: 'rust tutorial'");
    println!("   Search Type: videos (tbm=vid)");

    // News search
    let _news_query = SearchQuery::new("rust programming").news();
    println!("📰 News Search: 'rust programming'");
    println!("   Search Type: news (tbm=nws)");

    // Shopping search
    let _shopping_query = SearchQuery::new("rust programming book").shopping();
    println!("🛒 Shopping Search: 'rust programming book'");
    println!("   Search Type: shopping (tbm=shop)");

    // Local search
    let _local_query = SearchQuery::new("rust meetup")
        .location("San Francisco, CA");
    println!("📍 Local Search: 'rust meetup' in San Francisco, CA");
    println!("   Location: San Francisco, CA");

    println!("   ❌ All searches would make API calls (simulated)");
    println!();
    Ok(())
}

async fn demonstrate_streaming_search(_client: &SerpClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("4️⃣  Streaming Search (Pagination):");
    println!("----------------------------------");

    let stream_config = StreamConfig::new()
        .page_size(10)?
        .max_pages(3)
        .delay(Duration::from_millis(500));

    println!("🌊 Stream Configuration:");
    println!("   Page size: {}", stream_config.page_size);
    println!("   Max pages: {}", stream_config.max_pages);
    println!("   Delay between requests: {:?}", stream_config.delay_between_requests);

    // Simulate what streaming would look like
    println!("\n📄 Simulated Streaming Results:");
    for page in 1..=3 {
        println!("   📄 Page {}: Would fetch 10 results (offset: {})", page, (page - 1) * 10);
        if page < 3 {
            println!("      ⏱️  Waiting 500ms before next request...");
        }
    }

    println!("   ❌ Stream would make 3 API calls (simulated)");
    println!();
    Ok(())
}

async fn demonstrate_error_handling(_client: &SerpClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("5️⃣  Error Handling Examples:");
    println!("----------------------------");

    // Test parameter validation
    println!("🚫 Testing Query Validation:");
    match SearchQuery::new("test").limit(150) {
        Ok(_) => println!("   ❌ Should have failed"),
        Err(e) => println!("   ✅ Caught invalid limit: {}", e),
    }

    match SearchQuery::new("test").limit(0) {
        Ok(_) => println!("   ❌ Should have failed"), 
        Err(e) => println!("   ✅ Caught zero limit: {}", e),
    }

    // Test client validation
    println!("\n🚫 Testing Client Validation:");
    match SerpClient::builder().api_key("").build() {
        Ok(_) => println!("   ❌ Should have failed"),
        Err(e) => println!("   ✅ Caught empty API key: {}", e),
    }

    // Show error types that would occur in real usage
    println!("\n🚫 Real API Error Types (simulated):");
    println!("   • SerpError::RateLimited {{ retry_after: 60 }}");
    println!("   • SerpError::ApiError {{ code: 401, message: \"Invalid API key\" }}");
    println!("   • SerpError::RequestFailed(NetworkTimeout)");
    println!("   • SerpError::InvalidResponse(\"JSON parse error\")");

    println!();
    Ok(())
}

fn print_mock_response() {
    let mock_response = json!({
        "search_metadata": {
            "id": "search_12345",
            "status": "Success",
            "total_time_taken": 0.85,
            "google_url": "https://www.google.com/search?q=rust+programming"
        },
        "search_parameters": {
            "engine": "google",
            "q": "rust programming language",
            "hl": "en",
            "gl": "us"
        },
        "organic_results": [
            {
                "position": 1,
                "title": "Rust Programming Language",
                "link": "https://www.rust-lang.org/",
                "displayed_link": "https://www.rust-lang.org",
                "snippet": "A language empowering everyone to build reliable and efficient software."
            },
            {
                "position": 2,
                "title": "The Rust Programming Language - Official Book",
                "link": "https://doc.rust-lang.org/book/",
                "displayed_link": "https://doc.rust-lang.org",
                "snippet": "The Rust Programming Language by Steve Klabnik and Carol Nichols, with contributions from the Rust Community."
            },
            {
                "position": 3,
                "title": "rust-lang/rust: Empowering everyone to build reliable and ...",
                "link": "https://github.com/rust-lang/rust",
                "displayed_link": "https://github.com",
                "snippet": "Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency."
            }
        ],
        "related_searches": [
            { "query": "rust programming tutorial", "link": "..." },
            { "query": "rust vs c++", "link": "..." },
            { "query": "rust programming examples", "link": "..." }
        ]
    });

    println!("{}", serde_json::to_string_pretty(&mock_response).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_response() {
        // Test that our mock response structure is valid
        print_mock_response();
    }
}
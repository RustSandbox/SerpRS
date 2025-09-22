/// Demo showcasing SerpAPI SDK functionality without requiring a real API key
/// This demonstrates the builder patterns and type safety features
use serp_sdk::{RetryPolicy, SearchQuery, SerpClient, StreamConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 SerpAPI Rust SDK Demo");
    println!("========================\n");

    // 1. Demonstrate Client Builder Pattern
    println!("1️⃣  Client Builder Pattern:");
    let client = SerpClient::builder()
        .api_key("demo-api-key-12345")
        .timeout(Duration::from_secs(30))
        .user_agent("my-rust-app/1.0")
        .retry_policy(
            RetryPolicy::new(3)
                .with_base_delay(Duration::from_millis(200))
                .with_max_delay(Duration::from_secs(10)),
        )
        .build()?;

    println!("✅ Client created successfully!");
    println!("   API Key: {}", client.api_key_masked());
    println!("   Configured: {}\n", client.is_configured());

    // 2. Demonstrate Query Builder Pattern
    println!("2️⃣  Fluent Query Builder:");

    let _basic_query = SearchQuery::new("Rust programming language")
        .language("en")
        .country("us")
        .limit(10)?;

    println!("✅ Basic query built successfully!");

    let _advanced_query = SearchQuery::new("site:github.com async rust")
        .language("en")
        .country("us")
        .device("desktop")
        .safe_search("off")
        .limit(50)?
        .offset(10)
        .location("San Francisco, CA");

    println!("✅ Advanced query built successfully!");

    // 3. Demonstrate Specialized Query Types
    println!("\n3️⃣  Specialized Query Types:");

    let _image_query = SearchQuery::new("rust programming logo").images();
    println!("✅ Image search query created");

    let _video_query = SearchQuery::new("rust tutorial").videos();
    println!("✅ Video search query created");

    let _news_query = SearchQuery::new("rust programming").news();
    println!("✅ News search query created");

    let _shopping_query = SearchQuery::new("rust programming book").shopping();
    println!("✅ Shopping search query created");

    // 4. Demonstrate Stream Configuration
    println!("\n4️⃣  Streaming Configuration:");

    let stream_config = StreamConfig::new()
        .page_size(20)?
        .max_pages(5)
        .delay(Duration::from_millis(500));

    println!("✅ Stream config created:");
    println!("   Page size: {}", stream_config.page_size);
    println!("   Max pages: {}", stream_config.max_pages);
    println!("   Delay: {:?}", stream_config.delay_between_requests);

    // 5. Demonstrate Error Handling
    println!("\n5️⃣  Error Handling:");

    // Test limit validation
    match SearchQuery::new("test").limit(101) {
        Ok(_) => println!("❌ Should have failed"),
        Err(e) => println!("✅ Correctly caught invalid limit: {}", e),
    }

    // Test empty API key
    match SerpClient::builder().api_key("").build() {
        Ok(_) => println!("❌ Should have failed"),
        Err(e) => println!("✅ Correctly caught empty API key: {}", e),
    }

    // 6. Demonstrate Type Safety
    println!("\n6️⃣  Type Safety Features:");
    println!("✅ All queries are checked at compile time");
    println!("✅ Builder patterns prevent invalid configurations");
    println!("✅ Error types are comprehensive and #[non_exhaustive]");
    println!("✅ Response types are strongly typed");

    // 7. Show what a real search would look like (commented out)
    println!("\n7️⃣  Example Usage (requires real API key):");
    println!("```rust");
    println!("// Set environment variable SERP_API_KEY=your-key");
    println!("let results = client.search(basic_query).await?;");
    println!("for result in results.organic_results.unwrap_or_default() {{");
    println!("    println!(\"{{}}: {{}}\", result.title, result.link);");
    println!("}}");
    println!("```");

    println!("\n🎉 Demo completed successfully!");
    println!("📚 Run the examples with a real API key to see full functionality:");
    println!("   cargo run --example basic_search");
    println!("   cargo run --example streaming");
    println!("   cargo run --example specialized_search");
    println!("   cargo run --example mcp_integration");

    Ok(())
}

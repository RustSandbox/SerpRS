use serp_sdk::{SerpClient, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create client with API key from environment
    let client = SerpClient::builder()
        .build()?;

    // Execute a simple search
    let results = client.search(
        SearchQuery::new("Rust programming language")
            .language("en")
            .country("us")
            .limit(5)?
    ).await?;

    println!("Search completed successfully!");
    println!("Search ID: {}", results.search_metadata.id);
    
    if let Some(organic) = results.organic_results {
        println!("\nTop {} results:", organic.len());
        for (i, result) in organic.iter().enumerate() {
            println!("{}. {}", i + 1, result.title);
            println!("   {}", result.link);
            if let Some(snippet) = &result.snippet {
                println!("   {}", snippet);
            }
            println!();
        }
    }

    Ok(())
}

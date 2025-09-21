use serp_sdk::{SearchQuery, SerpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get API key from environment or command line
    let api_key = env::args()
        .nth(1)
        .or_else(|| env::var("SERP_API_KEY").ok())
        .expect("Please provide API key as argument or set SERP_API_KEY environment variable");

    // Initialize client with builder pattern
    let client = SerpClient::builder()
        .api_key(api_key)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    println!("🔍 Searching for 'Rust programming language'...\n");

    // Execute search with fluent query builder
    let results = client
        .search(
            SearchQuery::new("Rust programming language")
                .language("en")
                .country("us")
                .limit(10)?,
        )
        .await?;

    // Display metadata
    println!("✅ Search completed successfully!");
    println!("📊 Search ID: {}", results.search_metadata.id);
    println!(
        "⏱️  Total time: {:.2}s",
        results.search_metadata.total_time_taken
    );
    println!("🌐 Google URL: {}", results.search_metadata.google_url);
    println!();

    // Display organic results
    if let Some(organic) = results.organic_results {
        println!("📋 Found {} organic results:\n", organic.len());

        for (i, result) in organic.iter().enumerate() {
            println!("{}. 📎 {}", i + 1, result.title);
            println!("   🔗 {}", result.link);

            if let Some(snippet) = &result.snippet {
                println!("   📄 {}", snippet);
            }

            if let Some(date) = &result.date {
                println!("   📅 {}", date);
            }

            println!();
        }
    } else {
        println!("❌ No organic results found");
    }

    // Display answer box if available
    if let Some(answer_box) = results.answer_box {
        println!("💡 Answer Box:");
        println!("   Type: {}", answer_box.answer_type);

        if let Some(title) = answer_box.title {
            println!("   Title: {}", title);
        }

        if let Some(answer) = answer_box.answer {
            println!("   Answer: {}", answer);
        }

        if let Some(snippet) = answer_box.snippet {
            println!("   Snippet: {}", snippet);
        }

        println!();
    }

    // Display knowledge graph if available
    if let Some(kg) = results.knowledge_graph {
        println!("🧠 Knowledge Graph:");
        println!("   Title: {}", kg.title);

        if let Some(description) = kg.description {
            println!("   Description: {}", description);
        }

        if let Some(kg_type) = kg.knowledge_type {
            println!("   Type: {}", kg_type);
        }

        println!();
    }

    // Display related searches
    if let Some(related) = results.related_searches {
        println!("🔗 Related searches:");
        for search in related.iter().take(5) {
            println!("   • {}", search.query);
        }
        println!();
    }

    // Display pagination info
    if let Some(pagination) = results.pagination {
        println!("📄 Pagination:");
        println!("   Current page: {}", pagination.current);

        if let Some(next) = pagination.next {
            println!("   Next page available: {}", next);
        }

        println!();
    }

    Ok(())
}

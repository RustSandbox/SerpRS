use futures::StreamExt;
use serp_sdk::{SearchQuery, SerpClient, StreamConfig};
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

    // Initialize client
    let client = SerpClient::builder().api_key(api_key).build()?;

    println!("🌊 Streaming search results for 'Rust tutorials'...\n");

    // Configure streaming with 5 results per page, max 3 pages
    let stream_config = StreamConfig::new()
        .page_size(5)?
        .max_pages(3)
        .delay(std::time::Duration::from_millis(500)); // Be respectful to the API

    // Create search stream
    let mut stream = client.search_stream(
        SearchQuery::new("Rust tutorials")
            .language("en")
            .country("us"),
        stream_config,
    );

    let mut page_number = 1;
    let mut total_results = 0;

    // Process each page as it arrives
    while let Some(result) = stream.next().await {
        match result {
            Ok(page) => {
                println!("📄 Page {} Results:", page_number);
                println!(
                    "   ⏱️  Time taken: {:.2}s",
                    page.search_metadata.total_time_taken
                );

                if let Some(organic) = page.organic_results {
                    println!("   📊 Results on this page: {}", organic.len());
                    total_results += organic.len();

                    for (i, result) in organic.iter().enumerate() {
                        let global_position = (page_number - 1) * 5 + i + 1;
                        println!("   {}. {}", global_position, result.title);
                        println!("      🔗 {}", result.link);

                        if let Some(snippet) = &result.snippet {
                            let truncated = if snippet.len() > 100 {
                                format!("{}...", &snippet[..100])
                            } else {
                                snippet.clone()
                            };
                            println!("      📄 {}", truncated);
                        }
                    }
                } else {
                    println!("   ❌ No organic results on this page");
                }

                println!();
                page_number += 1;
            }
            Err(e) => {
                eprintln!("❌ Error fetching page {}: {}", page_number, e);
                break;
            }
        }
    }

    println!("✅ Streaming completed!");
    println!("📊 Total results collected: {}", total_results);

    // Demonstrate streaming individual results
    println!("\n🔄 Now streaming individual organic results...\n");

    let mut individual_stream = client.organic_results_stream(
        SearchQuery::new("Rust async programming"),
        StreamConfig::new().page_size(3)?.max_pages(2),
    );

    let mut count = 0;
    while let Some(result) = individual_stream.next().await {
        match result {
            Ok(organic) => {
                count += 1;
                println!("{}. 📎 {}", count, organic.title);
                println!("   🔗 {}", organic.link);
                println!("   📍 Position: {}", organic.position);
                println!();
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }

    println!("✅ Individual streaming completed!");
    println!("📊 Total individual results: {}", count);

    // Demonstrate conditional streaming (stop when we find a specific domain)
    println!("\n🎯 Streaming until we find a rust-lang.org result...\n");

    let mut conditional_stream = client.search_until(
        SearchQuery::new("Rust documentation"),
        StreamConfig::new().page_size(10)?.max_pages(5),
        |page| {
            // Stop if we find a result from rust-lang.org
            page.organic_results.as_ref().map_or(false, |results| {
                results.iter().any(|r| r.link.contains("rust-lang.org"))
            })
        },
    );

    let mut found_target = false;
    while let Some(result) = conditional_stream.next().await {
        match result {
            Ok(page) => {
                if let Some(organic) = page.organic_results {
                    for result in organic {
                        println!("🔍 Checking: {}", result.link);
                        if result.link.contains("rust-lang.org") {
                            println!("🎯 Found target! {}", result.title);
                            println!("   🔗 {}", result.link);
                            found_target = true;
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                break;
            }
        }
    }

    if found_target {
        println!("\n✅ Successfully found rust-lang.org result!");
    } else {
        println!("\n❌ Target not found in searched pages");
    }

    Ok(())
}

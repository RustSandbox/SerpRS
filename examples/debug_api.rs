use reqwest;
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("SERP_API_KEY")
        .expect("Please set SERP_API_KEY environment variable");

    let client = reqwest::Client::new();
    
    let params = [
        ("engine", "google"),
        ("q", "Rust tutorials"),
        ("api_key", &api_key),
        ("num", "5"),
        ("hl", "en"),
        ("gl", "us"),
    ];
    
    let response = client
        .get("https://serpapi.com/search.json")
        .query(&params)
        .send()
        .await?;
        
    let text = response.text().await?;
    
    // Parse as JSON Value to see structure
    let json: Value = serde_json::from_str(&text)?;
    
    // Pretty print the JSON
    println!("{}", serde_json::to_string_pretty(&json)?);
    
    Ok(())
}
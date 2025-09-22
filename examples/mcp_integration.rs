// MCP (Model Context Protocol) integration example
// This demonstrates how the SerpAPI SDK could be integrated as an MCP tool

use serde_json::{json, Value};
use serp_sdk::{SearchQuery, SerpClient};
use std::env;

/// MCP tool implementation for web search
pub struct SerpSearchTool {
    client: SerpClient,
}

impl SerpSearchTool {
    /// Create a new MCP search tool
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = SerpClient::builder().api_key(api_key).build()?;

        Ok(Self { client })
    }

    /// Execute a search and return MCP-compatible results
    pub async fn execute_search(&self, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // Parse MCP parameters
        let query = params["query"]
            .as_str()
            .ok_or("Missing 'query' parameter")?;

        let language = params["language"].as_str();
        let country = params["country"].as_str();
        let limit = params["limit"].as_u64().map(|n| n as u32);
        let search_type = params["type"].as_str();

        // Build search query
        let mut search_builder = SearchQuery::new(query);

        if let Some(lang) = language {
            search_builder = search_builder.language(lang);
        }

        if let Some(country_code) = country {
            search_builder = search_builder.country(country_code);
        }

        if let Some(limit_val) = limit {
            search_builder = search_builder.limit(limit_val)?;
        }

        // Apply search type
        search_builder = match search_type {
            Some("images") => search_builder.images(),
            Some("videos") => search_builder.videos(),
            Some("news") => search_builder.news(),
            Some("shopping") => search_builder.shopping(),
            _ => search_builder, // Default to web search
        };

        // Execute search
        let results = self.client.search(search_builder).await?;

        // Convert to MCP-compatible format
        let mut mcp_results = json!({
            "search_metadata": {
                "id": results.search_metadata.id,
                "status": results.search_metadata.status,
                "total_time_taken": results.search_metadata.total_time_taken,
                "query": query
            }
        });

        // Add organic results
        if let Some(organic) = results.organic_results {
            let organic_json: Vec<Value> = organic
                .into_iter()
                .map(|result| {
                    json!({
                        "position": result.position,
                        "title": result.title,
                        "link": result.link,
                        "displayed_link": result.displayed_link,
                        "snippet": result.snippet
                    })
                })
                .collect();

            mcp_results["organic_results"] = json!(organic_json);
        }

        // Add answer box if available
        if let Some(answer_box) = results.answer_box {
            mcp_results["answer_box"] = json!({
                "type": answer_box.answer_type,
                "title": answer_box.title,
                "answer": answer_box.answer,
                "snippet": answer_box.snippet
            });
        }

        // Add knowledge graph if available
        if let Some(kg) = results.knowledge_graph {
            mcp_results["knowledge_graph"] = json!({
                "title": kg.title,
                "type": kg.knowledge_type,
                "description": kg.description
            });
        }

        // Add related searches
        if let Some(related) = results.related_searches {
            let mut related_json: Vec<Value> = Vec::new();
            
            for search in related {
                match search {
                    serp_sdk::response::RelatedSearch::Simple { query, link, .. } => {
                        related_json.push(json!({
                            "query": query,
                            "link": link
                        }));
                    }
                    serp_sdk::response::RelatedSearch::Block { items, .. } => {
                        for item in items {
                            if let Some(name) = item.name {
                                related_json.push(json!({
                                    "query": name,
                                    "link": item.link
                                }));
                            }
                        }
                    }
                }
            }

            mcp_results["related_searches"] = json!(related_json);
        }

        Ok(mcp_results)
    }
}

/// MCP tool schema definition
pub fn get_tool_schema() -> Value {
    json!({
        "name": "web_search",
        "description": "Search the web using SerpAPI and return structured results",
        "parameters": {
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                },
                "language": {
                    "type": "string",
                    "description": "Interface language (e.g., 'en', 'es', 'fr')",
                    "default": "en"
                },
                "country": {
                    "type": "string",
                    "description": "Country for search results (e.g., 'us', 'uk', 'ca')",
                    "default": "us"
                },
                "limit": {
                    "type": "integer",
                    "description": "Number of results to return (1-100)",
                    "minimum": 1,
                    "maximum": 100,
                    "default": 10
                },
                "type": {
                    "type": "string",
                    "description": "Type of search",
                    "enum": ["web", "images", "videos", "news", "shopping"],
                    "default": "web"
                }
            },
            "required": ["query"]
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get API key
    let api_key = env::args()
        .nth(1)
        .or_else(|| env::var("SERP_API_KEY").ok())
        .expect("Please provide API key as argument or set SERP_API_KEY environment variable");

    // Create MCP tool
    let search_tool = SerpSearchTool::new(api_key)?;

    println!("🔧 MCP Tool Schema:");
    println!("{}", serde_json::to_string_pretty(&get_tool_schema())?);
    println!();

    // Example MCP tool execution
    println!("🔍 Executing MCP search tool...");

    let search_params = json!({
        "query": "Rust programming language",
        "language": "en",
        "country": "us",
        "limit": 5,
        "type": "web"
    });

    println!("📋 Input parameters:");
    println!("{}", serde_json::to_string_pretty(&search_params)?);
    println!();

    // Execute search
    let results = search_tool.execute_search(search_params).await?;

    println!("✅ MCP search results:");
    println!("{}", serde_json::to_string_pretty(&results)?);

    // Example of different search types
    println!("\n🖼️  Testing image search...");

    let image_params = json!({
        "query": "rust programming logo",
        "type": "images",
        "limit": 3
    });

    let image_results = search_tool.execute_search(image_params).await?;
    println!("📸 Image search results:");
    println!("{}", serde_json::to_string_pretty(&image_results)?);

    println!("\n🎯 MCP integration example completed!");
    println!("💡 This tool can be integrated into any MCP-compatible system");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_schema() {
        let schema = get_tool_schema();
        assert_eq!(schema["name"], "web_search");
        assert!(schema["parameters"]["properties"]["query"].is_object());
    }

    #[tokio::test]
    async fn test_search_tool_creation() {
        let result = SerpSearchTool::new("test-key".to_string());
        assert!(result.is_ok());
    }
}

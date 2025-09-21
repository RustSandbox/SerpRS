use serp_sdk::{SerpClient, SearchQuery};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get API key from environment or command line
    let api_key = env::args().nth(1)
        .or_else(|| env::var("SERP_API_KEY").ok())
        .expect("Please provide API key as argument or set SERP_API_KEY environment variable");

    // Initialize client
    let client = SerpClient::builder()
        .api_key(api_key)
        .build()?;

    // Image Search
    println!("🖼️  Image Search for 'Rust programming logo'...\n");
    
    let image_results = client.search(
        SearchQuery::new("Rust programming logo")
            .images()
            .language("en")
            .limit(5)?
    ).await?;

    if let Some(images) = image_results.inline_images {
        println!("📸 Found {} images:", images.len());
        for (i, image) in images.iter().enumerate() {
            println!("{}. {}", i + 1, image.title);
            println!("   🔗 Source: {}", image.source);
            println!("   🖼️  Thumbnail: {}", image.thumbnail);
            println!("   📏 Original: {}", image.original);
            println!();
        }
    } else {
        println!("❌ No images found");
    }

    // News Search
    println!("📰 News Search for 'Rust programming language'...\n");
    
    let news_results = client.search(
        SearchQuery::new("Rust programming language")
            .news()
            .language("en")
            .limit(5)?
    ).await?;

    if let Some(news) = news_results.news_results {
        println!("📰 Found {} news articles:", news.len());
        for (i, article) in news.iter().enumerate() {
            println!("{}. {}", i + 1, article.title);
            println!("   📅 Date: {}", article.date);
            println!("   📰 Source: {}", article.source);
            println!("   🔗 Link: {}", article.link);
            
            if let Some(snippet) = &article.snippet {
                println!("   📄 {}", snippet);
            }
            
            if let Some(thumbnail) = &article.thumbnail {
                println!("   🖼️  Thumbnail: {}", thumbnail);
            }
            
            println!();
        }
    } else {
        println!("❌ No news articles found");
    }

    // Video Search
    println!("🎥 Video Search for 'Rust tutorial'...\n");
    
    let video_results = client.search(
        SearchQuery::new("Rust tutorial")
            .videos()
            .language("en")
            .limit(5)?
    ).await?;

    if let Some(videos) = video_results.video_results {
        println!("🎥 Found {} videos:", videos.len());
        for (i, video) in videos.iter().enumerate() {
            println!("{}. {}", i + 1, video.title);
            println!("   📺 Channel: {}", video.channel);
            println!("   ⏱️  Duration: {}", video.duration);
            println!("   🏷️  Platform: {}", video.platform);
            println!("   📅 Date: {}", video.date);
            println!("   🔗 Link: {}", video.link);
            println!("   🖼️  Thumbnail: {}", video.thumbnail);
            println!();
        }
    } else {
        println!("❌ No videos found");
    }

    // Shopping Search
    println!("🛒 Shopping Search for 'Rust programming book'...\n");
    
    let shopping_results = client.search(
        SearchQuery::new("Rust programming book")
            .shopping()
            .language("en")
            .limit(5)?
    ).await?;

    if let Some(products) = shopping_results.shopping_results {
        println!("🛒 Found {} products:", products.len());
        for (i, product) in products.iter().enumerate() {
            println!("{}. {}", i + 1, product.title);
            println!("   💰 Price: {}", product.price);
            
            if let Some(rating) = product.rating {
                println!("   ⭐ Rating: {:.1}", rating);
            }
            
            if let Some(reviews) = product.reviews {
                println!("   📝 Reviews: {}", reviews);
            }
            
            println!("   🏪 Source: {}", product.source);
            println!("   🔗 Product Link: {}", product.product_link);
            println!("   🖼️  Thumbnail: {}", product.thumbnail);
            
            if let Some(extensions) = &product.extensions {
                println!("   🏷️  Tags: {}", extensions.join(", "));
            }
            
            println!();
        }
    } else {
        println!("❌ No products found");
    }

    // Local Search with Location
    println!("📍 Local Search for 'Rust programming meetup' in Austin, Texas...\n");
    
    let local_results = client.search(
        SearchQuery::new("Rust programming meetup")
            .location("Austin, Texas")
            .language("en")
            .limit(5)?
    ).await?;

    if let Some(local) = local_results.local_results {
        println!("📍 Found {} local places:", local.places.len());
        for (i, place) in local.places.iter().enumerate() {
            println!("{}. {}", i + 1, place.title);
            println!("   📍 Address: {}", place.address);
            
            if let Some(rating) = place.rating {
                println!("   ⭐ Rating: {:.1}", rating);
            }
            
            if let Some(reviews) = place.reviews {
                println!("   📝 Reviews: {}", reviews);
            }
            
            if let Some(phone) = &place.phone {
                println!("   📞 Phone: {}", phone);
            }
            
            if let Some(website) = &place.website {
                println!("   🌐 Website: {}", website);
            }
            
            if let Some(hours) = &place.hours {
                println!("   🕒 Hours: {}", hours);
            }
            
            if let Some(coords) = &place.gps_coordinates {
                println!("   🗺️  GPS: {:.6}, {:.6}", coords.latitude, coords.longitude);
            }
            
            println!();
        }
    } else {
        println!("❌ No local results found");
    }

    // Advanced Search with Multiple Parameters
    println!("🔧 Advanced Search with multiple parameters...\n");
    
    let advanced_results = client.search(
        SearchQuery::new("site:github.com Rust web framework")
            .language("en")
            .country("us")
            .device("desktop")
            .safe_search("off")
            .limit(5)?
    ).await?;

    println!("🔍 Advanced search completed!");
    println!("📊 Search parameters:");
    println!("   🌐 Language: {}", advanced_results.search_parameters.language.unwrap_or("default".to_string()));
    println!("   🗺️  Location: {}", advanced_results.search_parameters.geolocation.unwrap_or("default".to_string()));
    println!("   🔍 Query: {}", advanced_results.search_parameters.query);
    
    if let Some(organic) = advanced_results.organic_results {
        println!("   📋 Results found: {}", organic.len());
        for result in organic.iter().take(3) {
            println!("   • {}", result.title);
            println!("     {}", result.link);
        }
    }

    println!("\n✅ All specialized searches completed!");

    Ok(())
}
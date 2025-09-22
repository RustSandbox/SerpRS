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

    // Initialize client
    let client = SerpClient::builder().api_key(api_key).build()?;

    // Image Search
    println!("🖼️  Image Search for 'Rust programming logo'...\n");

    let image_results = client
        .search(
            SearchQuery::new("Rust programming logo")
                .images()
                .language("en")
                .limit(5)?,
        )
        .await?;

    if let Some(images) = image_results.inline_images {
        println!("📸 Found {} images:", images.len());
        for (i, image) in images.iter().enumerate() {
            println!(
                "{}. {}",
                i + 1,
                image.title.as_deref().unwrap_or("Untitled")
            );
            if let Some(source) = &image.source {
                println!("   🔗 Source: {}", source);
            }
            if let Some(thumbnail) = &image.thumbnail {
                println!("   🖼️  Thumbnail: {}", thumbnail);
            }
            if let Some(original) = &image.original {
                println!("   📏 Original: {}", original);
            }
            println!();
        }
    } else {
        println!("❌ No images found");
    }

    // News Search
    println!("📰 News Search for 'Rust programming language'...\n");

    let news_results = client
        .search(
            SearchQuery::new("Rust programming language")
                .news()
                .language("en")
                .limit(5)?,
        )
        .await?;

    if let Some(news) = news_results.news_results {
        println!("📰 Found {} news articles:", news.len());
        for (i, article) in news.iter().enumerate() {
            println!("{}. {}", i + 1, article.title);
            if let Some(date) = &article.date {
                println!("   📅 Date: {}", date);
            }
            if let Some(source) = &article.source {
                println!("   📰 Source: {}", source);
            }
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

    let video_results = client
        .search(
            SearchQuery::new("Rust tutorial")
                .videos()
                .language("en")
                .limit(5)?,
        )
        .await?;

    if let Some(videos) = video_results.video_results {
        println!("🎥 Found {} videos:", videos.len());
        for (i, video) in videos.iter().enumerate() {
            println!("{}. {}", i + 1, video.title);
            if let Some(channel) = &video.channel {
                println!("   📺 Channel: {}", channel);
            }
            if let Some(duration) = &video.duration {
                println!("   ⏱️  Duration: {}", duration);
            }
            if let Some(platform) = &video.platform {
                println!("   🏷️  Platform: {}", platform);
            }
            if let Some(date) = &video.date {
                println!("   📅 Date: {}", date);
            }
            println!("   🔗 Link: {}", video.link);
            if let Some(thumbnail) = &video.thumbnail {
                println!("   🖼️  Thumbnail: {}", thumbnail);
            }
            println!();
        }
    } else if let Some(inline_videos) = video_results.inline_videos {
        // Try inline videos instead
        println!("🎥 Found {} inline videos:", inline_videos.len());
        for (i, video) in inline_videos.iter().enumerate() {
            println!(
                "{}. {}",
                i + 1,
                video.title.as_deref().unwrap_or("Untitled")
            );
            if let Some(channel) = &video.channel {
                println!("   📺 Channel: {}", channel);
            }
            if let Some(duration) = &video.duration {
                println!("   ⏱️  Duration: {}", duration);
            }
            if let Some(platform) = &video.platform {
                println!("   🏷️  Platform: {}", platform);
            }
            if let Some(link) = &video.link {
                println!("   🔗 Link: {}", link);
            }
            if let Some(thumbnail) = &video.thumbnail {
                println!("   🖼️  Thumbnail: {}", thumbnail);
            }
            println!();
        }
    } else {
        println!("❌ No videos found");
    }

    // Shopping Search
    println!("🛒 Shopping Search for 'Rust programming book'...\n");

    let shopping_results = client
        .search(
            SearchQuery::new("Rust programming book")
                .shopping()
                .language("en")
                .limit(5)?,
        )
        .await?;

    if let Some(products) = shopping_results.shopping_results {
        println!("🛒 Found {} products:", products.len());
        for (i, product) in products.iter().enumerate() {
            println!("{}. {}", i + 1, product.title);
            if let Some(price) = &product.price {
                println!("   💰 Price: {}", price);
            }

            if let Some(rating) = product.rating {
                println!("   ⭐ Rating: {:.1}", rating);
            }

            if let Some(reviews) = product.reviews {
                println!("   📝 Reviews: {}", reviews);
            }

            if let Some(source) = &product.source {
                println!("   🏪 Source: {}", source);
            }
            if let Some(product_link) = &product.product_link {
                println!("   🔗 Product Link: {}", product_link);
            }
            if let Some(thumbnail) = &product.thumbnail {
                println!("   🖼️  Thumbnail: {}", thumbnail);
            }

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

    let local_results = client
        .search(
            SearchQuery::new("Rust programming meetup")
                .location("Austin, Texas")
                .language("en")
                .limit(5)?,
        )
        .await?;

    if let Some(local) = local_results.local_results {
        if let Some(places) = &local.places {
            println!("📍 Found {} local places:", places.len());
            for (i, place) in places.iter().enumerate() {
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
                    println!(
                        "   🗺️  GPS: {:.6}, {:.6}",
                        coords.latitude, coords.longitude
                    );
                }

                println!();
            }
        } else {
            println!("❌ No local places found");
        }
    } else {
        println!("❌ No local results found");
    }

    // Advanced Search with Multiple Parameters
    println!("🔧 Advanced Search with multiple parameters...\n");

    let advanced_results = client
        .search(
            SearchQuery::new("site:github.com Rust async")
                .language("en")
                .country("us")
                .device("desktop")
                .safe_search("off")
                .limit(10)?,
        )
        .await?;

    println!("✅ Advanced search completed!");
    println!("   🆔 Search ID: {}", advanced_results.search_metadata.id);
    if let Some(time_taken) = advanced_results.search_metadata.total_time_taken {
        println!("   ⏱️  Time taken: {:.2}s", time_taken);
    }

    if let Some(organic) = advanced_results.organic_results {
        println!("   📊 Found {} results", organic.len());

        for (i, result) in organic.iter().take(3).enumerate() {
            println!("\n   {}. {}", i + 1, result.title);
            println!("      🔗 {}", result.link);

            if let Some(date) = &result.date {
                println!("      📅 {}", date);
            }

            if let Some(snippet) = &result.snippet {
                println!("      📄 {}", snippet);
            }
        }
    }

    println!("\n✨ All specialized searches completed successfully!");

    Ok(())
}

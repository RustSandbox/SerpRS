#![allow(missing_docs)]

use serde::Deserialize;
use std::collections::HashMap;

/// Complete search results from SerpAPI.
///
/// This is the main response structure returned by search operations.
/// It contains all the different types of results that Google can return,
/// including organic results, ads, knowledge panels, and more.
///
/// # Examples
///
/// ```rust,no_run
/// use serp_sdk::{SerpClient, SearchQuery};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = SerpClient::builder().api_key("test").build()?;
/// let results = client.search(SearchQuery::new("rust programming")).await?;
///
/// // Process organic results
/// if let Some(organic) = results.organic_results {
///     for result in organic {
///         println!("{}: {}", result.title, result.link);
///     }
/// }
///
/// // Check for knowledge graph
/// if let Some(kg) = results.knowledge_graph {
///     println!("Knowledge panel: {}", kg.title);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SearchResults {
    /// Metadata about the search request execution
    pub search_metadata: SearchMetadata,
    /// Parameters that were used for the search
    pub search_parameters: SearchParameters,
    /// Main organic search results
    pub organic_results: Option<Vec<OrganicResult>>,
    /// Featured snippet or answer box
    pub answer_box: Option<AnswerBox>,
    /// Knowledge graph panel information
    pub knowledge_graph: Option<KnowledgeGraph>,
    /// Related search suggestions
    pub related_searches: Option<Vec<RelatedSearch>>,
    /// Pagination information for multi-page results
    pub pagination: Option<Pagination>,
    /// Sponsored advertisements
    pub ads: Option<Vec<Ad>>,
    /// Shopping/product results
    pub shopping_results: Option<Vec<ShoppingResult>>,
    /// Local business results
    pub local_results: Option<LocalResults>,
    /// News article results
    pub news_results: Option<Vec<NewsResult>>,
    /// Video search results
    pub video_results: Option<Vec<VideoResult>>,
    /// Inline image results
    pub inline_images: Option<Vec<InlineImage>>,
    /// Inline video results
    pub inline_videos: Option<Vec<InlineVideo>>,
    /// Short video results
    pub short_videos: Option<Vec<ShortVideo>>,
    /// Search information
    pub search_information: Option<SearchInformation>,
    /// SerpAPI pagination
    pub serpapi_pagination: Option<SerpapiPagination>,
}

/// Metadata about the search request execution.
///
/// Contains information about how the search was processed, including
/// timing data and unique identifiers for the request.
#[derive(Debug, Deserialize, Clone)]
pub struct SearchMetadata {
    /// Unique identifier for this search request
    pub id: String,
    /// Status of the search request ("Success", "Error", etc.)
    pub status: Option<String>,
    /// API endpoint URL for this specific search
    pub json_endpoint: Option<String>,
    /// Timestamp when the search was initiated
    pub created_at: Option<String>,
    /// Timestamp when the search was completed
    pub processed_at: Option<String>,
    /// Google search URL that would produce similar results
    pub google_url: Option<String>,
    /// URL to raw HTML file (if available)
    pub raw_html_file: Option<String>,
    /// Total processing time in seconds
    pub total_time_taken: Option<f64>,
    /// Pixel position endpoint
    pub pixel_position_endpoint: Option<String>,
}

/// Parameters used for the search
#[derive(Debug, Deserialize, Clone)]
pub struct SearchParameters {
    pub engine: String,
    #[serde(rename = "q")]
    pub query: String,
    pub google_domain: Option<String>,
    #[serde(rename = "gl")]
    pub geolocation: Option<String>,
    #[serde(rename = "hl")]
    pub language: Option<String>,
    pub device: Option<String>,
}

/// Organic search result
#[derive(Debug, Deserialize, Clone)]
pub struct OrganicResult {
    pub position: Option<u32>,
    pub title: String,
    pub link: String,
    pub displayed_link: Option<String>,
    pub snippet: Option<String>,
    pub snippet_highlighted_words: Option<Vec<String>>,
    pub cached_page_link: Option<String>,
    pub date: Option<String>,
    pub rich_snippet: Option<RichSnippet>,
    pub about_this_result: Option<AboutThisResult>,
}

/// Rich snippet information
#[derive(Debug, Deserialize, Clone)]
pub struct RichSnippet {
    pub top: Option<HashMap<String, String>>,
    pub bottom: Option<HashMap<String, String>>,
}

/// About this result information
#[derive(Debug, Deserialize, Clone)]
pub struct AboutThisResult {
    pub source: Option<Source>,
    pub keywords: Option<Vec<String>>,
    pub related_keywords: Option<Vec<String>>,
}

/// Source information
#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub description: Option<String>,
    pub source_info_link: Option<String>,
    pub security: Option<String>,
}

/// Answer box result
#[derive(Debug, Deserialize, Clone)]
pub struct AnswerBox {
    #[serde(rename = "type")]
    pub answer_type: String,
    pub title: Option<String>,
    pub answer: Option<String>,
    pub snippet: Option<String>,
    pub snippet_highlighted_words: Option<Vec<String>>,
    pub link: Option<String>,
    pub displayed_link: Option<String>,
}

/// Knowledge graph panel
#[derive(Debug, Deserialize, Clone)]
pub struct KnowledgeGraph {
    pub title: String,
    #[serde(rename = "type")]
    pub knowledge_type: Option<String>,
    pub kgmid: Option<String>,
    pub knowledge_graph_search_link: Option<String>,
    pub serpapi_knowledge_graph_search_link: Option<String>,
    pub description: Option<String>,
    pub source: Option<Source>,
    pub thumbnail: Option<String>,
}

/// Related search suggestion
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum RelatedSearch {
    Simple {
        query: String,
        link: Option<String>,
        serpapi_link: Option<String>,
    },
    Block {
        block_position: Option<u32>,
        items: Vec<RelatedSearchItem>,
    },
}

/// Related search item
#[derive(Debug, Deserialize, Clone)]
pub struct RelatedSearchItem {
    pub name: Option<String>,
    pub query: Option<String>,
    pub link: Option<String>,
    pub serpapi_link: Option<String>,
    pub image: Option<String>,
    pub stick: Option<String>,
}

/// Pagination information
#[derive(Debug, Deserialize, Clone)]
pub struct Pagination {
    pub current: u32,
    pub next: Option<String>,
    pub next_link: Option<String>,
    pub serpapi_next_link: Option<String>,
    pub other_pages: Option<HashMap<String, String>>,
}

/// Advertisement result
#[derive(Debug, Deserialize, Clone)]
pub struct Ad {
    pub position: Option<u32>,
    pub title: String,
    pub link: String,
    pub displayed_link: Option<String>,
    pub description: Option<String>,
    pub sitelinks: Option<Vec<SiteLink>>,
}

/// Site link in advertisement
#[derive(Debug, Deserialize, Clone)]
pub struct SiteLink {
    pub title: String,
    pub link: String,
}

/// Shopping result
#[derive(Debug, Deserialize, Clone)]
pub struct ShoppingResult {
    pub position: Option<u32>,
    pub title: String,
    pub link: Option<String>,
    pub product_link: Option<String>,
    pub product_id: Option<String>,
    pub serpapi_product_api: Option<String>,
    pub source: Option<String>,
    pub price: Option<String>,
    pub extracted_price: Option<f64>,
    pub rating: Option<f64>,
    pub reviews: Option<u32>,
    pub extensions: Option<Vec<String>>,
    pub thumbnail: Option<String>,
}

/// Local results
#[derive(Debug, Deserialize, Clone)]
pub struct LocalResults {
    pub more_locations_link: Option<String>,
    pub places: Option<Vec<LocalPlace>>,
}

/// Local place result
#[derive(Debug, Deserialize, Clone)]
pub struct LocalPlace {
    pub position: Option<u32>,
    pub title: String,
    pub place_id: String,
    pub data_id: String,
    pub data_cid: String,
    pub reviews_link: String,
    pub photos_link: String,
    pub gps_coordinates: Option<GpsCoordinates>,
    pub place_id_search: String,
    pub provider_id: String,
    pub rating: Option<f64>,
    pub reviews: Option<u32>,
    pub price: Option<String>,
    pub type_: Option<String>,
    pub types: Option<Vec<String>>,
    pub address: String,
    pub open_state: Option<String>,
    pub hours: Option<String>,
    pub operating_hours: Option<HashMap<String, String>>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub service_options: Option<HashMap<String, bool>>,
}

/// GPS coordinates
#[derive(Debug, Deserialize, Clone)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

/// News result
#[derive(Debug, Deserialize, Clone)]
pub struct NewsResult {
    pub position: Option<u32>,
    pub title: String,
    pub link: String,
    pub source: Option<String>,
    pub date: Option<String>,
    pub snippet: Option<String>,
    pub thumbnail: Option<String>,
}

/// Video result
#[derive(Debug, Deserialize, Clone)]
pub struct VideoResult {
    pub position: Option<u32>,
    pub title: String,
    pub link: String,
    pub displayed_link: Option<String>,
    pub thumbnail: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<String>,
    pub platform: Option<String>,
    pub date: Option<String>,
}

/// Inline image
#[derive(Debug, Deserialize, Clone)]
pub struct InlineImage {
    pub position: Option<u32>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub source: Option<String>,
    pub source_name: Option<String>,
    pub source_logo: Option<String>,
    pub thumbnail: Option<String>,
    pub original: Option<String>,
    pub is_product: Option<bool>,
}

/// Inline video result
#[derive(Debug, Deserialize, Clone)]
pub struct InlineVideo {
    pub position: Option<u32>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub thumbnail: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<String>,
    pub platform: Option<String>,
    pub date: Option<String>,
    pub key_moments: Option<Vec<KeyMoment>>,
}

/// Key moment in video
#[derive(Debug, Deserialize, Clone)]
pub struct KeyMoment {
    pub time: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
}

/// Short video result
#[derive(Debug, Deserialize, Clone)]
pub struct ShortVideo {
    pub position: Option<u32>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub thumbnail: Option<String>,
    pub channel: Option<String>,
    pub duration: Option<String>,
    pub platform: Option<String>,
}

/// Search information
#[derive(Debug, Deserialize, Clone)]
pub struct SearchInformation {
    pub organic_results_state: Option<String>,
    pub query_displayed: Option<String>,
    pub time_taken_displayed: Option<f64>,
    pub total_results: Option<u64>,
}

/// SerpAPI pagination
#[derive(Debug, Deserialize, Clone)]
pub struct SerpapiPagination {
    pub current: Option<u32>,
    pub next: Option<String>,
    pub next_link: Option<String>,
    pub other_pages: Option<HashMap<String, String>>,
}

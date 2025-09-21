use futures::stream::{self, Stream, StreamExt};
use std::pin::Pin;
use tracing::{debug, error};

use crate::{
    client::SerpClient,
    error::{SerpError, SerpResult},
    query::SearchQueryBuilder,
    response::SearchResults,
};

/// Configuration for streaming search results
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Number of results per page
    pub page_size: u32,
    /// Maximum number of pages to fetch
    pub max_pages: usize,
    /// Delay between requests to avoid rate limiting
    pub delay_between_requests: std::time::Duration,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            page_size: 10,
            max_pages: 10,
            delay_between_requests: std::time::Duration::from_millis(100),
        }
    }
}

impl StreamConfig {
    /// Create a new stream configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of results per page (1-100)
    pub fn page_size(mut self, size: u32) -> SerpResult<Self> {
        if size == 0 || size > 100 {
            return Err(SerpError::InvalidParameter(
                "page_size must be between 1 and 100".to_string()
            ));
        }
        self.page_size = size;
        Ok(self)
    }

    /// Set the maximum number of pages to fetch
    pub fn max_pages(mut self, pages: usize) -> Self {
        self.max_pages = pages;
        self
    }

    /// Set delay between requests
    pub fn delay(mut self, delay: std::time::Duration) -> Self {
        self.delay_between_requests = delay;
        self
    }
}

impl SerpClient {
    /// Stream paginated search results
    /// 
    /// This method returns a stream that yields `SearchResults` for each page.
    /// It automatically handles pagination by incrementing the start parameter.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use futures::StreamExt;
    /// use serp_sdk::{SerpClient, SearchQuery, StreamConfig};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SerpClient::builder().api_key("test").build()?;
    /// let mut stream = client.search_stream(
    ///     SearchQuery::new("rust programming"),
    ///     StreamConfig::default()
    /// );
    /// 
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(page) => println!("Got {} results", page.organic_results.as_ref().map_or(0, |r| r.len())),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_stream(
        &self,
        base_query: SearchQueryBuilder,
        config: StreamConfig,
    ) -> Pin<Box<dyn Stream<Item = SerpResult<SearchResults>> + Send + '_>> {
        let pages = stream::iter(0..config.max_pages)
            .then(move |page| {
                let query = base_query.clone()
                    .limit(config.page_size).unwrap_or_else(|_| base_query.clone())
                    .offset((page * config.page_size as usize) as u32);
                
                let delay = config.delay_between_requests;
                
                async move {
                    if page > 0 && !delay.is_zero() {
                        tokio::time::sleep(delay).await;
                    }
                    
                    debug!("Fetching page {} with offset {}", page + 1, page * config.page_size as usize);
                    self.search(query).await
                }
            });

        Box::pin(pages)
    }

    /// Stream individual organic results across multiple pages
    /// 
    /// This method flattens the paginated results into a stream of individual
    /// organic search results, making it easier to process results one by one.
    pub fn organic_results_stream(
        &self,
        base_query: SearchQueryBuilder,
        config: StreamConfig,
    ) -> Pin<Box<dyn Stream<Item = SerpResult<crate::response::OrganicResult>> + Send + '_>> {
        let search_stream = self.search_stream(base_query, config);
        
        let results_stream = search_stream.flat_map(|page_result| {
            match page_result {
                Ok(page) => {
                    let organic_results = page.organic_results.unwrap_or_default();
                    stream::iter(organic_results.into_iter().map(Ok)).left_stream()
                }
                Err(e) => {
                    error!("Failed to fetch page: {}", e);
                    stream::once(async move { Err(e) }).right_stream()
                }
            }
        });

        Box::pin(results_stream)
    }

    /// Stream results until a condition is met
    /// 
    /// This method continues fetching pages until the provided predicate returns true
    /// or an error occurs. Useful for searching until you find a specific result.
    pub fn search_until<F>(
        &self,
        base_query: SearchQueryBuilder,
        config: StreamConfig,
        mut predicate: F,
    ) -> Pin<Box<dyn Stream<Item = SerpResult<SearchResults>> + Send + '_>>
    where
        F: FnMut(&SearchResults) -> bool + Send + 'static,
    {
        let search_stream = self.search_stream(base_query, config);
        
        let conditional_stream = search_stream.take_while(move |result| {
            let should_continue = match result {
                Ok(page) => !predicate(page),
                Err(_) => false,
            };
            async move { should_continue }
        });

        Box::pin(conditional_stream)
    }

    /// Collect all results from multiple pages into a single vector
    /// 
    /// This method fetches all pages and combines the organic results into
    /// a single vector. Use with caution for large result sets.
    pub async fn search_all(
        &self,
        base_query: SearchQueryBuilder,
        config: StreamConfig,
    ) -> SerpResult<Vec<crate::response::OrganicResult>> {
        let mut all_results = Vec::new();
        let mut stream = self.organic_results_stream(base_query, config);

        while let Some(result) = stream.next().await {
            match result {
                Ok(organic_result) => all_results.push(organic_result),
                Err(e) => return Err(e),
            }
        }

        Ok(all_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_config() {
        let config = StreamConfig::new()
            .page_size(20).unwrap()
            .max_pages(5)
            .delay(std::time::Duration::from_millis(500));

        assert_eq!(config.page_size, 20);
        assert_eq!(config.max_pages, 5);
        assert_eq!(config.delay_between_requests, std::time::Duration::from_millis(500));
    }

    #[test]
    fn test_invalid_page_size() {
        let result = StreamConfig::new().page_size(0);
        assert!(result.is_err());

        let result = StreamConfig::new().page_size(101);
        assert!(result.is_err());
    }
}
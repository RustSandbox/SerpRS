use crate::error::{SerpError, SerpResult};
use serde::Serialize;

/// Fluent interface for building search queries
#[derive(Debug, Clone, Serialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    query: String,

    #[serde(rename = "hl", skip_serializing_if = "Option::is_none")]
    language: Option<String>,

    #[serde(rename = "gl", skip_serializing_if = "Option::is_none")]
    geolocation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    google_domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    safe: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tbm: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,

    #[serde(skip)]
    api_key: String,
}

impl SearchQuery {
    /// Create a new search query builder
    #[allow(clippy::new_ret_no_self)]
    pub fn new(query: impl Into<String>) -> SearchQueryBuilder {
        SearchQueryBuilder::new(query)
    }

    /// Get the query string
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Get the API key
    #[allow(dead_code)]
    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Convert to URL-encoded query string
    pub fn to_query_string(&self) -> SerpResult<String> {
        let mut params = serde_urlencoded::to_string(self)?;
        params.push_str(&format!("&api_key={}", self.api_key));
        Ok(params)
    }
}

/// Builder for constructing SearchQuery with fluent API
#[derive(Clone)]
pub struct SearchQueryBuilder {
    inner: SearchQuery,
}

impl SearchQueryBuilder {
    /// Create a new search query builder
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            inner: SearchQuery {
                query: query.into(),
                language: None,
                geolocation: None,
                google_domain: None,
                num: None,
                start: None,
                device: None,
                safe: None,
                tbm: None,
                location: None,
                api_key: String::new(),
            },
        }
    }

    /// Set the interface language (hl parameter)
    /// Common values: "en", "es", "fr", "de", "ja", "ko", "zh", etc.
    pub fn language(mut self, hl: impl Into<String>) -> Self {
        self.inner.language = Some(hl.into());
        self
    }

    /// Set the country for search results (gl parameter)
    /// Common values: "us", "uk", "ca", "au", "de", "fr", "jp", etc.
    pub fn country(mut self, gl: impl Into<String>) -> Self {
        self.inner.geolocation = Some(gl.into());
        self
    }

    /// Set the Google domain to use
    /// Examples: "google.com", "google.co.uk", "google.de", etc.
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.inner.google_domain = Some(domain.into());
        self
    }

    /// Set the number of results to return (1-100)
    pub fn limit(mut self, num: u32) -> SerpResult<Self> {
        if num == 0 || num > 100 {
            return Err(SerpError::InvalidParameter(
                "limit must be between 1 and 100".to_string(),
            ));
        }
        self.inner.num = Some(num);
        Ok(self)
    }

    /// Set the offset for pagination (start parameter)
    pub fn offset(mut self, start: u32) -> Self {
        self.inner.start = Some(start);
        self
    }

    /// Set the device type for search
    /// Common values: "desktop", "mobile", "tablet"
    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.inner.device = Some(device.into());
        self
    }

    /// Set SafeSearch setting
    /// Values: "active", "off"
    pub fn safe_search(mut self, safe: impl Into<String>) -> Self {
        self.inner.safe = Some(safe.into());
        self
    }

    /// Set search type (tbm parameter)
    /// Common values: "isch" (images), "vid" (videos), "nws" (news), "shop" (shopping)
    pub fn search_type(mut self, tbm: impl Into<String>) -> Self {
        self.inner.tbm = Some(tbm.into());
        self
    }

    /// Set location for local search
    /// Examples: "Austin, Texas", "New York, NY", "London, UK"
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.inner.location = Some(location.into());
        self
    }

    /// Build the search query (internal use)
    pub(crate) fn build(mut self, api_key: String) -> SearchQuery {
        self.inner.api_key = api_key;
        self.inner
    }
}

/// Specialized query builders for different search types
impl SearchQueryBuilder {
    /// Configure for image search
    pub fn images(self) -> Self {
        self.search_type("isch")
    }

    /// Configure for video search
    pub fn videos(self) -> Self {
        self.search_type("vid")
    }

    /// Configure for news search
    pub fn news(self) -> Self {
        self.search_type("nws")
    }

    /// Configure for shopping search
    pub fn shopping(self) -> Self {
        self.search_type("shop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = SearchQuery::new("rust programming")
            .language("en")
            .country("us")
            .limit(10)
            .unwrap()
            .build("test-key".to_string());

        assert_eq!(query.query(), "rust programming");
        assert_eq!(query.language.as_ref().unwrap(), "en");
        assert_eq!(query.geolocation.as_ref().unwrap(), "us");
        assert_eq!(query.num.unwrap(), 10);
    }

    #[test]
    fn test_limit_validation() {
        let result = SearchQuery::new("test").limit(101);
        assert!(result.is_err());

        let result = SearchQuery::new("test").limit(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_specialized_builders() {
        let query = SearchQuery::new("cats")
            .images()
            .build("test-key".to_string());

        assert_eq!(query.tbm.as_ref().unwrap(), "isch");
    }
}

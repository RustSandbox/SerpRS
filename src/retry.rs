use std::time::Duration;

/// Retry policy configuration for handling transient failures.
///
/// This struct configures how the SDK should retry failed requests, including
/// the number of retries, delay between attempts, and backoff strategy.
///
/// # Examples
///
/// ```rust
/// use serp_sdk::RetryPolicy;
/// use std::time::Duration;
///
/// // Default policy: 3 retries with exponential backoff
/// let default_policy = RetryPolicy::default();
///
/// // Custom policy: 5 retries with longer delays
/// let custom_policy = RetryPolicy::new(5)
///     .with_base_delay(Duration::from_millis(500))
///     .with_max_delay(Duration::from_secs(60))
///     .with_backoff_multiplier(1.5);
/// ```
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Base delay before the first retry attempt
    pub base_delay: Duration,
    /// Maximum delay between retry attempts
    pub max_delay: Duration,
    /// Multiplier for exponential backoff calculation
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new(max_retries: usize) -> Self {
        Self {
            max_retries,
            ..Default::default()
        }
    }

    /// Set the base delay between retries
    pub fn with_base_delay(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }

    /// Set the maximum delay between retries
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the backoff multiplier for exponential backoff
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Calculate the backoff duration for a given retry attempt
    pub fn backoff_duration(&self, attempt: usize) -> Duration {
        let delay =
            self.base_delay.as_millis() as f64 * self.backoff_multiplier.powi(attempt as i32);
        let delay_ms = delay.min(self.max_delay.as_millis() as f64) as u64;
        Duration::from_millis(delay_ms)
    }
}

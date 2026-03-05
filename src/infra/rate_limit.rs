//! Rate Limiting Module
//!
//! Token bucket rate limiter implementation for controlling request rates

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};

/// Token bucket rate limiter
#[derive(Debug)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    max_permits: usize,
    refill_interval: Duration,
    tokens_per_refill: usize,
    last_refill: std::sync::Mutex<Instant>,
}

impl RateLimiter {
    /// Create a new rate limiter
    /// - `rate_per_second`: Maximum requests per second
    /// - `burst_size`: Maximum burst capacity
    pub fn new(rate_per_second: u32, burst_size: u32) -> Self {
        let max_permits = burst_size as usize;
        let semaphore = Arc::new(Semaphore::new(max_permits));
        let refill_interval = Duration::from_millis(1000 / rate_per_second.max(1) as u64);

        Self {
            semaphore,
            max_permits,
            refill_interval,
            tokens_per_refill: 1,
            last_refill: std::sync::Mutex::new(Instant::now()),
        }
    }

    /// Wait for permission to proceed (acquire a token)
    pub async fn wait(&self) {
        // Refill tokens if needed
        self.refill_tokens();

        let _permit = self.semaphore.acquire().await.unwrap();
    }

    /// Refill tokens based on elapsed time
    fn refill_tokens(&self) {
        let mut last_refill = match self.last_refill.lock() {
            Ok(guard) => guard,
            Err(_) => return, // If poisoned, we skip refill this time
        };
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);

        if elapsed >= self.refill_interval {
            let refills = (elapsed.as_millis() / self.refill_interval.as_millis()) as usize;
            let tokens_to_add = refills * self.tokens_per_refill;

            // Add tokens up to burst capacity
            let current_permits = self.semaphore.available_permits();
            let add_permits = (self.max_permits - current_permits).min(tokens_to_add);

            if add_permits > 0 {
                self.semaphore.add_permits(add_permits);
            }

            *last_refill = now;
        }
    }

    /// Try to acquire a token without waiting
    pub fn try_wait(&self) -> bool {
        self.semaphore.try_acquire().is_ok()
    }

    /// Get current available capacity
    pub fn available(&self) -> usize {
        self.semaphore.available_permits()
    }
}

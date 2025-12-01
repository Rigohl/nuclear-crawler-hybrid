//! Módulo Rate Limiting - Control de velocidad de requests
//!
//! Implementa rate limiting con token bucket

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Rate limiter con token bucket
pub struct RateLimiter {
    max_tokens: u32,
    tokens: Arc<Mutex<u32>>,
    refill_rate: Duration,
    last_refill: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Crea un nuevo rate limiter
    pub fn new(requests_per_second: u32, burst_size: u32) -> Self {
        let max_tokens = burst_size.max(requests_per_second);
        let refill_rate = Duration::from_secs(1) / requests_per_second.max(1);

        Self {
            max_tokens,
            tokens: Arc::new(Mutex::new(max_tokens)),
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Espera hasta que haya un token disponible
    pub async fn wait(&self) {
        loop {
            let mut tokens = self.tokens.lock().await;
            let mut last_refill = self.last_refill.lock().await;

            // Refill tokens
            let now = Instant::now();
            let elapsed = now.duration_since(*last_refill);
            let tokens_to_add = (elapsed.as_secs_f64() / self.refill_rate.as_secs_f64()) as u32;

            if tokens_to_add > 0 {
                *tokens = (*tokens + tokens_to_add).min(self.max_tokens);
                *last_refill = now;
            }

            // Si hay tokens disponibles, usar uno
            if *tokens > 0 {
                *tokens -= 1;
                return;
            }

            // Esperar un poco antes de intentar de nuevo
            drop(tokens);
            drop(last_refill);
            sleep(self.refill_rate).await;
        }
    }

    /// Intenta obtener un token sin esperar
    pub async fn try_acquire(&self) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // Refill tokens
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() / self.refill_rate.as_secs_f64()) as u32;

        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.max_tokens);
            *last_refill = now;
        }

        // Intentar usar un token
        if *tokens > 0 {
            *tokens -= 1;
            true
        } else {
            false
        }
    }
}

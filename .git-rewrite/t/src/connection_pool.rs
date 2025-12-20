//! 🔥 Connection Pool Optimizer - Máximo Performance
//! Gestión de conexiones HTTP/HTTPS con pooling avanzado

use std::sync::Arc;

use dashmap::DashMap;
use backoff::{ExponentialBackoff, backoff::Backoff};
use std::time::Duration;

#[derive(Clone)]
pub struct ConnectionPool {
    connections: Arc<DashMap<String, Vec<Arc<reqwest::Client>>>>,
    config: Arc<PoolConfig>,
}

#[derive(Clone, Debug)]
pub struct PoolConfig {
    pub max_connections_per_host: usize,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_retries: u32,
    pub backoff_initial: Duration,
    pub backoff_max: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections_per_host: 50,
            connection_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(300),
            max_retries: 5,
            backoff_initial: Duration::from_millis(100),
            backoff_max: Duration::from_secs(5),
        }
    }
}

impl ConnectionPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            config: Arc::new(config),
        }
    }

    pub fn get_client(&self, host: &str) -> Arc<reqwest::Client> {
        let mut clients = self.connections
            .entry(host.to_string())
            .or_insert_with(Vec::new);

        if !clients.is_empty() {
            clients.pop().unwrap_or_else(|| Arc::new(Self::create_client(&self.config)))
        } else {
            Arc::new(Self::create_client(&self.config))
        }
    }

    pub fn return_client(&self, host: &str, client: Arc<reqwest::Client>) {
        let mut clients = self.connections
            .entry(host.to_string())
            .or_insert_with(Vec::new);

        if clients.len() < self.config.max_connections_per_host {
            clients.push(client);
        }
    }

    pub async fn with_retry<F, T>(&self, mut f: F) -> Result<T, String>
    where
        F: FnMut() -> futures::future::BoxFuture<'static, Result<T, String>>,
    {
        let mut backoff = ExponentialBackoff {
            current_interval: self.config.backoff_initial,
            initial_interval: self.config.backoff_initial,
            max_interval: self.config.backoff_max,
            max_elapsed_time: Some(Duration::from_secs(30)),
            multiplier: 2.0,
            randomization_factor: 0.1,
            start_time: std::time::Instant::now(),
            clock: backoff::SystemClock {},
        };

        for attempt in 0..self.config.max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt < self.config.max_retries - 1 {
                        if let Some(duration) = backoff.next_backoff() {
                            tokio::time::sleep(duration).await;
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err("Max retries exceeded".to_string())
    }

    fn create_client(config: &PoolConfig) -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(config.connection_timeout)
            .pool_max_idle_per_host(config.max_connections_per_host)
            .http2_prior_knowledge()
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    }

    pub fn stats(&self) -> PoolStats {
        let mut total_connections = 0;
        let mut hosts = 0;

        for entry in self.connections.iter() {
            hosts += 1;
            total_connections += entry.value().len();
        }

        PoolStats {
            total_connections,
            hosts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: usize,
    pub hosts: usize,
}

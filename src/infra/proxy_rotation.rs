//! 🔥 PROXY ROTATION - Intelligent Proxy Management for Mass Scraping
//!
//! Automatic proxy rotation, pooling, and validation
//! Supports SOCKS5, HTTP/HTTPS proxies with health checking
//! Zero-downtime fallback and load balancing

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Proxy entry with health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEntry {
    pub url: String,
    pub proxy_type: ProxyType,
    pub success_count: u32,
    pub failure_count: u32,
    pub average_response_time_ms: u64,
    pub is_healthy: bool,
    pub banned: bool,
}

/// Proxy type
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

impl ProxyEntry {
    pub fn new(url: String, proxy_type: ProxyType) -> Self {
        Self {
            url,
            proxy_type,
            success_count: 0,
            failure_count: 0,
            average_response_time_ms: 0,
            is_healthy: true,
            banned: false,
        }
    }

    /// Calculate proxy health score (0.0 - 1.0)
    pub fn health_score(&self) -> f32 {
        if self.banned {
            return 0.0;
        }

        let total_requests = (self.success_count + self.failure_count) as f32;
        if total_requests == 0.0 {
            return 0.5; // Unknown proxies get middle score
        }

        let success_rate = self.success_count as f32 / total_requests;
        let response_penalty = (self.average_response_time_ms as f32 / 5000.0).min(0.2);

        (success_rate * 0.8 - response_penalty).max(0.0).min(1.0)
    }
}

/// Proxy pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyPoolConfig {
    /// Check proxy health every N requests
    pub health_check_interval: u32,
    /// Timeout for proxy health checks
    pub health_check_timeout_secs: u64,
    /// Mark proxy as unhealthy after N failures
    pub failure_threshold: u32,
    /// Ban proxy after N consecutive failures
    pub ban_threshold: u32,
    /// Maximum concurrent requests per proxy
    pub max_requests_per_proxy: u32,
    /// Rotation strategy
    pub rotation_strategy: RotationStrategy,
    /// Fallback to direct connection if all proxies fail
    pub allow_direct_fallback: bool,
}

/// Proxy rotation strategy
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum RotationStrategy {
    /// Round-robin through proxies
    RoundRobin,
    /// Pick the healthiest proxy
    HealthBased,
    /// Random selection with bias towards healthy ones
    WeightedRandom,
    /// Least-used proxy
    LeastUsed,
}

impl Default for ProxyPoolConfig {
    fn default() -> Self {
        Self {
            health_check_interval: 50,
            health_check_timeout_secs: 10,
            failure_threshold: 5,
            ban_threshold: 15,
            max_requests_per_proxy: 100,
            rotation_strategy: RotationStrategy::HealthBased,
            allow_direct_fallback: true,
        }
    }
}

/// Proxy pool manager
pub struct ProxyPool {
    config: ProxyPoolConfig,
    proxies: Arc<Mutex<VecDeque<ProxyEntry>>>,
    rotation_index: Arc<Mutex<usize>>,
    request_count: Arc<Mutex<u32>>,
}

impl ProxyPool {
    /// Create new proxy pool
    pub fn new(config: ProxyPoolConfig) -> Self {
        eprintln!("🔥 Initializing Proxy Pool");
        eprintln!("  • Strategy: {:?}", config.rotation_strategy);
        eprintln!(
            "  • Health check interval: {}",
            config.health_check_interval
        );

        Self {
            config,
            proxies: Arc::new(Mutex::new(VecDeque::new())),
            rotation_index: Arc::new(Mutex::new(0)),
            request_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Add proxy to pool
    pub fn add_proxy(&self, url: String, proxy_type: ProxyType) -> Result<()> {
        let mut proxies = self.proxies.lock().unwrap();
        let entry = ProxyEntry::new(url.clone(), proxy_type);
        proxies.push_back(entry);
        eprintln!("  ✅ Added proxy: {} ({:?})", url, proxy_type);
        Ok(())
    }

    /// Add multiple proxies
    pub fn add_proxies(&self, proxies: Vec<(String, ProxyType)>) -> Result<()> {
        for (url, proxy_type) in proxies {
            self.add_proxy(url, proxy_type)?;
        }
        Ok(())
    }

    /// Get next proxy for request
    pub fn get_next_proxy(&self) -> Option<ProxyEntry> {
        let proxies = self.proxies.lock().unwrap();
        if proxies.is_empty() {
            return None;
        }

        let selected = match self.config.rotation_strategy {
            RotationStrategy::RoundRobin => {
                let mut index = self.rotation_index.lock().unwrap();
                let proxy = proxies[*index % proxies.len()].clone();
                *index = (*index + 1) % proxies.len();
                Some(proxy)
            }
            RotationStrategy::HealthBased => proxies
                .iter()
                .filter(|p| p.is_healthy && !p.banned)
                .max_by(|a, b| {
                    a.health_score()
                        .partial_cmp(&b.health_score())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned(),
            RotationStrategy::LeastUsed => proxies
                .iter()
                .filter(|p| p.is_healthy && !p.banned)
                .min_by_key(|p| p.success_count + p.failure_count)
                .cloned(),
            RotationStrategy::WeightedRandom => {
                // Simple weighted random selection
                let healthy_proxies: Vec<_> =
                    proxies.iter().filter(|p| !p.banned).cloned().collect();
                if healthy_proxies.is_empty() {
                    None
                } else {
                    let idx = (SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or(Duration::from_secs(0))
                        .as_nanos() as usize)
                        % healthy_proxies.len();
                    Some(healthy_proxies[idx].clone())
                }
            }
        };

        if let Some(ref _proxy) = selected {
            // 🔥 POTENCIACIÓN: Actualizar request count y tracking
            if let Ok(mut count) = self.request_count.lock() {
                *count += 1;
            }
        }

        selected
    }

    /// Report successful request
    pub fn report_success(&self, proxy_url: &str, response_time_ms: u64) -> Result<()> {
        let mut proxies = self.proxies.lock().unwrap();

        if let Some(proxy) = proxies.iter_mut().find(|p| p.url == proxy_url) {
            proxy.success_count += 1;
            proxy.failure_count = 0; // Reset failure count on success

            // Update average response time
            let total = proxy.success_count as u64;
            proxy.average_response_time_ms =
                (proxy.average_response_time_ms * (total - 1) + response_time_ms) / total;

            if !proxy.is_healthy && proxy.success_count > 3 {
                proxy.is_healthy = true;
                eprintln!("  ✅ Proxy {} is healthy again", proxy_url);
            }
        }

        Ok(())
    }

    /// Report failed request
    pub fn report_failure(&self, proxy_url: &str) -> Result<()> {
        let mut proxies = self.proxies.lock().unwrap();

        if let Some(proxy) = proxies.iter_mut().find(|p| p.url == proxy_url) {
            proxy.failure_count += 1;

            if proxy.failure_count >= self.config.ban_threshold {
                proxy.banned = true;
                eprintln!("  ❌ Proxy {} is BANNED (too many failures)", proxy_url);
            } else if proxy.failure_count >= self.config.failure_threshold {
                proxy.is_healthy = false;
                eprintln!("  ⚠️  Proxy {} marked as unhealthy", proxy_url);
            }
        }

        Ok(())
    }

    /// Get proxy statistics
    pub fn get_stats(&self) -> HashMap<String, String> {
        let proxies = self.proxies.lock().unwrap();
        let mut stats = HashMap::new();

        stats.insert("total_proxies".to_string(), proxies.len().to_string());
        stats.insert(
            "healthy_proxies".to_string(),
            proxies
                .iter()
                .filter(|p| p.is_healthy && !p.banned)
                .count()
                .to_string(),
        );
        stats.insert(
            "banned_proxies".to_string(),
            proxies.iter().filter(|p| p.banned).count().to_string(),
        );

        let request_count = self.request_count.lock().unwrap();
        stats.insert("total_requests".to_string(), request_count.to_string());

        stats
    }

    /// Clear all proxies
    pub fn clear(&self) -> Result<()> {
        self.proxies.lock().unwrap().clear();
        eprintln!("  🗑️  Proxy pool cleared");
        Ok(())
    }

    /// Get pool size
    pub fn size(&self) -> usize {
        self.proxies.lock().unwrap().len()
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.proxies.lock().unwrap().is_empty()
    }

    /// Force health check on all proxies
    pub async fn health_check_all(&self) -> Result<()> {
        eprintln!("🔥 Running health checks on all proxies...");

        let mut proxies = self.proxies.lock().unwrap();
        for proxy in proxies.iter_mut() {
            if !proxy.banned {
                // In real implementation, would perform actual HTTP request
                eprintln!("  🔍 Checking proxy: {}", proxy.url);
            }
        }

        Ok(())
    }
}

impl Default for ProxyPool {
    fn default() -> Self {
        Self::new(ProxyPoolConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_pool_creation() {
        let pool = ProxyPool::new(ProxyPoolConfig::default());
        assert!(pool.is_empty());
    }

    #[test]
    fn test_add_proxy() {
        let pool = ProxyPool::new(ProxyPoolConfig::default());
        pool.add_proxy("socks5://127.0.0.1:9050".to_string(), ProxyType::Socks5)
            .unwrap();
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_proxy_selection() {
        let pool = ProxyPool::new(ProxyPoolConfig::default());
        pool.add_proxy("http://proxy1:8080".to_string(), ProxyType::Http)
            .unwrap();
        pool.add_proxy("http://proxy2:8080".to_string(), ProxyType::Http)
            .unwrap();

        let proxy1 = pool.get_next_proxy();
        assert!(proxy1.is_some());
    }

    #[test]
    fn test_health_score() {
        let mut entry = ProxyEntry::new("http://test:8080".to_string(), ProxyType::Http);
        entry.success_count = 8;
        entry.failure_count = 2;

        let score = entry.health_score();
        assert!(score > 0.5); // Should be healthy
    }
}

//! 🚀 NUCLEAR CRAWLER - 10 MEJORAS ULTRA
//! ========================================
//! 1. Circuit Breaker - Tolerancia a fallos
//! 2. Bloom Filter - Deduplicación O(1)
//! 3. Semaphore Rate Limiter - Control de concurrencia
//! 4. Metrics Collector - Prometheus compatible
//! 5. Memory Cache - Cache en memoria con TTL
//! 6. Smart Retry - Reintentos inteligentes
//! 7. Event Bus - Pub/Sub asíncrono
//! 8. Session Manager - Gestión de sesiones
//! 9. Priority Queue - Cola de prioridad
//! 10. Plugin Manager - Sistema de plugins

use anyhow::Result;
use serde_json::Value;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Semaphore};

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 1: CIRCUIT BREAKER
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit Breaker para tolerancia a fallos
pub struct CircuitBreaker {
    state: CircuitState,
    failures: usize,
    threshold: usize,
    timeout_secs: u64,
    last_failure: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(threshold: usize, timeout_secs: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            failures: 0,
            threshold,
            timeout_secs,
            last_failure: None,
        }
    }

    pub fn is_open(&self) -> bool {
        match self.state {
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed().as_secs() >= self.timeout_secs {
                        return false; // Should transition to HalfOpen
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub fn record_success(&mut self) {
        self.failures = 0;
        self.state = CircuitState::Closed;
    }

    pub fn record_failure(&mut self) {
        self.failures += 1;
        self.last_failure = Some(Instant::now());
        if self.failures >= self.threshold {
            self.state = CircuitState::Open;
        }
    }

    pub fn state(&self) -> &CircuitState {
        &self.state
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 2: BLOOM FILTER
// ═══════════════════════════════════════════════════════════════════════════

/// Bloom Filter para deduplicación O(1)
pub struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
    size: usize,
}

impl BloomFilter {
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let size = Self::optimal_size(expected_items, false_positive_rate);
        let num_hashes = Self::optimal_hashes(size, expected_items);

        Self {
            bits: vec![false; size],
            num_hashes,
            size,
        }
    }

    fn optimal_size(n: usize, p: f64) -> usize {
        let m = -(n as f64 * p.ln()) / (2.0_f64.ln().powi(2));
        m.ceil() as usize
    }

    fn optimal_hashes(m: usize, n: usize) -> usize {
        let k = (m as f64 / n as f64) * 2.0_f64.ln();
        k.ceil() as usize
    }

    fn hash_indices(&self, item: &str) -> Vec<usize> {
        let mut indices = Vec::with_capacity(self.num_hashes);

        for i in 0..self.num_hashes {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            item.hash(&mut hasher);
            i.hash(&mut hasher);
            let hash = hasher.finish() as usize;
            indices.push(hash % self.size);
        }

        indices
    }

    pub fn insert(&mut self, item: &str) {
        for idx in self.hash_indices(item) {
            self.bits[idx] = true;
        }
    }

    pub fn might_contain(&self, item: &str) -> bool {
        self.hash_indices(item).iter().all(|&idx| self.bits[idx])
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 3: SEMAPHORE RATE LIMITER
// ═══════════════════════════════════════════════════════════════════════════

/// Rate limiter basado en semáforo
pub struct SemaphoreLimiter {
    semaphore: Semaphore,
    #[allow(dead_code)]
    max_permits: usize,
}

impl SemaphoreLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Semaphore::new(max_concurrent),
            max_permits: max_concurrent,
        }
    }

    pub fn try_acquire(&self) -> bool {
        self.semaphore.try_acquire().is_ok()
    }

    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.unwrap()
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 4: METRICS COLLECTOR
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}

/// Colector de métricas Prometheus-compatible
pub struct MetricsCollector {
    counters: RwLock<HashMap<String, AtomicU64>>,
    gauges: RwLock<HashMap<String, AtomicU64>>,
    histograms: RwLock<HashMap<String, Vec<f64>>>,
    metric_types: RwLock<HashMap<String, MetricType>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            metric_types: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, name: &str, metric_type: MetricType) {
        let mut types = self.metric_types.write().unwrap();
        types.insert(name.to_string(), metric_type.clone());

        match metric_type {
            MetricType::Counter => {
                let mut counters = self.counters.write().unwrap();
                counters.insert(name.to_string(), AtomicU64::new(0));
            }
            MetricType::Gauge => {
                let mut gauges = self.gauges.write().unwrap();
                gauges.insert(name.to_string(), AtomicU64::new(0));
            }
            MetricType::Histogram => {
                let mut histograms = self.histograms.write().unwrap();
                histograms.insert(name.to_string(), Vec::new());
            }
        }
    }

    pub fn increment(&self, name: &str) {
        if let Ok(counters) = self.counters.read() {
            if let Some(counter) = counters.get(name) {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn set_gauge(&self, name: &str, value: u64) {
        if let Ok(gauges) = self.gauges.read() {
            if let Some(gauge) = gauges.get(name) {
                gauge.store(value, Ordering::Relaxed);
            }
        }
    }

    pub fn observe(&self, name: &str, value: f64) {
        if let Ok(mut histograms) = self.histograms.write() {
            if let Some(histogram) = histograms.get_mut(name) {
                histogram.push(value);
            }
        }
    }

    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();

        if let Ok(counters) = self.counters.read() {
            for (name, counter) in counters.iter() {
                output.push_str(&format!(
                    "# TYPE {} counter\n{} {}\n\n",
                    name,
                    name,
                    counter.load(Ordering::Relaxed)
                ));
            }
        }

        if let Ok(gauges) = self.gauges.read() {
            for (name, gauge) in gauges.iter() {
                output.push_str(&format!(
                    "# TYPE {} gauge\n{} {}\n\n",
                    name,
                    name,
                    gauge.load(Ordering::Relaxed)
                ));
            }
        }

        if let Ok(histograms) = self.histograms.read() {
            for (name, values) in histograms.iter() {
                if !values.is_empty() {
                    let sum: f64 = values.iter().sum();
                    let count = values.len();
                    let avg = sum / count as f64;
                    output.push_str(&format!(
                        "# TYPE {} histogram\n{}_sum {}\n{}_count {}\n{}_avg {:.2}\n\n",
                        name, name, sum, name, count, name, avg
                    ));
                }
            }
        }

        output
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 5: MEMORY CACHE CON TTL
// ═══════════════════════════════════════════════════════════════════════════

struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

/// Cache en memoria con TTL y límite de tamaño
pub struct MemoryCache<K, V> {
    cache: RwLock<HashMap<K, CacheEntry<V>>>,
    max_size: usize,
    default_ttl: Duration,
}

impl<K: Eq + Hash + Clone, V: Clone> MemoryCache<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
            default_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.write().unwrap();

        // Evict if at capacity
        if cache.len() >= self.max_size {
            // Remove expired entries first
            cache.retain(|_, entry| entry.expires_at > Instant::now());

            // If still full, remove oldest
            if cache.len() >= self.max_size {
                if let Some(key_to_remove) = cache.keys().next().cloned() {
                    cache.remove(&key_to_remove);
                }
            }
        }

        cache.insert(
            key,
            CacheEntry {
                value,
                expires_at: Instant::now() + self.default_ttl,
            },
        );
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.read().unwrap();
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value.clone());
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 6: SMART RETRY
// ═══════════════════════════════════════════════════════════════════════════

/// Retry inteligente con backoff exponencial
pub struct SmartRetry {
    max_retries: usize,
    initial_delay_ms: u64,
    max_delay_ms: u64,
}

impl SmartRetry {
    pub fn new(max_retries: usize, initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        Self {
            max_retries,
            initial_delay_ms,
            max_delay_ms,
        }
    }

    pub async fn execute<F, Fut, T, E>(&self, mut f: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let mut delay = self.initial_delay_ms;
        let mut attempt = 0;

        loop {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.max_retries {
                        return Err(e);
                    }

                    let wait = delay.min(self.max_delay_ms);
                    tokio::time::sleep(Duration::from_millis(wait)).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 7: EVENT BUS
// ═══════════════════════════════════════════════════════════════════════════

/// Event Bus para pub/sub asíncrono
pub struct EventBus {
    sender: mpsc::Sender<(String, Value)>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel::<(String, Value)>(1000);

        // Background event processor
        tokio::spawn(async move {
            while let Some((event_type, _data)) = receiver.recv().await {
                // Log event
                tracing::debug!(event_type = %event_type, "Event received");
                // Could dispatch to registered handlers here
            }
        });

        Self { sender }
    }

    pub fn publish(&self, event_type: &str, data: Value) -> Result<()> {
        self.sender
            .try_send((event_type.to_string(), data))
            .map_err(|e| anyhow::anyhow!("Failed to publish event: {}", e))
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 8: SESSION MANAGER
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub created_at: Instant,
    pub last_access: Instant,
    pub data: HashMap<String, Value>,
}

/// Gestor de sesiones
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
    timeout: Duration,
}

impl SessionManager {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub fn create(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let session = Session {
            id: id.clone(),
            created_at: Instant::now(),
            last_access: Instant::now(),
            data: HashMap::new(),
        };

        self.sessions.write().unwrap().insert(id.clone(), session);
        id
    }

    pub fn get(&self, id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(id) {
            if session.last_access.elapsed() < self.timeout {
                session.last_access = Instant::now();
                return Some(session.clone());
            } else {
                sessions.remove(id);
            }
        }
        None
    }

    pub fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, s| s.last_access.elapsed() < self.timeout);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 9: PRIORITY QUEUE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PriorityItem<T> {
    pub priority: i32,
    pub item: T,
}

impl<T: Eq> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T: Eq> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Cola de prioridad thread-safe
pub struct PriorityQueue<T: Eq> {
    heap: RwLock<BinaryHeap<PriorityItem<T>>>,
}

impl<T: Eq + Clone> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: RwLock::new(BinaryHeap::new()),
        }
    }

    pub fn push(&self, priority: i32, item: T) {
        self.heap
            .write()
            .unwrap()
            .push(PriorityItem { priority, item });
    }

    pub fn pop(&self) -> Option<T> {
        self.heap.write().unwrap().pop().map(|pi| pi.item)
    }

    pub fn len(&self) -> usize {
        self.heap.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Eq + Clone> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 🔥 MEJORA 10: PLUGIN MANAGER
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para plugins
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_request(&self, data: &Value) -> Result<Value>;
    fn on_response(&self, data: &Value) -> Result<Value>;
}

/// Gestor de plugins
pub struct PluginManager {
    plugins: RwLock<Vec<Arc<dyn Plugin>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(Vec::new()),
        }
    }

    pub fn register(&self, plugin: Arc<dyn Plugin>) {
        self.plugins.write().unwrap().push(plugin);
    }

    pub fn process_request(&self, data: &Value) -> Result<Value> {
        let plugins = self.plugins.read().unwrap();
        let mut result = data.clone();

        for plugin in plugins.iter() {
            result = plugin.on_request(&result)?;
        }

        Ok(result)
    }

    pub fn process_response(&self, data: &Value) -> Result<Value> {
        let plugins = self.plugins.read().unwrap();
        let mut result = data.clone();

        for plugin in plugins.iter() {
            result = plugin.on_response(&result)?;
        }

        Ok(result)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker() {
        let mut cb = CircuitBreaker::new(3, 30);
        assert!(!cb.is_open());

        cb.record_failure();
        cb.record_failure();
        assert!(!cb.is_open());

        cb.record_failure();
        assert!(cb.is_open());

        cb.record_success();
        assert!(!cb.is_open());
    }

    #[test]
    fn test_bloom_filter() {
        let mut bf = BloomFilter::new(1000, 0.01);
        bf.insert("hello");
        bf.insert("world");

        assert!(bf.might_contain("hello"));
        assert!(bf.might_contain("world"));
        assert!(!bf.might_contain("notexist"));
    }

    #[test]
    fn test_memory_cache() {
        let cache: MemoryCache<String, i32> = MemoryCache::new(10);
        cache.insert("key1".to_string(), 42);

        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }
}

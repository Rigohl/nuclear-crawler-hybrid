//! Módulo Cache - Sistema de caché avanzado para Nuclear Scraper

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Entrada de caché con TTL
pub struct CacheEntry {
    pub data: String,
    pub expires_at: Instant,
}

/// 🔥 NUCLEAR: Respuesta cacheada con todos los datos de crawl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub url: String,
    pub status_code: u16,
    pub content_type: String,
    pub content_length: usize,
    #[serde(with = "duration_serde")]
    pub response_time: Duration,
    pub links_found: Vec<String>,
    pub html: String,
    pub cached_at: chrono::DateTime<chrono::Utc>,
}

/// Serializador para Duration
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_millis().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

/// Entrada de caché para CachedResponse
struct CachedResponseEntry {
    pub data: CachedResponse,
    pub expires_at: Instant,
}

/// Sistema de caché avanzado
pub struct Cache {
    storage: Arc<DashMap<String, CacheEntry>>,
    response_storage: Arc<DashMap<String, CachedResponseEntry>>,
    max_size: usize,
    default_ttl: Duration,
}

impl Cache {
    /// Crea un nuevo caché
    pub fn new(max_size: usize) -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
            response_storage: Arc::new(DashMap::new()),
            max_size,
            default_ttl: Duration::from_secs(3600),
        }
    }

    /// 🔥 NUCLEAR: Obtiene una respuesta cacheada (async para compatibilidad)
    pub async fn get(&self, key: &str) -> Option<CachedResponse> {
        if let Some(entry) = self.response_storage.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            } else {
                drop(entry);
                self.response_storage.remove(key);
            }
        }
        None
    }

    /// 🔥 NUCLEAR: Guarda una respuesta en caché (async para compatibilidad)
    pub async fn set(&self, key: &str, value: CachedResponse) {
        // Limpiar si está lleno
        if self.response_storage.len() >= self.max_size {
            if let Some(first) = self.response_storage.iter().next() {
                let key_to_remove = first.key().clone();
                drop(first);
                self.response_storage.remove(&key_to_remove);
            }
        }

        let entry = CachedResponseEntry {
            data: value,
            expires_at: Instant::now() + self.default_ttl,
        };

        self.response_storage.insert(key.to_string(), entry);
    }

    /// Obtiene un valor simple del caché
    pub fn get_simple(&self, key: &str) -> Option<String> {
        if let Some(entry) = self.storage.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            } else {
                drop(entry);
                self.storage.remove(key);
            }
        }
        None
    }

    /// Guarda un valor simple en el caché
    pub fn set_simple(&self, key: &str, value: String) {
        if self.storage.len() >= self.max_size {
            // Eliminar el primero (simple LRU)
            if let Some(first) = self.storage.iter().next() {
                let key_to_remove = first.key().clone();
                drop(first);
                self.storage.remove(&key_to_remove);
            }
        }

        let entry = CacheEntry {
            data: value,
            expires_at: Instant::now() + self.default_ttl,
        };

        self.storage.insert(key.to_string(), entry);
    }

    /// Limpia todo el caché
    pub fn clear(&self) {
        self.storage.clear();
        self.response_storage.clear();
    }

    /// Obtiene el tamaño actual
    pub fn len(&self) -> usize {
        self.storage.len() + self.response_storage.len()
    }

    /// Verifica si está vacío
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty() && self.response_storage.is_empty()
    }
}

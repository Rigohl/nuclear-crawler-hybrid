//! Módulo Cache - Sistema de caché de alto rendimiento
//!
//! Caché en memoria con TTL y límites de tamaño

use crate::crawler::CrawledUrl;
use dashmap::DashMap;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Entrada de caché con TTL
struct CacheEntry {
    data: CrawledUrl,
    expires_at: Instant,
}

/// Sistema de caché
pub struct Cache {
    storage: Arc<DashMap<String, CacheEntry>>,
    max_size: usize,
    default_ttl: Duration,
}

impl Cache {
    /// Crea un nuevo caché
    pub fn new(max_size: usize) -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
            max_size,
            default_ttl: Duration::from_secs(3600), // 1 hora por defecto
        }
    }

    /// Obtiene un valor del caché
    pub async fn get(&self, key: &str) -> Option<CrawledUrl> {
        if let Some(entry) = self.storage.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            } else {
                // Expiró, eliminar
                self.storage.remove(key);
            }
        }
        None
    }

    /// Guarda un valor en el caché
    pub async fn set(&self, key: &str, value: &CrawledUrl) {
        // Verificar tamaño máximo
        if self.storage.len() >= self.max_size {
            // Eliminar entradas expiradas primero
            self.cleanup_expired().await;

            // Si aún está lleno, eliminar la más antigua
            if self.storage.len() >= self.max_size {
                if let Some(oldest) = self.storage.iter().next() {
                    self.storage.remove(oldest.key());
                }
            }
        }

        let entry = CacheEntry {
            data: value.clone(),
            expires_at: Instant::now() + self.default_ttl,
        };

        self.storage.insert(key.to_string(), entry);
    }

    /// Limpia entradas expiradas
    async fn cleanup_expired(&self) {
        let now = Instant::now();
        self.storage.retain(|_, entry| entry.expires_at > now);
    }

    /// Limpia todo el caché
    pub async fn clear(&self) {
        self.storage.clear();
    }

    /// Obtiene el tamaño actual del caché
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Verifica si el caché está vacío
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

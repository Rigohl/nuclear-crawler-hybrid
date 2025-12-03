//! Módulo JAX Acceleration - Procesamiento Acelerado
//!
//! Usa técnicas similares a JAX para procesamiento paralelo masivo
//! y aceleración de operaciones de scraping

use rayon::prelude::*;
#[allow(unused_imports)]
use std::sync::Arc;
use std::time::Instant;

/// Procesador acelerado tipo JAX
#[allow(dead_code)]
pub struct JaxAccelerator {
    /// Número de workers paralelos
    num_workers: usize,
}

impl JaxAccelerator {
    /// Crea un nuevo acelerador (usa máximo de CPUs)
    pub fn new() -> Self {
        Self {
            num_workers: num_cpus::get() * 2,
        }
    }
    
    /// Crea con número específico de workers
    pub fn new_with_workers(num_workers: Option<usize>) -> Self {
        let num_workers = num_workers.unwrap_or_else(|| {
            num_cpus::get() * 2 // Doble de cores para máximo paralelismo
        });

        Self { num_workers }
    }
    
    /// Verifica si JAX acceleration está disponible
    pub fn is_available(&self) -> bool {
        self.num_workers > 0
    }

    /// Procesa URLs en paralelo masivo (estilo JAX vectorization)
    pub fn vectorized_process<F, T, R>(&self, items: Vec<T>, processor: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(T) -> R + Send + Sync,
    {
        items
            .into_par_iter()
            .with_min_len(1)
            .map(processor)
            .collect()
    }

    /// Procesa en batches paralelos (estilo JAX batching)
    pub fn batched_process<F, T, R>(&self, items: Vec<T>, batch_size: usize, processor: F) -> Vec<R>
    where
        T: Send + Sync + Clone,
        R: Send,
        F: Fn(Vec<T>) -> Vec<R> + Send + Sync,
    {
        items
            .chunks(batch_size)
            .collect::<Vec<_>>()
            .into_par_iter()
            .flat_map(|chunk| processor(chunk.to_vec()))
            .collect()
    }

    /// Procesa con pipeline paralelo
    pub fn pipeline_process<F1, F2, T, I, O>(&self, items: Vec<T>, stage1: F1, stage2: F2) -> Vec<O>
    where
        T: Send + Sync,
        I: Send + Sync,
        O: Send,
        F1: Fn(T) -> I + Send + Sync,
        F2: Fn(I) -> O + Send + Sync,
    {
        items
            .into_par_iter()
            .map(|item| {
                let intermediate = stage1(item);
                stage2(intermediate)
            })
            .collect()
    }

    /// Reduce paralelo (estilo JAX reduce)
    pub fn parallel_reduce<F, T>(&self, items: Vec<T>, reducer: F, identity: T) -> T
    where
        T: Send + Sync + Clone,
        F: Fn(T, T) -> T + Send + Sync,
    {
        items.into_par_iter().reduce(|| identity.clone(), &reducer)
    }

    /// Map-reduce paralelo
    pub fn map_reduce<F1, F2, T, O>(&self, items: Vec<T>, mapper: F1, reducer: F2, identity: O) -> O
    where
        T: Send + Sync,
        O: Send + Sync + Clone,
        F1: Fn(T) -> O + Send + Sync,
        F2: Fn(O, O) -> O + Send + Sync,
    {
        items
            .into_par_iter()
            .map(mapper)
            .reduce(|| identity.clone(), reducer)
    }
}

/// Benchmark de operaciones
pub fn benchmark_operation<F>(name: &str, operation: F) -> std::time::Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    operation();
    let elapsed = start.elapsed();
    println!("⏱️  {}: {:?}", name, elapsed);
    elapsed
}

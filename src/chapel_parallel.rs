//! 🚀 Chapel AI Parallel Wrapper
//! Interfaz Rust para invocar Chapel AI en FULL PARALLELISM

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

/// Contexto de tarea paralela para Chapel
#[derive(Clone, Debug)]
pub struct ChapelParallelTask {
    pub id: usize,
    pub operation: String,
    pub data: Vec<u8>,
}

/// Resultado de computación en Chapel
#[derive(Clone, Debug)]
pub struct ChapelComputeResult {
    pub task_id: usize,
    pub thread_id: usize,
    pub status: String,
    pub output: Vec<u8>,
}

/// 🔥 Ejecutor paralelo para Chapel AI
pub struct ChapelParallelExecutor {
    max_parallelism: usize,
    task_counter: Arc<AtomicUsize>,
}

impl ChapelParallelExecutor {
    /// Crear nuevo ejecutor
    pub fn new(max_parallelism: Option<usize>) -> Self {
        let cores = num_cpus::get();
        let parallelism = max_parallelism.unwrap_or(cores);
        
        Self {
            max_parallelism: parallelism,
            task_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    /// 🚀 Ejecutar tareas en FULL PARALLELISM
    pub fn execute_parallel(&self, tasks: Vec<ChapelParallelTask>) -> Vec<ChapelComputeResult> {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.max_parallelism)
            .build()
            .unwrap();
        
        thread_pool.install(|| {
            tasks
                .into_par_iter()
                .map(|task| self.execute_task(&task))
                .collect()
        })
    }
    
    /// Ejecutar tarea individual
    fn execute_task(&self, task: &ChapelParallelTask) -> ChapelComputeResult {
        let task_num = self.task_counter.fetch_add(1, Ordering::SeqCst);
        let thread_id = rayon::current_thread_index().unwrap_or(0);
        
        // 🔥 AQUÍ IRÍA LLAMADA FFI A CHAPEL EN PARALELO
        // unsafe { chapel_ai_compute(task.data.as_ptr(), task.data.len()); }
        
        ChapelComputeResult {
            task_id: task.id,
            thread_id,
            status: "success".to_string(),
            output: format!("CHAPEL[{}]:{}", task_num, task.operation).into_bytes(),
        }
    }
    
    /// Información del ejecutor
    pub fn info(&self) -> String {
        format!("ChapelParallelExecutor {{ parallelism: {}, tasks: {} }}",
                self.max_parallelism,
                self.task_counter.load(Ordering::SeqCst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_execution() {
        let executor = ChapelParallelExecutor::new(Some(4));
        
        let tasks = vec![
            ChapelParallelTask {
                id: 1,
                operation: "analyze".to_string(),
                data: b"test1".to_vec(),
            },
            ChapelParallelTask {
                id: 2,
                operation: "train".to_string(),
                data: b"test2".to_vec(),
            },
        ];
        
        let results = executor.execute_parallel(tasks);
        assert_eq!(results.len(), 2);
    }
}

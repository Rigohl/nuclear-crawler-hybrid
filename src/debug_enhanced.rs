/// Enhanced Debug & Error Handling Module
/// Proporciona logging estructurado, contexto de errores y timing instrumentado
use std::time::{Instant, Duration};
use chrono::Local;

use std::fmt;

/// Contexto mejorado para errores
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub module: String,
    pub function: String,
    pub line: u32,
    pub timestamp: String,
    pub stack_trace: Option<String>,
    pub duration_ms: f64,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ERROR] {}::{} @ line {} | {} | duration: {:.2}ms",
            self.module, self.function, self.line, self.timestamp, self.duration_ms
        )
    }
}

/// Logger estructurado para operaciones
pub struct OperationLogger {
    name: String,
    start_time: Instant,
}

impl OperationLogger {
    /// Crear nuevo logger para operación
    pub fn new(name: &str) -> Self {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        tracing::info!("🚀 [START] {} @ {}", name, timestamp);
        
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
        }
    }

    /// Log de checkpoint intermedio
    pub fn checkpoint(&self, msg: &str) {
        let elapsed = self.start_time.elapsed().as_millis();
        tracing::debug!("  ✓ [{}ms] {}", elapsed, msg);
    }

    /// Log de warning
    pub fn warn(&self, msg: &str) {
        let elapsed = self.start_time.elapsed().as_millis();
        tracing::warn!("  ⚠ [{}ms] {}", elapsed, msg);
    }

    /// Log de error
    pub fn error(&self, msg: &str) {
        let elapsed = self.start_time.elapsed().as_millis();
        tracing::error!("  ✗ [{}ms] {}", elapsed, msg);
    }

    /// Finalizar logger y retornar duración
    pub fn finish(self) -> Duration {
        let duration = self.start_time.elapsed();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        tracing::info!(
            "✅ [END] {} completed in {:.2}ms @ {}",
            self.name,
            duration.as_millis(),
            timestamp
        );
        duration
    }

    /// Finalizar con estado y retornar duración
    pub fn finish_with_status(self, success: bool, message: &str) -> Duration {
        let duration = self.start_time.elapsed();
        let status = if success { "✅ SUCCESS" } else { "❌ FAILED" };
        tracing::info!(
            "{} [{}] {} in {:.2}ms | {}",
            status,
            self.name,
            message,
            duration.as_millis(),
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        );
        duration
    }
}

/// Métricas de ejecución
#[derive(Debug, Clone)]
pub struct ExecutionMetrics {
    pub operation: String,
    pub start_time: String,
    pub duration_ms: f64,
    pub items_processed: usize,
    pub throughput: f64,
    pub memory_estimate_kb: f64,
}

impl ExecutionMetrics {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            start_time: Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            duration_ms: 0.0,
            items_processed: 0,
            throughput: 0.0,
            memory_estimate_kb: 0.0,
        }
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration_ms = duration.as_secs_f64() * 1000.0;
    }

    pub fn set_throughput(&mut self, items: usize) {
        self.items_processed = items;
        self.throughput = items as f64 / (self.duration_ms / 1000.0);
    }

    pub fn pretty_print(&self) {
        println!("\n╔════════════════════════════════════════╗");
        println!("║ 📊 EXECUTION METRICS: {:<19} ║", self.operation);
        println!("╠════════════════════════════════════════╣");
        println!("║ Started: {:<31} ║", self.start_time);
        println!("║ Duration: {:.2} ms{:<22} ║", self.duration_ms, "");
        println!("║ Items Processed: {:<21} ║", self.items_processed);
        println!("║ Throughput: {:.2} items/sec{:<13} ║", self.throughput, "");
        println!("╚════════════════════════════════════════╝\n");
    }
}

/// Setup de tracing para logging global
pub fn setup_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_line_number(true),
        )
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    tracing::info!("🔧 Tracing initialized");
}

/// Macro para instrumentar funciones automáticamente
#[macro_export]
macro_rules! debug_operation {
    ($name:expr, $body:expr) => {{
        let logger = $crate::debug_enhanced::OperationLogger::new($name);
        let result = $body;
        logger.finish();
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let ctx = ErrorContext {
            module: "test_module".to_string(),
            function: "test_function".to_string(),
            line: 42,
            timestamp: "2025-12-06 12:00:00".to_string(),
            stack_trace: None,
            duration_ms: 123.45,
        };
        
        assert_eq!(ctx.module, "test_module");
        assert!(ctx.to_string().contains("test_module"));
        assert!(ctx.to_string().contains("test_function"));
    }

    #[test]
    fn test_metrics() {
        let mut metrics = ExecutionMetrics::new("test_op");
        metrics.set_duration(Duration::from_millis(500));
        metrics.set_throughput(100);
        
        assert_eq!(metrics.operation, "test_op");
        assert_eq!(metrics.items_processed, 100);
        assert!(metrics.throughput > 190.0 && metrics.throughput < 210.0); // ~200 items/sec
    }
}

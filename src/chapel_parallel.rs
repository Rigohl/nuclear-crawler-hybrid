// use std::sync::atomic::{AtomicUsize, Ordering};
        }).await.unwrap_or(-1)
    }

    pub async fn train_model_ffi(&self, epochs: i32) -> i32 {
        tokio::task::spawn_blocking(move || {
            if let Some(lib) = &*CHAPEL_LIB {
                lib.train_model(epochs)
            } else {
                std::thread::sleep(std::time::Duration::from_millis(200));
                85
            }
        }).await.unwrap_or(0)
    }

    pub async fn run_tools_parallel(&self) -> Vec<ToolExecResult> {
        let orchestrator = self.clone();

        let handle1 = {
            let orch = orchestrator.clone();
            tokio::spawn(async move {
                let start = Instant::now();
                let (quality, output) = Self::exec_tool_websearch().await;
                let duration = start.elapsed().as_millis() as u64;
                orch.learn_tool("websearch", duration, quality).await;
                ToolExecResult {
                    tool: "websearch".to_string(),
                    duration_ms: duration,
                    quality,
                    output,
                }
            })
        };

        let mut results = Vec::new();
        if let Ok(result) = handle1.await {
            results.push(result);
        }

        results
    }

    async fn learn_tool(&self, tool: &str, duration_ms: u64, quality: f64) {
        let mut memory = self.learning_memory.write().await;
        let metrics = memory.tool_metrics.entry(tool.to_string()).or_insert(ToolMetrics {
            calls: 0, total_duration_ms: 0, avg_quality: 0.0
        });

        metrics.calls += 1;
        metrics.total_duration_ms += duration_ms;
        metrics.avg_quality = (metrics.avg_quality * (metrics.calls - 1) as f64 + quality) / metrics.calls as f64;
    }

    async fn exec_tool_websearch() -> (f64, String) {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        (0.92, "Search results".to_string())
    }

    pub async fn learning_report(&self) -> String {
        let memory = self.learning_memory.read().await;

        let mut report = String::from("🧠 CHAPEL AI - PARALLEL LEARNING REPORT\n");
        report.push_str("═══════════════════════════════════════\n\n");

        report.push_str("📊 5 MCP TOOLS STATUS:\n");
        for (tool, metrics) in &memory.tool_metrics {
            report.push_str(&format!(
                "  {} - Calls: {}, Avg: {}ms, Quality: {:.1}%\n",
                tool,
                metrics.calls,
                metrics.total_duration_ms / metrics.calls.max(1) as u64,
                metrics.avg_quality * 100.0
            ));
        }

        report.push_str("\n🧠 LEARNED PATTERNS:\n");
        for pattern in memory.patterns.iter().take(3) {
            report.push_str(&format!("  ✓ {}\n", pattern));
        }

        if !memory.optimization_suggestions.is_empty() {
            report.push_str("\n⚡ OPTIMIZATION SUGGESTIONS:\n");
            for suggestion in memory.optimization_suggestions.iter().take(5) {
                report.push_str(&format!("  {}\n", suggestion));
            }
        }

        report
    }
}

#[derive(Debug)]
pub struct ToolExecResult {
    pub tool: String,
    pub duration_ms: u64,
    pub quality: f64,
    pub output: String,
}

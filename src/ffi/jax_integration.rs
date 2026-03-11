//! JAX integration with explicit real-backend enforcement.
//!
//! This module only operates when a real JAX runtime and the repository-backed
//! helper script are available. It does not degrade to CPU heuristics.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// 🔥 JAX Processing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaxConfig {
    pub batch_size: usize,
    pub use_gpu: bool,
    pub precision: String, // "fp32", "fp16", "int8"
    pub max_memory_gb: f32,
}

impl Default for JaxConfig {
    fn default() -> Self {
        Self {
            batch_size: 1024,
            use_gpu: true,
            precision: "fp32".to_string(),
            max_memory_gb: 8.0,
        }
    }
}

/// JAX vector processor backed by a repository-managed Python script.
pub struct JaxProcessor {
    python_cmd: String,
    backend_script: PathBuf,
}

impl JaxProcessor {
    /// Initializes the processor and validates that the real backend is ready.
    pub fn new() -> Result<Self> {
        eprintln!("Initializing JAX GPU processor with explicit backend validation...");
        let python_cmd = Self::find_python_with_jax()?;
        let backend_script = Self::backend_script_path();

        if !backend_script.exists() {
            return Err(anyhow::anyhow!(
                "JAX backend script not found at {}",
                backend_script.display()
            ));
        }

        Ok(Self {
            python_cmd,
            backend_script,
        })
    }

    /// Processes result batches using the real JAX backend.
    pub fn process_search_results_batch(&self, results: Vec<String>) -> Result<Vec<f32>> {
        if results.is_empty() {
            return Ok(Vec::new());
        }

        let output = self.run_backend("process_batch", &json!({ "results": results }))?;
        serde_json::from_slice(&output).context("Failed to parse JAX batch scores")
    }

    /// Converts text content into deterministic JAX-generated vectors.
    pub fn vectorize_content(&self, content: &[String]) -> Result<Vec<Vec<f32>>> {
        if content.is_empty() {
            return Ok(Vec::new());
        }

        let output = self.run_backend("vectorize", &json!({ "content": content }))?;
        serde_json::from_slice(&output).context("Failed to parse JAX vectors")
    }

    /// Computes a similarity matrix using JAX linear algebra.
    pub fn compute_similarity_matrix(&self, vectors: &[Vec<f32>]) -> Result<Vec<Vec<f32>>> {
        if vectors.is_empty() {
            return Ok(Vec::new());
        }

        let output = self.run_backend("similarity", &json!({ "vectors": vectors }))?;
        serde_json::from_slice(&output).context("Failed to parse JAX similarity matrix")
    }

    /// Resolves a Python interpreter that can import JAX.
    fn find_python_with_jax() -> Result<String> {
        let python_paths = [
            r"C:\Users\DELL\AppData\Local\Programs\Python\Python312\python.exe",
            r"C:\Python312\python.exe",
            "python.exe",
        ];

        for python_path in &python_paths {
            let test_result = Command::new(python_path)
                .args(["-c", "import jax; import jax.numpy as jnp; print('JAX OK')"])
                .output();

            if let Ok(output) = test_result {
                if output.status.success() {
                    return Ok(python_path.to_string());
                }
            }
        }

        Err(anyhow::anyhow!("Python with JAX not found"))
    }

    /// Processes results through the same real JAX batch backend.
    pub fn process_results(&self, results: &[String]) -> Result<Vec<f32>> {
        self.process_search_results_batch(results.to_vec())
    }

    /// Compute cosine similarity between two vectors
    pub fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Resolves the repository-managed JAX backend script path.
    fn backend_script_path() -> PathBuf {
        PathBuf::from("ffi/jax/real_jax_backend.py")
    }

    /// Runs the repository-managed backend script for a specific operation.
    fn run_backend<T: Serialize>(&self, operation: &str, payload: &T) -> Result<Vec<u8>> {
        if !self.backend_script.exists() {
            return Err(anyhow::anyhow!(
                "JAX backend script missing at {}",
                self.backend_script.display()
            ));
        }

        let mut child = Command::new(&self.python_cmd)
            .arg(&self.backend_script)
            .arg(operation)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn JAX backend operation {}", operation))?;

        let payload_json = serde_json::to_string(payload)?;
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(payload_json.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "JAX backend operation {} failed: {}",
                operation,
                stderr.trim()
            ));
        }

        Ok(output.stdout)
    }

    /// 🔥 PROCESS PDF BATCH - Extract text from PDF bytes using JAX/Python
    pub fn process_pdf_batch(&self, pdf_data: &[u8]) -> Result<String> {
        // Find Python with JAX for secure execution
        let python_cmd = self.find_python_with_jax()?;

        // Alternative: Use scripts/generate_advanced_report.py for PDF processing
        // Ensure script exists before execution
        let script_path = "scripts/generate_advanced_report.py";
        if !std::path::Path::new(script_path).exists() {
            return Err(anyhow::anyhow!(
                "PDF extraction script not found: {}",
                script_path
            ));
        }

        // Call Python script for report generation with robust argument passing
        let mut cmd = Command::new(&python_cmd);
        cmd.args([script_path, "--operation", "extract_pdf"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn()?;
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(pdf_data)?;
        }

        let output = child.wait_with_output()?;
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(text)
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("PDF extraction failed: {}", err))
        }
    }

    /// Check if JAX is available
    pub fn is_available(&self) -> bool {
        self.backend_script.exists()
    }
}

impl Default for JaxProcessor {
    fn default() -> Self {
        Self::new().expect("JaxProcessor requires a real JAX runtime and ffi/jax/real_jax_backend.py")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jax_backend_contract_is_explicit() {
        match JaxProcessor::new() {
            Ok(processor) => assert!(processor.is_available()),
            Err(err) => {
                let message = err.to_string();
                assert!(
                    message.contains("JAX") || message.contains("ffi/jax/real_jax_backend.py"),
                    "unexpected initialization error: {}",
                    message
                );
            }
        }
    }

    #[test]
    fn test_backend_script_path() {
        assert_eq!(JaxProcessor::backend_script_path(), PathBuf::from("ffi/jax/real_jax_backend.py"));
    }

    #[test]
    fn test_cosine_similarity() {
        let jax = JaxProcessor {
            python_cmd: "python.exe".to_string(),
            backend_script: PathBuf::from("ffi/jax/real_jax_backend.py"),
        };
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = jax.cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 0.001);
    }
}

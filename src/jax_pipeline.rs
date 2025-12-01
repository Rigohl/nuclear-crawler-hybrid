//! Módulo JAX Pipeline - Pipeline completo de procesamiento acelerado
//!
//! Integra jax_pipeline.py para acelerar todo el procesamiento
//! y ahorrar código reutilizando el pipeline

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use std::process::Command; // No usado, solo AsyncCommand
use std::sync::Arc;
use tokio::process::Command as AsyncCommand;

/// Configuración del pipeline JAX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaxPipelineConfig {
    /// Usar JAX para aceleración
    pub enabled: bool,
    
    /// Dispositivo (cpu, gpu, tpu)
    pub device: String,
    
    /// Path al script Python
    pub script_path: String,
    
    /// Tamaño de batch
    pub batch_size: usize,
}

impl Default for JaxPipelineConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            device: "cpu".to_string(),
            script_path: "scripts/jax_pipeline.py".to_string(),
            batch_size: 32,
        }
    }
}

/// Pipeline JAX integrado
pub struct JaxPipeline {
    config: JaxPipelineConfig,
}

impl JaxPipeline {
    /// Crea un nuevo pipeline
    pub fn new(config: JaxPipelineConfig) -> Self {
        Self { config }
    }
    
    /// Procesa un batch de resultados de scraping
    pub async fn process_batch(&self, batch: Vec<Value>) -> Result<Vec<Value>> {
        if !self.config.enabled {
            return Ok(batch);
        }
        
        let json_data = serde_json::to_string(&batch)?;
        
        let output = AsyncCommand::new("python")
            .arg(&self.config.script_path)
            .arg("--device")
            .arg(&self.config.device)
            .arg("--operation")
            .arg("process_batch")
            .arg("--data")
            .arg(&json_data)
            .output()
            .await?;
        
        if output.status.success() {
            let result: Vec<Value> = serde_json::from_slice(&output.stdout)?;
            Ok(result)
        } else {
            // Fallback a procesamiento Rust
            Ok(self.process_batch_rust(batch))
        }
    }
    
    /// Vectoriza URLs
    pub async fn vectorize_urls(&self, urls: Vec<String>) -> Result<Vec<String>> {
        if !self.config.enabled {
            return Ok(urls);
        }
        
        let json_data = serde_json::to_string(&urls)?;
        
        let output = AsyncCommand::new("python")
            .arg(&self.config.script_path)
            .arg("--device")
            .arg(&self.config.device)
            .arg("--operation")
            .arg("vectorize_urls")
            .arg("--data")
            .arg(&json_data)
            .output()
            .await?;
        
        if output.status.success() {
            let result: Vec<String> = serde_json::from_slice(&output.stdout)?;
            Ok(result)
        } else {
            Ok(urls)
        }
    }
    
    /// Procesa URLs en batches
    pub async fn batch_process_urls(&self, urls: Vec<String>) -> Result<Vec<Value>> {
        if !self.config.enabled {
            return Ok(urls.into_iter().map(|u| json!({"url": u})).collect());
        }
        
        let json_data = serde_json::to_string(&urls)?;
        
        let output = AsyncCommand::new("python")
            .arg(&self.config.script_path)
            .arg("--device")
            .arg(&self.config.device)
            .arg("--operation")
            .arg("batch_process_urls")
            .arg("--data")
            .arg(&json_data)
            .arg("--batch_size")
            .arg(self.config.batch_size.to_string())
            .output()
            .await?;
        
        if output.status.success() {
            let result: Vec<Value> = serde_json::from_slice(&output.stdout)?;
            Ok(result)
        } else {
            Ok(urls.into_iter().map(|u| json!({"url": u})).collect())
        }
    }
    
    /// Pipeline completo de procesamiento
    pub async fn pipeline_process(
        &self,
        data: Vec<Value>,
        stages: Vec<String>,
    ) -> Result<Vec<Value>> {
        if !self.config.enabled {
            return Ok(data);
        }
        
        let json_data = serde_json::to_string(&data)?;
        let stages_str = stages.join(",");
        
        let output = AsyncCommand::new("python")
            .arg(&self.config.script_path)
            .arg("--device")
            .arg(&self.config.device)
            .arg("--operation")
            .arg("pipeline")
            .arg("--data")
            .arg(&json_data)
            .arg("--stages")
            .arg(&stages_str)
            .output()
            .await?;
        
        if output.status.success() {
            let result: Vec<Value> = serde_json::from_slice(&output.stdout)?;
            Ok(result)
        } else {
            Ok(data)
        }
    }
    
    /// Fallback a procesamiento Rust
    fn process_batch_rust(&self, batch: Vec<Value>) -> Vec<Value> {
        use rayon::prelude::*;
        
        batch
            .into_par_iter()
            .map(|mut item| {
                // Calcular scores básicos
                let html_len = item.get("html")
                    .and_then(|h| h.as_str())
                    .map(|s| s.len())
                    .unwrap_or(0) as f64;
                
                let links_count = item.get("links")
                    .and_then(|l| l.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0) as f64;
                
                let images_count = item.get("images")
                    .and_then(|i| i.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0) as f64;
                
                // Calcular quality score
                let quality_score = (html_len / 10000.0 * 0.3 +
                                   links_count / 100.0 * 0.2 +
                                   images_count / 50.0 * 0.2)
                    .min(1.0);
                
                let relevance_score = item.get("relevance_score")
                    .and_then(|r| r.as_f64())
                    .unwrap_or(0.5);
                
                let total_score = quality_score * 0.6 + relevance_score * 0.4;
                
                // Actualizar item (usar as_object_mut para mutabilidad)
                if let Some(obj) = item.as_object_mut() {
                    obj.insert("quality_score".to_string(), json!(quality_score));
                    obj.insert("relevance_score".to_string(), json!(relevance_score));
                    obj.insert("total_score".to_string(), json!(total_score));
                }
                
                item
            })
            .collect()
    }
}

/// Helper para usar el pipeline fácilmente
pub struct PipelineHelper {
    pipeline: Arc<JaxPipeline>,
}

impl PipelineHelper {
    /// Crea un nuevo helper
    pub fn new(config: JaxPipelineConfig) -> Self {
        Self {
            pipeline: Arc::new(JaxPipeline::new(config)),
        }
    }
    
    /// Procesa resultados de scraping masivamente
    pub async fn process_scraping_results(
        &self,
        results: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        // Convertir a formato del pipeline
        let batch: Vec<Value> = results
            .into_iter()
            .map(|r| {
                json!({
                    "url": r.get("url"),
                    "html": r.get("html").and_then(|h| h.as_str()).map(|s| s.len()),
                    "links": r.get("links_found"),
                    "images": r.get("images_found"),
                    "headings_count": r.get("extracted_data")
                        .and_then(|e| e.get("headings"))
                        .and_then(|h| h.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0),
                    "tables_count": r.get("extracted_data")
                        .and_then(|e| e.get("tables"))
                        .and_then(|t| t.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0),
                    "relevance_score": 0.5, // Se calculará después
                })
            })
            .collect();
        
        // Procesar con pipeline
        let processed = self.pipeline.process_batch(batch).await?;
        
        Ok(processed)
    }
    
    /// Pipeline completo para búsqueda web
    pub async fn process_web_search_results(
        &self,
        results: Vec<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        let stages = vec![
            "normalize".to_string(),
            "extract_features".to_string(),
            "calculate_scores".to_string(),
            "rank".to_string(),
            "filter".to_string(),
        ];
        
        self.pipeline.pipeline_process(results, stages).await
    }
}


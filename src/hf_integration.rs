//! Módulo Hugging Face Integration
//! 
//! Integración con modelos de Hugging Face usando Candle

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hf")]
use candle_core::{Device, Tensor};
#[cfg(feature = "hf")]
use candle_transformers::models::bert::{BertModel, Config};
#[cfg(feature = "hf")]
use candle_nn::VarBuilder;

/// Integración con Hugging Face
pub struct HfIntegration {
    enabled: bool,
    models_path: String,
}

impl HfIntegration {
    /// Crea nueva integración HF
    pub fn new(enabled: bool) -> Self {
        let models_path = std::env::var("HF_MODELS_PATH")
            .unwrap_or_else(|_| "C:\\Users\\DELL\\Desktop\\hf_spaces\\MODELOS_IA".to_string());
        
        Self {
            enabled,
            models_path,
        }
    }
    
    /// Lista modelos disponibles
    pub fn list_models(&self) -> Result<Vec<String>> {
        if !self.enabled {
            return Ok(vec![]);
        }
        
        let models_dir = std::path::Path::new(&self.models_path);
        if !models_dir.exists() {
            return Ok(vec![]);
        }
        
        let mut models = Vec::new();
        for entry in std::fs::read_dir(models_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    models.push(name.to_string());
                }
            }
        }
        
        Ok(models)
    }
    
    /// Carga un modelo (stub - implementar con Candle)
    #[cfg(feature = "hf")]
    pub fn load_model(&self, model_name: &str) -> Result<()> {
        let model_path = std::path::Path::new(&self.models_path).join(model_name);
        
        if !model_path.exists() {
            return Err(anyhow::anyhow!("Model not found: {}", model_name));
        }
        
        // TODO: Implementar carga con Candle
        // let device = Device::Cpu;
        // let config = Config::from_file(model_path.join("config.json"))?;
        // let vb = VarBuilder::from_pth(&model_path, DType::F32, &device)?;
        // let model = BertModel::load(&vb, &config)?;
        
        Ok(())
    }
    
    #[cfg(not(feature = "hf"))]
    pub fn load_model(&self, _model_name: &str) -> Result<()> {
        Err(anyhow::anyhow!("HF feature not enabled"))
    }
    
    /// Busca modelos en Hugging Face Hub
    pub async fn search_models(&self, query: &str) -> Result<Vec<ModelInfo>> {
        if !self.enabled {
            return Ok(vec![]);
        }
        
        let client = reqwest::Client::new();
        let url = format!("https://huggingface.co/api/models?search={}", 
            urlencoding::encode(query));
        
        let response = client.get(&url).send().await?;
        let models: Vec<ModelInfo> = response.json().await?;
        
        Ok(models)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub author: Option<String>,
    pub downloads: Option<u64>,
    pub tags: Option<Vec<String>>,
}

impl Default for HfIntegration {
    fn default() -> Self {
        Self::new(true)
    }
}


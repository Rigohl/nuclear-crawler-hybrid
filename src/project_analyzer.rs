//! Módulo Project Analyzer - Analiza proyecto y busca lo más moderno
//!
//! Analiza el proyecto y busca las mejores herramientas/librerías modernas

use crate::web_search::WebSearch;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

/// Configuración de análisis de proyecto
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectAnalysisConfig {
    /// Directorio del proyecto
    pub project_dir: PathBuf,

    /// Lenguaje de programación
    pub language: Option<String>,

    /// Buscar librerías modernas
    pub search_modern_libs: bool,

    /// Buscar mejores prácticas
    pub search_best_practices: bool,

    /// Buscar herramientas
    pub search_tools: bool,
}

/// Resultado de análisis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    /// Tipo de análisis
    pub analysis_type: String,

    /// Recomendaciones
    pub recommendations: Vec<Recommendation>,

    /// Score de modernidad
    pub modernity_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Título
    pub title: String,

    /// Descripción
    pub description: String,

    /// URL/Referencia
    pub reference: String,

    /// Prioridad (1-10)
    pub priority: u8,

    /// Categoría
    pub category: String,
}

/// Analizador de proyecto
pub struct ProjectAnalyzer {
    web_search: Arc<WebSearch>,
}

impl ProjectAnalyzer {
    /// Crea nuevo analizador
    pub fn new() -> Result<Self> {
        Ok(Self {
            web_search: Arc::new(WebSearch::new_with_storage(None)?),
        })
    }

    /// Analiza proyecto completo
    pub async fn analyze_project(
        &self,
        config: ProjectAnalysisConfig,
    ) -> Result<ProjectAnalysisResult> {
        println!("🔍 ANÁLISIS DE PROYECTO");
        println!("   Directorio: {}", config.project_dir.display());

        let mut recommendations = Vec::new();

        // 1. Detectar lenguaje
        let language = config
            .language
            .clone()
            .or_else(|| self.detect_language(&config.project_dir));
        println!("   Lenguaje detectado: {:?}", language);

        // 2. Analizar dependencias
        if let Some(lang) = &language {
            let deps = self.analyze_dependencies(&config.project_dir, lang).await?;
            recommendations.extend(deps);
        }

        // 3. Buscar librerías modernas
        if config.search_modern_libs {
            let modern_libs = self.search_modern_libraries(&language).await?;
            recommendations.extend(modern_libs);
        }

        // 4. Buscar mejores prácticas
        if config.search_best_practices {
            let best_practices = self.search_best_practices(&language).await?;
            recommendations.extend(best_practices);
        }

        // 5. Buscar herramientas
        if config.search_tools {
            let tools = self.search_tools(&language).await?;
            recommendations.extend(tools);
        }

        // Calcular score de modernidad
        let modernity_score = self.calculate_modernity_score(&recommendations);

        Ok(ProjectAnalysisResult {
            analysis_type: "full".to_string(),
            recommendations,
            modernity_score,
        })
    }

    /// Detecta lenguaje del proyecto
    fn detect_language(&self, dir: &Path) -> Option<String> {
        // Buscar archivos característicos
        if dir.join("Cargo.toml").exists() {
            return Some("rust".to_string());
        }
        if dir.join("package.json").exists() {
            return Some("javascript".to_string());
        }
        if dir.join("requirements.txt").exists() || dir.join("setup.py").exists() {
            return Some("python".to_string());
        }
        if dir.join("go.mod").exists() {
            return Some("go".to_string());
        }
        None
    }

    /// Analiza dependencias
    async fn analyze_dependencies(
        &self,
        dir: &Path,
        language: &str,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        if language == "rust" {
            if let Ok(content) = fs::read_to_string(dir.join("Cargo.toml")) {
                // Buscar dependencias desactualizadas
                if !content.contains("tokio") {
                    recommendations.push(Recommendation {
                        title: "Agregar Tokio para async".to_string(),
                        description: "Tokio es el runtime async más moderno para Rust".to_string(),
                        reference: "https://tokio.rs/".to_string(),
                        priority: 9,
                        category: "dependencies".to_string(),
                    });
                }
            }
        }

        Ok(recommendations)
    }

    /// Busca librerías modernas
    async fn search_modern_libraries(
        &self,
        language: &Option<String>,
    ) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("modern {} libraries 2024", lang)
        } else {
            "modern programming libraries 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 20,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "modern_libraries".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Busca mejores prácticas
    async fn search_best_practices(
        &self,
        language: &Option<String>,
    ) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("{} best practices 2024", lang)
        } else {
            "programming best practices 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 15,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "best_practices".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Busca herramientas
    async fn search_tools(&self, language: &Option<String>) -> Result<Vec<Recommendation>> {
        let query = if let Some(lang) = language {
            format!("best {} tools 2024", lang)
        } else {
            "best development tools 2024".to_string()
        };

        let search_config = crate::web_search::WebSearchConfig {
            query,
            max_results: 15,
            ..Default::default()
        };

        let results = self.web_search.search(search_config).await?;

        let recommendations: Vec<Recommendation> = results
            .into_iter()
            .take(10)
            .map(|r| Recommendation {
                title: r.title,
                description: r.description,
                reference: r.url,
                priority: (r.relevance * 10.0) as u8,
                category: "tools".to_string(),
            })
            .collect();

        Ok(recommendations)
    }

    /// Calcula score de modernidad
    fn calculate_modernity_score(&self, recommendations: &[Recommendation]) -> f32 {
        if recommendations.is_empty() {
            return 0.5;
        }

        let avg_priority = recommendations
            .iter()
            .map(|r| r.priority as f32)
            .sum::<f32>()
            / recommendations.len() as f32;

        (avg_priority / 10.0).min(1.0)
    }
}

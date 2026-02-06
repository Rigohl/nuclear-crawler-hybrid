use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ndarray::{Array1, Array2};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathDatasetEntry {
    pub id: String,
    pub source: String,
    pub category: String,
    pub content: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub embeddings: Option<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct ProcessedMathData {
    pub entries: Vec<MathDatasetEntry>,
    pub category_stats: HashMap<String, usize>,
    pub difficulty_distribution: HashMap<String, usize>,
    pub semantic_clusters: Vec<Vec<usize>>, // Grupos semánticos
    pub complexity_scores: Vec<f32>,
}

#[pyclass]
pub struct MathDatasetProcessor {
    dataset: ProcessedMathData,
}

#[pymethods]
impl MathDatasetProcessor {
    #[new]
    fn new() -> Self {
        MathDatasetProcessor {
            dataset: ProcessedMathData {
                entries: Vec::new(),
                category_stats: HashMap::new(),
                difficulty_distribution: HashMap::new(),
                semantic_clusters: Vec::new(),
                complexity_scores: Vec::new(),
            },
        }
    }

    fn load_dataset(&mut self, jsonl_path: &str) -> PyResult<()> {
        println!("Cargando dataset desde: {}", jsonl_path);

        let file = std::fs::File::open(jsonl_path)?;
        let reader = std::io::BufReader::new(file);

        let mut entries = Vec::new();
        let mut category_stats = HashMap::new();
        let mut difficulty_dist = HashMap::new();

        for line in std::io::BufRead::lines(reader) {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<MathDatasetEntry>(&line) {
                Ok(mut entry) => {
                    // Procesar estadísticas
                    *category_stats.entry(entry.category.clone()).or_insert(0) += 1;
                    *difficulty_dist.entry(entry.difficulty.clone()).or_insert(0) += 1;

                    entries.push(entry);
                }
                Err(e) => {
                    eprintln!("Error parseando línea: {}", e);
                    continue;
                }
            }
        }

        println!("Dataset cargado: {} entradas", entries.len());

        self.dataset = ProcessedMathData {
            entries,
            category_stats,
            difficulty_distribution: difficulty_dist,
            semantic_clusters: Vec::new(),
            complexity_scores: Vec::new(),
        };

        Ok(())
    }

    fn compute_complexity_scores(&mut self) -> PyResult<Vec<f32>> {
        println!("Computando scores de complejidad...");

        let scores: Vec<f32> = self.dataset.entries.par_iter()
            .map(|entry| self.compute_entry_complexity(entry))
            .collect();

        self.dataset.complexity_scores = scores.clone();
        println!("Scores de complejidad computados: {}", scores.len());

        Ok(scores)
    }

    fn cluster_by_semantics(&mut self, num_clusters: usize) -> PyResult<Vec<Vec<usize>>> {
        println!("Realizando clustering semántico con {} clusters...", num_clusters);

        // Implementación simplificada de clustering semántico
        // En producción usaría algoritmos más sofisticados como K-means o DBSCAN
        let mut clusters = vec![Vec::new(); num_clusters];

        for (i, entry) in self.dataset.entries.iter().enumerate() {
            let cluster_id = self.assign_cluster(entry, num_clusters);
            clusters[cluster_id].push(i);
        }

        self.dataset.semantic_clusters = clusters.clone();
        println!("Clustering completado");

        Ok(clusters)
    }

    fn optimize_dataset(&mut self) -> PyResult<()> {
        println!("Optimizando dataset...");

        // Paralelizar procesamiento de optimización
        self.dataset.entries.par_iter_mut().for_each(|entry| {
            self.optimize_entry(entry);
        });

        println!("Dataset optimizado");
        Ok(())
    }

    fn get_category_statistics(&self) -> PyResult<HashMap<String, usize>> {
        Ok(self.dataset.category_stats.clone())
    }

    fn get_difficulty_distribution(&self) -> PyResult<HashMap<String, usize>> {
        Ok(self.dataset.difficulty_distribution.clone())
    }

    fn search_similar_entries(&self, query_embedding: Vec<f32>, top_k: usize) -> PyResult<Vec<(usize, f32)>> {
        let mut similarities = Vec::new();

        for (i, entry) in self.dataset.entries.iter().enumerate() {
            if let Some(emb) = &entry.embeddings {
                let sim = cosine_similarity(&query_embedding, emb);
                similarities.push((i, sim));
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(top_k);

        Ok(similarities)
    }

    fn export_processed_dataset(&self, output_path: &str) -> PyResult<()> {
        println!("Exportando dataset procesado a: {}", output_path);

        let processed_data = serde_json::to_string_pretty(&self.dataset)?;

        std::fs::write(output_path, processed_data)?;
        println!("Dataset exportado exitosamente");

        Ok(())
    }
}

impl MathDatasetProcessor {
    fn compute_entry_complexity(&self, entry: &MathDatasetEntry) -> f32 {
        let mut score = 0.0;

        // Factor de dificultad base
        score += match entry.difficulty.as_str() {
            "beginner" => 1.0,
            "intermediate" => 2.0,
            "advanced" => 3.0,
            "expert" => 4.0,
            _ => 2.0,
        };

        // Factor de longitud del contenido
        let content_len = entry.content.len() as f32;
        score += (content_len / 1000.0).min(3.0);

        // Factor de tags especializados
        let specialized_tags = ["quantum", "topology", "category_theory", "differential_geometry"];
        let tag_bonus: f32 = entry.tags.iter()
            .filter(|tag| specialized_tags.contains(&tag.as_str()))
            .count() as f32;
        score += tag_bonus * 0.5;

        score.max(1.0).min(10.0)
    }

    fn assign_cluster(&self, entry: &MathDatasetEntry, num_clusters: usize) -> usize {
        // Asignación simple basada en hash del contenido
        // En producción usaría embeddings y clustering real
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        entry.content.hash(&mut hasher);
        entry.category.hash(&mut hasher);

        (hasher.finish() % num_clusters as u64) as usize
    }

    fn optimize_entry(&self, entry: &mut MathDatasetEntry) {
        // Optimizaciones básicas
        entry.content = entry.content.trim().to_string();

        // Normalizar tags
        entry.tags = entry.tags.iter()
            .map(|tag| tag.to_lowercase().replace(" ", "_"))
            .collect();

        // Remover duplicados
        let mut seen = std::collections::HashSet::new();
        entry.tags.retain(|tag| seen.insert(tag.clone()));
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

#[pymodule]
fn math_dataset_processor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MathDatasetProcessor>()?;
    Ok(())
}
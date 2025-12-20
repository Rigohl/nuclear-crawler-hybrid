use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::{Value, json};
use chrono::{NaiveDate, NaiveDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let websearch_dir = "resultados/websearch";
    let consolidated_dir = "resultados/websearch_consolidated";

    // Crear directorio consolidado si no existe
    fs::create_dir_all(consolidated_dir)?;

    // Leer todos los archivos JSON
    let mut files_by_date: HashMap<String, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(websearch_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                // Extraer fecha del nombre del archivo (formato: websearch_YYYYMMDD_HHMMSS.json)
                if let Some(date_part) = filename.strip_prefix("websearch_").and_then(|s| s.split('_').next()) {
                    if date_part.len() == 8 {
                        files_by_date.entry(date_part.to_string()).or_insert(Vec::new()).push(filename.to_string());
                    }
                }
            }
        }
    }

    println!("Encontrados {} grupos de fechas con archivos websearch", files_by_date.len());

    // Procesar cada grupo de fechas
    for (date, filenames) in files_by_date {
        println!("Procesando fecha {} con {} archivos", date, filenames.len());

        let mut consolidated_data = Vec::new();
        let mut total_execution_time = 0i64;
        let mut queries_count = 0;
        let mut results_count = 0;

        for filename in filenames {
            let filepath = format!("{}/{}", websearch_dir, filename);
            match fs::read_to_string(&filepath) {
                Ok(content) => {
                    match serde_json::from_str::<Value>(&content) {
                        Ok(json_data) => {
                            // Extraer información relevante
                            if let Some(execution_time) = json_data.get("total_execution_time_ms").and_then(|v| v.as_i64()) {
                                total_execution_time += execution_time;
                            }

                            if let Some(queries) = json_data.get("queries_count").and_then(|v| v.as_u64()) {
                                queries_count += queries as usize;
                            }

                            if let Some(results) = json_data.get("results") {
                                if let Some(results_array) = results.as_array() {
                                    results_count += results_array.len();
                                    consolidated_data.extend(results_array.clone());
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error parseando {}: {}", filename, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error leyendo {}: {}", filename, e);
                }
            }
        }

        // Crear archivo consolidado
        let consolidated_filename = format!("{}/websearch_consolidated_{}.json", consolidated_dir, date);
        let consolidated_json = json!({
            "date": date,
            "total_files_consolidated": consolidated_data.len(),
            "total_execution_time_ms": total_execution_time,
            "total_queries": queries_count,
            "total_results": results_count,
            "consolidated_at": chrono::Utc::now().to_rfc3339(),
            "data": consolidated_data
        });

        fs::write(&consolidated_filename, serde_json::to_string_pretty(&consolidated_json)?)?;
        println!("Creado archivo consolidado: {} ({} resultados)", consolidated_filename, consolidated_data.len());
    }

    println!("Consolidación completada exitosamente");
    Ok(())
}
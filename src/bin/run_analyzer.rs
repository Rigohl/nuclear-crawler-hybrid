use std::path::PathBuf;
use std::sync::Arc;
use nuclear_crawler_hybrid::project_analyzer::ProjectAnalyzer;
use nuclear_crawler_hybrid::web_search::WebSearch;

#[tokio::main]
async fn main() {
    eprintln!("🔍 Analizando proyecto en: .");
    eprintln!("═══════════════════════════════════════════════════════════════\n");
    
    match WebSearch::new_with_storage(None) {
        Ok(web_search) => {
            let analyzer = ProjectAnalyzer::new(Arc::new(web_search));
            match analyzer.analyze(&PathBuf::from(".")).await {
                Ok(result) => {
                    match serde_json::to_string_pretty(&result) {
                        Ok(json) => println!("{}", json),
                        Err(e) => eprintln!("❌ Error serializando: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Error analizando: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Error inicializando WebSearch: {}", e),
    }
}

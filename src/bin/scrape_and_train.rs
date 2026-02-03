use anyhow::Result;
use clap::{Parser, Subcommand};
use nuclear_crawler_hybrid::ai::onlyfans_chatbot::{ChatbotPersonality, OnlyFansChatbot};
///! Scrape and Train - Scrapea web y entrena bots automáticamente
use nuclear_crawler_hybrid::ai::training_pipeline::{TrainingPipeline, TrainingReport};

#[derive(Parser)]
#[command(name = "scrape-and-train")]
#[command(about = "Scrapea web y entrena bots automáticamente")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scrapea URLs y genera training data
    Scrape {
        /// URLs a scrapear
        #[arg(short, long, value_delimiter = ',')]
        urls: Vec<String>,

        /// Output file
        #[arg(short, long, default_value = "training_data.json")]
        output: String,
    },

    /// Entrena bot con datos scrapeados
    Train {
        /// Training data file
        #[arg(short, long, default_value = "training_data.json")]
        input: String,

        /// Bot name
        #[arg(short, long, default_value = "Luna")]
        name: String,

        /// Output bot file
        #[arg(short, long, default_value = "trained_bot.json")]
        output: String,
    },

    /// Scrapea Y entrena en un solo paso
    ScrapeAndTrain {
        /// URLs a scrapear
        #[arg(short, long, value_delimiter = ',')]
        urls: Vec<String>,

        /// Bot name
        #[arg(short, long, default_value = "Luna")]
        name: String,

        /// Output bot file
        #[arg(short, long, default_value = "trained_bot.json")]
        output: String,
    },

    /// Test bot entrenado
    Test {
        /// Bot file
        #[arg(short, long)]
        bot: String,

        /// Test messages
        #[arg(short, long, value_delimiter = ',')]
        messages: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scrape { urls, output } => {
            scrape_urls(urls, &output).await?;
        }

        Command::Train {
            input,
            name,
            output,
        } => {
            train_bot(&input, &name, &output)?;
        }

        Command::ScrapeAndTrain { urls, name, output } => {
            // Primero scrapear
            let temp_data = "temp_training_data.json";
            scrape_urls(urls, temp_data).await?;

            // Luego entrenar
            train_bot(temp_data, &name, &output)?;

            // Limpiar temp
            std::fs::remove_file(temp_data).ok();
        }

        Command::Test { bot, messages } => {
            test_bot(&bot, messages)?;
        }
    }

    Ok(())
}

async fn scrape_urls(urls: Vec<String>, output: &str) -> Result<()> {
    use reqwest;

    println!("🕷️  NUCLEAR SCRAPER - Starting...");
    println!("================================");
    println!();

    let mut all_training_data = serde_json::json!({
        "conversations": [],
        "patterns": []
    });

    for url in urls {
        println!("📡 Scraping: {}", url);

        let response = reqwest::get(&url).await?;
        let html = response.text().await?;

        // WASM scraping (simular, ya que WASM se ejecuta en browser)
        // En producción, esto se ejecutaría en un browser headless
        let scraped = scrape_html_fast(&html, &url);

        // Merge data
        if let Some(convs) = all_training_data["conversations"].as_array_mut() {
            convs.extend(scraped["conversations"].as_array().unwrap().clone());
        }
        if let Some(pats) = all_training_data["patterns"].as_array_mut() {
            pats.extend(scraped["patterns"].as_array().unwrap().clone());
        }

        println!(
            "   ✅ Scraped {} conversations",
            scraped["conversations"].as_array().unwrap().len()
        );
    }

    // Guardar
    std::fs::write(output, serde_json::to_string_pretty(&all_training_data)?)?;

    println!();
    println!("✅ Training data saved to: {}", output);
    println!(
        "   Total conversations: {}",
        all_training_data["conversations"].as_array().unwrap().len()
    );

    Ok(())
}

fn scrape_html_fast(html: &str, source: &str) -> serde_json::Value {
    // Scraping ultra-rápido (simplificado para demo)
    let mut conversations = Vec::new();
    let mut patterns = Vec::new();

    // Detectar mensajes en HTML
    let lines: Vec<&str> = html.lines().collect();
    let mut current_msgs = Vec::new();

    for line in lines.iter().take(100) {
        // Limitar para demo
        if line.contains("message") || line.contains("comment") {
            if let Some(text) = extract_text_simple(line) {
                current_msgs.push(serde_json::json!({
                    "text": text,
                    "likes": 0,
                    "replies": 0,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }));

                if current_msgs.len() >= 3 {
                    conversations.push(serde_json::json!({
                        "messages": current_msgs.clone(),
                        "source": source,
                        "engagement_score": 6.5,
                        "context": "general"
                    }));

                    patterns.push(serde_json::json!({
                        "pattern_type": "opening_line",
                        "text": current_msgs[0]["text"],
                        "effectiveness": 7.0,
                        "context": "greeting"
                    }));

                    current_msgs.clear();
                }
            }
        }
    }

    serde_json::json!({
        "conversations": conversations,
        "patterns": patterns
    })
}

fn extract_text_simple(html: &str) -> Option<String> {
    if let Some(start) = html.find('>') {
        if let Some(end) = html[start..].find('<') {
            let text = html[start + 1..start + end].trim();
            if !text.is_empty() && text.len() > 3 {
                return Some(text.to_string());
            }
        }
    }
    None
}

fn train_bot(input: &str, name: &str, output: &str) -> Result<()> {
    println!("🎓 TRAINING BOT: {}", name);
    println!("================================");
    println!();

    // Leer training data
    let json_data = std::fs::read_to_string(input)?;

    // Crear pipeline
    let mut pipeline = TrainingPipeline::new(name);

    // Cargar datos
    pipeline
        .load_scraped_data(&json_data)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Entrenar
    let report = pipeline.train();

    // Guardar bot
    pipeline.save_bot(output)?;

    println!();
    println!("🎉 Bot trained successfully!");
    println!("   Saved to: {}", output);

    Ok(())
}

fn test_bot(bot_file: &str, messages: Vec<String>) -> Result<()> {
    println!("🧪 TESTING BOT");
    println!("================================");
    println!();

    // Cargar bot (simplificado)
    let bot_data = std::fs::read_to_string(bot_file)?;

    println!("Bot loaded from: {}", bot_file);
    println!();

    // Crear bot para testing
    let mut bot = OnlyFansChatbot::new("Test".to_string());

    // Test responses
    for msg in messages {
        let response = bot.generate_response(&msg);
        println!("💬 User: {}", msg);
        println!("🤖 Bot:  {}", response);
        println!();
    }

    Ok(())
}

use anyhow::Result;
use clap::{Parser, Subcommand};
// Scrape and Train - Scrapea web y entrena bots automáticamente
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainingReport {
    bot_name: String,
    conversations: usize,
    patterns: usize,
    top_openers: usize,
    top_keywords: usize,
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainedBot {
    name: String,
    top_openers: Vec<String>,
    top_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrainedBotFile {
    bot: TrainedBot,
    report: TrainingReport,
}

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

    let parsed: serde_json::Value = serde_json::from_str(&json_data)?;
    let conversations_len = parsed["conversations"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    let patterns = parsed["patterns"].as_array().cloned().unwrap_or_default();

    // Extract openers from extracted patterns (real data produced by `scrape_urls`)
    let mut opener_freq: HashMap<String, usize> = HashMap::new();
    for p in &patterns {
        let is_opener = p["pattern_type"].as_str() == Some("opening_line");
        if !is_opener {
            continue;
        }
        if let Some(text) = p["text"].as_str() {
            let cleaned = text.trim();
            if cleaned.is_empty() {
                continue;
            }
            *opener_freq.entry(cleaned.to_string()).or_insert(0) += 1;
        }
    }

    let mut top_openers: Vec<(String, usize)> = opener_freq.into_iter().collect();
    top_openers.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top_openers: Vec<String> = top_openers.into_iter().take(20).map(|(s, _)| s).collect();

    // Extract basic keywords from messages (real, deterministic frequency analysis)
    let stopwords = [
        "the", "and", "for", "with", "that", "this", "you", "your", "are", "was", "were", "have",
        "has", "had", "not", "but", "from", "they", "them", "what", "when", "where", "who", "why",
        "how", "que", "para", "con", "una", "uno", "las", "los", "por", "del", "como", "esta",
        "esto", "hola",
    ];
    let stop: std::collections::HashSet<&'static str> = stopwords.into_iter().collect();

    let mut keyword_freq: HashMap<String, usize> = HashMap::new();
    if let Some(convs) = parsed["conversations"].as_array() {
        for conv in convs {
            if let Some(msgs) = conv["messages"].as_array() {
                for msg in msgs {
                    if let Some(text) = msg["text"].as_str() {
                        for raw in text.split_whitespace() {
                            let token = raw
                                .trim_matches(|c: char| !c.is_alphanumeric())
                                .to_lowercase();
                            if token.len() < 4 {
                                continue;
                            }
                            if stop.contains(token.as_str()) {
                                continue;
                            }
                            *keyword_freq.entry(token).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    let mut top_keywords: Vec<(String, usize)> = keyword_freq.into_iter().collect();
    top_keywords.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top_keywords: Vec<String> = top_keywords.into_iter().take(30).map(|(s, _)| s).collect();

    let bot = TrainedBot {
        name: name.to_string(),
        top_openers,
        top_keywords,
    };

    let report = TrainingReport {
        bot_name: name.to_string(),
        conversations: conversations_len,
        patterns: patterns.len(),
        top_openers: bot.top_openers.len(),
        top_keywords: bot.top_keywords.len(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let out = TrainedBotFile { bot, report };

    std::fs::write(output, serde_json::to_string_pretty(&out)?)?;

    println!();
    println!("🎉 Bot trained successfully!");
    println!("   Saved to: {}", output);

    Ok(())
}

fn test_bot(bot_file: &str, messages: Vec<String>) -> Result<()> {
    println!("🧪 TESTING BOT");
    println!("================================");
    println!();

    let bot_data = std::fs::read_to_string(bot_file)?;
    let trained: TrainedBotFile = serde_json::from_str(&bot_data)?;

    println!("Bot loaded from: {} (name: {})", bot_file, trained.bot.name);
    println!(
        "Training report: conversations={}, patterns={}",
        trained.report.conversations, trained.report.patterns
    );
    println!();

    // Test responses
    for msg in messages {
        let response = generate_response(&trained.bot, &msg);
        println!("💬 User: {}", msg);
        println!("🤖 Bot:  {}", response);
        println!();
    }

    Ok(())
}

fn generate_response(bot: &TrainedBot, message: &str) -> String {
    let msg_lower = message.to_lowercase();

    // Deterministic selection based on message hash (stable across runs)
    let pick = |items: &[String]| -> Option<String> {
        if items.is_empty() {
            return None;
        }
        let h = blake3::hash(message.as_bytes());
        let idx = (u64::from_le_bytes(h.as_bytes()[0..8].try_into().unwrap_or([0; 8])) as usize)
            % items.len();
        items.get(idx).cloned()
    };

    let is_greeting = msg_lower.contains("hello")
        || msg_lower.contains("hi")
        || msg_lower.contains("hola")
        || msg_lower.contains("buenas");

    if is_greeting {
        if let Some(opener) = pick(&bot.top_openers) {
            return opener;
        }
        return format!("Hola. Soy {}. ¿En qué te ayudo?", bot.name);
    }

    let is_question = msg_lower.contains('?')
        || msg_lower.contains("what")
        || msg_lower.contains("how")
        || msg_lower.contains("why")
        || msg_lower.contains("que")
        || msg_lower.contains("como")
        || msg_lower.contains("por que");

    if is_question {
        if let Some(kw) = pick(&bot.top_keywords) {
            return format!("Interesante. ¿Quieres profundizar en '{kw}'?");
        }
        return "Interesante. Cuéntame más para darte una mejor respuesta.".to_string();
    }

    if let Some(kw) = pick(&bot.top_keywords) {
        format!("Entendido. Estoy viendo el tema '{kw}'. ¿Qué objetivo tienes?")
    } else {
        "Entendido. ¿Qué objetivo tienes?".to_string()
    }
}

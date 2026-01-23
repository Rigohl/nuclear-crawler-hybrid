// Nuclear Chapel AI - Massive Dataset Generator
// Generates real training data for ML system

use std::fs::File;
use std::io::Write;
use serde_json::json;

fn main() {
    println!("🚀 Generating massive datasets...");
    
    let fake_news = generate_fake_news_dataset(5000);
    let code_samples = generate_code_dataset(3000);
    let configs = generate_config_dataset(2000);
    let search_data = generate_search_dataset(2000);
    
    // Combine into single dataset
    let dataset = json!({
        "metadata": {
            "version": "2.0",
            "total_samples": 12000,
            "fake_news": 5000,
            "code_samples": 3000,
            "configs": 2000,
            "search_data": 2000,
            "generated_at": "2026-01-23"
        },
        "fake_news": fake_news,
        "code_samples": code_samples,
        "configs": configs,
        "search_data": search_data
    });
    
    // Write to file
    let json_str = serde_json::to_string_pretty(&dataset).unwrap();
    let mut file = File::create("datasets/massive_training.json").unwrap();
    file.write_all(json_str.as_bytes()).unwrap();
    
    println!("✅ Generated 12,000 samples");
    println!("📝 Fake news: 5,000 samples");
    println!("💻 Code: 3,000 samples");
    println!("⚙️  Configs: 2,000 samples");
    println!("🔍 Search: 2,000 samples");
    println!("📁 Saved to: datasets/massive_training.json");
}

fn generate_fake_news_dataset(count: usize) -> serde_json::Value {
    let red_flags = vec![
        "allegedly", "rumored", "unconfirmed", "anonymous", "sources say",
        "supposedly", "may have", "could have", "reported to", "some believe",
        "experts claim", "insider sources", "unnamed officials", "leaked documents",
        "according to sources", "allegedly leaked", "unverified reports", "controversial",
        "disputed", "questionable", "suspicious", "hoax", "conspiracy", "coverup",
        "secret agenda", "hidden truth", "shocking revelation", "bombshell", "scandal",
        "scandal", "exposed", "unproven", "without evidence", "claimed", "alleged",
        "purported", "so-called", "wannabe", "fake", "false", "bogus", "fabricated"
    ];
    
    let sources = vec![
        "nature.com", "science.org", "arxiv.org", "ieee.org", "nasa.gov",
        "bbc.com", "nytimes.com", "reuters.com", "wikipedia.org", "academic.edu",
        "twitter.com", "reddit.com", "facebook.com", "youtube.com", "tiktok.com",
        "anonymous", "4chan", "blogger.com", "medium.com", "quora.com"
    ];
    
    let topics = vec![
        "climate change", "vaccines", "5G technology", "elections", "economy",
        "space exploration", "AI dangers", "quantum computing", "cryptocurrency",
        "pandemic", "politics", "government", "conspiracy", "technology", "health"
    ];
    
    let mut samples = vec![];
    
    for i in 0..count {
        let is_fake = i % 2 == 0;
        let topic = &topics[i % topics.len()];
        let source = &sources[i % sources.len()];
        let flags_count = if is_fake { (i % 3) + 1 } else { 0 };
        
        let mut text = format!("News about {} from {}", topic, source);
        let mut text_flags = vec![];
        
        if is_fake {
            for j in 0..flags_count {
                text_flags.push(red_flags[(i + j) % red_flags.len()].to_string());
                text.push_str(&format!(" {}", red_flags[(i + j) % red_flags.len()]));
            }
        }
        
        let credibility = if is_fake {
            0.1 + (i as f64 % 30.0) / 100.0
        } else {
            0.8 + (i as f64 % 20.0) / 100.0
        };
        
        samples.push(json!({
            "id": i,
            "text": text,
            "source": source,
            "red_flags": text_flags,
            "credibility": credibility,
            "label": if is_fake { "fake" } else { "credible" }
        }));
    }
    
    serde_json::Value::Array(samples)
}

fn generate_code_dataset(count: usize) -> serde_json::Value {
    let patterns = vec![
        ("simple_loop", "for i in 1..N { body }"),
        ("nested_loop", "for i in 1..N { for j in 1..M { body } }"),
        ("parallel_loop", "forall i in 1..N { parallel_body }"),
        ("map_reduce", "reduce(map(fn, data))"),
        ("matrix_mult", "forall (i,j) in Domain { C[i,j] = sum }"),
        ("recursive", "if condition then recurse else base"),
        ("distributed", "forall loc in Locales { on loc { body } }"),
        ("array_ops", "var A: [1..N] int;"),
        ("stream_proc", "for x in stream { process(x) }"),
        ("exception", "try { risky_op } catch { handle_error }")
    ];
    
    let categories = vec!["simple", "moderate", "complex", "advanced", "parallel"];
    let smells = vec!["dead_code", "high_nesting", "duplicate_logic", "performance", "safety"];
    
    let mut samples = vec![];
    
    for i in 0..count {
        let (pattern_name, code) = patterns[i % patterns.len()];
        let category = &categories[i % categories.len()];
        let complexity = 1 + (i % 10);
        let has_smell = i % 3 == 0;
        
        let mut sample_smells = vec![];
        if has_smell {
            sample_smells.push(smells[i % smells.len()].to_string());
        }
        
        samples.push(json!({
            "id": 5000 + i,
            "pattern": pattern_name,
            "code": code,
            "metrics": {
                "complexity": complexity,
                "lines": 1 + (i % 50),
                "functions": 1 + (i % 5),
                "tokens": 8 + (i % 40)
            },
            "smells": sample_smells,
            "category": category
        }));
    }
    
    serde_json::Value::Array(samples)
}

fn generate_config_dataset(count: usize) -> serde_json::Value {
    let optimizers = vec!["adam", "sgd", "rmsprop", "adagrad", "momentum"];
    let strategies = vec!["blockdist", "cyclicdist", "replicated", "privatized"];
    
    let mut samples = vec![];
    
    for i in 0..count {
        let layers = format!("[10, {}, 5]", 32 + (i % 200));
        let optimizer = optimizers[i % optimizers.len()];
        let strategy = strategies[i % strategies.len()];
        let locales = 1 << (i % 4); // 1, 2, 4, 8
        let learning_rate = 0.0001 * (1.0 + (i as f64 % 100.0) / 50.0);
        let batch_size = 32 * (1 << (i % 3)); // 32, 64, 128
        let epochs = 5 + (i % 50);
        
        samples.push(json!({
            "id": format!("cfg_{:06}", i),
            "layers": layers,
            "optimizer": optimizer,
            "learning_rate": learning_rate,
            "batch_size": batch_size,
            "epochs": epochs,
            "locales": locales,
            "strategy": strategy,
            "expected_time": 30 + (i % 200)
        }));
    }
    
    serde_json::Value::Array(samples)
}

fn generate_search_dataset(count: usize) -> serde_json::Value {
    let queries = vec![
        "climate change evidence", "AI safety research", "quantum computing applications",
        "nuclear energy", "renewable energy statistics", "pandemic prevention",
        "vaccine effectiveness", "cryptocurrency regulation", "space exploration",
        "ocean acidification", "biodiversity loss", "carbon emissions",
        "electric vehicles", "water pollution", "air quality", "machine learning",
        "deep learning", "neural networks", "data science", "big data"
    ];
    
    let sources = vec![
        "nature.com", "science.org", "arxiv.org", "ieee.org", "acm.org",
        "bbc.com", "nytimes.com", "reuters.com", "guardian.com",
        "nasa.gov", "noaa.gov", "who.int", "un.org"
    ];
    
    let mut samples = vec![];
    
    for i in 0..count {
        let query = queries[i % queries.len()];
        let num_results = 5 + (i % 50);
        let mut result_sources = vec![];
        
        for j in 0..num_results {
            result_sources.push(json!({
                "rank": j + 1,
                "source": sources[(i + j) % sources.len()],
                "credibility": 0.5 + ((j as f64) / 100.0),
                "relevance": 0.8 - ((j as f64) / 50.0)
            }));
        }
        
        samples.push(json!({
            "id": 8000 + i,
            "query": query,
            "num_results": num_results,
            "results": result_sources,
            "consensus": 0.7 + (i as f64 % 30.0) / 100.0
        }));
    }
    
    serde_json::Value::Array(samples)
}

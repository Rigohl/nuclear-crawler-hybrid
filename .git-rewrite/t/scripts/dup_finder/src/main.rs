use clap::Parser;
use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Source directory to scan
    #[arg(long, default_value = "src")]
    src: String,

    /// Output directory for JSON
    #[arg(long, default_value = "memories")]
    out: String,

    /// shingle size
    #[arg(long, default_value_t = 5)]
    k: usize,

    /// similarity threshold for near-duplicate
    #[arg(long, default_value_t = 0.8)]
    threshold: f64,
}

#[derive(Serialize)]
struct FileMeta {
    path: String,
    size: u64,
    sha256: String,
}

#[derive(Serialize)]
struct ExactGroup {
    sha256: String,
    files: Vec<String>,
    size: u64,
}

#[derive(Serialize)]
struct FuzzyPair {
    fileA: String,
    fileB: String,
    shaA: String,
    shaB: String,
    sizeA: u64,
    sizeB: u64,
    similarity: f64,
    top_shared_shingles: Vec<String>,
    suggested_action: String,
}

#[derive(Serialize)]
struct BlockOcc {
    file: String,
    start_line: usize,
    end_line: usize,
}

#[derive(Serialize)]
struct BlockDup {
    snippet: String,
    occurrences: Vec<BlockOcc>,
    tokens_count: usize,
}

#[derive(Serialize)]
struct Report {
    file_metadata: Vec<FileMeta>,
    exact_duplicates: Vec<ExactGroup>,
    fuzzy_duplicates: Vec<FuzzyPair>,
    block_duplicates: Vec<BlockDup>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}

fn normalize_text(path: &Path, s: &str) -> String {
    // Convert CRLF -> LF, collapse whitespace, replace tabs with 4 spaces, trim trailing spaces
    let mut text = s.replace("\r\n", "\n").replace('\r', "\n");
    text = text.replace('\t', "    ");
    // Remove language comments heuristically for Rust/JS/PY
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" | "js" | "ts" => {
                // remove // comments
                if let Ok(re_line) = Regex::new(r"(?m)//.*$") {
                    text = re_line.replace_all(&text, "").to_string();
                }
                // remove /* */
                if let Ok(re_block) = Regex::new(r"(?s)/\*.*?\*/") {
                    text = re_block.replace_all(&text, "").to_string();
                }
            }
            "py" => {
                if let Ok(re_line) = Regex::new(r"(?m)#.*$") {
                    text = re_line.replace_all(&text, "").to_string();
                }
                // Skipping complex triple-quote block removal for safety; rely on line comments removal
            }
            "toml" | "md" => {
                if let Ok(re_line) = Regex::new(r"(?m)^#.*$") {
                    text = re_line.replace_all(&text, "").to_string();
                }
            }
            _ => {}
        }
    }
    // collapse multiple spaces
    if let Ok(re_space) = Regex::new(r"[ \t]{2,}") {
        text = re_space.replace_all(&text, " ").to_string();
    }
    // remove trailing spaces on lines
    if let Ok(re_trail) = Regex::new(r"(?m) +$") {
        text = re_trail.replace_all(&text, "").to_string();
    }
    text.trim().to_string()
}

fn tokenize(s: &str) -> Vec<String> {
    // split on non-alphanumeric and underscore
    // split on non-word characters
    if let Ok(re_split) = Regex::new(r"[^A-Za-z0-9_]+") {
        re_split
            .split(s)
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect()
    } else {
        // fallback: split on whitespace
        s.split_whitespace().map(|t| t.to_string()).collect()
    }
}

fn shingles(tokens: &[String], k: usize) -> HashSet<String> {
    let mut set = HashSet::new();
    if tokens.len() < k {
        if !tokens.is_empty() {
            set.insert(tokens.join(" "));
        }
        return set;
    }
    for i in 0..=tokens.len()-k {
        let sh = tokens[i..i+k].join(" ");
        set.insert(sh);
    }
    set
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let inter = a.intersection(b).count() as f64;
    let uni = a.union(b).count() as f64;
    if uni == 0.0 { 0.0 } else { inter/uni }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let src_dir = PathBuf::from(&args.src);
    let out_dir = PathBuf::from(&args.out);
    fs::create_dir_all(&out_dir)?;

    let mut file_meta: Vec<FileMeta> = Vec::new();
    let mut content_map: HashMap<String, String> = HashMap::new();
    let mut tokens_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut shingles_map: HashMap<String, HashSet<String>> = HashMap::new();

    let exts = vec!["rs","md","toml","js","py","ts"];

    for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let p = entry.path();
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                if !exts.contains(&ext) { continue; }
            } else { continue; }
            let bytes = fs::read(p)?;
            let sha = sha256_hex(&bytes);
            let size = bytes.len() as u64;
            let path_str = p.to_string_lossy().to_string();
            file_meta.push(FileMeta { path: path_str.clone(), size, sha256: sha.clone() });
            let s = String::from_utf8_lossy(&bytes).to_string();
            let norm = normalize_text(p, &s);
            content_map.insert(path_str.clone(), norm.clone());
            let toks = tokenize(&norm);
            tokens_map.insert(path_str.clone(), toks.clone());
            let sh = shingles(&toks, args.k);
            shingles_map.insert(path_str.clone(), sh);
        }
    }

    // exact duplicates by sha256
    let mut sha_groups: HashMap<String, Vec<String>> = HashMap::new();
    let mut size_map: HashMap<String, u64> = HashMap::new();
    for m in &file_meta {
        sha_groups.entry(m.sha256.clone()).or_default().push(m.path.clone());
        size_map.entry(m.sha256.clone()).or_insert(m.size);
    }
    let mut exacts: Vec<ExactGroup> = Vec::new();
    for (sha, files) in sha_groups.iter() {
        if files.len() > 1 {
            let size = *size_map.get(sha).unwrap_or(&0);
            exacts.push(ExactGroup { sha256: sha.clone(), files: files.clone(), size });
        }
    }

    // fuzzy pairs
    let mut fuzzy: Vec<FuzzyPair> = Vec::new();
    let paths: Vec<String> = shingles_map.keys().cloned().collect();
    for i in 0..paths.len() {
        for j in i+1..paths.len() {
            let a = &paths[i];
            let b = &paths[j];
            let (set_a_opt, set_b_opt) = (shingles_map.get(a), shingles_map.get(b));
            let (set_a, set_b) = match (set_a_opt, set_b_opt) {
                (Some(sa), Some(sb)) => (sa, sb),
                _ => continue,
            };
            let sim = jaccard(set_a, set_b);
            if sim >= 0.5 {
                // top shared shingles
                let mut shared: Vec<String> = set_a.intersection(set_b).cloned().collect();
                shared.sort_by_key(|s| std::cmp::Reverse(s.len()));
                let top3 = shared.iter().take(3).cloned().collect();
                // suggested action
                let action = if sim >= args.threshold {
                    "extract_common_lib_or_merge".to_string()
                } else if sim >= 0.5 {
                    "review_suspicious_similarities".to_string()
                } else { "ignore".to_string() };
                let meta_a_opt = file_meta.iter().find(|m| m.path == *a);
                let meta_b_opt = file_meta.iter().find(|m| m.path == *b);
                let (meta_a, meta_b) = match (meta_a_opt, meta_b_opt) {
                    (Some(ma), Some(mb)) => (ma, mb),
                    _ => continue,
                };
                fuzzy.push(FuzzyPair {
                    fileA: a.clone(),
                    fileB: b.clone(),
                    shaA: meta_a.sha256.clone(),
                    shaB: meta_b.sha256.clone(),
                    sizeA: meta_a.size,
                    sizeB: meta_b.size,
                    similarity: (sim*10000.0).round()/10000.0,
                    top_shared_shingles: top3,
                    suggested_action: action,
                });
            }
        }
    }

    // block duplicates: find contiguous windows of >=30 tokens identical across files
    let mut window_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (path, toks) in &tokens_map {
        if toks.len() < 30 { continue; }
        for i in 0..=toks.len()-30 {
            let win = toks[i..i+30].join(" ");
            window_map.entry(win).or_default().push((path.clone(), i));
        }
    }
    let mut blocks: Vec<BlockDup> = Vec::new();
    for (snippet, occs) in window_map.into_iter() {
        if occs.len() > 1 {
            let mut occurrences: Vec<BlockOcc> = Vec::new();
            for (file, idx) in occs {
                // estimate line numbers by counting newlines up to token index - crude but acceptable
                let text = match content_map.get(&file) {
                    Some(t) => t,
                    None => continue,
                };
                let tokens = match tokens_map.get(&file) {
                    Some(toks) => toks,
                    None => continue,
                };
                let before = tokens[..idx].join(" ");
                let start_line = before.matches('\n').count() + 1;
                let end_line = start_line + 10; // heuristic
                occurrences.push(BlockOcc { file: file.clone(), start_line, end_line });
            }
            blocks.push(BlockDup { snippet: snippet.clone(), occurrences, tokens_count: 30 });
        }
    }

    let report = Report {
        file_metadata: file_meta,
        exact_duplicates: exacts,
        fuzzy_duplicates: fuzzy,
        block_duplicates: blocks,
    };

    let out_path = out_dir.join("duplicate_report.json");
    let json = serde_json::to_string_pretty(&report)?;
    fs::write(&out_path, &json)?;

    // also write files metadata
    let meta_path = out_dir.join("source_files.json");
    fs::write(&meta_path, serde_json::to_string_pretty(&report.file_metadata)?)?;

    println!("WROTE_REPORT {}", out_path.display());
    Ok(())
}

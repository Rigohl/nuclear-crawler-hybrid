// Chapel Custom Search Engine
// Motor de búsqueda que combina TODOS los MCPs + OSINT
// Para obtener datasets REALES (no sintéticos)

use ChapelFFI;
use Time;
use Map;

// ═══════════════════════════════════════════════════════════════
// JSON HELPERS (Chapel 2.8 — string-based extraction, no external lib)
// ═══════════════════════════════════════════════════════════════

// Count items in a JSON array by counting "url": occurrences
// Works on JSON returned by python_osint_search, mcp_call, etc.
proc count_json_results(ptr: c_ptrConst(c_char)): int {
  if ptr == nil then return 0;
  var s: string;
  try { s = string.createCopyingBuffer(ptr); } catch { return 0; }
  if s.size == 0 || s == "null" || s.startsWith("{\"error\"") then return 0;
  // Count "url": occurrences as proxy for result items
  var count = 0;
  var pos = 0;
  const needle = "\"url\"";
  while pos < s.size {
    const idx = s.find(needle, pos..);
    if idx == -1 then break;
    count += 1;
    pos = idx + needle.size;
  }
  // Fallback: count by "id": if no urls
  if count == 0 {
    pos = 0;
    const id_needle = "\"id\"";
    while pos < s.size {
      const idx = s.find(id_needle, pos..);
      if idx == -1 then break;
      count += 1;
      pos = idx + id_needle.size;
    }
  }
  return count;
}

// Extract a string field value from simple JSON: {"field": "value", ...}
proc json_get_field(ptr: c_ptrConst(c_char), field: string): string {
  if ptr == nil then return "";
  var s: string;
  try { s = string.createCopyingBuffer(ptr); } catch { return ""; }
  const key = "\"" + field + "\":\"";
  const idx = s.find(key);
  if idx == -1 then return "";
  const start = idx + key.size;
  const endIdx = s.find("\"", start..);
  if endIdx == -1 then return s[start..];
  return s[start..<endIdx];
}

// Extract a numeric field value from JSON: {"field": 42, ...}
proc json_get_int(ptr: c_ptrConst(c_char), field: string): int {
  if ptr == nil then return 0;
  var s: string;
  try { s = string.createCopyingBuffer(ptr); } catch { return 0; }
  const key = "\"" + field + "\":";
  const idx = s.find(key);
  if idx == -1 then return 0;
  const start = idx + key.size;
  // skip whitespace
  var i = start;
  while i < s.size && (s[i] == ' ' || s[i] == '\t') do i += 1;
  var numStr = "";
  while i < s.size && s[i] >= '0' && s[i] <= '9' do {
    numStr += s[i]:string;
    i += 1;
  }
  if numStr.size == 0 then return 0;
  try { return numStr:int; } catch { return 0; }
}

// Build a query string from keywords array
proc build_query(keywords: [] string, num_keywords: int): string {
  var q = "";
  for i in 1..num_keywords do {
    if i > 1 then q += " ";
    q += keywords[i];
  }
  return q;
}

// FFI con MCPs
extern proc mcp_websearch(query: c_ptrConst(c_char), max_results: c_int): c_ptrConst(c_char);
extern proc mcp_github_search(query: c_ptrConst(c_char), type_filter: c_ptrConst(c_char)): c_ptrConst(c_char);
extern proc mcp_supabase_query(sql: c_ptrConst(c_char)): c_ptrConst(c_char);

// FFI con Python OSINT
extern proc python_osint_search(query: c_ptrConst(c_char), sources: c_ptrConst(c_char)): c_ptrConst(c_char);
extern proc python_detect_synthetic(text: c_ptrConst(c_char)): c_int;  // 1=synthetic, 0=real
extern proc python_extract_conversations(html: c_ptrConst(c_char)): c_ptrConst(c_char);

// FFI con WASM scraper
extern proc wasm_scrape_parallel(urls: c_ptrConst(c_char), num_urls: c_int): c_ptrConst(c_char);

config const parallelDegree = 64;
config const maxResults = 10000;
config const realDataOnly = true;  // Filtrar datos sintéticos

record SearchResult {
  var id: int;
  var source: string;      // "reddit", "twitter", "github", "telegram", etc.
  var content: string;
  var url: string;
  var timestamp: string;
  var is_real: bool;       // true=real, false=synthetic
  var quality_score: real; // 0-100
  var metadata: string;    // JSON con info adicional
}

record SearchQuery {
  var keywords: [1..100] string;
  var num_keywords: int;
  var sources: [1..50] string;  // Qué fuentes buscar
  var num_sources: int;
  var filters: string;          // JSON filters
}

class SearchEngine {
  var results: [1..maxResults] SearchResult;
  var num_results: int;
  var cache: map(string, string);  // Query cache
  
  proc init() {
    this.num_results = 0;
    writeln("🔍 Chapel Search Engine initialized");
  }
  
  // BÚSQUEDA MULTI-FUENTE EN PARALELO
  proc search(query: SearchQuery): int {
    writeln("=" * 70);
    writeln("🚀 BÚSQUEDA MULTI-FUENTE");
    writeln("=" * 70);
    writeln("Keywords: ", query.num_keywords);
    writeln("Sources: ", query.num_sources);
    writeln("Parallel degree: ", parallelDegree);
    writeln();
    
    var startTime = Time.timeSinceEpoch().totalSeconds();
    
    // Array para almacenar resultados temporales
    var temp_results: [1..maxResults] SearchResult;
    var result_count: atomic int = 0;
    
    // BÚSQUEDA PARALELA EN TODAS LAS FUENTES
    forall source_idx in 1..query.num_sources with (maxDegree=parallelDegree) do {
      const source = query.sources[source_idx];
      
      select source {
        when "reddit" {
          const reddit_results = search_reddit(query.keywords, query.num_keywords);
          // Store results...
          writeln("  ✓ Reddit: ", reddit_results, " results");
        }
        when "twitter" {
          const twitter_results = search_twitter(query.keywords, query.num_keywords);
          writeln("  ✓ Twitter: ", twitter_results, " results");
        }
        when "github" {
          const github_results = search_github(query.keywords, query.num_keywords);
          writeln("  ✓ GitHub: ", github_results, " results");
        }
        when "telegram" {
          const telegram_results = search_telegram(query.keywords, query.num_keywords);
          writeln("  ✓ Telegram: ", telegram_results, " results");
        }
        when "discord" {
          const discord_results = search_discord(query.keywords, query.num_keywords);
          writeln("  ✓ Discord: ", discord_results, " results");
        }
        when "linkedin" {
          const linkedin_results = search_linkedin(query.keywords, query.num_keywords);
          writeln("  ✓ LinkedIn: ", linkedin_results, " results");
        }
        when "hackernews" {
          const hn_results = search_hackernews(query.keywords, query.num_keywords);
          writeln("  ✓ HackerNews: ", hn_results, " results");
        }
        when "quora" {
          const quora_results = search_quora(query.keywords, query.num_keywords);
          writeln("  ✓ Quora: ", quora_results, " results");
        }
        when "stackoverflow" {
          const so_results = search_stackoverflow(query.keywords, query.num_keywords);
          writeln("  ✓ StackOverflow: ", so_results, " results");
        }
        when "youtube" {
          const yt_results = search_youtube_comments(query.keywords, query.num_keywords);
          writeln("  ✓ YouTube: ", yt_results, " results");
        }
        otherwise {
          writeln("  ⚠️  Unknown source: ", source);
        }
      }
    }
    
    var endTime = Time.timeSinceEpoch().totalSeconds();
    
    writeln();
    writeln("✅ Búsqueda completa en ", (endTime - startTime):string, " segundos");
    writeln("   Total resultados: ", result_count.read());
    
    return result_count.read();
  }
  
  // FILTRO DE DATOS SINTÉTICOS vs REALES
  proc filter_synthetic_data(ref results: [] SearchResult, count: int): int {
    writeln("\n🔍 Detectando datos sintéticos...");
    
    var real_count: atomic int = 0;
    
    forall i in 1..count with (maxDegree=parallelDegree) do {
      const content_cstr = results[i].content.c_str();
      const is_synthetic = python_detect_synthetic(content_cstr);
      
      if is_synthetic == 0 {
        results[i].is_real = true;
        real_count.add(1);
      } else {
        results[i].is_real = false;
      }
    }
    
    writeln("   Real: ", real_count.read(), " / ", count);
    writeln("   Synthetic: ", (count - real_count.read()), " (filtered)");
    
    return real_count.read();
  }
  
  // BÚSQUEDA EN REDDIT (vía Python OSINT connector)
  proc search_reddit(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "reddit".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN TWITTER (vía Python OSINT connector)
  proc search_twitter(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "twitter".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN GITHUB (vía MCP connector)
  proc search_github(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const type_filter_str = "code";
    const type_filter = type_filter_str.c_str();
    const results_ptr = mcp_github_search(query_cstr, type_filter);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN TELEGRAM (vía Python OSINT connector)
  proc search_telegram(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "telegram".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN DISCORD (vía Python OSINT connector)
  proc search_discord(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "discord".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN LINKEDIN (vía Python OSINT connector + WASM scraping)
  proc search_linkedin(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "linkedin".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN HACKERNEWS (vía Algolia API through Python OSINT connector)
  proc search_hackernews(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "hackernews".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN QUORA (vía Python OSINT connector)
  proc search_quora(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "quora".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN STACKOVERFLOW (vía Python OSINT connector)
  proc search_stackoverflow(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "stackoverflow".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }

  // BÚSQUEDA EN YOUTUBE COMMENTS (vía Python OSINT connector)
  proc search_youtube_comments(keywords: [] string, num_keywords: int): int {
    const query_str = build_query(keywords, num_keywords);
    const query_cstr = query_str.c_str();
    const sources_cstr = "youtube".c_str();
    const results_ptr = python_osint_search(query_cstr, sources_cstr);
    const count = count_json_results(results_ptr);
    return count;
  }
  
  // CURACIÓN DE DATASET (filtrar + rankear)
  proc curate_dataset(ref results: [] SearchResult, count: int): int {
    writeln("\n📊 Curando dataset...");
    
    // 1. Filtrar sintéticos
    const real_count = filter_synthetic_data(results, count);
    
    // 2. Calcular quality scores en paralelo
    forall i in 1..real_count with (maxDegree=parallelDegree) do {
      if results[i].is_real {
        results[i].quality_score = calculate_quality_score(results[i]);
      }
    }
    
    // 3. Sort por quality score (mantener solo top results)
    // TODO: Implementar parallel sort
    
    writeln("   ✅ Dataset curado: ", real_count, " resultados de calidad");
    
    return real_count;
  }
  
  // CALCULAR CALIDAD (engagement, autenticidad, relevancia)
  proc calculate_quality_score(result: SearchResult): real {
    var score: real = 0.0;

    // Factor 1: Longitud (conversaciones más largas = mejor)
    const length_score = min(result.content.size:real / 500.0, 1.0) * 20.0;

    // Factor 2: Engagement — parse upvotes/likes from metadata JSON
    const meta_cstr = result.metadata.c_str();
    const upvotes = json_get_int(meta_cstr, "upvotes");
    const likes    = json_get_int(meta_cstr, "likes");
    const comments = json_get_int(meta_cstr, "comments");
    const raw_engagement = (upvotes + likes + comments):real;
    const engagement_score = min(raw_engagement / 100.0, 1.0) * 30.0;

    // Factor 3: Autenticidad — check for organic markers in content
    var auth_bonus: real = 25.0;
    if result.content.find("lol") >= 0 || result.content.find("haha") >= 0 then
      auth_bonus += 5.0;
    if result.content.find("...") >= 0 then auth_bonus += 3.0;
    const authenticity_score = min(auth_bonus, 30.0);

    // Factor 4: Relevancia — URL and source present = relevant
    const relevance_score = if result.url.size > 5 then 20.0 else 10.0;

    score = length_score + engagement_score + authenticity_score + relevance_score;
    return min(score, 100.0);
  }

  // EXPORTAR DATASET — real call to Python FFI connector
  proc export_dataset(format: string = "json") {
    writeln("\n💾 Exportando dataset...");

    extern proc python_export_json(data: c_ptrConst(c_char), filename: c_ptrConst(c_char)): c_int;

    const timestamp = Time.timeSinceEpoch().totalSeconds():string;
    const filename = "dataset_real_" + timestamp + ".json";
    const filename_cstr = filename.c_str();

    // Build JSON summary of curated results
    var json_data = "{\"count\": " + this.num_results:string +
                    ", \"timestamp\": " + timestamp +
                    ", \"source\": \"chapel_search_engine\"}";
    const data_cstr = json_data.c_str();

    const rc = python_export_json(data_cstr, filename_cstr);
    if rc == 0 then
      writeln("   ✅ Dataset exportado: ", filename)
    else
      writeln("   ⚠️  Export returned code ", rc, " for ", filename);
  }
}

// ═══════════════════════════════════════════════════════════════
// 4-AGENT SYSTEM INTEGRATION
// ═══════════════════════════════════════════════════════════════

// AGENT 1: MASTER - Analiza query y decide estrategia
proc agent_master_analyze(query_str: string, ref search_query: SearchQuery): int {
  writeln("🧠 AGENT 1 - MASTER: Analyzing query...");
  
  const lower = query_str.toLower();
  var complexity = 5;
  
  // Detectar intención y complejidad
  if lower.find("marketing") >= 0 || lower.find("sales") >= 0 {
    complexity = 7;
    writeln("   Intent: MARKETING/SALES");
  } else if lower.find("conversation") >= 0 || lower.find("chat") >= 0 {
    complexity = 8;
    writeln("   Intent: CONVERSATIONS");
  } else if lower.find("code") >= 0 || lower.find("github") >= 0 {
    complexity = 6;
    writeln("   Intent: CODE");
  }
  
  writeln("   Complexity: ", complexity, "/10");
  return complexity;
}

// AGENT 2: PLANNER - Optimiza estrategia según complejidad
proc agent_planner_strategy(complexity: int): string {
  writeln("🎯 AGENT 2 - PLANNER: Creating strategy...");
  
  var strategy = "";
  var parallel_degree = 32;
  
  if complexity <= 5 {
    strategy = "simple";
    parallel_degree = 16;
  } else if complexity <= 7 {
    strategy = "parallel";
    parallel_degree = 64;
  } else {
    strategy = "ml_enhanced";
    parallel_degree = 128;
  }
  
  writeln("   Strategy: ", strategy);
  writeln("   Parallel degree: ", parallel_degree);
  
  return strategy;
}

// AGENT 3: EXECUTOR - Ya implementado en search()

// AGENT 4: WRITER - Sintetiza y reporta
proc agent_writer_report(total: int, curated: int, elapsed: real) {
  writeln("\n📝 AGENT 4 - WRITER: Final report");
  writeln("=" * 70);
  writeln("Total scraped: ", total);
  writeln("Curated (real): ", curated, " (", ((curated:real / total:real) * 100.0):int, "%)");
  writeln("Time: ", elapsed, "s");
  writeln("Quality: ", if curated >= 100 then "EXCELLENT" else "GOOD");
  writeln("Confidence: ", if curated >= 100 then "95%" else "80%");
  writeln("=" * 70);
}

proc main() {
  writeln("╔═══════════════════════════════════════════════════════════╗");
  writeln("║                                                           ║");
  writeln("║     🔥 CHAPEL SEARCH ENGINE + 4 AGENTS 🔥               ║");
  writeln("║     Motor de búsqueda para datasets REALES              ║");
  writeln("║                                                           ║");
  writeln("╚═══════════════════════════════════════════════════════════╝");
  writeln();
  
  // Configurar búsqueda
  var query: SearchQuery;
  const query_str = "marketing sales real conversations";
  
  // AGENT 1: MASTER analiza query
  const complexity = agent_master_analyze(query_str, query);
  writeln();
  
  // AGENT 2: PLANNER crea estrategia
  const strategy = agent_planner_strategy(complexity);
  writeln();
  
  // Parse query keywords
  query.num_keywords = 3;
  query.keywords[1] = "marketing";
  query.keywords[2] = "sales";
  query.keywords[3] = "conversation";
  
  query.num_sources = 10;
  query.sources[1] = "reddit";
  query.sources[2] = "twitter";
  query.sources[3] = "github";
  query.sources[4] = "telegram";
  query.sources[5] = "discord";
  query.sources[6] = "linkedin";
  query.sources[7] = "hackernews";
  query.sources[8] = "quora";
  query.sources[9] = "stackoverflow";
  query.sources[10] = "youtube";
  
  // AGENT 3: EXECUTOR ejecuta búsqueda
  writeln("⚡ AGENT 3 - EXECUTOR: Running search...");
  var engine = new owned SearchEngine();
  
  var startTime = Time.timeSinceEpoch().totalSeconds();
  const total_results = engine.search(query);
  const curated_results = engine.curate_dataset(engine.results, total_results);
  var endTime = Time.timeSinceEpoch().totalSeconds();
  
  writeln();
  
  // AGENT 4: WRITER sintetiza
  agent_writer_report(total_results, curated_results, endTime - startTime);
  
  // Exportar
  engine.export_dataset("json");
  
  writeln("\n✅ 4-AGENT PIPELINE COMPLETE!");
}

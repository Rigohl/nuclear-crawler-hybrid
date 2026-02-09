# MCP TOOLS SPECIFICATION

## Overview

**7 Production-Grade Tools** exposed via JSON-RPC 2.0 HTTP API (Port 8079)

Each tool is **independent, composable, and can be invoked solo or in sequence**.

---

## TOOL 1: WEBSEARCH

**Massive web search across 55+ engines with 50K goroutine parallelism**

### Endpoint
```
POST /tools/websearch
```

### Request
```json
{
  "query": "string",
  "max_results": 1000,
  "engines": ["google", "bing", "duckduckgo"],
  "deep_search": false,
  "timeout_seconds": 30
}
```

### Response
```json
{
  "status": "success",
  "query": "string",
  "total_results": 150,
  "results": [
    {
      "title": "Result title",
      "url": "https://...",
      "snippet": "Result description",
      "rank_score": 0.95,
      "source": "google"
    }
  ],
  "metadata": {
    "search_time_ms": 1250,
    "engines_queried": 55,
    "parsing_method": "nim_ffi",
    "acceleration": "100x"
  }
}
```

### Internals
- Go FFI: 50,000 goroutines querying 55 engines in parallel
- Nim FFI: HTML parsing 100-200x faster than Rust regex
- Tantivy: Full-text indexing and ranking
- WASM: Additional result filtering/ranking (50x)
- Proxy rotation: Anonymous querying

---

## TOOL 2: PREMIUM_CONTENT

**Extract content from paywalled sites + Lateral Movement exploitation (21 techniques)**

### Endpoint
```
POST /tools/premium_content
```

### Request
```json
{
  "url": "https://paywall-site.com/article",
  "method": "auto",
  "auth_hints": "credentials_partial",
  "timeout_seconds": 30,
  "lateral_movement": {
    "enabled": true,
    "operation": "pass_the_hash_psexec",
    "targets": ["192.168.1.10"],
    "credentials": {
      "username": "Administrator",
      "hash": "NTLM_HASH"
    }
  }
}
```

### Response
```json
{
  "status": "success",
  "url": "https://...",
  "content": "Full article text (up to 100KB)",
  "title": "Article title",
  "author": "Author name",
  "publication_date": "2026-02-09T10:00:00Z",
  "access_method": "lateral_movement_psexec",
  "quality_score": 0.98,
  "lateral_movement_results": [
    {
      "target": "192.168.1.10",
      "status": "success",
      "output": "DOMAIN\\Administrator",
      "technique": "psexec_smb",
      "execution_time_ms": 1200
    }
  ],
  "metadata": {
    "extraction_time_ms": 3500,
    "content_length": 45000,
    "encoding": "utf-8",
    "lateral_movement_techniques_used": 1,
    "parallelism": "50K_goroutines"
  }
}
```

### Access Methods (10)
1. Direct HTTP (no protection)
2. CloudFlare bypass (Chrome Headless)
3. **Credential Theft & Reuse** (LSASS)
4. **Pass-the-Hash (SMB)**
5. **Pass-the-Ticket (Kerberos)**
6. **WinRM Exploitation** (RCE)
7. **Lateral Movement Intelligence** (21 techniques)
8. **Service Account Impersonation**
9. **Domain Controller Compromise** (NTDS.dit)
10. **NTLM Relay Attack** (MitM)

### 21 Lateral Movement Techniques (Integrated)
1. Pass-the-Hash (WMIExec)
2. Pass-the-Hash (PsExec)
3. Pass-the-Ticket (Kerberos)
4. Credential Theft & Reuse
5. SMB Exploitation
6. WinRM Exploitation (RCE)
7. RDP Hijacking
8. Service Account Impersonation
9. Domain Controller Compromise (NTDS.dit)
10. NTLM Relay Attack
11. Golden Ticket (Kerberos)
12. DCSync Attack
13. Kerberoasting
14. RBCD Attack
15. Unconstrained Delegation
16. Shadow Credentials
17. Zerologon (CVE-2020-1472)
18. PrinterSpooler (CVE-2021-1675)
19. Coercion + Relay
20. Advanced Relay (multi-hop)
21. Network Enumeration (50K parallel)

### Internals
- **Lateral Movement Engine** (21 techniques)
- **Credential Management** (theft, reuse, generation)
- **Network Exploitation** (SMB, WMI, RDP, SSH, Kerberos)
- **Chrome Headless** (JavaScript execution, CloudFlare)
- **Go FFI**: 50,000 goroutines for parallel probes
- **PowerShell**: Real command execution
- **Chapel AI**: Strategy selection (pre/post)
- **Nim FFI**: 100-200x HTML parsing speedup

---

## TOOL 3: SCAN_WORKSPACE

**Analyze files/folders: detect errors, warnings, vulnerabilities, suggest fixes**

### Endpoint
```
POST /tools/scan_workspace
```

### Request
```json
{
  "path": "/home/user/project",
  "language": "rust",
  "depth": 5,
  "include_suggestions": true,
  "check_vulnerabilities": true
}
```

### Response
```json
{
  "status": "success",
  "path": "/home/user/project",
  "summary": {
    "total_files": 127,
    "errors": 8,
    "warnings": 23,
    "vulnerable_libs": 3,
    "todo_items": 12
  },
  "issues": [
    {
      "file": "src/main.rs",
      "line": 42,
      "column": 15,
      "type": "error",
      "message": "Missing error handling",
      "severity": "critical",
      "suggestion": "Add Result return type and handle error case"
    }
  ],
  "vulnerable_dependencies": [
    {
      "name": "dependency_name",
      "version": "1.2.3",
      "vulnerability": "CVE-2024-XXXXX",
      "fix": "upgrade to 1.2.5"
    }
  ],
  "metadata": {
    "scan_time_ms": 4200,
    "files_scanned": 127,
    "detection_method": "static_analysis"
  }
}
```

### Detection Methods
- Static code analysis (errors/warnings)
- CVE database matching
- ML vulnerability detection
- Library security scanning
- Pattern-based issue detection

### Internals
- Zig SIMD: 256-bit vectorization for pattern analysis
- Code analyzer module
- CVE database integration
- Machine learning models (Chapel AI)
- Internet search for remediation suggestions

---

## TOOL 4: AI_DATASET_TRAINER

**Generate 5K-10K diversified datasets for ML model testing**

### Endpoint
```
POST /tools/ai_dataset_trainer
```

### Request
```json
{
  "topic": "lateral_movement",
  "sample_count": 7500,
  "diversity": 0.95,
  "include_edge_cases": true,
  "language": "python"
}
```

### Response
```json
{
  "status": "success",
  "topic": "lateral_movement",
  "dataset": {
    "format": "jsonl",
    "sample_count": 7500,
    "fields": ["id", "input", "output", "difficulty", "edge_case"],
    "samples": [
      {
        "id": 1,
        "input": "Target: 192.168.1.10, Method: SMB",
        "output": "Execute PsExec with hash credentials",
        "difficulty": "easy",
        "edge_case": false
      }
    ]
  },
  "statistics": {
    "total_variations": 7500,
    "difficulty_distribution": {
      "easy": 2500,
      "medium": 3500,
      "hard": 1500
    },
    "average_complexity": 0.72
  },
  "metadata": {
    "generation_time_ms": 8900,
    "model": "chapel_ai",
    "diversity_achieved": 0.94,
    "ready_for_training": true
  }
}
```

### Dataset Topics
- lateral_movement (exploitation techniques)
- web_scraping (data extraction)
- vulnerability_detection (security issues)
- code_generation (programming tasks)
- Any custom topic

### Internals
- Chapel AI: Intelligent dataset generation
- WASM: Dataset extraction (50x speedup)
- Multi-language support
- Edge case synthesis
- Automatic diversification

---

## TOOL 5: FILE_SEARCH

**Precision keyword search with exact line:column location and error detection**

### Endpoint
```
POST /tools/file_search
```

### Request
```json
{
  "path": "/home/user/project",
  "keyword": "unwrap()",
  "pattern_type": "exact",
  "recursive": true,
  "max_results": 1000
}
```

### Response
```json
{
  "status": "success",
  "keyword": "unwrap()",
  "total_matches": 47,
  "matches": [
    {
      "file": "src/main.rs",
      "line": 42,
      "column": 15,
      "content": "    let result = operation().unwrap();",
      "context": "let x = 41;\n    let result = operation().unwrap();\n    println!(\"{}\", result);",
      "match_type": "exact",
      "severity": "high"
    }
  ],
  "file_summary": [
    {
      "file_path": "src/main.rs",
      "matches_in_file": 8,
      "language": "rust"
    }
  ],
  "metadata": {
    "search_time_ms": 250,
    "files_searched": 127,
    "acceleration": "simd_256bit"
  }
}
```

### Pattern Types
- `exact` - Exact string match
- `regex` - Regular expression
- `error_pattern` - Detect error patterns (unwrap, panic, etc.)
- `warning_pattern` - Detect warning patterns
- `vulnerability_pattern` - Detect security issues

### Internals
- Zig SIMD: 256-bit vectorization
- WASM: 80x speedup
- Regex engine
- Error/warning detection heuristics
- Parallel file processing (50K goroutines Go)

---

## TOOL 7: INTELLIGENCE_OSINT

**Open Source Intelligence analysis: public information gathering & analysis**

### Endpoint
```
POST /tools/intelligence_osint
```

### Request
```json
{
  "target": "example.com",
  "target_type": "domain",
  "depth": 3,
  "sources": ["whois", "dns", "website", "social_media"]
}
```

### Response
```json
{
  "status": "success",
  "target": "example.com",
  "target_type": "domain",
  "intelligence": {
    "domain_info": {
      "registrant": "Organization Name",
      "registrar": "Registrar Inc",
      "nameservers": ["ns1.example.com"],
      "ip_addresses": ["192.0.2.1"],
      "created_date": "2010-02-08T00:31:39Z",
      "expiration_date": "2027-02-08T00:31:39Z"
    },
    "website_analysis": {
      "title": "Example Domain",
      "description": "Metadata description",
      "technologies": ["Apache", "PHP", "WordPress"],
      "emails_found": ["admin@example.com"],
      "links": ["https://..."]
    },
    "ip_intel": {
      "country": "US",
      "isp": "ISP Name",
      "hostname": "host.example.com",
      "services": ["HTTP", "HTTPS", "SSH"],
      "open_ports": [22, 80, 443]
    }
  },
  "analysis": {
    "risk_score": 3.5,
    "threat_level": "low",
    "correlations": ["Associated domain: related.com"],
    "relationships": ["Host: 192.0.2.2"]
  },
  "metadata": {
    "intelligence_time_ms": 2500,
    "sources_queried": 4,
    "legal_notice": "Public information only - OSINT compliant"
  }
}
```

### Information Gathering Sources
- **WHOIS** - Domain registrant, dates, nameservers
- **DNS** - Subdomains, MX records, A records, TXT records
- **Website Analysis** - Title, metadata, technologies, emails, links
- **IP Reputation** - Country, ISP, hostname, services, open ports
- **Public Records** - Company info, incorporation dates, filings
- **Search Engines** - Cached pages, historical data

### Analysis Methods
- **Bayesian Analysis** - Correlation of information
- **Game Theory** - Relationship mapping
- **Neural Networks** - Pattern detection
- **Risk Scoring** - Threat assessment

### Legal & Compliance
- ✅ **OSINT** - Public information only (LEGAL)
- ❌ **DOXING** - Private information (ILLEGAL) - NOT INCLUDED
- ✅ **Open Source** - Published data, registries, public APIs
- ⚠️ **Responsible** - Respects robots.txt, rate limits, ToS

### Internals
- **WebSearch** - 55+ engines for information gathering
- **WHOIS Queries** - Domain registration data
- **DNS Enumeration** - Subdomain discovery
- **Website Scraping** - Public data extraction
- **IP Databases** - Geolocation, ISP info
- **Bayesian Networks** - Correlation analysis
- **Game Theory** - Relationship analysis
- **Neural Networks** - Pattern recognition

**ML-powered code analysis: vulnerability detection, fix suggestions**

### Endpoint
```
POST /tools/code_intelligence
```

### Request
```json
{
  "code": "let x = file.read().unwrap();",
  "language": "rust",
  "analysis_type": "security",
  "include_fixes": true
}
```

### Response
```json
{
  "status": "success",
  "code": "let x = file.read().unwrap();",
  "language": "rust",
  "issues": [
    {
      "type": "vulnerability",
      "severity": "high",
      "message": "Unhandled error: unwrap() will panic on failure",
      "vulnerability_class": "error_handling",
      "fix": "Use Result return or match for error handling"
    }
  ],
  "ml_insights": {
    "code_smell_score": 0.82,
    "vulnerability_probability": 0.95,
    "suggested_patterns": [
      "Use ? operator for error propagation",
      "Implement Result-based error handling"
    ]
  },
  "metadata": {
    "analysis_time_ms": 150,
    "model": "chapel_ai",
    "confidence": 0.98
  }
}
```

### Analysis Types
- `security` - Security vulnerability detection
- `performance` - Performance issues
- `quality` - Code quality issues
- `all` - All analysis types

### Internals
- ML model (Chapel AI)
- Static analysis
- Pattern recognition
- Vulnerability database
- Remediation suggestions
- Zig SIMD: Code vectorization
- JAX GPU: ML inference acceleration

---

## Tool Invocation Patterns

### Single Tool
```bash
curl -X POST http://localhost:8079/tools/websearch \
  -d '{"query":"exploit techniques"}'
```

### Sequential Chain
```bash
# 1. Scan workspace
# 2. Find vulnerabilities
# 3. Generate exploit dataset
# 4. Test exploits
```

### Parallel Execution
```bash
# All 7 tools can run simultaneously
# Each has independent state/data
```

---

**Specification Version**: 2.0  
**Last Updated**: 9 de febrero de 2026

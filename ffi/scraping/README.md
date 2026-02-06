# FFI Scraping Libraries - Powerful Web Scraping Tools

This directory contains Foreign Function Interface (FFI) bindings for powerful web scraping libraries.

## 🔥 Integrated Libraries

### 1. **BeautifulSoup4 (Python)** - HTML/XML Parsing
- **Language**: Python
- **Power**: Industry standard for HTML/XML parsing
- **Features**:
  - Navigable tree structure
  - CSS selectors
  - Handles broken HTML
  - Unicode support
- **Usage**: Via Python FFI (PyO3 or python-launcher)

### 2. **Scrapy (Python)** - Web Scraping Framework
- **Language**: Python
- **Power**: Complete scraping framework with middleware
- **Features**:
  - Async crawling (Twisted)
  - Built-in pipelines
  - Item extraction
  - Middleware support
  - Robots.txt handling
- **Usage**: Via Python FFI (PyO3)

### 3. **LXML (Python/C)** - Fast XML/HTML Parsing
- **Language**: Python (with C backend via libxml2)
- **Power**: Fastest Python HTML/XML parser
- **Features**:
  - XPath 1.0/2.0 support
  - XSLT transformations
  - C-level speed
  - Memory efficient
- **Usage**: Via Python FFI or direct C bindings

### 4. **Playwright (Node.js/Python)** - Browser Automation
- **Language**: Node.js/Python
- **Power**: Modern browser automation (Chromium, Firefox, WebKit)
- **Features**:
  - Headless/headful mode
  - Mobile emulation
  - Network interception
  - Auto-wait for elements
  - Screenshots/PDFs
- **Usage**: Via Node.js FFI or Python FFI

### 5. **Selenium (Python/Java)** - Classic Browser Control
- **Language**: Python/Java
- **Power**: Industry standard for browser automation
- **Features**:
  - Multi-browser support
  - Grid support
  - Page Object Model
  - Waits and assertions
- **Usage**: Via Python FFI (PyO3)

### 6. **Puppeteer (Node.js)** - Chrome DevTools Protocol
- **Language**: Node.js
- **Power**: Direct Chrome/Chromium control
- **Features**:
  - Fast page loading
  - Network control
  - Performance profiling
  - Lighthouse integration
- **Usage**: Via Node.js FFI

## 🚀 Integration Strategy

### Rust → Python FFI (PyO3)
```rust
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn scrape_with_beautifulsoup(html: &str, selector: &str) -> PyResult<Vec<String>> {
    Python::with_gil(|py| {
        let bs4 = PyModule::import(py, "bs4")?;
        let soup = bs4.getattr("BeautifulSoup")?.call1((html, "html.parser"))?;
        let elements = soup.call_method1("select", (selector,))?;
        
        // Extract text from elements
        let mut results = Vec::new();
        for elem in elements.iter()? {
            let elem = elem?;
            let text: String = elem.call_method0("get_text")?.extract()?;
            results.push(text);
        }
        Ok(results)
    })
}
```

### Rust → Node.js FFI (neon)
```rust
use neon::prelude::*;

pub fn scrape_with_playwright(url: &str, selector: &str) -> Vec<String> {
    // Via neon bindings
    // This would call Playwright through Node.js
}
```

## 📦 Dependencies (Cargo.toml)

```toml
[dependencies]
# Python FFI
pyo3 = { version = "0.21", features = ["auto-initialize"] }

# Node.js FFI
neon = "1.0"

# Alternative: Process-based FFI
tokio = { version = "1.0", features = ["process"] }
```

## 🛠️ Build Instructions

1. **Install Python libraries**:
   ```bash
   pip install beautifulsoup4 lxml scrapy playwright selenium
   playwright install chromium
   ```

2. **Install Node.js libraries**:
   ```bash
   npm install playwright puppeteer
   ```

3. **Build Rust with FFI**:
   ```bash
   cargo build --release --features python_ffi,nodejs_ffi
   ```

## 🔒 Security Notes

- Always sanitize inputs when passing to FFI
- Use sandboxing for untrusted content
- Validate URLs before scraping
- Rate limit to avoid abuse
- Respect robots.txt

## 📊 Performance Comparison

| Library | Speed | Memory | JavaScript Support | Anti-Detection |
|---------|-------|--------|-------------------|----------------|
| LXML | ★★★★★ | ★★★★★ | ❌ | ❌ |
| BeautifulSoup | ★★★☆☆ | ★★★★☆ | ❌ | ❌ |
| Scrapy | ★★★★☆ | ★★★★☆ | ❌ | ★★★☆☆ |
| Playwright | ★★★★☆ | ★★☆☆☆ | ✅ | ★★★★★ |
| Puppeteer | ★★★★☆ | ★★☆☆☆ | ✅ | ★★★★☆ |
| Selenium | ★★★☆☆ | ★★☆☆☆ | ✅ | ★★★☆☆ |

## 🎯 Recommended Use Cases

- **Static HTML**: LXML (fastest)
- **Complex parsing**: BeautifulSoup4
- **Large-scale crawling**: Scrapy
- **JavaScript-heavy sites**: Playwright
- **Anti-bot bypass**: Playwright with stealth
- **Cross-browser testing**: Selenium

# Nuclear Crawler Hybrid - Codespace Configuration

This folder contains the GitHub Codespaces configuration for the Nuclear Crawler Hybrid project.

## 🚀 Quick Start

### 1. Create Codespace

Go to: https://github.com/yourusername/nuclear-crawler-hybrid

Click: `Code` → `Codespaces` → `Create codespace on main`

The environment will initialize automatically with:
- **Chapel 2.7** with FFI support
- **Python 3.11** with JAX, PyTorch, TensorFlow
- **Rust** with cargo and development tools
- **Go**  latest version
- **Node.js 20** with Puppeteer
- **Development Extensions** for VS Code

### 2. Verify Installation

```bash
# Run validation script
/workspace/validate-environment.sh

# Or manually check:
chpl --version
python --version
cargo --version
rustc --version
go version
node --version
```

### 3. Test FFI

```bash
# Chapel example
cd /workspace/chapel-examples
./hello

# Test Rust FFI
cargo build --release -C /workspace/ffi

# Test Python integration
python /workspace/ffi/python_ffi.py
```

## 📁 Project Structure

```
.devcontainer/
├── devcontainer.json      # Main Codespace configuration
├── setup.sh               # Installation and setup script
└── README.md             # This file

/workspace/
├── src/                   # Source code
├── chapel-examples/       # Chapel test programs
├── ffi/                   # FFI examples (Python, Rust, Go, C)
├── data/                  # Data directory
├── output/                # Output directory
├── config/                # Configuration files
└── validate-environment.sh # Environment validation script
```

## 🔗 FFI Capabilities

Nuclear Crawler Hybrid supports multi-language FFI:

### Python ↔ Chapel
```chapel
extern proc python_add(a: c_int, b: c_int): c_int;
```

### Rust ↔ Chapel
```chapel
extern proc rust_multiply(a: c_int, b: c_int): c_int;
```

### Go ↔ Chapel
Chapel can call Go via C interfaces.

### C ↔ Chapel
Direct C interoperability with Chapel.

## 🛠️ Available Tools

### Chapel
- Compiler: `chpl`
- Documentation: https://chapel-lang.org/docs/
- CHPL_HOME: `/opt/chapel`

### Python Packages
- **ML/AI**: JAX, PyTorch, TensorFlow, scikit-learn
- **Web**: Playwright, BeautifulSoup4, Selenium, Scrapy
- **Data**: Pandas, NumPy, SciPy, Polars
- **Tools**: Jupyter, IPython, pytest

### Rust
- Toolchain: Latest stable
- Extras: clippy, rustfmt, rust-analyzer
- Cargo watch available

### Go  
- Latest version (1.21+)
- Fiber web framework pre-installed

### Node.js
- Version: 20 LTS
- Puppeteer and chrome automation tools

## ⚙️ Customization

To modify the environment:

1. Edit `devcontainer.json`:
   - Add/remove features
   - Add/remove VS Code extensions
   - Change forwarded ports
   - Modify environment variables

2. Edit `setup.sh`:
   - Add/remove package installations
   - Modify Python dependencies
   - Update Chapel version
   - Customize post-install setup

3. Rebuild Codespace:
   - Click the Codespace menu
   - Select "Rebuild container"

## 📋 Environment Variables

| Variable | Value | Purpose |
|----------|-------|---------|
| `CHPL_HOME` | `/opt/chapel` | Chapel installation directory |
| `CHAPEL_HOME` | `/opt/chapel` | Alternative Chapel path |
| `NUCLEAR_CRAWLER_MODE` | `hybrid` | Crawler operating mode |
| `CRAWLER_WORKERS` | `8` | Number of worker threads |
| `ENABLE_FFI` | `true` | Enable FFI functionality |

## 🔌 Forwarded Ports

| Port | Service | Purpose |
|------|---------|---------|
| 3000 | Web App | Frontend application |
| 3001 | API | Backend API |
| 8000 | Server | Alternative server |
| 8080 | HTTP | Web server |
| 9000 | Custom | Custom service |
| 5432 | PostgreSQL | Database |
| 6379 | Redis | Cache/Session store |

## 📚 Documentation

- **Chapel Guide**: `/workspace/src/README.md`
- **FFI Examples**: `/workspace/ffi/`
- **Chapel Examples**: `/workspace/chapel-examples/`
- **Configuration**: `/workspace/config/crawler.toml`

## ⚡ Performance Tips

1. Use Chapel's parallelism:
   ```chapel
   forall i in 1..n do {
     // Parallel computation
   }
   ```

2. Enable Rust optimizations:
   ```toml
   [profile.release]
   opt-level = 3
   lto = true
   ```

3. Leverage Python NumPy vectorization for data processing

4. Use Go goroutines for concurrent I/O

## 🐛 Troubleshooting

### Chapel compilation errors
- Ensure `CHPL_HOME` is set correctly
- Check for syntax errors with `chpl --check`
- View detailed errors with `CHPL_DEVELOPER=1`

### FFI linkage errors
- Verify all dependencies are installed
- Check library paths in environment
- Use `ldd` to verify shared library linking

### Playwright browser issues
- Run: `playwright install chromium --with-deps`
- Check browser cache

### Port access issues
- VS Code automatically forwards configured ports
- Stop other containers using the same ports

## 🚀 Deployment

To deploy from Codespace:

1. Build all components:
   ```bash
   chpl src/main.chpl -o main
   cargo build --release
   ```

2. Package Docker image:
   ```bash
   docker build -t nuclear-crawler .
   ```

3. Push to registry or deploy to production

## 📞 Support

For issues:
1. Check the troubleshooting section
2. Review Chapel documentation at chapel-lang.org
3. Check project issues on GitHub

---

**Happy coding! 🚀**

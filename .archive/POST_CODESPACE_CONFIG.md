# 🎯 Post-Codespace Configuration Guide

After your Codespace is created and running, follow these steps to get fully operational:

## ✅ Immediate Actions (First 5 Minutes)

### 1. Validate Environment
```bash
bash validate-environment.sh
```

You should see:
```
✅ ALL SYSTEMS GO!
Summary:
✅ Passed: 45+
⚠️  Warnings: 0-2
```

### 2. Install Python Dependencies
```bash
make install-deps
# Or manually:
pip3 install -r requirements.txt
```

### 3. Quick Test
```bash
make quick
# Select option "3" to test JAX
```

## 📦 Environment Setup

### Chapel Configuration
The Chapel compiler is installed at `/opt/chapel`. To use it:
```bash
export CHPL_HOME=/opt/chapel
export PATH=$CHPL_HOME/bin/linux64-x86_64:$PATH
chpl --version
```

This is already in your `.bashrc` or `.bash_profile`, so it should work automatically.

### Python Virtual Environment (Optional but Recommended)
```bash
# Create virtual environment
python3 -m venv venv

# Activate
source venv/bin/activate

# Install dependencies
pip3 install -r requirements.txt

# Deactivate later
deactivate
```

### Rust Setup
Rust is pre-installed with:
- Rust compiler (cargo)
- Clippy (linter)
- Rustfmt (formatter)

Test it:
```bash
cargo --version
# Output: cargo 1.x.x
```

## 🧪 Run Your First Example

### Chapel Example
```bash
bash ffi-examples.sh  # Generates examples
cd ffi-examples/chapel
chpl hello.chpl -o hello
./hello
```

Expected output:
```
Chapel FFI Examples
Fibonacci(10) = 55
Matrix Multiply result:
30 24 18
84 69 54
138 114 90
```

### Python + JAX Example
```bash
python3 << 'EOF'
import jax
import jax.numpy as jnp

# Create array
x = jnp.array([1.0, 2.0, 3.0])

# Define function
def f(x):
    return jnp.sin(x) * jnp.exp(-x)

# Compute and gradient
result = f(x)
grad = jax.grad(f)(1.0)

print(f"JAX result: {result}")
print(f"Gradient: {grad}")
EOF
```

### Rust FFI Example
```bash
cd ffi-examples/rust
cargo build --release
# Compiled library available at: target/release/libchapel_ffi.so
```

### Go Example
```bash
cd ffi-examples/go
go run main.go
```

### Java Example
```bash
cd ffi-examples/java
javac ChapelFFI.java
java ChapelFFI
```

## 🛠️ Development Workflow

### Using Make Commands
```bash
make build          # Build all components
make test           # Run all tests
make format         # Format code
make lint           # Check code quality
make clean          # Clean up artifacts
make jupyter        # Start Jupyter Lab
```

### Development Tools

**VS Code Extensions Already Installed:**
- ✅ Python
- ✅ Rust
- ✅ C/C++
- ✅ Go
- ✅ Java
- ✅ Docker
- ✅ Jupyter
- ✅ Git Graph
- ✅ Prettier
- ✅ ES Lint
- ✅ Thunder Client (API testing)

**Example: Development Loop**
```bash
# 1. Edit your code
vim src/main.py

# 2. Format
make format

# 3. Test
make test

# 4. Lint
make lint

# 5. Run
make run
```

## 🔗 Multi-Language FFI Patterns

### Python Calling Chapel
```python
# Python code calls Chapel via ctypes or cffi
from ctypes import CDLL, c_int

lib = CDLL('./chapel_lib.so')
result = lib.fibonacci(c_int(10))
print(result)  # 55
```

### Rust Calling Chapel
```rust
// Rust FFI declaration
extern "C" {
    fn chapel_function(n: c_int) -> c_int;
}

// Call from Rust
unsafe {
    let result = chapel_function(10);
}
```

### Go Calling C Libraries
```go
// #cgo LDFLAGS: -lm
// #include <math.h>
import "C"

// Use C function
result := C.sqrt(16.0)
```

## 📊 Jupyter Notebooks

Start Jupyter Lab:
```bash
make jupyter
# Or manually:
jupyter lab --allow-root --ip=0.0.0.0
```

Access at: `http://localhost:8888` (token will be shown)

### Create Notebook
```bash
# The interface creates new notebooks via Jupyter GUI
# Or create manually:
jupyter notebook --generate-config
# Then use web interface
```

## 🗄️ Database Connections

**PostgreSQL** (if needed):
```bash
# Client installed
psql --version

# Connect to local instance (if running)
psql -h localhost -U postgres
```

**MongoDB** (if needed):
```bash
# Python client installed
python3 -c "import pymongo; print(pymongo.__version__)"
```

**Redis** (if needed):
```bash
# Python client installed
python3 -c "import redis; print(redis.__version__)"
```

## 🚀 Performance Tips

### Compiled Chapel
Chapel is slow in interpreted mode. Compile for production:
```bash
# Debug (helpful for development)
chpl --devel program.chpl -o program

# Production (much faster)
chpl --fast program.chpl -o program
```

### Python + JAX
JAX is optimized for GPU, but Codespace CPU is fine for learning:
```bash
# Check available devices
python3 -c "import jax; print(jax.devices())"
```

### Rust Optimization
```bash
# Debug build (fast compile, slow run)
cargo build

# Release build (slow compile, fast run)
cargo build --release
```

## 🔒 Save Your Work

### Git Commits
```bash
git add .
git commit -m "Progress checkpoint"
git push origin main
```

### Manual Backup
```bash
# Download entire /workspace
# Via Codespace interface: Explorer → right-click folder → Download
```

## 🆘 Troubleshooting

### Chapel not found
```bash
export CHPL_HOME=/opt/chapel
export PATH=$CHPL_HOME/bin/linux64-x86_64:$PATH
```

### Python package import errors
```bash
pip3 install --upgrade package_name
pip3 install -r requirements.txt
```

### Out of memory during compilation
```bash
# Use optimized Chapel compiler
chpl --optimized program.chpl -o program
```

### Codespace pauses automatically
- After 30 minutes of inactivity
- Click terminal or editor to reactivate

## 📚 Useful Resources

- **Chapel Docs**: https://chapel-lang.org/docs/
- **JAX Docs**: https://jax.readthedocs.io/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Go Packages**: https://golang.org/pkg/
- **GitHub Codespaces**: https://docs.github.com/en/codespaces

## ❓ For Help

1. **Check existing docs**: `.devcontainer/README.md`, `Makefile`
2. **Run validation**: `bash validate-environment.sh`
3. **View logs**: `~/.bash_history` for recent commands
4. **Ask for help**: GitHub Issues or project documentation

---

**You're all set! Start developing! 🎉**

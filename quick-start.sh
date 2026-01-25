#!/bin/bash
# quick-start.sh - Quick start examples for Nuclear Crawler Hybrid

set -e

echo "🚀 Nuclear Crawler Hybrid - Quick Start"
echo ""
echo "Choose an option:"
echo ""
echo "1. Validate environment"
echo "2. Run Chapel example"
echo "3. Test Python JAX"
echo "4. Start Jupyter Lab"
echo "5. Build Rust FFI module"
echo "6. Run pytest tests"
echo "7. Open interactive shell"
echo "8. Show available commands"
echo ""

read -p "Enter choice (1-8): " choice

case $choice in
    1)
        echo ""
        echo "🔍 Validating environment..."
        bash /workspace/validate-environment.sh
        ;;
    2)
        echo ""
        echo "📝 Chapel Example: Hello Nuclear Crawler"
        mkdir -p /workspace/chapel-examples
        
        if [ ! -f /workspace/chapel-examples/hello.chpl ]; then
            cat > /workspace/chapel-examples/hello.chpl << 'CHAPEL'
// Hello Chapel - Nuclear Crawler Hybrid
module HelloCrawler {
  
  proc main() {
    writeln("Chapel is ready! Nuclear Crawler Hybrid initialized.");
    
    // Matrix computation example
    const N = 10;
    var A: [1..N, 1..N] real;
    
    // Fill matrix
    forall (i,j) in {1..N, 1..N} {
      A[i,j] = (i:real) * (j:real);
    }
    
    // Compute sum
    var sum = 0.0;
    forall (i,j) in {1..N, 1..N} with (+ reduce sum) {
      sum += A[i,j];
    }
    
    writeln("Matrix sum: ", sum);
    
    // FFI call result
    writeln("FFI call result: ", 0);
  }
}
CHAPEL
        fi
        
        cd /workspace/chapel-examples
        chpl hello.chpl -o hello
        ./hello
        ;;
    3)
        echo ""
        echo "🤖 Testing Python JAX"
        python3 << 'PYTHON'
import jax
import jax.numpy as jnp

print("JAX version:", jax.__version__)
print("")

# Create array
x = jnp.array([1.0, 2.0, 3.0])
print("Input array:", x)

# Define function
def f(x):
    return jnp.sin(x) * jnp.exp(-x)

# Compute
result = f(x)
print("Result f(x):", result)

# Compute gradient
grad_f = jax.grad(f)
gradient = grad_f(x[0])
print("Gradient at x[0]:", gradient)

print("")
print("✅ JAX is working correctly!")
PYTHON
        ;;
    4)
        echo ""
        echo "📊 Starting Jupyter Lab..."
        echo "Access at: http://localhost:8888"
        echo ""
        jupyter lab --allow-root --ip=0.0.0.0
        ;;
    5)
        echo ""
        echo "🦀 Building Rust FFI Module"
        if [ -d /workspace/ffi ]; then
            cd /workspace/ffi
            cargo build --release
            echo ""
            echo "✅ Rust FFI module built successfully!"
            ls -la target/release/ | grep -E "\.so|\.dylib|\.dll" || echo "No compiled modules found yet"
        else
            echo "❌ FFI directory not found at /workspace/ffi"
        fi
        ;;
    6)
        echo ""
        echo "🧪 Running Tests"
        cd /workspace
        
        if [ -d tests ]; then
            pytest -v tests/ --tb=short
        elif [ -d test ]; then
            pytest -v test/ --tb=short
        else
            echo "❌ No tests directory found"
            echo "Create one with: mkdir -p tests/"
        fi
        ;;
    7)
        echo ""
        echo "🐚 Interactive Shell"
        echo 'Type "exit" to quit'
        echo ""
        bash
        ;;
    8)
        echo ""
        echo "📋 Available Commands:"
        echo ""
        echo "Chapel:"
        echo "  chpl --version                    # Check Chapel version"
        echo "  chpl program.chpl -o program     # Compile Chapel"
        echo "  ./program                         # Run compiled program"
        echo ""
        echo "Python:"
        echo "  python3 --version                 # Check Python version"
        echo "  pip3 install package             # Install package"
        echo "  jupyter lab                       # Start Jupyter Lab"
        echo "  pytest tests/                     # Run tests"
        echo ""
        echo "Rust:"
        echo "  cargo --version                   # Check Cargo version"
        echo "  cargo build                       # Build project"
        echo "  cargo build --release             # Build optimized"
        echo "  cargo test                        # Run tests"
        echo ""
        echo "Go:"
        echo "  go version                        # Check Go version"
        echo "  go run main.go                    # Run Go program"
        echo "  go build                          # Build Go project"
        echo ""
        echo "Utilities:"
        echo "  docker ps                         # List running containers"
        echo "  make --version                    # Check Make version"
        echo "  git status                        # Check git status"
        echo "  validate-environment.sh           # Full environment check"
        echo ""
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

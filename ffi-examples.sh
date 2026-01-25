#!/bin/bash
# ffi-examples.sh - Generate FFI examples for Chapel, Python, Rust, Go, and Java

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FFI_DIR="${SCRIPT_DIR}/ffi-examples"

echo "🔗 Generating FFI Examples for Nuclear Crawler Hybrid"
echo ""

# Create directory structure
mkdir -p "$FFI_DIR"/{chapel,python,rust,go,java,lib}

echo "📝 Generating example files..."

# ===== Chapel Examples =====
cat > "$FFI_DIR/chapel/hello.chpl" << 'CHAPEL'
// Chapel FFI Example - Basic computation

module HelloFFI {
  
  proc fibonacci(n: int): int {
    if n <= 1 then return n;
    return fibonacci(n-1) + fibonacci(n-2);
  }
  
  proc matrixMultiply(A: [?I, ?J] real, B: [?J, ?K] real): [I, K] real {
    var C: [I, K] real = 0;
    forall (i,k) in {I, K} {
      var sum = 0.0;
      for j in J {
        sum += A[i,j] * B[j,k];
      }
      C[i,k] = sum;
    }
    return C;
  }
  
  proc main() {
    writeln("Chapel FFI Examples");
    writeln("-------------------");
    
    // Fibonacci
    var fib10 = fibonacci(10);
    writeln("Fibonacci(10) = ", fib10);
    
    // Matrix (3x3)
    const N = 3;
    var A: [1..N, 1..N] real = [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]
    ];
    
    var B: [1..N, 1..N] real = [
      [9, 8, 7],
      [6, 5, 4],
      [3, 2, 1]
    ];
    
    var C = matrixMultiply(A, B);
    writeln("Matrix Multiply result:");
    for i in 1..N {
      for j in 1..N {
        write(C[i,j], " ");
      }
      writeln();
    }
  }
}
CHAPEL

# ===== Python FFI Example =====
cat > "$FFI_DIR/python/ffi_chapel.py" << 'PYTHON'
"""
Python FFI Example - Call Chapel from Python
Requires: cffi, ctypes
"""

from cffi import FFI
import ctypes
import os

class ChapelBridge:
    """Bridge to call Chapel compiled code from Python"""
    
    def __init__(self, lib_path):
        """Load compiled Chapel library"""
        if os.path.exists(lib_path):
            self.lib = ctypes.CDLL(lib_path)
        else:
            print(f"Warning: Library not found at {lib_path}")
            self.lib = None
    
    def fibonacci(self, n: int) -> int:
        """Call Chapel fibonacci function"""
        if self.lib is None:
            # Python fallback
            if n <= 1:
                return n
            return self.fibonacci(n-1) + self.fibonacci(n-2)
        
        try:
            fib_func = self.lib.fibonacci
            fib_func.argtypes = [ctypes.c_int]
            fib_func.restype = ctypes.c_int
            return fib_func(n)
        except AttributeError:
            print("Warning: fibonacci function not found in library, using Python")
            return self.fibonacci(n)

def main():
    print("Python FFI Example")
    print("------------------")
    print()
    
    # Example 1: Python Fibonacci
    print("Python Fibonacci (no Chapel):")
    bridge = ChapelBridge("")
    
    for i in range(10):
        result = bridge.fibonacci(i)
        print(f"fib({i}) = {result}", end="  ")
        if (i + 1) % 3 == 0:
            print()
    print()
    print()
    
    # Example 2: Pure Python for data processing
    print("NumPy-based Matrix Operations:")
    try:
        import numpy as np
        
        A = np.array([[1, 2, 3],
                      [4, 5, 6],
                      [7, 8, 9]], dtype=float)
        
        B = np.array([[9, 8, 7],
                      [6, 5, 4],
                      [3, 2, 1]], dtype=float)
        
        C = A @ B
        print("Matrix product:")
        print(C)
    except ImportError:
        print("NumPy not available")

if __name__ == "__main__":
    main()
PYTHON

# ===== Rust FFI Example =====
mkdir -p "$FFI_DIR/rust"
cat > "$FFI_DIR/rust/Cargo.toml" << 'TOML'
[package]
name = "chapel_ffi"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"

[profile.release]
opt-level = 3
lto = true
TOML

cat > "$FFI_DIR/rust/src/lib.rs" << 'RUST'
// Rust FFI Example - Callable from Chapel, Python, Go

use std::os::raw::c_int;

/// Fibonacci in Rust (exported for FFI)
#[no_mangle]
pub extern "C" fn fibonacci_rust(n: c_int) -> c_int {
    if n <= 1 {
        n
    } else {
        fibonacci_rust(n - 1) + fibonacci_rust(n - 2)
    }
}

/// Matrix multiply helper
#[no_mangle]
pub extern "C" fn matrix_multiply(
    n: c_int,
    a: *const f64,
    b: *const f64,
    c: *mut f64,
) -> c_int {
    unsafe {
        let size = (n * n) as usize;
        let a_slice = std::slice::from_raw_parts(a, size);
        let b_slice = std::slice::from_raw_parts(b, size);
        let c_slice = std::slice::from_raw_parts_mut(c, size);
        
        for i in 0..n as usize {
            for j in 0..n as usize {
                let mut sum = 0.0;
                for k in 0..n as usize {
                    sum += a_slice[i * n as usize + k] * b_slice[k * n as usize + j];
                }
                c_slice[i * n as usize + j] = sum;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_rust(0), 0);
        assert_eq!(fibonacci_rust(1), 1);
        assert_eq!(fibonacci_rust(5), 5);
        assert_eq!(fibonacci_rust(10), 55);
    }
}
RUST

# ===== Go FFI Example =====
mkdir -p "$FFI_DIR/go"
cat > "$FFI_DIR/go/main.go" << 'GO'
package main

import (
	"fmt"
	"unsafe"
)

// #cgo LDFLAGS: -ldl
// #include <dlfcn.h>
// typedef int (*fib_func)(int);
import "C"

// Fibonacci in Go
func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

// Call C/Chapel functions
func fibonacci_native(n int) int {
	// In a real scenario, load compiled Chapel/Rust library
	return fibonacci(n)
}

// Matrix multiplication in Go
func matrixMultiply(n int, a, b [][]float64) [][]float64 {
	c := make([][]float64, n)
	for i := range c {
		c[i] = make([]float64, n)
		for j := range c[i] {
			for k := 0; k < n; k++ {
				c[i][j] += a[i][k] * b[k][j]
			}
		}
	}
	return c
}

func main() {
	fmt.Println("Go FFI Example")
	fmt.Println("--------------")
	fmt.Println()

	// Fibonacci
	fmt.Println("Fibonacci sequence:")
	for i := 0; i < 10; i++ {
		result := fibonacci(i)
		fmt.Printf("fib(%d) = %d ", i, result)
		if (i+1)%3 == 0 {
			fmt.Println()
		}
	}
	fmt.Println()
	fmt.Println()

	// Matrix multiply
	fmt.Println("Matrix multiplication:")
	a := [][]float64{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}
	b := [][]float64{
		{9, 8, 7},
		{6, 5, 4},
		{3, 2, 1},
	}

	c := matrixMultiply(3, a, b)
	for _, row := range c {
		for _, val := range row {
			fmt.Printf("%8.0f ", val)
		}
		fmt.Println()
	}
}
GO

# ===== Java FFI Example =====
mkdir -p "$FFI_DIR/java"
cat > "$FFI_DIR/java/ChapelFFI.java" << 'JAVA'
/**
 * Java FFI Example - Call Chapel/Rust/C from Java
 * Uses JNI (Java Native Interface)
 */

public class ChapelFFI {
    
    // Load native library
    static {
        try {
            System.loadLibrary("chapel_ffi");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("Native library not found: " + e);
        }
    }
    
    // Native methods (must be implemented in C/Rust)
    public native int fibonacci(int n);
    
    public native int matrixMultiply(int n, double[] a, double[] b, double[] c);
    
    // Pure Java implementation (fallback)
    public int fibonacciJava(int n) {
        if (n <= 1) return n;
        return fibonacciJava(n - 1) + fibonacciJava(n - 2);
    }
    
    public static void main(String[] args) {
        System.out.println("Java FFI Example");
        System.out.println("----------------");
        System.out.println();
        
        ChapelFFI ffi = new ChapelFFI();
        
        // Fibonacci
        System.out.println("Fibonacci sequence (Java):");
        for (int i = 0; i < 10; i++) {
            int result = ffi.fibonacciJava(i);
            System.out.printf("fib(%d) = %d ", i, result);
            if ((i + 1) % 3 == 0) {
                System.out.println();
            }
        }
        System.out.println();
        System.out.println();
        
        // Matrix multiply
        System.out.println("Matrix multiplication (Java):");
        int n = 3;
        double[] a = {
            1, 2, 3,
            4, 5, 6,
            7, 8, 9
        };
        double[] b = {
            9, 8, 7,
            6, 5, 4,
            3, 2, 1
        };
        double[] c = new double[n * n];
        
        ffi.matrixMultiply(n, a, b, c);
        
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                System.out.printf("%8.0f ", c[i * n + j]);
            }
            System.out.println();
        }
    }
}
JAVA

echo "✅ FFI examples generated in: $FFI_DIR"
echo ""
echo "Generated files:"
echo "  ✓ Chapel: $FFI_DIR/chapel/hello.chpl"
echo "  ✓ Python: $FFI_DIR/python/ffi_chapel.py"
echo "  ✓ Rust: $FFI_DIR/rust/ (Cargo project)"
echo "  ✓ Go: $FFI_DIR/go/main.go"
echo "  ✓ Java: $FFI_DIR/java/ChapelFFI.java"
echo ""
echo "Next steps:"
echo "  1. cd $FFI_DIR/chapel && chpl hello.chpl -o hello && ./hello"
echo "  2. cd $FFI_DIR/rust && cargo build --release"
echo "  3. python3 $FFI_DIR/python/ffi_chapel.py"
echo "  4. cd $FFI_DIR/go && go run main.go"
echo "  5. cd $FFI_DIR/java && javac ChapelFFI.java && java ChapelFFI"
echo ""

# FFI Implementation Guide

This directory contains Foreign Function Interface (FFI) implementations for:
- **Go**: Parallel processing with goroutines
- **Zig**: SIMD hashing and pattern matching
- **Nim**: HTML parsing and text extraction

## Building FFI Libraries

### Go (Windows/MSVC)
```bash
cd ffi/go
go build -buildmode=c-archive -o stealth_go_msvc.lib stealth.go
```

### Zig (Windows/MSVC)
```bash
cd ffi/zig
zig build-lib -dynamic -target x86_64-windows-msvc nuclear_zig.zig
```

### Nim (Windows/MSVC)
```bash
cd ffi/nim
nim c --app:lib --cpu:amd64 --os:windows -d:release nuclear_nim.nim
```

## Linux/macOS Support

On non-Windows platforms, the Rust fallback implementations are used automatically.
No FFI compilation is required.

## Status

Current status: Skeleton implementations provided as examples.
For production use, compile the libraries on Windows/MSVC.

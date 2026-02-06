//! 🚀 FFI ACCELERATORS - High-Performance Multi-Language Computing
//!
//! Real FFI integrations with NO MOCKS - each module powers specific MCP tools:
//!
//! | FFI Module         | Language | Powers MCP Tools                                    |
//! |--------------------+----------+----------------------------------------------------|
//! | chapel_integration | Chapel   | ALL 7 tools (AI learning + optimization)            |
//! | go_integration     | Go       | websearch, premium, scan, parallel_engine, osint    |
//! | zig_integration    | Zig      | file_search, scan, ai_dataset_trainer, parallel     |
//! | nim_integration    | Nim      | websearch, premium, scan, osint_intelligence        |
//! | jax_integration    | Python   | ai_dataset_trainer, parallel_engine                 |
//! | wasm_ffi_bridge    | WASM     | ALL tools (portable Go+Zig+Nim via wasmtime)        |
//!
//! WASM Source: ffi/wasm/go/main.go, ffi/wasm/zig/main.zig, ffi/wasm/nim/main.nim
//! Chapel Lib:  ffi/chapel/ai/nuclear_chapel_ai.chpl -> libchapel_ai.so
//! Build WASM:  ffi/wasm/build_wasm.sh [all|go|zig|nim]
//! Build Chapel: cd ffi/chapel && make chapel-lib

pub mod chapel_integration;
pub mod go_integration;
pub mod jax_integration;
pub mod nim_integration;
pub mod wasm_ffi_bridge;
pub mod zig_integration;

pub use chapel_integration::{get_chapel_ai, ChapelAI, ChapelContext};
pub use go_integration::*;
pub use jax_integration::*;
pub use nim_integration::*;
pub use wasm_ffi_bridge::*;
pub use zig_integration::*;

//! 🚀 FFI ACCELERATORS - High-Performance Computing
//!
//! Multi-language integrations including WASM bridges

pub mod chapel_integration;
pub mod go_integration;
pub mod jax_integration;
pub mod nim_integration;
pub mod zig_integration;
pub mod wasm_ffi_bridge;

pub use chapel_integration::{get_chapel_ai, ChapelAI, ChapelContext};
pub use go_integration::*;
pub use jax_integration::*;
pub use nim_integration::*;
pub use zig_integration::*;
pub use wasm_ffi_bridge::*;

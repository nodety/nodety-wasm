//! nodety-js: WASM wrapper for nodety with a simple type system (Integer, String, Array).
//!
//! Usable as both a WASM library and a Rust library for other crates.

pub mod notation;
pub mod r#type;

pub use r#type::MyType;

#[cfg(feature = "wasm")]
pub mod wasm;

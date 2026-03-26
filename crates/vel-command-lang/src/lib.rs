//! Deterministic Vel command-language parsing, completion, and explanation.
//!
//! This crate is intentionally pure and transport-free so it can be reused by
//! the CLI, daemon, and future WASM-backed clients.

pub mod ast;
pub mod completion;
pub mod explain;
pub mod infer;
pub mod parse;
pub mod preview;
pub mod registry;
pub mod shell;
pub mod tokenize;

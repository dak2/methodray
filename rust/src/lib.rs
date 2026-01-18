//! Method-Ray Core - Static type checker for Ruby
//!
//! This crate provides the core type inference engine.

pub mod types;
pub mod parser;
pub mod graph;
pub mod env;
pub mod analyzer;
pub mod cache;
pub mod diagnostics;
pub mod source_map;

#[cfg(feature = "ruby-ffi")]
pub mod rbs;

#[cfg(any(feature = "cli", feature = "lsp"))]
pub mod checker;

#[cfg(feature = "lsp")]
pub mod lsp;

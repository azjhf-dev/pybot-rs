//! pybot-rs: a chatbot written in plain Rust. No weights, no training, just rules.
//!
//! A partial Rust port of PyBot (https://github.com/huggingface/pybot):
//! the same module runtime (observe / respond / arbiter / after, per-module
//! memory copies, priority ordering) and a curated subset of the rule
//! modules. See README.md for what is and isn't here.

pub mod memory;
pub mod modules;
pub mod runtime;

pub use runtime::Runtime;

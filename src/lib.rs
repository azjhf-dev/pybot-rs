//! pybot-rs: a chatbot written in plain Rust. No weights, no training, just rules.
//!
//! A Rust port of PyBot (https://github.com/huggingface/pybot): every upstream
//! Python module under bot/ is transpiled to Rust under src/gen/, running on a
//! Python-semantics runtime (src/pyrt/). See README.md.

pub mod memory;
pub mod runtime;

pub use pyrt;
pub use runtime::Runtime;

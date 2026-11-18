//! Shared helpers for the ported modules.

use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Compile a regex once.
#[macro_export]
macro_rules! re {
    ($lit:expr) => {{
        static RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($lit).unwrap());
        &*RE
    }};
}

static RNG_STATE: AtomicU64 = AtomicU64::new(0);

/// Pick one line from a pool, upstream-style (`random.choice`).
pub fn pick<S: AsRef<str>>(pool: &[S]) -> String {
    if pool.is_empty() {
        return String::new();
    }
    let mut state = RNG_STATE.load(Ordering::Relaxed);
    if state == 0 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ (d.as_secs() << 20))
            .unwrap_or(0x9e3779b97f4a7c15);
        state = nanos | 1;
    }
    // xorshift64*
    state ^= state >> 12;
    state ^= state << 25;
    state ^= state >> 27;
    RNG_STATE.store(state, Ordering::Relaxed);
    let idx = ((state.wrapping_mul(0x2545F4914F6CDD1D)) >> 33) as usize % pool.len();
    pool[idx].as_ref().to_string()
}

pub static WORD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z']+").unwrap());

/// Lowercased word set of the message.
pub fn words(text: &str) -> std::collections::HashSet<String> {
    WORD_RE
        .find_iter(&text.to_lowercase())
        .map(|m| m.as_str().to_string())
        .collect()
}

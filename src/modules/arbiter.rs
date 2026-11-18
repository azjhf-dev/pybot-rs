//! The chooser, ported from upstream `bot/modules/arbiter.py`.
//!
//! Upstream's arbiter is 5.5k lines of tie-breaking between candidate
//! replies (a safety card outranks chit-chat, a specific answer outranks a
//! generic one, a module that was just answered does not repeat itself).
//!
//! The runtime contract it must keep, and the part this port reproduces:
//! candidates arrive in priority order, the arbiter returns an index into
//! them, and when the arbiter cannot decide the runtime takes candidate 0,
//! the highest PRIORITY. This port keeps exactly one rule of its own on top
//! of that: the always-on fallback pool (priority -1000) is only ever chosen
//! when nothing else proposed, which is what the upstream seed documents for
//! it ("it only answers when no other module does").

use crate::memory::Memory;
use crate::runtime::Candidate;

pub fn choose(_text: &str, _memory: &Memory, candidates: &[Candidate]) -> usize {
    if candidates.len() < 2 {
        return 0;
    }
    match candidates.iter().position(|c| c.priority > -1000) {
        Some(i) => i,
        None => 0,
    }
}

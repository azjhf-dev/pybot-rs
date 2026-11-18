//! The seed's fallback: it only answers when no other module does.
//!
//! Ported from the role of upstream `bot/modules/greeting.py` and
//! `bot/compose/fallback*.py` (PRIORITY -1000). Upstream's fallback is
//! thousands of lines of contextual pools keyed on what the thread has been
//! about; this port keeps the contract - always propose something, sit at
//! the bottom of the priority order, and only win when nothing else spoke.

use crate::memory::Memory;
use crate::modules::util::pick;
use crate::runtime::Module;

pub struct Fallback;

const GENERIC: &[&str] = &[
    "Tell me more about that.",
    "Go on - I'm listening.",
    "Say a bit more and I'll do my best.",
    "What's the part of that you most want a hand with?",
    "I don't have a good answer for that one yet. What would you like to know?",
    "That's beyond my rules so far - talk me through it and I'll keep up as best I can.",
];

impl Module for Fallback {
    fn name(&self) -> &'static str {
        "fallback"
    }

    fn priority(&self) -> i64 {
        -1000
    }

    fn respond(&self, _text: &str, _memory: &mut Memory) -> Option<String> {
        Some(pick(GENERIC))
    }
}

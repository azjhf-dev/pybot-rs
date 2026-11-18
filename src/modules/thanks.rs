//! Thanks, ported from upstream `bot/modules/chat_replies.py` (pool verbatim).

use crate::memory::Memory;
use crate::modules::util::pick;
use crate::runtime::Module;
use crate::re;

pub struct Thanks;

const THANKS: &[&str] = &[
    "Any time.",
    "Glad that helped.",
    "You're welcome - shout if anything else comes up.",
    "No problem at all.",
    "Happy to help.",
    "No worries at all.",
];

impl Module for Thanks {
    fn name(&self) -> &'static str {
        "thanks"
    }

    fn priority(&self) -> i64 {
        80
    }

    fn respond(&self, text: &str, _memory: &mut Memory) -> Option<String> {
        // Only when the line *is* the thanks: "thanks, but how do I..." still
        // carries a question that a real answer module should take.
        if re!(r"(?i)^(?:thanks|thank you|thanks a lot|thank you so much|thx|ty|cheers|ta|much appreciated)[!.,\s]*$").is_match(text.trim()) {
            return Some(pick(THANKS));
        }
        None
    }
}

//! Sign-offs, ported from upstream `bot/modules/chat_replies.py`.
//!
//! Upstream split the night sign-off from the day one after "cya later"
//! was answered "Night! Sleep well." and "goodnight" was answered "Bye!
//! Take care." - the right words for the wrong half of the day.

use crate::memory::Memory;
use crate::modules::util::pick;
use crate::runtime::Module;
use crate::re;

pub struct Farewell;

const BYE: &[&str] = &[
    "Bye! Take care.",
    "See you - good luck with it.",
    "Take care. Come back any time.",
    "Bye for now!",
    "Take it easy!",
];

const BYE_NIGHT: &[&str] = &[
    "Night! Sleep well.",
    "Goodnight - sleep well.",
    "Sleep well. I'll be here tomorrow.",
];

impl Module for Farewell {
    fn name(&self) -> &'static str {
        "farewell"
    }

    fn priority(&self) -> i64 {
        80
    }

    fn respond(&self, text: &str, _memory: &mut Memory) -> Option<String> {
        let whole = text.trim();
        if re!(r"(?i)^(?:good\s*night|goodnight|night|night night|sleep well|off to bed)[!.,\s]*$").is_match(whole) {
            return Some(pick(BYE_NIGHT));
        }
        if re!(r"(?i)^(?:bye|bye bye|goodbye|good bye|see you|see ya|seeya|cya|later|take care|gtg|got to go|gotta go|i'?m off|farewell)[!.,\s]*$").is_match(whole) {
            return Some(pick(BYE));
        }
        None
    }
}

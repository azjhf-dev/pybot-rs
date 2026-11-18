//! "how are you" and its answers, ported from upstream
//! `bot/modules/chat_replies.py` (pools verbatim).

use crate::memory::Memory;
use crate::modules::util::pick;
use crate::runtime::Module;
use crate::re;

pub struct HowAreYou;

const HOWAREYOU: &[&str] = &[
    "Doing well, thanks - what about you?",
    "All good here. What's keeping you busy?",
    "Can't complain. What's on your mind?",
];

const BACK: &[&str] = &[
    "All good here, thanks for asking. How has your day been?",
    "I'm well, thanks for asking. How is your day going?",
    "Nothing to complain about here, thanks. How has your day been so far?",
];

const BACK_LOW: &[&str] = &[
    "Sorry to hear that. I'm all right, thanks for asking - what has been going on?",
    "That sounds rough. I'm fine, thanks for asking after me - what has the day been like?",
];

const NOTMUCH: &[&str] = &[
    "Not much here either. What are you up to?",
    "Same as ever - helping, chatting. How's your day going?",
    "Not a lot. What's keeping you busy?",
];

impl Module for HowAreYou {
    fn name(&self) -> &'static str {
        "howareyou"
    }

    fn priority(&self) -> i64 {
        90
    }

    fn respond(&self, text: &str, _memory: &mut Memory) -> Option<String> {
        // The person answers how they are and hands the question back:
        // "not bad, and you?", "im fine thanks, yourself".
        if re!(r"(?i)\b(?:and you|how about you|hbu|yourself|what about you)\b[!?\s.]*$").is_match(text) {
            if re!(r"(?i)\b(?:not great|not good|terrible|awful|rough|bad day|shit|crap|rubbish|poorly|sad|tired)\b").is_match(text) {
                return Some(pick(BACK_LOW));
            }
            return Some(pick(BACK));
        }
        // "not much" / "nm" as the whole line.
        if re!(r"(?i)^(?:not much|nm|nothing much|nvm|same old)[!.,\s]*$").is_match(text.trim()) {
            return Some(pick(NOTMUCH));
        }
        // The direct question.
        if re!(r"(?i)\bhow are you\b|\bhow'?s it going\b|\bhow are things\b|\bhow do you feel\b|\byou ok\b|\byou alright\b|\bwhat'?s up\b|\bhow'?s your day\b").is_match(text) {
            return Some(pick(HOWAREYOU));
        }
        None
    }
}

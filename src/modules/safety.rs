//! The safety net, ported from upstream `bot/modules/safety.py`.
//!
//! Upstream's safety module is 12k lines: wish-to-die readings with joke,
//! quote, lyric and third-party exclusions; medical red-flag cards that say
//! "ring 999 or 911" and never get outvoted (PRIORITY 9000: "an emergency
//! is never outvoted"); overdose counts; pet crises. This port carries the
//! two cards that must never be missed - a stated wish to die and a named
//! medical emergency - and keeps the fail-closed shape: when the words
//! match, this module answers above everything else. It is a *subset*: the
//! upstream exclusions (jokes, quotes, songs, third parties, dated events)
//! are only sketched here, so this port will over-trigger where upstream
//! would not. For a demo that is the safer side to err on.

use crate::memory::Memory;
use crate::runtime::Module;
use crate::re;

pub struct Safety;

/// A stated wish to die or to self-harm, in first person.
const WISH_TO_DIE: &str = r"(?i)\b(?:i (?:want|wanna|wish) (?:to )?(?:die|be dead|end (?:it|it all|my life))|i don'?t want to (?:live|be here|exist|go on|wake up)|(?:kill|end|hurt|harm) myself|suicid\w*|no (?:reason|point) (?:to live|in living)|better off (?:dead|without me)|end it all|take my own life|self ?harm\w*|cut(?:ting)? (?:myself|my (?:wrists?|arms?))|overdose\b|can'?t go on anymore)\b";

/// Ways the same words are not a disclosure: a joke, a quote, a song, a
/// third party, a game. (Subset of upstream's exclusion arms.)
const NOT_A_WISH: &str = r"(?i)\b(?:lol|lmao|haha|hehe|jk|j/k|joking|kidding|just kidding|the (?:song|lyrics|film|movie|book|game)|my (?:character|avatar)|in (?:the )?game)\b";

/// A named, right-now medical emergency.
const MEDICAL_RED_FLAG: &str = r"(?i)\b(?:heart attack|not breathing|stopped breathing|can'?t breathe|choking|unconscious|won'?t wake up|severe bleeding|bleeding won'?t stop|swallowed (?:bleach|poison|pills)|took an overdose|stroke)\b";

const CRISIS_REPLY: &str = "I'm really glad you said that out loud, and I'm listening. You don't have to carry this on your own tonight: please reach out to a crisis line now - Samaritans are on 116 123 (UK & Ireland) or call or text 988 (US), and anywhere else the emergency number is 112, 999 or 911. If you might act on these thoughts, treat it as an emergency and ring 999 or 911 now.";

const MEDICAL_REPLY: &str = "Ring 999 or 911 straight away - don't wait to see how it goes. Say exactly what happened, where you are, and stay on the line and do what the handler tells you while the ambulance comes.";

impl Module for Safety {
    fn name(&self) -> &'static str {
        "safety"
    }

    fn priority(&self) -> i64 {
        9000 // above the answer modules: an emergency is never outvoted
    }

    fn respond(&self, text: &str, _memory: &mut Memory) -> Option<String> {
        if re!(WISH_TO_DIE).is_match(text) && !re!(NOT_A_WISH).is_match(text) {
            return Some(CRISIS_REPLY.to_string());
        }
        if re!(MEDICAL_RED_FLAG).is_match(text) {
            return Some(MEDICAL_REPLY.to_string());
        }
        None
    }
}

//! Greetings and introductions, ported from upstream
//! `bot/modules/chat_replies.py` (the reply pools are verbatim).
//!
//! Upstream reads a greeting only when the line *is* a greeting: "hello,
//! can you fix my boiler" is a boiler question with hello in front. This
//! port keeps that: pure-greeting patterns are anchored to the whole line,
//! and introductions match anywhere.

use crate::memory::Memory;
use crate::modules::facts;
use crate::modules::util::pick;
use crate::runtime::Module;
use crate::re;

pub struct Greeting;

// Verbatim from upstream chat_replies.py.
const GREETING_MORNING: &[&str] = &[
    "Morning! How's your day going?",
    "Morning - what's on your mind?",
    "Morning! What can I help with?",
];
const GREETING_AFTERNOON: &[&str] = &[
    "Afternoon! What's on your mind?",
    "Afternoon - what can I help with?",
    "Afternoon! How's your day going?",
];
const GREETING_EVENING: &[&str] = &[
    "Evening! What's on your mind?",
    "Evening - what can I help with?",
    "Evening! How's your day been?",
];
const GREETING_PLAIN: &[&str] = &[
    "Hi! What's on your mind?",
    "Hello! What can I help with?",
    "Hey! What are we doing today?",
];

impl Module for Greeting {
    fn name(&self) -> &'static str {
        "greeting"
    }

    fn priority(&self) -> i64 {
        100
    }

    fn respond(&self, text: &str, memory: &mut Memory) -> Option<String> {
        let known = facts::name(memory);

        // "i'm Sam" / "my name is Sam" / "call me Sam": the introduction.
        // facts::observe has already filed the name, so greet it back.
        let intro = crate::re!(r"(?i)\b(?:my name(?:'?s| is)|call me|i'?m|i am)\s+[A-Za-z][A-Za-z'\-]*\b");
        if intro.is_match(text) {
            if let Some(name) = known {
                return Some(pick(&[
                    format!("Hi {} - good to meet you. What's on your mind?", name),
                    format!("Nice to meet you, {}. What can I help with?", name),
                    format!("Hello {}! What are we doing today?", name),
                ]));
            }
        }

        let whole = text.trim();
        if re!(r"(?i)^(?:good\s+)?morning[!.,\s]*$").is_match(whole) {
            return Some(pick(GREETING_MORNING));
        }
        if re!(r"(?i)^(?:good\s+)?afternoon[!.,\s]*$").is_match(whole) {
            return Some(pick(GREETING_AFTERNOON));
        }
        if re!(r"(?i)^(?:good\s+)?evening[!.,\s]*$").is_match(whole) {
            return Some(pick(GREETING_EVENING));
        }
        if re!(r"(?i)^(?:hi+|hello+|hey+|hiya|howdy|yo|hullo|heyy+)[!.,\s]*$").is_match(whole) {
            if let Some(name) = known {
                return Some(pick(&[
                    format!("Hi {}! What's on your mind?", name),
                    format!("Morning {}! What can I help with?", name),
                    format!("Hey {}! What are we doing today?", name),
                ]));
            }
            return Some(pick(GREETING_PLAIN));
        }
        None
    }
}

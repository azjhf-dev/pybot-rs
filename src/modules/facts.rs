//! Facts about the person, ported from upstream `bot/memory/facts.py`.
//!
//! Upstream keeps a small fact store: `{"name": "Jess", "job": "nurse",
//! "city": "Leeds"}` in an ordered log, so the bot can answer "what did I
//! say my name was?". This port carries the name, the fact the demo leans
//! on. The hard part upstream is not storing the name but *reading* one:
//! "im tired" is a state, not an introduction. The blocklist below is a
//! subset of upstream's state/condition/behaviour tables (its comments
//! explain the trade-off: a state read as a name is the worse mistake, so
//! when in doubt, do not file).

use crate::memory::{self, Memory};
use crate::runtime::Module;
use crate::{re, modules::util::pick};
use serde_json::{Map, Value};

pub struct Facts;

/// Words that after "i'm" describe how somebody is, never who they are.
/// (Subset of upstream's state tables; add, never guess.)
const NOT_A_NAME: &[&str] = &[
    "tired", "exhausted", "sleepy", "awake", "asleep", "hungry", "thirsty", "full", "sick",
    "ill", "unwell", "fine", "ok", "okay", "good", "great", "grand", "well", "bad", "sad",
    "happy", "glad", "mad", "angry", "annoyed", "bored", "busy", "free", "ready", "done",
    "sorry", "sure", "unsure", "lost", "confused", "worried", "scared", "afraid", "nervous",
    "stressed", "anxious", "depressed", "lonely", "alone", "cold", "hot", "warm", "cool",
    "wet", "dry", "sweaty", "itchy", "sore", "hurt", "injured", "drunk", "sober", "high",
    "lazy", "late", "early", "back", "here", "there", "home", "away", "out", "in", "up",
    "down", "off", "on", "not", "so", "very", "really", "just", "also", "still", "now",
    "nobody", "somebody", "rich", "poor", "broke", "fat", "thin", "fit", "weak", "strong",
    "pregnant", "married", "single", "divorced", "engaged", "stuck", "shocked", "surprised",
    "confused", "curious", "excited", "nervous", "frustrated", "furious", "devastated",
    "mardy", "peckish", "shattered", "knackered", "beat", "wrecked", "finished", "overwhelmed",
    "new", "old", "young", "dying", "bleeding", "vomiting", "crying", "laughing", "joking",
    "kidding", "serious", "desperate", "hopeless", "helpless", "worthless",
];

/// The introduction shapes: "i'm Sam", "my name is Sam", "call me Sam".
/// The candidate word must be capitalised in the original message, which is
/// what keeps "im tired" out without a table lookup first.
fn the_name_in(text: &str) -> Option<String> {
    let intro = re!(r"(?i)\b(?:my name(?:'?s| is)|call me|i'?m|i am)\s+([A-Za-z][A-Za-z'\-]*)\b");
    let caps = intro.captures(text)?;
    let word = caps.get(1)?.as_str();
    if NOT_A_NAME.contains(&word.to_lowercase().as_str()) {
        return None;
    }
    // Only a capitalised word is offered as a name ("I'm Jess", not "im tired"
    // after the table, and not "im eric" in a lower-cased shout unless the
    // writer capitalised it). Upstream keeps a second chance for lower-case
    // names; this port takes the safer half of that trade.
    if !word.chars().next()?.is_uppercase() {
        return None;
    }
    Some(word.to_string())
}

pub fn name(memory: &Memory) -> Option<String> {
    memory::get_str(memory, "facts", Some("name"))
}

impl Module for Facts {
    fn name(&self) -> &'static str {
        "facts"
    }

    /// observe: file the name. Never replies.
    fn observe(&self, text: &str, memory: &mut Memory) {
        if let Some(name) = the_name_in(text) {
            let facts = memory
                .entry("facts".to_string())
                .or_insert_with(|| Value::Object(Map::new()));
            if let Value::Object(map) = facts {
                map.insert("name".to_string(), Value::String(name));
            }
        }
    }

    /// respond: answer "what's my name?" from the store.
    fn respond(&self, text: &str, memory: &mut Memory) -> Option<String> {
        let asked = re!(r"(?i)\bwhat(?:'s| is| was) my name\b|\bwhat did i say my name (?:was|is)\b|\bdo you (?:know|remember) my name\b|\bremember my name\b");
        if !asked.is_match(text) {
            return None;
        }
        match name(memory) {
            Some(n) => Some(pick(&[&format!("You're {}.", n), &format!("You're {} - you told me earlier.", n)])),
            None => Some(pick(&[
                "You haven't told me your name yet.",
                "I don't know it yet - what should I call you?",
            ])),
        }
    }
}

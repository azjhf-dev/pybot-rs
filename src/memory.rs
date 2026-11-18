//! Conversation memory.
//!
//! Mirrors upstream `bot/runtime.py`: memory is a string-keyed map that
//! modules read and write. `memory["turns"]` is the runtime's own ordered
//! log of `{"user", "bot", "module"}` objects; modules may read it but
//! only the runtime appends to it. Every `respond` works on its own deep
//! copy, so a reply that is not chosen leaves no trace.

use serde_json::{Map, Value};

pub type Memory = Map<String, Value>;

pub fn new_memory() -> Memory {
    let mut m = Memory::new();
    m.insert("turns".to_string(), Value::Array(Vec::new()));
    m
}

/// Read a string at `key`, or at `key.sub` when `sub` is given.
pub fn get_str(memory: &Memory, key: &str, sub: Option<&str>) -> Option<String> {
    let v = memory.get(key)?;
    let v = match sub {
        Some(s) => v.get(s)?,
        None => v,
    };
    v.as_str().map(|s| s.to_string())
}

pub fn set(memory: &mut Memory, key: &str, value: Value) {
    memory.insert(key.to_string(), value);
}

pub fn push_turn(memory: &mut Memory, user: &str, bot: &str, module: Option<&str>) {
    let turns = memory
        .entry("turns".to_string())
        .or_insert_with(|| Value::Array(Vec::new()));
    if let Value::Array(list) = turns {
        let mut t = Map::new();
        t.insert("user".to_string(), Value::String(user.to_string()));
        t.insert("bot".to_string(), Value::String(bot.to_string()));
        t.insert(
            "module".to_string(),
            module.map_or(Value::Null, |m| Value::String(m.to_string())),
        );
        list.push(Value::Object(t));
    }
}

/// Number of user/bot turns so far.
pub fn turn_count(memory: &Memory) -> usize {
    match memory.get("turns") {
        Some(Value::Array(list)) => list.len(),
        _ => 0,
    }
}

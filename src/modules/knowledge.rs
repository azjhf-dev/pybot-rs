//! A small household-knowledge lookup with follow-up memory, in the shape
//! of upstream's `bot/knowledge/` modules.
//!
//! Upstream's knowledge tables are tens of thousands of lines of everyday
//! answers the swarm wrote by hand. This port carries a handful of entries
//! to show the shape, including the kettle-limescale answer from the
//! upstream README demo. Each entry remembers its topic and (where it has
//! one) its duration, so the FollowUp module can answer "how long does
//! that take?" the way the upstream demo does ("Half an hour.").

use crate::memory::{self, Memory};
use crate::modules::util::pick;
use crate::runtime::Module;
use crate::re;
use serde_json::{Map, Value};

pub struct Knowledge;
pub struct FollowUp;

struct Entry {
    pattern: &'static str,
    topic: &'static str,
    duration: Option<&'static str>,
    reply: &'static str,
}

const ENTRIES: &[Entry] = &[
    Entry {
        // Verbatim from the upstream README's example.
        pattern: r"(?i)\blimescale\b|\bkettle\b.*\b(?:scale|descale|clean)|\bdescale\b",
        topic: "kettle_limescale",
        duration: Some("Half an hour."),
        reply: "Half fill the kettle with equal parts water and white vinegar, or a sachet of citric acid, boil it and leave it half an hour, then tip it out and boil fresh water twice before you use it again. In a hard-water area do it every couple of months: the scale forms because hard water leaves calcium behind as it boils, and a thick layer makes the kettle slow and puts a scum on the tea.",
    },
    Entry {
        pattern: r"(?i)\bblocked (?:drain|sink)|\bunblock\b.*\bdrain|\bdrain\b.*\b(?:blocked|clogged|slow)|\bclogged\b",
        topic: "blocked_drain",
        duration: Some("About fifteen minutes."),
        reply: "Tip a kettle of hot (not boiling, if the pipes are plastic) water down first, then half a cup of bicarbonate of soda followed by a cup of white vinegar, cover the plughole and leave it ten minutes, then flush with another kettle of hot water. If it's still slow, a plastic drain snake down the plug will pull out the hair the fizz can't reach.",
    },
    Entry {
        pattern: r"(?i)\bred wine\b.*\bstain|\bstain\b.*\bred wine|\bspilled? wine\b",
        topic: "red_wine_stain",
        duration: None,
        reply: "Blot, don't rub - press a clean cloth on it to take up what you can. Cover the damp patch with salt and leave it to draw the wine out, then rinse from the back of the fabric with cold water and wash as usual. On carpet, soda water on the cloth helps lift what's left.",
    },
    Entry {
        pattern: r"(?i)\bhiccups\b|\bhiccoughs\b",
        topic: "hiccups",
        duration: None,
        reply: "The one with the best track record: take a deep breath, hold it as long as is comfortable, then swallow twice before you breathe out. Sipping cold water slowly, or a teaspoon of sugar on the tongue, works for plenty of people too. Hiccups that last more than a couple of days are worth a doctor's look.",
    },
    Entry {
        pattern: r"(?i)\bripen\b.*\bavocado|\bavocado\b.*\b(?:ripen|too hard|unripe)|\bhard avocados\b",
        topic: "ripen_avocado",
        duration: Some("A day or two."),
        reply: "Put the avocados in a paper bag with a banana and fold the top down: the ethylene the banana gives off speeds the ripening along. Check daily - a ripe one yields slightly at the stem end. Once they're there, the fridge holds them for a few days.",
    },
    Entry {
        pattern: r"(?i)\bfruit flies\b|\bfruit fly\b",
        topic: "fruit_flies",
        duration: Some("A day or two."),
        reply: "Set a trap: a small dish of apple cider vinegar with a drop of washing-up liquid, covered in cling film with a few holes poked in, or just left open - the soap breaks the surface so they sink. Then bin any overripe fruit, wipe the counters, and take the rubbish out, or a new brood replaces the ones you catch.",
    },
];

fn record_topic(memory: &mut Memory, topic: &str, duration: Option<&str>) {
    let mut last = Map::new();
    last.insert("topic".to_string(), Value::String(topic.to_string()));
    last.insert(
        "duration".to_string(),
        duration.map_or(Value::Null, |d| Value::String(d.to_string())),
    );
    memory::set(memory, "last_knowledge", Value::Object(last));
}

use once_cell::sync::Lazy;

static ENTRY_RES: Lazy<Vec<regex::Regex>> =
    Lazy::new(|| ENTRIES.iter().map(|e| regex::Regex::new(e.pattern).unwrap()).collect());

impl Module for Knowledge {
    fn name(&self) -> &'static str {
        "knowledge"
    }

    fn priority(&self) -> i64 {
        50
    }

    fn respond(&self, text: &str, memory: &mut Memory) -> Option<String> {
        for (i, entry) in ENTRIES.iter().enumerate() {
            if ENTRY_RES[i].is_match(text) {
                record_topic(memory, entry.topic, entry.duration);
                return Some(entry.reply.to_string());
            }
        }
        None
    }
}

impl Module for FollowUp {
    fn name(&self) -> &'static str {
        "knowledge_followup"
    }

    fn priority(&self) -> i64 {
        60 // above the lookup: the follow-up only makes sense against the last answer
    }

    fn respond(&self, text: &str, memory: &mut Memory) -> Option<String> {
        let asked = re!(r"(?i)^(?:and\s+)?how long (?:does|did|will) (?:that|it) take\b\??\s*$");
        if !asked.is_match(text.trim()) {
            return None;
        }
        let duration = memory::get_str(memory, "last_knowledge", Some("duration"))?;
        if memory::turn_count(memory) == 0 {
            return None;
        }
        Some(pick(&[duration.as_str()]))
    }
}

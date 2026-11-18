//! The fixed module interface, ported from upstream `bot/runtime.py`.
//!
//! Every turn runs the registered modules in descending `priority`
//! (default 0), then by name, in three phases:
//!
//! 1. `observe(text, memory)`: remember things (a name, a fact). Never replies.
//! 2. `respond(text, memory)`: propose a reply or pass (`None`). Every module
//!    works on its own deep copy of memory, so a reply that is not chosen
//!    leaves no trace.
//! 3. `arbiter::choose` picks one candidate, and the chosen module's copy of
//!    memory becomes the conversation's memory. Without a working arbiter the
//!    first candidate (the highest priority) is chosen.
//!
//! Then `after(text, reply, memory)` runs in every module that defines it,
//! and the runtime appends the turn to `memory["turns"]`. A module that
//! fails is skipped; a turn never fails.

use crate::memory::{self, Memory};
use crate::modules;

pub const FALLBACK: &str = "Hello!";

pub struct Candidate {
    pub module: &'static str,
    pub priority: i64,
    pub reply: String,
}

pub struct Turn {
    pub reply: String,
    pub module: Option<&'static str>,
    pub candidates: Vec<Candidate>,
}

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn priority(&self) -> i64 {
        0
    }
    /// Remember things. Runs on the shared memory. Never replies.
    fn observe(&self, _text: &str, _memory: &mut Memory) {}
    /// Propose a reply. `memory` is this module's own deep copy.
    fn respond(&self, _text: &str, _memory: &mut Memory) -> Option<String> {
        None
    }
    /// Runs on the shared memory after the reply is chosen.
    fn after(&self, _text: &str, _reply: &str, _memory: &mut Memory) {}
}

pub struct Runtime {
    modules: Vec<Box<dyn Module>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut modules = modules::all();
        // Highest priority first, then by name: the upstream sort is
        // (-PRIORITY, filename); the module name plays the filename's part.
        modules.sort_by(|a, b| b.priority().cmp(&a.priority()).then(a.name().cmp(b.name())));
        Runtime { modules }
    }

    pub fn turn(&self, text: &str, memory: &mut Memory) -> Turn {
        for m in &self.modules {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                m.observe(text, memory);
            }));
        }

        let mut candidates: Vec<Candidate> = Vec::new();
        let mut copies: Vec<Memory> = Vec::new();
        for m in &self.modules {
            let mut mine = memory.clone();
            let answer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                m.respond(text, &mut mine)
            }))
            .unwrap_or(None);
            if let Some(reply) = answer {
                let reply = reply.split_whitespace().collect::<Vec<_>>().join(" ");
                if !reply.is_empty() {
                    candidates.push(Candidate {
                        module: m.name(),
                        priority: m.priority(),
                        reply,
                    });
                    copies.push(mine);
                }
            }
        }

        let (reply, module) = if candidates.is_empty() {
            (FALLBACK.to_string(), None)
        } else {
            let index = modules::arbiter::choose(text, memory, &candidates);
            let reply = candidates[index].reply.clone();
            let module = candidates[index].module;
            // The chosen module's copy becomes the conversation's memory.
            let chosen = copies.swap_remove(index);
            memory.clear();
            memory.extend(chosen);
            (reply, Some(module))
        };

        for m in &self.modules {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                m.after(text, &reply, memory);
            }));
        }
        memory::push_turn(memory, text, &reply, module);

        Turn {
            reply,
            module,
            candidates,
        }
    }

    pub fn reply(&self, text: &str, memory: &mut Memory) -> String {
        self.turn(text, memory).reply
    }
}

//! The fixed module interface, ported from upstream `bot/runtime.py`.
//!
//! Every turn runs the registered modules in descending `priority`
//! (default 0), then by name, in three phases: observe, respond, choose;
//! then after. A module that fails is skipped; a turn never fails.

use crate::memory::Memory;
use crate::pyrt::exc;
use crate::pyrt::value::*;

pub const FALLBACK: &str = "Hello!";

pub struct Candidate {
    pub module: String,
    pub priority: i64,
    pub reply: String,
}

pub struct Turn {
    pub reply: String,
    pub module: Option<String>,
    pub candidates: Vec<Candidate>,
}

pub struct ModuleEntry {
    pub name: &'static str,
    pub priority: fn() -> i64,
    pub observe: Option<fn(&[Py]) -> Py>,
    pub respond: Option<fn(&[Py]) -> Py>,
    pub after: Option<fn(&[Py]) -> Py>,
}

pub struct Runtime {
    modules: Vec<ModuleEntry>,
}

impl Default for Runtime {
    fn default() -> Self { Self::new() }
}

impl Runtime {
    pub fn new() -> Self {
        let mut modules = crate::gen::registry();
        modules.sort_by(|a, b| (b.priority)().cmp(&(a.priority)()).then(a.name.cmp(b.name)));
        Runtime { modules }
    }

    pub fn turn(&self, text: &str, memory: &mut Memory) -> Turn {
        let text_py = py_str(text);
        for m in &self.modules {
            if let Some(observe) = m.observe {
                let mem = memory.clone();
                let tp = text_py.clone();
                let _ = exc::catch(move || observe(&[tp, mem]));
            }
        }

        let mut candidates: Vec<Candidate> = Vec::new();
        let mut copies: Vec<Memory> = Vec::new();
        for m in &self.modules {
            if let Some(respond) = m.respond {
                let mut mine = crate::pyrt::builtins::py_deepcopy(memory);
                let name = m.name;
                let answer = exc::catch({
                    let text_py = text_py.clone();
                    let mine_ref = mine.clone();
                    move || respond(&[text_py, mine_ref])
                }).unwrap_or(Py::None);
                if let Py::Str(reply) = &answer {
                    let cleaned = reply.split_whitespace().collect::<Vec<_>>().join(" ");
                    if !cleaned.is_empty() {
                        candidates.push(Candidate {
                            module: name.to_string(),
                            priority: (m.priority)(),
                            reply: cleaned,
                        });
                        // The upstream runtime keeps the module's mutated copy.
                        copies.push(mine);
                    }
                }
            }
        }

        let (reply, module) = if candidates.is_empty() {
            (FALLBACK.to_string(), None)
        } else {
            let index = choose(text, memory, &candidates);
            let reply = candidates[index].reply.clone();
            let module = candidates[index].module.clone();
            let chosen = copies.swap_remove(index);
            *memory = chosen;
            (reply, Some(module))
        };

        let reply_py = py_str(&reply);
        for m in &self.modules {
            if let Some(after) = m.after {
                let mem = memory.clone();
                let tp = text_py.clone();
                let rp = reply_py.clone();
                let _ = exc::catch(move || after(&[tp, rp, mem]));
            }
        }
        crate::memory::push_turn(memory, text, &reply, module.as_deref());

        Turn { reply, module, candidates }
    }

    pub fn reply(&self, text: &str, memory: &mut Memory) -> String {
        self.turn(text, memory).reply
    }
}

/// The arbiter's pick, as an index into candidates; 0 when it fails or
/// returns nonsense (mirrors runtime.py `_choose`).
fn choose(text: &str, memory: &Memory, candidates: &[Candidate]) -> usize {
    if candidates.len() < 2 { return 0; }
    let arbiter = match crate::gen::arbiter_choose() {
        Some(f) => f,
        None => return 0,
    };
    let views = py_list(candidates.iter().map(|c| py_dict(vec![
        (py_str("module"), py_str(&c.module)),
        (py_str("priority"), Py::Int(c.priority)),
        (py_str("reply"), py_str(&c.reply)),
    ])).collect());
    let picked = exc::catch({
        let text = py_str(text);
        let mem = memory.clone();
        move || arbiter(&[text, mem, views])
    });
    match picked {
        Ok(Py::Int(i)) if i >= 0 && (i as usize) < candidates.len() => i as usize,
        Ok(Py::Bool(_)) => 0,
        Ok(Py::Dict(d)) => {
            let name = d.borrow().get(&PyKey(py_str("module"))).cloned();
            match name {
                Some(Py::Str(n)) => candidates.iter().position(|c| c.module == *n).unwrap_or(0),
                _ => 0,
            }
        }
        _ => 0,
    }
}

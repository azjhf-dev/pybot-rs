#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use pyrt::builtins::*;
use pyrt::pybuiltins::*;
use pyrt::stdlib::*;
use pyrt::value::*;
use pyrt::exc;
use pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The two ends of one night, when the chat gives them one line at a time.\n\nA night is told in pieces, and only the first piece has a bedtime in it:\n\n    i feel knackered all the time   ->  the constant-tiredness card\n    i go to bed at about 11         ->  \"11pm is a good bedtime - eight hours before a seven\n                                        o'clock start\"\n    and wake at 6                   ->  \"Waking at the same time every night is common and it\n                                        passes\" (advice for somebody who cannot stay asleep)\n\nThe last line was read on its own, so the person who had just said their night is eleven to six\nwas given the wrong leaflet. `bot/memory/asleep.py` holds the reading: the bedtime, the alarm, the\nhours between them, and the seven to nine most adults need. This module is the turn it is asked\non, and it answers nothing else - `answer_for` returns None for every line that does not give a\nnight, so the chatter band and the knowledge entries keep every other turn.\n\nA sleep that names the thing it depends on - \"i cant fall asleep without the telly on\", \"i need\nthe fan on to get to sleep\" - is the same night told by its aid. `bot/knowledge/sleep_aids.py`\nholds those rows, and this module answers them here before `chat` can read the line as a bad week.\n\nAbove `chat` (300), which owns the night-waking pool this line used to fall into, and below the\nmemory store and the safety bands, whose turns these are not.\n");
    s_PRIORITY(Py::Int(1100));
}
pub fn respond(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut aid: Py = Py::None;
    let _ = py_str("The hours between the two ends of the night they described, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    aid = gen_knowledge::knowledge::sleep_aids::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&aid.clone()) {
    return aid.clone();
    }
    return { let __b = gen_memory::memory::asleep::answer_for(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.night".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.night", module_obj);
    pyrt::xrefs::reg_fn("modules.night", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.night", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

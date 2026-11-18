#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A count the person reports, read against the count the last answer named.\n\n\"how many steps should i do a day\" names a figure to aim at (\"about 7,000 to 8,000 steps a day\"),\nand the next thing people do is say how they are doing against it: \"i have about 7000 steps\". That\nline was answered with a search hedge, a receipt or a greeting, because the answer's own figure is\nsaid as a rate (\"7,000 to 8,000 steps a day\") and a rate is not read as an amount to compare with.\n\n`bot/memory/amounts.py` reads the two counts. This module is the thin part: one line in, one line\nout, and it hands the line back (None) whenever the counts are not the same thing said the same\nway - a count of something else, a question about an amount, a length that belongs to something\nelse (\"2 hours of meetings\"), or an answer naming two figures of that thing.\n\nIt answers **only** the turn that reports the count. The turn after it (\"how much more do i need\",\n\"is that enough\") belongs to the store, which already reads it and keeps its own guards, so\nnothing here is kept in `memory` and nothing is claimed twice.\n");
    s_PRIORITY(Py::Int(7000));
}
pub fn observe(args: &[Py]) -> Py {
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
    let _ = py_str("Nothing to keep: the report turn is read from the answer above it, every time.");
    return Py::None;
    Py::None
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
    let mut against: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Where the count they reported stands against the answer above it, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = crate::gen::memory::amounts::a_count_they_report(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    last = if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { turns.clone() } }) { mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { py_str("") };
    against = crate::gen::memory::amounts::against_the_last_answer(&[said.clone(), { let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(py_is(&against.clone(), &Py::None))) {
    return Py::None;
    }
    return crate::gen::memory::amounts::the_comparison(&[against.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_count_said".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

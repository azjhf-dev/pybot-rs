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
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_count_said: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A count the person reports, read against the count the last answer named.\n\n\"how many steps should i do a day\" names a figure to aim at (\"about 7,000 to 8,000 steps a day\"),\nand the next thing people do is say how they are doing against it: \"i have about 7000 steps\". That\nline was answered with a search hedge, a receipt or a greeting, because the answer's own figure is\nsaid as a rate (\"7,000 to 8,000 steps a day\") and a rate is not read as an amount to compare with.\n\n`bot/memory/amounts.py` reads the two counts. This module is the thin part: one line in, one line\nout, and it hands the line back (None) whenever the counts are not the same thing said the same\nway - a count of something else, a question about an amount, a length that belongs to something\nelse (\"2 hours of meetings\"), or an answer naming two figures of that thing.\n\nIt answers **only** the turn that reports the count. The turn after it (\"how much more do i need\",\n\"is that enough\") belongs to the store, which already reads it and keeps its own guards, so\nnothing here is kept in `memory` and nothing is claimed twice.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_count_said.PRIORITY"); }
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
    said = pyrt::xrefs::xcall("memory.amounts", "a_count_they_report", &[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    last = if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { turns.clone() } }) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { py_str("") };
    against = pyrt::xrefs::xcall("memory.amounts", "against_the_last_answer", &[said.clone(), { let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(py_is(&against.clone(), &Py::None))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("memory.amounts", "the_comparison", &[against.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_count_said".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "amounts" => Some(pyrt::xrefs::xmodule("memory.amounts")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_count_said", module_obj);
    pyrt::xrefs::reg_fn("modules.a_count_said", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.a_count_said", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_count_said", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

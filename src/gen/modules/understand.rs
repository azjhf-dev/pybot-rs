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
    let _ = py_str("Runs the understanding engine (bot/understanding) before any module proposes a reply.");
    s_PRIORITY(Py::Int(10000));
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
    let mut filled: Py = Py::None;
    let mut read: Py = Py::None;
    read = crate::gen::understanding::understand(&[text.clone(), memory.clone()]);
    filled = py_call(&crate::gen::memory::g_the_thing_it_is(), &[read.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&filled.clone(), &Py::None))) {
    let mut the_advice_ask_with_its_doing: Py = py_func("the_advice_ask_with_its_doing", |args: &[Py]| crate::gen::memory::topics::the_advice_ask_with_its_doing(args));
    filled = py_call(&the_advice_ask_with_its_doing, &[read.clone(), memory.clone()]);
    }
    let _ = py_setitem(&memory.clone(), &py_str("understood"), if py_truthy(&Py::Bool(!py_is(&filled.clone(), &Py::None))) { filled.clone() } else { read.clone() });
    let _ = py_setitem(&geti(&memory.clone(), &py_str("understood")), &py_str("looks_back"), crate::gen::memory::recall::the_earlier_turn(&[text.clone(), memory.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.understand".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            _ => None,
        }),
        set: None,
    }))
}

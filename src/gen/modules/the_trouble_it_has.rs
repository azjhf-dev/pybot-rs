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
    let _ = py_str("One question back about the thing that is not working, instead of a stall.\n\n\"my airpods are not showing up\" and \"my laptop is slow\" are statements of a trouble, and the bot\nanswers them with \"Got it. Is there anything you'd like a hand with?\" or a search, which fits any\nmessage at all. The question a friend asks instead is the same one for a boiler, a car and a\nlaptop - is it every time, and did anything change - and it is a question the bot can ask without\nknowing a single fact about the thing.\n\n`bot/compose/a_trouble.py` finds the thing in the person's own words and the question that goes\nwith it. This module sits in the band `where_it_answers` uses - above greeting's hedge, below\nevery card - and stands down unless the composer's own reply says nothing about the line.\n");
    s_PRIORITY(neg(&Py::Int(999)));
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
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Ask about the trouble the person named, or say nothing and let the composer answer.");
    clean = crate::gen::compose::fallback::_matchable(&[crate::gen::compose::fallback::_plain(&[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }])]);
    pool = crate::gen::compose::a_trouble::trouble_pool(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    said = py_call(&crate::gen::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::a_trouble::says_nothing_about_the_line(&[said.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])])))) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.the_trouble_it_has".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

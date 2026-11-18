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
    let _ = py_str("A statement that names something the person is deciding gets help, not a receipt.\n\n\"im looking at an electric blanket for the winter\", \"we are looking at a venue for the wedding\",\n\"its for a warehouse supervisor role\": nobody asked a question, so nothing is looked up, and the\nseed fallback answers with one of its three receipts - \"Got it. Is there anything you'd like a\nhand with?\". That is the right answer to \"the dog did a funny thing today\" and the wrong answer to\nsomebody standing in front of a decision: they have said which decision, and the bot has said\nnothing about it.\n\nThe reply is built in `bot/compose/a_thing_being_weighed.py`: the kind of decision is read off the\nwords, the thing is said back in the words the person used, and the two or three things that settle\nthat kind of decision come with the one question a friend would ask next. Nothing is looked up and\nno price, model or date is named, so no line here can go stale.\n\nWhy this is a module of its own rather than one more reader inside the seed fallback: a reader\nadded to `bot/modules/greeting.py` has to be inserted at the end of a chain other lanes add to\nevery few minutes, so it cannot be landed without a rebase, and a rebase that re-cuts somebody\nelse's reader is how a fallback loses a turn it had. As a module it is one new file, and the rule\nit needs is the one the chain already has: a receipt answers when nothing else does.\n\nIt sits at -999, above the seed fallback (-1000) and beside the other modules that replace one of\nits lines (`bulky`, `a_day_somebody_else_keeps`). It asks the seed fallback for the reply it would\ngive (`greeting.respond`, the same turn the runtime would have run) and answers only where that\nreply is one of the three receipts in `fallback_data.NEUTRAL`, so every card, every entry and\nevery reader's own answer keeps its turn: `looking at a new boiler` is still the boiler-age card's,\n`looking at an air fryer` is still the air fryer's.\n\nThe reading is narrow on purpose and passes on almost every line: it needs a thing named, the kind\nread off the last word of the name, and no reason to think the thing is already theirs\n(\"im thinking of selling my car\").\n");
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
    let mut reply: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::a_thing_being_weighed::pool(&[text.clone()])))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    reply = crate::gen::modules::greeting::respond(&[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    return crate::gen::compose::a_thing_being_weighed::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_thing_being_weighed".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

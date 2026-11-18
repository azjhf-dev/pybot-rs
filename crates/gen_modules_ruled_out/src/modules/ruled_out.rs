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
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.ruled_out: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing the person rules out is answered against the answer they just got.\n\nAfter \"what should i watch tonight\" the bot offered three films, and \"not a series\" got a fresh\nlist with two series in it; after \"how do i sleep better\", \"not tablets\" reached the repeat\nprescription card; after \"what car should i buy\", \"not a diesel\" got the misfuelling paragraph.\nThose are the plainest form of \"the bot did not listen to itself\": the person is steering the\nanswer they have, and what they ruled out was never in it.\n\n`bot/compose/ruledout.py` holds the reading and the wording; this module is here so the claim has\na band of its own. The hook is the one this shape needs: the line has nothing in it that a topic\nentry can match, so every module that proposes on it is guessing, and the entry that answered the\nthing in the first place is the one that gets it wrong - \"not a diesel\" is a car line, and the\nanswer to it is not the card about putting petrol in a diesel tank.\n\nA line whose thing the answer does name is left to the entry that gave the answer, and a line that\nnames no thing at all (`nothing that needs much water`, `not really`) is left alone: `respond`\nreturns None on every other line.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.ruled_out.PRIORITY"); }
    s_PRIORITY(op_neg(&Py::Int(450)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.ruled_out.TAKES_AS_IT_COMES"); }
    s_TAKES_AS_IT_COMES(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
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
    let _ = py_str("The thing ruled out, said back against the answer, or None.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = pyrt::xrefs::xcall("compose.ruledout", "reply_for", &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } }]);
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
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("Only on a line that is nothing but the rejection of something the answer never offered.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[pyrt::xrefs::xcall("compose.ruledout", "takes_as_it_comes", &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.ruled_out".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "ruledout" => Some(pyrt::xrefs::xmodule("compose.ruledout")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.ruled_out", module_obj);
    pyrt::xrefs::reg_fn("modules.ruled_out", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.ruled_out", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("modules.ruled_out", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.ruled_out", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
}

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


thread_local! { static G_A_RECEIPT_OF_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.first_line_open: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_RECEIPT_OF_NOTHING() -> Py { ensure_init(); G_A_RECEIPT_OF_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECEIPT_OF_NOTHING(v: Py) { ensure_init(); G_A_RECEIPT_OF_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_OPEN() -> Py { ensure_init(); G_COLD_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_OPEN(v: Py) { ensure_init(); G_COLD_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A first line is not receipted for something never said.\n\nTwo of the composer's three floor lines say the bot has taken something in (\"Got it. Is there\nanything you'd like a hand with?\", \"Thanks for telling me - I'm here if you want to dig into any of\nthat\"). A chat's first line is the one turn where nothing has been said yet, so the receipt claims\na turn that did not happen: thirty of the deck's nine hundred first lines got one of the two.\n\nOn the first turn only, a receipt with nothing behind it is replaced by an opener that says nothing\nabout the line and asks for the thing an answer would start from. This module sits above `greeting`\n(-1000) and below every module that answers a line of its own, and it asks greeting what it would\nsay, so a card, a place named in the answer, an ask for hands and a limit all keep the turn\n(\"the laundry needs doing\" keeps the chore card, a fence down in the wind keeps the damage card,\n\"my glucose is 2.9\" keeps the hypo card). The same receipt is right on every later turn, where\nthere is a turn behind it.\n");
    s_A_RECEIPT_OF_NOTHING(pyrt::xrefs::xget("compose.fallback_data", "A_RECEIPT_OF_NOTHING"));
    s_COLD_OPEN(pyrt::xrefs::xget("compose.fallback_data", "COLD_OPEN"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.first_line_open.PRIORITY"); }
    s_PRIORITY(op_neg(&Py::Float(999.5)));
}
pub fn _has_a_turn_behind_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut one: Py = Py::None;
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    let mut clean: Py = Py::None;
    let mut reply: Py = Py::None;
    if py_truthy(&_has_a_turn_behind_it(&[memory.clone()])) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    reply = pyrt::xrefs::xcall("modules.greeting", "respond", &[text.clone(), memory.clone()]);
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RECEIPT_OF_NOTHING(), "search", &[call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[g_COLD_OPEN(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.first_line_open".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_RECEIPT_OF_NOTHING" => Some(g_A_RECEIPT_OF_NOTHING()),
            "COLD_OPEN" => Some(g_COLD_OPEN()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_has_a_turn_behind_it" => Some(py_func("_has_a_turn_behind_it", |args: &[Py]| _has_a_turn_behind_it(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "greeting" => Some(pyrt::xrefs::xmodule("modules.greeting")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_RECEIPT_OF_NOTHING" => s_A_RECEIPT_OF_NOTHING(v),
            "COLD_OPEN" => s_COLD_OPEN(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.first_line_open", module_obj);
    pyrt::xrefs::reg_fn("modules.first_line_open", "_has_a_turn_behind_it", py_func("_has_a_turn_behind_it", |args: &[Py]| _has_a_turn_behind_it(args)));
    pyrt::xrefs::reg_fn("modules.first_line_open", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.first_line_open", "A_RECEIPT_OF_NOTHING", g_A_RECEIPT_OF_NOTHING, s_A_RECEIPT_OF_NOTHING);
    pyrt::xrefs::reg_access("modules.first_line_open", "COLD_OPEN", g_COLD_OPEN, s_COLD_OPEN);
    pyrt::xrefs::reg_access("modules.first_line_open", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

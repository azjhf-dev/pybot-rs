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


thread_local! { static G_A_BREAK_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ILL_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.breakup: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_BREAK_UP() -> Py { ensure_init(); G_A_BREAK_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BREAK_UP(v: Py) { ensure_init(); G_A_BREAK_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ILL_POOLS() -> Py { ensure_init(); G_THE_ILL_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ILL_POOLS(v: Py) { ensure_init(); G_THE_ILL_POOLS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A break-up is not a broken bone, so the question after it is not about a ward.\n\n`A_MEDICAL_THREAD` counts a body, a bed or a thing that happened to one, and `broke` and `broken`\nare in it because a broken leg is a body's. `broke up` is the same two words with the ending of a\nrelationship on them, so a thread that said it read as an injured body and the ask that follows a\nrough patch - \"is she going to be ok\" - was answered with the pool for somebody ill:\n\n    my mum is having a hard time since her and dad broke up\n    is she going to be ok\n    >>> I would be guessing, and this is not a thing to guess about. Ask whoever is treating\n        them, or ring 111.\n\nNobody is treating her. `fallback._a_hard_time_and_not_a_body` already keeps the ward pools off a\nrough patch, and it asks `A_MEDICAL_THREAD` whether there is a body in the thread - so the one word\n`up` decided it. The fix lives here rather than in that regex because `bot/compose/fallback.py` is\nover the size a change may be submitted at, and this module only ever replaces a reply already\nchosen from the pools that name a patient: no other reply is touched, and no line is answered that\nthe composer got right.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.breakup.THE_ILL_POOLS"); }
    s_THE_ILL_POOLS(op_add(&pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HOW_IT_GOES_FOR_SOMEONE"), &pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HOW_LONG_IN_HOSPITAL")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.breakup.A_BREAK_UP"); }
    s_A_BREAK_UP(stdlib_call("re", "compile", &[py_str("\\b(?:broke|broken|splitting|split|breaking)\\s+up\\b|\\bdivorce\\w*\\b|\\bseparation\\b|\\bseparated\\b|\\bwalked\\s+out\\b|\\bleft\\s+(?:me|him|her|them)\\b|\\bsplit\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _thread_said(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last few turns of the talk, the bot's own included, as one lower-case string.");
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(3)), &Py::None, &Py::None);
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    let __aug1 = op_add(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() });
    said = __aug1;
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    return mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[]);
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut c: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut without_the_split: Py = Py::None;
    let _ = py_str("The hard-patch answer where the ill-somebody pool was reached by a break-up, else None.\n\n    Only a reply from one of those pools is replaced, and only when taking the break-up words out\n    of the thread leaves nothing medical behind: `she broke her hip` / `is she going to be ok`\n    keeps the ward, and so does a thread that still says hospital or chemo somewhere.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THE_ILL_POOLS(), &reply.clone())) } }) {
    return Py::None;
    }
    said = _thread_said(&[memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_BREAK_UP(), "search", &[said.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "A_HARD_TIME"), "search", &[said.clone()])))) {
    return Py::None;
    }
    without_the_split = mcall(&g_A_BREAK_UP(), "sub", &[py_str(" "), said.clone()]);
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "A_MEDICAL_THREAD"), "search", &[without_the_split.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "AN_ILLNESS"), "search", &[without_the_split.clone()])) {
    return Py::None;
    }
    pool = pyrt::xrefs::xget("compose.fallback", "UNKNOWN_HOW_IT_GOES_FOR_A_HARD_TIME");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    last = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.breakup".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BREAK_UP" => Some(g_A_BREAK_UP()),
            "THE_ILL_POOLS" => Some(g_THE_ILL_POOLS()),
            "_thread_said" => Some(py_func("_thread_said", |args: &[Py]| _thread_said(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "random" => Some(stdlib_module("random")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BREAK_UP" => s_A_BREAK_UP(v),
            "THE_ILL_POOLS" => s_THE_ILL_POOLS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.breakup", module_obj);
    pyrt::xrefs::reg_fn("compose.breakup", "_thread_said", py_func("_thread_said", |args: &[Py]| _thread_said(args)));
    pyrt::xrefs::reg_fn("compose.breakup", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.breakup", "A_BREAK_UP", g_A_BREAK_UP, s_A_BREAK_UP);
    pyrt::xrefs::reg_access("compose.breakup", "THE_ILL_POOLS", g_THE_ILL_POOLS, s_THE_ILL_POOLS);
}

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


thread_local! { static G_A_DATE_SAID_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MARKER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_DATE_SAID_NOW() -> Py { ensure_init(); G_A_DATE_SAID_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_SAID_NOW(v: Py) { ensure_init(); G_A_DATE_SAID_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MARKER_SAID() -> Py { ensure_init(); G_A_MARKER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MARKER_SAID(v: Py) { ensure_init(); G_A_MARKER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A trip already on record, said again with the month they now mean.\n\n    im going to spain in june\n    actually we fly in september        ->  Got it - you're going to Spain in September.\n    when do we fly                      ->  In September.\n\nIt also reads the version with the reason for the change after it, which names the date with no\ngoing verb at all: \"im going to lisbon in may\" / \"actually its june now, my sister moved her\nholiday\" is the same move, said the way people say it when they explain themselves.\n\nThe line names how they travel rather than the date alone, so the words are unremarkable - a\ntrip being told sounds the same. Only the store can tell the two apart, because a date is a\ncorrection just when there is a trip for it to move: with no plan on record the same line is a\ntrip being told, and a birthday, a dentist appointment or a fear of flying in the same chat is\nnot the trip the line is about.\n\nThe store's own reader cannot be extended for it (its file is at the world's size limit), so the\ndate is read here, by bot/memory/facts.py, and handed to the store's correction path - which owns\nthe plan, the acknowledgement and the fact log. This module only decides that the line is a\ncorrection of the trip on record.\n");
    s_PRIORITY(Py::Int(6900));
    s_A_DATE_SAID_NOW(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:ok|okay|right|well|so|and|no|nope|sorry|actually|oops|wait|ah|oh|hmm)[,.]?\\s+){0,4}(?:(?:it'?s|its|it\\s+is|that'?s|thats)\\s+)?"), &gen_memory::memory::facts::g_A_DATE_VALUE()), &py_str("\\s+now\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MARKER_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:no|nope|sorry|actually|oops|wait|ah|oh|hmm)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_date_said_now(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The date a line moves the trip to when it says the reason in the same breath, or \"\".\n\n    A question moves nothing (\"is it june now?\"), and the value has to read as a date once it is\n    read - \"actually its done now\" and \"sorry its broken now\" name no date. The words in front are\n    the ones the trip readers take, so the two spellings of the same move cannot drift apart.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &py_str("?")))) {
    return py_str("");
    }
    found = mcall(&g_A_DATE_SAID_NOW(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_MARKER_SAID(), "search", &[slice_or_raise(&line.clone(), &Py::None, &mcall(&found.clone(), "start", &[Py::Int(1)]), &Py::None)])))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&mcall(&found.clone(), "groups", &[]));
    for __item in __iter1 {
    group = __item.clone();
    if py_truthy(&{ let __b = group.clone(); if !py_truthy(&__b) { __b } else { gen_memory::memory::facts::_a_date(&[group.clone()]) } }) {
    return gen_memory::memory::facts::_tidy(&[group.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_first_clause(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let _ = py_str("The line up to its first comma, when that leaves a clause to read.\n\n    A change is often said first and explained after it: \"actually we fly in september, my sister\n    moved her holiday\". The trip reader finishes at the end of the line, so the explanation stopped\n    it - and the explanation is never the part that names the date.\n    ");
    head = mcall(&geti(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[]);
    return if py_truthy(&op_ge(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { head.clone() } else { py_str("") };
    Py::None
}
pub fn _the_trip_it_moves(args: &[Py]) -> Py {
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
    let mut on_record: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut trip: Py = Py::None;
    let _ = py_str("The plan the date is about, or None when there is no trip for it to move.\n\n    A birthday or an appointment can be the newest plan in the store, so the trip on record has\n    to be the plan in hand as well: its own words have to be the ones the chat has up front.\n    ");
    plan = { let __b = py_call(&crate::modules::memory_store::g__stated_plan(), &[memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&py_call(&crate::modules::memory_store::g_user(), &[memory.clone()]), "get", &[py_str("plan")]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&plan.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    trip = gen_memory::memory::trips::_the_trip_so_far(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&trip.clone()))) {
    return Py::None;
    }
    on_record = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&trip.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&on_record.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&on_record.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&plan.clone(), "lower", &[]), "split", &[])])))) {
    return plan.clone();
    }
    said_before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::modules::memory_store::g_user(), &[memory.clone()]), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&said_before.clone(), &on_record.clone()))) {
    return mcall(&trip.clone(), "get", &[py_str("plan")]);
    }
    return Py::None;
    Py::None
}
pub fn _the_month_taken_back(args: &[Py]) -> Py {
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
    let mut month: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trip: Py = Py::None;
    let _ = py_str("The month the trip on record says, when the store keeps a plan that says it.\n\n    The store's correction path reads a date before it reads a plan, and a birthday in the same\n    chat is a date of its own, so a bare month would be moved in the birthday and not in the\n    trip. Naming the month the trip says puts the reader on the plan that says it - and when no\n    plan says it, nothing is moved rather than the wrong date.\n    ");
    trip = { let __b = gen_memory::memory::trips::_the_trip_so_far(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    month = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&trip.clone(), "get", &[py_str("month")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&month.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::modules::memory_store::g_user(), &[memory.clone()]), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    said = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &month.clone()))) {
    return month.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_dated_fact_beside_it(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("Whether someone else's dated fact - a birthday, an appointment - is in the store.\n\n    Such a fact is a date of its own, and the store reads a date before it reads a plan, so a bare\n    month can be moved in the birthday rather than in the trip. Only then does the month have to\n    be named.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    other = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[other.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("birthday"), py_str("appointment"), py_str("when"), py_str("date")])) { let mut key: Py = __item.clone(); __out.push(mcall(&other.clone(), "get", &[key.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
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
    let mut moved: Py = Py::None;
    let mut taken_back: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    moved = { let __b = gen_memory::memory::facts::a_trip_date_put_right(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_memory::memory::facts::a_trip_date_put_right(&[_the_first_clause(&[text.clone()])]); if py_truthy(&__b) { __b } else { _a_date_said_now(&[text.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&moved.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_trip_it_moves(&[memory.clone()])))) {
    return Py::None;
    }
    taken_back = if py_truthy(&_a_dated_fact_beside_it(&[memory.clone()])) { _the_month_taken_back(&[memory.clone()]) } else { Py::None };
    let _ = py_call(&crate::modules::memory_store::g__apply_correction(), &[memory.clone(), py_dict(vec![(py_str("to"), moved.clone()), (py_str("from"), taken_back.clone())]), text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.trip_date_said_again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DATE_SAID_NOW" => Some(g_A_DATE_SAID_NOW()),
            "A_MARKER_SAID" => Some(g_A_MARKER_SAID()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_a_date_said_now" => Some(py_func("_a_date_said_now", |args: &[Py]| _a_date_said_now(args))),
            "_a_dated_fact_beside_it" => Some(py_func("_a_dated_fact_beside_it", |args: &[Py]| _a_dated_fact_beside_it(args))),
            "_the_first_clause" => Some(py_func("_the_first_clause", |args: &[Py]| _the_first_clause(args))),
            "_the_month_taken_back" => Some(py_func("_the_month_taken_back", |args: &[Py]| _the_month_taken_back(args))),
            "_the_trip_it_moves" => Some(py_func("_the_trip_it_moves", |args: &[Py]| _the_trip_it_moves(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DATE_SAID_NOW" => s_A_DATE_SAID_NOW(v),
            "A_MARKER_SAID" => s_A_MARKER_SAID(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.trip_date_said_again", module_obj);
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "_a_date_said_now", py_func("_a_date_said_now", |args: &[Py]| _a_date_said_now(args)));
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "_the_first_clause", py_func("_the_first_clause", |args: &[Py]| _the_first_clause(args)));
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "_the_trip_it_moves", py_func("_the_trip_it_moves", |args: &[Py]| _the_trip_it_moves(args)));
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "_the_month_taken_back", py_func("_the_month_taken_back", |args: &[Py]| _the_month_taken_back(args)));
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "_a_dated_fact_beside_it", py_func("_a_dated_fact_beside_it", |args: &[Py]| _a_dated_fact_beside_it(args)));
    pyrt::xrefs::reg_fn("modules.trip_date_said_again", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_access("modules.trip_date_said_again", "A_DATE_SAID_NOW", g_A_DATE_SAID_NOW, s_A_DATE_SAID_NOW);
    pyrt::xrefs::reg_access("modules.trip_date_said_again", "A_MARKER_SAID", g_A_MARKER_SAID, s_A_MARKER_SAID);
    pyrt::xrefs::reg_access("modules.trip_date_said_again", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

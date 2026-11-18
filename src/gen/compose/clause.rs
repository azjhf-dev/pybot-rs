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


pub static NOTHING_TO_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_LOOK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OPENS_A_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SHAPES_FROM_A_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_NOTHING_TO_A_THING() -> Py { ensure_init(); NOTHING_TO_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_TO_A_THING.write() { *w = v; } }
pub fn g_WHAT_TO_LOOK() -> Py { ensure_init(); WHAT_TO_LOOK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_LOOK(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_LOOK.write() { *w = v; } }
pub fn g__OPENS_A_QUESTION() -> Py { ensure_init(); _OPENS_A_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OPENS_A_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = _OPENS_A_QUESTION.write() { *w = v; } }
pub fn g__SHAPES_FROM_A_CLAUSE() -> Py { ensure_init(); _SHAPES_FROM_A_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SHAPES_FROM_A_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = _SHAPES_FROM_A_CLAUSE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The question is the clause after the comma.\n\nPeople lead with the situation and ask at the end: \"camping, might rain, what to pack\", \"5 foot 2,\ntrousers for work, what do i look for when buying a pair\". The understanding engine already splits\nthe line - `understood[\"focus\"]` is the clause after the last comma - but the shape of answer is\nread from the whole line, and a lead-in can change what that reading looks like: \"might rain\"\ncarries an auxiliary, so the line reads as a yes/no, and the person who asked what to pack is owed\na list rather than a verdict.\n\nTwo things are read here, and only where the whole-line reading is weak:\n\n- the clause after the comma, when it is a question of its own (`what to pack`, `what do i need`,\n  `what do i look for`) and the line is one clause no longer;\n- a thing the person has not got (\"wedding in september, nothing to wear\"), which is the thing they\n  are asking for.\n\nA line that already asks for a shape of its own keeps it: this can only lift a reading that came\nback as nothing or as a yes/no, never overrule a time, an amount or a place. `bot/modules/intent.py`\ncalls `shape()` after `fit.wants()`, and the arbiter reads the result as it always has.\n");
    s_WHAT_TO_LOOK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:to|(?:do|should|shall|can|could|must)\\s+(?:i|we))\\s+(?:need to\\s+|have to\\s+|ought to\\s+)?(?:look(?!\\s+like\\b)(?:\\s+(?:out\\s+|around\\s+)?for)?|check(?:\\s+for)?|watch\\s+out\\s+for|ask(?:\\s+for)?|avoid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_TO_A_THING(stdlib_call("re", "compile", &[py_str("\\bnothing (?:to|for) (?:wear|put on|bring|take|pack|cook|make|eat|have|drink|watch|read|give|buy|get|play)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_A_QUESTION(py_tuple(vec![py_str("what"), py_str("which"), py_str("how")]));
    s__SHAPES_FROM_A_CLAUSE(py_tuple(vec![py_str("pick"), py_str("options"), py_str("steps")]));
}
pub fn _said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _focus(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut focus: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause after the last comma, or None when the line is one clause.");
    focus = mcall(&read.clone(), "get", &[py_str("focus")]);
    if py_truthy(&call_builtin("isinstance", &[focus.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    focus = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[focus.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    focus = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&focus.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&focus.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&focus.clone(), &said.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&gt(&call_builtin("len", &[focus.clone()]).unwrap_or_else(|e| exc::raise(e)), &mul(&Py::Float(0.85), &call_builtin("max", &[call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return focus.clone();
    Py::None
}
pub fn the_clause_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut focus: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The shape asked in the clause after the comma, or None when the clause asks for none.\n\n    Only the asks a clause can carry on its own are read - \"what to cook, wear, pack or buy\", \"what\n    to do\", \"what do i need\", \"what do i look for\" - so nothing else is invented from it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = _said(&[read.clone()]);
    focus = _focus(&[read.clone(), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&focus.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&focus.clone(), "startswith", &[g__OPENS_A_QUESTION()]))) } }) {
    return Py::None;
    }
    focus = add(&add(&py_str(" "), &focus.clone()), &py_str(" "));
    if py_truthy(&mcall(&crate::gen::compose::fit::g_WHAT_TO_PICK(), "search", &[focus.clone()])) {
    return py_str("pick");
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fit::g_WHAT_I_NEED(), "search", &[focus.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fit::g_WHAT_YOU_TAKE(), "search", &[focus.clone()]) } }) {
    return py_str("options");
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fit::g_WHAT_TO_STEPS(), "search", &[focus.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fit::g_WHAT_TO_DO(), "search", &[focus.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHAT_TO_LOOK(), "search", &[focus.clone()]) } } } }) {
    return py_str("steps");
    }
    return Py::None;
    Py::None
}
pub fn shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut said: Py = Py::None;
    let _ = py_str("`want`, lifted to the shape the clause after the comma asks, or left alone.\n\n    A shape the whole line already gave - a time, an amount, a place, or a pick, options or steps\n    of its own - is kept: the clause is only read where the line came back as nothing or as a\n    yes/no a lead-in put there.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__SHAPES_FROM_A_CLAUSE(), &want.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("amount"), py_str("place"), py_str("person"), py_str("meaning"), py_str("reason"), py_str("number"), py_str("clock"), py_str("price")]), &want.clone())) } }) {
    return want.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return want.clone();
    }
    said = _said(&[read.clone()]);
    if py_truthy(&{ let __b = mcall(&g_NOTHING_TO_A_THING(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let __b = mcall(&read.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("list"))) } }) {
    return py_str("pick");
    }
    if py_truthy(&mcall(&g_WHAT_TO_LOOK(), "search", &[add(&add(&py_str(" "), &said.clone()), &py_str(" "))])) {
    return py_str("steps");
    }
    return { let __b = the_clause_ask(&[read.clone()]); if py_truthy(&__b) { __b } else { want.clone() } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.clause".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "NOTHING_TO_A_THING" => Some(g_NOTHING_TO_A_THING()),
            "WHAT_TO_LOOK" => Some(g_WHAT_TO_LOOK()),
            "_OPENS_A_QUESTION" => Some(g__OPENS_A_QUESTION()),
            "_SHAPES_FROM_A_CLAUSE" => Some(g__SHAPES_FROM_A_CLAUSE()),
            "_focus" => Some(py_func("_focus", |args: &[Py]| _focus(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "shape" => Some(py_func("shape", |args: &[Py]| shape(args))),
            "the_clause_ask" => Some(py_func("the_clause_ask", |args: &[Py]| the_clause_ask(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static AFTER_WHICH_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_ABOUT_A_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SIDE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HURTING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHICH_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AFTER_WHICH_ONE() -> Py { ensure_init(); AFTER_WHICH_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_WHICH_ONE(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_WHICH_ONE.write() { *w = v; } }
pub fn g_ASKED_ABOUT_A_PART() -> Py { ensure_init(); ASKED_ABOUT_A_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_ABOUT_A_PART(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_ABOUT_A_PART.write() { *w = v; } }
pub fn g_A_SIDE_TAIL() -> Py { ensure_init(); A_SIDE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SIDE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_SIDE_TAIL.write() { *w = v; } }
pub fn g_BODY_PART() -> Py { ensure_init(); BODY_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_PART(v: Py) { ensure_init(); if let Ok(mut w) = BODY_PART.write() { *w = v; } }
pub fn g_BODY_PARTS() -> Py { ensure_init(); BODY_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = BODY_PARTS.write() { *w = v; } }
pub fn g_HURTING_ASK() -> Py { ensure_init(); HURTING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HURTING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = HURTING_ASK.write() { *w = v; } }
pub fn g_WHICH_PART() -> Py { ensure_init(); WHICH_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHICH_PART(v: Py) { ensure_init(); if let Ok(mut w) = WHICH_PART.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The part of them that hurts, and which one it is.\n\nThe store keeps one complaint of the person's in ``user[\"hurts\"]`` (the part, the side, the place\non it, and the part a put-right came from). These readers answer the lines about it. They live\nhere rather than in ``bot/modules/memory_store.py`` because that file is at the 1 MiB cap the\nworld enforces and a reader added to it cannot be submitted.\n");
    s_BODY_PARTS(crate::gen::memory::facts::g_BODY_PARTS());
    s_BODY_PART(crate::gen::memory::facts::g_BODY_PART());
    s_WHICH_PART(stdlib_call("re", "compile", &[add(&add(&py_str("\\bwhich (?:one|side|part|"), &mcall(&py_str("|"), "join", &[g_BODY_PARTS()])), &py_str(")\\b|\\bwhat side\\b|\\bleft or right\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_WHICH_ONE(stdlib_call("re", "compile", &[py_str("\\bwhich one\\b(.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SIDE_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str("was"), py_str("is"), py_str("it"), py_str("that"), py_str("did"), py_str("i"), py_str("say"), py_str("said"), py_str("then"), py_str("again"), py_str("exactly"), py_str("hurts"), py_str("hurt"), py_str("of"), py_str("them"), py_str("those"), py_str("the"), py_str("two"), py_str("please"), py_str("one"), py_str("side"), py_str("part"), py_str("was")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HURTING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:hurt|hurts|hurting|sore|ache|aching|pain|painful)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_ABOUT_A_PART(stdlib_call("re", "compile", &[py_str("^\\s*(?:who|what|which|when|where|why|how|does|do|did|is|are|was|should|shall|can|could|would|will|any)\\b|\\?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _vary(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fresh: Py = Py::None;
    let mut o: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("pick(), but over the whole reply: a lead-in and a body put together can repeat a line\n    the bot said, even when the pieces on their own do not.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(mcall(&t.clone(), "get", &[py_str("bot")])); } py_set(__out) })() };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&recent.clone(), &o.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    return mcall(&stdlib_call("random", "Random", &[call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    Py::None
}
pub fn which_one_is_a_side(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when \"which one\" is the whole of a question about choosing, not about a side.");
    found = mcall(&g_AFTER_WHICH_ONE(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_A_SIDE_TAIL(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn before(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let _ = py_str("The part the store holds before this line is read, carried across ``observe``.");
    held = { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("hurts")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("part")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    Py::None
}
pub fn after(args: &[Py]) -> Py {
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
    let mut held_before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held_before"))) };
    let mut held: Py = Py::None;
    let _ = py_str("A part named where another was held is a put-right, not a second ache.");
    held = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("hurts")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = held_before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("part")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &held_before.clone())) } } } }) {
    let _ = py_setitem(&held.clone(), &py_str("was"), held_before.clone());
    }
    Py::None
}
pub fn _a_question_about_a_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    return call_builtin("bool", &[mcall(&g_ASKED_ABOUT_A_PART(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn what_is_hurting_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut asked: Py = Py::None;
    let mut held: Py = Py::None;
    let mut one: Py = Py::None;
    let mut part: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut side: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut was: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("\"what did i say was hurting\": the part the store holds now, not an older line quoted.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_HURTING_ASK(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bsa(?:y|id)\\b|\\bwhat .*\\b(?:hurt|sore|pain)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    held = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("hurts")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("part")]))) } }) {
    previous = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = crate::gen::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&_a_question_about_a_part(&[previous.clone()])) {
    asked = mcall(&crate::gen::memory::facts::g_BODY_PART(), "search", &[mcall(&previous.clone(), "lower", &[])]);
    what = if py_truthy(&asked.clone()) { mcall(&mcall(&asked.clone(), "group", &[Py::Int(1)]), "lower", &[]) } else { py_str("it") };
    return _vary(&[py_list(vec![add(&add(&py_str("You asked about a sore "), &what.clone()), &py_str(" - you haven't told me about anything of your own hurting.")), add(&add(&py_str("Nothing of yours - you were asking about a sore "), &what.clone()), &py_str("."))]), memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::memory::facts::g_BODY_PART(), "search", &[mcall(&previous.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::facts::g_BODY_HURTS(), "search", &[mcall(&previous.clone(), "lower", &[])]) } }) {
    return crate::gen::memory::told::_quote(&[previous.clone(), memory.clone()]);
    }
    return _vary(&[py_list(vec![py_str("You haven't told me about anything hurting."), py_str("Nothing of yours is hurting as far as I know."), py_str("You haven't said that anything is hurting.")]), memory.clone()]);
    }
    part = call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("part")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    was = call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    side = call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("side")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    one = add(&add(&py_str("your "), &if py_truthy(&side.clone()) { add(&side.clone(), &py_str(" ")) } else { py_str("") }), &part.clone());
    verb = if py_truthy(&mcall(&part.clone(), "endswith", &[py_str("s")])) { py_str("were") } else { py_str("was") };
    if py_truthy(&was.clone()) {
    return _vary(&[py_list(vec![add(&add(&add(&add(&py_str("It is "), &one.clone()), &py_str(" - you put the ")), &was.clone()), &py_str(" right.")), add(&add(&add(&add(&mcall(&slice_or_raise(&one.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&one.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(", since you put the ")), &was.clone()), &py_str(" right.")), add(&add(&add(&add(&py_str("You said "), &one.clone()), &py_str(" in the end, not the ")), &was.clone()), &py_str("."))]), memory.clone()]);
    }
    return _vary(&[py_list(vec![add(&add(&add(&add(&py_str("It is "), &one.clone()), &py_str(" that you said ")), &verb.clone()), &py_str(" hurting.")), add(&add(&mcall(&slice_or_raise(&one.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&one.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" - that is the one.")), add(&add(&py_str("You said "), &one.clone()), &py_str("."))]), memory.clone()]);
    Py::None
}
pub fn the_part_asked(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut asked: Py = Py::None;
    let mut held: Py = Py::None;
    let mut part: Py = Py::None;
    let mut place: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut said: Py = Py::None;
    let mut side: Py = Py::None;
    let _ = py_str("The part of them that is sore, said back to a question about which one it is.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WHICH_PART(), "search", &[low.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&which_one_is_a_side(&[low.clone()])))) {
    return Py::None;
    }
    held = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("hurts")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("part")]))) } }) {
    return Py::None;
    }
    part = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("part")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    asked = mcall(&g_BODY_PART(), "search", &[low.clone()]);
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![part.clone(), mcall(&part.clone(), "rstrip", &[py_str("s")]), add(&part.clone(), &py_str("s"))]), &mcall(&mcall(&asked.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    previous = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::gen::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&previous.clone(), &part.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&previous.clone(), &mcall(&part.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::None;
    }
    }
    side = mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("side")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    place = mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("place")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&side.clone()) {
    said = _vary(&[py_list(vec![add(&add(&add(&add(&mcall(&slice_or_raise(&side.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&side.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &part.clone()), &py_str(".")), add(&add(&add(&add(&py_str("Your "), &side.clone()), &py_str(" ")), &part.clone()), &py_str(" - that is what you said.")), add(&add(&py_str("The "), &side.clone()), &py_str(" one."))]), memory.clone()]);
    if py_truthy(&place.clone()) {
    said = _vary(&[py_list(vec![add(&add(&add(&add(&add(&add(&mcall(&slice_or_raise(&side.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&side.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &part.clone()), &py_str(", ")), &place.clone()), &py_str(" side.")), add(&add(&add(&add(&add(&add(&py_str("Your "), &side.clone()), &py_str(" ")), &part.clone()), &py_str(", the ")), &place.clone()), &py_str(" of it."))]), memory.clone()]);
    }
    return said.clone();
    }
    if py_truthy(&place.clone()) {
    return _vary(&[py_list(vec![add(&add(&py_str("The "), &place.clone()), &py_str(" of it - you did not say which side.")), add(&add(&add(&add(&py_str("The "), &place.clone()), &py_str(" of your ")), &part.clone()), &py_str(", you said. Left or right you did not say.")), add(&add(&py_str("You said the "), &place.clone()), &py_str(" of it. Not which side."))]), memory.clone()]);
    }
    return _vary(&[py_list(vec![add(&add(&py_str("You said your "), &part.clone()), &py_str(", without saying which side.")), add(&add(&py_str("Just your "), &part.clone()), &py_str(" - you did not say left or right."))]), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.whathurts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_WHICH_ONE" => Some(g_AFTER_WHICH_ONE()),
            "ASKED_ABOUT_A_PART" => Some(g_ASKED_ABOUT_A_PART()),
            "A_SIDE_TAIL" => Some(g_A_SIDE_TAIL()),
            "BODY_PART" => Some(g_BODY_PART()),
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "HURTING_ASK" => Some(g_HURTING_ASK()),
            "WHICH_PART" => Some(g_WHICH_PART()),
            "_a_question_about_a_part" => Some(py_func("_a_question_about_a_part", |args: &[Py]| _a_question_about_a_part(args))),
            "_vary" => Some(py_func("_vary", |args: &[Py]| _vary(args))),
            "after" => Some(py_func("after", |args: &[Py]| after(args))),
            "before" => Some(py_func("before", |args: &[Py]| before(args))),
            "the_part_asked" => Some(py_func("the_part_asked", |args: &[Py]| the_part_asked(args))),
            "what_is_hurting_reply" => Some(py_func("what_is_hurting_reply", |args: &[Py]| what_is_hurting_reply(args))),
            "which_one_is_a_side" => Some(py_func("which_one_is_a_side", |args: &[Py]| which_one_is_a_side(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static A_BIRTH_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ALSO_AN_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BORN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOMEONE_ELSE_BORN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_BIRTH_YEAR() -> Py { ensure_init(); A_BIRTH_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BIRTH_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = A_BIRTH_YEAR.write() { *w = v; } }
pub fn g__ALSO_AN_AGE() -> Py { ensure_init(); _ALSO_AN_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ALSO_AN_AGE(v: Py) { ensure_init(); if let Ok(mut w) = _ALSO_AN_AGE.write() { *w = v; } }
pub fn g__A_BORN() -> Py { ensure_init(); _A_BORN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BORN(v: Py) { ensure_init(); if let Ok(mut w) = _A_BORN.write() { *w = v; } }
pub fn g__SOMEONE_ELSE_BORN() -> Py { ensure_init(); _SOMEONE_ELSE_BORN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_BORN(v: Py) { ensure_init(); if let Ok(mut w) = _SOMEONE_ELSE_BORN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A year of birth told in chat is the age, said the long way.\n\n    i was born in 1990\n    how old am i          -> main: \"You haven't told me your age yet.\"\n\nThe store has always answered \"how old am i\" from the age it was told (\"You told me you're 34\"),\nand it reads a figure after \"i am\" or \"i'm\". A year of birth is the same fact in other words, and\nnothing read it: the person says when they were born, asks their age a turn later, and is told\nthey never said.\n\nSo the year is filed as the age this chat would work out from it, and the store's own answer does\nthe rest - which is what the person hears in the wording the store already uses, not a second\nvoice saying the same thing in its own way.\n\nOnly the person's own year is taken. Somebody else's (\"my son was born in 2015\") is a fact about a\nperson the store keeps, and the readers that answer \"how old is my son\" own it. A line that gives\nboth keeps them apart: \"i was born in 1990 and im 35\" leaves the age it states.\n");
    s_A_BIRTH_YEAR(py_tuple(vec![py_str("Born in {born}, so you turn {turned} this year."), py_str("{born}. That makes you {turned} this year."), py_str("Noted - {turned} this year, going by {born}.")]));
    s__A_BORN(stdlib_call("re", "compile", &[py_str("\\b(?:i\\s+was\\s+born|i\\s+am\\s+born|im\\s+born|born)\\b[^0-9]{0,14}\\b(1[89]\\d\\d|20\\d\\d)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMEONE_ELSE_BORN(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|your|its)\\s+(?:\\w+\\s+){0,2}?(?:was|were|is|are)\\s+born\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ALSO_AN_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:i\\s?m|i\\s+am|im|i\\s+turned|i\\s+will\\s+be)\\s+(\\d{1,3})\\b(?!\\s*(?:years?\\s+(?:ago|from)|kg|pounds?|miles?|minutes?|hours?|days?|weeks?|months?))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _today(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    return stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_store(args: &[Py]) -> Py {
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
    let mut user: Py = py_func("user", |args: &[Py]| crate::gen::memory::user(args));
    return py_call(&user, &[memory.clone()]);
    Py::None
}
pub fn _a_slip(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut born: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line is only the year of birth, with no ask after it.");
    rest = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    born = mcall(&g__A_BORN(), "search", &[rest.clone()]);
    if py_truthy(&born.clone()) {
    rest = mcall(&add(&add(&slice_or_raise(&rest.clone(), &Py::None, &mcall(&born.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&rest.clone(), &mcall(&born.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    }
    return Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\?|\\b(?:how|what|when|where|why|who|which|should|can|could|would|do|does|did|is|are|will|any)\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The answer to a line that only says the year they were born, or None.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_BORN(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__SOMEONE_ELSE_BORN(), "search", &[said.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_slip(&[said.clone()]))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::None;
    }
    year = mcall(&memory.clone(), "get", &[py_str("born_year")]);
    let __try1 = exc::catch(|| {
    year = call_builtin("int", &[year.clone()]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    turned = sub(&gattr(&_today(&[]), "year"), &year.clone());
    let __try6 = exc::catch(|| {
    stated = mcall(&_the_store(&[memory.clone()]), "get", &[py_str("age")]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&stated.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { if py_truthy(&lt(&Py::Int(0), &{ let __cmp110 = call_builtin("int", &[stated.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp110.clone() })) { lt(&__cmp110.clone(), &Py::Int(130)) } else { Py::Bool(false) } } }) {
    turned = call_builtin("int", &[stated.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp120 = turned.clone(); __cmp120.clone() })) { lt(&__cmp120.clone(), &Py::Int(130)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::gen::compose::pick(args));
    return py_call(&pick, &[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_BIRTH_YEAR()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("born".to_string(), year.clone()), ("turned".to_string(), turned.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), said.clone()]);
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
    let mut born: Py = Py::None;
    let mut facts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut this_year: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("File a year of birth the line gives about the person as the age it works out to.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    born = mcall(&g__A_BORN(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&born.clone())); if py_truthy(&__b) { __b } else { mcall(&g__SOMEONE_ELSE_BORN(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    year = call_builtin("int", &[mcall(&born.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    this_year = gattr(&_today(&[]), "year");
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(1900), &{ let __cmp60 = year.clone(); __cmp60.clone() })) { le(&__cmp60.clone(), &this_year.clone()) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("born_year"), year.clone());
    if py_truthy(&mcall(&g__ALSO_AN_AGE(), "search", &[said.clone()])) {
    return Py::None;
    }
    turned = sub(&this_year.clone(), &year.clone());
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp70 = turned.clone(); __cmp70.clone() })) { lt(&__cmp70.clone(), &Py::Int(130)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    let __try8 = exc::catch(|| {
    facts = _the_store(&[memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["Exception"]) {
                __handled12 = true;
    return Py::None;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    if py_truthy(&Py::Bool(py_contains(&facts.clone(), &py_str("age")))) {
    return Py::None;
    }
    let mut remember: Py = py_func("remember", |args: &[Py]| crate::gen::memory::remember(args));
    let _ = py_call(&remember, &[memory.clone(), py_str("age"), turned.clone(), said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.a_year_of_birth".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BIRTH_YEAR" => Some(g_A_BIRTH_YEAR()),
            "_ALSO_AN_AGE" => Some(g__ALSO_AN_AGE()),
            "_A_BORN" => Some(g__A_BORN()),
            "_SOMEONE_ELSE_BORN" => Some(g__SOMEONE_ELSE_BORN()),
            "_a_slip" => Some(py_func("_a_slip", |args: &[Py]| _a_slip(args))),
            "_the_store" => Some(py_func("_the_store", |args: &[Py]| _the_store(args))),
            "_today" => Some(py_func("_today", |args: &[Py]| _today(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            _ => None,
        }),
        set: None,
    }))
}

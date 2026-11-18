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


thread_local! { static G_A_BIRTH_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALSO_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BORN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE_BORN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BIRTH_YEAR() -> Py { ensure_init(); G_A_BIRTH_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BIRTH_YEAR(v: Py) { ensure_init(); G_A_BIRTH_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__ALSO_AN_AGE() -> Py { ensure_init(); G__ALSO_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALSO_AN_AGE(v: Py) { ensure_init(); G__ALSO_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BORN() -> Py { ensure_init(); G__A_BORN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BORN(v: Py) { ensure_init(); G__A_BORN.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE_BORN() -> Py { ensure_init(); G__SOMEONE_ELSE_BORN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_BORN(v: Py) { ensure_init(); G__SOMEONE_ELSE_BORN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A year of birth told in chat is the age, said the long way.\n\n    i was born in 1990\n    how old am i          -> main: \"You haven't told me your age yet.\"\n\nThe store has always answered \"how old am i\" from the age it was told (\"You told me you're 34\"),\nand it reads a figure after \"i am\" or \"i'm\". A year of birth is the same fact in other words, and\nnothing read it: the person says when they were born, asks their age a turn later, and is told\nthey never said.\n\nSo the year is filed as the age this chat would work out from it, and the store's own answer does\nthe rest - which is what the person hears in the wording the store already uses, not a second\nvoice saying the same thing in its own way.\n\nOnly the person's own year is taken. Somebody else's (\"my son was born in 2015\") is a fact about a\nperson the store keeps, and the readers that answer \"how old is my son\" own it. A line that gives\nboth keeps them apart: \"i was born in 1990 and im 35\" leaves the age it states.\n");
    s_A_BIRTH_YEAR(py_tuple(vec![py_str("Born in {born}, so you turn {turned} this year."), py_str("{born}. That makes you {turned} this year."), py_str("Noted - {turned} this year, going by {born}.")]));
    s__A_BORN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i\\s+was\\s+born|i\\s+am\\s+born|im\\s+born|born)\\b[^0-9]{0,14}\\b(1[89]\\d\\d|20\\d\\d)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SOMEONE_ELSE_BORN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|his|her|their|our|your|its)\\s+(?:\\w+\\s+){0,2}?(?:was|were|is|are)\\s+born\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ALSO_AN_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i\\s?m|i\\s+am|im|i\\s+turned|i\\s+will\\s+be)\\s+(\\d{1,3})\\b(?!\\s*(?:years?\\s+(?:ago|from)|kg|pounds?|miles?|minutes?|hours?|days?|weeks?|months?))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "date.today", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut user: Py = py_func("user", |args: &[Py]| crate::memory::user(args));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&user, args))(&__args) };
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
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    born = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BORN(), "search", args))(&__args) };
    if py_truthy(&born.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&slice_or_raise(&rest.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&born.clone(), "start", args))(&__args) }, &Py::None), &py_str(" ")), &slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&born.clone(), "end", args))(&__args) }, &Py::None, &Py::None)), "strip", args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?|\\b(?:how|what|when|where|why|who|which|should|can|could|would|do|does|did|is|are|will|any)\\b")); __args.push(rest.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BORN(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_ELSE_BORN(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_slip(args))(&__args) })); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)) } }) {
    return Py::None;
    }
    year = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("born_year")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    let __try1 = exc::catch(|| {
    year = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(year.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    turned = op_sub(&gattr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _today(args))(&__args) }, "year"), &year.clone());
    let __try6 = exc::catch(|| {
    stated = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_store(args))(&__args) }, "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&stated.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __cmp111: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stated.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_lt(&Py::Int(0), &__cmp111)) { op_lt(&__cmp111, &Py::Int(130)) } else { Py::Bool(false) } } } }) {
    turned = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stated.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp121: Py = turned.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp121)) { op_lt(&__cmp121, &Py::Int(130)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    let mut pick: Py = py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_BIRTH_YEAR()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("born".to_string(), year.clone()), ("turned".to_string(), turned.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&pick, args))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    born = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BORN(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&born.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOMEONE_ELSE_BORN(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    year = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&born.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    this_year = gattr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _today(args))(&__args) }, "year");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp61: Py = year.clone(); if py_truthy(&op_le(&Py::Int(1900), &__cmp61)) { op_le(&__cmp61, &this_year.clone()) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("born_year"), year.clone());
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ALSO_AN_AGE(), "search", args))(&__args) }) {
    return Py::None;
    }
    turned = op_sub(&this_year.clone(), &year.clone());
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp71: Py = turned.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp71)) { op_lt(&__cmp71, &Py::Int(130)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    let __try8 = exc::catch(|| {
    facts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_store(args))(&__args) };
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
    let mut remember: Py = py_func("remember", |args: &[Py]| crate::memory::remember(args));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_str("age")); __args.push(turned.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&remember, args))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BIRTH_YEAR" => s_A_BIRTH_YEAR(v),
            "_ALSO_AN_AGE" => s__ALSO_AN_AGE(v),
            "_A_BORN" => s__A_BORN(v),
            "_SOMEONE_ELSE_BORN" => s__SOMEONE_ELSE_BORN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.a_year_of_birth", module_obj);
    pyrt::xrefs::reg_fn("memory.a_year_of_birth", "_today", py_func("_today", |args: &[Py]| _today(args)));
    pyrt::xrefs::reg_fn("memory.a_year_of_birth", "_the_store", py_func("_the_store", |args: &[Py]| _the_store(args)));
    pyrt::xrefs::reg_fn("memory.a_year_of_birth", "_a_slip", py_func("_a_slip", |args: &[Py]| _a_slip(args)));
    pyrt::xrefs::reg_fn("memory.a_year_of_birth", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_fn("memory.a_year_of_birth", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_access("memory.a_year_of_birth", "A_BIRTH_YEAR", g_A_BIRTH_YEAR, s_A_BIRTH_YEAR);
    pyrt::xrefs::reg_access("memory.a_year_of_birth", "_ALSO_AN_AGE", g__ALSO_AN_AGE, s__ALSO_AN_AGE);
    pyrt::xrefs::reg_access("memory.a_year_of_birth", "_A_BORN", g__A_BORN, s__A_BORN);
    pyrt::xrefs::reg_access("memory.a_year_of_birth", "_SOMEONE_ELSE_BORN", g__SOMEONE_ELSE_BORN, s__SOMEONE_ELSE_BORN);
}

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


pub static GENERIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_GOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OCCASION_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_GENERIC() -> Py { ensure_init(); GENERIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GENERIC(v: Py) { ensure_init(); if let Ok(mut w) = GENERIC.write() { *w = v; } }
pub fn g_GIVEN() -> Py { ensure_init(); GIVEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVEN(v: Py) { ensure_init(); if let Ok(mut w) = GIVEN.write() { *w = v; } }
pub fn g_STOP() -> Py { ensure_init(); STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); if let Ok(mut w) = STOP.write() { *w = v; } }
pub fn g_WORN() -> Py { ensure_init(); WORN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORN(v: Py) { ensure_init(); if let Ok(mut w) = WORN.write() { *w = v; } }
pub fn g__NOT_GOT() -> Py { ensure_init(); _NOT_GOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_GOT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_GOT.write() { *w = v; } }
pub fn g__OCCASION() -> Py { ensure_init(); _OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = _OCCASION.write() { *w = v; } }
pub fn g__OCCASION_HEAD() -> Py { ensure_init(); _OCCASION_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OCCASION_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = _OCCASION_HEAD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A plan with something still missing is the question the person did not ask.\n\nPeople announce the occasion and the gap in one breath, and the store files both:\n\n    ive got a wedding in 3 weeks and i still havent got an outfit\n      -> \"Noted: you've got a wedding in 3 weeks and you still haven't got an outfit.\"\n\nFiling it is right - the follow-up turns need the wedding and the deadline - but it is not an\nanswer, and the ask they left out is one the knowledge base already has:\n\n    what should i wear to a wedding in 3 weeks\n      -> \"White and cream are the one thing to avoid at a wedding ...\"\n    what should i get for a wedding\n      -> \"About fifty pounds is the usual gift from a friend ...\"\n\nSo a line that names what it has not got, and names an occasion it is for, is read as that\nquestion and answered from the entry it reaches. Three rules keep it honest: the line has to say\nin its own words that the thing is not got yet (not that it is wanted, not that it is lost), the\nthing has to be one of the kinds an occasion actually raises - something worn, or something given\n- and the rewritten question has to reach an entry, because a plan the knowledge base has nothing\nfor is left to the reply that files it, exactly as before.\n");
    s__NOT_GOT(py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:still\\s+)?(?:havent|haven't|hasnt|hasn't|aint|ain't)\\s+(?:got|gotten|had|bought|sorted|ordered|found|booked|chosen|picked)\\s+(?P<thing>[a-z][a-z' -]*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:dont|don't|do\\s+not|doesnt|doesn't)\\s+have\\s+(?P<thing>[a-z][a-z' -]*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:not|n't)\\s+(?:got|gotten)\\s+(?P<thing>[a-z][a-z' -]*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:no|without)\\s+(?P<thing>[a-z][a-z' -]*?)\\s+yet\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|needed)\\s+(?P<thing>[a-z][a-z' -]*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s__OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:got|have|having|going\\s+to|off\\s+to|invited\\s+to|for|at)\\s+(?P<occ>(?:a|an|the|my|our|his|her|their)\\s+[a-z0-9][a-z0-9' -]{2,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OCCASION_HEAD(stdlib_call("re", "compile", &[py_str("^(?P<occ>(?:a|an|the|my|our|his|her|their)\\s+[a-z][a-z0-9' -]{1,30}?)(?:\\s+(?:is|are|was|will\\s+be|'s)\\b.*)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORN(call_builtin("frozenset", &[mcall(&py_str("\noutfit outfits dress dresses suit suits shirt shirts blouse blouses top tops skirt skirts\ntrousers pants shoes heels hat hats coat coats jacket jackets tie ties frock frocks jumpsuit\nclothes clothing gown gowns boots sandals something\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVEN(call_builtin("frozenset", &[mcall(&py_str("\npresent presents gift gifts pressie card cards flowers bouquet wrapping paper\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GENERIC(call_builtin("frozenset", &[mcall(&py_str("\nanything everything much sorted sorted organised organized booked arranged planned done\nstarted chosen picked\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP(call_builtin("frozenset", &[mcall(&py_str("a an the my our your his her their for to of in on at with and yet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _clauses(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s*(?:,|;|\\band\\b|\\bbut\\b|\\bso\\b|\\bbecause\\b)\\s*"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[py_str(" .!?,")])); } } py_list(__out) })() };
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STOP(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _what_is_missing(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing not got yet and the phrase it was said in, or None.");
    let __iter1 = py_iter_or_raise(&_clauses(&[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    let __iter2 = py_iter_or_raise(&g__NOT_GOT());
    for __item in __iter2 {
    frame = __item.clone();
    found = mcall(&frame.clone(), "search", &[clause.clone()]);
    if py_truthy(&found.clone()) {
    phrase = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[]);
    thing = stdlib_call("re", "sub", &[py_str("^(?:a|an|the|my|our|any|some|another)\\s+"), py_str(""), phrase.clone()]).unwrap_or_else(|e| exc::raise(e));
    thing = mcall(&geti(&stdlib_call("re", "split", &[py_str("\\s+(?:for|to|in|at|on|from)\\s+|\\s+yet\\b"), thing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[]);
    if py_truthy(&thing.clone()) {
    return py_tuple(vec![thing.clone(), phrase.clone()]);
    }
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_occasion(args: &[Py]) -> Py {
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
    let mut missing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "missing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: missing"))) };
    let mut after: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut found_all: Py = Py::None;
    let mut head: Py = Py::None;
    let mut occ: Py = Py::None;
    let _ = py_str("The occasion the missing thing is for, as the person said it, or None.");
    found_all = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_clauses(&[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    let _ = mcall(&found_all.clone(), "extend", &[mcall(&g__OCCASION(), "finditer", &[clause.clone()])]);
    head = mcall(&g__OCCASION_HEAD(), "match", &[clause.clone()]);
    if py_truthy(&head.clone()) {
    let _ = mcall(&found_all.clone(), "append", &[head.clone()]);
    }
    }
    let __iter2 = py_iter_or_raise(&found_all.clone());
    for __item in __iter2 {
    found = __item.clone();
    occ = mcall(&mcall(&found.clone(), "group", &[py_str("occ")]), "strip", &[py_str(" -")]);
    after = stdlib_call("re", "split", &[py_str("\\s+(?:for|to)\\s+"), occ.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&gt(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    occ = geti(&after.clone(), &neg(&Py::Int(1)));
    }
    occ = mcall(&stdlib_call("re", "sub", &[py_str("\\s+(?:in|on|at|for|to|next|this|the|with|of|yet)$"), py_str(""), occ.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" -")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&occ.clone())); if py_truthy(&__b) { __b } else { bitand(&_content(&[occ.clone()]), &_content(&[missing.clone()])) } }) {
    continue;
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&occ.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7))) {
    continue;
    }
    return if py_truthy(&mcall(&mcall(&occ.clone(), "lower", &[]), "startswith", &[py_str("a ")])) { slice_or_raise(&occ.clone(), &Py::Int(2), &Py::None, &Py::None) } else { occ.clone() };
    }
    return Py::None;
    Py::None
}
pub fn as_question(args: &[Py]) -> Py {
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
    let mut missing: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The question the line left out, or None when it left out nothing the base can answer.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("?")))) {
    return Py::None;
    }
    missing = _what_is_missing(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&missing.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&missing.clone(), 2, None);
    thing = __unpk1[0].clone();
    phrase = __unpk1[1].clone();
    words = bitor(&_content(&[phrase.clone()]), &_content(&[thing.clone()]));
    occasion = _the_occasion(&[said.clone(), thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&occasion.clone()))) {
    return Py::None;
    }
    if py_truthy(&bitand(&words.clone(), &g_GIVEN())) {
    return add(&py_str("what should i get for "), &occasion.clone());
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_WORN()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("wear"))) } }) {
    return add(&py_str("what should i wear to "), &occasion.clone());
    }
    if py_truthy(&bitand(&words.clone(), &g_GENERIC())) {
    return add(&py_str("how do i plan "), &occasion.clone());
    }
    return Py::None;
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut entry: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut store: Py = Py::None;
    let _ = py_str("The entry's answer to the question the line left out, or None to leave the line as it is.");
    question = as_question(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&question.clone()))) {
    return Py::None;
    }
    store = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    let __try1 = exc::catch(|| {
    entry = crate::gen::knowledge::lookup::find(&[crate::gen::understanding::understand(&[question.clone(), store.clone()])]);
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
    if py_truthy(&Py::Bool(py_is(&entry.clone(), &Py::None))) {
    return Py::None;
    }
    reply = crate::gen::memory::plans::_the_reply(&[entry.clone(), question.clone(), store.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[]) } } } }) {
    return Py::None;
    }
    return mcall(&reply.clone(), "strip", &[]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.pending".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "GENERIC" => Some(g_GENERIC()),
            "GIVEN" => Some(g_GIVEN()),
            "STOP" => Some(g_STOP()),
            "WORN" => Some(g_WORN()),
            "_NOT_GOT" => Some(g__NOT_GOT()),
            "_OCCASION" => Some(g__OCCASION()),
            "_OCCASION_HEAD" => Some(g__OCCASION_HEAD()),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_the_occasion" => Some(py_func("_the_occasion", |args: &[Py]| _the_occasion(args))),
            "_what_is_missing" => Some(py_func("_what_is_missing", |args: &[Py]| _what_is_missing(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "as_question" => Some(py_func("as_question", |args: &[Py]| as_question(args))),
            _ => None,
        }),
        set: None,
    }))
}

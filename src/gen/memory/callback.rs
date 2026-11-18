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


pub static AN_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EPISODE_TURNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_A_MEDICAL_EPISODE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOST_WORDS_OF_NEWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_NAMED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_RELATIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_AGE() -> Py { ensure_init(); AN_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AGE(v: Py) { ensure_init(); if let Ok(mut w) = AN_AGE.write() { *w = v; } }
pub fn g_EPISODE_TURNS() -> Py { ensure_init(); EPISODE_TURNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EPISODE_TURNS(v: Py) { ensure_init(); if let Ok(mut w) = EPISODE_TURNS.write() { *w = v; } }
pub fn g_IN_A_MEDICAL_EPISODE() -> Py { ensure_init(); IN_A_MEDICAL_EPISODE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_A_MEDICAL_EPISODE(v: Py) { ensure_init(); if let Ok(mut w) = IN_A_MEDICAL_EPISODE.write() { *w = v; } }
pub fn g_MOST_WORDS_OF_NEWS() -> Py { ensure_init(); MOST_WORDS_OF_NEWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS_OF_NEWS(v: Py) { ensure_init(); if let Ok(mut w) = MOST_WORDS_OF_NEWS.write() { *w = v; } }
pub fn g_THE_ASK() -> Py { ensure_init(); THE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = THE_ASK.write() { *w = v; } }
pub fn g_THE_NAMED_ASK() -> Py { ensure_init(); THE_NAMED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_NAMED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = THE_NAMED_ASK.write() { *w = v; } }
pub fn g_THE_RELATIONS() -> Py { ensure_init(); THE_RELATIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_RELATIONS(v: Py) { ensure_init(); if let Ok(mut w) = THE_RELATIONS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A question about a person the chat already holds: answer it with what the chat holds.\n\n    my mum is in hospital / she had a fall / she is 82 / how is she doing\n        -> \"Same as before: Falls are usually several small things adding up ...\"\n\nA friend asked how their mum was doing, and the bot read the word \"doing\" as the fall it had\nalready answered and said the same paragraph again, headed \"Same as before\". The person was not\nasking for the advice twice; they were asking after their mum, and the only thing the bot knows\nabout her is what they told it.\n\nSo the reply is what the chat holds, said in one line: the person (\"your mum\"), what they said\nabout her (\"had a fall\"), her age if it was given, and the one thing a friend asks - whether\nthere is any word since. When the chat holds nobody, there is nothing to answer with and the\nmodule stands down.\n");
    s_THE_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|but\\s+)?(?:how(?:'s|\\s+is|\\s+are|\\s+was|\\s+were)\\s+(?P<who>he|she|they|him|her|them)\\s*(?:doing|getting\\s+on|going|coping|holding\\s+up|been)?|how\\s+(?:has|have)\\s+(?P<been>he|she|they)\\s+been|is\\s+(?P<better>he|she|they)\\s+(?:any\\s+)?better)\\s*(?:now|today|then|since|at\\s+the\\s+moment)?\\s*\\?*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_NAMED_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+)?(?:how(?:'s|\\s+is|\\s+was)\\s+(?:your\\s+|my\\s+)?(?P<who>[a-z]{2,})|any\\s+news\\s+(?:about|on|of)\\s+(?:your\\s+|my\\s+)?(?P<news>[a-z]{2,})|how\\s+did\\s+it\\s+go\\s+with\\s+(?:your\\s+|my\\s+)?(?P<went>[a-z]{2,}))\\s*(?:doing|getting\\s+on|now|then|today)?\\s*\\?*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_RELATIONS(bitor(&call_builtin("frozenset", &[crate::gen::memory::facts::g_RELATIONS()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&py_str("mum mom mam mother dad father gran granny grandma grandad grandpa nan nana pops"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_AN_AGE(stdlib_call("re", "compile", &[py_str("^(?:is|was|turned|is\\s+now|just\\s+turned)\\s+\\d{1,3}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS_OF_NEWS(Py::Int(9));
    s_IN_A_MEDICAL_EPISODE(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|hospitalis\\w*|hospitaliz\\w*|surgery|surgical|operation|a&?e|casualty|intensive care|icu|coronary care|hospice|care home|nursing home|the ward|on a ward|in a ward)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EPISODE_TURNS(Py::Int(6));
}
pub fn _the_person_held(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(relation, name, age) for the person the chat holds last, or None.");
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::gen::memory::others(args));
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = py_call(&the_others, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    one = __item.clone();
    relation = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&relation.clone()))) {
    continue;
    }
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    age = mcall(&one.clone(), "get", &[py_str("age")]);
    return py_tuple(vec![relation.clone(), name.clone(), age.clone()]);
    }
    named = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&crate::gen::memory::holds::lines(&[memory.clone()]));
    for __item in __iter2 {
    line = __item.clone();
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("\\b(?:my|our)\\s+([a-z]{3,})\\b"), mcall(&call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    found = __item.clone();
    word = mcall(&found.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THE_RELATIONS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&named.clone(), &word.clone())) } }) {
    let _ = mcall(&named.clone(), "append", &[word.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return py_tuple(vec![geti(&named.clone(), &Py::Int(0)), py_str(""), Py::None]);
    }
    if py_truthy(&named.clone()) {
    return py_tuple(vec![geti(&named.clone(), &neg(&Py::Int(1))), py_str(""), Py::None]);
    }
    return Py::None;
    Py::None
}
pub fn _the_news_about(args: &[Py]) -> Py {
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
    let mut relation: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "relation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: relation"))) };
    let mut name: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What an earlier line said about them, with the person taken off the front.\n\n    \"she had a fall\" becomes \"had a fall\", and \"my mum is in hospital\" becomes \"is in hospital\":\n    both read on from \"your mum\", so the reply can say the one thing the chat holds without\n    quoting the line back at them.\n    ");
    words = add(&add(&py_list(vec![relation.clone()]), &if py_truthy(&name.clone()) { py_list(vec![name.clone()]) } else { py_list(vec![]) }), &call_builtin("list", &[_the_pronouns(&[relation.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[crate::gen::memory::holds::lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&low.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    found = stdlib_call("re", "search", &[add(&add(&py_str("\\b(?:my|our|the)?\\s*"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    rest = mcall(&slice_or_raise(&low.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.;:!-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS_OF_NEWS()) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_AN_AGE(), "search", &[rest.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("is"), py_str("was"), py_str("had"), py_str("has"), py_str("have"), py_str("went"), py_str("got"), py_str("kept"), py_str("said"), py_str("wants"), py_str("wanted"), py_str("needs"), py_str("keeps"), py_str("lives"), py_str("does"), py_str("did"), py_str("came"), py_str("started"), py_str("turned"), py_str("seems")]), &geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0))))) {
    return mcall(&rest.clone(), "rstrip", &[py_str(".")]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_pronouns(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut relation: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "relation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: relation"))) };
    if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::facts::g_HE(), &relation.clone()))) {
    return py_tuple(vec![py_str("he")]);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::facts::g_SHE(), &relation.clone()))) {
    return py_tuple(vec![py_str("she")]);
    }
    return py_tuple(vec![py_str("they")]);
    Py::None
}
pub fn _the_ask(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut said: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(relation, name, age) for the person the line asks about, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_THE_ASK(), "match", &[said.clone()]);
    if py_truthy(&match_.clone()) {
    held = _the_person_held(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    pointed = { let __b = mcall(&match_.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("been")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("better")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she")]), &pointed.clone()))) {
    wanted = if py_truthy(&Py::Bool(py_eq(&pointed.clone(), &py_str("he")))) { py_str("he") } else { py_str("she") };
    if py_truthy(&Py::Bool(!py_eq(&geti(&_the_pronouns(&[geti(&held.clone(), &Py::Int(0))]), &Py::Int(0)), &wanted.clone()))) {
    return py_str("");
    }
    }
    return held.clone();
    }
    match_ = mcall(&g_THE_NAMED_ASK(), "match", &[said.clone()]);
    if py_truthy(&match_.clone()) {
    word = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("news")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("went")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_THE_RELATIONS(), &word.clone()))) {
    return py_str("");
    }
    held = _the_person_held(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![word.clone(), mcall(&word.clone(), "rstrip", &[py_str("s")])]), &geti(&held.clone(), &Py::Int(0))))) {
    return py_str("");
    }
    return held.clone();
    }
    return py_str("");
    Py::None
}
pub fn _a_medical_episode(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("Whether a turn the chat holds is a stay, an operation or a move into care.");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&crate::gen::memory::holds::lines(&[memory.clone()]), &neg(&g_EPISODE_TURNS()), &Py::None, &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&mcall(&g_IN_A_MEDICAL_EPISODE(), "search", &[call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut news: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut since: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("What the chat holds about the person the line asks after, or \"\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::holds::there_is_one(&[memory.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _a_medical_episode(&[memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_IN_A_MEDICAL_EPISODE(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return py_str("");
    }
    held = _the_ask(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&held.clone(), 3, None);
    relation = __unpk1[0].clone();
    name = __unpk1[1].clone();
    age = __unpk1[2].clone();
    who = if py_truthy(&name.clone()) { mcall(&name.clone(), "capitalize", &[]) } else { add(&py_str("your "), &relation.clone()) };
    news = _the_news_about(&[memory.clone(), relation.clone(), name.clone()]);
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { { let __b = news.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "fullmatch", &[add(&add(&py_str("(?:is|was)\\s+"), &stdlib_call("re", "escape", &[call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:\\s+years?\\s+old)?")), news.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    news = py_str("");
    }
    since = py_tuple(vec![py_str("Any word since then?"), py_str("Any news since?"), py_str("Has there been any change?"), py_str("Anything new since?")]);
    if py_truthy(&news.clone()) {
    pool = py_tuple(vec![add(&add(&py_str("You said {who} "), &news.clone()), &py_str(". {since}")), add(&add(&py_str("Last you said, {who} "), &news.clone()), &py_str(". {since}")), add(&add(&py_str("{who_cap} "), &news.clone()), &py_str(", you said. {since}"))]);
    return _choose(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&since.clone()) { let mut ask: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone()), ("who_cap".to_string(), mcall(&who.clone(), "capitalize", &[])), ("since".to_string(), ask.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } } py_list(__out) })() }, memory.clone()]);
    }
    if py_truthy(&age.clone()) {
    pool = py_tuple(vec![add(&add(&py_str("{who_cap} is "), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" - what has {pronoun} been saying?")), add(&add(&py_str("{who_cap} is "), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(", and that is all I have. What is {pronoun} up to at the moment?")), add(&add(&py_str("{who_cap} is "), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" - has {pronoun} been in touch?"))]);
    return _choose(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who_cap".to_string(), mcall(&who.clone(), "capitalize", &[])), ("pronoun".to_string(), geti(&_the_pronouns(&[relation.clone()]), &Py::Int(0)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn claims(args: &[Py]) -> Py {
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
    let _ = py_str("The lines this reading answers even when another module claims the turn as it comes.\n\n    \"how is she doing\" is a claim of `answers` - it takes the follow-up as it comes and reads\n    \"doing\" as the thing it last answered - so the module's own PRIORITY cannot take the turn.\n    The claim is given up only to a module that says so itself (README.md, \"Interface\"), and this\n    is the line where the chat holds the person being asked after.\n    ");
    return call_builtin("bool", &[reply_for(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fresh: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("One line from the pool, not one this chat has already had.\n\n    The lines are compared *as they would read*, not as they were built: the pool holds the\n    skeleton with `{who}` and `{since}` in it, so a skeleton never matched a sentence the bot had\n    already said and the whole pool counted as fresh. Two asks in a row about the same news then\n    got the same sentence word for word - \"Last you said, your mum had a fall on sunday. Any news\n    since?\" twice, which is the one thing a reply must never do.\n    ");
    let mut random: Py = stdlib_module("random");
    said = crate::gen::memory::holds::the_bot_lines(&[memory.clone()]);
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said.clone(), &line.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    return mcall(&random.clone(), "choice", &[{ let __b = fresh.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)) } }]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.callback".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_AGE" => Some(g_AN_AGE()),
            "EPISODE_TURNS" => Some(g_EPISODE_TURNS()),
            "IN_A_MEDICAL_EPISODE" => Some(g_IN_A_MEDICAL_EPISODE()),
            "MOST_WORDS_OF_NEWS" => Some(g_MOST_WORDS_OF_NEWS()),
            "THE_ASK" => Some(g_THE_ASK()),
            "THE_NAMED_ASK" => Some(g_THE_NAMED_ASK()),
            "THE_RELATIONS" => Some(g_THE_RELATIONS()),
            "_a_medical_episode" => Some(py_func("_a_medical_episode", |args: &[Py]| _a_medical_episode(args))),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_the_ask" => Some(py_func("_the_ask", |args: &[Py]| _the_ask(args))),
            "_the_news_about" => Some(py_func("_the_news_about", |args: &[Py]| _the_news_about(args))),
            "_the_person_held" => Some(py_func("_the_person_held", |args: &[Py]| _the_person_held(args))),
            "_the_pronouns" => Some(py_func("_the_pronouns", |args: &[Py]| _the_pronouns(args))),
            "claims" => Some(py_func("claims", |args: &[Py]| claims(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


thread_local! { static G_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EPISODE_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_MEDICAL_EPISODE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS_OF_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NAMED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_AGE() -> Py { ensure_init(); G_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE(v: Py) { ensure_init(); G_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_EPISODE_TURNS() -> Py { ensure_init(); G_EPISODE_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EPISODE_TURNS(v: Py) { ensure_init(); G_EPISODE_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_MEDICAL_EPISODE() -> Py { ensure_init(); G_IN_A_MEDICAL_EPISODE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_MEDICAL_EPISODE(v: Py) { ensure_init(); G_IN_A_MEDICAL_EPISODE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS_OF_NEWS() -> Py { ensure_init(); G_MOST_WORDS_OF_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS_OF_NEWS(v: Py) { ensure_init(); G_MOST_WORDS_OF_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASK() -> Py { ensure_init(); G_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); G_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NAMED_ASK() -> Py { ensure_init(); G_THE_NAMED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NAMED_ASK(v: Py) { ensure_init(); G_THE_NAMED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_RELATIONS() -> Py { ensure_init(); G_THE_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_RELATIONS(v: Py) { ensure_init(); G_THE_RELATIONS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about a person the chat already holds: answer it with what the chat holds.\n\n    my mum is in hospital / she had a fall / she is 82 / how is she doing\n        -> \"Same as before: Falls are usually several small things adding up ...\"\n\nA friend asked how their mum was doing, and the bot read the word \"doing\" as the fall it had\nalready answered and said the same paragraph again, headed \"Same as before\". The person was not\nasking for the advice twice; they were asking after their mum, and the only thing the bot knows\nabout her is what they told it.\n\nSo the reply is what the chat holds, said in one line: the person (\"your mum\"), what they said\nabout her (\"had a fall\"), her age if it was given, and the one thing a friend asks - whether\nthere is any word since. When the chat holds nobody, there is nothing to answer with and the\nmodule stands down.\n");
    s_THE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+|but\\s+)?(?:how(?:'s|\\s+is|\\s+are|\\s+was|\\s+were)\\s+(?P<who>he|she|they|him|her|them)\\s*(?:doing|getting\\s+on|going|coping|holding\\s+up|been)?|how\\s+(?:has|have)\\s+(?P<been>he|she|they)\\s+been|is\\s+(?P<better>he|she|they)\\s+(?:any\\s+)?better)\\s*(?:now|today|then|since|at\\s+the\\s+moment)?\\s*\\?*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_NAMED_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+)?(?:how(?:'s|\\s+is|\\s+was)\\s+(?:your\\s+|my\\s+)?(?P<who>[a-z]{2,})|any\\s+news\\s+(?:about|on|of)\\s+(?:your\\s+|my\\s+)?(?P<news>[a-z]{2,})|how\\s+did\\s+it\\s+go\\s+with\\s+(?:your\\s+|my\\s+)?(?P<went>[a-z]{2,}))\\s*(?:doing|getting\\s+on|now|then|today)?\\s*\\?*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_RELATIONS(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::facts::g_RELATIONS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mum mom mam mother dad father gran granny grandma grandad grandpa nan nana pops"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_AN_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:is|was|turned|is\\s+now|just\\s+turned)\\s+\\d{1,3}\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOST_WORDS_OF_NEWS(Py::Int(9));
    s_IN_A_MEDICAL_EPISODE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:hospital|hospitalis\\w*|hospitaliz\\w*|surgery|surgical|operation|a&?e|casualty|intensive care|icu|coronary care|hospice|care home|nursing home|the ward|on a ward|in a ward)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    let mut the_others: Py = py_func("others", |args: &[Py]| crate::memory::others(args));
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&the_others, args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(one.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    one = __item.clone();
    relation = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&relation.clone()))) {
    continue;
    }
    name = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) };
    return py_tuple(vec![relation.clone(), name.clone(), age.clone()]);
    }
    named = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::holds::lines(args))(&__args) });
    for __item in __iter2 {
    line = __item.clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+([a-z]{3,})\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    found = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THE_RELATIONS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&named.clone(), &word.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return py_tuple(vec![geti(&named.clone(), &Py::Int(0)), py_str(""), Py::None]);
    }
    if py_truthy(&named.clone()) {
    return py_tuple(vec![geti(&named.clone(), &op_neg(&Py::Int(1))), py_str(""), Py::None]);
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
    words = op_add(&op_add(&py_list(vec![relation.clone()]), &if py_truthy(&name.clone()) { py_list(vec![name.clone()]) } else { py_list(vec![]) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(relation.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pronouns(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::holds::lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&low.clone())) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|our|the)?\\s*"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&low.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_MOST_WORDS_OF_NEWS()) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_AGE(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("is"), py_str("was"), py_str("had"), py_str("has"), py_str("have"), py_str("went"), py_str("got"), py_str("kept"), py_str("said"), py_str("wants"), py_str("wanted"), py_str("needs"), py_str("keeps"), py_str("lives"), py_str("does"), py_str("did"), py_str("came"), py_str("started"), py_str("turned"), py_str("seems")]), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }, &Py::Int(0))))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "rstrip", args))(&__args) };
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
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_HE(), &relation.clone()))) {
    return py_tuple(vec![py_str("he")]);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_SHE(), &relation.clone()))) {
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_ASK(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_person_held(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    pointed = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("been")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("better")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } } } };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she")]), &pointed.clone()))) {
    wanted = if py_truthy(&Py::Bool(py_eq(&pointed.clone(), &py_str("he")))) { py_str("he") } else { py_str("she") };
    if py_truthy(&Py::Bool(!py_eq(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&held.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pronouns(args))(&__args) }, &Py::Int(0)), &wanted.clone()))) {
    return py_str("");
    }
    }
    return held.clone();
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_NAMED_ASK(), "match", args))(&__args) };
    if py_truthy(&match_.clone()) {
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("news")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("went")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&g_THE_RELATIONS(), &word.clone()))) {
    return py_str("");
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_person_held(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![word.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) }]), &geti(&held.clone(), &Py::Int(0))))) {
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
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::holds::lines(args))(&__args) }, &op_neg(&g_EPISODE_TURNS()), &Py::None, &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IN_A_MEDICAL_EPISODE(), "search", args))(&__args) }) {
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::holds::there_is_one(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_medical_episode(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IN_A_MEDICAL_EPISODE(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ask(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&held.clone(), 3, None);
    relation = __unpk1[0].clone();
    name = __unpk1[1].clone();
    age = __unpk1[2].clone();
    who = if py_truthy(&name.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "capitalize", args))(&__args) } } else { op_add(&py_str("your "), &relation.clone()) };
    news = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(relation.clone()); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_news_about(args))(&__args) };
    if py_truthy(&{ let __b = age.clone(); if !py_truthy(&__b) { __b } else { { let __b = news.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?:is|was)\\s+"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?:\\s+years?\\s+old)?"))); __args.push(news.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    news = py_str("");
    }
    since = py_tuple(vec![py_str("Any word since then?"), py_str("Any news since?"), py_str("Has there been any change?"), py_str("Anything new since?")]);
    if py_truthy(&news.clone()) {
    pool = py_tuple(vec![op_add(&op_add(&py_str("You said {who} "), &news.clone()), &py_str(". {since}")), op_add(&op_add(&py_str("Last you said, {who} "), &news.clone()), &py_str(". {since}")), op_add(&op_add(&py_str("{who_cap} "), &news.clone()), &py_str(", you said. {since}"))]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&since.clone()) { let mut ask: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone()), ("who_cap".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "capitalize", args))(&__args) }), ("since".to_string(), ask.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } } py_list(__out) })() }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&age.clone()) {
    pool = py_tuple(vec![op_add(&op_add(&py_str("{who_cap} is "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" - what has {pronoun} been saying?")), op_add(&op_add(&py_str("{who_cap} is "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(", and that is all I have. What is {pronoun} up to at the moment?")), op_add(&op_add(&py_str("{who_cap} is "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" - has {pronoun} been in touch?"))]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who_cap".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "capitalize", args))(&__args) }), ("pronoun".to_string(), geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(relation.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pronouns(args))(&__args) }, &Py::Int(0)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reply_for(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::holds::the_bot_lines(args))(&__args) };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said.clone(), &line.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = fresh.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&random.clone(), "choice", args))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_AGE" => s_AN_AGE(v),
            "EPISODE_TURNS" => s_EPISODE_TURNS(v),
            "IN_A_MEDICAL_EPISODE" => s_IN_A_MEDICAL_EPISODE(v),
            "MOST_WORDS_OF_NEWS" => s_MOST_WORDS_OF_NEWS(v),
            "THE_ASK" => s_THE_ASK(v),
            "THE_NAMED_ASK" => s_THE_NAMED_ASK(v),
            "THE_RELATIONS" => s_THE_RELATIONS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.callback", module_obj);
    pyrt::xrefs::reg_fn("memory.callback", "_the_person_held", py_func("_the_person_held", |args: &[Py]| _the_person_held(args)));
    pyrt::xrefs::reg_fn("memory.callback", "_the_news_about", py_func("_the_news_about", |args: &[Py]| _the_news_about(args)));
    pyrt::xrefs::reg_fn("memory.callback", "_the_pronouns", py_func("_the_pronouns", |args: &[Py]| _the_pronouns(args)));
    pyrt::xrefs::reg_fn("memory.callback", "_the_ask", py_func("_the_ask", |args: &[Py]| _the_ask(args)));
    pyrt::xrefs::reg_fn("memory.callback", "_a_medical_episode", py_func("_a_medical_episode", |args: &[Py]| _a_medical_episode(args)));
    pyrt::xrefs::reg_fn("memory.callback", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.callback", "claims", py_func("claims", |args: &[Py]| claims(args)));
    pyrt::xrefs::reg_fn("memory.callback", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_access("memory.callback", "AN_AGE", g_AN_AGE, s_AN_AGE);
    pyrt::xrefs::reg_access("memory.callback", "EPISODE_TURNS", g_EPISODE_TURNS, s_EPISODE_TURNS);
    pyrt::xrefs::reg_access("memory.callback", "IN_A_MEDICAL_EPISODE", g_IN_A_MEDICAL_EPISODE, s_IN_A_MEDICAL_EPISODE);
    pyrt::xrefs::reg_access("memory.callback", "MOST_WORDS_OF_NEWS", g_MOST_WORDS_OF_NEWS, s_MOST_WORDS_OF_NEWS);
    pyrt::xrefs::reg_access("memory.callback", "THE_ASK", g_THE_ASK, s_THE_ASK);
    pyrt::xrefs::reg_access("memory.callback", "THE_NAMED_ASK", g_THE_NAMED_ASK, s_THE_NAMED_ASK);
    pyrt::xrefs::reg_access("memory.callback", "THE_RELATIONS", g_THE_RELATIONS, s_THE_RELATIONS);
}

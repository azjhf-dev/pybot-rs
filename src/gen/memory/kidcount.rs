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


pub static A_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_RELATIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_CHILD() -> Py { ensure_init(); A_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = A_CHILD.write() { *w = v; } }
pub fn g_CHILD_RELATIONS() -> Py { ensure_init(); CHILD_RELATIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_RELATIONS(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_RELATIONS.write() { *w = v; } }
pub fn g_NOT_THEIR_OWN() -> Py { ensure_init(); NOT_THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THEIR_OWN.write() { *w = v; } }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); SOMEONE_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE.write() { *w = v; } }
pub fn g_THEIR_OWN() -> Py { ensure_init(); THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN.write() { *w = v; } }
pub fn g_THE_ASK() -> Py { ensure_init(); THE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = THE_ASK.write() { *w = v; } }
pub fn g_WORDS() -> Py { ensure_init(); WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WORDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The number of children, counted from the children the chat already holds.\n\nNo single line has to state a number for the count to be known. `my son is 6` and `and my\ndaughter is 9` are two children with their ages, and the ask after them, `how many children do i\nhave`, was answered \"You've mentioned your son - how many have you got altogether?\": the store\nreads a count off one line, and a line about one child gives one child.\n\nSo the count is read off what the store holds: each child relation the person named is one child,\na child named on their own is one however they came in (`my son jack is 6` and `my son sam is 9`\nare two), and the same child said twice is still one. A child who belongs to somebody else is not\ncounted, and one child keeps the honest ask for a number.\n\nOnly the ask is answered: `reply_for` returns None on every other line, and on a chat that\nalready has a number, because the store answers that one.\n");
    s_CHILD_RELATIONS(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("baby"), py_str("twin"), py_str("boy"), py_str("girl"), py_str("stepson"), py_str("stepdaughter"), py_str("teenager"), py_str("lad"), py_str("lass"), py_str("sons"), py_str("daughters"), py_str("children"), py_str("kids"), py_str("boys"), py_str("girls"), py_str("twins"), py_str("lads"), py_str("lasses")]));
    s_THE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+many\\b(?P<rest>[^.?!]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|my|our|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|did|has|have|had)\\s+(?:my|her|his|their|your|she|he|they|the|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[g_CHILD_RELATIONS()])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:in|at|per|into)\\s+(?:a|an|the|my|our)\\s+(?:class|classroom|school|nursery|team|group|year|country|world)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDS(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
}
pub fn _asks(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("Whether the line asks the person's own number of children.");
    low = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    match_ = mcall(&g_THE_ASK(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&match_.clone(), "group", &[py_str("rest")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_CHILD(), "search", &[rest.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SOMEONE_ELSE(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_THEIR_OWN(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_THEIR_OWN(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } };
    Py::None
}
pub fn _named_children(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut name: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The children the store holds by name, each one once.");
    held = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&held.clone(), &name.clone())) } }) {
    continue;
    }
    if py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("of")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[py_str("whose")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_RELATIONS(), &who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&who.clone())) } }) {
    let _ = mcall(&held.clone(), "add", &[name.clone()]);
    }
    }
    return held.clone();
    Py::None
}
pub fn _relations(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The child relations the person named, each one once, in the order they said them.");
    held = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    who = __item.clone();
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_RELATIONS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &word.clone())) } }) {
    let _ = mcall(&held.clone(), "append", &[word.clone()]);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_RELATIONS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &word.clone())) } }) {
    let _ = mcall(&held.clone(), "append", &[word.clone()]);
    }
    }
    return held.clone();
    Py::None
}
pub fn count(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut relations: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("How many children the chat holds, or None when it holds no number worth saying.");
    user = { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&mcall(&user.clone(), "get", &[py_str("kids")])) {
    return Py::None;
    }
    named = _named_children(&[memory.clone()]);
    relations = _relations(&[memory.clone()]);
    held = call_builtin("max", &[call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("len", &[relations.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&held.clone(), &Py::Int(2))) {
    return Py::None;
    }
    return held.clone();
    Py::None
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
    let _ = py_str("A choice that does not repeat the last thing the bot said.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&t.clone(), "get", &[py_str("bot")])); } } py_set(__out) })() };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&recent.clone(), &o.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    return mcall(&stdlib_call("random", "Random", &[call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
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
    let mut held: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The count said back, or None on every line that is not the ask for it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks(&[text.clone()]))) } }) {
    return Py::None;
    }
    held = count(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    word = mcall(&g_WORDS(), "get", &[held.clone(), call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    noun = py_str("children");
    return _vary(&[py_list(vec![add(&add(&add(&add(&py_str("You've got "), &word.clone()), &py_str(" ")), &noun.clone()), &py_str(".")), add(&add(&add(&mcall(&word.clone(), "capitalize", &[]), &py_str(" ")), &noun.clone()), &py_str(" — that's what you've told me.")), add(&add(&add(&add(&py_str("That makes "), &word.clone()), &py_str(" ")), &noun.clone()), &py_str(" you've mentioned."))]), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.kidcount".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHILD" => Some(g_A_CHILD()),
            "CHILD_RELATIONS" => Some(g_CHILD_RELATIONS()),
            "NOT_THEIR_OWN" => Some(g_NOT_THEIR_OWN()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "THEIR_OWN" => Some(g_THEIR_OWN()),
            "THE_ASK" => Some(g_THE_ASK()),
            "WORDS" => Some(g_WORDS()),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_named_children" => Some(py_func("_named_children", |args: &[Py]| _named_children(args))),
            "_relations" => Some(py_func("_relations", |args: &[Py]| _relations(args))),
            "_vary" => Some(py_func("_vary", |args: &[Py]| _vary(args))),
            "count" => Some(py_func("count", |args: &[Py]| count(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static ASKS_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_FIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_FULL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_SURNAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SURNAME_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DESCRIBES_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOWER_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEVER_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_A_SURNAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_NAME_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKS_AGAIN() -> Py { ensure_init(); ASKS_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_AGAIN.write() { *w = v; } }
pub fn g_ASKS_THE_FIRST() -> Py { ensure_init(); ASKS_THE_FIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_FIRST(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_FIRST.write() { *w = v; } }
pub fn g_ASKS_THE_FULL() -> Py { ensure_init(); ASKS_THE_FULL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_FULL(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_FULL.write() { *w = v; } }
pub fn g_ASKS_THE_SURNAME() -> Py { ensure_init(); ASKS_THE_SURNAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_SURNAME(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_SURNAME.write() { *w = v; } }
pub fn g_A_SURNAME_WORD() -> Py { ensure_init(); A_SURNAME_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SURNAME_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_SURNAME_WORD.write() { *w = v; } }
pub fn g_DESCRIBES_A_NAME() -> Py { ensure_init(); DESCRIBES_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DESCRIBES_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = DESCRIBES_A_NAME.write() { *w = v; } }
pub fn g_LOWER_PARTS() -> Py { ensure_init(); LOWER_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOWER_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = LOWER_PARTS.write() { *w = v; } }
pub fn g_NAME_PARTS() -> Py { ensure_init(); NAME_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = NAME_PARTS.write() { *w = v; } }
pub fn g_NEVER_A_NAME() -> Py { ensure_init(); NEVER_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEVER_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = NEVER_A_NAME.write() { *w = v; } }
pub fn g_NOT_THE_ASK() -> Py { ensure_init(); NOT_THE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_ASK.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_SAYS_A_SURNAME() -> Py { ensure_init(); SAYS_A_SURNAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_A_SURNAME(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_A_SURNAME.write() { *w = v; } }
pub fn g_THEIR_OWN() -> Py { ensure_init(); THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN.write() { *w = v; } }
pub fn g_THE_NAME_WORD() -> Py { ensure_init(); THE_NAME_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_NAME_WORD(v: Py) { ensure_init(); if let Ok(mut w) = THE_NAME_WORD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The family name a person gives, and the ask that wants it back.\n\nThe store keeps one name for a person, and it is the name they want to be called: \"my name is\nsam\" and \"call me sam\" both file `Sam`. A family name is a second name and not that one, so it is\nkept on its own. Without it, \"my last name is okafor\" was filed as nothing at all and the ask a\nturn later, \"whats my surname\", came back \"What's your surname? You've not said, so only you know\"\n\n- a fact the chat had just been given, said back as a fact it never had.\n\nTwo rules hold the reading together:\n\n* a surname is filed only when the line says the word is one - \"my last name is okafor\",\n  \"surname: okafor\", \"our family name is nguyen\". A bare word after \"i am\" is a first name, and\n  the store already reads those; guessing here would move a first name into the wrong slot.\n* the ask is answered only when the line asks for their own family name and is not a how-to.\n  \"how do i change my surname\" is a question about the world (a deed poll, a marriage, a form)\n  and belongs to whoever answers those, so a verb of that kind takes the line away from here.\n\nEverything else returns None and leaves the line exactly as it was.\n");
    s_A_SURNAME_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:surname|sur-name|last\\s*-?\\s*name|family\\s*-?\\s*name|second\\s*-?\\s*name)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_A_SURNAME(stdlib_call("re", "compile", &[py_str("\\b(?:(?:my|our|the)\\s+)?(?:surname|sur-name|last\\s*-?\\s*name|family\\s*-?\\s*name|second\\s*-?\\s*name)\\s*(?:is|was|'s|s|:|=)\\s*(?P<name>[A-Za-z][A-Za-z'\\u2019\\-]{1,30}(?:\\s+[A-Za-z][A-Za-z'\\u2019\\-]{1,30}){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_PARTS(call_builtin("frozenset", &[mcall(&py_str("van von der den de da di du dos das del della bin ibn al el la le mc mac st saint o o' ap"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DESCRIBES_A_NAME(call_builtin("frozenset", &[mcall(&py_str("\nhard difficult easy long short weird strange common rare unusual foreign hyphenated double\nspelled spelt misspelled misspelt mispelled changed change changing wrong right same different\npronounced unpronounceable tricky silly complicated confusing correct incorrect spelled okay ok\nalso really actually just still not now too boring ugly nice lovely beautiful awful terrible funny\nembarrassing stupid odd plain old big small\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEVER_A_NAME(call_builtin("frozenset", &[mcall(&py_str("\non in at from of for to with about under over after before by into onto near above below\nand or but if so that this it there when how why the a an\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOWER_PARTS(call_builtin("frozenset", &[mcall(&py_str("van von der den de da di du dos das del della bin ibn al el la le st saint"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_NAME_WORD(py_str("(?:surname|sur-name|last\\s*-?\\s*name|family\\s*-?\\s*name|second\\s*-?\\s*name)"));
    s_ASKS_THE_SURNAME(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![add(&add(&py_str("\\b(?:what|whats|what's|which)\\s+(?:is\\s+|was\\s+|were\\s+)?(?:my|our|the)?\\s*"), &g_THE_NAME_WORD()), &py_str("\\b")), add(&add(&py_str("\\b(?:which|what)\\s+"), &g_THE_NAME_WORD()), &py_str("\\s+(?:is|was)\\s+(?:mine|ours)\\b")), add(&add(&py_str("\\b(?:do|did|can|could)\\s+(?:you|u|ya)\\s+(?:still\\s+)?(?:know|remember)\\s+(?:what\\s+)?(?:my|our)\\s+"), &g_THE_NAME_WORD()), &py_str("\\b")), add(&add(&py_str("\\b(?:tell|remind)\\s+(?:me|us)\\s+(?:what\\s+)?(?:my|our)\\s+"), &g_THE_NAME_WORD()), &py_str("\\b")), add(&add(&py_str("\\b(?:my|our)\\s+"), &g_THE_NAME_WORD()), &py_str("\\s+again\\b")), add(&add(&py_str("\\bwhat\\s+(?:did|do)\\s+i\\s+(?:say|tell\\s+you)\\s+(?:my|our)\\s+"), &g_THE_NAME_WORD()), &py_str("\\b")), add(&add(&py_str("^(?:and\\s+|so\\s+)?(?:my|our)?\\s*"), &g_THE_NAME_WORD()), &py_str("\\s*\\??$"))])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how|change|changing|changed|deed\\s+poll|register|registering|certificate|passport|marri(?:ed|age|ying)|divorce|spell|spelt|spelled|pronounce|website|domain|form|online)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|mine|ours|i|we|me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_THE_FIRST(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|which|do|does|did|know|remember|again|tell)\\b[^?.!]*\\b(?:first\\s*-?\\s*name|given\\s*-?\\s*name)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_THE_FULL(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|which|do|does|did|know|remember|again|tell)\\b[^?.!]*\\bfull\\s*-?\\s*name\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|which|tell|remind|again|said)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|them|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut option: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A choice that does not repeat the last thing the bot said.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&turn.clone(), "get", &[py_str("bot")])); } } py_set(__out) })() };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&recent.clone(), &option.clone()))) {  __out.push(option.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    return mcall(&stdlib_call("random", "Random", &[call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    Py::None
}
pub fn _the_name_they_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut index: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The name the way it is written: \"okafor\" -> \"Okafor\", \"van der berg\" -> \"van der Berg\",\n    \"o'brien\" -> \"O'Brien\". A particle stays in lower case unless it is the first word, and the\n    letter after an apostrophe is a name's own capital.");
    parts = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    written = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    part = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LOWER_PARTS(), &mcall(&part.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { lt(&index.clone(), &sub(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    let _ = mcall(&written.clone(), "append", &[mcall(&part.clone(), "lower", &[])]);
    } else {
    let _ = mcall(&written.clone(), "append", &[add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    }
    said = mcall(&py_str(" "), "join", &[written.clone()]);
    return stdlib_call("re", "sub", &[py_str("(['\\u2019])([a-z])"), py_func("lambda", move |args: &[Py]| { let index = index.clone(); let part = part.clone(); let parts = parts.clone(); let said = said.clone(); let word = word.clone(); let written = written.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); add(&mcall(&m.clone(), "group", &[Py::Int(1)]), &mcall(&mcall(&m.clone(), "group", &[Py::Int(2)]), "upper", &[])) }), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_surname(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The family name this chat was told, or \"\".");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = mcall(&held.clone(), "get", &[py_str("surname")]);
    return if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&said.clone(), "strip", &[]) } else { py_str("") };
    Py::None
}
pub fn _given_one(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The family name the line gives, or \"\" when the line gives none.");
    let __iter1 = py_iter_or_raise(&mcall(&g_SAYS_A_SURNAME(), "finditer", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    match_ = __item.clone();
    name = mcall(&match_.clone(), "group", &[py_str("name")]);
    first = geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0));
    low = mcall(&mcall(&first.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DESCRIBES_A_NAME(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NEVER_A_NAME(), &low.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } } } }) {
    continue;
    }
    parts = py_list(vec![first.clone()]);
    let __iter2 = py_iter_or_raise(&slice_or_raise(&mcall(&name.clone(), "split", &[]), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NAME_PARTS(), &mcall(&word.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NAME_PARTS(), &mcall(&geti(&parts.clone(), &neg(&Py::Int(1))), "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "isalpha", &[]) } } } } } }) {
    let _ = mcall(&parts.clone(), "append", &[word.clone()]);
    continue;
    }
    break;
    }
    return _the_name_they_said(&[mcall(&py_str(" "), "join", &[parts.clone()])]);
    }
    return py_str("");
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
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("File a family name the line gives. True when one was kept.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    said = _given_one(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    held = mcall(&memory.clone(), "setdefault", &[py_str("user"), py_dict(vec![])]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let _ = py_setitem(&held.clone(), &py_str("surname"), said.clone());
    return Py::Bool(true);
    Py::None
}
pub fn _asks_for_it(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is the person asking for their own family name.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_SURNAME_WORD(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_THE_ASK(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASKS_THE_SURNAME()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_first_name(args: &[Py]) -> Py {
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
    let mut their_name: Py = py_func("name", |args: &[Py]| crate::gen::memory::name(args));
    return { let __b = py_call(&their_name, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn a_note(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("A short word back when the whole line is the family name and nothing else.\n\n    A line that only gives it was left to the greeting (\"Got it. Is there anything you'd like a\n    hand with?\"), which says nothing about what was heard. Only a whole line: a surname inside a\n    longer sentence (\"my last name is okafor and im moving to spain\") is filed and the rest of the\n    line is still the thing to answer.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    match_ = mcall(&g_SAYS_A_SURNAME(), "match", &[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .!?,;-")]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_given_one(&[said.clone()])))) {
    return Py::None;
    }
    held = the_surname(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    return _vary(&[py_list(vec![add(&held.clone(), &py_str(" — noted.")), add(&add(&py_str("Got it — "), &held.clone()), &py_str(".")), add(&held.clone(), &py_str(", I'll remember that."))]), memory.clone()]);
    Py::None
}
pub fn _the_lines_before(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Int(3) };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the last few turns said, to see whether the chat is still on the surname.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&back.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(add(&add(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str(" ")), &call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))); } py_list(__out) })() }]), "lower", &[]);
    Py::None
}
pub fn a_follow_up(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The family name for a follow-up that points back at it, or None.");
    said = mcall(&_plain(&[text.clone()]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&the_surname(&[memory.clone()]))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_THE_ASK(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_SURNAME_WORD(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_ASKS_AGAIN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_POINTS_BACK(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_SURNAME_WORD(), "search", &[_the_lines_before(&[memory.clone()])])))) {
    return Py::None;
    }
    held = the_surname(&[memory.clone()]);
    return _vary(&[py_list(vec![add(&add(&py_str("It's "), &held.clone()), &py_str(".")), add(&held.clone(), &py_str(" — as you told me.")), add(&add(&py_str("Still "), &held.clone()), &py_str("."))]), memory.clone()]);
    Py::None
}
pub fn there_is_a_chat(args: &[Py]) -> Py {
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
    let _ = py_str("True once a turn has been had, which is when the store has anything to read back.");
    let mut any_chat: Py = py_func("there_is_a_chat", |args: &[Py]| crate::gen::memory::there_is_a_chat(args));
    return py_call(&any_chat, &[memory.clone()]);
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
    let mut note: Py = Py::None;
    let _ = py_str("The family name said back, or an honest line when the chat never gave one.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    note = a_note(&[text.clone(), memory.clone()]);
    if py_truthy(&note.clone()) {
    return note.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&there_is_a_chat(&[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS_THE_FULL(), "search", &[_plain(&[text.clone()])])) {
    return _the_full_name(&[memory.clone()]);
    }
    if py_truthy(&mcall(&g_ASKS_THE_FIRST(), "search", &[_plain(&[text.clone()])])) {
    return _the_first_name_reply(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_asks_for_it(&[text.clone()])))) {
    return a_follow_up(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&_given_one(&[text.clone()])) {
    return Py::None;
    }
    held = the_surname(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    let _ = py_setitem(&memory.clone(), &py_str("asked_for"), py_str("surname"));
    return _vary(&[py_list(vec![py_str("You haven't told me your surname yet."), py_str("I don't have it — you've not said your surname."), py_str("You haven't given me a surname, so I'd only be guessing.")]), memory.clone()]);
    }
    return _vary(&[py_list(vec![add(&add(&py_str("It's "), &held.clone()), &py_str(".")), add(&held.clone(), &py_str(" — that's the one you gave me.")), add(&add(&py_str("Your surname's "), &held.clone()), &py_str("."))]), memory.clone()]);
    Py::None
}
pub fn _plain(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _the_full_name(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut first: Py = Py::None;
    let _ = py_str("Both names, when the chat holds them: \"Sam Okafor\".");
    let __unpk1 = py_unpack(&py_tuple(vec![_the_first_name(&[memory.clone()]), the_surname(&[memory.clone()])]), 2, None);
    first = __unpk1[0].clone();
    family = __unpk1[1].clone();
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { family.clone() } }) {
    return _vary(&[py_list(vec![add(&add(&add(&first.clone(), &py_str(" ")), &family.clone()), &py_str(".")), add(&add(&add(&add(&py_str("You're "), &first.clone()), &py_str(" ")), &family.clone()), &py_str(".")), add(&add(&add(&first.clone(), &py_str(" ")), &family.clone()), &py_str(" — that's both of the names you gave."))]), memory.clone()]);
    }
    if py_truthy(&first.clone()) {
    return add(&add(&py_str("I have your first name, "), &first.clone()), &py_str(", but you haven't given me a surname."));
    }
    if py_truthy(&family.clone()) {
    return add(&add(&py_str("I have your surname, "), &family.clone()), &py_str(", but you've not told me your first name."));
    }
    let _ = py_setitem(&memory.clone(), &py_str("asked_for"), py_str("name"));
    return _vary(&[py_list(vec![py_str("You haven't told me your name yet."), py_str("I don't have a name from you — you've not said.")]), memory.clone()]);
    Py::None
}
pub fn _the_first_name_reply(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    first = _the_first_name(&[memory.clone()]);
    if py_truthy(&first.clone()) {
    return _vary(&[py_list(vec![add(&first.clone(), &py_str(" — you told me.")), add(&add(&py_str("It's "), &first.clone()), &py_str(".")), add(&add(&py_str("Your first name's "), &first.clone()), &py_str("."))]), memory.clone()]);
    }
    let _ = py_setitem(&memory.clone(), &py_str("asked_for"), py_str("name"));
    return _vary(&[py_list(vec![py_str("You haven't told me your first name yet."), py_str("I don't have it — you've not said your first name.")]), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.surname".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_AGAIN" => Some(g_ASKS_AGAIN()),
            "ASKS_THE_FIRST" => Some(g_ASKS_THE_FIRST()),
            "ASKS_THE_FULL" => Some(g_ASKS_THE_FULL()),
            "ASKS_THE_SURNAME" => Some(g_ASKS_THE_SURNAME()),
            "A_SURNAME_WORD" => Some(g_A_SURNAME_WORD()),
            "DESCRIBES_A_NAME" => Some(g_DESCRIBES_A_NAME()),
            "LOWER_PARTS" => Some(g_LOWER_PARTS()),
            "NAME_PARTS" => Some(g_NAME_PARTS()),
            "NEVER_A_NAME" => Some(g_NEVER_A_NAME()),
            "NOT_THE_ASK" => Some(g_NOT_THE_ASK()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "SAYS_A_SURNAME" => Some(g_SAYS_A_SURNAME()),
            "THEIR_OWN" => Some(g_THEIR_OWN()),
            "THE_NAME_WORD" => Some(g_THE_NAME_WORD()),
            "_asks_for_it" => Some(py_func("_asks_for_it", |args: &[Py]| _asks_for_it(args))),
            "_given_one" => Some(py_func("_given_one", |args: &[Py]| _given_one(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_the_first_name" => Some(py_func("_the_first_name", |args: &[Py]| _the_first_name(args))),
            "_the_first_name_reply" => Some(py_func("_the_first_name_reply", |args: &[Py]| _the_first_name_reply(args))),
            "_the_full_name" => Some(py_func("_the_full_name", |args: &[Py]| _the_full_name(args))),
            "_the_lines_before" => Some(py_func("_the_lines_before", |args: &[Py]| _the_lines_before(args))),
            "_the_name_they_said" => Some(py_func("_the_name_they_said", |args: &[Py]| _the_name_they_said(args))),
            "_vary" => Some(py_func("_vary", |args: &[Py]| _vary(args))),
            "a_follow_up" => Some(py_func("a_follow_up", |args: &[Py]| a_follow_up(args))),
            "a_note" => Some(py_func("a_note", |args: &[Py]| a_note(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_surname" => Some(py_func("the_surname", |args: &[Py]| the_surname(args))),
            "there_is_a_chat" => Some(py_func("there_is_a_chat", |args: &[Py]| there_is_a_chat(args))),
            _ => None,
        }),
        set: None,
    }))
}

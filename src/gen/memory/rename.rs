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


pub static AFTER_A_NEW_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_IN_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NEW_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RENAMING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PERSONS_OWN_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AFTER_A_NEW_NAME() -> Py { ensure_init(); AFTER_A_NEW_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_NEW_NAME(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_A_NEW_NAME.write() { *w = v; } }
pub fn g_A_PERSON_IN_THE_LINE() -> Py { ensure_init(); A_PERSON_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_IN_THE_LINE.write() { *w = v; } }
pub fn g_A_THING_IN_FRONT() -> Py { ensure_init(); A_THING_IN_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_IN_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_IN_FRONT.write() { *w = v; } }
pub fn g_NOT_A_NEW_NAME() -> Py { ensure_init(); NOT_A_NEW_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NEW_NAME(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NEW_NAME.write() { *w = v; } }
pub fn g_RENAMING() -> Py { ensure_init(); RENAMING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RENAMING(v: Py) { ensure_init(); if let Ok(mut w) = RENAMING.write() { *w = v; } }
pub fn g_THE_PERSONS_OWN_NAME() -> Py { ensure_init(); THE_PERSONS_OWN_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PERSONS_OWN_NAME(v: Py) { ensure_init(); if let Ok(mut w) = THE_PERSONS_OWN_NAME.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A pet whose name has moved in the sentence: \"we renamed him max\", \"hes max now\".\n\nThe store reads a name put right (\"no hes called max\") and a name given (\"my dog is called rex\"),\nbut not the line that says in as many words that the name has changed, and it kept the name it\nhad:\n\n```\nmy dog is called rex / we renamed him max / whats my dog called\n  before: You've got a dog called Rex.\n  after:  You've got a dog called Max.\n```\n\nThe frames are closed because each of them says a name has changed: `renamed`, `i changed it to`,\n`we call him ... now`, `he answers to`, `now called`, `hes max now`. A line with only a pronoun\nand a name in it (\"hes max\") is as often a fact about somebody as a rename, so it is not read\nhere.\n\nTwo things keep this off the lines that only look like a rename. The name has to stand bare (a\ndeterminer in front of it makes it a thing: \"we renamed the pub\", \"he goes by the gate now\"), and\nthe line has to be about the animal - a person in it is somebody else's name (\"my son changed his\nname to sam\", \"i changed my name to max\").\n\nThis lives here, and not in `bot/modules/memory_store.py`, because that file is at the size the\nruntime allows; the store calls it the way it calls the other memory readers.\n");
    s_RENAMING(stdlib_call("re", "compile", &[py_str("\\brenam(?:e|ed|es|ing)\\b|\\bchang(?:e|ed|es|ing)\\b[^.?!]{0,24}?\\bnames?\\b|\\bchang(?:e|ed|es|ing)\\s+(?:it|him|her|them)\\s+(?:to|into)\\b|\\bcall\\s+(?:him|her|it|them)\\b[^.?!]*\\bnow\\b|\\banswers?\\s+to\\b|\\bgoes?\\s+by\\b|\\bnow\\s+(?:called|named)\\b|\\b(?:he|she|it|they)(?:'s|s| is| are)\\s+[a-z'-]{2,}\\s+now\\b|\\bnow\\s+(?:he|she|it|they)(?:'s|s| is)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_A_NEW_NAME(call_builtin("frozenset", &[mcall(&py_str("now instead please ok okay tbh tho though then actually too also\nfrom onwards today anyway honestly like"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NEW_NAME(call_builtin("frozenset", &[mcall(&py_str("it him her them the to as called named name rename renamed\nthey we i you us me dog cat puppy kitten pet animal"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_IN_FRONT(call_builtin("frozenset", &[mcall(&py_str("the a an this that these those my our his their your its\nsome any another one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:son|daughter|mum|mom|mother|father|dad|wife|husband|partner|boyfriend|girlfriend|brother|sister|friend|boss|colleague|neighbour|neighbor|child|kid|baby|gran|grandma|grandad|grandpa|nan|nana|aunt|uncle|cousin|mate|flatmate|housemate|carer|teacher|people|someone|somebody)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PERSONS_OWN_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+names?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn moves(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says in as many words that a name has changed.");
    return call_builtin("bool", &[mcall(&g_RENAMING(), "search", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_name_it_gave(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The name at the end of a line that says the name has moved, or \"\".\n\n    People put the new name where the new name goes - at the end: \"we renamed him max\", \"i changed\n    it to max\", \"he is now called max\", \"hes max now\". Only the last word that is not a word the\n    line trails off with is read, and only when it holds up as a name on its own\n    (`facts.a_name_could_be`), so \"i changed it to friday\" and \"we renamed him to the vet\" keep\n    their own reading.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:'\"")])); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_A_NEW_NAME(), &mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    word = geti(&words.clone(), &neg(&Py::Int(1)));
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_NEW_NAME(), &mcall(&word.clone(), "lower", &[]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_THING_IN_FRONT(), &mcall(&geti(&words.clone(), &neg(&Py::Int(2))), "lower", &[]))) } }) {
    return py_str("");
    }
    return if py_truthy(&crate::gen::memory::facts::a_name_could_be(&[word.clone()])) { word.clone() } else { py_str("") };
    Py::None
}
pub fn the_name_a_line_moved_on(args: &[Py]) -> Py {
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
    let mut pets: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut animal_named: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal_named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal_named"))) };
    let mut pet: Py = Py::None;
    let _ = py_str("The new name for the animal a rename line is about, or \"\".\n\n    `pets` is the store's list and `animal_named` says whether the line names the animal (\"we\n    renamed the dog milo\"). Either the line names the animal, or the rename verb has a bare\n    pronoun for its object and the store holds one named animal, which is what the pronoun is\n    (\"we renamed him max\", \"hes max now\"). A line that names a person, or says the person's own\n    name has changed, is never the pet's rename.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PERSON_IN_THE_LINE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_PERSONS_OWN_NAME(), "search", &[low.clone()]) } } } }) {
    return py_str("");
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = pets.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pets.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal_named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")])) {  __out.push(pet.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    return py_str("");
    }
    return the_name_it_gave(&[low.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.rename".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_A_NEW_NAME" => Some(g_AFTER_A_NEW_NAME()),
            "A_PERSON_IN_THE_LINE" => Some(g_A_PERSON_IN_THE_LINE()),
            "A_THING_IN_FRONT" => Some(g_A_THING_IN_FRONT()),
            "NOT_A_NEW_NAME" => Some(g_NOT_A_NEW_NAME()),
            "RENAMING" => Some(g_RENAMING()),
            "THE_PERSONS_OWN_NAME" => Some(g_THE_PERSONS_OWN_NAME()),
            "moves" => Some(py_func("moves", |args: &[Py]| moves(args))),
            "the_name_a_line_moved_on" => Some(py_func("the_name_a_line_moved_on", |args: &[Py]| the_name_a_line_moved_on(args))),
            "the_name_it_gave" => Some(py_func("the_name_it_gave", |args: &[Py]| the_name_it_gave(args))),
            _ => None,
        }),
        set: None,
    }))
}

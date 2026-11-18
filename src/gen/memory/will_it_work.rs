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


pub static ACHIEVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEW_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOINS_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_ACHIEVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_MOST_QUOTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRAILING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORK_CHECK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES_IN_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ACHIEVES() -> Py { ensure_init(); ACHIEVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACHIEVES(v: Py) { ensure_init(); if let Ok(mut w) = ACHIEVES.write() { *w = v; } }
pub fn g_A_NEW_CLAUSE() -> Py { ensure_init(); A_NEW_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEW_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = A_NEW_CLAUSE.write() { *w = v; } }
pub fn g_JOINS_ON() -> Py { ensure_init(); JOINS_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOINS_ON(v: Py) { ensure_init(); if let Ok(mut w) = JOINS_ON.write() { *w = v; } }
pub fn g_NOT_ACHIEVING() -> Py { ensure_init(); NOT_ACHIEVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_ACHIEVING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_ACHIEVING.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_THE_MOST_QUOTED() -> Py { ensure_init(); THE_MOST_QUOTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_MOST_QUOTED(v: Py) { ensure_init(); if let Ok(mut w) = THE_MOST_QUOTED.write() { *w = v; } }
pub fn g_TRAILING_FILLER() -> Py { ensure_init(); TRAILING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRAILING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = TRAILING_FILLER.write() { *w = v; } }
pub fn g_WORK_CHECK() -> Py { ensure_init(); WORK_CHECK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORK_CHECK(v: Py) { ensure_init(); if let Ok(mut w) = WORK_CHECK.write() { *w = v; } }
pub fn g_YES_IN_FRONT() -> Py { ensure_init(); YES_IN_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES_IN_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = YES_IN_FRONT.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A line that asks whether the advice will work is answered with what the advice achieves.\n\n\"will that work\" names nothing of its own. After a how-to it is not a new question and not small\ntalk: it is the one thing a person asks before they spend an hour on someone else's method. On\nmain it was answered with a stock nod to the method (\"Done that way, it needs no one else: that is\nthe method I would use.\"), which says nothing about whether the method works:\n\n    how do i clean a burnt pan\n      Put an inch of water and two tablespoons of bicarbonate of soda in it and simmer for ten ...\n    will that work\n      Done that way, it needs no one else: that is the method I would use.\n\nThe card has already answered it, in its own words - \"almost nothing needs scrubbing after that\",\n\"the glass catches the rest in a day or two\", \"a post with height lets her do it properly\" - so the\nreply is that clause, with the yes the person asked for in front of it. Nothing is judged and\nnothing is added: a card with no such clause passes, and the line is answered as it was.\n\nNarrow on purpose:\n\n* the line has to be *only* the check - \"will that work\", \"does that help\", \"would that fix it\" -\n  with nothing of its own after it, because \"will that work for a dark stain too\" asks about a\n  stain;\n* the bot's last turn has to be a card it really said (`measure._the_card_said_last`, the same\n  check the measure follow-up uses), so a chat line is never quoted back as an answer;\n* the clause has to say the method *achieves* something, and a clause that says it achieves\n  nothing (\"rice does nothing except leave grit in the ports\") is not that.\n");
    s_WORK_CHECK(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:will|would|does|do|might|should|can)\\s+(?:that|it|this|they|those)\\s+(?:work|help|do\\s+(?:it|the\\s+job)|fix\\s+(?:it|that)|be\\s+(?:enough|ok|okay|all\\s+right|alright|fine)|make\\s+a\\s+difference)(?:\\s+(?:for|on)\\s+(?:me|us|him|her|them|this|that))?(?:\\s+(?:then|though|now|too))?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACHIEVES(stdlib_call("re", "compile", &[py_str("\\b(?:almost nothing|nothing needs|nothing to scrub|does the job|does the trick|catches|clears|lifts|comes away|comes off|gets rid of|no more|beats|kills|sinks|survives|stops?|fixes|sorts|dries|settles|holds|works|helps?|lets (?:her|him|them|you)|keeps (?:them|it) (?:off|away|out))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ACHIEVING(stdlib_call("re", "compile", &[py_str("\\b(?:does nothing|nothing except|does not|doesn't|dont|don't|never|won't|wont|isn't|is not|not\\s+enough|makes it worse)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINS_ON(call_builtin("frozenset", &[py_tuple(vec![py_str("which"), py_str("that"), py_str("who"), py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("because"), py_str("though"), py_str("while"), py_str("as"), py_str("if"), py_str("when"), py_str("also")])]).unwrap_or_else(|e| exc::raise(e)));
    s_YES_IN_FRONT(py_tuple(vec![py_str("Yes"), py_str("It does"), py_str("That does it"), py_str("Yes, it does")]));
    s_THE_MOST_QUOTED(Py::Int(11));
    s_TRAILING_FILLER(py_tuple(vec![py_str("too"), py_str("as well"), py_str("also"), py_str("then"), py_str("though")]));
    s_A_NEW_CLAUSE(call_builtin("frozenset", &[mcall(&py_str("and so but or which is are was were then because when if that\nit they there this these with plus while as"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn asks_if_it_will_work(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is nothing but a check that the advice works.");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    return { let __b = call_builtin("bool", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_WORK_CHECK(), "match", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_clauses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The clauses of a sentence, in order: a card puts the method in one and its result in the\n    next (\"then lift the burnt layer with a wooden spoon: almost nothing needs scrubbing\").");
    parts = stdlib_call("re", "split", &[py_str("[,;:]\\s+|\\s+-\\s+"), call_builtin("str", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[py_str(" .")])) {  __out.push(mcall(&part.clone(), "strip", &[py_str(" .")])); } } py_list(__out) })() };
    Py::None
}
pub fn _without_filler(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut filler: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The clause without the words that only join it to the rest of the card (\"... help too\").");
    said = mcall(&clause.clone(), "strip", &[py_str(" .")]);
    let __iter1 = py_iter_or_raise(&g_TRAILING_FILLER());
    for __item in __iter1 {
    filler = __item.clone();
    if py_truthy(&mcall(&mcall(&said.clone(), "lower", &[]), "endswith", &[add(&py_str(" "), &filler.clone())])) {
    said = mcall(&slice_or_raise(&said.clone(), &Py::None, &sub(&neg(&call_builtin("len", &[filler.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1)), &Py::None), "rstrip", &[py_str(" ,.")]);
    }
    }
    return said.clone();
    Py::None
}
pub fn _what_it_achieves(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut card: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: card"))) };
    let mut clause: Py = Py::None;
    let mut part: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The clause of the card that says what the method achieves, or \"\" when it says nothing.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[call_builtin("str", &[card.clone()]).unwrap_or_else(|e| exc::raise(e))])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ACHIEVES(), "search", &[sentence.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_ACHIEVING(), "search", &[sentence.clone()]) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&_the_clauses(&[sentence.clone()]));
    for __item in __iter2 {
    clause = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ACHIEVES(), "search", &[clause.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_ACHIEVING(), "search", &[clause.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_JOINS_ON(), &mcall(&mcall(&geti(&{ let __b = stdlib_call("re", "split", &[py_str("\\s+"), clause.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.;:")])))) {
    continue;
    }
    clause = _without_filler(&[clause.clone()]);
    if py_truthy(&if py_truthy(&le(&Py::Int(4), &{ let __cmp30 = call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); __cmp30.clone() })) { le(&__cmp30.clone(), &g_THE_MOST_QUOTED()) } else { Py::Bool(false) }) {
    return clause.clone();
    }
    }
    }
    return py_str("");
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
    let mut at: Py = Py::None;
    let mut card: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The yes with the card's own account of why, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&asks_if_it_will_work(&[text.clone()])))) {
    return Py::None;
    }
    card = crate::gen::memory::measure::_the_card_said_last(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    said = _what_it_achieves(&[card.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&mcall(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lstrip", &[]), "lower", &[]), "startswith", &[py_tuple(vec![py_str("yes"), py_str("no")])])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::Int(2), &Py::None), "islower", &[]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::Int(2), &Py::None), &py_str(" "))); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&said.clone(), &Py::Int(2), &Py::Int(3), &Py::None), "islower", &[]) } } } } } }) {
    said = add(&mcall(&geti(&said.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = mcall(&said.clone(), "rstrip", &[py_str(" .")]);
    at = modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) { let mut letter: Py = __item.clone(); __out.push(call_builtin("ord", &[letter.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_YES_IN_FRONT()]).unwrap_or_else(|e| exc::raise(e)));
    return modulo(&py_str("%s — %s."), &py_tuple(vec![geti(&g_YES_IN_FRONT(), &at.clone()), said.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.will_it_work".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACHIEVES" => Some(g_ACHIEVES()),
            "A_NEW_CLAUSE" => Some(g_A_NEW_CLAUSE()),
            "JOINS_ON" => Some(g_JOINS_ON()),
            "NOT_ACHIEVING" => Some(g_NOT_ACHIEVING()),
            "SENTENCE" => Some(g_SENTENCE()),
            "THE_MOST_QUOTED" => Some(g_THE_MOST_QUOTED()),
            "TRAILING_FILLER" => Some(g_TRAILING_FILLER()),
            "WORK_CHECK" => Some(g_WORK_CHECK()),
            "YES_IN_FRONT" => Some(g_YES_IN_FRONT()),
            "_the_clauses" => Some(py_func("_the_clauses", |args: &[Py]| _the_clauses(args))),
            "_what_it_achieves" => Some(py_func("_what_it_achieves", |args: &[Py]| _what_it_achieves(args))),
            "_without_filler" => Some(py_func("_without_filler", |args: &[Py]| _without_filler(args))),
            "asks_if_it_will_work" => Some(py_func("asks_if_it_will_work", |args: &[Py]| asks_if_it_will_work(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static ADVICE_MODULES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_IF_IT_HELPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ATTEMPT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PROBLEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ADVICE_MODULES() -> Py { ensure_init(); ADVICE_MODULES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_MODULES(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_MODULES.write() { *w = v; } }
pub fn g_ASK_IF_IT_HELPED() -> Py { ensure_init(); ASK_IF_IT_HELPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_IF_IT_HELPED(v: Py) { ensure_init(); if let Ok(mut w) = ASK_IF_IT_HELPED.write() { *w = v; } }
pub fn g_ATTEMPT() -> Py { ensure_init(); ATTEMPT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ATTEMPT(v: Py) { ensure_init(); if let Ok(mut w) = ATTEMPT.write() { *w = v; } }
pub fn g_A_PROBLEM() -> Py { ensure_init(); A_PROBLEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PROBLEM(v: Py) { ensure_init(); if let Ok(mut w) = A_PROBLEM.write() { *w = v; } }
pub fn g_A_THING_LEADS() -> Py { ensure_init(); A_THING_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_LEADS.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); NOT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_THING.write() { *w = v; } }
pub fn g_POINTING() -> Py { ensure_init(); POINTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); if let Ok(mut w) = POINTING.write() { *w = v; } }
pub fn g_VERDICT() -> Py { ensure_init(); VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A named thing they tried after advice, said back as a question about whether it helped.\n\n\"my back hurts\" is answered with what usually helps, and the next line - \"i tried a hot bath\" - is\nnot news and not a new question: it is where they are with the advice, and the thing a friend says\nto it is whether it did anything. The bot said \"Good to hear. What was the best bit?\", which reads a\nremedy that has just been tried as a good time they had.\n\nThis reader answers the line only when all four of these hold, so nothing else is taken:\n\n- the line is an attempt at something, said as a person does (\"i tried\", \"ive just tried\", \"i took\",\n  \"i used\", \"we gave it a go\"), and it names the thing (\"a hot bath\", \"some paracetamol\");\n- it carries no verdict of its own (\"it didnt help\", \"no better\", \"that helped\", \"it made it\n  worse\"), because a report on the answer is `bot/memory/tried.py`'s line;\n- the thing is a thing and not a pronoun - \"i tried that\" is the answer's own steps, not this;\n- the turn before was advice (`answers` or `safety`), and the chat has a problem in it, so a happy\n  \"i tried the new thai place\" after a where-to-eat answer is left where it was.\n\nEverything else is passed on, unchanged, to the modules that were going to answer it.\n");
    s_ATTEMPT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|ok|okay|well|right|alright)\\s+)?(?:i|i've|ive|i'd|we|we've|weve)\\s*(?:have\\s+|had\\s+|just\\s+|already\\s+|also\\s+|finally\\s+)?(?P<verb>tried|took|used|gave|been using|been trying)\\s+(?P<thing>.+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_LEADS(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|some|my|his|her|their|our|that|this|one|two|three)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:did\\s?n'?t|does\\s?n'?t|not|no|never|helped|helps|worked|works|better|worse|relief|eased|eases|nothing|useless|pointless|yet|again|still)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING(call_builtin("frozenset", &[mcall(&py_str("that this it them those these everything all anything something\nnothing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PROBLEM(call_builtin("frozenset", &[mcall(&py_str("hurt hurts hurting hurtful pain painful ache aches aching achy achey sore\nill sick illness unwell cough coughing cold flu headache migraine fever temperature back knee\nshoulder neck ankle wrist hip throat stomach belly tooth toothache ear earache rash itch itchy burn\nburnt sprain sprained stiff stiffness swelling swollen dizzy faint sleep\nsleeping slept insomnia wake waking wakes sneeze sneezing blocked stuffy running congestion\nbleeding bleed cut bruise bruised lump dizzy nausea sick vomit diarrhoea constipated constipation heartburn\nreflux bloat bloated cramp cramps period contractions\ndripping drips leaking leaks leaked leak broken broke cracked squeaks squeaking squeak clogged\nblocked banging rattling stuck jammed seized mould mildew damp stains stained smell smells\nwont won'ts starts start starting\nfreezing freezes frozen slow slowly crashing crashes crashed glitching lagging laggy dropping\ndrops disconnects restart reboots overheating"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_MODULES(call_builtin("frozenset", &[py_tuple(vec![py_str("answers"), py_str("safety")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_IF_IT_HELPED(py_tuple(vec![py_str("Did the {thing} take the edge off, or nothing?"), py_str("{Thing} - did that do anything?"), py_str("And did the {thing} make a difference?"), py_str("{Thing} - any help at all?"), py_str("How did the {thing} go - anything, or no difference?")]));
    s_LEADS(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|oh|ok|okay|well|right|alright)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("to for at in on with about of it that this"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut match_: Py = Py::None;
    let mut options: Py = Py::None;
    let mut said: Py = Py::None;
    let mut template: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The question about the thing they tried, or \"\" when this is not that line.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    match_ = mcall(&g_ATTEMPT(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = _the_thing(&[mcall(&match_.clone(), "group", &[py_str("thing")]), mcall(&match_.clone(), "group", &[py_str("verb")])]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_VERDICT(), "search", &[{ let __b = slice_or_raise(&mcall(&match_.clone(), "group", &[py_str("thing")]), &call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_problem_was_spoken_of(&[memory.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_advice_was_given(&[memory.clone()]))) } }) {
    return py_str("");
    }
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_IF_IT_HELPED()) { let mut template: Py = __item.clone(); __out.push(_fill(&[template.clone(), thing.clone()])); } py_list(__out) })() };
    return _fresh(&[options.clone(), memory.clone()]);
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut verb: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The named thing they tried, without the article, as the words a sentence can carry.");
    said = mcall(&call_builtin("str", &[{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    said = geti(&stdlib_call("re", "split", &[py_str("\\b(?:but|so|then|though|because|again|twice|afterwards)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    said = geti(&stdlib_call("re", "split", &[py_str("\\band\\b(?=\\s+(?:it|its|it's|they|i|we|now|still|nothing|didnt|doesnt|no\\b|the\\s+\\w+\\s+(?:did|helped|worked)))"), said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POINTING(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("to"))) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&verb.clone(), &py_str("gave")))) {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&slice_or_raise(&words.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None), &py_list(vec![py_str("a"), py_str("go")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&words.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None), &py_list(vec![py_str("a"), py_str("try")]))) } }) {
    return py_str("");
    }
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None);
    }
    words = slice_or_raise(&words.clone(), &Py::None, &Py::Int(5), &Py::None);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("my"), py_str("our"), py_str("their")]), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("my"), py_str("our"), py_str("their"), py_str("his"), py_str("her"), py_str("of"), py_str("that"), py_str("this")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    if py_truthy(&lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("of")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _advice_was_given(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the chat holds an answer for them to be partway through.\n\n    Read over the last few turns, not just the one before: \"i tried a hot bath\" then \"i tried a\n    heat pad\" is a second attempt at the same advice, and the turn in between is this module's own\n    question, which is not advice of its own.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() } } else { py_list(vec![]) };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None), &Py::None, &Py::None, &neg(&Py::Int(1))));
    for __item in __iter1 {
    turn = __item.clone();
    module = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_ADVICE_MODULES(), &module.clone()))) {
    return Py::Bool(true);
    }
    reply = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:try|trying|keep|rest|ice|see (?:a|your) (?:gp|doctor))\\b"), mcall(&reply.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&reply.clone(), "endswith", &[py_str("?")])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_problem_was_spoken_of(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the last few turns have a problem in them, which is what an attempt is at.\n\n    Read over the last few turns rather than all of them: an attempt three turns after a problem\n    that has been answered and moved on from is a new experience (\"i tried the thai place\"), and\n    the reply it already had is the right one.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() } } else { py_list(vec![]) };
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    let _ = mcall(&words.clone(), "update", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return call_builtin("bool", &[bitand(&words.clone(), &g_A_PROBLEM())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fill(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut template: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "template") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: template"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let _ = py_str("The template with the thing in it, capitalised where it opens the sentence.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&template.clone(), "format", args))(&__args) };
    Py::None
}
pub fn _fresh(args: &[Py]) -> Py {
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
    let mut option: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut unseen: Py = Py::None;
    let _ = py_str("An option that has not just been said, so the same question does not come twice.");
    said = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]) } else { py_str("") };
    unseen = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mcall(&said.clone(), "lower", &[]), &mcall(&option.clone(), "lower", &[])))) {  __out.push(option.clone()); } } py_list(__out) })() };
    pool = { let __b = unseen.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    return mcall(&stdlib_call("random", "Random", &[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) }]).unwrap_or_else(|e| exc::raise(e)), "choice", &[pool.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.remedy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE_MODULES" => Some(g_ADVICE_MODULES()),
            "ASK_IF_IT_HELPED" => Some(g_ASK_IF_IT_HELPED()),
            "ATTEMPT" => Some(g_ATTEMPT()),
            "A_PROBLEM" => Some(g_A_PROBLEM()),
            "A_THING_LEADS" => Some(g_A_THING_LEADS()),
            "LEADS" => Some(g_LEADS()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "POINTING" => Some(g_POINTING()),
            "VERDICT" => Some(g_VERDICT()),
            "_a_problem_was_spoken_of" => Some(py_func("_a_problem_was_spoken_of", |args: &[Py]| _a_problem_was_spoken_of(args))),
            "_advice_was_given" => Some(py_func("_advice_was_given", |args: &[Py]| _advice_was_given(args))),
            "_fill" => Some(py_func("_fill", |args: &[Py]| _fill(args))),
            "_fresh" => Some(py_func("_fresh", |args: &[Py]| _fresh(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

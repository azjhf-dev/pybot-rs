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


pub static AMOUNTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMOUNTS_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARRY_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FINISHED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHILE_IT_LASTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COUNTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HAD_A_GO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OPENS_ON_THE_INSTRUCTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AMOUNTS() -> Py { ensure_init(); AMOUNTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNTS(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNTS.write() { *w = v; } }
pub fn g_AMOUNTS_LEADS() -> Py { ensure_init(); AMOUNTS_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNTS_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNTS_LEADS.write() { *w = v; } }
pub fn g_CARRY_ON() -> Py { ensure_init(); CARRY_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARRY_ON(v: Py) { ensure_init(); if let Ok(mut w) = CARRY_ON.write() { *w = v; } }
pub fn g_DONE() -> Py { ensure_init(); DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DONE(v: Py) { ensure_init(); if let Ok(mut w) = DONE.write() { *w = v; } }
pub fn g_FINISHED() -> Py { ensure_init(); FINISHED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FINISHED(v: Py) { ensure_init(); if let Ok(mut w) = FINISHED.write() { *w = v; } }
pub fn g_FIRST_LEADS() -> Py { ensure_init(); FIRST_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_LEADS.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_NOT_A_JOB() -> Py { ensure_init(); NOT_A_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_JOB.write() { *w = v; } }
pub fn g_WHILE_IT_LASTS() -> Py { ensure_init(); WHILE_IT_LASTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHILE_IT_LASTS(v: Py) { ensure_init(); if let Ok(mut w) = WHILE_IT_LASTS.write() { *w = v; } }
pub fn g__COUNTS() -> Py { ensure_init(); _COUNTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COUNTS(v: Py) { ensure_init(); if let Ok(mut w) = _COUNTS.write() { *w = v; } }
pub fn g__HAD_A_GO() -> Py { ensure_init(); _HAD_A_GO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HAD_A_GO(v: Py) { ensure_init(); if let Ok(mut w) = _HAD_A_GO.write() { *w = v; } }
pub fn g__NOT_THE_THING() -> Py { ensure_init(); _NOT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_THE_THING.write() { *w = v; } }
pub fn g__OPENS_ON_THE_INSTRUCTION() -> Py { ensure_init(); _OPENS_ON_THE_INSTRUCTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OPENS_ON_THE_INSTRUCTION(v: Py) { ensure_init(); if let Ok(mut w) = _OPENS_ON_THE_INSTRUCTION.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The order in an answer, kept in the chat, so \"what next\" moves on instead of starting again.\n\nAn answer that gives a sequence (\"turn the water off at the isolating valve, take the handle off,\nand take the old part to a hardware shop\") has an order in it, and a person working through it says\n\"what next\" when they have done one step. The chat had no memory of the order, so on main the tap\nchat went:\n\n    my bathroom tap drips\n    what next   -> Start with this: Turn the water off at the isolating valve under the sink, take\n                   the handle off, and take the old part to a hardware shop to match it.\n    and then    -> That's as far as I can take it. Say the word and we will go back over a piece.\n    what next   -> From just now: Turn the water off at the isolating valve under the sink...\n\nThe first step again, then a goodbye closing a job the person is halfway through, then the whole\ncard back. Two things are the store's: the step handed over is never remembered, so there is no\nnext one to give; and no module knows the answer has an order at all, so the nudge module decides\nit has run out. This module keeps the steps of the answer in hand and the step the chat has\nreached, and answers the bare asks that want the following one.\n\nWhat it does not do: write a step of its own. Every line here is a step the answer already gave, in\nthe answer's own words, in the answer's own order. The reading is `repeat._the_steps`, the same one\nthe \"what else\" readers use, so a sentence this module hands over is a sentence another reader in\nthe package would have called a step too.\n\nThe cursor is kept under `memory[\"steps\"]` - the key the conversation lane's doc lists - as the\nanswer it belongs to and how many of its steps have been handed over. It is not `memory[\"plan\"]`\nbecause the person's own plan is `user[\"plan\"]`, and one name for two things is how a store starts\nanswering about the wrong one. It starts again when the\nanswer changes, because a step count from the last answer is about a different job.\n");
    s_CARRY_ON(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|right|so|yeah|yes|alright|and|then|well|ah)\\s+)*(?:and\\s+then|then\\s+what|what\\s+next|what\\s+now|next|and\\s+after\\s+that|after\\s+that|and\\s+next|go\\s+on|carry\\s+on|keep\\s+going)\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_LEADS(py_tuple(vec![py_str("Start with this: {step}."), py_str("Begin here: {step}."), py_str("The thing to do first: {step}."), py_str("Do this bit first: {step}.")]));
    s_AMOUNTS_LEADS(py_tuple(vec![py_str("First up: {step}."), py_str("Begin with the mix: {step}."), py_str("Start with what goes in: {step}.")]));
    s__OPENS_ON_THE_INSTRUCTION(stdlib_call("re", "compile", &[py_str("^(?:start(?:\\s+(?:with|by|off))?|begin(?:\\s+(?:with|by))?|first(?:ly)?|to\\s+start|the\\s+first\\s+thing)\\b(?:[,:]|\\s)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__COUNTS(py_dict(vec![(Py::Int(2), py_str("I make it two steps.")), (Py::Int(3), py_str("I make it three steps.")), (Py::Int(4), py_str("I make it four steps.")), (Py::Int(5), py_str("I make it five steps."))]));
    s_LEADS(py_tuple(vec![py_str("Next: {step}."), py_str("Then: {step}."), py_str("The next bit: {step}."), py_str("Once that is done: {step}."), py_str("The step after that: {step}.")]));
    s_DONE(py_tuple(vec![py_str("There is nothing after that bit that I can pick out. The last of it is: {step}. Where have you got to?"), py_str("That is the last step I can pick out of it: {step}. Say where you are and I'll help with the rest."), py_str("I have nothing more to hand you after: {step}. Tell me which bit you are on.")]));
    s_FINISHED(py_tuple(vec![py_str("That is every step I can pick out of it - nothing left of what I can hand you. Tell me how it went."), py_str("There is nothing after that I can pick out. Say what happened and we will go from there."), py_str("Nothing more I can pull out of that answer. How did it go?")]));
    s_NOT_A_JOB(py_tuple(vec![py_str("loss"), py_str("grief"), py_str("safety")]));
    s_WHILE_IT_LASTS(Py::Int(3));
    s__NOT_THE_THING(py_set(vec![py_str("take"), py_str("have"), py_str("with"), py_str("that"), py_str("this"), py_str("what"), py_str("when"), py_str("them"), py_str("they"), py_str("your"), py_str("from"), py_str("into"), py_str("just"), py_str("like"), py_str("some"), py_str("then"), py_str("than"), py_str("also"), py_str("once"), py_str("done"), py_str("make"), py_str("over"), py_str("will"), py_str("back"), py_str("again"), py_str("away"), py_str("each"), py_str("every"), py_str("about"), py_str("after"), py_str("before"), py_str("under"), py_str("while"), py_str("there"), py_str("here"), py_str("been"), py_str("being")]));
    s__HAD_A_GO(stdlib_call("re", "compile", &[py_str("\\b(?:tried|trying|already|gave it|had a go|no luck|used)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AMOUNTS(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:g|kg|ml|l|oz|lb|tsp|tbsp|teaspoons?|tablespoons?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_thing_named(args: &[Py]) -> Py {
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
    let _ = py_str("The words in a line that name a thing to be done, without the grammar around them.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_THE_THING(), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _tried(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when a line just before this one reports having done this very thing already.\n\n    `repeat._they_say_they_tried` reads the same reports but counts any two shared words, and \"the\"\n    and \"off\" are two: \"ive already turned the water off\" made it skip \"take the handle off\" as\n    done. Here only the words that name the thing count, so a step is skipped when the person named\n    it and not when the two sentences share their grammar.\n    ");
    mine = _the_thing_named(&[step.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__HAD_A_GO(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&bitand(&mine.clone(), &_the_thing_named(&[line.clone()]))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_last_answer(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's last line in this chat, or \"\" when there is none.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    return call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_str("");
    Py::None
}
pub fn the_answer_in_hand(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut last: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("The whole answer the last line was a step of, or \"\" when the last line is not one of its\n    steps.\n\n    A step handed over is one sentence of the answer (\"Start with this: Turn the water off at the\n    isolating valve under the sink. There are three steps in all.\"), so a module that reads the\n    bot's last line to find what the answer says next sees an answer with nothing after it:\n    \"ive already turned the water off\" after that line found no step left and asked how it went,\n    where the answer still had two more. The plan keeps the whole answer, so this hands it back\n    and the readers of the last answer (bot/memory/already.py) read the answer, not one step of it.\n    ");
    plan = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("steps")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    answer = mcall(&call_builtin("str", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("answer")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    last = mcall(&mcall(&py_str(" "), "join", &[mcall(&the_last_answer(&[memory.clone()]), "split", &[])]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&last.clone(), &mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[])))) {
    return answer.clone();
    }
    return if py_truthy(&_our_own_line(&[last.clone(), plan.clone()])) { answer.clone() } else { py_str("") };
    Py::None
}
pub fn _module_of_last(args: &[Py]) -> Py {
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
    let _ = py_str("The module that answered the last line, or \"\" when the chat has not recorded one.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    return call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_str("");
    Py::None
}
pub fn _last_user(args: &[Py]) -> Py {
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
    let _ = py_str("The person's last line in this chat, or \"\" when there is none.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    return call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_cut_at_a_colon(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut head: Py = Py::None;
    let mut sep: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("A step cut off at a colon with a fragment after it is the step above the colon.\n\n    \"Cut the distance and the speed for a week or two: most running knee\" is how the shared reader\n    splits \"Cut the distance and the speed for a week or two: most running knee pain settles in a\n    few weeks.\" - the sentence after the colon is the reason, and its subject was taken for the\n    next instruction. What is above the colon is a step of its own and a friend hands that over;\n    what is below it is half a sentence.\n    ");
    let __unpk1 = py_unpack(&mcall(&step.clone(), "partition", &[py_str(": ")]), 3, None);
    head = __unpk1[0].clone();
    sep = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = sep.clone(); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&tail.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return mcall(&head.clone(), "strip", &[]);
    }
    return step.clone();
    Py::None
}
pub fn steps_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("The instructions in an answer, in the order it gives them, and only when there are two.\n\n    One instruction is not a sequence: \"put a hot water bottle on it\" has nothing after it, and a\n    \"what next\" after that answer wants more of the thinking, not a second step that is not there.\n    Returning [] leaves those turns to the readers that already answer them.\n    ");
    steps = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::repeat::_the_steps(&[answer.clone()])) { let mut step: Py = __item.clone(); __out.push(_a_cut_at_a_colon(&[step.clone()])); } py_list(__out) })() }) { let mut step: Py = __item.clone(); if py_truthy(&step.clone()) {  __out.push(step.clone()); } } py_list(__out) })() };
    steps = _the_amounts_first(&[answer.clone(), steps.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_list(vec![]);
    }
    return steps.clone();
    Py::None
}
pub fn _the_amounts_first(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut steps: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "steps") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: steps"))) };
    let mut first: Py = Py::None;
    let _ = py_str("An answer that opens with what goes in has its first piece of work in that sentence.\n\n    \"500g strong white flour, 7g dried yeast, 10g salt and 350ml warm water, kneaded for 10 minutes\n    until smooth and stretched. Leave it somewhere warm for an hour...\" is the bread answer, and the\n    shared reader passes the first sentence over: it opens on the amounts rather than on a verb. A\n    person halfway through the loaf who says \"what next\" then got \"Leave it somewhere warm\", and\n    the dough was never made. The sentence is the answer's own words and its own first step, so it goes\n    in front, and only when the reader did not already take a step from it.\n    ");
    first = mcall(&geti(&mcall(&crate::gen::memory::repeat::g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&g_AMOUNTS(), "findall", &[first.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return steps.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b[a-z]{3,}ed\\b"), mcall(&first.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))))) {
    return steps.clone();
    }
    if py_truthy(&{ let __b = steps.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&first.clone(), "lower", &[]), &slice_or_raise(&mcall(&geti(&steps.clone(), &Py::Int(0)), "lower", &[]), &Py::None, &Py::Int(40), &Py::None))) } }) {
    return steps.clone();
    }
    return add(&py_list(vec![mcall(&first.clone(), "rstrip", &[py_str(".")])]), &call_builtin("list", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let _ = py_str("The words worth comparing two sentences on: short ones are the grammar, not the subject.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _stems(args: &[Py]) -> Py {
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
    let _ = py_str("Words for comparing two lines on their subject: the endings are the grammar, not the thing.\n\n    \"ive already turned the water off\" and \"turn the water off at the isolating valve\" are about the\n    same valve, and only a stem makes \"turned\" and \"turn\" the same word. Four letters and up:\n    \"the\", \"and\" and \"off\" are in every answer, and a shared \"the\" would make any two lines look\n    like the same job.\n    ");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None)); } py_set(__out) })() };
    Py::None
}
pub fn _the_same_job(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("True when the last thing the bot said is still about the steps in hand.\n\n    Between two asks the chat can say something else about the same job: \"i tried the plunger\" is\n    answered with the later part of the answer, and that line has no sequence of its own, so a\n    reader that only looked at the answer in hand would start the whole thing again. Overlap is\n    what tells the two apart - a job's later part shares the words of its steps, and a reply about\n    something else shares none of them.\n    ");
    said = _words(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("steps")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    step = __item.clone();
    if py_truthy(&bitand(&_words(&[step.clone()]), &said.clone())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_same_topic(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut about: Py = Py::None;
    let mut here: Py = Py::None;
    let mut step: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("True while the chat is still on the subject the steps belong to.\n\n    Between two asks the bot can answer a line of its own about the same job - \"ive already turned\n    the water off\" is answered \"Then that's the first thing done\" - and that answer shares none of\n    the words of the steps. What the person said about the job is what says the steps in hand are\n    still the steps: a chat that has moved on to the boiler shares nothing with the tap, and the\n    tap's order is not its order.\n    ");
    here = bitor(&_stems(&[_last_user(&[memory.clone()])]), &_stems(&[the_last_answer(&[memory.clone()])]));
    if py_truthy(&Py::Bool(!py_truthy(&here.clone()))) {
    return Py::Bool(false);
    }
    about = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("steps")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    step = __item.clone();
    let __aug2 = bitor(&about.clone(), &_stems(&[step.clone()]));
    about = __aug2;
    }
    topic = mcall(&plan.clone(), "get", &[py_str("topic")]);
    let __aug3 = bitor(&about.clone(), &_stems(&[if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&topic.clone(), "get", &[py_str("text")]) } else { py_str("") }]));
    about = __aug3;
    return call_builtin("bool", &[bitand(&here.clone(), &about.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut at: Py = Py::None;
    let mut reached: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("How far into the steps the last bot line got, so a step it already said is not said again.");
    said = _words(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Int(0);
    }
    reached = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("steps")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    step = __unpk2[1].clone();
    if py_truthy(&bitand(&_words(&[step.clone()]), &said.clone())) {
    reached = add(&at.clone(), &Py::Int(1));
    }
    }
    return reached.clone();
    Py::None
}
pub fn _our_own_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut flat: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("True when the last thing the bot said is a step this module handed over.\n\n    The step handed over becomes the bot's last line, so on the next turn the answer in hand *is*\n    that step and it has no sequence in it. Without this the cursor would be thrown away after\n    every step and the chat would start the answer again - which is the fault this module exists\n    to fix, arrived at from the other side.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("steps")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    step = __item.clone();
    flat = mcall(&py_str(" "), "join", &[mcall(&step.clone(), "split", &[])]);
    if py_truthy(&{ let __b = flat.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&said.clone(), "lower", &[]), &mcall(&flat.clone(), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _plan_for(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The steps in hand and the step the chat has reached, made fresh when the answer changes.");
    answer = the_last_answer(&[memory.clone()]);
    plan = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("steps")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "get", &[py_str("steps")]) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&plan.clone(), "get", &[py_str("answer")]), &answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&plan.clone(), "get", &[py_str("said")]), &answer.clone())) } }) {
    return plan.clone();
    }
    if py_truthy(&_our_own_line(&[answer.clone(), plan.clone()])) {
    let _ = py_setitem(&plan.clone(), &py_str("answer"), answer.clone());
    let _ = py_setitem(&plan.clone(), &py_str("at"), call_builtin("max", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, _said_in(&[answer.clone(), plan.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    return plan.clone();
    }
    turns = call_builtin("len", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    fresh = steps_of(&[answer.clone()]);
    if py_truthy(&{ let __b = { let __b = Py::Bool(!py_truthy(&fresh.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&fresh.clone(), &mcall(&plan.clone(), "get", &[py_str("steps")]))) } }; if !py_truthy(&__b) { __b } else { { let __b = lt(&sub(&turns.clone(), &{ let __b = mcall(&plan.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }), &g_WHILE_IT_LASTS()); if !py_truthy(&__b) { __b } else { { let __b = _the_same_topic(&[memory.clone(), plan.clone()]); if py_truthy(&__b) { __b } else { _the_same_job(&[answer.clone(), plan.clone()]) } } } } } }) {
    let _ = py_setitem(&plan.clone(), &py_str("answer"), answer.clone());
    let _ = py_setitem(&plan.clone(), &py_str("at"), call_builtin("max", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, _said_in(&[answer.clone(), plan.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    return plan.clone();
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_JOB(), &_module_of_last(&[memory.clone()])))) {
    return Py::None;
    }
    steps = steps_of(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&steps.clone()))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("answer"), answer.clone()), (py_str("steps"), steps.clone()), (py_str("at"), Py::Int(0)), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("topic"), if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("topic")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { stdlib_call("copy", "deepcopy", &[mcall(&memory.clone(), "get", &[py_str("topic")])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None })]);
    Py::None
}
pub fn carry_on(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut last: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut line: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("The next step of the answer in hand, for a bare \"what next\" or \"and then\". Else None.\n\n    The step count is advanced only by this module, and only when it has just handed a step over,\n    so the order is the answer's order: the first ask gets the first step, the second the second,\n    and a person who says \"ive tried the plunger\" between two asks has the steps they named\n    skipped, which is the whole point of keeping the order rather than searching the answer's words\n    again.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_CARRY_ON(), "match", &[line.clone()])))) {
    return Py::None;
    }
    plan = _plan_for(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    here = call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&plan.clone(), "get", &[py_str("at_turn")]), &here.clone())); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "get", &[py_str("said")]) } }) {
    return geti(&plan.clone(), &py_str("said"));
    }
    steps = geti(&plan.clone(), &py_str("steps"));
    at = { let __b = mcall(&plan.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { Py::Int(0) } };
    while py_truthy(&{ let __b = lt(&at.clone(), &call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _tried(&[geti(&steps.clone(), &at.clone()), memory.clone()]) } }) {
    let __aug1 = add(&at.clone(), &Py::Int(1));
    at = __aug1;
    }
    if py_truthy(&ge(&at.clone(), &call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    last = geti(&steps.clone(), &neg(&Py::Int(1)));
    if py_truthy(&mcall(&plan.clone(), "get", &[py_str("done")])) {
    said = crate::gen::compose::pick(&[g_FINISHED(), memory.clone(), line.clone()]);
    } else {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), add(&mcall(&geti(&last.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&last.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_DONE(), memory.clone(), line.clone()]), "format", args))(&__args) };
    }
    let _ = _record(&[memory.clone(), { let __b = mcall(&plan.clone(), "get", &[py_str("answer")]); if py_truthy(&__b) { __b } else { py_str("") } }, steps.clone(), at.clone(), said.clone()]);
    let _ = py_setitem(&geti(&memory.clone(), &py_str("steps")), &py_str("done"), Py::Bool(true));
    return said.clone();
    }
    step = geti(&steps.clone(), &at.clone());
    if py_truthy(&mcall(&g__OPENS_ON_THE_INSTRUCTION(), "match", &[step.clone()])) {
    said = add(&mcall(&geti(&step.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])))) {
    let __aug2 = add(&said.clone(), &py_str("."));
    said = __aug2;
    }
    } else {
    if py_truthy(&Py::Bool(py_eq(&at.clone(), &Py::Int(0)))) {
    leads = if py_truthy(&mcall(&g_AMOUNTS(), "match", &[step.clone()])) { g_AMOUNTS_LEADS() } else { g_FIRST_LEADS() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), add(&mcall(&geti(&step.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[leads.clone(), memory.clone(), line.clone()]), "format", args))(&__args) };
    } else {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), add(&mcall(&geti(&step.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_LEADS(), &modulo(&sub(&at.clone(), &Py::Int(1)), &call_builtin("len", &[g_LEADS()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&at.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__COUNTS(), &call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    said = { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__COUNTS(), &call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e))), -1, "")); py_string(__s) };
    }
    let _ = _record(&[memory.clone(), { let __b = mcall(&plan.clone(), "get", &[py_str("answer")]); if py_truthy(&__b) { __b } else { py_str("") } }, steps.clone(), add(&at.clone(), &Py::Int(1)), said.clone()]);
    return said.clone();
    Py::None
}
pub fn _record(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut steps: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "steps") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: steps"))) };
    let mut at: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut said: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut before: Py = Py::None;
    let mut same: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Remember where the chat has got to, for the next \"what next\".");
    turn = call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    before = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("steps")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("steps")]) } else { py_dict(vec![]) };
    same = Py::Bool(py_eq(&mcall(&before.clone(), "get", &[py_str("steps")]), &steps.clone()));
    topic = mcall(&memory.clone(), "get", &[py_str("topic")]);
    let _ = py_setitem(&memory.clone(), &py_str("steps"), py_dict(vec![(py_str("answer"), answer.clone()), (py_str("steps"), steps.clone()), (py_str("at"), at.clone()), (py_str("turn"), turn.clone()), (py_str("at_turn"), turn.clone()), (py_str("said"), said.clone()), (py_str("done"), { let __b = call_builtin("bool", &[mcall(&before.clone(), "get", &[py_str("done")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { same.clone() } }), (py_str("topic"), if py_truthy(&{ let __b = same.clone(); if !py_truthy(&__b) { __b } else { mcall(&before.clone(), "get", &[py_str("topic")]) } }) { mcall(&before.clone(), "get", &[py_str("topic")]) } else { if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { stdlib_call("copy", "deepcopy", &[topic.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None } })]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.plan".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AMOUNTS" => Some(g_AMOUNTS()),
            "AMOUNTS_LEADS" => Some(g_AMOUNTS_LEADS()),
            "CARRY_ON" => Some(g_CARRY_ON()),
            "DONE" => Some(g_DONE()),
            "FINISHED" => Some(g_FINISHED()),
            "FIRST_LEADS" => Some(g_FIRST_LEADS()),
            "LEADS" => Some(g_LEADS()),
            "NOT_A_JOB" => Some(g_NOT_A_JOB()),
            "WHILE_IT_LASTS" => Some(g_WHILE_IT_LASTS()),
            "_COUNTS" => Some(g__COUNTS()),
            "_HAD_A_GO" => Some(g__HAD_A_GO()),
            "_NOT_THE_THING" => Some(g__NOT_THE_THING()),
            "_OPENS_ON_THE_INSTRUCTION" => Some(g__OPENS_ON_THE_INSTRUCTION()),
            "_a_cut_at_a_colon" => Some(py_func("_a_cut_at_a_colon", |args: &[Py]| _a_cut_at_a_colon(args))),
            "_last_user" => Some(py_func("_last_user", |args: &[Py]| _last_user(args))),
            "_module_of_last" => Some(py_func("_module_of_last", |args: &[Py]| _module_of_last(args))),
            "_our_own_line" => Some(py_func("_our_own_line", |args: &[Py]| _our_own_line(args))),
            "_plan_for" => Some(py_func("_plan_for", |args: &[Py]| _plan_for(args))),
            "_record" => Some(py_func("_record", |args: &[Py]| _record(args))),
            "_said_in" => Some(py_func("_said_in", |args: &[Py]| _said_in(args))),
            "_stems" => Some(py_func("_stems", |args: &[Py]| _stems(args))),
            "_the_amounts_first" => Some(py_func("_the_amounts_first", |args: &[Py]| _the_amounts_first(args))),
            "_the_same_job" => Some(py_func("_the_same_job", |args: &[Py]| _the_same_job(args))),
            "_the_same_topic" => Some(py_func("_the_same_topic", |args: &[Py]| _the_same_topic(args))),
            "_the_thing_named" => Some(py_func("_the_thing_named", |args: &[Py]| _the_thing_named(args))),
            "_tried" => Some(py_func("_tried", |args: &[Py]| _tried(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "carry_on" => Some(py_func("carry_on", |args: &[Py]| carry_on(args))),
            "steps_of" => Some(py_func("steps_of", |args: &[Py]| steps_of(args))),
            "the_answer_in_hand" => Some(py_func("the_answer_in_hand", |args: &[Py]| the_answer_in_hand(args))),
            "the_last_answer" => Some(py_func("the_last_answer", |args: &[Py]| the_last_answer(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static AN_AMOUNT_THEY_HAVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BACK_REFERENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PILE_TOO_BIG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PILE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HAVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_AMOUNT_THEY_HAVE() -> Py { ensure_init(); AN_AMOUNT_THEY_HAVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AMOUNT_THEY_HAVE(v: Py) { ensure_init(); if let Ok(mut w) = AN_AMOUNT_THEY_HAVE.write() { *w = v; } }
pub fn g_A_BACK_REFERENCE() -> Py { ensure_init(); A_BACK_REFERENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BACK_REFERENCE(v: Py) { ensure_init(); if let Ok(mut w) = A_BACK_REFERENCE.write() { *w = v; } }
pub fn g_A_DAY() -> Py { ensure_init(); A_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY.write() { *w = v; } }
pub fn g_A_PILE_TOO_BIG() -> Py { ensure_init(); A_PILE_TOO_BIG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PILE_TOO_BIG(v: Py) { ensure_init(); if let Ok(mut w) = A_PILE_TOO_BIG.write() { *w = v; } }
pub fn g_NOT_A_PILE() -> Py { ensure_init(); NOT_A_PILE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PILE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PILE.write() { *w = v; } }
pub fn g_SHORT() -> Py { ensure_init(); SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = SHORT.write() { *w = v; } }
pub fn g__A_FILLER() -> Py { ensure_init(); _A_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = _A_FILLER.write() { *w = v; } }
pub fn g__A_HAVE() -> Py { ensure_init(); _A_HAVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HAVE(v: Py) { ensure_init(); if let Ok(mut w) = _A_HAVE.write() { *w = v; } }
pub fn g__A_NUMBER() -> Py { ensure_init(); _A_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = _A_NUMBER.write() { *w = v; } }
fn module_body() {
    let _ = py_str("How much of a thing the person has, said as the whole line.\n\nA pile is not a day. \"ive got too many\" after the password answer, \"weve got plenty\", \"i have\ngot loads\" and \"ive got about thirty\" are all a person telling the bot the size of what they are\nholding, and nothing in any of them has happened. The account-of-the-day pool read the \"ive\" and\nthe \"got\" as a past doing and answered every one of them with a warm question about a day that\nnever took place:\n\n    ive got too many          -> \"Good to hear. What was the best bit?\"\n    weve got plenty           -> \"Nice one. How did the rest of it go?\"\n    ive got none left         -> \"How was it, overall?\"\n\nWhat a friend does with a pile is cut it down to a shortlist, because nobody gets through a pile\nin one go. That is the one thing that is true of a pile of passwords, a pile of emails, a pile of\nphotos and a pile of jobs in the loft, so it is what this module says - and then the person has\nnamed the thing, or said which of it matters, and the modules below have something to answer.\n\nThe reading is deliberately narrow, and it returns None for everything else:\n\n* the line is the person and their having, then an amount and no doing: \"i got too many\" is a\n  receipt of something that happened (\"i got lots of presents\") and is left alone, so only\n  \"i've got\", \"i have\" and their chat-speak spellings fire;\n* the amount is a surplus or a size, not a shortage of a thing the answer needs: \"i ran out of\n  oven cleaner\" and \"i dont have a plunger\" are `bot/memory/limit.py`'s, and they are answered\n  from the answer's own steps;\n* a day marker or \"done\" makes it a day again (\"i got a lot done today\"), and it is passed on;\n* the thing, when it is named, has to be a thing that comes in a pile. \"too much time on my\n  hands\" and \"too much on\" are the shape without the pile, and both have an answer of their own.\n");
    s__A_HAVE(py_str("(?:i|we)(?:'ve|ve|\\s+have)"));
    s__A_FILLER(py_str("(?:(?:\\s+|,\\s*)(?:only|just|still|already|now|got|have)){0,3}"));
    s__A_NUMBER(py_str("(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a hundred|a dozen|hundred|thousand)"));
    s_AN_AMOUNT_THEY_HAVE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^\\W*(?:(?:oh|well|but|and|so|yeah|yeh|yep|no|nah|ok|okay|god|blimey|honestly|honest)[,\\s]+)*(?:i|we)(?:'ve|ve|\\s+have)"), &g__A_FILLER()), &py_str("\\s+(?P<amount>too\\s+(?:many|much)|loads|plenty|lots|a\\s+lot|tons|hundreds|thousands|millions|dozens|(?:about|around|over|nearly|almost)\\s+")), &g__A_NUMBER()), &py_str(")(?:\\s+of\\s+(?:them|it|those|these|everything))?(?:\\s+(?P<thing>[a-z][a-z'-]*(?:\\s+[a-z][a-z'-]*){0,3}))?\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BACK_REFERENCE(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("them"), py_str("it"), py_str("those"), py_str("these"), py_str("everything"), py_str("of them"), py_str("of it"), py_str("of those"), py_str("of these"), py_str("of everything")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PILE(stdlib_call("re", "compile", &[py_str("\\b(?:time|on|to\\s+do|going\\s+on|happening|to\\s+say|on\\s+my\\s+plate|on\\s+at\\s+work)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|last night|yesterday|this morning|this afternoon|this evening|at the weekend|this weekend|last weekend|all day|all morning|the other day|done)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT(Py::Int(8));
    s_A_PILE_TOO_BIG(py_tuple(vec![py_str("Then don't take the whole pile on: the handful that actually matter first, and the rest can wait until they do."), py_str("Start with the ones you actually use and let the rest sit — a pile only gets smaller from the top. Which of it matters?"), py_str("That is more than anyone gets through in one go. Which of it would you miss if it went?"), py_str("Take the shortlist rather than the pile: what are the two or three that really count?")]));
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
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[])]), "lower", &[]);
    Py::None
}
pub fn reads_the_pile(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reading, for a caller that wants to know without a reply: True for a pile line.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()) } } } }) {
    return Py::Bool(false);
    }
    found = mcall(&g_AN_AMOUNT_THEY_HAVE(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_DAY(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    thing = _the_thing(&[found.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&g_A_BACK_REFERENCE(), &thing.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_A_PILE(), "search", &[thing.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
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
    else { Py::None };
    let _ = py_str("A reply to a pile, or None for every other line.\n\n    `memory` is the conversation's, and it is passed to `pick` so a pile said twice running is not\n    answered with the same sentence twice.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reads_the_pile(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[g_A_PILE_TOO_BIG(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.pile".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_AMOUNT_THEY_HAVE" => Some(g_AN_AMOUNT_THEY_HAVE()),
            "A_BACK_REFERENCE" => Some(g_A_BACK_REFERENCE()),
            "A_DAY" => Some(g_A_DAY()),
            "A_PILE_TOO_BIG" => Some(g_A_PILE_TOO_BIG()),
            "NOT_A_PILE" => Some(g_NOT_A_PILE()),
            "SHORT" => Some(g_SHORT()),
            "_A_FILLER" => Some(g__A_FILLER()),
            "_A_HAVE" => Some(g__A_HAVE()),
            "_A_NUMBER" => Some(g__A_NUMBER()),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "reads_the_pile" => Some(py_func("reads_the_pile", |args: &[Py]| reads_the_pile(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

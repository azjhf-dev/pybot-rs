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


pub static A_FIGURE_REPLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YES_OR_NO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INCLUDED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIVE_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OLD_ONE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEARCH_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_FIGURE_REPLY() -> Py { ensure_init(); A_FIGURE_REPLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_REPLY(v: Py) { ensure_init(); if let Ok(mut w) = A_FIGURE_REPLY.write() { *w = v; } }
pub fn g_A_YES_OR_NO_ASK() -> Py { ensure_init(); A_YES_OR_NO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YES_OR_NO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_YES_OR_NO_ASK.write() { *w = v; } }
pub fn g_INCLUDED_ASK() -> Py { ensure_init(); INCLUDED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INCLUDED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = INCLUDED_ASK.write() { *w = v; } }
pub fn g_LIVE_ONLY() -> Py { ensure_init(); LIVE_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIVE_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = LIVE_ONLY.write() { *w = v; } }
pub fn g_MONEY_WORDS() -> Py { ensure_init(); MONEY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_WORDS.write() { *w = v; } }
pub fn g_OLD_ONE_VERBS() -> Py { ensure_init(); OLD_ONE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OLD_ONE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = OLD_ONE_VERBS.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g_QUESTION_FILLER() -> Py { ensure_init(); QUESTION_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_FILLER.write() { *w = v; } }
pub fn g_SEARCH_ONLY() -> Py { ensure_init(); SEARCH_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEARCH_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = SEARCH_ONLY.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The questions whose answer is nowhere in this bot, said where the answer lives.\n\nWhat a pass, a warranty or a policy covers, what somebody else's rules allow, what is in a hotel\nroom, and what happens to the old unit when the new one arrives cannot be answered from here: the\nsmall print, the rules and the booking page are theirs, and a guess would be worse than none.\n`bot/compose/routes.py` reads those kinds and names the one place that holds the answer - the terms\npage, the insurer, the operator, the council, the booking page, Citizens Advice or ACAS.\n\nThis module sits just above greeting's honest hedge (\"a search should sort you out quickly\") and\nbelow every light remark and every card, so it only ever speaks where the bot would otherwise have\ntold the person to go and search. It asks the composer first and stands down unless the answer\nreally is a hedge, so a reader in `bot/compose/fallback.py` that has found the right body to ask\n(the landlord, the union, the school office) keeps its turn even though this module outranks\ngreeting. There are two exceptions, and they are one fault: a card whose subject is not the\nquestion. A card about the *rate* for parking, answering a line that only asked whether there is\nany, and a card about today's timetables, answering whether a venue has disabled access. In both\nthe person has not been told the thing is there yet, so the rate or the timetable is an answer to a\nquestion they did not ask, and the pool says both.\n\nNothing here holds a fact about the world, so nothing here can be wrong or go stale, and\n`routes.py` stands down on a statement, on a lid rather than a policy, on a child's own permission\nand on any line a card answers.\n");
    s_SEARCH_ONLY(stdlib_call("re", "compile", &[py_str("\\b(?:beats searching|a search (?:should|will)|quick search|searching for it will|quickest route to a straight answer|would rather not guess|beyond what i know|not sure about that one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_YES_OR_NO_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is there|are there|do they have|does (?:it|this|that|the [a-z]+) have|have they got)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_ONLY(stdlib_call("re", "compile", &[py_str("(timetables? and listings|change by the hour|that is live|live, sorry|today's included|the current one, today)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INCLUDED_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:includes?|included|in with|comes? with|part of the price|on top|extra)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_REPLY(stdlib_call("re", "compile", &[py_str("(?:comes from a quote or a listing|no list price|price the same description|ask two firms to price|compare what each one leaves out)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:rate|rates|charge|charges|price|prices|pricing|cost|costs|fee|fees|tariff|per hour|per day|per night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the is are was were be been being am do does did have has had having there here it this\nthat these those they them their theirs i me my mine we us our you your yours he him his she\nher hers who whom whose what which when where why how much many long far often any some all\nand or but if then than so not no yes can could will would should may might must shall to of\nin on at for with from by about into over under up down out off again more most very just\nonly also too own same other another such per each every both either neither nor as like\nget got gets getting go going goes went come comes came coming take takes took taken taking\none ones thing things still yet ever never always really quite rather bit lot\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OLD_ONE_VERBS(call_builtin("frozenset", &[mcall(&py_str("keep keeping kept take taking taken away rid getting get gets\nthrow throwing leave leaving want wants wanted happen happens happened dispose disposal"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRIORITY(neg(&Py::Int(999)));
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { py_tuple(vec![]) };
    let mut w: Py = Py::None;
    let _ = py_str("The words of a line that carry its subject: no filler, no figure, and no verb of the ask.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_QUESTION_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&skip.clone(), &w.clone())) } } } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _would_be_said(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The reply the person would get if this module said nothing, which is greeting's.\n\n    `fallback.reply_for` is one layer below that: greeting's own tables answer a good many of\n    these lines before any pool is reached, so asking the fallback instead of the greeting threw a\n    real answer away for a page. `is there a plug socket in the room` had the watts-and-13-amp\n    answer and got the booking page. The gate has to be the reply that would actually be said, so\n    it is asked of greeting itself, and the fallback only when greeting passes.\n    ");
    let __try1 = exc::catch(|| {
    let mut greeting: Py = crate::gen::modules::greeting::module_obj();
    said = mcall(&greeting.clone(), "respond", &[text.clone(), memory.clone()]);
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
    said = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = py_call(&crate::gen::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]);
    }
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn _answers_the_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut route: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "route") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: route"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("Whether the reply the person would get is an answer to the question in `clean`.\n\n    Three ways a reply is not: a price instead of a yes (\"That is a published rate...\"), today's\n    listings instead of what a building has, and - the worst - a card about another subject taking\n    the line on one word of it. On the world's main \"is there a lift at the station\" gets a lifting\n    technique, \"is there a freezer in the room\" the maker's sheet for a machine, \"can i keep the\n    old sofa\" freezer times and \"will they take the old fridge away\" what to wear for the weather.\n    A reply about a thing shares a word with the question about it; one that shares none is\n    answering something else, and the honest \"here is where the answer is\" is the better line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&route.clone(), &crate::gen::compose::routes::g_A_PLACE_HAS())); if !py_truthy(&__b) { __b } else { mcall(&g_A_YES_OR_NO_ASK(), "search", &[clean.clone()]) } }) {
    if py_truthy(&{ let __b = mcall(&g_MONEY_WORDS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_LIVE_ONLY(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&_content(&[said.clone()]), &_content(&[clean.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_is(&route.clone(), &crate::gen::compose::routes::g_THE_OLD_ONE_POOL()))) {
    return call_builtin("bool", &[bitand(&_content(&[said.clone()]), &_content(&[clean.clone(), g_OLD_ONE_VERBS()]))]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&route.clone(), &crate::gen::compose::routes::g_INCLUDED())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_INCLUDED_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE_REPLY(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut route: Py = Py::None;
    let mut said: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Name the place, or say nothing and let the composer answer as it did.");
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = crate::gen::compose::fallback::_matchable(&[crate::gen::compose::fallback::_plain(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }])]);
    if py_truthy(&crate::gen::compose::findout::reads_as_busy(&[clean.clone()])) {
    return Py::None;
    }
    route = crate::gen::compose::routes::options(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&route.clone()))) {
    return Py::None;
    }
    said = _would_be_said(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_is_a_hedge(&[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SEARCH_ONLY(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[route.clone(), memory.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&route.clone(), &crate::gen::compose::routes::g_INCLUDED())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_INCLUDED_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE_REPLY(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[route.clone(), memory.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_answers_the_ask(&[route.clone(), clean.clone(), { let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[route.clone(), memory.clone(), crate::gen::compose::fallback::_matchable(&[text.clone()])]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.where_it_answers".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_FIGURE_REPLY" => Some(g_A_FIGURE_REPLY()),
            "A_YES_OR_NO_ASK" => Some(g_A_YES_OR_NO_ASK()),
            "INCLUDED_ASK" => Some(g_INCLUDED_ASK()),
            "LIVE_ONLY" => Some(g_LIVE_ONLY()),
            "MONEY_WORDS" => Some(g_MONEY_WORDS()),
            "OLD_ONE_VERBS" => Some(g_OLD_ONE_VERBS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "QUESTION_FILLER" => Some(g_QUESTION_FILLER()),
            "SEARCH_ONLY" => Some(g_SEARCH_ONLY()),
            "_answers_the_ask" => Some(py_func("_answers_the_ask", |args: &[Py]| _answers_the_ask(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_would_be_said" => Some(py_func("_would_be_said", |args: &[Py]| _would_be_said(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

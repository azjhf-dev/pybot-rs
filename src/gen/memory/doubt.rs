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


pub static DOUBTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOUBTS_THE_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOUBT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOST_WORDS_IN_A_DOUBT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_THE_ANSWER_IS_WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORTEST_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNCLEAR_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MEANT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_DOUBTING() -> Py { ensure_init(); DOUBTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBTING(v: Py) { ensure_init(); if let Ok(mut w) = DOUBTING.write() { *w = v; } }
pub fn g_DOUBTS_THE_ANSWER() -> Py { ensure_init(); DOUBTS_THE_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBTS_THE_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = DOUBTS_THE_ANSWER.write() { *w = v; } }
pub fn g_DOUBT_TAIL() -> Py { ensure_init(); DOUBT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = DOUBT_TAIL.write() { *w = v; } }
pub fn g_MOST_WORDS_IN_A_DOUBT() -> Py { ensure_init(); MOST_WORDS_IN_A_DOUBT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS_IN_A_DOUBT(v: Py) { ensure_init(); if let Ok(mut w) = MOST_WORDS_IN_A_DOUBT.write() { *w = v; } }
pub fn g_SAYS_THE_ANSWER_IS_WRONG() -> Py { ensure_init(); SAYS_THE_ANSWER_IS_WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_THE_ANSWER_IS_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_THE_ANSWER_IS_WRONG.write() { *w = v; } }
pub fn g_SHORTEST_ANSWER() -> Py { ensure_init(); SHORTEST_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORTEST_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = SHORTEST_ANSWER.write() { *w = v; } }
pub fn g_UNCLEAR_ANSWER() -> Py { ensure_init(); UNCLEAR_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNCLEAR_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = UNCLEAR_ANSWER.write() { *w = v; } }
pub fn g__MEANT_LEADS() -> Py { ensure_init(); _MEANT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MEANT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = _MEANT_LEADS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A line that takes issue with the answer just given is answered about that answer.\n\nA person who says the bot's own last sentence makes no sense is not shrugging at the weather and\nis not upset about something of their own: they are asking for it again, or they think it is\nwrong. Nothing in the line names a thing of its own, so no knowledge entry can answer it, and\nevery fallback pool on main read it as a shrug or as bad news about the person:\n\n    that makes no sense        -> \"Fair enough.\"\n    that doesnt make sense     -> \"I'm listening - say more if it helps.\"\n    youre not making sense     -> \"Thanks for telling me - I'm here if you want to dig into any\n                                  of that.\"\n    that cant be right         -> \"Got it. Is there anything you'd like a hand with?\"\n    youre confusing me         -> \"Got it. Is there anything you'd like a hand with?\"\n\nEach one says the bot heard nothing, when the person has just said the bot's paragraph did not\nland. The reading here is deliberately narrow, because the same words are ordinary inside a\nsentence about something else: \"it doesnt make sense to buy new\" is a line about buying new and\n\"that makes no sense, the council said they would ring\" is a line about the council. Only a doubt\nthat is *what the line says*, bar a little chatter after it, is read here.\n\nTwo things are told apart, because they want different answers. A line that says the answer is\n*wrong* (\"that cant be right\", \"are you sure\") is conceding nothing of the person's: the bot says\nwhere it got it, since the person has already pointed at what they doubt. A line that says it is\n*unclear* (\"i dont follow\", \"youre confusing me\") is the bot's paragraph failing, so the bot owns\nthat (\"That is on me\") rather than sending the person away to say it better - the fault is the\nsentence the bot chose, not the person's words.\n\nWhere the bot has a real answer it can hand back plainer, it does that instead of asking which\nbit: \"i dont follow\" after the egg card wants the first move again in fewer words, and\n`bot/memory/meant.py` already knows how to do that. Its \"nothing of mine to go back over yet\" is\nnot a hand-back, so that is left to the pool.\n");
    s_DOUBTS_THE_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:that|this|it|you|youre|you're)\\s+(?:makes?|made|making|doesnt|doesn't|isnt|isn't|are|arent|aren't)\\s+(?:no|not|any)\\s+(?:sense|sensible)\\b|\\b(?:not|never) making (?:any )?sense\\b|\\b(?:doesnt|doesn't|didnt|didn't|wouldnt|wouldn't|wont|won't)\\s+(?:make|makes|made)\\s+(?:any\\s+)?sense\\b|\\bare you (?:sure|certain)\\b|\\byou sure\\b|\\b(?:that|this|it) (?:cant|can't|cannot|doesnt|doesn't|wont|won't) be right\\b|\\b(?:thats|that's|this is) (?:nonsense|rubbish|wrong|not right|a load of)\\b|\\b(?:youre|you're|you are|u r) (?:confusing|confused) me\\b|\\bi (?:dont|don't) follow\\b|\\bi (?:dont|don't) get (?:that|this|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_THE_ANSWER_IS_WRONG(stdlib_call("re", "compile", &[py_str("\\b(?:cant|can't|cannot|doesnt|doesn't|wont|won't) be right\\b|\\b(?:nonsense|rubbish|not right|wrong|a load of)\\b|\\bare you (?:sure|certain)\\b|\\byou sure\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_TAIL(call_builtin("frozenset", &[mcall(&py_str("at all though tho tbh really sorry mate love god no way innit like\n    ok okay then haha lol"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNCLEAR_ANSWER(py_tuple(vec![py_str("That is on me. Which bit was the muddled one?"), py_str("Fair enough. Say which part and I will put it another way."), py_str("Let me try that again, plainer. Which bit lost you?")]));
    s_DOUBTING(py_tuple(vec![py_str("Fair enough to check it. Which bit looks wrong, and I'll say where I got it."), py_str("Then I may have it wrong. Say what you were expecting and we'll compare the two."), py_str("It could be out of date, that is a real risk with these things. Point at the part and I will go through it.")]));
    s__MEANT_LEADS(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&crate::gen::memory::meant::g_REPEAT_LEADS(), &crate::gen::memory::meant::g_LEADS())) { let mut lead: Py = __item.clone(); __out.push(mcall(&mcall(&geti(&mcall(&lead.clone(), "split", &[py_str("{"), Py::Int(1)]), &Py::Int(0)), "strip", &[]), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORTEST_ANSWER(Py::Int(40));
    s_MOST_WORDS_IN_A_DOUBT(Py::Int(9));
}
pub fn _the_answer_just_given(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own last line, or \"\" when there is none in this conversation yet.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    return if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    Py::None
}
pub fn the_doubt(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line doubts the bot's own last answer: \"wrong\", \"unclear\", or None.\n\n    Only when the bot has said something to doubt: a first line of \"that makes no sense\" is a\n    person with nothing in front of them, and the pools for that are elsewhere.\n    ");
    said = _the_answer_just_given(&[memory.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORTEST_ANSWER())) {
    return Py::None;
    }
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS_IN_A_DOUBT()) } }) {
    return Py::None;
    }
    hit = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_DOUBTS_THE_ANSWER(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    tail = mcall(&slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,!?;:-")]);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&tail.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOUBT_TAIL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    hit = found.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&hit.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SAYS_THE_ANSWER_IS_WRONG(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bsense\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("wrong");
    }
    return py_str("unclear");
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
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut plainer: Py = Py::None;
    let _ = py_str("The reply to a line that doubts the bot's own answer, or None for every other line.");
    kind = the_doubt(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return Py::None;
    }
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("wrong")))) {
    return crate::gen::compose::pick(&[g_DOUBTING(), memory.clone(), line.clone()]);
    }
    plainer = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { crate::gen::memory::meant::answer_for(&[line.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = plainer.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&plainer.clone(), "lower", &[]), "startswith", &[g__MEANT_LEADS()]) } }) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[g_UNCLEAR_ANSWER(), memory.clone(), line.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.doubt".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DOUBTING" => Some(g_DOUBTING()),
            "DOUBTS_THE_ANSWER" => Some(g_DOUBTS_THE_ANSWER()),
            "DOUBT_TAIL" => Some(g_DOUBT_TAIL()),
            "MOST_WORDS_IN_A_DOUBT" => Some(g_MOST_WORDS_IN_A_DOUBT()),
            "SAYS_THE_ANSWER_IS_WRONG" => Some(g_SAYS_THE_ANSWER_IS_WRONG()),
            "SHORTEST_ANSWER" => Some(g_SHORTEST_ANSWER()),
            "UNCLEAR_ANSWER" => Some(g_UNCLEAR_ANSWER()),
            "_MEANT_LEADS" => Some(g__MEANT_LEADS()),
            "_the_answer_just_given" => Some(py_func("_the_answer_just_given", |args: &[Py]| _the_answer_just_given(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_doubt" => Some(py_func("the_doubt", |args: &[Py]| the_doubt(args))),
            _ => None,
        }),
        set: None,
    }))
}

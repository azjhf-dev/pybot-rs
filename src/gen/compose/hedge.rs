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


pub static MISREAD_AS_GOOD_NEWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OUT_OF_THEIR_HANDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_MISREAD_AS_GOOD_NEWS() -> Py { ensure_init(); MISREAD_AS_GOOD_NEWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISREAD_AS_GOOD_NEWS(v: Py) { ensure_init(); if let Ok(mut w) = MISREAD_AS_GOOD_NEWS.write() { *w = v; } }
pub fn g_NOTHING_SAID() -> Py { ensure_init(); NOTHING_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_SAID.write() { *w = v; } }
pub fn g_OUT_OF_THEIR_HANDS() -> Py { ensure_init(); OUT_OF_THEIR_HANDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OUT_OF_THEIR_HANDS(v: Py) { ensure_init(); if let Ok(mut w) = OUT_OF_THEIR_HANDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The composer's reply to a line, asked before a module answers over it.\n\nA module that sits just above the composer floor (the greeting pool, -1000) takes every turn\nnobody else claims. Most of those turns deserve taking: the echo of the person's own words, the\nnod, the search hedge. Some do not, because the composer's floor is not only those three - it\nanswers a dog with what a dog needs, a school label with the SENCO card, a form nobody can read\nwith the three things to look for. A module that answered over those would be trading a real\nanswer for its own.\n\nSo a module asks the composer first, in the same way `bot/memory/cause.py` does\n(`_the_bot_would_only_shrug`): copy the memory, put the random state back afterwards, and read\nwhat `fallback.reply_for` says for the line on its own (`verdict=False`, the words rather than\nthis turn).\n");
    s_NOTHING_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m listening|say more|not sure about that one|don'?t know enough|beyond what i know|rather not guess|everything i have on that one|not much (?:i can add|to go on)|which bit do you|tell me more|i'?m here if you want|anything you'?d like a hand with|thanks for telling me)\\b|\\u2014\\s*noted\\.|\\s-\\s*noted\\.|^noted[:.]|,\\s*good to know\\.?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OUT_OF_THEIR_HANDS(stdlib_call("re", "compile", &[py_str("out of your hands|when do you find out"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISREAD_AS_GOOD_NEWS(stdlib_call("re", "compile", &[py_str("\\b(?:good to hear|glad to hear|best bit|nice one|sounds alright|sounds good|lovely|how lovely|brilliant)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn what_the_composer_would_say(args: &[Py]) -> Py {
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
    let mut probe: Py = Py::None;
    let mut state: Py = Py::None;
    let _ = py_str("The composer's own reply to this line, or \"\" when it has none.");
    probe = stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    state = stdlib_call("random", "getstate", &[]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(probe.clone()); __kws.extend(vec![("verdict".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::gen::compose::fallback::g_reply_for(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    let _ = stdlib_call("random", "setstate", &[state.clone()]).unwrap_or_else(|e| exc::raise(e));
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn only_a_shrug(args: &[Py]) -> Py {
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
    let mut misreads: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "misreads") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let _ = py_str("True when the composer has no answer of its own for this line.\n\n    `misreads` is a pattern for the answers that are worse than nothing on this kind of line (a\n    congratulation where the person said something would not go in). A reply matching it counts\n    as the composer having nothing, because the line cannot mean what it read.\n    ");
    said = what_the_composer_would_say(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_NOTHING_SAID(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OUT_OF_THEIR_HANDS(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&misreads.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&misreads.clone(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.hedge".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "MISREAD_AS_GOOD_NEWS" => Some(g_MISREAD_AS_GOOD_NEWS()),
            "NOTHING_SAID" => Some(g_NOTHING_SAID()),
            "OUT_OF_THEIR_HANDS" => Some(g_OUT_OF_THEIR_HANDS()),
            "only_a_shrug" => Some(py_func("only_a_shrug", |args: &[Py]| only_a_shrug(args))),
            "what_the_composer_would_say" => Some(py_func("what_the_composer_would_say", |args: &[Py]| what_the_composer_would_say(args))),
            _ => None,
        }),
        set: None,
    }))
}

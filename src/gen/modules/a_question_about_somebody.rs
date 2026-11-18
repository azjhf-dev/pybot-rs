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


pub static AN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FACT_ABOUT_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THIRD_PARTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEBODY_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_IS_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static pick: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_ASK() -> Py { ensure_init(); AN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = AN_ASK.write() { *w = v; } }
pub fn g_A_FACT_ABOUT_THEM() -> Py { ensure_init(); A_FACT_ABOUT_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FACT_ABOUT_THEM(v: Py) { ensure_init(); if let Ok(mut w) = A_FACT_ABOUT_THEM.write() { *w = v; } }
pub fn g_A_THIRD_PARTY() -> Py { ensure_init(); A_THIRD_PARTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THIRD_PARTY(v: Py) { ensure_init(); if let Ok(mut w) = A_THIRD_PARTY.write() { *w = v; } }
pub fn g_NAMES_THEM() -> Py { ensure_init(); NAMES_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_THEM(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_THEM.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g_SOMEBODY_IN_THE_LINE() -> Py { ensure_init(); SOMEBODY_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = SOMEBODY_IN_THE_LINE.write() { *w = v; } }
pub fn g_WHO_IS_IT() -> Py { ensure_init(); WHO_IS_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_IS_IT(v: Py) { ensure_init(); if let Ok(mut w) = WHO_IS_IT.write() { *w = v; } }
pub fn g_pick() -> Py { ensure_init(); pick.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); if let Ok(mut w) = pick.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A question about somebody the chat has never named asks who they mean.\n\nThe first line of a chat has nobody in it. Every way of asking a fact about a person was answered\nas if the chat held one, and each in its own wrong way:\n\n    how old are they       -> You haven't told me how old they are.\n    how old is he          -> I'm not sure about that one, sorry - I'd rather not guess. ...\n    what do they do        -> I don't know enough to answer that honestly. ...\n    where does she work    -> You would have to ask her - that is not a thing I can look up.\n    whats their name       -> I don't know it yet.\n\nNone of those is an answer to the line. A denial about something the person \"hasn't told\" us is a\nclaim about a conversation that has not happened; the hedge is the bot shrugging at a question it\nunderstood perfectly; \"you would have to ask her\" hands the answer to a her the chat has never been\ntold about. What is missing is the person, so the reply asks for them, in the words the receipt\nreader already uses when a figure arrives with nobody attached (\"hes 9\" -> \"Who's 9?\").\n\nThe rule is one question, asked once: is there a chat behind this line? `bot/memory.there_is_a_chat`\nis the chat's own answer to that, the same one every reader that leans on the past asks. With a\nchat behind it the pronoun points at somebody real and this module says nothing - \"my son is 14\"\nthen \"how old is he\" is still answered \"Your son is 14.\" by the store.\n\nWith nobody behind it, the line still has to be asking about a person: a third party named only by\na pronoun, in a line that asks something, with nobody in the line to be about. \"how do you know if\nthey like you\" is about the pair of them and the person asking, \"what should i do if they dont text\nback\" is about people in general, and \"how old is the dog\" names the dog - all three keep whatever\nthe bot would have said, because asking \"who do you mean?\" there is asking the person to repeat\nthemselves.\n");
    let __try1 = exc::catch(|| {
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
    s_pick(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_WHO_IS_IT(py_tuple(vec![py_str("Who do you mean?"), py_str("Who's that, then?"), py_str("Who are we talking about?")]));
    s_PRIORITY(Py::Int(7300));
    s_A_THIRD_PARTY(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|him|her|his|hers|they|them|their|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEBODY_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im|me|my|mine|myself|we|us|our|ours|you|your|yours|yourself|yourselves|someone|somebody|anyone|anybody|everyone|everybody|people|nobody|no one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|this|that|these|those)\\s+[a-z][a-z'-]{1,}"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FACT_ABOUT_THEM(stdlib_call("re", "compile", &[py_str("\\bhow (?:old|tall|big|heavy|far|long)\\b|\\b(?:what|whats|what's)\\b[^?]*\\b(?:name|age|job|birthday|number|address)\\b|\\bwhere (?:does|do|did|is|was)\\b[^?]*\\b(?:live|work|come from|from|stay|based)\\b|\\b(?:is|are|was|were)\\s+(?:he|she|they)\\b|\\bhow (?:is|are|was|were)\\s+(?:he|she|they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:how|what|whats|what's|where|when|which|why|is|are|was|were|does|do|did|has|have|can|could|would|will)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _there_is_a_chat(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the person said anything before this line.\n\n    The rule the reply leans on is the chat's: a reply about a past line needs a past line, and on\n    a first message there is none. Every reader that leans on the chat asks this one question, so\n    it is asked once, here, off the key the runtime writes.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _flat(args: &[Py]) -> Py {
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
    let _ = py_str("The line with its case, spacing and end punctuation taken off.");
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn about_somebody_unnamed(args: &[Py]) -> Py {
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
    let _ = py_str("Is this an ask about a third party the chat has never been told about?");
    if py_truthy(&_there_is_a_chat(&[memory.clone()])) {
    return Py::Bool(false);
    }
    said = _flat(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_AN_ASK(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_THIRD_PARTY(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_FACT_ABOUT_THEM(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&mcall(&g_SOMEBODY_IN_THE_LINE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NAMES_THEM(), "search", &[said.clone()]))) } };
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
    let _ = py_str("Asking who they mean, or None for every other line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&about_somebody_unnamed(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&g_pick(), &Py::None))) {
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_call(&g_pick(), &[g_WHO_IS_IT(), memory.clone(), _flat(&[text.clone()])])]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    return geti(&g_WHO_IS_IT(), &Py::Int(0));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_question_about_somebody".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ASK" => Some(g_AN_ASK()),
            "A_FACT_ABOUT_THEM" => Some(g_A_FACT_ABOUT_THEM()),
            "A_THIRD_PARTY" => Some(g_A_THIRD_PARTY()),
            "NAMES_THEM" => Some(g_NAMES_THEM()),
            "PRIORITY" => Some(g_PRIORITY()),
            "SOMEBODY_IN_THE_LINE" => Some(g_SOMEBODY_IN_THE_LINE()),
            "WHO_IS_IT" => Some(g_WHO_IS_IT()),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_there_is_a_chat" => Some(py_func("_there_is_a_chat", |args: &[Py]| _there_is_a_chat(args))),
            "about_somebody_unnamed" => Some(py_func("about_somebody_unnamed", |args: &[Py]| about_somebody_unnamed(args))),
            "pick" => Some(g_pick()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

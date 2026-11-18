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


pub static ANSWERS_INSTEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_GENERIC_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LIFE_AT_RISK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); ANSWERS_INSTEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS_INSTEAD.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g__A_GENERIC_OPENER() -> Py { ensure_init(); _A_GENERIC_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_GENERIC_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = _A_GENERIC_OPENER.write() { *w = v; } }
pub fn g__A_LIFE_AT_RISK() -> Py { ensure_init(); _A_LIFE_AT_RISK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LIFE_AT_RISK(v: Py) { ensure_init(); if let Ok(mut w) = _A_LIFE_AT_RISK.write() { *w = v; } }
pub fn g__ROUTES() -> Py { ensure_init(); _ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = _ROUTES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Somebody asks this bot for its ear and names the thing: an answer, not a shrug.\n\n\"can you talk to me about my anxiety\" was answered with the search hedge (\"I don't know enough to\nanswer that honestly. A search should sort you out quickly\") and \"can you help me think about my\noptions\" with \"I'm not sure about that one, sorry - I'd rather not guess\": two ways of telling a\nperson to go and look it up, when what they asked for was to talk it through. The bot cannot know\ntheir anxiety, but it can do the one thing asked for, so the reply is a yes in a few words and the\nquestion that starts it (bot/compose/talkto.py, STYLE.md \"Not knowing, honestly\" and its one short\nquestion).\n\nNothing that answers the line is displaced, and that is declared rather than hoped for (the routing\ncontract, tests/test_routing_contract.py). `ANSWERS_INSTEAD` says the line read whole is small talk\n- somebody asking for the bot's ear - so the small-talk entries do not answer it on a word of their\nkey. `DEFERS_TO` says the rest: every entry that answers the thing named keeps the turn, because a\ncard knows more than an opener does - \"can we talk about my mum\" is the carers card, \"can you help\nme think about my options for childcare\" the cost of nursery. And the composer's own route to a\nperson is a better answer than an opener too: \"can you help me think about my tax\" already names\nCitizens Advice, and that survives (see `_the_composer_says_a_route`).\n");
    s_PRIORITY(neg(&Py::Int(500)));
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("small-talk")]));
    s__A_GENERIC_OPENER(call_builtin("frozenset", &[py_tuple(vec![py_str("conversation-openers")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ROUTES(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback_data::g_WHO_TO_ASK()) { let __unpk1736 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk1736[0].clone(); let mut who: Py = __unpk1736[1].clone(); __out.push(who.clone()); } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LIFE_AT_RISK(stdlib_call("re", "compile", &[py_str("suicid|kill(?:ing)?\\s+myself|self[\\s-]?harm|hurt(?:ing)?\\s+myself|end(?:ing)?\\s+my\\s+life|end(?:ing)?\\s+it\\s+all|take\\s+my\\s+own\\s+life|want(?:ing|ed)?\\s+to\\s+die|(?:not\\s+want(?:ing)?|do(?:n'?t|\\s+not)\\s+want(?:ing)?)\\s+to\\s+(?:be\\s+here|live|go\\s+on|carry\\s+on)|better\\s+off\\s+dead|no\\s+reason\\s+to\\s+live"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_entry_for(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The entry the line's own words reach, or None when nothing fits.\n\n    Asked about the line `bot/compose/talkto.py` read, with the named day taken out, because the\n    lookup has to see the thing named and nothing else: \"can we talk tomorrow about my mum\" reaches\n    the carers card only once \"tomorrow\" is out of the way, and with it in the lookup found the\n    small-talk opener instead and this module took a turn the card answers better.\n    ");
    let __try1 = exc::catch(|| {
    text = crate::gen::compose::talkto::the_line_as_read(&[text.clone()]);
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
    text = text.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    words = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), crate::gen::knowledge::lookup::find(&[py_dict(vec![(py_str("clean"), words.clone()), (py_str("words"), mcall(&words.clone(), "split", &[]))])])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn DEFERS_TO(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut found: Py = Py::None;
    let _ = py_str("Yes for every entry that answers the line: the opener row is the one exception.\n\n    A card about the thing named is a better answer than an opener about it, so it keeps the turn.\n    The small-talk opener is not: it matched the word \"about\", and its advice is for somebody\n    standing in a queue next to a stranger. That row alone does not keep the turn, or the yes never\n    reaches the person who asked for it.\n    ");
    found = _the_entry_for(&[text.clone()]);
    return { let __b = Py::Bool(py_is(&found.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__A_GENERIC_OPENER(), &gattr(&found.clone(), "id"))) } };
    Py::None
}
pub fn _the_entry_the_line_reached(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("The entry the understanding engine's reading reaches, or None.");
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), crate::gen::knowledge::lookup::find(&[understood.clone()])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _is_the_opener(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let _ = py_str("True when the only thing the line reached is the small-talk opener row.");
    return { let __b = Py::Bool(!py_is(&entry.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_GENERIC_OPENER(), &gattr(&entry.clone(), "id"))) } };
    Py::None
}
pub fn _a_route_is_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    said = mcall(&{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ROUTES()) { let mut who: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[who.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_composer_says_a_route(args: &[Py]) -> Py {
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
    let _ = py_str("True when the composer's own reply to this line names somebody to ask.");
    let mut greeting: Py = crate::gen::modules::greeting::module_obj();
    let __try1 = exc::catch(|| {
    said = mcall(&greeting.clone(), "respond", &[text.clone(), call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return _a_route_is_named(&[said.clone()]);
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
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The yes and the one question, or None for every other line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    memory = py_dict(vec![]);
    }
    if py_truthy(&crate::gen::compose::urgent::reply_for(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_LIFE_AT_RISK(), "search", &[text.clone()])) {
    return Py::None;
    }
    entry = _the_entry_the_line_reached(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&entry.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_the_opener(&[entry.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&entry.clone(), &Py::None))) {
    found = _the_entry_for(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&found.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_the_opener(&[found.clone()]))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_is_the_opener(&[entry.clone()]))); if !py_truthy(&__b) { __b } else { _the_composer_says_a_route(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    clean = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { py_str("") };
    return { let __b = crate::gen::compose::talkto::answer_for(&[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { text.clone() } }, memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.talkto".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "DEFERS_TO" => Some(py_func("DEFERS_TO", |args: &[Py]| DEFERS_TO(args))),
            "PRIORITY" => Some(g_PRIORITY()),
            "_A_GENERIC_OPENER" => Some(g__A_GENERIC_OPENER()),
            "_A_LIFE_AT_RISK" => Some(g__A_LIFE_AT_RISK()),
            "_ROUTES" => Some(g__ROUTES()),
            "_a_route_is_named" => Some(py_func("_a_route_is_named", |args: &[Py]| _a_route_is_named(args))),
            "_is_the_opener" => Some(py_func("_is_the_opener", |args: &[Py]| _is_the_opener(args))),
            "_the_composer_says_a_route" => Some(py_func("_the_composer_says_a_route", |args: &[Py]| _the_composer_says_a_route(args))),
            "_the_entry_for" => Some(py_func("_the_entry_for", |args: &[Py]| _the_entry_for(args))),
            "_the_entry_the_line_reached" => Some(py_func("_the_entry_the_line_reached", |args: &[Py]| _the_entry_the_line_reached(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_AWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GO_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANDOFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NAMES_A_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_AT_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT() -> Py { ensure_init(); ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT.write() { *w = v; } }
pub fn g_ABOUT_THE_THING() -> Py { ensure_init(); ABOUT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_THING.write() { *w = v; } }
pub fn g_ASK_AWAY() -> Py { ensure_init(); ASK_AWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_AWAY(v: Py) { ensure_init(); if let Ok(mut w) = ASK_AWAY.write() { *w = v; } }
pub fn g_BACK() -> Py { ensure_init(); BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK(v: Py) { ensure_init(); if let Ok(mut w) = BACK.write() { *w = v; } }
pub fn g_GO_ON() -> Py { ensure_init(); GO_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GO_ON(v: Py) { ensure_init(); if let Ok(mut w) = GO_ON.write() { *w = v; } }
pub fn g_HANDOFF() -> Py { ensure_init(); HANDOFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANDOFF(v: Py) { ensure_init(); if let Ok(mut w) = HANDOFF.write() { *w = v; } }
pub fn g__CORE() -> Py { ensure_init(); _CORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CORE(v: Py) { ensure_init(); if let Ok(mut w) = _CORE.write() { *w = v; } }
pub fn g__FRONT() -> Py { ensure_init(); _FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FRONT(v: Py) { ensure_init(); if let Ok(mut w) = _FRONT.write() { *w = v; } }
pub fn g__NAMES_A_QUESTION() -> Py { ensure_init(); _NAMES_A_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NAMES_A_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = _NAMES_A_QUESTION.write() { *w = v; } }
pub fn g__NOT_A_SUBJECT() -> Py { ensure_init(); _NOT_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_SUBJECT.write() { *w = v; } }
pub fn g__POINTS_AT_NOTHING() -> Py { ensure_init(); _POINTS_AT_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_AT_NOTHING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A line that says something is coming and has not said it yet: \"one more thing\", \"quick one\".\n\nThese lines carry no subject of their own. Nothing has been asked, so there is nothing to answer,\nand nothing to search for either - main answered \"one more thing\" with a knowledge entry about\nstudying (\"Test yourself instead of rereading\") because \"one\" and \"thing\" are words in the base,\nand \"another thing\" with \"Another thing — I'll remember that.\", which files away a line that has\nsaid nothing at all. A friend says go on. Anything more is invented: the person has not said what\nthe thing is.\n\nSo the whole line has to be the hand-off and nothing else. \"quick one, how do i descale a\nkettle\" names the thing after the comma and keeps whatever answers it, and a hand-off with a\nsubject (\"one more thing about the boiler\") has something in it to go on with.\n\nRead by the arbiter as well as by the composer: the line has no words to match, so the module\nthat answered last would otherwise keep the turn and reply to its own last answer. The composer\nsays it read this line, and the arbiter asks that before the never-repeat rule (README.md,\n\"Interface\").\n");
    s__FRONT(py_str("(?:(?:and|oh|ooh|ok|okay|so|right|ah|er|um|umm|hmm|sorry|also|just|plus|hey|ta|anyway|anyways|first|then|actually|also)\\b[\\s,]+)*"));
    s_BACK(py_str("(?:\\s+(?:actually|too|as well|then|now|first|ok|okay|sorry|thanks|tho|though|i think|i suppose|i reckon))?"));
    s__CORE(py_str("(?:one|two|three|a couple|a few|couple)(?: more| other| final| last) (?:things?|questions?|qs?|asks?|quer(?:y|ies)|bits?|points?|ones?)|another (?:things?|questions?|qs?|asks?|ones?|points?|quer(?:y|ies))|(?:one |the |a )?(?:last|final|next|other|second) (?:things?|questions?|qs?|asks?|ones?|bits?)|a? ?quick (?:ones?|questions?|qs?|asks?|things?|quer(?:y|ies))|(?:a |one |an )?(?:different|separate|unrelated|side) (?:questions?|ones?|asks?|things?|quer(?:y|ies))|one thing more|before i forget|while (?:im|i am) (?:here|at it)"));
    s_HANDOFF(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("^"), &g__FRONT()), &py_str("(?:")), &g__CORE()), &py_str(")")), &g_BACK()), &py_str("[\\s.?!,]*$")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("^"), &g__FRONT()), &py_str("(?:")), &g__CORE()), &py_str(")")), &g_BACK()), &py_str("[\\s,:-]*(?:about|on|to do with|regarding|around)\\s+(?P<thing>(?:(?:the|my|our|your|his|her|their|its|a|an|this|that)\\s+)?[\\w'-]+(?:\\s+[\\w'-]+){0,3})[\\s.?!,]*$")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_AT_NOTHING(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("they"), py_str("us"), py_str("you"), py_str("me"), py_str("him"), py_str("her"), py_str("there"), py_str("things"), py_str("thing"), py_str("stuff"), py_str("everything"), py_str("nothing"), py_str("something"), py_str("anything"), py_str("it all"), py_str("the lot")])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("whether"), py_str("if"), py_str("how"), py_str("what"), py_str("when"), py_str("why"), py_str("where"), py_str("who"), py_str("whom"), py_str("which"), py_str("that"), py_str("because"), py_str("so"), py_str("but"), py_str("and")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_THING(py_tuple(vec![py_str("Go on — what about {thing}?"), py_str("Ask away about {thing}."), py_str("{Thing} — right. What do you want to know?"), py_str("What about {thing}? I'm listening.")]));
    s__NAMES_A_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:question|questions|q|qs|ask|asks|query|queries)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_AWAY(py_tuple(vec![py_str("Ask away."), py_str("Sure — what is it you want to know?"), py_str("Go on, I'm listening."), py_str("Yes, ask away — I'm here.")]));
    s_GO_ON(py_tuple(vec![py_str("Go on — what is it?"), py_str("Sure, go ahead."), py_str("I'm listening — tell me."), py_str("Yes, what else?"), py_str("Go ahead, I'm here.")]));
}
pub fn reads_as_handoff_about_a_thing(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The subject a hand-off names (\"one more thing about the boiler\"), or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_ABOUT(), "match", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" ,.?!:-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__POINTS_AT_NOTHING(), &geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__POINTS_AT_NOTHING(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_A_SUBJECT(), &geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)))) } }) {
    return Py::None;
    }
    return thing.clone();
    Py::None
}
pub fn reads_as_handoff(args: &[Py]) -> Py {
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
    let _ = py_str("Is the whole line a hand-off with nothing said after it?");
    return call_builtin("bool", &[mcall(&g_HANDOFF(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("Read by the arbiter: a hand-off has no words to match, so this module answers it.\n\n    It says yes to every bare hand-off and no to every line that goes on to say something, and it\n    says yes to a hand-off that names its subject, which the composer answers by taking it.\n    ");
    return { let __b = reads_as_handoff(&[text.clone()]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[reads_as_handoff_about_a_thing(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut clean: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reply to a hand-off, or None when the line says something after it.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    thing = reads_as_handoff_about_a_thing(&[clean.clone()]);
    if py_truthy(&thing.clone()) {
    lines = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ABOUT_THE_THING()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return crate::gen::compose::pick(&[lines.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HANDOFF(), "match", &[clean.clone()])))) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[if py_truthy(&mcall(&g__NAMES_A_QUESTION(), "search", &[clean.clone()])) { g_ASK_AWAY() } else { g_GO_ON() }, memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.handoff".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT" => Some(g_ABOUT()),
            "ABOUT_THE_THING" => Some(g_ABOUT_THE_THING()),
            "ASK_AWAY" => Some(g_ASK_AWAY()),
            "BACK" => Some(g_BACK()),
            "GO_ON" => Some(g_GO_ON()),
            "HANDOFF" => Some(g_HANDOFF()),
            "_CORE" => Some(g__CORE()),
            "_FRONT" => Some(g__FRONT()),
            "_NAMES_A_QUESTION" => Some(g__NAMES_A_QUESTION()),
            "_NOT_A_SUBJECT" => Some(g__NOT_A_SUBJECT()),
            "_POINTS_AT_NOTHING" => Some(g__POINTS_AT_NOTHING()),
            "reads_as_handoff" => Some(py_func("reads_as_handoff", |args: &[Py]| reads_as_handoff(args))),
            "reads_as_handoff_about_a_thing" => Some(py_func("reads_as_handoff_about_a_thing", |args: &[Py]| reads_as_handoff_about_a_thing(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static ABOUT_THEMSELVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLACE_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEALTH_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVE_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_AND_MONTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_NOT_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_FIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_MISSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_TAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_WITH_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORK_TOLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THEMSELVES() -> Py { ensure_init(); ABOUT_THEMSELVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THEMSELVES(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THEMSELVES.write() { *w = v; } }
pub fn g_A_PLACE_NAMED() -> Py { ensure_init(); A_PLACE_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = A_PLACE_NAMED.write() { *w = v; } }
pub fn g_EVENT_TOLD() -> Py { ensure_init(); EVENT_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_TOLD.write() { *w = v; } }
pub fn g_HEALTH_TOLD() -> Py { ensure_init(); HEALTH_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEALTH_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = HEALTH_TOLD.write() { *w = v; } }
pub fn g_MOVE_TOLD() -> Py { ensure_init(); MOVE_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVE_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = MOVE_TOLD.write() { *w = v; } }
pub fn g_PLACE_AND_MONTH() -> Py { ensure_init(); PLACE_AND_MONTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_AND_MONTH(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_AND_MONTH.write() { *w = v; } }
pub fn g_PLAN_KINDS() -> Py { ensure_init(); PLAN_KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_KINDS.write() { *w = v; } }
pub fn g_PLAN_NOT_A_PLACE() -> Py { ensure_init(); PLAN_NOT_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_NOT_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_NOT_A_PLACE.write() { *w = v; } }
pub fn g_TIME_DOING() -> Py { ensure_init(); TIME_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_DOING(v: Py) { ensure_init(); if let Ok(mut w) = TIME_DOING.write() { *w = v; } }
pub fn g_TRIP_TOLD() -> Py { ensure_init(); TRIP_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = TRIP_TOLD.write() { *w = v; } }
pub fn g_WHAT_FIRST() -> Py { ensure_init(); WHAT_FIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_FIRST(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_FIRST.write() { *w = v; } }
pub fn g_WHAT_MISSED() -> Py { ensure_init(); WHAT_MISSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_MISSED(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_MISSED.write() { *w = v; } }
pub fn g_WHAT_TIME() -> Py { ensure_init(); WHAT_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TIME(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TIME.write() { *w = v; } }
pub fn g_WHAT_TO_TAKE() -> Py { ensure_init(); WHAT_TO_TAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_TAKE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_TAKE.write() { *w = v; } }
pub fn g_WHO_WITH_ME() -> Py { ensure_init(); WHO_WITH_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_WITH_ME(v: Py) { ensure_init(); if let Ok(mut w) = WHO_WITH_ME.write() { *w = v; } }
pub fn g_WORK_TOLD() -> Py { ensure_init(); WORK_TOLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORK_TOLD(v: Py) { ensure_init(); if let Ok(mut w) = WORK_TOLD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The ask about a plan the person has just told us about.\n\n\"we fly on the 12th\" then \"what time do i need to leave\". No knowledge entry holds it, nothing on\nthe web has it, and the hedge (\"A search should sort you out quickly\") sends a person to a search\nengine about their own booking. The plan is theirs and the missing half is in their ticket or their\nletter. What a friend does is give the rule of thumb that is worth knowing and ask the one thing\nthat fills the gap: \"What time do you need to be there? Work back from the journey, and add half an\nhour for the queue.\"\n\nThe reader is narrow on two sides. The ask has to be about them (i, me, we, us) and it has to name\nno record to look up, because \"when does my parcel arrive\" is a delivery with its own answer. And\nthe plan has to have been told to us in this chat, a few lines back: a bare \"what time do i need to\nleave\" with nothing before it belongs to whoever can answer it, not here.\n");
    s_TRIP_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:going|off|away|flying|fly|flies|travelling|traveling|trip|holiday|hols|flight|abroad|visiting|weekend away)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:new job|job interview|interview|first day|start(?:ing)? (?:work|my new job)|back to work|work at|work in|new role|promotion|training day|induction|night shift)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HEALTH_TOLD(stdlib_call("re", "compile", &[py_str("\\b(dentist|doctors?|gp|surgery|hospital|check-?up|scan|x-?ray|optician|jab|vaccine|physio|clinic|antenatal)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:moving|move house|moving house|new flat|new house|moving out|house move|removal van|packing up|exchanging contracts)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EVENT_TOLD(stdlib_call("re", "compile", &[py_str("\\b(wedding|getting married|funeral|christening|birthday party|concert|gig|match|conference|exam|driving test|course)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_AND_MONTH(stdlib_call("re", "compile", &[py_str("\\b([a-z][a-z'\\-]{2,})\\s+in\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|the (?:spring|summer|autumn))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_KINDS(py_tuple(vec![py_tuple(vec![py_str("move"), g_MOVE_TOLD()]), py_tuple(vec![py_str("health"), g_HEALTH_TOLD()]), py_tuple(vec![py_str("work"), g_WORK_TOLD()]), py_tuple(vec![py_str("event"), g_EVENT_TOLD()]), py_tuple(vec![py_str("trip"), g_TRIP_TOLD()])]));
    s_A_PLACE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:to|in|back to|visiting|off to)\\s+([a-z][a-z'\\-]{2,})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_NOT_A_PLACE(call_builtin("frozenset", &[mcall(&py_str("\nabout and appointment back bed before be been being birthday business check check-up christening\nconcert conference course do doctors early exam funeral get gig go going have holiday honeymoon\nhome hospital interview it its late match me my night party scan see sleep somebody someone test\nthat the them then there today tomorrow trip wedding work yesterday you\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_WITH_ME(stdlib_call("re", "compile", &[py_str("\\bwho(?:'s|s| is| else| all)?\\b[^?]{0,24}\\b(?:with|coming|going|travelling|traveling)\\b[^?]{0,16}\\b(?:me|us|my|our)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+time\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:leave|leaving|set off|get there|be there|arrive|arriving|set out|head off|check in|start|go|be)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_TAKE(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything)\\b[^?]{0,34}\\b(?:bring|bringing|take|taking|pack|packing|wear|wearing)\\b[^?]{0,24}\\b(?:with (?:me|us)|for the|on the|for a|to the|for my|for our)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_FIRST(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|is there anything|what else)\\b[^?]{0,44}\\b(?:before i (?:go|leave|set off|travel)|before we (?:go|leave|set off|travel)|beforehand|on the day|the day before|when i get there|when we get there|to do before (?:i|we) go|to sort before|to organise before|to organize before)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_MISSED(stdlib_call("re", "compile", &[py_str("\\b(?:anything|what) (?:have|did) i (?:miss|missed)\\b[^?]{0,30}\\b(?:while|off|away|on (?:holiday|leave)|out of)\\b|\\bwhile i was (?:off|away|on holiday|on leave)\\b|\\bcatch up on\\b[^?]{0,20}\\b(?:while|since|off|back)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THEMSELVES(stdlib_call("re", "compile", &[py_str("\\b(?:i|me|my|we|us|our|i'm|im|i've|ive)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_plan_told(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut behind: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(kind, place, word) of a plan the chat has named, or None. Only the last few lines count.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    mine = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    behind = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &mine.clone())) } }) {
    continue;
    }
    let __aug2 = add(&behind.clone(), &Py::Int(1));
    behind = __aug2;
    if py_truthy(&gt(&behind.clone(), &Py::Int(3))) {
    break;
    }
    let __iter3 = py_iter_or_raise(&g_PLAN_KINDS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk4[0].clone();
    pattern = __unpk4[1].clone();
    found = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    place = py_str("");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("trip")))) {
    let __iter5 = py_iter_or_raise(&mcall(&g_A_PLACE_NAMED(), "finditer", &[said.clone()]));
    for __item in __iter5 {
    named = __item.clone();
    word = mcall(&named.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&Py::Bool(!py_contains(&g_PLAN_NOT_A_PLACE(), &word.clone()))) {
    place = word.clone();
    break;
    }
    }
    }
    word = if py_truthy(&mcall(&found.clone(), "groups", &[])) { mcall(&found.clone(), "group", &[Py::Int(1)]) } else { py_str("") };
    return py_tuple(vec![kind.clone(), place.clone(), word.clone()]);
    }
    named = mcall(&g_PLACE_AND_MONTH(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLAN_NOT_A_PLACE(), &mcall(&named.clone(), "group", &[Py::Int(1)]))) } }) {
    return py_tuple(vec![py_str("trip"), mcall(&named.clone(), "group", &[Py::Int(1)]), py_str("")]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("Which question about the plan the line is, or None.");
    if py_truthy(&mcall(&g_WHAT_MISSED(), "search", &[clean.clone()])) {
    return py_str("missed");
    }
    if py_truthy(&mcall(&g_WHO_WITH_ME(), "search", &[clean.clone()])) {
    return py_str("who");
    }
    if py_truthy(&{ let __b = mcall(&g_WHAT_TIME(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_TIME_DOING(), "search", &[clean.clone()]) } }) {
    return py_str("time");
    }
    if py_truthy(&mcall(&g_WHAT_TO_TAKE(), "search", &[clean.clone()])) {
    return py_str("take");
    }
    if py_truthy(&mcall(&g_WHAT_FIRST(), "search", &[clean.clone()])) {
    return py_str("first");
    }
    return Py::None;
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut place: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line for an ask about their own plan, or None.\n\n    Called where the composer has reached for the search hedge: a line it has an answer of its own\n    for keeps the turn.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ABOUT_THEMSELVES(), "search", &[said.clone()])))) {
    return Py::None;
    }
    ask = _the_ask(&[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&ask.clone(), &Py::None))) {
    return Py::None;
    }
    plan = _the_plan_told(&[memory.clone(), said.clone()]);
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&plan.clone(), 3, None);
    kind = __unpk1[0].clone();
    place = __unpk1[1].clone();
    word = __unpk1[2].clone();
    return _the_line(&[ask.clone(), kind.clone(), place.clone(), word.clone()]);
    Py::None
}
pub fn _the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut place: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut word: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { py_str("") };
    let mut where_: Py = Py::None;
    let _ = py_str("One short question, and the rule worth knowing first where there is one.");
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("who")))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("health")))) {
    return py_str("Nobody has to come with you unless you are having something that needs a lift home - then ask. Is anyone going with you?");
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("event")))) {
    return py_str("Check who is named on the invitation or ticket. Who are you going with?");
    }
    if py_truthy(&place.clone()) {
    return modulo(&py_str("The booking lists everyone who is travelling to %s on it. Who are you going with?"), &mcall(&place.clone(), "capitalize", &[]));
    }
    return py_str("The booking lists everyone who is travelling on it. Who are you going with?");
    }
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("time")))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("health")))) {
    where_ = if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("check-up"), py_str("scan"), py_str("clinic")]), &word.clone())) } }) { modulo(&py_str(" at the %s"), &word.clone()) } else { py_str(" at the surgery") };
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("work")))) {
    where_ = py_str(" at work");
    } else {
    if py_truthy(&place.clone()) {
    where_ = modulo(&py_str(" in %s"), &mcall(&place.clone(), "capitalize", &[]));
    } else {
    where_ = py_str(" there");
    }
    }
    }
    return modulo(&py_str("What time do you need to be%s? Work back from the journey, and add half an hour for the queue."), &where_.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("take")))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("move")))) {
    return py_str("Pack one box of the things you need on day one and keep it with you - what's top of that list?");
    }
    if py_truthy(&place.clone()) {
    return modulo(&py_str("The forecast for %s decides most of the bag, so check it the day before. Will you be on your feet most of the day?"), &mcall(&place.clone(), "capitalize", &[]));
    }
    return py_str("It depends on what you will be doing, more than on the weather: on your feet all day, or sitting down?");
    }
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("first")))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("move")))) {
    return py_str("Meter readings and a change of address are the two that cost money if they slip. Are those done?");
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("health")))) {
    return py_str("Write down what you want to ask while you're there - it's easy to forget on the day. What's the main thing?");
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("work")))) {
    return py_str("Sort the paperwork they asked for and the route there the night before. What time are you due in?");
    }
    return py_str("Sort the one thing that has a date on it first. What's already booked?");
    }
    if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("missed")))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("work")))) {
    return py_str("Ask one person what changed while you were off - that's the whole catch-up. Who would you ask?");
    }
    return py_str("Most of what comes in while you're away sorts itself out. What's the one thing with a deadline this week?");
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.plans".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THEMSELVES" => Some(g_ABOUT_THEMSELVES()),
            "A_PLACE_NAMED" => Some(g_A_PLACE_NAMED()),
            "EVENT_TOLD" => Some(g_EVENT_TOLD()),
            "HEALTH_TOLD" => Some(g_HEALTH_TOLD()),
            "MOVE_TOLD" => Some(g_MOVE_TOLD()),
            "PLACE_AND_MONTH" => Some(g_PLACE_AND_MONTH()),
            "PLAN_KINDS" => Some(g_PLAN_KINDS()),
            "PLAN_NOT_A_PLACE" => Some(g_PLAN_NOT_A_PLACE()),
            "TIME_DOING" => Some(g_TIME_DOING()),
            "TRIP_TOLD" => Some(g_TRIP_TOLD()),
            "WHAT_FIRST" => Some(g_WHAT_FIRST()),
            "WHAT_MISSED" => Some(g_WHAT_MISSED()),
            "WHAT_TIME" => Some(g_WHAT_TIME()),
            "WHAT_TO_TAKE" => Some(g_WHAT_TO_TAKE()),
            "WHO_WITH_ME" => Some(g_WHO_WITH_ME()),
            "WORK_TOLD" => Some(g_WORK_TOLD()),
            "_the_ask" => Some(py_func("_the_ask", |args: &[Py]| _the_ask(args))),
            "_the_line" => Some(py_func("_the_line", |args: &[Py]| _the_line(args))),
            "_the_plan_told" => Some(py_func("_the_plan_told", |args: &[Py]| _the_plan_told(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

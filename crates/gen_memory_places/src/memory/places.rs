#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use pyrt::builtins::*;
use pyrt::pybuiltins::*;
use pyrt::stdlib::*;
use pyrt::value::*;
use pyrt::exc;
use pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


thread_local! { static G_A_QUESTION_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PERSON_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PLACE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_others: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.places: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_QUESTION_ASKED() -> Py { ensure_init(); G_A_QUESTION_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ASKED(v: Py) { ensure_init(); G_A_QUESTION_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE_VALUE() -> Py { ensure_init(); G_NOT_A_PLACE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE_VALUE(v: Py) { ensure_init(); G_NOT_A_PLACE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PERSON_NAMED() -> Py { ensure_init(); G_OTHER_PERSON_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PERSON_NAMED(v: Py) { ensure_init(); G_OTHER_PERSON_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PLACE() -> Py { ensure_init(); G_OTHER_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PLACE(v: Py) { ensure_init(); G_OTHER_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PLACE_PRONOUN() -> Py { ensure_init(); G_OTHER_PLACE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PLACE_PRONOUN(v: Py) { ensure_init(); G_OTHER_PLACE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ENDS() -> Py { ensure_init(); G_PLACE_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ENDS(v: Py) { ensure_init(); G_PLACE_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASK() -> Py { ensure_init(); G_WHERE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASK(v: Py) { ensure_init(); G_WHERE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_others() -> Py { ensure_init(); G_others.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_others(v: Py) { ensure_init(); G_others.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Where somebody lives, read out of what they said - and said back.\n\nThese are the readers the store answers \"where does my sister live\" from: the town a line gave\n(\"my sister lives in cork\"), the pronoun that stands for the person the chat has just been about\n(\"she moved to york\"), the correction that moves somebody's town (\"no, york\"), and the say-back\nline built from them.\n\nThey live here rather than in `bot/modules/memory_store.py` because that module had grown past the\nsize the world accepts a change to. Nothing changed in the move: the store imports every name\nback, so the readers and their callers are the same objects they were.\n");
    s_others(py_func("others", |args: &[Py]| pyrt::xrefs::xcall("memory", "others", args)));
    s_WHERE_ASK(pyrt::xrefs::xget("memory.store_lines", "WHERE_ASK"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.OTHER_PLACE"); }
    s_OTHER_PLACE(stdlib_call("re", "compile", &[py_str("\\bmy (\\w+)s?\\b[^.!?]{0,60}?\\b(?:live|lives|living|moved|moves|moving|settled|based|staying|relocated)\\s+(?:in|to|near|at)\\s+([A-Za-z][A-Za-z'-]{1,}(?:[ -][A-Za-z][A-Za-z'-]{1,})?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.OTHER_PLACE_PRONOUN"); }
    s_OTHER_PLACE_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they)\\s+(?:has\\s+|had\\s+|just\\s+|now\\s+)?(?:live[sd]?|lives|living|moved|moves|moving|settled|relocated|is\\s+based|are\\s+based)\\s+(?:in|to|near|at)\\s+([A-Za-z][A-Za-z'-]{1,}(?:[ -][A-Za-z][A-Za-z'-]{1,})?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.PLACE_ENDS"); }
    s_PLACE_ENDS(call_builtin("frozenset", &[mcall(&py_str("last next this that year years month months week weeks ago previously\nbefore then soon now already after"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.OTHER_PERSON_NAMED"); }
    s_OTHER_PERSON_NAMED(stdlib_call("re", "compile", &[py_str("\\bmy (\\w+)s?\\b(?:[^.!?]{0,40}?\\b(?:is|are|was|were)\\s+(?:called|named|known as)\\b|[^.!?]{0,12}$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.A_QUESTION_ASKED"); }
    s_A_QUESTION_ASKED(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|but\\s+|so\\s+)?(?:do|does|did|is|are|was|were|has|have|had|can|could|will|would|should|shall|may|might|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.places.NOT_A_PLACE_VALUE"); }
    s_NOT_A_PLACE_VALUE(call_builtin("frozenset", &[mcall(&py_str("\ni me my we us our you your he him his she her it its they them their\none two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen\nsixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety\nhundred thousand million first second third last next again too either neither\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut first: Py = Py::None;
    let mut low: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The place in a phrase of words, or None when the words are not one.");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[ -]"), value.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_ENDS(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "NOT_PLACES"), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HOME_STOP"), &low.clone())) } } } }) {
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    first = mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HOME_STOP"), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "NOT_PLACES"), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_ENDS(), &first.clone())) } } } }) {
    return Py::None;
    }
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(if py_truthy(&mcall(&w.clone(), "islower", &[])) { op_add(&mcall(&slice_or_raise(&w.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&w.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { w.clone() }); } py_list(__out) })() }]);
    Py::None
}
pub fn _the_place_alone(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The place a line names with no frame around it: \"no, york\", \"sorry, in york\", \"i meant york\".\n\n    A correction that names only the new value leaves the frame to the line it corrects, and a\n    place for somebody else is read out of the whole conversation rather than out of one line, so\n    \"my sister lives in perth\" then \"no, york\" had nothing to move: the walk below only ever read\n    a place out of a line with a person and a live verb in it. Only a line that opens with a\n    take-back and carries a single place is read this way - \"no, tree nuts\" and \"no, seven\" name\n    no town, and a line that states a fact of its own is left to the readers that claim it.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "rstrip", &[py_str(".!,")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&low.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    opener = mcall(&pyrt::xrefs::xget("memory.facts", "CORRECTION_OPENER"), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&opener.clone()))) {
    opener = stdlib_call("re", "match", &[py_str("not\\s+[a-z'-]+[, ]+"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&opener.clone()))) {
    return Py::None;
    }
    }
    rest = mcall(&slice_or_raise(&low.clone(), &mcall(&opener.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,!-")]);
    rest = stdlib_call("re", "sub", &[py_str("^(?:in|to|at|from|near)\\s+"), py_str(""), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_PLACE_VALUE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return Py::None;
    }
    return { let __b = pyrt::xrefs::xcall("memory.facts", "place_value", &[rest.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _said_places(args: &[Py]) -> Py {
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
    let mut alone: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut gave: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut place: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Every place the user gave for somebody else, in the order they gave them.\n\n    The person is named in the line (\"my sister lives in cork\") or left to the pronoun that\n    stands for the last person named (\"she moved to york\"), so the walk carries the two\n    together: what the chat says last about somebody is what the store says back. A correction\n    that names only the new town (\"no, york\") is the same thing said again, so it moves the town\n    of the person the line before it gave one for - and only there, so a bare \"no, york\" about\n    their own city never moves somebody else's.\n    ");
    found = py_list(vec![]);
    last = py_str("");
    fresh = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    text = { let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&text.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION_ASKED(), "search", &[text.clone()]) } } } }) {
    fresh = Py::Bool(false);
    continue;
    }
    named = py_str("");
    gave = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&mcall(&g_OTHER_PLACE(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    place = _a_place(&[mcall(&match_.clone(), "group", &[Py::Int(2)])]);
    named = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&place.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![named.clone(), place.clone()])]);
    gave = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    asked = mcall(&g_OTHER_PERSON_NAMED(), "search", &[mcall(&text.clone(), "rstrip", &[])]);
    if py_truthy(&asked.clone()) {
    named = mcall(&asked.clone(), "group", &[Py::Int(1)]);
    }
    }
    if py_truthy(&named.clone()) {
    last = named.clone();
    } else {
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_OTHER_PLACE_PRONOUN(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { fresh.clone() } } } }) {
    alone = _the_place_alone(&[text.clone()]);
    if py_truthy(&alone.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![last.clone(), alone.clone()])]);
    gave = Py::Bool(true);
    }
    }
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gave.clone())) } }) {
    let __iter3 = py_iter_or_raise(&mcall(&g_OTHER_PLACE_PRONOUN(), "finditer", &[text.clone()]));
    for __item in __iter3 {
    match_ = __item.clone();
    place = _a_place(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&place.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![last.clone(), place.clone()])]);
    gave = Py::Bool(true);
    }
    }
    }
    fresh = gave.clone();
    }
    return found.clone();
    Py::None
}
pub fn _a_town_moved(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut alone: Py = Py::None;
    let mut before: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut low: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The place a correction names for somebody else, said back as a line.\n\n    \"my sister lives in perth\" then \"no, york\" was a shrug and the store still said Perth, so the\n    correction is answered where it is made and not only a turn later when they ask about her.\n    ");
    alone = _the_place_alone(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&alone.clone()))) {
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "split", &[])]);
    frame = if py_truthy(&mcall(&pyrt::xrefs::xget("memory.facts", "CORRECTION_OPENER"), "match", &[low.clone()])) { mcall(&g_OTHER_PLACE_PRONOUN(), "search", &[low.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&frame.clone()))) {
    return Py::None;
    }
    alone = _a_place(&[mcall(&frame.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(!py_truthy(&alone.clone()))) {
    return Py::None;
    }
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); __out.push({ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }); } py_list(__out) })() };
    before = if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_OTHER_PLACE(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OTHER_PLACE_PRONOUN(), "search", &[before.clone()]) } })) } }) {
    return Py::None;
    }
    said = _said_places(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&geti(&said.clone(), &op_neg(&Py::Int(1))), &Py::Int(1)), "lower", &[]), &mcall(&alone.clone(), "lower", &[]))) } }) {
    return Py::None;
    }
    who = mcall(&call_builtin("str", &[geti(&geti(&said.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    plural = mcall(&who.clone(), "endswith", &[py_str("s")]);
    return op_add(&op_add(&op_add(&op_add(&py_str("Got it — your "), &who.clone()), &if py_truthy(&plural.clone()) { py_str(" live in ") } else { py_str(" lives in ") }), &alone.clone()), &py_str("."));
    Py::None
}
pub fn _other_place(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut found: Py = Py::None;
    let mut place: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("The last place the user gave for this person, said back as a line.");
    want = mcall(&mcall(&{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "rstrip", &[py_str("s")]);
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_said_places(&[memory.clone()])) { let __unpk8940 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk8940[0].clone(); let mut place: Py = __unpk8940[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&want.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&said.clone(), "lower", &[]), "rstrip", &[py_str("s")]), &want.clone())) } }) {  __out.push(py_tuple(vec![said.clone(), place.clone()])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&geti(&found.clone(), &op_neg(&Py::Int(1))), 2, None);
    said = __unpk1[0].clone();
    place = __unpk1[1].clone();
    plural = mcall(&mcall(&said.clone(), "lower", &[]), "endswith", &[py_str("s")]);
    return op_add(&op_add(&op_add(&op_add(&py_str("Your "), &mcall(&said.clone(), "lower", &[])), &if py_truthy(&plural.clone()) { py_str(" live in ") } else { py_str(" lives in ") }), &place.clone()), &py_str("."));
    Py::None
}
pub fn _asked_person(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut by_name: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Who a place ask is about, when it is about somebody other than the user.\n\n    \"where does my sister live\" names her, \"and where does she live\" leaves her to the pronoun\n    the chat has just been about, and \"where does jane live\" names her rather than her relation.\n    All three ask the same thing, so all three are read here and answered from the chat.\n    ");
    named = stdlib_call("re", "search", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_WHERE_ASK()), &py_str(" (?:does|do) my (\\w+) (?:live|come from)\\b|\\b")), &g_WHERE_ASK()), &py_str(" is my (\\w+) (?:living|based)\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&named.clone()) {
    return { let __b = mcall(&named.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&named.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_WHERE_ASK()), &py_str(" (?:does|do) (?:he|she|they) (?:live|come from)\\b|\\b")), &g_WHERE_ASK()), &py_str(" is (?:he|she|they) (?:living|based)\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    person = mcall(&memory.clone(), "get", &[py_str("person")]);
    who = if py_truthy(&call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&person.clone(), "get", &[py_str("who")]) } else { py_str("") };
    if py_truthy(&who.clone()) {
    return mcall(&call_builtin("str", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    said = _said_places(&[memory.clone()]);
    return if py_truthy(&said.clone()) { mcall(&call_builtin("str", &[geti(&geti(&said.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    }
    plain = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &g_WHERE_ASK()), &py_str(" (?:is|are|was|were) (?:my|our) ([a-z]+)\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&plain.clone()) {
    word = mcall(&mcall(&plain.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "KIN_WORDS"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &word.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()])) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return word.clone();
    }
    }
    by_name = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &g_WHERE_ASK()), &py_str(" (?:does|do) ([a-z][a-z'-]{1,}) (?:live|come from)\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&by_name.clone()) {
    word = mcall(&mcall(&by_name.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("my"), py_str("your"), py_str("i"), py_str("we"), py_str("it"), py_str("the"), py_str("a")]), &word.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &word.clone()))) {
    return mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.places".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_QUESTION_ASKED" => Some(g_A_QUESTION_ASKED()),
            "NOT_A_PLACE_VALUE" => Some(g_NOT_A_PLACE_VALUE()),
            "OTHER_PERSON_NAMED" => Some(g_OTHER_PERSON_NAMED()),
            "OTHER_PLACE" => Some(g_OTHER_PLACE()),
            "OTHER_PLACE_PRONOUN" => Some(g_OTHER_PLACE_PRONOUN()),
            "PLACE_ENDS" => Some(g_PLACE_ENDS()),
            "WHERE_ASK" => Some(g_WHERE_ASK()),
            "_a_place" => Some(py_func("_a_place", |args: &[Py]| _a_place(args))),
            "_a_town_moved" => Some(py_func("_a_town_moved", |args: &[Py]| _a_town_moved(args))),
            "_asked_person" => Some(py_func("_asked_person", |args: &[Py]| _asked_person(args))),
            "_other_place" => Some(py_func("_other_place", |args: &[Py]| _other_place(args))),
            "_said_places" => Some(py_func("_said_places", |args: &[Py]| _said_places(args))),
            "_the_place_alone" => Some(py_func("_the_place_alone", |args: &[Py]| _the_place_alone(args))),
            "others" => Some(g_others()),
            "extract" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_QUESTION_ASKED" => s_A_QUESTION_ASKED(v),
            "NOT_A_PLACE_VALUE" => s_NOT_A_PLACE_VALUE(v),
            "OTHER_PERSON_NAMED" => s_OTHER_PERSON_NAMED(v),
            "OTHER_PLACE" => s_OTHER_PLACE(v),
            "OTHER_PLACE_PRONOUN" => s_OTHER_PLACE_PRONOUN(v),
            "PLACE_ENDS" => s_PLACE_ENDS(v),
            "WHERE_ASK" => s_WHERE_ASK(v),
            "others" => s_others(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.places", module_obj);
    pyrt::xrefs::reg_fn("memory.places", "_a_place", py_func("_a_place", |args: &[Py]| _a_place(args)));
    pyrt::xrefs::reg_fn("memory.places", "_the_place_alone", py_func("_the_place_alone", |args: &[Py]| _the_place_alone(args)));
    pyrt::xrefs::reg_fn("memory.places", "_said_places", py_func("_said_places", |args: &[Py]| _said_places(args)));
    pyrt::xrefs::reg_fn("memory.places", "_a_town_moved", py_func("_a_town_moved", |args: &[Py]| _a_town_moved(args)));
    pyrt::xrefs::reg_fn("memory.places", "_other_place", py_func("_other_place", |args: &[Py]| _other_place(args)));
    pyrt::xrefs::reg_fn("memory.places", "_asked_person", py_func("_asked_person", |args: &[Py]| _asked_person(args)));
    pyrt::xrefs::reg_access("memory.places", "A_QUESTION_ASKED", g_A_QUESTION_ASKED, s_A_QUESTION_ASKED);
    pyrt::xrefs::reg_access("memory.places", "NOT_A_PLACE_VALUE", g_NOT_A_PLACE_VALUE, s_NOT_A_PLACE_VALUE);
    pyrt::xrefs::reg_access("memory.places", "OTHER_PERSON_NAMED", g_OTHER_PERSON_NAMED, s_OTHER_PERSON_NAMED);
    pyrt::xrefs::reg_access("memory.places", "OTHER_PLACE", g_OTHER_PLACE, s_OTHER_PLACE);
    pyrt::xrefs::reg_access("memory.places", "OTHER_PLACE_PRONOUN", g_OTHER_PLACE_PRONOUN, s_OTHER_PLACE_PRONOUN);
    pyrt::xrefs::reg_access("memory.places", "PLACE_ENDS", g_PLACE_ENDS, s_PLACE_ENDS);
    pyrt::xrefs::reg_access("memory.places", "WHERE_ASK", g_WHERE_ASK, s_WHERE_ASK);
    pyrt::xrefs::reg_access("memory.places", "others", g_others, s_others);
}

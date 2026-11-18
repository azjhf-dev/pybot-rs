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


thread_local! { static G_BARE_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALL_MADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_READ_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANTITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY_TOLD_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BARE_DOUBT() -> Py { ensure_init(); G_BARE_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DOUBT(v: Py) { ensure_init(); G_BARE_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_CALL_MADE() -> Py { ensure_init(); G_CALL_MADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALL_MADE(v: Py) { ensure_init(); G_CALL_MADE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_LINES() -> Py { ensure_init(); G_CARE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_LINES(v: Py) { ensure_init(); G_CARE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_NAMED() -> Py { ensure_init(); G_CARE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_NAMED(v: Py) { ensure_init(); G_CARE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_READ_BACK() -> Py { ensure_init(); G_NOT_READ_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_READ_BACK(v: Py) { ensure_init(); G_NOT_READ_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANTITY_ASK() -> Py { ensure_init(); G_QUANTITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANTITY_ASK(v: Py) { ensure_init(); G_QUANTITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY_TOLD_THEM() -> Py { ensure_init(); G_SOMEBODY_TOLD_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_TOLD_THEM(v: Py) { ensure_init(); G_SOMEBODY_TOLD_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_TOLD() -> Py { ensure_init(); G_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOLD(v: Py) { ensure_init(); G_TOLD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A doubt on top of \"get it looked at\" is \"was that an overreaction\", and it was not.\n\n`bot/compose/verdict` answers \"is that bad\" over an answer from the knowledge base, and it does\nthat well. Where it stands aside, chat's acknowledgement pool closes the conversation instead:\nafter \"Get it checked by a GP within two weeks\" the person asked \"is that bad\" and got \"Alright\nthen - good luck with it.\", and the same pool answered \"is that ok\" and \"is that enough\" with the\nsame sign-off. The acknowledgement pool is for the end of a topic, and this is not the end of one.\n\nOnly the shape this file can answer is here. A doubt on top of an answer that named somebody to\nsee or somewhere to go (\"get it checked\", \"see your GP\", \"ring 111\", \"go to A&E\") is asking\nwhether the call was fussing, and the answer is that it was not: the call is the point of the\nanswer above. Every other doubt is left to the composer and to chat, because what it is about is\nin the answer above and guessing at it from here is how a sentence lands about the wrong thing.\n\nNothing here names a symptom, a body part, a person or a pronoun, so it cannot say whose it is.\nA safety, urgent, grief or loss answer is never read back: after one of those the person has\nalready been told what to do now, and a line about the call being right is not the next thing.\n");
    s_BARE_DOUBT(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |ok |okay |oh |right |hmm )*(?:(?:is|was|are|were|does|do|did) (?:that|this|it)(?: all)?(?: (?:really|actually|very|too|that|so))?(?: (?:bad|normal|serious|dangerous|ok|okay|fine|safe|good|alright|a lot|too much|too often|too many|enough|much of a problem|a problem|something to worry about|anything to worry about))|(?:should|do|ought) i (?:be )?(?:worried|worry|worrying|bothered)|am i (?:ok|okay|alright|all right|normal|right))(?: then| though| honestly| here)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUANTITY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:enough|a lot|too much|too many|too often)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALL_MADE(py_str("(?:get (?:it|that|them|yourself) (?:checked|looked at|seen)|get (?:an|a) (?:appointment|referral)|see (?:a|your|the) (?:gp|doctor|nurse|dentist|optician|physio)|ring (?:999|911|112|111|your gp|the gp|the surgery)|call (?:999|911|112|111)|go to (?:a&e|the hospital|a walk-in|minor injuries)|book (?:an appointment|a gp)|photograph it|have it looked at)"));
    s_CARE_NAMED(stdlib_call("re", "compile", &[op_add(&py_str("(?:^|[.!?]\\s+)(?:then |also |first )?"), &g_CALL_MADE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEBODY_TOLD_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:gp|doctor|doctors|pharmacist|chemist|nurse|practice|surgery|dentist|optician|physio|physiotherapist|midwife|health visitor|consultant|vet|vets|111|a&e|hospital|walk-in|paramedic|ambulance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|say|told|tells|tell|advised|advises|wants|wanted|reckons|thinks|suggested|recommended|sent me|referred|booked|making me|making me go|insisted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_LINES(py_tuple(vec![py_str("Yes - worth getting seen rather than waiting on."), py_str("Yes, get it seen. Asking is not fussing."), py_str("Yes - that is a call worth making, not an overreaction.")]));
    s_NOT_READ_BACK(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("urgent"), py_str("grief"), py_str("loss")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _last_bot(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's last line before this turn, as (text, module).");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&text.clone()) {
    return py_tuple(vec![text.clone(), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn reads_as_a_doubt(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let _ = py_str("True when the whole line is a doubt about the answer above and nothing else.");
    flat = stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    flat = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), flat.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return { let __b = call_builtin("bool", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&g_BARE_DOUBT(), "match", &[flat.clone()]), &Py::None)) } };
    Py::None
}
pub fn _the_line_behind(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own line that the answer above was answering, lowercased.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) {
    return mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    }
    return py_str("");
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut behind: Py = Py::None;
    let mut module: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The answer to a doubt on top of a call to get it seen, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_doubt(&[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_QUANTITY_ASK(), "search", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_last_bot(&[memory.clone()]), 2, None);
    said = __unpk1[0].clone();
    module = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_READ_BACK(), &module.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_CARE_NAMED(), "search", &[said.clone()])))) {
    return Py::None;
    }
    behind = _the_line_behind(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SOMEBODY_TOLD_THEM(), "search", &[behind.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_TOLD(), "search", &[behind.clone()]) } }))) {
    return Py::None;
    }
    return crate::compose::pick(&[g_CARE_LINES(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.doubt".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BARE_DOUBT" => Some(g_BARE_DOUBT()),
            "CALL_MADE" => Some(g_CALL_MADE()),
            "CARE_LINES" => Some(g_CARE_LINES()),
            "CARE_NAMED" => Some(g_CARE_NAMED()),
            "NOT_READ_BACK" => Some(g_NOT_READ_BACK()),
            "QUANTITY_ASK" => Some(g_QUANTITY_ASK()),
            "SOMEBODY_TOLD_THEM" => Some(g_SOMEBODY_TOLD_THEM()),
            "TOLD" => Some(g_TOLD()),
            "_last_bot" => Some(py_func("_last_bot", |args: &[Py]| _last_bot(args))),
            "_the_line_behind" => Some(py_func("_the_line_behind", |args: &[Py]| _the_line_behind(args))),
            "reads_as_a_doubt" => Some(py_func("reads_as_a_doubt", |args: &[Py]| reads_as_a_doubt(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BARE_DOUBT" => s_BARE_DOUBT(v),
            "CALL_MADE" => s_CALL_MADE(v),
            "CARE_LINES" => s_CARE_LINES(v),
            "CARE_NAMED" => s_CARE_NAMED(v),
            "NOT_READ_BACK" => s_NOT_READ_BACK(v),
            "QUANTITY_ASK" => s_QUANTITY_ASK(v),
            "SOMEBODY_TOLD_THEM" => s_SOMEBODY_TOLD_THEM(v),
            "TOLD" => s_TOLD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.doubt", module_obj);
    pyrt::xrefs::reg_fn("compose.doubt", "_last_bot", py_func("_last_bot", |args: &[Py]| _last_bot(args)));
    pyrt::xrefs::reg_fn("compose.doubt", "reads_as_a_doubt", py_func("reads_as_a_doubt", |args: &[Py]| reads_as_a_doubt(args)));
    pyrt::xrefs::reg_fn("compose.doubt", "_the_line_behind", py_func("_the_line_behind", |args: &[Py]| _the_line_behind(args)));
    pyrt::xrefs::reg_fn("compose.doubt", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.doubt", "BARE_DOUBT", g_BARE_DOUBT, s_BARE_DOUBT);
    pyrt::xrefs::reg_access("compose.doubt", "CALL_MADE", g_CALL_MADE, s_CALL_MADE);
    pyrt::xrefs::reg_access("compose.doubt", "CARE_LINES", g_CARE_LINES, s_CARE_LINES);
    pyrt::xrefs::reg_access("compose.doubt", "CARE_NAMED", g_CARE_NAMED, s_CARE_NAMED);
    pyrt::xrefs::reg_access("compose.doubt", "NOT_READ_BACK", g_NOT_READ_BACK, s_NOT_READ_BACK);
    pyrt::xrefs::reg_access("compose.doubt", "QUANTITY_ASK", g_QUANTITY_ASK, s_QUANTITY_ASK);
    pyrt::xrefs::reg_access("compose.doubt", "SOMEBODY_TOLD_THEM", g_SOMEBODY_TOLD_THEM, s_SOMEBODY_TOLD_THEM);
    pyrt::xrefs::reg_access("compose.doubt", "TOLD", g_TOLD, s_TOLD);
}

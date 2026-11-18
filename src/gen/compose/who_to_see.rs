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


pub static ANYBODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_OBJECT_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LONG_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SIGN_MEANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUBSET_OF_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITION_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_PHONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANYBODY() -> Py { ensure_init(); ANYBODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANYBODY(v: Py) { ensure_init(); if let Ok(mut w) = ANYBODY.write() { *w = v; } }
pub fn g_AN_OBJECT_OF() -> Py { ensure_init(); AN_OBJECT_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_OBJECT_OF(v: Py) { ensure_init(); if let Ok(mut w) = AN_OBJECT_OF.write() { *w = v; } }
pub fn g_ASK() -> Py { ensure_init(); ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ASK.write() { *w = v; } }
pub fn g_A_LONG_CALL() -> Py { ensure_init(); A_LONG_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LONG_CALL(v: Py) { ensure_init(); if let Ok(mut w) = A_LONG_CALL.write() { *w = v; } }
pub fn g_A_PERSON() -> Py { ensure_init(); A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON.write() { *w = v; } }
pub fn g_A_PLACE() -> Py { ensure_init(); A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = A_PLACE.write() { *w = v; } }
pub fn g_A_SIGN_MEANS() -> Py { ensure_init(); A_SIGN_MEANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SIGN_MEANS(v: Py) { ensure_init(); if let Ok(mut w) = A_SIGN_MEANS.write() { *w = v; } }
pub fn g_A_SUBSET_OF_TIME() -> Py { ensure_init(); A_SUBSET_OF_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUBSET_OF_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_SUBSET_OF_TIME.write() { *w = v; } }
pub fn g_CONDITION_LINES() -> Py { ensure_init(); CONDITION_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITION_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CONDITION_LINES.write() { *w = v; } }
pub fn g_ON_THE_PHONE_LINES() -> Py { ensure_init(); ON_THE_PHONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_PHONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_PHONE_LINES.write() { *w = v; } }
pub fn g_PLACE_LINES() -> Py { ensure_init(); PLACE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_LINES.write() { *w = v; } }
pub fn g_SAYS_TO_DO() -> Py { ensure_init(); SAYS_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_TO_DO.write() { *w = v; } }
pub fn g_SOMEONE_LINES() -> Py { ensure_init(); SOMEONE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_LINES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A bare \"do i need to see anyone\" is answered by the someone the answer already named.\n\n    my dog ate chocolate\n      \"Yes, chocolate is poisonous to dogs: the darker the chocolate, the worse it is. Ring your\n       vet with the dog's weight and roughly how much and what kind of chocolate it ate; don't\n       wait for symptoms.\"\n    do i need to see anyone\n      before: \"That's beyond what I know well, sorry. Searching for it will turn up better sources\n               than I can offer; tell me what you find and I'll help you weigh it up.\"\n      after:  \"Yes - your vet. Ring them with the dog's weight and roughly how much and what kind\n               of chocolate it ate.\"\n\nThe answer had already said who to ring, in an instruction with no fence around it, and the line\nasks only whether somebody should be seen. Nothing in the knowledge base matches those words -\n\"anyone\" is not a topic - so the turn fell to the don't-know line, which is the one reply that\nthrows away an answer the bot had just given.\n\nWhat this file is careful about:\n\n* The ask must be **bare**. A line that names what it is about (\"do i need to see anyone about my\n  tax return\", \"i need to see a doctor tomorrow\") is somebody else's, and is left alone.\n* The answer in hand must name a **person to contact**, in an instruction with no \"if\", \"when\" or\n  \"most days\" in front of it - the reading `verdict._referral` already makes, so the fence rules\n  written there are the ones used here. An answer that says \"see a doctor if it is still there in\n  three weeks\" is advice for later, and the doubt about it is answered by the condition, not by\n  this.\n* The reply is the answer's own words, cut to the instruction. Nothing is added about the thing\n  the answer was about, so the bot cannot say more than it already did.\n");
    s_ASK(stdlib_call("re", "compile", &[py_str("(?:do|should|shall|must|need|will|would|can|could|have)\\s+(?:i|we|you)\\s+(?:(?:need|have|got|want)\\s+to\\s+|really\\s+|actually\\s+|even\\s+|just\\s+)?(?:see|ring|call|visit|contact|speak\\s+to|talk\\s+to|go\\s+(?:to|and\\s+see)|get\\s+seen\\s+by|book\\s+(?:in\\s+with|an?\\s+appointment\\s+with)|get\\s+(?:an?\\s+)?appointment\\s+with)\\s+(?:somebody|someone|anybody|anyone|a\\s+professional|a\\s+(?:doctor|vet|dentist|nurse|gp|pharmacist|optician|midwife|lawyer|plumber|electrician|specialist)|the\\s+(?:doctor|vet|dentist|nurse|gp|pharmacist|optician))\\b\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:your|the|a|an|their|his|her)?\\s*(vet|doctor|gp|dentist|nurse|pharmacist|optician|midwife|health visitor|paramedic|physio(?:therapist)?|practice nurse|a&e|111|999)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE(stdlib_call("re", "compile", &[py_str("(?:a&e|the\\s+emergency\\s+department|walk-?in\\s+centre|minor\\s+injuries(?:\\s+unit)?|urgent\\s+care|out\\s+of\\s+hours|999|911|111|112)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_LINES(py_tuple(vec![py_str("Yes - {person}. {call}"), py_str("Yes, {person} - {call_lower}"), py_str("The answer named one: {person}. {call}"), py_str("Yes: {person}. {call}")]));
    s_PLACE_LINES(py_tuple(vec![py_str("Yes - {call_lower}"), py_str("{call}"), py_str("Yes, that one: {call_lower}")]));
    s_ON_THE_PHONE_LINES(py_tuple(vec![py_str("{call}"), py_str("Yes - {call_lower}"), py_str("The answer said it plainly: {call_lower}")]));
    s_A_LONG_CALL(Py::Int(16));
    s_A_SIGN_MEANS(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|when|whenever|means|would\\s+mean|otherwise|in\\s+case)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANYBODY(stdlib_call("re", "compile", &[py_str("\\b(?:someone|somebody|anyone|anybody|a\\s+professional)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_OBJECT_OF(stdlib_call("re", "compile", &[py_str("\\b(?:see|ring|call|contact|visit|ask|speak\\s+to|talk\\s+to|say\\s+to|saying\\s+to|go\\s+to|book\\s+in\\s+with|appointments?\\s+with|with|to)\\s*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_LINES(py_tuple(vec![py_str("Not always. {clause}"), py_str("It depends. {clause}"), py_str("Go by this. {clause}"), py_str("Only if it comes to that. {clause}")]));
    s_A_SUBSET_OF_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:that|which|who)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_TO_DO(stdlib_call("re", "compile", &[py_str("\\b(?:worth|should|needs?|must|ring|call|see|speak|talk|book|get|go|make an appointment|appointment|check(?:ed)?|look(?:ed)? at)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _clean(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _the_answer_in_hand(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply the bot gave last, or \"\".");
    turns = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("bot")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_condition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut person: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence making a referral depend on a sign, cut to the sign.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    person = { let __b = mcall(&g_A_PERSON(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ANYBODY(), "search", &[sentence.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::compose::verdict::g_NAMES_A_CLINICIAN(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ANYBODY(), "search", &[sentence.clone()]) } })) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_SIGN_MEANS(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { _a_sign_not_a_step(&[sentence.clone(), person.clone()]) } }))) {
    continue;
    }
    clause = mcall(&sentence.clone(), "strip", &[]);
    cut = mcall(&clause.clone(), "rfind", &[py_str(", but ")]);
    if py_truthy(&{ let __b = gt(&cut.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { _a_sign_not_a_step(&[clause.clone(), person.clone()]) } }) {
    clause = slice_or_raise(&clause.clone(), &add(&cut.clone(), &call_builtin("len", &[py_str(", but ")]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    clause = add(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return clause.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { py_str("") };
    let mut said_now: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own question, if it asked one the person can answer.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    said_now = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&{ let __b = said_now.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&clause.clone(), &said_now.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&said_now.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said_now.clone(), &py_str("? "))) } } } } } }) {
    if py_truthy(&Py::Bool(py_contains(&said_now.clone(), &py_str("?")))) {
    return slice_or_raise(&said_now.clone(), &Py::None, &add(&mcall(&said_now.clone(), "index", &[py_str("?")]), &Py::Int(1)), &Py::None);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_condition_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clause: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut written: Py = Py::None;
    clause = _the_condition(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return py_str("");
    }
    written = add(&mcall(&clause.clone(), "rstrip", &[py_str(" .")]), &py_str("."));
    reply = mcall(&crate::gen::compose::pick(&[g_CONDITION_LINES(), memory.clone(), clause.clone()]), "replace", &[py_str("{clause}"), written.clone()]);
    question = _the_question(&[said.clone(), clause.clone()]);
    if py_truthy(&{ let __b = question.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&reply.clone(), &question.clone())) } }) {
    reply = add(&add(&mcall(&reply.clone(), "rstrip", &[]), &py_str(" ")), &question.clone());
    }
    return reply.clone();
    Py::None
}
pub fn _a_sign_not_a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when what comes before the named person makes the referral depend on a sign.");
    if py_truthy(&Py::Bool(py_is(&person.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &mcall(&person.clone(), "start", &[]), &Py::None);
    return call_builtin("bool", &[{ let __b = mcall(&g_A_SIGN_MEANS(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SUBSET_OF_TIME(), "search", &[before.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_sentence_that_names_someone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence naming somebody to contact, if it is advice and not a condition.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::compose::verdict::g_NAMES_A_CLINICIAN(), "search", &[sentence.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&crate::gen::compose::verdict::g_FENCED_IN_SENTENCE(), "search", &[sentence.clone()])) {
    continue;
    }
    if py_truthy(&_a_sign_not_a_step(&[sentence.clone(), mcall(&g_A_PERSON(), "search", &[sentence.clone()])])) {
    continue;
    }
    if py_truthy(&mcall(&g_SAYS_TO_DO(), "search", &[sentence.clone()])) {
    return sentence.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_call(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut breaks: Py = Py::None;
    let mut call: Py = Py::None;
    let mut head: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut person: Py = Py::None;
    let mut person_in_it: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The instruction in the answer that names somebody to contact, and who it names.");
    sentence = crate::gen::compose::verdict::_the_referral_sentence(&[said.clone()]);
    person_in_it = if py_truthy(&sentence.clone()) { mcall(&g_A_PERSON(), "search", &[sentence.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&person_in_it.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _a_sign_not_a_step(&[sentence.clone(), person_in_it.clone()]) } } } }) {
    sentence = py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&crate::gen::compose::verdict::_referral(&[said.clone()]), &Py::None)) } }) {
    sentence = _a_sentence_that_names_someone(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    return py_tuple(vec![py_str(""), Py::None]);
    }
    }
    text = mcall(&call_builtin("str", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    call = text.clone();
    head = mcall(&geti(&mcall(&text.clone(), "split", &[py_str(";")]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&mcall(&g_A_PERSON(), "search", &[head.clone()])) {
    call = head.clone();
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&call.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_LONG_CALL())) {
    person = mcall(&g_A_PERSON(), "search", &[call.clone()]);
    tail = if py_truthy(&person.clone()) { slice_or_raise(&call.clone(), &mcall(&person.clone(), "end", &[]), &Py::None, &Py::None) } else { py_str("") };
    breaks = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(","), py_str(":"), py_str(";")])) { let mut mark: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&tail.clone(), &mark.clone()))) {  __out.push(mcall(&tail.clone(), "index", &[mark.clone()])); } } py_list(__out) })() };
    if py_truthy(&breaks.clone()) {
    call = mcall(&slice_or_raise(&call.clone(), &Py::None, &add(&mcall(&person.clone(), "end", &[]), &call_builtin("min", &[breaks.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "strip", &[]);
    }
    }
    call = mcall(&call.clone(), "rstrip", &[py_str(" .;:,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PERSON(), "search", &[call.clone()]))) } }) {
    return py_tuple(vec![py_str(""), Py::None]);
    }
    return py_tuple(vec![add(&mcall(&geti(&call.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&call.clone(), &Py::Int(1), &Py::None, &Py::None)), mcall(&g_A_PERSON(), "search", &[call.clone()])]);
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
    let mut a_place: Py = Py::None;
    let mut after: Py = Py::None;
    let mut call: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut on_the_phone: Py = Py::None;
    let mut person: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_again: Py = Py::None;
    let _ = py_str("The reply to a bare \"do i need to see someone\", or None.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASK(), "match", &[_clean(&[text.clone()])])))) {
    return Py::None;
    }
    said = _the_answer_in_hand(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_call(&[said.clone()]), 2, None);
    call = __unpk1[0].clone();
    person = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&person.clone(), &Py::None)) } }) {
    return { let __b = _the_condition_reply(&[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    named = mcall(&py_str(" "), "join", &[mcall(&mcall(&person.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    on_the_phone = Py::Bool(py_contains(&py_tuple(vec![py_str("999"), py_str("911"), py_str("111"), py_str("112")]), &mcall(&mcall(&named.clone(), "strip", &[py_str("the ")]), "lower", &[])));
    a_place = { let __b = on_the_phone.clone(); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_PLACE(), "match", &[named.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&a_place.clone()) {
    said_again = call.clone();
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AN_OBJECT_OF(), "search", &[slice_or_raise(&call.clone(), &Py::None, &mcall(&person.clone(), "start", &[]), &Py::None)])))) {
    said_again = call.clone();
    } else {
    after = slice_or_raise(&call.clone(), &mcall(&person.clone(), "end", &[]), &Py::None, &Py::None);
    place = stdlib_call("re", "match", &[py_str("\\s*(appointments?|visits?|checks?)\\b"), after.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&place.clone()) {
    noun = mcall(&place.clone(), "group", &[Py::Int(1)]);
    pronoun = if py_truthy(&mcall(&noun.clone(), "startswith", &[py_str("appointment")])) { py_str("an appointment") } else { add(&py_str("a "), &mcall(&noun.clone(), "rstrip", &[py_str("s")])) };
    said_again = mcall(&add(&add(&add(&mcall(&slice_or_raise(&call.clone(), &Py::None, &mcall(&person.clone(), "start", &[]), &Py::None), "rstrip", &[]), &py_str(" ")), &pronoun.clone()), &mcall(&slice_or_raise(&after.clone(), &mcall(&place.clone(), "end", &[]), &Py::None, &Py::None), "rstrip", &[])), "strip", &[]);
    } else {
    said_again = mcall(&add(&add(&mcall(&slice_or_raise(&call.clone(), &Py::None, &mcall(&person.clone(), "start", &[]), &Py::None), "rstrip", &[]), &py_str(" them ")), &mcall(&after.clone(), "lstrip", &[])), "strip", &[]);
    }
    }
    }
    pool = if py_truthy(&on_the_phone.clone()) { g_ON_THE_PHONE_LINES() } else { if py_truthy(&a_place.clone()) { g_PLACE_LINES() } else { g_SOMEONE_LINES() } };
    line = mcall(&crate::gen::compose::pick(&[pool.clone(), memory.clone(), call.clone()]), "replace", &[py_str("{person}"), named.clone()]);
    said_again = add(&add(&mcall(&slice_or_raise(&said_again.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said_again.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    return mcall(&mcall(&line.clone(), "replace", &[py_str("{call}"), said_again.clone()]), "replace", &[py_str("{call_lower}"), add(&mcall(&slice_or_raise(&said_again.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said_again.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn replies_to_a_who_to_see(args: &[Py]) -> Py {
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
    let _ = py_str("True when this file has the line, for the module that declares the claim.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[reply_for(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.who_to_see".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANYBODY" => Some(g_ANYBODY()),
            "AN_OBJECT_OF" => Some(g_AN_OBJECT_OF()),
            "ASK" => Some(g_ASK()),
            "A_LONG_CALL" => Some(g_A_LONG_CALL()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_SIGN_MEANS" => Some(g_A_SIGN_MEANS()),
            "A_SUBSET_OF_TIME" => Some(g_A_SUBSET_OF_TIME()),
            "CONDITION_LINES" => Some(g_CONDITION_LINES()),
            "ON_THE_PHONE_LINES" => Some(g_ON_THE_PHONE_LINES()),
            "PLACE_LINES" => Some(g_PLACE_LINES()),
            "SAYS_TO_DO" => Some(g_SAYS_TO_DO()),
            "SOMEONE_LINES" => Some(g_SOMEONE_LINES()),
            "_a_sentence_that_names_someone" => Some(py_func("_a_sentence_that_names_someone", |args: &[Py]| _a_sentence_that_names_someone(args))),
            "_a_sign_not_a_step" => Some(py_func("_a_sign_not_a_step", |args: &[Py]| _a_sign_not_a_step(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_the_answer_in_hand" => Some(py_func("_the_answer_in_hand", |args: &[Py]| _the_answer_in_hand(args))),
            "_the_call" => Some(py_func("_the_call", |args: &[Py]| _the_call(args))),
            "_the_condition" => Some(py_func("_the_condition", |args: &[Py]| _the_condition(args))),
            "_the_condition_reply" => Some(py_func("_the_condition_reply", |args: &[Py]| _the_condition_reply(args))),
            "_the_question" => Some(py_func("_the_question", |args: &[Py]| _the_question(args))),
            "replies_to_a_who_to_see" => Some(py_func("replies_to_a_who_to_see", |args: &[Py]| replies_to_a_who_to_see(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

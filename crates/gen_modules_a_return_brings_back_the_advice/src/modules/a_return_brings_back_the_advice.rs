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


thread_local! { static G_A_RECEIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBSTANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_return_brings_back_the_advice: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_RECEIPT() -> Py { ensure_init(); G_A_RECEIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECEIPT(v: Py) { ensure_init(); G_A_RECEIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBSTANCE() -> Py { ensure_init(); G_SUBSTANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBSTANCE(v: Py) { ensure_init(); G_SUBSTANCE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A return to a subject brings back what was said about it, not a question about where we were.\n\nA person can leave a subject for a turn or two and come back to it by its name alone:\n\n    my boiler is making a noise\n    how much is a new one\n    i rent so who pays\n    back to the noise          -> \"OK, back to the noise. Where were we?\"\n\nThe bot had the subject, what the person said about it and what it had answered, and it handed the\nquestion back. A friend asked to go back to the boiler says what they said about it the first time:\n\"Back to the noise: banging or rumbling when the heating starts is usually air or a pump set too\nfast, so bleed the radiators and check the pressure is 1 to 1.5 bar.\"\n\nNothing new is read here. The store's own reading (`bot/memory/topics.py`) has already noted that\nthis line goes back to an earlier topic (`memory[\"returned\"]`), and it has already moved the answer\nthat belongs to that topic to the top of the kept answers. This module adds the one thing the\nstore's pool of return lines does not have - the substance - and passes on every return with an ask\nof its own (\"back to the laptop, is it worth servicing?\"), because the module for the thing named\nanswers those with the question in front of it.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_return_brings_back_the_advice.PRIORITY"); }
    s_PRIORITY(Py::Int(7050));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_return_brings_back_the_advice.LEADS"); }
    s_LEADS(py_tuple(vec![py_str("Back to {thing}: {advice}"), py_str("Back on {thing}: {advice}"), py_str("On {thing}, then: {advice}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_return_brings_back_the_advice.A_RECEIPT"); }
    s_A_RECEIPT(stdlib_call("re", "compile", &[py_str("^(?:got it|noted|ok|okay|sure|right|you said|you've told me|you told me|that's right|that is right|makes sense|fair enough|understood|i'm not sure|i am not sure|that's beyond|that is beyond|no number for that|i can't|i cannot|i'd rather not|i would rather not|searching for it|a quick search)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_return_brings_back_the_advice.SUBSTANCE"); }
    s_SUBSTANCE(Py::Int(6));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_return_brings_back_the_advice.SENTENCE"); }
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_advice(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The sentence of the earlier answer that carries the advice, or \"\" when it has none.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[text.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(1), &Py::None);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(2), &Py::None);
    }
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn _the_answers_for(args: &[Py]) -> Py {
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
    let mut topic_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic_text"))) };
    let mut best_score: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut record: Py = Py::None;
    let mut score: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answers this chat gave for that subject, each one really said, oldest first.\n\n    A return goes back to the subject as it was first raised, and the answers that came after it\n    were about the questions that followed (\"what should i say\", \"how much is a new one\"). The\n    first answer for the subject is the one that says what the chat knows about it, and it is the\n    one a person who says \"back to the noise\" wants again. Only the answers that fit the subject\n    best are kept, and where two fit as well as each other the older one wins: a later answer is\n    about the question that followed (\"what about a turkey\") and it shares the word \"roast\".\n\n    Every record is checked against the turns, because the arbiter may have chosen another\n    module's reply and quoting an answer the person never saw would be inventing one.\n    ");
    kept = mcall(&memory.clone(), "get", &[py_str("entry_answers")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    kept = py_list(vec![mcall(&memory.clone(), "get", &[py_str("entry_answer")])]);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[topic_text.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![word.clone(), pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()])])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    let __aug1 = op_sub(&wanted.clone(), &pyrt::xrefs::xget("memory.topics", "STOPWORDS"));
    wanted = __aug1;
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return py_list(vec![]);
    }
    best_score = Py::Int(0);
    found = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&kept.clone());
    for __item in __iter2 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    continue;
    }
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&text.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(8), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&pyrt::xrefs::xget("memory.topics", "ANSWER_KEPT")), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "startswith", &[opening.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("set", &[{ let __b = mcall(&record.clone(), "get", &[py_str("keys")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) { let mut key: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![key.clone(), pyrt::xrefs::xcall("memory.topics", "stem", &[key.clone()])])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    score = call_builtin("len", &[op_bitand(&wanted.clone(), &op_sub(&forms.clone(), &pyrt::xrefs::xget("memory.topics", "STOPWORDS")))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk3 = py_unpack(&py_tuple(vec![score.clone(), py_list(vec![text.clone()])]), 2, None);
    best_score = __unpk3[0].clone();
    found = __unpk3[1].clone();
    } else {
    if py_truthy(&{ let __b = score.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&score.clone(), &best_score.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[text.clone()]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_turn_answers_for(args: &[Py]) -> Py {
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
    let mut topic_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic_text"))) };
    let mut best_score: Py = Py::None;
    let mut form: Py = Py::None;
    let mut found: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answers the chat gave on the turns about that subject, oldest first.\n\n    The kept answers are the entries' own replies and only the last few are held, so the answer to\n    the first turn about a subject can be gone by the time the person comes back to it: \"my boss\n    wants a meeting\" is followed by \"i hate meetings\" (small talk, not an entry) and \"what should i\n    say\", and the kept list holds only the last of the three. The transcript still has the turn the\n    subject was raised on, and its answer is what the chat said about it.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[topic_text.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![word.clone(), pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()])])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    let __aug1 = op_sub(&wanted.clone(), &pyrt::xrefs::xget("memory.topics", "STOPWORDS"));
    wanted = __aug1;
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return py_list(vec![]);
    }
    best_score = Py::Int(0);
    found = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&turns.clone());
    for __item in __iter2 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    reply = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&reply.clone())) } }) {
    continue;
    }
    score = call_builtin("len", &[op_bitand(&wanted.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[said.clone()])) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![word.clone(), pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()])])) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk3 = py_unpack(&py_tuple(vec![score.clone(), py_list(vec![reply.clone()])]), 2, None);
    best_score = __unpk3[0].clone();
    found = __unpk3[1].clone();
    } else {
    if py_truthy(&{ let __b = score.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&score.clone(), &best_score.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[reply.clone()]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _said_recently(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut advice: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "advice") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: advice"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut opening: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the chat has just said this, so bringing it back would be the same answer twice.\n\n    A return to a subject whose advice was the last thing said (\"my boss wants a meeting\" ... \"what\n    should i say\") has nothing new in it, and the arbiter passes over a reply that only says the\n    last turn again. Two turns back is enough: the answer to the turn the subject was raised on is\n    the source of the advice itself, and it is not a repeat to say it again later.\n    ");
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&advice.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(6), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "startswith", &[opening.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _has_substance(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut advice: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "advice") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: advice"))) };
    let _ = py_str("True when this is advice and not a receipt, a question or a shrug.");
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&advice.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SUBSTANCE())) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_RECEIPT(), "match", &[advice.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&mcall(&advice.clone(), "rstrip", &[]), "endswith", &[py_str("?")])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_return_advice(args: &[Py]) -> Py {
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
    let mut advice: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut record: Py = Py::None;
    let mut short: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut topic_text: Py = Py::None;
    let _ = py_str("The thing the person went back to and the advice we gave about it, or (\"\", \"\").");
    let __try1 = exc::catch(|| {
    record = pyrt::xrefs::xcall("memory.topics", "resumed", &[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&record.clone(), "get", &[py_str("asks")]) } }) {
    return py_tuple(vec![Py::Bool(true), py_tuple(vec![py_str(""), py_str("")])]);
    }
    thing = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("asked")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_tuple(vec![Py::Bool(true), py_tuple(vec![py_str(""), py_str("")])]);
    }
    topic_text = call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    advice = py_str("");
    answers = op_add(&op_add(&_the_answers_for(&[memory.clone(), topic_text.clone()]), &_the_turn_answers_for(&[memory.clone(), topic_text.clone()])), &py_list(vec![_the_advice(&[pyrt::xrefs::xcall("memory.topics", "entry_answer", &[memory.clone(), topic_text.clone()])])]));
    let __iter6 = py_iter_or_raise(&answers.clone());
    for __item in __iter6 {
    answer = __item.clone();
    short = _the_advice(&[answer.clone()]);
    if py_truthy(&{ let __b = _has_substance(&[short.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_recently(&[short.clone(), memory.clone()]))) } }) {
    advice = short.clone();
    break;
    }
    }
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
    return py_tuple(vec![py_str(""), py_str("")]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&_has_substance(&[advice.clone()])))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    return py_tuple(vec![thing.clone(), advice.clone()]);
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
    let mut advice: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The advice this chat gave about the subject the person went back to.");
    let __unpk1 = py_unpack(&_the_return_advice(&[memory.clone()]), 2, None);
    thing = __unpk1[0].clone();
    advice = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    lead = geti(&g_LEADS(), &op_modulo(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_LEADS()]).unwrap_or_else(|e| exc::raise(e))));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("advice".to_string(), advice.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) };
    Py::None
}
pub fn CLAIMS_INSTEAD(args: &[Py]) -> Py {
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
    let mut advice: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("This line is a bare return to a subject, and the advice is the answer to it.\n\n    The store answers the same lines from its pool of \"Where were we?\" lines as a claim - it read\n    the line as a return, which is right. The substance is the one thing its pool cannot have, so\n    the turn is asked for on the lines where the advice is in hand and nowhere else: a return that\n    carries an ask of its own is left to the module for the thing named, and a return to a subject\n    with no advice behind it keeps the store's question, which is the right reply there.\n    ");
    let __unpk1 = py_unpack(&_the_return_advice(&[memory.clone()]), 2, None);
    thing = __unpk1[0].clone();
    advice = __unpk1[1].clone();
    return call_builtin("bool", &[{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { advice.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_return_brings_back_the_advice".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_RECEIPT" => Some(g_A_RECEIPT()),
            "CLAIMS_INSTEAD" => Some(py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args))),
            "LEADS" => Some(g_LEADS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SUBSTANCE" => Some(g_SUBSTANCE()),
            "_has_substance" => Some(py_func("_has_substance", |args: &[Py]| _has_substance(args))),
            "_said_recently" => Some(py_func("_said_recently", |args: &[Py]| _said_recently(args))),
            "_the_advice" => Some(py_func("_the_advice", |args: &[Py]| _the_advice(args))),
            "_the_answers_for" => Some(py_func("_the_answers_for", |args: &[Py]| _the_answers_for(args))),
            "_the_return_advice" => Some(py_func("_the_return_advice", |args: &[Py]| _the_return_advice(args))),
            "_the_turn_answers_for" => Some(py_func("_the_turn_answers_for", |args: &[Py]| _the_turn_answers_for(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_RECEIPT" => s_A_RECEIPT(v),
            "LEADS" => s_LEADS(v),
            "PRIORITY" => s_PRIORITY(v),
            "SENTENCE" => s_SENTENCE(v),
            "SUBSTANCE" => s_SUBSTANCE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_return_brings_back_the_advice", module_obj);
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_the_advice", py_func("_the_advice", |args: &[Py]| _the_advice(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_the_answers_for", py_func("_the_answers_for", |args: &[Py]| _the_answers_for(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_the_turn_answers_for", py_func("_the_turn_answers_for", |args: &[Py]| _the_turn_answers_for(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_said_recently", py_func("_said_recently", |args: &[Py]| _said_recently(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_has_substance", py_func("_has_substance", |args: &[Py]| _has_substance(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "_the_return_advice", py_func("_the_return_advice", |args: &[Py]| _the_return_advice(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.a_return_brings_back_the_advice", "CLAIMS_INSTEAD", py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args)));
    pyrt::xrefs::reg_access("modules.a_return_brings_back_the_advice", "A_RECEIPT", g_A_RECEIPT, s_A_RECEIPT);
    pyrt::xrefs::reg_access("modules.a_return_brings_back_the_advice", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("modules.a_return_brings_back_the_advice", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_return_brings_back_the_advice", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("modules.a_return_brings_back_the_advice", "SUBSTANCE", g_SUBSTANCE, s_SUBSTANCE);
}

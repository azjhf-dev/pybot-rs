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


pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Is there a line of theirs that carries what this turn is about? One check, every reply.\n\nTwo failures look different and are the same fault. `shes 7 going on 8` was answered \"Got it\", and\nthe next turn `how old did i say she was` was answered **\"You haven't told me how old she is\"** -\nthe chat said it, and the store's own reader for that slot found nothing, so the store wrote a\ndenial over the top of it. On the other side `what size did i say` after `its a 2kg chicken`,\n`when did i say we fly` after `we fly on the 14th` and `how often did i say i go` after `i go to\nthe gym three times a week` were handed the search hedge, with the answer in the line above.\n\nOne question decides all of them: **which of their lines carries a value of the kind being asked\nabout?** It lives in `bot/memory/earlier.py`, once, and the kinds it knows are data rather than\nrules in each reader: an age, a name, a day, a size, a sum, how often, a length, a count, a place.\n\n`bot/modules/said_back.py` (priority 350) already asks the same question of the shapes people use\nto ask for something said earlier (`bot/memory/lineback.py`). This module is the same check at the\nfunnel, where it can also see the reply the store was about to send:\n\n* a reply that claims the person never said a thing, while a line of theirs carries it, is\n  replaced by their line handed back: \"You told me: \\\"shes 7 going on 8\\\".\";\n* a recall ask with nothing behind it (\"what size did i say\") is answered from the line that\n  carried the value;\n* anything else is a pass (None): the store's own answer stands word for word, and a line it\n  answers stays with whichever module answers it today.\n\nThe reply speaks for the store, so it declares the store's own topics (`ANSWERS`, read from\n`bot/memory/store_lines.py`): on a line about the family or a pet the routing lane would otherwise\nlift the store's candidate over this one by its topic, and the store's candidate is the denial\nthis module exists to replace.\n\nNothing is claimed that the chat does not hold: when no line carries the value, the store's reply\nstands exactly as it was, denial included, because then the denial is true.\n");
    s_PRIORITY(Py::Int(7050));
}
pub fn observe(args: &[Py]) -> Py {
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
    let _ = py_str("Nothing is kept here: the check reads the turns, it does not add to them.");
    return Py::None;
    Py::None
}
pub fn _the_store_would_say(args: &[Py]) -> Py {
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
    let _ = py_str("What the memory store answers this line, on this module's memory (so its notes are kept).");
    let mut memory_store: Py = crate::gen::modules::memory_store::module_obj();
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), mcall(&memory_store.clone(), "respond", &[text.clone(), memory.clone()])]);
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
pub fn _nothing_to_give(args: &[Py]) -> Py {
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
    let _ = py_str("True for the two replies that carry no answer: the search hedge and the hand-off.");
    let mut fallback: Py = crate::gen::compose::fallback::module_obj();
    let mut fit: Py = crate::gen::compose::fit::module_obj();
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = mcall(&fallback.clone(), "is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&fit.clone(), "is_a_punt", &[reply.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _someone_answers_it(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the knowledge base has a reply of its own for this line.\n\n    The store this module reads is only the pets-and-family one, so it has nothing to say about\n    most lines - and a module that took that silence as \"nobody can answer this\" talked over the\n    knowledge base on every question that carried a value of some kind: \"how often should i be\n    checking the gauge\" was told \"You told me: \"our boiler keeps dropping to zero on the gauge\"\"\n    instead of the boiler answer, and \"how many portions do i need for 6 people\" was handed the\n    guest line instead of the 1.8 kg. A real answer always outranks a quotation of an earlier line,\n    so this module only speaks where the knowledge base has nothing (a search hedge, a hand-off,\n    or a false \"you never said\").\n    ");
    let mut answers: Py = crate::gen::modules::answers::module_obj();
    let __try1 = exc::catch(|| {
    said = mcall(&answers.clone(), "respond", &[text.clone(), memory.clone()]);
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
    return call_builtin("bool", &[{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _another_module_gives_the_value(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a module that answers these shapes with the value itself already has an answer.\n\n    `attribute_said_again` answers \"did i say when we were going\" with \"You said October.\" - the\n    value alone, which is the answer a person wants. (`memory_store` said \"No - that's on the\n    Saturday.\", reading the ask's \"when\" off the wrong line.) Where that module has the value, its\n    answer is better than a quotation of the whole line, so this one passes.\n    ");
    let __try1 = exc::catch(|| {
    let mut attribute_said_again: Py = crate::gen::modules::attribute_said_again::module_obj();
    said = mcall(&attribute_said_again.clone(), "respond", &[text.clone(), memory.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[crate::gen::memory::earlier::gives_the_named_value(&[text.clone(), memory.clone(), said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_back_already(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the module that owns the ask-for-it-again shapes has an answer for this line.\n\n    It is below this one (350 against 7050), so its reply is not in the running when this module\n    speaks. Asking it first keeps every line it answers today answering exactly the same way.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[crate::gen::memory::lineback::reply_for(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let mut put: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("The store claims a recall ask as its own, which is right until it denies their own line.\n\n    `memory_store` declares that it answers a recall ask as it comes (\"what did i say her names\n    age was\"), and on a line like that the arbiter hands it the turn before PRIORITY is read: the\n    store's reader is the one that knows the line is a recall at all. So a reply that hands their\n    line back still lost the turn, and the denial was what the person got. This module takes the\n    turn only where it has read the whole line against their own words: the ask is a recall ask\n    (`did i say`, `remind me`, `what was`) and the store's own answer to it contradicts a line of\n    theirs. Everywhere else it says no, and the store's answer stands as it always did.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::memory::earlier::g_RECALL(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])])))) {
    return Py::Bool(false);
    }
    theirs = _the_store_would_say(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[theirs.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&theirs.clone(), "strip", &[]) } }))) {
    return Py::Bool(false);
    }
    put = { let __b = crate::gen::memory::earlier::put_right(&[theirs.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::earlier::a_misquoted_line(&[theirs.clone(), memory.clone(), text.clone()]) } };
    if py_truthy(&{ let __b = put.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&put.clone(), &theirs.clone())) } }) {
    return Py::Bool(true);
    }
    return crate::gen::memory::earlier::another_line_was_asked_for(&[text.clone(), memory.clone(), theirs.clone()]);
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
    let mut put: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("Their line handed back where the reply would deny it or miss it, else None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    theirs = _the_store_would_say(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[theirs.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&theirs.clone(), "strip", &[]) } }) {
    put = { let __b = crate::gen::memory::earlier::put_right(&[theirs.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::earlier::a_misquoted_line(&[theirs.clone(), memory.clone(), text.clone()]) } };
    if py_truthy(&put.clone()) {
    return put.clone();
    }
    if py_truthy(&{ let __b = crate::gen::memory::earlier::another_line_was_asked_for(&[text.clone(), memory.clone(), theirs.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_another_module_gives_the_value(&[text.clone(), memory.clone()]))) } }) {
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&_nothing_to_give(&[theirs.clone()])))) {
    return Py::None;
    }
    }
    }
    if py_truthy(&_said_back_already(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::memory::earlier::their_own_words(&[text.clone()]))); if !py_truthy(&__b) { __b } else { _someone_answers_it(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    return { let __b = crate::gen::memory::earlier::answer_for(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::earlier::from_the_line(&[text.clone(), memory.clone()]) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.said_fact_back".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_another_module_gives_the_value" => Some(py_func("_another_module_gives_the_value", |args: &[Py]| _another_module_gives_the_value(args))),
            "_nothing_to_give" => Some(py_func("_nothing_to_give", |args: &[Py]| _nothing_to_give(args))),
            "_said_back_already" => Some(py_func("_said_back_already", |args: &[Py]| _said_back_already(args))),
            "_someone_answers_it" => Some(py_func("_someone_answers_it", |args: &[Py]| _someone_answers_it(args))),
            "_the_store_would_say" => Some(py_func("_the_store_would_say", |args: &[Py]| _the_store_would_say(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

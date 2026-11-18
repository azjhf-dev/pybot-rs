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


thread_local! { static G_COUNT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHILE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHILE_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_stop_ask: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.keeping: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_COUNT_LINES() -> Py { ensure_init(); G_COUNT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_LINES(v: Py) { ensure_init(); G_COUNT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_LINES() -> Py { ensure_init(); G_KEEPING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_LINES(v: Py) { ensure_init(); G_KEEPING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_TAILS() -> Py { ensure_init(); G_KEEPING_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_TAILS(v: Py) { ensure_init(); G_KEEPING_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_STOP() -> Py { ensure_init(); G_THEIR_OWN_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_STOP(v: Py) { ensure_init(); G_THEIR_OWN_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_WHILE_LINES() -> Py { ensure_init(); G_WHILE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHILE_LINES(v: Py) { ensure_init(); G_WHILE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHILE_TAILS() -> Py { ensure_init(); G_WHILE_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHILE_TAILS(v: Py) { ensure_init(); G_WHILE_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNT() -> Py { ensure_init(); G__A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); G__A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_a_stop_ask() -> Py { ensure_init(); G_a_stop_ask.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_stop_ask(v: Py) { ensure_init(); G_a_stop_ask.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about this conversation itself is answered plainly, by the bot, in its own words.\n\n\"How long can we talk for\", \"is there a limit to how long we can talk\", \"have you got all day\" and\n\"how many messages can i send you\" ask how long the conversation can run, and \"am i boring you\",\n\"am i keeping you up\", \"am i talking too much\", \"do you want me to stop\" ask whether the person is\nbeing a nuisance. Both are questions only the bot can answer, and both were answered by something\nelse: the search hedge (\"I'd rather not guess - a quick search online will get you a solid\nanswer\"), the identity card (\"I'm a chatbot - here to help and chat\"), the message template (three\nlines to write to your landlord) and, for \"am i boring you\", the advice to try a jigsaw.\n\nThe reading is `bot/understanding/kinds.py`, which says which of the two the line is\n(`understood[\"about_bot_kind\"]`, `\"while\"` or `\"keeping\"`). Nothing here reads words itself: a line\nthat asks about a fridge, a bill, a tablet or a lunch break is not one of these, and the reading\nsays so before this module sees it.\n\nThe answer is short and honest. Nobody at this end watches a clock, so there is no limit to give;\nthe bot is following what the person says, so the answer to \"am i boring you\" is no, and it says\nhow it would tell them if it lost the thread. Then it hands the floor back.\n");
    s_a_stop_ask(py_func("a_stop_ask", |args: &[Py]| pyrt::xrefs::xcall("understanding.kinds", "a_stop_ask", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping._A_COUNT"); }
    s__A_COUNT(stdlib_call("re", "compile", &[py_str("\\bhow\\s+many\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.COUNT_LINES"); }
    s_COUNT_LINES(py_tuple(vec![py_str("As many as you like - there's no cap on messages here, so send them as they come."), py_str("No limit on messages: as many as you want, long or short."), py_str("Send as many as you like; there's no count I'm keeping.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.PRIORITY"); }
    s_PRIORITY(Py::Int(7400));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.WHILE_LINES"); }
    s_WHILE_LINES(py_tuple(vec![py_str("As long as you like — I'm not watching a clock, and nothing else is waiting on me. Say when you've had enough and I'll leave it there."), py_str("No limit I can see: there's nowhere else I need to be, so take the time you need. If you'd rather pick it up later, that's fine too."), py_str("All the time you want — I'm not counting lines or minutes, and I keep the thread while we talk. Say the word when you want to stop."), py_str("There's no clock on this at my end. Go at your own pace, and if you'd rather stop and come back to it, that's no trouble.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.KEEPING_LINES"); }
    s_KEEPING_LINES(py_tuple(vec![py_str("Not at all — carry on, I'm following. Say the rest of it."), py_str("No, and I'd say so if it got hard to follow. Keep going."), py_str("You're not boring me. Take as long as it needs."), py_str("Not a bit of it. Nothing else is happening at this end, so go on."), py_str("You're fine. I'm reading all of it, and I'll ask if I lose the thread.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.WHILE_TAILS"); }
    s_WHILE_TAILS(py_tuple(vec![py_str(""), py_str("Where were we?"), py_str("What were you in the middle of?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.KEEPING_TAILS"); }
    s_KEEPING_TAILS(py_tuple(vec![py_str("")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.keeping.THEIR_OWN_STOP"); }
    s_THEIR_OWN_STOP(stdlib_call("re", "compile", &[py_str("(?:should|shall|can|could|may)\\s+(?:i|we)\\s+(?:stop|quit|give\\s+up|pack\\s+it\\s+in|leave\\s+it|jack\\s+it\\s+in)(?:\\s+(?:now|then|it|this|that|doing\\s+(?:it|this|that)))?\\s*[.!?]*")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _seed(args: &[Py]) -> Py {
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
    let _ = py_str("A number that stands for this line, so the same words always get the same reply.");
    return mcall(&py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e))), "from_bytes", &[slice_or_raise(&mcall(&stdlib_call("hashlib", "sha1", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "encode", &[py_str("utf-8")])]).unwrap_or_else(|e| exc::raise(e)), "digest", &[]), &Py::None, &Py::Int(6), &Py::None), py_str("big")]);
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("One line, chosen by the line itself, so the same words get the same reply.");
    return geti(&lines.clone(), &op_modulo(&_seed(&[text.clone()]), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_bare_stop_about_a_thing(args: &[Py]) -> Py {
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
    let mut topic: Py = Py::None;
    let _ = py_str("True when a bare \"should i stop\" is about the thing the chat is on, not about the chat.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THEIR_OWN_STOP(), "fullmatch", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("understanding.kinds", "a_stop_ask", &[said.clone()])))) {
    return Py::Bool(false);
    }
    topic = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return call_builtin("bool", &[mcall(&call_builtin("str", &[{ let __b = mcall(&topic.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut kind: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    kind = mcall(&understood.clone(), "get", &[py_str("about_bot_kind")]);
    return if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("while"), py_str("keeping")]), &kind.clone()))) { kind.clone() } else { Py::None };
    Py::None
}
pub fn _already_said(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the bot said last, so the same sentence is not repeated to a person who asks twice.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_str("");
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
    else { Py::None };
    let mut understood: Py = Py::None;
    let _ = py_str("This line is answered as it comes: nobody else can answer a question about this chat.\n\n    The store takes these lines itself (\"do i talk too much\" was read as a thing called \"too\n    much\", and answered \"There's nothing on too much in what I keep\"), and a take is settled by\n    PRIORITY: this module sits above the store so the claim is its own.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    return Py::Bool(!py_is(&_the_kind(&[understood.clone()]), &Py::None));
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
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut order: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut tails: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The reply for a question about how long this chat can go, or whether they are a nuisance.");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    kind = _the_kind(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("keeping"))); if !py_truthy(&__b) { __b } else { _a_bare_stop_about_a_thing(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    said = _already_said(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("while"))); if !py_truthy(&__b) { __b } else { mcall(&g__A_COUNT(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![g_COUNT_LINES(), py_tuple(vec![py_str("")])]), 2, None);
    lines = __unpk1[0].clone();
    tails = __unpk1[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("while")))) {
    let __unpk2 = py_unpack(&py_tuple(vec![g_WHILE_LINES(), g_WHILE_TAILS()]), 2, None);
    lines = __unpk2[0].clone();
    tails = __unpk2[1].clone();
    } else {
    let __unpk3 = py_unpack(&py_tuple(vec![g_KEEPING_LINES(), g_KEEPING_TAILS()]), 2, None);
    lines = __unpk3[0].clone();
    tails = __unpk3[1].clone();
    }
    }
    order = _rotate(&[lines.clone(), text.clone()]);
    let __iter4 = py_iter_or_raise(&order.clone());
    let mut __else5 = false;
    for __item in __iter4 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &line.clone()))) {
    break;
    }
    }
    line = geti(&order.clone(), &Py::Int(0));
    tail = _pick(&[tails.clone(), op_add(&text.clone(), &py_str(" tail"))]);
    return mcall(&op_add(&line.clone(), &if py_truthy(&tail.clone()) { op_add(&py_str(" "), &tail.clone()) } else { py_str("") }), "strip", &[]);
    Py::None
}
pub fn _rotate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut at: Py = Py::None;
    let _ = py_str("The pool in an order that starts where this line's digest points.");
    at = op_modulo(&_seed(&[text.clone()]), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return op_add(&call_builtin("tuple", &[slice_or_raise(&lines.clone(), &at.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[slice_or_raise(&lines.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.keeping".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "COUNT_LINES" => Some(g_COUNT_LINES()),
            "KEEPING_LINES" => Some(g_KEEPING_LINES()),
            "KEEPING_TAILS" => Some(g_KEEPING_TAILS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "THEIR_OWN_STOP" => Some(g_THEIR_OWN_STOP()),
            "WHILE_LINES" => Some(g_WHILE_LINES()),
            "WHILE_TAILS" => Some(g_WHILE_TAILS()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_a_bare_stop_about_a_thing" => Some(py_func("_a_bare_stop_about_a_thing", |args: &[Py]| _a_bare_stop_about_a_thing(args))),
            "_already_said" => Some(py_func("_already_said", |args: &[Py]| _already_said(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_rotate" => Some(py_func("_rotate", |args: &[Py]| _rotate(args))),
            "_seed" => Some(py_func("_seed", |args: &[Py]| _seed(args))),
            "_the_kind" => Some(py_func("_the_kind", |args: &[Py]| _the_kind(args))),
            "a_stop_ask" => Some(g_a_stop_ask()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "hashlib" => Some(stdlib_module("hashlib")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "COUNT_LINES" => s_COUNT_LINES(v),
            "KEEPING_LINES" => s_KEEPING_LINES(v),
            "KEEPING_TAILS" => s_KEEPING_TAILS(v),
            "PRIORITY" => s_PRIORITY(v),
            "THEIR_OWN_STOP" => s_THEIR_OWN_STOP(v),
            "WHILE_LINES" => s_WHILE_LINES(v),
            "WHILE_TAILS" => s_WHILE_TAILS(v),
            "_A_COUNT" => s__A_COUNT(v),
            "a_stop_ask" => s_a_stop_ask(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.keeping", module_obj);
    pyrt::xrefs::reg_fn("modules.keeping", "_seed", py_func("_seed", |args: &[Py]| _seed(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "_a_bare_stop_about_a_thing", py_func("_a_bare_stop_about_a_thing", |args: &[Py]| _a_bare_stop_about_a_thing(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "_the_kind", py_func("_the_kind", |args: &[Py]| _the_kind(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "_already_said", py_func("_already_said", |args: &[Py]| _already_said(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.keeping", "_rotate", py_func("_rotate", |args: &[Py]| _rotate(args)));
    pyrt::xrefs::reg_access("modules.keeping", "COUNT_LINES", g_COUNT_LINES, s_COUNT_LINES);
    pyrt::xrefs::reg_access("modules.keeping", "KEEPING_LINES", g_KEEPING_LINES, s_KEEPING_LINES);
    pyrt::xrefs::reg_access("modules.keeping", "KEEPING_TAILS", g_KEEPING_TAILS, s_KEEPING_TAILS);
    pyrt::xrefs::reg_access("modules.keeping", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.keeping", "THEIR_OWN_STOP", g_THEIR_OWN_STOP, s_THEIR_OWN_STOP);
    pyrt::xrefs::reg_access("modules.keeping", "WHILE_LINES", g_WHILE_LINES, s_WHILE_LINES);
    pyrt::xrefs::reg_access("modules.keeping", "WHILE_TAILS", g_WHILE_TAILS, s_WHILE_TAILS);
    pyrt::xrefs::reg_access("modules.keeping", "_A_COUNT", g__A_COUNT, s__A_COUNT);
    pyrt::xrefs::reg_access("modules.keeping", "a_stop_ask", g_a_stop_ask, s_a_stop_ask);
}

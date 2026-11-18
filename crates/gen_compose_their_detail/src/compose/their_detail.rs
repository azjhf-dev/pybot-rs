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


thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONSIDERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.their_detail: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONSIDERING() -> Py { ensure_init(); G_CONSIDERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONSIDERING(v: Py) { ensure_init(); G_CONSIDERING.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_WORD() -> Py { ensure_init(); G_DIET_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_WORD(v: Py) { ensure_init(); G_DIET_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_WORDS() -> Py { ensure_init(); G_FOOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_WORDS(v: Py) { ensure_init(); G_FOOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING() -> Py { ensure_init(); G_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING(v: Py) { ensure_init(); G_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH() -> Py { ensure_init(); G_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH(v: Py) { ensure_init(); G_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER() -> Py { ensure_init(); G_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); G_NUMBER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A detail the person adds to the subject is answered by the question it implies.\n\nA conversation goes: the bot answers, and the person adds a thing about it rather than asking\nagain.\n\n    **User:** im doing the food for a party\n    **Bot:** ...\n    **User:** theres eight of us at home\n    **Bot:** Got it - there's eight of you at home.\n\n    **User:** im sorting a trip for the summer\n    **User:** were off to rome in september\n    **Bot:** Got it - you are off to rome in september.\n\nThere is no question in the line, so nothing looks anything up, and the store says the line back.\nThe knowledge base does answer the question the detail implies, and it answers it well:\n\n    \"how much food for eight people\"                       -> about 300 g each, 2.4 kg for eight\n    \"what should i know before a trip to rome in september\" -> the Colosseum with a timed ticket\n    \"what can i cook for a vegetarian\"                      -> chickpea curry, in 25 minutes\n    \"what do i need to know before buying an electric blanket\" -> 20 minutes before bed\n\nSo this reads the kind of detail, writes the question that kind implies out of the person's own\nwords, and puts it to the knowledge base. Nothing is invented: every word of the reply comes from\na card that the implied question reached. If the knowledge base has nothing for the implied\nquestion the module returns None and the line keeps whatever the composer had.\n\nIt runs where nothing better does: it replaces a receipt only, in greeting, which is what said\nthese lines back. A card, an entry or a light remark keeps its own turn.\n");
    s_ASKS(stdlib_call("re", "compile", &[py_str("^\\W*(?:how|what|whats|what's|which|when|where|who|why)\\b|^\\W*(?:is|are|was|were|do|does|did|can|could|should|would|will|have|has|had|am)\\s+(?:i|you|we|they|it|that|this|there|he|she|any|my|your|his|her|their|the|a|an)\\b|\\b(?:what|how|which|who|when|where) about\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DIET_WORD(stdlib_call("re", "compile", &[py_str("\\b(?P<diet>vegan|vegetarian|coeliac|celiac|dairy[- ]free|gluten[- ]free|pescatarian)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT(stdlib_call("re", "compile", &[py_str("\\b(?:there(?:'?s| is| are)\\s+)?(?P<n>\\d+|one|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:of us|people|adults|kids|children|guests|coming|more|extra|others|mouths)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER(py_dict(vec![(py_str("one"), py_str("one")), (py_str("two"), py_str("two")), (py_str("three"), py_str("three")), (py_str("four"), py_str("four")), (py_str("five"), py_str("five")), (py_str("six"), py_str("six")), (py_str("seven"), py_str("seven")), (py_str("eight"), py_str("eight")), (py_str("nine"), py_str("nine")), (py_str("ten"), py_str("ten"))]));
    s_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:off|going|heading|away|flying|driving|travelling|traveling)\\s+to\\s+(?P<place>[a-z][a-z' -]{2,24}?)(?:\\s+(?:in|on|for|next|at)\\b|$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH(stdlib_call("re", "compile", &[py_str("\\b(january|february|march|april|may|june|july|august|september|october|november|december|spring|summer|autumn|winter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONSIDERING(stdlib_call("re", "compile", &[py_str("\\b(?:looking at|thinking (?:of|about)|considering|about to (?:buy|get|order)|going to (?:buy|get|order)|planning (?:to get|on getting))\\s+(?:an\\s+|the\\s+|some\\s+|a\\s+)?(?P<thing>[a-z][a-z' -]{2,30}?)(?:\\s+(?:for|in|because|to|but)\\b|[.,!?]|$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:food|cook|cooking|cooked|eat|eats|eating|dish|recipe|meal|meals|menu|portion|portions|serves|serving|dinner|roast|bake|baking|ingredient|ingredients|cheese|chicken|pasta|rice|soup|salad)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _about_food(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the turn above was about food, which is what a headcount is a detail of.");
    said = py_list(vec![]);
    last = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("last_answer")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&last.clone(), "get", &[py_str("reply")]) } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&last.clone(), &py_str("reply"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e))]);
    break;
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&g_FOOD_WORDS(), "search", &[mcall(&one.clone(), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut month: Py = Py::None;
    let _ = py_str("The question this detail implies, written out of the line's own words, or None.");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("dietword")))) {
    return op_modulo(&py_str("what can i cook for a %s"), &detail.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("count")))) {
    if py_truthy(&Py::Bool(py_eq(&detail.clone(), &py_str("one more")))) {
    return py_str("how much food for one more person");
    }
    return op_modulo(&py_str("how much food for %s people"), &detail.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("going")))) {
    month = mcall(&g_MONTH(), "search", &[line.clone()]);
    if py_truthy(&month.clone()) {
    return op_modulo(&py_str("what should i know before a trip to %s in %s"), &py_tuple(vec![detail.clone(), mcall(&mcall(&month.clone(), "group", &[Py::Int(1)]), "lower", &[])]));
    }
    return op_modulo(&py_str("what should i know before a trip to %s"), &detail.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("considering")))) {
    return op_modulo(&py_str("what do i need to know before buying %s"), &detail.clone());
    }
    return Py::None;
    Py::None
}
pub fn _the_read(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The kind of detail this line adds, and the words it names, or (None, None).");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_ASKS(), "search", &[geti(&mcall(&clean.clone(), "split", &[py_str(",")]), &Py::Int(0))]) } } } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    match_ = mcall(&g_DIET_WORD(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    return py_tuple(vec![py_str("dietword"), mcall(&mcall(&match_.clone(), "group", &[py_str("diet")]), "lower", &[])]);
    }
    match_ = mcall(&g_COUNT(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    number = mcall(&mcall(&match_.clone(), "group", &[py_str("n")]), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:more|extra|another)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![py_str("count"), py_str("one more")]);
    }
    return py_tuple(vec![py_str("count"), mcall(&g_NUMBER(), "get", &[number.clone(), number.clone()])]);
    }
    match_ = mcall(&g_GOING(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    return py_tuple(vec![py_str("going"), mcall(&mcall(&match_.clone(), "group", &[py_str("place")]), "strip", &[])]);
    }
    match_ = mcall(&g_CONSIDERING(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    thing = stdlib_call("re", "sub", &[py_str("^(?:(?:buying|getting|ordering)\\s+)?(?:a|an|the)\\s+"), py_str(""), mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![py_str("considering"), thing.clone()]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _a_real_answer(args: &[Py]) -> Py {
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
    let _ = py_str("True when the knowledge base answered rather than hedging or filing the line.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.fallback", "_is_a_hedge", &[said.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[said.clone(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "NEUTRAL")])]) } }));
    Py::None
}
pub fn _said_already(args: &[Py]) -> Py {
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
    let _ = py_str("Every line the bot has said in this conversation, in its flat form.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    let _ = mcall(&said.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[mcall(&turn.clone(), "get", &[py_str("bot")])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    }
    }
    return said.clone();
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
    let mut answers: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The knowledge base's answer to the question this detail implies, or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __unpk1 = py_unpack(&_the_read(&[clean.clone()]), 2, None);
    kind = __unpk1[0].clone();
    detail = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("count"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_food(&[memory.clone()]))) } }) {
    return Py::None;
    }
    question = _asked(&[kind.clone(), detail.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&question.clone()))) {
    return Py::None;
    }
    answers = pyrt::xrefs::xmodule("modules.answers");
    let __try2 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[question.clone(), py_dict(vec![])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    return Py::None;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&Py::Bool(!py_truthy(&_a_real_answer(&[reply.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("going"), py_str("considering")]), &kind.clone()))) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{4,}"), detail.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_list(__out) })() };
    said = mcall(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(py_contains(&_said_already(&[memory.clone()]), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])))) {
    return Py::None;
    }
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.their_detail".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "CONSIDERING" => Some(g_CONSIDERING()),
            "COUNT" => Some(g_COUNT()),
            "DIET_WORD" => Some(g_DIET_WORD()),
            "FOOD_WORDS" => Some(g_FOOD_WORDS()),
            "GOING" => Some(g_GOING()),
            "MONTH" => Some(g_MONTH()),
            "NUMBER" => Some(g_NUMBER()),
            "_a_real_answer" => Some(py_func("_a_real_answer", |args: &[Py]| _a_real_answer(args))),
            "_about_food" => Some(py_func("_about_food", |args: &[Py]| _about_food(args))),
            "_asked" => Some(py_func("_asked", |args: &[Py]| _asked(args))),
            "_said_already" => Some(py_func("_said_already", |args: &[Py]| _said_already(args))),
            "_the_read" => Some(py_func("_the_read", |args: &[Py]| _the_read(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS" => s_ASKS(v),
            "CONSIDERING" => s_CONSIDERING(v),
            "COUNT" => s_COUNT(v),
            "DIET_WORD" => s_DIET_WORD(v),
            "FOOD_WORDS" => s_FOOD_WORDS(v),
            "GOING" => s_GOING(v),
            "MONTH" => s_MONTH(v),
            "NUMBER" => s_NUMBER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.their_detail", module_obj);
    pyrt::xrefs::reg_fn("compose.their_detail", "_about_food", py_func("_about_food", |args: &[Py]| _about_food(args)));
    pyrt::xrefs::reg_fn("compose.their_detail", "_asked", py_func("_asked", |args: &[Py]| _asked(args)));
    pyrt::xrefs::reg_fn("compose.their_detail", "_the_read", py_func("_the_read", |args: &[Py]| _the_read(args)));
    pyrt::xrefs::reg_fn("compose.their_detail", "_a_real_answer", py_func("_a_real_answer", |args: &[Py]| _a_real_answer(args)));
    pyrt::xrefs::reg_fn("compose.their_detail", "_said_already", py_func("_said_already", |args: &[Py]| _said_already(args)));
    pyrt::xrefs::reg_fn("compose.their_detail", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.their_detail", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.their_detail", "CONSIDERING", g_CONSIDERING, s_CONSIDERING);
    pyrt::xrefs::reg_access("compose.their_detail", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("compose.their_detail", "DIET_WORD", g_DIET_WORD, s_DIET_WORD);
    pyrt::xrefs::reg_access("compose.their_detail", "FOOD_WORDS", g_FOOD_WORDS, s_FOOD_WORDS);
    pyrt::xrefs::reg_access("compose.their_detail", "GOING", g_GOING, s_GOING);
    pyrt::xrefs::reg_access("compose.their_detail", "MONTH", g_MONTH, s_MONTH);
    pyrt::xrefs::reg_access("compose.their_detail", "NUMBER", g_NUMBER, s_NUMBER);
}

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


thread_local! { static G_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_BED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_DOORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_TOGETHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.plan_ask: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PLAN_ASK() -> Py { ensure_init(); G_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASK(v: Py) { ensure_init(); G_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_BED() -> Py { ensure_init(); G_PLAN_BED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_BED(v: Py) { ensure_init(); G_PLAN_BED.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_BOOKED() -> Py { ensure_init(); G_PLAN_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_BOOKED(v: Py) { ensure_init(); G_PLAN_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_DOORS() -> Py { ensure_init(); G_PLAN_DOORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_DOORS(v: Py) { ensure_init(); G_PLAN_DOORS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_EMPTY() -> Py { ensure_init(); G_PLAN_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_EMPTY(v: Py) { ensure_init(); G_PLAN_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_MEAL() -> Py { ensure_init(); G_PLAN_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_MEAL(v: Py) { ensure_init(); G_PLAN_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_PLACE() -> Py { ensure_init(); G_PLAN_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_PLACE(v: Py) { ensure_init(); G_PLAN_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_QUESTION() -> Py { ensure_init(); G_PLAN_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_QUESTION(v: Py) { ensure_init(); G_PLAN_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_TOGETHER() -> Py { ensure_init(); G_PLAN_TOGETHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_TOGETHER(v: Py) { ensure_init(); G_PLAN_TOGETHER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A plan asked for when the thread already holds one.\n\nThe reader and its patterns live here rather than in `fallback.py` so that the file every lane\nadds an ending to keeps one line of it, and the walk back over the thread is somewhere a reader\ncan find it.\n");
    s_PLAN_ASK(stdlib_call("re", "compile", &[py_str("(?:whats? (?:the |our )?plan|what is the plan|the plan for (?:the|our|this) (?:day|trip|weekend|evening|visit)|what (?:should|shall|do) we do (?:first|next|then)|where (?:do|should) we start|whats? the order|how (?:should|do|shall) we plan|plan (?:out )?the (?:day|trip|weekend)|sort (?:out )?the plan)(?=\\s*(?:$|[.?!]|(?:then|now|today|tonight|tomorrow|weekend)\\b|for (?:the|this|our|next|\\w+day)\\b|on \\w+day\\b))")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:gig|gigs|venue|concert|show|match|game|festival|tour|theatre|cinema|museum|exhibition|station|train|flight|airport|coach|trip|day out|weekend away|night out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_MEAL(stdlib_call("re", "compile", &[py_str("\\b(?:eat|eats|eating|food|meal|dinner|lunch|breakfast|restaurant|table|bite|pub|pizza|curry)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_BED(stdlib_call("re", "compile", &[py_str("\\b(?:stay over|stay the night|staying over|hotel|hotels|bed|room|sleep|sleeping|airbnb|hostel)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_BOOKED(stdlib_call("re", "compile", &[py_str("\\b(?:booked|reserved|sorted|got a table|have a table|already got)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_TOGETHER(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|two|three|four|five|six|seven|eight|several|few|both) of (?:us|them|you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_DOORS(stdlib_call("re", "compile", &[py_str("\\b(?:gig|gigs|venue|concert|show|match|game|festival|tour|theatre|cinema|museum|exhibition)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_QUESTION(stdlib_call("re", "compile", &[py_str("^(?:so |ok |okay |and |but |then )*(?:what|why|how|when|where|who|which)\\b|^(?:so |ok |okay |and |but |then )*(?:can|could|should|shall|do|does|did|is|are|was|were|will|would|have|has|am|any)\\s+(?:i|you|we|they|he|she|it|there|this|that|one)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_EMPTY(call_builtin("frozenset", &[mcall(&py_str("\nhi hello hey ok okay cool nice great thanks thank you yes yeah no nope sure right ah oh hmm\nlol haha good fine maybe alright\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_statement(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line, if it says how the thing will be, with nothing of the plan machinery in it.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    said = mcall(&said.clone(), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_PLAN_QUESTION(), "match", &[said.clone()]) } } } }) {
    return py_str("");
    }
    words = mcall(&said.clone(), "split", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)); if py_truthy(&__b) { __b } else { op_le(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PLAN_EMPTY()) } } } }) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _the_plan_the_thread_holds(args: &[Py]) -> Py {
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
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_str("") };
    let mut bed: Py = Py::None;
    let mut book: Py = Py::None;
    let mut booked: Py = Py::None;
    let mut meal: Py = Py::None;
    let mut one: Py = Py::None;
    let mut order: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut t: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut thread: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The thing the plan is for, and up to three things the person has settled about it.\n\n    A plan asked for as a line on its own - \"so whats the plan for the day then\" - cannot be\n    answered out of a file: the plan is made of what this person has already said, four turns of\n    it, and every part of it is in the thread. What a friend does is put the order round it (the\n    hour that cannot move, the meal, the bed) and say back what has been settled, so the person\n    can see the plan is nearly all decided and only the times are left. So this reads the thread\n    rather than the line: the first thing said is what the plan is for, and the last three\n    statements are the constraints it has to fit.\n\n    It also has to be a day out: a gig, a venue, a train, a hotel. A move or a party is a plan\n    with a different order, and the day-out order said over one of those is a confident wrong\n    answer, so one of the day-out parts has to be named before this reads anything.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    ask = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = ask.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut t: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &ask.clone()))) {  __out.push(_a_statement(&[mcall(&t.clone(), "get", &[py_str("user")])])); } } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    thread = mcall(&py_str(" "), "join", &[said.clone()]);
    parts = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![g_PLAN_PLACE(), g_PLAN_MEAL(), g_PLAN_BED(), g_PLAN_TOGETHER()])) { let mut one: Py = __item.clone(); __out.push(call_builtin("bool", &[mcall(&one.clone(), "search", &[thread.clone()])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_lt(&parts.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_PLAN_PLACE(), "search", &[thread.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLAN_BED(), "search", &[thread.clone()]) } })) } }) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    thing = geti(&said.clone(), &Py::Int(0));
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("bool", &[mcall(&g_PLAN_MEAL(), "search", &[thread.clone()])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("bool", &[mcall(&g_PLAN_BED(), "search", &[thread.clone()])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    meal = __unpk1[0].clone();
    bed = __unpk1[1].clone();
    booked = call_builtin("bool", &[mcall(&g_PLAN_BOOKED(), "search", &[thread.clone()])]).unwrap_or_else(|e| exc::raise(e));
    settled = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&settled.clone(), &one.clone()))) {
    continue;
    }
    let _ = mcall(&settled.clone(), "append", &[one.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[settled.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    let _ = mcall(&settled.clone(), "reverse", &[]);
    order = py_str("the hour that cannot move");
    if py_truthy(&meal.clone()) {
    let __aug3 = op_add(&order.clone(), &py_str(", then the meal"));
    order = __aug3;
    }
    if py_truthy(&bed.clone()) {
    let __aug4 = op_add(&order.clone(), &py_str(", then the bed"));
    order = __aug4;
    }
    book = if py_truthy(&{ let __b = meal.clone(); if !py_truthy(&__b) { __b } else { bed.clone() } }) { py_str("the table and the room") } else { if py_truthy(&meal.clone()) { py_str("the table") } else { if py_truthy(&bed.clone()) { py_str("the room") } else { py_str("the two bookings") } } };
    return py_tuple(vec![thing.clone(), settled.clone(), order.clone(), book.clone(), booked.clone()]);
    Py::None
}
pub fn _quoted_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut settled: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "settled") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: settled"))) };
    let mut one: Py = Py::None;
    let mut quoted: Py = Py::None;
    let _ = py_str("The lines said back in the person's own words, joined the way a person says a list.");
    quoted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&settled.clone()) { let mut one: Py = __item.clone(); __out.push(op_modulo(&py_str("\"%s\""), &one.clone())); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[quoted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&quoted.clone(), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[quoted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_modulo(&py_str("%s and %s"), &py_tuple(vec![geti(&quoted.clone(), &Py::Int(0)), geti(&quoted.clone(), &Py::Int(1))]));
    }
    return op_modulo(&py_str("%s and %s"), &py_tuple(vec![mcall(&py_str(", "), "join", &[slice_or_raise(&quoted.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), geti(&quoted.clone(), &op_neg(&Py::Int(1)))]));
    Py::None
}
pub fn plan_ask_pool(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut _plan: Py = Py::None;
    let mut book: Py = Py::None;
    let mut booked: Py = Py::None;
    let mut close: Py = Py::None;
    let mut left: Py = Py::None;
    let mut named: Py = Py::None;
    let mut order: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The pool for \"what's the plan\" when the thread already holds one, else None.\n\n    Every part of a day out is in the thread - three of them, eating first, staying over, cheap\n    and cheerful - and the line that asks for the plan of it was answered \"That's beyond what I\n    know well, sorry. Searching for it will turn up better sources\": the one thing nobody can\n    look up is their own day. The order is not knowledge, it is how a day works, and the\n    constraints are the person's own words, so the bot can lay both out and hand back the single\n    thing it cannot know: the hour the doors open.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PLAN_ASK(), "search", &[said.clone()])))) {
    return Py::None;
    }
    _plan = _the_plan_the_thread_holds(&[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&geti(&_plan.clone(), &Py::Int(1))))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_plan.clone(), 5, None);
    thing = __unpk1[0].clone();
    settled = __unpk1[1].clone();
    order = __unpk1[2].clone();
    book = __unpk1[3].clone();
    booked = __unpk1[4].clone();
    named = _quoted_back(&[op_add(&py_list(vec![thing.clone()]), &call_builtin("list", &[settled.clone()]).unwrap_or_else(|e| exc::raise(e)))]);
    said_back = named.clone();
    left = if py_truthy(&mcall(&g_PLAN_DOORS(), "search", &[op_add(&op_add(&mcall(&py_str(" "), "join", &[settled.clone()]), &py_str(" ")), &thing.clone())])) { py_str("the doors time - the venue's own page has that") } else { py_str("the hour you have to be there - the booking page has that") };
    if py_truthy(&booked.clone()) {
    tail = py_str("The one thing that runs out is done already, so the hours are all that is left.");
    close = py_str("Fix the hours round it and the day runs itself.");
    } else {
    tail = op_modulo(&py_str("Book %s tonight: the rest is only times."), &book.clone());
    close = op_modulo(&py_str("Get %s settled and the day runs itself."), &book.clone());
    }
    return py_tuple(vec![op_modulo(&py_str("You have told me %s, so the order is %s. %s"), &py_tuple(vec![said_back.clone(), order.clone(), tail.clone()])), op_modulo(&py_str("I have it all: %s. The order is %s, and the one thing left to find is %s."), &py_tuple(vec![said_back.clone(), order.clone(), left.clone()])), op_modulo(&py_str("Nothing here needs looking up - you have told me %s, and that is a day out with the order already in it: %s. %s"), &py_tuple(vec![said_back.clone(), order.clone(), close.clone()]))]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.plan_ask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PLAN_ASK" => Some(g_PLAN_ASK()),
            "PLAN_BED" => Some(g_PLAN_BED()),
            "PLAN_BOOKED" => Some(g_PLAN_BOOKED()),
            "PLAN_DOORS" => Some(g_PLAN_DOORS()),
            "PLAN_EMPTY" => Some(g_PLAN_EMPTY()),
            "PLAN_MEAL" => Some(g_PLAN_MEAL()),
            "PLAN_PLACE" => Some(g_PLAN_PLACE()),
            "PLAN_QUESTION" => Some(g_PLAN_QUESTION()),
            "PLAN_TOGETHER" => Some(g_PLAN_TOGETHER()),
            "_a_statement" => Some(py_func("_a_statement", |args: &[Py]| _a_statement(args))),
            "_quoted_back" => Some(py_func("_quoted_back", |args: &[Py]| _quoted_back(args))),
            "_the_plan_the_thread_holds" => Some(py_func("_the_plan_the_thread_holds", |args: &[Py]| _the_plan_the_thread_holds(args))),
            "plan_ask_pool" => Some(py_func("plan_ask_pool", |args: &[Py]| plan_ask_pool(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PLAN_ASK" => s_PLAN_ASK(v),
            "PLAN_BED" => s_PLAN_BED(v),
            "PLAN_BOOKED" => s_PLAN_BOOKED(v),
            "PLAN_DOORS" => s_PLAN_DOORS(v),
            "PLAN_EMPTY" => s_PLAN_EMPTY(v),
            "PLAN_MEAL" => s_PLAN_MEAL(v),
            "PLAN_PLACE" => s_PLAN_PLACE(v),
            "PLAN_QUESTION" => s_PLAN_QUESTION(v),
            "PLAN_TOGETHER" => s_PLAN_TOGETHER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.plan_ask", module_obj);
    pyrt::xrefs::reg_fn("compose.plan_ask", "_a_statement", py_func("_a_statement", |args: &[Py]| _a_statement(args)));
    pyrt::xrefs::reg_fn("compose.plan_ask", "_the_plan_the_thread_holds", py_func("_the_plan_the_thread_holds", |args: &[Py]| _the_plan_the_thread_holds(args)));
    pyrt::xrefs::reg_fn("compose.plan_ask", "_quoted_back", py_func("_quoted_back", |args: &[Py]| _quoted_back(args)));
    pyrt::xrefs::reg_fn("compose.plan_ask", "plan_ask_pool", py_func("plan_ask_pool", |args: &[Py]| plan_ask_pool(args)));
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_ASK", g_PLAN_ASK, s_PLAN_ASK);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_BED", g_PLAN_BED, s_PLAN_BED);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_BOOKED", g_PLAN_BOOKED, s_PLAN_BOOKED);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_DOORS", g_PLAN_DOORS, s_PLAN_DOORS);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_EMPTY", g_PLAN_EMPTY, s_PLAN_EMPTY);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_MEAL", g_PLAN_MEAL, s_PLAN_MEAL);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_PLACE", g_PLAN_PLACE, s_PLAN_PLACE);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_QUESTION", g_PLAN_QUESTION, s_PLAN_QUESTION);
    pyrt::xrefs::reg_access("compose.plan_ask", "PLAN_TOGETHER", g_PLAN_TOGETHER, s_PLAN_TOGETHER);
}

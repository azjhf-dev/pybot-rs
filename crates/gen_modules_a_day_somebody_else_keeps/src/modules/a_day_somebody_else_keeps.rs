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


thread_local! { static G_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_CARE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_CARE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCHOOL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_bot: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_day_somebody_else_keeps: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_CHILD() -> Py { ensure_init(); G_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD(v: Py) { ensure_init(); G_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASS_LINES() -> Py { ensure_init(); G_CLASS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS_LINES(v: Py) { ensure_init(); G_CLASS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_CARE_ASK() -> Py { ensure_init(); G_MY_CARE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_CARE_ASK(v: Py) { ensure_init(); G_MY_CARE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_CARE_NOT() -> Py { ensure_init(); G_MY_CARE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_CARE_NOT(v: Py) { ensure_init(); G_MY_CARE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_SCHOOL_LINES() -> Py { ensure_init(); G_SCHOOL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCHOOL_LINES(v: Py) { ensure_init(); G_SCHOOL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_CLASS() -> Py { ensure_init(); G_THEIR_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_CLASS(v: Py) { ensure_init(); G_THEIR_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_last_bot() -> Py { ensure_init(); G_last_bot.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_bot(v: Py) { ensure_init(); G_last_bot.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A day that belongs to a school or a club is named for whoever keeps it, not searched for.\n\n\"What time does my son finish school\" and \"what time does my class start\" reached the composer's\nhedge - \"I don't know enough to answer that honestly. A search should sort you out quickly\", or\n\"That's beyond what I know well, sorry\" - which names neither whose timetable it is nor where the\ntimes live. Nobody can search that answer out of the air either: the finish time is set by the\nschool, the club or the class, and the one place that has it is their office, their app, the\nnewsletter or the letter.\n\nEach line leads with the part that is knowable - a school day holds steady through a term, and it\nis the last day of term, an inset day or a trip that moves it; a class moves more often, because a\nsession can be cancelled or a tutor changed - then names who has the rest, then offers the part\nthat can still be done here: the arithmetic on the journey or the rest of the day. No time is ever\ngiven, because none is known.\n\nThe fences are the entries that already answer. Their own shift keeps the rota line (\"what time do\ni finish work\"), a bus or a train keeps the operator's (\"what time does my bus leave\"), a surgery\nkeeps the record line (\"what time does my appointment start\"), and an exam or a results day is the\nhealth record's, not a timetable's. A child's ordinary evening (\"my son wont eat vegetables\") is\nnot a timetable at all, and the ask pattern needs a start or finish verb to match at all.\n\nThis sits above the greeting band (PRIORITY -1000) and below every knowledge module, so it only\never answers a line no entry reached: a term-dates card, a starting-school card or an answers card\nkeeps its turn.\n");
    s_last_bot(py_func("last_bot", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_bot", args)));
    s_PRIORITY(op_neg(&Py::Int(999)));
    s_MY_CARE_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what time|when|which time|what hours)\\b[^?]{0,40}?\\b(?:my|our)\\s+(?:son|daughter|kid|kids|child|children|boy|girl|little one|baby|niece|nephew|grandson|granddaughter|grandchild|grandchildren|childminder|babysitter|class|classes|lesson|lessons|course|club|swimming|training|nursery|school)\\b[^?]{0,30}?\\b(?:finish|finishes|start|starts|get out|gets out|break up|breaks up|come out|comes out|be done|is done|end|ends|go in|goes in|be picked up|gets picked up|get home|gets home|be home|is home|get there|gets there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_CARE_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:bus|buses|train|trains|tram|tube|flight|flights|coach|ferry|traffic|motorway|road|roads|strike|delayed|cancelled|appointment|appointments|doctor|dentist|hospital|physio|clinic|surgery|nurse|pharmacy|chemist|tablets?|medicine|medication|exam|test results?|results?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_CLASS(stdlib_call("re", "compile", &[py_str("\\b(?:class|classes|lesson|lessons|course|club|swimming|training)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:son|daughter|kid|kids|child|children|boy|girl|little one|baby|niece|nephew|grandson|granddaughter|grandchild|grandchildren|childminder|babysitter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SCHOOL_LINES(py_tuple(vec![py_str("A school day is usually the same length every day of the week, so the finish time they gave you in September still holds unless it is the last day of term, an inset day or a trip. The school office, its app or the newsletter has this term's version; give me the times and how long the journey takes and I'll work out when you need to set off."), py_str("The times are set by the school or club, not by anything I can look up, but they rarely move inside a term - it is the last day of term, inset days and trips that shift them. Their office will tell you in one call and the app you sign them in on has the same; hand me the times once you have them and I'll plan the pick-up around the rest of your day."), py_str("Those hours belong to the school or club, so I can't know this week's, but a school day holds steady through a term apart from the last day of it and any inset day - and those are on the school's own calendar. Tell me the finish time and the trip and the rest is arithmetic I can do with you.")]));
    s_CLASS_LINES(py_tuple(vec![py_str("A class time is set by whoever runs the course and it can move week to week - a new tutor, a session off, a room change - so the timetable you were given is the starting point rather than the answer. The provider's app, email or the notice on the door carries this week's; tell me the times and what you're fitting around them and I'll plan it with you."), py_str("I can't see your timetable from here. What I can say is that classes move more often than people expect - a session cancelled, a tutor changed - and it is the provider's own message that carries the change. Bring me the times and I'll do the planning around them."), py_str("The provider keeps that timetable, not me, and it changes: a bank holiday, a cancelled session, a new room. Their app or the email they send is where this week's version sits; give me the times and I'll work out the rest of your day around them.")]));
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
pub fn _choose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut c: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut options: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A line from the pool, never the same one twice running, and the same one every run.\n\n    `random.choice` here was unseeded, so the same line at the same turn of the same conversation\n    gave a different sentence in two runs of the bot. Every comparison the swarm makes is a diff of\n    two runs, so that read as a change no commit had made: the seed is the turn count and the line\n    itself, which is what `bot/compose/pick` does for every other pool.\n    ");
    last = mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_bot", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    options = { let __b = fresh.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[options.clone()]);
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
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    said = _clean(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_MY_CARE_NOT(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MY_CARE_ASK(), "search", &[said.clone()])))) {
    return Py::None;
    }
    pool = if py_truthy(&{ let __b = mcall(&g_THEIR_CLASS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_CHILD(), "search", &[said.clone()]))) } }) { g_CLASS_LINES() } else { g_SCHOOL_LINES() };
    return _choose(&[pool.clone(), memory.clone(), said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_day_somebody_else_keeps".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHILD" => Some(g_A_CHILD()),
            "CLASS_LINES" => Some(g_CLASS_LINES()),
            "MY_CARE_ASK" => Some(g_MY_CARE_ASK()),
            "MY_CARE_NOT" => Some(g_MY_CARE_NOT()),
            "PRIORITY" => Some(g_PRIORITY()),
            "SCHOOL_LINES" => Some(g_SCHOOL_LINES()),
            "THEIR_CLASS" => Some(g_THEIR_CLASS()),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "last_bot" => Some(g_last_bot()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "random" => Some(stdlib_module("random")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CHILD" => s_A_CHILD(v),
            "CLASS_LINES" => s_CLASS_LINES(v),
            "MY_CARE_ASK" => s_MY_CARE_ASK(v),
            "MY_CARE_NOT" => s_MY_CARE_NOT(v),
            "PRIORITY" => s_PRIORITY(v),
            "SCHOOL_LINES" => s_SCHOOL_LINES(v),
            "THEIR_CLASS" => s_THEIR_CLASS(v),
            "last_bot" => s_last_bot(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_day_somebody_else_keeps", module_obj);
    pyrt::xrefs::reg_fn("modules.a_day_somebody_else_keeps", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.a_day_somebody_else_keeps", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("modules.a_day_somebody_else_keeps", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "A_CHILD", g_A_CHILD, s_A_CHILD);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "CLASS_LINES", g_CLASS_LINES, s_CLASS_LINES);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "MY_CARE_ASK", g_MY_CARE_ASK, s_MY_CARE_ASK);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "MY_CARE_NOT", g_MY_CARE_NOT, s_MY_CARE_NOT);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "SCHOOL_LINES", g_SCHOOL_LINES, s_SCHOOL_LINES);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "THEIR_CLASS", g_THEIR_CLASS, s_THEIR_CLASS);
    pyrt::xrefs::reg_access("modules.a_day_somebody_else_keeps", "last_bot", g_last_bot, s_last_bot);
}

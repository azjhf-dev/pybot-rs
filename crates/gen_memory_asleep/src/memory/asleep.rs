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


thread_local! { static G_ASKS_A_BEDTIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BED_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRIES_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_WAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NIGHT_WAKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_NOT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAKE_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ATOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.asleep: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKS_A_BEDTIME() -> Py { ensure_init(); G_ASKS_A_BEDTIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_BEDTIME(v: Py) { ensure_init(); G_ASKS_A_BEDTIME.with(|c| *c.borrow_mut() = v); }
pub fn g_BED_MARKER() -> Py { ensure_init(); G_BED_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BED_MARKER(v: Py) { ensure_init(); G_BED_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRIES_ON() -> Py { ensure_init(); G_CARRIES_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRIES_ON(v: Py) { ensure_init(); G_CARRIES_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_FINE() -> Py { ensure_init(); G_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINE(v: Py) { ensure_init(); G_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_A_SUBJECT() -> Py { ensure_init(); G_HAS_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_A_SUBJECT(v: Py) { ensure_init(); G_HAS_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_I_WAKE() -> Py { ensure_init(); G_I_WAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_WAKE(v: Py) { ensure_init(); G_I_WAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG() -> Py { ensure_init(); G_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG(v: Py) { ensure_init(); G_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_NIGHT_WAKING() -> Py { ensure_init(); G_NIGHT_WAKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NIGHT_WAKING(v: Py) { ensure_init(); G_NIGHT_WAKING.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_NOT_ME() -> Py { ensure_init(); G_THEY_NOT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_NOT_ME(v: Py) { ensure_init(); G_THEY_NOT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_WAKE_MARKER() -> Py { ensure_init(); G_WAKE_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAKE_MARKER(v: Py) { ensure_init(); G_WAKE_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g__ATOM() -> Py { ensure_init(); G__ATOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ATOM(v: Py) { ensure_init(); G__ATOM.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD() -> Py { ensure_init(); G__WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD(v: Py) { ensure_init(); G__WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_HOURS() -> Py { ensure_init(); G__WORD_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_HOURS(v: Py) { ensure_init(); G__WORD_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The night the person described: two ends of it, and the hours between them.\n\nPeople give a night one end at a time. \"i go to bed at about 11\" is answered from the bedtime\narithmetic with a seven o'clock start assumed, and then \"and wake at 6\" - the other end of the\nsame night - was read on its own and answered as a night that went wrong:\n\n```\ni feel knackered all the time / i go to bed at about 11 / and wake at 6\n  before: Waking at the same time every night is common and it passes. Don't watch the clock...\n```\n\nThat is advice for somebody who cannot stay asleep, and the person had just said what their night\nis. The two times are one night when the chat gives both, and the answer is the length between\nthem read against the seven to nine hours most adults need - which is the whole of what the\nperson is asking about when they say their bedtime and their alarm in the same breath.\n\nThe rule is narrow on purpose:\n* a line that gives both ends answers on its own; a line that gives only the wake time has to be\n  the other end of the night the turn before described, so \"wake at 6\" after a bedtime is the\n  night, and \"wake at 6\" after anything else is left to the module that reads it;\n* a fragment is recognised by its opening (\"and\", \"also\", \"then\", \"but\", \"plus\") or by having no\n  subject of its own (\"wake at 6\", \"up at 6.30\"), or by being nothing but the person's own wake\n  marker and the hour in a line of six words or fewer (\"i wake at 5\", \"i get up at 6\"); a whole\n  sentence about waking is a different thing, and \"i wake at 6 every night and cant get back\"\n  stays the night-waking answer;\n* nobody else's night is read: a child, a baby, a pet or a partner named in the line or in the\n  line the time came from keeps its own answer;\n* a wake time has to be a morning one (4am to 11am), so \"i wake at 3\" is the broken night it\n  sounds like and not a six-hour night;\n* a pair of times that gives less than two hours or more than fourteen is not a night's sleep,\n  so nothing is said about the person's health off a pair of times that do not add up.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|her|his|their|the)\\s+(?:son|daughter|baby|boy|girl|child|children|kid|kids|toddler|teenager|teen|mum|mom|mother|dad|father|partner|husband|wife|boyfriend|girlfriend|dog|cat|puppy|kitten|pet|flatmate|housemate|roommate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_NOT_ME(stdlib_call("re", "compile", &[py_str("^\\s*(?:he|she|they|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NIGHT_WAKING(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|always|still|constantly|often)\\s+wak(?:e|es|ing)\\b|\\bwak(?:e|es|ing)\\s+up\\s+(?:in the night|during the night|several times|a few times)\\b|\\b(?:in|during) the night\\b|\\bget back to sleep\\b|\\bcant sleep\\b|\\bcan't sleep\\b|\\binsomnia\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARRIES_ON(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|also|then|but|plus|oh and|,\\s*and)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAS_A_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|you|he|she|they|it)\\b|'m\\b|\\bim\\b|\\bmy\\b|\\bour\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_I_WAKE(stdlib_call("re", "compile", &[py_str("^\\s*(?:i|im|i'm|ive|i've|we|we're|were|weve|we've)\\s+(?:usually\\s+|normally\\s+|often\\s+|just\\s+|then\\s+|still\\s+)?(?:wake|wakes|waking|woke|woken|waked|waken|get\\s+up|gets\\s+up|got\\s+up|getting\\s+up|get|got|am|up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORD_HOURS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]));
    s__WORD(mcall(&py_str("|"), "join", &[g__WORD_HOURS()]));
    s__ATOM(op_add(&op_add(&op_add(&op_add(&py_str("(?:(?:at|around|about|by|till|until)\\s+)?(?P<atom>midnight|noon|half\\s+(?P<half>"), &g__WORD()), &py_str(")|(?P<clock>[01]?\\d|2[0-3])(?:[:.](?P<minutes>[0-5]\\d))?\\s*(?P<when>am|pm|a\\.m\\.|p\\.m\\.)?|(?P<word>")), &g__WORD()), &py_str(")(?=\\b))")));
    s_BED_MARKER(stdlib_call("re", "compile", &[py_str("\\b(?:go(?:es|ing)?\\s+(?:off\\s+)?to\\s+(?:bed|sleep)|went\\s+to\\s+(?:bed|sleep)|to\\s+bed|in\\s+bed|bedtime|bed|asleep|sleep(?:s|ing)?|slept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAKE_MARKER(stdlib_call("re", "compile", &[py_str("\\b(?:wake|wakes|waking|woke)(?:\\s+up)?\\b|\\b(?:get|gets|getting|got)\\s+up\\b|\\b(?:am|are|be|im|i'm)\\s+up\\b|\\bup\\b(?=\\s*(?:at|around|about|by)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_BEDTIME(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+time|when|what\\s+hour|how\\s+long\\s+before|what\\s+sort\\s+of\\s+time)\\b[^.!?]{0,40}?\\b(?:go|going|be|get|getting)\\s+(?:off\\s+)?to\\s+(?:bed|sleep)\\b|\\b(?:go|going)\\s+to\\s+bed\\s+(?:to|if|so\\s+that|before)\\b|\\bshould\\s+i\\s+(?:go\\s+to\\s+bed|be\\s+in\\s+bed|be\\s+going\\s+to\\s+bed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT(py_tuple(vec![py_str("{length} between bed and the alarm is under the seven to nine most adults need, so the end worth moving is the bedtime."), py_str("{length} is short of the seven to nine most adults do best on. The going to bed is the end that gives, because the wake time is what sets the clock.")]));
    s_FINE(py_tuple(vec![py_str("{length} is inside the seven to nine most adults need, so it is the quality of the night rather than the length that is worth looking at."), py_str("{length} sits in the seven to nine range, so the length is not the problem - a warm room, a drink, or a phone in bed are what break a night that long.")]));
    s_LONG(py_tuple(vec![py_str("{length} is more than the seven to nine most adults need, and sleeping past that often leaves a person groggy rather than rested."), py_str("{length} is past the seven to nine range. A slightly earlier alarm usually leaves somebody more awake, not less.")]));
}
pub fn _hour_from(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut atom: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "atom") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: atom"))) };
    let mut hour: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The hour an atom names, or None when the atom cannot be read on its own.");
    if py_truthy(&Py::Bool(py_is(&atom.clone(), &Py::None))) {
    return Py::None;
    }
    said = mcall(&mcall(&mcall(&atom.clone(), "group", &[py_str("atom")]), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("midnight")])) {
    return Py::Int(0);
    }
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("noon")])) {
    return Py::Int(12);
    }
    if py_truthy(&mcall(&atom.clone(), "group", &[py_str("half")])) {
    hour = mcall(&g__WORD_HOURS(), "get", &[mcall(&mcall(&atom.clone(), "group", &[py_str("half")]), "lower", &[])]);
    if py_truthy(&Py::Bool(py_is(&hour.clone(), &Py::None))) {
    return Py::None;
    }
    return if py_truthy(&{ let __cmp11: Py = hour.clone(); if py_truthy(&op_le(&Py::Int(12), &__cmp11)) { op_le(&__cmp11, &Py::Int(23)) } else { Py::Bool(false) } }) { op_add(&call_builtin("float", &[hour.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.5)) } else { op_add(&call_builtin("float", &[hour.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(12.5)) };
    }
    if py_truthy(&mcall(&atom.clone(), "group", &[py_str("word")])) {
    return mcall(&g__WORD_HOURS(), "get", &[mcall(&mcall(&atom.clone(), "group", &[py_str("word")]), "lower", &[])]);
    }
    hour = call_builtin("int", &[mcall(&atom.clone(), "group", &[py_str("clock")])]).unwrap_or_else(|e| exc::raise(e));
    when = mcall(&mcall(&{ let __b = mcall(&atom.clone(), "group", &[py_str("when")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "replace", &[py_str("."), py_str("")]);
    if py_truthy(&mcall(&when.clone(), "startswith", &[py_str("p")])) {
    return if py_truthy(&op_lt(&hour.clone(), &Py::Int(12))) { op_add(&hour.clone(), &Py::Int(12)) } else { hour.clone() };
    }
    if py_truthy(&mcall(&when.clone(), "startswith", &[py_str("a")])) {
    return if py_truthy(&Py::Bool(py_eq(&hour.clone(), &Py::Int(12)))) { Py::Int(0) } else { hour.clone() };
    }
    return hour.clone();
    Py::None
}
pub fn _end(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut marker: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "marker") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: marker"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut after_bed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_bed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after_bed"))) };
    let mut limit: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(26) };
    let mut atom: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hour: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut window: Py = Py::None;
    let _ = py_str("The time said just after the word that names one end of the night, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&marker.clone(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    window = slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &op_add(&mcall(&found.clone(), "end", &[]), &limit.clone()), &Py::None);
    atom = stdlib_call("re", "search", &[g__ATOM(), window.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&atom.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = after_bed.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_WAKE_MARKER(), "search", &[slice_or_raise(&window.clone(), &Py::None, &mcall(&atom.clone(), "start", &[]), &Py::None)]) } }) {
    continue;
    }
    hour = _hour_from(&[atom.clone()]);
    if py_truthy(&Py::Bool(py_is(&hour.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&mcall(&atom.clone(), "group", &[py_str("when")])) {
    return hour.clone();
    }
    if py_truthy(&mcall(&atom.clone(), "group", &[py_str("clock")])) {
    minutes = op_truediv(&call_builtin("float", &[{ let __b = mcall(&atom.clone(), "group", &[py_str("minutes")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(60.0));
    if py_truthy(&{ let __b = after_bed.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&atom.clone(), "group", &[py_str("when")]))) } }) {
    if py_truthy(&Py::Bool(py_eq(&hour.clone(), &Py::Int(12)))) {
    return minutes.clone();
    }
    return if py_truthy(&{ let __cmp21: Py = hour.clone(); if py_truthy(&op_le(&Py::Int(6), &__cmp21)) { op_le(&__cmp21, &Py::Int(11)) } else { Py::Bool(false) } }) { op_add(&op_add(&hour.clone(), &Py::Int(12)), &minutes.clone()) } else { Py::None };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after_bed.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&atom.clone(), "group", &[py_str("when")]))) } }) {
    return if py_truthy(&{ let __cmp31: Py = hour.clone(); if py_truthy(&op_le(&Py::Int(4), &__cmp31)) { op_le(&__cmp31, &Py::Int(11)) } else { Py::Bool(false) } }) { op_add(&hour.clone(), &minutes.clone()) } else { Py::None };
    }
    return op_add(&hour.clone(), &minutes.clone());
    }
    if py_truthy(&after_bed.clone()) {
    return if py_truthy(&{ let __cmp41: Py = hour.clone(); if py_truthy(&op_le(&Py::Int(6), &__cmp41)) { op_le(&__cmp41, &Py::Int(11)) } else { Py::Bool(false) } }) { op_add(&hour.clone(), &Py::Int(12)) } else { hour.clone() };
    }
    return if py_truthy(&{ let __cmp51: Py = hour.clone(); if py_truthy(&op_le(&Py::Int(4), &__cmp51)) { op_le(&__cmp51, &Py::Int(11)) } else { Py::Bool(false) } }) { hour.clone() } else { Py::None };
    }
    return Py::None;
    Py::None
}
pub fn _ends(args: &[Py]) -> Py {
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
    let mut wake_only: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wake_only") { v.clone() }
    else { Py::Bool(false) };
    let mut bed: Py = Py::None;
    let mut text: Py = Py::None;
    let mut wake: Py = Py::None;
    let _ = py_str("The bedtime and the wake time a line names, as (bed, wake), either one possibly None.");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    bed = if py_truthy(&wake_only.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_BED_MARKER()); __args.push(text.clone()); __kws.extend(vec![("after_bed".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end(args))(&__args) } };
    wake = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WAKE_MARKER()); __args.push(text.clone()); __kws.extend(vec![("after_bed".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _end(args))(&__args) };
    return py_tuple(vec![bed.clone(), wake.clone()]);
    Py::None
}
pub fn _last_night(args: &[Py]) -> Py {
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
    let mut bed: Py = Py::None;
    let mut low: Py = Py::None;
    let mut one_bed: Py = Py::None;
    let mut one_wake: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wake: Py = Py::None;
    let _ = py_str("The bedtime and the wake time the chat has given, newest line first.");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::None]), 2, None);
    bed = __unpk1[0].clone();
    wake = __unpk1[1].clone();
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    continue;
    }
    low = mcall(&said.clone(), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g_SOMEONE_ELSE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEY_NOT_ME(), "search", &[low.clone()]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    let __unpk3 = py_unpack(&_ends(&[said.clone()]), 2, None);
    one_bed = __unpk3[0].clone();
    one_wake = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&bed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&one_bed.clone(), &Py::None)) } }) {
    bed = one_bed.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&wake.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&one_wake.clone(), &Py::None)) } }) {
    wake = one_wake.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&bed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&wake.clone(), &Py::None)) } }) {
    break;
    }
    }
    return py_tuple(vec![bed.clone(), wake.clone()]);
    Py::None
}
pub fn _a_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut hours: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "hours") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: hours"))) };
    let mut half: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A length of a night the way a person says it: seven hours, seven and a half hours.");
    whole = call_builtin("int", &[hours.clone()]).unwrap_or_else(|e| exc::raise(e));
    half = call_builtin("int", &[call_builtin("round", &[op_mul(&op_sub(&hours.clone(), &whole.clone()), &Py::Int(60))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&half.clone(), &Py::Int(30)))) {
    return op_modulo(&py_str("%s and a half hours"), &_a_word(&[whole.clone()]));
    }
    if py_truthy(&half.clone()) {
    return op_modulo(&py_str("%s hours and %d minutes"), &py_tuple(vec![_a_word(&[whole.clone()]), half.clone()]));
    }
    return op_modulo(&py_str("%s hours"), &_a_word(&[whole.clone()]));
    Py::None
}
pub fn _a_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut words: Py = Py::None;
    words = py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve")), (Py::Int(13), py_str("thirteen")), (Py::Int(14), py_str("fourteen"))]);
    return mcall(&words.clone(), "get", &[number.clone(), call_builtin("str", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _seen(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("The reply, remembered, so the next turn can add to it rather than say it again.");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&memory.clone(), &py_str("asleep_said"), slice_or_raise(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(120), &Py::None));
    }
    return reply.clone();
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
    else { Py::None };
    let mut _was_wake: Py = Py::None;
    let mut bed: Py = Py::None;
    let mut carries_on: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine_and_short: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shapes: Py = Py::None;
    let mut wake: Py = Py::None;
    let mut was_bed: Py = Py::None;
    let _ = py_str("The hours between the two ends of the night they described, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::None;
    }
    low = mcall(&said.clone(), "lower", &[]);
    if py_truthy(&mcall(&g_ASKS_A_BEDTIME(), "search", &[low.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_ends(&[said.clone()]), 2, None);
    bed = __unpk1[0].clone();
    wake = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&bed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&wake.clone(), &Py::None)) } }) {
    if py_truthy(&{ let __b = mcall(&g_SOMEONE_ELSE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THEY_NOT_ME(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NIGHT_WAKING(), "search", &[low.clone()]) } } } }) {
    return Py::None;
    }
    mine_and_short = { let __b = call_builtin("bool", &[mcall(&g_I_WAKE(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } };
    carries_on = { let __b = call_builtin("bool", &[mcall(&g_CARRIES_ON(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_HAS_A_SUBJECT(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { mine_and_short.clone() } } } };
    if py_truthy(&Py::Bool(!py_truthy(&carries_on.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_last_night(&[memory.clone()]), 2, None);
    was_bed = __unpk2[0].clone();
    _was_wake = __unpk2[1].clone();
    bed = was_bed.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&bed.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&wake.clone(), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_SOMEONE_ELSE(), "search", &[low.clone()])) {
    return Py::None;
    }
    hours = op_modulo(&op_sub(&wake.clone(), &bed.clone()), &Py::Int(24));
    if py_truthy(&{ let __b = op_lt(&hours.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { op_gt(&hours.clone(), &Py::Int(14)) } }) {
    return Py::None;
    }
    if py_truthy(&op_lt(&hours.clone(), &Py::Int(7))) {
    shapes = g_SHORT();
    } else {
    if py_truthy(&op_le(&hours.clone(), &Py::Int(9))) {
    shapes = g_FINE();
    } else {
    shapes = g_LONG();
    }
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("length".to_string(), _a_length(&[hours.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[shapes.clone(), memory.clone(), said.clone()]), "format", args))(&__args) };
    return _seen(&[memory.clone(), op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.asleep".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_A_BEDTIME" => Some(g_ASKS_A_BEDTIME()),
            "BED_MARKER" => Some(g_BED_MARKER()),
            "CARRIES_ON" => Some(g_CARRIES_ON()),
            "FINE" => Some(g_FINE()),
            "HAS_A_SUBJECT" => Some(g_HAS_A_SUBJECT()),
            "I_WAKE" => Some(g_I_WAKE()),
            "LONG" => Some(g_LONG()),
            "NIGHT_WAKING" => Some(g_NIGHT_WAKING()),
            "SHORT" => Some(g_SHORT()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "THEY_NOT_ME" => Some(g_THEY_NOT_ME()),
            "WAKE_MARKER" => Some(g_WAKE_MARKER()),
            "_ATOM" => Some(g__ATOM()),
            "_WORD" => Some(g__WORD()),
            "_WORD_HOURS" => Some(g__WORD_HOURS()),
            "_a_length" => Some(py_func("_a_length", |args: &[Py]| _a_length(args))),
            "_a_word" => Some(py_func("_a_word", |args: &[Py]| _a_word(args))),
            "_end" => Some(py_func("_end", |args: &[Py]| _end(args))),
            "_ends" => Some(py_func("_ends", |args: &[Py]| _ends(args))),
            "_hour_from" => Some(py_func("_hour_from", |args: &[Py]| _hour_from(args))),
            "_last_night" => Some(py_func("_last_night", |args: &[Py]| _last_night(args))),
            "_seen" => Some(py_func("_seen", |args: &[Py]| _seen(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_A_BEDTIME" => s_ASKS_A_BEDTIME(v),
            "BED_MARKER" => s_BED_MARKER(v),
            "CARRIES_ON" => s_CARRIES_ON(v),
            "FINE" => s_FINE(v),
            "HAS_A_SUBJECT" => s_HAS_A_SUBJECT(v),
            "I_WAKE" => s_I_WAKE(v),
            "LONG" => s_LONG(v),
            "NIGHT_WAKING" => s_NIGHT_WAKING(v),
            "SHORT" => s_SHORT(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            "THEY_NOT_ME" => s_THEY_NOT_ME(v),
            "WAKE_MARKER" => s_WAKE_MARKER(v),
            "_ATOM" => s__ATOM(v),
            "_WORD" => s__WORD(v),
            "_WORD_HOURS" => s__WORD_HOURS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.asleep", module_obj);
    pyrt::xrefs::reg_fn("memory.asleep", "_hour_from", py_func("_hour_from", |args: &[Py]| _hour_from(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_end", py_func("_end", |args: &[Py]| _end(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_ends", py_func("_ends", |args: &[Py]| _ends(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_last_night", py_func("_last_night", |args: &[Py]| _last_night(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_a_length", py_func("_a_length", |args: &[Py]| _a_length(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_a_word", py_func("_a_word", |args: &[Py]| _a_word(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "_seen", py_func("_seen", |args: &[Py]| _seen(args)));
    pyrt::xrefs::reg_fn("memory.asleep", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.asleep", "ASKS_A_BEDTIME", g_ASKS_A_BEDTIME, s_ASKS_A_BEDTIME);
    pyrt::xrefs::reg_access("memory.asleep", "BED_MARKER", g_BED_MARKER, s_BED_MARKER);
    pyrt::xrefs::reg_access("memory.asleep", "CARRIES_ON", g_CARRIES_ON, s_CARRIES_ON);
    pyrt::xrefs::reg_access("memory.asleep", "FINE", g_FINE, s_FINE);
    pyrt::xrefs::reg_access("memory.asleep", "HAS_A_SUBJECT", g_HAS_A_SUBJECT, s_HAS_A_SUBJECT);
    pyrt::xrefs::reg_access("memory.asleep", "I_WAKE", g_I_WAKE, s_I_WAKE);
    pyrt::xrefs::reg_access("memory.asleep", "LONG", g_LONG, s_LONG);
    pyrt::xrefs::reg_access("memory.asleep", "NIGHT_WAKING", g_NIGHT_WAKING, s_NIGHT_WAKING);
    pyrt::xrefs::reg_access("memory.asleep", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("memory.asleep", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("memory.asleep", "THEY_NOT_ME", g_THEY_NOT_ME, s_THEY_NOT_ME);
    pyrt::xrefs::reg_access("memory.asleep", "WAKE_MARKER", g_WAKE_MARKER, s_WAKE_MARKER);
    pyrt::xrefs::reg_access("memory.asleep", "_ATOM", g__ATOM, s__ATOM);
    pyrt::xrefs::reg_access("memory.asleep", "_WORD", g__WORD, s__WORD);
    pyrt::xrefs::reg_access("memory.asleep", "_WORD_HOURS", g__WORD_HOURS, s__WORD_HOURS);
    pyrt::xrefs::reg_access("memory.asleep", "pick", g_pick, s_pick);
}

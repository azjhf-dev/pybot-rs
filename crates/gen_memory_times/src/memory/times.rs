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


thread_local! { static G_CLOCK_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_CLAIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SAY_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SHIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_WORK_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTINE_WORK_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.times: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CLOCK_SAID() -> Py { ensure_init(); G_CLOCK_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK_SAID(v: Py) { ensure_init(); G_CLOCK_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_ASK() -> Py { ensure_init(); G_ROUTINE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_ASK(v: Py) { ensure_init(); G_ROUTINE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_ASK_FILLER() -> Py { ensure_init(); G_ROUTINE_ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_ASK_FILLER(v: Py) { ensure_init(); G_ROUTINE_ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_CLAIM() -> Py { ensure_init(); G_ROUTINE_CLAIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_CLAIM(v: Py) { ensure_init(); G_ROUTINE_CLAIM.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_MORE() -> Py { ensure_init(); G_ROUTINE_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_MORE(v: Py) { ensure_init(); G_ROUTINE_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SAY_BACK() -> Py { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SAY_BACK(v: Py) { ensure_init(); G_ROUTINE_SAY_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SHIFT() -> Py { ensure_init(); G_ROUTINE_SHIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SHIFT(v: Py) { ensure_init(); G_ROUTINE_SHIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_SUBJECT() -> Py { ensure_init(); G_ROUTINE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_SUBJECT(v: Py) { ensure_init(); G_ROUTINE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_WORK_END() -> Py { ensure_init(); G_ROUTINE_WORK_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_WORK_END(v: Py) { ensure_init(); G_ROUTINE_WORK_END.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTINE_WORK_START() -> Py { ensure_init(); G_ROUTINE_WORK_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTINE_WORK_START(v: Py) { ensure_init(); G_ROUTINE_WORK_START.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_FIX() -> Py { ensure_init(); G_TIME_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_FIX(v: Py) { ensure_init(); G_TIME_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g_last_user() -> Py { ensure_init(); G_last_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_user(v: Py) { ensure_init(); G_last_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The hours a person's own day runs on: the times they give, and the ask that wants them back.\n\n\"i get up at six thirty\" is filed against the doing, newest winning, and \"what time do i get up\"\nthree turns later is answered from it. The reader moved here out of `bot/modules/memory_store.py`\nbecause that file is at the Hub's 1 MB limit and every reader that leaves it buys back room; the\nbehaviour is the same, so the move is a pure one and every reply it makes is byte-identical.\n");
    s_last_user(py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args)));
    s_CLOCK_SAID(pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID"));
    s_ROUTINE_SAY_BACK(pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SAY_BACK"));
    s_ROUTINE_SUBJECT(pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SUBJECT"));
    s_ROUTINE_CLAIM(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SUBJECT")), &py_str("\\b(?P<mid>[a-z' ,]{1,26}?)\\b(?:at|around|about|round|by)\\s+(?P<at>")), &pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID")), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_SHIFT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bmy (?P<thing>shift|work|job|hours)\\s+(?P<verb>starts?|begins?|ends?|finishes?)\\s+(?:at|around|about)\\s+(?P<at>"), &pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID")), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_WORK_START(stdlib_call("re", "compile", &[py_str("\\b(?:start|starts|started|starting|begin|begins|beginning|clock in|clocking in|get in|gets in|arrive|arrives)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_WORK_END(stdlib_call("re", "compile", &[py_str("\\b(?:finish|finishes|finishing|leave|leaves|leaving|get off|getting off|clock out|clocking out|end|ends|ending)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_MORE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and |it |that |then )*(?P<verb>ends?|finishes?|starts?|begins?)\\s+(?:at|around|about)\\s+(?P<at>"), &pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID")), &py_str(")[\\s.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_FIX(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^(?:(?:ok|okay|oh|sorry|no|nope|nah|wait|actually|rather|i mean|i meant|meant|i said|make that|make it|hmm|erm)[ ,]+)*(?:at |more like |it's |its )?(?P<at>"), &pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID")), &py_str(")(?:\\s*(?:instead|o'clock|oclock|sorry|then))*(?:\\s+not\\s+(?P<was>")), &pyrt::xrefs::xget("memory.store_lines", "CLOCK_SAID")), &py_str("))?[\\s.!?]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and |so |ok |okay |oh |but |also |then )*(?:at what time|what time|what's the time|when)\\b(?P<rest>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTINE_ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("do does did will am are is was were i we my the a an your our\nusually normally generally tend tends to of for going go about around"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_routine(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { Py::Bool(false) };
    let mut low: Py = Py::None;
    let mut work: Py = Py::None;
    let _ = py_str("Which part of the day a phrase is about, or \"\" when it is about none of them.\n\n    The words are a person's own (\"i usually get up\", \"wake up\", \"i'm up\"), so the reading is by\n    the thing named - bed, home, work - and not by the verb. A phrase that names none of them\n    (\"i met my friend\") is not a routine, and no time is filed from it. `ask` also reads the bare\n    verb a question can carry, where a statement would be too vague to keep: \"what time do i\n    finish\" is about work, while \"i finish at five\" could be anything.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z' ]"), py_str(" "), mcall(&phrase.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return py_str("");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bbed\\b|\\bsleep\\b|\\bturn in\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("go to bed");
    }
    work = { let __b = Py::Bool(py_contains(&low.clone(), &py_str("work"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&low.clone(), &py_str("shift"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("job"))) } } } };
    if py_truthy(&{ let __b = work.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ROUTINE_WORK_END(), "search", &[low.clone()]) } }) {
    return py_str("finish work");
    }
    if py_truthy(&{ let __b = work.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ROUTINE_WORK_START(), "search", &[low.clone()]) } }) {
    return py_str("start work");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bhome\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("get home");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bup\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("get up");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:leave|leaves|leaving|set off|head off|go out)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("leave");
    }
    if py_truthy(&ask.clone()) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:start|starts|begin|begins)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("start work");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:finish|finishes|end|ends|clock out)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("finish work");
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_times(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    held = mcall(&memory.clone(), "get", &[py_str("times")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    held = py_list(vec![]);
    }
    return held.clone();
    Py::None
}
pub fn _keep_a_time(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut at: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("File one routine time, newest winning, under the one key the ask is answered from.");
    at = mcall(&call_builtin("str", &[{ let __b = at.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!;:-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&at.clone())) } }) {
    return Py::None;
    }
    held = _the_times(&[memory.clone()]);
    turn = call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&held.clone());
    let mut __else2 = false;
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&item.clone(), "get", &[py_str("what")]), &what.clone())) } }) {
    let _ = py_setitem(&item.clone(), &py_str("at"), at.clone());
    let _ = py_setitem(&item.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&item.clone(), &py_str("turn"), turn.clone());
    break;
    }
    }
    let _ = mcall(&held.clone(), "append", &[py_dict(vec![(py_str("what"), what.clone()), (py_str("at"), at.clone()), (py_str("said"), said.clone()), (py_str("turn"), turn.clone())])]);
    let _ = py_setitem(&memory.clone(), &py_str("times"), slice_or_raise(&held.clone(), &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    Py::None
}
pub fn keep(args: &[Py]) -> Py {
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
    let mut filed: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut more: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut was: Py = Py::None;
    let mut what: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("Keep the times the person's day runs on, and the one they put right on the next line.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    low = mcall(&said.clone(), "lower", &[]);
    filed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&mcall(&g_ROUTINE_SHIFT(), "finditer", &[low.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    start = mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("verb")]), "lower", &[]), "startswith", &[py_tuple(vec![py_str("start"), py_str("begin")])]);
    let _ = _keep_a_time(&[memory.clone(), if py_truthy(&start.clone()) { py_str("start work") } else { py_str("finish work") }, mcall(&match_.clone(), "group", &[py_str("at")]), said.clone()]);
    filed = Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_ROUTINE_CLAIM(), "finditer", &[low.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    what = _a_routine(&[mcall(&match_.clone(), "group", &[py_str("mid")])]);
    if py_truthy(&what.clone()) {
    let _ = _keep_a_time(&[memory.clone(), what.clone(), mcall(&match_.clone(), "group", &[py_str("at")]), said.clone()]);
    filed = Py::Bool(true);
    }
    }
    if py_truthy(&filed.clone()) {
    return Py::None;
    }
    held = _the_times(&[memory.clone()]);
    more = mcall(&g_ROUTINE_MORE(), "match", &[mcall(&low.clone(), "strip", &[py_str(" .,!;")])]);
    if py_truthy(&{ let __b = more.clone(); if !py_truthy(&__b) { __b } else { { let __b = held.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&held.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    last = geti(&held.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("start work"), py_str("finish work")]), &mcall(&last.clone(), "get", &[py_str("what")])))) {
    if py_truthy(&op_le(&op_sub(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&last.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))) {
    which = if py_truthy(&mcall(&mcall(&mcall(&more.clone(), "group", &[py_str("verb")]), "lower", &[]), "startswith", &[py_tuple(vec![py_str("end"), py_str("finish")])])) { py_str("finish work") } else { py_str("start work") };
    let _ = _keep_a_time(&[memory.clone(), which.clone(), mcall(&more.clone(), "group", &[py_str("at")]), said.clone()]);
    return Py::None;
    }
    }
    }
    held = _the_times(&[memory.clone()]);
    fixed = mcall(&g_TIME_FIX(), "match", &[mcall(&low.clone(), "strip", &[py_str(" .,!;")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&fixed.clone())) } }) {
    return Py::None;
    }
    last = geti(&held.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&op_gt(&op_sub(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&last.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))) {
    if py_truthy(&Py::Bool(!py_truthy(&answer(&[{ let __b = pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }, memory.clone()])))) {
    return Py::None;
    }
    }
    was = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let _ = py_setitem(&last.clone(), &py_str("at"), mcall(&mcall(&fixed.clone(), "group", &[py_str("at")]), "strip", &[py_str(" .,!;:")]));
    let _ = py_setitem(&last.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&last.clone(), &py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&memory.clone(), &py_str("fixed_time"), py_dict(vec![(py_str("what"), mcall(&last.clone(), "get", &[py_str("what")])), (py_str("at"), geti(&last.clone(), &py_str("at"))), (py_str("from"), was.clone()), (py_str("turn"), geti(&last.clone(), &py_str("turn"))), (py_str("said"), said.clone())]));
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut at: Py = Py::None;
    let mut end_at: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut start_at: Py = Py::None;
    let mut what: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The time of their own day the ask names, said back in the words they gave it in.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    asked = mcall(&g_ROUTINE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    rest = mcall(&asked.clone(), "group", &[py_str("rest")]);
    held = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&_the_times(&[memory.clone()])) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&item.clone(), "get", &[py_str("at")]) } }) {  __out.push((mcall(&item.clone(), "get", &[py_str("what")]), mcall(&item.clone(), "get", &[py_str("at")]))); } } py_dict(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __kws.extend(vec![("ask".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_routine(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&what.clone()))) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:shift|work|job)\\b"), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&held.clone(), "get", &[py_str("start work")]), mcall(&held.clone(), "get", &[py_str("finish work")])]), 2, None);
    start_at = __unpk1[0].clone();
    end_at = __unpk1[1].clone();
    if py_truthy(&{ let __b = start_at.clone(); if !py_truthy(&__b) { __b } else { end_at.clone() } }) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Your shift runs from "), &start_at.clone()), &py_str(" to ")), &end_at.clone()), &py_str("."));
    }
    what = if py_truthy(&start_at.clone()) { py_str("start work") } else { if py_truthy(&end_at.clone()) { py_str("finish work") } else { py_str("") } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&what.clone()))) {
    return Py::None;
    }
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&stdlib_call("re", "sub", &[py_str("[^a-z' ]"), py_str(" "), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ROUTINE_ASK_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    at = mcall(&held.clone(), "get", &[what.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&at.clone()))) {
    return Py::None;
    }
    said_back = { let __b = mcall(&pyrt::xrefs::xget("memory.store_lines", "ROUTINE_SAY_BACK"), "get", &[what.clone()]); if py_truthy(&__b) { __b } else { py_str("You {what} at {at}.") } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("at".to_string(), at.clone()), ("what".to_string(), what.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_back.clone(), "format", args))(&__args) };
    Py::None
}
pub fn is_a_time_ask(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut what: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Is this line only the ask for one of the times the store keeps?");
    if py_truthy(&Py::Bool(!py_truthy(&answer(&[text.clone(), memory.clone()])))) {
    return Py::Bool(false);
    }
    asked = mcall(&g_ROUTINE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&asked.clone(), "group", &[py_str("rest")]);
    what = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __kws.extend(vec![("ask".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_routine(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:shift|work|job)\\b"), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    what = py_str("start work");
    }
    if py_truthy(&Py::Bool(!py_truthy(&what.clone()))) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&stdlib_call("re", "sub", &[py_str("[^a-z' ]"), py_str(" "), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ROUTINE_ASK_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.times".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CLOCK_SAID" => Some(g_CLOCK_SAID()),
            "ROUTINE_ASK" => Some(g_ROUTINE_ASK()),
            "ROUTINE_ASK_FILLER" => Some(g_ROUTINE_ASK_FILLER()),
            "ROUTINE_CLAIM" => Some(g_ROUTINE_CLAIM()),
            "ROUTINE_MORE" => Some(g_ROUTINE_MORE()),
            "ROUTINE_SAY_BACK" => Some(g_ROUTINE_SAY_BACK()),
            "ROUTINE_SHIFT" => Some(g_ROUTINE_SHIFT()),
            "ROUTINE_SUBJECT" => Some(g_ROUTINE_SUBJECT()),
            "ROUTINE_WORK_END" => Some(g_ROUTINE_WORK_END()),
            "ROUTINE_WORK_START" => Some(g_ROUTINE_WORK_START()),
            "TIME_FIX" => Some(g_TIME_FIX()),
            "_a_routine" => Some(py_func("_a_routine", |args: &[Py]| _a_routine(args))),
            "_keep_a_time" => Some(py_func("_keep_a_time", |args: &[Py]| _keep_a_time(args))),
            "_the_times" => Some(py_func("_the_times", |args: &[Py]| _the_times(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "is_a_time_ask" => Some(py_func("is_a_time_ask", |args: &[Py]| is_a_time_ask(args))),
            "keep" => Some(py_func("keep", |args: &[Py]| keep(args))),
            "last_user" => Some(g_last_user()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CLOCK_SAID" => s_CLOCK_SAID(v),
            "ROUTINE_ASK" => s_ROUTINE_ASK(v),
            "ROUTINE_ASK_FILLER" => s_ROUTINE_ASK_FILLER(v),
            "ROUTINE_CLAIM" => s_ROUTINE_CLAIM(v),
            "ROUTINE_MORE" => s_ROUTINE_MORE(v),
            "ROUTINE_SAY_BACK" => s_ROUTINE_SAY_BACK(v),
            "ROUTINE_SHIFT" => s_ROUTINE_SHIFT(v),
            "ROUTINE_SUBJECT" => s_ROUTINE_SUBJECT(v),
            "ROUTINE_WORK_END" => s_ROUTINE_WORK_END(v),
            "ROUTINE_WORK_START" => s_ROUTINE_WORK_START(v),
            "TIME_FIX" => s_TIME_FIX(v),
            "last_user" => s_last_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.times", module_obj);
    pyrt::xrefs::reg_fn("memory.times", "_a_routine", py_func("_a_routine", |args: &[Py]| _a_routine(args)));
    pyrt::xrefs::reg_fn("memory.times", "_the_times", py_func("_the_times", |args: &[Py]| _the_times(args)));
    pyrt::xrefs::reg_fn("memory.times", "_keep_a_time", py_func("_keep_a_time", |args: &[Py]| _keep_a_time(args)));
    pyrt::xrefs::reg_fn("memory.times", "keep", py_func("keep", |args: &[Py]| keep(args)));
    pyrt::xrefs::reg_fn("memory.times", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.times", "is_a_time_ask", py_func("is_a_time_ask", |args: &[Py]| is_a_time_ask(args)));
    pyrt::xrefs::reg_access("memory.times", "CLOCK_SAID", g_CLOCK_SAID, s_CLOCK_SAID);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_ASK", g_ROUTINE_ASK, s_ROUTINE_ASK);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_ASK_FILLER", g_ROUTINE_ASK_FILLER, s_ROUTINE_ASK_FILLER);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_CLAIM", g_ROUTINE_CLAIM, s_ROUTINE_CLAIM);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_MORE", g_ROUTINE_MORE, s_ROUTINE_MORE);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_SAY_BACK", g_ROUTINE_SAY_BACK, s_ROUTINE_SAY_BACK);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_SHIFT", g_ROUTINE_SHIFT, s_ROUTINE_SHIFT);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_SUBJECT", g_ROUTINE_SUBJECT, s_ROUTINE_SUBJECT);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_WORK_END", g_ROUTINE_WORK_END, s_ROUTINE_WORK_END);
    pyrt::xrefs::reg_access("memory.times", "ROUTINE_WORK_START", g_ROUTINE_WORK_START, s_ROUTINE_WORK_START);
    pyrt::xrefs::reg_access("memory.times", "TIME_FIX", g_TIME_FIX, s_TIME_FIX);
    pyrt::xrefs::reg_access("memory.times", "last_user", g_last_user, s_last_user);
}

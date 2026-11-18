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


thread_local! { static G_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_TO_LOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROGRESS_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.goals: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_LOSS() -> Py { ensure_init(); G_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOSS(v: Py) { ensure_init(); G_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_TO_LOSE() -> Py { ensure_init(); G_A_PLAN_TO_LOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_TO_LOSE(v: Py) { ensure_init(); G_A_PLAN_TO_LOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST() -> Py { ensure_init(); G_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST(v: Py) { ensure_init(); G_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_UNIT() -> Py { ensure_init(); G_GLUED_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_UNIT(v: Py) { ensure_init(); G_GLUED_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_GOING() -> Py { ensure_init(); G_HOW_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_GOING(v: Py) { ensure_init(); G_HOW_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_PROGRESS_REPLY() -> Py { ensure_init(); G_PROGRESS_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROGRESS_REPLY(v: Py) { ensure_init(); G_PROGRESS_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_BACK() -> Py { ensure_init(); G_SAID_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_BACK(v: Py) { ensure_init(); G_SAID_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS() -> Py { ensure_init(); G__NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS(v: Py) { ensure_init(); G__NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A loss the chat already has the reason for is not a loss that needs checking.\n\n    im trying to lose weight\n      -> Change a few things you can keep doing rather than going on a diet ...\n    i lost 2 kg\n      -> Was that on purpose, or has it been dropping on its own? If it's going without you\n         trying, a word with the GP is worth it.                        (main)\n\nThat reply belongs to weight coming off by itself, which is worth a word with a doctor. Here the\nperson said one turn ago that they were trying to lose it, so the answer is a shrug at a plan\nworking, and the figure said back. Nobody wants to be asked whether their diet was on purpose.\n\nThe rule is about the room, not the words: the same line in a chat with no plan to lose weight on\nfile keeps the check, because there it is unexplained loss. Nothing else changes - \"i lost my job\"\nis still bad news, and \"i lost 2 kg\" on its own is still the check, because nothing in the chat\nsays the person wanted to.\n");
    s_A_LOSS(stdlib_call("re", "compile", &[py_str("\\b(?:i|ive|i have)\\s+lost\\s+(?P<how>half\\s+a\\s+stone|a\\s+stone|(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten)\\s*(?:kilos?|kg|kgs|pounds?|lbs?|stone)|(?:some|a\\s+lot\\s+of)\\s+weight|weight)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAN_TO_LOSE(stdlib_call("re", "compile", &[py_str("\\b(?:lose|losing|lost|slim|slimming|diet|dieting|weight|kg|kilos?|pounds?|lbs?|stone|calories|cutting\\s+down|get\\s+fitter|fitness)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GLUED_UNIT(stdlib_call("re", "compile", &[py_str("(?<=\\d)(kgs?|kilos?|pounds?|lbs?|stone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_BACK(py_tuple(vec![py_str("{X} off is a real loss, and slow is the way it stays off. It is on file."), py_str("{X} down is the plan working, which is the only thing that matters here."), py_str("That is {x2} of the plan doing its job. I'll keep the meals on the lean side when you ask.")]));
    s_FIRST(py_tuple(vec![py_str("{X} off - good going, and at a rate that stays off. What is working so far?"), py_str("{X} down is real progress: slow enough to keep, which is the part most people miss."), py_str("Nice one: {X} off, and no starving yourself for it. Say if you want meal ideas that hold up.")]));
    s_HOW_GOING(stdlib_call("re", "compile", &[py_str("^(?:so\\s+|and\\s+|ok(?:ay)?\\s+,?\\s+)?(?:how\\s+am\\s+i\\s+(?:doing|getting\\s+on|going)|how\\s+(?:is|s)\\s+it\\s+going|how\\s+am\\s+i\\s+getting\\s+on|how\\s+is\\s+the\\s+(?:diet|plan)\\s+going|how\\s+much\\s+have\\s+i\\s+lost|how\\s+much\\s+weight\\s+have\\s+i\\s+lost|(?:is|was)\\s+that\\s+(?:good|ok|okay|alright|any\\s+good)\\s+(?:going|so\\s+far)?|am\\s+i\\s+doing\\s+(?:well|ok|okay|alright|it))\\s*[.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROGRESS_REPLY(py_tuple(vec![py_str("{X} so far, by your own count - and slow is the rate that stays off, so that is going well. Say if you want the meals tightened up."), py_str("{X} down and still going, which is the part most people lose. Nothing to change if it is still coming off slowly."), py_str("You have {x2} off since you started. That is on the good side of the pace, so keep the plan as it is unless it stalls.")]));
    s__NUMBER_WORDS(py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten"))]));
}
pub fn _tidy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut how: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let mut said: Py = Py::None;
    let _ = py_str("\"2kg\" -> \"2 kg\", \"half a stone\" -> \"Half a stone\".");
    said = mcall(&g_GLUED_UNIT(), "sub", &[{ let how = how.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let how = how.clone(); let said = said.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); op_add(&py_str(" "), &mcall(&m.clone(), "group", &[Py::Int(1)])) }) }, mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = how.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    said = mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    return if py_truthy(&said.clone()) { op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
    Py::None
}
pub fn _the_plan_to_lose(args: &[Py]) -> Py {
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
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut user: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("True when the chat holds an aim to lose weight, in the store's own words.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    user = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plan"), py_str("goal"), py_str("aim"), py_str("diet")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&user.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[value.clone()]);
    }
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("plans"), py_str("goals")]));
    for __item in __iter2 {
    key = __item.clone();
    value = mcall(&user.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut text: Py = __item.clone(); if py_truthy(&text.clone()) {  __out.push(mcall(&g_A_PLAN_TO_LOSE(), "search", &[text.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_turn_that_said_it(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("How many times the chat has been told about a loss, this line included.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_A_LOSS(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    }
    }
    return seen.clone();
    Py::None
}
pub fn the_loss_that_was_the_plan(args: &[Py]) -> Py {
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
    let mut how: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a loss said in a chat that was already trying to lose it, or \"\".");
    match_ = mcall(&g_A_LOSS(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_plan_to_lose(&[memory.clone()])))) {
    return py_str("");
    }
    how = _tidy(&[mcall(&match_.clone(), "group", &[py_str("how")])]);
    pool = if py_truthy(&op_le(&_the_turn_that_said_it(&[memory.clone()]), &Py::Int(1))) { g_FIRST() } else { g_SAID_BACK() };
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    reply = geti(&pool.clone(), &op_modulo(&call_builtin("len", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&Py::Bool(py_contains(&reply.clone(), &py_str("{X}")))) {
    reply = mcall(&reply.clone(), "replace", &[py_str("{X}"), how.clone()]);
    } else {
    reply = mcall(&reply.clone(), "replace", &[py_str("{x2}"), op_add(&mcall(&slice_or_raise(&how.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&how.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&geti(&memory.clone(), &py_str("user")), &py_str("progress"), py_dict(vec![(py_str("lost"), how.clone())]));
    }
    return reply.clone();
    Py::None
}
pub fn _the_progress(args: &[Py]) -> Py {
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
    let mut lost: Py = Py::None;
    let mut progress: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("The figure the chat holds for the plan in hand, or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    user = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    progress = if py_truthy(&call_builtin("isinstance", &[mcall(&user.clone(), "get", &[py_str("progress")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&user.clone(), "get", &[py_str("progress")]) } else { py_dict(vec![]) };
    lost = mcall(&call_builtin("str", &[{ let __b = mcall(&progress.clone(), "get", &[py_str("lost")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return if py_truthy(&{ let __b = lost.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("weight"), py_str("some weight")]), &mcall(&lost.clone(), "lower", &[]))) } }) { lost.clone() } else { py_str("") };
    Py::None
}
pub fn a_progress_asked(args: &[Py]) -> Py {
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
    let mut lost: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to \"how am i doing\" when the chat holds a figure for the plan, or \"\".\n\n    Nothing else answers it: without a figure on file the line is a general question about a diet\n    and the knowledge base's answer is the right one, so this returns \"\" and that answer stands.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HOW_GOING(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])))) {
    return py_str("");
    }
    lost = _the_progress(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&lost.clone()))) {
    return py_str("");
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    reply = geti(&g_PROGRESS_REPLY(), &op_modulo(&call_builtin("len", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_PROGRESS_REPLY()]).unwrap_or_else(|e| exc::raise(e))));
    return mcall(&mcall(&reply.clone(), "replace", &[py_str("{X}"), lost.clone()]), "replace", &[py_str("{x2}"), op_add(&mcall(&slice_or_raise(&lost.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&lost.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.goals".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_LOSS" => Some(g_A_LOSS()),
            "A_PLAN_TO_LOSE" => Some(g_A_PLAN_TO_LOSE()),
            "FIRST" => Some(g_FIRST()),
            "GLUED_UNIT" => Some(g_GLUED_UNIT()),
            "HOW_GOING" => Some(g_HOW_GOING()),
            "PROGRESS_REPLY" => Some(g_PROGRESS_REPLY()),
            "SAID_BACK" => Some(g_SAID_BACK()),
            "_NUMBER_WORDS" => Some(g__NUMBER_WORDS()),
            "_the_plan_to_lose" => Some(py_func("_the_plan_to_lose", |args: &[Py]| _the_plan_to_lose(args))),
            "_the_progress" => Some(py_func("_the_progress", |args: &[Py]| _the_progress(args))),
            "_the_turn_that_said_it" => Some(py_func("_the_turn_that_said_it", |args: &[Py]| _the_turn_that_said_it(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "a_progress_asked" => Some(py_func("a_progress_asked", |args: &[Py]| a_progress_asked(args))),
            "the_loss_that_was_the_plan" => Some(py_func("the_loss_that_was_the_plan", |args: &[Py]| the_loss_that_was_the_plan(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_LOSS" => s_A_LOSS(v),
            "A_PLAN_TO_LOSE" => s_A_PLAN_TO_LOSE(v),
            "FIRST" => s_FIRST(v),
            "GLUED_UNIT" => s_GLUED_UNIT(v),
            "HOW_GOING" => s_HOW_GOING(v),
            "PROGRESS_REPLY" => s_PROGRESS_REPLY(v),
            "SAID_BACK" => s_SAID_BACK(v),
            "_NUMBER_WORDS" => s__NUMBER_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.goals", module_obj);
    pyrt::xrefs::reg_fn("memory.goals", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_fn("memory.goals", "_the_plan_to_lose", py_func("_the_plan_to_lose", |args: &[Py]| _the_plan_to_lose(args)));
    pyrt::xrefs::reg_fn("memory.goals", "_the_turn_that_said_it", py_func("_the_turn_that_said_it", |args: &[Py]| _the_turn_that_said_it(args)));
    pyrt::xrefs::reg_fn("memory.goals", "the_loss_that_was_the_plan", py_func("the_loss_that_was_the_plan", |args: &[Py]| the_loss_that_was_the_plan(args)));
    pyrt::xrefs::reg_fn("memory.goals", "_the_progress", py_func("_the_progress", |args: &[Py]| _the_progress(args)));
    pyrt::xrefs::reg_fn("memory.goals", "a_progress_asked", py_func("a_progress_asked", |args: &[Py]| a_progress_asked(args)));
    pyrt::xrefs::reg_access("memory.goals", "A_LOSS", g_A_LOSS, s_A_LOSS);
    pyrt::xrefs::reg_access("memory.goals", "A_PLAN_TO_LOSE", g_A_PLAN_TO_LOSE, s_A_PLAN_TO_LOSE);
    pyrt::xrefs::reg_access("memory.goals", "FIRST", g_FIRST, s_FIRST);
    pyrt::xrefs::reg_access("memory.goals", "GLUED_UNIT", g_GLUED_UNIT, s_GLUED_UNIT);
    pyrt::xrefs::reg_access("memory.goals", "HOW_GOING", g_HOW_GOING, s_HOW_GOING);
    pyrt::xrefs::reg_access("memory.goals", "PROGRESS_REPLY", g_PROGRESS_REPLY, s_PROGRESS_REPLY);
    pyrt::xrefs::reg_access("memory.goals", "SAID_BACK", g_SAID_BACK, s_SAID_BACK);
    pyrt::xrefs::reg_access("memory.goals", "_NUMBER_WORDS", g__NUMBER_WORDS, s__NUMBER_WORDS);
}

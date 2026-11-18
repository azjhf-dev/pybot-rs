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


pub static A_A_TASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIRST_FENCED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIRST_NAMES_ITS_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIRST_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_A_TASK() -> Py { ensure_init(); A_A_TASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_A_TASK(v: Py) { ensure_init(); if let Ok(mut w) = A_A_TASK.write() { *w = v; } }
pub fn g_A_FIRST_FENCED() -> Py { ensure_init(); A_FIRST_FENCED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIRST_FENCED(v: Py) { ensure_init(); if let Ok(mut w) = A_FIRST_FENCED.write() { *w = v; } }
pub fn g_A_FIRST_NAMES_ITS_THING() -> Py { ensure_init(); A_FIRST_NAMES_ITS_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIRST_NAMES_ITS_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_FIRST_NAMES_ITS_THING.write() { *w = v; } }
pub fn g_A_FIRST_TIME() -> Py { ensure_init(); A_FIRST_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIRST_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_FIRST_TIME.write() { *w = v; } }
pub fn g_FIRST_TIME() -> Py { ensure_init(); FIRST_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_TIME(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_TIME.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A first time is a task the person is facing, not a first taste.\n\nTwo plain accounts that reached the floor. \"how do i give a talk at work\" then \"ive never done one\nbefore\" was answered \"I'm listening - say more if it helps\", and the person is standing in front of\nthe thing already. Nothing here needs to know what the first is: that the first one is the worst\none, and that the preparation people skip is saying it out loud once, is true of a talk, a race and\na first shift.\n\nA first that is a loss, a birth or a body is fenced off (grief and health own those), and so is a\nfirst taste (\"ive never had a curry before\"), which is a thing eaten rather than a thing faced.\n");
    s_A_FIRST_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:first time|first one|first ever|first go|first race|first shift|first day|never done (?:one|this|it|that) before|never done (?:one|this|it|that)|never (?:done|run|given|taught|led|organised|organized|played|performed)\\b[^?]*\\bbefore|never had one|havent done (?:one|this|it) before|haven't done (?:one|this|it) before|(?:its|it's|this is) my first|my first time|ive never|i've never|not done one before)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIRST_FENCED(stdlib_call("re", "compile", &[py_str("\\b(?:died|death|dying|dead|loss|lost|grief|bereave\\w*|funeral|miscarriage|stillborn|cancer|hospice|hospital|operation|surgery|diagnos\\w*|chemo|baby|babies|pregnan\\w*|birth|numb|passed away|put to sleep|euthanis\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_A_TASK(stdlib_call("re", "compile", &[py_str("\\b(?:talk|speech|presentation|pitch|demo|interview|viva|defence|defense|panel|race|marathon|half marathon|10k|5k|parkrun|triathlon|swim|match|game|final|heat|test|exam|driving test|theory test|practical|audition|performance|gig|show|shift|first day|day one|lesson|class|course|lecture|seminar|reading|sermon|wedding speech|best man|maid of honour|toast|eulogy|chair|chairing|meeting|appointment|scan|procedure|surgery|flight|driving lesson|lesson)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIRST_NAMES_ITS_THING(stdlib_call("re", "compile", &[py_str("\\b(?:had|eaten|tried|cooked|made|bought|seen|visited|flown|driven|used|caught|owned|worn|grown|sold|wrote|read|played|kept|baked|brewed)\\s+(?:a|an|the|any|some)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_TIME(py_tuple(vec![py_str("The first one is the one you over-prepare for, and that is the right mistake to make. Say it out loud once, all the way through, to a person or the wall - that is the step people skip and the one that takes the nerves down. What is the part that worries you most?"), py_str("Nobody is calm for their first one, and the nerves go in the first two minutes once you are actually in it. Walk it through out loud once beforehand rather than reading it over again in your head. What is it you are facing?"), py_str("Everybody's first is the worst one, and almost nobody does it as badly as they think. Practise it once, out loud and timed, and know that the beginning is the part to have ready. What would go wrong if it went wrong?")]));
}
pub fn _a_task_they_faced(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the first they are talking about is a task, in this line or the one before.");
    if py_truthy(&mcall(&g_A_A_TASK(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_A_FIRST_NAMES_ITS_THING(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if !py_truthy(&__b) { __b } else { mcall(&g_A_A_TASK(), "search", &[call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    else { Py::None };
    let mut low: Py = Py::None;
    let _ = py_str("The one thing that helps a first time, or None for the floor to answer as before.");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&low.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_FIRST_TIME(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_FIRST_FENCED(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_task_they_faced(&[low.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])))) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[g_FIRST_TIME(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, low.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_first_time".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_A_TASK" => Some(g_A_A_TASK()),
            "A_FIRST_FENCED" => Some(g_A_FIRST_FENCED()),
            "A_FIRST_NAMES_ITS_THING" => Some(g_A_FIRST_NAMES_ITS_THING()),
            "A_FIRST_TIME" => Some(g_A_FIRST_TIME()),
            "FIRST_TIME" => Some(g_FIRST_TIME()),
            "_a_task_they_faced" => Some(py_func("_a_task_they_faced", |args: &[Py]| _a_task_they_faced(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

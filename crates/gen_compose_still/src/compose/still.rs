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


thread_local! { static G_HARD_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUDGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORROW_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_AFTER_A_HARD_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_AFTER_A_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPATHY_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_HARD_MODULES() -> Py { ensure_init(); G_HARD_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HARD_MODULES(v: Py) { ensure_init(); G_HARD_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING() -> Py { ensure_init(); G_LEADING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING(v: Py) { ensure_init(); G_LEADING.with(|c| *c.borrow_mut() = v); }
pub fn g_NUDGES() -> Py { ensure_init(); G_NUDGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUDGES(v: Py) { ensure_init(); G_NUDGES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_BACK() -> Py { ensure_init(); G_SHORT_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_BACK(v: Py) { ensure_init(); G_SHORT_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_SORROW_MODULES() -> Py { ensure_init(); G_SORROW_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORROW_MODULES(v: Py) { ensure_init(); G_SORROW_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_AFTER_A_HARD_DAY() -> Py { ensure_init(); G_STILL_AFTER_A_HARD_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_AFTER_A_HARD_DAY(v: Py) { ensure_init(); G_STILL_AFTER_A_HARD_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_AFTER_A_LOSS() -> Py { ensure_init(); G_STILL_AFTER_A_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_AFTER_A_LOSS(v: Py) { ensure_init(); G_STILL_AFTER_A_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_END() -> Py { ensure_init(); G_STILL_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_END(v: Py) { ensure_init(); G_STILL_END.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPATHY_OPEN() -> Py { ensure_init(); G_SYMPATHY_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPATHY_OPEN(v: Py) { ensure_init(); G_SYMPATHY_OPEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A nudge after an answer is not the end of the conversation.\n\nA person who says \"and\" or \"go on\" after being told what to do is still in it. On main that line\nis read as the conversation closing, or as an echo of the answer's first sentence:\n\n    my knee is sore when i run / and            \"Alright then - good luck with it.\"\n    my knee is sore when i run / and            \"Thanks for telling me - I'm here if you want...\"\n    how do i clean my oven / and then           \"That's the one: Make a paste of bicarbonate...\"\n    i want to start running / and               \"Alright then - good luck with it.\"\n\nThe first two close a conversation the person is still in, and the third hands back the sentence\nthey have just read. This module does not hand anything back: an answer that is already in front of\nthem has been read, so repeating it as \"the other part\" is the bot talking to itself. What a nudge\ngets instead is the honest end of the answer, with the offer that is missing from a goodbye - the\nperson can go back over any part of it.\n\nSo the three faults are answered with one rule: **nothing is ever repeated, and the turn is not\nclosed.** That is why the reading here is small. It has to know a nudge from a line with a subject\n(\"and the radiator\" is a question about the radiator), and it has to know that there was an answer\nbehind it that gave something to do; everything else stands down and the floor answers as it did\nbefore.\n\nThe wording lives next to the reading in `bot/compose/still.py`, and `bot/modules/nudge.py` gives\nit a band of its own, because a nudge has to be answered before the chat module's goodbye.\n");
    s_NUDGES(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("then"), py_str("and then"), py_str("the rest"), py_str("and the rest")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("ok"), py_str("okay"), py_str("right"), py_str("well"), py_str("so"), py_str("yes"), py_str("yeah"), py_str("and"), py_str("but"), py_str("then"), py_str("alright"), py_str("cool"), py_str("great"), py_str("thanks"), py_str("ta"), py_str("ah"), py_str("oh")]) }
    s_LEADING(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT_BACK(Py::Int(120));
    s_SYMPATHY_OPEN(py_tuple(vec![py_str("sorry to hear"), py_str("so sorry"), py_str("i'm sorry"), py_str("im sorry"), py_str("that sounds"), py_str("awful"), py_str("that's tough"), py_str("thats tough"), py_str("that's such"), py_str("thats such"), py_str("how awful")]));
    s_STILL_END(py_tuple(vec![py_str("That's the lot of it from me on that one. Say if you want to go back over any part of it."), py_str("That's everything I have on it, and it is all above. Which bit do you want to unpick?"), py_str("That's the whole of it as far as I know. Is there a part of it you are unsure about?"), py_str("Nothing more from me on that one - it is all said. Anything in it worth going over?"), py_str("That's as far as I can take it. Say the word and we will go back over a piece of it.")]));
    s_STILL_AFTER_A_LOSS(py_tuple(vec![py_str("No rush on any of it. If you want to say more about them, I am here for that."), py_str("Take your time - nothing here needs finishing. Say more about them whenever you like."), py_str("That's all from me, and there is no hurry. I am still here if you want to talk about them.")]));
    s_STILL_AFTER_A_HARD_DAY(py_tuple(vec![py_str("No rush with it. I am still here if you want to say more about how things are."), py_str("Nothing to finish here - take your time. Say more whenever you want to."), py_str("That's all from me for now, and no hurry at all. I am here if you want to keep going.")]));
    s_SORROW_MODULES(py_tuple(vec![py_str("loss"), py_str("grief")]));
    s_HARD_MODULES(py_tuple(vec![py_str("safety"), py_str("urgent")]));
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn _is_a_nudge(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut flat: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "flat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: flat"))) };
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a line that only says \"go on\", with any \"ok\"/\"so\" in front of it taken off.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&flat.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&flat.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&flat.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:-")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&g_NUDGES(), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() }])))) {
    return Py::Bool(true);
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADING(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    return Py::Bool(py_contains(&g_NUDGES(), &mcall(&py_str(" "), "join", &[words.clone()])));
    Py::None
}
pub fn _ours(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut head: Py = Py::None;
    let mut low: Py = Py::None;
    let mut template: Py = Py::None;
    let _ = py_str("True for a line this module wrote: the floor's end line is not the answer.");
    low = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_STILL_END());
    for __item in __iter1 {
    template = __item.clone();
    head = mcall(&mcall(&geti(&mcall(&template.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { mcall(&low.clone(), "startswith", &[head.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _answer_behind(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut walked: Py = Py::None;
    let _ = py_str("Whether the last thing the bot said gave the person something to do, and what it was.\n\n    The walk back goes over this module's own end lines, nods and sign-offs, and stops at a\n    condolence or an emergency: \"that's the lot of it\" is not a thing to say after either, and a\n    line like that is not a list of steps to be finished with. A question the bot asked is not an\n    answer either - a nudge after it is a person thinking, not a person asking for more of a list.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __try1 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    walked = call_builtin("list", &[call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter6 = py_iter_or_raise(&call_builtin("enumerate", &[walked.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    turn = __unpk7[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("compose.verdict", "UNREADABLE"), &mcall(&turn.clone(), "get", &[py_str("module")])))) {
    return py_str("");
    }
    said = mcall(&turn.clone(), "get", &[py_str("bot")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { _ours(&[said.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("greeting")))) {
    if py_truthy(&mcall(&mcall(&said.clone(), "strip", &[]), "endswith", &[py_str("?")])) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_continues_an_answer(&[slice_or_raise(&walked.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)])))) {
    return py_str("");
    }
    continue;
    }
    if py_truthy(&mcall(&mcall(&said.clone(), "strip", &[]), "endswith", &[py_str("?")])) {
    return py_str("");
    }
    if py_truthy(&mcall(&_flat(&[said.clone()]), "startswith", &[g_SYMPATHY_OPEN()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("memory_store"))); if !py_truthy(&__b) { __b } else { _a_hand_back(&[said.clone()]) } }) {
    return said.clone();
    }
    return if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.verdict", "_steps", &[said.clone()]); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[stdlib_call("re", "split", &[py_str("(?<=[.!?]) "), mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) { said.clone() } else { py_str("") };
    }
    return py_str("");
    Py::None
}
pub fn _continues_an_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut behind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "behind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: behind"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when one of the turns behind this floor line is an answer with something in it.\n\n    The raw test, not `_answer_behind`: this asks only whether the floor line was spoken for an\n    answer at all, so a question the bot asked and a sympathy opener are answers here - the walk\n    carries on and `_answer_behind` applies its own tests to whichever line it lands on.\n    ");
    let __iter1 = py_iter_or_raise(&behind.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_add(&py_tuple(vec![py_str("greeting")]), &call_builtin("tuple", &[pyrt::xrefs::xget("compose.verdict", "UNREADABLE")]).unwrap_or_else(|e| exc::raise(e))), &mcall(&turn.clone(), "get", &[py_str("module")]))) } }) {
    continue;
    }
    said = mcall(&turn.clone(), "get", &[py_str("bot")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_hand_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut repeat: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the memory store's line is part of an answer rather than a nod of its own.\n\n    The store writes both kinds: \"If you only do one thing from that answer: Ice after a run and\n    run on softer ground...\" hands over a sentence of the answer, while \"Got it - they're 4 and 7\"\n    is a nod. Only the first is something a nudge can be the end of, and the test is the length it\n    hands over rather than the lead in front of it, because the lead-in list is the store's to\n    change and this module should not keep a copy of it.\n    ");
    text = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    repeat = pyrt::xrefs::xmodule("memory.repeat");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&gattr(&repeat.clone(), "SENTENCE"), "split", &[mcall(&text.clone(), "strip", &[])])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
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
    return { let __b = call_builtin("bool", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } };
    Py::None
}
pub fn _sorrow_behind(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the last thing the bot said was a condolence, and which kind: \"loss\", \"hard\" or \"\".\n\n    Read off the turn itself and not off the answer's worth: a death answered by the loss module,\n    or any answer that opens by taking the hard thing in. An emergency is neither - \"say more\n    whenever you want to\" is the wrong line while somebody is waiting for an ambulance.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_hedged_recently", &[memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    module = mcall(&last.clone(), "get", &[py_str("module")]);
    if py_truthy(&Py::Bool(py_contains(&g_HARD_MODULES(), &module.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_SORROW_MODULES(), &module.clone()))) {
    return py_str("loss");
    }
    said = mcall(&last.clone(), "get", &[py_str("bot")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&_flat(&[said.clone()]), "startswith", &[g_SYMPATHY_OPEN()]) } }) {
    return py_str("hard");
    }
    return py_str("");
    Py::None
}
pub fn _the_rest_is_anothers(args: &[Py]) -> Py {
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
    let _ = py_str("True when the module that owns the rest of an answer reads this line as asking for it.\n\n    `bot/memory/another.py` is the module that hands back an answer's second half, and it reads a\n    carry-on the same way this one does. Where it speaks it is the better answer - it gives the\n    rest of the answer where this module can only say there is no rest - so it keeps the line, and\n    the reading is asked for at call time rather than copied: whatever that module claims\n    tomorrow, this one stands down on, with no second list here to keep in step.\n\n    What is asked is whether it *has* the rest, not whether it reads the line as a carry-on: a\n    condolence has no second half in it, and a line that is read as a carry-on anyway is not a\n    reason to give up the turn. The loss threads are nudge's own sign-off (\"That's all from me,\n    and there is no hurry\") either way, which is the pool this module leaves to it.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&pyrt::xrefs::xcall("memory.another", "other_option", &[clean.clone(), memory.clone()]), &Py::None))]);
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
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("Only on a nudge with an answer behind it: the line is read whole, and nowhere else.");
    let __try1 = exc::catch(|| {
    if py_truthy(&_the_rest_is_anothers(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = _is_a_nudge(&[_flat(&[text.clone()])]); if !py_truthy(&__b) { __b } else { { let __b = _answer_behind(&[memory.clone()]); if py_truthy(&__b) { __b } else { _sorrow_behind(&[memory.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pool: Py = Py::None;
    let mut sorrow: Py = Py::None;
    let _ = py_str("The honest end of the answer for a nudge that names nothing of its own. Else None.");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&takes_as_it_comes(&[clean.clone(), memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    sorrow = _sorrow_behind(&[memory.clone()]);
    pool = mcall(&py_dict(vec![(py_str("loss"), g_STILL_AFTER_A_LOSS()), (py_str("hard"), g_STILL_AFTER_A_HARD_DAY())]), "get", &[sorrow.clone(), g_STILL_END()]);
    return py_tuple(vec![Py::Bool(true), pyrt::xrefs::xcall("compose.verdict", "_say", &[pool.clone(), memory.clone(), _flat(&[clean.clone()])])]);
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
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.still".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "HARD_MODULES" => Some(g_HARD_MODULES()),
            "LEADING" => Some(g_LEADING()),
            "NUDGES" => Some(g_NUDGES()),
            "SHORT_BACK" => Some(g_SHORT_BACK()),
            "SORROW_MODULES" => Some(g_SORROW_MODULES()),
            "STILL_AFTER_A_HARD_DAY" => Some(g_STILL_AFTER_A_HARD_DAY()),
            "STILL_AFTER_A_LOSS" => Some(g_STILL_AFTER_A_LOSS()),
            "STILL_END" => Some(g_STILL_END()),
            "SYMPATHY_OPEN" => Some(g_SYMPATHY_OPEN()),
            "_a_hand_back" => Some(py_func("_a_hand_back", |args: &[Py]| _a_hand_back(args))),
            "_answer_behind" => Some(py_func("_answer_behind", |args: &[Py]| _answer_behind(args))),
            "_continues_an_answer" => Some(py_func("_continues_an_answer", |args: &[Py]| _continues_an_answer(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_is_a_nudge" => Some(py_func("_is_a_nudge", |args: &[Py]| _is_a_nudge(args))),
            "_ours" => Some(py_func("_ours", |args: &[Py]| _ours(args))),
            "_sorrow_behind" => Some(py_func("_sorrow_behind", |args: &[Py]| _sorrow_behind(args))),
            "_the_rest_is_anothers" => Some(py_func("_the_rest_is_anothers", |args: &[Py]| _the_rest_is_anothers(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "HARD_MODULES" => s_HARD_MODULES(v),
            "LEADING" => s_LEADING(v),
            "NUDGES" => s_NUDGES(v),
            "SHORT_BACK" => s_SHORT_BACK(v),
            "SORROW_MODULES" => s_SORROW_MODULES(v),
            "STILL_AFTER_A_HARD_DAY" => s_STILL_AFTER_A_HARD_DAY(v),
            "STILL_AFTER_A_LOSS" => s_STILL_AFTER_A_LOSS(v),
            "STILL_END" => s_STILL_END(v),
            "SYMPATHY_OPEN" => s_SYMPATHY_OPEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.still", module_obj);
    pyrt::xrefs::reg_fn("compose.still", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("compose.still", "_is_a_nudge", py_func("_is_a_nudge", |args: &[Py]| _is_a_nudge(args)));
    pyrt::xrefs::reg_fn("compose.still", "_ours", py_func("_ours", |args: &[Py]| _ours(args)));
    pyrt::xrefs::reg_fn("compose.still", "_answer_behind", py_func("_answer_behind", |args: &[Py]| _answer_behind(args)));
    pyrt::xrefs::reg_fn("compose.still", "_continues_an_answer", py_func("_continues_an_answer", |args: &[Py]| _continues_an_answer(args)));
    pyrt::xrefs::reg_fn("compose.still", "_a_hand_back", py_func("_a_hand_back", |args: &[Py]| _a_hand_back(args)));
    pyrt::xrefs::reg_fn("compose.still", "_sorrow_behind", py_func("_sorrow_behind", |args: &[Py]| _sorrow_behind(args)));
    pyrt::xrefs::reg_fn("compose.still", "_the_rest_is_anothers", py_func("_the_rest_is_anothers", |args: &[Py]| _the_rest_is_anothers(args)));
    pyrt::xrefs::reg_fn("compose.still", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("compose.still", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.still", "HARD_MODULES", g_HARD_MODULES, s_HARD_MODULES);
    pyrt::xrefs::reg_access("compose.still", "LEADING", g_LEADING, s_LEADING);
    pyrt::xrefs::reg_access("compose.still", "NUDGES", g_NUDGES, s_NUDGES);
    pyrt::xrefs::reg_access("compose.still", "SHORT_BACK", g_SHORT_BACK, s_SHORT_BACK);
    pyrt::xrefs::reg_access("compose.still", "SORROW_MODULES", g_SORROW_MODULES, s_SORROW_MODULES);
    pyrt::xrefs::reg_access("compose.still", "STILL_AFTER_A_HARD_DAY", g_STILL_AFTER_A_HARD_DAY, s_STILL_AFTER_A_HARD_DAY);
    pyrt::xrefs::reg_access("compose.still", "STILL_AFTER_A_LOSS", g_STILL_AFTER_A_LOSS, s_STILL_AFTER_A_LOSS);
    pyrt::xrefs::reg_access("compose.still", "STILL_END", g_STILL_END, s_STILL_END);
    pyrt::xrefs::reg_access("compose.still", "SYMPATHY_OPEN", g_SYMPATHY_OPEN, s_SYMPATHY_OPEN);
}

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


thread_local! { static G_AFTER_THE_CALL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGREES_IT_IS_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CALLS_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EMERGENCY_CASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HELP_IS_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.safety_doubt: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AFTER_THE_CALL_LINES() -> Py { ensure_init(); G_AFTER_THE_CALL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_CALL_LINES(v: Py) { ensure_init(); G_AFTER_THE_CALL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_LINES() -> Py { ensure_init(); G_YES_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_LINES(v: Py) { ensure_init(); G_YES_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__AGREES_IT_IS_BAD() -> Py { ensure_init(); G__AGREES_IT_IS_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGREES_IT_IS_BAD(v: Py) { ensure_init(); G__AGREES_IT_IS_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g__CALLS_SOMEBODY() -> Py { ensure_init(); G__CALLS_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CALLS_SOMEBODY(v: Py) { ensure_init(); G__CALLS_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINUATION() -> Py { ensure_init(); G__CONTINUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINUATION(v: Py) { ensure_init(); G__CONTINUATION.with(|c| *c.borrow_mut() = v); }
pub fn g__EMERGENCY_CASES() -> Py { ensure_init(); G__EMERGENCY_CASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EMERGENCY_CASES(v: Py) { ensure_init(); G__EMERGENCY_CASES.with(|c| *c.borrow_mut() = v); }
pub fn g__HELP_IS_COMING() -> Py { ensure_init(); G__HELP_IS_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HELP_IS_COMING(v: Py) { ensure_init(); G__HELP_IS_COMING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A bare doubt said on top of a safety answer is about the emergency itself.\n\nThe safety card is the bot's own answer, and a bare verdict ask after it (\"is that bad\") is the\nperson checking that the thing is as serious as it sounded. Three different voices answered that\ninstead, each of them written for a different thread:\n\n    i cant breathe / is that bad       \"It is worth having it seen to, and that answer already\n                                        said by whom. Get that booked\" - an ambulance turned into\n                                        a clinic appointment\n    i hit my wife / is that bad        \"Keep an eye on it and tell me if anything changes; being\n                                        seen early beats waiting\" - an admission of violence\n                                        answered as a symptom\n    my husband hit me / is that bad    \"Keep someone with you tonight, and ring back if it gets\n                                        heavier\" - somebody else's crisis, said to a person who is\n                                        being hit, by the person they live with\n\nThe continuations in `bot/compose/fallback.py` (`_after_safety`) are written for lines *inside* an\nemergency - \"ok i rang them\", \"the ambulance is here\", \"i feel a bit better\" - and a verdict ask\nmatches none of them, so it reaches the care pool of a symptom or the crisis pool of somebody else.\nThe yes here is the answer's own, said back, with the call still the next thing to do.\n\nNot every answer from `bot/modules/safety.py` is an emergency. The same module hands out the\nfirst-aid cards - ice on a swollen ankle, a burn under the tap, pinch a nosebleed - and the triage\ncards, which name 999 or A&E as the *test* for how bad a thing is, not as the thing to do now. A\nbare doubt after those is a different question, and one the floor already answers (\"Ordinary on its\nown, yes. It stops being ordinary if...\"), so this reading only fires when the card itself says an\nemergency: the case the safety module recorded is one it only records for emergencies, and the\ncard names somebody to ring now.\n");
    s_YES_LINES(py_tuple(vec![py_str("Yes - it is serious, and that is why ringing is the thing to do now rather than something to sleep on. Tell me how it goes."), py_str("Yes. Ringing is not an overreaction here: it is the whole point of the line I gave you. Do that first, and come back to me after."), py_str("Yes it is, and the number I gave you is what it is for. Make the call, and we can go through the rest after."), py_str("Yes - this is not one to wait and see about. Do the call now, and come back to me when you have.")]));
    s__CONTINUATION(py_tuple(vec![py_str("greeting"), py_str("already")]));
    fn __data1() -> Py { py_tuple(vec![py_str("abuse"), py_str("bleeding"), py_str("breathing"), py_str("chest"), py_str("chest_slow"), py_str("dog_attack"), py_str("dog_bite"), py_str("fire"), py_str("gas"), py_str("general"), py_str("hanging"), py_str("head_injury"), py_str("heat"), py_str("overdose"), py_str("poison"), py_str("seizure"), py_str("self_harm"), py_str("stroke"), py_str("swallowed_thing"), py_str("unconscious")]) }
    s__EMERGENCY_CASES(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_THE_CALL_LINES(py_tuple(vec![py_str("Still yes - and you have made the call, which was the part that mattered. Stay with what they tell you, and say if anything changes."), py_str("Still yes, and it is with people who can do something about it now. Do what they say over anything I could offer, and tell me how it goes."), py_str("Yes it is, and the call was the thing to do - you have done it. Whatever they told you to do next is what matters now.")]));
    s__CALLS_SOMEBODY(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|ambulance|paramedics|emergency\\s+services|116\\s?123|988|0808|0800|1-?800)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__HELP_IS_COMING(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they)\\s+(?:have\\s+|had\\s+|has\\s+)?(?:already\\s+)?(?:rang|phoned|called)\\b|\\brang\\s+(?:them|999|911|112|an\\s+ambulance|the\\s+(?:police|line))\\b|\\b(?:on\\s+(?:their|its|the)\\s+way|are\\s+coming|is\\s+coming|are\\s+here|is\\s+here|has\\s+arrived|have\\s+arrived|the\\s+crew|paramedics|the\\s+operator|the\\s+emergency\\s+line|fire\\s+brigade|emergency\\s+line\\s+says|they\\s+said)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AGREES_IT_IS_BAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:that|thats|that'?s|this|it|its|it'?s)\\s+(?:is\\s+|was\\s+)?(?:really\\s+|pretty\\s+|very\\s+|so\\s+|proper\\s+|well\\s+)?(?:bad|awful|terrible|serious|grim|horrible|scary|frightening)\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_answer_behind(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The safety answer the doubt is about and what was said after it, or (None, []).\n\n    Walks back past this bot's own continuations to the turn that started the thread, and stops at\n    any other module's answer: once something else has answered, the doubt is about that, and this\n    reading has nothing to say. Capped a few turns back so an emergency from earlier in the talk\n    cannot colour a line today.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    after = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("safety")))) {
    let _ = mcall(&after.clone(), "append", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    return py_tuple(vec![mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), after.clone()]);
    }
    if py_truthy(&Py::Bool(!py_contains(&g__CONTINUATION(), &mcall(&turn.clone(), "get", &[py_str("module")])))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    said = mcall(&py_str(" "), "join", &[py_tuple(vec![call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]);
    let _ = mcall(&after.clone(), "append", &[mcall(&said.clone(), "lower", &[])]);
    }
    return py_tuple(vec![Py::None, py_list(vec![])]);
    Py::None
}
pub fn _a_bare_verdict_ask(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut read: Py = Py::None;
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&read.clone(), "get", &[py_str("asks_a_verdict")]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__AGREES_IT_IS_BAD(), "match", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[pyrt::xrefs::xcall("compose.fallback", "reads_as_a_verdict", &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    let mut after: Py = Py::None;
    let mut behind: Py = Py::None;
    let mut c: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut line: Py = Py::None;
    let mut options: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut t: Py = Py::None;
    let _ = py_str("The answer's own yes to a bare doubt in a safety thread, or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_answer_behind(&[memory.clone()]), 2, None);
    behind = __unpk1[0].clone();
    after = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&behind.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g__EMERGENCY_CASES(), &mcall(&memory.clone(), "get", &[py_str("safety_case")])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&mcall(&memory.clone(), "get", &[py_str("safety_safe")]), &Py::Bool(true)))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__CALLS_SOMEBODY(), "search", &[behind.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_bare_verdict_ask(&[text.clone(), memory.clone(), clean.clone()])))) {
    return Py::None;
    }
    pool = if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut said: Py = __item.clone(); __out.push(mcall(&g__HELP_IS_COMING(), "search", &[said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { g_AFTER_THE_CALL_LINES() } else { g_YES_LINES() };
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(5)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(mcall(&t.clone(), "get", &[py_str("bot")])); } py_list(__out) })() }, &line.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&options.clone()))) {
    options = call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    seed = op_add(&op_mul(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&clean.clone(), &behind.clone())) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[options.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.safety_doubt".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_THE_CALL_LINES" => Some(g_AFTER_THE_CALL_LINES()),
            "YES_LINES" => Some(g_YES_LINES()),
            "_AGREES_IT_IS_BAD" => Some(g__AGREES_IT_IS_BAD()),
            "_CALLS_SOMEBODY" => Some(g__CALLS_SOMEBODY()),
            "_CONTINUATION" => Some(g__CONTINUATION()),
            "_EMERGENCY_CASES" => Some(g__EMERGENCY_CASES()),
            "_HELP_IS_COMING" => Some(g__HELP_IS_COMING()),
            "_a_bare_verdict_ask" => Some(py_func("_a_bare_verdict_ask", |args: &[Py]| _a_bare_verdict_ask(args))),
            "_the_answer_behind" => Some(py_func("_the_answer_behind", |args: &[Py]| _the_answer_behind(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "random" => Some(stdlib_module("random")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFTER_THE_CALL_LINES" => s_AFTER_THE_CALL_LINES(v),
            "YES_LINES" => s_YES_LINES(v),
            "_AGREES_IT_IS_BAD" => s__AGREES_IT_IS_BAD(v),
            "_CALLS_SOMEBODY" => s__CALLS_SOMEBODY(v),
            "_CONTINUATION" => s__CONTINUATION(v),
            "_EMERGENCY_CASES" => s__EMERGENCY_CASES(v),
            "_HELP_IS_COMING" => s__HELP_IS_COMING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.safety_doubt", module_obj);
    pyrt::xrefs::reg_fn("compose.safety_doubt", "_the_answer_behind", py_func("_the_answer_behind", |args: &[Py]| _the_answer_behind(args)));
    pyrt::xrefs::reg_fn("compose.safety_doubt", "_a_bare_verdict_ask", py_func("_a_bare_verdict_ask", |args: &[Py]| _a_bare_verdict_ask(args)));
    pyrt::xrefs::reg_fn("compose.safety_doubt", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.safety_doubt", "AFTER_THE_CALL_LINES", g_AFTER_THE_CALL_LINES, s_AFTER_THE_CALL_LINES);
    pyrt::xrefs::reg_access("compose.safety_doubt", "YES_LINES", g_YES_LINES, s_YES_LINES);
    pyrt::xrefs::reg_access("compose.safety_doubt", "_AGREES_IT_IS_BAD", g__AGREES_IT_IS_BAD, s__AGREES_IT_IS_BAD);
    pyrt::xrefs::reg_access("compose.safety_doubt", "_CALLS_SOMEBODY", g__CALLS_SOMEBODY, s__CALLS_SOMEBODY);
    pyrt::xrefs::reg_access("compose.safety_doubt", "_CONTINUATION", g__CONTINUATION, s__CONTINUATION);
    pyrt::xrefs::reg_access("compose.safety_doubt", "_EMERGENCY_CASES", g__EMERGENCY_CASES, s__EMERGENCY_CASES);
    pyrt::xrefs::reg_access("compose.safety_doubt", "_HELP_IS_COMING", g__HELP_IS_COMING, s__HELP_IS_COMING);
}

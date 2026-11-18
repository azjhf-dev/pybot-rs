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


thread_local! { static G_ENOUGH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AIM_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DOCTOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TEMPERATURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORD_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONVERT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOURS_AND_MINUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERIOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOO_FAR_ABOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOO_FAR_BELOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENOUGH_ASK() -> Py { ensure_init(); G_ENOUGH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENOUGH_ASK(v: Py) { ensure_init(); G_ENOUGH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MORE() -> Py { ensure_init(); G_HOW_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MORE(v: Py) { ensure_init(); G_HOW_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__AIM_CUE() -> Py { ensure_init(); G__AIM_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AIM_CUE(v: Py) { ensure_init(); G__AIM_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNTS() -> Py { ensure_init(); G__AMOUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNTS(v: Py) { ensure_init(); G__AMOUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT_WORDS() -> Py { ensure_init(); G__AMOUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT_WORDS(v: Py) { ensure_init(); G__AMOUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DOCTOR() -> Py { ensure_init(); G__A_DOCTOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DOCTOR(v: Py) { ensure_init(); G__A_DOCTOR.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIMIT() -> Py { ensure_init(); G__A_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIMIT(v: Py) { ensure_init(); G__A_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERIOD() -> Py { ensure_init(); G__A_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERIOD(v: Py) { ensure_init(); G__A_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TEMPERATURE() -> Py { ensure_init(); G__A_TEMPERATURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TEMPERATURE(v: Py) { ensure_init(); G__A_TEMPERATURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORD_AFTER() -> Py { ensure_init(); G__A_WORD_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORD_AFTER(v: Py) { ensure_init(); G__A_WORD_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__CONVERT() -> Py { ensure_init(); G__CONVERT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONVERT(v: Py) { ensure_init(); G__CONVERT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORD() -> Py { ensure_init(); G__COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORD(v: Py) { ensure_init(); G__COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__HOURS_AND_MINUTES() -> Py { ensure_init(); G__HOURS_AND_MINUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOURS_AND_MINUTES(v: Py) { ensure_init(); G__HOURS_AND_MINUTES.with(|c| *c.borrow_mut() = v); }
pub fn g__PERIOD_WORDS() -> Py { ensure_init(); G__PERIOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERIOD_WORDS(v: Py) { ensure_init(); G__PERIOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TOO_FAR_ABOVE() -> Py { ensure_init(); G__TOO_FAR_ABOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOO_FAR_ABOVE(v: Py) { ensure_init(); G__TOO_FAR_ABOVE.with(|c| *c.borrow_mut() = v); }
pub fn g__TOO_FAR_BELOW() -> Py { ensure_init(); G__TOO_FAR_BELOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOO_FAR_BELOW(v: Py) { ensure_init(); G__TOO_FAR_BELOW.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_SAID() -> Py { ensure_init(); G__UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_SAID(v: Py) { ensure_init(); G__UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKLY() -> Py { ensure_init(); G__WEEKLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKLY(v: Py) { ensure_init(); G__WEEKLY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whether the answer already on the table is the amount the person asked about.\n\n\"is that enough\", \"how much more do i need\", \"is 300 too much\" - the figure in the answer and the\nfigure the person said have to be put side by side before anything can be said about them, and the\nunits have to match (a week of something is not a month of it). No other reader in the store needs\nany of it: `enough_reply` is the whole surface, and it returns None for every other line.\n\nMoved out of `memory_store` whole, constants and all, because that file is over the 1 MB the Hub\nwill accept in a pull request: see `bot/memory/times.py` for the first move of this kind.\n");
    s_NUMBER_WORDS(crate::memory::store_lines::g_NUMBER_WORDS());
    s__AMOUNTS(crate::memory::store_lines::g__AMOUNTS());
    s__AMOUNT_WORDS(crate::memory::store_lines::g__AMOUNT_WORDS());
    s__COUNT_WORD(crate::memory::store_lines::g__COUNT_WORD());
    s__CONVERT(crate::memory::store_lines::g__CONVERT());
    s__UNIT_SAID(crate::memory::store_lines::g__UNIT_SAID());
    s__WEEKLY(crate::memory::store_lines::g__WEEKLY());
    s_ENOUGH_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis (?:that|this|it) (?:enough|too much|too little|too often|about right|alright|ok|okay|fine|any good)\\b|\\bam i (?:doing|getting|eating|drinking|having|sleeping) enough\\b|\\bis that (?:a )?(?:good|healthy|normal|right) amount\\b|\\b(?:will|would|does) that (?:be )?(?:enough|work|do|help)\\b|\\bis\\s+(?:\\d[\\d,]*(?:\\.\\d+)?|once|twice|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)(?:[ -][a-z]{2,12}){0,2}\\s+(?:enough|too much|too often|too little|about right|alright|ok|okay|fine)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_MORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:much|many) (?:more|extra)\\b|\\bwhat (?:else|more) (?:do|should) i (?:need|add)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DOCTOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:see|talk to|speak to|ask|call)\\s+(?:a|your|the|nhs|111)?\\s*(?:gp|doctor|nurse|pharmacist|midwife|dentist|111)\\b|\\bworth (?:a |an |the )?(?:gp|doctor|nurse|check|visit|test|blood test|check-up|checking|seeing|getting|mentioning)\\b|\\bget (?:it|them|this) checked\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_LIMIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\blimits?\\b|\\bno more than\\b|\\bnot (?:more|much more) than\\b|\\bat most\\b|\\bcap(?:ped)?\\b|\\bcut (?:back|down)\\b|\\bkeep (?:it|them) (?:under|below)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_TEMPERATURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d{2,3}\\s*(?:\\u00b0\\s*)?(?:c|degrees?|gas mark)\\b|\\buntil (?:the juices|it is cooked|browned|piping hot)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HOURS_AND_MINUTES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:hours?|hrs?)\\s+(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:minutes?|mins?)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PERIOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:a|an|per|each|every|most)\\s+(day|week|night|month)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TOO_FAR_ABOVE(Py::Float(3.5));
    s__TOO_FAR_BELOW(Py::Float(20.0));
    s__AIM_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:aim (?:for|at)|aims? (?:for|at)|target(?:s|ing)?\\b|guideline|should (?:be|get|do|have)|try (?:for|to)|recommended|the idea is)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PERIOD_WORDS(py_set(vec![py_str("day"), py_str("days"), py_str("week"), py_str("weeks"), py_str("night"), py_str("nights"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("time"), py_str("times")]));
    s__A_WORD_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\s+[a-z]+(?:-[a-z]+)?){1,4}")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _a_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The number a word stands for, from a digit or a word, or 0 when it is neither.");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+(?:,\\d{3})*(?:\\.\\d+)?")); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__AMOUNT_WORDS(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _amount_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut family: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The family a unit word belongs to, or \"\" when it is not one we can compare.");
    let __iter1 = py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "strip", args))(&__args) }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return family.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _period_after(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("The period said right after the figure ending at `at`, as 'a day' or 'a week'.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERIOD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    return if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("week")))) { crate::memory::store_lines::g__WEEKLY() } else { py_str("a day") };
    Py::None
}
pub fn _a_period_near(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut at: Py = Py::None;
    let _ = py_str("The period said after `phrase` in `text`, as 'a day' or 'a week'.");
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, "find", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&at.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&phrase.clone())) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _period_after(args))(&__args) };
    Py::None
}
pub fn _the_period_of_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("The period an answer is written on, when it says one: 'a day' or 'a week'.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:a|per|each|every)\\s+(day|week|night)\\b")); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    return if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("week")))) { crate::memory::store_lines::g__WEEKLY() } else { py_str("a day") };
    }
    return Py::None;
    Py::None
}
pub fn _the_periods_the_answer_says(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every period an answer is written on: {'a day', 'a week'} for a guideline that says both.\n\n    A month is kept as a month: \"give it six weeks before you judge it\" and \"in a month\" are not\n    the person's day, and reading one as a day let a weekly answer slide past the period check -\n    \"20 mins a day\" against \"Three half-hour sessions a week is enough ... a walk for ten minutes\n    to warm up\" was answered \"20 mins is past the ten minutes that answer named\".\n    ");
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:a|an|per|each|every)\\s+(day|week|night|month|year)\\b")); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(op_add(&py_str("a "), &word.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_dict(vec![(py_str("week"), crate::memory::store_lines::g__WEEKLY()), (py_str("day"), py_str("a day")), (py_str("night"), py_str("a day"))]), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "add", args))(&__args) };
    }
    return out.clone();
    Py::None
}
pub fn _periods_disagree(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut theirs: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "theirs") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: theirs"))) };
    let mut also: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "also") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_str("True when the two figures are said on periods that cannot be read against each other.\n\n    A weekly figure and a daily one are not the same amount, and a weekly total against a figure\n    with no period at all (\"50 hours a week\" against \"seven to nine hours\") is a rate nobody\n    stated. A figure said per night is a figure said per day for every count in this module.\n    ");
    if py_truthy(&Py::Bool(py_is(&mine.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&also.clone(), &mine.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_is(&theirs.clone(), &Py::None))) {
    return Py::Bool(py_eq(&mine.clone(), &crate::memory::store_lines::g__WEEKLY()));
    }
    return Py::Bool(!py_eq(&mine.clone(), &theirs.clone()));
    Py::None
}
pub fn _too_far_to_compare(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let _ = py_str("True when the answer's figure and theirs are the same unit but not the same quantity.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { { let __b = low.clone(); if !py_truthy(&__b) { __b } else { high.clone() } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_gt(&value.clone(), &high.clone()); if !py_truthy(&__b) { __b } else { op_gt(&value.clone(), &op_mul(&g__TOO_FAR_ABOVE(), &high.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_lt(&value.clone(), &low.clone()); if !py_truthy(&__b) { __b } else { op_gt(&low.clone(), &op_mul(&g__TOO_FAR_BELOW(), &value.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _amounts(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every figure with a unit in the line: (value, family, as it was said).");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::spoken::_spoken_as_numbers(args))(&__args) };
    found = py_list(vec![]);
    spans = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOURS_AND_MINUTES(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    hours = __unpk2[0].clone();
    minutes = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = hours.clone(); if py_truthy(&__b) { __b } else { minutes.clone() } }))) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![op_add(&op_mul(&hours.clone(), &Py::Int(60)), &minutes.clone()), py_str("minutes"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "append", args))(&__args) };
    }
    pattern = op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:x\\s*)?")), &py_str("(")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk3709 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk3709[0].clone(); let mut p: Py = __unpk3709[1].clone(); __out.push(p.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")"));
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(text.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    match_ = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk9403 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk9403[0].clone(); let mut end: Py = __unpk9403[1].clone(); __out.push({ let __b = op_le(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &end.clone()) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![value.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let end = end.clone(); let found = found.clone(); let hours = hours.clone(); let match_ = match_.clone(); let minutes = minutes.clone(); let p = p.clone(); let pattern = pattern.clone(); let spans = spans.clone(); let start = start.clone(); let text = text.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let end = end.clone(); let found = found.clone(); let hours = hours.clone(); let match_ = match_.clone(); let minutes = minutes.clone(); let p = p.clone(); let pattern = pattern.clone(); let spans = spans.clone(); let start = start.clone(); let text = text.clone(); let value = value.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a week"), py_str("a day")]), &geti(&item.clone(), &Py::Int(1))))) { Py::Int(0) } else { Py::Int(1) } }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    return found.clone();
    Py::None
}
pub fn _target(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut mine: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { Py::None };
    let mut bare_ok: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare_ok") { v.clone() }
    else { Py::Bool(false) };
    let mut period: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut at: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut best: Py = Py::None;
    let mut each: Py = Py::None;
    let mut end: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut high: Py = Py::None;
    let mut high_family: Py = Py::None;
    let mut high_in: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut is_their_number: Py = Py::None;
    let mut long_spans: Py = Py::None;
    let mut low: Py = Py::None;
    let mut low_family: Py = Py::None;
    let mut low_in: Py = Py::None;
    let mut m: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut ranges: Py = Py::None;
    let mut ranges_on: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_on: Py = Py::None;
    let mut singles: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut total: Py = Py::None;
    let mut units: Py = Py::None;
    let mut useful: Py = Py::None;
    let mut useful_on: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The figure the last answer named in the same family: (low, high, as it was said).\n\n    A guideline often opens by saying the person's own figure back - \"Five hours is on the short\n    side: most adults need seven to nine\" - and that opening is not the figure to aim at, it is\n    the answer measuring them against it. When the first figure in the answer is the number they\n    gave, the range the answer names is the target, unit or no unit.\n\n    `period` is the period the person's own figure is said on (\"a week\" for \"two and a half hours\n    a week\"). An answer often says the same aim twice in two periods - \"Aim for about 150 minutes\n    a week, which is around 20 minutes a day\" - and read as one family that is two figures, which\n    is no figure to aim at, so the ones said on the person's own period are the ones to read.\n    ");
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::spoken::_spoken_as_numbers(args))(&__args) };
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk157 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk157[0].clone(); let mut p: Py = __unpk157[1].clone(); __out.push(p.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) };
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    ranges = __unpk1[0].clone();
    singles = __unpk1[1].clone();
    long_spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOURS_AND_MINUTES(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "span", args))(&__args) }); } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|-|\\u2013|and)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone()))) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    if py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { { let __b = high.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rate(args))(&__args) })) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, py_tuple(vec![low.clone(), high.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }])])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ranges.clone(), "append", args))(&__args) };
    }
    }
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|-|\\u2013|and)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut m: Py = __item.clone(); __out.push(py_tuple(vec![m.clone(), Py::None])); } py_list(__out) })() }) { let __unpk1981 = py_unpack(&__item.clone(), 2, None); let mut match_: Py = __unpk1981[0].clone(); let mut _u: Py = __unpk1981[1].clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }])); } py_list(__out) })() };
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(")), &units.clone()), &py_str(")\\s*(?:to|-|\\u2013|and)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(")), &units.clone()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    match_ = __item.clone();
    let __unpk5 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(4)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }]), 2, None);
    low_family = __unpk5[0].clone();
    high_family = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_eq(&low_family.clone(), &high_family.clone()))) {
    continue;
    }
    let __unpk6 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk6[0].clone();
    high = __unpk6[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { high.clone() } }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![low_family.clone(), high_family.clone()]), &family.clone()))) {
    continue;
    }
    low_in = op_mul(&low.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![family.clone(), low_family.clone()])); __args.push(if py_truthy(&Py::Bool(py_eq(&low_family.clone(), &family.clone()))) { Py::Float(1.0) } else { Py::Int(0) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__CONVERT(), "get", args))(&__args) });
    high_in = op_mul(&high.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![family.clone(), high_family.clone()])); __args.push(if py_truthy(&Py::Bool(py_eq(&high_family.clone(), &family.clone()))) { Py::Float(1.0) } else { Py::Int(0) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__CONVERT(), "get", args))(&__args) });
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = low_in.clone(); if !py_truthy(&__b) { __b } else { high_in.clone() } }))) {
    continue;
    }
    said = op_modulo(&py_str("%s to %s %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low_in.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low_in.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { low_in.clone() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(high_in.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(high_in.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { high_in.clone() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) }, geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(family.clone()); __args.push(py_tuple(vec![family.clone(), family.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__UNIT_SAID(), "get", args))(&__args) }, &Py::Int(1))]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, py_tuple(vec![low_in.clone(), high_in.clone(), said.clone()])])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ranges.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "append", args))(&__args) };
    }
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter7 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str("-")))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&spans.clone(), &long_spans.clone())) { let __unpk7068 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk7068[0].clone(); let mut end: Py = __unpk7068[1].clone(); __out.push({ let __b = op_le(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &end.clone()) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rate(args))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, py_tuple(vec![value.clone(), value.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }])])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&singles.clone(), "append", args))(&__args) };
    }
    }
    let __iter8 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOURS_AND_MINUTES(), "finditer", args))(&__args) });
    for __item in __iter8 {
    match_ = __item.clone();
    let __unpk9 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    hours = __unpk9[0].clone();
    minutes = __unpk9[1].clone();
    if py_truthy(&{ let __b = hours.clone(); if py_truthy(&__b) { __b } else { minutes.clone() } }) {
    total = op_add(&op_mul(&hours.clone(), &Py::Int(60)), &minutes.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, py_tuple(vec![total.clone(), total.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }])])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&singles.clone(), "append", args))(&__args) };
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let at = at.clone(); let bare = bare.clone(); let bare_ok = bare_ok.clone(); let best = best.clone(); let each = each.clone(); let end = end.clone(); let family = family.clone(); let figure = figure.clone(); let figures = figures.clone(); let high = high.clone(); let high_family = high_family.clone(); let high_in = high_in.clone(); let hours = hours.clone(); let is_their_number = is_their_number.clone(); let long_spans = long_spans.clone(); let low = low.clone(); let low_family = low_family.clone(); let low_in = low_in.clone(); let m = m.clone(); let match_ = match_.clone(); let mine = mine.clone(); let minutes = minutes.clone(); let p = p.clone(); let pair = pair.clone(); let period = period.clone(); let ranges = ranges.clone(); let ranges_on = ranges_on.clone(); let said = said.clone(); let said_on = said_on.clone(); let singles = singles.clone(); let spans = spans.clone(); let start = start.clone(); let total = total.clone(); let units = units.clone(); let useful = useful.clone(); let useful_on = useful_on.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let at = at.clone(); let bare = bare.clone(); let bare_ok = bare_ok.clone(); let best = best.clone(); let each = each.clone(); let end = end.clone(); let family = family.clone(); let figure = figure.clone(); let figures = figures.clone(); let high = high.clone(); let high_family = high_family.clone(); let high_in = high_in.clone(); let hours = hours.clone(); let is_their_number = is_their_number.clone(); let long_spans = long_spans.clone(); let low = low.clone(); let low_family = low_family.clone(); let low_in = low_in.clone(); let m = m.clone(); let match_ = match_.clone(); let mine = mine.clone(); let minutes = minutes.clone(); let p = p.clone(); let pair = pair.clone(); let period = period.clone(); let ranges = ranges.clone(); let ranges_on = ranges_on.clone(); let said = said.clone(); let said_on = said_on.clone(); let singles = singles.clone(); let spans = spans.clone(); let start = start.clone(); let total = total.clone(); let units = units.clone(); let useful = useful.clone(); let useful_on = useful_on.clone(); let value = value.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ranges.clone(), "sort", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let answer = answer.clone(); let at = at.clone(); let bare = bare.clone(); let bare_ok = bare_ok.clone(); let best = best.clone(); let each = each.clone(); let end = end.clone(); let family = family.clone(); let figure = figure.clone(); let figures = figures.clone(); let high = high.clone(); let high_family = high_family.clone(); let high_in = high_in.clone(); let hours = hours.clone(); let is_their_number = is_their_number.clone(); let long_spans = long_spans.clone(); let low = low.clone(); let low_family = low_family.clone(); let low_in = low_in.clone(); let m = m.clone(); let match_ = match_.clone(); let mine = mine.clone(); let minutes = minutes.clone(); let p = p.clone(); let pair = pair.clone(); let period = period.clone(); let ranges = ranges.clone(); let ranges_on = ranges_on.clone(); let said = said.clone(); let said_on = said_on.clone(); let singles = singles.clone(); let spans = spans.clone(); let start = start.clone(); let total = total.clone(); let units = units.clone(); let useful = useful.clone(); let useful_on = useful_on.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let answer = answer.clone(); let at = at.clone(); let bare = bare.clone(); let bare_ok = bare_ok.clone(); let best = best.clone(); let each = each.clone(); let end = end.clone(); let family = family.clone(); let figure = figure.clone(); let figures = figures.clone(); let high = high.clone(); let high_family = high_family.clone(); let high_in = high_in.clone(); let hours = hours.clone(); let is_their_number = is_their_number.clone(); let long_spans = long_spans.clone(); let low = low.clone(); let low_family = low_family.clone(); let low_in = low_in.clone(); let m = m.clone(); let match_ = match_.clone(); let mine = mine.clone(); let minutes = minutes.clone(); let p = p.clone(); let pair = pair.clone(); let period = period.clone(); let ranges = ranges.clone(); let ranges_on = ranges_on.clone(); let said = said.clone(); let said_on = said_on.clone(); let singles = singles.clone(); let spans = spans.clone(); let start = start.clone(); let total = total.clone(); let units = units.clone(); let useful = useful.clone(); let useful_on = useful_on.clone(); let value = value.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&singles.clone(), "sort", args))(&__args) };
pub fn __nested_is_their_number_9(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _u: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut answer: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut bare: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut bare_ok: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut best: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut each: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut end: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut family: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut figure: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut figures: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut high: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut high_family: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut high_in: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut hours: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut is_their_number: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut long_spans: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut low: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut low_family: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut low_in: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut m: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut mine: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut minutes: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut p: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut pair: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut period: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut ranges: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut ranges_on: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut said_on: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut singles: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut spans: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut start: Py = __pos.get(33).cloned().unwrap_or(Py::None);
    let mut total: Py = __pos.get(34).cloned().unwrap_or(Py::None);
    let mut units: Py = __pos.get(35).cloned().unwrap_or(Py::None);
    let mut useful: Py = __pos.get(36).cloned().unwrap_or(Py::None);
    let mut useful_on: Py = __pos.get(37).cloned().unwrap_or(Py::None);
    let mut value: Py = __pos.get(38).cloned().unwrap_or(Py::None);
    let mut figure: Py = __pos.get(39).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "figure").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = Py::Bool(!py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = geti(&figure.clone(), &Py::Int(1)); if py_truthy(&Py::Bool(py_eq(&geti(&figure.clone(), &Py::Int(0)), &__cmp11))) { Py::Bool(py_eq(&__cmp11, &mine.clone())) } else { Py::Bool(false) } } } };
    Py::None
}
    is_their_number = {{ let _u = _u.clone(); let answer = answer.clone(); let at = at.clone(); let bare = bare.clone(); let bare_ok = bare_ok.clone(); let best = best.clone(); let each = each.clone(); let end = end.clone(); let family = family.clone(); let figure = figure.clone(); let figures = figures.clone(); let high = high.clone(); let high_family = high_family.clone(); let high_in = high_in.clone(); let hours = hours.clone(); let is_their_number = is_their_number.clone(); let long_spans = long_spans.clone(); let low = low.clone(); let low_family = low_family.clone(); let low_in = low_in.clone(); let m = m.clone(); let match_ = match_.clone(); let mine = mine.clone(); let minutes = minutes.clone(); let p = p.clone(); let pair = pair.clone(); let period = period.clone(); let ranges = ranges.clone(); let ranges_on = ranges_on.clone(); let said = said.clone(); let said_on = said_on.clone(); let singles = singles.clone(); let spans = spans.clone(); let start = start.clone(); let total = total.clone(); let units = units.clone(); let useful = useful.clone(); let useful_on = useful_on.clone(); let value = value.clone(); py_func("is_their_number", move |args: &[Py]| { let mut full = vec![_u.clone(), answer.clone(), at.clone(), bare.clone(), bare_ok.clone(), best.clone(), each.clone(), end.clone(), family.clone(), figure.clone(), figures.clone(), high.clone(), high_family.clone(), high_in.clone(), hours.clone(), is_their_number.clone(), long_spans.clone(), low.clone(), low_family.clone(), low_in.clone(), m.clone(), match_.clone(), mine.clone(), minutes.clone(), p.clone(), pair.clone(), period.clone(), ranges.clone(), ranges_on.clone(), said.clone(), said_on.clone(), singles.clone(), spans.clone(), start.clone(), total.clone(), units.clone(), useful.clone(), useful_on.clone(), value.clone()]; full.extend_from_slice(args); __nested_is_their_number_9(&full) }) }};
    useful = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&singles.clone()) { let __unpk5009 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk5009[0].clone(); let mut figure: Py = __unpk5009[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = ranges.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(figure.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&is_their_number, args))(&__args) } } }))) {  __out.push(py_tuple(vec![at.clone(), figure.clone()])); } } py_list(__out) })() };
    figures = op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let __unpk5057 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk5057[0].clone(); let mut figure: Py = __unpk5057[1].clone(); __out.push(figure.clone()); } py_list(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&useful.clone()) { let __unpk9362 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk9362[0].clone(); let mut figure: Py = __unpk9362[1].clone(); __out.push(figure.clone()); } py_list(__out) })() });
    if py_truthy(&{ let __b = period.clone(); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figures.clone()) { let mut figure: Py = __item.clone(); __out.push(slice_or_raise(&figure.clone(), &Py::None, &Py::Int(2), &Py::None)); } py_set(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } }) {
    ranges_on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(ranges.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_periods_of(args))(&__args) };
    useful_on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(useful.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_periods_of(args))(&__args) };
    said_on = op_add(&ranges_on.clone(), &useful_on.clone());
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_on.clone()) { let mut each: Py = __item.clone(); __out.push(Py::Bool(py_eq(&each.clone(), &period.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_on.clone()) { let mut each: Py = __item.clone(); __out.push({ let __b = each.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&each.clone(), &period.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    ranges = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ranges.clone()); __args.push(ranges_on.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("zip", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk3218 = py_unpack(&__item.clone(), 2, None); let mut pair: Py = __unpk3218[0].clone(); let mut each: Py = __unpk3218[1].clone(); if py_truthy(&Py::Bool(py_eq(&each.clone(), &period.clone()))) {  __out.push(pair.clone()); } } py_list(__out) })() };
    useful = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(useful.clone()); __args.push(useful_on.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("zip", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk9377 = py_unpack(&__item.clone(), 2, None); let mut pair: Py = __unpk9377[0].clone(); let mut each: Py = __unpk9377[1].clone(); if py_truthy(&Py::Bool(py_eq(&each.clone(), &period.clone()))) {  __out.push(pair.clone()); } } py_list(__out) })() };
    figures = op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let __unpk6113 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk6113[0].clone(); let mut figure: Py = __unpk6113[1].clone(); __out.push(figure.clone()); } py_list(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&useful.clone()) { let __unpk6290 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk6290[0].clone(); let mut figure: Py = __unpk6290[1].clone(); __out.push(figure.clone()); } py_list(__out) })() });
    }
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figures.clone()) { let mut figure: Py = __item.clone(); __out.push(slice_or_raise(&figure.clone(), &Py::None, &Py::Int(2), &Py::None)); } py_set(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) {
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut figure: Py = __item.clone(); __out.push(slice_or_raise(&figure.clone(), &Py::None, &Py::Int(2), &Py::None)); } py_set(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    best = geti(&geti(&ranges.clone(), &Py::Int(0)), &Py::Int(1));
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&is_their_number, args))(&__args) }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(best.clone()); __args.push(period.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_its_period(args))(&__args) };
    }
    }
    return Py::None;
    }
    best = if py_truthy(&ranges.clone()) { geti(&geti(&ranges.clone(), &Py::Int(0)), &Py::Int(1)) } else { if py_truthy(&useful.clone()) { geti(&geti(&useful.clone(), &Py::Int(0)), &Py::Int(1)) } else { Py::None } };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&is_their_number, args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(best.clone()); __args.push(period.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_its_period(args))(&__args) };
    }
    bare = if py_truthy(&bare_ok.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(mine.clone()); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _near_bare_range(args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&bare.clone(), &Py::None))) {
    return bare.clone();
    }
    return if py_truthy(&Py::Bool(!py_is(&best.clone(), &Py::None))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(best.clone()); __args.push(period.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_its_period(args))(&__args) } } else { Py::None };
    Py::None
}
pub fn _the_aim_that_is_their_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut mine: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut _u: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut p: Py = Py::None;
    let mut period: Py = Py::None;
    let mut said: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("The figure the answer aims at when it is the number they gave: (low, high, said) or None.\n\n    A guideline is often written as the figure they already do - \"Aim for about 150 minutes a\n    week\" to somebody who has just said \"i do two and a half hours a week\" - and `_target` passes\n    over it, because a short answer that repeats their number is a receipt. Read as an aim, the\n    same figure is the answer: their 150 minutes a week is the 150 minutes a week it named.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk1111 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk1111[0].clone(); let mut p: Py = __unpk1111[1].clone(); __out.push(p.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")"))); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, &mine.clone()))) {
    continue;
    }
    before = slice_or_raise(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(40))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AIM_CUE(), "search", args))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    period = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _period_after(args))(&__args) };
    if py_truthy(&period.clone()) {
    let __aug2 = op_add(&said.clone(), &op_add(&py_str(" "), &if py_truthy(&Py::Bool(py_eq(&period.clone(), &crate::memory::store_lines::g__WEEKLY()))) { py_str("a week") } else { py_str("a day") }));
    said = __aug2;
    }
    return py_tuple(vec![mine.clone(), mine.clone(), said.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _with_its_period(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut figure: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut period: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut at: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The figure with the period it is said on, when the answer says it on the person's own.\n\n    \"150 minutes\" is read off \"Aim for about 150 minutes a week\", and said back without the \"a\n    week\" it leaves out what the figure is per: \"2.5 hours is at the 150 minutes that answer\n    named\" against \"at the 150 minutes a week that answer named\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = period.clone(); if !py_truthy(&__b) { __b } else { figure.clone() } }))) {
    return figure.clone();
    }
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&figure.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "find", args))(&__args) };
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    return figure.clone();
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&figure.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERIOD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return figure.clone();
    }
    said = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("week")))) { crate::memory::store_lines::g__WEEKLY() } else { py_str("a day") };
    if py_truthy(&Py::Bool(!py_eq(&said.clone(), &period.clone()))) {
    return figure.clone();
    }
    return py_tuple(vec![geti(&figure.clone(), &Py::Int(0)), geti(&figure.clone(), &Py::Int(1)), op_add(&op_add(&geti(&figure.clone(), &Py::Int(2)), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) })]);
    Py::None
}
pub fn _the_periods_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut figures: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figures") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figures"))) };
    let mut at: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("The period each figure in `figures` (start, (low, high, said)) is said on, or None.\n\n    The period is written right after the figure - \"150 minutes **a week**\" - and a figure with\n    none is a figure on no stated period, which is not the same as one said on another.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&figures.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    figure = __unpk2[1].clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERIOD(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &op_add(&op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&figure.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(12))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("week")))) { crate::memory::store_lines::g__WEEKLY() } else { py_str("a day") }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    return out.clone();
    Py::None
}
pub fn _aims_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut family: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut _f: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut p: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut units: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The distinct figures of this kind the answer names, a range counting as one figure.\n\n    \"Lower eggs into already boiling water: 6 minutes for a runny yolk, 7 for jammy, 9 to 10 for\n    hard boiled\" names three timings, and reading one of them as the figure to aim at answered\n    \"is 6 minutes enough\" with \"short of the 9 to 10 minutes\". A guideline names one figure or one\n    range - \"seven to nine hours\", \"2,000 to 2,500 calories\" - and not a list of choices.\n    ");
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk4950 = py_unpack(&__item.clone(), 2, None); let mut _f: Py = __unpk4950[0].clone(); let mut p: Py = __unpk4950[1].clone(); __out.push(p.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) };
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![])]), 2, None);
    found = __unpk1[0].clone();
    spans = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|and|or|\\u2013|-)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")?"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    let __unpk3 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { { let __b = high.clone(); if !py_truthy(&__b) { __b } else { op_lt(&low.clone(), &high.clone()) } } } }))) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "span", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "append", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone())) } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![low.clone(), high.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "add", args))(&__args) };
    }
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(")), &units.clone()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk2949 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk2949[0].clone(); let mut end: Py = __unpk2949[1].clone(); __out.push({ let __b = op_le(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &end.clone()) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "span", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "append", args))(&__args) };
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone()))) {
    continue;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&value.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![value.clone(), value.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "add", args))(&__args) };
    }
    }
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter5 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk5862 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk5862[0].clone(); let mut end: Py = __unpk5862[1].clone(); __out.push({ let __b = op_le(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &end.clone()) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mine.clone(); if !py_truthy(&__b) { __b } else { { let __cmp61: Py = value.clone(); if py_truthy(&op_le(&op_truediv(&mine.clone(), &Py::Int(3)), &__cmp61)) { op_le(&__cmp61, &op_mul(&mine.clone(), &Py::Int(3))) } else { Py::Bool(false) } } } })) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counts_things(args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![value.clone(), value.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "add", args))(&__args) };
    }
    return found.clone();
    Py::None
}
pub fn _counts_things(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut at_word: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the number ending at `at` counts things rather than measuring an amount.\n\n    A count of things is an example in a list (\"three twenty-minute sessions\") and not an aim,\n    unless it is said as a rate: \"Three half-hour sessions a week is enough to feel different in a\n    month\" is the answer's aim, and read past it the store answered \"20 mins is past the ten\n    minutes that answer named\" from the warm-up in the sentence after it.\n    ");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WORD_AFTER(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "split", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at_word = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PERIOD_WORDS(), &word.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&slice_or_raise(&words.clone(), &op_add(&at_word.clone(), &Py::Int(1)), &Py::None, &Py::None), &py_list(vec![py_str("")]))) { let mut other: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__PERIOD_WORDS(), &other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &py_str("a week"))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _figures_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut _family: Py = Py::None;
    let mut _said: Py = Py::None;
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every number in the answer, with or without a unit: \"20 minutes\", \"seven to nine\".\n\n    Used to tell an answer that is only saying the person's own figure back from one that names\n    another figure for them to reach.\n    ");
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amounts(args))(&__args) }) { let __unpk4965 = py_unpack(&__item.clone(), 3, None); let mut value: Py = __unpk4965[0].clone(); let mut _family: Py = __unpk4965[1].clone(); let mut _said: Py = __unpk4965[2].clone(); __out.push(value.clone()); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|and|or|\\u2013|-)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\b"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    if py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { { let __b = high.clone(); if !py_truthy(&__b) { __b } else { op_lt(&low.clone(), &high.clone()) } } } }) {
    let __aug3 = op_add(&found.clone(), &py_list(vec![low.clone(), high.clone()]));
    found = __aug3;
    }
    }
    return found.clone();
    Py::None
}
pub fn _a_rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let _ = py_str("The figure is a rate rather than an amount: \"0.75g per kilo of body weight\", \"2% of it\".\n\n    \"how much protein should i eat\" answered \"0.75g per kilo ... roughly 50g for a 65kg adult\",\n    and 60g of protein was called \"past the 0.75g that answer named\". A rate is not a figure to be\n    over or under; the worked example beside it is, and this reading does not pick it out.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:%|per cent\\b|percent\\b|per (?:kilo|kg|kilogram|kilo of|pound|lb|100|head|person|day per)\\b|g\\s?/\\s?kg\\b|mg\\s?/\\s?kg\\b)")); __args.push(slice_or_raise(&answer.clone(), &end.clone(), &op_add(&end.clone(), &Py::Int(24)), &Py::None)); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _another_kind_follows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut family: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut _f: Py = Py::None;
    let mut following: Py = Py::None;
    let mut p: Py = Py::None;
    let _ = py_str("A unit of another kind sits right after the figure, so the figure is not this kind's.\n\n    \"three or four runs a week\" is a count of runs; read as kilometres it turned \"i run 10k every\n    day\" into \"more than the three or four km that answer named\".\n    ");
    following = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\s*("), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk1845 = py_unpack(&__item.clone(), 2, None); let mut _f: Py = __unpk1845[0].clone(); let mut p: Py = __unpk1845[1].clone(); __out.push(p.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); __args.push(slice_or_raise(&answer.clone(), &end.clone(), &Py::None, &Py::None)); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(following.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&following.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amount_family(args))(&__args) }, &family.clone())) } };
    Py::None
}
pub fn _near_bare_range(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut family: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { Py::None };
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("A range of numbers said without a unit, near enough their own figure to be about it.\n\n    \"most adults need seven to nine\" is the range a line about five hours is measured against.\n    \"7,000 to 8,000 steps\" in an answer about walking is a range in another unit entirely, and a\n    range that far from their own number is not the figure they are being compared with.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|and|or|\\u2013|-)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\b"))); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { { let __b = high.clone(); if !py_truthy(&__b) { __b } else { op_lt(&low.clone(), &high.clone()) } } } }))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_le(&low.clone(), &op_mul(&mine.clone(), &Py::Int(3))); if !py_truthy(&__b) { __b } else { op_le(&mine.clone(), &op_mul(&high.clone(), &Py::Int(3))) } })) } }) {
    continue;
    }
    if py_truthy(&{ let __b = family.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _another_kind_follows(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rate(args))(&__args) }) {
    continue;
    }
    return py_tuple(vec![low.clone(), high.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }]);
    }
    return Py::None;
    Py::None
}
pub fn _with_its_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut aim: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "aim") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: aim"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut high: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut answer: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { py_str("") };
    let mut at: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("The aim with its unit said, when the answer named the range without one.\n\n    \"most adults need seven to nine\" is a range about hours, and answering \"you are short of the\n    seven to nine that answer named\" leaves out the thing that is counted. A range that is\n    followed by a unit of another kind is that other kind's range: \"three or four runs a week\" is\n    a count of runs, and saying it in kilometres turned a 10k a day into \"more than the three or\n    four km that answer named\".\n    ");
    pattern = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::store_lines::g__AMOUNTS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pattern.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(aim.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return aim.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("minutes"), py_str("hours")]), &family.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:minutes?|mins?|hours?|hrs?)\\b")); __args.push(aim.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return aim.clone();
    }
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(aim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "find", args))(&__args) };
    if py_truthy(&{ let __b = op_ge(&at.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(aim.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _another_kind_follows(args))(&__args) } } }) {
    return aim.clone();
    }
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__UNIT_SAID(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&units.clone()))) {
    return aim.clone();
    }
    return op_add(&op_add(&aim.clone(), &py_str(" ")), &if py_truthy(&Py::Bool(py_eq(&high.clone(), &Py::Int(1)))) { geti(&units.clone(), &Py::Int(0)) } else { geti(&units.clone(), &Py::Int(1)) });
    Py::None
}
pub fn _the_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut whole: Py = Py::None;
    whole = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { value.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn enough_reply(args: &[Py]) -> Py {
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
    let mut _family: Py = Py::None;
    let mut _said: Py = Py::None;
    let mut found: Py = Py::None;
    let mut given: Py = Py::None;
    let mut item: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line_text: Py = Py::None;
    let mut mine_values: Py = Py::None;
    let mut more: Py = Py::None;
    let mut purse_reply: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut too_much: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Where the amount they gave stands against the figure the last answer named, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    more = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_MORE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&more.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENOUGH_ASK(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    last = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turns.clone(), &op_neg(&Py::Int(1)))); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }))) {
    return Py::None;
    }
    too_much = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\btoo (?:much|often|many)\\b")); __args.push(text.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let mut purse: Py = crate::memory::purse::module_obj();
    purse_reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&purse.clone(), "against_the_bands", args))(&__args) };
    if py_truthy(&purse_reply.clone()) {
    return purse_reply.clone();
    }
    said = py_str("");
    given = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut t: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }));
    for __item in __iter1 {
    line_text = __item.clone();
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _amounts(args))(&__args) }) { let mut item: Py = __item.clone(); if py_truthy(&geti(&item.clone(), &Py::Int(1))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&found.clone()) {
    given = found.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::spoken::_spoken_as_numbers(args))(&__args) };
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&given.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(text.clone()); __args.push(more.clone()); __args.push(too_much.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _enough_bare(args))(&__args) };
    }
    mine_values = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&given.clone()) { let __unpk2586 = py_unpack(&__item.clone(), 3, None); let mut value: Py = __unpk2586[0].clone(); let mut _family: Py = __unpk2586[1].clone(); let mut _said: Py = __unpk2586[2].clone(); __out.push(value.clone()); } py_set(__out) })() };
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(mine_values.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _candidate_answers(args))(&__args) }); __args.push(given.clone()); __args.push(mine_values.clone()); __args.push(said.clone()); __args.push(more.clone()); __args.push(too_much.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _enough_against(args))(&__args) };
    return reply.clone();
    Py::None
}
pub fn _enough_bare(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut more: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "more") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: more"))) };
    let mut too_much: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "too_much") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: too_much"))) };
    let mut aim: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut high: Py = Py::None;
    let mut line_text: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut t: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The reading for a figure the person gave without a unit, or None.\n\n    Their line says the number and the answer says the kind of amount: \"i eat 1200\" against\n    \"roughly 2,000 to 2,500 calories a day\". Nothing is read from an answer whose range is not\n    near enough their number to be about it, and a small bare number is left alone.\n    ");
    mine = Py::Int(0);
    let __iter1 = py_iter_or_raise(&op_add(&py_list(vec![text.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut t: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }));
    for __item in __iter1 {
    line_text = __item.clone();
    bare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bare_figures(args))(&__args) }) { let mut value: Py = __item.clone(); if py_truthy(&op_ge(&value.clone(), &Py::Int(100))) {  __out.push(value.clone()); } } py_list(__out) })() };
    if py_truthy(&bare.clone()) {
    mine = geti(&bare.clone(), &Py::Int(0));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_set(vec![mine.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _candidate_answers(args))(&__args) });
    for __item in __iter2 {
    answer = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _range_near(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&found.clone(), &Py::Int(0)), geti(&found.clone(), &Py::Int(1))]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    aim = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&geti(&found.clone(), &Py::Int(2)), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&geti(&found.clone(), &Py::Int(2)), &Py::Int(1), &Py::None, &Py::None));
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&spoken.clone(), &py_str("1")))) {
    spoken = py_str("one");
    }
    if py_truthy(&more.clone()) {
    gap = op_sub(&low.clone(), &mine.clone());
    if py_truthy(&op_le(&gap.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("None - %s is already at the %s that answer named."), &py_tuple(vec![spoken.clone(), aim.clone()]));
    }
    return op_modulo(&py_str("%s more would put you at the %s that answer named."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gap.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) }, aim.clone()]));
    }
    if py_truthy(&op_lt(&mine.clone(), &low.clone())) {
    return op_modulo(&py_str("%s is short of the %s that answer named."), &py_tuple(vec![spoken.clone(), aim.clone()]));
    }
    if py_truthy(&op_gt(&mine.clone(), &high.clone())) {
    return op_modulo(&py_str("%s is past the %s that answer named, so that side is worth watching."), &py_tuple(vec![spoken.clone(), aim.clone()]));
    }
    return op_modulo(&py_str("No - %s is inside the %s that answer named, so you are fine."), &py_tuple(vec![spoken.clone(), aim.clone()]));
    }
    Py::None
}
pub fn _range_near(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut _family: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("The range in the answer near enough their number to be about it, its unit included.\n\n    \"7,000 to 8,000 steps\" is not the figure a line about 1,200 calories is measured against, so\n    a range three times away from their number either side is passed over. A range said with its\n    unit (\"2,000 to 2,500 calories\") is read before one said without (\"seven to nine\").\n    ");
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk7876 = py_unpack(&__item.clone(), 2, None); let mut _family: Py = __unpk7876[0].clone(); let mut pattern: Py = __unpk7876[1].clone(); __out.push(pattern.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|and|or|\\u2013|-)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:")), &units.clone()), &py_str(")\\b")), op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:to|and|or|\\u2013|-)\\s*(")), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\b"))]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(answer.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    let __unpk3 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) }]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { { let __b = high.clone(); if !py_truthy(&__b) { __b } else { op_lt(&low.clone(), &high.clone()) } } } }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_le(&low.clone(), &op_mul(&mine.clone(), &Py::Int(3))); if !py_truthy(&__b) { __b } else { op_le(&mine.clone(), &op_mul(&high.clone(), &Py::Int(3))) } }))) {
    continue;
    }
    return py_tuple(vec![low.clone(), high.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _bare_figures(args: &[Py]) -> Py {
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
    let mut _family: Py = Py::None;
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let mut with_unit: Py = Py::None;
    let _ = py_str("Every number in the line that is not part of an amount with a unit.");
    with_unit = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")\\s*(?:-\\s*)?(?:")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g__AMOUNTS()) { let __unpk5301 = py_unpack(&__item.clone(), 2, None); let mut _family: Py = __unpk5301[0].clone(); let mut pattern: Py = __unpk5301[1].clone(); __out.push(pattern.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")"))); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut match_: Py = __item.clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }])); } py_list(__out) })() };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("("), &crate::memory::store_lines::g__COUNT_WORD()), &py_str(")"))); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&with_unit.clone()) { let __unpk6822 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk6822[0].clone(); let mut end: Py = __unpk6822[1].clone(); __out.push({ let __b = op_le(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &end.clone()) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value(args))(&__args) };
    if py_truthy(&value.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    return out.clone();
    Py::None
}
pub fn _candidate_answers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut mine_values: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine_values") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine_values"))) };
    let mut answer: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut out: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot answers to read a figure out of, newest first, up to three turns back.\n\n    The last answer is often the store saying their figure back (\"Got it - you have 9 grams a\n    day\") and the answer that named the range is the one before it, so an answer whose only\n    figures are the person's own is a receipt and is stepped over. Reading no further than three\n    turns keeps the figure and the range about the same thing.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    answer = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mine_values.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figures_in(args))(&__args) }) { let mut figure: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mine_values.clone(), &figure.clone()))) {  __out.push(figure.clone()); } } py_list(__out) })() })) } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    return out.clone();
    Py::None
}
pub fn _enough_against(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answers: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answers"))) };
    let mut given: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "given") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: given"))) };
    let mut mine_values: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine_values") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine_values"))) };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut more: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "more") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: more"))) };
    let mut too_much: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "too_much") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: too_much"))) };
    let mut quoted_ok: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "quoted_ok") { v.clone() }
    else { Py::Bool(false) };
    let mut aim: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut at: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut family: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut other: Py = Py::None;
    let mut period: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut read_family: Py = Py::None;
    let mut read_value: Py = Py::None;
    let mut said_unit: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut target: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut units: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The reply for the best of `answers`, or None when none of them names a figure to aim at.");
    quoted = Py::None;
    let __iter1 = py_iter_or_raise(&answers.clone());
    for __item in __iter1 {
    answer = __item.clone();
    let __iter2 = py_iter_or_raise(&given.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    value = __unpk3[0].clone();
    family = __unpk3[1].clone();
    spoken = __unpk3[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(spoken.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_period_near(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_period_of_the_answer(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_periods_the_answer_says(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _periods_disagree(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("minutes"), py_str("hours")]), &family.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TEMPERATURE(), "search", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOCTOR(), "search", args))(&__args) }) {
    continue;
    }
    period = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(spoken.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_period_near(args))(&__args) };
    let __unpk4 = py_unpack(&py_tuple(vec![family.clone(), value.clone()]), 2, None);
    read_family = __unpk4[0].clone();
    read_value = __unpk4[1].clone();
    target = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(family.clone()); __args.push(value.clone()); __kws.extend(vec![("bare_ok".to_string(), Py::Bool(true)), ("period".to_string(), period.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _target(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&target.clone()))) {
    target = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(family.clone()); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_aim_that_is_their_number(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&target.clone()))) {
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__CONVERT(), "items", args))(&__args) });
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    let __unpk7 = py_unpack(&__unpk6[0].clone(), 2, None);
    mine = __unpk7[0].clone();
    theirs = __unpk7[1].clone();
    factor = __unpk6[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mine.clone(), &family.clone()))) {
    continue;
    }
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(theirs.clone()); __kws.extend(vec![("period".to_string(), period.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _target(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&other.clone()))) {
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(theirs.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_truediv(&value.clone(), &factor.clone())); __args.push(Py::Int(4)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_aim_that_is_their_number(args))(&__args) };
    }
    if py_truthy(&other.clone()) {
    target = py_tuple(vec![op_mul(&geti(&other.clone(), &Py::Int(0)), &factor.clone()), op_mul(&geti(&other.clone(), &Py::Int(1)), &factor.clone()), geti(&other.clone(), &Py::Int(2))]);
    let __unpk8 = py_unpack(&py_tuple(vec![theirs.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_truediv(&value.clone(), &factor.clone())); __args.push(Py::Int(4)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    read_family = __unpk8[0].clone();
    read_value = __unpk8[1].clone();
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&target.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&quoted_ok.clone())); if !py_truthy(&__b) { __b } else { { let __b = spoken.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = geti(&target.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spoken.clone(), "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&geti(&target.clone(), &Py::Int(0)), &value.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(1e-09)); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&geti(&target.clone(), &Py::Int(1)), &value.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(1e-09)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AIM_CUE(), "search", args))(&__args) })) } } } } } } } } } }) {
    if py_truthy(&Py::Bool(py_is(&quoted.clone(), &Py::None))) {
    quoted = py_tuple(vec![answer.clone(), py_list(vec![py_tuple(vec![value.clone(), family.clone(), spoken.clone()])])]);
    }
    continue;
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(read_value.clone()); __args.push(read_family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _aims_named(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    continue;
    }
    if py_truthy(&{ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&target.clone(), &Py::Int(2)), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spoken.clone(), "lower", args))(&__args) })) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = mine_values.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figures_in(args))(&__args) }) { let mut figure: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mine_values.clone(), &figure.clone()))) {  __out.push(figure.clone()); } } py_list(__out) })() })) } }) {
    continue;
    }
    let __unpk9 = py_unpack(&py_tuple(vec![geti(&target.clone(), &Py::Int(0)), geti(&target.clone(), &Py::Int(1))]), 2, None);
    low = __unpk9[0].clone();
    high = __unpk9[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(low.clone()); __args.push(high.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _too_far_to_compare(args))(&__args) }) {
    continue;
    }
    aim = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&geti(&target.clone(), &Py::Int(2)), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&geti(&target.clone(), &Py::Int(2)), &Py::Int(1), &Py::None, &Py::None));
    aim = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(aim.clone()); __args.push(family.clone()); __args.push(high.clone()); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_its_unit(args))(&__args) };
    if py_truthy(&more.clone()) {
    gap = op_sub(&low.clone(), &value.clone());
    if py_truthy(&op_le(&gap.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("None - %s is already %s the %s that answer named."), &py_tuple(vec![spoken.clone(), if py_truthy(&op_le(&value.clone(), &high.clone())) { py_str("at") } else { py_str("past") }, aim.clone()]));
    }
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(family.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g__UNIT_SAID(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&units.clone()))) {
    continue;
    }
    said_unit = if py_truthy(&Py::Bool(py_eq(&gap.clone(), &Py::Int(1)))) { geti(&units.clone(), &Py::Int(0)) } else { geti(&units.clone(), &Py::Int(1)) };
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gap.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&amount.clone(), &py_str("1")))) {
    amount = py_str("one");
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("2"), py_str("3"), py_str("4"), py_str("5"), py_str("6"), py_str("7"), py_str("8"), py_str("9"), py_str("10")]), &amount.clone()))) {
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::store_lines::g_NUMBER_WORDS(), "get", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a week"), py_str("a day")]), &family.clone()))) {
    return op_modulo(&py_str("%s more %s would put you at the %s that answer named."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&amount.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&amount.clone(), &Py::Int(1), &Py::None, &Py::None)), said_unit.clone(), aim.clone()]));
    }
    return op_modulo(&py_str("Another %s %s would put you at the %s that answer named."), &py_tuple(vec![amount.clone(), said_unit.clone(), aim.clone()]));
    }
    if py_truthy(&too_much.clone()) {
    if py_truthy(&op_gt(&value.clone(), &high.clone())) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LIMIT(), "search", args))(&__args) }) {
    return op_modulo(&py_str("That is more than the %s that answer named, so trim it back towards that."), &aim.clone());
    }
    return op_modulo(&py_str("That is more than the %s that answer named."), &aim.clone());
    }
    if py_truthy(&{ let __cmp101: Py = value.clone(); if py_truthy(&op_le(&low.clone(), &__cmp101)) { op_le(&__cmp101, &high.clone()) } else { Py::Bool(false) } }) {
    return op_modulo(&py_str("No - %s is inside the %s that answer named, so you are fine."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), aim.clone()]));
    }
    continue;
    }
    if py_truthy(&op_gt(&value.clone(), &high.clone())) {
    return op_modulo(&py_str("%s is past the %s that answer named, so that side is worth watching."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), aim.clone()]));
    }
    if py_truthy(&op_ge(&value.clone(), &high.clone())) {
    at = if py_truthy(&op_gt(&high.clone(), &low.clone())) { py_str("at the top of") } else { py_str("at") };
    return op_modulo(&py_str("%s is %s the %s that answer named, so you are there already."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), at.clone(), aim.clone()]));
    }
    if py_truthy(&{ let __cmp111: Py = value.clone(); if py_truthy(&op_le(&low.clone(), &__cmp111)) { op_lt(&__cmp111, &high.clone()) } else { Py::Bool(false) } }) {
    return op_modulo(&py_str("%s is inside the %s that answer named, so you are fine."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), aim.clone()]));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a week"), py_str("a day")]), &family.clone()))) {
    return op_modulo(&py_str("%s is under the %s that answer named - one more would put you there."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), aim.clone()]));
    }
    return op_modulo(&py_str("%s is short of the %s that answer named."), &py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&spoken.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&spoken.clone(), &Py::Int(1), &Py::None, &Py::None)), aim.clone()]));
    }
    }
    if py_truthy(&quoted.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![geti(&quoted.clone(), &Py::Int(0))])); __args.push(geti(&quoted.clone(), &Py::Int(1))); __args.push(mine_values.clone()); __args.push(said.clone()); __args.push(more.clone()); __args.push(too_much.clone()); __kws.extend(vec![("quoted_ok".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _enough_against(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.enough".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ENOUGH_ASK" => Some(g_ENOUGH_ASK()),
            "HOW_MORE" => Some(g_HOW_MORE()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "_AIM_CUE" => Some(g__AIM_CUE()),
            "_AMOUNTS" => Some(g__AMOUNTS()),
            "_AMOUNT_WORDS" => Some(g__AMOUNT_WORDS()),
            "_A_DOCTOR" => Some(g__A_DOCTOR()),
            "_A_LIMIT" => Some(g__A_LIMIT()),
            "_A_PERIOD" => Some(g__A_PERIOD()),
            "_A_TEMPERATURE" => Some(g__A_TEMPERATURE()),
            "_A_WORD_AFTER" => Some(g__A_WORD_AFTER()),
            "_CONVERT" => Some(g__CONVERT()),
            "_COUNT_WORD" => Some(g__COUNT_WORD()),
            "_HOURS_AND_MINUTES" => Some(g__HOURS_AND_MINUTES()),
            "_PERIOD_WORDS" => Some(g__PERIOD_WORDS()),
            "_TOO_FAR_ABOVE" => Some(g__TOO_FAR_ABOVE()),
            "_TOO_FAR_BELOW" => Some(g__TOO_FAR_BELOW()),
            "_UNIT_SAID" => Some(g__UNIT_SAID()),
            "_WEEKLY" => Some(g__WEEKLY()),
            "_a_period_near" => Some(py_func("_a_period_near", |args: &[Py]| _a_period_near(args))),
            "_a_rate" => Some(py_func("_a_rate", |args: &[Py]| _a_rate(args))),
            "_a_value" => Some(py_func("_a_value", |args: &[Py]| _a_value(args))),
            "_aims_named" => Some(py_func("_aims_named", |args: &[Py]| _aims_named(args))),
            "_amount_family" => Some(py_func("_amount_family", |args: &[Py]| _amount_family(args))),
            "_amounts" => Some(py_func("_amounts", |args: &[Py]| _amounts(args))),
            "_another_kind_follows" => Some(py_func("_another_kind_follows", |args: &[Py]| _another_kind_follows(args))),
            "_bare_figures" => Some(py_func("_bare_figures", |args: &[Py]| _bare_figures(args))),
            "_candidate_answers" => Some(py_func("_candidate_answers", |args: &[Py]| _candidate_answers(args))),
            "_counts_things" => Some(py_func("_counts_things", |args: &[Py]| _counts_things(args))),
            "_enough_against" => Some(py_func("_enough_against", |args: &[Py]| _enough_against(args))),
            "_enough_bare" => Some(py_func("_enough_bare", |args: &[Py]| _enough_bare(args))),
            "_figures_in" => Some(py_func("_figures_in", |args: &[Py]| _figures_in(args))),
            "_near_bare_range" => Some(py_func("_near_bare_range", |args: &[Py]| _near_bare_range(args))),
            "_period_after" => Some(py_func("_period_after", |args: &[Py]| _period_after(args))),
            "_periods_disagree" => Some(py_func("_periods_disagree", |args: &[Py]| _periods_disagree(args))),
            "_range_near" => Some(py_func("_range_near", |args: &[Py]| _range_near(args))),
            "_target" => Some(py_func("_target", |args: &[Py]| _target(args))),
            "_the_aim_that_is_their_number" => Some(py_func("_the_aim_that_is_their_number", |args: &[Py]| _the_aim_that_is_their_number(args))),
            "_the_number" => Some(py_func("_the_number", |args: &[Py]| _the_number(args))),
            "_the_period_of_the_answer" => Some(py_func("_the_period_of_the_answer", |args: &[Py]| _the_period_of_the_answer(args))),
            "_the_periods_of" => Some(py_func("_the_periods_of", |args: &[Py]| _the_periods_of(args))),
            "_the_periods_the_answer_says" => Some(py_func("_the_periods_the_answer_says", |args: &[Py]| _the_periods_the_answer_says(args))),
            "_too_far_to_compare" => Some(py_func("_too_far_to_compare", |args: &[Py]| _too_far_to_compare(args))),
            "_with_its_period" => Some(py_func("_with_its_period", |args: &[Py]| _with_its_period(args))),
            "_with_its_unit" => Some(py_func("_with_its_unit", |args: &[Py]| _with_its_unit(args))),
            "enough_reply" => Some(py_func("enough_reply", |args: &[Py]| enough_reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ENOUGH_ASK" => s_ENOUGH_ASK(v),
            "HOW_MORE" => s_HOW_MORE(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "_AIM_CUE" => s__AIM_CUE(v),
            "_AMOUNTS" => s__AMOUNTS(v),
            "_AMOUNT_WORDS" => s__AMOUNT_WORDS(v),
            "_A_DOCTOR" => s__A_DOCTOR(v),
            "_A_LIMIT" => s__A_LIMIT(v),
            "_A_PERIOD" => s__A_PERIOD(v),
            "_A_TEMPERATURE" => s__A_TEMPERATURE(v),
            "_A_WORD_AFTER" => s__A_WORD_AFTER(v),
            "_CONVERT" => s__CONVERT(v),
            "_COUNT_WORD" => s__COUNT_WORD(v),
            "_HOURS_AND_MINUTES" => s__HOURS_AND_MINUTES(v),
            "_PERIOD_WORDS" => s__PERIOD_WORDS(v),
            "_TOO_FAR_ABOVE" => s__TOO_FAR_ABOVE(v),
            "_TOO_FAR_BELOW" => s__TOO_FAR_BELOW(v),
            "_UNIT_SAID" => s__UNIT_SAID(v),
            "_WEEKLY" => s__WEEKLY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.enough", module_obj);
    pyrt::xrefs::reg_fn("memory.enough", "_a_value", py_func("_a_value", |args: &[Py]| _a_value(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_amount_family", py_func("_amount_family", |args: &[Py]| _amount_family(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_period_after", py_func("_period_after", |args: &[Py]| _period_after(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_a_period_near", py_func("_a_period_near", |args: &[Py]| _a_period_near(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_the_period_of_the_answer", py_func("_the_period_of_the_answer", |args: &[Py]| _the_period_of_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_the_periods_the_answer_says", py_func("_the_periods_the_answer_says", |args: &[Py]| _the_periods_the_answer_says(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_periods_disagree", py_func("_periods_disagree", |args: &[Py]| _periods_disagree(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_too_far_to_compare", py_func("_too_far_to_compare", |args: &[Py]| _too_far_to_compare(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_amounts", py_func("_amounts", |args: &[Py]| _amounts(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_target", py_func("_target", |args: &[Py]| _target(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_the_aim_that_is_their_number", py_func("_the_aim_that_is_their_number", |args: &[Py]| _the_aim_that_is_their_number(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_with_its_period", py_func("_with_its_period", |args: &[Py]| _with_its_period(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_the_periods_of", py_func("_the_periods_of", |args: &[Py]| _the_periods_of(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_aims_named", py_func("_aims_named", |args: &[Py]| _aims_named(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_counts_things", py_func("_counts_things", |args: &[Py]| _counts_things(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_figures_in", py_func("_figures_in", |args: &[Py]| _figures_in(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_a_rate", py_func("_a_rate", |args: &[Py]| _a_rate(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_another_kind_follows", py_func("_another_kind_follows", |args: &[Py]| _another_kind_follows(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_near_bare_range", py_func("_near_bare_range", |args: &[Py]| _near_bare_range(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_with_its_unit", py_func("_with_its_unit", |args: &[Py]| _with_its_unit(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_the_number", py_func("_the_number", |args: &[Py]| _the_number(args)));
    pyrt::xrefs::reg_fn("memory.enough", "enough_reply", py_func("enough_reply", |args: &[Py]| enough_reply(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_enough_bare", py_func("_enough_bare", |args: &[Py]| _enough_bare(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_range_near", py_func("_range_near", |args: &[Py]| _range_near(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_bare_figures", py_func("_bare_figures", |args: &[Py]| _bare_figures(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_candidate_answers", py_func("_candidate_answers", |args: &[Py]| _candidate_answers(args)));
    pyrt::xrefs::reg_fn("memory.enough", "_enough_against", py_func("_enough_against", |args: &[Py]| _enough_against(args)));
    pyrt::xrefs::reg_access("memory.enough", "ENOUGH_ASK", g_ENOUGH_ASK, s_ENOUGH_ASK);
    pyrt::xrefs::reg_access("memory.enough", "HOW_MORE", g_HOW_MORE, s_HOW_MORE);
    pyrt::xrefs::reg_access("memory.enough", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("memory.enough", "_AIM_CUE", g__AIM_CUE, s__AIM_CUE);
    pyrt::xrefs::reg_access("memory.enough", "_AMOUNTS", g__AMOUNTS, s__AMOUNTS);
    pyrt::xrefs::reg_access("memory.enough", "_AMOUNT_WORDS", g__AMOUNT_WORDS, s__AMOUNT_WORDS);
    pyrt::xrefs::reg_access("memory.enough", "_A_DOCTOR", g__A_DOCTOR, s__A_DOCTOR);
    pyrt::xrefs::reg_access("memory.enough", "_A_LIMIT", g__A_LIMIT, s__A_LIMIT);
    pyrt::xrefs::reg_access("memory.enough", "_A_PERIOD", g__A_PERIOD, s__A_PERIOD);
    pyrt::xrefs::reg_access("memory.enough", "_A_TEMPERATURE", g__A_TEMPERATURE, s__A_TEMPERATURE);
    pyrt::xrefs::reg_access("memory.enough", "_A_WORD_AFTER", g__A_WORD_AFTER, s__A_WORD_AFTER);
    pyrt::xrefs::reg_access("memory.enough", "_CONVERT", g__CONVERT, s__CONVERT);
    pyrt::xrefs::reg_access("memory.enough", "_COUNT_WORD", g__COUNT_WORD, s__COUNT_WORD);
    pyrt::xrefs::reg_access("memory.enough", "_HOURS_AND_MINUTES", g__HOURS_AND_MINUTES, s__HOURS_AND_MINUTES);
    pyrt::xrefs::reg_access("memory.enough", "_PERIOD_WORDS", g__PERIOD_WORDS, s__PERIOD_WORDS);
    pyrt::xrefs::reg_access("memory.enough", "_TOO_FAR_ABOVE", g__TOO_FAR_ABOVE, s__TOO_FAR_ABOVE);
    pyrt::xrefs::reg_access("memory.enough", "_TOO_FAR_BELOW", g__TOO_FAR_BELOW, s__TOO_FAR_BELOW);
    pyrt::xrefs::reg_access("memory.enough", "_UNIT_SAID", g__UNIT_SAID, s__UNIT_SAID);
    pyrt::xrefs::reg_access("memory.enough", "_WEEKLY", g__WEEKLY, s__WEEKLY);
}

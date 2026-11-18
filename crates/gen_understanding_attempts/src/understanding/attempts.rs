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


thread_local! { static G_ENDS_THE_ATTEMPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDS_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_CLAUSE_AFTER_AND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTS_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STARTS_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.attempts: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ENDS_THE_ATTEMPT() -> Py { ensure_init(); G_ENDS_THE_ATTEMPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENDS_THE_ATTEMPT(v: Py) { ensure_init(); G_ENDS_THE_ATTEMPT.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDS_OVER() -> Py { ensure_init(); G_HANDS_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDS_OVER(v: Py) { ensure_init(); G_HANDS_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_CLAUSE_AFTER_AND() -> Py { ensure_init(); G_NOT_A_CLAUSE_AFTER_AND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CLAUSE_AFTER_AND(v: Py) { ensure_init(); G_NOT_A_CLAUSE_AFTER_AND.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_MORE() -> Py { ensure_init(); G_ONE_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_MORE(v: Py) { ensure_init(); G_ONE_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_SUBJECTS() -> Py { ensure_init(); G_REPORT_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_SUBJECTS(v: Py) { ensure_init(); G_REPORT_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTS_A_CLAUSE() -> Py { ensure_init(); G_STARTS_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTS_A_CLAUSE(v: Py) { ensure_init(); G_STARTS_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_STARTS_A_QUESTION() -> Py { ensure_init(); G_STARTS_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STARTS_A_QUESTION(v: Py) { ensure_init(); G_STARTS_A_QUESTION.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The things a line says the person has already tried, so nothing takes one for what they want.\n\n\"we tried a bath n a story\", \"ive tried the plunger\", \"i tried the cream\" - the thing is named\nas done with. Every reader that looks for what a line is about finds the tried thing first,\nbecause it is a plain noun in the middle of a sentence, and it is the one word in the line the\nperson has just said is behind them:\n\n* \"we tried a bath n a story\" after advice about a toddler who will not sleep was answered with\n  a story made up on the spot, which is the thing they had already done at bedtime;\n* \"ive tried the cream\" after advice about a skin problem was answered with macaroni cheese, on\n  the word \"cream\";\n* \"i tried to bleed the rad but no water came out\" was answered with how to bleed a radiator.\n\nSo this reads the words of the attempt and publishes them as `tried`: the words are what the\nline says is finished, not what it is asking about. A reader that offers a thing on purpose asks\nthis reading first (`bot/knowledge/entertainment.py`, where the fixed forms - a story, a poem, a\nrap, a song - stand down for the form the line says it has already had). Nothing else consults\nit yet: the knowledge lookup still reads the line exactly as it did, so the three lines above are\nanswered as before, and this is the reading to hang those fixes on.\n\nOnly a report is read, never a question: \"have you tried turning it off and on\" is the bot being\nasked, and \"what should i try\" asks for an idea, so neither has anything behind it. And only the\npast tense, because \"i try to walk every day\" is a habit, not a step behind them.\n\nThe same line is often a report and an ask at once - \"i tried a haiku but it was rubbish can you\ndo one\" - so `asks_for_one_more` says whether the words after the attempt hand the making over\nand ask for one more, which is the reading a reader needs before it stands down for a try.\n");
    s_ENDS_THE_ATTEMPT(call_builtin("frozenset", &[mcall(&py_str("\nbut though although however yet still whereas while because since\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTS_A_CLAUSE(call_builtin("frozenset", &[mcall(&py_str("\ni we it he she they you that this there its im hes shes thatll even though\nher him them me us my our\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CLAUSE_AFTER_AND(call_builtin("frozenset", &[mcall(&py_str("\nstill now nothing no none neither nor so then he she they it its i we you that this there\neverything anything anymore\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\na an the it them that this those these one ones thing stuff and or but so\nalready again twice once too also just even really very much more some any all lot lots bit\nbits quite rather sort kind my our your his her their of to in on at for with off out up down\nnot no yeah ok okay well then now since yesterday today tonight morning afternoon evening night\nweek weeks month months year years hour hours day days time times earlier later sooner\neverything anything something nothing anyone someone everyone\nabout around through over back away before after first last both either\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i we ive i've weve we've id wed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOOK_BACK(Py::Int(4));
    s_ONE_MORE(call_builtin("frozenset", &[mcall(&py_str("\nanother one more again instead\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HANDS_OVER(call_builtin("frozenset", &[mcall(&py_str("\nwrite tell make give do sing rap read say come think have got need want\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STARTS_A_QUESTION(call_builtin("frozenset", &[mcall(&py_str("\nwhat whats which how who where when why can could would shall should whether any anything\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(24));
}
pub fn _tokens(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = norm.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    word = mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&word.clone()) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_report(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the \"tried\" at `at` is the person's own report of what they did.");
    if py_truthy(&Py::Bool(py_eq(&at.clone(), &Py::Int(0)))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&at.clone(), &g_LOOK_BACK())]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_REPORT_SUBJECTS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _where_the_attempt_ends(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The index of the first word after the thing the \"tried\" at `at` was done to.\n\n    A hand-over of the making ends it as surely as a new clause does: \"we tried a bath n a story\n    write me another\" is a report and then an ask, and \"write\" is not a thing that was tried.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_add(&at.clone(), &Py::Int(1)), call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    word = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ENDS_THE_ATTEMPT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STARTS_A_CLAUSE(), &word.clone())) } }) {
    return index.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("and"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_CLAUSE_AFTER_AND(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } } } }) {
    return index.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_STARTS_A_QUESTION(), &word.clone()))) {
    before = if py_truthy(&index.clone()) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &op_add(&at.clone(), &Py::Int(1)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("now"), py_str("everything"), py_str("anything"), py_str("something"), py_str("that"), py_str("this"), py_str("it")]), &before.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(5)), &Py::None)) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ONE_MORE(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return index.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HANDS_OVER(), &word.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(5)), &Py::None)) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ONE_MORE(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return index.clone();
    }
    }
    return call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_things_they_tried(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the things the line says were tried, in the order said: `[\"bath\", \"story\"]`.\n\n    Only words that name something are kept: \"i tried that\" and \"ive tried everything\" name no\n    thing, and a line with nothing to drop hands back an empty list.\n    ");
    words = _tokens(&[norm.clone()]);
    tried = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&word.clone(), &py_str("tried"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_report(&[words.clone(), at.clone()]))) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[op_add(&at.clone(), &Py::Int(1)), _where_the_attempt_ends(&[words.clone(), at.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    index = __item.clone();
    word = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&tried.clone(), &word.clone()))) {
    let _ = mcall(&tried.clone(), "append", &[word.clone()]);
    }
    }
    }
    return tried.clone();
    Py::None
}
pub fn the_rest_after_the_try(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words left in the line once the thing it says was tried is behind it.\n\n    \"i tried a rap but it was rubbish write me another\" is a report and an ask in one line, and\n    only the report is finished with. This hands back the ask - the words after the attempt -\n    so a reader that stood down for the try can see the person asking for one anyway.\n    ");
    words = _tokens(&[norm.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&word.clone(), &py_str("tried"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_report(&[words.clone(), at.clone()]))) } }) {
    continue;
    }
    return slice_or_raise(&words.clone(), &_where_the_attempt_ends(&[words.clone(), at.clone()]), &Py::None, &Py::None);
    }
    return py_list(vec![]);
    Py::None
}
pub fn asks_for_one_more(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut joined: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line, past whatever was tried, asks for one more of it.\n\n    Needs both halves: handing the making over (\"write\", \"do\", \"can you\") and the word for one\n    more (\"another\", \"one more\", \"again\", \"instead\"). A line that only asks what to do about the\n    try (\"i tried the cream and it stung, what now\") asks for nothing more, so the reader that\n    offered the tried thing still stands down for it.\n    ");
    rest = the_rest_after_the_try(&[norm.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ONE_MORE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HANDS_OVER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    joined = mcall(&py_str(" "), "join", &[rest.clone()]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:can|could|would|will)\\s+(?:you|u)\\b"), joined.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STARTS_A_QUESTION(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HANDS_OVER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bfor (?:me|us)\\b|\\bplease\\b"), joined.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn things_tried(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { Py::None };
    let mut words: Py = Py::None;
    let _ = py_str("`the_things_they_tried` for a typed line, or [] when the line is too long to be a report.");
    words = if py_truthy(&Py::Bool(!py_is(&norm.clone(), &Py::None))) { _tokens(&[norm.clone()]) } else { _tokens(&[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e))]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&words.clone(), &py_str("tried"))) } } } }) {
    return py_list(vec![]);
    }
    return the_things_they_tried(&[words.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.attempts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ENDS_THE_ATTEMPT" => Some(g_ENDS_THE_ATTEMPT()),
            "HANDS_OVER" => Some(g_HANDS_OVER()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NOT_A_CLAUSE_AFTER_AND" => Some(g_NOT_A_CLAUSE_AFTER_AND()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "ONE_MORE" => Some(g_ONE_MORE()),
            "REPORT_SUBJECTS" => Some(g_REPORT_SUBJECTS()),
            "STARTS_A_CLAUSE" => Some(g_STARTS_A_CLAUSE()),
            "STARTS_A_QUESTION" => Some(g_STARTS_A_QUESTION()),
            "_a_report" => Some(py_func("_a_report", |args: &[Py]| _a_report(args))),
            "_tokens" => Some(py_func("_tokens", |args: &[Py]| _tokens(args))),
            "_where_the_attempt_ends" => Some(py_func("_where_the_attempt_ends", |args: &[Py]| _where_the_attempt_ends(args))),
            "asks_for_one_more" => Some(py_func("asks_for_one_more", |args: &[Py]| asks_for_one_more(args))),
            "the_rest_after_the_try" => Some(py_func("the_rest_after_the_try", |args: &[Py]| the_rest_after_the_try(args))),
            "the_things_they_tried" => Some(py_func("the_things_they_tried", |args: &[Py]| the_things_they_tried(args))),
            "things_tried" => Some(py_func("things_tried", |args: &[Py]| things_tried(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ENDS_THE_ATTEMPT" => s_ENDS_THE_ATTEMPT(v),
            "HANDS_OVER" => s_HANDS_OVER(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NOT_A_CLAUSE_AFTER_AND" => s_NOT_A_CLAUSE_AFTER_AND(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "ONE_MORE" => s_ONE_MORE(v),
            "REPORT_SUBJECTS" => s_REPORT_SUBJECTS(v),
            "STARTS_A_CLAUSE" => s_STARTS_A_CLAUSE(v),
            "STARTS_A_QUESTION" => s_STARTS_A_QUESTION(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.attempts", module_obj);
    pyrt::xrefs::reg_fn("understanding.attempts", "_tokens", py_func("_tokens", |args: &[Py]| _tokens(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "_a_report", py_func("_a_report", |args: &[Py]| _a_report(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "_where_the_attempt_ends", py_func("_where_the_attempt_ends", |args: &[Py]| _where_the_attempt_ends(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "the_things_they_tried", py_func("the_things_they_tried", |args: &[Py]| the_things_they_tried(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "the_rest_after_the_try", py_func("the_rest_after_the_try", |args: &[Py]| the_rest_after_the_try(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "asks_for_one_more", py_func("asks_for_one_more", |args: &[Py]| asks_for_one_more(args)));
    pyrt::xrefs::reg_fn("understanding.attempts", "things_tried", py_func("things_tried", |args: &[Py]| things_tried(args)));
    pyrt::xrefs::reg_access("understanding.attempts", "ENDS_THE_ATTEMPT", g_ENDS_THE_ATTEMPT, s_ENDS_THE_ATTEMPT);
    pyrt::xrefs::reg_access("understanding.attempts", "HANDS_OVER", g_HANDS_OVER, s_HANDS_OVER);
    pyrt::xrefs::reg_access("understanding.attempts", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("understanding.attempts", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("understanding.attempts", "NOT_A_CLAUSE_AFTER_AND", g_NOT_A_CLAUSE_AFTER_AND, s_NOT_A_CLAUSE_AFTER_AND);
    pyrt::xrefs::reg_access("understanding.attempts", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("understanding.attempts", "ONE_MORE", g_ONE_MORE, s_ONE_MORE);
    pyrt::xrefs::reg_access("understanding.attempts", "REPORT_SUBJECTS", g_REPORT_SUBJECTS, s_REPORT_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.attempts", "STARTS_A_CLAUSE", g_STARTS_A_CLAUSE, s_STARTS_A_CLAUSE);
    pyrt::xrefs::reg_access("understanding.attempts", "STARTS_A_QUESTION", g_STARTS_A_QUESTION, s_STARTS_A_QUESTION);
}

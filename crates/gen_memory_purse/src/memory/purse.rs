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


thread_local! { static G__A_CLAUSE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY_BAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE_BAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.purse: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g__A_CLAUSE_END() -> Py { ensure_init(); G__A_CLAUSE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_END(v: Py) { ensure_init(); G__A_CLAUSE_END.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY() -> Py { ensure_init(); G__A_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY(v: Py) { ensure_init(); G__A_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY_BAND() -> Py { ensure_init(); G__A_MONEY_BAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY_BAND(v: Py) { ensure_init(); G__A_MONEY_BAND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE_BAND() -> Py { ensure_init(); G__A_RATE_BAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE_BAND(v: Py) { ensure_init(); G__A_RATE_BAND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB_FIRST() -> Py { ensure_init(); G__A_VERB_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB_FIRST(v: Py) { ensure_init(); G__A_VERB_FIRST.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whether their money is enough: a price read against the price band the answer named.\n\n\"is that enough\" is about an amount the person gave, and the answer that priced the thing has the\nfigure in it. Money is the amount people ask it about most, and the one the answer writes as a\nband of prices rather than a number to reach:\n\n    what coat should i get for winter\n    -> Pick the coat by the weather you are actually out in: ... £80 to £150 buys a padded coat\n       that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.\n    i have 80 pounds, is that enough\n    -> £80 is inside the £80 to £150 that answer named, and that buys a padded coat that lasts a\n       decade.\n\n    i have 200 pounds to spend on a bike\n    -> £400 to £700 buys a dependable new commuter, and £150 to £300 buys a sound second-hand one.\n    is that enough\n    -> £200 is inside the £150 to £300 that answer named, and that buys a sound second-hand one.\n\nOn main those two chats got \"I'm not sure about that one, sorry - I'd rather not guess.\" and\n\"Alright then - good luck with it.\", because the amounts the store reads in\n`bot/modules/memory_store.py` (`_AMOUNTS`) are all a unit that follows the count, and a price is\nwritten with the mark in front (\"£200\") or the word behind (\"200 pounds\"), so it was in no family\nat all and the two numbers were never read together. `bot/modules/memory_store.py` is the module\nthat answers \"is that enough\", and it calls `against_the_bands` from there.\n\nFences, each a test in `tests/test_a_purse_against_the_price_band.py`:\n\n* a band has to hold their money, or be the cheapest one above it - the one they cannot reach;\n  money over every band is left to the answer that named them, which usually says what more money\n  buys (\"£900\" after a Rome answer whose best line is the daily cost);\n* a band stated as a rate is the cost of a day, not the price of the thing (\"£55 to £95 a day per\n  person\"), so it is passed over;\n* the figure can be in the ask (\"is 40 pounds enough\") or in their own line just before it, and\n  the band can be in the answer one turn back, because a line that only says how much they have is\n  answered by the store asking what it has to stretch to;\n* a price named alone, with no band, is not stood against (\"You have 500 pounds for a laptop -\n  good, and everything I suggest will be within it\");\n* only pounds and quid are read, so the reply never puts a pound sign on somebody's euros.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.purse._A_MONEY"); }
    s__A_MONEY(stdlib_call("re", "compile", &[py_str("\\u00a3\\s*(?P<mark>\\d+(?:,\\d{3})*(?:\\.\\d+)?)|(?P<word>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:pounds?|quid|gbp)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.purse._A_MONEY_BAND"); }
    s__A_MONEY_BAND(stdlib_call("re", "compile", &[py_str("\\u00a3\\s*(?P<a1>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:to|and|or|-|\\u2013)\\s*\\u00a3?\\s*(?P<a2>\\d+(?:,\\d{3})*(?:\\.\\d+)?)|(?P<b1>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:to|and|or|-|\\u2013)\\s*(?P<b2>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:pounds?|quid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.purse._A_CLAUSE_END"); }
    s__A_CLAUSE_END(stdlib_call("re", "compile", &[py_str("[.!?;]|\\s+-\\s+|\\u2014|,\\s+(?:and|or|but|so)\\s+|,\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.purse._A_VERB_FIRST"); }
    s__A_VERB_FIRST(stdlib_call("re", "compile", &[py_str("^(?:buys?|covers?|gets?|is|are|was|were|will|goes?|holds?|pays?|takes?|does|fits?|lasts?|means|comes?|makes?|gives?|finds?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.purse._A_RATE_BAND"); }
    s__A_RATE_BAND(stdlib_call("re", "compile", &[py_str("^\\s*(?:\\(?\\s*)?(?:a|per|each|every)\\s+(?:day|night|week|month|person|head|hour)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_number(args: &[Py]) -> Py {
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
    let _ = py_str("The number a figure stands for, or 0 when it is not one.");
    word = mcall(&mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d+(?:,\\d{3})*(?:\\.\\d+)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("float", &[mcall(&word.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _money_in(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every price in the line, in the order it was said.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g__A_MONEY(), "finditer", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    match_ = __item.clone();
    value = _a_number(&[{ let __b = mcall(&match_.clone(), "group", &[py_str("mark")]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[py_str("word")]) } }]);
    if py_truthy(&value.clone()) {
    let _ = mcall(&found.clone(), "append", &[value.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_money(args: &[Py]) -> Py {
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
    let _ = py_str("A price written the way money is written: \"£1,500\", \"£2.50\" - never \"£1500.0\".");
    if py_truthy(&mcall(&call_builtin("float", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "is_integer", &[])) {
    return mcall(&py_str("£{:,}"), "format", &[call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return op_modulo(&py_str("£%.2f"), &value.clone());
    Py::None
}
pub fn _the_words_after(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut one: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The clause the answer puts after a price band, or \"\" when it puts none worth repeating.");
    rest = slice_or_raise(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &at.clone(), &Py::None, &Py::None);
    cut = mcall(&g__A_CLAUSE_END(), "search", &[rest.clone()]);
    words = mcall(&if py_truthy(&cut.clone()) { slice_or_raise(&rest.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None) } else { rest.clone() }, "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&words.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("£"))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&words.clone(), "split", &[])) { let mut one: Py = __item.clone(); __out.push(stdlib_call("re", "fullmatch", &[py_str("\\d[\\d,]*\\.?\\d*"), one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    return words.clone();
    Py::None
}
pub fn _money_bands(args: &[Py]) -> Py {
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
    let mut bands: Py = Py::None;
    let mut first: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("Every price band the answer names: (low, high, the clause it is written with).");
    bands = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g__A_MONEY_BAND(), "finditer", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    match_ = __item.clone();
    first = { let __b = mcall(&match_.clone(), "group", &[py_str("a1")]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[py_str("b1")]) } };
    second = { let __b = mcall(&match_.clone(), "group", &[py_str("a2")]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[py_str("b2")]) } };
    let __unpk2 = py_unpack(&py_tuple(vec![_a_number(&[first.clone()]), _a_number(&[second.clone()])]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { high.clone() } })); if py_truthy(&__b) { __b } else { op_ge(&low.clone(), &high.clone()) } }) {
    continue;
    }
    if py_truthy(&mcall(&g__A_RATE_BAND(), "match", &[slice_or_raise(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    let _ = mcall(&bands.clone(), "append", &[py_tuple(vec![low.clone(), high.clone(), _the_words_after(&[answer.clone(), mcall(&match_.clone(), "end", &[])])])]);
    }
    return bands.clone();
    Py::None
}
pub fn against_the_bands(args: &[Py]) -> Py {
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
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut above: Py = Py::None;
    let mut after: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut band: Py = Py::None;
    let mut bands: Py = Py::None;
    let mut high: Py = Py::None;
    let mut inside: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Where their money stands against the price bands of the last answer, or None.\n\n    The figure is read from the ask first (\"is 40 pounds enough\"), and from their own lines just\n    before it when the ask only says \"is that enough\". A band that holds their money is the\n    answer, said with the clause the answer wrote for that band, so the reply carries what the\n    price buys rather than the price twice.\n    ");
    theirs = _money_in(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    theirs = _money_in(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&theirs.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    return Py::None;
    }
    mine = geti(&theirs.clone(), &op_neg(&Py::Int(1)));
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    let mut __else3 = false;
    for __item in __iter2 {
    turn = __item.clone();
    answer = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    bands = _money_bands(&[answer.clone()]);
    if py_truthy(&bands.clone()) {
    break;
    }
    }
    return Py::None;
    spoken = _the_money(&[mine.clone()]);
    inside = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let mut band: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = mine.clone(); if py_truthy(&op_le(&geti(&band.clone(), &Py::Int(0)), &__cmp11)) { op_le(&__cmp11, &geti(&band.clone(), &Py::Int(1))) } else { Py::Bool(false) } }) {  __out.push(band.clone()); } } py_list(__out) })() };
    if py_truthy(&inside.clone()) {
    let __unpk4 = py_unpack(&geti(&inside.clone(), &Py::Int(0)), 3, None);
    low = __unpk4[0].clone();
    high = __unpk4[1].clone();
    after = __unpk4[2].clone();
    said = op_modulo(&py_str("%s to %s"), &py_tuple(vec![_the_money(&[low.clone()]), _the_money(&[high.clone()])]));
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return op_modulo(&py_str("%s is inside the %s that answer named."), &py_tuple(vec![spoken.clone(), said.clone()]));
    }
    if py_truthy(&mcall(&g__A_VERB_FIRST(), "match", &[after.clone()])) {
    return op_modulo(&py_str("%s is inside the %s that answer named, and that %s."), &py_tuple(vec![spoken.clone(), said.clone(), after.clone()]));
    }
    return op_modulo(&py_str("%s is inside the %s that answer named: %s."), &py_tuple(vec![spoken.clone(), said.clone(), after.clone()]));
    }
    above = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let mut band: Py = __item.clone(); if py_truthy(&op_gt(&geti(&band.clone(), &Py::Int(0)), &mine.clone())) {  __out.push(band.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), { let above = above.clone(); let after = after.clone(); let answer = answer.clone(); let band = band.clone(); let bands = bands.clone(); let high = high.clone(); let inside = inside.clone(); let low = low.clone(); let mine = mine.clone(); let said = said.clone(); let spoken = spoken.clone(); let text = text.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let turns = turns.clone(); py_func("lambda", move |args: &[Py]| { let above = above.clone(); let after = after.clone(); let answer = answer.clone(); let band = band.clone(); let bands = bands.clone(); let high = high.clone(); let inside = inside.clone(); let low = low.clone(); let mine = mine.clone(); let said = said.clone(); let spoken = spoken.clone(); let text = text.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let turns = turns.clone(); let band: Py = __kwp(args, 0, "band").unwrap_or(Py::None); geti(&band.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&above.clone()))) {
    return Py::None;
    }
    return op_modulo(&py_str("%s is under the %s that answer named."), &py_tuple(vec![spoken.clone(), op_modulo(&py_str("%s to %s"), &py_tuple(vec![_the_money(&[geti(&geti(&above.clone(), &Py::Int(0)), &Py::Int(0))]), _the_money(&[geti(&geti(&above.clone(), &Py::Int(0)), &Py::Int(1))])]))]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.purse".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_A_CLAUSE_END" => Some(g__A_CLAUSE_END()),
            "_A_MONEY" => Some(g__A_MONEY()),
            "_A_MONEY_BAND" => Some(g__A_MONEY_BAND()),
            "_A_RATE_BAND" => Some(g__A_RATE_BAND()),
            "_A_VERB_FIRST" => Some(g__A_VERB_FIRST()),
            "_a_number" => Some(py_func("_a_number", |args: &[Py]| _a_number(args))),
            "_money_bands" => Some(py_func("_money_bands", |args: &[Py]| _money_bands(args))),
            "_money_in" => Some(py_func("_money_in", |args: &[Py]| _money_in(args))),
            "_the_money" => Some(py_func("_the_money", |args: &[Py]| _the_money(args))),
            "_the_words_after" => Some(py_func("_the_words_after", |args: &[Py]| _the_words_after(args))),
            "against_the_bands" => Some(py_func("against_the_bands", |args: &[Py]| against_the_bands(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_A_CLAUSE_END" => s__A_CLAUSE_END(v),
            "_A_MONEY" => s__A_MONEY(v),
            "_A_MONEY_BAND" => s__A_MONEY_BAND(v),
            "_A_RATE_BAND" => s__A_RATE_BAND(v),
            "_A_VERB_FIRST" => s__A_VERB_FIRST(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.purse", module_obj);
    pyrt::xrefs::reg_fn("memory.purse", "_a_number", py_func("_a_number", |args: &[Py]| _a_number(args)));
    pyrt::xrefs::reg_fn("memory.purse", "_money_in", py_func("_money_in", |args: &[Py]| _money_in(args)));
    pyrt::xrefs::reg_fn("memory.purse", "_the_money", py_func("_the_money", |args: &[Py]| _the_money(args)));
    pyrt::xrefs::reg_fn("memory.purse", "_the_words_after", py_func("_the_words_after", |args: &[Py]| _the_words_after(args)));
    pyrt::xrefs::reg_fn("memory.purse", "_money_bands", py_func("_money_bands", |args: &[Py]| _money_bands(args)));
    pyrt::xrefs::reg_fn("memory.purse", "against_the_bands", py_func("against_the_bands", |args: &[Py]| against_the_bands(args)));
    pyrt::xrefs::reg_access("memory.purse", "_A_CLAUSE_END", g__A_CLAUSE_END, s__A_CLAUSE_END);
    pyrt::xrefs::reg_access("memory.purse", "_A_MONEY", g__A_MONEY, s__A_MONEY);
    pyrt::xrefs::reg_access("memory.purse", "_A_MONEY_BAND", g__A_MONEY_BAND, s__A_MONEY_BAND);
    pyrt::xrefs::reg_access("memory.purse", "_A_RATE_BAND", g__A_RATE_BAND, s__A_RATE_BAND);
    pyrt::xrefs::reg_access("memory.purse", "_A_VERB_FIRST", g__A_VERB_FIRST, s__A_VERB_FIRST);
}

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
        let _ = exc::catch(|| { module_body(); Py::None });
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
    s__A_MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\u00a3\\s*(?P<mark>\\d+(?:,\\d{3})*(?:\\.\\d+)?)|(?P<word>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:pounds?|quid|gbp)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MONEY_BAND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\u00a3\\s*(?P<a1>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:to|and|or|-|\\u2013)\\s*\\u00a3?\\s*(?P<a2>\\d+(?:,\\d{3})*(?:\\.\\d+)?)|(?P<b1>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:to|and|or|-|\\u2013)\\s*(?P<b2>\\d+(?:,\\d{3})*(?:\\.\\d+)?)\\s*(?:pounds?|quid)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CLAUSE_END({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.!?;]|\\s+-\\s+|\\u2014|,\\s+(?:and|or|but|so)\\s+|,\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_VERB_FIRST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:buys?|covers?|gets?|is|are|was|were|will|goes?|holds?|pays?|takes?|does|fits?|lasts?|means|comes?|makes?|gives?|finds?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_RATE_BAND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:\\(?\\s*)?(?:a|per|each|every)\\s+(?:day|night|week|month|person|head|hour)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+(?:,\\d{3})*(?:\\.\\d+)?")); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MONEY(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mark")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_number(args))(&__args) };
    if py_truthy(&value.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
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
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("£{:,}"), "format", args))(&__args) };
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
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CLAUSE_END(), "search", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&cut.clone()) { slice_or_raise(&rest.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "start", args))(&__args) }, &Py::None) } else { rest.clone() }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("£"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d[\\d,]*\\.?\\d*")); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
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
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MONEY_BAND(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    first = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a1")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("b1")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } };
    second = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("b2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } };
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_number(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(second.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_number(args))(&__args) }]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { high.clone() } })); if py_truthy(&__b) { __b } else { op_ge(&low.clone(), &high.clone()) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RATE_BAND(), "match", args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![low.clone(), high.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words_after(args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bands.clone(), "append", args))(&__args) };
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
    theirs = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    theirs = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money_in(args))(&__args) };
    if py_truthy(&theirs.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    return Py::None;
    }
    mine = geti(&theirs.clone(), &op_neg(&Py::Int(1)));
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let mut __else3 = false;
    for __item in __iter2 {
    turn = __item.clone();
    answer = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    bands = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money_bands(args))(&__args) };
    if py_truthy(&bands.clone()) {
    break;
    }
    }
    return Py::None;
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money(args))(&__args) };
    inside = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let mut band: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = mine.clone(); if py_truthy(&op_le(&geti(&band.clone(), &Py::Int(0)), &__cmp11)) { op_le(&__cmp11, &geti(&band.clone(), &Py::Int(1))) } else { Py::Bool(false) } }) {  __out.push(band.clone()); } } py_list(__out) })() };
    if py_truthy(&inside.clone()) {
    let __unpk4 = py_unpack(&geti(&inside.clone(), &Py::Int(0)), 3, None);
    low = __unpk4[0].clone();
    high = __unpk4[1].clone();
    after = __unpk4[2].clone();
    said = op_modulo(&py_str("%s to %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(high.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money(args))(&__args) }]));
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return op_modulo(&py_str("%s is inside the %s that answer named."), &py_tuple(vec![spoken.clone(), said.clone()]));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_VERB_FIRST(), "match", args))(&__args) }) {
    return op_modulo(&py_str("%s is inside the %s that answer named, and that %s."), &py_tuple(vec![spoken.clone(), said.clone(), after.clone()]));
    }
    return op_modulo(&py_str("%s is inside the %s that answer named: %s."), &py_tuple(vec![spoken.clone(), said.clone(), after.clone()]));
    }
    above = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bands.clone()) { let mut band: Py = __item.clone(); if py_truthy(&op_gt(&geti(&band.clone(), &Py::Int(0)), &mine.clone())) {  __out.push(band.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), { let above = above.clone(); let after = after.clone(); let answer = answer.clone(); let band = band.clone(); let bands = bands.clone(); let high = high.clone(); let inside = inside.clone(); let low = low.clone(); let mine = mine.clone(); let said = said.clone(); let spoken = spoken.clone(); let text = text.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let turns = turns.clone(); py_func("lambda", move |args: &[Py]| { let above = above.clone(); let after = after.clone(); let answer = answer.clone(); let band = band.clone(); let bands = bands.clone(); let high = high.clone(); let inside = inside.clone(); let low = low.clone(); let mine = mine.clone(); let said = said.clone(); let spoken = spoken.clone(); let text = text.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let turns = turns.clone(); let band: Py = __kwp(args, 0, "band").unwrap_or(Py::None); geti(&band.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&above.clone()))) {
    return Py::None;
    }
    return op_modulo(&py_str("%s is under the %s that answer named."), &py_tuple(vec![spoken.clone(), op_modulo(&py_str("%s to %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&above.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&above.clone(), &Py::Int(0)), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_money(args))(&__args) }]))]));
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

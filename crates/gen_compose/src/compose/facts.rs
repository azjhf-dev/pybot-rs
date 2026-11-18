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


thread_local! { static G_A_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_TOKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_MOMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_CLOCK() -> Py { ensure_init(); G_A_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLOCK(v: Py) { ensure_init(); G_A_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONTH() -> Py { ensure_init(); G_A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONTH(v: Py) { ensure_init(); G_A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_TOKEN() -> Py { ensure_init(); G_DATE_TOKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_TOKEN(v: Py) { ensure_init(); G_DATE_TOKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_MOMENT() -> Py { ensure_init(); G_HOW_TO_MOMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_MOMENT(v: Py) { ensure_init(); G_HOW_TO_MOMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASK() -> Py { ensure_init(); G_WHEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); G_WHEN_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A card is not the date the person has already given.\n\n\"when do we fly\" and \"when is my dentist appointment\" ask for a fact the person has told the bot,\nand the memory store hands the fact straight back - \"In July.\", \"You're at the dentist on\ntuesday.\" Both were answered by a knowledge card that read the subject and none of the date: the\nairport check-in card for the flight, and \"ring practices near you and ask whether they take new\nNHS patients\" to somebody who already has an appointment.\n\nSo the reading is two questions, kept apart here and put together by the arbiter: does the line ask\n*when* about the thing, and does a reply carry the date? A card that answers a different question\nthan the one asked then stands down, and nothing else about it changes.\n");
    s_WHEN_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:and|so|but|ok|okay|oh|ah|right|well|yes|yeah|hi|hey|please|then|now|quick|sorry|also)\\W+)*(?:when|what\\s+time|what\\s+day|which\\s+day|what\\s+date|which\\s+date)\\s+(?:is|are|was|were|do|does|did|will|shall|have|has|am)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_TO_MOMENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:best|good|right|ideal|worst)\\s+time\\b|\\btime\\s+to\\s+(?:plant|sow|prune|book|go|start|take|give|use|apply|cut|paint|ring)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MONTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|jun(?:e)?|jul(?:y)?|aug(?:ust)?|sep(?:t|tember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CLOCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at|by|around|about|from|until|till)\\s+\\d{1,2}(?::\\d{2})?\\s*(?:am|pm|a\\.m|p\\.m|o'?clock)?\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOST_WORDS(Py::Int(45));
    s_DATE_TOKEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mon|tues?|wed(?:nes)?|thur?s?|fri|sat(?:ur)?|sun(?:day)?)(?:day)?\\b|\\b(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\\b|\\b\\d{1,2}(?:st|nd|rd|th)\\b|\\b\\d{1,2}(?::\\d{2})?\\s*(?:am|pm|a\\.m|p\\.m|o'?clock)\\b|\\b(?:19|20)\\d{2}\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn asks_for_a_when(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks when something is, rather than how to do it.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_TO_MOMENT(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHEN_ASK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn holds_a_date(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether a reply carries a date, a weekday or a clock time.\n\n    `narrow.has_a_date` already knows what a written date looks like, and it counts a bare number\n    only when it is not money, so \"over £150,000\" is not read as a date here either.\n    ");
    said = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::narrow::has_a_date(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CLOCK(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _date_words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DATE_TOKEN(), "findall", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "rstrip", args))(&__args) }); } py_set(__out) })() };
    Py::None
}
pub fn the_same_date(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut two: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "two") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: two"))) };
    let mut first: Py = Py::None;
    let _ = py_str("Whether two replies name the same date, so a card that carries it needs no help.");
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _date_words(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = first.clone(); if !py_truthy(&__b) { __b } else { op_bitand(&first.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(two.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _date_words(args))(&__args) }) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_fact_asked_for(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("Whether this reply is the date the line asked for.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_for_a_when(args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_MOST_WORDS())) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| holds_a_date(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.facts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CLOCK" => Some(g_A_CLOCK()),
            "A_MONTH" => Some(g_A_MONTH()),
            "DATE_TOKEN" => Some(g_DATE_TOKEN()),
            "HOW_TO_MOMENT" => Some(g_HOW_TO_MOMENT()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "_date_words" => Some(py_func("_date_words", |args: &[Py]| _date_words(args))),
            "asks_for_a_when" => Some(py_func("asks_for_a_when", |args: &[Py]| asks_for_a_when(args))),
            "holds_a_date" => Some(py_func("holds_a_date", |args: &[Py]| holds_a_date(args))),
            "the_fact_asked_for" => Some(py_func("the_fact_asked_for", |args: &[Py]| the_fact_asked_for(args))),
            "the_same_date" => Some(py_func("the_same_date", |args: &[Py]| the_same_date(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CLOCK" => s_A_CLOCK(v),
            "A_MONTH" => s_A_MONTH(v),
            "DATE_TOKEN" => s_DATE_TOKEN(v),
            "HOW_TO_MOMENT" => s_HOW_TO_MOMENT(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "WHEN_ASK" => s_WHEN_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.facts", module_obj);
    pyrt::xrefs::reg_fn("compose.facts", "asks_for_a_when", py_func("asks_for_a_when", |args: &[Py]| asks_for_a_when(args)));
    pyrt::xrefs::reg_fn("compose.facts", "holds_a_date", py_func("holds_a_date", |args: &[Py]| holds_a_date(args)));
    pyrt::xrefs::reg_fn("compose.facts", "_date_words", py_func("_date_words", |args: &[Py]| _date_words(args)));
    pyrt::xrefs::reg_fn("compose.facts", "the_same_date", py_func("the_same_date", |args: &[Py]| the_same_date(args)));
    pyrt::xrefs::reg_fn("compose.facts", "the_fact_asked_for", py_func("the_fact_asked_for", |args: &[Py]| the_fact_asked_for(args)));
    pyrt::xrefs::reg_access("compose.facts", "A_CLOCK", g_A_CLOCK, s_A_CLOCK);
    pyrt::xrefs::reg_access("compose.facts", "A_MONTH", g_A_MONTH, s_A_MONTH);
    pyrt::xrefs::reg_access("compose.facts", "DATE_TOKEN", g_DATE_TOKEN, s_DATE_TOKEN);
    pyrt::xrefs::reg_access("compose.facts", "HOW_TO_MOMENT", g_HOW_TO_MOMENT, s_HOW_TO_MOMENT);
    pyrt::xrefs::reg_access("compose.facts", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("compose.facts", "WHEN_ASK", g_WHEN_ASK, s_WHEN_ASK);
}

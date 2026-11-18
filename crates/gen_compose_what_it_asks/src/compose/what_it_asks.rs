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


thread_local! { static G_ALTERNATIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_VERB_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PRESENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_LOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPEND_IT_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.what_it_asks: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALTERNATIVES() -> Py { ensure_init(); G_ALTERNATIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALTERNATIVES(v: Py) { ensure_init(); G_ALTERNATIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT() -> Py { ensure_init(); G_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT(v: Py) { ensure_init(); G_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_VERB_ASK() -> Py { ensure_init(); G_MONEY_VERB_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_VERB_ASK(v: Py) { ensure_init(); G_MONEY_VERB_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PRESENT_ASK() -> Py { ensure_init(); G_NOT_A_PRESENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PRESENT_ASK(v: Py) { ensure_init(); G_NOT_A_PRESENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTIONS() -> Py { ensure_init(); G_OPTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTIONS(v: Py) { ensure_init(); G_OPTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK() -> Py { ensure_init(); G_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK(v: Py) { ensure_init(); G_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_LOOKING() -> Py { ensure_init(); G_PRESENT_LOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_LOOKING(v: Py) { ensure_init(); G_PRESENT_LOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g_SPEND_IT_ON() -> Py { ensure_init(); G_SPEND_IT_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPEND_IT_ON(v: Py) { ensure_init(); G_SPEND_IT_ON.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The shape a line asks for, where `fit.wants` reads it as something else.\n\n`fit.wants` reads the question words of a line and the kinds of thing it names, and for three\neveryday asks it lands on a shape that is not the one the person is owed. Each is read from the\nperson's own words, and nothing else comes back changed.\n\nA money verb after \"what should i\" is read as a method, because the chain reads a `what should i`\nas \"how do I\". The money verb is what makes it a question of how much:\n\n    what should i tip my hairdresser        -> amount\n    what should i spend on my mums birthday -> amount\n\n\"what should i spend it on\" is the other way round: the money is in hand and the person wants the\nthing to spend it on, so the `it` keeps it a pick, and that `it` is the whole difference from\n\"what should i spend on my mum\".\n\nAnd a present asked for by who or what it is for asks nothing the chain can see: \"present for a new\npuppy\", \"gifts for kids under a tenner\" carry no question word, so the shape was left unset and the\nreply was never checked against the ask at all. A present asked for is a few things to choose\nbetween.\n\nThe last line of each arm is the one that names alternatives: \"what should i pay my landlord first\nthe rent or the gas bill\" has a money verb and an object, and is still a pick, because a line that\nputs two things either side of an `or` is asking which. That is read as a stand-down rather than a\npick: the chain has already read it, and nothing here overrides a reading it did not come for.\n\nIt is read after `fit.wants`. The present arm only fills a shape the chain left unset; the two\nmoney arms read the line themselves and stand down on a line that names alternatives.\n");
    s_AMOUNT(pyrt::xrefs::xget("compose.fit", "AMOUNT"));
    s_OPTIONS(pyrt::xrefs::xget("compose.fit", "OPTIONS"));
    s_PICK(pyrt::xrefs::xget("compose.fit", "PICK"));
    s_MONEY_VERB_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat should (?:i|we) (?:tip\\b|spend (?:on)(?! it)\\b|budget for\\b|pay (?:for|a|an|my|the|him|her|them)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEND_IT_ON(stdlib_call("re", "compile", &[py_str("\\bwhat (?:should|shall|do) (?:i|we) (?:spend|put) it (?:on|into|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRESENT_LOOKING(stdlib_call("re", "compile", &[py_str("\\b(?:present|gift|pressie)s?\\b[^?.!]{0,20}?\\bfor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PRESENT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:thank(?:s| you)|refund|receipt|return(?:ed|ing)?|buying|bought|got|gave|given|giving|keeps)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALTERNATIVES(stdlib_call("re", "compile", &[py_str("\\bor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _names_alternatives(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut part: Py = Py::None;
    let mut seat: Py = Py::None;
    let mut spent: Py = Py::None;
    let mut window: Py = Py::None;
    let _ = py_str("Whether the clause the money verb is in puts two things either side of an `or`.\n\n    Read clause by clause, so an `or` in a later sentence (\"what should i tip a hairdresser? or is\n    ten percent rude\") does not take the ask away. Standing down rather than answering `PICK`,\n    because the chain has already read the line and this is the arm for a line it read as a method.\n    ");
    window = slice_or_raise(&line.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(80))]).unwrap_or_else(|e| exc::raise(e)), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(80)), &Py::None);
    seat = op_sub(&mcall(&match_.clone(), "start", &[]), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(80))]).unwrap_or_else(|e| exc::raise(e)));
    spent = Py::Int(0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[,;:?.!]"), window.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __cmp21: Py = seat.clone(); if py_truthy(&op_le(&spent.clone(), &__cmp21)) { op_le(&__cmp21, &op_add(&spent.clone(), &call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)))) } else { Py::Bool(false) } }) {
    return call_builtin("bool", &[mcall(&g_ALTERNATIVES(), "search", &[part.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    let __aug3 = op_add(&spent.clone(), &op_add(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    spent = __aug3;
    }
    return call_builtin("bool", &[mcall(&g_ALTERNATIVES(), "search", &[window.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_line_once(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The line as the person typed it, once.\n\n    `fit._said` piles the reading's own copies of the line together - `norm`, `norm_text`, `clean`\n    and `text` - which is right for a word searched anywhere and wrong for a shape read from the\n    head of the line or the end of it.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plain"), py_str("text"), py_str("clean")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&understood.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&value.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn the_shape_it_asks(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = Py::None;
    let mut money: Py = Py::None;
    let _ = py_str("`want`, corrected for the three asks above; every other line comes back as it was read.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return want.clone();
    }
    line = _the_line_once(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return want.clone();
    }
    if py_truthy(&mcall(&g_SPEND_IT_ON(), "search", &[line.clone()])) {
    return pyrt::xrefs::xget("compose.fit", "PICK");
    }
    money = mcall(&g_MONEY_VERB_ASK(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = money.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_alternatives(&[line.clone(), money.clone()]))) } }) {
    return pyrt::xrefs::xget("compose.fit", "AMOUNT");
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&want.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PRESENT_LOOKING(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_PRESENT_ASK(), "search", &[line.clone()]))) } } } }) {
    return pyrt::xrefs::xget("compose.fit", "OPTIONS");
    }
    return want.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.what_it_asks".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALTERNATIVES" => Some(g_ALTERNATIVES()),
            "AMOUNT" => Some(g_AMOUNT()),
            "MONEY_VERB_ASK" => Some(g_MONEY_VERB_ASK()),
            "NOT_A_PRESENT_ASK" => Some(g_NOT_A_PRESENT_ASK()),
            "OPTIONS" => Some(g_OPTIONS()),
            "PICK" => Some(g_PICK()),
            "PRESENT_LOOKING" => Some(g_PRESENT_LOOKING()),
            "SPEND_IT_ON" => Some(g_SPEND_IT_ON()),
            "_names_alternatives" => Some(py_func("_names_alternatives", |args: &[Py]| _names_alternatives(args))),
            "_the_line_once" => Some(py_func("_the_line_once", |args: &[Py]| _the_line_once(args))),
            "the_shape_it_asks" => Some(py_func("the_shape_it_asks", |args: &[Py]| the_shape_it_asks(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALTERNATIVES" => s_ALTERNATIVES(v),
            "AMOUNT" => s_AMOUNT(v),
            "MONEY_VERB_ASK" => s_MONEY_VERB_ASK(v),
            "NOT_A_PRESENT_ASK" => s_NOT_A_PRESENT_ASK(v),
            "OPTIONS" => s_OPTIONS(v),
            "PICK" => s_PICK(v),
            "PRESENT_LOOKING" => s_PRESENT_LOOKING(v),
            "SPEND_IT_ON" => s_SPEND_IT_ON(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.what_it_asks", module_obj);
    pyrt::xrefs::reg_fn("compose.what_it_asks", "_names_alternatives", py_func("_names_alternatives", |args: &[Py]| _names_alternatives(args)));
    pyrt::xrefs::reg_fn("compose.what_it_asks", "_the_line_once", py_func("_the_line_once", |args: &[Py]| _the_line_once(args)));
    pyrt::xrefs::reg_fn("compose.what_it_asks", "the_shape_it_asks", py_func("the_shape_it_asks", |args: &[Py]| the_shape_it_asks(args)));
    pyrt::xrefs::reg_access("compose.what_it_asks", "ALTERNATIVES", g_ALTERNATIVES, s_ALTERNATIVES);
    pyrt::xrefs::reg_access("compose.what_it_asks", "AMOUNT", g_AMOUNT, s_AMOUNT);
    pyrt::xrefs::reg_access("compose.what_it_asks", "MONEY_VERB_ASK", g_MONEY_VERB_ASK, s_MONEY_VERB_ASK);
    pyrt::xrefs::reg_access("compose.what_it_asks", "NOT_A_PRESENT_ASK", g_NOT_A_PRESENT_ASK, s_NOT_A_PRESENT_ASK);
    pyrt::xrefs::reg_access("compose.what_it_asks", "OPTIONS", g_OPTIONS, s_OPTIONS);
    pyrt::xrefs::reg_access("compose.what_it_asks", "PICK", g_PICK, s_PICK);
    pyrt::xrefs::reg_access("compose.what_it_asks", "PRESENT_LOOKING", g_PRESENT_LOOKING, s_PRESENT_LOOKING);
    pyrt::xrefs::reg_access("compose.what_it_asks", "SPEND_IT_ON", g_SPEND_IT_ON, s_SPEND_IT_ON);
}

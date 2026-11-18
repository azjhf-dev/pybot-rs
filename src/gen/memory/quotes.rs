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


pub static NAME_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENS_ON_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_NAME_END() -> Py { ensure_init(); NAME_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_END(v: Py) { ensure_init(); if let Ok(mut w) = NAME_END.write() { *w = v; } }
pub fn g_OPENS_ON_A_FIGURE() -> Py { ensure_init(); OPENS_ON_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENS_ON_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = OPENS_ON_A_FIGURE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The words an answer hands over to say, kept whole when the answer is cut down.\n\nA lot of the knowledge base's best answers end in the words the person can send: a letter to a\nlandlord, a message to a neighbour, the sentence to say at a counter. Those words sit inside\nquotation marks, and they have full stops of their own - a letter is two or three sentences long.\n\nEvery short form of an answer is made by cutting it at a full stop. That is right for a paragraph\nof advice, and wrong for a quoted letter: the cut landed inside the quotes and the person was\ngiven\n\n    Keep it short, dated and specific, and ask for a date: \"Hi — I reported the boiler on [date]\n    and nothing has been done.\n\n- half a letter, with the opening quotation mark never closed. Said after \"how do i ask him\nagain\", that is what the bot printed. Half a quotation is not a shorter version of the answer,\nit is a broken one.\n\nSo the two marks are counted, and a cut that stops with a quote still open is carried on until the\nquote closes. Nothing else about the cut changes: a reply with no quotation in it comes back\nexactly as the caller would have cut it.\n");
    s_NAME_END(stdlib_call("re", "compile", &[py_str("\\b(?:no|nos|vol|vols|ch|pt|pp|fig|e\\.g|i\\.e|etc|vs|dr|mr|mrs|ms|prof|st|al|inc|ltd|co|dept|tel|ref)\\.$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENS_ON_A_FIGURE(stdlib_call("re", "compile", &[py_str("^[\\d*\\u00a3$\\u20ac]")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn left_open(args: &[Py]) -> Py {
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
    let _ = py_str("True when `said` opens a quotation it does not close.\n\n    Each convention is counted on its own: an odd number of straight quotes, or more curly openers\n    than closers. Cheap, and it does not need to know which one an entry writes in.\n    ");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&modulo(&mcall(&said.clone(), "count", &[py_str("\"")]), &Py::Int(2))) {
    return Py::Bool(true);
    }
    return gt(&mcall(&said.clone(), "count", &[py_str("“")]), &mcall(&said.clone(), "count", &[py_str("”")]));
    Py::None
}
pub fn closed(args: &[Py]) -> Py {
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
    let mut rest: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { py_str("") };
    let mut part: Py = Py::None;
    let _ = py_str("`said` carried on through `rest` until the quotation in it closes.\n\n    `said` is the short form the caller cut, `rest` the sentences after it in the same order, and\n    `whole` the answer they came from. A quote that never closes inside the answer cannot be cut\n    short at all, and `whole` is what comes back: it is long, but it is not broken.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&left_open(&[said.clone()])))) {
    return said.clone();
    }
    let __iter1 = py_iter_or_raise(&rest.clone());
    for __item in __iter1 {
    part = __item.clone();
    said = mcall(&add(&add(&said.clone(), &py_str(" ")), &mcall(&call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&left_open(&[said.clone()])))) {
    return said.clone();
    }
    }
    return mcall(&call_builtin("str", &[{ let __b = whole.clone(); if py_truthy(&__b) { __b } else { said.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn a_name(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let _ = py_str("True when the full stop ending `text[start:end]` is part of a name rather than a sentence.");
    before = mcall(&call_builtin("str", &[slice_or_raise(&text.clone(), &start.clone(), &end.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&before.clone(), "endswith", &[py_str(".")])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NAME_END(), "search", &[before.clone()])))) {
    return Py::Bool(false);
    }
    after = mcall(&call_builtin("str", &[slice_or_raise(&text.clone(), &end.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), "lstrip", &[]);
    return call_builtin("bool", &[mcall(&g_OPENS_ON_A_FIGURE(), "match", &[after.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn sentences(args: &[Py]) -> Py {
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
    let mut splitter: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "splitter") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: splitter"))) };
    let mut end: Py = Py::None;
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("`splitter`'s sentences, with a quotation or a name kept in one piece.\n\n    The sentence splitters in this package cut at a full stop and a space, which is right for a\n    paragraph of advice and wrong inside a quoted letter: \"...nothing has been done. Please let\n    me know when an engineer can come\" is one thing to hand over and two sentences to a splitter.\n    Half of it reached the person as a sentence of its own - \"how do i ask him again\" after the\n    landlord answer came back as the first half of the letter, quote open. A part that opens a\n    quotation belongs with the parts after it until the quote closes, so it is only a sentence\n    when the quote in it is finished.\n    ");
    text = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("hasattr", &[splitter.clone(), py_str("finditer")]).unwrap_or_else(|e| exc::raise(e))))) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&splitter.clone(), "split", &[text.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    parts = __unpk1[0].clone();
    start = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&splitter.clone(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    end = __item.clone();
    if py_truthy(&a_name(&[text.clone(), start.clone(), mcall(&end.clone(), "start", &[])])) {
    continue;
    }
    let _ = mcall(&parts.clone(), "append", &[mcall(&slice_or_raise(&text.clone(), &start.clone(), &mcall(&end.clone(), "start", &[]), &Py::None), "strip", &[])]);
    start = mcall(&end.clone(), "end", &[]);
    }
    let _ = mcall(&parts.clone(), "append", &[mcall(&slice_or_raise(&text.clone(), &start.clone(), &Py::None, &Py::None), "strip", &[])]);
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() };
    out = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&parts.clone());
    for __item in __iter3 {
    part = __item.clone();
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { left_open(&[geti(&out.clone(), &neg(&Py::Int(1)))]) } }) {
    let _ = py_setitem(&out.clone(), &neg(&Py::Int(1)), mcall(&add(&add(&geti(&out.clone(), &neg(&Py::Int(1))), &py_str(" ")), &part.clone()), "strip", &[]));
    } else {
    let _ = mcall(&out.clone(), "append", &[part.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.quotes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "NAME_END" => Some(g_NAME_END()),
            "OPENS_ON_A_FIGURE" => Some(g_OPENS_ON_A_FIGURE()),
            "a_name" => Some(py_func("a_name", |args: &[Py]| a_name(args))),
            "closed" => Some(py_func("closed", |args: &[Py]| closed(args))),
            "left_open" => Some(py_func("left_open", |args: &[Py]| left_open(args))),
            "sentences" => Some(py_func("sentences", |args: &[Py]| sentences(args))),
            _ => None,
        }),
        set: None,
    }))
}

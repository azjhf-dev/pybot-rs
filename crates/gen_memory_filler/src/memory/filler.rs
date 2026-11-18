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


thread_local! { static G_ASK_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_THE_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK_TAIL() -> Py { ensure_init(); G_ASK_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_TAIL(v: Py) { ensure_init(); G_ASK_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_THE_WAY() -> Py { ensure_init(); G_IN_THE_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_THE_WAY(v: Py) { ensure_init(); G_IN_THE_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_OFF() -> Py { ensure_init(); G_TAIL_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_OFF(v: Py) { ensure_init(); G_TAIL_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_PHRASES() -> Py { ensure_init(); G_TAIL_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_PHRASES(v: Py) { ensure_init(); G_TAIL_PHRASES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The chat-speak people hang off the end of a line, and the line without it.\n\nIt is not part of what they told you: the fact is \"my sister is called meg\", and \"you said your\nsister is called meg to be honest\" says the fact back with a shrug on it. The reply that notes a\nline, the summary that quotes it, and the plan the store keeps all read this, so the words that\nare only a shrug are dropped in one place rather than three.\n");
    s_TAIL_OFF(call_builtin("frozenset", &[mcall(&py_str("tho though lol lmao haha ha hehe tbh ngl imo idk innit x xx xxx\n    actually btw"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_PHRASES(py_tuple(vec![py_str("by the way"), py_str("out of interest"), py_str("while im at it"), py_str("whilst im at it"), py_str("as it happens"), py_str("if thats ok"), py_str("if that's ok"), py_str("if you dont mind"), py_str("if you don't mind"), py_str("come to think of it"), py_str("now i think of it"), py_str("now that i think of it")]));
    s_IN_THE_WAY(call_builtin("frozenset", &[mcall(&py_str("actually really honestly literally genuinely basically"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("by the way"), py_str("btw"), py_str("out of interest"), py_str("while im at it"), py_str("whilst im at it"), py_str("as it happens"), py_str("if thats ok"), py_str("if that's ok"), py_str("if you dont mind"), py_str("if you don't mind"), py_str("come to think of it"), py_str("now i think of it"), py_str("now that i think of it"), py_str("actually")]) }
    s_ASK_TAIL(__data1());
}
pub fn without_filler(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`said` without the chat-speak hanging off the end: \"im veggie tho\" -> \"im veggie\".");
    words = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAIL_OFF(), &mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(".,;:!?")]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { text.clone() } }) {
    changed = Py::Bool(false);
    stripped = mcall(&text.clone(), "rstrip", &[py_str(" .,;:!?")]);
    low = mcall(&stripped.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_TAIL_PHRASES());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&{ let __b = mcall(&low.clone(), "endswith", &[op_add(&py_str(" "), &phrase.clone())]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&low.clone(), &phrase.clone())) } }) {
    cut = mcall(&slice_or_raise(&stripped.clone(), &Py::None, &op_sub(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "rstrip", &[py_str(" ,;:-")]);
    if py_truthy(&cut.clone()) {
    text = mcall(&py_str(" "), "join", &[mcall(&cut.clone(), "split", &[])]);
    changed = Py::Bool(true);
    }
    break;
    }
    }
    }
    return text.clone();
    Py::None
}
pub fn without_the_end_shrug(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("`said` without the shrug hung on the end of an ask: \"how old is my son by the way\".");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { text.clone() } }) {
    changed = Py::Bool(false);
    stripped = mcall(&text.clone(), "rstrip", &[py_str(" .,;:!?")]);
    low = mcall(&stripped.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ASK_TAIL()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&{ let __b = mcall(&low.clone(), "endswith", &[op_add(&py_str(" "), &phrase.clone())]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&low.clone(), &phrase.clone())) } }) {
    cut = mcall(&slice_or_raise(&stripped.clone(), &Py::None, &op_sub(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "rstrip", &[py_str(" ,;:-")]);
    if py_truthy(&cut.clone()) {
    text = mcall(&py_str(" "), "join", &[mcall(&cut.clone(), "split", &[])]);
    changed = Py::Bool(true);
    }
    break;
    }
    }
    }
    return text.clone();
    Py::None
}
pub fn without_the_shrug(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`said` without the fillers in the middle of it: \"what can i actually do\" -> \"what can i do\".\n\n    One word is always left, so a line that was nothing but a shrug comes back as it was.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_IN_THE_WAY(), &mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&kept.clone()) { mcall(&py_str(" "), "join", &[kept.clone()]) } else { mcall(&py_str(" "), "join", &[words.clone()]) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.filler".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_TAIL" => Some(g_ASK_TAIL()),
            "IN_THE_WAY" => Some(g_IN_THE_WAY()),
            "TAIL_OFF" => Some(g_TAIL_OFF()),
            "TAIL_PHRASES" => Some(g_TAIL_PHRASES()),
            "without_filler" => Some(py_func("without_filler", |args: &[Py]| without_filler(args))),
            "without_the_end_shrug" => Some(py_func("without_the_end_shrug", |args: &[Py]| without_the_end_shrug(args))),
            "without_the_shrug" => Some(py_func("without_the_shrug", |args: &[Py]| without_the_shrug(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK_TAIL" => s_ASK_TAIL(v),
            "IN_THE_WAY" => s_IN_THE_WAY(v),
            "TAIL_OFF" => s_TAIL_OFF(v),
            "TAIL_PHRASES" => s_TAIL_PHRASES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.filler", module_obj);
    pyrt::xrefs::reg_fn("memory.filler", "without_filler", py_func("without_filler", |args: &[Py]| without_filler(args)));
    pyrt::xrefs::reg_fn("memory.filler", "without_the_end_shrug", py_func("without_the_end_shrug", |args: &[Py]| without_the_end_shrug(args)));
    pyrt::xrefs::reg_fn("memory.filler", "without_the_shrug", py_func("without_the_shrug", |args: &[Py]| without_the_shrug(args)));
    pyrt::xrefs::reg_access("memory.filler", "ASK_TAIL", g_ASK_TAIL, s_ASK_TAIL);
    pyrt::xrefs::reg_access("memory.filler", "IN_THE_WAY", g_IN_THE_WAY, s_IN_THE_WAY);
    pyrt::xrefs::reg_access("memory.filler", "TAIL_OFF", g_TAIL_OFF, s_TAIL_OFF);
    pyrt::xrefs::reg_access("memory.filler", "TAIL_PHRASES", g_TAIL_PHRASES, s_TAIL_PHRASES);
}

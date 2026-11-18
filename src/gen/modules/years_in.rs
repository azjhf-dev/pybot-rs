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


pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKES_AS_IT_COMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); TAKES_AS_IT_COMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); if let Ok(mut w) = TAKES_AS_IT_COMES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("How long they have been at the job they filed, said back as something worth a word.\n\n    im a teacher\n    i have been doing it for 10 years\n\nThe first turn is answered well. The second was answered by the greeting fallback with \"Got it -\nyou have been doing it for 10 years\" - the line said back with a receipt on it. It knows nothing\nabout the job filed a turn ago and nothing about what ten years means, and it is the whole reply:\nno warmth, nothing to say next. Telling someone how long you have stuck at something is a bid to\ntalk about it, not a fact being filed.\n\n`bot/memory/years_in.py` holds the reading and the wording. This module is the turn it is asked\non: a band above the chat module, so the length is answered as the follow-up it is, and the store's\nown readers keep every other line. `respond` returns None on a line it is not about, so a chat\nwith no job in it, or a line with a feeling or a question in it, is answered as it was before.\n");
    s_PRIORITY(Py::Int(1150));
    s_TAKES_AS_IT_COMES(g_takes_as_it_comes());
}
pub fn respond(args: &[Py]) -> Py {
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
    let _ = py_str("The reply for a line that is only how long they have been at their job, else None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return { let __b = crate::gen::memory::years_in::the_years_in_the_doing(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::memory::years_in::the_years_in_their_doing(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } } } };
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
    let _ = py_str("This line is a length I read whole, so it is answered as it comes.\n\n    Without the hook the specialist rule hands the turn to the knowledge base: \"ive been teaching\n    for 20 years\" is read as a school line by the word \"teaching\", the cooking entry declares the\n    area too, and an oven-dish card answered it. The claim says what is true - this line is the\n    length of the job the chat already filed, and nothing else in it is a subject to look up.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = crate::gen::memory::years_in::the_years_in_the_doing(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::years_in::the_years_in_their_doing(&[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.years_in".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}

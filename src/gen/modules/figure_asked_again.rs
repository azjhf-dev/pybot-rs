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
    let _ = py_str("A yes/no about a figure the bot itself gave: \"so is eight weeks still right\".\n\nThe figure is the answer's - the chat already said eight weeks - and the question is whether it\nstill stands. Nothing in the line names a thing of its own, so the knowledge base read the words\nit could see and answered from an entry about something else: \"so is eight weeks still right\"\nafter a 5k plan got \"Ask in writing and name what you want: a refund, a repair, or money for the\ntrouble. Once eight weeks pass...\" - a consumer-complaint entry, reached by the figure's words.\n\nThe reading and the wording live in `bot/memory/stands.py`: the answer is the figure with the\nreason the chat already gave for it, because that is the sentence the bot used and a friend who is\nasked \"is that still right\" says where the number came from rather than looking it up again.\n\nThis module sits above the knowledge base (0), because the entry that answered these lines was the\nknowledge base reading a figure as a subject, and below the safety band and the loss and grief\nmodules, which keep their turns: \"is three sessions a week still right\" during a bereavement is\nnot a question about the figure. It returns None on every line that is not a yes/no about a figure\nof this chat's own answers.\n");
    s_PRIORITY(Py::Int(1000));
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
    let _ = py_str("The reply to a yes/no about a figure the bot gave, or None for every other line.\n\n    Two shapes: a figure in the line (\"so is eight weeks still right\"), and a change with no figure\n    in it (\"does that change the time?\", after \"actually its a veggie one, no mince\"). Both are the\n    chat's own answer handed back, and both are here because the answer is the same figure.\n    ");
    return { let __b = crate::gen::memory::stands::reply_for(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::stands::change_reply(&[text.clone(), memory.clone()]) } };
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
    let _ = py_str("Only on a yes/no about a figure of this chat's own answers, and nowhere else.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = crate::gen::memory::stands::the_figure_asked_about(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::stands::the_change_asked_about(&[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.figure_asked_again".to_string(),
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

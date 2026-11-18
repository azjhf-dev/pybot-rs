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
fn module_body() {
    let _ = py_str("Somebody else's habit, said out loud, gets the sentence to raise it with.\n\n\"My brother keeps asking me for money\", \"my son keeps leaving his shoes in the hall\", \"my mate\nnever invites me to anything\", \"the kids keep leaving their toys everywhere\": each of these\nreached a line that fits any message - \"Got it. Is there anything you'd like a hand with?\", \"I'm\nlistening - say more if it helps.\", \"Noted: your brother keeps asking for money\". The composer\nhas a pool for the same shape (GRIEVANCE), but it keys on the verb, so the two commonest ways\npeople say it - an imposition (\"asks me for money\", \"uses my toothbrush\", \"leaves her dishes in\nthe sink\") and a neglect (\"never invites me\", \"never thanks me\", \"never notices\") - are not in\nits list.\n\nThe one help a fallback can honestly give here is the sentence to raise it with: the bot cannot\nsee the brother or the hall, and a question back (\"what would you like to do about it\") leaves\nthe person holding the whole of it. bot/compose/habit.py reads the shape and holds the words.\n\nThree kinds of placeholder are replaced and nothing else - the pools that mean \"I cannot answer\"\n(`fallback._is_a_hedge`), the neutral lines that fit any message, and a receipt (the person's own\nsentence handed back with nothing added). A line any entry or module answered never gets here:\nthis sits at PRIORITY -999, above the greeting band and below every module that knows the\nsubject, so a knowledge card, the small talk, or a safety line keeps its turn.\n");
    s_PRIORITY(neg(&Py::Int(999)));
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
    let mut raised: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The sentence to raise it with, or None when this is not that shape.");
    raised = crate::gen::compose::habit::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&raised.clone()))) {
    return Py::None;
    }
    said = py_call(&crate::gen::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_is_a_hedge(&[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_in_a_pool(&[said.clone(), py_tuple(vec![crate::gen::compose::fallback::g_NEUTRAL()])]); if py_truthy(&__b) { __b } else { crate::gen::memory::ack::reads_as_a_receipt(&[said.clone()]) } } } }))) {
    return Py::None;
    }
    return raised.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_habit_is_raised".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

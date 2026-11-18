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


pub static LONGEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_ON_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _gives_before_this_module: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_LONGEST() -> Py { ensure_init(); LONGEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LONGEST(v: Py) { ensure_init(); if let Ok(mut w) = LONGEST.write() { *w = v; } }
pub fn g_NOT_ON_RECORD() -> Py { ensure_init(); NOT_ON_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_ON_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_ON_RECORD.write() { *w = v; } }
pub fn g_ON_RECORD() -> Py { ensure_init(); ON_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = ON_RECORD.write() { *w = v; } }
pub fn g__gives_before_this_module() -> Py { ensure_init(); _gives_before_this_module.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__gives_before_this_module(v: Py) { ensure_init(); if let Ok(mut w) = _gives_before_this_module.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A question about the person's own record is answered by the store, even when it holds nothing.\n\n\"Where did i park?\" -> \"You haven't told me where you parked.\" Nobody else can answer a question\nabout the person's own life: the store is the only part of the bot that could, and the store\nsaying it holds nothing is the whole of the answer there is. `bot/compose/fit.py` reads those\nwords as a shrug - `PUNT` opens \"you haven't told me ...\" - so the reply that is exactly right for\nthe line was counted a miss for every shape, and the answer the person needed (that the bot has\nnothing on record, and would like to be told) lost its credit to nothing.\n\nThe fence is the one the store's own sentence gives us:\n\n  * only the fact shapes a question about their own record can ask for - a pick, a method, a\n    reason or a few ideas is the bot's own judgement, and \"You haven't told me what you like\" is\n    a question back rather than the pick that was asked for;\n  * the sentence has to be the opening one, so a paragraph that carries the words somewhere past\n    the middle is not credited;\n  * no \"?\" anywhere in the reply: a question back is not an answer, whatever it says before it;\n  * a short reply, so the store's own one-liner is what is being read.\n\nIt is a module rather than a line in `bot/compose/fit.py` only because the foot of that file is\nwhere every arm of the swarm lands at once and a change there is refused while main moves. It\nwraps `fit.gives` once, at import, and saves the function it wrapped; `bot/modules/arbiter.py`\nand `bot/compose/lead.py` both call `fit.gives` by attribute, so both see this check. Every\nreading it does not name is the one `fit.gives` gives.\n");
    s_ON_RECORD(call_builtin("frozenset", &[py_tuple(vec![crate::gen::compose::fit::g_TIME(), crate::gen::compose::fit::g_AMOUNT(), crate::gen::compose::fit::g_PLACE(), crate::gen::compose::fit::g_PERSON(), crate::gen::compose::fit::g_MEANING(), crate::gen::compose::fit::g_YESNO()])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ON_RECORD(stdlib_call("re", "compile", &[py_str("\\b(?:you|ye) (?:have not|haven'?t) (?:said|told|mentioned|let me know)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LONGEST(Py::Int(180));
    s__gives_before_this_module(py_func("gives", |args: &[Py]| crate::gen::compose::fit::gives(args)));
    let _ = py_setattr(&crate::gen::compose::fit::module_obj(), "gives", g_gives());
}
pub fn gives(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut got: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("`fit.gives`, with the store's own \"i hold nothing about that\" counted as the answer.");
    got = py_call(&g__gives_before_this_module(), &[reply.clone(), want.clone(), line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&got.clone(), &Py::Bool(true))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ON_RECORD(), &want.clone())) } }) {
    return got.clone();
    }
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONGEST()) } }) {
    return got.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NOT_ON_RECORD(), "search", &[crate::gen::compose::fit::first_sentence(&[text.clone()])])))) {
    return got.clone();
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.on_record".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "LONGEST" => Some(g_LONGEST()),
            "NOT_ON_RECORD" => Some(g_NOT_ON_RECORD()),
            "ON_RECORD" => Some(g_ON_RECORD()),
            "_gives_before_this_module" => Some(g__gives_before_this_module()),
            "gives" => Some(py_func("gives", |args: &[Py]| gives(args))),
            _ => None,
        }),
        set: None,
    }))
}

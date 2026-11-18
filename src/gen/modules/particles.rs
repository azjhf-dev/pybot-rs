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


pub static A_PARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VERB_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_PARTICLE() -> Py { ensure_init(); A_PARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = A_PARTICLE.write() { *w = v; } }
pub fn g_A_VERB_TAIL() -> Py { ensure_init(); A_VERB_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VERB_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_VERB_TAIL.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The tail of a phrasal verb belongs to the verb, not to the thing the person named.\n\n\"i need my dress taken up\" names a dress, and what they want done to it. The bot read the last\nword as the thing and asked \"What kind of up do you usually go for?\"; \"i need my car washed\" got\n\"What kind of washed do you usually go for?\". Neither is a sentence a person can answer, because\nthe thing being talked about is the dress and the car, and the rest of the line says what is to be\ndone to it.\n\nSo the words that make up a \"something done\" tail - the verb of it and its particle - are kept\nhere, and `bot/modules/chat.py` asks this module before it decides a thing comes in kinds. The\nlist is deliberately only the verbs people use for work somebody else does on something of theirs,\nso an ordinary \"i need my car\" or \"i want a clean shirt\" is untouched.\n");
    s_A_PARTICLE(call_builtin("frozenset", &[mcall(&py_str("up down in out off over away along through round back"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERB_TAIL(call_builtin("frozenset", &[mcall(&py_str("\ntaken took taking put putting let letting thrown threw given gave done doing made making\nfixed fixing mended mending altered altering hemmed hemming patched darned\nrepaired repairing cleaned cleaning washed washing cut cutting painted painting\ncleared clearing tidied tidying mowed mowing serviced servicing trimmed pruning\nswept hoovered vacuumed dusted walked boarded shortened lengthened\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_thing_without_its_tail(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The thing said back without the job worded after it: \"your dress taken up\" -> \"your dress\".\n\n    Only whole words come off, and never the last one: a line that is all tail keeps what it has,\n    because an empty thing is worse than an awkward one.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    while py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&g_A_PARTICLE(), &g_A_VERB_TAIL()), &mcall(&mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(".,!?")]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn a_job_word(args: &[Py]) -> Py {
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
    let _ = py_str("True when this word is the verb or particle of a job, not a thing to pick a kind of.");
    return Py::Bool(py_contains(&bitor(&g_A_VERB_TAIL(), &g_A_PARTICLE()), &mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str(" .,!?;:'\"()")])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.particles".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_PARTICLE" => Some(g_A_PARTICLE()),
            "A_VERB_TAIL" => Some(g_A_VERB_TAIL()),
            "a_job_word" => Some(py_func("a_job_word", |args: &[Py]| a_job_word(args))),
            "a_thing_without_its_tail" => Some(py_func("a_thing_without_its_tail", |args: &[Py]| a_thing_without_its_tail(args))),
            _ => None,
        }),
        set: None,
    }))
}

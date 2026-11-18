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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing of theirs in a state that wants doing is asked about, not receipted.\n\n\"the grass is long\", \"the car is dirty\", \"the windows are filthy\", \"my shoes are worn out\" and\n\"my coat is tatty\" were all answered with the composer's receipt - \"Thanks for telling me - I'm\nhere if you want to dig into any of that\" - which says nothing back about the thing the person has\njust handed over and asks nothing about it. Dumping the candidates for two of them shows nothing\nelse proposes anything at all: there is no card for either, though `when should i cut it` two turns\nlater gets the mowing card. So this is the floor, and the floor was a shrug in better words.\n\n`bot/compose/a_thing_wants_doing.py` reads the shape - `my/our/the <thing> is <state>`, with a\nstate that wants doing - and stands down for a body part, for a person, for a thing gone wrong or\nsimply old, and for anything the line says the person feels.\n\nThis module sits just above `greeting` (-1000) and below every card, so it only ever speaks where\nthe bot would otherwise have receipted the line. It asks `greeting.respond` first - the composer\nand every reader in it - and stands down unless the answer that came back really is that receipt,\nso a card, an entry, a reader of the composer's own and the day pools all keep their turn even\nthough this module outranks greeting. It writes nothing about the world, so nothing here can be\nwrong or go stale, and like the reading it is asked only on a line it recognises.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
}
pub fn _the_thing_and_its_state(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut understood: Py = Py::None;
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = { let __b = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { Py::None }; if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]) } };
    return py_tuple(vec![clean.clone(), gen_compose::compose::a_thing_wants_doing::pool(&[clean.clone(), memory.clone()])]);
    Py::None
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
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A question about the thing, or None where the bot already said something about it.");
    let __unpk1 = py_unpack(&_the_thing_and_its_state(&[text.clone(), memory.clone()]), 2, None);
    clean = __unpk1[0].clone();
    pool = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    said = { let __b = crate::modules::greeting::respond(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_call(&gen_compose::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&gen_compose::compose::fallback::_in_a_pool(&[said.clone(), py_tuple(vec![gen_compose::compose::fallback::g_NEUTRAL()])])))) {
    return Py::None;
    }
    return py_call(&gen_compose::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_thing_wants_doing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "_the_thing_and_its_state" => Some(py_func("_the_thing_and_its_state", |args: &[Py]| _the_thing_and_its_state(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_thing_wants_doing", module_obj);
    pyrt::xrefs::reg_fn("modules.a_thing_wants_doing", "_the_thing_and_its_state", py_func("_the_thing_and_its_state", |args: &[Py]| _the_thing_and_its_state(args)));
    pyrt::xrefs::reg_fn("modules.a_thing_wants_doing", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_thing_wants_doing", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

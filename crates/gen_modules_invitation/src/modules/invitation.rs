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


thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An invitation to the bot, answered as a friend who cannot come answers it.\n\n\"do you fancy a walk\" is a person asking for company. Every module that could have answered it\nread it as a question about the world: the composer's last resort sent it to the search hedge\n(\"that's beyond what I know well, sorry\") and the other wording of it to the intake form (\"Got\nit. Is there anything you'd like a hand with?\"). This module proposes the reply that fits - the\nrefusal said plainly, then the next question about the thing itself (bot/compose/invitation.py).\n\nThe PRIORITY is below the knowledge base on purpose. An invitation that names something the\nanswers know is still that thing's turn: \"shall we get a takeaway\" keeps the fish and chips and\n\"you up for a curry on friday\" keeps the quick curry, because those replies are better than\nanything about the invitation would be. What is left here is the invitation nothing else answers.\n\nThat rule is declared, not implied. A module that proposes from the fallback band has to say\nwhat it answers and when it stands aside (the routing contract, `tests/test_routing_contract.py`),\notherwise the band is a place to hide a catch-all in: `ANSWERS_INSTEAD` says the invitation\nreading takes the line in place of the small-talk entries, and `DEFERS_TO` says the standing\naside - every answer at a real priority keeps its turn, which is the knowledge base's turn on the\nline that names a thing. So this module is the floor of one shape of line, not a competitor for\nany topic.\n");
    s_PRIORITY(op_neg(&Py::Int(500)));
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("small-talk")]));
}
pub fn DEFERS_TO(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("Yes for every answer at a real priority: the invitation is the floor of its shape.\n\n    The arbiter asks this of the candidates that are not in the fallback band (see\n    `arbiter._defers`), so an answer that a lane stands behind keeps the turn and the refusal is\n    left for the line nobody else answered: \"shall we get a takeaway\" keeps the fish and chips.\n    The wording is the composer's and the bands are the arbiter's; this only says out loud what\n    the PRIORITY already meant, so that a module in the fallback band cannot quietly become the\n    answer to everything.\n    ");
    return Py::Bool(true);
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
    let _ = py_str("The line an invitation to the bot gets; None for every other line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    memory = py_dict(vec![]);
    }
    return pyrt::xrefs::xcall("compose.invitation", "reply_for", &[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.invitation".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "DEFERS_TO" => Some(py_func("DEFERS_TO", |args: &[Py]| DEFERS_TO(args))),
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.invitation", module_obj);
    pyrt::xrefs::reg_fn("modules.invitation", "DEFERS_TO", py_func("DEFERS_TO", |args: &[Py]| DEFERS_TO(args)));
    pyrt::xrefs::reg_fn("modules.invitation", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.invitation", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.invitation", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

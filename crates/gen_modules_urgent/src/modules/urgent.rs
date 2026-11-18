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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.urgent: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Someone they look after in trouble wins the turn over a pleasantry, and only that.\n\n`bot/compose/urgent.py` holds the rules and the wording; this module is the proposal, so the\narbiter has something better than the fallback's nod to choose.\n\nIt answers in two places, and no others:\n\n- the lines nothing claims, where the fallback would have nodded (a body in trouble is not small\n  talk, so it sits above the greeting);\n- **in place of the health topic entries**, because a swallowed thing is a poison line and the\n  entries get it wrong: `my daughter ate a washing tablet` is screen-time advice on the entries and\n  a 999 call here. `ANSWERS_INSTEAD` scopes that to one topic, and the reading is strict - a\n  relative and an ingest verb with a chemical, medicine or small object - so nothing else moves.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.urgent.PRIORITY"); }
    s_PRIORITY(op_neg(&Py::Int(900)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.urgent.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.urgent.ANSWERS_INSTEAD"); }
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("health")]));
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
    return pyrt::xrefs::xcall("compose.urgent", "reply_for", &[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.urgent".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "urgent" => Some(pyrt::xrefs::xmodule("compose.urgent")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.urgent", module_obj);
    pyrt::xrefs::reg_fn("modules.urgent", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.urgent", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.urgent", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.urgent", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

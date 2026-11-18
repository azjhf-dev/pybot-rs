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
    let _ = py_str("The detail a person drops into a thread, taken up rather than nodded at.\n\n\"she doesnt like surprises\" while a 70th is being planned, \"it was fine for two years\" about a\nboiler, \"ive said no once already\" about a boss: the turn after each of those is where the bot\nsays nothing - \"I'm listening - say more if it helps\", the line said back, or a search - and the\nturn after that, asked as a question, gets a real card. The thread was there all along.\n\n`bot/compose/thread_detail.py` does the reading: three kinds of detail (a limit somebody has, an\nattempt already made, a history), and the one question each of them raises. This module sits in\nthe band `where_it_answers` and `a_day_somebody_else_keeps` use - above greeting's hedge, below\nevery card - and stands down unless the composer's own reply says nothing about the line, so any\nreader that found its own answer keeps the turn.\n\nNothing here holds a fact, so nothing here can be wrong or go stale.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
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
    let _ = py_str("Ask the one question the detail raises, or say nothing and let the composer answer.");
    clean = pyrt::xrefs::xcall("compose.fallback", "_matchable", &[pyrt::xrefs::xcall("compose.fallback", "_plain", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }])]);
    pool = pyrt::xrefs::xcall("compose.thread_detail", "detail_pool", &[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    said = py_call(&pyrt::xrefs::xget("compose.fallback", "reply_for"), &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.thread_detail", "says_nothing_about_the_line", &[said.clone(), pyrt::xrefs::xcall("compose.fallback", "_matchable", &[text.clone()])])))) {
    return Py::None;
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), pyrt::xrefs::xcall("compose.fallback", "_matchable", &[text.clone()])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.the_detail_the_thread_adds".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
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
    pyrt::xrefs::reg_module("modules.the_detail_the_thread_adds", module_obj);
    pyrt::xrefs::reg_fn("modules.the_detail_the_thread_adds", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.the_detail_the_thread_adds", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

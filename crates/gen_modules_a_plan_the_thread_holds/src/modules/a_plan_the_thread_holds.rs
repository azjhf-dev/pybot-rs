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
thread_local! { static G_SEARCH_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_SEARCH_ONLY() -> Py { ensure_init(); G_SEARCH_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEARCH_ONLY(v: Py) { ensure_init(); G_SEARCH_ONLY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The plan of a day, when the thread already holds every part of it.\n\nA day out is said over several turns - a gig on Saturday, three of us, eating first, staying\nover, cheap and cheerful - and then somebody asks the obvious question: \"so whats the plan for\nthe day then\". No card answers that, because nobody can look up somebody else's day. The order of\na day out is not knowledge either: the hour that cannot move, the meal, the bed. Every constraint\non it was said by the person, so the reply lays the order out, says their own words back, and\nnames the one hour the bot cannot know instead of guessing it.\n\nThis module sits just above greeting's honest hedge and below every light remark and every card,\nso it only ever speaks where the bot would otherwise have told the person to go and search. It\nasks the composer first and stands down unless the answer really is a hedge, so any reader in\n`bot/compose/fallback.py` that has found its own body (a venue's page, a booking, a timetable)\nkeeps its turn even though this module outranks greeting.\n\n`bot/compose/plan_ask.py` does the reading. Nothing here holds a fact about the world: the order\nis cut to the day the thread describes, a train home the same evening is not told to book a room,\na booking that is already made is not made again, and a meal cooked at home is not a day out.\n");
    s_SEARCH_ONLY(py_func("_is_a_hedge", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_is_a_hedge", args)));
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
    let _ = py_str("Lay out the plan the thread holds, or say nothing and let the composer answer as it did.");
    clean = pyrt::xrefs::xcall("compose.fallback", "_matchable", &[pyrt::xrefs::xcall("compose.fallback", "_plain", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }])]);
    pool = pyrt::xrefs::xcall("compose.plan_ask", "plan_ask_pool", &[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    return Py::None;
    }
    said = py_call(&pyrt::xrefs::xget("compose.fallback", "reply_for"), &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&g_SEARCH_ONLY(), &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), pyrt::xrefs::xcall("compose.fallback", "_matchable", &[text.clone()])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_plan_the_thread_holds".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "SEARCH_ONLY" => Some(g_SEARCH_ONLY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "SEARCH_ONLY" => s_SEARCH_ONLY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_plan_the_thread_holds", module_obj);
    pyrt::xrefs::reg_fn("modules.a_plan_the_thread_holds", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_plan_the_thread_holds", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_plan_the_thread_holds", "SEARCH_ONLY", g_SEARCH_ONLY, s_SEARCH_ONLY);
}

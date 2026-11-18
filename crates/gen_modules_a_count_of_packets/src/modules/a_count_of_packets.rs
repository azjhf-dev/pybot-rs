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
thread_local! { static G_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_is_a_dose: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_REPLIES() -> Py { ensure_init(); G_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPLIES(v: Py) { ensure_init(); G_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_is_a_dose() -> Py { ensure_init(); G_is_a_dose.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_is_a_dose(v: Py) { ensure_init(); G_is_a_dose.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A count of packets is the poisons card, not a nod and not the tablet card.\n\nThe reading is in `bot/compose/a_count_of_packets.py`; this module is here so the shape has a band\nof its own. It asks for nothing safety already answers: `is_a_dose` reads only a count of\ncontainers, which no arm of the poisons case in `safety.py` reads, and the card it hands back is\nsafety's own pool and safety's own note, so a follow-up (\"is he going to be alright\") lands in the\nsame conversation.\n\nThe band is the safety one (9000), because this is an emergency card and nothing below that band\nsurvives the arbiter's passing-over rules - at 8900 the tablet-taking card still won the turn. A\ncard safety already has is never displaced: `safety.respond` is asked first, and a line it answers\nis left to it.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_is_a_dose(py_func("is_a_dose", |args: &[Py]| pyrt::xrefs::xcall("compose.a_count_of_packets", "is_a_dose", args)));
    s_REPLIES(pyrt::xrefs::xget("knowledge.crisis_replies", "REPLIES"));
    s_PRIORITY(Py::Int(9000));
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
    let _ = py_str("The poisons card for a count of packets, or None for every other line.");
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.a_count_of_packets", "is_a_dose", &[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("modules.safety", "respond", &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("modules.urgent", "respond", &[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&memory.clone(), &py_str("safety_case"), py_str("poison"));
    let _ = py_setitem(&memory.clone(), &py_str("safety_safe"), Py::Bool(false));
    }
    return pyrt::xrefs::xcall("modules.safety", "_fit", &[pyrt::xrefs::xcall("compose", "pick", &[geti(&pyrt::xrefs::xget("knowledge.crisis_replies", "REPLIES"), &py_str("poison")), memory.clone()]), text.clone(), py_str("poison")]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_count_of_packets".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "REPLIES" => Some(g_REPLIES()),
            "is_a_dose" => Some(g_is_a_dose()),
            "pick" => Some(g_pick()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "REPLIES" => s_REPLIES(v),
            "is_a_dose" => s_is_a_dose(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_count_of_packets", module_obj);
    pyrt::xrefs::reg_fn("modules.a_count_of_packets", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_count_of_packets", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_count_of_packets", "REPLIES", g_REPLIES, s_REPLIES);
    pyrt::xrefs::reg_access("modules.a_count_of_packets", "is_a_dose", g_is_a_dose, s_is_a_dose);
    pyrt::xrefs::reg_access("modules.a_count_of_packets", "pick", g_pick, s_pick);
}

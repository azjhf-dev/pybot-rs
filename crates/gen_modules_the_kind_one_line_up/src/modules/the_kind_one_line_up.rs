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


thread_local! { static G_FALLBACK_MODULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.the_kind_one_line_up: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_FALLBACK_MODULES() -> Py { ensure_init(); G_FALLBACK_MODULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FALLBACK_MODULES(v: Py) { ensure_init(); G_FALLBACK_MODULES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that names nothing of its own takes the kind of answer the line above named.\n\n    my order hasnt come        \"Book a bulky waste collection with your council - most charge\n                                twenty to forty pounds and collect from the doorstep.\"\n    and how long               the hedge (\"I'm not sure about that one, sorry - I'd rather not\n                                guess. A quick search online will get you a solid answer\")\n\n    im on benefits             the Universal Credit card\n    can i save anything        the hedge again\n\nThe second line is the same *kind* of question as the first line of each pair - how long it takes,\nwhat you are allowed while you claim - and the answer is not in a card, because the card above\nanswered a question about a different subject. A friend keeps the kind and says what that kind of\nthing turns on, or names the one place it is written down, in the same breath.\n\nThe reading and the wording live in `bot/compose/kept_nearby.py`. It is deliberately narrow: the\nline must lean on what is above it (`and how long`, `what about the cost`), must name no thing of\nits own, and the person's own earlier line must be one the table has a kind for. A follow-up that\nbrings a subject decides its own turn, so it is not this reading's (white-peach's three guards on\n#12543: a measure word is a kind of ask, a `how do i ...` is a method and not the kind above it,\nand two lines back is only in thread when the line between them leaned on it too).\n\nThe band is why this is a module and not a branch in the greeting's hook: **-999**, above the\ngreeting fallback (-1000) and under every module that has a real answer for the line. It reads the\nbot's own last reply to stay in that seat: the turn it replaces is a hedge or a receipt, so the\nmodule stands down when the last reply came from anything but the fallback (`greeting`, `chat`).\nThat is the same reading the hedge gate made in `bot/modules/greeting.py`, done where a module can\ndeclare its own priority instead of editing the file every other reader hooks into.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
    s_FALLBACK_MODULES(call_builtin("frozenset", &[py_tuple(vec![py_str("greeting"), py_str("chat")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _answered_by_the_fallback(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut turns: Py = Py::None;
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_FALLBACK_MODULES(), &call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))));
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
    let mut nearby: Py = Py::None;
    let _ = py_str("The kind of answer the earlier line named, or None on every other line.");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&_answered_by_the_fallback(&[memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    nearby = pyrt::xrefs::xcall("compose.kept_nearby", "answer_for", &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&nearby.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    return py_tuple(vec![Py::Bool(true), py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[nearby.clone(), memory.clone(), text.clone()])]);
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
    return Py::None;
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
    Py::Module(Rc::new(PyModule { name: "bot.modules.the_kind_one_line_up".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "FALLBACK_MODULES" => Some(g_FALLBACK_MODULES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_answered_by_the_fallback" => Some(py_func("_answered_by_the_fallback", |args: &[Py]| _answered_by_the_fallback(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "kept_nearby" => Some(pyrt::xrefs::xmodule("compose.kept_nearby")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "FALLBACK_MODULES" => s_FALLBACK_MODULES(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.the_kind_one_line_up", module_obj);
    pyrt::xrefs::reg_fn("modules.the_kind_one_line_up", "_answered_by_the_fallback", py_func("_answered_by_the_fallback", |args: &[Py]| _answered_by_the_fallback(args)));
    pyrt::xrefs::reg_fn("modules.the_kind_one_line_up", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.the_kind_one_line_up", "FALLBACK_MODULES", g_FALLBACK_MODULES, s_FALLBACK_MODULES);
    pyrt::xrefs::reg_access("modules.the_kind_one_line_up", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

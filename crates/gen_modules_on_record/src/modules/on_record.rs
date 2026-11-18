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


thread_local! { static G_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ON_RECORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_RECORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__gives_before_this_module: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.on_record: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_LONGEST() -> Py { ensure_init(); G_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONGEST(v: Py) { ensure_init(); G_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ON_RECORD() -> Py { ensure_init(); G_NOT_ON_RECORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ON_RECORD(v: Py) { ensure_init(); G_NOT_ON_RECORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_RECORD() -> Py { ensure_init(); G_ON_RECORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_RECORD(v: Py) { ensure_init(); G_ON_RECORD.with(|c| *c.borrow_mut() = v); }
pub fn g__gives_before_this_module() -> Py { ensure_init(); G__gives_before_this_module.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__gives_before_this_module(v: Py) { ensure_init(); G__gives_before_this_module.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about the person's own record is answered by the store, even when it holds nothing.\n\n\"Where did i park?\" -> \"You haven't told me where you parked.\" Nobody else can answer a question\nabout the person's own life: the store is the only part of the bot that could, and the store\nsaying it holds nothing is the whole of the answer there is. `bot/compose/fit.py` reads those\nwords as a shrug - `PUNT` opens \"you haven't told me ...\" - so the reply that is exactly right for\nthe line was counted a miss for every shape, and the answer the person needed (that the bot has\nnothing on record, and would like to be told) lost its credit to nothing.\n\nThe fence is the one the store's own sentence gives us:\n\n  * only the fact shapes a question about their own record can ask for - a pick, a method, a\n    reason or a few ideas is the bot's own judgement, and \"You haven't told me what you like\" is\n    a question back rather than the pick that was asked for;\n  * the sentence has to be the opening one, so a paragraph that carries the words somewhere past\n    the middle is not credited;\n  * no \"?\" anywhere in the reply: a question back is not an answer, whatever it says before it;\n  * a short reply, so the store's own one-liner is what is being read.\n\nIt is a module rather than a line in `bot/compose/fit.py` only because the foot of that file is\nwhere every arm of the swarm lands at once and a change there is refused while main moves. It\nwraps `fit.gives` once, at import, and saves the function it wrapped; `bot/modules/arbiter.py`\nand `bot/compose/lead.py` both call `fit.gives` by attribute, so both see this check. Every\nreading it does not name is the one `fit.gives` gives.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.on_record.ON_RECORD"); }
    s_ON_RECORD(call_builtin("frozenset", &[py_tuple(vec![pyrt::xrefs::xget("compose.fit", "TIME"), pyrt::xrefs::xget("compose.fit", "AMOUNT"), pyrt::xrefs::xget("compose.fit", "PLACE"), pyrt::xrefs::xget("compose.fit", "PERSON"), pyrt::xrefs::xget("compose.fit", "MEANING"), pyrt::xrefs::xget("compose.fit", "YESNO")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.on_record.NOT_ON_RECORD"); }
    s_NOT_ON_RECORD(stdlib_call("re", "compile", &[py_str("\\b(?:you|ye) (?:have not|haven'?t) (?:said|told|mentioned|let me know)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.on_record.LONGEST"); }
    s_LONGEST(Py::Int(180));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.on_record._gives_before_this_module"); }
    s__gives_before_this_module(py_func("gives", |args: &[Py]| pyrt::xrefs::xcall("compose.fit", "gives", args)));
    let _ = py_setattr(&pyrt::xrefs::xmodule("compose.fit"), "gives", py_func("gives", |args: &[Py]| gives(args)));
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONGEST()) } }) {
    return got.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NOT_ON_RECORD(), "search", &[pyrt::xrefs::xcall("compose.fit", "first_sentence", &[text.clone()])])))) {
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
            "fit" => Some(pyrt::xrefs::xmodule("compose.fit")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "LONGEST" => s_LONGEST(v),
            "NOT_ON_RECORD" => s_NOT_ON_RECORD(v),
            "ON_RECORD" => s_ON_RECORD(v),
            "_gives_before_this_module" => s__gives_before_this_module(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.on_record", module_obj);
    pyrt::xrefs::reg_fn("modules.on_record", "gives", py_func("gives", |args: &[Py]| gives(args)));
    pyrt::xrefs::reg_access("modules.on_record", "LONGEST", g_LONGEST, s_LONGEST);
    pyrt::xrefs::reg_access("modules.on_record", "NOT_ON_RECORD", g_NOT_ON_RECORD, s_NOT_ON_RECORD);
    pyrt::xrefs::reg_access("modules.on_record", "ON_RECORD", g_ON_RECORD, s_ON_RECORD);
    pyrt::xrefs::reg_access("modules.on_record", "_gives_before_this_module", g__gives_before_this_module, s__gives_before_this_module);
}

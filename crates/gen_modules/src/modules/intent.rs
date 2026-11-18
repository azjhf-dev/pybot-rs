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
    let _ = py_str("The kind of answer the line wants, kept where every module and tool can read it.\n\nThe understanding engine reads what the line is about and which question words it carries; this\nwatches that reading and writes `memory[\"understood\"][\"wants\"]`: the shape of answer the person is\nowed - a time, an amount, a place, a person, a meaning, a yes or no, a pick, steps, options or a\nreason (`bot/compose/fit.py` holds the list and the check).\n\nIt observes only, and it never replies. It runs at PRIORITY 9500: below `understand.py` (10000),\nwhich writes `memory[\"understood\"]`, and above every other observer (the memory store is 7000), so\nthe key exists before any module reads the line. A line that asks for no shape of its own - a\ngreeting, a feeling, a question about the bot itself - leaves `wants` unset rather than guessing,\nand a module that does not find the key simply reads the line as it did before.\n\n`bot/compose/clause.py` reads the question out of the clause after the comma (\"camping, might\nrain, what to pack\" is a list, not a verdict) and lifts a reading that the whole line made weak;\na shape the whole line gave is kept.\n\nThe shape is kept in `memory[\"intent_wants\"]` too, so the next line can be read against it: a bare\nfollow-up (\"and for brown rice?\", \"what about once it is up in july\") is asking the same kind of\nanswer about something else, and on its own words it asks for nothing in particular.\n");
    s_PRIORITY(Py::Int(9500));
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut read: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("Write `memory[\"understood\"][\"wants\"]` from this turn's reading, or leave it unset.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    before = mcall(&memory.clone(), "get", &[py_str("intent_wants")]);
    want = gen_compose::compose::what_it_asks::the_shape_it_asks(&[read.clone(), gen_compose::compose::clause::shape(&[read.clone(), gen_compose::compose::fit::wants(&[read.clone(), if py_truthy(&call_builtin("isinstance", &[before.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { before.clone() } else { Py::None }])])]);
    let _ = py_setitem(&memory.clone(), &py_str("intent_wants"), want.clone());
    if py_truthy(&want.clone()) {
    let _ = py_setitem(&read.clone(), &py_str("wants"), want.clone());
    } else {
    let _ = mcall(&read.clone(), "pop", &[py_str("wants"), Py::None]);
    }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.intent".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.intent", module_obj);
    pyrt::xrefs::reg_fn("modules.intent", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_access("modules.intent", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

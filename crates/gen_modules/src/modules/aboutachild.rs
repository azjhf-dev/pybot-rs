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
    let _ = py_str("A question about a child is asked with the age the store has for that child.\n\n\"my son is 5\" then \"how much sleep does he need\" is a line with no age in it, and the sleep\nanswer a pronoun reaches is the adult one - seven to nine hours, coffee at three - while the\nfive year old's is ten to thirteen. The store kept the age a turn ago, so the line is written\nout as \"how much sleep does a five year old need\" and answered from the child's own row.\n\nIt passes on everything else: no child in the store, no pronoun in the line, or a written ask\nthat reaches no row. The line then goes on to the module it went to before this one existed.\n");
    s_PRIORITY(Py::Int(1100));
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This line is the one to answer as it comes: it asks about the child the store keeps.");
    return call_builtin("bool", &[gen_memory::memory::aboutthem::with_the_age(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    let mut reply: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut written: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    written = gen_memory::memory::aboutthem::with_the_age(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&written.clone()))) {
    return Py::None;
    }
    seen = stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&seen.clone(), &py_str("understood"), gen_understanding::understanding::understand(&[written.clone(), seen.clone()]));
    reply = crate::modules::answers::_answer(&[written.clone(), seen.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::None;
    }
    return { let __b = mcall(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.aboutachild".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
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
    pyrt::xrefs::reg_module("modules.aboutachild", module_obj);
    pyrt::xrefs::reg_fn("modules.aboutachild", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.aboutachild", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.aboutachild", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

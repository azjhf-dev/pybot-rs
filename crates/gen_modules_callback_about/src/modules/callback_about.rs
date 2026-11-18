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
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.callback_about: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Asking after a person the chat already holds: answer with what the chat holds.\n\nThe reading and the wording are in `bot/memory/callback.py`. This is the module that offers it,\nand it is a module of its own for the reason `theirperson` is one: the shape is a shape, and a\nhook in the seed fallback would have to grow a branch for the next one.\n\nPRIORITY 260 puts it above `answers` (0) and below `chat` (300) and `theirperson` (250 is below\nit, and both stand down unless the line is their shape). What it can take is the answer to\n`how is she doing`, which without it was the last paragraph the chat had already had, headed\n\"Same as before\"; what it cannot take is a line with its own reading.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.callback_about.PRIORITY"); }
    s_PRIORITY(Py::Int(260));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.callback_about.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("family-and-parenting"), py_str("friends-and-relationships"), py_str("health"), py_str("work"), py_str("school-and-study"), py_str("pets")]));
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
    return { let __b = pyrt::xrefs::xcall("memory.callback", "reply_for", &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn CLAIMS_INSTEAD(args: &[Py]) -> Py {
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
    let _ = py_str("The ask after a person the chat holds, where a module below claims the follow-up.");
    return pyrt::xrefs::xcall("memory.callback", "claims", &[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.callback_about".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "CLAIMS_INSTEAD" => Some(py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args))),
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "callback" => Some(pyrt::xrefs::xmodule("memory.callback")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.callback_about", module_obj);
    pyrt::xrefs::reg_fn("modules.callback_about", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.callback_about", "CLAIMS_INSTEAD", py_func("CLAIMS_INSTEAD", |args: &[Py]| CLAIMS_INSTEAD(args)));
    pyrt::xrefs::reg_access("modules.callback_about", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.callback_about", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

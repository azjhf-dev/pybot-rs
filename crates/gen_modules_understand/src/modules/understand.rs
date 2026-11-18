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
thread_local! { static G_the_thing_it_is: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_understand: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_the_thing_it_is() -> Py { ensure_init(); G_the_thing_it_is.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_thing_it_is(v: Py) { ensure_init(); G_the_thing_it_is.with(|c| *c.borrow_mut() = v); }
pub fn g_understand() -> Py { ensure_init(); G_understand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_understand(v: Py) { ensure_init(); G_understand.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Runs the understanding engine (bot/understanding) before any module proposes a reply.");
    s_the_thing_it_is(pyrt::xrefs::xget("memory", "the_thing_it_is"));
    s_understand(py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args)));
    s_PRIORITY(Py::Int(10000));
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
    let mut filled: Py = Py::None;
    let mut read: Py = Py::None;
    let mut the_advice_ask_with_its_doing: Py = Py::None;
    read = pyrt::xrefs::xcall("understanding", "understand", &[text.clone(), memory.clone()]);
    filled = py_call(&pyrt::xrefs::xget("memory", "the_thing_it_is"), &[read.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&filled.clone(), &Py::None))) {
    the_advice_ask_with_its_doing = py_func("the_advice_ask_with_its_doing", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_advice_ask_with_its_doing", args));
    filled = py_call(&the_advice_ask_with_its_doing, &[read.clone(), memory.clone()]);
    }
    let _ = py_setitem(&memory.clone(), &py_str("understood"), if py_truthy(&Py::Bool(!py_is(&filled.clone(), &Py::None))) { filled.clone() } else { read.clone() });
    let _ = py_setitem(&geti(&memory.clone(), &py_str("understood")), &py_str("looks_back"), pyrt::xrefs::xcall("memory.recall", "the_earlier_turn", &[text.clone(), memory.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.understand".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "the_thing_it_is" => Some(g_the_thing_it_is()),
            "understand" => Some(g_understand()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "the_thing_it_is" => s_the_thing_it_is(v),
            "understand" => s_understand(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.understand", module_obj);
    pyrt::xrefs::reg_fn("modules.understand", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_access("modules.understand", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.understand", "the_thing_it_is", g_the_thing_it_is, s_the_thing_it_is);
    pyrt::xrefs::reg_access("modules.understand", "understand", g_understand, s_understand);
}

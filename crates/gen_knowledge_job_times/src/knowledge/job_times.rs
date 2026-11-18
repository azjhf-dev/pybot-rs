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
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_time_entries: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.job_times: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_time_entries() -> Py { ensure_init(); G_time_entries.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_time_entries(v: Py) { ensure_init(); G_time_entries.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How long the everyday jobs in the house take, kept apart from the big home domain.\n\n`bot/knowledge/home.py` is past the publisher's 1 MiB file limit, so a row added to it cannot\nship: this file answers the home job whose length people ask for - a house survey - and declares\nthe same topic, so it answers exactly what a row in `home.py` would. The row itself is built\nfrom `JOB_TIMES` in `bot/knowledge/costs.py`, beside the prices for the same jobs, and a job's\nrow is only built where its length is missing from another card: the boiler service already has\nits length on main, so it is not in that table.\n\nBeing a small separate domain has a second use: a mistake in an import line here loses this row\nand nothing else, where a broken import in `home.py` would take the whole house with it\n(`domains()` skips a module that fails to import).\n");
    s_time_entries(py_func("time_entries", |args: &[Py]| pyrt::xrefs::xcall("knowledge.costs", "time_entries", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.job_times.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.job_times.ENTRIES"); }
    s_ENTRIES(pyrt::xrefs::xcall("knowledge.costs", "time_entries", &[py_str("house-survey")]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.job_times".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "time_entries" => Some(g_time_entries()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "time_entries" => s_time_entries(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.job_times", module_obj);
    pyrt::xrefs::reg_access("knowledge.job_times", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.job_times", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.job_times", "time_entries", g_time_entries, s_time_entries);
}

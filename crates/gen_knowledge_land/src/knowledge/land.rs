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
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.land: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Land and title: who owns a piece of ground, and how you find out.\n\nThe answers lane owns this domain. The rows are here rather than in `home.py` because\n`home.py` is where every property line already lands: an append to a file that a dozen other\nanswers change every day is a merge conflict waiting to happen, and the ground and the\nregister that records who holds it are a domain of their own, not one more thing about a\nkitchen.\n\nThe Moon stays out of this row by its own avoid list (\"moon\", \"planet\", \"space\", \"orbit\",\n\"satellite\"): \"who owns the moon\" is a space question, and `history.py`'s `moon-landing` keeps it.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("money")]));
    fn __data1() -> Py { py_tuple(vec![py_str("who owns"), py_str("who owned"), py_str("land registry"), py_str("title register"), py_str("title deeds"), py_str("deeds"), py_str("piece of land"), py_str("plot of land"), py_str("land next to"), py_str("land behind"), py_str("field behind"), py_str("property boundary"), py_str("boundary line"), py_str("boundary dispute")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("land"), py_str("plot"), py_str("field"), py_str("boundary"), py_str("deeds"), py_str("register"), py_str("property"), py_str("garden"), py_str("hedge"), py_str("owns"), py_str("owner"), py_str("mine"), py_str("next door"), py_str("neighbour"), py_str("neighbors")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("what is"), py_str("what are"), py_str("whats"), py_str("what does"), py_str("meaning"), py_str("difference between"), py_str("define"), py_str("explain"), py_str("sell"), py_str("selling"), py_str("sold"), py_str("buy land"), py_str("buying land"), py_str("build on"), py_str("permission"), py_str("planning permission"), py_str("mortgage"), py_str("stamp duty"), py_str("tax"), py_str("inheritance"), py_str("will"), py_str("squatter"), py_str("adverse possession"), py_str("trespass"), py_str("fence cost"), py_str("how much"), py_str("cost of"), py_str("moon"), py_str("planet"), py_str("space"), py_str("orbit"), py_str("satellite"), py_str("buy a piece of land"), py_str("buy some land"), py_str("buy land"), py_str("buying a piece of land"), py_str("buying some land"), py_str("buying land")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who-owns-some-land")); __args.push(py_tuple(vec![__data1(), __data2()])); __args.push(py_str("In England and Wales the owner is on the title at HM Land Registry: use its Find a property service, pay about £3, and the title register gives you the owner's name, the price last paid and the boundaries on a map, without telling them you asked. Scotland is Registers of Scotland, and Northern Ireland is Land and Property Services. If the plot is unregistered, the deeds are with the owner or held by the Land Registry, and your council can help you find out who to ask.")); __kws.extend(vec![("avoid".to_string(), __data3()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.land".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.land", module_obj);
    pyrt::xrefs::reg_access("knowledge.land", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.land", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.land", "entry", g_entry, s_entry);
}

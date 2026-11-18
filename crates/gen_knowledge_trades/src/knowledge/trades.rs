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
        let _ = exc::catch(|| { module_body(); Py::None });
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
    let _ = py_str("Trades: getting a plumber, an electrician or a locksmith to come out.\n\nThe answers lane owns this domain. The row is here rather than in `home.py` for the reason\n`land.py` gives: `home.py` is what every property line already reaches, and this question -\nwhen a trade comes and what they charge - is a service question, not a thing about the house.\n\nThe composer's hand-off to a trade (\"a plumber or electrician will spot it in minutes\") is\nwritten for a line with something broken in it. This row answers first for the lines that ask\nhow to get hold of one, so the hand-off is only reached by a line that names a fault.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("money")]));
    fn __data1() -> Py { py_tuple(vec![py_str("plumber"), py_str("plumbers"), py_str("electrician"), py_str("electricians"), py_str("gas engineer"), py_str("gas man"), py_str("heating engineer"), py_str("boiler engineer"), py_str("handyman"), py_str("locksmith"), py_str("roofer"), py_str("joiner"), py_str("drain man"), py_str("drainage")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("get"), py_str("getting"), py_str("find"), py_str("finding"), py_str("book"), py_str("booking"), py_str("hire"), py_str("hiring"), py_str("recommend"), py_str("recommendation"), py_str("weekend"), py_str("weekends"), py_str("saturday"), py_str("sundays"), py_str("sunday"), py_str("evening"), py_str("evenings"), py_str("tonight"), py_str("night"), py_str("bank holiday"), py_str("out of hours"), py_str("emergency"), py_str("call out"), py_str("callout"), py_str("near me"), py_str("same day"), py_str("today")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("fix"), py_str("mend"), py_str("repair"), py_str("unblock"), py_str("bleed"), py_str("install"), py_str("fit it"), py_str("myself"), py_str("my own"), py_str("how do i change"), py_str("quote"), py_str("cost"), py_str("charge"), py_str("how much"), py_str("salary"), py_str("training"), py_str("become a plumber"), py_str("become an electrician"), py_str("course"), py_str("apprenticeship"), py_str("what is"), py_str("what does a"), py_str("difference between"), py_str("wire a"), py_str("socket"), py_str("fuse box"), py_str("boiler is"), py_str("boiler makes"), py_str("boiler noise"), py_str("pressure"), py_str("radiator"), py_str("leak")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("trades-out-of-hours")); __args.push(py_tuple(vec![__data1(), __data2()])); __args.push(py_str("Book early and ask two questions: the call-out charge and the hourly rate after the first hour, because a weekend call-out is usually about double the weekday one and the quote over the phone is what holds them to it. Ring three local firms from eight in the morning and say what is wrong and when you need someone; the one who can tell you what it probably is on the phone is the one to book. Anything with water coming through is worth a call straight away - turn the stopcock off first - and a smell of gas is the free 24-hour gas emergency line, 0800 111 999, not a plumber.")); __kws.extend(vec![("avoid".to_string(), __data3())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.trades".to_string(),
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
    pyrt::xrefs::reg_module("knowledge.trades", module_obj);
    pyrt::xrefs::reg_access("knowledge.trades", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.trades", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.trades", "entry", g_entry, s_entry);
}

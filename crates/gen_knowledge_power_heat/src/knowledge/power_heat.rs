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
thread_local! { static G__HARDWARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_WASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__HARDWARE() -> Py { ensure_init(); G__HARDWARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HARDWARE(v: Py) { ensure_init(); G__HARDWARE.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAT() -> Py { ensure_init(); G__HEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAT(v: Py) { ensure_init(); G__HEAT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_WASH() -> Py { ensure_init(); G__NOT_A_WASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_WASH(v: Py) { ensure_init(); G__NOT_A_WASH.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Mains power that is hot: a plug, a socket, a cable or a charger.\n\n`use-the-washer` asks on the bare phrase \"the washing\", so a line about a hot plug beside a\nwashing machine was answered with how to sort a load, and `after-a-tooth-out` keys \"socket\",\nso a hot wall socket was answered as a socket in the mouth. Both are the row that owns the\nword, not the row that owns the question.\n\nThis row owns the question: electrical hardware that is hot, warm, buzzing, scorched or\nsmoking. It stands down for the dial of a wash, for a tooth, and for a how-to-use question.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("technology")]));
    fn __data1() -> Py { py_tuple(vec![py_str("hot"), py_str("warm"), py_str("warm to the touch"), py_str("hot to the touch"), py_str("too hot"), py_str("very hot"), py_str("burning"), py_str("burnt"), py_str("scorched"), py_str("scorch"), py_str("smells of burning"), py_str("smell of burning"), py_str("smells like burning"), py_str("smells of hot plastic"), py_str("smoking"), py_str("smoke"), py_str("sparks"), py_str("sparking"), py_str("buzzing"), py_str("buzzes"), py_str("melting"), py_str("melted"), py_str("overheating")]) }
    s__HEAT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("plug"), py_str("plugs"), py_str("plug socket"), py_str("socket"), py_str("sockets"), py_str("wall socket"), py_str("mains"), py_str("cable"), py_str("cables"), py_str("lead"), py_str("leads"), py_str("flex"), py_str("wire"), py_str("wires"), py_str("wiring"), py_str("extension lead"), py_str("fuse"), py_str("fuses"), py_str("dimmer"), py_str("light switch")]) }
    s__HARDWARE(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("temperature"), py_str("degrees"), py_str("what temperature"), py_str("wool"), py_str("shrink"), py_str("shrunk"), py_str("stain"), py_str("stains"), py_str("how do i"), py_str("how to"), py_str("wash my clothes"), py_str("load"), py_str("sort"), py_str("lights"), py_str("darks"), py_str("wash at"), py_str("cheaper"), py_str("soap"), py_str("detergent"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("dentist"), py_str("mouth"), py_str("extraction"), py_str("wisdom"), py_str("filling"), py_str("dentures"), py_str("dry socket"), py_str("bleeding"), py_str("blood"), py_str("clot"), py_str("ache")]) }
    s__NOT_A_WASH(__data3());
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("power-hot")); __args.push(py_tuple(vec![g__HARDWARE(), g__HEAT()])); __args.push(py_str("A plug, socket or cable that is hot, buzzing, scorched or smells of burning is not safe to keep using: unplug it at the wall, or switch that circuit off, and leave it off. Warm to the touch is normal for a charger; too hot to hold, crackling, brown marks, a fishy smell of hot plastic or a fuse that keeps tripping is not, and that plug or socket needs replacing by an electrician before it is used again. Never open a socket yourself, and do not just change the fuse and carry on.")); __kws.extend(vec![("avoid".to_string(), g__NOT_A_WASH()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.power_heat".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_HARDWARE" => Some(g__HARDWARE()),
            "_HEAT" => Some(g__HEAT()),
            "_NOT_A_WASH" => Some(g__NOT_A_WASH()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_HARDWARE" => s__HARDWARE(v),
            "_HEAT" => s__HEAT(v),
            "_NOT_A_WASH" => s__NOT_A_WASH(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.power_heat", module_obj);
    pyrt::xrefs::reg_access("knowledge.power_heat", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.power_heat", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.power_heat", "_HARDWARE", g__HARDWARE, s__HARDWARE);
    pyrt::xrefs::reg_access("knowledge.power_heat", "_HEAT", g__HEAT, s__HEAT);
    pyrt::xrefs::reg_access("knowledge.power_heat", "_NOT_A_WASH", g__NOT_A_WASH, s__NOT_A_WASH);
    pyrt::xrefs::reg_access("knowledge.power_heat", "entry", g_entry, s_entry);
}

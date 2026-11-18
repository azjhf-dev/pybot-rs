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
thread_local! { static G__ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__ASKING() -> Py { ensure_init(); G__ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING(v: Py) { ensure_init(); G__ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISE() -> Py { ensure_init(); G__RAISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISE(v: Py) { ensure_init(); G__RAISE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How high the screen sits, asked as \"should i get a laptop stand\".\n\nscience.py's posture row already says what to do - screen at eye level, feet flat, elbows at\nninety degrees - but it is keyed on the complaint (\"my back aches at my desk\"), so the question\npeople actually ask when they are about to spend money, \"should i get a laptop stand\", scored on\n\"get\" and \"laptop\" and landed on the laptop buying card with 16GB of RAM in it.\n");
    s_ANSWERS(py_tuple(vec![py_str("technology"), py_str("home-and-diy"), py_str("everyday-help")]));
    s__RAISE(py_tuple(vec![py_str("screen"), py_str("monitor"), py_str("stand"), py_str("riser"), py_str("raiser"), py_str("laptop stand"), py_str("laptop riser"), py_str("laptop raiser"), py_str("monitor stand"), py_str("monitor riser"), py_str("screen stand"), py_str("screen riser"), py_str("raise my screen"), py_str("raise the screen"), py_str("screen height"), py_str("screen is low"), py_str("screen too low"), py_str("screen quite low"), py_str("screen is quite low"), py_str("laptop on books"), py_str("laptop on a box"), py_str("screen at eye level")]));
    s__ASKING(py_tuple(vec![py_str("should i"), py_str("worth"), py_str("worth it"), py_str("get"), py_str("buy"), py_str("need"), py_str("any good"), py_str("help"), py_str("good idea"), py_str("does it help"), py_str("is it better"), py_str("which"), py_str("stand"), py_str("riser"), py_str("raiser"), py_str("height"), py_str("low"), py_str("too low"), py_str("quite low"), py_str("hurts"), py_str("hurting"), py_str("aching"), py_str("aching away")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("desk-screen-height")); __args.push(py_tuple(vec![g__RAISE(), g__ASKING()])); __args.push(py_str("Yes, if your screen is below eye level: a stand or a pile of books brings the top of the screen to eye level, an external keyboard and mouse go at elbow height, and that is what stops the neck craning down all day. A laptop alone is always too low for long sessions, so the cheap version - box under the laptop, separate keyboard - does most of the job; buy the stand if you want it to look tidy or to swing up and down.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("battery"), py_str("charger"), py_str("crash"), py_str("won't turn on"), py_str("wont turn on"), py_str("bulging"), py_str("overheating"), py_str("fan"), py_str("hard drive"), py_str("ssd"), py_str("ram"), py_str("memory upgrade"), py_str("spec"), py_str("specs"), py_str("new laptop"), py_str("recommend a laptop"), py_str("standing desk"), py_str("standing desks"), py_str("sit stand"), py_str("sit-stand"), py_str("sit to stand"), py_str("walking pad"), py_str("treadmill desk"), py_str("desk you stand at"), py_str("cracked"), py_str("crack"), py_str("tv"), py_str("telly"), py_str("phone"), py_str("broken"), py_str("flickering"), py_str("flicker"), py_str("bigger"), py_str("larger"), py_str("4k"), py_str("resolution"), py_str("frozen"), py_str("black screen"), py_str("second monitor"), py_str("second screen"), py_str("two screens"), py_str("two monitors"), py_str("dual monitor"), py_str("dual screen"), py_str("dual monitors"), py_str("extra screen"), py_str("another monitor"), py_str("another screen"), py_str("third screen")])), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.desk_setup".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKING" => Some(g__ASKING()),
            "_RAISE" => Some(g__RAISE()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKING" => s__ASKING(v),
            "_RAISE" => s__RAISE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.desk_setup", module_obj);
    pyrt::xrefs::reg_access("knowledge.desk_setup", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.desk_setup", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.desk_setup", "_ASKING", g__ASKING, s__ASKING);
    pyrt::xrefs::reg_access("knowledge.desk_setup", "_RAISE", g__RAISE, s__RAISE);
}

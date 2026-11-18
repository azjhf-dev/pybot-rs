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
fn module_body() {
    let _ = py_str("Parking on the pavement, a verge or a kerb: the one legality ask with no card.\n\n`is it legal to park on the pavement` and `is it against the law to park on the pavement` both\nreached the search hedge, while nine of the eleven legality lines in the deck answer from a card.\nThe answer depends on where the person lives, so the card says where it depends - an offence\nright across London and in Scotland, elsewhere only where a traffic order covers that street -\nand names the signs on that street and the council's own page as what decides their road, rather\nthan giving a flat yes or no.\n\nIt is its own file rather than a row inside cars.py because the surface word is the whole\nquestion, and a card about a footway has nothing to do with engines, tyres or warning lights.\n\nThe row is gated on the surface word, so \"where can i park\" is not this question and keeps its\nlocal-rules line, and a pavement that is broken, icy or covered in litter is a repair, a fall or\na report rather than a parking rule.\n");
    s_ANSWERS(py_tuple(vec![py_str("cars")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("parking-on-the-pavement")); __args.push(py_tuple(vec![py_tuple(vec![py_str("park on the pavement"), py_str("parking on the pavement"), py_str("pavement"), py_str("pavements"), py_str("park on the verge"), py_str("parking on the verge"), py_str("verge"), py_str("grass verge"), py_str("footway"), py_str("footpaths"), py_str("kerb"), py_str("curb"), py_str("dropped kerb")]), py_tuple(vec![py_str("legal"), py_str("illegal"), py_str("allowed"), py_str("allow"), py_str("against the law"), py_str("law"), py_str("permitted"), py_str("ticket"), py_str("fined"), py_str("fine"), py_str("penalty"), py_str("can i"), py_str("is it"), py_str("is that"), py_str("am i"), py_str("park"), py_str("parked"), py_str("parking"), py_str("ok"), py_str("okay")])])); __args.push(py_str("Parking on the pavement is an offence right across London and in Scotland, and elsewhere only where a traffic order covers that street - so the signs and lines on it, and the council's own parking page, are what decide your road, and that is where to check rather than the country as a whole. Where it is allowed you must still leave the pavement clear enough for a wheelchair or a pushchair, and blocking a dropped kerb blocks someone's drive. A ticket for it is a penalty charge notice, argued in the council's own appeal rather than in a court.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("pavement"), py_str("pavements"), py_str("verge"), py_str("grass verge"), py_str("footway"), py_str("footpaths"), py_str("kerb"), py_str("curb"), py_str("dropped kerb")])), ("avoid".to_string(), py_tuple(vec![py_str("trip"), py_str("tripped"), py_str("tripping"), py_str("fell"), py_str("fell over"), py_str("slipped"), py_str("slippy"), py_str("icy"), py_str("ice"), py_str("uneven"), py_str("pothole"), py_str("potholes"), py_str("cracked"), py_str("damaged"), py_str("injur"), py_str("injured"), py_str("claim"), py_str("compensation"), py_str("wheelchair access"), py_str("pushed"), py_str("buggy"), py_str("litter"), py_str("dog mess"), py_str("graffiti"), py_str("fixed"), py_str("repair")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.parking".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.parking", module_obj);
    pyrt::xrefs::reg_access("knowledge.parking", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.parking", "ENTRIES", g_ENTRIES, s_ENTRIES);
}

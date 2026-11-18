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
    let _ = py_str("The first day at secondary school: a school year is not a length of time.\n\nThe answers lane owns this domain. It is one question, asked over and over in the last week of\nAugust by a parent who has just been told which form their child is in: what do I send them with.\n\nBoth rows that used to answer it were answering a different question. The gone-quiet row is keyed\non \"in years\", the length of time a silence is measured in, and the matcher folds inflections, so\n\"my daughter is in year 7, what should she take on her first day\" reached \"A relative going quiet\nis usually nothing more than a busy year\". The weaning card is keyed on a child's words plus\n\"first\", so \"what should my son take on his first day at secondary school\" reached \"Start with\nsoft food they can hold themselves\".\n\nA school year is a phrase in its own right here - \"year 7\" to \"year 11\", \"first day of year 8\",\n\"starting secondary\", \"big school\" - so the row reaches the same line the gone-quiet row reaches\nand scores higher on the day itself, rather than standing another row down.\n\nEverything else about a child that age already has a good answer and keeps it: the school's\nuniform list, whether they need a phone, packed lunch ideas, being nervous about the change, and\nwhat time the day starts. Each of those is fenced out of this row by name.\n");
    s_ANSWERS(py_tuple(vec![py_str("family-and-parenting"), py_str("school-and-study")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("secondary-first-day-kit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("first day at school"), py_str("first day at secondary"), py_str("first day at big school"), py_str("first day of year 7"), py_str("first day of year 8"), py_str("first day of year 9"), py_str("first day in year 7"), py_str("first day in year 8"), py_str("first day in year 9"), py_str("starting year 7"), py_str("starting year 8"), py_str("starting year 9"), py_str("starting year 10"), py_str("starts year 7"), py_str("starts year 8"), py_str("starts year 10"), py_str("starting secondary"), py_str("starts secondary"), py_str("starting high school"), py_str("starts high school"), py_str("starting big school"), py_str("starts big school"), py_str("new school"), py_str("year 7"), py_str("year 8"), py_str("year 9"), py_str("year 10"), py_str("year 11"), py_str("yr 7"), py_str("yr 8"), py_str("yr 9"), py_str("yr 10"), py_str("secondary school"), py_str("high school"), py_str("big school")]), py_tuple(vec![py_str("take"), py_str("bring"), py_str("pack"), py_str("put in"), py_str("need"), py_str("buy"), py_str("get"), py_str("kit"), py_str("equipment"), py_str("bag"), py_str("rucksack"), py_str("wear"), py_str("first"), py_str("day"), py_str("start")])])); __args.push(py_str("Nothing fancy for the first day: a named bag with a pencil case (two pens, a pencil, a rubber, a ruler), a water bottle, a packed lunch or the money loaded on the dinner account, and the PE kit the school asked for, in a bag of its own if they can. Label everything, including the coat and the shoes, and put the timetable in the front pocket. Buy the cheap version of each until the school sends its list, then fill the gaps.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("baby"), py_str("weaning"), py_str("puree"), py_str("pureed"), py_str("milk"), py_str("bottle"), py_str("breast"), py_str("formula"), py_str("solids"), py_str("teething"), py_str("nursery"), py_str("reception"), py_str("first birthday"), py_str("first tooth"), py_str("phone"), py_str("mobile"), py_str("smartphone"), py_str("ipad"), py_str("tablet"), py_str("laptop"), py_str("console"), py_str("uniform"), py_str("list"), py_str("buy"), py_str("buying"), py_str("prices"), py_str("price"), py_str("how much"), py_str("cost"), py_str("nervous"), py_str("worried"), py_str("worry"), py_str("scared"), py_str("anxious"), py_str("anxiety"), py_str("settle"), py_str("settling"), py_str("cope"), py_str("coping"), py_str("make friends"), py_str("friends"), py_str("bully"), py_str("bullied"), py_str("what time"), py_str("times"), py_str("hours"), py_str("bus"), py_str("route"), py_str("walk to school"), py_str("term dates"), py_str("holidays"), py_str("first word"), py_str("first steps"), py_str("first day at work"), py_str("first day in the job"), py_str("new job"), py_str("back at work"), py_str("interview"), py_str("leaving"), py_str("left"), py_str("return to work"), py_str("gone quiet"), py_str("hasnt called"), py_str("wont talk"), py_str("not speaking")])), ("ask".to_string(), py_tuple(vec![py_str("what"), py_str("need"), py_str("take"), py_str("should"), py_str("do i"), py_str("first day"), py_str("starting"), py_str("starts")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.school_day".to_string(),
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
    pyrt::xrefs::reg_module("knowledge.school_day", module_obj);
    pyrt::xrefs::reg_access("knowledge.school_day", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.school_day", "ENTRIES", g_ENTRIES, s_ENTRIES);
}

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


thread_local! { static G_CAREER_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAREER_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__career_subjects: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.nursing: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CAREER_SUBJECTS() -> Py { ensure_init(); G_CAREER_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAREER_SUBJECTS(v: Py) { ensure_init(); G_CAREER_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__CAREER_NAMES() -> Py { ensure_init(); G__CAREER_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAREER_NAMES(v: Py) { ensure_init(); G__CAREER_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__POINT_WORDS() -> Py { ensure_init(); G__POINT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINT_WORDS(v: Py) { ensure_init(); G__POINT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__career_subjects() -> Py { ensure_init(); G__career_subjects.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__career_subjects(v: Py) { ensure_init(); G__career_subjects.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Entry points for a named job, and the other things a line can call points.\n\n\"how many points do i need for nursing\" was answered with \"Yes - it is the normal way to feed a\nbaby and it is what the milk is for. Feed on demand ... eight to twelve times in 24 hours.\" The\nfeeding card keys on \"nursing\" and on \"how many\", the driving card keys on \"points\", and nothing\nheld either of them off a question about UCAS points. The careers card in study.py has the\nanswer - \"Nursing wants five GCSEs at grade 4 or above\" - but its key asks for the exam words\n(\"what gcses are needed\") and a line that says \"points\" only reached the two cards that are\nabout something else.\n\nThe row here is the pairing that makes the question: a job or a course, and a points word. It\nlives in its own domain on purpose. The matcher weighs a key word by how rare it is inside its\nown domain, so a second row in study.py that lists every career name would make \"nursing\" common\nthere and quietly change which card two other lines reached. A domain of its own leaves every\nother card's score exactly as it was.\n\nThe answer is study.py's own table, so there is one place that knows what nursing asks for.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_CAREER_SUBJECTS(pyrt::xrefs::xget("knowledge.study", "CAREER_SUBJECTS"));
    s__career_subjects(py_func("_career_subjects", |args: &[Py]| pyrt::xrefs::xcall("knowledge.study", "_career_subjects", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nursing.ENTRIES"); }
    s_ENTRIES(py_list(vec![]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nursing._POINT_WORDS"); }
    s__POINT_WORDS(py_tuple(vec![py_str("points"), py_str("ucas points"), py_str("ucas"), py_str("entry points"), py_str("tariff points"), py_str("tariff"), py_str("how many points"), py_str("points do i need"), py_str("points needed"), py_str("points for")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nursing._CAREER_NAMES"); }
    s__CAREER_NAMES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CAREER_SUBJECTS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("licence"), py_str("license"), py_str("driving"), py_str("driver"), py_str("penalty"), py_str("speeding"), py_str("ban"), py_str("banned"), py_str("insurance"), py_str("f1"), py_str("formula 1"), py_str("formula one"), py_str("grand prix"), py_str("motogp"), py_str("lap"), py_str("laps"), py_str("try"), py_str("tries"), py_str("goal"), py_str("goals"), py_str("touchdown"), py_str("conversion"), py_str("rugby"), py_str("football"), py_str("soccer"), py_str("cricket"), py_str("netball"), py_str("basketball"), py_str("hockey"), py_str("tennis"), py_str("match"), py_str("game"), py_str("games"), py_str("score"), py_str("scored"), py_str("scoring"), py_str("season"), py_str("team"), py_str("teams"), py_str("loyalty"), py_str("reward"), py_str("rewards"), py_str("reward card"), py_str("nectar"), py_str("clubcard"), py_str("club card"), py_str("store card"), py_str("voucher"), py_str("vouchers"), py_str("coffee"), py_str("free coffee"), py_str("free drink"), py_str("supermarket"), py_str("shop"), py_str("shopping"), py_str("air miles"), py_str("my card"), py_str("dehumidifier"), py_str("point of")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("career-entry-points")); __args.push(py_tuple(vec![g__CAREER_NAMES(), g__POINT_WORDS()])); __args.push(g__career_subjects()); __kws.extend(vec![("avoid".to_string(), __data1()), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.nursing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CAREER_SUBJECTS" => Some(g_CAREER_SUBJECTS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_CAREER_NAMES" => Some(g__CAREER_NAMES()),
            "_POINT_WORDS" => Some(g__POINT_WORDS()),
            "_career_subjects" => Some(g__career_subjects()),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CAREER_SUBJECTS" => s_CAREER_SUBJECTS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_CAREER_NAMES" => s__CAREER_NAMES(v),
            "_POINT_WORDS" => s__POINT_WORDS(v),
            "_career_subjects" => s__career_subjects(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.nursing", module_obj);
    pyrt::xrefs::reg_access("knowledge.nursing", "CAREER_SUBJECTS", g_CAREER_SUBJECTS, s_CAREER_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.nursing", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.nursing", "_CAREER_NAMES", g__CAREER_NAMES, s__CAREER_NAMES);
    pyrt::xrefs::reg_access("knowledge.nursing", "_POINT_WORDS", g__POINT_WORDS, s__POINT_WORDS);
    pyrt::xrefs::reg_access("knowledge.nursing", "_career_subjects", g__career_subjects, s__career_subjects);
    pyrt::xrefs::reg_access("knowledge.nursing", "entry", g_entry, s_entry);
}

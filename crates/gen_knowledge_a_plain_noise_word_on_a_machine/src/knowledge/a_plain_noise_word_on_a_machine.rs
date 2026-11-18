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
thread_local! { static G__NOISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PIPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.a_plain_noise_word_on_a_machine: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOISE() -> Py { ensure_init(); G__NOISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOISE(v: Py) { ensure_init(); G__NOISE.with(|c| *c.borrow_mut() = v); }
pub fn g__PIPES() -> Py { ensure_init(); G__PIPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PIPES(v: Py) { ensure_init(); G__PIPES.with(|c| *c.borrow_mut() = v); }
pub fn g__VENTS() -> Py { ensure_init(); G__VENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VENTS(v: Py) { ensure_init(); G__VENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A sound a machine makes, said with a word the machine's own card did not know by name.\n\n\"The radiator is clattering\" and \"my laptop fan is making a racket\" are the same faults as \"the\nradiator is noisy\" and \"my laptop fan is loud\", and until now only the second pair got the card:\nthe first pair came back as a reflection of the line (\"Got it - the radiator is clattering\").\n\nThe radiator card and the laptop fan card live in two files that every lane is editing, and a\nchange inside their word lists is always a conflict. These two rows are the same cards keyed on\nthe plain words for a sound, so they are here as their own domain and nothing hot has to move. The\nmachine still has to be named and the words are about a sound, so a leaking radiator, a bleeding\none, a cold top, a thermostat, a quarrel and a room fan all keep the answers they had.\n\nEach row mirrors one card and says which: `radiator-plain-noise` is the pipes answer of\n`radiator-noise` in `home.py`, and `laptop-fan-plain-noise` is the vents answer of\n`laptop-fan-loud` in `technology.py`. If either card's wording is rewritten, its row here has to\nfollow, which is the price of not editing the hot files.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.a_plain_noise_word_on_a_machine.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("technology")]));
    fn __data1() -> Py { py_tuple(vec![py_str("clatter"), py_str("clatters"), py_str("clattering"), py_str("racket"), py_str("racketing"), py_str("din"), py_str("drone"), py_str("droning"), py_str("thunderous"), py_str("deafening"), py_str("palaver"), py_str("rattling"), py_str("roaring"), py_str("howling")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.a_plain_noise_word_on_a_machine._NOISE"); }
    s__NOISE(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.a_plain_noise_word_on_a_machine._PIPES"); }
    s__PIPES(py_str("Ticking and clicking is the pipes expanding as they heat, which is normal and stops once the system is warm. Banging or gurgling means air or trapped water: bleed the radiators and check the boiler pressure sits between 1 and 1.5 bar. If a radiator clatters or drones while the heating is running, the valve may be part open or the pump set too fast: open the valve fully and turn the pump down a step."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.a_plain_noise_word_on_a_machine._VENTS"); }
    s__VENTS(py_str("It is cooling itself because it is hot, so clear the vents with a can of air and check what is using the processor in Task Manager or Activity Monitor. A four-year-old laptop often just needs the dust out and new thermal paste (about £40 at a repair shop); if the noise continues at idle, the fan bearing is going."));
    fn __data2() -> Py { py_tuple(vec![py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("leaked"), py_str("cold"), py_str("colder"), py_str("bleed"), py_str("bleeding"), py_str("bled"), py_str("thermostat"), py_str("valve"), py_str("valves"), py_str("pressure"), py_str("bar"), py_str("not working"), py_str("doesnt work"), py_str("doesn't work"), py_str("wont heat"), py_str("won't heat"), py_str("radiator paint"), py_str("painting"), py_str("remove"), py_str("take off"), py_str("replace"), py_str("size"), py_str("how many")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("my fan"), py_str("fan heater"), py_str("ceiling"), py_str("desk fan"), py_str("pedestal"), py_str("floor fan"), py_str("tower fan"), py_str("box fan"), py_str("wall fan"), py_str("room fan"), py_str("bathroom"), py_str("kitchen"), py_str("extractor"), py_str("cooker hood"), py_str("oven"), py_str("fridge"), py_str("freezer"), py_str("boiler"), py_str("radiator"), py_str("washer"), py_str("washing machine"), py_str("dishwasher"), py_str("tumble"), py_str("dryer"), py_str("car"), py_str("van"), py_str("engine"), py_str("console"), py_str("playstation"), py_str("ps5"), py_str("ps4"), py_str("xbox"), py_str("nintendo"), py_str("computer fan"), py_str("pc fan"), py_str("desktop"), py_str("tower"), py_str("not charging"), py_str("won't turn on"), py_str("wont turn on"), py_str("black screen"), py_str("slow"), py_str("overheat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.a_plain_noise_word_on_a_machine.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("radiator-plain-noise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("radiator"), py_str("radiators")]), g__NOISE()])); __args.push(g__PIPES()); __kws.extend(vec![("avoid".to_string(), __data2())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("laptop-fan-plain-noise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("laptop"), py_str("laptops"), py_str("macbook"), py_str("notebook")]), g__NOISE()])); __args.push(g__VENTS()); __kws.extend(vec![("avoid".to_string(), __data3())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.a_plain_noise_word_on_a_machine".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_NOISE" => Some(g__NOISE()),
            "_PIPES" => Some(g__PIPES()),
            "_VENTS" => Some(g__VENTS()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_NOISE" => s__NOISE(v),
            "_PIPES" => s__PIPES(v),
            "_VENTS" => s__VENTS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.a_plain_noise_word_on_a_machine", module_obj);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "_NOISE", g__NOISE, s__NOISE);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "_PIPES", g__PIPES, s__PIPES);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "_VENTS", g__VENTS, s__VENTS);
    pyrt::xrefs::reg_access("knowledge.a_plain_noise_word_on_a_machine", "entry", g_entry, s_entry);
}

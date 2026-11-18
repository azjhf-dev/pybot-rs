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
thread_local! { static G_MICROWAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.microwave_methods: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MICROWAVE() -> Py { ensure_init(); G_MICROWAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MICROWAVE(v: Py) { ensure_init(); G_MICROWAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_METHOD() -> Py { ensure_init(); G_NOT_A_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_METHOD(v: Py) { ensure_init(); G_NOT_A_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Cooking in the microwave: pasta, scrambled eggs and vegetables.\n\nThe microwave answers were all about something else, because the word \"microwave\" is in the key of\nthe appliance cards rather than of a method:\n\n  how do i cook pasta in the microwave      \"Empty one cupboard at a time, wipe the shelves with hot\n                                            soapy water\" - the cupboards card\n  how do i scramble eggs in the microwave   \"melt a knob of butter in a pan\" - a hob recipe\n  how do i steam vegetables in the microwave \"That's beyond what I know well, sorry\"\n\nThe three that already answer well keep their lines: `how do i reheat rice safely`, `how do i cook\njacket potato in the microwave` and `how do i cook rice in the microwave` are not this table's\nquestion, and \"microwave\" is not in these keys without a method word beside it.\n\nEach row needs the appliance and its own dish, so a hob recipe, a reheat and a fault stay where they\nare.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking")]));
    s_MICROWAVE(py_tuple(vec![py_str("microwave"), py_str("microwave oven"), py_str("microwaves"), py_str("microvave"), py_str("microwaving"), py_str("microwave it")]));
    fn __data1() -> Py { py_tuple(vec![py_str("not working"), py_str("broken"), py_str("sparking"), py_str("sparks"), py_str("smell"), py_str("smells"), py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("repair"), py_str("buy"), py_str("buying"), py_str("which microwave"), py_str("best"), py_str("reheat"), py_str("reheating"), py_str("reheated"), py_str("warm up"), py_str("warm it up"), py_str("defrost"), py_str("defrosting"), py_str("how long"), py_str("jacket potato"), py_str("baked potato"), py_str("rice")]) }
    s_NOT_A_METHOD(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("broccoli"), py_str("green beans"), py_str("carrots"), py_str("carrot"), py_str("cauliflower"), py_str("peas"), py_str("sweetcorn"), py_str("cabbage"), py_str("spinach"), py_str("kale"), py_str("courgette"), py_str("asparagus"), py_str("sprouts"), py_str("steam"), py_str("steaming"), py_str("steamed")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-in-the-microwave")); __args.push(py_tuple(vec![g_MICROWAVE(), py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("noodles"), py_str("tagliatelle"), py_str("lasagne sheets"), py_str("cook pasta"), py_str("boil pasta")])])); __args.push(py_str("Pasta cooks fine in a microwave, in a big bowl with plenty of room: cover it with boiling water from the kettle, add a good pinch of salt, and give it 8 to 10 minutes on full power, stirring once at the halfway point so the pieces at the top get turned under. Use a bowl at least twice the size of the pasta, because it foams up over the edge otherwise. Test a piece at 8 minutes - it goes from chalky to mushy quickly - then drain it straight away.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("boil"), py_str("make"), py_str("how do i"), py_str("how to")])), ("avoid".to_string(), g_NOT_A_METHOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("scrambled-eggs-in-the-microwave")); __args.push(py_tuple(vec![g_MICROWAVE(), py_tuple(vec![py_str("scrambled eggs"), py_str("scramble eggs"), py_str("scrambled egg"), py_str("scramble an egg"), py_str("eggs"), py_str("egg")])])); __args.push(py_str("Beat two or three eggs with a pinch of salt and a splash of milk in a mug or a small bowl, add a small knob of butter, and cook on full power in 30-second bursts, stirring properly between each one. Two eggs take about 90 seconds to two minutes in total, and they should come out still slightly wet - they carry on setting as you stir, and a microwave takes them from soft to rubbery in twenty seconds. Salt again at the end, not before. A whole egg in its shell is the one thing to keep out: it bursts, so if you want a fried egg on a plate, break it and prick the yolk first.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("scramble"), py_str("scrambled"), py_str("scrambling"), py_str("cook"), py_str("cooking"), py_str("make"), py_str("how do i"), py_str("how to")])), ("avoid".to_string(), op_add(&g_NOT_A_METHOD(), &py_tuple(vec![py_str("poach"), py_str("poached"), py_str("fried"), py_str("fry"), py_str("frying"), py_str("omelette"), py_str("scotch egg"), py_str("egg mayo")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("veg-in-the-microwave")); __args.push(py_tuple(vec![g_MICROWAVE(), __data2()])); __args.push(py_str("Put the veg in a microwave-safe bowl with two or three tablespoons of water, cover it with a plate or cling film with a corner turned back, and cook on full power: green beans and peas 3 to 4 minutes, broccoli and cauliflower florets 4 to 5, carrot batons 5 to 6, from raw. Stir or turn them over at halfway, and stand for a minute before you lift the cover, because the steam is hotter than boiling water. Stop while they are still bright and a knife goes in with a little resistance - they carry on cooking as they stand.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("steam"), py_str("steaming"), py_str("make"), py_str("how do i"), py_str("how to"), py_str("heat")])), ("avoid".to_string(), op_add(&g_NOT_A_METHOD(), &py_tuple(vec![py_str("roast"), py_str("roasting"), py_str("frozen veg"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("freeze"), py_str("freezing"), py_str("blanch"), py_str("boil them"), py_str("boiled")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.microwave_methods".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "MICROWAVE" => Some(g_MICROWAVE()),
            "NOT_A_METHOD" => Some(g_NOT_A_METHOD()),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "MICROWAVE" => s_MICROWAVE(v),
            "NOT_A_METHOD" => s_NOT_A_METHOD(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.microwave_methods", module_obj);
    pyrt::xrefs::reg_access("knowledge.microwave_methods", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.microwave_methods", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.microwave_methods", "MICROWAVE", g_MICROWAVE, s_MICROWAVE);
    pyrt::xrefs::reg_access("knowledge.microwave_methods", "NOT_A_METHOD", g_NOT_A_METHOD, s_NOT_A_METHOD);
    pyrt::xrefs::reg_access("knowledge.microwave_methods", "entry", g_entry, s_entry);
}

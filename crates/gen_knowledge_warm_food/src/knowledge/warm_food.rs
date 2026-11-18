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
thread_local! { static G__BREADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAD_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEFROST_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXERCISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FROM_FROZEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FROZEN_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_REHEATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_WARMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OVEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PASTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PASTRY_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REHEAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THAWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOASTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOAST_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WARM_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.warm_food: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__BREADS() -> Py { ensure_init(); G__BREADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREADS(v: Py) { ensure_init(); G__BREADS.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAD_REPLY() -> Py { ensure_init(); G__BREAD_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAD_REPLY(v: Py) { ensure_init(); G__BREAD_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__DEFROST_REPLY() -> Py { ensure_init(); G__DEFROST_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEFROST_REPLY(v: Py) { ensure_init(); G__DEFROST_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__EXERCISE() -> Py { ensure_init(); G__EXERCISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXERCISE(v: Py) { ensure_init(); G__EXERCISE.with(|c| *c.borrow_mut() = v); }
pub fn g__FROM_FROZEN() -> Py { ensure_init(); G__FROM_FROZEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FROM_FROZEN(v: Py) { ensure_init(); G__FROM_FROZEN.with(|c| *c.borrow_mut() = v); }
pub fn g__FROZEN_CLAUSE() -> Py { ensure_init(); G__FROZEN_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FROZEN_CLAUSE(v: Py) { ensure_init(); G__FROZEN_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__HEATING() -> Py { ensure_init(); G__HEATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEATING(v: Py) { ensure_init(); G__HEATING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_REHEATING() -> Py { ensure_init(); G__NOT_REHEATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_REHEATING(v: Py) { ensure_init(); G__NOT_REHEATING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_WARMING() -> Py { ensure_init(); G__NOT_WARMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_WARMING(v: Py) { ensure_init(); G__NOT_WARMING.with(|c| *c.borrow_mut() = v); }
pub fn g__OVEN_WORDS() -> Py { ensure_init(); G__OVEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OVEN_WORDS(v: Py) { ensure_init(); G__OVEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PASTRY() -> Py { ensure_init(); G__PASTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PASTRY(v: Py) { ensure_init(); G__PASTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__PASTRY_REPLY() -> Py { ensure_init(); G__PASTRY_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PASTRY_REPLY(v: Py) { ensure_init(); G__PASTRY_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__REHEAT_WORDS() -> Py { ensure_init(); G__REHEAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REHEAT_WORDS(v: Py) { ensure_init(); G__REHEAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THAWING() -> Py { ensure_init(); G__THAWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THAWING(v: Py) { ensure_init(); G__THAWING.with(|c| *c.borrow_mut() = v); }
pub fn g__TOASTING() -> Py { ensure_init(); G__TOASTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOASTING(v: Py) { ensure_init(); G__TOASTING.with(|c| *c.borrow_mut() = v); }
pub fn g__TOAST_REPLY() -> Py { ensure_init(); G__TOAST_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOAST_REPLY(v: Py) { ensure_init(); G__TOAST_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__WARM_WORDS() -> Py { ensure_init(); G__WARM_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WARM_WORDS(v: Py) { ensure_init(); G__WARM_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Bread and pastry warmed up: two answers a kitchen asks for in exactly the same words as a gym.\n\n\"how do i warm up naan\", \"how do i warm up a pitta\", \"how do i warm up garlic bread\" and \"how do\ni warm up a wrap\" all got the five-minute exercise warm-up, because \"warm up\" is the phrase that\nrow keys on and nothing in the line said the thing being warmed was food. \"how do i reheat a\nsausage roll\" got the rough puff recipe, \"how do i reheat a croissant\", a pasty, a samosa and a\nquiche were a shrug, and \"how do i reheat a pie\" got the answer for reheating a sauce - a splash\nof water, a lid on, a stir halfway.\n\nTwo rows, kept apart by the thing being heated: a bread wants three minutes and a splash of\nwater, a filled pastry wants eight to ten and a bare shelf. Both name the food, so the exercise\nrow's \"warm up\" cannot reach them, and both avoid the exercise words anyway, for the line that\nsays \"warm up my pitta after a run\".\n\nNot in cooking.py, which is past the 1 MB file wall.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("before a run"), py_str("before running"), py_str("before exercise"), py_str("before the gym"), py_str("for a run"), py_str("exercise"), py_str("gym"), py_str("workout"), py_str("stretches"), py_str("stretching"), py_str("jog"), py_str("jogging"), py_str("sprints"), py_str("injury"), py_str("injured"), py_str("match"), py_str("football"), py_str("netball"), py_str("training session"), py_str("cool down"), py_str("after a run"), py_str("muscles")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._EXERCISE"); }
    s__EXERCISE(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("naan"), py_str("naans"), py_str("naan bread"), py_str("pitta"), py_str("pita"), py_str("pitta bread"), py_str("pita bread"), py_str("tortilla"), py_str("tortillas"), py_str("wrap"), py_str("wraps"), py_str("flatbread"), py_str("flat bread"), py_str("chapati"), py_str("chapatis"), py_str("roti"), py_str("rotis"), py_str("baguette"), py_str("french bread"), py_str("bread roll"), py_str("bread rolls"), py_str("rolls"), py_str("bun"), py_str("buns"), py_str("bagel"), py_str("bagels"), py_str("ciabatta"), py_str("focaccia"), py_str("garlic bread"), py_str("bread"), py_str("pide"), py_str("scone"), py_str("scones"), py_str("crumpet"), py_str("crumpets"), py_str("teacake"), py_str("teacakes"), py_str("pancake"), py_str("pancakes"), py_str("waffle"), py_str("waffles")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._BREADS"); }
    s__BREADS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("warm up"), py_str("warm"), py_str("warming"), py_str("warm through"), py_str("reheat"), py_str("reheating"), py_str("heat up"), py_str("hot"), py_str("hot again"), py_str("crisp"), py_str("crispy"), py_str("crisp up"), py_str("soft again"), py_str("soften"), py_str("fresh again"), py_str("refresh"), py_str("not cold"), py_str("cold bread"), py_str("toaster"), py_str("in the toaster"), py_str("in a pan"), py_str("dry pan"), py_str("toast"), py_str("toasting")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._WARM_WORDS"); }
    s__WARM_WORDS(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("bake"), py_str("baking"), py_str("recipe"), py_str("dough"), py_str("sourdough"), py_str("yeast"), py_str("flour"), py_str("how long does"), py_str("keep"), py_str("keeps"), py_str("freeze"), py_str("freezing"), py_str("frozen"), py_str("stale"), py_str("stale bread"), py_str("bread sauce"), py_str("bread and butter pudding"), py_str("crumbs"), py_str("breadcrumbs"), py_str("stuffing"), py_str("make toast"), py_str("how do i make toast"), py_str("toast it"), py_str("beans on toast"), py_str("cheese on toast"), py_str("sandwich"), py_str("which bread"), py_str("healthiest"), py_str("gluten"), py_str("carbs"), py_str("calories"), py_str("how many calories"), py_str("buy"), py_str("best bread"), py_str("sliced")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._NOT_WARMING"); }
    s__NOT_WARMING(op_add(&g__EXERCISE(), &__data4()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._BREAD_REPLY"); }
    s__BREAD_REPLY(py_str("Splash it with a little water and give it 3 to 4 minutes in a 180C (160C fan) oven - 6 to 8 for a whole loaf - foil around it for a soft bread like naan or pitta, straight on the shelf for a crusty one. Twenty seconds in the microwave works if you eat it at once, and a wrap or pitta only wants 20 to 30 seconds a side in a dry hot pan or a toaster, with an air fryer doing the same job in 3 minutes at 160C."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._TOASTING"); }
    s__TOASTING(call_builtin("frozenset", &[py_tuple(vec![py_str("toast"), py_str("toasting"), py_str("toaster")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._OVEN_WORDS"); }
    s__OVEN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("oven"), py_str("bake"), py_str("baking"), py_str("warm"), py_str("warming"), py_str("reheat"), py_str("reheating"), py_str("heat"), py_str("grill")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._TOAST_REPLY"); }
    s__TOAST_REPLY(py_str("A toaster or a dry hot pan does it in 20 to 30 seconds a side - cut a pitta or a bagel open first, and watch it, because they go from pale to burnt in seconds. Under the grill it is a minute at most, turning once."));
    fn __data5() -> Py { py_tuple(vec![py_str("sausage roll"), py_str("sausage rolls"), py_str("pasty"), py_str("pastie"), py_str("pastries"), py_str("pasty"), py_str("samosa"), py_str("samosas"), py_str("spring roll"), py_str("spring rolls"), py_str("croissant"), py_str("croissants"), py_str("quiche"), py_str("quiches"), py_str("pie"), py_str("pies"), py_str("turnover"), py_str("turnovers"), py_str("pigs in blankets"), py_str("vol au vent"), py_str("vol au vents"), py_str("danish pastry"), py_str("pain au chocolat"), py_str("baklava"), py_str("empanada"), py_str("empanadas"), py_str("cornish pasty"), py_str("filo parcel"), py_str("filo parcels")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._PASTRY"); }
    s__PASTRY(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("reheat"), py_str("reheating"), py_str("heat up"), py_str("heating up"), py_str("warm up"), py_str("warm"), py_str("hot again"), py_str("cold"), py_str("microwave"), py_str("in the oven"), py_str("oven"), py_str("air fryer"), py_str("crisp"), py_str("crispy"), py_str("crisp up"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._REHEAT_WORDS"); }
    s__REHEAT_WORDS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("bake"), py_str("baking"), py_str("recipe"), py_str("dough"), py_str("rough puff"), py_str("shortcrust"), py_str("puff pastry"), py_str("filo"), py_str("pastry case"), py_str("blind bake"), py_str("blind baking"), py_str("line a tin"), py_str("roll out"), py_str("how long does"), py_str("keep"), py_str("keeps"), py_str("freeze"), py_str("freezing"), py_str("use by"), py_str("still ok"), py_str("safe to eat"), py_str("gone off"), py_str("how long can"), py_str("calories"), py_str("how many calories"), py_str("healthy"), py_str("buy"), py_str("where can i"), py_str("bakery"), py_str("supermarket"), py_str("filling"), py_str("what goes in"), py_str("base"), py_str("soggy base"), py_str("blind"), py_str("always"), py_str("how do i stop"), py_str("how do i stop that"), py_str("comes out"), py_str("mince"), py_str("steak and"), py_str("chicken pie"), py_str("apple pie"), py_str("shepherds pie"), py_str("cottage pie"), py_str("fish pie"), py_str("who makes"), py_str("how long do i cook"), py_str("how long to cook"), py_str("how long do you cook"), py_str("how long should i cook"), py_str("how long does it take to cook"), py_str("how long do i bake"), py_str("how long to bake"), py_str("how long should i bake"), py_str("how long does it take to bake")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._NOT_REHEATING"); }
    s__NOT_REHEATING(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._PASTRY_REPLY"); }
    s__PASTRY_REPLY(py_str("Give it 8 to 10 minutes at 180C (160C fan) on a bare shelf, or 4 to 5 minutes in an air fryer at 160C, and it is hot when a skewer into the middle comes out hot. A microwave heats the middle but leaves the pastry limp, so if you use one give it 2 minutes in the oven after to crisp the outside."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._FROM_FROZEN"); }
    s__FROM_FROZEN(call_builtin("frozenset", &[py_tuple(vec![py_str("frozen"), py_str("freezer"), py_str("defrost"), py_str("defrosting"), py_str("defrosted"), py_str("from frozen"), py_str("still frozen"), py_str("thaw"), py_str("thawing"), py_str("thawed")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._THAWING"); }
    s__THAWING(call_builtin("frozenset", &[py_tuple(vec![py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._HEATING"); }
    s__HEATING(call_builtin("frozenset", &[py_tuple(vec![py_str("reheat"), py_str("reheating"), py_str("heat up"), py_str("heating up"), py_str("warm up"), py_str("warm"), py_str("warming"), py_str("hot again")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._FROZEN_CLAUSE"); }
    s__FROZEN_CLAUSE(py_str(" From frozen it wants defrosting first - overnight in the fridge, or 25 to 30 minutes at 160C from frozen, then the 8 to 10 minutes at 180C to crisp it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food._DEFROST_REPLY"); }
    s__DEFROST_REPLY(py_str("Defrost it overnight in the fridge - a pie or quiche on a plate on the bottom shelf, still in its packaging - then give it 8 to 10 minutes at 180C (160C fan) on a bare shelf, or 4 to 5 minutes in an air fryer at 160C, to heat the middle through and crisp the pastry. Do not defrost one on the worktop."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.warm_food.ENTRIES"); }
    s_ENTRIES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("warm-bread")); __args.push(py_tuple(vec![g__BREADS(), g__WARM_WORDS()])); __args.push(py_func("_bread_reply", |args: &[Py]| _bread_reply(args))); __kws.extend(vec![("avoid".to_string(), g__NOT_WARMING()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reheat-pastry")); __args.push(py_tuple(vec![g__PASTRY(), g__REHEAT_WORDS()])); __args.push(py_func("_pastry_reply", |args: &[Py]| _pastry_reply(args))); __kws.extend(vec![("avoid".to_string(), g__NOT_REHEATING()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _bread_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The bread times, or the toaster ones when the line says toast.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g__TOASTING()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g__OVEN_WORDS()))) } }) {
    return g__TOAST_REPLY();
    }
    return g__BREAD_REPLY();
    Py::None
}
pub fn _pastry_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut cold: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The pastry times, leading with the defrost when that is what the line asked for.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    text = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    cold = { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &g__FROM_FROZEN())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("frozen"))) } };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g__THAWING()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g__HEATING()))) } }) {
    return g__DEFROST_REPLY();
    }
    if py_truthy(&cold.clone()) {
    return op_add(&g__PASTRY_REPLY(), &g__FROZEN_CLAUSE());
    }
    return g__PASTRY_REPLY();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.warm_food".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_BREADS" => Some(g__BREADS()),
            "_BREAD_REPLY" => Some(g__BREAD_REPLY()),
            "_DEFROST_REPLY" => Some(g__DEFROST_REPLY()),
            "_EXERCISE" => Some(g__EXERCISE()),
            "_FROM_FROZEN" => Some(g__FROM_FROZEN()),
            "_FROZEN_CLAUSE" => Some(g__FROZEN_CLAUSE()),
            "_HEATING" => Some(g__HEATING()),
            "_NOT_REHEATING" => Some(g__NOT_REHEATING()),
            "_NOT_WARMING" => Some(g__NOT_WARMING()),
            "_OVEN_WORDS" => Some(g__OVEN_WORDS()),
            "_PASTRY" => Some(g__PASTRY()),
            "_PASTRY_REPLY" => Some(g__PASTRY_REPLY()),
            "_REHEAT_WORDS" => Some(g__REHEAT_WORDS()),
            "_THAWING" => Some(g__THAWING()),
            "_TOASTING" => Some(g__TOASTING()),
            "_TOAST_REPLY" => Some(g__TOAST_REPLY()),
            "_WARM_WORDS" => Some(g__WARM_WORDS()),
            "_bread_reply" => Some(py_func("_bread_reply", |args: &[Py]| _bread_reply(args))),
            "_pastry_reply" => Some(py_func("_pastry_reply", |args: &[Py]| _pastry_reply(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_BREADS" => s__BREADS(v),
            "_BREAD_REPLY" => s__BREAD_REPLY(v),
            "_DEFROST_REPLY" => s__DEFROST_REPLY(v),
            "_EXERCISE" => s__EXERCISE(v),
            "_FROM_FROZEN" => s__FROM_FROZEN(v),
            "_FROZEN_CLAUSE" => s__FROZEN_CLAUSE(v),
            "_HEATING" => s__HEATING(v),
            "_NOT_REHEATING" => s__NOT_REHEATING(v),
            "_NOT_WARMING" => s__NOT_WARMING(v),
            "_OVEN_WORDS" => s__OVEN_WORDS(v),
            "_PASTRY" => s__PASTRY(v),
            "_PASTRY_REPLY" => s__PASTRY_REPLY(v),
            "_REHEAT_WORDS" => s__REHEAT_WORDS(v),
            "_THAWING" => s__THAWING(v),
            "_TOASTING" => s__TOASTING(v),
            "_TOAST_REPLY" => s__TOAST_REPLY(v),
            "_WARM_WORDS" => s__WARM_WORDS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.warm_food", module_obj);
    pyrt::xrefs::reg_fn("knowledge.warm_food", "_bread_reply", py_func("_bread_reply", |args: &[Py]| _bread_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.warm_food", "_pastry_reply", py_func("_pastry_reply", |args: &[Py]| _pastry_reply(args)));
    pyrt::xrefs::reg_access("knowledge.warm_food", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.warm_food", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_BREADS", g__BREADS, s__BREADS);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_BREAD_REPLY", g__BREAD_REPLY, s__BREAD_REPLY);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_DEFROST_REPLY", g__DEFROST_REPLY, s__DEFROST_REPLY);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_EXERCISE", g__EXERCISE, s__EXERCISE);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_FROM_FROZEN", g__FROM_FROZEN, s__FROM_FROZEN);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_FROZEN_CLAUSE", g__FROZEN_CLAUSE, s__FROZEN_CLAUSE);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_HEATING", g__HEATING, s__HEATING);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_NOT_REHEATING", g__NOT_REHEATING, s__NOT_REHEATING);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_NOT_WARMING", g__NOT_WARMING, s__NOT_WARMING);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_OVEN_WORDS", g__OVEN_WORDS, s__OVEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_PASTRY", g__PASTRY, s__PASTRY);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_PASTRY_REPLY", g__PASTRY_REPLY, s__PASTRY_REPLY);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_REHEAT_WORDS", g__REHEAT_WORDS, s__REHEAT_WORDS);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_THAWING", g__THAWING, s__THAWING);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_TOASTING", g__TOASTING, s__TOASTING);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_TOAST_REPLY", g__TOAST_REPLY, s__TOAST_REPLY);
    pyrt::xrefs::reg_access("knowledge.warm_food", "_WARM_WORDS", g__WARM_WORDS, s__WARM_WORDS);
    pyrt::xrefs::reg_access("knowledge.warm_food", "entry", g_entry, s_entry);
}

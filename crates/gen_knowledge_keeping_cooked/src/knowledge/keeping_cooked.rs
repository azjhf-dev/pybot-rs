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
thread_local! { static G_COOKED_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_KEEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_KEEP_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_SAFETY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_REHEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REHEAT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.keeping_cooked: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_KEEPS() -> Py { ensure_init(); G_COOKED_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_KEEPS(v: Py) { ensure_init(); G_COOKED_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_KEEP_ASK() -> Py { ensure_init(); G_COOKED_KEEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_KEEP_ASK(v: Py) { ensure_init(); G_COOKED_KEEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_KEEP_AVOID() -> Py { ensure_init(); G_COOKED_KEEP_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_KEEP_AVOID(v: Py) { ensure_init(); G_COOKED_KEEP_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_SAFETY() -> Py { ensure_init(); G_FOOD_SAFETY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_SAFETY(v: Py) { ensure_init(); G_FOOD_SAFETY.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_REHEAT() -> Py { ensure_init(); G_HOW_REHEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_REHEAT(v: Py) { ensure_init(); G_HOW_REHEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_WORDS() -> Py { ensure_init(); G_LEFT_OUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_WORDS(v: Py) { ensure_init(); G_LEFT_OUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REHEAT_AGAIN() -> Py { ensure_init(); G_REHEAT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REHEAT_AGAIN(v: Py) { ensure_init(); G_REHEAT_AGAIN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Keeping for food that is already cooked.\n\nA food that is already cooked keeps on a different clock from the same food raw, and the\ncooking-time entry for it owns every word of the question: \"how long does cooked bacon keep\" was\nanswered with the oven time, \"how long does cooked mince last\" with how to brown it, and the fish\nwith how long to bake it. One table, because the second half of every one of these questions is\nthe same question: how many days, in what, and how do I reheat it.\n\nThe rows are data only. bot/knowledge/cooking.py builds an Entry out of each one and appends it\nto its ENTRIES, so the table is still one table and the matcher, the ranking and the domain stay\ncooking's. They live here because cooking.py is close to the thousand-kilobyte file that the\nworld refuses to accept, and a keeping row per food is the part of it that grows.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("cooked"), py_str("cooked it"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("roast"), py_str("roasted"), py_str("fried"), py_str("boiled"), py_str("reheated"), py_str("reheat"), py_str("reheating"), py_str("made it"), py_str("made too much"), py_str("already cooked"), py_str("yesterday")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.COOKED_KEEP_ASK"); }
    s_COOKED_KEEP_ASK(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("left out"), py_str("left it out"), py_str("sat out"), py_str("sitting out"), py_str("left in the car"), py_str("left on the side"), py_str("overnight"), py_str("all night"), py_str("all day"), py_str("is it safe"), py_str("safe to eat"), py_str("still ok"), py_str("still okay"), py_str("still safe")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.LEFT_OUT_WORDS"); }
    s_LEFT_OUT_WORDS(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.REHEAT_AGAIN"); }
    s_REHEAT_AGAIN(py_tuple(vec![py_str("twice"), py_str("two times"), py_str("again"), py_str("second time"), py_str("how many times"), py_str("more than once"), py_str("a third time")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.FOOD_SAFETY"); }
    s_FOOD_SAFETY(py_tuple(vec![py_str("food poisoning"), py_str("poisoning"), py_str("poisoned"), py_str("food safety"), py_str("danger zone")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.HOW_REHEAT"); }
    s_HOW_REHEAT(py_tuple(vec![py_str("how do i reheat"), py_str("how to reheat"), py_str("how should i reheat"), py_str("how long do i reheat"), py_str("how do i warm"), py_str("how to warm"), py_str("how do i heat up"), py_str("how to heat up"), py_str("best way to reheat"), py_str("best way to warm"), py_str("what is the best way to reheat")]));
    fn __data3() -> Py { py_tuple(vec![py_str("cook it"), py_str("cook them"), py_str("cook it for"), py_str("cook them for"), py_str("boil it"), py_str("boil them"), py_str("bake it"), py_str("bake them"), py_str("roast it"), py_str("roast them"), py_str("fry it"), py_str("fry them"), py_str("simmer it"), py_str("simmer them"), py_str("how long do i cook"), py_str("how long to cook"), py_str("how long should i cook"), py_str("how long do i boil"), py_str("how long to boil"), py_str("how long do i bake"), py_str("how long do i reheat it for"), py_str("pressure cook"), py_str("juicy"), py_str("juicier"), py_str("moist"), py_str("moister"), py_str("tender"), py_str("tenderer"), py_str("succulent"), py_str("stays moist"), py_str("keep it moist"), py_str("not dry"), py_str("does not dry out"), py_str("crispy"), py_str("crisp"), py_str("crispier"), py_str("crunchy"), py_str("soggy"), py_str("limp"), py_str("goes soft"), py_str("gone soft"), py_str("not crispy"), py_str("wont crisp"), py_str("won't crisp")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.COOKED_KEEP_AVOID"); }
    s_COOKED_KEEP_AVOID(op_add(&op_add(&op_add(&op_add(&g_LEFT_OUT_WORDS(), &g_REHEAT_AGAIN()), &g_FOOD_SAFETY()), &g_HOW_REHEAT()), &__data3()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping_cooked.COOKED_KEEPS"); }
    s_COOKED_KEEPS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chicken breast"), py_str("chicken breasts"), py_str("chicken thighs"), py_str("turkey"), py_str("roast chicken")]), py_str("Cooked chicken or turkey keeps 3 days in the fridge in a covered box, and freezes for up to 3 months. Cool it within 2 hours of cooking, reheat it once until it is piping hot all through, and eat it cold only if it went straight into the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("mince"), py_str("minced beef"), py_str("mincemeat"), py_str("bolognese"), py_str("chilli"), py_str("cottage pie"), py_str("shepherds pie")]), py_str("Cooked mince keeps 3 days in the fridge in a sealed box, and freezes for 2 to 3 months. Reheat it once until it is piping hot right through, and give it 2 days rather than 3 if it was in a sauce with cream in it.")]), py_tuple(vec![py_tuple(vec![py_str("sausage"), py_str("sausages"), py_str("sausage meat")]), py_str("Cooked sausages keep 3 to 4 days in the fridge in a covered box, and freeze for 2 months. Reheat them in a pan or the oven until they are hot right through rather than in a microwave, which leaves the middle cool.")]), py_tuple(vec![py_tuple(vec![py_str("bacon"), py_str("bacon rashers"), py_str("streaky bacon"), py_str("back bacon")]), py_str("Cooked bacon keeps 4 to 5 days in the fridge, wrapped or in a box, and freezes for 2 months. It is already cured, so it lasts longer than other cooked meat: reheat it in a dry pan until it sizzles, and eat it cold within 3 days.")]), py_tuple(vec![py_tuple(vec![py_str("gammon"), py_str("ham"), py_str("ham joint"), py_str("roast pork"), py_str("pork")]), py_str("A cooked gammon or ham joint keeps 3 to 5 days in the fridge, and sliced ham 3 days once the pack is open. Wrap it so it does not dry out, keep it away from raw meat, and freeze slices for up to 2 months.")]), py_tuple(vec![py_tuple(vec![py_str("salmon"), py_str("fish"), py_str("cod"), py_str("haddock"), py_str("trout"), py_str("prawns")]), py_str("Cooked fish keeps 2 days in the fridge, less than meat: box it once it has cooled, and eat it cold the next day rather than reheating it, because fish goes dry and rubbery the second time. It freezes for a month if you cooked too much.")]), py_tuple(vec![py_tuple(vec![py_str("lentils"), py_str("beans"), py_str("chickpeas"), py_str("kidney beans"), py_str("baked beans"), py_str("dhal"), py_str("dal")]), py_str("Cooked beans, lentils and chickpeas keep 3 days in the fridge in a covered box, and freeze for 2 months in portions. Reheat them until they are hot right through, and bin any that smell sour or have gone fizzy.")]), py_tuple(vec![py_tuple(vec![py_str("beef"), py_str("roast beef"), py_str("lamb"), py_str("roast lamb"), py_str("steak"), py_str("pork chop")]), py_str("Roast meat keeps 3 to 4 days in the fridge, sliced and covered, and freezes for 2 to 3 months. Rare beef is at the shorter end: 3 days. Reheat it once in gravy so it does not dry out, and eat the rest cold.")])]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.keeping_cooked".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "COOKED_KEEPS" => Some(g_COOKED_KEEPS()),
            "COOKED_KEEP_ASK" => Some(g_COOKED_KEEP_ASK()),
            "COOKED_KEEP_AVOID" => Some(g_COOKED_KEEP_AVOID()),
            "FOOD_SAFETY" => Some(g_FOOD_SAFETY()),
            "HOW_REHEAT" => Some(g_HOW_REHEAT()),
            "LEFT_OUT_WORDS" => Some(g_LEFT_OUT_WORDS()),
            "REHEAT_AGAIN" => Some(g_REHEAT_AGAIN()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "COOKED_KEEPS" => s_COOKED_KEEPS(v),
            "COOKED_KEEP_ASK" => s_COOKED_KEEP_ASK(v),
            "COOKED_KEEP_AVOID" => s_COOKED_KEEP_AVOID(v),
            "FOOD_SAFETY" => s_FOOD_SAFETY(v),
            "HOW_REHEAT" => s_HOW_REHEAT(v),
            "LEFT_OUT_WORDS" => s_LEFT_OUT_WORDS(v),
            "REHEAT_AGAIN" => s_REHEAT_AGAIN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.keeping_cooked", module_obj);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "COOKED_KEEPS", g_COOKED_KEEPS, s_COOKED_KEEPS);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "COOKED_KEEP_ASK", g_COOKED_KEEP_ASK, s_COOKED_KEEP_ASK);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "COOKED_KEEP_AVOID", g_COOKED_KEEP_AVOID, s_COOKED_KEEP_AVOID);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "FOOD_SAFETY", g_FOOD_SAFETY, s_FOOD_SAFETY);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "HOW_REHEAT", g_HOW_REHEAT, s_HOW_REHEAT);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "LEFT_OUT_WORDS", g_LEFT_OUT_WORDS, s_LEFT_OUT_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping_cooked", "REHEAT_AGAIN", g_REHEAT_AGAIN, s_REHEAT_AGAIN);
}

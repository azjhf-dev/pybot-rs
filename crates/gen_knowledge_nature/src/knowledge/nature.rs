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


thread_local! { static G_AGGREGATE_BERRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BAD_NIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHAT_TO_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOTANICAL_FRUITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KITCHEN_FRUITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_PLANTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_MOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANT_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRODUCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYNODIC_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HARVEST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TREES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRUE_BERRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_MAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_BY_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_LONG_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_SCOTLAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_SHORT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_SUMMER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYLIGHT_WINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANTS_IN_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANT_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SETTING_CROPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.nature: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AGGREGATE_BERRIES() -> Py { ensure_init(); G_AGGREGATE_BERRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGGREGATE_BERRIES(v: Py) { ensure_init(); G_AGGREGATE_BERRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BAD_NIGHT() -> Py { ensure_init(); G_A_BAD_NIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BAD_NIGHT(v: Py) { ensure_init(); G_A_BAD_NIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHAT_TO_WEAR() -> Py { ensure_init(); G_A_WHAT_TO_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_TO_WEAR(v: Py) { ensure_init(); G_A_WHAT_TO_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_BOTANICAL_FRUITS() -> Py { ensure_init(); G_BOTANICAL_FRUITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOTANICAL_FRUITS(v: Py) { ensure_init(); G_BOTANICAL_FRUITS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_KITCHEN_FRUITS() -> Py { ensure_init(); G_KITCHEN_FRUITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KITCHEN_FRUITS(v: Py) { ensure_init(); G_KITCHEN_FRUITS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_PLANTS() -> Py { ensure_init(); G_NAMED_PLANTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_PLANTS(v: Py) { ensure_init(); G_NAMED_PLANTS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_MOON() -> Py { ensure_init(); G_NEW_MOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_MOON(v: Py) { ensure_init(); G_NEW_MOON.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANT_PARTS() -> Py { ensure_init(); G_PLANT_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANT_PARTS(v: Py) { ensure_init(); G_PLANT_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRODUCE() -> Py { ensure_init(); G_PRODUCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRODUCE(v: Py) { ensure_init(); G_PRODUCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SYNODIC_MONTH() -> Py { ensure_init(); G_SYNODIC_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYNODIC_MONTH(v: Py) { ensure_init(); G_SYNODIC_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HARVEST_ASK() -> Py { ensure_init(); G_THE_HARVEST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HARVEST_ASK(v: Py) { ensure_init(); G_THE_HARVEST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TREES() -> Py { ensure_init(); G_TREES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TREES(v: Py) { ensure_init(); G_TREES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRUE_BERRIES() -> Py { ensure_init(); G_TRUE_BERRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRUE_BERRIES(v: Py) { ensure_init(); G_TRUE_BERRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_MAKES() -> Py { ensure_init(); G_WHAT_MAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_MAKES(v: Py) { ensure_init(); G_WHAT_MAKES.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_BY_MONTH() -> Py { ensure_init(); G__DAYLIGHT_BY_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_BY_MONTH(v: Py) { ensure_init(); G__DAYLIGHT_BY_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_LONG_WORDS() -> Py { ensure_init(); G__DAYLIGHT_LONG_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_LONG_WORDS(v: Py) { ensure_init(); G__DAYLIGHT_LONG_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_SCOTLAND() -> Py { ensure_init(); G__DAYLIGHT_SCOTLAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_SCOTLAND(v: Py) { ensure_init(); G__DAYLIGHT_SCOTLAND.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_SHORT_WORDS() -> Py { ensure_init(); G__DAYLIGHT_SHORT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_SHORT_WORDS(v: Py) { ensure_init(); G__DAYLIGHT_SHORT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_SUMMER() -> Py { ensure_init(); G__DAYLIGHT_SUMMER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_SUMMER(v: Py) { ensure_init(); G__DAYLIGHT_SUMMER.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYLIGHT_WINTER() -> Py { ensure_init(); G__DAYLIGHT_WINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYLIGHT_WINTER(v: Py) { ensure_init(); G__DAYLIGHT_WINTER.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANTS_IN_HAND() -> Py { ensure_init(); G__PLANTS_IN_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANTS_IN_HAND(v: Py) { ensure_init(); G__PLANTS_IN_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANT_TROUBLE() -> Py { ensure_init(); G__PLANT_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANT_TROUBLE(v: Py) { ensure_init(); G__PLANT_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g__SETTING_CROPS() -> Py { ensure_init(); G__SETTING_CROPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SETTING_CROPS(v: Py) { ensure_init(); G__SETTING_CROPS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Nature and weather: what happens outside, and what to do about it.\n\nThe answers lane owns this domain. One Entry per everyday phenomenon or outdoors problem,\nkeyed by the words that identify it (\"leaves\", \"autumn\", \"colour\" rather than one sentence),\nso the same fact answers \"why do leaves change colour\", \"why do the leaves go brown in\nautumn\" and \"when do leaves turn\". Facts stay at school level and explain the cause in one\nline, then add the practical part when there is one.\n");
    s_A_WHAT_TO_WEAR(pyrt::xrefs::xget("knowledge.lookup", "A_WHAT_TO_WEAR"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("nature-and-weather")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.WHAT_MAKES"); }
    s_WHAT_MAKES(py_tuple(vec![py_str("what makes"), py_str("what causes"), py_str("what cause"), py_str("what made"), py_str("what makes it"), py_str("what causes it"), py_str("makes it"), py_str("causes it"), py_str("how come")]));
    fn __data1() -> Py { py_tuple(vec![py_str("cant sleep"), py_str("can't sleep"), py_str("cannot sleep"), py_str("couldnt sleep"), py_str("couldn't sleep"), py_str("wont sleep"), py_str("won't sleep"), py_str("not sleeping"), py_str("sleep badly"), py_str("sleeping badly"), py_str("slept badly"), py_str("keeps me awake"), py_str("keep me awake"), py_str("kept me awake"), py_str("awake all night"), py_str("toss and turn"), py_str("tossing and turning"), py_str("insomnia")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.A_BAD_NIGHT"); }
    s_A_BAD_NIGHT(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.NEW_MOON"); }
    s_NEW_MOON(stdlib_call("datetime", "datetime", &[Py::Int(2000), Py::Int(1), Py::Int(6), Py::Int(18), Py::Int(14)]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.SYNODIC_MONTH"); }
    s_SYNODIC_MONTH(Py::Float(29.530588853));
    fn __data2() -> Py { py_tuple(vec![py_str("basil"), py_str("hydrangea"), py_str("lavender"), py_str("tulip"), py_str("garlic"), py_str("monstera"), py_str("orchid"), py_str("peace lily"), py_str("snake plant"), py_str("spider plant"), py_str("ficus"), py_str("weeping fig"), py_str("fern"), py_str("succulent"), py_str("cactus"), py_str("cheese plant")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.NAMED_PLANTS"); }
    s_NAMED_PLANTS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("tree"), py_str("trees"), py_str("oak"), py_str("maple"), py_str("birch"), py_str("beech"), py_str("ash"), py_str("willow"), py_str("hedge"), py_str("shrub"), py_str("garden"), py_str("outside"), py_str("orchard"), py_str("woods"), py_str("woodland"), py_str("forest")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.TREES"); }
    s_TREES(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.BOTANICAL_FRUITS"); }
    s_BOTANICAL_FRUITS(py_dict(vec![(py_str("tomato"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("cucumber"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("courgette"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("zucchini"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("aubergine"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("eggplant"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("pepper"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("chilli"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("chili"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("pumpkin"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("squash"), py_str("fruit that grows from the flower and holds its seeds")), (py_str("avocado"), py_str("fruit that grows from the flower and holds its seeds"))]));
    fn __data4() -> Py { py_tuple(vec![py_str("banana"), py_str("tomato"), py_str("grape"), py_str("blueberry"), py_str("cucumber"), py_str("pepper"), py_str("chilli"), py_str("chili"), py_str("aubergine"), py_str("eggplant"), py_str("courgette"), py_str("zucchini"), py_str("pumpkin"), py_str("squash"), py_str("avocado"), py_str("lemon"), py_str("lime"), py_str("kiwi")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.TRUE_BERRIES"); }
    s_TRUE_BERRIES(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.AGGREGATE_BERRIES"); }
    s_AGGREGATE_BERRIES(call_builtin("frozenset", &[py_tuple(vec![py_str("strawberry"), py_str("raspberry"), py_str("blackberry")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.KITCHEN_FRUITS"); }
    s_KITCHEN_FRUITS(call_builtin("frozenset", &[py_tuple(vec![py_str("banana"), py_str("grape"), py_str("blueberry"), py_str("lemon"), py_str("lime"), py_str("kiwi"), py_str("cherry")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.PLANT_PARTS"); }
    s_PLANT_PARTS(py_dict(vec![(py_str("carrot"), py_str("a root")), (py_str("potato"), py_str("a tuber")), (py_str("onion"), py_str("a bulb")), (py_str("garlic"), py_str("a bulb")), (py_str("lettuce"), py_str("leaves")), (py_str("celery"), py_str("a stem")), (py_str("rhubarb"), py_str("a stem")), (py_str("broccoli"), py_str("a flower")), (py_str("cauliflower"), py_str("a flower"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.PRODUCE"); }
    s_PRODUCE(call_builtin("tuple", &[call_builtin("sorted", &[op_bitor(&op_bitor(&op_bitor(&op_bitor(&call_builtin("set", &[g_BOTANICAL_FRUITS()]).unwrap_or_else(|e| exc::raise(e)), &g_TRUE_BERRIES()), &g_AGGREGATE_BERRIES()), &g_KITCHEN_FRUITS()), &call_builtin("set", &[g_PLANT_PARTS()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("pepper"), py_str("peppers"), py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("tomato"), py_str("tomatoes"), py_str("aubergine"), py_str("aubergines"), py_str("eggplant"), py_str("eggplants"), py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("cucumber"), py_str("cucumbers"), py_str("squash"), py_str("pumpkin"), py_str("pumpkins"), py_str("strawberry"), py_str("strawberries"), py_str("bean"), py_str("beans"), py_str("pea"), py_str("peas"), py_str("melon"), py_str("melons"), py_str("raspberry"), py_str("raspberries"), py_str("blackberry"), py_str("blackberries")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._SETTING_CROPS"); }
    s__SETTING_CROPS(call_builtin("frozenset", &[__data5()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._PLANT_TROUBLE"); }
    s__PLANT_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|no|none|nothing|wont|won't|arent|aren't|isnt|isn't|hasnt|hasn't|havent|haven't|didnt|didn't|cant|can't|will not|drop|drops|dropping|dropped|fall|falls|falling|fell|small|tiny|shrivelled|bitter|yellow|brown|wilting|rot|rotten|split)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("still fruit"), py_str("will it fruit"), py_str("will they fruit"), py_str("will my fruit"), py_str("fruit this year"), py_str("still crop"), py_str("will it crop"), py_str("will they crop"), py_str("crop this year"), py_str("still produce"), py_str("more fruit"), py_str("set fruit"), py_str("setting fruit"), py_str("fruiting"), py_str("yield"), py_str("harvest"), py_str("how much fruit"), py_str("any fruit"), py_str("still get fruit"), py_str("get fruit this")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.THE_HARVEST_ASK"); }
    s_THE_HARVEST_ASK(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("pepper"), py_str("peppers"), py_str("courgette"), py_str("courgettes"), py_str("cucumber"), py_str("cucumbers"), py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("potato"), py_str("potatoes"), py_str("aubergine"), py_str("aubergines"), py_str("strawberry"), py_str("strawberries"), py_str("raspberry"), py_str("raspberries"), py_str("blueberry"), py_str("blueberries"), py_str("basil"), py_str("mint"), py_str("parsley"), py_str("coriander"), py_str("thyme"), py_str("rosemary"), py_str("sage"), py_str("chives"), py_str("oregano"), py_str("houseplant"), py_str("houseplants"), py_str("house plant"), py_str("house plants"), py_str("monstera"), py_str("orchid"), py_str("peace lily"), py_str("spider plant"), py_str("snake plant"), py_str("pothos"), py_str("aloe"), py_str("cactus"), py_str("succulent"), py_str("bonsai"), py_str("ficus"), py_str("fern"), py_str("seedling"), py_str("seedlings"), py_str("cutting"), py_str("cuttings"), py_str("my plant"), py_str("my plants"), py_str("veg"), py_str("vegetables"), py_str("allotment")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._PLANTS_IN_HAND"); }
    s__PLANTS_IN_HAND(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_SHORT_WORDS"); }
    s__DAYLIGHT_SHORT_WORDS(py_tuple(vec![py_str("december"), py_str("shorter"), py_str("shortest"), py_str("shorter days"), py_str("short days"), py_str("the days are short"), py_str("dark at"), py_str("4pm"), py_str("5pm"), py_str("21 december")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_LONG_WORDS"); }
    s__DAYLIGHT_LONG_WORDS(py_tuple(vec![py_str("summer"), py_str("june"), py_str("july"), py_str("august"), py_str("longer"), py_str("longest"), py_str("longer days"), py_str("midsummer"), py_str("solstice"), py_str("light nights"), py_str("never gets dark")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_WINTER"); }
    s__DAYLIGHT_WINTER(py_str("Daylight hours swing with the Earth's tilt: your half leans away from the Sun in winter, so it rises later, climbs less high and sets earlier, and the day is short. In the UK that is about 7 hours 45 minutes of daylight around 21 December, against 16 hours 45 minutes in June."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_SUMMER"); }
    s__DAYLIGHT_SUMMER(py_str("The same tilt the other way: your half leans towards the Sun in summer, so it rises earlier, climbs higher and sets later, and the day is long. In the UK that is about 16 hours 45 minutes of daylight around 21 June, against 7 hours 45 minutes in December."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_SCOTLAND"); }
    s__DAYLIGHT_SCOTLAND(py_str(" In the far north of Scotland around midsummer the sun only dips just below the horizon, so it never gets properly dark - the simmer dim - and Shetland's longest day is over 18 hours of light."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature._DAYLIGHT_BY_MONTH"); }
    s__DAYLIGHT_BY_MONTH(py_tuple(vec![py_tuple(vec![py_str("january"), Py::Int(8), Py::Int(10)]), py_tuple(vec![py_str("february"), Py::Int(9), Py::Int(55)]), py_tuple(vec![py_str("march"), Py::Int(11), Py::Int(55)]), py_tuple(vec![py_str("april"), Py::Int(14), Py::Int(0)]), py_tuple(vec![py_str("may"), Py::Int(15), Py::Int(55)]), py_tuple(vec![py_str("september"), Py::Int(12), Py::Int(30)]), py_tuple(vec![py_str("october"), Py::Int(10), Py::Int(30)]), py_tuple(vec![py_str("november"), Py::Int(8), Py::Int(40)])]));
    fn __data9() -> Py { py_tuple(vec![py_str("garden"), py_str("shed"), py_str("door"), py_str("window"), py_str("battle"), py_str("war"), py_str("beach"), py_str("shop"), py_str("queue"), py_str("house"), py_str("car"), py_str("row"), py_str("seat")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("washing"), py_str("laundry"), py_str("washing line"), py_str("pegs"), py_str("tumble dryer"), py_str("dryer"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("damp"), py_str("roof"), py_str("gutter"), py_str("guttering"), py_str("flood"), py_str("garden"), py_str("watering"), py_str("water the"), py_str("plants"), py_str("hose"), py_str("lawn"), py_str("grass seed"), py_str("flight"), py_str("flights"), py_str("airport"), py_str("match"), py_str("cricket"), py_str("football"), py_str("rugby"), py_str("tennis"), py_str("cancelled"), py_str("called off"), py_str("school"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("bored"), py_str("drive"), py_str("driving"), py_str("car"), py_str("bike"), py_str("cycle"), py_str("commute"), py_str("step"), py_str("steps"), py_str("how many"), py_str("walking"), py_str("steps a day")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("why"), py_str("form"), py_str("forms"), py_str("make"), py_str("makes"), py_str("made"), py_str("appear"), py_str("how come"), py_str("happen"), py_str("see"), py_str("colour"), py_str("colours"), py_str("cause"), py_str("causes"), py_str("what")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("feel windy"), py_str("feeling windy"), py_str("felt windy"), py_str("bloated"), py_str("bloating"), py_str("trapped wind"), py_str("fart"), py_str("farting"), py_str("gas"), py_str("my gut"), py_str("stomach"), py_str("tummy"), py_str("how windy")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("time"), py_str("times"), py_str("table"), py_str("tables"), py_str("timetable"), py_str("when"), py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("high tide"), py_str("low tide"), py_str("height"), py_str("how high"), py_str("how low"), py_str("across the causeway"), py_str("now"), py_str("walk"), py_str("crossing"), py_str("cross"), py_str("safe"), py_str("in or out"), py_str("coming in"), py_str("going out"), py_str("doing"), py_str("up or down"), py_str("high or low"), py_str("turn")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("why"), py_str("cause"), py_str("causes"), py_str("work"), py_str("works"), py_str("working"), py_str("moon"), py_str("sun"), py_str("gravity"), py_str("explain"), py_str("neap"), py_str("spring tide"), py_str("spring tides"), py_str("planet"), py_str("affect"), py_str("rock pool"), py_str("rock pools"), py_str("rockpool"), py_str("rockpools"), py_str("fishing"), py_str("fish")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("shorter"), py_str("short"), py_str("shortest"), py_str("longest"), py_str("longer"), py_str("hours"), py_str("early"), py_str("earlier"), py_str("get dark"), py_str("dark at"), py_str("sunset"), py_str("sunrise"), py_str("4pm"), py_str("5pm"), py_str("6pm"), py_str("december"), py_str("june"), py_str("winter"), py_str("summer")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("wash"), py_str("washing"), py_str("laundry"), py_str("load"), py_str("loads"), py_str("whites"), py_str("colours"), py_str("colors"), py_str("dark colours"), py_str("towels"), py_str("bedding"), py_str("sheets"), py_str("machine"), py_str("programme"), py_str("shirts"), py_str("denim"), py_str("wool"), py_str("silk"), py_str("bleach"), py_str("iron"), py_str("tumble"), py_str("dryer"), py_str("light bulb"), py_str("bulb"), py_str("lamp"), py_str("traffic light"), py_str("beer"), py_str("lager"), py_str("stout"), py_str("spice"), py_str("spices"), py_str("mars"), py_str("jupiter"), py_str("saturn"), py_str("venus"), py_str("mercury"), py_str("neptune"), py_str("uranus"), py_str("planet"), py_str("planets"), py_str("moon"), py_str("year"), py_str("years"), py_str("movie"), py_str("film"), py_str("book"), py_str("shift"), py_str("work"), py_str("here"), py_str("room"), py_str("rooms"), py_str("kitchen"), py_str("bedroom"), py_str("hallway"), py_str("outside"), py_str("screen"), py_str("phone"), py_str("camera"), py_str("photo"), py_str("photo is"), py_str("hair"), py_str("skin"), py_str("nail"), py_str("nails"), py_str("revision"), py_str("revise"), py_str("revising"), py_str("study"), py_str("studying"), py_str("studied"), py_str("coursework"), py_str("homework"), py_str("exam"), py_str("exams"), py_str("revised"), py_str("earth"), py_str("night"), py_str("nights"), py_str("tonight"), py_str("today"), py_str("tomorrow"), py_str("sky"), py_str("stars"), py_str("eclipse"), py_str("horizon"), py_str("eyes"), py_str("sleep"), py_str("blind"), py_str("circles"), py_str("bags"), py_str("under my eyes"), py_str("spots"), py_str("patches"), py_str("stains"), py_str("roots"), py_str("hair dye"), py_str("boiler"), py_str("radiator"), py_str("radiators"), py_str("thermostat"), py_str("central heating"), py_str("heating"), py_str("hot water"), py_str("bill"), py_str("bills"), py_str("gas bill"), py_str("electricity"), py_str("tariff"), py_str("immersion"), py_str("energy"), py_str("cheaper"), py_str("cost"), py_str("supplier"), py_str("meter"), py_str("standing charge"), py_str("double"), py_str("insulation"), py_str("loft"), py_str("draught"), py_str("draughts"), py_str("damp"), py_str("condensation"), py_str("mould"), py_str("thermals")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("daylight"), py_str("dark"), py_str("shorter"), py_str("shortest"), py_str("longer"), py_str("longest"), py_str("sun"), py_str("sunlight"), py_str("winter"), py_str("summer"), py_str("december"), py_str("june"), py_str("mars"), py_str("moon"), py_str("planet"), py_str("planets"), py_str("jupiter"), py_str("saturn"), py_str("venus"), py_str("mercury"), py_str("work"), py_str("shift"), py_str("school"), py_str("off"), py_str("holiday"), py_str("bad day"), py_str("good day"), py_str("one day"), py_str("someday"), py_str("birthday"), py_str("mothers day"), py_str("valentines"), py_str("pay"), py_str("sick"), py_str("feed"), py_str("walk"), py_str("hospital"), py_str("pole"), py_str("poles"), py_str("north pole"), py_str("south pole"), py_str("arctic"), py_str("antarctic"), py_str("desert"), py_str("space"), py_str("in space"), py_str("orbit"), py_str("station"), py_str("iss")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("boiler"), py_str("radiator"), py_str("radiators"), py_str("thermostat"), py_str("central heating"), py_str("heating"), py_str("hot water"), py_str("plumber"), py_str("radiator"), py_str("thermals"), py_str("coat"), py_str("dressing"), py_str("dress"), py_str("wear"), py_str("what should i"), py_str("i have a cold"), py_str("cold sore"), py_str("sore throat"), py_str("flu"), py_str("chest"), py_str("cough"), py_str("shiver"), py_str("chills"), py_str("freezing pipes"), py_str("frozen pipe"), py_str("fridge"), py_str("freezer"), py_str("cold brew"), py_str("cold water"), py_str("beer"), py_str("summer"), py_str("all the time"), py_str("always"), py_str("feel cold"), py_str("feeling cold"), py_str("feel the cold"), py_str("get cold"), py_str("getting cold"), py_str("hands"), py_str("feet"), py_str("toes"), py_str("fingers"), py_str("shivery"), py_str("shiver"), py_str("i am cold"), py_str("im cold"), py_str("i'm cold"), py_str("in bed"), py_str("bedroom"), py_str("house"), py_str("flat"), py_str("office"), py_str("coat"), py_str("jumper"), py_str("shed"), py_str("sheds"), py_str("cellar"), py_str("cellars"), py_str("basement"), py_str("basements"), py_str("workshop"), py_str("workshops"), py_str("garage"), py_str("garages"), py_str("conservatory"), py_str("loft"), py_str("attic"), py_str("hallway"), py_str("hall"), py_str("porch"), py_str("caravan"), py_str("bungalow"), py_str("cottage"), py_str("kitchen"), py_str("living room"), py_str("sitting room"), py_str("lounge"), py_str("room"), py_str("rooms"), py_str("home"), py_str("homes"), py_str("housemate"), py_str("outbuilding"), py_str("outbuildings"), py_str("greenhouse"), py_str("tent"), py_str("car"), py_str("van"), py_str("classroom"), py_str("morning"), py_str("mornings"), py_str("at night"), py_str("night"), py_str("nights"), py_str("desert"), py_str("the desert"), py_str("north pole"), py_str("south pole"), py_str("arctic"), py_str("antarctic"), py_str("so long"), py_str("how long"), py_str("mountain"), py_str("mountains"), py_str("mountain top"), py_str("hill"), py_str("hills"), py_str("altitude"), py_str("higher up"), py_str("at the top of"), py_str("summit"), py_str("sea"), py_str("ocean"), py_str("the sea"), py_str("the ocean"), py_str("seawater"), py_str("sea water"), py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("this week"), py_str("this morning"), py_str("right now"), py_str("at the moment"), py_str("forecast"), py_str("weather")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("time"), py_str("times"), py_str("table"), py_str("timetable"), py_str("when"), py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("height"), py_str("causeway"), py_str("doing"), py_str("swim"), py_str("swimming"), py_str("crossing")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("plant"), py_str("houseplant"), py_str("basil"), py_str("pot"), py_str("indoor"), py_str("tips"), py_str("house"), py_str("sick"), py_str("bonsai"), py_str("bonsais"), py_str("bonsai tree"), py_str("ficus"), py_str("my leaves"), py_str("my leaf"), py_str("my plant"), py_str("wrong"), py_str("mint"), py_str("herb"), py_str("herbs"), py_str("parsley"), py_str("coriander"), py_str("tomato"), py_str("spots"), py_str("curling"), py_str("wilting"), py_str("drooping"), py_str("not looking"), py_str("monstera"), py_str("orchid"), py_str("peace lily"), py_str("spider plant"), py_str("snake plant"), py_str("pothos"), py_str("aloe"), py_str("cactus"), py_str("succulent"), py_str("fiddle leaf"), py_str("potted"), py_str("spinach"), py_str("kale"), py_str("lettuce"), py_str("rocket"), py_str("greens"), py_str("cabbage"), py_str("chard"), py_str("eat"), py_str("eating"), py_str("ate"), py_str("bag of"), py_str("salad")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("my plant"), py_str("my plants"), py_str("my leaves"), py_str("the leaves"), py_str("my veg"), py_str("my vegetables"), py_str("my crops"), py_str("my flowers"), py_str("seedlings"), py_str("the plants"), py_str("the veg"), py_str("my basil"), py_str("my mint"), py_str("my parsley"), py_str("my coriander"), py_str("my chives"), py_str("my rosemary"), py_str("my thyme"), py_str("my tomatoes"), py_str("my tomato"), py_str("my lettuce"), py_str("my strawberries"), py_str("my herbs"), py_str("my herb"), py_str("my rose"), py_str("my roses"), py_str("my shrub"), py_str("my shrubs"), py_str("my tree"), py_str("my seedlings"), py_str("my cabbage"), py_str("my kale"), py_str("my courgettes"), py_str("my courgette"), py_str("my beans"), py_str("my peas"), py_str("my potatoes"), py_str("my carrots"), py_str("my hostas"), py_str("my hosta"), py_str("my lupins"), py_str("my sunflower"), py_str("my sunflowers")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("bird"), py_str("birds"), py_str("chicken"), py_str("chickens"), py_str("slug"), py_str("slugs"), py_str("snail"), py_str("snails"), py_str("caterpillar"), py_str("caterpillars"), py_str("aphid"), py_str("aphids"), py_str("pigeon"), py_str("pigeons"), py_str("rabbit"), py_str("rabbits"), py_str("deer"), py_str("mice"), py_str("mouse"), py_str("ants"), py_str("weevil"), py_str("beetle"), py_str("food"), py_str("photosynthesis"), py_str("sunlight"), py_str("drink"), py_str("eat them"), py_str("eat it"), py_str("toddler"), py_str("child"), py_str("children"), py_str("kids"), py_str("baby"), py_str("son"), py_str("daughter"), py_str("dinner"), py_str("lunch"), py_str("tea"), py_str("fussy"), py_str("appetite")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("bird food"), py_str("bird seed"), py_str("bird feeder"), py_str("feeder"), py_str("feeders"), py_str("bird table"), py_str("keep off"), py_str("keep them off"), py_str("stop"), py_str("stopping"), py_str("eating"), py_str("steal"), py_str("stealing"), py_str("get into"), py_str("get at"), py_str("raiding"), py_str("peanuts"), py_str("seed"), py_str("fat balls"), py_str("nuts")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("wasp"), py_str("bee"), py_str("swell"), py_str("swelling"), py_str("allergic"), py_str("throat"), py_str("hand"), py_str("hands"), py_str("foot"), py_str("feet"), py_str("leg"), py_str("legs"), py_str("arm"), py_str("arms"), py_str("finger"), py_str("fingers"), py_str("thumb"), py_str("wrist"), py_str("ankle"), py_str("toe"), py_str("toes"), py_str("palm"), py_str("cheek"), py_str("chin"), py_str("face"), py_str("neck"), py_str("shoulder"), py_str("ear"), py_str("ears"), py_str("lip"), py_str("lips"), py_str("eye"), py_str("eyelid"), py_str("head"), py_str("skin"), py_str("back"), py_str("knee"), py_str("knees"), py_str("tummy"), py_str("stomach")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("sing"), py_str("sings"), py_str("singing"), py_str("song"), py_str("chorus"), py_str("dawn"), py_str("morning"), py_str("4am"), py_str("4 am"), py_str("5am"), py_str("5 am"), py_str("early"), py_str("wake"), py_str("woken")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("feed"), py_str("feeder"), py_str("food"), py_str("feeding"), py_str("park"), py_str("city"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("brings in"), py_str("brings a"), py_str("bringing in"), py_str("brought in"), py_str("brought a"), py_str("keeps bringing"), py_str("prey"), py_str("dead"), py_str("mouse"), py_str("mice"), py_str("rat"), py_str("rats"), py_str("killed"), py_str("kills"), py_str("caught"), py_str("carcass"), py_str("carcasses")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("throat"), py_str("sore throat"), py_str("lump"), py_str("clicks"), py_str("clicking"), py_str("hurts to swallow"), py_str("trouble swallowing"), py_str("hard to swallow"), py_str("difficulty swallowing"), py_str("cannot swallow"), py_str("cant swallow"), py_str("can't swallow"), py_str("swallow food"), py_str("when i swallow"), py_str("painful to swallow"), py_str("tonsils")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("in the room"), py_str("in my room"), py_str("in here"), py_str("in the kitchen"), py_str("in the bedroom"), py_str("in the house"), py_str("in the bathroom"), py_str("in the living room"), py_str("indoors"), py_str("inside"), py_str("in the car"), py_str("in my kitchen"), py_str("in my bedroom"), py_str("in my house"), py_str("in my bathroom"), py_str("in my living room"), py_str("in my lounge"), py_str("in my bath"), py_str("in my car"), py_str("in the lounge"), py_str("in my office"), py_str("in the office"), py_str("out of my bathroom"), py_str("out of my bedroom"), py_str("out of my kitchen"), py_str("out of my house"), py_str("out of my car"), py_str("out of the lounge"), py_str("out of my lounge"), py_str("on me"), py_str("on my arm"), py_str("on my leg"), py_str("in the bath"), py_str("buzzing"), py_str("flying around"), py_str("trapped"), py_str("got in"), py_str("out of the bathroom"), py_str("out of the bedroom"), py_str("out of the kitchen"), py_str("out of the house"), py_str("out of my room"), py_str("out of the car"), py_str("out of here"), py_str("flew in"), py_str("came in"), py_str("in the window"), py_str("through the window")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("nest"), py_str("nests"), py_str("loft"), py_str("attic"), py_str("autumn"), py_str("september"), py_str("october"), py_str("swarm"), py_str("everywhere"), py_str("sting"), py_str("stung"), py_str("swell"), py_str("swelling"), py_str("allergic"), py_str("throat"), py_str("anaphyla"), py_str("epipen"), py_str("get rid of"), py_str("remove"), py_str("pest control"), py_str("queen")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("autumn"), py_str("indoors"), py_str("inside"), py_str("house"), py_str("come in"), py_str("coming in"), py_str("everywhere"), py_str("bath"), py_str("big"), py_str("huge"), py_str("rid"), py_str("stop"), py_str("keep out"), py_str("keeping out"), py_str("coming back"), py_str("afraid"), py_str("scared")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("change"), py_str("back"), py_str("forward"), py_str("darker"), py_str("lighter"), py_str("daylight saving"), py_str("daylight savings"), py_str("summer time"), py_str("bst"), py_str("nights"), py_str("point"), py_str("whats the point"), py_str("what is the point"), py_str("why"), py_str("reason"), py_str("still"), py_str("why bother"), py_str("who decides"), py_str("history")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("healthy"), py_str("healthier"), py_str("good for me"), py_str("bad for me"), py_str("calories"), py_str("carb"), py_str("carbs"), py_str("sugar"), py_str("diet"), py_str("weight"), py_str("buy"), py_str("shop"), py_str("shopping"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("plants"), py_str("allergy"), py_str("allergic"), py_str("wont fruit"), py_str("wont flower"), py_str("will not fruit"), py_str("will not flower"), py_str("not fruiting"), py_str("arent fruiting"), py_str("aren't fruiting"), py_str("not setting"), py_str("arent setting"), py_str("aren't setting"), py_str("not producing fruit"), py_str("no fruit"), py_str("havent set"), py_str("hasnt set"), py_str("didnt set"), py_str("set nothing"), py_str("setting nothing"), py_str("flowers drop"), py_str("flowers keep dropping"), py_str("flower drop"), py_str("flowers fall off"), py_str("dropping its flowers"), py_str("dropping their flowers"), py_str("flowers but no"), py_str("flowering but"), py_str("no flowers")]) }
    fn __data8() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("weather-front-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("front"), py_str("fronts")]), py_tuple(vec![py_str("weather"), py_str("warm"), py_str("cold"), py_str("occluded"), py_str("stationary")])])); __args.push(py_str("A front is the boundary where a warm air mass meets a cold one. On a cold front the cold air pushes under the warm: heavy rain in a narrow band, then a sharp drop in temperature. On a warm front the warm air climbs over the cold: thickening cloud, drizzle, and it turns milder. The rain comes at the front and the clearing is behind it.")); __kws.extend(vec![("avoid".to_string(), __data9())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("storm-distance"), py_tuple(vec![py_tuple(vec![py_str("thunder"), py_str("lightning"), py_str("storm")]), py_tuple(vec![py_str("far"), py_str("distance"), py_str("away"), py_str("how"), py_str("count"), py_str("hear"), py_str("close")])]), py_str("Count the seconds between the lightning and the thunder: every 3 seconds is about 1 km (5 seconds per mile). If the gap is under 30 seconds, the storm is close enough to be dangerous, so get indoors.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("hurricane-or-tornado"), py_tuple(vec![py_tuple(vec![py_str("hurricane"), py_str("hurricanes"), py_str("typhoon"), py_str("cyclone"), py_str("tornado"), py_str("tornadoes"), py_str("twister")]), py_tuple(vec![py_str("difference"), py_str("differ"), py_str("different"), py_str("vs"), py_str("versus"), py_str("worse"), py_str("stronger"), py_str("same"), py_str("which"), py_str("what"), py_str("how"), py_str("compare")])]), py_str("A hurricane is a whole storm system hundreds of kilometres across, born over warm sea and lasting days, with winds over 119 km/h. A tornado is a narrow funnel of spinning air under a thunderstorm, usually tens of metres wide and gone in minutes. Hurricanes are forecast days ahead; tornadoes give minutes.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("thunderstorm-outside"), py_tuple(vec![py_tuple(vec![py_str("thunderstorm"), py_str("storm")]), py_tuple(vec![py_str("outside"), py_str("caught"), py_str("open"), py_str("safe"), py_str("shelter"), py_str("should"), py_str("what"), py_str("do")])]), py_str("Get into a building or a hard-topped car, away from windows, and stay there 30 minutes after the last thunder. If you're caught in the open, move off high ground and away from lone trees, water and metal fences, and crouch low on the balls of your feet.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("flood-warning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flood"), py_str("flooding")]), py_tuple(vec![py_str("warning"), py_str("what"), py_str("should"), py_str("do"), py_str("home"), py_str("safe"), py_str("water")])])); __args.push(py_str("Move valuables and medicines upstairs, unplug what you can safely reach and stay out of the water: 30 cm of moving water can float a car. Follow your local flood service, and if they tell you to leave, go early rather than wait to see how high it gets.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("email"), py_str("emails"), py_str("inbox"), py_str("newsletter"), py_str("newsletters"), py_str("post"), py_str("letters"), py_str("messages"), py_str("notifications"), py_str("comments")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("heatwave"), py_tuple(vec![py_tuple(vec![py_str("heatwave"), py_str("heat wave")]), py_tuple(vec![py_str("what"), py_str("do"), py_str("safe"), py_str("hot"), py_str("keep"), py_str("cool")])]), py_str("Keep the sun out during the day, open windows at night, drink water before you feel thirsty and check on anyone elderly or unwell nearby. Avoid heavy work between 11am and 3pm, when the sun is strongest.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("forecast-wrong"), py_tuple(vec![py_tuple(vec![py_str("forecast"), py_str("weather forecast"), py_str("forecasts")]), py_tuple(vec![py_str("wrong"), py_str("often"), py_str("why"), py_str("accurate"), py_str("reliable"), py_str("right")])]), py_str("Weather is chaotic: a tiny error in today's readings grows over days, so a forecast a week ahead is a good guess and tomorrow's is usually close. Showers are also small enough to miss your street while soaking the next town, which is why a 30% chance of rain still means a wet walk sometimes.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wet-day-on-a-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("if it rains"), py_str("if it rains all week"), py_str("if it rains every day"), py_str("if its raining"), py_str("if it's raining"), py_str("if it rains a lot"), py_str("if it pours"), py_str("if the weather turns"), py_str("if the weather is bad"), py_str("rains all week"), py_str("raining all week"), py_str("wet all week")]), py_tuple(vec![py_str("what"), py_str("should"), py_str("shall"), py_str("can"), py_str("could"), py_str("do"), py_str("will"), py_str("if"), py_str("and"), py_str("plan")])])); __args.push(py_func("_the_wet_day", |args: &[Py]| _the_wet_day(args))); __kws.extend(vec![("avoid".to_string(), __data10())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rain-why")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rain"), py_str("rains"), py_str("raining"), py_str("raining so much")]), op_add(&py_tuple(vec![py_str("why"), py_str("does"), py_str("form"), py_str("forms"), py_str("falls"), py_str("come"), py_str("cloud"), py_str("what"), py_str("what is")]), &g_WHAT_MAKES())])); __args.push(py_str("The sun evaporates water, it rises and cools into cloud droplets, and the droplets bump together until they're heavy enough to fall. Rain is just that water coming back down, usually after a cloud has grown thick and tall.")); __kws.extend(vec![("avoid".to_string(), g_A_WHAT_TO_WEAR())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("sunset-orange"), py_tuple(vec![py_str("sunset"), py_tuple(vec![py_str("orange"), py_str("red"), py_str("sky"), py_str("why"), py_str("sunrise"), py_str("evening")])]), py_str("Low sun sends its light through far more atmosphere, which scatters the blues away and leaves the reds and oranges to reach you. Dust and cloud make it stronger, which is why the best sunsets come after a clear, dusty day.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("rainbow"), py_tuple(vec![py_str("rainbow"), __data11()]), py_str("Sunlight bends as it enters raindrops, reflects off the back and bends again on the way out, which splits it into colours. You only see one with the sun behind you and rain in front, and the red band is always on the outside of the arc.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fog")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fog"), py_str("foggy"), py_str("mist"), py_str("misty")]), op_add(&py_tuple(vec![py_str("why"), py_str("form"), py_str("forms"), py_str("does"), py_str("drive"), py_str("driving"), py_str("safe"), py_str("thick"), py_str("what"), py_str("what is")]), &g_WHAT_MAKES())])); __args.push(py_str("Fog forms when damp air cools to the point where its water vapour condenses into droplets near the ground. Driving in it, slow down and use dipped headlights or fog lights, not full beam: full beam bounces off the fog and makes it worse.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("glasses"), py_str("spectacles"), py_str("specs"), py_str("goggles"), py_str("lenses"), py_str("lens"), py_str("visor"), py_str("mask"), py_str("mirror"), py_str("windscreen"), py_str("windshield"), py_str("bathroom")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("clouds"), py_tuple(vec![py_str("cloud"), op_add(&py_tuple(vec![py_str("what"), py_str("made"), py_str("form"), py_str("forms"), py_str("why"), py_str("white"), py_str("fluffy")]), &g_WHAT_MAKES())]), py_str("Clouds are tiny water droplets or ice crystals hanging on specks of dust, formed when rising damp air cools. They look white because the droplets scatter all colours of sunlight equally; they turn grey when they're thick enough to block the light.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("snow")); __args.push(py_tuple(vec![py_tuple(vec![py_str("snow"), py_str("snows"), py_str("snowing")]), op_add(&py_tuple(vec![py_str("why"), py_str("form"), py_str("forms"), py_str("does"), py_str("made"), py_str("fall"), py_str("falls"), py_str("happen"), py_str("flake"), py_str("what"), py_str("what is")]), &g_WHAT_MAKES())])); __args.push(py_str("Snow forms when water vapour freezes straight into ice crystals high in a cloud, and the crystals stick together as they fall. It only reaches the ground as snow if the air stays cold all the way down, which is why a cold snap often brings sleet instead.")); __kws.extend(vec![("avoid".to_string(), g_A_WHAT_TO_WEAR())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wind")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wind"), py_str("winds"), py_str("windy"), py_str("breezy"), py_str("gale"), py_str("gales")]), op_add(&py_tuple(vec![py_str("why"), py_str("blow"), py_str("blows"), py_str("does"), py_str("form"), py_str("cause"), py_str("windy"), py_str("so windy"), py_str("what"), py_str("what is")]), &g_WHAT_MAKES())])); __args.push(py_str("Wind is air moving from an area of high pressure to one of low pressure, and the sun creates those differences by heating land and sea unevenly. The bigger the pressure gap, the stronger the wind.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data12(), &g_A_WHAT_TO_WEAR()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("seasons"), py_tuple(vec![py_tuple(vec![py_str("season"), py_str("seasons"), py_str("why do we have seasons"), py_str("why does the earth have seasons"), py_str("what causes the seasons"), py_str("explain the seasons"), py_str("why do we get seasons")]), py_tuple(vec![py_str("why"), py_str("change"), py_str("summer"), py_str("winter"), py_str("cause"), py_str("earth"), py_str("tilt"), py_str("have"), py_str("happen"), py_str("seasons")])]), py_str("Seasons come from the Earth's 23.5-degree tilt, not from how close it is to the sun. When your half of the planet leans towards the sun you get summer: longer days and more direct light.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tide-times")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tide"), py_str("tides"), py_str("tidal"), py_str("high water"), py_str("low water"), py_str("causeway"), py_str("tidal road"), py_str("tidal causeway")]), __data13()])); __args.push(py_func("_the_tide", |args: &[Py]| _the_tide(args))); __kws.extend(vec![("avoid".to_string(), __data14()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("winter-daylight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("daylight"), py_str("days"), py_str("day"), py_str("dark"), py_str("light"), py_str("sunlight"), py_str("hours of light")]), __data15()])); __args.push(py_func("_daylight", |args: &[Py]| _daylight(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data16())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("length-of-a-day")); __args.push(py_tuple(vec![py_tuple(vec![py_str("day"), py_str("days"), py_str("earth day")]), py_tuple(vec![py_str("how long"), py_str("length"), py_str("how many hours"), py_str("hours long"), py_str("24 hours")])])); __args.push(py_str("A day is 24 hours - the time from one noon to the next. One full turn against the stars takes 23 hours 56 minutes; the extra four minutes are because the Earth has moved on along its orbit as it turned.")); __kws.extend(vec![("avoid".to_string(), __data17())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("winter-cold")); __args.push(py_tuple(vec![py_tuple(vec![py_str("winter"), py_str("colder"), py_str("cold"), py_str("coldest")]), py_tuple(vec![py_str("why"), py_str("so cold"), py_str("colder"), py_str("coldest"), py_str("than summer"), py_str("than june"), py_str("more"), py_str("cause"), py_str("explain"), py_str("feels")])])); __args.push(py_str("Winter is cold because your half of the Earth leans away from the Sun: the sunlight comes in at a low angle and is spread over more ground, and the shorter days give the ground less time to warm up. That is also why the winter sun feels weak even on a clear day.")); __kws.extend(vec![("avoid".to_string(), __data18())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tides")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tide"), py_str("tides")]), py_tuple(vec![py_str("why"), py_str("moon"), py_str("sun"), py_str("cause"), py_str("causes"), py_str("high"), py_str("low"), py_str("rise"), py_str("work"), py_str("works"), py_str("how"), py_str("what")])])); __args.push(py_str("The Moon's gravity pulls the sea towards it, making a bulge on the near side and a matching one on the far side, and the Earth turns through both. That gives two high tides roughly 12 hours 25 minutes apart, and the Sun's pull makes spring tides bigger at full and new moon.")); __kws.extend(vec![("avoid".to_string(), __data19())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("northern-lights"), py_tuple(vec![py_tuple(vec![py_str("northern lights"), py_str("aurora"), py_str("aurora borealis")]), py_tuple(vec![py_str("why"), py_str("what"), py_str("cause"), py_str("see"), py_str("where"), py_str("when"), py_str("happen")])]), py_str("Charged particles from the sun hit gases high in the atmosphere and make them glow: green from oxygen, reds and purples from nitrogen. You need a dark, clear night and a high latitude, so northern Scotland, Iceland and Scandinavia are the usual spots.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leaves-autumn")); __args.push(py_tuple(vec![py_tuple(vec![py_str("leaf"), py_str("leaves")]), py_tuple(vec![py_str("change"), py_str("colour"), py_str("color"), py_str("autumn"), py_str("fall"), py_str("turn"), py_str("brown"), py_str("red"), py_str("yellow"), py_str("lose"), py_str("drop"), py_str("tree")])])); __args.push(py_func("_the_autumn_leaves", |args: &[Py]| _the_autumn_leaves(args))); __kws.extend(vec![("avoid".to_string(), op_add(&__data20(), &g_NAMED_PLANTS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pests-eating-plants")); __args.push(py_tuple(vec![__data21(), py_tuple(vec![py_str("eating"), py_str("eaten"), py_str("nibbled"), py_str("nibbling"), py_str("chewed"), py_str("holes"), py_str("bites"), py_str("destroyed"), py_str("vanished")])])); __args.push(py_str("Look at when and how it is eaten. Ragged holes and slime trails, worst overnight, are slugs and snails: go out with a torch after dark and pick them off, or set a beer trap, and put copper tape round pots. Clean holes in the middle of leaves with the edges left alone are caterpillars - look under the leaves in daylight and pick them off. Tops nibbled to a stump with a clean cut are pigeons or rabbits, so net or cage it. Seedlings cut off at soil level are cutworms or mice.")); __kws.extend(vec![("avoid".to_string(), __data22())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("city-birds-winter")); __args.push(py_tuple(vec![py_str("bird"), py_tuple(vec![py_str("winter"), py_str("park"), py_str("city"), py_str("outside")])])); __args.push(py_str("In a British city park in winter you'll usually see robins, blackbirds, blue tits, great tits, pigeons, crows and magpies, with gulls and starlings on the grass. Look in the bushes and bare trees rather than the open lawn, and early morning is the busiest time.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("feed"), py_str("feeder"), py_str("food"), py_str("feeding")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("feed-birds-winter")); __args.push(py_tuple(vec![py_str("bird"), py_tuple(vec![py_str("feed"), py_str("feeder"), py_str("food"), py_str("feeding"), py_str("seed"), py_str("seeds"), py_str("fat balls"), py_str("mealworms"), py_str("bird table")])])); __args.push(py_str("Feed birds seeds, fat balls and mealworms, and put out water too; skip bread, which fills them up without much goodness. Once you start in winter, keep going, because they come to rely on that spot.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("see"), py_str("seen"), py_str("might i see"), py_str("which birds"), py_str("what birds"), py_str("spot"), py_str("spotted"), py_str("identify"), py_str("park"), py_str("city"), py_str("in the park")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("squirrels-on-the-feeder")); __args.push(py_tuple(vec![py_tuple(vec![py_str("squirrel"), py_str("squirrels"), py_str("grey squirrel"), py_str("grey squirrels")]), __data23()])); __args.push(py_str("A baffle is the whole answer: one on the pole under the feeder, or a set of four plastic baffles on the pole, which a squirrel cannot get past because it climbs with a grip. Then give it nowhere to launch from - 2m from a fence, a tree or a shed and 2m off the ground, because a grey squirrel jumps about 2m across and drops further. A caged feeder lets tits in and keeps the rest out, a weight-triggered one shuts under a squirrel, and chilli powder mixed into the seed is ignored by birds and hated by squirrels.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("what does a squirrel eat"), py_str("what do squirrels eat"), py_str("what does a grey squirrel eat"), py_str("diet"), py_str("what to feed"), py_str("what should i feed")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dogs-thunder")); __args.push(py_tuple(vec![py_str("dog"), py_tuple(vec![py_str("thunder"), py_str("thunderstorms"), py_str("storms")])])); __args.push(py_str("Dogs hear thunder much more loudly than we do and feel the pressure change, so the fear is real. Give them a small den to hide in, stay calm and don't force them out; background noise and closing the curtains help, and a vet can suggest calming options before the next storm.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dogs-fireworks")); __args.push(py_tuple(vec![py_str("dog"), py_tuple(vec![py_str("firework"), py_str("fireworks"), py_str("bonfire"), py_str("bonfire night"), py_str("guy fawkes"), py_str("sparklers"), py_str("bangs"), py_str("bang"), py_str("loud noises"), py_str("loud noise"), py_str("loud bangs"), py_str("explosions")])])); __args.push(py_str("Dogs hear bangs far louder than we do and can't tell they are harmless, so plan the night instead of waiting for the first one: walk him before dark, then shut the windows and curtains, put the radio on and let him hide where he likes with something to chew. Stay in with him, don't take him to a display and never tell a frightened dog off; if he panics badly, ask the vet about a calming plan a few weeks ahead and check his microchip details are up to date in case he bolts.")); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bees-important")); __args.push(py_tuple(vec![py_str("bee"), py_tuple(vec![py_str("important"), py_str("why"), py_str("do"), py_str("disappear"), py_str("dying"), py_str("pollinate")])])); __args.push(py_str("Bees pollinate a large share of the fruit, vegetables and nuts we eat, so losing them would make food scarcer and dearer. You can help with bee-friendly flowers, a shallow dish of water and a corner of garden left a bit wild.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("sting"), py_str("stung"), py_str("swell"), py_str("swelling"), py_str("throat"), py_str("allergic")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("tick-bite"), py_tuple(vec![py_str("tick"), py_tuple(vec![py_str("bite"), py_str("remove"), py_str("lyme"), py_str("safe"), py_str("found"), py_str("stuck")])]), py_str("Grip the tick as close to your skin as you can with fine tweezers and pull steadily upwards without twisting or squeezing. Then watch the spot: a spreading rash or a fever in the next few weeks means see a doctor and mention the bite.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("sting"), py_tuple(vec![py_tuple(vec![py_str("sting"), py_str("stung"), py_str("beesting"), py_str("beestings"), py_str("waspsting"), py_str("waspstings")]), __data24()]), py_str("Scrape the sting out sideways with a fingernail or card, wash it and put something cold on it; an antihistamine helps the itch. Get help fast if the person swells away from the sting, struggles to breathe or feels faint.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dawn-chorus")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bird"), py_str("birds"), py_str("blackbird"), py_str("thrush"), py_str("robin")]), __data25()])); __args.push(py_str("That is the dawn chorus: the blackbird starts about an hour before sunrise, then the robin and song thrush, then blue tits and the rest. The males are claiming territory and calling for a mate, and the still morning air carries the song further. It fades by about 7am, once they stop defending and start feeding.")); __kws.extend(vec![("avoid".to_string(), __data26())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("swallows-migrate")); __args.push(py_tuple(vec![py_tuple(vec![py_str("swallow"), py_str("swallows"), py_str("swift"), py_str("swifts"), py_str("martin"), py_str("martins")]), py_tuple(vec![py_str("arrive"), py_str("arriving"), py_str("arrived"), py_str("come"), py_str("back"), py_str("leave"), py_str("left"), py_str("gone"), py_str("when"), py_str("migrate"), py_str("migration"), py_str("nest")])])); __args.push(py_str("Swallows come back to Britain from mid-April, a few as early as March, and leave between late September and early October; swifts arrive in early May and are gone by mid-August. They winter in southern Africa. So an empty nest in early October is normal, not a sign that something went wrong.")); __kws.extend(vec![("avoid".to_string(), __data27())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wasps-autumn")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wasp"), py_str("wasps"), py_str("hornet"), py_str("hornets")]), py_tuple(vec![py_str("autumn"), py_str("september"), py_str("october"), py_str("everywhere"), py_str("bothering"), py_str("mean"), py_str("aggressive"), py_str("swarming"), py_str("nest"), py_str("many")])])); __args.push(py_str("In autumn the colony is winding down: the queen stops laying, so the workers have no grubs to feed and go looking for sugar instead, which is why they crowd round jam, fruit and bins in September. They are not nastier, just hungry and idle. They die at the first hard frosts, only the new queen survives, and the old nest is never reused.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("sting"), py_str("stung"), py_str("swell"), py_str("swelling"), py_str("allergic"), py_str("throat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wasp-in-the-room")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wasp"), py_str("wasps"), py_str("hornet"), py_str("hornets")]), __data28()])); __args.push(py_str("A wasp or a hornet in a room: open a window wide and step back - it flies to the light and goes out on its own, and it will not chase you. Do not swat it - a squashed wasp is what stings, and the ones in late summer are only after sugar, so leave a jam jar away from you and it will go there instead. If it is in the car, pull over somewhere safe, open every window and step out until it leaves; a wasp in a moving car is how people crash.")); __kws.extend(vec![("avoid".to_string(), __data29())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spring-blossom")); __args.push(py_tuple(vec![py_tuple(vec![py_str("blossom"), py_str("tree"), py_str("hedge")]), py_tuple(vec![py_str("white"), py_str("pink"), py_str("march"), py_str("april"), py_str("spring"), py_str("blackthorn")])])); __args.push(py_str("Blackthorn is the first: white blossom on bare, spiny hedgerow twigs in March, before its leaves. Cherry plum and wild cherry come next, then apple and hawthorn in May. White flowers on a leafless hedge in early spring is almost always blackthorn; blackthorn has thorny twigs and hawthorn flowers after its leaves are out.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("plant"), py_str("planting"), py_str("planted"), py_str("sow"), py_str("sowing"), py_str("seed"), py_str("seeds"), py_str("soil"), py_str("compost"), py_str("prune")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spiders-indoors")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spider"), py_str("spiders")]), __data30()])); __args.push(py_str("Autumn is when the male house spiders wander about looking for a mate, and a warm room is where the insects are. British house spiders are harmless: a glass and a piece of card is the kindest way to move one out. The cobwebs up in the ceiling corners are worth leaving, since they catch flies and midges.")); __kws.extend(vec![("report".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("clocks-change"), py_tuple(vec![py_tuple(vec![py_str("clocks"), py_str("clock"), py_str("daylight saving"), py_str("daylight savings"), py_str("summer time"), py_str("bst")]), __data31()]), py_str("The clocks go back an hour at 2am on the last Sunday in October and forward an hour at 2am on the last Sunday in March. The point of it is the light: the hour is moved to the part of the day when most people are up and out, so summer evenings stay light and winter mornings are not dark until nine. Late October buys an extra hour in bed, and the evenings close in by about 5pm; late March hands the hour back.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("garden-birds-gone")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bird"), py_str("birds")]), py_tuple(vec![py_str("gone"), py_str("no birds"), py_str("missing"), py_str("disappeared"), py_str("empty"), py_str("stopped"), py_str("not coming"), py_str("vanished"), py_str("none"), py_str("quiet")])])); __args.push(py_str("The usual reasons are timing and cover. Birds go quiet and hide in late summer while they moult, and a garden with no shrubs or trees has nowhere safe to wait. A cat, or a sparrowhawk that has found your feeders, will clear a garden for a few weeks. Keep the feeders topped up with water nearby, add a leafy shrub or two, and give it a fortnight.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("park"), py_str("city"), py_str("winter"), py_str("feed"), py_str("feeder"), py_str("food")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("butterflies-garden"), py_tuple(vec![py_tuple(vec![py_str("butterfly"), py_str("butterflies")]), py_tuple(vec![py_str("garden"), py_str("attract"), py_str("bring"), py_str("plant"), py_str("flowers"), py_str("help")])]), py_str("Plant for the whole season rather than one week: lavender, oregano and marjoram in summer, buddleia for July and August, sedum and ivy flowers for September. Leave a sunny corner of long grass and a patch of nettles, because that is where they lay, and put out a shallow dish of water with a few stones in it.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("hedgehog-food"), py_tuple(vec![py_tuple(vec![py_str("hedgehog"), py_str("hedgehogs")]), py_tuple(vec![py_str("eat"), py_str("feed"), py_str("food"), py_str("visiting"), py_str("garden"), py_str("hungry"), py_str("what")])]), py_str("Hedgehogs eat beetles, caterpillars, earthworms and slugs, and by autumn they need to put on weight before hibernation. To feed one, put out cat or dog food (chicken or turkey, not fish), chopped unsalted nuts and a shallow dish of water, after dark. Never milk or bread, and feed under a slate or in a box so cats cannot get at it.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("plant-breathe"), py_tuple(vec![py_tuple(vec![py_str("plant"), py_str("plants"), py_str("stomata"), py_str("leaf"), py_str("leaves")]), py_tuple(vec![py_str("breathe"), py_str("breathing"), py_str("breath"), py_str("respiration"), py_str("oxygen")])]), py_str("Plants breathe through tiny holes called stomata, mostly on the underside of leaves, which open to take in carbon dioxide and let out oxygen and water vapour. Roots take oxygen from air pockets in the soil, which is why a plant in soggy, airless compost can drown. They do it day and night; in daylight photosynthesis makes more oxygen than respiration uses.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("moon-distance")); __args.push(py_tuple(vec![py_str("moon"), py_tuple(vec![py_str("far"), py_str("how far"), py_str("how far away"), py_str("away"), py_str("distance"), py_str("miles"), py_str("km"), py_str("kilometres")])])); __args.push(py_str("About 384,400 km (238,900 miles) away on average, which is roughly 30 Earths side by side, and light takes 1.3 seconds to reach us from it. It is drifting about 4cm further away each year.")); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("moon-daytime"), py_tuple(vec![py_str("moon"), py_tuple(vec![py_str("daytime"), py_str("daylight"), py_str("afternoon"), py_str("morning"), py_str("visible"), py_str("day")])]), py_str("The Moon is above the horizon in daylight most days: it is lit by the sun and simply looks pale against a blue sky. It is easiest to spot in the morning or late afternoon, when the blue is deeper. Around new moon it sits too close to the sun in the sky to see at all.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("full-moon-next")); __args.push(py_tuple(vec![py_tuple(vec![py_str("full moon"), py_str("moon")]), py_tuple(vec![py_str("next"), py_str("when"), py_str("coming"), py_str("due")])])); __args.push(py_func("_next_full_moon", |args: &[Py]| _next_full_moon(args))); __kws.extend(vec![("avoid".to_string(), g_A_BAD_NIGHT())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("moon-phases")); __args.push(py_tuple(vec![py_str("moon"), py_tuple(vec![py_str("phases"), py_str("phase"), py_str("shape"), py_str("crescent"), py_str("change"), py_str("different"), py_str("full")])])); __args.push(py_str("The Moon doesn't change shape; half of it is always lit by the sun, and we see a different slice of that lit half as it orbits the Earth. A full moon takes about 29.5 days to come round again.")); __kws.extend(vec![("avoid".to_string(), g_A_BAD_NIGHT())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fruit-or-berry")); __args.push(py_tuple(vec![g_PRODUCE(), py_tuple(vec![py_str("fruit"), py_str("fruits"), py_str("berry"), py_str("berries"), py_str("vegetable"), py_str("vegetables"), py_str("botanically"), py_str("really")])])); __args.push(py_func("_fruit_or_berry", |args: &[Py]| _fruit_or_berry(args))); __kws.extend(vec![("avoid".to_string(), __data32()), ("guard".to_string(), py_func("_not_the_definition", |args: &[Py]| _not_the_definition(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.nature.ENTRIES"); }
    s_ENTRIES(__data8());
}
pub fn _next_full_moon(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut cycles: Py = Py::None;
    let mut days: Py = Py::None;
    let mut full: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut now: Py = Py::None;
    let mut when: Py = Py::None;
    let mut when_text: Py = Py::None;
    let _ = py_str("The date of the next full moon, worked out from a known new moon.\n\n    The phases repeat every 29.53 days, so one reference date plus the date today is\n    enough; no table and no network. The answer rounds to the day, which is all that is\n    useful for \"when is the next full moon\".\n    ");
    now = stdlib_call("datetime", "datetime.now", &[]).unwrap_or_else(|e| exc::raise(e));
    cycles = op_truediv(&op_truediv(&mcall(&op_sub(&now.clone(), &g_NEW_MOON()), "total_seconds", &[]), &Py::Float(86400.0)), &g_SYNODIC_MONTH());
    full = op_add(&call_builtin("int", &[op_sub(&cycles.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    when = op_add(&g_NEW_MOON(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_mul(&op_add(&full.clone(), &Py::Float(0.5)), &g_SYNODIC_MONTH()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    days = gattr(&op_sub(&mcall(&when.clone(), "date", &[]), &mcall(&now.clone(), "date", &[])), "days");
    when_text = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&when.clone(), "strftime", &[py_str("%A")]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&when.clone(), "strftime", &[py_str("%B")]), -1, "")); py_string(__s) };
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    lead = py_str("Tonight is a full moon");
    } else {
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    lead = { let mut __s = String::new(); __s.push_str("The next full moon is tomorrow, "); __s.push_str(&fstr(&when_text.clone(), -1, "")); py_string(__s) };
    } else {
    lead = { let mut __s = String::new(); __s.push_str("The next full moon is on "); __s.push_str(&fstr(&when_text.clone(), -1, "")); __s.push_str(", in "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days"); py_string(__s) };
    }
    }
    return op_add(&lead.clone(), &py_str(". Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer."));
    Py::None
}
pub fn _not_the_definition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True while the line asks what the produce is, not what its plant is doing.");
    line = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&{ let __b = op_bitand(&words.clone(), &g__SETTING_CROPS()); if !py_truthy(&__b) { __b } else { mcall(&g__PLANT_TROUBLE(), "search", &[line.clone()]) } }));
    Py::None
}
pub fn _fruiting_of_a_crop(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks whether a plant will crop, not what kind of thing the crop is.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_HARVEST_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fruit_or_berry(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut art: Py = Py::None;
    let mut asks_berry: Py = Py::None;
    let mut asks_vegetable: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the produce in the line is a fruit, a berry or a root, and why.");
    line = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&_fruiting_of_a_crop(&[line.clone()])) {
    return Py::None;
    }
    word = py_str("");
    let __iter1 = py_iter_or_raise(&g_PRODUCE());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_str("y")])) {
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), py_str("(?:y|ies)")]), 2, None);
    stem = __unpk2[0].clone();
    plural = __unpk2[1].clone();
    } else {
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_str("o")])) {
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), py_str("(?:o|os|oes)")]), 2, None);
    stem = __unpk3[0].clone();
    plural = __unpk3[1].clone();
    } else {
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_tuple(vec![py_str("ch"), py_str("sh"), py_str("s"), py_str("x")])])) {
    let __unpk4 = py_unpack(&py_tuple(vec![name.clone(), py_str("(?:es)?")]), 2, None);
    stem = __unpk4[0].clone();
    plural = __unpk4[1].clone();
    } else {
    let __unpk5 = py_unpack(&py_tuple(vec![name.clone(), py_str("s?")]), 2, None);
    stem = __unpk5[0].clone();
    plural = __unpk5[1].clone();
    }
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e))), &plural.clone()), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    word = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("Botanically a fruit is the part that grows from the flower and holds the seeds, so tomatoes, cucumbers and peppers are fruits. A berry is a fruit from one ovary with seeds in the flesh, which makes bananas berries but strawberries not.");
    }
    art = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an") } else { py_str("a") };
    asks_berry = Py::Bool(py_contains(&line.clone(), &py_str("berr")));
    asks_vegetable = { let __b = Py::Bool(py_contains(&line.clone(), &py_str("vegetable"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_berry.clone())) } };
    if py_truthy(&Py::Bool(py_contains(&g_PLANT_PARTS(), &word.clone()))) {
    part = geti(&g_PLANT_PARTS(), &word.clone());
    if py_truthy(&asks_vegetable.clone()) {
    return { let mut __s = String::new(); __s.push_str("Yes - in the kitchen "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a vegetable, but botanically it is "); __s.push_str(&fstr(&part.clone(), -1, "")); __s.push_str(", not a fruit: the fruit is the part that holds the seeds."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("No - "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&part.clone(), -1, "")); __s.push_str(", not a fruit. The fruit is the part that holds the seeds, so "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a vegetable in the kitchen and "); __s.push_str(&fstr(&part.clone(), -1, "")); __s.push_str(" in the ground."); py_string(__s) };
    }
    if py_truthy(&asks_berry.clone()) {
    if py_truthy(&Py::Bool(py_contains(&g_AGGREGATE_BERRIES(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str("No - "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("cherry")))) {
    return py_str("No - a cherry is a drupe: one seed inside a hard stone, not a berry. A banana or a tomato really is a berry, which is where the rule surprises people.");
    }
    return { let mut __s = String::new(); __s.push_str("Yes - "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&g_BOTANICAL_FRUITS(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str("Botanically "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a "); __s.push_str(&fstr(&geti(&g_BOTANICAL_FRUITS(), &word.clone()), -1, "")); __s.push_str(". Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&g_KITCHEN_FRUITS(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str("Botanically "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("Botanically "); __s.push_str(&fstr(&art.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" is a fruit, because it grows from the flower and holds its seeds. A vegetable is the other parts of the plant, such as roots, leaves and stems."); py_string(__s) };
    Py::None
}
pub fn _the_wet_day(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the rain does to the plan, and what to do when it is set in for the week.");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    said = py_str("Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("all week"), py_str("every day"), py_str("a lot"), py_str("pours"), py_str("turns"), py_str("weather is bad")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&said.clone(), &py_str(" If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all."));
    said = __aug1;
    }
    return said.clone();
    Py::None
}
pub fn _the_older_turns(args: &[Py]) -> Py {
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
    let mut keep: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { Py::Int(3) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the last few turns said, for a follow-up that names only the part.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&keep.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    return op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    Py::None
}
pub fn _a_plant_they_own_is_the_subject(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the conversation is about a plant the person is looking after.");
    said = _the_older_turns(&[memory.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PLANTS_IN_HAND()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &word.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %ss "), &word.clone()))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_autumn_leaves(args: &[Py]) -> Py {
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
    let _ = py_str("Why leaves turn, unless the leaves are a plant of theirs going wrong.");
    if py_truthy(&_a_plant_they_own_is_the_subject(&[memory.clone()])) {
    return Py::None;
    }
    return py_str("Shorter days tell trees to stop making chlorophyll, the green stuff, and as it fades the yellow and orange pigments underneath show through; some trees also make red pigments. The tree then seals off each leaf and lets it drop, so it loses less water over winter.");
    Py::None
}
pub fn _the_tide(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The tide at the place the person is going: a table for one port, not the sea's physics.\n\n    \"what time is high tide\", \"how high is the tide\" and \"is the tide coming in\" are one shape of\n    question - a number for one port on one day - and the answer a person acts on is that port's\n    own table. What the bot can add is how to read it, how much later it comes each day, and how\n    much water to leave behind on flat sand. The why of the bulge stays with the `tides` row.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("in or out"), py_str("coming in"), py_str("going out"), py_str("tide doing"), py_str("rising"), py_str("falling"), py_str("dropping"), py_str("state of the tide")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Watch it rather than guess: on a flooding tide the water climbs a few inches every ten minutes and the weed floats past in one direction; on the ebb the wet sand shows above the water line and the rocks come out. The nearest port's table tells you the turn of it, and the turn is what the times are.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("causeway"), py_str("cross the sand"), py_str("crossing the sand"), py_str("walk across"), py_str("mudflat"), py_str("quicksand"), py_str("tidal road")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Do not cross on a guess, and never on another port's table: the bay's own crossing times are published by the coastguard or the local guides, and they are the only ones that count. Tell somebody where you are going and when you will be back, and never park below the high-water mark - cars are lost that way every year.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("swim"), py_str("swimming"), py_str("paddle"), py_str("paddling"), py_str("in the water")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Swim near the turn of the tide rather than on a big spring ebb, keep to a lifeguarded beach, and stay off flat sand where the flood gets behind you: the water comes back faster than it left. The nearest port's table gives you the turn, and aim to be out of it well before.");
    }
    return py_str("A tide time is local, so the nearest port's table is the only one that counts - the harbour, the coastguard, or the tide page for that port - and high water comes about 50 minutes later each day, so yesterday's table is no use. The sea slacks for a few minutes either side of the turn, and on flat sand the flood comes in faster than it looks: be off the beach an hour before the table says.");
    Py::None
}
pub fn _month_daylight(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut hours: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The month's own figure, or None when the line named no month we answer for.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g__DAYLIGHT_BY_MONTH());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    hours = __unpk2[1].clone();
    minutes = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("may")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str(" in may "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" of may "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" during may "))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str(" may")]) } } } } } }) {
    return py_tuple(vec![name.clone(), hours.clone(), minutes.clone()]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))) {
    return py_tuple(vec![name.clone(), hours.clone(), minutes.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _daylight(args: &[Py]) -> Py {
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
    let mut hours: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The long day or the short one, whichever the line is asking about.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DAYLIGHT_SHORT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__DAYLIGHT_WINTER();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DAYLIGHT_LONG_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("scotland"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("scottish"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("shetland"))) } } } }) {
    return op_add(&g__DAYLIGHT_SUMMER(), &g__DAYLIGHT_SCOTLAND());
    }
    return g__DAYLIGHT_SUMMER();
    }
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &py_str(" december ")))) {
    month = _month_daylight(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    let __unpk1 = py_unpack(&month.clone(), 3, None);
    name = __unpk1[0].clone();
    hours = __unpk1[1].clone();
    minutes = __unpk1[2].clone();
    return op_modulo(&py_str("In the UK, about the middle of %s gives roughly %d hours %d minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes."), &py_tuple(vec![name.clone(), hours.clone(), minutes.clone()]));
    }
    }
    return g__DAYLIGHT_WINTER();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.nature".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGGREGATE_BERRIES" => Some(g_AGGREGATE_BERRIES()),
            "ANSWERS" => Some(g_ANSWERS()),
            "A_BAD_NIGHT" => Some(g_A_BAD_NIGHT()),
            "A_WHAT_TO_WEAR" => Some(g_A_WHAT_TO_WEAR()),
            "BOTANICAL_FRUITS" => Some(g_BOTANICAL_FRUITS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "KITCHEN_FRUITS" => Some(g_KITCHEN_FRUITS()),
            "NAMED_PLANTS" => Some(g_NAMED_PLANTS()),
            "NEW_MOON" => Some(g_NEW_MOON()),
            "PLANT_PARTS" => Some(g_PLANT_PARTS()),
            "PRODUCE" => Some(g_PRODUCE()),
            "SYNODIC_MONTH" => Some(g_SYNODIC_MONTH()),
            "THE_HARVEST_ASK" => Some(g_THE_HARVEST_ASK()),
            "TREES" => Some(g_TREES()),
            "TRUE_BERRIES" => Some(g_TRUE_BERRIES()),
            "WHAT_MAKES" => Some(g_WHAT_MAKES()),
            "_DAYLIGHT_BY_MONTH" => Some(g__DAYLIGHT_BY_MONTH()),
            "_DAYLIGHT_LONG_WORDS" => Some(g__DAYLIGHT_LONG_WORDS()),
            "_DAYLIGHT_SCOTLAND" => Some(g__DAYLIGHT_SCOTLAND()),
            "_DAYLIGHT_SHORT_WORDS" => Some(g__DAYLIGHT_SHORT_WORDS()),
            "_DAYLIGHT_SUMMER" => Some(g__DAYLIGHT_SUMMER()),
            "_DAYLIGHT_WINTER" => Some(g__DAYLIGHT_WINTER()),
            "_PLANTS_IN_HAND" => Some(g__PLANTS_IN_HAND()),
            "_PLANT_TROUBLE" => Some(g__PLANT_TROUBLE()),
            "_SETTING_CROPS" => Some(g__SETTING_CROPS()),
            "_a_plant_they_own_is_the_subject" => Some(py_func("_a_plant_they_own_is_the_subject", |args: &[Py]| _a_plant_they_own_is_the_subject(args))),
            "_daylight" => Some(py_func("_daylight", |args: &[Py]| _daylight(args))),
            "_fruit_or_berry" => Some(py_func("_fruit_or_berry", |args: &[Py]| _fruit_or_berry(args))),
            "_fruiting_of_a_crop" => Some(py_func("_fruiting_of_a_crop", |args: &[Py]| _fruiting_of_a_crop(args))),
            "_month_daylight" => Some(py_func("_month_daylight", |args: &[Py]| _month_daylight(args))),
            "_next_full_moon" => Some(py_func("_next_full_moon", |args: &[Py]| _next_full_moon(args))),
            "_not_the_definition" => Some(py_func("_not_the_definition", |args: &[Py]| _not_the_definition(args))),
            "_the_autumn_leaves" => Some(py_func("_the_autumn_leaves", |args: &[Py]| _the_autumn_leaves(args))),
            "_the_older_turns" => Some(py_func("_the_older_turns", |args: &[Py]| _the_older_turns(args))),
            "_the_tide" => Some(py_func("_the_tide", |args: &[Py]| _the_tide(args))),
            "_the_wet_day" => Some(py_func("_the_wet_day", |args: &[Py]| _the_wet_day(args))),
            "entry" => Some(g_entry()),
            "_dt" => Some(stdlib_module("datetime")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGGREGATE_BERRIES" => s_AGGREGATE_BERRIES(v),
            "ANSWERS" => s_ANSWERS(v),
            "A_BAD_NIGHT" => s_A_BAD_NIGHT(v),
            "A_WHAT_TO_WEAR" => s_A_WHAT_TO_WEAR(v),
            "BOTANICAL_FRUITS" => s_BOTANICAL_FRUITS(v),
            "ENTRIES" => s_ENTRIES(v),
            "KITCHEN_FRUITS" => s_KITCHEN_FRUITS(v),
            "NAMED_PLANTS" => s_NAMED_PLANTS(v),
            "NEW_MOON" => s_NEW_MOON(v),
            "PLANT_PARTS" => s_PLANT_PARTS(v),
            "PRODUCE" => s_PRODUCE(v),
            "SYNODIC_MONTH" => s_SYNODIC_MONTH(v),
            "THE_HARVEST_ASK" => s_THE_HARVEST_ASK(v),
            "TREES" => s_TREES(v),
            "TRUE_BERRIES" => s_TRUE_BERRIES(v),
            "WHAT_MAKES" => s_WHAT_MAKES(v),
            "_DAYLIGHT_BY_MONTH" => s__DAYLIGHT_BY_MONTH(v),
            "_DAYLIGHT_LONG_WORDS" => s__DAYLIGHT_LONG_WORDS(v),
            "_DAYLIGHT_SCOTLAND" => s__DAYLIGHT_SCOTLAND(v),
            "_DAYLIGHT_SHORT_WORDS" => s__DAYLIGHT_SHORT_WORDS(v),
            "_DAYLIGHT_SUMMER" => s__DAYLIGHT_SUMMER(v),
            "_DAYLIGHT_WINTER" => s__DAYLIGHT_WINTER(v),
            "_PLANTS_IN_HAND" => s__PLANTS_IN_HAND(v),
            "_PLANT_TROUBLE" => s__PLANT_TROUBLE(v),
            "_SETTING_CROPS" => s__SETTING_CROPS(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.nature", module_obj);
    pyrt::xrefs::reg_fn("knowledge.nature", "_next_full_moon", py_func("_next_full_moon", |args: &[Py]| _next_full_moon(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_not_the_definition", py_func("_not_the_definition", |args: &[Py]| _not_the_definition(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_fruiting_of_a_crop", py_func("_fruiting_of_a_crop", |args: &[Py]| _fruiting_of_a_crop(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_fruit_or_berry", py_func("_fruit_or_berry", |args: &[Py]| _fruit_or_berry(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_the_wet_day", py_func("_the_wet_day", |args: &[Py]| _the_wet_day(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_the_older_turns", py_func("_the_older_turns", |args: &[Py]| _the_older_turns(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_a_plant_they_own_is_the_subject", py_func("_a_plant_they_own_is_the_subject", |args: &[Py]| _a_plant_they_own_is_the_subject(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_the_autumn_leaves", py_func("_the_autumn_leaves", |args: &[Py]| _the_autumn_leaves(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_the_tide", py_func("_the_tide", |args: &[Py]| _the_tide(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_month_daylight", py_func("_month_daylight", |args: &[Py]| _month_daylight(args)));
    pyrt::xrefs::reg_fn("knowledge.nature", "_daylight", py_func("_daylight", |args: &[Py]| _daylight(args)));
    pyrt::xrefs::reg_access("knowledge.nature", "AGGREGATE_BERRIES", g_AGGREGATE_BERRIES, s_AGGREGATE_BERRIES);
    pyrt::xrefs::reg_access("knowledge.nature", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.nature", "A_BAD_NIGHT", g_A_BAD_NIGHT, s_A_BAD_NIGHT);
    pyrt::xrefs::reg_access("knowledge.nature", "A_WHAT_TO_WEAR", g_A_WHAT_TO_WEAR, s_A_WHAT_TO_WEAR);
    pyrt::xrefs::reg_access("knowledge.nature", "BOTANICAL_FRUITS", g_BOTANICAL_FRUITS, s_BOTANICAL_FRUITS);
    pyrt::xrefs::reg_access("knowledge.nature", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.nature", "KITCHEN_FRUITS", g_KITCHEN_FRUITS, s_KITCHEN_FRUITS);
    pyrt::xrefs::reg_access("knowledge.nature", "NAMED_PLANTS", g_NAMED_PLANTS, s_NAMED_PLANTS);
    pyrt::xrefs::reg_access("knowledge.nature", "NEW_MOON", g_NEW_MOON, s_NEW_MOON);
    pyrt::xrefs::reg_access("knowledge.nature", "PLANT_PARTS", g_PLANT_PARTS, s_PLANT_PARTS);
    pyrt::xrefs::reg_access("knowledge.nature", "PRODUCE", g_PRODUCE, s_PRODUCE);
    pyrt::xrefs::reg_access("knowledge.nature", "SYNODIC_MONTH", g_SYNODIC_MONTH, s_SYNODIC_MONTH);
    pyrt::xrefs::reg_access("knowledge.nature", "THE_HARVEST_ASK", g_THE_HARVEST_ASK, s_THE_HARVEST_ASK);
    pyrt::xrefs::reg_access("knowledge.nature", "TREES", g_TREES, s_TREES);
    pyrt::xrefs::reg_access("knowledge.nature", "TRUE_BERRIES", g_TRUE_BERRIES, s_TRUE_BERRIES);
    pyrt::xrefs::reg_access("knowledge.nature", "WHAT_MAKES", g_WHAT_MAKES, s_WHAT_MAKES);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_BY_MONTH", g__DAYLIGHT_BY_MONTH, s__DAYLIGHT_BY_MONTH);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_LONG_WORDS", g__DAYLIGHT_LONG_WORDS, s__DAYLIGHT_LONG_WORDS);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_SCOTLAND", g__DAYLIGHT_SCOTLAND, s__DAYLIGHT_SCOTLAND);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_SHORT_WORDS", g__DAYLIGHT_SHORT_WORDS, s__DAYLIGHT_SHORT_WORDS);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_SUMMER", g__DAYLIGHT_SUMMER, s__DAYLIGHT_SUMMER);
    pyrt::xrefs::reg_access("knowledge.nature", "_DAYLIGHT_WINTER", g__DAYLIGHT_WINTER, s__DAYLIGHT_WINTER);
    pyrt::xrefs::reg_access("knowledge.nature", "_PLANTS_IN_HAND", g__PLANTS_IN_HAND, s__PLANTS_IN_HAND);
    pyrt::xrefs::reg_access("knowledge.nature", "_PLANT_TROUBLE", g__PLANT_TROUBLE, s__PLANT_TROUBLE);
    pyrt::xrefs::reg_access("knowledge.nature", "_SETTING_CROPS", g__SETTING_CROPS, s__SETTING_CROPS);
    pyrt::xrefs::reg_access("knowledge.nature", "entry", g_entry, s_entry);
}

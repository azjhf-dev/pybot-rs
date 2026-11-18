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
thread_local! { static G_DRINK_COOLING_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRINK_COOLING_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_THING_COOLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_DRINK_COOLING_ASKS() -> Py { ensure_init(); G_DRINK_COOLING_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINK_COOLING_ASKS(v: Py) { ensure_init(); G_DRINK_COOLING_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DRINK_COOLING_NAMES() -> Py { ensure_init(); G_DRINK_COOLING_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINK_COOLING_NAMES(v: Py) { ensure_init(); G_DRINK_COOLING_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_THING_COOLING() -> Py { ensure_init(); G_HOT_THING_COOLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_THING_COOLING(v: Py) { ensure_init(); G_HOT_THING_COOLING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A hot thing asked how long it takes to cool gets a length of time.\n\nFour cards answered \"how long does tea take to cool\" and each was wrong on its own word: the\nshelf-life card said tea does not go off, the caffeine card said five hours, the picnic cool bag\ntook the rest. Three more hot things had the same fault: \"how long does a hot water bottle take to\ncool down\" was the five-minute easy walk, \"how long does boiled water take to cool for a baby\" was\na first labour, and \"how long does rice take to cool\" was how long rice takes to cook.\n\nEach line is the right topic and the wrong shape, so this is a shape row of its own: the thing and\na cooling phrase together, answered with the minutes the heat takes to come down. It lives in a\nfile of its own because the rows are keyed on a phrase (a cooling verb) rather than on a food, and\nbecause the per-thing times are the part of this that will keep growing.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink"), py_str("family-and-parenting")]));
    s_ENTRIES(py_list(vec![]));
    s_DRINK_COOLING_NAMES(py_tuple(vec![py_str("tea"), py_str("coffee"), py_str("cuppa"), py_str("brew"), py_str("hot chocolate"), py_str("mug"), py_str("cup"), py_str("drink"), py_str("kettle"), py_str("teapot")]));
    s_DRINK_COOLING_ASKS(py_tuple(vec![py_str("to cool"), py_str("takes to cool"), py_str("take to cool"), py_str("cool down"), py_str("cooling down"), py_str("cool off"), py_str("cooling off"), py_str("cool enough"), py_str("too hot to drink"), py_str("too hot to sip"), py_str("cool before drinking"), py_str("cool before you drink"), py_str("cool before i drink"), py_str("let it cool"), py_str("let my tea cool"), py_str("let my coffee cool"), py_str("let the tea cool"), py_str("let the coffee cool"), py_str("let a cup"), py_str("let my cup"), py_str("let the cup"), py_str("let it cool down")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-drink-cooling")); __args.push(py_tuple(vec![g_DRINK_COOLING_NAMES(), g_DRINK_COOLING_ASKS()])); __args.push(py_str("Five to ten minutes in an ordinary room: a drink comes off the kettle at about 90C and is comfortable to sip at about 60C, and a splash of cold milk takes it down at once. A wide mug cools faster than a tall narrow one, and standing the mug in a bowl of cold water does it in a couple of minutes - stir it and test it with your lip, not a finger. A lid on the mug keeps the heat in, so take it off if you want it sooner.")); __kws.extend(vec![("ask".to_string(), g_DRINK_COOLING_ASKS()), ("avoid".to_string(), py_tuple(vec![py_str("soup"), py_str("stew"), py_str("curry"), py_str("casserole"), py_str("broth"), py_str("gravy"), py_str("porridge"), py_str("chilli"), py_str("leftover"), py_str("leftovers"), py_str("in the fridge"), py_str("before the fridge"), py_str("freezer"), py_str("ice"), py_str("iced"), py_str("cold brew"), py_str("ice cream"), py_str("fizzy"), py_str("soda"), py_str("beer"), py_str("wine")])), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    s_HOT_THING_COOLING(py_tuple(vec![py_str("to cool"), py_str("takes to cool"), py_str("take to cool"), py_str("cool down"), py_str("cooling down"), py_str("cool off"), py_str("cooling off"), py_str("cool enough"), py_str("too hot"), py_str("how long until it is"), py_str("how long before it is"), py_str("cools"), py_str("cool before"), py_str("cool for")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-hot-water-bottle-cooling")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hot water bottle"), py_str("hottie"), py_str("hot water bottles"), py_str("wheat bag"), py_str("heat pack")]), g_HOT_THING_COOLING()])); __args.push(py_str("A hot water bottle stays warm for four to six hours under the covers, and it is only too hot for the first twenty minutes or so. If you want it gentler from the start, fill it from the hot tap rather than the kettle and wrap it in a towel or a pillowcase - running it under the cold tap for a few seconds does the same. Never fill it with boiling water for a child or an older person, and check the stopper and the rubber for cracks before each winter.")); __kws.extend(vec![("ask".to_string(), g_HOT_THING_COOLING()), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("price"), py_str("much"), py_str("buy"), py_str("worth"), py_str("last"), py_str("lasts"), py_str("burst"), py_str("leak"), py_str("smell"), py_str("rubber"), py_str("how often"), py_str("replace"), py_str("how do i cool"), py_str("how to cool"), py_str("how can i cool"), py_str("ways to cool"), py_str("make it cooler"), py_str("cool it down quickly")])), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("boiled-water-for-a-feed-cooling")); __args.push(py_tuple(vec![py_tuple(vec![py_str("boiled water"), py_str("kettle water"), py_str("water for the formula"), py_str("water for a feed"), py_str("water for the bottle"), py_str("formula water"), py_str("sterilised water")]), g_HOT_THING_COOLING()])); __args.push(py_str("Make the feed within 30 minutes of boiling: the water should still be at least 70C when it goes onto the powder, because that is what kills anything in it. Then cool the bottle under a running cold tap or stand it in a jug of cold water - about 10 to 15 minutes, and shake a drop onto the inside of your wrist to test it. Water that has been boiled once and left to cool is fine for drinking, but make a fresh feed rather than reheating an old one.")); __kws.extend(vec![("ask".to_string(), g_HOT_THING_COOLING()), ("avoid".to_string(), py_tuple(vec![py_str("bath"), py_str("bathwater"), py_str("hot water bottle"), py_str("washing up"), py_str("tap water"), py_str("hard water"), py_str("filter"), py_str("kettle limescale"), py_str("descale")])), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-pot-of-rice-cooling")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("cooked rice"), py_str("leftover rice")]), g_HOT_THING_COOLING()])); __args.push(py_str("Spread the rice out on a tray or a wide plate and it is cool enough for the fridge in about 20 to 30 minutes - a shallow layer, not a deep pan with the lid on. Get it into the fridge within the hour, keep it no more than a day, and reheat it once, until it is steaming hot all through. Rice left standing at room temperature is the one to bin, because of the spores that survive cooking.")); __kws.extend(vec![("ask".to_string(), g_HOT_THING_COOLING()), ("avoid".to_string(), py_tuple(vec![py_str("how long to cook"), py_str("take to cook"), py_str("simmer"), py_str("boil"), py_str("boiling"), py_str("absorption"), py_str("cup"), py_str("grams"), py_str("portion"), py_str("water ratio"), py_str("rice cooker"), py_str("pilau"), py_str("risotto")])), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]);
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cooling".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "DRINK_COOLING_ASKS" => Some(g_DRINK_COOLING_ASKS()),
            "DRINK_COOLING_NAMES" => Some(g_DRINK_COOLING_NAMES()),
            "ENTRIES" => Some(g_ENTRIES()),
            "HOT_THING_COOLING" => Some(g_HOT_THING_COOLING()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "DRINK_COOLING_ASKS" => s_DRINK_COOLING_ASKS(v),
            "DRINK_COOLING_NAMES" => s_DRINK_COOLING_NAMES(v),
            "ENTRIES" => s_ENTRIES(v),
            "HOT_THING_COOLING" => s_HOT_THING_COOLING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.cooling", module_obj);
    pyrt::xrefs::reg_access("knowledge.cooling", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.cooling", "DRINK_COOLING_ASKS", g_DRINK_COOLING_ASKS, s_DRINK_COOLING_ASKS);
    pyrt::xrefs::reg_access("knowledge.cooling", "DRINK_COOLING_NAMES", g_DRINK_COOLING_NAMES, s_DRINK_COOLING_NAMES);
    pyrt::xrefs::reg_access("knowledge.cooling", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.cooling", "HOT_THING_COOLING", g_HOT_THING_COOLING, s_HOT_THING_COOLING);
}

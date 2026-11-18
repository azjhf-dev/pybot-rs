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
thread_local! { static G_WATER_OUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_OUT_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.water_out: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_OUT_ASK() -> Py { ensure_init(); G_WATER_OUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_OUT_ASK(v: Py) { ensure_init(); G_WATER_OUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_OUT_OF() -> Py { ensure_init(); G_WATER_OUT_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_OUT_OF(v: Py) { ensure_init(); G_WATER_OUT_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Getting the water out of a bath, a hose, a freezer, a loft tank, a waterbed or a fish tank.\n\nThe answers lane owns this domain. All six are one question - how do I get a lot of water out\nof a thing - and the thing is what decides how they are emptied, so this file holds the table\nand the one row that reads it, the thing out of the line, rather than four rows opening with\nthe same sentence.\n\nWhy a file of its own: the rows for the house live in `home.py` and its tail `home_craft.py`,\nand that tail is at 879 KB of the publisher's 1 MiB ceiling, so a new block of rows and its\ntable goes beside them, the way `bake_faults.py` sits beside `cooking.py`. Nothing else needs\nto change: a domain is a file with ENTRIES.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
    s_WATER_OUT_OF(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bath"), py_str("baths"), py_str("bathtub"), py_str("bath tub")]), py_str("Pull the plug and let it go: a full bath is 80 to 100 litres and an ordinary waste takes it in a few minutes. If the plug hole will not clear, bail most of it into a bucket with a jug first, then syphon the last of it: fill a garden hose with water, keep one end in the bath and hang the other end lower down - out of the window into a drain, or into a bucket - and it runs on its own. Never tip a full bath down a floor drain or a shower tray, which will not take it fast enough.")]), py_tuple(vec![py_tuple(vec![py_str("caravan"), py_str("caravans"), py_str("motorhome"), py_str("motor home"), py_str("camper van"), py_str("campervan"), py_str("camper")]), py_str("Open the drain plug or valve on the water tank and the taps at the sink first, so air gets in and the water runs out instead of glugging, then find the drain on the water heater - a small tap or a plug on its side - and empty that too, because a heater left full splits when it freezes. Empty the waste water carrier and the toilet cassette, run the pump dry for a second only, and leave the taps open. A caravan tank is 40 to 100 litres, so put a bucket under it, not your shoes.")]), py_tuple(vec![py_tuple(vec![py_str("water feature"), py_str("water fountain"), py_str("fountain"), py_str("fountains"), py_str("waterfall"), py_str("water spout"), py_str("water wall")]), py_str("Switch the pump off at the plug before you touch the water, then let it drain: most features have a drain plug or a bung at the lowest point, and without one you syphon it into a bucket with a hose. Lift the pump out and clean the sponge, tip the bowl rather than dragging it, and never let the pump run dry - it burns out in a minute once the water is below its intake.")]), py_tuple(vec![py_tuple(vec![py_str("bilge"), py_str("bilges"), py_str("boat bilge")]), py_str("Pump the bilge rather than bailing it: an electric bilge pump on a float switch should do it on its own, and a hand pump or a bucket and a sponge does the rest when it does not. Get it dry afterwards and find out where the water came from - a bilge that fills when the engine is off is a leak, not rain, and that is a lift-out job.")]), py_tuple(vec![py_tuple(vec![py_str("hose"), py_str("hoses"), py_str("hose pipe"), py_str("hosepipe"), py_str("garden hose"), py_str("watering hose")]), py_str("Take the nozzle off and walk the hose out straight downhill, lifting it hand over hand so the water runs to the low end - a minute's job and it does not kink. Coil it loosely and hang it with both ends down so what is left drips out, and bring it into the shed or indoors before the first frost: a hose left holding water splits when it freezes, and one put away wet grows slime inside.")]), py_tuple(vec![py_tuple(vec![py_str("freezer"), py_str("freezers"), py_str("chest freezer"), py_str("fridge freezer"), py_str("ice box")]), py_str("Switch it off at the wall, put the food in a cool box or wrap it in towels, and let the ice melt rather than chipping at it - a knife through the lining lets the gas out and writes the freezer off. Bowls of hot water on the shelves speed it up, and a folded towel or a washing-up bowl at the bottom catches the water. A chest freezer has a small drain bung or cap at the front: pull it out, stand a shallow tray under it and the meltwater runs out there instead of over the floor.")]), py_tuple(vec![py_tuple(vec![py_str("waterbed"), py_str("water bed"), py_str("water beds")]), py_str("Borrow or hire a waterbed pump - the same one fills and empties it - because a double holds 200 to 400 litres and a jug will not do it. Without a pump, syphon it with a garden hose to a drain or out of a low window, and empty it fully before you move the frame: a hundred litres left in the mattress is more than a floor will carry up the stairs. The water is fine on the garden, but not on grass you want to keep.")]), py_tuple(vec![py_tuple(vec![py_str("fish tank"), py_str("fish tanks"), py_str("aquarium"), py_str("fish bowl")]), py_str("Syphon the water out into buckets with a length of airline or a gravel cleaner, and keep about half of it: new water all at once shocks the fish and loses the bacteria the filter needs. Keep the filter sponges wet in the old water while you work, leave the fish in a bucket of tank water with the lid off the light, and put the tank back on the same stand it came off - a full one is far too heavy for a table or a shelf.")]), py_tuple(vec![py_tuple(vec![py_str("water tank"), py_str("header tank"), py_str("cold water tank"), py_str("loft tank"), py_str("cold tank")]), py_str("Stop the water going in before you empty it, or it refills as fast as it drains: tie the float valve up with a bit of string, or turn the stopcock off. Then either open the drain cock at the bottom of the tank - a small tap on the side, with a hose on its spout to a drain or a bucket - or drop a garden hose in, fill the hose with water, and hang the other end lower than the tank, because a loft tank is above the house and gravity does the rest. A domestic cold water tank holds 100 to 250 litres, so have a drain or a big container ready; a mop and towels are not a plan.")])]));
    fn __data1() -> Py { py_tuple(vec![py_str("drain"), py_str("drains"), py_str("draining"), py_str("drained"), py_str("empty"), py_str("emptying"), py_str("emptied"), py_str("pump out"), py_str("pump it out"), py_str("siphon"), py_str("syphon"), py_str("syphoning"), py_str("get the water out"), py_str("water out"), py_str("tip out"), py_str("tip it out"), py_str("pour away"), py_str("melt water"), py_str("melted water"), py_str("defrosted water"), py_str("store"), py_str("storing"), py_str("put away"), py_str("winter"), py_str("winterise"), py_str("winterize")]) }
    s_WATER_OUT_ASK(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("bath"), py_str("baths"), py_str("bathtub"), py_str("bath tub"), py_str("hose"), py_str("hoses"), py_str("hose pipe"), py_str("hosepipe"), py_str("garden hose"), py_str("watering hose"), py_str("freezer"), py_str("freezers"), py_str("chest freezer"), py_str("fridge freezer"), py_str("ice box"), py_str("water tank"), py_str("header tank"), py_str("cold water tank"), py_str("loft tank"), py_str("cold tank"), py_str("fish tank"), py_str("fish tanks"), py_str("aquarium"), py_str("fish bowl"), py_str("caravan"), py_str("caravans"), py_str("motorhome"), py_str("motor home"), py_str("camper van"), py_str("campervan"), py_str("camper"), py_str("bilge"), py_str("bilges"), py_str("water feature"), py_str("water fountain"), py_str("fountain"), py_str("fountains"), py_str("waterfall"), py_str("water spout"), py_str("water wall"), py_str("waterbed"), py_str("water bed"), py_str("water beds")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("wont drain"), py_str("won't drain"), py_str("not draining"), py_str("isnt draining"), py_str("isn't draining"), py_str("doesnt drain"), py_str("doesn't drain"), py_str("slowly"), py_str("slow"), py_str("ages"), py_str("takes ages"), py_str("takes forever"), py_str("a long time"), py_str("bit by bit"), py_str("gurgles"), py_str("blocked"), py_str("blockage"), py_str("backed up"), py_str("unblock"), py_str("clogged"), py_str("clog"), py_str("smells"), py_str("smelly"), py_str("stinks"), py_str("hair"), py_str("plunger"), py_str("plughole"), py_str("plug hole"), py_str("defrost"), py_str("defrosting"), py_str("defrosted"), py_str("frost"), py_str("ices up"), py_str("ice build"), py_str("ice built"), py_str("how often"), py_str("buy"), py_str("buying"), py_str("fit a"), py_str("fitting"), py_str("new"), py_str("replace"), py_str("repair"), py_str("broken"), py_str("cost"), py_str("worth it"), py_str("hose pipe ban"), py_str("height"), py_str("size"), py_str("how big"), py_str("how deep"), py_str("paddling pool"), py_str("swimming pool"), py_str("hot tub"), py_str("jacuzzi"), py_str("water butt"), py_str("rain barrel"), py_str("pond"), py_str("ponds"), py_str("radiator"), py_str("radiators"), py_str("boiler"), py_str("central heating"), py_str("expansion tank"), py_str("septic tank"), py_str("oil tank"), py_str("dishwasher"), py_str("washing machine"), py_str("washer"), py_str("dryer"), py_str("toilet"), py_str("cistern"), py_str("van"), py_str("car"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("rice"), py_str("potato"), py_str("potatoes"), py_str("veg"), py_str("vegetables"), py_str("broccoli"), py_str("carrots"), py_str("chickpeas"), py_str("beans"), py_str("tuna"), py_str("sweetcorn"), py_str("colander"), py_str("cooking"), py_str("boil"), py_str("boiling"), py_str("kettle"), py_str("sauce"), py_str("tin of"), py_str("can of"), py_str("tights"), py_str("stockings"), py_str("nylons"), py_str("pantyhose"), py_str("football"), py_str("soccer"), py_str("bath city"), py_str("roman baths"), py_str("spa town"), py_str("fuel tank"), py_str("petrol"), py_str("diesel"), py_str("tank top"), py_str("think tank"), py_str("tank of"), py_str("hot water"), py_str("hot water tank"), py_str("hot water cylinder"), py_str("hot tank"), py_str("hot cylinder"), py_str("immersion"), py_str("immersion heater"), py_str("cylinder"), py_str("lagging")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("water-out-of-a-thing")); __args.push(py_tuple(vec![__data2(), g_WATER_OUT_ASK()])); __args.push(py_func("_water_out_of_a_thing", |args: &[Py]| _water_out_of_a_thing(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data3())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _water_out_of_a_thing(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The way out of the thing the line names, or the general way when it names none.\n\n    The table is in the order the things are told apart: a caravan has a water tank in it, so\n    the caravan comes before the tank, and \"how do i empty a caravan water tank\" gets the\n    caravan's drain plug and taps rather than the loft tank's float valve.\n    ");
    line = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    line = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    }
    let __iter1 = py_iter_or_raise(&g_WATER_OUT_OF());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    }
    return py_str("A bucket and gravity: bail most of it into a bucket, then syphon the rest with a garden hose kept full of water, with the outlet end lower than the water. Never tip a lot of water down a floor drain or a shower tray, which will not take it fast enough to keep up.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.water_out".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "WATER_OUT_ASK" => Some(g_WATER_OUT_ASK()),
            "WATER_OUT_OF" => Some(g_WATER_OUT_OF()),
            "_water_out_of_a_thing" => Some(py_func("_water_out_of_a_thing", |args: &[Py]| _water_out_of_a_thing(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "WATER_OUT_ASK" => s_WATER_OUT_ASK(v),
            "WATER_OUT_OF" => s_WATER_OUT_OF(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.water_out", module_obj);
    pyrt::xrefs::reg_fn("knowledge.water_out", "_water_out_of_a_thing", py_func("_water_out_of_a_thing", |args: &[Py]| _water_out_of_a_thing(args)));
    pyrt::xrefs::reg_access("knowledge.water_out", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.water_out", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.water_out", "WATER_OUT_ASK", g_WATER_OUT_ASK, s_WATER_OUT_ASK);
    pyrt::xrefs::reg_access("knowledge.water_out", "WATER_OUT_OF", g_WATER_OUT_OF, s_WATER_OUT_OF);
    pyrt::xrefs::reg_access("knowledge.water_out", "entry", g_entry, s_entry);
}

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
thread_local! { static G__ASKS_MATERIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_MATERIAL_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MATERIAL_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSONS_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAG_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAG_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BULK_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_JOBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONCRETE_SURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEPTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLOOR_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLOOR_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIND_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAST_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LENGTH_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MATERIAL_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SLAB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SQUARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__ASKS_MATERIAL() -> Py { ensure_init(); G__ASKS_MATERIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_MATERIAL(v: Py) { ensure_init(); G__ASKS_MATERIAL.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_MATERIAL_AVOID() -> Py { ensure_init(); G__ASKS_MATERIAL_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_MATERIAL_AVOID(v: Py) { ensure_init(); G__ASKS_MATERIAL_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION() -> Py { ensure_init(); G__A_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION(v: Py) { ensure_init(); G__A_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MATERIAL_WEIGHT() -> Py { ensure_init(); G__A_MATERIAL_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MATERIAL_WEIGHT(v: Py) { ensure_init(); G__A_MATERIAL_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NUMBER() -> Py { ensure_init(); G__A_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NUMBER(v: Py) { ensure_init(); G__A_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSONS_WEIGHT() -> Py { ensure_init(); G__A_PERSONS_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSONS_WEIGHT(v: Py) { ensure_init(); G__A_PERSONS_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__BAG_AVOID() -> Py { ensure_init(); G__BAG_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAG_AVOID(v: Py) { ensure_init(); G__BAG_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__BAG_WORD() -> Py { ensure_init(); G__BAG_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAG_WORD(v: Py) { ensure_init(); G__BAG_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__BULK_WEIGHT() -> Py { ensure_init(); G__BULK_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BULK_WEIGHT(v: Py) { ensure_init(); G__BULK_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_ASKS() -> Py { ensure_init(); G__CONCRETE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_ASKS(v: Py) { ensure_init(); G__CONCRETE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_AVOID() -> Py { ensure_init(); G__CONCRETE_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_AVOID(v: Py) { ensure_init(); G__CONCRETE_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_JOB() -> Py { ensure_init(); G__CONCRETE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_JOB(v: Py) { ensure_init(); G__CONCRETE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_JOBS() -> Py { ensure_init(); G__CONCRETE_JOBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_JOBS(v: Py) { ensure_init(); G__CONCRETE_JOBS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_NAMES() -> Py { ensure_init(); G__CONCRETE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_NAMES(v: Py) { ensure_init(); G__CONCRETE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__CONCRETE_SURE() -> Py { ensure_init(); G__CONCRETE_SURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONCRETE_SURE(v: Py) { ensure_init(); G__CONCRETE_SURE.with(|c| *c.borrow_mut() = v); }
pub fn g__DEPTH() -> Py { ensure_init(); G__DEPTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEPTH(v: Py) { ensure_init(); G__DEPTH.with(|c| *c.borrow_mut() = v); }
pub fn g__FEET() -> Py { ensure_init(); G__FEET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEET(v: Py) { ensure_init(); G__FEET.with(|c| *c.borrow_mut() = v); }
pub fn g__FLOOR_AVOID() -> Py { ensure_init(); G__FLOOR_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLOOR_AVOID(v: Py) { ensure_init(); G__FLOOR_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__FLOOR_NAMES() -> Py { ensure_init(); G__FLOOR_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLOOR_NAMES(v: Py) { ensure_init(); G__FLOOR_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__KIND_ASKS() -> Py { ensure_init(); G__KIND_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIND_ASKS(v: Py) { ensure_init(); G__KIND_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__LAST_TURNS() -> Py { ensure_init(); G__LAST_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAST_TURNS(v: Py) { ensure_init(); G__LAST_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g__LENGTH_NAMES() -> Py { ensure_init(); G__LENGTH_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LENGTH_NAMES(v: Py) { ensure_init(); G__LENGTH_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOSE() -> Py { ensure_init(); G__LOOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOSE(v: Py) { ensure_init(); G__LOOSE.with(|c| *c.borrow_mut() = v); }
pub fn g__MATERIAL_AVOID() -> Py { ensure_init(); G__MATERIAL_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MATERIAL_AVOID(v: Py) { ensure_init(); G__MATERIAL_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__NUM() -> Py { ensure_init(); G__NUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUM(v: Py) { ensure_init(); G__NUM.with(|c| *c.borrow_mut() = v); }
pub fn g__PAIR() -> Py { ensure_init(); G__PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAIR(v: Py) { ensure_init(); G__PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g__SLAB() -> Py { ensure_init(); G__SLAB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SLAB(v: Py) { ensure_init(); G__SLAB.with(|c| *c.borrow_mut() = v); }
pub fn g__SQUARE() -> Py { ensure_init(); G__SQUARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SQUARE(v: Py) { ensure_init(); G__SQUARE.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT() -> Py { ensure_init(); G__UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT(v: Py) { ensure_init(); G__UNIT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How much of a thing to buy: gravel, topsoil, compost, sand, turf, paving slabs, underlay,\ngrout and tile adhesive.\n\nThese are arithmetic questions, not advice questions. \"How much gravel do i need for my drive\",\n\"how many paving slabs for a patio\", \"how much topsoil do i need\", \"how much underlay for a 12\nsquare metre room\", \"how much grout do i need for a 12 square metre floor\" and \"how much tile\nadhesive do i need\" were each a shrug, a search hedge, the definition of area or the method for\nlaying tiles, and never a number. The rate per square metre is the fact; the line's own figures\nare the arithmetic, so one row per material answers every size of drive, bed, floor and pit.\n\nThe rates are the ones the trade works to: loose material at the depth it is usually laid, a bulk\nbag or a tonne turned into square metres, underlay in 15 square metre rolls and grout and\nadhesive by the bag, and a tenth more than the sums say because a bag never spreads as far as the\nlabel claims. Underlay, grout and adhesive are here rather than in home.py, which holds the paint\nand wallpaper quantities and is past the publisher's one-megabyte limit.\n\nThe second turn of a conversation that measured the floor names no size at all - \"the room is 4m\nby 3m how much carpet\" then \"and how much underlay\" - so the bag rows read the figure from the\nturns before when the line has none.\n");
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("garden-and-plants"), py_str("maths-and-numbers")]));
    s__NUM(py_str("(\\d+(?:\\.\\d+)?)"));
    s__PAIR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&g__NUM(), &py_str("\\s*(?:m|metres?|meters?|ft|feet|foot|')?\\s*(?:x|by|\\*|×|and)\\s*")), &g__NUM()), &py_str("\\s*(?:m|metres?|meters?|ft|feet|foot|')?"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SQUARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g__NUM(), &py_str("\\s*(?:square\\s*(?:metres?|meters?|foot|feet|ft)|sq\\s*m|sqm|m2|m²)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FEET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ft|feet|foot|'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d\\s*(?:m|metres?|meters?|ft|feet|foot|')")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DEPTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g__NUM(), &py_str("\\s*(mm|cm|in|inch|inches)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LOOSE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("gravel"), py_str("shingle"), py_str("chippings"), py_str("chips"), py_str("pea gravel"), py_str("pea shingle"), py_str("pebbles"), py_str("stones"), py_str("slate"), py_str("bark"), py_str("bark chippings"), py_str("woodchip"), py_str("wood chips"), py_str("mulch"), py_str("aggregate")]), Py::Int(50), Py::Int(10), py_str("A 850kg bulk bag of {thing} covers about 10 square metres at 50mm, and a 25kg bag about 0.3 square metres"), py_str("Put a weed membrane down first if you do not want to hoe it, and keep 20mm below the damp course on a wall so the rain can drain away.")]), py_tuple(vec![py_tuple(vec![py_str("topsoil"), py_str("soil"), py_str("loam")]), Py::Int(100), Py::Int(7), py_str("A 850kg bulk bag of {thing} (about 0.7 cubic metres) covers about 7 square metres at 100mm"), py_str("Screened topsoil, not subsoil, for anything you want to grow; it settles about a fifth, so fill to slightly proud of the level you want.")]), py_tuple(vec![py_tuple(vec![py_str("compost"), py_str("multipurpose compost"), py_str("manure"), py_str("composted manure")]), Py::Int(50), Py::Int(12), py_str("A 60 litre bag of {thing} covers about 1 square metre at 50mm, and a 145 litre bag about 2.5 square metres"), py_str("For a raised bed, fill the top 300mm with compost and the rest with rougher soil or rubble at the bottom; a bed one metre by two at 300mm deep takes about 6 bags of 60 litres.")]), py_tuple(vec![py_tuple(vec![py_str("sand"), py_str("play sand"), py_str("sharp sand"), py_str("builder's sand")]), Py::Int(50), Py::Float(0.6), py_str("A 25kg bag of {thing} covers about 0.6 square metres at 50mm, and about a tonne and a half fills a cubic metre"), py_str("Play sand, washed and graded, for a sandpit rather than builder's sand; cover it at night so cats and rain leave it alone.")])]));
    s__SLAB(py_tuple(vec![py_str("Paving slabs"), py_str("100mm of compacted MOT Type 1, then 30 to 40mm of sharp sand mixed 5 to 1 with cement as the bed")]));
    s__BAG_WORD(py_dict(vec![(py_str("sand"), py_str("bag")), (py_str("play sand"), py_str("bag")), (py_str("sharp sand"), py_str("bag")), (py_str("builder's sand"), py_str("bag"))]));
    s__A_NUMBER(py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine"))]));
    s__A_FRACTION(py_dict(vec![(Py::Float(0.25), py_str("a quarter")), (Py::Float(0.5), py_str("a half")), (Py::Float(0.75), py_str("three quarters"))]));
    s__ASKS_MATERIAL(py_tuple(vec![py_str("how much"), py_str("how many"), py_str("much "), py_str("many "), py_str("do i need"), py_str("will i need"), py_str("needed"), py_str("need for"), py_str("covers"), py_str("cover a"), py_str("how far does"), py_str("bags"), py_str("how much do i need"), py_str("tonne"), py_str("tonnes"), py_str("bulk bag")]));
    s__ASKS_MATERIAL_AVOID(py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("cheap"), py_str("cheapest"), py_str("how much does"), py_str("how much is"), py_str("how much are"), py_str("quid"), py_str("pounds"), py_str("£"), py_str("how do i lay"), py_str("how to lay"), py_str("how do i clean"), py_str("clean"), py_str("cleaning"), py_str("jet wash"), py_str("pressure wash"), py_str("how do i fix"), py_str("repair"), py_str("how do i stop weeds"), py_str("kill weeds")]));
    s__MATERIAL_AVOID(py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("cheap"), py_str("cheapest"), py_str("how much does"), py_str("how much is"), py_str("how much are"), py_str("quid"), py_str("pounds"), py_str("£"), py_str("how do i lay"), py_str("how to lay"), py_str("how do i clean"), py_str("clean"), py_str("cleaning"), py_str("jet wash"), py_str("pressure wash"), py_str("how do i fix"), py_str("repair"), py_str("how do i stop weeds"), py_str("kill weeds"), py_str("how deep"), py_str("how thick"), py_str("what depth"), py_str("mortar"), py_str("pointing"), py_str("render"), py_str("rendering"), py_str("cement"), py_str("screed"), py_str("concrete"), py_str("ballast"), py_str("block paving"), py_str("bedding"), py_str("patio base"), py_str("sand base"), py_str("for a base"), py_str("fish tank"), py_str("fish tank gravel"), py_str("aquarium"), py_str("tank"), py_str("pond"), py_str("filter"), py_str("weed membrane"), py_str("membrane"), py_str("landscape fabric"), py_str("sub-base"), py_str("type 1"), py_str("were "), py_str("was "), py_str("did i"), py_str("did we"), py_str("did he"), py_str("did she"), py_str("did they"), py_str("do i own"), py_str("do we own"), py_str("have i got"), py_str("have we got"), py_str("had left"), py_str("are left"), py_str("is left"), py_str("left in"), py_str("left over"), py_str("got left"), py_str("are on the"), py_str("is on the"), py_str("went down"), py_str("used to"), py_str("how long"), py_str("long does"), py_str("long will"), py_str("long do"), py_str("long should"), py_str("long to"), py_str("long before"), py_str("how many hours"), py_str("how many days"), py_str("how many minutes"), py_str("how much time"), py_str("wait before")]));
    s__A_PERSONS_WEIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lose|loses|losing|lost|shed|shift|drop|dropped|weigh|weighs|weighing|weight|put\\s+on|gained?|gaining|diet|diets|dieting|slim|slimming|bmi|calories|kilos?|kg|pounds|lbs?|pregnant)\\b[^.?!]{0,40}\\bstone\\b|\\b(?:half\\s+a|a\\s+stone\\s+and\\s+a\\s+half|two|three|1|2|3)\\s+stone\\b[^.?!]{0,40}\\b(?:weight|lose|lost|heavier|lighter|diet|target|goal)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MATERIAL_WEIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bulk\\s+bag|tonne|tonnes|ton\\s+of|tons\\s+of|bag\\s+of|bags\\s+of|cubic\\s+met(?:re|er)|how\\s+heavy|a\\s+load\\s+of)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BULK_WEIGHT(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("bark"), py_str("woodchip"), py_str("wood chip"), py_str("mulch")]), py_str("about 350kg to 400kg, and it fills close to a cubic metre and a half")]), py_tuple(vec![py_tuple(vec![py_str("compost"), py_str("soil"), py_str("topsoil"), py_str("loam")]), py_str("about 700kg to 750kg, which is roughly 0.7 of a cubic metre")]), py_tuple(vec![py_tuple(vec![py_str("sand"), py_str("ballast"), py_str("sharp sand"), py_str("grit sand")]), py_str("about 850kg, which is about half a cubic metre")]), py_tuple(vec![py_tuple(vec![py_str("slate"), py_str("pebbles"), py_str("pebble"), py_str("shingle"), py_str("chippings"), py_str("gravel"), py_str("aggregate"), py_str("stone"), py_str("stones")]), py_str("about 850kg, which is about half a cubic metre at 20mm")])]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-gravel")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gravel"), py_str("shingle"), py_str("chippings"), py_str("pea gravel"), py_str("pea shingle"), py_str("pebbles"), py_str("bark"), py_str("bark chippings"), py_str("woodchip"), py_str("mulch"), py_str("slate"), py_str("stones"), py_str("aggregate")]), g__ASKS_MATERIAL()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gravel")); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _loose_reply(args))(&__args) } })); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("guard".to_string(), g__not_a_persons_weight()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-topsoil")); __args.push(py_tuple(vec![py_tuple(vec![py_str("topsoil"), py_str("soil"), py_str("loam")]), g__ASKS_MATERIAL()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topsoil")); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _loose_reply(args))(&__args) } })); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-compost")); __args.push(py_tuple(vec![py_tuple(vec![py_str("compost"), py_str("multipurpose compost"), py_str("manure")]), g__ASKS_MATERIAL()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("compost")); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _loose_reply(args))(&__args) } })); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-sand")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sand"), py_str("play sand"), py_str("sharp sand")]), g__ASKS_MATERIAL()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sand")); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _loose_reply(args))(&__args) } })); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__not_a_sanding_job())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-heavy-is-a-bulk-bag")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bulk bag"), py_str("bulk bags"), py_str("tonne bag"), py_str("tonne bags"), py_str("builder's bag"), py_str("builders bag"), py_str("1 tonne bag"), py_str("one tonne bag"), py_str("a tonne"), py_str("cubic metre"), py_str("cubic yard")]), py_tuple(vec![py_str("weigh"), py_str("weighs"), py_str("weighing"), py_str("weight"), py_str("heavy"), py_str("how much"), py_str("how many kilos"), py_str("kilos"), py_str("kg"), py_str("what does"), py_str("tonne"), py_str("ton")])])); __args.push(g__bulk_bag_weight()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("heavy"), py_str("how much"), py_str("kilos"), py_str("kg")])), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("cheap"), py_str("how much does it cost"), py_str("delivery"), py_str("deliver"), py_str("hire"), py_str("skip"), py_str("van"), py_str("trailer"), py_str("car"), py_str("boot"), py_str("lift"), py_str("move"), py_str("carry"), py_str("bag of cement"), py_str("sandbag"), py_str("dog food")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sand-and-cement-for-a-base")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sand and cement"), py_str("sand cement"), py_str("mortar mix"), py_str("patio base"), py_str("patio foundation"), py_str("base for a patio"), py_str("base for the patio"), py_str("sand base"), py_str("sub base"), py_str("sub-base"), py_str("type 1"), py_str("type one"), py_str("slab base"), py_str("bedding for slabs"), py_str("under the slabs"), py_str("under patio slabs"), py_str("under the paving"), py_str("under a patio")]), g__ASKS_MATERIAL()])); __args.push(py_str("A patio slab sits on a full bed of mortar, not on sand alone and never on five dabs: 40mm to 50mm of sharp sand mixed 4 to 1 with cement, so about half a cubic metre of sharp sand for 10 square metres - roughly 800kg, most of an 850kg bulk bag - with 5 bags of 25kg cement. Double both for 20 square metres. Mix it stiff enough to hold the slab when you tap it down with a mallet and lay to a string line, and under a patio on soil put 75mm to 100mm of compacted type 1 down first, which is the part that stops it sinking. Sand alone shifts and washes out, which is why patios laid on it go wobbly in a couple of winters.")); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), op_add(&g__ASKS_MATERIAL_AVOID(), &py_tuple(vec![py_str("block paving"), py_str("block paved"), py_str("block paver"), py_str("block pavers"), py_str("paving blocks"), py_str("driveway blocks"), py_str("setts"), py_str("block paving drive"), py_str("paved drive"), py_str("paving the drive"), py_str("wall"), py_str("walls"), py_str("garden wall"), py_str("brick"), py_str("bricks"), py_str("brickwork"), py_str("bricklaying"), py_str("render"), py_str("rendering"), py_str("block wall"), py_str("breeze block"), py_str("footing"), py_str("footings")]))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-many-slabs")); __args.push(py_tuple(vec![py_tuple(vec![py_str("slabs"), py_str("paving slabs"), py_str("paving"), py_str("pavers"), py_str("flags"), py_str("patio slabs"), py_str("block paving"), py_str("paving stones")]), g__ASKS_MATERIAL()])); __args.push(g__slabs_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-turf")); __args.push(py_tuple(vec![py_tuple(vec![py_str("turf"), py_str("rolls of turf")]), g__ASKS_MATERIAL()])); __args.push(g__turf_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__MATERIAL_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    s__LAST_TURNS(Py::Int(3));
    s__BAG_AVOID(op_add(&g__MATERIAL_AVOID(), &py_tuple(vec![py_str("coming out"), py_str("come out"), py_str("crumbling"), py_str("crumbles"), py_str("crumbled"), py_str("powdery"), py_str("discoloured"), py_str("discolored"), py_str("stained"), py_str("staining"), py_str("mouldy"), py_str("mildew"), py_str("re do"), py_str("redo"), py_str("re-do"), py_str("hack out"), py_str("chisel"), py_str("hollow"), py_str("wont set"), py_str("won't set"), py_str("not setting"), py_str("keeps coming"), py_str("lifting"), py_str("cracked"), py_str("cracking"), py_str("loose"), py_str("uneven"), py_str("squeak"), py_str("squeaking"), py_str("underfloor heating"), py_str("wet room"), py_str("tile cutter"), py_str("spacers"), py_str("which tile"), py_str("what tile")])));
    s__FLOOR_NAMES(py_tuple(vec![py_str("carpet"), py_str("carpets"), py_str("carpeting"), py_str("laminate"), py_str("laminate flooring"), py_str("wood flooring"), py_str("engineered wood"), py_str("vinyl"), py_str("vinyl flooring"), py_str("lino"), py_str("linoleum"), py_str("flooring"), py_str("skirting"), py_str("skirting board"), py_str("skirting boards"), py_str("coving"), py_str("dado rail"), py_str("dado rails"), py_str("floor tiles"), py_str("tile floor")]));
    s__LENGTH_NAMES(py_tuple(vec![py_str("skirting"), py_str("skirting board"), py_str("skirting boards"), py_str("coving"), py_str("dado rail"), py_str("dado rails")]));
    s__FLOOR_AVOID(op_add(&g__MATERIAL_AVOID(), &py_tuple(vec![py_str("how much left"), py_str("left over"), py_str("last"), py_str("lasts"), py_str("how long does")])));
    s__CONCRETE_NAMES(py_tuple(vec![py_str("concrete"), py_str("postcrete"), py_str("post mix"), py_str("postmix"), py_str("cement"), py_str("ballast"), py_str("concrete mix"), py_str("ready mix"), py_str("readymix"), py_str("mot type 1"), py_str("hardcore"), py_str("sub base"), py_str("subbase")]));
    s__CONCRETE_ASKS(op_add(&g__ASKS_MATERIAL(), &py_tuple(vec![py_str("bags of"), py_str("how many bags"), py_str("how much concrete"), py_str("how much ballast"), py_str("how much cement"), py_str("how much postcrete")])));
    s__KIND_ASKS(py_tuple(vec![py_str("which"), py_str("what kind"), py_str("what type"), py_str("best"), py_str("recommend"), py_str("suitable"), py_str("should i get"), py_str("should i use"), py_str("what concrete"), py_str("what postcrete"), py_str("what cement"), py_str("what mortar"), py_str("what ballast"), py_str("what mix"), py_str("what ratio")]));
    s__CONCRETE_JOBS(py_tuple(vec![py_str("fence post"), py_str("fence posts"), py_str("gate post"), py_str("post"), py_str("posts"), py_str("post hole"), py_str("post holes"), py_str("shed"), py_str("shed base"), py_str("sheds"), py_str("base"), py_str("bases"), py_str("slab"), py_str("slabs"), py_str("foundation"), py_str("foundations"), py_str("footing"), py_str("footings"), py_str("drive"), py_str("driveway"), py_str("path"), py_str("patio"), py_str("wall"), py_str("garage")]));
    s__CONCRETE_AVOID(py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("cheap"), py_str("cheapest"), py_str("how much does"), py_str("how much is"), py_str("how much are"), py_str("quid"), py_str("pounds"), py_str("£"), py_str("quote"), py_str("hire"), py_str("skip"), py_str("cracked"), py_str("cracking"), py_str("crumbly"), py_str("dusty"), py_str("damp"), py_str("rising damp"), py_str("render"), py_str("rendering"), py_str("plaster"), py_str("mortar"), py_str("pointing"), py_str("screed"), py_str("driveway sealer"), py_str("seal"), py_str("repair"), py_str("patch"), py_str("filler"), py_str("glue"), py_str("resin"), py_str("anchor"), py_str("bolt"), py_str("broken"), py_str("sinking"), py_str("sunken"), py_str("uneven"), py_str("pothole"), py_str("weeds"), py_str("acid"), py_str("acidic"), py_str("alkaline"), py_str("alkali"), py_str("ph"), py_str("litmus"), py_str("neutralise"), py_str("neutralize"), py_str("chemical"), py_str("chemicals"), py_str("erosion"), py_str("co2"), py_str("carbon"), py_str("emissions"), py_str("cure"), py_str("curing"), py_str("brick"), py_str("bricks"), py_str("block"), py_str("blocks"), py_str("bricklaying"), py_str("brickwork"), py_str("masonry")]));
    s__CONCRETE_SURE(py_tuple(vec![py_str("concrete"), py_str("postcrete"), py_str("post mix"), py_str("postmix"), py_str("concrete mix"), py_str("ready mix"), py_str("readymix"), py_str("cement"), py_str("mortar")]));
    s__CONCRETE_JOB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fence|gate|post|posts|hole|holes|shed|sheds|base|bases|slab|slabs|footing|footings|foundation|foundations|drive|driveway|driveways|path|paths|patio|patios|wall|walls|garage|garages|floor|floors)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __aug1 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-concrete")); __args.push(py_tuple(vec![g__CONCRETE_NAMES(), op_add(&g__CONCRETE_ASKS(), &g__CONCRETE_JOBS())])); __args.push(g__concrete_reply()); __kws.extend(vec![("ask".to_string(), op_add(&g__CONCRETE_ASKS(), &g__KIND_ASKS())), ("avoid".to_string(), g__CONCRETE_AVOID()), ("guard".to_string(), g__concrete_is_what_is_asked()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-underlay")); __args.push(py_tuple(vec![py_tuple(vec![py_str("underlay"), py_str("underlayment"), py_str("underlay roll")]), g__ASKS_MATERIAL()])); __args.push(g__underlay_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__BAG_AVOID()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-grout")); __args.push(py_tuple(vec![py_tuple(vec![py_str("grout"), py_str("grouting")]), g__ASKS_MATERIAL()])); __args.push(g__grout_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__BAG_AVOID()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-tile-adhesive")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tile adhesive"), py_str("adhesive"), py_str("tile cement"), py_str("thin set"), py_str("thinset"), py_str("tile glue")]), g__ASKS_MATERIAL()])); __args.push(g__adhesive_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__BAG_AVOID()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-much-flooring")); __args.push(py_tuple(vec![g__FLOOR_NAMES(), g__ASKS_MATERIAL()])); __args.push(g__floor_reply()); __kws.extend(vec![("ask".to_string(), g__ASKS_MATERIAL()), ("avoid".to_string(), g__FLOOR_AVOID()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    s_ENTRIES(__aug1);
}
pub fn _area(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut area: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut square: Py = Py::None;
    let mut two: Py = Py::None;
    let _ = py_str("The area in square metres the line asks about, or None when it names no size.");
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PAIR(), "search", args))(&__args) };
    if py_truthy(&{ let __b = pair.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNIT(), "search", args))(&__args) } } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    one = __unpk1[0].clone();
    two = __unpk1[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FEET(), "search", args))(&__args) }) {
    let __unpk2 = py_unpack(&py_tuple(vec![op_mul(&one.clone(), &Py::Float(0.3048)), op_mul(&two.clone(), &Py::Float(0.3048))]), 2, None);
    one = __unpk2[0].clone();
    two = __unpk2[1].clone();
    }
    area = op_mul(&one.clone(), &two.clone());
    if py_truthy(&{ let __cmp31: Py = area.clone(); if py_truthy(&op_le(&Py::Float(0.2), &__cmp31)) { op_le(&__cmp31, &Py::Int(20000)) } else { Py::Bool(false) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    square = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SQUARE(), "search", args))(&__args) };
    if py_truthy(&square.clone()) {
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&square.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&square.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FEET(), "search", args))(&__args) }) {
    area = op_mul(&area.clone(), &Py::Float(0.0929));
    }
    if py_truthy(&{ let __cmp41: Py = area.clone(); if py_truthy(&op_le(&Py::Float(0.2), &__cmp41)) { op_le(&__cmp41, &Py::Int(20000)) } else { Py::Bool(false) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _depth_mm(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut default: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "default") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: default"))) };
    let mut amount: Py = Py::None;
    let mut depth: Py = Py::None;
    let mut found: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The depth in the line in millimetres, or the material's usual depth.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DEPTH(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return default.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }]), 2, None);
    amount = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "startswith", args))(&__args) }) {
    depth = amount.clone();
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "startswith", args))(&__args) }) {
    depth = op_mul(&amount.clone(), &Py::Int(10));
    } else {
    depth = op_mul(&amount.clone(), &Py::Int(25));
    }
    }
    if py_truthy(&{ let __cmp21: Py = depth.clone(); if py_truthy(&op_le(&Py::Int(10), &__cmp21)) { op_le(&__cmp21, &Py::Int(400)) } else { Py::Bool(false) } }) {
    return depth.clone();
    }
    return default.clone();
    Py::None
}
pub fn _sizes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("A number of square metres as a person writes it: 12, not 12.0.");
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&value.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(0.05))) {
    return op_modulo(&py_str("%d"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("0")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_modulo(&py_str("%.1f"), &value.clone()), "rstrip", args))(&__args) }, "rstrip", args))(&__args) };
    Py::None
}
pub fn _a_bag_count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut bags: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bags") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bags"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut part: Py = Py::None;
    let mut quarters: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("How many bags to buy, said the way a person would: \"about half a bulk bag\".\n\n    A count was said as a decimal, so a small job was told \"4.5 square metres at 50mm needs\n    about 0.5 bulk bags of gravel\" - a bag you cannot buy, and a half with an s on it. The\n    quarters are the ones people say: a quarter, half, three quarters, and one and a half.\n    ");
    quarters = op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&bags.clone(), &Py::Int(4))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(4.0));
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(quarters.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&quarters.clone(), &whole.clone())); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&quarters.clone(), &Py::Float(0.25))) {
    return op_modulo(&py_str("about a tenth of a %s"), &word.clone());
    }
    if py_truthy(&op_lt(&quarters.clone(), &Py::Int(1))) {
    if py_truthy(&Py::Bool(py_eq(&part.clone(), &Py::Float(0.25)))) {
    return op_modulo(&py_str("about a quarter of a %s"), &word.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&part.clone(), &Py::Float(0.75)))) {
    return op_modulo(&py_str("about three quarters of a %s"), &word.clone());
    }
    return op_modulo(&py_str("about half a %s"), &word.clone());
    }
    if py_truthy(&op_ge(&quarters.clone(), &Py::Float(9.5))) {
    return op_modulo(&py_str("about %d %ss"), &py_tuple(vec![if py_truthy(&op_lt(&bags.clone(), &Py::Int(20))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bags.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bags.clone()); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } }, word.clone()]));
    }
    if py_truthy(&Py::Bool(py_eq(&part.clone(), &Py::Float(0.75)))) {
    let __unpk1 = py_unpack(&py_tuple(vec![op_add(&whole.clone(), &Py::Int(1)), Py::Int(0)]), 2, None);
    whole = __unpk1[0].clone();
    part = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) {
    return op_modulo(&py_str("about one %s"), &word.clone());
    }
    return op_modulo(&py_str("about %s %ss"), &py_tuple(vec![geti(&g__A_NUMBER(), &whole.clone()), word.clone()]));
    }
    if py_truthy(&part.clone()) {
    return op_modulo(&py_str("about %s and %s %ss"), &py_tuple(vec![geti(&g__A_NUMBER(), &whole.clone()), geti(&g__A_FRACTION(), &part.clone()), word.clone()]));
    }
    if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) {
    return op_modulo(&py_str("about one %s"), &word.clone());
    }
    return op_modulo(&py_str("about %s %ss"), &py_tuple(vec![geti(&g__A_NUMBER(), &whole.clone()), word.clone()]));
    Py::None
}
pub fn _count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut halves: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How a person would say a number of things: \"about two and a half\", not \"2.5\".");
    if py_truthy(&op_ge(&value.clone(), &Py::Int(20))) {
    return op_modulo(&py_str("about %d"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    if py_truthy(&op_ge(&value.clone(), &Py::Int(4))) {
    return op_modulo(&py_str("about %d"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    halves = op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&value.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2));
    if py_truthy(&Py::Bool(py_eq(&halves.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(halves.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(halves.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_NUMBER(), "get", args))(&__args) };
    if py_truthy(&word.clone()) {
    return op_modulo(&py_str("about %s"), &word.clone());
    }
    }
    return op_modulo(&py_str("about %s"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("0")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_modulo(&py_str("%.1f"), &halves.clone()), "rstrip", args))(&__args) }, "rstrip", args))(&__args) });
    Py::None
}
pub fn _loose_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut area: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut bags: Py = Py::None;
    let mut covers: Py = Py::None;
    let mut depth: Py = Py::None;
    let mut extra: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut name_word: Py = Py::None;
    let mut names: Py = Py::None;
    let mut per_bag: Py = Py::None;
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The bags one loose material needs at the depth the line asks for.");
    text = op_add(&op_add(&py_str(" "), &{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    let __unpk1 = py_unpack(&geti(&g__LOOSE(), &Py::Int(0)), 5, None);
    names = __unpk1[0].clone();
    depth = __unpk1[1].clone();
    covers = __unpk1[2].clone();
    lead = __unpk1[3].clone();
    extra = __unpk1[4].clone();
    let __iter2 = py_iter_or_raise(&g__LOOSE());
    for __item in __iter2 {
    row = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&geti(&row.clone(), &Py::Int(0)), &name.clone()))) {
    let __unpk3 = py_unpack(&row.clone(), 5, None);
    names = __unpk3[0].clone();
    depth = __unpk3[1].clone();
    covers = __unpk3[2].clone();
    lead = __unpk3[3].clone();
    extra = __unpk3[4].clone();
    break;
    }
    }
    thing = geti(&names.clone(), &Py::Int(0));
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    name_word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &name_word.clone()))) {
    thing = name_word.clone();
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&thing.clone(), &py_str("sand"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("sandpit"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("sand pit"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("play sand"))) } } } } } }) {
    thing = py_str("play sand");
    lead = py_str("A 25kg bag of play sand covers about 0.6 square metres at 50mm, so a 1.2m square sandpit filled to 150mm takes about 10 bags");
    }
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) };
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(depth.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _depth_mm(args))(&__args) };
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    per_bag = op_mul(&covers.clone(), &op_truediv(&depth.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    if py_truthy(&area.clone()) {
    bags = op_truediv(&area.clone(), &per_bag.clone());
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); __args.push(py_str("bulk bag")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BAG_WORD(), "get", args))(&__args) };
    return op_modulo(&py_str("%s square metres at %dmm needs %s of %s. %s, and it is worth a tenth more than the sums say. %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, asked.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bags.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_bag_count(args))(&__args) }, thing.clone(), lead.clone(), extra.clone()]));
    }
    return op_modulo(&py_str("%s. Measure the area and work out the volume at the depth you want, then divide by what a bag holds, and order a tenth more than the sums say - a bag never spreads as far as the label claims. %s"), &py_tuple(vec![lead.clone(), extra.clone()]));
    Py::None
}
pub fn _slabs_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut one: Py = Py::None;
    let mut per_square: Py = Py::None;
    let mut side: Py = Py::None;
    let mut size: Py = Py::None;
    let mut text: Py = Py::None;
    let mut two: Py = Py::None;
    text = op_add(&op_add(&py_str(" "), &{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    per_square = Py::Float(4.0);
    size = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&g__NUM(), &py_str("\\s*(?:mm)?\\s*(?:x|by)\\s*")), &g__NUM()), &py_str("\\s*(?:mm|cm)?"))); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&size.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&size.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&size.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    one = __unpk1[0].clone();
    two = __unpk1[1].clone();
    if py_truthy(&op_lt(&one.clone(), &Py::Int(20))) {
    let __aug2 = op_mul(&one.clone(), &Py::Int(1000));
    one = __aug2;
    }
    if py_truthy(&op_lt(&two.clone(), &Py::Int(20))) {
    let __aug3 = op_mul(&two.clone(), &Py::Int(1000));
    two = __aug3;
    }
    if py_truthy(&{ let __b = { let __cmp41: Py = one.clone(); if py_truthy(&op_le(&Py::Int(200), &__cmp41)) { op_le(&__cmp41, &Py::Int(1200)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let __cmp51: Py = two.clone(); if py_truthy(&op_le(&Py::Int(200), &__cmp51)) { op_le(&__cmp51, &Py::Int(1200)) } else { Py::Bool(false) } } } }) {
    per_square = op_truediv(&Py::Float(1000000.0), &op_mul(&one.clone(), &two.clone()));
    }
    }
    side = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_truediv(&Py::Int(1000), &op_pow(&per_square.clone(), &Py::Float(0.5)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&area.clone()) {
    return op_modulo(&py_str("%s square metres needs %s slabs of %dmm, working at %.1f to the square metre, with a tenth spare for cuts and breakages. Under them: %s."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&op_mul(&area.clone(), &per_square.clone()), &Py::Float(1.1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) }, side.clone(), per_square.clone(), geti(&g__SLAB(), &Py::Int(1))]));
    }
    return op_modulo(&py_str("Work it out at four 500mm slabs to the square metre, six of 400mm and five and a half of 600 x 300, then add a tenth for cuts and breakages. Under them: %s."), &geti(&g__SLAB(), &Py::Int(1)));
    Py::None
}
pub fn _turf_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut text: Py = Py::None;
    text = op_add(&op_add(&py_str(" "), &{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    if py_truthy(&area.clone()) {
    return op_modulo(&py_str("%s square metres is %s rolls of turf, ordered a few extra for the edges and any that go thin. Lay it within a day of delivery, water it the same evening, and keep off it for a fortnight."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&area.clone(), &Py::Float(1.05))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) }]));
    }
    return py_str("Turf rolls cover about a square metre each, so the area in square metres is the number of rolls, plus a few for the edges and any that go thin. Lay it within a day of delivery, water it the same evening, and keep off it for a fortnight.");
    Py::None
}
pub fn _bulk_bag_weight(args: &[Py]) -> Py {
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
    let mut figures: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut line: Py = Py::None;
    let mut n: Py = Py::None;
    let mut names: Py = Py::None;
    let mut text: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("What one bulk bag weighs, by what is in it.");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "));
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("gravel or sand"), py_str("about 850kg, which is about half a cubic metre")]), 2, None);
    what = __unpk1[0].clone();
    figures = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g__BULK_WEIGHT());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    names = __unpk3[0].clone();
    line = __unpk3[1].clone();
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut n: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &n.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &n.clone()), &py_str("s ")))) } }) {  __out.push(n.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&hit.clone()) {
    let __unpk4 = py_unpack(&py_tuple(vec![hit.clone(), line.clone()]), 2, None);
    what = __unpk4[0].clone();
    figures = __unpk4[1].clone();
    break;
    }
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("A bulk bag of "), &what.clone()), &py_str(" is ")), &figures.clone()), &py_str(" - the bags sold as a tonne bag are filled to about 850kg for stone and sand, because that is what the bag and a lorry will take, so buy by the bag rather than by the tonne if the weight matters. That is the same figure the coverage sums use: 850kg of gravel covers about 10 square metres at 50mm. A 25kg bag is the same material in a bag you can lift, so 34 of them make a bulk bag."));
    Py::None
}
pub fn _not_a_persons_weight(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("False when the stone in the line is a weight rather than a bag of them.");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MATERIAL_WEIGHT(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERSONS_WEIGHT(), "search", args))(&__args) }));
    Py::None
}
pub fn _not_a_sanding_job(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is about buying sand rather than sanding something down.");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::a_sanding_job(args))(&__args) }));
    Py::None
}
pub fn _asked_area(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut key: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The area this line is about: from the line, else from the last few turns.\n\n    A floor gets measured in one turn and bought for in the next - \"the room is 4m by 3m how\n    much carpet\" then \"and how much underlay\" - and the second line names no size at all, so the\n    figure from the turn before is the one being asked about.\n    ");
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    if py_truthy(&area.clone()) {
    return area.clone();
    }
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&g__LAST_TURNS()), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("reply"), py_str("user")]));
    for __item in __iter2 {
    key = __item.clone();
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    if py_truthy(&area.clone()) {
    return area.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _rolls(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut area: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "area") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: area"))) };
    let mut covers: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covers"))) };
    let mut spare: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spare") { v.clone() }
    else { Py::Float(1.1) };
    let _ = py_str("How many rolls or bags of `covers` square metres each an area of `area` needs.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_neg(&op_floordiv(&op_neg(&op_mul(&area.clone(), &spare.clone())), &covers.clone()))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _underlay_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut how: Py = Py::None;
    let mut rolls: Py = Py::None;
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_area(args))(&__args) };
    how = py_str("Roll it across the room rather than along it, butt the seams without overlapping and tape the joins. Foam crumb is the cheap default; rubber crumb or felt is worth it under a thin carpet and over a hard floor.");
    if py_truthy(&area.clone()) {
    rolls = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); __args.push(Py::Int(15)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _rolls(args))(&__args) };
    return op_modulo(&py_str("%s square metres needs %s roll%s of underlay, a roll being 15 square metres (1.37m by 11m), and the tenth extra covers the cuts round the edges and under the doorway. %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rolls.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) }, if py_truthy(&Py::Bool(py_eq(&rolls.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, how.clone()]));
    }
    return op_modulo(&py_str("Underlay is sold by area: a roll is usually 15 square metres (1.37m by 11m), so measure the floor and add a tenth for the cuts. %s"), &how.clone());
    Py::None
}
pub fn _grout_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut bags: Py = Py::None;
    let mut how: Py = Py::None;
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_area(args))(&__args) };
    how = py_str("Mix it to the thickness of toothpaste and only as much as you can use in half an hour, because it goes off in the bucket.");
    if py_truthy(&area.clone()) {
    bags = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); __args.push(Py::Float(4.5)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _rolls(args))(&__args) };
    return op_modulo(&py_str("%s square metres needs %s bag%s of grout, 5kg each: a bag does 4 to 5 square metres of 30cm tiles with 3mm joints, and small tiles or wide joints take about twice as much. %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bags.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) }, if py_truthy(&Py::Bool(py_eq(&bags.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, how.clone()]));
    }
    return op_modulo(&py_str("A 5kg bag of grout does about 4 to 5 square metres of 30cm tiles with 3mm joints, so a bathroom floor is one bag and a kitchen wall two; mosaic or wide joints take about twice as much for the same area. %s"), &how.clone());
    Py::None
}
pub fn _adhesive_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut bags: Py = Py::None;
    let mut how: Py = Py::None;
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_area(args))(&__args) };
    how = py_str("Comb it out in one direction, press each tile on across the ridges, and lift one now and then to check the back is covered.");
    if py_truthy(&area.clone()) {
    bags = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); __args.push(Py::Float(4.5)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _rolls(args))(&__args) };
    return op_modulo(&py_str("%s square metres needs %s bag%s of tile adhesive, 20kg each: a bag covers 4 to 5 square metres with a 6mm notched trowel, and large tiles or a deeper notch take it nearer 3. %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bags.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) }, if py_truthy(&Py::Bool(py_eq(&bags.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, how.clone()]));
    }
    return op_modulo(&py_str("A 20kg bag of tile adhesive covers about 4 to 5 square metres with a 6mm notched trowel, so a small bathroom wall is one bag; large tiles, a 10mm notch or back-buttering take it nearer 3 square metres a bag. %s"), &how.clone());
    Py::None
}
pub fn _floor_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut around: Py = Py::None;
    let mut boards: Py = Py::None;
    let mut count: Py = Py::None;
    let mut one: Py = Py::None;
    let mut packs: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut text: Py = Py::None;
    let mut two: Py = Py::None;
    let mut waste: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Carpet, laminate, vinyl and skirting: something bought by the area or by the length.");
    text = op_add(&op_add(&py_str(" "), &{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PAIR(), "search", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__LENGTH_NAMES()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    if py_truthy(&pair.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    one = __unpk1[0].clone();
    two = __unpk1[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FEET(), "search", args))(&__args) }) {
    let __unpk2 = py_unpack(&py_tuple(vec![op_mul(&one.clone(), &Py::Float(0.3048)), op_mul(&two.clone(), &Py::Float(0.3048))]), 2, None);
    one = __unpk2[0].clone();
    two = __unpk2[1].clone();
    }
    around = op_mul(&Py::Int(2), &op_add(&one.clone(), &two.clone()));
    boards = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_truediv(&around.clone(), &Py::Float(2.4))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1));
    return op_modulo(&py_str("A room %.1fm by %.1fm has %.1f metres round it, so %.1f metres once a couple of doorways are taken off - and boards come in 2.4m lengths, so buy %d of them. Cut the joints where two boards meet over a straight bit of wall, because the joint shows less there than at a corner."), &py_tuple(vec![one.clone(), two.clone(), around.clone(), op_sub(&around.clone(), &Py::Float(1.6)), boards.clone()]));
    }
    return py_str("Skirting, coving and dado rail are bought by the length, not the area: run a tape right round the room and take off the doorways, about 0.8m each. A doorway takes the skirting away and the coving carries on over it. Boards come in 2.4m lengths, so buy whole boards and join them over a straight run, and add one board for the corners and the cuts.");
    }
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _area(args))(&__args) };
    if py_truthy(&area.clone()) {
    waste = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&area.clone(), &Py::Float(1.1))); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    packs = op_truediv(&op_mul(&area.clone(), &Py::Float(1.1)), &Py::Float(2.2));
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(packs.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _count(args))(&__args) };
    return op_modulo(&py_str("%s square metres needs %s square metres of carpet or flooring with the tenth spare for the cuts and the doorway, which is %s packs of laminate at about 2.2 square metres a pack. A carpet roll is 4m or 5m wide, which is why the shop asks for the longest and widest measurement of each room rather than the area alone: a room under 4m wide comes out of one piece, across the width of the roll, and a wider one needs a join. Keep the offcuts, because a spare pack for a repair later is worth more than the few pounds it costs."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(waste.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, count.clone()]));
    }
    return py_str("Measure each room on the floor, not on the plan: length times width in metres, and an L-shaped room as two rectangles added together. Add a tenth for the cuts and the doorway, so 12 square metres of floor takes 13.2 square metres of carpet, or six packs of laminate at 2.2 square metres a pack. A carpet roll is 4m or 5m wide, so the shop wants the longest and the widest measurement of each room as well as the area, and a room wider than the roll needs a join.");
    Py::None
}
pub fn _concrete_is_what_is_asked(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line is loose ballast or hardcore with no job named, which is the volume row.\n\n    \"how much ballast do i need\" and \"how much hardcore do i need\" are answered by the material\n    row that gives a tonne per cubic metre, and they were before this one existed. \"how much\n    ballast for a shed base\" is not: it was the price of a shed, and it needs the volume of a\n    base.\n    ");
    said = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text")])) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CONCRETE_SURE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CONCRETE_JOB(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _concrete_reply(args: &[Py]) -> Py {
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
    let mut area: Py = Py::None;
    let mut ballast: Py = Py::None;
    let mut cement: Py = Py::None;
    let mut mixing: Py = Py::None;
    let mut text: Py = Py::None;
    let mut volume: Py = Py::None;
    text = op_add(&op_add(&py_str(" "), &{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    mixing = py_str("Concrete is one part cement to six of ballast by volume - six shovels of ballast to one of cement, mixed dry first, then water until it holds together - and a cubic metre takes about 1.75 tonnes of ballast and six or seven 25kg bags of cement, which is two bulk bags of ballast.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fence|gate|post|posts|hole|holes|postcrete|post mix|postmix)\\b")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return op_modulo(&py_str("One 20kg bag of Postcrete sets one post: the hole wants to be about 250 to 300mm across and 600mm (two foot) deep, with 50 to 75mm of gravel in the bottom for drainage. Stand the post, check it upright with a level on two sides, pour the dry mix in and add the water on top of it. A thicker post, a gate post or a hole much wider than that takes two bags. %s"), &mixing.clone());
    }
    area = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_area(args))(&__args) };
    if py_truthy(&area.clone()) {
    volume = op_mul(&area.clone(), &op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(Py::Int(100)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _depth_mm(args))(&__args) }, &Py::Float(1000.0)));
    ballast = op_mul(&volume.clone(), &Py::Float(1.75));
    cement = op_mul(&volume.clone(), &Py::Float(6.5));
    return op_modulo(&py_str("%s square metres at %dmm is %.2f cubic metres of concrete: about %.1f tonnes of ballast and %d bags of cement, or %.0f 25kg bags of ready-mixed concrete. %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(area.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sizes(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(Py::Int(100)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _depth_mm(args))(&__args) }, volume.clone(), ballast.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_neg(&op_floordiv(&op_neg(&cement.clone()), &Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_neg(&op_floordiv(&op_neg(&op_truediv(&op_mul(&volume.clone(), &Py::Int(2400)), &Py::Int(25))), &Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, mixing.clone()]));
    }
    return op_modulo(&py_str("Work it out as a volume: length times width times depth in metres, so a base 100mm thick takes 0.1 cubic metres for every square metre, and a 6 by 8 foot shed is about 4.5 square metres - 0.45 cubic metres, which is one 850kg bulk bag of ballast and three 25kg bags of cement. A single fence post is one 20kg bag of postcrete. %s"), &mixing.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.materials".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASKS_MATERIAL" => Some(g__ASKS_MATERIAL()),
            "_ASKS_MATERIAL_AVOID" => Some(g__ASKS_MATERIAL_AVOID()),
            "_A_FRACTION" => Some(g__A_FRACTION()),
            "_A_MATERIAL_WEIGHT" => Some(g__A_MATERIAL_WEIGHT()),
            "_A_NUMBER" => Some(g__A_NUMBER()),
            "_A_PERSONS_WEIGHT" => Some(g__A_PERSONS_WEIGHT()),
            "_BAG_AVOID" => Some(g__BAG_AVOID()),
            "_BAG_WORD" => Some(g__BAG_WORD()),
            "_BULK_WEIGHT" => Some(g__BULK_WEIGHT()),
            "_CONCRETE_ASKS" => Some(g__CONCRETE_ASKS()),
            "_CONCRETE_AVOID" => Some(g__CONCRETE_AVOID()),
            "_CONCRETE_JOB" => Some(g__CONCRETE_JOB()),
            "_CONCRETE_JOBS" => Some(g__CONCRETE_JOBS()),
            "_CONCRETE_NAMES" => Some(g__CONCRETE_NAMES()),
            "_CONCRETE_SURE" => Some(g__CONCRETE_SURE()),
            "_DEPTH" => Some(g__DEPTH()),
            "_FEET" => Some(g__FEET()),
            "_FLOOR_AVOID" => Some(g__FLOOR_AVOID()),
            "_FLOOR_NAMES" => Some(g__FLOOR_NAMES()),
            "_KIND_ASKS" => Some(g__KIND_ASKS()),
            "_LAST_TURNS" => Some(g__LAST_TURNS()),
            "_LENGTH_NAMES" => Some(g__LENGTH_NAMES()),
            "_LOOSE" => Some(g__LOOSE()),
            "_MATERIAL_AVOID" => Some(g__MATERIAL_AVOID()),
            "_NUM" => Some(g__NUM()),
            "_PAIR" => Some(g__PAIR()),
            "_SLAB" => Some(g__SLAB()),
            "_SQUARE" => Some(g__SQUARE()),
            "_UNIT" => Some(g__UNIT()),
            "_a_bag_count" => Some(py_func("_a_bag_count", |args: &[Py]| _a_bag_count(args))),
            "_adhesive_reply" => Some(py_func("_adhesive_reply", |args: &[Py]| _adhesive_reply(args))),
            "_area" => Some(py_func("_area", |args: &[Py]| _area(args))),
            "_asked_area" => Some(py_func("_asked_area", |args: &[Py]| _asked_area(args))),
            "_bulk_bag_weight" => Some(py_func("_bulk_bag_weight", |args: &[Py]| _bulk_bag_weight(args))),
            "_concrete_is_what_is_asked" => Some(py_func("_concrete_is_what_is_asked", |args: &[Py]| _concrete_is_what_is_asked(args))),
            "_concrete_reply" => Some(py_func("_concrete_reply", |args: &[Py]| _concrete_reply(args))),
            "_count" => Some(py_func("_count", |args: &[Py]| _count(args))),
            "_depth_mm" => Some(py_func("_depth_mm", |args: &[Py]| _depth_mm(args))),
            "_floor_reply" => Some(py_func("_floor_reply", |args: &[Py]| _floor_reply(args))),
            "_grout_reply" => Some(py_func("_grout_reply", |args: &[Py]| _grout_reply(args))),
            "_loose_reply" => Some(py_func("_loose_reply", |args: &[Py]| _loose_reply(args))),
            "_not_a_persons_weight" => Some(py_func("_not_a_persons_weight", |args: &[Py]| _not_a_persons_weight(args))),
            "_not_a_sanding_job" => Some(py_func("_not_a_sanding_job", |args: &[Py]| _not_a_sanding_job(args))),
            "_rolls" => Some(py_func("_rolls", |args: &[Py]| _rolls(args))),
            "_sizes" => Some(py_func("_sizes", |args: &[Py]| _sizes(args))),
            "_slabs_reply" => Some(py_func("_slabs_reply", |args: &[Py]| _slabs_reply(args))),
            "_turf_reply" => Some(py_func("_turf_reply", |args: &[Py]| _turf_reply(args))),
            "_underlay_reply" => Some(py_func("_underlay_reply", |args: &[Py]| _underlay_reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASKS_MATERIAL" => s__ASKS_MATERIAL(v),
            "_ASKS_MATERIAL_AVOID" => s__ASKS_MATERIAL_AVOID(v),
            "_A_FRACTION" => s__A_FRACTION(v),
            "_A_MATERIAL_WEIGHT" => s__A_MATERIAL_WEIGHT(v),
            "_A_NUMBER" => s__A_NUMBER(v),
            "_A_PERSONS_WEIGHT" => s__A_PERSONS_WEIGHT(v),
            "_BAG_AVOID" => s__BAG_AVOID(v),
            "_BAG_WORD" => s__BAG_WORD(v),
            "_BULK_WEIGHT" => s__BULK_WEIGHT(v),
            "_CONCRETE_ASKS" => s__CONCRETE_ASKS(v),
            "_CONCRETE_AVOID" => s__CONCRETE_AVOID(v),
            "_CONCRETE_JOB" => s__CONCRETE_JOB(v),
            "_CONCRETE_JOBS" => s__CONCRETE_JOBS(v),
            "_CONCRETE_NAMES" => s__CONCRETE_NAMES(v),
            "_CONCRETE_SURE" => s__CONCRETE_SURE(v),
            "_DEPTH" => s__DEPTH(v),
            "_FEET" => s__FEET(v),
            "_FLOOR_AVOID" => s__FLOOR_AVOID(v),
            "_FLOOR_NAMES" => s__FLOOR_NAMES(v),
            "_KIND_ASKS" => s__KIND_ASKS(v),
            "_LAST_TURNS" => s__LAST_TURNS(v),
            "_LENGTH_NAMES" => s__LENGTH_NAMES(v),
            "_LOOSE" => s__LOOSE(v),
            "_MATERIAL_AVOID" => s__MATERIAL_AVOID(v),
            "_NUM" => s__NUM(v),
            "_PAIR" => s__PAIR(v),
            "_SLAB" => s__SLAB(v),
            "_SQUARE" => s__SQUARE(v),
            "_UNIT" => s__UNIT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.materials", module_obj);
    pyrt::xrefs::reg_fn("knowledge.materials", "_area", py_func("_area", |args: &[Py]| _area(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_depth_mm", py_func("_depth_mm", |args: &[Py]| _depth_mm(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_sizes", py_func("_sizes", |args: &[Py]| _sizes(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_a_bag_count", py_func("_a_bag_count", |args: &[Py]| _a_bag_count(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_count", py_func("_count", |args: &[Py]| _count(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_loose_reply", py_func("_loose_reply", |args: &[Py]| _loose_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_slabs_reply", py_func("_slabs_reply", |args: &[Py]| _slabs_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_turf_reply", py_func("_turf_reply", |args: &[Py]| _turf_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_bulk_bag_weight", py_func("_bulk_bag_weight", |args: &[Py]| _bulk_bag_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_not_a_persons_weight", py_func("_not_a_persons_weight", |args: &[Py]| _not_a_persons_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_not_a_sanding_job", py_func("_not_a_sanding_job", |args: &[Py]| _not_a_sanding_job(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_asked_area", py_func("_asked_area", |args: &[Py]| _asked_area(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_rolls", py_func("_rolls", |args: &[Py]| _rolls(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_underlay_reply", py_func("_underlay_reply", |args: &[Py]| _underlay_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_grout_reply", py_func("_grout_reply", |args: &[Py]| _grout_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_adhesive_reply", py_func("_adhesive_reply", |args: &[Py]| _adhesive_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_floor_reply", py_func("_floor_reply", |args: &[Py]| _floor_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_concrete_is_what_is_asked", py_func("_concrete_is_what_is_asked", |args: &[Py]| _concrete_is_what_is_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.materials", "_concrete_reply", py_func("_concrete_reply", |args: &[Py]| _concrete_reply(args)));
    pyrt::xrefs::reg_access("knowledge.materials", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.materials", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.materials", "_ASKS_MATERIAL", g__ASKS_MATERIAL, s__ASKS_MATERIAL);
    pyrt::xrefs::reg_access("knowledge.materials", "_ASKS_MATERIAL_AVOID", g__ASKS_MATERIAL_AVOID, s__ASKS_MATERIAL_AVOID);
    pyrt::xrefs::reg_access("knowledge.materials", "_A_FRACTION", g__A_FRACTION, s__A_FRACTION);
    pyrt::xrefs::reg_access("knowledge.materials", "_A_MATERIAL_WEIGHT", g__A_MATERIAL_WEIGHT, s__A_MATERIAL_WEIGHT);
    pyrt::xrefs::reg_access("knowledge.materials", "_A_NUMBER", g__A_NUMBER, s__A_NUMBER);
    pyrt::xrefs::reg_access("knowledge.materials", "_A_PERSONS_WEIGHT", g__A_PERSONS_WEIGHT, s__A_PERSONS_WEIGHT);
    pyrt::xrefs::reg_access("knowledge.materials", "_BAG_AVOID", g__BAG_AVOID, s__BAG_AVOID);
    pyrt::xrefs::reg_access("knowledge.materials", "_BAG_WORD", g__BAG_WORD, s__BAG_WORD);
    pyrt::xrefs::reg_access("knowledge.materials", "_BULK_WEIGHT", g__BULK_WEIGHT, s__BULK_WEIGHT);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_ASKS", g__CONCRETE_ASKS, s__CONCRETE_ASKS);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_AVOID", g__CONCRETE_AVOID, s__CONCRETE_AVOID);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_JOB", g__CONCRETE_JOB, s__CONCRETE_JOB);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_JOBS", g__CONCRETE_JOBS, s__CONCRETE_JOBS);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_NAMES", g__CONCRETE_NAMES, s__CONCRETE_NAMES);
    pyrt::xrefs::reg_access("knowledge.materials", "_CONCRETE_SURE", g__CONCRETE_SURE, s__CONCRETE_SURE);
    pyrt::xrefs::reg_access("knowledge.materials", "_DEPTH", g__DEPTH, s__DEPTH);
    pyrt::xrefs::reg_access("knowledge.materials", "_FEET", g__FEET, s__FEET);
    pyrt::xrefs::reg_access("knowledge.materials", "_FLOOR_AVOID", g__FLOOR_AVOID, s__FLOOR_AVOID);
    pyrt::xrefs::reg_access("knowledge.materials", "_FLOOR_NAMES", g__FLOOR_NAMES, s__FLOOR_NAMES);
    pyrt::xrefs::reg_access("knowledge.materials", "_KIND_ASKS", g__KIND_ASKS, s__KIND_ASKS);
    pyrt::xrefs::reg_access("knowledge.materials", "_LAST_TURNS", g__LAST_TURNS, s__LAST_TURNS);
    pyrt::xrefs::reg_access("knowledge.materials", "_LENGTH_NAMES", g__LENGTH_NAMES, s__LENGTH_NAMES);
    pyrt::xrefs::reg_access("knowledge.materials", "_LOOSE", g__LOOSE, s__LOOSE);
    pyrt::xrefs::reg_access("knowledge.materials", "_MATERIAL_AVOID", g__MATERIAL_AVOID, s__MATERIAL_AVOID);
    pyrt::xrefs::reg_access("knowledge.materials", "_NUM", g__NUM, s__NUM);
    pyrt::xrefs::reg_access("knowledge.materials", "_PAIR", g__PAIR, s__PAIR);
    pyrt::xrefs::reg_access("knowledge.materials", "_SLAB", g__SLAB, s__SLAB);
    pyrt::xrefs::reg_access("knowledge.materials", "_SQUARE", g__SQUARE, s__SQUARE);
    pyrt::xrefs::reg_access("knowledge.materials", "_UNIT", g__UNIT, s__UNIT);
}

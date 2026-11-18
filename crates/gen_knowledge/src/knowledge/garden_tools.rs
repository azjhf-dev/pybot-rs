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


thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHAIN_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUTS_PLANTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEFINITION_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAULT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FUEL_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GARDEN_MACHINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LINE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOWER_PUT_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_CUTTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_FEEDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__START_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STORE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__USE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WET_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__CARE_IT() -> Py { ensure_init(); G__CARE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARE_IT(v: Py) { ensure_init(); G__CARE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHAIN_OFF() -> Py { ensure_init(); G__CHAIN_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHAIN_OFF(v: Py) { ensure_init(); G__CHAIN_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__CUTS_PLANTS() -> Py { ensure_init(); G__CUTS_PLANTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUTS_PLANTS(v: Py) { ensure_init(); G__CUTS_PLANTS.with(|c| *c.borrow_mut() = v); }
pub fn g__DEFINITION_IT() -> Py { ensure_init(); G__DEFINITION_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_IT(v: Py) { ensure_init(); G__DEFINITION_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__FAULT_IT() -> Py { ensure_init(); G__FAULT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAULT_IT(v: Py) { ensure_init(); G__FAULT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__FUEL_IT() -> Py { ensure_init(); G__FUEL_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FUEL_IT(v: Py) { ensure_init(); G__FUEL_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__GARDEN_MACHINES() -> Py { ensure_init(); G__GARDEN_MACHINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GARDEN_MACHINES(v: Py) { ensure_init(); G__GARDEN_MACHINES.with(|c| *c.borrow_mut() = v); }
pub fn g__LINE_IT() -> Py { ensure_init(); G__LINE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LINE_IT(v: Py) { ensure_init(); G__LINE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__MOWER_PUT_AWAY() -> Py { ensure_init(); G__MOWER_PUT_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOWER_PUT_AWAY(v: Py) { ensure_init(); G__MOWER_PUT_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_CUTTING() -> Py { ensure_init(); G__NOT_CUTTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_CUTTING(v: Py) { ensure_init(); G__NOT_CUTTING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_FEEDING() -> Py { ensure_init(); G__NOT_FEEDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_FEEDING(v: Py) { ensure_init(); G__NOT_FEEDING.with(|c| *c.borrow_mut() = v); }
pub fn g__START_IT() -> Py { ensure_init(); G__START_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__START_IT(v: Py) { ensure_init(); G__START_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__STORE_IT() -> Py { ensure_init(); G__STORE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STORE_IT(v: Py) { ensure_init(); G__STORE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__USE_IT() -> Py { ensure_init(); G__USE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__USE_IT(v: Py) { ensure_init(); G__USE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__WET_IT() -> Py { ensure_init(); G__WET_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WET_IT(v: Py) { ensure_init(); G__WET_IT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The garden machines that are not a mower: a hedge trimmer, a strimmer, a leaf blower, a\nchainsaw.\n\n\"can i keep a hedge trimmer in a shed\" and \"how do i use a hedge trimmer\" were answered with the\nbeard-trimming card - \"trim it dry, on a guard a couple of steps longer than the length you\nwant... take the neckline just above the Adam's apple\" - because \"trimmer\" is in that row's key\nand a garden machine is not a face. \"how do i store my strimmer\" and \"how do i use a strimmer\"\nwere the mower's air filter, \"how do i sharpen my hedge trimmer\" was the razor blade, and\n\"what is a strimmer\", \"do i need a leaf blower\" and \"how do i change the line on a strimmer\"\nwere punts.\n\nThese machines share a mower's problems - a small engine, a cutting edge, a winter in a shed -\nbut not its answers, so they are their own domain with the machine named and the answer picked\nfrom what the line asks: what it is, how it cuts, how it is looked after, how it is put away,\nwhy it will not start, and which one to buy. The mower keeps its own rows in home.py, and the\nhedge itself - how often to cut it, when, and what to plant - stays with garden.py.\n");
    s_ENTRIES(py_list(vec![]));
    s__GARDEN_MACHINES(py_tuple(vec![py_str("hedge trimmer"), py_str("hedge trimmers"), py_str("hedge cutters"), py_str("strimmer"), py_str("strimmers"), py_str("grass trimmer"), py_str("grass trimmers"), py_str("leaf blower"), py_str("leafblower"), py_str("leaf blowers"), py_str("chainsaw"), py_str("chain saw"), py_str("chainsaws"), py_str("rotavator"), py_str("garden tools"), py_str("garden tool"), py_str("garden machinery"), py_str("cut a hedge"), py_str("cut the hedge"), py_str("cut my hedge"), py_str("trim a hedge"), py_str("trim the hedge"), py_str("trim my hedge"), py_str("cut hedges"), py_str("hedge straight"), py_str("edge my lawn"), py_str("edge the lawn"), py_str("edging a lawn"), py_str("lawn edges"), py_str("edges of my lawn")]));
    s__STORE_IT(py_tuple(vec![py_str("store"), py_str("storing"), py_str("stored"), py_str("storage"), py_str("keep it"), py_str("keep my"), py_str("keeping"), py_str("put away"), py_str("put it away"), py_str("over winter"), py_str("for the winter"), py_str("winter"), py_str("wintering"), py_str("winterise"), py_str("winterize"), py_str("shed"), py_str("garage"), py_str("damp"), py_str("rust"), py_str("rusty"), py_str("away until")]));
    s__CARE_IT(py_tuple(vec![py_str("sharpen"), py_str("sharpening"), py_str("sharp"), py_str("sharper"), py_str("blunt"), py_str("blades"), py_str("blade"), py_str("oil"), py_str("oiling"), py_str("clean"), py_str("cleaning"), py_str("care"), py_str("look after"), py_str("last longer"), py_str("maintenance"), py_str("servicing"), py_str("jammed"), py_str("stuck"), py_str("which oil")]));
    s__USE_IT(py_tuple(vec![py_str("how do i use"), py_str("how to use"), py_str("how do i cut"), py_str("how to cut"), py_str("how do i trim"), py_str("how to trim"), py_str("how do i edge"), py_str("straight"), py_str("technique"), py_str("right way"), py_str("hold it"), py_str("cut a hedge"), py_str("cut the hedge"), py_str("trim a hedge"), py_str("trim the hedge"), py_str("cut my hedge"), py_str("trim my hedge"), py_str("edge my lawn"), py_str("edges"), py_str("shape a hedge")]));
    s__FUEL_IT(py_tuple(vec![py_str("wont start"), py_str("won t start"), py_str("not starting"), py_str("not start"), py_str("stalls"), py_str("stalling"), py_str("cutting out"), py_str("smoking"), py_str("smoke"), py_str("flooded")]));
    s__START_IT(py_tuple(vec![py_str("how do i start"), py_str("how to start"), py_str("how do i get it started"), py_str("get it started"), py_str("start it from cold"), py_str("from cold"), py_str("cold start"), py_str("how do i start it"), py_str("starting from cold"), py_str("choke")]));
    s__NOT_CUTTING(py_tuple(vec![py_str("not cutting"), py_str("wont cut"), py_str("won t cut"), py_str("doesnt cut"), py_str("doesn t cut"), py_str("not cut"), py_str("tears the leaves"), py_str("tearing"), py_str("chews the"), py_str("bites the"), py_str("not cutting well"), py_str("stopped cutting"), py_str("not sharp"), py_str("judders"), py_str("juddering"), py_str("stops when i"), py_str("cuts out when i")]));
    s__WET_IT(py_tuple(vec![py_str("wet grass"), py_str("wet lawn"), py_str("wet cutting"), py_str("damp grass"), py_str("in the rain"), py_str("when its wet"), py_str("when it s wet"), py_str("after rain"), py_str("after the rain"), py_str("raining"), py_str("early morning dew"), py_str("dew")]));
    s__LINE_IT(py_tuple(vec![py_str("new line"), py_str("change the line"), py_str("change line"), py_str("replace the line"), py_str("the line"), py_str("spool"), py_str("keeps snapping"), py_str("keeps breaking"), py_str("line snapping"), py_str("line breaking"), py_str("line keeps"), py_str("line breaks"), py_str("line break"), py_str("line snapped"), py_str("line too short"), py_str("line not coming out"), py_str("wind the line")]));
    s__CHAIN_OFF(py_tuple(vec![py_str("chain keeps coming off"), py_str("chain coming off"), py_str("chain comes off"), py_str("chain fell off"), py_str("chain keeps jumping"), py_str("chain jumps off"), py_str("chain jumps"), py_str("chain falls off"), py_str("chain keeps coming loose"), py_str("chain loose"), py_str("chain slipping"), py_str("chain slips"), py_str("chain off the bar"), py_str("chain came off")]));
    s__NOT_FEEDING(py_tuple(vec![py_str("not feeding"), py_str("wont feed"), py_str("won t feed"), py_str("doesnt feed"), py_str("doesn t feed"), py_str("isn t feeding"), py_str("stopped feeding"), py_str("keeps stopping feeding"), py_str("line not feeding"), py_str("feeding the line"), py_str("feed the line"), py_str("line wont come out"), py_str("line stuck in the head")]));
    s__CUTS_PLANTS(py_tuple(vec![py_str("cutting the plants"), py_str("cuts the plants"), py_str("cut the plants"), py_str("cutting my plants"), py_str("cut my plants"), py_str("cutting the flowers"), py_str("cuts the flowers"), py_str("cut my flowers"), py_str("cutting the border"), py_str("cutting into the border"), py_str("cutting the hedge plants"), py_str("line cuts"), py_str("cutting the shrubs"), py_str("cut the shrubs")]));
    s__FAULT_IT(py_tuple(vec![py_str("keeps coming off"), py_str("coming off"), py_str("keeps falling"), py_str("wont"), py_str("won t"), py_str("not working"), py_str("stopped working"), py_str("broken"), py_str("doesnt work"), py_str("doesn t work"), py_str("smoking"), py_str("smokes"), py_str("leaks"), py_str("leaking"), py_str("cuts out"), py_str("keeps cutting out"), py_str("rattles"), py_str("rattling"), py_str("vibrates"), py_str("seized"), py_str("seizing"), py_str("jammed"), py_str("not spinning"), py_str("not turning"), py_str("not feeding"), py_str("overheating"), py_str("gets hot")]));
    s__DEFINITION_IT(py_tuple(vec![py_str("what is"), py_str("whats a"), py_str("what s a"), py_str("what's a"), py_str("what are"), py_str("what does"), py_str("mean"), py_str("means"), py_str("definition"), py_str("called"), py_str("used for"), py_str("difference between")]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("garden-machine")); __args.push(py_tuple(vec![g__GARDEN_MACHINES(), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("what does"), py_str("used for"), py_str("mean"), py_str("cut a hedge"), py_str("cut the hedge"), py_str("cut my hedge"), py_str("trim a hedge"), py_str("trim the hedge"), py_str("trim my hedge"), py_str("edge my lawn"), py_str("edge the lawn"), py_str("edging"), py_str("garden tools"), py_str("called"), py_str("store"), py_str("storing"), py_str("storage"), py_str("keep"), py_str("put away"), py_str("winter"), py_str("shed"), py_str("garage"), py_str("rust"), py_str("sharpen"), py_str("blade"), py_str("blunt"), py_str("oil"), py_str("clean"), py_str("care"), py_str("look after"), py_str("maintenance"), py_str("wont start"), py_str("won t start"), py_str("not starting"), py_str("stalls"), py_str("cut"), py_str("trim"), py_str("edge"), py_str("use"), py_str("spool"), py_str("line"), py_str("change the line"), py_str("snapping"), py_str("snapped"), py_str("snaps"), py_str("keeps breaking"), py_str("keeps snapping"), py_str("too short"), py_str("not cutting"), py_str("wont cut"), py_str("won t cut"), py_str("tears"), py_str("jammed"), py_str("wet grass"), py_str("wet lawn"), py_str("damp grass"), py_str("raining"), py_str("in the rain"), py_str("choke"), py_str("from cold"), py_str("to buy"), py_str("buy"), py_str("best"), py_str("straight"), py_str("line"), py_str("spool"), py_str("should i buy"), py_str("which"), py_str("worth it"), py_str("do i need")])])); __args.push(g__garden_machine()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("worth repairing"), py_str("repair or replace"), py_str("repair it or"), py_str("or replace"), py_str("scrap"), py_str("cheapest"), py_str("price"), py_str("cost"), py_str("review"), py_str("recommend"), py_str("how often"), py_str("how short"), py_str("best time"), py_str("when should i cut"), py_str("when to cut"), py_str("birds"), py_str("nesting"), py_str("moss"), py_str("weed"), py_str("weeds"), py_str("new lawn"), py_str("seed"), py_str("turf"), py_str("leaf mould"), py_str("compost"), py_str("what to plant"), py_str("which hedge"), py_str("fast growing"), py_str("privacy"), py_str("beard"), py_str("hair"), py_str("nose"), py_str("ear"), py_str("kitchen"), py_str("knife"), py_str("bread"), py_str("cheese"), py_str("tile"), py_str("metal"), py_str("pipe"), py_str("tree surgeon")])), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    s__MOWER_PUT_AWAY(py_str("Run the tank dry, or fill it right up with a fuel stabiliser in it - a half-full tank of last autumn's petrol is what makes a spring start-up a repair. Then change the oil while the engine is still warm so the dirt comes out with it, scrape the grass from under the deck and oil the blade. Keep it in a dry shed or a garage on a shelf or a pallet rather than on a concrete floor, which holds damp and rusts the deck from underneath, and take the battery off and charge it once a month if it has one."));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mower-put-away")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lawnmower"), py_str("lawn mower"), py_str("lawnmowers"), py_str("mower"), py_str("mowers"), py_str("hover mower"), py_str("ride on mower")]), py_tuple(vec![py_str("store"), py_str("storing"), py_str("stored"), py_str("storage"), py_str("put away"), py_str("put it away"), py_str("over winter"), py_str("for the winter"), py_str("winter"), py_str("winterise"), py_str("winterize"), py_str("wintering"), py_str("keep it"), py_str("keep my"), py_str("where do i keep"), py_str("shed"), py_str("garage"), py_str("away until"), py_str("before winter")])])); __args.push(g__MOWER_PUT_AWAY()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("filter"), py_str("air filter"), py_str("oil"), py_str("oil change"), py_str("spark plug"), py_str("blade"), py_str("sharpen"), py_str("sharp"), py_str("blunt"), py_str("wont start"), py_str("won t start"), py_str("not starting"), py_str("starts then"), py_str("service"), py_str("servicing"), py_str("clean"), py_str("cleaning"), py_str("broken"), py_str("repair"), py_str("replace"), py_str("buy"), py_str("cheapest"), py_str("best"), py_str("which"), py_str("worth")])), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
}
pub fn _the_guard_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("True for the safety question: what to wear, and how not to cut yourself.");
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("safety"), py_str("protective"), py_str("goggles"), py_str("glasses"), py_str("visor"), py_str("chaps"), py_str("helmet"), py_str("defenders"), py_str("gloves"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("ppE"), "lower", args))(&__args) }]))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("safety gear"), py_str("safety equipment"), py_str("protective gear"), py_str("protective equipment"), py_str("what should i wear"), py_str("what do i wear"), py_str("cut my fingers"), py_str("cutting my fingers"), py_str("cut my leg"), py_str("cutting my leg"), py_str("keep catching"), py_str("slice my"), py_str("kickback"), py_str("kick back")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _garden_machine(args: &[Py]) -> Py {
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
    let mut blower: Py = Py::None;
    let mut first: Py = Py::None;
    let mut hedge: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut putting_away: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_them: Py = Py::None;
    let mut saw: Py = Py::None;
    let mut says: Py = Py::None;
    let mut strimmer: Py = Py::None;
    let mut tiller: Py = Py::None;
    let mut warning: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A hedge trimmer, a strimmer, a blower or a chainsaw: what it is, how it cuts, how it keeps.");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    line = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
pub fn __nested_says_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut blower: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut first: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut hedge: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut named: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut putting_away: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut reading: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut rest: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut said_them: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut saw: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut strimmer: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut tiller: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut warning: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut phrases: Py = __pos.get(19).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "phrases").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut phrase: Py = Py::None;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
    says = {{ let blower = blower.clone(); let first = first.clone(); let hedge = hedge.clone(); let line = line.clone(); let memory = memory.clone(); let named = named.clone(); let putting_away = putting_away.clone(); let reading = reading.clone(); let rest = rest.clone(); let said = said.clone(); let said_them = said_them.clone(); let saw = saw.clone(); let says = says.clone(); let strimmer = strimmer.clone(); let tiller = tiller.clone(); let understood = understood.clone(); let warning = warning.clone(); let word = word.clone(); let words = words.clone(); py_func("says", move |args: &[Py]| { let mut full = vec![blower.clone(), first.clone(), hedge.clone(), line.clone(), memory.clone(), named.clone(), putting_away.clone(), reading.clone(), rest.clone(), said.clone(), said_them.clone(), saw.clone(), says.clone(), strimmer.clone(), tiller.clone(), understood.clone(), warning.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_says_0(&full) }) }};
    hedge = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("hedge"), py_str("hedges"), py_str("hedge trimmer"), py_str("hedge trimmers"), py_str("hedge cutters"), py_str("trimming"), py_str("topiary")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("a hedge"), py_str("the hedge"), py_str("my hedge"), py_str("hedges")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) } } };
    strimmer = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("strimmer"), py_str("strimmers"), py_str("grass trimmer"), py_str("grass trimmers")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("strimmer"), py_str("grass trimmer"), py_str("edge my lawn"), py_str("edge the lawn"), py_str("edging"), py_str("lawn edge"), py_str("lawn edges"), py_str("edges of my lawn")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) } } };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &py_set(vec![py_str("edge"), py_str("edges"), py_str("edging")])); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("lawn"), py_str("lawns"), py_str("grass"), py_str("path"), py_str("paths"), py_str("drive"), py_str("driveway"), py_str("border"), py_str("borders"), py_str("kerb"), py_str("paving")])) } }) {
    strimmer = Py::Bool(true);
    }
    blower = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("leaf blower"), py_str("leafblower"), py_str("leaf blowers")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("leaf blower"), py_str("leafblower"), py_str("blow the leaves"), py_str("blow leaves"), py_str("leaves off")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) } } };
    tiller = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("rotavator"), py_str("rotovator"), py_str("rotavating"), py_str("rotovating"), py_str("tiller"), py_str("cultivator")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("rotavator"), py_str("rotovator"), py_str("dig over"), py_str("break up the soil")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) } } };
    saw = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("chainsaw"), py_str("chain saw"), py_str("chainsaws")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("chainsaw"), py_str("chain saw"), py_str("fell a tree"), py_str("cut a log"), py_str("cut logs")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) } } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__LINE_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = strimmer.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = hedge.clone(); if py_truthy(&__b) { __b } else { { let __b = blower.clone(); if py_truthy(&__b) { __b } else { saw.clone() } } } })) } } } }) {
    return py_str("Take the spool off the head, wind the new line on in the direction of the arrow on it, and hook each end through the eyelet on opposite sides so it cuts level. Leave about 10 to 15cm of line out on each side and let the head spin for a moment before you cut, because a line wound the wrong way will not feed. If it keeps snapping, it is usually too long, or the line is old and brittle.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__FUEL_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    if py_truthy(&{ let __b = strimmer.clone(); if py_truthy(&__b) { __b } else { saw.clone() } }) {
    return py_str("Two-stroke machines are nine times in ten old fuel: the mix goes off in about a month, so tip it out, put fresh petrol with the right two-stroke oil in it (50:1 on most) and try again with the choke on for two pulls only, or the engine floods. Then the spark plug out and dried, the air filter washed, and on a strimmer the line trimmed to the right length so it is not grabbing the guard.");
    }
    return py_str("Nine times in ten it is the fuel: petrol goes off in about a month, so drain last year's half tank and refill. Then the spark plug out and cleaned or replaced, the air filter shaken out, and the blades checked for a jammed stem or a build-up of cuttings behind the guard, which drags the motor down and makes it stall.");
    }
    putting_away = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("store"), py_str("storing"), py_str("stored"), py_str("storage"), py_str("put away"), py_str("put it away"), py_str("over winter"), py_str("for the winter"), py_str("winter"), py_str("wintering"), py_str("winterise"), py_str("winterize"), py_str("shed"), py_str("garage"), py_str("rust"), py_str("rusty"), py_str("away until")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__STORE_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = putting_away.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__CARE_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) })) } } } }) {
    if py_truthy(&blower.clone()) {
    return py_str("Empty the bag and wipe the inside of the tube, because leaves left in it go soggy and rot the housing. Run the tank dry if it is petrol, or charge the battery to about half if it is not, and hang it in a dry shed rather than standing it on a damp concrete floor.");
    }
    if py_truthy(&saw.clone()) {
    return py_str("Empty the fuel tank and run it dry, because petrol left in a carburettor over winter turns to varnish and that is what turns a spring start into a repair. Clean the bar groove, oil the chain, slacken the chain tension so it is not stretched over months, and store it with the bar in its cover in a dry shed.");
    }
    if py_truthy(&{ let __b = strimmer.clone(); if py_truthy(&__b) { __b } else { hedge.clone() } }) {
    return py_str("Empty the fuel tank and run the engine until it stops, because petrol left in a carburettor over winter turns to varnish and that is what makes a spring start-up a repair. Wipe the blades or the cutting head clean, oil the blades with a rag and a few drops of three-in-one, and put the guard back on. Hang it in a dry shed rather than standing it on a concrete floor, and charge a battery tool to about half and keep it indoors, since a battery left flat in a cold shed does not come back.");
    }
    return py_str("Store garden machines the same way: fuel out and the engine run dry, blades wiped and oiled, guards on, and everything up off a damp concrete floor in a dry shed. Batteries come indoors at about half charge, because the cold is what kills them.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__USE_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    if py_truthy(&tiller.clone()) {
    return py_str("Go over the ground twice, the first pass shallow and slow with the tines just breaking the surface and the second deeper, and only when the soil is damp but not wet enough to smear. Take it in strips the width of the machine, keep the tines clear of stones and roots, and let the soil settle a week or two before planting or sowing.");
    }
    if py_truthy(&hedge.clone()) {
    return py_str("Cut the sides first, bottom to top in a sweep with the blades at about 10 to 15 degrees to the hedge, then the top flat - working upwards throws the cuttings clear instead of back in your face. A string line along the top and one down each end is what makes a hedge look straight, and take the top back to the same width as the bottom, so light reaches the base.");
    }
    if py_truthy(&strimmer.clone()) {
    return py_str("Hold the head just off the ground and sweep it side to side from left to right with the line hitting the grass at full speed, and work away from anything you do not want cut. Edge along a path by walking it with the head turned 90 degrees, and tap the line out on the ground rather than pulling it by hand.");
    }
    if py_truthy(&blower.clone()) {
    return py_str("Work with the wind and from the edges inwards, holding the tube just off the ground and using short sweeps rather than full power - and never blow cuttings into a drain, a road or a neighbour's garden. Damp leaves want a rake first, because a blower only moves them.");
    }
    if py_truthy(&saw.clone()) {
    return py_str("Start the cut with the saw running at full speed and let the chain do the work, keeping the bar out of the dirt and your body to one side of the cut. Cut from the top down on a branch, keep the chain oiled and tensioned, and never cut above shoulder height or with the tip, which is what causes kickback.");
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__WET_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &py_set(vec![py_str("grass"), py_str("lawn"), py_str("lawns"), py_str("cut"), py_str("cutting"), py_str("strim"), py_str("strimming"), py_str("edge"), py_str("edging"), py_str("mow"), py_str("mowing")])); if py_truthy(&__b) { __b } else { strimmer.clone() } } } }) {
    return py_str("Wet grass clogs the guard and welts the line, so the head drags and the motor bogs down; leave it until it is dry and it cuts in half the time. If you have to, raise the head a notch, take a narrower strip, use the guard and clean the grass out of the spool housing before it sets. A petrol machine also cuts long wet grass badly, so a couple of passes at different heights beat one deep one.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__NOT_CUTTING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    if py_truthy(&hedge.clone()) {
    return py_str("A trimmer bar that tears rather than cuts is blunt, or the blades are loose or jammed with sap: clean the bar with white spirit, check the bolts are nipped up, and file each blade at the angle it already has. A hedge also tears if you are cutting woody growth more than a finger thick, which wants loppers or a saw first.");
    }
    if py_truthy(&strimmer.clone()) {
    return py_str("A line that will not cut is usually too long, too old, or wound the wrong way: trim it to about 12cm out of the head, wind it on in the direction of the arrow, and if it is brittle, replace it. Check too that the line is not welded into a lump in the spool, which happens when the head is held in long grass.");
    }
    if py_truthy(&saw.clone()) {
    return py_str("A chain that will not cut is blunt or too loose: file every tooth on the angle marked on it, take the depth gauges down a touch behind the teeth, and check the tension cold so it lifts about 3mm off the bar. A dry chain also stretches and burns the bar, so keep the oil tank full.");
    }
    return py_str("Clean the cutting head, check the blade or the line, and look for a jam: a build-up of cuttings behind the guard drags the motor down, a blunt blade or an old line tears instead of cutting, and a loose blade judders. Nothing sharp on a garden machine is worth using blunt, because a dull edge is what makes people push and slip.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_guard_question(args))(&__args) }) {
    if py_truthy(&saw.clone()) {
    return py_str("Chaps or tough trousers over the legs, a helmet with a visor and ear defenders, gloves and boots - and a first-aid kit within reach, because the cut that goes wrong is the one on your own leg. Kickback comes from the tip of the bar, so keep the tip out of the wood and stand to one side of the cut.");
    }
    if py_truthy(&hedge.clone()) {
    return py_str("Gloves, eye protection and ear defenders, and the guard back on the blades before you start. Keep the lead behind you and over your shoulder rather than trailing in front, hold the trimmer with both hands and never reach over the hedge to the far side, which is how fingers get between the blades.");
    }
    if py_truthy(&strimmer.clone()) {
    return py_str("Eye protection, gloves and trousers rather than shorts, because the line throws stones as well as grass. The guard stays on, the head stays below waist height, and nobody else - child or pet - is in the same part of the garden while it spins.");
    }
    return py_str("Eye protection, gloves, ear defenders and boots, with the guard on and the lead behind you rather than in front, and nobody else in the garden while it is running.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__START_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    if py_truthy(&{ let __b = saw.clone(); if py_truthy(&__b) { __b } else { strimmer.clone() } }) {
    return py_str("Cold start: choke fully out, two or three pulls only - any more and it floods - then the choke half in and it should catch. If it fires and dies, the mix is stale or the idle is set too low; if it does not fire at all, the plug is wet or the air filter is clogged. A hedgetrimmer with a battery or a lead has none of that: hold the trigger lock, squeeze, and if it is dead check the lead and the breaker.");
    }
    return py_str("Cold start: choke out, a couple of pulls, then choke half in. If it fires and dies the fuel is old, and if it does not fire the plug or the air filter is the next thing to look at.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__CARE_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    if py_truthy(&hedge.clone()) {
    return py_str("Take the blades off with the plug lead pulled or the battery out, clamp them in a vice and run a flat file along each blade at the angle it already has, about a dozen strokes, then a drop of oil along the bar and a run for a minute to spread it. Clean the sap off with a rag and white spirit first, because sap is what makes a trimmer bind.");
    }
    if py_truthy(&saw.clone()) {
    return py_str("File every tooth on the same angle with a round file of the size the chain says, two or three strokes each, and take the depth gauges down a touch behind them so the teeth can bite. Check the tension cold - it should lift about 3mm off the bar - and keep the oil tank full, because a dry chain stretches and then breaks.");
    }
    return py_str("Clean the cutting head and the air intake of grass and sap, oil the moving parts with a few drops, and check the blade or the line: a strimmer line that is too long drags the motor down and a blunt blade tears rather than cuts. A chain wants its tension checked every use and a file across the teeth every few tanks.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("should i buy"), py_str("which one"), py_str("worth it"), py_str("to buy"), py_str("buy"), py_str("buying"), py_str("worth buying"), py_str("best one"), py_str("best to get"), py_str("should i get"), py_str("what should i get"), py_str("do i need"), py_str("need a"), py_str("or a"), py_str("instead of"), py_str("which is better"), py_str("recommend"), py_str("recommendation"), py_str("petrol or"), py_str("battery or"), py_str("corded or"), py_str("mains or")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("which")])) } }) {
    if py_truthy(&hedge.clone()) {
    return py_str("Get a corded trimmer for one hedge under about 10 metres and a battery one if the garden has no outdoor socket or the hedge is longer, because a cable overhead is the thing that makes people put it off. Petrol only for a hedge over about 2 metres or a long run of it.");
    }
    if py_truthy(&strimmer.clone()) {
    return py_str("A mains strimmer is the cheapest and lightest for a small lawn, battery for anything over about 50 metres of edge or where there is no outdoor socket, and petrol only for rough ground and long sessions. A bump-feed head is worth more than extra power for ordinary garden edging.");
    }
    if py_truthy(&blower.clone()) {
    return py_str("You need one if leaves sit on gravel, decking or a lawn you want tidy, and you do not if the garden is small enough to rake in ten minutes. A battery blower is quieter and enough for a suburban garden; petrol only for a big garden and a lot of leaves at once.");
    }
    if py_truthy(&saw.clone()) {
    return py_str("A battery saw is enough for logs and a fallen branch up to about 20cm and is the one most people use; petrol only for a lot of timber and a whole afternoon of cutting. Get one with a chain brake whatever else you choose.");
    }
    return py_str("Match the tool to the job and the size: mains for a small garden and a short run, battery where there is no outdoor socket or the run is long, and petrol only for a big garden where the noise and the fuel are worth it.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__CHAIN_OFF()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = saw.clone(); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("chain"), py_str("chainsaw"), py_str("chain saw")])) } } } }) {
    return py_str("A chain that keeps coming off is loose tension or a worn drive sprocket: check the two bar nuts are nipped up, set the tension cold so the chain lifts only about 3mm off the bar, then turn the bar over and look at the nose sprocket, because a worn one lets the chain climb out of the groove. If it still jumps the chain is stretched or the sprocket is hooked, and either will wear the bar out if you keep cutting with it.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__NOT_FEEDING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }) {
    return py_str("A head that will not feed is usually wound the wrong way or the line has welded into one lump: take the spool off, pull all the line out, rewind it in the direction of the arrow moulded on the spool and leave about 10 to 15cm out on each side. Old line goes brittle and fuses when it is held in long grass, so cut off the fused part and rewind the rest, or fit new line of the thickness the head says.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__CUTS_PLANTS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = strimmer.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("cutting the"), py_str("cuts the")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("plants"), py_str("flowers"), py_str("shrubs"), py_str("border"), py_str("borders"), py_str("beds")])) } } } } } }) {
    return py_str("Keep the line off the beds by cutting the edge in two passes: the first with the head tilted so the line works the grass and not the soil, sweeping away from the border rather than along it. A line about 10cm long throws less than a long one, and a plant guard or a board stood along the edge catches what still flicks over; string a line between two canes where you cannot see the boundary.");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__FAULT_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__DEFINITION_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&says, args))(&__args) })) } }) {
    return py_str("Treat it as three things in order: is it getting fuel and spark, is the cutting edge free, is anything loose. Tip out old petrol before anything else, dry or change the plug, wash the air filter, then take the guard off and clear the cuttings and grass packed behind it, which is what makes a head drag and a motor bog down. Check every nut, bolt and the bar or blade mounting for slack, because a machine that rattles is one that is about to throw something.");
    }
    named = py_list(vec![]);
    if py_truthy(&strimmer.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("A strimmer cuts grass and edges of a lawn with a nylon line on a spinning head")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    if py_truthy(&hedge.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a hedge trimmer is a bar of blades for the flat sides and top of a hedge")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    if py_truthy(&blower.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a leaf blower is a fan in a tube that shifts leaves off gravel, decking and grass")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    if py_truthy(&saw.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a chainsaw is a loop of chain on a bar, for timber only")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    if py_truthy(&tiller.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a rotavator is a small engine turning a row of curved tines to break up and mix soil before planting")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    if py_truthy(&named.clone()) {
    first = geti(&named.clone(), &Py::Int(0));
    rest = slice_or_raise(&named.clone(), &Py::Int(1), &Py::None, &Py::None);
    said_them = if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) { py_str("") } else { op_add(&op_add(&py_str(" The others in the shed: "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("; "), "join", args))(&__args) }), &py_str(".")) };
    warning = if py_truthy(&saw.clone()) { py_str(" It is not a hedge tool - it chews small growth instead of cutting it clean, and it kicks.") } else { py_str("") };
    return op_add(&op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&first.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&first.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(".")), &said_them.clone()), &warning.clone());
    }
    return py_str("A strimmer cuts grass and edges of a lawn with a nylon line on a spinning head, a hedge trimmer is a bar of blades for the flat sides and top of a hedge, a leaf blower is a fan in a tube, and a chainsaw is a loop of chain on a bar for timber. The strimmer is for grass, the trimmer for a hedge's shape, and the chainsaw only for wood - not for a hedge, where it chews the growth instead of cutting it clean.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.garden_tools".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ENTRIES" => Some(g_ENTRIES()),
            "_CARE_IT" => Some(g__CARE_IT()),
            "_CHAIN_OFF" => Some(g__CHAIN_OFF()),
            "_CUTS_PLANTS" => Some(g__CUTS_PLANTS()),
            "_DEFINITION_IT" => Some(g__DEFINITION_IT()),
            "_FAULT_IT" => Some(g__FAULT_IT()),
            "_FUEL_IT" => Some(g__FUEL_IT()),
            "_GARDEN_MACHINES" => Some(g__GARDEN_MACHINES()),
            "_LINE_IT" => Some(g__LINE_IT()),
            "_MOWER_PUT_AWAY" => Some(g__MOWER_PUT_AWAY()),
            "_NOT_CUTTING" => Some(g__NOT_CUTTING()),
            "_NOT_FEEDING" => Some(g__NOT_FEEDING()),
            "_START_IT" => Some(g__START_IT()),
            "_STORE_IT" => Some(g__STORE_IT()),
            "_USE_IT" => Some(g__USE_IT()),
            "_WET_IT" => Some(g__WET_IT()),
            "_garden_machine" => Some(py_func("_garden_machine", |args: &[Py]| _garden_machine(args))),
            "_the_guard_question" => Some(py_func("_the_guard_question", |args: &[Py]| _the_guard_question(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ENTRIES" => s_ENTRIES(v),
            "_CARE_IT" => s__CARE_IT(v),
            "_CHAIN_OFF" => s__CHAIN_OFF(v),
            "_CUTS_PLANTS" => s__CUTS_PLANTS(v),
            "_DEFINITION_IT" => s__DEFINITION_IT(v),
            "_FAULT_IT" => s__FAULT_IT(v),
            "_FUEL_IT" => s__FUEL_IT(v),
            "_GARDEN_MACHINES" => s__GARDEN_MACHINES(v),
            "_LINE_IT" => s__LINE_IT(v),
            "_MOWER_PUT_AWAY" => s__MOWER_PUT_AWAY(v),
            "_NOT_CUTTING" => s__NOT_CUTTING(v),
            "_NOT_FEEDING" => s__NOT_FEEDING(v),
            "_START_IT" => s__START_IT(v),
            "_STORE_IT" => s__STORE_IT(v),
            "_USE_IT" => s__USE_IT(v),
            "_WET_IT" => s__WET_IT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.garden_tools", module_obj);
    pyrt::xrefs::reg_fn("knowledge.garden_tools", "_the_guard_question", py_func("_the_guard_question", |args: &[Py]| _the_guard_question(args)));
    pyrt::xrefs::reg_fn("knowledge.garden_tools", "_garden_machine", py_func("_garden_machine", |args: &[Py]| _garden_machine(args)));
    pyrt::xrefs::reg_access("knowledge.garden_tools", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_CARE_IT", g__CARE_IT, s__CARE_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_CHAIN_OFF", g__CHAIN_OFF, s__CHAIN_OFF);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_CUTS_PLANTS", g__CUTS_PLANTS, s__CUTS_PLANTS);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_DEFINITION_IT", g__DEFINITION_IT, s__DEFINITION_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_FAULT_IT", g__FAULT_IT, s__FAULT_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_FUEL_IT", g__FUEL_IT, s__FUEL_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_GARDEN_MACHINES", g__GARDEN_MACHINES, s__GARDEN_MACHINES);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_LINE_IT", g__LINE_IT, s__LINE_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_MOWER_PUT_AWAY", g__MOWER_PUT_AWAY, s__MOWER_PUT_AWAY);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_NOT_CUTTING", g__NOT_CUTTING, s__NOT_CUTTING);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_NOT_FEEDING", g__NOT_FEEDING, s__NOT_FEEDING);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_START_IT", g__START_IT, s__START_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_STORE_IT", g__STORE_IT, s__STORE_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_USE_IT", g__USE_IT, s__USE_IT);
    pyrt::xrefs::reg_access("knowledge.garden_tools", "_WET_IT", g__WET_IT, s__WET_IT);
}

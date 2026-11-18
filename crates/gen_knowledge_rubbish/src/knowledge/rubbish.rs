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
thread_local! { static G_A_HOURS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_DAY_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_DAY_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONSUMABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURING_ASIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITEM_ROUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEDICINE_DISPOSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_RUBBISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECYCLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECYCLE_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIP_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHICH_BIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHICH_BIN_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTAINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_cook_ask: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.rubbish: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOURS_ASK() -> Py { ensure_init(); G_A_HOURS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOURS_ASK(v: Py) { ensure_init(); G_A_HOURS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MACHINE() -> Py { ensure_init(); G_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE(v: Py) { ensure_init(); G_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_ASK() -> Py { ensure_init(); G_BIN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_ASK(v: Py) { ensure_init(); G_BIN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_DAY() -> Py { ensure_init(); G_BIN_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_DAY(v: Py) { ensure_init(); G_BIN_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_DAY_AVOID() -> Py { ensure_init(); G_BIN_DAY_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_DAY_AVOID(v: Py) { ensure_init(); G_BIN_DAY_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_DAY_KEY() -> Py { ensure_init(); G_BIN_DAY_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_DAY_KEY(v: Py) { ensure_init(); G_BIN_DAY_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_WORDS() -> Py { ensure_init(); G_BIN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_WORDS(v: Py) { ensure_init(); G_BIN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONSUMABLE() -> Py { ensure_init(); G_CONSUMABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONSUMABLE(v: Py) { ensure_init(); G_CONSUMABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CURING_ASIDE() -> Py { ensure_init(); G_CURING_ASIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURING_ASIDE(v: Py) { ensure_init(); G_CURING_ASIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ITEMS() -> Py { ensure_init(); G_ITEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITEMS(v: Py) { ensure_init(); G_ITEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_ITEM_ROUTES() -> Py { ensure_init(); G_ITEM_ROUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITEM_ROUTES(v: Py) { ensure_init(); G_ITEM_ROUTES.with(|c| *c.borrow_mut() = v); }
pub fn g_MEDICINE_DISPOSAL() -> Py { ensure_init(); G_MEDICINE_DISPOSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEDICINE_DISPOSAL(v: Py) { ensure_init(); G_MEDICINE_DISPOSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_RUBBISH() -> Py { ensure_init(); G_NOT_RUBBISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_RUBBISH(v: Py) { ensure_init(); G_NOT_RUBBISH.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_MACHINE() -> Py { ensure_init(); G_ONLY_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_MACHINE(v: Py) { ensure_init(); G_ONLY_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACEMENT() -> Py { ensure_init(); G_PLACEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACEMENT(v: Py) { ensure_init(); G_PLACEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_INTO() -> Py { ensure_init(); G_PUT_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_INTO(v: Py) { ensure_init(); G_PUT_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_RECYCLE() -> Py { ensure_init(); G_RECYCLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECYCLE(v: Py) { ensure_init(); G_RECYCLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECYCLE_AVOID() -> Py { ensure_init(); G_RECYCLE_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECYCLE_AVOID(v: Py) { ensure_init(); G_RECYCLE_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_RID() -> Py { ensure_init(); G_RID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RID(v: Py) { ensure_init(); G_RID.with(|c| *c.borrow_mut() = v); }
pub fn g_TIP() -> Py { ensure_init(); G_TIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIP(v: Py) { ensure_init(); G_TIP.with(|c| *c.borrow_mut() = v); }
pub fn g_TIP_AVOID() -> Py { ensure_init(); G_TIP_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIP_AVOID(v: Py) { ensure_init(); G_TIP_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_WHICH_BIN() -> Py { ensure_init(); G_WHICH_BIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHICH_BIN(v: Py) { ensure_init(); G_WHICH_BIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHICH_BIN_AVOID() -> Py { ensure_init(); G_WHICH_BIN_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHICH_BIN_AVOID(v: Py) { ensure_init(); G_WHICH_BIN_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTAINERS() -> Py { ensure_init(); G__CONTAINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTAINERS(v: Py) { ensure_init(); G__CONTAINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_cook_ask() -> Py { ensure_init(); G_cook_ask.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_cook_ask(v: Py) { ensure_init(); G_cook_ask.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Getting rid of things: the bin, the tip, the shop, and the council collection.\n\nThe answers lane owns this domain. Every house holds things that cannot simply go out with the\nrubbish - a battery, a bulb, half a tin of medicine, a mattress, a fridge - and the question is\nalways the same shape: this thing, where does it go? Nobody else in the tree answers it: the one\nentry that did, `home.paint-tins`, exists because liquid paint is legally not household waste, and\nit is the pattern these entries follow.\n\nSo the fact here is the *route*, one per item, keyed by the item and one of the words that asks\nwhat to do with it (\"get rid of\", \"throw away\", \"in the bin\", \"recycle\", \"the tip\"). The route\nwords are shared by every entry on purpose: they say what is being asked, and the item says what\nabout, so one row answers \"can i put batteries in the bin\", \"how do i get rid of old batteries\"\nand \"where do i recycle batteries\" without three keys. Each entry carries an `avoid` list for the\nneighbouring question about the same item - a mattress to *buy* or *clean*, a fridge that is not\n*cold*, a phone *contract* - because those are other entries' lines and this one is only for the\nthing on its way out.\n\nWhat to do with an item nobody here knows is its own entry: rather than a shrug, it says the\nthree routes a thing usually takes and asks which item it is.\n");
    s_cook_ask(py_func("cook_ask", |args: &[Py]| pyrt::xrefs::xcall("knowledge.dishes", "cook_ask", args)));
    s_A_HOURS_ASK(pyrt::xrefs::xget("knowledge.lookup", "A_HOURS_ASK"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("technology")]));
    fn __data1() -> Py { py_tuple(vec![py_str("get rid"), py_str("rid of"), py_str("dispose"), py_str("disposal"), py_str("throw away"), py_str("throw out"), py_str("throwing away"), py_str("donate"), py_str("donated"), py_str("donating"), py_str("donation"), py_str("donations"), py_str("give away"), py_str("giving away"), py_str("gave away"), py_str("charity"), py_str("charity shop"), py_str("charity shops"), py_str("free ad"), py_str("free ads"), py_str("freecycle"), py_str("pass on"), py_str("where do i take"), py_str("where can i take"), py_str("where do i put"), py_str("where can i put"), py_str("drop off"), py_str("get shot of"), py_str("throw"), py_str("throwing"), py_str("toss"), py_str("chuck"), py_str("chuck out"), py_str("chucking out"), py_str("bin"), py_str("bins"), py_str("the tip"), py_str("to the tip"), py_str("tip it"), py_str("recycling centre"), py_str("recycle"), py_str("recycling"), py_str("kerbside"), py_str("waste"), py_str("dump"), py_str("scrap"), py_str("skip"), py_str("get shot"), py_str("household waste"), py_str("council collection"), py_str("collection"), py_str("do with")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.RID"); }
    s_RID(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.BIN_WORDS"); }
    s_BIN_WORDS(py_tuple(vec![py_str("bin"), py_str("bins"), py_str("the tip"), py_str("to the tip"), py_str("recycling"), py_str("recycle"), py_str("kerbside"), py_str("household waste")]));
    fn __data2() -> Py { py_tuple(vec![py_str("put"), py_str("throw"), py_str("chuck"), py_str("dump"), py_str("dispose"), py_str("get rid"), py_str("go in"), py_str("goes in"), py_str("go"), py_str("goes"), py_str("belong"), py_str("belongs"), py_str("which"), py_str("where")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.BIN_ASK"); }
    s_BIN_ASK(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.CURING_ASIDE"); }
    s_CURING_ASIDE(py_tuple(vec![py_str("without pills"), py_str("without tablets"), py_str("without medicine"), py_str("without medicines"), py_str("without medication"), py_str("without antibiotics"), py_str("without painkillers"), py_str("without any pills"), py_str("without any tablets"), py_str("without any medicine"), py_str("without drugs"), py_str("without meds")]));
    fn __data3() -> Py { py_tuple(vec![py_str("old"), py_str("unused"), py_str("unwanted"), py_str("leftover"), py_str("left over"), py_str("expired"), py_str("out of date"), py_str("past it"), py_str("no longer"), py_str("stopped taking"), py_str("dont need"), py_str("don't need"), py_str("not taking"), py_str("off them"), py_str("bin"), py_str("bins"), py_str("tip"), py_str("recycling"), py_str("recycle"), py_str("dispose"), py_str("throw"), py_str("throwing"), py_str("chuck"), py_str("pharmacy"), py_str("chemist"), py_str("where do i"), py_str("where can i"), py_str("where should i"), py_str("what do i do with"), py_str("get rid of the tablets"), py_str("get rid of the pills"), py_str("get rid of the medicine"), py_str("get rid of the medicines"), py_str("get rid of my tablets"), py_str("get rid of my pills"), py_str("get rid of my medicine"), py_str("get rid of my medicines"), py_str("get rid of these tablets"), py_str("get rid of these pills"), py_str("get rid of all the tablets"), py_str("get rid of all these pills"), py_str("get rid of pills"), py_str("get rid of tablets"), py_str("get rid of medicine"), py_str("get rid of medicines"), py_str("rid of old pills"), py_str("rid of leftover tablets")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.MEDICINE_DISPOSAL"); }
    s_MEDICINE_DISPOSAL(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.NOT_RUBBISH"); }
    s_NOT_RUBBISH(py_tuple(vec![py_str("waste of money"), py_str("waste of time"), py_str("wasted money"), py_str("wasting money"), py_str("worth the money"), py_str("worth it"), py_str("any point in"), py_str("money well spent"), py_str("a good buy"), py_str("a bad buy")]));
    fn __data5() -> Py { py_tuple(vec![py_str("pizza box"), py_str("pizza boxes"), py_str("greasy cardboard"), py_str("oily cardboard"), py_str("dirty cardboard"), py_str("takeaway box"), py_str("takeaway boxes"), py_str("chip box"), py_str("food box"), py_str("food boxes"), py_str("cardboard box"), py_str("cardboard boxes"), py_str("greasy box")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("leftover pizza"), py_str("cold pizza"), py_str("reheat"), py_str("reheating"), py_str("eat"), py_str("freeze"), py_str("freezing"), py_str("how long"), py_str("order"), py_str("delivery"), py_str("delivered"), py_str("arrived"), py_str("takeaway pizza"), py_str("slice")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("cardboard"), py_str("cardboard box"), py_str("cardboard boxes"), py_str("moving boxes"), py_str("moving box"), py_str("packing boxes"), py_str("delivery boxes"), py_str("delivery box"), py_str("packaging"), py_str("boxes"), py_str("box"), py_str("cartons"), py_str("carton"), py_str("empty boxes"), py_str("pile of boxes")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("pizza"), py_str("greasy"), py_str("oily"), py_str("dirty"), py_str("takeaway"), py_str("chip box"), py_str("food box"), py_str("leftover"), py_str("gift"), py_str("present"), py_str("cereal"), py_str("hamper"), py_str("batteries"), py_str("cat"), py_str("litter"), py_str("pack"), py_str("packing"), py_str("packed"), py_str("label"), py_str("tape"), py_str("flat pack"), py_str("flatpack"), py_str("shadow"), py_str("cardboard cutout")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("flat"), py_str("flat battery"), py_str("wont start"), py_str("won't start"), py_str("jump"), py_str("jump start"), py_str("charging"), py_str("charge"), py_str("charger"), py_str("dies"), py_str("dying"), py_str("last"), py_str("lasts"), py_str("life"), py_str("how long"), py_str("replace"), py_str("replacement"), py_str("test"), py_str("cold weather"), py_str("warning light")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("medicine"), py_str("medicines"), py_str("tablets"), py_str("pills"), py_str("capsules"), py_str("old medicine"), py_str("unused medicine"), py_str("medical waste"), py_str("syringe"), py_str("inhaler"), py_str("antibiotics"), py_str("painkillers"), py_str("antidepressants"), py_str("old tablets"), py_str("leftover tablets"), py_str("unused tablets"), py_str("expired tablets"), py_str("old pills"), py_str("leftover medicine"), py_str("insulin pen"), py_str("epipen")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("im going to"), py_str("going to"), py_str("gonna"), py_str("plan to"), py_str("skip"), py_str("skips"), py_str("skipping"), py_str("stopped"), py_str("stopping"), py_str("stop taking"), py_str("come off"), py_str("coming off"), py_str("cut down"), py_str("cutting down"), py_str("halve"), py_str("halving"), py_str("half"), py_str("reduce"), py_str("reducing"), py_str("increase"), py_str("increasing"), py_str("double"), py_str("doubling"), py_str("extra dose"), py_str("change my dose"), py_str("my dose"), py_str("taper"), py_str("wean"), py_str("instead of"), py_str("can i take"), py_str("should i take"), py_str("safe to take"), py_str("ok to take"), py_str("take one"), py_str("take two"), py_str("take them"), py_str("take it"), py_str("take with"), py_str("take for"), py_str("still taking"), py_str("dose"), py_str("dosage"), py_str("side effect"), py_str("side effects"), py_str("how many"), py_str("overdose"), py_str("missed"), py_str("forgot"), py_str("forget"), py_str("headache"), py_str("headaches"), py_str("head ache"), py_str("migraine"), py_str("migraines"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("sore"), py_str("toothache"), py_str("tooth ache"), py_str("stomach ache"), py_str("earache"), py_str("back pain"), py_str("period pain"), py_str("cramp"), py_str("cramps"), py_str("hurt"), py_str("hurts"), py_str("ill"), py_str("unwell"), py_str("symptom"), py_str("symptoms"), py_str("cure"), py_str("relief"), py_str("relieve"), py_str("ease"), py_str("calm down"), py_str("cold"), py_str("flu"), py_str("cough"), py_str("colds"), py_str("infection"), py_str("sick"), py_str("sickness"), py_str("fever"), py_str("temperature"), py_str("hangover"), py_str("allergy"), py_str("allergies"), py_str("hayfever"), py_str("hay fever"), py_str("insect bite"), py_str("sting")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("buy"), py_str("buying"), py_str("best"), py_str("which"), py_str("decent"), py_str("clean"), py_str("cleaning"), py_str("stain"), py_str("stains"), py_str("wash"), py_str("smell"), py_str("smells"), py_str("size"), py_str("firm"), py_str("soft")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("cold"), py_str("not cold"), py_str("freezing"), py_str("repair"), py_str("fix"), py_str("fixing"), py_str("smells"), py_str("smell"), py_str("noisy"), py_str("noise"), py_str("frost"), py_str("temperature"), py_str("ice"), py_str("not working"), py_str("wont"), py_str("won't"), py_str("leaking"), py_str("installing"), py_str("install"), py_str("second hand"), py_str("cheese"), py_str("butter"), py_str("yoghurt"), py_str("yogurt"), py_str("leftovers"), py_str("bacon"), py_str("ham"), py_str("eggs"), py_str("raw meat")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("limescale"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("musty"), py_str("filter"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("mildew"), py_str("black mould"), py_str("black mold"), py_str("not draining"), py_str("wont drain"), py_str("won't drain"), py_str("not spinning"), py_str("wont spin"), py_str("won't spin"), py_str("repair"), py_str("fix"), py_str("faulty"), py_str("not working"), py_str("broken"), py_str("error"), py_str("install"), py_str("installing"), py_str("plumb"), py_str("plumbing"), py_str("leaking"), py_str("leak"), py_str("noisy"), py_str("banging"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("scrap"), py_str("scrapes"), py_str("scraping"), py_str("rattle"), py_str("rattles"), py_str("rattling"), py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("squeal"), py_str("squeals"), py_str("squealing"), py_str("thump"), py_str("thumping"), py_str("knock"), py_str("knocking"), py_str("vibrate"), py_str("vibrating"), py_str("shaking"), py_str("how often"), py_str("which"), py_str("best"), py_str("buy"), py_str("buying"), py_str("bake"), py_str("baking"), py_str("roast"), py_str("cook"), py_str("cooking"), py_str("preheat"), py_str("temperature"), py_str("fan oven"), py_str("self clean")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("change"), py_str("changing"), py_str("replace"), py_str("replacement"), py_str("fitting"), py_str("watt"), py_str("watts"), py_str("bright"), py_str("brighter"), py_str("dimmable"), py_str("flicker"), py_str("flickering"), py_str("not working"), py_str("blown"), py_str("which"), py_str("best"), py_str("warm white"), py_str("energy")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("tv"), py_str("television"), py_str("telly"), py_str("laptop"), py_str("computer"), py_str("pc"), py_str("monitor"), py_str("printer"), py_str("kettle"), py_str("toaster"), py_str("hoover"), py_str("vacuum cleaner"), py_str("phone"), py_str("mobile"), py_str("mobile phone"), py_str("old phone"), py_str("iphone"), py_str("tablet"), py_str("ipad"), py_str("charger"), py_str("cable"), py_str("electricals"), py_str("electronics"), py_str("appliance"), py_str("appliances"), py_str("game console"), py_str("xbox"), py_str("playstation")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("limescale"), py_str("scale"), py_str("descale"), py_str("furred"), py_str("virus"), py_str("viruses"), py_str("pop up"), py_str("popup"), py_str("malware"), py_str("spyware"), py_str("adware"), py_str("software"), py_str("app"), py_str("apps"), py_str("update"), py_str("browser"), py_str("email"), py_str("wifi"), py_str("program"), py_str("programs"), py_str("uninstall"), py_str("uninstalling"), py_str("uninstalled"), py_str("reinstall"), py_str("wi-fi"), py_str("speed"), py_str("slow"), py_str("taking ages"), py_str("taking so long"), py_str("taking long"), py_str("is taking"), py_str("takes ages"), py_str("takes forever"), py_str("took a"), py_str("took one"), py_str("took it"), py_str("took my"), py_str("swallow"), py_str("swallowed"), py_str("pill"), py_str("pills"), py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("migraines"), py_str("toothache"), py_str("earache"), py_str("sore throat"), py_str("stomach ache"), py_str("cramp"), py_str("cramps"), py_str("nausea"), py_str("rash"), py_str("itching"), py_str("pain"), py_str("pains"), py_str("aching"), py_str("hurts"), py_str("hurting"), py_str("fever"), py_str("medicine"), py_str("dose"), py_str("dies"), py_str("dying"), py_str("lunchtime"), py_str("battery life"), py_str("cushion"), py_str("cushions"), py_str("tablets"), py_str("painkillers"), py_str("paracetamol"), py_str("ibuprofen"), py_str("prescription"), py_str("headache"), py_str("migraine"), py_str("symptom"), py_str("symptoms"), py_str("without tablets"), py_str("without pills"), py_str("contract"), py_str("contracts"), py_str("bill"), py_str("bills"), py_str("tariff"), py_str("upgrade"), py_str("unlock"), py_str("network"), py_str("sim"), py_str("number"), py_str("account"), py_str("cracked"), py_str("screen"), py_str("not working"), py_str("repair"), py_str("fix"), py_str("warranty"), py_str("no signal"), py_str("slow"), py_str("storage"), py_str("battery life"), py_str("charge"), py_str("charging"), py_str("watch"), py_str("channel"), py_str("channels"), py_str("remote"), py_str("print"), py_str("printing"), py_str("cartridge"), py_str("boil"), py_str("boiling"), py_str("toast"), py_str("vacuum"), py_str("hoovering"), py_str("recycle bin"), py_str("recycling bin"), py_str("trash can"), py_str("trash"), py_str("empty the trash"), py_str("deleted"), py_str("delete"), py_str("folder"), py_str("folders"), py_str("files"), py_str("desktop"), py_str("dust"), py_str("dusty"), py_str("dusted"), py_str("cobweb"), py_str("cobwebs"), py_str("settling"), py_str("smear"), py_str("smears"), py_str("smudges"), py_str("fingerprints"), py_str("polish"), py_str("polishing"), py_str("wipe"), py_str("wiping"), py_str("cleaning"), py_str("clean the")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("clothes"), py_str("clothing"), py_str("old clothes"), py_str("shoes"), py_str("trainers"), py_str("coats"), py_str("jumpers"), py_str("textiles"), py_str("rags"), py_str("bedding"), py_str("duvet"), py_str("pillows"), py_str("towels"), py_str("socks"), py_str("sock"), py_str("pants"), py_str("underwear"), py_str("knickers"), py_str("bras"), py_str("gloves"), py_str("scarves"), py_str("hats"), py_str("sheets"), py_str("curtains"), py_str("fabric"), py_str("linen"), py_str("blankets"), py_str("slippers"), py_str("boots"), py_str("sandals")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("pet"), py_str("pets"), py_str("died"), py_str("dies"), py_str("passed away"), py_str("gone"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("baby"), py_str("hospital"), py_str("his"), py_str("her bed"), py_str("wash"), py_str("washing"), py_str("shrink"), py_str("shrank"), py_str("stain"), py_str("stains"), py_str("iron"), py_str("ironing"), py_str("fit"), py_str("size"), py_str("wear it"), py_str("wear them"), py_str("to wear"), py_str("smell"), py_str("smells"), py_str("dry"), py_str("drying"), py_str("hole"), py_str("holes"), py_str("repair"), py_str("refashion"), py_str("style"), py_str("outfit"), py_str("what to wear"), py_str("static"), py_str("static electricity"), py_str("clinging"), py_str("cling"), py_str("moth"), py_str("moths"), py_str("mothball"), py_str("mothballs"), py_str("lint"), py_str("bobbles"), py_str("bobbling"), py_str("pilling"), py_str("pills"), py_str("chewing gum"), py_str("gum"), py_str("oil"), py_str("grease"), py_str("paint"), py_str("ink"), py_str("blood"), py_str("wine"), py_str("coffee"), py_str("tea"), py_str("chocolate"), py_str("mud"), py_str("grass"), py_str("marker"), py_str("pen"), py_str("sick"), py_str("vomit"), py_str("wee"), py_str("urine"), py_str("out of my"), py_str("off my"), py_str("off the"), py_str("out of the"), py_str("flower bed"), py_str("flower beds"), py_str("flowerbed"), py_str("garden"), py_str("gardens"), py_str("lawn"), py_str("lawns"), py_str("border"), py_str("borders"), py_str("allotment"), py_str("allotments"), py_str("weed"), py_str("weeds"), py_str("weeding"), py_str("plant"), py_str("plants"), py_str("shrub"), py_str("shrubs"), py_str("rose"), py_str("roses"), py_str("tomato"), py_str("tomatoes"), py_str("greenhouse"), py_str("compost")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("sofa"), py_str("couch"), py_str("settee"), py_str("armchair"), py_str("wardrobe"), py_str("table"), py_str("chairs"), py_str("bookcase"), py_str("furniture"), py_str("sideboard"), py_str("dresser"), py_str("desk"), py_str("cabinet")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("cushion"), py_str("cushions"), py_str("new"), py_str("replacement"), py_str("cover"), py_str("covers"), py_str("which"), py_str("start"), py_str("not comfortable"), py_str("uncomfortable"), py_str("wobbly"), py_str("squeaks"), py_str("squeaking"), py_str("moths"), py_str("moth"), py_str("mice"), py_str("mouse"), py_str("rats"), py_str("ant"), py_str("ants"), py_str("flies"), py_str("bedbugs"), py_str("woodworm"), py_str("mould"), py_str("mold"), py_str("damp"), py_str("silverfish"), py_str("cockroach"), py_str("spiders"), py_str("mark"), py_str("marks"), py_str("water mark"), py_str("water marks"), py_str("ring"), py_str("rings"), py_str("heat mark"), py_str("scratch"), py_str("scratches"), py_str("dent"), py_str("dents"), py_str("gouge"), py_str("chipped"), py_str("polish"), py_str("polishing"), py_str("clean"), py_str("cleaning"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("upstairs"), py_str("delivery"), py_str("assemble"), py_str("assembling"), py_str("flat pack"), py_str("cover"), py_str("repair"), py_str("buy"), py_str("buying"), py_str("best"), py_str("which"), py_str("arrange"), py_str("layout"), py_str("small room")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("book"), py_str("books"), py_str("old books"), py_str("paperback"), py_str("paperbacks"), py_str("novels"), py_str("textbooks"), py_str("magazines"), py_str("comics"), py_str("encyclopaedias"), py_str("encyclopedias"), py_str("cookbooks"), py_str("book shelf"), py_str("bookshelf"), py_str("bookcase full of books")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("book a"), py_str("book the"), py_str("reserve"), py_str("reservation"), py_str("table"), py_str("hotel"), py_str("flight"), py_str("flights"), py_str("train"), py_str("tickets"), py_str("appointment"), py_str("read"), py_str("reading"), py_str("reads"), py_str("recommend"), py_str("recommendation"), py_str("recommendations"), py_str("what book"), py_str("which book"), py_str("good book"), py_str("best book"), py_str("books for"), py_str("book for"), py_str("kindle"), py_str("e reader"), py_str("e-reader"), py_str("audiobook"), py_str("library"), py_str("borrow"), py_str("store"), py_str("storage"), py_str("shelf"), py_str("shelves"), py_str("display")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("toy"), py_str("toys"), py_str("old toys"), py_str("lego"), py_str("duplo"), py_str("playmobil"), py_str("teddy"), py_str("teddies"), py_str("cuddly toys"), py_str("soft toys"), py_str("dolls"), py_str("action figures"), py_str("train set"), py_str("board games"), py_str("jigsaws"), py_str("puzzles"), py_str("childrens toys"), py_str("kids toys")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("buy"), py_str("buying"), py_str("best"), py_str("which"), py_str("for a 3"), py_str("for a 2"), py_str("for a 4"), py_str("for a 5"), py_str("year old"), py_str("years old"), py_str("gift"), py_str("present"), py_str("birthday"), py_str("christmas"), py_str("play with"), py_str("clean"), py_str("cleaning"), py_str("sterilise"), py_str("wash"), py_str("smell"), py_str("recall"), py_str("safety"), py_str("choking"), py_str("store"), py_str("storage"), py_str("tidy"), py_str("organise"), py_str("floor")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("ride"), py_str("riding"), py_str("rider"), py_str("buy"), py_str("buying"), py_str("which"), py_str("best"), py_str("cheap"), py_str("repair"), py_str("fix"), py_str("mend"), py_str("puncture"), py_str("tyre"), py_str("tyres"), py_str("chain"), py_str("brakes"), py_str("gears"), py_str("saddle"), py_str("lock"), py_str("helmet"), py_str("lights"), py_str("electric"), py_str("e-bike"), py_str("how far"), py_str("how long"), py_str("train"), py_str("lose weight"), py_str("fitness"), py_str("commute"), py_str("storage"), py_str("store"), py_str("shed"), py_str("carrier")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("belly"), py_str("lose"), py_str("losing"), py_str("burn"), py_str("burning"), py_str("diet"), py_str("weight"), py_str("body"), py_str("stomach"), py_str("abs"), py_str("waist"), py_str("tummy"), py_str("thigh"), py_str("thighs"), py_str("hips"), py_str("flabby"), py_str("fat around"), py_str("fat round"), py_str("bingo wings"), py_str("muffin top"), py_str("spot reduce"), py_str("calories"), py_str("reduce"), py_str("cut down"), py_str("best"), py_str("healthy"), py_str("healthiest"), py_str("which oil"), py_str("olive"), py_str("rapeseed"), py_str("smoke point"), py_str("temperature"), py_str("how hot"), py_str("calories"), py_str("reuse"), py_str("reusing"), py_str("use again"), py_str("sink"), py_str("blocked"), py_str("drain"), py_str("smell"), py_str("clothes"), py_str("clothing"), py_str("shirt"), py_str("shirts"), py_str("trousers"), py_str("jeans"), py_str("dress"), py_str("dresses"), py_str("jumper"), py_str("jumpers"), py_str("top"), py_str("tops"), py_str("t shirt"), py_str("tshirt"), py_str("hoodie"), py_str("cardigan"), py_str("coat"), py_str("jacket"), py_str("fabric"), py_str("carpet"), py_str("rug"), py_str("sofa"), py_str("settee"), py_str("couch"), py_str("upholstery"), py_str("floor"), py_str("tiles"), py_str("wall"), py_str("walls"), py_str("worktop"), py_str("drive"), py_str("driveway"), py_str("patio"), py_str("shoes"), py_str("trainers")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("garden waste"), py_str("grass cuttings"), py_str("prunings"), py_str("hedge cuttings"), py_str("branches"), py_str("fallen leaves"), py_str("dead leaves"), py_str("autumn leaves"), py_str("leaves in the garden"), py_str("leaves in my garden"), py_str("leaves from the garden"), py_str("leaves from my garden"), py_str("leaves on the lawn"), py_str("leaves on my lawn"), py_str("rake the leaves"), py_str("raked leaves"), py_str("leaf litter"), py_str("bag of leaves"), py_str("bags of leaves"), py_str("get rid of leaves"), py_str("lawn clippings"), py_str("green waste"), py_str("compost"), py_str("green bin"), py_str("green bins"), py_str("in the green bin"), py_str("brown bin"), py_str("brown bins"), py_str("in the brown bin"), py_str("green wheely bin"), py_str("garden waste bin")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("compost bin"), py_str("how to compost"), py_str("composting"), py_str("bin smells"), py_str("smell"), py_str("rats"), py_str("flies"), py_str("what goes in"), py_str("what can go in"), py_str("what can i put in"), py_str("recycling"), py_str("card and paper"), py_str("when to cut"), py_str("cut back"), py_str("prune"), py_str("pruning"), py_str("mow"), py_str("mowing"), py_str("lawn"), py_str("brown tips"), py_str("brown at the tips"), py_str("tips of the leaves"), py_str("leaf edges"), py_str("houseplant"), py_str("houseplants"), py_str("indoor plant"), py_str("pot plant"), py_str("yellow leaves"), py_str("yellowing")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("decorate"), py_str("decorating"), py_str("lights"), py_str("when should i put up"), py_str("take down"), py_str("artificial"), py_str("which tree"), py_str("buy"), py_str("best"), py_str("price"), py_str("cost"), py_str("needle"), py_str("needles"), py_str("drop"), py_str("dropping")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("chemicals"), py_str("weedkiller"), py_str("bleach"), py_str("white spirit"), py_str("thinners"), py_str("paint thinner"), py_str("pesticide"), py_str("antifreeze"), py_str("drain cleaner"), py_str("caustic soda"), py_str("ammonia"), py_str("old petrol"), py_str("paraffin"), py_str("gas bottle"), py_str("gas canister"), py_str("camping gas")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("mix"), py_str("mixing"), py_str("how much"), py_str("dose"), py_str("use"), py_str("using"), py_str("safe"), py_str("dangerous"), py_str("fumes"), py_str("smell"), py_str("kill"), py_str("weeds"), py_str("mould"), py_str("mold"), py_str("clean"), py_str("cleaning"), py_str("drain"), py_str("blocked"), py_str("unblock"), py_str("stain"), py_str("stains"), py_str("toilet"), py_str("sink"), py_str("shower"), py_str("grout")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("clean"), py_str("hang"), py_str("hanging"), py_str("mount"), py_str("steam"), py_str("streak"), py_str("smear"), py_str("smears"), py_str("crack"), py_str("chipped"), py_str("replace"), py_str("which glass"), py_str("wine glass"), py_str("bake"), py_str("baking"), py_str("ovenproof"), py_str("cut"), py_str("cutting")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("use"), py_str("using"), py_str("safe"), py_str("dangerous"), py_str("explode"), py_str("heat"), py_str("hot"), py_str("cold"), py_str("spray"), py_str("spraying"), py_str("paint"), py_str("paint the"), py_str("squeak"), py_str("squeaky"), py_str("hinge"), py_str("rust"), py_str("lock")]) }
    fn __data4() -> Py { py_tuple(vec![py_tuple(vec![py_str("pizza-box"), __data5(), py_str("Tear off the clean lid and put that in the recycling; the greasy base goes in the general waste, or in the food waste if your council takes that. Cheese and oil soaked into card spoil the load at the sorting plant, so a soaked box is the one thing worth separating."), __data6()]), py_tuple(vec![py_str("cardboard-boxes"), __data7(), py_str("Flatten them and keep them dry: clean cardboard goes out with the kerbside recycling, cut down to the size your council takes, and tape, polystyrene and any food grease keep a box out of it. More than the bin will take is a bulky collection or a trip to the recycling centre, and removal boxes in good nick go fast on a local free-and-wanted group."), __data8(), Py::Int(2)]), py_tuple(vec![py_str("car-battery"), py_tuple(vec![py_str("car battery"), py_str("car batteries"), py_str("van battery"), py_str("leisure battery"), py_str("motorcycle battery")]), py_str("A garage will take the old one when it fits a new one, and a scrap dealer or a metal merchant buys it by weight, so it is worth a few pounds rather than a trip to the tip - which takes them too. Never put it in the bin or leave it in the garage for years: it holds acid and lead."), __data9()]), py_tuple(vec![py_str("medicines"), __data10(), py_str("Take them back to a pharmacy: every pharmacy takes unwanted medicines for safe disposal, and you do not need to explain. Don't flush them and don't bin them - they reach the water supply, and a child or a pet can find them. Leave them in their packet with the label on, and put a used inhaler in with them rather than in the bin."), __data11()]), py_tuple(vec![py_str("mattress"), py_tuple(vec![py_str("mattress"), py_str("mattresses"), py_str("old bed"), py_str("bed frame"), py_str("divan"), py_str("headboard")]), py_str("Book a bulky waste collection with your council - most charge twenty to forty pounds and collect from the doorstep - or take it to the recycling centre, where it is stripped for its springs. Charity shops rarely take one for hygiene reasons, so a free-ad site is the other realistic route if it is clean and you say collection only."), __data12()]), py_tuple(vec![py_str("fridge"), py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("fridge freezer"), py_str("chest freezer")]), py_str("A fridge or freezer holds refrigerant gas that has to be taken out by someone equipped for it, so it can never go out with the rubbish. The shop selling you the new one has to take the old one away on a like-for-like basis, often free or for a small fee, and the recycling centre takes it. Take the doors off before it leaves the kitchen if children or pets will be near it."), __data13()]), py_tuple(vec![py_str("white-goods"), py_tuple(vec![py_str("washing machine"), py_str("washer"), py_str("tumble dryer"), py_str("dryer"), py_str("dishwasher"), py_str("oven"), py_str("cooker"), py_str("microwave"), py_str("hob"), py_str("fridge unit")]), py_str("The shop delivering a new one has to take the old one away on a like-for-like basis, and some charge a small fee for it, so ask when you buy. Otherwise the tip has a skip for white goods, or a scrap dealer collects for the metal - and a machine that still works will go on a free-ad site in a day. Unplug it, drain the water out of the filter, and tape the door shut before it leaves the kitchen."), __data14(), Py::Int(2)]), py_tuple(vec![py_str("bulbs"), py_tuple(vec![py_str("light bulb"), py_str("light bulbs"), py_str("bulb"), py_str("bulbs"), py_str("halogen"), py_str("fluorescent"), py_str("cfl"), py_str("led bulb"), py_str("led bulbs"), py_str("strip light"), py_str("tube light")]), py_str("Old-style filament and halogen bulbs are just glass and wire, so they go in the bin. The coiled low-energy ones hold a little mercury: take those to the tip or to a shop that sells them. If one breaks, air the room for fifteen minutes, don't vacuum, and wipe it up with a damp paper towel. LEDs are electronics, so they belong with the small appliances at the tip."), __data15()]), py_tuple(vec![py_str("electricals"), __data16(), py_str("Anything with a plug, a battery or a cable is electrical waste and can't go in the bin. A shop that sells you a new one has to take the old one back, like for like, and the tip has a small-appliance skip. Wipe a phone or laptop and take the SIM and memory card out first; a charity shop or a repair cafe can make use of one that still works."), __data17()]), py_tuple(vec![py_str("clothes"), __data18(), py_str("Charity shops, the clothes banks in car parks and the textile skip at the tip take anything wearable. Worn-out fabric still earns its place in a textile bank, where it becomes rags or stuffing, so a bag of split trousers is not waste. Wash it first, put shoes in pairs, and a duvet or pillow that is past it goes to the textile bank rather than the bin."), __data19()]), py_tuple(vec![py_str("furniture"), __data20(), py_str("Three routes: a council bulky waste collection (usually charged), the tip if you can drive it there, or a charity that collects - the British Heart Foundation takes sofas and wardrobes free if they have their fire labels and can come down the stairs. A free-ad site clears most furniture within a week if you say collection only."), __data21()]), py_tuple(vec![py_str("books"), __data22(), py_str("Ring the charity shop before you load the car: most take a bag or two of books and say no to a bootful, and a book bank at the tip or a supermarket car park takes as many as you have. Paperbacks go down well at a hospital, a care home or a phone box library, and a school fair clears children's books. Anything mouldy, damp or water-damaged goes in the paper recycling, not to a shop."), __data23()]), py_tuple(vec![py_str("toys"), __data24(), py_str("Charity shops take toys only if they are complete and clean, so a nursery, a sure start centre or a refuge will take what a shop will not, and a free-ad site clears a box of lego in a day. Wash what you have and put the pieces back in their boxes first. A car seat or a cot is the exception: charity shops cannot take them, so it goes to the tip unless a friend wants it, and broken plastic is general waste."), __data25()]), py_tuple(vec![py_str("bikes"), py_tuple(vec![py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("bicycles"), py_str("old bike"), py_str("kids bike"), py_str("childs bike"), py_str("mountain bike"), py_str("push bike")]), py_str("A bike project or a charity that refurbishes them takes one that does not ride: Re-Cycle and most city bike workshops want frames and parts, and a free-ad listing marked collection only clears one in a day. A scrap dealer buys the frame by weight, and the tip takes it with the metal. Charity shops rarely take bikes, so ring the project first."), __data26()]), py_tuple(vec![py_str("polystyrene"), py_tuple(vec![py_str("polystyrene"), py_str("styrofoam"), py_str("packaging"), py_str("packing peanuts"), py_str("bubble wrap")]), py_str("The white beaded stuff (expanded polystyrene) is not collected at the kerbside in most places, so it goes in the general waste - break it up so it takes less room, and put the pieces in a bag so they don't blow about. A few tips take it if you check your council's A to Z page, and the hard plastic trays often go in with the plastics."), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("food"), py_str("insulate"), py_str("insulation"), py_str("sound"), py_str("noise"), py_str("craft"), py_str("paint"), py_str("glue"), py_str("fix"), py_str("filler")])]), py_tuple(vec![py_str("oil"), py_tuple(vec![py_str("cooking oil"), py_str("used oil"), py_str("frying oil"), py_str("old oil"), py_str("chip pan"), py_str("deep fryer"), py_str("fat"), py_str("grease"), py_str("lard"), py_str("turkey fat"), py_str("bacon fat")]), py_str("Let it cool, pour it into a bottle or a tub, and put that in the general waste - never down the sink, where it sets with wipes into a blockage. A little fat can go in the food waste caddy if your council collects one, and a recycling centre takes larger amounts of used cooking oil."), __data27()]), py_tuple(vec![py_str("garden-waste"), __data28(), py_str("The council's green bin is the cheapest route if you have one - many charge for the subscription and collect fortnightly - and the tip has a green waste skip. Grass, leaves and prunings can go in a compost bin at home too, but leave out cooked food, weeds in seed and anything diseased."), __data29()]), py_tuple(vec![py_str("christmas-tree"), py_tuple(vec![py_str("christmas tree"), py_str("real tree"), py_str("xmas tree"), py_str("old christmas tree")]), py_str("Councils collect real trees in the first couple of January weeks and shred them for mulch - check the day on the council's site, and strip the decorations and the stand first. Garden centres often take them back, and one that still has its roots in a pot can be planted out instead."), __data30()]), py_tuple(vec![py_str("chemicals"), __data31(), py_str("Anything with a hazard symbol on it is household chemicals, and the tip's hazardous waste skip is where it goes - most sites take it from householders, and a few councils run set days, so ring before you drive over. Keep it in its own labelled container, never mix two together, and never pour it down the drain or into the bin. A gas bottle goes back to the shop that sells them."), __data32()]), py_tuple(vec![py_str("mirror"), py_tuple(vec![py_str("mirror"), py_str("mirrors"), py_str("pyrex"), py_str("glass dish"), py_str("drinking glass"), py_str("window glass"), py_str("broken glass"), py_str("vase"), py_str("ceramic"), py_str("crockery"), py_str("plate")]), py_str("Mirrors, Pyrex and drinking glasses are not bottle-bank glass: they are made to take heat, so they melt at a different temperature and spoil the batch. Wrap it so nobody cuts themselves and put it in the general waste, or take it to the tip."), __data33()]), py_tuple(vec![py_str("aerosols"), py_tuple(vec![py_str("aerosol"), py_str("aerosols"), py_str("spray can"), py_str("spray cans"), py_str("deodorant can"), py_str("hairspray"), py_str("paint spray"), py_str("wd40"), py_str("wd-40")]), py_str("An empty aerosol can go in the kerbside recycling with the tins in most areas - check the council's list, and don't pierce or crush it. One that still has something in it is not rubbish: it goes to the tip, and a spray paint or a solvent goes to the hazardous waste."), __data34()])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.ITEMS"); }
    s_ITEMS(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.BIN_DAY"); }
    s_BIN_DAY(py_str("Your council's bin-day page will say: put your postcode in and it gives the day for your street, and that is the only place that is right, because bank holidays and ice move a round by a day or two and the paper calendar on the fridge goes stale. If it says tomorrow, put it out tonight; either way it has to be out by about 6.30 in the morning, so the night before is safest, and it comes back in the same day. If it is missed, report it on the council's missed-bin form that day rather than ringing - it is free and the round usually comes back within two working days."));
    fn __data35() -> Py { py_tuple(vec![py_str("bin day"), py_str("bins out"), py_str("bin out"), py_str("the bins out"), py_str("put my bin out"), py_str("put the bin out"), py_str("put the bins out"), py_str("put my bins out"), py_str("put the rubbish out"), py_str("put my rubbish out"), py_str("put the recycling out"), py_str("put the green bin out"), py_str("bins go out"), py_str("bin goes out"), py_str("collection day"), py_str("collection days"), py_str("bin collection"), py_str("bins collected"), py_str("bin collected"), py_str("bins are collected"), py_str("bin is collected"), py_str("bins emptied"), py_str("bin emptied"), py_str("bins are emptied"), py_str("bin men"), py_str("bin lorry"), py_str("bin round"), py_str("bin crew"), py_str("wheelie bin day"), py_str("recycling collected"), py_str("recycling is collected"), py_str("recycling emptied"), py_str("recycling collection"), py_str("rubbish collected"), py_str("rubbish is collected"), py_str("rubbish collection"), py_str("refuse collection"), py_str("garden waste collection"), py_str("my recycling"), py_str("my rubbish")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("when"), py_str("what time"), py_str("what day"), py_str("which day"), py_str("what days"), py_str("how often"), py_str("find out"), py_str("check"), py_str("due"), py_str("should i put"), py_str("do i put"), py_str("am i supposed to put"), py_str("is it"), py_str("are they"), py_str("know"), py_str("changed"), py_str("changes"), py_str("has changed"), py_str("have changed"), py_str("change of"), py_str("has moved"), py_str("moved to"), py_str("moved day"), py_str("collection moved"), py_str("bin day moved"), py_str("swapped"), py_str("rescheduled"), py_str("new day"), py_str("different day"), py_str("which morning"), py_str("what morning")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.BIN_DAY_KEY"); }
    s_BIN_DAY_KEY(py_tuple(vec![__data35(), __data36()]));
    fn __data37() -> Py { py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelly"), py_str("stink"), py_str("stinks"), py_str("maggots"), py_str("wash"), py_str("clean"), py_str("cleaning"), py_str("rats"), py_str("rat"), py_str("mice"), py_str("mouse"), py_str("fox"), py_str("foxes"), py_str("seagull"), py_str("seagulls"), py_str("cat"), py_str("cats"), py_str("missed"), py_str("wasnt collected"), py_str("wasn't collected"), py_str("not collected"), py_str("didnt collect"), py_str("didn't collect"), py_str("havent collected"), py_str("haven't collected"), py_str("still there"), py_str("overflowing"), py_str("where do i put"), py_str("where can i put"), py_str("which bin"), py_str("what goes in"), py_str("what can i put in"), py_str("can i put"), py_str("how do i get rid"), py_str("get rid of"), py_str("tip"), py_str("recycling centre"), py_str("lid"), py_str("bags"), py_str("bin bags"), py_str("bin bag"), py_str("lock"), py_str("key"), py_str("replace"), py_str("buy"), py_str("size"), py_str("how big")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.BIN_DAY_AVOID"); }
    s_BIN_DAY_AVOID(__data37());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.WHICH_BIN"); }
    s_WHICH_BIN(py_str("That depends on the item: anything with a plug or a battery goes back to a shop or the tip, chemicals go to the tip's hazardous skip, and bulky things are a council collection. Which item is it?"));
    fn __data38() -> Py { py_tuple(vec![py_str("tin"), py_str("tins"), py_str("tin can"), py_str("tin cans"), py_str("cans"), py_str("drink can"), py_str("drink cans"), py_str("food tin"), py_str("food tins"), py_str("baked bean tin"), py_str("soup tin"), py_str("pet food tin"), py_str("aluminium")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("paper"), py_str("newspaper"), py_str("newspapers"), py_str("card"), py_str("cardboard"), py_str("carton"), py_str("cartons"), py_str("junk mail"), py_str("envelope"), py_str("envelopes"), py_str("magazine"), py_str("magazines"), py_str("cereal box"), py_str("cereal boxes")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("plastic bottle"), py_str("plastic bottles"), py_str("bottle"), py_str("bottles"), py_str("tub"), py_str("tubs"), py_str("yoghurt pot"), py_str("yoghurt pots"), py_str("margarine tub"), py_str("plastic tub"), py_str("plastic tubs"), py_str("milk bottle"), py_str("shampoo bottle")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("clothes"), py_str("clothing"), py_str("old clothes"), py_str("textiles"), py_str("textile"), py_str("shoes"), py_str("old shoes"), py_str("curtains"), py_str("bedding"), py_str("duvet"), py_str("duvets"), py_str("towels"), py_str("rags")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.ITEM_ROUTES"); }
    s_ITEM_ROUTES(py_tuple(vec![py_tuple(vec![py_str("tins"), __data38(), py_str("Tins and cans go in the kerbside recycling: rinse them, leave them whole and push the lid inside, and do not flatten them - the plant sorts steel from aluminium, and a flattened tin can be read as the wrong metal. An aerosol is the one can that goes back to a shop or the tip if it still has something in it.")]), py_tuple(vec![py_str("foil"), py_tuple(vec![py_str("foil"), py_str("tin foil"), py_str("kitchen foil"), py_str("foil tray"), py_str("foil trays"), py_str("foil container"), py_str("bacofoil")]), py_str("Clean foil goes in the kerbside recycling, scrunched into a ball about the size of a fist so the machine can pick it out - loose foil and a foil tray with food stuck to it do not get sorted, and a greasy one goes in the general waste.")]), py_tuple(vec![py_str("jars and bottles"), py_tuple(vec![py_str("jar"), py_str("jars"), py_str("glass jar"), py_str("glass jars"), py_str("glass bottle"), py_str("glass bottles"), py_str("wine bottle"), py_str("beer bottle"), py_str("jam jar"), py_str("sauce jar")]), py_str("Glass jars and bottles go in the kerbside recycling: empty and rinsed, lids off, and the lids go in with the metal if they are metal. Drinking glasses, Pyrex and ovenware are a different glass and go in the general waste, because they melt at the wrong temperature.")]), py_tuple(vec![py_str("paper and card"), __data39(), py_str("Paper and card go in the kerbside recycling, flattened and dry: envelopes with a window, cereal boxes and junk mail all go in. A food carton or a tetra pak is only collected where the council says so, and a greasy pizza box goes in the general waste apart from its clean lid.")]), py_tuple(vec![py_str("plastic bottles and tubs"), __data40(), py_str("Plastic bottles and tubs go in the kerbside recycling: rinse them, squash the bottles, and leave the lids on or off as your council says, because both are collected in most places now. A black plastic tray is the one that is often not sorted, and your council's A to Z page will say.")]), py_tuple(vec![py_str("soft plastic"), py_tuple(vec![py_str("crisp packet"), py_str("crisp packets"), py_str("carrier bag"), py_str("carrier bags"), py_str("plastic bag"), py_str("plastic bags"), py_str("bread bag"), py_str("film lid"), py_str("cling film"), py_str("bubble wrap"), py_str("soft plastic"), py_str("padded envelope")]), py_str("Soft plastic - carrier bags, crisp packets, bread bags, film lids, bubble wrap - is not collected at the kerbside in most places. The big supermarkets take it in a bin by the door, and the collected plastic is turned into bin bags and pellets. A padded envelope is the same, and it is worth keeping a bag by the door for it.")]), py_tuple(vec![py_str("polystyrene"), py_tuple(vec![py_str("polystyrene"), py_str("styrofoam"), py_str("packing peanuts"), py_str("meat tray"), py_str("meat trays"), py_str("takeaway box"), py_str("takeaway boxes")]), py_str("Polystyrene goes in the general waste, not the recycling: expanded foam is mostly air, and no kerbside service takes it. A clean meat tray is the exception in some areas because some trays are the recyclable kind - squeeze it, and if it springs back it is foam and goes in the bin.")]), py_tuple(vec![py_str("nappies"), py_tuple(vec![py_str("nappy"), py_str("nappies"), py_str("diaper"), py_str("diapers"), py_str("baby wipes"), py_str("wet wipes"), py_str("sanitary"), py_str("sanitary pad"), py_str("sanitary pads"), py_str("tampon"), py_str("tampons"), py_str("incontinence")]), py_str("Nappies, wipes, pads and tampons all go in the general waste - nothing absorbent is recycled, and a wet wipe is not flushable however the packet reads, because it does not break down and it is most of what blocks a sewer. Bag it and bin it; a nappy bin with a lid keeps the room livable.")]), py_tuple(vec![py_str("clothes and textiles"), __data41(), py_str("Clothes, shoes, bedding and curtains do not go in the kerbside bin: the charity bank or the textile bank at the supermarket car park takes them, even worn ones, and they are sorted into resale, rags and fibre. A duvet and a pillow are the awkward ones - most banks will not take them, so ring the council for a bulky collection or take them to the tip's textile container.")])]));
    fn __data42() -> Py { py_tuple(vec![py_str("collection"), py_str("collected"), py_str("collection day"), py_str("missed"), py_str("empty"), py_str("emptied"), py_str("smell"), py_str("smells"), py_str("rats"), py_str("mice"), py_str("fox"), py_str("foxes"), py_str("seagull"), py_str("seagulls"), py_str("bags"), py_str("lid"), py_str("wheelie"), py_str("what goes in"), py_str("how often"), py_str("bins out"), py_str("bin out"), py_str("put the bins"), py_str("the bins out"), py_str("bin day"), py_str("what time"), py_str("tonight"), py_str("tomorrow"), py_str("this morning"), py_str("last night"), py_str("recycling bin"), py_str("tips of"), py_str("brown tips"), py_str("white tips"), py_str("split ends"), py_str("dry tips"), py_str("fingertips"), py_str("finger tips"), py_str("nails"), py_str("hair"), py_str("plant"), py_str("plants"), py_str("leaves")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.WHICH_BIN_AVOID"); }
    s_WHICH_BIN_AVOID(__data42());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.RECYCLE"); }
    s_RECYCLE(py_str("Roughly: flattened card and paper, glass jars and bottles, tins and cans, plastic bottles and tubs, and clean foil - rinsed, loose rather than bagged, because a bag of the right things still goes to waste. Soft plastic (carrier bags, crisp packets, film lids) is not collected at the kerbside in most places: the big supermarkets take it in a bin by the door. For anything odd, the council's A to Z page has a line for it."));
    fn __data43() -> Py { py_tuple(vec![py_str("first aid"), py_str("kit"), py_str("sandwich"), py_str("lunch"), py_str("lunchbox"), py_str("picnic"), py_str("stocking"), py_str("stockings"), py_str("hamper"), py_str("hospital bag"), py_str("empty"), py_str("emptied"), py_str("computer"), py_str("laptop"), py_str("windows"), py_str("mac"), py_str("file"), py_str("files"), py_str("desktop"), py_str("disk"), py_str("drive"), py_str("deleted"), py_str("delete"), py_str("restore"), py_str("email"), py_str("folder"), py_str("folders"), py_str("phone"), py_str("hard drive"), py_str("bin day"), py_str("bins out"), py_str("put the bins"), py_str("collection day"), py_str("what time"), py_str("bin smells"), py_str("smell"), py_str("smells"), py_str("limescale"), py_str("phone contract")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.RECYCLE_AVOID"); }
    s_RECYCLE_AVOID(op_add(&__data43(), &g_A_HOURS_ASK()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.TIP"); }
    s_TIP(py_str("Load it so it can be tipped in one go, and put the awkward things on top: a car load usually drives straight in, but many sites ask for proof of address, and a van or a trailer is often booked. Paint, chemicals, gas bottles, batteries and anything with a screen go to a staffed container or to a steward rather than into a skip, so ask at the gate. Soil, rubble and plasterboard are usually limited to a few bags a visit. If you cannot drive it there, the council's bulky waste collection is the other route: usually charged, booked online or by phone, and they take it from the doorstep."));
    fn __data44() -> Py { py_tuple(vec![py_str("any tips"), py_str("tips for"), py_str("tip in"), py_str("tip on"), py_str("the waiter"), py_str("waitress"), py_str("restaurant"), py_str("pub"), py_str("percent"), py_str("gratuity"), py_str("tidy"), py_str("garden tip"), py_str("work out a tip"), py_str("work out the tip"), py_str("work out a service"), py_str("work out the service"), py_str("how do i tip"), py_str("how to tip"), py_str("how should i tip"), py_str("buried"), py_str("bury"), py_str("burying"), py_str("instead of"), py_str("ended up"), py_str("how much"), py_str("cost"), py_str("charge"), py_str("price"), py_str("open"), py_str("closing"), py_str("opening"), py_str("what time"), py_str("close"), py_str("calculate a tip"), py_str("calculate the tip"), py_str("calculate a service"), py_str("calculate the service"), py_str("how much should i tip"), py_str("how much do i tip"), py_str("nearest"), py_str("where is"), py_str("council tax"), py_str("holiday"), py_str("bank holiday"), py_str("tips of"), py_str("brown tips"), py_str("white tips"), py_str("split ends"), py_str("dry tips"), py_str("fingertips"), py_str("finger tips"), py_str("nails"), py_str("hair"), py_str("plant"), py_str("plants"), py_str("leaves")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.TIP_AVOID"); }
    s_TIP_AVOID(__data44());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish._CONTAINERS"); }
    s__CONTAINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ITEMS()) { let mut item: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&item.clone(), &Py::Int(1))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.PUT_INTO"); }
    s_PUT_INTO(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:put|puts|putting|place|places|placing|keep|keeps|keeping|store|stores|storing|add|adding|load|loading|pour|pouring|stick|sticking|poke|poking|stand|standing)\\b[^.;?!]{0,40}?\\b(?:in|into|inside|on|onto|under|behind|beside|near)\\s+(?:the\\s+|my\\s+|our\\s+|a\\s+|an\\s+|your\\s+|its\\s+)?(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CONTAINERS()) { let mut name: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.PLACEMENT"); }
    s_PLACEMENT(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:do|does|can|could|should|shall|would)\\s+(?:i|we|you|it)\\s+put\\b|\\bwhere\\s+to\\s+put\\b|\\bwhere\\s+i\\s+put\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.ONLY_A_MACHINE"); }
    s_ONLY_A_MACHINE(py_tuple(vec![py_str("rinse aid"), py_str("rinse-aid"), py_str("fabric softener"), py_str("descaler")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.A_MACHINE"); }
    s_A_MACHINE(stdlib_call("re", "compile", &[py_str("\\b(?:dishwasher|dish washer|washing machine|washer|washer dryer|washer-dryer|dishdrawer|dish drawer|tumble dryer|tumble drier|dryer)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data45() -> Py { py_tuple(vec![py_str("salt"), py_str("tablet"), py_str("tablets"), py_str("powder"), py_str("detergent"), py_str("rinse aid"), py_str("rinse-aid"), py_str("pod"), py_str("pods"), py_str("capsule"), py_str("capsules"), py_str("softener"), py_str("descaler")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.CONSUMABLE"); }
    s_CONSUMABLE(__data45());
    fn __data46() -> Py { py_tuple(vec![py_str("what"), py_str("take"), py_str("bring"), py_str("need"), py_str("allowed"), py_str("can i"), py_str("how"), py_str("load"), py_str("van"), py_str("go"), py_str("goes"), py_str("going"), py_str("collection"), py_str("bulky"), py_str("worth")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("local tip"), py_str("my local tip"), py_str("nearest tip"), py_str("closest tip"), py_str("tip near me"), py_str("council tip near me"), py_str("where is the council tip"), py_str("wheres the council tip"), py_str("where is my council tip"), py_str("the council tip near me"), py_str("the tip near me"), py_str("tip local to me"), py_str("where is the tip"), py_str("where is my tip"), py_str("where is the dump"), py_str("the dump near me"), py_str("local dump"), py_str("nearest dump"), py_str("local recycling centre"), py_str("local recycling center"), py_str("nearest recycling centre"), py_str("nearest recycling center"), py_str("recycling centre near me"), py_str("recycling center near me"), py_str("closest recycling centre"), py_str("household waste site near me"), py_str("household waste near me"), py_str("find a tip"), py_str("find the tip"), py_str("find a recycling centre"), py_str("find the recycling centre"), py_str("find the dump"), py_str("which tip"), py_str("which recycling centre"), py_str("my nearest tip"), py_str("dump near me")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("where"), py_str("how"), py_str("find"), py_str("nearest"), py_str("closest"), py_str("local"), py_str("near me"), py_str("near"), py_str("look up"), py_str("search"), py_str("which"), py_str("is my"), py_str("the")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("any tips"), py_str("tips for"), py_str("tip in"), py_str("tip on"), py_str("the waiter"), py_str("waitress"), py_str("restaurant"), py_str("pub"), py_str("percent"), py_str("gratuity"), py_str("work out a tip"), py_str("how do i tip"), py_str("how to tip"), py_str("how should i tip"), py_str("how much"), py_str("cost"), py_str("charge"), py_str("price"), py_str("open"), py_str("closing"), py_str("opening"), py_str("what time"), py_str("close"), py_str("council tax"), py_str("holiday"), py_str("bank holiday"), py_str("service charge"), py_str("tattoo"), py_str("barber"), py_str("hairdresser"), py_str("closed"), py_str("is closed"), py_str("is shut"), py_str("shut"), py_str("says"), py_str("said"), py_str("told me"), py_str("they say"), py_str("they said"), py_str("reckons"), py_str("apparently")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("cost"), py_str("costs"), py_str("charge"), py_str("charges"), py_str("charged"), py_str("free"), py_str("pay"), py_str("how much"), py_str("price"), py_str("fees"), py_str("fee"), py_str("rubble"), py_str("tyres"), py_str("tyre"), py_str("plasterboard"), py_str("soil"), py_str("hardcore"), py_str("diy"), py_str("van"), py_str("trailer"), py_str("permit")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("any tips"), py_str("tips for"), py_str("how much to tip"), py_str("should i tip"), py_str("the waiter"), py_str("restaurant"), py_str("percent"), py_str("gratuity"), py_str("service charge"), py_str("discretionary"), py_str("difference between"), py_str("same as a tip"), py_str("is a tip"), py_str("the bill"), py_str("on the bill"), py_str("tattoo"), py_str("hairdresser"), py_str("barber"), py_str("sofa"), py_str("settee"), py_str("couch"), py_str("armchair"), py_str("furniture"), py_str("wardrobe"), py_str("mattress"), py_str("bed"), py_str("table"), py_str("chairs"), py_str("bulky"), py_str("charity"), py_str("fridge"), py_str("freezer"), py_str("washing machine"), py_str("cooker"), py_str("tumble dryer"), py_str("dishwasher"), py_str("can i take"), py_str("can i bring"), py_str("can i put"), py_str("what can i take"), py_str("what can i bring"), py_str("allowed"), py_str("am i allowed"), py_str("what goes"), py_str("what do you take"), py_str("will they take"), py_str("do they take"), py_str("accept")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.rubbish.ENTRIES"); }
    s_ENTRIES(op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ITEMS()) { let mut item: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("dispose-"), &geti(&item.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![geti(&item.clone(), &Py::Int(1)), g_RID()])); __args.push(geti(&item.clone(), &Py::Int(2))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&geti(&item.clone(), &Py::Int(3)), &g_CURING_ASIDE()), &g_NOT_RUBBISH())), ("bias".to_string(), if py_truthy(&op_gt(&call_builtin("len", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) { geti(&item.clone(), &Py::Int(4)) } else { Py::Int(1) }), ("guard".to_string(), py_func("_disposal_guard", |args: &[Py]| _disposal_guard(args))), ("ask".to_string(), if py_truthy(&op_gt(&call_builtin("len", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) { geti(&item.clone(), &Py::Int(5)) } else { py_tuple(vec![]) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() }, &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dispose-which-bin")); __args.push(py_tuple(vec![g_BIN_WORDS(), g_BIN_ASK()])); __args.push(py_func("_a_bin_for_the_item", |args: &[Py]| _a_bin_for_the_item(args))); __kws.extend(vec![("avoid".to_string(), g_WHICH_BIN_AVOID()), ("bias".to_string(), op_neg(&Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("recycle-what-goes-where")); __args.push(py_tuple(vec![py_tuple(vec![py_str("recycle"), py_str("recycling"), py_str("recycled"), py_str("what goes in"), py_str("landfill")]), py_tuple(vec![py_str("what"), py_str("which"), py_str("how"), py_str("sort"), py_str("sorting"), py_str("goes"), py_str("go in"), py_str("in the"), py_str("can i"), py_str("do i")])])); __args.push(g_RECYCLE()); __kws.extend(vec![("avoid".to_string(), g_RECYCLE_AVOID()), ("bias".to_string(), op_neg(&Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bin-day")); __args.push(g_BIN_DAY_KEY()); __args.push(g_BIN_DAY()); __kws.extend(vec![("avoid".to_string(), g_BIN_DAY_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dispose-tip-visit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("the tip"), py_str("a tip"), py_str("to the tip"), py_str("tip run"), py_str("recycling centre"), py_str("household waste site"), py_str("the dump"), py_str("council tip"), py_str("council dump"), py_str("council recycling centre"), py_str("council recycling center")]), __data46()])); __args.push(g_TIP()); __kws.extend(vec![("avoid".to_string(), g_TIP_AVOID()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("find-your-tip")); __args.push(py_tuple(vec![__data47(), __data48()])); __args.push(py_str("Put your postcode into your council's recycling centre page - searching your postcode and \"recycling centre\" lands on it, and gov.uk links every council's site finder - and that page has the opening hours, what the site takes and whether you have to book. Take something that proves you live in the area, because most sites check it at the gate, and book a slot first if your council runs a booking system, which the busiest ones do at weekends.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data49())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tip-charges")); __args.push(py_tuple(vec![py_tuple(vec![py_str("the tip"), py_str("a tip"), py_str("to the tip"), py_str("tip run"), py_str("recycling centre"), py_str("household waste"), py_str("the dump"), py_str("council tip")]), __data50()])); __args.push(py_str("Household rubbish is free at a council tip; what they charge for is the DIY waste - rubble, soil, plasterboard, old tyres and sometimes garden waste - and the price is set by your council, so check its page for your site before you load the car. Vans and trailers usually need a permit booked in advance, and a few sites want proof you live in the area.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data51())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
}
pub fn _the_item_named(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spelling: Py = Py::None;
    let mut spellings: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The route for the item this line names, or None when it names none of them.");
    words = op_modulo(&py_str(" %s "), &mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]));
    let __iter1 = py_iter_or_raise(&g_ITEM_ROUTES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    spellings = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spellings.clone()) { let mut spelling: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &op_modulo(&py_str(" %s "), &spelling.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return said.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_bin_for_the_item(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The route for the item in the line, or the depends-on-the-item question when there is none.");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    said = _the_item_named(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return g_WHICH_BIN();
    Py::None
}
pub fn _not_a_cook_ask(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line is asking what to cook, so no item row answers.");
    return Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.dishes", "cook_ask", &[understood.clone()])));
    Py::None
}
pub fn _said(args: &[Py]) -> Py {
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
    let _ = py_str("The line as typed, then the reading's own text - what a word match here looks at.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _not_a_container(args: &[Py]) -> Py {
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
    let _ = py_str("False when the item named holds what the line is putting into it.");
    return Py::Bool(!py_truthy(&mcall(&g_PUT_INTO(), "search", &[mcall(&_said(&[understood.clone()]), "lower", &[])])));
    Py::None
}
pub fn _a_consumable_is_placed(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True only when the line asks where to put something that goes in a machine.");
    said = mcall(&_said(&[understood.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PLACEMENT(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ONLY_A_MACHINE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_A_MACHINE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CONSUMABLE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _disposal_guard(args: &[Py]) -> Py {
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
    let _ = py_str("False when this line is another row's question: a cook ask, a container or a placement.");
    return { let __b = _not_a_cook_ask(&[understood.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _not_a_container(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_consumable_is_placed(&[understood.clone()]))) } } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.rubbish".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "A_HOURS_ASK" => Some(g_A_HOURS_ASK()),
            "A_MACHINE" => Some(g_A_MACHINE()),
            "BIN_ASK" => Some(g_BIN_ASK()),
            "BIN_DAY" => Some(g_BIN_DAY()),
            "BIN_DAY_AVOID" => Some(g_BIN_DAY_AVOID()),
            "BIN_DAY_KEY" => Some(g_BIN_DAY_KEY()),
            "BIN_WORDS" => Some(g_BIN_WORDS()),
            "CONSUMABLE" => Some(g_CONSUMABLE()),
            "CURING_ASIDE" => Some(g_CURING_ASIDE()),
            "ENTRIES" => Some(g_ENTRIES()),
            "ITEMS" => Some(g_ITEMS()),
            "ITEM_ROUTES" => Some(g_ITEM_ROUTES()),
            "MEDICINE_DISPOSAL" => Some(g_MEDICINE_DISPOSAL()),
            "NOT_RUBBISH" => Some(g_NOT_RUBBISH()),
            "ONLY_A_MACHINE" => Some(g_ONLY_A_MACHINE()),
            "PLACEMENT" => Some(g_PLACEMENT()),
            "PUT_INTO" => Some(g_PUT_INTO()),
            "RECYCLE" => Some(g_RECYCLE()),
            "RECYCLE_AVOID" => Some(g_RECYCLE_AVOID()),
            "RID" => Some(g_RID()),
            "TIP" => Some(g_TIP()),
            "TIP_AVOID" => Some(g_TIP_AVOID()),
            "WHICH_BIN" => Some(g_WHICH_BIN()),
            "WHICH_BIN_AVOID" => Some(g_WHICH_BIN_AVOID()),
            "_CONTAINERS" => Some(g__CONTAINERS()),
            "_a_bin_for_the_item" => Some(py_func("_a_bin_for_the_item", |args: &[Py]| _a_bin_for_the_item(args))),
            "_a_consumable_is_placed" => Some(py_func("_a_consumable_is_placed", |args: &[Py]| _a_consumable_is_placed(args))),
            "_disposal_guard" => Some(py_func("_disposal_guard", |args: &[Py]| _disposal_guard(args))),
            "_not_a_container" => Some(py_func("_not_a_container", |args: &[Py]| _not_a_container(args))),
            "_not_a_cook_ask" => Some(py_func("_not_a_cook_ask", |args: &[Py]| _not_a_cook_ask(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_the_item_named" => Some(py_func("_the_item_named", |args: &[Py]| _the_item_named(args))),
            "cook_ask" => Some(g_cook_ask()),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "A_HOURS_ASK" => s_A_HOURS_ASK(v),
            "A_MACHINE" => s_A_MACHINE(v),
            "BIN_ASK" => s_BIN_ASK(v),
            "BIN_DAY" => s_BIN_DAY(v),
            "BIN_DAY_AVOID" => s_BIN_DAY_AVOID(v),
            "BIN_DAY_KEY" => s_BIN_DAY_KEY(v),
            "BIN_WORDS" => s_BIN_WORDS(v),
            "CONSUMABLE" => s_CONSUMABLE(v),
            "CURING_ASIDE" => s_CURING_ASIDE(v),
            "ENTRIES" => s_ENTRIES(v),
            "ITEMS" => s_ITEMS(v),
            "ITEM_ROUTES" => s_ITEM_ROUTES(v),
            "MEDICINE_DISPOSAL" => s_MEDICINE_DISPOSAL(v),
            "NOT_RUBBISH" => s_NOT_RUBBISH(v),
            "ONLY_A_MACHINE" => s_ONLY_A_MACHINE(v),
            "PLACEMENT" => s_PLACEMENT(v),
            "PUT_INTO" => s_PUT_INTO(v),
            "RECYCLE" => s_RECYCLE(v),
            "RECYCLE_AVOID" => s_RECYCLE_AVOID(v),
            "RID" => s_RID(v),
            "TIP" => s_TIP(v),
            "TIP_AVOID" => s_TIP_AVOID(v),
            "WHICH_BIN" => s_WHICH_BIN(v),
            "WHICH_BIN_AVOID" => s_WHICH_BIN_AVOID(v),
            "_CONTAINERS" => s__CONTAINERS(v),
            "cook_ask" => s_cook_ask(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.rubbish", module_obj);
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_the_item_named", py_func("_the_item_named", |args: &[Py]| _the_item_named(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_a_bin_for_the_item", py_func("_a_bin_for_the_item", |args: &[Py]| _a_bin_for_the_item(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_not_a_cook_ask", py_func("_not_a_cook_ask", |args: &[Py]| _not_a_cook_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_not_a_container", py_func("_not_a_container", |args: &[Py]| _not_a_container(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_a_consumable_is_placed", py_func("_a_consumable_is_placed", |args: &[Py]| _a_consumable_is_placed(args)));
    pyrt::xrefs::reg_fn("knowledge.rubbish", "_disposal_guard", py_func("_disposal_guard", |args: &[Py]| _disposal_guard(args)));
    pyrt::xrefs::reg_access("knowledge.rubbish", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.rubbish", "A_HOURS_ASK", g_A_HOURS_ASK, s_A_HOURS_ASK);
    pyrt::xrefs::reg_access("knowledge.rubbish", "A_MACHINE", g_A_MACHINE, s_A_MACHINE);
    pyrt::xrefs::reg_access("knowledge.rubbish", "BIN_ASK", g_BIN_ASK, s_BIN_ASK);
    pyrt::xrefs::reg_access("knowledge.rubbish", "BIN_DAY", g_BIN_DAY, s_BIN_DAY);
    pyrt::xrefs::reg_access("knowledge.rubbish", "BIN_DAY_AVOID", g_BIN_DAY_AVOID, s_BIN_DAY_AVOID);
    pyrt::xrefs::reg_access("knowledge.rubbish", "BIN_DAY_KEY", g_BIN_DAY_KEY, s_BIN_DAY_KEY);
    pyrt::xrefs::reg_access("knowledge.rubbish", "BIN_WORDS", g_BIN_WORDS, s_BIN_WORDS);
    pyrt::xrefs::reg_access("knowledge.rubbish", "CONSUMABLE", g_CONSUMABLE, s_CONSUMABLE);
    pyrt::xrefs::reg_access("knowledge.rubbish", "CURING_ASIDE", g_CURING_ASIDE, s_CURING_ASIDE);
    pyrt::xrefs::reg_access("knowledge.rubbish", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.rubbish", "ITEMS", g_ITEMS, s_ITEMS);
    pyrt::xrefs::reg_access("knowledge.rubbish", "ITEM_ROUTES", g_ITEM_ROUTES, s_ITEM_ROUTES);
    pyrt::xrefs::reg_access("knowledge.rubbish", "MEDICINE_DISPOSAL", g_MEDICINE_DISPOSAL, s_MEDICINE_DISPOSAL);
    pyrt::xrefs::reg_access("knowledge.rubbish", "NOT_RUBBISH", g_NOT_RUBBISH, s_NOT_RUBBISH);
    pyrt::xrefs::reg_access("knowledge.rubbish", "ONLY_A_MACHINE", g_ONLY_A_MACHINE, s_ONLY_A_MACHINE);
    pyrt::xrefs::reg_access("knowledge.rubbish", "PLACEMENT", g_PLACEMENT, s_PLACEMENT);
    pyrt::xrefs::reg_access("knowledge.rubbish", "PUT_INTO", g_PUT_INTO, s_PUT_INTO);
    pyrt::xrefs::reg_access("knowledge.rubbish", "RECYCLE", g_RECYCLE, s_RECYCLE);
    pyrt::xrefs::reg_access("knowledge.rubbish", "RECYCLE_AVOID", g_RECYCLE_AVOID, s_RECYCLE_AVOID);
    pyrt::xrefs::reg_access("knowledge.rubbish", "RID", g_RID, s_RID);
    pyrt::xrefs::reg_access("knowledge.rubbish", "TIP", g_TIP, s_TIP);
    pyrt::xrefs::reg_access("knowledge.rubbish", "TIP_AVOID", g_TIP_AVOID, s_TIP_AVOID);
    pyrt::xrefs::reg_access("knowledge.rubbish", "WHICH_BIN", g_WHICH_BIN, s_WHICH_BIN);
    pyrt::xrefs::reg_access("knowledge.rubbish", "WHICH_BIN_AVOID", g_WHICH_BIN_AVOID, s_WHICH_BIN_AVOID);
    pyrt::xrefs::reg_access("knowledge.rubbish", "_CONTAINERS", g__CONTAINERS, s__CONTAINERS);
    pyrt::xrefs::reg_access("knowledge.rubbish", "cook_ask", g_cook_ask, s_cook_ask);
    pyrt::xrefs::reg_access("knowledge.rubbish", "entry", g_entry, s_entry);
}

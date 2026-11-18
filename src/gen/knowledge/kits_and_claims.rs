#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Three questions people ask that the neighbouring cards could not answer, as data.\n\nEach row here answers a line that was reaching a card about something next door:\n\n- a car the person's own insurer has written off is a settlement question. `write-off-category`\n  explains Cat S and Cat N, the dealer row in `consumer.py` is about buying one, and\n  `repair-or-scrap` in `cars.py` is the decision to mend or sell, so \"my car was written off, do\n  i get a payout\" was answered with the dealer's duty and \"how much will my insurer pay\" with a\n  savings ladder.\n- \"what do i need for a home office\" is a shopping list, and `work-from-home-council` answered it\n  with council tax.\n- \"what do i need for a day case\" is a bag, and the 111 line answered it while \"what should i\n  take to hospital for an operation\" got the gift card.\n\nThey live in one module rather than three because each is a single row and a row belongs beside\nthe rows it shares words with, which here is nothing: the three subjects have no keys in common.\n");
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("car-write-off-payout")); __args.push(py_tuple(vec![py_tuple(vec![py_str("written off"), py_str("write off"), py_str("wrote it off"), py_str("wrote off"), py_str("total loss"), py_str("write-off"), py_str("cat s"), py_str("cat n"), py_str("wrote my car off"), py_str("wrote the car off"), py_str("wrote my van off"), py_str("write my car off"), py_str("writing my car off"), py_str("wrote mine off"), py_str("write it off")]), py_tuple(vec![py_str("payout"), py_str("pay out"), py_str("paid out"), py_str("how much"), py_str("settlement"), py_str("settle"), py_str("offer"), py_str("do i get"), py_str("get the money"), py_str("market value"), py_str("keep the car"), py_str("buy it back"), py_str("salvage"), py_str("argue"), py_str("appeal"), py_str("disagree"), py_str("too low"), py_str("process"), py_str("what now"), py_str("what happens"), py_str("accept")])])); __args.push(py_str("The insurer settles at the car's market value just before the crash, not what you paid for it: check their offer against three or four adverts for the same age, mileage and trim, and send them the links if it is low. Repairs can be offered instead when they are cheaper and safe. Ask what the salvage is worth if you want to keep the car - you buy it back, and it usually becomes Cat S or Cat N.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("dealer"), py_str("dealership"), py_str("forecourt"), py_str("sold me"), py_str("sold as seen"), py_str("what does"), py_str("what is cat"), py_str("difference between"), py_str("mean")])), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("car"), py_str("van"), py_str("vehicle"), py_str("insurer"), py_str("insurance"), py_str("payout"), py_str("settle"), py_str("offer"), py_str("salvage"), py_str("written off"), py_str("write off"), py_str("total loss")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("home-office-desk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("home office"), py_str("office at home"), py_str("desk setup"), py_str("desk set up"), py_str("wfh"), py_str("work from home"), py_str("working from home"), py_str("homeworking")]), py_tuple(vec![py_str("need"), py_str("buy"), py_str("get"), py_str("kit"), py_str("what do i need"), py_str("what should i"), py_str("essentials")])])); __args.push(py_str("Get the chair right first and the rest is cheap: an adjustable one so your feet are flat and the arms sit at elbow height, the screen top at eye level, a stack of books or a riser under a laptop, and a lamp beside your writing hand rather than behind you. A separate keyboard and mouse are the other fix worth paying for.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("council tax"), py_str("business rates"), py_str("tax"), py_str("expenses"), py_str("allowance"), py_str("claim"), py_str("employer"), py_str("extension"), py_str("convert"), py_str("conversion"), py_str("loft"), py_str("build"), py_str("building"), py_str("paint"), py_str("decorate"), py_str("rent"), py_str("landlord"), py_str("homework"), py_str("school"), py_str("teacher"), py_str("year old"), py_str("kids"), py_str("children"), py_str("child"), py_str("revision"), py_str("exam"), py_str("study"), py_str("toddler")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hospital-bag-for-an-operation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hospital"), py_str("operation"), py_str("surgery"), py_str("day case"), py_str("day surgery"), py_str("admitted"), py_str("admission"), py_str("procedure"), py_str("overnight stay")]), py_tuple(vec![py_str("take"), py_str("bring"), py_str("pack"), py_str("bag"), py_str("need"), py_str("what to take"), py_str("what should i take"), py_str("what do i bring"), py_str("what to bring"), py_str("what do i need"), py_str("come in with")])])); __args.push(py_str("Take your medicines in their boxes, the admission letter, phone and charger, and loose clothes you can get on one-handed; for a day case add slippers with a back and a named bag for your clothes. Leave valuables at home, and ask the ward what they provide before you pack towels and toiletries.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hospital bag"), py_str("maternity"), py_str("labour"), py_str("birth"), py_str("baby"), py_str("pregnant"), py_str("pregnancy"), py_str("visit"), py_str("visiting"), py_str("gift"), py_str("present"), py_str("flowers"), py_str("should i go"), py_str("do i need to go"), py_str("a and e"), py_str("casualty"), py_str("111"), py_str("999"), py_str("ambulance")])), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("take"), py_str("bring"), py_str("pack"), py_str("bag"), py_str("need"), py_str("hospital"), py_str("operation"), py_str("surgery")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.kits_and_claims".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ENTRIES" => Some(g_ENTRIES()),
            _ => None,
        }),
        set: None,
    }))
}

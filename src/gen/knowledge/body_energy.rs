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


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BODYS_ENERGY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IN_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_HOUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__A_BODYS_ENERGY() -> Py { ensure_init(); _A_BODYS_ENERGY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BODYS_ENERGY(v: Py) { ensure_init(); if let Ok(mut w) = _A_BODYS_ENERGY.write() { *w = v; } }
pub fn g__IN_THE_DAY() -> Py { ensure_init(); _IN_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IN_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = _IN_THE_DAY.write() { *w = v; } }
pub fn g__THE_HOUSE() -> Py { ensure_init(); _THE_HOUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_HOUSE(v: Py) { ensure_init(); if let Ok(mut w) = _THE_HOUSE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The energy a person has, which is not the energy they buy.\n\nThe answers lane owns this domain. Everything here is data: the row is keyed by meaning (the\ndip after lunch, and the tiredness that comes with it) and phrased the way a sensible friend\nwould answer.\n\nWhy the row is here rather than in `sleep` or `home`: the tariff answer (`cheapest-hours`) keys\nthe word `energy` beside the hours of the day, because on a two-rate tariff the night is\ncheaper and people ask when to run a machine. So \"how do i keep my energy up in the afternoon\"\nand \"i have no energy in the afternoons\" were answered with Economy 7 and a tumble dryer - the\nsingle most confident wrong answer in the knowledge base. `sleep` has `afternoon-slump` and it\nsays the right thing, but no line carrying the word `energy` could reach it while the tariff\nrow scored on the same afternoon.\n\nSo this row takes the body's energy as its own subject, and the guard keeps the tariff, the\nbill, the meter and the appliances out of it.\n");
    s_ANSWERS(py_tuple(vec![py_str("sleep"), py_str("health")]));
    s__A_BODYS_ENERGY(py_tuple(vec![py_str("no energy"), py_str("energy up"), py_str("keep my energy"), py_str("keep energy"), py_str("my energy"), py_str("energy levels"), py_str("energy level"), py_str("more energy"), py_str("less energy"), py_str("low energy"), py_str("lack of energy"), py_str("energy crash"), py_str("energy dip"), py_str("energy slump"), py_str("energy drops"), py_str("energy goes"), py_str("energy down"), py_str("energy left"), py_str("energy back"), py_str("energy all day")]));
    s__IN_THE_DAY(py_tuple(vec![py_str("afternoon"), py_str("afternoons"), py_str("after lunch"), py_str("after my lunch"), py_str("after eating lunch"), py_str("midday"), py_str("mid-afternoon"), py_str("middle of the day"), py_str("in the day"), py_str("daytime"), py_str("after work"), py_str("2pm"), py_str("3pm"), py_str("4pm"), py_str("3 o'clock"), py_str("2 o'clock"), py_str("4 o'clock"), py_str("post-lunch"), py_str("lunchtime"), py_str("every afternoon"), py_str("most afternoons"), py_str("all afternoon")]));
    s__THE_HOUSE(stdlib_call("re", "compile", &[py_str("\\b(?:tariff|tariffs|economy\\s*7|off-?peak|peak|kwh|kilowatt|meter|metre|reading|bill|bills|billed|standing charge|direct debit|supplier|switch|tariffs|electric|electricity|gas|heating|boiler|radiator|immersion|washing machine|washer|tumble dryer|dryer|dishwasher|immersion|charge|charging|battery|car|van|cheapest|cheaper|cheap|price|prices|cost|costs|unit rate)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-afternoon-dip")); __args.push(py_tuple(vec![g__A_BODYS_ENERGY(), g__IN_THE_DAY()])); __args.push(py_str("That dip after lunch is your body clock, not laziness or a lack of willpower, and it passes. Eat a lunch with protein and fibre rather than a big plate of white carbs, then get ten minutes of daylight and a short walk at the low point instead of another coffee. Drink water, keep the caffeine to before 2pm so it does not cost you tonight, and if you can, take a 15-minute nap rather than a long one.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__the_energy_is_the_bodys())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_energy_is_the_bodys(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the energy in the line is the person's, not the house's.\n\n    True means this row may answer. A line about a tariff, a bill, a meter, an appliance or a\n    cheaper hour stands down here and keeps the answer it had, and so does a line about charging\n    a car or a phone.\n    ");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    words = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    text = { let __b = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { words.clone() } };
    return Py::Bool(!py_truthy(&mcall(&g__THE_HOUSE(), "search", &[text.clone()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.body_energy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_BODYS_ENERGY" => Some(g__A_BODYS_ENERGY()),
            "_IN_THE_DAY" => Some(g__IN_THE_DAY()),
            "_THE_HOUSE" => Some(g__THE_HOUSE()),
            "_the_energy_is_the_bodys" => Some(py_func("_the_energy_is_the_bodys", |args: &[Py]| _the_energy_is_the_bodys(args))),
            _ => None,
        }),
        set: None,
    }))
}

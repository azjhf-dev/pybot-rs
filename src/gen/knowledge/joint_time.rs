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
pub static _BOTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BY_WEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MEATS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NUMBERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PORK_IN_SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SLOW_COOKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SLOW_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WEIGHTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g__BOTH() -> Py { ensure_init(); _BOTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BOTH(v: Py) { ensure_init(); if let Ok(mut w) = _BOTH.write() { *w = v; } }
pub fn g__BY_WEIGHT() -> Py { ensure_init(); _BY_WEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BY_WEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _BY_WEIGHT.write() { *w = v; } }
pub fn g__MEATS() -> Py { ensure_init(); _MEATS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MEATS(v: Py) { ensure_init(); if let Ok(mut w) = _MEATS.write() { *w = v; } }
pub fn g__NUMBERS() -> Py { ensure_init(); _NUMBERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NUMBERS(v: Py) { ensure_init(); if let Ok(mut w) = _NUMBERS.write() { *w = v; } }
pub fn g__PORK_IN_SHORT() -> Py { ensure_init(); _PORK_IN_SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PORK_IN_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = _PORK_IN_SHORT.write() { *w = v; } }
pub fn g__SLOW_COOKER() -> Py { ensure_init(); _SLOW_COOKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SLOW_COOKER(v: Py) { ensure_init(); if let Ok(mut w) = _SLOW_COOKER.write() { *w = v; } }
pub fn g__SLOW_WORDS() -> Py { ensure_init(); _SLOW_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SLOW_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _SLOW_WORDS.write() { *w = v; } }
pub fn g__WEIGHTS() -> Py { ensure_init(); _WEIGHTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WEIGHTS(v: Py) { ensure_init(); if let Ok(mut w) = _WEIGHTS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A joint asked about by its weight: the roast timetable, not the diet card.\n\n`bot/knowledge/cooking.py`'s `roast-joint` row wants the joint named, either as a cut (\"joint of\nbeef\", \"leg of lamb\") or as the pair (\"roast joint\"). \"How long does a 2 kilo joint take\" names\nneither: it has a weight, the word \"joint\" and a time. So it matched no cooking row at all, and\nfell to the diet advice, whose key carries a weight word and the word \"how\".\n\nThe row here is that one shape - the bare \"joint\" with a weight - and nothing else. A line that\nnames its meat (\"how long do i cook a joint of beef for\") still goes to `roast-joint`, and the\navoid list keeps a knee out of it, because an aching joint is a body before it is a dinner.\n\nThe timetable itself is not copied: `_roast_joint` is asked for the answer it would give a named\njoint, so the two rows cannot drift apart. This is its own module rather than an edit to\ncooking.py because cooking.py is at the 1 MB file wall.\n\nThe answers lane owns this file; pass a line you do not answer rather than guessing.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__BOTH(add(&add(&crate::gen::knowledge::cooking::g_JOINT_BEEF(), &py_str(" ")), &crate::gen::knowledge::cooking::g_JOINT_PORK()));
    s__PORK_IN_SHORT(py_str("Pork is 30 minutes a 500g at 180C (160C fan) and 70C in the middle."));
    s__SLOW_COOKER(py_str("A joint up to 2 kilos takes 6 to 8 hours on low, or 4 to 5 on high: stand it on a trivet or a bed of onion and carrot with 150 to 250ml of stock, and keep the lid on. Pork is done at 70C inside; a joint you want to pull wants 90C, which is another few hours on low. No crackling in a slow cooker - finish that under a hot grill."));
    s__WEIGHTS(py_tuple(vec![py_str("kilo"), py_str("kilos"), py_str("kg"), py_str("kgs"), py_str("kilogram"), py_str("kilograms"), py_str("pound"), py_str("pounds")]));
    s__MEATS(py_tuple(vec![py_str("pork"), py_str("beef"), py_str("lamb"), py_str("veal"), py_str("mutton"), py_str("ham")]));
    s__NUMBERS(py_tuple(vec![py_str("1"), py_str("2"), py_str("3"), py_str("4"), py_str("5"), py_str("6"), py_str("7"), py_str("8"), py_str("9")]));
    s__BY_WEIGHT(add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WEIGHTS()) { let mut weight: Py = __item.clone(); for __item in py_iter_or_raise(&g__MEATS()) { let mut meat: Py = __item.clone(); __out.push(add(&add(&weight.clone(), &py_str(" of ")), &meat.clone())); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NUMBERS()) { let mut number: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("kg"), py_str("kgs"), py_str("lb"), py_str("lbs"), py_str("G")])) { let mut unit: Py = __item.clone(); for __item in py_iter_or_raise(&g__MEATS()) { let mut meat: Py = __item.clone(); __out.push(add(&add(&add(&number.clone(), &unit.clone()), &py_str(" of ")), &meat.clone())); } } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s__SLOW_WORDS(py_tuple(vec![py_str("slow cooker"), py_str("slow cook"), py_str("slowcooker"), py_str("slow-cooker"), py_str("crockpot"), py_str("crock pot"), py_str("crock-pot")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-time")); __args.push(py_tuple(vec![add(&py_tuple(vec![py_str("joint"), py_str("gammon"), py_str("ham")]), &g__BY_WEIGHT()), py_tuple(vec![py_str("how long"), py_str("time"), py_str("roast"), py_str("roasting"), py_str("oven"), py_str("temperature"), py_str("cook"), py_str("cooking"), py_str("minutes"), py_str("done"), py_str("kilos"), py_str("kilo"), py_str("kg"), py_str("kgs"), py_str("pounds"), py_str("pound"), py_str("lbs"), py_str("slow cooker"), py_str("slow cook"), py_str("slowcooker"), py_str("slow-cooker"), py_str("crockpot"), py_str("crock pot"), py_str("crock-pot")])])); __args.push(g__joint_time()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("joint of"), py_str("roast joint"), py_str("leg of"), py_str("shoulder of"), py_str("beef joint"), py_str("pork joint"), py_str("chicken"), py_str("turkey"), py_str("duck"), py_str("goose"), py_str("bird"), py_str("mince"), py_str("my joint"), py_str("joints hurt"), py_str("joint pain"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("shoulder hurts"), py_str("shoulder"), py_str("shoulders"), py_str("brisket"), py_str("briskets"), py_str("slow roast"), py_str("slowly"), py_str("low and slow"), py_str("overnight"), py_str("pulled"), py_str("calorie"), py_str("calories"), py_str("kcal"), py_str("how many calories"), py_str("how much is"), py_str("how much are"), py_str("how much does a kilo"), py_str("price"), py_str("prices"), py_str("cost"), py_str("costs"), py_str("per kilo"), py_str("a kilo cost"), py_str("cheapest"), py_str("cheaper"), py_str("offer"), py_str("on offer"), py_str("supermarket"), py_str("butcher charge"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing"), py_str("frozen"), py_str("freeze"), py_str("freezer"), py_str("from frozen"), py_str("rest"), py_str("resting"), py_str("rests"), py_str("rested"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("fridge"), py_str("ice"), py_str("icing"), py_str("heal"), py_str("healing"), py_str("heals"), py_str("recover"), py_str("recovery")])), ("guard".to_string(), g__not_a_gammon_in_the_oven())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _joint_by_weight(args: &[Py]) -> Py {
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
    let _ = py_str("The timetables come from cooking.py, so the two rows cannot drift apart.");
    said = crate::gen::knowledge::cooking::_roast_joint(&[memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(py_eq(&said.clone(), &g__BOTH()))) {
    return add(&add(&crate::gen::knowledge::cooking::g_JOINT_BEEF(), &py_str(" ")), &g__PORK_IN_SHORT());
    }
    return said.clone();
    Py::None
}
pub fn _not_a_gammon_in_the_oven(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &py_str("gammon")))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SLOW_WORDS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _joint_time(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("One row for every joint time question: the slow cooker has its own hours, the oven its own.");
    words = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SLOW_WORDS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__SLOW_COOKER();
    }
    return _joint_by_weight(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.joint_time".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_BOTH" => Some(g__BOTH()),
            "_BY_WEIGHT" => Some(g__BY_WEIGHT()),
            "_MEATS" => Some(g__MEATS()),
            "_NUMBERS" => Some(g__NUMBERS()),
            "_PORK_IN_SHORT" => Some(g__PORK_IN_SHORT()),
            "_SLOW_COOKER" => Some(g__SLOW_COOKER()),
            "_SLOW_WORDS" => Some(g__SLOW_WORDS()),
            "_WEIGHTS" => Some(g__WEIGHTS()),
            "_joint_by_weight" => Some(py_func("_joint_by_weight", |args: &[Py]| _joint_by_weight(args))),
            "_joint_time" => Some(py_func("_joint_time", |args: &[Py]| _joint_time(args))),
            "_not_a_gammon_in_the_oven" => Some(py_func("_not_a_gammon_in_the_oven", |args: &[Py]| _not_a_gammon_in_the_oven(args))),
            _ => None,
        }),
        set: None,
    }))
}

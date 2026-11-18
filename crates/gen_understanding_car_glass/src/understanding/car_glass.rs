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


thread_local! { static G_A_WINDOW_ON_A_CAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_DID_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_GLASS_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.car_glass: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_WINDOW_ON_A_CAR() -> Py { ensure_init(); G_A_WINDOW_ON_A_CAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WINDOW_ON_A_CAR(v: Py) { ensure_init(); G_A_WINDOW_ON_A_CAR.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAULT() -> Py { ensure_init(); G__A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAULT(v: Py) { ensure_init(); G__A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_DID_IT() -> Py { ensure_init(); G__SOMEBODY_DID_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_DID_IT(v: Py) { ensure_init(); G__SOMEBODY_DID_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_GLASS_ITSELF() -> Py { ensure_init(); G__THE_GLASS_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_GLASS_ITSELF(v: Py) { ensure_init(); G__THE_GLASS_ITSELF.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The glass of a car, which is a question about light and weather, not about damage.\n\nThe claim row in `bot/knowledge/cars.py` keys the window itself - \"car window\", \"my car window\" -\nand the word *car* inside that phrase also counts for the second half of the row's trigger, so one\nphrase proved both halves of the match. Every question about a car window landed on the claim\ncard:\n\n    can you tan through a car window       -> \"Nobody was hurt, so this is a report and a claim\"\n    why do my car windows mist up          -> the same\n    how do i stop my car window icing over -> the same\n\nThree rows already answer those lines - the glass, the demister and the scraper - and none of them\ngot the line. So the row is held off by a reading rather than by a longer key list: a line that\nnames a car window and says nothing about damage is somebody else's answer.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.car_glass.A_WINDOW_ON_A_CAR"); }
    s_A_WINDOW_ON_A_CAR(stdlib_call("re", "compile", &[py_str("\\bcar windows?\\b|\\bwindows? (?:on|of|in) (?:the|my|our|a) car\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("keyed"), py_str("keying"), py_str("smashed"), py_str("smash"), py_str("shattered"), py_str("knocked off"), py_str("knocked out"), py_str("broke"), py_str("broken into"), py_str("break in"), py_str("steal"), py_str("stolen"), py_str("nicked"), py_str("damaged"), py_str("vandalised"), py_str("vandalized"), py_str("reversed into"), py_str("took it"), py_str("taken"), py_str("gone"), py_str("missing"), py_str("someone"), py_str("somebody"), py_str("thief"), py_str("thieves"), py_str("burglary"), py_str("attempted")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.car_glass._SOMEBODY_DID_IT"); }
    s__SOMEBODY_DID_IT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("tan"), py_str("tanned"), py_str("tanning"), py_str("suntan"), py_str("sun tan"), py_str(" uv"), py_str(" uva"), py_str(" uvb"), py_str("sunlight"), py_str("sun "), py_str("sunny"), py_str("tint"), py_str("tints"), py_str("tinted"), py_str("tinting"), py_str("tint film"), py_str("clean"), py_str("cleaning"), py_str("cleaner"), py_str("wash"), py_str("washing"), py_str("dirty"), py_str("smear"), py_str("smears"), py_str("smudged"), py_str("squeak"), py_str("squeaky"), py_str("mist"), py_str("misted"), py_str("misting"), py_str("misty"), py_str("steam"), py_str("steamed"), py_str("steaming"), py_str("steamy"), py_str("fog"), py_str("foggy"), py_str("fogged"), py_str("condensation"), py_str("demist"), py_str("demister"), py_str("defrost"), py_str("frost"), py_str("frosted"), py_str("frosty"), py_str("frozen"), py_str("ice"), py_str("iced"), py_str("icing"), py_str("wiper"), py_str("wipers"), py_str("reflection"), py_str("glare"), py_str("blind spot")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.car_glass._THE_GLASS_ITSELF"); }
    s__THE_GLASS_ITSELF(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("crack"), py_str("cracked"), py_str("cracks"), py_str("chip"), py_str("chipped"), py_str("chips"), py_str("stone chip"), py_str("star chip"), py_str("wont open"), py_str("wont close"), py_str("wont go"), py_str("wont wind"), py_str("wont shut"), py_str("doesnt open"), py_str("doesnt close"), py_str("does not open"), py_str("does not close"), py_str("stuck"), py_str("sticking"), py_str("jammed"), py_str("jam"), py_str("off its runner"), py_str("water"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("letting in"), py_str("coming in"), py_str("seal"), py_str("seals"), py_str("rubber"), py_str("regulator"), py_str("switch"), py_str("handle"), py_str("whistle"), py_str("whistling"), py_str("noise"), py_str("rattles"), py_str("rattling"), py_str("squeaks"), py_str("squeaking"), py_str("repair"), py_str("repairs"), py_str("replace"), py_str("replacement"), py_str("how much"), py_str("cost"), py_str("price"), py_str("cheap"), py_str("cheapest"), py_str("fix")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.car_glass._A_FAULT"); }
    s__A_FAULT(op_add(&g__THE_GLASS_ITSELF(), &__data3()));
}
pub fn _names_one_of(args: &[Py]) -> Py {
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
    let mut parts: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut part: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&op_modulo(&py_str(" %s"), &said.clone()), &op_modulo(&py_str(" %s"), &part.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_window_on_a_car_is_not_a_damage(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("False when the line is about the car's glass rather than about damage to it.\n\n    The claim row keys the window itself, so every question about a car window landed on it. Both\n    halves of that proof are in the reading, so the reading is where it is undone: a line that\n    names a car window and asks about the glass or a fault of it, with nobody in the line doing\n    anything, is somebody else's answer. A line that says somebody smashed, keyed or broke into\n    the car keeps the claim card, because the person doing it is in the line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![mcall(&understood.clone(), "get", &[py_str("text")]), mcall(&understood.clone(), "get", &[py_str("clean")]), mcall(&understood.clone(), "get", &[py_str("raw")]), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_WINDOW_ON_A_CAR(), "search", &[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_names_one_of(&[said.clone(), g__SOMEBODY_DID_IT()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&_names_one_of(&[said.clone(), g__A_FAULT()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.car_glass".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_WINDOW_ON_A_CAR" => Some(g_A_WINDOW_ON_A_CAR()),
            "_A_FAULT" => Some(g__A_FAULT()),
            "_SOMEBODY_DID_IT" => Some(g__SOMEBODY_DID_IT()),
            "_THE_GLASS_ITSELF" => Some(g__THE_GLASS_ITSELF()),
            "_names_one_of" => Some(py_func("_names_one_of", |args: &[Py]| _names_one_of(args))),
            "a_window_on_a_car_is_not_a_damage" => Some(py_func("a_window_on_a_car_is_not_a_damage", |args: &[Py]| a_window_on_a_car_is_not_a_damage(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_WINDOW_ON_A_CAR" => s_A_WINDOW_ON_A_CAR(v),
            "_A_FAULT" => s__A_FAULT(v),
            "_SOMEBODY_DID_IT" => s__SOMEBODY_DID_IT(v),
            "_THE_GLASS_ITSELF" => s__THE_GLASS_ITSELF(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.car_glass", module_obj);
    pyrt::xrefs::reg_fn("understanding.car_glass", "_names_one_of", py_func("_names_one_of", |args: &[Py]| _names_one_of(args)));
    pyrt::xrefs::reg_fn("understanding.car_glass", "a_window_on_a_car_is_not_a_damage", py_func("a_window_on_a_car_is_not_a_damage", |args: &[Py]| a_window_on_a_car_is_not_a_damage(args)));
    pyrt::xrefs::reg_access("understanding.car_glass", "A_WINDOW_ON_A_CAR", g_A_WINDOW_ON_A_CAR, s_A_WINDOW_ON_A_CAR);
    pyrt::xrefs::reg_access("understanding.car_glass", "_A_FAULT", g__A_FAULT, s__A_FAULT);
    pyrt::xrefs::reg_access("understanding.car_glass", "_SOMEBODY_DID_IT", g__SOMEBODY_DID_IT, s__SOMEBODY_DID_IT);
    pyrt::xrefs::reg_access("understanding.car_glass", "_THE_GLASS_ITSELF", g__THE_GLASS_ITSELF, s__THE_GLASS_ITSELF);
}

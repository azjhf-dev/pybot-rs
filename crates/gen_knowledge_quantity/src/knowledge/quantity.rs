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


thread_local! { static G_AFTER_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.quantity: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AFTER_A_FIGURE() -> Py { ensure_init(); G_AFTER_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_FIGURE(v: Py) { ensure_init(); G_AFTER_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS() -> Py { ensure_init(); G_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); G_UNITS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A figure with a unit on it is an amount, not a code.\n\nA degree class is written with a dot or a colon in it, and a phone writes \"2:1\" as \"2 1\", so the\nknowledge base has to hold \"2.1\" and \"2.2\" as words of their own. A joint of meat weighing 2.2\nkilos is then the same two characters: \"the lamb is 2.2 kilos not 2\" was answered with what a\ndegree passes at, in the middle of a slow roast.\n\nThe understanding engine already reads a number and the unit it is counted in - \"2.2 kilos\" comes\nback as a number of 2.2 with a unit of kg - so a card keyed on the digits alone can ask here and\nstand down when a unit is on them.\n");
    s_UNITS(call_builtin("frozenset", &[mcall(&py_str("\nkg kgs kilo kilos g gram grams gm gms lb lbs pound pounds st stone stones\nl litre litres liter liters ml cl cm mm m metre metres meter meters km\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:[.,]\\d+)?\\s*(?:kg|kgs|kilo|kilos|g|gram|grams|gm|gms|lb|lbs|pound|pounds|st|stone|stones|l|litre|litres|ml|cl|cm|mm|metre|metres|km)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_figure_with_a_unit(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut body: Py = Py::None;
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("True when the line counts something in a unit, so a figure is an amount, not a code.");
    numbers = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("numbers")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    let __iter1 = py_iter_or_raise(&numbers.clone());
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    unit = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&number.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_UNITS(), &unit.clone()))) {
    return Py::Bool(true);
    }
    }
    body = { let __b = text.clone(); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)) } };
    return call_builtin("bool", &[mcall(&g_AFTER_A_FIGURE(), "search", &[body.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.quantity".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_A_FIGURE" => Some(g_AFTER_A_FIGURE()),
            "UNITS" => Some(g_UNITS()),
            "a_figure_with_a_unit" => Some(py_func("a_figure_with_a_unit", |args: &[Py]| a_figure_with_a_unit(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFTER_A_FIGURE" => s_AFTER_A_FIGURE(v),
            "UNITS" => s_UNITS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.quantity", module_obj);
    pyrt::xrefs::reg_fn("knowledge.quantity", "a_figure_with_a_unit", py_func("a_figure_with_a_unit", |args: &[Py]| a_figure_with_a_unit(args)));
    pyrt::xrefs::reg_access("knowledge.quantity", "AFTER_A_FIGURE", g_AFTER_A_FIGURE, s_AFTER_A_FIGURE);
    pyrt::xrefs::reg_access("knowledge.quantity", "UNITS", g_UNITS, s_UNITS);
}

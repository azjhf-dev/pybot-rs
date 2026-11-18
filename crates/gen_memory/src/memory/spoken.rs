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


thread_local! { static G__A_COUNT_AND_A_HALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SPOKEN_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SPOKEN_HALF_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TENS_AND_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORDED_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINUTES_CEILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONE_TO_NINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPOKEN_FRACTION_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPOKEN_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPOKEN_TENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPOKEN_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_spoken_as_numbers: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__A_COUNT_AND_A_HALF() -> Py { ensure_init(); G__A_COUNT_AND_A_HALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT_AND_A_HALF(v: Py) { ensure_init(); G__A_COUNT_AND_A_HALF.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE_UNIT() -> Py { ensure_init(); G__A_RATE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE_UNIT(v: Py) { ensure_init(); G__A_RATE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SPOKEN_FRACTION() -> Py { ensure_init(); G__A_SPOKEN_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SPOKEN_FRACTION(v: Py) { ensure_init(); G__A_SPOKEN_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SPOKEN_HALF_HOUR() -> Py { ensure_init(); G__A_SPOKEN_HALF_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SPOKEN_HALF_HOUR(v: Py) { ensure_init(); G__A_SPOKEN_HALF_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TENS_AND_UNITS() -> Py { ensure_init(); G__A_TENS_AND_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TENS_AND_UNITS(v: Py) { ensure_init(); G__A_TENS_AND_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORDED_FIGURE() -> Py { ensure_init(); G__A_WORDED_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORDED_FIGURE(v: Py) { ensure_init(); G__A_WORDED_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__MINUTES_CEILING() -> Py { ensure_init(); G__MINUTES_CEILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINUTES_CEILING(v: Py) { ensure_init(); G__MINUTES_CEILING.with(|c| *c.borrow_mut() = v); }
pub fn g__ONE_TO_NINE() -> Py { ensure_init(); G__ONE_TO_NINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONE_TO_NINE(v: Py) { ensure_init(); G__ONE_TO_NINE.with(|c| *c.borrow_mut() = v); }
pub fn g__SPOKEN_FRACTION_VALUE() -> Py { ensure_init(); G__SPOKEN_FRACTION_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPOKEN_FRACTION_VALUE(v: Py) { ensure_init(); G__SPOKEN_FRACTION_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__SPOKEN_NUMBER() -> Py { ensure_init(); G__SPOKEN_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPOKEN_NUMBER(v: Py) { ensure_init(); G__SPOKEN_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__SPOKEN_TENS() -> Py { ensure_init(); G__SPOKEN_TENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPOKEN_TENS(v: Py) { ensure_init(); G__SPOKEN_TENS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPOKEN_UNIT() -> Py { ensure_init(); G__SPOKEN_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPOKEN_UNIT(v: Py) { ensure_init(); G__SPOKEN_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_WORDS() -> Py { ensure_init(); G__UNIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_WORDS(v: Py) { ensure_init(); G__UNIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_spoken_as_numbers() -> Py { ensure_init(); G_spoken_as_numbers.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_spoken_as_numbers(v: Py) { ensure_init(); G_spoken_as_numbers.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A figure said out loud, written as the same figure in digits.\n\nPeople say the figure in words as often as in digits, and the amount reader in `memory_store`\nknew only a digit or a word up to twelve: \"half an hour\" was not a figure at all, so \"is that\nenough\" had nothing to measure against the answer. This module has no imports but `re`, and\n`memory_store` reads every line through it before it looks for an amount.\n");
    s__SPOKEN_UNIT(py_str("(?:minutes?|mins?|hours?|hrs?|days?|weeks?|nights?|miles?|km|kilometres?|kilometers?|litres?|liters?|glasses?|steps?|grams?|grammes?|calories|cals?|micrograms?|milligrams?)"));
    s__A_SPOKEN_FRACTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(half|a half|a quarter|quarter|three quarters?|a couple of|a couple|a dozen|a few)\\s+(?:of\\s+)?(?:a\\b|an\\b)?\\s*("), &g__SPOKEN_UNIT()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SPOKEN_FRACTION_VALUE(py_dict(vec![(py_str("half"), Py::Float(0.5)), (py_str("a half"), Py::Float(0.5)), (py_str("a quarter"), Py::Float(0.25)), (py_str("quarter"), Py::Float(0.25)), (py_str("three quarters"), Py::Float(0.75)), (py_str("three quarter"), Py::Float(0.75)), (py_str("a couple of"), Py::Float(2.0)), (py_str("a couple"), Py::Float(2.0)), (py_str("a dozen"), Py::Float(12.0))]));
    s__A_SPOKEN_HALF_HOUR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:(an?|one|two|three|four|five|six|seven|eight|nine)\\s+)?("), &g__SPOKEN_UNIT()), &py_str(")\\s+and\\s+(?:a\\s+)?(half|quarter)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SPOKEN_NUMBER(py_dict(vec![(py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("a hundred"), Py::Int(100)), (py_str("hundred"), Py::Int(100))]));
    s__A_RATE_UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:days?|weeks?|nights?)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_COUNT_AND_A_HALF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(one|two|three|four|five|six|seven|eight|nine|ten|a|an|\\d+)\\s+and\\s+a\\s+(half|quarter)\\s+("), &g__SPOKEN_UNIT()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SPOKEN_TENS(py_str("(?:twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)"));
    s__A_WORDED_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b("), &g__SPOKEN_TENS()), &py_str("(?:[-\\s](?:one|two|three|four|five|six|seven|eight|nine))?|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|a hundred|an?)\\s+(")), &g__SPOKEN_UNIT()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_TENS_AND_UNITS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b("), &g__SPOKEN_TENS()), &py_str(")[-\\s](one|two|three|four|five|six|seven|eight|nine)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ONE_TO_NINE(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9))]));
    s__MINUTES_CEILING(Py::Int(180));
    s__UNIT_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("a"), Py::Float(1.0)), (py_str("an"), Py::Float(1.0)), (py_str("once"), Py::Int(1)), (py_str("twice"), Py::Int(2))]));
    s_spoken_as_numbers(g__spoken_as_numbers());
}
pub fn _the_number(args: &[Py]) -> Py {
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
    let mut whole: Py = Py::None;
    let _ = py_str("A figure as digits, whole numbers without a trailing '.0'.");
    whole = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { value.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_hours(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    return Py::Bool(py_contains(&py_tuple(vec![py_str("hour"), py_str("hr")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }, "rstrip", args))(&__args) }));
    Py::None
}
pub fn _in_digits(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut amount: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "amount") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: amount"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut minutes: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One spoken figure as the same figure in digits: '30 minutes'.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_hours(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) })) } }) {
    minutes = op_mul(&amount.clone(), &Py::Int(60));
    if py_truthy(&op_le(&minutes.clone(), &g__MINUTES_CEILING())) {
    word = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(minutes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(minutes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { minutes.clone() };
    return op_modulo(&py_str("%s minutes"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) });
    }
    return op_modulo(&py_str("%s hours"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) });
    }
    word = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { amount.clone() };
    plural = if py_truthy(&{ let __b = Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(1.0))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "endswith", args))(&__args) })) } }) { py_str("s") } else { py_str("") };
    return op_modulo(&py_str("%s %s%s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number(args))(&__args) }, unit.clone(), plural.clone()]));
    Py::None
}
pub fn _spoken_as_numbers(args: &[Py]) -> Py {
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
    let mut count_and_a_half: Py = Py::None;
    let mut fractions: Py = Py::None;
    let mut half_hours: Py = Py::None;
    let mut tens_and_units: Py = Py::None;
    let mut worded: Py = Py::None;
    let _ = py_str("The line with every figure said out loud written in digits, or the line unchanged.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    return { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    }
pub fn __nested_half_hours_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count_and_a_half: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut fractions: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut half_hours: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut tens_and_units: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut worded: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut count: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) };
    count = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNIT_WORDS(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { Py::Float(1.0) } };
    part = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("half")))) { Py::Float(0.5) } else { Py::Float(0.25) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&count.clone(), &part.clone())); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _in_digits(args))(&__args) };
    Py::None
}
    half_hours = {{ let count_and_a_half = count_and_a_half.clone(); let fractions = fractions.clone(); let half_hours = half_hours.clone(); let tens_and_units = tens_and_units.clone(); let text = text.clone(); let worded = worded.clone(); py_func("half_hours", move |args: &[Py]| { let mut full = vec![count_and_a_half.clone(), fractions.clone(), half_hours.clone(), tens_and_units.clone(), text.clone(), worded.clone()]; full.extend_from_slice(args); __nested_half_hours_0(&full) }) }};
pub fn __nested_fractions_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count_and_a_half: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut fractions: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut half_hours: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut tens_and_units: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut worded: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut amount: Py = Py::None;
    let mut word: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPOKEN_FRACTION_VALUE(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&amount.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _in_digits(args))(&__args) };
    Py::None
}
    fractions = {{ let count_and_a_half = count_and_a_half.clone(); let fractions = fractions.clone(); let half_hours = half_hours.clone(); let tens_and_units = tens_and_units.clone(); let text = text.clone(); let worded = worded.clone(); py_func("fractions", move |args: &[Py]| { let mut full = vec![count_and_a_half.clone(), fractions.clone(), half_hours.clone(), tens_and_units.clone(), text.clone(), worded.clone()]; full.extend_from_slice(args); __nested_fractions_1(&full) }) }};
pub fn __nested_tens_and_units_2(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count_and_a_half: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut fractions: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut half_hours: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut tens_and_units: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut worded: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut tens: Py = Py::None;
    tens = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPOKEN_NUMBER(), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&tens.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ONE_TO_NINE(), "get", args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
    tens_and_units = {{ let count_and_a_half = count_and_a_half.clone(); let fractions = fractions.clone(); let half_hours = half_hours.clone(); let tens_and_units = tens_and_units.clone(); let text = text.clone(); let worded = worded.clone(); py_func("tens_and_units", move |args: &[Py]| { let mut full = vec![count_and_a_half.clone(), fractions.clone(), half_hours.clone(), tens_and_units.clone(), text.clone(), worded.clone()]; full.extend_from_slice(args); __nested_tens_and_units_2(&full) }) }};
pub fn __nested_count_and_a_half_3(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count_and_a_half: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut fractions: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut half_hours: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut tens_and_units: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut worded: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut count: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNIT_WORDS(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    part = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str("half")))) { Py::Float(0.5) } else { Py::Float(0.25) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&count.clone(), &part.clone())); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _in_digits(args))(&__args) };
    Py::None
}
    count_and_a_half = {{ let count_and_a_half = count_and_a_half.clone(); let fractions = fractions.clone(); let half_hours = half_hours.clone(); let tens_and_units = tens_and_units.clone(); let text = text.clone(); let worded = worded.clone(); py_func("count_and_a_half", move |args: &[Py]| { let mut full = vec![count_and_a_half.clone(), fractions.clone(), half_hours.clone(), tens_and_units.clone(), text.clone(), worded.clone()]; full.extend_from_slice(args); __nested_count_and_a_half_3(&full) }) }};
pub fn __nested_worded_4(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count_and_a_half: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut fractions: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut half_hours: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut tens_and_units: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut worded: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut amount: Py = Py::None;
    let mut word: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &word.clone()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RATE_UNIT(), "match", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    amount = Py::Float(1.0);
    } else {
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPOKEN_NUMBER(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&amount.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _in_digits(args))(&__args) };
    Py::None
}
    worded = {{ let count_and_a_half = count_and_a_half.clone(); let fractions = fractions.clone(); let half_hours = half_hours.clone(); let tens_and_units = tens_and_units.clone(); let text = text.clone(); let worded = worded.clone(); py_func("worded", move |args: &[Py]| { let mut full = vec![count_and_a_half.clone(), fractions.clone(), half_hours.clone(), tens_and_units.clone(), text.clone(), worded.clone()]; full.extend_from_slice(args); __nested_worded_4(&full) }) }};
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(count_and_a_half.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_COUNT_AND_A_HALF(), "sub", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(half_hours.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SPOKEN_HALF_HOUR(), "sub", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fractions.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SPOKEN_FRACTION(), "sub", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tens_and_units.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TENS_AND_UNITS(), "sub", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(worded.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WORDED_FIGURE(), "sub", args))(&__args) };
    return text.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.spoken".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_A_COUNT_AND_A_HALF" => Some(g__A_COUNT_AND_A_HALF()),
            "_A_RATE_UNIT" => Some(g__A_RATE_UNIT()),
            "_A_SPOKEN_FRACTION" => Some(g__A_SPOKEN_FRACTION()),
            "_A_SPOKEN_HALF_HOUR" => Some(g__A_SPOKEN_HALF_HOUR()),
            "_A_TENS_AND_UNITS" => Some(g__A_TENS_AND_UNITS()),
            "_A_WORDED_FIGURE" => Some(g__A_WORDED_FIGURE()),
            "_MINUTES_CEILING" => Some(g__MINUTES_CEILING()),
            "_ONE_TO_NINE" => Some(g__ONE_TO_NINE()),
            "_SPOKEN_FRACTION_VALUE" => Some(g__SPOKEN_FRACTION_VALUE()),
            "_SPOKEN_NUMBER" => Some(g__SPOKEN_NUMBER()),
            "_SPOKEN_TENS" => Some(g__SPOKEN_TENS()),
            "_SPOKEN_UNIT" => Some(g__SPOKEN_UNIT()),
            "_UNIT_WORDS" => Some(g__UNIT_WORDS()),
            "_in_digits" => Some(py_func("_in_digits", |args: &[Py]| _in_digits(args))),
            "_is_hours" => Some(py_func("_is_hours", |args: &[Py]| _is_hours(args))),
            "_spoken_as_numbers" => Some(py_func("_spoken_as_numbers", |args: &[Py]| _spoken_as_numbers(args))),
            "_the_number" => Some(py_func("_the_number", |args: &[Py]| _the_number(args))),
            "spoken_as_numbers" => Some(g_spoken_as_numbers()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_A_COUNT_AND_A_HALF" => s__A_COUNT_AND_A_HALF(v),
            "_A_RATE_UNIT" => s__A_RATE_UNIT(v),
            "_A_SPOKEN_FRACTION" => s__A_SPOKEN_FRACTION(v),
            "_A_SPOKEN_HALF_HOUR" => s__A_SPOKEN_HALF_HOUR(v),
            "_A_TENS_AND_UNITS" => s__A_TENS_AND_UNITS(v),
            "_A_WORDED_FIGURE" => s__A_WORDED_FIGURE(v),
            "_MINUTES_CEILING" => s__MINUTES_CEILING(v),
            "_ONE_TO_NINE" => s__ONE_TO_NINE(v),
            "_SPOKEN_FRACTION_VALUE" => s__SPOKEN_FRACTION_VALUE(v),
            "_SPOKEN_NUMBER" => s__SPOKEN_NUMBER(v),
            "_SPOKEN_TENS" => s__SPOKEN_TENS(v),
            "_SPOKEN_UNIT" => s__SPOKEN_UNIT(v),
            "_UNIT_WORDS" => s__UNIT_WORDS(v),
            "spoken_as_numbers" => s_spoken_as_numbers(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.spoken", module_obj);
    pyrt::xrefs::reg_fn("memory.spoken", "_the_number", py_func("_the_number", |args: &[Py]| _the_number(args)));
    pyrt::xrefs::reg_fn("memory.spoken", "_is_hours", py_func("_is_hours", |args: &[Py]| _is_hours(args)));
    pyrt::xrefs::reg_fn("memory.spoken", "_in_digits", py_func("_in_digits", |args: &[Py]| _in_digits(args)));
    pyrt::xrefs::reg_fn("memory.spoken", "_spoken_as_numbers", py_func("_spoken_as_numbers", |args: &[Py]| _spoken_as_numbers(args)));
    pyrt::xrefs::reg_access("memory.spoken", "_A_COUNT_AND_A_HALF", g__A_COUNT_AND_A_HALF, s__A_COUNT_AND_A_HALF);
    pyrt::xrefs::reg_access("memory.spoken", "_A_RATE_UNIT", g__A_RATE_UNIT, s__A_RATE_UNIT);
    pyrt::xrefs::reg_access("memory.spoken", "_A_SPOKEN_FRACTION", g__A_SPOKEN_FRACTION, s__A_SPOKEN_FRACTION);
    pyrt::xrefs::reg_access("memory.spoken", "_A_SPOKEN_HALF_HOUR", g__A_SPOKEN_HALF_HOUR, s__A_SPOKEN_HALF_HOUR);
    pyrt::xrefs::reg_access("memory.spoken", "_A_TENS_AND_UNITS", g__A_TENS_AND_UNITS, s__A_TENS_AND_UNITS);
    pyrt::xrefs::reg_access("memory.spoken", "_A_WORDED_FIGURE", g__A_WORDED_FIGURE, s__A_WORDED_FIGURE);
    pyrt::xrefs::reg_access("memory.spoken", "_MINUTES_CEILING", g__MINUTES_CEILING, s__MINUTES_CEILING);
    pyrt::xrefs::reg_access("memory.spoken", "_ONE_TO_NINE", g__ONE_TO_NINE, s__ONE_TO_NINE);
    pyrt::xrefs::reg_access("memory.spoken", "_SPOKEN_FRACTION_VALUE", g__SPOKEN_FRACTION_VALUE, s__SPOKEN_FRACTION_VALUE);
    pyrt::xrefs::reg_access("memory.spoken", "_SPOKEN_NUMBER", g__SPOKEN_NUMBER, s__SPOKEN_NUMBER);
    pyrt::xrefs::reg_access("memory.spoken", "_SPOKEN_TENS", g__SPOKEN_TENS, s__SPOKEN_TENS);
    pyrt::xrefs::reg_access("memory.spoken", "_SPOKEN_UNIT", g__SPOKEN_UNIT, s__SPOKEN_UNIT);
    pyrt::xrefs::reg_access("memory.spoken", "_UNIT_WORDS", g__UNIT_WORDS, s__UNIT_WORDS);
    pyrt::xrefs::reg_access("memory.spoken", "spoken_as_numbers", g_spoken_as_numbers, s_spoken_as_numbers);
}

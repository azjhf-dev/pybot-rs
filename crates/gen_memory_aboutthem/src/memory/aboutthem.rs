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


thread_local! { static G_AHEAD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AHEAD_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_SAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPOKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.aboutthem: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AHEAD_ASK() -> Py { ensure_init(); G_AHEAD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AHEAD_ASK(v: Py) { ensure_init(); G_AHEAD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AHEAD_TIME() -> Py { ensure_init(); G_AHEAD_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AHEAD_TIME(v: Py) { ensure_init(); G_AHEAD_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_MONTHS() -> Py { ensure_init(); G_BABY_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_MONTHS(v: Py) { ensure_init(); G_BABY_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_AGE() -> Py { ensure_init(); G_CHILD_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_AGE(v: Py) { ensure_init(); G_CHILD_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVEN() -> Py { ensure_init(); G_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVEN(v: Py) { ensure_init(); G_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_HE() -> Py { ensure_init(); G_HE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HE(v: Py) { ensure_init(); G_HE.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_SAME() -> Py { ensure_init(); G_KIN_SAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_SAME(v: Py) { ensure_init(); G_KIN_SAME.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN() -> Py { ensure_init(); G_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); G_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_SHE() -> Py { ensure_init(); G_SHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHE(v: Py) { ensure_init(); G_SHE.with(|c| *c.borrow_mut() = v); }
pub fn g_SPOKEN() -> Py { ensure_init(); G_SPOKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPOKEN(v: Py) { ensure_init(); G_SPOKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_AGE() -> Py { ensure_init(); G_THEIR_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_AGE(v: Py) { ensure_init(); G_THEIR_AGE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An ask about a person in the chat, written out with the age the store keeps for them.\n\n\"my son is 5\" then \"how much sleep does he need\" was answered with the adult figure - seven to\nnine hours, coffee at three, how you feel by mid-afternoon - because a line whose subject is a\npronoun names no age for the knowledge base to read, and the sleep entries are keyed on the age\n(\"how much sleep does a 5 year old need\" is the ten-to-thirteen answer). The store knew the age\nall along: it filed it on the turn before.\n\nSo the ask is written out as the question a person would ask if they were talking to someone who\ncould not see their family: the pronoun becomes \"a 5 year old\". Only a child, and only when the\nstore holds the age: an adult's sleep, water and bedtime are the figures the adult entries already\ngive, so there is nothing to write out and no answer to change.\n");
    s_HE(pyrt::xrefs::xget("memory.facts", "HE"));
    s_SHE(pyrt::xrefs::xget("memory.facts", "SHE"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.PRONOUN"); }
    s_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?P<own>his|her|their)\\s+(?P<thing>[a-z][a-z' -]{0,20})|\\b(?P<who>he|she|they)\\b|\\b(?P<obj>him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.THEIR_AGE"); }
    s_THEIR_AGE(stdlib_call("re", "compile", &[py_str("\\bhow\\s+old\\b|\\bwhat\\s+age\\b|\\bhow\\s+big\\b|\\btheir\\s+age\\b|\\bhis\\s+age\\b|\\bher\\s+age\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.ASKS"); }
    s_ASKS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many|long|often|early|late)\\b|\\bwhat\\s+time\\b|\\bwhen\\s+(?:is|are|do|does|will|can)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.GIVEN"); }
    s_GIVEN(stdlib_call("re", "compile", &[py_str("\\b\\d{1,3}\\b|\\ba\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.CHILD_AGE"); }
    s_CHILD_AGE(Py::Int(18));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.BABY_MONTHS"); }
    s_BABY_MONTHS(Py::Int(24));
    fn __data1() -> Py { py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve")), (Py::Int(13), py_str("thirteen")), (Py::Int(14), py_str("fourteen")), (Py::Int(15), py_str("fifteen")), (Py::Int(16), py_str("sixteen")), (Py::Int(17), py_str("seventeen"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.SPOKEN"); }
    s_SPOKEN(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.AHEAD_ASK"); }
    s_AHEAD_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+old|what\\s+age)\\s+(?:will|would|is|does)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.AHEAD_TIME"); }
    s_AHEAD_TIME(stdlib_call("re", "compile", &[py_str("\\bnext\\s+(?P<next>year|summer|birthday)\\b|\\bthis\\s+time\\s+next\\s+year\\b|\\bin\\s+(?P<years>\\d{1,2}|a|one|two|three|four|five|six|seven|eight|nine|ten)\\s+years?\\b|\\bin\\s+(?P<year>(?:19|20)\\d\\d)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.COUNT"); }
    s_COUNT({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[py_tuple(vec![py_str("zero"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve")]), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4855 = py_unpack(&__item.clone(), 2, None); let mut number: Py = __unpk4855[0].clone(); let mut word: Py = __unpk4855[1].clone(); __out.push((word.clone(), number.clone())); } py_dict(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.KIN"); }
    s_KIN(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:little\\s+|big\\s+|eldest\\s+|youngest\\s+)?(?P<who>son|daughter|boy|girl|child|kid|lad|little one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.aboutthem.KIN_SAME"); }
    s_KIN_SAME(py_dict(vec![(py_str("son"), py_tuple(vec![py_str("son"), py_str("boy")])), (py_str("boy"), py_tuple(vec![py_str("boy"), py_str("son")])), (py_str("lad"), py_tuple(vec![py_str("lad"), py_str("boy"), py_str("son")])), (py_str("daughter"), py_tuple(vec![py_str("daughter"), py_str("girl")])), (py_str("girl"), py_tuple(vec![py_str("girl"), py_str("daughter")])), (py_str("child"), py_tuple(vec![py_str("child"), py_str("kid"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl")])), (py_str("kid"), py_tuple(vec![py_str("kid"), py_str("child"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl")])), (py_str("little one"), py_tuple(vec![py_str("little one"), py_str("child"), py_str("kid"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl")]))]));
}
pub fn _the_child(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut age: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut found: Py = Py::None;
    let mut months: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(how old the child is, written out, and the person) or (None, \"\").");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    if py_truthy(&{ let __b = mcall(&g_THEIR_AGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GIVEN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS(), "search", &[said.clone()]))) } } } }) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.topics", "_leads_a_question", &[said.clone()]))) } }) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    found = mcall(&g_PRONOUN(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    word = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("own")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("obj")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("she"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("her"))) } }) {
    fits = pyrt::xrefs::xget("memory.facts", "SHE");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("he"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("his"))) } }) {
    fits = pyrt::xrefs::xget("memory.facts", "HE");
    } else {
    fits = op_bitor(&pyrt::xrefs::xget("memory.facts", "SHE"), &pyrt::xrefs::xget("memory.facts", "HE"));
    }
    }
    let __iter1 = py_iter_or_raise(&_people(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&fits.clone(), &who.clone())) } }) {
    continue;
    }
    months = mcall(&entry.clone(), "get", &[py_str("months")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[months.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __cmp21: Py = months.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp21)) { op_lt(&__cmp21, &g_BABY_MONTHS()) } else { Py::Bool(false) } } } }) {
    return py_tuple(vec![op_modulo(&py_str("%s month old"), &mcall(&g_SPOKEN(), "get", &[months.clone(), call_builtin("str", &[months.clone()]).unwrap_or_else(|e| exc::raise(e))])), who.clone()]);
    }
    age = mcall(&entry.clone(), "get", &[py_str("age")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __cmp31: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp31)) { op_lt(&__cmp31, &g_CHILD_AGE()) } else { Py::Bool(false) } } } }) {
    return py_tuple(vec![op_modulo(&py_str("%s year old"), &mcall(&g_SPOKEN(), "get", &[age.clone(), call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))])), who.clone()]);
    }
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _people(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut person: Py = Py::None;
    let _ = py_str("The person the chat was last about first, then everybody the store keeps a fact about.");
    found = py_list(vec![]);
    person = mcall(&memory.clone(), "get", &[py_str("person")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&person.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&mcall(&person.clone(), "get", &[py_str("age")]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&person.clone(), "get", &[py_str("months")]), &Py::None)) } } } } } }) {
    let _ = mcall(&found.clone(), "append", &[person.clone()]);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[entry.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn with_the_age(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _who: Py = Py::None;
    let mut found: Py = Py::None;
    let mut how_old: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The line asked about a child of that age: \"how much sleep does he need\" -> \"how much\n    sleep does a 5 year old need\". The line itself when there is no child in it.");
    let __unpk1 = py_unpack(&_the_child(&[text.clone(), memory.clone()]), 2, None);
    how_old = __unpk1[0].clone();
    _who = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&how_old.clone()))) {
    return py_str("");
    }
    said = call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e));
    found = mcall(&g_PRONOUN(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    if py_truthy(&mcall(&found.clone(), "group", &[py_str("own")])) {
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[]);
    written = op_add(&op_add(&slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &op_modulo(&py_str("a %s's %s"), &py_tuple(vec![how_old.clone(), thing.clone()]))), &slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    } else {
    written = op_add(&op_add(&op_add(&slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &py_str("a ")), &how_old.clone()), &slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    }
    written = mcall(&py_str(" "), "join", &[mcall(&written.clone(), "split", &[])]);
    return if py_truthy(&Py::Bool(!py_eq(&written.clone(), &said.clone()))) { written.clone() } else { py_str("") };
    Py::None
}
pub fn an_age_ahead(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut age: Py = Py::None;
    let mut day: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut when: Py = Py::None;
    let mut words: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("The age the child will be at a day further ahead, said back, or None.\n\n    Only for a child whose age the store keeps in whole years: a baby kept in months is a length\n    the arithmetic here would get wrong, so its ask is left to whoever knows more.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AHEAD_ASK(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    when = mcall(&g_AHEAD_TIME(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_one_asked_about(&[said.clone(), memory.clone()]), 2, None);
    age = __unpk1[0].clone();
    words = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    return Py::None;
    }
    years = _the_years_ahead(&[when.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&years.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&years.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    day = _the_day_named(&[when.clone(), years.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&day.clone()))) {
    return Py::None;
    }
    subject = _the_subject(&[words.clone()]);
    verb = op_add(&if py_truthy(&mcall(&words.clone(), "startswith", &[py_str("your ")])) { py_str(" ") } else { py_str("") }, &_the_future(&[words.clone()]));
    return op_modulo(&py_str("%s%s %s %s."), &py_tuple(vec![subject.clone(), verb.clone(), _an_age_said(&[op_add(&age.clone(), &years.clone())]), day.clone()]));
    Py::None
}
pub fn _the_years_ahead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut count: Py = Py::None;
    let _ = py_str("How many years ahead the ask is, or None when the day it names is one to leave alone.");
    if py_truthy(&mcall(&when.clone(), "group", &[py_str("next")])) {
    return if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&when.clone(), "group", &[py_str("next")]), "lower", &[]), &py_str("year")))) { Py::Int(1) } else { Py::None };
    }
    if py_truthy(&mcall(&when.clone(), "group", &[py_str("year")])) {
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), op_sub(&call_builtin("int", &[mcall(&when.clone(), "group", &[py_str("year")])]).unwrap_or_else(|e| exc::raise(e)), &gattr(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "year"))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    count = mcall(&when.clone(), "group", &[py_str("years")]);
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return Py::None;
    }
    return if py_truthy(&mcall(&count.clone(), "isdigit", &[])) { call_builtin("int", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g_COUNT(), "get", &[mcall(&count.clone(), "lower", &[])]) };
    Py::None
}
pub fn _the_day_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut years: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "years") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: years"))) };
    let _ = py_str("The day the ask names, said the way a person says it: \"next year\", \"in 2030\", \"in two years\".");
    if py_truthy(&Py::Bool(py_eq(&years.clone(), &Py::Int(0)))) {
    return if py_truthy(&Py::Bool(!py_truthy(&mcall(&when.clone(), "group", &[py_str("year")])))) { py_str("now") } else { op_modulo(&py_str("in %s"), &mcall(&when.clone(), "group", &[py_str("year")])) };
    }
    if py_truthy(&mcall(&when.clone(), "group", &[py_str("year")])) {
    return op_modulo(&py_str("in %s"), &mcall(&when.clone(), "group", &[py_str("year")]));
    }
    if py_truthy(&Py::Bool(py_eq(&years.clone(), &Py::Int(1)))) {
    return py_str("next year");
    }
    return op_modulo(&py_str("in %s years"), &mcall(&g_SPOKEN(), "get", &[years.clone(), call_builtin("str", &[years.clone()]).unwrap_or_else(|e| exc::raise(e))]));
    Py::None
}
pub fn _the_one_asked_about(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut age: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut people: Py = Py::None;
    let mut said_who: Py = Py::None;
    let mut who: Py = Py::None;
    let mut who_said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(the age the store keeps, the words the reply uses for that person) or (None, \"\").");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    people = _people(&[memory.clone()]);
    named = mcall(&g_KIN(), "search", &[said.clone()]);
    if py_truthy(&named.clone()) {
    who_said = mcall(&mcall(&named.clone(), "group", &[py_str("who")]), "lower", &[]);
    fits = mcall(&g_KIN_SAME(), "get", &[who_said.clone(), py_tuple(vec![who_said.clone()])]);
    let __iter1 = py_iter_or_raise(&people.clone());
    for __item in __iter1 {
    entry = __item.clone();
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&fits.clone(), &who.clone())) } }) {
    continue;
    }
    age = mcall(&entry.clone(), "get", &[py_str("age")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __cmp21: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp21)) { op_lt(&__cmp21, &g_CHILD_AGE()) } else { Py::Bool(false) } } } }) {
    return py_tuple(vec![age.clone(), op_add(&py_str("your "), &who_said.clone())]);
    }
    }
    return py_tuple(vec![Py::None, py_str("")]);
    }
    found = mcall(&g_PRONOUN(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    word = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("own")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("obj")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("her")]), &word.clone()))) {
    let __unpk3 = py_unpack(&py_tuple(vec![pyrt::xrefs::xget("memory.facts", "SHE"), py_str("she")]), 2, None);
    fits = __unpk3[0].clone();
    said_who = __unpk3[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("his"), py_str("him")]), &word.clone()))) {
    let __unpk4 = py_unpack(&py_tuple(vec![pyrt::xrefs::xget("memory.facts", "HE"), py_str("he")]), 2, None);
    fits = __unpk4[0].clone();
    said_who = __unpk4[1].clone();
    } else {
    let __unpk5 = py_unpack(&py_tuple(vec![op_bitor(&pyrt::xrefs::xget("memory.facts", "SHE"), &pyrt::xrefs::xget("memory.facts", "HE")), py_str("they")]), 2, None);
    fits = __unpk5[0].clone();
    said_who = __unpk5[1].clone();
    }
    }
    let __iter6 = py_iter_or_raise(&people.clone());
    for __item in __iter6 {
    entry = __item.clone();
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&fits.clone(), &who.clone())) } }) {
    continue;
    }
    age = mcall(&entry.clone(), "get", &[py_str("age")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __cmp71: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp71)) { op_lt(&__cmp71, &g_CHILD_AGE()) } else { Py::Bool(false) } } } }) {
    return py_tuple(vec![age.clone(), said_who.clone()]);
    }
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _an_age_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut age: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let _ = py_str("An age said the way a person says it: \"seven\" while a child, \"34\" after that.");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(12)) } else { Py::Bool(false) } } } }) {
    return mcall(&g_SPOKEN(), "get", &[age.clone(), call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_subject(args: &[Py]) -> Py {
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
    let _ = py_str("The words a reply about that person opens with.");
    words = mcall(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    return if py_truthy(&words.clone()) { op_add(&mcall(&slice_or_raise(&words.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { py_str("They") };
    Py::None
}
pub fn _the_future(args: &[Py]) -> Py {
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
    let _ = py_str("The verb, joined to the person the way a person says it: \"He'll be\", \"Your son will be\".");
    return if py_truthy(&mcall(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "startswith", &[py_str("your ")])) { py_str("will be") } else { py_str("'ll be") };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.aboutthem".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AHEAD_ASK" => Some(g_AHEAD_ASK()),
            "AHEAD_TIME" => Some(g_AHEAD_TIME()),
            "ASKS" => Some(g_ASKS()),
            "BABY_MONTHS" => Some(g_BABY_MONTHS()),
            "CHILD_AGE" => Some(g_CHILD_AGE()),
            "COUNT" => Some(g_COUNT()),
            "GIVEN" => Some(g_GIVEN()),
            "HE" => Some(g_HE()),
            "KIN" => Some(g_KIN()),
            "KIN_SAME" => Some(g_KIN_SAME()),
            "PRONOUN" => Some(g_PRONOUN()),
            "SHE" => Some(g_SHE()),
            "SPOKEN" => Some(g_SPOKEN()),
            "THEIR_AGE" => Some(g_THEIR_AGE()),
            "_an_age_said" => Some(py_func("_an_age_said", |args: &[Py]| _an_age_said(args))),
            "_people" => Some(py_func("_people", |args: &[Py]| _people(args))),
            "_the_child" => Some(py_func("_the_child", |args: &[Py]| _the_child(args))),
            "_the_day_named" => Some(py_func("_the_day_named", |args: &[Py]| _the_day_named(args))),
            "_the_future" => Some(py_func("_the_future", |args: &[Py]| _the_future(args))),
            "_the_one_asked_about" => Some(py_func("_the_one_asked_about", |args: &[Py]| _the_one_asked_about(args))),
            "_the_subject" => Some(py_func("_the_subject", |args: &[Py]| _the_subject(args))),
            "_the_years_ahead" => Some(py_func("_the_years_ahead", |args: &[Py]| _the_years_ahead(args))),
            "an_age_ahead" => Some(py_func("an_age_ahead", |args: &[Py]| an_age_ahead(args))),
            "with_the_age" => Some(py_func("with_the_age", |args: &[Py]| with_the_age(args))),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "datetime" => Some(stdlib_module("datetime")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AHEAD_ASK" => s_AHEAD_ASK(v),
            "AHEAD_TIME" => s_AHEAD_TIME(v),
            "ASKS" => s_ASKS(v),
            "BABY_MONTHS" => s_BABY_MONTHS(v),
            "CHILD_AGE" => s_CHILD_AGE(v),
            "COUNT" => s_COUNT(v),
            "GIVEN" => s_GIVEN(v),
            "HE" => s_HE(v),
            "KIN" => s_KIN(v),
            "KIN_SAME" => s_KIN_SAME(v),
            "PRONOUN" => s_PRONOUN(v),
            "SHE" => s_SHE(v),
            "SPOKEN" => s_SPOKEN(v),
            "THEIR_AGE" => s_THEIR_AGE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.aboutthem", module_obj);
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_child", py_func("_the_child", |args: &[Py]| _the_child(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_people", py_func("_people", |args: &[Py]| _people(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "with_the_age", py_func("with_the_age", |args: &[Py]| with_the_age(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "an_age_ahead", py_func("an_age_ahead", |args: &[Py]| an_age_ahead(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_years_ahead", py_func("_the_years_ahead", |args: &[Py]| _the_years_ahead(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_day_named", py_func("_the_day_named", |args: &[Py]| _the_day_named(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_one_asked_about", py_func("_the_one_asked_about", |args: &[Py]| _the_one_asked_about(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_an_age_said", py_func("_an_age_said", |args: &[Py]| _an_age_said(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_subject", py_func("_the_subject", |args: &[Py]| _the_subject(args)));
    pyrt::xrefs::reg_fn("memory.aboutthem", "_the_future", py_func("_the_future", |args: &[Py]| _the_future(args)));
    pyrt::xrefs::reg_access("memory.aboutthem", "AHEAD_ASK", g_AHEAD_ASK, s_AHEAD_ASK);
    pyrt::xrefs::reg_access("memory.aboutthem", "AHEAD_TIME", g_AHEAD_TIME, s_AHEAD_TIME);
    pyrt::xrefs::reg_access("memory.aboutthem", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.aboutthem", "BABY_MONTHS", g_BABY_MONTHS, s_BABY_MONTHS);
    pyrt::xrefs::reg_access("memory.aboutthem", "CHILD_AGE", g_CHILD_AGE, s_CHILD_AGE);
    pyrt::xrefs::reg_access("memory.aboutthem", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("memory.aboutthem", "GIVEN", g_GIVEN, s_GIVEN);
    pyrt::xrefs::reg_access("memory.aboutthem", "HE", g_HE, s_HE);
    pyrt::xrefs::reg_access("memory.aboutthem", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("memory.aboutthem", "KIN_SAME", g_KIN_SAME, s_KIN_SAME);
    pyrt::xrefs::reg_access("memory.aboutthem", "PRONOUN", g_PRONOUN, s_PRONOUN);
    pyrt::xrefs::reg_access("memory.aboutthem", "SHE", g_SHE, s_SHE);
    pyrt::xrefs::reg_access("memory.aboutthem", "SPOKEN", g_SPOKEN, s_SPOKEN);
    pyrt::xrefs::reg_access("memory.aboutthem", "THEIR_AGE", g_THEIR_AGE, s_THEIR_AGE);
}

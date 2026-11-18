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


thread_local! { static G_A_BIRTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BIRTH_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_BORN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.newborn: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_BIRTH() -> Py { ensure_init(); G_A_BIRTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BIRTH(v: Py) { ensure_init(); G_A_BIRTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BIRTH_WEIGHT() -> Py { ensure_init(); G_A_BIRTH_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BIRTH_WEIGHT(v: Py) { ensure_init(); G_A_BIRTH_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_YEAR() -> Py { ensure_init(); G_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_YEAR(v: Py) { ensure_init(); G_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_WORDS() -> Py { ensure_init(); G_BABY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_WORDS(v: Py) { ensure_init(); G_BABY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY_TURNS() -> Py { ensure_init(); G_HOW_MANY_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_TURNS(v: Py) { ensure_init(); G_HOW_MANY_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_BORN() -> Py { ensure_init(); G_JUST_BORN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_BORN(v: Py) { ensure_init(); G_JUST_BORN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_BABY() -> Py { ensure_init(); G_NOT_THE_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_BABY(v: Py) { ensure_init(); G_NOT_THE_BABY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A new baby in the chat is what the next few lines are about.\n\nPeople tell you the news and then talk about it for a while without naming it again: \"my sister\nhad her baby on tuesday\", \"a girl, 7lb 2oz\", then \"what should i get them\". The store keeps the\nturns, so a line that names no subject can still be read as being about the baby. This answers one\nquestion only - is the chat about a new baby - and leaves what to say to whoever asked.\n");
    s_BABY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nbaby babies babydoll newborn babe infant toddler twins nappy nappies pram buggy cot\nlabour labor midwife caesarean csection breastfeeding bottle-feeding christening\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BIRTH(stdlib_call("re", "compile", &[py_str("\\b(?:had|has|having|gave|give|giving|born|arrived|arrives|delivered|induced|in\\s+labour|in\\s+labor|went\\s+into\\s+labour|went\\s+into\\s+labor)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUST_BORN(stdlib_call("re", "compile", &[py_str("\\bborn\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_YEAR(stdlib_call("re", "compile", &[py_str("\\b(?:19|20)\\d\\d\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BIRTH_WEIGHT(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:lb|lbs|pound|pounds|oz|ounces)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:puppy|kitten|dog|cat|lamb|calf|chick|seedling|plant)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_TURNS(Py::Int(3));
}
pub fn _the_lines(args: &[Py]) -> Py {
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
    let mut current: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The last few turns as words, newest first, without the line being answered.");
    lines = py_list(vec![]);
    current = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("topic")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[current.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&lines.clone(), "append", &[current.clone()]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_HOW_MANY_TURNS()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&lines.clone(), "append", &[record.clone()]);
    }
    }
    said = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&lines.clone());
    for __item in __iter2 {
    record = __item.clone();
    words = mcall(&record.clone(), "get", &[py_str("words")]);
    text = { let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } };
    let _ = mcall(&said.clone(), "append", &[mcall(&py_str(" "), "join", &[if py_truthy(&{ let __b = call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { words.clone() } }) { op_add(&py_list(vec![text.clone()]), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { py_list(vec![text.clone()]) }])]);
    }
    return said.clone();
    Py::None
}
pub fn the_thread_has_a_new_baby(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut together: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the last few turns are about a baby arriving.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_THE_BABY(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    said = _the_lines(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    together = mcall(&py_str(" "), "join", &[said.clone()]);
    if py_truthy(&mcall(&g_NOT_THE_BABY(), "search", &[together.clone()])) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&together.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_JUST_BORN(), "search", &[together.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_YEAR(), "search", &[together.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_BABY_WORDS()); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_BIRTH(), "search", &[together.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_BIRTH_WEIGHT(), "search", &[together.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_A_BIRTH_WEIGHT(), "search", &[together.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_BIRTH(), "search", &[together.clone()]) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_baby_is_new(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("True when the baby in the chat is a newborn, so a present is the newborn kind.");
    return the_thread_has_a_new_baby(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.newborn".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BIRTH" => Some(g_A_BIRTH()),
            "A_BIRTH_WEIGHT" => Some(g_A_BIRTH_WEIGHT()),
            "A_YEAR" => Some(g_A_YEAR()),
            "BABY_WORDS" => Some(g_BABY_WORDS()),
            "HOW_MANY_TURNS" => Some(g_HOW_MANY_TURNS()),
            "JUST_BORN" => Some(g_JUST_BORN()),
            "NOT_THE_BABY" => Some(g_NOT_THE_BABY()),
            "_the_lines" => Some(py_func("_the_lines", |args: &[Py]| _the_lines(args))),
            "the_baby_is_new" => Some(py_func("the_baby_is_new", |args: &[Py]| the_baby_is_new(args))),
            "the_thread_has_a_new_baby" => Some(py_func("the_thread_has_a_new_baby", |args: &[Py]| the_thread_has_a_new_baby(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BIRTH" => s_A_BIRTH(v),
            "A_BIRTH_WEIGHT" => s_A_BIRTH_WEIGHT(v),
            "A_YEAR" => s_A_YEAR(v),
            "BABY_WORDS" => s_BABY_WORDS(v),
            "HOW_MANY_TURNS" => s_HOW_MANY_TURNS(v),
            "JUST_BORN" => s_JUST_BORN(v),
            "NOT_THE_BABY" => s_NOT_THE_BABY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.newborn", module_obj);
    pyrt::xrefs::reg_fn("memory.newborn", "_the_lines", py_func("_the_lines", |args: &[Py]| _the_lines(args)));
    pyrt::xrefs::reg_fn("memory.newborn", "the_thread_has_a_new_baby", py_func("the_thread_has_a_new_baby", |args: &[Py]| the_thread_has_a_new_baby(args)));
    pyrt::xrefs::reg_fn("memory.newborn", "the_baby_is_new", py_func("the_baby_is_new", |args: &[Py]| the_baby_is_new(args)));
    pyrt::xrefs::reg_access("memory.newborn", "A_BIRTH", g_A_BIRTH, s_A_BIRTH);
    pyrt::xrefs::reg_access("memory.newborn", "A_BIRTH_WEIGHT", g_A_BIRTH_WEIGHT, s_A_BIRTH_WEIGHT);
    pyrt::xrefs::reg_access("memory.newborn", "A_YEAR", g_A_YEAR, s_A_YEAR);
    pyrt::xrefs::reg_access("memory.newborn", "BABY_WORDS", g_BABY_WORDS, s_BABY_WORDS);
    pyrt::xrefs::reg_access("memory.newborn", "HOW_MANY_TURNS", g_HOW_MANY_TURNS, s_HOW_MANY_TURNS);
    pyrt::xrefs::reg_access("memory.newborn", "JUST_BORN", g_JUST_BORN, s_JUST_BORN);
    pyrt::xrefs::reg_access("memory.newborn", "NOT_THE_BABY", g_NOT_THE_BABY, s_NOT_THE_BABY);
}

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
pub static _A_TENANCY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_RENT_RAISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g__A_TENANCY() -> Py { ensure_init(); _A_TENANCY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_TENANCY(v: Py) { ensure_init(); if let Ok(mut w) = _A_TENANCY.write() { *w = v; } }
pub fn g__NOT_A_RENT_RAISE() -> Py { ensure_init(); _NOT_A_RENT_RAISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_RENT_RAISE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_RENT_RAISE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A tenancy is not a rent rise and not a manager: what a tenant can do about the landlord.\n\nThe home-and-DIY rows answer the fault itself (the damp, the boiler, the repair letter) and the\nmoney rows answer the rent, so a complaint about a landlord fell between them. On main:\n\n    i want to raise a complaint about my landlord        -> the section 13 rent-rise card\n    i need to raise an issue with my landlord about ...  -> \"Ask for a short private meeting\"\n\nTwo things were wrong, and both are fixed here rather than in the files that held the rows, so that\nthis is one file with one subject and no half of somebody else's file changes under them:\n\n1. `money.py`'s rent-rise row keys on \"landlord\" and on the verb \"raise\" alone, and `work.py`'s\n   raise-with-boss row keys on the verb too. \"raise\" is a verb whose object decides the question: a\n   rent rise is a raise of the rent, and a complaint, a concern, an issue, an alarm, a glass, a\n   voice, prices or funds are somebody else's question. A tenancy is not a manager either. The two\n   fences below stand those rows down, the way `bot/knowledge/__init__.py` gives a whole domain a\n   STANDS_ASIDE guard - a row that already has a guard of its own keeps it.\n2. The complaint itself had no card anywhere, so the line fell to an echo. It is a procedure with\n   its own order - the letter and 14 days, the agent's free redress scheme, the council's housing\n   team for a hazard, the deposit scheme, Shelter or Citizens Advice - and it is the row below.\n\nThe file is a domain like any other: ENTRIES, keyed on who runs the tenancy and on the act of\ncomplaining, and nothing keyed on a wording.\n");
    s__NOT_A_RENT_RAISE(stdlib_call("re", "compile", &[py_str("\\brais(?:e|ed|es|ing)\\s+(?:a|an|the|my|our|your|their|any|some)?\\s*(?:complaints?|concerns?|issues?|problems?|matters?|questions?|grievances?|objections?|disputes?|alarms?|glass|glasses|toast|voices?|prices?|funds?|money|children|kids|awareness|profile|hell)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TENANCY(stdlib_call("re", "compile", &[py_str("\\b(?:landlords?|landlad(?:y|ies)|letting agents?|letting agency|lettings agency|tenants?|tenancy|tenancies|housing association|council house)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let _ = _fence(&[py_tuple(vec![crate::gen::knowledge::money::module_obj()]), py_str("rent-rise"), g__a_raise_with_an_object()]);
    let _ = _fence(&[py_tuple(vec![crate::gen::knowledge::work::module_obj()]), py_str("raise-with-boss"), g__a_tenancy_and_not_a_manager()]);
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("landlord-complaint")); __args.push(py_tuple(vec![py_tuple(vec![py_str("landlord"), py_str("landlords"), py_str("landlady"), py_str("landladies"), py_str("letting agent"), py_str("letting agents"), py_str("letting agency"), py_str("lettings agent"), py_str("housing association"), py_str("my tenancy"), py_str("the tenancy"), py_str("my letting agent")]), py_tuple(vec![py_str("complain"), py_str("complains"), py_str("complained"), py_str("complaining"), py_str("complaint"), py_str("complaints"), py_str("formal complaint"), py_str("raise a complaint"), py_str("put in a complaint"), py_str("make a complaint"), py_str("made a complaint"), py_str("log a complaint"), py_str("report"), py_str("reported"), py_str("redress"), py_str("ombudsman"), py_str("deadlock"), py_str("not happy with"), py_str("unhappy with"), py_str("dispute"), py_str("issue"), py_str("issues"), py_str("concern"), py_str("concerns"), py_str("problem"), py_str("problems"), py_str("matter"), py_str("matters"), py_str("grievance")])])); __args.push(py_str("Put it in writing to the landlord - a dated email or letter, what happened, what you want done and a deadline of 14 days - and keep a copy, because every later step asks for it. If a letting agent runs the flat, the agent has to belong to a free redress scheme (the Property Redress Scheme or The Property Ombudsman), and a complaint can go there after eight weeks, or sooner if the agent deadlocks it. A hazard - damp, no heating, a broken boiler, a fire risk - is your council's housing team, who can inspect and order the work, and the deposit scheme holding your deposit sorts out a deposit argument. Shelter or Citizens Advice will read the letter before you send it, free.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("landlord"), py_str("landlady"), py_str("agent"), py_str("tenancy"), py_str("tenants"), py_str("tenant"), py_str("flat"), py_str("complain"), py_str("complaint"), py_str("redress"), py_str("ombudsman")])), ("avoid".to_string(), py_tuple(vec![py_str("fix"), py_str("fixed"), py_str("fixing"), py_str("repair"), py_str("repairs"), py_str("broken"), py_str("boiler"), py_str("heating"), py_str("hot water"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("deposit"), py_str("deposits"), py_str("selling"), py_str("sell"), py_str("sale"), py_str("sold"), py_str("evict"), py_str("evicted"), py_str("eviction"), py_str("section 21"), py_str("section 8"), py_str("notice to quit"), py_str("kick me out"), py_str("throw me out"), py_str("arrears"), py_str("behind on rent"), py_str("rent increase"), py_str("raise the rent"), py_str("raised the rent"), py_str("put the rent up"), py_str("eviction notice")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy")]));
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
    let mut reading: Py = Py::None;
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return mcall(&understood.clone(), "lower", &[]);
    }
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    Py::None
}
pub fn _a_raise_with_an_object(args: &[Py]) -> Py {
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
    let _ = py_str("True when the raise is of something that is not the rent.");
    return call_builtin("bool", &[mcall(&g__NOT_A_RENT_RAISE(), "search", &[_said(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_tenancy_and_not_a_manager(args: &[Py]) -> Py {
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
    let _ = py_str("True when the landlord in the line makes it a tenancy question, not a manager one.");
    return call_builtin("bool", &[mcall(&g__A_TENANCY(), "search", &[_said(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut modules: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "modules") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: modules"))) };
    let mut row_id: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row_id") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row_id"))) };
    let mut when: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut module: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("Give one row of another domain a guard, unless it already has one of its own.");
    let __iter1 = py_iter_or_raise(&modules.clone());
    for __item in __iter1 {
    module = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[module.clone(), py_str("ENTRIES"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    row = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&gattr(&row.clone(), "id"), &row_id.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&call_builtin("getattr", &[row.clone(), py_str("guard"), Py::None]).unwrap_or_else(|e| exc::raise(e)), &Py::None)) } }) {
    let _ = py_setattr(&row.clone(), "guard", py_func("lambda", move |args: &[Py]| { let module = module.clone(); let modules = modules.clone(); let row = row.clone(); let row_id = row_id.clone(); let when = when.clone(); let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); let when: Py = __kwp(args, 1, "when").unwrap_or_else(|| when.clone()); Py::Bool(!py_truthy(&py_call(&when, &[understood.clone()]))) }));
    }
    }
    }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.tenancy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_TENANCY" => Some(g__A_TENANCY()),
            "_NOT_A_RENT_RAISE" => Some(g__NOT_A_RENT_RAISE()),
            "_a_raise_with_an_object" => Some(py_func("_a_raise_with_an_object", |args: &[Py]| _a_raise_with_an_object(args))),
            "_a_tenancy_and_not_a_manager" => Some(py_func("_a_tenancy_and_not_a_manager", |args: &[Py]| _a_tenancy_and_not_a_manager(args))),
            "_fence" => Some(py_func("_fence", |args: &[Py]| _fence(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            _ => None,
        }),
        set: None,
    }))
}

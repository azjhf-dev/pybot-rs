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
pub static ICING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ICING_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_ICING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SET_ICING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FROSTING_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ICING() -> Py { ensure_init(); ICING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ICING(v: Py) { ensure_init(); if let Ok(mut w) = ICING.write() { *w = v; } }
pub fn g_ICING_KEEP() -> Py { ensure_init(); ICING_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ICING_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = ICING_KEEP.write() { *w = v; } }
pub fn g_NOT_THE_ICING() -> Py { ensure_init(); NOT_THE_ICING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_ICING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_ICING.write() { *w = v; } }
pub fn g_SET_ICING() -> Py { ensure_init(); SET_ICING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SET_ICING(v: Py) { ensure_init(); if let Ok(mut w) = SET_ICING.write() { *w = v; } }
pub fn g__FROSTING_UP() -> Py { ensure_init(); _FROSTING_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FROSTING_UP(v: Py) { ensure_init(); if let Ok(mut w) = _FROSTING_UP.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The icing on a cake: how long it keeps, where to keep it, and whether it freezes.\n\nThe commonest question about a frosting is not how to make one - it is whether it can be made\ntomorrow, whether it survives the fridge and what to do with what is left. Every one of those is\na keeping question about the icing, and the icing is a subject of its own: buttercream is not the\ncake, and the numbers are different (a cake keeps 3 days in a tin; a buttercream keeps 3 to 4 days\nin a tub in the fridge and 2 months in the freezer).\n\nWhy a domain of its own: \"frost\" is the first four letters of \"frosting\", and a short key word\nmatches the start of a longer line word so that \"friend\" reaches \"friends\". That made the icing\nthe wrong kind of frost in three tables at once - the garden's frost cover answered \"how do i\nstore the frosting\", the freezer's defrost card answered \"can i keep the frosting in the fridge\",\nand the general cooked-food keeping table answered \"how long does frosting last in the fridge\".\nEach of those rows can be fenced in its own file, and each fence then has to be repeated for the\nnext row added beside it. Naming the icing as its own subject here is the one change that gives\nthe icing question an answer of its own, whichever of those rows is next to take the words.\n\nThe ask words are the keeping words and never the icing itself, so one row answers all of the\nphrasings: \"how long does the icing last\", \"can i keep the frosting in the fridge\", \"should\nbuttercream be kept in the fridge\", \"does the ganache freeze\", \"can i make the icing the day\nbefore\". The bag of icing sugar in the cupboard is not an icing on a cake, and sugar paste and\nroyal icing set rather than keep soft, so each has its own row or its own fence.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking-and-food")]));
    s_ICING(py_tuple(vec![py_str("frosting"), py_str("frostings"), py_str("icing"), py_str("icings"), py_str("buttercream"), py_str("butter cream"), py_str("ganache"), py_str("cream cheese icing"), py_str("cream cheese frosting"), py_str("glace icing")]));
    s_ICING_KEEP(py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("how long"), py_str("store"), py_str("stores"), py_str("stored"), py_str("storing"), py_str("freeze"), py_str("freezes"), py_str("frozen"), py_str("overnight"), py_str("room temperature"), py_str("ahead"), py_str("ahead of time"), py_str("day before"), py_str("night before"), py_str("make it the day before"), py_str("make ahead"), py_str("sit out"), py_str("sat out"), py_str("left out"), py_str("out overnight"), py_str("shelf life"), py_str("still ok"), py_str("still good"), py_str("gone off"), py_str("go off"), py_str("goes off")]));
    s__FROSTING_UP(stdlib_call("re", "compile", &[py_str("\\b(?:frosting|frosted|icing|iced) up\\b|\\bice building up\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_ICING(py_tuple(vec![py_str("icing sugar"), py_str("sugar paste"), py_str("frosting mix"), py_str("buttercream powder"), py_str("powdered sugar"), py_str("confectioners sugar"), py_str("fondant icing sugar")]));
    s_SET_ICING(py_tuple(vec![py_str("royal icing"), py_str("sugar paste"), py_str("writing icing"), py_str("piping icing"), py_str("ready to roll icing"), py_str("glace icing"), py_str("glacé icing"), py_str("fondant"), py_str("marzipan")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("icing-keeps")); __args.push(py_tuple(vec![g_ICING(), g_ICING_KEEP()])); __args.push(py_str("Buttercream or ganache keeps 3 to 4 days in a sealed tub in the fridge, or 2 months in the freezer: thaw it in the fridge overnight and beat it again, because it splits when it is cold. Cream cheese frosting is the short one at 2 to 3 days, and it does not freeze well. Keep an iced cake in a tin rather than the fridge, where it sweats.")); __kws.extend(vec![("avoid".to_string(), g_NOT_THE_ICING()), ("guard".to_string(), g__not_a_freezer_frosting_up()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("icing-that-sets")); __args.push(py_tuple(vec![g_SET_ICING(), g_ICING_KEEP()])); __args.push(py_str("Royal icing sets hard: keep it in an airtight tub at room temperature for about 2 weeks, or freeze it for up to 3 months, and cover the surface with cling film straight away because it starts to crust as soon as it meets the air. Sugar paste keeps 2 to 3 weeks in a sealed bag, and a drop of water is what revives a dry one. Neither belongs in the fridge - the damp makes both go sticky.")); __kws.extend(vec![("avoid".to_string(), g_NOT_THE_ICING()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _not_a_freezer_frosting_up(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("False when the line is a freezer frosting up rather than an icing being kept.");
    line = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    return Py::Bool(!py_truthy(&mcall(&g__FROSTING_UP(), "search", &[line.clone()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.icing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "ICING" => Some(g_ICING()),
            "ICING_KEEP" => Some(g_ICING_KEEP()),
            "NOT_THE_ICING" => Some(g_NOT_THE_ICING()),
            "SET_ICING" => Some(g_SET_ICING()),
            "_FROSTING_UP" => Some(g__FROSTING_UP()),
            "_not_a_freezer_frosting_up" => Some(py_func("_not_a_freezer_frosting_up", |args: &[Py]| _not_a_freezer_frosting_up(args))),
            _ => None,
        }),
        set: None,
    }))
}

#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
pub mod a_plain_noise_word_on_a_machine;
pub mod affording;
pub mod animals;
pub mod appointments;
pub mod babyfood;
pub mod bake_faults;
pub mod bikes;
pub mod body;
pub mod body_energy;
pub mod build_jobs;
pub mod business;
pub mod cakes;
pub mod caring;
pub mod cars;
pub mod child_work;
pub mod chilli;
pub mod cleaning;
pub mod clothing;
pub mod coats;
pub mod comeback;
pub mod complaints;
pub mod conditions;
pub mod consumer;
pub mod containers_frost;
pub mod cook_kit;
pub mod cooking;
pub mod cooking_entries;
pub mod cooking_fats;
pub mod cooking_gaps;
pub mod cooking_more;
pub mod cooking_tables;
pub mod cooling;
pub mod costs;
pub mod creative;
pub mod crisis_replies;
pub mod dates;
pub mod decorating;
pub mod desk_setup;
pub mod devices;
pub mod differences;
pub mod dish_verdicts;
pub mod dishes;
pub mod drinks;
pub mod eithers;
pub mod entertainment;
pub mod everyday_lines;
pub mod explains;
pub mod fabrics;
pub mod family;
pub mod family_more;
pub mod fears;
pub mod fitness;
pub mod fitweek;
pub mod food_left_out;
pub mod freshness;
pub mod friends;
pub mod fruit;
pub mod fun;
pub mod garden;
pub mod garden_tools;
pub mod geography;
pub mod habits;
pub mod health;
pub mod history;
pub mod home;
pub mod home_craft;
pub mod house_odds;
pub mod how_it_works;
pub mod icing;
pub mod idioms;
pub mod income_periods;
pub mod ingredient_swaps;
pub mod job_times;
pub mod joint_time;
pub mod joints;
pub mod keeping;
pub mod keeping_cooked;
pub mod kept_animals;
pub mod kit_lists;
pub mod kitchen;
pub mod kitchen_odds;
pub mod kitchen_prep;
pub mod kits_and_claims;
pub mod land;
pub mod language;
pub mod left_on;
pub mod lifeadmin;
pub mod lookup;
pub mod machine_door;
pub mod materials;
pub mod maths;
pub mod meals;
pub mod measures;
pub mod meat_in_a_card;
pub mod microwave_methods;
pub mod money;
pub mod money_words;
pub mod music;
pub mod nature;
pub mod newborn;
pub mod no_kit;
pub mod notable;
pub mod nursing;
pub mod nutrition;
pub mod occasions;
pub mod odours;
pub mod opinions;
pub mod painting_faults;
pub mod panic_talk;
pub mod parking;
pub mod pasta;
pub mod pay;
pub mod pensions;
pub mod pets;
pub mod pets_health;
pub mod pharmacy;
pub mod phone_use;
pub mod photography;
pub mod picking;
pub mod picks;
pub mod plans;
pub mod power_heat;
pub mod pregnancy;
pub mod prices;
pub mod procedures;
pub mod quantity;
pub mod rice;
pub mod routines;
pub mod rubbish;
pub mod sauces;
pub mod school_day;
pub mod school_days;
pub mod schoolmaths;
pub mod science;
pub mod seasoning;
pub mod shopping;
pub mod sleep;
pub mod sleep_aids;
pub mod spice_heat;
pub mod sport;
pub mod still_ok;
pub mod study;
pub mod swaps;
pub mod takes;
pub mod technology;
pub mod tenancy;
pub mod trades;
pub mod travel;
pub mod travel_health;
pub mod units;
pub mod vaccines;
pub mod vegetables;
pub mod warm_food;
pub mod water_out;
pub mod what_damages_what;
pub mod whenbuy;
pub mod wording;
pub mod work;

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


pub static TOPIC_ALIASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _INTERNAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _cache: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_TOPIC_ALIASES() -> Py { ensure_init(); TOPIC_ALIASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOPIC_ALIASES(v: Py) { ensure_init(); if let Ok(mut w) = TOPIC_ALIASES.write() { *w = v; } }
pub fn g__INTERNAL() -> Py { ensure_init(); _INTERNAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__INTERNAL(v: Py) { ensure_init(); if let Ok(mut w) = _INTERNAL.write() { *w = v; } }
pub fn g__cache() -> Py { ensure_init(); _cache.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__cache(v: Py) { ensure_init(); if let Ok(mut w) = _cache.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The knowledge base: facts about the world, kept as data. The answers lane owns it.\n\nOne module per domain (bot/knowledge/cooking.py, bot/knowledge/travel.py, ...) holds plain data\nkeyed by meaning (a dish, a city, a symptom), not by how a question is worded. A domain lists\nits knowledge as ENTRIES, built with bot.knowledge.lookup.entry(), so the shared matcher in\nbot/knowledge/lookup.py can answer every way of asking for it. Answer modules in bot/modules/\nfind the entry here and compose the reply. The bot cannot read files, so the data lives in .py\nfiles.\n\nAdding a domain is adding one file with ENTRIES; nothing else needs to change.\n\nA domain also says which topics it can answer, so the arbiter knows this is the specialist\nfor them: `ANSWERS = (\"cooking\", \"food-and-drink\")`, the same attribute name a reply module\ndeclares, with the topic ids from bot/understanding/topics.py. bot/modules/answers.py declares\nthe union as its own ANSWERS, so adding a domain never means editing a list somewhere else.\n\nThe shared tables are not domains: bot/knowledge/lookup.py matches keys to lines and\nbot/knowledge/units.py converts measures, both used by every domain, so they are kept out\nof the registry in _INTERNAL.\n");
    s__INTERNAL(py_set(vec![py_str("lookup"), py_str("units"), py_str("cooking_entries"), py_str("keeping_cooked"), py_str("crisis_replies"), py_str("home_craft"), py_str("quantity"), py_str("cooking_more"), py_str("bake_faults"), py_str("family_more"), py_str("cooking_tables")]));
    s_TOPIC_ALIASES(py_dict(vec![(py_str("home"), py_str("home-and-diy")), (py_str("travel"), py_str("geography-and-travel")), (py_str("nature"), py_str("nature-and-weather")), (py_str("garden"), py_str("garden-and-plants")), (py_str("food"), py_str("food-and-drink")), (py_str("food_and_drink"), py_str("food-and-drink")), (py_str("maths"), py_str("maths-and-numbers")), (py_str("school"), py_str("school-and-study")), (py_str("language"), py_str("language-and-writing"))]));
    s__cache(py_dict(vec![]));
}
pub fn domains(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cached: Py = Py::None;
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let _ = py_str("{domain name: module} for every knowledge domain in this package, by name.");
    cached = mcall(&g__cache(), "get", &[py_str("domains")]);
    if py_truthy(&Py::Bool(!py_is(&cached.clone(), &Py::None))) {
    return cached.clone();
    }
    found = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("pkgutil", "iter_modules", &[g___path__()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__INTERNAL(), &gattr(&item.clone(), "name")))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    let _ = py_setitem(&found.clone(), &gattr(&item.clone(), "name"), stdlib_call("importlib", "import_module", &[{ let mut __s = String::new(); __s.push_str(&fstr(&g___name__(), -1, "")); __s.push_str("."); __s.push_str(&fstr(&gattr(&item.clone(), "name"), -1, "")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    let _ = py_setitem(&g__cache(), &py_str("domains"), found.clone());
    return found.clone();
    Py::None
}
pub fn by_topic(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { Py::None };
    let mut cached: Py = Py::None;
    let mut declared: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut item: Py = Py::None;
    let mut module: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let mut remembered: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("{domain name: the topic ids its own entries answer}, so one domain can be judged.\n\n    Worked out exactly as topics() does it, one domain at a time. A reader that already has\n    an entry in hand uses this to ask whether that entry belongs in a conversation about a\n    given topic at all. `found` is domains(), for tests.\n    ");
    remembered = Py::Bool(py_is(&found.clone(), &Py::None));
    cached = if py_truthy(&remembered.clone()) { mcall(&g__cache(), "get", &[py_str("by_topic")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&cached.clone(), &Py::None))) {
    return cached.clone();
    }
    if py_truthy(&remembered.clone()) {
    found = domains(&[]);
    }
    out = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&found.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    module = __unpk2[1].clone();
    declared = call_builtin("getattr", &[module.clone(), py_str("ANSWERS"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[declared.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    declared = py_tuple(vec![declared.clone()]);
    }
    let __try3 = exc::catch(|| {
    declared = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&declared.clone()) { let mut item: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&mcall(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } } py_list(__out) })() };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["TypeError"]) {
                __handled7 = true;
    declared = py_list(vec![]);
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&Py::Bool(!py_truthy(&declared.clone()))) {
    flat = mcall(&mcall(&mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("_"), py_str("-")]);
    declared = py_list(vec![mcall(&g_TOPIC_ALIASES(), "get", &[flat.clone(), flat.clone()])]);
    }
    let _ = py_setitem(&out.clone(), &call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&declared.clone()) { let mut topic: Py = __item.clone(); if py_truthy(&topic.clone()) {  __out.push(topic.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    }
    if py_truthy(&remembered.clone()) {
    let _ = py_setitem(&g__cache(), &py_str("by_topic"), out.clone());
    }
    return out.clone();
    Py::None
}
pub fn topics(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { Py::None };
    let mut answered: Py = Py::None;
    let mut declared: Py = Py::None;
    let _ = py_str("The topic ids (bot/understanding/topics.py) this knowledge base can answer, sorted.\n\n    A domain's declared ANSWERS wins; otherwise its name stands for one topic, through\n    TOPIC_ALIASES when the name is not already an id. So a domain that forgets to declare\n    is still routed, and a new domain needs no edit here. `found` is domains(), for tests.\n    ");
    answered = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&by_topic(&[found.clone()]), "values", &[]));
    for __item in __iter1 {
    declared = __item.clone();
    let _ = mcall(&answered.clone(), "update", &[declared.clone()]);
    }
    let _ = mcall(&answered.clone(), "discard", &[py_str("")]);
    return call_builtin("tuple", &[call_builtin("sorted", &[answered.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _with_one_more_test(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut guard: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "guard") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: guard"))) };
    let mut extra: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "extra") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: extra"))) };
    let mut guard_and_extra: Py = Py::None;
    let _ = py_str("A row's guard with one more condition folded in, either side allowed to be missing.\n\n    A guard says whether the row may answer, so two conditions are an `and`: the row answers\n    only when both agree it should. The row keeps whatever guard it already had, so this cannot\n    take a test away, and a row with none simply takes the new one.\n    ");
    if py_truthy(&Py::Bool(py_is(&guard.clone(), &Py::None))) {
    return extra.clone();
    }
pub fn __nested_guard_and_extra_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut extra: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut guard: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut guard_and_extra: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = call_builtin("bool", &[py_call(&guard, &[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[py_call(&extra, &[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
    guard_and_extra = py_func("guard_and_extra", move |args: &[Py]| { let mut full = vec![extra.clone(), guard.clone(), guard_and_extra.clone()]; full.extend_from_slice(args); __nested_guard_and_extra_0(&full) });
    return guard_and_extra.clone();
    Py::None
}
pub fn entries(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut extra: Py = Py::None;
    let mut found: Py = Py::None;
    let mut guard_folds: Py = Py::None;
    let mut item: Py = Py::None;
    let mut module: Py = Py::None;
    let mut name: Py = Py::None;
    let mut stands_aside: Py = Py::None;
    let _ = py_str("Every Entry in every domain, in a stable order (cached after the first call).\n\n    Each entry carries the name of the domain it came from, so the matcher can weigh a key\n    word by how rare it is inside that domain rather than across the whole knowledge base:\n    a new domain then cannot take a point away from an entry somewhere else.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&g__cache(), &py_str("all")))) {
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[mcall(&domains(&[]), "items", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    module = __unpk2[1].clone();
    stands_aside = call_builtin("getattr", &[module.clone(), py_str("STANDS_ASIDE"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    guard_folds = { let __b = call_builtin("getattr", &[module.clone(), py_str("GUARD_FOLDS"), Py::None]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __iter3 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[module.clone(), py_str("ENTRIES"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    item = __item.clone();
    let __try4 = exc::catch(|| {
    let _ = py_setattr(&item.clone(), "domain", name.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_is(&stands_aside.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&call_builtin("getattr", &[item.clone(), py_str("guard"), Py::None]).unwrap_or_else(|e| exc::raise(e)), &Py::None)) } }) {
    let _ = py_setattr(&item.clone(), "guard", stands_aside.clone());
    }
    extra = mcall(&guard_folds.clone(), "get", &[call_builtin("getattr", &[item.clone(), py_str("id"), py_str("")]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_is(&extra.clone(), &Py::None))) {
    let _ = py_setattr(&item.clone(), "guard", _with_one_more_test(&[gattr(&item.clone(), "guard"), extra.clone()]));
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret5: Py = Py::Bool(false);
    let mut __val6: Py = Py::None;
    let mut __unhandled7: Option<exc::PyExc> = None;
    match __try4 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret5 = __p[0].clone(); __val6 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled8 = false;
            if !__handled8 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled8 = true;
            }
            if !__handled8 { __unhandled7 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled7 { exc::raise(__e); }
    if py_truthy(&__ret5) { return __val6; }
    let _ = mcall(&found.clone(), "append", &[item.clone()]);
    }
    }
    let _ = py_setitem(&g__cache(), &py_str("all"), found.clone());
    }
    return geti(&g__cache(), &py_str("all"));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "TOPIC_ALIASES" => Some(g_TOPIC_ALIASES()),
            "_INTERNAL" => Some(g__INTERNAL()),
            "_cache" => Some(g__cache()),
            "_with_one_more_test" => Some(py_func("_with_one_more_test", |args: &[Py]| _with_one_more_test(args))),
            "by_topic" => Some(py_func("by_topic", |args: &[Py]| by_topic(args))),
            "domains" => Some(py_func("domains", |args: &[Py]| domains(args))),
            "entries" => Some(py_func("entries", |args: &[Py]| entries(args))),
            "topics" => Some(py_func("topics", |args: &[Py]| topics(args))),
            _ => None,
        }),
        set: None,
    }))
}

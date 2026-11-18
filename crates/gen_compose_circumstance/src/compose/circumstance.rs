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


thread_local! { static G__AM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SITUATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.circumstance: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g__AM() -> Py { ensure_init(); G__AM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AM(v: Py) { ensure_init(); G__AM.with(|c| *c.borrow_mut() = v); }
pub fn g__ARE() -> Py { ensure_init(); G__ARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARE(v: Py) { ensure_init(); G__ARE.with(|c| *c.borrow_mut() = v); }
pub fn g__POOLS() -> Py { ensure_init(); G__POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POOLS(v: Py) { ensure_init(); G__POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g__SITUATIONS() -> Py { ensure_init(); G__SITUATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SITUATIONS(v: Py) { ensure_init(); G__SITUATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__TONE() -> Py { ensure_init(); G__TONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TONE(v: Py) { ensure_init(); G__TONE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A standing situation of theirs, said on its own, is where they are starting from.\n\nPeople open a conversation with a stranger with the fact that decides what help is worth giving:\n\"i rent\", \"im a carer\", \"im self employed\", \"im on benefits\", \"im disabled\", \"im divorced\".\nToday each of those is read as something it is not:\n\n- \"im a tenant\", \"im a carer\", \"im a single parent\" reach the role pool, which asks about it as\n  a job - \"Tenant - good to know. What do you like most about it?\";\n- \"im disabled\" and \"im self employed\" reach the small-day pool, which answers a day that went\n  by - \"Good to hear. What was the best bit?\" to somebody saying they are disabled;\n- \"im divorced\" is read as bad news - \"I'm sorry - that's frustrating\";\n- \"i rent\", \"im on benefits\" reach the receipt - \"Thanks for telling me\", which files the fact\n  and gives the person nothing.\n\nNone of those is a reply to the situation. What a friend does with it is take it in, say the one\nthing it changes, and ask the question that opens the next turn. That is what this module does,\nwith its own lines for each kind, so the reply is about their situation rather than any message.\n\nOnly the whole line counts, and only the simple forms of it. \"im a carer for my mum who has\ndementia\" is about her and keeps its own reading, as does \"i rent and the boiler is broken\",\nwhich is about the boiler.\n");
    s__AM(py_str("(?:i\\s*m|im|iam|i\\s+am)"));
    s__ARE(py_str("(?:we\\s*'?\\s*re|we\\s+are)"));
    s__TONE(stdlib_call("re", "compile", &[py_str("(?:\\s+(?:tbh|tho|though|now|anyway|actually|really|literally|honestly|i\\s+suppose|i\\s+guess|still|also|too|by\\s+the\\s+way|btw|atm|at\\s+the\\s+moment|lol|ha|haha|lmao|so\\s+yeah|yeah|innit|mate|dude))+$")]).unwrap_or_else(|e| exc::raise(e)));
    s__SITUATIONS(py_tuple(vec![py_tuple(vec![py_str("tenant"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str("|"); __s.push_str(&fstr(&g__ARE(), -1, "")); __s.push_str(")\\s+(?:a\\s+|an\\s+)?(?:private\\s+|council\\s+|housing\\s+association\\s+)?(?:tenant|tenants|renter|renters|lodger|lodgers)$|^(?:i|we)\\s+rent(?:ing)?(?:\\s+(?:a|an|my|our|the)\\s+(?:flat|house|home|place|bungalow|room|council\\s+flat|housing\\s+association\\s+flat))?$|^(?:i|we)\\s+pay\\s+rent$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Good to know - renting decides who has to act when something in the place goes wrong. What's on your mind?"), py_str("Right, so you rent. What are you trying to sort out?"), py_str("Renting, noted. What would you like a hand with?")])]), py_tuple(vec![py_str("carer"), stdlib_call("re", "compile", &[py_str("^(?:i\\s*m|im|iam|i\\s+am)\\s+(?:a\\s+|an\\s+)?(?:unpaid\\s+|full[- ]time\\s+|part[- ]time\\s+|young\\s+|family\\s+)?carer$|^(?:i\\s*m|im|iam|i\\s+am)\\s+(?:a\\s+)?carer\\s+for\\s+(?:a\\s+)?(?:relative|someone|somebody|family)$")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Carers carry a lot that nobody gets to see. Is it help for you you're after, or something for the person you look after?"), py_str("Good to know. What would help most - something for you, or something for them?"), py_str("Caring is a job with no clocking off. What's the hard bit at the moment?")])]), py_tuple(vec![py_str("single_parent"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:a\\s+|an\\s+)?(?:single|lone|solo)\\s+(?:parent|mum|mom|dad|mother|father|mam)$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("One pair of hands for all of it. What's the part that's hardest at the moment?"), py_str("Good to know - that's a lot to hold on your own. What are you trying to sort out?"), py_str("Right. What would help most this week?")])]), py_tuple(vec![py_str("student"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:a\\s+|an\\s+)?(?:(?:mature|full[- ]time|part[- ]time|postgrad|phd|undergrad|university|uni|college)\\s+)*student$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Student, then. What are you trying to work out?"), py_str("Good to know. What's the thing on your mind?"), py_str("Student life - what would help?")])]), py_tuple(vec![py_str("owner"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:a\\s+|an\\s+)?home\\s*owner$|^(?:i|we)\\s+own\\s+(?:my|our|the)\\s+(?:house|flat|home|place)$|^(?:i|we)\\s+own\\s+(?:it|mine|ours)$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Good to know - owning it means the repairs are yours to arrange. What's on your mind?"), py_str("So it's your own place. What are you trying to sort out?"), py_str("Home owner, noted. What would you like a hand with?")])]), py_tuple(vec![py_str("self_employed"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:a\\s+)?self[- ]?employed$|^(?:i|we)\\s+work\\s+for\\s+(?:myself|ourselves)$|^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:a\\s+)?(?:freelancer|contractor|sole\\s+trader)$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Self-employed, so no payroll to fall back on and nobody to ask upstairs. What are you trying to sort out?"), py_str("Working for yourself, then. What's on your mind?"), py_str("Good to know. What would help most?")])]), py_tuple(vec![py_str("benefits"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str("|"); __s.push_str(&fstr(&g__ARE(), -1, "")); __s.push_str(")\\s+on\\s+(?:benefits|universal\\s+credit|uc|tax\\s+credits|pip|esa|jsa|income\\s+support|housing\\s+benefit)$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Good to know - the rules on that are worth checking before you decide anything. What are you trying to work out?"), py_str("Right. What are you trying to sort out?"), py_str("Noted. What's the thing you need a hand with?")])]), py_tuple(vec![py_str("disabled"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:disabled|a\\s+disabled\\s+person)$|^(?:i|we)\\s+have\\s+(?:a\\s+)?disabilit(?:y|ies)$|^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:registered\\s+)?(?:blind|deaf|partially\\s+sighted)$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Good to know, and I won't assume what it means for you. What do you need a hand with?"), py_str("Thanks for telling me - it's useful to know rather than guess. What would help most?"), py_str("Right, noted. What's on your mind?")])]), py_tuple(vec![py_str("separated"), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__AM(), -1, "")); __s.push_str(")\\s+(?:divorced|separated|a\\s+divorcee|newly\\s+single|recently\\s+single)$|^(?:i|we)\\s+got\\s+divorced$"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Right - that's a lot to carry, and a pile of admin along with it. What needs sorting first?"), py_str("Good to know. What's on your mind about it?"), py_str("Noted. What would you like a hand with?")])])]));
    s__POOLS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g__SITUATIONS()) { let __unpk3545 = py_unpack(&__item.clone(), 3, None); let mut kind: Py = __unpk3545[0].clone(); let mut _pattern: Py = __unpk3545[1].clone(); let mut lines: Py = __unpk3545[2].clone(); __out.push((kind.clone(), lines.clone())); } py_dict(__out) })() });
}
pub fn _line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut said: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let _ = py_str("The line as this reader wants it: lower case, no edge punctuation, no tone on the end.");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "strip", &[]), "lower", &[]);
    said = mcall(&said.clone(), "strip", &[py_str(" .,!;:-\"'")]);
    said = stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&Py::Bool(true)) {
    trimmed = mcall(&g__TONE(), "sub", &[py_str(""), said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &said.clone()))) {
    return said.clone();
    }
    said = trimmed.clone();
    }
    Py::None
}
pub fn reads_as_situation(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut _lines: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The kind of situation the whole line is, or None.\n\n    Whole lines only: a line that goes on to say something else (\"i rent and the boiler is\n    broken\", \"im a carer for my mum who has dementia\") is about that something else.\n    ");
    said = _line(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__SITUATIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    _lines = __unpk2[2].clone();
    if py_truthy(&mcall(&pattern.clone(), "match", &[said.clone()])) {
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut kind: Py = Py::None;
    let _ = py_str("The lines that answer this situation, or None when the line is not one.");
    kind = reads_as_situation(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    return geti(&g__POOLS(), &kind.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.circumstance".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_AM" => Some(g__AM()),
            "_ARE" => Some(g__ARE()),
            "_POOLS" => Some(g__POOLS()),
            "_SITUATIONS" => Some(g__SITUATIONS()),
            "_TONE" => Some(g__TONE()),
            "_line" => Some(py_func("_line", |args: &[Py]| _line(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads_as_situation" => Some(py_func("reads_as_situation", |args: &[Py]| reads_as_situation(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_AM" => s__AM(v),
            "_ARE" => s__ARE(v),
            "_POOLS" => s__POOLS(v),
            "_SITUATIONS" => s__SITUATIONS(v),
            "_TONE" => s__TONE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.circumstance", module_obj);
    pyrt::xrefs::reg_fn("compose.circumstance", "_line", py_func("_line", |args: &[Py]| _line(args)));
    pyrt::xrefs::reg_fn("compose.circumstance", "reads_as_situation", py_func("reads_as_situation", |args: &[Py]| reads_as_situation(args)));
    pyrt::xrefs::reg_fn("compose.circumstance", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_access("compose.circumstance", "_AM", g__AM, s__AM);
    pyrt::xrefs::reg_access("compose.circumstance", "_ARE", g__ARE, s__ARE);
    pyrt::xrefs::reg_access("compose.circumstance", "_POOLS", g__POOLS, s__POOLS);
    pyrt::xrefs::reg_access("compose.circumstance", "_SITUATIONS", g__SITUATIONS, s__SITUATIONS);
    pyrt::xrefs::reg_access("compose.circumstance", "_TONE", g__TONE, s__TONE);
}

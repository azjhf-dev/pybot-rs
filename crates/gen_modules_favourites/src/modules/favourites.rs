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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_read_tastes: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.favourites: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_read_tastes() -> Py { ensure_init(); G_read_tastes.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_read_tastes(v: Py) { ensure_init(); G_read_tastes.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A favourite the person names, and the ask for it a turn later.\n\nA favourite is a fact about the person and nothing in the knowledge base knows it: \"my favourite\nfood is curry\" was kept as a taste (and became a dish), \"my favourite band is blur\" reached the\nentry for \"band\" as an object, \"what is my favourite genre\" reached the entry for genes, and\n\"what is my favourite colour\" the paint card. The slot and its wording are in\n`bot/memory/favourite.py`; this module is the store's face for them.\n\n`observe` keeps one: a colour, a film, a food, a team, a genre, whatever they name, plus the\nchange said out loud (\"i prefer blue now\") and the correction (\"no wait its blade runner\").\n`respond` says it back - the receipt for the line that said it, the one thing they named for the\nask - and leaves the turn alone whenever something better answers it: a line that states a\nfavourite and asks something else in the same breath, and a favourite of a thing the knowledge\nbase has an entry for, which is answered about the thing rather than as a receipt.\n\nThe whole of it is in one module on purpose: a favourite is nobody else's business, and the store\nis a big file every lane edits.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_read_tastes(py_func("read_all", |args: &[Py]| pyrt::xrefs::xcall("memory.taste", "read_all", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.favourites.PRIORITY"); }
    s_PRIORITY(Py::Int(7000));
}
pub fn observe(args: &[Py]) -> Py {
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
    let _ = py_str("Keep the favourite a line names, corrects or changes (bot/memory/favourite.py).");
    let __try1 = exc::catch(|| {
    let _ = pyrt::xrefs::xcall("memory.favourite", "watch", &[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("Whether this line is one this module answers as it comes (bot/modules/arbiter.py).\n\n    The arbiter settles a specialist against a generalist by the topic the line fits, and the ask\n    for a favourite fits one: \"what is my favourite genre\" is read as an opinions line, so the\n    entry that owns novels and television was given the turn over the one thing the person had\n    already said (\"A gene is a section of DNA...\", for a favourite thing they had named a line\n    earlier). A favourite is what the person said about themselves, the same claim `memory_store`\n    makes for a name or a plan, and it is answered as it comes for the same reason.\n\n    Asked on a shallow copy of memory, because answering pops the flags the reply was marked\n    with - the live memory is the store's to write, not this hook's.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&_said_back(&[text.clone(), call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e))]), &Py::None))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let _ = py_str("Say the favourite back, or leave the turn to whoever answers the line better.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), _said_back(&[text.clone(), memory.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _said_back(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last: Py = Py::None;
    let mut options: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_all: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut told: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The favourite a line names or asks for, said back (bot/memory/favourite.py).\n\n    \"my favourite food is curry\" is a fact about the person and nothing in the knowledge base\n    knows it, so the store keeps it. The ask comes back as the one thing they named - \"Your\n    favourite colour is green.\" - because the reply that repeated the last paragraph (\"Same as\n    before: White or cream lets green breathe...\") answers a question they did not ask. A thing\n    they have never said a favourite of is left to the other modules rather than denied.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    told = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "pop", &[py_str("favourite_told"), Py::None]) } else { Py::None };
    changed = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "pop", &[py_str("favourite_changed"), Py::None]) } else { Py::None };
    if py_truthy(&pyrt::xrefs::xcall("memory.favourite", "asks_something_too", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[changed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&changed.clone(), "get", &[py_str("thing")]) } }) {
    shown = pyrt::xrefs::xcall("memory.favourite", "their_words", &[mcall(&changed.clone(), "get", &[py_str("value")])]);
    shown = op_add(&mcall(&slice_or_raise(&shown.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&shown.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&mcall(&changed.clone(), "get", &[py_str("was")])) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&op_add(&op_add(&op_add(&shown.clone(), &py_str(" now, then — I have your favourite ")), &call_builtin("str", &[geti(&changed.clone(), &py_str("thing"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(" as ")), &pyrt::xrefs::xcall("memory.favourite", "their_words", &[mcall(&changed.clone(), "get", &[py_str("value")])])), &py_str(".")), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Changed: your favourite "), &call_builtin("str", &[geti(&changed.clone(), &py_str("thing"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(" is ")), &pyrt::xrefs::xcall("memory.favourite", "their_words", &[mcall(&changed.clone(), "get", &[py_str("value")])])), &py_str(" now, not ")), &pyrt::xrefs::xcall("memory.favourite", "their_words", &[mcall(&changed.clone(), "get", &[py_str("was")])])), &py_str("."))]), memory.clone(), text.clone()]);
    }
    return op_add(&op_add(&op_add(&shown.clone(), &py_str(" for your favourite ")), &call_builtin("str", &[geti(&changed.clone(), &py_str("thing"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(", then — noted."));
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[told.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&told.clone(), "get", &[py_str("thing")]) } }) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Noted: your favourite "), &call_builtin("str", &[geti(&told.clone(), &py_str("thing"))]).unwrap_or_else(|e| exc::raise(e))), &py_str(" is ")), &pyrt::xrefs::xcall("memory.favourite", "their_words", &[mcall(&told.clone(), "get", &[py_str("value")])])), &py_str("."));
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.favourite", "asks_all", &[text.clone()])) {
    held = pyrt::xrefs::xcall("memory.favourite", "the_favourites", &[memory.clone()]);
    if py_truthy(&held.clone()) {
    last = slice_or_raise(&held.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None);
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&last.clone()) { let __unpk5590 = py_unpack(&__item.clone(), 2, None); let mut thing: Py = __unpk5590[0].clone(); let mut value: Py = __unpk5590[1].clone(); __out.push(op_add(&op_add(&op_add(&py_str("your favourite "), &call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" is ")), &call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    said_all = op_add(&op_add(&py_str("You have told me one: "), &geti(&parts.clone(), &Py::Int(0))), &py_str("."));
    } else {
    said_all = op_add(&op_add(&op_add(&op_add(&py_str("You have told me "), &mcall(&py_str(", "), "join", &[slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])), &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1)))), &py_str("."));
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![said_all.clone(), mcall(&said_all.clone(), "replace", &[py_str("You have told me "), py_str("Here is what I have: ")])]), memory.clone(), text.clone()]);
    }
    return Py::None;
    }
    asked = pyrt::xrefs::xcall("memory.favourite", "asked", &[text.clone()]);
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.favourite", "kept", &[memory.clone(), asked.clone()]))); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.taste", "read_all", &[memory.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&asked.clone()) {
    value = pyrt::xrefs::xcall("memory.favourite", "kept", &[memory.clone(), asked.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    let _ = py_setitem(&memory.clone(), &py_str("favourite_asked"), asked.clone());
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You haven't told me your favourite "), &asked.clone()), &py_str(" — what is it?")), op_add(&op_add(&py_str("I don't have a favourite "), &asked.clone()), &py_str(" for you yet. What is it?")), op_add(&op_add(&py_str("Your favourite "), &asked.clone()), &py_str(" hasn't come up yet. What is it?"))]), memory.clone(), text.clone()]);
    }
    value = pyrt::xrefs::xcall("memory.favourite", "their_words", &[value.clone()]);
    shown = op_add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None));
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("Your favourite "), &asked.clone()), &py_str(" is ")), &value.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("You said "), &value.clone()), &py_str(" — that is your favourite ")), &asked.clone()), &py_str(".")), op_add(&op_add(&op_add(&shown.clone(), &py_str(", you said. That is your favourite ")), &asked.clone()), &py_str("."))]), memory.clone(), text.clone()]);
    }
    said = pyrt::xrefs::xcall("memory.favourite", "read", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("understood")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_is(&pyrt::xrefs::xcall("knowledge.lookup", "find", &[geti(&memory.clone(), &py_str("understood"))]), &Py::None))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    value = pyrt::xrefs::xcall("memory.favourite", "their_words", &[geti(&said.clone(), &py_str("value"))]);
    options = py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("Noted: your favourite "), &geti(&said.clone(), &py_str("thing"))), &py_str(" is ")), &value.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("Favourite "), &geti(&said.clone(), &py_str("thing"))), &py_str(": ")), &value.clone()), &py_str(". Got it."))]);
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[mcall(&value.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&value.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("my "), py_str("the ")])]))) } }) {
    let _ = mcall(&options.clone(), "append", &[op_add(&op_add(&op_add(&op_add(&mcall(&slice_or_raise(&value.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" for your favourite ")), &geti(&said.clone(), &py_str("thing"))), &py_str(", then — noted."))]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.favourites".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_said_back" => Some(py_func("_said_back", |args: &[Py]| _said_back(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "pick" => Some(g_pick()),
            "read_tastes" => Some(g_read_tastes()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "lookup" => Some(pyrt::xrefs::xmodule("knowledge.lookup")),
            "favourite" => Some(pyrt::xrefs::xmodule("memory.favourite")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "pick" => s_pick(v),
            "read_tastes" => s_read_tastes(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.favourites", module_obj);
    pyrt::xrefs::reg_fn("modules.favourites", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.favourites", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.favourites", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.favourites", "_said_back", py_func("_said_back", |args: &[Py]| _said_back(args)));
    pyrt::xrefs::reg_access("modules.favourites", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.favourites", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("modules.favourites", "read_tastes", g_read_tastes, s_read_tastes);
}

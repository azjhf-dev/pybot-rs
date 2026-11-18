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


thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_SOMEBODY() -> Py { ensure_init(); G_NAMED_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_SOMEBODY(v: Py) { ensure_init(); G_NAMED_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_BODY() -> Py { ensure_init(); G_OWN_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_BODY(v: Py) { ensure_init(); G_OWN_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET() -> Py { ensure_init(); G_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET(v: Py) { ensure_init(); G_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_BODY() -> Py { ensure_init(); G_PRONOUN_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_BODY(v: Py) { ensure_init(); G_PRONOUN_BODY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whose body a line is about when it says \"he\", \"she\" or \"they\" and no name.\n\n\"my dad is 70\" then \"hes got a bad hip\" is one person's trouble told over two turns, and the\nsecond turn names nobody: the \"he\" is the dad the chat has just named. Read as the speaker's own\nbody it got \"Sorry to hear it - go easy on yourself today, and see someone if it's not shifting in\na few days\", which is advice for a body that is not the one that hurts, and it tells the person\ntyping to see someone about their father's hip.\n\nSo this answers one question only - is a body on this line somebody else's? - for the composer's\ncarer reading to use. It needs both halves: a third-person pronoun carrying the body, and somebody\nnamed in the chat within the last few turns. \"i told my dad about my back\" says \"my\", so the body\nis the speaker's and this says nothing.\n");
    s_KIN(py_str("(?:dad|daddy|father|mum|mummy|mother|mam|nan|nanna|gran|granny|grandma|grandad|grandpa|granddad|son|daughter|child|kid|boy|girl|baby|little one|husband|wife|partner|other half|boyfriend|girlfriend|brother|sister|niece|nephew|cousin|aunt|uncle|friend|mate|neighbour|carer|flatmate|housemate|in laws|in-law)"));
    s_NAMED_SOMEBODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __s = String::new(); __s.push_str("\\b(?:my|our|his|her|their)\\s+(?:poor\\s+|little\\s+|eldest\\s+|youngest\\s+|old\\s+|older\\s+|elderly\\s+|grown\\s*up\\s+)?"); __s.push_str(&fstr(&g_KIN(), -1, "")); __s.push_str("\\b"); py_string(__s) }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRONOUN_BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|hed|shed|theyd)(?:'s|s|\\s+is|\\s+has|\\s+had|\\s+got|\\s+has\\s+got|\\s+keeps\\s+getting|\\s+was|\\s+been)\\b[^.!?]{0,26}?\\b(?:bad|sore|dodgy|gammy|weak|dicky|knackered|broken|cracked|twisted|swollen|stiff|aching|hurting|hurts?|soreness|pain|stiffness|giving\\s+him\\s+trouble)\\b|\\b(?:his|her|their)\\s+(?:[a-z]+\\s+){0,2}(?:hip|knee|back|shoulder|leg|arm|foot|feet|hand|wrist|ankle|neck|stomach|chest|head|eye|ear|joints?)\\b[^.!?]{0,18}?\\b(?:is|are|feels?|has\\s+been|keeps)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:[a-z]+\\s+){0,2}(?:hip|knee|back|shoulder|leg|arm|foot|feet|hand|wrist|ankle|neck|stomach|chest|head|eye|ear|joints?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dog|dogs|puppy|pup|cat|cats|kitten|rabbit|bunny|hamster|guinea\\s+pig|horse|pony|budgie|parrot|bird|fish|tortoise|snake|lizard|ferret|gerbil)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn somebody_was_named(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the chat has named somebody the person looks after, in the last few turns.");
    let __try1 = exc::catch(|| {
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter6 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter6 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_SOMEBODY(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_body_of_a_pronoun(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line hangs a body or a hurt on he/she/they rather than on \"my\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_BODY(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRONOUN_BODY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn somebody_elses_body(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("True when this line is somebody else's body, named earlier in the chat.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_body_of_a_pronoun(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| somebody_was_named(args))(&__args) } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.whose_body".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "KIN" => Some(g_KIN()),
            "NAMED_SOMEBODY" => Some(g_NAMED_SOMEBODY()),
            "OWN_BODY" => Some(g_OWN_BODY()),
            "PET" => Some(g_PET()),
            "PRONOUN_BODY" => Some(g_PRONOUN_BODY()),
            "a_body_of_a_pronoun" => Some(py_func("a_body_of_a_pronoun", |args: &[Py]| a_body_of_a_pronoun(args))),
            "somebody_elses_body" => Some(py_func("somebody_elses_body", |args: &[Py]| somebody_elses_body(args))),
            "somebody_was_named" => Some(py_func("somebody_was_named", |args: &[Py]| somebody_was_named(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "KIN" => s_KIN(v),
            "NAMED_SOMEBODY" => s_NAMED_SOMEBODY(v),
            "OWN_BODY" => s_OWN_BODY(v),
            "PET" => s_PET(v),
            "PRONOUN_BODY" => s_PRONOUN_BODY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.whose_body", module_obj);
    pyrt::xrefs::reg_fn("memory.whose_body", "somebody_was_named", py_func("somebody_was_named", |args: &[Py]| somebody_was_named(args)));
    pyrt::xrefs::reg_fn("memory.whose_body", "a_body_of_a_pronoun", py_func("a_body_of_a_pronoun", |args: &[Py]| a_body_of_a_pronoun(args)));
    pyrt::xrefs::reg_fn("memory.whose_body", "somebody_elses_body", py_func("somebody_elses_body", |args: &[Py]| somebody_elses_body(args)));
    pyrt::xrefs::reg_access("memory.whose_body", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("memory.whose_body", "NAMED_SOMEBODY", g_NAMED_SOMEBODY, s_NAMED_SOMEBODY);
    pyrt::xrefs::reg_access("memory.whose_body", "OWN_BODY", g_OWN_BODY, s_OWN_BODY);
    pyrt::xrefs::reg_access("memory.whose_body", "PET", g_PET, s_PET);
    pyrt::xrefs::reg_access("memory.whose_body", "PRONOUN_BODY", g_PRONOUN_BODY, s_PRONOUN_BODY);
}

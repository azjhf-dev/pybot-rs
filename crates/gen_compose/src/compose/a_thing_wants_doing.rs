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


thread_local! { static G_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_WANTS_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THING_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_FEELING() -> Py { ensure_init(); G_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEELING(v: Py) { ensure_init(); G_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE() -> Py { ensure_init(); G_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE(v: Py) { ensure_init(); G_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_WANTS_DOING() -> Py { ensure_init(); G_A_THING_WANTS_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_WANTS_DOING(v: Py) { ensure_init(); G_A_THING_WANTS_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_PARTS() -> Py { ensure_init(); G_BODY_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); G_BODY_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THING_IS() -> Py { ensure_init(); G_THE_THING_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THING_IS(v: Py) { ensure_init(); G_THE_THING_IS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing of theirs in a state that wants doing, and nobody in the line.\n\n\"the grass is long\", \"the car is dirty\", \"the windows are filthy\", \"my shoes are worn out\" and\n\"my coat is tatty\" were all answered with the composer's receipt - \"Thanks for telling me - I'm\nhere if you want to dig into any of that\" - which says nothing back about the thing the person has\njust handed over and asks nothing about it either. A friend looks at the thing with them: how bad\nit is, whether it is a quick job, where they would start.\n\nThe reading is the shape `my/our/the <thing> is <state>`, where the state is one of the adjectives\nthat says the thing wants doing rather than that it has gone wrong (`trouble` has the gone-wrong\nones: broken, cracked, leaking). It stands down for a body part, for a person, and for anything the\nline says the person feels - those have answers of their own - and it is asked only on a receipt,\nso a card, an entry or any other reply keeps the turn.\n");
    s_BODY_PARTS(crate::compose::fallback_data::g_BODY_PARTS());
    s_A_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("long"), py_str("overgrown"), py_str("dirty"), py_str("filthy"), py_str("grimy"), py_str("mucky"), py_str("tatty"), py_str("shabby"), py_str("scruffy"), py_str("worn out"), py_str("worn"), py_str("messy"), py_str("untidy"), py_str("dusty"), py_str("faded"), py_str("rusty"), py_str("grim"), py_str("cluttered"), py_str("piled up"), py_str("manky"), py_str("minging"), py_str("grubby")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nmum mom mam dad father mother son daughter child children kid kids baby toddler boy girl husband\nwife partner boyfriend girlfriend brother sister mate mates friend friends nan granny grandma\ngrandad grandpa aunt uncle cousin neighbour neighbor boss colleague client cat dog puppy kitten\npuppies rabbit hamster horse hens chickens bird birds fish\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FEELING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nsick tired fed up had enough annoyed angry cross sad upset stressed worried depressed\nembarrassed ashamed guilty lonely done exhausted shattered\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_THING_IS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the|my|our|your|his|her|their)\\s+(?P<thing>[a-z]+(?:\\s+[a-z]+)?)\\s+(?:is|are|looks|seems|gets|got)\\s+(?:a bit |a little |really |very |so |quite |proper |right |pretty |well )?(?P<state>worn out|piled up|[a-z]+)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_WANTS_DOING(py_tuple(vec![py_str("How bad is it — something you can do in an hour, or has it got away from you?"), py_str("What's the job on it? I'll work the next step out with you."), py_str("Tell me how bad it is and I'll work out where to start."), py_str("Right. How long has it been like that?"), py_str("Where would you start with it?")]));
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut found: Py = Py::None;
    let mut state: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the line hands over, when it is in a state that wants doing.");
    flat = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&flat.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&flat.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&flat.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_FEELING(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(flat.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_THING_IS(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("state")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }]), 2, None);
    state = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_A_STATE(), &state.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_A_PERSON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::compose::fallback_data::g_BODY_PARTS(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    return thing.clone();
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads(args))(&__args) }))) {
    return Py::None;
    }
    return g_A_THING_WANTS_DOING();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_thing_wants_doing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_FEELING" => Some(g_A_FEELING()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_STATE" => Some(g_A_STATE()),
            "A_THING_WANTS_DOING" => Some(g_A_THING_WANTS_DOING()),
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "THE_THING_IS" => Some(g_THE_THING_IS()),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_FEELING" => s_A_FEELING(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_STATE" => s_A_STATE(v),
            "A_THING_WANTS_DOING" => s_A_THING_WANTS_DOING(v),
            "BODY_PARTS" => s_BODY_PARTS(v),
            "THE_THING_IS" => s_THE_THING_IS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_thing_wants_doing", module_obj);
    pyrt::xrefs::reg_fn("compose.a_thing_wants_doing", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_wants_doing", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "A_FEELING", g_A_FEELING, s_A_FEELING);
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "A_STATE", g_A_STATE, s_A_STATE);
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "A_THING_WANTS_DOING", g_A_THING_WANTS_DOING, s_A_THING_WANTS_DOING);
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "BODY_PARTS", g_BODY_PARTS, s_BODY_PARTS);
    pyrt::xrefs::reg_access("compose.a_thing_wants_doing", "THE_THING_IS", g_THE_THING_IS, s_THE_THING_IS);
}

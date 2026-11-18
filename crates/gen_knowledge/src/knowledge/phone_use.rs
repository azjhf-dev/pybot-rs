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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BUYING_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CALLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_USE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHONE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHOTOS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BUYING_QUESTION() -> Py { ensure_init(); G__A_BUYING_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BUYING_QUESTION(v: Py) { ensure_init(); G__A_BUYING_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__CALLS() -> Py { ensure_init(); G__CALLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CALLS(v: Py) { ensure_init(); G__CALLS.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAVY() -> Py { ensure_init(); G__HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAVY(v: Py) { ensure_init(); G__HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_USE() -> Py { ensure_init(); G__NOT_A_USE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_USE(v: Py) { ensure_init(); G__NOT_A_USE.with(|c| *c.borrow_mut() = v); }
pub fn g__PHONE_THING() -> Py { ensure_init(); G__PHONE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHONE_THING(v: Py) { ensure_init(); G__PHONE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PHOTOS() -> Py { ensure_init(); G__PHOTOS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHOTOS(v: Py) { ensure_init(); G__PHOTOS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A phone bought for what the person does with it.\n\n\"i need a new phone\" and then \"i only use it for photos and calls\" is one buying question, and the\nuse is the half of it that decides the answer: photos want storage and a decent camera, calls and\ntexts want the cheapest handset that still gets security updates. The transfer row in\n`technology.py` keys \"new phone\" and \"photos\", so a use statement reached it and a person who had\nnot chosen a phone yet was told how to copy their data onto one.\n\nThis is its own file rather than rows in `shopping.py` and `technology.py` because those two are\nlong and busy: a domain is one file with ENTRIES in it, and a use statement about a phone is not\nthe only row either of them needs. The key carries the use as a phrase (\"use it for\", \"only use\nit for\") so it scores as the phrase it is against the transfer row's single word \"photos\", and\nthe guard below keeps it out of every line that is a move, a fault or a phone already in hand.\n");
    s_ANSWERS(py_tuple(vec![py_str("technology")]));
    s__PHOTOS(stdlib_call("re", "compile", &[py_str("\\b(?:foto|fotos|photo|photos|picture|pictures|pics|camera|video|videos|instagram|tiktok|youtube)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CALLS(stdlib_call("re", "compile", &[py_str("\\b(?:call|calls|calling|text|texts|texting|message|messages|messaging|ring|ringing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PHONE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:phone|phones|iphone|iphones|android|handset|mobiles?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEAVY(stdlib_call("re", "compile", &[py_str("\\b(?:game|games|gaming|work|email|emails|spreadsheet|spreadsheets|editing|design)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_USE(stdlib_call("re", "compile", &[py_str("\\b(?:transfer|moved|move|migrat|sync|backed up|back up|set up|setup|restore)\\b|\\b(?:battery|batteries|charg\\w*|screen|cracked|crack|broken|repair|fix|water|wet|hot|slow|full|storage|wifi|wi-fi|signal|sim|contract|bill|tariff|stolen|lost|wont|won't|doesn't|not working)\\b|\\b(?:got a new phone|got a phone|have a new phone|have a phone|already got|already have|just got|just bought|i bought|ive bought|how do i|how to|how can i)\\b|\\b(?:my child|my son|my daughter|my kid|my kids|my teen|my teenager|for a child|for children|for my son|for my daughter|for a kid|for a 1[0-6] year old|grandson|granddaughter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BUYING_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|needed|want|wants|looking|after|get|getting|buy|buying|bought|should|which|what|whats|recommend|suggest|advice|worth|budget|cheap|cheapest|best|good|better|new|old|oldest|upgrade|replace|replacing|broke|broken|dying|dead|knackered|ancient)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-phone-for-that-use")); __args.push(py_tuple(vec![py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("android"), py_str("handset"), py_str("mobile")]), py_tuple(vec![py_str("use it for"), py_str("use my phone for"), py_str("use a phone for"), py_str("used for"), py_str("using it for"), py_str("only use"), py_str("just use"), py_str("mostly use"), py_str("mainly use"), py_str("only need"), py_str("just need"), py_str("mostly for"), py_str("mainly for"), py_str("only for"), py_str("just for"), py_str("it is for"), py_str("its for"), py_str("for photos"), py_str("for pictures"), py_str("for calls"), py_str("for texts"), py_str("for messages"), py_str("for games"), py_str("for emails")])])); __args.push(g__phone_for_that_use()); __kws.extend(vec![("bias".to_string(), Py::Int(6)), ("ask".to_string(), py_tuple(vec![py_str("use it for"), py_str("use my phone for"), py_str("use a phone for"), py_str("using it for"), py_str("used for"), py_str("only use"), py_str("just use"), py_str("mostly use"), py_str("mainly use"), py_str("only need"), py_str("just need"), py_str("mostly for"), py_str("mainly for"), py_str("only for"), py_str("just for"), py_str("it is for"), py_str("its for"), py_str("for photos"), py_str("for pictures"), py_str("for calls"), py_str("for texts"), py_str("for messages"), py_str("for games"), py_str("for emails")])), ("guard".to_string(), g__a_use())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_use(args: &[Py]) -> Py {
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
    let _ = py_str("The line as text: the reading carries it, and an entry only gets the dict.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _phone_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    return call_builtin("bool", &[mcall(&g__PHONE_THING(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_use(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says what the phone is used for, rather than how to move or fix one.");
    line = _the_use(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__NOT_A_USE(), "search", &[line.clone()])));
    Py::None
}
pub fn _phone_for_that_use(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut before: Py = Py::None;
    let mut calls: Py = Py::None;
    let mut heavy: Py = Py::None;
    let mut line: Py = Py::None;
    let mut photos: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("The handset for what the line says the phone is used for, or None when it says no use.");
    line = mcall(&_the_use(&[understood.clone()]), "lower", &[]);
    before = py_str("");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    topic = { let __b = mcall(&memory.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    before = call_builtin("str", &[{ let __b = mcall(&topic.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&topic.clone(), "get", &[py_str("of")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_phone_words(&[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_phone_words(&[before.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_BUYING_QUESTION(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_BUYING_QUESTION(), "search", &[before.clone()]) } }))) {
    return Py::None;
    }
    photos = call_builtin("bool", &[mcall(&g__PHOTOS(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    calls = call_builtin("bool", &[mcall(&g__CALLS(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    heavy = call_builtin("bool", &[mcall(&g__HEAVY(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = photos.clone(); if py_truthy(&__b) { __b } else { { let __b = calls.clone(); if py_truthy(&__b) { __b } else { heavy.clone() } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = photos.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&heavy.clone())) } }) {
    return py_str("Photos are the part worth paying for: a mid-range phone at £200 to £300 takes pictures as good as a flagship did a few years ago, so put the extra into storage - 128GB at least, 256GB if you keep videos - and let cloud backup at a few pounds a month carry the rest.");
    }
    if py_truthy(&{ let __b = calls.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&photos.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&heavy.clone())) } } } }) {
    return py_str("Calls and texts need nothing special, so buy the cheapest phone that still gets security updates: about £120 to £180 new, less refurbished. Check the maker promises four or five years of updates and that the battery is 4,500mAh or more, and ignore how many cameras it has.");
    }
    if py_truthy(&{ let __b = heavy.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&photos.clone())) } }) {
    return py_str("For games or work on a phone, spend on the processor and the screen: £300 to £450 buys a mid-range chip that will not stutter, and 256GB of storage, because games take a lot of room. A two-year-old flagship at the same money is the other good buy if the battery health is over 85%.");
    }
    return py_str("For photos and everything else, £200 to £300 buys a mid-range phone that covers calls, photos, maps and video, and 128GB of storage matters more than the camera. Buy it outright rather than on a contract if you can, because the contract usually costs more over two years.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.phone_use".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_BUYING_QUESTION" => Some(g__A_BUYING_QUESTION()),
            "_CALLS" => Some(g__CALLS()),
            "_HEAVY" => Some(g__HEAVY()),
            "_NOT_A_USE" => Some(g__NOT_A_USE()),
            "_PHONE_THING" => Some(g__PHONE_THING()),
            "_PHOTOS" => Some(g__PHOTOS()),
            "_a_use" => Some(py_func("_a_use", |args: &[Py]| _a_use(args))),
            "_phone_for_that_use" => Some(py_func("_phone_for_that_use", |args: &[Py]| _phone_for_that_use(args))),
            "_phone_words" => Some(py_func("_phone_words", |args: &[Py]| _phone_words(args))),
            "_the_use" => Some(py_func("_the_use", |args: &[Py]| _the_use(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_A_BUYING_QUESTION" => s__A_BUYING_QUESTION(v),
            "_CALLS" => s__CALLS(v),
            "_HEAVY" => s__HEAVY(v),
            "_NOT_A_USE" => s__NOT_A_USE(v),
            "_PHONE_THING" => s__PHONE_THING(v),
            "_PHOTOS" => s__PHOTOS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.phone_use", module_obj);
    pyrt::xrefs::reg_fn("knowledge.phone_use", "_the_use", py_func("_the_use", |args: &[Py]| _the_use(args)));
    pyrt::xrefs::reg_fn("knowledge.phone_use", "_phone_words", py_func("_phone_words", |args: &[Py]| _phone_words(args)));
    pyrt::xrefs::reg_fn("knowledge.phone_use", "_a_use", py_func("_a_use", |args: &[Py]| _a_use(args)));
    pyrt::xrefs::reg_fn("knowledge.phone_use", "_phone_for_that_use", py_func("_phone_for_that_use", |args: &[Py]| _phone_for_that_use(args)));
    pyrt::xrefs::reg_access("knowledge.phone_use", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.phone_use", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_A_BUYING_QUESTION", g__A_BUYING_QUESTION, s__A_BUYING_QUESTION);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_CALLS", g__CALLS, s__CALLS);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_HEAVY", g__HEAVY, s__HEAVY);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_NOT_A_USE", g__NOT_A_USE, s__NOT_A_USE);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_PHONE_THING", g__PHONE_THING, s__PHONE_THING);
    pyrt::xrefs::reg_access("knowledge.phone_use", "_PHOTOS", g__PHOTOS, s__PHOTOS);
}

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


thread_local! { static G_ASK_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANTITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STORE_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK_OPENS() -> Py { ensure_init(); G_ASK_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENS(v: Py) { ensure_init(); G_ASK_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NUMBER() -> Py { ensure_init(); G_A_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NUMBER(v: Py) { ensure_init(); G_A_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED() -> Py { ensure_init(); G_NOTHING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED(v: Py) { ensure_init(); G_NOTHING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENERS() -> Py { ensure_init(); G_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); G_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTER() -> Py { ensure_init(); G_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTER(v: Py) { ensure_init(); G_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANTITY_ASK() -> Py { ensure_init(); G_QUANTITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANTITY_ASK(v: Py) { ensure_init(); G_QUANTITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_STORE_FRAME() -> Py { ensure_init(); G_STORE_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STORE_FRAME(v: Py) { ensure_init(); G_STORE_FRAME.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that takes the chat back to something said earlier, and asks about it.\n\n    im going to lisbon in may\n    my son starts school in september\n    back to lisbon, what should i see\n\nThe last line names the thing it returns to and then asks about it. Read whole, it reached the\nback-pain entry: \"back\" is the return in \"back to\", not the body part, and the thing the person\nnamed - Lisbon - never got to the question. So `find()` answered \"A back like that: give it two\nto six weeks\". The same shape is a hedge when the thing does have an entry but the question no\nlonger names it: \"so about the washing machine, what should i get\" reached nothing at all.\n\nWhat this reads is the two parts separately: the opener is the return, the thing after it is what\nthe question is about, and the ask is the rest. The ask is then put back together with the thing\nin it - after the first question word when the ask has one (\"what should i get\" / \"washing\nmachine\" -> \"what washing machine should i get\", which is how English asks it), and at the end\notherwise (\"what should i see\" + \"lisbon\") - and only a version that reaches a real entry, that\nnames the thing, and that is not the entry the whole line already reached, is handed on.\n\nNothing is invented: the ask is the person's words and the reply comes from the entry. A line\nwith no ask (\"back to lisbon\" alone), a thing the chat never mentioned, or no entry for it all\nleave the turn exactly as it was.\n");
    s_OPENERS(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|right|alright|well|now)\\s+)*(?P<opener>back\\s+to|back\\s+on|returning\\s+to|as\\s+for|(?:anyway|so)\\s+about|about)\\s+(?P<rest>[^.?!]+.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STORE_FRAME(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|so\\s+)?(?:what|how)\\s+about\\s+(?=(?:back\\s+(?:to|on)|returning\\s+to|as\\s+for|(?:anyway|so)\\s+about|about)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTER(call_builtin("frozenset", &[mcall(&py_str("the a an my our your his her its their this that these those some any"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_OPENS(stdlib_call("re", "compile", &[py_str("^(?:what|which|whose|how\\s+much|how\\s+many|where|when|who|why|should|can|could|do|does|did|is|are|will|would)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_NAMED(call_builtin("frozenset", &[mcall(&py_str("it its that this these those they them there their one ones\nhe she him her his anything something everything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUANTITY_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:how\\s+(?:much|many)|what\\s+(?:weight|size|length|price|quantity))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NUMBER(stdlib_call("re", "compile", &[py_str("\\b(?:\\d|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|half|a couple|a few)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_chat_has(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the chat has said one of these words, so the return points at something real.");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = op_bitor(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&mcall(&said.clone(), "intersection", &[words.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_parts(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the line goes back to, and the ask after it, or None.\n\n    Both orders people use are here: the comma (\"back to lisbon, what should i see\") and the ask\n    straight after the thing (\"back to lisbon what should i see\"), where the split is at the\n    first question word.\n    ");
    match_ = mcall(&g_OPENERS(), "match", &[mcall(&g_STORE_FRAME(), "sub", &[py_str(""), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    rest = mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, py_str("")]), 2, None);
    thing = __unpk1[0].clone();
    ask = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&rest.clone(), &py_str(",")))) {
    let __unpk2 = py_unpack(&mcall(&rest.clone(), "split", &[py_str(","), Py::Int(1)]), 2, None);
    thing = __unpk2[0].clone();
    ask = __unpk2[1].clone();
    } else {
    found = stdlib_call("re", "search", &[py_str("\\s+(?=(?:what|which|where|when|who|why|how|should|can|could|do|does|is|are|will|would)\\b)"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&rest.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), slice_or_raise(&rest.clone(), &mcall(&found.clone(), "start", &[]), &Py::None, &Py::None)]), 2, None);
    thing = __unpk3[0].clone();
    ask = __unpk3[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&thing.clone(), &Py::None))) {
    return Py::None;
    }
    thing = mcall(&thing.clone(), "strip", &[py_str(" ,.:;?!")]);
    ask = mcall(&ask.clone(), "strip", &[py_str(" ,.:;?!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ask.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("opener")]), "lower", &[]), "strip", &[]), &py_str("about"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&rest.clone(), &py_str(","))) } }) {
    return Py::None;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[thing.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POINTER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { op_le(&call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOTHING_NAMED()) } }) {
    return Py::None;
    }
    return py_tuple(vec![named.clone(), ask.clone()]);
    Py::None
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
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut after: Py = Py::None;
    let mut first: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut space: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The ask with the thing written into it, in the orders worth trying.\n\n    Only the orders English uses. \"what\" and \"which\" take the thing straight after them (\"what\n    washing machine should i get\"), \"how much\" and \"how many\" the same (\"how much paint will the\n    front room need\"), and any other ask takes the thing at its end (\"where should i eat\" +\n    \"lisbon\"). A length ask is not written at all: \"how long will it take\" + \"weight\" is no\n    question anyone asks, and the entry it would reach answers how to lose weight rather than how\n    long it takes.\n    ");
    thing = mcall(&py_str(" "), "join", &[named.clone()]);
    measure = stdlib_call("re", "match", &[py_str("^(how\\s+(?:much|many))\\s+(.+)$"), ask.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&measure.clone()) {
    return py_list(vec![mcall(&op_add(&op_add(&op_add(&op_add(&mcall(&measure.clone(), "group", &[Py::Int(1)]), &py_str(" ")), &thing.clone()), &py_str(" ")), &mcall(&measure.clone(), "group", &[Py::Int(2)])), "strip", &[])]);
    }
    let __unpk1 = py_unpack(&mcall(&ask.clone(), "partition", &[py_str(" ")]), 3, None);
    first = __unpk1[0].clone();
    space = __unpk1[1].clone();
    after = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("what"), py_str("which"), py_str("whose")]), &mcall(&first.clone(), "lower", &[])))) {
    return py_list(vec![mcall(&op_add(&op_add(&op_add(&op_add(&first.clone(), &py_str(" ")), &thing.clone()), &py_str(" ")), &after.clone()), "strip", &[]), mcall(&op_add(&op_add(&ask.clone(), &py_str(" ")), &thing.clone()), "strip", &[])]);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^how\\b"), ask.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_list(vec![]);
    }
    return py_list(vec![mcall(&op_add(&op_add(&ask.clone(), &py_str(" ")), &thing.clone()), "strip", &[])]);
    Py::None
}
pub fn _the_domain_of_the_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut item: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The domain of the entry the chat's own line about this thing reached, or \"\".\n\n    A thing is not always a thing. \"back to the house, what should we offer\" was written into\n    \"what should we offer house\", and the entry that names a house is the one on house music: the\n    person buying a house was handed a Daft Punk album. What the chat already knows about the\n    thing is the line that named it, and the entry that line reached is filed under a domain -\n    the house was `money`, the music is `entertainment`. A return stays in the domain of the line\n    that named the thing; where that line reached nothing there is nothing to check against and\n    the line is left to stand on its own words.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = { let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "intersection", &[pyrt::xrefs::xcall("memory.topics", "words", &[line.clone()])])))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    item = pyrt::xrefs::xcall("memory.topics", "_entry", &[line.clone()]);
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
    return py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    return if py_truthy(&Py::Bool(!py_is(&item.clone(), &Py::None))) { call_builtin("str", &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    }
    return py_str("");
    Py::None
}
pub fn a_return_to_the_topic(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut domain: Py = Py::None;
    let mut item: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut without: Py = Py::None;
    let _ = py_str("The ask with the thing the line goes back to written into it, or \"\".\n\n    One line only: the first version that reaches an entry which the whole line does not, and\n    which the thing brought with it. Everything else leaves the turn exactly as it was.\n    ");
    parts = _the_parts(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&parts.clone(), 2, None);
    named = __unpk1[0].clone();
    ask = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_the_chat_has(&[call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()])))) {
    return py_str("");
    }
    let __try2 = exc::catch(|| {
    whole = pyrt::xrefs::xcall("memory.topics", "_entry", &[text.clone()]);
    without = pyrt::xrefs::xcall("memory.topics", "_entry", &[ask.clone()]);
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
    return py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    domain = _the_domain_of_the_chat(&[named.clone(), memory.clone()]);
    let __iter7 = py_iter_or_raise(&_the_lines(&[named.clone(), ask.clone()]));
    for __item in __iter7 {
    line = __item.clone();
    let __try8 = exc::catch(|| {
    item = pyrt::xrefs::xcall("memory.topics", "_entry", &[line.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["Exception"]) {
                __handled12 = true;
    continue;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&whole.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&item.clone(), &whole.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.topics", "_names_it", &[line.clone(), item.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&without.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&without.clone(), &item.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = domain.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &domain.clone())) } }) {
    continue;
    }
    return line.clone();
    }
    return py_str("");
    Py::None
}
pub fn the_thing_named(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The thing the line goes back to, as the person typed it, or \"\".");
    match_ = mcall(&g_OPENERS(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    rest = mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_contains(&rest.clone(), &py_str(",")))) {
    return py_str("");
    }
    return mcall(&geti(&mcall(&rest.clone(), "split", &[py_str(","), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(" ,.:;?!")]);
    Py::None
}
pub fn _the_sentence_with_the_amount(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of `answer` that gives the amount, or \"\" when none of them does.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = mcall(&sentence.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { mcall(&g_A_NUMBER(), "search", &[sentence.clone()]) } }) {
    return mcall(&sentence.clone(), "strip", &[]);
    }
    }
    return py_str("");
    Py::None
}
pub fn as_the_same_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut short: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "short") { v.clone() }
    else { Py::None };
    let mut ask: Py = Py::None;
    let mut named: Py = Py::None;
    let mut same_again: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The answer framed as the same one, for a return that lands on what is already on screen.\n\n    A return can reach the very answer the person already has, and handing it back word for word\n    is not a second answer (STYLE.md). The lead-in says the answer has not changed and keeps the\n    thing named, and `bot/memory/again.py` already carries both wordings - the one that says the\n    answer covers the thing, and the one that says the entry has nothing separate for it.\n    ");
    same_again = pyrt::xrefs::xmodule("memory.again");
    thing = { let __b = the_thing_named(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("that") } };
    ask = geti(&{ let __b = _the_parts(&[text.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![py_str(""), py_str("")]) } }, &Py::Int(1));
    if py_truthy(&{ let __b = ask.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUANTITY_ASK(), "match", &[ask.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_NUMBER(), "search", &[call_builtin("str", &[{ let __b = short.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } } } }) {
    short = { let __b = _the_sentence_with_the_amount(&[reply.clone()]); if py_truthy(&__b) { __b } else { short.clone() } };
    }
    short = mcall(&same_again.clone(), "_short", &[{ let __b = short.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&same_again.clone(), "unframed", &[reply.clone()]); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } } } }]);
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(short.clone()); __args.push(thing.clone()); __kws.extend(vec![("card".to_string(), reply.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same_again.clone(), "the_name_the_answer_says", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&short.clone(), "lower", &[]), &mcall(&named.clone(), "lower", &[]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), short.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&gattr(&same_again.clone(), "AGAIN"), &Py::Int(0)), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("detail".to_string(), named.clone()), ("reply".to_string(), short.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&gattr(&same_again.clone(), "DETAIL_AGAIN"), &Py::Int(1)), "format", args))(&__args) };
    Py::None
}
pub fn a_bare_return(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line only takes the chat back to a thing and asks nothing about it.\n\n    \"back to lisbon\" on its own names the thing and stops. There is no ask for the thing to be\n    written into, so nothing of the answer is being asked about either: read for a detail, the\n    line found the word \"back\" and said \"Nothing separate for back - it is the same advice: In\n    Lisbon: ...\" to a person who had only said which thing they meant, and \"about the boiler\"\n    was read for the word \"about\". A return with no ask names no part of an answer.\n    ");
    match_ = mcall(&g_OPENERS(), "match", &[mcall(&g_STORE_FRAME(), "sub", &[py_str(""), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&rest.clone(), &py_str(","))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:what|which|where|when|who|why|how|should|can|could|do|does|did|is|are|will|would)\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[rest.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POINTER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { op_le(&call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOTHING_NAMED()) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = Py::Bool(py_is(&memory.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _the_chat_has(&[call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.returns".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_OPENS" => Some(g_ASK_OPENS()),
            "A_NUMBER" => Some(g_A_NUMBER()),
            "NOTHING_NAMED" => Some(g_NOTHING_NAMED()),
            "OPENERS" => Some(g_OPENERS()),
            "POINTER" => Some(g_POINTER()),
            "QUANTITY_ASK" => Some(g_QUANTITY_ASK()),
            "STORE_FRAME" => Some(g_STORE_FRAME()),
            "_the_chat_has" => Some(py_func("_the_chat_has", |args: &[Py]| _the_chat_has(args))),
            "_the_domain_of_the_chat" => Some(py_func("_the_domain_of_the_chat", |args: &[Py]| _the_domain_of_the_chat(args))),
            "_the_lines" => Some(py_func("_the_lines", |args: &[Py]| _the_lines(args))),
            "_the_parts" => Some(py_func("_the_parts", |args: &[Py]| _the_parts(args))),
            "_the_sentence_with_the_amount" => Some(py_func("_the_sentence_with_the_amount", |args: &[Py]| _the_sentence_with_the_amount(args))),
            "a_bare_return" => Some(py_func("a_bare_return", |args: &[Py]| a_bare_return(args))),
            "a_return_to_the_topic" => Some(py_func("a_return_to_the_topic", |args: &[Py]| a_return_to_the_topic(args))),
            "as_the_same_answer" => Some(py_func("as_the_same_answer", |args: &[Py]| as_the_same_answer(args))),
            "the_thing_named" => Some(py_func("the_thing_named", |args: &[Py]| the_thing_named(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK_OPENS" => s_ASK_OPENS(v),
            "A_NUMBER" => s_A_NUMBER(v),
            "NOTHING_NAMED" => s_NOTHING_NAMED(v),
            "OPENERS" => s_OPENERS(v),
            "POINTER" => s_POINTER(v),
            "QUANTITY_ASK" => s_QUANTITY_ASK(v),
            "STORE_FRAME" => s_STORE_FRAME(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.returns", module_obj);
    pyrt::xrefs::reg_fn("memory.returns", "_the_chat_has", py_func("_the_chat_has", |args: &[Py]| _the_chat_has(args)));
    pyrt::xrefs::reg_fn("memory.returns", "_the_parts", py_func("_the_parts", |args: &[Py]| _the_parts(args)));
    pyrt::xrefs::reg_fn("memory.returns", "_the_lines", py_func("_the_lines", |args: &[Py]| _the_lines(args)));
    pyrt::xrefs::reg_fn("memory.returns", "_the_domain_of_the_chat", py_func("_the_domain_of_the_chat", |args: &[Py]| _the_domain_of_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.returns", "a_return_to_the_topic", py_func("a_return_to_the_topic", |args: &[Py]| a_return_to_the_topic(args)));
    pyrt::xrefs::reg_fn("memory.returns", "the_thing_named", py_func("the_thing_named", |args: &[Py]| the_thing_named(args)));
    pyrt::xrefs::reg_fn("memory.returns", "_the_sentence_with_the_amount", py_func("_the_sentence_with_the_amount", |args: &[Py]| _the_sentence_with_the_amount(args)));
    pyrt::xrefs::reg_fn("memory.returns", "as_the_same_answer", py_func("as_the_same_answer", |args: &[Py]| as_the_same_answer(args)));
    pyrt::xrefs::reg_fn("memory.returns", "a_bare_return", py_func("a_bare_return", |args: &[Py]| a_bare_return(args)));
    pyrt::xrefs::reg_access("memory.returns", "ASK_OPENS", g_ASK_OPENS, s_ASK_OPENS);
    pyrt::xrefs::reg_access("memory.returns", "A_NUMBER", g_A_NUMBER, s_A_NUMBER);
    pyrt::xrefs::reg_access("memory.returns", "NOTHING_NAMED", g_NOTHING_NAMED, s_NOTHING_NAMED);
    pyrt::xrefs::reg_access("memory.returns", "OPENERS", g_OPENERS, s_OPENERS);
    pyrt::xrefs::reg_access("memory.returns", "POINTER", g_POINTER, s_POINTER);
    pyrt::xrefs::reg_access("memory.returns", "QUANTITY_ASK", g_QUANTITY_ASK, s_QUANTITY_ASK);
    pyrt::xrefs::reg_access("memory.returns", "STORE_FRAME", g_STORE_FRAME, s_STORE_FRAME);
}

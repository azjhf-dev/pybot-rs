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


thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOAN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOAN() -> Py { ensure_init(); G_A_MOAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOAN(v: Py) { ensure_init(); G_A_MOAN.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_ASK() -> Py { ensure_init(); G_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_ASK(v: Py) { ensure_init(); G_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOKBACK() -> Py { ensure_init(); G_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOKBACK(v: Py) { ensure_init(); G_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MOAN_VERB() -> Py { ensure_init(); G_MOAN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOAN_VERB(v: Py) { ensure_init(); G_MOAN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_DONE() -> Py { ensure_init(); G_NOT_A_THING_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_DONE(v: Py) { ensure_init(); G_NOT_A_THING_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An ask about how long to do a thing, written out with the thing the chat is about.\n\n    i started learning guitar last month\n    my fingers hurt\n    how long should i practise          -> \"About six weeks for most bones - six for a wrist or\n                                            collarbone, six to eight for an arm ...\"\n\nThe last line is a question about the practice, and the chat is on the guitar: an answer about a\nfracture is the wrong answer to a person who has said nothing about a break. The line was read\nagainst the line before it, whose words are a complaint about fingers, and the entry on bone\nhealing keys both \"fingers\" and \"how long\", so it took the turn.\n\nThe doing word in the line (\"practise\") is what the ask is about, and the thing it is done with\n(\"guitar\") is what the chat has been doing. Written out with that thing - \"how long should i\npractise guitar?\" - the entry filed for the instrument answers seven words, which is the answer\nthe person wanted. The module only offers the rewrites: the caller looks each one up and takes the\nfirst that answers with a length, so a line whose thing reaches nothing keeps the turn it had.\n");
    s_LENGTH_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:long|many (?:hours|minutes|days|weeks|months|years))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nhow long many much hours hour days day weeks week months month years year minutes minute should\nshall can could would will do does did is are was i me my we you it that this to for the a an in\non at about each per every and or of time\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TIME_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nday days week weeks month months year years weekend summer winter spring autumn today tomorrow\nyesterday morning evening night ago last next this recent lately start started\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_THING_DONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("heal recover bleed"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOOKBACK(Py::Int(12));
    s_A_MOAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:can\\s+)?(?:never|always|keep|constantly|usually|often|still)\\b(?:\\s+(?P<verb>getting|get|doing|do|making|make|cooking|cook|picking|pick|finding|find|managing|manage))?\\b(?P<thing>[^?.!,]{1,40}?)\\s+(?:wrong|right|messed up|ruined|spoiled|spoilt|screwed up|backwards)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOAN_VERB(py_dict(vec![(py_str("getting"), py_str("get")), (py_str("doing"), py_str("do")), (py_str("making"), py_str("make")), (py_str("cooking"), py_str("cook")), (py_str("picking"), py_str("pick")), (py_str("finding"), py_str("find")), (py_str("managing"), py_str("manage"))]));
}
pub fn _is_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut doing: Py = Py::None;
    let _ = py_str("Whether this word is one of the things a person does, in the form it is typed in.\n\n    The list is written in the stem form (\"start\", \"learn\", \"practise\") and people type \"started\"\n    and \"learning\", so a word counts when a doing word of four letters or more is how it begins.\n    Two-letter stems are left out of that: \"dog\" begins \"do\" and no one does a dog.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING_DONE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING_DONE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::stem(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::topics::g_ACTION(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&crate::memory::topics::g_ACTION());
    for __item in __iter1 {
    doing = __item.clone();
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("e")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&doing.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&doing.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn doing_word(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The doing word in this line, in the words it is typed in, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_doing(args))(&__args) }) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn asks_about_doing(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("Whether this is a length ask about doing something, in the words it is typed in.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LENGTH_ASK(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| doing_word(args))(&__args) }, &Py::None));
    Py::None
}
pub fn _the_things(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut record: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The things this chat has been about, newest first, as the words a line can be written on.\n\n    A thing is what a doing in the chat is done with: the nouns of an earlier line that names\n    something the person does (\"started learning guitar\" is a practice, and the guitar is what it\n    is done with), minus the doing words themselves (the ask has its own), the time words and the\n    grammar. A line with no doing in it names a state rather than the thing a practice is done\n    with: \"my fingers hurt\" is what happened to the person, and \"how long should i practise\n    finger\" is the bone-healing card, which is the fault this module exists for.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topics")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_LOOKBACK()), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    record = __item.clone();
    text = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| doing_word(args))(&__args) }, &Py::None))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::keywords(args))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TIME_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_doing(args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&out.clone(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn written_out(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line written out with each thing the chat is about, newest thing first.\n\n    Empty when the line is not a length ask about doing something, when it names a thing of its\n    own that is not the ask's grammar (\"how long should i practise guitar\" needs no rewrite), or\n    when the chat has done nothing yet.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?.! ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_about_doing(args))(&__args) }))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_ASK_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_doing(args))(&__args) })) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_tuple(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_things(args))(&__args) });
    for __item in __iter1 {
    thing = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&said.clone(), &py_str(" ")), &thing.clone()), &py_str("?"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&said.clone(), &py_str(" ")), &thing.clone()), &py_str(" each day?"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_ask_behind_a_moan(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The how-to ask behind \"i always get rice wrong\", or \"\" when the line is not that.\n\n    Only the rewrite is offered; the caller looks it up and takes the answer only when it is\n    about the thing the person named, because read loosely the rewrite reaches anything (\"how do\n    i get the kids to bed right\" reaches the frost card, which is none of it).\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)) } }) {
    return py_str("");
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MOAN(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) })); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::keywords(args))(&__args) }))) {
    return py_str("");
    }
    verb = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("verb")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("get") } }, "lower", args))(&__args) };
    return op_add(&op_add(&op_add(&op_add(&py_str("how do i "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(verb.clone()); __args.push(verb.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MOAN_VERB(), "get", args))(&__args) }), &py_str(" ")), &thing.clone()), &py_str(" right?"));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.doing_it".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "A_MOAN" => Some(g_A_MOAN()),
            "LENGTH_ASK" => Some(g_LENGTH_ASK()),
            "LOOKBACK" => Some(g_LOOKBACK()),
            "MOAN_VERB" => Some(g_MOAN_VERB()),
            "NOT_A_THING_DONE" => Some(g_NOT_A_THING_DONE()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "_is_doing" => Some(py_func("_is_doing", |args: &[Py]| _is_doing(args))),
            "_the_things" => Some(py_func("_the_things", |args: &[Py]| _the_things(args))),
            "asks_about_doing" => Some(py_func("asks_about_doing", |args: &[Py]| asks_about_doing(args))),
            "doing_word" => Some(py_func("doing_word", |args: &[Py]| doing_word(args))),
            "the_ask_behind_a_moan" => Some(py_func("the_ask_behind_a_moan", |args: &[Py]| the_ask_behind_a_moan(args))),
            "written_out" => Some(py_func("written_out", |args: &[Py]| written_out(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK_WORDS" => s_ASK_WORDS(v),
            "A_MOAN" => s_A_MOAN(v),
            "LENGTH_ASK" => s_LENGTH_ASK(v),
            "LOOKBACK" => s_LOOKBACK(v),
            "MOAN_VERB" => s_MOAN_VERB(v),
            "NOT_A_THING_DONE" => s_NOT_A_THING_DONE(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.doing_it", module_obj);
    pyrt::xrefs::reg_fn("memory.doing_it", "_is_doing", py_func("_is_doing", |args: &[Py]| _is_doing(args)));
    pyrt::xrefs::reg_fn("memory.doing_it", "doing_word", py_func("doing_word", |args: &[Py]| doing_word(args)));
    pyrt::xrefs::reg_fn("memory.doing_it", "asks_about_doing", py_func("asks_about_doing", |args: &[Py]| asks_about_doing(args)));
    pyrt::xrefs::reg_fn("memory.doing_it", "_the_things", py_func("_the_things", |args: &[Py]| _the_things(args)));
    pyrt::xrefs::reg_fn("memory.doing_it", "written_out", py_func("written_out", |args: &[Py]| written_out(args)));
    pyrt::xrefs::reg_fn("memory.doing_it", "the_ask_behind_a_moan", py_func("the_ask_behind_a_moan", |args: &[Py]| the_ask_behind_a_moan(args)));
    pyrt::xrefs::reg_access("memory.doing_it", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("memory.doing_it", "A_MOAN", g_A_MOAN, s_A_MOAN);
    pyrt::xrefs::reg_access("memory.doing_it", "LENGTH_ASK", g_LENGTH_ASK, s_LENGTH_ASK);
    pyrt::xrefs::reg_access("memory.doing_it", "LOOKBACK", g_LOOKBACK, s_LOOKBACK);
    pyrt::xrefs::reg_access("memory.doing_it", "MOAN_VERB", g_MOAN_VERB, s_MOAN_VERB);
    pyrt::xrefs::reg_access("memory.doing_it", "NOT_A_THING_DONE", g_NOT_A_THING_DONE, s_NOT_A_THING_DONE);
    pyrt::xrefs::reg_access("memory.doing_it", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
}

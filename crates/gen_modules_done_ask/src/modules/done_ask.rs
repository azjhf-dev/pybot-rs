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


thread_local! { static G_A_DONENESS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONENESS_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONENESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.done_ask: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_DONENESS_ASK() -> Py { ensure_init(); G_A_DONENESS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DONENESS_ASK(v: Py) { ensure_init(); G_A_DONENESS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DONENESS_KEYS() -> Py { ensure_init(); G_DONENESS_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONENESS_KEYS(v: Py) { ensure_init(); G_DONENESS_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONENESS_WORDS() -> Py { ensure_init(); G_DONENESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONENESS_WORDS(v: Py) { ensure_init(); G_DONENESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A doneness ask names a state and no dish, so it is read against the dish the chat is on.\n\n    im cooking a roast chicken for six on sunday\n      About 20 minutes per 500g plus 20 minutes at 190C ... check the thigh juices run clear.\n    its about two kilos\n      About two kilos, then. ...\n    and how do i know its cooked\n      Chicken is cooked when the thickest part reaches 75C on a digital thermometer ...\n\nThe last line asks about a state (\"cooked\") and names nothing, so on its own words it reaches\nno entry and the person is told the bot would rather not guess - about the roast it has just\ngiven the timings for. The dish is in the chat, so its own word goes in front of the line\n(\"chicken how do i know its cooked\") and the reading in memory carries that.\n\nThe fill is written to `memory[\"understood\"]` by an observer, because that is the reading every\nmodule below answers from: the answers, the composer and the store's own recall all read it, so\na fill-in written anywhere else would only move one of them. The module answers nothing itself\n- it has no `respond` - so it can only ever change how a line is read, never take a turn.\n\nThe rule is general, not a list of dishes. A word from the chat's own topic counts only when all\nthree hold:\n\n1. the entry it reaches is keyed to the word;\n2. that key is the entry's *first* group, the thing the entry is about, not the words it is\n   asked by, so \"oven how will I know when it's done\" does not reach the when-to-buy entry\n   through \"when\";\n3. the entry is keyed to a done state (\"done\", \"cooked\", \"ready\", \"temperature\", ...).\n\nA line that names its own dish needs nothing filled in, so it is left exactly as it came in,\nand a chat with no such dish (\"we are out all day\" / \"is it ready yet\") keeps the answer it\nalready had.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.done_ask.PRIORITY"); }
    s_PRIORITY(Py::Int(6900));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.done_ask.A_DONENESS_ASK"); }
    s_A_DONENESS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how (?:will|would|do|can|should) (?:i|you) (?:know|tell)|when (?:will|do) (?:i|you) know|when (?:is|will) it(?: be)?|is it|are they|will it be|how long (?:do|should) (?:i|you) (?:cook|leave|bake|roast|boil|fry|grill|simmer))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.done_ask.DONENESS_WORDS"); }
    s_DONENESS_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:done|dones|cooked|uncooked|overdone|undercooked|overcooked|ready)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.done_ask.DONENESS_KEYS"); }
    s_DONENESS_KEYS(py_tuple(vec![py_str("done"), py_str("cooked"), py_str("cooked through"), py_str("ready"), py_str("how can i tell"), py_str("how do i tell"), py_str("how do you tell"), py_str("when is it done"), py_str("when its done"), py_str("temperature"), py_str("temp"), py_str("thermometer")]));
}
pub fn _entry(args: &[Py]) -> Py {
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
    let mut find: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The knowledge entry a line reaches, or None (the knowledge base is read, not changed).");
    find = py_func("find", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "find", args));
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return py_call(&find, &[py_dict(vec![(py_str("clean"), said.clone()), (py_str("words"), pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()]))])]);
    Py::None
}
pub fn _the_answer_to_lean_on(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The bot's last reply, or ''.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    one = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_topics_on_the_chat(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The topic in hand first, then the ones the chat has been on, newest first.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    one = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&out.clone(), &said.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[said.clone()]);
    }
    }
    record = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]);
    said = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&out.clone(), &said.clone()))) {
    let _ = mcall(&out.clone(), "remove", &[said.clone()]);
    }
    if py_truthy(&said.clone()) {
    let _ = mcall(&out.clone(), "insert", &[Py::Int(0), said.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _key_groups(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut groups: Py = Py::None;
    let mut one: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The groups of phrases an entry is keyed by, in the order the entry says them.");
    groups = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&groups.clone(), "append", &[py_tuple(vec![part.clone()])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&groups.clone(), "append", &[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&part.clone()) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    }
    return groups.clone();
    Py::None
}
pub fn _keys_the_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrases: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut one: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a key group holds the word, in any of the forms it takes (\"cooked\" for \"cook\").");
    word = call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    let _ = mcall(&said.clone(), "update", &[pyrt::xrefs::xcall("memory.topics", "words", &[phrase.clone()])]);
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    one = __item.clone();
    let __try3 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xcall("memory.topics", "forms", &[one.clone()]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]), &one.clone())) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["Exception"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _about_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut groups: Py = Py::None;
    let _ = py_str("True when the entry's first key group is the word as the thing it is about.\n\n    An entry's first group is the thing and the groups after it are how the thing is asked\n    about, so \"roast how will I know when it's done\" reaches the general roasting entry through\n    a word of its ask group, where the dish is a word it happens to mention. Only the first\n    group says the entry is about the word.\n    ");
    groups = _key_groups(&[item.clone()]);
    return { let __b = call_builtin("bool", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _keys_the_word(&[geti(&groups.clone(), &Py::Int(0)), word.clone()]) } };
    Py::None
}
pub fn _says_how_it_turns_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut group: Py = Py::None;
    let mut one: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the entry is keyed to a done state, not only to the thing.\n\n    A group holds the state as it is written (\"cooked through\", \"how can i tell\"), so every\n    word of the state has to be in one group: a \"cook\" somewhere and a \"through\" somewhere else\n    is not the entry that says how the thing turns out.\n    ");
    let __iter1 = py_iter_or_raise(&_key_groups(&[item.clone()]));
    for __item in __iter1 {
    group = __item.clone();
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&group.clone());
    for __item in __iter2 {
    phrase = __item.clone();
    let _ = mcall(&said.clone(), "update", &[pyrt::xrefs::xcall("memory.topics", "words", &[phrase.clone()])]);
    }
    let __iter3 = py_iter_or_raise(&g_DONENESS_KEYS());
    for __item in __iter3 {
    one = __item.clone();
    if py_truthy(&op_le(&call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[one.clone()])]).unwrap_or_else(|e| exc::raise(e)), &said.clone())) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_thing_words(args: &[Py]) -> Py {
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
    let mut said_words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_words"))) };
    let mut out: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of the things the chat has been on, newest first, that the line does not say.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_the_topics_on_the_chat(&[memory.clone()]));
    for __item in __iter1 {
    topic = __item.clone();
    let __iter2 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[topic.clone()]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said_words.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.topics", "MEASURE_WORDS"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&out.clone(), &word.clone())) } } } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _the_chat_has_moved_on(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut here: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("True when the last thing the chat was about reaches an entry in another domain.\n\n    The search walks every topic the chat has been on, so after the subject has changed an old\n    dish comes back: \"im doing a roast chicken for six on sunday\" / \"how long do i cook it\" /\n    \"also my car needs a service\" / \"is it ready yet\" has the car's \"it\", and a temperature for\n    the chicken is a confident answer about a thing the person has left. A wrong subject is worse\n    than the hedge it replaces.\n\n    The test is the newest topic that reaches an entry at all - the ask itself reaches none, and\n    neither does a line that only says how the oven is running, so neither of those is a move: in\n    a lamb roast, \"my oven runs a bit cool\" leaves the roast as what the chat is on.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood"), py_dict(vec![])]), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    mine = call_builtin("str", &[{ let __b = call_builtin("getattr", &[found.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&_the_topics_on_the_chat(&[memory.clone()]));
    for __item in __iter1 {
    topic = __item.clone();
    topic = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = topic.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&topic.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&topic.clone(), &said.clone())) } }) {
    continue;
    }
    here = _entry(&[topic.clone()]);
    if py_truthy(&Py::Bool(py_is(&here.clone(), &Py::None))) {
    continue;
    }
    theirs = call_builtin("str", &[{ let __b = call_builtin("getattr", &[here.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[theirs.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&theirs.clone(), &mine.clone())) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_done_state_of_the_thing_in_hand(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut filled: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The doneness ask with the word of the thing the chat is on in it, or \"\".\n\n    \"and how will I know when it's done\" after \"a slow roast shoulder of lamb\" names a state\n    and no dish, so it reached nothing and the person was told the bot would rather not guess\n    about its own roast answer. The dish is in the chat, so its own word is put in front of the\n    line: \"lamb how will i know when its done\" is the lamb temperature entry.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_DONENESS_ASK(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DONENESS_WORDS(), "search", &[said.clone()]))) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_answer_to_lean_on(&[memory.clone()])))) {
    return py_str("");
    }
    said_words = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&_the_thing_words(&[memory.clone(), said_words.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![op_add(&op_add(&word.clone(), &py_str(" ")), &said.clone()), op_add(&op_add(&mcall(&said.clone(), "rstrip", &[py_str("?.! ")]), &py_str(" ")), &word.clone())]));
    for __item in __iter2 {
    filled = __item.clone();
    found = _entry(&[filled.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_about_the_thing(&[found.clone(), word.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_says_how_it_turns_out(&[found.clone()])))) {
    continue;
    }
    if py_truthy(&_the_chat_has_moved_on(&[found.clone(), memory.clone()])) {
    return py_str("");
    }
    return filled.clone();
    }
    }
    return py_str("");
    Py::None
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut filled: Py = Py::None;
    let mut read: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Read a doneness ask against the dish the chat is on, and write that reading down.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    filled = the_done_state_of_the_thing_in_hand(&[said.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&filled.clone()))) {
    return Py::None;
    }
    reading = call_builtin("dict", &[read.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&reading.clone(), &py_str("clean"), filled.clone());
    let _ = py_setitem(&reading.clone(), &py_str("words"), pyrt::xrefs::xcall("memory.topics", "words", &[filled.clone()]));
    if py_truthy(&Py::Bool(py_contains(&reading.clone(), &py_str("norm_text")))) {
    let _ = py_setitem(&reading.clone(), &py_str("norm_text"), filled.clone());
    }
    let _ = py_setitem(&memory.clone(), &py_str("understood"), reading.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.done_ask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DONENESS_ASK" => Some(g_A_DONENESS_ASK()),
            "DONENESS_KEYS" => Some(g_DONENESS_KEYS()),
            "DONENESS_WORDS" => Some(g_DONENESS_WORDS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_about_the_thing" => Some(py_func("_about_the_thing", |args: &[Py]| _about_the_thing(args))),
            "_entry" => Some(py_func("_entry", |args: &[Py]| _entry(args))),
            "_key_groups" => Some(py_func("_key_groups", |args: &[Py]| _key_groups(args))),
            "_keys_the_word" => Some(py_func("_keys_the_word", |args: &[Py]| _keys_the_word(args))),
            "_says_how_it_turns_out" => Some(py_func("_says_how_it_turns_out", |args: &[Py]| _says_how_it_turns_out(args))),
            "_the_answer_to_lean_on" => Some(py_func("_the_answer_to_lean_on", |args: &[Py]| _the_answer_to_lean_on(args))),
            "_the_chat_has_moved_on" => Some(py_func("_the_chat_has_moved_on", |args: &[Py]| _the_chat_has_moved_on(args))),
            "_the_thing_words" => Some(py_func("_the_thing_words", |args: &[Py]| _the_thing_words(args))),
            "_the_topics_on_the_chat" => Some(py_func("_the_topics_on_the_chat", |args: &[Py]| _the_topics_on_the_chat(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "the_done_state_of_the_thing_in_hand" => Some(py_func("the_done_state_of_the_thing_in_hand", |args: &[Py]| the_done_state_of_the_thing_in_hand(args))),
            "_topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DONENESS_ASK" => s_A_DONENESS_ASK(v),
            "DONENESS_KEYS" => s_DONENESS_KEYS(v),
            "DONENESS_WORDS" => s_DONENESS_WORDS(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.done_ask", module_obj);
    pyrt::xrefs::reg_fn("modules.done_ask", "_entry", py_func("_entry", |args: &[Py]| _entry(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_the_answer_to_lean_on", py_func("_the_answer_to_lean_on", |args: &[Py]| _the_answer_to_lean_on(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_the_topics_on_the_chat", py_func("_the_topics_on_the_chat", |args: &[Py]| _the_topics_on_the_chat(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_key_groups", py_func("_key_groups", |args: &[Py]| _key_groups(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_keys_the_word", py_func("_keys_the_word", |args: &[Py]| _keys_the_word(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_about_the_thing", py_func("_about_the_thing", |args: &[Py]| _about_the_thing(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_says_how_it_turns_out", py_func("_says_how_it_turns_out", |args: &[Py]| _says_how_it_turns_out(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_the_thing_words", py_func("_the_thing_words", |args: &[Py]| _the_thing_words(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "_the_chat_has_moved_on", py_func("_the_chat_has_moved_on", |args: &[Py]| _the_chat_has_moved_on(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "the_done_state_of_the_thing_in_hand", py_func("the_done_state_of_the_thing_in_hand", |args: &[Py]| the_done_state_of_the_thing_in_hand(args)));
    pyrt::xrefs::reg_fn("modules.done_ask", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_access("modules.done_ask", "A_DONENESS_ASK", g_A_DONENESS_ASK, s_A_DONENESS_ASK);
    pyrt::xrefs::reg_access("modules.done_ask", "DONENESS_KEYS", g_DONENESS_KEYS, s_DONENESS_KEYS);
    pyrt::xrefs::reg_access("modules.done_ask", "DONENESS_WORDS", g_DONENESS_WORDS, s_DONENESS_WORDS);
    pyrt::xrefs::reg_access("modules.done_ask", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

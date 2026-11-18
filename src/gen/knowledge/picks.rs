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
pub static MEDIUM_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOOD_NEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_ORDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SERIES_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MOOD_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TITLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_MEDIUM_WORDS() -> Py { ensure_init(); MEDIUM_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDIUM_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MEDIUM_WORDS.write() { *w = v; } }
pub fn g_MOOD_NEAR() -> Py { ensure_init(); MOOD_NEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOOD_NEAR(v: Py) { ensure_init(); if let Ok(mut w) = MOOD_NEAR.write() { *w = v; } }
pub fn g_PICKS() -> Py { ensure_init(); PICKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICKS(v: Py) { ensure_init(); if let Ok(mut w) = PICKS.write() { *w = v; } }
pub fn g_PICK_ORDER() -> Py { ensure_init(); PICK_ORDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_ORDER(v: Py) { ensure_init(); if let Ok(mut w) = PICK_ORDER.write() { *w = v; } }
pub fn g_SERIES_SIDE() -> Py { ensure_init(); SERIES_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SERIES_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = SERIES_SIDE.write() { *w = v; } }
pub fn g__MOOD_LEAD() -> Py { ensure_init(); _MOOD_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MOOD_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _MOOD_LEAD.write() { *w = v; } }
pub fn g__TITLED() -> Py { ensure_init(); _TITLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TITLED(v: Py) { ensure_init(); if let Ok(mut w) = _TITLED.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A pick asked for again: another film, another book, from the same table.\n\nThe answers lane owns this domain. \"any other films\" and \"something else to watch\" after a\nrecommendation are among the commonest follow-ups people send, and each of the mood tables in\nbot/knowledge/entertainment.py holds more than the one reply that was given. Nothing new has to\nbe written down for it: this domain reads the pick already given, and hands over a part of the\nsame table the chat has not used yet. It is a domain of its own because it leans on the whole\nchat (what was recommended, what mood was asked for) rather than on the line alone.\n\nThe key names the medium on purpose. A key of bare follow-up words (\"anything else\", \"any other\nideas\") would make the understanding engine read every such follow-up as naming a subject of\nits own, which breaks follow-up detection for the whole chat: with that key, \"its a work laptop\nso i cant install much\" stopped being a follow-up and lost its answer.\n");
    s_ANSWERS(py_tuple(vec![py_str("entertainment"), py_str("creative")]));
    s_PICKS(py_dict(vec![(py_str("film"), crate::gen::knowledge::entertainment::g_FILM_MOODS()), (py_str("book"), crate::gen::knowledge::entertainment::g_BOOK_MOODS())]));
    s_MEDIUM_WORDS(py_dict(vec![(py_str("film"), py_tuple(vec![py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("watch"), py_str("watching"), py_str("netflix"), py_str("telly"), py_str("tv"), py_str("series"), py_str("cinema")])), (py_str("book"), py_tuple(vec![py_str("book"), py_str("books"), py_str("read"), py_str("reading"), py_str("novel"), py_str("novels"), py_str("author"), py_str("audiobook")]))]));
    s__TITLED(stdlib_call("re", "compile", &[py_str("\\*([^*\\n]{2,60})\\*")]).unwrap_or_else(|e| exc::raise(e)));
    s__MOOD_LEAD(stdlib_call("re", "compile", &[py_str("^for (?:a|an|the)[^:\\n]{0,40}:\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOOD_NEAR(py_dict(vec![(py_str("funny"), py_tuple(vec![py_str("light"), py_str("uplifting"), py_str("twist")])), (py_str("light"), py_tuple(vec![py_str("uplifting"), py_str("funny"), py_str("short")])), (py_str("uplifting"), py_tuple(vec![py_str("light"), py_str("funny"), py_str("romance")])), (py_str("sad"), py_tuple(vec![py_str("romance"), py_str("think"), py_str("uplifting")])), (py_str("think"), py_tuple(vec![py_str("twist"), py_str("true"), py_str("gripping")])), (py_str("gripping"), py_tuple(vec![py_str("twist"), py_str("think"), py_str("true")])), (py_str("twist"), py_tuple(vec![py_str("gripping"), py_str("think"), py_str("true")])), (py_str("true"), py_tuple(vec![py_str("think"), py_str("gripping"), py_str("uplifting")])), (py_str("short"), py_tuple(vec![py_str("light"), py_str("funny"), py_str("romance")])), (py_str("romance"), py_tuple(vec![py_str("uplifting"), py_str("sad"), py_str("light")])), (py_str("scary"), py_tuple(vec![py_str("think"), py_str("gripping"), py_str("twist")]))]));
    s_PICK_ORDER(py_tuple(vec![py_str("light"), py_str("uplifting"), py_str("funny"), py_str("think"), py_str("gripping"), py_str("twist"), py_str("true"), py_str("short"), py_str("romance"), py_str("scary"), py_str("sad")]));
    s_SERIES_SIDE(call_builtin("frozenset", &[mcall(&py_str("series sitcom episodes episode netflix iplayer prime tv telly channel\nstreaming broadcast"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("another-pick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("any other film"), py_str("any other films"), py_str("any other movie"), py_str("any other movies"), py_str("any other book"), py_str("any other books"), py_str("any other series"), py_str("another film"), py_str("another movie"), py_str("another book"), py_str("something else to watch"), py_str("anything else to watch"), py_str("something to watch after"), py_str("one more film"), py_str("one more book")]), py_tuple(vec![py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("book"), py_str("books"), py_str("series"), py_str("watch"), py_str("read")])])); __args.push(g__other_pick()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("other"), py_str("another"), py_str("else"), py_str("more"), py_str("after"), py_str("next")])), ("avoid".to_string(), add(&crate::gen::knowledge::entertainment::g_GRAVE(), &py_tuple(vec![py_str("instead of"), py_str("rather than"), py_str("how do i"), py_str("how can i"), py_str("cancel"), py_str("stop"), py_str("delete"), py_str("hide"), py_str("cheaper"), py_str("free"), py_str("where can i")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _titles_in(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let _ = py_str("The works a reply names, which it writes between two asterisks.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__TITLED(), "findall", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "strip", &[])); } py_list(__out) })() };
    Py::None
}
pub fn _medium_of(args: &[Py]) -> Py {
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
    let mut cues: Py = Py::None;
    let mut found: Py = Py::None;
    let mut medium: Py = Py::None;
    let _ = py_str("Which table a line is asking from: \"film\" or \"book\", or None.");
    found = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_MEDIUM_WORDS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    medium = __unpk2[0].clone();
    cues = __unpk2[1].clone();
    if py_truthy(&bitand(&found.clone(), &call_builtin("set", &[cues.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return medium.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_line(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut key: Py = Py::None;
    let mut one: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line being answered: the reading carries it, since memory does not hold it yet.");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean"), py_str("line")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&understood.clone(), "get", &[key.clone()]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&geti(&understood.clone(), &key.clone()), "strip", &[]) } }) {
    return geti(&understood.clone(), &key.clone());
    }
    }
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    return call_builtin("str", &[{ let __b = if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _pick_in_play(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut here: Py = Py::None;
    let mut last: Py = Py::None;
    let mut medium: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut replies: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut title: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The medium of the pick the last reply gave and the works it named: (medium, named).\n\n    A reply that names fewer than two works is not a pick - it is a question about the one\n    work, and a follow-up is about that work, not a list. The medium is the one the follow-up\n    itself names first (\"any other books\" after a film is a book, not the film it came from),\n    then the words the reply used (\"a series\", \"by Richard Osman\").\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    replies = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "get", &[py_str("bot")])) {  __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&replies.clone()))) {
    return py_tuple(vec![Py::None, call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    last = geti(&replies.clone(), &neg(&Py::Int(1)));
    if py_truthy(&lt(&call_builtin("len", &[_titles_in(&[last.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![Py::None, call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&replies.clone()) { let mut reply: Py = __item.clone(); for __item in py_iter_or_raise(&_titles_in(&[reply.clone()])) { let mut title: Py = __item.clone(); __out.push(mcall(&title.clone(), "lower", &[])); } } py_set(__out) })() };
    here = _the_line(&[understood.clone(), memory.clone()]);
    medium = { let __b = _medium_of(&[here.clone()]); if py_truthy(&__b) { __b } else { _medium_of(&[last.clone()]) } };
    return py_tuple(vec![medium.clone(), named.clone()]);
    Py::None
}
pub fn _moods_named(args: &[Py]) -> Py {
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
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: table"))) };
    let mut mood: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The moods of the table a line names, in table order.");
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&table.clone()) { let mut mood: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &mood.clone()))) {  __out.push(mood.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _other_pick(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut before: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut medium: Py = Py::None;
    let mut mood: Py = Py::None;
    let mut named: Py = Py::None;
    let mut near: Py = Py::None;
    let mut one: Py = Py::None;
    let mut order: Py = Py::None;
    let mut part: Py = Py::None;
    let mut seen_moods: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut table: Py = Py::None;
    let mut title: Py = Py::None;
    let mut titles: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A second recommendation, from the same table as the pick just given.\n\n    None when the last reply was not a list of picks, which is what keeps the entry off a\n    follow-up about something else that happens to mention a film.\n    ");
    let __unpk1 = py_unpack(&_pick_in_play(&[memory.clone(), understood.clone()]), 2, None);
    medium = __unpk1[0].clone();
    named = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&medium.clone()))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    asked = _the_line(&[understood.clone(), memory.clone()]);
    if py_truthy(&bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&asked.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_SERIES_SIDE())) {
    return Py::None;
    }
    table = geti(&g_PICKS(), &medium.clone());
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    order = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&py_tuple(vec![asked.clone(), if py_truthy(&before.clone()) { geti(&before.clone(), &neg(&Py::Int(1))) } else { py_str("") }, mcall(&py_str(" "), "join", &[before.clone()])]));
    for __item in __iter2 {
    part = __item.clone();
    let __iter3 = py_iter_or_raise(&_moods_named(&[part.clone(), table.clone()]));
    for __item in __iter3 {
    mood = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&order.clone(), &mood.clone()))) {
    continue;
    }
    let _ = mcall(&order.clone(), "append", &[mood.clone()]);
    let __aug4 = add(&order.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MOOD_NEAR(), "get", &[mood.clone(), py_tuple(vec![])])) { let mut near: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&table.clone(), &near.clone()))) {  __out.push(near.clone()); } } py_list(__out) })() });
    order = __aug4;
    }
    }
    let __aug5 = add(&order.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PICK_ORDER()) { let mut mood: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&table.clone(), &mood.clone()))) {  __out.push(mood.clone()); } } py_list(__out) })() });
    order = __aug5;
    let __aug6 = add(&order.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&table.clone()) { let mut mood: Py = __item.clone(); __out.push(mood.clone()); } py_list(__out) })() });
    order = __aug6;
    seen_moods = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    listed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut title: Py = __item.clone(); __out.push(mcall(&title.clone(), "lower", &[])); } py_set(__out) })() };
    let __iter7 = py_iter_or_raise(&order.clone());
    for __item in __iter7 {
    mood = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&seen_moods.clone(), &mood.clone()))) {
    continue;
    }
    let _ = mcall(&seen_moods.clone(), "add", &[mood.clone()]);
    sentence = geti(&table.clone(), &mood.clone());
    titles = _titles_in(&[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&titles.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&titles.clone()) { let mut title: Py = __item.clone(); __out.push(Py::Bool(py_contains(&listed.clone(), &mcall(&title.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    return add(&py_str("Something different then: "), &mcall(&g__MOOD_LEAD(), "sub", &[py_str(""), sentence.clone()]));
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.picks".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "MEDIUM_WORDS" => Some(g_MEDIUM_WORDS()),
            "MOOD_NEAR" => Some(g_MOOD_NEAR()),
            "PICKS" => Some(g_PICKS()),
            "PICK_ORDER" => Some(g_PICK_ORDER()),
            "SERIES_SIDE" => Some(g_SERIES_SIDE()),
            "_MOOD_LEAD" => Some(g__MOOD_LEAD()),
            "_TITLED" => Some(g__TITLED()),
            "_medium_of" => Some(py_func("_medium_of", |args: &[Py]| _medium_of(args))),
            "_moods_named" => Some(py_func("_moods_named", |args: &[Py]| _moods_named(args))),
            "_other_pick" => Some(py_func("_other_pick", |args: &[Py]| _other_pick(args))),
            "_pick_in_play" => Some(py_func("_pick_in_play", |args: &[Py]| _pick_in_play(args))),
            "_the_line" => Some(py_func("_the_line", |args: &[Py]| _the_line(args))),
            "_titles_in" => Some(py_func("_titles_in", |args: &[Py]| _titles_in(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static MEDIUM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEDIUM_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOST_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TASTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TITLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _VAGUE_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_MEDIUM() -> Py { ensure_init(); MEDIUM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDIUM(v: Py) { ensure_init(); if let Ok(mut w) = MEDIUM.write() { *w = v; } }
pub fn g_MEDIUM_WORDS() -> Py { ensure_init(); MEDIUM_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDIUM_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MEDIUM_WORDS.write() { *w = v; } }
pub fn g_MOST_WORDS() -> Py { ensure_init(); MOST_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MOST_WORDS.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_REFUSED() -> Py { ensure_init(); REFUSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED.write() { *w = v; } }
pub fn g_SOMETHING() -> Py { ensure_init(); SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = SOMETHING.write() { *w = v; } }
pub fn g_TASTE() -> Py { ensure_init(); TASTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TASTE(v: Py) { ensure_init(); if let Ok(mut w) = TASTE.write() { *w = v; } }
pub fn g_WANTS() -> Py { ensure_init(); WANTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTS(v: Py) { ensure_init(); if let Ok(mut w) = WANTS.write() { *w = v; } }
pub fn g__ASKS_SOMETHING() -> Py { ensure_init(); _ASKS_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_SOMETHING.write() { *w = v; } }
pub fn g__LEAD() -> Py { ensure_init(); _LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _LEAD.write() { *w = v; } }
pub fn g__TITLE() -> Py { ensure_init(); _TITLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TITLE(v: Py) { ensure_init(); if let Ok(mut w) = _TITLE.write() { *w = v; } }
pub fn g__VAGUE_LEAD() -> Py { ensure_init(); _VAGUE_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__VAGUE_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _VAGUE_LEAD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A taste named on the next line is a request for more of it, in the thing on the table.\n\n    what should i watch tonight / i like sci fi      -> the science fiction films\n    any good books / i like crime                    -> the crime books\n\nBoth used to be acknowledged and dropped (\"Got it - you like sci fi.\"), because \"i like sci fi\" on\nits own names no medium: the knowledge base knows science fiction films and science fiction books,\nand nothing in the line says which. The medium is in the answer just given, so the line carries it\nalong and the entry that answered the first turn answers this one too.\n\nThree things keep it from firing where it should not:\n\n  * the answer before it has to have offered choices (two named things to pick between) - \"any\n    good books for my son\" is answered with how to read with him, and a taste said next to that is\n    not asking for a list of books;\n  * a taste with nothing on the table is a fact about the person (\"i like crime\" on its own), and\n    stays one;\n  * a taste said twice running is served once (the second one would hand the same list back);\n  * the answer it reaches has to say something new, which is checked where the answer is made\n    (`bot/modules/answers.py`, `_same_as_the_last_answer`), because an entry with no picks for\n    that taste hands the same list back and the person hears what they have just heard.\n");
    s_TASTE(stdlib_call("re", "compile", &[py_str("^(?:(?:i|we)\\s+)?(?:really\\s+|do\\s+|just\\s+)?(?:like|love|enjoy|prefer|am into|'m into|im into|am a fan of|'m a fan of|im a fan of)\\s+(.+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS(stdlib_call("re", "compile", &[py_str("^(?:(?:i|we)\\s*)?(?:'?m|'?re|am|are)?\\s*(?:in\\s+the\\s+mood\\s+for|in\\s+the\\s+humou?r\\s+for|keen\\s+on|up\\s+for|in\\s+the\\s+market\\s+for|fancy|fancied|fancying|'?d\\s+go\\s+for|would\\s+go\\s+for)\\s+(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING(stdlib_call("re", "compile", &[py_str("^(?:something|anything)\\s+(\\S.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__VAGUE_LEAD(stdlib_call("re", "compile", &[py_str("^(?:something|anything|some|any|a|an|the)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_SOMETHING(stdlib_call("re", "compile", &[py_str("\\?|^(?:what|how|why|when|where|which|who|can|could|should|do|does)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED(stdlib_call("re", "compile", &[py_str("\\b(?:not|no|never|none|nope|hate|hates|dislike|dislikes|instead|rather|except|apart|prefer|prefers|can'?t stand|cannot stand|stood|sick of|bored of|tired of|over|without|anything but)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEDIUM(py_tuple(vec![py_tuple(vec![py_str("books"), stdlib_call("re", "compile", &[py_str("\\b(?:book|books|novel|novels|read|reading|author|paperback)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("films"), stdlib_call("re", "compile", &[py_str("\\b(?:film|films|movie|movies|watch|watching|series|tv|cinema)\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    s__TITLE(stdlib_call("re", "compile", &[py_str("\\*[^*\\n]{2,}\\*")]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAD(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|good|nice|great)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(py_tuple(vec![py_str("you"), py_str("me"), py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("him"), py_str("her")]));
    s_MOST_WORDS(Py::Int(4));
    s_MEDIUM_WORDS(py_dict(vec![(py_str("books"), stdlib_call("re", "compile", &[py_str("\\b(?:books?|novels?|read(?:ing|s)?|author|paperback|stor(?:y|ies))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("films"), stdlib_call("re", "compile", &[py_str("\\b(?:films?|movies?|watch(?:ing)?|series|tv|cinema|documentar(?:y|ies))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
}
pub fn _offered_choices(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the answer named things to choose between (\"*Groundhog Day*, *Knives Out* ...\").");
    return ge(&call_builtin("len", &[mcall(&g__TITLE(), "findall", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _said_the_medium(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut at: Py = Py::None;
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let mut medium: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The medium a line is about (\"books\", \"films\"), or \"\" - the earliest one named.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_str(""), Py::None]), 2, None);
    best = __unpk1[0].clone();
    at = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_MEDIUM());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    medium = __unpk3[0].clone();
    pattern = __unpk3[1].clone();
    found = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&at.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&mcall(&found.clone(), "start", &[]), &at.clone()) } } } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![medium.clone(), mcall(&found.clone(), "start", &[])]), 2, None);
    best = __unpk4[0].clone();
    at = __unpk4[1].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _last_turn(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What was asked and what was answered, which is where the medium is written.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    return py_tuple(vec![call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _taste(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What they like, in a few words, or \"\" when the line is not only a taste.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g__ASKS_SOMETHING(), "search", &[said.clone()])) {
    return py_str("");
    }
    found = { let __b = mcall(&g_TASTE(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WANTS(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SOMETHING(), "match", &[said.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    taste = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" .,!?")]);
    if py_truthy(&mcall(&g_REFUSED(), "search", &[said.clone()])) {
    return py_str("");
    }
    while py_truthy(&mcall(&g__LEAD(), "match", &[taste.clone()])) {
    taste = mcall(&g__LEAD(), "sub", &[py_str(""), taste.clone()]);
    }
    while py_truthy(&mcall(&g__VAGUE_LEAD(), "match", &[taste.clone()])) {
    taste = mcall(&g__VAGUE_LEAD(), "sub", &[py_str(""), taste.clone()]);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&taste.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))) {
    return py_str("");
    }
    }
    if py_truthy(&_said_the_medium(&[taste.clone()])) {
    return py_str("");
    }
    return taste.clone();
    Py::None
}
pub fn about_the_medium(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answered: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut medium: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the answer a taste bought is about the medium the taste was put in.\n\n    True when there is no medium to check against, so the caller's own guards decide.\n    ");
    let __unpk1 = py_unpack(&_last_turn(&[memory.clone()]), 2, None);
    asked = __unpk1[0].clone();
    answered = __unpk1[1].clone();
    medium = { let __b = _said_the_medium(&[asked.clone()]); if py_truthy(&__b) { __b } else { _said_the_medium(&[answered.clone()]) } };
    pattern = if py_truthy(&medium.clone()) { mcall(&g_MEDIUM_WORDS(), "get", &[medium.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    said = call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g__TITLE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&pattern.clone(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn served_already(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut taste: Py = Py::None;
    let _ = py_str("True when this is the same taste as the last turn, which was already answered with picks.\n\n    \"i like sci fi\" said twice running is one request, served once: asked again it would reach the\n    same entry and hand the same list back with a \"Same as before\" in front of it, which is the\n    bot talking at the person. A different taste in the same place (\"i like crime\" after the sci\n    fi films) has not been served and is not this.\n    ");
    taste = _taste(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&taste.clone()))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&_last_turn(&[memory.clone()]), 2, None);
    asked = __unpk1[0].clone();
    answered = __unpk1[1].clone();
    return { let __b = Py::Bool(py_eq(&_taste(&[asked.clone()]), &taste.clone())); if !py_truthy(&__b) { __b } else { _offered_choices(&[answered.clone()]) } };
    Py::None
}
pub fn follow_up(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut medium: Py = Py::None;
    let mut taste: Py = Py::None;
    let _ = py_str("The ask this line carries (\"any good crime books\"), or \"\" when it carries none.");
    taste = _taste(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&taste.clone()))) {
    return py_str("");
    }
    if py_truthy(&served_already(&[text.clone(), memory.clone()])) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&_last_turn(&[memory.clone()]), 2, None);
    asked = __unpk1[0].clone();
    answered = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_offered_choices(&[answered.clone()])))) {
    return py_str("");
    }
    medium = { let __b = _said_the_medium(&[asked.clone()]); if py_truthy(&__b) { __b } else { _said_the_medium(&[answered.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&medium.clone()))) {
    return py_str("");
    }
    return add(&add(&add(&py_str("any good "), &taste.clone()), &py_str(" ")), &medium.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.prefer".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "MEDIUM" => Some(g_MEDIUM()),
            "MEDIUM_WORDS" => Some(g_MEDIUM_WORDS()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "REFUSED" => Some(g_REFUSED()),
            "SOMETHING" => Some(g_SOMETHING()),
            "TASTE" => Some(g_TASTE()),
            "WANTS" => Some(g_WANTS()),
            "_ASKS_SOMETHING" => Some(g__ASKS_SOMETHING()),
            "_LEAD" => Some(g__LEAD()),
            "_TITLE" => Some(g__TITLE()),
            "_VAGUE_LEAD" => Some(g__VAGUE_LEAD()),
            "_last_turn" => Some(py_func("_last_turn", |args: &[Py]| _last_turn(args))),
            "_offered_choices" => Some(py_func("_offered_choices", |args: &[Py]| _offered_choices(args))),
            "_said_the_medium" => Some(py_func("_said_the_medium", |args: &[Py]| _said_the_medium(args))),
            "_taste" => Some(py_func("_taste", |args: &[Py]| _taste(args))),
            "about_the_medium" => Some(py_func("about_the_medium", |args: &[Py]| about_the_medium(args))),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            "served_already" => Some(py_func("served_already", |args: &[Py]| served_already(args))),
            _ => None,
        }),
        set: None,
    }))
}

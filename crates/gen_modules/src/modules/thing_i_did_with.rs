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


thread_local! { static G_ATTRIBUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DID_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAS_IT_I: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ATTRIBUTE() -> Py { ensure_init(); G_ATTRIBUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTRIBUTE(v: Py) { ensure_init(); G_ATTRIBUTE.with(|c| *c.borrow_mut() = v); }
pub fn g_DID_WITH() -> Py { ensure_init(); G_DID_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DID_WITH(v: Py) { ensure_init(); G_DID_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g_WAS_IT_I() -> Py { ensure_init(); G_WAS_IT_I.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAS_IT_I(v: Py) { ensure_init(); G_WAS_IT_I.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing the person did with, asked back in their own words.\n\n    i ordered a coat in a medium last tuesday\n    its too big across the shoulders\n    ...\n    which coat was it i ordered    -> the shop's coat-buying guide, the card the chat had\n                                      already been given at the top, word for word again\n    what coat did i order          -> the same card behind \"Same as before:\"\n    what size coat did i order     -> the clothes size chart\n\nThe ask names a thing they told the chat about and asks what they did with it. Nothing in it is a\nword the store reads as a question about the conversation (\"say\", \"told\", \"remember\", \"again\"), so\nno reader of what was said claimed the line, and the answers entry that owns the word \"coat\" - or\n\"size\" - took the turn and printed a card. Their own line is the answer.\n\nThe reading is narrow on purpose, and every part of it is needed:\n\n* the shape is the thing **in front of** what they did - `what|which <thing> did i <verb>`, and\n  `which <thing> was it i <verb>`. A `say`, `tell` or `mention` in the verb place is the ask for\n  their own words, which has its own reader;\n* a word in front of the thing that names which part of it is asked about (\"what size coat\") is\n  dropped: no line of theirs says \"size\", theirs says \"in a medium\". With only the attribute and\n  no thing (\"and what size did i say\") there is nothing to look a line up by, and the turn is\n  left alone;\n* **and the thing has to be in one of their own lines.** That is what makes this a recall rather\n  than a question about the world: `what car did i buy` with no car said is still the motoring\n  card, and it is left to the knowledge base.\n\nThe answer is the store's own idiom for their line (\"You said: ...\"), which is what the store\nanswers a recall with, and it is the whole line rather than a rewording: what they want back is\nthe detail they typed - the size, the day, the name - and a rewrite is where a detail goes\nmissing.\n");
    s_PRIORITY(Py::Int(6900));
    s_LEAD(py_str("^(?:and |so |but |also |oh |ok |okay )?"));
    s_DID_WITH(stdlib_call("re", "compile", &[op_add(&g_LEAD(), &py_str("(?:what|which) (?P<thing>[a-z][a-z' ]{1,24}?)\\s*(?:did i|have i)\\s+(?!said?\\b|tell\\b|told\\b|mention\\b|mentioning\\b)(?P<did>[a-z]+)\\s*[?.!]*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAS_IT_I(stdlib_call("re", "compile", &[op_add(&g_LEAD(), &py_str("which (?P<thing>[a-z][a-z' ]{1,24}?)\\s*(?:was it|is it|was that)\\s+(?:that\\s+)?i\\s+(?!said?\\b|tell\\b|told\\b|mention\\b)(?P<did>[a-z]+)\\s*[?.!]*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_ATTRIBUTE(call_builtin("frozenset", &[mcall(&py_str("size colour color brand make model number price cost amount weight\nlength width height age year month day date time"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("one thing things bit bits part parts point way kind sort time other\nelse lot lots"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(g_takes_as_it_comes());
}
pub fn _thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the ask is about, or \"\" when the line is not this shape.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_WAS_IT_I(), g_DID_WITH()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ATTRIBUTE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ATTRIBUTE(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &thing.clone())) } } } }) {
    return py_str("");
    }
    return thing.clone();
    }
    return py_str("");
    Py::None
}
pub fn _did(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The doing in the ask: \"what coat did i order\" -> \"order\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_WAS_IT_I(), g_DID_WITH()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "match", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&match_.clone(), "group", &[py_str("did")]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
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
    let mut suffix: Py = Py::None;
    let _ = py_str("The word with a plural or a participle taken off, so \"coat\" finds \"coats\".");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str("'")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[suffix.clone()]) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn _says_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tokens: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tokens") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tokens"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut stems: Py = Py::None;
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every word of the thing is one of these tokens, in any of its forms.");
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens.clone()) { let mut token: Py = __item.clone(); __out.push(_stem(&[token.clone()])); } py_set(__out) })() };
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&thing.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&stems.clone(), &_stem(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut did: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "did") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: did"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut doing: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut token: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line of theirs that names the thing, or \"\".\n\n    A line that asks something is skipped: it names the thing and states nothing, and reading it\n    back would answer the question with a question (\"what coat should i get\" a turn earlier).\n\n    The line that says what they did is the answer, so a line with the doing in it wins over a\n    later line that only names the thing: \"i ordered a coat in a medium\" over \"the coat arrived\n    scratched\", when the ask is which coat they ordered. The newest line is the fallback - people\n    say what they did, then talk about the thing.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_says_it(&[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), thing.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&line.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { _thing(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&line.clone(), "lower", &[]), "split", &[])])]) } }) {
    continue;
    }
    let _ = mcall(&lines.clone(), "append", &[line.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return py_str("");
    }
    doing = _stem(&[did.clone()]);
    let __iter2 = py_iter_or_raise(&lines.clone());
    for __item in __iter2 {
    line = __item.clone();
    tokens = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut token: Py = __item.clone(); __out.push(_stem(&[token.clone()])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&tokens.clone(), &doing.clone()))) {
    return line.clone();
    }
    }
    return geti(&lines.clone(), &Py::Int(0));
    Py::None
}
pub fn _times_said(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("How many of the last few replies already carry this line.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    said = mcall(&line.clone(), "lower", &[]);
    return call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_recall(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut times: Py = Py::None;
    let _ = py_str("The reply for a line that asks after what they did with a thing, or None.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    thing = _thing(&[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    line = _the_line(&[thing.clone(), _did(&[low.clone()]), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    quoted = op_add(&op_add(&py_str("\""), &line.clone()), &py_str("\""));
    times = _times_said(&[line.clone(), memory.clone()]);
    if py_truthy(&times.clone()) {
    return op_add(&geti(&py_tuple(vec![py_str("Still the same one: "), py_str("Same line again: ")]), &op_modulo(&times.clone(), &Py::Int(2))), &quoted.clone());
    }
    return op_add(&py_str("You said: "), &quoted.clone());
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("True on exactly the lines `respond` answers, and nowhere else.\n\n    The coat guide declares the topic a coat chat is read as, and a card that declares the line\n    takes it off a module underneath it whatever the bands say (`arbiter._as_it_comes`). On these\n    lines the ask is about what the person did, so nothing about the world can answer it: the\n    module says so rather than raising `PRIORITY` over every other line it reads.\n    ");
    return Py::Bool(!py_is(&_the_recall(&[text.clone(), memory.clone()]), &Py::None));
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
    let mut low: Py = Py::None;
    let _ = py_str("The thing they did with, when they told the chat about it, else None.\n\n    Never a line the store can already answer. `what phone have i got` after \"my phone is a\n    pixel\" is a recall the store reads better than this does - it says the thing back in the\n    chat's own words (\"Your phone is a Pixel.\") where this hands back the line it was said in\n    (\"You said: \"my phone is a pixel\"\"). The ask is the same shape as the coat one that this\n    module exists for, so the only difference between them is which reader has the line, and the\n    reader that already has it wins.\n    ");
    let mut memory_store: Py = crate::modules::memory_store::module_obj();
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" ,.;:!?")]);
    if py_truthy(&mcall(&memory_store.clone(), "respond", &[low.clone(), memory.clone()])) {
    return Py::None;
    }
    return _the_recall(&[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.thing_i_did_with".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ATTRIBUTE" => Some(g_ATTRIBUTE()),
            "DID_WITH" => Some(g_DID_WITH()),
            "LEAD" => Some(g_LEAD()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "WAS_IT_I" => Some(g_WAS_IT_I()),
            "_did" => Some(py_func("_did", |args: &[Py]| _did(args))),
            "_says_it" => Some(py_func("_says_it", |args: &[Py]| _says_it(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_line" => Some(py_func("_the_line", |args: &[Py]| _the_line(args))),
            "_the_recall" => Some(py_func("_the_recall", |args: &[Py]| _the_recall(args))),
            "_thing" => Some(py_func("_thing", |args: &[Py]| _thing(args))),
            "_times_said" => Some(py_func("_times_said", |args: &[Py]| _times_said(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ATTRIBUTE" => s_ATTRIBUTE(v),
            "DID_WITH" => s_DID_WITH(v),
            "LEAD" => s_LEAD(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "PRIORITY" => s_PRIORITY(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            "WAS_IT_I" => s_WAS_IT_I(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.thing_i_did_with", module_obj);
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_thing", py_func("_thing", |args: &[Py]| _thing(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_did", py_func("_did", |args: &[Py]| _did(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_says_it", py_func("_says_it", |args: &[Py]| _says_it(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_the_line", py_func("_the_line", |args: &[Py]| _the_line(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_times_said", py_func("_times_said", |args: &[Py]| _times_said(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "_the_recall", py_func("_the_recall", |args: &[Py]| _the_recall(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("modules.thing_i_did_with", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "ATTRIBUTE", g_ATTRIBUTE, s_ATTRIBUTE);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "DID_WITH", g_DID_WITH, s_DID_WITH);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.thing_i_did_with", "WAS_IT_I", g_WAS_IT_I, s_WAS_IT_I);
}

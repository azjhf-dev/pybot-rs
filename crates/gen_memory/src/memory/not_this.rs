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


thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_AFTER_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_THE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BE_WILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT() -> Py { ensure_init(); G_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT(v: Py) { ensure_init(); G_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_AFTER_A_QUESTION() -> Py { ensure_init(); G_OUT_AFTER_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_AFTER_A_QUESTION(v: Py) { ensure_init(); G_OUT_AFTER_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS() -> Py { ensure_init(); G_STANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS(v: Py) { ensure_init(); G_STANDS.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_THE_NAME() -> Py { ensure_init(); G__AFTER_THE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_THE_NAME(v: Py) { ensure_init(); G__AFTER_THE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g__BE_WILL() -> Py { ensure_init(); G__BE_WILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BE_WILL(v: Py) { ensure_init(); G__BE_WILL.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTRACTIONS() -> Py { ensure_init(); G__CONTRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTRACTIONS(v: Py) { ensure_init(); G__CONTRACTIONS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that only rules a place out is not a line about that place.\n\n```\nwe want to go to italy / not rome   -> \"In Rome: the Colosseum with a timed ticket, the Pantheon...\"\nim going to spain / not portugal    -> \"Portugal: Lisbon for trams, tarts and the river...\"\n```\n\nThe person took the place back and the bot answered with the card for the place they had just\nruled out. It reads as if it had not heard them, and it is the one reply that cannot help: what\nthey want is somewhere else.\n\n`bot/compose/ruledout.py` reads the shape of a rule-out a thing arrives in - \"not a series\",\n\"not the chemical stuff\", \"its not the trap\" - and every arm of it needs a determiner or a plural,\nso a bare name slips through: \"not portugal\" and \"not rome\" name a place with no article and no\nplural, and were read as a fresh mention of it. A name is the one kind of thing that needs no\ndeterminer, so it is read here, where the store already knows which words are places\n(`bot.memory.facts.place_value`, and the towns and countries the knowledge base has a card for).\n\nThe reply says what is left standing: the other place the chat is on (\"Spain, then - no Portugal\"),\nand when nothing else is on the table it says the place is out and asks where instead. A line that\nnames a thing that is not a place is left to the readers that know about it.\n");
    s__AFTER_THE_NAME(py_str("(?:\\s*,?\\s*(?:then|though|after all|actually|really))?"));
    s_NOT_A_PLACE(py_tuple(vec![stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and\\s+|but\\s+|so\\s+|ok\\s+|okay\\s+)?(?:no,?\\s+|nope,?\\s+|nah,?\\s+)?(?:definitely\\s+|absolutely\\s+|really\\s+|probably\\s+|actually\\s+)?not\\s+(?:to\\s+)?(?P<place>[a-z][a-z' -]{1,24}?)"), &g__AFTER_THE_NAME()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and\\s+|but\\s+|so\\s+|ok\\s+|okay\\s+)?(?:no|nope|nah),?\\s+(?:definitely\\s+|absolutely\\s+|really\\s+|probably\\s+|actually\\s+)?(?P<place>[a-z][a-z' -]{1,24}?)"), &g__AFTER_THE_NAME()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and\\s+|but\\s+)?i\\s+(?:said|meant|told you)\\s+not\\s+(?P<place>[a-z][a-z' -]{1,24}?)"), &g__AFTER_THE_NAME()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:and\\s+|but\\s+)?(?:we|i|they|he|she)\\s+(?:am|are|is|was|were|will|would|re|m|s)?\\s*not\\s+(?:going|heading|off|flying|driving)\\s+(?:to|into)\\s+(?P<place>[a-z][a-z' -]{0,24}?)\\s*(?:anymore|any\\s+more|now|then|after\\s+all|this\\s+time)?$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:and\\s+|but\\s+)?(?:we|i)\\s+(?:dont|don't|do not)\\s+(?:want|fancy)\\s+(?:to go to\\s+)?(?P<place>[a-z][a-z' -]{1,24}?)"), &g__AFTER_THE_NAME()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e))]));
    s__BE_WILL(py_str("are|is|were|was|do|does|did|could|would|should|will|can|must"));
    s__CONTRACTIONS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(we|i|they|he|she|it|you)(?:'re|re)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("\\1 are")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(i)(?:'m|m)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("\\1 am")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(we|i|they|he|she|it)(?:'s|s)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("\\1 is")]), py_tuple(vec![stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &g__BE_WILL()), &py_str(")n'?t\\b"))]).unwrap_or_else(|e| exc::raise(e)), py_str("\\1 not")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\baint\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("is not")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bim\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("i am")])]));
    s_STANDS(py_tuple(vec![py_str("{place}, then - no {gone}."), py_str("Right - {place}, not {gone}."), py_str("So {place} it is, without {gone}."), py_str("{place} on its own, then.")]));
    s_OUT(py_tuple(vec![py_str("No {gone}, then. Where are you thinking instead?"), py_str("Fair enough - not {gone}. What are you after instead?"), py_str("{gone} is out, then. Where else are you looking?")]));
    s_OUT_AFTER_A_QUESTION(py_tuple(vec![py_str("No {gone}, then - say where instead and I will go with that."), py_str("Fair enough - not {gone}. Tell me where you are looking and I will help from there."), py_str("{gone} is out, then. Point me at somewhere else when you know it.")]));
}
pub fn _the_place_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut gone: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "gone") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: gone"))) };
    let mut held: Py = Py::None;
    let mut named: Py = Py::None;
    let mut place: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("The place the chat holds that is not the one they ruled out, or \"\".\n\n    What the store keeps is the line they typed (\"Go to italy\"), not a name, and a name is what\n    the reply has a slot for: \"So Go to italy it is, without Rome.\" puts a capital mid-sentence and\n    a whole sentence where \"Italy\" belongs. The place is named the way the knowledge base holds\n    it, and a stored line with no place in it is no place to say.\n    ");
    held = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    place = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[place.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&place.clone(), "strip", &[]) } }) {
    let _ = mcall(&held.clone(), "append", &[mcall(&place.clone(), "strip", &[])]);
    }
    }
    plan = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "strip", &[]) } }) {
    let _ = mcall(&held.clone(), "append", &[mcall(&plan.clone(), "strip", &[])]);
    }
    let __iter2 = py_iter_or_raise(&held.clone());
    for __item in __iter2 {
    place = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&place.clone(), "lower", &[]), &mcall(&gone.clone(), "lower", &[])))) {
    continue;
    }
    named = _the_name_in(&[place.clone(), gone.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_name_in(args: &[Py]) -> Py {
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
    let mut gone: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "gone") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: gone"))) };
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place the knowledge base knows inside a stored line, or \"\".");
    whole = _a_place_the_chat_knows(&[said.clone()]);
    if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&whole.clone(), "lower", &[]), &mcall(&gone.clone(), "lower", &[]))) } }) {
    return whole.clone();
    }
    found = py_str("");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z][a-z']*"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &mcall(&gone.clone(), "lower", &[])))) {
    continue;
    }
    named = _a_place_the_chat_knows(&[word.clone()]);
    if py_truthy(&named.clone()) {
    found = named.clone();
    }
    }
    return found.clone();
    Py::None
}
pub fn _a_place_in_play(args: &[Py]) -> Py {
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
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the chat is on a place, so a ruled-out place is a place they were going to.\n\n    \"im going to spain\" / \"not portugal\" is a person narrowing a trip, and the store holds Spain.\n    \"i watched a film last night\" / \"not rome\" is a place name out of nowhere: nothing in the ~chat\n    is about somewhere, and a wire reply about Rome would be about a conversation that never\n    happened, so the line is left to the readers that know what it is about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    held = crate::memory::user(&[memory.clone()]);
    if py_truthy(&{ let __b = mcall(&held.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&held.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { mcall(&held.clone(), "get", &[py_str("city")]) } } } }) {
    return Py::Bool(true);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![mcall(&turn.clone(), "get", &[py_str("user")]), mcall(&turn.clone(), "get", &[py_str("bot")])]));
    for __item in __iter2 {
    said = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z][a-z']*"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&word.clone(), &mcall(&place.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { _a_place_the_chat_knows(&[word.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asked_last(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's own last line asked something.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&last.clone(), "get", &[py_str("bot")]) } else { Py::None };
    return { let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "strip", &[]), "endswith", &[py_str("?")]) } };
    Py::None
}
pub fn _a_place_the_chat_knows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The name written as the knowledge base holds it, or \"\" for a word that is no place.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("knowledge.travel", "CITIES"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("knowledge.travel", "CITY_COUNTRY"), &word.clone())) } }) {
    return { let __b = crate::memory::facts::place_value(&[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "capitalize", &[]) } };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&call_builtin("set", &[mcall(&pyrt::xrefs::xget("knowledge.travel", "CITY_COUNTRY"), "values", &[])]).unwrap_or_else(|e| exc::raise(e)), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("knowledge.travel", "CAPITALS"), &word.clone())) } }) {
    return mcall(&word.clone(), "capitalize", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _in_plain_words(args: &[Py]) -> Py {
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
    let mut long_form: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("The line in long form, lower case, single-spaced and with its end stop taken off.");
    line = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    let __iter1 = py_iter_or_raise(&g__CONTRACTIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    shape = __unpk2[0].clone();
    long_form = __unpk2[1].clone();
    line = mcall(&shape.clone(), "sub", &[long_form.clone(), line.clone()]);
    }
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&line.clone(), "split", &[])]), "rstrip", &[py_str(".!,")]);
    Py::None
}
pub fn the_place_ruled_out(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut place: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("The place a line of nothing but its rejection names, or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    line = _in_plain_words(&[text.clone()]);
    line = mcall(&line.clone(), "rstrip", &[py_str(".!,")]);
    let __iter1 = py_iter_or_raise(&g_NOT_A_PLACE());
    for __item in __iter1 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    place = _a_place_the_chat_knows(&[mcall(&found.clone(), "group", &[py_str("place")])]);
    if py_truthy(&place.clone()) {
    return place.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_place_ruled_out(args: &[Py]) -> Py {
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
    let mut fresh: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said_lately: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut stands: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a line that only rules a place out, or None.");
    gone = the_place_ruled_out(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gone.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    stands = _the_place_the_chat_is_on(&[memory.clone(), gone.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stands.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_place_in_play(&[memory.clone(), gone.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&stands.clone()) {
    pool = g_STANDS();
    } else {
    if py_truthy(&_asked_last(&[memory.clone()])) {
    pool = g_OUT_AFTER_A_QUESTION();
    } else {
    pool = g_OUT();
    }
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut shape: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), stands.clone()), ("gone".to_string(), gone.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shape.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    turns = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("turns")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { py_list(vec![]) };
    said_lately = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&turn.clone(), "get", &[py_str("bot")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(mcall(&turn.clone(), "get", &[py_str("bot")])); } } py_set(__out) })() };
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said_lately.clone(), &line.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { lines.clone() } };
    return pyrt::xrefs::xcall("compose", "pick", &[fresh.clone(), memory.clone(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut ruled_out: Py = Py::None;
    let _ = py_str("The reply to a line that only rules a place out, or a thing said again with \"still\", or None.\n\n    Both readings are the store's before any card is opened, and the store asks this reader first,\n    so they are answered here in one call. The still reading is `bot.memory.tried`'s: it is a\n    follow-up on the chat's own thing, and the one word the line carries (\"cold\", \"freezing\")\n    must not open a card about something else.\n    ");
    ruled_out = _the_place_ruled_out(&[text.clone(), memory.clone()]);
    if py_truthy(&ruled_out.clone()) {
    return ruled_out.clone();
    }
    let mut tried: Py = crate::memory::tried::module_obj();
    return mcall(&tried.clone(), "reply_for", &[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.not_this".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "OUT" => Some(g_OUT()),
            "OUT_AFTER_A_QUESTION" => Some(g_OUT_AFTER_A_QUESTION()),
            "STANDS" => Some(g_STANDS()),
            "_AFTER_THE_NAME" => Some(g__AFTER_THE_NAME()),
            "_BE_WILL" => Some(g__BE_WILL()),
            "_CONTRACTIONS" => Some(g__CONTRACTIONS()),
            "_a_place_in_play" => Some(py_func("_a_place_in_play", |args: &[Py]| _a_place_in_play(args))),
            "_a_place_the_chat_knows" => Some(py_func("_a_place_the_chat_knows", |args: &[Py]| _a_place_the_chat_knows(args))),
            "_asked_last" => Some(py_func("_asked_last", |args: &[Py]| _asked_last(args))),
            "_in_plain_words" => Some(py_func("_in_plain_words", |args: &[Py]| _in_plain_words(args))),
            "_the_name_in" => Some(py_func("_the_name_in", |args: &[Py]| _the_name_in(args))),
            "_the_place_ruled_out" => Some(py_func("_the_place_ruled_out", |args: &[Py]| _the_place_ruled_out(args))),
            "_the_place_the_chat_is_on" => Some(py_func("_the_place_the_chat_is_on", |args: &[Py]| _the_place_the_chat_is_on(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_place_ruled_out" => Some(py_func("the_place_ruled_out", |args: &[Py]| the_place_ruled_out(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "OUT" => s_OUT(v),
            "OUT_AFTER_A_QUESTION" => s_OUT_AFTER_A_QUESTION(v),
            "STANDS" => s_STANDS(v),
            "_AFTER_THE_NAME" => s__AFTER_THE_NAME(v),
            "_BE_WILL" => s__BE_WILL(v),
            "_CONTRACTIONS" => s__CONTRACTIONS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.not_this", module_obj);
    pyrt::xrefs::reg_fn("memory.not_this", "_the_place_the_chat_is_on", py_func("_the_place_the_chat_is_on", |args: &[Py]| _the_place_the_chat_is_on(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_the_name_in", py_func("_the_name_in", |args: &[Py]| _the_name_in(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_a_place_in_play", py_func("_a_place_in_play", |args: &[Py]| _a_place_in_play(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_asked_last", py_func("_asked_last", |args: &[Py]| _asked_last(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_a_place_the_chat_knows", py_func("_a_place_the_chat_knows", |args: &[Py]| _a_place_the_chat_knows(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_in_plain_words", py_func("_in_plain_words", |args: &[Py]| _in_plain_words(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "the_place_ruled_out", py_func("the_place_ruled_out", |args: &[Py]| the_place_ruled_out(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "_the_place_ruled_out", py_func("_the_place_ruled_out", |args: &[Py]| _the_place_ruled_out(args)));
    pyrt::xrefs::reg_fn("memory.not_this", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.not_this", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.not_this", "OUT", g_OUT, s_OUT);
    pyrt::xrefs::reg_access("memory.not_this", "OUT_AFTER_A_QUESTION", g_OUT_AFTER_A_QUESTION, s_OUT_AFTER_A_QUESTION);
    pyrt::xrefs::reg_access("memory.not_this", "STANDS", g_STANDS, s_STANDS);
    pyrt::xrefs::reg_access("memory.not_this", "_AFTER_THE_NAME", g__AFTER_THE_NAME, s__AFTER_THE_NAME);
    pyrt::xrefs::reg_access("memory.not_this", "_BE_WILL", g__BE_WILL, s__BE_WILL);
    pyrt::xrefs::reg_access("memory.not_this", "_CONTRACTIONS", g__CONTRACTIONS, s__CONTRACTIONS);
}

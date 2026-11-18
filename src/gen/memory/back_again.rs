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


pub static AFTER_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AWAY_FROM_HOME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_OR_PET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SYMPTOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETERMINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUN_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNNAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AFTER_BACK() -> Py { ensure_init(); AFTER_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_BACK(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_BACK.write() { *w = v; } }
pub fn g_AWAY_FROM_HOME() -> Py { ensure_init(); AWAY_FROM_HOME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AWAY_FROM_HOME(v: Py) { ensure_init(); if let Ok(mut w) = AWAY_FROM_HOME.write() { *w = v; } }
pub fn g_A_PERSON_OR_PET() -> Py { ensure_init(); A_PERSON_OR_PET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OR_PET(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_OR_PET.write() { *w = v; } }
pub fn g_A_SYMPTOM() -> Py { ensure_init(); A_SYMPTOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SYMPTOM(v: Py) { ensure_init(); if let Ok(mut w) = A_SYMPTOM.write() { *w = v; } }
pub fn g_BARE_BACK() -> Py { ensure_init(); BARE_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_BACK(v: Py) { ensure_init(); if let Ok(mut w) = BARE_BACK.write() { *w = v; } }
pub fn g_DETERMINERS() -> Py { ensure_init(); DETERMINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); if let Ok(mut w) = DETERMINERS.write() { *w = v; } }
pub fn g_NAMED() -> Py { ensure_init(); NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = NAMED.write() { *w = v; } }
pub fn g_PRONOUN_BACK() -> Py { ensure_init(); PRONOUN_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_BACK(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUN_BACK.write() { *w = v; } }
pub fn g_UNNAMED() -> Py { ensure_init(); UNNAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNNAMED(v: Py) { ensure_init(); if let Ok(mut w) = UNNAMED.write() { *w = v; } }
pub fn g_VERB_BACK() -> Py { ensure_init(); VERB_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_BACK(v: Py) { ensure_init(); if let Ok(mut w) = VERB_BACK.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A trouble the chat already had, said to be back.\n\nFour chats, one shape:\n\n    i have a headache              / its back                -> \"Got it. Is there anything you'd like a hand with?\"\n    how do i get rid of slugs      / the slugs are back      -> \"Got it - the slugs are back.\"\n    my boiler is making a banging noise / the banging is back -> \"Thanks for telling me - I'm here if you want to dig into any of that.\"\n    i think ive got a cold         / its come back           -> the nod, or the cough card by luck\n\nThe person is not opening a subject or changing one: they are reporting that the thing the chat\nwas working on has returned, and that says something the bot should pick up - the advice it gave\ndid not hold, or the thing never really went. The reply has to reach the thing and say so. The nod\n(\"Got it. Is there anything you'd like a hand with?\") is the reply nobody wants, and the echo\n(\"Got it - the slugs are back.\") tells them what they just said.\n\nWhat is left alone:\n\n* a line that asks instead of reporting (\"is it back?\", \"will it come back?\") - that is a question\n  about the thing, and the cards answer it;\n* a person or an animal said to be back (\"my mum is back\", \"the cat is back\") - good news or a\n  separate subject, never a returning trouble;\n* a thing the chat never had (nothing to be back from);\n* \"im back\" and \"we're back\", which are the person, not the thing.\n");
    s_AFTER_BACK(stdlib_call("re", "compile", &[py_str("^(?:\\s*(?:again|already|now|too|so soon|as well|for the (?:\\w+ )?time|and (?:its|it is|its worse|worse|its back|the \\w+ is|its the same|same as before)|its worse|it is worse|worse than (?:before|last time)|same as before))*\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_BACK(stdlib_call("re", "compile", &[py_str("^(?:and|so|well|oh|ok|okay|right)?\\s*(?:its|it's|it is|theyre|they're|they are|thats|that's|that is|these are|those are)\\s+(?:come\\s+|came\\s+)?back\\b(?P<tail>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_BACK(stdlib_call("re", "compile", &[py_str("^(?:(?P<cop>is|are|was|were|s|'s|has|have|had)\\s+(?:come\\s+)?back|(?P<main>came|come|comes|returned|returns)\\s+back|(?P<alone>returned|returns))\\b(?P<tail>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DETERMINERS(py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that")]));
    s_BARE_BACK(stdlib_call("re", "compile", &[py_str("^(?:and|so|well|oh|ok|okay)?\\s*back (?:again|already|now)\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_OR_PET(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[crate::gen::memory::facts::g_RELATIONS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str("|dog|puppy|cat|kitten|rabbit|hamster|parrot|goldfish|horse|tortoise|guinea)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED(py_tuple(vec![py_str("{The thing}, back again - did {they} pass at all in between, or {has} {they} just been quiet?"), py_str("{The thing} came back. Same as before, or worse this time?"), py_str("{The thing} again. What did you do for {them} the first time, and how long did that hold?"), py_str("{The thing} being back usually means whatever set it off is still there - has anything changed since?")]));
    s_UNNAMED(py_tuple(vec![py_str("Back again - did it pass at all in between, or has it been there under the surface?"), py_str("So it eased off and came back. Is it the same as before, or worse this time?"), py_str("Back so soon. What did you do for it the first time, and how long did that hold?"), py_str("Something coming back usually means whatever set it off is still there - has anything changed since?")]));
    s_AWAY_FROM_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|are|were|been|being|stayed|sent|took|taken|gone|went|dropped|left)\\s+(?:at|in|to|with|into|off to|round to)\\s+(?:the|a|an|my|our|his|her|their|its)\\b|\\b(?:came out|came round|coming out|booked in|in for a service|getting (?:fixed|repaired|serviced|cleaned|mended|done))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SYMPTOM(stdlib_call("re", "compile", &[py_str("\\b(?:noise|noisy|banging|rumbling|knocking|squeaking|whistling|rattling|humming|buzzing|grinding|clicking|dripping|hissing|scraping|leak|leaking|damp|mould|mold|smell|smells|reek|sore|ache|aches|aching|pain|painful|hurts?|hurting|bleeding|rash|cough|fever|swollen|broken|cracked|fault|faulty|stuck|jammed|stopped working|not working|isnt working|wont work|wont start|slugs?|mice|rats|ants|flies|aphids|weeds?|worse|chest|throat|head|stomach|back|knee|shoulder|ankle|ear|tooth|toothache)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_earlier_line(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut have: Py = Py::None;
    let mut line: Py = Py::None;
    let mut now: Py = Py::None;
    let mut other: Py = Py::None;
    let mut record: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The person's own earlier line about the thing, or \"\" when there is none.");
    wanted = if py_truthy(&thing.clone()) { crate::gen::memory::topics::keywords(&[thing.clone()]) } else { py_list(vec![]) };
    record = crate::gen::memory::topics::current(&[memory.clone()]);
    words = { let __b = call_builtin("list", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("list", &[{ let __b = mcall(&{ let __b = record.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    now = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&line.clone(), "lower", &[]), "split", &[])]), &now.clone()))) {
    continue;
    }
    have = crate::gen::memory::topics::keywords(&[line.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&have.clone()) { let mut other: Py = __item.clone(); __out.push(crate::gen::memory::topics::alike(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_thing_went_away(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut earlier: Py = Py::None;
    let _ = py_str("True when the earlier line about the thing puts it somewhere else, or a caller came out.");
    earlier = _the_earlier_line(&[memory.clone(), thing.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&earlier.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_SYMPTOM(), "search", &[earlier.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_AWAY_FROM_HOME(), "search", &[earlier.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plural: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plural") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plural"))) };
    let _ = py_str("The pronouns a reply needs for one thing or for several.");
    if py_truthy(&plural.clone()) {
    return py_dict(vec![(py_str("they"), py_str("they")), (py_str("them"), py_str("them")), (py_str("has"), py_str("have"))]);
    }
    return py_dict(vec![(py_str("they"), py_str("it")), (py_str("them"), py_str("it")), (py_str("has"), py_str("has"))]);
    Py::None
}
pub fn _filled(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut plural: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plural") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plural"))) };
    let mut key: Py = Py::None;
    let mut out: Py = Py::None;
    let mut value: Py = Py::None;
    let mut words: Py = Py::None;
    words = _the_words(&[plural.clone()]);
    if py_truthy(&thing.clone()) {
    let _ = py_setitem(&words.clone(), &py_str("the thing"), add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)));
    } else {
    let _ = py_setitem(&words.clone(), &py_str("the thing"), py_str("It"));
    }
    out = line.clone();
    let __iter1 = py_iter_or_raise(&mcall(&words.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    out = mcall(&out.clone(), "replace", &[add(&add(&py_str("{"), &key.clone()), &py_str("}")), value.clone()]);
    out = mcall(&out.clone(), "replace", &[add(&add(&add(&py_str("{"), &mcall(&geti(&key.clone(), &Py::Int(0)), "upper", &[])), &slice_or_raise(&key.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("}")), add(&mcall(&geti(&value.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    if py_truthy(&thing.clone()) {
    return out.clone();
    }
    return add(&mcall(&geti(&out.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _an_earlier_topic(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut have: Py = Py::None;
    let mut here: Py = Py::None;
    let mut other: Py = Py::None;
    let mut record: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a turn before this one was about the thing: \"the slugs\" after the slug question.");
    wanted = crate::gen::memory::topics::keywords(&[thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(false);
    }
    here = call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&record.clone(), "get", &[py_str("turn")]), &here.clone()))) {
    continue;
    }
    have = { let __b = mcall(&record.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&have.clone()) { let mut other: Py = __item.clone(); __out.push(crate::gen::memory::topics::alike(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _from_this_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("True when the topic record was written for the line being answered right now.");
    return Py::Bool(py_eq(&mcall(&record.clone(), "get", &[py_str("turn")]), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing said to be back: (the words for it, is it plural), or None.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("so"), py_str("well"), py_str("oh"), py_str("ok"), py_str("okay"), py_str("right")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    found = py_list(vec![]);
    numbers = py_dict(vec![]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("min", &[Py::Int(3), sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    count = __item.clone();
    thing = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &count.clone(), &Py::None)]);
    said = mcall(&g_VERB_BACK(), "match", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &count.clone(), &Py::None, &Py::None)])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AFTER_BACK(), "match", &[{ let __b = mcall(&said.clone(), "group", &[py_str("tail")]); if py_truthy(&__b) { __b } else { py_str("") } }]))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_PERSON_OR_PET(), "search", &[thing.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_an_earlier_topic(&[memory.clone(), thing.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&said.clone(), "group", &[py_str("cop")])) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![Py::Int(0), neg(&count.clone()), thing.clone()])]);
    } else {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![Py::Int(1), neg(&count.clone()), thing.clone()])]);
    }
    if py_truthy(&mcall(&said.clone(), "group", &[py_str("cop")])) {
    number = Py::Bool(py_contains(&py_tuple(vec![py_str("are"), py_str("were"), py_str("have")]), &mcall(&mcall(&said.clone(), "group", &[py_str("cop")]), "lower", &[])));
    } else {
    number = mcall(&geti(&mcall(&thing.clone(), "split", &[]), &neg(&Py::Int(1))), "endswith", &[py_str("s")]);
    }
    let _ = py_setitem(&numbers.clone(), &thing.clone(), number.clone());
    }
    if py_truthy(&found.clone()) {
    let _ = mcall(&found.clone(), "sort", &[]);
    thing = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(2));
    return py_tuple(vec![thing.clone(), geti(&numbers.clone(), &thing.clone())]);
    }
    return Py::None;
    }
    spoken = mcall(&g_PRONOUN_BACK(), "match", &[line.clone()]);
    if py_truthy(&spoken.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AFTER_BACK(), "match", &[{ let __b = mcall(&spoken.clone(), "group", &[py_str("tail")]); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    record = crate::gen::memory::topics::current(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&record.clone())); if py_truthy(&__b) { __b } else { _from_this_line(&[record.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_PERSON_OR_PET(), "search", &[mcall(&py_str(" "), "join", &[{ let __b = mcall(&record.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])])) {
    return Py::None;
    }
    plural = mcall(&line.clone(), "startswith", &[py_tuple(vec![py_str("theyre"), py_str("they're"), py_str("they are"), py_str("these"), py_str("those")])]);
    return py_tuple(vec![Py::None, plural.clone()]);
    }
    if py_truthy(&mcall(&g_BARE_BACK(), "match", &[line.clone()])) {
    record = crate::gen::memory::topics::current(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&record.clone())); if py_truthy(&__b) { __b } else { _from_this_line(&[record.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    }
    return Py::None;
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reply to a trouble said to be back, or None when the line is something else.");
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::memory::history::an_earlier_turn(&[memory.clone(), text.clone()])))) {
    return Py::None;
    }
    found = _the_thing(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    thing = __unpk1[0].clone();
    plural = __unpk1[1].clone();
    if py_truthy(&_the_thing_went_away(&[memory.clone(), thing.clone(), text.clone()])) {
    return Py::None;
    }
    line = stdlib_call("random", "choice", &[if py_truthy(&thing.clone()) { g_NAMED() } else { g_UNNAMED() }]).unwrap_or_else(|e| exc::raise(e));
    return _filled(&[line.clone(), thing.clone(), plural.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.back_again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_BACK" => Some(g_AFTER_BACK()),
            "AWAY_FROM_HOME" => Some(g_AWAY_FROM_HOME()),
            "A_PERSON_OR_PET" => Some(g_A_PERSON_OR_PET()),
            "A_SYMPTOM" => Some(g_A_SYMPTOM()),
            "BARE_BACK" => Some(g_BARE_BACK()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "NAMED" => Some(g_NAMED()),
            "PRONOUN_BACK" => Some(g_PRONOUN_BACK()),
            "UNNAMED" => Some(g_UNNAMED()),
            "VERB_BACK" => Some(g_VERB_BACK()),
            "_an_earlier_topic" => Some(py_func("_an_earlier_topic", |args: &[Py]| _an_earlier_topic(args))),
            "_filled" => Some(py_func("_filled", |args: &[Py]| _filled(args))),
            "_from_this_line" => Some(py_func("_from_this_line", |args: &[Py]| _from_this_line(args))),
            "_the_earlier_line" => Some(py_func("_the_earlier_line", |args: &[Py]| _the_earlier_line(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_thing_went_away" => Some(py_func("_the_thing_went_away", |args: &[Py]| _the_thing_went_away(args))),
            "_the_words" => Some(py_func("_the_words", |args: &[Py]| _the_words(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

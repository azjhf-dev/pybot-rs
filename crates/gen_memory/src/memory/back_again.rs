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


thread_local! { static G_AFTER_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AWAY_FROM_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_OR_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AFTER_BACK() -> Py { ensure_init(); G_AFTER_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_BACK(v: Py) { ensure_init(); G_AFTER_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_AWAY_FROM_HOME() -> Py { ensure_init(); G_AWAY_FROM_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AWAY_FROM_HOME(v: Py) { ensure_init(); G_AWAY_FROM_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_OR_PET() -> Py { ensure_init(); G_A_PERSON_OR_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OR_PET(v: Py) { ensure_init(); G_A_PERSON_OR_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SYMPTOM() -> Py { ensure_init(); G_A_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SYMPTOM(v: Py) { ensure_init(); G_A_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_BACK() -> Py { ensure_init(); G_BARE_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_BACK(v: Py) { ensure_init(); G_BARE_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED() -> Py { ensure_init(); G_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); G_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_BACK() -> Py { ensure_init(); G_PRONOUN_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_BACK(v: Py) { ensure_init(); G_PRONOUN_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_UNNAMED() -> Py { ensure_init(); G_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNNAMED(v: Py) { ensure_init(); G_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_BACK() -> Py { ensure_init(); G_VERB_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_BACK(v: Py) { ensure_init(); G_VERB_BACK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A trouble the chat already had, said to be back.\n\nFour chats, one shape:\n\n    i have a headache              / its back                -> \"Got it. Is there anything you'd like a hand with?\"\n    how do i get rid of slugs      / the slugs are back      -> \"Got it - the slugs are back.\"\n    my boiler is making a banging noise / the banging is back -> \"Thanks for telling me - I'm here if you want to dig into any of that.\"\n    i think ive got a cold         / its come back           -> the nod, or the cough card by luck\n\nThe person is not opening a subject or changing one: they are reporting that the thing the chat\nwas working on has returned, and that says something the bot should pick up - the advice it gave\ndid not hold, or the thing never really went. The reply has to reach the thing and say so. The nod\n(\"Got it. Is there anything you'd like a hand with?\") is the reply nobody wants, and the echo\n(\"Got it - the slugs are back.\") tells them what they just said.\n\nWhat is left alone:\n\n* a line that asks instead of reporting (\"is it back?\", \"will it come back?\") - that is a question\n  about the thing, and the cards answer it;\n* a person or an animal said to be back (\"my mum is back\", \"the cat is back\") - good news or a\n  separate subject, never a returning trouble;\n* a thing the chat never had (nothing to be back from);\n* \"im back\" and \"we're back\", which are the person, not the thing.\n");
    s_AFTER_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:\\s*(?:again|already|now|too|so soon|as well|for the (?:\\w+ )?time|and (?:its|it is|its worse|worse|its back|the \\w+ is|its the same|same as before)|its worse|it is worse|worse than (?:before|last time)|same as before))*\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRONOUN_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|so|well|oh|ok|okay|right)?\\s*(?:its|it's|it is|theyre|they're|they are|thats|that's|that is|these are|those are)\\s+(?:come\\s+|came\\s+)?back\\b(?P<tail>.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_VERB_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?P<cop>is|are|was|were|s|'s|has|have|had)\\s+(?:come\\s+)?back|(?P<main>came|come|comes|returned|returns)\\s+back|(?P<alone>returned|returns))\\b(?P<tail>.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DETERMINERS(py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that")]));
    s_BARE_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|so|well|oh|ok|okay)?\\s*back (?:again|already|now)\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_OR_PET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::facts::g_RELATIONS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str("|dog|puppy|cat|kitten|rabbit|hamster|parrot|goldfish|horse|tortoise|guinea)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED(py_tuple(vec![py_str("{The thing}, back again - did {they} pass at all in between, or {has} {they} just been quiet?"), py_str("{The thing} came back. Same as before, or worse this time?"), py_str("{The thing} again. What did you do for {them} the first time, and how long did that hold?"), py_str("{The thing} being back usually means whatever set it off is still there - has anything changed since?")]));
    s_UNNAMED(py_tuple(vec![py_str("Back again - did it pass at all in between, or has it been there under the surface?"), py_str("So it eased off and came back. Is it the same as before, or worse this time?"), py_str("Back so soon. What did you do for it the first time, and how long did that hold?"), py_str("Something coming back usually means whatever set it off is still there - has anything changed since?")]));
    s_AWAY_FROM_HOME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was|are|were|been|being|stayed|sent|took|taken|gone|went|dropped|left)\\s+(?:at|in|to|with|into|off to|round to)\\s+(?:the|a|an|my|our|his|her|their|its)\\b|\\b(?:came out|came round|coming out|booked in|in for a service|getting (?:fixed|repaired|serviced|cleaned|mended|done))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SYMPTOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:noise|noisy|banging|rumbling|knocking|squeaking|whistling|rattling|humming|buzzing|grinding|clicking|dripping|hissing|scraping|leak|leaking|damp|mould|mold|smell|smells|reek|sore|ache|aches|aching|pain|painful|hurts?|hurting|bleeding|rash|cough|fever|swollen|broken|cracked|fault|faulty|stuck|jammed|stopped working|not working|isnt working|wont work|wont start|slugs?|mice|rats|ants|flies|aphids|weeds?|worse|chest|throat|head|stomach|back|knee|shoulder|ankle|ear|tooth|toothache)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    wanted = if py_truthy(&thing.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::keywords(args))(&__args) } } else { py_list(vec![]) };
    record = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::current(args))(&__args) };
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = record.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &now.clone()))) {
    continue;
    }
    have = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::keywords(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&have.clone()) { let mut other: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::alike(args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
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
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(thing.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_earlier_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&earlier.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SYMPTOM(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AWAY_FROM_HOME(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plural.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) };
    if py_truthy(&thing.clone()) {
    let _ = py_setitem(&words.clone(), &py_str("the thing"), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)));
    } else {
    let _ = py_setitem(&words.clone(), &py_str("the thing"), py_str("It"));
    }
    out = line.clone();
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "items", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("{"), &key.clone()), &py_str("}"))); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "replace", args))(&__args) };
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("{"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&key.clone(), &Py::Int(0)), "upper", args))(&__args) }), &slice_or_raise(&key.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("}"))); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&value.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&value.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "replace", args))(&__args) };
    }
    if py_truthy(&thing.clone()) {
    return out.clone();
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&out.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::keywords(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(false);
    }
    here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topics")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }, &here.clone()))) {
    continue;
    }
    have = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&have.clone()) { let mut other: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::alike(args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
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
    return Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
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
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("so"), py_str("well"), py_str("oh"), py_str("ok"), py_str("okay"), py_str("right")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    found = py_list(vec![]);
    numbers = py_dict(vec![]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(Py::Int(0)); __args.push(op_neg(&Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    count = __item.clone();
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::None, &count.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &count.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_VERB_BACK(), "match", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tail")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER_BACK(), "match", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_PET(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_earlier_topic(args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cop")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "group", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![Py::Int(0), op_neg(&count.clone()), thing.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    } else {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![Py::Int(1), op_neg(&count.clone()), thing.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cop")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "group", args))(&__args) }) {
    number = Py::Bool(py_contains(&py_tuple(vec![py_str("are"), py_str("were"), py_str("have")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cop")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "group", args))(&__args) }, "lower", args))(&__args) }));
    } else {
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1))), "endswith", args))(&__args) };
    }
    let _ = py_setitem(&numbers.clone(), &thing.clone(), number.clone());
    }
    if py_truthy(&found.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    thing = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(2));
    return py_tuple(vec![thing.clone(), geti(&numbers.clone(), &thing.clone())]);
    }
    return Py::None;
    }
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRONOUN_BACK(), "match", args))(&__args) };
    if py_truthy(&spoken.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tail")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spoken.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER_BACK(), "match", args))(&__args) }))) {
    return Py::None;
    }
    record = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::current(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&record.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_this_line(args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_PET(), "search", args))(&__args) }) {
    return Py::None;
    }
    plural = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("theyre"), py_str("they're"), py_str("they are"), py_str("these"), py_str("those")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "startswith", args))(&__args) };
    return py_tuple(vec![Py::None, plural.clone()]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_BACK(), "match", args))(&__args) }) {
    record = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::current(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&record.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_this_line(args))(&__args) } } }) {
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::history::an_earlier_turn(args))(&__args) }))) {
    return Py::None;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    thing = __unpk1[0].clone();
    plural = __unpk1[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(thing.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_went_away(args))(&__args) }) {
    return Py::None;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&thing.clone()) { g_NAMED() } else { g_UNNAMED() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("random", "choice", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(thing.clone()); __args.push(plural.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _filled(args))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFTER_BACK" => s_AFTER_BACK(v),
            "AWAY_FROM_HOME" => s_AWAY_FROM_HOME(v),
            "A_PERSON_OR_PET" => s_A_PERSON_OR_PET(v),
            "A_SYMPTOM" => s_A_SYMPTOM(v),
            "BARE_BACK" => s_BARE_BACK(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "NAMED" => s_NAMED(v),
            "PRONOUN_BACK" => s_PRONOUN_BACK(v),
            "UNNAMED" => s_UNNAMED(v),
            "VERB_BACK" => s_VERB_BACK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.back_again", module_obj);
    pyrt::xrefs::reg_fn("memory.back_again", "_the_earlier_line", py_func("_the_earlier_line", |args: &[Py]| _the_earlier_line(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_the_thing_went_away", py_func("_the_thing_went_away", |args: &[Py]| _the_thing_went_away(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_the_words", py_func("_the_words", |args: &[Py]| _the_words(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_filled", py_func("_filled", |args: &[Py]| _filled(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_an_earlier_topic", py_func("_an_earlier_topic", |args: &[Py]| _an_earlier_topic(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_from_this_line", py_func("_from_this_line", |args: &[Py]| _from_this_line(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("memory.back_again", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.back_again", "AFTER_BACK", g_AFTER_BACK, s_AFTER_BACK);
    pyrt::xrefs::reg_access("memory.back_again", "AWAY_FROM_HOME", g_AWAY_FROM_HOME, s_AWAY_FROM_HOME);
    pyrt::xrefs::reg_access("memory.back_again", "A_PERSON_OR_PET", g_A_PERSON_OR_PET, s_A_PERSON_OR_PET);
    pyrt::xrefs::reg_access("memory.back_again", "A_SYMPTOM", g_A_SYMPTOM, s_A_SYMPTOM);
    pyrt::xrefs::reg_access("memory.back_again", "BARE_BACK", g_BARE_BACK, s_BARE_BACK);
    pyrt::xrefs::reg_access("memory.back_again", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("memory.back_again", "NAMED", g_NAMED, s_NAMED);
    pyrt::xrefs::reg_access("memory.back_again", "PRONOUN_BACK", g_PRONOUN_BACK, s_PRONOUN_BACK);
    pyrt::xrefs::reg_access("memory.back_again", "UNNAMED", g_UNNAMED, s_UNNAMED);
    pyrt::xrefs::reg_access("memory.back_again", "VERB_BACK", g_VERB_BACK, s_VERB_BACK);
}

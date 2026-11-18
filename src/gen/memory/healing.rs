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


pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_TIMELINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_TIME_THEN_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BARE_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ONLY_A_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _REPEAT_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TIMELINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_CARE() -> Py { ensure_init(); CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE(v: Py) { ensure_init(); if let Ok(mut w) = CARE.write() { *w = v; } }
pub fn g_HEDGE() -> Py { ensure_init(); HEDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE.write() { *w = v; } }
pub fn g_NAMES_ITS_OWN() -> Py { ensure_init(); NAMES_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_ITS_OWN.write() { *w = v; } }
pub fn g_NO_TIMELINE() -> Py { ensure_init(); NO_TIMELINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_TIMELINE(v: Py) { ensure_init(); if let Ok(mut w) = NO_TIMELINE.write() { *w = v; } }
pub fn g_NO_TIME_THEN_STEP() -> Py { ensure_init(); NO_TIME_THEN_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_TIME_THEN_STEP(v: Py) { ensure_init(); if let Ok(mut w) = NO_TIME_THEN_STEP.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_WORD() -> Py { ensure_init(); WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WORD.write() { *w = v; } }
pub fn g__ASK_WORDS() -> Py { ensure_init(); _ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_WORDS.write() { *w = v; } }
pub fn g__BARE_SPAN() -> Py { ensure_init(); _BARE_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BARE_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = _BARE_SPAN.write() { *w = v; } }
pub fn g__ONLY_A_SPAN() -> Py { ensure_init(); _ONLY_A_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ONLY_A_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = _ONLY_A_SPAN.write() { *w = v; } }
pub fn g__REPEAT_LEAD() -> Py { ensure_init(); _REPEAT_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__REPEAT_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _REPEAT_LEAD.write() { *w = v; } }
pub fn g__SPAN() -> Py { ensure_init(); _SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SPAN(v: Py) { ensure_init(); if let Ok(mut w) = _SPAN.write() { *w = v; } }
pub fn g__TIMELINE() -> Py { ensure_init(); _TIMELINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TIMELINE(v: Py) { ensure_init(); if let Ok(mut w) = _TIMELINE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A healing ask answered from the card the person was just given.\n\n\"my back hurts\" gets a paragraph about what helps, and the next line is\n\"how long will it take to heal\". The paragraph is what the bot knows about that back, and the\nspan in its first sentence - \"settles in two to six weeks\" - is the answer to the ask. On its own\nthe ask names nothing, so the readers that follow it look for an entry keyed on the words of the\nline, and the entries that answer \"how long to heal\" are keyed on bones and on sprains:\n\n    my knee hurts / how long does it take to get better -> \"Two days.\"\n    my shoulder hurts / how long does it take to heal   -> \"About six weeks for most bones ...\"\n    i cut my finger / how long will it take to heal     -> \"About six weeks for most bones ...\"\n\n\"Two days\" is the icing in the knee card (\"for the first two days\") read back as the healing time,\nand the bones card answers a cut finger because \"heal\" is its word. So a recovery ask is answered\nfrom the answer already on the screen: the span that answer states, or, where it states none, the\nplain fact that what the bot has is the care rather than the timeline. A figure picked up from\nanother card is wrong, and being wrong about how long a body takes is worse than saying so.\n\nOnly two things make a line this kind of ask: it asks how long, and it names recovering, healing,\ngetting better, clearing up or going away. Only a care card in front of it makes the turn this\nmodule's: \"im learning to drive\" / \"how long will it take to get better\" is answered by the\ndriving entry, and a card that states no care is not a body being looked after.\n");
    s_ASKS(stdlib_call("re", "compile", &[py_str("\\bhow long\\b[^.?!]{0,40}?\\b(?:heal|heals|healing|mend|mends|mending|recover|recovers|recovering|get better|gets better|getting better|get over|gets over|getting over|clear up|clears up|clearing up|go away|goes away|going away|die down|dies down|settle down|settles down|stop hurting|ease off|eases off|get well|be better|feel better|feels better|feel normal|back to normal|be like this|stay like this|stays like this|be gone|keep me off my feet|last|lasts|lasted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_ITS_OWN(stdlib_call("re", "compile", &[py_str("\\bhow long (?:does|do|will|would|should|is|are|can|could)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE(stdlib_call("re", "compile", &[py_str("\\b(?:ice it|ice for|rest it|rest the|raise it|keep it up|wrap it|press a clean|cool running water|paracetamol|ibuprofen|antihistamine|hydrocortisone|see a gp|ring the surgery|get it looked at|get it checked|have it looked at|ease off|keep it moving|keep moving|plenty of (?:fluids|water)|drink plenty|prop yourself up|a spoonful of honey|warm drinks|early night|stay off it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SPAN(py_str("(?:\\d+|an?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a couple of|a few|several)(?:\\s*(?:to|or|-)\\s*(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few))?\\s*(?:hours?|days?|weeks?|fortnights?|months?|years?)"));
    s__TIMELINE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?P<verb>settles?|settled|eases?|eased|heals?|healed|mends?|mended|recovers?|recovered|lasts?|lasted|clears? up|cleared up|fades?|faded|dies down|die down|goes away|go away|gets? better|get better|getting better|is properly better|are properly better|properly better|is better|are better|feels? normal|back to normal|takes|took|hangs? on|hang on|lingers?|settling|easing|healing|mending)\\b(?:[^.?!;:]{0,30}?)\\b(?:in|within|after|for|about|around)?\\s*(?P<span>"), &g__SPAN()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_SPAN(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s*(?:about|around|roughly|usually)?\\s*(?P<span>"), &g__SPAN()), &py_str(")\\s*[.!]?\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ONLY_A_SPAN(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s*(?:about|around|roughly|usually)?\\s*"), &g__SPAN()), &py_str("\\s*[.!]?\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_TIMELINE(py_tuple(vec![py_str("I know what helps with it, but not how long it takes to mend - I'd rather not guess at a number. If it's no easier after a few days, that's the point to get it looked at."), py_str("How long it takes is the part I would be guessing at. What I have is what helps in the meantime, and if there's no change in a few days, that's when to get it looked at."), py_str("I'd rather say I don't have a time than make one up. Keep doing what's in the card above and give it a few days - if it's no better by then, get it looked at.")]));
    s_NO_TIME_THEN_STEP(py_tuple(vec![py_str("{step} How long it takes is the part I would be guessing at."), py_str("{step} I can't say how long it will take - the care is the part I know."), py_str("{step} Past that I can't put a time on it, sorry.")]));
    s_HEDGE(stdlib_call("re", "compile", &[py_str("\\b(?:i'm not sure about that one|i don't know|beyond what i know|i'd rather not guess|a quick search|search online|can't put a time on|would be guessing at|up to date on that|beyond me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z']+")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("how"), py_str("long"), py_str("will"), py_str("would"), py_str("should"), py_str("could"), py_str("does"), py_str("take"), py_str("takes"), py_str("took"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("heal"), py_str("heals"), py_str("healing"), py_str("mend"), py_str("mends"), py_str("mending"), py_str("recover"), py_str("recovers"), py_str("recovering"), py_str("better"), py_str("clear"), py_str("clears"), py_str("clearing"), py_str("away"), py_str("away"), py_str("goes"), py_str("going"), py_str("gone"), py_str("settle"), py_str("settles"), py_str("settling"), py_str("down"), py_str("ease"), py_str("eases"), py_str("easing"), py_str("time"), py_str("till"), py_str("until"), py_str("before"), py_str("after"), py_str("much"), py_str("many"), py_str("usually"), py_str("normal"), py_str("back"), py_str("stop"), py_str("feel"), py_str("feels"), py_str("well"), py_str("this"), py_str("that"), py_str("there"), py_str("then"), py_str("than"), py_str("them"), py_str("they"), py_str("their")])]).unwrap_or_else(|e| exc::raise(e)));
    s__REPEAT_LEAD(stdlib_call("re", "compile", &[py_str("^(?:same as before|as i said|same again)\\s*[:,]?\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn asks(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks how long something takes to mend.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn the_timeline(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The sentence of an answer that states how long a thing takes to mend, or \"\".\n\n    The sentence is cut at the end of the length itself, because what follows it in a card is the\n    next thing to do rather than the answer: \"Most back pain is muscular and settles in two to six\n    weeks, and staying active is what helps most: keep walking ...\" answers the ask up to \"weeks\".\n    ");
    text = call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[])))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[text.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    match_ = mcall(&g__TIMELINE(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    end = mcall(&match_.clone(), "end", &[py_str("span")]);
    clause = mcall(&slice_or_raise(&sentence.clone(), &Py::None, &end.clone(), &Py::None), "rstrip", &[py_str(" ,;:-")]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    return add(&add(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    return py_str("");
    Py::None
}
pub fn _the_same_length(args: &[Py]) -> Py {
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
    let mut timeline: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "timeline") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: timeline"))) };
    let mut said: Py = Py::None;
    let _ = py_str("Whether a reply that is only a length says the length the timeline says.");
    said = mcall(&g__ONLY_A_SPAN(), "match", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&timeline.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&mcall(&timeline.clone(), "lower", &[]), &mcall(&mcall(&mcall(&said.clone(), "group", &[Py::Int(0)]), "strip", &[py_str(" .!")]), "lower", &[])));
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
    let mut ending: Py = Py::None;
    let _ = py_str("A word with its ending taken off, so \"sprained\" and \"sprain\" count as the same thing.\n\n    Only the endings that change nothing about the subject: a bone is not \"boned\", and \"knees\"\n    and \"knee\" are the same knee.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(4)) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn _carries(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line that say what it is about, without the words of the ask.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__ASK_WORDS(), &word.clone())) } }) {  __out.push(_stem(&[word.clone()])); } } py_set(__out) })() };
    Py::None
}
pub fn _said_before(args: &[Py]) -> Py {
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
    let mut keep: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { Py::Int(2) };
    let mut lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The user's own lines before this turn, newest first.");
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[mcall(&turn.clone(), "get", &[py_str("user")])]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&ge(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &keep.clone())) {
    break;
    }
    }
    }
    return lines.clone();
    Py::None
}
pub fn derived(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the reply on the screen was read off the answer above it rather than out of an entry.\n\n    The card behind a healing line is the answer a later doubt is about - \"my knee hurts\" / \"how\n    long does it take to get better\" / \"and what should i do\" asks its \"what should i do\" of the\n    knee card, and the healing line has no steps in it to give. The mark is the turn the line was\n    spoken on, so it cannot outlive that turn: `len(turns)` grows and the equality fails.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    at = mcall(&memory.clone(), "get", &[py_str("healing_at")]);
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&at.clone(), &sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))));
    Py::None
}
pub fn _no_timeline(args: &[Py]) -> Py {
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
    let mut step: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { py_str("") };
    let mut turns: Py = Py::None;
    let _ = py_str("The honest line, a different one each time the same ask comes back.");
    turns = call_builtin("len", &[call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    step = mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&step.clone()) {
    step = add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_NO_TIME_THEN_STEP(), &modulo(&turns.clone(), &call_builtin("len", &[g_NO_TIME_THEN_STEP()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    }
    return geti(&g_NO_TIMELINE(), &modulo(&turns.clone(), &call_builtin("len", &[g_NO_TIMELINE()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_hedge(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the reply already says the bot has no figure for the ask.");
    return call_builtin("bool", &[mcall(&g_HEDGE(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_card(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The answer on the screen: the bot's last line, or the card behind a healing line.\n\n    A healing line is read off the card above it, so a second ask in a row is about that card and\n    not about the line: reading the line gave a step that was the line's own disclaimer, and the\n    ask was answered with \"How long it takes is the part I would be guessing at. Past that I can't\n    put a time on it, sorry.\"\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {  __out.push(call_builtin("str", &[mcall(&turn.clone(), "get", &[py_str("bot")])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&{ let __b = derived(&[memory.clone()]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return geti(&said.clone(), &neg(&Py::Int(2)));
    }
    return if py_truthy(&said.clone()) { geti(&said.clone(), &neg(&Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn vetted(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut step_of: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step_of") { v.clone() }
    else { Py::None };
    let mut card: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut timeline: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut vetted_reply: Py = Py::None;
    let _ = py_str("The reply to a healing ask, or the reply it had.\n\n    A healing ask is answered from the card the person was just given: the timeline it states,\n    or the plain statement that it states none. Two replies are left alone. One is a paragraph of\n    the same body - the card that answers \"how long\" for a twisted ankle is the sprain one, and it\n    names the ankle - and the other is the length the card states, said short (\"Two to three\n    weeks.\" after a cough, whose card says \"settle in two to three weeks\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&asks(&[text.clone()])))) {
    return reply.clone();
    }
    card = _the_card(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&card.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CARE(), "search", &[card.clone()]))) } }) {
    return reply.clone();
    }
    timeline = the_timeline(&[card.clone()]);
    said = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&timeline.clone())) } }) {
    return reply.clone();
    }
    step = py_str("");
    if py_truthy(&call_builtin("callable", &[step_of.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try1 = exc::catch(|| {
    step = mcall(&call_builtin("str", &[{ let __b = py_call(&step_of, &[card.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    step = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    vetted_reply = _the_healing_line(&[said.clone(), timeline.clone(), card.clone(), memory.clone(), step.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&vetted_reply.clone(), &said.clone())); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    let _ = py_setitem(&memory.clone(), &py_str("healing_at"), if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) });
    }
    return vetted_reply.clone();
    Py::None
}
pub fn _the_same_as_the_card(args: &[Py]) -> Py {
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
    let mut card: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: card"))) };
    let mut body: Py = Py::None;
    let _ = py_str("Whether the reply is the card the person already has, said again.\n\n    A length ask about a card that states no timeline has nothing to hand back: the shoulder\n    paragraph's times are how often to do the exercise (\"a few times a day\"), so the paragraph\n    read back for \"how long will it last\" is the card again, and the frame in front of it\n    (\"Same as before:\") says as much. What is owed is the honest line.\n    ");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    card = mcall(&mcall(&call_builtin("str", &[{ let __b = card.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[card.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)) } }) {
    return Py::Bool(false);
    }
    body = mcall(&g__REPEAT_LEAD(), "sub", &[py_str(""), said.clone()]);
    return { let __b = Py::Bool(py_contains(&card.clone(), &body.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&body.clone(), &card.clone())) } };
    Py::None
}
pub fn _the_healing_line(args: &[Py]) -> Py {
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
    let mut timeline: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "timeline") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: timeline"))) };
    let mut card: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: card"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut step: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { py_str("") };
    let mut before: Py = Py::None;
    let mut subject: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return { let __b = timeline.clone(); if py_truthy(&__b) { __b } else { _no_timeline(&[memory.clone(), step.clone()]) } };
    }
    if py_truthy(&_the_same_as_the_card(&[said.clone(), card.clone()])) {
    return { let __b = timeline.clone(); if py_truthy(&__b) { __b } else { _no_timeline(&[memory.clone(), step.clone()]) } };
    }
    if py_truthy(&_a_hedge(&[said.clone()])) {
    return { let __b = timeline.clone(); if py_truthy(&__b) { __b } else { said.clone() } };
    }
    if py_truthy(&mcall(&g__ONLY_A_SPAN(), "match", &[said.clone()])) {
    return if py_truthy(&_the_same_length(&[said.clone(), timeline.clone()])) { said.clone() } else { { let __b = timeline.clone(); if py_truthy(&__b) { __b } else { _no_timeline(&[memory.clone(), step.clone()]) } } };
    }
    if py_truthy(&gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    before = _said_before(&[memory.clone()]);
    subject = if py_truthy(&before.clone()) { _carries(&[mcall(&py_str(" "), "join", &[before.clone()])]) } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&bitand(&subject.clone(), &_carries(&[said.clone()]))) {
    return said.clone();
    }
    if py_truthy(&{ let __b = timeline.clone(); if !py_truthy(&__b) { __b } else { bitand(&_carries(&[timeline.clone()]), &_carries(&[said.clone()])) } }) {
    return said.clone();
    }
    return { let __b = timeline.clone(); if py_truthy(&__b) { __b } else { _no_timeline(&[memory.clone(), step.clone()]) } };
    }
    return said.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.healing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "CARE" => Some(g_CARE()),
            "HEDGE" => Some(g_HEDGE()),
            "NAMES_ITS_OWN" => Some(g_NAMES_ITS_OWN()),
            "NO_TIMELINE" => Some(g_NO_TIMELINE()),
            "NO_TIME_THEN_STEP" => Some(g_NO_TIME_THEN_STEP()),
            "SENTENCE" => Some(g_SENTENCE()),
            "WORD" => Some(g_WORD()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_BARE_SPAN" => Some(g__BARE_SPAN()),
            "_ONLY_A_SPAN" => Some(g__ONLY_A_SPAN()),
            "_REPEAT_LEAD" => Some(g__REPEAT_LEAD()),
            "_SPAN" => Some(g__SPAN()),
            "_TIMELINE" => Some(g__TIMELINE()),
            "_a_hedge" => Some(py_func("_a_hedge", |args: &[Py]| _a_hedge(args))),
            "_carries" => Some(py_func("_carries", |args: &[Py]| _carries(args))),
            "_no_timeline" => Some(py_func("_no_timeline", |args: &[Py]| _no_timeline(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_card" => Some(py_func("_the_card", |args: &[Py]| _the_card(args))),
            "_the_healing_line" => Some(py_func("_the_healing_line", |args: &[Py]| _the_healing_line(args))),
            "_the_same_as_the_card" => Some(py_func("_the_same_as_the_card", |args: &[Py]| _the_same_as_the_card(args))),
            "_the_same_length" => Some(py_func("_the_same_length", |args: &[Py]| _the_same_length(args))),
            "asks" => Some(py_func("asks", |args: &[Py]| asks(args))),
            "derived" => Some(py_func("derived", |args: &[Py]| derived(args))),
            "the_timeline" => Some(py_func("the_timeline", |args: &[Py]| the_timeline(args))),
            "vetted" => Some(py_func("vetted", |args: &[Py]| vetted(args))),
            _ => None,
        }),
        set: None,
    }))
}

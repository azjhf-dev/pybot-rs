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


pub static ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COURTESY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEICTIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETERMINER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREPOSITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREPOSITION_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKE_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASK() -> Py { ensure_init(); ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ASK.write() { *w = v; } }
pub fn g_ASKS_A_THING() -> Py { ensure_init(); ASKS_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_A_THING.write() { *w = v; } }
pub fn g_A_CLAUSE() -> Py { ensure_init(); A_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = A_CLAUSE.write() { *w = v; } }
pub fn g_A_COURTESY() -> Py { ensure_init(); A_COURTESY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COURTESY(v: Py) { ensure_init(); if let Ok(mut w) = A_COURTESY.write() { *w = v; } }
pub fn g_DEICTIC() -> Py { ensure_init(); DEICTIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEICTIC(v: Py) { ensure_init(); if let Ok(mut w) = DEICTIC.write() { *w = v; } }
pub fn g_DETERMINER() -> Py { ensure_init(); DETERMINER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETERMINER(v: Py) { ensure_init(); if let Ok(mut w) = DETERMINER.write() { *w = v; } }
pub fn g_HEDGE_OFF() -> Py { ensure_init(); HEDGE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = HEDGE_OFF.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_OPENERS() -> Py { ensure_init(); OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = OPENERS.write() { *w = v; } }
pub fn g_OWNER() -> Py { ensure_init(); OWNER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNER(v: Py) { ensure_init(); if let Ok(mut w) = OWNER.write() { *w = v; } }
pub fn g_OWNER_WORDS() -> Py { ensure_init(); OWNER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = OWNER_WORDS.write() { *w = v; } }
pub fn g_PERSON_FILLER() -> Py { ensure_init(); PERSON_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_FILLER.write() { *w = v; } }
pub fn g_PERSON_LEAD() -> Py { ensure_init(); PERSON_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_LEAD.write() { *w = v; } }
pub fn g_PREPOSITION() -> Py { ensure_init(); PREPOSITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREPOSITION(v: Py) { ensure_init(); if let Ok(mut w) = PREPOSITION.write() { *w = v; } }
pub fn g_PREPOSITION_TAIL() -> Py { ensure_init(); PREPOSITION_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREPOSITION_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = PREPOSITION_TAIL.write() { *w = v; } }
pub fn g_REPAIR() -> Py { ensure_init(); REPAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR.write() { *w = v; } }
pub fn g_STOP() -> Py { ensure_init(); STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); if let Ok(mut w) = STOP.write() { *w = v; } }
pub fn g_TAIL_OFF() -> Py { ensure_init(); TAIL_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_OFF(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_OFF.write() { *w = v; } }
pub fn g_TAKE_BACK() -> Py { ensure_init(); TAKE_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKE_BACK(v: Py) { ensure_init(); if let Ok(mut w) = TAKE_BACK.write() { *w = v; } }
pub fn g_THING() -> Py { ensure_init(); THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING(v: Py) { ensure_init(); if let Ok(mut w) = THING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A follow-up that only names a new thing: the same question, with that thing in it.\n\n\"can you give me a dinner idea\" ... \"what about a dessert\" is one question asked about two\nthings. The second line names no ask at all - just a thing and the words that say it is a\nfollow-up - so read on its own no entry fits it, and the bot shrugs (\"I don't know enough to\nanswer that honestly\") at a question it answers perfectly well when it is asked in full (\"any\ndessert ideas\" gets a crumble). Said again as the earlier question with the earlier thing\nswapped for the new one, it reaches the same entry.\n\nWhich word is swapped is read off the conversation rather than guessed: the earlier question's\nown content word that the answer just given took up is the thing being talked about (\"dinner\"\nin \"can you give me a dinner idea\", taken up by \"Ten-minute dinners: ...\"), and that is the\nword the new line replaces.\n\nOnly a line that says nothing but \"and X\" / \"what about X\" is rewritten; a line with an ask of\nits own, a pronoun (\"what about it\", \"and that one\") or a figure is left alone. The rewritten\nline still has to reach an entry whose answer is about the new thing, which is the caller's\ncheck: this module offers a reading, it does not answer.\n");
    s_LEAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|right|so|yes|yeah|and)\\b[\\s,]*)*(?:(?:and|but|or)\\s+)?(?:(?:what|how)\\s+about|what\\s+of|and|also|plus)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nope|sorry|apologies|actually|wait|hold on|oops)[\\s,.!-]+)*(?:i\\s+(?:mean|meant|said)(?:\\s+to\\s+say)?|(?:you\\s+)?mean|i\\s+did\\s+mean)[\\s,.!-]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKE_BACK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nope|wait|hold on|or rather|scratch that|oops)[\\s,.!-]+)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COURTESY(call_builtin("frozenset", &[mcall(&py_str("\n    thanks thank thankyou ta cheers mate love hun pal duck fella sir madam\n    worries worry anyway anyhow though please fine alright ok okay all much\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isnt|arent|wasnt|werent|does|did|do|doesnt|didnt|dont|has|have|had|hasnt|havent|hadnt|will|would|wont|wouldnt|can|cant|could|couldnt|should|shouldnt|must|im|ive|id|ill|weve|youre|its|thats|theres)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING(stdlib_call("re", "compile", &[py_str("^(?:(?:a|an|the|some|any|another)\\s+)?([a-z][a-z' -]{0,24})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_THING(stdlib_call("re", "compile", &[py_str("^(?:what|which|how|why|when|where|who|whose|whom)\\b|^(?:can|could|should|would|will|shall|do|does|did|is|are|was|were|have|has|had|am|may|might|must|need)\\s+(?:i|we|you|they|it|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_OFF(stdlib_call("re", "compile", &[py_str("[\\s,]*(?:\\b(?:too|as well|also|instead|then|now|though|please|thanks)\\b[\\s,.!?]*)+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGE_OFF(stdlib_call("re", "compile", &[py_str("[\\s,]*(?:\\b(?:probably|maybe|perhaps|possibly|definitely|likely|obviously|actually|really|sure|suppose|supposedly|hopefully|honestly|frankly)\\b|\\b(?:i|we)\\s+(?:think|guess|suppose|reckon|imagine|hope)\\b|\\bof course\\b|\\bfor now\\b|\\bright\\b|\\bok(?:ay)?\\b)[\\s,.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEICTIC(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("its"), py_str("it's"), py_str("that"), py_str("this"), py_str("them"), py_str("they"), py_str("those"), py_str("these"), py_str("him"), py_str("her"), py_str("one"), py_str("ones"), py_str("the same"), py_str("same"), py_str("the other"), py_str("rest"), py_str("more"), py_str("less"), py_str("instead"), py_str("then"), py_str("now"), py_str("there"), py_str("here"), py_str("you"), py_str("me"), py_str("us"), py_str("my"), py_str("your"), py_str("mine"), py_str("yours")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("and"), py_str("or"), py_str("but"), py_str("if"), py_str("of"), py_str("for"), py_str("to"), py_str("in"), py_str("on"), py_str("at"), py_str("with"), py_str("about"), py_str("as"), py_str("by"), py_str("from"), py_str("into"), py_str("over"), py_str("after"), py_str("before"), py_str("than"), py_str("then"), py_str("so"), py_str("i"), py_str("im"), py_str("i'm"), py_str("me"), py_str("my"), py_str("we"), py_str("you"), py_str("your"), py_str("it"), py_str("its"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("am"), py_str("do"), py_str("does"), py_str("did"), py_str("done"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("will"), py_str("shall"), py_str("may"), py_str("might"), py_str("must"), py_str("have"), py_str("has"), py_str("had"), py_str("get"), py_str("got"), py_str("give"), py_str("make"), py_str("much"), py_str("many"), py_str("long"), py_str("how"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("who"), py_str("why"), py_str("any"), py_str("some"), py_str("good"), py_str("best"), py_str("better"), py_str("more"), py_str("most"), py_str("not"), py_str("no"), py_str("yes"), py_str("please"), py_str("there"), py_str("here"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("them"), py_str("they"), py_str("he"), py_str("she"), py_str("his"), py_str("her"), py_str("him"), py_str("want"), py_str("need"), py_str("like"), py_str("one"), py_str("also"), py_str("just"), py_str("really"), py_str("very"), py_str("know"), py_str("tell"), py_str("think"), py_str("say"), py_str("said"), py_str("help"), py_str("idea"), py_str("ideas"), py_str("thing"), py_str("things"), py_str("anything"), py_str("something"), py_str("way"), py_str("ways"), py_str("time"), py_str("times"), py_str("kind"), py_str("kinds"), py_str("sort"), py_str("sorts")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENERS(py_tuple(vec![py_str("how"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("who"), py_str("why"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("do"), py_str("does"), py_str("did"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("will"), py_str("shall")]));
    s_ASK(call_builtin("frozenset", &[py_tuple(vec![py_str("whats"), py_str("hows"), py_str("wheres"), py_str("whos"), py_str("whys"), py_str("whens"), py_str("whatre"), py_str("hows")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_LEAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|right|so|yes|yeah|and)\\b[\\s,]*)*(?:(?:and|but|or)\\s+)?(?:(?:what|how)\\s+about|what\\s+of|and|also|plus)\\b[\\s,]*(?:something\\s+for|one\\s+for|a\\s+gift\\s+for|the\\s+same\\s+for|presents?\\s+for)?[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("for"), py_str("to"), py_str("about"), py_str("with"), py_str("then"), py_str("too"), py_str("also"), py_str("instead"), py_str("please"), py_str("thanks"), py_str("the"), py_str("a"), py_str("an"), py_str("get"), py_str("buy"), py_str("got"), py_str("them"), py_str("him"), py_str("her"), py_str("and"), py_str("as"), py_str("well"), py_str("his"), py_str("hers"), py_str("their"), py_str("birthday"), py_str("christmas"), py_str("present"), py_str("gift"), py_str("presents"), py_str("gifts")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNER(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("your"), py_str("their"), py_str("his"), py_str("her"), py_str("the"), py_str("a"), py_str("an"), py_str("some"), py_str("this"), py_str("that")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNER_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("your"), py_str("their")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DETERMINER(call_builtin("frozenset", &[mcall(&py_str("the a an my your his her its their our this that these those some any\nanother"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PREPOSITION(call_builtin("frozenset", &[mcall(&py_str("in on at for with from by to near inside outside under over behind\nabove below beneath underneath between beside besides alongside around across along through\nagainst towards toward onto upon within without off out up down past opposite round via\nabout"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PREPOSITION_TAIL(call_builtin("frozenset", &[mcall(&py_str("next close right just all the top bottom front back inside\noutside end side"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _after_a_take_back(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("What a line says after its \"no wait\", or None when the line is not a take-back and a thing.\n\n    The rest has to be short and name no happening of its own, because the reading it is offered\n    is the last question with this thing swapped in: \"no wait, a laptop\" after \"i need a new\n    phone\" is that sentence about a laptop, while \"no, the wifi is down\" is news.\n    ");
    lead = mcall(&g_TAKE_BACK(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    return Py::None;
    }
    rest = mcall(&mcall(&g_TAIL_OFF(), "sub", &[py_str(""), mcall(&slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ?!.,")])]), "strip", &[py_str(" ?!.,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { mcall(&g_A_CLAUSE(), "search", &[rest.clone()]) } } } }) {
    return Py::None;
    }
    return rest.clone();
    Py::None
}
pub fn _said(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
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
    let mut end: Py = Py::None;
    word = mcall(&word.clone(), "strip", &[py_str("?.!,;:")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ies"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    end = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[end.clone()]) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[end.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn thing(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hedged: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut name: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The new thing a follow-up names, or None: \"what about a dessert\" -> \"dessert\".");
    said = mcall(&_said(&[text.clone()]), "strip", &[py_str(" ?!.")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    lead = mcall(&g_LEAD(), "match", &[said.clone()]);
    repaired = Py::Bool(false);
    if py_truthy(&lead.clone()) {
    rest = slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None);
    } else {
    lead = mcall(&g_REPAIR(), "match", &[said.clone()]);
    if py_truthy(&lead.clone()) {
    rest = slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None);
    repaired = Py::Bool(true);
    } else {
    rest = _after_a_take_back(&[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&rest.clone(), &Py::None))) {
    return Py::None;
    }
    }
    }
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_CLAUSE(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    rest = mcall(&mcall(&g_TAIL_OFF(), "sub", &[py_str(""), mcall(&rest.clone(), "strip", &[py_str(" ?!.,")])]), "strip", &[py_str(" ?!.,")]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    hedged = mcall(&mcall(&g_HEDGE_OFF(), "sub", &[py_str(""), rest.clone()]), "strip", &[py_str(" ?!.,")]);
    if py_truthy(&Py::Bool(py_eq(&hedged.clone(), &rest.clone()))) {
    break;
    }
    rest = hedged.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::None;
    }
    found = mcall(&g_THING(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    name = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[ ']"), name.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNER_WORDS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    name = mcall(&py_str(" "), "join", &[words.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DEICTIC(), &name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DEICTIC(), &mcall(&py_str(" "), "join", &[words.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DEICTIC(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS_A_THING(), "search", &[name.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STOP(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_A_COURTESY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STOP(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return name.clone();
    Py::None
}
pub fn someone(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut left: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person a follow-up only names, or None: \"what about my dad\" -> \"dad\".\n\n    \"and something for my brother\", \"what about my mum then\" and \"no i mean my sister\" are the same\n    reading. A line that says anything else about them (\"what about my dad's birthday\", \"how old\n    is my dad\") is not this: it asks its own thing, and reading it as the earlier question would\n    throw that away.\n    ");
    let mut person: Py = py_func("person", |args: &[Py]| crate::gen::memory::topics::person(args));
    said = mcall(&_said(&[text.clone()]), "strip", &[py_str(" ?!.")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::None;
    }
    lead = { let __b = mcall(&g_PERSON_LEAD(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REPAIR(), "match", &[said.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    return Py::None;
    }
    rest = mcall(&slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ?!.,")]);
    who = py_call(&person, &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    left = stdlib_call("re", "sub", &[py_str("'s\\b"), py_str(""), stdlib_call("re", "sub", &[add(&add(&py_str("\\b(?:my|our)\\s+"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), py_str(" "), rest.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    spare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[ ']"), left.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PERSON_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STOP(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&spare.clone()) {
    return Py::None;
    }
    return who.clone();
    Py::None
}
pub fn _earlier(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut n: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(4) };
    let mut wide: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wide") { v.clone() }
    else { Py::Bool(false) };
    let mut answer: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut question: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The person's own questions before this line, newest first, with the answer each got.\n\n    *wide* takes statements as well as questions. A repair is the person saying the earlier line\n    got its subject wrong, so it can take back a statement too: \"my tomato plants have yellow\n    leaves\" ... \"i meant my pepper plants\". Nothing else asks for that, because a \"what about X\"\n    hanging off a statement is not a rewrite of it.\n    ");
    let mut recent: Py = py_func("recent", |args: &[Py]| crate::gen::memory::recent(args));
    here = _said(&[text.clone()]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&recent, &[memory.clone(), add(&n.clone(), &Py::Int(2))])) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    question = _said(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = question.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&question.clone(), &here.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&question.clone(), &mcall(&here.clone(), "rstrip", &[py_str(" ?!.")]))) } } } }) {
    continue;
    }
    answer = _said(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&question.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    continue;
    }
    first = mcall(&geti(&mcall(&question.clone(), "split", &[]), &Py::Int(0)), "strip", &[py_str("?,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wide.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&question.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_OPENERS(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASK(), &first.clone())) } } } } } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![question.clone(), answer.clone()])]);
    if py_truthy(&ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &n.clone())) {
    break;
    }
    }
    return found.clone();
    Py::None
}
pub fn _about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut read: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing an earlier line is asked about, or None: the chicken in \"how long does a\n    chicken take to roast\".\n\n    Read off the understanding of the earlier line rather than guessed from its words, because\n    the word an answer takes up is often the verb (\"about 20 minutes per 500g\" takes up \"take\"\n    and never says chicken), and a correction puts the new thing in place of the thing asked\n    about, not in place of the asking. A line with no subject of its own (\"how do i get rid of\n    ants\") has nothing to offer.\n    ");
    let mut understand: Py = py_func("understand", |args: &[Py]| crate::gen::understanding::understand(args));
    let __try1 = exc::catch(|| {
    read = py_call(&understand, &[question.clone(), py_dict(vec![])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    subject = _said(&[if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = read.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("subject")]) } else { py_str("") }]);
    subject = mcall(&subject.clone(), "strip", &[py_str(" ?!.,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&subject.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&subject.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    words = mcall(&subject.clone(), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNER(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STOP(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _reads_as_a_question(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the earlier line asks something of its own.\n\n    A line with a question mark, or one that opens with a question word (\"whats the capital of\n    australia\" has no question mark and is still a question), is a question. Anything else is a\n    statement, and a statement is read differently: see `rewritten`.\n    ");
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &py_str("?")))) {
    return Py::Bool(true);
    }
    first = mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str("?,!")]);
    return { let __b = Py::Bool(py_contains(&g_OPENERS(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK(), &first.clone())) } };
    Py::None
}
pub fn _word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut new_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new_words"))) };
    let mut taken_up: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "taken_up") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: taken_up"))) };
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The earlier question's content word to swap: the one the answer took up (or, on the\n    second pass, its last content word), or None.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&question.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str("?.!,;:")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STOP(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&new_words.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DEICTIC(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = taken_up.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _stem(&[word.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&answer.clone(), &_stem(&[word.clone()]))) } })) } }) {
    continue;
    }
    return word.clone();
    }
    return Py::None;
    Py::None
}
pub fn _replace(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut new: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut rewritten: Py = Py::None;
    let _ = py_str("The earlier question with *word* taken out and the new thing in its place.");
    found = stdlib_call("re", "search", &[add(&add(&py_str("\\b(?:(a|an)\\s+)?"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), question.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&found.clone(), "group", &[Py::Int(1)])) {
    lead = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&new.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an ") } else { py_str("a ") };
    } else {
    lead = py_str("");
    }
    rewritten = add(&add(&add(&slice_or_raise(&question.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &lead.clone()), &new.clone()), &slice_or_raise(&question.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    rewritten = stdlib_call("re", "sub", &[py_str("\\b(\\w+)\\s+\\1\\b"), py_str("\\1"), rewritten.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&rewritten.clone(), &question.clone()))) {
    return Py::None;
    }
    head = mcall(&geti(&mcall(&new.clone(), "split", &[]), &neg(&Py::Int(1))), "lower", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&new.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { gt(&mcall(&mcall(&rewritten.clone(), "lower", &[]), "count", &[head.clone()]), &Py::Int(1)) } }) {
    return Py::None;
    }
    return rewritten.clone();
    Py::None
}
pub fn _spans(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut at: Py = Py::None;
    let mut back: Py = Py::None;
    let mut found: Py = Py::None;
    let mut may_be_it: Py = Py::None;
    let mut out: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The stretches of the earlier question a new thing could take the place of.\n\n    The thing asked about is often more than its head word: \"how do i clean a stainless steel\n    pan\" asked about cast iron is that question with \"stainless steel\" put right, and \"how\n    much does a boiler service cost\" asked about a gas safety check is \"how much does a gas\n    safety check cost\", with the \"service\" gone. So the words around the subject are offered\n    too, longest first, because a longer stretch leaves less of the old thing behind.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&question.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str("?.!,;:")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&subject.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&words.clone(), &subject.clone())) } }) {
    return py_list(vec![subject.clone()]);
    }
    at = mcall(&words.clone(), "index", &[subject.clone()]);
pub fn __nested_may_be_it_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut at: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut back: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut found: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut may_be_it: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut out: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut question: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut span: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut spans: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut subject: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut w: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "word").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = call_builtin("bool", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_STOP(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_DEICTIC(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASK(), &word.clone())) } } } } } } } };
    Py::None
}
    may_be_it = py_func("may_be_it", move |args: &[Py]| { let mut full = vec![at.clone(), back.clone(), found.clone(), may_be_it.clone(), out.clone(), question.clone(), span.clone(), spans.clone(), subject.clone(), w.clone(), words.clone()]; full.extend_from_slice(args); __nested_may_be_it_0(&full) });
    spans = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&py_tuple(vec![Py::Int(2), Py::Int(1)]));
    for __item in __iter2 {
    back = __item.clone();
    if py_truthy(&{ let __b = ge(&sub(&at.clone(), &back.clone()), &Py::Int(0)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &sub(&at.clone(), &back.clone()), &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(py_call(&may_be_it, &[w.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&spans.clone(), "append", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &sub(&at.clone(), &back.clone()), &add(&at.clone(), &Py::Int(1)), &Py::None)])]);
    }
    }
    if py_truthy(&{ let __b = lt(&add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { py_call(&may_be_it, &[geti(&words.clone(), &add(&at.clone(), &Py::Int(1)))]) } }) {
    let _ = mcall(&spans.clone(), "append", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &at.clone(), &add(&at.clone(), &Py::Int(2)), &Py::None)])]);
    }
    let _ = mcall(&spans.clone(), "append", &[subject.clone()]);
    let __iter3 = py_iter_or_raise(&py_tuple(vec![Py::Int(2), Py::Int(1)]));
    for __item in __iter3 {
    back = __item.clone();
    if py_truthy(&{ let __b = ge(&sub(&at.clone(), &back.clone()), &Py::Int(0)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &sub(&at.clone(), &back.clone()), &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(py_call(&may_be_it, &[w.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&spans.clone(), "append", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &sub(&at.clone(), &back.clone()), &at.clone(), &Py::None)])]);
    }
    }
    let __unpk4 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![])]), 2, None);
    found = __unpk4[0].clone();
    out = __unpk4[1].clone();
    let __iter5 = py_iter_or_raise(&spans.clone());
    for __item in __iter5 {
    span = __item.clone();
    if py_truthy(&{ let __b = span.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&found.clone(), &span.clone())) } }) {
    let _ = mcall(&found.clone(), "add", &[span.clone()]);
    let _ = mcall(&out.clone(), "append", &[span.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn is_repair(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line takes back the thing it named: \"i meant a turkey\", \"no wait, duck\".");
    said = mcall(&_said(&[text.clone()]), "strip", &[py_str(" ?!.")]);
    return { let __b = call_builtin("bool", &[mcall(&g_REPAIR(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_after_a_take_back(&[said.clone()]), &Py::None)) } };
    Py::None
}
pub fn candidates(args: &[Py]) -> Py {
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
    let mut most: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "most") { v.clone() }
    else { Py::Int(3) };
    let mut _old: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut found: Py = Py::None;
    let mut had: Py = Py::None;
    let mut line: Py = Py::None;
    let mut new: Py = Py::None;
    let mut new_words: Py = Py::None;
    let mut place: Py = Py::None;
    let mut question: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The lines this one could be read as, best first, each with the word it replaced.\n\n    A follow-up that only names a new thing is the earlier question with that thing in it, and\n    the earlier question is not always about the word its answer took up: \"how long does a\n    chicken take to roast\" is answered with \"20 minutes per 500g plus 20\", which takes up\n    \"take\" and never says chicken, so \"i meant a turkey\" was swapped into \"how long does a\n    chicken turkey to roast\" and reached nothing. The thing the earlier line is asked about is\n    tried first, then the word the older reading picked, then its last content word - and the\n    caller holds every reading to an entry whose answer is about the new thing, so a swap the\n    conversation does not support costs a shrug, not a wrong answer.\n    ");
    new = thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&new.clone()))) {
    return py_list(vec![]);
    }
    repaired = is_repair(&[text.clone()]);
    new_words = call_builtin("set", &[stdlib_call("re", "split", &[py_str("[ ']"), new.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    place = call_builtin("bool", &[_the_preposition_of(&[new.clone()])]).unwrap_or_else(|e| exc::raise(e));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); __kws.extend(vec![("wide".to_string(), repaired.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _earlier(args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    question = __unpk2[0].clone();
    answer = __unpk2[1].clone();
    subject = _about(&[question.clone()]);
    words = py_list(vec![]);
    if py_truthy(&place.clone()) {
    had = _the_place_after_the_same_preposition(&[question.clone(), new.clone()]);
    if py_truthy(&had.clone()) {
    let _ = mcall(&words.clone(), "append", &[had.clone()]);
    }
    }
    let __aug3 = add(&words.clone(), &if py_truthy(&subject.clone()) { _spans(&[question.clone(), subject.clone()]) } else { py_list(vec![]) });
    words = __aug3;
    let __aug4 = add(&words.clone(), &py_list(vec![_word(&[question.clone(), answer.clone(), new_words.clone(), Py::Bool(true)]), _word(&[question.clone(), answer.clone(), new_words.clone(), Py::Bool(false)])]));
    words = __aug4;
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    line = if py_truthy(&place.clone()) { placed(&[question.clone(), word.clone(), new.clone()]) } else { _replace(&[question.clone(), word.clone(), new.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { _a_second_place(&[question.clone(), line.clone(), new.clone()]) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk4489 = py_unpack(&__item.clone(), 2, None); let mut had: Py = __unpk4489[0].clone(); let mut _old: Py = __unpk4489[1].clone(); __out.push(Py::Bool(py_eq(&line.clone(), &had.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![line.clone(), word.clone()])]);
    if py_truthy(&ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &most.clone())) {
    return found.clone();
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn rewritten(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut head: Py = Py::None;
    let mut in_between: Py = Py::None;
    let mut line: Py = Py::None;
    let mut new: Py = Py::None;
    let mut new_words: Py = Py::None;
    let mut question: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut swap: Py = Py::None;
    let mut taken_up: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("The line read as the earlier question with the earlier thing swapped, or None.\n\n    Returns the rewritten question and the word it replaced: the caller needs the replaced word\n    to tell an answer about the new thing from the old answer said again.\n    ");
    new = thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&new.clone()))) {
    return Py::None;
    }
    new_words = call_builtin("set", &[stdlib_call("re", "split", &[py_str("[ ']"), new.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); __kws.extend(vec![("wide".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _earlier(args))(&__args) };
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let __unpk8425 = py_unpack(&__item.clone(), 2, None); let mut line: Py = __unpk8425[0].clone(); let mut answer: Py = __unpk8425[1].clone(); if py_truthy(&_reads_as_a_question(&[line.clone()])) {  __out.push(py_tuple(vec![line.clone(), answer.clone()])); } } py_list(__out) })() };
    told = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let __unpk5449 = py_unpack(&__item.clone(), 2, None); let mut line: Py = __unpk5449[0].clone(); let mut answer: Py = __unpk5449[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&_reads_as_a_question(&[line.clone()])))) {  __out.push(py_tuple(vec![line.clone(), answer.clone()])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Bool(true), Py::Bool(false)]));
    for __item in __iter1 {
    taken_up = __item.clone();
    let __iter2 = py_iter_or_raise(&asked.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    question = __unpk3[0].clone();
    answer = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    continue;
    }
    swap = _word(&[question.clone(), answer.clone(), new_words.clone(), taken_up.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&swap.clone()))) {
    continue;
    }
    candidate = placed(&[question.clone(), swap.clone(), new.clone()]);
    if py_truthy(&{ let __b = candidate.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&candidate.clone(), &question.clone())) } }) {
    return py_tuple(vec![candidate.clone(), swap.clone()]);
    }
    }
    }
    in_between = if py_truthy(&earlier.clone()) { thing(&[geti(&geti(&earlier.clone(), &Py::Int(0)), &Py::Int(0))]) } else { Py::None };
    head = if py_truthy(&gt(&call_builtin("len", &[mcall(&new.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&mcall(&geti(&mcall(&new.clone(), "split", &[]), &neg(&Py::Int(1))), "strip", &[py_str("?.!,;:")]), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = in_between.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&in_between.clone(), "split", &[]), &head.clone())) } }) {
    return Py::None;
    }
    let __iter4 = py_iter_or_raise(&told.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    question = __unpk5[0].clone();
    answer = __unpk5[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    continue;
    }
    subject = _about(&[question.clone()]);
    let __iter6 = py_iter_or_raise(&if py_truthy(&subject.clone()) { _spans(&[question.clone(), subject.clone()]) } else { py_list(vec![]) });
    for __item in __iter6 {
    swap = __item.clone();
    candidate = placed(&[question.clone(), swap.clone(), new.clone()]);
    if py_truthy(&{ let __b = candidate.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&candidate.clone(), &question.clone())) } }) {
    return py_tuple(vec![candidate.clone(), swap.clone()]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _named_somebody(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut n: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(4) };
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The person's own lines before this one that name somebody, newest first, with the answer.\n\n    A statement counts here where the thing swap asks for a question: \"I'm buying a gift for my\n    sister\" ... \"what about my brother\" is one question about two people even though the first\n    line was not phrased as one. Nothing is guessed from it - the rewritten line still has to\n    reach an entry, and the answer still has to be about the new person.\n    ");
    let mut recent: Py = py_func("recent", |args: &[Py]| crate::gen::memory::recent(args));
    here = _said(&[text.clone()]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&recent, &[memory.clone(), add(&n.clone(), &Py::Int(2))])) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = _said(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&line.clone(), &here.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&line.clone(), &mcall(&here.clone(), "rstrip", &[py_str(" ?!.")]))) } } } }) {
    continue;
    }
    if py_truthy(&lt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![line.clone(), _said(&[mcall(&turn.clone(), "get", &[py_str("bot")])])])]);
    if py_truthy(&ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &n.clone())) {
    break;
    }
    }
    return found.clone();
    Py::None
}
pub fn rewritten_person(args: &[Py]) -> Py {
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
    let mut _answer: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut old: Py = Py::None;
    let mut question: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The line read as the earlier question with the earlier person swapped, or None.\n\n    \"What should I get my mum for her birthday\" ... \"what about my dad\" is that question about\n    the dad, and asked in full it reaches the entry that answers it. The earlier question has to\n    name somebody of its own: with no person in it (\"how do i stop my cat scratching the sofa\")\n    there is nothing to put the new one in place of, and the line keeps whatever reading it has.\n\n    Returns the rewritten question and the person it replaced.\n    ");
    let mut person: Py = py_func("person", |args: &[Py]| crate::gen::memory::topics::person(args));
    who = someone(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_named_somebody(&[memory.clone(), text.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    question = __unpk2[0].clone();
    _answer = __unpk2[1].clone();
    old = py_call(&person, &[question.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&old.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&old.clone(), &who.clone())) } }) {
    continue;
    }
    named = stdlib_call("re", "compile", &[add(&add(&py_str("\\b(my|our)\\s+"), &stdlib_call("re", "escape", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    found = mcall(&named.clone(), "search", &[question.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    candidate = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_func("lambda", move |args: &[Py]| { let _answer = _answer.clone(); let candidate = candidate.clone(); let found = found.clone(); let memory = memory.clone(); let named = named.clone(); let old = old.clone(); let person = person.clone(); let question = question.clone(); let text = text.clone(); let who = who.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); add(&add(&mcall(&m.clone(), "group", &[Py::Int(1)]), &py_str(" ")), &who.clone()) })); __args.push(question.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(!py_eq(&candidate.clone(), &question.clone()))) {
    return py_tuple(vec![candidate.clone(), old.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn names_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut old: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut last: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { py_str("") };
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("True when an answer to the rewritten question is about the new person, not the old one.\n\n    A gift answer names neither \"mum\" nor \"dad\" (\"Go consumable and a notch better than he'd\n    buy himself\"), so there is no word to look for. What can be said is what must not happen: the\n    old person's name in the new answer, or the answer just given handed over again as if it were\n    about somebody else.\n    ");
    who = someone(&[text.clone()]);
    said = _said(&[answer.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = old.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said.clone(), &_said(&[last.clone()]))) } };
    Py::None
}
pub fn _the_preposition_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut new: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut head: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The preposition a new thing arrives with (\"from birmingham\" -> \"from\"), or \"\".");
    words = mcall(&call_builtin("str", &[{ let __b = new.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    head = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str("?.!,;:")]), "lower", &[]);
    return if py_truthy(&Py::Bool(py_contains(&g_PREPOSITION(), &head.clone()))) { head.clone() } else { py_str("") };
    Py::None
}
pub fn _the_place_after_the_same_preposition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut new: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut follow: Py = Py::None;
    let mut head: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier question's place, said with the preposition the new thing brings, or None.\n\n    \"whats a good day trip from manchester\" corrected to \"sorry, i meant from birmingham\" is the\n    same question about the other place, so the word to swap is Manchester and the preposition\n    goes with it. Only the phrase right after that same preposition is offered: the place is what\n    the new one takes the place of, and a question with no such phrase has nothing to put it in.\n    ");
    head = _the_preposition_of(&[new.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::None;
    }
    said = mcall(&question.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&word.clone(), "strip", &[py_str("?.!,;:")]), "lower", &[]), &head.clone()))) {
    continue;
    }
    after = add(&at.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&after.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINER(), &mcall(&mcall(&geti(&said.clone(), &after.clone()), "strip", &[py_str("?.!,;:")]), "lower", &[]))) } }) {
    let __aug3 = add(&after.clone(), &Py::Int(1));
    after = __aug3;
    }
    span = py_list(vec![]);
    let __iter4 = py_iter_or_raise(&slice_or_raise(&said.clone(), &after.clone(), &Py::None, &Py::None));
    for __item in __iter4 {
    follow = __item.clone();
    bare = mcall(&follow.clone(), "strip", &[py_str("?.!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&bare.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_STOP(), &mcall(&bare.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREPOSITION(), &mcall(&bare.clone(), "lower", &[]))) } } } }) {
    break;
    }
    let _ = mcall(&span.clone(), "append", &[bare.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    if py_truthy(&span.clone()) {
    return mcall(&py_str(" "), "join", &[span.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_second_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut new: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut count: Py = Py::None;
    let mut head: Py = Py::None;
    let _ = py_str("True when a reading hangs a second place on the earlier question with the same preposition.\n\n    \"from ... from ...\" and \"in the in the freezer\" are sentences nobody writes: a place that\n    brings its own preposition takes the old place's place. The reading is not the question the\n    person meant, so it is dropped rather than offered to the entries.\n    ");
    head = _the_preposition_of(&[new.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::Bool(false);
    }
pub fn __nested_count_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut count: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut head: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut new: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut question: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "text").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut word: Py = Py::None;
    return call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&word.clone(), "strip", &[py_str("?.!,;:")]), "lower", &[]), &head.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    count = py_func("count", move |args: &[Py]| { let mut full = vec![count.clone(), head.clone(), line.clone(), new.clone(), question.clone()]; full.extend_from_slice(args); __nested_count_0(&full) });
    return gt(&py_call(&count, &[line.clone()]), &py_call(&count, &[question.clone()]));
    Py::None
}
pub fn placed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut new: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut at: Py = Py::None;
    let mut head: Py = Py::None;
    let mut i: Py = Py::None;
    let mut new_words: Py = Py::None;
    let mut rewritten: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The earlier question with the old thing taken out and the new one in its place, or None.\n\n    A new thing that carries its own preposition (\"and in the freezer too\", \"what about under the\n    sink\") goes where the old place was whole: the preposition in front of the old thing, any\n    \"my\"/\"the\" in between, and the thing itself come out together, so the question is asked again\n    as \"the smell in my freezer\" or \"the smell under the sink\". Where the earlier question has no\n    preposition in front of the thing (\"how do i clean the oven\") there is no place to put the\n    new one, and None is returned: a hedge, not \"the smell in my under the sink\".\n    ");
    words = mcall(&question.clone(), "split", &[]);
    new_words = mcall(&new.clone(), "split", &[]);
    at = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5774 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk5774[0].clone(); let mut word: Py = __unpk5774[1].clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&word.clone(), "strip", &[py_str("?.!,;:")]), &old.clone()))) {  __out.push(i.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_is(&at.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&new_words.clone())) } }) {
    return Py::None;
    }
    head = mcall(&mcall(&geti(&new_words.clone(), &Py::Int(0)), "strip", &[py_str("?.!,;:")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PREPOSITION(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREPOSITION_TAIL(), &head.clone())) } }) {
    start = at.clone();
    while py_truthy(&{ let __b = gt(&start.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINER(), &mcall(&mcall(&geti(&words.clone(), &sub(&start.clone(), &Py::Int(1))), "strip", &[py_str("?.!,;:")]), "lower", &[]))) } }) {
    let __aug1 = sub(&start.clone(), &Py::Int(1));
    start = __aug1;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&start.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PREPOSITION(), &mcall(&mcall(&geti(&words.clone(), &sub(&start.clone(), &Py::Int(1))), "strip", &[py_str("?.!,;:")]), "lower", &[]))) } }) {
    return mcall(&py_str(" "), "join", &[add(&add(&slice_or_raise(&words.clone(), &Py::None, &start.clone(), &Py::None), &new_words.clone()), &slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    }
    let __aug2 = sub(&start.clone(), &Py::Int(1));
    start = __aug2;
    if py_truthy(&{ let __b = gt(&start.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREPOSITION_TAIL(), &mcall(&mcall(&geti(&words.clone(), &sub(&start.clone(), &Py::Int(1))), "strip", &[py_str("?.!,;:")]), "lower", &[]))) } }) {
    let __aug3 = sub(&start.clone(), &Py::Int(1));
    start = __aug3;
    }
    return mcall(&py_str(" "), "join", &[add(&add(&slice_or_raise(&words.clone(), &Py::None, &start.clone(), &Py::None), &new_words.clone()), &slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    }
    rewritten = mcall(&question.clone(), "replace", &[old.clone(), new.clone(), Py::Int(1)]);
    head = mcall(&mcall(&geti(&new_words.clone(), &neg(&Py::Int(1))), "strip", &[py_str("?.!,;:")]), "lower", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[new_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { gt(&mcall(&mcall(&rewritten.clone(), "lower", &[]), "count", &[head.clone()]), &Py::Int(1)) } }) {
    return Py::None;
    }
    return rewritten.clone();
    Py::None
}
pub fn _said_again(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this answer is the one just given, word for word.\n\n    A follow-up that hands back the paragraph it just gave is the one reply worse than a hedge.\n    The fridge entry is keyed on \"smell\" and says nothing about either place, so the rewritten\n    freezer question reaches it and the fridge paragraph comes back whole.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&last.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&last.clone(), "lower", &[]), "split", &[])])));
    Py::None
}
pub fn _brings_its_own_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut new: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut words: Py = Py::None;
    let _ = py_str("Whether the new thing is a place said with its own preposition (\"under the sink\").");
    words = mcall(&call_builtin("str", &[{ let __b = new.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREPOSITION(), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str("?.!,;:")]), "lower", &[]))) } };
    Py::None
}
pub fn names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut old: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut question: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { Py::None };
    let mut asked: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut new: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when an answer to the rewritten question is about the new thing, not the old one.\n\n    \"can you give me a dessert idea\" reaches the crumble, and the crumble is not about dinners:\n    an answer that still talks about the thing the line replaced is the earlier answer said\n    again (\"how long do i roast a dessert for\" reaches the chicken entry and talks chicken), and\n    handing that over is worse than not answering. An answer that names neither is taken, since\n    the entry was reached by a question about the new thing.\n\n    A place that brings its own preposition is the one case where naming neither is not enough.\n    The rewritten question (\"how do i get rid of the smell in the shed\") reaches entries on a\n    reading of a line nobody wrote, and \"the smell in the shed\" reaches the weight-loss entry and\n    gets \"fill up on protein and fibre first\" - an answer about nothing the person asked. So for\n    a place the answer also has to be about the question: one of its own subject words\n    (\"smell\" for the smell in the shed's sink) has to be in what comes back, or the line keeps\n    its hedge.\n    ");
    new = thing(&[text.clone()]);
    said = _said(&[answer.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&new.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_said_again(&[answer.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[ ']"), new.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STOP(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &_stem(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&_brings_its_own_place(&[new.clone()])) {
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[ ']"), _said(&[question.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STOP(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &_stem(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(!py_contains(&said.clone(), &_stem(&[{ let __b = old.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.swap".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK" => Some(g_ASK()),
            "ASKS_A_THING" => Some(g_ASKS_A_THING()),
            "A_CLAUSE" => Some(g_A_CLAUSE()),
            "A_COURTESY" => Some(g_A_COURTESY()),
            "DEICTIC" => Some(g_DEICTIC()),
            "DETERMINER" => Some(g_DETERMINER()),
            "HEDGE_OFF" => Some(g_HEDGE_OFF()),
            "LEAD" => Some(g_LEAD()),
            "OPENERS" => Some(g_OPENERS()),
            "OWNER" => Some(g_OWNER()),
            "OWNER_WORDS" => Some(g_OWNER_WORDS()),
            "PERSON_FILLER" => Some(g_PERSON_FILLER()),
            "PERSON_LEAD" => Some(g_PERSON_LEAD()),
            "PREPOSITION" => Some(g_PREPOSITION()),
            "PREPOSITION_TAIL" => Some(g_PREPOSITION_TAIL()),
            "REPAIR" => Some(g_REPAIR()),
            "STOP" => Some(g_STOP()),
            "TAIL_OFF" => Some(g_TAIL_OFF()),
            "TAKE_BACK" => Some(g_TAKE_BACK()),
            "THING" => Some(g_THING()),
            "_a_second_place" => Some(py_func("_a_second_place", |args: &[Py]| _a_second_place(args))),
            "_about" => Some(py_func("_about", |args: &[Py]| _about(args))),
            "_after_a_take_back" => Some(py_func("_after_a_take_back", |args: &[Py]| _after_a_take_back(args))),
            "_brings_its_own_place" => Some(py_func("_brings_its_own_place", |args: &[Py]| _brings_its_own_place(args))),
            "_earlier" => Some(py_func("_earlier", |args: &[Py]| _earlier(args))),
            "_named_somebody" => Some(py_func("_named_somebody", |args: &[Py]| _named_somebody(args))),
            "_reads_as_a_question" => Some(py_func("_reads_as_a_question", |args: &[Py]| _reads_as_a_question(args))),
            "_replace" => Some(py_func("_replace", |args: &[Py]| _replace(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_said_again" => Some(py_func("_said_again", |args: &[Py]| _said_again(args))),
            "_spans" => Some(py_func("_spans", |args: &[Py]| _spans(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_place_after_the_same_preposition" => Some(py_func("_the_place_after_the_same_preposition", |args: &[Py]| _the_place_after_the_same_preposition(args))),
            "_the_preposition_of" => Some(py_func("_the_preposition_of", |args: &[Py]| _the_preposition_of(args))),
            "_word" => Some(py_func("_word", |args: &[Py]| _word(args))),
            "candidates" => Some(py_func("candidates", |args: &[Py]| candidates(args))),
            "is_repair" => Some(py_func("is_repair", |args: &[Py]| is_repair(args))),
            "names" => Some(py_func("names", |args: &[Py]| names(args))),
            "names_person" => Some(py_func("names_person", |args: &[Py]| names_person(args))),
            "placed" => Some(py_func("placed", |args: &[Py]| placed(args))),
            "rewritten" => Some(py_func("rewritten", |args: &[Py]| rewritten(args))),
            "rewritten_person" => Some(py_func("rewritten_person", |args: &[Py]| rewritten_person(args))),
            "someone" => Some(py_func("someone", |args: &[Py]| someone(args))),
            "thing" => Some(py_func("thing", |args: &[Py]| thing(args))),
            _ => None,
        }),
        set: None,
    }))
}

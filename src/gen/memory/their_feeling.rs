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


pub static ASKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEFINITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISLIKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIKES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHOUT_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITH_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKING() -> Py { ensure_init(); ASKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); if let Ok(mut w) = ASKING.write() { *w = v; } }
pub fn g_A_THING() -> Py { ensure_init(); A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_THING.write() { *w = v; } }
pub fn g_DEFINITE() -> Py { ensure_init(); DEFINITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEFINITE(v: Py) { ensure_init(); if let Ok(mut w) = DEFINITE.write() { *w = v; } }
pub fn g_DISLIKE() -> Py { ensure_init(); DISLIKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISLIKE(v: Py) { ensure_init(); if let Ok(mut w) = DISLIKE.write() { *w = v; } }
pub fn g_FEELING() -> Py { ensure_init(); FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_LIKES() -> Py { ensure_init(); LIKES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIKES(v: Py) { ensure_init(); if let Ok(mut w) = LIKES.write() { *w = v; } }
pub fn g_PERSON() -> Py { ensure_init(); PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = PERSON.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_WITHOUT_THING() -> Py { ensure_init(); WITHOUT_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_THING(v: Py) { ensure_init(); if let Ok(mut w) = WITHOUT_THING.write() { *w = v; } }
pub fn g_WITH_THING() -> Py { ensure_init(); WITH_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITH_THING(v: Py) { ensure_init(); if let Ok(mut w) = WITH_THING.write() { *w = v; } }
pub fn g_WORDS() -> Py { ensure_init(); WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WORDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Somebody the chat has named, and how they are finding a thing the chat has named.\n\n    my son is 7 / hes just started school / he hates it\n      was: I'm listening - say more if it helps.\n      now: Starting school is hard going for your son, then. What's the worst part?\n\n    my daughter started a new job / she hates it\n      was: Got it. Is there anything you'd like a hand with?\n      now: ...\n\n    my mum is in hospital / she hates the food\n      was: Go on. I'm here.\n      now: ...\n\nThe composer's note that fits any message (\"I'm listening - say more if it helps\") is what a\nfeeling report from somebody else got, because every reader of feelings in the bot is about the\nperson typing: `obstacle.the_thing` reads \"i hate it\" and refuses the pronoun on purpose, and the\nacknowledgements read a day going badly for them. A line about a third person (\"he hates it\",\n\"she cant stand the food\", \"my partner hates his job\") names no trouble of their own, so it fell\nthrough to the note.\n\nWhat the line carries is a person and a feeling; what the chat carries is what the feeling is\nabout, which the question the bot asked last turn (\"How's it going so far?\", \"How were the first\ndays?\") is the one thing still owed. So the reply names the person and asks for the part of it\nthat is worst - a friend's question, not a line that would fit any message.\n\nThe reader refuses more than it takes:\n\n* a question (\"do you think he hates it?\") is for the answer modules;\n* the first person (\"i hate it\") is `obstacle`'s and the composer's, and the frame does not fit;\n* an opening line naming a fresh subject in a plain word (\"my son hates school\") is a subject of\n  its own and the knowledge base answers it with the school answer - reading it here would put a\n  question in place of an answer. A first line is read only when the thing is a possessive or a\n  definite one (\"my partner hates his job\", \"she hates the food\"), which is a continuation of\n  something the person has in mind rather than a request about a subject;\n* a pronoun with nobody in the chat for it to point at names nobody, and \"he hates it\" three\n  lines after the last person is a guess.\n\nThe feeling is read in both directions: a thing somebody hates, and a thing somebody loves, since\n\"That's a good sign - what does he like best about his teacher?\" is the reply to the second and\nthe school-letters answer the bot gave was about a different question.\n");
    s_LEAD(py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|honestly|god)\\W+)*"));
    s_PERSON(py_str("(?P<who>he|she|they|him|her|them|(?:my|our|his|her|their)\\s+[a-z][a-z'-]*)"));
    s_DISLIKE(py_str("(?:hates?|hated|dislikes?|disliked|cant\\s+stand|cant\\s+face|can\\s*not\\s+stand|couldnt\\s+stand|doesnt\\s+like|does\\s*not\\s+like|didnt\\s+like|isnt\\s+keen\\s+on|not\\s+keen\\s+on|is\\s+struggling\\s+with|struggles\\s+with|is\\s+fed\\s+up\\s+with)"));
    s_LIKES(py_str("(?:loves?|loved|adores?|adored|likes?|liked|enjoys?|enjoyed|is\\s+loving|is\\s+enjoying|gets?\\s+on\\s+well\\s+with|has\\s+taken\\s+to)"));
    s_FEELING(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&g_LEAD(), &g_PERSON()), &py_str("\\s+(?:(?:really|just|now|still|actually|absolutely|completely|totally|properly)\\s+)*(?P<how>")), &g_DISLIKE()), &py_str("|")), &g_LIKES()), &py_str(")\\s+(?P<thing>.+?)\\s*[.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING(stdlib_call("re", "compile", &[py_str("^(?:(?:the|his|her|their|its|my|our|a|an)\\s+)?[a-z][a-z'-]*(?:\\s+[a-z][a-z'-]*){0,3}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:it|that|this|these|those|them|him|her|everything|all\\s+of\\s+it|the\\s+whole\\s+thing)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what|why|how|should|shall|can|could|would|any\\s+idea|advice|help|do\\s+you\\s+think|is\\s+it|does\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEFINITE(stdlib_call("re", "compile", &[py_str("^(?:the|his|her|their|its|my|our|this|that|these|those)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDS(call_builtin("frozenset", &[mcall(&py_str("it that this these those them him her everything all\n    the a an and but so oh also well yeah yep ok okay right alright\n    really just now still actually absolutely completely totally properly\n    very quite bit lot little much more less"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WITH_THING(py_dict(vec![(py_str("no"), py_tuple(vec![py_str("{thingcap} is hard going for {person}, then. What's the worst part?"), py_str("That's a shame about {thing}. What's {person} finding hardest?"), py_str("{thingcap} isn't working for {person}, then. What would make the most difference?")])), (py_str("yes"), py_tuple(vec![py_str("That's good - {thingcap} is working for {person}, then. What does {they} like best?"), py_str("Good news. What is it about {thing} that {person} has taken to?"), py_str("That's the best news of the day, that one. What's {person} enjoying most about {thing}?")]))]));
    s_WITHOUT_THING(py_dict(vec![(py_str("no"), py_tuple(vec![py_str("That's hard going for {person}, then. What's the worst part?"), py_str("Not what you hoped for {person}. Which bit is worst?"), py_str("That's a shame. What's {person} finding hardest?")])), (py_str("yes"), py_tuple(vec![py_str("That's good news. What does {person} like best about it?"), py_str("Good to hear - what's {person} enjoying most?"), py_str("That's a good sign. What made the difference for {person}?")]))]));
}
pub fn _the_last_person(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The person the line is about, as the store words them: \"son\", \"partner\", \"mum\".");
    low = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    match_ = mcall(&g_FEELING(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    who = mcall(&match_.clone(), "group", &[py_str("who")]);
    if py_truthy(&mcall(&who.clone(), "startswith", &[py_tuple(vec![py_str("my "), py_str("our ")])])) {
    return py_tuple(vec![add(&py_str("your "), &geti(&mcall(&who.clone(), "split", &[]), &neg(&Py::Int(1)))), geti(&mcall(&who.clone(), "split", &[]), &neg(&Py::Int(1)))]);
    }
    if py_truthy(&mcall(&who.clone(), "startswith", &[py_tuple(vec![py_str("his "), py_str("her "), py_str("their ")])])) {
    return py_tuple(vec![py_str(""), who.clone()]);
    }
    person = py_str("");
    let __try1 = exc::catch(|| {
    person = { let __b = crate::gen::memory::relations::_person_from_pronoun(&[low.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
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
    person = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("^(?:my|our|your|his|her|their)\\b"), person.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    person = add(&py_str("your "), &person.clone());
    }
    return py_tuple(vec![person.clone(), who.clone()]);
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
    let mut _u: Py = Py::None;
    let mut back: Py = Py::None;
    let mut filled: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut how: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reply to a line about how somebody else is finding a thing, or None.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&g_ASKING(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    match_ = mcall(&g_FEELING(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_last_person(&[low.clone(), memory.clone()]), 2, None);
    person = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return Py::None;
    }
    thing = mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" .,!;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    back = call_builtin("bool", &[mcall(&g_POINTS_BACK(), "match", &[thing.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_THING(), "match", &[thing.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DEFINITE(), "match", &[thing.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_earlier_turn(&[memory.clone(), low.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DEFINITE(), "match", &[thing.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_something(&[thing.clone()]))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_an_earlier_turn(&[memory.clone(), low.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DEFINITE(), "match", &[thing.clone()]))) } }) {
    return Py::None;
    }
    how = if py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("(?:^|\\s)(?:"), &g_DISLIKE()), &py_str(")(?:\\s|$)")), mcall(&match_.clone(), "group", &[py_str("how")])]).unwrap_or_else(|e| exc::raise(e))) { py_str("no") } else { py_str("yes") };
    pool = if py_truthy(&back.clone()) { geti(&g_WITHOUT_THING(), &how.clone()) } else { geti(&g_WITH_THING(), &how.clone()) };
    filled = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut frame: Py = __item.clone(); __out.push(_fill(&[frame.clone(), person.clone(), if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) { thing.clone() } else { py_str("") }])); } py_list(__out) })() };
    return _vary(&[filled.clone(), memory.clone(), low.clone()]);
    Py::None
}
pub fn _says_something(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the thing named is more than the words that say nothing on their own.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z']+"), mcall(&thing.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _an_earlier_turn(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut an_earlier_turn: Py = py_func("an_earlier_turn", |args: &[Py]| crate::gen::memory::history::an_earlier_turn(args));
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_call(&an_earlier_turn, &[memory.clone(), low.clone()])]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _fill(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut said: Py = Py::None;
    let mut they: Py = Py::None;
    let _ = py_str("One frame with the person and the thing in it, and the words around them agreeing.");
    they = geti(&crate::gen::compose::remark::person_pronouns(&[person.clone()]), &Py::Int(0));
    said = mcall(&mcall(&frame.clone(), "replace", &[py_str("{person}"), person.clone()]), "replace", &[py_str("{they}"), they.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&frame.clone(), &py_str("{thing"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&frame.clone(), &py_str("{Thing}"))) } }) {
    said = mcall(&mcall(&mcall(&said.clone(), "replace", &[py_str("{thingcap}"), add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))]), "replace", &[py_str("{Thing}"), add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))]), "replace", &[py_str("{thing}"), thing.clone()]);
    }
    return add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _vary(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let _ = py_str("One of the ways to say it, not the same one twice while the others are unused.");
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::gen::compose::pick(args));
    return py_call(&pick, &[options.clone(), memory.clone(), line.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.their_feeling".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKING" => Some(g_ASKING()),
            "A_THING" => Some(g_A_THING()),
            "DEFINITE" => Some(g_DEFINITE()),
            "DISLIKE" => Some(g_DISLIKE()),
            "FEELING" => Some(g_FEELING()),
            "LEAD" => Some(g_LEAD()),
            "LIKES" => Some(g_LIKES()),
            "PERSON" => Some(g_PERSON()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "WITHOUT_THING" => Some(g_WITHOUT_THING()),
            "WITH_THING" => Some(g_WITH_THING()),
            "WORDS" => Some(g_WORDS()),
            "_an_earlier_turn" => Some(py_func("_an_earlier_turn", |args: &[Py]| _an_earlier_turn(args))),
            "_fill" => Some(py_func("_fill", |args: &[Py]| _fill(args))),
            "_says_something" => Some(py_func("_says_something", |args: &[Py]| _says_something(args))),
            "_the_last_person" => Some(py_func("_the_last_person", |args: &[Py]| _the_last_person(args))),
            "_vary" => Some(py_func("_vary", |args: &[Py]| _vary(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}

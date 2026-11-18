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


thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFINITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING() -> Py { ensure_init(); G_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING(v: Py) { ensure_init(); G_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFINITE() -> Py { ensure_init(); G_DEFINITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFINITE(v: Py) { ensure_init(); G_DEFINITE.with(|c| *c.borrow_mut() = v); }
pub fn g_DISLIKE() -> Py { ensure_init(); G_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKE(v: Py) { ensure_init(); G_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING() -> Py { ensure_init(); G_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); G_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LIKES() -> Py { ensure_init(); G_LIKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIKES(v: Py) { ensure_init(); G_LIKES.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON() -> Py { ensure_init(); G_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); G_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_THING() -> Py { ensure_init(); G_WITHOUT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_THING(v: Py) { ensure_init(); G_WITHOUT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_THING() -> Py { ensure_init(); G_WITH_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_THING(v: Py) { ensure_init(); G_WITH_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDS() -> Py { ensure_init(); G_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDS(v: Py) { ensure_init(); G_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Somebody the chat has named, and how they are finding a thing the chat has named.\n\n    my son is 7 / hes just started school / he hates it\n      was: I'm listening - say more if it helps.\n      now: Starting school is hard going for your son, then. What's the worst part?\n\n    my daughter started a new job / she hates it\n      was: Got it. Is there anything you'd like a hand with?\n      now: ...\n\n    my mum is in hospital / she hates the food\n      was: Go on. I'm here.\n      now: ...\n\nThe composer's note that fits any message (\"I'm listening - say more if it helps\") is what a\nfeeling report from somebody else got, because every reader of feelings in the bot is about the\nperson typing: `obstacle.the_thing` reads \"i hate it\" and refuses the pronoun on purpose, and the\nacknowledgements read a day going badly for them. A line about a third person (\"he hates it\",\n\"she cant stand the food\", \"my partner hates his job\") names no trouble of their own, so it fell\nthrough to the note.\n\nWhat the line carries is a person and a feeling; what the chat carries is what the feeling is\nabout, which the question the bot asked last turn (\"How's it going so far?\", \"How were the first\ndays?\") is the one thing still owed. So the reply names the person and asks for the part of it\nthat is worst - a friend's question, not a line that would fit any message.\n\nThe reader refuses more than it takes:\n\n* a question (\"do you think he hates it?\") is for the answer modules;\n* the first person (\"i hate it\") is `obstacle`'s and the composer's, and the frame does not fit;\n* an opening line naming a fresh subject in a plain word (\"my son hates school\") is a subject of\n  its own and the knowledge base answers it with the school answer - reading it here would put a\n  question in place of an answer. A first line is read only when the thing is a possessive or a\n  definite one (\"my partner hates his job\", \"she hates the food\"), which is a continuation of\n  something the person has in mind rather than a request about a subject;\n* a pronoun with nobody in the chat for it to point at names nobody, and \"he hates it\" three\n  lines after the last person is a guess.\n\nThe feeling is read in both directions: a thing somebody hates, and a thing somebody loves, since\n\"That's a good sign - what does he like best about his teacher?\" is the reply to the second and\nthe school-letters answer the bot gave was about a different question.\n");
    s_LEAD(py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|honestly|god)\\W+)*"));
    s_PERSON(py_str("(?P<who>he|she|they|him|her|them|(?:my|our|his|her|their)\\s+[a-z][a-z'-]*)"));
    s_DISLIKE(py_str("(?:hates?|hated|dislikes?|disliked|cant\\s+stand|cant\\s+face|can\\s*not\\s+stand|couldnt\\s+stand|doesnt\\s+like|does\\s*not\\s+like|didnt\\s+like|isnt\\s+keen\\s+on|not\\s+keen\\s+on|is\\s+struggling\\s+with|struggles\\s+with|is\\s+fed\\s+up\\s+with)"));
    s_LIKES(py_str("(?:loves?|loved|adores?|adored|likes?|liked|enjoys?|enjoyed|is\\s+loving|is\\s+enjoying|gets?\\s+on\\s+well\\s+with|has\\s+taken\\s+to)"));
    s_FEELING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_LEAD(), &g_PERSON()), &py_str("\\s+(?:(?:really|just|now|still|actually|absolutely|completely|totally|properly)\\s+)*(?P<how>")), &g_DISLIKE()), &py_str("|")), &g_LIKES()), &py_str(")\\s+(?P<thing>.+?)\\s*[.!]*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:the|his|her|their|its|my|our|a|an)\\s+)?[a-z][a-z'-]*(?:\\s+[a-z][a-z'-]*){0,3}$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:it|that|this|these|those|them|him|her|everything|all\\s+of\\s+it|the\\s+whole\\s+thing)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?|\\b(?:what|why|how|should|shall|can|could|would|any\\s+idea|advice|help|do\\s+you\\s+think|is\\s+it|does\\s+it)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DEFINITE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the|his|her|their|its|my|our|this|that|these|those)\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("it that this these those them him her everything all\n    the a an and but so oh also well yeah yep ok okay right alright\n    really just now still actually absolutely completely totally properly\n    very quite bit lot little much more less"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FEELING(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    who = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("my "), py_str("our ")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "startswith", args))(&__args) }) {
    return py_tuple(vec![op_add(&py_str("your "), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1)))), geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1)))]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("his "), py_str("her "), py_str("their ")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "startswith", args))(&__args) }) {
    return py_tuple(vec![py_str(""), who.clone()]);
    }
    person = py_str("");
    let __try1 = exc::catch(|| {
    person = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::relations::_person_from_pronoun(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
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
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:my|our|your|his|her|their)\\b")); __args.push(person.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    person = op_add(&py_str("your "), &person.clone());
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
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKING(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FEELING(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_last_person(args))(&__args) }, 2, None);
    person = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTS_BACK(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING(), "match", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINITE(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_earlier_turn(args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINITE(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_something(args))(&__args) })) } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_earlier_turn(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINITE(), "match", args))(&__args) })) } }) {
    return Py::None;
    }
    how = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?:^|\\s)(?:"), &g_DISLIKE()), &py_str(")(?:\\s|$)"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { py_str("no") } else { py_str("yes") };
    pool = if py_truthy(&back.clone()) { geti(&g_WITHOUT_THING(), &how.clone()) } else { geti(&g_WITH_THING(), &how.clone()) };
    filled = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(frame.clone()); __args.push(person.clone()); __args.push(if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) { thing.clone() } else { py_str("") }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(filled.clone()); __args.push(memory.clone()); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _vary(args))(&__args) };
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
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
    let mut an_earlier_turn: Py = py_func("an_earlier_turn", |args: &[Py]| crate::memory::history::an_earlier_turn(args));
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&an_earlier_turn, args))(&__args) }]);
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
    they = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(person.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.remark", "person_pronouns", args))(&__args) }, &Py::Int(0));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{they}")); __args.push(they.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{person}")); __args.push(person.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&frame.clone(), &py_str("{thing"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&frame.clone(), &py_str("{Thing}"))) } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{thing}")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Thing}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{thingcap}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) };
    }
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    let mut pick: Py = py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&pick, args))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKING" => s_ASKING(v),
            "A_THING" => s_A_THING(v),
            "DEFINITE" => s_DEFINITE(v),
            "DISLIKE" => s_DISLIKE(v),
            "FEELING" => s_FEELING(v),
            "LEAD" => s_LEAD(v),
            "LIKES" => s_LIKES(v),
            "PERSON" => s_PERSON(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "WITHOUT_THING" => s_WITHOUT_THING(v),
            "WITH_THING" => s_WITH_THING(v),
            "WORDS" => s_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.their_feeling", module_obj);
    pyrt::xrefs::reg_fn("memory.their_feeling", "_the_last_person", py_func("_the_last_person", |args: &[Py]| _the_last_person(args)));
    pyrt::xrefs::reg_fn("memory.their_feeling", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.their_feeling", "_says_something", py_func("_says_something", |args: &[Py]| _says_something(args)));
    pyrt::xrefs::reg_fn("memory.their_feeling", "_an_earlier_turn", py_func("_an_earlier_turn", |args: &[Py]| _an_earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.their_feeling", "_fill", py_func("_fill", |args: &[Py]| _fill(args)));
    pyrt::xrefs::reg_fn("memory.their_feeling", "_vary", py_func("_vary", |args: &[Py]| _vary(args)));
    pyrt::xrefs::reg_access("memory.their_feeling", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.their_feeling", "A_THING", g_A_THING, s_A_THING);
    pyrt::xrefs::reg_access("memory.their_feeling", "DEFINITE", g_DEFINITE, s_DEFINITE);
    pyrt::xrefs::reg_access("memory.their_feeling", "DISLIKE", g_DISLIKE, s_DISLIKE);
    pyrt::xrefs::reg_access("memory.their_feeling", "FEELING", g_FEELING, s_FEELING);
    pyrt::xrefs::reg_access("memory.their_feeling", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.their_feeling", "LIKES", g_LIKES, s_LIKES);
    pyrt::xrefs::reg_access("memory.their_feeling", "PERSON", g_PERSON, s_PERSON);
    pyrt::xrefs::reg_access("memory.their_feeling", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.their_feeling", "WITHOUT_THING", g_WITHOUT_THING, s_WITHOUT_THING);
    pyrt::xrefs::reg_access("memory.their_feeling", "WITH_THING", g_WITH_THING, s_WITH_THING);
    pyrt::xrefs::reg_access("memory.their_feeling", "WORDS", g_WORDS, s_WORDS);
}

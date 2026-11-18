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
pub static ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_ING_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DETERMINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LINKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MARKERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PHOBIA_GERUNDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TRAILING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASK() -> Py { ensure_init(); ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ASK.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_FEAR() -> Py { ensure_init(); FEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEAR(v: Py) { ensure_init(); if let Ok(mut w) = FEAR.write() { *w = v; } }
pub fn g__AN_ING_NOUN() -> Py { ensure_init(); _AN_ING_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_ING_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = _AN_ING_NOUN.write() { *w = v; } }
pub fn g__DETERMINERS() -> Py { ensure_init(); _DETERMINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DETERMINERS(v: Py) { ensure_init(); if let Ok(mut w) = _DETERMINERS.write() { *w = v; } }
pub fn g__LINKS() -> Py { ensure_init(); _LINKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LINKS(v: Py) { ensure_init(); if let Ok(mut w) = _LINKS.write() { *w = v; } }
pub fn g__MARKERS() -> Py { ensure_init(); _MARKERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MARKERS(v: Py) { ensure_init(); if let Ok(mut w) = _MARKERS.write() { *w = v; } }
pub fn g__PHOBIA_GERUNDS() -> Py { ensure_init(); _PHOBIA_GERUNDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PHOBIA_GERUNDS(v: Py) { ensure_init(); if let Ok(mut w) = _PHOBIA_GERUNDS.write() { *w = v; } }
pub fn g__STOP() -> Py { ensure_init(); _STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STOP(v: Py) { ensure_init(); if let Ok(mut w) = _STOP.write() { *w = v; } }
pub fn g__TRAILING_FILLER() -> Py { ensure_init(); _TRAILING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TRAILING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = _TRAILING_FILLER.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A fear of something, and how to get over it.\n\n\"how do i get over my fear of needles\" was answered with what pins and needles are, \"a fear of\nspiders\" with the silk glands, \"a fear of flying\" with the book-six-weeks-ahead fares, and \"i\nhave a phobia of lifts\" with \"Noted: you have a phobia of lifts.\" Four phrasings, four different\nwrong turns, and one shape underneath: the line names a thing the person is frightened of and\nasks how to stop being frightened of it.\n\nSo this is one row, keyed on the fear itself rather than on any of the things people fear. The\nthing named is quoted back in the first sentence - the reply is about *their* fear, not about\nfear in general - and the method is the one with the evidence: graded exposure, a ladder of\nsmall steps, breath rather than reassurance.\n\nTwo kinds of line are held out. A child's fear is a parent's question, and it is answered where\na child's fear lives (the dogs row, the dark at bedtime); a specific fear that already has its\nown answer - the dentist, the doctor's needle - keeps that answer, because a dental appointment\nneeds the phone call in it, not a ladder.\n");
    s_ANSWERS(py_tuple(vec![py_str("health")]));
    s_FEAR(py_tuple(vec![py_str("fear of"), py_str("fear off"), py_str("phobia"), py_str("phobias"), py_str("phobic of"), py_str("phobic about"), py_str("scared of"), py_str("scared about"), py_str("scared stiff"), py_str("afraid of"), py_str("terrified of"), py_str("terrified by"), py_str("frightened of"), py_str("dread"), py_str("dreads"), py_str("dreading"), py_str("petrified of"), py_str("hate flying"), py_str("hate spiders"), py_str("cant stand spiders"), py_str("panic when i"), py_str("panics when"), py_str("panic attack when"), py_str("panic attacks when"), py_str("get the fear"), py_str("get over"), py_str("getting over"), py_str("got over"), py_str("overcome"), py_str("overcoming"), py_str("cure my"), py_str("stop being scared"), py_str("stop being afraid"), py_str("not be scared"), py_str("less scared"), py_str("irrational fear"), py_str("irrational"), py_str("how do i stop being")]));
    s_ASK(py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("how to"), py_str("what can i do"), py_str("what should i do"), py_str("help"), py_str("get over"), py_str("getting over"), py_str("overcome"), py_str("overcoming"), py_str("cure"), py_str("stop"), py_str("deal with"), py_str("cope"), py_str("manage"), py_str("fix"), py_str("advice"), py_str("tips"), py_str("any tips"), py_str("any advice"), py_str("whats the best way"), py_str("what is the best way"), py_str("is there a way"), py_str("can i"), py_str("am i"), py_str("why am i"), py_str("what is wrong with me"), py_str("im scared of"), py_str("i am scared of"), py_str("im afraid of"), py_str("i am afraid of"), py_str("im terrified of"), py_str("i am terrified of"), py_str("im frightened of"), py_str("i am frightened of"), py_str("im petrified of"), py_str("i am petrified of"), py_str("im phobic of"), py_str("i am phobic of"), py_str("im phobic about"), py_str("i am phobic about")]));
    s__MARKERS(py_tuple(vec![py_str("fear of"), py_str("fear off"), py_str("phobia of"), py_str("phobia about"), py_str("phobic of"), py_str("phobic about"), py_str("scared of"), py_str("scared about"), py_str("afraid of"), py_str("terrified of"), py_str("terrified by"), py_str("frightened of"), py_str("petrified of"), py_str("phobia")]));
    s__STOP(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("ive"), py_str("i've"), py_str("id"), py_str("and"), py_str("but"), py_str("so"), py_str("because"), py_str("cos"), py_str("when"), py_str("since"), py_str("what"), py_str("whats"), py_str("how"), py_str("can"), py_str("could"), py_str("should"), py_str("do"), py_str("does"), py_str("did"), py_str("help"), py_str("any"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("it"), py_str("its"), py_str("my"), py_str("we"), py_str("he"), py_str("she"), py_str("they"), py_str("them"), py_str("that"), py_str("this"), py_str("never"), py_str("always"), py_str("really"), py_str("very"), py_str("too"), py_str("now"), py_str("please"), py_str("thanks"), py_str("thank"), py_str("need"), py_str("have"), py_str("has"), py_str("to"), py_str("will"), py_str("would"), py_str("am"), py_str("for"), py_str("if"), py_str("of"), py_str("get"), py_str("getting"), py_str("over"), py_str("overcome"), py_str("stop"), py_str("deal"), py_str("cope"), py_str("even"), py_str("just"), py_str("also"), py_str("about"), py_str("on"), py_str("at"), py_str("in"), py_str("with"), py_str("from"), py_str("there"), py_str("here"), py_str("as"), py_str("than"), py_str("then"), py_str("make"), py_str("makes")])]).unwrap_or_else(|e| exc::raise(e)));
    s__TRAILING_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("tbh"), py_str("tbf"), py_str("lol"), py_str("lols"), py_str("haha"), py_str("hah"), py_str("tho"), py_str("though"), py_str("rn"), py_str("atm"), py_str("idk"), py_str("nvm"), py_str("omg"), py_str("honestly"), py_str("actually"), py_str("basically"), py_str("kinda"), py_str("sorta"), py_str("mate"), py_str("pal"), py_str("thanks"), py_str("thank"), py_str("thx"), py_str("please"), py_str("xx"), py_str("x"), py_str("hey"), py_str("now"), py_str("still"), py_str("again")])]).unwrap_or_else(|e| exc::raise(e)));
    s__LINKS(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("on"), py_str("in"), py_str("at"), py_str("into"), py_str("about"), py_str("with"), py_str("of"), py_str("near"), py_str("around"), py_str("from"), py_str("inside")])]).unwrap_or_else(|e| exc::raise(e)));
    s__DETERMINERS(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that"), py_str("its")])]).unwrap_or_else(|e| exc::raise(e)));
    s__PHOBIA_GERUNDS(call_builtin("frozenset", &[py_tuple(vec![py_str("flying"), py_str("driving"), py_str("vomiting"), py_str("being sick"), py_str("travelling"), py_str("traveling"), py_str("sailing"), py_str("cruising"), py_str("going out"), py_str("speaking"), py_str("presenting"), py_str("dating"), py_str("eating"), py_str("sleeping"), py_str("swallowing"), py_str("choking"), py_str("blushing"), py_str("sweating")])]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ING_NOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("building"), py_str("buildings"), py_str("ceiling"), py_str("ceilings"), py_str("string"), py_str("strings"), py_str("ring"), py_str("rings"), py_str("thing"), py_str("things"), py_str("evening"), py_str("evenings"), py_str("morning"), py_str("mornings"), py_str("pudding"), py_str("puddings"), py_str("filling"), py_str("icing"), py_str("sibling"), py_str("siblings"), py_str("offspring"), py_str("darling"), py_str("earring"), py_str("earrings"), py_str("duckling"), py_str("lightning"), py_str("clothing"), py_str("savings"), py_str("innings"), py_str("weddings"), py_str("meeting"), py_str("meetings")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fear-of-something")); __args.push(py_tuple(vec![g_FEAR(), g_ASK()])); __args.push(g__the_fear_reply()); __kws.extend(vec![("guard".to_string(), g__names_the_thing()), ("avoid".to_string(), py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my toddler"), py_str("my baby"), py_str("my grandson"), py_str("my granddaughter"), py_str("4 year old"), py_str("5 year old"), py_str("3 year old"), py_str("2 year old"), py_str("6 year old"), py_str("toddler"), py_str("puppy"), py_str("kitten"), py_str("my dog"), py_str("my cat"), py_str("my rabbit"), py_str("my horse"), py_str("the dog"), py_str("the cat"), py_str("dog"), py_str("dogs"), py_str("a dog"), py_str("puppies"), py_str("public speaking"), py_str("presentation"), py_str("presentations"), py_str("presenting"), py_str("speech"), py_str("speech in front"), py_str("talk in front"), py_str("talking in front"), py_str("speaking in front"), py_str("stage fright"), py_str("interview"), py_str("interviews"), py_str("wedding speech"), py_str("best man speech"), py_str("speaking up"), py_str("speak up"), py_str("spoke up"), py_str("talking in meetings"), py_str("meetings"), py_str("meeting"), py_str("standup"), py_str("stand-up"), py_str("stand up"), py_str("in the room"), py_str("say something in"), py_str("board meeting"), py_str("team meeting"), py_str("dentist"), py_str("dental"), py_str("hygienist"), py_str("doctor"), py_str("gp"), py_str("hospital appointment"), py_str("blood test"), py_str("blood tests"), py_str("injection"), py_str("vaccine"), py_str("jab"), py_str("anaesthetic"), py_str("surgery"), py_str("operation"), py_str("childbirth"), py_str("birth"), py_str("labour"), py_str("pregnancy"), py_str("scan"), py_str("mri"), py_str("x ray"), py_str("xray"), py_str("chemo"), py_str("needle phobia is stopping me"), py_str("lose"), py_str("lost my"), py_str("my job"), py_str("redundancy"), py_str("redundant"), py_str("being made redundant"), py_str("sacked"), py_str("fired"), py_str("unemployed"), py_str("monday"), py_str("mondays"), py_str("monday morning"), py_str("monday mornings"), py_str("work tomorrow"), py_str("going to work"), py_str("commitment"), py_str("intimacy"), py_str("intimate"), py_str("relationship"), py_str("relationships"), py_str("my marriage"), py_str("my partner"), py_str("my girlfriend"), py_str("my boyfriend"), py_str("missing out"), py_str("fomo"), py_str("being alone"), py_str("loneliness"), py_str("being judged"), py_str("rejection"), py_str("being rejected"), py_str("abandonment"), py_str("mice"), py_str("mouse"), py_str("rat"), py_str("rats"), py_str("rodent"), py_str("rodents"), py_str("kitchen"), py_str("mortgage"), py_str("mortgages"), py_str("arrears"), py_str("bailiff"), py_str("bailiffs"), py_str("debt"), py_str("debts"), py_str("council tax"), py_str("summons"), py_str("eviction"), py_str("evicted"), py_str("landlord"), py_str("court"), py_str("credit card"), py_str("overdraft"), py_str("swim"), py_str("swims"), py_str("swimming"), py_str("swimmer"), py_str("pool"), py_str("pools"), py_str("water"), py_str("sea"), py_str("face in the water"), py_str("lessons"), py_str("lesson"), py_str("instructor"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("horror"), py_str("watch"), py_str("watching"), py_str("tv"), py_str("series"), py_str("documentary"), py_str("why am i scared of dying"), py_str("death"), py_str("dying"), py_str("panic attack now"), py_str("heart is racing"), py_str("cant breathe"), py_str("chest pain"), py_str("suicidal"), py_str("kill myself"), py_str("abusive"), py_str("hit me"), py_str("hits me"), py_str("threatened"), py_str("stalker"), py_str("following me")])), ("report".to_string(), Py::Bool(true)), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _the_thing_they_fear(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut found: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The feared thing as the person wrote it, or \"\" when the line names none.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = py_str("");
    at = neg(&Py::Int(1));
    let __iter1 = py_iter_or_raise(&g__MARKERS());
    for __item in __iter1 {
    marker = __item.clone();
    pos = mcall(&said.clone(), "find", &[add(&add(&py_str(" "), &marker.clone()), &py_str(" "))]);
    if py_truthy(&lt(&pos.clone(), &Py::Int(0))) {
    if py_truthy(&mcall(&said.clone(), "startswith", &[add(&marker.clone(), &py_str(" "))])) {
    pos = neg(&Py::Int(1));
    } else {
    continue;
    }
    }
    if py_truthy(&{ let __b = lt(&at.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { { let __b = lt(&pos.clone(), &at.clone()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&pos.clone(), &at.clone())); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[marker.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } } } }) {
    at = pos.clone();
    found = marker.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    rest = mcall(&slice_or_raise(&said.clone(), &add(&add(&at.clone(), &call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1)), &Py::None, &Py::None), "strip", &[]);
    words = py_list(vec![]);
    doing = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&mcall(&rest.clone(), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str(",.!?;:\"'")]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    break;
    }
    if py_truthy(&ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &if py_truthy(&doing.clone()) { Py::Int(5) } else { Py::Int(3) })) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g__STOP(), &word.clone()))) {
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__LINKS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DETERMINERS(), &word.clone())) } } } }) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    continue;
    }
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } } } }) {
    doing = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__TRAILING_FILLER(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    while py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("on"), py_str("in"), py_str("at"), py_str("into"), py_str("about"), py_str("with"), py_str("of"), py_str("near"), py_str("around"), py_str("from"), py_str("inside")]), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    thing = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("it"), py_str("them"), py_str("that"), py_str("this")]), &thing.clone()))) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _the_fear_reply(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut thing: Py = Py::None;
    thing = _the_thing_they_fear(&[understood.clone()]);
    opening = if py_truthy(&thing.clone()) { modulo(&py_str("A fear of %s is learned, so it can be unlearned"), &thing.clone()) } else { py_str("A fear like that is learned, so it can be unlearned") };
    return add(&opening.clone(), &py_str(" in small steps: write a ladder from the easiest version of it to the hardest - a photo of it, then the smallest real version, then all of it - and only move up when the step you are on has stopped raising your heart. Pick the easiest rung and stay with it until the wave passes: leaving early is what teaches the fear that it was right. If it is limiting your life, ask the GP for CBT with exposure, which is the treatment with the best evidence."));
    Py::None
}
pub fn _names_the_thing(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Only a line that names what is feared: a bare \"what is a phobia\" is the definition card's.\n\n    A definition of the word (\"what is a phobia\", \"why am i scared all the time\") is a different\n    answer from \"how do I stop being frightened of X\", and one row cannot be both. This row takes\n    the named thing and passes the rest, so the phobia card that explains the word keeps it.\n\n    A gerund in that place is a situation and not a phobia, unless it is one of the classic\n    ones, so \"im scared of losing my job\" keeps the redundancy advice and \"how do i overcome my\n    fear of failure at work\" is still this row's line - `work` could not be in the avoid list to\n    settle it, because that word takes the failure line with it.\n    ");
    thing = _the_thing_they_fear(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    first = geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&first.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__AN_ING_NOUN(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__PHOBIA_GERUNDS(), &first.clone())) } } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.fears".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK" => Some(g_ASK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FEAR" => Some(g_FEAR()),
            "_AN_ING_NOUN" => Some(g__AN_ING_NOUN()),
            "_DETERMINERS" => Some(g__DETERMINERS()),
            "_LINKS" => Some(g__LINKS()),
            "_MARKERS" => Some(g__MARKERS()),
            "_PHOBIA_GERUNDS" => Some(g__PHOBIA_GERUNDS()),
            "_STOP" => Some(g__STOP()),
            "_TRAILING_FILLER" => Some(g__TRAILING_FILLER()),
            "_names_the_thing" => Some(py_func("_names_the_thing", |args: &[Py]| _names_the_thing(args))),
            "_the_fear_reply" => Some(py_func("_the_fear_reply", |args: &[Py]| _the_fear_reply(args))),
            "_the_thing_they_fear" => Some(py_func("_the_thing_they_fear", |args: &[Py]| _the_thing_they_fear(args))),
            _ => None,
        }),
        set: None,
    }))
}

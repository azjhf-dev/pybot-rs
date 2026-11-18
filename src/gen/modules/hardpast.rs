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


pub static A_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLOSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLOSING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HARD_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HARD_PAST_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LOSS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEWS_ARRIVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_OWNED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_BETWEEN() -> Py { ensure_init(); A_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = A_BETWEEN.write() { *w = v; } }
pub fn g_A_CLOSED() -> Py { ensure_init(); A_CLOSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLOSED(v: Py) { ensure_init(); if let Ok(mut w) = A_CLOSED.write() { *w = v; } }
pub fn g_A_CLOSING_WORD() -> Py { ensure_init(); A_CLOSING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLOSING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_CLOSING_WORD.write() { *w = v; } }
pub fn g_A_HARD_PAST() -> Py { ensure_init(); A_HARD_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HARD_PAST(v: Py) { ensure_init(); if let Ok(mut w) = A_HARD_PAST.write() { *w = v; } }
pub fn g_A_HARD_PAST_SAID() -> Py { ensure_init(); A_HARD_PAST_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HARD_PAST_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_HARD_PAST_SAID.write() { *w = v; } }
pub fn g_A_LOSS() -> Py { ensure_init(); A_LOSS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LOSS(v: Py) { ensure_init(); if let Ok(mut w) = A_LOSS.write() { *w = v; } }
pub fn g_A_NEWS_ARRIVING() -> Py { ensure_init(); A_NEWS_ARRIVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_ARRIVING(v: Py) { ensure_init(); if let Ok(mut w) = A_NEWS_ARRIVING.write() { *w = v; } }
pub fn g_A_OWNED() -> Py { ensure_init(); A_OWNED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_OWNED(v: Py) { ensure_init(); if let Ok(mut w) = A_OWNED.write() { *w = v; } }
pub fn g_A_SUBJECT() -> Py { ensure_init(); A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_SUBJECT.write() { *w = v; } }
pub fn g_A_THE_PAST() -> Py { ensure_init(); A_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = A_THE_PAST.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A hard thing in the past is not a small account of the day.\n\nOne fresh process a line, on main:\n\n    my dad was an alcoholic        -> \"Sounds alright. Anything good in it?\"\n    i was homeless for a bit       -> \"Good to hear. What was the best bit?\"\n    my marriage ended two years ago -> \"Nice one. How did the rest of it go?\"\n    my dad walked out when i was ten -> \"Sounds alright. Anything good in it?\"\n\nReported by ample-fan on `#routing` (94443). The person has just said the worst thing in their\nlife, flat, as where they come from, and the bot claps.\n\nEvery guard there passes the line: the subject is the person's own (\"my dad\", \"my marriage\"), the\nverb is a past tense (\"was\", \"ended\", \"walked\"), the line is short, and the bad-day list is a list\nof one bad day - hospital, sacked, a row - rather than of a life event said once.\n\nSo this is a band of its own, not a reading inside the small-day pool: the words below name\nsomething that is still the person's history whatever else the line says, and a pool of the\ngreeting's must not be the answer to any of them.\n\nUnder `answers` (0) and every module with something of its own, because a card built for the line\nis a better answer than this: \"my dad was a heavy drinker\" keeps the calm, sober moment, and\n\"i had a crash on the motorway\" keeps the roadside card. Above the greeting module's pools\n(-1000), which is what these lines were reaching. A line that asks something (\"is my dad an\nalcoholic\") is not this module's: nothing is being told, so the hedge and the cards keep it.\n\nTwo things are fenced out of it, and both were found by a reviewer rather than by me:\n\n- **The hard thing has to be theirs.** A noun and a past tense say nothing about whose the prison\n  is, so \"the prison was built in 1800\" and \"the divorce rate went down last year\" were being\n  condoled with. The thing hangs on the person when a possessive or a family word stands in front\n  of it (\"my dad\", \"mum\", \"our marriage\") or when the person is the subject (\"i was homeless\",\n  \"i went to prison\", \"we were\"). Anything else is the world's, and this module passes on it.\n- **A loss is the grief module's.** \"my wife had a miscarriage last year\" and \"my baby was\n  stillborn\" had a warmer line already (\"I'm so sorry. That's a real loss\"), and this pool took it\n  away, so a death, a lost pregnancy and a widow keep their own pool.\n- **A blow is the safety modules'.** \"my dad used to hit me\" and \"he beat me up\" had a line that\n  offers something (\"Sorry to hear that. What would help most right now?\"), and a pool that only\n  asks about it is the weaker answer to the one line in a chat where somebody was hurt, so the\n  violence words are not in the list at all.\n\nThe words of the pool are said by `pick`, which keeps the last one it said out of the way, and\nnone of them opens on \"Noted\": that is the store's receipt word, the very thing this module\nexists to stop answering these lines. None of them says anything about the person's present\neither - \"it is still in your week\", \"it still comes up\", \"how much of it is around now\" were\nclaims about today that the line may have just contradicted, so every line is about the past and\nleaves the door open without a tense.\n\n- **A hard thing the line says is over is not this module's.** \"i went to prison years ago but i\n  have a job now\", \"i was in rehab in 2019 and im clean since\", \"my dad was an alcoholic but hes\n  been dry 20 years\" are one sentence about a hard past and a good present, and a pool that has\n  nothing to say about the ending must not answer the sentence that has one (`A_CLOSED`).\n\nA module that answers only the lines it is good at passes on everything else: `respond` returns\nNone on every other line and the bot answers it as it did before.\n");
    s_A_HARD_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:alcoholic|alcoholics|drinker|drinkers|addict(?:s|ed|ion)?|overdos(?:e|ed)|rehab|prison|jail|divorce[ds]?|custody|abusive|abuser|widow|widowed|widower|estrang\\w*|homeless|bankrupt\\w*|gambling|dementia|stillborn|stillbirth|miscarriage|miscarried|car crash|in a crash|had a crash|bad crash|fatal crash|marriage ended|marriage broke|relationship ended|walked out|left us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|had been|used to|grew up|growing up|as a kid|when i was|went to|walked out|ended|ago|for a bit|for a while|for years|back then|in the end|at the time|last (?:year|summer|winter|decade)|years back|after that)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEWS_ARRIVING(stdlib_call("re", "compile", &[py_str("\\b(?:getting|going through|just told|just said|just heard|asked for|wants|wanting|is leaving|has asked|separating|divorcing|this morning|tonight|right now|at the moment|today)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLOSING_WORD(py_str("(?:fine|okay|ok|good|better|clean|dry|sober|sorted|happy|healthy|stable|working|employed|a job|job|a house|a flat|a home|back on my feet|still friends|still together|paid off|all paid)"));
    s_A_CLOSED(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b"), &g_A_CLOSING_WORD()), &py_str("\\b[^.!?]{0,24}?\\b(?:now|since|these days|any ?more)\\b|\\b(?:now|since|these days)\\b[^.!?]{0,24}?\\b")), &g_A_CLOSING_WORD()), &py_str("\\b|\\b(?:dry|clean|sober|straight|off it)\\b[^.!?]{0,16}?\\b\\d+\\s*(?:years?|months?|decades?)\\b|\\b(?:never looked back|put it behind me|put that behind me|moved on from it|turned (?:it|things|my life) around|came out the other side|over it now|past it now|in a better place|still friends|no longer)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_OWNED(call_builtin("frozenset", &[mcall(&py_str("\n    my our his her their your mum mums mum's dad dads dad's mother father brother sister son\n    daughter husband wife partner ex nan gran granny grandma grandad grandpa uncle aunt cousin\n    parents family stepdad stepmum\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BETWEEN(call_builtin("frozenset", &[mcall(&py_str("\n    i we ive weve been was were am had have has got did do done went never once really badly\n    eventually finally long time terrible bad hard worst back then ago after before so that\n    this it there with from at to a an the in into through out on off for of and but also both\n    years year months weeks days month week twice again now still just\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("ive"), py_str("weve")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOSS(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|death|dead|passed away|funeral|buried|miscarriage|miscarried|stillborn|stillbirth|widow|widows|widowed|widower|bereaved|bereavement)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HARD_PAST_SAID(py_tuple(vec![py_str("That is a heavy thing to have lived with. Say more about it if you want to."), py_str("That is not a small thing to carry. I'm listening if you want to say more."), py_str("That one leaves a mark. Go on if you want to."), py_str("That is a lot to have been through. Tell me as much or as little as you like.")]));
    s_PRIORITY(neg(&Py::Int(980)));
}
pub fn _hangs_on_the_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut before: Py = Py::None;
    let mut i: Py = Py::None;
    let mut low: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the hard thing starting at `at` is the person's own rather than the world's.");
    before = slice_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&clean.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(6)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&before.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_OWNED(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("'s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_OWNED(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "replace", &[py_str("'"), py_str("")])); } py_list(__out) })() };
    seen = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk6963 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk6963[0].clone(); let mut word: Py = __unpk6963[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_A_SUBJECT(), &word.clone()))) {  __out.push(i.clone()); } } py_list(__out) })() };
    if py_truthy(&seen.clone()) {
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&low.clone(), &add(&geti(&seen.clone(), &neg(&Py::Int(1))), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_BETWEEN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_hard_thing(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The tidied line when it is a hard thing in the past told flat, else \"\".");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return py_str("");
    }
    understood = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("question"))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_HARD_PAST(), "search", &[clean.clone()])))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_HARD_PAST(), "finditer", &[clean.clone()])) { let mut found: Py = __item.clone(); __out.push(_hangs_on_the_person(&[clean.clone(), mcall(&found.clone(), "start", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_NEWS_ARRIVING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_THE_PAST(), "search", &[clean.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_LOSS(), "search", &[clean.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_CLOSED(), "search", &[clean.clone()])) {
    return py_str("");
    }
    return clean.clone();
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
    let _ = py_str("The claim is one reading of the whole line, so it is declared as well as banded.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[_the_hard_thing(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    let mut clean: Py = Py::None;
    let _ = py_str("What a friend says to a hard thing from the past, or None on any other line.");
    let __try1 = exc::catch(|| {
    clean = _the_hard_thing(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    return py_tuple(vec![Py::Bool(true), crate::gen::compose::pick(&[g_A_HARD_PAST_SAID(), memory.clone(), clean.clone()])]);
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
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.hardpast".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BETWEEN" => Some(g_A_BETWEEN()),
            "A_CLOSED" => Some(g_A_CLOSED()),
            "A_CLOSING_WORD" => Some(g_A_CLOSING_WORD()),
            "A_HARD_PAST" => Some(g_A_HARD_PAST()),
            "A_HARD_PAST_SAID" => Some(g_A_HARD_PAST_SAID()),
            "A_LOSS" => Some(g_A_LOSS()),
            "A_NEWS_ARRIVING" => Some(g_A_NEWS_ARRIVING()),
            "A_OWNED" => Some(g_A_OWNED()),
            "A_SUBJECT" => Some(g_A_SUBJECT()),
            "A_THE_PAST" => Some(g_A_THE_PAST()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_hangs_on_the_person" => Some(py_func("_hangs_on_the_person", |args: &[Py]| _hangs_on_the_person(args))),
            "_the_hard_thing" => Some(py_func("_the_hard_thing", |args: &[Py]| _the_hard_thing(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}

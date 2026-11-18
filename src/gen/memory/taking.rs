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


pub static A_CARD_IS_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FORM_OF_A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPLAINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEEDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FORMULAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_PAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAINKILLER_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREGNANT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_ASKS_A_LINE_MAKES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_SUBJECTS_A_CHAT_HAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_WORDS_A_SUBJECT_HAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_CARD_IS_LONG() -> Py { ensure_init(); A_CARD_IS_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CARD_IS_LONG(v: Py) { ensure_init(); if let Ok(mut w) = A_CARD_IS_LONG.write() { *w = v; } }
pub fn g_A_FORM_OF_A_VERB() -> Py { ensure_init(); A_FORM_OF_A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FORM_OF_A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_FORM_OF_A_VERB.write() { *w = v; } }
pub fn g_A_SENTENCE() -> Py { ensure_init(); A_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = A_SENTENCE.write() { *w = v; } }
pub fn g_COMPLAINT() -> Py { ensure_init(); COMPLAINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT(v: Py) { ensure_init(); if let Ok(mut w) = COMPLAINT.write() { *w = v; } }
pub fn g_FEEDING() -> Py { ensure_init(); FEEDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEEDING(v: Py) { ensure_init(); if let Ok(mut w) = FEEDING.write() { *w = v; } }
pub fn g_FILLER() -> Py { ensure_init(); FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FILLER.write() { *w = v; } }
pub fn g_FORMULAS() -> Py { ensure_init(); FORMULAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FORMULAS(v: Py) { ensure_init(); if let Ok(mut w) = FORMULAS.write() { *w = v; } }
pub fn g_IN_PAIN() -> Py { ensure_init(); IN_PAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_PAIN(v: Py) { ensure_init(); if let Ok(mut w) = IN_PAIN.write() { *w = v; } }
pub fn g_NOTHING_NAMED() -> Py { ensure_init(); NOTHING_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_NAMED.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_THEIRS() -> Py { ensure_init(); NOT_THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THEIRS.write() { *w = v; } }
pub fn g_PAINKILLER_ASKS() -> Py { ensure_init(); PAINKILLER_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAINKILLER_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = PAINKILLER_ASKS.write() { *w = v; } }
pub fn g_PREGNANT() -> Py { ensure_init(); PREGNANT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREGNANT(v: Py) { ensure_init(); if let Ok(mut w) = PREGNANT.write() { *w = v; } }
pub fn g_THEIRS() -> Py { ensure_init(); THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = THEIRS.write() { *w = v; } }
pub fn g_THE_ASKS_A_LINE_MAKES() -> Py { ensure_init(); THE_ASKS_A_LINE_MAKES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_ASKS_A_LINE_MAKES(v: Py) { ensure_init(); if let Ok(mut w) = THE_ASKS_A_LINE_MAKES.write() { *w = v; } }
pub fn g_THE_SUBJECTS_A_CHAT_HAS() -> Py { ensure_init(); THE_SUBJECTS_A_CHAT_HAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_SUBJECTS_A_CHAT_HAS(v: Py) { ensure_init(); if let Ok(mut w) = THE_SUBJECTS_A_CHAT_HAS.write() { *w = v; } }
pub fn g_THE_WORDS_A_SUBJECT_HAS() -> Py { ensure_init(); THE_WORDS_A_SUBJECT_HAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_WORDS_A_SUBJECT_HAS(v: Py) { ensure_init(); if let Ok(mut w) = THE_WORDS_A_SUBJECT_HAS.write() { *w = v; } }
pub fn g_TRIP() -> Py { ensure_init(); TRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP(v: Py) { ensure_init(); if let Ok(mut w) = TRIP.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A bare \"what can i take\" is a question about the thing the chat is on.\n\nThe four words name nothing to take and nothing to take it for, so every reader that needs a\nsubject reads them off the words alone. On main the person who had just said \"i keep getting\nheartburn\" got a packing question, and the person who had just said \"my back is killing me\" got\nthe duty-free allowance:\n\n    i keep getting heartburn / is it normal / what can i take\n      -> \"What are you going to, and how long for? Then I can say what to take and what to leave.\"\n    my back is killing me / what can i take\n      -> \"Coming into the UK you can bring 4 litres of spirits, 18 litres of wine...\"\n\nThe knowledge base has the answer to the question the person is actually asking. What it cannot\ndo is see the thing the chat is on: a line with no subject in it reaches no entry. So this module\nputs the subject in and asks, in the words the entries are keyed on (\"what can i take for\nheartburn\", \"what can i take for back pain\"), and answers only when\n\n- the line is the bare ask and names nothing itself - a thing of its own (\"what can i take to\n  spain\", \"can i take ibuprofen\") is left to the readers that can read it,\n- the chat is not a trip, where the packing question is the right answer,\n- the chat says a subject with the person in it (\"my back is killing me\", not \"the boiler is\n  making a noise\" - nothing is taken for a boiler), and\n- the answer the entry gives names that subject and is not the answer already on the screen.\n\nEverything else is passed on, and the reply the person is getting today stands.\n");
    s_FILLER(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|but|ok|okay|well|right|ah|oh|hmm|then|now|also|please)\\b[\\s,]+)*")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_NAMED(stdlib_call("re", "compile", &[py_str("^(?:(?:what|which)\\s+(?:(?:else|kind|sort)\\s+)*(?:can|could|should|may|shall|would)\\s+(?:i|we)\\s+(?:take|use|have)\\b|(?:is\\s+there\\s+)?(?:anything|something)\\s+(?:else\\s+)?(?:that\\s+)?(?:i|we)\\s+(?:can|could|should|may)\\s+(?:take|use|have)\\b|(?:what|which)\\s+is\\s+(?:safe|ok|okay|best)\\s+to\\s+(?:take|use)\\b)(?:\\s+for\\s+(?:it|this|that|them|those|the pain|the ache|the soreness|the hurt|any of it|any of that|any of those|the same))?[\\s!.,?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:flight|flights|flying|plane|airport|holiday|holidays|trip|travel|abroad|away|luggage|suitcase|bag|bags|hand luggage|carry on|packing|packed|passport|camping|campsite|hotel|ferry|cruise)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|id|i'd|my|me|we|we're|weve|our|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nache aching bad badly better bit constantly really still sore tender terrible awful worse worst\nkeep keeps keeping get gets getting got gotten feel feels feeling felt hurt hurts hurting kill\nkills killing make makes making made go goes going gone come comes coming give gives giving put\nwont wont sleep sleeping woke wake waking too very lot lots much many every always sometimes\nagain anymore either though bother bothering last night morning evening day days week weeks\nmonth months hour hours while since thing things stuff\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FORM_OF_A_VERB(stdlib_call("re", "compile", &[py_str("^(?:[a-z]{3,})(?:ing|ed)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_FORMULAS(py_tuple(vec![py_str("what can i take for %s"), py_str("what can i take for %s pain")]));
    s_PAINKILLER_ASKS(py_tuple(vec![py_str("what painkiller can i take")]));
    s_IN_PAIN(stdlib_call("re", "compile", &[py_str("\\b(?:pain|pains|ache|aches|aching|achy|hurts?|hurting|sore|throbbing|killing|agony|stinging|burning|tender)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEEDING(stdlib_call("re", "compile", &[py_str("\\b(?:breastfeeding|breast feeding|breastfeed|nursing|expressing|pumping)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREGNANT(stdlib_call("re", "compile", &[py_str("\\b(?:pregnan\\w*|expecting|trimester|weeks gone|weeks along|midwife)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SENTENCE(stdlib_call("re", "compile", &[py_str("[^.!?]+(?:[.!?]+|$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CARD_IS_LONG(Py::Int(120));
    s_THE_SUBJECTS_A_CHAT_HAS(Py::Int(2));
    s_THE_WORDS_A_SUBJECT_HAS(Py::Int(2));
    s_THE_ASKS_A_LINE_MAKES(Py::Int(3));
    s_COMPLAINT(Py::None);
    s_NOT_THEIRS(Py::None);
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _asks_what_to_take(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is the bare ask and names nothing to take or to take it for.");
    said = mcall(&mcall(&g_FILLER(), "sub", &[py_str(""), mcall(&_flat(&[text.clone()]), "lower", &[])]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOTHING_NAMED(), "match", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _complaint_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words that make a line a complaint, taken from the health knowledge where it has them.\n\n    A bare \"what can i take\" is a question about a body only where something is wrong with it:\n    \"im getting the train to manchester\" and \"im off to my brothers tonight\" are conversations\n    where the packing list is the right answer, and this is the test that keeps them.\n    ");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    let mut body: Py = crate::gen::knowledge::body::module_obj();
    let _ = mcall(&words.clone(), "update", &[gattr(&body.clone(), "HURT")]);
    let _ = mcall(&words.clone(), "update", &[gattr(&body.clone(), "_SORE_ANYWHERE")]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let _ = mcall(&words.clone(), "update", &[mcall(&py_str("\n        heartburn indigestion reflux nausea migraine headache toothache earache backache stomachache\n        cough coughing cold flu covid temperature fever cystitis thrush hayfever rash hives eczema\n        psoriasis dandruff ulcer blister sunburn burn sprain cramp cramping constipation diarrhoea\n        diarrea sick vomiting warts verruca sciatica gout arthritis ibs period swollen swelling\n        infected infection throat tooth teeth gum gums ear ears stomach tummy belly chest queasy\n        nauseous vomit vomiting dizzy faint fainted bleeding rash pregnancy pregnant\n        "), "split", &[])]);
    return call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _someone_else(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let _ = py_str("The words that make a complaint someone else's (\"my son\", \"his back\").");
    let __try1 = exc::catch(|| {
    let mut body: Py = crate::gen::knowledge::body::module_obj();
    phrases = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&body.clone(), "THIRD_PARTY")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return py_tuple(vec![Py::Bool(true), stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _a_complaint(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the person's own line says something is wrong with them.");
    if py_truthy(&Py::Bool(py_is(&g_COMPLAINT(), &Py::None))) {
    s_COMPLAINT(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(_complaint_words(&[])); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIRS(_someone_else(&[]));
    }
    said = _flat(&[subject.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g_NOT_THEIRS(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_THEIRS(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_COMPLAINT(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _subjects(args: &[Py]) -> Py {
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
    let mut other: Py = Py::None;
    let mut others: Py = Py::None;
    let mut out: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let _ = py_str("The things the chat has been on, newest first, as the person's own lines.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    seen = __unpk1[0].clone();
    out = __unpk1[1].clone();
    record = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&seen.clone(), "append", &[record.clone()]);
    }
    others = mcall(&memory.clone(), "get", &[py_str("topics")]);
    if py_truthy(&call_builtin("isinstance", &[others.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&seen.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[others.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut other: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[other.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(other.clone()); } } py_list(__out) })() }]);
    }
    let __iter2 = py_iter_or_raise(&seen.clone());
    for __item in __iter2 {
    other = __item.clone();
    said = _flat(&[mcall(&other.clone(), "get", &[py_str("text")])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&out.clone(), &said.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[said.clone()]);
    }
    }
    return slice_or_raise(&out.clone(), &Py::None, &g_THE_SUBJECTS_A_CHAT_HAS(), &Py::None);
    Py::None
}
pub fn _the_things_to_take_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the person's line that name something to take something for.");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::gen::memory::topics::keywords(&[subject.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("^[a-z]+$"), word.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_FORM_OF_A_VERB(), "match", &[word.clone()])) {
    word = if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")])) { slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None) } else { slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None) };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &neg(&Py::Int(1))), &geti(&word.clone(), &neg(&Py::Int(2))))) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())) } }) {
    continue;
    }
    }
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &word.clone()))) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    return slice_or_raise(&words.clone(), &Py::None, &g_THE_WORDS_A_SUBJECT_HAS(), &Py::None);
    Py::None
}
pub fn _a_trip_above(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the chat is about going somewhere, where the packing answer is the right one.");
    if py_truthy(&mcall(&g_TRIP(), "search", &[_flat(&[text.clone()])])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&g_TRIP(), "search", &[_flat(&[mcall(&turn.clone(), "get", &[py_str("user")])])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_answer(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let _ = py_str("What the knowledge base says to this line, asked on its own.");
    let mut answers: Py = crate::gen::modules::answers::module_obj();
    fresh = py_dict(vec![(py_str("turns"), py_list(vec![])), (py_str("aside"), Py::Bool(true))]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("others"), py_str("facts_log")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&memory.clone(), &key.clone()))) {
    let _ = py_setitem(&fresh.clone(), &key.clone(), geti(&memory.clone(), &key.clone()));
    }
    }
    let __try2 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = mcall(&answers.clone(), "respond", &[text.clone(), fresh.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    return py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    Py::None
}
pub fn _the_said_in_the_chat(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Every line the person has typed in this chat, as one string.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(_flat(&[mcall(&turn.clone(), "get", &[py_str("user")])])); } py_list(__out) })() }]);
    Py::None
}
pub fn _the_last_reply(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { _flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])]) } else { py_str("") };
    Py::None
}
pub fn _the_situation_this_chat_is_in(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The card, cut to the situation the person is actually in.\n\n    Only one cut is made, and only where the chat has said it: a card that answers for pregnancy\n    and for feeding is read by a pregnant person as the pregnancy part. Nothing is added and\n    nothing is reworded - a sentence about another situation is left out.\n    ");
    said = _the_said_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = mcall(&g_FEEDING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_PREGNANT(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FEEDING(), "search", &[reply.clone()]))) } } } }) {
    return reply.clone();
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_SENTENCE(), "findall", &[reply.clone()])) { let mut sentence: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FEEDING(), "search", &[sentence.clone()])))) {  __out.push(sentence.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&g_A_SENTENCE(), "findall", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return reply.clone();
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&py_str(" "), "join", &[kept.clone()]), "split", &[])]);
    Py::None
}
pub fn _the_answer_today(args: &[Py]) -> Py {
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
    let _ = py_str("The answer the knowledge base gives this line in this chat, as the line stands.");
    let mut answers: Py = crate::gen::modules::answers::module_obj();
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = mcall(&answers.clone(), "respond", &[text.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { py_dict(vec![]) }]); if py_truthy(&__b) { __b } else { py_str("") } }]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_same_answer(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut today: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let _ = py_str("True when the knowledge base answers the line as asked with these same words.\n\n    The bare ask reaches an answer of its own, and where that answer is the one this module found\n    there is nothing to add: the reply the person gets is the one they were going to get, and a\n    module that takes the turn to say the same thing only makes the comparison harder to read.\n    `today` is a one-line cache for the turn, so the knowledge base is asked about the line once.\n    The answer on its own is not the answer the person gets, though: the composer frames a repeat\n    and cuts it (\"From just now: ... take paracetamol or ibuprofen if you.\"), and where the last\n    turn was one of those short framed lines the whole card is the better reply and this module\n    is where it comes from. So a short line above the ask never counts as already answered.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&today.clone()))) {
    let _ = mcall(&today.clone(), "append", &[_the_answer_today(&[text.clone(), memory.clone()])]);
    }
    return { let __b = Py::Bool(py_eq(&_flat(&[geti(&today.clone(), &Py::Int(0))]), &_flat(&[reply.clone()]))); if !py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[_the_last_reply(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_A_CARD_IS_LONG()) } };
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the answer says the word back, a plural or a form of it counting.");
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    other = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other.clone(), &word.clone())); if py_truthy(&__b) { __b } else { crate::gen::memory::topics::alike(&[other.clone(), word.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_answer_above(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut opening: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this is the answer the person has just read, in its own opening words.\n\n    Only the turn before counts: an entry that said the same thing four turns back is saying it to\n    a different question now, and the chat has moved since.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&_flat(&[reply.clone()]), "lower", &[]), "split", &[]), &Py::None, &Py::Int(10), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&mcall(&_flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])]), "lower", &[]), &opening.clone()));
    Py::None
}
pub fn _the_subjects_to_answer(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The things in the chat it is worth answering a bare ask about, newest first.\n\n    A subject is worth answering when it is the person's own line, when it says something is\n    wrong with them, and when a word of it can be put into the question; everything else - a\n    house, a boiler, a train, a picnic, a question - is not something to take anything for.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_subjects(&[memory.clone()]));
    for __item in __iter1 {
    subject = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THEIRS(), "search", &[subject.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&subject.clone(), &py_str("?"))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_complaint(&[subject.clone()])))) {
    continue;
    }
    words = _the_things_to_take_for(&[subject.clone()]);
    if py_truthy(&words.clone()) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![subject.clone(), words.clone()])]);
    }
    }
    return out.clone();
    Py::None
}
pub fn the_ask_and_the_subject(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is the bare ask and the chat has something to answer it about.\n\n    Read by the arbiter as the module's claim (`TAKES_AS_IT_COMES`), so it asks no entry: the\n    reading of the chat is cheap, and the answer is not written until the turn is this module's.\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&_asks_what_to_take(&[text.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    if py_truthy(&_a_trip_above(&[memory.clone(), text.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[_the_subjects_to_answer(&[memory.clone(), text.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    let mut already: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut formula: Py = Py::None;
    let mut in_pain: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut today: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answer to a bare \"what can i take\", about the thing the chat is on, or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&_asks_what_to_take(&[text.clone()])))) {
    return py_str("");
    }
    if py_truthy(&_a_trip_above(&[memory.clone(), text.clone()])) {
    return py_str("");
    }
    asked = Py::Int(0);
    today = py_list(vec![]);
    already = py_str("");
    in_pain = py_str("");
    let __iter1 = py_iter_or_raise(&_subjects(&[memory.clone()]));
    for __item in __iter1 {
    subject = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THEIRS(), "search", &[subject.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&subject.clone(), &py_str("?"))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_complaint(&[subject.clone()])))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&_the_things_to_take_for(&[subject.clone()]));
    for __item in __iter2 {
    word = __item.clone();
    let __iter3 = py_iter_or_raise(&g_FORMULAS());
    for __item in __iter3 {
    formula = __item.clone();
    if py_truthy(&ge(&asked.clone(), &g_THE_ASKS_A_LINE_MAKES())) {
    return py_str("");
    }
    let __aug4 = add(&asked.clone(), &Py::Int(1));
    asked = __aug4;
    reply = _the_answer(&[modulo(&formula.clone(), &word.clone()), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[reply.clone(), word.clone()]))) } }) {
    continue;
    }
    if py_truthy(&_the_answer_above(&[reply.clone(), memory.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&already.clone()))) {
    already = reply.clone();
    in_pain = subject.clone();
    }
    continue;
    }
    if py_truthy(&_the_same_answer(&[reply.clone(), text.clone(), memory.clone(), today.clone()])) {
    continue;
    }
    return _the_situation_this_chat_is_in(&[reply.clone(), memory.clone()]);
    }
    }
    }
    if py_truthy(&{ let __b = in_pain.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_IN_PAIN(), "search", &[in_pain.clone()]) } }) {
    reply = _the_answer(&[geti(&g_PAINKILLER_ASKS(), &Py::Int(0)), memory.clone()]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_the_answer_above(&[reply.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_same_answer(&[reply.clone(), text.clone(), memory.clone(), today.clone()]))) } } } }) {
    return reply.clone();
    }
    }
    if py_truthy(&{ let __b = already.clone(); if !py_truthy(&__b) { __b } else { _the_same_answer(&[already.clone(), text.clone(), memory.clone(), today.clone()]) } }) {
    return py_str("");
    }
    return already.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.taking".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CARD_IS_LONG" => Some(g_A_CARD_IS_LONG()),
            "A_FORM_OF_A_VERB" => Some(g_A_FORM_OF_A_VERB()),
            "A_SENTENCE" => Some(g_A_SENTENCE()),
            "COMPLAINT" => Some(g_COMPLAINT()),
            "FEEDING" => Some(g_FEEDING()),
            "FILLER" => Some(g_FILLER()),
            "FORMULAS" => Some(g_FORMULAS()),
            "IN_PAIN" => Some(g_IN_PAIN()),
            "NOTHING_NAMED" => Some(g_NOTHING_NAMED()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_THEIRS" => Some(g_NOT_THEIRS()),
            "PAINKILLER_ASKS" => Some(g_PAINKILLER_ASKS()),
            "PREGNANT" => Some(g_PREGNANT()),
            "THEIRS" => Some(g_THEIRS()),
            "THE_ASKS_A_LINE_MAKES" => Some(g_THE_ASKS_A_LINE_MAKES()),
            "THE_SUBJECTS_A_CHAT_HAS" => Some(g_THE_SUBJECTS_A_CHAT_HAS()),
            "THE_WORDS_A_SUBJECT_HAS" => Some(g_THE_WORDS_A_SUBJECT_HAS()),
            "TRIP" => Some(g_TRIP()),
            "_a_complaint" => Some(py_func("_a_complaint", |args: &[Py]| _a_complaint(args))),
            "_a_trip_above" => Some(py_func("_a_trip_above", |args: &[Py]| _a_trip_above(args))),
            "_asks_what_to_take" => Some(py_func("_asks_what_to_take", |args: &[Py]| _asks_what_to_take(args))),
            "_complaint_words" => Some(py_func("_complaint_words", |args: &[Py]| _complaint_words(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_someone_else" => Some(py_func("_someone_else", |args: &[Py]| _someone_else(args))),
            "_subjects" => Some(py_func("_subjects", |args: &[Py]| _subjects(args))),
            "_the_answer" => Some(py_func("_the_answer", |args: &[Py]| _the_answer(args))),
            "_the_answer_above" => Some(py_func("_the_answer_above", |args: &[Py]| _the_answer_above(args))),
            "_the_answer_today" => Some(py_func("_the_answer_today", |args: &[Py]| _the_answer_today(args))),
            "_the_last_reply" => Some(py_func("_the_last_reply", |args: &[Py]| _the_last_reply(args))),
            "_the_said_in_the_chat" => Some(py_func("_the_said_in_the_chat", |args: &[Py]| _the_said_in_the_chat(args))),
            "_the_same_answer" => Some(py_func("_the_same_answer", |args: &[Py]| _the_same_answer(args))),
            "_the_situation_this_chat_is_in" => Some(py_func("_the_situation_this_chat_is_in", |args: &[Py]| _the_situation_this_chat_is_in(args))),
            "_the_subjects_to_answer" => Some(py_func("_the_subjects_to_answer", |args: &[Py]| _the_subjects_to_answer(args))),
            "_the_things_to_take_for" => Some(py_func("_the_things_to_take_for", |args: &[Py]| _the_things_to_take_for(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_ask_and_the_subject" => Some(py_func("the_ask_and_the_subject", |args: &[Py]| the_ask_and_the_subject(args))),
            _ => None,
        }),
        set: None,
    }))
}

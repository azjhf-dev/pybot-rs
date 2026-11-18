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


thread_local! { static G_A_REPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_MATTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVES_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_DIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_OLD_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OLDEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONE_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOUNGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.matters: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_REPORT() -> Py { ensure_init(); G_A_REPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPORT(v: Py) { ensure_init(); G_A_REPORT.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_MATTER() -> Py { ensure_init(); G_CARE_MATTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_MATTER(v: Py) { ensure_init(); G_CARE_MATTER.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_VERB() -> Py { ensure_init(); G_FACT_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_VERB(v: Py) { ensure_init(); G_FACT_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_FINE_REPLY() -> Py { ensure_init(); G_FINE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINE_REPLY(v: Py) { ensure_init(); G_FINE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVES_ALONE() -> Py { ensure_init(); G_LIVES_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVES_ALONE(v: Py) { ensure_init(); G_LIVES_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_MINE() -> Py { ensure_init(); G_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); G_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THEIR_OWN() -> Py { ensure_init(); G_ON_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THEIR_OWN(v: Py) { ensure_init(); G_ON_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SO() -> Py { ensure_init(); G_SO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SO(v: Py) { ensure_init(); G_SO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_DIET() -> Py { ensure_init(); G_THEIR_DIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_DIET(v: Py) { ensure_init(); G_THEIR_DIET.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_WORD() -> Py { ensure_init(); G_THEIR_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_WORD(v: Py) { ensure_init(); G_THEIR_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CORRECTION() -> Py { ensure_init(); G__A_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CORRECTION(v: Py) { ensure_init(); G__A_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_OLD_ONE() -> Py { ensure_init(); G__NOT_THE_OLD_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_OLD_ONE(v: Py) { ensure_init(); G__NOT_THE_OLD_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__OLDEST() -> Py { ensure_init(); G__OLDEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OLDEST(v: Py) { ensure_init(); G__OLDEST.with(|c| *c.borrow_mut() = v); }
pub fn g__ONE_AGE() -> Py { ensure_init(); G__ONE_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONE_AGE(v: Py) { ensure_init(); G__ONE_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__YOUNGEST() -> Py { ensure_init(); G__YOUNGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOUNGEST(v: Py) { ensure_init(); G__YOUNGEST.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that says a fact about the person the answer was about, written out with them in it.\n\n    my mum is 78 and keeps falling\n      -> Falls are usually several small things adding up ... Ask the GP for a falls review ...\n    she lives alone\n      -> Got it - she lives alone.                      (base: the receipt STYLE.md rules out)\n\n\"she lives alone\" is not news about the person and not a new question: it is the one fact that\nchanges which part of that answer matters. On her own, the strength and balance class is not the\nfirst thing - a free care needs assessment, a pendant alarm, a key safe and a neighbour with a key\nare - and the knowledge base already has that answer under \"my mum lives alone\".\n\nWhat it does not have is \"she lives alone\": the store holds the person as \"mum\", so a line that\nsays \"she\" names nobody the entries are keyed on, the lookup finds nothing, and the turn falls to\nthe store, which says the line back. This module writes the line out with the person in it and\nhands it on, so the answer is the answer to the fact and not a receipt.\n\nOnly the narrow shape is taken, because a pronoun is not always the person on screen:\n  * the line opens with \"she\" or \"he\" and says nothing else of its own - no question mark, a\n    handful of words, and the rest of the line starts with a verb a fact about a person uses\n    (\"lives\", \"is\", \"has\", \"keeps\", \"falls\", \"wont\", \"refuses\", \"uses\");\n  * the store holds exactly one person (`memory[\"others\"]`), so \"she\" has one person to be;\n  * that person's sex agrees with the pronoun, where the store knows it.\nEverything else - \"my sister lives alone\", which the entries already reach, two people in the\nstore, a question about her (\"should she move?\"), a report (\"she said thanks\") - is passed on\nunchanged, and the turn is answered exactly as it was before this module existed.\n");
    s_MINE(py_str("my "));
    s_SO(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|but|also|oh|well|so|see)?\\s*,?\\s*(?P<who>she|he)(?P<is>'s|s)?\\s+(?P<rest>[^?]{2,60})\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_VERB(call_builtin("frozenset", &[mcall(&py_str("\nlive lives lived living is are was were has have had keeps keep kept falls fall fell falling\nrefuses refuse refused wont won't cant can't cannot doesnt doesn't never always still only on\ntakes take uses use needs need gets get goes go wants want sits sit walks walk forgets forget\nwears wear uses stopped stops started starts keeps manages manages\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT(stdlib_call("re", "compile", &[py_str("^\\s*(?:says?|said|told|thinks?|wants?|asked|keeps saying|thinks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_DIET(stdlib_call("re", "compile", &[py_str("^(?:(?:does\\s*nt|does\\s+not|do\\s*nt|don'?t|won'?t|wont|cant|can'?t|never|only)\\s+(?:eat|eats|drink|drinks|touch|touches|stand|stands|like|likes|want|wants)\\b|(?:is\\s+)?(?:hates?|likes?|loves?|avoids?|allergic|refuses?)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(8));
    s_ON_THEIR_OWN(stdlib_call("re", "compile", &[py_str("^liv(?:es?|ing)\\s+(?:on\\s+(?:her|his|their|its)\\s+own|by\\s+(?:her|him|them)sel(?:f|ves)|alone)\\b\\s*|^(?:is\\s+)?(?:on\\s+(?:her|his|their|its)\\s+own|by\\s+(?:her|him|them)sel(?:f|ves)|alone)(?:\\s+at\\s+home)?\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVES_ALONE(py_str("lives alone"));
    s_THEIR_WORD(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|but|so|also|oh|well|see)?\\s*,?\\s*(?P<who>she|he|they|mum|mom|dad|nan|nana|gran|granny|grandma|grandad)\\b(?:'s|s|\\s+is|\\s+are)?\\s+(?:says?|said|tells? me|told me|insists?|insisted|reckons?|reckoned|claims?|claimed|keeps saying|thinks?)\\s+(?:(?:that\\s+)?(?:she|he|they)(?:'s|s|\\s+is|\\s+are)?\\s+)?(?P<what>fine|alright|all right|ok|okay|grand|managing|coping|all good|not too bad)\\b(?:\\s*,?\\s*(?:thank you|thanks|i think|i reckon|really|honestly|though|anyway|actually)){0,2}\\s*[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_MATTER(stdlib_call("re", "compile", &[py_str("\\b(?:fall|falls|fallen|fell|falling|carer|carers|care|dementia|alzheim\\w*|memory|forget\\w*|confused|hospital|stroke|doctor|gp|surgery|social services|needs assessment|pendant|alarm|key safe|blood thinner\\w*|warfarin|tablets|medication)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FINE_REPLY(py_tuple(vec![py_str("That usually means your {who} doesn't want a fuss, not that nothing is wrong. Don't argue the point - ask for one small thing instead, the GP appointment with you in the room, and let the doctor carry the worry."), py_str("They say that to keep the peace, and it does not mean it has stopped. Keep a note of when it happens and ring your {who}'s GP yourself: they can act on what you tell them, and nobody has to ask for it."), py_str("How your {who} feels is worth believing; the risk is not something to take their word for. Make the whole ask one thing you do together - the free falls review, or a walk to the end of the road - so there is nothing to admit to.")]));
    s__ONE_AGE(stdlib_call("re", "compile", &[py_str("\\b(?P<age>\\d{1,3})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_OLD_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:not|instead of|rather than|no wait,? not)\\s+(?P<old>\\d{1,3})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OLDEST(stdlib_call("re", "compile", &[py_str("\\b(?:oldest|eldest|older|bigger|elder|big one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__YOUNGEST(stdlib_call("re", "compile", &[py_str("\\b(?:youngest|younger|littlest|little|smallest|baby|wee one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:actually|no|nope|sorry|wait|i mean|i meant|make it|makes it|not|instead)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_person(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut others: Py = Py::None;
    let mut record: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut sex: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The one person the chat knows: (relation, sex) or None. Two people is nobody in particular.");
    others = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("others")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[others.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    seen = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&others.clone());
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&seen.clone()) { let mut one: Py = __item.clone(); __out.push(geti(&one.clone(), &Py::Int(0))); } py_list(__out) })() }, &who.clone())) } }) {
    let _ = mcall(&seen.clone(), "append", &[py_tuple(vec![who.clone(), record.clone()])]);
    }
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&geti(&seen.clone(), &Py::Int(0)), 2, None);
    who = __unpk2[0].clone();
    record = __unpk2[1].clone();
    sex = py_str("");
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.people", "HIM"), &who.clone()))) {
    sex = py_str("him");
    } else {
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.people", "HER"), &who.clone()))) {
    sex = py_str("her");
    }
    }
    return py_tuple(vec![who.clone(), record.clone(), sex.clone()]);
    Py::None
}
pub fn about_them(args: &[Py]) -> Py {
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
    let mut _record: Py = Py::None;
    let mut alone: Py = Py::None;
    let mut found: Py = Py::None;
    let mut person: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sex: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The line written with the person in it: \"she lives alone\" -> \"my mum lives alone\".\n\n    Empty when the line is not the narrow shape above, so the caller answers exactly as before.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    found = mcall(&g_SO(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    rest = mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "split", &[])]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS())) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_REPORT(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_DIET(), "match", &[rest.clone()]) } }) {
    return py_str("");
    }
    alone = mcall(&g_ON_THEIR_OWN(), "match", &[rest.clone()]);
    if py_truthy(&alone.clone()) {
    rest = mcall(&op_add(&op_add(&g_LIVES_ALONE(), &py_str(" ")), &slice_or_raise(&rest.clone(), &mcall(&alone.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    } else {
    if py_truthy(&Py::Bool(!py_contains(&g_FACT_VERB(), &mcall(&mcall(&geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)), "strip", &[py_str(",.")]), "lower", &[])))) {
    return py_str("");
    }
    }
    person = _the_person(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&person.clone(), &Py::None))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&person.clone(), 3, None);
    who = __unpk1[0].clone();
    _record = __unpk1[1].clone();
    sex = __unpk1[2].clone();
    said = mcall(&mcall(&found.clone(), "group", &[py_str("who")]), "lower", &[]);
    if py_truthy(&{ let __b = sex.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_eq(&said.clone(), &py_str("she"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&sex.clone(), &py_str("her"))) } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&said.clone(), &py_str("he"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&sex.clone(), &py_str("him"))) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&mcall(&rest.clone(), "lower", &[]), "split", &[]), &who.clone()))) {
    return py_str("");
    }
    if py_truthy(&alone.clone()) {
    return op_add(&op_add(&op_add(&g_MINE(), &who.clone()), &py_str(" ")), &rest.clone());
    }
    if py_truthy(&mcall(&found.clone(), "group", &[py_str("is")])) {
    return op_add(&op_add(&op_add(&g_MINE(), &who.clone()), &py_str(" is ")), &rest.clone());
    }
    return op_add(&op_add(&op_add(&g_MINE(), &who.clone()), &py_str(" ")), &rest.clone());
    Py::None
}
pub fn _a_worry_in_the_chat(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when the chat has named a care worry about the person, and not just a mood.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(5)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { turn.clone() } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[turn.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    }
    return call_builtin("bool", &[mcall(&g_CARE_MATTER(), "search", &[mcall(&py_str(" "), "join", &[said.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn what_they_say(args: &[Py]) -> Py {
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
    let mut _record: Py = Py::None;
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sex: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The reply for \"she says she's fine\" about the person on screen, or \"\" to pass.\n\n    Asked only when the store holds one person, the pronoun is that person's, and the chat has a\n    care worry in it: anything else is somebody's ordinary remark and keeps its own turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    found = mcall(&g_THEIR_WORD(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    person = _the_person(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&person.clone(), &Py::None))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&person.clone(), 3, None);
    who = __unpk1[0].clone();
    _record = __unpk1[1].clone();
    sex = __unpk1[2].clone();
    said = mcall(&mcall(&found.clone(), "group", &[py_str("who")]), "lower", &[]);
    if py_truthy(&{ let __b = sex.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("her")]), &said.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&sex.clone(), &py_str("her"))) } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&said.clone(), &py_str("he"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&sex.clone(), &py_str("him"))) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_worry_in_the_chat(&[memory.clone()])))) {
    return py_str("");
    }
    pool = call_builtin("list", &[g_FINE_REPLY()]).unwrap_or_else(|e| exc::raise(e));
    at = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("fine_said")]) } else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&at.clone(), &Py::Int(0)) } }) {
    at = Py::Int(0);
    }
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&memory.clone(), &py_str("fine_said"), op_add(&at.clone(), &Py::Int(1)));
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&pool.clone(), &op_modulo(&at.clone(), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    Py::None
}
pub fn _the_group_ages(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The single group of ages the store holds, or (None, []).");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    ages = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("ages")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut age: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(age.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![entry.clone(), ages.clone()])]);
    }
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return py_tuple(vec![Py::None, py_list(vec![])]);
    }
    return geti(&found.clone(), &Py::Int(0));
    Py::None
}
pub fn _say_the_ages(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ages: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ages") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ages"))) };
    let mut age: Py = Py::None;
    ages = call_builtin("sorted", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_modulo(&py_str("%d and %d"), &py_tuple(vec![geti(&ages.clone(), &Py::Int(0)), geti(&ages.clone(), &Py::Int(1))]));
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&ages.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut age: Py = __item.clone(); __out.push(call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), &py_str(" and ")), &call_builtin("str", &[geti(&ages.clone(), &op_neg(&Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn a_corrected_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut correcting: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut low: Py = Py::None;
    let mut m: Py = Py::None;
    let mut new: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut replaced: Py = Py::None;
    let mut was: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("The reply for a corrected age in a group of children, or \"\" to pass.\n\n    Only the corrections that say which child they are about are applied; one that could be either\n    asks which, because guessing puts the wrong age on a child and the chat then answers every\n    later question with it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&_the_group_ages(&[memory.clone()]), 2, None);
    entry = __unpk1[0].clone();
    ages = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&entry.clone(), &Py::None))) {
    return py_str("");
    }
    numbers = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__ONE_AGE(), "finditer", &[low.clone()])) { let mut m: Py = __item.clone(); __out.push(call_builtin("int", &[mcall(&m.clone(), "group", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    numbers = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&numbers.clone()) { let mut age: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(120)) } else { Py::Bool(false) } }) {  __out.push(age.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&numbers.clone()))) {
    return py_str("");
    }
    correcting = call_builtin("bool", &[mcall(&g__A_CORRECTION(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    replaced = mcall(&g__NOT_THE_OLD_ONE(), "search", &[low.clone()]);
    if py_truthy(&replaced.clone()) {
    was = call_builtin("int", &[mcall(&replaced.clone(), "group", &[py_str("old")])]).unwrap_or_else(|e| exc::raise(e));
    new = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&numbers.clone()) { let mut age: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&age.clone(), &was.clone()))) {  __out.push(age.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&ages.clone(), &was.clone())); if !py_truthy(&__b) { __b } else { new.clone() } }) {
    let _ = py_setitem(&ages.clone(), &mcall(&ages.clone(), "index", &[was.clone()]), geti(&new.clone(), &Py::Int(0)));
    let _ = py_setitem(&entry.clone(), &py_str("ages"), call_builtin("sorted", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return op_modulo(&py_str("Right — %s, then."), &_say_the_ages(&[geti(&entry.clone(), &py_str("ages"))]));
    }
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__OLDEST(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__YOUNGEST(), "search", &[low.clone()]))) } }) {
    let _ = py_setitem(&ages.clone(), &mcall(&ages.clone(), "index", &[call_builtin("max", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e))]), geti(&numbers.clone(), &Py::Int(0)));
    let _ = py_setitem(&entry.clone(), &py_str("ages"), call_builtin("sorted", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return op_modulo(&py_str("Right — %s, then."), &_say_the_ages(&[geti(&entry.clone(), &py_str("ages"))]));
    }
    if py_truthy(&{ let __b = mcall(&g__YOUNGEST(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__OLDEST(), "search", &[low.clone()]))) } }) {
    let _ = py_setitem(&ages.clone(), &mcall(&ages.clone(), "index", &[call_builtin("min", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e))]), geti(&numbers.clone(), &Py::Int(0)));
    let _ = py_setitem(&entry.clone(), &py_str("ages"), call_builtin("sorted", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return op_modulo(&py_str("Right — %s, then."), &_say_the_ages(&[geti(&entry.clone(), &py_str("ages"))]));
    }
    if py_truthy(&{ let __b = correcting.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&ages.clone(), &geti(&numbers.clone(), &Py::Int(0)))) } }) {
    which = mcall(&py_str(" or the "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("sorted", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut age: Py = __item.clone(); __out.push(call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    return op_modulo(&py_str("Which one is %d — the %s?"), &py_tuple(vec![geti(&numbers.clone(), &Py::Int(0)), which.clone()]));
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.matters".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_REPORT" => Some(g_A_REPORT()),
            "CARE_MATTER" => Some(g_CARE_MATTER()),
            "FACT_VERB" => Some(g_FACT_VERB()),
            "FINE_REPLY" => Some(g_FINE_REPLY()),
            "LIVES_ALONE" => Some(g_LIVES_ALONE()),
            "MINE" => Some(g_MINE()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "ON_THEIR_OWN" => Some(g_ON_THEIR_OWN()),
            "SO" => Some(g_SO()),
            "THEIR_DIET" => Some(g_THEIR_DIET()),
            "THEIR_WORD" => Some(g_THEIR_WORD()),
            "_A_CORRECTION" => Some(g__A_CORRECTION()),
            "_NOT_THE_OLD_ONE" => Some(g__NOT_THE_OLD_ONE()),
            "_OLDEST" => Some(g__OLDEST()),
            "_ONE_AGE" => Some(g__ONE_AGE()),
            "_YOUNGEST" => Some(g__YOUNGEST()),
            "_a_worry_in_the_chat" => Some(py_func("_a_worry_in_the_chat", |args: &[Py]| _a_worry_in_the_chat(args))),
            "_say_the_ages" => Some(py_func("_say_the_ages", |args: &[Py]| _say_the_ages(args))),
            "_the_group_ages" => Some(py_func("_the_group_ages", |args: &[Py]| _the_group_ages(args))),
            "_the_person" => Some(py_func("_the_person", |args: &[Py]| _the_person(args))),
            "a_corrected_age" => Some(py_func("a_corrected_age", |args: &[Py]| a_corrected_age(args))),
            "about_them" => Some(py_func("about_them", |args: &[Py]| about_them(args))),
            "what_they_say" => Some(py_func("what_they_say", |args: &[Py]| what_they_say(args))),
            "people" => Some(pyrt::xrefs::xmodule("memory.people")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_REPORT" => s_A_REPORT(v),
            "CARE_MATTER" => s_CARE_MATTER(v),
            "FACT_VERB" => s_FACT_VERB(v),
            "FINE_REPLY" => s_FINE_REPLY(v),
            "LIVES_ALONE" => s_LIVES_ALONE(v),
            "MINE" => s_MINE(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "ON_THEIR_OWN" => s_ON_THEIR_OWN(v),
            "SO" => s_SO(v),
            "THEIR_DIET" => s_THEIR_DIET(v),
            "THEIR_WORD" => s_THEIR_WORD(v),
            "_A_CORRECTION" => s__A_CORRECTION(v),
            "_NOT_THE_OLD_ONE" => s__NOT_THE_OLD_ONE(v),
            "_OLDEST" => s__OLDEST(v),
            "_ONE_AGE" => s__ONE_AGE(v),
            "_YOUNGEST" => s__YOUNGEST(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.matters", module_obj);
    pyrt::xrefs::reg_fn("memory.matters", "_the_person", py_func("_the_person", |args: &[Py]| _the_person(args)));
    pyrt::xrefs::reg_fn("memory.matters", "about_them", py_func("about_them", |args: &[Py]| about_them(args)));
    pyrt::xrefs::reg_fn("memory.matters", "_a_worry_in_the_chat", py_func("_a_worry_in_the_chat", |args: &[Py]| _a_worry_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.matters", "what_they_say", py_func("what_they_say", |args: &[Py]| what_they_say(args)));
    pyrt::xrefs::reg_fn("memory.matters", "_the_group_ages", py_func("_the_group_ages", |args: &[Py]| _the_group_ages(args)));
    pyrt::xrefs::reg_fn("memory.matters", "_say_the_ages", py_func("_say_the_ages", |args: &[Py]| _say_the_ages(args)));
    pyrt::xrefs::reg_fn("memory.matters", "a_corrected_age", py_func("a_corrected_age", |args: &[Py]| a_corrected_age(args)));
    pyrt::xrefs::reg_access("memory.matters", "A_REPORT", g_A_REPORT, s_A_REPORT);
    pyrt::xrefs::reg_access("memory.matters", "CARE_MATTER", g_CARE_MATTER, s_CARE_MATTER);
    pyrt::xrefs::reg_access("memory.matters", "FACT_VERB", g_FACT_VERB, s_FACT_VERB);
    pyrt::xrefs::reg_access("memory.matters", "FINE_REPLY", g_FINE_REPLY, s_FINE_REPLY);
    pyrt::xrefs::reg_access("memory.matters", "LIVES_ALONE", g_LIVES_ALONE, s_LIVES_ALONE);
    pyrt::xrefs::reg_access("memory.matters", "MINE", g_MINE, s_MINE);
    pyrt::xrefs::reg_access("memory.matters", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.matters", "ON_THEIR_OWN", g_ON_THEIR_OWN, s_ON_THEIR_OWN);
    pyrt::xrefs::reg_access("memory.matters", "SO", g_SO, s_SO);
    pyrt::xrefs::reg_access("memory.matters", "THEIR_DIET", g_THEIR_DIET, s_THEIR_DIET);
    pyrt::xrefs::reg_access("memory.matters", "THEIR_WORD", g_THEIR_WORD, s_THEIR_WORD);
    pyrt::xrefs::reg_access("memory.matters", "_A_CORRECTION", g__A_CORRECTION, s__A_CORRECTION);
    pyrt::xrefs::reg_access("memory.matters", "_NOT_THE_OLD_ONE", g__NOT_THE_OLD_ONE, s__NOT_THE_OLD_ONE);
    pyrt::xrefs::reg_access("memory.matters", "_OLDEST", g__OLDEST, s__OLDEST);
    pyrt::xrefs::reg_access("memory.matters", "_ONE_AGE", g__ONE_AGE, s__ONE_AGE);
    pyrt::xrefs::reg_access("memory.matters", "_YOUNGEST", g__YOUNGEST, s__YOUNGEST);
}

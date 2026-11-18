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


thread_local! { static G_ANIMALS_IN_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COORDINATORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOWUP_STARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAVES_SOMETHING_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMES_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PERSONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACEHOLDER_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE_PRONOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS_FOR_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_THE_APOSTROPHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.references: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANIMALS_IN_HAND() -> Py { ensure_init(); G_ANIMALS_IN_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS_IN_HAND(v: Py) { ensure_init(); G_ANIMALS_IN_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_NAMES() -> Py { ensure_init(); G_ANIMAL_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_NAMES(v: Py) { ensure_init(); G_ANIMAL_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPENERS() -> Py { ensure_init(); G_ASK_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENERS(v: Py) { ensure_init(); G_ASK_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_STOPS() -> Py { ensure_init(); G_CLAUSE_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_STOPS(v: Py) { ensure_init(); G_CLAUSE_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_SUBJECTS() -> Py { ensure_init(); G_CLAUSE_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_SUBJECTS(v: Py) { ensure_init(); G_CLAUSE_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_COORDINATORS() -> Py { ensure_init(); G_COORDINATORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COORDINATORS(v: Py) { ensure_init(); G_COORDINATORS.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOWUP_STARTS() -> Py { ensure_init(); G_FOLLOWUP_STARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOWUP_STARTS(v: Py) { ensure_init(); G_FOLLOWUP_STARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAVES_SOMETHING_OUT() -> Py { ensure_init(); G_LEAVES_SOMETHING_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAVES_SOMETHING_OUT(v: Py) { ensure_init(); G_LEAVES_SOMETHING_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_MINE() -> Py { ensure_init(); G_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); G_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMES_A_THING() -> Py { ensure_init(); G_NAMES_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMES_A_THING(v: Py) { ensure_init(); G_NAMES_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_PERSON() -> Py { ensure_init(); G_ONE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_PERSON(v: Py) { ensure_init(); G_ONE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PERSONS() -> Py { ensure_init(); G_OWN_PERSONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PERSONS(v: Py) { ensure_init(); G_OWN_PERSONS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SUBJECT() -> Py { ensure_init(); G_OWN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SUBJECT(v: Py) { ensure_init(); G_OWN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_THINGS() -> Py { ensure_init(); G_OWN_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_THINGS(v: Py) { ensure_init(); G_OWN_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACEHOLDER_IT() -> Py { ensure_init(); G_PLACEHOLDER_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACEHOLDER_IT(v: Py) { ensure_init(); G_PLACEHOLDER_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_BACK() -> Py { ensure_init(); G_POINTING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_BACK(v: Py) { ensure_init(); G_POINTING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE_PRONOUNS() -> Py { ensure_init(); G_POSSESSIVE_PRONOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE_PRONOUNS(v: Py) { ensure_init(); G_POSSESSIVE_PRONOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUNS() -> Py { ensure_init(); G_PRONOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUNS(v: Py) { ensure_init(); G_PRONOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_WORDS() -> Py { ensure_init(); G_PRONOUN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_WORDS(v: Py) { ensure_init(); G_PRONOUN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS_FOR_SOMETHING() -> Py { ensure_init(); G_STANDS_FOR_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS_FOR_SOMETHING(v: Py) { ensure_init(); G_STANDS_FOR_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON() -> Py { ensure_init(); G_THIRD_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON(v: Py) { ensure_init(); G_THIRD_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_RE() -> Py { ensure_init(); G_TIME_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_RE(v: Py) { ensure_init(); G_TIME_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_WORDS() -> Py { ensure_init(); G_WHERE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_WORDS(v: Py) { ensure_init(); G_WHERE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_THE_APOSTROPHE() -> Py { ensure_init(); G_WITHOUT_THE_APOSTROPHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_THE_APOSTROPHE(v: Py) { ensure_init(); G_WITHOUT_THE_APOSTROPHE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What \"it\", \"that\", \"they\" and \"there\" point at, and whether this line follows the last one.\n\nPeople answer in fragments and lean on the last turn: \"How long will it take?\", \"And if I\nstuff it?\", \"Anything cheaper there?\". This module reads the previous user line and says what\na pronoun most likely means, so a module can answer about the right thing.\n\n`resolve()` never guesses when there is nothing to point at: on the first turn every referent\nis None and `followup` is False.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.PRONOUNS"); }
    s_PRONOUNS(py_dict(vec![(py_str("it"), py_tuple(vec![py_str("it"), py_str("its"), py_str("itself")])), (py_str("that"), py_tuple(vec![py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("one"), py_str("ones")])), (py_str("they"), py_tuple(vec![py_str("they"), py_str("them"), py_str("their"), py_str("theirs")])), (py_str("there"), py_tuple(vec![py_str("there")])), (py_str("he"), py_tuple(vec![py_str("he"), py_str("him"), py_str("his"), py_str("himself")])), (py_str("she"), py_tuple(vec![py_str("she"), py_str("her"), py_str("hers"), py_str("herself")]))]));
    fn __data1() -> Py { py_tuple(vec![py_str("and"), py_str("also"), py_str("but"), py_str("so"), py_str("what about"), py_str("how about"), py_str("what if"), py_str("and what"), py_str("then"), py_str("why"), py_str("how long"), py_str("how much"), py_str("how many"), py_str("how often"), py_str("when"), py_str("where"), py_str("which"), py_str("same"), py_str("another"), py_str("more"), py_str("else"), py_str("other"), py_str("instead"), py_str("so that"), py_str("ok and"), py_str("ok so"), py_str("even if"), py_str("does it"), py_str("is it"), py_str("can it"), py_str("will it"), py_str("does that"), py_str("is that"), py_str("can that"), py_str("will that"), py_str("would it"), py_str("should it"), py_str("what else"), py_str("anything else"), py_str("and then")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.FOLLOWUP_STARTS"); }
    s_FOLLOWUP_STARTS(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.POINTING_BACK"); }
    s_POINTING_BACK(call_builtin("frozenset", &[mcall(&py_str("on in at with to for from by under over near into onto around\n    inside outside about off through across behind beside"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.ASK_OPENERS"); }
    s_ASK_OPENERS(call_builtin("frozenset", &[mcall(&py_str("will would shall should can could do does did may might must"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.WHERE_WORDS"); }
    s_WHERE_WORDS(call_builtin("frozenset", &[mcall(&py_str("there where place city area town country"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.CLAUSE_SUBJECTS"); }
    s_CLAUSE_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it there its im ive dont cant thats"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.TIME_RE"); }
    s_TIME_RE(stdlib_call("re", "compile", &[py_str("^(and|also|but|so)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.OWN_THINGS"); }
    s_OWN_THINGS(py_tuple(vec![py_str("title"), py_str("name"), py_str("person"), py_str("place"), py_str("thing")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.OWN_PERSONS"); }
    s_OWN_PERSONS(py_tuple(vec![py_str("title"), py_str("name"), py_str("person")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.DETERMINERS"); }
    s_DETERMINERS(call_builtin("frozenset", &[mcall(&py_str("a an the this these those that my your his her its our their\n    some any each every another"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.COORDINATORS"); }
    s_COORDINATORS(call_builtin("frozenset", &[mcall(&py_str("and but so then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.POSSESSIVE_PRONOUNS"); }
    s_POSSESSIVE_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("its their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.PRONOUN_WORDS"); }
    s_PRONOUN_WORDS(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PRONOUNS(), "values", &[])) { let mut forms: Py = __item.clone(); for __item in py_iter_or_raise(&forms.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.CLAUSE_STOPS"); }
    s_CLAUSE_STOPS(call_builtin("frozenset", &[mcall(&py_str("is was are were be been am do does did have has had will would can\n    could should to of in on at for from with by if when because so that which who"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.ONE_PERSON"); }
    s_ONE_PERSON(py_tuple(vec![py_str("he"), py_str("him"), py_str("his"), py_str("she"), py_str("her"), py_str("hers")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.PLACEHOLDER_IT"); }
    s_PLACEHOLDER_IT(stdlib_call("re", "compile", &[py_str("^(?:is|was|would|will|does|did|can|could|should)(?:n'?t)?\\s+it\\s+(?:[a-z']+\\s+){0,3}?to\\s+[a-z]+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.NAMES_A_THING"); }
    s_NAMES_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|some|any|my|your|our|their|his|her|its|another)\\s+[a-z]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.STANDS_FOR_SOMETHING"); }
    s_STANDS_FOR_SOMETHING(call_builtin("frozenset", &[mcall(&py_str("it its them they that this these those him her one ones\n    there"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.LEAVES_SOMETHING_OUT"); }
    s_LEAVES_SOMETHING_OUT(call_builtin("frozenset", &[mcall(&py_str("after before then while during since until till instead\n    without with on in at for to from about when if and but or that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.ANIMALS_IN_HAND"); }
    s_ANIMALS_IN_HAND(stdlib_call("re", "compile", &[py_str("\\b(dogs?|pupp(?:y|ies)|cats?|kittens?|rabbits?|bunn(?:y|ies)|hamsters?|guinea\\s+pigs?|parrots?|budgies?|goldfish|fish|tortoises?|turtles?|snakes?|lizards?|geckos?|horses?|pon(?:y|ies))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.OWN_SUBJECT"); }
    s_OWN_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill we weve wed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.WITHOUT_THE_APOSTROPHE"); }
    s_WITHOUT_THE_APOSTROPHE(py_dict(vec![(py_str("he"), py_tuple(vec![py_str("hes"), py_str("hed"), py_str("hisself")])), (py_str("she"), py_tuple(vec![py_str("shes"), py_str("shed")])), (py_str("they"), py_tuple(vec![py_str("theyre"), py_str("theyd"), py_str("theyll")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.ANIMAL_NAMES"); }
    s_ANIMAL_NAMES(call_builtin("frozenset", &[mcall(&py_str("dog dogs puppy puppies cat cats kitten kittens"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.THIRD_PERSON"); }
    s_THIRD_PERSON(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRONOUNS()) { let mut name: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&g_PRONOUNS(), &name.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.references.MINE"); }
    s_MINE(call_builtin("frozenset", &[mcall(&py_str("my mine our ours"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _target(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut prev_line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_line"))) };
    let mut prev_entities: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_entities"))) };
    let mut prev_topic: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_topic"))) };
    let mut at: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut later: Py = Py::None;
    let mut things: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the last user line was about: its short phrase, and what kind of thing it is.\n\n    A pronoun stands for what the line was *about*, not for the last noun in it: after \"I'm\n    baking a birthday cake for my son\", \"it\" is the cake, not the son. So things come first,\n    anchored on the first thing the line names, with the thing words next to it joined,\n    because \"birthday cake\" and \"beef stew\" are one thing. When the line names nothing, the\n    last resort is the topic it was read as, and `kind` says so: a topic label (\"school and\n    study\") is not a thing a sentence can be about, so a caller that needs a noun can tell the\n    two apart.\n    ");
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&prev_entities.clone()) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&entity.clone(), &py_str("type")), &py_str("thing")))) {  __out.push(entity.clone()); } } py_list(__out) })() };
    if py_truthy(&things.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![geti(&geti(&things.clone(), &Py::Int(0)), &py_str("text"))]), geti(&geti(&things.clone(), &Py::Int(0)), &py_str("at"))]), 2, None);
    words = __unpk1[0].clone();
    at = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&slice_or_raise(&things.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter2 {
    later = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&later.clone(), &py_str("at")), &op_add(&at.clone(), &Py::Int(1))))) {
    let _ = mcall(&words.clone(), "append", &[geti(&later.clone(), &py_str("text"))]);
    at = geti(&later.clone(), &py_str("at"));
    }
    }
    return py_tuple(vec![mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "lower", &[]), py_str("thing")]);
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("title"), py_str("name"), py_str("place"), py_str("person")]));
    for __item in __iter3 {
    kind = __item.clone();
    let __iter4 = py_iter_or_raise(&prev_entities.clone());
    for __item in __iter4 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&entity.clone(), &py_str("type")), &kind.clone()))) {
    return py_tuple(vec![mcall(&geti(&entity.clone(), &py_str("text")), "lower", &[]), kind.clone()]);
    }
    }
    }
    if py_truthy(&prev_topic.clone()) {
    return py_tuple(vec![mcall(&prev_topic.clone(), "replace", &[py_str("-"), py_str(" ")]), py_str("topic")]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut prev_entities: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_entities"))) };
    let mut fallback: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fallback") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fallback"))) };
    let mut entity: Py = Py::None;
    let mut kind: Py = Py::None;
    let _ = py_str("The person the last line was about, for he/she: a proper name first, then the\n    relation (\"my daughter\"), so \"her\" points at Mia rather than at the phrase.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("name"), py_str("title"), py_str("person")]));
    for __item in __iter1 {
    kind = __item.clone();
    let __iter2 = py_iter_or_raise(&prev_entities.clone());
    for __item in __iter2 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&entity.clone(), &py_str("type")), &kind.clone()))) {
    return mcall(&geti(&entity.clone(), &py_str("text")), "lower", &[]);
    }
    }
    }
    return fallback.clone();
    Py::None
}
pub fn _place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut prev_entities: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_entities"))) };
    let mut prev_topic: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_topic"))) };
    let mut entity: Py = Py::None;
    let __iter1 = py_iter_or_raise(&prev_entities.clone());
    for __item in __iter1 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&entity.clone(), &py_str("type")), &py_str("place")))) {
    return mcall(&geti(&entity.clone(), &py_str("text")), "lower", &[]);
    }
    }
    return if py_truthy(&Py::Bool(py_eq(&prev_topic.clone(), &py_str("geography-and-travel")))) { mcall(&prev_topic.clone(), "replace", &[py_str("-"), py_str(" ")]) } else { Py::None };
    Py::None
}
pub fn _moved(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut prev_topic: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_topic"))) };
    let mut topic: Py = Py::None;
    let _ = py_str("True when this line has left the topic the last turn was about.");
    topic = mcall(&read.clone(), "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&topic.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&prev_topic.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_eq(&topic.clone(), &prev_topic.clone()));
    Py::None
}
pub fn _has_moved(args: &[Py]) -> Py {
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
    let mut prev_topic: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_topic"))) };
    let mut _u: Py = Py::None;
    let mut own: Py = Py::None;
    let _ = py_str("True when the line's own topic is not the last turn's.\n\n    `resolve()` is called with the line's first reading (`kinds.classify`), which has no\n    `topic` in it, so `_moved(read, prev_topic)` is always False on that path and a thing the\n    line names in front of its pronoun never takes the pronoun back: after \"my cat keeps\n    scratching the sofa\", \"im 3 weeks into a new job and hate it already\" read as a follow-up\n    about the sofa and answered with the return-goods card. Reading the line's own topic here\n    (nothing inherited) is what says whether it has left the last turn behind, so `a new job`,\n    `a bike` and `my shed door` can be the subject of their own `it`.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&prev_topic.clone()))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&pyrt::xrefs::xcall("understanding.topics", "detect_topic", &[line.clone(), Py::None, pyrt::xrefs::xcall("understanding.topics", "detect_entities", &[line.clone()])]), 2, None);
    own = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    return { let __b = call_builtin("bool", &[own.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&own.clone(), &prev_topic.clone())) } };
    Py::None
}
pub fn _a_thing_of_its_own(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut opened: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("True when a noun phrase the line opens itself is what the pronoun's clause is about.\n\n    \"a bike\", \"my shed door\" and \"a new job\" start a thing; a bare noun is an argument of\n    the sentence (\"will vinegar fix it\", \"will lime turn it pink\"), so it starts nothing. The\n    phrase also has to be joined to the pronoun's clause by a coordinator (\"bought a bike and\n    i love it\") or be the thing whose own part a possessive names (\"my car failed its mot\").\n    ");
    opened = _last_noun_phrase(&[norm.clone(), at.clone()]);
    if py_truthy(&Py::Bool(py_is(&opened.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_POSSESSIVE_PRONOUNS(), &word.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &opened.clone(), &at.clone(), &Py::None)) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_COORDINATORS(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_noun_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the last determiner in the words before `at` opens a noun phrase, or None.");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&at.clone(), &Py::Int(2)), call_builtin("max", &[op_neg(&Py::Int(1)), op_sub(&at.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    word = geti(&norm.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DETERMINERS(), &word.clone())); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&index.clone(), &Py::Int(1)), &at.clone()) } }) {
    return index.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_PRONOUN_WORDS(), &word.clone()))) {
    return Py::None;
    }
    }
    return Py::None;
    Py::None
}
pub fn _named_before(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut moved: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "moved") { v.clone() }
    else { Py::Bool(true) };
    let mut at: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut found: Py = Py::None;
    let mut i: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut people: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pronouns of `names` that come after a thing the line names itself.\n\n    \"my mum keeps asking when we're having kids and it's doing my head in\" names the mum and the\n    kids before its \"it\", so that \"it\" is about the mum's nagging, not the chicken cooked a turn\n    earlier. In \"how do i stop it chewing the furniture?\" the \"it\" comes first, so it still\n    points back at the puppy.\n\n    `moved` says whether the line has left the topic of the last turn. A thing named in the\n    same conversation is usually an argument of the sentence rather than a new subject -\n    \"will vinegar fix it\" and \"50 pounds to spend on it\" name a tool and a price, and their\n    \"it\" is still the pan and the thing being bought. A person named in front of the pronoun\n    takes it away on its own; a thing has to be a noun phrase of its own (\"a new job\", \"my\n    shed door\") in a clause joined to the pronoun's (\"and i hate it\"), or be the thing whose\n    own part the possessive pronoun names (\"my car failed its mot\").\n    ");
    norm = geti(&line.clone(), &py_str("norm"));
    people = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("understanding.topics", "detect_entities", &[line.clone()])) { let mut entity: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_OWN_PERSONS(), &mcall(&entity.clone(), "get", &[py_str("type")])))) {  __out.push(mcall(&entity.clone(), "get", &[py_str("at"), Py::Int(0)])); } } py_list(__out) })() };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    at = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9973 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk9973[0].clone(); let mut word: Py = __unpk9973[1].clone(); if py_truthy(&Py::Bool(py_contains(&geti(&g_PRONOUNS(), &name.clone()), &word.clone()))) {  __out.push(i.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = at.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTING_BACK(), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    if py_truthy(&_asks_about(&[line.clone(), py_tuple(vec![name.clone()])])) {
    continue;
    }
    named = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&people.clone()) { let mut spot: Py = __item.clone(); __out.push(op_lt(&spot.clone(), &at.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { moved.clone() } }) {
    named = _a_thing_of_its_own(&[norm.clone(), at.clone(), geti(&norm.clone(), &at.clone())]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[name.clone()]);
    }
    return found.clone();
    Py::None
}
pub fn _asks_about(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut aliases: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("True when the clause that holds one of `names` is a question: \"will that work\".\n\n    A clause that asks is leaning on the last turn whatever it names: \"my neighbour uses\n    vinegar, will that work\" asks whether the vinegar works *for the ants*, so the answer is\n    still the ants.\n    ");
    aliases = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    let _ = mcall(&aliases.clone(), "update", &[geti(&g_PRONOUNS(), &name.clone())]);
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&line.clone(), "get", &[py_str("clauses")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    clause = __item.clone();
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_OPENERS(), &geti(&clause.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { op_bitand(&aliases.clone(), &call_builtin("set", &[slice_or_raise(&clause.clone(), &Py::None, &Py::Int(3), &Py::None)]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn person_is_new(args: &[Py]) -> Py {
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
    let mut prev_entities: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: prev_entities"))) };
    let mut entity: Py = Py::None;
    let mut people_here: Py = Py::None;
    let _ = py_str("True when the line is about a person no line in the chat has introduced yet.\n\n    \"should i text him back\" is a question about a person, and after an answer about a knee\n    there is no him anywhere in the chat: nobody has been named, so the person is new. Read as\n    a follow-up (\"should i text him back\" with \"my knee clicks when i climb stairs\" in front of\n    it) the knee entry answers it again and the person gets their own question back. The same\n    line after \"my landlord wont fix the boiler\" is about the landlord the last line named, so\n    it is a follow-up and keeps leaning.\n\n    Kept as a plain question about the words and the last line's people, so a caller can use it\n    as a fence: a line that brings somebody new is a line of its own.\n    ");
    people_here = { let __b = call_builtin("bool", &[pyrt::xrefs::xcall("understanding.topics", "person_names", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("understanding.topics", "detect_entities", &[line.clone()])) { let mut entity: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("person")))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&people_here.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[{ let __b = mcall(&line.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_ONE_PERSON()]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = prev_entities.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut entity: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("person")))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _only_the_purpose(args: &[Py]) -> Py {
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
    let mut norm: Py = Py::None;
    let _ = py_str("True for a line that is only a purpose clause: \"to cycle to work\".\n\n    Nobody opens a conversation with \"to cycle to work\". It is the rest of the last turn's plan\n    (\"i want to get fit\"), so the line goes on with what was being talked about - and read as a\n    greeting it was answered as if the person had just said hello (\"Got it - to cycle to work\").\n    The infinitive is the whole line: a clause with its own subject after it (\"to be honest it\n    hurts\") is the speaker's own sentence, and only what \"it\" is comes from the last turn.\n    ");
    norm = geti(&line.clone(), &py_str("norm"));
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&norm.clone(), &Py::Int(0)), &py_str("to"))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[slice_or_raise(&norm.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_CLAUSE_SUBJECTS())));
    Py::None
}
pub fn _a_placeholder_it(args: &[Py]) -> Py {
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
    let mut frame: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True for \"is it <test> to <verb> ...\" where the clause names nothing of its own.\n\n    The whole line is the test, so a caller can use it as a fence: a line that brings a\n    placeholder is a line of its own and the last turn's thing is not what it is about.\n    ");
    said = mcall(&py_str(" "), "join", &[geti(&line.clone(), &py_str("norm"))]);
    frame = mcall(&g_PLACEHOLDER_IT(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&frame.clone()))) {
    return Py::Bool(false);
    }
    rest = slice_or_raise(&said.clone(), &mcall(&frame.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&mcall(&g_NAMES_A_THING(), "search", &[rest.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STANDS_FOR_SOMETHING())) {
    return Py::Bool(false);
    }
    tail = slice_or_raise(&mcall(&rest.clone(), "split", &[]), &op_neg(&Py::Int(1)), &Py::None, &Py::None);
    return Py::Bool(!py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEAVES_SOMETHING_OUT(), &geti(&tail.clone(), &Py::Int(0)))) } }));
    Py::None
}
pub fn _the_breed_in(args: &[Py]) -> Py {
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
    let mut BREED_KIND: Py = Py::None;
    let mut filled: Py = Py::None;
    let mut index: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut the_animal_a_breed_is: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The animal a breed word in the line names, or \"\" when the line names none.\n\n    \"we got a rescue greyhound on saturday\" and then \"she wont eat in the day\" is a question\n    about the dog: the breed is the only animal word in the thread, and the pronouns a person\n    uses for a dog are the same ones they use for a cat. The table of breeds and the frames that\n    make one an animal (not a bus, not a race) is `breeds.py`'s, so this asks it rather than\n    keeping a second list of dog names here.\n    ");
    BREED_KIND = pyrt::xrefs::xget("understanding.breeds", "BREED_KIND");
    the_animal_a_breed_is = py_func("the_animal_a_breed_is", |args: &[Py]| pyrt::xrefs::xcall("understanding.breeds", "the_animal_a_breed_is", args));
    whole = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&BREED_KIND.clone(), "get", &[whole.clone()])) {
    return geti(&BREED_KIND.clone(), &whole.clone());
    }
    said = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    filled = py_call(&the_animal_a_breed_is, &[call_builtin("list", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[filled.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[filled.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return py_str("");
    }
    index = Py::Int(0);
    let __iter1 = py_iter_or_raise(&filled.clone());
    for __item in __iter1 {
    piece = __item.clone();
    if py_truthy(&{ let __b = op_lt(&index.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&piece.clone(), &geti(&said.clone(), &index.clone()))) } }) {
    let __aug2 = op_add(&index.clone(), &Py::Int(1));
    index = __aug2;
    continue;
    }
    return mcall(&call_builtin("str", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _the_pronouns(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    return op_bitor(&call_builtin("frozenset", &[geti(&g_PRONOUNS(), &name.clone())]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&g_WITHOUT_THE_APOSTROPHE(), "get", &[name.clone(), py_tuple(vec![])])]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn the_animal_the_chat_is_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut last_read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last_read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last_read"))) };
    let mut animal: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The animal the line before had in hand, when this line only points at it.\n\n    A pronoun in this line has to point at something, and a line that names no animal of its own\n    is pointing at the one the chat has been about. A line that makes the speaker its subject\n    (\"i've got a headache too\") or that names another animal keeps what it says.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last_read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    animal = mcall(&call_builtin("str", &[{ let __b = mcall(&last_read.clone(), "get", &[py_str("animal_in_hand")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&animal.clone()))) {
    return py_str("");
    }
    bare = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    let _ = mcall(&bare.clone(), "add", &[geti(&mcall(&mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[op_add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’"))]), "split", &[py_str("'")]), &Py::Int(0))]);
    }
    if py_truthy(&{ let __b = op_bitand(&bare.clone(), &g_ANIMAL_NAMES()); if py_truthy(&__b) { __b } else { op_bitand(&bare.clone(), &g_MINE()) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&bare.clone(), &g_THIRD_PERSON())))) {
    return py_str("");
    }
    return animal.clone();
    Py::None
}
pub fn the_animal_in_hand(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ref_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ref") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ref"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut animal: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut breed: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The animal a follow-up only points at, or \"\" when the line is about something else.\n\n    Only for a line whose subject is the pronoun: a line that says nothing of its own about which\n    animal (\"she's been drinking a lot more water\") cannot be answered by any entry keyed on a cat\n    or a dog. A line with a subject of its own keeps it, and so does one that already names an\n    animal.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&ref_.clone(), "get", &[py_str("followup")]))) } }) {
    return py_str("");
    }
    bare = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    let _ = mcall(&bare.clone(), "add", &[geti(&mcall(&mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[op_add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’"))]), "split", &[py_str("'")]), &Py::Int(0))]);
    }
    if py_truthy(&{ let __b = op_bitand(&bare.clone(), &g_OWN_SUBJECT()); if py_truthy(&__b) { __b } else { op_bitand(&bare.clone(), &g_ANIMAL_NAMES()) } }) {
    return py_str("");
    }
    let __iter2 = py_iter_or_raise(&g_PRONOUNS());
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&bare.clone(), &_the_pronouns(&[name.clone()]))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&ref_.clone(), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    animal = mcall(&g_ANIMALS_IN_HAND(), "search", &[said.clone()]);
    if py_truthy(&animal.clone()) {
    return mcall(&mcall(&animal.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    }
    breed = _the_breed_in(&[said.clone()]);
    if py_truthy(&breed.clone()) {
    return breed.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn resolve(args: &[Py]) -> Py {
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
    let mut read: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut prev_line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_line") { v.clone() }
    else { Py::None };
    let mut prev_entities: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_entities") { v.clone() }
    else { Py::None };
    let mut prev_topic: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prev_topic") { v.clone() }
    else { Py::None };
    let mut adds_topic: Py = Py::None;
    let mut aliases: Py = Py::None;
    let mut asks_without_topic: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named_first: Py = Py::None;
    let mut opens_like_followup: Py = Py::None;
    let mut p: Py = Py::None;
    let mut person: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut placeholder: Py = Py::None;
    let mut pointed_back: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut reference: Py = Py::None;
    let mut short: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut target: Py = Py::None;
    let mut used: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{\"it\", \"that\", \"they\", \"there\", \"followup\"} for the current line.");
    prev_entities = { let __b = prev_entities.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    words = call_builtin("set", &[geti(&line.clone(), &py_str("norm"))]).unwrap_or_else(|e| exc::raise(e));
    used = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PRONOUNS(), "items", &[])) { let __unpk9445 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk9445[0].clone(); let mut aliases: Py = __unpk9445[1].clone(); __out.push((name.clone(), call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&aliases.clone()) { let mut p: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &p.clone()))) {  __out.push(p.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e)))); } py_dict(__out) })() };
    ref_ = py_dict(vec![(py_str("it"), Py::None), (py_str("that"), Py::None), (py_str("they"), Py::None), (py_str("there"), Py::None), (py_str("he"), Py::None), (py_str("she"), Py::None), (py_str("kind"), Py::None), (py_str("followup"), Py::Bool(false))]);
    if py_truthy(&Py::Bool(py_is(&prev_line.clone(), &Py::None))) {
    return ref_.clone();
    }
    let __unpk1 = py_unpack(&_target(&[prev_line.clone(), prev_entities.clone(), prev_topic.clone()]), 2, None);
    target = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    person = _person(&[prev_entities.clone(), target.clone()]);
    reference = call_builtin("bool", &[{ let __b = geti(&used.clone(), &py_str("it")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("that")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("they")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("there")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("he")); if py_truthy(&__b) { __b } else { geti(&used.clone(), &py_str("she")) } } } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    placeholder = _a_placeholder_it(&[line.clone()]);
    starts = mcall(&py_str(" "), "join", &[slice_or_raise(&geti(&line.clone(), &py_str("norm")), &Py::None, &Py::Int(4), &Py::None)]);
    opens_like_followup = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOLLOWUP_STARTS()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&starts.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { mcall(&starts.clone(), "startswith", &[op_add(&phrase.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { _only_the_purpose(&[line.clone()]) } };
    if py_truthy(&{ let __b = placeholder.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = geti(&used.clone(), &py_str("that")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("they")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("there")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("he")); if py_truthy(&__b) { __b } else { geti(&used.clone(), &py_str("she")) } } } } } } } })) } }) {
    opens_like_followup = Py::Bool(false);
    asks_without_topic = Py::Bool(false);
    }
    short = { let __b = op_le(&call_builtin("len", &[geti(&line.clone(), &py_str("norm"))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("greeting"), py_str("thanks")]), &geti(&read.clone(), &py_str("kind")))) } };
    adds_topic = Py::Bool(!py_is(&pyrt::xrefs::xcall("understanding.topics", "subject_topic", &[line.clone(), pyrt::xrefs::xcall("understanding.topics", "detect_entities", &[line.clone()])]), &Py::None));
    asks_without_topic = { let __b = Py::Bool(py_eq(&geti(&read.clone(), &py_str("kind")), &py_str("question"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&adds_topic.clone())) } };
    let _ = py_setitem(&ref_.clone(), &py_str("kind"), kind.clone());
    if py_truthy(&{ let __b = reference.clone(); if !py_truthy(&__b) { __b } else { { let __b = placeholder.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = geti(&used.clone(), &py_str("that")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("they")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("there")); if py_truthy(&__b) { __b } else { { let __b = geti(&used.clone(), &py_str("he")); if py_truthy(&__b) { __b } else { geti(&used.clone(), &py_str("she")) } } } } } } } })) } } } }) {
    reference = Py::Bool(false);
    }
    if py_truthy(&reference.clone()) {
    let _ = py_setitem(&ref_.clone(), &py_str("it"), target.clone());
    let _ = py_setitem(&ref_.clone(), &py_str("that"), target.clone());
    let _ = py_setitem(&ref_.clone(), &py_str("they"), target.clone());
    let _ = py_setitem(&ref_.clone(), &py_str("he"), person.clone());
    let _ = py_setitem(&ref_.clone(), &py_str("she"), person.clone());
    }
    let _ = py_setitem(&ref_.clone(), &py_str("there"), _place(&[prev_entities.clone(), prev_topic.clone()]));
    let _ = py_setitem(&ref_.clone(), &py_str("followup"), call_builtin("bool", &[{ let __b = reference.clone(); if py_truthy(&__b) { __b } else { { let __b = opens_like_followup.clone(); if py_truthy(&__b) { __b } else { { let __b = short.clone(); if py_truthy(&__b) { __b } else { asks_without_topic.clone() } } } } } }]).unwrap_or_else(|e| exc::raise(e)));
    named_first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_tuple(vec![py_str("it"), py_str("that"), py_str("they"), py_str("there")])); __kws.extend(vec![("moved".to_string(), { let __b = _moved(&[read.clone(), prev_topic.clone()]); if py_truthy(&__b) { __b } else { _has_moved(&[line.clone(), prev_topic.clone()]) } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_before(args))(&__args) };
    let __iter2 = py_iter_or_raise(&named_first.clone());
    for __item in __iter2 {
    name = __item.clone();
    let _ = py_setitem(&ref_.clone(), &name.clone(), Py::None);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("they"), py_str("that")]), &name.clone()))) {
    let _ = py_setitem(&ref_.clone(), &py_str("he"), Py::None);
    let _ = py_setitem(&ref_.clone(), &py_str("she"), Py::None);
    }
    }
    pointed_back = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("it"), py_str("that"), py_str("they"), py_str("there")])) { let mut name: Py = __item.clone(); if py_truthy(&geti(&used.clone(), &name.clone())) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = pointed_back.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("set", &[pointed_back.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[named_first.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = geti(&used.clone(), &py_str("he")); if py_truthy(&__b) { __b } else { geti(&used.clone(), &py_str("she")) } })) } } } }) {
    let _ = py_setitem(&ref_.clone(), &py_str("followup"), call_builtin("bool", &[{ let __b = opens_like_followup.clone(); if py_truthy(&__b) { __b } else { { let __b = short.clone(); if py_truthy(&__b) { __b } else { asks_without_topic.clone() } } } }]).unwrap_or_else(|e| exc::raise(e)));
    }
    return ref_.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.references".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMALS_IN_HAND" => Some(g_ANIMALS_IN_HAND()),
            "ANIMAL_NAMES" => Some(g_ANIMAL_NAMES()),
            "ASK_OPENERS" => Some(g_ASK_OPENERS()),
            "CLAUSE_STOPS" => Some(g_CLAUSE_STOPS()),
            "CLAUSE_SUBJECTS" => Some(g_CLAUSE_SUBJECTS()),
            "COORDINATORS" => Some(g_COORDINATORS()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "FOLLOWUP_STARTS" => Some(g_FOLLOWUP_STARTS()),
            "LEAVES_SOMETHING_OUT" => Some(g_LEAVES_SOMETHING_OUT()),
            "MINE" => Some(g_MINE()),
            "NAMES_A_THING" => Some(g_NAMES_A_THING()),
            "ONE_PERSON" => Some(g_ONE_PERSON()),
            "OWN_PERSONS" => Some(g_OWN_PERSONS()),
            "OWN_SUBJECT" => Some(g_OWN_SUBJECT()),
            "OWN_THINGS" => Some(g_OWN_THINGS()),
            "PLACEHOLDER_IT" => Some(g_PLACEHOLDER_IT()),
            "POINTING_BACK" => Some(g_POINTING_BACK()),
            "POSSESSIVE_PRONOUNS" => Some(g_POSSESSIVE_PRONOUNS()),
            "PRONOUNS" => Some(g_PRONOUNS()),
            "PRONOUN_WORDS" => Some(g_PRONOUN_WORDS()),
            "STANDS_FOR_SOMETHING" => Some(g_STANDS_FOR_SOMETHING()),
            "THIRD_PERSON" => Some(g_THIRD_PERSON()),
            "TIME_RE" => Some(g_TIME_RE()),
            "WHERE_WORDS" => Some(g_WHERE_WORDS()),
            "WITHOUT_THE_APOSTROPHE" => Some(g_WITHOUT_THE_APOSTROPHE()),
            "_a_placeholder_it" => Some(py_func("_a_placeholder_it", |args: &[Py]| _a_placeholder_it(args))),
            "_a_thing_of_its_own" => Some(py_func("_a_thing_of_its_own", |args: &[Py]| _a_thing_of_its_own(args))),
            "_asks_about" => Some(py_func("_asks_about", |args: &[Py]| _asks_about(args))),
            "_has_moved" => Some(py_func("_has_moved", |args: &[Py]| _has_moved(args))),
            "_last_noun_phrase" => Some(py_func("_last_noun_phrase", |args: &[Py]| _last_noun_phrase(args))),
            "_moved" => Some(py_func("_moved", |args: &[Py]| _moved(args))),
            "_named_before" => Some(py_func("_named_before", |args: &[Py]| _named_before(args))),
            "_only_the_purpose" => Some(py_func("_only_the_purpose", |args: &[Py]| _only_the_purpose(args))),
            "_person" => Some(py_func("_person", |args: &[Py]| _person(args))),
            "_place" => Some(py_func("_place", |args: &[Py]| _place(args))),
            "_target" => Some(py_func("_target", |args: &[Py]| _target(args))),
            "_the_breed_in" => Some(py_func("_the_breed_in", |args: &[Py]| _the_breed_in(args))),
            "_the_pronouns" => Some(py_func("_the_pronouns", |args: &[Py]| _the_pronouns(args))),
            "person_is_new" => Some(py_func("person_is_new", |args: &[Py]| person_is_new(args))),
            "resolve" => Some(py_func("resolve", |args: &[Py]| resolve(args))),
            "the_animal_in_hand" => Some(py_func("the_animal_in_hand", |args: &[Py]| the_animal_in_hand(args))),
            "the_animal_the_chat_is_on" => Some(py_func("the_animal_the_chat_is_on", |args: &[Py]| the_animal_the_chat_is_on(args))),
            "topics" => Some(pyrt::xrefs::xmodule("understanding.topics")),
            "re" => Some(stdlib_module("re")),
            "string" => Some(stdlib_module("string")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMALS_IN_HAND" => s_ANIMALS_IN_HAND(v),
            "ANIMAL_NAMES" => s_ANIMAL_NAMES(v),
            "ASK_OPENERS" => s_ASK_OPENERS(v),
            "CLAUSE_STOPS" => s_CLAUSE_STOPS(v),
            "CLAUSE_SUBJECTS" => s_CLAUSE_SUBJECTS(v),
            "COORDINATORS" => s_COORDINATORS(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "FOLLOWUP_STARTS" => s_FOLLOWUP_STARTS(v),
            "LEAVES_SOMETHING_OUT" => s_LEAVES_SOMETHING_OUT(v),
            "MINE" => s_MINE(v),
            "NAMES_A_THING" => s_NAMES_A_THING(v),
            "ONE_PERSON" => s_ONE_PERSON(v),
            "OWN_PERSONS" => s_OWN_PERSONS(v),
            "OWN_SUBJECT" => s_OWN_SUBJECT(v),
            "OWN_THINGS" => s_OWN_THINGS(v),
            "PLACEHOLDER_IT" => s_PLACEHOLDER_IT(v),
            "POINTING_BACK" => s_POINTING_BACK(v),
            "POSSESSIVE_PRONOUNS" => s_POSSESSIVE_PRONOUNS(v),
            "PRONOUNS" => s_PRONOUNS(v),
            "PRONOUN_WORDS" => s_PRONOUN_WORDS(v),
            "STANDS_FOR_SOMETHING" => s_STANDS_FOR_SOMETHING(v),
            "THIRD_PERSON" => s_THIRD_PERSON(v),
            "TIME_RE" => s_TIME_RE(v),
            "WHERE_WORDS" => s_WHERE_WORDS(v),
            "WITHOUT_THE_APOSTROPHE" => s_WITHOUT_THE_APOSTROPHE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.references", module_obj);
    pyrt::xrefs::reg_fn("understanding.references", "_target", py_func("_target", |args: &[Py]| _target(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_person", py_func("_person", |args: &[Py]| _person(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_place", py_func("_place", |args: &[Py]| _place(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_moved", py_func("_moved", |args: &[Py]| _moved(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_has_moved", py_func("_has_moved", |args: &[Py]| _has_moved(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_a_thing_of_its_own", py_func("_a_thing_of_its_own", |args: &[Py]| _a_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_last_noun_phrase", py_func("_last_noun_phrase", |args: &[Py]| _last_noun_phrase(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_named_before", py_func("_named_before", |args: &[Py]| _named_before(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_asks_about", py_func("_asks_about", |args: &[Py]| _asks_about(args)));
    pyrt::xrefs::reg_fn("understanding.references", "person_is_new", py_func("person_is_new", |args: &[Py]| person_is_new(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_only_the_purpose", py_func("_only_the_purpose", |args: &[Py]| _only_the_purpose(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_a_placeholder_it", py_func("_a_placeholder_it", |args: &[Py]| _a_placeholder_it(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_the_breed_in", py_func("_the_breed_in", |args: &[Py]| _the_breed_in(args)));
    pyrt::xrefs::reg_fn("understanding.references", "_the_pronouns", py_func("_the_pronouns", |args: &[Py]| _the_pronouns(args)));
    pyrt::xrefs::reg_fn("understanding.references", "the_animal_the_chat_is_on", py_func("the_animal_the_chat_is_on", |args: &[Py]| the_animal_the_chat_is_on(args)));
    pyrt::xrefs::reg_fn("understanding.references", "the_animal_in_hand", py_func("the_animal_in_hand", |args: &[Py]| the_animal_in_hand(args)));
    pyrt::xrefs::reg_fn("understanding.references", "resolve", py_func("resolve", |args: &[Py]| resolve(args)));
    pyrt::xrefs::reg_access("understanding.references", "ANIMALS_IN_HAND", g_ANIMALS_IN_HAND, s_ANIMALS_IN_HAND);
    pyrt::xrefs::reg_access("understanding.references", "ANIMAL_NAMES", g_ANIMAL_NAMES, s_ANIMAL_NAMES);
    pyrt::xrefs::reg_access("understanding.references", "ASK_OPENERS", g_ASK_OPENERS, s_ASK_OPENERS);
    pyrt::xrefs::reg_access("understanding.references", "CLAUSE_STOPS", g_CLAUSE_STOPS, s_CLAUSE_STOPS);
    pyrt::xrefs::reg_access("understanding.references", "CLAUSE_SUBJECTS", g_CLAUSE_SUBJECTS, s_CLAUSE_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.references", "COORDINATORS", g_COORDINATORS, s_COORDINATORS);
    pyrt::xrefs::reg_access("understanding.references", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("understanding.references", "FOLLOWUP_STARTS", g_FOLLOWUP_STARTS, s_FOLLOWUP_STARTS);
    pyrt::xrefs::reg_access("understanding.references", "LEAVES_SOMETHING_OUT", g_LEAVES_SOMETHING_OUT, s_LEAVES_SOMETHING_OUT);
    pyrt::xrefs::reg_access("understanding.references", "MINE", g_MINE, s_MINE);
    pyrt::xrefs::reg_access("understanding.references", "NAMES_A_THING", g_NAMES_A_THING, s_NAMES_A_THING);
    pyrt::xrefs::reg_access("understanding.references", "ONE_PERSON", g_ONE_PERSON, s_ONE_PERSON);
    pyrt::xrefs::reg_access("understanding.references", "OWN_PERSONS", g_OWN_PERSONS, s_OWN_PERSONS);
    pyrt::xrefs::reg_access("understanding.references", "OWN_SUBJECT", g_OWN_SUBJECT, s_OWN_SUBJECT);
    pyrt::xrefs::reg_access("understanding.references", "OWN_THINGS", g_OWN_THINGS, s_OWN_THINGS);
    pyrt::xrefs::reg_access("understanding.references", "PLACEHOLDER_IT", g_PLACEHOLDER_IT, s_PLACEHOLDER_IT);
    pyrt::xrefs::reg_access("understanding.references", "POINTING_BACK", g_POINTING_BACK, s_POINTING_BACK);
    pyrt::xrefs::reg_access("understanding.references", "POSSESSIVE_PRONOUNS", g_POSSESSIVE_PRONOUNS, s_POSSESSIVE_PRONOUNS);
    pyrt::xrefs::reg_access("understanding.references", "PRONOUNS", g_PRONOUNS, s_PRONOUNS);
    pyrt::xrefs::reg_access("understanding.references", "PRONOUN_WORDS", g_PRONOUN_WORDS, s_PRONOUN_WORDS);
    pyrt::xrefs::reg_access("understanding.references", "STANDS_FOR_SOMETHING", g_STANDS_FOR_SOMETHING, s_STANDS_FOR_SOMETHING);
    pyrt::xrefs::reg_access("understanding.references", "THIRD_PERSON", g_THIRD_PERSON, s_THIRD_PERSON);
    pyrt::xrefs::reg_access("understanding.references", "TIME_RE", g_TIME_RE, s_TIME_RE);
    pyrt::xrefs::reg_access("understanding.references", "WHERE_WORDS", g_WHERE_WORDS, s_WHERE_WORDS);
    pyrt::xrefs::reg_access("understanding.references", "WITHOUT_THE_APOSTROPHE", g_WITHOUT_THE_APOSTROPHE, s_WITHOUT_THE_APOSTROPHE);
}

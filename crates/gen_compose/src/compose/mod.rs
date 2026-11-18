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


thread_local! { static G_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BELIEVE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTED_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINGERED_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS_ON_A_FRAGMENT_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_IT_HAS_LINGERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WHEN_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_AT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAPITAL_I: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HELLO_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_FRAGMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONE_WORD_THEN_A_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_A_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHORT_RUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AUX() -> Py { ensure_init(); G_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AUX(v: Py) { ensure_init(); G_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_ASK() -> Py { ensure_init(); G_A_STEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_ASK(v: Py) { ensure_init(); G_A_STEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BELIEVE_IT() -> Py { ensure_init(); G_BELIEVE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BELIEVE_IT(v: Py) { ensure_init(); G_BELIEVE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT() -> Py { ensure_init(); G_CANNOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT(v: Py) { ensure_init(); G_CANNOT.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTED_AUX() -> Py { ensure_init(); G_CONTRACTED_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTED_AUX(v: Py) { ensure_init(); G_CONTRACTED_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT() -> Py { ensure_init(); G_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT(v: Py) { ensure_init(); G_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBTED() -> Py { ensure_init(); G_DOUBTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBTED(v: Py) { ensure_init(); G_DOUBTED.with(|c| *c.borrow_mut() = v); }
pub fn g_LINGERED_LINE() -> Py { ensure_init(); G_LINGERED_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINGERED_LINE(v: Py) { ensure_init(); G_LINGERED_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARLY() -> Py { ensure_init(); G_NEARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARLY(v: Py) { ensure_init(); G_NEARLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATORS() -> Py { ensure_init(); G_NEGATORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATORS(v: Py) { ensure_init(); G_NEGATORS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS_ON_A_FRAGMENT_QUESTION() -> Py { ensure_init(); G_OPENS_ON_A_FRAGMENT_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS_ON_A_FRAGMENT_QUESTION(v: Py) { ensure_init(); G_OPENS_ON_A_FRAGMENT_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_IT_HAS_LINGERED() -> Py { ensure_init(); G_SAYS_IT_HAS_LINGERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_HAS_LINGERED(v: Py) { ensure_init(); G_SAYS_IT_HAS_LINGERED.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WHEN_LOW() -> Py { ensure_init(); G_STEP_WHEN_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WHEN_LOW(v: Py) { ensure_init(); G_STEP_WHEN_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_AT_IT() -> Py { ensure_init(); G_STILL_AT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_AT_IT(v: Py) { ensure_init(); G_STILL_AT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_SURE() -> Py { ensure_init(); G_SURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SURE(v: Py) { ensure_init(); G_SURE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAPITAL_I() -> Py { ensure_init(); G__CAPITAL_I.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAPITAL_I(v: Py) { ensure_init(); G__CAPITAL_I.with(|c| *c.borrow_mut() = v); }
pub fn g__HELLO_OPENS() -> Py { ensure_init(); G__HELLO_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HELLO_OPENS(v: Py) { ensure_init(); G__HELLO_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_FRAGMENT() -> Py { ensure_init(); G__NOT_A_FRAGMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_FRAGMENT(v: Py) { ensure_init(); G__NOT_A_FRAGMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__ONE_WORD_THEN_A_BREAK() -> Py { ensure_init(); G__ONE_WORD_THEN_A_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONE_WORD_THEN_A_BREAK(v: Py) { ensure_init(); G__ONE_WORD_THEN_A_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_A_SENTENCE() -> Py { ensure_init(); G__OPENS_A_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_A_SENTENCE(v: Py) { ensure_init(); G__OPENS_A_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_WORD() -> Py { ensure_init(); G__PERSON_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_WORD(v: Py) { ensure_init(); G__PERSON_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__SHORT_RUN() -> Py { ensure_init(); G__SHORT_RUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHORT_RUN(v: Py) { ensure_init(); G__SHORT_RUN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Phrasing helpers for the response composer. The routing lane owns the composer: what to do\nthis turn (bot/modules/arbiter.py chooses among the modules' replies) and how it is phrased.\n");
    fn __data1() -> Py { py_tuple(vec![py_str("not"), py_str("no"), py_str("never"), py_str("hardly"), py_str("barely"), py_str("nothing"), py_str("without"), py_str("cant"), py_str("cannot"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("isnt"), py_str("wasnt"), py_str("arent"), py_str("werent"), py_str("wont"), py_str("wouldnt"), py_str("couldnt"), py_str("shouldnt"), py_str("havent"), py_str("hasnt"), py_str("aint")]) }
    s_NEGATORS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s_NEARLY(call_builtin("frozenset", &[py_tuple(vec![py_str("nearly"), py_str("almost")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_IT_HAS_LINGERED(stdlib_call("re", "compile", &[py_str("\\b(?:wont|won'?t|not)\\s+(?:go|shift|clear|budge|settle)\\w*|\\bfor\\s+(?:\\d+|a\\s+couple\\s+of|a\\s+few|a|an|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:days?|weeks?|months?|years?)\\b|\\b(?:been going|going on|dragging on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_AT_IT(stdlib_call("re", "compile", &[py_str("\\bstill\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SHORT_RUN(stdlib_call("re", "compile", &[py_str("\\b(?:(?:one|two|a\\s+couple\\s+of)\\s+days?|day\\s+(?:one|two|three|1|2|3))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LINGERED_LINE(py_tuple(vec![py_str("Anything that has hung on is worth getting looked at — a pharmacist will see you quicker than a GP, and they'll tell you if it needs one."), py_str("That is long enough to be worth a proper look rather than another stretch of waiting: a pharmacist today, and a GP if it is still there after that."), py_str("Sorry it is still going. Note when it is worst and what eases it, then take that to a pharmacist — a thing that has lasted this long is worth putting a name to.")]));
    fn __data2() -> Py { py_tuple(vec![py_str("can"), py_str("could"), py_str("do"), py_str("does"), py_str("did"), py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("would"), py_str("should"), py_str("have"), py_str("has"), py_str("had"), py_str("wo"), py_str("ai"), py_str("must"), py_str("need"), py_str("might")]) }
    s_AUX(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("do"), py_str("did"), py_str("ca"), py_str("wo"), py_str("ai"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("has"), py_str("have"), py_str("had"), py_str("could"), py_str("would"), py_str("should"), py_str("must"), py_str("need"), py_str("dare"), py_str("does"), py_str("am"), py_str("might")]) }
    s_CONTRACTED_AUX(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT(call_builtin("frozenset", &[py_tuple(vec![py_str("believe"), py_str("believed"), py_str("imagine"), py_str("imagined"), py_str("thought"), py_str("think")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CANNOT(call_builtin("frozenset", &[py_tuple(vec![py_str("cant"), py_str("cannot"), py_str("couldnt"), py_str("never"), py_str("always")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("think"), py_str("thought"), py_str("know"), py_str("knew"), py_str("reckon"), py_str("guess"), py_str("suppose"), py_str("sure"), py_str("expect"), py_str("imagine"), py_str("say"), py_str("believe"), py_str("believed"), py_str("certain")]) }
    s_SURE(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBTED(call_builtin("frozenset", &[py_tuple(vec![py_str("doubt"), py_str("doubts"), py_str("doubted"), py_str("doubting"), py_str("unsure"), py_str("uncertain")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BELIEVE_IT(call_builtin("frozenset", &[py_tuple(vec![py_str("believe"), py_str("believed")])]).unwrap_or_else(|e| exc::raise(e)));
    s__PERSON_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|his|hers|their|himself|herself|themselves)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CAPITAL_I(stdlib_call("re", "compile", &[py_str("I(?:'|’|\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__HELLO_OPENS(py_tuple(vec![py_str("hi "), py_str("hi,"), py_str("hello"), py_str("hey"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("nice to meet"), py_str("good to meet")]));
    s__ONE_WORD_THEN_A_BREAK(stdlib_call("re", "compile", &[py_str("^([A-Za-z'\\u2019-]{1,20})\\s*(?:\\u2014|--|-|\\?|!|:|,)")]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_A_SENTENCE(call_builtin("frozenset", &[mcall(&py_str("\n    a an the and but or nor so yet if then when while with without for from to of in on at by\n    it its it's i i'm im you you're ur your yours he she they we us them there here this that\n    these those what whats why how who whom where which yes no not never always often usually\n    sometimes maybe perhaps probably honestly actually really just only also still even too\n    one two three four five few some any all each both most many more less least enough\n    good bad best worst great fine okay ok right well now today tonight tomorrow first next last\n    check try keep stop start put take give get go going let leave look make use ask call add\n    avoid buy clean cook cover cut drink eat drive hold pick raise read ring run say see set sit\n    sort speak stick tell turn walk wash wear write fill fix top find change wait watch worth\n    sounds looks seems sorry thanks thank please sorry glad sadly luckily hopefully\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENS_ON_A_FRAGMENT_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*([A-Z][\\w'\\u2019]*(?:\\s+[\\w'\\u2019]+){0,3})\\?\\s")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_FRAGMENT(call_builtin("frozenset", &[mcall(&py_str("what who why when where which how whatever honestly really\n    sorry right ok okay so then and but well anyway"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_WHEN_LOW(py_tuple(vec![py_str("Start with the smallest thing that would help: something to eat, a shower, ten minutes outside. One of those, and that is today's job."), py_str("Write down the three things on your mind, then do the smallest one. The other two can wait an hour."), py_str("Tell one person today — a friend, family, whoever is easiest — that it is a rough one. It weighs less once somebody else knows."), py_str("If it has been like this for weeks rather than days, the GP is the practical step: that is what they are there for, and you do not have to explain it well."), py_str("Do the small thing in front of you — the dishes, a walk, a shower. The push to do the next one comes after the first, not before.")]));
    s_A_STEP_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:so\\s+|and\\s+|ok(?:ay)?\\s+|right\\s+|well\\s+)?(?:what|where|how)\\s+(?:should|do|can|could|shall|would)\\s+(?:i|we)\\s+(?:do|start|begin|try|go|handle|sort|fix|deal)(?:\\s+(?:now|then|next|first|it|this|that|about it|about that|from here|today|tonight|at all))?\\s*[?.!]*\\s*$|^\\s*(?:so\\s+|and\\s+|ok(?:ay)?\\s+|right\\s+)?what\\s+(?:now|next)\\s*[?.!]*\\s*$|^\\s*(?:so\\s+|and\\s+|ok(?:ay)?\\s+|right\\s+)?now\\s+what\\s*[?.!]*\\s*$|^\\s*(?:so\\s+|and\\s+|ok(?:ay)?\\s+|right\\s+)?(?:what|anything)\\s+(?:helps|would\\s+help|can\\s+help|might\\s+help)\\s*[?.!]*\\s*$|^\\s*(?:so\\s+|and\\s+)?how\\s+(?:do|can|could|should)\\s+i\\s+(?:feel|get|make\\s+myself\\s+feel)\\s+better\\b[^?.!]*[?.!]*\\s*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn says_it_has_lingered(args: &[Py]) -> Py {
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
    let mut illness: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "illness") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("True when the line says the thing has been going long enough to be a different question.\n\n    A bare \"still\" is not that: it says \"not gone yet\", and how long that is the line never says.\n    Reading it as \"long enough\" answered \"im still full of cold\" with \"that is long enough to be\n    worth a proper look rather than another stretch of waiting\", which is a different claim from\n    the one the person made, and \"im still full of cold, day two\" with the same sentence about a\n    bug two days old. So the duration has to be in the words: a run of days, weeks or months, a\n    thing that will not shift, or a thing that has been dragging on. `illness` is kept for the\n    callers that already know they read a cold or a bug, and it no longer decides on its own.\n    ");
    return call_builtin("bool", &[{ let __b = mcall(&g_SAYS_IT_HAS_LINGERED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SHORT_RUN(), "search", &[clean.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn negator(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("True when the word turns the next one round: \"not\", \"didn't\", \"can't\".");
    plain = mcall(&mcall(&word.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]);
    if py_truthy(&Py::Bool(py_contains(&g_NEGATORS(), &plain.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = op_gt(&call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&plain.clone(), "endswith", &[py_str("nt")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CONTRACTED_AUX(), &slice_or_raise(&plain.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None))) } } } };
    Py::None
}
pub fn turned(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when these words turn the next one round: \"not\", \"didn't\", and also \"can t\",\n    which is how a line that lost its apostrophe splits into two tokens.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&negator(&[word.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("t"))); if !py_truthy(&__b) { __b } else { { let __b = i.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AUX(), &geti(&words.clone(), &op_sub(&i.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn turns_round(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when these words (the ones just before a cue) turn it round.\n\n    A doubt word after can't/couldn't/never/always is an intensifier, not a negation: \"I can't\n    believe I finished it\" and \"I never thought I'd be so happy\" are delight. A plain \"not\"/\"don't\"\n    before \"think\" stays a real negation (\"I don't think I'm happy\" is not a happy line), so\n    \"think\" only counts after those four.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&turned(&[words.clone()])))) {
    return Py::Bool(false);
    }
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOUBT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CANNOT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn negated(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut window: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "window") { v.clone() }
    else { Py::Int(3) };
    let mut match_: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("True when a negator stands just before the phrase: \"im not feeling great\".\n\n    A phrase that opens with a negator is a shape of its own (\"not ok\" is a feeling), so it is\n    left alone.");
    parts = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&{ let __b = parts.clone(); if !py_truthy(&__b) { __b } else { negator(&[geti(&parts.clone(), &Py::Int(0))]) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&turns_round(&[slice_or_raise(&mcall(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]), &op_neg(&window.clone()), &Py::None, &Py::None)])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn nearly(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut window: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "window") { v.clone() }
    else { Py::Int(2) };
    let mut match_: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("True when \"nearly\" or \"almost\" stands just before the phrase: \"im almost happy\".\n\n    A thing nearly done is not done: \"i nearly finished it\" is not a result to congratulate, and\n    \"im almost happy\" is not a claim to a happy line. It is left out rather than counted the\n    other way, because \"i almost won\" is not bad news either.");
    parts = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&{ let __b = parts.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NEARLY(), &geti(&parts.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&op_bitand(&call_builtin("set", &[slice_or_raise(&mcall(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]), &op_neg(&window.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_NEARLY())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn doubted(args: &[Py]) -> Py {
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
    let mut window: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "window") { v.clone() }
    else { Py::Int(4) };
    let mut i: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line only thinks, hopes or doubts what it says afterwards.\n\n    Read before a result is congratulated, never after: \"i dont think i passed\", \"i doubt i\n    passed\", \"i dont think it went well\". A doubt word alone is enough; a plain negator needs\n    one of the asking words after it, so \"i dont think i passed\" is a doubt and \"i cant believe\n    i passed\" stays a win.\n    ");
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:\"'")]), "replace", &[py_str("’"), py_str("")])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOUBTED(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_SURE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns_round(&[slice_or_raise(&plain.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&i.clone(), &window.clone())]).unwrap_or_else(|e| exc::raise(e)), &i.clone(), &Py::None)]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DOUBT(), &geti(&plain.clone(), &i.clone()))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&plain.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&i.clone(), &window.clone())]).unwrap_or_else(|e| exc::raise(e)), &i.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CANNOT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BELIEVE_IT(), &geti(&plain.clone(), &i.clone()))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&plain.clone(), &op_add(&i.clone(), &Py::Int(1))), &py_str("it"))) } } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn said_again(args: &[Py]) -> Py {
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
    let _ = py_str("The sentence with whoever it is about blanked, so two sayings of it compare equal.");
    return mcall(&g__PERSON_WORD(), "sub", &[py_str("@"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    Py::None
}
pub fn _opens_with_a_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut body: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut match_: Py = Py::None;
    let _ = py_str("True when the reply opens with a name or a place rather than a word that starts a sentence.\n\n    Only the shape of a one-word opening followed by a break (\"Bristol — nice\", \"Leeds? Good to\n    know\", \"Biscuit — good name for a dog\") is read this way. A reply that opens with an ordinary\n    word keeps the hello's own dash and is lowered as before (\"Hi Jess — most people who cannot\n    sleep\", \"Hi Jim — the things that work\").\n    ");
    match_ = mcall(&g__ONE_WORD_THEN_A_BREAK(), "match", &[mcall(&call_builtin("str", &[{ let __b = body.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g__OPENS_A_SENTENCE(), &mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "strip", &[py_str("'’")])));
    Py::None
}
pub fn greet_with(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut body: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("The answer to a line that introduced someone, with the hello in front of it.\n\n    \"hi im jess, i cant sleep\" says two things: who they are and what they need. The welcome on\n    its own answers neither (\"Nice to meet you, Jess. What can I help with?\"), and the answer on\n    its own loses the name, so the answer is what is said, with the hello in front of it: \"Hi\n    Jess — most people who cannot sleep at this hour are still wound up from the evening.\" A\n    reply that already opens with a hello is left alone, and so is an empty one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } } } }) {
    return reply.clone();
    }
    body = mcall(&reply.clone(), "strip", &[]);
    plain = mcall(&body.clone(), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&plain.clone(), "startswith", &[g__HELLO_OPENS()]); if py_truthy(&__b) { __b } else { mcall(&plain.clone(), "startswith", &[op_add(&py_str("i'm "), &mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))]) } }) {
    return body.clone();
    }
    name = mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&mcall(&g__CAPITAL_I(), "match", &[body.clone()])) {
    return op_add(&op_add(&op_add(&py_str("Hi "), &name.clone()), &py_str(" — ")), &body.clone());
    }
    if py_truthy(&_opens_with_a_name(&[body.clone()])) {
    return op_add(&op_add(&op_add(&py_str("Hi "), &name.clone()), &py_str(". ")), &body.clone());
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("Hi "), &name.clone()), &py_str(" — ")), &mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _asks_about_a_thing_nobody_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut option: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "option") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: option"))) };
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut fragment: Py = Py::None;
    let mut heard: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a pool line opens on a fragment question about a thing neither side has named.\n\n    A fragment is a question with no verb and no asking word in it (\"Long one?\", \"Again?\", \"Been\n    one of those?\"), so it points at something already on the table. If the bot asked nothing and\n    the person's line does not hold the words, there is nothing on the table and the line cannot\n    be the reply - the pool has others that can open a conversation.\n    ");
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&call_builtin("str", &[last.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "endswith", &[py_str("?")]) } }) {
    return Py::Bool(false);
    }
    match_ = mcall(&g_OPENS_ON_A_FRAGMENT_QUESTION(), "match", &[call_builtin("str", &[{ let __b = option.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    fragment = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&fragment.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g__NOT_A_FRAGMENT()) } }) {
    return Py::Bool(false);
    }
    heard = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, -1, "")); py_string(__s) }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&op_bitand(&words.clone(), &heard.clone())));
    Py::None
}
pub fn pick(args: &[Py]) -> Py {
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
    let mut seed_text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seed_text") { v.clone() }
    else { py_str("") };
    let mut backed: Py = Py::None;
    let mut c: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut o: Py = Py::None;
    let mut oldest: Py = Py::None;
    let mut position: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut standing: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut unsaid: Py = Py::None;
    let _ = py_str("One of options, never the reply just given, preferring one the bot has not said lately.\n\n    `seed_text` is the user's line when the caller has it: two different lines answered from\n    the same pool should not get the same sentence just because they are turn one of their\n    conversation. Same line, same turn, same reply, so a run can be compared.\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { Py::None };
    standing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_asks_about_a_thing_nobody_said(&[o.clone(), last.clone(), seed_text.clone()])))) {  __out.push(o.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = standing.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[standing.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[options.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    options = standing.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.history", "an_earlier_turn", &[memory.clone()])))) {
    backed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.history", "a_past_that_is_not_there", &[o.clone(), memory.clone()])))) {  __out.push(o.clone()); } } py_list(__out) })() };
    if py_truthy(&backed.clone()) {
    options = backed.clone();
    }
    }
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&o.clone(), &last.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    said = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "setdefault", &[said_again(&[mcall(&turn.clone(), "get", &[py_str("bot")])]), position.clone()]);
    }
    }
    unsaid = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said.clone(), &said_again(&[o.clone()])))) {  __out.push(o.clone()); } } py_list(__out) })() };
    if py_truthy(&unsaid.clone()) {
    fresh = unsaid.clone();
    } else {
    oldest = call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut o: Py = __item.clone(); __out.push(mcall(&said.clone(), "get", &[said_again(&[o.clone()]), op_neg(&Py::Int(1))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&said.clone(), "get", &[said_again(&[o.clone()]), op_neg(&Py::Int(1))]), &oldest.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&seed_text.clone()) {
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("str", &[seed_text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    } else {
    seed = call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[fresh.clone()]);
    Py::None
}
pub fn _name_just_given(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut log: Py = Py::None;
    let mut now: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The name the person gave in this very turn, plain, or \"\".\n\n    The memory lane records every fact with the turn it was learned on, so an entry whose turn\n    is this one is something the person has just said. This reads the store, not the words, so a\n    name mentioned in passing (\"my brother jess\") is not a person introducing themselves.\n    ");
    if py_truthy(&mcall(&memory.clone(), "get", &[py_str("aside")])) {
    return py_str("");
    }
    log = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[log.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    now = if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[log.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&entry.clone(), "get", &[py_str("turn")]), &now.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("name")))) {
    continue;
    }
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return mcall(&value.clone(), "strip", &[]);
    }
    }
    return py_str("");
    Py::None
}
pub fn with_name(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("The answer the person is owed, with the name they just gave in front of it.\n\n    \"hi im sam, how do i bleed a radiator\" has two halves and the two are answered by different\n    things: the entry has the substance and the welcome has the person. Handing the turn to the\n    welcome trades a real answer for \"nice to meet you, Sam. What can I help with?\", which fits\n    any message and tells nobody anything (STYLE.md). So the name goes in front of the answer\n    that won instead - \"Hi Sam — radiators that are hot at the bottom ...\".\n\n    The hello and the name are `greet_with`'s, not this function's own wording: the same\n    introduction reaches an answer by two roads (this one from an entry, and `chat.respond` from\n    a line that greets and says something the chat module can answer), and two roads must not give\n    a person two different greetings. \"Jess — the things that work\" and \"Hi Jess — sorry to hear\n    that\" were both live, so the same introduction was answered in two shapes.\n\n    This is called only where an entry has answered the line, so a reply coming back already\n    means there was something to answer: a name said on its own (\"hi im sam\") never gets here,\n    because the welcome takes that turn. Nothing is added either for a reply that already says\n    the name, or on a turn where no name was given.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    name = _name_just_given(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return reply.clone();
    }
    plain = mcall(&py_str(" "), "join", &[mcall(&name.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&plain.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return reply.clone();
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&plain.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("'?s?\\b")), mcall(&reply.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    return greet_with(&[mcall(&plain.clone(), "capitalize", &[]), reply.clone()]);
    Py::None
}
pub fn _says_one_of(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut flat: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("True when the bot's last line says one of `said`, dressed up or not.\n\n    A pool line rarely comes back bare: the greeting module puts the person's name in front\n    (\"Hi Eve — sorry to hear that. A walk and someone to talk to usually help more than\n    sitting with it.\") and `with_name` puts it there for an entry. Exact equality missed every\n    one of those, so `hi im eve, im feeling really low` followed by `what should i do` still got\n    the search hedge while the same pair without the name got a step. The line said is the\n    thing that matters, not whether a name was said with it, so the pool line is looked for\n    inside the reply - and case-folded, because the greeting in front of it drops the capital\n    (\"Hi Eve — sorry to hear that. A walk...\").\n    ");
    flat = mcall(&text.clone(), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); __out.push({ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &mcall(&line.clone(), "lower", &[]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn step_when_low(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut comfort_said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "comfort_said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: comfort_said"))) };
    let mut last: Py = Py::None;
    let mut reads_as_a_leaving_already_gone: Py = Py::None;
    let mut reads_as_a_reported_leaving: Py = Py::None;
    let mut reads_as_a_sacking: Py = Py::None;
    let mut reads_as_bad_news: Py = Py::None;
    let mut reads_as_the_leaving: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A next step for a bare \"what should i do\" after one of the bot's own comfort lines.\n\n    `comfort_said` is the set of lines the calling composer can say as comfort, so each\n    composer decides which of its own threads count. A step asked for anywhere else - after a\n    recipe, a verdict, a safety line - is that thread's and this returns None.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_STEP_ASK(), "match", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])))) {
    return Py::None;
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    turn = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    last = mcall(&{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_one_of(&[last.clone(), comfort_said.clone()]))) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    reads_as_a_leaving_already_gone = py_func("reads_as_a_leaving_already_gone", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_a_leaving_already_gone", args));
    reads_as_a_reported_leaving = py_func("reads_as_a_reported_leaving", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_a_reported_leaving", args));
    reads_as_a_sacking = py_func("reads_as_a_sacking", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_a_sacking", args));
    reads_as_bad_news = py_func("reads_as_bad_news", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", args));
    reads_as_the_leaving = py_func("reads_as_the_leaving", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_the_leaving", args));
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&py_call(&reads_as_a_reported_leaving, &[said.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    if py_truthy(&{ let __b = py_call(&reads_as_bad_news, &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&reads_as_the_leaving, &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&reads_as_a_leaving_already_gone, &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&reads_as_a_sacking, &[said.clone()]))) } } } } } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return pick(&[call_builtin("list", &[g_STEP_WHEN_LOW()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AUX" => Some(g_AUX()),
            "A_STEP_ASK" => Some(g_A_STEP_ASK()),
            "BELIEVE_IT" => Some(g_BELIEVE_IT()),
            "CANNOT" => Some(g_CANNOT()),
            "CONTRACTED_AUX" => Some(g_CONTRACTED_AUX()),
            "DOUBT" => Some(g_DOUBT()),
            "DOUBTED" => Some(g_DOUBTED()),
            "LINGERED_LINE" => Some(g_LINGERED_LINE()),
            "NEARLY" => Some(g_NEARLY()),
            "NEGATORS" => Some(g_NEGATORS()),
            "OPENS_ON_A_FRAGMENT_QUESTION" => Some(g_OPENS_ON_A_FRAGMENT_QUESTION()),
            "SAYS_IT_HAS_LINGERED" => Some(g_SAYS_IT_HAS_LINGERED()),
            "STEP_WHEN_LOW" => Some(g_STEP_WHEN_LOW()),
            "STILL_AT_IT" => Some(g_STILL_AT_IT()),
            "SURE" => Some(g_SURE()),
            "_CAPITAL_I" => Some(g__CAPITAL_I()),
            "_HELLO_OPENS" => Some(g__HELLO_OPENS()),
            "_NOT_A_FRAGMENT" => Some(g__NOT_A_FRAGMENT()),
            "_ONE_WORD_THEN_A_BREAK" => Some(g__ONE_WORD_THEN_A_BREAK()),
            "_OPENS_A_SENTENCE" => Some(g__OPENS_A_SENTENCE()),
            "_PERSON_WORD" => Some(g__PERSON_WORD()),
            "_SHORT_RUN" => Some(g__SHORT_RUN()),
            "_asks_about_a_thing_nobody_said" => Some(py_func("_asks_about_a_thing_nobody_said", |args: &[Py]| _asks_about_a_thing_nobody_said(args))),
            "_name_just_given" => Some(py_func("_name_just_given", |args: &[Py]| _name_just_given(args))),
            "_opens_with_a_name" => Some(py_func("_opens_with_a_name", |args: &[Py]| _opens_with_a_name(args))),
            "_says_one_of" => Some(py_func("_says_one_of", |args: &[Py]| _says_one_of(args))),
            "doubted" => Some(py_func("doubted", |args: &[Py]| doubted(args))),
            "greet_with" => Some(py_func("greet_with", |args: &[Py]| greet_with(args))),
            "nearly" => Some(py_func("nearly", |args: &[Py]| nearly(args))),
            "negated" => Some(py_func("negated", |args: &[Py]| negated(args))),
            "negator" => Some(py_func("negator", |args: &[Py]| negator(args))),
            "pick" => Some(py_func("pick", |args: &[Py]| pick(args))),
            "said_again" => Some(py_func("said_again", |args: &[Py]| said_again(args))),
            "says_it_has_lingered" => Some(py_func("says_it_has_lingered", |args: &[Py]| says_it_has_lingered(args))),
            "step_when_low" => Some(py_func("step_when_low", |args: &[Py]| step_when_low(args))),
            "turned" => Some(py_func("turned", |args: &[Py]| turned(args))),
            "turns_round" => Some(py_func("turns_round", |args: &[Py]| turns_round(args))),
            "with_name" => Some(py_func("with_name", |args: &[Py]| with_name(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AUX" => s_AUX(v),
            "A_STEP_ASK" => s_A_STEP_ASK(v),
            "BELIEVE_IT" => s_BELIEVE_IT(v),
            "CANNOT" => s_CANNOT(v),
            "CONTRACTED_AUX" => s_CONTRACTED_AUX(v),
            "DOUBT" => s_DOUBT(v),
            "DOUBTED" => s_DOUBTED(v),
            "LINGERED_LINE" => s_LINGERED_LINE(v),
            "NEARLY" => s_NEARLY(v),
            "NEGATORS" => s_NEGATORS(v),
            "OPENS_ON_A_FRAGMENT_QUESTION" => s_OPENS_ON_A_FRAGMENT_QUESTION(v),
            "SAYS_IT_HAS_LINGERED" => s_SAYS_IT_HAS_LINGERED(v),
            "STEP_WHEN_LOW" => s_STEP_WHEN_LOW(v),
            "STILL_AT_IT" => s_STILL_AT_IT(v),
            "SURE" => s_SURE(v),
            "_CAPITAL_I" => s__CAPITAL_I(v),
            "_HELLO_OPENS" => s__HELLO_OPENS(v),
            "_NOT_A_FRAGMENT" => s__NOT_A_FRAGMENT(v),
            "_ONE_WORD_THEN_A_BREAK" => s__ONE_WORD_THEN_A_BREAK(v),
            "_OPENS_A_SENTENCE" => s__OPENS_A_SENTENCE(v),
            "_PERSON_WORD" => s__PERSON_WORD(v),
            "_SHORT_RUN" => s__SHORT_RUN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose", module_obj);
    pyrt::xrefs::reg_fn("compose", "says_it_has_lingered", py_func("says_it_has_lingered", |args: &[Py]| says_it_has_lingered(args)));
    pyrt::xrefs::reg_fn("compose", "negator", py_func("negator", |args: &[Py]| negator(args)));
    pyrt::xrefs::reg_fn("compose", "turned", py_func("turned", |args: &[Py]| turned(args)));
    pyrt::xrefs::reg_fn("compose", "turns_round", py_func("turns_round", |args: &[Py]| turns_round(args)));
    pyrt::xrefs::reg_fn("compose", "negated", py_func("negated", |args: &[Py]| negated(args)));
    pyrt::xrefs::reg_fn("compose", "nearly", py_func("nearly", |args: &[Py]| nearly(args)));
    pyrt::xrefs::reg_fn("compose", "doubted", py_func("doubted", |args: &[Py]| doubted(args)));
    pyrt::xrefs::reg_fn("compose", "said_again", py_func("said_again", |args: &[Py]| said_again(args)));
    pyrt::xrefs::reg_fn("compose", "_opens_with_a_name", py_func("_opens_with_a_name", |args: &[Py]| _opens_with_a_name(args)));
    pyrt::xrefs::reg_fn("compose", "greet_with", py_func("greet_with", |args: &[Py]| greet_with(args)));
    pyrt::xrefs::reg_fn("compose", "_asks_about_a_thing_nobody_said", py_func("_asks_about_a_thing_nobody_said", |args: &[Py]| _asks_about_a_thing_nobody_said(args)));
    pyrt::xrefs::reg_fn("compose", "pick", py_func("pick", |args: &[Py]| pick(args)));
    pyrt::xrefs::reg_fn("compose", "_name_just_given", py_func("_name_just_given", |args: &[Py]| _name_just_given(args)));
    pyrt::xrefs::reg_fn("compose", "with_name", py_func("with_name", |args: &[Py]| with_name(args)));
    pyrt::xrefs::reg_fn("compose", "_says_one_of", py_func("_says_one_of", |args: &[Py]| _says_one_of(args)));
    pyrt::xrefs::reg_fn("compose", "step_when_low", py_func("step_when_low", |args: &[Py]| step_when_low(args)));
    pyrt::xrefs::reg_access("compose", "AUX", g_AUX, s_AUX);
    pyrt::xrefs::reg_access("compose", "A_STEP_ASK", g_A_STEP_ASK, s_A_STEP_ASK);
    pyrt::xrefs::reg_access("compose", "BELIEVE_IT", g_BELIEVE_IT, s_BELIEVE_IT);
    pyrt::xrefs::reg_access("compose", "CANNOT", g_CANNOT, s_CANNOT);
    pyrt::xrefs::reg_access("compose", "CONTRACTED_AUX", g_CONTRACTED_AUX, s_CONTRACTED_AUX);
    pyrt::xrefs::reg_access("compose", "DOUBT", g_DOUBT, s_DOUBT);
    pyrt::xrefs::reg_access("compose", "DOUBTED", g_DOUBTED, s_DOUBTED);
    pyrt::xrefs::reg_access("compose", "LINGERED_LINE", g_LINGERED_LINE, s_LINGERED_LINE);
    pyrt::xrefs::reg_access("compose", "NEARLY", g_NEARLY, s_NEARLY);
    pyrt::xrefs::reg_access("compose", "NEGATORS", g_NEGATORS, s_NEGATORS);
    pyrt::xrefs::reg_access("compose", "OPENS_ON_A_FRAGMENT_QUESTION", g_OPENS_ON_A_FRAGMENT_QUESTION, s_OPENS_ON_A_FRAGMENT_QUESTION);
    pyrt::xrefs::reg_access("compose", "SAYS_IT_HAS_LINGERED", g_SAYS_IT_HAS_LINGERED, s_SAYS_IT_HAS_LINGERED);
    pyrt::xrefs::reg_access("compose", "STEP_WHEN_LOW", g_STEP_WHEN_LOW, s_STEP_WHEN_LOW);
    pyrt::xrefs::reg_access("compose", "STILL_AT_IT", g_STILL_AT_IT, s_STILL_AT_IT);
    pyrt::xrefs::reg_access("compose", "SURE", g_SURE, s_SURE);
    pyrt::xrefs::reg_access("compose", "_CAPITAL_I", g__CAPITAL_I, s__CAPITAL_I);
    pyrt::xrefs::reg_access("compose", "_HELLO_OPENS", g__HELLO_OPENS, s__HELLO_OPENS);
    pyrt::xrefs::reg_access("compose", "_NOT_A_FRAGMENT", g__NOT_A_FRAGMENT, s__NOT_A_FRAGMENT);
    pyrt::xrefs::reg_access("compose", "_ONE_WORD_THEN_A_BREAK", g__ONE_WORD_THEN_A_BREAK, s__ONE_WORD_THEN_A_BREAK);
    pyrt::xrefs::reg_access("compose", "_OPENS_A_SENTENCE", g__OPENS_A_SENTENCE, s__OPENS_A_SENTENCE);
    pyrt::xrefs::reg_access("compose", "_PERSON_WORD", g__PERSON_WORD, s__PERSON_WORD);
    pyrt::xrefs::reg_access("compose", "_SHORT_RUN", g__SHORT_RUN, s__SHORT_RUN);
}

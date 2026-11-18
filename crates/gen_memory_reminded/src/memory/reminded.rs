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


thread_local! { static G_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMINDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IT_DID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_content: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_forms: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_recent: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_stem: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AGAIN() -> Py { ensure_init(); G_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); G_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPTY() -> Py { ensure_init(); G_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY(v: Py) { ensure_init(); G_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_THE_PAST() -> Py { ensure_init(); G_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_THE_PAST(v: Py) { ensure_init(); G_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_REMINDING() -> Py { ensure_init(); G_REMINDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMINDING(v: Py) { ensure_init(); G_REMINDING.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_ASKS() -> Py { ensure_init(); G_STATE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_ASKS(v: Py) { ensure_init(); G_STATE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_SAID() -> Py { ensure_init(); G_STATE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_SAID(v: Py) { ensure_init(); G_STATE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_THING() -> Py { ensure_init(); G_THEIR_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_THING(v: Py) { ensure_init(); G_THEIR_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_TAIL() -> Py { ensure_init(); G_THING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_TAIL(v: Py) { ensure_init(); G_THING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAILING() -> Py { ensure_init(); G_TRAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAILING(v: Py) { ensure_init(); G_TRAILING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IT_DID() -> Py { ensure_init(); G_WHAT_IT_DID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IT_DID(v: Py) { ensure_init(); G_WHAT_IT_DID.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_THE_THING() -> Py { ensure_init(); G_WITH_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_THE_THING(v: Py) { ensure_init(); G_WITH_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_content() -> Py { ensure_init(); G_content.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_content(v: Py) { ensure_init(); G_content.with(|c| *c.borrow_mut() = v); }
pub fn g_forms() -> Py { ensure_init(); G_forms.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_forms(v: Py) { ensure_init(); G_forms.with(|c| *c.borrow_mut() = v); }
pub fn g_recent() -> Py { ensure_init(); G_recent.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_recent(v: Py) { ensure_init(); G_recent.with(|c| *c.borrow_mut() = v); }
pub fn g_stem() -> Py { ensure_init(); G_stem.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_stem(v: Py) { ensure_init(); G_stem.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that asks to be reminded of the thing they told us about, and not about the world.\n\n\"my boiler keeps losing pressure\" then \"what was wrong with my boiler again\" is the person\ngoing back over what they said: the past tense and the \"again\" are the reminder, and the answer\nthey want is their own line. Read as a question about the world it is a fault to diagnose, so\nthe answers lane took it and handed back the flow-temperature card, and the store - which had\nthe boiler on file - was never asked.\n\nThe reading is a shape, not a phrase. What makes a line this ask is all three of:\n\n  * a past-tense ask (\"what was\", \"what did\", \"remind me what was\"),\n  * an ask about a thing's state rather than its price or its recipe (\"wrong\", \"up\", \"the\n    problem\", \"happening\", \"keep doing\"), and\n  * a thing named after \"with\" - \"with my boiler\", \"with the car\", \"with it\" - where the thing\n    is one they really did say something about earlier. A boiler they only heard the bot\n    mention is not theirs to be reminded of, and a present-tense \"what's wrong with my boiler\"\n    is a fault to diagnose, not a reminder: only the past tense and the reminding frames below\n    (with \"again\", with \"remind me\") make it about the chat.\n\nA pronoun is the thing the understanding engine resolved it to (`memory[\"understood\"][\"ref\"]`),\nwhich is how \"what was wrong with it again\" reaches the boiler and not the last line said.\n");
    s_recent(py_func("recent", |args: &[Py]| pyrt::xrefs::xcall("memory", "recent", args)));
    s_content(py_func("content", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "content", args)));
    s_forms(py_func("forms", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "forms", args)));
    s_stem(py_func("stem", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "stem", args)));
    s_WITH_THE_THING(stdlib_call("re", "compile", &[py_str("\\bwith\\s+(?:((?:my|our|the)\\s+[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,2})|(it|that|this|them|those))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAILING(call_builtin("frozenset", &[mcall(&py_str("again before earlier already then yesterday today please so far\nuntil up now one ones thing things stuff lot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_EMPTY(call_builtin("frozenset", &[mcall(&py_str("one ones thing things stuff lot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:wrong|up|(?:the\\s+)?(?:problem|matter|issue|fault|trouble|deal)|happen\\w*|going\\s+on|doing|keep\\s+doing|kept\\s+doing|said\\s+about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:keep\\w*|kept|won'?t|wont|stopped|stopping|not\\s+work\\w*|doesnt\\s+work\\w*|hurts?|hurting|sore|pain\\w*|aches?|aching|swell\\w*|swollen|stiff|broken|broke|breaks?|cracked|leak\\w*|drip\\w*|dripping|slip\\w*|slipped|jammed|stuck|went|gone|dying|dead|failed|failing|crash\\w*|dragging|drains?|drained|struggl\\w*|worr\\w*|sort\\w*|sorted|fault\\w*|wrong|issue\\w*|problem\\w*|trouble|stinks?|stink\\w*|smell\\w*|noise\\w*|noisy|squeak\\w*|rattle\\w*|vibrat\\w*|slow\\w*|weak\\w*|blocked|clogged|mould\\w*|damp)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |also |ok |okay )?(?:what|whats|which)\\s+(?:was|were|did|has|have|had)\\b|\\b(?:what|whats)\\s+(?:was|were)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:again|before|earlier|remind me|remind me again|(?:cant|can not|cannot|dont|do not) remember|what did i say|what did i tell you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMINDING(stdlib_call("re", "compile", &[py_str("\\b(?:remind me|(?:cant|can not|cannot|dont|do not) remember)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_THING(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|which)\\s+(?:was|were|is|are|did|does|has|have|had|d)\\s*((?:my|our)\\s+[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,3})\\b|\\b(?:what|whats|which)\\s+((?:my|our)\\s+[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,3})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("problem problems matter issue fault trouble deal going happening doing\nwrong up again before earlier then so far please"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_TAIL(op_bitor(&g_ASK_WORDS(), &call_builtin("frozenset", &[mcall(&py_str("was were is are be been being did does has have had"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_WHAT_IT_DID(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats)\\s+(?:did|does|has|have)\\s+((?:my|our|the)\\s+[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,2}?)\\s+(?:keep\\s+|kept\\s+)?(?:doing|do|say|saying)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _trim(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The named thing without the words that point at the remembering rather than the thing.");
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRAILING(), &mcall(&word.clone(), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return mcall(&mcall(&py_str(" "), "join", &[kept.clone()]), "strip", &[py_str(" .,!?")]);
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
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when one of their own earlier lines names this thing.\n\n    The guard is what keeps the reading to the chat: a boiler the bot mentioned, or one named\n    for the first time in the ask itself (\"my boiler is making a noise again\" / \"what was wrong\n    with my boiler again\" in the same breath), has no line of theirs to be reminded of.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[thing.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_EMPTY(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "recent", &[memory.clone(), Py::Int(12)]));
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    said = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut seen: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&word.clone(), &seen.clone())); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]), &pyrt::xrefs::xcall("memory.topics", "forms", &[seen.clone()])); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()]), &pyrt::xrefs::xcall("memory.topics", "stem", &[seen.clone()]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_thing(args: &[Py]) -> Py {
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
    let mut did: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing a reminder is about (\"my boiler\", \"the car\", \"it\"), or None when the line is\n    not a reminder about a thing they named.\n\n    The reading is shared by the gate that says the line is about the chat (the store's\n    `_about_this_chat`) and the reply that quotes the line (`_recall_reply`), so the two cannot\n    disagree about which lines are reminders.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_STATE_ASKS(), "search", &[line.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_IN_THE_PAST(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REMINDING(), "search", &[line.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AGAIN(), "search", &[line.clone()])))) {
    return Py::None;
    }
    thing = py_str("");
    named = mcall(&g_WITH_THE_THING(), "search", &[line.clone()]);
    if py_truthy(&named.clone()) {
    if py_truthy(&mcall(&named.clone(), "group", &[Py::Int(1)])) {
    thing = _trim(&[mcall(&named.clone(), "group", &[Py::Int(1)])]);
    } else {
    thing = _resolved(&[mcall(&named.clone(), "group", &[Py::Int(2)]), memory.clone()]);
    }
    } else {
    did = mcall(&g_WHAT_IT_DID(), "search", &[line.clone()]);
    if py_truthy(&did.clone()) {
    thing = _trim(&[mcall(&did.clone(), "group", &[Py::Int(1)])]);
    } else {
    theirs = mcall(&g_THEIR_THING(), "search", &[line.clone()]);
    if py_truthy(&theirs.clone()) {
    thing = _the_thing_itself(&[{ let __b = mcall(&theirs.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { mcall(&theirs.clone(), "group", &[Py::Int(2)]) } }]);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.topics", "content", &[thing.clone()]))) } }) {
    return Py::None;
    }
    return thing.clone();
    Py::None
}
pub fn _the_thing_itself(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing out of \"my bike problem\": the phrase without the words that ask about its state.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THING_TAIL(), &mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]), "lower", &[]))) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return _trim(&[mcall(&py_str(" "), "join", &[words.clone()])]);
    Py::None
}
pub fn _resolved(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pronoun: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pronoun") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pronoun"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut read: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The thing a pronoun points at, as the understanding engine read it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    ref_ = mcall(&read.clone(), "get", &[py_str("ref")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    value = mcall(&ref_.clone(), "get", &[mcall(&call_builtin("str", &[{ let __b = pronoun.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return _trim(&[value.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _said_what_was_wrong(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when one of their own lines says something happened to this thing.\n\n    The reminder names a thing and asks for its state, so the chat has to hold the state as well\n    as the name. A line that only mentions the thing in passing does not: asking \"what was my\n    bike problem again\" after \"my bike is red\" was answered `You said: \"my bike is red\"` - a\n    claim the chat does not support, since nothing was said to be wrong with the bike. With no\n    line of theirs saying so, the reading is not a reminder at all and the turn goes to whoever\n    the words belong to, which is where the ask was answered before.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[thing.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_EMPTY(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "recent", &[memory.clone(), Py::Int(12)]));
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_STATE_SAID(), "search", &[line.clone()]))) } }) {
    continue;
    }
    said = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut seen: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&word.clone(), &seen.clone())); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]), &pyrt::xrefs::xcall("memory.topics", "forms", &[seen.clone()])); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()]), &pyrt::xrefs::xcall("memory.topics", "stem", &[seen.clone()]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn about_a_thing_they_said(args: &[Py]) -> Py {
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
    let mut thing: Py = Py::None;
    let _ = py_str("True when the line is a reminder about a thing of theirs, so the chat is what it asks\n    about rather than the world.");
    let __try1 = exc::catch(|| {
    thing = the_thing(&[text.clone(), memory.clone()]);
    return py_tuple(vec![Py::Bool(true), { let __b = call_builtin("bool", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = _said_before(&[thing.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { _said_what_was_wrong(&[thing.clone(), memory.clone()]) } } } }]);
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
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.reminded".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGAIN" => Some(g_AGAIN()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "EMPTY" => Some(g_EMPTY()),
            "IN_THE_PAST" => Some(g_IN_THE_PAST()),
            "REMINDING" => Some(g_REMINDING()),
            "STATE_ASKS" => Some(g_STATE_ASKS()),
            "STATE_SAID" => Some(g_STATE_SAID()),
            "THEIR_THING" => Some(g_THEIR_THING()),
            "THING_TAIL" => Some(g_THING_TAIL()),
            "TRAILING" => Some(g_TRAILING()),
            "WHAT_IT_DID" => Some(g_WHAT_IT_DID()),
            "WITH_THE_THING" => Some(g_WITH_THE_THING()),
            "_resolved" => Some(py_func("_resolved", |args: &[Py]| _resolved(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_said_what_was_wrong" => Some(py_func("_said_what_was_wrong", |args: &[Py]| _said_what_was_wrong(args))),
            "_the_thing_itself" => Some(py_func("_the_thing_itself", |args: &[Py]| _the_thing_itself(args))),
            "_trim" => Some(py_func("_trim", |args: &[Py]| _trim(args))),
            "about_a_thing_they_said" => Some(py_func("about_a_thing_they_said", |args: &[Py]| about_a_thing_they_said(args))),
            "content" => Some(g_content()),
            "forms" => Some(g_forms()),
            "recent" => Some(g_recent()),
            "stem" => Some(g_stem()),
            "the_thing" => Some(py_func("the_thing", |args: &[Py]| the_thing(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGAIN" => s_AGAIN(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "EMPTY" => s_EMPTY(v),
            "IN_THE_PAST" => s_IN_THE_PAST(v),
            "REMINDING" => s_REMINDING(v),
            "STATE_ASKS" => s_STATE_ASKS(v),
            "STATE_SAID" => s_STATE_SAID(v),
            "THEIR_THING" => s_THEIR_THING(v),
            "THING_TAIL" => s_THING_TAIL(v),
            "TRAILING" => s_TRAILING(v),
            "WHAT_IT_DID" => s_WHAT_IT_DID(v),
            "WITH_THE_THING" => s_WITH_THE_THING(v),
            "content" => s_content(v),
            "forms" => s_forms(v),
            "recent" => s_recent(v),
            "stem" => s_stem(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.reminded", module_obj);
    pyrt::xrefs::reg_fn("memory.reminded", "_trim", py_func("_trim", |args: &[Py]| _trim(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "the_thing", py_func("the_thing", |args: &[Py]| the_thing(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "_the_thing_itself", py_func("_the_thing_itself", |args: &[Py]| _the_thing_itself(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "_resolved", py_func("_resolved", |args: &[Py]| _resolved(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "_said_what_was_wrong", py_func("_said_what_was_wrong", |args: &[Py]| _said_what_was_wrong(args)));
    pyrt::xrefs::reg_fn("memory.reminded", "about_a_thing_they_said", py_func("about_a_thing_they_said", |args: &[Py]| about_a_thing_they_said(args)));
    pyrt::xrefs::reg_access("memory.reminded", "AGAIN", g_AGAIN, s_AGAIN);
    pyrt::xrefs::reg_access("memory.reminded", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("memory.reminded", "EMPTY", g_EMPTY, s_EMPTY);
    pyrt::xrefs::reg_access("memory.reminded", "IN_THE_PAST", g_IN_THE_PAST, s_IN_THE_PAST);
    pyrt::xrefs::reg_access("memory.reminded", "REMINDING", g_REMINDING, s_REMINDING);
    pyrt::xrefs::reg_access("memory.reminded", "STATE_ASKS", g_STATE_ASKS, s_STATE_ASKS);
    pyrt::xrefs::reg_access("memory.reminded", "STATE_SAID", g_STATE_SAID, s_STATE_SAID);
    pyrt::xrefs::reg_access("memory.reminded", "THEIR_THING", g_THEIR_THING, s_THEIR_THING);
    pyrt::xrefs::reg_access("memory.reminded", "THING_TAIL", g_THING_TAIL, s_THING_TAIL);
    pyrt::xrefs::reg_access("memory.reminded", "TRAILING", g_TRAILING, s_TRAILING);
    pyrt::xrefs::reg_access("memory.reminded", "WHAT_IT_DID", g_WHAT_IT_DID, s_WHAT_IT_DID);
    pyrt::xrefs::reg_access("memory.reminded", "WITH_THE_THING", g_WITH_THE_THING, s_WITH_THE_THING);
    pyrt::xrefs::reg_access("memory.reminded", "content", g_content, s_content);
    pyrt::xrefs::reg_access("memory.reminded", "forms", g_forms, s_forms);
    pyrt::xrefs::reg_access("memory.reminded", "recent", g_recent, s_recent);
    pyrt::xrefs::reg_access("memory.reminded", "stem", g_stem, s_stem);
}

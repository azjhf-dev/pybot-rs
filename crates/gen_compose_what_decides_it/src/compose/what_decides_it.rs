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


thread_local! { static G_ANSWER_MODULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HANDOFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MORE_THAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEGATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUESTION_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.what_decides_it: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWER_MODULE() -> Py { ensure_init(); G_ANSWER_MODULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWER_MODULE(v: Py) { ensure_init(); G_ANSWER_MODULE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT() -> Py { ensure_init(); G__AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT(v: Py) { ensure_init(); G__AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__CONDITION() -> Py { ensure_init(); G__CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONDITION(v: Py) { ensure_init(); G__CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g__CORRECTION() -> Py { ensure_init(); G__CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CORRECTION(v: Py) { ensure_init(); G__CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g__HANDOFF() -> Py { ensure_init(); G__HANDOFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HANDOFF(v: Py) { ensure_init(); G__HANDOFF.with(|c| *c.borrow_mut() = v); }
pub fn g__LIMIT() -> Py { ensure_init(); G__LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIMIT(v: Py) { ensure_init(); G__LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g__MORE_THAN() -> Py { ensure_init(); G__MORE_THAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MORE_THAN(v: Py) { ensure_init(); G__MORE_THAN.with(|c| *c.borrow_mut() = v); }
pub fn g__NEGATION() -> Py { ensure_init(); G__NEGATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEGATION(v: Py) { ensure_init(); G__NEGATION.with(|c| *c.borrow_mut() = v); }
pub fn g__QUESTION_START() -> Py { ensure_init(); G__QUESTION_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUESTION_START(v: Py) { ensure_init(); G__QUESTION_START.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A qualifier after a knowledge answer: say the test the answer gave, in its own words.\n\n    i keep getting heartburn      \"An antacid settles most heartburn ... See a GP if it lasts more\n                                   than three weeks or you find food sticking on the way down.\"\n    im 34 and not overweight      the fallback: \"I'm listening - say more if it helps.\"\n\n    what should i get from the chippy    \"Fish and chips feeds a group for the least money ...\"\n    not that hungry                      the fallback: \"Got it. Is there anything you'd like a\n                                         hand with?\"\n\nThe second line is not a question and asks for nothing new. It is the person putting a fact about\nthemselves against what they were just told - an age, a weight, a negation (\"not that hungry\",\n\"i dont smoke\"), the kind of thing they have (\"its the flat pack kind with two doors\"). A friend\nanswers that by saying which part of what they said that fact actually bears on, and the answer is\nalready in the reply: the test it gave (\"if it lasts more than three weeks\", \"if you find food\nsticking\") is what decides, not the age or the weight.\n\nThis reader quotes that test back and nothing else. It never invents a rule, never says whether the\nperson's fact matters - when the qualifier and the test share a content word (`i dont smoke` against\n`if you smoke, take extra care`) the two are in contact and the reply stops being the bot's to give,\nso the reading stands down and the line goes where it went before. The bar for it is the same one\n`bot/compose/kept_nearby.py` keeps: name the kind, stay inside what the bot itself said.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it.ANSWER_MODULE"); }
    s_ANSWER_MODULE(py_str("answers"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._CONDITION"); }
    s__CONDITION(stdlib_call("re", "compile", &[py_str("\\b(if|unless|until|once|before|after|whenever|as long as)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._HANDOFF"); }
    s__HANDOFF(stdlib_call("re", "compile", &[py_str("\\b(gp|doctor|nurse|pharmacist|chemist|dentist|vet|council|landlord|seller|retailer|shop|airline|insurer|bank|999|111|nhs|advice|adviser|advisor|helpline|ombudsman|employer|school)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._LIMIT"); }
    s__LIMIT(stdlib_call("re", "compile", &[py_str("\\b(\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve|fourteen|twenty)\\s+(day|days|week|weeks|month|months|year|years)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._MORE_THAN"); }
    s__MORE_THAN(stdlib_call("re", "compile", &[py_str("\\b(more than|over|longer than|up to|at least)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._NEGATION"); }
    s__NEGATION(stdlib_call("re", "compile", &[py_str("\\b(not|no|none|never|dont|doesnt|didnt|isnt|arent|wasnt|cant|cannot|without|nobody|nothing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._AMOUNT"); }
    s__AMOUNT(stdlib_call("re", "compile", &[py_str("\\b(\\d+|one|two|three|four|five|six|seven|eight|nine|ten)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("what"), py_str("how"), py_str("why"), py_str("when"), py_str("where"), py_str("who"), py_str("whom"), py_str("whose"), py_str("which"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("shall"), py_str("will"), py_str("do"), py_str("does"), py_str("did"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("have"), py_str("has"), py_str("had"), py_str("may"), py_str("might"), py_str("must"), py_str("any"), py_str("anything"), py_str("isnt"), py_str("arent"), py_str("dont"), py_str("doesnt"), py_str("cant"), py_str("wont"), py_str("whats"), py_str("hows"), py_str("wheres"), py_str("whys")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._QUESTION_START"); }
    s__QUESTION_START(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it._CORRECTION"); }
    s__CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(i meant|i mean|i said|i wasnt saying|im talking about|i was talking about|the other one|not my|not the)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it.LEAD"); }
    s_LEAD(py_tuple(vec![py_str("Right."), py_str("Got it."), py_str("Fair enough."), py_str("Okay."), py_str("Right then.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it.POOL"); }
    s_POOL(py_tuple(vec![py_str("{lead} The test does not turn on that: {clause}."), py_str("{lead} Same test either way: {clause}."), py_str("{lead} That leaves the test where it was: {clause}."), py_str("{lead} The bit that decides it is unchanged: {clause}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.what_decides_it.STOP"); }
    s_STOP(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if then than that this these those it its it's is are was were be been being\ni im i'm me my you your we our they their he she his her them us of in on at to for from with\nabout into over under as so not no do does did done have has had can could should would will\njust very really quite too also more most much many some any thing things kind sort lot bit\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _turns(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _clean(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn _answer_above(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own last reply, when it was a knowledge answer.");
    turns = _turns(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWER_MODULE()))) {
    return py_str("");
    }
    return mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn _is_a_qualifier(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("A short statement about themselves, not a question and not a new subject.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    words = mcall(&clean.clone(), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g__QUESTION_START(), &mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(",.")])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__CORRECTION(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__NEGATION(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__AMOUNT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("kind"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("sort of"))) } } } } } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), _clean(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str("'")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STOP(), &word.clone())) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "add", &[if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() }]);
    }
    return out.clone();
    Py::None
}
pub fn _sentences(args: &[Py]) -> Py {
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
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    parts = stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn _is_a_test(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut lowered: Py = Py::None;
    lowered = op_add(&op_add(&py_str(" "), &mcall(&sentence.clone(), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__CONDITION(), "search", &[lowered.clone()]))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__HANDOFF(), "search", &[lowered.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__LIMIT(), "search", &[lowered.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__MORE_THAN(), "search", &[lowered.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__LIMIT(), "search", &[lowered.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_test(args: &[Py]) -> Py {
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
    let mut comma: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer's own deciding sentence: the last one that carries a condition.");
    found = py_str("");
    let __iter1 = py_iter_or_raise(&_sentences(&[answer.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&_is_a_test(&[sentence.clone()])) {
    found = sentence.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    words = mcall(&mcall(&found.clone(), "rstrip", &[py_str(" .;:,")]), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(34))) {
    cut = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(34), &Py::None)]);
    comma = mcall(&cut.clone(), "rfind", &[py_str(",")]);
    words = mcall(&if py_truthy(&op_gt(&comma.clone(), &Py::Int(60))) { slice_or_raise(&cut.clone(), &Py::None, &comma.clone(), &Py::None) } else { cut.clone() }, "split", &[]);
    }
    found = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "rstrip", &[py_str(" ,;:")]);
    first = geti(&mcall(&found.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&first.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&first.clone(), &Py::Int(1), &Py::None, &Py::None), "islower", &[]) } }) {
    found = op_add(&mcall(&geti(&found.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&found.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return found.clone();
    Py::None
}
pub fn _in_contact(args: &[Py]) -> Py {
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
    let mut test: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "test") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: test"))) };
    let _ = py_str("True when the person's own words bear on the test: then the reply is not ours to give.");
    return call_builtin("bool", &[op_bitand(&_content(&[clean.clone()]), &_content(&[test.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut c: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("One line naming the test the answer above gave, or None when the line is not a qualifier.");
    clean = _clean(&[text.clone()]);
    answer = _answer_above(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_a_qualifier(&[clean.clone()]))) } }) {
    return Py::None;
    }
    test = _the_test(&[answer.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&test.clone())); if py_truthy(&__b) { __b } else { _in_contact(&[clean.clone(), test.clone()]) } }) {
    return Py::None;
    }
    lead = geti(&g_LEAD(), &op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_LEAD()]).unwrap_or_else(|e| exc::raise(e))));
    line = geti(&g_POOL(), &op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&clean.clone(), &test.clone())) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_POOL()]).unwrap_or_else(|e| exc::raise(e))));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("lead".to_string(), lead.clone()), ("clause".to_string(), test.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.what_decides_it".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWER_MODULE" => Some(g_ANSWER_MODULE()),
            "LEAD" => Some(g_LEAD()),
            "POOL" => Some(g_POOL()),
            "STOP" => Some(g_STOP()),
            "_AMOUNT" => Some(g__AMOUNT()),
            "_CONDITION" => Some(g__CONDITION()),
            "_CORRECTION" => Some(g__CORRECTION()),
            "_HANDOFF" => Some(g__HANDOFF()),
            "_LIMIT" => Some(g__LIMIT()),
            "_MORE_THAN" => Some(g__MORE_THAN()),
            "_NEGATION" => Some(g__NEGATION()),
            "_QUESTION_START" => Some(g__QUESTION_START()),
            "_answer_above" => Some(py_func("_answer_above", |args: &[Py]| _answer_above(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_in_contact" => Some(py_func("_in_contact", |args: &[Py]| _in_contact(args))),
            "_is_a_qualifier" => Some(py_func("_is_a_qualifier", |args: &[Py]| _is_a_qualifier(args))),
            "_is_a_test" => Some(py_func("_is_a_test", |args: &[Py]| _is_a_test(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_the_test" => Some(py_func("_the_test", |args: &[Py]| _the_test(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWER_MODULE" => s_ANSWER_MODULE(v),
            "LEAD" => s_LEAD(v),
            "POOL" => s_POOL(v),
            "STOP" => s_STOP(v),
            "_AMOUNT" => s__AMOUNT(v),
            "_CONDITION" => s__CONDITION(v),
            "_CORRECTION" => s__CORRECTION(v),
            "_HANDOFF" => s__HANDOFF(v),
            "_LIMIT" => s__LIMIT(v),
            "_MORE_THAN" => s__MORE_THAN(v),
            "_NEGATION" => s__NEGATION(v),
            "_QUESTION_START" => s__QUESTION_START(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.what_decides_it", module_obj);
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_answer_above", py_func("_answer_above", |args: &[Py]| _answer_above(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_is_a_qualifier", py_func("_is_a_qualifier", |args: &[Py]| _is_a_qualifier(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_is_a_test", py_func("_is_a_test", |args: &[Py]| _is_a_test(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_the_test", py_func("_the_test", |args: &[Py]| _the_test(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "_in_contact", py_func("_in_contact", |args: &[Py]| _in_contact(args)));
    pyrt::xrefs::reg_fn("compose.what_decides_it", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.what_decides_it", "ANSWER_MODULE", g_ANSWER_MODULE, s_ANSWER_MODULE);
    pyrt::xrefs::reg_access("compose.what_decides_it", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("compose.what_decides_it", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("compose.what_decides_it", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_AMOUNT", g__AMOUNT, s__AMOUNT);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_CONDITION", g__CONDITION, s__CONDITION);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_CORRECTION", g__CORRECTION, s__CORRECTION);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_HANDOFF", g__HANDOFF, s__HANDOFF);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_LIMIT", g__LIMIT, s__LIMIT);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_MORE_THAN", g__MORE_THAN, s__MORE_THAN);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_NEGATION", g__NEGATION, s__NEGATION);
    pyrt::xrefs::reg_access("compose.what_decides_it", "_QUESTION_START", g__QUESTION_START, s__QUESTION_START);
}

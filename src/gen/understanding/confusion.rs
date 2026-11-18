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


pub static CONFUSION_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_BY_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_CANNOT_FOLLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_CUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_HARD_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_HEAD_NOR_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_HEAD_SPINNING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_HELPER_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_MAKES_NO_SENSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_POINTS_AT_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_SAID_OF_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONFUSION_TRAILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_CONFUSION_A_THING() -> Py { ensure_init(); CONFUSION_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_A_THING.write() { *w = v; } }
pub fn g_CONFUSION_BY_A_THING() -> Py { ensure_init(); CONFUSION_BY_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_BY_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_BY_A_THING.write() { *w = v; } }
pub fn g_CONFUSION_CANNOT_FOLLOW() -> Py { ensure_init(); CONFUSION_CANNOT_FOLLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_CANNOT_FOLLOW(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_CANNOT_FOLLOW.write() { *w = v; } }
pub fn g_CONFUSION_CUT() -> Py { ensure_init(); CONFUSION_CUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_CUT(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_CUT.write() { *w = v; } }
pub fn g_CONFUSION_HARD_TO() -> Py { ensure_init(); CONFUSION_HARD_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_HARD_TO(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_HARD_TO.write() { *w = v; } }
pub fn g_CONFUSION_HEAD_NOR_TAIL() -> Py { ensure_init(); CONFUSION_HEAD_NOR_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_HEAD_NOR_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_HEAD_NOR_TAIL.write() { *w = v; } }
pub fn g_CONFUSION_HEAD_SPINNING() -> Py { ensure_init(); CONFUSION_HEAD_SPINNING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_HEAD_SPINNING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_HEAD_SPINNING.write() { *w = v; } }
pub fn g_CONFUSION_HELPER_NOT() -> Py { ensure_init(); CONFUSION_HELPER_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_HELPER_NOT(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_HELPER_NOT.write() { *w = v; } }
pub fn g_CONFUSION_MAKES_NO_SENSE() -> Py { ensure_init(); CONFUSION_MAKES_NO_SENSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_MAKES_NO_SENSE(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_MAKES_NO_SENSE.write() { *w = v; } }
pub fn g_CONFUSION_POINTS_AT_NOTHING() -> Py { ensure_init(); CONFUSION_POINTS_AT_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_POINTS_AT_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_POINTS_AT_NOTHING.write() { *w = v; } }
pub fn g_CONFUSION_SAID_OF_A_THING() -> Py { ensure_init(); CONFUSION_SAID_OF_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_SAID_OF_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_SAID_OF_A_THING.write() { *w = v; } }
pub fn g_CONFUSION_TRAILING() -> Py { ensure_init(); CONFUSION_TRAILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONFUSION_TRAILING(v: Py) { ensure_init(); if let Ok(mut w) = CONFUSION_TRAILING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The thing a line says it cannot follow: \"the instructions are confusing\".\n\nRead once per turn for every module, from `bot/understanding/__init__.py`. It is a reading of its\nown with its own words and shapes - which thing the line names, and how it says it - so it lives\nhere rather than at the end of kinds.py.\n");
    s_CONFUSION_A_THING(py_str("(?:the|this|that|these|those|my|our|your|his|her|their|its|a|an)\\s+(?:[a-z'-]+\\s+){0,8}?[a-z'-]+"));
    s_CONFUSION_CUT(call_builtin("frozenset", &[mcall(&py_str("\nof for with and in at to from about by on that which but so because when if as while\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_TRAILING(call_builtin("frozenset", &[mcall(&py_str("\nthe a an it its is are was were this these those my our me i we you\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_SAID_OF_A_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")\\s+(?:is|are|was|were|looks?|seems?|reads?|sounds?)\\s+(?:(?:so|really|very|totally|just|completely|properly|bloody|dead|too|all)\\s+)?(?P<cue>confusing|baffling|impossible|incomprehensible|unreadable|a nightmare|double dutch|gobbledygook)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_POINTS_AT_NOTHING(stdlib_call("re", "compile", &[py_str("\\b(?:this|that|it|all this|all of this|everything|the whole thing)\\s+(?:is|are|was)\\s+(?:(?:so|really|very|totally|just|completely|properly)\\s+)?(?P<cue>confusing|baffling|impossible|incomprehensible|a nightmare|double dutch)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_CANNOT_FOLLOW(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|we)\\s+(?:just\\s+|still\\s+|really\\s+|honestly\\s+|literally\\s+|even\\s+)?(?:can not|cannot|cant|could not|couldnt|do not|dont|did not|didnt)\\s+(?P<cue>follow|understand|make\\s+sense\\s+of|figure\\s+out|work\\s+out|get)\\s+(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_BY_A_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i am|im|i feel|i felt|i was|i get|i got|we are|we got|were|feeling)\\s+(?:(?:so|really|totally|completely|properly|just|a bit)\\s+)?(?P<cue>baffled|lost|stumped|puzzled|confused|flummoxed)\\s+(?:by|with|on|at)\\s+(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_HARD_TO(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")\\s+(?:is|are|was|were)\\s+(?:(?:so|really|very|too|just|almost)\\s+)?(?:impossible|hard|difficult|tough|not\\s+easy|awkward)\\s+to\\s+(?P<cue>read|follow|understand|fill\\s+in|make\\s+sense\\s+of|work\\s+out|get\\s+through)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_HEAD_NOR_TAIL(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|we)\\s+(?:(?:just|still|really)\\s+)?(?:can not|cannot|cant|could not|couldnt|do not|dont|did not|didnt)\\s+(?P<cue>make|tell)\\s+(?:head|heads)\\s+(?:nor|or)\\s+tail\\s+of\\s+(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_MAKES_NO_SENSE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?P<thing>"), &g_CONFUSION_A_THING()), &py_str(")\\s+(?P<cue>makes|made|does not make|doesnt make|did not make|didnt make)\\s+(?:any\\s+|no\\s+|much\\s+)?sense\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_HEAD_SPINNING(stdlib_call("re", "compile", &[add(&add(&py_str("\\bmy\\s+head\\s+(?:is|was|'s)\\s+(?:spinning|swimming|in a spin|all over the place)\\b\\s+(?:with|from|over)\\s+(?P<thing>(?:(?:all|both)\\s+(?:of\\s+)?)?"), &g_CONFUSION_A_THING()), &py_str("|all\\s+(?:this|that|it)|everything|it)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONFUSION_HELPER_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:can\\s+not|cannot|do\\s+not|dont|does\\s+not|doesnt|did\\s+not|didnt|could\\s+not|couldnt|would\\s+not|wouldnt|will\\s+not|wont|shall\\s+not|should\\s+not|shouldnt|is\\s+not|isnt|are\\s+not|arent|was\\s+not|wasnt|were\\s+not|werent|have\\s+not|havent|has\\s+not|hasnt|had\\s+not|hadnt|must\\s+not)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_confusion_ruled_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before_cue: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_cue"))) };
    let _ = py_str("True when the words in front of the cue rule it out (\"im not confused about the manual\").");
    return call_builtin("bool", &[mcall(&crate::gen::understanding::kinds::g_DENY_BEFORE(), "search", &[mcall(&g_CONFUSION_HELPER_NOT(), "sub", &[py_str(""), before_cue.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thing_trimmed(args: &[Py]) -> Py {
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
    let _ = py_str("The thing itself: up to where it is joined to something else, then trimmed of little words.\n\n    \"the instructions for the new boiler\" is the instructions, and \"this\" or \"it\" on its own names\n    nothing to ask about.\n    ");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_CONFUSION_CUT(), &word.clone()))) {
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CONFUSION_TRAILING(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_CONFUSION_TRAILING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn confusion_of(args: &[Py]) -> Py {
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
    let mut before_cue: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut part: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the line says it cannot follow, as {\"shape\", \"thing\"}, or None.\n\n    `shape` says how the line said it - `said_of_a_thing` (\"the instructions are confusing\"),\n    `cannot_follow` (\"i cant follow the recipe\"), `by_a_thing` (\"im baffled by my payslip\"),\n    `head_spinning` (\"my head is spinning with all this\") or `nothing_named` (\"this is so\n    confusing\") - and `thing` is the thing itself, or None when the line named none.\n\n    None for a line that rules it out (\"im not confused\", \"the instructions are not confusing\"),\n    for someone else's trouble (\"my son cant follow the instructions\"), and for every line with no\n    thing that will not go in, which is what keeps \"i dont understand why shes angry\" (a question\n    about her) and \"im confused\" (the state, nothing named) where they were.\n    ");
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = mcall(&line.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("|")])) { let mut part: Py = __item.clone(); __out.push(mcall(&part.clone(), "strip", &[])); } py_list(__out) })() });
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    found = mcall(&g_CONFUSION_POINTS_AT_NOTHING(), "search", &[clause.clone()]);
    if py_truthy(&found.clone()) {
    return py_dict(vec![(py_str("shape"), py_str("nothing_named")), (py_str("thing"), Py::None)]);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_CONFUSION_HARD_TO(), py_str("hard_to")]), py_tuple(vec![g_CONFUSION_SAID_OF_A_THING(), py_str("said_of_a_thing")]), py_tuple(vec![g_CONFUSION_MAKES_NO_SENSE(), py_str("said_of_a_thing")]), py_tuple(vec![g_CONFUSION_CANNOT_FOLLOW(), py_str("cannot_follow")]), py_tuple(vec![g_CONFUSION_HEAD_NOR_TAIL(), py_str("cannot_follow")]), py_tuple(vec![g_CONFUSION_BY_A_THING(), py_str("by_a_thing")]), py_tuple(vec![g_CONFUSION_HEAD_SPINNING(), py_str("head_spinning")])]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk3[0].clone();
    shape = __unpk3[1].clone();
    found = mcall(&pattern.clone(), "search", &[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    before_cue = if py_truthy(&Py::Bool(py_contains(&mcall(&found.clone(), "groupdict", &[]), &py_str("cue")))) { slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[py_str("cue")]), &Py::None) } else { slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None) };
    if py_truthy(&_a_confusion_ruled_out(&[before_cue.clone()])) {
    continue;
    }
    thing = _a_thing_trimmed(&[mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("thing")])]);
    return py_dict(vec![(py_str("shape"), shape.clone()), (py_str("thing"), thing.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.confusion".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CONFUSION_A_THING" => Some(g_CONFUSION_A_THING()),
            "CONFUSION_BY_A_THING" => Some(g_CONFUSION_BY_A_THING()),
            "CONFUSION_CANNOT_FOLLOW" => Some(g_CONFUSION_CANNOT_FOLLOW()),
            "CONFUSION_CUT" => Some(g_CONFUSION_CUT()),
            "CONFUSION_HARD_TO" => Some(g_CONFUSION_HARD_TO()),
            "CONFUSION_HEAD_NOR_TAIL" => Some(g_CONFUSION_HEAD_NOR_TAIL()),
            "CONFUSION_HEAD_SPINNING" => Some(g_CONFUSION_HEAD_SPINNING()),
            "CONFUSION_HELPER_NOT" => Some(g_CONFUSION_HELPER_NOT()),
            "CONFUSION_MAKES_NO_SENSE" => Some(g_CONFUSION_MAKES_NO_SENSE()),
            "CONFUSION_POINTS_AT_NOTHING" => Some(g_CONFUSION_POINTS_AT_NOTHING()),
            "CONFUSION_SAID_OF_A_THING" => Some(g_CONFUSION_SAID_OF_A_THING()),
            "CONFUSION_TRAILING" => Some(g_CONFUSION_TRAILING()),
            "_a_confusion_ruled_out" => Some(py_func("_a_confusion_ruled_out", |args: &[Py]| _a_confusion_ruled_out(args))),
            "_a_thing_trimmed" => Some(py_func("_a_thing_trimmed", |args: &[Py]| _a_thing_trimmed(args))),
            "confusion_of" => Some(py_func("confusion_of", |args: &[Py]| confusion_of(args))),
            _ => None,
        }),
        set: None,
    }))
}

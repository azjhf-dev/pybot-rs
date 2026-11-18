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


pub static A_LIMIT_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STATE_IS_NOT_A_LIMIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_REFERENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTINUES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEPT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAST_KEPT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAST_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSED_LIMIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_LEFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_THAT_LIMITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_LAUGH_OR_SCARES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESTATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_LIMIT_SHAPE() -> Py { ensure_init(); A_LIMIT_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = A_LIMIT_SHAPE.write() { *w = v; } }
pub fn g_A_STATE_IS_NOT_A_LIMIT() -> Py { ensure_init(); A_STATE_IS_NOT_A_LIMIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STATE_IS_NOT_A_LIMIT(v: Py) { ensure_init(); if let Ok(mut w) = A_STATE_IS_NOT_A_LIMIT.write() { *w = v; } }
pub fn g_BACK_REFERENCE() -> Py { ensure_init(); BACK_REFERENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_REFERENCE(v: Py) { ensure_init(); if let Ok(mut w) = BACK_REFERENCE.write() { *w = v; } }
pub fn g_CONTINUES() -> Py { ensure_init(); CONTINUES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTINUES(v: Py) { ensure_init(); if let Ok(mut w) = CONTINUES.write() { *w = v; } }
pub fn g_COUNTED() -> Py { ensure_init(); COUNTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTED(v: Py) { ensure_init(); if let Ok(mut w) = COUNTED.write() { *w = v; } }
pub fn g_FRAME() -> Py { ensure_init(); FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = FRAME.write() { *w = v; } }
pub fn g_KEPT() -> Py { ensure_init(); KEPT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEPT(v: Py) { ensure_init(); if let Ok(mut w) = KEPT.write() { *w = v; } }
pub fn g_LEAST_KEPT() -> Py { ensure_init(); LEAST_KEPT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAST_KEPT(v: Py) { ensure_init(); if let Ok(mut w) = LEAST_KEPT.write() { *w = v; } }
pub fn g_LEAST_WORDS() -> Py { ensure_init(); LEAST_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAST_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LEAST_WORDS.write() { *w = v; } }
pub fn g_MISSED_LIMIT() -> Py { ensure_init(); MISSED_LIMIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSED_LIMIT(v: Py) { ensure_init(); if let Ok(mut w) = MISSED_LIMIT.write() { *w = v; } }
pub fn g_NOTHING_LEFT() -> Py { ensure_init(); NOTHING_LEFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_LEFT(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_LEFT.write() { *w = v; } }
pub fn g_NOTHING_THAT_LIMITS() -> Py { ensure_init(); NOTHING_THAT_LIMITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_THAT_LIMITS(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_THAT_LIMITS.write() { *w = v; } }
pub fn g_NO_LAUGH_OR_SCARES() -> Py { ensure_init(); NO_LAUGH_OR_SCARES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_LAUGH_OR_SCARES(v: Py) { ensure_init(); if let Ok(mut w) = NO_LAUGH_OR_SCARES.write() { *w = v; } }
pub fn g_PART() -> Py { ensure_init(); PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PART(v: Py) { ensure_init(); if let Ok(mut w) = PART.write() { *w = v; } }
pub fn g_RESTATED() -> Py { ensure_init(); RESTATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESTATED(v: Py) { ensure_init(); if let Ok(mut w) = RESTATED.write() { *w = v; } }
pub fn g_WORD() -> Py { ensure_init(); WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WORD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A line that only rules something out is answered inside the answer before it.\n\n\"recommend a film\" / \"no horror\" and \"training the lead\" / \"no treats\" are follow-ups, not new\nsubjects. Left to the modules, the word after \"no\" is a topic word like any other and the bot\nanswers the thing the person has just taken away (a horror list after \"no horror\", the treat\nagain after \"no treats\"). The understanding engine reads the line as a limit on the last\nanswer (`memory[\"understood\"][\"excludes\"]`, `[\"constraint\"]`); this composer turns that reading\ninto a reply.\n\nThe rule, in one line: keep the bot's own last answer, take out the parts of it that carry the\nthing they ruled out, and say what is left. Nothing here knows what a genre, a food or a dog\ntreat is. It works on the words of the answer above, so the same code answers a horror film, a\njar of garlic and a pocket of treats.\n\nIt only speaks when the answer above actually carries the thing: where it never mentioned it\nthere is nothing to take out, and the answer that was already given (or the next module) is\nleft alone rather than replaced with a remark about itself.\n");
    s_PART(stdlib_call("re", "compile", &[py_str("(?<=[.?!;:,])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTINUES(stdlib_call("re", "compile", &[py_str("^(?:and|or|then|but|plus|so|because|with|also|instead|though|which|that|to|for|in|on|at|of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED(stdlib_call("re", "compile", &[py_str("\\b(?:all three|all four|all five|all of them|both of them|both|either|any of those|any of them|none of them|three of|the three)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']*[a-z][a-z0-9']*")]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME(stdlib_call("re", "compile", &[py_str("^(?:same as before|same answer(?:, and it covers this)?|as i said|still|that is still the thing to do|nothing new to add on it|nothing more on it(?: i'm afraid)?|that is all i have on it|there is nothing further to add|same again for|it is the same for|same for|the same for|it is the same\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_REFERENCE(stdlib_call("re", "compile", &[py_str("^(?:the same|this|that|these|those|it|its|it's|they|them|their|both|either|which|such|the latter|the former)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEPT(py_tuple(vec![py_str("Right — no {thing}, then. {kept}"), py_str("Then leave the {thing} out: {kept}"), py_str("No {thing}, then — {kept}"), py_str("Fair enough, no {thing}: {kept}")]));
    s_NOTHING_LEFT(py_tuple(vec![py_str("Right — no {thing}, then, and that changes the answer: what would you use instead?"), py_str("With no {thing} in it there is not much of that answer left — what would you rather use?"), py_str("Then {thing} out changes it. What should it be instead?")]));
    s_LEAST_KEPT(Py::Int(20));
    s_LEAST_WORDS(Py::Int(4));
    s_RESTATED(stdlib_call("re", "compile", &[py_str("^(?:right\\s*[-\\u2014]\\s*no\\s+\\S[^.]{0,40},\\s*then[.\\s,:\\u2014-]|no\\s+\\S[^.]{0,40},\\s*then\\s*[-\\u2014]|then leave the\\s+\\S[^.]{0,40}\\s+out[:,]|fair enough, no\\s+\\S[^.]{0,40}:|with no\\s+\\S[^.]{0,40}\\s+in it there is not much|then\\s+\\S[^.]{0,30}\\s+out changes it)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STATE_IS_NOT_A_LIMIT(stdlib_call("re", "compile", &[py_str("^(?:feel|feels|felt|feeling|is|are|was|were|be|been|being|seems|seem|seemed|i|im|i'm|ive|i've|me|my|we|we're|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_THAT_LIMITS(stdlib_call("re", "compile", &[py_str("\\bnothing\\s+(?:with|that|too|but|except|without|involving|containing|featuring|before|after|under|over|like)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_LAUGH_OR_SCARES(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|without|except|minus)\\s+(?:a\\s+|the\\s+|any\\s+|too\\s+|more\\s+)?(?:laugh\\s?track|laughter|laughing|comedy|comedies|jokes?|funny|scares?|scary|horror|gore|violence|subtitles|sad\\s+ones?|sad\\s+films?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIMIT_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:nothing|no|not|without|except|minus)\\s+(?:with|that|too|more|any|much|a|the)?\\s*\\S")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED_LIMIT(py_tuple(vec![py_str("Leave that out, then - which of those was nearest, and I'll find another in the same vein."), py_str("Fair enough, that one's out. Tell me which of those you'd keep and I'll follow the thread."), py_str("Then it's out. Which of them came closest, and what would you change about it?")]));
}
pub fn restates_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut bot_line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bot_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bot_line"))) };
    let _ = py_str("True when this bot line is one of ours: the answer above, said again with a limit out.");
    return call_builtin("bool", &[mcall(&g_RESTATED(), "match", &[mcall(&call_builtin("str", &[{ let __b = bot_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn forms_of(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word forms of a ruled-out thing that count as the same thing in the bot's own line.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&thing.clone(), "lower", &[])])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_tuple(vec![]);
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    let _ = mcall(&out.clone(), "add", &[word.clone()]);
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None);
    let _ = mcall(&out.clone(), "update", &[py_set(vec![stem.clone(), add(&stem.clone(), &py_str("e"))])]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&stem.clone(), &neg(&Py::Int(1))), &geti(&stem.clone(), &neg(&Py::Int(2))))) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&stem.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ies")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&out.clone(), "add", &[add(&slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None), &py_str("y"))]);
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]);
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    } else {
    let _ = mcall(&out.clone(), "add", &[add(&word.clone(), &py_str("s"))]);
    let _ = mcall(&out.clone(), "add", &[add(&word.clone(), &py_str("es"))]);
    }
    }
    }
    }
    return call_builtin("tuple", &[call_builtin("sorted", &[out.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _carries(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut form: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when this part of the bot's own answer is about the thing ruled out.");
    said = call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&part.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &form.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_answer(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own last line, or \"\" when there is not one. A safety line is never edited.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("safety"), py_str("loss"), py_str("badnews")]), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("One line from a pool, never the line the bot has just said.");
    last = _last_answer(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    line = __item.clone();
    head = mcall(&geti(&mcall(&line.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&last.clone(), &head.clone())) } } } }) {
    continue;
    }
    return line.clone();
    }
    return geti(&pool.clone(), &Py::Int(0));
    Py::None
}
pub fn _take_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut _dropped: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut left: Py = Py::None;
    let _ = py_str("The answer above with the parts about the thing taken out, or \"\" when enough is left.");
    let __unpk1 = py_unpack(&_the_parts_left(&[said.clone(), forms.clone()]), 2, None);
    kept = __unpk1[0].clone();
    _dropped = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return py_str("");
    }
    left = _joined(&[kept.clone()]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LEAST_KEPT()); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&left.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_LEAST_WORDS()) } }) {
    return py_str("");
    }
    return left.clone();
    Py::None
}
pub fn steps_left(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _dropped: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let _ = py_str("The parts of the answer above that the limit leaves, in the order they were given.\n\n    The reply is the whole of them, so the turn after it - \"whats the first thing to change\" - is\n    the first of them rather than a new search (`bot/memory/limit.py`, `move_after_a_limit`).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("constraint")]))) } }) {
    return py_list(vec![]);
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("excludes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut thing: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    said = _last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&things.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return py_list(vec![]);
    }
    let __unpk1 = py_unpack(&_the_parts_left(&[said.clone(), forms_of(&[geti(&things.clone(), &Py::Int(0))])]), 2, None);
    kept = __unpk1[0].clone();
    _dropped = __unpk1[1].clone();
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut part: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&part.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&part.clone(), "strip", &[]), "endswith", &[py_str(":")]))) } }) {  __out.push(mcall(&mcall(&part.clone(), "strip", &[]), "rstrip", &[py_str(".,;:")])); } } py_list(__out) })() };
    Py::None
}
pub fn _the_parts_left(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut after_a_colon: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("(the parts of the answer that survive the limit, how many were taken out).");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PART(), "split", &[said.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(part.clone()); } } py_list(__out) })() };
    kept = py_list(vec![]);
    dropped = Py::Int(0);
    after_a_colon = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_FRAME(), "match", &[mcall(&part.clone(), "strip", &[])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_carries(&[part.clone(), forms.clone()]))) } } } }) {
    let __aug2 = add(&dropped.clone(), &Py::Int(1));
    dropped = __aug2;
    continue;
    }
    if py_truthy(&{ let __b = _carries(&[part.clone(), forms.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = dropped.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_CONTINUES(), "match", &[part.clone()]) } }; if py_truthy(&__b) { __b } else { after_a_colon.clone() } } } }) {
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&geti(&kept.clone(), &neg(&Py::Int(1))), "rstrip", &[]), "endswith", &[py_str(":")]) } }) {
    let _ = mcall(&kept.clone(), "pop", &[]);
    }
    after_a_colon = mcall(&mcall(&part.clone(), "rstrip", &[]), "endswith", &[py_str(":")]);
    let __aug3 = add(&dropped.clone(), &Py::Int(1));
    dropped = __aug3;
    continue;
    }
    after_a_colon = Py::Bool(false);
    if py_truthy(&{ let __b = dropped.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_COUNTED(), "search", &[part.clone()]) } }) {
    let __aug4 = add(&dropped.clone(), &Py::Int(1));
    dropped = __aug4;
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[mcall(&part.clone(), "strip", &[])]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&dropped.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    return py_tuple(vec![py_list(vec![]), dropped.clone()]);
    }
    if py_truthy(&mcall(&g_BACK_REFERENCE(), "match", &[geti(&kept.clone(), &Py::Int(0))])) {
    return py_tuple(vec![py_list(vec![]), dropped.clone()]);
    }
    return py_tuple(vec![kept.clone(), dropped.clone()]);
    Py::None
}
pub fn _joined(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kept: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut left: Py = Py::None;
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut starts_a_sentence: Py = Py::None;
    let _ = py_str("The kept parts as one line, with the punctuation the missing parts have left behind fixed.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&kept.clone());
    for __item in __iter1 {
    part = __item.clone();
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|or|then|but|plus|so|because|also|instead)\\s+")); __args.push(py_str("")); __args.push(part.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    continue;
    }
    starts_a_sentence = { let __b = mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("*\"'("), &slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None))) } };
    if py_truthy(&out.clone()) {
    if py_truthy(&{ let __b = starts_a_sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(",;:"), &geti(&geti(&out.clone(), &neg(&Py::Int(1))), &neg(&Py::Int(1))))) } }) {
    let _ = py_setitem(&out.clone(), &neg(&Py::Int(1)), add(&slice_or_raise(&geti(&out.clone(), &neg(&Py::Int(1))), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str(".")));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str(".!?"), &geti(&geti(&out.clone(), &neg(&Py::Int(1))), &neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[]) } }) {
    part = add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    let _ = mcall(&out.clone(), "append", &[part.clone()]);
    }
    left = mcall(&mcall(&py_str(" "), "join", &[out.clone()]), "rstrip", &[py_str(" ,;:")]);
    if py_truthy(&{ let __b = left.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str(".!?"), &geti(&left.clone(), &neg(&Py::Int(1))))) } }) {
    let __aug2 = add(&left.clone(), &py_str("."));
    left = __aug2;
    }
    if py_truthy(&mcall(&slice_or_raise(&left.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[])) {
    left = add(&mcall(&slice_or_raise(&left.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return left.clone();
    Py::None
}
pub fn _a_fact_put_right(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let _ = py_str("True when this line corrects one of their own facts rather than limiting the answer.\n\n    \"we are going to rome in may\" / \"no, florence not rome\": the reading that files anything\n    after a \"no\" as a thing ruled out takes the whole pair as the excluded thing (\"florence not\n    rome\"), and the Rome paragraph above is said back with a clause of it on the end - \"Right -\n    no florence not rome, then. The Pantheon. Book the Vatican museums online.\" The person did\n    not rule anything out: the value after the \"no\" is the one they are moving *to*, and the\n    memory store has already replaced Rome with Florence and written the line that says which\n    value the bot goes with now.\n\n    The store writes `memory[\"correction\"]` on the turn of the correction and it carries the\n    value that replaced the old one, so the composer asks that rather than reading the sentence a\n    second time. Where nothing was put right the key is absent and the limit reading stands - the\n    case it was written for (\"recommend a film\" / \"no horror\" takes the horror out of the list)\n    is untouched.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    fixed = mcall(&memory.clone(), "get", &[py_str("correction")]);
    return { let __b = call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&fixed.clone(), "get", &[py_str("value")])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn a_limit_on_the_answer(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a thing taken out of the answer above, not a report of their day.");
    said = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    return call_builtin("bool", &[{ let __b = mcall(&g_NOTHING_THAT_LIMITS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NO_LAUGH_OR_SCARES(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_limit_the_reading_missed(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reply for a limit the reading did not file, or None to leave the line alone.");
    said = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_LIMIT_SHAPE(), "match", &[mcall(&said.clone(), "strip", &[])]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_limit_on_the_answer(&[said.clone()]))) } }) {
    return Py::None;
    }
    above = _last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&above.clone())); if py_truthy(&__b) { __b } else { mcall(&mcall(&above.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    return _pick(&[g_MISSED_LIMIT(), memory.clone()]);
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut left: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let _ = py_str("The reply for a line that only rules something out, or None to leave the line alone.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("constraint")]))) } }) {
    return Py::None;
    }
    if py_truthy(&_a_fact_put_right(&[memory.clone()])) {
    return Py::None;
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("excludes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[t.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[t.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&things.clone()))) {
    return Py::None;
    }
    said = _last_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    thing = geti(&things.clone(), &Py::Int(0));
    if py_truthy(&mcall(&g_A_STATE_IS_NOT_A_LIMIT(), "match", &[thing.clone()])) {
    return Py::None;
    }
    left = _take_out(&[said.clone(), forms_of(&[thing.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&left.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&_carries(&[said.clone(), forms_of(&[thing.clone()])])))) {
    return Py::None;
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_pick(&[g_NOTHING_LEFT(), memory.clone()]), "format", args))(&__args) };
    } else {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("kept".to_string(), left.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_pick(&[g_KEPT(), memory.clone()]), "format", args))(&__args) };
    }
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.limit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_LIMIT_SHAPE" => Some(g_A_LIMIT_SHAPE()),
            "A_STATE_IS_NOT_A_LIMIT" => Some(g_A_STATE_IS_NOT_A_LIMIT()),
            "BACK_REFERENCE" => Some(g_BACK_REFERENCE()),
            "CONTINUES" => Some(g_CONTINUES()),
            "COUNTED" => Some(g_COUNTED()),
            "FRAME" => Some(g_FRAME()),
            "KEPT" => Some(g_KEPT()),
            "LEAST_KEPT" => Some(g_LEAST_KEPT()),
            "LEAST_WORDS" => Some(g_LEAST_WORDS()),
            "MISSED_LIMIT" => Some(g_MISSED_LIMIT()),
            "NOTHING_LEFT" => Some(g_NOTHING_LEFT()),
            "NOTHING_THAT_LIMITS" => Some(g_NOTHING_THAT_LIMITS()),
            "NO_LAUGH_OR_SCARES" => Some(g_NO_LAUGH_OR_SCARES()),
            "PART" => Some(g_PART()),
            "RESTATED" => Some(g_RESTATED()),
            "WORD" => Some(g_WORD()),
            "_a_fact_put_right" => Some(py_func("_a_fact_put_right", |args: &[Py]| _a_fact_put_right(args))),
            "_carries" => Some(py_func("_carries", |args: &[Py]| _carries(args))),
            "_joined" => Some(py_func("_joined", |args: &[Py]| _joined(args))),
            "_last_answer" => Some(py_func("_last_answer", |args: &[Py]| _last_answer(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_take_out" => Some(py_func("_take_out", |args: &[Py]| _take_out(args))),
            "_the_parts_left" => Some(py_func("_the_parts_left", |args: &[Py]| _the_parts_left(args))),
            "a_limit_on_the_answer" => Some(py_func("a_limit_on_the_answer", |args: &[Py]| a_limit_on_the_answer(args))),
            "a_limit_the_reading_missed" => Some(py_func("a_limit_the_reading_missed", |args: &[Py]| a_limit_the_reading_missed(args))),
            "forms_of" => Some(py_func("forms_of", |args: &[Py]| forms_of(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "restates_the_answer" => Some(py_func("restates_the_answer", |args: &[Py]| restates_the_answer(args))),
            "steps_left" => Some(py_func("steps_left", |args: &[Py]| steps_left(args))),
            _ => None,
        }),
        set: None,
    }))
}

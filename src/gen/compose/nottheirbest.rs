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


pub static A_CHEER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_DENIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_GRADED_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_OWNS_THE_GRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAD_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SHRUG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_CHEER() -> Py { ensure_init(); A_CHEER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHEER(v: Py) { ensure_init(); if let Ok(mut w) = A_CHEER.write() { *w = v; } }
pub fn g_A_DAY_DENIED() -> Py { ensure_init(); A_DAY_DENIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_DENIED(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_DENIED.write() { *w = v; } }
pub fn g_A_DAY_GRADED_ALONE() -> Py { ensure_init(); A_DAY_GRADED_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_GRADED_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_GRADED_ALONE.write() { *w = v; } }
pub fn g_A_DAY_OWNS_THE_GRADE() -> Py { ensure_init(); A_DAY_OWNS_THE_GRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_OWNS_THE_GRADE(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_OWNS_THE_GRADE.write() { *w = v; } }
pub fn g_A_HAD_BETTER() -> Py { ensure_init(); A_HAD_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAD_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = A_HAD_BETTER.write() { *w = v; } }
pub fn g_A_SHRUG() -> Py { ensure_init(); A_SHRUG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SHRUG(v: Py) { ensure_init(); if let Ok(mut w) = A_SHRUG.write() { *w = v; } }
pub fn g_QUESTION_OPENERS() -> Py { ensure_init(); QUESTION_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION_OPENERS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A day that is not their best is not a good one, and it is not a search.\n\nThree shapes reached the wrong pool on main, and both pools are wrong in the same way - they read\nthe words and missed that the person was reporting a bad time:\n\n    not my best day           -> \"That sounds like a good time - what was the best of it?\"\n    morning / could be better -> \"A quick search online will get you a solid answer\"\n    ive had better weeks      -> \"I'd rather not guess... a search will sort you out\"\n\nThe first congratulates somebody on the bad day they had just reported, because \"best day\" is a good\nword in a good-time pool that cannot see the \"not\" in front of it. The other two are a day graded low\nby comparison rather than named bad - no BAD_STRETCH word in the line at all - so nothing in the\ncomposer recognised them, and the understanding engine read the shape as a request and sent them to\nthe search hedge. Nobody looks up how their own week is going.\n\nThe reply is the warm pool a bad week already gets (`BAD_STRETCH_REPLIES`): one question back about\nwhat has made it like that. Nothing is invented and no fact is added, and only a reply that is a\ncongratulation, a neutral ack or a hedge is replaced, so a line with an answer of its own keeps it.\n");
    s_A_DAY_DENIED(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|hardly|isnt|isn't|wasnt|wasn't|arent|aren't|aint|ain't)\\s+(?:\\w+\\s+){0,2}(?:the\\s+)?(?:best|good|great|nice|fine|brilliant|amazing|alright|lush|smashing)\\s+(?:day|week|weekend|night|morning|evening|afternoon|shift|start|time)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAD_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:ive|i've|weve|we've|i)\\s+had\\s+better\\s+(?:day|days|week|weeks|weekend|weekends|night|nights|morning|mornings|evening|evenings|afternoon|shift|shifts)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_OWNS_THE_GRADE(stdlib_call("re", "compile", &[py_str("(?:\\b(?:i|im|i'm|we|we're|things|everything|life|work|today)\\b|\\b(?:my|our)\\s+(?:day|week|weekend|night|morning|evening|afternoon|shift|start|time|life|mood|year)\\b|\\bthe\\s+(?:day|week|weekend|morning|night)\\b)\\s+(?:could|would|can)\\s+(?:have\\s+been|of\\s+been|be\\s+going|be)\\s+(?:a\\s+lot\\s+|much\\s+|alright\\s+|ok\\s+)?better\\b|\\b(?:could|would)\\s+be\\s+going\\s+better\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_GRADED_ALONE(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:it|so|well|yeah|yea|aye|oh|hi)\\s+){0,2}(?:could|would|can|might)\\s+(?:have\\s+been|of\\s+been|be\\s+going|be)\\s+(?:a\\s+lot\\s+|much\\s+|alright\\s+|ok\\s+)?better\\b(?:\\s+(?:tbh|honestly|today|now|so\\s+far|though|i\\s+guess|i\\s+suppose|lately|at\\s+the\\s+moment|right\\s+now|this\\s+week|this\\s+morning|thanks|cheers)\\b)*\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_OPENERS(call_builtin("frozenset", &[mcall(&py_str("\nis isnt isn't was wasnt wasn't are arent aren't do does did will would should could can may might\nhave has had who what when where why how\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHEER(stdlib_call("re", "compile", &[py_str("\\bgood time\\b|\\bbest of it\\b|\\bnice one\\b|\\bpleased\\b|\\bwhat happened\\b|\\bthe best bit\\b|\\bhow did the rest of it go\\b|\\blovely\\b|\\bhappy for you\\b|\\bgreat news\\b|\\bwell done\\b|\\bcongratulat"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SHRUG(stdlib_call("re", "compile", &[py_str("\\bthanks for telling me\\b|\\bis there anything you'?d like a hand with\\b|\\bsay more if it helps\\b|\\bgot it\\b|\\bnoted\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn reads_as_a_low_day(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut graded: Py = Py::None;
    let _ = py_str("True when the line is the person's own day or week, denied or graded low by comparison.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    graded = { let __b = call_builtin("bool", &[mcall(&g_A_DAY_GRADED_ALONE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = { let __b = mcall(&g_A_DAY_DENIED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HAD_BETTER(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY_OWNS_THE_GRADE(), "search", &[clean.clone()]) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_QUESTION_OPENERS(), &geti(&{ let __b = mcall(&clean.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)))) } }]).unwrap_or_else(|e| exc::raise(e)) } };
    return graded.clone();
    Py::None
}
pub fn _may_replace(args: &[Py]) -> Py {
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
    let _ = py_str("True when the composer's line says the wrong thing about a day like this.\n\n    Only the three kinds are replaced - a congratulation, a shrug of an ack, and a hedge - so a\n    card's own answer to a line that happens to carry the shape keeps the turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    let mut is_a_hedge: Py = py_func("is_a_hedge", |args: &[Py]| crate::gen::compose::fallback::is_a_hedge(args));
    if py_truthy(&py_call(&is_a_hedge, &[reply.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&reply.clone(), "startswith", &[py_str("Noted:")])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_CHEER(), "search", &[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SHRUG(), "search", &[reply.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The bad-week pool for a day denied or graded low, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_low_day(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_may_replace(&[reply.clone()])))) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[crate::gen::compose::fallback_data::g_BAD_STRETCH_REPLIES(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.nottheirbest".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHEER" => Some(g_A_CHEER()),
            "A_DAY_DENIED" => Some(g_A_DAY_DENIED()),
            "A_DAY_GRADED_ALONE" => Some(g_A_DAY_GRADED_ALONE()),
            "A_DAY_OWNS_THE_GRADE" => Some(g_A_DAY_OWNS_THE_GRADE()),
            "A_HAD_BETTER" => Some(g_A_HAD_BETTER()),
            "A_SHRUG" => Some(g_A_SHRUG()),
            "QUESTION_OPENERS" => Some(g_QUESTION_OPENERS()),
            "_may_replace" => Some(py_func("_may_replace", |args: &[Py]| _may_replace(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reads_as_a_low_day" => Some(py_func("reads_as_a_low_day", |args: &[Py]| reads_as_a_low_day(args))),
            _ => None,
        }),
        set: None,
    }))
}

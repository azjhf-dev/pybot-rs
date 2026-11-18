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


pub static FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GROUPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FINGERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _GOOD_LUCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LUCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_FILLER() -> Py { ensure_init(); FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FILLER.write() { *w = v; } }
pub fn g_FOR_SOMEONE() -> Py { ensure_init(); FOR_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = FOR_SOMEONE.write() { *w = v; } }
pub fn g_GROUPS() -> Py { ensure_init(); GROUPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GROUPS(v: Py) { ensure_init(); if let Ok(mut w) = GROUPS.write() { *w = v; } }
pub fn g_WISH() -> Py { ensure_init(); WISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH(v: Py) { ensure_init(); if let Ok(mut w) = WISH.write() { *w = v; } }
pub fn g__FINGERS() -> Py { ensure_init(); _FINGERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FINGERS(v: Py) { ensure_init(); if let Ok(mut w) = _FINGERS.write() { *w = v; } }
pub fn g__GOOD_LUCK() -> Py { ensure_init(); _GOOD_LUCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__GOOD_LUCK(v: Py) { ensure_init(); if let Ok(mut w) = _GOOD_LUCK.write() { *w = v; } }
pub fn g__LUCK() -> Py { ensure_init(); _LUCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LUCK(v: Py) { ensure_init(); if let Ok(mut w) = _LUCK.write() { *w = v; } }
pub fn g__WOOD() -> Py { ensure_init(); _WOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WOOD(v: Py) { ensure_init(); if let Ok(mut w) = _WOOD.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A hopeful aside after the advice: \"fingers crossed\", \"with luck\", \"hope it works\".\n\nPeople end a turn about a job in hand with a wish, not a question. The bot answered them badly\nin three different ways, all of them because the line names nothing to look up:\n\n* \"fingers crossed\" after the condensation answer reached the health entry about fingers going\n  white, and the person who had just been told to open a window was told about Raynaud's;\n* \"with luck\" came back as \"The same goes for the luck: use the extractor fan ...\" - the detail\n  reader took the word as a part of the answer;\n* \"hope it works\" was filed away as a receipt: \"Got it - hope it works.\"\n\nNone of those is what a friend says. A friend says the wish back, briefly, warmly, and leaves the\ndoor open: \"Fingers crossed. Tell me how it goes.\" That is the whole reply, so this module has one\npool and no knowledge in it.\n\nThe reading is strict, because a line with something of its own in it belongs to whatever answers\nthat thing:\n\n* the whole line has to be the wish and the fillers around it (\"ok fingers crossed\", \"fingers\n  crossed for me\", \"with any luck\", \"touch wood\", \"hope so\", \"i hope it does the trick\"): a line\n  that says anything else (\"im going to rome, fingers crossed\") is left alone;\n* a hope about somebody or something named (\"hope you feel better\", \"hope the scan goes well\") is\n  a wish for a person, not an aside about the plan, so it is left to the greeting, which knows how\n  to answer a good wish;\n* asking is not hoping: a line with a question mark is a question (\"any luck?\"), and the answer is\n  what they found, not a wish back.\n\nThe wish people make most of all is \"good luck\", and it was the one shape missing: after an answer\nit fell past this module to whatever had just answered, which printed the answer again - the\nmarathon programme came back word for word under \"alright then, good luck\", \"good luck!\" and\n\"right, best of luck\". No line of that shape names anything to look up, so it belongs here with\nthe rest.\n");
    s_FILLER(py_str("(?:(?:ok|okay|well|right|alright|and|so|then|anyway|honestly|please|thanks|ta|ha|lol|x)\\b[,;\\s]+)*"));
    s_WISH(stdlib_call("re", "compile", &[add(&add(&add(&g_FILLER(), &py_str("(?:(?:good|best\\s+of|all\\s+the)\\s+luck(?:\\s+(?:with|on|for)\\s+(?:it|that|this|them|me|us|(?:the|your|my)\\s+[a-z']+(?:\\s+[a-z']+)?))?|all\\s+the\\s+best(?:\\s+(?:with|to)\\s+(?:it|that|this|you|them))?|(?:keep\\s+)?(?:your|my|our)?\\s*(?:fingers|fingers and toes|thumbs)(?:\\s+(?:and|n)?\\s*(?:toes))?\\s*(?:crossed|are crossed)?(?:\\s+for\\s+(?:me|us))?|(?:with\\s+)?(?:any\\s+)?luck(?:,?\\s+(?:then|anyway))?|touch\\s+wood|heres?\\s+hoping|(?:hopefully(?:,?\\s+(?:then|anyway))?(?:\\s+(?:that|it|this)(?:\\s+(?:works?|does(?:\\s+(?:the\\s+trick|it|the\\s+job))?|helps?|is\\s+enough|sorts\\s+it))?)?)|(?:i\\s+)?(?:really\\s+|do\\s+|just\\s+)?hope\\s+(?:so|it|that|this)(?:\\s+(?:works?|does(?:\\s+(?:the\\s+trick|it|the job))?|helps?|goes\\s+well|lands|is\\s+enough|comes\\s+off|holds|clears(?:\\s+up)?|sorts\\s+it))?)")), &g_FILLER()), &py_str("[!. ]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GROUPS(py_dict(vec![(py_str("goodluck"), py_tuple(vec![py_str("Good luck with it — tell me how it goes."), py_str("Good luck. Say how you get on.")])), (py_str("fingers"), py_tuple(vec![py_str("Fingers crossed. Tell me how it goes."), py_str("Crossing mine as well. Say how you get on.")])), (py_str("luck"), py_tuple(vec![py_str("With luck, then. I would like to hear how it turns out."), py_str("With luck. Tell me how it goes.")])), (py_str("wood"), py_tuple(vec![py_str("Touch wood. Tell me how it goes once you know."), py_str("Touch wood, then. Say how it turns out.")])), (py_str("hope"), py_tuple(vec![py_str("Hope so too. Come back and say how it landed."), py_str("Hoping with you. Tell me how it goes.")]))]));
    s__FINGERS(stdlib_call("re", "compile", &[py_str("\\b(?:fingers|thumbs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WOOD(stdlib_call("re", "compile", &[py_str("touch\\s+wood"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LUCK(stdlib_call("re", "compile", &[py_str("\\bluck\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__GOOD_LUCK(stdlib_call("re", "compile", &[py_str("\\b(?:good|best\\s+of|all\\s+the)\\s+luck\\b|\\ball\\s+the\\s+best\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_SOMEONE(stdlib_call("re", "compile", &[py_str("\\bhope\\s+(?:you|your|he|she|they|it\\s+goes|the|everything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _group(args: &[Py]) -> Py {
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
    if py_truthy(&mcall(&g__GOOD_LUCK(), "search", &[line.clone()])) {
    return py_str("goodluck");
    }
    if py_truthy(&mcall(&g__FINGERS(), "search", &[line.clone()])) {
    return py_str("fingers");
    }
    if py_truthy(&mcall(&g__WOOD(), "search", &[line.clone()])) {
    return py_str("wood");
    }
    if py_truthy(&mcall(&g__LUCK(), "search", &[line.clone()])) {
    return py_str("luck");
    }
    return py_str("hope");
    Py::None
}
pub fn reading(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("Whether the whole line is a hopeful aside about the thing in hand.");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_FOR_SOMEONE(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_WISH(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut count: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The brief reply to a hopeful aside, or None when the line is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&reading(&[text.clone()])))) {
    return Py::None;
    }
    pool = geti(&g_GROUPS(), &_group(&[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    count = mcall(&memory.clone(), "get", &[py_str("hope_said")]);
    count = if py_truthy(&call_builtin("isinstance", &[count.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { add(&count.clone(), &Py::Int(1)) } else { Py::Int(1) };
    let _ = py_setitem(&memory.clone(), &py_str("hope_said"), count.clone());
    return geti(&pool.clone(), &modulo(&sub(&count.clone(), &Py::Int(1)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    }
    return geti(&pool.clone(), &Py::Int(0));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.hope".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "FILLER" => Some(g_FILLER()),
            "FOR_SOMEONE" => Some(g_FOR_SOMEONE()),
            "GROUPS" => Some(g_GROUPS()),
            "WISH" => Some(g_WISH()),
            "_FINGERS" => Some(g__FINGERS()),
            "_GOOD_LUCK" => Some(g__GOOD_LUCK()),
            "_LUCK" => Some(g__LUCK()),
            "_WOOD" => Some(g__WOOD()),
            "_group" => Some(py_func("_group", |args: &[Py]| _group(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reading" => Some(py_func("reading", |args: &[Py]| reading(args))),
            _ => None,
        }),
        set: None,
    }))
}

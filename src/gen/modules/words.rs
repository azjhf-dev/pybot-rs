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


pub static ASKS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXAMPLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUOTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKS_WORDS() -> Py { ensure_init(); ASKS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WORDS.write() { *w = v; } }
pub fn g_ASK_FILLER() -> Py { ensure_init(); ASK_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FILLER.write() { *w = v; } }
pub fn g_ASK_OBJECT() -> Py { ensure_init(); ASK_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = ASK_OBJECT.write() { *w = v; } }
pub fn g_EXAMPLED() -> Py { ensure_init(); EXAMPLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXAMPLED(v: Py) { ensure_init(); if let Ok(mut w) = EXAMPLED.write() { *w = v; } }
pub fn g_LINES() -> Py { ensure_init(); LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LINES.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g_QUOTED() -> Py { ensure_init(); QUOTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); if let Ok(mut w) = QUOTED.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The words to say, when the answer just gave them.\n\nA person who has been told what to do often asks for the words next: \"what should i say to\nher\", \"how do i word it\", \"what do i text him\", \"what should i put in the email\". By then the\nanswer has usually already put the sentence in their mouth - \"Mention it lightly instead of\nletting it fester: 'I missed hearing from you on my birthday, did it slip your mind?'\", \"ask\nfor a specific figure with the reason attached: \"I'd like to move to 34,000, given...\"\" - and\nthe bot was answering all of those with \"Keep it to two sentences... send me the gist and I'll\ndraft it\", asking again for what it had already written. A wording ask is answered with the\nwords the last answer quoted, so the person can copy them.\n\nSo this module answers only when all three hold: the line asks for the words themselves, the\nbot has just said something, and that something quoted a sentence of three words or more. It\nnever invents a sentence and never guesses at one: no quote in the last answer, no reply.\n");
    s_PRIORITY(Py::Int(1200));
    s_ASKS_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+(?:say|text|email|message|write|reply|answer|word|phrase|tell|open|start)\\b|\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+put\\s+(?:it|this|that|these|those|them)\\b|\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+put\\s+(?:[^?.]{0,30}?\\b(?:in|into|on)\\s+)?(?:the|my|his|her|their|your|our|a|an)\\s+(?:e-?mail|letter|card|note|text|message|msg|reply|answer|application|invitation|invite|whatsapp|bio|post|comment|review|essay|report)\\b|\\bhow\\s+(?:do|should|would|can|could)\\s+i\\s+(?:word|phrase|put)\\s+(?:it|this|that)\\b|\\bhow\\s+(?:do|should|would|can|could)\\s+i\\s+ask\\s+(?:for\\s+)?(?:it|that|this|them|him|her|someone|somebody)\\b|\\b(?:can|could|should)\\s+i\\s+ask\\s+(?:for\\s+)?(?:it|that|this|them|him|her)\\b(?:\\s+(?:back|again))?[\\s.?!]*$|\\b(?:can|could|would)\\s+you\\s+(?:word|phrase)\\s+(?:it|this|that)\\b|\\b(?:give|gimme)\\s+me\\s+the\\s+words\\b|\\bwhat\\s+are\\s+the\\s+words\\b|\\bwhat(?:'s| is)\\s+the\\s+wording\\b|\\b(?:don'?t|do\\s+not|dont|no\\s+idea|not\\s+sure|never\\s+know\\s+how|can'?t\\s+work\\s+out)\\s+(?:know\\s+)?how\\s+to\\s+(?:tell|say|word|phrase|put|ask|bring|raise|mention|open|start|write|text|email|message)\\b|\\b(?:don'?t|do\\s+not|dont|no\\s+idea|not\\s+sure)\\s+what\\s+to\\s+(?:say|text|write|reply|message|email|call|tell)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTED(stdlib_call("re", "compile", &[py_str("(?<![A-Za-z0-9])'((?:[^']|'(?=[A-Za-z])){12,300})'(?![A-Za-z0-9])|(?<![A-Za-z0-9])\\\"((?:[^\\\"]|\\\"(?=[A-Za-z])){12,300})\\\"(?![A-Za-z0-9])|\\u201c([^\\u201d]{12,300})\\u201d|\\u2018([^\\u2019]{12,300})\\u2019")]).unwrap_or_else(|e| exc::raise(e)));
    s_LINES(py_tuple(vec![py_str("{words} Say that, and leave it there - it gives them an easy way to answer."), py_str("{words} That is the whole of it, so don't add a second explanation after it."), py_str("{words} Say it once, early, and let the silence do the rest."), py_str("{words} Keep to those words and let them reply in their own time."), py_str("{words} The short version lands better than a speech, so stop there.")]));
    s_EXAMPLED(stdlib_call("re", "compile", &[py_str("(?:\\b(?:a|an|the|one|like|such\\s+as|e\\.g\\.))\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_OBJECT(stdlib_call("re", "compile", &[py_str("^(?:(?:it|this|that|them|those)\\b|(?:to|for|at|with|in|on)\\s+(?:him|her|them|it|this|that|me|us|everyone|somebody|anybody)\\b|(?:to|for|at|with|in|on)\\s+(?:my|his|her|their|our|your|the|a|an)\\s+[a-z']+|(?:him|her|them|everyone|somebody)\\b|(?:my|his|her|their|our|your|the)\\s+[a-z']+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FILLER(stdlib_call("re", "compile", &[py_str("^(?:please|then|instead|just|really|up|out|loud|back)\\b[,.]?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _quoted(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first quoted sentence of three words or more in the bot's last reply, with its marks.\n\n    The marks come back with it because the answer chose them: a sentence the answer set in\n    double quotes is handed back in double quotes, so it looks on screen as it did a turn ago.\n    ");
    text = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_QUOTED(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    words = mcall(&call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&match_.clone(), "groups", &[])) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&words.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    continue;
    }
    if py_truthy(&mcall(&g_EXAMPLED(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(18))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None)])) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[?!.]$"), words.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&words.clone(), &py_str(","))) } }) {
    continue;
    }
    }
    return mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "strip", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _names_its_own(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut filler: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("True when the line asks for the words *and* says what they are for.\n\n    \"what should i say to him\" is the ask and nothing else, so the words of the last answer are\n    the answer. \"how do i say sorry\" and \"how do i say it without a row\" carry their own subject,\n    and the answer two turns back is not it.\n    ");
    match_ = mcall(&g_ASKS_WORDS(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(true);
    }
    rest = mcall(&slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(2)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    rest = mcall(&rest.clone(), "strip", &[py_str(" ,.-")]);
    moved = mcall(&g_ASK_OBJECT(), "match", &[rest.clone()]);
    if py_truthy(&moved.clone()) {
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&moved.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    _u = __item.clone();
    filler = mcall(&g_ASK_FILLER(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&filler.clone()))) {
    break;
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&filler.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    }
    return call_builtin("bool", &[stdlib_call("re", "sub", &[py_str("[^a-z0-9]"), py_str(""), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_words(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The quoted words this line is asking for, or \"\" when this line is not that ask.");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_WORDS(), "search", &[line.clone()]))) } } } }) {
    return py_str("");
    }
    if py_truthy(&_names_its_own(&[line.clone()])) {
    return py_str("");
    }
    return _quoted(&[crate::gen::memory::last_bot(&[memory.clone()])]);
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This line is asking for the words the last answer already gave, so it is this module's.\n\n    The reply is a quote of the answer, so on the surface it looks like the bot saying the same\n    thing twice, and a general module that answers the same line about something else - a\n    birthday-card line for \"what should i say to her\" - is preferred to it by the specialist\n    fit. Declaring the line instead of raising PRIORITY says that for this reading only.\n    ");
    return call_builtin("bool", &[_the_words(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    words = _the_words(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("words".to_string(), words.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_LINES(), memory.clone(), line.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.words".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_WORDS" => Some(g_ASKS_WORDS()),
            "ASK_FILLER" => Some(g_ASK_FILLER()),
            "ASK_OBJECT" => Some(g_ASK_OBJECT()),
            "EXAMPLED" => Some(g_EXAMPLED()),
            "LINES" => Some(g_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "QUOTED" => Some(g_QUOTED()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_names_its_own" => Some(py_func("_names_its_own", |args: &[Py]| _names_its_own(args))),
            "_quoted" => Some(py_func("_quoted", |args: &[Py]| _quoted(args))),
            "_the_words" => Some(py_func("_the_words", |args: &[Py]| _the_words(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

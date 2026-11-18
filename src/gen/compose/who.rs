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


pub static BY_TOPIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_TOPICS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EARLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPERTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREAMBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUIET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKES_AS_IT_COMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_BY_TOPIC() -> Py { ensure_init(); BY_TOPIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BY_TOPIC(v: Py) { ensure_init(); if let Ok(mut w) = BY_TOPIC.write() { *w = v; } }
pub fn g_CARE_TOPICS() -> Py { ensure_init(); CARE_TOPICS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_TOPICS(v: Py) { ensure_init(); if let Ok(mut w) = CARE_TOPICS.write() { *w = v; } }
pub fn g_EARLY() -> Py { ensure_init(); EARLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EARLY(v: Py) { ensure_init(); if let Ok(mut w) = EARLY.write() { *w = v; } }
pub fn g_EXPERTS() -> Py { ensure_init(); EXPERTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPERTS(v: Py) { ensure_init(); if let Ok(mut w) = EXPERTS.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_PREAMBLE() -> Py { ensure_init(); PREAMBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREAMBLE(v: Py) { ensure_init(); if let Ok(mut w) = PREAMBLE.write() { *w = v; } }
pub fn g_QUIET() -> Py { ensure_init(); QUIET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUIET(v: Py) { ensure_init(); if let Ok(mut w) = QUIET.write() { *w = v; } }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); TAKES_AS_IT_COMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); if let Ok(mut w) = TAKES_AS_IT_COMES.write() { *w = v; } }
pub fn g_THREAD() -> Py { ensure_init(); THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = THREAD.write() { *w = v; } }
pub fn g_WHO_ASK() -> Py { ensure_init(); WHO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHO_ASK.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Who to see, when the follow-up asks for the person and not the advice again.\n\n\"my back hurts\" gets what to do about it. \"who should i see\" is a second, different ask: it wants\nthe person and the way in - a GP, a vet, a dentist, a physio - and nothing else in the line. On\nmain it got the advice a second time (the back paragraph, the scratching paragraph), the flat\n\"That is everything I have on that one\", or the search hedge.\n\nThe person is usually already in the bot's own last line: the answer to a symptom, a pet or a\nchild's rash names the one to go to, because that is how the entry ends. So this reads the person\nout of that answer and hands that sentence back, with one line of its own on how to get to them.\nNothing is invented about the subject: the sentence was said a moment ago, and the line after it\nis only about booking, not about the condition.\n\nWhen the answer named nobody (the worry answer, the low-mood answer), the person comes from the\ntopic the line was read as: health and low mood to a GP, pets to a vet, with the obvious exception\nread off the thread (\"tooth\" is a dentist, not a GP). Anything else - a household fault, a money\nquestion, a bereavement - is left alone, because those threads already answer \"who do i call\"\nthemselves.\n\nIt only reads a line that is nothing but the ask (a bare \"who should i see\", \"who do i call then\",\n\"who to ask about it\"): a line that carries its own subject (\"who do i call about the boiler\",\n\"which doctor is best for skin\") belongs to the entry that knows the subject.\n");
    s_EXPERTS(py_tuple(vec![py_tuple(vec![py_str("health visitor"), stdlib_call("re", "compile", &[py_str("\\bhealth visitors?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring them and they will talk it through with you today"), py_str("ring them and describe what you have seen: they would rather hear from you than not")])]), py_tuple(vec![py_str("vet"), stdlib_call("re", "compile", &[py_str("\\bvets?\\b|\\bvet's\\b|\\bveterinary\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring the practice and say how long it has been going and what you have already tried"), py_str("book an appointment and take a note of what you have seen and any treatment they are on")])]), py_tuple(vec![py_str("dentist"), stdlib_call("re", "compile", &[py_str("\\bdentists?\\b|\\bhygienist\\b|\\bdental (?:practice|surgery)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring the practice and ask for the earliest appointment"), py_str("ring the practice and ask for an urgent slot")])]), py_tuple(vec![py_str("GP"), stdlib_call("re", "compile", &[py_str("\\bgp\\b|\\bgp's\\b|\\bdoctors?\\b|\\bfamily doctor\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring the surgery and ask for the earliest appointment"), py_str("book in and take a note of when it started and what makes it worse")])]), py_tuple(vec![py_str("physio"), stdlib_call("re", "compile", &[py_str("\\bphysios?\\b|\\bphysiotherapist\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("you can usually book with one directly, without going through a GP"), py_str("book an appointment and take a note of when the pain comes on")])]), py_tuple(vec![py_str("optician"), stdlib_call("re", "compile", &[py_str("\\bopticians?\\b|\\boptometrist\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("book an eye test, and say if anything has changed suddenly"), py_str("ring them and say what has changed, and they will say how soon to come in")])]), py_tuple(vec![py_str("pharmacist"), stdlib_call("re", "compile", &[py_str("\\bpharmacists?\\b|\\bchemist\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("they can advise today without an appointment"), py_str("ask at the counter, and take the packet of anything you already take")])]), py_tuple(vec![py_str("midwife"), stdlib_call("re", "compile", &[py_str("\\bmidwives?\\b|\\bmaternity unit\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring your midwife or the maternity unit and they will say how soon to be seen"), py_str("ring the maternity unit rather than waiting for the next appointment")])]), py_tuple(vec![py_str("nurse"), stdlib_call("re", "compile", &[py_str("\\bpractice nurse\\b|\\bnurse\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("ring the surgery and ask whether the practice nurse can see you"), py_str("ask at the desk for the practice nurse")])])]));
    s_LEAD(py_dict(vec![(py_str("health visitor"), py_str("Your health visitor")), (py_str("vet"), py_str("A vet")), (py_str("dentist"), py_str("A dentist")), (py_str("GP"), py_str("Your GP")), (py_str("physio"), py_str("A physio")), (py_str("optician"), py_str("An optician")), (py_str("pharmacist"), py_str("A pharmacist")), (py_str("midwife"), py_str("Your midwife")), (py_str("nurse"), py_str("The practice nurse"))]));
    s_WHO_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|ok|okay|but|right|then|well)\\s+)*(?:who\\s+(?:should|shall|do|can|could|would|must|might|am\\s+i)\\s+(?:i|we)\\s+(?:see|call|ring|contact|ask|book|speak\\s+to|talk\\s+to|go\\s+and\\s+see|go\\s+to|get\\s+in\\s+touch\\s+with|be\\s+seeing|need\\s+to\\s+see)|who\\s+(?:to\\s+(?:see|call|ring|ask|contact|speak\\s+to|talk\\s+to))|who\\s+do\\s+i\\s+need\\s+to\\s+(?:see|call)|which\\s+(?:doctor|dentist|vet|specialist|professional|person|one)\\s+(?:should|do|would)\\s+i\\s+(?:see|call|ring|go\\s+to)|where\\s+do\\s+i\\s+go\\s+for\\s+(?:it|that|this))(?:\\s+(?:about|for|with)\\s+(?:it|that|this|them|someone))?(?:\\s+(?:then|now|instead|first|please|again|next))?[\\s,.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREAMBLE(stdlib_call("re", "compile", &[py_str("(?:,\\s*(?:and|but|so)\\s+|;\\s*|\\.\\s+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EARLY(Py::Int(45));
    s_BY_TOPIC(py_tuple(vec![py_tuple(vec![py_str("pets"), py_str("vet")]), py_tuple(vec![py_str("health"), py_str("GP")]), py_tuple(vec![py_str("feelings"), py_str("GP")])]));
    s_THREAD(py_tuple(vec![py_tuple(vec![py_str("dentist"), stdlib_call("re", "compile", &[py_str("\\btooth\\b|\\bteeth\\b|\\bgums?\\b|\\bdental\\b|\\bdentist\\b|\\bmolar\\b|\\babscess\\b|\\bbraces?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("optician"), stdlib_call("re", "compile", &[py_str("\\beyes?\\b|\\beyesight\\b|\\bvision\\b|\\boptician\\b|\\bblurry\\b|\\bblurred\\b|\\bcontact lenses?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("midwife"), stdlib_call("re", "compile", &[py_str("\\bpregnan|\\bmidwife\\b|\\bmaternity\\b|\\bunborn\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]));
    s_QUIET(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("urgent"), py_str("grief"), py_str("loss")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("health"), py_str("feelings"), py_str("family-and-parenting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(g_takes_as_it_comes());
}
pub fn _flat(args: &[Py]) -> Py {
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
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    text = mcall(&mcall(&text.clone(), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("‘"), py_str("'")]);
    return stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_who(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is nothing but a request for the person to see.");
    return call_builtin("bool", &[mcall(&g_WHO_ASK(), "match", &[mcall(&_flat(&[text.clone()]), "strip", &[py_str(" .!?")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_module(args: &[Py]) -> Py {
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
    return crate::gen::memory::holds::last_module(&[memory.clone()]);
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    return crate::gen::memory::holds::last_reply(&[memory.clone()]);
    Py::None
}
pub fn _names_no_subject(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for a line that only says to go on: \"what can i do\", \"and then\", \"thanks\".\n\n    The understander's keywords are what is left of a line once the grammar is gone, so a line\n    with none of them named nothing: it is a follow-up about the subject before it.\n    ");
    if py_truthy(&asks_who(&[line.clone()])) {
    return Py::Bool(true);
    }
    read = crate::gen::understanding::understand(&[_flat(&[line.clone()]), py_dict(vec![])]);
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&read.clone(), "get", &[py_str("keywords")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _the_subject(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("(turn index, the answer to it) for the last line that named a subject.\n\n    A bare \"who do i ring\" is about the last thing the person described, which is not always the\n    last line and is almost never the last answer. \"she keeps falling\" is answered \"a second one\n    is worth a GP visit\", \"what can i do\" is answered with the grab-rails checklist, and the ask\n    arrives after an answer that names nobody: the person the thread named is in the answer to the\n    line that named the subject.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    turn = geti(&turns.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { _names_no_subject(&[line.clone()]) } }) {
    continue;
    }
    return py_tuple(vec![index.clone(), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), line.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _thread_text(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { py_str("") };
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the person said over the last few lines, which is what the thread is about.\n\n    Their own words only. The bot's answers are where the *person* is read from, and they are the\n    worst place to read the subject from: advice names things the person never mentioned, as the\n    thing to rule out or to check. \"my mum keeps falling\" / \"what can i do\" ends \"...and get the\n    eyes checked\", and \"who do i ring\" was answered \"An optician: book an eye test\" - the mother\n    had fallen, and a passing mention in the advice became the subject of the thread.\n    ");
    said = py_list(vec![answer.clone()]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&crate::gen::memory::holds::turns(&[memory.clone()]), &neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn _sentence_at(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut begin: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("(the sentence of `text` the offset `at` sits in, where that sentence starts).");
    begin = Py::Int(0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("(?<=[.!?]) "), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&ge(&mcall(&found.clone(), "start", &[]), &at.clone())) {
    return py_tuple(vec![slice_or_raise(&text.clone(), &begin.clone(), &mcall(&found.clone(), "start", &[]), &Py::None), begin.clone()]);
    }
    begin = mcall(&found.clone(), "end", &[]);
    }
    return py_tuple(vec![slice_or_raise(&text.clone(), &begin.clone(), &Py::None, &Py::None), begin.clone()]);
    Py::None
}
pub fn _person_in(args: &[Py]) -> Py {
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
    let mut _way: Py = Py::None;
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut start: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(person, the sentence that names them, offset of the person in it) from an answer, or None.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_EXPERTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    words = __unpk2[1].clone();
    _way = __unpk2[2].clone();
    found = mcall(&words.clone(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { ge(&mcall(&found.clone(), "start", &[]), &geti(&best.clone(), &Py::Int(3))) } }) {
    continue;
    }
    let __unpk3 = py_unpack(&_sentence_at(&[said.clone(), mcall(&found.clone(), "start", &[])]), 2, None);
    sentence = __unpk3[0].clone();
    start = __unpk3[1].clone();
    best = py_tuple(vec![name.clone(), sentence.clone(), call_builtin("max", &[Py::Int(0), sub(&mcall(&found.clone(), "start", &[]), &start.clone())]).unwrap_or_else(|e| exc::raise(e)), mcall(&found.clone(), "start", &[])]);
    }
    if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) {
    return Py::None;
    }
    return py_tuple(vec![geti(&best.clone(), &Py::Int(0)), geti(&best.clone(), &Py::Int(1)), geti(&best.clone(), &Py::Int(2))]);
    Py::None
}
pub fn _cut_the_preamble(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut found: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The referral from the person's own clause in, without the run-up to it.\n\n    \"Weeks of it is not a bad day any more, and it is worth saying to a GP rather than waiting it\n    out\" is a referral from the \"and\": handed back whole, the person gets the mood sentence again\n    before the name they asked for.\n    ");
    start = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_PREAMBLE(), "finditer", &[sentence.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&gt(&mcall(&found.clone(), "end", &[]), &at.clone())) {
    break;
    }
    if py_truthy(&le(&sub(&at.clone(), &mcall(&found.clone(), "end", &[])), &Py::Int(80))) {
    start = mcall(&found.clone(), "end", &[]);
    }
    }
    return py_tuple(vec![mcall(&slice_or_raise(&sentence.clone(), &start.clone(), &Py::None, &Py::None), "strip", &[]), start.clone()]);
    Py::None
}
pub fn _tidy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    clause = mcall(&mcall(&mcall(&mcall(&clause.clone(), "strip", &[]), "strip", &[py_str("\"")]), "strip", &[]), "rstrip", &[py_str(" ,;:")]);
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&clause.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])]))) } }) {
    let __aug1 = add(&clause.clone(), &py_str("."));
    clause = __aug1;
    }
    if py_truthy(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[])) {
    clause = add(&mcall(&geti(&clause.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return clause.clone();
    Py::None
}
pub fn _lead_of(args: &[Py]) -> Py {
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
    return mcall(&g_LEAD(), "get", &[name.clone(), name.clone()]);
    Py::None
}
pub fn _the_way_in(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut clause: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { py_str("") };
    let mut _words: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut options: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said_flat: Py = Py::None;
    let mut way: Py = Py::None;
    let mut ways: Py = Py::None;
    let _ = py_str("The one line of the reply that is not the answer's: how to get to the person.\n\n    A way in the answer already gave (\"ring the practice\", \"ask for the earliest appointment\") is\n    not said again: the line is there to add the step, not to say it twice.\n    ");
    ways = py_tuple(vec![]);
    let __iter1 = py_iter_or_raise(&g_EXPERTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    other = __unpk2[0].clone();
    _words = __unpk2[1].clone();
    options = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&other.clone(), &name.clone()))) {
    ways = options.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&ways.clone()))) {
    return py_str("");
    }
    said_flat = _flat(&[clause.clone()]);
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ways.clone()) { let mut way: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_says_it_again(&[way.clone(), said_flat.clone()])))) {  __out.push(way.clone()); } } py_list(__out) })() };
    return crate::gen::compose::pick(&[{ let __b = fresh.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[ways.clone()]).unwrap_or_else(|e| exc::raise(e)) } }, memory.clone(), said.clone()]);
    Py::None
}
pub fn _says_it_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut way: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "way") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: way"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut at: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the way in repeats a run of three words the clause already used.");
    words = mcall(&_flat(&[way.clone()]), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), call_builtin("max", &[Py::Int(0), sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &at.clone(), &add(&at.clone(), &Py::Int(3)), &Py::None)])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _person_for_the_topic(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut subject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { py_str("") };
    let mut care: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut read: Py = Py::None;
    let mut t: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut thread: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The person the thread is about, when the answer named nobody.\n\n    `subject` is the last line that named one: its reading beats the carried topic, which can be\n    a topic of its own (\"my dog keeps scratching\" then \"my boiler is noisy\" left `pets` on the\n    reading of a who-ask, and \"who do i call\" was answered \"A vet\").\n    ");
    read = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    topics = py_list(vec![]);
    if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    topics = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&read.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[t.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    one = mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&one.clone()) {
    let _ = mcall(&topics.clone(), "append", &[one.clone()]);
    }
    }
    if py_truthy(&subject.clone()) {
    theirs = crate::gen::understanding::understand(&[_flat(&[subject.clone()]), py_dict(vec![])]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[theirs.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&theirs.clone(), "get", &[py_str("topics")]) } }) {
    topics = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&theirs.clone(), &py_str("topics"))) { let mut t: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[t.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    }
    }
    care = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topics.clone()) { let mut topic: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_CARE_TOPICS(), &topic.clone()))) {  __out.push(topic.clone()); } } py_list(__out) })() };
    if py_truthy(&care.clone()) {
    thread = _thread_text(&[memory.clone(), said.clone()]);
    let __iter1 = py_iter_or_raise(&g_THREAD());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&mcall(&words.clone(), "search", &[thread.clone()])) {
    return name.clone();
    }
    }
    }
    let __iter3 = py_iter_or_raise(&g_BY_TOPIC());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk4[0].clone();
    name = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_contains(&topics.clone(), &topic.clone()))) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _from_the_answer(args: &[Py]) -> Py {
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
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut at: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut repeat: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repeat") { v.clone() }
    else { Py::Bool(true) };
    let mut body: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut start: Py = Py::None;
    let mut way: Py = Py::None;
    let _ = py_str("The person the answer named, with the one line on how to get to them.\n\n    `repeat` is False when the sentence comes from an answer the person has already heard: the\n    answer above the last one is not said again, so the reply is the person and the way in.\n    ");
    let __unpk1 = py_unpack(&_cut_the_preamble(&[sentence.clone(), at.clone()]), 2, None);
    clause = __unpk1[0].clone();
    start = __unpk1[1].clone();
    way = _the_way_in(&[name.clone(), memory.clone(), said.clone(), clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&way.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = repeat.clone(); if !py_truthy(&__b) { __b } else { le(&sub(&at.clone(), &start.clone()), &g_EARLY()) } }) {
    body = _tidy(&[clause.clone()]);
    if py_truthy(&{ let __b = way.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_it_again(&[way.clone(), _flat(&[body.clone()])]))) } }) {
    body = add(&add(&add(&add(&body.clone(), &py_str(" ")), &mcall(&geti(&way.clone(), &Py::Int(0)), "upper", &[])), &slice_or_raise(&way.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    return body.clone();
    }
    return add(&add(&add(&_lead_of(&[name.clone()]), &py_str(": ")), &way.clone()), &py_str("."));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answer: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut way: Py = Py::None;
    let _ = py_str("The person to see, or None on every line that is not a bare ask for one.");
    memory = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&asks_who(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_QUIET(), &_last_module(&[memory.clone()])))) {
    return Py::None;
    }
    answer = _last_reply(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    found = _person_in(&[answer.clone()]);
    if py_truthy(&found.clone()) {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&found.clone())); __kws.extend(vec![("memory".to_string(), memory.clone()), ("said".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_the_answer(args))(&__args) };
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    }
    subject = _the_subject(&[memory.clone()]);
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { { let __b = geti(&subject.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&subject.clone(), &Py::Int(0)), &sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) } } } }) {
    found = _person_in(&[geti(&subject.clone(), &Py::Int(1))]);
    if py_truthy(&found.clone()) {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&found.clone())); __kws.extend(vec![("memory".to_string(), memory.clone()), ("said".to_string(), text.clone()), ("repeat".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_the_answer(args))(&__args) };
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    }
    }
    name = _person_for_the_topic(&[memory.clone(), text.clone(), if py_truthy(&subject.clone()) { geti(&subject.clone(), &Py::Int(2)) } else { py_str("") }]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    way = _the_way_in(&[name.clone(), memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&way.clone()))) {
    return Py::None;
    }
    return add(&add(&add(&_lead_of(&[name.clone()]), &py_str(": ")), &way.clone()), &py_str("."));
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("Only on a line that is nothing but the ask, over a thread that has a person to name.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&reply_for(&[text.clone(), memory.clone()]), &Py::None))]);
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
    Py::Module(Rc::new(PyModule { name: "bot.compose.who".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BY_TOPIC" => Some(g_BY_TOPIC()),
            "CARE_TOPICS" => Some(g_CARE_TOPICS()),
            "EARLY" => Some(g_EARLY()),
            "EXPERTS" => Some(g_EXPERTS()),
            "LEAD" => Some(g_LEAD()),
            "PREAMBLE" => Some(g_PREAMBLE()),
            "QUIET" => Some(g_QUIET()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "THREAD" => Some(g_THREAD()),
            "WHO_ASK" => Some(g_WHO_ASK()),
            "_cut_the_preamble" => Some(py_func("_cut_the_preamble", |args: &[Py]| _cut_the_preamble(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_from_the_answer" => Some(py_func("_from_the_answer", |args: &[Py]| _from_the_answer(args))),
            "_last_module" => Some(py_func("_last_module", |args: &[Py]| _last_module(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_lead_of" => Some(py_func("_lead_of", |args: &[Py]| _lead_of(args))),
            "_names_no_subject" => Some(py_func("_names_no_subject", |args: &[Py]| _names_no_subject(args))),
            "_person_for_the_topic" => Some(py_func("_person_for_the_topic", |args: &[Py]| _person_for_the_topic(args))),
            "_person_in" => Some(py_func("_person_in", |args: &[Py]| _person_in(args))),
            "_says_it_again" => Some(py_func("_says_it_again", |args: &[Py]| _says_it_again(args))),
            "_sentence_at" => Some(py_func("_sentence_at", |args: &[Py]| _sentence_at(args))),
            "_the_subject" => Some(py_func("_the_subject", |args: &[Py]| _the_subject(args))),
            "_the_way_in" => Some(py_func("_the_way_in", |args: &[Py]| _the_way_in(args))),
            "_thread_text" => Some(py_func("_thread_text", |args: &[Py]| _thread_text(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "asks_who" => Some(py_func("asks_who", |args: &[Py]| asks_who(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}

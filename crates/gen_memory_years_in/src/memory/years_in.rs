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


thread_local! { static G_BEEN_AT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEEN_AT_IT_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_STINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_JOB_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_GERUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LISTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.years_in: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_BEEN_AT_IT() -> Py { ensure_init(); G_BEEN_AT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEEN_AT_IT(v: Py) { ensure_init(); G_BEEN_AT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_BEEN_AT_IT_BARE() -> Py { ensure_init(); G_BEEN_AT_IT_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEEN_AT_IT_BARE(v: Py) { ensure_init(); G_BEEN_AT_IT_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_STINT() -> Py { ensure_init(); G_NOT_A_STINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STINT(v: Py) { ensure_init(); G_NOT_A_STINT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_LENGTH() -> Py { ensure_init(); G_THEIR_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_LENGTH(v: Py) { ensure_init(); G_THEIR_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_JOB_NOUNS() -> Py { ensure_init(); G_THE_JOB_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_JOB_NOUNS(v: Py) { ensure_init(); G_THE_JOB_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PRONOUN() -> Py { ensure_init(); G_THE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PRONOUN(v: Py) { ensure_init(); G_THE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_GERUND() -> Py { ensure_init(); G__A_GERUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_GERUND(v: Py) { ensure_init(); G__A_GERUND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LEAD_IN() -> Py { ensure_init(); G__A_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LEAD_IN(v: Py) { ensure_init(); G__A_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LENGTH() -> Py { ensure_init(); G__A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH(v: Py) { ensure_init(); G__A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TAIL() -> Py { ensure_init(); G__A_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TAIL(v: Py) { ensure_init(); G__A_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__LISTEN() -> Py { ensure_init(); G__LISTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LISTEN(v: Py) { ensure_init(); G__LISTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBERS() -> Py { ensure_init(); G__NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBERS(v: Py) { ensure_init(); G__NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__TENS() -> Py { ensure_init(); G__TENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TENS(v: Py) { ensure_init(); G__TENS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How long they have been doing the job they told me about, said back as a person would.\n\nTwo turns of a chat:\n\n    im a teacher\n    i have been doing it for 10 years\n\nThe first turn is answered well (\"Teacher - good to know. What do you like most about it?\"). The\nsecond is not: the whole bot says \"Got it - you have been doing it for 10 years\", which is the\nline said back with a receipt on it. It answers nothing, asks nothing, and does not know that ten\nyears is a thing worth a word. The person is not filing a fact, they are telling me how long they\nhave stuck at something.\n\nSo this reads the length, finds the job it is the length of - in the line (\"i've been teaching\")\nor in the store, where the job was filed two turns ago - and says something about it. It is a\nnarrow reading on purpose:\n\n- the line has to be only the length statement: no question, no ask, nothing else in it. \"i have\n  been doing it for 10 years and i hate it\" is a line about the last half, and the store's own\n  readers keep it.\n- there has to be a job: one filed in this chat, or one the line names with a job word. \"i've been\n  waiting for 10 years\" is a length, and it is not a job, so it is left alone.\n- the length has to be years or months. \"i've been doing it for 10 minutes\" is not a stint.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._A_LEAD_IN"); }
    s__A_LEAD_IN(py_str("(?:(?:so|well|oh|and|actually|tbh|honestly|yeah|yes)\\s+,?\\s*)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._LISTEN"); }
    s__LISTEN(py_str("listen\\s*,\\s*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._A_LENGTH"); }
    s__A_LENGTH(py_str("(?:for\\s+|about\\s+|nearly\\s+|over\\s+|almost\\s+)?(?P<count>\\d{1,2}|an?|one|a few|a couple of|several|many)\\s*(?P<unit>years?|yrs?|months?)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._A_TAIL"); }
    s__A_TAIL(py_str("(?:\\s+(?:now|already|so far|in total|and counting|today|lol|haha|hah))?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.BEEN_AT_IT"); }
    s_BEEN_AT_IT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&g__A_LEAD_IN(), &py_str("(?:i(?:'ve| have)?|ive)\\s+been\\s+(?P<doing>[a-z][a-z' ]{0,24}?)\\s+")), &g__A_LENGTH()), &g__A_TAIL()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.BEEN_AT_IT_BARE"); }
    s_BEEN_AT_IT_BARE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&g__A_LEAD_IN(), &py_str("(?:been\\s+|doing\\s+it\\s+)(?P<doing>[a-z][a-z' ]{0,20}?)?\\s*")), &g__A_LENGTH()), &g__A_TAIL()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.NOT_A_STINT"); }
    s_NOT_A_STINT(stdlib_call("re", "compile", &[py_str("\\b(?:minutes?|mins?|hours?|hrs?|days?|weeks?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._A_GERUND"); }
    s__A_GERUND(stdlib_call("re", "compile", &[py_str("^(?P<stem>[a-z]{3,}?)(?:ing|in)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.THE_JOB_NOUNS"); }
    s_THE_JOB_NOUNS(call_builtin("frozenset", &[pyrt::xrefs::xget("memory.facts", "JOB_WORDS")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("zero"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve"), py_str("thirteen"), py_str("fourteen"), py_str("fifteen"), py_str("sixteen"), py_str("seventeen"), py_str("eighteen"), py_str("nineteen")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._NUMBERS"); }
    s__NUMBERS(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in._TENS"); }
    s__TENS(py_tuple(vec![py_str(""), py_str(""), py_str("twenty"), py_str("thirty"), py_str("forty"), py_str("fifty"), py_str("sixty"), py_str("seventy"), py_str("eighty"), py_str("ninety"), py_str("")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.THE_PRONOUN"); }
    s_THE_PRONOUN(stdlib_call("re", "compile", &[py_str("^(?:so\\s+)?(?P<who>she|he|they)(?:'s|s| has| have|'ve| have been|s been)?\\s+been\\s+(?P<doing>[a-z][a-z' ]{0,24}?)\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.years_in.THEIR_LENGTH"); }
    s_THEIR_LENGTH(stdlib_call("re", "compile", &[py_str("^(?:so\\s+)?(?P<who>she|he|they)(?:'s|s| has|'ve| have)?\\s+been\\s+(?P<doing>[a-z][a-z' ]{0,24}?)\\s+(?:(?:for|about|nearly|over|almost)\\s+)?(?P<count>\\d{1,2}|an?|one|a few|a couple of|several|many|years)\\s*(?P<unit>years?|yrs?|months?)?\\b(?:\\s+(?:now|already|so far|in total|and counting|lol|haha))?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_job_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the doing is a job word or the gerund of one.");
    low = mcall(&mcall(&doing.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_THE_JOB_NOUNS(), &low.clone()))) {
    return Py::Bool(true);
    }
    match_ = mcall(&g__A_GERUND(), "match", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    stem = mcall(&match_.clone(), "group", &[py_str("stem")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_JOB_NOUNS()) { let mut word: Py = __item.clone(); __out.push({ let __b = stem.clone(); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[stem.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _alike(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut job: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "job") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: job"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let _ = py_str("The line's doing and the filed job are the same work: \"teaching\" and \"teacher\".");
    a = stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&doing.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    b = stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&job.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&b.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&b.clone(), &a.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&a.clone(), &b.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_eq(&slice_or_raise(&a.clone(), &Py::None, &Py::Int(4), &Py::None), &slice_or_raise(&b.clone(), &Py::None, &Py::Int(4), &Py::None)));
    Py::None
}
pub fn _the_filed_job(args: &[Py]) -> Py {
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
    let mut fact: Py = Py::None;
    let mut job: Py = Py::None;
    let mut log: Py = Py::None;
    let mut user: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The job this chat filed, or \"\".");
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    job = mcall(&user.clone(), "get", &[py_str("job")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[job.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&job.clone(), "strip", &[]) } }) {
    return mcall(&job.clone(), "strip", &[]);
    }
    }
    log = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("facts_log")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = log.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    fact = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[fact.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&fact.clone(), "get", &[py_str("key")]), &py_str("job"))) } }) {
    value = mcall(&fact.clone(), "get", &[py_str("value")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return mcall(&value.clone(), "strip", &[]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_doing_and_the_length(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut filed: Py = Py::None;
    let mut filing: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The job and the length this line is the length of, or (\"\", 0, \"years\").");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    said = mcall(&text.clone(), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    if py_truthy(&mcall(&g_NOT_A_STINT(), "search", &[said.clone()])) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    match_ = { let __b = mcall(&g_BEEN_AT_IT(), "fullmatch", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BEEN_AT_IT_BARE(), "fullmatch", &[said.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    doing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("doing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the)\\s+")); __args.push(py_str("")); __args.push(doing.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    filed = _the_filed_job(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&filed.clone()))) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    named = Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[py_str("(?:doing\\s+)?(?:it|that|this|the\\s+job)"), { let __b = doing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_alike(&[doing.clone(), filed.clone()]))) } }) {
    return py_tuple(vec![py_str(""), Py::Int(0), py_str("years")]);
    }
    filing = filed.clone();
    count = _the_count(&[mcall(&match_.clone(), "group", &[py_str("count")])]);
    unit = if py_truthy(&mcall(&mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "startswith", &[py_tuple(vec![py_str("month"), py_str("mnth")])])) { py_str("months") } else { py_str("years") };
    return py_tuple(vec![filing.clone(), count.clone(), unit.clone()]);
    Py::None
}
pub fn _the_count(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("The figure a person said, as a number.");
    low = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&low.clone(), "isdigit", &[])) {
    return call_builtin("int", &[low.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_contains(&g__NUMBERS(), &low.clone()))) {
    return mcall(&g__NUMBERS(), "index", &[low.clone()]);
    }
    return mcall(&py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("a few"), Py::Int(3)), (py_str("a couple of"), Py::Int(2)), (py_str("several"), Py::Int(4)), (py_str("many"), Py::Int(8))]), "get", &[low.clone(), Py::Int(0)]);
    Py::None
}
pub fn _the_figure_in_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut ones: Py = Py::None;
    let mut tens: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("Ten, twelve, fifteen: a length as a person says it.");
    whole = call_builtin("int", &[call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __cmp11: Py = whole.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp11)) { op_lt(&__cmp11, &call_builtin("len", &[g__NUMBERS()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) } }) {
    return mcall(&geti(&g__NUMBERS(), &whole.clone()), "capitalize", &[]);
    }
    if py_truthy(&op_lt(&whole.clone(), &Py::Int(100))) {
    let __unpk2 = py_unpack(&call_builtin("divmod", &[whole.clone(), Py::Int(10)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    tens = __unpk2[0].clone();
    ones = __unpk2[1].clone();
    return mcall(&op_add(&TENS_WORD(&[tens.clone()]), &if py_truthy(&ones.clone()) { op_add(&py_str("-"), &geti(&g__NUMBERS(), &ones.clone())) } else { py_str("") }), "capitalize", &[]);
    }
    return call_builtin("str", &[whole.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn TENS_WORD(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tens: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tens") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tens"))) };
    let _ = py_str("The tens a figure is said in: 2 -> \"twenty\".");
    return if py_truthy(&{ let __cmp11: Py = tens.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp11)) { op_lt(&__cmp11, &call_builtin("len", &[g__TENS()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) } }) { geti(&g__TENS(), &tens.clone()) } else { call_builtin("str", &[tens.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    Py::None
}
pub fn _with_an(args: &[Py]) -> Py {
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
    let _ = py_str("\"an engineer\", \"a teacher\".");
    return op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])))) { py_str("an ") } else { py_str("a ") }, &word.clone());
    Py::None
}
pub fn _the_stint(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut job: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "job") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: job"))) };
    let mut count: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut unit: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { py_str("years") };
    let mut f: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut frames: Py = Py::None;
    let _ = py_str("The sentence the length deserves, in the words the job is said in.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("months"))); if !py_truthy(&__b) { __b } else { op_lt(&count.clone(), &Py::Int(24)) } }) {
    figure = _the_figure_in_words(&[count.clone()]);
    if py_truthy(&op_lt(&count.clone(), &Py::Int(12))) {
    frames = py_tuple(vec![py_str("{figure} months {at_it} - the first year is the hardest stretch of it."), py_str("{figure} months {at_it}. That is still the part where everyone is finding their feet."), py_str("{figure} months {at_it} - how are you finding it so far?")]);
    } else {
    frames = py_tuple(vec![py_str("{figure} months {at_it} - past the first year, then."), py_str("{figure} months {at_it}. The first year is the one that tests everyone."), py_str("{figure} months {at_it} - what has got easier?")]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut f: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure.clone()), ("at_it".to_string(), _as_a(&[job.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&f.clone(), "format", args))(&__args) }); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&job.clone(), -1, "")); py_string(__s) }]);
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("months")))) {
    count = op_floordiv(&count.clone(), &Py::Int(12));
    }
    figure = _the_figure_in_words(&[count.clone()]);
    if py_truthy(&op_lt(&count.clone(), &Py::Float(1.5))) {
    frames = py_tuple(vec![py_str("A year {at_it} - the first one is the hardest."), py_str("A year in already. It gets easier from here, or at least more familiar."), py_str("A year {at_it}. You've got past the bit where everyone feels useless.")]);
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut f: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("at_it".to_string(), _as_a(&[job.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&f.clone(), "format", args))(&__args) }); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&job.clone(), -1, "")); py_string(__s) }]);
    }
    frames = py_tuple(vec![py_str("{figure} years {at_it} - that's a long stretch."), py_str("{figure} years {at_it}. You must know it inside out by now."), py_str("{figure} years {at_it} is more than most people stick at anything."), py_str("{figure} years {at_it} - what keeps you at it?")]);
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut f: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure.clone()), ("at_it".to_string(), _as_a(&[job.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&f.clone(), "format", args))(&__args) }); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&job.clone(), -1, "")); py_string(__s) }]);
    Py::None
}
pub fn _as_a(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut job: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "job") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: job"))) };
    let _ = py_str("\"as a teacher\", \"as an engineer\".");
    return op_add(&py_str("as "), &_with_an(&[job.clone()]));
    Py::None
}
pub fn the_years_in_the_doing(args: &[Py]) -> Py {
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
    let mut job: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The reply for a line that is only how long they have been at the job, or \"\".");
    let __unpk1 = py_unpack(&_the_doing_and_the_length(&[text.clone(), memory.clone()]), 3, None);
    job = __unpk1[0].clone();
    count = __unpk1[1].clone();
    unit = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&job.clone())); if py_truthy(&__b) { __b } else { op_le(&count.clone(), &Py::Int(0)) } }) {
    return py_str("");
    }
    return _the_stint(&[job.clone(), count.clone(), memory.clone(), text.clone(), unit.clone()]);
    Py::None
}
pub fn _the_person_holding_a_job(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut fact: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut job: Py = Py::None;
    let mut log: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The most recent person of theirs with a job filed, whose pronoun fits, or (\"\", \"\").");
    fits = mcall(&py_dict(vec![(py_str("she"), pyrt::xrefs::xget("memory.facts", "SHE")), (py_str("he"), pyrt::xrefs::xget("memory.facts", "HE"))]), "get", &[who.clone(), op_bitor(&pyrt::xrefs::xget("memory.facts", "SHE"), &pyrt::xrefs::xget("memory.facts", "HE"))]);
    log = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("facts_log")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = log.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    fact = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[fact.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&fact.clone(), "get", &[py_str("key")]), &py_str("other"))) } }) {
    continue;
    }
    value = mcall(&fact.clone(), "get", &[py_str("value")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    relation = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    job = mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("job")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&relation.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&job.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&fits.clone(), &relation.clone()))) {
    return py_tuple(vec![relation.clone(), job.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _their_stint(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut relation: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "relation") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: relation"))) };
    let mut job: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "job") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: job"))) };
    let mut count: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut said: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut unit: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { py_str("years") };
    let mut f: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut low: Py = Py::None;
    let mut span: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("The sentence their length of service deserves.");
    theirs = op_add(&py_str("Your "), &relation.clone());
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    frames = py_tuple(vec![py_str("{theirs} has been at it a while, then. That is not nothing."), py_str("{theirs} has put the years in - no wonder it comes up.")]);
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut f: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("theirs".to_string(), theirs.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&f.clone(), "format", args))(&__args) }); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&job.clone(), -1, "")); py_string(__s) }]);
    }
    span = if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("months"))); if !py_truthy(&__b) { __b } else { op_lt(&count.clone(), &Py::Int(24)) } }) { py_str("months") } else { py_str("years") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&span.clone(), &py_str("years"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&unit.clone(), &py_str("months"))) } }) {
    count = op_floordiv(&count.clone(), &Py::Int(12));
    }
    figure = _the_figure_in_words(&[count.clone()]);
    low = mcall(&figure.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&span.clone(), &py_str("months"))); if !py_truthy(&__b) { __b } else { op_lt(&count.clone(), &Py::Int(12)) } }) {
    frames = py_tuple(vec![py_str("{theirs} has been {a_job} for {low} months - early days, then."), py_str("{low} months in as {a_job} for {theirs_lower}. The first year is the hard one."), py_str("{theirs} has been {a_job} for {low} months - how is it going?")]);
    } else {
    frames = py_tuple(vec![op_add(&op_add(&py_str("{theirs} has been {a_job} for {low} "), &span.clone()), &py_str(" - that's a long stretch.")), op_add(&op_add(&py_str("{figure} "), &span.clone()), &py_str(" is a long time for {theirs_lower} to stay in one job.")), op_add(&op_add(&py_str("{theirs} has been {a_job} for {low} "), &span.clone()), &py_str(". That is a lot of years behind them."))]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut f: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("theirs".to_string(), theirs.clone()), ("theirs_lower".to_string(), mcall(&theirs.clone(), "lower", &[])), ("figure".to_string(), figure.clone()), ("low".to_string(), low.clone()), ("a_job".to_string(), _with_an(&[job.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&f.clone(), "format", args))(&__args) }); } py_list(__out) })() }, { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&job.clone(), -1, "")); py_string(__s) }]);
    Py::None
}
pub fn the_years_in_their_doing(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut job: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut months: Py = Py::None;
    let mut named: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The reply for a line that is only how long a person of theirs has been at their job, or \"\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    said = mcall(&text.clone(), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return py_str("");
    }
    match_ = mcall(&g_THEIR_LENGTH(), "fullmatch", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    unit = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("years") } }, "lower", &[]);
    let __unpk1 = py_unpack(&_the_person_holding_a_job(&[memory.clone(), mcall(&mcall(&match_.clone(), "group", &[py_str("who")]), "lower", &[])]), 2, None);
    relation = __unpk1[0].clone();
    job = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&relation.clone()))) {
    return py_str("");
    }
    doing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("doing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the)\\s+")); __args.push(py_str("")); __args.push(doing.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    named = Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[py_str("(?:doing\\s+)?(?:it|that|this|the\\s+job)"), { let __b = doing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_alike(&[doing.clone(), job.clone()]))) } }) {
    return py_str("");
    }
    count = _the_count(&[mcall(&match_.clone(), "group", &[py_str("count")])]);
    months = mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("month"), py_str("mnth")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("years")]), &mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("count")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]))) } }) {
    return py_str("");
    }
    return _their_stint(&[relation.clone(), job.clone(), count.clone(), memory.clone(), said.clone(), if py_truthy(&months.clone()) { py_str("months") } else { py_str("years") }]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.years_in".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BEEN_AT_IT" => Some(g_BEEN_AT_IT()),
            "BEEN_AT_IT_BARE" => Some(g_BEEN_AT_IT_BARE()),
            "NOT_A_STINT" => Some(g_NOT_A_STINT()),
            "TENS_WORD" => Some(py_func("TENS_WORD", |args: &[Py]| TENS_WORD(args))),
            "THEIR_LENGTH" => Some(g_THEIR_LENGTH()),
            "THE_JOB_NOUNS" => Some(g_THE_JOB_NOUNS()),
            "THE_PRONOUN" => Some(g_THE_PRONOUN()),
            "_A_GERUND" => Some(g__A_GERUND()),
            "_A_LEAD_IN" => Some(g__A_LEAD_IN()),
            "_A_LENGTH" => Some(g__A_LENGTH()),
            "_A_TAIL" => Some(g__A_TAIL()),
            "_LISTEN" => Some(g__LISTEN()),
            "_NUMBERS" => Some(g__NUMBERS()),
            "_TENS" => Some(g__TENS()),
            "_a_job_word" => Some(py_func("_a_job_word", |args: &[Py]| _a_job_word(args))),
            "_alike" => Some(py_func("_alike", |args: &[Py]| _alike(args))),
            "_as_a" => Some(py_func("_as_a", |args: &[Py]| _as_a(args))),
            "_the_count" => Some(py_func("_the_count", |args: &[Py]| _the_count(args))),
            "_the_doing_and_the_length" => Some(py_func("_the_doing_and_the_length", |args: &[Py]| _the_doing_and_the_length(args))),
            "_the_figure_in_words" => Some(py_func("_the_figure_in_words", |args: &[Py]| _the_figure_in_words(args))),
            "_the_filed_job" => Some(py_func("_the_filed_job", |args: &[Py]| _the_filed_job(args))),
            "_the_person_holding_a_job" => Some(py_func("_the_person_holding_a_job", |args: &[Py]| _the_person_holding_a_job(args))),
            "_the_stint" => Some(py_func("_the_stint", |args: &[Py]| _the_stint(args))),
            "_their_stint" => Some(py_func("_their_stint", |args: &[Py]| _their_stint(args))),
            "_with_an" => Some(py_func("_with_an", |args: &[Py]| _with_an(args))),
            "pick" => Some(g_pick()),
            "the_years_in_the_doing" => Some(py_func("the_years_in_the_doing", |args: &[Py]| the_years_in_the_doing(args))),
            "the_years_in_their_doing" => Some(py_func("the_years_in_their_doing", |args: &[Py]| the_years_in_their_doing(args))),
            "facts" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BEEN_AT_IT" => s_BEEN_AT_IT(v),
            "BEEN_AT_IT_BARE" => s_BEEN_AT_IT_BARE(v),
            "NOT_A_STINT" => s_NOT_A_STINT(v),
            "THEIR_LENGTH" => s_THEIR_LENGTH(v),
            "THE_JOB_NOUNS" => s_THE_JOB_NOUNS(v),
            "THE_PRONOUN" => s_THE_PRONOUN(v),
            "_A_GERUND" => s__A_GERUND(v),
            "_A_LEAD_IN" => s__A_LEAD_IN(v),
            "_A_LENGTH" => s__A_LENGTH(v),
            "_A_TAIL" => s__A_TAIL(v),
            "_LISTEN" => s__LISTEN(v),
            "_NUMBERS" => s__NUMBERS(v),
            "_TENS" => s__TENS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.years_in", module_obj);
    pyrt::xrefs::reg_fn("memory.years_in", "_a_job_word", py_func("_a_job_word", |args: &[Py]| _a_job_word(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_alike", py_func("_alike", |args: &[Py]| _alike(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_filed_job", py_func("_the_filed_job", |args: &[Py]| _the_filed_job(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_doing_and_the_length", py_func("_the_doing_and_the_length", |args: &[Py]| _the_doing_and_the_length(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_count", py_func("_the_count", |args: &[Py]| _the_count(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_figure_in_words", py_func("_the_figure_in_words", |args: &[Py]| _the_figure_in_words(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "TENS_WORD", py_func("TENS_WORD", |args: &[Py]| TENS_WORD(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_with_an", py_func("_with_an", |args: &[Py]| _with_an(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_stint", py_func("_the_stint", |args: &[Py]| _the_stint(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_as_a", py_func("_as_a", |args: &[Py]| _as_a(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "the_years_in_the_doing", py_func("the_years_in_the_doing", |args: &[Py]| the_years_in_the_doing(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_the_person_holding_a_job", py_func("_the_person_holding_a_job", |args: &[Py]| _the_person_holding_a_job(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "_their_stint", py_func("_their_stint", |args: &[Py]| _their_stint(args)));
    pyrt::xrefs::reg_fn("memory.years_in", "the_years_in_their_doing", py_func("the_years_in_their_doing", |args: &[Py]| the_years_in_their_doing(args)));
    pyrt::xrefs::reg_access("memory.years_in", "BEEN_AT_IT", g_BEEN_AT_IT, s_BEEN_AT_IT);
    pyrt::xrefs::reg_access("memory.years_in", "BEEN_AT_IT_BARE", g_BEEN_AT_IT_BARE, s_BEEN_AT_IT_BARE);
    pyrt::xrefs::reg_access("memory.years_in", "NOT_A_STINT", g_NOT_A_STINT, s_NOT_A_STINT);
    pyrt::xrefs::reg_access("memory.years_in", "THEIR_LENGTH", g_THEIR_LENGTH, s_THEIR_LENGTH);
    pyrt::xrefs::reg_access("memory.years_in", "THE_JOB_NOUNS", g_THE_JOB_NOUNS, s_THE_JOB_NOUNS);
    pyrt::xrefs::reg_access("memory.years_in", "THE_PRONOUN", g_THE_PRONOUN, s_THE_PRONOUN);
    pyrt::xrefs::reg_access("memory.years_in", "_A_GERUND", g__A_GERUND, s__A_GERUND);
    pyrt::xrefs::reg_access("memory.years_in", "_A_LEAD_IN", g__A_LEAD_IN, s__A_LEAD_IN);
    pyrt::xrefs::reg_access("memory.years_in", "_A_LENGTH", g__A_LENGTH, s__A_LENGTH);
    pyrt::xrefs::reg_access("memory.years_in", "_A_TAIL", g__A_TAIL, s__A_TAIL);
    pyrt::xrefs::reg_access("memory.years_in", "_LISTEN", g__LISTEN, s__LISTEN);
    pyrt::xrefs::reg_access("memory.years_in", "_NUMBERS", g__NUMBERS, s__NUMBERS);
    pyrt::xrefs::reg_access("memory.years_in", "_TENS", g__TENS, s__TENS);
    pyrt::xrefs::reg_access("memory.years_in", "pick", g_pick, s_pick);
}

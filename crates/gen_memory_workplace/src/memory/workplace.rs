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


thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHIFT_NOT_A_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USED_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_THE_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FROM_HOME_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.workplace: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHIFT_NOT_A_TRADE() -> Py { ensure_init(); G_A_SHIFT_NOT_A_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHIFT_NOT_A_TRADE(v: Py) { ensure_init(); G_A_SHIFT_NOT_A_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_I_WORK() -> Py { ensure_init(); G_I_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_WORK(v: Py) { ensure_init(); G_I_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE() -> Py { ensure_init(); G_NOT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE(v: Py) { ensure_init(); G_NOT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_USED_TO() -> Py { ensure_init(); G_USED_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USED_TO(v: Py) { ensure_init(); G_USED_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_THE_DETAIL() -> Py { ensure_init(); G_WITH_THE_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_THE_DETAIL(v: Py) { ensure_init(); G_WITH_THE_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__FROM_HOME_TAILS() -> Py { ensure_init(); G__FROM_HOME_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FROM_HOME_TAILS(v: Py) { ensure_init(); G__FROM_HOME_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g__TAILS() -> Py { ensure_init(); G__TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAILS(v: Py) { ensure_init(); G__TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Where they work, said as a fact about them and not as a how-to for the place.\n\nTwo lines from one chat, both about the person typing:\n\n    im a teacher\n    i work in a primary school\n      before: \"Apply through your council's school admissions service, not to the school: the\n               deadline is 31 October for a secondary place and 15 January ...\"\n      after:  \"A teacher in a primary school - how long have you been there?\"\n\n    im a teacher\n    i work from home\n      before: \"Ask in person after class with one specific thing ...\"\n      after:  \"A teacher working from home - does it leave you much time for yourself?\"\n\nThe store files the place (`user[\"workplace\"]`), so the fact is not lost; what was lost was the\nreply. A line that says where the work is answers in the frame of the job the chat already holds.\n\nIt reads only a line that is about the person's own work and nothing else. It passes on a\nquestion (\"how do i get a job in a school\"), on somebody else's work (\"my sister works at the\nhospital\"), on the conditions rather than the place (\"i work long hours\", \"i work nights\"), on a\njob said as the work (\"i work as a nurse\" - the store answers that one), and on a line that files\nanother fact as well, because then the thing said back is two things.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.I_WORK"); }
    s_I_WORK(stdlib_call("re", "compile", &[py_str("^\\s*(?:so|and|oh|well|ok|okay|right|actually|anyway|hi|hey|hello)?[\\s,]*\\b(?:i|i'?m)\\s+(?:still\\s+|now\\s+)?(?:work|working)\\s+(?P<rest>[^.!?]{2,60})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.NOT_MINE"); }
    s_NOT_MINE(stdlib_call("re", "compile", &[py_str("\\b(?:(?:she|he|they|we|his|her|their|our|my)\\b[^.!?]{0,20}?(?:works|worked|working)|(?:they|we)\\s+work)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.A_QUESTION"); }
    s_A_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:how|what|where|why|when|who|which|can|could|should|do|does|did|is|are|will|would|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.NOT_A_PLACE"); }
    s_NOT_A_PLACE(stdlib_call("re", "compile", &[py_str("^\\s*(?:as|with|too|so|hard|late|early|flat|out|up|nights?|night\\s+shifts?|shifts|evenings?|weekends?|part[- ]time|full[- ]time|long\\s+hours|all\\s+hours|away|abroad|remotely|on\\s+my\\s+own|for\\s+myself|for\\s+a\\s+living|in\\s+a\\s+team|from\\s+home\\s+now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.A_PLACE"); }
    s_A_PLACE(stdlib_call("re", "compile", &[py_str("^\\s*(?P<prep>in|at|for|from)\\s+(?P<place>[a-z0-9][a-z0-9' \\u2019,&/.-]{1,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.WITH_THE_DETAIL"); }
    s_WITH_THE_DETAIL(stdlib_call("re", "compile", &[py_str("\\b(?:shift|shifts|nights?|hours?|overtime|weekends?|all\\s+day|all\\s+week|every\\s+day|part[- ]time|full[- ]time|long\\s+days|on\\s+call)\\b|\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.USED_TO"); }
    s_USED_TO(stdlib_call("re", "compile", &[py_str("\\b(?:used\\s+to|use\\s+to|stopped|quit|left)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace._TAILS"); }
    s__TAILS(py_tuple(vec![py_str("how long have you been there?"), py_str("what's the best part of the week there?"), py_str("does it leave you much time for yourself?"), py_str("busy one, I should think."), py_str("what do you do there?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace._FROM_HOME_TAILS"); }
    s__FROM_HOME_TAILS(py_tuple(vec![py_str("how do you keep the day from running into the evening?"), py_str("the commute is hard to beat."), py_str("quiet, or people at the door all day?"), py_str("what's the best part of it?"), py_str("does it give you more of the day back?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.workplace.A_SHIFT_NOT_A_TRADE"); }
    s_A_SHIFT_NOT_A_TRADE(stdlib_call("re", "compile", &[py_str("^(?:nights?|days?|evenings?|weekends?|shifts?|lates?|early|mornings?|afternoons?|part[- ]time|full[- ]time|hours|overtime|on\\s+call|long\\s+hours)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_same_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a"))) };
    let mut b: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "b") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: b"))) };
    let mut clean: Py = Py::None;
    let _ = py_str("Whether two lines are the same after case and spacing: \"I work in a school.\".");
pub fn __nested_clean_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut a: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut b: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut clean: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "said").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    Py::None
}
    clean = {{ let a = a.clone(); let b = b.clone(); let clean = clean.clone(); py_func("clean", move |args: &[Py]| { let mut full = vec![a.clone(), b.clone(), clean.clone()]; full.extend_from_slice(args); __nested_clean_0(&full) }) }};
    return Py::Bool(py_eq(&py_call(&clean, &[a.clone()]), &py_call(&clean, &[b.clone()])));
    Py::None
}
pub fn _the_job_the_chat_holds(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut job: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("The job the person said of themselves of their own work, in the words they used.\n\n    It must come from an earlier turn: this module runs after the store has observed the line, and\n    the store files a job for \"i work in a primary school\" too, which is the place and not the\n    trade. The last job entry that is not this line is the one the chat holds.\n    ");
    entries = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("facts_log")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("job"))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("value")])))) {
    continue;
    }
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { _the_same_line(&[mcall(&entry.clone(), "get", &[py_str("text")]), text.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_SHIFT_NOT_A_TRADE(), "match", &[mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("value"))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    continue;
    }
    return mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("value"))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    return py_str("");
    }
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]) } else { Py::None };
    job = if py_truthy(&call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&user.clone(), "get", &[py_str("job")]) } else { py_str("") };
    return mcall(&call_builtin("str", &[{ let __b = job.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn _with_an_article(args: &[Py]) -> Py {
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
    let mut article: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The job the way it opens a sentence: \"a teacher\", \"an engineer\".");
    word = mcall(&call_builtin("str", &[{ let __b = job.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])))) { py_str("an") } else { py_str("a") };
    return op_add(&op_add(&article.clone(), &py_str(" ")), &word.clone());
    Py::None
}
pub fn _a_clean_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The place with its trailing punctuation and filler taken off: \"a primary school\".");
    word = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:and|but|because|so|when|while|although|though|which|where|if)\\b")); __args.push(call_builtin("str", &[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    word = stdlib_call("re", "sub", &[py_str("[\\s,.;:!?'\\u2019]+$"), py_str(""), mcall(&word.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:anyway|now|though|too|still)$")); __args.push(py_str("")); __args.push(word.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = mcall(&word.clone(), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    word = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(5), &Py::None)]);
    }
    return mcall(&word.clone(), "strip", &[]);
    Py::None
}
pub fn _the_place_the_line_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut place: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The preposition and the place, when the line says where the person's work is.");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_I_WORK(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_QUESTION(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_USED_TO(), "search", &[said.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_MINE(), "search", &[said.clone()])) {
    return Py::None;
    }
    found = mcall(&g_I_WORK(), "search", &[said.clone()]);
    rest = mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_PLACE(), "match", &[rest.clone()]) } }) {
    return Py::None;
    }
    place = mcall(&g_A_PLACE(), "match", &[rest.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if py_truthy(&__b) { __b } else { mcall(&g_WITH_THE_DETAIL(), "search", &[mcall(&place.clone(), "group", &[py_str("place")])]) } }) {
    return Py::None;
    }
    clean = _a_clean_place(&[mcall(&place.clone(), "group", &[py_str("place")])]);
    if py_truthy(&op_lt(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    tail = mcall(&slice_or_raise(&rest.clone(), &mcall(&place.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.;:!?'’")]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&tail.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    return py_tuple(vec![mcall(&mcall(&place.clone(), "group", &[py_str("prep")]), "lower", &[]), clean.clone()]);
    Py::None
}
pub fn the_place_the_person_works(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut home: Py = Py::None;
    let mut job: Py = Py::None;
    let mut place: Py = Py::None;
    let mut prep: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The reply to a line that says where the person's work is, or None.");
    found = _the_place_the_line_said(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    job = _the_job_the_chat_holds(&[memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&job.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    prep = __unpk1[0].clone();
    place = __unpk1[1].clone();
    home = { let __b = Py::Bool(py_eq(&prep.clone(), &py_str("from"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("home"), py_str("the house")]), &mcall(&place.clone(), "lower", &[]))) } };
    if py_truthy(&home.clone()) {
    return op_add(&op_add(&mcall(&_with_an_article(&[job.clone()]), "capitalize", &[]), &py_str(" working from home - ")), &pyrt::xrefs::xcall("compose", "pick", &[g__FROM_HOME_TAILS(), memory.clone(), text.clone()]));
    }
    where_ = op_add(&op_add(&prep.clone(), &py_str(" ")), &place.clone());
    return op_add(&op_add(&op_add(&op_add(&mcall(&_with_an_article(&[job.clone()]), "capitalize", &[]), &py_str(" ")), &where_.clone()), &py_str(" - ")), &pyrt::xrefs::xcall("compose", "pick", &[g__TAILS(), memory.clone(), text.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.workplace".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_PLACE" => Some(g_A_PLACE()),
            "A_QUESTION" => Some(g_A_QUESTION()),
            "A_SHIFT_NOT_A_TRADE" => Some(g_A_SHIFT_NOT_A_TRADE()),
            "I_WORK" => Some(g_I_WORK()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NOT_MINE" => Some(g_NOT_MINE()),
            "USED_TO" => Some(g_USED_TO()),
            "WITH_THE_DETAIL" => Some(g_WITH_THE_DETAIL()),
            "_FROM_HOME_TAILS" => Some(g__FROM_HOME_TAILS()),
            "_TAILS" => Some(g__TAILS()),
            "_a_clean_place" => Some(py_func("_a_clean_place", |args: &[Py]| _a_clean_place(args))),
            "_the_job_the_chat_holds" => Some(py_func("_the_job_the_chat_holds", |args: &[Py]| _the_job_the_chat_holds(args))),
            "_the_place_the_line_said" => Some(py_func("_the_place_the_line_said", |args: &[Py]| _the_place_the_line_said(args))),
            "_the_same_line" => Some(py_func("_the_same_line", |args: &[Py]| _the_same_line(args))),
            "_with_an_article" => Some(py_func("_with_an_article", |args: &[Py]| _with_an_article(args))),
            "pick" => Some(g_pick()),
            "the_place_the_person_works" => Some(py_func("the_place_the_person_works", |args: &[Py]| the_place_the_person_works(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_PLACE" => s_A_PLACE(v),
            "A_QUESTION" => s_A_QUESTION(v),
            "A_SHIFT_NOT_A_TRADE" => s_A_SHIFT_NOT_A_TRADE(v),
            "I_WORK" => s_I_WORK(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NOT_MINE" => s_NOT_MINE(v),
            "USED_TO" => s_USED_TO(v),
            "WITH_THE_DETAIL" => s_WITH_THE_DETAIL(v),
            "_FROM_HOME_TAILS" => s__FROM_HOME_TAILS(v),
            "_TAILS" => s__TAILS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.workplace", module_obj);
    pyrt::xrefs::reg_fn("memory.workplace", "_the_same_line", py_func("_the_same_line", |args: &[Py]| _the_same_line(args)));
    pyrt::xrefs::reg_fn("memory.workplace", "_the_job_the_chat_holds", py_func("_the_job_the_chat_holds", |args: &[Py]| _the_job_the_chat_holds(args)));
    pyrt::xrefs::reg_fn("memory.workplace", "_with_an_article", py_func("_with_an_article", |args: &[Py]| _with_an_article(args)));
    pyrt::xrefs::reg_fn("memory.workplace", "_a_clean_place", py_func("_a_clean_place", |args: &[Py]| _a_clean_place(args)));
    pyrt::xrefs::reg_fn("memory.workplace", "_the_place_the_line_said", py_func("_the_place_the_line_said", |args: &[Py]| _the_place_the_line_said(args)));
    pyrt::xrefs::reg_fn("memory.workplace", "the_place_the_person_works", py_func("the_place_the_person_works", |args: &[Py]| the_place_the_person_works(args)));
    pyrt::xrefs::reg_access("memory.workplace", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("memory.workplace", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("memory.workplace", "A_SHIFT_NOT_A_TRADE", g_A_SHIFT_NOT_A_TRADE, s_A_SHIFT_NOT_A_TRADE);
    pyrt::xrefs::reg_access("memory.workplace", "I_WORK", g_I_WORK, s_I_WORK);
    pyrt::xrefs::reg_access("memory.workplace", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.workplace", "NOT_MINE", g_NOT_MINE, s_NOT_MINE);
    pyrt::xrefs::reg_access("memory.workplace", "USED_TO", g_USED_TO, s_USED_TO);
    pyrt::xrefs::reg_access("memory.workplace", "WITH_THE_DETAIL", g_WITH_THE_DETAIL, s_WITH_THE_DETAIL);
    pyrt::xrefs::reg_access("memory.workplace", "_FROM_HOME_TAILS", g__FROM_HOME_TAILS, s__FROM_HOME_TAILS);
    pyrt::xrefs::reg_access("memory.workplace", "_TAILS", g__TAILS, s__TAILS);
    pyrt::xrefs::reg_access("memory.workplace", "pick", g_pick, s_pick);
}

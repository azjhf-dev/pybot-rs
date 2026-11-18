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
thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TARGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_TO_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_RECALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORD_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_fact_asked_again: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AGAIN() -> Py { ensure_init(); G_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); G_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE() -> Py { ensure_init(); G_A_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE(v: Py) { ensure_init(); G_A_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TARGET() -> Py { ensure_init(); G_A_TARGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TARGET(v: Py) { ensure_init(); G_A_TARGET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_TO_AVOID() -> Py { ensure_init(); G_A_THING_TO_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_TO_AVOID(v: Py) { ensure_init(); G_A_THING_TO_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME() -> Py { ensure_init(); G_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME(v: Py) { ensure_init(); G_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_RECALL() -> Py { ensure_init(); G_NOT_A_RECALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RECALL(v: Py) { ensure_init(); G_NOT_A_RECALL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_OPENS() -> Py { ensure_init(); G_PLAN_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_OPENS(v: Py) { ensure_init(); G_PLAN_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORD_ASKED() -> Py { ensure_init(); G_THE_WORD_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORD_ASKED(v: Py) { ensure_init(); G_THE_WORD_ASKED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A fact asked for \"again\" is answered from the fact the store kept, not from the words of the ask.\n\nPeople finish a line with \"again\" when they mean \"the thing I told you\": \"and whats my target\nagain\", \"what was i keeping away from again\", \"when is my return again\". On `9cdc62fe0` all three\nwere punts in the chat that holds the answer:\n\n    im 14 stone / i want to get to 12 / and whats my target again\n        -> That's beyond what I know well, sorry. Searching for it will turn up better sources...\n    my daughter is allergic to nuts / what was i keeping away from again\n        -> I don't know enough to answer that honestly. A search should sort you out quickly...\n    im coming back on the 14th / when is my return again\n        -> I'm not sure about that one, sorry - I'd rather not guess. A quick search online...\n\nThe fault is not one wording. The store had already **kept** each of these facts under its own key -\n`user[\"plans\"]` holds \"Get to 12\" and \"im coming back on the 14th\", `others[0][\"allergies\"]` holds\n\"nuts\" - and every reader in the store looks for the ask's own words in the chat. \"target\" is not\nin \"i want to get to 12\", \"keeping away from\" is not in \"allergic to nuts\", \"return\" is not in\n\"coming back\", so nothing matched and the turn fell to a search hedge.\n\nSo the rule here is the other way round: **an \"again\" ask is a request for something already kept,\nand the kind of thing it asks for picks the key.** A time asked for again is the dated plan they\ntold us; a thing to avoid asked for again is the allergy the store filed; a target asked for again\nis the plan they stated. The words of the ask never have to appear in the line that answered it.\n\nThree things keep it off lines it should not answer:\n\n- **The store's own reading comes first.** `bot/memory/when.py::when_answer` answers \"when is my\n  flight again\" from the words of the ask, and that answer is better than anything said back from a\n  key, so this module passes whenever the store can read the time ask at all.\n- **A question about the world is not a recall.** \"should i eat less again\", \"what should i do\n  again\" want advice, and the \"again\" in them is not pointing at a fact.\n- **One dated plan, or none.** A time asked for again is answered only when the chat holds a single\n  dated plan. Two dates in the chat and there is no way to tell which one the ask means, so the\n  turn is left alone rather than guessed at.\n");
    s_PRIORITY(Py::Int(7050));
    s_AGAIN(stdlib_call("re", "compile", &[py_str("\\bagain\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|when|which|where|who|how much|how many)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_RECALL(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|ought|best|better|tips?|advice|recommend\\w*|worth|cheapest|easiest|normal|usually|ok|okay|fine|bad|safe|possible|allowed|do i need to|can i|could i|is it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME(stdlib_call("re", "compile", &[py_str("\\bwhen\\b|\\bwhat (?:day|date|time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_TO_AVOID(stdlib_call("re", "compile", &[py_str("(?:keep\\w* (?:away|off)|stay\\w* (?:away|off)|avoid\\w*|allerg\\w*|cant have|can'?t have|not allowed|off limits|cant eat|can'?t eat)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TARGET(stdlib_call("re", "compile", &[py_str("\\b(?:target|goal|aim|trying to get to|trying to reach|hoping to get to|hoping for|working towards|saving for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE(py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)\\s+(?:of\\s+)?(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\\s+\\d{1,2}(?:st|nd|rd|th)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bnext\\s+(?:week|month|year|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_PLAN_OPENS(py_tuple(vec![py_str("i want to "), py_str("i wanna "), py_str("i'm trying to "), py_str("im trying to "), py_str("i am trying to "), py_str("i'm hoping to "), py_str("im hoping to "), py_str("i am hoping to "), py_str("i hope to "), py_str("to ")]));
    s_THE_WORD_ASKED(py_tuple(vec![py_tuple(vec![py_str("goal"), py_str("goal")]), py_tuple(vec![py_str("aim"), py_str("aim")]), py_tuple(vec![py_str("target"), py_str("target")]), py_tuple(vec![py_str("plan"), py_str("plan")])]));
}
pub fn _the_plans(args: &[Py]) -> Py {
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
    let mut latest: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("What the store kept as their plans, oldest first, newest last.");
    user = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&user.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut plan: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "strip", &[]) } }) {  __out.push(plan.clone()); } } py_list(__out) })() };
    latest = mcall(&user.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&latest.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&plans.clone(), &latest.clone())) } } } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut plan: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&plan.clone(), "split", &[])])); } py_list(__out) })() };
    Py::None
}
pub fn _the_allergies(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut what: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("[(who, what)] the store filed for the people in the chat, newest last.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&one.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    what = __item.clone();
    what = mcall(&call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&what.clone()) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![who.clone(), what.clone()])]);
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn _the_date_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The date a plan carries, in the words the plan says it, or None.");
    let __iter1 = py_iter_or_raise(&g_A_DATE());
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[plan.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_time_asked_again(args: &[Py]) -> Py {
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
    let mut date: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("The one dated plan the chat holds, said back, or None.");
    let __try1 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("memory.when", "when_answer", &[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    dated = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_plans(&[memory.clone()])) { let mut plan: Py = __item.clone(); __out.push(_the_date_in(&[plan.clone()])); } py_list(__out) })() }) { let mut date: Py = __item.clone(); if py_truthy(&date.clone()) {  __out.push(date.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[dated.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return op_add(&op_add(&py_str("You said "), &geti(&dated.clone(), &Py::Int(0))), &py_str("."));
    Py::None
}
pub fn _the_thing_to_avoid_asked_again(args: &[Py]) -> Py {
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
    let mut allergies: Py = Py::None;
    let mut what: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The allergy the store filed, said back, or None.");
    allergies = _the_allergies(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&allergies.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&geti(&allergies.clone(), &op_neg(&Py::Int(1))), 2, None);
    who = __unpk1[0].clone();
    what = __unpk1[1].clone();
    if py_truthy(&who.clone()) {
    return op_add(&op_add(&op_add(&mcall(&what.clone(), "capitalize", &[]), &py_str(" - your ")), &who.clone()), &py_str(" is allergic to them."));
    }
    return op_add(&mcall(&what.clone(), "capitalize", &[]), &py_str("."));
    Py::None
}
pub fn _the_noun_asked(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person's own word for the thing they asked about again.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&g_THE_WORD_ASKED());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    noun = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return noun.clone();
    }
    }
    return py_str("plan");
    Py::None
}
pub fn _a_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut first: Py = Py::None;
    let _ = py_str("Whether the word the plan opens on is somebody's or something's name, so it keeps its capital.");
    first = if py_truthy(&mcall(&plan.clone(), "split", &[])) { geti(&mcall(&plan.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    return { let __b = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:vauxhall|ford|toyota|nissan|bmw|audi|honda|ikea|argos|january|february|march|april|may|june|july|august|september|october|november|december|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b"), first.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&first.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]) } };
    Py::None
}
pub fn _the_target_asked_again(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("The plan they stated, said back under the word they used for it, or None.");
    plans = _the_plans(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&plans.clone()))) {
    return Py::None;
    }
    plan = geti(&plans.clone(), &op_neg(&Py::Int(1)));
    low = mcall(&plan.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_PLAN_OPENS());
    for __item in __iter1 {
    opener = __item.clone();
    if py_truthy(&mcall(&low.clone(), "startswith", &[opener.clone()])) {
    plan = slice_or_raise(&plan.clone(), &call_builtin("len", &[opener.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    noun = _the_noun_asked(&[text.clone()]);
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&plan.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&slice_or_raise(&plan.clone(), &Py::None, &Py::Int(2), &Py::None), "isupper", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_name(&[plan.clone()]))) } } } }) {
    plan = op_add(&mcall(&geti(&plan.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&plan.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("You said your "), &noun.clone()), &py_str(" is to ")), &mcall(&plan.clone(), "rstrip", &[py_str(".!?")])), &py_str("."));
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
    let mut low: Py = Py::None;
    let _ = py_str("The fact the store kept for the kind of thing the line asks for again, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_AGAIN(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_AN_ASK(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_RECALL(), "search", &[low.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_TIME(), "search", &[low.clone()])) {
    return _the_time_asked_again(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_A_THING_TO_AVOID(), "search", &[low.clone()])) {
    return _the_thing_to_avoid_asked_again(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_A_TARGET(), "search", &[low.clone()])) {
    return _the_target_asked_again(&[text.clone(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_fact_asked_again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGAIN" => Some(g_AGAIN()),
            "AN_ASK" => Some(g_AN_ASK()),
            "A_DATE" => Some(g_A_DATE()),
            "A_TARGET" => Some(g_A_TARGET()),
            "A_THING_TO_AVOID" => Some(g_A_THING_TO_AVOID()),
            "A_TIME" => Some(g_A_TIME()),
            "NOT_A_RECALL" => Some(g_NOT_A_RECALL()),
            "PLAN_OPENS" => Some(g_PLAN_OPENS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "THE_WORD_ASKED" => Some(g_THE_WORD_ASKED()),
            "_a_name" => Some(py_func("_a_name", |args: &[Py]| _a_name(args))),
            "_the_allergies" => Some(py_func("_the_allergies", |args: &[Py]| _the_allergies(args))),
            "_the_date_in" => Some(py_func("_the_date_in", |args: &[Py]| _the_date_in(args))),
            "_the_noun_asked" => Some(py_func("_the_noun_asked", |args: &[Py]| _the_noun_asked(args))),
            "_the_plans" => Some(py_func("_the_plans", |args: &[Py]| _the_plans(args))),
            "_the_target_asked_again" => Some(py_func("_the_target_asked_again", |args: &[Py]| _the_target_asked_again(args))),
            "_the_thing_to_avoid_asked_again" => Some(py_func("_the_thing_to_avoid_asked_again", |args: &[Py]| _the_thing_to_avoid_asked_again(args))),
            "_the_time_asked_again" => Some(py_func("_the_time_asked_again", |args: &[Py]| _the_time_asked_again(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "when_store" => Some(pyrt::xrefs::xmodule("memory.when")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGAIN" => s_AGAIN(v),
            "AN_ASK" => s_AN_ASK(v),
            "A_DATE" => s_A_DATE(v),
            "A_TARGET" => s_A_TARGET(v),
            "A_THING_TO_AVOID" => s_A_THING_TO_AVOID(v),
            "A_TIME" => s_A_TIME(v),
            "NOT_A_RECALL" => s_NOT_A_RECALL(v),
            "PLAN_OPENS" => s_PLAN_OPENS(v),
            "PRIORITY" => s_PRIORITY(v),
            "THE_WORD_ASKED" => s_THE_WORD_ASKED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_fact_asked_again", module_obj);
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_plans", py_func("_the_plans", |args: &[Py]| _the_plans(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_allergies", py_func("_the_allergies", |args: &[Py]| _the_allergies(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_date_in", py_func("_the_date_in", |args: &[Py]| _the_date_in(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_time_asked_again", py_func("_the_time_asked_again", |args: &[Py]| _the_time_asked_again(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_thing_to_avoid_asked_again", py_func("_the_thing_to_avoid_asked_again", |args: &[Py]| _the_thing_to_avoid_asked_again(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_noun_asked", py_func("_the_noun_asked", |args: &[Py]| _the_noun_asked(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_a_name", py_func("_a_name", |args: &[Py]| _a_name(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "_the_target_asked_again", py_func("_the_target_asked_again", |args: &[Py]| _the_target_asked_again(args)));
    pyrt::xrefs::reg_fn("modules.a_fact_asked_again", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "AGAIN", g_AGAIN, s_AGAIN);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "A_DATE", g_A_DATE, s_A_DATE);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "A_TARGET", g_A_TARGET, s_A_TARGET);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "A_THING_TO_AVOID", g_A_THING_TO_AVOID, s_A_THING_TO_AVOID);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "A_TIME", g_A_TIME, s_A_TIME);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "NOT_A_RECALL", g_NOT_A_RECALL, s_NOT_A_RECALL);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "PLAN_OPENS", g_PLAN_OPENS, s_PLAN_OPENS);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_fact_asked_again", "THE_WORD_ASKED", g_THE_WORD_ASKED, s_THE_WORD_ASKED);
}

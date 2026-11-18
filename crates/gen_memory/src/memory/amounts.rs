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


thread_local! { static G_A_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_LENGTH_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CEILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_AN_ASK() -> Py { ensure_init(); G_A_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AN_ASK(v: Py) { ensure_init(); G_A_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_LENGTH_OF() -> Py { ensure_init(); G_A_A_LENGTH_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_LENGTH_OF(v: Py) { ensure_init(); G_A_A_LENGTH_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CEILING() -> Py { ensure_init(); G_A_CEILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CEILING(v: Py) { ensure_init(); G_A_CEILING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THEIR_OWN() -> Py { ensure_init(); G_A_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THEIR_OWN(v: Py) { ensure_init(); G_A_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT() -> Py { ensure_init(); G__COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT(v: Py) { ensure_init(); G__COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THINGS() -> Py { ensure_init(); G__THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THINGS(v: Py) { ensure_init(); G__THINGS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A count the person reports, read against the count the last answer named.\n\nAdvice names a figure to aim at (\"six to eight glasses a day\", \"about 7,000 to 8,000 steps a day\"),\nand the next thing people do is say how they are doing against it: \"i have about 4 glasses\".\nNothing read the two together. The count in the answer is said with how often it is done, which\nfiles it as a rate rather than an amount, so the person's own count of the same thing was answered\nwith a receipt, a search hedge or a greeting.\n\nThe reading here is deliberately narrow:\n\n* the line is a count the person says they do, not a question about an amount (the ask patterns\n  own those) and not a count of something else (\"i have 3 children\");\n* the length they give is their own, not spent on something (\"2 hours of meetings\");\n* the answer names one figure of that same thing, a range counting as one (\"aim for 30 minutes a\n  day, and 60 if you can manage it\" is a list of choices, not one figure to aim at);\n* and the figure is not the person's own number said back (\"four glasses is on the short side\").\n\nThen the person is told where they stand, and the turn after can ask for the gap: \"how much more do\ni need\" names nothing of its own, and the counts read one turn earlier answer it.\n");
    s__COUNT(py_str("(?:\\d[\\d,]*(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)"));
    s__THINGS(py_tuple(vec![py_tuple(vec![py_str("glasses"), py_str("glasses?")]), py_tuple(vec![py_str("steps"), py_str("steps?")]), py_tuple(vec![py_str("minutes"), py_str("minutes?|mins?")]), py_tuple(vec![py_str("hours"), py_str("hours?|hrs?")]), py_tuple(vec![py_str("grams"), py_str("grams?|grammes?|g")]), py_tuple(vec![py_str("milligrams"), py_str("milligrams?|mg")]), py_tuple(vec![py_str("micrograms"), py_str("micrograms?|mcg")]), py_tuple(vec![py_str("kilometres"), py_str("kilometres?|kilometers?|kms?")]), py_tuple(vec![py_str("miles"), py_str("miles?")]), py_tuple(vec![py_str("sessions"), py_str("sessions?")])]));
    s_A_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\b[^.?!]{0,24}?\\b(?:have|had|get|got|do|eat|drink|walk|run|take|use|manage|average|track|log|hit|only|now|usually|normally|still|am on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_AN_ASK(stdlib_call("re", "compile", &[py_str("\\?|^\\s*(?:how|what|which|why|when|where|who|should|can|could|do|does|did|is|are|was|were|will|would|any|got any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_A_LENGTH_OF(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &g__COUNT()), &py_str("\\s*(?:hours?|hrs?|minutes?|mins?)\\s+of\\s+\\w")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__COUNT_WORDS(py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
    s_A_CEILING(stdlib_call("re", "compile", &[py_str("\\blimits?\\b|\\bno more than\\b|\\bat most\\b|\\bcap(?:ped)?\\b|\\bsafe (?:limit|amount|upper)\\b|\\bdo not (?:go )?(?:over|exceed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_value(args: &[Py]) -> Py {
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
    let _ = py_str("The number a count says: \"4\", \"7,000\", \"six\". None when it is not a count.");
    word = mcall(&mcall(&mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]), "replace", &[py_str(","), py_str("")]);
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("float", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError"]) {
                __handled5 = true;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&Py::Bool(py_contains(&mcall(&g__COUNT_WORDS(), "values", &[]), &word.clone()))) { call_builtin("float", &[op_add(&mcall(&call_builtin("list", &[mcall(&g__COUNT_WORDS(), "values", &[])]).unwrap_or_else(|e| exc::raise(e)), "index", &[word.clone()]), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    Py::None
}
pub fn _a_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("A count as the reply says it: \"two\", \"1,000\".");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&value.clone(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__COUNT_WORDS(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return geti(&g__COUNT_WORDS(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    return if py_truthy(&Py::Bool(py_eq(&value.clone(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))))) { mcall(&py_str("{:,}"), "format", &[call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]) } else { call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    Py::None
}
pub fn _counts(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every figure said with a unit of the table, in the order it was said.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g__THINGS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&op_add(&op_add(&py_str("("), &g__COUNT()), &py_str(")\\s*(?:-\\s*)?(?:")), &pattern.clone()), &py_str(")\\b")), { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    match_ = __item.clone();
    value = _a_value(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_dict(vec![(py_str("at"), mcall(&match_.clone(), "start", &[])), (py_str("value"), value.clone()), (py_str("thing"), thing.clone()), (py_str("said"), mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "strip", &[py_str(" .,!?")])), (py_str("period"), _the_period_at(&[text.clone(), mcall(&match_.clone(), "end", &[])]))])]);
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let found = found.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let text = text.clone(); let thing = thing.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let found = found.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let text = text.clone(); let thing = thing.clone(); let value = value.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &py_str("at")) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    return found.clone();
    Py::None
}
pub fn _the_period_at(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The period said right after the figure ending at `at`, as \"a day\" or \"a week\".");
    match_ = stdlib_call("re", "match", &[py_str("\\s*(?:a|an|per|each|every|most)\\s+(day|week|night|month)\\b"), slice_or_raise(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &at.clone(), &Py::None, &Py::None), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    return if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("week"), py_str("month")]), &mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])))) { py_str("a week") } else { py_str("a day") };
    Py::None
}
pub fn a_count_they_report(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("{\"value\", \"thing\", \"said\", \"period\"} for a count the person reports, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_AN_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_THEIR_OWN(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_A_LENGTH_OF(), "search", &[text.clone()])) {
    return Py::None;
    }
    found = _counts(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return geti(&found.clone(), &Py::Int(0));
    Py::None
}
pub fn _the_figure_named(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut mine: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut end: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut ranges: Py = Py::None;
    let mut singles: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The one figure of `thing` the answer names, as {\"low\", \"high\", \"said\", \"period\"}.");
    pattern = mcall(&call_builtin("dict", &[g__THINGS()]).unwrap_or_else(|e| exc::raise(e)), "get", &[thing.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pattern.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&answer.clone())) } }) {
    return Py::None;
    }
    ranges = call_builtin("list", &[stdlib_call("re", "finditer", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("("), &g__COUNT()), &py_str(")\\s*(?:to|-|\\u2013|and|or)\\s*(")), &g__COUNT()), &py_str(")\\s*(?:-\\s*)?(?:")), &pattern.clone()), &py_str(")\\b")), answer.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranges.clone()) { let mut match_: Py = __item.clone(); __out.push(mcall(&match_.clone(), "span", &[])); } py_list(__out) })() };
    singles = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&op_add(&op_add(&py_str("("), &g__COUNT()), &py_str(")\\s*(?:-\\s*)?(?:")), &pattern.clone()), &py_str(")\\b")), answer.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut match_: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk4524 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk4524[0].clone(); let mut end: Py = __unpk4524[1].clone(); __out.push({ let __b = op_le(&start.clone(), &mcall(&match_.clone(), "start", &[])); if !py_truthy(&__b) { __b } else { op_le(&mcall(&match_.clone(), "end", &[]), &end.clone()) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(match_.clone()); } } py_list(__out) })() };
    figures = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&ranges.clone());
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![_a_value(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _a_value(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&low.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&high.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_ge(&low.clone(), &high.clone()) } } } }) {
    continue;
    }
    let _ = mcall(&figures.clone(), "append", &[py_dict(vec![(py_str("low"), low.clone()), (py_str("high"), high.clone()), (py_str("said"), mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "strip", &[py_str(" .,!?")])), (py_str("period"), _the_period_at(&[answer.clone(), mcall(&match_.clone(), "end", &[])]))])]);
    }
    let __iter3 = py_iter_or_raise(&singles.clone());
    for __item in __iter3 {
    match_ = __item.clone();
    value = _a_value(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    continue;
    }
    let _ = mcall(&figures.clone(), "append", &[py_dict(vec![(py_str("low"), value.clone()), (py_str("high"), value.clone()), (py_str("said"), mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "strip", &[py_str(" .,!?")])), (py_str("period"), _the_period_at(&[answer.clone(), mcall(&match_.clone(), "end", &[])]))])]);
    }
    named = call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e));
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figures.clone()) { let mut figure: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = geti(&figure.clone(), &py_str("high")); if py_truthy(&Py::Bool(py_eq(&geti(&figure.clone(), &py_str("low")), &__cmp11))) { Py::Bool(py_eq(&__cmp11, &mine.clone())) } else { Py::Bool(false) } }))) {  __out.push(figure.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = op_gt(&named.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { figures.clone() } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return geti(&figures.clone(), &Py::Int(0));
    Py::None
}
pub fn _the_periods_disagree(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut theirs: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "theirs") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: theirs"))) };
    let _ = py_str("True when the two figures are said on periods that cannot be read against each other.");
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&theirs.clone(), &Py::None)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_eq(&mine.clone(), &theirs.clone()));
    Py::None
}
pub fn against_the_last_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut figure: Py = Py::None;
    let _ = py_str("The count they reported against the figure the last answer named, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    figure = _the_figure_named(&[answer.clone(), mcall(&said.clone(), "get", &[py_str("thing")]), mcall(&said.clone(), "get", &[py_str("value")])]);
    if py_truthy(&Py::Bool(py_is(&figure.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&_the_periods_disagree(&[mcall(&said.clone(), "get", &[py_str("period")]), mcall(&figure.clone(), "get", &[py_str("period")])])) {
    return Py::None;
    }
    if py_truthy(&op_le(&geti(&figure.clone(), &py_str("low")), &Py::Int(0))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("thing"), geti(&said.clone(), &py_str("thing"))), (py_str("mine"), geti(&said.clone(), &py_str("value"))), (py_str("mine_said"), geti(&said.clone(), &py_str("said"))), (py_str("low"), geti(&figure.clone(), &py_str("low"))), (py_str("high"), geti(&figure.clone(), &py_str("high"))), (py_str("figure"), geti(&figure.clone(), &py_str("said"))), (py_str("figure_period"), mcall(&figure.clone(), "get", &[py_str("period")])), (py_str("a_ceiling"), call_builtin("bool", &[mcall(&g_A_CEILING(), "search", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)))]);
    Py::None
}
pub fn _the_figure_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut against: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "against") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: against"))) };
    let mut period: Py = Py::None;
    let mut said: Py = Py::None;
    said = op_add(&py_str("the "), &_a_mid_sentence(&[geti(&against.clone(), &py_str("figure"))]));
    period = mcall(&against.clone(), "get", &[py_str("figure_period")]);
    if py_truthy(&{ let __b = period.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &period.clone())) } }) {
    let __aug1 = op_add(&said.clone(), &op_add(&py_str(" "), &period.clone()));
    said = __aug1;
    }
    return said.clone();
    Py::None
}
pub fn _a_mid_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut figure: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The answer's own figure, written mid-sentence: \"six to eight glasses a day\".");
    word = geti(&mcall(&{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[py_str(" ")]), &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&g__COUNT_WORDS(), "values", &[]), &mcall(&word.clone(), "lower", &[]))) } }) {
    return op_add(&mcall(&geti(&figure.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return figure.clone();
    Py::None
}
pub fn _the_aim_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut against: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "against") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: against"))) };
    let mut aim: Py = Py::None;
    let mut period: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The answer's figure, written mid-sentence and said the way the answer said it.\n\n    Two things about this line are read by the turn after it. `_enough_reply` in the store answers\n    \"how much more do i need\" and \"is that enough\" from the figures in the reply above, and it reads\n    a range as the figure to aim at only when the range is followed by its unit: \"the six to eight\n    glasses\", not \"the six to eight\". It also skips a range said with a rate straight after it\n    (\"six to eight glasses **a day**\"), because a figure said with how often it is done is a rate\n    rather than an amount. So the unit stays and the period goes, and the follow-up keeps the\n    answer it has on main:\n\n        i have about 4 glasses / how much more do i need\n          -> Another two glasses would put you at the six to eight glasses ...\n\n    `the_comparison` is read by a person, so the line goes on to say where that figure comes from\n    rather than talking about the bot's own earlier answer.\n    ");
    said = _a_mid_sentence(&[geti(&against.clone(), &py_str("figure"))]);
    aim = op_add(&py_str("the "), &said.clone());
    period = mcall(&against.clone(), "get", &[py_str("figure_period")]);
    if py_truthy(&{ let __b = period.clone(); if !py_truthy(&__b) { __b } else { mcall(&aim.clone(), "endswith", &[op_add(&py_str(" "), &period.clone())]) } }) {
    aim = slice_or_raise(&aim.clone(), &Py::None, &op_sub(&op_neg(&call_builtin("len", &[period.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1)), &Py::None);
    }
    return aim.clone();
    Py::None
}
pub fn the_comparison(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut against: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "against") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: against"))) };
    let mut aim: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Where the count they reported stands against the figure the answer named.");
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&against.clone(), &py_str("mine")), geti(&against.clone(), &py_str("low")), geti(&against.clone(), &py_str("high"))]), 3, None);
    mine = __unpk1[0].clone();
    low = __unpk1[1].clone();
    high = __unpk1[2].clone();
    let __unpk2 = py_unpack(&py_tuple(vec![_the_aim_said(&[against.clone()]), geti(&against.clone(), &py_str("mine_said"))]), 2, None);
    aim = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&mcall(&against.clone(), "get", &[py_str("a_ceiling")])) {
    if py_truthy(&op_gt(&mine.clone(), &high.clone())) {
    return mcall(&py_str("That is over {}, which is worth trimming back."), "format", &[aim.clone()]);
    }
    if py_truthy(&op_lt(&mine.clone(), &low.clone())) {
    return mcall(&py_str("That is under {}, which is where you want to be."), "format", &[aim.clone()]);
    }
    return mcall(&py_str("That is inside {}, which is fine - that is a ceiling, not a target."), "format", &[aim.clone()]);
    }
    if py_truthy(&op_lt(&mine.clone(), &low.clone())) {
    return mcall(&py_str("That is a fair way under {} most people need."), "format", &[aim.clone()]);
    }
    if py_truthy(&op_gt(&mine.clone(), &high.clone())) {
    return mcall(&py_str("That is past {}, which is fine - more than that is no problem."), "format", &[aim.clone()]);
    }
    return mcall(&py_str("That is inside {} most people need."), "format", &[aim.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.amounts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_AN_ASK" => Some(g_A_AN_ASK()),
            "A_A_LENGTH_OF" => Some(g_A_A_LENGTH_OF()),
            "A_CEILING" => Some(g_A_CEILING()),
            "A_THEIR_OWN" => Some(g_A_THEIR_OWN()),
            "_COUNT" => Some(g__COUNT()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_THINGS" => Some(g__THINGS()),
            "_a_mid_sentence" => Some(py_func("_a_mid_sentence", |args: &[Py]| _a_mid_sentence(args))),
            "_a_number" => Some(py_func("_a_number", |args: &[Py]| _a_number(args))),
            "_a_value" => Some(py_func("_a_value", |args: &[Py]| _a_value(args))),
            "_counts" => Some(py_func("_counts", |args: &[Py]| _counts(args))),
            "_the_aim_said" => Some(py_func("_the_aim_said", |args: &[Py]| _the_aim_said(args))),
            "_the_figure_named" => Some(py_func("_the_figure_named", |args: &[Py]| _the_figure_named(args))),
            "_the_figure_said" => Some(py_func("_the_figure_said", |args: &[Py]| _the_figure_said(args))),
            "_the_period_at" => Some(py_func("_the_period_at", |args: &[Py]| _the_period_at(args))),
            "_the_periods_disagree" => Some(py_func("_the_periods_disagree", |args: &[Py]| _the_periods_disagree(args))),
            "a_count_they_report" => Some(py_func("a_count_they_report", |args: &[Py]| a_count_they_report(args))),
            "against_the_last_answer" => Some(py_func("against_the_last_answer", |args: &[Py]| against_the_last_answer(args))),
            "the_comparison" => Some(py_func("the_comparison", |args: &[Py]| the_comparison(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_AN_ASK" => s_A_AN_ASK(v),
            "A_A_LENGTH_OF" => s_A_A_LENGTH_OF(v),
            "A_CEILING" => s_A_CEILING(v),
            "A_THEIR_OWN" => s_A_THEIR_OWN(v),
            "_COUNT" => s__COUNT(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_THINGS" => s__THINGS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.amounts", module_obj);
    pyrt::xrefs::reg_fn("memory.amounts", "_a_value", py_func("_a_value", |args: &[Py]| _a_value(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_a_number", py_func("_a_number", |args: &[Py]| _a_number(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_counts", py_func("_counts", |args: &[Py]| _counts(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_the_period_at", py_func("_the_period_at", |args: &[Py]| _the_period_at(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "a_count_they_report", py_func("a_count_they_report", |args: &[Py]| a_count_they_report(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_the_figure_named", py_func("_the_figure_named", |args: &[Py]| _the_figure_named(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_the_periods_disagree", py_func("_the_periods_disagree", |args: &[Py]| _the_periods_disagree(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "against_the_last_answer", py_func("against_the_last_answer", |args: &[Py]| against_the_last_answer(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_the_figure_said", py_func("_the_figure_said", |args: &[Py]| _the_figure_said(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_a_mid_sentence", py_func("_a_mid_sentence", |args: &[Py]| _a_mid_sentence(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "_the_aim_said", py_func("_the_aim_said", |args: &[Py]| _the_aim_said(args)));
    pyrt::xrefs::reg_fn("memory.amounts", "the_comparison", py_func("the_comparison", |args: &[Py]| the_comparison(args)));
    pyrt::xrefs::reg_access("memory.amounts", "A_AN_ASK", g_A_AN_ASK, s_A_AN_ASK);
    pyrt::xrefs::reg_access("memory.amounts", "A_A_LENGTH_OF", g_A_A_LENGTH_OF, s_A_A_LENGTH_OF);
    pyrt::xrefs::reg_access("memory.amounts", "A_CEILING", g_A_CEILING, s_A_CEILING);
    pyrt::xrefs::reg_access("memory.amounts", "A_THEIR_OWN", g_A_THEIR_OWN, s_A_THEIR_OWN);
    pyrt::xrefs::reg_access("memory.amounts", "_COUNT", g__COUNT, s__COUNT);
    pyrt::xrefs::reg_access("memory.amounts", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("memory.amounts", "_THINGS", g__THINGS, s__THINGS);
}

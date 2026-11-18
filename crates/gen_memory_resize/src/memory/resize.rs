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


thread_local! { static G_AIMS_AT_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPLACEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEADCOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANTITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOME_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_AT_THE_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PORTION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.resize: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AIMS_AT_A_FIGURE() -> Py { ensure_init(); G_AIMS_AT_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIMS_AT_A_FIGURE(v: Py) { ensure_init(); G_AIMS_AT_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CORRECTION() -> Py { ensure_init(); G_A_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION(v: Py) { ensure_init(); G_A_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPLACEMENT() -> Py { ensure_init(); G_A_REPLACEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPLACEMENT(v: Py) { ensure_init(); G_A_REPLACEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_HEADCOUNT() -> Py { ensure_init(); G_HEADCOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEADCOUNT(v: Py) { ensure_init(); G_HEADCOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_COUNT() -> Py { ensure_init(); G_ONLY_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_COUNT(v: Py) { ensure_init(); G_ONLY_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_RATE() -> Py { ensure_init(); G_ONLY_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_RATE(v: Py) { ensure_init(); G_ONLY_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANTITY() -> Py { ensure_init(); G_QUANTITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANTITY(v: Py) { ensure_init(); G_QUANTITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE() -> Py { ensure_init(); G_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE(v: Py) { ensure_init(); G_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_UNIT() -> Py { ensure_init(); G_RATE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_UNIT(v: Py) { ensure_init(); G_RATE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_OUT() -> Py { ensure_init(); G_SOMEONE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_OUT(v: Py) { ensure_init(); G_SOMEONE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOME_OF_THEM() -> Py { ensure_init(); G_SOME_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOME_OF_THEM(v: Py) { ensure_init(); G_SOME_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_AT_THE_TABLE() -> Py { ensure_init(); G_STILL_AT_THE_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_AT_THE_TABLE(v: Py) { ensure_init(); G_STILL_AT_THE_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PORTION_ASK() -> Py { ensure_init(); G__A_PORTION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PORTION_ASK(v: Py) { ensure_init(); G__A_PORTION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP() -> Py { ensure_init(); G__GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP(v: Py) { ensure_init(); G__GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_COUNTS() -> Py { ensure_init(); G__TWO_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_COUNTS(v: Py) { ensure_init(); G__TWO_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_VALUE() -> Py { ensure_init(); G__WORD_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_VALUE(v: Py) { ensure_init(); G__WORD_VALUE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that only changes a figure: the same question, with the new figure in it.\n\n\"how long do i cook a 2kg chicken for\" ... \"what about a 3kg one\" is one question asked twice.\nThe ask is the same and only the weight has changed, but read on its own the second line names\nnothing (\"a 3kg one\"), so no entry fits it and the bot shrugs at a question it has just answered\nin full for the smaller bird. Said again with the figure the earlier line had and the new one in\nits place, it reaches the same entry, and the knowledge base works the new weight out from its\nown rule (\"20 minutes per 500g plus 20\").\n\nA headcount works the same way: \"how much rice for 4 people\" then \"actually only 3 people\" is\nthat question for three, and the portion answer gives 225 g rather than the 300 g for four it\nhad just given. The count is a figure of the same kind as the weight, and the one people correct\nmost often.\n\nOnly a line that names a figure of its own is rewritten, only when the earlier line named a\nfigure of the same kind, and only when nothing else the line says is a subject of its own. The\nrewritten line still has to reach an entry, which is the caller's check: this module offers a\nreading, it does not answer.\n");
    s_QUANTITY(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d+(?:\\.\\d+)?)\\s*(kg|kilos?|kilograms?|g|grams?|lb|lbs|pounds?|oz|litres?|liters?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT(py_str("(?:\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)"));
    s_HEADCOUNT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![\\w.])("), &g_COUNT()), &py_str(")\\s+(?:people|persons?|adults?|kids|children|guests?|lads|mates|friends|of us|of them|of you)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ONLY_A_COUNT(call_builtin("frozenset", &[mcall(&py_str("\n    a about actually and are for instead it it's its just make no not now of only please so\n    sorry that the there too us wait we we're were\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PORTION_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE(stdlib_call("re", "compile", &[py_str("(?<![\\w.])£?\\s*(\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:pounds?|quid)?\\s*(a|per|each|every|/)\\s*(month|week|fortnight|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_UNIT(py_dict(vec![(py_str("month"), py_str("a month")), (py_str("week"), py_str("a week")), (py_str("fortnight"), py_str("a fortnight")), (py_str("year"), py_str("a year"))]));
    s_ONLY_A_RATE(call_builtin("frozenset", &[mcall(&py_str("\n    a about actually afford and at be bump can could do for guess i if instead it just let lets\n    make maybe manage more much now of only or please rather really say so still stretch that\n    think to up want was we what would you\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPLACEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:make|instead|actually|only|just|no|not|wait|sorry|now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isn'?t|aren'?t|isnt|arent)\\s+not\\b|\\b(?:isn'?t|aren'?t|not)\\s+(?:coming|eating|having|joining)\\b|\\bcan'?t\\b|\\bcannot\\b|\\bwont\\b|\\bwon'?t\\b|\\bca\\s*n'?t\\b|\\b(?:doesn'?t|does not|dont|do not)\\s+(?:eat|want|like|come)\\b|\\bpulled out\\b|\\bdropped out\\b|\\bflaked\\b|\\bcancelled\\b|\\bcanceled\\b|\\b(?:is|are)\\s+(?:ill|unwell|sick|away)\\b|\\b(?:isnt|isn'?t)\\s+(?:well|free)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOME_OF_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few|a couple)\\s+of\\s+(?:them|those|us|you)\\s+(?:is|are|was|were|has|have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_AT_THE_TABLE(stdlib_call("re", "compile", &[py_str("\\bof\\s+(?:them|those|us|you)\\s+(?:is|are|was|were)\\s+(?:still\\s+)?(?:eating|having|coming|drinking|joining|there|in|here)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__GROUP(py_str("(?:people|persons?|adults?|kids?|children|guests?|lads?|mates?|friends?|boys?|girls?|men|women|veg(?:etarian)?s?|veggies|teenagers?|toddlers?|grown-?ups?|of us|of them|of you)"));
    s__TWO_COUNTS(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("(?<![\\w.])("), &g_COUNT()), &py_str(")\\s+")), &g__GROUP()), &py_str("")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORD_VALUE(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]));
    fn __data1() -> Py { py_dict(vec![(py_str("kg"), py_str("weight")), (py_str("kilo"), py_str("weight")), (py_str("kilos"), py_str("weight")), (py_str("kilogram"), py_str("weight")), (py_str("kilograms"), py_str("weight")), (py_str("g"), py_str("weight")), (py_str("gram"), py_str("weight")), (py_str("grams"), py_str("weight")), (py_str("lb"), py_str("weight")), (py_str("lbs"), py_str("weight")), (py_str("pound"), py_str("weight")), (py_str("pounds"), py_str("weight")), (py_str("oz"), py_str("weight")), (py_str("litre"), py_str("volume")), (py_str("litres"), py_str("volume")), (py_str("liter"), py_str("volume")), (py_str("liters"), py_str("volume"))]) }
    s_FAMILY(__data1());
    s_AIMS_AT_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|need|needs|save|saving|target|aim|aiming)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:actually|instead|rather|only|just)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn quantity(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The figure in a line and what it is a figure of: \"a 3kg one\" -> (3.0, \"kg\", span).");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_QUANTITY(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    unit = mcall(&found.clone(), "group", &[Py::Int(2)]);
    return py_tuple(vec![value.clone(), unit.clone(), mcall(&found.clone(), "span", &[]), mcall(&g_FAMILY(), "get", &[unit.clone(), py_str("")])]);
    Py::None
}
pub fn _earlier_lines(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(4) };
    let mut lines: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The person's own lines before this one, newest first.");
    recent = py_func("recent", |args: &[Py]| pyrt::xrefs::xcall("memory", "recent", args));
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_call(&recent, &[memory.clone(), op_add(&n.clone(), &Py::Int(1))]));
    for __item in __iter1 {
    turn = __item.clone();
    said = { let __b = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } };
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&said.clone()) {
    let _ = mcall(&lines.clone(), "append", &[said.clone()]);
    }
    }
    return slice_or_raise(&lines.clone(), &Py::None, &n.clone(), &Py::None);
    Py::None
}
pub fn rate(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The money a line puts by each period: \"50 a month\" -> (50.0, \"month\", span, \"rate\").");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_RATE(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return py_tuple(vec![value.clone(), mcall(&found.clone(), "group", &[Py::Int(3)]), mcall(&found.clone(), "span", &[]), py_str("rate")]);
    Py::None
}
pub fn _a_bare_rate(args: &[Py]) -> Py {
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
    let mut others: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The figure in a line that says nothing but a new figure: \"what if i do 100\" -> 100.0.\n\n    Allowed only when every other word is a frame word (\"what if\", \"make it\", \"say\"), so a figure\n    with a subject of its own (\"100 calories a day\", \"20 press ups\") is left alone. What the\n    figure is a figure of comes from the earlier line, which is where a correction points.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    found = stdlib_call("re", "search", &[py_str("(?<![\\w.])£?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:pounds?|quid)?"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:\"'")])); } py_list(__out) })() };
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = w.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ONLY_A_RATE(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&others.clone(), &py_list(vec![mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[])])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_AIMS_AT_A_FIGURE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_CORRECTION(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _with_a_new_rate(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut candidate: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut end: Py = Py::None;
    let mut low: Py = Py::None;
    let mut period: Py = Py::None;
    let mut start: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The earlier line's rate swapped for this one, or None.\n\n    \"i can put away 50 a month\" then \"what if i do 100\" is that line with a new rate in it: the\n    goal is the same one, and the money entry works out how long it takes at 100. The period is\n    the new line's own when it names one (\"what about 200 a week\") and the earlier line's when it\n    does not, because that is where a bare figure gets its meaning. A line with no rate in the\n    chat before it is not a corrected rate.\n    ");
    let __iter1 = py_iter_or_raise(&_earlier_lines(&[memory.clone()]));
    for __item in __iter1 {
    earlier = __item.clone();
    there = rate(&[earlier.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&there.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&unit.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&there.clone(), &Py::Int(0)), &value.clone())) } } } }) {
    continue;
    }
    period = { let __b = unit.clone(); if py_truthy(&__b) { __b } else { geti(&there.clone(), &Py::Int(1)) } };
    let __unpk2 = py_unpack(&geti(&there.clone(), &Py::Int(2)), 2, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&earlier.clone(), "lower", &[]), "split", &[])]);
    candidate = mcall(&op_add(&op_add(&op_add(&slice_or_raise(&low.clone(), &Py::None, &start.clone(), &Py::None), &op_modulo(&py_str("%g "), &value.clone())), &geti(&g_RATE_UNIT(), &period.clone())), &slice_or_raise(&low.clone(), &end.clone(), &Py::None, &Py::None)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_eq(&candidate.clone(), &mcall(&said.clone(), "lower", &[])))) {
    return candidate.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut raw: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: raw"))) };
    let _ = py_str("The number a word or a figure stands for: \"three\" and \"3\" are both 3.");
    raw = mcall(&mcall(&call_builtin("str", &[{ let __b = raw.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) {
    return call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&g__WORD_VALUE(), "get", &[raw.clone()]);
    Py::None
}
pub fn headcount(args: &[Py]) -> Py {
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
    let mut groups: Py = Py::None;
    let mut more: Py = Py::None;
    let mut number: Py = Py::None;
    let mut others: Py = Py::None;
    let mut said: Py = Py::None;
    let mut total: Py = Py::None;
    let mut value: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The number of people a line gives and where it stands: \"only 3 people\" -> (3, 5, 6).\n\n    A figure counts as a headcount when the line says what it counts (\"3 people\", \"two of us\",\n    \"4 guests\"), or when the line is nothing but a new figure for the table already on the table\n    (\"make it for 3\", \"3 instead\", \"we're 6 now\"): the earlier line is where the number of\n    people comes from, and a person correcting it says the new number on its own. A bare figure\n    in a line with any other subject is left alone, because nothing says what it counts.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    groups = call_builtin("list", &[mcall(&g__TWO_COUNTS(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&call_builtin("len", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    total = { let __b = _value(&[mcall(&geti(&groups.clone(), &Py::Int(0)), "group", &[Py::Int(1)])]); if py_truthy(&__b) { __b } else { Py::Int(0) } };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&groups.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    more = __item.clone();
    let __aug2 = op_add(&total.clone(), &{ let __b = _value(&[mcall(&more.clone(), "group", &[Py::Int(1)])]); if py_truthy(&__b) { __b } else { Py::Int(0) } });
    total = __aug2;
    }
    if py_truthy(&total.clone()) {
    return py_tuple(vec![total.clone(), mcall(&geti(&groups.clone(), &Py::Int(0)), "start", &[Py::Int(1)]), mcall(&geti(&groups.clone(), &op_neg(&Py::Int(1))), "end", &[Py::Int(1)])]);
    }
    }
    found = mcall(&g_HEADCOUNT(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    value = _value(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    return if py_truthy(&value.clone()) { py_tuple(vec![value.clone(), mcall(&found.clone(), "start", &[Py::Int(1)]), mcall(&found.clone(), "end", &[Py::Int(1)])]) } else { Py::None };
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    return Py::None;
    }
    number = stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![\\w.])("), &g_COUNT()), &py_str(")\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return Py::None;
    }
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:\"'")])); } py_list(__out) })() };
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = w.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ONLY_A_COUNT(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[others.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    value = _value(&[mcall(&number.clone(), "group", &[Py::Int(1)])]);
    return if py_truthy(&value.clone()) { py_tuple(vec![value.clone(), mcall(&number.clone(), "start", &[Py::Int(1)]), mcall(&number.clone(), "end", &[Py::Int(1)])]) } else { Py::None };
    Py::None
}
pub fn _a_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut earlier: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "earlier") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: earlier"))) };
    let mut said: Py = Py::None;
    let _ = py_str("Whether the earlier line asked something, so the rewrite is an answer rather than a fact.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = earlier.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:how|what|which)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "endswith", &[py_str("?")]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ones_left(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut earlier: Py = Py::None;
    let mut left: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The table with the people who are out taken off it, or None when there is no table.\n\n    \"one of them isnt coming\" after a question for eight is that question for seven: the figure\n    in the line is how many are out, not the new total. Nothing is returned when no earlier line\n    names a table, and nothing when the sum leaves nobody, because a portion for one person is\n    worse than leaving the earlier answer alone.\n    ");
    let __iter1 = py_iter_or_raise(&_earlier_lines(&[memory.clone()]));
    for __item in __iter1 {
    earlier = __item.clone();
    there = headcount(&[earlier.clone()]);
    if py_truthy(&Py::Bool(py_is(&there.clone(), &Py::None))) {
    continue;
    }
    left = op_sub(&geti(&there.clone(), &Py::Int(0)), &value.clone());
    return if py_truthy(&op_ge(&left.clone(), &Py::Int(1))) { left.clone() } else { Py::None };
    }
    return Py::None;
    Py::None
}
pub fn _with_a_new_table(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut candidate: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut earlier_ask: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The portion question on the table, asked for a new number of people, or None.\n\n    \"how much rice for 4 people\" then \"actually only 3 people\" is that question for three, and\n    the portion tables work out the new total from their own per-person figure. The earlier line\n    has to be a question and has to name the table, so a count in a statement is not read as one\n    (\"we are 4 people\" then \"make it 6\" is not a portion ask).\n    ");
    let __iter1 = py_iter_or_raise(&_earlier_lines(&[memory.clone()]));
    for __item in __iter1 {
    earlier = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_a_question(&[earlier.clone()])))) {
    continue;
    }
    there = headcount(&[earlier.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&there.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&there.clone(), &Py::Int(0)), &value.clone())) } }) {
    continue;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&earlier.clone(), "lower", &[]), "split", &[])]);
    candidate = mcall(&op_add(&op_add(&slice_or_raise(&low.clone(), &Py::None, &geti(&there.clone(), &Py::Int(1)), &Py::None), &call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), &slice_or_raise(&low.clone(), &geti(&there.clone(), &Py::Int(2)), &Py::None, &Py::None)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_eq(&candidate.clone(), &mcall(&said.clone(), "lower", &[])))) {
    return candidate.clone();
    }
    }
    earlier_ask = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_earlier_lines(&[memory.clone()])) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = _a_question(&[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_PORTION_ASK(), "search", &[mcall(&line.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&headcount(&[line.clone()]), &Py::None)) } } } }) {  __out.push(line.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&earlier_ask.clone()) {
    return op_modulo(&py_str("%s for %d people"), &py_tuple(vec![mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&earlier_ask.clone(), "lower", &[]), "split", &[])]), "rstrip", &[py_str("?. ")]), value.clone()]));
    }
    return Py::None;
    Py::None
}
pub fn _with_a_new_weight(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _span: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut end: Py = Py::None;
    let mut family: Py = Py::None;
    let mut low: Py = Py::None;
    let mut now: Py = Py::None;
    let mut start: Py = Py::None;
    let mut there: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The earlier question with its figure swapped for this one, or None.\n\n    The figure carries its unit when the line gives one (\"what about a 3kg one\") and takes the\n    earlier line's unit when it does not (\"make it 3\" after \"how long do i cook a 2kg chicken\n    for\"): the earlier question is where the number gets its meaning. A figure of another kind\n    (litres after a weight) is not the same question, and the same figure again is no change.\n    ");
    now = quantity(&[said.clone()]);
    unit = Py::None;
    family = Py::None;
    if py_truthy(&Py::Bool(!py_is(&now.clone(), &Py::None))) {
    let __unpk1 = py_unpack(&now.clone(), 4, None);
    value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    _span = __unpk1[2].clone();
    family = __unpk1[3].clone();
    }
    let __iter2 = py_iter_or_raise(&_earlier_lines(&[memory.clone()]));
    for __item in __iter2 {
    earlier = __item.clone();
    there = quantity(&[earlier.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&there.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&there.clone(), &Py::Int(0)), &value.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&unit.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_question(&[earlier.clone()])))) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&there.clone(), &Py::Int(1)), geti(&there.clone(), &Py::Int(3))]), 2, None);
    unit = __unpk3[0].clone();
    family = __unpk3[1].clone();
    } else {
    if py_truthy(&Py::Bool(!py_eq(&geti(&there.clone(), &Py::Int(3)), &family.clone()))) {
    continue;
    }
    }
    let __unpk4 = py_unpack(&geti(&there.clone(), &Py::Int(2)), 2, None);
    start = __unpk4[0].clone();
    end = __unpk4[1].clone();
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&earlier.clone(), "lower", &[]), "split", &[])]);
    candidate = mcall(&op_add(&op_add(&op_add(&slice_or_raise(&low.clone(), &Py::None, &start.clone(), &Py::None), &op_modulo(&py_str("%g"), &value.clone())), &unit.clone()), &slice_or_raise(&low.clone(), &end.clone(), &Py::None, &Py::None)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_eq(&candidate.clone(), &mcall(&said.clone(), "lower", &[])))) {
    return candidate.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn rewritten(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut left: Py = Py::None;
    let mut now: Py = Py::None;
    let mut resized: Py = Py::None;
    let mut said: Py = Py::None;
    let mut table: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The line read with the earlier line's figure swapped for this one, or None.\n\n    \"what about a 3kg one\" after \"how long do i cook a 2kg chicken for\" is read as \"how long do\n    i cook a 3kg chicken for\". A line whose figure is the same as the earlier one is no change,\n    and a figure of another kind (litres after a weight) is not the same question.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::None;
    }
    now = rate(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&now.clone(), &Py::None))) {
    return _with_a_new_rate(&[said.clone(), geti(&now.clone(), &Py::Int(0)), geti(&now.clone(), &Py::Int(1)), memory.clone()]);
    }
    bare = _a_bare_rate(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&bare.clone(), &Py::None))) {
    resized = _with_a_new_rate(&[said.clone(), bare.clone(), Py::None, memory.clone()]);
    if py_truthy(&resized.clone()) {
    return resized.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_is(&quantity(&[said.clone()]), &Py::None))) {
    return _with_a_new_weight(&[said.clone(), Py::None, memory.clone()]);
    }
    table = headcount(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&table.clone(), &Py::None))) {
    value = geti(&table.clone(), &Py::Int(0));
    if py_truthy(&mcall(&g_SOMEONE_OUT(), "search", &[said.clone()])) {
    left = _the_ones_left(&[value.clone(), memory.clone()]);
    return if py_truthy(&left.clone()) { _with_a_new_table(&[said.clone(), left.clone(), memory.clone()]) } else { Py::None };
    }
    if py_truthy(&{ let __b = mcall(&g_SOME_OF_THEM(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_STILL_AT_THE_TABLE(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HEADCOUNT(), "search", &[mcall(&said.clone(), "lower", &[])])) {
    return _with_a_new_table(&[said.clone(), value.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_REPLACEMENT(), "search", &[said.clone()])))) {
    return Py::None;
    }
    return { let __b = _with_a_new_table(&[said.clone(), value.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _with_a_new_weight(&[said.clone(), value.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _with_a_new_rate(&[said.clone(), value.clone(), Py::None, memory.clone()]) } } } };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.resize".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AIMS_AT_A_FIGURE" => Some(g_AIMS_AT_A_FIGURE()),
            "A_CORRECTION" => Some(g_A_CORRECTION()),
            "A_REPLACEMENT" => Some(g_A_REPLACEMENT()),
            "COUNT" => Some(g_COUNT()),
            "FAMILY" => Some(g_FAMILY()),
            "HEADCOUNT" => Some(g_HEADCOUNT()),
            "ONLY_A_COUNT" => Some(g_ONLY_A_COUNT()),
            "ONLY_A_RATE" => Some(g_ONLY_A_RATE()),
            "QUANTITY" => Some(g_QUANTITY()),
            "RATE" => Some(g_RATE()),
            "RATE_UNIT" => Some(g_RATE_UNIT()),
            "SOMEONE_OUT" => Some(g_SOMEONE_OUT()),
            "SOME_OF_THEM" => Some(g_SOME_OF_THEM()),
            "STILL_AT_THE_TABLE" => Some(g_STILL_AT_THE_TABLE()),
            "_A_PORTION_ASK" => Some(g__A_PORTION_ASK()),
            "_GROUP" => Some(g__GROUP()),
            "_TWO_COUNTS" => Some(g__TWO_COUNTS()),
            "_WORD_VALUE" => Some(g__WORD_VALUE()),
            "_a_bare_rate" => Some(py_func("_a_bare_rate", |args: &[Py]| _a_bare_rate(args))),
            "_a_question" => Some(py_func("_a_question", |args: &[Py]| _a_question(args))),
            "_earlier_lines" => Some(py_func("_earlier_lines", |args: &[Py]| _earlier_lines(args))),
            "_the_ones_left" => Some(py_func("_the_ones_left", |args: &[Py]| _the_ones_left(args))),
            "_value" => Some(py_func("_value", |args: &[Py]| _value(args))),
            "_with_a_new_rate" => Some(py_func("_with_a_new_rate", |args: &[Py]| _with_a_new_rate(args))),
            "_with_a_new_table" => Some(py_func("_with_a_new_table", |args: &[Py]| _with_a_new_table(args))),
            "_with_a_new_weight" => Some(py_func("_with_a_new_weight", |args: &[Py]| _with_a_new_weight(args))),
            "headcount" => Some(py_func("headcount", |args: &[Py]| headcount(args))),
            "quantity" => Some(py_func("quantity", |args: &[Py]| quantity(args))),
            "rate" => Some(py_func("rate", |args: &[Py]| rate(args))),
            "rewritten" => Some(py_func("rewritten", |args: &[Py]| rewritten(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AIMS_AT_A_FIGURE" => s_AIMS_AT_A_FIGURE(v),
            "A_CORRECTION" => s_A_CORRECTION(v),
            "A_REPLACEMENT" => s_A_REPLACEMENT(v),
            "COUNT" => s_COUNT(v),
            "FAMILY" => s_FAMILY(v),
            "HEADCOUNT" => s_HEADCOUNT(v),
            "ONLY_A_COUNT" => s_ONLY_A_COUNT(v),
            "ONLY_A_RATE" => s_ONLY_A_RATE(v),
            "QUANTITY" => s_QUANTITY(v),
            "RATE" => s_RATE(v),
            "RATE_UNIT" => s_RATE_UNIT(v),
            "SOMEONE_OUT" => s_SOMEONE_OUT(v),
            "SOME_OF_THEM" => s_SOME_OF_THEM(v),
            "STILL_AT_THE_TABLE" => s_STILL_AT_THE_TABLE(v),
            "_A_PORTION_ASK" => s__A_PORTION_ASK(v),
            "_GROUP" => s__GROUP(v),
            "_TWO_COUNTS" => s__TWO_COUNTS(v),
            "_WORD_VALUE" => s__WORD_VALUE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.resize", module_obj);
    pyrt::xrefs::reg_fn("memory.resize", "quantity", py_func("quantity", |args: &[Py]| quantity(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_earlier_lines", py_func("_earlier_lines", |args: &[Py]| _earlier_lines(args)));
    pyrt::xrefs::reg_fn("memory.resize", "rate", py_func("rate", |args: &[Py]| rate(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_a_bare_rate", py_func("_a_bare_rate", |args: &[Py]| _a_bare_rate(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_with_a_new_rate", py_func("_with_a_new_rate", |args: &[Py]| _with_a_new_rate(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_value", py_func("_value", |args: &[Py]| _value(args)));
    pyrt::xrefs::reg_fn("memory.resize", "headcount", py_func("headcount", |args: &[Py]| headcount(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_a_question", py_func("_a_question", |args: &[Py]| _a_question(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_the_ones_left", py_func("_the_ones_left", |args: &[Py]| _the_ones_left(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_with_a_new_table", py_func("_with_a_new_table", |args: &[Py]| _with_a_new_table(args)));
    pyrt::xrefs::reg_fn("memory.resize", "_with_a_new_weight", py_func("_with_a_new_weight", |args: &[Py]| _with_a_new_weight(args)));
    pyrt::xrefs::reg_fn("memory.resize", "rewritten", py_func("rewritten", |args: &[Py]| rewritten(args)));
    pyrt::xrefs::reg_access("memory.resize", "AIMS_AT_A_FIGURE", g_AIMS_AT_A_FIGURE, s_AIMS_AT_A_FIGURE);
    pyrt::xrefs::reg_access("memory.resize", "A_CORRECTION", g_A_CORRECTION, s_A_CORRECTION);
    pyrt::xrefs::reg_access("memory.resize", "A_REPLACEMENT", g_A_REPLACEMENT, s_A_REPLACEMENT);
    pyrt::xrefs::reg_access("memory.resize", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("memory.resize", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("memory.resize", "HEADCOUNT", g_HEADCOUNT, s_HEADCOUNT);
    pyrt::xrefs::reg_access("memory.resize", "ONLY_A_COUNT", g_ONLY_A_COUNT, s_ONLY_A_COUNT);
    pyrt::xrefs::reg_access("memory.resize", "ONLY_A_RATE", g_ONLY_A_RATE, s_ONLY_A_RATE);
    pyrt::xrefs::reg_access("memory.resize", "QUANTITY", g_QUANTITY, s_QUANTITY);
    pyrt::xrefs::reg_access("memory.resize", "RATE", g_RATE, s_RATE);
    pyrt::xrefs::reg_access("memory.resize", "RATE_UNIT", g_RATE_UNIT, s_RATE_UNIT);
    pyrt::xrefs::reg_access("memory.resize", "SOMEONE_OUT", g_SOMEONE_OUT, s_SOMEONE_OUT);
    pyrt::xrefs::reg_access("memory.resize", "SOME_OF_THEM", g_SOME_OF_THEM, s_SOME_OF_THEM);
    pyrt::xrefs::reg_access("memory.resize", "STILL_AT_THE_TABLE", g_STILL_AT_THE_TABLE, s_STILL_AT_THE_TABLE);
    pyrt::xrefs::reg_access("memory.resize", "_A_PORTION_ASK", g__A_PORTION_ASK, s__A_PORTION_ASK);
    pyrt::xrefs::reg_access("memory.resize", "_GROUP", g__GROUP, s__GROUP);
    pyrt::xrefs::reg_access("memory.resize", "_TWO_COUNTS", g__TWO_COUNTS, s__TWO_COUNTS);
    pyrt::xrefs::reg_access("memory.resize", "_WORD_VALUE", g__WORD_VALUE, s__WORD_VALUE);
}

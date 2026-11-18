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


thread_local! { static G_A_STRETCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTRA_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PILE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PILE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROLE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEATHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_STRETCH() -> Py { ensure_init(); G_A_STRETCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STRETCH(v: Py) { ensure_init(); G_A_STRETCH.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTENT() -> Py { ensure_init(); G_CONTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTENT(v: Py) { ensure_init(); G_CONTENT.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTRA_BODY() -> Py { ensure_init(); G_EXTRA_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA_BODY(v: Py) { ensure_init(); G_EXTRA_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_REPLIES() -> Py { ensure_init(); G_JOB_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_REPLIES(v: Py) { ensure_init(); G_JOB_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOB() -> Py { ensure_init(); G_NOT_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB(v: Py) { ensure_init(); G_NOT_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_PILE() -> Py { ensure_init(); G_PILE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PILE(v: Py) { ensure_init(); G_PILE.with(|c| *c.borrow_mut() = v); }
pub fn g_PILE_REPLIES() -> Py { ensure_init(); G_PILE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PILE_REPLIES(v: Py) { ensure_init(); G_PILE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING() -> Py { ensure_init(); G_POINTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); G_POINTING.with(|c| *c.borrow_mut() = v); }
pub fn g_ROLE() -> Py { ensure_init(); G_ROLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROLE(v: Py) { ensure_init(); G_ROLE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROLE_REPLIES() -> Py { ensure_init(); G_ROLE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROLE_REPLIES(v: Py) { ensure_init(); G_ROLE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WEAR() -> Py { ensure_init(); G_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEAR(v: Py) { ensure_init(); G_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_WEATHER() -> Py { ensure_init(); G_WEATHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEATHER(v: Py) { ensure_init(); G_WEATHER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing, or a pile of them, that is wearing the person down.\n\n\"This spreadsheet is killing me\", \"my inbox is a nightmare\", \"i have so much marking to do\":\nnothing is broken, no body hurts and nobody else is in the line, so nothing claimed them and\nthey all got one of the three stock lines (\"I'm listening - say more if it helps\"). That is the\nexample STYLE.md opens with, so the composer answers the wear itself: name it briefly, then ask\nthe one question that gets the job moving.\n\nTwo shapes, because a pile and a single job want different questions:\n\n* a pile of things to do - \"i have so much marking to do\", \"my inbox is a nightmare\",\n  \"the washing is piling up\"\n* one job that fights back - \"this spreadsheet is killing me\", \"this essay is killing me\",\n  \"painting the hallway is taking forever\"\n\nEvery rule here is a guard against taking a turn that is not that, and each one is paid for by a\nline it must not take:\n\n* never a body: \"my back is killing me\" and \"me knees are killing me\" are an ache, and\n  bot/compose/trouble.py asks the right question about them. Its reading settles this one;\n* never a broken thing: \"my boiler is on the blink\" is a fault with a next step in it, and that\n  is trouble's reply, not a question about how the job is going;\n* **never another person**: \"my mate at work is a nightmare\" and \"my kids are driving me mad\"\n  are about somebody, so they belong to bot/modules/chat.py and to safety, whatever else the\n  line says;\n* never the weather: \"this heat is killing me\" is a line about the day, and a question about\n  where you are up to with the job is the wrong thing to say to it;\n* never a stretch of time: \"this week has been a nightmare\" and \"last night was a nightmare\" are\n  a rough week or a bad night, not a task, and the week has its own warm reply;\n* never a question, and never a plea for tips (\"what should i do about my inbox\") - those ask\n  something back and are answered as questions.\n");
    s_WEAR(stdlib_call("re", "compile", &[py_str("\n    \\b(?:is|are|was|were|'s|its|it's|gets|got|has\\s+been|have\\s+been|had\\s+been)\\b\n    [^.!?,;]{0,22}?\n    \\b(?:\n        killing\\s+me | the\\s+death\\s+of\\s+me\n      | doing\\s+my\\s+head\\s+in\n      | driving\\s+me\\s+(?:mad|crazy|nuts|up\\s+the\\s+wall|round\\s+the\\s+bend|insane)\n      | a\\s+(?:(?:real|right|total|proper|absolute|bit\\s+of\\s+a|such\\s+a|bloody)\\s+)?\n        (?:nightmare|mare|pain|slog|chore)\n      | wearing\\s+me\\s+(?:down|out)\n      | getting\\s+on\\s+top\\s+of\\s+me\n      | taking\\s+(?:forever|ages|all\\s+day|all\\s+week|so\\s+long|longer\\s+than)\n      | never\\s+ending | endless\n      | (?:getting\\s+)?out\\s+of\\s+control\n      | going\\s+on\\s+forever\n      | taking\\s+over\\s+my\\s+life\n    )\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PILE(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        (?:so\\s+much|so\\s+many|loads\\s+of|lots\\s+of|tons\\s+of|masses\\s+of|piles?\\s+of|a\\s+heap\\s+of)\n        [^.!?,;]{0,30}?\n        \\b(?:to\\s+(?:do|sort|mark|write|finish|file|get\\s+through|deal\\s+with|catch\\s+up\\s+on\n                       |work\\s+through|go\\s+through))\n      | (?:i'?m|im|i\\s+am|i'?ve|ive|i\\s+have|we'?re|we\\s+are)\\b[^.!?,;]{0,22}?\n        \\b(?:swamped|snowed\\s+under|drowning|behind\\s+on|way\\s+behind|backed\\s+up)\n      | \\b(?:the\\s+)?(?:backlog|to[- ]?do\\s+list|housework|ironing|washing\\s+up|paperwork)\\b\n        [^.!?,;]{0,20}?\n        \\b(?:piling\\s+up|stacking\\s+up|mounting\\s+up|out\\s+of\\s+control|never\\s+ends)\n      | \\b(?:a\\s+)?(?:mountain|heap|pile|ton|tonne|stack)\\s+of\\s+(?:the\\s+|my\\s+|our\\s+)?[a-z]+\n      | \\b(?:piling|piling\\s+up|stacking\\s+up|mounting\\s+up)\\b\n    )\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY(call_builtin("frozenset", &[py_tuple(vec![py_str("rent"), py_str("mortgage"), py_str("bill"), py_str("bills"), py_str("debt"), py_str("debts"), py_str("arrears"), py_str("council"), py_str("loan"), py_str("loans"), py_str("overdraft"), py_str("credit"), py_str("card"), py_str("cards"), py_str("repayment"), py_str("repayments"), py_str("interest"), py_str("owe"), py_str("owed"), py_str("fines")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WEATHER(call_builtin("frozenset", &[py_tuple(vec![py_str("heat"), py_str("cold"), py_str("weather"), py_str("humidity"), py_str("sun"), py_str("rain"), py_str("summer"), py_str("winter"), py_str("wind"), py_str("snow"), py_str("frost"), py_str("traffic"), py_str("commute"), py_str("bus"), py_str("train"), py_str("tube")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STRETCH(call_builtin("frozenset", &[py_tuple(vec![py_str("week"), py_str("weeks"), py_str("month"), py_str("months"), py_str("day"), py_str("days"), py_str("year"), py_str("years"), py_str("term"), py_str("shift"), py_str("shifts"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("nights"), py_str("weekend"), py_str("fortnight"), py_str("time"), py_str("while"), py_str("hour"), py_str("hours")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EXTRA_BODY(call_builtin("frozenset", &[py_tuple(vec![py_str("pelvis"), py_str("bump"), py_str("tummy"), py_str("womb"), py_str("cervix"), py_str("abdomen")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_JOB(call_builtin("frozenset", &[py_tuple(vec![py_str("everything"), py_str("anything"), py_str("things"), py_str("thing"), py_str("stuff"), py_str("life"), py_str("all"), py_str("lot"), py_str("world"), py_str("loads"), py_str("too"), py_str("much"), py_str("up"), py_str("piling"), py_str("stacking"), py_str("mounting"), py_str("behind"), py_str("snowed"), py_str("under"), py_str("swamped"), py_str("drowning"), py_str("backed")])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING(call_builtin("frozenset", &[py_tuple(vec![py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("it"), py_str("its"), py_str("it's"), py_str("last"), py_str("past"), py_str("few"), py_str("couple"), py_str("of"), py_str("whole"), py_str("every"), py_str("all"), py_str("and"), py_str("been"), py_str("has"), py_str("have"), py_str("had"), py_str("always"), py_str("so"), py_str("far"), py_str("now"), py_str("recently"), py_str("lately"), py_str("today"), py_str("i"), py_str("im"), py_str("i'm"), py_str("ive"), py_str("i've"), py_str("we"), py_str("we're"), py_str("with"), py_str("to"), py_str("many"), py_str("again"), py_str("on"), py_str("out"), py_str("control"), py_str("sort"), py_str("sorting"), py_str("sorted")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROLE(call_builtin("frozenset", &[py_tuple(vec![py_str("work"), py_str("job"), py_str("jobs"), py_str("office"), py_str("employer"), py_str("boss"), py_str("school"), py_str("uni"), py_str("university"), py_str("college"), py_str("course"), py_str("degree"), py_str("business"), py_str("placement"), py_str("apprenticeship"), py_str("phd"), py_str("thesis")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PILE_REPLIES(py_tuple(vec![py_str("Then it's a sorting job before it's a doing job: what has to happen today, and what can wait until next week?"), py_str("Start with the smallest one and finish it before you look at the rest of the pile."), py_str("Piles like that shrink the moment you start. Which one is actually due first?"), py_str("Ugh. Pick the one that would be a relief to have done, and give it twenty minutes.")]));
    s_JOB_REPLIES(py_tuple(vec![py_str("That one's a slog. What's it fighting you on?"), py_str("It's usually the fiddly part that eats the time. Which bit is the worst?"), py_str("Ugh, the kind of job that takes longer than anyone says. Where are you up to with it?"), py_str("Fair enough - that sort of thing gets into your head. What's the bit you keep putting off?")]));
    s_ROLE_REPLIES(py_tuple(vec![py_str("Then something has to give. Is it the hours, the people, or the work itself?"), py_str("That wears you down. What's the worst part of it at the moment?"), py_str("Fair enough - a job like that takes it out of you. What's the bit that does it?"), py_str("That's worth taking seriously: what would make the week easier?")]));
    s_CONTENT(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shape: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shape"))) };
    let _ = py_str("The replies for a shape: a pile of things to do, one job that fights back, or the work\n    itself.");
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("pile")))) {
    return g_PILE_REPLIES();
    }
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("role")))) {
    return g_ROLE_REPLIES();
    }
    return g_JOB_REPLIES();
    Py::None
}
pub fn reads_as_worn(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { py_str("") };
    let mut shape: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("'pile' for a stack of things to do, 'job' for one thing that wears, None for anything else.\n\n    `repaired` is the same line with its wrong words put right (the reading's `plain`), and it is\n    only consulted when the line as typed is not a job: \"this spreadsheat is killing me\" is the\n    same complaint as the spelled-out line and only the repair knows it.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    shape = _shape(&[text.clone(), words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&shape.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &text.clone())) } } } }) {
    shape = _shape(&[repaired.clone()]);
    }
    return shape.clone();
    Py::None
}
pub fn _shape(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut match_: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("The shape of the wear in this one text, or None.");
    words = if py_truthy(&words.clone()) { call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("set", &[mcall(&g_CONTENT(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    if py_truthy(&crate::compose::trouble::someone_elses(&[words.clone()])) {
    return Py::None;
    }
    if py_truthy(&crate::compose::trouble::reads_as_trouble(&[text.clone(), words.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&crate::compose::trouble::g_PLEA(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&words.clone(), &g_WEATHER())) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&words.clone(), &g_MONEY())) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_PILE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&_names_a_job(&[text.clone()])) {
    return py_str("pile");
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_WEAR(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    subject = slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&g_CONTENT(), "findall", &[subject.clone()])]).unwrap_or_else(|e| exc::raise(e)), &op_bitor(&crate::compose::trouble::g_BODY_PART(), &g_EXTRA_BODY()))) {
    continue;
    }
    if py_truthy(&_is_a_role(&[subject.clone()])) {
    return py_str("role");
    }
    if py_truthy(&_names_a_job(&[subject.clone()])) {
    return py_str("job");
    }
    }
    return Py::None;
    Py::None
}
pub fn _content_words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The words of a clause that name something, without the ones that only point at it.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_CONTENT(), "findall", &[clause.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_POINTING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_STRETCH(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _is_a_role(args: &[Py]) -> Py {
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
    let mut content: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the wear is about their work or their course, and nothing else.\n\n    \"work is killing me\" and \"my job is a pain\" are the job itself; \"this spreadsheet is killing\n    me\" is one task inside it, which wants a question about the task.");
    content = _content_words(&[clause.clone()]);
    return { let __b = call_builtin("bool", &[content.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ROLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _names_a_job(args: &[Py]) -> Py {
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
    let mut content: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the clause names something to be done, not just a stretch of time.\n\n    \"this week has been a nightmare\" and \"the last few days have been a slog\" name a week and\n    days, and a question about where they are up to with the job is nonsense for both.");
    content = _content_words(&[clause.clone()]);
    return { let __b = call_builtin("bool", &[content.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_JOB(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.worn".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_STRETCH" => Some(g_A_STRETCH()),
            "CONTENT" => Some(g_CONTENT()),
            "EXTRA_BODY" => Some(g_EXTRA_BODY()),
            "JOB_REPLIES" => Some(g_JOB_REPLIES()),
            "MONEY" => Some(g_MONEY()),
            "NOT_A_JOB" => Some(g_NOT_A_JOB()),
            "PILE" => Some(g_PILE()),
            "PILE_REPLIES" => Some(g_PILE_REPLIES()),
            "POINTING" => Some(g_POINTING()),
            "ROLE" => Some(g_ROLE()),
            "ROLE_REPLIES" => Some(g_ROLE_REPLIES()),
            "WEAR" => Some(g_WEAR()),
            "WEATHER" => Some(g_WEATHER()),
            "_content_words" => Some(py_func("_content_words", |args: &[Py]| _content_words(args))),
            "_is_a_role" => Some(py_func("_is_a_role", |args: &[Py]| _is_a_role(args))),
            "_names_a_job" => Some(py_func("_names_a_job", |args: &[Py]| _names_a_job(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads_as_worn" => Some(py_func("reads_as_worn", |args: &[Py]| reads_as_worn(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_STRETCH" => s_A_STRETCH(v),
            "CONTENT" => s_CONTENT(v),
            "EXTRA_BODY" => s_EXTRA_BODY(v),
            "JOB_REPLIES" => s_JOB_REPLIES(v),
            "MONEY" => s_MONEY(v),
            "NOT_A_JOB" => s_NOT_A_JOB(v),
            "PILE" => s_PILE(v),
            "PILE_REPLIES" => s_PILE_REPLIES(v),
            "POINTING" => s_POINTING(v),
            "ROLE" => s_ROLE(v),
            "ROLE_REPLIES" => s_ROLE_REPLIES(v),
            "WEAR" => s_WEAR(v),
            "WEATHER" => s_WEATHER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.worn", module_obj);
    pyrt::xrefs::reg_fn("compose.worn", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_fn("compose.worn", "reads_as_worn", py_func("reads_as_worn", |args: &[Py]| reads_as_worn(args)));
    pyrt::xrefs::reg_fn("compose.worn", "_shape", py_func("_shape", |args: &[Py]| _shape(args)));
    pyrt::xrefs::reg_fn("compose.worn", "_content_words", py_func("_content_words", |args: &[Py]| _content_words(args)));
    pyrt::xrefs::reg_fn("compose.worn", "_is_a_role", py_func("_is_a_role", |args: &[Py]| _is_a_role(args)));
    pyrt::xrefs::reg_fn("compose.worn", "_names_a_job", py_func("_names_a_job", |args: &[Py]| _names_a_job(args)));
    pyrt::xrefs::reg_access("compose.worn", "A_STRETCH", g_A_STRETCH, s_A_STRETCH);
    pyrt::xrefs::reg_access("compose.worn", "CONTENT", g_CONTENT, s_CONTENT);
    pyrt::xrefs::reg_access("compose.worn", "EXTRA_BODY", g_EXTRA_BODY, s_EXTRA_BODY);
    pyrt::xrefs::reg_access("compose.worn", "JOB_REPLIES", g_JOB_REPLIES, s_JOB_REPLIES);
    pyrt::xrefs::reg_access("compose.worn", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("compose.worn", "NOT_A_JOB", g_NOT_A_JOB, s_NOT_A_JOB);
    pyrt::xrefs::reg_access("compose.worn", "PILE", g_PILE, s_PILE);
    pyrt::xrefs::reg_access("compose.worn", "PILE_REPLIES", g_PILE_REPLIES, s_PILE_REPLIES);
    pyrt::xrefs::reg_access("compose.worn", "POINTING", g_POINTING, s_POINTING);
    pyrt::xrefs::reg_access("compose.worn", "ROLE", g_ROLE, s_ROLE);
    pyrt::xrefs::reg_access("compose.worn", "ROLE_REPLIES", g_ROLE_REPLIES, s_ROLE_REPLIES);
    pyrt::xrefs::reg_access("compose.worn", "WEAR", g_WEAR, s_WEAR);
    pyrt::xrefs::reg_access("compose.worn", "WEATHER", g_WEATHER, s_WEATHER);
}

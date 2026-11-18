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


thread_local! { static G_AND_THE_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_NUMBER_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BEHIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPEAT_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AND_THE_NEXT() -> Py { ensure_init(); G_AND_THE_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_THE_NEXT(v: Py) { ensure_init(); G_AND_THE_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_AGAIN() -> Py { ensure_init(); G_ASKS_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_AGAIN(v: Py) { ensure_init(); G_ASKS_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_NAME() -> Py { ensure_init(); G_A_BARE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_NAME(v: Py) { ensure_init(); G_A_BARE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES() -> Py { ensure_init(); G_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES(v: Py) { ensure_init(); G_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_NUMBER_HERE() -> Py { ensure_init(); G_NO_NUMBER_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_NUMBER_HERE(v: Py) { ensure_init(); G_NO_NUMBER_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g__BEHIND() -> Py { ensure_init(); G__BEHIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BEHIND(v: Py) { ensure_init(); G__BEHIND.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_FRONT() -> Py { ensure_init(); G__IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_FRONT(v: Py) { ensure_init(); G__IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g__REPEAT_TURNS() -> Py { ensure_init(); G__REPEAT_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPEAT_TURNS(v: Py) { ensure_init(); G__REPEAT_TURNS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A person asking for a phone number gets the number the bot already knows.\n\n\"whats the number for the samaritans\", \"whats the number for 111\", \"childline number\",\n\"how do i ring acas\" - on main every one of those is answered by the same sentence:\n\n    \"No directory here, sorry. A map app search for the name shows the address and the\n     number, and the place's own page has the hours.\"\n\nThat is the right route for a shop or a surgery, whose number only a directory has. It is the\nwrong route for a national line, because the bot already states those numbers in its own cards:\n\n    116 123        Samaritans, free, any hour   (bot/modules/safety_cards.py, science.py, ...)\n    0808 2000 247  the domestic abuse helpline  (bot/modules/carers_turn.py)\n    0300 123 1100  ACAS                         (bot/knowledge/work.py)\n    111            NHS, free, any hour          (bot/modules/... many)\n    988            the US crisis line          (bot/knowledge/crisis_replies.py)\n    999 / 112       the emergency number        (bot/compose/verdict.py)\n\nSo this adds no fact. It reads the asking, finds which of the lines the person named, and says\nthat number with the one thing that makes it usable (whether it is free, and whether it is open\nany hour). A person in trouble should not be sent to a map app for the Samaritans, and this is\nthe turn they are most likely to type.\n\nNothing here guesses a number, and a line that names a shop, a surgery or a person - whose\nnumber nobody in the tree knows - gets None, so the directory route keeps it.\n");
    s_LINES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("samaritans"), py_str("samaritan")]), py_str("Samaritans is 116 123 - free, any hour.")]), py_tuple(vec![py_tuple(vec![py_str("domestic abuse helpline"), py_str("domestic violence helpline"), py_str("national domestic abuse helpline"), py_str("refuge"), py_str("womens aid"), py_str("women's aid")]), py_str("The domestic abuse helpline is 0808 2000 247 - free, any hour, and they will plan the next step with you.")]), py_tuple(vec![py_tuple(vec![py_str("acas")]), py_str("ACAS is 0300 123 1100, charged at your usual rate, and they will tell you where you stand on a work problem.")]), py_tuple(vec![py_tuple(vec![py_str("nhs 111"), py_str("111"), py_str("nhs direct")]), py_str("NHS 111 is on 111 - free, any hour, and they will say what to do about a symptom and where to go.")]), py_tuple(vec![py_tuple(vec![py_str("988")]), py_str("988 is the crisis line in the US - free, any hour, and you can call or text it.")]), py_tuple(vec![py_tuple(vec![py_str("999"), py_str("911"), py_str("112"), py_str("emergency services")]), py_str("Ring 999 - or 911, or 112 - and say where you are and what has happened.")])]));
    s_ASKS(py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what(?:'s|s| is)?\\s+the\\s+|what\\s+is\\s+the\\s+|the\\s+|whats\\s+|what\\s+)?(?:phone\\s+|telephone\\s+)?number\\s+(?:for|of|to)\\s+(.{2,60})$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:do you have|have you got|any idea of)\\s+(?:a\\s+|the\\s+)?(?:phone\\s+)?number\\s+(?:for|of|to)\\s+(.{2,60})$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how do i|how can i|how to|where do i|can i|do i)\\s+(?:ring|call|phone|dial|contact|reach)\\s+(.{2,60})$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what(?:'s|s| is)?\\s+the\\s+)?(?:phone\\s+)?number\\s+(?:for\\s+)?(.{2,40}?)\\s+again$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(.{2,40}?)(?:'s|s)?\\s+(?:phone\\s+)?number$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(.{2,40}?)\\s+(?:phone\\s+)?number\\s*\\?*$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("(?<![a-z])([a-z][a-z' ]{1,40}?)\\s+(?:phone\\s+)?number\\s*[?!.]*$")]).unwrap_or_else(|e| exc::raise(e))]));
    s_NO_NUMBER_HERE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("childline")]), py_str("I don't have Childline's number here. A search for 'childline' brings up their own page, and the free number is on it.")]), py_tuple(vec![py_tuple(vec![py_str("alcoholics anonymous"), py_str("al anon"), py_str("alanon")]), py_str("I don't have Alcoholics Anonymous's number here. A search for 'alcoholics anonymous' brings up their own page, and a local meeting line is on it.")]), py_tuple(vec![py_tuple(vec![py_str("mind"), py_str("shelter"), py_str("stepchange"), py_str("age uk"), py_str("ageuk"), py_str("victim support"), py_str("cruse"), py_str("nspcc"), py_str("rethink")]), py_str("I don't hold that number. Their own page is where it lives - a search for the name brings it up, and a helpline's number is the one on it (a map app would only give you a branch).")])]));
    s__REPEAT_TURNS(Py::Int(3));
    s_AND_THE_NEXT(stdlib_call("re", "compile", &[py_str("^(?:and|also|what about|how about|whats|what is|what's)\\b[\\s,]*(?:the\\s+|a\\s+)?(?:number\\s+(?:for|of)\\s+)?(?:for\\s+|of\\s+)?(.{2,40})$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_NAME(stdlib_call("re", "compile", &[py_str("^[a-z][a-z' ]{1,30}$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:again|what was (?:it|that)|remind me|one more time|i missed (?:it|that))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_FRONT(stdlib_call("re", "compile", &[py_str("^(?:sorry|hi|hello|hey|ok|okay|oh|so|and|also|er|um)\\s*[,!.]\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    s__BEHIND(stdlib_call("re", "compile", &[py_str("\\s*\\b(?:again|please|one more time|now|thanks|thank you)\\b\\s*[?!.]*$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _said_before(args: &[Py]) -> Py {
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
    let mut _names_: Py = Py::None;
    let mut last: Py = Py::None;
    let mut replies: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line this module gave on the last turn, if that is what the last turn was, else \"\".");
    replies = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_replies(&[memory.clone()])) { let mut reply: Py = __item.clone(); if py_truthy(&reply.clone()) {  __out.push(reply.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&replies.clone()))) {
    return py_str("");
    }
    last = geti(&replies.clone(), &op_neg(&Py::Int(1)));
    let __iter1 = py_iter_or_raise(&op_add(&g_LINES(), &g_NO_NUMBER_HERE()));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _names_ = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&last.clone(), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])))) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _carried_thing(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The thing a line that only points back at the last question is asking about, or \"\".");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?!.,")]);
    found = mcall(&g_AND_THE_NEXT(), "match", &[line.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ?!.,'\"")]);
    }
    if py_truthy(&mcall(&g_A_BARE_NAME(), "match", &[line.clone()])) {
    return line.clone();
    }
    return py_str("");
    Py::None
}
pub fn _asked_about(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing whose number is wanted, cleaned of trailing punctuation, or \"\".");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    line = mcall(&line.clone(), "strip", &[py_str(" ?!.,")]);
    line = mcall(&mcall(&g__IN_FRONT(), "sub", &[py_str(""), mcall(&g__BEHIND(), "sub", &[py_str(""), line.clone()])]), "strip", &[py_str(" ?!.,")]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_ASKS());
    for __item in __iter1 {
    ask = __item.clone();
    found = mcall(&ask.clone(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    thing = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ?!.,'\"")]);
    thing = stdlib_call("re", "sub", &[py_str("^(?:the|a|an)\\s+"), py_str(""), thing.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&thing.clone()) {
    return thing.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut name: Py = Py::None;
    let _ = py_str("True when the thing wanted is one of the names, as a whole word or a whole phrase.\n\n    Whole words, because \"refuge\" is inside \"refugees\" and \"111\" inside \"1111\": the number asked\n    about has to be the one named, not a word that holds it.\n    ");
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), thing.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _replies(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the bot said on each turn, whichever shape the store keeps a turn in.\n\n    The store keeps a turn as a dict in some builds and as a (user, bot) pair in others, so reading\n    only the dict shape made both the guard below and the \"asked again\" reader above silently never\n    fire - the fault that let \"sorry, what was it again\" be answered with a hedge.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&turn.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    } else {
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[turn.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = geti(&turn.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    } else {
    let _ = mcall(&out.clone(), "append", &[py_str("")]);
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn already_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the bot has said this number in the last few turns.");
    said = mcall(&call_builtin("str", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str(" "), py_str("")]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&_replies(&[memory.clone()]), &op_neg(&g__REPEAT_TURNS()), &Py::None, &Py::None));
    for __item in __iter1 {
    reply = __item.clone();
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&reply.clone(), "replace", &[py_str(" "), py_str("")]), &said.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn not_a_number(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply the bot is about to send holds no number at all.\n\n    \"i need to talk to someone, whats the samaritans number\" was answered \"Worth doing. What would\n    you like help with first?\" - a question about the person's day when they asked for a crisis\n    line. A reply with no digit in it cannot be the answer to a number asked for, so the number the\n    bot knows is given instead. It only matters for the lines above, which are few and named.\n    ");
    return Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
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
    else { Py::None };
    let mut again: Py = Py::None;
    let mut before: Py = Py::None;
    let mut line: Py = Py::None;
    let mut names: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The number, when the person asked for one this bot knows. None for everything else.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    again = call_builtin("bool", &[mcall(&g_ASKS_AGAIN(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    before = _said_before(&[memory.clone()]);
    thing = _asked_about(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if !py_truthy(&__b) { __b } else { before.clone() } }) {
    thing = _carried_thing(&[text.clone()]);
    }
    if py_truthy(&thing.clone()) {
    let __iter1 = py_iter_or_raise(&g_LINES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_names(&[thing.clone(), names.clone()])))) {
    continue;
    }
    number = stdlib_call("re", "search", &[py_str("\\d[\\d ]{2,}"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let __b = already_said(&[mcall(&number.clone(), "group", &[Py::Int(0)]), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&again.clone())) } } } }) {
    return Py::None;
    }
    return said.clone();
    }
    let __iter3 = py_iter_or_raise(&g_NO_NUMBER_HERE());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    names = __unpk4[0].clone();
    said = __unpk4[1].clone();
    if py_truthy(&_names(&[thing.clone(), names.clone()])) {
    return said.clone();
    }
    }
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { again.clone() } }) {
    return before.clone();
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_number_the_bot_knows".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AND_THE_NEXT" => Some(g_AND_THE_NEXT()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_AGAIN" => Some(g_ASKS_AGAIN()),
            "A_BARE_NAME" => Some(g_A_BARE_NAME()),
            "LINES" => Some(g_LINES()),
            "NO_NUMBER_HERE" => Some(g_NO_NUMBER_HERE()),
            "_BEHIND" => Some(g__BEHIND()),
            "_IN_FRONT" => Some(g__IN_FRONT()),
            "_REPEAT_TURNS" => Some(g__REPEAT_TURNS()),
            "_asked_about" => Some(py_func("_asked_about", |args: &[Py]| _asked_about(args))),
            "_carried_thing" => Some(py_func("_carried_thing", |args: &[Py]| _carried_thing(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_replies" => Some(py_func("_replies", |args: &[Py]| _replies(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "already_said" => Some(py_func("already_said", |args: &[Py]| already_said(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "not_a_number" => Some(py_func("not_a_number", |args: &[Py]| not_a_number(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AND_THE_NEXT" => s_AND_THE_NEXT(v),
            "ASKS" => s_ASKS(v),
            "ASKS_AGAIN" => s_ASKS_AGAIN(v),
            "A_BARE_NAME" => s_A_BARE_NAME(v),
            "LINES" => s_LINES(v),
            "NO_NUMBER_HERE" => s_NO_NUMBER_HERE(v),
            "_BEHIND" => s__BEHIND(v),
            "_IN_FRONT" => s__IN_FRONT(v),
            "_REPEAT_TURNS" => s__REPEAT_TURNS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_number_the_bot_knows", module_obj);
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "_carried_thing", py_func("_carried_thing", |args: &[Py]| _carried_thing(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "_asked_about", py_func("_asked_about", |args: &[Py]| _asked_about(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "_replies", py_func("_replies", |args: &[Py]| _replies(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "already_said", py_func("already_said", |args: &[Py]| already_said(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "not_a_number", py_func("not_a_number", |args: &[Py]| not_a_number(args)));
    pyrt::xrefs::reg_fn("compose.a_number_the_bot_knows", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "AND_THE_NEXT", g_AND_THE_NEXT, s_AND_THE_NEXT);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "ASKS_AGAIN", g_ASKS_AGAIN, s_ASKS_AGAIN);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "A_BARE_NAME", g_A_BARE_NAME, s_A_BARE_NAME);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "LINES", g_LINES, s_LINES);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "NO_NUMBER_HERE", g_NO_NUMBER_HERE, s_NO_NUMBER_HERE);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "_BEHIND", g__BEHIND, s__BEHIND);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "_IN_FRONT", g__IN_FRONT, s__IN_FRONT);
    pyrt::xrefs::reg_access("compose.a_number_the_bot_knows", "_REPEAT_TURNS", g__REPEAT_TURNS, s__REPEAT_TURNS);
}

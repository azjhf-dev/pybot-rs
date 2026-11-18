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


thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTINUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GO_ON_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.more: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTINUE() -> Py { ensure_init(); G_CONTINUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTINUE(v: Py) { ensure_init(); G_CONTINUE.with(|c| *c.borrow_mut() = v); }
pub fn g_GO_ON_WITH() -> Py { ensure_init(); G_GO_ON_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GO_ON_WITH(v: Py) { ensure_init(); G_GO_ON_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_ABOUT() -> Py { ensure_init(); G_MORE_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_ABOUT(v: Py) { ensure_init(); G_MORE_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_OF_IT() -> Py { ensure_init(); G_MORE_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_OF_IT(v: Py) { ensure_init(); G_MORE_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MORE() -> Py { ensure_init(); G_NO_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MORE(v: Py) { ensure_init(); G_NO_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g__BACK() -> Py { ensure_init(); G__BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BACK(v: Py) { ensure_init(); G__BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__FRONT() -> Py { ensure_init(); G__FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRONT(v: Py) { ensure_init(); G__FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD() -> Py { ensure_init(); G__WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD(v: Py) { ensure_init(); G__WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("They asked the bot to go on, and got told to run a search.\n\n\"tell me more\", \"go on\", \"and?\", \"the rest of it\" is the ordinary way to keep a friend talking\nabout the thing they are already on. The line names nothing of its own, so no entry can answer\nit, and the bot fell through to the plain hedge (\"A search should sort you out quickly\") - a turn\nafter it had answered the subject in full. That reads as if the answer had never been given: the\nperson is asking about the answer, and a search engine cannot see it.\n\nSo the answer behind the ask is handed back to, in the shape a friend uses when they have said\nall they know: name the subject, say it is the whole of it, and ask which part to take further.\nNothing is invented, because there is nothing more in the base to give - an answer the bot cannot\nback is worse than saying plainly that it has finished.\n\nThe rule is narrow on purpose:\n* the whole line has to be the continuation and nothing else (\"tell me more about the boiler\"\n  names a subject and is answered from its own words);\n* \"what else\" and \"anything else\" are not here: they ask for another option, which\n  bot/compose/other.py answers from the answer already given;\n* only the turn just before counts, and only an answer: \"tell me more\" after a stock line or a\n  refusal has nothing behind it to go on with;\n* the bot's own hedges are never continued from.\n\nThe one thing that is handed over before saying there is nothing left: an answer that came back a\nsecond time in its short form was still the whole answer, and the part the short form left out has\nnot been said again, so the person asking about it gets that part (`_the_rest`).\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more._FRONT"); }
    s__FRONT(py_str("(?:(?:and|so|ok|okay|right|well|alright|yes|yeah|hmm|ta|please|then|but)\\b[\\s,]+)*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more._BACK"); }
    s__BACK(py_str("(?:[\\s,]+(?:then|please|now|tho|though|thanks|ta|about (?:it|that|this)|on (?:it|that|this)|of (?:it|that)|for (?:it|that)))*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.CONTINUE"); }
    s_CONTINUE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__FRONT()), &py_str("(?:tell me (?:a bit |a little |a lot )?more|more|more please|more detail|more details|in more detail|a bit more|a little more|go on|go on then|go ahead|carry on|keep going|keep talking|on you go|continue|say more|say a bit more|keep it coming|(?:tell me )?the rest(?: of (?:it|that|this))?|elaborate|expand|go deeper|dig deeper|and\\?|and then\\?)")), &g__BACK()), &py_str("[\\s.?!,]*$")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.AN_ASK"); }
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("^(?:tell me|let me know|say (?:which|what|how)|which|what|have you|do you|are you|is there|how (?:long|much|many|often))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.GO_ON_WITH"); }
    s_GO_ON_WITH(py_tuple(vec![py_str("Gladly. {ask}"), py_str("Happy to. {ask}"), py_str("Yes - and the next step is yours: {ask}"), py_str("That is the bit that decides it: {ask}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.NO_MORE"); }
    s_NO_MORE(py_tuple(vec![py_str("That's the whole of what I know on it - tell me which bit to take further and I'll come at it from another side."), py_str("That's about the lot I have on that. Which part do you want me to go into?"), py_str("I have given you all of it. If you say what is still not working I'll take it from there."), py_str("That answer is everything I have on it. Point me at the part that is not clear and I'll put it another way.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.MORE_ABOUT"); }
    s_MORE_ABOUT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^"), &g__FRONT()), &py_str("(?:tell me (?:a bit |a little )?(?:more|about)|more|go on|carry on|keep going|say more|the rest(?: of it)?) (?:about|on) (?P<what>[a-z0-9' ]{2,40})")), &g__BACK()), &py_str("[\\s.?!,]*$")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more._WORD"); }
    s__WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.more.MORE_OF_IT"); }
    s_MORE_OF_IT(py_tuple(vec![py_str("There was more to that answer: {rest}"), py_str("That answer had more in it: {rest}"), py_str("And the rest of what I said: {rest}")]));
}
pub fn _its_ask(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The thing the answer asked for at its end, or \"\" when it asked for nothing.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.repeat", "SENTENCE"), "split", &[mcall(&said.clone(), "strip", &[])])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    last = geti(&parts.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&last.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_AN_ASK(), "match", &[last.clone()]) } }))) {
    return py_str("");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&last.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(22))) {
    return py_str("");
    }
    return last.clone();
    Py::None
}
pub fn _as_a_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The answer's ask as a sentence of its own: a capital only where one belongs.");
    said = mcall(&ask.clone(), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(2), &Py::None), "isupper", &[]))) } }) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("?"), py_str("!")])])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    Py::None
}
pub fn _saying_something(args: &[Py]) -> Py {
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
    let _ = py_str("The line as tidied words, or \"\" when it names a subject this reader cannot check.");
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _about_what(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("\"tell me more about X\": X, when the line is that ask and X is not \"it\" or \"that\".");
    match_ = mcall(&g_MORE_ABOUT(), "match", &[_saying_something(&[text.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    what = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("those")]), &mcall(&what.clone(), "lower", &[])))) {
    return py_str("");
    }
    return what.clone();
    Py::None
}
pub fn _is_what_was_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut what: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut asked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut known: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every word of the thing named is a word of the line the answer was given to.");
    known = call_builtin("set", &[mcall(&mcall(&call_builtin("str", &[{ let __b = asked.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __aug1 = op_bitor(&known.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() });
    known = __aug1;
    let __iter2 = py_iter_or_raise(&mcall(&mcall(&what.clone(), "lower", &[]), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("our"), py_str("this"), py_str("that"), py_str("it")]), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&known.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&known.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
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
    return mcall(&py_str(" "), "join", &[mcall(&g__WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]);
    Py::None
}
pub fn _the_rest(args: &[Py]) -> Py {
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
    let mut answers: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answers"))) };
    let mut _asked: Py = Py::None;
    let mut _back: Py = Py::None;
    let mut fuller: Py = Py::None;
    let mut given: Py = Py::None;
    let mut index: Py = Py::None;
    let mut left: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The part of a fuller answer from a turn or two back that the short one left out, or \"\".\n\n    The fuller answer has to open with the sentence just given, so it is the same answer and not\n    another one on the same subject, and every sentence before the first leftover one has to be in\n    the short answer already, so what is handed over is the part that has not been said.\n    ");
    given = _flat(&[said.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&given.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&answers.clone())) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&answers.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _back = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    fuller = __unpk2[2].clone();
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.repeat", "SENTENCE"), "split", &[mcall(&call_builtin("str", &[{ let __b = fuller.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&given.clone(), &_flat(&[geti(&parts.clone(), &Py::Int(0))]))) } }) {
    continue;
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5361 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk5361[0].clone(); let mut part: Py = __unpk5361[1].clone(); if py_truthy(&Py::Bool(!py_contains(&given.clone(), &_flat(&[part.clone()])))) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&left.clone(), &Py::Int(0)), &Py::Int(0))) } }) {
    continue;
    }
    return mcall(&py_str(" "), "join", &[slice_or_raise(&parts.clone(), &geti(&left.clone(), &Py::Int(0)), &Py::None, &Py::None)]);
    }
    return py_str("");
    Py::None
}
pub fn asks_for_more(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the whole line is \"carry on\" and nothing else.");
    said = _saying_something(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_CONTINUE(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MORE_ABOUT(), "match", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut answers: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut named: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Keep going on the answer just given, or None when this is not that turn.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_for_more(&[text.clone()]))) } }) {
    return Py::None;
    }
    named = _about_what(&[text.clone()]);
    answers = py_call(&pyrt::xrefs::xget("memory.repeat", "recent_answers"), &[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answers.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&geti(&answers.clone(), &Py::Int(0)), 3, None);
    back = __unpk1[0].clone();
    asked = __unpk1[1].clone();
    said = __unpk1[2].clone();
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("memory.repeat", "same_subject"), &[asked.clone(), said.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_what_was_asked(&[named.clone(), asked.clone()]))) } }) {
    return Py::None;
    }
    rest = _the_rest(&[said.clone(), answers.clone()]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("memory.repeat", "said_recently"), &[rest.clone(), memory.clone()]))) } }) {
    return pyrt::xrefs::xcall("memory.repeat", "handed_back", &[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rest".to_string(), rest.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_MORE_OF_IT(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    ask = _its_ask(&[said.clone()]);
    if py_truthy(&ask.clone()) {
    return pyrt::xrefs::xcall("memory.repeat", "handed_back", &[memory.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("ask".to_string(), _as_a_line(&[ask.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_GO_ON_WITH(), memory.clone(), text.clone()]), "format", args))(&__args) }]);
    }
    reply = pyrt::xrefs::xcall("compose", "pick", &[g_NO_MORE(), memory.clone(), text.clone()]);
    if py_truthy(&py_call(&pyrt::xrefs::xget("memory.repeat", "said_recently"), &[reply.clone(), memory.clone()])) {
    reply = py_str("That is all of it. Tell me what you have tried and I will work from that.");
    if py_truthy(&py_call(&pyrt::xrefs::xget("memory.repeat", "said_recently"), &[reply.clone(), memory.clone()])) {
    return Py::None;
    }
    }
    return pyrt::xrefs::xcall("memory.repeat", "handed_back", &[memory.clone(), reply.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.more".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ASK" => Some(g_AN_ASK()),
            "CONTINUE" => Some(g_CONTINUE()),
            "GO_ON_WITH" => Some(g_GO_ON_WITH()),
            "MORE_ABOUT" => Some(g_MORE_ABOUT()),
            "MORE_OF_IT" => Some(g_MORE_OF_IT()),
            "NO_MORE" => Some(g_NO_MORE()),
            "_BACK" => Some(g__BACK()),
            "_FRONT" => Some(g__FRONT()),
            "_WORD" => Some(g__WORD()),
            "_about_what" => Some(py_func("_about_what", |args: &[Py]| _about_what(args))),
            "_as_a_line" => Some(py_func("_as_a_line", |args: &[Py]| _as_a_line(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_is_what_was_asked" => Some(py_func("_is_what_was_asked", |args: &[Py]| _is_what_was_asked(args))),
            "_its_ask" => Some(py_func("_its_ask", |args: &[Py]| _its_ask(args))),
            "_saying_something" => Some(py_func("_saying_something", |args: &[Py]| _saying_something(args))),
            "_the_rest" => Some(py_func("_the_rest", |args: &[Py]| _the_rest(args))),
            "asks_for_more" => Some(py_func("asks_for_more", |args: &[Py]| asks_for_more(args))),
            "pick" => Some(g_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "repeat" => Some(pyrt::xrefs::xmodule("memory.repeat")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ASK" => s_AN_ASK(v),
            "CONTINUE" => s_CONTINUE(v),
            "GO_ON_WITH" => s_GO_ON_WITH(v),
            "MORE_ABOUT" => s_MORE_ABOUT(v),
            "MORE_OF_IT" => s_MORE_OF_IT(v),
            "NO_MORE" => s_NO_MORE(v),
            "_BACK" => s__BACK(v),
            "_FRONT" => s__FRONT(v),
            "_WORD" => s__WORD(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.more", module_obj);
    pyrt::xrefs::reg_fn("memory.more", "_its_ask", py_func("_its_ask", |args: &[Py]| _its_ask(args)));
    pyrt::xrefs::reg_fn("memory.more", "_as_a_line", py_func("_as_a_line", |args: &[Py]| _as_a_line(args)));
    pyrt::xrefs::reg_fn("memory.more", "_saying_something", py_func("_saying_something", |args: &[Py]| _saying_something(args)));
    pyrt::xrefs::reg_fn("memory.more", "_about_what", py_func("_about_what", |args: &[Py]| _about_what(args)));
    pyrt::xrefs::reg_fn("memory.more", "_is_what_was_asked", py_func("_is_what_was_asked", |args: &[Py]| _is_what_was_asked(args)));
    pyrt::xrefs::reg_fn("memory.more", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.more", "_the_rest", py_func("_the_rest", |args: &[Py]| _the_rest(args)));
    pyrt::xrefs::reg_fn("memory.more", "asks_for_more", py_func("asks_for_more", |args: &[Py]| asks_for_more(args)));
    pyrt::xrefs::reg_fn("memory.more", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.more", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("memory.more", "CONTINUE", g_CONTINUE, s_CONTINUE);
    pyrt::xrefs::reg_access("memory.more", "GO_ON_WITH", g_GO_ON_WITH, s_GO_ON_WITH);
    pyrt::xrefs::reg_access("memory.more", "MORE_ABOUT", g_MORE_ABOUT, s_MORE_ABOUT);
    pyrt::xrefs::reg_access("memory.more", "MORE_OF_IT", g_MORE_OF_IT, s_MORE_OF_IT);
    pyrt::xrefs::reg_access("memory.more", "NO_MORE", g_NO_MORE, s_NO_MORE);
    pyrt::xrefs::reg_access("memory.more", "_BACK", g__BACK, s__BACK);
    pyrt::xrefs::reg_access("memory.more", "_FRONT", g__FRONT, s__FRONT);
    pyrt::xrefs::reg_access("memory.more", "_WORD", g__WORD, s__WORD);
    pyrt::xrefs::reg_access("memory.more", "pick", g_pick, s_pick);
}

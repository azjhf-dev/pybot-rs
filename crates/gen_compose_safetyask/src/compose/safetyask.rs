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


thread_local! { static G_ASK_IF_A_THING_IS_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DANGER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FAULT_DESCRIBED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RULING_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAFETY_ALREADY_RULED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAFETY_TEST_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SAFETY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_THAT_PLUGS_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_bot: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.safetyask: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASK_IF_A_THING_IS_SAFE() -> Py { ensure_init(); G_ASK_IF_A_THING_IS_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_IF_A_THING_IS_SAFE(v: Py) { ensure_init(); G_ASK_IF_A_THING_IS_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DANGER_SAID() -> Py { ensure_init(); G_A_DANGER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DANGER_SAID(v: Py) { ensure_init(); G_A_DANGER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FAULT_DESCRIBED() -> Py { ensure_init(); G_A_FAULT_DESCRIBED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FAULT_DESCRIBED(v: Py) { ensure_init(); G_A_FAULT_DESCRIBED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RULING_ALREADY() -> Py { ensure_init(); G_A_RULING_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RULING_ALREADY(v: Py) { ensure_init(); G_A_RULING_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAFETY_ALREADY_RULED() -> Py { ensure_init(); G_A_SAFETY_ALREADY_RULED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAFETY_ALREADY_RULED(v: Py) { ensure_init(); G_A_SAFETY_ALREADY_RULED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAFETY_TEST_LINES() -> Py { ensure_init(); G_A_SAFETY_TEST_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAFETY_TEST_LINES(v: Py) { ensure_init(); G_A_SAFETY_TEST_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SAFETY_WORD() -> Py { ensure_init(); G_A_SAFETY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SAFETY_WORD(v: Py) { ensure_init(); G_A_SAFETY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_THAT_PLUGS_IN() -> Py { ensure_init(); G_A_THING_THAT_PLUGS_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_THAT_PLUGS_IN(v: Py) { ensure_init(); G_A_THING_THAT_PLUGS_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_last_bot() -> Py { ensure_init(); G_last_bot.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_bot(v: Py) { ensure_init(); G_last_bot.with(|c| *c.borrow_mut() = v); }
pub fn g_last_user() -> Py { ensure_init(); G_last_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_user(v: Py) { ensure_init(); G_last_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reply to a safety question about a thing of theirs.\n\nThe second turn of a thread about something in the house: \"boiler dripping water\" / \"is it safe\nto keep it on tonight\", \"the plug socket is cracked\" / \"can i still use it\", \"my heater smells of\nburning\" / \"can i leave it on overnight\", \"my laptop charger is frayed\" / \"is it safe to keep\nusing it\", \"is the kettle safe to leave on\". Before this reading those reached the repeat frame\n(\"That was the whole of my answer on it. Tell me which part to take further and I will\"), the\nsearch shrug (\"That's beyond what I know well, sorry\") and \"I'm not sure about that one, sorry -\nI'd rather not guess\". None of the three is a reply to somebody standing next to a dripping\nboiler asking whether to leave it on overnight.\n\nThe person is asking one thing - do I stop using it tonight - and its answer does not depend on\nthe make: what makes a thing a stop is a burning smell, sparks, or a plug or socket hot to the\ntouch, with water near the electrics as the same kind of tell; failing those it can wait for the\nmorning. Every line says both halves, because \"is it safe\" is a yes or no.\n\nTwo things keep this reading narrow. The thing has to be one with a plug, a flame or water in it,\nnamed in this line or in the line before, so a knee, a child with a temperature and a question\nabout what to cook are left alone. And the knowledge module is asked first: where its card names\nthe same thing (\"a charger that is warm is normal\", \"what makes a boiler unsafe is...\"), the card\nkeeps the turn and this reading says nothing. That is the same rule as #6308's bare doubt - an\nentry takes the turn on a word of the subject, not on a word of the line.\n\nWhen the bot's own last reply has already ruled (\"don't use it again until it has been checked\"),\nthe answer is that ruling said once, not a fresh test and not a shrug.\n\nThis lives in its own file because `bot/compose/fallback.py` is within 3.4 kB of the 1 MiB the\nworld will upload, and a reading of this size cannot go in it. The caller owns everything wider:\n`fallback._reply_for` decides when the composer is allowed to speak at all, and picks the line.\n");
    s_last_bot(py_func("last_bot", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_bot", args)));
    s_last_user(py_func("last_user", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_user", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.ASK_IF_A_THING_IS_SAFE"); }
    s_ASK_IF_A_THING_IS_SAFE(stdlib_call("re", "compile", &[py_str("\\bis\\s+(?:it|that|this|they)\\s+(?:safe|dangerous|a\\s+(?:fire\\s+)?(?:risk|hazard))\\b|\\bis\\s+it\\s+(?:safe|alright|ok|okay)\\s+to\\s+(?:use|keep|leave|run|plug|carry\\s+on)\\b|\\bis\\s+(?:the|my|our)\\s+[a-z']{3,20}\\s+(?:safe|dangerous|a\\s+(?:fire\\s+)?(?:risk|hazard))\\b|\\bcan\\s+i\\s+(?:still\\s+)?(?:use|keep|leave|run|plug)\\s+(?:it|this|that)\\b|\\bcan\\s+(?:it|that|this)\\s+be\\s+(?:used|left|kept)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_THING_THAT_PLUGS_IN"); }
    s_A_THING_THAT_PLUGS_IN(call_builtin("frozenset", &[mcall(&py_str("\n    boiler heater charger plug socket cable lead extension kettle toaster oven hob grill\n    microwave fridge freezer washing machine washer tumble dryer dishwasher immersion shower\n    radiator thermostat lamp light lights fan dehumidifier laptop phone mobile tablet\n    hairdryer straighteners iron blanket fire chimney gas electrics wiring fuse\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_SAFETY_WORD"); }
    s_A_SAFETY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:safe|safely|unsafe|dangerous|danger|risk|risky|hazard|hazardous|fire|electrocut\\w*|shock|shocks|shocking)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_FAULT_DESCRIBED"); }
    s_A_FAULT_DESCRIBED(stdlib_call("re", "compile", &[py_str("\\b(?:cracked|cracking|frayed|fraying|exposed|bare|sparks?|sparking|smoking|smokes|smelt|smells?|burning|burnt|scorched|melting|melted|toasted|hot|overheat\\w*|buzzing|buzzes|fizzing|tripping|trips|dripping|drips|leaking|leaks|leak|wet|damp|loose|live|popped|banging|loud\\s+bang)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_SAFETY_TEST_LINES"); }
    s_A_SAFETY_TEST_LINES(py_tuple(vec![py_str("The test with {thing} is a burning smell, sparks, or heat you would not leave the room with: any of those and switch it off at the wall and leave it until somebody has looked at it. None of them and it will keep until the morning."), py_str("Three things make {thing} a no for tonight - a burning smell, sparks, or a plug or socket that is hot to the touch. Any of those, off at the wall and not again until it has been looked at. If it is only a noise or a drip, it can wait for the morning."), py_str("Nothing about {thing} needs deciding tonight unless it smells of burning, or water is anywhere near the electrics: then switch it off at the wall and have it looked at. Otherwise leave it be and ring somebody in the morning."), py_str("Keep it plain with {thing}: if it smells of burning or the socket is hot, off at the wall and no more of it tonight. A drip or a rattle can wait for the morning - bowl under it, nothing electrical near it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_RULING_ALREADY"); }
    s_A_RULING_ALREADY(py_tuple(vec![py_str("off at the wall"), py_str("unplug"), py_str("don't use it"), py_str("do not use it"), py_str("stop using it"), py_str("turn it off"), py_str("switch it off"), py_str("ring 999"), py_str("call 999"), py_str("get out")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_SAFETY_ALREADY_RULED"); }
    s_A_SAFETY_ALREADY_RULED(py_tuple(vec![py_str("No - the answer above still stands: it stays off until somebody has looked at it."), py_str("Same answer as before: not used again until it has been checked."), py_str("That has not changed: off, and looked at before it goes back on.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.safetyask.A_DANGER_SAID"); }
    s_A_DANGER_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:fires?|flames?|smoke|shocks?|shocking|electrocut\\w*|burns?|burnt|scorch\\w*|unsafe|kills?|fatal|explodes?|explosion|catch(?:es)? fire)\\b")]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut card: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The lines for a safety question about a thing of theirs, or None.\n\n    None wherever somebody else has the better answer: a knowledge card that names the thing the\n    question is about, and a thing that is not one of the above (a body, a person, a food).\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASK_IF_A_THING_IS_SAFE(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    thing = _the_thing_on_the_table(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _a_safety_word(&[text.clone()]); if py_truthy(&__b) { __b } else { _the_thing_is_described_faulty(&[memory.clone(), thing.clone()]) } }))) {
    return Py::None;
    }
    card = _the_answers_have_this(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = card.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&mcall(&card.clone(), "lower", &[]), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_shrug(&[card.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&the_bot_already_ruled(&[memory.clone()])) {
    return g_A_SAFETY_ALREADY_RULED();
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_SAFETY_TEST_LINES()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), op_add(&py_str("the "), &thing.clone()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_safety_word(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line itself asks about safety rather than only about using a thing.");
    return call_builtin("bool", &[mcall(&g_A_SAFETY_WORD(), "search", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_is_described_faulty(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut said: Py = Py::None;
    let _ = py_str("True when the line before describes the thing as faulty in a way that could bite.\n\n    The fault has to be beside the thing: \"my heater smells of burning\" is, \"my laptop is slow\"\n    is not, and a fault word from an unrelated line further back is not either.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &thing.clone())) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_FAULT_DESCRIBED(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_shrug(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when an answer to this line says the bot cannot answer it.\n\n    A card that names the thing keeps the turn, and so does any answer of its own: the only reply\n    this reading takes the turn from is one that sends the person off to search.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("i don't know enough"), py_str("i do not know enough"), py_str("beyond what i know"), py_str("rather not guess"), py_str("not sure about that one"), py_str("i'm not sure about that"), py_str("search should sort"), py_str("quick search"), py_str("search online"), py_str("better sources"), py_str("can't call that one"), py_str("cant call that one")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_on_the_table(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The appliance a follow-up safety question is about, or \"\" when there is not one.\n\n    Read from the line itself (\"is the boiler safe to leave on\") and, failing that, from the\n    thing the person named in the line before (\"boiler dripping water\" / \"is it safe to keep it\n    on tonight\"). The last one found is the thing the question is about: \"the plug socket is\n    cracked\" is about the socket, and the words before it are how it is being described.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), pyrt::xrefs::xcall("memory", "last_user", &[memory.clone()])]));
    for __item in __iter1 {
    text = __item.clone();
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']{3,20}"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_A_THING_THAT_PLUGS_IN(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&found.clone()) {
    return geti(&found.clone(), &op_neg(&Py::Int(1)));
    }
    }
    return py_str("");
    Py::None
}
pub fn the_bot_already_ruled(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the bot's own last reply told the person to stop using the thing, and why not to.\n\n    Read by the verdict reading too, so an answer that ruled a thing unsafe is not followed by an\n    all-clear (\"Ordinary on its own, yes\") about the same thing. The harm has to be in the same\n    answer: a how-to says \"unplug it first\" of everything in the house.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_bot", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_RULING_ALREADY()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_DANGER_SAID(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_answers_have_this(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answers: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The knowledge module's own answer to this line, or None.\n\n    Asked with the conversation behind it, because the card for \"is it safe to leave it plugged\n    in\" is built from the charger the line before named. A card that does not mention the thing\n    on the table is not an answer to this question: the freezer in the garage reached the card\n    about a car's brakes.\n    ");
    let __try1 = exc::catch(|| {
    answers = pyrt::xrefs::xmodule("modules.answers");
    said = mcall(&answers.clone(), "respond", &[clean.clone(), stdlib_call("copy", "deepcopy", &[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }]).unwrap_or_else(|e| exc::raise(e))]);
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
    return if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) { said.clone() } else { Py::None };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.safetyask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_IF_A_THING_IS_SAFE" => Some(g_ASK_IF_A_THING_IS_SAFE()),
            "A_DANGER_SAID" => Some(g_A_DANGER_SAID()),
            "A_FAULT_DESCRIBED" => Some(g_A_FAULT_DESCRIBED()),
            "A_RULING_ALREADY" => Some(g_A_RULING_ALREADY()),
            "A_SAFETY_ALREADY_RULED" => Some(g_A_SAFETY_ALREADY_RULED()),
            "A_SAFETY_TEST_LINES" => Some(g_A_SAFETY_TEST_LINES()),
            "A_SAFETY_WORD" => Some(g_A_SAFETY_WORD()),
            "A_THING_THAT_PLUGS_IN" => Some(g_A_THING_THAT_PLUGS_IN()),
            "_a_safety_word" => Some(py_func("_a_safety_word", |args: &[Py]| _a_safety_word(args))),
            "_a_shrug" => Some(py_func("_a_shrug", |args: &[Py]| _a_shrug(args))),
            "_the_answers_have_this" => Some(py_func("_the_answers_have_this", |args: &[Py]| _the_answers_have_this(args))),
            "_the_thing_is_described_faulty" => Some(py_func("_the_thing_is_described_faulty", |args: &[Py]| _the_thing_is_described_faulty(args))),
            "_the_thing_on_the_table" => Some(py_func("_the_thing_on_the_table", |args: &[Py]| _the_thing_on_the_table(args))),
            "last_bot" => Some(g_last_bot()),
            "last_user" => Some(g_last_user()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_bot_already_ruled" => Some(py_func("the_bot_already_ruled", |args: &[Py]| the_bot_already_ruled(args))),
            "copy" => Some(stdlib_module("copy")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK_IF_A_THING_IS_SAFE" => s_ASK_IF_A_THING_IS_SAFE(v),
            "A_DANGER_SAID" => s_A_DANGER_SAID(v),
            "A_FAULT_DESCRIBED" => s_A_FAULT_DESCRIBED(v),
            "A_RULING_ALREADY" => s_A_RULING_ALREADY(v),
            "A_SAFETY_ALREADY_RULED" => s_A_SAFETY_ALREADY_RULED(v),
            "A_SAFETY_TEST_LINES" => s_A_SAFETY_TEST_LINES(v),
            "A_SAFETY_WORD" => s_A_SAFETY_WORD(v),
            "A_THING_THAT_PLUGS_IN" => s_A_THING_THAT_PLUGS_IN(v),
            "last_bot" => s_last_bot(v),
            "last_user" => s_last_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.safetyask", module_obj);
    pyrt::xrefs::reg_fn("compose.safetyask", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "_a_safety_word", py_func("_a_safety_word", |args: &[Py]| _a_safety_word(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "_the_thing_is_described_faulty", py_func("_the_thing_is_described_faulty", |args: &[Py]| _the_thing_is_described_faulty(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "_a_shrug", py_func("_a_shrug", |args: &[Py]| _a_shrug(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "_the_thing_on_the_table", py_func("_the_thing_on_the_table", |args: &[Py]| _the_thing_on_the_table(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "the_bot_already_ruled", py_func("the_bot_already_ruled", |args: &[Py]| the_bot_already_ruled(args)));
    pyrt::xrefs::reg_fn("compose.safetyask", "_the_answers_have_this", py_func("_the_answers_have_this", |args: &[Py]| _the_answers_have_this(args)));
    pyrt::xrefs::reg_access("compose.safetyask", "ASK_IF_A_THING_IS_SAFE", g_ASK_IF_A_THING_IS_SAFE, s_ASK_IF_A_THING_IS_SAFE);
    pyrt::xrefs::reg_access("compose.safetyask", "A_DANGER_SAID", g_A_DANGER_SAID, s_A_DANGER_SAID);
    pyrt::xrefs::reg_access("compose.safetyask", "A_FAULT_DESCRIBED", g_A_FAULT_DESCRIBED, s_A_FAULT_DESCRIBED);
    pyrt::xrefs::reg_access("compose.safetyask", "A_RULING_ALREADY", g_A_RULING_ALREADY, s_A_RULING_ALREADY);
    pyrt::xrefs::reg_access("compose.safetyask", "A_SAFETY_ALREADY_RULED", g_A_SAFETY_ALREADY_RULED, s_A_SAFETY_ALREADY_RULED);
    pyrt::xrefs::reg_access("compose.safetyask", "A_SAFETY_TEST_LINES", g_A_SAFETY_TEST_LINES, s_A_SAFETY_TEST_LINES);
    pyrt::xrefs::reg_access("compose.safetyask", "A_SAFETY_WORD", g_A_SAFETY_WORD, s_A_SAFETY_WORD);
    pyrt::xrefs::reg_access("compose.safetyask", "A_THING_THAT_PLUGS_IN", g_A_THING_THAT_PLUGS_IN, s_A_THING_THAT_PLUGS_IN);
    pyrt::xrefs::reg_access("compose.safetyask", "last_bot", g_last_bot, s_last_bot);
    pyrt::xrefs::reg_access("compose.safetyask", "last_user", g_last_user, s_last_user);
}

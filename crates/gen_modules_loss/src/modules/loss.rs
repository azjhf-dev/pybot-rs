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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFERS_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_understand: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.loss: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFERS_TO() -> Py { ensure_init(); G_DEFERS_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFERS_TO(v: Py) { ensure_init(); G_DEFERS_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_understand() -> Py { ensure_init(); G_understand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_understand(v: Py) { ensure_init(); G_understand.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A death is never small talk, and never a topic a gift entry can win.\n\n\"My dad died in march\" used to get \"Got it.\", and \"some days i can't get out of bed since my mum\ndied\" got a present-buying answer, because a topic entry matched the word \"mum\". The reading and\nthe pools live in `bot/compose/loss.py`: plain sympathy for the user's own loss, what to say to\nthe person who had the loss, words for a condolence card, a normalising answer to a question\nabout grief, and company for a short follow-up (\"it was really hard\").\n\nThis module is the speaker for those lines, so it says what it answers in two ways:\n\n* `ANSWERS = (\"loss\", \"grief\")` — the topics, like every other module;\n* `ANSWERS_INSTEAD = (\"loss\", \"grief\")` — it answers *in place of* the topic entries. A topic\n  entry matches a word or a topic id of the line (\"mum\", \"cat\", \"card\"), so on a bereavement\n  line it answers a different question; the arbiter lets a declaring module answer instead, and\n  the safety band still comes first;\n* `DEFERS_TO = loss.acknowledges` — it stands back when another candidate's reply already\n  acknowledges the death and knows something more about it.\n\nIt proposes only a line it is sure of, so a false reading would take a turn — that is why the\nread demands a person or a pet behind the death word (bot/compose/loss.py).\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_understand(py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args)));
    s_PRIORITY(Py::Int(2000));
    s_ANSWERS(py_tuple(vec![py_str("loss"), py_str("grief")]));
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("loss"), py_str("grief")]));
    s_DEFERS_TO(py_func("defers_to", |args: &[Py]| defers_to(args)));
}
pub fn defers_to(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut want: Py = Py::None;
    let _ = py_str("True when the entry's reply should keep the turn.\n\n    Five things have to hold: the entry says the sad thing itself, the death in the line is a\n    pet's (\"my aunt died and her dog needs a home\" is not the pets entry's line), the line asks\n    what to do rather than how the person feels (\"my cat died and i keep crying\"), something is\n    asked at all (\"my dog died\" on its own is a person who needs to be met, not a request for the\n    cremation card), and the ask is one the entry answers (\"can we bury him at home\", not \"when\n    should i get another one\"). The last two are separate because a line can ask for a decision\n    the card has no answer to, and a hamster does not get a paw print for \"is it my fault\".\n    ");
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.loss", "acknowledges", &[reply.clone()]); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.loss", "a_pet_died", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.loss", "names_a_feeling", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.loss", "asks_for_a_decision", &[line.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.loss", "asks_about_the_remains", &[line.clone()]) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.loss", "asks_the_administration", &[line.clone()]))); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.loss", "names_a_feeling", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.loss", "is_about_the_paperwork", &[reply.clone(), line.clone()])))) {
    return Py::Bool(false);
    }
    want = pyrt::xrefs::xcall("compose.fit", "wants", &[pyrt::xrefs::xcall("understanding", "understand", &[line.clone()])]);
    return call_builtin("bool", &[{ let __b = want.clone(); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fit", "gives", &[reply.clone(), want.clone(), line.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut clean: Py = Py::None;
    let mut options: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply a death asks for, or None when this is an ordinary line.\n\n    A follow-up (\"it was really hard\", \"what do i even say to him\") is read the same way while\n    the bot's own last replies were about the loss; a change of subject ends it.\n    ");
    understood = _reading(&[text.clone(), memory.clone()]);
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("about_bot")])) {
    return Py::None;
    }
    clean = mcall(&understood.clone(), "get", &[py_str("clean")]);
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    words = Py::None;
    }
    options = _options(&[text.clone(), memory.clone(), clean.clone(), understood.clone(), words.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&options.clone()))) {
    return Py::None;
    }
    reply = pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), text.clone()]);
    let _ = _the_missing_said(&[memory.clone(), reply.clone()]);
    return reply.clone();
    Py::None
}
pub fn _the_missing_said(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut turns: Py = Py::None;
    let _ = py_str("Note that the missing was met, for the turn after it (bot/memory/ack.py).\n\n    Written where the reply is chosen, so it lands in this module's copy of the memory: the\n    arbiter keeps the copy of the module whose reply the person got, and a note made by a module\n    that lost is thrown away with its copy. Without it the line after \"i miss my mum\" (\"sorry i\n    meant my nan\") lost the reason a friend answers it warmly - the engine reads the missing as\n    news, so bot/memory/ack.py's `_after_a_feeling` was the only thing that could tell, and the\n    note bot/modules/chat.py writes for its own heavy pools never landed.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("compose.loss", "MISS_REPLIES"), &reply.clone())) } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    let _ = py_setitem(&memory.clone(), &py_str("heavy_reply"), py_dict(vec![(py_str("feeling"), py_str("missing")), (py_str("turn"), if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) })]));
    Py::None
}
pub fn _reading(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut stored: Py = Py::None;
    let _ = py_str("The understanding engine's reading of *this* line.\n\n    `memory[\"understood\"]` is written by the observer before the modules run, so it is this\n    line; a module called on its own (a test, another module) may be handed a memory that is\n    still on the line before, so the reading is taken again when it does not match.\n    ");
    stored = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[stored.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _same(&[mcall(&stored.clone(), "get", &[py_str("clean")]), text.clone()]) } }) {
    return stored.clone();
    }
    let __try1 = exc::catch(|| {
    read = pyrt::xrefs::xcall("understanding", "understand", &[text.clone(), memory.clone()]);
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
    return py_dict(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { read.clone() } else { py_dict(vec![]) };
    Py::None
}
pub fn _same(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stored: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stored") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stored"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("True when the stored reading is of this line (the words, not the punctuation).");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[stored.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&stored.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[pyrt::xrefs::xcall("compose.loss", "words_of", &[stored.clone()])]), &mcall(&py_str(" "), "join", &[pyrt::xrefs::xcall("compose.loss", "words_of", &[text.clone()])])));
    Py::None
}
pub fn _options(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut understood: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut words: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = pyrt::xrefs::xcall("compose.loss", "words_of", &[text.clone()]);
    }
    return py_tuple(vec![Py::Bool(true), pyrt::xrefs::xcall("compose.loss", "reply_for", &[text.clone(), memory.clone(), clean.clone(), mcall(&understood.clone(), "get", &[py_str("topics")]), _mood(&[words.clone()])])]);
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
    Py::None
}
pub fn _mood(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut cues: Py = Py::None;
    let _ = py_str("'negative' for a hard follow-up (\"it was awful\"), so it gets company, not a stock ack.");
    let __try1 = exc::catch(|| {
    cues = _cues(&[]);
    if py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &cues.clone())) {
    return py_tuple(vec![Py::Bool(true), py_str("negative")]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return Py::None;
    Py::None
}
pub fn _cues(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut NEGATIVE_CUES: Py = Py::None;
    NEGATIVE_CUES = pyrt::xrefs::xget("compose.fallback", "NEGATIVE_CUES");
    return NEGATIVE_CUES.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.loss".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "DEFERS_TO" => Some(g_DEFERS_TO()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_cues" => Some(py_func("_cues", |args: &[Py]| _cues(args))),
            "_mood" => Some(py_func("_mood", |args: &[Py]| _mood(args))),
            "_options" => Some(py_func("_options", |args: &[Py]| _options(args))),
            "_reading" => Some(py_func("_reading", |args: &[Py]| _reading(args))),
            "_same" => Some(py_func("_same", |args: &[Py]| _same(args))),
            "_the_missing_said" => Some(py_func("_the_missing_said", |args: &[Py]| _the_missing_said(args))),
            "defers_to" => Some(py_func("defers_to", |args: &[Py]| defers_to(args))),
            "pick" => Some(g_pick()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "understand" => Some(g_understand()),
            "fit" => Some(pyrt::xrefs::xmodule("compose.fit")),
            "loss" => Some(pyrt::xrefs::xmodule("compose.loss")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "DEFERS_TO" => s_DEFERS_TO(v),
            "PRIORITY" => s_PRIORITY(v),
            "pick" => s_pick(v),
            "understand" => s_understand(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.loss", module_obj);
    pyrt::xrefs::reg_fn("modules.loss", "defers_to", py_func("defers_to", |args: &[Py]| defers_to(args)));
    pyrt::xrefs::reg_fn("modules.loss", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_the_missing_said", py_func("_the_missing_said", |args: &[Py]| _the_missing_said(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_reading", py_func("_reading", |args: &[Py]| _reading(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_same", py_func("_same", |args: &[Py]| _same(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_options", py_func("_options", |args: &[Py]| _options(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_mood", py_func("_mood", |args: &[Py]| _mood(args)));
    pyrt::xrefs::reg_fn("modules.loss", "_cues", py_func("_cues", |args: &[Py]| _cues(args)));
    pyrt::xrefs::reg_access("modules.loss", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.loss", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.loss", "DEFERS_TO", g_DEFERS_TO, s_DEFERS_TO);
    pyrt::xrefs::reg_access("modules.loss", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.loss", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("modules.loss", "understand", g_understand, s_understand);
}

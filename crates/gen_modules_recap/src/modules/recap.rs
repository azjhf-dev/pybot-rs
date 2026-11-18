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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTED_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTED_AGAIN() -> Py { ensure_init(); G_WANTED_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTED_AGAIN(v: Py) { ensure_init(); G_WANTED_AGAIN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A recap asked for with the wrapper still on it is still the recap.\n\nMain reads the recap as the whole line: \"what were we talking about\", \"what were we saying\",\n\"where were we\". People say it with the reminder wrapper still on, with a word in front, or\nwith the place instead of the words:\n\n    remind me what we were talking about\n    remind me where we got to\n    back to my question\n    where were we up to\n    what were we looking at\n    what were we just saying\n\nNone of those is the line the store's gate reads, so the store hands the turn on and the\npunting modules answer: \"That's a job in the world and I can't reach one\", \"Searching for it\nwill turn up better sources\", \"That's beyond what I know well, sorry\". The person asked what\nthe two of you were on and got a shrug from a bot that had just been told.\n\nNothing new is said here. The gate is wider, so the store's own recap is reached on those\nlines and the answer is the store's (\"You said you have a washer and a spanner. Want to\ncarry on?\"). It proposes at the recall band because that is what the line is: a question about\nthe conversation, not about the thing its words name. A line that ends on the ask only - \"what\nwe were talking about was the roof\" - is a statement, so every shape here has to end the line.\n");
    s_PRIORITY(Py::Int(7000));
    s_WANTED_AGAIN(stdlib_call("re", "compile", &[py_str("\\bwhat (?:were|are|was) (?:we|i|you) (?:just )?(?:talking|chatting|saying|on) about\\b|\\bwhat (?:were|are|was) (?:we|i|you) (?:just )?saying\\b|\\bwhat we were (?:just )?(?:talking|chatting|saying) about(?=\\s*(?:[?.!,]|$|again\\b|earlier\\b|before\\b|just now\\b))|\\bwhat we were (?:just )?saying(?=\\s*(?:[?.!,]|$|again\\b|earlier\\b|before\\b))|\\bwhere (?:were we|are we)(?=\\s*(?:[?.!,]|$|up to\\b|now\\b|at\\b))|\\bwhere (?:i|we) (?:were|was|are|got to|left off|ended up)\\b(?=\\s*(?:[?.!,]|$|up to\\b|now\\b|again\\b))|\\bwhat (?:were|was) (?:we|i) (?:looking at|doing)\\b(?=\\s*(?:[?.!,]|$|again\\b|earlier\\b|before\\b))|\\bwhat we were (?:just )?(?:looking at|doing)\\b(?=\\s*(?:[?.!,]|$|again\\b|earlier\\b|before\\b))|\\bback to (?:my|your|the) question\\b(?=\\s*(?:[?.!,]|$))")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _recap_shape(args: &[Py]) -> Py {
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
    let _ = py_str("The line, folded, when it is a recap main's own gate does not read, else \"\".");
    let __try1 = exc::catch(|| {
    low = py_call(&pyrt::xrefs::xget("modules.memory_store", "_folded"), &[text.clone(), memory.clone()]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WANTED_AGAIN(), "search", &[low.clone()]))) } }) {
    return py_str("");
    }
    return low.clone();
    Py::None
}
pub fn ASKS_ABOUT_THE_CHAT(args: &[Py]) -> Py {
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
    let _ = py_str("This line asks about the conversation, whatever topic its words happen to name.");
    return call_builtin("bool", &[_recap_shape(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _not_a_recap(args: &[Py]) -> Py {
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
    let _ = py_str("True when a line is about the conversation rather than about the world it names.\n\n    The store's own reading, widened by the shapes this module adds: a recap ask said back\n    beside the thing we were on is the machine reading its own question out - \"back to my\n    question\" came back as \"You said remind you where you were\" - so the newest line that is\n    one of these is never the thing the chat is on, and never a thing the chat said.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = call_builtin("bool", &[py_call(&pyrt::xrefs::xget("modules.memory_store", "_about_this_chat"), &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[_recap_shape(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)) } }]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_recap(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut ask: Py = Py::None;
    let mut options: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut thread: Py = Py::None;
    let _ = py_str("The thing we were on and the ask, said back in the person's own words.");
    thread = pyrt::xrefs::xmodule("memory.thread");
    said = mcall(&thread.clone(), "last_said", &[memory.clone(), py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args))]);
    thing = mcall(&thread.clone(), "about", &[memory.clone(), py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args))]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&pyrt::xrefs::xget("modules.memory_store", "_a_thing"), &[thing.clone()]))) } }) {
    thing = Py::None;
    }
    ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args))); __kws.extend(vec![("not_before".to_string(), mcall(&thread.clone(), "topic_index", &[memory.clone(), py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thread.clone(), "last_ask", args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { ask.clone() } }) {
    options = py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("We were on "), &thing.clone()), &py_str(" — you asked ")), &ask.clone()), &py_str(". Want to carry on?")), op_add(&op_add(&op_add(&op_add(&py_str("Still on "), &thing.clone()), &py_str(": you asked ")), &ask.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("The last thing we got into was "), &thing.clone()), &py_str(", and you asked ")), &ask.clone()), &py_str("."))]);
    } else {
    if py_truthy(&{ let __b = ask.clone(); if !py_truthy(&__b) { __b } else { said.clone() } }) {
    options = py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("You said "), &said.clone()), &py_str(". You asked ")), &ask.clone()), &py_str(". Want to carry on?")), op_add(&op_add(&op_add(&op_add(&py_str("You said "), &said.clone()), &py_str(" - and you asked ")), &ask.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(". Your question was ")), &ask.clone()), &py_str("."))]);
    } else {
    if py_truthy(&ask.clone()) {
    options = py_list(vec![op_add(&op_add(&py_str("You asked "), &ask.clone()), &py_str(". Want to carry on?")), op_add(&op_add(&py_str("You were asking "), &ask.clone()), &py_str("."))]);
    } else {
    if py_truthy(&said.clone()) {
    options = py_list(vec![op_add(&op_add(&py_str("You said "), &said.clone()), &py_str(". Want to carry on?")), op_add(&op_add(&py_str("You last said "), &said.clone()), &py_str(".")), op_add(&op_add(&py_str("You'd told me "), &said.clone()), &py_str(".")), op_add(&op_add(&py_str("Last thing you said was "), &said.clone()), &py_str("."))]);
    } else {
    if py_truthy(&thing.clone()) {
    options = py_list(vec![op_add(&op_add(&py_str("We were on "), &thing.clone()), &py_str(".")), op_add(&op_add(&py_str("Still "), &thing.clone()), &py_str("."))]);
    } else {
    return py_str("");
    }
    }
    }
    }
    }
    return _fresh(&[options.clone(), memory.clone()]);
    Py::None
}
pub fn _fresh(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut option: Py = Py::None;
    let _ = py_str("An option that is not the line the bot has just said.\n\n    A recap is short and made of the same few words as the last one, so a second ask in a row\n    can land on the same sentence - \"where were we up to\" straight after \"what were we just\n    saying\" - and the arbiter passes over a reply that only repeats the bot's last turn. Varying\n    the words is the module's job, so the last line said is dropped here before the pick.\n    ");
    let __try1 = exc::catch(|| {
    last = mcall(&call_builtin("str", &[{ let __b = py_call(&pyrt::xrefs::xget("modules.memory_store", "last_bot"), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
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
    last = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&option.clone(), "strip", &[]), &last.clone()))) {  __out.push(option.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { options.clone() } };
    return py_call(&pyrt::xrefs::xget("modules.memory_store", "_vary"), &[fresh.clone(), memory.clone()]);
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
    let mut said: Py = Py::None;
    let _ = py_str("The recap itself, on the shapes the store's own gate does not read.");
    low = _recap_shape(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&py_call(&pyrt::xrefs::xget("modules.memory_store", "_about_the_bot_here"), &[memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    said = _the_recap(&[memory.clone(), low.clone()]);
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
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.recap".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_ABOUT_THE_CHAT" => Some(py_func("ASKS_ABOUT_THE_CHAT", |args: &[Py]| ASKS_ABOUT_THE_CHAT(args))),
            "PRIORITY" => Some(g_PRIORITY()),
            "WANTED_AGAIN" => Some(g_WANTED_AGAIN()),
            "_fresh" => Some(py_func("_fresh", |args: &[Py]| _fresh(args))),
            "_not_a_recap" => Some(py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args))),
            "_recap_shape" => Some(py_func("_recap_shape", |args: &[Py]| _recap_shape(args))),
            "_the_recap" => Some(py_func("_the_recap", |args: &[Py]| _the_recap(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "WANTED_AGAIN" => s_WANTED_AGAIN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.recap", module_obj);
    pyrt::xrefs::reg_fn("modules.recap", "_recap_shape", py_func("_recap_shape", |args: &[Py]| _recap_shape(args)));
    pyrt::xrefs::reg_fn("modules.recap", "ASKS_ABOUT_THE_CHAT", py_func("ASKS_ABOUT_THE_CHAT", |args: &[Py]| ASKS_ABOUT_THE_CHAT(args)));
    pyrt::xrefs::reg_fn("modules.recap", "_not_a_recap", py_func("_not_a_recap", |args: &[Py]| _not_a_recap(args)));
    pyrt::xrefs::reg_fn("modules.recap", "_the_recap", py_func("_the_recap", |args: &[Py]| _the_recap(args)));
    pyrt::xrefs::reg_fn("modules.recap", "_fresh", py_func("_fresh", |args: &[Py]| _fresh(args)));
    pyrt::xrefs::reg_fn("modules.recap", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.recap", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.recap", "WANTED_AGAIN", g_WANTED_AGAIN, s_WANTED_AGAIN);
}

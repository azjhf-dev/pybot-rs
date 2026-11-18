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
thread_local! { static G__A_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_shape_without_the_fact: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g__A_HEADS() -> Py { ensure_init(); G__A_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HEADS(v: Py) { ensure_init(); G__A_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_the_shape_without_the_fact() -> Py { ensure_init(); G_the_shape_without_the_fact.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_shape_without_the_fact(v: Py) { ensure_init(); G_the_shape_without_the_fact.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The last resort: the shape of the answer, when the bot has not got the fact itself.\n\nNothing in the bot knows some lines. \"what would you recommend instead\", \"how much does a\nreplacement one go for\", \"when should i start\" - no card claims them, no route to a place fits\nthem, and the reply is one of three search hedges: \"I'm not sure about that one, sorry - I'd rather\nnot guess. A quick search online will get you a solid answer.\" That is true of every line ever\ntyped and tells the person nothing about the one they asked. Pointing elsewhere is not a reply.\n\nThere is one thing the bot still knows about a line like that, and it is the shape of the answer: a\npick wants the thing that decides a pick, a figure wants the three things that set it, a verdict\nwants what deciding one would weigh. That is what a friend says while getting to the detail, and it\nis the difference between \"go and look it up\" and \"here is what the answer depends on\".\n\n`bot/modules/intent.py` has already written the shape to `memory[\"understood\"][\"wants\"]`, so this\nreads that and never a word list of its own (bot/compose/the_shape_without_the_fact.py holds the\npools and the readings it corrects). It sits one band above `greeting`, which is the module that\nsays the hedge, so it only ever stands where the reply would have been that hedge or a bare\nreceipt: a real answer from any module is above this one, and the composer's own line is asked for\nand read, so a hedge that already says something about this line keeps its turn.\n\nIt replaces the three plain hedges and the receipts only. The composer has its own lines for the\nshapes it knows - \"That's your call, not mine. Tell me what you're weighing and I'll help you pick\"\nfor a decision - and a general frame over the top of one of those is a step down from it.\n\nThree ways this was worse before the guards went in, each kept as a test:\n\n  - a date printed on a paper is read off the paper, not weighed: \"my car insurance is up for\n    renewal in march\" ... \"ok and when does it run out\" is on the policy;\n  - food that is still good is a fact, not a decision: \"the stuffing burnt\" ... \"is the rest still\n    ok\" wants the keeping row for a joint, not a test for a decision;\n  - a count of the thing just named in the thread is written on the page for that thing, not a sum.\n\nAnd it stands down on a care, loss or crisis thread and on a question about the body, where a\ngeneral frame about how to choose is the wrong thing to say to somebody worried about a person.\n");
    s_the_shape_without_the_fact(py_func("the_shape_without_the_fact", |args: &[Py]| pyrt::xrefs::xcall("compose.the_shape_without_the_fact", "the_shape_without_the_fact", args)));
    s_PRIORITY(op_neg(&Py::Float(999.5)));
    s__A_HEADS(Py::Int(40));
}
pub fn _the_reply_is_only_a_placeholder(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[reply.clone(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "NEUTRAL")])])) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback", "UNKNOWN")) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&mcall(&line.clone(), "lower", &[]), &Py::None, &g__A_HEADS(), &Py::None)); } py_list(__out) })() }, &slice_or_raise(&said.clone(), &Py::None, &g__A_HEADS(), &Py::None)));
    Py::None
}
pub fn _the_composer_would_hedge(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("True when the composer's own line for this turn is a placeholder, not an answer.");
    let __try1 = exc::catch(|| {
    reply = py_call(&pyrt::xrefs::xget("compose.fallback", "reply_for"), &[text.clone(), memory.clone()]);
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
    return _the_reply_is_only_a_placeholder(&[reply.clone()]);
    Py::None
}
pub fn _the_floor_would_say_a_placeholder(args: &[Py]) -> Py {
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
    let mut greeting: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when `greeting` - the module that says the hedge - would say a placeholder this turn.\n\n    The composer is only part of what the floor does: `greeting` also hands back a step of the\n    answer above, and a follow-up like \"what about the containers\" is that step, not a hedge. A\n    rule read off the composer alone took the turn from it - twice on a 200-chat sample - so the\n    module that would otherwise answer is asked, on a copy of memory, and its own line decides.\n    ");
    let __try1 = exc::catch(|| {
    greeting = pyrt::xrefs::xmodule("modules.greeting");
    reply = mcall(&greeting.clone(), "respond", &[text.clone(), _a_copy_of(&[memory.clone()])]);
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
    return _the_reply_is_only_a_placeholder(&[reply.clone()]);
    Py::None
}
pub fn _a_copy_of(args: &[Py]) -> Py {
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
    let mut copy: Py = Py::None;
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Memory as a module may read it: everything copied, the turns shared, as the runtime does.");
    let __try1 = exc::catch(|| {
    copy = stdlib_module("copy");
    return py_tuple(vec![Py::Bool(true), { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&memory.clone(), "items", &[])) { let __unpk773 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk773[0].clone(); let mut value: Py = __unpk773[1].clone(); __out.push((key.clone(), if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("turns")))) { value.clone() } else { mcall(&copy.clone(), "deepcopy", &[value.clone()]) })); } py_dict(__out) })() }]);
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
    return call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _a_thread_with_a_person_in_trouble(args: &[Py]) -> Py {
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
    let mut reader: Py = Py::None;
    let _ = py_str("A worry about somebody's health, a death, a crisis, or a line about the body.\n\n    A general frame about how to weigh a choice is the wrong thing to say to somebody whose mother\n    is in hospital, and the bot has readers of its own for those turns.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("_in_a_care_thread"), py_str("_in_a_loss_thread"), py_str("_in_a_crisis_thread"), py_str("_thread_is_about_the_body")]));
    for __item in __iter1 {
    reader = __item.clone();
    let __try2 = exc::catch(|| {
    if py_truthy(&py_call(&call_builtin("getattr", &[pyrt::xrefs::xmodule("compose.fallback"), reader.clone()]).unwrap_or_else(|e| exc::raise(e)), &[memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return Py::Bool(false);
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
    let mut pool: Py = Py::None;
    let mut understood: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    pool = pyrt::xrefs::xcall("compose.the_shape_without_the_fact", "the_shape_without_the_fact", &[understood.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&_a_thread_with_a_person_in_trouble(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_composer_would_hedge(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_floor_would_say_a_placeholder(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]) } };
    return py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.the_shape_of_the_answer".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "_A_HEADS" => Some(g__A_HEADS()),
            "_a_copy_of" => Some(py_func("_a_copy_of", |args: &[Py]| _a_copy_of(args))),
            "_a_thread_with_a_person_in_trouble" => Some(py_func("_a_thread_with_a_person_in_trouble", |args: &[Py]| _a_thread_with_a_person_in_trouble(args))),
            "_the_composer_would_hedge" => Some(py_func("_the_composer_would_hedge", |args: &[Py]| _the_composer_would_hedge(args))),
            "_the_floor_would_say_a_placeholder" => Some(py_func("_the_floor_would_say_a_placeholder", |args: &[Py]| _the_floor_would_say_a_placeholder(args))),
            "_the_reply_is_only_a_placeholder" => Some(py_func("_the_reply_is_only_a_placeholder", |args: &[Py]| _the_reply_is_only_a_placeholder(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "the_shape_without_the_fact" => Some(g_the_shape_without_the_fact()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "_A_HEADS" => s__A_HEADS(v),
            "the_shape_without_the_fact" => s_the_shape_without_the_fact(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.the_shape_of_the_answer", module_obj);
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "_the_reply_is_only_a_placeholder", py_func("_the_reply_is_only_a_placeholder", |args: &[Py]| _the_reply_is_only_a_placeholder(args)));
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "_the_composer_would_hedge", py_func("_the_composer_would_hedge", |args: &[Py]| _the_composer_would_hedge(args)));
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "_the_floor_would_say_a_placeholder", py_func("_the_floor_would_say_a_placeholder", |args: &[Py]| _the_floor_would_say_a_placeholder(args)));
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "_a_copy_of", py_func("_a_copy_of", |args: &[Py]| _a_copy_of(args)));
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "_a_thread_with_a_person_in_trouble", py_func("_a_thread_with_a_person_in_trouble", |args: &[Py]| _a_thread_with_a_person_in_trouble(args)));
    pyrt::xrefs::reg_fn("modules.the_shape_of_the_answer", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.the_shape_of_the_answer", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.the_shape_of_the_answer", "_A_HEADS", g__A_HEADS, s__A_HEADS);
    pyrt::xrefs::reg_access("modules.the_shape_of_the_answer", "the_shape_without_the_fact", g_the_shape_without_the_fact, s_the_shape_without_the_fact);
}

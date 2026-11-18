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


thread_local! { static G_NAME_TAKEN_BACK_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_TAKEN_BACK_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.name_taken_back: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_NAME_TAKEN_BACK_BARE() -> Py { ensure_init(); G_NAME_TAKEN_BACK_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_TAKEN_BACK_BARE(v: Py) { ensure_init(); G_NAME_TAKEN_BACK_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_TAKEN_BACK_SAY() -> Py { ensure_init(); G_NAME_TAKEN_BACK_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_TAKEN_BACK_SAY(v: Py) { ensure_init(); G_NAME_TAKEN_BACK_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_user() -> Py { ensure_init(); G_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_user(v: Py) { ensure_init(); G_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The name on record, taken back by the person who gave it.\n\n    my name is kay\n    im not kay                     ->  Sorry — I had you as Kay. What should I call you?\n    dave                           ->  Hi Dave — good to meet you.\n    whats my name                  ->  You're Dave.\n\nThree things went wrong on main `bc4425e`, and the store was the last of them. The line was\nanswered with nothing at all (\"I'm listening — say more if it helps\"), or with the person's own\nwords said back (\"Got it — that's not your name.\", \"Got it — your names not ok.\"), and the name\nstayed on record, so every turn after it called them by the name they had just said was wrong. A\nperson who says the bot has their name wrong is asking it to stop using it; the reply that does\nthat is an apology and one question, and the next bare word is the answer to it.\n\nThe reading is `bot/memory/facts.py:the_name_they_take_back`, because it is the same reader for\nthe store's own name (pets, and the names of people they mention) and it belongs beside them. It\nfires only against the name the store holds, so \"im not sure\" and \"im not hungry\" - the commonest\nopenings of any line - are never read as a denial of a name nobody gave.\n\nThis module exists rather than a branch inside `bot/modules/memory_store.py` because that file is\npast the world's size cap and cannot be uploaded (the same reason as\n`bot/modules/trip_date_said_again.py`). It keeps the store's own record up to date and asks for\nnothing else: `memory[\"asked_for\"]` is the store's marker for a name question the bot has just\nasked, which is what makes the bare \"dave\" on the next line read as a name rather than as noise.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_user(py_func("user", |args: &[Py]| pyrt::xrefs::xcall("memory", "user", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.name_taken_back.PRIORITY"); }
    s_PRIORITY(Py::Int(6600));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.name_taken_back.NAME_TAKEN_BACK_SAY"); }
    s_NAME_TAKEN_BACK_SAY(py_tuple(vec![py_str("Sorry — I had you as {name}. What should I call you?"), py_str("My mistake on {name}. What should I call you?"), py_str("Sorry about that — {name} was wrong. What do you go by?"), py_str("Ah, sorry. Not {name}, then — what should I call you?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.name_taken_back.NAME_TAKEN_BACK_BARE"); }
    s_NAME_TAKEN_BACK_BARE(py_tuple(vec![py_str("Sorry about that — what should I call you?"), py_str("My mistake. What should I call you?"), py_str("Sorry — what do you go by?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.name_taken_back.TAKES_AS_IT_COMES"); }
    s_TAKES_AS_IT_COMES(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
}
pub fn _the_name_i_had(args: &[Py]) -> Py {
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
    let _ = py_str("The name the store holds for the person, or \"\" when it holds none.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return mcall(&call_builtin("str", &[{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn _note(args: &[Py]) -> Py {
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
    let _ = py_str("The line as the store keeps it for comparing one turn with the next.");
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("Take the name off the record, and leave the note the reply is written from.\n\n    The name goes here rather than in `respond` because the store keeps what a line says whatever\n    its own reply is: left in place, the next turn calls the person by the name they have just\n    said is wrong, and that is the fault this module is for.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    known = _the_name_i_had(&[memory.clone()]);
    taken = pyrt::xrefs::xcall("memory.facts", "the_name_they_take_back", &[text.clone(), known.clone()]);
    if py_truthy(&Py::Bool(py_is(&taken.clone(), &Py::None))) {
    return Py::None;
    }
    let _ = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "pop", &[py_str("name"), Py::None]);
    let _ = py_setitem(&memory.clone(), &py_str("name_taken_back"), py_dict(vec![(py_str("name"), call_builtin("str", &[taken.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("said"), _note(&[text.clone()]))]));
    let _ = py_setitem(&memory.clone(), &py_str("asked_for"), py_str("name"));
    Py::None
}
pub fn _this_line(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut held: Py = Py::None;
    let _ = py_str("The note left by `observe` when it was made on this line, else None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    held = mcall(&memory.clone(), "get", &[py_str("name_taken_back")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&held.clone(), "get", &[py_str("said")]), &_note(&[text.clone()]))) } }) {
    return Py::None;
    }
    return held.clone();
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
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    held = _this_line(&[memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::None))) {
    return Py::None;
    }
    name = mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_NAME_TAKEN_BACK_BARE(), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NAME_TAKEN_BACK_SAY()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), name.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("True only on a line that denies the name on record: nothing else has this answer.");
    return Py::Bool(!py_is(&_this_line(&[memory.clone(), text.clone()]), &Py::None));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.name_taken_back".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "NAME_TAKEN_BACK_BARE" => Some(g_NAME_TAKEN_BACK_BARE()),
            "NAME_TAKEN_BACK_SAY" => Some(g_NAME_TAKEN_BACK_SAY()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "_note" => Some(py_func("_note", |args: &[Py]| _note(args))),
            "_the_name_i_had" => Some(py_func("_the_name_i_had", |args: &[Py]| _the_name_i_had(args))),
            "_this_line" => Some(py_func("_this_line", |args: &[Py]| _this_line(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "pick" => Some(g_pick()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "user" => Some(g_user()),
            "extract" => Some(pyrt::xrefs::xmodule("memory.facts")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "NAME_TAKEN_BACK_BARE" => s_NAME_TAKEN_BACK_BARE(v),
            "NAME_TAKEN_BACK_SAY" => s_NAME_TAKEN_BACK_SAY(v),
            "PRIORITY" => s_PRIORITY(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            "pick" => s_pick(v),
            "user" => s_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.name_taken_back", module_obj);
    pyrt::xrefs::reg_fn("modules.name_taken_back", "_the_name_i_had", py_func("_the_name_i_had", |args: &[Py]| _the_name_i_had(args)));
    pyrt::xrefs::reg_fn("modules.name_taken_back", "_note", py_func("_note", |args: &[Py]| _note(args)));
    pyrt::xrefs::reg_fn("modules.name_taken_back", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.name_taken_back", "_this_line", py_func("_this_line", |args: &[Py]| _this_line(args)));
    pyrt::xrefs::reg_fn("modules.name_taken_back", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.name_taken_back", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("modules.name_taken_back", "NAME_TAKEN_BACK_BARE", g_NAME_TAKEN_BACK_BARE, s_NAME_TAKEN_BACK_BARE);
    pyrt::xrefs::reg_access("modules.name_taken_back", "NAME_TAKEN_BACK_SAY", g_NAME_TAKEN_BACK_SAY, s_NAME_TAKEN_BACK_SAY);
    pyrt::xrefs::reg_access("modules.name_taken_back", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.name_taken_back", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.name_taken_back", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("modules.name_taken_back", "user", g_user, s_user);
}

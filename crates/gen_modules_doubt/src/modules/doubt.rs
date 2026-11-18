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
thread_local! { static G_SUBJECT_UNCLEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.doubt: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_UNCLEAR() -> Py { ensure_init(); G_SUBJECT_UNCLEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_UNCLEAR(v: Py) { ensure_init(); G_SUBJECT_UNCLEAR.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that takes issue with the bot's own answer is answered about that answer.\n\n\"that makes no sense\", \"that cant be right\", \"youre confusing me\" name nothing of their own: the\nthing they are about is the paragraph the bot has just printed. On main every one of them was read\nas a shrug or as bad news about the person (\"Fair enough.\", \"Thanks for telling me - I'm here if\nyou want to dig into any of that\"), which is the bot saying it heard nothing at the one moment the\nperson is telling it the answer failed.\n\nThe reading and the wording live in `bot/memory/doubt.py`, next to the store it reads; this module\nis here so the claim has a band of its own. It sits above the small talk (`chat.py`, 300), which is\nwhat answered these lines with a stock nod, and below the safety band: a line in a crisis is never\nthis module's, however it is phrased. Nothing is claimed from any other module's real answer -\n`respond` returns None on every line that is not a doubt about the bot's own last sentence.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.doubt.PRIORITY"); }
    s_PRIORITY(Py::Int(400));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.doubt.SUBJECT_UNCLEAR"); }
    s_SUBJECT_UNCLEAR(py_tuple(vec![py_str("I'm not sure which {word} you mean - a word or two about it and I'll answer the right one."), py_str("Which {word} do you mean? Tell me a bit more and I'll take it from there."), py_str("There's more than one thing '{word}' could mean. What's the one you're asking about?")]));
}
pub fn _the_question(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut at: Py = Py::None;
    let mut letter: Py = Py::None;
    let _ = py_str("One of the lines above, chosen by the line so the same chat does not repeat one.");
    at = op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }) { let mut letter: Py = __item.clone(); __out.push(call_builtin("ord", &[letter.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_SUBJECT_UNCLEAR()]).unwrap_or_else(|e| exc::raise(e)));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("word".to_string(), word.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SUBJECT_UNCLEAR(), &at.clone()), "format", args))(&__args) };
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
    let mut understood: Py = Py::None;
    let _ = py_str("The reply to a doubt about the bot's own last answer, or None for every other line.");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("subject_unclear")]) } }) {
    return _the_question(&[text.clone(), geti(&understood.clone(), &py_str("subject_unclear"))]);
    }
    return pyrt::xrefs::xcall("memory.doubt", "reply_for", &[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.doubt".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "SUBJECT_UNCLEAR" => Some(g_SUBJECT_UNCLEAR()),
            "_the_question" => Some(py_func("_the_question", |args: &[Py]| _the_question(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "doubt" => Some(pyrt::xrefs::xmodule("memory.doubt")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "SUBJECT_UNCLEAR" => s_SUBJECT_UNCLEAR(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.doubt", module_obj);
    pyrt::xrefs::reg_fn("modules.doubt", "_the_question", py_func("_the_question", |args: &[Py]| _the_question(args)));
    pyrt::xrefs::reg_fn("modules.doubt", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.doubt", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.doubt", "SUBJECT_UNCLEAR", g_SUBJECT_UNCLEAR, s_SUBJECT_UNCLEAR);
}

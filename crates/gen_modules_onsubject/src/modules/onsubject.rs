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
fn module_body() {
    let _ = py_str("A short follow-up names the thing the chat is on, not a search.\n\n\"can he do that\" after a rent rise, \"do i need an adapter there\" after a holiday in Spain, \"will\nthey cut me off\" after an electric bill: the line names nothing of its own, so no entry is reached\nand the seed fallback answers it with the search hedge, as if the bot had not been listening. The\nperson said what they are talking about one line before, so a friend says it back and says who\nsettles it. That reply is built in `bot/compose/onsubject.py`.\n\nWhy this is a module of its own rather than one more reader inside the seed fallback: a reader\nadded to `bot/modules/greeting.py` has to be inserted at the end of a chain that other lanes add to\nevery few minutes, so it cannot be landed without a rebase, and a rebase that re-cuts somebody\nelse's reader is how a fallback loses a turn it had. As a module it is one new file, and the only\nrule it needs is the one the chain already has: a fallback answers when nothing else does.\n\nIt sits at -998, above the seed fallback (-1000) and below every fallback-band module that answers\na line by name (`bulky`, -999, is the nearest: one thing being thrown out). It asks the composer\nfor the reply the seed would give (`fallback.reply_for`, the same call `greeting.respond` makes at\nits top) and answers only where that reply is the plain \"I don't know\" pool - the search hedge,\nthe shrug, the echo. A hedge that already names who to ask keeps the turn, and so does any real\nanswer.\n\nThe readers the seed runs after that reply is built keep their lines too, because the reading here\nis narrower than any of them: a line with no question of its own that names the thing the chat has\njust been on (`onsubject.pool`). An answer still on the person's screen, a judgement put to the\nbot about their own people, a thing they want to find, and a thing of theirs damaged or refused\nare all questions or statements about something else, and this passes them by. What is left is the\nline that reaches the search hedge with nothing said back, which is this module's turn.\n");
    s_PRIORITY(op_neg(&Py::Int(998)));
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
    let mut reading: Py = Py::None;
    let mut reply: Py = Py::None;
    reading = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.onsubject", "pool", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&reading.clone()))) {
    reading = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("memory".to_string(), memory.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.onsubject", "place_pool", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&reading.clone()))) {
    return Py::None;
    }
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[reply.clone(), py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "UNKNOWN")])]))) } }) {
    return Py::None;
    }
    return reading.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.onsubject".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.onsubject", module_obj);
    pyrt::xrefs::reg_fn("modules.onsubject", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.onsubject", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

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


thread_local! { static G_A_LEAD_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_LEAD_ALREADY() -> Py { ensure_init(); G_A_LEAD_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAD_ALREADY(v: Py) { ensure_init(); G_A_LEAD_ALREADY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A paragraph the chat has had is not printed again in the same words.\n\nEvery reader in the answers module frames a repeat it can see - the row handed back a turn ago,\nthe sentence of it this line names - and each of them compares the answer it would give with the\nanswer *just above*. What none of them sees is a reply that comes back a few turns later, after a\nturn about something else, because the line reached the same row again:\n\n    the slugs are eating my hostas\n      Go out after dark with a torch and pick them off, which beats most products; copper tape ...\n    how do i get rid of them\n      Start with this: Go out after dark with a torch and pick them off.\n    changing subject, the lawn needs cutting\n      Mow little and often in the growing season ...\n    when should i mow it\n      Mow little and often in the growing season ...          (the turn above, word for word)\n    back to the slugs, when should i put the trap out\n      Go out after dark with a torch and pick them off, which beats most products; copper tape ...\n\nA person reading that hears a bot that has stopped listening, and it is the thing a person notices\nfirst in a long chat: the words are not wrong, they are the ones they have already read.\n\nSo the framing is done once, for whatever route the reply came by, rather than in each of the\nreaders above: `bot/memory/again.instead` holds the lead-ins and every guard that says when a\nrepeat is not one (a figure asked back, a part of the answer the line adds, a lead-in already in\nfront), and it passes a reply the chat has not had, so this returns exactly what it was given.\n\nWhy here and not in the answers module: the answers module is one of four parts, and the same\nparagraph can come back through another one (a saving, the store's own reply). The reading is a\nproperty of the chat, not of the part that chose the words, so it lives with the memory.\n");
    s_A_LEAD_ALREADY(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("memory.again", "FRAMES")) { let mut frame: Py = __item.clone(); if py_truthy(&mcall(&geti(&mcall(&frame.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[])) {  __out.push(mcall(&mcall(&geti(&mcall(&frame.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]), "lower", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_paragraph_the_chat_has_had(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut framed: Py = Py::None;
    let _ = py_str("`again.instead(reply)`, or the reply itself when it is new to the chat.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&mcall(&mcall(&reply.clone(), "lower", &[]), "startswith", &[g_A_LEAD_ALREADY()])) {
    return py_tuple(vec![Py::Bool(true), { let __b = the_same_paragraph_again(&[reply.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { reply.clone() } }]);
    }
    framed = { let __b = pyrt::xrefs::xcall("memory.again", "instead", &[reply.clone(), text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.again", "already_had", &[reply.clone(), memory.clone()]) } };
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
    return reply.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&framed.clone()))) {
    return reply.clone();
    }
    return { let __b = the_same_paragraph_again(&[framed.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { framed.clone() } };
    Py::None
}
pub fn the_same_paragraph_again(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut short: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A reader above framed it, and the chat has had that framed reply word for word.\n\n    The reader's own guard compares the whole reply with the turn above it, so the second time\n    the same card reaches the same reader the paragraph goes back out under the same lead - a\n    reply that repeats an earlier reply, which is the thing the first frame was there to avoid:\n\n        where do i get the part   -> Same answer, and it covers this: A dripping tap is nearly ...\n        do i need to turn the water off -> Same answer, and it covers this: A dripping tap is ...\n\n    So the short of the paragraph goes back behind the next lead in `again.SHORT_AGAIN` - the\n    leads other modules recognise as framed (\"Same as before:\", \"As I said:\"), which keeps a\n    second lead off it - and once those run out the person is asked what went wrong rather than\n    being handed the paragraph a fourth time. A reply with no lead-in is left alone here: the\n    reader above chose it, and `already_had` had its turn.\n    ");
    body = pyrt::xrefs::xcall("memory.again", "unframed", &[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return Py::None;
    }
    mine = pyrt::xrefs::xcall("memory.again", "_flat", &[reply.clone()]);
    said = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&pyrt::xrefs::xcall("memory.again", "_flat", &[mcall(&turn.clone(), "get", &[py_str("bot")])]), &mine.clone())) } }) {
    let __aug2 = op_add(&said.clone(), &Py::Int(1));
    said = __aug2;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    short = pyrt::xrefs::xcall("memory.again", "_short", &[body.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&short.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&pyrt::xrefs::xcall("memory.again", "_flat", &[short.clone()]), &pyrt::xrefs::xcall("memory.again", "_flat", &[body.clone()]))) } }) {
    short = body.clone();
    }
    if py_truthy(&op_ge(&said.clone(), &call_builtin("len", &[pyrt::xrefs::xget("memory.again", "SHORT_AGAIN")]).unwrap_or_else(|e| exc::raise(e)))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), short.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xget("memory.again", "AGAIN_HELP"), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), short.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&pyrt::xrefs::xget("memory.again", "SHORT_AGAIN"), &op_sub(&said.clone(), &Py::Int(1))), "format", args))(&__args) };
    Py::None
}
pub fn then_framed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut respond: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "respond") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: respond"))) };
    let mut a_reply_framed_as_the_repeat_it_is: Py = Py::None;
    let _ = py_str("Wrap a `respond(text, memory)` in the floor, so the module that owns the words is untouched.\n\n    The answers module already rebinds `respond` once at the end of the file (`again.replied`), so\n    the floor is applied the same way rather than written into the reader it sits under: the diff\n    to a module this busy stays at the file's tail, and the floor is read on its own here.\n    ");
pub fn __nested_a_reply_framed_as_the_repeat_it_is_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut a_reply_framed_as_the_repeat_it_is: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut respond: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "text").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return a_paragraph_the_chat_has_had(&[py_call(&respond, &[text.clone(), memory.clone()]), text.clone(), memory.clone()]);
    Py::None
}
    a_reply_framed_as_the_repeat_it_is = {{ let a_reply_framed_as_the_repeat_it_is = a_reply_framed_as_the_repeat_it_is.clone(); let respond = respond.clone(); py_func("a_reply_framed_as_the_repeat_it_is", move |args: &[Py]| { let mut full = vec![a_reply_framed_as_the_repeat_it_is.clone(), respond.clone()]; full.extend_from_slice(args); __nested_a_reply_framed_as_the_repeat_it_is_0(&full) }) }};
    return a_reply_framed_as_the_repeat_it_is.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.paragraph".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_LEAD_ALREADY" => Some(g_A_LEAD_ALREADY()),
            "a_paragraph_the_chat_has_had" => Some(py_func("a_paragraph_the_chat_has_had", |args: &[Py]| a_paragraph_the_chat_has_had(args))),
            "the_same_paragraph_again" => Some(py_func("the_same_paragraph_again", |args: &[Py]| the_same_paragraph_again(args))),
            "then_framed" => Some(py_func("then_framed", |args: &[Py]| then_framed(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_LEAD_ALREADY" => s_A_LEAD_ALREADY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.paragraph", module_obj);
    pyrt::xrefs::reg_fn("memory.paragraph", "a_paragraph_the_chat_has_had", py_func("a_paragraph_the_chat_has_had", |args: &[Py]| a_paragraph_the_chat_has_had(args)));
    pyrt::xrefs::reg_fn("memory.paragraph", "the_same_paragraph_again", py_func("the_same_paragraph_again", |args: &[Py]| the_same_paragraph_again(args)));
    pyrt::xrefs::reg_fn("memory.paragraph", "then_framed", py_func("then_framed", |args: &[Py]| then_framed(args)));
    pyrt::xrefs::reg_access("memory.paragraph", "A_LEAD_ALREADY", g_A_LEAD_ALREADY, s_A_LEAD_ALREADY);
}

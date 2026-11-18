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


thread_local! { static G_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLEASED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELIEF_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESULT_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_GOOD() -> Py { ensure_init(); G_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD(v: Py) { ensure_init(); G_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS() -> Py { ensure_init(); G_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS(v: Py) { ensure_init(); G_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_LINE() -> Py { ensure_init(); G_NEWS_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_LINE(v: Py) { ensure_init(); G_NEWS_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_GOOD() -> Py { ensure_init(); G_NOT_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_GOOD(v: Py) { ensure_init(); G_NOT_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTSIDE() -> Py { ensure_init(); G_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTSIDE(v: Py) { ensure_init(); G_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_IN_IT() -> Py { ensure_init(); G_OUT_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_IN_IT(v: Py) { ensure_init(); G_OUT_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLEASED() -> Py { ensure_init(); G_PLEASED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLEASED(v: Py) { ensure_init(); G_PLEASED.with(|c| *c.borrow_mut() = v); }
pub fn g_RELIEF() -> Py { ensure_init(); G_RELIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELIEF(v: Py) { ensure_init(); G_RELIEF.with(|c| *c.borrow_mut() = v); }
pub fn g_RELIEF_LINE() -> Py { ensure_init(); G_RELIEF_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELIEF_LINE(v: Py) { ensure_init(); G_RELIEF_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_RESULT() -> Py { ensure_init(); G_RESULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESULT(v: Py) { ensure_init(); G_RESULT.with(|c| *c.borrow_mut() = v); }
pub fn g_RESULT_LINE() -> Py { ensure_init(); G_RESULT_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESULT_LINE(v: Py) { ensure_init(); G_RESULT_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_THING() -> Py { ensure_init(); G_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING(v: Py) { ensure_init(); G_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_A() -> Py { ensure_init(); G_WHAT_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_A(v: Py) { ensure_init(); G_WHAT_A.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A pleased remark is answered as pleasure, not as something to look up.\n\n\"What a lovely day\", \"what a beautiful morning\", \"what a brilliant day\" and \"what a gorgeous day\"\nall reach the composer's search hedge on main - \"A quick search online will get you a solid\nanswer\" - and so do the short ones that carry the same feeling with no thing in them: \"what a\nrelief\", \"what a result\". Nothing about the weather is being asked and nothing needs looking up;\nthe person has said that something is good, and the reply a friend gives says so back in a few\nwords and gives them somewhere to go with it.\n\nThe bad half of the same shape already reads well (\"what a nightmare\" gets \"Ouch is right. Go\non.\", \"what a shame\" gets \"What happened, then?\"), which is the whole reading this file needs: the\nfeeling is in the words \"what a\" and in whether the word after them is one of pleasure. The pool\nthat follows is the one thing the bot can honestly do with a good moment it cannot see - say it\nsounds good and ask one light question about it - and the outside words (a day, a morning, a\nwindow) get the pool that talks about being out in it, because that is what people say next.\n\nTwo things are deliberately left alone. A thing that is not good however the adjective reads\n(\"what a fine mess\", \"a great shame\") is a complaint, and it stands down here. And a remark that\nnames nothing of its own and has no adjective (\"what a day\") is already read as a rough one by\n`bot/modules/chat.py`; if this file took it, a hard day would be congratulated.\n\nThis is a last resort and it is written to be one: `bot/modules/greeting.py` asks it where the\nother readers of a hedge are asked, and only a hedge is replaced.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_GOOD(py_str("lovely|beautiful|gorgeous|brilliant|great|wonderful|fantastic|perfect|cracking|smashing|nice|amazing|glorious|grand|super|fab|stunning|lush|decent"));
    s_THING(py_str("[a-z][a-z' -]{0,24}"));
    fn __data1() -> Py { py_tuple(vec![py_str("day"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("weather"), py_str("sun"), py_str("sunshine"), py_str("sky"), py_str("view"), py_str("sunset"), py_str("sunrise"), py_str("night"), py_str("light"), py_str("breeze")]) }
    s_OUTSIDE(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_GOOD(call_builtin("frozenset", &[mcall(&py_str("\nmess shame pity nightmare waste disaster worry state pain fuss palaver joke\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_A(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^(?:oh|ah|wow|ooh)[,\\s]+what a\\s+(?:"), &g_GOOD()), &py_str(")\\s+(?P<thing>")), &g_THING()), &py_str(")$|^(?:what|such) a\\s+(?:")), &g_GOOD()), &py_str(")\\s+(?P<thing2>")), &g_THING()), &py_str(")$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_RELIEF_LINE(stdlib_call("re", "compile", &[py_str("^(?:what|such) a (?:relief|weight off|load off)$|^that'?s? a relief$")]).unwrap_or_else(|e| exc::raise(e)));
    s_RESULT_LINE(stdlib_call("re", "compile", &[py_str("^(?:what|such) a (?:result|win|good result|great result)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS_LINE(stdlib_call("re", "compile", &[py_str("^(?:lovely|great|wonderful|brilliant|smashing|nice) news$")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEASED(py_tuple(vec![py_str("Good to hear. What are you up to today?"), py_str("Nice one. Make the most of it - what's the plan?"), py_str("That's good. Tell me about it.")]));
    s_OUT_IN_IT(py_tuple(vec![py_str("Good to hear. Are you getting out in it?"), py_str("Lovely. What are you up to today?"), py_str("That's the kind of day to be outside. What's the plan?")]));
    s_RELIEF(py_tuple(vec![py_str("That's a weight off. What was it about?"), py_str("Good - that's the end of it, then. What changed?"), py_str("Phew. What was it in the end?")]));
    s_NEWS(py_tuple(vec![py_str("Good news. What is it?"), py_str("Nice one. Tell me."), py_str("Lovely. What's happened?")]));
    s_RESULT(py_tuple(vec![py_str("Good result. What was it for?"), py_str("Nice one. Tell me how it went."), py_str("That's a good one. What happened?")]));
}
pub fn _thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    return mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
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
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]), "rstrip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&line.clone(), &py_str("?")))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_RELIEF_LINE(), "match", &[line.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_RELIEF()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    }
    if py_truthy(&mcall(&g_RESULT_LINE(), "match", &[line.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_RESULT()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    }
    if py_truthy(&mcall(&g_NEWS_LINE(), "match", &[line.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_NEWS()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    }
    match_ = mcall(&g_WHAT_A(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = _thing(&[match_.clone()]);
    words = call_builtin("set", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_NOT_GOOD()) } }) {
    return Py::None;
    }
    pool = if py_truthy(&op_bitand(&words.clone(), &g_OUTSIDE())) { g_OUT_IN_IT() } else { g_PLEASED() };
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, line.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.pleased".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "GOOD" => Some(g_GOOD()),
            "NEWS" => Some(g_NEWS()),
            "NEWS_LINE" => Some(g_NEWS_LINE()),
            "NOT_GOOD" => Some(g_NOT_GOOD()),
            "OUTSIDE" => Some(g_OUTSIDE()),
            "OUT_IN_IT" => Some(g_OUT_IN_IT()),
            "PLEASED" => Some(g_PLEASED()),
            "RELIEF" => Some(g_RELIEF()),
            "RELIEF_LINE" => Some(g_RELIEF_LINE()),
            "RESULT" => Some(g_RESULT()),
            "RESULT_LINE" => Some(g_RESULT_LINE()),
            "THING" => Some(g_THING()),
            "WHAT_A" => Some(g_WHAT_A()),
            "_thing" => Some(py_func("_thing", |args: &[Py]| _thing(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "GOOD" => s_GOOD(v),
            "NEWS" => s_NEWS(v),
            "NEWS_LINE" => s_NEWS_LINE(v),
            "NOT_GOOD" => s_NOT_GOOD(v),
            "OUTSIDE" => s_OUTSIDE(v),
            "OUT_IN_IT" => s_OUT_IN_IT(v),
            "PLEASED" => s_PLEASED(v),
            "RELIEF" => s_RELIEF(v),
            "RELIEF_LINE" => s_RELIEF_LINE(v),
            "RESULT" => s_RESULT(v),
            "RESULT_LINE" => s_RESULT_LINE(v),
            "THING" => s_THING(v),
            "WHAT_A" => s_WHAT_A(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.pleased", module_obj);
    pyrt::xrefs::reg_fn("compose.pleased", "_thing", py_func("_thing", |args: &[Py]| _thing(args)));
    pyrt::xrefs::reg_fn("compose.pleased", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.pleased", "GOOD", g_GOOD, s_GOOD);
    pyrt::xrefs::reg_access("compose.pleased", "NEWS", g_NEWS, s_NEWS);
    pyrt::xrefs::reg_access("compose.pleased", "NEWS_LINE", g_NEWS_LINE, s_NEWS_LINE);
    pyrt::xrefs::reg_access("compose.pleased", "NOT_GOOD", g_NOT_GOOD, s_NOT_GOOD);
    pyrt::xrefs::reg_access("compose.pleased", "OUTSIDE", g_OUTSIDE, s_OUTSIDE);
    pyrt::xrefs::reg_access("compose.pleased", "OUT_IN_IT", g_OUT_IN_IT, s_OUT_IN_IT);
    pyrt::xrefs::reg_access("compose.pleased", "PLEASED", g_PLEASED, s_PLEASED);
    pyrt::xrefs::reg_access("compose.pleased", "RELIEF", g_RELIEF, s_RELIEF);
    pyrt::xrefs::reg_access("compose.pleased", "RELIEF_LINE", g_RELIEF_LINE, s_RELIEF_LINE);
    pyrt::xrefs::reg_access("compose.pleased", "RESULT", g_RESULT, s_RESULT);
    pyrt::xrefs::reg_access("compose.pleased", "RESULT_LINE", g_RESULT_LINE, s_RESULT_LINE);
    pyrt::xrefs::reg_access("compose.pleased", "THING", g_THING, s_THING);
    pyrt::xrefs::reg_access("compose.pleased", "WHAT_A", g_WHAT_A, s_WHAT_A);
    pyrt::xrefs::reg_access("compose.pleased", "pick", g_pick, s_pick);
}

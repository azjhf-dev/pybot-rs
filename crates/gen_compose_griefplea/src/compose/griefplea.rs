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


thread_local! { static G_A_PLEA_WITH_NOTHING_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_SO_FAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.griefplea: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_PLEA_WITH_NOTHING_IN_IT() -> Py { ensure_init(); G_A_PLEA_WITH_NOTHING_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLEA_WITH_NOTHING_IN_IT(v: Py) { ensure_init(); G_A_PLEA_WITH_NOTHING_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_OF_ITS_OWN() -> Py { ensure_init(); G_NOTHING_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_OF_ITS_OWN(v: Py) { ensure_init(); G_NOTHING_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_SO_FAR() -> Py { ensure_init(); G_TAIL_SO_FAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_SO_FAR(v: Py) { ensure_init(); G_TAIL_SO_FAR.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The plea for a hand, said into a chat that is already a bereavement.\n\n\"i dont know what to do\" a turn after \"my mum died last month\" is not a request for a search\nresult. It is the person asking how to keep going, and the composer had two replies for it that\nare the worst lines in the file: the search hedge (\"That's beyond what I know well, sorry.\nSearching for it will turn up better sources than I can offer\") and the nod (\"I'm listening - say\nmore if it helps\"). Neither is true of a bereavement, and both send somebody who has just said\nthey are lost to a search engine.\n\nThe line names no death and no subject of its own, so nothing else can read it as part of this\nchat: the death is in the turns above. `loss.continues` is what says so, and the words below are\nthe ones that ask for nothing in particular.\n\nWhat is replaced is only a reply that left the answer where it was - the nod or one of the two\nsearch hedges (`fallback.NEUTRAL`, `fallback.UNKNOWN`). A reading with something of its own to say\nto the line keeps the turn: \"i dont know what to do\" after \"our dog was put down yesterday\" is\nalready answered by the pool that starts with \"Keep it small. One phone call, one meal\", and that\nis better than anything here, so this file stands down the moment the composer has an answer.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.griefplea.TAIL_SO_FAR"); }
    s_TAIL_SO_FAR(py_str("(?:\\s+(?:now|next|then|though|really|at\\s+all|anymore|today|please))*\\s*\\?*\\s*$"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.griefplea.A_PLEA_WITH_NOTHING_IN_IT"); }
    s_A_PLEA_WITH_NOTHING_IN_IT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^(?:(?:ok|okay|so|right|and|but|well|please|now|honestly|yeah)\\s+)*(?:(?:any\\s+)?(?:ideas?|thoughts?|advice|suggestions?|tips?|help)\\s*(?:please|then|for me)?\\s*\\?*\\s*$|what\\s+(?:do|would|should)\\s+(?:you|i)\\s+(?:think|do|say|suggest)"), &g_TAIL_SO_FAR()), &py_str("|(?:i\\s+)?(?:really\\s+|just\\s+)?(?:dont|don'?t|do not)\\s+know\\s+what\\s+to\\s+do")), &g_TAIL_SO_FAR()), &py_str("|(?:im|i'?m|i\\s+feel|i\\s+am)\\s+(?:so\\s+|totally\\s+|a\\s+bit\\s+|completely\\s+)?lost")), &g_TAIL_SO_FAR()), &py_str("|(?:where\\s+do\\s+i\\s+(?:even\\s+)?start|(?:i\\s+)?(?:dont|don'?t|do not)\\s+know\\s+where\\s+to\\s+start)")), &g_TAIL_SO_FAR()), &py_str("|(?:not\\s+sure|no\\s+idea)\\s+what\\s+to\\s+do")), &g_TAIL_SO_FAR()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.griefplea.NOTHING_OF_ITS_OWN"); }
    s_NOTHING_OF_ITS_OWN(py_tuple(vec![pyrt::xrefs::xget("compose.fallback", "NEUTRAL"), pyrt::xrefs::xget("compose.fallback", "UNKNOWN"), pyrt::xrefs::xget("compose.fallback", "TIPS_LINES")]));
}
pub fn _said(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut already: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A keeping-going line for a plea inside a bereavement, or None.");
    said = _said(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PLEA_WITH_NOTHING_IN_IT(), "match", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_in_a_pool", &[reply.clone(), g_NOTHING_OF_ITS_OWN()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("compose.loss", "continues", &[memory.clone()])))) {
    return Py::None;
    }
    pool = pyrt::xrefs::xget("compose.loss", "GETTING_THROUGH_REPLIES");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    already = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    fresh = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&already.clone(), &line.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    return py_call(&pyrt::xrefs::xget("compose.fallback", "pick"), &[fresh.clone(), { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.griefplea".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_PLEA_WITH_NOTHING_IN_IT" => Some(g_A_PLEA_WITH_NOTHING_IN_IT()),
            "NOTHING_OF_ITS_OWN" => Some(g_NOTHING_OF_ITS_OWN()),
            "TAIL_SO_FAR" => Some(g_TAIL_SO_FAR()),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "loss" => Some(pyrt::xrefs::xmodule("compose.loss")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_PLEA_WITH_NOTHING_IN_IT" => s_A_PLEA_WITH_NOTHING_IN_IT(v),
            "NOTHING_OF_ITS_OWN" => s_NOTHING_OF_ITS_OWN(v),
            "TAIL_SO_FAR" => s_TAIL_SO_FAR(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.griefplea", module_obj);
    pyrt::xrefs::reg_fn("compose.griefplea", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.griefplea", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.griefplea", "A_PLEA_WITH_NOTHING_IN_IT", g_A_PLEA_WITH_NOTHING_IN_IT, s_A_PLEA_WITH_NOTHING_IN_IT);
    pyrt::xrefs::reg_access("compose.griefplea", "NOTHING_OF_ITS_OWN", g_NOTHING_OF_ITS_OWN, s_NOTHING_OF_ITS_OWN);
    pyrt::xrefs::reg_access("compose.griefplea", "TAIL_SO_FAR", g_TAIL_SO_FAR, s_TAIL_SO_FAR);
}

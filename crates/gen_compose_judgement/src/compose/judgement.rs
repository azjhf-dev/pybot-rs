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


thread_local! { static G_JUDGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_ASK_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_BARE_FACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__matchable: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__the_plan_on_the_table: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_JUDGE_ASK() -> Py { ensure_init(); G_JUDGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ASK(v: Py) { ensure_init(); G_JUDGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_ASK_BARE() -> Py { ensure_init(); G_JUDGE_ASK_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_ASK_BARE(v: Py) { ensure_init(); G_JUDGE_ASK_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_BARE_FACES() -> Py { ensure_init(); G_JUDGE_BARE_FACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_BARE_FACES(v: Py) { ensure_init(); G_JUDGE_BARE_FACES.with(|c| *c.borrow_mut() = v); }
pub fn g__matchable() -> Py { ensure_init(); G__matchable.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__matchable(v: Py) { ensure_init(); G__matchable.with(|c| *c.borrow_mut() = v); }
pub fn g__the_plan_on_the_table() -> Py { ensure_init(); G__the_plan_on_the_table.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__the_plan_on_the_table(v: Py) { ensure_init(); G__the_plan_on_the_table.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Is this line asking the bot to rule on the plan already on the table?\n\nThe pools for \"is that a good idea\" live in `bot/compose/fallback.py` (`_a_verdict_on_their_plan`\nbuilds the reply from them), and the facts behind them in `bot/compose/fallback_data.py`. This\nmodule is the same reading without the reply: the ask on its own, so a caller that is not writing a\nverdict can tell that one is being asked for.\n\nThe arbiter wants exactly that. `_a_verdict_on_their_plan` also decides whether a verdict is the\nright thing to say at all - \"is that bad\" after somebody's news is not one, and an answer that has\nalready ruled leaves nothing to add - and the arbiter needs the narrower question: has the person\nput a plan down and asked for it to be judged? A hand-back (\"I cannot call that one for you\") is\nnot an answer to that, so a card that names the plan takes the turn off it.\n\nIt lives here rather than in `fallback.py` because that file is past the size the world will\nupload, so nothing there can be changed (see #general, `fallback.py` is over 1 MB).\n");
    s_JUDGE_BARE_FACES(pyrt::xrefs::xget("compose.fallback_data", "JUDGE_BARE_FACES"));
    s_JUDGE_ASK(pyrt::xrefs::xget("compose.fallback", "JUDGE_ASK"));
    s_JUDGE_ASK_BARE(pyrt::xrefs::xget("compose.fallback", "JUDGE_ASK_BARE"));
    s__matchable(py_func("_matchable", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_matchable", args)));
    s__the_plan_on_the_table(py_func("_the_plan_on_the_table", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_the_plan_on_the_table", args)));
}
pub fn asks_for_a_judgement(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks the bot to rule on the plan on the table, else False.\n\n    \"is that a good idea\", \"is that stupid\", \"am i mad to\", \"sensible?\" - the plan is on the table\n    from the line before, and the person wants it judged. A bare adjective only counts where a plan\n    is already named: \"daft\" with nothing on the table is not a question about anything.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __try1 = exc::catch(|| {
    said = mcall(&pyrt::xrefs::xcall("compose.fallback", "_matchable", &[clean.clone()]), "rstrip", &[py_str(" ?.!,")]);
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "JUDGE_ASK"), "match", &[said.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    bare = mcall(&pyrt::xrefs::xget("compose.fallback", "JUDGE_ASK_BARE"), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&bare.clone(), "group", &[py_str("bare_noun")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("compose.fallback_data", "JUDGE_BARE_FACES"), &mcall(&bare.clone(), "group", &[py_str("face")]))) } }))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[pyrt::xrefs::xcall("compose.fallback", "_the_plan_on_the_table", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.judgement".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "JUDGE_ASK" => Some(g_JUDGE_ASK()),
            "JUDGE_ASK_BARE" => Some(g_JUDGE_ASK_BARE()),
            "JUDGE_BARE_FACES" => Some(g_JUDGE_BARE_FACES()),
            "_matchable" => Some(g__matchable()),
            "_the_plan_on_the_table" => Some(g__the_plan_on_the_table()),
            "asks_for_a_judgement" => Some(py_func("asks_for_a_judgement", |args: &[Py]| asks_for_a_judgement(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "JUDGE_ASK" => s_JUDGE_ASK(v),
            "JUDGE_ASK_BARE" => s_JUDGE_ASK_BARE(v),
            "JUDGE_BARE_FACES" => s_JUDGE_BARE_FACES(v),
            "_matchable" => s__matchable(v),
            "_the_plan_on_the_table" => s__the_plan_on_the_table(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.judgement", module_obj);
    pyrt::xrefs::reg_fn("compose.judgement", "asks_for_a_judgement", py_func("asks_for_a_judgement", |args: &[Py]| asks_for_a_judgement(args)));
    pyrt::xrefs::reg_access("compose.judgement", "JUDGE_ASK", g_JUDGE_ASK, s_JUDGE_ASK);
    pyrt::xrefs::reg_access("compose.judgement", "JUDGE_ASK_BARE", g_JUDGE_ASK_BARE, s_JUDGE_ASK_BARE);
    pyrt::xrefs::reg_access("compose.judgement", "JUDGE_BARE_FACES", g_JUDGE_BARE_FACES, s_JUDGE_BARE_FACES);
    pyrt::xrefs::reg_access("compose.judgement", "_matchable", g__matchable, s__matchable);
    pyrt::xrefs::reg_access("compose.judgement", "_the_plan_on_the_table", g__the_plan_on_the_table, s__the_plan_on_the_table);
}

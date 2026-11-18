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


thread_local! { static G_GRIEF_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.recall: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_GRIEF_PLAN() -> Py { ensure_init(); G_GRIEF_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIEF_PLAN(v: Py) { ensure_init(); G_GRIEF_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_PLAN() -> Py { ensure_init(); G_LONG_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_PLAN(v: Py) { ensure_init(); G_LONG_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_WORDS() -> Py { ensure_init(); G_PLAN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_WORDS(v: Py) { ensure_init(); G_PLAN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAILS() -> Py { ensure_init(); G_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAILS(v: Py) { ensure_init(); G_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A plan said back on its own, with something to say after it.\n\n\"What am i doing this week\" is answered out of the store, and when the store holds one plan the\nanswer is that line and nothing else:\n\n    i have an exam on friday  /  what am i doing this week\n      \"You have an exam on friday.\"\n\nIt is the right fact and it is the whole reply. A friend told what the week holds says the thing\nand then says something with it; reading the plan out and stopping is the person's own sentence\nhanded back, which is the one thing a reply must not be.\n\nSo a single plan gets a short tail. The tail asks about the plan, never about the chat, and it is\nleft off when the plan is long enough that the line already has a shape of its own. Several plans\nneed no tail: two sentences read back are a list, and the second is the thing to say after the\nfirst.\n\nNor does a plan that was said back on the last turn: a repeat is framed as the repeat it is\n(\"Same answer, and it covers this: ...\"), and that framing can only see the sentence itself.\n\nTwo plans get no tail at all. One that names nothing but the days has no thing for \"it\" to hold,\nand one that is a funeral is not an errand: after a death the reply does not move to the\narrangements unless the person asks.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_TAILS(py_tuple(vec![py_str("How are you feeling about it?"), py_str("Is there anything you need before it?"), py_str("What do you want to have ready for it?")]));
    s_LONG_PLAN(Py::Int(78));
    s_PLAN_WORDS(call_builtin("frozenset", &[mcall(&py_str("a an the of to on in at for with and or but my your our their his her\nits it you i we they he she be been being am is are was were will would can could shall should\nmay might must going go gone away back off up down out over about around next last this that\nthese those coming week weeks month months year years day days night nights tomorrow today\ntonight soon again still just also too very really quite nearly almost more most less"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:funeral|funerals|wake|burial|cremation|crematorium|memorial|mourners?|viewing|interment|graveside|obsequies|death|deaths|died|dying|passing|passed away|grave|remembrance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _is_a_death(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("True when the plan itself is a death, a funeral, or the arrangements around one.\n\n    The arrangements are not the only way a plan names a death: \"my mums death anniversary\" and\n    \"my dads death certificate appointment\" are both a death with a date on it, and asking what\n    to have ready for them is the same misreading as asking it of a funeral.\n    ");
    return call_builtin("bool", &[mcall(&g_GRIEF_PLAN(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_on_the_last_turn(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut bot: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's own last line was this plan.\n\n    The tail is a question the bot puts after its own sentence, so the repeat reading has to\n    look through it: the same plan handed back twice in a row is the same answer twice, and\n    the turn that says so can only see it if the second one is the sentence on its own.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    bot = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return { let __b = call_builtin("bool", &[bot.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&bot.clone(), &said.clone())); if py_truthy(&__b) { __b } else { mcall(&bot.clone(), "startswith", &[op_add(&said.clone(), &py_str(" "))]) } } } };
    Py::None
}
pub fn _names_a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the plan says what it is about, so \"it\" in the tail has something to hold.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&mcall(&mcall(&said.clone(), "strip", &[py_str(" .")]), "split", &[]), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push({ let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLAN_WORDS(), &mcall(&word.clone(), "lower", &[]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn with_a_tail(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("The one plan said back, and a short question after it.\n\n    `said` is the sentence the store already built, ending in a full stop. Nothing about the\n    plan is added, and the tail asks about the plan rather than saying anything about the world.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONG_PLAN()); if py_truthy(&__b) { __b } else { op_gt(&mcall(&said.clone(), "count", &[py_str(".")]), &Py::Int(1)) } } } }) {
    return said.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_names_a_thing(&[said.clone()]))); if py_truthy(&__b) { __b } else { _is_a_death(&[said.clone()]) } }) {
    return said.clone();
    }
    if py_truthy(&_said_on_the_last_turn(&[said.clone(), memory.clone()])) {
    return said.clone();
    }
    return op_add(&op_add(&said.clone(), &py_str(" ")), &pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_TAILS()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), said.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.recall".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "GRIEF_PLAN" => Some(g_GRIEF_PLAN()),
            "LONG_PLAN" => Some(g_LONG_PLAN()),
            "PLAN_WORDS" => Some(g_PLAN_WORDS()),
            "TAILS" => Some(g_TAILS()),
            "_is_a_death" => Some(py_func("_is_a_death", |args: &[Py]| _is_a_death(args))),
            "_names_a_thing" => Some(py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args))),
            "_said_on_the_last_turn" => Some(py_func("_said_on_the_last_turn", |args: &[Py]| _said_on_the_last_turn(args))),
            "pick" => Some(g_pick()),
            "with_a_tail" => Some(py_func("with_a_tail", |args: &[Py]| with_a_tail(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "GRIEF_PLAN" => s_GRIEF_PLAN(v),
            "LONG_PLAN" => s_LONG_PLAN(v),
            "PLAN_WORDS" => s_PLAN_WORDS(v),
            "TAILS" => s_TAILS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.recall", module_obj);
    pyrt::xrefs::reg_fn("compose.recall", "_is_a_death", py_func("_is_a_death", |args: &[Py]| _is_a_death(args)));
    pyrt::xrefs::reg_fn("compose.recall", "_said_on_the_last_turn", py_func("_said_on_the_last_turn", |args: &[Py]| _said_on_the_last_turn(args)));
    pyrt::xrefs::reg_fn("compose.recall", "_names_a_thing", py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args)));
    pyrt::xrefs::reg_fn("compose.recall", "with_a_tail", py_func("with_a_tail", |args: &[Py]| with_a_tail(args)));
    pyrt::xrefs::reg_access("compose.recall", "GRIEF_PLAN", g_GRIEF_PLAN, s_GRIEF_PLAN);
    pyrt::xrefs::reg_access("compose.recall", "LONG_PLAN", g_LONG_PLAN, s_LONG_PLAN);
    pyrt::xrefs::reg_access("compose.recall", "PLAN_WORDS", g_PLAN_WORDS, s_PLAN_WORDS);
    pyrt::xrefs::reg_access("compose.recall", "TAILS", g_TAILS, s_TAILS);
    pyrt::xrefs::reg_access("compose.recall", "pick", g_pick, s_pick);
}

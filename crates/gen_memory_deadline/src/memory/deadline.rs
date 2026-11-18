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


thread_local! { static G_DEADLINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.deadline: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_DEADLINE() -> Py { ensure_init(); G_DEADLINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEADLINE(v: Py) { ensure_init(); G_DEADLINE.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN() -> Py { ensure_init(); G_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN(v: Py) { ensure_init(); G_WHEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that is only a deadline is a deadline on the plan the chat is already on.\n\n\"we want to save 3000 pounds\" / \"by december\" is a person adding the date to a goal they have just\ngiven, and the bot answered \"I'm listening - say more if it helps.\" Nothing in \"by december\" can\nbe looked up on its own, so the lookup finds nothing, and every reader that wants a thing named\npasses it by. The plan being paced is the one said a turn ago, and the date is the only new fact\nin the line.\n\nSo the line is read as the plan with the date on it (\"we want to save 3000 pounds by december\"),\nand the entry that answered the goal answers that: the saving entry works the per-month figure out\nfrom the date, where the goal alone gets the general advice. The words of the plan are never\ninvented - they are the turn before this one, unchanged - and the module passes on everything\nelse:\n\n* a deadline needs a plan in front of it. \"by december\" as the first thing said is a fragment,\n  and a conversation that opens with it has nothing to pace yet.\n* the plan has to be a plan: the line before has to ask for nothing and carry a goal in it (\"we\n  want to save 3000 pounds\", \"i need to get the kitchen done\"). A date after a question is a\n  date for that question's answer, and after a plain statement of fact (\"my exam is in june\") it\n  is not a deadline at all.\n* the line has to name nothing but a date. \"by december i want to be in a flat\" names what they\n  want and keeps its own turn.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.deadline.DEADLINE"); }
    s_DEADLINE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|so|but|ok|okay|right|well|also|actually|probably|ideally|hopefully|definitely|really|just|maybe)[ ,]+)*(?P<lead>by|before|until|til|till|up\\s+to|in\\s+time\\s+for|from)\\s+(?P<when>.+?)\\s*[.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.deadline.WHEN"); }
    s_WHEN(stdlib_call("re", "compile", &[py_str("^(?:(?:the\\s+)?(?:end|start|beginning|middle)\\s+of\\s+(?:the\\s+)?(?:month|year|week|summer)|(?:this|next|the)\\s+(?:month|year|week|summer|winter|spring|autumn|christmas|easter|new\\s+year|payday|weekend)|(?:christmas|easter|payday|new\\s+year|summer|winter|spring|autumn|nightfall|then|tomorrow|tonight|today)|(?:a|one)\\s+(?:year|month|week|fortnight|couple\\s+of\\s+(?:weeks?|months?|years?))|(?:\\d{1,2}|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:days?|weeks?|months?|years?|hours?)|(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec)|(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|\\d{1,2}(?:st|nd|rd|th))$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.deadline.MONEY"); }
    s_MONEY(stdlib_call("re", "compile", &[py_str("£\\s?\\d|\\b\\d+\\s?(?:grand|pounds?|quid)\\b|\\b\\d{3,}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.deadline.FILLER"); }
    s_FILLER(call_builtin("frozenset", &[mcall(&py_str("and so ok okay right well also actually just maybe probably ideally\n    hopefully definitely really then now too please thanks"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_deadline(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The date part of a line that is only a deadline (\"by december\" -> \"december\"), or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    found = mcall(&g_DEADLINE(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    when = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("when")]), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&when.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHEN(), "match", &[when.clone()]))) } }) {
    return py_str("");
    }
    return when.clone();
    Py::None
}
pub fn _a_plan(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line before is a plan the date can be put on.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "words", &[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_MONEY(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn follow_up(args: &[Py]) -> Py {
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
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The plan this deadline belongs to, or \"\" when the line is not that turn.");
    when = the_deadline(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_a_plan(&[said.clone()])) {
    plan = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&said.clone(), "strip", &[]), "rstrip", &[py_str(".!,")]), "split", &[])]);
    return op_add(&op_add(&plan.clone(), &py_str(" by ")), &when.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.deadline".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DEADLINE" => Some(g_DEADLINE()),
            "FILLER" => Some(g_FILLER()),
            "MONEY" => Some(g_MONEY()),
            "WHEN" => Some(g_WHEN()),
            "_a_plan" => Some(py_func("_a_plan", |args: &[Py]| _a_plan(args))),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            "the_deadline" => Some(py_func("the_deadline", |args: &[Py]| the_deadline(args))),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "DEADLINE" => s_DEADLINE(v),
            "FILLER" => s_FILLER(v),
            "MONEY" => s_MONEY(v),
            "WHEN" => s_WHEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.deadline", module_obj);
    pyrt::xrefs::reg_fn("memory.deadline", "the_deadline", py_func("the_deadline", |args: &[Py]| the_deadline(args)));
    pyrt::xrefs::reg_fn("memory.deadline", "_a_plan", py_func("_a_plan", |args: &[Py]| _a_plan(args)));
    pyrt::xrefs::reg_fn("memory.deadline", "follow_up", py_func("follow_up", |args: &[Py]| follow_up(args)));
    pyrt::xrefs::reg_access("memory.deadline", "DEADLINE", g_DEADLINE, s_DEADLINE);
    pyrt::xrefs::reg_access("memory.deadline", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("memory.deadline", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("memory.deadline", "WHEN", g_WHEN, s_WHEN);
}

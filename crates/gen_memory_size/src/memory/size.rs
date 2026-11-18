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


thread_local! { static G_BIGGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_BIGGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_SMALLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER_SIZE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BY_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUR_RULE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUR_SHORT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RULE_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIZED_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIZED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIZE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.size: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_BIGGER() -> Py { ensure_init(); G_BIGGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIGGER(v: Py) { ensure_init(); G_BIGGER.with(|c| *c.borrow_mut() = v); }
pub fn g_LESS() -> Py { ensure_init(); G_LESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LESS(v: Py) { ensure_init(); G_LESS.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE() -> Py { ensure_init(); G_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE(v: Py) { ensure_init(); G_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_BIGGER() -> Py { ensure_init(); G_SAID_BIGGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_BIGGER(v: Py) { ensure_init(); G_SAID_BIGGER.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_SMALLER() -> Py { ensure_init(); G_SAID_SMALLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_SMALLER(v: Py) { ensure_init(); G_SAID_SMALLER.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALLER() -> Py { ensure_init(); G_SMALLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALLER(v: Py) { ensure_init(); G_SMALLER.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER_SIZE() -> Py { ensure_init(); G__ANOTHER_SIZE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_SIZE(v: Py) { ensure_init(); G__ANOTHER_SIZE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE() -> Py { ensure_init(); G__A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE(v: Py) { ensure_init(); G__A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME() -> Py { ensure_init(); G__A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME(v: Py) { ensure_init(); G__A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__BY_WEIGHT() -> Py { ensure_init(); G__BY_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BY_WEIGHT(v: Py) { ensure_init(); G__BY_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD() -> Py { ensure_init(); G__FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD(v: Py) { ensure_init(); G__FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__OUR_RULE_LEADS() -> Py { ensure_init(); G__OUR_RULE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUR_RULE_LEADS(v: Py) { ensure_init(); G__OUR_RULE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__OUR_SHORT_LINES() -> Py { ensure_init(); G__OUR_SHORT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUR_SHORT_LINES(v: Py) { ensure_init(); G__OUR_SHORT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_BACK() -> Py { ensure_init(); G__POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_BACK(v: Py) { ensure_init(); G__POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__RULE_STOPS() -> Py { ensure_init(); G__RULE_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RULE_STOPS(v: Py) { ensure_init(); G__RULE_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g__SENTENCE() -> Py { ensure_init(); G__SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SENTENCE(v: Py) { ensure_init(); G__SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__SIZED_ASKED() -> Py { ensure_init(); G__SIZED_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIZED_ASKED(v: Py) { ensure_init(); G__SIZED_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__SIZED_THING() -> Py { ensure_init(); G__SIZED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIZED_THING(v: Py) { ensure_init(); G__SIZED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__SIZE_WORD() -> Py { ensure_init(); G__SIZE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIZE_WORD(v: Py) { ensure_init(); G__SIZE_WORD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The same thing in another size, asked straight after the answer.\n\n\"how long do i roast a chicken\" is answered with a rule that already carries the size in it (\"20\nminutes per 500g plus 20 minutes\"), and \"what about a bigger one\" is the person asking that rule\nfor their own bird. The follow-up names no food and no number, so every reader refused it and the\nperson was told the bot did not know enough to answer honestly, with a search suggested - one\nturn after an answer that had the whole of it.\n\nTwo things follow the answer, and both are said with what the answer already said:\n\n* a rate (\"per 500g\", \"for every pound\") is the answer for any size, so the rule is said again\n  and the sum is left to the person;\n* times that were given for one unnamed size (\"6 minutes for a runny yolk\", \"about an hour for a\n  medium potato\") are times for that size, so a bigger one wants a few minutes more and a smaller\n  one a few minutes less, with the method unchanged.\n\nOnly a follow-up that says \"bigger\", \"larger\", \"smaller\" and so on about \"one\" reaches this, and\nonly when the answer on the screen gave a time. Anything else is left to the modules below.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._ANOTHER_SIZE"); }
    s__ANOTHER_SIZE(stdlib_call("re", "compile", &[py_str("\\b(?:big|bigger|large|larger|small|smaller|little|littler|thicker|thinner|wider|narrower)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._SIZE_WORD"); }
    s__SIZE_WORD(py_str("(?:bigger|larger|smaller|littler|thicker|thinner|wider|narrower)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._POINTS_BACK"); }
    s__POINTS_BACK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&g__SIZE_WORD(), &py_str("\\s+(?:one|ones|version|size|sized|that|it|them|those|batch|tin|pot|dish|piece|bird|cut|joint|loaf|pan|mug|cup|glass|portion|helping)\\b|\\b(?:what about|and|how about|what if|or|then)\\s+(?:a|an|the)?\\s*(?:bit\\s+|little\\s+|much\\s+|whole\\s+)?(?:a|an|the)?\\s*")), &g__SIZE_WORD()), &py_str("\\s*[?.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._A_TIME"); }
    s__A_TIME(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:-|to)?\\s*\\d*\\s*(?:minutes?|mins?|hours?|hrs?)\\b|\\b\\d+\\s*(?:per|a)\\b|\\b(?:an?|one|two|three|four|five|ten|twenty|thirty|forty|half an)\\s+(?:minutes?|mins?|hours?|hrs?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._RULE_STOPS"); }
    s__RULE_STOPS(stdlib_call("re", "compile", &[py_str(",\\s*(?:so|which) |; |\\s-\\s|\\s\\u2014\\s")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._A_RATE"); }
    s__A_RATE(stdlib_call("re", "compile", &[py_str("\\bper\\s+(?:\\d+\\s*)?(?:g|kg|gram|grams|kilo|kilos|pound|pounds|lb|lbs|centimetre|centimetres|cm|inch|inches|litre|litres|person)\\b|\\bfor every\\b|\\beach\\s+\\d+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._SENTENCE"); }
    s__SENTENCE(stdlib_call("re", "compile", &[py_str("[^.!?]+[.!?]?")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.BIGGER"); }
    s_BIGGER(py_tuple(vec![py_str("Same rule, just a bigger sum: {rule}"), py_str("The rule works at any size: {rule}"), py_str("Same as before, and the sum does the work: {rule}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.SMALLER"); }
    s_SMALLER(py_tuple(vec![py_str("Same rule, just a smaller sum: {rule}"), py_str("The rule works at any size: {rule}"), py_str("Same as before, and the sum does the work: {rule}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.SAID_BIGGER"); }
    s_SAID_BIGGER(py_tuple(vec![py_str("That rule is the whole of it - the size goes into it and the time comes out."), py_str("Same rule, still: nothing changes with a bigger one except the figure you put into it."), py_str("It does not change with the size - put your figure into that rule and it holds.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.SAID_SMALLER"); }
    s_SAID_SMALLER(py_tuple(vec![py_str("That rule is the whole of it - the size goes into it and the time comes out."), py_str("Same rule, still: nothing changes with a smaller one except the figure you put into it."), py_str("It does not change with the size - put your figure into that rule and it holds.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.MORE"); }
    s_MORE(py_tuple(vec![py_str("A bigger one wants a few minutes more than that - same method, just check it later."), py_str("Give a bigger one longer than those times: a few minutes more, and check it."), py_str("Those times are for a normal one, so a bigger one takes a few minutes longer.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size.LESS"); }
    s_LESS(py_tuple(vec![py_str("A smaller one wants a few minutes less than that - same method, just check it earlier."), py_str("Give a smaller one less time than that: a few minutes fewer, then check it."), py_str("Those times are for a normal one, so a smaller one takes a few minutes less.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._OUR_SHORT_LINES"); }
    s__OUR_SHORT_LINES(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&op_add(&g_MORE(), &g_LESS()), &g_SAID_BIGGER()), &g_SAID_SMALLER())) { let mut shape: Py = __item.clone(); __out.push(_flat(&[shape.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._OUR_RULE_LEADS"); }
    s__OUR_RULE_LEADS(py_tuple(vec![py_str("same rule, just a bigger sum:"), py_str("same rule, just a smaller sum:"), py_str("the rule works at any size:"), py_str("same as before, and the sum does the work:")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._BY_WEIGHT"); }
    s__BY_WEIGHT(stdlib_call("re", "compile", &[py_str("\\bper\\s+(?:\\d+\\s*)?(?:g|kg|gram|grams|kilo|kilos|pound|pounds|lb|lbs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._FOOD"); }
    s__FOOD(py_str("chickens?|birds?|turkeys?|duck|lamb|beef|pork|ham|roasts?|steaks?|chops?|fillets?|fish|salmon|cod|trout|eggs?|potatoes|potato|onions?|carrots?|tomatoes|tomato|peppers?|courgettes?|aubergines?|cabbage|squash|melon|sausages?|cakes?|loaf|loaves|bread|pizza|pastry|pies?|tarts?|sponge|scones?|buns?|pancakes?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._SIZED_THING"); }
    s__SIZED_THING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g__FOOD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.size._SIZED_ASKED"); }
    s__SIZED_ASKED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g__FOOD()), &py_str("|joints?|breasts?|thighs?|legs?|cuts?|tins?|pots?|pans?|dishes|trays?|batches?|portions?|helpings?|mugs?|cups?|glasses)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "rstrip", &[py_str(".")]), "split", &[])]), "lower", &[]);
    Py::None
}
pub fn _one_of_ours(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut flat: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("True when the line is one of this module's own shapes.");
    flat = _flat(&[answer.clone()]);
    if py_truthy(&Py::Bool(py_contains(&g__OUR_SHORT_LINES(), &flat.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__OUR_RULE_LEADS()) { let mut lead: Py = __item.clone(); __out.push(mcall(&flat.clone(), "startswith", &[lead.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_asked(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut here: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line that the answer above answered, word for word, or \"\".\n\n    The walk goes back the way `_the_answer` does: the current line and this module's own size asks\n    are both skipped, so the egg row's second size ask still finds \"how long do i boil an egg\".\n    ");
    here = _flat(&[text.clone()]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    asked = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[asked.clone()]), &here.clone())) } }) {
    continue;
    }
    if py_truthy(&_one_of_ours(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    continue;
    }
    return asked.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_last_line(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's last line, word for word, or \"\".");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    return mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last answer with a time in it, walking back past this module's own lines.\n\n    A line of anybody else's with no time in it is not the answer, so the walk stops there and the\n    ask is left to the modules below.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&turn.clone(), "get", &[py_str("bot")]))) } }) {
    continue;
    }
    answer = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&_one_of_ours(&[answer.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g__A_TIME(), "search", &[answer.clone()])) {
    return answer.clone();
    }
    return py_str("");
    }
    return py_str("");
    Py::None
}
pub fn _the_rule(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut after: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut stop: Py = Py::None;
    let _ = py_str("The sentence of the answer that is the rule for any size, or \"\".");
    let __iter1 = py_iter_or_raise(&mcall(&g__SENTENCE(), "finditer", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    sentence = __item.clone();
    said = mcall(&mcall(&sentence.clone(), "group", &[Py::Int(0)]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_RATE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_TIME(), "search", &[said.clone()]) } }))) {
    continue;
    }
    after = if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str(": ")))) { mcall(&geti(&mcall(&said.clone(), "rsplit", &[py_str(": "), Py::Int(1)]), &Py::Int(1)), "strip", &[]) } else { py_str("") };
    let __iter2 = py_iter_or_raise(&py_tuple(vec![after.clone(), said.clone()]));
    for __item in __iter2 {
    piece = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_RATE(), "search", &[piece.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_TIME(), "search", &[piece.clone()]) } }))) {
    continue;
    }
    stop = mcall(&g__RULE_STOPS(), "search", &[piece.clone()]);
    if py_truthy(&stop.clone()) {
    piece = mcall(&slice_or_raise(&piece.clone(), &Py::None, &mcall(&stop.clone(), "start", &[]), &Py::None), "strip", &[]);
    }
    if py_truthy(&piece.clone()) {
    return piece.clone();
    }
    }
    }
    return py_str("");
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
    let mut answer: Py = Py::None;
    let mut answer_shapes: Py = Py::None;
    let mut bigger: Py = Py::None;
    let mut first: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shapes: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("The same thing in another size, when that is what the line asks; otherwise None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__ANOTHER_SIZE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__POINTS_BACK(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    answer = _the_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__BY_WEIGHT(), "search", &[answer.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__SIZED_THING(), "search", &[answer.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__SIZED_ASKED(), "search", &[_the_thing_asked(&[memory.clone(), said.clone()])]) } } } }))) {
    return Py::None;
    }
    bigger = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:small|smaller|little|littler|thinner|narrower)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    which = call_builtin("int", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("size_said")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&memory.clone(), &py_str("size_said"), op_add(&which.clone(), &Py::Int(1)));
    rule = _the_rule(&[answer.clone()]);
    if py_truthy(&rule.clone()) {
    if py_truthy(&_one_of_ours(&[_the_last_line(&[memory.clone()])])) {
    answer_shapes = if py_truthy(&bigger.clone()) { g_SAID_BIGGER() } else { g_SAID_SMALLER() };
    return geti(&answer_shapes.clone(), &op_modulo(&which.clone(), &call_builtin("len", &[answer_shapes.clone()]).unwrap_or_else(|e| exc::raise(e))));
    }
    shapes = if py_truthy(&bigger.clone()) { g_BIGGER() } else { g_SMALLER() };
    said = mcall(&rule.clone(), "rstrip", &[py_str(".")]);
    first = if py_truthy(&mcall(&said.clone(), "split", &[])) { geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&first.clone(), "isupper", &[]); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&first.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]) } })) } }) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rule".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&shapes.clone(), &op_modulo(&which.clone(), &call_builtin("len", &[shapes.clone()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    return op_add(&op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    shapes = if py_truthy(&bigger.clone()) { g_MORE() } else { g_LESS() };
    return geti(&shapes.clone(), &op_modulo(&which.clone(), &call_builtin("len", &[shapes.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.size".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BIGGER" => Some(g_BIGGER()),
            "LESS" => Some(g_LESS()),
            "MORE" => Some(g_MORE()),
            "SAID_BIGGER" => Some(g_SAID_BIGGER()),
            "SAID_SMALLER" => Some(g_SAID_SMALLER()),
            "SMALLER" => Some(g_SMALLER()),
            "_ANOTHER_SIZE" => Some(g__ANOTHER_SIZE()),
            "_A_RATE" => Some(g__A_RATE()),
            "_A_TIME" => Some(g__A_TIME()),
            "_BY_WEIGHT" => Some(g__BY_WEIGHT()),
            "_FOOD" => Some(g__FOOD()),
            "_OUR_RULE_LEADS" => Some(g__OUR_RULE_LEADS()),
            "_OUR_SHORT_LINES" => Some(g__OUR_SHORT_LINES()),
            "_POINTS_BACK" => Some(g__POINTS_BACK()),
            "_RULE_STOPS" => Some(g__RULE_STOPS()),
            "_SENTENCE" => Some(g__SENTENCE()),
            "_SIZED_ASKED" => Some(g__SIZED_ASKED()),
            "_SIZED_THING" => Some(g__SIZED_THING()),
            "_SIZE_WORD" => Some(g__SIZE_WORD()),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_one_of_ours" => Some(py_func("_one_of_ours", |args: &[Py]| _one_of_ours(args))),
            "_the_answer" => Some(py_func("_the_answer", |args: &[Py]| _the_answer(args))),
            "_the_last_line" => Some(py_func("_the_last_line", |args: &[Py]| _the_last_line(args))),
            "_the_rule" => Some(py_func("_the_rule", |args: &[Py]| _the_rule(args))),
            "_the_thing_asked" => Some(py_func("_the_thing_asked", |args: &[Py]| _the_thing_asked(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BIGGER" => s_BIGGER(v),
            "LESS" => s_LESS(v),
            "MORE" => s_MORE(v),
            "SAID_BIGGER" => s_SAID_BIGGER(v),
            "SAID_SMALLER" => s_SAID_SMALLER(v),
            "SMALLER" => s_SMALLER(v),
            "_ANOTHER_SIZE" => s__ANOTHER_SIZE(v),
            "_A_RATE" => s__A_RATE(v),
            "_A_TIME" => s__A_TIME(v),
            "_BY_WEIGHT" => s__BY_WEIGHT(v),
            "_FOOD" => s__FOOD(v),
            "_OUR_RULE_LEADS" => s__OUR_RULE_LEADS(v),
            "_OUR_SHORT_LINES" => s__OUR_SHORT_LINES(v),
            "_POINTS_BACK" => s__POINTS_BACK(v),
            "_RULE_STOPS" => s__RULE_STOPS(v),
            "_SENTENCE" => s__SENTENCE(v),
            "_SIZED_ASKED" => s__SIZED_ASKED(v),
            "_SIZED_THING" => s__SIZED_THING(v),
            "_SIZE_WORD" => s__SIZE_WORD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.size", module_obj);
    pyrt::xrefs::reg_fn("memory.size", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.size", "_one_of_ours", py_func("_one_of_ours", |args: &[Py]| _one_of_ours(args)));
    pyrt::xrefs::reg_fn("memory.size", "_the_thing_asked", py_func("_the_thing_asked", |args: &[Py]| _the_thing_asked(args)));
    pyrt::xrefs::reg_fn("memory.size", "_the_last_line", py_func("_the_last_line", |args: &[Py]| _the_last_line(args)));
    pyrt::xrefs::reg_fn("memory.size", "_the_answer", py_func("_the_answer", |args: &[Py]| _the_answer(args)));
    pyrt::xrefs::reg_fn("memory.size", "_the_rule", py_func("_the_rule", |args: &[Py]| _the_rule(args)));
    pyrt::xrefs::reg_fn("memory.size", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.size", "BIGGER", g_BIGGER, s_BIGGER);
    pyrt::xrefs::reg_access("memory.size", "LESS", g_LESS, s_LESS);
    pyrt::xrefs::reg_access("memory.size", "MORE", g_MORE, s_MORE);
    pyrt::xrefs::reg_access("memory.size", "SAID_BIGGER", g_SAID_BIGGER, s_SAID_BIGGER);
    pyrt::xrefs::reg_access("memory.size", "SAID_SMALLER", g_SAID_SMALLER, s_SAID_SMALLER);
    pyrt::xrefs::reg_access("memory.size", "SMALLER", g_SMALLER, s_SMALLER);
    pyrt::xrefs::reg_access("memory.size", "_ANOTHER_SIZE", g__ANOTHER_SIZE, s__ANOTHER_SIZE);
    pyrt::xrefs::reg_access("memory.size", "_A_RATE", g__A_RATE, s__A_RATE);
    pyrt::xrefs::reg_access("memory.size", "_A_TIME", g__A_TIME, s__A_TIME);
    pyrt::xrefs::reg_access("memory.size", "_BY_WEIGHT", g__BY_WEIGHT, s__BY_WEIGHT);
    pyrt::xrefs::reg_access("memory.size", "_FOOD", g__FOOD, s__FOOD);
    pyrt::xrefs::reg_access("memory.size", "_OUR_RULE_LEADS", g__OUR_RULE_LEADS, s__OUR_RULE_LEADS);
    pyrt::xrefs::reg_access("memory.size", "_OUR_SHORT_LINES", g__OUR_SHORT_LINES, s__OUR_SHORT_LINES);
    pyrt::xrefs::reg_access("memory.size", "_POINTS_BACK", g__POINTS_BACK, s__POINTS_BACK);
    pyrt::xrefs::reg_access("memory.size", "_RULE_STOPS", g__RULE_STOPS, s__RULE_STOPS);
    pyrt::xrefs::reg_access("memory.size", "_SENTENCE", g__SENTENCE, s__SENTENCE);
    pyrt::xrefs::reg_access("memory.size", "_SIZED_ASKED", g__SIZED_ASKED, s__SIZED_ASKED);
    pyrt::xrefs::reg_access("memory.size", "_SIZED_THING", g__SIZED_THING, s__SIZED_THING);
    pyrt::xrefs::reg_access("memory.size", "_SIZE_WORD", g__SIZE_WORD, s__SIZE_WORD);
}

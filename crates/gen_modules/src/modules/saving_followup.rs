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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_IN_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_respond: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_IN_MONTHS() -> Py { ensure_init(); G_A_LENGTH_IN_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_IN_MONTHS(v: Py) { ensure_init(); G_A_LENGTH_IN_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__SEEN() -> Py { ensure_init(); G__SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SEEN(v: Py) { ensure_init(); G__SEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_respond() -> Py { ensure_init(); G_respond.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_respond(v: Py) { ensure_init(); G_respond.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    s_respond(py_func("respond", |args: &[Py]| respond__wrapped(args)));
    let _ = py_str("The follow-up turns of a saving plan, answered before the general readings get them.\n\nA saving goal, the date for it and the rate are one question asked over three turns, and only\nthe first of the three says the word \"save\":\n\n    i want to save 2000 pounds    -> \"Then start with the smallest version of it\" (advice about a\n                                     habit, which says nothing about money)\n    by june                       -> \"Got it. Is there anything you'd like a hand with?\"\n    im putting by 500 a month     -> \"Got it - you're putting by 500 a month.\"\n\nEach of those lines on its own names nothing a knowledge entry can look up, so the readings that\nanswer a subjectless line took them and answered something else. Together they are arithmetic:\nthe goal, the weeks left and the rate give the figure the person actually wants.\n\n`bot/memory/saving.py` holds all of the reading. This module is the turn it is asked on: the\ncandidates below are tried in the order a person would ask them, and the first one that\nrecognises the line answers it. Every one of them returns \"\" for a line it is not about, so a\nchat with no saving plan in it is untouched.\n\nThe arithmetic itself is the money entry's - the goal, the date and the weekly figure are what\n`bot/knowledge/money.py` already works out - so the question is written the way that entry reads\nit (`saving.rate_question`) and answered in the knowledge base, rather than a second copy of the\nsums living here.\n");
    s_PRIORITY(Py::Int(1100));
    s_ANSWERS(py_tuple(vec![py_str("money")]));
    s_A_LENGTH_IN_MONTHS(stdlib_call("re", "compile", &[py_str("\\b(?:is|in|takes|takes\\s+about)\\s+(?:about\\s+)?(?:\\d[\\d,]*|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:month|months|year|years|week|weeks)\\b|\\bone\\s+pay\\s+packet\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SEEN(py_dict(vec![]));
    s_respond(gen_memory::memory::again::replied(&[py_func("respond", |args: &[Py]| respond__wrapped(args))]));
}
pub fn _lower_first(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The answer with its first letter lowered, for a lead-in sentence to run into.");
    text = call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&geti(&text.clone(), &Py::Int(0)), "isupper", &[]))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(2), &Py::None), "isupper", &[]) } } } }) {
    return text.clone();
    }
    return op_add(&mcall(&geti(&text.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _the_money_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answer: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The knowledge base's answer to the money question the line really is, or \"\".\n\n    Two rates asked in a row are two figures with the same closing sentence both times, which is\n    the answer above coming back with a new number sewn onto it, so the part already on the screen\n    is left off (`saving.without_what_was_just_said`).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&question.clone()))) {
    return py_str("");
    }
    was = mcall(&memory.clone(), "get", &[py_str("understood")]);
    let __try1 = exc::catch(|| {
    let _ = py_setitem(&memory.clone(), &py_str("understood"), gen_understanding::understanding::understand(&[question.clone(), memory.clone()]));
    answer = { let __b = crate::modules::answers::_respond(&[question.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if py_truthy(&Py::Bool(py_is(&was.clone(), &Py::None))) {
    let _ = mcall(&memory.clone(), "pop", &[py_str("understood"), Py::None]);
    } else {
    let _ = py_setitem(&memory.clone(), &py_str("understood"), was.clone());
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    return { let __b = gen_memory::memory::saving::without_what_was_just_said(&[answer.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { answer.clone() } };
    Py::None
}
pub fn _the_reply(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply this line asks for, or \"\" - read once per line per chat length.");
    let __try1 = exc::catch(|| {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    key = py_tuple(vec![call_builtin("id", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") }]);
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
    key = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&key.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__SEEN(), &key.clone())) } }) {
    return geti(&g__SEEN(), &key.clone());
    }
    reply = { let __b = _read(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_is(&key.clone(), &Py::None))) {
    if py_truthy(&op_gt(&call_builtin("len", &[g__SEEN()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(32))) {
    let _ = mcall(&g__SEEN(), "clear", &[]);
    }
    let _ = py_setitem(&g__SEEN(), &key.clone(), reply.clone());
    }
    return reply.clone();
    Py::None
}
pub fn respond__wrapped(args: &[Py]) -> Py {
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
    let _ = py_str("The reply this line asks for as part of a saving plan, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return { let __b = _the_reply(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This line is a saving-plan follow-up, read whole, so it is answered as it comes.\n\n    Without the hook the specialist rule hands the turn to the knowledge base: \"and if i cut the\n    gym thats another 40\" is read as money *and* fitness, the money entry declares both, and the\n    £40 answer about a pot nobody asked about wins on the fit. The claim says what is true - this\n    line is the rate half of a question this module has already read in full.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[_the_reply(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _read(args: &[Py]) -> Py {
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
    let mut a_rate_given: Py = Py::None;
    let mut again: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut at_the_chat_rate: Py = Py::None;
    let mut enough: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut length: Py = Py::None;
    let mut money_question: Py = Py::None;
    let mut more_a_month: Py = Py::None;
    let mut need_a_date: Py = Py::None;
    let mut no_price: Py = Py::None;
    let mut other: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut position: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut pot_alone: Py = Py::None;
    let mut price: Py = Py::None;
    let mut put_in: Py = Py::None;
    let mut raised: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut restated: Py = Py::None;
    let mut same_figure: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The reply this line asks for as part of a saving plan, or \"\".");
    gap = gen_memory::memory::saving::the_gap_left(&[text.clone(), memory.clone()]);
    if py_truthy(&gap.clone()) {
    return gap.clone();
    }
    position = gen_memory::memory::saving::the_pot_position(&[text.clone(), memory.clone()]);
    if py_truthy(&position.clone()) {
    return position.clone();
    }
    more_a_month = gen_memory::memory::saving::the_extra_a_month_asked(&[text.clone(), memory.clone()]);
    if py_truthy(&more_a_month.clone()) {
    return more_a_month.clone();
    }
    same_figure = gen_memory::memory::saving::the_same_figure_a_month(&[text.clone(), memory.clone()]);
    if py_truthy(&same_figure.clone()) {
    return same_figure.clone();
    }
    enough = gen_memory::memory::saving::a_yes_no_about_the_rate(&[text.clone(), memory.clone()]);
    if py_truthy(&enough.clone()) {
    return enough.clone();
    }
    need_a_date = gen_memory::memory::saving::a_rate_with_no_date(&[text.clone(), memory.clone()]);
    if py_truthy(&need_a_date.clone()) {
    return need_a_date.clone();
    }
    raised = gen_memory::memory::saving::a_rate_raised(&[text.clone(), memory.clone()]);
    if py_truthy(&raised.clone()) {
    return raised.clone();
    }
    restated = gen_memory::memory::saving::a_rate_restated(&[text.clone(), memory.clone()]);
    if py_truthy(&restated.clone()) {
    return restated.clone();
    }
    put_in = gen_memory::memory::saving::a_rate_put_in_with_a_verb(&[text.clone(), memory.clone()]);
    if py_truthy(&put_in.clone()) {
    return put_in.clone();
    }
    at_the_chat_rate = gen_memory::memory::saving::a_length_at_the_rate_in_the_chat(&[text.clone(), memory.clone()]);
    if py_truthy(&at_the_chat_rate.clone()) {
    answer = _the_money_answer(&[at_the_chat_rate.clone(), memory.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    no_price = gen_memory::memory::saving::a_length_with_no_price(&[text.clone(), memory.clone()]);
    if py_truthy(&no_price.clone()) {
    return no_price.clone();
    }
    price = gen_memory::memory::saving::a_price_in_an_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&price.clone()) {
    return price.clone();
    }
    a_rate_given = gen_memory::memory::saving::a_rate_for_the_goal(&[text.clone(), memory.clone()]);
    if py_truthy(&a_rate_given.clone()) {
    return a_rate_given.clone();
    }
    if py_truthy(&gen_memory::memory::saving::asks_how_long(&[text.clone()])) {
    length = gen_memory::memory::saving::how_long_it_takes(&[memory.clone()]);
    if py_truthy(&length.clone()) {
    return length.clone();
    }
    }
    pot_alone = gen_memory::memory::saving::the_pot_said_alone(&[text.clone()]);
    if py_truthy(&pot_alone.clone()) {
    whole = gen_memory::memory::saving::_the_goal_in_the_chat(&[memory.clone()]);
    if py_truthy(&op_gt(&whole.clone(), &pot_alone.clone())) {
    rest = op_sub(&whole.clone(), &pot_alone.clone());
    answer = _the_money_answer(&[gen_memory::memory::saving::rate_question_for(&[rest.clone()]), memory.clone()]);
    if py_truthy(&answer.clone()) {
    return op_modulo(&py_str("£%s still to find: %s"), &py_tuple(vec![call_builtin("format", &[rest.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _lower_first(&[gen_memory::memory::saving::without_the_pot_opener(&[answer.clone()])])]));
    }
    }
    }
    if py_truthy(&{ let __b = gen_memory::memory::saving::asks_about_saving(&[text.clone()]); if !py_truthy(&__b) { __b } else { gen_memory::memory::saving::_names_a_goal(&[text.clone()]) } }) {
    let __unpk1 = py_unpack(&gen_memory::memory::saving::the_pot_and_the_rest(&[text.clone()]), 2, None);
    pot = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    answer = py_str("");
    if py_truthy(&{ let __b = geti(&gen_memory::memory::saving::_a_rate_said(&[text.clone()]), &Py::Int(0)); if py_truthy(&__b) { __b } else { gen_memory::memory::saving::a_length_was_said(&[text.clone()]) } }) {
    plain = _the_money_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_LENGTH_IN_MONTHS(), "search", &[plain.clone()]) } }) {
    answer = plain.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    answer = _the_money_answer(&[gen_memory::memory::saving::rate_question(&[text.clone(), memory.clone()]), memory.clone()]);
    }
    if py_truthy(&answer.clone()) {
    if py_truthy(&{ let __b = pot.clone(); if !py_truthy(&__b) { __b } else { rest.clone() } }) {
    return op_modulo(&py_str("£%s to find on top of the £%s you already have: %s"), &py_tuple(vec![call_builtin("format", &[rest.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[pot.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _lower_first(&[gen_memory::memory::saving::without_the_pot_opener(&[answer.clone()])])]));
    }
    return answer.clone();
    }
    }
    if py_truthy(&gen_memory::memory::saving::asks_the_rate(&[text.clone(), memory.clone()])) {
    answer = _the_money_answer(&[gen_memory::memory::saving::rate_question(&[text.clone(), memory.clone()]), memory.clone()]);
    if py_truthy(&{ let __b = answer.clone(); if !py_truthy(&__b) { __b } else { gen_memory::memory::saving::said_already(&[answer.clone(), memory.clone()]) } }) {
    other = gen_memory::memory::saving::a_figure_each_week(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&other.clone())); if py_truthy(&__b) { __b } else { gen_memory::memory::saving::said_already(&[other.clone(), memory.clone()]) } }) {
    fresh = gen_memory::memory::saving::the_figure_a_week(&[memory.clone()]);
    other = if py_truthy(&fresh.clone()) { _the_money_answer(&[fresh.clone(), memory.clone()]) } else { py_str("") };
    }
    if py_truthy(&{ let __b = other.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::saving::said_already(&[other.clone(), memory.clone()]))) } }) {
    answer = other.clone();
    }
    }
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    if py_truthy(&gen_memory::memory::saving::is_only_a_date(&[text.clone()])) {
    settled = gen_memory::memory::saving::a_date_on_a_settled_rate(&[text.clone(), memory.clone()]);
    if py_truthy(&settled.clone()) {
    return settled.clone();
    }
    money_question = gen_memory::memory::saving::rate_question(&[text.clone(), memory.clone()]);
    if py_truthy(&money_question.clone()) {
    answer = _the_money_answer(&[money_question.clone(), memory.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    }
    again = gen_memory::memory::saving::another_sum_asked(&[text.clone(), memory.clone()]);
    if py_truthy(&again.clone()) {
    fresh = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = mcall(&fresh.clone(), "pop", &[py_str("understood"), Py::None]);
    answer = _the_money_answer(&[gen_memory::memory::saving::rate_question(&[again.clone(), fresh.clone()]), fresh.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    answer = _the_money_answer(&[again.clone(), fresh.clone()]);
    }
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.saving_followup".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "A_LENGTH_IN_MONTHS" => Some(g_A_LENGTH_IN_MONTHS()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_SEEN" => Some(g__SEEN()),
            "_lower_first" => Some(py_func("_lower_first", |args: &[Py]| _lower_first(args))),
            "_read" => Some(py_func("_read", |args: &[Py]| _read(args))),
            "_the_money_answer" => Some(py_func("_the_money_answer", |args: &[Py]| _the_money_answer(args))),
            "_the_reply" => Some(py_func("_the_reply", |args: &[Py]| _the_reply(args))),
            "respond" => Some(g_respond()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "A_LENGTH_IN_MONTHS" => s_A_LENGTH_IN_MONTHS(v),
            "PRIORITY" => s_PRIORITY(v),
            "_SEEN" => s__SEEN(v),
            "respond" => s_respond(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.saving_followup", module_obj);
    pyrt::xrefs::reg_fn("modules.saving_followup", "_lower_first", py_func("_lower_first", |args: &[Py]| _lower_first(args)));
    pyrt::xrefs::reg_fn("modules.saving_followup", "_the_money_answer", py_func("_the_money_answer", |args: &[Py]| _the_money_answer(args)));
    pyrt::xrefs::reg_fn("modules.saving_followup", "_the_reply", py_func("_the_reply", |args: &[Py]| _the_reply(args)));
    pyrt::xrefs::reg_fn("modules.saving_followup", "respond", py_func("respond", |args: &[Py]| py_call(&g_respond(), args)));
    pyrt::xrefs::reg_fn("modules.saving_followup", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.saving_followup", "_read", py_func("_read", |args: &[Py]| _read(args)));
    pyrt::xrefs::reg_access("modules.saving_followup", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.saving_followup", "A_LENGTH_IN_MONTHS", g_A_LENGTH_IN_MONTHS, s_A_LENGTH_IN_MONTHS);
    pyrt::xrefs::reg_access("modules.saving_followup", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.saving_followup", "_SEEN", g__SEEN, s__SEEN);
    pyrt::xrefs::reg_access("modules.saving_followup", "respond", g_respond, s_respond);
}

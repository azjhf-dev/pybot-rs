#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static MISSED_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSED_STEP_KEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSED_STEP_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_MISSED_STEP() -> Py { ensure_init(); MISSED_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP(v: Py) { ensure_init(); if let Ok(mut w) = MISSED_STEP.write() { *w = v; } }
pub fn g_MISSED_STEP_KEEPS() -> Py { ensure_init(); MISSED_STEP_KEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_KEEPS(v: Py) { ensure_init(); if let Ok(mut w) = MISSED_STEP_KEEPS.write() { *w = v; } }
pub fn g_MISSED_STEP_PLAN() -> Py { ensure_init(); MISSED_STEP_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = MISSED_STEP_PLAN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The two pool readers that weigh most, lifted out of fallback.py.\n\n`bot/compose/fallback.py` was over the publisher's 1 MiB per-file limit on main, and one file\nthat big stops every PR in the swarm from being filed, whoever wrote it: tables moved to\n`fallback_data.py` in #10007 and it was still 5,354 bytes over.\n\nSo the two readers that weigh most moved out whole - `_unknown`, which composes the honest\nreply when nothing else knows, and `_the_view_asked_for`, which gives the bot's own view back -\nwith every comment and every word unchanged. They read the tables and helpers in `fallback.py`,\nand `fallback.py` calls them through two small wrappers that import this module on the first\ncall, so neither module closes an import circle. No reply changes: this is a move.\n");
    s_MISSED_STEP(stdlib_call("re", "compile", &[py_str("\\bwhat (?:if|happens if) (?:i|we|you) (?:miss|skip|dont do|cant do|cannot do)\\b|\\bwhat (?:if|happens if) (?:i|we) (?:have to )?(?:miss|skip)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED_STEP_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|one|two|three|four|\\d+|a couple of|a few|couple of|few)?\\s*(?:days?|weeks?|fortnight|months?|sessions?|runs?|rides?|swims?|classes?|lessons?|workouts?|practice|training)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSED_STEP_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:pills?|doses?|drops|tablets?|meds|medication|medications|prescriptions?|insulin|inhalers?|injections?|jabs?|appointments?|periods?|contracepti\\w*|antidepressants?|antibiotics?|lenses?|dressing|dressings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_repeat_answer(args: &[Py]) -> Py {
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
    let mut sentences: Py = Py::None;
    let _ = py_str("The answer the person is asking for again, short enough to say again.\n\n    The same guard has two forms and only one of them answers. \"Same answer, and it covers this:\n    <card>\" carries what it names; \"that is everything I have on that one, so I would only repeat\n    myself\" names the answer on the screen and then hands the person none of it. Ask twice in\n    other words - \"can i leave my bags at the hotel\", then \"can i leave my case at the hotel\" -\n    and that is the turn they get, on a card that answers both.\n\n    So the answer above is carried, cut to its first two sentences when the card ran longer: the\n    repeat is the same answer, and the person asked for it.\n    ");
    answer = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = crate::gen::compose::fallback::_the_answer_above(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    sentences = stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), answer.clone()]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&gt(&call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { mcall(&py_str(" "), "join", &[slice_or_raise(&sentences.clone(), &Py::None, &Py::Int(2), &Py::None)]) } else { answer.clone() };
    Py::None
}
pub fn _drafting_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("The drafting invite, without asking for what the line already said.\n\n    \"Give me who it's for\" is the right question after \"can you write a text for me\", and the\n    wrong one after \"can you send a text to my landlord\" - the person named the landlord in the\n    same breath. So the pool is picked on the shape: when an addressee is already named, the\n    invite asks for the gist and the tone, which is what is actually missing.\n    ");
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_no_eyes(), "names_the_addressee", &[clean.clone()])) {
    return crate::gen::compose::fallback::g_UNKNOWN_WORD_NAMED();
    }
    return crate::gen::compose::fallback::g_UNKNOWN_WORD();
    Py::None
}
pub fn _a_care_thread(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(3) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few things the person said were about somebody being looked after.\n\n    Read from the thread rather than the line, because the line is the bare ask: \"my dog is\n    having an operation tomorrow\", \"im worried\", \"how long will it take\". A rule of thumb about\n    a roast is not one about a wait at a vet or a ward, and the thread says so when the line\n    cannot.\n    ");
    if py_truthy(&crate::gen::compose::fallback::_thread_is_about_a_pet(&[memory.clone()])) {
    return Py::Bool(true);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&crate::gen::compose::the_rule_for_it::g_A_CARE_WORD(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_money_thread(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(3) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few things the person said were about money they owe or are owed.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&crate::gen::compose::the_rule_for_it::g_A_MONEY_WORD(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_rule_for_it_for_the_turn(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut pool: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What settles this kind of ask, said before where the rest of it is kept, or None.\n\n    Three kinds have a rule the bot can always give: a price (where you buy it, the size, the\n    day - and a job is priced by the quote), an amount (a portion per head, times the count) and\n    a keeping question (how long it has been, and how cold). The pools live in\n    `the_rule_for_it.py` so this file stays the reader it was.\n\n    What stands down: a thread that is a crisis, a loss or care; a line naming a body, a symptom\n    or a health word, because a rule of thumb about a portion is not one about a dose; and a\n    thread that is about a body, for the same reason.\n    ");
    if py_truthy(&{ let __b = crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_grief_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]); if py_truthy(&__b) { __b } else { _a_care_thread(&[memory.clone()]) } } } } } } } } } }) {
    return Py::None;
    }
    words = bitor(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&bitand(&words.clone(), &bitor(&bitor(&crate::gen::compose::fallback::g_BODY_PARTS(), &crate::gen::compose::fallback::g_SYMPTOM_WORDS()), &crate::gen::compose::fallback::g_HEALTH_WORDS()))) {
    return Py::None;
    }
    pool = crate::gen::compose::the_rule_for_it::the_rule_for_it(&[clean.clone(), repaired.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::the_rule_for_it::g_TIME_RULE())); if !py_truthy(&__b) { __b } else { _a_money_thread(&[memory.clone()]) } }) {
    return Py::None;
    }
    return pool.clone();
    Py::None
}
pub fn _a_missed_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut plan: Py = Py::None;
    let _ = py_str("The pool for a stepped plan with one step missed, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MISSED_STEP(), "search", &[clean.clone()])))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&repaired.clone(), &clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MISSED_STEP(), "search", &[repaired.clone()]))) } }) {
    return Py::None;
    }
    }
    plan = { let __b = mcall(&g_MISSED_STEP_PLAN(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MISSED_STEP_PLAN(), "search", &[repaired.clone()]) } };
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MISSED_STEP_KEEPS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MISSED_STEP_KEEPS(), "search", &[repaired.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_HEALTH_WORDS()); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_BODY_PARTS()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_grief_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_thread_is_about_a_pet(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thread_is_about_someone_else(&[memory.clone()]) } } } } } } } } } }) {
    return Py::None;
    }
    return crate::gen::compose::fallback_data::g_MISSED_STEP_LINES();
    Py::None
}
pub fn _unknown(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut about_a_person: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut body_lines: Py = Py::None;
    let mut cannot_see: Py = Py::None;
    let mut care: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut day: Py = Py::None;
    let mut described: Py = Py::None;
    let mut for_the_thread: Py = Py::None;
    let mut forecast: Py = Py::None;
    let mut given: Py = Py::None;
    let mut how_line: Py = Py::None;
    let mut how_to: Py = Py::None;
    let mut in_the_room: Py = Py::None;
    let mut line: Py = Py::None;
    let mut line_words: Py = Py::None;
    let mut lost: Py = Py::None;
    let mut method: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut missed: Py = Py::None;
    let mut named: Py = Py::None;
    let mut no_thing_named: Py = Py::None;
    let mut own: Py = Py::None;
    let mut own_day: Py = Py::None;
    let mut own_thing: Py = Py::None;
    let mut pick: Py = Py::None;
    let mut place: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut price: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut routed: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut switched: Py = Py::None;
    let mut tips: Py = Py::None;
    let mut w: Py = Py::None;
    let mut waited: Py = Py::None;
    let mut weird: Py = Py::None;
    let mut words: Py = Py::None;
    let mut worry: Py = Py::None;
    let _ = py_str("An honest reply to a question nothing can answer: name who can, when it is clear.");
    repaired = { let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { clean.clone() } };
    own_day = crate::gen::compose::fallback::_their_own_day(&[clean.clone()]);
    if py_truthy(&own_day.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[own_day.clone(), memory.clone(), clean.clone()]);
    }
    body_lines = mcall(&crate::gen::compose::fallback::g_no_eyes(), "look_at_a_body_lines", &[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&body_lines.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[body_lines.clone(), memory.clone(), clean.clone()]);
    }
    cannot_see = mcall(&crate::gen::compose::fallback::g_no_eyes(), "cannot_see_your_thing", &[clean.clone()]);
    if py_truthy(&cannot_see.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[cannot_see.clone(), memory.clone(), clean.clone()]);
    }
    described = mcall(&crate::gen::compose::fallback::g_no_eyes(), "what_they_described", &[clean.clone(), memory.clone()]);
    if py_truthy(&described.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[described.clone(), memory.clone(), clean.clone()]);
    }
    in_the_room = mcall(&crate::gen::compose::fallback::g_no_eyes(), "a_thing_in_the_room", &[clean.clone()]);
    if py_truthy(&in_the_room.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[in_the_room.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_no_eyes(), "writes_the_message", &[clean.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[_drafting_pool(&[clean.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_in_a_crisis_thread(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_asks_for_a_person_to_talk_to(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_CRISIS_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_WHO_FOR_IT(), "match", &[clean.clone()]) } } } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_A_PERSON_IN_A_CRISIS(), memory.clone(), clean.clone()]);
    }
    lost = crate::gen::compose::fallback::the_lost_ask_reply(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&lost.clone()) {
    return lost.clone();
    }
    if py_truthy(&crate::gen::compose::fallback::_asks_a_name_back(&[clean.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])) {
    given = crate::gen::compose::fallback::_name_given_before(&[clean.clone(), memory.clone()]);
    if py_truthy(&given.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_HANDED_BACK()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), given.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), clean.clone()]);
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_NOT_NOTED(), memory.clone(), clean.clone()]);
    }
    forecast = crate::gen::compose::fallback::_forecast_pool(&[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&forecast.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[forecast.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::reads_as_if_i_saw(&[clean.clone(), memory.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_ASKED_IF_I_SAW_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_asked_about_the_day_they_had(&[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_grief_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]))) } } } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_WASNT_THERE(), memory.clone(), clean.clone()]);
    }
    how_to = mcall(&crate::gen::compose::fallback::g_HOW_TO_DECIDE(), "search", &[clean.clone()]);
    how_line = clean.clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    how_to = mcall(&crate::gen::compose::fallback::g_HOW_TO_DECIDE(), "search", &[repaired.clone()]);
    how_line = repaired.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_LIFE_DIRECTION(), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_nothing_to_choose(&[slice_or_raise(&how_line.clone(), &mcall(&how_to.clone(), "end", &[]), &Py::None, &Py::None)]) } } } }) {
    said_words = add(&add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), how_line.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    plain = mcall(&mcall(&said_words.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_HEALTH_WORDS()); if py_truthy(&__b) { __b } else { { let __b = bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_EXPERT_WORDS()); if py_truthy(&__b) { __b } else { { let __b = bitand(&call_builtin("set", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::compose::fallback::g_BODY_PARTS()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_ANIMALS(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback::g_ANIMALS(), &mcall(&w.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_HOW_TO_DECIDE_LINES(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&crate::gen::compose::fallback::_reads_as_taste(&[clean.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_UNKNOWN(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_WHETHER(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_WHAT_IT_IS_WORTH(), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::who_settles(&[clean.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_a_repair_already_spoken_of(&[memory.clone()]))) } } } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback_data::g_REPAIR_WHETHER_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_DECISION(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_TRADE(), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_PERSON(), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_LEAVING_IN_THE_LINE(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thread_is_about_a_person(&[memory.clone()]) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_TARIFF(), "search"), clean.clone(), repaired.clone()]))) } } } } } } } }) {
    if py_truthy(&{ let __b = crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_grief_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_CARE_HERE(), memory.clone(), clean.clone()]);
    }
    own_thing = { let __b = crate::gen::compose::fallback::_names_its_own_thing(&[clean.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_names_its_own_thing(&[repaired.clone()]) } };
    line_words = bitor(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), repaired.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    no_thing_named = { let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_REPAIR_BARE_IT(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_thread_names_a_machine(&[memory.clone()]))) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&own_thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let __b = crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_thread_names_a_machine(&[memory.clone()]))) } }; if py_truthy(&__b) { __b } else { bitand(&line_words.clone(), &bitor(&bitor(&crate::gen::compose::fallback::g_BODY_PARTS(), &crate::gen::compose::fallback::g_SYMPTOM_WORDS()), &crate::gen::compose::fallback::g_HEALTH_WORDS())) } } } }) {
    if py_truthy(&crate::gen::compose::fallback::_offers_to_fix_themselves(&[clean.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_BODY_IS_NOT_A_MACHINE(), memory.clone(), clean.clone()]);
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_UNKNOWN_HEALTH(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&no_thing_named.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_thread_is_about_a_person(&[memory.clone()]))); if py_truthy(&__b) { __b } else { { let __b = own_thing.clone(); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_thing_was_named(&[memory.clone()]) } } } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_REPAIR_RULE(), memory.clone(), clean.clone()]);
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_UNKNOWN_DECIDE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_TOO_LATE(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_thing_they_started(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]))) } } } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_UNKNOWN_TOO_LATE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = { let __b = crate::gen::compose::fallback::_a_plan_doubt(&[gattr(&crate::gen::compose::fallback::g_REALISTIC(), "search"), clean.clone(), repaired.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_plan_doubt(&[gattr(&crate::gen::compose::fallback::g_REALISTIC_ABOUT_ME(), "search"), clean.clone(), repaired.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_plan_in_the_thread(&[clean.clone(), memory.clone()]) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))) } }) {
    pool = if py_truthy(&crate::gen::compose::fallback::_the_thread_is_heavy(&[memory.clone()])) { crate::gen::compose::fallback::g_UNKNOWN_REALISTIC_CARE() } else { crate::gen::compose::fallback::g_UNKNOWN_REALISTIC() };
    return py_call(&crate::gen::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), clean.clone(), add(&add(&clean.clone(), &py_str(" ")), &crate::gen::compose::fallback::_the_line_before(&[memory.clone()]))]);
    }
    said = add(&add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), clean.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    words = mcall(&mcall(&said.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    waited = crate::gen::compose::waited_for::for_the_ask(&[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&waited.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&waited.clone(), &crate::gen::compose::waited_for::g_NOTHING_HELD())); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::compose::fallback::g_BARE_FOLLOW_UP(), "match", &[clean.clone()]) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[waited.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_BARE_FOLLOW_UP(), "match", &[clean.clone()])) {
    pool = if py_truthy(&crate::gen::compose::fallback::_the_thread_is_heavy(&[memory.clone()])) { crate::gen::compose::fallback::g_BARE_FOLLOW_UP_CARE_REPLIES() } else { crate::gen::compose::fallback::g_BARE_FOLLOW_UP_REPLIES() };
    return py_call(&crate::gen::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), clean.clone(), add(&add(&clean.clone(), &py_str(" ")), &crate::gen::compose::fallback::_the_line_before(&[memory.clone()]))]);
    }
    switched = crate::gen::compose::fallback::_a_switch_off_asked(&[memory.clone(), clean.clone()]);
    if py_truthy(&switched.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[switched.clone(), memory.clone(), add(&add(&clean.clone(), &py_str(" ")), &crate::gen::compose::fallback::_the_line_before(&[memory.clone()]))]);
    }
    if py_truthy(&crate::gen::compose::fallback::_points_at_nothing(&[clean.clone(), memory.clone(), repaired.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_POINTING_LINES(), memory.clone(), clean.clone()]);
    }
    weird = if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()])))) { crate::gen::compose::fallback::_the_weird_ask_pool(&[clean.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&weird.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[weird.clone(), memory.clone(), clean.clone()]);
    }
    pool = crate::gen::compose::fallback::unknown_pool(&[clean.clone(), words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    said = add(&add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9']"), py_str(" "), repaired.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    words = mcall(&mcall(&said.clone(), "replace", &[py_str("'"), py_str("")]), "split", &[]);
    pool = crate::gen::compose::fallback::unknown_pool(&[repaired.clone(), words.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_DECIDE())) } }) {
    missed = _a_missed_step(&[clean.clone(), repaired.clone(), memory.clone(), words.clone()]);
    if py_truthy(&Py::Bool(!py_is(&missed.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[missed.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_bare_worry(&[clean.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thread_is_about_the_body(&[memory.clone()]) } } } }) {
    pool = crate::gen::compose::fallback::g_UNKNOWN_HEALTH();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_bare_worry(&[clean.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thread_is_about_a_pet(&[memory.clone()]) } } } }) {
    pool = crate::gen::compose::fallback::g_UNKNOWN_ABOUT_A_PET();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_a_bare_worry(&[clean.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_thread_is_about_someone_else(&[memory.clone()]) } } } }) {
    pool = crate::gen::compose::fallback::g_UNKNOWN_ABOUT_SOMEONE();
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_DECIDE())) } }) {
    if py_truthy(&{ let __b = waited.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&waited.clone(), &crate::gen::compose::waited_for::g_NOTHING_HELD())) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[waited.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORRY())) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))) } }) {
    worry = crate::gen::compose::fallback::_the_pool_for_a_worry_about_them(&[clean.clone(), memory.clone()]);
    if py_truthy(&worry.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[worry.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORD()))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[_drafting_pool(&[clean.clone()]), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::asks_for_more(&[said.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]))) } }) {
    if py_truthy(&crate::gen::compose::fallback::_answered_recently(&[memory.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_HELP_ALREADY(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN()))) {
    routed = mcall(&crate::gen::compose::fallback::g_finding_out(), "for_the_ask", &[clean.clone(), repaired.clone(), understood.clone()]);
    if py_truthy(&routed.clone()) {
    said = py_call(&crate::gen::compose::fallback::g__choose(), &[routed.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    day = crate::gen::compose::for_the_day::for_the_ask(&[clean.clone(), repaired.clone()]);
    if py_truthy(&day.clone()) {
    said = py_call(&crate::gen::compose::fallback::g__choose(), &[day.clone(), memory.clone(), clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    named = crate::gen::compose::fallback::_advisor(&[memory.clone(), clean.clone(), said.clone(), words.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    for_the_thread = crate::gen::compose::fallback::_who_for_the_thread(&[memory.clone(), clean.clone(), understood.clone()]);
    if py_truthy(&for_the_thread.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[for_the_thread.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = py_call(&crate::gen::compose::fallback::g__reads(), &[py_func("_is_a_tips_ask", |args: &[Py]| crate::gen::compose::fallback::_is_a_tips_ask(args)), clean.clone(), repaired.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("help_ask")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_NOT_A_TIPS_ASK(), "search", &[clean.clone()]))) } } } }) {
    tips = crate::gen::compose::fallback::_tips(&[memory.clone(), clean.clone(), repaired.clone()]);
    if py_truthy(&tips.clone()) {
    return tips.clone();
    }
    if py_truthy(&{ let __b = mcall(&understood.clone(), "get", &[py_str("help_ask")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&py_call(&crate::gen::compose::fallback::g__reads(), &[gattr(&crate::gen::compose::fallback::g_TIPS_ASKS(), "search"), clean.clone(), repaired.clone()]))); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_hedged_recently(&[memory.clone()]) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_answered_already(&[memory.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_HELP_ALREADY(), memory.clone(), clean.clone()]);
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_TIPS_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("tag")])) {
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_answered_already(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::compose::fallback::g_TAG_CAN(), "search", &[clean.clone()]))) } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_TAG_CONFIRM(), memory.clone(), clean.clone()]);
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_TAG_UNSURE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_says_it_already(&[clean.clone(), call_builtin("list", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]) } }) {
    answer = _the_repeat_answer(&[memory.clone()]);
    if py_truthy(&answer.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("answer".to_string(), answer.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_SAME_AGAIN_WITH_THE_ANSWER(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_SAME_AGAIN(), memory.clone(), clean.clone()]);
    }
    pick = { let __b = crate::gen::compose::fallback::_bare_pick(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_bare_pick(&[repaired.clone(), memory.clone()]) } };
    if py_truthy(&pick.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[pick.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::_a_dead_end(&[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_in_a_care_thread(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = crate::gen::compose::fallback::_news_above(&[memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_a_subject_above(&[memory.clone()]) } })) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_DITHER_OPEN_LINES(), memory.clone(), clean.clone()]);
    }
    about_a_person = crate::gen::compose::fallback::_a_question_about_the_person(&[clean.clone(), memory.clone()]);
    if py_truthy(&about_a_person.clone()) {
    return about_a_person.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::compose::fallback::g_A_STEP_ASK(), "match", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_news_above(&[memory.clone()]) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_BARE_PICK_AFTER_NEWS(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN()))) {
    care = py_call(&crate::gen::compose::fallback::g__a_care_home_line(), &[understood.clone(), memory.clone()]);
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    }
    price = crate::gen::compose::fallback::_a_price_wants_a_verdict(&[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = price.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())) } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[price.clone(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN()))) {
    mine = crate::gen::compose::fallback_data::the_bots_own_answer(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    mine = crate::gen::compose::fallback_data::the_bots_own_answer(&[repaired.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&mine.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[mine.clone(), memory.clone(), clean.clone()]);
    }
    rule = _the_rule_for_it_for_the_turn(&[clean.clone(), repaired.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&rule.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[rule.clone(), memory.clone(), clean.clone()]);
    }
    place = if py_truthy(&crate::gen::compose::findout::reads_as_crowd(&[clean.clone()])) { Py::None } else { crate::gen::compose::fallback_data::the_place_the_answer_is_kept(&[clean.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::compose::findout::reads_as_crowd(&[clean.clone()]))) } } } }) {
    place = crate::gen::compose::fallback_data::the_place_the_answer_is_kept(&[repaired.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&place.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[place.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN_WORRY())) } }) {
    let mut own_ask: Py = crate::gen::compose::own_ask::module_obj();
    own = mcall(&own_ask.clone(), "answer", &[clean.clone(), memory.clone()]);
    if py_truthy(&own.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[py_list(vec![own.clone()]), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::gen::compose::fallback::g_UNKNOWN()))) {
    method = crate::gen::compose::fallback::method_ask_pool(&[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&method.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    method = crate::gen::compose::fallback::method_ask_pool(&[repaired.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&method.clone(), &Py::None))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[method.clone(), memory.clone(), clean.clone()]);
    }
    }
    counted = _a_countdown_in_a_care_thread_(&[memory.clone(), clean.clone(), repaired.clone(), understood.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _a_countdown_in_a_care_thread_(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut understood: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("The countdown branch, imported on the first call to keep the two modules apart.");
    let mut _a_countdown_in_a_care_thread: Py = py_func("_a_countdown_in_a_care_thread", |args: &[Py]| crate::gen::compose::fallback_chain::_a_countdown_in_a_care_thread(args));
    return py_call(&_a_countdown_in_a_care_thread, &[memory.clone(), clean.clone(), repaired.clone(), understood.clone()]);
    Py::None
}
pub fn _the_view_asked_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answered: Py = Py::None;
    let mut first: Py = Py::None;
    let mut headline: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut mirrored: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own view, given back, when that is all the line asks for.\n\n    Only one turn back, and only when that turn answered something: after a hedge there is no\n    view to give the person, and \"my view has not changed\" on the back of \"I don't know enough\"\n    is a worse non-answer than the hedge itself.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_is_a_view_ask(&[clean.clone()])))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    if py_truthy(&mcall(&crate::gen::compose::fallback::g_VIEW_NAMES_SOMEBODY(), "search", &[clean.clone()])) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_NO_SUBJECT(), memory.clone(), clean.clone()]);
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&crate::gen::compose::fallback::_only_the_floor(&[said.clone()])) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { lt(&mcall(&turns.clone(), "index", &[t.clone()]), &mcall(&turns.clone(), "index", &[last.clone()])) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&crate::gen::compose::fallback::_nothing_to_stand_behind(&[said.clone()])) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_ack(), "reads_as_a_receipt", &[said.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_in_a_pool(&[said.clone(), py_tuple(vec![crate::gen::compose::fallback::g_NEUTRAL()])]) } }))) {
    said = { let __b = crate::gen::compose::fallback::_the_answer_in_reply(&[mcall(&last.clone(), "get", &[py_str("user")]), said.clone()]); if py_truthy(&__b) { __b } else { said.clone() } };
    }
    while py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_ack(), "reads_as_a_receipt", &[said.clone()]); if py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_in_a_pool(&[said.clone(), py_tuple(vec![crate::gen::compose::fallback::g_NEUTRAL()])]) } }) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { lt(&mcall(&turns.clone(), "index", &[t.clone()]), &mcall(&turns.clone(), "index", &[last.clone()])) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &neg(&Py::Int(1)));
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    if py_truthy(&crate::gen::compose::fallback::_only_a_reaction(&[mcall(&last.clone(), "get", &[py_str("module")]), said.clone()])) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_ON_GOOD_NEWS(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::compose::fallback::g_CARE_MODULES(), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("loss"), py_str("grief")]), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_AFTER_A_DEATH(), memory.clone(), clean.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::compose::fallback::g_VIEW_HANDBACK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&said.clone(), "lower", &[]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_DECIDE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&crate::gen::compose::fallback::_is_a_hedge(&[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_VIEW_MIRROR(), "match", &[mcall(&said.clone(), "lower", &[])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::compose::fallback::_reads_as_a_claim(&[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&crate::gen::compose::fallback::_the_view_of(&[said.clone()]), &Py::Int(0)))) } } } }) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_COMPANY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::compose::fallback::_reads_as_a_claim(&[said.clone()])))) {
    first = mcall(&mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::compose::fallback::g_VIEW_MIRROR(), "match", &[first.clone()]); if !py_truthy(&__b) { __b } else { geti(&crate::gen::compose::fallback::_the_view_of(&[said.clone()]), &Py::Int(0)) } }))) {
    return Py::None;
    }
    }
    let __unpk1 = py_unpack(&crate::gen::compose::fallback::_the_view_of(&[said.clone()]), 2, None);
    headline = __unpk1[0].clone();
    mirrored = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&headline.clone()))) {
    if py_truthy(&mirrored.clone()) {
    return py_call(&crate::gen::compose::fallback::g__choose(), &[crate::gen::compose::fallback::g_VIEW_COMPANY(), memory.clone(), clean.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&headline.clone(), "endswith", &[py_str("?")])) {
    lines = crate::gen::compose::fallback::g_VIEW_ASK_LINES();
    } else {
    if py_truthy(&mirrored.clone()) {
    lines = crate::gen::compose::fallback::g_VIEW_MIRROR_LINES();
    } else {
    lines = crate::gen::compose::fallback::g_VIEW_LINES();
    }
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), crate::gen::compose::fallback::_after_a_colon(&[headline.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_pools".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "MISSED_STEP" => Some(g_MISSED_STEP()),
            "MISSED_STEP_KEEPS" => Some(g_MISSED_STEP_KEEPS()),
            "MISSED_STEP_PLAN" => Some(g_MISSED_STEP_PLAN()),
            "_a_care_thread" => Some(py_func("_a_care_thread", |args: &[Py]| _a_care_thread(args))),
            "_a_countdown_in_a_care_thread_" => Some(py_func("_a_countdown_in_a_care_thread_", |args: &[Py]| _a_countdown_in_a_care_thread_(args))),
            "_a_missed_step" => Some(py_func("_a_missed_step", |args: &[Py]| _a_missed_step(args))),
            "_a_money_thread" => Some(py_func("_a_money_thread", |args: &[Py]| _a_money_thread(args))),
            "_drafting_pool" => Some(py_func("_drafting_pool", |args: &[Py]| _drafting_pool(args))),
            "_the_repeat_answer" => Some(py_func("_the_repeat_answer", |args: &[Py]| _the_repeat_answer(args))),
            "_the_rule_for_it_for_the_turn" => Some(py_func("_the_rule_for_it_for_the_turn", |args: &[Py]| _the_rule_for_it_for_the_turn(args))),
            "_the_view_asked_for" => Some(py_func("_the_view_asked_for", |args: &[Py]| _the_view_asked_for(args))),
            "_unknown" => Some(py_func("_unknown", |args: &[Py]| _unknown(args))),
            _ => None,
        }),
        set: None,
    }))
}

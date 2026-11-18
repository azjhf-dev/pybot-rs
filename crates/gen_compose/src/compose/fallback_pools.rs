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


thread_local! { static G_A_CARE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIFE_DIRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSED_STEP_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TIPS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REALISTIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REALISTIC_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WHETHER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_CAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIPS_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_MIRROR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_NAMES_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_CARE_WORD() -> Py { ensure_init(); G_A_CARE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_WORD(v: Py) { ensure_init(); G_A_CARE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); G_A_MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); G_A_MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FOLLOW_UP() -> Py { ensure_init(); G_BARE_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FOLLOW_UP(v: Py) { ensure_init(); G_BARE_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_DECIDE() -> Py { ensure_init(); G_HOW_TO_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_DECIDE(v: Py) { ensure_init(); G_HOW_TO_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIFE_DIRECTION() -> Py { ensure_init(); G_LIFE_DIRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIFE_DIRECTION(v: Py) { ensure_init(); G_LIFE_DIRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP() -> Py { ensure_init(); G_MISSED_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP(v: Py) { ensure_init(); G_MISSED_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_KEEPS() -> Py { ensure_init(); G_MISSED_STEP_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_KEEPS(v: Py) { ensure_init(); G_MISSED_STEP_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_LINES() -> Py { ensure_init(); G_MISSED_STEP_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_LINES(v: Py) { ensure_init(); G_MISSED_STEP_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSED_STEP_PLAN() -> Py { ensure_init(); G_MISSED_STEP_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSED_STEP_PLAN(v: Py) { ensure_init(); G_MISSED_STEP_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TIPS_ASK() -> Py { ensure_init(); G_NOT_A_TIPS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TIPS_ASK(v: Py) { ensure_init(); G_NOT_A_TIPS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_LINES() -> Py { ensure_init(); G_POINTING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_LINES(v: Py) { ensure_init(); G_POINTING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_REALISTIC() -> Py { ensure_init(); G_REALISTIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REALISTIC(v: Py) { ensure_init(); G_REALISTIC.with(|c| *c.borrow_mut() = v); }
pub fn g_REALISTIC_ABOUT_ME() -> Py { ensure_init(); G_REALISTIC_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REALISTIC_ABOUT_ME(v: Py) { ensure_init(); G_REALISTIC_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WHETHER_LINES() -> Py { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHETHER_LINES(v: Py) { ensure_init(); G_REPAIR_WHETHER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_CAN() -> Py { ensure_init(); G_TAG_CAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_CAN(v: Py) { ensure_init(); G_TAG_CAN.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_RULE() -> Py { ensure_init(); G_TIME_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_RULE(v: Py) { ensure_init(); G_TIME_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIPS_ASKS() -> Py { ensure_init(); G_TIPS_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIPS_ASKS(v: Py) { ensure_init(); G_TIPS_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_LATE() -> Py { ensure_init(); G_TOO_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_LATE(v: Py) { ensure_init(); G_TOO_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN() -> Py { ensure_init(); G_UNKNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN(v: Py) { ensure_init(); G_UNKNOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_MIRROR() -> Py { ensure_init(); G_VIEW_MIRROR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_MIRROR(v: Py) { ensure_init(); G_VIEW_MIRROR.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_NAMES_SOMEBODY() -> Py { ensure_init(); G_VIEW_NAMES_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_NAMES_SOMEBODY(v: Py) { ensure_init(); G_VIEW_NAMES_SOMEBODY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The two pool readers that weigh most, lifted out of fallback.py.\n\n`bot/compose/fallback.py` was over the publisher's 1 MiB per-file limit on main, and one file\nthat big stops every PR in the swarm from being filed, whoever wrote it: tables moved to\n`fallback_data.py` in #10007 and it was still 5,354 bytes over.\n\nSo the two readers that weigh most moved out whole - `_unknown`, which composes the honest\nreply when nothing else knows, and `_the_view_asked_for`, which gives the bot's own view back -\nwith every comment and every word unchanged. They read the tables and helpers in `fallback.py`,\nand `fallback.py` calls them through two small wrappers that import this module on the first\ncall, so neither module closes an import circle. No reply changes: this is a move.\n");
    s_MISSED_STEP_LINES(crate::compose::fallback_data::g_MISSED_STEP_LINES());
    s_REPAIR_WHETHER_LINES(crate::compose::fallback_data::g_REPAIR_WHETHER_LINES());
    s_A_CARE_WORD(crate::compose::the_rule_for_it::g_A_CARE_WORD());
    s_A_MONEY_WORD(crate::compose::the_rule_for_it::g_A_MONEY_WORD());
    s_TIME_RULE(crate::compose::the_rule_for_it::g_TIME_RULE());
    s_BARE_FOLLOW_UP(crate::compose::fallback::g_BARE_FOLLOW_UP());
    s_HOW_TO_DECIDE(crate::compose::fallback::g_HOW_TO_DECIDE());
    s_LIFE_DIRECTION(crate::compose::fallback::g_LIFE_DIRECTION());
    s_NOT_A_TIPS_ASK(crate::compose::fallback::g_NOT_A_TIPS_ASK());
    s_POINTING_LINES(crate::compose::fallback::g_POINTING_LINES());
    s_REALISTIC(crate::compose::fallback::g_REALISTIC());
    s_REALISTIC_ABOUT_ME(crate::compose::fallback::g_REALISTIC_ABOUT_ME());
    s_TAG_CAN(crate::compose::fallback::g_TAG_CAN());
    s_TIPS_ASKS(crate::compose::fallback::g_TIPS_ASKS());
    s_TOO_LATE(crate::compose::fallback::g_TOO_LATE());
    s_UNKNOWN(crate::compose::fallback::g_UNKNOWN());
    s_VIEW_MIRROR(crate::compose::fallback::g_VIEW_MIRROR());
    s_VIEW_NAMES_SOMEBODY(crate::compose::fallback::g_VIEW_NAMES_SOMEBODY());
    s_MISSED_STEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:if|happens if) (?:i|we|you) (?:miss|skip|dont do|cant do|cannot do)\\b|\\bwhat (?:if|happens if) (?:i|we) (?:have to )?(?:miss|skip)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MISSED_STEP_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|one|two|three|four|\\d+|a couple of|a few|couple of|few)?\\s*(?:days?|weeks?|fortnight|months?|sessions?|runs?|rides?|swims?|classes?|lessons?|workouts?|practice|training)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MISSED_STEP_KEEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pills?|doses?|drops|tablets?|meds|medication|medications|prescriptions?|insulin|inhalers?|injections?|jabs?|appointments?|periods?|contracepti\\w*|antidepressants?|antibiotics?|lenses?|dressing|dressings)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_answer_above(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    sentences = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentences.clone(), &Py::None, &Py::Int(2), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } else { answer.clone() };
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
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "names_the_addressee", args))(&__args) }) {
    return crate::compose::fallback::g_UNKNOWN_WORD_NAMED();
    }
    return crate::compose::fallback::g_UNKNOWN_WORD();
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
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_a_pet(args))(&__args) }) {
    return Py::Bool(true);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::the_rule_for_it::g_A_CARE_WORD(), "search", args))(&__args) }) {
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
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::the_rule_for_it::g_A_MONEY_WORD(), "search", args))(&__args) }) {
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
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::in_a_safety_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_grief_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_loss_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_care_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_care_thread(args))(&__args) } } } } } } } } } } }) {
    return Py::None;
    }
    words = op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if py_truthy(&op_bitand(&words.clone(), &op_bitor(&op_bitor(&crate::compose::fallback::g_BODY_PARTS(), &crate::compose::fallback::g_SYMPTOM_WORDS()), &crate::compose::fallback::g_HEALTH_WORDS()))) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::the_rule_for_it::the_rule_for_it(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::the_rule_for_it::g_TIME_RULE())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_money_thread(args))(&__args) } } }) {
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP(), "search", args))(&__args) }))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&repaired.clone(), &clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    }
    plan = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP_PLAN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP_PLAN(), "search", args))(&__args) } } };
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP_KEEPS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MISSED_STEP_KEEPS(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::compose::fallback::g_HEALTH_WORDS()); if py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::compose::fallback::g_BODY_PARTS()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::in_a_safety_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_grief_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_loss_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_a_pet(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_someone_else(args))(&__args) } } } } } } } } } } }) {
    return Py::None;
    }
    return crate::compose::fallback_data::g_MISSED_STEP_LINES();
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
    repaired = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plain")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { clean.clone() } };
    own_day = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_their_own_day(args))(&__args) };
    if py_truthy(&own_day.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(own_day.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    body_lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "look_at_a_body_lines", args))(&__args) };
    if py_truthy(&body_lines.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body_lines.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    cannot_see = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "cannot_see_your_thing", args))(&__args) };
    if py_truthy(&cannot_see.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cannot_see.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    described = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "what_they_described", args))(&__args) };
    if py_truthy(&described.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(described.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    in_the_room = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "a_thing_in_the_room", args))(&__args) };
    if py_truthy(&in_the_room.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(in_the_room.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_no_eyes(), "writes_the_message", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _drafting_pool(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_crisis_thread(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_asks_for_a_person_to_talk_to(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_CRISIS_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_WHO_FOR_IT(), "match", args))(&__args) } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_A_PERSON_IN_A_CRISIS()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    lost = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::the_lost_ask_reply(args))(&__args) };
    if py_truthy(&lost.clone()) {
    return lost.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_asks_a_name_back(args))(&__args) }) {
    given = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_name_given_before(args))(&__args) };
    if py_truthy(&given.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback::g_HANDED_BACK()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("name".to_string(), given.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_NOT_NOTED()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    forecast = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_forecast_pool(args))(&__args) };
    if py_truthy(&forecast.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(forecast.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::reads_as_if_i_saw(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_ASKED_IF_I_SAW_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_asked_about_the_day_they_had(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::in_a_safety_thread(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_grief_thread(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_loss_thread(args))(&__args) })) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_WASNT_THERE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    how_to = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_HOW_TO_DECIDE(), "search", args))(&__args) };
    how_line = clean.clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    how_to = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_HOW_TO_DECIDE(), "search", args))(&__args) };
    how_line = repaired.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&how_to.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_LIFE_DIRECTION(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&how_line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&how_to.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_nothing_to_choose(args))(&__args) } } } } }) {
    said_words = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']")); __args.push(py_str(" ")); __args.push(how_line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "));
    plain = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_words.clone(), "replace", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::compose::fallback::g_HEALTH_WORDS()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::compose::fallback::g_EXPERT_WORDS()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &crate::compose::fallback::g_BODY_PARTS()); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&crate::compose::fallback::g_ANIMALS(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::compose::fallback::g_ANIMALS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "rstrip", args))(&__args) })) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_HOW_TO_DECIDE_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_reads_as_taste(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_UNKNOWN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_WHETHER(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_WHAT_IT_IS_WORTH(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::who_settles(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_repair_already_spoken_of(args))(&__args) })) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback_data::g_REPAIR_WHETHER_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_DECISION(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_TRADE(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_PERSON(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_LEAVING_IN_THE_LINE(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_a_person(args))(&__args) } } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_TARIFF(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })) } } } } } } } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::in_a_safety_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_grief_thread(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_loss_thread(args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_CARE_HERE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    own_thing = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_names_its_own_thing(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_names_its_own_thing(args))(&__args) } } };
    line_words = op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    no_thing_named = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REPAIR_BARE_IT(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_names_a_machine(args))(&__args) })) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&own_thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_names_a_machine(args))(&__args) })) } }; if py_truthy(&__b) { __b } else { op_bitand(&line_words.clone(), &op_bitor(&op_bitor(&crate::compose::fallback::g_BODY_PARTS(), &crate::compose::fallback::g_SYMPTOM_WORDS()), &crate::compose::fallback::g_HEALTH_WORDS())) } } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_offers_to_fix_themselves(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_BODY_IS_NOT_A_MACHINE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_UNKNOWN_HEALTH()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&no_thing_named.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_a_person(args))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = own_thing.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_thing_was_named(args))(&__args) } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_REPAIR_RULE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_UNKNOWN_DECIDE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_TOO_LATE(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_thing_they_started(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) })) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_UNKNOWN_TOO_LATE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REALISTIC(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_plan_doubt(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_REALISTIC_ABOUT_ME(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_plan_doubt(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_plan_in_the_thread(args))(&__args) } } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) })) } }) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_thread_is_heavy(args))(&__args) }) { crate::compose::fallback::g_UNKNOWN_REALISTIC_CARE() } else { crate::compose::fallback::g_UNKNOWN_REALISTIC() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(op_add(&op_add(&clean.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_line_before(args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    said = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']")); __args.push(py_str(" ")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "));
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "replace", args))(&__args) }, "split", args))(&__args) };
    waited = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::waited_for::for_the_ask(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&waited.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&waited.clone(), &crate::compose::waited_for::g_NOTHING_HELD())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_BARE_FOLLOW_UP(), "match", args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(waited.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_BARE_FOLLOW_UP(), "match", args))(&__args) }) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_thread_is_heavy(args))(&__args) }) { crate::compose::fallback::g_BARE_FOLLOW_UP_CARE_REPLIES() } else { crate::compose::fallback::g_BARE_FOLLOW_UP_REPLIES() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(op_add(&op_add(&clean.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_line_before(args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    switched = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_switch_off_asked(args))(&__args) };
    if py_truthy(&switched.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(switched.clone()); __args.push(memory.clone()); __args.push(op_add(&op_add(&clean.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_line_before(args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_points_at_nothing(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_POINTING_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    weird = if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) }))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_weird_ask_pool(args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&weird.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(weird.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::unknown_pool(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    said = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']")); __args.push(py_str(" ")); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str(" "));
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "replace", args))(&__args) }, "split", args))(&__args) };
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::unknown_pool(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_DECIDE())) } }) {
    missed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(memory.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_missed_step(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&missed.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(missed.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_bare_worry(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_the_body(args))(&__args) } } } } }) {
    pool = crate::compose::fallback::g_UNKNOWN_HEALTH();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_bare_worry(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_a_pet(args))(&__args) } } } } }) {
    pool = crate::compose::fallback::g_UNKNOWN_ABOUT_A_PET();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORRY())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_bare_worry(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_thread_is_about_someone_else(args))(&__args) } } } } }) {
    pool = crate::compose::fallback::g_UNKNOWN_ABOUT_SOMEONE();
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_DECIDE())) } }) {
    if py_truthy(&{ let __b = waited.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&waited.clone(), &crate::compose::waited_for::g_NOTHING_HELD())) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(waited.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORRY())) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) })) } }) {
    worry = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_pool_for_a_worry_about_them(args))(&__args) };
    if py_truthy(&worry.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(worry.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORD()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _drafting_pool(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::asks_for_more(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) })) } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_answered_recently(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_HELP_ALREADY()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN()))) {
    routed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_finding_out(), "for_the_ask", args))(&__args) };
    if py_truthy(&routed.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(routed.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    day = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::for_the_day::for_the_ask(args))(&__args) };
    if py_truthy(&day.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(day.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(said.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_advisor(args))(&__args) };
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    for_the_thread = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_who_for_the_thread(args))(&__args) };
    if py_truthy(&for_the_thread.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(for_the_thread.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_func("_is_a_tips_ask", |args: &[Py]| crate::compose::fallback::_is_a_tips_ask(args))); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("help_ask")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_NOT_A_TIPS_ASK(), "search", args))(&__args) })) } } } }) {
    tips = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_tips(args))(&__args) };
    if py_truthy(&tips.clone()) {
    return tips.clone();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("help_ask")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(gattr(&crate::compose::fallback::g_TIPS_ASKS(), "search")); __args.push(clean.clone()); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__reads(), args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_hedged_recently(args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_answered_already(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_HELP_ALREADY()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_TIPS_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tag")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_answered_already(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_TAG_CAN(), "search", args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_TAG_CONFIRM()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_TAG_UNSURE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_says_it_already(args))(&__args) } } }) {
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_repeat_answer(args))(&__args) };
    if py_truthy(&answer.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("answer".to_string(), answer.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_SAME_AGAIN_WITH_THE_ANSWER()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_SAME_AGAIN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    pick = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_bare_pick(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_bare_pick(args))(&__args) } } };
    if py_truthy(&pick.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pick.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_dead_end(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_care_thread(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_news_above(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_subject_above(args))(&__args) } } })) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_DITHER_OPEN_LINES()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    about_a_person = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_question_about_the_person(args))(&__args) };
    if py_truthy(&about_a_person.clone()) {
    return about_a_person.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_A_STEP_ASK(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_news_above(args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_BARE_PICK_AFTER_NEWS()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN()))) {
    care = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__a_care_home_line(), args))(&__args) };
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    }
    price = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_a_price_wants_a_verdict(args))(&__args) };
    if py_truthy(&{ let __b = price.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(price.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN()))) {
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_bots_own_answer(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_bots_own_answer(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_is(&mine.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    rule = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rule_for_it_for_the_turn(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&rule.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rule.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    place = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::findout::reads_as_crowd(args))(&__args) }) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_place_the_answer_is_kept(args))(&__args) } };
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::findout::reads_as_crowd(args))(&__args) })) } } } }) {
    place = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_place_the_answer_is_kept(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_is(&place.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(place.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN_WORRY())) } }) {
    let mut own_ask: Py = crate::compose::own_ask::module_obj();
    own = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&own_ask.clone(), "answer", args))(&__args) };
    if py_truthy(&own.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![own.clone()])); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &crate::compose::fallback::g_UNKNOWN()))) {
    method = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::method_ask_pool(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&method.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &clean.clone())) } }) {
    method = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::method_ask_pool(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_is(&method.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(method.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    }
    counted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_countdown_in_a_care_thread_(args))(&__args) };
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
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
    let mut _a_countdown_in_a_care_thread: Py = py_func("_a_countdown_in_a_care_thread", |args: &[Py]| crate::compose::fallback_chain::_a_countdown_in_a_care_thread(args));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); __args.push(repaired.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&_a_countdown_in_a_care_thread, args))(&__args) };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_is_a_view_ask(args))(&__args) }))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_VIEW_NAMES_SOMEBODY(), "search", args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_NO_SUBJECT()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_only_the_floor(args))(&__args) }) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turns.clone(), "index", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turns.clone(), "index", args))(&__args) }) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &op_neg(&Py::Int(1)));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_nothing_to_stand_behind(args))(&__args) }) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_ack(), "reads_as_a_receipt", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_tuple(vec![crate::compose::fallback::g_NEUTRAL()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_pool(args))(&__args) } } }))) {
    said = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_answer_in_reply(args))(&__args) }; if py_truthy(&__b) { __b } else { said.clone() } };
    }
    while py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_ack(), "reads_as_a_receipt", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_tuple(vec![crate::compose::fallback::g_NEUTRAL()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_in_a_pool(args))(&__args) } } }) {
    answered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&t.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turns.clone(), "index", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turns.clone(), "index", args))(&__args) }) } } } } } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answered.clone()))) {
    return Py::None;
    }
    last = geti(&answered.clone(), &op_neg(&Py::Int(1)));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_only_a_reaction(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_ON_GOOD_NEWS()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&crate::compose::fallback::g_CARE_MODULES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("loss"), py_str("grief")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_AFTER_A_DEATH()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback::g_VIEW_HANDBACK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_DECIDE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_is_a_hedge(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_VIEW_MIRROR(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_reads_as_a_claim(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_view_of(args))(&__args) }, &Py::Int(0)))) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_COMPANY()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_reads_as_a_claim(args))(&__args) }))) {
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fallback::g_VIEW_MIRROR(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_view_of(args))(&__args) }, &Py::Int(0)) } }))) {
    return Py::None;
    }
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_the_view_of(args))(&__args) }, 2, None);
    headline = __unpk1[0].clone();
    mirrored = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&headline.clone()))) {
    if py_truthy(&mirrored.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::fallback::g_VIEW_COMPANY()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&headline.clone(), "endswith", args))(&__args) }) {
    lines = crate::compose::fallback::g_VIEW_ASK_LINES();
    } else {
    if py_truthy(&mirrored.clone()) {
    lines = crate::compose::fallback::g_VIEW_MIRROR_LINES();
    } else {
    lines = crate::compose::fallback::g_VIEW_LINES();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(headline.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::_after_a_colon(args))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_pools".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CARE_WORD" => Some(g_A_CARE_WORD()),
            "A_MONEY_WORD" => Some(g_A_MONEY_WORD()),
            "BARE_FOLLOW_UP" => Some(g_BARE_FOLLOW_UP()),
            "HOW_TO_DECIDE" => Some(g_HOW_TO_DECIDE()),
            "LIFE_DIRECTION" => Some(g_LIFE_DIRECTION()),
            "MISSED_STEP" => Some(g_MISSED_STEP()),
            "MISSED_STEP_KEEPS" => Some(g_MISSED_STEP_KEEPS()),
            "MISSED_STEP_LINES" => Some(g_MISSED_STEP_LINES()),
            "MISSED_STEP_PLAN" => Some(g_MISSED_STEP_PLAN()),
            "NOT_A_TIPS_ASK" => Some(g_NOT_A_TIPS_ASK()),
            "POINTING_LINES" => Some(g_POINTING_LINES()),
            "REALISTIC" => Some(g_REALISTIC()),
            "REALISTIC_ABOUT_ME" => Some(g_REALISTIC_ABOUT_ME()),
            "REPAIR_WHETHER_LINES" => Some(g_REPAIR_WHETHER_LINES()),
            "TAG_CAN" => Some(g_TAG_CAN()),
            "TIME_RULE" => Some(g_TIME_RULE()),
            "TIPS_ASKS" => Some(g_TIPS_ASKS()),
            "TOO_LATE" => Some(g_TOO_LATE()),
            "UNKNOWN" => Some(g_UNKNOWN()),
            "VIEW_MIRROR" => Some(g_VIEW_MIRROR()),
            "VIEW_NAMES_SOMEBODY" => Some(g_VIEW_NAMES_SOMEBODY()),
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CARE_WORD" => s_A_CARE_WORD(v),
            "A_MONEY_WORD" => s_A_MONEY_WORD(v),
            "BARE_FOLLOW_UP" => s_BARE_FOLLOW_UP(v),
            "HOW_TO_DECIDE" => s_HOW_TO_DECIDE(v),
            "LIFE_DIRECTION" => s_LIFE_DIRECTION(v),
            "MISSED_STEP" => s_MISSED_STEP(v),
            "MISSED_STEP_KEEPS" => s_MISSED_STEP_KEEPS(v),
            "MISSED_STEP_LINES" => s_MISSED_STEP_LINES(v),
            "MISSED_STEP_PLAN" => s_MISSED_STEP_PLAN(v),
            "NOT_A_TIPS_ASK" => s_NOT_A_TIPS_ASK(v),
            "POINTING_LINES" => s_POINTING_LINES(v),
            "REALISTIC" => s_REALISTIC(v),
            "REALISTIC_ABOUT_ME" => s_REALISTIC_ABOUT_ME(v),
            "REPAIR_WHETHER_LINES" => s_REPAIR_WHETHER_LINES(v),
            "TAG_CAN" => s_TAG_CAN(v),
            "TIME_RULE" => s_TIME_RULE(v),
            "TIPS_ASKS" => s_TIPS_ASKS(v),
            "TOO_LATE" => s_TOO_LATE(v),
            "UNKNOWN" => s_UNKNOWN(v),
            "VIEW_MIRROR" => s_VIEW_MIRROR(v),
            "VIEW_NAMES_SOMEBODY" => s_VIEW_NAMES_SOMEBODY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.fallback_pools", module_obj);
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_repeat_answer", py_func("_the_repeat_answer", |args: &[Py]| _the_repeat_answer(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_drafting_pool", py_func("_drafting_pool", |args: &[Py]| _drafting_pool(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_care_thread", py_func("_a_care_thread", |args: &[Py]| _a_care_thread(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_money_thread", py_func("_a_money_thread", |args: &[Py]| _a_money_thread(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_rule_for_it_for_the_turn", py_func("_the_rule_for_it_for_the_turn", |args: &[Py]| _the_rule_for_it_for_the_turn(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_missed_step", py_func("_a_missed_step", |args: &[Py]| _a_missed_step(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_unknown", py_func("_unknown", |args: &[Py]| _unknown(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_a_countdown_in_a_care_thread_", py_func("_a_countdown_in_a_care_thread_", |args: &[Py]| _a_countdown_in_a_care_thread_(args)));
    pyrt::xrefs::reg_fn("compose.fallback_pools", "_the_view_asked_for", py_func("_the_view_asked_for", |args: &[Py]| _the_view_asked_for(args)));
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_CARE_WORD", g_A_CARE_WORD, s_A_CARE_WORD);
    pyrt::xrefs::reg_access("compose.fallback_pools", "A_MONEY_WORD", g_A_MONEY_WORD, s_A_MONEY_WORD);
    pyrt::xrefs::reg_access("compose.fallback_pools", "BARE_FOLLOW_UP", g_BARE_FOLLOW_UP, s_BARE_FOLLOW_UP);
    pyrt::xrefs::reg_access("compose.fallback_pools", "HOW_TO_DECIDE", g_HOW_TO_DECIDE, s_HOW_TO_DECIDE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "LIFE_DIRECTION", g_LIFE_DIRECTION, s_LIFE_DIRECTION);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP", g_MISSED_STEP, s_MISSED_STEP);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_KEEPS", g_MISSED_STEP_KEEPS, s_MISSED_STEP_KEEPS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_LINES", g_MISSED_STEP_LINES, s_MISSED_STEP_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "MISSED_STEP_PLAN", g_MISSED_STEP_PLAN, s_MISSED_STEP_PLAN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "NOT_A_TIPS_ASK", g_NOT_A_TIPS_ASK, s_NOT_A_TIPS_ASK);
    pyrt::xrefs::reg_access("compose.fallback_pools", "POINTING_LINES", g_POINTING_LINES, s_POINTING_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REALISTIC", g_REALISTIC, s_REALISTIC);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REALISTIC_ABOUT_ME", g_REALISTIC_ABOUT_ME, s_REALISTIC_ABOUT_ME);
    pyrt::xrefs::reg_access("compose.fallback_pools", "REPAIR_WHETHER_LINES", g_REPAIR_WHETHER_LINES, s_REPAIR_WHETHER_LINES);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TAG_CAN", g_TAG_CAN, s_TAG_CAN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TIME_RULE", g_TIME_RULE, s_TIME_RULE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TIPS_ASKS", g_TIPS_ASKS, s_TIPS_ASKS);
    pyrt::xrefs::reg_access("compose.fallback_pools", "TOO_LATE", g_TOO_LATE, s_TOO_LATE);
    pyrt::xrefs::reg_access("compose.fallback_pools", "UNKNOWN", g_UNKNOWN, s_UNKNOWN);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_MIRROR", g_VIEW_MIRROR, s_VIEW_MIRROR);
    pyrt::xrefs::reg_access("compose.fallback_pools", "VIEW_NAMES_SOMEBODY", g_VIEW_NAMES_SOMEBODY, s_VIEW_NAMES_SOMEBODY);
}

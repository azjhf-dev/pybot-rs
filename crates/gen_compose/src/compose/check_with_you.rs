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


thread_local! { static G_AGREED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHECK_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_FOR_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB_FOR_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERB_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_FOR_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_STANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AGREED() -> Py { ensure_init(); G_AGREED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGREED(v: Py) { ensure_init(); G_AGREED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHECK() -> Py { ensure_init(); G_A_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHECK(v: Py) { ensure_init(); G_A_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHECK_ASKED() -> Py { ensure_init(); G__A_CHECK_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHECK_ASKED(v: Py) { ensure_init(); G__A_CHECK_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_FOR_YOU() -> Py { ensure_init(); G__A_THING_FOR_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_FOR_YOU(v: Py) { ensure_init(); G__A_THING_FOR_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB_FOR_YOU() -> Py { ensure_init(); G__A_VERB_FOR_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB_FOR_YOU(v: Py) { ensure_init(); G__A_VERB_FOR_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERB_YOU() -> Py { ensure_init(); G__A_VERB_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERB_YOU(v: Py) { ensure_init(); G__A_VERB_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_FOR_YOU() -> Py { ensure_init(); G__BARE_FOR_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_FOR_YOU(v: Py) { ensure_init(); G__BARE_FOR_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_STANCE() -> Py { ensure_init(); G__BARE_STANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_STANCE(v: Py) { ensure_init(); G__BARE_STANCE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_CHECK() -> Py { ensure_init(); G__NOT_A_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_CHECK(v: Py) { ensure_init(); G__NOT_A_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g__OBJECT() -> Py { ensure_init(); G__OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OBJECT(v: Py) { ensure_init(); G__OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__STANCE() -> Py { ensure_init(); G__STANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STANCE(v: Py) { ensure_init(); G__STANCE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A check-with-you question - \"u ok with that\", \"is that ok with you\" - gets an answer.\n\nOne shape, and today it lands in seven different places, none of them an answer:\n\n    u ok with that           -> \"Thanks for telling me - I'm here if you want to dig into any of that.\"\n    are you ok with that     -> \"I'm a chatbot, not a person: a program built to help ...\"\n    are you fine with that   -> \"That's beyond what I know well, sorry. Searching for it ...\"\n    u happy with that        -> \"Lovely to hear. Enjoy it!\"\n    you good with that plan  -> \"Got it. Is there anything you'd like a hand with?\"\n    is that ok with you      -> \"I don't know enough to answer that honestly. A search should ...\"\n    does that work for you   -> \"That's beyond what I know well, sorry. Searching for it ...\"\n\nThree of those are the search hedge, two are receipts (the person's own words handed back), one\nreads \"are you ok\" as a question about what the bot is, and \"u happy with that\" was taken as a\nreport of good news. The line is not any of those things: the person is checking a plan against\nthe bot, and the only honest answer is agreement plus whose call it is. The bot has no stake; the\nperson does, so the line says it is fine by the bot and leaves it with them.\n\nThe shapes are tight on purpose - the bot as the subject with a stance word and a connector, or\nan object stance with the bot as the one it has to suit - because the same words in other orders\nbelong to other readers: \"is that ok to drive on\" is a question about a thing (the cards have the\ndriving rules), \"are you safe to talk to\" is the privacy card, and \"u alright to drive\" is the\nperson checking themselves, which has no business being answered by a bot.\n");
    s__STANCE(py_str("(?:ok|okay|alright|all\\s+right|fine|happy|cool|good|easy|content)"));
    s__OBJECT(py_str("(?:that|this|it|them|those|these|the\\s+\\w+|my\\s+\\w+|me(?:\\s+\\w+ing)?|me\\s+\\w+|if\\s+i\\b|it\\s+if)"));
    s__A_CHECK_ASKED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:are|r|is|was|do|does|would|will|could)?\\s*(?:you|u|ya)\\s+(?:really\\s+|still\\s+|definitely\\s+)?(?:be\\s+)?"), &g__STANCE()), &py_str("\\s+(?:with|wiv|wid|about|by)\\s+")), &g__OBJECT()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_FOR_YOU(stdlib_call("re", "compile", &[py_str("\\b(?:(?:is|was|are|were)\\s+)?(?:that|this|it|those|these)\\s+(?:really\\s+)?(?:ok|okay|alright|all\\s+right|fine)\\s+(?:with|wiv|wid|by|for)\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_VERB_FOR_YOU(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|would|will|did)?\\s*(?:that|this|it)\\s+(?:work|suit|square|be\\s+alright|be\\s+ok|be\\s+fine)\\s+(?:for|with|wiv|wid)\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_VERB_YOU(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|would|will|did|is)\\s+(?:that|this|it)\\s+(?:work|suit|do)\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_STANCE(py_str("(?:ok|okay|alright|all\\s+right|fine|happy|cool|easy|content)"));
    s__BARE_FOR_YOU(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:so\\s+)?(?:"), &g__BARE_STANCE()), &py_str("|work|works|suit|suits|square)\\s+(?:for|with|wiv|wid|by)\\s+(?:you|u|ya)\\s*\\??\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHECK(py_tuple(vec![g__A_CHECK_ASKED(), g__A_THING_FOR_YOU(), g__A_VERB_FOR_YOU(), g__A_VERB_YOU(), g__BARE_FOR_YOU()]));
    s_AGREED(py_tuple(vec![py_str("Fine by me - and it's your call, not mine."), py_str("That suits me. Whether it suits you is the part I can't answer."), py_str("No objection here. You're the one it has to work for, though."), py_str("Sounds alright to me. I've got nothing at stake in it either way."), py_str("Happy with that. It's your call at the end of it."), py_str("Works for me - just don't count me as the one who decided it."), py_str("That's fine as far as I'm concerned. What matters is whether it's fine by you.")]));
    s__NOT_A_CHECK(stdlib_call("re", "compile", &[py_str("\\b(?:drive|driving|tow|towing|insure|sign|signing|sue|arrest|publish|fly|flying|sail|carry|carrying|keep\\s+it|eat|drink)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn reads_as_a_check(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("True when the line is a check of a plan against the bot.");
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9'? ]"), py_str(" "), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&mcall(&g__NOT_A_CHECK(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_A_CHECK());
    for __item in __iter1 {
    shape = __item.clone();
    if py_truthy(&{ let __b = mcall(&shape.clone(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&shape.clone(), &g__BARE_FOR_YOU())); if !py_truthy(&__b) { __b } else { mcall(&shape.clone(), "search", &[mcall(&said.clone(), "strip", &[])]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    let mut choose: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "choose") { v.clone() }
    else { Py::None };
    let _ = py_str("The agreement line, or None when this is not a check-with-you question.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_check(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&choose.clone(), &Py::None))) {
    let mut random: Py = stdlib_module("random");
    choose = gattr(&random.clone(), "choice");
    }
    return py_call(&choose, &[g_AGREED()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.check_with_you".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGREED" => Some(g_AGREED()),
            "A_CHECK" => Some(g_A_CHECK()),
            "_A_CHECK_ASKED" => Some(g__A_CHECK_ASKED()),
            "_A_THING_FOR_YOU" => Some(g__A_THING_FOR_YOU()),
            "_A_VERB_FOR_YOU" => Some(g__A_VERB_FOR_YOU()),
            "_A_VERB_YOU" => Some(g__A_VERB_YOU()),
            "_BARE_FOR_YOU" => Some(g__BARE_FOR_YOU()),
            "_BARE_STANCE" => Some(g__BARE_STANCE()),
            "_NOT_A_CHECK" => Some(g__NOT_A_CHECK()),
            "_OBJECT" => Some(g__OBJECT()),
            "_STANCE" => Some(g__STANCE()),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reads_as_a_check" => Some(py_func("reads_as_a_check", |args: &[Py]| reads_as_a_check(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGREED" => s_AGREED(v),
            "A_CHECK" => s_A_CHECK(v),
            "_A_CHECK_ASKED" => s__A_CHECK_ASKED(v),
            "_A_THING_FOR_YOU" => s__A_THING_FOR_YOU(v),
            "_A_VERB_FOR_YOU" => s__A_VERB_FOR_YOU(v),
            "_A_VERB_YOU" => s__A_VERB_YOU(v),
            "_BARE_FOR_YOU" => s__BARE_FOR_YOU(v),
            "_BARE_STANCE" => s__BARE_STANCE(v),
            "_NOT_A_CHECK" => s__NOT_A_CHECK(v),
            "_OBJECT" => s__OBJECT(v),
            "_STANCE" => s__STANCE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.check_with_you", module_obj);
    pyrt::xrefs::reg_fn("compose.check_with_you", "reads_as_a_check", py_func("reads_as_a_check", |args: &[Py]| reads_as_a_check(args)));
    pyrt::xrefs::reg_fn("compose.check_with_you", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.check_with_you", "AGREED", g_AGREED, s_AGREED);
    pyrt::xrefs::reg_access("compose.check_with_you", "A_CHECK", g_A_CHECK, s_A_CHECK);
    pyrt::xrefs::reg_access("compose.check_with_you", "_A_CHECK_ASKED", g__A_CHECK_ASKED, s__A_CHECK_ASKED);
    pyrt::xrefs::reg_access("compose.check_with_you", "_A_THING_FOR_YOU", g__A_THING_FOR_YOU, s__A_THING_FOR_YOU);
    pyrt::xrefs::reg_access("compose.check_with_you", "_A_VERB_FOR_YOU", g__A_VERB_FOR_YOU, s__A_VERB_FOR_YOU);
    pyrt::xrefs::reg_access("compose.check_with_you", "_A_VERB_YOU", g__A_VERB_YOU, s__A_VERB_YOU);
    pyrt::xrefs::reg_access("compose.check_with_you", "_BARE_FOR_YOU", g__BARE_FOR_YOU, s__BARE_FOR_YOU);
    pyrt::xrefs::reg_access("compose.check_with_you", "_BARE_STANCE", g__BARE_STANCE, s__BARE_STANCE);
    pyrt::xrefs::reg_access("compose.check_with_you", "_NOT_A_CHECK", g__NOT_A_CHECK, s__NOT_A_CHECK);
    pyrt::xrefs::reg_access("compose.check_with_you", "_OBJECT", g__OBJECT, s__OBJECT);
    pyrt::xrefs::reg_access("compose.check_with_you", "_STANCE", g__STANCE, s__STANCE);
}

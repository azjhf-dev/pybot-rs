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


thread_local! { static G_ACHIEVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ACHIEVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MOST_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAILING_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACHIEVES() -> Py { ensure_init(); G_ACHIEVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACHIEVES(v: Py) { ensure_init(); G_ACHIEVES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_CLAUSE() -> Py { ensure_init(); G_A_NEW_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_CLAUSE(v: Py) { ensure_init(); G_A_NEW_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINS_ON() -> Py { ensure_init(); G_JOINS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINS_ON(v: Py) { ensure_init(); G_JOINS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ACHIEVING() -> Py { ensure_init(); G_NOT_ACHIEVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ACHIEVING(v: Py) { ensure_init(); G_NOT_ACHIEVING.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MOST_QUOTED() -> Py { ensure_init(); G_THE_MOST_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MOST_QUOTED(v: Py) { ensure_init(); G_THE_MOST_QUOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAILING_FILLER() -> Py { ensure_init(); G_TRAILING_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAILING_FILLER(v: Py) { ensure_init(); G_TRAILING_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_CHECK() -> Py { ensure_init(); G_WORK_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_CHECK(v: Py) { ensure_init(); G_WORK_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_IN_FRONT() -> Py { ensure_init(); G_YES_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_IN_FRONT(v: Py) { ensure_init(); G_YES_IN_FRONT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that asks whether the advice will work is answered with what the advice achieves.\n\n\"will that work\" names nothing of its own. After a how-to it is not a new question and not small\ntalk: it is the one thing a person asks before they spend an hour on someone else's method. On\nmain it was answered with a stock nod to the method (\"Done that way, it needs no one else: that is\nthe method I would use.\"), which says nothing about whether the method works:\n\n    how do i clean a burnt pan\n      Put an inch of water and two tablespoons of bicarbonate of soda in it and simmer for ten ...\n    will that work\n      Done that way, it needs no one else: that is the method I would use.\n\nThe card has already answered it, in its own words - \"almost nothing needs scrubbing after that\",\n\"the glass catches the rest in a day or two\", \"a post with height lets her do it properly\" - so the\nreply is that clause, with the yes the person asked for in front of it. Nothing is judged and\nnothing is added: a card with no such clause passes, and the line is answered as it was.\n\nNarrow on purpose:\n\n* the line has to be *only* the check - \"will that work\", \"does that help\", \"would that fix it\" -\n  with nothing of its own after it, because \"will that work for a dark stain too\" asks about a\n  stain;\n* the bot's last turn has to be a card it really said (`measure._the_card_said_last`, the same\n  check the measure follow-up uses), so a chat line is never quoted back as an answer;\n* the clause has to say the method *achieves* something, and a clause that says it achieves\n  nothing (\"rice does nothing except leave grit in the ports\") is not that.\n");
    s_WORK_CHECK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:will|would|does|do|might|should|can)\\s+(?:that|it|this|they|those)\\s+(?:work|help|do\\s+(?:it|the\\s+job)|fix\\s+(?:it|that)|be\\s+(?:enough|ok|okay|all\\s+right|alright|fine)|make\\s+a\\s+difference)(?:\\s+(?:for|on)\\s+(?:me|us|him|her|them|this|that))?(?:\\s+(?:then|though|now|too))?\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ACHIEVES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:almost nothing|nothing needs|nothing to scrub|does the job|does the trick|catches|clears|lifts|comes away|comes off|gets rid of|no more|beats|kills|sinks|survives|stops?|fixes|sorts|dries|settles|holds|works|helps?|lets (?:her|him|them|you)|keeps (?:them|it) (?:off|away|out))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ACHIEVING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:does nothing|nothing except|does not|doesn't|dont|don't|never|won't|wont|isn't|is not|not\\s+enough|makes it worse)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JOINS_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("which"), py_str("that"), py_str("who"), py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("because"), py_str("though"), py_str("while"), py_str("as"), py_str("if"), py_str("when"), py_str("also")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_YES_IN_FRONT(py_tuple(vec![py_str("Yes"), py_str("It does"), py_str("That does it"), py_str("Yes, it does")]));
    s_THE_MOST_QUOTED(Py::Int(11));
    s_TRAILING_FILLER(py_tuple(vec![py_str("too"), py_str("as well"), py_str("also"), py_str("then"), py_str("though")]));
    s_A_NEW_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("and so but or which is are was were then because when if that\nit they there this these with plus while as"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn asks_if_it_will_work(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is nothing but a check that the advice works.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORK_CHECK(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _the_clauses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The clauses of a sentence, in order: a card puts the method in one and its result in the\n    next (\"then lift the burnt layer with a wooden spoon: almost nothing needs scrubbing\").");
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;:]\\s+|\\s+-\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    Py::None
}
pub fn _without_filler(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut filler: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The clause without the words that only join it to the rest of the card (\"... help too\").");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_TRAILING_FILLER());
    for __item in __iter1 {
    filler = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str(" "), &filler.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, "endswith", args))(&__args) }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &op_sub(&op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(filler.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(1)), &Py::None), "rstrip", args))(&__args) };
    }
    }
    return said.clone();
    Py::None
}
pub fn _what_it_achieves(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut card: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: card"))) };
    let mut clause: Py = Py::None;
    let mut part: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The clause of the card that says what the method achieves, or \"\" when it says nothing.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(card.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ACHIEVES(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ACHIEVING(), "search", args))(&__args) } } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_clauses(args))(&__args) });
    for __item in __iter2 {
    clause = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ACHIEVES(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ACHIEVING(), "search", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_JOINS_ON(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) }))) {
    continue;
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_filler(args))(&__args) };
    if py_truthy(&{ let __cmp31: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_le(&Py::Int(4), &__cmp31)) { op_le(&__cmp31, &g_THE_MOST_QUOTED()) } else { Py::Bool(false) } }) {
    return clause.clone();
    }
    }
    }
    return py_str("");
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut at: Py = Py::None;
    let mut card: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The yes with the card's own account of why, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_if_it_will_work(args))(&__args) }))) {
    return Py::None;
    }
    card = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::measure::_the_card_said_last(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(card.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _what_it_achieves(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("yes"), py_str("no")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lstrip", args))(&__args) }, "lower", args))(&__args) }, "startswith", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::Int(2), &Py::None), "islower", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::Int(2), &Py::None), &py_str(" "))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::Int(2), &Py::Int(3), &Py::None), "islower", args))(&__args) } } } } } } }) {
    said = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&said.clone(), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "rstrip", args))(&__args) };
    at = op_modulo(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut letter: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(letter.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("ord", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_YES_IN_FRONT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return op_modulo(&py_str("%s — %s."), &py_tuple(vec![geti(&g_YES_IN_FRONT(), &at.clone()), said.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.will_it_work".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACHIEVES" => Some(g_ACHIEVES()),
            "A_NEW_CLAUSE" => Some(g_A_NEW_CLAUSE()),
            "JOINS_ON" => Some(g_JOINS_ON()),
            "NOT_ACHIEVING" => Some(g_NOT_ACHIEVING()),
            "SENTENCE" => Some(g_SENTENCE()),
            "THE_MOST_QUOTED" => Some(g_THE_MOST_QUOTED()),
            "TRAILING_FILLER" => Some(g_TRAILING_FILLER()),
            "WORK_CHECK" => Some(g_WORK_CHECK()),
            "YES_IN_FRONT" => Some(g_YES_IN_FRONT()),
            "_the_clauses" => Some(py_func("_the_clauses", |args: &[Py]| _the_clauses(args))),
            "_what_it_achieves" => Some(py_func("_what_it_achieves", |args: &[Py]| _what_it_achieves(args))),
            "_without_filler" => Some(py_func("_without_filler", |args: &[Py]| _without_filler(args))),
            "asks_if_it_will_work" => Some(py_func("asks_if_it_will_work", |args: &[Py]| asks_if_it_will_work(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACHIEVES" => s_ACHIEVES(v),
            "A_NEW_CLAUSE" => s_A_NEW_CLAUSE(v),
            "JOINS_ON" => s_JOINS_ON(v),
            "NOT_ACHIEVING" => s_NOT_ACHIEVING(v),
            "SENTENCE" => s_SENTENCE(v),
            "THE_MOST_QUOTED" => s_THE_MOST_QUOTED(v),
            "TRAILING_FILLER" => s_TRAILING_FILLER(v),
            "WORK_CHECK" => s_WORK_CHECK(v),
            "YES_IN_FRONT" => s_YES_IN_FRONT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.will_it_work", module_obj);
    pyrt::xrefs::reg_fn("memory.will_it_work", "asks_if_it_will_work", py_func("asks_if_it_will_work", |args: &[Py]| asks_if_it_will_work(args)));
    pyrt::xrefs::reg_fn("memory.will_it_work", "_the_clauses", py_func("_the_clauses", |args: &[Py]| _the_clauses(args)));
    pyrt::xrefs::reg_fn("memory.will_it_work", "_without_filler", py_func("_without_filler", |args: &[Py]| _without_filler(args)));
    pyrt::xrefs::reg_fn("memory.will_it_work", "_what_it_achieves", py_func("_what_it_achieves", |args: &[Py]| _what_it_achieves(args)));
    pyrt::xrefs::reg_fn("memory.will_it_work", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.will_it_work", "ACHIEVES", g_ACHIEVES, s_ACHIEVES);
    pyrt::xrefs::reg_access("memory.will_it_work", "A_NEW_CLAUSE", g_A_NEW_CLAUSE, s_A_NEW_CLAUSE);
    pyrt::xrefs::reg_access("memory.will_it_work", "JOINS_ON", g_JOINS_ON, s_JOINS_ON);
    pyrt::xrefs::reg_access("memory.will_it_work", "NOT_ACHIEVING", g_NOT_ACHIEVING, s_NOT_ACHIEVING);
    pyrt::xrefs::reg_access("memory.will_it_work", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.will_it_work", "THE_MOST_QUOTED", g_THE_MOST_QUOTED, s_THE_MOST_QUOTED);
    pyrt::xrefs::reg_access("memory.will_it_work", "TRAILING_FILLER", g_TRAILING_FILLER, s_TRAILING_FILLER);
    pyrt::xrefs::reg_access("memory.will_it_work", "WORK_CHECK", g_WORK_CHECK, s_WORK_CHECK);
    pyrt::xrefs::reg_access("memory.will_it_work", "YES_IN_FRONT", g_YES_IN_FRONT, s_YES_IN_FRONT);
}

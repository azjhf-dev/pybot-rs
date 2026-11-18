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


thread_local! { static G_A_BORROWING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_BALANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_BNPL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_CHEAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_NO_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_STORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BALANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BORROWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEBT_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEBT_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_GREETING_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JOINED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NOT_THIS_DEBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PAYMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THINKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BORROWING_WORDS() -> Py { ensure_init(); G_A_BORROWING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BORROWING_WORDS(v: Py) { ensure_init(); G_A_BORROWING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_BALANCE() -> Py { ensure_init(); G_A_DEBT_BALANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_BALANCE(v: Py) { ensure_init(); G_A_DEBT_BALANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_BNPL() -> Py { ensure_init(); G_A_DEBT_BNPL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_BNPL(v: Py) { ensure_init(); G_A_DEBT_BNPL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_CHEAP() -> Py { ensure_init(); G_A_DEBT_CHEAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_CHEAP(v: Py) { ensure_init(); G_A_DEBT_CHEAP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_NO_FIGURE() -> Py { ensure_init(); G_A_DEBT_NO_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_NO_FIGURE(v: Py) { ensure_init(); G_A_DEBT_NO_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_RATE() -> Py { ensure_init(); G_A_DEBT_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_RATE(v: Py) { ensure_init(); G_A_DEBT_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_STORE() -> Py { ensure_init(); G_A_DEBT_STORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_STORE(v: Py) { ensure_init(); G_A_DEBT_STORE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_ASK() -> Py { ensure_init(); G__A_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_ASK(v: Py) { ensure_init(); G__A_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BALANCE() -> Py { ensure_init(); G__A_BALANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BALANCE(v: Py) { ensure_init(); G__A_BALANCE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BORROWING() -> Py { ensure_init(); G__A_BORROWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BORROWING(v: Py) { ensure_init(); G__A_BORROWING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEBT_DONE() -> Py { ensure_init(); G__A_DEBT_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEBT_DONE(v: Py) { ensure_init(); G__A_DEBT_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEBT_NAMED() -> Py { ensure_init(); G__A_DEBT_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEBT_NAMED(v: Py) { ensure_init(); G__A_DEBT_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_GREETING_CARD() -> Py { ensure_init(); G__A_GREETING_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_GREETING_CARD(v: Py) { ensure_init(); G__A_GREETING_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HAVE() -> Py { ensure_init(); G__A_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HAVE(v: Py) { ensure_init(); G__A_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JOINED() -> Py { ensure_init(); G__A_JOINED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JOINED(v: Py) { ensure_init(); G__A_JOINED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NOT_THIS_DEBT() -> Py { ensure_init(); G__A_NOT_THIS_DEBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NOT_THIS_DEBT(v: Py) { ensure_init(); G__A_NOT_THIS_DEBT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PAYMENT() -> Py { ensure_init(); G__A_PAYMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PAYMENT(v: Py) { ensure_init(); G__A_PAYMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE() -> Py { ensure_init(); G__A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE(v: Py) { ensure_init(); G__A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THINKING() -> Py { ensure_init(); G__A_THINKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THINKING(v: Py) { ensure_init(); G__A_THINKING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A debt said flat: the rate on it, or the balance, answered with what it means.\n\nSomebody who tells the bot about a card, a loan or a debt is handing over the number that decides\neverything about their money, and the number is often the whole line:\n\n    ive got a credit card at 24 percent\n    ive got 5 grand on the card\n    weve got 2000 on the visa at 20 percent\n\nNone of those is a question, so no card in the knowledge base is reached - the debt card wants the\nquestion (\"should i pay off my card or save\") - and what answered them was the day pool (\"Sounds\nalright. Anything good in it?\", \"How was it, overall?\") or the store's receipt (\"Noted: you've got\na loan at 30 percent.\"). Neither is an answer: a comment on somebody's day says nothing about the\ndebt, and the receipt is the coldest line in the file.\n\nWhat the bot does know here it can say. A rate tells you whether the balance should be cleared\nbefore anything is saved - that is the reading the debt card makes, made on the number the person\nactually gave. A balance tells you the same about the order, and asks the one thing still missing\n(the rate, or whether there is more than one). Below the point where \"it costs more than saving\npays\" is true, the claim is not made at all: a cheap rate gets said to be cheap, and the question\nthat catches people out - when it runs out - is asked instead.\n\nThe module answers only the turn that says the debt. A question about it (\"should i pay it off\",\n\"what is the minimum\") is a card's, and the store's follow-up turns are the store's; this returns\nNone for both.\n");
    s_PRIORITY(op_neg(&Py::Int(500)));
    s__A_RATE(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2}(?:\\.\\d{1,2})?)\\s*(?:%|per\\s?cent|percent)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BORROWING(stdlib_call("re", "compile", &[py_str("\\b(?:credit\\s+card|store\\s+card|storecard|credit\\s+cards|payday\\s+loan|overdraft|debt|debts|catalogue|klarna|bnpl|buy\\s+now\\s+pay\\s+later|a\\s+loan|my\\s+loan|the\\s+loan|loan|loans|card|cards)\\b|\\b(?:my|the|on\\s+the|our|me)\\s+(?:visa|mastercard|amex|american\\s+express)\\b|\\b(?:visa|mastercard)\\s+card\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_JOINED(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|also|then|plus|or|what\\s+about|how\\s+about|same|still|anyway)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BALANCE(stdlib_call("re", "compile", &[py_str("£\\s?\\d[\\d,]*|\\b\\d[\\d,]{2,7}\\s?(?:pounds|quid|grand|k\\b)?|\\b\\d+\\s?(?:grand|k\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DEBT_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:paid\\s+off|paid\\s+up|paying\\s+off|cleared|clearing|settled|rid\\s+of|getting\\s+rid|sorted\\s+out|over\\s+with|done\\s+with)\\b|\\b(?:paid|pay|paying|settled)\\b[^.?!]{0,20}\\b(?:off|up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NOT_THIS_DEBT(stdlib_call("re", "compile", &[py_str("\\bstudent\\s+(?:loan|loans|debt|debts|finance|loan\\s+company)\\b|\\bplan\\s+[1245]\\b|\\bmaintenance\\s+loan\\b|\\bmortgage\\b|\\bmortgages\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PAYMENT(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,]{1,7}\\s?(?:pounds|quid|grand|k)?\\s*(?:a|per|each)\\s*(?:month|week|year|fortnight)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DEBT_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:credit\\s+cards?|store\\s*cards?|storecards?|overdraft|catalogue|klarna|bnpl|pay\\s?day\\s+loans?|buy\\s+now\\s+pay\\s+later|a\\s+loan|my\\s+loan|the\\s+loan|loans|debts?)\\b|\\b(?:my|the|our|on\\s+the|me)\\s+(?:visa|mastercard|amex|american\\s+express)\\b|\\b(?:visa|mastercard)\\s+card\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_GREETING_CARD(stdlib_call("re", "compile", &[py_str("\\b(?:birthday|christmas|xmas|thank[- ]?you|sympathy|get[- ]well|anniversary|leaving|wedding|new\\s+baby|congratulations|farewell)\\s+cards?\\b|\\bcards?\\s+(?:for|from|to)\\s+(?!my\\s+card|the\\s+card|pay)\\w+|\\b(?:bought|buy|buying|sent|send|sending|gave|give|giving|made|making|got|getting|picked\\s+up|picking\\s+up)\\b(?:\\s+(?!on\\b|off\\b)\\w+){0,3}\\s+(?:a|an|the|some)\\s+cards?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HAVE(stdlib_call("re", "compile", &[py_str("\\b(?:got|have|has|owe|owes|ive|weve|theyve|there'?s)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THINKING(stdlib_call("re", "compile", &[py_str("\\b(?:spend|spending|spent|overspend|stop|again|keep|shouldn'?t|worried|scared|hate|stuck|struggling)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_ASK(stdlib_call("re", "compile", &[py_str("^(?:should|shall|can|could|would|do|does|did|is|are|was|were|what|how|which|when|where|who|why|worth|better|best)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEBT_RATE(py_tuple(vec![py_str("That is an expensive rate: {rate}% a year is more than most savings accounts pay, so spare money does more going off the balance than into a saver. Roughly what is on it?"), py_str("{rate}% is dear money, more than most savers pay you, so clearing it first beats saving alongside it. How much is left on it?"), py_str("At {rate}% that is costing you every month it sits there, and it is more than most savings pay, so the balance comes first. How much is on it?")]));
    s_A_DEBT_BALANCE(py_tuple(vec![py_str("A balance like that is usually the first thing to clear: it costs more than most saving pays, so spare money is better off going to it than to a saver. Can you pay more than the minimum?"), py_str("That is the one to aim spare money at, since borrowing costs more than a saver pays you. What is the rate on it, and is it the only one?"), py_str("It is usually the first thing to clear, ahead of saving: what goes on the balance saves you more than the same money earns in a saver. Is it just the one balance?")]));
    s_A_DEBT_CHEAP(py_tuple(vec![py_str("At {rate}% that is cheap borrowing, so it is not the one to panic about; what matters is whether the rate is fixed or runs out, because a cheap rate that jumps is where these get away from people. Do you know when it ends?"), py_str("{rate}% is a low rate, so it is not costing you much; the thing to check is how long it lasts and what it becomes afterwards, which the terms you were given will say. Is it fixed for a set time?")]));
    s_A_DEBT_STORE(py_tuple(vec![py_str("Store cards and catalogues are usually the dearest borrowing in the shop: the rate is on the statement, and it is often twice what a credit card charges. How much is on it?"), py_str("Catalogue and store card rates are the ones worth checking first, because they are sold at the till rather than compared. What is the rate, and how much is left on it?")]));
    s_A_DEBT_BNPL(py_tuple(vec![py_str("Buy now pay later is free until the date it is not: what catches people is the deadline passing and the fee that lands with it. When is the payment due?"), py_str("The thing that matters with buy now pay later is the date the interest-free bit ends, and whether the repayments come out on time. Which one is it, and when?")]));
    s_A_DEBT_NO_FIGURE(py_tuple(vec![py_str("The rate is the figure that decides what to do about a borrowing - it is on the statement or the agreement - and after that what is left on it. Do you know either?"), py_str("What to do about borrowing turns on two things: what it costs a year, and how much is left on it. The rate is usually the one printed on the statement. Which have you got?")]));
    s_A_BORROWING_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:store\\s*cards?|storecards?|catalogue)\\b|\\b(?:bnpl|klarna|buy\\s+now\\s+pay\\s+later)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn observe(args: &[Py]) -> Py {
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
    let _ = py_str("Nothing to keep: the turn is read off the line itself, every time.");
    return Py::None;
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut percent: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The rate or the balance said back with what it means, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_BORROWING(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_DEBT_DONE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__A_ASK(), "match", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_RATE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_JOINED(), "match", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_NOT_THIS_DEBT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_GREETING_CARD(), "search", &[clean.clone()])) {
    return Py::None;
    }
    rate = mcall(&g__A_RATE(), "search", &[clean.clone()]);
    if py_truthy(&rate.clone()) {
    percent = mcall(&rate.clone(), "group", &[Py::Int(1)]);
    let __try1 = exc::catch(|| {
    value = call_builtin("float", &[percent.clone()]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&op_ge(&value.clone(), &Py::Float(8.0))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rate".to_string(), percent.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gen_compose::compose::pick(&[g_A_DEBT_RATE(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rate".to_string(), percent.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gen_compose::compose::pick(&[g_A_DEBT_CHEAP(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&mcall(&g__A_BALANCE(), "search", &[clean.clone()])) {
    if py_truthy(&mcall(&g__A_PAYMENT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    return gen_compose::compose::pick(&[g_A_DEBT_BALANCE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_DEBT_NAMED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_HAVE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_THINKING(), "search", &[clean.clone()]))) } } } }) {
    kind = mcall(&g_A_BORROWING_WORDS(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = kind.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&kind.clone(), "group", &[Py::Int(0)]), "startswith", &[py_tuple(vec![py_str("bnpl"), py_str("klarna"), py_str("buy")])]) } }) {
    return gen_compose::compose::pick(&[g_A_DEBT_BNPL(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&kind.clone()) {
    return gen_compose::compose::pick(&[g_A_DEBT_STORE(), memory.clone(), clean.clone()]);
    }
    return gen_compose::compose::pick(&[g_A_DEBT_NO_FIGURE(), memory.clone(), clean.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_debt_said_flat".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BORROWING_WORDS" => Some(g_A_BORROWING_WORDS()),
            "A_DEBT_BALANCE" => Some(g_A_DEBT_BALANCE()),
            "A_DEBT_BNPL" => Some(g_A_DEBT_BNPL()),
            "A_DEBT_CHEAP" => Some(g_A_DEBT_CHEAP()),
            "A_DEBT_NO_FIGURE" => Some(g_A_DEBT_NO_FIGURE()),
            "A_DEBT_RATE" => Some(g_A_DEBT_RATE()),
            "A_DEBT_STORE" => Some(g_A_DEBT_STORE()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_A_ASK" => Some(g__A_ASK()),
            "_A_BALANCE" => Some(g__A_BALANCE()),
            "_A_BORROWING" => Some(g__A_BORROWING()),
            "_A_DEBT_DONE" => Some(g__A_DEBT_DONE()),
            "_A_DEBT_NAMED" => Some(g__A_DEBT_NAMED()),
            "_A_GREETING_CARD" => Some(g__A_GREETING_CARD()),
            "_A_HAVE" => Some(g__A_HAVE()),
            "_A_JOINED" => Some(g__A_JOINED()),
            "_A_NOT_THIS_DEBT" => Some(g__A_NOT_THIS_DEBT()),
            "_A_PAYMENT" => Some(g__A_PAYMENT()),
            "_A_RATE" => Some(g__A_RATE()),
            "_A_THINKING" => Some(g__A_THINKING()),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BORROWING_WORDS" => s_A_BORROWING_WORDS(v),
            "A_DEBT_BALANCE" => s_A_DEBT_BALANCE(v),
            "A_DEBT_BNPL" => s_A_DEBT_BNPL(v),
            "A_DEBT_CHEAP" => s_A_DEBT_CHEAP(v),
            "A_DEBT_NO_FIGURE" => s_A_DEBT_NO_FIGURE(v),
            "A_DEBT_RATE" => s_A_DEBT_RATE(v),
            "A_DEBT_STORE" => s_A_DEBT_STORE(v),
            "PRIORITY" => s_PRIORITY(v),
            "_A_ASK" => s__A_ASK(v),
            "_A_BALANCE" => s__A_BALANCE(v),
            "_A_BORROWING" => s__A_BORROWING(v),
            "_A_DEBT_DONE" => s__A_DEBT_DONE(v),
            "_A_DEBT_NAMED" => s__A_DEBT_NAMED(v),
            "_A_GREETING_CARD" => s__A_GREETING_CARD(v),
            "_A_HAVE" => s__A_HAVE(v),
            "_A_JOINED" => s__A_JOINED(v),
            "_A_NOT_THIS_DEBT" => s__A_NOT_THIS_DEBT(v),
            "_A_PAYMENT" => s__A_PAYMENT(v),
            "_A_RATE" => s__A_RATE(v),
            "_A_THINKING" => s__A_THINKING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_debt_said_flat", module_obj);
    pyrt::xrefs::reg_fn("modules.a_debt_said_flat", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.a_debt_said_flat", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_BORROWING_WORDS", g_A_BORROWING_WORDS, s_A_BORROWING_WORDS);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_BALANCE", g_A_DEBT_BALANCE, s_A_DEBT_BALANCE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_BNPL", g_A_DEBT_BNPL, s_A_DEBT_BNPL);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_CHEAP", g_A_DEBT_CHEAP, s_A_DEBT_CHEAP);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_NO_FIGURE", g_A_DEBT_NO_FIGURE, s_A_DEBT_NO_FIGURE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_RATE", g_A_DEBT_RATE, s_A_DEBT_RATE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "A_DEBT_STORE", g_A_DEBT_STORE, s_A_DEBT_STORE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_ASK", g__A_ASK, s__A_ASK);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_BALANCE", g__A_BALANCE, s__A_BALANCE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_BORROWING", g__A_BORROWING, s__A_BORROWING);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_DEBT_DONE", g__A_DEBT_DONE, s__A_DEBT_DONE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_DEBT_NAMED", g__A_DEBT_NAMED, s__A_DEBT_NAMED);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_GREETING_CARD", g__A_GREETING_CARD, s__A_GREETING_CARD);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_HAVE", g__A_HAVE, s__A_HAVE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_JOINED", g__A_JOINED, s__A_JOINED);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_NOT_THIS_DEBT", g__A_NOT_THIS_DEBT, s__A_NOT_THIS_DEBT);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_PAYMENT", g__A_PAYMENT, s__A_PAYMENT);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_RATE", g__A_RATE, s__A_RATE);
    pyrt::xrefs::reg_access("modules.a_debt_said_flat", "_A_THINKING", g__A_THINKING, s__A_THINKING);
}

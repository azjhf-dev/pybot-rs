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


pub static DELIVERY_AGAIN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DELIVERY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DELIVERY_TROUBLE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIBRARY_CARD_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIBRARY_LATE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIBRARY_LOAN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIBRARY_LOST_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIORITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BARE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DELIVERY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DELIVERY_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIBRARY_CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIBRARY_LOAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_THE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _QUALIFIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STATE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TROUBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_DELIVERY_AGAIN_LINES() -> Py { ensure_init(); DELIVERY_AGAIN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DELIVERY_AGAIN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DELIVERY_AGAIN_LINES.write() { *w = v; } }
pub fn g_DELIVERY_LINES() -> Py { ensure_init(); DELIVERY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DELIVERY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DELIVERY_LINES.write() { *w = v; } }
pub fn g_DELIVERY_TROUBLE_LINES() -> Py { ensure_init(); DELIVERY_TROUBLE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DELIVERY_TROUBLE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DELIVERY_TROUBLE_LINES.write() { *w = v; } }
pub fn g_LIBRARY_CARD_LINES() -> Py { ensure_init(); LIBRARY_CARD_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIBRARY_CARD_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LIBRARY_CARD_LINES.write() { *w = v; } }
pub fn g_LIBRARY_LATE_LINES() -> Py { ensure_init(); LIBRARY_LATE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIBRARY_LATE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LIBRARY_LATE_LINES.write() { *w = v; } }
pub fn g_LIBRARY_LOAN_LINES() -> Py { ensure_init(); LIBRARY_LOAN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIBRARY_LOAN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LIBRARY_LOAN_LINES.write() { *w = v; } }
pub fn g_LIBRARY_LOST_LINES() -> Py { ensure_init(); LIBRARY_LOST_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIBRARY_LOST_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LIBRARY_LOST_LINES.write() { *w = v; } }
pub fn g_PRIORITY() -> Py { ensure_init(); PRIORITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); if let Ok(mut w) = PRIORITY.write() { *w = v; } }
pub fn g__AGAIN() -> Py { ensure_init(); _AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = _AGAIN.write() { *w = v; } }
pub fn g__ASK() -> Py { ensure_init(); _ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); if let Ok(mut w) = _ASK.write() { *w = v; } }
pub fn g__BARE_STATE() -> Py { ensure_init(); _BARE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BARE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = _BARE_STATE.write() { *w = v; } }
pub fn g__DELIVERY() -> Py { ensure_init(); _DELIVERY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DELIVERY(v: Py) { ensure_init(); if let Ok(mut w) = _DELIVERY.write() { *w = v; } }
pub fn g__DELIVERY_STATE() -> Py { ensure_init(); _DELIVERY_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DELIVERY_STATE(v: Py) { ensure_init(); if let Ok(mut w) = _DELIVERY_STATE.write() { *w = v; } }
pub fn g__LATE() -> Py { ensure_init(); _LATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LATE(v: Py) { ensure_init(); if let Ok(mut w) = _LATE.write() { *w = v; } }
pub fn g__LIBRARY_CARD() -> Py { ensure_init(); _LIBRARY_CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIBRARY_CARD(v: Py) { ensure_init(); if let Ok(mut w) = _LIBRARY_CARD.write() { *w = v; } }
pub fn g__LIBRARY_LOAN() -> Py { ensure_init(); _LIBRARY_LOAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIBRARY_LOAN(v: Py) { ensure_init(); if let Ok(mut w) = _LIBRARY_LOAN.write() { *w = v; } }
pub fn g__LOST() -> Py { ensure_init(); _LOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LOST(v: Py) { ensure_init(); if let Ok(mut w) = _LOST.write() { *w = v; } }
pub fn g__NOT_THE_STATE() -> Py { ensure_init(); _NOT_THE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_THE_STATE.write() { *w = v; } }
pub fn g__QUALIFIED() -> Py { ensure_init(); _QUALIFIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__QUALIFIED(v: Py) { ensure_init(); if let Ok(mut w) = _QUALIFIED.write() { *w = v; } }
pub fn g__RECORD() -> Py { ensure_init(); _RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RECORD(v: Py) { ensure_init(); if let Ok(mut w) = _RECORD.write() { *w = v; } }
pub fn g__STATE_AFTER() -> Py { ensure_init(); _STATE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STATE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = _STATE_AFTER.write() { *w = v; } }
pub fn g__TROUBLE() -> Py { ensure_init(); _TROUBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TROUBLE(v: Py) { ensure_init(); if let Ok(mut w) = _TROUBLE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A thing of theirs with a date on it is the holder's to answer, not a search.\n\n\"Will my parcel come before friday\" and \"is my library book due back\" name a record the person\nowns and a date only the holder can see. The fallback used to send them to a search, and the\nplace pools could read the parcel as \"my account\" and answer about a bank balance, or read the\nlibrary card as a venue and answer about opening hours. Both are the wrong subject.\n\nSo this module answers only a line that names one of those records and asks about its state or\ndate, and it says the two things a friend can: I cannot see it from here, and this is where it\nlives. Everything else, including \"how do i return my order\" (a procedure) and \"my order of the\nwords\" (not a record at all), passes.\n");
    s_PRIORITY(neg(&Py::Int(500)));
    s__RECORD(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:[a-z']+\\s+){0,2}(?P<thing>parcels?|orders?|deliver(?:y|ies)|tracking|couriers?|dispatch|library\\s+cards?|library\\s+books?|library\\s+loans?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__QUALIFIED(stdlib_call("re", "compile", &[py_str("\\s+(?:of|that|which)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:so|and|but|ok|okay|also|hi|hello|morning|quick one)?\\s*,?\\s*(?:will|is|are|was|were|has|have|did|does|do|can|could|when|where|what|how|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__STATE_AFTER(stdlib_call("re", "compile", &[py_str("\\b(?:(?:still|never|now|already)\\s+)?(?:is|are|was|were|has|have|had|did|does|do)(?:n't|nt)?\\s+(?:not\\s+|yet\\s+)?(?:[a-z]+\\s+){0,2}(?:late|overdue|due|back|out|renewed|expired|dispatched|delivered|arrived|arriving|coming|come|came|delayed|lost|missing|stuck|held|waiting|ready|moved|on\\s+its\\s+way|turned\\s+up|shown\\s+up|meant\\s+to\\s+(?:arrive|come)|supposed\\s+to\\s+(?:arrive|come)|expected)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:do|can|could|should)\\s+i|how\\s+to|should\\s+i|do\\s+i\\s+need|return|refund|cancel|change|complain|worth|warranty|guarantee|recycle)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DELIVERY_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:arrive|arrives|arrived|arriving|come|comes|coming|came|get\\s+here|be\\s+here|turn\\s+up|turned\\s+up|show\\s+up|shown\\s+up|delivered|delivery|dispatch|dispatched|tracking|track|check|overdue|late|delayed|lost|missing|stuck|held|waiting|ready|moved|on\\s+its\\s+way|expired|where|when|before|by|due|today|tomorrow|tonight|this\\s+week|next\\s+week|mon(?:day)?|tues(?:day)?|wed(?:nesday)?|thurs(?:day)?|fri(?:day)?|sat(?:urday)?|sun(?:day)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIBRARY_CARD(py_tuple(vec![py_str("library card"), py_str("library cards")]));
    s__LIBRARY_LOAN(py_tuple(vec![py_str("library book"), py_str("library books"), py_str("library loan"), py_str("library loans")]));
    s__DELIVERY(py_tuple(vec![py_str("parcel"), py_str("parcels"), py_str("order"), py_str("orders"), py_str("delivery"), py_str("deliveries"), py_str("tracking"), py_str("courier"), py_str("couriers"), py_str("dispatch")]));
    s_LIBRARY_CARD_LINES(py_tuple(vec![py_str("Your library account has the expiry date: sign in with the card number and PIN, or ring the branch and they can renew the card in a minute - usually free if nothing is owed on it."), py_str("That lives with the library rather than anywhere I can see. The card number gets you into your account, which shows when it runs out, and the desk can renew it on the spot.")]));
    s_LIBRARY_LOAN_LINES(py_tuple(vec![py_str("Your library account has the due date: sign in with the card number and PIN to see the loans and renew them, and the desk or the number on the card can do it in a minute."), py_str("That one lives in your library account rather than here. The card number shows what is out, when it is due and how many renewals are left; the branch can renew it by phone.")]));
    s_LIBRARY_LATE_LINES(py_tuple(vec![py_str("Ugh, the late ones are a pain. Renew it in your library account first: the card number and PIN do it if nothing is reserved, and that stops the clock. If it is already overdue, take it back and the desk will tell you what is owed - I can't see the fine from here."), py_str("Late books happen. The account with the card number and PIN is where the renewal is, if the book is not reserved. Once it is overdue the desk has the amount and can take it back; I can't see either from here.")]));
    s__LATE(stdlib_call("re", "compile", &[py_str("\\b(?:late|overdue)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:never|not)\\s+(?:arrived|came|come|turned\\s+up|shown\\s+up|moved)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LOST(stdlib_call("re", "compile", &[py_str("\\b(?:lost|stolen)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIBRARY_LOST_LINES(py_tuple(vec![py_str("Tell the library as soon as you can: the desk or the number on their site can cancel the card or mark the book lost, and tell you what a replacement costs. I can't see the charge from here."), py_str("That is the library's to sort, not something I can see: ring them or call at the desk with your card number, and they will cancel it or mark the book lost and say what is owed.")]));
    s_DELIVERY_LINES(py_tuple(vec![py_str("The tracking link in the confirmation email or the shop's app has the last scan, and if it has not moved for a day or two it is the shop to chase - they open the case with the courier, not you. I can't see the tracking from here."), py_str("The tracking link in the order email shows where a parcel is and the day it is due, and if that day has gone the shop chases the courier for you; no window on it from this end.")]));
    s_DELIVERY_TROUBLE_LINES(py_tuple(vec![py_str("That's a pain. The tracking link in the order email has the last scan and the day it was due; if that day has gone or it has not moved for a day or two, the shop chases the courier - they open the case, not you."), py_str("Sorry - that is annoying. The order email's tracking shows where it got to; once the promised day has gone, the shop can chase the courier or refund and replace it, so ask them rather than the courier.")]));
    s_DELIVERY_AGAIN_LINES(py_tuple(vec![py_str("Again? That's a pain. The tracking in the order email has the last scan; if it has not moved for a day or two, the shop is the one to chase the courier, and they can refund or replace it."), py_str("Not again - sorry. Check the tracking in the order email first, then ask the shop to chase the courier: they open the case and can refund or replace it, and the courier cannot do that for you.")]));
    s__TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:late|delayed|lost|missing|stuck|held|still|never|overdue)\\b|\\bhas(?:n't|nt)\\b|\\bnot\\s+moved\\b|\\b(?:meant|supposed)\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AGAIN(stdlib_call("re", "compile", &[py_str("\\bagain\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_record(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The person's own record the line names, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g__RECORD(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__QUALIFIED(), "match", &[clean.clone(), mcall(&match_.clone(), "end", &[])])))) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "lower", &[]), "split", &[])]);
    }
    }
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
    let mut record: Py = Py::None;
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__STATE_AFTER(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__BARE_STATE(), "search", &[clean.clone()]) } } } })) } }) {
    return Py::None;
    }
    record = _the_record(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&record.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__NOT_THE_STATE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__LIBRARY_CARD(), &record.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__LIBRARY_LOAN(), &record.clone())) } }) {
    if py_truthy(&mcall(&g__LOST(), "search", &[clean.clone()])) {
    return _pick(&[g_LIBRARY_LOST_LINES(), memory.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g__LIBRARY_CARD(), &record.clone()))) {
    return _pick(&[g_LIBRARY_CARD_LINES(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__LIBRARY_LOAN(), &record.clone()))) {
    if py_truthy(&mcall(&g__LATE(), "search", &[clean.clone()])) {
    return _pick(&[g_LIBRARY_LATE_LINES(), memory.clone()]);
    }
    return _pick(&[g_LIBRARY_LOAN_LINES(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__DELIVERY(), &record.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__DELIVERY_STATE(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__TROUBLE(), "search", &[clean.clone()])) {
    if py_truthy(&mcall(&g__AGAIN(), "search", &[clean.clone()])) {
    return _pick(&[g_DELIVERY_AGAIN_LINES(), memory.clone()]);
    }
    return _pick(&[g_DELIVERY_TROUBLE_LINES(), memory.clone()]);
    }
    return _pick(&[g_DELIVERY_LINES(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("Two wordings, so a second record ask does not repeat the first reply word for word.");
    let __try1 = exc::catch(|| {
    last = call_builtin("str", &[{ let __b = mcall(&geti(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![py_dict(vec![])]) } }, &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError", "IndexError", "TypeError"]) {
                __handled5 = true;
    last = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&lines.clone());
    for __item in __iter6 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {
    return line.clone();
    }
    }
    return geti(&lines.clone(), &Py::Int(0));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.record_due".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DELIVERY_AGAIN_LINES" => Some(g_DELIVERY_AGAIN_LINES()),
            "DELIVERY_LINES" => Some(g_DELIVERY_LINES()),
            "DELIVERY_TROUBLE_LINES" => Some(g_DELIVERY_TROUBLE_LINES()),
            "LIBRARY_CARD_LINES" => Some(g_LIBRARY_CARD_LINES()),
            "LIBRARY_LATE_LINES" => Some(g_LIBRARY_LATE_LINES()),
            "LIBRARY_LOAN_LINES" => Some(g_LIBRARY_LOAN_LINES()),
            "LIBRARY_LOST_LINES" => Some(g_LIBRARY_LOST_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_AGAIN" => Some(g__AGAIN()),
            "_ASK" => Some(g__ASK()),
            "_BARE_STATE" => Some(g__BARE_STATE()),
            "_DELIVERY" => Some(g__DELIVERY()),
            "_DELIVERY_STATE" => Some(g__DELIVERY_STATE()),
            "_LATE" => Some(g__LATE()),
            "_LIBRARY_CARD" => Some(g__LIBRARY_CARD()),
            "_LIBRARY_LOAN" => Some(g__LIBRARY_LOAN()),
            "_LOST" => Some(g__LOST()),
            "_NOT_THE_STATE" => Some(g__NOT_THE_STATE()),
            "_QUALIFIED" => Some(g__QUALIFIED()),
            "_RECORD" => Some(g__RECORD()),
            "_STATE_AFTER" => Some(g__STATE_AFTER()),
            "_TROUBLE" => Some(g__TROUBLE()),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_the_record" => Some(py_func("_the_record", |args: &[Py]| _the_record(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: None,
    }))
}

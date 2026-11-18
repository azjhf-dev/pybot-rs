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


thread_local! { static G_BILL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_YET_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARRIVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUTGOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPELLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_letter_about_money: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_BILL_LINES() -> Py { ensure_init(); G_BILL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BILL_LINES(v: Py) { ensure_init(); G_BILL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTER_LINES() -> Py { ensure_init(); G_LETTER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTER_LINES(v: Py) { ensure_init(); G_LETTER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_YET_LINES() -> Py { ensure_init(); G_NOT_YET_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_YET_LINES(v: Py) { ensure_init(); G_NOT_YET_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT() -> Py { ensure_init(); G__AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT(v: Py) { ensure_init(); G__AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__ARRIVED() -> Py { ensure_init(); G__ARRIVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARRIVED(v: Py) { ensure_init(); G__ARRIVED.with(|c| *c.borrow_mut() = v); }
pub fn g__DATE() -> Py { ensure_init(); G__DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DATE(v: Py) { ensure_init(); G__DATE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY() -> Py { ensure_init(); G__MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); G__MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_YET() -> Py { ensure_init(); G__NOT_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_YET(v: Py) { ensure_init(); G__NOT_YET.with(|c| *c.borrow_mut() = v); }
pub fn g__OUTGOING() -> Py { ensure_init(); G__OUTGOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUTGOING(v: Py) { ensure_init(); G__OUTGOING.with(|c| *c.borrow_mut() = v); }
pub fn g__SPELLINGS() -> Py { ensure_init(); G__SPELLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPELLINGS(v: Py) { ensure_init(); G__SPELLINGS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A letter about money is answered by what is on the page, not by guessing at it.\n\n\"i got a letter about a benefit overpayment\" and \"i got a bill i wasnt expecting\" name a piece\nof post the person has and the bot cannot see. The knowledge base has cards for some of these\nsubjects, but the ones it does not know fell to the greeting - \"Go on then - what was the answer?\"\nto a letter nobody had asked about, or \"Noted: you got a bill you weren't expecting.\" That is the\nwrong subject: the person needs to know what to do with the letter.\n\nSo this module answers only a line that says a letter, notice or bill has arrived and names\nmoney on it, and it says the three things a friend can: what to find on the page, to ask for the\nbreakdown and answer in writing, and to keep a copy. A card that knows the subject sits above\nthis one and keeps the turn; a letter about anything else is not this answer.\n\nThe mirror line - \"the bill hasnt arrived\", \"my benefit letter never arrived\" - is the same\nsubject and the opposite state, and the arrival reader matches its verb. It gets its own short\nanswer (check the sender's account or phone line, confirm the address, note the date), because\ntelling somebody what is on a page they do not have is the confident wrong answer this module\nexists to avoid.\n");
    s_PRIORITY(op_neg(&Py::Int(600)));
    s__ARRIVED(stdlib_call("re", "compile", &[py_str("\\b(?:got|received|receive|had|have(?!\\s+to))\\b[^.!?]{0,30}\\b(?:a|an|this|the|my|our)\\s+(?:[a-z'-]+\\s+){0,2}(?P<thing>letter|notice|bill)\\b|\\b(?:was|were|been|be|got)\\s+sent\\b[^.!?]{0,20}\\b(?:a|an|this|the|my|our)\\s+(?:[a-z'-]+\\s+){0,2}(?P<thing4>letter|notice|bill)\\b|\\bsent\\s+(?:me|us)\\b[^.!?]{0,20}\\b(?:a|an|this|the|my|our)\\s+(?:[a-z'-]+\\s+){0,2}(?P<thing5>letter|notice|bill)\\b|\\b(?:a|an|my|our|the|this)\\s+(?:[a-z'-]+\\s+){0,2}(?P<thing2>letter|notice|bill)\\b[^.!?]{0,25}\\b(?:says?|said|about|from|saying|came|arrived|turned\\s+up)\\b|\\b(?:a|an|my|our|the|this)\\s+(?:[a-z'-]+\\s+){0,2}(?P<thing3>letter|notice|bill)\\b[^.!?]{0,20}\\b(?:came|arrived|turned\\s+up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_YET(stdlib_call("re", "compile", &[py_str("\\b(?:hasn'?t|hasnt|haven'?t|havent|hadn'?t|hadnt|didn'?t|didnt|never|not|still\\s+no|no)\\b[^.!?]{0,30}\\b(?:arrived|arrive|come|came|turned\\s+up|turned\\s+up|show(?:ed|n)?\\s+up|been\\s+sent|receive|received|got|get|had)\\b|\\b(?:still\\s+)?waiting\\s+(?:for|on)\\b|\\bno\\s+(?:[a-z'-]+\\s+){0,3}(?:letter|notice|bill)\\b[^.!?]{0,20}\\byet\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OUTGOING(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:sent|send|sending|posted|post|posting|wrote|write|writing|need|needs|want|wants|asked|asking)\\b|\\b(?:have|had|got|need|needs|want|wants)\\s+to\\s+(?:write|send|post|get|receive)\\b|\\b(?:can|could|may|might|should|would|will)\\s+(?:i|we)\\s+(?:get|have|receive|request|ask)\\b|\\b(?:could|can|would|will)\\s+you\\s+(?:send|post|write|get)\\b|\\b(?:writing|sending|posting)\\s+(?:a|an|the|my|our)\\s+(?:letter|notice|bill)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SPELLINGS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\brecieved\\b|\\brecived\\b|\\breceved\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("received")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bbenifits?\\b|\\bbenfits?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("benefits")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\blett(?:re|r|ar|e)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("letter")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\buniveral\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("universal")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\boverpaymant\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("overpayment")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhousng\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("housing")])]));
    s__MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:overpay(?:ment|ments)?|overpaid|benefits?|universal\\s+credit|tax\\s+credits?|housing\\s+benefit|child\\s+benefit|dwp|hmrc|tax|taxes|council\\s+tax|arrears?|debt|debts|owe|owed|owes|repay|repayment|money|amount|charge|charges|bill|bills|rent|rents|mortgage|insurance|loan|loans|credit\\s+card|pension|pip|dla|esa|jsa|income\\s+support|student\\s+loan|fine|fines)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_LINES(py_tuple(vec![py_str("I can't read the letter from here, but the answer is on the page: the date they want a reply by, the amount and the period it covers, and the reference. Ask for the breakdown in writing, answer in writing before that date, and keep a copy; if it is a decision, the letter says how to challenge it and by when. Tell me the amount and the dates on it and I'll help you check it."), py_str("That letter has three things worth finding before anything else: the date they want an answer by, the amount and the period it covers, and the reference. Ask for the breakdown in writing and put your answer in writing too, keeping a copy. If it is a decision you can challenge, the deadline is on the letter. What does it say the amount is?")]));
    s_BILL_LINES(py_tuple(vec![py_str("A bill is worth checking before you pay it: the dates it covers, what it says you are being charged for, and the reference. Ask for the breakdown in writing, and if it is wrong say so in writing before the date on it; keep a copy. Tell me the amount and the dates on it and I'll help you check it."), py_str("Before paying a bill you did not expect, check the dates it covers, what it is charging for, and the reference. Ask for the breakdown in writing, and answer in writing before the date on it, keeping a copy. What is it for, and how much?")]));
    s_NOT_YET_LINES(py_tuple(vec![py_str("If it should have arrived, the sender's own account page or phone line is where to check it was sent and to which address - ask them to confirm the date it went out, and note who you spoke to and when. When it comes, the reply-by date and the reference are the two things to look for first."), py_str("Before it arrives, check the sender has your current address: their account page or a phone call can confirm when it was sent and to where. Keep a note of the call. When it comes, the reply-by date and the reference are the two things to note first.")]));
    s__AMOUNT(stdlib_call("re", "compile", &[py_str("(?:£\\s*\\d|\\b\\d+(?:[.,]\\d+)?\\s*(?:pounds?|quid|p|pc|per\\s?cent|%)\\b|\\b(?:for|of|cost(?:s|ing)?|paid|owe[sd]?)\\s+£?\\d+\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__DATE(stdlib_call("re", "compile", &[py_str("\\b(?:today|yesterday|tomorrow|this\\s+(?:week|month|year)|last\\s+(?:week|month|year)|next\\s+(?:week|month|year)|(?:january|february|march|april|may|june|july|august|september|october|november|december)|(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|\\d{1,2}[/-]\\d{1,2}(?:[/-]\\d{2,4})?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _ask_for_what_is_missing(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut has_amount: Py = Py::None;
    let mut has_date: Py = Py::None;
    let _ = py_str("Point the question at the detail the line does not already carry.\n\n    The first reply asks for both the amount and the dates; a person who has just written\n    \"my bill came to 80 pounds\" has answered half of that. Asking it back reads as if the bot\n    did not listen, and the useful thing left is the period, the reference or what it is for.\n    ");
    has_amount = call_builtin("bool", &[mcall(&g__AMOUNT(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    has_date = call_builtin("bool", &[mcall(&g__DATE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&has_amount.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&has_date.clone()))) {
    return line.clone();
    }
    return mcall(&line.clone(), "replace", &[py_str("Tell me the amount and the dates on it and I'll help you check it."), py_str("Tell me the amount on it and I'll help you check it.")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&has_date.clone()))) {
    return mcall(&mcall(&line.clone(), "replace", &[py_str("Tell me the amount and the dates on it and I'll help you check it."), py_str("Tell me the dates it covers and I'll help you check it.")]), "replace", &[py_str("What is it for, and how much?"), py_str("What is it for, and what dates does it cover?")]);
    }
    return mcall(&mcall(&mcall(&line.clone(), "replace", &[py_str("Tell me the amount and the dates on it and I'll help you check it."), py_str("Tell me the reference and I'll help you check it.")]), "replace", &[py_str("What does it say the amount is?"), py_str("What is the reference on it?")]), "replace", &[py_str("What is it for, and how much?"), py_str("What is it for, and what is the reference?")]);
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut right: Py = Py::None;
    let mut thing: Py = Py::None;
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__SPELLINGS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    right = __unpk2[1].clone();
    clean = mcall(&pattern.clone(), "sub", &[right.clone(), clean.clone()]);
    }
    match_ = mcall(&g__ARRIVED(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&match_.clone(), &Py::None))) {
    if py_truthy(&{ let __b = mcall(&g__NOT_YET(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__MONEY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\b(?:letter|notice|bill)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__OUTGOING(), "search", &[clean.clone()]))) } } } } } }) {
    return _pick(&[g_NOT_YET_LINES(), memory.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&g__OUTGOING(), "search", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)])) {
    return Py::None;
    }
    thing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing2")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing3")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing4")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing5")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } } } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__MONEY(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__NOT_YET(), "search", &[clean.clone()])) {
    return _pick(&[g_NOT_YET_LINES(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&thing.clone(), &py_str("bill")))) {
    return _ask_for_what_is_missing(&[_pick(&[g_BILL_LINES(), memory.clone()]), clean.clone(), thing.clone()]);
    }
    return _ask_for_what_is_missing(&[_pick(&[g_LETTER_LINES(), memory.clone()]), clean.clone(), thing.clone()]);
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
    let _ = py_str("Two wordings, so a second letter ask does not repeat the first reply word for word.");
    let __try1 = exc::catch(|| {
    last = call_builtin("str", &[{ let __b = mcall(&geti(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![py_dict(vec![])]) } }, &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
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
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_letter_about_money".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BILL_LINES" => Some(g_BILL_LINES()),
            "LETTER_LINES" => Some(g_LETTER_LINES()),
            "NOT_YET_LINES" => Some(g_NOT_YET_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_AMOUNT" => Some(g__AMOUNT()),
            "_ARRIVED" => Some(g__ARRIVED()),
            "_DATE" => Some(g__DATE()),
            "_MONEY" => Some(g__MONEY()),
            "_NOT_YET" => Some(g__NOT_YET()),
            "_OUTGOING" => Some(g__OUTGOING()),
            "_SPELLINGS" => Some(g__SPELLINGS()),
            "_ask_for_what_is_missing" => Some(py_func("_ask_for_what_is_missing", |args: &[Py]| _ask_for_what_is_missing(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BILL_LINES" => s_BILL_LINES(v),
            "LETTER_LINES" => s_LETTER_LINES(v),
            "NOT_YET_LINES" => s_NOT_YET_LINES(v),
            "PRIORITY" => s_PRIORITY(v),
            "_AMOUNT" => s__AMOUNT(v),
            "_ARRIVED" => s__ARRIVED(v),
            "_DATE" => s__DATE(v),
            "_MONEY" => s__MONEY(v),
            "_NOT_YET" => s__NOT_YET(v),
            "_OUTGOING" => s__OUTGOING(v),
            "_SPELLINGS" => s__SPELLINGS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_letter_about_money", module_obj);
    pyrt::xrefs::reg_fn("modules.a_letter_about_money", "_ask_for_what_is_missing", py_func("_ask_for_what_is_missing", |args: &[Py]| _ask_for_what_is_missing(args)));
    pyrt::xrefs::reg_fn("modules.a_letter_about_money", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.a_letter_about_money", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "BILL_LINES", g_BILL_LINES, s_BILL_LINES);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "LETTER_LINES", g_LETTER_LINES, s_LETTER_LINES);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "NOT_YET_LINES", g_NOT_YET_LINES, s_NOT_YET_LINES);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_AMOUNT", g__AMOUNT, s__AMOUNT);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_ARRIVED", g__ARRIVED, s__ARRIVED);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_DATE", g__DATE, s__DATE);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_MONEY", g__MONEY, s__MONEY);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_NOT_YET", g__NOT_YET, s__NOT_YET);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_OUTGOING", g__OUTGOING, s__OUTGOING);
    pyrt::xrefs::reg_access("modules.a_letter_about_money", "_SPELLINGS", g__SPELLINGS, s__SPELLINGS);
}

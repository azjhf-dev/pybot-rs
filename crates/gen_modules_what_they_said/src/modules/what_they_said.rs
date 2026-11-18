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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THIRD_PARTY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_UNIT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPORTING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_SPOKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.what_they_said: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_KIND() -> Py { ensure_init(); G__A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_KIND(v: Py) { ensure_init(); G__A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONEY() -> Py { ensure_init(); G__A_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONEY(v: Py) { ensure_init(); G__A_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THIRD_PARTY_ASK() -> Py { ensure_init(); G__A_THIRD_PARTY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THIRD_PARTY_ASK(v: Py) { ensure_init(); G__A_THIRD_PARTY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_UNIT_AFTER() -> Py { ensure_init(); G__A_UNIT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_UNIT_AFTER(v: Py) { ensure_init(); G__A_UNIT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DETAIL() -> Py { ensure_init(); G__NOT_A_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DETAIL(v: Py) { ensure_init(); G__NOT_A_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__QUESTION() -> Py { ensure_init(); G__QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUESTION(v: Py) { ensure_init(); G__QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__REPORTING_VERB() -> Py { ensure_init(); G__REPORTING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPORTING_VERB(v: Py) { ensure_init(); G__REPORTING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_SPOKE() -> Py { ensure_init(); G__SOMEBODY_SPOKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_SPOKE(v: Py) { ensure_init(); G__SOMEBODY_SPOKE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PERSON() -> Py { ensure_init(); G__THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PERSON(v: Py) { ensure_init(); G__THE_PERSON.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What somebody else said is a thing the person told the bot, and the chat says it back.\n\n    we got quoted 400 for the fence\n    what did they quote me                    before  I'm not sure about that one, sorry - a\n                                                      quick search online will get you an answer\n                                              now     You said 400.\n\n    i told the garage about the noise and they said bring it in tuesday\n    what did the garage say                   before  a card about getting the price in writing\n                                              now     You said bring it in tuesday.\n\n    i need a new boiler, british gas said 3 grand\n    what did they say it would cost           before  the whole boiler card\n                                              now     You said 3 grand.\n\nThe person is not asking about the world in any of these. They are asking for something they said\nearlier in this chat, and the reply is in the chat: the line where somebody else was quoted, and,\nwhen the ask names a figure, the figure out of that line.\n\n`bot/memory/lineback.py` answers the recall asks that are about the person's own facts (\"what did\ni say my budget was\"). This module is the same reading one step out - what a builder, a garage, a\nvet or a child said - and it is here rather than in that module because the ask names no fact of\nthe person's own to look up: what it needs is the last line where the person quoted somebody.\n\nThe two things that were missing everywhere, and are the whole of the change:\n\n- **A pronoun is not a detail.** Read as content, \"they\" in \"what did they quote me\" was the one\n  word the ask had to share with a line, so it matched nothing and the turn went to a search\n  hedge. The people words and the modals (\"would\", \"could\", \"will\", ...) are scaffolding in these\n  asks in the same way \"i\" and \"my\" are, and come off before anything is matched.\n- **A quote is a price.** \"what did they quote me\" names the kind of answer it wants - a figure -\n  in the words people use for a builder's figure, so the figure is said back rather than the whole\n  line.\n\nIt answers only these asks and passes on everything else: a line with no earlier turn behind it,\na line that names no speaker and no kind of answer, and a chat where the last thing said was a\nquestion rather than somebody being quoted all keep the turn they had.\n");
    s_PRIORITY(Py::Int(7060));
    s__A_THIRD_PARTY_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+did\\s+(?:they|he|she|them|it|the\\s+[a-z]+|(?!(?:i|we|you|my|our)\\b)[a-z]+)\\s+(?:say|said|tell\\s+(?:you|me)|told\\s+(?:you|me)|quote|quoted|charge|charged|offer|offered|reckon|reckoned|suggest|suggested|text|texted|come\\s+back\\s+with|wants?)\\b|\\bwhat\\s+(?:did|have|has)\\s+(?:they|he|she)\\s+(?:said|told\\s+(?:you|me))\\b|\\bremind\\s+me\\s+what\\s+(?:they|he|she)\\s+(?:said|say|told\\s+you|quoted|wanted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_DETAIL(call_builtin("frozenset", &[mcall(&py_str("\nwhat which who whom whose when where why how say said saying says tell told telling mention\nmentioned quote quoted quotes charge charged offer offered reckon suggested text texted remind\nme my our we us i you your it its that this these those was were is are be been being did do does\ndone about of to the a an and or for with in on at by as if so but not any some please just really\nthing things one ones kind sort name called exact exactly already again\nthey them their theirs he him his she her hers\nwould could should shall will may might must can cant cannot itll id\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_dict(vec![(py_str("cost"), py_str("figure")), (py_str("costs"), py_str("figure")), (py_str("price"), py_str("figure")), (py_str("quote"), py_str("figure")), (py_str("quoted"), py_str("figure")), (py_str("charge"), py_str("figure")), (py_str("charged"), py_str("figure")), (py_str("offer"), py_str("figure")), (py_str("offered"), py_str("figure")), (py_str("pay"), py_str("figure")), (py_str("paid"), py_str("figure")), (py_str("much"), py_str("figure")), (py_str("many"), py_str("figure")), (py_str("number"), py_str("figure"))]) }
    s__A_KIND(__data1());
    s__SOMEBODY_SPOKE(stdlib_call("re", "compile", &[py_str("\\b(?!i\\b|we\\b)(?:they|he|she|the\\s+[a-z]+|[a-z]+)\\s+(?:said|says|say|told|tells|quoted|asked|charges?|offer(?:s|ed)?|reckons|thinks|texted|called|wants)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FIGURE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)\\s?\\d[\\d,.]*\\s?(?:k\\b|m\\b)?|\\b\\d[\\d,.]*\\s?(?:k\\b|grand\\b|thousand\\b|quid\\b|pounds?\\b|p\\b)|\\b\\d[\\d,.]*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|which|who|when|where|why|how|should|shall|can|could|would|do|does|did|is|are|was|were|will|have|has|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__REPORTING_VERB(stdlib_call("re", "compile", &[py_str("\\b(?P<who>(?:[a-z]+\\s+)?[a-z]+)\\s+(?:said|says|telling\\s+me|told\\s+me|tells\\s+me|reckoned|texted|quoted|charged|offered|asked|wants|wanted|asks|needs|went)\\b\\s+(?:that\\s+)?(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("me"), py_str("us"), py_str("im"), py_str("ive"), py_str("id")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MONEY(stdlib_call("re", "compile", &[py_str("[£$€]|\\b\\d[\\d,.]*\\s?(?:k|m|grand|thousand|quid|pounds?|p)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_UNIT_AFTER(stdlib_call("re", "compile", &[py_str("^\\s*(?:years?|yrs?|months?|weeks?|days?|hours?|hrs?|minutes?|mins?|miles?|km|kilometres?|kilograms?|kg|stone|feet|foot|inches|times?|percent|%|o'?clock)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _details(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The content words of a line, as flat words: what it says that is not scaffolding.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__NOT_A_DETAIL(), &mcall(&word.clone(), "strip", &[py_str("'")]))); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&word.clone(), "strip", &[py_str("'")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {  __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } } py_set(__out) })() };
    Py::None
}
pub fn _the_kind_of_answer(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kind of answer the ask names - \"figure\" for a price - or \"\" when it names none.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    kind = mcall(&g__A_KIND(), "get", &[mcall(&word.clone(), "strip", &[py_str("'")])]);
    if py_truthy(&kind.clone()) {
    return kind.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_last_somebody_spoke(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The last earlier line where the person quoted somebody, or None.");
    wanted = _details(&[said.clone()]);
    kind = _the_kind_of_answer(&[said.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory.lineback", "the_lines", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = mcall(&g__QUESTION(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.lineback", "is_a_recall_ask", &[line.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SOMEBODY_SPOKE(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = kind.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&wanted.clone())); if py_truthy(&__b) { __b } else { op_bitand(&wanted.clone(), &_details(&[line.clone()])) } } } }) {
    return line.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_reported_clause(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What somebody else said, when the line hands it on in a clause, or \"\".\n\n    Only a verb whose subject is somebody other than the person reports somebody else's words:\n    \"i told my boss i was quitting\" and \"i asked for a raise and got told no\" say what the person\n    did, not what the boss said, and the clause after them is the person's own line.\n    ");
    clause = py_str("");
    let __iter1 = py_iter_or_raise(&mcall(&g__REPORTING_VERB(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    who = mcall(&geti(&mcall(&mcall(&match_.clone(), "group", &[py_str("who")]), "split", &[]), &op_neg(&Py::Int(1))), "strip", &[py_str("',")]);
    if py_truthy(&Py::Bool(py_contains(&g__THE_PERSON(), &who.clone()))) {
    continue;
    }
    clause = mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "strip", &[]);
    }
    clause = mcall(&clause.clone(), "rstrip", &[py_str(" .!?,")]);
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:for|to|about|of|at)\\s+(?=(?:£|\\$|€)?\\s?\\d)")); __args.push(py_str("")); __args.push(clause.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = mcall(&clause.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__QUESTION(), "search", &[clause.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[a-z0-9]"), clause.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    return clause.clone();
    Py::None
}
pub fn _the_figure_the_line_gave(args: &[Py]) -> Py {
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
    let mut figures: Py = Py::None;
    let mut found: Py = Py::None;
    let mut money: Py = Py::None;
    let _ = py_str("The figure in the line that answers an ask for a price, or \"\".\n\n    A bare number is a price only when it is the only figure in the line: \"we got quoted 400 for\n    the fence\" is one, and \"the car is 3 years old and they said bring it in tuesday\" is an age.\n    Where the ask names a price and the line holds more than one figure, the money one is the\n    answer, and a line with two of them says nothing for certain.\n    ");
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__A_FIGURE(), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_UNIT_AFTER(), "match", &[slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])))) {  __out.push(found.clone()); } } py_list(__out) })() };
    money = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figures.clone()) { let mut found: Py = __item.clone(); if py_truthy(&mcall(&g__A_MONEY(), "search", &[mcall(&found.clone(), "group", &[Py::Int(0)])])) {  __out.push(found.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[money.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return mcall(&mcall(&geti(&money.clone(), &Py::Int(0)), "group", &[Py::Int(0)]), "strip", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&money.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    return mcall(&mcall(&geti(&figures.clone(), &Py::Int(0)), "group", &[Py::Int(0)]), "strip", &[]);
    }
    return py_str("");
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
    let mut clause: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    said = pyrt::xrefs::xcall("memory.lineback", "flat", &[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_THIRD_PARTY_ASK(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    line = _the_last_somebody_spoke(&[said.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&_the_kind_of_answer(&[said.clone()]), &py_str("figure")))) {
    figure = _the_figure_the_line_gave(&[line.clone()]);
    if py_truthy(&figure.clone()) {
    return op_add(&op_add(&py_str("You said "), &figure.clone()), &py_str("."));
    }
    }
    clause = _the_reported_clause(&[line.clone()]);
    if py_truthy(&clause.clone()) {
    return op_add(&op_add(&py_str("You said "), &clause.clone()), &py_str("."));
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.what_they_said".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_KIND" => Some(g__A_KIND()),
            "_A_MONEY" => Some(g__A_MONEY()),
            "_A_THIRD_PARTY_ASK" => Some(g__A_THIRD_PARTY_ASK()),
            "_A_UNIT_AFTER" => Some(g__A_UNIT_AFTER()),
            "_NOT_A_DETAIL" => Some(g__NOT_A_DETAIL()),
            "_QUESTION" => Some(g__QUESTION()),
            "_REPORTING_VERB" => Some(g__REPORTING_VERB()),
            "_SOMEBODY_SPOKE" => Some(g__SOMEBODY_SPOKE()),
            "_THE_PERSON" => Some(g__THE_PERSON()),
            "_details" => Some(py_func("_details", |args: &[Py]| _details(args))),
            "_the_figure_the_line_gave" => Some(py_func("_the_figure_the_line_gave", |args: &[Py]| _the_figure_the_line_gave(args))),
            "_the_kind_of_answer" => Some(py_func("_the_kind_of_answer", |args: &[Py]| _the_kind_of_answer(args))),
            "_the_last_somebody_spoke" => Some(py_func("_the_last_somebody_spoke", |args: &[Py]| _the_last_somebody_spoke(args))),
            "_the_reported_clause" => Some(py_func("_the_reported_clause", |args: &[Py]| _the_reported_clause(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "lineback" => Some(pyrt::xrefs::xmodule("memory.lineback")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_KIND" => s__A_KIND(v),
            "_A_MONEY" => s__A_MONEY(v),
            "_A_THIRD_PARTY_ASK" => s__A_THIRD_PARTY_ASK(v),
            "_A_UNIT_AFTER" => s__A_UNIT_AFTER(v),
            "_NOT_A_DETAIL" => s__NOT_A_DETAIL(v),
            "_QUESTION" => s__QUESTION(v),
            "_REPORTING_VERB" => s__REPORTING_VERB(v),
            "_SOMEBODY_SPOKE" => s__SOMEBODY_SPOKE(v),
            "_THE_PERSON" => s__THE_PERSON(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.what_they_said", module_obj);
    pyrt::xrefs::reg_fn("modules.what_they_said", "_details", py_func("_details", |args: &[Py]| _details(args)));
    pyrt::xrefs::reg_fn("modules.what_they_said", "_the_kind_of_answer", py_func("_the_kind_of_answer", |args: &[Py]| _the_kind_of_answer(args)));
    pyrt::xrefs::reg_fn("modules.what_they_said", "_the_last_somebody_spoke", py_func("_the_last_somebody_spoke", |args: &[Py]| _the_last_somebody_spoke(args)));
    pyrt::xrefs::reg_fn("modules.what_they_said", "_the_reported_clause", py_func("_the_reported_clause", |args: &[Py]| _the_reported_clause(args)));
    pyrt::xrefs::reg_fn("modules.what_they_said", "_the_figure_the_line_gave", py_func("_the_figure_the_line_gave", |args: &[Py]| _the_figure_the_line_gave(args)));
    pyrt::xrefs::reg_fn("modules.what_they_said", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.what_they_said", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.what_they_said", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("modules.what_they_said", "_A_KIND", g__A_KIND, s__A_KIND);
    pyrt::xrefs::reg_access("modules.what_they_said", "_A_MONEY", g__A_MONEY, s__A_MONEY);
    pyrt::xrefs::reg_access("modules.what_they_said", "_A_THIRD_PARTY_ASK", g__A_THIRD_PARTY_ASK, s__A_THIRD_PARTY_ASK);
    pyrt::xrefs::reg_access("modules.what_they_said", "_A_UNIT_AFTER", g__A_UNIT_AFTER, s__A_UNIT_AFTER);
    pyrt::xrefs::reg_access("modules.what_they_said", "_NOT_A_DETAIL", g__NOT_A_DETAIL, s__NOT_A_DETAIL);
    pyrt::xrefs::reg_access("modules.what_they_said", "_QUESTION", g__QUESTION, s__QUESTION);
    pyrt::xrefs::reg_access("modules.what_they_said", "_REPORTING_VERB", g__REPORTING_VERB, s__REPORTING_VERB);
    pyrt::xrefs::reg_access("modules.what_they_said", "_SOMEBODY_SPOKE", g__SOMEBODY_SPOKE, s__SOMEBODY_SPOKE);
    pyrt::xrefs::reg_access("modules.what_they_said", "_THE_PERSON", g__THE_PERSON, s__THE_PERSON);
}

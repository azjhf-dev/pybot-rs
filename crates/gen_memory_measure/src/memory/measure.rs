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


thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONTAINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_THEIR_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOPS_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_NOT_WORTH_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.measure: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONTAINER() -> Py { ensure_init(); G_A_CONTAINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONTAINER(v: Py) { ensure_init(); G_A_CONTAINER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_CLAUSE() -> Py { ensure_init(); G_A_NEW_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_CLAUSE(v: Py) { ensure_init(); G_A_NEW_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_THEIR_TURN() -> Py { ensure_init(); G_KEEP_THEIR_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_THEIR_TURN(v: Py) { ensure_init(); G_KEEP_THEIR_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURES() -> Py { ensure_init(); G_MEASURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURES(v: Py) { ensure_init(); G_MEASURES.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_STOPS_THE_THING() -> Py { ensure_init(); G_STOPS_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOPS_THE_THING(v: Py) { ensure_init(); G_STOPS_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_AFTER() -> Py { ensure_init(); G_THING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_AFTER(v: Py) { ensure_init(); G_THING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_NOT_WORTH_SAYING() -> Py { ensure_init(); G_THING_NOT_WORTH_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_NOT_WORTH_SAYING(v: Py) { ensure_init(); G_THING_NOT_WORTH_SAYING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that asks the measure again: \"and how much\", \"and how often\", \"and how long\".\n\nNobody says the subject twice. The person asked how much water to drink, the answer gave six to\neight glasses, and then they asked \"and how much\" - and nothing in those three words says water,\nso the knowledge base read them as words in their own right and answered from a different entry:\n\n    how much water should i drink a day\n      Six to eight glasses a day, about 1.5 to 2 litres, is plenty, and more in heat or after ...\n    and how much\n      The UK limit is 14 units a week for men and women ... (the alcohol card, reached by \"drink\")\n\nThe same shape took \"and how often\" after a dog-walking answer to the puppy paragraph, and after\na roast-chicken answer to \"Same answer, and it covers this: About 20 minutes per 500g ...\". The\nfigure is already in the answer above, so the follow-up is answered from that answer: the\nsentence of it that carries the measure the person asked for, in the answer's own words.\n\nNarrow on purpose:\n\n* the line has to be *only* the measure - \"and how much\", \"how often again\", \"and how long is\n  that\" - with no subject of its own, because \"how much water for a puppy\" asks about a puppy;\n* the answer above has to be a card the bot really said on the turn before (the same check\n  `bot/memory/topics.entry_answer` makes), so a chat line is never quoted back as an answer;\n* the sentence handed back has to carry the measure that was asked for, or the module passes and\n  the line is answered exactly as it was.\n");
    s_MEASURES(py_dict(vec![(py_str("much"), stdlib_call("re", "compile", &[py_str("\\b\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("many"), stdlib_call("re", "compile", &[py_str("\\b\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|dozen)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("often"), stdlib_call("re", "compile", &[py_str("\\b(?:once|twice|thrice|every|each\\s+(?:day|week|month|morning|night)|daily|weekly|monthly|hourly|times\\s+a|a\\s+day|a\\s+week|a\\s+month|\\d\\s*(?:-|to)?\\s*\\d*\\s*times)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("long"), stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:second|minute|hour|day|week|month|year)s?\\b|\\b\\d+\\s*(?:-|to)\\s*\\d+\\s*(?:second|minute|hour|day|week|month|year)s?\\b|\\bovernight\\b|\\ball\\s+(?:day|night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("far"), stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:mile|km|kilometre|kilometer|metre|meter|yard|foot|feet)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("hot"), stdlib_call("re", "compile", &[py_str("\\b\\d{2,3}\\s*(?:\\u00b0|c\\b|f\\b|degrees?)|\\bgas\\s*mark\\s*\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("deep"), stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:cm|centimetre|centimeter|inch|in|mm|metre|meter)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("big"), stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:cm|centimetre|centimeter|inch|in|mm|litre|liter|gram|kilo)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
    s_LEAD(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|ok|okay|but|then|right|yeah|yes|well|oh|er|um|hmm)\\b[\\s,]*)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL(stdlib_call("re", "compile", &[py_str("[\\s,]*(?:(?:again|then|exactly|please|thanks|though|for\\s+that|for\\s+one|for\\s+each|per\\s+(?:day|week|person|one)|roughly|about|or\\s+so|is\\s+that|was\\s+that|would\\s+that\\s+be|does\\s+that|do\\s+i|did\\s+i|would\\s+i|should\\s+i|will\\s+i|of\\s+that|there)\\b[\\s,]*)*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(much|many|often|long|far|hot|deep|big)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_THEIR_TURN(call_builtin("frozenset", &[py_tuple(vec![py_str("loss"), py_str("grief"), py_str("safety"), py_str("safety_cards"), py_str("safety_phrases")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEW_CLAUSE(call_builtin("frozenset", &[mcall(&py_str("and so but or which is are was were then because when if that\nit they there this these with plus while as"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_AFTER(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many|often|long|far|hot|deep|big)\\s+(?:of\\s+)?(?P<thing>[a-z][a-z\\\\s'&/-]{0,28})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STOPS_THE_THING(call_builtin("frozenset", &[mcall(&py_str("should do can could would shall will did does is are was were\nto for before when if after and or but in on at with by the a an my your his her their our\nit them that this i we you they"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_NOT_WORTH_SAYING(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("there"), py_str("one")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONTAINER(stdlib_call("re", "compile", &[py_str("\\b(?:glasses|glass|cups|cup|pints|pint|slices|slice|portions|portion|pieces|piece|sachets|sachet|scoops|scoop|cloves|clove|tins|tin|cans|can|teaspoons|teaspoon|tablespoons|tablespoon|sprays|spray|doses|dose|wipes|wipe|sheets|sheet)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_measure_asked(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Which measure the line asks for, or \"\" when the line is not only a measure.\n\n    \"and how much\" is the ask; \"and how much is that in litres\" carries words of its own and is a\n    question the person asked in full, so it is left to the module that can answer it.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    text = mcall(&g_LEAD(), "sub", &[py_str(""), text.clone()]);
    text = mcall(&g_TAIL(), "sub", &[py_str(""), text.clone()]);
    text = mcall(&text.clone(), "strip", &[py_str(" ?.!,:;-")]);
    text = mcall(&mcall(&g_LEAD(), "sub", &[py_str(""), text.clone()]), "strip", &[py_str(" ?.!,:;-")]);
    match_ = mcall(&g_ASKED(), "fullmatch", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    return mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    Py::None
}
pub fn _the_card_said_last(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut record: Py = Py::None;
    let mut records: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The card the bot answered with on the turn before, or \"\" when the last turn was not one.");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_KEEP_THEIR_TURN(), &call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    records = mcall(&memory.clone(), "get", &[py_str("entry_answers")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[records.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&records.clone())) } }) {
    records = py_list(vec![mcall(&memory.clone(), "get", &[py_str("entry_answer")])]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[records.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    continue;
    }
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&text.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(8), &Py::None)]);
    if py_truthy(&{ let __b = mcall(&said.clone(), "startswith", &[opening.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&text.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(5), &Py::None)]))) } }) {
    return text.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_clause_that_carries(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { Py::None };
    let mut boundary: Py = Py::None;
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The first clause of the sentence that carries the measure, in the sentence's own words.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("[,;:]\\s+"), sentence.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    boundary = __item.clone();
    rest = slice_or_raise(&sentence.clone(), &mcall(&boundary.clone(), "end", &[]), &Py::None, &Py::None);
    first = mcall(&mcall(&geti(&{ let __b = stdlib_call("re", "split", &[py_str("\\s+"), mcall(&rest.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "strip", &[py_str(",.;:")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_A_NEW_CLAUSE(), &first.clone()))) {
    said = mcall(&slice_or_raise(&sentence.clone(), &Py::None, &mcall(&boundary.clone(), "start", &[]), &Py::None), "strip", &[]);
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return said.clone();
    }
    }
    }
    return sentence.clone();
    Py::None
}
pub fn _the_sentence_with(args: &[Py]) -> Py {
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
    let mut measure: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "measure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: measure"))) };
    let mut part: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let _ = py_str("The clause of an answer that carries the measure asked for, or \"\" when none does.");
    pattern = mcall(&g_MEASURES(), "get", &[measure.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pattern.clone()))) {
    return py_str("");
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[text.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[sentence.clone()])) {
    return _the_clause_that_carries(&[sentence.clone(), pattern.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_thing_the_line_named(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The short phrase the earlier line put straight after its measure word, or \"\".\n\n    Only when it is one to three words with no verb in it, so \"how much water\" gives \"water\" and\n    \"how long do i roast a chicken for\" gives nothing.\n    ");
    said = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    match_ = mcall(&g_THING_AFTER(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+"), mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    kept = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_STOPS_THE_THING(), &mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(",.;:")])))) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[mcall(&word.clone(), "strip", &[py_str(",.;:")])]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    break;
    }
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[kept.clone()]), "strip", &[py_str(" ,.;:-")]);
    return if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THING_NOT_WORTH_SAYING(), &mcall(&thing.clone(), "lower", &[]))) } }) { thing.clone() } else { py_str("") };
    Py::None
}
pub fn _with_the_thing(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The sentence with the thing the line named set in front of its first container, or \"\".");
    found = mcall(&g_A_CONTAINER(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    head = slice_or_raise(&sentence.clone(), &Py::None, &mcall(&found.clone(), "end", &[]), &Py::None);
    tail = slice_or_raise(&sentence.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    return op_modulo(&py_str("%s of %s%s"), &py_tuple(vec![head.clone(), thing.clone(), tail.clone()]));
    Py::None
}
pub fn _as_a_headline(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The thing as a one-word headline, then the answer in lower case where it reads as prose.");
    first = geti(&sentence.clone(), &Py::Int(0));
    rest = slice_or_raise(&sentence.clone(), &Py::Int(1), &Py::None, &Py::None);
    if py_truthy(&{ let __b = mcall(&first.clone(), "isupper", &[]); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&rest.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[]) } }) {
    sentence = op_add(&mcall(&first.clone(), "lower", &[]), &rest.clone());
    }
    return op_modulo(&py_str("%s: %s"), &py_tuple(vec![op_add(&mcall(&geti(&thing.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)), sentence.clone()]));
    Py::None
}
pub fn _as_a_sentence(args: &[Py]) -> Py {
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
    let _ = py_str("The clause as the one sentence the reply is: a card's clause loses its full stop when it is\n    cut out of a longer sentence, and a reply line that ends on \"about once a week\" reads as if the\n    bot were cut off.");
    if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str(".!?"), &geti(&text.clone(), &op_neg(&Py::Int(1))))) } }) {
    return op_add(&text.clone(), &py_str("."));
    }
    return text.clone();
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
    let mut card: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The clause of the answer above that answers the measure asked again, or None.");
    measure = the_measure_asked(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&measure.clone()))) {
    return Py::None;
    }
    card = _the_card_said_last(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    sentence = _the_sentence_with(&[card.clone(), measure.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    return Py::None;
    }
    thing = _the_thing_the_line_named(&[memory.clone()]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&sentence.clone(), "lower", &[]), &mcall(&thing.clone(), "lower", &[]))) } }) {
    said = _with_the_thing(&[sentence.clone(), thing.clone()]);
    if py_truthy(&said.clone()) {
    return _as_a_sentence(&[said.clone()]);
    }
    return _as_a_sentence(&[_as_a_headline(&[thing.clone(), sentence.clone()])]);
    }
    return _as_a_sentence(&[sentence.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.measure".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKED" => Some(g_ASKED()),
            "A_CONTAINER" => Some(g_A_CONTAINER()),
            "A_NEW_CLAUSE" => Some(g_A_NEW_CLAUSE()),
            "KEEP_THEIR_TURN" => Some(g_KEEP_THEIR_TURN()),
            "LEAD" => Some(g_LEAD()),
            "MEASURES" => Some(g_MEASURES()),
            "SENTENCE" => Some(g_SENTENCE()),
            "STOPS_THE_THING" => Some(g_STOPS_THE_THING()),
            "TAIL" => Some(g_TAIL()),
            "THING_AFTER" => Some(g_THING_AFTER()),
            "THING_NOT_WORTH_SAYING" => Some(g_THING_NOT_WORTH_SAYING()),
            "_as_a_headline" => Some(py_func("_as_a_headline", |args: &[Py]| _as_a_headline(args))),
            "_as_a_sentence" => Some(py_func("_as_a_sentence", |args: &[Py]| _as_a_sentence(args))),
            "_the_card_said_last" => Some(py_func("_the_card_said_last", |args: &[Py]| _the_card_said_last(args))),
            "_the_clause_that_carries" => Some(py_func("_the_clause_that_carries", |args: &[Py]| _the_clause_that_carries(args))),
            "_the_sentence_with" => Some(py_func("_the_sentence_with", |args: &[Py]| _the_sentence_with(args))),
            "_the_thing_the_line_named" => Some(py_func("_the_thing_the_line_named", |args: &[Py]| _the_thing_the_line_named(args))),
            "_with_the_thing" => Some(py_func("_with_the_thing", |args: &[Py]| _with_the_thing(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_measure_asked" => Some(py_func("the_measure_asked", |args: &[Py]| the_measure_asked(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKED" => s_ASKED(v),
            "A_CONTAINER" => s_A_CONTAINER(v),
            "A_NEW_CLAUSE" => s_A_NEW_CLAUSE(v),
            "KEEP_THEIR_TURN" => s_KEEP_THEIR_TURN(v),
            "LEAD" => s_LEAD(v),
            "MEASURES" => s_MEASURES(v),
            "SENTENCE" => s_SENTENCE(v),
            "STOPS_THE_THING" => s_STOPS_THE_THING(v),
            "TAIL" => s_TAIL(v),
            "THING_AFTER" => s_THING_AFTER(v),
            "THING_NOT_WORTH_SAYING" => s_THING_NOT_WORTH_SAYING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.measure", module_obj);
    pyrt::xrefs::reg_fn("memory.measure", "the_measure_asked", py_func("the_measure_asked", |args: &[Py]| the_measure_asked(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_the_card_said_last", py_func("_the_card_said_last", |args: &[Py]| _the_card_said_last(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_the_clause_that_carries", py_func("_the_clause_that_carries", |args: &[Py]| _the_clause_that_carries(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_the_sentence_with", py_func("_the_sentence_with", |args: &[Py]| _the_sentence_with(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_the_thing_the_line_named", py_func("_the_thing_the_line_named", |args: &[Py]| _the_thing_the_line_named(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_with_the_thing", py_func("_with_the_thing", |args: &[Py]| _with_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_as_a_headline", py_func("_as_a_headline", |args: &[Py]| _as_a_headline(args)));
    pyrt::xrefs::reg_fn("memory.measure", "_as_a_sentence", py_func("_as_a_sentence", |args: &[Py]| _as_a_sentence(args)));
    pyrt::xrefs::reg_fn("memory.measure", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.measure", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("memory.measure", "A_CONTAINER", g_A_CONTAINER, s_A_CONTAINER);
    pyrt::xrefs::reg_access("memory.measure", "A_NEW_CLAUSE", g_A_NEW_CLAUSE, s_A_NEW_CLAUSE);
    pyrt::xrefs::reg_access("memory.measure", "KEEP_THEIR_TURN", g_KEEP_THEIR_TURN, s_KEEP_THEIR_TURN);
    pyrt::xrefs::reg_access("memory.measure", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.measure", "MEASURES", g_MEASURES, s_MEASURES);
    pyrt::xrefs::reg_access("memory.measure", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.measure", "STOPS_THE_THING", g_STOPS_THE_THING, s_STOPS_THE_THING);
    pyrt::xrefs::reg_access("memory.measure", "TAIL", g_TAIL, s_TAIL);
    pyrt::xrefs::reg_access("memory.measure", "THING_AFTER", g_THING_AFTER, s_THING_AFTER);
    pyrt::xrefs::reg_access("memory.measure", "THING_NOT_WORTH_SAYING", g_THING_NOT_WORTH_SAYING, s_THING_NOT_WORTH_SAYING);
}

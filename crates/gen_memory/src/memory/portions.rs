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


thread_local! { static G_AN_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONFIRMATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_OF_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_AND_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PORTION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_AFTER_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_EDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEDS_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NEW_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NUMBER_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_UNIT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORDS_FOR_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_AMOUNT() -> Py { ensure_init(); G_AN_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AMOUNT(v: Py) { ensure_init(); G_AN_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_COUNT() -> Py { ensure_init(); G_A_BARE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_COUNT(v: Py) { ensure_init(); G_A_BARE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONFIRMATION() -> Py { ensure_init(); G_A_CONFIRMATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONFIRMATION(v: Py) { ensure_init(); G_A_CONFIRMATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_OF_PEOPLE() -> Py { ensure_init(); G_A_COUNT_OF_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_OF_PEOPLE(v: Py) { ensure_init(); G_A_COUNT_OF_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_AND_UNIT() -> Py { ensure_init(); G_A_FIGURE_AND_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_AND_UNIT(v: Py) { ensure_init(); G_A_FIGURE_AND_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PORTION_ASK() -> Py { ensure_init(); G_A_PORTION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PORTION_ASK(v: Py) { ensure_init(); G_A_PORTION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_AFTER_A_COUNT() -> Py { ensure_init(); G_A_TIME_AFTER_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_AFTER_A_COUNT(v: Py) { ensure_init(); G_A_TIME_AFTER_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_EDGE() -> Py { ensure_init(); G_CLAUSE_EDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_EDGE(v: Py) { ensure_init(); G_CLAUSE_EDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEDS_A_COUNT() -> Py { ensure_init(); G_FEEDS_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEDS_A_COUNT(v: Py) { ensure_init(); G_FEEDS_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_A_COUNT() -> Py { ensure_init(); G_FOR_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_A_COUNT(v: Py) { ensure_init(); G_FOR_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_PER_PERSON() -> Py { ensure_init(); G_PER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PER_PERSON(v: Py) { ensure_init(); G_PER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NEW_COUNTS() -> Py { ensure_init(); G__A_NEW_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NEW_COUNTS(v: Py) { ensure_init(); G__A_NEW_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NUMBER_ANYWHERE() -> Py { ensure_init(); G__A_NUMBER_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NUMBER_ANYWHERE(v: Py) { ensure_init(); G__A_NUMBER_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_UNIT_AFTER() -> Py { ensure_init(); G__A_UNIT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_UNIT_AFTER(v: Py) { ensure_init(); G__A_UNIT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT() -> Py { ensure_init(); G__COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT(v: Py) { ensure_init(); G__COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORD() -> Py { ensure_init(); G__NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORD(v: Py) { ensure_init(); G__NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__WORDS_FOR_NUMBERS() -> Py { ensure_init(); G__WORDS_FOR_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORDS_FOR_NUMBERS(v: Py) { ensure_init(); G__WORDS_FOR_NUMBERS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up asking how many the answer in hand feeds is answered by that answer's own count.\n\n    how much pasta for 4 people\n      \"About 100 g of dry pasta each, so 400 g for four.\"\n    how many does that serve\n      \"I'm not sure about that one, sorry — I'd rather not guess. A quick search online...\"\n\n    how much potato salad for a party\n      \"About 250 g of potatoes each.\"\n    how many does that feed\n      \"Cheap party food people actually eat: a big pot of chilli or a pasta bake, garlic bread...\"\n\nBoth lines are about the answer on the screen: the first answer already says who it is for (\"for\nfour\") and the second only says what it is for per person. Nothing read either, so the first fell\nto a search hedge and the second to the party-food card, which is a different question.\n\nWhat this reads is only the counting half of a portion answer, and it says one of two things:\n\n* the answer named the number it works out to (\"so 400 g for four\", \"feeds 6\", \"serves four\"), and\n  that number is the answer to the line;\n* the answer gives an amount per person and no total (\"250 g of potatoes each\"), so the line has\n  no answer yet: the total depends on how many are coming, and the reply asks for that - and the\n  count they answer with is worked out from the figure the answer gave (\"about 5 kg for twenty\"),\n  because a promise to do the sums has to be kept on the next line or it is worse than no promise.\n\nIt is narrow on purpose, and returns None for everything else:\n\n* the line has to ask how many the thing in hand feeds or serves, in the words people use for it\n  (\"how many does that feed\", \"how many people does that serve\") - \"how many do i need\" is a\n  shopping question and belongs to the modules that weigh a shop;\n* the answer in hand has to be a portion answer: an amount in grams, kilos, millilitres or cups,\n  or a per-person figure. An answer about the number of days a thing keeps, or the number of\n  people at a party, is not a portion, and neither is a line about a recipe nobody has asked\n  about yet;\n* a count the answer gives for something else is not read as the portion: \"for 4 minutes\" is a\n  time, so a figure followed by a unit of time is not the count the answer works out.\n");
    s__COUNT(py_str("(?:\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|dozen|a dozen)"));
    s_A_PORTION_ASK(stdlib_call("re", "compile", &[py_str("\\bhow many\\b[^.?!]{0,30}?\\b(?:does|do|would|will|can)\\s+(?:that|it|this)\\s+\\w+|\\bhow many people (?:does|do|will|would)\\s+(?:that|it|this)\\b|\\b(?:that|it|this)\\s+(?:feeds|serves|makes|does|will do)\\s+(?:how many|how much)\\b|\\bhow many(?: people| mouths)?\\s+(?:is|are|will|would)\\s+(?:that|it|this)\\b|\\bhow many(?: people| mouths)?\\s+(?:is|are)\\s+(?:that|it|this)\\s+\\w+|\\b(?:that|it|this)\\s+(?:is|are)\\s+for\\s+how many\\b|\\b(?:enough|good)\\s+for\\s+how many\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEEDS_A_COUNT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:feeds|serves|makes)\\s+(?P<n>"), &g__COUNT()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_A_COUNT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bfor\\s+(?P<n>"), &g__COUNT()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PER_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:each|per person|apiece|a head|per head|each person|per serving)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AMOUNT(stdlib_call("re", "compile", &[py_str("(?<![\\d.])\\d+(?:\\.\\d+)?\\s*(?:g|grams?|kg|kilos?|ml|millilitres?|litres?|l|oz|ounces?|lbs?|pounds?|cups?|tablespoons?|tbsps?|teaspoons?|tsps?)\\b|\\bhalf a (?:cup|mug|kilo|pint)\\b|\\ba (?:cup|mug|kilo|pint)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_AFTER_A_COUNT(stdlib_call("re", "compile", &[py_str("\\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NUMBER_WORD(py_str("(?:\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a hundred|a dozen)"));
    s_A_COUNT_OF_PEOPLE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:about\\s+|around\\s+|just\\s+|only\\s+|all\\s+)?(?P<n>"), &g__NUMBER_WORD()), &py_str(")\\s+(?:people|guests|of us|adults|coming|mouths)\\b|\\b(?:theres|there are|there will be|we are|were|we have|weve got|make it|make that)\\s+(?:about\\s+|around\\s+)?(?P<m>")), &g__NUMBER_WORD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_COUNT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:and|so|ok|okay|right|then|but|no|yes|yeah)?[\\s,]*(?:how\\s+much|how\\s+many)?[\\s,]*(?:is\\s+that|would\\s+that\\s+be|does\\s+that\\s+make)?[\\s,]*(?:for|between)\\s+(?P<n>"), &g__NUMBER_WORD()), &py_str(")\\b[\\s.?!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_AND_UNIT(stdlib_call("re", "compile", &[py_str("\\b(?P<n>\\d+(?:\\.\\d+)?)\\s*(?P<u>kg|kilos?|g|grams?|ml|millilitres?|litres?|l|oz|ounces?|lbs?|pounds?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_EDGE(stdlib_call("re", "compile", &[py_str(",\\s*so\\s+|;\\s+|,\\s*(?:which|and that)\\s+|\\.\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORDS_FOR_NUMBERS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("a hundred"), Py::Int(100)), (py_str("a dozen"), Py::Int(12)), (py_str("dozen"), Py::Int(12))]));
    s__A_NEW_COUNTS(py_tuple(vec![stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:make it|make that|we are|were|we'll be|we will be|there are|theres|there's)\\s+(?P<n>"), &g__NUMBER_WORD()), &py_str(")(?:\\s+not\\s+(?P<old>")), &g__NUMBER_WORD()), &py_str("))?")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?P<n>"), &g__NUMBER_WORD()), &py_str(")\\s+not\\s+(?P<old>")), &g__NUMBER_WORD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:actually|no|sorry|oh|hang on)\\s+(?P<n>"), &g__NUMBER_WORD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:and\\s+|so\\s+)?(?P<n>"), &g__NUMBER_WORD()), &py_str(")\\s+(?:people|of us|guests|adults)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s__A_NUMBER_ANYWHERE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g__NUMBER_WORD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_UNIT_AFTER(stdlib_call("re", "compile", &[py_str("\\s*(?:kg|kilos?|grams?|g\\b|ml|millilitres?|litres?|l\\b|oz|ounces?|lbs?|pounds?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONFIRMATION(stdlib_call("re", "compile", &[py_str("\\b(?:confirmed|confirm|yes|yep|yeah|yup|exactly|right|correct|thats right|that's right|spot on|sure|indeed|definitely|probably|maybe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_time_not_a_portion(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("True when the count the answer gives is a length of time, not a number of people.");
    return call_builtin("bool", &[mcall(&g_A_TIME_AFTER_A_COUNT(), "match", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, mcall(&found.clone(), "end", &[py_str("n")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_last_answer(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's last line, which is the answer the person is looking at.\n\n    The reach back into the conversation is `bot/memory/history.py`'s one check, not a window\n    spelled out here: a module that reads a follow-up out of an earlier turn has to step back over\n    the same turns as every other one, and \"\" is what a chat with no earlier turn gives.\n    ");
    return crate::memory::history::the_last_answer(&[memory.clone()]);
    Py::None
}
pub fn _the_clause_with_the_count(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut edge: Py = Py::None;
    let mut left: Py = Py::None;
    let _ = py_str("The sentence or clause of the answer around the count, so the reply can quote the figure.");
    left = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE_EDGE(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    edge = __item.clone();
    if py_truthy(&op_le(&mcall(&edge.clone(), "end", &[]), &start.clone())) {
    left = mcall(&edge.clone(), "end", &[]);
    } else {
    if py_truthy(&op_ge(&mcall(&edge.clone(), "start", &[]), &end.clone())) {
    return mcall(&slice_or_raise(&said.clone(), &left.clone(), &mcall(&edge.clone(), "start", &[]), &Py::None), "strip", &[py_str(" .")]);
    }
    }
    }
    return mcall(&slice_or_raise(&said.clone(), &left.clone(), &Py::None, &Py::None), "strip", &[py_str(" .")]);
    Py::None
}
pub fn _the_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    Py::None
}
pub fn _as_a_count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("The number a count was said with, or 0 when it is not one this reads.");
    word = _the_number(&[word.clone()]);
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&g__WORDS_FOR_NUMBERS(), "get", &[word.clone(), Py::Int(0)]);
    Py::None
}
pub fn _in_grams(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut figure: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("A per-head figure as grams, so the sums stay in one unit; 0 for a unit this cannot weigh.");
    unit = mcall(&unit.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("kg"), py_str("kilo"), py_str("kilos")]), &unit.clone()))) {
    return op_mul(&figure.clone(), &Py::Float(1000.0));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("g"), py_str("gram"), py_str("grams")]), &unit.clone()))) {
    return figure.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("oz"), py_str("ounce"), py_str("ounces")]), &unit.clone()))) {
    return op_mul(&figure.clone(), &Py::Float(28.35));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("lb"), py_str("lbs"), py_str("pound"), py_str("pounds")]), &unit.clone()))) {
    return op_mul(&figure.clone(), &Py::Float(453.6));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("ml"), py_str("millilitre"), py_str("millilitres"), py_str("l"), py_str("litre"), py_str("litres")]), &unit.clone()))) {
    return op_mul(&figure.clone(), &if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("l"), py_str("litre"), py_str("litres")]), &unit.clone()))) { Py::Float(1000.0) } else { Py::Float(1.0) });
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _counts_people(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when a per-person word counts people rather than the parts of a recipe.\n\n    \"About 250 g of potatoes each\" weighs one person. \"about 175g of each for three medium eggs\"\n    weighs the butter, the sugar and the flour, and answering \"for 8\" out of it would say a person\n    eats 1.4 kg of cake mix. The word is the same, so the little word in front of it decides.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), &py_str("each")))) {
    return Py::Bool(true);
    }
    before = mcall(&mcall(&slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "rstrip", &[]), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&before.clone(), "endswith", &[py_tuple(vec![py_str("of"), py_str("in"), py_str("for"), py_str("with")])])));
    Py::None
}
pub fn _the_per_head_figure(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut head: Py = Py::None;
    let _ = py_str("The grams one person gets, as the answer says it, or 0 when it does not weigh one person.\n\n    The figure is read from the clause the per-person word is in, not from the answer as a whole:\n    \"About 75 g of dry rice each, so 300 g for four\" weighs one person at 75 g, and taking the\n    first figure of the answer only happens to agree with that.\n    ");
    said = { let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    let __iter1 = py_iter_or_raise(&mcall(&g_PER_PERSON(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_counts_people(&[said.clone(), found.clone()])))) {
    continue;
    }
    head = slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    clause = slice_or_raise(&head.clone(), &op_add(&call_builtin("max", &[mcall(&head.clone(), "rfind", &[py_str(".")]), mcall(&head.clone(), "rfind", &[py_str(",")]), mcall(&head.clone(), "rfind", &[py_str(";")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &Py::None, &Py::None);
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[mcall(&g_A_FIGURE_AND_UNIT(), "finditer", &[clause.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    figure = __item.clone();
    grams = _in_grams(&[call_builtin("float", &[mcall(&figure.clone(), "group", &[py_str("n")])]).unwrap_or_else(|e| exc::raise(e)), mcall(&figure.clone(), "group", &[py_str("u")])]);
    if py_truthy(&grams.clone()) {
    return grams.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&mcall(&g_A_FIGURE_AND_UNIT(), "finditer", &[said.clone()]));
    for __item in __iter3 {
    figure = __item.clone();
    grams = _in_grams(&[call_builtin("float", &[mcall(&figure.clone(), "group", &[py_str("n")])]).unwrap_or_else(|e| exc::raise(e)), mcall(&figure.clone(), "group", &[py_str("u")])]);
    if py_truthy(&grams.clone()) {
    return grams.clone();
    }
    }
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _a_total(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut grams: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grams") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: grams"))) };
    let mut kilos: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A total weight said the way a person says one: whole kilos above a kilo, grams below.");
    if py_truthy(&op_ge(&grams.clone(), &Py::Int(1000))) {
    kilos = call_builtin("round", &[op_truediv(&grams.clone(), &Py::Float(1000.0)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e));
    text = op_modulo(&py_str("%g"), &kilos.clone());
    return op_add(&text.clone(), &py_str(" kg"));
    }
    return op_modulo(&py_str("%g g"), &call_builtin("round", &[grams.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn the_portion_answer(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The answer in hand when it is a portion answer, or \"\" when it is about something else.");
    said = _the_last_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_AN_AMOUNT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PER_PERSON(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FEEDS_A_COUNT(), "search", &[said.clone()]) } } } }))) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn an_ask(args: &[Py]) -> Py {
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
    let mut counted: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when this line asks how many the portion answer in hand feeds.\n\n    This is the reading the arbiter asks for before the knowledge entries propose anything: the\n    party-food card is a good card and an answer to a different question, so the line that asks\n    what the answer in hand feeds is this module's, whatever else proposed. Kept side-effect free\n    so the claim and the reply cannot disagree.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_PORTION_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    said = the_portion_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    counted = { let __b = mcall(&g_FEEDS_A_COUNT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FOR_A_COUNT(), "search", &[said.clone()]) } };
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_time_not_a_portion(&[said.clone(), counted.clone()]))) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_PER_PERSON(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { _the_per_head_figure(&[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut amount: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reply to a line asking how many the answer in hand feeds, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_PORTION_ASK(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    said = the_portion_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    counted = { let __b = mcall(&g_FEEDS_A_COUNT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FOR_A_COUNT(), "search", &[said.clone()]) } };
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_time_not_a_portion(&[said.clone(), counted.clone()]))) } }) {
    number = _the_number(&[mcall(&counted.clone(), "group", &[py_str("n")])]);
    clause = _the_clause_with_the_count(&[said.clone(), mcall(&counted.clone(), "start", &[]), mcall(&counted.clone(), "end", &[])]);
    amount = mcall(&g_AN_AMOUNT(), "search", &[clause.clone()]);
    if py_truthy(&amount.clone()) {
    return op_add(&op_add(&op_add(&op_add(&py_str("That is for "), &number.clone()), &py_str(" - the answer puts it at ")), &mcall(&py_str(" "), "join", &[mcall(&mcall(&amount.clone(), "group", &[Py::Int(0)]), "split", &[])])), &py_str(" in total."));
    }
    return op_add(&op_add(&py_str("That is for "), &number.clone()), &py_str("."));
    }
    if py_truthy(&mcall(&g_PER_PERSON(), "search", &[said.clone()])) {
    grams = _the_per_head_figure(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&grams.clone()))) {
    return py_str("That is per person, so it goes up with however many you are feeding.");
    }
    let _ = py_setitem(&memory.clone(), &py_str("portion"), py_dict(vec![(py_str("grams"), grams.clone()), (py_str("turn"), call_builtin("len", &[crate::memory::history::turns_before(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)))]));
    return py_str("That is per person, not a total - say how many you are feeding and I will work the total out for you.");
    }
    return Py::None;
    Py::None
}
pub fn _is_an_amount_not_a_count(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut tail: Py = Py::None;
    let _ = py_str("True when the figure is an amount they want rather than a number of people.\n\n    \"how much rice for 8 people\" / \"make it 500g instead\" is a person changing the amount and not\n    the company: reading 500 as a count gave \"about 37.5 kg in total for 500\".\n    ");
    tail = slice_or_raise(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&found.clone(), "end", &[py_str("n")]), &Py::None, &Py::None);
    return call_builtin("bool", &[mcall(&g__A_UNIT_AFTER(), "match", &[tail.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_two_counts(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("True when the line works out a count of its own - \"3 kids and 2 adults\" - and not this one.\n\n    A sum of two groups is answered by the module that adds them up (375 g for five), and reading\n    the first number alone answers a question nobody asked.\n    ");
    if py_truthy(&mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("old")])) {
    return Py::Bool(false);
    }
    return op_gt(&call_builtin("len", &[mcall(&g__A_NUMBER_ANYWHERE(), "findall", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _the_new_count(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The count a correction names, as the line says it, or None.");
    let __iter1 = py_iter_or_raise(&g__A_NEW_COUNTS());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { _as_a_count(&[mcall(&found.clone(), "group", &[py_str("n")])]) } }) {
    if py_truthy(&{ let __b = _is_an_amount_not_a_count(&[text.clone(), found.clone()]); if py_truthy(&__b) { __b } else { _says_two_counts(&[text.clone(), found.clone()]) } }) {
    return Py::None;
    }
    return found.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _is_a_digit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "isdigit", &[]);
    Py::None
}
pub fn _the_count_they_gave(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The count the person answered the ask with, as (count, the word they used), or (0, \"\").\n\n    The reply before it asked how many are coming, so a number on this line is the answer - and a\n    person answers in their own words: \"so thats twenty confirmed\" and \"yep twenty\" name nobody at\n    all, and the who-word the first reading needs is not there. A figure in digits with nothing\n    confirming it is left alone, because \"i have 2000 on a credit card\" is not a party.\n    ");
    text = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    found = mcall(&g_A_COUNT_OF_PEOPLE(), "search", &[text.clone()]);
    if py_truthy(&found.clone()) {
    word = { let __b = mcall(&found.clone(), "group", &[py_str("n")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("m")]) } };
    count = _as_a_count(&[word.clone()]);
    if py_truthy(&{ let __b = count.clone(); if !py_truthy(&__b) { __b } else { op_le(&count.clone(), &Py::Int(500)) } }) {
    return py_tuple(vec![count.clone(), word.clone()]);
    }
    }
    let __iter1 = py_iter_or_raise(&mcall(&g__A_NUMBER_ANYWHERE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    word = mcall(&found.clone(), "group", &[Py::Int(0)]);
    count = _as_a_count(&[word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if py_truthy(&__b) { __b } else { op_gt(&count.clone(), &Py::Int(500)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = _is_a_digit(&[word.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_CONFIRMATION(), "search", &[text.clone()]))) } }) {
    continue;
    }
    return py_tuple(vec![count.clone(), word.clone()]);
    }
    return py_tuple(vec![Py::Int(0), py_str("")]);
    Py::None
}
pub fn _the_said_total(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut grams: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grams") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: grams"))) };
    let mut count: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut count_word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count_word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count_word"))) };
    let mut old_word: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old_word") { v.clone() }
    else { Py::None };
    let _ = py_str("The total and the per-head figure, with the number corrected said back.\n\n    A correction that is not read back is not answered: \"make it twenty not ten\" after the ask got\n    a total for twenty and no sign that the ten was dropped.\n    ");
    if py_truthy(&old_word.clone()) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("Not "), &_the_number(&[old_word.clone()])), &py_str(" - ")), &_the_number(&[count_word.clone()])), &py_str(": about ")), &_a_total(&[op_mul(&grams.clone(), &count.clone())])), &py_str(" in total, at ")), &_a_total(&[grams.clone()])), &py_str(" a head."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("About "), &_a_total(&[op_mul(&grams.clone(), &count.clone())])), &py_str(" in total for ")), &_the_number(&[count_word.clone()])), &py_str(" - that is ")), &_a_total(&[grams.clone()])), &py_str(" a head."));
    Py::None
}
pub fn _a_total_for_a_bare_count(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut old_word: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The same sum, when the line gives the number and the answer in hand gave the rest.\n\n    \"how much rice for 4 people\" is answered \"About 75 g of dry rice each, so 300 g for four\", and\n    the next line is \"so how much for 8\". The person has not named rice again, but the answer on the\n    screen has: it weighed one person and scaled to a count, so a bare number is the new count and\n    the figure is its own per-head one. The reply works the sum out for the number asked, because\n    saying the old total back is the one answer that is certainly wrong.\n    ");
    found = { let __b = mcall(&g_A_BARE_COUNT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { _the_new_count(&[text.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = _is_an_amount_not_a_count(&[text.clone(), found.clone()]); if py_truthy(&__b) { __b } else { _says_two_counts(&[text.clone(), found.clone()]) } } } }) {
    return Py::None;
    }
    said = _the_last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PER_PERSON(), "finditer", &[said.clone()])) { let mut found: Py = __item.clone(); __out.push(_counts_people(&[said.clone(), found.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    grams = _the_per_head_figure(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&grams.clone()))) {
    return Py::None;
    }
    count = _as_a_count(&[mcall(&found.clone(), "group", &[py_str("n")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if py_truthy(&__b) { __b } else { op_gt(&count.clone(), &Py::Int(500)) } }) {
    return Py::None;
    }
    old_word = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("old")]);
    return _the_said_total(&[grams.clone(), count.clone(), mcall(&found.clone(), "group", &[py_str("n")]), old_word.clone()]);
    Py::None
}
pub fn a_bare_count_to_fill(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is a bare count answering the per-head figure the answer in hand gave.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    found = { let __b = mcall(&g_A_BARE_COUNT(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { _the_new_count(&[text.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = _is_an_amount_not_a_count(&[text.clone(), found.clone()]); if py_truthy(&__b) { __b } else { _says_two_counts(&[text.clone(), found.clone()]) } } } }) {
    return Py::Bool(false);
    }
    said = _the_last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PER_PERSON(), "finditer", &[said.clone()])) { let mut found: Py = __item.clone(); __out.push(_counts_people(&[said.clone(), found.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_per_head_figure(&[said.clone()])))) {
    return Py::Bool(false);
    }
    count = _as_a_count(&[mcall(&found.clone(), "group", &[py_str("n")])]);
    return { let __b = call_builtin("bool", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&count.clone(), &Py::Int(500)) } };
    Py::None
}
pub fn a_count_to_fill(args: &[Py]) -> Py {
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
    let mut portion: Py = Py::None;
    let _ = py_str("True when this line answers the question the reply before it asked about a portion.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    portion = mcall(&memory.clone(), "get", &[py_str("portion")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[portion.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&portion.clone(), "get", &[py_str("grams")]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&op_sub(&call_builtin("len", &[crate::memory::history::turns_before(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&portion.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[geti(&_the_count_they_gave(&[text.clone()]), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_total_for(args: &[Py]) -> Py {
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
    let mut corrected: Py = Py::None;
    let mut count: Py = Py::None;
    let mut old_word: Py = Py::None;
    let mut portion: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The total a count of people makes of the figure the answer gave, or None.\n\n    A line that names how many are coming, said on the turn after the reply asked for it, is the\n    other half of the same question: \"how much potato salad for a party\" (250 g a head) / \"how many\n    does that feed\" / \"theres about twenty coming\" is a person finishing the sum the bot said it\n    would do. Nothing else reads it - the count has to name who is coming, and the ask has to be\n    the turn before - so a figure on a line of its own stays whatever it was about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    portion = mcall(&memory.clone(), "get", &[py_str("portion")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[portion.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&portion.clone(), "get", &[py_str("grams")]))) } }) {
    return _a_total_for_a_bare_count(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&op_gt(&op_sub(&call_builtin("len", &[crate::memory::history::turns_before(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&portion.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))) {
    let _ = mcall(&memory.clone(), "pop", &[py_str("portion"), Py::None]);
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_count_they_gave(&[text.clone()]), 2, None);
    count = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return Py::None;
    }
    let _ = mcall(&memory.clone(), "pop", &[py_str("portion"), Py::None]);
    corrected = _the_new_count(&[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    old_word = if py_truthy(&Py::Bool(!py_is(&corrected.clone(), &Py::None))) { mcall(&mcall(&corrected.clone(), "groupdict", &[]), "get", &[py_str("old")]) } else { Py::None };
    return _the_said_total(&[call_builtin("float", &[geti(&portion.clone(), &py_str("grams"))]).unwrap_or_else(|e| exc::raise(e)), count.clone(), said.clone(), old_word.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.portions".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_AMOUNT" => Some(g_AN_AMOUNT()),
            "A_BARE_COUNT" => Some(g_A_BARE_COUNT()),
            "A_CONFIRMATION" => Some(g_A_CONFIRMATION()),
            "A_COUNT_OF_PEOPLE" => Some(g_A_COUNT_OF_PEOPLE()),
            "A_FIGURE_AND_UNIT" => Some(g_A_FIGURE_AND_UNIT()),
            "A_PORTION_ASK" => Some(g_A_PORTION_ASK()),
            "A_TIME_AFTER_A_COUNT" => Some(g_A_TIME_AFTER_A_COUNT()),
            "CLAUSE_EDGE" => Some(g_CLAUSE_EDGE()),
            "FEEDS_A_COUNT" => Some(g_FEEDS_A_COUNT()),
            "FOR_A_COUNT" => Some(g_FOR_A_COUNT()),
            "PER_PERSON" => Some(g_PER_PERSON()),
            "_A_NEW_COUNTS" => Some(g__A_NEW_COUNTS()),
            "_A_NUMBER_ANYWHERE" => Some(g__A_NUMBER_ANYWHERE()),
            "_A_UNIT_AFTER" => Some(g__A_UNIT_AFTER()),
            "_COUNT" => Some(g__COUNT()),
            "_NUMBER_WORD" => Some(g__NUMBER_WORD()),
            "_WORDS_FOR_NUMBERS" => Some(g__WORDS_FOR_NUMBERS()),
            "_a_time_not_a_portion" => Some(py_func("_a_time_not_a_portion", |args: &[Py]| _a_time_not_a_portion(args))),
            "_a_total" => Some(py_func("_a_total", |args: &[Py]| _a_total(args))),
            "_a_total_for_a_bare_count" => Some(py_func("_a_total_for_a_bare_count", |args: &[Py]| _a_total_for_a_bare_count(args))),
            "_as_a_count" => Some(py_func("_as_a_count", |args: &[Py]| _as_a_count(args))),
            "_counts_people" => Some(py_func("_counts_people", |args: &[Py]| _counts_people(args))),
            "_in_grams" => Some(py_func("_in_grams", |args: &[Py]| _in_grams(args))),
            "_is_a_digit" => Some(py_func("_is_a_digit", |args: &[Py]| _is_a_digit(args))),
            "_is_an_amount_not_a_count" => Some(py_func("_is_an_amount_not_a_count", |args: &[Py]| _is_an_amount_not_a_count(args))),
            "_says_two_counts" => Some(py_func("_says_two_counts", |args: &[Py]| _says_two_counts(args))),
            "_the_clause_with_the_count" => Some(py_func("_the_clause_with_the_count", |args: &[Py]| _the_clause_with_the_count(args))),
            "_the_count_they_gave" => Some(py_func("_the_count_they_gave", |args: &[Py]| _the_count_they_gave(args))),
            "_the_last_answer" => Some(py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args))),
            "_the_new_count" => Some(py_func("_the_new_count", |args: &[Py]| _the_new_count(args))),
            "_the_number" => Some(py_func("_the_number", |args: &[Py]| _the_number(args))),
            "_the_per_head_figure" => Some(py_func("_the_per_head_figure", |args: &[Py]| _the_per_head_figure(args))),
            "_the_said_total" => Some(py_func("_the_said_total", |args: &[Py]| _the_said_total(args))),
            "a_bare_count_to_fill" => Some(py_func("a_bare_count_to_fill", |args: &[Py]| a_bare_count_to_fill(args))),
            "a_count_to_fill" => Some(py_func("a_count_to_fill", |args: &[Py]| a_count_to_fill(args))),
            "an_ask" => Some(py_func("an_ask", |args: &[Py]| an_ask(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_portion_answer" => Some(py_func("the_portion_answer", |args: &[Py]| the_portion_answer(args))),
            "the_total_for" => Some(py_func("the_total_for", |args: &[Py]| the_total_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_AMOUNT" => s_AN_AMOUNT(v),
            "A_BARE_COUNT" => s_A_BARE_COUNT(v),
            "A_CONFIRMATION" => s_A_CONFIRMATION(v),
            "A_COUNT_OF_PEOPLE" => s_A_COUNT_OF_PEOPLE(v),
            "A_FIGURE_AND_UNIT" => s_A_FIGURE_AND_UNIT(v),
            "A_PORTION_ASK" => s_A_PORTION_ASK(v),
            "A_TIME_AFTER_A_COUNT" => s_A_TIME_AFTER_A_COUNT(v),
            "CLAUSE_EDGE" => s_CLAUSE_EDGE(v),
            "FEEDS_A_COUNT" => s_FEEDS_A_COUNT(v),
            "FOR_A_COUNT" => s_FOR_A_COUNT(v),
            "PER_PERSON" => s_PER_PERSON(v),
            "_A_NEW_COUNTS" => s__A_NEW_COUNTS(v),
            "_A_NUMBER_ANYWHERE" => s__A_NUMBER_ANYWHERE(v),
            "_A_UNIT_AFTER" => s__A_UNIT_AFTER(v),
            "_COUNT" => s__COUNT(v),
            "_NUMBER_WORD" => s__NUMBER_WORD(v),
            "_WORDS_FOR_NUMBERS" => s__WORDS_FOR_NUMBERS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.portions", module_obj);
    pyrt::xrefs::reg_fn("memory.portions", "_a_time_not_a_portion", py_func("_a_time_not_a_portion", |args: &[Py]| _a_time_not_a_portion(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_last_answer", py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_clause_with_the_count", py_func("_the_clause_with_the_count", |args: &[Py]| _the_clause_with_the_count(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_number", py_func("_the_number", |args: &[Py]| _the_number(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_as_a_count", py_func("_as_a_count", |args: &[Py]| _as_a_count(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_in_grams", py_func("_in_grams", |args: &[Py]| _in_grams(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_counts_people", py_func("_counts_people", |args: &[Py]| _counts_people(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_per_head_figure", py_func("_the_per_head_figure", |args: &[Py]| _the_per_head_figure(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_a_total", py_func("_a_total", |args: &[Py]| _a_total(args)));
    pyrt::xrefs::reg_fn("memory.portions", "the_portion_answer", py_func("the_portion_answer", |args: &[Py]| the_portion_answer(args)));
    pyrt::xrefs::reg_fn("memory.portions", "an_ask", py_func("an_ask", |args: &[Py]| an_ask(args)));
    pyrt::xrefs::reg_fn("memory.portions", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_is_an_amount_not_a_count", py_func("_is_an_amount_not_a_count", |args: &[Py]| _is_an_amount_not_a_count(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_says_two_counts", py_func("_says_two_counts", |args: &[Py]| _says_two_counts(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_new_count", py_func("_the_new_count", |args: &[Py]| _the_new_count(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_is_a_digit", py_func("_is_a_digit", |args: &[Py]| _is_a_digit(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_count_they_gave", py_func("_the_count_they_gave", |args: &[Py]| _the_count_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_the_said_total", py_func("_the_said_total", |args: &[Py]| _the_said_total(args)));
    pyrt::xrefs::reg_fn("memory.portions", "_a_total_for_a_bare_count", py_func("_a_total_for_a_bare_count", |args: &[Py]| _a_total_for_a_bare_count(args)));
    pyrt::xrefs::reg_fn("memory.portions", "a_bare_count_to_fill", py_func("a_bare_count_to_fill", |args: &[Py]| a_bare_count_to_fill(args)));
    pyrt::xrefs::reg_fn("memory.portions", "a_count_to_fill", py_func("a_count_to_fill", |args: &[Py]| a_count_to_fill(args)));
    pyrt::xrefs::reg_fn("memory.portions", "the_total_for", py_func("the_total_for", |args: &[Py]| the_total_for(args)));
    pyrt::xrefs::reg_access("memory.portions", "AN_AMOUNT", g_AN_AMOUNT, s_AN_AMOUNT);
    pyrt::xrefs::reg_access("memory.portions", "A_BARE_COUNT", g_A_BARE_COUNT, s_A_BARE_COUNT);
    pyrt::xrefs::reg_access("memory.portions", "A_CONFIRMATION", g_A_CONFIRMATION, s_A_CONFIRMATION);
    pyrt::xrefs::reg_access("memory.portions", "A_COUNT_OF_PEOPLE", g_A_COUNT_OF_PEOPLE, s_A_COUNT_OF_PEOPLE);
    pyrt::xrefs::reg_access("memory.portions", "A_FIGURE_AND_UNIT", g_A_FIGURE_AND_UNIT, s_A_FIGURE_AND_UNIT);
    pyrt::xrefs::reg_access("memory.portions", "A_PORTION_ASK", g_A_PORTION_ASK, s_A_PORTION_ASK);
    pyrt::xrefs::reg_access("memory.portions", "A_TIME_AFTER_A_COUNT", g_A_TIME_AFTER_A_COUNT, s_A_TIME_AFTER_A_COUNT);
    pyrt::xrefs::reg_access("memory.portions", "CLAUSE_EDGE", g_CLAUSE_EDGE, s_CLAUSE_EDGE);
    pyrt::xrefs::reg_access("memory.portions", "FEEDS_A_COUNT", g_FEEDS_A_COUNT, s_FEEDS_A_COUNT);
    pyrt::xrefs::reg_access("memory.portions", "FOR_A_COUNT", g_FOR_A_COUNT, s_FOR_A_COUNT);
    pyrt::xrefs::reg_access("memory.portions", "PER_PERSON", g_PER_PERSON, s_PER_PERSON);
    pyrt::xrefs::reg_access("memory.portions", "_A_NEW_COUNTS", g__A_NEW_COUNTS, s__A_NEW_COUNTS);
    pyrt::xrefs::reg_access("memory.portions", "_A_NUMBER_ANYWHERE", g__A_NUMBER_ANYWHERE, s__A_NUMBER_ANYWHERE);
    pyrt::xrefs::reg_access("memory.portions", "_A_UNIT_AFTER", g__A_UNIT_AFTER, s__A_UNIT_AFTER);
    pyrt::xrefs::reg_access("memory.portions", "_COUNT", g__COUNT, s__COUNT);
    pyrt::xrefs::reg_access("memory.portions", "_NUMBER_WORD", g__NUMBER_WORD, s__NUMBER_WORD);
    pyrt::xrefs::reg_access("memory.portions", "_WORDS_FOR_NUMBERS", g__WORDS_FOR_NUMBERS, s__WORDS_FOR_NUMBERS);
}

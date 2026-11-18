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


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ANOTHER_PERIOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_FOR_THE_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS_WHAT_IS_NEEDED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_MONTH_OF_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PERIOD_IN_A_SUM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SUM_A_MONTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COPING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _EARN_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ENOUGH_TO_LIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _EVERYDAY_PERIOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LONDON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PERIODS_IN_A_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAID_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__ANOTHER_PERIOD() -> Py { ensure_init(); _ANOTHER_PERIOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_PERIOD(v: Py) { ensure_init(); if let Ok(mut w) = _ANOTHER_PERIOD.write() { *w = v; } }
pub fn g__ASKS_FOR_THE_FIGURE() -> Py { ensure_init(); _ASKS_FOR_THE_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_THE_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_FOR_THE_FIGURE.write() { *w = v; } }
pub fn g__ASKS_WHAT_IS_NEEDED() -> Py { ensure_init(); _ASKS_WHAT_IS_NEEDED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS_WHAT_IS_NEEDED(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS_WHAT_IS_NEEDED.write() { *w = v; } }
pub fn g__A_FIGURE() -> Py { ensure_init(); _A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _A_FIGURE.write() { *w = v; } }
pub fn g__A_MONTH_OF_IT() -> Py { ensure_init(); _A_MONTH_OF_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_MONTH_OF_IT(v: Py) { ensure_init(); if let Ok(mut w) = _A_MONTH_OF_IT.write() { *w = v; } }
pub fn g__A_PERIOD_IN_A_SUM() -> Py { ensure_init(); _A_PERIOD_IN_A_SUM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PERIOD_IN_A_SUM(v: Py) { ensure_init(); if let Ok(mut w) = _A_PERIOD_IN_A_SUM.write() { *w = v; } }
pub fn g__A_SUM_A_MONTH() -> Py { ensure_init(); _A_SUM_A_MONTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SUM_A_MONTH(v: Py) { ensure_init(); if let Ok(mut w) = _A_SUM_A_MONTH.write() { *w = v; } }
pub fn g__COPING() -> Py { ensure_init(); _COPING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COPING(v: Py) { ensure_init(); if let Ok(mut w) = _COPING.write() { *w = v; } }
pub fn g__EARN_ENOUGH() -> Py { ensure_init(); _EARN_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__EARN_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = _EARN_ENOUGH.write() { *w = v; } }
pub fn g__ENOUGH_TO_LIVE() -> Py { ensure_init(); _ENOUGH_TO_LIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ENOUGH_TO_LIVE(v: Py) { ensure_init(); if let Ok(mut w) = _ENOUGH_TO_LIVE.write() { *w = v; } }
pub fn g__EVERYDAY_PERIOD() -> Py { ensure_init(); _EVERYDAY_PERIOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__EVERYDAY_PERIOD(v: Py) { ensure_init(); if let Ok(mut w) = _EVERYDAY_PERIOD.write() { *w = v; } }
pub fn g__LIVING() -> Py { ensure_init(); _LIVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIVING(v: Py) { ensure_init(); if let Ok(mut w) = _LIVING.write() { *w = v; } }
pub fn g__LONDON() -> Py { ensure_init(); _LONDON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LONDON(v: Py) { ensure_init(); if let Ok(mut w) = _LONDON.write() { *w = v; } }
pub fn g__PERIODS_IN_A_YEAR() -> Py { ensure_init(); _PERIODS_IN_A_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PERIODS_IN_A_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = _PERIODS_IN_A_YEAR.write() { *w = v; } }
pub fn g__SAID_AGAIN() -> Py { ensure_init(); _SAID_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAID_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = _SAID_AGAIN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Sums about pay and time: what a month leaves you, and what a figure is in another period.\n\nThe arithmetic half of the money domain, in its own table because the two rules are one: a sum\nthe person typed, with a period on it, is a question about that sum. `bot/knowledge/money.py`\nowns the ordinary advice; this file owns the rows that do the sum - one adult's month against\nwhat the person has, and a figure said in one period asked in another.\n\nBoth rows read the figure and the period from the line and answer with the number. Both use a\n`guard`, never a `return None` inside the reply, because a row that wins the words and then says\nnothing leaves the line with no answer at all.\n");
    s_ANSWERS(py_tuple(vec![py_str("money")]));
    s__LIVING(py_tuple(vec![py_str("enough to live on"), py_str("enough to live"), py_str("enough to get by"), py_str("enough to survive"), py_str("to live on my own"), py_str("to live alone"), py_str("to live in london"), py_str("to live on your own"), py_str("enough money to live"), py_str("money to live on"), py_str("cost of living"), py_str("living costs"), py_str("afford to live"), py_str("live on"), py_str("live off"), py_str("survive on"), py_str("get by on"), py_str("need to live"), py_str("left to live on")]));
    s__COPING(py_tuple(vec![py_str("enough"), py_str("get by"), py_str("manage"), py_str("survive"), py_str("cope"), py_str("cover"), py_str("afford"), py_str("live off"), py_str("stretch to"), py_str("left over")]));
    s__ENOUGH_TO_LIVE(add(&g__LIVING(), &g__COPING()));
    s__A_MONTH_OF_IT(py_tuple(vec![py_str("a month"), py_str("per month"), py_str("each month"), py_str("monthly"), py_str("a week"), py_str("per week"), py_str("each week"), py_str("weekly"), py_str("a year"), py_str("per year"), py_str("each year"), py_str("yearly"), py_str("annual"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("income"), py_str("take home"), py_str("on my own"), py_str("on your own"), py_str("by myself"), py_str("by yourself"), py_str("living alone"), py_str("live alone")]));
    s__ASKS_WHAT_IS_NEEDED(py_tuple(vec![py_str("how much does a single person need"), py_str("how much does someone need"), py_str("how much do i need"), py_str("how much money do i need"), py_str("how much money does it take"), py_str("what do i need to live"), py_str("how much does it cost")]));
    s__A_SUM_A_MONTH(add(&g__A_MONTH_OF_IT(), &g__ASKS_WHAT_IS_NEEDED()));
    s__EARN_ENOUGH(py_tuple(vec![py_str("need to earn"), py_str("earn to live"), py_str("earn enough"), py_str("salary do i need"), py_str("salary would i need"), py_str("wage do i need"), py_str("how much do i need to earn"), py_str("what do i need to earn")]));
    s__LONDON(stdlib_call("re", "compile", &[py_str("\\blondon\\b|zone [12]|inner city"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FIGURE(stdlib_call("re", "compile", &[crate::gen::knowledge::money::g__MONEY(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("enough-to-live-on")); __args.push(py_tuple(vec![g__ENOUGH_TO_LIVE(), g__A_SUM_A_MONTH()])); __args.push(g__enough_to_live_on()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__a_month_of_life()), ("avoid".to_string(), py_tuple(vec![py_str("deposit"), py_str("saved"), py_str("savings goal"), py_str("mortgage"), py_str("buy a flat"), py_str("buy a house"), py_str("for food"), py_str("on food"), py_str("food shop"), py_str("for petrol"), py_str("on petrol"), py_str("universal credit"), py_str("benefit"), py_str("benefits"), py_str("carer"), py_str("carers"), py_str("pension"), py_str("abroad"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("pet"), py_str("pets"), py_str("plants"), py_str("my partner"), py_str("my husband"), py_str("my wife"), py_str("my son"), py_str("my daughter"), py_str("family of"), py_str("two of us"), py_str("kids"), py_str("children"), py_str("rent"), py_str("deposit"), py_str("council tax"), py_str("energy bill"), py_str("gas bill"), py_str("electric bill")])), ("ask".to_string(), py_tuple(vec![py_str("enough"), py_str("much"), py_str("how do i"), py_str("can i"), py_str("could i"), py_str("will i"), py_str("do i"), py_str("is it"), py_str("would i"), py_str("need"), py_str("get by"), py_str("afford"), py_str("only"), py_str("just"), py_str("survive"), py_str("manage"), py_str("cover")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s__A_PERIOD_IN_A_SUM(stdlib_call("re", "compile", &[add(&crate::gen::knowledge::money::g__MONEY(), &py_str("\\s*(?:a|an|per|each|every|/)\\s*(week|weekly|fortnight|fortnightly|month|monthly|year|yearly|annually|annual)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PERIODS_IN_A_YEAR(py_dict(vec![(py_str("week"), Py::Float(52.0)), (py_str("weekly"), Py::Float(52.0)), (py_str("fortnight"), Py::Float(26.0)), (py_str("fortnightly"), Py::Float(26.0)), (py_str("month"), Py::Float(12.0)), (py_str("monthly"), Py::Float(12.0)), (py_str("year"), Py::Float(1.0)), (py_str("yearly"), Py::Float(1.0)), (py_str("annual"), Py::Float(1.0)), (py_str("annually"), Py::Float(1.0))]));
    s__ANOTHER_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|per|each|every|over|in|for|of|the)\\s*(week|fortnight|month|year)\\b|\\b(yearly|annually|annual|monthly|weekly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__EVERYDAY_PERIOD(py_dict(vec![(py_str("yearly"), py_str("year")), (py_str("annually"), py_str("year")), (py_str("annual"), py_str("year")), (py_str("monthly"), py_str("month")), (py_str("weekly"), py_str("week"))]));
    s__SAID_AGAIN(py_dict(vec![(py_str("year"), py_str("a year")), (py_str("month"), py_str("a month")), (py_str("week"), py_str("a week")), (py_str("fortnight"), py_str("a fortnight"))]));
    s__ASKS_FOR_THE_FIGURE(py_tuple(vec![py_str("how much"), py_str("how many"), py_str("whats"), py_str("what's"), py_str("what is"), py_str("what does"), py_str("what would"), py_str("what will"), py_str("comes to"), py_str("come to"), py_str("convert"), py_str("worth"), py_str("equivalent"), py_str("in a year"), py_str("in a month"), py_str("in a week"), py_str("over a year"), py_str("over a month"), py_str("a year in total"), py_str("a year altogether")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-sum-in-another-period")); __args.push(py_tuple(vec![py_tuple(vec![py_str("a month"), py_str("a week"), py_str("a fortnight"), py_str("a month"), py_str("per month"), py_str("monthly"), py_str("each month"), py_str("weekly"), py_str("every week"), py_str("each week"), py_str("a month")]), py_tuple(vec![py_str("a year"), py_str("yearly"), py_str("a yr"), py_str("per year"), py_str("annually"), py_str("annual"), py_str("over a year"), py_str("in a year"), py_str("a month"), py_str("monthly"), py_str("per month"), py_str("a week"), py_str("weekly"), py_str("per week")])])); __args.push(g__a_sum_in_another_period()); __kws.extend(vec![("guard".to_string(), g__two_periods_named()), ("avoid".to_string(), py_tuple(vec![py_str("spare"), py_str("save"), py_str("saving"), py_str("savings"), py_str("put by"), py_str("put aside"), py_str("put away"), py_str("set aside"), py_str("rent"), py_str("landlord"), py_str("deposit"), py_str("mortgage"), py_str("afford"), py_str("budget"), py_str("cheapest"), py_str("how long"), py_str("records"), py_str("receipts"), py_str("after tax"), py_str("take home"), py_str("take-home"), py_str("net pay"), py_str("tax")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
}
pub fn _living_not_a_bill(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let _ = py_str("\"the cost of living\" and \"living costs\" name the thing asked about, not money going out.\n\n    `_per_month` reads a bill out of the line, and reading these as one made \"would 1200 a month\n    cover living costs\" lose its figure and get the costs with an ask for a figure it already had.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("cost of living"), py_str("costs of living"), py_str("living costs"), py_str("living cost")]));
    for __item in __iter1 {
    name = __item.clone();
    text = mcall(&text.clone(), "replace", &[name.clone(), py_str("living")]);
    }
    return text.clone();
    Py::None
}
pub fn _the_gross_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut net_a_month: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "net_a_month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: net_a_month"))) };
    let mut _u: Py = Py::None;
    let mut gross: Py = Py::None;
    let mut ni: Py = Py::None;
    let mut tax: Py = Py::None;
    let _ = py_str("The salary that leaves about this much in the bank, rounded up to the next £100.");
    gross = call_builtin("int", &[mul(&net_a_month.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(40)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    let __unpk2 = py_unpack(&crate::gen::knowledge::money::_income_tax_and_ni(&[gross.clone()]), 2, None);
    tax = __unpk2[0].clone();
    ni = __unpk2[1].clone();
    if py_truthy(&ge(&truediv(&sub(&sub(&gross.clone(), &tax.clone()), &ni.clone()), &Py::Float(12.0)), &net_a_month.clone())) {
    return call_builtin("int", &[mul(&neg(&floordiv(&neg(&gross.clone()), &Py::Int(100))), &Py::Int(100))]).unwrap_or_else(|e| exc::raise(e));
    }
    let __aug3 = add(&gross.clone(), &Py::Int(100));
    gross = __aug3;
    }
    return gross.clone();
    Py::None
}
pub fn _a_months_costs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut london: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "london") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: london"))) };
    let _ = py_str("What one adult's month costs, in the few numbers a friend would say.");
    if py_truthy(&london.clone()) {
    return py_str("In London the same month is roughly £2,000, nearly all of it rent: a one-bed is £1,300 to £1,900 there before the bills.");
    }
    return py_str("Outside London one adult needs roughly £1,200 a month: rent £600 to £900, council tax about £180, energy about £145, food £150 to £200, and about £100 for broadband, phone and travel.");
    Py::None
}
pub fn _a_month_of_life(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about a month of ordinary life and not about one purchase.\n\n    A figure in the line is what the row is about. With one, a living phrase answers on its own. A bare coping word does not: with only \"enough\", \"afford\" or \"cover\" in\n    it, the line could be a holiday (\"is 1,200 enough for a week in spain\"), a mortgage (\"can we\n    afford a house on one salary\") or something that is not money at all (\"how much exercise a\n    week is enough\"). Those need a figure in the line and a reading that names no other subject -\n    the understood topic is money or nothing - which is the difference between \"is 60k a year\n    enough\" and a week in Spain. A guard, not a `return None` in the reply: a row that has won\n    the words and then says nothing leaves the line with \"I'm listening\".\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&mcall(&g__A_FIGURE(), "search", &[text.clone()])) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__LIVING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    topic = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COPING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), py_str("money")]), &topic.clone())) } };
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKS_WHAT_IS_NEEDED()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { _earn_asked(&[text.clone()]) } }) {
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__LIVING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COPING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _earn_asked(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the line asks what to earn rather than what a sum you have will cover.");
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EARN_ENOUGH()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(salary|wage)\\b.{0,20}\\b(need|needed|require[sd]?)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _enough_to_live_on(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut enough: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut london: Py = Py::None;
    let mut monthly: Py = Py::None;
    let mut ni: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut salary: Py = Py::None;
    let mut tax: Py = Py::None;
    let mut text: Py = Py::None;
    let mut tight: Py = Py::None;
    let mut verdict: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("A sum beside \"enough to live on\" is a subtraction, not a lesson in budgeting.");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    london = call_builtin("bool", &[mcall(&g__LONDON(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), py_str("")]), 2, None);
    monthly = __unpk1[0].clone();
    lead = __unpk1[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[py_str("a year|per year|each year|yearly|annual"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    salary = crate::gen::knowledge::money::_salary_in(&[text.clone()]);
    if py_truthy(&{ let __b = salary.clone(); if !py_truthy(&__b) { __b } else { ge(&salary.clone(), &Py::Int(5000)) } }) {
    let __unpk2 = py_unpack(&crate::gen::knowledge::money::_income_tax_and_ni(&[salary.clone()]), 2, None);
    tax = __unpk2[0].clone();
    ni = __unpk2[1].clone();
    monthly = call_builtin("int", &[call_builtin("round", &[truediv(&sub(&sub(&salary.clone(), &tax.clone()), &ni.clone()), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    lead = mcall(&py_str("On £{:,} a year you take home about £{:,} a month. "), "format", &[salary.clone(), monthly.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&monthly.clone()))) {
    monthly = crate::gen::knowledge::money::_per_month(&[_living_not_a_bill(&[text.clone()])]);
    }
    let __unpk3 = py_unpack(&if py_truthy(&london.clone()) { py_tuple(vec![Py::Int(1750), Py::Int(2100)]) } else { py_tuple(vec![Py::Int(1050), Py::Int(1300)]) }, 2, None);
    tight = __unpk3[0].clone();
    enough = __unpk3[1].clone();
    where_ = if py_truthy(&london.clone()) { py_str("in London") } else { py_str("outside London") };
    if py_truthy(&_earn_asked(&[text.clone()])) {
    return add(&add(&modulo(&py_str("You need roughly £%s a year before tax to live on your own %s - about £%s a month after tax. "), &py_tuple(vec![call_builtin("format", &[_the_gross_for(&[enough.clone()]), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), where_.clone(), call_builtin("format", &[enough.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))])), &_a_months_costs(&[london.clone()])), &py_str(" Both move, so check this year's band and the price cap."));
    }
    if py_truthy(&Py::Bool(!py_truthy(&monthly.clone()))) {
    return add(&_a_months_costs(&[london.clone()]), &py_str(" Tell me the figure you have in mind and I will say whether it covers the month."));
    }
    if py_truthy(&ge(&monthly.clone(), &enough.clone())) {
    verdict = modulo(&py_str("£%s a month is enough %s, with room to save."), &py_tuple(vec![call_builtin("format", &[monthly.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), where_.clone()]));
    } else {
    if py_truthy(&ge(&monthly.clone(), &tight.clone())) {
    verdict = modulo(&py_str("£%s a month is enough %s only with a cheap rent - there is little left at the end of the month."), &py_tuple(vec![call_builtin("format", &[monthly.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), where_.clone()]));
    } else {
    verdict = modulo(&py_str("£%s a month leaves you about £%s short of what one adult needs %s."), &py_tuple(vec![call_builtin("format", &[monthly.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[sub(&enough.clone(), &monthly.clone()), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), where_.clone()]));
    }
    }
    return add(&add(&add(&add(&lead.clone(), &verdict.clone()), &py_str(" ")), &_a_months_costs(&[london.clone()])), &py_str(" Both move, so check this year's band and the price cap."));
    Py::None
}
pub fn _per_period(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut other: Py = Py::None;
    let mut own: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("(sum, its period, the other period asked for) from a line that names both, or None.");
    match_ = mcall(&g__A_PERIOD_IN_A_SUM(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) {
    let __aug1 = mul(&amount.clone(), &Py::Int(1000));
    amount = __aug1;
    }
    own = mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "lower", &[]);
    own = mcall(&g__EVERYDAY_PERIOD(), "get", &[own.clone(), own.clone()]);
    let __iter2 = py_iter_or_raise(&mcall(&g__ANOTHER_PERIOD(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    other = __item.clone();
    if py_truthy(&{ let __b = lt(&mcall(&other.clone(), "start", &[]), &mcall(&match_.clone(), "end", &[])); if !py_truthy(&__b) { __b } else { gt(&mcall(&other.clone(), "end", &[]), &mcall(&match_.clone(), "start", &[])) } }) {
    continue;
    }
    wanted = mcall(&{ let __b = mcall(&other.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&other.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    wanted = mcall(&g__EVERYDAY_PERIOD(), "get", &[wanted.clone(), wanted.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("week"), py_str("fortnight"), py_str("month"), py_str("year")]), &wanted.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&wanted.clone(), &own.clone())) } }) {
    return py_tuple(vec![amount.clone(), own.clone(), wanted.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _two_periods_named(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line names a sum in one period and asks for another.\n\n    A guard rather than a `return None` in the reply: a row that has won the words and then says\n    nothing leaves the line with no answer at all, which is how \"30k a year is how much a month\"\n    became \"I'm listening\".\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("bool", &[_per_period(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_sum_in_another_period(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut amount: Py = Py::None;
    let mut annual: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut left: Py = Py::None;
    let mut ni: Py = Py::None;
    let mut own: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tax: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One period's sum worked into another period, with the tax where the sum is pay.");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("keep"), py_str("records"), py_str("receipts"), py_str("payslips"), py_str("statements"), py_str("tax return"), py_str("how long"), py_str("how many years")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    found = _per_period(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKS_FOR_THE_FIGURE()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 3, None);
    amount = __unpk1[0].clone();
    own = __unpk1[1].clone();
    wanted = __unpk1[2].clone();
    annual = mul(&amount.clone(), &geti(&g__PERIODS_IN_A_YEAR(), &own.clone()));
    value = truediv(&annual.clone(), &geti(&g__PERIODS_IN_A_YEAR(), &wanted.clone()));
    said = mcall(&py_str("£{:,} {} is £{:,} {}."), "format", &[call_builtin("int", &[call_builtin("round", &[amount.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), geti(&g__SAID_AGAIN(), &own.clone()), call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), geti(&g__SAID_AGAIN(), &wanted.clone())]);
    clause = crate::gen::knowledge::money::_clause_about(&[text.clone(), Py::Int(0), call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("earn"), py_str("earns"), py_str("salary"), py_str("wage"), py_str("wages"), py_str("paid"), py_str("pay"), py_str("job")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clause.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk2 = py_unpack(&crate::gen::knowledge::money::_income_tax_and_ni(&[call_builtin("int", &[call_builtin("round", &[annual.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    tax = __unpk2[0].clone();
    ni = __unpk2[1].clone();
    left = sub(&sub(&annual.clone(), &tax.clone()), &ni.clone());
    let __aug3 = add(&said.clone(), &mcall(&py_str(" After tax and National Insurance that is about £{:,} a month, or £{:,} a year."), "format", &[call_builtin("int", &[call_builtin("round", &[truediv(&left.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[call_builtin("round", &[left.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    said = __aug3;
    }
    return said.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.income_periods".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ANOTHER_PERIOD" => Some(g__ANOTHER_PERIOD()),
            "_ASKS_FOR_THE_FIGURE" => Some(g__ASKS_FOR_THE_FIGURE()),
            "_ASKS_WHAT_IS_NEEDED" => Some(g__ASKS_WHAT_IS_NEEDED()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_MONTH_OF_IT" => Some(g__A_MONTH_OF_IT()),
            "_A_PERIOD_IN_A_SUM" => Some(g__A_PERIOD_IN_A_SUM()),
            "_A_SUM_A_MONTH" => Some(g__A_SUM_A_MONTH()),
            "_COPING" => Some(g__COPING()),
            "_EARN_ENOUGH" => Some(g__EARN_ENOUGH()),
            "_ENOUGH_TO_LIVE" => Some(g__ENOUGH_TO_LIVE()),
            "_EVERYDAY_PERIOD" => Some(g__EVERYDAY_PERIOD()),
            "_LIVING" => Some(g__LIVING()),
            "_LONDON" => Some(g__LONDON()),
            "_PERIODS_IN_A_YEAR" => Some(g__PERIODS_IN_A_YEAR()),
            "_SAID_AGAIN" => Some(g__SAID_AGAIN()),
            "_a_month_of_life" => Some(py_func("_a_month_of_life", |args: &[Py]| _a_month_of_life(args))),
            "_a_months_costs" => Some(py_func("_a_months_costs", |args: &[Py]| _a_months_costs(args))),
            "_a_sum_in_another_period" => Some(py_func("_a_sum_in_another_period", |args: &[Py]| _a_sum_in_another_period(args))),
            "_earn_asked" => Some(py_func("_earn_asked", |args: &[Py]| _earn_asked(args))),
            "_enough_to_live_on" => Some(py_func("_enough_to_live_on", |args: &[Py]| _enough_to_live_on(args))),
            "_living_not_a_bill" => Some(py_func("_living_not_a_bill", |args: &[Py]| _living_not_a_bill(args))),
            "_per_period" => Some(py_func("_per_period", |args: &[Py]| _per_period(args))),
            "_the_gross_for" => Some(py_func("_the_gross_for", |args: &[Py]| _the_gross_for(args))),
            "_two_periods_named" => Some(py_func("_two_periods_named", |args: &[Py]| _two_periods_named(args))),
            _ => None,
        }),
        set: None,
    }))
}

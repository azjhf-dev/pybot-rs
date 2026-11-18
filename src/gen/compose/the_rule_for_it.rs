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


pub static ABOUT_A_PERSON_OR_AN_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMOUNT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMOUNT_COVER_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AMOUNT_FEEDING_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_AN_OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CAB_OR_A_LIFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CARE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COOKING_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DEAR_AS_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FARE_OR_A_TICKET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAB_FARE_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COOKING_TIME_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FARE_PRICE_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEEDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_OR_PACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GONE_OFF_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPING_HAS_NO_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEP_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MATERIAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_ON_A_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_ABOUT_A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_KEEPING_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OCCASION_PRICE_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFF_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_VERDICT_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_HEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_POOLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_GOOD_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_NOT_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_A_PERSON_OR_AN_ANIMAL() -> Py { ensure_init(); ABOUT_A_PERSON_OR_AN_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_A_PERSON_OR_AN_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_A_PERSON_OR_AN_ANIMAL.write() { *w = v; } }
pub fn g_AMOUNT_ASK() -> Py { ensure_init(); AMOUNT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNT_ASK.write() { *w = v; } }
pub fn g_AMOUNT_COVER_RULE() -> Py { ensure_init(); AMOUNT_COVER_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_COVER_RULE(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNT_COVER_RULE.write() { *w = v; } }
pub fn g_AMOUNT_FEEDING_RULE() -> Py { ensure_init(); AMOUNT_FEEDING_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_FEEDING_RULE(v: Py) { ensure_init(); if let Ok(mut w) = AMOUNT_FEEDING_RULE.write() { *w = v; } }
pub fn g_A_AN_OCCASION() -> Py { ensure_init(); A_AN_OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_AN_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = A_AN_OCCASION.write() { *w = v; } }
pub fn g_A_CAB_OR_A_LIFT() -> Py { ensure_init(); A_CAB_OR_A_LIFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CAB_OR_A_LIFT(v: Py) { ensure_init(); if let Ok(mut w) = A_CAB_OR_A_LIFT.write() { *w = v; } }
pub fn g_A_CARE_WORD() -> Py { ensure_init(); A_CARE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CARE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_CARE_WORD.write() { *w = v; } }
pub fn g_A_COOKING_TIME() -> Py { ensure_init(); A_COOKING_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COOKING_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_COOKING_TIME.write() { *w = v; } }
pub fn g_A_DEAR_AS_A_NAME() -> Py { ensure_init(); A_DEAR_AS_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DEAR_AS_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = A_DEAR_AS_A_NAME.write() { *w = v; } }
pub fn g_A_FARE_OR_A_TICKET() -> Py { ensure_init(); A_FARE_OR_A_TICKET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FARE_OR_A_TICKET(v: Py) { ensure_init(); if let Ok(mut w) = A_FARE_OR_A_TICKET.write() { *w = v; } }
pub fn g_A_FOOD() -> Py { ensure_init(); A_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = A_FOOD.write() { *w = v; } }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); A_MONEY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_WORD.write() { *w = v; } }
pub fn g_CAB_FARE_RULE() -> Py { ensure_init(); CAB_FARE_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAB_FARE_RULE(v: Py) { ensure_init(); if let Ok(mut w) = CAB_FARE_RULE.write() { *w = v; } }
pub fn g_COOKING_TIME_RULE() -> Py { ensure_init(); COOKING_TIME_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COOKING_TIME_RULE(v: Py) { ensure_init(); if let Ok(mut w) = COOKING_TIME_RULE.write() { *w = v; } }
pub fn g_FARE_PRICE_RULE() -> Py { ensure_init(); FARE_PRICE_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FARE_PRICE_RULE(v: Py) { ensure_init(); if let Ok(mut w) = FARE_PRICE_RULE.write() { *w = v; } }
pub fn g_FEEDING() -> Py { ensure_init(); FEEDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEEDING(v: Py) { ensure_init(); if let Ok(mut w) = FEEDING.write() { *w = v; } }
pub fn g_FOOD_OR_PACK() -> Py { ensure_init(); FOOD_OR_PACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_OR_PACK(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_OR_PACK.write() { *w = v; } }
pub fn g_GONE_OFF_ASK() -> Py { ensure_init(); GONE_OFF_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GONE_OFF_ASK(v: Py) { ensure_init(); if let Ok(mut w) = GONE_OFF_ASK.write() { *w = v; } }
pub fn g_KEEPING_HAS_NO_DATE() -> Py { ensure_init(); KEEPING_HAS_NO_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPING_HAS_NO_DATE(v: Py) { ensure_init(); if let Ok(mut w) = KEEPING_HAS_NO_DATE.write() { *w = v; } }
pub fn g_KEEP_ASK() -> Py { ensure_init(); KEEP_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEP_ASK(v: Py) { ensure_init(); if let Ok(mut w) = KEEP_ASK.write() { *w = v; } }
pub fn g_MATERIAL() -> Py { ensure_init(); MATERIAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MATERIAL(v: Py) { ensure_init(); if let Ok(mut w) = MATERIAL.write() { *w = v; } }
pub fn g_MONEY_ON_A_JOB() -> Py { ensure_init(); MONEY_ON_A_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_ON_A_JOB(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_ON_A_JOB.write() { *w = v; } }
pub fn g_NOT_ABOUT_A_PRICE() -> Py { ensure_init(); NOT_ABOUT_A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_ABOUT_A_PRICE.write() { *w = v; } }
pub fn g_NOT_A_KEEPING_QUESTION() -> Py { ensure_init(); NOT_A_KEEPING_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_KEEPING_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_KEEPING_QUESTION.write() { *w = v; } }
pub fn g_OCCASION_PRICE_RULE() -> Py { ensure_init(); OCCASION_PRICE_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OCCASION_PRICE_RULE(v: Py) { ensure_init(); if let Ok(mut w) = OCCASION_PRICE_RULE.write() { *w = v; } }
pub fn g_OFF_ASK() -> Py { ensure_init(); OFF_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFF_ASK(v: Py) { ensure_init(); if let Ok(mut w) = OFF_ASK.write() { *w = v; } }
pub fn g_PRICE_ASK() -> Py { ensure_init(); PRICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ASK.write() { *w = v; } }
pub fn g_PRICE_RULE() -> Py { ensure_init(); PRICE_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_RULE(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_RULE.write() { *w = v; } }
pub fn g_PRICE_VERDICT() -> Py { ensure_init(); PRICE_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_VERDICT.write() { *w = v; } }
pub fn g_PRICE_VERDICT_RULE() -> Py { ensure_init(); PRICE_VERDICT_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_VERDICT_RULE(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_VERDICT_RULE.write() { *w = v; } }
pub fn g_RULE_HEADS() -> Py { ensure_init(); RULE_HEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_HEADS(v: Py) { ensure_init(); if let Ok(mut w) = RULE_HEADS.write() { *w = v; } }
pub fn g_RULE_POOLS() -> Py { ensure_init(); RULE_POOLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_POOLS(v: Py) { ensure_init(); if let Ok(mut w) = RULE_POOLS.write() { *w = v; } }
pub fn g_STILL_ASK() -> Py { ensure_init(); STILL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ASK.write() { *w = v; } }
pub fn g_STILL_GOOD_RULE() -> Py { ensure_init(); STILL_GOOD_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_GOOD_RULE(v: Py) { ensure_init(); if let Ok(mut w) = STILL_GOOD_RULE.write() { *w = v; } }
pub fn g_TIME_ASK() -> Py { ensure_init(); TIME_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TIME_ASK.write() { *w = v; } }
pub fn g_TIME_NOT_THIS() -> Py { ensure_init(); TIME_NOT_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_NOT_THIS(v: Py) { ensure_init(); if let Ok(mut w) = TIME_NOT_THIS.write() { *w = v; } }
pub fn g_TIME_RULE() -> Py { ensure_init(); TIME_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_RULE(v: Py) { ensure_init(); if let Ok(mut w) = TIME_RULE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("What the bot knows about the kind of question, said before it says where the rest is.\n\nSTYLE.md's honest band has one rule this file exists for: a pointer on its own is not a reply.\n\"I can't look up live prices, sorry. The seller's site has today's figure\" names no thing, says\nnothing about the thing, and leaves the person exactly where they were - and the same shape was\nin the plain shrug, the place pools and the live notes, so it was the commonest failing reply in\nthe band (843 shrugs on the deck at base `49408e53`, from three lines).\n\nThe general rule, not a list of wordings: **everyday asks come in kinds, and each kind has one\nthing that always settles it.** A price turns on where you buy it and the size, never on the\nfirst number you see. An amount turns on a portion per head and how many are eating, or on how\nfar one pack covers and how big the job is. Whether a thing is still good turns on how long it\nhas been and how it was kept. None of that is a fact about the world that could go out of date:\nit is the shape of the answer, which is the part the bot can always give, and it is what makes\nthe pointer that follows worth reading.\n\nSo a reply here is three beats, in this order: what settles it (the rule, and their own subject\nin it where the line has one), where the current figure or date is kept, and the part the bot can\nstill do - the sums, the weighing. It ends there, and it asks nothing back: the person has just\nsaid what they want.\n\nWhat it will not touch, because a rule of thumb about soup is wrong about a dose:\n\n* a line naming a medicine, a dose or a body part - `safety.py` and the health cards own those;\n* a line about a person rather than a thing: \"how heavy should a 6 month old be\" is a baby on a\n  scale, not a joint on a plate, and a portion the count multiplies is the wrong arithmetic;\n* a line about money on an account (\"up front\", a deposit, a bill) - the figure there is real and\n  in one place, and the place pools say where;\n* a keeping question with nothing in it that keeps: \"where can i eat with my dog\" and \"my son has\n  a nut allergy, what can i cook\" both contain the words of a keeping ask and neither is one.\n\nEvery pool here is read last, after every reading that could answer the line, so it can only ever\nstand in for a shrug.\n");
    s_PRICE_RULE(py_tuple(vec![py_str("What a figure like that turns on is which one it is, where you buy it and the day, so two or three sellers compared all-in will beat any one number you see. The current figures are on their own pages, and bring them back and I will work out what they come to a month."), py_str("A job is priced by the quote, not by a list: the same job gets two or three quotes on the same spec, in writing, and that is the number worth comparing. A thing on a shelf is the seller's own page, all in with delivery - tell me what you find and I will do the sums."), py_str("Prices move by exactly what you are buying and by where, so one number from one seller is not an answer to it. Two sellers, what it comes to all in, and then the part I am good at: what it leaves you a month.")]));
    s_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow much (?:is|are|was|were|does|do|did|would|will|should|might)\\b[^?.!,]{0,40}?\\b(?:cost|costs|charge|charge for|worth|set me back|come to|pay|paying|spend)\\b|\\bhow much (?:should|shall|ought to|do|would)\\b[^?.!,]{0,24}?\\b(?:pay|spend|be paying)\\b|\\bwhat (?:does|do|did|would|will|should)\\b[^?.!,]{0,36}?\\bcost\\b|\\bwhat(?:'s|s)? the (?:cost|price|going rate|damage|ballpark)\\b|\\bwhat about the cost\\b|\\bthe cost of\\b|\\bprice of (?:a|an|the)\\b|\\bhow much (?:is|are) (?:a|an|the)\\b|\\bis (?:that|it|this) (?:a )?(?:fair|good|normal|about right) (?:price|money)\\b|\\bis (?:that|it|this) (?:too )?(?:expensive|dear|cheap)\\b|\\bcan i get (?:it|that|one) (?:any )?cheaper\\b|\\bwhat would that cost\\b|\\bhow much would that (?:be|cost)\\b|\\bhow much (?:would|will|might|could) (?:a|an|the|one)\\b[^?.!]{0,30}?\\bbe\\b|\\bhow much (?:is|are|was|were)\\b(?!\\s*(?:enough|too\\b|left|more|of\\b|in\\b|there\\b|it\\b|that\\b|this\\b|going\\b))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|will|would|does|do)\\b(?!\\s*(?:you|u)\\b)[^?.!,]{0,40}?\\b(?:expensive|pricey|costly|spenny|overpriced)\\b|\\b(?:is|are|was|were)\\b[^?.!,]{0,40}?\\b(?:too|so|very|quite|really|that|a bit|dead|bloody)\\s+dear\\b|\\bdear(?:er)?\\s+than\\b|\\b(?:is|are)\\s+(?!you\\b|she\\b|he\\b|they\\b|we\\b|i\\b)(?:[a-z']+\\s+){1,3}(?:too\\s+)?dear\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEAR_AS_A_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:my|your|our|his|her|their|the|a|an)\\s+dear\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FARE_OR_A_TICKET(stdlib_call("re", "compile", &[py_str("\\b(?:trains?|flights?|buses|coach|ferry|tubes?|parking|park|tickets?|entry|admission|petrol|fuel|diesel|delivery|shipping|car hire|hire car)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FARE_PRICE_RULE(py_tuple(vec![py_str("The day is what sets that one, not the seller: the same trip off-peak on a Tuesday can be a fraction of the peak on a Monday, and booking further ahead is nearly always cheaper than booking late. The operator's own page has today's figure - bring it back and I will work out what it comes to for you."), py_str("Booked far enough ahead, a fare is a different number from the one a week before, so compare the same journey on the day you actually go rather than the first figure you see. Tell me the route and the dates and I will do the sums against the money you have."), py_str("That price turns on the day, the time and how far ahead you book, and on a fare it is the time of day that moves it most. The figure is on the operator's own page; bring it back with what you can spend and I will tell you whether it stacks up.")]));
    s_A_CAB_OR_A_LIFT(stdlib_call("re", "compile", &[py_str("\\b(?:taxis?|cabs?|minicabs?|private hire|black cab|ubers?|lyft|a cab|a taxi)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAB_FARE_RULE(py_tuple(vec![py_str("A cab is metered, so it is the clock and the traffic that set the price, not the seller: the same trip at 8 in the morning and at 11 at night are different numbers. Ring the firm with the pickup, the drop-off and the time and ask for a fixed quote - most will give one, and that quote is the number worth comparing."), py_str("A taxi fare is the meter, so the time of day and the traffic move it; there is nothing to book further ahead. Ask two firms for a fixed price for your trip and compare those, and ask whether the quote covers waiting and the return.")]));
    s_PRICE_VERDICT_RULE(py_tuple(vec![py_str("Whether that is dear depends on which one and where from: the same thing is a different number in two places and on two days. Two prices, all in, is the comparison - bring them back and I will tell you which one leaves you better off at the end of the month."), py_str("There is no one price for it. What it comes to turns on the size, the seller and the day, so the useful thing is two quotes for the same thing with everything included. Tell me the two figures and I will work out what each leaves you."), py_str("That is impossible to call from one number: what it costs moves with which one you pick, where you get it and when. Find two prices for the same thing, all in, and the difference between them is the part worth acting on.")]));
    s_A_AN_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:weddings?|funerals?|christenings?|baptisms?|namings?|birthdays?|birthday party|part(?:y|ies)|anniversar(?:y|ies)|honeymoons?|stag do|stag night|hen do|hen night|graduations?|receptions?|eating out|eating away|meals out|meal out|dinner out|restaurants?|takeaways?|catering|caterers?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASION_PRICE_RULE(py_tuple(vec![py_str("That one is priced by the head and by the day rather than by the seller: how many people, which day, and what is inside the price - the meal, the drinks, the extras. Two quotes on the same numbers is the only comparison worth making; tell me yours and I will do the sums."), py_str("What it comes to turns on the count and the date, so fix the guest list and the day first, then ask two or three places for the same thing all in. Bring the figures back and I will work out what it leaves you a month."), py_str("Every one of these is quoted per person and moves with the day of the week, so one figure from one place says little on its own. The same count and the same extras asked of two places is the comparison - and the venue's own page has today's numbers.")]));
    s_NOT_ABOUT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\bmy (?:account|balance|bill|invoice|statement|payment|direct debit|order|parcel|quote|refund|payslip|wage|salary|pension|mortgage|rent|card|loan|overdraft|credit)\\b|\\b(?:up front|deposit|instal?ments?|per month|a month|a week|per hour|pay off|paid off|overdue)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AMOUNT_FEEDING_RULE(py_tuple(vec![py_str("That one is arithmetic rather than knowledge: a portion each, times how many you are feeding, and a little more if it is the only dish. The pack, the butcher or the recipe you are using carries the per-head figure - give me the count and I will turn it into a total."), py_str("Work it from the number of people, not from the whole: what one person gets, multiplied, and a bit on top if it is the only thing being served. Tell me how many and I will do the sum with you."), py_str("How much you need is per head, and whether it is the main thing or one of several decides which per-head figure: the pack or the recipe has that half. Bring me the numbers and I will work out what to buy.")]));
    s_AMOUNT_COVER_RULE(py_tuple(vec![py_str("That one is a coverage sum rather than a fact: how far one pack, tin or scoop goes, times the size of the job. How far it goes is on the label, and the size of the job is yours to measure - tell me it and I will work out how many you need."), py_str("How much you need comes from how much one of them covers and how big the job is, so start with the coverage figure on the label and then it is arithmetic. Give me the measurements and I will do the working."), py_str("The label says what one pack covers and the rest is a multiplication: that, times the area or the number of loads you are doing. Tell me the two numbers and I will turn them into how much to buy.")]));
    s_AMOUNT_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\b[^?.!]{0,40}?\\b(?:do|should|would|will|might|can|shall)\\b[^?.!]{0,12}?\\b(?:i|we|you) (?:need|use|buy|get|make|cook|add|put|give|take|order|hire)\\b|\\bhow (?:much|many) (?:do|does) (?:i|we|you) need\\b|\\bhow (?:much|many)\\b[^?.!]{0,30}?\\bfor (?:two|three|four|five|six|seven|eight|nine|ten|twelve|twenty|\\d+)\\b|\\bhow (?:heavy|big|large|thick) should\\b|\\bhow many (?:portions|servings|heads)\\b|\\bhow much per (?:person|head|portion)\\b|\\bper (?:head|person)\\b|\\bhow much\\b[^?.!]{0,28}?\\b(?:goes|goes in|goes into)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEEDING(stdlib_call("re", "compile", &[py_str("\\b(?:people|person|persons|head|heads|portion|portions|serving|servings|guests?|kids?|children|adults?|family|each|everyone|for (?:two|three|four|five|six|seven|eight|nine|ten|twelve|twenty|\\d+))\\b|\\b(?:eat|eating|feed|feeding|dinner|lunch|supper|tea|buffet|barbecue|bbq)\\b|\\b(?:meat|chicken|beef|pork|lamb|mince|rice|pasta|potato|potatoes|veg|vegetables?|salad|soup|sauce|gravy|stuffing|bread|pastry|cheese|milk|cream|fruit|berries|cake|biscuits?|pudding|dessert|starter|curry|stew|lasagne|lasagna|pie|quiche|sausages?|bacon|eggs?|fish|salmon|prawns|prawn|prawns|cheese|butter|flour|sugar|oats|yoghurt|yogurt|beans?|lentils?|chickpeas|nuts?|spuds|chips|roast|joint|turkey|duck|ham|tofu|hummus)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MATERIAL(stdlib_call("re", "compile", &[py_str("\\b(?:paint|primer|undercoat|gloss|emulsion|wallpaper|paste|tiles?|grout|adhesive|sealant|filler|plaster|cement|mortar|sand|gravel|bark|chippings|soil|compost|mulch|feed|fertiliser|fertilizer|weedkiller|seeds?|grass seed|oil|varnish|stain|lacquer|thinner|white spirit|bleach|detergent|washing powder|liquid|tablets|conditioner|softener|disinfectant|cleaner|shampoo|foil|cling film|bin bags?|kitchen roll|cat litter|litter|screws?|nails?|rawlplugs|dowels?|cable|wire|rope|string|tape)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_ON_A_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:up front|deposit|pay|paid|paying|money|afford|budget|cost|costs|price|[£$€])\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_GOOD_RULE(py_tuple(vec![py_str("Whether it is still good turns on two things: how long it has been and how cold it stayed. The date on the pack is the line - past it, or warm for hours, it goes - and I will say what I would do with the details: which it is, and when it went in."), py_str("Two things decide that, and neither is on my side of the screen: how long, and how it has been kept. The pack's own date is the limit; inside it and cold all along is fine, and past it or out for half a day is not worth the risk. The date on it is the one to read, and tell me that and when it went in and I will say what I would do."), py_str("That is a keeping question rather than a recipe one: the date to go by is the one on the pack, and how it has been kept since decides the rest. Tell me when it went in and how it has been stored, and I will say what I would do.")]));
    s_GONE_OFF_ASK(stdlib_call("re", "compile", &[py_str("\\bgone off\\b|\\bout of date\\b|\\b(?:use by|best before|sell by)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ASK(stdlib_call("re", "compile", &[py_str("\\bstill\\b[^?.!]{0,40}?\\b(?:ok|okay|fine|safe|alright|good|eat|eaten|use|using|keep|have|drink|give|serve|cook|reheat|freeze|edible)\\b|\\b(?:it|that|this|they|these)\\b[^?.!]{0,30}?\\bstill\\b|\\bgone off\\b|\\bout of date\\b|\\bpast (?:its|the|their) (?:date|use by|best before)\\b|\\buse by\\b|\\bbest before\\b|\\bhow long (?:does|do|will|can|would)\\b[^?.!]{0,30}?\\b(?:keep|last|stay good|stay fresh)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long (?:does|do|will|can|would)\\b[^?.!]{0,30}?\\b(?:keep|stay fresh|stay good)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_ASK(stdlib_call("re", "compile", &[py_str("\\bis it (?:ok|okay|fine|safe|alright|good) (?:to|for)\\b|\\bcan i (?:eat|use|keep|drink|have|give|serve|freeze|reheat|cook)\\b|\\bis (?:it|that|this) (?:ok|okay|fine|safe|alright)\\b|\\bwill (?:it|that|they|these) be (?:ok|okay|fine|alright|safe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_KEEPING_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:medicine|medicines|tablets?|pills?|capsules?|dose|doses|dosage|paracetamol|ibuprofen|calpol|antibiotics?|inhaler|insulin|syrup|prescription|allergy|allergic)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPING_HAS_NO_DATE(stdlib_call("re", "compile", &[py_str("\\b(?:freeze|freezes|freezer|freezers|freezing|frozen|thaw|thaws|thawed|thawing|defrost|defrosted|refreeze|re-freeze)\\b|\\b(?:i|we) (?:made|make|baked|cooked|roasted|prepared|whipped up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_A_PERSON_OR_AN_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|infant|newborn|toddler|child|children|boy|girl|son|daughter|kid|kids|pup|puppy|kitten|dog|cat|months? old|years? old|weigh|weighs|weight|stone|kg|lbs|pounds)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_OR_PACK(stdlib_call("re", "compile", &[py_str("\\b(?:chicken|beef|pork|lamb|turkey|duck|ham|bacon|mince|sausage|sausages|fish|salmon|prawns|shellfish|rice|pasta|noodles|potato|potatoes|veg|vegetables?|salad|soup|sauce|gravy|bread|toast|pastry|cheese|milk|cream|yoghurt|yogurt|butter|eggs?|fruit|berries|cake|quiche|pie|pudding|leftovers?|takeaway|curry|stew|lasagne|lasagna|sandwich|meat|food|pack|packet|tin|jar|bottle|carton|use by|best before|date on it|label)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_RULE(py_tuple(vec![py_str("A time is a rate times a size: minutes per kilo, hours per coat, days a chapter. Whoever sets the rate - the recipe, the course, the trade - says what it is, and give me your size and I will do the multiplication."), py_str("How long it takes comes from how big the job is and how fast it goes, so the number to find is the rate rather than the total, and the rate is published - on the pack, in the recipe, or by whoever does the work. Tell me the two figures and I will work out the time."), py_str("Time is the part I can work out but cannot know: the size of the job, times the rate it goes at. Bring me the rate from whoever does it and the size of yours, and I will turn it into a finish time you can plan round.")]));
    s_TIME_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long (?:will|would|does|do|should|shall|can|could|might|must)\\b[^?.!]{0,44}?\\b(?:take|takes|last|lasts|go in|sit|rest|stay in|be in|preheat|defrost|chill|set|rise|prove|boil|simmer|roast|cook|bake|fry|grill|steam|soak|marinate|dry|run|walk|drive|train|learn|build|fit|paint|decorate)\\b|\\bhow long (?:is|would be) (?:it|that|this)\\b[^?.!]{0,30}?\\b(?:before|until|to)\\b|\\bhow long (?:until|before)\\b[^?.!]{0,40}?\\b(?:done|ready|cooked|finished|it is|its)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:ago|been|queue|waiting list|on hold|does it keep|how long have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COOKING_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:roast|roasting|cook|cooking|bake|baking|boil|boiling|simmer|fry|frying|grill|grilling|steam|steaming|poach|oven|go in|goes in|be in|sit in|stay in|rest|barbecue|bbq|air fryer|slow cooker|pressure cooker|pan)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:lamb|beef|chicken|turkey|duck|goose|pork|ham|gammon|bacon|sausages?|mince|steak|joint|bird|casserole|stew|pie|tart|cake|sponge|bread|loaf|scones?|biscuits?|pudding|potatoes?|chips?|veg|vegetables?|carrots?|parsnips?|onions?|rice|pasta|lasagne|lasagna|eggs?|fish|salmon|trout|cod|prawns?|tofu|halloumi|mushrooms?|tomatoes?|peppers?|dough|batter|pastry)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CARE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:operation|surgery|theatre|hospital|hospitalised|ward|vets?|vet's|recovery|recovering|rehab|treatment|chemo|chemotherapy|dialysis|scans?|consultant|nurse|nurses|doctor|doctors|gp|111|a&e|icu|hospice|palliative|dementia|carer|carers|in labour|due date|pregnant|pregnancy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:credit card|credit cards|card|cards|apr|loan|loans|mortgage|mortgages|overdraft|debt|debts|interest|repayments?|overpay|overpaying|arrears|rent arrears|invoice|bailiffs?|iva|ccj|payday|minimum payment|balance)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COOKING_TIME_RULE(py_tuple(vec![py_str("Oven times go by weight and heat: minutes per 500g at a set temperature, and both are on the pack or in the recipe. Tell me the weight and the heat and I will work out the clock time for you."), py_str("That one is a minutes-per-500g job at a set heat, and the figure for the thing you are cooking is on the pack, the recipe or the book. How heavy is it, and what heat are you using?"), py_str("The weight and the oven heat decide it, and the rate for that joint or bird is written down somewhere you already have - the pack, the recipe, or the shop's own guide. Bring me the weight and I will do the arithmetic.")]));
    s_RULE_POOLS(py_tuple(vec![g_PRICE_RULE(), g_PRICE_VERDICT_RULE(), g_FARE_PRICE_RULE(), g_CAB_FARE_RULE(), g_OCCASION_PRICE_RULE(), g_AMOUNT_FEEDING_RULE(), g_AMOUNT_COVER_RULE(), g_STILL_GOOD_RULE(), g_COOKING_TIME_RULE(), g_TIME_RULE()]));
    s_RULE_HEADS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RULE_POOLS()) { let mut pool: Py = __item.clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&line.clone(), &Py::None, &Py::Int(40), &Py::None)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_rule_for_it(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pool for the kind of ask this line is, or None when it is not one of the kinds.\n\n    Read only for a line that is about to be shrugged at, and read on the repaired line as well\n    as the typed one, the way every other pool here is: \"how mutch does a tattoo cost\" is the\n    same ask.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_KEEPING_QUESTION(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ABOUT_A_PERSON_OR_AN_ANIMAL(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_KEEPING_HAS_NO_DATE(), "search", &[said.clone()])))) {
    if py_truthy(&{ let __b = mcall(&g_KEEP_ASK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_GONE_OFF_ASK(), "search", &[said.clone()]) } }) {
    return g_STILL_GOOD_RULE();
    }
    if py_truthy(&{ let __b = mcall(&g_STILL_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_FOOD_OR_PACK(), "search", &[said.clone()]) } }) {
    return g_STILL_GOOD_RULE();
    }
    if py_truthy(&{ let __b = mcall(&g_OFF_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_FOOD_OR_PACK(), "search", &[said.clone()]) } }) {
    return g_STILL_GOOD_RULE();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_AMOUNT_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MONEY_ON_A_JOB(), "search", &[said.clone()]))) } }) {
    if py_truthy(&{ let __b = mcall(&g_MATERIAL(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FEEDING(), "search", &[said.clone()]))) } }) {
    return g_AMOUNT_COVER_RULE();
    }
    if py_truthy(&mcall(&g_FEEDING(), "search", &[said.clone()])) {
    return g_AMOUNT_FEEDING_RULE();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_PRICE_VERDICT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_ABOUT_A_PRICE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DEAR_AS_A_NAME(), "search", &[said.clone()]))) } } } }) {
    if py_truthy(&mcall(&g_A_CAB_OR_A_LIFT(), "search", &[said.clone()])) {
    return g_CAB_FARE_RULE();
    }
    if py_truthy(&mcall(&g_A_AN_OCCASION(), "search", &[said.clone()])) {
    return g_OCCASION_PRICE_RULE();
    }
    if py_truthy(&mcall(&g_A_FARE_OR_A_TICKET(), "search", &[said.clone()])) {
    return g_FARE_PRICE_RULE();
    }
    return g_PRICE_VERDICT_RULE();
    }
    if py_truthy(&{ let __b = mcall(&g_PRICE_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_ABOUT_A_PRICE(), "search", &[said.clone()]))) } }) {
    if py_truthy(&mcall(&g_A_CAB_OR_A_LIFT(), "search", &[said.clone()])) {
    return g_CAB_FARE_RULE();
    }
    if py_truthy(&mcall(&g_A_AN_OCCASION(), "search", &[said.clone()])) {
    return g_OCCASION_PRICE_RULE();
    }
    return g_PRICE_RULE();
    }
    if py_truthy(&{ let __b = mcall(&g_TIME_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TIME_NOT_THIS(), "search", &[said.clone()]))) } }) {
    if py_truthy(&{ let __b = mcall(&g_A_COOKING_TIME(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_FOOD(), "search", &[said.clone()]) } }) {
    return g_COOKING_TIME_RULE();
    }
    return g_TIME_RULE();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.the_rule_for_it".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_A_PERSON_OR_AN_ANIMAL" => Some(g_ABOUT_A_PERSON_OR_AN_ANIMAL()),
            "AMOUNT_ASK" => Some(g_AMOUNT_ASK()),
            "AMOUNT_COVER_RULE" => Some(g_AMOUNT_COVER_RULE()),
            "AMOUNT_FEEDING_RULE" => Some(g_AMOUNT_FEEDING_RULE()),
            "A_AN_OCCASION" => Some(g_A_AN_OCCASION()),
            "A_CAB_OR_A_LIFT" => Some(g_A_CAB_OR_A_LIFT()),
            "A_CARE_WORD" => Some(g_A_CARE_WORD()),
            "A_COOKING_TIME" => Some(g_A_COOKING_TIME()),
            "A_DEAR_AS_A_NAME" => Some(g_A_DEAR_AS_A_NAME()),
            "A_FARE_OR_A_TICKET" => Some(g_A_FARE_OR_A_TICKET()),
            "A_FOOD" => Some(g_A_FOOD()),
            "A_MONEY_WORD" => Some(g_A_MONEY_WORD()),
            "CAB_FARE_RULE" => Some(g_CAB_FARE_RULE()),
            "COOKING_TIME_RULE" => Some(g_COOKING_TIME_RULE()),
            "FARE_PRICE_RULE" => Some(g_FARE_PRICE_RULE()),
            "FEEDING" => Some(g_FEEDING()),
            "FOOD_OR_PACK" => Some(g_FOOD_OR_PACK()),
            "GONE_OFF_ASK" => Some(g_GONE_OFF_ASK()),
            "KEEPING_HAS_NO_DATE" => Some(g_KEEPING_HAS_NO_DATE()),
            "KEEP_ASK" => Some(g_KEEP_ASK()),
            "MATERIAL" => Some(g_MATERIAL()),
            "MONEY_ON_A_JOB" => Some(g_MONEY_ON_A_JOB()),
            "NOT_ABOUT_A_PRICE" => Some(g_NOT_ABOUT_A_PRICE()),
            "NOT_A_KEEPING_QUESTION" => Some(g_NOT_A_KEEPING_QUESTION()),
            "OCCASION_PRICE_RULE" => Some(g_OCCASION_PRICE_RULE()),
            "OFF_ASK" => Some(g_OFF_ASK()),
            "PRICE_ASK" => Some(g_PRICE_ASK()),
            "PRICE_RULE" => Some(g_PRICE_RULE()),
            "PRICE_VERDICT" => Some(g_PRICE_VERDICT()),
            "PRICE_VERDICT_RULE" => Some(g_PRICE_VERDICT_RULE()),
            "RULE_HEADS" => Some(g_RULE_HEADS()),
            "RULE_POOLS" => Some(g_RULE_POOLS()),
            "STILL_ASK" => Some(g_STILL_ASK()),
            "STILL_GOOD_RULE" => Some(g_STILL_GOOD_RULE()),
            "TIME_ASK" => Some(g_TIME_ASK()),
            "TIME_NOT_THIS" => Some(g_TIME_NOT_THIS()),
            "TIME_RULE" => Some(g_TIME_RULE()),
            "the_rule_for_it" => Some(py_func("the_rule_for_it", |args: &[Py]| the_rule_for_it(args))),
            _ => None,
        }),
        set: None,
    }))
}

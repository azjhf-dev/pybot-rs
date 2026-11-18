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


thread_local! { static G_ABOUT_A_PERSON_OR_AN_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_COVER_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_FEEDING_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_AN_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CAB_OR_A_LIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COOKING_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEAR_AS_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FARE_OR_A_TICKET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAB_FARE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_TIME_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FARE_PRICE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_OR_PACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GONE_OFF_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_HAS_NO_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MATERIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_ON_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_A_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_KEEPING_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION_PRICE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_VERDICT_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULE_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULE_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_GOOD_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.the_rule_for_it: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_A_PERSON_OR_AN_ANIMAL() -> Py { ensure_init(); G_ABOUT_A_PERSON_OR_AN_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_A_PERSON_OR_AN_ANIMAL(v: Py) { ensure_init(); G_ABOUT_A_PERSON_OR_AN_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_ASK() -> Py { ensure_init(); G_AMOUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_ASK(v: Py) { ensure_init(); G_AMOUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_COVER_RULE() -> Py { ensure_init(); G_AMOUNT_COVER_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_COVER_RULE(v: Py) { ensure_init(); G_AMOUNT_COVER_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_FEEDING_RULE() -> Py { ensure_init(); G_AMOUNT_FEEDING_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_FEEDING_RULE(v: Py) { ensure_init(); G_AMOUNT_FEEDING_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_AN_OCCASION() -> Py { ensure_init(); G_A_AN_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AN_OCCASION(v: Py) { ensure_init(); G_A_AN_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CAB_OR_A_LIFT() -> Py { ensure_init(); G_A_CAB_OR_A_LIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CAB_OR_A_LIFT(v: Py) { ensure_init(); G_A_CAB_OR_A_LIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_WORD() -> Py { ensure_init(); G_A_CARE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_WORD(v: Py) { ensure_init(); G_A_CARE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COOKING_TIME() -> Py { ensure_init(); G_A_COOKING_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COOKING_TIME(v: Py) { ensure_init(); G_A_COOKING_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEAR_AS_A_NAME() -> Py { ensure_init(); G_A_DEAR_AS_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEAR_AS_A_NAME(v: Py) { ensure_init(); G_A_DEAR_AS_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FARE_OR_A_TICKET() -> Py { ensure_init(); G_A_FARE_OR_A_TICKET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FARE_OR_A_TICKET(v: Py) { ensure_init(); G_A_FARE_OR_A_TICKET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FOOD() -> Py { ensure_init(); G_A_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FOOD(v: Py) { ensure_init(); G_A_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); G_A_MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); G_A_MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_CAB_FARE_RULE() -> Py { ensure_init(); G_CAB_FARE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAB_FARE_RULE(v: Py) { ensure_init(); G_CAB_FARE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_TIME_RULE() -> Py { ensure_init(); G_COOKING_TIME_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_TIME_RULE(v: Py) { ensure_init(); G_COOKING_TIME_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_FARE_PRICE_RULE() -> Py { ensure_init(); G_FARE_PRICE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FARE_PRICE_RULE(v: Py) { ensure_init(); G_FARE_PRICE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEDING() -> Py { ensure_init(); G_FEEDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEDING(v: Py) { ensure_init(); G_FEEDING.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_OR_PACK() -> Py { ensure_init(); G_FOOD_OR_PACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_OR_PACK(v: Py) { ensure_init(); G_FOOD_OR_PACK.with(|c| *c.borrow_mut() = v); }
pub fn g_GONE_OFF_ASK() -> Py { ensure_init(); G_GONE_OFF_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GONE_OFF_ASK(v: Py) { ensure_init(); G_GONE_OFF_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_HAS_NO_DATE() -> Py { ensure_init(); G_KEEPING_HAS_NO_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_HAS_NO_DATE(v: Py) { ensure_init(); G_KEEPING_HAS_NO_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_ASK() -> Py { ensure_init(); G_KEEP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_ASK(v: Py) { ensure_init(); G_KEEP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MATERIAL() -> Py { ensure_init(); G_MATERIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MATERIAL(v: Py) { ensure_init(); G_MATERIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_ON_A_JOB() -> Py { ensure_init(); G_MONEY_ON_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_ON_A_JOB(v: Py) { ensure_init(); G_MONEY_ON_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_A_PRICE() -> Py { ensure_init(); G_NOT_ABOUT_A_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_A_PRICE(v: Py) { ensure_init(); G_NOT_ABOUT_A_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_KEEPING_QUESTION() -> Py { ensure_init(); G_NOT_A_KEEPING_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_KEEPING_QUESTION(v: Py) { ensure_init(); G_NOT_A_KEEPING_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION_PRICE_RULE() -> Py { ensure_init(); G_OCCASION_PRICE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION_PRICE_RULE(v: Py) { ensure_init(); G_OCCASION_PRICE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_ASK() -> Py { ensure_init(); G_OFF_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_ASK(v: Py) { ensure_init(); G_OFF_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_ASK() -> Py { ensure_init(); G_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); G_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_RULE() -> Py { ensure_init(); G_PRICE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_RULE(v: Py) { ensure_init(); G_PRICE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_VERDICT() -> Py { ensure_init(); G_PRICE_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_VERDICT(v: Py) { ensure_init(); G_PRICE_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_VERDICT_RULE() -> Py { ensure_init(); G_PRICE_VERDICT_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_VERDICT_RULE(v: Py) { ensure_init(); G_PRICE_VERDICT_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULE_HEADS() -> Py { ensure_init(); G_RULE_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULE_HEADS(v: Py) { ensure_init(); G_RULE_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_RULE_POOLS() -> Py { ensure_init(); G_RULE_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULE_POOLS(v: Py) { ensure_init(); G_RULE_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_ASK() -> Py { ensure_init(); G_STILL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_ASK(v: Py) { ensure_init(); G_STILL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_GOOD_RULE() -> Py { ensure_init(); G_STILL_GOOD_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_GOOD_RULE(v: Py) { ensure_init(); G_STILL_GOOD_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASK() -> Py { ensure_init(); G_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASK(v: Py) { ensure_init(); G_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_NOT_THIS() -> Py { ensure_init(); G_TIME_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_NOT_THIS(v: Py) { ensure_init(); G_TIME_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_RULE() -> Py { ensure_init(); G_TIME_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_RULE(v: Py) { ensure_init(); G_TIME_RULE.with(|c| *c.borrow_mut() = v); }
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
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_A_PERSON_OR_AN_ANIMAL" => s_ABOUT_A_PERSON_OR_AN_ANIMAL(v),
            "AMOUNT_ASK" => s_AMOUNT_ASK(v),
            "AMOUNT_COVER_RULE" => s_AMOUNT_COVER_RULE(v),
            "AMOUNT_FEEDING_RULE" => s_AMOUNT_FEEDING_RULE(v),
            "A_AN_OCCASION" => s_A_AN_OCCASION(v),
            "A_CAB_OR_A_LIFT" => s_A_CAB_OR_A_LIFT(v),
            "A_CARE_WORD" => s_A_CARE_WORD(v),
            "A_COOKING_TIME" => s_A_COOKING_TIME(v),
            "A_DEAR_AS_A_NAME" => s_A_DEAR_AS_A_NAME(v),
            "A_FARE_OR_A_TICKET" => s_A_FARE_OR_A_TICKET(v),
            "A_FOOD" => s_A_FOOD(v),
            "A_MONEY_WORD" => s_A_MONEY_WORD(v),
            "CAB_FARE_RULE" => s_CAB_FARE_RULE(v),
            "COOKING_TIME_RULE" => s_COOKING_TIME_RULE(v),
            "FARE_PRICE_RULE" => s_FARE_PRICE_RULE(v),
            "FEEDING" => s_FEEDING(v),
            "FOOD_OR_PACK" => s_FOOD_OR_PACK(v),
            "GONE_OFF_ASK" => s_GONE_OFF_ASK(v),
            "KEEPING_HAS_NO_DATE" => s_KEEPING_HAS_NO_DATE(v),
            "KEEP_ASK" => s_KEEP_ASK(v),
            "MATERIAL" => s_MATERIAL(v),
            "MONEY_ON_A_JOB" => s_MONEY_ON_A_JOB(v),
            "NOT_ABOUT_A_PRICE" => s_NOT_ABOUT_A_PRICE(v),
            "NOT_A_KEEPING_QUESTION" => s_NOT_A_KEEPING_QUESTION(v),
            "OCCASION_PRICE_RULE" => s_OCCASION_PRICE_RULE(v),
            "OFF_ASK" => s_OFF_ASK(v),
            "PRICE_ASK" => s_PRICE_ASK(v),
            "PRICE_RULE" => s_PRICE_RULE(v),
            "PRICE_VERDICT" => s_PRICE_VERDICT(v),
            "PRICE_VERDICT_RULE" => s_PRICE_VERDICT_RULE(v),
            "RULE_HEADS" => s_RULE_HEADS(v),
            "RULE_POOLS" => s_RULE_POOLS(v),
            "STILL_ASK" => s_STILL_ASK(v),
            "STILL_GOOD_RULE" => s_STILL_GOOD_RULE(v),
            "TIME_ASK" => s_TIME_ASK(v),
            "TIME_NOT_THIS" => s_TIME_NOT_THIS(v),
            "TIME_RULE" => s_TIME_RULE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.the_rule_for_it", module_obj);
    pyrt::xrefs::reg_fn("compose.the_rule_for_it", "the_rule_for_it", py_func("the_rule_for_it", |args: &[Py]| the_rule_for_it(args)));
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "ABOUT_A_PERSON_OR_AN_ANIMAL", g_ABOUT_A_PERSON_OR_AN_ANIMAL, s_ABOUT_A_PERSON_OR_AN_ANIMAL);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "AMOUNT_ASK", g_AMOUNT_ASK, s_AMOUNT_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "AMOUNT_COVER_RULE", g_AMOUNT_COVER_RULE, s_AMOUNT_COVER_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "AMOUNT_FEEDING_RULE", g_AMOUNT_FEEDING_RULE, s_AMOUNT_FEEDING_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_AN_OCCASION", g_A_AN_OCCASION, s_A_AN_OCCASION);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_CAB_OR_A_LIFT", g_A_CAB_OR_A_LIFT, s_A_CAB_OR_A_LIFT);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_CARE_WORD", g_A_CARE_WORD, s_A_CARE_WORD);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_COOKING_TIME", g_A_COOKING_TIME, s_A_COOKING_TIME);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_DEAR_AS_A_NAME", g_A_DEAR_AS_A_NAME, s_A_DEAR_AS_A_NAME);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_FARE_OR_A_TICKET", g_A_FARE_OR_A_TICKET, s_A_FARE_OR_A_TICKET);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_FOOD", g_A_FOOD, s_A_FOOD);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "A_MONEY_WORD", g_A_MONEY_WORD, s_A_MONEY_WORD);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "CAB_FARE_RULE", g_CAB_FARE_RULE, s_CAB_FARE_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "COOKING_TIME_RULE", g_COOKING_TIME_RULE, s_COOKING_TIME_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "FARE_PRICE_RULE", g_FARE_PRICE_RULE, s_FARE_PRICE_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "FEEDING", g_FEEDING, s_FEEDING);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "FOOD_OR_PACK", g_FOOD_OR_PACK, s_FOOD_OR_PACK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "GONE_OFF_ASK", g_GONE_OFF_ASK, s_GONE_OFF_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "KEEPING_HAS_NO_DATE", g_KEEPING_HAS_NO_DATE, s_KEEPING_HAS_NO_DATE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "KEEP_ASK", g_KEEP_ASK, s_KEEP_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "MATERIAL", g_MATERIAL, s_MATERIAL);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "MONEY_ON_A_JOB", g_MONEY_ON_A_JOB, s_MONEY_ON_A_JOB);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "NOT_ABOUT_A_PRICE", g_NOT_ABOUT_A_PRICE, s_NOT_ABOUT_A_PRICE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "NOT_A_KEEPING_QUESTION", g_NOT_A_KEEPING_QUESTION, s_NOT_A_KEEPING_QUESTION);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "OCCASION_PRICE_RULE", g_OCCASION_PRICE_RULE, s_OCCASION_PRICE_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "OFF_ASK", g_OFF_ASK, s_OFF_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "PRICE_ASK", g_PRICE_ASK, s_PRICE_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "PRICE_RULE", g_PRICE_RULE, s_PRICE_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "PRICE_VERDICT", g_PRICE_VERDICT, s_PRICE_VERDICT);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "PRICE_VERDICT_RULE", g_PRICE_VERDICT_RULE, s_PRICE_VERDICT_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "RULE_HEADS", g_RULE_HEADS, s_RULE_HEADS);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "RULE_POOLS", g_RULE_POOLS, s_RULE_POOLS);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "STILL_ASK", g_STILL_ASK, s_STILL_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "STILL_GOOD_RULE", g_STILL_GOOD_RULE, s_STILL_GOOD_RULE);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "TIME_ASK", g_TIME_ASK, s_TIME_ASK);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "TIME_NOT_THIS", g_TIME_NOT_THIS, s_TIME_NOT_THIS);
    pyrt::xrefs::reg_access("compose.the_rule_for_it", "TIME_RULE", g_TIME_RULE, s_TIME_RULE);
}

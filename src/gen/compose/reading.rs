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


pub static ABOUT_RIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOVE_THE_RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOVE_THE_USUAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALREADY_JUDGED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_AGE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FOR_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FOR_A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BILL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CEILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLOCK_TIME_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CLOCK_TIME_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DISTANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GAUGE_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_JOURNEY_IS_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LONE_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERIOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_NEXT_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RANGE_THAT_IS_NOT_A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE_OF_THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STRETCH_OF_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WORD_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_CEILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_DOUBTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BELOW_THE_RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARRIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAYS_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOUBT_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOUBT_UNITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOUBT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENOUGH_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_THE_ARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIGH_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOK_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOW_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASURED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_DOUBTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MORE_THAN_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEXT_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOO_LITTLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOO_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOO_LONG_INSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOO_MUCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TOO_SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_PRICE_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_HIGH_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_LONG_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_LOW_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_SHORT_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PER_UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACES_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLURALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTING_AT_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTING_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUANTITY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RIGHT_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_HIGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_LOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAME_LEAGUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAG_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAILS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_LOW_SIDE_FOUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_LOW_SIDE_FOUND_FOR_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PRICE_IT_GAVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_RANGE_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_REST_IS_UNSURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_USUAL_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_DOUBT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOO_LITTLE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOO_MUCH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNDER_THE_USUAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNIT_ALIAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNIT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD_ALIAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD_FIGURE_COUNTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD_NUMBERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD_RANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASIDE_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BUYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_AT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAME_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAYS_NOTHING_ABOUT_THE_TIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_RIGHT() -> Py { ensure_init(); ABOUT_RIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_RIGHT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_RIGHT.write() { *w = v; } }
pub fn g_ABOVE_THE_RANGE() -> Py { ensure_init(); ABOVE_THE_RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOVE_THE_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = ABOVE_THE_RANGE.write() { *w = v; } }
pub fn g_ABOVE_THE_USUAL() -> Py { ensure_init(); ABOVE_THE_USUAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOVE_THE_USUAL(v: Py) { ensure_init(); if let Ok(mut w) = ABOVE_THE_USUAL.write() { *w = v; } }
pub fn g_ALREADY_JUDGED() -> Py { ensure_init(); ALREADY_JUDGED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALREADY_JUDGED(v: Py) { ensure_init(); if let Ok(mut w) = ALREADY_JUDGED.write() { *w = v; } }
pub fn g_AN_AGE() -> Py { ensure_init(); AN_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AGE(v: Py) { ensure_init(); if let Ok(mut w) = AN_AGE.write() { *w = v; } }
pub fn g_AN_AGE_AFTER() -> Py { ensure_init(); AN_AGE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AGE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = AN_AGE_AFTER.write() { *w = v; } }
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_ASKS_FOR_A_FIGURE() -> Py { ensure_init(); ASKS_FOR_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FOR_A_FIGURE.write() { *w = v; } }
pub fn g_ASKS_FOR_A_PRICE() -> Py { ensure_init(); ASKS_FOR_A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FOR_A_PRICE.write() { *w = v; } }
pub fn g_A_BILL() -> Py { ensure_init(); A_BILL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BILL(v: Py) { ensure_init(); if let Ok(mut w) = A_BILL.write() { *w = v; } }
pub fn g_A_CEILING() -> Py { ensure_init(); A_CEILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CEILING(v: Py) { ensure_init(); if let Ok(mut w) = A_CEILING.write() { *w = v; } }
pub fn g_A_CLOCK_TIME_AFTER() -> Py { ensure_init(); A_CLOCK_TIME_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLOCK_TIME_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = A_CLOCK_TIME_AFTER.write() { *w = v; } }
pub fn g_A_CLOCK_TIME_BEFORE() -> Py { ensure_init(); A_CLOCK_TIME_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CLOCK_TIME_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = A_CLOCK_TIME_BEFORE.write() { *w = v; } }
pub fn g_A_DISTANCE() -> Py { ensure_init(); A_DISTANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DISTANCE(v: Py) { ensure_init(); if let Ok(mut w) = A_DISTANCE.write() { *w = v; } }
pub fn g_A_GAUGE_UNIT() -> Py { ensure_init(); A_GAUGE_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GAUGE_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = A_GAUGE_UNIT.write() { *w = v; } }
pub fn g_A_JOURNEY_IS_TIME() -> Py { ensure_init(); A_JOURNEY_IS_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_JOURNEY_IS_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_JOURNEY_IS_TIME.write() { *w = v; } }
pub fn g_A_LONE_PRICE() -> Py { ensure_init(); A_LONE_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LONE_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = A_LONE_PRICE.write() { *w = v; } }
pub fn g_A_MONEY_PHRASE() -> Py { ensure_init(); A_MONEY_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_PHRASE.write() { *w = v; } }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); A_MONEY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_WORD.write() { *w = v; } }
pub fn g_A_PERIOD() -> Py { ensure_init(); A_PERIOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERIOD(v: Py) { ensure_init(); if let Ok(mut w) = A_PERIOD.write() { *w = v; } }
pub fn g_A_PRICE_NEXT_TO() -> Py { ensure_init(); A_PRICE_NEXT_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_NEXT_TO(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_NEXT_TO.write() { *w = v; } }
pub fn g_A_PRICE_VERB() -> Py { ensure_init(); A_PRICE_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_VERB.write() { *w = v; } }
pub fn g_A_RANGE_THAT_IS_NOT_A_PRICE() -> Py { ensure_init(); A_RANGE_THAT_IS_NOT_A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RANGE_THAT_IS_NOT_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = A_RANGE_THAT_IS_NOT_A_PRICE.write() { *w = v; } }
pub fn g_A_RATE_OF_THEIRS() -> Py { ensure_init(); A_RATE_OF_THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE_OF_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE_OF_THEIRS.write() { *w = v; } }
pub fn g_A_STRETCH() -> Py { ensure_init(); A_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = A_STRETCH.write() { *w = v; } }
pub fn g_A_STRETCH_OF_TIME() -> Py { ensure_init(); A_STRETCH_OF_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STRETCH_OF_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_STRETCH_OF_TIME.write() { *w = v; } }
pub fn g_A_WORD_FIGURE() -> Py { ensure_init(); A_WORD_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WORD_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = A_WORD_FIGURE.write() { *w = v; } }
pub fn g_BARE_CEILING() -> Py { ensure_init(); BARE_CEILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_CEILING(v: Py) { ensure_init(); if let Ok(mut w) = BARE_CEILING.write() { *w = v; } }
pub fn g_BARE_DOUBTS() -> Py { ensure_init(); BARE_DOUBTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_DOUBTS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_DOUBTS.write() { *w = v; } }
pub fn g_BELOW_THE_RANGE() -> Py { ensure_init(); BELOW_THE_RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BELOW_THE_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = BELOW_THE_RANGE.write() { *w = v; } }
pub fn g_CARRIED() -> Py { ensure_init(); CARRIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARRIED(v: Py) { ensure_init(); if let Ok(mut w) = CARRIED.write() { *w = v; } }
pub fn g_DAYS_IN() -> Py { ensure_init(); DAYS_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAYS_IN(v: Py) { ensure_init(); if let Ok(mut w) = DAYS_IN.write() { *w = v; } }
pub fn g_DOUBT_UNIT() -> Py { ensure_init(); DOUBT_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBT_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = DOUBT_UNIT.write() { *w = v; } }
pub fn g_DOUBT_UNITS() -> Py { ensure_init(); DOUBT_UNITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBT_UNITS(v: Py) { ensure_init(); if let Ok(mut w) = DOUBT_UNITS.write() { *w = v; } }
pub fn g_DOUBT_WORDS() -> Py { ensure_init(); DOUBT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOUBT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DOUBT_WORDS.write() { *w = v; } }
pub fn g_ENOUGH() -> Py { ensure_init(); ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = ENOUGH.write() { *w = v; } }
pub fn g_ENOUGH_FOR_THE_THING() -> Py { ensure_init(); ENOUGH_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENOUGH_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = ENOUGH_FOR_THE_THING.write() { *w = v; } }
pub fn g_FIGURE() -> Py { ensure_init(); FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE.write() { *w = v; } }
pub fn g_FOR_THE_ARTICLE() -> Py { ensure_init(); FOR_THE_ARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_THE_ARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = FOR_THE_ARTICLE.write() { *w = v; } }
pub fn g_FOR_THE_THING() -> Py { ensure_init(); FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = FOR_THE_THING.write() { *w = v; } }
pub fn g_HIGH_FOR_THE_THING() -> Py { ensure_init(); HIGH_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIGH_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = HIGH_FOR_THE_THING.write() { *w = v; } }
pub fn g_INSIDE() -> Py { ensure_init(); INSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSIDE(v: Py) { ensure_init(); if let Ok(mut w) = INSIDE.write() { *w = v; } }
pub fn g_LEADING() -> Py { ensure_init(); LEADING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADING(v: Py) { ensure_init(); if let Ok(mut w) = LEADING.write() { *w = v; } }
pub fn g_LOOK_BACK() -> Py { ensure_init(); LOOK_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); if let Ok(mut w) = LOOK_BACK.write() { *w = v; } }
pub fn g_LOW_FOR_THE_THING() -> Py { ensure_init(); LOW_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOW_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = LOW_FOR_THE_THING.write() { *w = v; } }
pub fn g_MEASURED() -> Py { ensure_init(); MEASURED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASURED(v: Py) { ensure_init(); if let Ok(mut w) = MEASURED.write() { *w = v; } }
pub fn g_MONEY_DOUBTS() -> Py { ensure_init(); MONEY_DOUBTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_DOUBTS(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_DOUBTS.write() { *w = v; } }
pub fn g_MONEY_RANGE() -> Py { ensure_init(); MONEY_RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_RANGE.write() { *w = v; } }
pub fn g_MONEY_WORDS() -> Py { ensure_init(); MONEY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_WORDS.write() { *w = v; } }
pub fn g_MORE_THAN_ENOUGH() -> Py { ensure_init(); MORE_THAN_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MORE_THAN_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = MORE_THAN_ENOUGH.write() { *w = v; } }
pub fn g_NEXT_YEAR() -> Py { ensure_init(); NEXT_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEXT_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = NEXT_YEAR.write() { *w = v; } }
pub fn g_NOT_TOO_LITTLE() -> Py { ensure_init(); NOT_TOO_LITTLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_LITTLE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOO_LITTLE.write() { *w = v; } }
pub fn g_NOT_TOO_LONG() -> Py { ensure_init(); NOT_TOO_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_LONG(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOO_LONG.write() { *w = v; } }
pub fn g_NOT_TOO_LONG_INSIDE() -> Py { ensure_init(); NOT_TOO_LONG_INSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_LONG_INSIDE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOO_LONG_INSIDE.write() { *w = v; } }
pub fn g_NOT_TOO_MUCH() -> Py { ensure_init(); NOT_TOO_MUCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_MUCH(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOO_MUCH.write() { *w = v; } }
pub fn g_NOT_TOO_SHORT() -> Py { ensure_init(); NOT_TOO_SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TOO_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TOO_SHORT.write() { *w = v; } }
pub fn g_NO_PRICE_SAID() -> Py { ensure_init(); NO_PRICE_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_PRICE_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NO_PRICE_SAID.write() { *w = v; } }
pub fn g_NUMBER() -> Py { ensure_init(); NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER.write() { *w = v; } }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); NUMBER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER_WORDS.write() { *w = v; } }
pub fn g_ON_THE_HIGH_SIDE() -> Py { ensure_init(); ON_THE_HIGH_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_HIGH_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_HIGH_SIDE.write() { *w = v; } }
pub fn g_ON_THE_LONG_SIDE() -> Py { ensure_init(); ON_THE_LONG_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_LONG_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_LONG_SIDE.write() { *w = v; } }
pub fn g_ON_THE_LOW_SIDE() -> Py { ensure_init(); ON_THE_LOW_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_LOW_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_LOW_SIDE.write() { *w = v; } }
pub fn g_ON_THE_SHORT_SIDE() -> Py { ensure_init(); ON_THE_SHORT_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_SHORT_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_SHORT_SIDE.write() { *w = v; } }
pub fn g_PER_UNIT() -> Py { ensure_init(); PER_UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PER_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = PER_UNIT.write() { *w = v; } }
pub fn g_PLACES_A_FIGURE() -> Py { ensure_init(); PLACES_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACES_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = PLACES_A_FIGURE.write() { *w = v; } }
pub fn g_PLURALS() -> Py { ensure_init(); PLURALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLURALS(v: Py) { ensure_init(); if let Ok(mut w) = PLURALS.write() { *w = v; } }
pub fn g_POINTING_AT_SOMETHING() -> Py { ensure_init(); POINTING_AT_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTING_AT_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = POINTING_AT_SOMETHING.write() { *w = v; } }
pub fn g_POINTING_ONE() -> Py { ensure_init(); POINTING_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTING_ONE(v: Py) { ensure_init(); if let Ok(mut w) = POINTING_ONE.write() { *w = v; } }
pub fn g_QUANTITY_WORDS() -> Py { ensure_init(); QUANTITY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUANTITY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = QUANTITY_WORDS.write() { *w = v; } }
pub fn g_QUESTION() -> Py { ensure_init(); QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION.write() { *w = v; } }
pub fn g_RANGE() -> Py { ensure_init(); RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = RANGE.write() { *w = v; } }
pub fn g_RIGHT_FOR_THE_THING() -> Py { ensure_init(); RIGHT_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RIGHT_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = RIGHT_FOR_THE_THING.write() { *w = v; } }
pub fn g_SAID_HIGH() -> Py { ensure_init(); SAID_HIGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_HIGH(v: Py) { ensure_init(); if let Ok(mut w) = SAID_HIGH.write() { *w = v; } }
pub fn g_SAID_LOW() -> Py { ensure_init(); SAID_LOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_LOW(v: Py) { ensure_init(); if let Ok(mut w) = SAID_LOW.write() { *w = v; } }
pub fn g_SAME_LEAGUE() -> Py { ensure_init(); SAME_LEAGUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAME_LEAGUE(v: Py) { ensure_init(); if let Ok(mut w) = SAME_LEAGUE.write() { *w = v; } }
pub fn g_SHORT() -> Py { ensure_init(); SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = SHORT.write() { *w = v; } }
pub fn g_SHORT_FOR_THE_THING() -> Py { ensure_init(); SHORT_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_FOR_THE_THING.write() { *w = v; } }
pub fn g_STOP_AFTER() -> Py { ensure_init(); STOP_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = STOP_AFTER.write() { *w = v; } }
pub fn g_SUBJECTS() -> Py { ensure_init(); SUBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECTS.write() { *w = v; } }
pub fn g_TAG_ASK() -> Py { ensure_init(); TAG_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAG_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TAG_ASK.write() { *w = v; } }
pub fn g_TAILS() -> Py { ensure_init(); TAILS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAILS(v: Py) { ensure_init(); if let Ok(mut w) = TAILS.write() { *w = v; } }
pub fn g_THEIR_FIGURE() -> Py { ensure_init(); THEIR_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_FIGURE.write() { *w = v; } }
pub fn g_THEIR_NUMBER() -> Py { ensure_init(); THEIR_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_NUMBER.write() { *w = v; } }
pub fn g_THE_LOW_SIDE_FOUND() -> Py { ensure_init(); THE_LOW_SIDE_FOUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_LOW_SIDE_FOUND(v: Py) { ensure_init(); if let Ok(mut w) = THE_LOW_SIDE_FOUND.write() { *w = v; } }
pub fn g_THE_LOW_SIDE_FOUND_FOR_THE_THING() -> Py { ensure_init(); THE_LOW_SIDE_FOUND_FOR_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_LOW_SIDE_FOUND_FOR_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = THE_LOW_SIDE_FOUND_FOR_THE_THING.write() { *w = v; } }
pub fn g_THE_PRICE_IT_GAVE() -> Py { ensure_init(); THE_PRICE_IT_GAVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PRICE_IT_GAVE(v: Py) { ensure_init(); if let Ok(mut w) = THE_PRICE_IT_GAVE.write() { *w = v; } }
pub fn g_THE_RANGE_ITSELF() -> Py { ensure_init(); THE_RANGE_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_RANGE_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = THE_RANGE_ITSELF.write() { *w = v; } }
pub fn g_THE_REST_IS_UNSURE() -> Py { ensure_init(); THE_REST_IS_UNSURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_REST_IS_UNSURE(v: Py) { ensure_init(); if let Ok(mut w) = THE_REST_IS_UNSURE.write() { *w = v; } }
pub fn g_THE_USUAL_NO() -> Py { ensure_init(); THE_USUAL_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_USUAL_NO(v: Py) { ensure_init(); if let Ok(mut w) = THE_USUAL_NO.write() { *w = v; } }
pub fn g_TIME_DOUBT() -> Py { ensure_init(); TIME_DOUBT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_DOUBT(v: Py) { ensure_init(); if let Ok(mut w) = TIME_DOUBT.write() { *w = v; } }
pub fn g_TOO_LITTLE_WORDS() -> Py { ensure_init(); TOO_LITTLE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOO_LITTLE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TOO_LITTLE_WORDS.write() { *w = v; } }
pub fn g_TOO_MUCH_WORDS() -> Py { ensure_init(); TOO_MUCH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOO_MUCH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TOO_MUCH_WORDS.write() { *w = v; } }
pub fn g_UNDER_THE_USUAL() -> Py { ensure_init(); UNDER_THE_USUAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNDER_THE_USUAL(v: Py) { ensure_init(); if let Ok(mut w) = UNDER_THE_USUAL.write() { *w = v; } }
pub fn g_UNIT() -> Py { ensure_init(); UNIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNIT(v: Py) { ensure_init(); if let Ok(mut w) = UNIT.write() { *w = v; } }
pub fn g_UNIT_ALIAS() -> Py { ensure_init(); UNIT_ALIAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNIT_ALIAS(v: Py) { ensure_init(); if let Ok(mut w) = UNIT_ALIAS.write() { *w = v; } }
pub fn g_UNIT_WORDS() -> Py { ensure_init(); UNIT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNIT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = UNIT_WORDS.write() { *w = v; } }
pub fn g_WORD_ALIAS() -> Py { ensure_init(); WORD_ALIAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD_ALIAS(v: Py) { ensure_init(); if let Ok(mut w) = WORD_ALIAS.write() { *w = v; } }
pub fn g_WORD_FIGURE_COUNTS() -> Py { ensure_init(); WORD_FIGURE_COUNTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD_FIGURE_COUNTS(v: Py) { ensure_init(); if let Ok(mut w) = WORD_FIGURE_COUNTS.write() { *w = v; } }
pub fn g_WORD_NUMBERS() -> Py { ensure_init(); WORD_NUMBERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD_NUMBERS(v: Py) { ensure_init(); if let Ok(mut w) = WORD_NUMBERS.write() { *w = v; } }
pub fn g_WORD_RANGE() -> Py { ensure_init(); WORD_RANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD_RANGE(v: Py) { ensure_init(); if let Ok(mut w) = WORD_RANGE.write() { *w = v; } }
pub fn g_YEAR() -> Py { ensure_init(); YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = YEAR.write() { *w = v; } }
pub fn g__ASIDE_ENDS() -> Py { ensure_init(); _ASIDE_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASIDE_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = _ASIDE_ENDS.write() { *w = v; } }
pub fn g__A_RATE() -> Py { ensure_init(); _A_RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_RATE(v: Py) { ensure_init(); if let Ok(mut w) = _A_RATE.write() { *w = v; } }
pub fn g__BUYS() -> Py { ensure_init(); _BUYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BUYS(v: Py) { ensure_init(); if let Ok(mut w) = _BUYS.write() { *w = v; } }
pub fn g__POINTS_AT_IT() -> Py { ensure_init(); _POINTS_AT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_IT(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_AT_IT.write() { *w = v; } }
pub fn g__SAME_JOB() -> Py { ensure_init(); _SAME_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAME_JOB(v: Py) { ensure_init(); if let Ok(mut w) = _SAME_JOB.write() { *w = v; } }
pub fn g__SAYS_NOTHING_ABOUT_THE_TIER() -> Py { ensure_init(); _SAYS_NOTHING_ABOUT_THE_TIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAYS_NOTHING_ABOUT_THE_TIER(v: Py) { ensure_init(); if let Ok(mut w) = _SAYS_NOTHING_ABOUT_THE_TIER.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A number offered into a thread the bot has just answered.\n\nPeople answer the bot's advice with the reading they are looking at: \"i think my boiler pressure\nis low\" gets \"top the pressure up to between 1 and 1.5 bar\", and the next line is \"the gauge says\n0.5\". Main said that line back (\"Got it - the gauge says 0.5\"), which repeats it and tells no one\nanything: the person is holding a number and wants to know whether it is the number.\n\nThe answer they are asking about is usually in the bot's own last line, because knowledge entries\nstate the range somewhere in it (\"between 1 and 1.5 bar\", \"between 0 and 5C\", \"16-18C\"). So this\nreads the number, reads the range out of the answer just given, and says which way it is off,\nwith the step the answer already gave. It answers only a line that is nothing but a reading and\nthat follows a knowledge answer which really states a range - everything else goes to the modules\nthat know better.\n");
    s_NUMBER(stdlib_call("re", "compile", &[py_str("(?<![\\w.,])(-?\\d+(?:\\.\\d+)?)(?!\\d|[.,]\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    s_UNIT(stdlib_call("re", "compile", &[py_str("(?:\\d\\s*)?(bar|bars|psi|c|f|celsius|centigrade|degrees|deg|mm|cm|kg|litres?|liters?|ml|watts?|volts?|hours?|hrs?|days?|weeks?|months?|years?|%)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_RANGE(stdlib_call("re", "compile", &[py_str("(?:between\\s+)?(-?\\d+(?:\\.\\d+)?)\\s*(?:and|to|--|-|\\u2013)\\s*(-?\\d+(?:\\.\\d+)?)\\s*([a-z%]{1,10})?\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURED(call_builtin("frozenset", &[py_tuple(vec![py_str("bar"), py_str("bars"), py_str("psi"), py_str("c"), py_str("f"), py_str("celsius"), py_str("centigrade"), py_str("degrees"), py_str("deg"), py_str("mm"), py_str("cm"), py_str("kg"), py_str("litre"), py_str("litres"), py_str("liter"), py_str("liters"), py_str("ml"), py_str("watt"), py_str("watts"), py_str("volt"), py_str("volts"), py_str("%")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CARRIED(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("it"), py_str("its"), py_str("it's"), py_str("the"), py_str("my"), py_str("we"), py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("be"), py_str("been"), py_str("says"), py_str("say"), py_str("said"), py_str("reads"), py_str("read"), py_str("shows"), py_str("shown"), py_str("showing"), py_str("gives"), py_str("gave"), py_str("gets"), py_str("has"), py_str("have"), py_str("had"), py_str("at"), py_str("about"), py_str("around"), py_str("roughly"), py_str("now"), py_str("still"), py_str("only"), py_str("just"), py_str("currently"), py_str("currently"), py_str("down"), py_str("up"), py_str("to"), py_str("of"), py_str("on"), py_str("in"), py_str("a"), py_str("an"), py_str("and"), py_str("but"), py_str("actually"), py_str("today"), py_str("tonight"), py_str("this"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("gauge"), py_str("dial"), py_str("meter"), py_str("thermometer"), py_str("display"), py_str("screen"), py_str("number"), py_str("reading"), py_str("there"), py_str("that"), py_str("then"), py_str("so"), py_str("or"), py_str("they"), py_str("she"), py_str("he"), py_str("here"), py_str("sitting"), py_str("sits"), py_str("hovering"), py_str("by"), py_str("from"), py_str("door"), py_str("shelf"), py_str("fridge"), py_str("freezer"), py_str("boiler"), py_str("pressure"), py_str("temp"), py_str("temperature"), py_str("room"), py_str("house"), py_str("car"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("oven"), py_str("probe"), py_str("water"), py_str("bar"), py_str("bars"), py_str("psi"), py_str("c"), py_str("f"), py_str("deg"), py_str("degree"), py_str("degrees"), py_str("centigrade"), py_str("celsius"), py_str("mm"), py_str("cm"), py_str("kg"), py_str("litre"), py_str("litres"), py_str("ml"), py_str("watt"), py_str("watts"), py_str("volt"), py_str("volts")])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:should|shall|is it|is that|do i|does it|what|why|how|which|when|where|would|could|can)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90))]));
    s_WORD_NUMBERS(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NUMBER_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_WORD_RANGE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("(?:between\\s+)?\\b("), &g_WORD_NUMBERS()), &py_str(")\\b\\s*(?:and|to|\\u2013|-)\\s*(")), &g_WORD_NUMBERS()), &py_str(")\\b\\s*([a-z%]{1,10})?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLURALS(py_dict(vec![(py_str("hour"), py_str("hours")), (py_str("minute"), py_str("minutes")), (py_str("day"), py_str("days")), (py_str("week"), py_str("weeks")), (py_str("month"), py_str("months")), (py_str("year"), py_str("years")), (py_str("mile"), py_str("miles")), (py_str("litre"), py_str("litres")), (py_str("liter"), py_str("liters")), (py_str("gram"), py_str("grams")), (py_str("calorie"), py_str("calories")), (py_str("step"), py_str("steps")), (py_str("hr"), py_str("hrs")), (py_str("min"), py_str("mins"))]));
    s_STOP_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("for"), py_str("of"), py_str("a"), py_str("an"), py_str("the"), py_str("in"), py_str("to"), py_str("and"), py_str("or"), py_str("at"), py_str("on"), py_str("with"), py_str("but"), py_str("so"), py_str("if"), py_str("when"), py_str("which"), py_str("that"), py_str("is"), py_str("are"), py_str("it"), py_str("you"), py_str("we"), py_str("they"), py_str("per"), py_str("each"), py_str("roughly"), py_str("about"), py_str("around"), py_str("plus"), py_str("max"), py_str("maximum"), py_str("minimum"), py_str("more"), py_str("less"), py_str("up"), py_str("down"), py_str("off"), py_str("out"), py_str("from"), py_str("by"), py_str("as"), py_str("than"), py_str("then"), py_str("there"), py_str("this"), py_str("these"), py_str("those"), py_str("his"), py_str("her"), py_str("their"), py_str("your"), py_str("my"), py_str("our"), py_str("its")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT(Py::Int(6));
    s_AN_AGE(stdlib_call("re", "compile", &[py_str("^(?:i'?m|im|i am|ive|i've|i just turned|i turned)\\s+\\d{1,3}(?:\\s*(?:years?|yrs?)(?:\\s+old)?)?(?:\\s+(?:now|this year))?\\s*[.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING(py_tuple(vec![py_str("and"), py_str("so"), py_str("but"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("well"), py_str("hmm"), py_str("umm"), py_str("er")]));
    s_ASKS(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("would"), py_str("will"), py_str("does"), py_str("do"), py_str("did"), py_str("should")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECTS(py_tuple(vec![py_str("that"), py_str("this"), py_str("it"), py_str("there"), py_str("they"), py_str("those")]));
    s_MONEY_DOUBTS(call_builtin("frozenset", &[py_tuple(vec![py_str("a good price"), py_str("a fair price"), py_str("a good deal"), py_str("expensive"), py_str("cheap"), py_str("steep"), py_str("on the high side"), py_str("on the low side"), py_str("about the going rate"), py_str("the going rate")])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUANTITY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("a lot"), py_str("much"), py_str("too much"), py_str("too high"), py_str("too low"), py_str("too little"), py_str("about right"), py_str("about average"), py_str("enough"), py_str("a good price"), py_str("a fair price"), py_str("a good deal"), py_str("expensive"), py_str("cheap"), py_str("steep"), py_str("on the high side"), py_str("on the low side"), py_str("over the top"), py_str("about the going rate"), py_str("the going rate"), py_str("average"), py_str("above average"), py_str("below average"), py_str("too long"), py_str("too short")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_DOUBT(call_builtin("frozenset", &[py_tuple(vec![py_str("too long"), py_str("too short")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_WORDS(bitor(&g_QUANTITY_WORDS(), &call_builtin("frozenset", &[py_tuple(vec![py_str("normal"), py_str("reasonable"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("fair"), py_str("right")])]).unwrap_or_else(|e| exc::raise(e))));
    s_BARE_DOUBTS(sub(&g_QUANTITY_WORDS(), &py_set(vec![py_str("much"), py_str("about average")])));
    s_WORD_ALIAS(py_dict(vec![(py_str("too expensive"), py_str("expensive")), (py_str("too dear"), py_str("expensive")), (py_str("too pricey"), py_str("expensive")), (py_str("too cheap"), py_str("cheap")), (py_str("a bit much"), py_str("too much")), (py_str("bit much"), py_str("too much")), (py_str("a bit steep"), py_str("steep")), (py_str("a bit dear"), py_str("expensive")), (py_str("a bit pricey"), py_str("expensive")), (py_str("a bit expensive"), py_str("expensive")), (py_str("quite a lot"), py_str("a lot")), (py_str("loads"), py_str("a lot")), (py_str("loads of money"), py_str("a lot")), (py_str("overpriced"), py_str("expensive")), (py_str("pricey"), py_str("expensive")), (py_str("dear"), py_str("expensive")), (py_str("a rip off"), py_str("expensive")), (py_str("rip off"), py_str("expensive")), (py_str("ripoff"), py_str("expensive")), (py_str("a ripoff"), py_str("expensive")), (py_str("ripped off"), py_str("expensive")), (py_str("ripping me off"), py_str("expensive")), (py_str("get ripped off"), py_str("expensive")), (py_str("got ripped off"), py_str("expensive")), (py_str("i get ripped off"), py_str("expensive")), (py_str("i got ripped off"), py_str("expensive")), (py_str("i was ripped off"), py_str("expensive")), (py_str("i am being ripped off"), py_str("expensive")), (py_str("overcharged"), py_str("expensive")), (py_str("overcharging me"), py_str("expensive")), (py_str("i was overcharged"), py_str("expensive")), (py_str("been overcharged"), py_str("expensive")), (py_str("get overcharged"), py_str("expensive")), (py_str("get done"), py_str("expensive")), (py_str("i got done"), py_str("expensive")), (py_str("overpaying"), py_str("expensive")), (py_str("paying too much"), py_str("too much")), (py_str("over the odds"), py_str("expensive")), (py_str("a con"), py_str("expensive")), (py_str("excessive"), py_str("too much")), (py_str("unreasonable"), py_str("too much")), (py_str("exorbitant"), py_str("too much")), (py_str("having a laugh"), py_str("expensive")), (py_str("taking the mick"), py_str("expensive")), (py_str("having me on"), py_str("expensive")), (py_str("daylight robbery"), py_str("expensive")), (py_str("a joke"), py_str("expensive")), (py_str("how does that compare"), py_str("about right")), (py_str("in the ballpark"), py_str("about right")), (py_str("in line"), py_str("about average")), (py_str("in line with what youd expect"), py_str("about average")), (py_str("in line with what youd pay"), py_str("about average")), (py_str("the normal price"), py_str("about average")), (py_str("normal price"), py_str("about average")), (py_str("a normal price"), py_str("about average")), (py_str("price about right"), py_str("about right")), (py_str("what it should cost"), py_str("about right")), (py_str("sound about right"), py_str("about right")), (py_str("sounds about right"), py_str("about right")), (py_str("look about right"), py_str("about right")), (py_str("looks about right"), py_str("about right")), (py_str("cost that much"), py_str("too much")), (py_str("that much"), py_str("too much")), (py_str("this much"), py_str("too much")), (py_str("acceptable"), py_str("about right")), (py_str("alright"), py_str("about right")), (py_str("all right"), py_str("about right")), (py_str("sensible"), py_str("about right")), (py_str("much good"), py_str("about right")), (py_str("fair enough"), py_str("about right")), (py_str("high"), py_str("too high")), (py_str("low"), py_str("too low"))]));
    s_TAILS(py_tuple(vec![py_str("really"), py_str("then"), py_str("though"), py_str("to me"), py_str("i think"), py_str("for that"), py_str("for it"), py_str("for one"), py_str("to pay"), py_str("to spend"), py_str("of money"), py_str("of it"), py_str("like that"), py_str("in total"), py_str("altogether"), py_str("for what it is"), py_str("for what they are"), py_str("for the size of it")]));
    s_TAG_ASK(stdlib_call("re", "compile", &[py_str("[\\s,]+(?:is(?:nt|n't)?\\s+(?:it|that|this|they)|innit|aint\\s+it|eh|dont\\s+(?:you|ya)\\s+think|do\\s+you\\s+think|wouldnt\\s+you\\s+say)[\\s?!.]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_THE_THING(stdlib_call("re", "compile", &[py_str("\\s+(?:for|on)\\s+(?P<thing>(?:(?:a|an|the|my|our) )?[a-z][a-z' -]{1,40})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_THE_ARTICLE(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|my|our) "), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_AT_SOMETHING(call_builtin("frozenset", &[py_tuple(vec![py_str("one"), py_str("ones"), py_str("them"), py_str("it"), py_str("that"), py_str("this"), py_str("those"), py_str("these"), py_str("same")])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:one|ones)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE(stdlib_call("re", "compile", &[py_str("(?:(?P<mark>[£$€])\\s?)?(?P<number>\\d[\\d,]*(?:\\.\\d+)?)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_UNITS(call_builtin("frozenset", &[py_tuple(vec![py_str("bar"), py_str("bars"), py_str("psi"), py_str("c"), py_str("f"), py_str("celsius"), py_str("centigrade"), py_str("degrees"), py_str("deg"), py_str("mm"), py_str("cm"), py_str("kg"), py_str("g"), py_str("grams"), py_str("litre"), py_str("litres"), py_str("liter"), py_str("liters"), py_str("ml"), py_str("watt"), py_str("watts"), py_str("volt"), py_str("volts"), py_str("%"), py_str("hour"), py_str("hours"), py_str("hr"), py_str("hrs"), py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins"), py_str("day"), py_str("days"), py_str("week"), py_str("weeks"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("mile"), py_str("miles"), py_str("km"), py_str("kcal"), py_str("calories"), py_str("steps"), py_str("degrees")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACES_A_FIGURE(sub(&g_QUANTITY_WORDS(), &py_set(vec![py_str("enough")])));
    s_A_GAUGE_UNIT(call_builtin("frozenset", &[py_tuple(vec![py_str("c"), py_str("f"), py_str("celsius"), py_str("centigrade"), py_str("deg"), py_str("degree"), py_str("degrees"), py_str("bar"), py_str("bars"), py_str("psi")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PER_UNIT(stdlib_call("re", "compile", &[py_str("(?:an?|per|each)\\s+(?=(?:bar|psi|degrees?|hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|kilos?|kg|litres?|steps)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_UNIT(stdlib_call("re", "compile", &[py_str("(bar|bars|psi|c|f|celsius|centigrade|degrees|deg|mm|cm|kg|kilos?|kilograms?|grams?|g|stone|stones|lb|lbs|litres?|liters?|ml|pints?|watts?|volts?|%|hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|k|kcal|calories?|steps|pounds?|quid|dollars?|euros?|bucks?|pence)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_UNIT_ALIAS(py_dict(vec![(py_str("kilo"), py_str("kg")), (py_str("kilos"), py_str("kg")), (py_str("kilogram"), py_str("kg")), (py_str("kilograms"), py_str("kg")), (py_str("k"), py_str("km")), (py_str("stone"), py_str("stone")), (py_str("stones"), py_str("stone")), (py_str("lb"), py_str("lb")), (py_str("lbs"), py_str("lb")), (py_str("pint"), py_str("pints"))]));
    s_MONEY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("pound"), py_str("pounds"), py_str("quid"), py_str("dollar"), py_str("dollars"), py_str("euro"), py_str("euros"), py_str("buck"), py_str("bucks"), py_str("pence")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_RANGE(stdlib_call("re", "compile", &[add(&add(&py_str("(?:(?P<mark>[£$€])\\s?)?(?P<low>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:to|and|\\u2013|-|--)\\s*(?:(?P<mark2>[£$€])\\s?)?(?P<high>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?P<word>"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_MONEY_WORDS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")?\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_YEAR(Py::Int(1900));
    s_NEXT_YEAR(Py::Int(2100));
    s_AN_AGE_AFTER(stdlib_call("re", "compile", &[py_str("^[\\s,|\\-]{0,3}(?:years?|months?|weeks?|days?)?\\s*olds?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_MUCH_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("a lot"), py_str("much"), py_str("too much"), py_str("too high"), py_str("expensive"), py_str("steep"), py_str("over the top"), py_str("on the high side")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_RIGHT(py_tuple(vec![py_str("That is about right - {span} is the usual."), py_str("That sits inside the {span} that is usual for it."), py_str("Right in the usual range: {span}.")]));
    s_NOT_TOO_LITTLE(py_tuple(vec![py_str("No, that is about right - {span} is the usual."), py_str("No, you are inside it: the usual is {span}.")]));
    s_NOT_TOO_MUCH(py_tuple(vec![py_str("No, that is about right - {span} is the usual."), py_str("No, that is fine: the usual is {span}."), py_str("No - {span} is the usual, so you are inside it.")]));
    s_ON_THE_HIGH_SIDE(py_tuple(vec![py_str("That is on the high side: the usual is about {span}."), py_str("Yes, a bit much - {span} is the usual.")]));
    s_NOT_TOO_LONG(py_tuple(vec![py_str("No - {span} is the usual, and that is under it."), py_str("No, that is short of the usual {span}.")]));
    s_NOT_TOO_LONG_INSIDE(py_tuple(vec![py_str("No, that is inside the usual {span}."), py_str("No - {span} is the usual, and that sits inside it.")]));
    s_NOT_TOO_SHORT(py_tuple(vec![py_str("No, that is not too short - {span} is the usual."), py_str("No - {span} is the usual, so there is plenty of room there.")]));
    s_ON_THE_LONG_SIDE(py_tuple(vec![py_str("Yes, that is on the long side - the usual is {span}."), py_str("Yes, that is longer than the usual {span}.")]));
    s_ON_THE_SHORT_SIDE(py_tuple(vec![py_str("Yes, that is on the short side - the usual is {span}."), py_str("Yes, that is shorter than the usual {span}.")]));
    s_ABOVE_THE_USUAL(py_tuple(vec![py_str("No, that is on the high side: the usual is about {span}."), py_str("No - {span} is the usual, so that is above it.")]));
    s_ENOUGH(py_tuple(vec![py_str("Yes, that is enough - the usual is {span}."), py_str("That is plenty: the usual is {span}.")]));
    s_ON_THE_LOW_SIDE(py_tuple(vec![py_str("That is on the low side - the usual is about {span}."), py_str("Yes, a bit low - {span} is the usual.")]));
    s_THE_LOW_SIDE_FOUND(py_tuple(vec![py_str("That is on the low side - the usual is about {span}."), py_str("No, that is under the {span} that is usual.")]));
    s_UNDER_THE_USUAL(py_tuple(vec![py_str("That is a little short - the usual is {span}."), py_str("No, that is under the {span} it needs.")]));
    s_MORE_THAN_ENOUGH(py_tuple(vec![py_str("Yes, more than enough - the usual is {span}."), py_str("That is plenty: {span} is the usual.")]));
    s__SAME_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|with|unless|once|where|should)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RATE(stdlib_call("re", "compile", &[py_str("^\\s*(?:a|an|per|each)\\s+(?:day|hour|week|month|visit|year|sq\\s?m)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SAYS_NOTHING_ABOUT_THE_TIER(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if when with without for of in on at to from by as so also plus than then\nis are was were be been being it its this that these those there here his her their your our my\none ones some any more most less much many about around roughly usually normally typically per\neach over under near up down out off just only too very quite really still even\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_LITTLE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("too low"), py_str("too little"), py_str("cheap"), py_str("on the low side")])]).unwrap_or_else(|e| exc::raise(e)));
    s__BUYS(stdlib_call("re", "compile", &[py_str("^(?:buys|gets you|gets|gets a|pays for|fits|covers|includes|holds|lasts|makes|gives|comes to|is|are|will)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASIDE_ENDS(stdlib_call("re", "compile", &[py_str("[,.;:()]|\\s+\\band\\s+(?:£|\\d)|\\s+\\u2014\\s+|\\s+-\\s+|\\s+but\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOOK_BACK(Py::Int(3));
    s_SAME_LEAGUE(Py::Float(100.0));
    s_DAYS_IN(py_dict(vec![(py_str("minute"), truediv(&Py::Int(1), &Py::Float(1440.0))), (py_str("min"), truediv(&Py::Int(1), &Py::Float(1440.0))), (py_str("hour"), truediv(&Py::Int(1), &Py::Float(24.0))), (py_str("hr"), truediv(&Py::Int(1), &Py::Float(24.0))), (py_str("day"), Py::Float(1.0)), (py_str("week"), Py::Float(7.0)), (py_str("month"), Py::Float(30.0)), (py_str("year"), Py::Float(365.0))]));
    s_THE_USUAL_NO(py_tuple(vec![py_str("No - {span} is the usual, so it depends how long yours has been going on."), py_str("No, that is the usual: {span}. How long has yours been going on?"), py_str("No - {span} is the normal spread, so say how long yours has lasted and I'll place it.")]));
    s_A_WORD_FIGURE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b("), &g_WORD_NUMBERS()), &py_str("|a|an|couple|few)\\s+(")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DOUBT_UNITS()) { let mut unit: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[unit.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD_FIGURE_COUNTS(py_dict(vec![(py_str("a"), Py::Float(1.0)), (py_str("an"), Py::Float(1.0)), (py_str("couple"), Py::Float(2.0)), (py_str("few"), Py::Float(3.0))]));
    s_A_CLOCK_TIME_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:at|by|until|till|til|before|after|past)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLOCK_TIME_AFTER(stdlib_call("re", "compile", &[py_str("^\\s*(?:am|pm|a\\.m|p\\.m|o'?clock)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_A_PRICE(py_tuple(vec![py_str("What were you quoted? {span} is the usual, and then I can say whether yours is fair."), py_str("What price have you got? {span} is the usual for that, so I can tell you where you sit.")]));
    s_ASKS_FOR_A_FIGURE(py_tuple(vec![py_str("What figure have you got? The usual is {span}, and I'll say which side of it you are on."), py_str("What is yours? {span} is the usual for it, so I can tell you where it lands.")]));
    s_NO_PRICE_SAID(py_tuple(vec![py_str("I didn't give you a price there, so I can't call it cheap or dear yet. Tell me the figure you have in mind and what it is for, and I'll say whether it is fair."), py_str("There was no price in what I said, so there is nothing to weigh up yet. Give me the number and what it is for."), py_str("None of that was money, so I can't tell you expensive or not yet. Give me the figure and what it is for, and I'll say whether it is fair.")]));
    s_A_LONE_PRICE(stdlib_call("re", "compile", &[add(&add(&py_str("(?P<mark>[£$€])\\s?(?P<amount>\\d[\\d,]*(?:\\.\\d+)?)|\\b(?P<amount2>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?P<word>"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_MONEY_WORDS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PRICE_IT_GAVE(py_tuple(vec![py_str("{span} was the only price in that, so there is nothing to call dear yet. What have you been quoted?"), py_str("That answer named {span} and no range, so cheap or dear needs a second price. What figure are you weighing up?"), py_str("{span} is the figure I gave - one price, no range - so tell me the number you have in mind and I'll say where it sits.")]));
    s_A_RANGE_THAT_IS_NOT_A_PRICE(py_tuple(vec![py_str("That was a range of {unit}, not a price: {span} is what it said, so cheap or dear does not come into it. If you are weighing up a figure, tell me it and what it is for."), py_str("{span} is a usual range, not a price, so there is nothing there to call expensive. Give me the figure you are judging and I'll say where it sits.")]));
    s_UNIT_WORDS(py_dict(vec![(py_str("hour"), py_str("hours")), (py_str("hours"), py_str("hours")), (py_str("hr"), py_str("hours")), (py_str("hrs"), py_str("hours")), (py_str("minute"), py_str("minutes")), (py_str("minutes"), py_str("minutes")), (py_str("min"), py_str("minutes")), (py_str("mins"), py_str("minutes")), (py_str("day"), py_str("days")), (py_str("days"), py_str("days")), (py_str("week"), py_str("weeks")), (py_str("weeks"), py_str("weeks")), (py_str("month"), py_str("months")), (py_str("months"), py_str("months")), (py_str("year"), py_str("years")), (py_str("years"), py_str("years")), (py_str("mile"), py_str("miles")), (py_str("miles"), py_str("miles")), (py_str("km"), py_str("kilometres")), (py_str("calories"), py_str("calories")), (py_str("kcal"), py_str("calories")), (py_str("steps"), py_str("steps")), (py_str("g"), py_str("grams")), (py_str("grams"), py_str("grams")), (py_str("kg"), py_str("kilos")), (py_str("litre"), py_str("litres")), (py_str("litres"), py_str("litres")), (py_str("liter"), py_str("litres")), (py_str("liters"), py_str("litres")), (py_str("ml"), py_str("millilitres"))]));
    s_THE_RANGE_ITSELF(py_tuple(vec![py_str("That is the range itself: {span} is what it says, so nothing in it is out of the ordinary. If you have a figure of your own, tell me and I'll say where it sits."), py_str("{span} is the usual, so that is the figure to hold next to yours. Give me your own and I'll place it on the range."), py_str("Nothing there was out of the ordinary - {span} is the usual, and that is the figure I gave you. Tell me your own and I'll judge that one instead.")]));
    s_BELOW_THE_RANGE(py_tuple(vec![py_str("Under {span} would be low, so the bottom of that is what to aim for. Tell me your figure and I'll say how far under it is."), py_str("{span} is the usual, so anything under it is on the low side. What is yours?")]));
    s_ABOVE_THE_RANGE(py_tuple(vec![py_str("Over {span} would be high, so anything at the top of it is fine. Tell me your figure and I'll say how far over it is."), py_str("{span} is the usual, so anything above that is on the high side. What is yours?")]));
    s_SAID_LOW(call_builtin("frozenset", &[py_tuple(vec![py_str("too low"), py_str("on the low side"), py_str("below average"), py_str("too little"), py_str("not enough"), py_str("enough")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_HIGH(call_builtin("frozenset", &[py_tuple(vec![py_str("too high"), py_str("on the high side"), py_str("above average"), py_str("too much"), py_str("over the top")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RIGHT_FOR_THE_THING(py_tuple(vec![py_str("That is about right for {thing} - {span} is the usual."), py_str("Yes, that is in the normal range: {span} is the usual for {thing}.")]));
    s_HIGH_FOR_THE_THING(py_tuple(vec![py_str("That is on the high side for {thing}: the usual is about {span}."), py_str("Yes, a bit much for {thing} - {span} is the usual.")]));
    s_LOW_FOR_THE_THING(py_tuple(vec![py_str("That is on the low side for {thing} - the usual is about {span}."), py_str("Yes, a bit low for {thing} - {span} is the usual.")]));
    s_ENOUGH_FOR_THE_THING(py_tuple(vec![py_str("Yes, that is enough for {thing} - the usual is {span}."), py_str("That is plenty for {thing}: {span} is the usual.")]));
    s_SHORT_FOR_THE_THING(py_tuple(vec![py_str("That is a little short for {thing} - the usual is {span}."), py_str("No, that is under the {span} it needs for {thing}.")]));
    s_THE_LOW_SIDE_FOUND_FOR_THE_THING(py_tuple(vec![py_str("That is on the low side for {thing} - the usual is about {span}."), py_str("No, that is under the {span} that {thing} usually costs.")]));
    s__POINTS_AT_IT(call_builtin("frozenset", &[mcall(&py_str("\n    one ones it that this these those them they him her he she thing things\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_OF_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:times?|x|runs?|sessions?|workouts?)\\s+(?:a|per|each|every)\\s+(?:day|week|month|year)\\b|\\btwice\\s+(?:a|per|each|every)\\s+(?:day|week|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_FIGURE(stdlib_call("re", "compile", &[py_str("(?:(?P<mark>[\\u00a3$\\u20ac])\\s?)?(?P<number>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:a|an|per|each)?\\s*(?P<unit>bar|bars|psi|hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|kilos?|kg|grams?|g|steps|calories?|kcal|litres?|liters?|ml|pints?|watts?|volts?|degrees?|deg|pounds?|quid|dollars?|euros?|bucks?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_NUMBER(stdlib_call("re", "compile", &[py_str("(?:(?P<mark>[\\u00a3$\\u20ac])\\s?)?(?P<number>\\d[\\d,]*(?:\\.\\d+)?)(?!\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BILL(call_builtin("frozenset", &[py_tuple(vec![py_str("bill"), py_str("bills"), py_str("electricity"), py_str("gas"), py_str("water"), py_str("heating"), py_str("rent"), py_str("mortgage"), py_str("insurance"), py_str("subscription"), py_str("subscriptions"), py_str("mobile"), py_str("broadband"), py_str("wifi"), py_str("council"), py_str("tax"), py_str("paying"), py_str("pay"), py_str("paid"), py_str("pays"), py_str("cost"), py_str("costs"), py_str("costing"), py_str("quote"), py_str("quoted"), py_str("price"), py_str("priced"), py_str("charged"), py_str("charge"), py_str("charging"), py_str("fee"), py_str("fees"), py_str("fare"), py_str("fares"), py_str("deposit"), py_str("debit"), py_str("invoice"), py_str("loan"), py_str("instalments"), py_str("installments"), py_str("monthly"), py_str("annual"), py_str("saving"), py_str("savings"), py_str("spend"), py_str("spends"), py_str("spending"), py_str("spent"), py_str("budget"), py_str("budgeting"), py_str("shop"), py_str("shopping")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERIOD(call_builtin("frozenset", &[py_tuple(vec![py_str("month"), py_str("months"), py_str("week"), py_str("weeks"), py_str("year"), py_str("years")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:the whole thing|the whole lot|for the lot|all in|altogether|in total|all told|the lot)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STRETCH(call_builtin("frozenset", &[py_tuple(vec![py_str("hour"), py_str("hours"), py_str("hr"), py_str("hrs"), py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DISTANCE(call_builtin("frozenset", &[py_tuple(vec![py_str("mile"), py_str("miles"), py_str("km")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRICE_NEXT_TO(py_tuple(vec![py_str("{said} is only high or low next to another figure for the same thing: what you were paying before, or what somebody else pays for it. What was it before?"), py_str("{said} needs a second figure beside it - your last one, or what a friend pays - before anyone can call it a lot. What is the other one?")]));
    s_A_JOURNEY_IS_TIME(py_tuple(vec![py_str("{said} is a real journey. Whether it is too far is about the time it takes you and what you want to be able to do at the other end - how long does it take?"), py_str("{said} comes down to the time rather than the miles: easy for one person, a long trip for another. How long does it take you?")]));
    s_A_STRETCH_OF_TIME(py_tuple(vec![py_str("{said} is only too long against the rest of the day. What else has to fit into it?"), py_str("{said} - whether that is too much depends on what else the same time has to hold. What else is in the day?")]));
    s_THE_REST_IS_UNSURE(py_tuple(vec![py_str("{said} is not something I can call a lot from here. What are you comparing it with?"), py_str("{said} - too much against what? Name the thing you are measuring it against and I will take it from there.")]));
    s_A_PRICE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|cost|costs|cost me|paid|pay|spent|spend|charged|comes to|came to|set me back)\\s+(?:me\\s+|us\\s+)?[\\u00a3$\\u20ac]?\\d"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_WORD(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]|\\b(?:money|quid|pounds?|dollars?|euros?|bucks?|cost|costs|price|prices|pay|paying|spend|spending|afford)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_CEILING(stdlib_call("re", "compile", &[py_str("^(?:and |so |ok(?:ay)?,? )?(?:is|was|were|would|does|do)\\s+(?:that|it|this|they|there)?\\s*(?:really\\s+|actually\\s+|still\\s+)?(?P<word>too (?:far|long|many|much|high)|a lot|about right|over the top|expensive|steep)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CEILING(call_builtin("frozenset", &[py_tuple(vec![py_str("a lot"), py_str("much"), py_str("too much"), py_str("too high"), py_str("too many"), py_str("too far"), py_str("too long"), py_str("expensive"), py_str("steep"), py_str("over the top"), py_str("about right"), py_str("a good price"), py_str("a fair price"), py_str("the going rate"), py_str("about the going rate")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_JUDGED(stdlib_call("re", "compile", &[py_str("\\b(?:about where|most people|any(?:thing|one) (?:over|above|past)|more than about|\\\n    no more than|up to about|past that|beyond that|on the high side|on the low side|\\\n    on the dear side|on the cheap side|cheaper than|dearer than|more expensive than|\\\n    about right|is a lot|isn't a lot|is not a lot|fair price|good price|the going rate|\\\n    the usual (?:is|rate|range)|the average is)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INSIDE(py_tuple(vec![py_str("That is where it should be - {span}."), py_str("{value} is in the {span} it wants, so nothing to do there."), py_str("Spot on: {span} is right for it.")]));
}
pub fn _number(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(value, unit) for a line that is nothing but a reading, or None.");
    if py_truthy(&mcall(&g_QUESTION(), "search", &[clean.clone()])) {
    return Py::None;
    }
    found = mcall(&g_NUMBER(), "findall", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_CARRIED(), &word.clone()))) {
    return Py::None;
    }
    }
    unit = mcall(&g_UNIT(), "search", &[clean.clone()]);
    return py_tuple(vec![call_builtin("float", &[geti(&found.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), if py_truthy(&unit.clone()) { mcall(&unit.clone(), "group", &[Py::Int(1)]) } else { py_str("") }]);
    Py::None
}
pub fn _range(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("(low, high, unit) for the first measured range in the answer, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g_RANGE(), "finditer", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    found = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)]), mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]), 3, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    unit = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_contains(&g_MEASURED(), &unit.clone()))) {
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("float", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("float", &[high.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &high.clone()))) {
    continue;
    }
    return if py_truthy(&lt(&low.clone(), &high.clone())) { py_tuple(vec![low.clone(), high.clone(), unit.clone()]) } else { py_tuple(vec![high.clone(), low.clone(), unit.clone()]) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _say(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("A number as a person writes it: 0.5 stays 0.5, 1.0 is 1.");
    return modulo(&py_str("%g"), &value.clone());
    Py::None
}
pub fn _unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("The unit as it is written: \"1 to 1.5 bar\", \"0 to 5C\".");
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("c"), py_str("celsius"), py_str("centigrade")]), &unit.clone()))) {
    return py_str("C");
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("f")))) {
    return py_str("F");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("deg"), py_str("degree"), py_str("degrees")]), &unit.clone()))) {
    return py_str(" degrees");
    }
    return add(&py_str(" "), &unit.clone());
    Py::None
}
pub fn _word_number(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("A count the answer wrote in words, or None.");
    text = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return if py_truthy(&Py::Bool(py_contains(&g_NUMBER_WORDS(), &text.clone()))) { call_builtin("float", &[geti(&g_NUMBER_WORDS(), &text.clone())]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    Py::None
}
pub fn _plural(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut high: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let _ = py_str("The unit as a range of that many of it is written.");
    if py_truthy(&{ let __b = Py::Bool(py_is(&high.clone(), &Py::None)); if py_truthy(&__b) { __b } else { le(&high.clone(), &Py::Int(1)) } }) {
    return unit.clone();
    }
    return mcall(&g_PLURALS(), "get", &[unit.clone(), unit.clone()]);
    Py::None
}
pub fn _usual_unit(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut counts: Py = Py::None;
    let mut found: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The one measurement the answer keeps using, for a range it wrote without one.\n\n    Only a unit the answer has already used counts (`before` is where the range sits). A unit that\n    first appears after the range may belong to another figure: \"two to five, an hour of good\n    programmes a day\" is a range of ages, and borrowing the \"an hour\" that follows it made it \"2\n    to 5 hours\" of screen time.\n    ");
    counts = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[add(&py_str("\\b"), &gattr(&g_DOUBT_UNIT(), "pattern")), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { gt(&mcall(&found.clone(), "start", &[]), &before.clone()) } }) {
    continue;
    }
    unit = mcall(&g_UNIT_ALIAS(), "get", &[mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_MONEY_WORDS(), &unit.clone())) } }) {
    let _ = py_setitem(&counts.clone(), &unit.clone(), add(&mcall(&counts.clone(), "get", &[unit.clone(), Py::Int(0)]), &Py::Int(1)));
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&counts.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(counts.clone()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let before = before.clone(); let counts = counts.clone(); let found = found.clone(); let text = text.clone(); let unit = unit.clone(); let unit: Py = __kwp(args, 0, "unit").unwrap_or(Py::None); geti(&counts.clone(), &unit.clone()) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _add_measured(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut spans: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spans"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut unit: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut usual: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "usual") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: usual"))) };
    let mut at: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { Py::None };
    let mut matched: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "matched") { v.clone() }
    else { py_str("") };
    let mut span: Py = Py::None;
    let _ = py_str("Add one measured span, taking the answer's own unit when the range did not name one.");
    if py_truthy(&{ let __b = Py::Bool(py_is(&low.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&high.clone(), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_DOUBT_UNITS(), &unit.clone()))) {
    unit = if py_truthy(&Py::Bool(py_contains(&g_DOUBT_UNITS(), &usual.clone()))) { usual.clone() } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &high.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("min", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("max", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    low = __unpk1[0].clone();
    high = __unpk1[1].clone();
    span = py_dict(vec![(py_str("low"), low.clone()), (py_str("high"), high.clone()), (py_str("mark"), py_str("")), (py_str("money"), Py::Bool(false)), (py_str("unit"), unit.clone()), (py_str("text"), modulo(&py_str("%s to %s%s"), &py_tuple(vec![_say(&[low.clone()]), _say(&[high.clone()]), _unit(&[_plural(&[unit.clone(), high.clone()])])])))]);
    if py_truthy(&Py::Bool(!py_is(&at.clone(), &Py::None))) {
    let _ = py_setitem(&span.clone(), &py_str("at"), at.clone());
    let _ = py_setitem(&span.clone(), &py_str("end"), if py_truthy(&matched.clone()) { add(&at.clone(), &call_builtin("len", &[matched.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { at.clone() });
    }
    let _ = mcall(&spans.clone(), "append", &[span.clone()]);
    Py::None
}
pub fn _same_measure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut left: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "left") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: left"))) };
    let mut right: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "right") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: right"))) };
    let mut cold: Py = Py::None;
    let mut group: Py = Py::None;
    let mut span: Py = Py::None;
    let mut warm: Py = Py::None;
    let _ = py_str("True when two unit words measure the same thing (a bar is a bar, a degree is a degree).");
    warm = call_builtin("frozenset", &[py_tuple(vec![py_str("c"), py_str("celsius"), py_str("centigrade"), py_str("deg"), py_str("degree"), py_str("degrees")])]).unwrap_or_else(|e| exc::raise(e));
    cold = call_builtin("frozenset", &[py_tuple(vec![py_str("f"), py_str("fahrenheit")])]).unwrap_or_else(|e| exc::raise(e));
    span = call_builtin("frozenset", &[py_tuple(vec![py_str("deg"), py_str("degree"), py_str("degrees")])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![warm.clone(), cold.clone(), span.clone()]));
    for __item in __iter1 {
    group = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&group.clone(), &left.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&group.clone(), &right.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(py_eq(&slice_or_raise(&left.clone(), &Py::None, &Py::Int(3), &Py::None), &slice_or_raise(&right.clone(), &Py::None, &Py::Int(3), &Py::None)));
    Py::None
}
pub fn _where(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut unit: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("Which side of the range the reading is on.");
    if py_truthy(&lt(&value.clone(), &low.clone())) {
    return py_str("below");
    }
    if py_truthy(&gt(&value.clone(), &high.clone())) {
    return py_str("above");
    }
    return py_str("inside");
    Py::None
}
pub fn _step(args: &[Py]) -> Py {
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
    let _ = py_str("The first thing the answer told them to do, in the answer's own words.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), crate::gen::compose::verdict::_step(&[said.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _said(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer this reading is about: the entry answer the line points at, or the last one.");
    text = crate::gen::memory::topics::entry_answer(&[memory.clone(), clean.clone()]);
    if py_truthy(&text.clone()) {
    return call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&Py::Int(1))) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    return py_str("");
    }
    if py_truthy(&gt(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT())) {
    return py_str("");
    }
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    record = { let __b = mcall(&memory.clone(), "get", &[py_str("entry_answer")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    kept = call_builtin("str", &[{ let __b = mcall(&{ let __b = record.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]), &mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&kept.clone(), "lower", &[]), "split", &[]), &Py::None, &Py::Int(6), &Py::None)]))))) } }) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _the_figure_in_the_doubt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut found: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(value, money, unit, rest) for a figure and its unit at the front of a doubt, if any.\n\n    \"12 a lot\", \"£2,400 too much\", \"1.5 bar about right\", \"8000 steps enough\".\n    ");
    found = mcall(&g_FIGURE(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![Py::None, Py::Bool(false), py_str(""), rest.clone(), Py::Bool(false)]);
    }
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("number")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    unit = py_str("");
    rate = call_builtin("bool", &[mcall(&g_PER_UNIT(), "match", &[rest.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&rate.clone()) {
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&mcall(&g_PER_UNIT(), "match", &[rest.clone()]), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    measured = mcall(&g_DOUBT_UNIT(), "match", &[rest.clone()]);
    if py_truthy(&measured.clone()) {
    unit = mcall(&mcall(&measured.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    unit = mcall(&g_UNIT_ALIAS(), "get", &[unit.clone(), unit.clone()]);
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&measured.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    money = call_builtin("bool", &[mcall(&found.clone(), "group", &[py_str("mark")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_MONEY_WORDS(), &unit.clone()))) {
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Bool(true), py_str(""), Py::Bool(false)]), 3, None);
    money = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    rate = __unpk1[2].clone();
    }
    return py_tuple(vec![value.clone(), money.clone(), unit.clone(), rest.clone(), rate.clone()]);
    Py::None
}
pub fn _doubt(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut for_thing: Py = Py::None;
    let mut impression: Py = Py::None;
    let mut line: Py = Py::None;
    let mut money: Py = Py::None;
    let mut opinion: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut short_thing: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The doubt in a line that only asks whether a figure is a lot or enough, else None.\n\n    {\"word\": \"a lot\", \"value\": 2400.0 or None, \"money\": bool, \"unit\": \"bar\" or \"\"}\n    ");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" ?!.,;")]);
    line = stdlib_call("re", "sub", &[py_str("^(?:(?:good )?(?:hi|hiya|hey|hello|morning|afternoon|evening|yo)\\b[ ,!.]*)+"), py_str(""), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    line = stdlib_call("re", "sub", &[py_str("^(?:(?:and|so|but|ok|okay|alright|well|hmm|umm|er)[ ,]+)+"), py_str(""), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    line = mcall(&mcall(&g_TAG_ASK(), "sub", &[py_str(""), line.clone()]), "strip", &[py_str(" ?!.,;")]);
    impression = stdlib_call("re", "match", &[py_str("(?P<figure>[£$€]?\\s?\\d[\\d,]*(?:\\.\\d+)?)\\s+(?:seems|seemed|feels|felt|looks|sounds)\\s+(?P<rest>\\S.*)$"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&impression.clone()) {
    line = modulo(&py_str("is %s %s"), &py_tuple(vec![mcall(&mcall(&impression.clone(), "group", &[py_str("figure")]), "replace", &[py_str(" "), py_str("")]), mcall(&mcall(&impression.clone(), "group", &[py_str("rest")]), "strip", &[])]));
    }
    turned = stdlib_call("re", "match", &[py_str("(?P<figure>[£$€]?\\s?\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:pounds?|quid|euros?|dollars?)?\\s*(?:for\\s+(?!a\\s+(?:day|week|month|year|hour)\\b)(?P<thing>[a-z][\\w' ]{0,40}?))?\\s*[-,]?\\s*(?:is|was|would)\\s+(?:that|this|it)\\s+(?P<word>a lot|too much|expensive|reasonable|ok|okay|alright|fine|fair|about right|steep|dear|pricey|cheap)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&turned.clone()) {
    thing = mcall(&{ let __b = mcall(&turned.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    line = modulo(&py_str("is %s %s%s"), &py_tuple(vec![mcall(&mcall(&turned.clone(), "group", &[py_str("figure")]), "replace", &[py_str(" "), py_str("")]), mcall(&turned.clone(), "group", &[py_str("word")]), if py_truthy(&thing.clone()) { add(&py_str(" for "), &thing.clone()) } else { py_str("") }]));
    }
    opinion = stdlib_call("re", "search", &[py_str("(?P<figure>[£$€]?\\s?\\d[\\d,]*(?:\\.\\d+)?)(?P<between>[^0-9]{0,24}?)\\b(?:seems|seemed|feels|felt|looks|sounds)\\s+(?P<word>high|low|steep|expensive|cheap|dear|pricey|a lot|too much|too high|too low|about right|about average|a rip off|loads)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&opinion.clone()) {
    word = mcall(&g_WORD_ALIAS(), "get", &[mcall(&opinion.clone(), "group", &[py_str("word")]), mcall(&opinion.clone(), "group", &[py_str("word")])]);
    value = call_builtin("float", &[mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&opinion.clone(), "group", &[py_str("figure")]), "replace", &[py_str(","), py_str("")]), "replace", &[py_str("£"), py_str("")]), "replace", &[py_str("$"), py_str("")]), "replace", &[py_str("€"), py_str("")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    money = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("[£$€]"), mcall(&opinion.clone(), "group", &[py_str("figure")])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DOUBT_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(!py_is(&value.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = if py_truthy(&le(&g_YEAR(), &{ let __cmp10 = value.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &g_NEXT_YEAR()) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { { let __b = modulo(&value.clone(), &Py::Int(100)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&money.clone())) } } } } } })) } }) {
    return py_dict(vec![(py_str("word"), word.clone()), (py_str("value"), value.clone()), (py_str("money"), money.clone()), (py_str("unit"), py_str("")), (py_str("subject"), py_str("")), (py_str("rate"), Py::Bool(false))]);
    }
    }
    verb = stdlib_call("re", "match", &[py_str("(?:is|was|were|are|would|will|does|do|did|should|isn'?t|isnt|aren'?t|arent|that's|that is|thats|it's|it is|its|this is|am i being|am i|are they|are we being|have i been|did i)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&verb.clone()))) {
    bare = mcall(&g_WORD_ALIAS(), "get", &[line.clone(), line.clone()]);
    if py_truthy(&Py::Bool(py_contains(&g_BARE_DOUBTS(), &bare.clone()))) {
    return py_dict(vec![(py_str("word"), bare.clone()), (py_str("value"), Py::None), (py_str("money"), Py::Bool(false)), (py_str("unit"), py_str("")), (py_str("subject"), py_str("")), (py_str("rate"), Py::Bool(false))]);
    }
    return Py::None;
    }
    rest = mcall(&slice_or_raise(&line.clone(), &mcall(&verb.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    let __unpk2 = py_unpack(&py_tuple(vec![Py::None, Py::Bool(false), py_str("")]), 3, None);
    value = __unpk2[0].clone();
    money = __unpk2[1].clone();
    unit = __unpk2[2].clone();
    stated = Py::Bool(py_contains(&py_tuple(vec![py_str("that's"), py_str("that is"), py_str("thats"), py_str("it's"), py_str("it is"), py_str("its"), py_str("this is"), py_str("am i being"), py_str("am i"), py_str("are they"), py_str("are we being"), py_str("have i been")]), &mcall(&verb.clone(), "group", &[Py::Int(0)])));
    if py_truthy(&{ let __b = stated.clone(); if py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("(?:that|this|it|there|they|those)\\b"), rest.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    rest = mcall(&stdlib_call("re", "sub", &[py_str("^(?:that|this|it|there|they|those)\\b"), py_str(""), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __unpk3 = py_unpack(&if py_truthy(&stated.clone()) { _the_figure_in_the_doubt(&[rest.clone()]) } else { py_tuple(vec![Py::None, Py::Bool(false), py_str(""), rest.clone(), Py::Bool(false)]) }, 5, None);
    value = __unpk3[0].clone();
    money = __unpk3[1].clone();
    unit = __unpk3[2].clone();
    rest = __unpk3[3].clone();
    rate = __unpk3[4].clone();
    } else {
    let __unpk4 = py_unpack(&_the_figure_in_the_doubt(&[rest.clone()]), 5, None);
    value = __unpk4[0].clone();
    money = __unpk4[1].clone();
    unit = __unpk4[2].clone();
    rest = __unpk4[3].clone();
    rate = __unpk4[4].clone();
    }
    rest = mcall(&stdlib_call("re", "sub", &[py_str("^(?:be|being|really|actually|roughly)\\s+"), py_str(""), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { rest.clone() } }) {
    changed = Py::Bool(false);
    let __iter5 = py_iter_or_raise(&g_TAILS());
    for __item in __iter5 {
    tail = __item.clone();
    if py_truthy(&mcall(&rest.clone(), "endswith", &[add(&py_str(" "), &tail.clone())])) {
    rest = mcall(&slice_or_raise(&rest.clone(), &Py::None, &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), "strip", &[]);
    changed = Py::Bool(true);
    }
    }
    }
    subject = py_str("");
    for_thing = mcall(&g_FOR_THE_THING(), "search", &[rest.clone()]);
    if py_truthy(&{ let __b = for_thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&value.clone(), &Py::None)) } }) {
    thing = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&for_thing.clone(), "group", &[py_str("thing")]), "strip", &[]), "lower", &[]), "split", &[])]);
    short_thing = mcall(&g_FOR_THE_ARTICLE(), "sub", &[py_str(""), thing.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_POINTING_AT_SOMETHING(), &short_thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_POINTING_ONE(), "search", &[short_thing.clone()]))) } }) {
    subject = thing.clone();
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &Py::None, &mcall(&for_thing.clone(), "start", &[]), &Py::None), "strip", &[]);
    }
    rest = mcall(&g_WORD_ALIAS(), "get", &[rest.clone(), rest.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&g_DOUBT_WORDS(), &rest.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&value.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = if py_truthy(&le(&g_YEAR(), &{ let __cmp60 = value.clone(); __cmp60.clone() })) { le(&__cmp60.clone(), &g_NEXT_YEAR()) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { { let __b = modulo(&value.clone(), &Py::Int(100)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&money.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unit.clone())) } } } } } } } }) {
    return Py::None;
    }
    return py_dict(vec![(py_str("word"), rest.clone()), (py_str("value"), value.clone()), (py_str("money"), money.clone()), (py_str("unit"), unit.clone()), (py_str("subject"), subject.clone()), (py_str("rate"), rate.clone())]);
    Py::None
}
pub fn _doubt_ranges(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut span: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("(money, measured) spans the answer states, each a dict of its two ends and its own words.\n\n    An answer often holds more than one (\"raw chicken keeps 1 to 2 days, cooked chicken 3 days\",\n    \"£1,800 to £3,000, and £3,000 to £4,500 with the pipework\"), so every range is kept and the\n    one the figure falls in is the one it is judged against.\n    ");
    text = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    money = __unpk1[0].clone();
    measured = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_MONEY_RANGE(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&found.clone(), "group", &[py_str("mark")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("mark2")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("word")]) } } } }))) {
    continue;
    }
    low = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("low")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    high = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("high")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &high.clone()))) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("min", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("max", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    low = __unpk3[0].clone();
    high = __unpk3[1].clone();
    mark = { let __b = mcall(&found.clone(), "group", &[py_str("mark")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("mark2")]); if py_truthy(&__b) { __b } else { py_str("£") } } } };
    let _ = mcall(&money.clone(), "append", &[py_dict(vec![(py_str("low"), low.clone()), (py_str("high"), high.clone()), (py_str("mark"), mark.clone()), (py_str("money"), Py::Bool(true)), (py_str("unit"), py_str("")), (py_str("at"), mcall(&found.clone(), "start", &[])), (py_str("text"), modulo(&py_str("%s to %s"), &py_tuple(vec![_money(&[low.clone(), mark.clone()]), _money(&[high.clone(), mark.clone()])])))])]);
    }
    let __iter4 = py_iter_or_raise(&mcall(&g_RANGE(), "finditer", &[text.clone()]));
    for __item in __iter4 {
    found = __item.clone();
    unit = mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_DOUBT_UNITS(), &unit.clone()))) {
    continue;
    }
    low = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    high = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &high.clone()))) {
    continue;
    }
    let __unpk5 = py_unpack(&py_tuple(vec![call_builtin("min", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("max", &[low.clone(), high.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    low = __unpk5[0].clone();
    high = __unpk5[1].clone();
    let _ = mcall(&measured.clone(), "append", &[py_dict(vec![(py_str("low"), low.clone()), (py_str("high"), high.clone()), (py_str("mark"), py_str("")), (py_str("money"), Py::Bool(false)), (py_str("unit"), unit.clone()), (py_str("at"), mcall(&found.clone(), "start", &[])), (py_str("end"), mcall(&found.clone(), "end", &[])), (py_str("text"), modulo(&py_str("%s to %s%s"), &py_tuple(vec![_say(&[low.clone()]), _say(&[high.clone()]), _unit(&[unit.clone()])])))])]);
    }
    let __iter6 = py_iter_or_raise(&mcall(&g_WORD_RANGE(), "finditer", &[text.clone()]));
    for __item in __iter6 {
    found = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(measured.clone()); __args.push(_word_number(&[mcall(&found.clone(), "group", &[Py::Int(1)])])); __args.push(_word_number(&[mcall(&found.clone(), "group", &[Py::Int(2)])])); __args.push(mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])); __args.push(_usual_unit(&[text.clone(), mcall(&found.clone(), "start", &[])])); __kws.extend(vec![("at".to_string(), mcall(&found.clone(), "start", &[])), ("matched".to_string(), mcall(&found.clone(), "group", &[Py::Int(0)]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _add_measured(args))(&__args) };
    }
    measured = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let mut span: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_an_age(&[text.clone(), span.clone()])))) {  __out.push(span.clone()); } } py_list(__out) })() };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let found = found.clone(); let high = high.clone(); let low = low.clone(); let mark = mark.clone(); let measured = measured.clone(); let money = money.clone(); let said = said.clone(); let span = span.clone(); let text = text.clone(); let unit = unit.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); mcall(&span.clone(), "get", &[py_str("at"), Py::Int(0)]) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&measured.clone(), "sort", args))(&__args) };
    return py_tuple(vec![money.clone(), measured.clone()]);
    Py::None
}
pub fn _an_age(args: &[Py]) -> Py {
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
    let mut span: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span"))) };
    let mut end: Py = Py::None;
    let _ = py_str("True when the range is somebody's age (\"one to three year old\"), not a stretch of time.");
    end = mcall(&span.clone(), "get", &[py_str("end")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[end.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_AN_AGE_AFTER(), "match", &[slice_or_raise(&text.clone(), &end.clone(), &add(&end.clone(), &Py::Int(24)), &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _money(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut mark: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mark") { v.clone() }
    else { py_str("£") };
    let mut body: Py = Py::None;
    let _ = py_str("A price as a person writes it: 2400 is £2,400.");
    body = if py_truthy(&mcall(&call_builtin("float", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "is_integer", &[])) { mcall(&py_str("{:,}"), "format", &[call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]) } else { mcall(&py_str("{:,.2f}"), "format", &[value.clone()]) };
    return modulo(&py_str("%s%s"), &py_tuple(vec![mark.clone(), body.clone()]));
    Py::None
}
pub fn _they_named_the_tier(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut qualifier: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "qualifier") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: qualifier"))) };
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut spoken: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the person's own words name what a later range is for (\"in London\", \"a whole roof\").");
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(false);
    }
    spoken = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = qualifier.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__SAYS_NOTHING_ABOUT_THE_TIER(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&spoken.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&spoken.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_range_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut spans: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spans"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut mine: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { py_str("") };
    let mut after: Py = Py::None;
    let mut apart: Py = Py::None;
    let mut before: Py = Py::None;
    let mut end: Py = Py::None;
    let mut place: Py = Py::None;
    let mut qualifier: Py = Py::None;
    let mut span: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The range the figure is about among several, or None to keep the answer's leading one.\n\n    An answer often names several prices: some are tiers of the same job (\"£1,800 to £3,000, and\n    £3,000 to £4,500 if the pipework has to change\") and some are different jobs (\"a patch is £150\n    to £400, a flat roof on a garage £1,500 to £3,000, a whole roof £5,000 to £15,000\"). A quote\n    that falls in a later *tier* is above the usual, which is the question asked. A quote that\n    falls in a range for another job is about that job: \"i got quoted 400 to fix the roof\" is the\n    patch, and read against the roofer's £200 to £300 a day it came back \"Yes, a bit much\".\n\n    A later range that is *dearer* than the one the answer leads with is a special case of the same\n    thing - \"£4 to £5.50 in most pubs and £6 to £7.50 in London\" - and the answer leads with what\n    most people pay, so it wins the figure only when the person's own words put them in it\n    (\"we're in london this weekend, is 7 a lot\") or when the leading range is a rate. A £6 pint\n    read against the London price came back \"No - £6 to £7.50 is the usual, so you are inside it\"\n    for somebody who had said nothing about London.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&value.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[spans.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    if py_truthy(&if py_truthy(&le(&geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("low")), &{ let __cmp10 = value.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("high"))) } else { Py::Bool(false) }) {
    return Py::None;
    }
    text = call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    apart = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&spans.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    place = __unpk3[0].clone();
    span = __unpk3[1].clone();
    before = slice_or_raise(&text.clone(), &add(&geti(&geti(&spans.clone(), &sub(&place.clone(), &Py::Int(1))), &py_str("at")), &call_builtin("len", &[geti(&geti(&spans.clone(), &sub(&place.clone(), &Py::Int(1))), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e))), &geti(&span.clone(), &py_str("at")), &Py::None);
    end = add(&geti(&span.clone(), &py_str("at")), &call_builtin("len", &[geti(&span.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e)));
    after = slice_or_raise(&text.clone(), &end.clone(), &if py_truthy(&lt(&add(&place.clone(), &Py::Int(1)), &call_builtin("len", &[spans.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&geti(&spans.clone(), &add(&place.clone(), &Py::Int(1))), &py_str("at")) } else { call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)) }, &Py::None);
    after = geti(&stdlib_call("re", "split", &[py_str("[.;:!?]| - |\\band\\b"), after.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__SAME_JOB(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__SAME_JOB(), "search", &[after.clone()]) } }))) {
    let _ = mcall(&apart.clone(), "append", &[py_tuple(vec![span.clone(), add(&add(&before.clone(), &py_str(" ")), &after.clone())])]);
    }
    }
    let __iter4 = py_iter_or_raise(&apart.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    span = __unpk5[0].clone();
    qualifier = __unpk5[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&geti(&span.clone(), &py_str("low")), &{ let __cmp60 = value.clone(); __cmp60.clone() })) { le(&__cmp60.clone(), &geti(&span.clone(), &py_str("high"))) } else { Py::Bool(false) }))) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&geti(&span.clone(), &py_str("high")), &geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("high"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_they_named_the_tier(&[qualifier.clone(), mine.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_RATE(), "match", &[slice_or_raise(&text.clone(), &add(&geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("at")), &call_builtin("len", &[geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None)]))) } } } }) {
    continue;
    }
    return py_list(vec![span.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _the_person_words(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line and the last few things the person said, as one string.\n\n    A later range in an answer can be a special case of the same thing (\"£6 to £7.50 in London\"),\n    and only the person's own words say whether it is their case.\n    ");
    words = py_list(vec![call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "lower", &[]);
    Py::None
}
pub fn _the_span(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut mine: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { py_str("") };
    let mut both: Py = Py::None;
    let mut inside: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut priced: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The span the figure is judged against, or None when the answer states none it can be.");
    let __unpk1 = py_unpack(&_doubt_ranges(&[said.clone()]), 2, None);
    money = __unpk1[0].clone();
    measured = __unpk1[1].clone();
    if py_truthy(&geti(&doubt.clone(), &py_str("money"))) {
    spans = { let __b = _the_range_for(&[geti(&doubt.clone(), &py_str("value")), money.clone(), said.clone(), mine.clone()]); if py_truthy(&__b) { __b } else { money.clone() } };
    } else {
    if py_truthy(&geti(&doubt.clone(), &py_str("unit"))) {
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let mut span: Py = __item.clone(); if py_truthy(&_same_measure(&[geti(&doubt.clone(), &py_str("unit")), geti(&span.clone(), &py_str("unit"))])) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&doubt.clone(), "get", &[py_str("rate")])) {
    priced = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&money.clone()) { let mut span: Py = __item.clone(); if py_truthy(&stdlib_call("re", "search", &[add(&add(&add(&stdlib_call("re", "escape", &[geti(&span.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e)), &py_str("\\s*(?:an?|per|each)?\\s*")), &stdlib_call("re", "escape", &[geti(&doubt.clone(), &py_str("unit"))]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone(), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&priced.clone()) {
    spans = priced.clone();
    }
    }
    } else {
    measured = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let mut span: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_A_GAUGE_UNIT(), &geti(&span.clone(), &py_str("unit"))))) {  __out.push(span.clone()); } } py_list(__out) })() };
    both = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(add(&money.clone(), &measured.clone())); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let both = both.clone(); let doubt = doubt.clone(); let inside = inside.clone(); let measured = measured.clone(); let mine = mine.clone(); let money = money.clone(); let priced = priced.clone(); let said = said.clone(); let span = span.clone(); let spans = spans.clone(); let value = value.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); geti(&span.clone(), &py_str("at")) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    inside = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&both.clone()) { let mut span: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None)); if !py_truthy(&__b) { __b } else { if py_truthy(&le(&geti(&span.clone(), &py_str("low")), &{ let __cmp10 = geti(&doubt.clone(), &py_str("value")); __cmp10.clone() })) { le(&__cmp10.clone(), &geti(&span.clone(), &py_str("high"))) } else { Py::Bool(false) } } }) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = inside.clone(); if !py_truthy(&__b) { __b } else { { let __b = money.clone(); if !py_truthy(&__b) { __b } else { measured.clone() } } } }) {
    spans = py_list(vec![geti(&inside.clone(), &Py::Int(0))]);
    } else {
    spans = { let __b = _the_range_for(&[geti(&doubt.clone(), &py_str("value")), both.clone(), said.clone(), mine.clone()]); if py_truthy(&__b) { __b } else { { let __b = money.clone(); if py_truthy(&__b) { __b } else { measured.clone() } } } };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&money.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = spans.clone(); if !py_truthy(&__b) { __b } else { { let __b = gt(&geti(&doubt.clone(), &py_str("value")), &mul(&geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("high")), &Py::Int(10))); if py_truthy(&__b) { __b } else { lt(&mul(&geti(&doubt.clone(), &py_str("value")), &Py::Int(10)), &geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("low"))) } } } } } } } }) {
    return Py::None;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push(py_tuple(vec![geti(&span.clone(), &py_str("low")), geti(&span.clone(), &py_str("high"))])); } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = lt(&mul(&geti(&doubt.clone(), &py_str("value")), &Py::Int(2)), &geti(&geti(&spans.clone(), &Py::Int(0)), &py_str("low"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push(if py_truthy(&le(&geti(&span.clone(), &py_str("low")), &{ let __cmp10 = geti(&doubt.clone(), &py_str("value")); __cmp10.clone() })) { le(&__cmp10.clone(), &geti(&span.clone(), &py_str("high"))) } else { Py::Bool(false) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None)); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[spans.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    value = geti(&doubt.clone(), &py_str("value"));
    if py_truthy(&{ let __b = gt(&value.clone(), &call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push(geti(&span.clone(), &py_str("high"))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { lt(&value.clone(), &call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push(geti(&span.clone(), &py_str("low"))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let both = both.clone(); let doubt = doubt.clone(); let inside = inside.clone(); let measured = measured.clone(); let mine = mine.clone(); let money = money.clone(); let priced = priced.clone(); let said = said.clone(); let span = span.clone(); let spans = spans.clone(); let value = value.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); call_builtin("max", &[sub(&geti(&span.clone(), &py_str("low")), &value.clone()), sub(&value.clone(), &geti(&span.clone(), &py_str("high"))), Py::Float(0.0)]).unwrap_or_else(|e| exc::raise(e)) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return geti(&spans.clone(), &Py::Int(0));
    Py::None
}
pub fn _the_side_asked(args: &[Py]) -> Py {
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
    let _ = py_str("Which side of the range the doubt's own word asks about, or None when it names neither.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_TOO_MUCH_WORDS(), &word.clone()))) {
    return py_str("high");
    }
    if py_truthy(&Py::Bool(py_contains(&g_TOO_LITTLE_WORDS(), &word.clone()))) {
    return py_str("low");
    }
    return Py::None;
    Py::None
}
pub fn _the_side_decides(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut doubt: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut low: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut low_side: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low_side") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low_side"))) };
    let mut found_side: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found_side") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found_side"))) };
    let mut asked: Py = Py::None;
    let _ = py_str("The frames the doubt's own word asks for, on the side the figure is on.\n\n    A doubt has two parts - the side its own word asks about, and the side the figure is on - and\n    a verdict has to answer both. \"is that too high\" at a figure under the range is a No that\n    points the other way (`found_side`), \"is that too low\" at the same figure is a Yes\n    (`low_side`), and a doubt that names no side (\"about right\", \"a fair price\", \"the going\n    rate\") is answered by the range: the frames the range gave it, which are `found_side` when the\n    figure is under it and `options` otherwise.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("enough"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TIME_DOUBT(), &geti(&doubt.clone(), &py_str("word")))) } }) {
    return options.clone();
    }
    asked = _the_side_asked(&[geti(&doubt.clone(), &py_str("word"))]);
    if py_truthy(&if py_truthy(&le(&low.clone(), &{ let __cmp10 = value.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &high.clone()) } else { Py::Bool(false) }) {
    return if py_truthy(&Py::Bool(py_eq(&asked.clone(), &py_str("low")))) { g_NOT_TOO_LITTLE() } else { options.clone() };
    }
    if py_truthy(&lt(&value.clone(), &low.clone())) {
    return if py_truthy(&Py::Bool(py_eq(&asked.clone(), &py_str("low")))) { low_side.clone() } else { found_side.clone() };
    }
    return options.clone();
    Py::None
}
pub fn _judged(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut mine: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { py_str("") };
    let mut aside: Py = Py::None;
    let mut high: Py = Py::None;
    let mut line: Py = Py::None;
    let mut long: Py = Py::None;
    let mut low: Py = Py::None;
    let mut options: Py = Py::None;
    let mut span: Py = Py::None;
    let mut span_text: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The verdict on the figure in the answer's own range, or None when nothing can be judged.");
    value = geti(&doubt.clone(), &py_str("value"));
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::None;
    }
    span = _the_span(&[doubt.clone(), said.clone(), mine.clone()]);
    if py_truthy(&Py::Bool(py_is(&span.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&span.clone(), &py_str("low")), geti(&span.clone(), &py_str("high")), geti(&span.clone(), &py_str("text"))]), 3, None);
    low = __unpk1[0].clone();
    high = __unpk1[1].clone();
    span_text = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&g_TIME_DOUBT(), &geti(&doubt.clone(), &py_str("word"))))) {
    long = Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("too long")));
    if py_truthy(&if py_truthy(&le(&low.clone(), &{ let __cmp20 = value.clone(); __cmp20.clone() })) { le(&__cmp20.clone(), &high.clone()) } else { Py::Bool(false) }) {
    options = if py_truthy(&long.clone()) { g_NOT_TOO_LONG_INSIDE() } else { g_NOT_TOO_SHORT() };
    } else {
    if py_truthy(&gt(&value.clone(), &high.clone())) {
    options = if py_truthy(&long.clone()) { g_ON_THE_LONG_SIDE() } else { g_NOT_TOO_SHORT() };
    } else {
    options = if py_truthy(&long.clone()) { g_NOT_TOO_LONG() } else { g_ON_THE_SHORT_SIDE() };
    }
    }
    } else {
    if py_truthy(&if py_truthy(&le(&low.clone(), &{ let __cmp30 = value.clone(); __cmp30.clone() })) { le(&__cmp30.clone(), &high.clone()) } else { Py::Bool(false) }) {
    if py_truthy(&Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("enough")))) {
    options = g_ENOUGH();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_TOO_MUCH_WORDS(), &geti(&doubt.clone(), &py_str("word"))))) {
    options = g_NOT_TOO_MUCH();
    } else {
    options = g_ABOUT_RIGHT();
    }
    }
    } else {
    if py_truthy(&gt(&value.clone(), &high.clone())) {
    if py_truthy(&Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("enough")))) {
    options = g_MORE_THAN_ENOUGH();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_TOO_MUCH_WORDS(), &geti(&doubt.clone(), &py_str("word"))))) {
    options = g_ON_THE_HIGH_SIDE();
    } else {
    options = g_ABOVE_THE_USUAL();
    }
    }
    } else {
    options = if py_truthy(&Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("enough")))) { g_UNDER_THE_USUAL() } else { g_ON_THE_LOW_SIDE() };
    }
    }
    }
    options = _the_side_decides(&[options.clone(), doubt.clone(), value.clone(), low.clone(), high.clone(), g_ON_THE_LOW_SIDE(), g_THE_LOW_SIDE_FOUND()]);
    line = py_call(&crate::gen::compose::verdict::g_pick(), &[call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)), py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), span_text.clone())])]))]), span_text.clone()]);
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), span_text.clone()), ("value".to_string(), _money(&[value.clone(), { let __b = geti(&span.clone(), &py_str("mark")); if py_truthy(&__b) { __b } else { py_str("£") } }]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    aside = _the_band_aside(&[said.clone(), span_text.clone()]);
    if py_truthy(&aside.clone()) {
    line = _with_the_aside(&[line.clone(), span_text.clone(), aside.clone()]);
    }
    return line.clone();
    Py::None
}
pub fn _the_band_aside(args: &[Py]) -> Py {
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
    let mut span_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span_text"))) };
    let mut after: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("What the answer says the range buys, or \"\" - the clause after the range, when it has one.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&span_text.clone())) } }) {
    return py_str("");
    }
    found = mcall(&said.clone(), "find", &[span_text.clone()]);
    if py_truthy(&lt(&found.clone(), &Py::Int(0))) {
    return py_str("");
    }
    after = slice_or_raise(&said.clone(), &add(&found.clone(), &call_builtin("len", &[span_text.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    end = mcall(&g__ASIDE_ENDS(), "search", &[after.clone()]);
    clause = mcall(&if py_truthy(&end.clone()) { slice_or_raise(&after.clone(), &Py::None, &mcall(&end.clone(), "start", &[]), &Py::None) } else { after.clone() }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(70)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__BUYS(), "match", &[clause.clone()])))) {
    return py_str("");
    }
    return mcall(&clause.clone(), "rstrip", &[py_str(" .,;:")]);
    Py::None
}
pub fn _with_the_aside(args: &[Py]) -> Py {
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
    let mut span_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span_text"))) };
    let mut aside: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "aside") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: aside"))) };
    let mut body: Py = Py::None;
    let _ = py_str("The verdict with the answer's own clause on the end, when the verdict ends on the range.");
    body = mcall(&line.clone(), "rstrip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&body.clone(), "endswith", &[add(&span_text.clone(), &py_str("."))])))) {
    return line.clone();
    }
    return add(&add(&add(&slice_or_raise(&body.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str(", which ")), &aside.clone()), &py_str("."));
    Py::None
}
pub fn _in_the_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut wanted: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut mine: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("`value` counted in `wanted`, when both units count time, or None.");
    mine = mcall(&g_DAYS_IN(), "get", &[mcall(&mcall(&call_builtin("str", &[{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])]);
    theirs = mcall(&g_DAYS_IN(), "get", &[mcall(&mcall(&call_builtin("str", &[{ let __b = wanted.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&theirs.clone())) } }) {
    return Py::None;
    }
    return truediv(&mul(&value.clone(), &mine.clone()), &theirs.clone());
    Py::None
}
pub fn _a_stretch_of_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("True when a span's unit counts time: days, weeks and months can be too long, bars cannot.");
    return Py::Bool(py_contains(&g_DAYS_IN(), &mcall(&mcall(&call_builtin("str", &[{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])));
    Py::None
}
pub fn _the_usual_is_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut span: Py = Py::None;
    let _ = py_str("The verdict on the answer's own usual stretch of time, for \"is that too long\" with no figure.\n\n    \"how long does back pain last\" answers two to six weeks, and \"is that too long\" is a question\n    about that stretch, not a request to have it said again: the answer came back as \"Same as\n    before:\" with the whole paragraph under it, which repeats it and answers nothing. So the range\n    the bot itself just gave is the yardstick, and its own end is the figure held up: \"No - 2 to 6\n    weeks is the usual for it.\"\n\n    Only a time doubt comes here. \"is that enough\" and \"is that a lot\" ask for the figure instead,\n    because the memory store reads their own count against the answer already, and a person who gave\n    none is better asked for it than told that a range nobody named a figure for is fine.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&g_TIME_DOUBT(), &mcall(&doubt.clone(), "get", &[py_str("word")])))) {
    return Py::None;
    }
    span = _the_span(&[doubt.clone(), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&span.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = mcall(&span.clone(), "get", &[py_str("money")]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_stretch_of_time(&[mcall(&span.clone(), "get", &[py_str("unit")])]))) } } } }) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_USUAL_NO()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), geti(&span.clone(), &py_str("text")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), geti(&span.clone(), &py_str("text"))]);
    Py::None
}
pub fn _a_word_figure(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("{value, money, unit} for a figure the person wrote in words, or None.\n\n    The last one in the line, because a line that counts twice is talking about the later count: \"a\n    week of nights then five days off\" holds five days up, not the week before it.\n    ");
    found = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_A_WORD_FIGURE(), "finditer", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    found = __item.clone();
    }
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::None;
    }
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    value = _word_number(&[word.clone()]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    value = mcall(&g_WORD_FIGURE_COUNTS(), "get", &[word.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::None;
    }
    unit = mcall(&g_UNIT_ALIAS(), "get", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[]), mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[])]);
    return py_dict(vec![(py_str("value"), value.clone()), (py_str("money"), Py::Bool(false)), (py_str("unit"), unit.clone())]);
    Py::None
}
pub fn _the_last_figure_in(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The last figure in one of their lines, in digits or in words, or None.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    found = call_builtin("list", &[mcall(&g_FIGURE(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    figure = _the_figure_in(&[said.clone(), geti(&found.clone(), &neg(&Py::Int(1)))]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    }
    return _a_word_figure(&[said.clone()]);
    Py::None
}
pub fn _in_the_same_league(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let _ = py_str("Whether a range in this answer could be about a figure this size.");
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&_doubt_ranges(&[said.clone()]), 2, None);
    money = __unpk1[0].clone();
    measured = __unpk1[1].clone();
    spans = { let __b = money.clone(); if py_truthy(&__b) { __b } else { measured.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push({ let __b = le(&geti(&span.clone(), &py_str("low")), &mul(&value.clone(), &g_SAME_LEAGUE())); if !py_truthy(&__b) { __b } else { le(&value.clone(), &mul(&geti(&span.clone(), &py_str("high")), &g_SAME_LEAGUE())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _answer_before(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { Py::None };
    let mut back: Py = Py::None;
    let mut elsewhere: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("(the knowledge answer, its index) the doubt is about, or (\"\", -1) when there is none.\n\n    `value` is the figure in hand. The nearest answer with a range in it is the one the doubt is\n    about, unless its ranges are in another league from the figure: then the answer the figure is\n    about is further back, and the nearest answer that could hold it is the one read. An answer\n    whose ranges are all in another league is still better than nothing, so it is the fallback.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_tuple(vec![py_str(""), neg(&Py::Int(1))]);
    }
    elsewhere = py_tuple(vec![py_str(""), neg(&Py::Int(1))]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), add(&call_builtin("min", &[g_LOOK_BACK(), call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    back = __item.clone();
    turn = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&back.clone())), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&back.clone())) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = { let __b = said.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_RANGE(), "finditer", &[said.clone()])) { let mut found: Py = __item.clone(); __out.push({ let __b = mcall(&found.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MONEY_RANGE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORD_RANGE(), "search", &[said.clone()]) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&value.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _in_the_same_league(&[value.clone(), said.clone()]) } }) {
    return py_tuple(vec![said.clone(), sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone())]);
    }
    if py_truthy(&lt(&geti(&elsewhere.clone(), &Py::Int(1)), &Py::Int(0))) {
    elsewhere = py_tuple(vec![said.clone(), sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone())]);
    }
    }
    }
    if py_truthy(&{ let __b = ge(&geti(&elsewhere.clone(), &Py::Int(1)), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&value.clone(), &Py::None)) } }) {
    return elsewhere.clone();
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), add(&call_builtin("min", &[g_LOOK_BACK(), call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    back = __item.clone();
    turn = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&back.clone())), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&back.clone())) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_eq(&mcall(&turn.clone(), "get", &[py_str("module")]), &py_str("answers")))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_a_lone_price(&[said.clone()])) {
    return py_tuple(vec![said.clone(), sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &back.clone())]);
    }
    }
    return elsewhere.clone();
    Py::None
}
pub fn _figure_since(args: &[Py]) -> Py {
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
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut narrow: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "narrow") { v.clone() }
    else { Py::Bool(false) };
    let mut figure: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last figure the person gave around the answer, or None.\n\n    \"Around\" because the figure usually comes in the same turn as the answer: the person says\n    what they are being charged (\"my boiler service is 90 pounds\"), the bot answers with the usual\n    range, and the next line is \"is that a lot\". The answer's own turn holds their words, so it is\n    read too - and only then the turns after it, which is where a quote given as its own line\n    lands (\"they want 250\" / \"is that too much\").\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    said_before = py_str("");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&narrow.clone())); if !py_truthy(&__b) { __b } else { { let __b = if py_truthy(&le(&Py::Int(0), &{ let __cmp10 = after.clone(); __cmp10.clone() })) { lt(&__cmp10.clone(), &call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &after.clone()), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    said_before = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &after.clone()), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&said_before.clone()) {
    figure = _the_last_figure_in(&[said_before.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &add(&after.clone(), &Py::Int(1)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { py_str("") };
    figure = _the_last_figure_in(&[said.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&narrow.clone())); if !py_truthy(&__b) { __b } else { { let __b = ge(&after.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &sub(&after.clone(), &Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    figure = _the_last_figure_in(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &sub(&after.clone(), &Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_clock_time(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("Whether this number of the line names an hour of the day rather than a count of anything.\n\n    \"at 250\" is a sum said without its pound sign, so only a number that could be an hour of the\n    day is fenced by the word before it; \"5am\" and \"5 o'clock\" say what they are.\n    ");
    if py_truthy(&mcall(&g_A_CLOCK_TIME_AFTER(), "match", &[slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return Py::Bool(true);
    }
    return { let __b = if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = value.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(24)) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_CLOCK_TIME_BEFORE(), "search", &[slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_figure_in(args: &[Py]) -> Py {
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
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("{value, money, unit} for one number in the person's line, or None when it is not a figure.\n\n    A year (\"1998\") and an age (\"im 52\") are numbers a line uses without asking anything to be\n    judged, so they are not figures here. Nor is an hour of the day: \"at 5\" and \"5.30\" say when\n    something happens, and a unit after the number (\"5 hours\", \"5 weeks\") is what makes it a count.\n    ");
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("number")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    money = call_builtin("bool", &[mcall(&found.clone(), "group", &[py_str("mark")])]).unwrap_or_else(|e| exc::raise(e));
    unit = py_str("");
    measured = mcall(&g_UNIT(), "match", &[mcall(&slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])]);
    if py_truthy(&measured.clone()) {
    unit = mcall(&mcall(&measured.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    }
    if py_truthy(&{ let __b = if py_truthy(&le(&g_YEAR(), &{ let __cmp10 = value.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &g_NEXT_YEAR()) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { { let __b = modulo(&value.clone(), &Py::Int(100)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&money.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unit.clone())) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_AN_AGE(), "match", &[mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[])])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&unit.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&money.clone())); if !py_truthy(&__b) { __b } else { _a_clock_time(&[said.clone(), found.clone(), value.clone()]) } } } }) {
    return Py::None;
    }
    return py_dict(vec![(py_str("value"), value.clone()), (py_str("money"), money.clone()), (py_str("unit"), unit.clone())]);
    Py::None
}
pub fn _a_lone_price(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut found: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The text of a single price in the answer, when the answer gave one and no range.");
    text = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&_doubt_ranges(&[text.clone()]), 2, None);
    money = __unpk1[0].clone();
    measured = __unpk1[1].clone();
    if py_truthy(&money.clone()) {
    return py_str("");
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_A_LONE_PRICE(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    amount = { let __b = mcall(&found.clone(), "group", &[py_str("amount")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("amount2")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&amount.clone()))) {
    continue;
    }
    let __try3 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&amount.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["ValueError"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&le(&value.clone(), &Py::Int(0))) {
    continue;
    }
    word = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("word")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = mcall(&found.clone(), "group", &[py_str("mark")]); if py_truthy(&__b) { __b } else { word.clone() } })); if !py_truthy(&__b) { __b } else { if py_truthy(&le(&g_YEAR(), &{ let __cmp80 = value.clone(); __cmp80.clone() })) { le(&__cmp80.clone(), &g_NEXT_YEAR()) } else { Py::Bool(false) } } }) {
    continue;
    }
    return mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]);
    }
    return py_str("");
    Py::None
}
pub fn asks_for_the_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut options: Py = Py::None;
    let mut single: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The reply that asks for the figure a doubt is about, or None when it is not that doubt.");
    if py_truthy(&Py::Bool(!py_contains(&g_QUANTITY_WORDS(), &geti(&doubt.clone(), &py_str("word"))))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_doubt_ranges(&[said.clone()]), 2, None);
    money = __unpk1[0].clone();
    measured = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_MONEY_DOUBTS(), &geti(&doubt.clone(), &py_str("word"))))) {
    spans = money.clone();
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    spoken = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let mut span: Py = __item.clone(); if py_truthy(&mcall(&g_UNIT_WORDS(), "get", &[mcall(&span.clone(), "get", &[py_str("unit")])])) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&spoken.clone()) {
    span = geti(&spoken.clone(), &Py::Int(0));
    unit = geti(&g_UNIT_WORDS(), &geti(&span.clone(), &py_str("unit")));
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_RANGE_THAT_IS_NOT_A_PRICE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), geti(&span.clone(), &py_str("text"))), ("unit".to_string(), unit.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    }
    single = _a_lone_price(&[said.clone()]);
    if py_truthy(&single.clone()) {
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_PRICE_IT_GAVE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), single.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    }
    if py_truthy(&{ let __b = measured.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SAID_LOW(), &geti(&doubt.clone(), &py_str("word")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAID_HIGH(), &geti(&doubt.clone(), &py_str("word")))) } } } }) {
    span = geti(&geti(&measured.clone(), &Py::Int(0)), &py_str("text"));
    options = if py_truthy(&Py::Bool(py_contains(&g_SAID_LOW(), &geti(&doubt.clone(), &py_str("word"))))) { g_BELOW_THE_RANGE() } else { g_ABOVE_THE_RANGE() };
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), span.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    }
    if py_truthy(&measured.clone()) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::verdict::g_pick(), &[call_builtin("list", &[g_NO_PRICE_SAID()]).unwrap_or_else(|e| exc::raise(e)), py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    }
    } else {
    spans = { let __b = money.clone(); if py_truthy(&__b) { __b } else { measured.clone() } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return Py::None;
    }
    span = geti(&spans.clone(), &Py::Int(0));
    if py_truthy(&geti(&span.clone(), &py_str("money"))) {
    options = g_ASKS_FOR_A_PRICE();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_SAID_LOW(), &geti(&doubt.clone(), &py_str("word"))))) {
    options = g_BELOW_THE_RANGE();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_SAID_HIGH(), &geti(&doubt.clone(), &py_str("word"))))) {
    options = g_ABOVE_THE_RANGE();
    } else {
    options = g_THE_RANGE_ITSELF();
    }
    }
    }
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("span".to_string(), geti(&span.clone(), &py_str("text")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    Py::None
}
pub fn _doubt_and_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut at: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("(the doubt, the answer it is about, that answer's turn) or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    doubt = _doubt(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&doubt.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_answer_before(&[memory.clone()]), 2, None);
    said = __unpk1[0].clone();
    at = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&doubt.clone(), "get", &[py_str("subject")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_answer_is_about(&[said.clone(), geti(&doubt.clone(), &py_str("subject"))]))) } }) {
    return Py::None;
    }
    return py_tuple(vec![doubt.clone(), said.clone(), at.clone()]);
    Py::None
}
pub fn _the_answer_is_about(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut there: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the answer's opening is about the thing the figure in the line is for.\n\n    Only the opening sentence, which is where a price answer names what it is pricing, and only a\n    word they share: \"a haircut\" is the \"cut\" of \"A cut is usually £15 to £35 in a salon\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&_the_opening_sentence(&[said.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    there = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&there.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&word.clone(), &there.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _only_says_that(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let _ = py_str("True for a doubt word that says \"that\" and no more (\"normal\", \"ok\", \"reasonable\", \"fine\").\n\n    These are about the thing being usual or the advice standing, and the memory store reads the\n    figure that came with them. \"is that normal\" after \"its gone up to 90 a month\" is about the\n    bill, not about placing 90 in a range, so the figure read for it stays with main's narrow one:\n    the turns after the answer, which is where a quote given as its own line lands (\"what does a\n    plumber charge\" / \"I was quoted 300\" / \"is that normal\", still judged). A word that names the\n    doubt itself (\"is that a lot\", \"is that enough\") reads the answer's own turn and one turn back\n    as well, because that is where the price they are being charged usually sits and nothing else\n    reads it.\n    ");
    return Py::Bool(!py_contains(&g_QUANTITY_WORDS(), &mcall(&doubt.clone(), "get", &[py_str("word")])));
    Py::None
}
pub fn _the_usuals(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut money: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The price ranges the knowledge base gives for this thing, or () when it gives none.\n\n    The facts are already written down for \"how much does a haircut cost\", and the person asking\n    \"is £40 a lot for a haircut\" wants the same range with their own figure placed in it. So the\n    subject they named is put to the knowledge base as the price question it is, and the ranges in\n    that answer are the yardstick. Nothing is guessed: a subject the base has no price for comes\n    back empty, and the line is left to the modules that know better.\n    ");
    short = mcall(&g_FOR_THE_ARTICLE(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&short.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&short.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return py_tuple(vec![]);
    }
    let __try1 = exc::catch(|| {
    let mut lookup: Py = crate::gen::knowledge::lookup::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return py_tuple(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&py_tuple(vec![modulo(&py_str("how much does a %s cost"), &short.clone()), modulo(&py_str("how much does %s cost"), &short.clone()), modulo(&py_str("how much is a %s"), &short.clone())]));
    for __item in __iter6 {
    question = __item.clone();
    let __try7 = exc::catch(|| {
    found = mcall(&lookup.clone(), "find", &[py_dict(vec![(py_str("clean"), question.clone()), (py_str("words"), mcall(&question.clone(), "split", &[]))])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    reply = call_builtin("getattr", &[found.clone(), py_str("reply"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try12 = exc::catch(|| {
    reply = py_call(&reply, &[py_dict(vec![]), py_dict(vec![(py_str("clean"), question.clone()), (py_str("words"), mcall(&question.clone(), "split", &[]))])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret13: Py = Py::Bool(false);
    let mut __val14: Py = Py::None;
    let mut __unhandled15: Option<exc::PyExc> = None;
    match __try12 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret13 = __p[0].clone(); __val14 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled16 = false;
            if !__handled16 && exc::matches_any(&__e, &["Exception"]) {
                __handled16 = true;
    continue;
            }
            if !__handled16 { __unhandled15 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled15 { exc::raise(__e); }
    if py_truthy(&__ret13) { return __val14; }
    }
    let __unpk17 = py_unpack(&_doubt_ranges(&[_the_opening_sentence(&[reply.clone()])]), 2, None);
    money = __unpk17[0].clone();
    _u = __unpk17[1].clone();
    if py_truthy(&money.clone()) {
    return call_builtin("tuple", &[money.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _the_opening_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut cut: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The first sentence of an answer, which is where a price answer states its price.\n\n    The range a verdict is put against has to be the price of the thing asked about, and an answer\n    holds other ranges further down that are about something else: the car insurance card has no\n    price in its first sentence and \"a new driver pays £1,500 to £2,500 in the first year\" in its\n    third, so reading the whole card made \"is 350 a lot for car insurance\" a quote for a new driver.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str(". "), py_str("! "), py_str("? ")]));
    for __item in __iter1 {
    mark = __item.clone();
    cut = mcall(&text.clone(), "find", &[mark.clone()]);
    if py_truthy(&gt(&cut.clone(), &Py::Int(0))) {
    text = slice_or_raise(&text.clone(), &Py::None, &cut.clone(), &Py::None);
    }
    }
    return text.clone();
    Py::None
}
pub fn _judged_on_its_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut enough: Py = Py::None;
    let mut high: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut options: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The verdict on a figure the person brought with them, or None when nothing can judge it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&doubt.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&doubt.clone(), "get", &[py_str("value")]), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&doubt.clone(), "get", &[py_str("subject")])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&doubt.clone(), "get", &[py_str("unit")])) {
    return Py::None;
    }
    spans = _the_usuals(&[geti(&doubt.clone(), &py_str("subject"))]);
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return Py::None;
    }
    span = geti(&spans.clone(), &Py::Int(0));
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&doubt.clone(), &py_str("value")), geti(&span.clone(), &py_str("low")), geti(&span.clone(), &py_str("high"))]), 3, None);
    value = __unpk1[0].clone();
    low = __unpk1[1].clone();
    high = __unpk1[2].clone();
    enough = Py::Bool(py_eq(&geti(&doubt.clone(), &py_str("word")), &py_str("enough")));
    if py_truthy(&if py_truthy(&le(&low.clone(), &{ let __cmp20 = value.clone(); __cmp20.clone() })) { le(&__cmp20.clone(), &high.clone()) } else { Py::Bool(false) }) {
    options = if py_truthy(&enough.clone()) { g_ENOUGH_FOR_THE_THING() } else { g_RIGHT_FOR_THE_THING() };
    } else {
    if py_truthy(&gt(&value.clone(), &high.clone())) {
    options = if py_truthy(&enough.clone()) { g_ENOUGH_FOR_THE_THING() } else { g_HIGH_FOR_THE_THING() };
    } else {
    options = if py_truthy(&enough.clone()) { g_SHORT_FOR_THE_THING() } else { g_LOW_FOR_THE_THING() };
    }
    }
    options = _the_side_decides(&[options.clone(), doubt.clone(), value.clone(), low.clone(), high.clone(), g_LOW_FOR_THE_THING(), g_THE_LOW_SIDE_FOUND_FOR_THE_THING()]);
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), geti(&doubt.clone(), &py_str("subject"))), ("span".to_string(), geti(&span.clone(), &py_str("text")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), geti(&span.clone(), &py_str("text")))])]))]), geti(&span.clone(), &py_str("text"))]);
    Py::None
}
pub fn _the_thing_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The thing the conversation is on, as \"a chicken\", or \"\" when nothing says.");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    ref_ = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("it"), py_str("that"), py_str("they"), py_str("them")]));
    for __item in __iter1 {
    key = __item.clone();
    thing = mcall(&call_builtin("str", &[{ let __b = mcall(&ref_.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__POINTS_AT_IT(), &mcall(&thing.clone(), "lower", &[]))) } }) {
    return if py_truthy(&mcall(&g_FOR_THE_ARTICLE(), "match", &[thing.clone()])) { thing.clone() } else { add(&py_str("a "), &thing.clone()) };
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_doubt_with_its_thing(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut doubt: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The doubt about the figure, about the thing the chat is on when the line names none.\n\n    \"im cooking a chicken\" then \"is 4 pounds a lot for one\" names no thing of its own, and the\n    thing on the table is the chicken: the price row for a chicken is what the figure is judged\n    against, and without it the reply is a paragraph about shopping around.\n    ");
    doubt = _doubt(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&doubt.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&mcall(&doubt.clone(), "get", &[py_str("value")]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&doubt.clone(), "get", &[py_str("money")]))) } } } }) {
    return doubt.clone();
    }
    subject = mcall(&call_builtin("str", &[{ let __b = mcall(&doubt.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__POINTS_AT_IT(), &mcall(&subject.clone(), "lower", &[]))) } }) {
    return doubt.clone();
    }
    thing = _the_thing_the_chat_is_on(&[memory.clone()]);
    if py_truthy(&thing.clone()) {
    doubt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doubt.clone()); __kws.extend(vec![("subject".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return doubt.clone();
    Py::None
}
pub fn _its_own_subject_judges_it(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _u: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line names both a figure and the thing it is for, and the base has a range.\n\n    Only when the answer before it holds no range of its own: a doubt straight after an answer\n    with a range in it is judged on that answer first (\"how much does a haircut cost\" / \"is 40 a\n    lot for a haircut\" is the answer's £15 to £35, not a second lookup), and this is what answers\n    the line when there is nothing above it to judge against. Only when the knowledge base does not\n    answer the line on its own words: \"is 400 enough for a bike\" reaches the bike price row, and\n    its answer is better than a comparison.\n    ");
    doubt = _the_doubt_with_its_thing(&[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&doubt.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&doubt.clone(), "get", &[py_str("subject")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&doubt.clone(), "get", &[py_str("value")]), &Py::None)) } } } }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&_answer_before(&[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }]), 2, None);
    said = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { _the_answer_is_about(&[said.clone(), geti(&doubt.clone(), &py_str("subject"))]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_card_answers(&[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&_judged_on_its_subject(&[doubt.clone()]), &Py::None));
    Py::None
}
pub fn _a_card_answers(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the knowledge base answers this line on its own words.");
    let __try1 = exc::catch(|| {
    let mut lookup: Py = crate::gen::knowledge::lookup::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&mcall(&lookup.clone(), "find", &[py_dict(vec![(py_str("clean"), text.clone()), (py_str("words"), mcall(&text.clone(), "split", &[]))])]), &Py::None))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn judges_a_figure(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("True when the line is a doubt this module answers, with an answer it can be judged by.\n\n    This is the claim `bot/modules/chat.py` makes as it comes: the doubt about the answer on the\n    table, or a figure the person brought with its own thing (\"is 800 enough for a long weekend in\n    Lisbon\"), which `_its_own_subject_judges_it` answers from that thing's own price row. A figure\n    the memory store judges in its own words is claimed too: that sentence is the reply, and the\n    claim is what keeps the turn from falling through to a module that says the paragraph again.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return { let __b = judges_the_figure_on_the_table(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _its_own_subject_judges_it(&[clean.clone(), memory.clone()]) } };
    Py::None
}
pub fn judges_the_figure_on_the_table(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("True when the line is a doubt about the figure the answer on the table already named.\n\n    The narrower claim, and the one `bot/modules/answers.py` stands down for: it is a line whose\n    answer is the comparison with the range the bot has just given (\"my boiler service is 90\n    pounds\" / \"is that a lot\"), and that module's card would say the whole price range over again\n    instead of answering. A line that names its own thing is not this: \"is 800 enough for a long\n    weekend in Lisbon\" has a trip row of its own, and the row is better than a verdict here.\n\n    It is narrow on purpose. A\n    doubt that only says \"that\" (\"is that enough\") may be about a figure the person gave in their\n    own words, and the memory store reads a figure that came with a unit against the answer's\n    range already (`bot/modules/memory_store.py:_enough_reply`), so a measured figure stays with\n    it. What is left is the price the person gave (\"my boiler service is 90 pounds\", \"is that a\n    lot\"), which nothing else reads - the store's own words for it are \"enough\", \"too much\" and\n    \"about right\", and a price held up and called \"a lot\" fell between the two of them and came\n    back as \"What were you quoted?\", a question about a figure they had just given - and the\n    figure named in the doubt itself (\"is 3500 too much\") or typed between the answer and the\n    doubt (\"how much does a boiler cost\", \"2400\", \"is that a lot\").\n    ");
    return Py::Bool(!py_is(&_the_verdict_for_the_table(&[clean.clone(), memory.clone()]), &Py::None));
    Py::None
}
pub fn _a_rate_of_theirs(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the person's own figure was given as a frequency (\"3 times a week\").");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { py_str("") };
    if py_truthy(&mcall(&g_A_RATE_OF_THEIRS(), "search", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_verdict_for_the_table(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut converted: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut figured: Py = Py::None;
    let mut held: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let _ = py_str("The verdict on the figure the answer on the table named, or None.\n\n    Never the question back, which is a reply but not a verdict: the claim above is this verdict,\n    and a line with no figure in it and no answer to say which is not claimed.\n    ");
    read = _doubt_and_answer(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&read.clone(), 3, None);
    doubt = __unpk1[0].clone();
    said = __unpk1[1].clone();
    at = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES_A_FIGURE(), &mcall(&doubt.clone(), "get", &[py_str("word")]))) } }) {
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(at.clone()); __kws.extend(vec![("narrow".to_string(), _only_says_that(&[doubt.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure_since(args))(&__args) };
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { geti(&held.clone(), &py_str("unit")) } }) {
    figured = _figure_of_the_answer(&[memory.clone(), at.clone()]);
    if py_truthy(&figured.clone()) {
    doubt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doubt.clone()); for (k, v) in kw_pairs(&figured.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None))) {
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(at.clone()); __kws.extend(vec![("narrow".to_string(), _only_says_that(&[doubt.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure_since(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    if py_truthy(&Py::Bool(py_contains(&g_MONEY_DOUBTS(), &geti(&doubt.clone(), &py_str("word"))))) {
    asked = asks_for_the_figure(&[doubt.clone(), said.clone()]);
    if py_truthy(&asked.clone()) {
    return asked.clone();
    }
    }
    return _the_usual_is_the_answer(&[doubt.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = geti(&before.clone(), &py_str("unit")); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TIME_DOUBT(), &geti(&doubt.clone(), &py_str("word")))) } }) {
    span = _the_span(&[doubt.clone(), said.clone()]);
    converted = _in_the_unit(&[geti(&before.clone(), &py_str("value")), geti(&before.clone(), &py_str("unit")), if py_truthy(&span.clone()) { mcall(&span.clone(), "get", &[py_str("unit")]) } else { py_str("") }]);
    if py_truthy(&Py::Bool(!py_is(&converted.clone(), &Py::None))) {
    return _judged(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doubt.clone()); __kws.extend(vec![("value".to_string(), converted.clone()), ("money".to_string(), Py::Bool(false)), ("unit".to_string(), mcall(&span.clone(), "get", &[py_str("unit")])), ("rate".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, said.clone()]);
    }
    }
    if py_truthy(&geti(&before.clone(), &py_str("unit"))) {
    return Py::None;
    }
    doubt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doubt.clone()); for (k, v) in kw_pairs(&before.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    moved = _the_answer_about_this_figure(&[doubt.clone(), memory.clone(), at.clone()]);
    if py_truthy(&Py::Bool(py_is(&moved.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&moved.clone(), 2, None);
    said = __unpk2[0].clone();
    at = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&doubt.clone(), "get", &[py_str("unit")]))); if !py_truthy(&__b) { __b } else { _a_rate_of_theirs(&[memory.clone()]) } }) {
    span = _the_span(&[doubt.clone(), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&span.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _a_stretch_of_time(&[mcall(&span.clone(), "get", &[py_str("unit")])]) } }) {
    return Py::None;
    }
    }
    mine = _judged(&[doubt.clone(), said.clone(), _the_person_words(&[memory.clone(), clean.clone()])]);
    if py_truthy(&Py::Bool(py_is(&mine.clone(), &Py::None))) {
    return Py::None;
    }
    return { let __b = _the_price_verdict(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { mine.clone() } };
    Py::None
}
pub fn _the_price_verdict(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The memory store's judgement of the figure on the table, when it is the better reply.\n\n    `bot/memory/price.py` has read a price held up against the answer before it since before this\n    module existed, and where the figure is outside the range its pools say so and say what to do\n    about it (\"250 is over the £60 to £120 the usual price comes to, so it is worth a second\n    quote\"), in the answer's own words. That is the reply, and the reading stands down for it.\n\n    Only when the figure is *outside* the range: inside it the store's line is \"That is about\n    right\", which does not answer \"is that a lot\", and the yes or no the person asked for is the\n    whole of what is left to say (\"No, that is fine: the usual is £60 to £120\"). Imported here\n    because the store is the memory's and this module is the composer's.\n    ");
    let __try1 = exc::catch(|| {
    let mut a_price: Py = crate::gen::memory::price::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    said = { let __b = mcall(&a_price.clone(), "judged", &[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_store_found_it_outside(&[clean.clone(), memory.clone(), a_price.clone()]))) } }) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn _the_store_found_it_outside(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut a_price: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_price") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a_price"))) };
    let mut _said: Py = Py::None;
    let mut aim: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut mine: Py = Py::None;
    let _ = py_str("Whether the figure the store judged is outside the range it judged it against.\n\n    The store's own reading is asked, so the two never disagree about which range is meant: it is\n    the same answer and the same figure. When anything about it cannot be read, the answer is yes,\n    because the store's sentence is the one main gives and the reading has nothing better to add.\n    ");
    let __try1 = exc::catch(|| {
    let __unpk6 = py_unpack(&mcall(&a_price.clone(), "_the_price_they_mean", &[clean.clone(), memory.clone()]), 2, None);
    mine = __unpk6[0].clone();
    _said = __unpk6[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    answers = mcall(&a_price.clone(), "_answers_in", &[mcall(&memory.clone(), "get", &[py_str("turns")])]);
    aim = mcall(&a_price.clone(), "_the_range_that_is_about_it", &[answers.clone(), mine.clone(), mcall(&a_price.clone(), "_the_period_they_said", &[clean.clone(), memory.clone()])]);
    if py_truthy(&Py::Bool(py_is(&aim.clone(), &Py::None))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_truthy(&if py_truthy(&le(&geti(&aim.clone(), &Py::Int(0)), &{ let __cmp70 = mine.clone(); __cmp70.clone() })) { le(&__cmp70.clone(), &geti(&aim.clone(), &Py::Int(1))) } else { Py::Bool(false) }))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(true);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn reply_to_a_doubt(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("The reply to a line that only asks whether a figure is a lot, enough or about right.");
    verdict = _the_verdict_for_the_table(&[clean.clone(), memory.clone()]);
    if py_truthy(&verdict.clone()) {
    return verdict.clone();
    }
    read = _doubt_and_answer(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&_its_own_subject_judges_it(&[clean.clone(), memory.clone()])))) {
    return Py::None;
    }
    return _judged_on_its_subject(&[_the_doubt_with_its_thing(&[clean.clone(), memory.clone()])]);
    }
    let __unpk1 = py_unpack(&read.clone(), 3, None);
    doubt = __unpk1[0].clone();
    said = __unpk1[1].clone();
    at = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None))) {
    return Py::None;
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(at.clone()); __kws.extend(vec![("narrow".to_string(), _only_says_that(&[doubt.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure_since(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { geti(&before.clone(), &py_str("unit")) } }) {
    return if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) { asks_for_the_figure(&[doubt.clone(), said.clone()]) } else { Py::None };
    }
    return Py::None;
    Py::None
}
pub fn _the_answer_about_this_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doubt: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doubt") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doubt"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut at: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut again: Py = Py::None;
    let mut found_at: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(the answer, its index) the figure is about, or None when no answer can hold it.\n\n    The answer found first is the newest with a range in it, which is usually the right one. A\n    figure in another league from its ranges means the turn in between changed subject, so the\n    answer the figure is about is the one back that could hold it, and the figure has to be found\n    again around that answer.\n    ");
    value = mcall(&doubt.clone(), "get", &[py_str("value")]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_answer_before(&[memory.clone(), value.clone()]), 2, None);
    said = __unpk1[0].clone();
    found_at = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&found_at.clone(), &at.clone()))) {
    return py_tuple(vec![said.clone(), at.clone()]);
    }
    again = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(found_at.clone()); __kws.extend(vec![("narrow".to_string(), _only_says_that(&[doubt.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure_since(args))(&__args) };
    if py_truthy(&again.clone()) {
    let _ = mcall(&doubt.clone(), "update", &[again.clone()]);
    }
    return py_tuple(vec![said.clone(), found_at.clone()]);
    Py::None
}
pub fn _figure_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut mark: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mark") { v.clone() }
    else { py_str("") };
    let mut text: Py = Py::None;
    let _ = py_str("The figure as it is said back: 45 with \"month\" is \"45 a month\", 6 with \"miles\" is \"6 miles\".");
    text = modulo(&py_str("%s%s"), &py_tuple(vec![mark.clone(), number.clone()]));
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return text.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PERIOD(), &unit.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("month"), py_str("week"), py_str("year"), py_str("day")]), &unit.clone())) } }) {
    return modulo(&py_str("%s a %s"), &py_tuple(vec![text.clone(), unit.clone()]));
    }
    return add(&text.clone(), &_unit(&[unit.clone()]));
    Py::None
}
pub fn _their_figure(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Int(1) };
    let mut found: Py = Py::None;
    let mut money: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The last figure the person gave in their own words, as (number, unit, text, money), or None.\n\n    The figure a doubt is about is usually the one they said just before it (\"its 45 a month\" /\n    \"is that a lot\"), and it is theirs: nothing is read out of the bot's own lines here, because\n    the bot's lines are the yardstick and there is no yardstick in this case.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&gt(&seen.clone(), &back.clone())) {
    break;
    }
    found = Py::None;
    let __iter3 = py_iter_or_raise(&mcall(&g_THEIR_FIGURE(), "finditer", &[said.clone()]));
    for __item in __iter3 {
    found = __item.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&words.clone(), &g_A_BILL()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_MONEY_PHRASE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[\\u00a3$\\u20ac]"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PRICE_VERB(), "search", &[mcall(&said.clone(), "lower", &[])]))) } } } } } }) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&mcall(&g_THEIR_NUMBER(), "finditer", &[said.clone()]));
    for __item in __iter4 {
    found = __item.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    }
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("number")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    unit = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = if py_truthy(&le(&g_YEAR(), &{ let __cmp50 = value.clone(); __cmp50.clone() })) { le(&__cmp50.clone(), &g_NEXT_YEAR()) } else { Py::Bool(false) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&found.clone(), "group", &[py_str("mark")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("miles"), py_str("km")]), &unit.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_AN_AGE(), "match", &[mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[])]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:years?|yrs?|months?)\\s+old\\b"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    money = { let __b = call_builtin("bool", &[mcall(&found.clone(), "group", &[py_str("mark")])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONEY_WORDS(), &unit.clone())); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_A_BILL())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_A_MONEY_PHRASE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&unit.clone())); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_PRICE_VERB(), "search", &[mcall(&said.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)) } } } } } } } } } };
    text = _figure_words(&[mcall(&found.clone(), "group", &[py_str("number")]), unit.clone(), { let __b = mcall(&found.clone(), "group", &[py_str("mark")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return py_tuple(vec![value.clone(), unit.clone(), text.clone(), money.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _already_judged(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's own last line already put the figure against a threshold.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    return call_builtin("bool", &[mcall(&g_ALREADY_JUDGED(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_bare_doubt(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _at: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_line: Py = Py::None;
    let _ = py_str("(value, unit, text, money) for a bare doubt with the person's own figure and no range.\n\n    Bare is the whole of it: the doubt says \"that\", and the figure is the one the person said the\n    turn before (\"its 45 a month\" / \"is that a lot\"). A doubt that names its own figure (\"is 120\n    pounds too much to spend\") is not this reader's line - the composer's price check reads it, and\n    it says more than this can.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    doubt = _doubt(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&doubt.clone(), &Py::None))) {
    said_line = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "rstrip", &[py_str("?!. ")]);
    match_ = mcall(&g_BARE_CEILING(), "match", &[said_line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    doubt = py_dict(vec![(py_str("word"), mcall(&match_.clone(), "group", &[py_str("word")])), (py_str("value"), Py::None), (py_str("unit"), py_str("")), (py_str("money"), Py::Bool(false))]);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_A_CEILING(), &geti(&doubt.clone(), &py_str("word"))))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_MONEY_WORD(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_answer_before(&[memory.clone()]), 2, None);
    said = __unpk1[0].clone();
    _at = __unpk1[1].clone();
    if py_truthy(&said.clone()) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&geti(&doubt.clone(), &py_str("value")), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&_already_judged(&[memory.clone()])) {
    return Py::None;
    }
    return _their_figure(&[memory.clone()]);
    Py::None
}
pub fn reply_to_a_bare_doubt(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _value: Py = Py::None;
    let mut line: Py = Py::None;
    let mut money: Py = Py::None;
    let mut options: Py = Py::None;
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The honest reply to a doubt with nothing behind it to judge the figure against.");
    read = _a_bare_doubt(&[clean.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&read.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&read.clone(), 4, None);
    _value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    text = __unpk1[2].clone();
    money = __unpk1[3].clone();
    if py_truthy(&money.clone()) {
    options = g_A_PRICE_NEXT_TO();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_A_DISTANCE(), &unit.clone()))) {
    options = g_A_JOURNEY_IS_TIME();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_A_STRETCH(), &unit.clone()))) {
    options = g_A_STRETCH_OF_TIME();
    } else {
    options = g_THE_REST_IS_UNSURE();
    }
    }
    }
    return py_call(&crate::gen::compose::verdict::g_pick(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("t"), text.clone())])]))]), text.clone()]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut bare: Py = Py::None;
    let mut high: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says_unit: Py = Py::None;
    let mut span: Py = Py::None;
    let mut span_text: Py = Py::None;
    let mut step: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The reading in the line, checked against the range in the answer just given.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_AN_AGE(), "match", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    return Py::None;
    }
    judged = reply_to_a_doubt(&[clean.clone(), memory.clone()]);
    if py_truthy(&judged.clone()) {
    return judged.clone();
    }
    bare = reply_to_a_bare_doubt(&[clean.clone(), memory.clone()]);
    if py_truthy(&bare.clone()) {
    return bare.clone();
    }
    read = _number(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&read.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&read.clone(), 2, None);
    value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    said = _said(&[memory.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    span = _range(&[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&span.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&span.clone(), 3, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    says_unit = __unpk2[2].clone();
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { { let __b = says_unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_same_measure(&[unit.clone(), says_unit.clone()]))) } } } }) {
    return Py::None;
    }
    span_text = modulo(&py_str("%s to %s%s"), &py_tuple(vec![_say(&[low.clone()]), _say(&[high.clone()]), _unit(&[says_unit.clone()])]));
    where_ = _where(&[value.clone(), low.clone(), high.clone(), says_unit.clone()]);
    if py_truthy(&Py::Bool(py_eq(&where_.clone(), &py_str("inside")))) {
    return pick_inside(&[value.clone(), span_text.clone(), said.clone()]);
    }
    step = _step(&[said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&where_.clone(), &py_str("below")))) {
    line = modulo(&py_str("%s is under the %s it should be"), &py_tuple(vec![_say(&[value.clone()]), span_text.clone()]));
    if py_truthy(&step.clone()) {
    return modulo(&py_str("%s: %s."), &py_tuple(vec![line.clone(), mcall(&step.clone(), "rstrip", &[py_str(".")])]));
    }
    return add(&line.clone(), &py_str(" - that is the thing to put right."));
    }
    return modulo(&py_str("%s is over the %s it should be, which is the thing to put right."), &py_tuple(vec![_say(&[value.clone()]), span_text.clone()]));
    Py::None
}
pub fn pick_inside(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut span_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span_text"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut options: Py = Py::None;
    options = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_INSIDE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("value".to_string(), _say(&[value.clone()])), ("span".to_string(), span_text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    return py_call(&crate::gen::compose::verdict::g_pick(), &[options.clone(), py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("bot"), said.clone())])]))]), said.clone()]);
    Py::None
}
pub fn _figure_of_the_answer(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut money: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The figure in the line the answer was given to, when the answer can hold it.\n\n    \"my boiler pressure is 1 bar\" is answered with the card that says what the pressure should be\n    (1 to 1.5 bar), and the next line is \"is that too low\". The figure the doubt is about came in\n    the line the answer answers, before the doubt, so a read that only looks forward asks the\n    person for a figure they have just typed out.\n\n    Only a figure the answer can be held against is read there: a price where the answer states a\n    price range, a reading where it states a range in the same unit. A bare number is not put\n    against a measured range at all - \"my heating bill was 200 a month\" under the boiler card put\n    200 against \"55 to 60C\" and came back \"yes, a bit much\" - so a bare number is only read where\n    the answer's own range is a price, which is the only kind of figure it can be there.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(0), &{ let __cmp10 = at.clone(); __cmp10.clone() })) { lt(&__cmp10.clone(), &call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) })) } }) {
    return Py::None;
    }
    turn = geti(&turns.clone(), &at.clone());
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    found = call_builtin("list", &[mcall(&g_FIGURE(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    figure = _the_figure_in(&[said.clone(), geti(&found.clone(), &neg(&Py::Int(1)))]);
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_doubt_ranges(&[mcall(&turn.clone(), "get", &[py_str("bot")])]), 2, None);
    money = __unpk2[0].clone();
    measured = __unpk2[1].clone();
    if py_truthy(&geti(&figure.clone(), &py_str("unit"))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let mut span: Py = __item.clone(); __out.push(Py::Bool(py_eq(&geti(&span.clone(), &py_str("unit")), &geti(&figure.clone(), &py_str("unit"))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    } else {
    if py_truthy(&geti(&figure.clone(), &py_str("money"))) {
    if py_truthy(&Py::Bool(!py_truthy(&money.clone()))) {
    return Py::None;
    }
    } else {
    if py_truthy(&{ let __b = measured.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&money.clone())) } }) {
    return Py::None;
    }
    }
    }
    return figure.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.reading".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_RIGHT" => Some(g_ABOUT_RIGHT()),
            "ABOVE_THE_RANGE" => Some(g_ABOVE_THE_RANGE()),
            "ABOVE_THE_USUAL" => Some(g_ABOVE_THE_USUAL()),
            "ALREADY_JUDGED" => Some(g_ALREADY_JUDGED()),
            "AN_AGE" => Some(g_AN_AGE()),
            "AN_AGE_AFTER" => Some(g_AN_AGE_AFTER()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_FOR_A_FIGURE" => Some(g_ASKS_FOR_A_FIGURE()),
            "ASKS_FOR_A_PRICE" => Some(g_ASKS_FOR_A_PRICE()),
            "A_BILL" => Some(g_A_BILL()),
            "A_CEILING" => Some(g_A_CEILING()),
            "A_CLOCK_TIME_AFTER" => Some(g_A_CLOCK_TIME_AFTER()),
            "A_CLOCK_TIME_BEFORE" => Some(g_A_CLOCK_TIME_BEFORE()),
            "A_DISTANCE" => Some(g_A_DISTANCE()),
            "A_GAUGE_UNIT" => Some(g_A_GAUGE_UNIT()),
            "A_JOURNEY_IS_TIME" => Some(g_A_JOURNEY_IS_TIME()),
            "A_LONE_PRICE" => Some(g_A_LONE_PRICE()),
            "A_MONEY_PHRASE" => Some(g_A_MONEY_PHRASE()),
            "A_MONEY_WORD" => Some(g_A_MONEY_WORD()),
            "A_PERIOD" => Some(g_A_PERIOD()),
            "A_PRICE_NEXT_TO" => Some(g_A_PRICE_NEXT_TO()),
            "A_PRICE_VERB" => Some(g_A_PRICE_VERB()),
            "A_RANGE_THAT_IS_NOT_A_PRICE" => Some(g_A_RANGE_THAT_IS_NOT_A_PRICE()),
            "A_RATE_OF_THEIRS" => Some(g_A_RATE_OF_THEIRS()),
            "A_STRETCH" => Some(g_A_STRETCH()),
            "A_STRETCH_OF_TIME" => Some(g_A_STRETCH_OF_TIME()),
            "A_WORD_FIGURE" => Some(g_A_WORD_FIGURE()),
            "BARE_CEILING" => Some(g_BARE_CEILING()),
            "BARE_DOUBTS" => Some(g_BARE_DOUBTS()),
            "BELOW_THE_RANGE" => Some(g_BELOW_THE_RANGE()),
            "CARRIED" => Some(g_CARRIED()),
            "DAYS_IN" => Some(g_DAYS_IN()),
            "DOUBT_UNIT" => Some(g_DOUBT_UNIT()),
            "DOUBT_UNITS" => Some(g_DOUBT_UNITS()),
            "DOUBT_WORDS" => Some(g_DOUBT_WORDS()),
            "ENOUGH" => Some(g_ENOUGH()),
            "ENOUGH_FOR_THE_THING" => Some(g_ENOUGH_FOR_THE_THING()),
            "FIGURE" => Some(g_FIGURE()),
            "FOR_THE_ARTICLE" => Some(g_FOR_THE_ARTICLE()),
            "FOR_THE_THING" => Some(g_FOR_THE_THING()),
            "HIGH_FOR_THE_THING" => Some(g_HIGH_FOR_THE_THING()),
            "INSIDE" => Some(g_INSIDE()),
            "LEADING" => Some(g_LEADING()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "LOW_FOR_THE_THING" => Some(g_LOW_FOR_THE_THING()),
            "MEASURED" => Some(g_MEASURED()),
            "MONEY_DOUBTS" => Some(g_MONEY_DOUBTS()),
            "MONEY_RANGE" => Some(g_MONEY_RANGE()),
            "MONEY_WORDS" => Some(g_MONEY_WORDS()),
            "MORE_THAN_ENOUGH" => Some(g_MORE_THAN_ENOUGH()),
            "NEXT_YEAR" => Some(g_NEXT_YEAR()),
            "NOT_TOO_LITTLE" => Some(g_NOT_TOO_LITTLE()),
            "NOT_TOO_LONG" => Some(g_NOT_TOO_LONG()),
            "NOT_TOO_LONG_INSIDE" => Some(g_NOT_TOO_LONG_INSIDE()),
            "NOT_TOO_MUCH" => Some(g_NOT_TOO_MUCH()),
            "NOT_TOO_SHORT" => Some(g_NOT_TOO_SHORT()),
            "NO_PRICE_SAID" => Some(g_NO_PRICE_SAID()),
            "NUMBER" => Some(g_NUMBER()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "ON_THE_HIGH_SIDE" => Some(g_ON_THE_HIGH_SIDE()),
            "ON_THE_LONG_SIDE" => Some(g_ON_THE_LONG_SIDE()),
            "ON_THE_LOW_SIDE" => Some(g_ON_THE_LOW_SIDE()),
            "ON_THE_SHORT_SIDE" => Some(g_ON_THE_SHORT_SIDE()),
            "PER_UNIT" => Some(g_PER_UNIT()),
            "PLACES_A_FIGURE" => Some(g_PLACES_A_FIGURE()),
            "PLURALS" => Some(g_PLURALS()),
            "POINTING_AT_SOMETHING" => Some(g_POINTING_AT_SOMETHING()),
            "POINTING_ONE" => Some(g_POINTING_ONE()),
            "QUANTITY_WORDS" => Some(g_QUANTITY_WORDS()),
            "QUESTION" => Some(g_QUESTION()),
            "RANGE" => Some(g_RANGE()),
            "RIGHT_FOR_THE_THING" => Some(g_RIGHT_FOR_THE_THING()),
            "SAID_HIGH" => Some(g_SAID_HIGH()),
            "SAID_LOW" => Some(g_SAID_LOW()),
            "SAME_LEAGUE" => Some(g_SAME_LEAGUE()),
            "SHORT" => Some(g_SHORT()),
            "SHORT_FOR_THE_THING" => Some(g_SHORT_FOR_THE_THING()),
            "STOP_AFTER" => Some(g_STOP_AFTER()),
            "SUBJECTS" => Some(g_SUBJECTS()),
            "TAG_ASK" => Some(g_TAG_ASK()),
            "TAILS" => Some(g_TAILS()),
            "THEIR_FIGURE" => Some(g_THEIR_FIGURE()),
            "THEIR_NUMBER" => Some(g_THEIR_NUMBER()),
            "THE_LOW_SIDE_FOUND" => Some(g_THE_LOW_SIDE_FOUND()),
            "THE_LOW_SIDE_FOUND_FOR_THE_THING" => Some(g_THE_LOW_SIDE_FOUND_FOR_THE_THING()),
            "THE_PRICE_IT_GAVE" => Some(g_THE_PRICE_IT_GAVE()),
            "THE_RANGE_ITSELF" => Some(g_THE_RANGE_ITSELF()),
            "THE_REST_IS_UNSURE" => Some(g_THE_REST_IS_UNSURE()),
            "THE_USUAL_NO" => Some(g_THE_USUAL_NO()),
            "TIME_DOUBT" => Some(g_TIME_DOUBT()),
            "TOO_LITTLE_WORDS" => Some(g_TOO_LITTLE_WORDS()),
            "TOO_MUCH_WORDS" => Some(g_TOO_MUCH_WORDS()),
            "UNDER_THE_USUAL" => Some(g_UNDER_THE_USUAL()),
            "UNIT" => Some(g_UNIT()),
            "UNIT_ALIAS" => Some(g_UNIT_ALIAS()),
            "UNIT_WORDS" => Some(g_UNIT_WORDS()),
            "WORD_ALIAS" => Some(g_WORD_ALIAS()),
            "WORD_FIGURE_COUNTS" => Some(g_WORD_FIGURE_COUNTS()),
            "WORD_NUMBERS" => Some(g_WORD_NUMBERS()),
            "WORD_RANGE" => Some(g_WORD_RANGE()),
            "YEAR" => Some(g_YEAR()),
            "_ASIDE_ENDS" => Some(g__ASIDE_ENDS()),
            "_A_RATE" => Some(g__A_RATE()),
            "_BUYS" => Some(g__BUYS()),
            "_POINTS_AT_IT" => Some(g__POINTS_AT_IT()),
            "_SAME_JOB" => Some(g__SAME_JOB()),
            "_SAYS_NOTHING_ABOUT_THE_TIER" => Some(g__SAYS_NOTHING_ABOUT_THE_TIER()),
            "_a_bare_doubt" => Some(py_func("_a_bare_doubt", |args: &[Py]| _a_bare_doubt(args))),
            "_a_card_answers" => Some(py_func("_a_card_answers", |args: &[Py]| _a_card_answers(args))),
            "_a_clock_time" => Some(py_func("_a_clock_time", |args: &[Py]| _a_clock_time(args))),
            "_a_lone_price" => Some(py_func("_a_lone_price", |args: &[Py]| _a_lone_price(args))),
            "_a_rate_of_theirs" => Some(py_func("_a_rate_of_theirs", |args: &[Py]| _a_rate_of_theirs(args))),
            "_a_stretch_of_time" => Some(py_func("_a_stretch_of_time", |args: &[Py]| _a_stretch_of_time(args))),
            "_a_word_figure" => Some(py_func("_a_word_figure", |args: &[Py]| _a_word_figure(args))),
            "_add_measured" => Some(py_func("_add_measured", |args: &[Py]| _add_measured(args))),
            "_already_judged" => Some(py_func("_already_judged", |args: &[Py]| _already_judged(args))),
            "_an_age" => Some(py_func("_an_age", |args: &[Py]| _an_age(args))),
            "_answer_before" => Some(py_func("_answer_before", |args: &[Py]| _answer_before(args))),
            "_doubt" => Some(py_func("_doubt", |args: &[Py]| _doubt(args))),
            "_doubt_and_answer" => Some(py_func("_doubt_and_answer", |args: &[Py]| _doubt_and_answer(args))),
            "_doubt_ranges" => Some(py_func("_doubt_ranges", |args: &[Py]| _doubt_ranges(args))),
            "_figure_of_the_answer" => Some(py_func("_figure_of_the_answer", |args: &[Py]| _figure_of_the_answer(args))),
            "_figure_since" => Some(py_func("_figure_since", |args: &[Py]| _figure_since(args))),
            "_figure_words" => Some(py_func("_figure_words", |args: &[Py]| _figure_words(args))),
            "_in_the_same_league" => Some(py_func("_in_the_same_league", |args: &[Py]| _in_the_same_league(args))),
            "_in_the_unit" => Some(py_func("_in_the_unit", |args: &[Py]| _in_the_unit(args))),
            "_its_own_subject_judges_it" => Some(py_func("_its_own_subject_judges_it", |args: &[Py]| _its_own_subject_judges_it(args))),
            "_judged" => Some(py_func("_judged", |args: &[Py]| _judged(args))),
            "_judged_on_its_subject" => Some(py_func("_judged_on_its_subject", |args: &[Py]| _judged_on_its_subject(args))),
            "_money" => Some(py_func("_money", |args: &[Py]| _money(args))),
            "_number" => Some(py_func("_number", |args: &[Py]| _number(args))),
            "_only_says_that" => Some(py_func("_only_says_that", |args: &[Py]| _only_says_that(args))),
            "_plural" => Some(py_func("_plural", |args: &[Py]| _plural(args))),
            "_range" => Some(py_func("_range", |args: &[Py]| _range(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_same_measure" => Some(py_func("_same_measure", |args: &[Py]| _same_measure(args))),
            "_say" => Some(py_func("_say", |args: &[Py]| _say(args))),
            "_step" => Some(py_func("_step", |args: &[Py]| _step(args))),
            "_the_answer_about_this_figure" => Some(py_func("_the_answer_about_this_figure", |args: &[Py]| _the_answer_about_this_figure(args))),
            "_the_answer_is_about" => Some(py_func("_the_answer_is_about", |args: &[Py]| _the_answer_is_about(args))),
            "_the_band_aside" => Some(py_func("_the_band_aside", |args: &[Py]| _the_band_aside(args))),
            "_the_doubt_with_its_thing" => Some(py_func("_the_doubt_with_its_thing", |args: &[Py]| _the_doubt_with_its_thing(args))),
            "_the_figure_in" => Some(py_func("_the_figure_in", |args: &[Py]| _the_figure_in(args))),
            "_the_figure_in_the_doubt" => Some(py_func("_the_figure_in_the_doubt", |args: &[Py]| _the_figure_in_the_doubt(args))),
            "_the_last_figure_in" => Some(py_func("_the_last_figure_in", |args: &[Py]| _the_last_figure_in(args))),
            "_the_opening_sentence" => Some(py_func("_the_opening_sentence", |args: &[Py]| _the_opening_sentence(args))),
            "_the_person_words" => Some(py_func("_the_person_words", |args: &[Py]| _the_person_words(args))),
            "_the_price_verdict" => Some(py_func("_the_price_verdict", |args: &[Py]| _the_price_verdict(args))),
            "_the_range_for" => Some(py_func("_the_range_for", |args: &[Py]| _the_range_for(args))),
            "_the_side_asked" => Some(py_func("_the_side_asked", |args: &[Py]| _the_side_asked(args))),
            "_the_side_decides" => Some(py_func("_the_side_decides", |args: &[Py]| _the_side_decides(args))),
            "_the_span" => Some(py_func("_the_span", |args: &[Py]| _the_span(args))),
            "_the_store_found_it_outside" => Some(py_func("_the_store_found_it_outside", |args: &[Py]| _the_store_found_it_outside(args))),
            "_the_thing_the_chat_is_on" => Some(py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args))),
            "_the_usual_is_the_answer" => Some(py_func("_the_usual_is_the_answer", |args: &[Py]| _the_usual_is_the_answer(args))),
            "_the_usuals" => Some(py_func("_the_usuals", |args: &[Py]| _the_usuals(args))),
            "_the_verdict_for_the_table" => Some(py_func("_the_verdict_for_the_table", |args: &[Py]| _the_verdict_for_the_table(args))),
            "_their_figure" => Some(py_func("_their_figure", |args: &[Py]| _their_figure(args))),
            "_they_named_the_tier" => Some(py_func("_they_named_the_tier", |args: &[Py]| _they_named_the_tier(args))),
            "_unit" => Some(py_func("_unit", |args: &[Py]| _unit(args))),
            "_usual_unit" => Some(py_func("_usual_unit", |args: &[Py]| _usual_unit(args))),
            "_where" => Some(py_func("_where", |args: &[Py]| _where(args))),
            "_with_the_aside" => Some(py_func("_with_the_aside", |args: &[Py]| _with_the_aside(args))),
            "_word_number" => Some(py_func("_word_number", |args: &[Py]| _word_number(args))),
            "asks_for_the_figure" => Some(py_func("asks_for_the_figure", |args: &[Py]| asks_for_the_figure(args))),
            "judges_a_figure" => Some(py_func("judges_a_figure", |args: &[Py]| judges_a_figure(args))),
            "judges_the_figure_on_the_table" => Some(py_func("judges_the_figure_on_the_table", |args: &[Py]| judges_the_figure_on_the_table(args))),
            "pick_inside" => Some(py_func("pick_inside", |args: &[Py]| pick_inside(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "reply_to_a_bare_doubt" => Some(py_func("reply_to_a_bare_doubt", |args: &[Py]| reply_to_a_bare_doubt(args))),
            "reply_to_a_doubt" => Some(py_func("reply_to_a_doubt", |args: &[Py]| reply_to_a_doubt(args))),
            _ => None,
        }),
        set: None,
    }))
}

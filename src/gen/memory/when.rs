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


pub static AGO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGO_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALREADY_DONE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ASK_IN_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_INTERVAL_AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKED_ABOUT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_THE_MONTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_THROWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COUNT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DATE_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DATE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LENGTH_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TRIP_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_UNIT_OF_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YEAR_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_TURNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_CALENDAR_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_UNIT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEGAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BOOKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_PUT_RIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOES_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOING_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_LONG_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_OFTEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INTERVAL_JOINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOINED_TO_A_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAST_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTH_AFTER_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTH_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTH_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTH_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DATE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DATE_TO_GIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_LENGTH_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PLURAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_IN_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_DATE_YET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_PREP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_VERB_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_IN_THE_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAME_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_IT_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_IT_IS_COMING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPELLED_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_MEAN_NOTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRAILING_JOINER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP_BOOKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP_BOOKED_ONLY_WITH_A_BOOKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNTIL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNTIL_A_LEARNING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNTIL_COUNTDOWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNTIL_NAMES_THE_EVENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNTIL_THROWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANT_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAS_GOING_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKDAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKDAY_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKDAY_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKDAY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_DAY_SAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DAY_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MOVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PLAN_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_AT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AGO_ASK() -> Py { ensure_init(); AGO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = AGO_ASK.write() { *w = v; } }
pub fn g_AGO_LENGTH() -> Py { ensure_init(); AGO_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGO_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = AGO_LENGTH.write() { *w = v; } }
pub fn g_ALREADY_DONE_ASK() -> Py { ensure_init(); ALREADY_DONE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALREADY_DONE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ALREADY_DONE_ASK.write() { *w = v; } }
pub fn g_AN_ASK_IN_THE_PAST() -> Py { ensure_init(); AN_ASK_IN_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ASK_IN_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = AN_ASK_IN_THE_PAST.write() { *w = v; } }
pub fn g_AN_INTERVAL_AHEAD() -> Py { ensure_init(); AN_INTERVAL_AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_INTERVAL_AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = AN_INTERVAL_AHEAD.write() { *w = v; } }
pub fn g_ASKED_ABOUT_THE_THING() -> Py { ensure_init(); ASKED_ABOUT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKED_ABOUT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = ASKED_ABOUT_THE_THING.write() { *w = v; } }
pub fn g_ASKS_THE_DAY() -> Py { ensure_init(); ASKS_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_DAY.write() { *w = v; } }
pub fn g_ASKS_THE_MONTH() -> Py { ensure_init(); ASKS_THE_MONTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_MONTH(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_THE_MONTH.write() { *w = v; } }
pub fn g_ASKS_WHEN() -> Py { ensure_init(); ASKS_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHEN.write() { *w = v; } }
pub fn g_ASKS_WHERE() -> Py { ensure_init(); ASKS_WHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHERE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHERE.write() { *w = v; } }
pub fn g_ASK_OPENER() -> Py { ensure_init(); ASK_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = ASK_OPENER.write() { *w = v; } }
pub fn g_ASK_THROWS() -> Py { ensure_init(); ASK_THROWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_THROWS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_THROWS.write() { *w = v; } }
pub fn g_A_COUNT_TAIL() -> Py { ensure_init(); A_COUNT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_COUNT_TAIL.write() { *w = v; } }
pub fn g_A_DATE_ITSELF() -> Py { ensure_init(); A_DATE_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DATE_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = A_DATE_ITSELF.write() { *w = v; } }
pub fn g_A_DATE_WORD() -> Py { ensure_init(); A_DATE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DATE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_DATE_WORD.write() { *w = v; } }
pub fn g_A_DAY_FORM() -> Py { ensure_init(); A_DAY_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_FORM(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_FORM.write() { *w = v; } }
pub fn g_A_GOING() -> Py { ensure_init(); A_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GOING(v: Py) { ensure_init(); if let Ok(mut w) = A_GOING.write() { *w = v; } }
pub fn g_A_LENGTH_IN_THE_LINE() -> Py { ensure_init(); A_LENGTH_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_LENGTH_IN_THE_LINE.write() { *w = v; } }
pub fn g_A_THING_NAMED() -> Py { ensure_init(); A_THING_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_NAMED.write() { *w = v; } }
pub fn g_A_TRIP_SAID() -> Py { ensure_init(); A_TRIP_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TRIP_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_TRIP_SAID.write() { *w = v; } }
pub fn g_A_UNIT_OF_TIME() -> Py { ensure_init(); A_UNIT_OF_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_UNIT_OF_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_UNIT_OF_TIME.write() { *w = v; } }
pub fn g_A_VERB() -> Py { ensure_init(); A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_VERB.write() { *w = v; } }
pub fn g_A_YEAR_SAID() -> Py { ensure_init(); A_YEAR_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YEAR_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_YEAR_SAID.write() { *w = v; } }
pub fn g_BACK_TURNS() -> Py { ensure_init(); BACK_TURNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_TURNS(v: Py) { ensure_init(); if let Ok(mut w) = BACK_TURNS.write() { *w = v; } }
pub fn g_BARE_CALENDAR_ASK() -> Py { ensure_init(); BARE_CALENDAR_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_CALENDAR_ASK(v: Py) { ensure_init(); if let Ok(mut w) = BARE_CALENDAR_ASK.write() { *w = v; } }
pub fn g_BARE_DAY() -> Py { ensure_init(); BARE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = BARE_DAY.write() { *w = v; } }
pub fn g_BARE_UNIT_ASK() -> Py { ensure_init(); BARE_UNIT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_UNIT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = BARE_UNIT_ASK.write() { *w = v; } }
pub fn g_BEGAN() -> Py { ensure_init(); BEGAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEGAN(v: Py) { ensure_init(); if let Ok(mut w) = BEGAN.write() { *w = v; } }
pub fn g_BOOKING() -> Py { ensure_init(); BOOKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BOOKING(v: Py) { ensure_init(); if let Ok(mut w) = BOOKING.write() { *w = v; } }
pub fn g_CALENDAR_NOW() -> Py { ensure_init(); CALENDAR_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_NOW(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_NOW.write() { *w = v; } }
pub fn g_DAY_NUMBER() -> Py { ensure_init(); DAY_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = DAY_NUMBER.write() { *w = v; } }
pub fn g_DAY_PUT_RIGHT() -> Py { ensure_init(); DAY_PUT_RIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_PUT_RIGHT(v: Py) { ensure_init(); if let Ok(mut w) = DAY_PUT_RIGHT.write() { *w = v; } }
pub fn g_EVENT_NAME() -> Py { ensure_init(); EVENT_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_NAME(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_NAME.write() { *w = v; } }
pub fn g_EVENT_WORD() -> Py { ensure_init(); EVENT_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_WORD(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_WORD.write() { *w = v; } }
pub fn g_FILLER() -> Py { ensure_init(); FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FILLER.write() { *w = v; } }
pub fn g_FOR_LENGTH() -> Py { ensure_init(); FOR_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = FOR_LENGTH.write() { *w = v; } }
pub fn g_GOES_TO() -> Py { ensure_init(); GOES_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOES_TO(v: Py) { ensure_init(); if let Ok(mut w) = GOES_TO.write() { *w = v; } }
pub fn g_GOING_ON() -> Py { ensure_init(); GOING_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOING_ON(v: Py) { ensure_init(); if let Ok(mut w) = GOING_ON.write() { *w = v; } }
pub fn g_HOW_LONG_ASK() -> Py { ensure_init(); HOW_LONG_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_ASK(v: Py) { ensure_init(); if let Ok(mut w) = HOW_LONG_ASK.write() { *w = v; } }
pub fn g_HOW_OFTEN() -> Py { ensure_init(); HOW_OFTEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_OFTEN(v: Py) { ensure_init(); if let Ok(mut w) = HOW_OFTEN.write() { *w = v; } }
pub fn g_INTERVAL_JOINERS() -> Py { ensure_init(); INTERVAL_JOINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INTERVAL_JOINERS(v: Py) { ensure_init(); if let Ok(mut w) = INTERVAL_JOINERS.write() { *w = v; } }
pub fn g_JOINED_TO_A_DATE() -> Py { ensure_init(); JOINED_TO_A_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOINED_TO_A_DATE(v: Py) { ensure_init(); if let Ok(mut w) = JOINED_TO_A_DATE.write() { *w = v; } }
pub fn g_LAST_LENGTH() -> Py { ensure_init(); LAST_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAST_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = LAST_LENGTH.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_MONTH_AFTER_THE_DAY() -> Py { ensure_init(); MONTH_AFTER_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTH_AFTER_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = MONTH_AFTER_THE_DAY.write() { *w = v; } }
pub fn g_MONTH_NAME() -> Py { ensure_init(); MONTH_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTH_NAME(v: Py) { ensure_init(); if let Ok(mut w) = MONTH_NAME.write() { *w = v; } }
pub fn g_MONTH_NAMES() -> Py { ensure_init(); MONTH_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTH_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = MONTH_NAMES.write() { *w = v; } }
pub fn g_MONTH_NUMBER() -> Py { ensure_init(); MONTH_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTH_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = MONTH_NUMBER.write() { *w = v; } }
pub fn g_NOT_A_DATE_ASK() -> Py { ensure_init(); NOT_A_DATE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DATE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DATE_ASK.write() { *w = v; } }
pub fn g_NOT_A_DATE_TO_GIVE() -> Py { ensure_init(); NOT_A_DATE_TO_GIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DATE_TO_GIVE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DATE_TO_GIVE.write() { *w = v; } }
pub fn g_NOT_A_LENGTH_ASK() -> Py { ensure_init(); NOT_A_LENGTH_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LENGTH_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_LENGTH_ASK.write() { *w = v; } }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); NOT_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PLACE.write() { *w = v; } }
pub fn g_NOT_A_PLURAL() -> Py { ensure_init(); NOT_A_PLURAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLURAL(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PLURAL.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_IN_THE_PAST() -> Py { ensure_init(); NOT_IN_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_IN_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = NOT_IN_THE_PAST.write() { *w = v; } }
pub fn g_NO_DATE_YET() -> Py { ensure_init(); NO_DATE_YET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_DATE_YET(v: Py) { ensure_init(); if let Ok(mut w) = NO_DATE_YET.write() { *w = v; } }
pub fn g_NO_PLACE() -> Py { ensure_init(); NO_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = NO_PLACE.write() { *w = v; } }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); NUMBER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER_WORDS.write() { *w = v; } }
pub fn g_PAST_WHEN() -> Py { ensure_init(); PAST_WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_WHEN(v: Py) { ensure_init(); if let Ok(mut w) = PAST_WHEN.write() { *w = v; } }
pub fn g_PLACE_PREP() -> Py { ensure_init(); PLACE_PREP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_PREP(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_PREP.write() { *w = v; } }
pub fn g_PLAN_VERB_ASK() -> Py { ensure_init(); PLAN_VERB_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_VERB_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_VERB_ASK.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_SAID_IN_THE_PAST() -> Py { ensure_init(); SAID_IN_THE_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_IN_THE_PAST(v: Py) { ensure_init(); if let Ok(mut w) = SAID_IN_THE_PAST.write() { *w = v; } }
pub fn g_SAME_THING() -> Py { ensure_init(); SAME_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAME_THING(v: Py) { ensure_init(); if let Ok(mut w) = SAME_THING.write() { *w = v; } }
pub fn g_SAYS_IT_AGAIN() -> Py { ensure_init(); SAYS_IT_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_IT_AGAIN.write() { *w = v; } }
pub fn g_SAYS_IT_IS_COMING() -> Py { ensure_init(); SAYS_IT_IS_COMING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_IS_COMING(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_IT_IS_COMING.write() { *w = v; } }
pub fn g_SHORT_DATE() -> Py { ensure_init(); SHORT_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_DATE(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_DATE.write() { *w = v; } }
pub fn g_SPELLED_COUNT() -> Py { ensure_init(); SPELLED_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPELLED_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = SPELLED_COUNT.write() { *w = v; } }
pub fn g_THEIR_OWN_DEATH() -> Py { ensure_init(); THEIR_OWN_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_DEATH.write() { *w = v; } }
pub fn g_THE_MEAN_NOTE() -> Py { ensure_init(); THE_MEAN_NOTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_MEAN_NOTE(v: Py) { ensure_init(); if let Ok(mut w) = THE_MEAN_NOTE.write() { *w = v; } }
pub fn g_TRAILING_JOINER() -> Py { ensure_init(); TRAILING_JOINER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRAILING_JOINER(v: Py) { ensure_init(); if let Ok(mut w) = TRAILING_JOINER.write() { *w = v; } }
pub fn g_TRIP_BOOKED() -> Py { ensure_init(); TRIP_BOOKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP_BOOKED(v: Py) { ensure_init(); if let Ok(mut w) = TRIP_BOOKED.write() { *w = v; } }
pub fn g_TRIP_BOOKED_ONLY_WITH_A_BOOKING() -> Py { ensure_init(); TRIP_BOOKED_ONLY_WITH_A_BOOKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP_BOOKED_ONLY_WITH_A_BOOKING(v: Py) { ensure_init(); if let Ok(mut w) = TRIP_BOOKED_ONLY_WITH_A_BOOKING.write() { *w = v; } }
pub fn g_TRIP_LINE() -> Py { ensure_init(); TRIP_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP_LINE(v: Py) { ensure_init(); if let Ok(mut w) = TRIP_LINE.write() { *w = v; } }
pub fn g_TRIP_WORDS() -> Py { ensure_init(); TRIP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TRIP_WORDS.write() { *w = v; } }
pub fn g_UNTIL_ASK() -> Py { ensure_init(); UNTIL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNTIL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = UNTIL_ASK.write() { *w = v; } }
pub fn g_UNTIL_A_LEARNING() -> Py { ensure_init(); UNTIL_A_LEARNING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNTIL_A_LEARNING(v: Py) { ensure_init(); if let Ok(mut w) = UNTIL_A_LEARNING.write() { *w = v; } }
pub fn g_UNTIL_COUNTDOWN() -> Py { ensure_init(); UNTIL_COUNTDOWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNTIL_COUNTDOWN(v: Py) { ensure_init(); if let Ok(mut w) = UNTIL_COUNTDOWN.write() { *w = v; } }
pub fn g_UNTIL_NAMES_THE_EVENT() -> Py { ensure_init(); UNTIL_NAMES_THE_EVENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNTIL_NAMES_THE_EVENT(v: Py) { ensure_init(); if let Ok(mut w) = UNTIL_NAMES_THE_EVENT.write() { *w = v; } }
pub fn g_UNTIL_THROWS() -> Py { ensure_init(); UNTIL_THROWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNTIL_THROWS(v: Py) { ensure_init(); if let Ok(mut w) = UNTIL_THROWS.write() { *w = v; } }
pub fn g_WANT_LINE() -> Py { ensure_init(); WANT_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANT_LINE(v: Py) { ensure_init(); if let Ok(mut w) = WANT_LINE.write() { *w = v; } }
pub fn g_WAS_GOING_ON() -> Py { ensure_init(); WAS_GOING_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAS_GOING_ON(v: Py) { ensure_init(); if let Ok(mut w) = WAS_GOING_ON.write() { *w = v; } }
pub fn g_WEEKDAYS() -> Py { ensure_init(); WEEKDAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKDAYS(v: Py) { ensure_init(); if let Ok(mut w) = WEEKDAYS.write() { *w = v; } }
pub fn g_WEEKDAY_NAME() -> Py { ensure_init(); WEEKDAY_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_NAME(v: Py) { ensure_init(); if let Ok(mut w) = WEEKDAY_NAME.write() { *w = v; } }
pub fn g_WEEKDAY_NUMBER() -> Py { ensure_init(); WEEKDAY_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = WEEKDAY_NUMBER.write() { *w = v; } }
pub fn g_WEEKDAY_SAID() -> Py { ensure_init(); WEEKDAY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = WEEKDAY_SAID.write() { *w = v; } }
pub fn g_WHAT_DAY_SAYS() -> Py { ensure_init(); WHAT_DAY_SAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_DAY_SAYS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_DAY_SAYS.write() { *w = v; } }
pub fn g_WHEN_ASK() -> Py { ensure_init(); WHEN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_ASK.write() { *w = v; } }
pub fn g_WHEN_SAID() -> Py { ensure_init(); WHEN_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_SAID(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_SAID.write() { *w = v; } }
pub fn g_WORD() -> Py { ensure_init(); WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WORD.write() { *w = v; } }
pub fn g__DAY_NAMED() -> Py { ensure_init(); _DAY_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DAY_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = _DAY_NAMED.write() { *w = v; } }
pub fn g__MOVED() -> Py { ensure_init(); _MOVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MOVED(v: Py) { ensure_init(); if let Ok(mut w) = _MOVED.write() { *w = v; } }
pub fn g__PLAN_VERB() -> Py { ensure_init(); _PLAN_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PLAN_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _PLAN_VERB.write() { *w = v; } }
pub fn g__POINTS_AT_IT() -> Py { ensure_init(); _POINTS_AT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_IT(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_AT_IT.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A \"when ...\" follow-up, answered from what the person already told us.\n\nPeople ask about their own dates with the thing itself and no time in the ask: \"when am I\nmoving?\", \"when's my exam?\", \"when is it?\". The line that stated it is in the conversation, and\nit carries the date, so the answer is the date alone rather than the whole plan again.\n\n    i'm moving house in march            / when am i moving    -> In March.\n    my exam is in june                   / when's my exam      -> In June.\n    i have a dentist appointment on tuesday / when is it       -> On Tuesday.\n\nThe helper answers only when the ask and an earlier line are about the same thing: a word in the\nask has to be in that line (in any of its forms, so \"moving\" finds \"move\"), and the line has to\ncarry a date. When a line carries two dates - \"i'm going to japan in march and i've got a\ndentist appointment on tuesday\" - the date that belongs to the thing asked is the one just after\nit. Everything else is left to the modules that know about it.\n");
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and |so |but |oh |right |also |ok(?:ay)?[ ,]+)*(?:when(?:'s|s| is| are| was| am| do| does| did| will| would)\\b|what (?:day|date)\\b|which day\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_CALENDAR_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|oh|right|also|ok|okay)\\b[\\s,]*)?what(?:'s|s| is| was)?\\s+(?:the\\s+)?(?:day|date)(?:\\s+(?:is\\s+it|it\\s+is|is\\s+that|it|today|now|please|then|already|again))*\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_SAID(stdlib_call("re", "compile", &[py_str("\\bwhen did (?:i|we|you)\\b[^.?!]*\\b(?:say|said|tell|told|mention|mentioned|ask|asked|bring|brought|talk|talks|talked|discuss|discussed|cover|covered)\\b|\\bwhen (?:was|were) it (?:i|we) (?:said|told)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR_NOW(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|ok|okay|right|oh|also|hi|hey|quick|sorry|excuse me)\\W+)*what(?:'s|s| is| are| was)?\\s+(?:the\\s+)?(?:day|date)(?:[\\s,]*(?:is it|it is|today|now|tonight|tomorrow|this (?:week|morning|evening)|right now))*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THROWS(call_builtin("frozenset", &[mcall(&py_str("exactly again then just roughly about long before after tonight due\nstart starts starting end ends ending say said tell told know"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_DAY_SAYS(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("date")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_NAMES(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_SHORT_DATE(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\b(?:for|in|on|by|until|till|around|from)\\s+(?:the\\s+)?(?:(?:early|mid|late)\\s+)?(?:"), &g_MONTH_NAMES()), &py_str("|")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_DAYS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str("|(?:0?[1-9]|[12]\\d|3[01])(?:st|nd|rd|th)?(?!\\s*(?:k|grand|thousand|hundred|million|billion|quid|pounds?|dollars?|euros?|cents?|percent|%|per\\b|a\\s+(?:week|month|year|day)))(?:\\s+(?:of\\s+)?(?:")), &g_MONTH_NAMES()), &py_str("))?)\\b|\\b(?:is|was|it's|its)\\s+(?:in\\s+|on\\s+)?(?:")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DAY(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:start|starts|starting|begin|begins|beginning|finish|finishes|finishing|end|ends|ending|leave|leaves|leaving|go|goes|going|come|comes|coming|return|returns|returning|fly|flies|flying|arrive|arrives|arriving|land|lands|landing|move|moves|moving|open|opens|opening|close|closes|closing|visit|visits|visiting|meet|meets|meeting|submit|submits|hand|hands|deliver|delivers|book|booked|due|held|happens|happen|is|are|was|were|have|has|got)\\s+(?:on\\s+|this\\s+|next\\s+)?(?:(?!and\\b|but\\b|because\\b|if\\b|[.!?])[\\w']+\\s+){0,3}("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_DAYS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b(?!\\s+(?:is|was|are|were)\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_WHEN(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:last|back in|this past)\\s+(?:(?:early|mid|late)\\s+)?(?:"), &g_MONTH_NAMES()), &py_str("|spring|summer|autumn|winter|year)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP_WORDS(call_builtin("frozenset", &[mcall(&py_str("holiday holidays trip break vacation getaway going go flying fly travel\ntravelling traveling leaving leave away"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:go|going|goes|went|fly|flying|flies|flew|travel\\w*|leave|leaving|left|trip|holiday|break|vacation|getaway|visiting|visit|abroad)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP_BOOKED(stdlib_call("re", "compile", &[py_str("\\b(?:tickets?|flights?|ferry|plane|abroad|check(?:ed)?\\s*-?\\s?in|boarding)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BOOKING(stdlib_call("re", "compile", &[py_str("\\b(?:book(?:ed|ing)|reserv\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP_BOOKED_ONLY_WITH_A_BOOKING(stdlib_call("re", "compile", &[py_str("\\b(?:trains?|coach|coaches|seat|seats|hotel|room|rooms|table)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("it that this them there then him her those these"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_TURNS(Py::Int(3));
    s_EVENT_NAME(py_dict(vec![(py_str("wedding"), py_tuple(vec![py_str("married"), py_str("marry"), py_str("wedding")])), (py_str("birthday"), py_tuple(vec![py_str("birthday"), py_str("born"), py_str("turns")])), (py_str("operation"), py_tuple(vec![py_str("operation"), py_str("surgery")]))]));
    s_NOT_A_DATE_TO_GIVE(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|dead|death|killed|passed\\s+away|passed\\s+on|funeral|burial|buried|cremation|cremated|put\\s+to\\s+sleep|euthanis\\w*|widow|widowed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:kill\\s+myself|killing\\s+myself|kill\\s+me(?!\\s+a)|suicide|suicidal|end\\s+my\\s+(?:own\\s+)?life|take\\s+my\\s+(?:own\\s+)?life|hang\\s+myself|hang\\s+myself|overdose|cut\\s+myself|done\\s+with\\s+living|better\\s+off\\s+dead|(?:(?:do(?:es)?|did)\\s+not|don'?t|dont|didn'?t)\\s+want\\s+to\\s+(?:be\\s+here|live|carry\\s+on|go\\s+on)(?=\\s*(?:$|[.,!?;:]|and\\b|any\\s?more|anymore|at\\s+all|like\\s+this|like\\s+that|for\\s+good|for\\s+much\\s+longer|much\\s+longer|any\\s+longer|tonight|today|this\\s+(?:week|month|year|time)|(?:in|by|at|before|until|till|after)\\s+(?:\\d{4}|jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)))|want\\s+to\\s+die|going\\s+to\\s+die|(?:do(?:es)?\\s+not|did\\s+not|don'?t|dont|didn'?t)\\s+want\\s+to\\s+(?:live|be\\s+here)\\s+(?:forever|for\\s+ever|in\\s+this\\s+world|on\\s+this\\s+earth))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_DATE_YET(py_tuple(vec![py_str("You haven't told me when — when is it?"), py_str("No date from you on that one. When is it?"), py_str("I don't have a date from you for that. When is it?")]));
    s_NOT_A_DATE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|ought|best|better|good|safe|easiest|cheapest|likely|worth|advice|tips?|look\\w*|search\\w*|appl\\w*|think\\w*|tr(?:y|ying)|plan\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EVENT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|pregnan\\w*|birth|scan|job|work|wedding|marri\\w*|wed|move|moving|moved|flat|house|trip|holiday|honeymoon|exam|exams|test|appointment|interview|operation|surgery|course|term|flight|ferry|train|placement|retire\\w*|start\\w*|begin\\w*|due|deadline|due\\s+date|results?|course)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANT_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:want|wanna|wanted|would\\s+like|hope|hoping|trying|try|thinking\\s+of|dream\\w*|plan\\s+to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_VERB_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|ok|okay|right)\\W+)*when(?:'s|s)?\\s+(?:do|does|did|will|would|is|are|was|were|am)\\s+(?:i|we|it|that|this|they|he|she)\\s+(?:start|starts|begin|begins|finish|end|ends|go|going|leave|leaving|due|happen\\w*|arrive\\w*|back|home|on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_OPENER(stdlib_call("re", "compile", &[py_str("^\\W*(?:do|does|did|can|could|should|would|will|shall|is|are|was|were|have|has|had|am|how|what|when|where|who|which|why|any|anyone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nabout actually also because been before day days going coming sorry wrong now still yet again\nthink thought said say told tell really surely maybe right okay ok well thanks thank got get\nhave has had was were is are be been being turn turns\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_MEAN_NOTE(stdlib_call("re", "compile", &[py_str("\\s*\\(\\s*(?:i\\s+|sorry,?\\s+i\\s+)?mean\\s+([^)]*)\\)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINED_TO_A_DATE(stdlib_call("re", "compile", &[py_str("(?i)\\s*(?:(?:for|is|was|it's|its|'s|in|on|at|by|next|this|last)\\s+)*")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD(py_tuple(vec![py_str("for"), py_str("is"), py_str("was"), py_str("it's"), py_str("its"), py_str("'s")]));
    s_MONTH_AFTER_THE_DAY(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s+(?:of\\s+)?(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|your|his|her|their|the|this|that|these|those|a|an)\\s+([a-z][a-z'-]{1,19})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAME_THING(py_dict(vec![(py_str("wedding"), call_builtin("frozenset", &[mcall(&py_str("married marry marrying wed weddings"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("married"), call_builtin("frozenset", &[mcall(&py_str("wedding weddings marry marrying wed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("marry"), call_builtin("frozenset", &[mcall(&py_str("wedding weddings married marrying"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("baby"), call_builtin("frozenset", &[mcall(&py_str("pregnant pregnancy birth born due babies"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("babies"), call_builtin("frozenset", &[mcall(&py_str("pregnant pregnancy birth born due baby"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("pregnant"), call_builtin("frozenset", &[mcall(&py_str("baby babies pregnancy birth due"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("pregnancy"), call_builtin("frozenset", &[mcall(&py_str("baby babies pregnant birth due"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("job"), call_builtin("frozenset", &[mcall(&py_str("work working career"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("work"), call_builtin("frozenset", &[mcall(&py_str("job career"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))]));
    s_NOT_A_PLURAL(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is"), py_str("as"), py_str("ws")]));
    s_ASKS_WHERE(stdlib_call("re", "compile", &[py_str("\\bwhere(?:abouts)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHEN(stdlib_call("re", "compile", &[py_str("\\bwhen\\b|\\b(?:which|what) (?:month|day|week|date|time of year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_THE_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:which|what) month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_THE_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:which|what) (?:day|week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_PLACE(call_builtin("frozenset", &[mcall(&py_str("away out off back home there somewhere anywhere everywhere now soon\nlater today tomorrow tonight weekend week month year again then bed sleep work"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERB(call_builtin("frozenset", &[mcall(&py_str("move moving go going went fly flying drive driving travel travelling\ntraveling visit visiting start starting begin beginning take taking run running play playing work\nworking book booking sort sorting pack packing have having get getting see seeing meet meeting\nbuy buying sell selling pay paying do doing make making"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOES_TO(stdlib_call("re", "compile", &[py_str("\\b(?:go|goes|going|went|gone|off|head|headed|heading|fly|flying|flies|flew|flown|move|moves|moving|moved|travel|travelling|traveling|drive|driving|drove|back|booked|book|trip|tour|visit|visiting|holiday|vacation|head(?:ing|ed)?|away)\\b[^.;!?]*?\\b(?:to|towards)\\s+([^,.;!?]+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_PREP(stdlib_call("re", "compile", &[py_str("\\b(?:to|towards|via)\\s+(\\S.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAILING_JOINER(stdlib_call("re", "compile", &[py_str("(?:[\\s,]+\\b(?:in|on|at|for|by|from|to|and|around|during|early|mid|late|the|of)\\b)+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_IT_IS_COMING(stdlib_call("re", "compile", &[py_str("\\b(?:booked|reserved|arranged|scheduled|organi[sz]ed|planned)\\b|\\b(?:got|bought|picked\\s+up)\\b[^.!?]{0,20}?\\b(?:tickets?|flights?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_ON(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|they|he|she)(?:'?m|'?re|'?s)?\\s+[a-z]+ing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAS_GOING_ON(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|had|been)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_DONE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:did|have|has|do)\\s+(?:i|we|that|it)\\b[^.?!]*?\\b(?:already|yet|happen(?:ed)?|been|go|gone|going|did)\\b|\\b(?:already|yet)\\s+(?:go|gone|been|happen(?:ed)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_IT_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:(?:where|what|when|why|who|how\\s+often|how\\s+many\\s+times)\\s+)?did\\s+(?:i|we)\\s+(?:ever\\s+)?(?:say|tell\\s+you|mention)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_OFTEN(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:often|many\\s+times)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:did|was|were|had|have\\s+i|has\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_INTERVAL_AHEAD(stdlib_call("re", "compile", &[py_str("\\bin\\s+(?P<n>\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|couple\\s+of|a\\s+couple\\s+of|a\\s+few|few|several)\\s+(?P<unit>days?|nights?|weeks?|fortnight|fortnights?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INTERVAL_JOINERS(call_builtin("frozenset", &[mcall(&py_str("is are was were will be gets get got has have had and so but\n  only just now then again roughly about around nearly maybe probably ideally hopefully"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNTIL_A_LEARNING(stdlib_call("re", "compile", &[py_str("\\b(?:till|until|til)\\s+i\\s+(?:know|hear|find\\s+out|learn|s\\s*(?:out|back)|get\\s+(?:the\\s+)?(?:results?|answer|news))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNTIL_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:long|soon|far|many\\s+(?:days?|weeks?|months?|nights?))\\b[^.?!]*?(?:\\b(?:till|til|until|to\\s+go|left|away|off|have\\s+i\\s+got|do\\s+(?:i|we)\\s+have|does\\s+that\\s+leave|ago|since)\\b|\\b(?:is|was|are)\\s+(?:that|it|this)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGO_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:ago|since|back\\s+was|has\\s+it\\s+been)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:had|was|were|went|did|been|got|took|made|bought|brought|sold|lost|met|left|saw|said|told|felt|broke|broke\\s+up|came|died|kept|paid|ran|sat|stood|wrote|spoke|\\w+ed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_IN_THE_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:will|going\\s+to|next|upcoming|tomorrow|have|has|is|are|am)\\b|'(?:re|m)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNTIL_COUNTDOWN(stdlib_call("re", "compile", &[py_str("\\b(till|til|until|to\\s+go|have\\s+i\\s+got|do\\s+(?:i|we)\\s+have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNTIL_NAMES_THE_EVENT(call_builtin("frozenset", &[py_tuple(vec![py_str("till"), py_str("til"), py_str("until")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_UNIT_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|right|then|now|well|yeah)\\W+)*how\\s+many\\s+(?:days?|weeks?|months?|nights?)\\b(?:\\W+(?:have\\s+i\\s+got|is\\s+that|to\\s+go|left|away|off|now|then|again))*\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a|an|a\\s+couple|couple|few|several)\\s+(?:days?|nights?|weeks?|months?|hours?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:have\\s+i\\s+got|is\\s+that|to\\s+go|left|away|off|till|til|until)\\b|\\bhow\\s+(?:long|soon|far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRIP_SAID(call_builtin("frozenset", &[mcall(&py_str("trip vacation honeymoon"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOING(call_builtin("frozenset", &[mcall(&py_str("going off flying travelling traveling heading driving"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PLACE(call_builtin("frozenset", &[mcall(&py_str("be get go do try have see make take start stop need leave work bed school\n    bed sleep the a an my our your their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNTIL_THROWS(call_builtin("frozenset", &[mcall(&py_str("how long soon far many much day days week weeks month months night\nnights till til until to go going left away off have got do does did i we you it that this then\nthem there is was are be time left ago since"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_NUMBER(py_dict(vec![(py_str("january"), Py::Int(1)), (py_str("february"), Py::Int(2)), (py_str("march"), Py::Int(3)), (py_str("april"), Py::Int(4)), (py_str("may"), Py::Int(5)), (py_str("june"), Py::Int(6)), (py_str("july"), Py::Int(7)), (py_str("august"), Py::Int(8)), (py_str("september"), Py::Int(9)), (py_str("october"), Py::Int(10)), (py_str("november"), Py::Int(11)), (py_str("december"), Py::Int(12))]));
    s_MONTH_NAME({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MONTH_NUMBER(), "items", &[])) { let __unpk6449 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk6449[0].clone(); let mut number: Py = __unpk6449[1].clone(); __out.push((number.clone(), mcall(&name.clone(), "capitalize", &[]))); } py_dict(__out) })() });
    s_WEEKDAY_NUMBER(py_dict(vec![(py_str("monday"), Py::Int(0)), (py_str("tuesday"), Py::Int(1)), (py_str("wednesday"), Py::Int(2)), (py_str("thursday"), Py::Int(3)), (py_str("friday"), Py::Int(4)), (py_str("saturday"), Py::Int(5)), (py_str("sunday"), Py::Int(6))]));
    s_WEEKDAY_NAME({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WEEKDAY_NUMBER(), "items", &[])) { let __unpk113 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk113[0].clone(); let mut number: Py = __unpk113[1].clone(); __out.push((number.clone(), mcall(&name.clone(), "capitalize", &[]))); } py_dict(__out) })() });
    s_A_DATE_WORD(bitor(&bitor(&call_builtin("frozenset", &[g_MONTH_NUMBER()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_WEEKDAY_NUMBER()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("early"), py_str("mid"), py_str("late"), py_str("end"), py_str("ends"), py_str("ending"), py_str("next"), py_str("this"), py_str("month"), py_str("week"), py_str("the"), py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("start"), py_str("beginning"), py_str("weekend"), py_str("a"), py_str("an"), py_str("for"), py_str("from"), py_str("to"), py_str("until"), py_str("till"), py_str("around"), py_str("about")])]).unwrap_or_else(|e| exc::raise(e))));
    s_DAY_NUMBER(stdlib_call("re", "compile", &[py_str("^\\d{1,2}(?:st|nd|rd|th)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_ITSELF(bitor(&bitor(&call_builtin("frozenset", &[g_MONTH_NUMBER()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_WEEKDAY_NUMBER()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("early"), py_str("mid"), py_str("late"), py_str("end"), py_str("ends"), py_str("ending"), py_str("next"), py_str("this"), py_str("month"), py_str("week")])]).unwrap_or_else(|e| exc::raise(e))));
    s_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("a"), py_str("an"), py_str("for"), py_str("from"), py_str("to"), py_str("until"), py_str("till"), py_str("around"), py_str("about")])]).unwrap_or_else(|e| exc::raise(e)));
    s__DAY_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:mon|tues|wednes|thurs|fri|satur|sun)day|tomorrow|tonight|the weekend|next week|next month)(?:\\s+(?:night|morning|afternoon|evening|lunchtime|midday))?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MOVED(stdlib_call("re", "compile", &[py_str("\\b(?:instead|move it|push it|put it off|leave it (?:till|until|to)|another day|a day later)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PLAN_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:make|cook|do|have|serve|eat|start|finish|go|come|meet|post|send|call|book|clean|get|buy|pick|deliver|visit)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_AT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_LONG_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:have|has|had|did)\\b|\\bhow\\s+(?:many|much)\\s+(?:months?|weeks?|years?|days?)\\s+(?:has|have|had)\\s+it\\s+been\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:have|has|had)\\s+(?:i|we|you|it|they)\\s+(?:got|get|to\\s+wait|to\\s+go|left|before)\\b|\\bhow\\s+long\\s+(?:until|till|before|after)\\b|\\bhow\\s+long\\s+(?:do|does|will|would|should|can|could|must)\\b|\\bhow\\s+long\\s+(?:ago|is|was)\\b|\\bhow\\s+long\\s+(?:have|has|had)\\s+(?:i|we|you)\\s+(?:got|got\\s+to|to|need)\\b|\\bhow\\s+long\\s+(?:have|has|had)\\s+(?:you|it|that)\\s+been\\s+(?:like\\s+that|this|with\\s+you|since|now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGO_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?P<n>\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|a\\s+couple\\s+of|a\\s+few|couple\\s+of|few)\\s+(?P<unit>days?|weeks?|fortnights?|months?|years?)\\s+ago\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LAST_LENGTH(stdlib_call("re", "compile", &[py_str("\\blast\\s+(?P<unit>week|fortnight|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_LENGTH(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?P<n>\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|a\\s+couple\\s+of|a\\s+few|couple\\s+of)\\s+(?P<unit>days?|weeks?|fortnights?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKDAY_SAID(stdlib_call("re", "compile", &[py_str("\\bon\\s+(?P<day>monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKDAYS(py_list(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")]));
    s_A_YEAR_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:19|20)\\d\\d\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BEGAN(stdlib_call("re", "compile", &[py_str("\\b(?:started|start|began|begin|kicked\\s+off|took\\s+up|moved|stopped|quit|gave\\s+up|joined|arrived|switched|changed|since|ago|for\\s+(?:\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:days?|weeks?|months?|years?)|got\\s+(?:married|engaged|together|a\\s+dog|a\\s+cat|a\\s+puppy|a\\s+kitten|the\\s+keys)|married|adopted|bought|got|last\\s+(?:week|fortnight|month|year))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_ABOUT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("it this that them there here doing done at it going on\nbeen am is was long how lived living worked working gone many much month months week weeks year\nyears day days time now since already has have had did is are were was being like get got"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORDS(py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20))]));
    s_A_UNIT_OF_TIME(py_dict(vec![(py_str("day"), Py::Int(1)), (py_str("week"), Py::Int(7)), (py_str("fortnight"), Py::Int(14)), (py_str("month"), Py::Int(30)), (py_str("year"), Py::Int(365))]));
    s_DAY_PUT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:no,?\\s+|actually,?\\s+|sorry,?\\s+)?(?:it\\s+was|it's|it is|i\\s+meant|i\\s+mean|make\\s+that)\\b[^.,!?]*\\b(?P<day>january|february|march|april|may|june|july|august|september|october|november|december|(?:19|20)\\d\\d|last\\s+(?:week|month|year)|\\d+\\s+(?:days?|weeks?|months?|years?)\\s+ago)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|last\\s+(?:week|month|year))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPELLED_COUNT(py_dict(vec![(Py::Int(1), py_str("a")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
}
pub fn _is_a_trip_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when this line is about a journey, whether said by its verb or by what was booked.");
    if py_truthy(&{ let __b = mcall(&g_TRIP_LINE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TRIP_BOOKED(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_TRIP_BOOKED_ONLY_WITH_A_BOOKING(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_BOOKING(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_that_event(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut says: Py = Py::None;
    let _ = py_str("True when the line states, with `other`, the event the ask named with `word`.");
    let __iter1 = py_iter_or_raise(&mcall(&g_EVENT_NAME(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    says = __unpk2[1].clone();
    if py_truthy(&{ let __b = _same(&[word.clone(), name.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&says.clone()) { let mut one: Py = __item.clone(); __out.push(_same(&[one.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_what_today_is(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("\"What day is it\", \"what's the date\", \"what date is it today\": the calendar now.\n\n    A question about today, not about anything the chat is on. The store reads an ask that names\n    a fact and leaves the value out as one about the thing being talked about, which is right for\n    \"and my job?\" and wrong here: in a chat about a new job the dress code for a first day\n    answered \"what day is it\", and in a chat about a bad week a sympathy line did. The date is\n    the answer, and the calendar is read where the calendar is read.\n    ");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "rstrip", &[py_str("?!. ")]);
    return call_builtin("bool", &[mcall(&g_CALENDAR_NOW(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _more_than_one_day(args: &[Py]) -> Py {
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
    let mut dated: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spans: Py = Py::None;
    let _ = py_str("True when the chat holds two different days, and neither of them is a day put right.\n\n    A correction is them telling us which day to use (\"i start monday\" / \"actually i meant\n    tuesday\"), so the two days are one answer and the newest is right. Two days about two things\n    are a choice the line has to make, and a line with no words of its own cannot make it.\n    ");
    dated = _dated(&[memory.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let __unpk541 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk541[0].clone(); let mut spans: Py = __unpk541[1].clone(); __out.push(crate::gen::memory::facts::a_date_put_right(&[said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let __unpk8188 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk8188[0].clone(); let mut spans: Py = __unpk8188[1].clone(); if py_truthy(&spans.clone()) {  __out.push(geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0))); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn when_answer(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut anchors: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut line: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut said_date: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut trip: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The date of the thing the line asks about, or None when no earlier line says it.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_CALENDAR_NOW(), "match", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_WHEN_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_SAID(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_BARE_CALENDAR_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&line.clone(), "split", &[])])])) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_WHAT_DAY_SAYS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { _points_back(&[line.clone()]) } }) {
    dated = _dated(&[memory.clone()]);
    if py_truthy(&{ let __b = dated.clone(); if !py_truthy(&__b) { __b } else { { let __b = crate::gen::memory::facts::a_date_put_right(&[geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { _said_in_the_last_few_turns(&[memory.clone(), geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(0))]) } } } }) {
    joined = if py_truthy(&gt(&call_builtin("len", &[dated.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { _a_day_and_a_month(&[geti(&geti(&geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(1)), &neg(&Py::Int(1))), &Py::Int(0)), geti(&geti(&geti(&geti(&dated.clone(), &Py::Int(1)), &Py::Int(1)), &neg(&Py::Int(1))), &Py::Int(0))]) } else { py_str("") };
    if py_truthy(&joined.clone()) {
    return _sentence(&[joined.clone()]);
    }
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&dated.clone(), &Py::None, &Py::Int(1), &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    spans = __unpk2[1].clone();
    if py_truthy(&_said_in_the_last_few_turns(&[memory.clone(), said.clone()])) {
    return _sentence(&[geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0))]);
    }
    }
    return Py::None;
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    trip = Py::Bool(!py_eq(&asked.clone(), &words.clone()));
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    said_before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_dated(&[memory.clone()])) { let __unpk7372 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk7372[0].clone(); let mut spans: Py = __unpk7372[1].clone(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[said.clone()])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() };
    anchors = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said_before.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&anchors.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![anchors.clone(), Py::Bool(false)]), 2, None);
    asked = __unpk3[0].clone();
    trip = __unpk3[1].clone();
    } else {
    if py_truthy(&_more_than_one_day(&[memory.clone()])) {
    return Py::None;
    }
    }
    }
    put_right = py_str("");
    let __iter4 = py_iter_or_raise(&_dated(&[memory.clone()]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    said = __unpk5[0].clone();
    spans = __unpk5[1].clone();
    if py_truthy(&mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()])) {
    continue;
    }
    if py_truthy(&crate::gen::memory::facts::a_date_put_right(&[said.clone()])) {
    put_right = { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0)) } };
    continue;
    }
    if py_truthy(&{ let __b = trip.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_a_trip_line(&[said.clone()]))) } }) {
    continue;
    }
    when = _time_for(&[asked.clone(), said.clone(), spans.clone()]);
    if py_truthy(&when.clone()) {
    said_date = { let __b = _a_day_and_a_month(&[put_right.clone(), when.clone()]); if py_truthy(&__b) { __b } else { { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { when.clone() } } } };
    if py_truthy(&{ let __b = _has_happened(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AN_ASK_IN_THE_PAST(), "search", &[line.clone()]))) } }) {
    return _was_sentence(&[said_date.clone()]);
    }
    return _sentence(&[said_date.clone()]);
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_thing(&[said.clone(), spans.clone()]))) } }) {
    above = _the_thing_before_the_date(&[memory.clone(), said.clone(), asked.clone()]);
    if py_truthy(&above.clone()) {
    return _sentence(&[{ let __b = _a_day_and_a_month(&[put_right.clone(), geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0))]); if py_truthy(&__b) { __b } else { { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0)) } } } }]);
    }
    }
    put_right = py_str("");
    }
    return _no_date_yet(&[asked.clone(), line.clone(), memory.clone()]);
    Py::None
}
pub fn _is_an_ask(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is a question, not a statement of what they are doing.");
    text = mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = mcall(&text.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_ASK_OPENER(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _backs_off(args: &[Py]) -> Py {
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
    let mut tokens: Py = Py::None;
    let _ = py_str("True when the ask ends on a word that points at something already said.\n\n    The same test as `_points_back` with the words that ask where in the line the answer is\n    (\"due\", \"exactly\", \"again\") taken off first: \"when is it due\" points back through the \"due\".\n    ");
    tokens = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = tokens.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_THROWS(), &geti(&tokens.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&tokens.clone(), "pop", &[]);
    }
    return { let __b = call_builtin("bool", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &geti(&tokens.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _no_date_yet(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut asked: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut latest: Py = Py::None;
    let mut other: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply for a thing they have told us about but never dated, or None.\n\n    Only when the newest line the ask is about carries no date at all: a line with a date in it is\n    the other readers' business, and saying \"you haven't told me when\" over a date they did give\n    would be worse than the hedge.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DATE_ASK(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])); } } py_list(__out) })() };
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    latest = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")]);
    if py_truthy(&call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    recent = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut said: Py = __item.clone(); if py_truthy(&said.clone()) {  __out.push(said.clone()); } } py_list(__out) })() }, &neg(&g_BACK_TURNS()), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&add(&recent.clone(), &call_builtin("list", &[call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    said = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&_is_an_ask(&[said.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("list", &[mcall(&crate::gen::memory::facts::g_FUTURE(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&asked.clone()) {
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&theirs.clone()) { let mut other: Py = __item.clone(); __out.push(_same(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _backs_off(&[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PLAN_VERB_ASK(), "search", &[line.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_EVENT_WORD(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_WANT_LINE(), "search", &[said.clone()]) } }) {
    continue;
    }
    }
    return geti(&g_NO_DATE_YET(), &modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_NO_DATE_YET()]).unwrap_or_else(|e| exc::raise(e))));
    }
    return Py::None;
    Py::None
}
pub fn _a_day_and_a_month(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut a_month: Py = Py::None;
    let mut the_day: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The two halves of one date joined: \"on the 4th\" put right under \"in june\".\n\n    A day put right says the day and nothing else, so the month has to come from the line it\n    corrects: \"i'm going to spain in june\" / \"sorry, it's the 4th\" / \"when do we fly\" is\n    \"On the 4th of june.\", not \"On the 4th.\" (no month) or \"In June.\" (the month they took\n    back). Returns \"\" unless one half is a day and the other a month.\n    ");
    the_day = stdlib_call("re", "search", &[py_str("\\b\\d{1,2}(?:st|nd|rd|th)\\b"), call_builtin("str", &[{ let __b = day.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    a_month = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z]+"), call_builtin("str", &[{ let __b = month.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &mcall(&word.clone(), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = the_day.clone(); if !py_truthy(&__b) { __b } else { a_month.clone() } }))) {
    return py_str("");
    }
    return modulo(&py_str("on the %s of %s"), &py_tuple(vec![mcall(&the_day.clone(), "group", &[Py::Int(0)]), mcall(&a_month.clone(), "lower", &[])]));
    Py::None
}
pub fn _points_back(args: &[Py]) -> Py {
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
    let mut tokens: Py = Py::None;
    let _ = py_str("True when the ask ends on a word that points at something already said.\n\n    \"when is the best time to prep for it\" is about the thing \"it\" names; \"when is the best\n    time to take a mortgage\" ends on a thing of its own and is a question about the world.\n    ");
    tokens = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &geti(&tokens.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn the_plan(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("The newest plan the store keeps, in the person's own words, or None.\n\n    The store rewrites this line when the plan is corrected, so \"i'm going to lisbon in may\"\n    followed by \"sorry, june\" leaves *this* as \"Lisbon in June\". Anything that reads a time out\n    of the conversation should read it from here before it reads the older turns, or a value the\n    person has already taken back goes on being used.\n    ");
    known = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } } } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    plan = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "strip", &[]) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&plan.clone(), "split", &[])]);
    }
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&p.clone(), "strip", &[]) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    return if py_truthy(&plans.clone()) { mcall(&py_str(" "), "join", &[mcall(&geti(&plans.clone(), &neg(&Py::Int(1))), "split", &[])]) } else { Py::None };
    Py::None
}
pub fn _said_in_the_last_few_turns(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut key: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when that line is still one of the last few things the person typed.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])); } } py_list(__out) })() };
    key = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return Py::Bool(py_contains(&slice_or_raise(&turns.clone(), &neg(&g_BACK_TURNS()), &Py::None, &Py::None), &key.clone()));
    Py::None
}
pub fn _names_a_thing(args: &[Py]) -> Py {
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
    let mut spans: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spans"))) };
    let mut _start: Py = Py::None;
    let mut left: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names something other than the date it carries.\n\n    \"my test is on the 3rd of may\" names a test; \"its going to be on friday\" names nothing but\n    the day, so the date in it belongs to whatever the line before it is about. The date itself\n    is taken out before the words are counted, and the little words that only carry a date along\n    (\"going\", \"sorry\", \"wrong\") are not things.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let _start = _start.clone(); let left = left.clone(); let said = said.clone(); let spans = spans.clone(); let text = text.clone(); let when = when.clone(); let word = word.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); neg(&geti(&span.clone(), &Py::Int(1))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    when = __unpk2[0].clone();
    _start = __unpk2[1].clone();
    said = mcall(&said.clone(), "replace", &[when.clone(), py_str(" "), Py::Int(1)]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("bool", &[left.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_before_the_date(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut line: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier line the ask is about, when the date was said with no subject of its own.\n\n    \"weve got my in laws coming for dinner\" / \"its going to be on friday\" / \"when is the\n    dinner\" is Friday: the second line says the date with an \"it\" and names nothing, so read on\n    its own the date has nothing to belong to and the ask was answered \"you haven't told me\n    when\". The thing is the nearest line above that the ask's own words match, within the last\n    few turns, so a date from long ago is still a different thing.\n\n    A date said with a subject of its own is not bridged: \"my test is on the 3rd of may\" is the\n    test's date, and an ask about the boiler is not answered with it.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])); } } py_list(__out) })() };
    key = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&turns.clone(), &key.clone()))) {
    cut = sub(&sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &mcall(&slice_or_raise(&turns.clone(), &Py::None, &Py::None, &neg(&Py::Int(1))), "index", &[key.clone()]));
    } else {
    cut = call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &call_builtin("max", &[Py::Int(0), sub(&cut.clone(), &mul(&g_BACK_TURNS(), &Py::Int(2)))]).unwrap_or_else(|e| exc::raise(e)), &cut.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&line.clone(), &key.clone())) } }) {
    continue;
    }
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&crate::gen::memory::facts::g_FUTURE(), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(mcall(&found.clone(), "group", &[Py::Int(0)])); } py_list(__out) })() };
    let __aug2 = add(&spans.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SHORT_DATE(), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(mcall(&found.clone(), "group", &[Py::Int(0)])); } py_list(__out) })() });
    spans = __aug2;
    if py_truthy(&crate::gen::memory::facts::a_date_put_right(&[line.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_a_thing(&[line.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); __out.push(py_tuple(vec![span.clone(), Py::Int(0)])); } py_list(__out) })() }])))) {
    continue;
    }
    if py_truthy(&spans.clone()) {
    break;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(_same(&[word.clone(), found.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_line_put_right(args: &[Py]) -> Py {
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
    let mut joined: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut meant: Py = Py::None;
    let mut note: Py = Py::None;
    let mut old: Py = Py::None;
    let mut record: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The topic's line with the value they meant in place of the date they took back.\n\n    The store writes the correction onto the topic as a note after the line (\"... on saturday\n    (mean Sunday)\") and files the corrected plan beside it. The note is what says which date is\n    in force once the turn of the correction has gone by: without it the newest line they typed\n    still carries the old date, and a later ask about the day is answered with the date they had\n    already put right. Returns the line they meant and the date it replaced, or (\"\", \"\").\n    ");
    record = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]) } else { Py::None };
    text = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]) } else { py_str("") };
    note = mcall(&g_THE_MEAN_NOTE(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&note.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    meant = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&note.clone(), "group", &[Py::Int(1)]), "split", &[])]), "strip", &[py_str(" .,")]);
    line = mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&note.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&meant.clone())) } }) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::memory::facts::a_moved_time(&[meant.clone()])))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    spans = add(&call_builtin("list", &[mcall(&crate::gen::memory::facts::g_FUTURE(), "finditer", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let joined = joined.clone(); let last = last.clone(); let line = line.clone(); let meant = meant.clone(); let memory = memory.clone(); let note = note.clone(); let old = old.clone(); let record = record.clone(); let spans = spans.clone(); let swapped = swapped.clone(); let text = text.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); py_tuple(vec![mcall(&found.clone(), "start", &[]), call_builtin("len", &[mcall(&found.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    joined = mcall(&mcall(&g_JOINED_TO_A_DATE(), "match", &[mcall(&last.clone(), "group", &[Py::Int(0)])]), "group", &[Py::Int(0)]);
    swapped = add(&add(&add(&slice_or_raise(&line.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &joined.clone()), &meant.clone()), &slice_or_raise(&line.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
    old = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(mcall(&last.clone(), "group", &[Py::Int(0)])); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JOINED_TO_A_DATE(), "sub", args))(&__args) };
    return py_tuple(vec![mcall(&py_str(" "), "join", &[mcall(&swapped.clone(), "split", &[])]), _plain(&[old.clone()])]);
    Py::None
}
pub fn _dated(args: &[Py]) -> Py {
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
    let mut fixed_sentence: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut latest: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut out: Py = Py::None;
    let mut p: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut replaced: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("Every earlier line that carries a date, with its dates: newest first.\n\n    The plans are the dated lines the store keeps as plans, and the turns are every line the\n    person typed, so \"the wedding is in september\" answers a \"when\" ask even though the store\n    keeps no plan for it.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    out = __unpk1[0].clone();
    seen = __unpk1[1].clone();
    known = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } } } else { py_dict(vec![]) };
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    latest = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&plans.clone(), &latest.clone())) } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    lines = typed.clone();
    let __unpk2 = py_unpack(&_put_right(&[memory.clone()]), 2, None);
    fixed_sentence = __unpk2[0].clone();
    replaced = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&fixed_sentence.clone()))) {
    let __unpk3 = py_unpack(&_the_line_put_right(&[memory.clone()]), 2, None);
    fixed_sentence = __unpk3[0].clone();
    replaced = __unpk3[1].clone();
    }
    if py_truthy(&fixed_sentence.clone()) {
    lines = add(&py_list(vec![fixed_sentence.clone()]), &lines.clone());
    }
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&mcall(&py_str(" "), "join", &[mcall(&line.clone(), "split", &[])]), "lower", &[])); } py_set(__out) })() };
    let __iter4 = py_iter_or_raise(&add(&lines.clone(), &call_builtin("list", &[call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter4 {
    said = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    key = mcall(&said.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&seen.clone(), &key.clone())) } }) {
    continue;
    }
    let __unpk5 = py_unpack(&py_tuple(vec![py_list(vec![]), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    spans = __unpk5[0].clone();
    places = __unpk5[1].clone();
    let __iter6 = py_iter_or_raise(&mcall(&crate::gen::memory::facts::g_FUTURE(), "finditer", &[said.clone()]));
    for __item in __iter6 {
    found = __item.clone();
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_lead(&[_with_the_month_after(&[said.clone(), found.clone()])]), mcall(&found.clone(), "start", &[])])]);
    let _ = mcall(&places.clone(), "add", &[mcall(&found.clone(), "start", &[])]);
    }
    let __iter7 = py_iter_or_raise(&mcall(&g_SHORT_DATE(), "finditer", &[said.clone()]));
    for __item in __iter7 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&places.clone(), &mcall(&found.clone(), "start", &[])))) {
    continue;
    }
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_lead(&[mcall(&found.clone(), "group", &[Py::Int(0)])]), mcall(&found.clone(), "start", &[])])]);
    let _ = mcall(&places.clone(), "add", &[mcall(&found.clone(), "start", &[])]);
    }
    let __iter8 = py_iter_or_raise(&mcall(&g_PAST_WHEN(), "finditer", &[said.clone()]));
    for __item in __iter8 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&places.clone(), &mcall(&found.clone(), "start", &[])))) {
    continue;
    }
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_lead(&[mcall(&found.clone(), "group", &[Py::Int(0)])]), mcall(&found.clone(), "start", &[])])]);
    let _ = mcall(&places.clone(), "add", &[mcall(&found.clone(), "start", &[])]);
    }
    let __iter9 = py_iter_or_raise(&mcall(&g_BARE_DAY(), "finditer", &[said.clone()]));
    for __item in __iter9 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&places.clone(), &mcall(&found.clone(), "start", &[Py::Int(1)])))) {
    continue;
    }
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_lead(&[add(&py_str("on "), &mcall(&found.clone(), "group", &[Py::Int(1)]))]), mcall(&found.clone(), "start", &[Py::Int(1)])])]);
    let _ = mcall(&places.clone(), "add", &[mcall(&found.clone(), "start", &[Py::Int(1)])]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&spans.clone())); if !py_truthy(&__b) { __b } else { crate::gen::memory::facts::a_date_put_right(&[said.clone()]) } }) {
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_a_date_said(&[crate::gen::memory::facts::the_date_put_right(&[said.clone()])]), Py::Int(0)])]);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let fixed_sentence = fixed_sentence.clone(); let found = found.clone(); let key = key.clone(); let known = known.clone(); let latest = latest.clone(); let line = line.clone(); let lines = lines.clone(); let memory = memory.clone(); let out = out.clone(); let p = p.clone(); let places = places.clone(); let plans = plans.clone(); let replaced = replaced.clone(); let said = said.clone(); let seen = seen.clone(); let span = span.clone(); let spans = spans.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let typed = typed.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); geti(&span.clone(), &Py::Int(1)) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "sort", args))(&__args) };
    if py_truthy(&{ let __b = replaced.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &key.clone())) } }) {
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&mcall(&_plain(&[geti(&span.clone(), &Py::Int(0))]), "split", &[]), &replaced.clone()))) {  __out.push(span.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    continue;
    }
    let _ = mcall(&seen.clone(), "add", &[key.clone()]);
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![said.clone(), spans.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _lead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A date given with the word that joins it to the thing it belongs to.\n\n    \"the wedding *is* september\" is September, and \"i booked a trip *for* april\" is April: the\n    joining word says what the date is for, not that it is a deadline. The date is said back with\n    the preposition a date takes (\"in july\", \"on tuesday\") so that a copula is never the answer:\n    \"my dentist appointment is tuesday\" / \"when is it\" is \"On Tuesday.\", not \"Is Tuesday.\"\n    ");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" ,;.")]);
    words = mcall(&text.clone(), "split", &[]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEAD(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    rest = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    if py_truthy(&mcall(&mcall(&rest.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("in "), py_str("on "), py_str("at "), py_str("by "), py_str("next "), py_str("this "), py_str("last ")])])) {
    return rest.clone();
    }
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("[\\s,]"), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late")]), &first.clone())) } }) {
    return add(&py_str("in "), &rest.clone());
    }
    return add(&py_str("on "), &rest.clone());
    }
    return text.clone();
    Py::None
}
pub fn _time_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut asked: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut spans: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spans"))) };
    let mut after: Py = Py::None;
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut score: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut start: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The date in that line that belongs to the thing asked about.");
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0));
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    hits = __unpk1[0].clone();
    seen = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_WORD(), "finditer", &[said.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    let __iter3 = py_iter_or_raise(&asked.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&seen.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = _same(&[word.clone(), mcall(&found.clone(), "group", &[Py::Int(0)])]); if py_truthy(&__b) { __b } else { _names_that_event(&[word.clone(), mcall(&found.clone(), "group", &[Py::Int(0)])]) } } } }) {
    let _ = mcall(&seen.clone(), "add", &[word.clone()]);
    let _ = mcall(&hits.clone(), "append", &[mcall(&found.clone(), "start", &[])]);
    }
    }
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = lt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e)), &sub(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } } } }) {
    return Py::None;
    }
    best = Py::None;
    let __iter4 = py_iter_or_raise(&spans.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    when = __unpk5[0].clone();
    start = __unpk5[1].clone();
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hits.clone()) { let mut hit: Py = __item.clone(); if py_truthy(&ge(&start.clone(), &hit.clone())) {  __out.push(sub(&start.clone(), &hit.clone())); } } py_list(__out) })() };
    score = if py_truthy(&after.clone()) { py_tuple(vec![call_builtin("min", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0)]) } else { py_tuple(vec![call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hits.clone()) { let mut hit: Py = __item.clone(); __out.push(call_builtin("abs", &[sub(&start.clone(), &hit.clone())]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { lt(&score.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![score.clone(), when.clone()]);
    }
    }
    return geti(&best.clone(), &Py::Int(1));
    Py::None
}
pub fn _with_the_month_after(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The date text, plus the month said right after a bare day number.");
    text = mcall(&found.clone(), "group", &[Py::Int(0)]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d{1,2}(?:st|nd|rd|th)$"), mcall(&text.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))))) {
    return text.clone();
    }
    after = mcall(&g_MONTH_AFTER_THE_DAY(), "match", &[{ let __b = slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return text.clone();
    }
    return add(&text.clone(), &mcall(&after.clone(), "group", &[Py::Int(0)]));
    Py::None
}
pub fn _names_a_thing_of_its_own(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut behind: Py = Py::None;
    let mut body: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut index: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a countdown ask names a thing the dated line is not about.\n\n    \"i have an exam in june\" / \"how long until my food arrives\" is a question about the food - the\n    date in the chat is not what is being counted to, and the answer said \"about 7 months away -\n    sometime in June\" to somebody waiting on a delivery. Same for \"how long until the boiler is\n    fixed\" after a scan date. The thing is a noun behind a determiner (\"my food\", \"the boiler\"),\n    and it is this line's own subject unless the dated line says that word too, which is what keeps\n    \"how long have i got to revise\" (a bare verb, no thing) and \"how long until my exam\" (the exam\n    is the dated line) answered as before.\n\n    A thing needs no determiner to be its own: \"how many weeks till christmas\" and \"how many days\n    till payday\" were counted to a holiday in June, 204 days to a payday that was never in the\n    chat. \"Till\" and \"until\" are followed by the event itself, so a countdown that uses one and\n    whose words the dated line does not hold is about that event. The other countdown leads are\n    not: \"have i got to revise\" and \"to go\" say what the time is for, and they keep the date.\n    ");
    found = mcall(&g_UNTIL_COUNTDOWN(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    body = slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&mcall(&g__POINTS_AT_IT(), "search", &[body.clone()])) {
    return Py::Bool(false);
    }
    dated = call_builtin("set", &[mcall(&g_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_UNTIL_NAMES_THE_EVENT(), &mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    words = mcall(&g_WORD(), "findall", &[mcall(&body.clone(), "lower", &[])]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_UNTIL_THROWS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_THROWS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&dated.clone(), &word.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let mut held: Py = __item.clone(); __out.push(_same(&[word.clone(), held.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    behind = if py_truthy(&index.clone()) { geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_DETERMINERS(), &behind.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_doing_word(&[slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)]))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    let __iter3 = py_iter_or_raise(&mcall(&g_A_THING_NAMED(), "finditer", &[body.clone()]));
    for __item in __iter3 {
    match_ = __item.clone();
    thing = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&dated.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let mut word: Py = __item.clone(); __out.push(_same(&[thing.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_UNTIL_THROWS(), &thing.clone()))) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_doing_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether any of the words says what is done, rather than naming the thing.\n\n    \"until the boiler is fixed\" and \"until my food arrives\" say what will happen to the thing;\n    \"till the wedding\" only names it. Read from the shape of the word, so no list of verbs has to\n    be kept: a participle or a third-person form ends the thing named and starts the event.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been")]), &word.clone()))) {  __out.push({ let __b = mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ed"), py_str("ing")])]); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } } } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _plain(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The date with the word that joins it stripped: \"in july\" -> \"july\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEAD(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _put_right(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut old: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The date a correction settled on, and the old one it replaced.\n\n    The store keeps the line the person used to put it right (`say`/`from`/`sentence`), and the\n    date they were put right on is the date in it. Returns (\"\", \"\") when nothing was corrected\n    or the corrected line carries no date, so every other chat is unchanged.\n    ");
    fixed = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("correction")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    sentence = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("sentence")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    old = _plain(&[mcall(&fixed.clone(), "get", &[py_str("from")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&old.clone())) } }) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    found = add(&call_builtin("list", &[mcall(&crate::gen::memory::facts::g_FUTURE(), "finditer", &[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    return py_tuple(vec![sentence.clone(), old.clone()]);
    Py::None
}
pub fn _a_date_said(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A bare date said back the way a date is said: \"june\" -> \"in June\", \"the 4th\" -> \"on the\n    4th\", \"4th may\" -> \"on the 4th of may\".\n\n    The date is stored the way it was typed, and the answer has to be a date somebody would say,\n    so the joining word is put back on it. A month takes \"in\" and a day takes \"on\", which is the\n    same rule `_lead` follows for a date that already had its joining word.\n    ");
    phrase = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!?")]);
    words = mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "fullmatch", &[py_str("\\d{1,2}(?:st|nd|rd|th)?"), geti(&words.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&words.clone(), &Py::Int(1)))); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(1)), &py_str("of"))) } } } } } } } }) {
    rest = if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&words.clone(), &Py::Int(1))))) { mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]) } else { mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(2), &Py::None, &Py::None)]) };
    return add(&add(&add(&py_str("on the "), &geti(&words.clone(), &Py::Int(0))), &py_str(" of ")), &rest.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return add(&py_str("in "), &phrase.clone());
    }
    return add(&py_str("on "), &phrase.clone());
    Py::None
}
pub fn _same(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let _ = py_str("True when the two words are the same word, with or without a verb ending on it.");
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&word.clone(), "lower", &[]), mcall(&other.clone(), "lower", &[])]), 2, None);
    word = __unpk1[0].clone();
    other = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &other.clone())); if py_truthy(&__b) { __b } else { bitand(&crate::gen::memory::topics::forms(&[word.clone()]), &crate::gen::memory::topics::forms(&[other.clone()])) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_eq(&_root(&[word.clone()]), &_root(&[other.clone()])))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&_a_plural_stem(&[word.clone()]), &other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &_a_plural_stem(&[other.clone()]))) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&mcall(&g_SAME_THING(), "get", &[word.clone(), py_tuple(vec![])]), &other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&g_SAME_THING(), "get", &[other.clone(), py_tuple(vec![])]), &word.clone())) } };
    Py::None
}
pub fn _root(args: &[Py]) -> Py {
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
    let mut ending: Py = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(2)) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn _a_plural_stem(args: &[Py]) -> Py {
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
    let _ = py_str("The word without a plural \"s\", or \"\" when there is none to take off.");
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[g_NOT_A_PLURAL()]))) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    return py_str("");
    Py::None
}
pub fn the_part_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut is_a_place: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "is_a_place") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut text: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The part of a plan an ask names on its own, as a whole sentence, or \"\".\n\n    \"where are we going again\" after \"we're off to portugal in august\" is \"Portugal.\", and\n    \"which month are we going\" is \"In August.\". The plan read out whole is right and is what the\n    bot said the turn before, so a person asking for a part gets the part. \"\" means the ask names\n    a part this plan does not carry (a where ask on a plan with no place, a month ask on a plan\n    with only a day), and the caller keeps its own answer.\n\n    `line` is the sentence the plan was said in, when the store can find it. A plan is kept\n    without its verb (\"house in june\"), and only that sentence can say whether anybody is going\n    anywhere: it is where the place after \"to\" comes from. `is_a_place` is the caller's own\n    reading of a place (the store has one), used to tell a place named on its own (\"Portugal\",\n    \"Bristol\") from the object of a verb kept without it (\"House\").\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    ask = call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ask.clone())) } }) {
    return py_str("");
    }
    spans = _the_dates_in(&[text.clone()]);
    if py_truthy(&mcall(&g_ASKS_WHERE(), "search", &[ask.clone()])) {
    place = _the_place(&[text.clone(), line.clone(), is_a_place.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    return add(&_a_name(&[place.clone()]), &py_str("."));
    }
    if py_truthy(&mcall(&g_ASKS_WHEN(), "search", &[ask.clone()])) {
    when = _the_date_asked(&[ask.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk5523 = py_unpack(&__item.clone(), 3, None); let mut _u: Py = __unpk5523[0].clone(); let mut _u: Py = __unpk5523[1].clone(); let mut said: Py = __unpk5523[2].clone(); __out.push(said.clone()); } py_list(__out) })() }]);
    return if py_truthy(&when.clone()) { _sentence(&[when.clone()]) } else { py_str("") };
    }
    return py_str("");
    Py::None
}
pub fn _the_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut is_a_place: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "is_a_place") { v.clone() }
    else { Py::None };
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The place a plan carries, said the way the person said it, or \"\" when it has none.\n\n    The sentence the plan came from is asked first, because it still has the verb that makes the\n    place one (\"i'm going to the gym tomorrow\" -> \"the gym\"). The plan on its own is used only\n    when what is left of it is a place by the caller's own reading (\"Portugal in august\" ->\n    \"Portugal\"), which is how the store keeps a place it lifted out of a trip; \"house in june\"\n    is the object of a move, and a where ask about it keeps the plan sentence.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&said.clone()) {
    found = mcall(&g_GOES_TO(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    place = _without_the_date(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    place = mcall(&mcall(&g_TRAILING_JOINER(), "sub", &[py_str(""), place.clone()]), "strip", &[py_str(" .,;:!?-")]);
    if py_truthy(&_names_a_place(&[place.clone()])) {
    return place.clone();
    }
    }
    }
    place = _without_the_date(&[plan.clone()]);
    place = mcall(&mcall(&g_TRAILING_JOINER(), "sub", &[py_str(""), place.clone()]), "strip", &[py_str(" .,;:!?-")]);
    cut = mcall(&g_PLACE_PREP(), "search", &[place.clone()]);
    if py_truthy(&cut.clone()) {
    place = mcall(&mcall(&cut.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" .,;:!?-")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_a_place(&[place.clone()])))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&is_a_place.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&is_a_place, &[place.clone()]))) } }) {
    return py_str("");
    }
    return place.clone();
    Py::None
}
pub fn _a_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut out: Py = Py::None;
    let mut small: Py = Py::None;
    let mut title: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A place said as a name: \"new york\" -> \"New York\", \"the gym\" -> \"The gym\".\n\n    A name of more than one word with no small word in it is a proper name and is written in\n    title case (\"New York\"); everything else keeps its own capitals and takes one on the first\n    letter, so \"the gym\" is not \"The Gym\".\n    ");
    small = call_builtin("frozenset", &[mcall(&py_str("the a an of and in on at for to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    words = mcall(&place.clone(), "split", &[]);
    title = { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&small.clone(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    turn = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&turn.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { title.clone() } } } }) {
    word = add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _without_the_date(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut end: Py = Py::None;
    let mut out: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("A phrase with every date taken out of it: \"portugal in august\" -> \"portugal\".");
    out = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_the_dates_in(&[out.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    _u = __unpk2[2].clone();
    out = add(&add(&slice_or_raise(&out.clone(), &Py::None, &start.clone(), &Py::None), &py_str(" ")), &slice_or_raise(&out.clone(), &end.clone(), &Py::None, &Py::None));
    }
    return mcall(&py_str(" "), "join", &[mcall(&out.clone(), "split", &[])]);
    Py::None
}
pub fn _names_a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut plain: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a phrase left over from a plan is a place and not the remains of a sentence.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[a-z]"), place.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    plain = mcall(&mcall(&place.clone(), "lower", &[]), "strip", &[py_str(" .,;:!?-")]);
    if py_truthy(&Py::Bool(py_contains(&g_NO_PLACE(), &plain.clone()))) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), plain.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_VERB()) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:is|are|was|were|am|and|with|for|about)\\b"), plain.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _the_dates_in(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("Every date in a piece of a line, in the order it is said, with where it sits.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![crate::gen::memory::facts::g_FUTURE(), g_SHORT_DATE(), g_PAST_WHEN()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let __unpk7292 = py_unpack(&__item.clone(), 3, None); let mut start: Py = __unpk7292[0].clone(); let mut end: Py = __unpk7292[1].clone(); let mut _u: Py = __unpk7292[2].clone(); __out.push({ let __b = lt(&mcall(&found.clone(), "start", &[]), &end.clone()); if !py_truthy(&__b) { __b } else { lt(&start.clone(), &mcall(&found.clone(), "end", &[])) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[]), mcall(&found.clone(), "group", &[Py::Int(0)])])]);
    }
    }
    let _ = mcall(&out.clone(), "sort", &[]);
    return out.clone();
    Py::None
}
pub fn _the_date_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut dates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dates"))) };
    let mut plain: Py = Py::None;
    let mut when: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The date the ask names the kind of: the month it asks for, the day it asks for.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[dates.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    when = __item.clone();
    plain = mcall(&_plain(&[when.clone()]), "lower", &[]);
    words = call_builtin("set", &[mcall(&plain.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_ASKS_THE_MONTH(), "search", &[ask.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &crate::gen::memory::facts::g_MONTHS()))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_ASKS_THE_DAY(), "search", &[ask.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = bitand(&words.clone(), &crate::gen::memory::facts::g_DAYS()); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), plain.clone()]).unwrap_or_else(|e| exc::raise(e)) } })) } }) {
    continue;
    }
    return when.clone();
    }
    return py_str("");
    Py::None
}
pub fn already_answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut line: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_when: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the thing the person asks about has happened, from the line that dated it.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ALREADY_DONE_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_ASK(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SAYS_IT_AGAIN(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_OFTEN(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[line.clone()])) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&_dated(&[memory.clone()]), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    spans = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_in_the_last_few_turns(&[memory.clone(), said.clone()]))) } }) {
    continue;
    }
    when = { let __b = if py_truthy(&asked.clone()) { _time_for(&[asked.clone(), said.clone(), spans.clone()]) } else { py_str("") }; if py_truthy(&__b) { __b } else { if py_truthy(&spans.clone()) { geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0)) } else { py_str("") } } };
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    continue;
    }
    plain = mcall(&_plain(&[when.clone()]), "split", &[]);
    if py_truthy(&_has_happened(&[said.clone()])) {
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_DAYS(), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return add(&py_str("Yes - that was in "), &_sentence(&[when.clone()]));
    }
    said_when = _sentence(&[when.clone()]);
    return add(&add(&py_str("Yes - that was "), &mcall(&slice_or_raise(&said_when.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said_when.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_DAYS(), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return add(&py_str("No - that's in "), &_sentence(&[when.clone()]));
    }
    said_when = _sentence(&[when.clone()]);
    return add(&add(&py_str("No - that's "), &mcall(&slice_or_raise(&said_when.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said_when.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return Py::None;
    Py::None
}
pub fn _has_happened(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line that gave the date says the thing is behind the person, not ahead.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_SAYS_IT_IS_COMING(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_GOING_ON(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WAS_GOING_ON(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    return _said_in_the_past(&[said.clone()]);
    Py::None
}
pub fn _was_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A date the line says has already happened: \"in june\" -> \"That was in June.\" ");
    plain = mcall(&_plain(&[when.clone()]), "split", &[]);
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_DAYS(), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return add(&py_str("That was in "), &_sentence(&[when.clone()]));
    }
    said = _sentence(&[when.clone()]);
    return add(&add(&py_str("That was "), &mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut low: Py = Py::None;
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A date said on its own: \"in march\" -> \"In March.\" ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&when.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    low = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(",;:")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_DAYS(), &low.clone())) } }) {
    word = add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[out.clone()]), "rstrip", &[py_str(" .")]);
    return add(&add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _the_interval_it_means(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut length: Py = Py::None;
    let mut line_words: Py = Py::None;
    let mut named: Py = Py::None;
    let mut other: Py = Py::None;
    let mut points_back: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The length said for the thing the ask names, or \"\" when no recent line gives one.\n\n    Only a line that is still one of the last few things they said counts, and the ask has to name\n    that line's thing - or point back at it (\"how long have i got\", \"how many weeks left\") - so a\n    length said about one thing is not handed to an ask about another. Nothing here counts\n    calendar days: the line gave a length, not a date, so the answer is that length said back and\n    the honest note that it was measured from when they said it.\n    ");
    said_lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&g_BACK_TURNS()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(_said_before_now(&[memory.clone(), turn.clone()])); } py_list(__out) })() };
    said_lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_lines.clone()) { let mut said: Py = __item.clone(); if py_truthy(&said.clone()) {  __out.push(said.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said_lines.clone()))) {
    return py_str("");
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNTIL_THROWS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    points_back = { let __b = _points_back(&[line.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asked.clone())) } };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[said_lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    said = __item.clone();
    found = mcall(&g_AN_INTERVAL_AHEAD(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]), "endswith", &[py_tuple(vec![py_str("day"), py_str("days")])]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:ago|since)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&points_back.clone())); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    line_words = call_builtin("set", &[crate::gen::memory::topics::content(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&line_words.clone()) { let mut other: Py = __item.clone(); __out.push(_same(&[word.clone(), other.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    }
    length = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[]), &Py::Int(1), &Py::None, &Py::None)]);
    return { let mut __s = String::new(); __s.push_str("You said "); __s.push_str(&fstr(&length.clone(), -1, "")); __s.push_str(", so a little less than that by now."); py_string(__s) };
    }
    return py_str("");
    Py::None
}
pub fn _said_before_now(args: &[Py]) -> Py {
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
    let mut turn: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turn") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turn"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The line the person typed on that turn, or \"\" when it is the bot's own turn.");
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    return if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]) } else { py_str("") };
    Py::None
}
pub fn _said_in_the_past(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line the date came from says the thing has already happened.");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_SAID_IN_THE_PAST(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_IN_THE_PAST(), "search", &[said.clone()]))) } };
    Py::None
}
pub fn _a_trip_is_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut dated: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dated") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dated"))) };
    let mut place: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line says a trip or a journey to a place of its own.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = dated.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_TRIP_SAID(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_GOING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&place.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &add(&place.clone(), &Py::Int(1))), &py_str("to"))) } } } }) {
    if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_PLACE(), &geti(&words.clone(), &add(&place.clone(), &Py::Int(2)))))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_bare_unit_ask_wants_a_length(args: &[Py]) -> Py {
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
    let mut dated: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dated") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dated"))) };
    let mut day: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a bare unit ask is about a length, not a countdown.");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_COUNT_TAIL(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    dated = call_builtin("str", &[{ let __b = dated.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_LENGTH_IN_THE_LINE(), "search", &[dated.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bdays?\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&_the_day(&[dated.clone()]), 2, None);
    day = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&day.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&kind.clone(), &py_str("month"))) } }) {
    return Py::Bool(false);
    }
    return _a_trip_is_named(&[dated.clone()]);
    Py::None
}
pub fn until_ask_matches(args: &[Py]) -> Py {
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
    let mut again: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "again") { v.clone() }
    else { Py::None };
    let _ = py_str("Whether the line asks how long until something, a bare unit ask counting as one.");
    if py_truthy(&Py::Bool(py_is(&again.clone(), &Py::None))) {
    again = mcall(&g_BARE_UNIT_ASK(), "match", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_UNTIL_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { again.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _today(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("The day these replies count from. A plain function so a test can stand still.");
    return stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn until_answer(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut day: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut month: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("How long until the thing the line asks about, or None when no line dates it.\n\n    Every line is offered this: the ask has to be a how-long-until ask and a dated line has to be\n    in the last few turns, and anything else falls through untouched.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    again = mcall(&g_BARE_UNIT_ASK(), "match", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&until_ask_matches(&[line.clone(), again.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_SAID(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN_DEATH(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_UNTIL_A_LEARNING(), "search", &[line.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_date_it_means(&[if py_truthy(&again.clone()) { py_str("how long until that") } else { line.clone() }, memory.clone()]), 2, None);
    when = __unpk1[0].clone();
    dated = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    return { let __b = _the_interval_it_means(&[line.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    if py_truthy(&{ let __b = again.clone(); if !py_truthy(&__b) { __b } else { _a_bare_unit_ask_wants_a_length(&[line.clone(), dated.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_AN_INTERVAL_AHEAD(), "search", &[when.clone()])) {
    return { let __b = _the_interval_it_means(&[line.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    month = py_str("");
    if py_truthy(&_a_day_but_no_month(&[when.clone()])) {
    month = _the_month_in_play(&[when.clone(), memory.clone()]);
    }
    let __unpk2 = py_unpack(&_the_day(&[if py_truthy(&month.clone()) { add(&add(&when.clone(), &py_str(" ")), &month.clone()) } else { when.clone() }]), 2, None);
    day = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_is(&day.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&_said_in_the_past(&[dated.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AGO_ASK(), "search", &[line.clone()])))) {
    return Py::None;
    }
    return _how_long_ago(&[day.clone(), kind.clone(), line.clone()]);
    }
    if py_truthy(&mcall(&g_AGO_ASK(), "search", &[line.clone()])) {
    return Py::None;
    }
    return _how_long(&[day.clone(), kind.clone(), line.clone()]);
    Py::None
}
pub fn _the_date_it_means(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut newest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut trip: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The date words the ask is about, or None. The ask's own body, not a `when` ask.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNTIL_THROWS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { _points_back(&[line.clone()]) } }) {
    return _the_newest_date(&[memory.clone()]);
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    trip = Py::Bool(!py_eq(&asked.clone(), &words.clone()));
    let __iter1 = py_iter_or_raise(&_dated(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    spans = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN_DEATH(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = trip.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_a_trip_line(&[said.clone()]))) } }) {
    continue;
    }
    when = _time_for(&[asked.clone(), said.clone(), spans.clone()]);
    if py_truthy(&when.clone()) {
    return py_tuple(vec![when.clone(), said.clone()]);
    }
    }
    if py_truthy(&{ let __b = mcall(&g_UNTIL_COUNTDOWN(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_DATE_ASK(), "search", &[line.clone()]))) } }) {
    let __unpk3 = py_unpack(&_the_newest_date(&[memory.clone()]), 2, None);
    newest = __unpk3[0].clone();
    said = __unpk3[1].clone();
    if py_truthy(&{ let __b = newest.clone(); if !py_truthy(&__b) { __b } else { _names_a_thing_of_its_own(&[line.clone(), said.clone()]) } }) {
    return py_tuple(vec![Py::None, py_str("")]);
    }
    return py_tuple(vec![newest.clone(), said.clone()]);
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _the_newest_date(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spans: Py = Py::None;
    let _ = py_str("The date on the newest line they dated, or None when that line has gone cold.\n\n    A date on a death is not a date to count down to, and a line from ten turns ago is not what\n    \"how long till that\" means. A date they put right counts as the newest line: the store\n    rewrites those in their own words, so the rewritten line is not among the turns.\n    ");
    let __unpk1 = py_unpack(&_put_right(&[memory.clone()]), 2, None);
    fixed = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    fixed = if py_truthy(&fixed.clone()) { mcall(&py_str(" "), "join", &[mcall(&fixed.clone(), "split", &[])]) } else { py_str("") };
    let __iter2 = py_iter_or_raise(&_dated(&[memory.clone()]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    said = __unpk3[0].clone();
    spans = __unpk3[1].clone();
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN_DEATH(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = _said_in_the_last_few_turns(&[memory.clone(), said.clone()]); if py_truthy(&__b) { __b } else { { let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &fixed.clone())) } } } }) {
    return py_tuple(vec![geti(&geti(&spans.clone(), &neg(&Py::Int(1))), &Py::Int(0)), said.clone()]);
    }
    return py_tuple(vec![Py::None, py_str("")]);
    }
    return py_tuple(vec![Py::None, py_str("")]);
    Py::None
}
pub fn _a_day_but_no_month(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the date says which day of the month and not which month.\n\n    \"on the 12th\" is half a date and the month it belongs to is the one in play; \"on the 12th of\n    may\", \"in march\", \"on monday\", \"tomorrow\" are whole enough to read by themselves.\n    ");
    words = mcall(&mcall(&_plain(&[when.clone()]), "replace", &[py_str(","), py_str(" ")]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WEEKDAY_NUMBER(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("weekend"), py_str("month"), py_str("week"), py_str("year")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "fullmatch", &[py_str("\\d{1,2}(?:st|nd|rd|th)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_month_in_play(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut month: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The month a recent line named, for a day given on its own (\"on the 12th\").\n\n    A date is often given in two goes - \"we're going to rome in may\", then \"we fly on the 12th\" -\n    and the second line is the same date completed. The month is taken from the newest line that\n    named one, and only while that line is still in play: a month from ten turns ago is a different\n    thing's month, and guessing it would put the count on a day nobody gave.\n    ");
    month = py_str("");
    let __iter1 = py_iter_or_raise(&_dated(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    spans = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_said_in_the_last_few_turns(&[memory.clone(), said.clone()])))) {
    break;
    }
    let __iter3 = py_iter_or_raise(&spans.clone());
    for __item in __iter3 {
    span = __item.clone();
    words = mcall(&mcall(&_plain(&[geti(&span.clone(), &Py::Int(0))]), "replace", &[py_str(","), py_str(" ")]), "split", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    named = geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&mcall(&_plain(&[geti(&span.clone(), &Py::Int(0))]), "strip", &[]), &mcall(&_plain(&[when.clone()]), "strip", &[])))) {
    return named.clone();
    }
    month = { let __b = month.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    }
    }
    }
    return month.clone();
    Py::None
}
pub fn _a_day_number(args: &[Py]) -> Py {
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
    return call_builtin("bool", &[mcall(&g_DAY_NUMBER(), "match", &[word.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_day(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { Py::None };
    let mut ahead: Py = Py::None;
    let mut anchored: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut day: Py = Py::None;
    let mut early: Py = Py::None;
    let mut has_end: Py = Py::None;
    let mut has_month: Py = Py::None;
    let mut has_next: Py = Py::None;
    let mut has_this: Py = Py::None;
    let mut has_week: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut month: Py = Py::None;
    let mut started: Py = Py::None;
    let mut upcoming: Py = Py::None;
    let mut weekday: Py = Py::None;
    let mut when_day: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The calendar day a date said the short way falls on, and how it was said.\n\n    The kinds are how much of the date the person gave, because the reply says back only that\n    much: a month alone is \"sometime in October\", a day is \"Tuesday 14 October\". A date already\n    gone is the next time it comes round, which is how people say \"in march\" in November.\n    ");
    today = { let __b = today.clone(); if py_truthy(&__b) { __b } else { _today(&[]) } };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&_plain(&[when.clone()]), "replace", &[py_str(","), py_str(" ")]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    month = Py::None;
    day = Py::None;
    weekday = Py::None;
    early = Py::Bool(false);
    has_end = Py::Bool(false);
    has_next = Py::Bool(false);
    has_this = Py::Bool(false);
    has_month = Py::Bool(false);
    has_week = Py::Bool(false);
    dated = Py::Bool(false);
    anchored = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WEEKDAY_NUMBER(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&dated.clone()))) {
    started = Py::Bool(py_contains(&g_A_DATE_ITSELF(), &word.clone()));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&started.clone())); if !py_truthy(&__b) { __b } else { _a_day_number(&[word.clone()]) } }) {
    started = { let __b = anchored.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() })) } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&started.clone()))) {
    continue;
    }
    }
    dated = Py::Bool(true);
    if py_truthy(&Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone()))) {
    month = geti(&g_MONTH_NUMBER(), &word.clone());
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_WEEKDAY_NUMBER(), &word.clone()))) {
    weekday = geti(&g_WEEKDAY_NUMBER(), &word.clone());
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late")]), &word.clone()))) {
    early = Py::Bool(true);
    day = geti(&py_dict(vec![(py_str("early"), Py::Int(1)), (py_str("mid"), Py::Int(15)), (py_str("late"), Py::Int(25))]), &word.clone());
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("end"), py_str("ends"), py_str("ending")]), &word.clone()))) {
    has_end = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("next")))) {
    has_next = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("this")))) {
    has_this = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("month")))) {
    has_month = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("week")))) {
    has_week = Py::Bool(true);
    } else {
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d{1,2}(?:st|nd|rd|th)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    day = call_builtin("int", &[mcall(&stdlib_call("re", "match", &[py_str("\\d{1,2}"), word.clone()]).unwrap_or_else(|e| exc::raise(e)), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e));
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("start"), py_str("beginning"), py_str("weekend"), py_str("a"), py_str("an"), py_str("for"), py_str("from"), py_str("to"), py_str("until"), py_str("till"), py_str("around"), py_str("about")]), &word.clone()))) {
    continue;
    } else {
    return py_tuple(vec![Py::None, Py::None]);
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    }
    if py_truthy(&{ let __b = has_month.clone(); if !py_truthy(&__b) { __b } else { has_next.clone() } }) {
    return py_tuple(vec![_add_months(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) }, Py::Int(1)]), py_str("month")]);
    }
    if py_truthy(&{ let __b = has_month.clone(); if !py_truthy(&__b) { __b } else { has_this.clone() } }) {
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) }, py_str("month")]);
    }
    if py_truthy(&{ let __b = has_end.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&day.clone(), &Py::None)) } }) {
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    return py_tuple(vec![_last_day(&[_a_year_for(&[today.clone(), month.clone(), Py::Int(1)])]), py_str("end")]);
    }
    if py_truthy(&has_next.clone()) {
    return py_tuple(vec![_last_day(&[_add_months(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) }, Py::Int(1)])]), py_str("end")]);
    }
    return py_tuple(vec![_last_day(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) }]), py_str("end")]);
    }
    if py_truthy(&{ let __b = has_week.clone(); if !py_truthy(&__b) { __b } else { has_next.clone() } }) {
    return py_tuple(vec![add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(7))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), py_str("week")]);
    }
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    when_day = _a_year_for(&[today.clone(), month.clone(), day.clone()]);
    return py_tuple(vec![when_day.clone(), if py_truthy(&{ let __b = Py::Bool(py_is(&day.clone(), &Py::None)); if py_truthy(&__b) { __b } else { early.clone() } }) { py_str("month") } else { py_str("date") }]);
    }
    if py_truthy(&Py::Bool(!py_is(&weekday.clone(), &Py::None))) {
    ahead = { let __b = modulo(&sub(&weekday.clone(), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7)); if py_truthy(&__b) { __b } else { Py::Int(7) } };
    return py_tuple(vec![add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), ahead.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), py_str("date")]);
    }
    if py_truthy(&Py::Bool(!py_is(&day.clone(), &Py::None))) {
    let __unpk3 = py_unpack(&_this_month_or_next(&[today.clone(), day.clone()]), 2, None);
    upcoming = __unpk3[0].clone();
    kind = __unpk3[1].clone();
    return py_tuple(vec![upcoming.clone(), if py_truthy(&early.clone()) { py_str("month") } else { kind.clone() }]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _this_month_or_next(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut today: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut day: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut found: Py = Py::None;
    let _ = py_str("That day number in this month, or next month when this month's has gone.");
    let __try1 = exc::catch(|| {
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), day.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) };
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
    return py_tuple(vec![_a_year_for(&[today.clone(), add(&gattr(&today.clone(), "month"), &Py::Int(1)), day.clone()]), py_str("date")]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&ge(&found.clone(), &today.clone())) {
    return py_tuple(vec![found.clone(), py_str("date")]);
    }
    return py_tuple(vec![_a_year_for(&[today.clone(), add(&gattr(&today.clone(), "month"), &Py::Int(1)), day.clone()]), py_str("date")]);
    Py::None
}
pub fn _a_year_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut today: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut day: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut found: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("That month and day in the year it next comes round, spoken of as a date.");
    while py_truthy(&gt(&month.clone(), &Py::Int(12))) {
    let __aug1 = sub(&month.clone(), &Py::Int(12));
    month = __aug1;
    today = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("year".to_string(), add(&gattr(&today.clone(), "year"), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) };
    }
    day = { let __b = day.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } };
    let __iter2 = py_iter_or_raise(&py_tuple(vec![gattr(&today.clone(), "year"), add(&gattr(&today.clone(), "year"), &Py::Int(1))]));
    for __item in __iter2 {
    year = __item.clone();
    let __try3 = exc::catch(|| {
    found = stdlib_call("datetime", "date", &[year.clone(), month.clone(), call_builtin("min", &[day.clone(), _days_in(&[year.clone(), month.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&ge(&found.clone(), &today.clone())) {
    return found.clone();
    }
    }
    return stdlib_call("datetime", "date", &[add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _days_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut year: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "year") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: year"))) };
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    if py_truthy(&Py::Bool(py_eq(&month.clone(), &Py::Int(12)))) {
    return Py::Int(31);
    }
    return gattr(&sub(&stdlib_call("datetime", "date", &[year.clone(), add(&month.clone(), &Py::Int(1)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &stdlib_call("datetime", "date", &[year.clone(), month.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))), "days");
    Py::None
}
pub fn _last_day(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut first_of_month: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "first_of_month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: first_of_month"))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), _days_in(&[gattr(&first_of_month.clone(), "year"), gattr(&first_of_month.clone(), "month")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first_of_month.clone(), "replace", args))(&__args) };
    Py::None
}
pub fn _add_months(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut months: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: months"))) };
    let mut month: Py = Py::None;
    let mut year: Py = Py::None;
    month = add(&gattr(&day.clone(), "month"), &months.clone());
    year = gattr(&day.clone(), "year");
    while py_truthy(&gt(&month.clone(), &Py::Int(12))) {
    let __aug1 = sub(&month.clone(), &Py::Int(12));
    month = __aug1;
    let __aug2 = add(&year.clone(), &Py::Int(1));
    year = __aug2;
    }
    return stdlib_call("datetime", "date", &[year.clone(), month.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_long(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asked: Py = Py::None;
    let mut days: Py = Py::None;
    let mut head: Py = Py::None;
    let mut months: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("The reply: how many days or weeks off it is, and the day itself when they said one.");
    today = _today(&[]);
    days = gattr(&sub(&day.clone(), &today.clone()), "days");
    if py_truthy(&lt(&days.clone(), &Py::Int(0))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return py_str("That's today.");
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    return py_str("That's tomorrow.");
    }
    asked = mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    named = py_str("");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bweeks?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    named = py_str("week");
    unit = py_str("week");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bdays?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    named = py_str("day");
    unit = py_str("day");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonths?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    named = py_str("month");
    unit = py_str("month");
    } else {
    if py_truthy(&le(&days.clone(), &Py::Int(14))) {
    unit = py_str("day");
    } else {
    if py_truthy(&le(&days.clone(), &Py::Int(84))) {
    unit = py_str("week");
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("day")))) {
    head = modulo(&py_str("That's %d days away"), &days.clone());
    } else {
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    weeks = call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&le(&weeks.clone(), &Py::Int(1))) {
    head = py_str("That's just over a week away");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&named.clone(), &py_str("week"))); if py_truthy(&__b) { __b } else { le(&weeks.clone(), &Py::Int(30)) } }) {
    head = modulo(&py_str("That's about %d weeks away"), &weeks.clone());
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("month")))) {
    months = call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&le(&months.clone(), &Py::Int(1))) { py_str("That's about a month away") } else { modulo(&py_str("That's about %d months away"), &months.clone()) };
    }
    said = _the_day_said(&[day.clone(), kind.clone()]);
    return if py_truthy(&said.clone()) { add(&add(&add(&head.clone(), &py_str(" - ")), &said.clone()), &py_str(".")) } else { add(&head.clone(), &py_str(".")) };
    Py::None
}
pub fn _how_long_ago(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asked: Py = Py::None;
    let mut days: Py = Py::None;
    let mut head: Py = Py::None;
    let mut months: Py = Py::None;
    let mut past: Py = Py::None;
    let mut said: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("The reply to a count-back: how long ago the day they gave was, and the day itself.\n\n    The day on file is the next time that date comes round (that is how a date said the short way\n    is read), so the same month and day a year or two back is the one they mean: \"i had an exam in\n    june\" asked \"how long ago was that\" is \"That was about 5 months ago - in June.\"\n    ");
    today = _today(&[]);
    past = _a_day_in_the_past(&[day.clone(), today.clone()]);
    if py_truthy(&Py::Bool(py_is(&past.clone(), &Py::None))) {
    return Py::None;
    }
    days = gattr(&sub(&today.clone(), &past.clone()), "days");
    if py_truthy(&le(&days.clone(), &Py::Int(0))) {
    return Py::None;
    }
    asked = mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bweeks?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    unit = py_str("week");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bdays?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    unit = py_str("day");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonths?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    unit = py_str("month");
    } else {
    if py_truthy(&le(&days.clone(), &Py::Int(14))) {
    unit = py_str("day");
    } else {
    if py_truthy(&le(&days.clone(), &Py::Int(84))) {
    unit = py_str("week");
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("day")))) {
    head = modulo(&py_str("That was %d days ago"), &days.clone());
    } else {
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    weeks = call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&le(&weeks.clone(), &Py::Int(1))) { py_str("That was just over a week ago") } else { modulo(&py_str("That was about %d weeks ago"), &weeks.clone()) };
    } else {
    months = call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&le(&months.clone(), &Py::Int(1))) { py_str("That was about a month ago") } else { modulo(&py_str("That was about %d months ago"), &months.clone()) };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("month")))) {
    said = add(&py_str("in "), &geti(&g_MONTH_NAME(), &gattr(&past.clone(), "month")));
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("end")))) {
    said = add(&py_str("at the end of "), &geti(&g_MONTH_NAME(), &gattr(&past.clone(), "month")));
    } else {
    said = _the_day_said(&[past.clone(), kind.clone()]);
    }
    }
    return if py_truthy(&said.clone()) { add(&add(&add(&head.clone(), &py_str(" - ")), &said.clone()), &py_str(".")) } else { add(&head.clone(), &py_str(".")) };
    Py::None
}
pub fn _a_day_in_the_past(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut found: Py = Py::None;
    let mut month: Py = Py::None;
    let mut number: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The same month and day as `day`, on the side of today that has already happened.");
    let __unpk1 = py_unpack(&py_tuple(vec![gattr(&day.clone(), "month"), gattr(&day.clone(), "day")]), 2, None);
    month = __unpk1[0].clone();
    number = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![sub(&gattr(&day.clone(), "year"), &Py::Int(1)), sub(&gattr(&day.clone(), "year"), &Py::Int(2)), sub(&gattr(&day.clone(), "year"), &Py::Int(3))]));
    for __item in __iter2 {
    year = __item.clone();
    let __try3 = exc::catch(|| {
    found = stdlib_call("datetime", "date", &[year.clone(), month.clone(), call_builtin("min", &[number.clone(), _days_in(&[year.clone(), month.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&le(&found.clone(), &today.clone())) {
    return found.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_day_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut month: Py = Py::None;
    let _ = py_str("The day itself, said as much as the person said it: \"sometime in October\", \"Sunday 15\n    November\", \"the end of November\".");
    month = geti(&g_MONTH_NAME(), &gattr(&day.clone(), "month"));
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("month")))) {
    return add(&py_str("sometime in "), &month.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("end")))) {
    return add(&py_str("the end of "), &month.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("week")))) {
    return add(&py_str("around "), &geti(&g_WEEKDAY_NAME(), &mcall(&day.clone(), "weekday", &[])));
    }
    return modulo(&py_str("%s %d %s"), &py_tuple(vec![geti(&g_WEEKDAY_NAME(), &mcall(&day.clone(), "weekday", &[])), gattr(&day.clone(), "day"), month.clone()]));
    Py::None
}
pub fn a_plan_moved_to_a_day(args: &[Py]) -> Py {
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
    let mut day: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("{\"day\": \"friday night\", \"line\": \"can i make it ahead of time\"}, or None.\n\n    \"im cooking chilli for 6\" / \"can i make it friday night instead\" is a plan moved to another\n    day, and the question inside it is whether the dish can be made ahead: the answer that says\n    it keeps three days and is better the next day is exactly what was asked, and it was given for\n    \"can i make it the day before\" already. The line is read by putting \"ahead of time\" where the\n    day was, so the same reader answers both, and the day is handed back so the reply can say the\n    day out loud.\n\n    None unless the line names a day, says the plan moved, names a plan verb and points back at\n    the plan with \"it\"/\"that\", so nothing that says what it is about is rewritten here.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::None;
    }
    day = mcall(&g__DAY_NAMED(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&day.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__MOVED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[add(&py_str("\\b(?:on|till|until)\\s+"), &stdlib_call("re", "escape", &[mcall(&day.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__PLAN_VERB(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__POINTS_AT_IT(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ahead of time")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DAY_NAMED(), "sub", args))(&__args) };
    line = mcall(&g__MOVED(), "sub", &[py_str(""), line.clone()]);
    line = stdlib_call("re", "sub", &[py_str("\\b(?:on|till|until)\\s+ahead of time\\b"), py_str("ahead of time"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    line = mcall(&py_str(" "), "join", &[mcall(&line.clone(), "split", &[])]);
    if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("ahead of time")))) {
    return Py::None;
    }
    return py_dict(vec![(py_str("day"), mcall(&py_str(" "), "join", &[mcall(&mcall(&day.clone(), "group", &[Py::Int(0)]), "split", &[])])), (py_str("line"), line.clone())]);
    Py::None
}
pub fn how_long_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The length the chat says it has been, or None.\n\n    Asked only of a line that is the \"how long have/has\" question and of no other, so every reply\n    it makes is about a day the person gave and nothing else in the bot changes.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_HOW_LONG_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_LENGTH_ASK(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_CALENDAR_NOW(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_SAID(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    put_right = _a_day_put_right(&[memory.clone()]);
    if py_truthy(&put_right.clone()) {
    return put_right.clone();
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::content(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASKED_ABOUT_THE_THING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&_the_lines_that_began(&[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    said_words = __unpk2[1].clone();
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_it(&[named.clone(), said.clone()]))) } }) {
    continue;
    }
    answer = _the_length_from(&[said.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_day_put_right(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut day: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The length from a day the person has just put right, or None.\n\n    Only the newest line is read, and only when the line before it is one that began: a correction\n    of the bot's own last answer about something else is not a day of theirs.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&Py::Int(1))) } else { Py::None };
    before = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(2))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&Py::Int(2))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&before.clone())) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    earlier = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&before.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&earlier.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BEGAN(), "search", &[earlier.clone()]))) } } } }) {
    return Py::None;
    }
    found = mcall(&g_DAY_PUT_RIGHT(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    day = mcall(&found.clone(), "group", &[py_str("day")]);
    return _the_length_from(&[if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DAY_FORM(), "search", &[day.clone()])))) { add(&py_str("it began in "), &day.clone()) } else { add(&py_str("it began "), &day.clone()) }]);
    Py::None
}
pub fn _the_lines_that_began(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The lines of theirs that say when something began, newest first, with their words.\n\n    A line counts when it says a thing began and when it says how long ago - \"i moved to leeds two\n    years ago\" has both, and either alone is a line the ask can lean on. A line that asks a\n    question is not one: \"how long have you had that cough\" is a question of theirs about the bot's\n    last line, not a day of their own.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { _is_an_ask(&[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_BEGAN(), "search", &[said.clone()])))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![said.clone(), crate::gen::memory::topics::content(&[said.clone()])])]);
    if py_truthy(&ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    break;
    }
    }
    return found.clone();
    Py::None
}
pub fn _names_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut other: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the ask's own words are in the line the day came from.\n\n    \"how long have i been playing the piano\" after \"i started learning the guitar in january\" is a\n    different thing, and a length from the wrong line is worse than no answer. A word matches the\n    way people vary it, so \"playing\" finds \"play\" and \"learning\" finds \"learn\".\n    ");
    said_words = crate::gen::memory::topics::content(&[said.clone()]);
    let __iter1 = py_iter_or_raise(&named.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_words.clone()) { let mut other: Py = __item.clone(); __out.push(_same(&[word.clone(), other.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_length_from(args: &[Py]) -> Py {
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
    let mut _at: Py = Py::None;
    let mut ago: Py = Py::None;
    let mut count: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said_length: Py = Py::None;
    let mut span: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut weekday: Py = Py::None;
    let mut when: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The length from today back to the day this line names, said as a person says it, or None.");
    ago = mcall(&g_AGO_LENGTH(), "search", &[said.clone()]);
    if py_truthy(&ago.clone()) {
    count = _a_count(&[mcall(&ago.clone(), "group", &[py_str("n")])]);
    unit = mcall(&mcall(&mcall(&ago.clone(), "group", &[py_str("unit")]), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("couple")))) {
    return _about(&[py_str("a couple of weeks"), py_str("")]);
    }
    days = mul(&count.clone(), &mcall(&g_A_UNIT_OF_TIME(), "get", &[unit.clone(), Py::Int(0)]));
    said_length = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&ago.clone(), "group", &[Py::Int(0)]), "split", &[]), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    when = py_str("");
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("year")))) {
    when = call_builtin("str", &[sub(&gattr(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "year"), &count.clone())]).unwrap_or_else(|e| exc::raise(e));
    }
    return if py_truthy(&days.clone()) { _about(&[said_length.clone(), when.clone()]) } else { Py::None };
    }
    last = mcall(&g_LAST_LENGTH(), "search", &[said.clone()]);
    if py_truthy(&last.clone()) {
    unit = mcall(&mcall(&last.clone(), "group", &[py_str("unit")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    return _about(&[py_str("a week"), py_str("")]);
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("fortnight")))) {
    return _about(&[py_str("a fortnight"), py_str("")]);
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("month")))) {
    return _about(&[py_str("a month"), _the_month_back(&[Py::Int(1)])]);
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("year")))) {
    return _about(&[py_str("a year"), call_builtin("str", &[sub(&gattr(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "year"), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    held = mcall(&g_FOR_LENGTH(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AGO_LENGTH(), "search", &[said.clone()]))) } }) {
    count = _a_count(&[mcall(&held.clone(), "group", &[py_str("n")])]);
    unit = mcall(&mcall(&mcall(&held.clone(), "group", &[py_str("unit")]), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&{ let __b = count.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_UNIT_OF_TIME(), &unit.clone())) } }) {
    return _about(&[modulo(&py_str("%s %s%s"), &py_tuple(vec![_a_count_said(&[count.clone()]), unit.clone(), if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }])), py_str("")]);
    }
    }
    let __iter1 = py_iter_or_raise(&_dated(&[memory_of_one(&[said.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    span = __unpk2[0].clone();
    _at = __unpk2[1].clone();
    day = _the_day_a_span_names(&[span.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&day.clone()))) {
    continue;
    }
    return _about(&[_a_length_in_words(&[gattr(&sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), _a_day_said(&[day.clone()])]);
    }
    weekday = mcall(&g_WEEKDAY_SAID(), "search", &[said.clone()]);
    if py_truthy(&weekday.clone()) {
    day = _the_day_a_weekday_names(&[mcall(&weekday.clone(), "group", &[py_str("day")])]);
    if py_truthy(&day.clone()) {
    return _about(&[_a_length_in_words(&[gattr(&sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), if py_truthy(&Py::Bool(py_eq(&gattr(&day.clone(), "day"), &Py::Int(1)))) { _the_month_name(&[gattr(&day.clone(), "month")]) } else { mcall(&mcall(&weekday.clone(), "group", &[py_str("day")]), "capitalize", &[]) }]);
    }
    }
    year = mcall(&g_A_YEAR_SAID(), "search", &[said.clone()]);
    if py_truthy(&year.clone()) {
    day = _the_day_a_span_names(&[mcall(&year.clone(), "group", &[Py::Int(0)])]);
    if py_truthy(&day.clone()) {
    return _about(&[_a_length_in_words(&[gattr(&sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), mcall(&year.clone(), "group", &[Py::Int(0)])]);
    }
    }
    return Py::None;
    Py::None
}
pub fn memory_of_one(args: &[Py]) -> Py {
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
    let _ = py_str("One line of theirs, in the shape the date readers take.");
    return py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("user"), said.clone()), (py_str("bot"), py_str(""))])]))]);
    Py::None
}
pub fn _a_count(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The number a length is said with, or 0.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&g_NUMBER_WORDS(), "get", &[word.clone(), Py::Int(0)]);
    Py::None
}
pub fn _the_day_a_span_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut span: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span"))) };
    let mut back: Py = Py::None;
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut month: Py = Py::None;
    let mut today: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The day a date in a line names, as a date, choosing the one behind us.\n\n    \"in january\" said in November is this year's January and said in March is last year's: a thing\n    that began began, so the day is the most recent one that has already been.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9]+"), mcall(&call_builtin("str", &[{ let __b = span.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    month = Py::Int(0);
    day = Py::Int(0);
    year = Py::Int(0);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::facts::g_MONTHS(), &word.clone()))) {
    month = _the_month_number(&[word.clone()]);
    } else {
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d{4}"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    year = call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    } else {
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d{1,2}(?:st|nd|rd|th)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    day = call_builtin("int", &[stdlib_call("re", "sub", &[py_str("\\D"), py_str(""), word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late"), py_str("in"), py_str("on"), py_str("the"), py_str("of"), py_str("at"), py_str("since"), py_str("back")]), &word.clone()))) {
    continue;
    }
    }
    }
    }
    }
    today = stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&year.clone()) {
    return stdlib_call("datetime", "date", &[year.clone(), { let __b = month.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } }, { let __b = day.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&month.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    back = __item.clone();
    let __try3 = exc::catch(|| {
    found = stdlib_call("datetime", "date", &[sub(&gattr(&today.clone(), "year"), &back.clone()), month.clone(), { let __b = day.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e));
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
    return Py::None;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&le(&found.clone(), &today.clone())) {
    return found.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_day_a_weekday_names(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let _ = py_str("The most recent day with that name - today when today is that day.");
    let __try1 = exc::catch(|| {
    back = modulo(&sub(&mcall(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "weekday", &[]), &mcall(&g_WEEKDAYS(), "index", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])), &Py::Int(7));
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
    return sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), back.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    Py::None
}
pub fn _the_month_number(args: &[Py]) -> Py {
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
    let mut order: Py = Py::None;
    let _ = py_str("The number of a month, 1 to 12.");
    order = py_list(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")]);
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), add(&mcall(&order.clone(), "index", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), &Py::Int(1))]);
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
    return Py::Int(0);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_month_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut months: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: months"))) };
    let mut _u: Py = Py::None;
    let mut month: Py = Py::None;
    let mut today: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The name of the month this many months ago.");
    today = stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e));
    let __unpk1 = py_unpack(&py_tuple(vec![gattr(&today.clone(), "year"), gattr(&today.clone(), "month")]), 2, None);
    year = __unpk1[0].clone();
    month = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("max", &[Py::Int(1), call_builtin("int", &[{ let __b = months.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    _u = __item.clone();
    let __aug3 = sub(&month.clone(), &Py::Int(1));
    month = __aug3;
    if py_truthy(&lt(&month.clone(), &Py::Int(1))) {
    let __unpk4 = py_unpack(&py_tuple(vec![Py::Int(12), sub(&year.clone(), &Py::Int(1))]), 2, None);
    month = __unpk4[0].clone();
    year = __unpk4[1].clone();
    }
    }
    return _the_month_name(&[month.clone()]);
    Py::None
}
pub fn _the_month_name(args: &[Py]) -> Py {
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
    let mut order: Py = Py::None;
    order = py_list(vec![py_str("January"), py_str("February"), py_str("March"), py_str("April"), py_str("May"), py_str("June"), py_str("July"), py_str("August"), py_str("September"), py_str("October"), py_str("November"), py_str("December")]);
    return if py_truthy(&if py_truthy(&le(&Py::Int(1), &{ let __cmp10 = number.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(12)) } else { Py::Bool(false) }) { geti(&order.clone(), &sub(&number.clone(), &Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn _a_day_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let _ = py_str("A date said the way a person says one in a sentence - \"January\", \"the 4th of May\".");
    if py_truthy(&Py::Bool(py_eq(&gattr(&day.clone(), "day"), &Py::Int(1)))) {
    return _the_month_name(&[gattr(&day.clone(), "month")]);
    }
    return modulo(&py_str("the %d%s of %s"), &py_tuple(vec![gattr(&day.clone(), "day"), _an_ordinal_end(&[gattr(&day.clone(), "day")]), _the_month_name(&[gattr(&day.clone(), "month")])]));
    Py::None
}
pub fn _an_ordinal_end(args: &[Py]) -> Py {
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
    if py_truthy(&if py_truthy(&le(&Py::Int(10), &{ let __cmp10 = modulo(&number.clone(), &Py::Int(100)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(20)) } else { Py::Bool(false) }) {
    return py_str("th");
    }
    return mcall(&py_dict(vec![(Py::Int(1), py_str("st")), (Py::Int(2), py_str("nd")), (Py::Int(3), py_str("rd"))]), "get", &[modulo(&number.clone(), &Py::Int(10)), py_str("th")]);
    Py::None
}
pub fn _a_count_said(args: &[Py]) -> Py {
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
    let _ = py_str("A small count as a word, a larger one as a figure.");
    number = call_builtin("int", &[{ let __b = number.clone(); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&g_SPELLED_COUNT(), "get", &[number.clone(), call_builtin("str", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _a_length_in_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut days: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "days") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: days"))) };
    let mut halves: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("A number of days said the way a person says a length: \"about five months\", \"two years\".");
    days = call_builtin("max", &[Py::Int(0), call_builtin("int", &[{ let __b = days.clone(); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&days.clone(), &Py::Int(1))) {
    return py_str("a day");
    }
    if py_truthy(&lt(&days.clone(), &Py::Int(12))) {
    return modulo(&py_str("%s %s"), &py_tuple(vec![_a_count_said(&[days.clone()]), if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) { py_str("day") } else { py_str("days") }]));
    }
    if py_truthy(&lt(&days.clone(), &Py::Int(60))) {
    return modulo(&py_str("%s weeks"), &_a_count_said(&[call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    }
    if py_truthy(&lt(&days.clone(), &Py::Int(730))) {
    return modulo(&py_str("%s months"), &_a_count_said(&[call_builtin("int", &[call_builtin("round", &[truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    }
    years = truediv(&days.clone(), &Py::Float(365.25));
    halves = truediv(&call_builtin("int", &[mul(&years.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(2.0));
    if py_truthy(&Py::Bool(py_eq(&halves.clone(), &call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return modulo(&py_str("%s %s"), &py_tuple(vec![_a_count_said(&[call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))]), if py_truthy(&Py::Bool(py_eq(&halves.clone(), &Py::Int(1)))) { py_str("year") } else { py_str("years") }]));
    }
    return modulo(&py_str("%s and a half years"), &_a_count_said(&[call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))]));
    Py::None
}
pub fn _about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut length: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "length") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: length"))) };
    let mut when: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let _ = py_str("The reply: the length, and the day it is counted from when the day adds something.");
    length = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = length.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&length.clone()))) {
    return Py::None;
    }
    if py_truthy(&when.clone()) {
    return modulo(&py_str("%s - since %s."), &py_tuple(vec![_tidy(&[length.clone()]), when.clone()]));
    }
    return modulo(&py_str("%s."), &_tidy(&[length.clone()]));
    Py::None
}
pub fn _tidy(args: &[Py]) -> Py {
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
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return if py_truthy(&said.clone()) { add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.when".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGO_ASK" => Some(g_AGO_ASK()),
            "AGO_LENGTH" => Some(g_AGO_LENGTH()),
            "ALREADY_DONE_ASK" => Some(g_ALREADY_DONE_ASK()),
            "AN_ASK_IN_THE_PAST" => Some(g_AN_ASK_IN_THE_PAST()),
            "AN_INTERVAL_AHEAD" => Some(g_AN_INTERVAL_AHEAD()),
            "ASKED_ABOUT_THE_THING" => Some(g_ASKED_ABOUT_THE_THING()),
            "ASKS_THE_DAY" => Some(g_ASKS_THE_DAY()),
            "ASKS_THE_MONTH" => Some(g_ASKS_THE_MONTH()),
            "ASKS_WHEN" => Some(g_ASKS_WHEN()),
            "ASKS_WHERE" => Some(g_ASKS_WHERE()),
            "ASK_OPENER" => Some(g_ASK_OPENER()),
            "ASK_THROWS" => Some(g_ASK_THROWS()),
            "A_COUNT_TAIL" => Some(g_A_COUNT_TAIL()),
            "A_DATE_ITSELF" => Some(g_A_DATE_ITSELF()),
            "A_DATE_WORD" => Some(g_A_DATE_WORD()),
            "A_DAY_FORM" => Some(g_A_DAY_FORM()),
            "A_GOING" => Some(g_A_GOING()),
            "A_LENGTH_IN_THE_LINE" => Some(g_A_LENGTH_IN_THE_LINE()),
            "A_THING_NAMED" => Some(g_A_THING_NAMED()),
            "A_TRIP_SAID" => Some(g_A_TRIP_SAID()),
            "A_UNIT_OF_TIME" => Some(g_A_UNIT_OF_TIME()),
            "A_VERB" => Some(g_A_VERB()),
            "A_YEAR_SAID" => Some(g_A_YEAR_SAID()),
            "BACK_TURNS" => Some(g_BACK_TURNS()),
            "BARE_CALENDAR_ASK" => Some(g_BARE_CALENDAR_ASK()),
            "BARE_DAY" => Some(g_BARE_DAY()),
            "BARE_UNIT_ASK" => Some(g_BARE_UNIT_ASK()),
            "BEGAN" => Some(g_BEGAN()),
            "BOOKING" => Some(g_BOOKING()),
            "CALENDAR_NOW" => Some(g_CALENDAR_NOW()),
            "DAY_NUMBER" => Some(g_DAY_NUMBER()),
            "DAY_PUT_RIGHT" => Some(g_DAY_PUT_RIGHT()),
            "EVENT_NAME" => Some(g_EVENT_NAME()),
            "EVENT_WORD" => Some(g_EVENT_WORD()),
            "FILLER" => Some(g_FILLER()),
            "FOR_LENGTH" => Some(g_FOR_LENGTH()),
            "GOES_TO" => Some(g_GOES_TO()),
            "GOING_ON" => Some(g_GOING_ON()),
            "HOW_LONG_ASK" => Some(g_HOW_LONG_ASK()),
            "HOW_OFTEN" => Some(g_HOW_OFTEN()),
            "INTERVAL_JOINERS" => Some(g_INTERVAL_JOINERS()),
            "JOINED_TO_A_DATE" => Some(g_JOINED_TO_A_DATE()),
            "LAST_LENGTH" => Some(g_LAST_LENGTH()),
            "LEAD" => Some(g_LEAD()),
            "MONTH_AFTER_THE_DAY" => Some(g_MONTH_AFTER_THE_DAY()),
            "MONTH_NAME" => Some(g_MONTH_NAME()),
            "MONTH_NAMES" => Some(g_MONTH_NAMES()),
            "MONTH_NUMBER" => Some(g_MONTH_NUMBER()),
            "NOT_A_DATE_ASK" => Some(g_NOT_A_DATE_ASK()),
            "NOT_A_DATE_TO_GIVE" => Some(g_NOT_A_DATE_TO_GIVE()),
            "NOT_A_LENGTH_ASK" => Some(g_NOT_A_LENGTH_ASK()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NOT_A_PLURAL" => Some(g_NOT_A_PLURAL()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_IN_THE_PAST" => Some(g_NOT_IN_THE_PAST()),
            "NO_DATE_YET" => Some(g_NO_DATE_YET()),
            "NO_PLACE" => Some(g_NO_PLACE()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "PAST_WHEN" => Some(g_PAST_WHEN()),
            "PLACE_PREP" => Some(g_PLACE_PREP()),
            "PLAN_VERB_ASK" => Some(g_PLAN_VERB_ASK()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "SAID_IN_THE_PAST" => Some(g_SAID_IN_THE_PAST()),
            "SAME_THING" => Some(g_SAME_THING()),
            "SAYS_IT_AGAIN" => Some(g_SAYS_IT_AGAIN()),
            "SAYS_IT_IS_COMING" => Some(g_SAYS_IT_IS_COMING()),
            "SHORT_DATE" => Some(g_SHORT_DATE()),
            "SPELLED_COUNT" => Some(g_SPELLED_COUNT()),
            "THEIR_OWN_DEATH" => Some(g_THEIR_OWN_DEATH()),
            "THE_MEAN_NOTE" => Some(g_THE_MEAN_NOTE()),
            "TRAILING_JOINER" => Some(g_TRAILING_JOINER()),
            "TRIP_BOOKED" => Some(g_TRIP_BOOKED()),
            "TRIP_BOOKED_ONLY_WITH_A_BOOKING" => Some(g_TRIP_BOOKED_ONLY_WITH_A_BOOKING()),
            "TRIP_LINE" => Some(g_TRIP_LINE()),
            "TRIP_WORDS" => Some(g_TRIP_WORDS()),
            "UNTIL_ASK" => Some(g_UNTIL_ASK()),
            "UNTIL_A_LEARNING" => Some(g_UNTIL_A_LEARNING()),
            "UNTIL_COUNTDOWN" => Some(g_UNTIL_COUNTDOWN()),
            "UNTIL_NAMES_THE_EVENT" => Some(g_UNTIL_NAMES_THE_EVENT()),
            "UNTIL_THROWS" => Some(g_UNTIL_THROWS()),
            "WANT_LINE" => Some(g_WANT_LINE()),
            "WAS_GOING_ON" => Some(g_WAS_GOING_ON()),
            "WEEKDAYS" => Some(g_WEEKDAYS()),
            "WEEKDAY_NAME" => Some(g_WEEKDAY_NAME()),
            "WEEKDAY_NUMBER" => Some(g_WEEKDAY_NUMBER()),
            "WEEKDAY_SAID" => Some(g_WEEKDAY_SAID()),
            "WHAT_DAY_SAYS" => Some(g_WHAT_DAY_SAYS()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "WHEN_SAID" => Some(g_WHEN_SAID()),
            "WORD" => Some(g_WORD()),
            "_DAY_NAMED" => Some(g__DAY_NAMED()),
            "_MOVED" => Some(g__MOVED()),
            "_PLAN_VERB" => Some(g__PLAN_VERB()),
            "_POINTS_AT_IT" => Some(g__POINTS_AT_IT()),
            "_a_bare_unit_ask_wants_a_length" => Some(py_func("_a_bare_unit_ask_wants_a_length", |args: &[Py]| _a_bare_unit_ask_wants_a_length(args))),
            "_a_count" => Some(py_func("_a_count", |args: &[Py]| _a_count(args))),
            "_a_count_said" => Some(py_func("_a_count_said", |args: &[Py]| _a_count_said(args))),
            "_a_date_said" => Some(py_func("_a_date_said", |args: &[Py]| _a_date_said(args))),
            "_a_day_and_a_month" => Some(py_func("_a_day_and_a_month", |args: &[Py]| _a_day_and_a_month(args))),
            "_a_day_but_no_month" => Some(py_func("_a_day_but_no_month", |args: &[Py]| _a_day_but_no_month(args))),
            "_a_day_in_the_past" => Some(py_func("_a_day_in_the_past", |args: &[Py]| _a_day_in_the_past(args))),
            "_a_day_number" => Some(py_func("_a_day_number", |args: &[Py]| _a_day_number(args))),
            "_a_day_put_right" => Some(py_func("_a_day_put_right", |args: &[Py]| _a_day_put_right(args))),
            "_a_day_said" => Some(py_func("_a_day_said", |args: &[Py]| _a_day_said(args))),
            "_a_doing_word" => Some(py_func("_a_doing_word", |args: &[Py]| _a_doing_word(args))),
            "_a_length_in_words" => Some(py_func("_a_length_in_words", |args: &[Py]| _a_length_in_words(args))),
            "_a_name" => Some(py_func("_a_name", |args: &[Py]| _a_name(args))),
            "_a_plural_stem" => Some(py_func("_a_plural_stem", |args: &[Py]| _a_plural_stem(args))),
            "_a_trip_is_named" => Some(py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args))),
            "_a_year_for" => Some(py_func("_a_year_for", |args: &[Py]| _a_year_for(args))),
            "_about" => Some(py_func("_about", |args: &[Py]| _about(args))),
            "_add_months" => Some(py_func("_add_months", |args: &[Py]| _add_months(args))),
            "_an_ordinal_end" => Some(py_func("_an_ordinal_end", |args: &[Py]| _an_ordinal_end(args))),
            "_backs_off" => Some(py_func("_backs_off", |args: &[Py]| _backs_off(args))),
            "_dated" => Some(py_func("_dated", |args: &[Py]| _dated(args))),
            "_days_in" => Some(py_func("_days_in", |args: &[Py]| _days_in(args))),
            "_has_happened" => Some(py_func("_has_happened", |args: &[Py]| _has_happened(args))),
            "_how_long" => Some(py_func("_how_long", |args: &[Py]| _how_long(args))),
            "_how_long_ago" => Some(py_func("_how_long_ago", |args: &[Py]| _how_long_ago(args))),
            "_is_a_trip_line" => Some(py_func("_is_a_trip_line", |args: &[Py]| _is_a_trip_line(args))),
            "_is_an_ask" => Some(py_func("_is_an_ask", |args: &[Py]| _is_an_ask(args))),
            "_last_day" => Some(py_func("_last_day", |args: &[Py]| _last_day(args))),
            "_lead" => Some(py_func("_lead", |args: &[Py]| _lead(args))),
            "_more_than_one_day" => Some(py_func("_more_than_one_day", |args: &[Py]| _more_than_one_day(args))),
            "_names_a_place" => Some(py_func("_names_a_place", |args: &[Py]| _names_a_place(args))),
            "_names_a_thing" => Some(py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args))),
            "_names_a_thing_of_its_own" => Some(py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args))),
            "_names_it" => Some(py_func("_names_it", |args: &[Py]| _names_it(args))),
            "_names_that_event" => Some(py_func("_names_that_event", |args: &[Py]| _names_that_event(args))),
            "_no_date_yet" => Some(py_func("_no_date_yet", |args: &[Py]| _no_date_yet(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_points_back" => Some(py_func("_points_back", |args: &[Py]| _points_back(args))),
            "_put_right" => Some(py_func("_put_right", |args: &[Py]| _put_right(args))),
            "_root" => Some(py_func("_root", |args: &[Py]| _root(args))),
            "_said_before_now" => Some(py_func("_said_before_now", |args: &[Py]| _said_before_now(args))),
            "_said_in_the_last_few_turns" => Some(py_func("_said_in_the_last_few_turns", |args: &[Py]| _said_in_the_last_few_turns(args))),
            "_said_in_the_past" => Some(py_func("_said_in_the_past", |args: &[Py]| _said_in_the_past(args))),
            "_same" => Some(py_func("_same", |args: &[Py]| _same(args))),
            "_sentence" => Some(py_func("_sentence", |args: &[Py]| _sentence(args))),
            "_the_date_asked" => Some(py_func("_the_date_asked", |args: &[Py]| _the_date_asked(args))),
            "_the_date_it_means" => Some(py_func("_the_date_it_means", |args: &[Py]| _the_date_it_means(args))),
            "_the_dates_in" => Some(py_func("_the_dates_in", |args: &[Py]| _the_dates_in(args))),
            "_the_day" => Some(py_func("_the_day", |args: &[Py]| _the_day(args))),
            "_the_day_a_span_names" => Some(py_func("_the_day_a_span_names", |args: &[Py]| _the_day_a_span_names(args))),
            "_the_day_a_weekday_names" => Some(py_func("_the_day_a_weekday_names", |args: &[Py]| _the_day_a_weekday_names(args))),
            "_the_day_said" => Some(py_func("_the_day_said", |args: &[Py]| _the_day_said(args))),
            "_the_interval_it_means" => Some(py_func("_the_interval_it_means", |args: &[Py]| _the_interval_it_means(args))),
            "_the_length_from" => Some(py_func("_the_length_from", |args: &[Py]| _the_length_from(args))),
            "_the_line_put_right" => Some(py_func("_the_line_put_right", |args: &[Py]| _the_line_put_right(args))),
            "_the_lines_that_began" => Some(py_func("_the_lines_that_began", |args: &[Py]| _the_lines_that_began(args))),
            "_the_month_back" => Some(py_func("_the_month_back", |args: &[Py]| _the_month_back(args))),
            "_the_month_in_play" => Some(py_func("_the_month_in_play", |args: &[Py]| _the_month_in_play(args))),
            "_the_month_name" => Some(py_func("_the_month_name", |args: &[Py]| _the_month_name(args))),
            "_the_month_number" => Some(py_func("_the_month_number", |args: &[Py]| _the_month_number(args))),
            "_the_newest_date" => Some(py_func("_the_newest_date", |args: &[Py]| _the_newest_date(args))),
            "_the_place" => Some(py_func("_the_place", |args: &[Py]| _the_place(args))),
            "_the_thing_before_the_date" => Some(py_func("_the_thing_before_the_date", |args: &[Py]| _the_thing_before_the_date(args))),
            "_this_month_or_next" => Some(py_func("_this_month_or_next", |args: &[Py]| _this_month_or_next(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "_time_for" => Some(py_func("_time_for", |args: &[Py]| _time_for(args))),
            "_today" => Some(py_func("_today", |args: &[Py]| _today(args))),
            "_was_sentence" => Some(py_func("_was_sentence", |args: &[Py]| _was_sentence(args))),
            "_with_the_month_after" => Some(py_func("_with_the_month_after", |args: &[Py]| _with_the_month_after(args))),
            "_without_the_date" => Some(py_func("_without_the_date", |args: &[Py]| _without_the_date(args))),
            "a_plan_moved_to_a_day" => Some(py_func("a_plan_moved_to_a_day", |args: &[Py]| a_plan_moved_to_a_day(args))),
            "already_answer" => Some(py_func("already_answer", |args: &[Py]| already_answer(args))),
            "asks_what_today_is" => Some(py_func("asks_what_today_is", |args: &[Py]| asks_what_today_is(args))),
            "how_long_answer" => Some(py_func("how_long_answer", |args: &[Py]| how_long_answer(args))),
            "memory_of_one" => Some(py_func("memory_of_one", |args: &[Py]| memory_of_one(args))),
            "the_part_asked" => Some(py_func("the_part_asked", |args: &[Py]| the_part_asked(args))),
            "the_plan" => Some(py_func("the_plan", |args: &[Py]| the_plan(args))),
            "until_answer" => Some(py_func("until_answer", |args: &[Py]| until_answer(args))),
            "until_ask_matches" => Some(py_func("until_ask_matches", |args: &[Py]| until_ask_matches(args))),
            "when_answer" => Some(py_func("when_answer", |args: &[Py]| when_answer(args))),
            _ => None,
        }),
        set: None,
    }))
}

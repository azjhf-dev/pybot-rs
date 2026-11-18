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


thread_local! { static G_AGO_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGO_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_DONE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ASK_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_INTERVAL_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_ABOUT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_THE_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_THROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRIP_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_UNIT_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_YEAR_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_CALENDAR_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_UNIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEGAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVENT_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVENT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FUTURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOES_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_LONG_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_OFTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTERVAL_JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINED_TO_A_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAST_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_AFTER_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DATE_TO_GIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_DATE_YET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_PREP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_VERB_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_IT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_IT_IS_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLED_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MEAN_NOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAILING_JOINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_BOOKED_ONLY_WITH_A_BOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNTIL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNTIL_A_LEARNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNTIL_COUNTDOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNTIL_NAMES_THE_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNTIL_THROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANT_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAS_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_DAY_SAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_AT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_date_put_right: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_moved_time: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_content: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_forms: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_date_put_right: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.when: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AGO_ASK() -> Py { ensure_init(); G_AGO_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGO_ASK(v: Py) { ensure_init(); G_AGO_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGO_LENGTH() -> Py { ensure_init(); G_AGO_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGO_LENGTH(v: Py) { ensure_init(); G_AGO_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_DONE_ASK() -> Py { ensure_init(); G_ALREADY_DONE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_DONE_ASK(v: Py) { ensure_init(); G_ALREADY_DONE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK_IN_THE_PAST() -> Py { ensure_init(); G_AN_ASK_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK_IN_THE_PAST(v: Py) { ensure_init(); G_AN_ASK_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_INTERVAL_AHEAD() -> Py { ensure_init(); G_AN_INTERVAL_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_INTERVAL_AHEAD(v: Py) { ensure_init(); G_AN_INTERVAL_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_ABOUT_THE_THING() -> Py { ensure_init(); G_ASKED_ABOUT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_ABOUT_THE_THING(v: Py) { ensure_init(); G_ASKED_ABOUT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_THE_DAY() -> Py { ensure_init(); G_ASKS_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_DAY(v: Py) { ensure_init(); G_ASKS_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_THE_MONTH() -> Py { ensure_init(); G_ASKS_THE_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_MONTH(v: Py) { ensure_init(); G_ASKS_THE_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHEN() -> Py { ensure_init(); G_ASKS_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHEN(v: Py) { ensure_init(); G_ASKS_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHERE() -> Py { ensure_init(); G_ASKS_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHERE(v: Py) { ensure_init(); G_ASKS_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPENER() -> Py { ensure_init(); G_ASK_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENER(v: Py) { ensure_init(); G_ASK_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_THROWS() -> Py { ensure_init(); G_ASK_THROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_THROWS(v: Py) { ensure_init(); G_ASK_THROWS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_TAIL() -> Py { ensure_init(); G_A_COUNT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_TAIL(v: Py) { ensure_init(); G_A_COUNT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_ITSELF() -> Py { ensure_init(); G_A_DATE_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_ITSELF(v: Py) { ensure_init(); G_A_DATE_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_WORD() -> Py { ensure_init(); G_A_DATE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_WORD(v: Py) { ensure_init(); G_A_DATE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_FORM() -> Py { ensure_init(); G_A_DAY_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_FORM(v: Py) { ensure_init(); G_A_DAY_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOING() -> Py { ensure_init(); G_A_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOING(v: Py) { ensure_init(); G_A_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_IN_THE_LINE() -> Py { ensure_init(); G_A_LENGTH_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_IN_THE_LINE(v: Py) { ensure_init(); G_A_LENGTH_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED() -> Py { ensure_init(); G_A_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED(v: Py) { ensure_init(); G_A_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRIP_SAID() -> Py { ensure_init(); G_A_TRIP_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRIP_SAID(v: Py) { ensure_init(); G_A_TRIP_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_UNIT_OF_TIME() -> Py { ensure_init(); G_A_UNIT_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_UNIT_OF_TIME(v: Py) { ensure_init(); G_A_UNIT_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB() -> Py { ensure_init(); G_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB(v: Py) { ensure_init(); G_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_YEAR_SAID() -> Py { ensure_init(); G_A_YEAR_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_YEAR_SAID(v: Py) { ensure_init(); G_A_YEAR_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_TURNS() -> Py { ensure_init(); G_BACK_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_TURNS(v: Py) { ensure_init(); G_BACK_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_CALENDAR_ASK() -> Py { ensure_init(); G_BARE_CALENDAR_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_CALENDAR_ASK(v: Py) { ensure_init(); G_BARE_CALENDAR_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DAY() -> Py { ensure_init(); G_BARE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DAY(v: Py) { ensure_init(); G_BARE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_UNIT_ASK() -> Py { ensure_init(); G_BARE_UNIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_UNIT_ASK(v: Py) { ensure_init(); G_BARE_UNIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BEGAN() -> Py { ensure_init(); G_BEGAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEGAN(v: Py) { ensure_init(); G_BEGAN.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKING() -> Py { ensure_init(); G_BOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKING(v: Py) { ensure_init(); G_BOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_NOW() -> Py { ensure_init(); G_CALENDAR_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_NOW(v: Py) { ensure_init(); G_CALENDAR_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_DAYS() -> Py { ensure_init(); G_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAYS(v: Py) { ensure_init(); G_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_NUMBER() -> Py { ensure_init(); G_DAY_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_NUMBER(v: Py) { ensure_init(); G_DAY_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_PUT_RIGHT() -> Py { ensure_init(); G_DAY_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_PUT_RIGHT(v: Py) { ensure_init(); G_DAY_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_EVENT_NAME() -> Py { ensure_init(); G_EVENT_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVENT_NAME(v: Py) { ensure_init(); G_EVENT_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_EVENT_WORD() -> Py { ensure_init(); G_EVENT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVENT_WORD(v: Py) { ensure_init(); G_EVENT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_LENGTH() -> Py { ensure_init(); G_FOR_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_LENGTH(v: Py) { ensure_init(); G_FOR_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_FUTURE() -> Py { ensure_init(); G_FUTURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FUTURE(v: Py) { ensure_init(); G_FUTURE.with(|c| *c.borrow_mut() = v); }
pub fn g_GOES_TO() -> Py { ensure_init(); G_GOES_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOES_TO(v: Py) { ensure_init(); G_GOES_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_ON() -> Py { ensure_init(); G_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_ON(v: Py) { ensure_init(); G_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_LONG_ASK() -> Py { ensure_init(); G_HOW_LONG_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_ASK(v: Py) { ensure_init(); G_HOW_LONG_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_OFTEN() -> Py { ensure_init(); G_HOW_OFTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_OFTEN(v: Py) { ensure_init(); G_HOW_OFTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_INTERVAL_JOINERS() -> Py { ensure_init(); G_INTERVAL_JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTERVAL_JOINERS(v: Py) { ensure_init(); G_INTERVAL_JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINED_TO_A_DATE() -> Py { ensure_init(); G_JOINED_TO_A_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINED_TO_A_DATE(v: Py) { ensure_init(); G_JOINED_TO_A_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_LAST_LENGTH() -> Py { ensure_init(); G_LAST_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAST_LENGTH(v: Py) { ensure_init(); G_LAST_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_AFTER_THE_DAY() -> Py { ensure_init(); G_MONTH_AFTER_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_AFTER_THE_DAY(v: Py) { ensure_init(); G_MONTH_AFTER_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_NAME() -> Py { ensure_init(); G_MONTH_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_NAME(v: Py) { ensure_init(); G_MONTH_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_NAMES() -> Py { ensure_init(); G_MONTH_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_NAMES(v: Py) { ensure_init(); G_MONTH_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_NUMBER() -> Py { ensure_init(); G_MONTH_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_NUMBER(v: Py) { ensure_init(); G_MONTH_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DATE_ASK() -> Py { ensure_init(); G_NOT_A_DATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DATE_ASK(v: Py) { ensure_init(); G_NOT_A_DATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DATE_TO_GIVE() -> Py { ensure_init(); G_NOT_A_DATE_TO_GIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DATE_TO_GIVE(v: Py) { ensure_init(); G_NOT_A_DATE_TO_GIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_LENGTH_ASK() -> Py { ensure_init(); G_NOT_A_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LENGTH_ASK(v: Py) { ensure_init(); G_NOT_A_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLURAL() -> Py { ensure_init(); G_NOT_A_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLURAL(v: Py) { ensure_init(); G_NOT_A_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_IN_THE_PAST() -> Py { ensure_init(); G_NOT_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_IN_THE_PAST(v: Py) { ensure_init(); G_NOT_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_DATE_YET() -> Py { ensure_init(); G_NO_DATE_YET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_DATE_YET(v: Py) { ensure_init(); G_NO_DATE_YET.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_PLACE() -> Py { ensure_init(); G_NO_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_PLACE(v: Py) { ensure_init(); G_NO_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_WHEN() -> Py { ensure_init(); G_PAST_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_WHEN(v: Py) { ensure_init(); G_PAST_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_PREP() -> Py { ensure_init(); G_PLACE_PREP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_PREP(v: Py) { ensure_init(); G_PLACE_PREP.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_VERB_ASK() -> Py { ensure_init(); G_PLAN_VERB_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_VERB_ASK(v: Py) { ensure_init(); G_PLAN_VERB_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_IN_THE_PAST() -> Py { ensure_init(); G_SAID_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_IN_THE_PAST(v: Py) { ensure_init(); G_SAID_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_THING() -> Py { ensure_init(); G_SAME_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_THING(v: Py) { ensure_init(); G_SAME_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_IT_AGAIN() -> Py { ensure_init(); G_SAYS_IT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_AGAIN(v: Py) { ensure_init(); G_SAYS_IT_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_IT_IS_COMING() -> Py { ensure_init(); G_SAYS_IT_IS_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_IS_COMING(v: Py) { ensure_init(); G_SAYS_IT_IS_COMING.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_DATE() -> Py { ensure_init(); G_SHORT_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_DATE(v: Py) { ensure_init(); G_SHORT_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLED_COUNT() -> Py { ensure_init(); G_SPELLED_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLED_COUNT(v: Py) { ensure_init(); G_SPELLED_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_DEATH() -> Py { ensure_init(); G_THEIR_OWN_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_DEATH(v: Py) { ensure_init(); G_THEIR_OWN_DEATH.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MEAN_NOTE() -> Py { ensure_init(); G_THE_MEAN_NOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MEAN_NOTE(v: Py) { ensure_init(); G_THE_MEAN_NOTE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAILING_JOINER() -> Py { ensure_init(); G_TRAILING_JOINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAILING_JOINER(v: Py) { ensure_init(); G_TRAILING_JOINER.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_BOOKED() -> Py { ensure_init(); G_TRIP_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_BOOKED(v: Py) { ensure_init(); G_TRIP_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_BOOKED_ONLY_WITH_A_BOOKING() -> Py { ensure_init(); G_TRIP_BOOKED_ONLY_WITH_A_BOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_BOOKED_ONLY_WITH_A_BOOKING(v: Py) { ensure_init(); G_TRIP_BOOKED_ONLY_WITH_A_BOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_LINE() -> Py { ensure_init(); G_TRIP_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_LINE(v: Py) { ensure_init(); G_TRIP_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP_WORDS() -> Py { ensure_init(); G_TRIP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP_WORDS(v: Py) { ensure_init(); G_TRIP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNTIL_ASK() -> Py { ensure_init(); G_UNTIL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNTIL_ASK(v: Py) { ensure_init(); G_UNTIL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_UNTIL_A_LEARNING() -> Py { ensure_init(); G_UNTIL_A_LEARNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNTIL_A_LEARNING(v: Py) { ensure_init(); G_UNTIL_A_LEARNING.with(|c| *c.borrow_mut() = v); }
pub fn g_UNTIL_COUNTDOWN() -> Py { ensure_init(); G_UNTIL_COUNTDOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNTIL_COUNTDOWN(v: Py) { ensure_init(); G_UNTIL_COUNTDOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_UNTIL_NAMES_THE_EVENT() -> Py { ensure_init(); G_UNTIL_NAMES_THE_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNTIL_NAMES_THE_EVENT(v: Py) { ensure_init(); G_UNTIL_NAMES_THE_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_UNTIL_THROWS() -> Py { ensure_init(); G_UNTIL_THROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNTIL_THROWS(v: Py) { ensure_init(); G_UNTIL_THROWS.with(|c| *c.borrow_mut() = v); }
pub fn g_WANT_LINE() -> Py { ensure_init(); G_WANT_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANT_LINE(v: Py) { ensure_init(); G_WANT_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_WAS_GOING_ON() -> Py { ensure_init(); G_WAS_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAS_GOING_ON(v: Py) { ensure_init(); G_WAS_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAYS() -> Py { ensure_init(); G_WEEKDAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAYS(v: Py) { ensure_init(); G_WEEKDAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_NAME() -> Py { ensure_init(); G_WEEKDAY_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_NAME(v: Py) { ensure_init(); G_WEEKDAY_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_NUMBER() -> Py { ensure_init(); G_WEEKDAY_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_NUMBER(v: Py) { ensure_init(); G_WEEKDAY_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_SAID() -> Py { ensure_init(); G_WEEKDAY_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_SAID(v: Py) { ensure_init(); G_WEEKDAY_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_DAY_SAYS() -> Py { ensure_init(); G_WHAT_DAY_SAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_DAY_SAYS(v: Py) { ensure_init(); G_WHAT_DAY_SAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASK() -> Py { ensure_init(); G_WHEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); G_WHEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_SAID() -> Py { ensure_init(); G_WHEN_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_SAID(v: Py) { ensure_init(); G_WHEN_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_NAMED() -> Py { ensure_init(); G__DAY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_NAMED(v: Py) { ensure_init(); G__DAY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVED() -> Py { ensure_init(); G__MOVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVED(v: Py) { ensure_init(); G__MOVED.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAN_VERB() -> Py { ensure_init(); G__PLAN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAN_VERB(v: Py) { ensure_init(); G__PLAN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_AT_IT() -> Py { ensure_init(); G__POINTS_AT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_IT(v: Py) { ensure_init(); G__POINTS_AT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_a_date_put_right() -> Py { ensure_init(); G_a_date_put_right.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_date_put_right(v: Py) { ensure_init(); G_a_date_put_right.with(|c| *c.borrow_mut() = v); }
pub fn g_a_moved_time() -> Py { ensure_init(); G_a_moved_time.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_moved_time(v: Py) { ensure_init(); G_a_moved_time.with(|c| *c.borrow_mut() = v); }
pub fn g_content() -> Py { ensure_init(); G_content.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_content(v: Py) { ensure_init(); G_content.with(|c| *c.borrow_mut() = v); }
pub fn g_forms() -> Py { ensure_init(); G_forms.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_forms(v: Py) { ensure_init(); G_forms.with(|c| *c.borrow_mut() = v); }
pub fn g_the_date_put_right() -> Py { ensure_init(); G_the_date_put_right.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_date_put_right(v: Py) { ensure_init(); G_the_date_put_right.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A \"when ...\" follow-up, answered from what the person already told us.\n\nPeople ask about their own dates with the thing itself and no time in the ask: \"when am I\nmoving?\", \"when's my exam?\", \"when is it?\". The line that stated it is in the conversation, and\nit carries the date, so the answer is the date alone rather than the whole plan again.\n\n    i'm moving house in march            / when am i moving    -> In March.\n    my exam is in june                   / when's my exam      -> In June.\n    i have a dentist appointment on tuesday / when is it       -> On Tuesday.\n\nThe helper answers only when the ask and an earlier line are about the same thing: a word in the\nask has to be in that line (in any of its forms, so \"moving\" finds \"move\"), and the line has to\ncarry a date. When a line carries two dates - \"i'm going to japan in march and i've got a\ndentist appointment on tuesday\" - the date that belongs to the thing asked is the one just after\nit. Everything else is left to the modules that know about it.\n");
    s_DAYS(pyrt::xrefs::xget("memory.facts", "DAYS"));
    s_DETERMINERS(pyrt::xrefs::xget("memory.facts", "DETERMINERS"));
    s_FUTURE(pyrt::xrefs::xget("memory.facts", "FUTURE"));
    s_MONTHS(pyrt::xrefs::xget("memory.facts", "MONTHS"));
    s_a_date_put_right(py_func("a_date_put_right", |args: &[Py]| pyrt::xrefs::xcall("memory.facts", "a_date_put_right", args)));
    s_a_moved_time(py_func("a_moved_time", |args: &[Py]| pyrt::xrefs::xcall("memory.facts", "a_moved_time", args)));
    s_the_date_put_right(py_func("the_date_put_right", |args: &[Py]| pyrt::xrefs::xcall("memory.facts", "the_date_put_right", args)));
    s_content(py_func("content", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "content", args)));
    s_forms(py_func("forms", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "forms", args)));
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:and |so |but |oh |right |also |ok(?:ay)?[ ,]+)*(?:when(?:'s|s| is| are| was| am| do| does| did| will| would)\\b|what (?:day|date)\\b|which day\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_CALENDAR_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|oh|right|also|ok|okay)\\b[\\s,]*)?what(?:'s|s| is| was)?\\s+(?:the\\s+)?(?:day|date)(?:\\s+(?:is\\s+it|it\\s+is|is\\s+that|it|today|now|please|then|already|again))*\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_SAID(stdlib_call("re", "compile", &[py_str("\\bwhen did (?:i|we|you)\\b[^.?!]*\\b(?:say|said|tell|told|mention|mentioned|ask|asked|bring|brought|talk|talks|talked|discuss|discussed|cover|covered)\\b|\\bwhen (?:was|were) it (?:i|we) (?:said|told)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR_NOW(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|so|but|ok|okay|right|oh|also|hi|hey|quick|sorry|excuse me)\\W+)*what(?:'s|s| is| are| was)?\\s+(?:the\\s+)?(?:day|date)(?:[\\s,]*(?:is it|it is|today|now|tonight|tomorrow|this (?:week|morning|evening)|right now))*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_THROWS(call_builtin("frozenset", &[mcall(&py_str("exactly again then just roughly about long before after tonight due\nstart starts starting end ends ending say said tell told know"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_DAY_SAYS(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("date")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_NAMES(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "MONTHS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_SHORT_DATE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:for|in|on|by|until|till|around|from)\\s+(?:the\\s+)?(?:(?:early|mid|late)\\s+)?(?:"), &g_MONTH_NAMES()), &py_str("|")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "DAYS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str("|(?:0?[1-9]|[12]\\d|3[01])(?:st|nd|rd|th)?(?!\\s*(?:k|grand|thousand|hundred|million|billion|quid|pounds?|dollars?|euros?|cents?|percent|%|per\\b|a\\s+(?:week|month|year|day)))(?:\\s+(?:of\\s+)?(?:")), &g_MONTH_NAMES()), &py_str("))?)\\b|\\b(?:is|was|it's|its)\\s+(?:in\\s+|on\\s+)?(?:")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "MONTHS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DAY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:start|starts|starting|begin|begins|beginning|finish|finishes|finishing|end|ends|ending|leave|leaves|leaving|go|goes|going|come|comes|coming|return|returns|returning|fly|flies|flying|arrive|arrives|arriving|land|lands|landing|move|moves|moving|open|opens|opening|close|closes|closing|visit|visits|visiting|meet|meets|meeting|submit|submits|hand|hands|deliver|delivers|book|booked|due|held|happens|happen|is|are|was|were|have|has|got)\\s+(?:on\\s+|this\\s+|next\\s+)?(?:(?!and\\b|but\\b|because\\b|if\\b|[.!?])[\\w']+\\s+){0,3}("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "DAYS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b(?!\\s+(?:is|was|are|were)\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_WHEN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:last|back in|this past)\\s+(?:(?:early|mid|late)\\s+)?(?:"), &g_MONTH_NAMES()), &py_str("|spring|summer|autumn|winter|year)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    s_MONTH_AFTER_THE_DAY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s+(?:of\\s+)?(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "MONTHS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    s_MONTH_NAME({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MONTH_NUMBER(), "items", &[])) { let __unpk2962 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk2962[0].clone(); let mut number: Py = __unpk2962[1].clone(); __out.push((number.clone(), mcall(&name.clone(), "capitalize", &[]))); } py_dict(__out) })() });
    s_WEEKDAY_NUMBER(py_dict(vec![(py_str("monday"), Py::Int(0)), (py_str("tuesday"), Py::Int(1)), (py_str("wednesday"), Py::Int(2)), (py_str("thursday"), Py::Int(3)), (py_str("friday"), Py::Int(4)), (py_str("saturday"), Py::Int(5)), (py_str("sunday"), Py::Int(6))]));
    s_WEEKDAY_NAME({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WEEKDAY_NUMBER(), "items", &[])) { let __unpk4498 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk4498[0].clone(); let mut number: Py = __unpk4498[1].clone(); __out.push((number.clone(), mcall(&name.clone(), "capitalize", &[]))); } py_dict(__out) })() });
    fn __data1() -> Py { py_tuple(vec![py_str("early"), py_str("mid"), py_str("late"), py_str("end"), py_str("ends"), py_str("ending"), py_str("next"), py_str("this"), py_str("month"), py_str("week"), py_str("the"), py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("start"), py_str("beginning"), py_str("weekend"), py_str("a"), py_str("an"), py_str("for"), py_str("from"), py_str("to"), py_str("until"), py_str("till"), py_str("around"), py_str("about")]) }
    s_A_DATE_WORD(op_bitor(&op_bitor(&call_builtin("frozenset", &[g_MONTH_NUMBER()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_WEEKDAY_NUMBER()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e))));
    s_DAY_NUMBER(stdlib_call("re", "compile", &[py_str("^\\d{1,2}(?:st|nd|rd|th)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_ITSELF(op_bitor(&op_bitor(&call_builtin("frozenset", &[g_MONTH_NUMBER()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_WEEKDAY_NUMBER()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("early"), py_str("mid"), py_str("late"), py_str("end"), py_str("ends"), py_str("ending"), py_str("next"), py_str("this"), py_str("month"), py_str("week")])]).unwrap_or_else(|e| exc::raise(e))));
    fn __data2() -> Py { py_tuple(vec![py_str("the"), py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("a"), py_str("an"), py_str("for"), py_str("from"), py_str("to"), py_str("until"), py_str("till"), py_str("around"), py_str("about")]) }
    s_FILLER(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
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
    fn __data3() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20))]) }
    s_NUMBER_WORDS(__data3());
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
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let __unpk5718 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk5718[0].clone(); let mut spans: Py = __unpk5718[1].clone(); __out.push(pyrt::xrefs::xcall("memory.facts", "a_date_put_right", &[said.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return op_gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dated.clone()) { let __unpk9879 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk9879[0].clone(); let mut spans: Py = __unpk9879[1].clone(); if py_truthy(&spans.clone()) {  __out.push(geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_WHAT_DAY_SAYS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { _points_back(&[line.clone()]) } }) {
    dated = _dated(&[memory.clone()]);
    if py_truthy(&{ let __b = dated.clone(); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.facts", "a_date_put_right", &[geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { _said_in_the_last_few_turns(&[memory.clone(), geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(0))]) } } } }) {
    joined = if py_truthy(&op_gt(&call_builtin("len", &[dated.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { _a_day_and_a_month(&[geti(&geti(&geti(&geti(&dated.clone(), &Py::Int(0)), &Py::Int(1)), &op_neg(&Py::Int(1))), &Py::Int(0)), geti(&geti(&geti(&geti(&dated.clone(), &Py::Int(1)), &Py::Int(1)), &op_neg(&Py::Int(1))), &Py::Int(0))]) } else { py_str("") };
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
    return _sentence(&[geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))]);
    }
    }
    return Py::None;
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    trip = Py::Bool(!py_eq(&asked.clone(), &words.clone()));
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    said_before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_dated(&[memory.clone()])) { let __unpk8727 = py_unpack(&__item.clone(), 2, None); let mut said: Py = __unpk8727[0].clone(); let mut spans: Py = __unpk8727[1].clone(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()])) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() };
    anchors = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said_before.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    if py_truthy(&pyrt::xrefs::xcall("memory.facts", "a_date_put_right", &[said.clone()])) {
    put_right = { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)) } };
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
    return _sentence(&[{ let __b = _a_day_and_a_month(&[put_right.clone(), geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0))]); if py_truthy(&__b) { __b } else { { let __b = put_right.clone(); if py_truthy(&__b) { __b } else { geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)) } } } }]);
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
    while py_truthy(&{ let __b = tokens.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_THROWS(), &geti(&tokens.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&tokens.clone(), "pop", &[]);
    }
    return { let __b = call_builtin("bool", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &geti(&tokens.clone(), &op_neg(&Py::Int(1))))) } };
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
    recent = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut said: Py = __item.clone(); if py_truthy(&said.clone()) {  __out.push(said.clone()); } } py_list(__out) })() }, &op_neg(&g_BACK_TURNS()), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&op_add(&recent.clone(), &call_builtin("list", &[call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    said = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&_is_an_ask(&[said.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("list", &[mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&asked.clone()) {
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    return geti(&g_NO_DATE_YET(), &op_modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_NO_DATE_YET()]).unwrap_or_else(|e| exc::raise(e))));
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
    a_month = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[A-Za-z]+"), call_builtin("str", &[{ let __b = month.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &mcall(&word.clone(), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = the_day.clone(); if !py_truthy(&__b) { __b } else { a_month.clone() } }))) {
    return py_str("");
    }
    return op_modulo(&py_str("on the %s of %s"), &py_tuple(vec![mcall(&the_day.clone(), "group", &[Py::Int(0)]), mcall(&a_month.clone(), "lower", &[])]));
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
    return { let __b = call_builtin("bool", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &geti(&tokens.clone(), &op_neg(&Py::Int(1))))) } };
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
    return if py_truthy(&plans.clone()) { mcall(&py_str(" "), "join", &[mcall(&geti(&plans.clone(), &op_neg(&Py::Int(1))), "split", &[])]) } else { Py::None };
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
    return Py::Bool(py_contains(&slice_or_raise(&turns.clone(), &op_neg(&g_BACK_TURNS()), &Py::None, &Py::None), &key.clone()));
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
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); __kws.extend(vec![("key".to_string(), { let _start = _start.clone(); let left = left.clone(); let said = said.clone(); let spans = spans.clone(); let text = text.clone(); let when = when.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let _start = _start.clone(); let left = left.clone(); let said = said.clone(); let spans = spans.clone(); let text = text.clone(); let when = when.clone(); let word = word.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); op_neg(&geti(&span.clone(), &Py::Int(1))) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    when = __unpk2[0].clone();
    _start = __unpk2[1].clone();
    said = mcall(&said.clone(), "replace", &[when.clone(), py_str(" "), Py::Int(1)]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    cut = op_sub(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &mcall(&slice_or_raise(&turns.clone(), &Py::None, &Py::None, &op_neg(&Py::Int(1))), "index", &[key.clone()]));
    } else {
    cut = call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&cut.clone(), &op_mul(&g_BACK_TURNS(), &Py::Int(2)))]).unwrap_or_else(|e| exc::raise(e)), &cut.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&line.clone(), &key.clone())) } }) {
    continue;
    }
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(mcall(&found.clone(), "group", &[Py::Int(0)])); } py_list(__out) })() };
    let __aug2 = op_add(&spans.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SHORT_DATE(), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(mcall(&found.clone(), "group", &[Py::Int(0)])); } py_list(__out) })() });
    spans = __aug2;
    if py_truthy(&pyrt::xrefs::xcall("memory.facts", "a_date_put_right", &[line.clone()])) {
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
    if py_truthy(&Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.facts", "a_moved_time", &[meant.clone()])))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    spans = op_add(&call_builtin("list", &[mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "finditer", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); __kws.extend(vec![("key".to_string(), { let joined = joined.clone(); let last = last.clone(); let line = line.clone(); let meant = meant.clone(); let memory = memory.clone(); let note = note.clone(); let old = old.clone(); let record = record.clone(); let spans = spans.clone(); let swapped = swapped.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let joined = joined.clone(); let last = last.clone(); let line = line.clone(); let meant = meant.clone(); let memory = memory.clone(); let note = note.clone(); let old = old.clone(); let record = record.clone(); let spans = spans.clone(); let swapped = swapped.clone(); let text = text.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); py_tuple(vec![mcall(&found.clone(), "start", &[]), call_builtin("len", &[mcall(&found.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    joined = mcall(&mcall(&g_JOINED_TO_A_DATE(), "match", &[mcall(&last.clone(), "group", &[Py::Int(0)])]), "group", &[Py::Int(0)]);
    swapped = op_add(&op_add(&op_add(&slice_or_raise(&line.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &joined.clone()), &meant.clone()), &slice_or_raise(&line.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
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
    lines = op_add(&py_list(vec![fixed_sentence.clone()]), &lines.clone());
    }
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&mcall(&py_str(" "), "join", &[mcall(&line.clone(), "split", &[])]), "lower", &[])); } py_set(__out) })() };
    let __iter4 = py_iter_or_raise(&op_add(&lines.clone(), &call_builtin("list", &[call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
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
    let __iter6 = py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "finditer", &[said.clone()]));
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
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_lead(&[op_add(&py_str("on "), &mcall(&found.clone(), "group", &[Py::Int(1)]))]), mcall(&found.clone(), "start", &[Py::Int(1)])])]);
    let _ = mcall(&places.clone(), "add", &[mcall(&found.clone(), "start", &[Py::Int(1)])]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&spans.clone())); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.facts", "a_date_put_right", &[said.clone()]) } }) {
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![_a_date_said(&[pyrt::xrefs::xcall("memory.facts", "the_date_put_right", &[said.clone()])]), Py::Int(0)])]);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let fixed_sentence = fixed_sentence.clone(); let found = found.clone(); let key = key.clone(); let known = known.clone(); let latest = latest.clone(); let line = line.clone(); let lines = lines.clone(); let memory = memory.clone(); let out = out.clone(); let p = p.clone(); let places = places.clone(); let plans = plans.clone(); let replaced = replaced.clone(); let said = said.clone(); let seen = seen.clone(); let span = span.clone(); let spans = spans.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let typed = typed.clone(); py_func("lambda", move |args: &[Py]| { let fixed_sentence = fixed_sentence.clone(); let found = found.clone(); let key = key.clone(); let known = known.clone(); let latest = latest.clone(); let line = line.clone(); let lines = lines.clone(); let memory = memory.clone(); let out = out.clone(); let p = p.clone(); let places = places.clone(); let plans = plans.clone(); let replaced = replaced.clone(); let said = said.clone(); let seen = seen.clone(); let span = span.clone(); let spans = spans.clone(); let theirs = theirs.clone(); let turn = turn.clone(); let typed = typed.clone(); let span: Py = __kwp(args, 0, "span").unwrap_or(Py::None); geti(&span.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spans.clone(), "sort", args))(&__args) };
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
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEAD(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    rest = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    if py_truthy(&mcall(&mcall(&rest.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("in "), py_str("on "), py_str("at "), py_str("by "), py_str("next "), py_str("this "), py_str("last ")])])) {
    return rest.clone();
    }
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("[\\s,]"), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late")]), &first.clone())) } }) {
    return op_add(&py_str("in "), &rest.clone());
    }
    return op_add(&py_str("on "), &rest.clone());
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
    return geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0));
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
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_sub(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } } } }) {
    return Py::None;
    }
    best = Py::None;
    let __iter4 = py_iter_or_raise(&spans.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    when = __unpk5[0].clone();
    start = __unpk5[1].clone();
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hits.clone()) { let mut hit: Py = __item.clone(); if py_truthy(&op_ge(&start.clone(), &hit.clone())) {  __out.push(op_sub(&start.clone(), &hit.clone())); } } py_list(__out) })() };
    score = if py_truthy(&after.clone()) { py_tuple(vec![call_builtin("min", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0)]) } else { py_tuple(vec![call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hits.clone()) { let mut hit: Py = __item.clone(); __out.push(call_builtin("abs", &[op_sub(&start.clone(), &hit.clone())]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&score.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
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
    return op_add(&text.clone(), &mcall(&after.clone(), "group", &[Py::Int(0)]));
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
    behind = if py_truthy(&index.clone()) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DETERMINERS"), &behind.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_doing_word(&[slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)]))) } }) {
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
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been")]), &word.clone()))) {  __out.push({ let __b = mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ed"), py_str("ing")])]); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } } } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
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
    found = op_add(&call_builtin("list", &[mcall(&pyrt::xrefs::xget("memory.facts", "FUTURE"), "finditer", &[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[mcall(&g_SHORT_DATE(), "finditer", &[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)));
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
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "fullmatch", &[py_str("\\d{1,2}(?:st|nd|rd|th)?"), geti(&words.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&words.clone(), &Py::Int(1)))); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(1)), &py_str("of"))) } } } } } } } }) {
    rest = if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&words.clone(), &Py::Int(1))))) { mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]) } else { mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(2), &Py::None, &Py::None)]) };
    return op_add(&op_add(&op_add(&py_str("on the "), &geti(&words.clone(), &Py::Int(0))), &py_str(" of ")), &rest.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("early"), py_str("mid"), py_str("late")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return op_add(&py_str("in "), &phrase.clone());
    }
    return op_add(&py_str("on "), &phrase.clone());
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
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &other.clone())); if py_truthy(&__b) { __b } else { op_bitand(&pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]), &pyrt::xrefs::xcall("memory.topics", "forms", &[other.clone()])) } }) {
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
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_ge(&op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(2)) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
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
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[g_NOT_A_PLURAL()]))) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
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
    return op_add(&_a_name(&[place.clone()]), &py_str("."));
    }
    if py_truthy(&mcall(&g_ASKS_WHEN(), "search", &[ask.clone()])) {
    when = _the_date_asked(&[ask.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let __unpk4310 = py_unpack(&__item.clone(), 3, None); let mut _u: Py = __unpk4310[0].clone(); let mut _u: Py = __unpk4310[1].clone(); let mut said: Py = __unpk4310[2].clone(); __out.push(said.clone()); } py_list(__out) })() }]);
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
    title = { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&small.clone(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    turn = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&turn.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { title.clone() } } } }) {
    word = op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    out = op_add(&op_add(&slice_or_raise(&out.clone(), &Py::None, &start.clone(), &Py::None), &py_str(" ")), &slice_or_raise(&out.clone(), &end.clone(), &Py::None, &Py::None));
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_VERB()) } }) {
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
    let __iter1 = py_iter_or_raise(&py_tuple(vec![pyrt::xrefs::xget("memory.facts", "FUTURE"), g_SHORT_DATE(), g_PAST_WHEN()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let __unpk3444 = py_unpack(&__item.clone(), 3, None); let mut start: Py = __unpk3444[0].clone(); let mut end: Py = __unpk3444[1].clone(); let mut _u: Py = __unpk3444[2].clone(); __out.push({ let __b = op_lt(&mcall(&found.clone(), "start", &[]), &end.clone()); if !py_truthy(&__b) { __b } else { op_lt(&start.clone(), &mcall(&found.clone(), "end", &[])) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
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
    if py_truthy(&{ let __b = mcall(&g_ASKS_THE_MONTH(), "search", &[ask.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &pyrt::xrefs::xget("memory.facts", "MONTHS")))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_ASKS_THE_DAY(), "search", &[ask.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_bitand(&words.clone(), &pyrt::xrefs::xget("memory.facts", "DAYS")); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), plain.clone()]).unwrap_or_else(|e| exc::raise(e)) } })) } }) {
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_TRIP_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&_dated(&[memory.clone()]), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    spans = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g_NOT_A_DATE_TO_GIVE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_in_the_last_few_turns(&[memory.clone(), said.clone()]))) } }) {
    continue;
    }
    when = { let __b = if py_truthy(&asked.clone()) { _time_for(&[asked.clone(), said.clone(), spans.clone()]) } else { py_str("") }; if py_truthy(&__b) { __b } else { if py_truthy(&spans.clone()) { geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)) } else { py_str("") } } };
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    continue;
    }
    plain = mcall(&_plain(&[when.clone()]), "split", &[]);
    if py_truthy(&_has_happened(&[said.clone()])) {
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return op_add(&py_str("Yes - that was in "), &_sentence(&[when.clone()]));
    }
    said_when = _sentence(&[when.clone()]);
    return op_add(&op_add(&py_str("Yes - that was "), &mcall(&slice_or_raise(&said_when.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said_when.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return op_add(&py_str("No - that's in "), &_sentence(&[when.clone()]));
    }
    said_when = _sentence(&[when.clone()]);
    return op_add(&op_add(&py_str("No - that's "), &mcall(&slice_or_raise(&said_when.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said_when.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &geti(&plain.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &geti(&plain.clone(), &Py::Int(0)))) } } } }) {
    return op_add(&py_str("That was in "), &_sentence(&[when.clone()]));
    }
    said = _sentence(&[when.clone()]);
    return op_add(&op_add(&py_str("That was "), &mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DAYS"), &low.clone())) } }) {
    word = op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[out.clone()]), "rstrip", &[py_str(" .")]);
    return op_add(&op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
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
    said_lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&g_BACK_TURNS()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(_said_before_now(&[memory.clone(), turn.clone()])); } py_list(__out) })() };
    said_lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_lines.clone()) { let mut said: Py = __item.clone(); if py_truthy(&said.clone()) {  __out.push(said.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said_lines.clone()))) {
    return py_str("");
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNTIL_THROWS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    line_words = call_builtin("set", &[pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_GOING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&place.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &op_add(&place.clone(), &Py::Int(1))), &py_str("to"))) } } } }) {
    if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_PLACE(), &geti(&words.clone(), &op_add(&place.clone(), &Py::Int(2)))))) {
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
    let __unpk2 = py_unpack(&_the_day(&[if py_truthy(&month.clone()) { op_add(&op_add(&when.clone(), &py_str(" ")), &month.clone()) } else { when.clone() }]), 2, None);
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNTIL_THROWS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    return py_tuple(vec![geti(&geti(&spans.clone(), &op_neg(&Py::Int(1))), &Py::Int(0)), said.clone()]);
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
    named = geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_MONTH_NUMBER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(1)));
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
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(7))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), py_str("week")]);
    }
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    when_day = _a_year_for(&[today.clone(), month.clone(), day.clone()]);
    return py_tuple(vec![when_day.clone(), if py_truthy(&{ let __b = Py::Bool(py_is(&day.clone(), &Py::None)); if py_truthy(&__b) { __b } else { early.clone() } }) { py_str("month") } else { py_str("date") }]);
    }
    if py_truthy(&Py::Bool(!py_is(&weekday.clone(), &Py::None))) {
    ahead = { let __b = op_modulo(&op_sub(&weekday.clone(), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7)); if py_truthy(&__b) { __b } else { Py::Int(7) } };
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), ahead.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), py_str("date")]);
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
    return py_tuple(vec![_a_year_for(&[today.clone(), op_add(&gattr(&today.clone(), "month"), &Py::Int(1)), day.clone()]), py_str("date")]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&op_ge(&found.clone(), &today.clone())) {
    return py_tuple(vec![found.clone(), py_str("date")]);
    }
    return py_tuple(vec![_a_year_for(&[today.clone(), op_add(&gattr(&today.clone(), "month"), &Py::Int(1)), day.clone()]), py_str("date")]);
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
    while py_truthy(&op_gt(&month.clone(), &Py::Int(12))) {
    let __aug1 = op_sub(&month.clone(), &Py::Int(12));
    month = __aug1;
    today = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("year".to_string(), op_add(&gattr(&today.clone(), "year"), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&today.clone(), "replace", args))(&__args) };
    }
    day = { let __b = day.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } };
    let __iter2 = py_iter_or_raise(&py_tuple(vec![gattr(&today.clone(), "year"), op_add(&gattr(&today.clone(), "year"), &Py::Int(1))]));
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
    if py_truthy(&op_ge(&found.clone(), &today.clone())) {
    return found.clone();
    }
    }
    return stdlib_call("datetime", "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e));
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
    return gattr(&op_sub(&stdlib_call("datetime", "date", &[year.clone(), op_add(&month.clone(), &Py::Int(1)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &stdlib_call("datetime", "date", &[year.clone(), month.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))), "days");
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
    month = op_add(&gattr(&day.clone(), "month"), &months.clone());
    year = gattr(&day.clone(), "year");
    while py_truthy(&op_gt(&month.clone(), &Py::Int(12))) {
    let __aug1 = op_sub(&month.clone(), &Py::Int(12));
    month = __aug1;
    let __aug2 = op_add(&year.clone(), &Py::Int(1));
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
    days = gattr(&op_sub(&day.clone(), &today.clone()), "days");
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
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
    if py_truthy(&op_le(&days.clone(), &Py::Int(14))) {
    unit = py_str("day");
    } else {
    if py_truthy(&op_le(&days.clone(), &Py::Int(84))) {
    unit = py_str("week");
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("day")))) {
    head = op_modulo(&py_str("That's %d days away"), &days.clone());
    } else {
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    weeks = call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_le(&weeks.clone(), &Py::Int(1))) {
    head = py_str("That's just over a week away");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&named.clone(), &py_str("week"))); if py_truthy(&__b) { __b } else { op_le(&weeks.clone(), &Py::Int(30)) } }) {
    head = op_modulo(&py_str("That's about %d weeks away"), &weeks.clone());
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("month")))) {
    months = call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&op_le(&months.clone(), &Py::Int(1))) { py_str("That's about a month away") } else { op_modulo(&py_str("That's about %d months away"), &months.clone()) };
    }
    said = _the_day_said(&[day.clone(), kind.clone()]);
    return if py_truthy(&said.clone()) { op_add(&op_add(&op_add(&head.clone(), &py_str(" - ")), &said.clone()), &py_str(".")) } else { op_add(&head.clone(), &py_str(".")) };
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
    days = gattr(&op_sub(&today.clone(), &past.clone()), "days");
    if py_truthy(&op_le(&days.clone(), &Py::Int(0))) {
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
    if py_truthy(&op_le(&days.clone(), &Py::Int(14))) {
    unit = py_str("day");
    } else {
    if py_truthy(&op_le(&days.clone(), &Py::Int(84))) {
    unit = py_str("week");
    } else {
    unit = py_str("month");
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("day")))) {
    head = op_modulo(&py_str("That was %d days ago"), &days.clone());
    } else {
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    weeks = call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&op_le(&weeks.clone(), &Py::Int(1))) { py_str("That was just over a week ago") } else { op_modulo(&py_str("That was about %d weeks ago"), &weeks.clone()) };
    } else {
    months = call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    head = if py_truthy(&op_le(&months.clone(), &Py::Int(1))) { py_str("That was about a month ago") } else { op_modulo(&py_str("That was about %d months ago"), &months.clone()) };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("month")))) {
    said = op_add(&py_str("in "), &geti(&g_MONTH_NAME(), &gattr(&past.clone(), "month")));
    } else {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("end")))) {
    said = op_add(&py_str("at the end of "), &geti(&g_MONTH_NAME(), &gattr(&past.clone(), "month")));
    } else {
    said = _the_day_said(&[past.clone(), kind.clone()]);
    }
    }
    return if py_truthy(&said.clone()) { op_add(&op_add(&op_add(&head.clone(), &py_str(" - ")), &said.clone()), &py_str(".")) } else { op_add(&head.clone(), &py_str(".")) };
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
    let __iter2 = py_iter_or_raise(&py_tuple(vec![op_sub(&gattr(&day.clone(), "year"), &Py::Int(1)), op_sub(&gattr(&day.clone(), "year"), &Py::Int(2)), op_sub(&gattr(&day.clone(), "year"), &Py::Int(3))]));
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
    if py_truthy(&op_le(&found.clone(), &today.clone())) {
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
    return op_add(&py_str("sometime in "), &month.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("end")))) {
    return op_add(&py_str("the end of "), &month.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("week")))) {
    return op_add(&py_str("around "), &geti(&g_WEEKDAY_NAME(), &mcall(&day.clone(), "weekday", &[])));
    }
    return op_modulo(&py_str("%s %d %s"), &py_tuple(vec![geti(&g_WEEKDAY_NAME(), &mcall(&day.clone(), "weekday", &[])), gattr(&day.clone(), "day"), month.clone()]));
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::None;
    }
    day = mcall(&g__DAY_NAMED(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&day.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__MOVED(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&py_str("\\b(?:on|till|until)\\s+"), &stdlib_call("re", "escape", &[mcall(&day.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "content", &[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASK_THROWS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    before = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(2))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(2))) } else { Py::None };
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
    return _the_length_from(&[if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DAY_FORM(), "search", &[day.clone()])))) { op_add(&py_str("it began in "), &day.clone()) } else { op_add(&py_str("it began "), &day.clone()) }]);
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
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![said.clone(), pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()])])]);
    if py_truthy(&op_ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
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
    said_words = pyrt::xrefs::xcall("memory.topics", "content", &[said.clone()]);
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
    days = op_mul(&count.clone(), &mcall(&g_A_UNIT_OF_TIME(), "get", &[unit.clone(), Py::Int(0)]));
    said_length = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&ago.clone(), "group", &[Py::Int(0)]), "split", &[]), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    when = py_str("");
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("year")))) {
    when = call_builtin("str", &[op_sub(&gattr(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "year"), &count.clone())]).unwrap_or_else(|e| exc::raise(e));
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
    return _about(&[py_str("a year"), call_builtin("str", &[op_sub(&gattr(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "year"), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    held = mcall(&g_FOR_LENGTH(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AGO_LENGTH(), "search", &[said.clone()]))) } }) {
    count = _a_count(&[mcall(&held.clone(), "group", &[py_str("n")])]);
    unit = mcall(&mcall(&mcall(&held.clone(), "group", &[py_str("unit")]), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&{ let __b = count.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_UNIT_OF_TIME(), &unit.clone())) } }) {
    return _about(&[op_modulo(&py_str("%s %s%s"), &py_tuple(vec![_a_count_said(&[count.clone()]), unit.clone(), if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }])), py_str("")]);
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
    return _about(&[_a_length_in_words(&[gattr(&op_sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), _a_day_said(&[day.clone()])]);
    }
    weekday = mcall(&g_WEEKDAY_SAID(), "search", &[said.clone()]);
    if py_truthy(&weekday.clone()) {
    day = _the_day_a_weekday_names(&[mcall(&weekday.clone(), "group", &[py_str("day")])]);
    if py_truthy(&day.clone()) {
    return _about(&[_a_length_in_words(&[gattr(&op_sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), if py_truthy(&Py::Bool(py_eq(&gattr(&day.clone(), "day"), &Py::Int(1)))) { _the_month_name(&[gattr(&day.clone(), "month")]) } else { mcall(&mcall(&weekday.clone(), "group", &[py_str("day")]), "capitalize", &[]) }]);
    }
    }
    year = mcall(&g_A_YEAR_SAID(), "search", &[said.clone()]);
    if py_truthy(&year.clone()) {
    day = _the_day_a_span_names(&[mcall(&year.clone(), "group", &[Py::Int(0)])]);
    if py_truthy(&day.clone()) {
    return _about(&[_a_length_in_words(&[gattr(&op_sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &day.clone()), "days")]), mcall(&year.clone(), "group", &[Py::Int(0)])]);
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
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &word.clone()))) {
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
    found = stdlib_call("datetime", "date", &[op_sub(&gattr(&today.clone(), "year"), &back.clone()), month.clone(), { let __b = day.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&op_le(&found.clone(), &today.clone())) {
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
    back = op_modulo(&op_sub(&mcall(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), "weekday", &[]), &mcall(&g_WEEKDAYS(), "index", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])), &Py::Int(7));
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
    return op_sub(&stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), back.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    return py_tuple(vec![Py::Bool(true), op_add(&mcall(&order.clone(), "index", &[mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), &Py::Int(1))]);
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
    let __aug3 = op_sub(&month.clone(), &Py::Int(1));
    month = __aug3;
    if py_truthy(&op_lt(&month.clone(), &Py::Int(1))) {
    let __unpk4 = py_unpack(&py_tuple(vec![Py::Int(12), op_sub(&year.clone(), &Py::Int(1))]), 2, None);
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
    return if py_truthy(&{ let __cmp11: Py = number.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(12)) } else { Py::Bool(false) } }) { geti(&order.clone(), &op_sub(&number.clone(), &Py::Int(1))) } else { py_str("") };
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
    return op_modulo(&py_str("the %d%s of %s"), &py_tuple(vec![gattr(&day.clone(), "day"), _an_ordinal_end(&[gattr(&day.clone(), "day")]), _the_month_name(&[gattr(&day.clone(), "month")])]));
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
    if py_truthy(&{ let __cmp11: Py = op_modulo(&number.clone(), &Py::Int(100)); if py_truthy(&op_le(&Py::Int(10), &__cmp11)) { op_le(&__cmp11, &Py::Int(20)) } else { Py::Bool(false) } }) {
    return py_str("th");
    }
    return mcall(&py_dict(vec![(Py::Int(1), py_str("st")), (Py::Int(2), py_str("nd")), (Py::Int(3), py_str("rd"))]), "get", &[op_modulo(&number.clone(), &Py::Int(10)), py_str("th")]);
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
    if py_truthy(&op_lt(&days.clone(), &Py::Int(1))) {
    return py_str("a day");
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(12))) {
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![_a_count_said(&[days.clone()]), if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) { py_str("day") } else { py_str("days") }]));
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(60))) {
    return op_modulo(&py_str("%s weeks"), &_a_count_said(&[call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(7.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(730))) {
    return op_modulo(&py_str("%s months"), &_a_count_said(&[call_builtin("int", &[call_builtin("round", &[op_truediv(&days.clone(), &Py::Float(30.44))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    }
    years = op_truediv(&days.clone(), &Py::Float(365.25));
    halves = op_truediv(&call_builtin("int", &[op_mul(&years.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(2.0));
    if py_truthy(&Py::Bool(py_eq(&halves.clone(), &call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![_a_count_said(&[call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))]), if py_truthy(&Py::Bool(py_eq(&halves.clone(), &Py::Int(1)))) { py_str("year") } else { py_str("years") }]));
    }
    return op_modulo(&py_str("%s and a half years"), &_a_count_said(&[call_builtin("int", &[halves.clone()]).unwrap_or_else(|e| exc::raise(e))]));
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
    return op_modulo(&py_str("%s - since %s."), &py_tuple(vec![_tidy(&[length.clone()]), when.clone()]));
    }
    return op_modulo(&py_str("%s."), &_tidy(&[length.clone()]));
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
    return if py_truthy(&said.clone()) { op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
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
            "DAYS" => Some(g_DAYS()),
            "DAY_NUMBER" => Some(g_DAY_NUMBER()),
            "DAY_PUT_RIGHT" => Some(g_DAY_PUT_RIGHT()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "EVENT_NAME" => Some(g_EVENT_NAME()),
            "EVENT_WORD" => Some(g_EVENT_WORD()),
            "FILLER" => Some(g_FILLER()),
            "FOR_LENGTH" => Some(g_FOR_LENGTH()),
            "FUTURE" => Some(g_FUTURE()),
            "GOES_TO" => Some(g_GOES_TO()),
            "GOING_ON" => Some(g_GOING_ON()),
            "HOW_LONG_ASK" => Some(g_HOW_LONG_ASK()),
            "HOW_OFTEN" => Some(g_HOW_OFTEN()),
            "INTERVAL_JOINERS" => Some(g_INTERVAL_JOINERS()),
            "JOINED_TO_A_DATE" => Some(g_JOINED_TO_A_DATE()),
            "LAST_LENGTH" => Some(g_LAST_LENGTH()),
            "LEAD" => Some(g_LEAD()),
            "MONTHS" => Some(g_MONTHS()),
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
            "a_date_put_right" => Some(g_a_date_put_right()),
            "a_moved_time" => Some(g_a_moved_time()),
            "a_plan_moved_to_a_day" => Some(py_func("a_plan_moved_to_a_day", |args: &[Py]| a_plan_moved_to_a_day(args))),
            "already_answer" => Some(py_func("already_answer", |args: &[Py]| already_answer(args))),
            "asks_what_today_is" => Some(py_func("asks_what_today_is", |args: &[Py]| asks_what_today_is(args))),
            "content" => Some(g_content()),
            "forms" => Some(g_forms()),
            "how_long_answer" => Some(py_func("how_long_answer", |args: &[Py]| how_long_answer(args))),
            "memory_of_one" => Some(py_func("memory_of_one", |args: &[Py]| memory_of_one(args))),
            "the_date_put_right" => Some(g_the_date_put_right()),
            "the_part_asked" => Some(py_func("the_part_asked", |args: &[Py]| the_part_asked(args))),
            "the_plan" => Some(py_func("the_plan", |args: &[Py]| the_plan(args))),
            "until_answer" => Some(py_func("until_answer", |args: &[Py]| until_answer(args))),
            "until_ask_matches" => Some(py_func("until_ask_matches", |args: &[Py]| until_ask_matches(args))),
            "when_answer" => Some(py_func("when_answer", |args: &[Py]| when_answer(args))),
            "datetime" => Some(stdlib_module("datetime")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGO_ASK" => s_AGO_ASK(v),
            "AGO_LENGTH" => s_AGO_LENGTH(v),
            "ALREADY_DONE_ASK" => s_ALREADY_DONE_ASK(v),
            "AN_ASK_IN_THE_PAST" => s_AN_ASK_IN_THE_PAST(v),
            "AN_INTERVAL_AHEAD" => s_AN_INTERVAL_AHEAD(v),
            "ASKED_ABOUT_THE_THING" => s_ASKED_ABOUT_THE_THING(v),
            "ASKS_THE_DAY" => s_ASKS_THE_DAY(v),
            "ASKS_THE_MONTH" => s_ASKS_THE_MONTH(v),
            "ASKS_WHEN" => s_ASKS_WHEN(v),
            "ASKS_WHERE" => s_ASKS_WHERE(v),
            "ASK_OPENER" => s_ASK_OPENER(v),
            "ASK_THROWS" => s_ASK_THROWS(v),
            "A_COUNT_TAIL" => s_A_COUNT_TAIL(v),
            "A_DATE_ITSELF" => s_A_DATE_ITSELF(v),
            "A_DATE_WORD" => s_A_DATE_WORD(v),
            "A_DAY_FORM" => s_A_DAY_FORM(v),
            "A_GOING" => s_A_GOING(v),
            "A_LENGTH_IN_THE_LINE" => s_A_LENGTH_IN_THE_LINE(v),
            "A_THING_NAMED" => s_A_THING_NAMED(v),
            "A_TRIP_SAID" => s_A_TRIP_SAID(v),
            "A_UNIT_OF_TIME" => s_A_UNIT_OF_TIME(v),
            "A_VERB" => s_A_VERB(v),
            "A_YEAR_SAID" => s_A_YEAR_SAID(v),
            "BACK_TURNS" => s_BACK_TURNS(v),
            "BARE_CALENDAR_ASK" => s_BARE_CALENDAR_ASK(v),
            "BARE_DAY" => s_BARE_DAY(v),
            "BARE_UNIT_ASK" => s_BARE_UNIT_ASK(v),
            "BEGAN" => s_BEGAN(v),
            "BOOKING" => s_BOOKING(v),
            "CALENDAR_NOW" => s_CALENDAR_NOW(v),
            "DAYS" => s_DAYS(v),
            "DAY_NUMBER" => s_DAY_NUMBER(v),
            "DAY_PUT_RIGHT" => s_DAY_PUT_RIGHT(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "EVENT_NAME" => s_EVENT_NAME(v),
            "EVENT_WORD" => s_EVENT_WORD(v),
            "FILLER" => s_FILLER(v),
            "FOR_LENGTH" => s_FOR_LENGTH(v),
            "FUTURE" => s_FUTURE(v),
            "GOES_TO" => s_GOES_TO(v),
            "GOING_ON" => s_GOING_ON(v),
            "HOW_LONG_ASK" => s_HOW_LONG_ASK(v),
            "HOW_OFTEN" => s_HOW_OFTEN(v),
            "INTERVAL_JOINERS" => s_INTERVAL_JOINERS(v),
            "JOINED_TO_A_DATE" => s_JOINED_TO_A_DATE(v),
            "LAST_LENGTH" => s_LAST_LENGTH(v),
            "LEAD" => s_LEAD(v),
            "MONTHS" => s_MONTHS(v),
            "MONTH_AFTER_THE_DAY" => s_MONTH_AFTER_THE_DAY(v),
            "MONTH_NAME" => s_MONTH_NAME(v),
            "MONTH_NAMES" => s_MONTH_NAMES(v),
            "MONTH_NUMBER" => s_MONTH_NUMBER(v),
            "NOT_A_DATE_ASK" => s_NOT_A_DATE_ASK(v),
            "NOT_A_DATE_TO_GIVE" => s_NOT_A_DATE_TO_GIVE(v),
            "NOT_A_LENGTH_ASK" => s_NOT_A_LENGTH_ASK(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NOT_A_PLURAL" => s_NOT_A_PLURAL(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_IN_THE_PAST" => s_NOT_IN_THE_PAST(v),
            "NO_DATE_YET" => s_NO_DATE_YET(v),
            "NO_PLACE" => s_NO_PLACE(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "PAST_WHEN" => s_PAST_WHEN(v),
            "PLACE_PREP" => s_PLACE_PREP(v),
            "PLAN_VERB_ASK" => s_PLAN_VERB_ASK(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "SAID_IN_THE_PAST" => s_SAID_IN_THE_PAST(v),
            "SAME_THING" => s_SAME_THING(v),
            "SAYS_IT_AGAIN" => s_SAYS_IT_AGAIN(v),
            "SAYS_IT_IS_COMING" => s_SAYS_IT_IS_COMING(v),
            "SHORT_DATE" => s_SHORT_DATE(v),
            "SPELLED_COUNT" => s_SPELLED_COUNT(v),
            "THEIR_OWN_DEATH" => s_THEIR_OWN_DEATH(v),
            "THE_MEAN_NOTE" => s_THE_MEAN_NOTE(v),
            "TRAILING_JOINER" => s_TRAILING_JOINER(v),
            "TRIP_BOOKED" => s_TRIP_BOOKED(v),
            "TRIP_BOOKED_ONLY_WITH_A_BOOKING" => s_TRIP_BOOKED_ONLY_WITH_A_BOOKING(v),
            "TRIP_LINE" => s_TRIP_LINE(v),
            "TRIP_WORDS" => s_TRIP_WORDS(v),
            "UNTIL_ASK" => s_UNTIL_ASK(v),
            "UNTIL_A_LEARNING" => s_UNTIL_A_LEARNING(v),
            "UNTIL_COUNTDOWN" => s_UNTIL_COUNTDOWN(v),
            "UNTIL_NAMES_THE_EVENT" => s_UNTIL_NAMES_THE_EVENT(v),
            "UNTIL_THROWS" => s_UNTIL_THROWS(v),
            "WANT_LINE" => s_WANT_LINE(v),
            "WAS_GOING_ON" => s_WAS_GOING_ON(v),
            "WEEKDAYS" => s_WEEKDAYS(v),
            "WEEKDAY_NAME" => s_WEEKDAY_NAME(v),
            "WEEKDAY_NUMBER" => s_WEEKDAY_NUMBER(v),
            "WEEKDAY_SAID" => s_WEEKDAY_SAID(v),
            "WHAT_DAY_SAYS" => s_WHAT_DAY_SAYS(v),
            "WHEN_ASK" => s_WHEN_ASK(v),
            "WHEN_SAID" => s_WHEN_SAID(v),
            "WORD" => s_WORD(v),
            "_DAY_NAMED" => s__DAY_NAMED(v),
            "_MOVED" => s__MOVED(v),
            "_PLAN_VERB" => s__PLAN_VERB(v),
            "_POINTS_AT_IT" => s__POINTS_AT_IT(v),
            "a_date_put_right" => s_a_date_put_right(v),
            "a_moved_time" => s_a_moved_time(v),
            "content" => s_content(v),
            "forms" => s_forms(v),
            "the_date_put_right" => s_the_date_put_right(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.when", module_obj);
    pyrt::xrefs::reg_fn("memory.when", "_is_a_trip_line", py_func("_is_a_trip_line", |args: &[Py]| _is_a_trip_line(args)));
    pyrt::xrefs::reg_fn("memory.when", "_names_that_event", py_func("_names_that_event", |args: &[Py]| _names_that_event(args)));
    pyrt::xrefs::reg_fn("memory.when", "asks_what_today_is", py_func("asks_what_today_is", |args: &[Py]| asks_what_today_is(args)));
    pyrt::xrefs::reg_fn("memory.when", "_more_than_one_day", py_func("_more_than_one_day", |args: &[Py]| _more_than_one_day(args)));
    pyrt::xrefs::reg_fn("memory.when", "when_answer", py_func("when_answer", |args: &[Py]| when_answer(args)));
    pyrt::xrefs::reg_fn("memory.when", "_is_an_ask", py_func("_is_an_ask", |args: &[Py]| _is_an_ask(args)));
    pyrt::xrefs::reg_fn("memory.when", "_backs_off", py_func("_backs_off", |args: &[Py]| _backs_off(args)));
    pyrt::xrefs::reg_fn("memory.when", "_no_date_yet", py_func("_no_date_yet", |args: &[Py]| _no_date_yet(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_and_a_month", py_func("_a_day_and_a_month", |args: &[Py]| _a_day_and_a_month(args)));
    pyrt::xrefs::reg_fn("memory.when", "_points_back", py_func("_points_back", |args: &[Py]| _points_back(args)));
    pyrt::xrefs::reg_fn("memory.when", "the_plan", py_func("the_plan", |args: &[Py]| the_plan(args)));
    pyrt::xrefs::reg_fn("memory.when", "_said_in_the_last_few_turns", py_func("_said_in_the_last_few_turns", |args: &[Py]| _said_in_the_last_few_turns(args)));
    pyrt::xrefs::reg_fn("memory.when", "_names_a_thing", py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_thing_before_the_date", py_func("_the_thing_before_the_date", |args: &[Py]| _the_thing_before_the_date(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_line_put_right", py_func("_the_line_put_right", |args: &[Py]| _the_line_put_right(args)));
    pyrt::xrefs::reg_fn("memory.when", "_dated", py_func("_dated", |args: &[Py]| _dated(args)));
    pyrt::xrefs::reg_fn("memory.when", "_lead", py_func("_lead", |args: &[Py]| _lead(args)));
    pyrt::xrefs::reg_fn("memory.when", "_time_for", py_func("_time_for", |args: &[Py]| _time_for(args)));
    pyrt::xrefs::reg_fn("memory.when", "_with_the_month_after", py_func("_with_the_month_after", |args: &[Py]| _with_the_month_after(args)));
    pyrt::xrefs::reg_fn("memory.when", "_names_a_thing_of_its_own", py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_doing_word", py_func("_a_doing_word", |args: &[Py]| _a_doing_word(args)));
    pyrt::xrefs::reg_fn("memory.when", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("memory.when", "_put_right", py_func("_put_right", |args: &[Py]| _put_right(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_date_said", py_func("_a_date_said", |args: &[Py]| _a_date_said(args)));
    pyrt::xrefs::reg_fn("memory.when", "_same", py_func("_same", |args: &[Py]| _same(args)));
    pyrt::xrefs::reg_fn("memory.when", "_root", py_func("_root", |args: &[Py]| _root(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_plural_stem", py_func("_a_plural_stem", |args: &[Py]| _a_plural_stem(args)));
    pyrt::xrefs::reg_fn("memory.when", "the_part_asked", py_func("the_part_asked", |args: &[Py]| the_part_asked(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_place", py_func("_the_place", |args: &[Py]| _the_place(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_name", py_func("_a_name", |args: &[Py]| _a_name(args)));
    pyrt::xrefs::reg_fn("memory.when", "_without_the_date", py_func("_without_the_date", |args: &[Py]| _without_the_date(args)));
    pyrt::xrefs::reg_fn("memory.when", "_names_a_place", py_func("_names_a_place", |args: &[Py]| _names_a_place(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_dates_in", py_func("_the_dates_in", |args: &[Py]| _the_dates_in(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_date_asked", py_func("_the_date_asked", |args: &[Py]| _the_date_asked(args)));
    pyrt::xrefs::reg_fn("memory.when", "already_answer", py_func("already_answer", |args: &[Py]| already_answer(args)));
    pyrt::xrefs::reg_fn("memory.when", "_has_happened", py_func("_has_happened", |args: &[Py]| _has_happened(args)));
    pyrt::xrefs::reg_fn("memory.when", "_was_sentence", py_func("_was_sentence", |args: &[Py]| _was_sentence(args)));
    pyrt::xrefs::reg_fn("memory.when", "_sentence", py_func("_sentence", |args: &[Py]| _sentence(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_interval_it_means", py_func("_the_interval_it_means", |args: &[Py]| _the_interval_it_means(args)));
    pyrt::xrefs::reg_fn("memory.when", "_said_before_now", py_func("_said_before_now", |args: &[Py]| _said_before_now(args)));
    pyrt::xrefs::reg_fn("memory.when", "_said_in_the_past", py_func("_said_in_the_past", |args: &[Py]| _said_in_the_past(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_trip_is_named", py_func("_a_trip_is_named", |args: &[Py]| _a_trip_is_named(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_bare_unit_ask_wants_a_length", py_func("_a_bare_unit_ask_wants_a_length", |args: &[Py]| _a_bare_unit_ask_wants_a_length(args)));
    pyrt::xrefs::reg_fn("memory.when", "until_ask_matches", py_func("until_ask_matches", |args: &[Py]| until_ask_matches(args)));
    pyrt::xrefs::reg_fn("memory.when", "_today", py_func("_today", |args: &[Py]| _today(args)));
    pyrt::xrefs::reg_fn("memory.when", "until_answer", py_func("until_answer", |args: &[Py]| until_answer(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_date_it_means", py_func("_the_date_it_means", |args: &[Py]| _the_date_it_means(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_newest_date", py_func("_the_newest_date", |args: &[Py]| _the_newest_date(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_but_no_month", py_func("_a_day_but_no_month", |args: &[Py]| _a_day_but_no_month(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_month_in_play", py_func("_the_month_in_play", |args: &[Py]| _the_month_in_play(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_number", py_func("_a_day_number", |args: &[Py]| _a_day_number(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_day", py_func("_the_day", |args: &[Py]| _the_day(args)));
    pyrt::xrefs::reg_fn("memory.when", "_this_month_or_next", py_func("_this_month_or_next", |args: &[Py]| _this_month_or_next(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_year_for", py_func("_a_year_for", |args: &[Py]| _a_year_for(args)));
    pyrt::xrefs::reg_fn("memory.when", "_days_in", py_func("_days_in", |args: &[Py]| _days_in(args)));
    pyrt::xrefs::reg_fn("memory.when", "_last_day", py_func("_last_day", |args: &[Py]| _last_day(args)));
    pyrt::xrefs::reg_fn("memory.when", "_add_months", py_func("_add_months", |args: &[Py]| _add_months(args)));
    pyrt::xrefs::reg_fn("memory.when", "_how_long", py_func("_how_long", |args: &[Py]| _how_long(args)));
    pyrt::xrefs::reg_fn("memory.when", "_how_long_ago", py_func("_how_long_ago", |args: &[Py]| _how_long_ago(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_in_the_past", py_func("_a_day_in_the_past", |args: &[Py]| _a_day_in_the_past(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_day_said", py_func("_the_day_said", |args: &[Py]| _the_day_said(args)));
    pyrt::xrefs::reg_fn("memory.when", "a_plan_moved_to_a_day", py_func("a_plan_moved_to_a_day", |args: &[Py]| a_plan_moved_to_a_day(args)));
    pyrt::xrefs::reg_fn("memory.when", "how_long_answer", py_func("how_long_answer", |args: &[Py]| how_long_answer(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_put_right", py_func("_a_day_put_right", |args: &[Py]| _a_day_put_right(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_lines_that_began", py_func("_the_lines_that_began", |args: &[Py]| _the_lines_that_began(args)));
    pyrt::xrefs::reg_fn("memory.when", "_names_it", py_func("_names_it", |args: &[Py]| _names_it(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_length_from", py_func("_the_length_from", |args: &[Py]| _the_length_from(args)));
    pyrt::xrefs::reg_fn("memory.when", "memory_of_one", py_func("memory_of_one", |args: &[Py]| memory_of_one(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_count", py_func("_a_count", |args: &[Py]| _a_count(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_day_a_span_names", py_func("_the_day_a_span_names", |args: &[Py]| _the_day_a_span_names(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_day_a_weekday_names", py_func("_the_day_a_weekday_names", |args: &[Py]| _the_day_a_weekday_names(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_month_number", py_func("_the_month_number", |args: &[Py]| _the_month_number(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_month_back", py_func("_the_month_back", |args: &[Py]| _the_month_back(args)));
    pyrt::xrefs::reg_fn("memory.when", "_the_month_name", py_func("_the_month_name", |args: &[Py]| _the_month_name(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_day_said", py_func("_a_day_said", |args: &[Py]| _a_day_said(args)));
    pyrt::xrefs::reg_fn("memory.when", "_an_ordinal_end", py_func("_an_ordinal_end", |args: &[Py]| _an_ordinal_end(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_count_said", py_func("_a_count_said", |args: &[Py]| _a_count_said(args)));
    pyrt::xrefs::reg_fn("memory.when", "_a_length_in_words", py_func("_a_length_in_words", |args: &[Py]| _a_length_in_words(args)));
    pyrt::xrefs::reg_fn("memory.when", "_about", py_func("_about", |args: &[Py]| _about(args)));
    pyrt::xrefs::reg_fn("memory.when", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_access("memory.when", "AGO_ASK", g_AGO_ASK, s_AGO_ASK);
    pyrt::xrefs::reg_access("memory.when", "AGO_LENGTH", g_AGO_LENGTH, s_AGO_LENGTH);
    pyrt::xrefs::reg_access("memory.when", "ALREADY_DONE_ASK", g_ALREADY_DONE_ASK, s_ALREADY_DONE_ASK);
    pyrt::xrefs::reg_access("memory.when", "AN_ASK_IN_THE_PAST", g_AN_ASK_IN_THE_PAST, s_AN_ASK_IN_THE_PAST);
    pyrt::xrefs::reg_access("memory.when", "AN_INTERVAL_AHEAD", g_AN_INTERVAL_AHEAD, s_AN_INTERVAL_AHEAD);
    pyrt::xrefs::reg_access("memory.when", "ASKED_ABOUT_THE_THING", g_ASKED_ABOUT_THE_THING, s_ASKED_ABOUT_THE_THING);
    pyrt::xrefs::reg_access("memory.when", "ASKS_THE_DAY", g_ASKS_THE_DAY, s_ASKS_THE_DAY);
    pyrt::xrefs::reg_access("memory.when", "ASKS_THE_MONTH", g_ASKS_THE_MONTH, s_ASKS_THE_MONTH);
    pyrt::xrefs::reg_access("memory.when", "ASKS_WHEN", g_ASKS_WHEN, s_ASKS_WHEN);
    pyrt::xrefs::reg_access("memory.when", "ASKS_WHERE", g_ASKS_WHERE, s_ASKS_WHERE);
    pyrt::xrefs::reg_access("memory.when", "ASK_OPENER", g_ASK_OPENER, s_ASK_OPENER);
    pyrt::xrefs::reg_access("memory.when", "ASK_THROWS", g_ASK_THROWS, s_ASK_THROWS);
    pyrt::xrefs::reg_access("memory.when", "A_COUNT_TAIL", g_A_COUNT_TAIL, s_A_COUNT_TAIL);
    pyrt::xrefs::reg_access("memory.when", "A_DATE_ITSELF", g_A_DATE_ITSELF, s_A_DATE_ITSELF);
    pyrt::xrefs::reg_access("memory.when", "A_DATE_WORD", g_A_DATE_WORD, s_A_DATE_WORD);
    pyrt::xrefs::reg_access("memory.when", "A_DAY_FORM", g_A_DAY_FORM, s_A_DAY_FORM);
    pyrt::xrefs::reg_access("memory.when", "A_GOING", g_A_GOING, s_A_GOING);
    pyrt::xrefs::reg_access("memory.when", "A_LENGTH_IN_THE_LINE", g_A_LENGTH_IN_THE_LINE, s_A_LENGTH_IN_THE_LINE);
    pyrt::xrefs::reg_access("memory.when", "A_THING_NAMED", g_A_THING_NAMED, s_A_THING_NAMED);
    pyrt::xrefs::reg_access("memory.when", "A_TRIP_SAID", g_A_TRIP_SAID, s_A_TRIP_SAID);
    pyrt::xrefs::reg_access("memory.when", "A_UNIT_OF_TIME", g_A_UNIT_OF_TIME, s_A_UNIT_OF_TIME);
    pyrt::xrefs::reg_access("memory.when", "A_VERB", g_A_VERB, s_A_VERB);
    pyrt::xrefs::reg_access("memory.when", "A_YEAR_SAID", g_A_YEAR_SAID, s_A_YEAR_SAID);
    pyrt::xrefs::reg_access("memory.when", "BACK_TURNS", g_BACK_TURNS, s_BACK_TURNS);
    pyrt::xrefs::reg_access("memory.when", "BARE_CALENDAR_ASK", g_BARE_CALENDAR_ASK, s_BARE_CALENDAR_ASK);
    pyrt::xrefs::reg_access("memory.when", "BARE_DAY", g_BARE_DAY, s_BARE_DAY);
    pyrt::xrefs::reg_access("memory.when", "BARE_UNIT_ASK", g_BARE_UNIT_ASK, s_BARE_UNIT_ASK);
    pyrt::xrefs::reg_access("memory.when", "BEGAN", g_BEGAN, s_BEGAN);
    pyrt::xrefs::reg_access("memory.when", "BOOKING", g_BOOKING, s_BOOKING);
    pyrt::xrefs::reg_access("memory.when", "CALENDAR_NOW", g_CALENDAR_NOW, s_CALENDAR_NOW);
    pyrt::xrefs::reg_access("memory.when", "DAYS", g_DAYS, s_DAYS);
    pyrt::xrefs::reg_access("memory.when", "DAY_NUMBER", g_DAY_NUMBER, s_DAY_NUMBER);
    pyrt::xrefs::reg_access("memory.when", "DAY_PUT_RIGHT", g_DAY_PUT_RIGHT, s_DAY_PUT_RIGHT);
    pyrt::xrefs::reg_access("memory.when", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("memory.when", "EVENT_NAME", g_EVENT_NAME, s_EVENT_NAME);
    pyrt::xrefs::reg_access("memory.when", "EVENT_WORD", g_EVENT_WORD, s_EVENT_WORD);
    pyrt::xrefs::reg_access("memory.when", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("memory.when", "FOR_LENGTH", g_FOR_LENGTH, s_FOR_LENGTH);
    pyrt::xrefs::reg_access("memory.when", "FUTURE", g_FUTURE, s_FUTURE);
    pyrt::xrefs::reg_access("memory.when", "GOES_TO", g_GOES_TO, s_GOES_TO);
    pyrt::xrefs::reg_access("memory.when", "GOING_ON", g_GOING_ON, s_GOING_ON);
    pyrt::xrefs::reg_access("memory.when", "HOW_LONG_ASK", g_HOW_LONG_ASK, s_HOW_LONG_ASK);
    pyrt::xrefs::reg_access("memory.when", "HOW_OFTEN", g_HOW_OFTEN, s_HOW_OFTEN);
    pyrt::xrefs::reg_access("memory.when", "INTERVAL_JOINERS", g_INTERVAL_JOINERS, s_INTERVAL_JOINERS);
    pyrt::xrefs::reg_access("memory.when", "JOINED_TO_A_DATE", g_JOINED_TO_A_DATE, s_JOINED_TO_A_DATE);
    pyrt::xrefs::reg_access("memory.when", "LAST_LENGTH", g_LAST_LENGTH, s_LAST_LENGTH);
    pyrt::xrefs::reg_access("memory.when", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.when", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.when", "MONTH_AFTER_THE_DAY", g_MONTH_AFTER_THE_DAY, s_MONTH_AFTER_THE_DAY);
    pyrt::xrefs::reg_access("memory.when", "MONTH_NAME", g_MONTH_NAME, s_MONTH_NAME);
    pyrt::xrefs::reg_access("memory.when", "MONTH_NAMES", g_MONTH_NAMES, s_MONTH_NAMES);
    pyrt::xrefs::reg_access("memory.when", "MONTH_NUMBER", g_MONTH_NUMBER, s_MONTH_NUMBER);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_DATE_ASK", g_NOT_A_DATE_ASK, s_NOT_A_DATE_ASK);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_DATE_TO_GIVE", g_NOT_A_DATE_TO_GIVE, s_NOT_A_DATE_TO_GIVE);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_LENGTH_ASK", g_NOT_A_LENGTH_ASK, s_NOT_A_LENGTH_ASK);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_PLURAL", g_NOT_A_PLURAL, s_NOT_A_PLURAL);
    pyrt::xrefs::reg_access("memory.when", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.when", "NOT_IN_THE_PAST", g_NOT_IN_THE_PAST, s_NOT_IN_THE_PAST);
    pyrt::xrefs::reg_access("memory.when", "NO_DATE_YET", g_NO_DATE_YET, s_NO_DATE_YET);
    pyrt::xrefs::reg_access("memory.when", "NO_PLACE", g_NO_PLACE, s_NO_PLACE);
    pyrt::xrefs::reg_access("memory.when", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("memory.when", "PAST_WHEN", g_PAST_WHEN, s_PAST_WHEN);
    pyrt::xrefs::reg_access("memory.when", "PLACE_PREP", g_PLACE_PREP, s_PLACE_PREP);
    pyrt::xrefs::reg_access("memory.when", "PLAN_VERB_ASK", g_PLAN_VERB_ASK, s_PLAN_VERB_ASK);
    pyrt::xrefs::reg_access("memory.when", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.when", "SAID_IN_THE_PAST", g_SAID_IN_THE_PAST, s_SAID_IN_THE_PAST);
    pyrt::xrefs::reg_access("memory.when", "SAME_THING", g_SAME_THING, s_SAME_THING);
    pyrt::xrefs::reg_access("memory.when", "SAYS_IT_AGAIN", g_SAYS_IT_AGAIN, s_SAYS_IT_AGAIN);
    pyrt::xrefs::reg_access("memory.when", "SAYS_IT_IS_COMING", g_SAYS_IT_IS_COMING, s_SAYS_IT_IS_COMING);
    pyrt::xrefs::reg_access("memory.when", "SHORT_DATE", g_SHORT_DATE, s_SHORT_DATE);
    pyrt::xrefs::reg_access("memory.when", "SPELLED_COUNT", g_SPELLED_COUNT, s_SPELLED_COUNT);
    pyrt::xrefs::reg_access("memory.when", "THEIR_OWN_DEATH", g_THEIR_OWN_DEATH, s_THEIR_OWN_DEATH);
    pyrt::xrefs::reg_access("memory.when", "THE_MEAN_NOTE", g_THE_MEAN_NOTE, s_THE_MEAN_NOTE);
    pyrt::xrefs::reg_access("memory.when", "TRAILING_JOINER", g_TRAILING_JOINER, s_TRAILING_JOINER);
    pyrt::xrefs::reg_access("memory.when", "TRIP_BOOKED", g_TRIP_BOOKED, s_TRIP_BOOKED);
    pyrt::xrefs::reg_access("memory.when", "TRIP_BOOKED_ONLY_WITH_A_BOOKING", g_TRIP_BOOKED_ONLY_WITH_A_BOOKING, s_TRIP_BOOKED_ONLY_WITH_A_BOOKING);
    pyrt::xrefs::reg_access("memory.when", "TRIP_LINE", g_TRIP_LINE, s_TRIP_LINE);
    pyrt::xrefs::reg_access("memory.when", "TRIP_WORDS", g_TRIP_WORDS, s_TRIP_WORDS);
    pyrt::xrefs::reg_access("memory.when", "UNTIL_ASK", g_UNTIL_ASK, s_UNTIL_ASK);
    pyrt::xrefs::reg_access("memory.when", "UNTIL_A_LEARNING", g_UNTIL_A_LEARNING, s_UNTIL_A_LEARNING);
    pyrt::xrefs::reg_access("memory.when", "UNTIL_COUNTDOWN", g_UNTIL_COUNTDOWN, s_UNTIL_COUNTDOWN);
    pyrt::xrefs::reg_access("memory.when", "UNTIL_NAMES_THE_EVENT", g_UNTIL_NAMES_THE_EVENT, s_UNTIL_NAMES_THE_EVENT);
    pyrt::xrefs::reg_access("memory.when", "UNTIL_THROWS", g_UNTIL_THROWS, s_UNTIL_THROWS);
    pyrt::xrefs::reg_access("memory.when", "WANT_LINE", g_WANT_LINE, s_WANT_LINE);
    pyrt::xrefs::reg_access("memory.when", "WAS_GOING_ON", g_WAS_GOING_ON, s_WAS_GOING_ON);
    pyrt::xrefs::reg_access("memory.when", "WEEKDAYS", g_WEEKDAYS, s_WEEKDAYS);
    pyrt::xrefs::reg_access("memory.when", "WEEKDAY_NAME", g_WEEKDAY_NAME, s_WEEKDAY_NAME);
    pyrt::xrefs::reg_access("memory.when", "WEEKDAY_NUMBER", g_WEEKDAY_NUMBER, s_WEEKDAY_NUMBER);
    pyrt::xrefs::reg_access("memory.when", "WEEKDAY_SAID", g_WEEKDAY_SAID, s_WEEKDAY_SAID);
    pyrt::xrefs::reg_access("memory.when", "WHAT_DAY_SAYS", g_WHAT_DAY_SAYS, s_WHAT_DAY_SAYS);
    pyrt::xrefs::reg_access("memory.when", "WHEN_ASK", g_WHEN_ASK, s_WHEN_ASK);
    pyrt::xrefs::reg_access("memory.when", "WHEN_SAID", g_WHEN_SAID, s_WHEN_SAID);
    pyrt::xrefs::reg_access("memory.when", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.when", "_DAY_NAMED", g__DAY_NAMED, s__DAY_NAMED);
    pyrt::xrefs::reg_access("memory.when", "_MOVED", g__MOVED, s__MOVED);
    pyrt::xrefs::reg_access("memory.when", "_PLAN_VERB", g__PLAN_VERB, s__PLAN_VERB);
    pyrt::xrefs::reg_access("memory.when", "_POINTS_AT_IT", g__POINTS_AT_IT, s__POINTS_AT_IT);
    pyrt::xrefs::reg_access("memory.when", "a_date_put_right", g_a_date_put_right, s_a_date_put_right);
    pyrt::xrefs::reg_access("memory.when", "a_moved_time", g_a_moved_time, s_a_moved_time);
    pyrt::xrefs::reg_access("memory.when", "content", g_content, s_content);
    pyrt::xrefs::reg_access("memory.when", "forms", g_forms, s_forms);
    pyrt::xrefs::reg_access("memory.when", "the_date_put_right", g_the_date_put_right, s_the_date_put_right);
}

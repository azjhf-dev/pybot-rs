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


thread_local! { static G_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASIDE_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_A_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_THE_DATE_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_RANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOUNDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNTDOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FOR_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_OFTEN_THE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERIOD_IN_A_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_ON_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RELATIVE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SET_OF_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_START_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_UNIT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VALIDITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEATHER_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHEN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAREST_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_RANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRIES_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_BREAK_COLON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATED_STRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_OR_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONENESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DURATION_ADJECTIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EITHER_OR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY_AFTER_AT_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY_AFTER_TIMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY_IGNORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQ_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQ_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQ_PORTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FULL_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEANING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_NAMED_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_OF_THE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMES_A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMING_IGNORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_OPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_INTERVAL_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SET_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUM_RANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_THE_INTERVAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTCOME_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIER_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED_WORDS_SINGLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RANGE_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_FREQ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_INTERVAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEDY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEDY_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEDY_JOINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEQUENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THOUSANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THOUSANDS_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIGGER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIGGER_PATTERN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_URGENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_OR_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CUT_OFF_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CUT_OFF_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DURATION_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERIOD_OF_EXTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_MONTHS_OR_YEARS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_LENGTH_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AFTER() -> Py { ensure_init(); G_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER(v: Py) { ensure_init(); G_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_UNIT() -> Py { ensure_init(); G_AMOUNT_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_UNIT(v: Py) { ensure_init(); G_AMOUNT_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASIDE() -> Py { ensure_init(); G_ASIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASIDE(v: Py) { ensure_init(); G_ASIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASIDE_ON() -> Py { ensure_init(); G_ASIDE_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASIDE_ON(v: Py) { ensure_init(); G_ASIDE_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_UNIT() -> Py { ensure_init(); G_ASKED_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_UNIT(v: Py) { ensure_init(); G_ASKED_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_ONLY() -> Py { ensure_init(); G_ASKING_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_ONLY(v: Py) { ensure_init(); G_ASKING_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_ABOUT_IT() -> Py { ensure_init(); G_ASKS_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_IT(v: Py) { ensure_init(); G_ASKS_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_A_HAND() -> Py { ensure_init(); G_ASKS_FOR_A_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_HAND(v: Py) { ensure_init(); G_ASKS_FOR_A_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FREQUENCY() -> Py { ensure_init(); G_ASKS_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FREQUENCY(v: Py) { ensure_init(); G_ASKS_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_THE_DATE_ITSELF() -> Py { ensure_init(); G_ASKS_THE_DATE_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_DATE_ITSELF(v: Py) { ensure_init(); G_ASKS_THE_DATE_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_A_TIME() -> Py { ensure_init(); G_AT_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_A_TIME(v: Py) { ensure_init(); G_AT_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_COUNT() -> Py { ensure_init(); G_A_BARE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_COUNT(v: Py) { ensure_init(); G_A_BARE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_RANGE() -> Py { ensure_init(); G_A_BARE_RANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_RANGE(v: Py) { ensure_init(); G_A_BARE_RANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOUNDED() -> Py { ensure_init(); G_A_BOUNDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOUNDED(v: Py) { ensure_init(); G_A_BOUNDED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNTDOWN() -> Py { ensure_init(); G_A_COUNTDOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNTDOWN(v: Py) { ensure_init(); G_A_COUNTDOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FOR_A_THING() -> Py { ensure_init(); G_A_FOR_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FOR_A_THING(v: Py) { ensure_init(); G_A_FOR_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_OFTEN_THE_DOING() -> Py { ensure_init(); G_A_HOW_OFTEN_THE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_OFTEN_THE_DOING(v: Py) { ensure_init(); G_A_HOW_OFTEN_THE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH() -> Py { ensure_init(); G_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH(v: Py) { ensure_init(); G_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOMENT() -> Py { ensure_init(); G_A_MOMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOMENT(v: Py) { ensure_init(); G_A_MOMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERIOD_IN_A_LINE() -> Py { ensure_init(); G_A_PERIOD_IN_A_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERIOD_IN_A_LINE(v: Py) { ensure_init(); G_A_PERIOD_IN_A_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_ON_A_FIGURE() -> Py { ensure_init(); G_A_RATE_ON_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_ON_A_FIGURE(v: Py) { ensure_init(); G_A_RATE_ON_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RELATIVE_AFTER() -> Py { ensure_init(); G_A_RELATIVE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RELATIVE_AFTER(v: Py) { ensure_init(); G_A_RELATIVE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SET_OF_STEPS() -> Py { ensure_init(); G_A_SET_OF_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SET_OF_STEPS(v: Py) { ensure_init(); G_A_SET_OF_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAGE() -> Py { ensure_init(); G_A_STAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAGE(v: Py) { ensure_init(); G_A_STAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_START_LEAD() -> Py { ensure_init(); G_A_START_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_START_LEAD(v: Py) { ensure_init(); G_A_START_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_FOR() -> Py { ensure_init(); G_A_STATE_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_FOR(v: Py) { ensure_init(); G_A_STATE_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_UNIT_WORD() -> Py { ensure_init(); G_A_UNIT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_UNIT_WORD(v: Py) { ensure_init(); G_A_UNIT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VALIDITY_ASK() -> Py { ensure_init(); G_A_VALIDITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VALIDITY_ASK(v: Py) { ensure_init(); G_A_VALIDITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEATHER_WINDOW() -> Py { ensure_init(); G_A_WEATHER_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEATHER_WINDOW(v: Py) { ensure_init(); G_A_WEATHER_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHEN_WORD() -> Py { ensure_init(); G_A_WHEN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHEN_WORD(v: Py) { ensure_init(); G_A_WHEN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BAREST_VERDICT() -> Py { ensure_init(); G_BAREST_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAREST_VERDICT(v: Py) { ensure_init(); G_BAREST_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_RANGE() -> Py { ensure_init(); G_BARE_RANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_RANGE(v: Py) { ensure_init(); G_BARE_RANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_UNIT() -> Py { ensure_init(); G_BARE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_UNIT(v: Py) { ensure_init(); G_BARE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE() -> Py { ensure_init(); G_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE(v: Py) { ensure_init(); G_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRIES_ON() -> Py { ensure_init(); G_CARRIES_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRIES_ON(v: Py) { ensure_init(); G_CARRIES_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE() -> Py { ensure_init(); G_CAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE(v: Py) { ensure_init(); G_CAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_BREAK() -> Py { ensure_init(); G_CLAUSE_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_BREAK(v: Py) { ensure_init(); G_CLAUSE_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_BREAK_COLON() -> Py { ensure_init(); G_CLAUSE_BREAK_COLON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_BREAK_COLON(v: Py) { ensure_init(); G_CLAUSE_BREAK_COLON.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK() -> Py { ensure_init(); G_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK(v: Py) { ensure_init(); G_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKS() -> Py { ensure_init(); G_COOKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKS(v: Py) { ensure_init(); G_COOKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DATED_STRONG() -> Py { ensure_init(); G_DATED_STRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATED_STRONG(v: Py) { ensure_init(); G_DATED_STRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_OR_YEAR() -> Py { ensure_init(); G_DAY_OR_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_OR_YEAR(v: Py) { ensure_init(); G_DAY_OR_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_DONENESS() -> Py { ensure_init(); G_DONENESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONENESS(v: Py) { ensure_init(); G_DONENESS.with(|c| *c.borrow_mut() = v); }
pub fn g_DURATION_ADJECTIVE() -> Py { ensure_init(); G_DURATION_ADJECTIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DURATION_ADJECTIVE(v: Py) { ensure_init(); G_DURATION_ADJECTIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_EITHER_OR() -> Py { ensure_init(); G_EITHER_OR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EITHER_OR(v: Py) { ensure_init(); G_EITHER_OR.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPTY() -> Py { ensure_init(); G_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY(v: Py) { ensure_init(); G_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_MEASURE() -> Py { ensure_init(); G_FIGURE_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_MEASURE(v: Py) { ensure_init(); G_FIGURE_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP() -> Py { ensure_init(); G_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP(v: Py) { ensure_init(); G_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY() -> Py { ensure_init(); G_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY(v: Py) { ensure_init(); G_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY_AFTER() -> Py { ensure_init(); G_FREQUENCY_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_AFTER(v: Py) { ensure_init(); G_FREQUENCY_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY_AFTER_AT_A_TIME() -> Py { ensure_init(); G_FREQUENCY_AFTER_AT_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_AFTER_AT_A_TIME(v: Py) { ensure_init(); G_FREQUENCY_AFTER_AT_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY_AFTER_TIMES() -> Py { ensure_init(); G_FREQUENCY_AFTER_TIMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_AFTER_TIMES(v: Py) { ensure_init(); G_FREQUENCY_AFTER_TIMES.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY_BEFORE() -> Py { ensure_init(); G_FREQUENCY_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_BEFORE(v: Py) { ensure_init(); G_FREQUENCY_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY_IGNORE() -> Py { ensure_init(); G_FREQUENCY_IGNORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_IGNORE(v: Py) { ensure_init(); G_FREQUENCY_IGNORE.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQ_CLAUSE() -> Py { ensure_init(); G_FREQ_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQ_CLAUSE(v: Py) { ensure_init(); G_FREQ_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQ_LIMIT() -> Py { ensure_init(); G_FREQ_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQ_LIMIT(v: Py) { ensure_init(); G_FREQ_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQ_PORTION() -> Py { ensure_init(); G_FREQ_PORTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQ_PORTION(v: Py) { ensure_init(); G_FREQ_PORTION.with(|c| *c.borrow_mut() = v); }
pub fn g_FULL_LENGTH() -> Py { ensure_init(); G_FULL_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FULL_LENGTH(v: Py) { ensure_init(); G_FULL_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD() -> Py { ensure_init(); G_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD(v: Py) { ensure_init(); G_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_CONDITION() -> Py { ensure_init(); G_IN_A_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_CONDITION(v: Py) { ensure_init(); G_IN_A_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_RELATIVE() -> Py { ensure_init(); G_IN_A_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_RELATIVE(v: Py) { ensure_init(); G_IN_A_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINS() -> Py { ensure_init(); G_JOINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINS(v: Py) { ensure_init(); G_JOINS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS() -> Py { ensure_init(); G_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS(v: Py) { ensure_init(); G_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEANING() -> Py { ensure_init(); G_LEANING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEANING(v: Py) { ensure_init(); G_LEANING.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_FILLER() -> Py { ensure_init(); G_LENGTH_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_FILLER(v: Py) { ensure_init(); G_LENGTH_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_NAMED_DAYS() -> Py { ensure_init(); G_LENGTH_NAMED_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_NAMED_DAYS(v: Py) { ensure_init(); G_LENGTH_NAMED_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_OF_THE_JOB() -> Py { ensure_init(); G_LENGTH_OF_THE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_OF_THE_JOB(v: Py) { ensure_init(); G_LENGTH_OF_THE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS_LONG() -> Py { ensure_init(); G_MONTHS_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS_LONG(v: Py) { ensure_init(); G_MONTHS_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_BREAK() -> Py { ensure_init(); G_NAMED_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_BREAK(v: Py) { ensure_init(); G_NAMED_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMES_A_PART() -> Py { ensure_init(); G_NAMES_A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMES_A_PART(v: Py) { ensure_init(); G_NAMES_A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMING_IGNORE() -> Py { ensure_init(); G_NAMING_IGNORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMING_IGNORE(v: Py) { ensure_init(); G_NAMING_IGNORE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_OPTION() -> Py { ensure_init(); G_NOT_OPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_OPTION(v: Py) { ensure_init(); G_NOT_OPTION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_RELATIVE() -> Py { ensure_init(); G_NOT_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_RELATIVE(v: Py) { ensure_init(); G_NOT_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_INTERVAL_SAID() -> Py { ensure_init(); G_NO_INTERVAL_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_INTERVAL_SAID(v: Py) { ensure_init(); G_NO_INTERVAL_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_LENGTH() -> Py { ensure_init(); G_NO_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_LENGTH(v: Py) { ensure_init(); G_NO_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SET_TIME() -> Py { ensure_init(); G_NO_SET_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SET_TIME(v: Py) { ensure_init(); G_NO_SET_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_NUM_RANGE() -> Py { ensure_init(); G_NUM_RANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUM_RANGE(v: Py) { ensure_init(); G_NUM_RANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_THE_INTERVAL_ASK() -> Py { ensure_init(); G_ONLY_THE_INTERVAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_THE_INTERVAL_ASK(v: Py) { ensure_init(); G_ONLY_THE_INTERVAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTION() -> Py { ensure_init(); G_OPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTION(v: Py) { ensure_init(); G_OPTION.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTCOME_ASKS() -> Py { ensure_init(); G_OUTCOME_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTCOME_ASKS(v: Py) { ensure_init(); G_OUTCOME_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OVEN() -> Py { ensure_init(); G_OVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVEN(v: Py) { ensure_init(); G_OVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SAVING() -> Py { ensure_init(); G_OWN_SAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SAVING(v: Py) { ensure_init(); G_OWN_SAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK() -> Py { ensure_init(); G_PAPERWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK(v: Py) { ensure_init(); G_PAPERWORK.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE() -> Py { ensure_init(); G_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); G_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIER() -> Py { ensure_init(); G_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIER(v: Py) { ensure_init(); G_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIER_FILLER() -> Py { ensure_init(); G_QUALIFIER_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIER_FILLER(v: Py) { ensure_init(); G_QUALIFIER_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIER_WORDS() -> Py { ensure_init(); G_QUALIFIER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIER_WORDS(v: Py) { ensure_init(); G_QUALIFIER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED_WORDS() -> Py { ensure_init(); G_QUOTED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED_WORDS(v: Py) { ensure_init(); G_QUOTED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED_WORDS_SINGLE() -> Py { ensure_init(); G_QUOTED_WORDS_SINGLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED_WORDS_SINGLE(v: Py) { ensure_init(); G_QUOTED_WORDS_SINGLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RANGE_JOIN() -> Py { ensure_init(); G_RANGE_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RANGE_JOIN(v: Py) { ensure_init(); G_RANGE_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE() -> Py { ensure_init(); G_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE(v: Py) { ensure_init(); G_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_FREQ() -> Py { ensure_init(); G_RATE_FREQ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_FREQ(v: Py) { ensure_init(); G_RATE_FREQ.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_INTERVAL() -> Py { ensure_init(); G_RATE_INTERVAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_INTERVAL(v: Py) { ensure_init(); G_RATE_INTERVAL.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_LEAD() -> Py { ensure_init(); G_RATE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_LEAD(v: Py) { ensure_init(); G_RATE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEDY() -> Py { ensure_init(); G_REMEDY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEDY(v: Py) { ensure_init(); G_REMEDY.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEDY_CUT() -> Py { ensure_init(); G_REMEDY_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEDY_CUT(v: Py) { ensure_init(); G_REMEDY_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEDY_JOINER() -> Py { ensure_init(); G_REMEDY_JOINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEDY_JOINER(v: Py) { ensure_init(); G_REMEDY_JOINER.with(|c| *c.borrow_mut() = v); }
pub fn g_RESULT() -> Py { ensure_init(); G_RESULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESULT(v: Py) { ensure_init(); G_RESULT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME() -> Py { ensure_init(); G_SAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME(v: Py) { ensure_init(); G_SAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_SOMETHING() -> Py { ensure_init(); G_SAYS_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_SOMETHING(v: Py) { ensure_init(); G_SAYS_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SEQUENCE() -> Py { ensure_init(); G_SEQUENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEQUENCE(v: Py) { ensure_init(); G_SEQUENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNALS() -> Py { ensure_init(); G_SIGNALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNALS(v: Py) { ensure_init(); G_SIGNALS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPANS() -> Py { ensure_init(); G_SPANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPANS(v: Py) { ensure_init(); G_SPANS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_FILLER() -> Py { ensure_init(); G_SPAN_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_FILLER(v: Py) { ensure_init(); G_SPAN_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP() -> Py { ensure_init(); G_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP(v: Py) { ensure_init(); G_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_VERB() -> Py { ensure_init(); G_STEP_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_VERB(v: Py) { ensure_init(); G_STEP_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM() -> Py { ensure_init(); G_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM(v: Py) { ensure_init(); G_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_THOUSANDS() -> Py { ensure_init(); G_THOUSANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THOUSANDS(v: Py) { ensure_init(); G_THOUSANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THOUSANDS_JOIN() -> Py { ensure_init(); G_THOUSANDS_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THOUSANDS_JOIN(v: Py) { ensure_init(); G_THOUSANDS_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIGGER() -> Py { ensure_init(); G_TRIGGER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIGGER(v: Py) { ensure_init(); G_TRIGGER.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIGGER_PATTERN() -> Py { ensure_init(); G_TRIGGER_PATTERN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIGGER_PATTERN(v: Py) { ensure_init(); G_TRIGGER_PATTERN.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT() -> Py { ensure_init(); G_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT(v: Py) { ensure_init(); G_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_URGENT() -> Py { ensure_init(); G_URGENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_URGENT(v: Py) { ensure_init(); G_URGENT.with(|c| *c.borrow_mut() = v); }
pub fn g_WAIT() -> Py { ensure_init(); G_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAIT(v: Py) { ensure_init(); G_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_OR_CLOCK() -> Py { ensure_init(); G_WEEKDAY_OR_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_OR_CLOCK(v: Py) { ensure_init(); G_WEEKDAY_OR_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE() -> Py { ensure_init(); G_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE(v: Py) { ensure_init(); G_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_NO() -> Py { ensure_init(); G_YES_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_NO(v: Py) { ensure_init(); G_YES_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_FIGURE() -> Py { ensure_init(); G__A_BARE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_FIGURE(v: Py) { ensure_init(); G__A_BARE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CUT_OFF_AFTER() -> Py { ensure_init(); G__A_CUT_OFF_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CUT_OFF_AFTER(v: Py) { ensure_init(); G__A_CUT_OFF_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CUT_OFF_BEFORE() -> Py { ensure_init(); G__A_CUT_OFF_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CUT_OFF_BEFORE(v: Py) { ensure_init(); G__A_CUT_OFF_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DOING() -> Py { ensure_init(); G__A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DOING(v: Py) { ensure_init(); G__A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DURATION_VERB() -> Py { ensure_init(); G__A_DURATION_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DURATION_VERB(v: Py) { ensure_init(); G__A_DURATION_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERIOD_OF_EXTENT() -> Py { ensure_init(); G__A_PERIOD_OF_EXTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERIOD_OF_EXTENT(v: Py) { ensure_init(); G__A_PERIOD_OF_EXTENT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RANGE() -> Py { ensure_init(); G__A_RANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RANGE(v: Py) { ensure_init(); G__A_RANGE.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_UNIT() -> Py { ensure_init(); G__COUNTED_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_UNIT(v: Py) { ensure_init(); G__COUNTED_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_MONTHS_OR_YEARS() -> Py { ensure_init(); G__IN_MONTHS_OR_YEARS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_MONTHS_OR_YEARS(v: Py) { ensure_init(); G__IN_MONTHS_OR_YEARS.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_LENGTH_ASKS() -> Py { ensure_init(); G__STATE_LENGTH_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_LENGTH_ASKS(v: Py) { ensure_init(); G__STATE_LENGTH_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNITS() -> Py { ensure_init(); G__UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNITS(v: Py) { ensure_init(); G__UNITS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The part of an answer that a follow-up asks for.\n\nA follow-up often asks about the answer the bot has just given: after \"How do I roast a\nchicken?\", \"what temperature should the oven be?\" wants the oven temperature, not the whole\nmethod again. STYLE.md says never answer the same way twice in a row, and the full answer is\nalready on screen, so this returns the sentence, clause or number that answers the property\nasked for (\"190C (170C fan).\", \"40 to 45 minutes.\", \"20 minutes per 500g.\").\n\nIt is deliberately conservative. It only acts when the line asks for a property it knows how\nto find (time, heat, amount, money, distance, order), and it returns None rather than guess:\na sentence with several different times is handed back whole unless the question or the\nsentence says which one is meant. The caller decides whether the entry is the right one; this\nmodule only reads the answer.\n");
    s_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:oven|hob|pan|grill|fridge|freezer|microwave|slow cooker|barbecue|barbie|air fryer|steamer|skillet|airing cupboard|kettle|pot|saucepan|dish|tray|tin)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COOKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bake|baking|roast|roasting|boil|boiling|fry|frying|grill|grilling|simmer|simmering|steam|steaming|cook|cooking|reheat|reheating|warm|warming)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_URGENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:999|111|a&e|a and e|emergency|urgent|chest pain|can(?:no|'|’)t breathe|cannot breathe|difficulty breathing|stiff neck|can(?:no|'|’)t swallow|cannot swallow|unconscious|not breathing|suicid\\w*|overdos\\w*|self[- ]harm|see (?:a|your|the) (?:gp|doctor|physio|nurse|dentist|midwife|pharmacist))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS(py_tuple(vec![py_tuple(vec![py_str("money"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow much (?:does|do|is|are|will|would|should|can|could)\\b.*\\b(?:cost|costs|pay|price|spend|budget|charge|charges|fee|fees|quote|rate|rates)\\b|\\bwhat(?:'s| is) the price\\b|\\bhow much for\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("money"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|will|would|does|do|did|should)\\s+(?:that|this|it|they|those|these|the\\s+[a-z]+|a\\s+[a-z]+|an\\s+[a-z]+)(?:\\s+(?:be|going to be))?(?:\\s+[a-z]+){0,3}\\s+(?:expensive|pricey|costly|cheap)\\b|\\bhow much (?:is|was|are|were)\\s+(?:that|this|it|those|these)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("pressure"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat pressure\\b|\\bhow much pressure\\b|\\bwhat should the pressure\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("distance"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow far\\b|\\bhow many (?:miles|km|kilometres|kilometers)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("clock"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat time\\b|\\bat what time\\b|\\bwhat(?:'s| is) the time\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("time"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow long\\b|\\bhow many (?:minutes|mins|hours|hrs|days|weeks)\\b|\\bhow much time\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("heat"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which) (?:temperature|temp|temps|degrees)\\b|\\bhow (?:hot|warm)\\b|\\bwhat heat\\b|\\bwhat gas mark\\b|\\bhow (?:high|low) (?:a )?(?:heat|temperature)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow much\\b|\\bhow many\\b|\\bhow (?:big|large)\\b|\\bwhat size\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("order"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbefore (?:or|and) after\\b|\\bwhich (?:first|way round)\\b|\\bin what order\\b|\\b(?:the day before|ahead of time|in advance)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("cause"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhy\\b|\\bwhat(?:'s| is| was| could be| might be| else could be)?\\s+(?:causing|caused|causes?|cause|behind)\\b|\\bwhat(?:'s| is) the reason\\b|\\breason (?:for|behind)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("remedy"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:can|could|should|do) i (?:take|use)\\b[^?]{0,30}\\b(?:it|that|this)\\b|\\banything (?:i can|to) take for (?:it|that|this)\\b|\\bhow (?:do|can) i (?:treat|soothe|ease) (?:it|this|that)\\b|\\bwhat (?:eases|soothes|helps with) (?:it|this|that)\\b|^\\s*(?:and\\s+|so\\s+|ok\\s+|okay\\s+|but\\s+|well\\s+|right\\s+)*(?:is\\s+there\\s+(?:anything|something)|anything|(?:what|which)\\s+(?:can|could|should|shall|do|would)\\s+i)(?:\\s+i)?\\s*(?:can\\s+|could\\s+|should\\s+|shall\\s+)?(?:take|use)\\b(?:\\s+(?:for|of)\\s+(?:it|that|this|them|those|the pain|the ache))?(?:\\s+(?:instead|now|then|please|again))?\\W*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("date"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhen (?:was|were|did|does|do|is|are|had|could|can|might)\\b|\\bwhen exactly\\b|\\bwhat (?:year|date)\\b|\\bwhich year\\b|\\bhow long ago\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_DONENESS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:done|ready|cooked|baked|finished|ready to eat|is it ready)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_THE_DATE_ITSELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which)\\s+(?:year|date)\\b|\\bhow long ago\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LENGTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?P<lead>for|since|about|nearly|almost|over|going on|the last|the past|these last|the best part of)\\s+(?:(?P<amount>\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a couple of|couple of|a few|few|several|a good)\\s+)?(?P<unit>hours?|days?|weeks?|months?|years?|nights?|fortnight|weekends?|monday|tuesday|wednesday|thursday|friday|saturday|sunday|christmas|yesterday)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LENGTH_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    its it's it has is been being bin like that this the same way here still yeah yep yes well\n    honestly god ugh urgh so and but no not much pretty just only nothing change changed\n    different doing happening going on now we we've were i im ive i've of a an half\n    for since about nearly almost over going the last past these best part a an couple few\n    several good\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LENGTH_NAMED_DAYS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("christmas")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_RELATIVE(py_str("(?!\\s+(?:less|more|earlier|later|longer|shorter)\\b)"));
    s_SPAN_FILLER(py_str("\\s+(?:(?:full|whole|solid|clear)\\s+)?"));
    s_CLOCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d{1,2}\\s*(?::\\s*\\d{2}\\s*(?:am|pm|a\\.m\\.|p\\.m\\.)?|\\s*(?:am|pm|a\\.m\\.|p\\.m\\.))|\\b\\d{1,2}\\.\\d{2}\\b|\\b(?:midday|midnight|noon)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:because|since|due to|caused by|causes?|causing|usually|commonly|linked to|comes from)\\b|\\b(?:is|are|was|were)\\b[^.!?]*:|,\\s*so\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REMEDY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:paracetamol|ibuprofen|aspirin|painkillers?|pain-relief|antihistamines?|antacids?|lozenges?|gargle|gargles|honey|lemon|salt[- ]water|steam|plasters?|antiseptic|rehydration|over[- ]the[- ]counter|pharmac(?:y|ist)|plenty to drink|drink plenty|\\bdrops?\\b|drink water|anti-inflammator[^ ]*)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SYMPTOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pain|ache|aches|aching|achy|sore|hurt|hurts|hurting|ill|sick|unwell|symptom|symptoms|cold|flu|cough|coughs|coughing|throat|headache|migraine|temperature|fever|nausea|rash|rashy|itch|itches|itching|swelling|swollen|nose|ear|earache|earwax|eye|eyes|stomach|tummy|constipat\\w*|diarrhoea|hayfever|hay fever|allerg\\w*|sprain\\w*|strain\\w*|cramp|cramps|burn|burns|burnt|blister\\w*|tooth|toothache|gum|gums|verruca|hearing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REMEDY_CUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*[,;:]\\s*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REMEDY_JOINER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|but|so|then|plus|also|or|instead)\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MONTHS_LONG(py_str("(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept?|oct|nov|dec)"));
    s_WEEKDAY_OR_CLOCK(py_str("|\\b(?:(?:this|next|last|coming|bank)\\s+)?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b|\\b\\d{1,2}(?::\\d{2})?\\s*(?:am|pm)\\b"));
    s_DAY_OR_YEAR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b\\d{1,4}\\s*(?:bc|bce|ad|ce)\\b|\\b(?:1[0-9]|20[0-9])\\d{2}\\b|\\b[1-9]\\d{2}\\b|\\b(?:[1-9]|[12]\\d|3[01])(?:st|nd|rd|th)?\\s+(?:of\\s+)?"), &g_MONTHS_LONG()), &py_str("\\b|\\b")), &g_MONTHS_LONG()), &py_str("\\s+\\d{1,2}(?:st|nd|rd|th)?\\b")), &g_WEEKDAY_OR_CLOCK())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DATED_STRONG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b\\d{1,4}\\s*(?:bc|bce|ad|ce)\\b|\\b(?:[1-9]|[12]\\d|3[01])(?:st|nd|rd|th)?\\s+(?:of\\s+)?"), &g_MONTHS_LONG()), &py_str("\\b|\\b")), &g_MONTHS_LONG()), &py_str("\\s+\\d{1,2}(?:st|nd|rd|th)?\\b")), &g_WEEKDAY_OR_CLOCK())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FULL_LENGTH(py_str("(?:\\d+(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few|a couple of|a|an)\\s+(?:full|whole|entire)\\s+(?:days?|nights?|weeks?|months?|years?|hours?|minutes?)\\b"));
    s_AMOUNT_UNIT(py_str("microgrammes?|micrograms?|milligrammes?|milligrams?|millilitres?|milliliters?|centilitres?|decilitres?|grammes?|grams?|mg|mcg|\\u00b5g|g|kg|ml|l|litres?|liters?|tbsp|tsp|tablespoons?|teaspoons?|cups?|parts?|people|slices?|tins?|oz|lbs?|pounds?|tablets?|capsules?|drops?|puffs?|sprays?|units?|iu|kcal|calories|pouches?|sachets?|scoops?"));
    s_SIGNALS(py_dict(vec![(py_str("money"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("£\\s?\\d|\\b\\d[\\d,.]*\\s*(?:pounds?|quid|euros?|dollars?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("distance"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:m|km|metres?|meters?|miles?|mi)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("pressure"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:bar|psi)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("clock"), g_CLOCK()), (py_str("time"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b\\d+(?:\\.\\d+)?(?:\\s*(?:-|to|–|and)\\s*\\d+(?:\\.\\d+)?)?"), &g_SPAN_FILLER()), &py_str("(?:minutes?|mins?|hours?|hrs?|seconds?|secs?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|forty-five|fifty|sixty|seventy|eighty|ninety|twenty-four|thirty-six)")), &g_SPAN_FILLER()), &py_str("(?:minutes?|mins?|hours?|hrs?|seconds?|secs?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:an?|one|two|three|four|five|six|seven|eight|nine|ten|a couple of|a few)\\s+(?:hours?|minutes?|days?|weeks?|months?|years?)\\s+and\\s+a\\s+half\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:an?|one|two|three|four|five|six|seven|eight|nine|ten|a couple of|a few)\\s+and\\s+a\\s+half\\s+(?:hours?|minutes?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\bhalf\\s+an?\\s+(?:hour|minute|second)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:a|an|one|half an|a couple of|a few|a handful of)\\s+(?:(?:full|whole|solid|clear)\\s+)?(?:minute|second|hour|day|week|month|year)s?\\b")), &g_NOT_RELATIVE()), &py_str("|\\b\\d+(?:\\.\\d+)?\\s*[-\\u2013]\\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b\\d+\\s*hours?\\s*\\d+\\b")), &g_NOT_RELATIVE()), &py_str("|\\b")), &g_FULL_LENGTH()), &g_NOT_RELATIVE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("heat"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d{2,3}\\s*(?:°\\s*)?c\\b|\\b\\d{2,3}\\s*degrees?\\b|\\bgas mark \\d\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:"), &g_AMOUNT_UNIT()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("order"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:before|after|first|last|ahead|in advance)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("date"), g_DAY_OR_YEAR())]));
    s_SPANS(py_dict(vec![(py_str("money"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("£\\s?\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:-|to|–)\\s*£?\\s?\\d[\\d,]*(?:\\.\\d+)?)?")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("distance"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:m|km|metres?|meters?|miles?|mi)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("pressure"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:between\\s+)?\\d+(?:\\.\\d+)?(?:\\s*(?:-|to|and|–)\\s*\\d+(?:\\.\\d+)?)?\\s*(?:bar|psi)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("clock"), g_CLOCK()), (py_str("time"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b\\d+\\s*(?:months?|years?)\\s*(?:-|to|or|and|–)\\s*(?:\\d+(?:\\.\\d+)?|two|three|four|five|six|seven|eight|nine|ten|twelve|eighteen|twenty-four)\\s*(?:months?|years?)\\b|\\b\\d+\\s*hours?\\s*(?:and\\s*)?\\d+\\s*(?:minutes?|mins?)?\\b|\\b\\d+(?:\\.\\d+)?(?:\\s*(?:-|to|–|and)\\s*\\d+(?:\\.\\d+)?)?\\s*(?:minutes?|mins?|hours?|hrs?|seconds?|secs?|days?|weeks?|years?|months?|decades?|centur(?:y|ies))(?:\\s*(?:per|a)\\s*\\d*\\s*(?:g|kg|kilo|pound|lb))?\\b"), &g_NOT_RELATIVE()), &py_str("|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|forty-five|fifty|sixty|seventy|eighty|ninety)(?:\\s*(?:or|to|and|-|\\u2013)\\s*(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety))?\\s+(?:minutes?|mins?|hours?|hrs?|seconds?|secs?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:an?|one|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:hours?|minutes?|days?|weeks?|months?|years?)\\s+and\\s+a\\s+half\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:an?|one|two|three|four|five|six|seven|eight|nine|ten)\\s+and\\s+a\\s+half\\s+(?:hours?|minutes?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\bhalf\\s+an?\\s+(?:hour|minute|second)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:an|one|half an) hour\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:a|an|one)\\s+(?:second|minute|hour|day|week|month|year)\\s+or\\s+(?:two|three|four|five|six)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:a couple of|a few)\\s+(?:weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b(?:a|an|one)\\s+(?:full|whole|solid)\\s+(?:minutes?|hours?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b\\d+(?:\\.\\d+)?\\s*[-\\u2013]\\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")), &g_NOT_RELATIVE()), &py_str("|\\b")), &g_FULL_LENGTH()), &g_NOT_RELATIVE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("heat"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d{2,3}\\s*(?:°\\s*)?c\\b(?:\\s*\\(\\s*\\d{2,3}\\s*°?\\s*c\\s*fan\\s*\\))?|\\bgas mark \\d\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("amount"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:"), &g_AMOUNT_UNIT()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("cause"), g_CAUSE()), (py_str("date"), g_DAY_OR_YEAR())]));
    s_RATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bper\\b\\s*(?:kilo|kg|500g|pound|lb|person|head|each|serving|litre|liter)\\b|\\ba (?:kilo|pound|person)\\b|\\beach\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:a|an|per|each|every|once\\s+a|twice\\s+a|\\d+\\s*times\\s+a)\\s*(?:day|week|month|year|session|evening|morning|night|go|class|lesson|walk|run|fortnight|hour)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY_AFTER_TIMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*times\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:every|each|once\\s+every|once\\s+in)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY_AFTER_AT_A_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*at\\s+a\\s+time\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LENGTH_OF_THE_JOB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow long (?:does|do|will|would|is|did|should) (?:it|that|this|they|these|the)\\b|\\bhow long (?:until|before|till|til)\\b|\\bhow much time\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    register_class_method("_NoLength", "__bool__", py_func("_NoLength.__bool__", |args: &[Py]| _NoLength____bool__(args)));
    register_class_method("_NoLength", "__repr__", py_func("_NoLength.__repr__", |args: &[Py]| _NoLength____repr__(args)));
    s_NO_LENGTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _NoLength(args))(&__args) });
    s_A_COUNTDOWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow long (?:until|before|till|til)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MOMENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|fifteen|twenty|thirty|forty-five|sixty|a few|a couple of)\\s*(?:seconds?|secs?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATE_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:with|at|on|after|using|in)\\b[^,;:]{0,60},\\s*")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SET_OF_STEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:then|next|after that|finally|lastly|step \\d)\\b|\\b(?:wipe|rinse|boil|scrub|soak|coat|vacuum|hoover|descale|bleed|unblock|flush|brush|blot|vacuum|lather|scrub|polish|add|stir|mix|fill|leave it|leave them|run a|wash|clean)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SEQUENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:then|next|after that|finally|lastly|plus|followed by)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WAIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:rest|resting|cool|cooling|leave|leaving|sit|sitting|stand|standing|wait|waiting|chill|chilling|set|setting)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";\\s*|:\\s*|,\\s*(?:then|and then|and)\\s+|\\s+(?:then|and then|and)\\s+|\\s+but\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:off the heat|out of the oven|rest|resting|cool|cooling|leave|leaving|sit|sitting|stand|standing|wait|waiting|chill|chilling|set|setting)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OVEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("oven"), py_str("bake"), py_str("baked"), py_str("bakes"), py_str("baking"), py_str("roast"), py_str("roasted"), py_str("roasts"), py_str("roasting")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("oven hob grill pan pot microwave barbecue bbq steamer fryer airfryer"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:before|until|till|til|by the time)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RESULT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve|eighteen|twenty-four)(?:\\s*(?:-|to|or)\\s*(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve))?\\s*(?:days?|weeks?|months?|years?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\d+(?:[.,]\\d+)?\\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)[.!]?$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KEEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:keeps?|keeping|kept|fridge|freezer|frozen|freeze|leftovers?|shelf life|stored?|storing|use by)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUALIFIER_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("hard soft runny jammy medium rare well done fan gas top bottom thick thin large small whole skimmed semi"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUALIFIER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_QUALIFIER_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUALIFIER_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the and or but so too very quite not no more less most it its it's this that these those"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NUM_RANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:between\\s+)?-?\\d+(?:\\.\\d+)?\\s+and\\s+-?\\d+(?:\\.\\d+)?")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RANGE_JOIN(py_str("\0"));
    s_THOUSANDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=\\d),(?=\\d{3}(?:\\D|$))")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THOUSANDS_JOIN(py_str("\u{02}"));
    s_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EMPTY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the and or but if then than that this these those it its is are was were be been being\nam do does did doing done have has had having i me my mine you your yours we us our they them\ntheir he she his her one ones so too very just also as at by for from in into of off on onto\nto up with about after before again any can could should would will shall may might must not\nno yes ok okay please thanks thank there here what which who whom whose when where why how\nmuch many long need needs needed want wants like get gets got make makes made use uses used\ngood best better still only even more most less few all some same other own now\n# \"around\" and its family say a figure is approximate, not what a clause is about. Left in,\n# \"and how do i get around\" was one content word, and the sentence it named in the answer before\n# it was the one that happens to write \"around twelve to fifteen euros\" - so a question about\n# the trams was answered with the cheap-meal card.\naround roughly approximately\ntake takes took taken\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAME(py_dict(vec![]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("crisp crispy crunchy crackling")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("soft tender mushy soggy")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("juicy moist")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hot warm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold chilled cool")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quick fast quickest speedy faster")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheap cheaper cheapest budget affordable saving")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("expensive pricey costly")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("big large huge bigger")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("small little tiny smaller")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leave left leaving leaves")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same(args))(&__args) };
    s_OPTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and|what about|for)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_OPTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|do|does|did|will|would|can|could|should|shall|have|has|had|change|changes|take|takes|need|needs|use|uses)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FOLLOW_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and|also|plus|what about|how about|what if|if|then|does that|would that|will that|does it|is that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:minutes?|mins?|hours?|hrs?|seconds?|days?|weeks?|degrees?|g|kg|ml|litres?|liters?|tbsp|tsp|cups?|bar|psi|miles?|metres?|meters?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMING_IGNORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nuse using used instead recipe version way time times take takes taking need needs\nput putting cook cooking make making do doing think reckon okay ok actually really\nalso still there then already right again another one ones bit lot kind sort thing things\nworth good best better worse fine matter matters any much should shall must\nsafe saf unsafe unsaf dangerous dangerou alright normal risky sensible sensibl\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_RANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d(?:[\\d.,]*\\d)?\\s*(?:-|to|and|–)\\s*\\d(?:[\\d.,]*\\d)?\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_ABOUT_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:is|are|do|does|should|shall|can|could|will|would|must|need|have|was|were)\\b|\\?\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKING_ONLY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("how what when where which who why much many long time times take takes\ntaking keep keeps keeping last lasts stay stays good safe should does do is are was were it its\nand or the a an of in on at for to from with you we i that this these those still about"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_YES_NO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:do|does|should|shall|can|could|is|are|will|would|must|need|have)\\s+(?:i|we|you|it|they|he|she|this|that|there)\\b|\\bdo (?:i|we|you) need to\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STEP_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:do|does|should|shall|can|could|will|would|must|need|have)\\s+(?:i|we|you)\\s+(?:(?:need|have|want|got)\\s+to\\s+|be\\s+)?([a-z']+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COUNTED_UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+(?:\\.\\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a couple of|a few)\\s+(?:full\\s+|whole\\s+|solid\\s+|straight\\s+)?(seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_BREAK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";\\s+|,\\s+(?:and|but|so)\\s+|,\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PAPERWORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:passport|licen[cs]e|visa|insurance|policy|certificate|document|membership|permit|registration|warranty|guarantee|contract|tenancy|agreement|ticket|card|badge|ehic|ghic|prescription|mortgage|bond|deed|statement|mot)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_VALIDITY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhen\\s+(?:does|do|will|did)\\b[^?]{0,40}\\bexpir|\\bexpiry\\b|\\bexpiration\\b|\\bhow\\s+long\\s+(?:is|are|does|do|will)\\b[^?]{0,40}\\b(?:valid|last)\\b|\\b(?:is|are)\\b[^?]{0,40}\\b(?:still\\s+)?(?:valid|in\\s+date|out\\s+of\\s+date|expired)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OUTCOME_ASKS(py_tuple(vec![py_tuple(vec![py_str("survives"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwill (?:it|that|this|they) (?:be )?(?:alright|all right|ok|okay|fine|safe)\\b|\\bis (?:it|that|this) (?:going to be )?(?:alright|all right|ok|okay|fine|safe)\\b|\\bwill (?:it|that|this) (?:survive|recover|work again|be ruined|be knackered)\\b|\\bis (?:it|that|this) (?:ruined|knackered|a write[- ]off|done for)\\b|\\bcan (?:it|that|this) be (?:saved|fixed|repaired|salvaged)\\b|\\bis (?:it|that|this) worth (?:fixing|repairing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("serious"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis (?:that|this|it) (?:bad|serious|dangerous|normal|common)\\b|\\bhow (?:bad|serious) is (?:it|that|this)\\b|\\bshould i (?:be )?worr(?:y|ied)\\b|\\bis (?:that|this) (?:something|anything) to worry about\\b|\\bis that (?:a|normal)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("next"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat happens (?:now|next)\\b|\\bwhat next\\b|\\bwhat(?:'s| is| was) (?:the )?next\\b|\\bso what now\\b|\\bwhat now\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("explain"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat does (?:that|this|it) mean\\b|\\bwhat do you mean\\b|\\bwhat(?:'s| is) that (?:mean|mean then)\\b|\\bmeaning what\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_A_RATE_ON_A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:a|an|per|each)\\s+(year|month|week|day|hour)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERIOD_IN_A_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|per|each)\\s+(year|month|week|day|hour)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JOINS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:which|that|who|whom|whose|where|because|though|although|whereas|while|yet|however|instead|plus|as well)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STATE_LENGTH_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:last|lasts|lasted|keep|keeps|kept|stays?|stay like this|be like this|go away|goes away|clear up|clears up|get better|gets better|recover|recovers|heal|heals)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ice|icing|rest|resting|take|taking|apply|applying|use|using|hold|holding|keep|keeping|wear|wearing|put|putting|wrap|wrapping|elevate|elevating|soak|soaking|leave|leaving|rinse|rinsing|clean|cleaning|stretch|stretching|massage|massaging|drink|drinking|eat|eating|swallow|swallowing|avoid|avoiding|stop|stopping|exercise|exercising|compress|bandage|dress|dressing|change|changing|walk|walking|run|running|sit|sitting|lie|lying)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CUT_OFF_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:over|under|above|below|past|aged|beyond|around|about|from|at|by|until|till|after|before|of)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CUT_OFF_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:or\\s+(?:over|older|under|younger|more|less)|and\\s+(?:over|under)|of\\s+age)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DURATION_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:last|lasts|lasted|lasting|take|takes|took|taking|settle|settles|go|goes|went|gone|going|run|runs|ran|keeping|kept|stay|stays|stayed|heal|heals|healed|clear|clears|cleared|recover|recovers|recovered|recede|recedes|ease|eases|expires?|expiring|is|are|was|were|be|been|being)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IN_MONTHS_OR_YEARS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:month|months|year|years)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_RANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:to|or|-|and)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_BARE_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|\\d{1,3}|a|an|half)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_SET_TIME(py_str("No set time on that one - "));
    s_A_STAGE(py_tuple(vec![py_str("It's a stage rather than a set stretch: {when} is where it starts.")]));
    s_A_START_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:around|about|at|by|from|after|past)\\s+$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PERIOD_OF_EXTENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:until|till|til|from|after|by|for|about|around|up to|past|through|over|under|beyond)\\s+(?:about\\s+|around\\s+|roughly\\s+|nearly\\s+|almost\\s+|just\\s+)?$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WHEN_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:within|by|in|on|at|from|after|before|until|till|til|since|every|each|during|over|under|about|around|past|for|of)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_FREQUENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:often|regularly|frequently)\\b|\\bhow many times\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bonce (?:a|an|every|per|or twice|in a)\\b|\\b(?:twice|thrice|nightly|daily|weekly|fortnightly|monthly|yearly|annually)\\b|\\b(?:two|three|four|five|six|seven|eight|ten|twelve) (?:times|sessions|days|nights|weeks|months|years) (?:a|per|each)\\b|\\b(?:two|three|four) (?:or|to) (?:three|four|five) (?:times|sessions|days|nights)\\b|\\bevery (?:(?:couple|few|other|one|two|three|four|six|twelve|\\d+)\\s+(?:of\\s+)?)?(?:day|week|month|year|morning|evening|night|fortnight)s?|\\bnot on a fixed\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQUENCY_IGNORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\noften regular regularly frequent frequently times time many much usually normally should\nshall need needs want wants how do does did i we you they it them the a an of for in on at\nwinter summer spring autumn fall morning evening night day week month year now\nand but so then or also plus because while which\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKED_UNIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many\\s+(seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__UNITS(py_dict(vec![(py_str("second"), py_str("second")), (py_str("sec"), py_str("second")), (py_str("minute"), py_str("minute")), (py_str("min"), py_str("minute")), (py_str("hour"), py_str("hour")), (py_str("hr"), py_str("hour")), (py_str("day"), py_str("day")), (py_str("week"), py_str("week")), (py_str("month"), py_str("month")), (py_str("year"), py_str("year"))]));
    s_RATE_FREQ({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|per|each) (?:day|week|month|year)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQ_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";\\s*|(?<=[.!?])\\s+|,\\s+(?:and|but|so|then)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRIGGER_PATTERN(py_str("\\b(?:whenever|when|as soon as|if)\\s+(?:it\\s+|they\\s+|you\\s+)?(?:needs?|starts?|looks?|furs?|smells?|sounds?|shows?|gets?|builds?|feels?|goes?|is\\b)"));
    s_TRIGGER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TRIGGER_PATTERN()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATE_INTERVAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\bonce (?:a|an|every|per|or twice|in a)\\b|\\b(?:twice|thrice)\\s+(?:a|an|per|each|every|daily|weekly|monthly|yearly|nightly)\\b|\\b(?:daily|weekly|fortnightly|monthly|yearly|annually|nightly)\\b|\\b(?:two|three|four|five|six|seven|eight|ten|twelve) (?:times|sessions|days|nights|weeks|months|years) (?:a|per|each)\\b|\\bevery (?:other )?(?:day|week|fortnight|month|year|morning|evening|few|couple|\\d+)\\b|\\bevery (?:month|week|day|year) or (?:two|so)\\b"), &py_str("|")), &g_TRIGGER_PATTERN())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BARE_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:once|twice|thrice|(?:two|three|four|five|ten) times)\\b(?!\\s*(?:a|an|per|each|every|daily|weekly|monthly|yearly|nightly|or\\b|in a\\b))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ONLY_THE_INTERVAL_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and |also |so |ok |okay |oh |right )*(?:how (?:often|regularly|frequently)|how many times)(?:\\s+(?:should|do|would|must|need|needs|shall)\\s+(?:i|we|you))?(?:\\s+(?:i|we)\\s+(?:need|have|ought)\\s+to)?(?:\\s+do(?:es)?\\s+(?:that|it|this|so))?(?:\\s+(?:do|does)\\s+(?:i|we)\\s+need\\s+to)?(?:\\s+(?:it|that|this|them|those))?(?:\\s+(?:again|then|now|really))?\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_INTERVAL_SAID(py_str("The steps don't say how often, and I'd rather not make a number up - go by when it needs doing again rather than by a calendar."));
    s_A_HOW_OFTEN_THE_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:how often|how many times|how regularly|how frequently)\\b.*\\b(?:it|that|this|them|those)(?:\\s+(?:up|out|down|off|in|on|round|around|over|through|away|back|again|properly|too))?\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQ_LIMIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:never|no more than|not more than|at most|at least|less than|more than|up to|until|before)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREQ_PORTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+\\s*(?:g|kg|ml|cl|l|oz)\\b|\\b(?:pouches?|grams?|tins?|cans?|packets?|litres?|liters?|sachets?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_FOR_A_HAND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tips?|tricks?|hacks?|advice|pointers?|suggestions?|ideas?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMES_A_PART(Py::Int(2));
    s_OWN_SAVING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:start|begin|keep|stop|try|need|want)\\s+(?:to\\s+)?sav(?:e|ing)\\b|\\bsav(?:e|ing)\\s+(?:up\\b|money\\b|for\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAYS_SOMETHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and |then |so )?(?:say|tell|ask|text|message|write|word|phrase|reply|mention|bring|raise|broach)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BAREST_VERDICT(Py::Int(24));
    s_QUOTED_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\"[^\"]{10,}\"")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUOTED_WORDS_SINGLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?::\\s|,\\s)'(.{10,}?[.?!])'(?=\\s|$)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DURATION_ADJECTIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\d+(?:\\.\\d+)?)\\s*[-–]\\s*(minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("until till til whole thing things"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARRIES_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and|so|also|then|but|ok|okay|right|plus|same|also|what about|how about|what of)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EITHER_OR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d[^.;]{0,40}\\bor\\b[^.;]{0,40}\\d")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIGURE_MEASURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+(?:\\.\\d+)?\\s*(?:to\\s*\\d+(?:\\.\\d+)?\\s*)?[-\\u2013]?\\s*([a-z%]+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_UNIT_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BARE_RANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:-|to|or|and|\\u2013)\\s*\\d+(?:\\.\\d+)?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FOR_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?)?\\s*for\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IN_A_RELATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:who|which|whose)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WEATHER_WINDOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:forecast|rain|showers?|dry|damp|frost|wind|sunshine?|weather|mild|warm|cold)\\b[^.;:,]{0,25}\\bfor\\s+$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AT_A_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:at a time|to warm up|to cool|of it)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IN_A_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:if|unless|when|whenever|once)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BOUNDED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:more than|less than|at least|at most|up to|within|over|under|no more than|not more than)\\s+$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_STATE_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gone on|been|lasted|going on|had (?:it|this|that))\\b[^.;:]{0,30}\\bfor\\s+$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_RELATIVE_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:before|after)\\s+(?!you\\b|i\\b|we\\b|they\\b|it\\b|he\\b|she\\b|\\w+ing\\b)\\w")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASIDE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+-(?!\\s*-)[^-;.!?]{1,200}?-\\s+|[\\u2014\\u2013][^\\u2014\\u2013]{1,200}?[\\u2014\\u2013]|\\([^()]{1,200}\\)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASIDE_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?:"), &gattr(&g_ASIDE(), "pattern")), &py_str(")(?:(?:and|but|so)\\s+|,\\s*|;\\s*)?"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HELD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\x01(\\d+)\\x01")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED_BREAK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";\\s*|(?<=[.!?])\\s+|,\\s+(?:and|but|so)\\s+|\\s+(?:and|but|so)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEANING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:or|and|but|so|plus|also)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_BREAK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:;\\s*|,(?!\\d)\\s*|\\s+(?:and|but|so)\\s+)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_BREAK_COLON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:;\\s*|,(?!\\d)\\s*|\\s+(?:and|but|so)\\s+|:\\s*)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn only_a_length(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line says nothing but how long the thing has been going.\n\n    \"its been like that for a week\" after an answer is the person telling that answer how long\n    it has had, and it is not a new ask: the episode it belongs to is the answer, and the reply\n    that fits is the question a length asks. Read here rather than in one module, because both\n    the store's own reply and the knowledge base's repeat handling need the same reading, and\n    neither may answer such a line twice.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LENGTH(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "start", args))(&__args) }, &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None)), "replace", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LENGTH_FILLER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_remedy_in(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut s: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The clause of a symptom answer that says what to take, or None when there is none.\n\n    The remedy is rarely the answer's opening. \"Most back pain is muscular and settles in two to\n    six weeks ... and take paracetamol or ibuprofen if you can\" gives what to take at the end of\n    its longest sentence, and that clause - not the paragraph it sits in - is the answer to\n    \"what can i take\". A sentence that names a remedy in three words takes the clause after it,\n    because \"Paracetamol or ibuprofen\" on its own says the medicine and nothing about it.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEDY(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SYMPTOM(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "split", args))(&__args) }) { let mut s: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }); } } py_list(__out) })() });
    for __item in __iter1 {
    sentence = __item.clone();
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEDY_CUT(), "split", args))(&__args) }) { let mut piece: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    piece = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEDY(), "search", args))(&__args) }))) {
    continue;
    }
    kept = py_list(vec![piece.clone()]);
    while py_truthy(&{ let __b = op_lt(&op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&pieces.clone(), &op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEDY_JOINER(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "count", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(")")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "count", args))(&__args) })) } }) {
    return part.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn carries(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut span: Py = Py::None;
    let _ = py_str("Whether an answer holds the part a line of this kind asks for.\n\n    A figure is the part for money, a time or a date; a remedy is a sentence that names one, so\n    it is read by its own words rather than by a number. A caller that only needs to know\n    whether handing this answer back would answer the line uses this, and the answer itself is\n    still given in full or under a lead-in.\n    ");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("remedy")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMEDY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    span = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = kind.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPANS(), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = span.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&span.clone(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn has_a_date(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a sentence gives a date, counting a bare number only when it is not money.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DATED_STRONG(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SPANS(), &py_str("money")), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SPANS(), &py_str("date")), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn pieces_of(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut guarded: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("`text` cut into pieces on `pattern`, with a numeric range and a thousands comma kept whole.");
    guarded = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let guarded = guarded.clone(); let pattern = pattern.clone(); let piece = piece.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let guarded = guarded.clone(); let pattern = pattern.clone(); let piece = piece.clone(); let text = text.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" and ")); __args.push(g_RANGE_JOIN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "replace", args))(&__args) } }) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NUM_RANGE(), "sub", args))(&__args) };
    guarded = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THOUSANDS_JOIN()); __args.push(guarded.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THOUSANDS(), "sub", args))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(guarded.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut piece: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THOUSANDS_JOIN()); __args.push(py_str(",")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_RANGE_JOIN()); __args.push(py_str(" and ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() };
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
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut group: Py = Py::None;
    let mut word: Py = Py::None;
    group = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&group.clone());
    for __item in __iter1 {
    word = __item.clone();
    let _ = py_setitem(&g_SAME(), &word.clone(), group.clone());
    }
    Py::None
}
pub fn step_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut found: Py = Py::None;
    let _ = py_str("The verb a yes/no question names as its step, or None when it names none.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STEP_VERB(), "search", args))(&__args) };
    return if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) } } else { Py::None };
    Py::None
}
pub fn kind_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The property the line asks for: money, distance, time, heat, amount, order or None.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = question.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("date"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DONENESS(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn asks_a_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let _ = py_str("Whether a follow-up asks for a part of the answer: a figure, a step or a property.");
    return { let __b = Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kind_of(args))(&__args) }, &Py::None)); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _property_ask(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _option_ask(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_follow_up(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_YES_NO(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FREQUENCY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FOR_A_HAND(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } } } } } } } };
    Py::None
}
pub fn _named_follow_up(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clean: Py = Py::None;
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line follows up on a thing it names (\"and if I use a loaf tin?\").");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FOLLOW_UP(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NAMING_IGNORE(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(4)) } else { Py::Bool(false) } };
    Py::None
}
pub fn _gives_a_figure_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut hit: Py = Py::None;
    let _ = py_str("Whether this part of an answer gives a figure in that unit (\"about eight weeks\").\n\n    The unit word has to be counting a figure, and the figure has to be the word right before it:\n    \"three times a week\" says how often and \"each week add a minute\" says what to add, and\n    neither is the answer to \"how many weeks\".\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COUNTED_UNIT(), "finditer", args))(&__args) }) { let mut hit: Py = __item.clone(); __out.push(Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hit.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) }, &unit.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_clause_in_the_unit(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut clause: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The clause of that sentence that gives the figure, said on its own, or None.\n\n    \"Each week add a minute to the jog and take one off the walk, and in about eight weeks you\n    will run 20 minutes without stopping\" holds the figure in its second half, and that half is\n    what answers \"how many weeks\".\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE_BREAK(), "split", args))(&__args) });
    for __item in __iter1 {
    clause = __item.clone();
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gives_a_figure_in(args))(&__args) })) } }) {
    continue;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|but|so|then|also)\\s+")); __args.push(py_str("")); __args.push(part.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_sentence_in_the_unit(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut part: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence that gives a figure in that unit, or None.\n\n    The plain narrowing hands back the first length in the answer, and an answer to a plan names\n    several: \"one minute of easy jogging, two minutes of walking, repeated for 20 minutes, three\n    times a week ... and in about eight weeks you will run 20 minutes without stopping\". Asked\n    \"how many weeks\", the 20 minutes answers a question nobody asked, so the sentence whose\n    figure is written in the unit asked is the one to hand back.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gives_a_figure_in(args))(&__args) } } }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_clause_in_the_unit(args))(&__args) }; if py_truthy(&__b) { __b } else { part.clone() } };
    }
    }
    return Py::None;
    Py::None
}
pub fn part_in_the_unit(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut asked: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The part of an answer that gives a figure in the unit the line asks for, or None.\n\n    \"How many weeks should I train\" after a plan that says sixteen weeks and three runs a week\n    wants the sixteen weeks, and this is the one caller that insists on it: a line about another\n    unit (\"how many days a week\" after an answer of 20 minutes) gets nothing here and is left to\n    whoever reads the sentence.\n    ");
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = question.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKED_UNIT(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&asked.clone(), &Py::None))) {
    return Py::None;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| narrow(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) })) } }) {
    return Py::None;
    }
    unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[A-Za-z]+")); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) }, &unit.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__UNITS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "rstrip", args))(&__args) })) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&said.clone()) { part.clone() } else { Py::None };
    Py::None
}
pub fn asked_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut asked: Py = Py::None;
    let _ = py_str("The unit a \"how many weeks\" line asks for, as a stem, or None when it names none.");
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = question.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKED_UNIT(), "search", args))(&__args) };
    return if py_truthy(&asked.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) } } else { Py::None };
    Py::None
}
pub fn _states_a_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut _u: Py = Py::None;
    let _ = py_str("Whether this part of an answer gives a figure in any unit at all (\"20 minutes\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COUNTED_UNIT(), "finditer", args))(&__args) }) { let mut _u: Py = __item.clone(); __out.push(Py::Bool(true)); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_outcome_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("Which of the outcome asks the line makes, or \"\" when it makes none.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_OUTCOME_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    which = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return which.clone();
    }
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
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut about: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { py_str("") };
    let mut part: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The part of the answer the line asks for, with the unit it names respected.\n\n    \"how many weeks\" after a training plan was answered \"20 minutes.\" - the narrowing takes the\n    first length in the answer, and that length is written in another unit than the one asked\n    for. A line that names its unit is answered by the figure the answer writes in that unit,\n    when the answer has one. A part that states no length at all (\"9 to 10 for hard boiled\") is\n    the answer's own shape for the figure asked, so it stands, and so does a line that names no\n    unit (\"how long\"), which is the narrowing's business.\n    ");
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(question.clone()); __args.push(about.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| narrow(args))(&__args) };
    unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asked_unit(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&unit.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gives_a_figure_in(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _states_a_length(args))(&__args) })) } } } }) {
    return part.clone();
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_sentence_in_the_unit(args))(&__args) }; if py_truthy(&__b) { __b } else { part.clone() } };
    Py::None
}
pub fn _a_rate_cut_from_its_figure(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut asked: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut periods: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("Whether `part` is a figure that left the period making it a rate behind.\n\n    \"and how much does one cost\" after a dog's \"budget about £1,500 to £2,000 a year\" was\n    handed \"£1,500 to £2,000.\", the yearly cost of keeping a dog said as if it were the price\n    of one. A rate the line did not ask about is not the price it asked for; a line that asks\n    \"a year\" keeps it. Only a period written directly on the figure counts: \"a service is £60\n    to £120 once a year\" prices the service, not the year.\n    ");
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SPANS(), &py_str("money")), "finditer", args))(&__args) }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_is(&figure.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERIOD_IN_A_LINE(), "search", args))(&__args) };
    periods = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SPANS(), &py_str("money")), "finditer", args))(&__args) });
    for __item in __iter1 {
    hit = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hit.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "group", args))(&__args) }, "lower", args))(&__args) }))) {
    continue;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&whole.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hit.clone(), "end", args))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hit.clone(), "end", args))(&__args) }, &Py::Int(14)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RATE_ON_A_FIGURE(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&periods.clone(), "append", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&periods.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&asked.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&periods.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }, "lower", args))(&__args) })) } };
    Py::None
}
pub fn narrow(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut about: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { py_str("") };
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The part of `reply` that answers `question`, or None when it cannot say safely.\n\n    None also means the part would be the whole reply: the caller must not repeat it.\n    about is what the last answer was about, for a question too short to say its own subject\n    (\"when does it expire\" after the passport): it decides whether the line is asking about a\n    document, and only a document's life is a validity question.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VALIDITY_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PAPERWORK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(about.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PAPERWORK(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _narrow(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JOINS(), "match", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kind_of(args))(&__args) }, &py_str("money"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rate_cut_from_its_figure(args))(&__args) } } }) {
    return Py::None;
    }
    return part.clone();
    Py::None
}
pub fn _a_length_of_a_doing(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut at: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when `part` is how long something the advice tells them to do goes on for.\n\n    \"my knee hurts\" then \"how long will it last\" was answered \"Two days.\", cut out of \"ice it for\n    20 minutes at a time with a cloth round it for the first two days\": the two days are how long\n    to keep icing it, not how long the sore knee lasts, and \"Two days.\" to somebody worried about\n    their knee is a promise nobody made. A line that names the doing keeps its figure (\"how long\n    should i ice it\"), and a length with no doing in front of it is the thing's own (\"most coughs\n    after a cold last two to three weeks\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STATE_LENGTH_ASKS(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    at = if py_truthy(&cut.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) } } else { op_neg(&Py::Int(1)) };
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "findall", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&doing.clone())); if py_truthy(&__b) { __b } else { op_bitand(&doing.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "findall", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_clause_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DOING(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_cut_off_not_a_length(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the figure `part` is the cut-off a rule starts at, not a length of anything.\n\n    Only the words around the figure decide it. A figure with over/under/aged/around in front of\n    it, whose own clause has no verb of duration, is a condition on the person or thing it\n    describes (\"a baby over six months\", \"children under five\"), so cutting it out answers \"how\n    long will this last\" with an age nobody said was a length. The same words with a duration verb\n    in front of them (\"most back pain lasts over two weeks\") are the thing's own length and stand.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(". ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&cut.clone()))) {
    return Py::Bool(false);
    }
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) };
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None);
    after = slice_or_raise(&said.clone(), &op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CUT_OFF_BEFORE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CUT_OFF_AFTER(), "match", args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__IN_MONTHS_OR_YEARS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BARE_FIGURE(), "fullmatch", args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RANGE(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_clause_before(args))(&__args) };
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DURATION_VERB(), "search", args))(&__args) }));
    Py::None
}
pub fn _a_clause_before(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut cut: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The clause `at` sits in: back to the nearest comma, colon or semicolon.\n\n    The nearest mark, whatever it is: taking the first kind of mark that turns up reads across a\n    semicolon into the clause before it, and \"most coughs after a cold settle in two to three\n    weeks\" then looked like an instruction to keep doing something, because the half-sentence\n    before it in the same sentence says \"keep the room from getting dry\".\n    ");
    start = Py::Int(0);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str(","), py_str(";"), py_str(":")]));
    for __item in __iter1 {
    mark = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); __args.push(Py::Int(0)); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "rfind", args))(&__args) };
    if py_truthy(&op_ge(&cut.clone(), &Py::Int(0))) {
    start = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(start.clone()); __args.push(op_add(&cut.clone(), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return slice_or_raise(&said.clone(), &start.clone(), &at.clone(), &Py::None);
    Py::None
}
pub fn the_doings_length(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The clause of `reply` that gives a time for something to keep doing, or \"\".\n\n    `_narrow` skips such a part, which leaves a length ask with nothing - and nothing is not an\n    improvement on the wrong figure. \"my knee hurts\" then \"how long will it last\" was answered\n    \"Two days.\", then left the card below it to guess at bones; what the card does hold is a real\n    instruction with a clock on it (\"ice it for 20 minutes at a time with a cloth round it for the\n    first two days\"), and a caller can say honestly that it holds no length of its own and give\n    that instruction instead.\n\n    Nothing is handed back when the answer states a length of the thing itself (\"most coughs\n    after a cold last two to three weeks\"), because then there is a length to answer with, and\n    nothing when the line names the doing (\"how long should i ice it\"), because that figure is\n    the answer.\n    ");
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&whole.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STATE_LENGTH_ASKS(), "search", args))(&__args) })) } }) {
    return py_str("");
    }
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kind_of(args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("time"), py_str("clock")]), &kind.clone()))) {
    return py_str("");
    }
    clauses = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "split", args))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &kind.clone()), "search", args))(&__args) })) } }) {
    continue;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(kind.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&part.clone(), &g_NO_LENGTH())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_length_of_a_doing(args))(&__args) }))) {
    return py_str("");
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_clause_around(args))(&__args) };
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clauses.clone(), "append", args))(&__args) };
    }
    }
    return if py_truthy(&clauses.clone()) { geti(&clauses.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _the_clause_around(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut cut: Py = Py::None;
    let mut end: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The clause `at` sits in, back to the mark before it and up to the mark after it.");
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    return py_str("");
    }
    start = Py::Int(0);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str(","), py_str(";"), py_str(":")]));
    for __item in __iter1 {
    mark = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); __args.push(Py::Int(0)); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "rfind", args))(&__args) };
    if py_truthy(&op_ge(&cut.clone(), &Py::Int(0))) {
    start = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(start.clone()); __args.push(op_add(&cut.clone(), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    end = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str(","), py_str(";"), py_str(":")]));
    for __item in __iter2 {
    mark = __item.clone();
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) };
    if py_truthy(&op_ge(&cut.clone(), &Py::Int(0))) {
    end = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(end.clone()); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &start.clone(), &end.clone(), &Py::None), "strip", args))(&__args) };
    Py::None
}
pub fn _at_another_place(args: &[Py]) -> Py {
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
    let mut part: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut asked: Py = Py::None;
    let mut at: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether this part gives the figure of a place other than the one the line asks about.\n\n    \"8 minutes in the microwave first cuts the oven time to 25\" holds the microwave's figure, and\n    asked \"how long in the oven\" the part cut out of that sentence was \"8 minutes\": the words just\n    after a figure say whose figure it is, and the oven's own time is in the sentence around it\n    (\"bake at 200C for about an hour\"). A figure followed by the place asked for (\"45 minutes in\n    the oven\") is that place's own, and so is a figure no place follows.\n    ");
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLACE(), "findall", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    at = if py_truthy(&cut.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "find", args))(&__args) } } else { op_neg(&Py::Int(1)) };
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &op_add(&at.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLACE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&asked.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }));
    Py::None
}
pub fn _gives_another_places_figure(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut part: Py = Py::None;
    let _ = py_str("Whether the sentence's own figure for this ask is written at another place.");
    if py_truthy(&Py::Bool(!py_contains(&g_SPANS(), &kind.clone()))) {
    return Py::Bool(false);
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(kind.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&part.clone(), &g_NO_LENGTH())) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _at_another_place(args))(&__args) };
    Py::None
}
pub fn _narrow(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut both: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut found: Py = Py::None;
    let mut guess: Py = Py::None;
    let mut holder: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut naming: Py = Py::None;
    let mut often: Py = Py::None;
    let mut one: Py = Py::None;
    let mut only: Py = Py::None;
    let mut openings: Py = Py::None;
    let mut part: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut s: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut skipped_a_cut_off: Py = Py::None;
    let mut wait: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut when_ask: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "strip", args))(&__args) })) } }) {
    return Py::None;
    }
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| kind_of(args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("remedy")))) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_remedy_in(args))(&__args) };
    return if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) })) } }) { part.clone() } else { Py::None };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&kind.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SPANS(), &kind.clone())) } }) {
    return Py::None;
    }
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_URGENT(), "search", args))(&__args) }) {
    return Py::None;
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "split", args))(&__args) }) { let mut s: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&s.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    when_ask = Py::Bool(false);
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("date")))) {
    dated = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _dated(args))(&__args) };
    if py_truthy(&dated.clone()) {
    return dated.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_THE_DATE_ITSELF(), "search", args))(&__args) }) {
    return Py::None;
    }
    when_ask = Py::Bool(true);
    kind = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &py_str("time")), "search", args))(&__args) }) { py_str("time") } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_said_again(args))(&__args) };
    }
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POINTS_BACK(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&wanted.clone()) {
    naming = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&naming.clone()) {
    sentences = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&naming.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(kind.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gives_another_places_figure(args))(&__args) }))) {  __out.push(one.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { sentences.clone() } };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("cause")))) {
    only = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&only.clone()) {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&op_bitand(&only.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&named.clone()) {
    sentences = named.clone();
    }
    }
    openings = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?!.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, "strip", args))(&__args) }, &py_str("why")))) { slice_or_raise(&sentences.clone(), &Py::None, &Py::Int(1), &Py::None) } else { sentences.clone() };
    let __iter1 = py_iter_or_raise(&openings.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CAUSE(), "search", args))(&__args) }) {
    return if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone()))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) } } else { Py::None };
    }
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLACE(), "search", args))(&__args) } } }) {
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLACE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COOKS(), "search", args))(&__args) } } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BEFORE(), "search", args))(&__args) } } }) {
    let __iter2 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESULT(), "search", args))(&__args) }) {  __out.push(one.clone()); } } py_list(__out) })() });
    for __item in __iter2 {
    sentence = __item.clone();
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESULT(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } py_list(__out) })() };
    part = if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&found.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return part.clone();
    }
    }
    }
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_clause(args))(&__args) };
    if py_truthy(&piece.clone()) {
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_unit(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&kind.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SPANS(), &kind.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &kind.clone()), "search", args))(&__args) } } } } }) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push(kind.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&part.clone(), &g_NO_LENGTH()))) {
    part = Py::None;
    }
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _at_another_place(args))(&__args) } } }) {
    part = Py::None;
    }
    if py_truthy(&part.clone()) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return part.clone();
    }
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    often = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _how_often(args))(&__args) };
    if py_truthy(&often.clone()) {
    return often.clone();
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); __args.push(whole.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_a_figure(args))(&__args) };
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_part_they_named(args))(&__args) };
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("time"), py_str("money"), py_str("distance"), py_str("amount"), py_str("heat")]));
    for __item in __iter3 {
    guess = __item.clone();
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = piece.clone(); if py_truthy(&__b) { __b } else { reply.clone() } }); __args.push(guess.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_item_they_named(args))(&__args) };
    if py_truthy(&one.clone()) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return part.clone();
    }
    }
    }
    if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FOLLOW_UP(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_YES_NO(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_ABOUT_IT(), "search", args))(&__args) } } } } } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _informative(args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cuts_a_list(args))(&__args) } } }) {
    holder = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }))) {  __out.push(one.clone()); } } py_list(__out) })() };
    longer = if py_truthy(&holder.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&holder.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = longer.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&longer.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(longer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } } } }) {
    return longer.clone();
    }
    }
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return said.clone();
    }
    }
    return Py::None;
    return Py::None;
    }
    wait = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    skipped_a_cut_off = Py::Bool(false);
    let __iter4 = py_iter_or_raise(&sentences.clone());
    for __item in __iter4 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &kind.clone()), "search", args))(&__args) }))) {
    continue;
    }
    both = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure_each(args))(&__args) };
    if py_truthy(&both.clone()) {
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(both.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_half_they_named(args))(&__args) };
    if py_truthy(&one.clone()) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return part.clone();
    }
    }
    }
    if py_truthy(&{ let __b = wait.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) })) } } } }) {
    continue;
    }
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(kind.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&part.clone(), &g_NO_LENGTH()))) {
    continue;
    }
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _at_another_place(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&part.clone(), &g_NO_LENGTH())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(part.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_length_of_a_doing(args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&part.clone(), &g_NO_LENGTH())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_cut_off_not_a_length(args))(&__args) } } } } }) {
    skipped_a_cut_off = Py::Bool(true);
    continue;
    }
    if py_truthy(&part.clone()) {
    part = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_a_length(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    holder = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(sentence.clone()); __args.push(whole.clone()); __args.push(when_ask.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_sentence_a_figure_hangs_on(args))(&__args) };
    if py_truthy(&holder.clone()) {
    return holder.clone();
    }
    return part.clone();
    }
    }
    if py_truthy(&when_ask.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_said_again(args))(&__args) };
    }
    if py_truthy(&skipped_a_cut_off.clone()) {
    continue;
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LENGTH_OF_THE_JOB(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STATE_LENGTH_ASKS(), "search", args))(&__args) } } } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_time_it_does_keep(args))(&__args) };
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_start_age(args: &[Py]) -> Py {
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
    let mut figure: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut at: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut before: Py = Py::None;
    let mut flat_sentence: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The figure as a start age (\"around six months\"), or None if it is not read as one.");
    flat_sentence = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(py_str("split")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("hasattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    let __try1 = exc::catch(|| {
    before = slice_or_raise(&flat_sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&at.clone(), &Py::Int(24))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &at.clone(), &Py::None);
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
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_START_LEAD(), "search", args))(&__args) };
    if py_truthy(&lead.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, &py_str(" ")), &figure.clone()), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _the_time_it_does_keep(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut whole: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut clause: Py = Py::None;
    let mut last_end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut start_age: Py = Py::None;
    let mut timed: Py = Py::None;
    let _ = py_str("The sentence of the answer that talks about time, said as the help it is.");
    start_age = Py::None;
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &py_str("time")), "search", args))(&__args) }))) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![py_list(vec![]), op_neg(&Py::Int(1))]), 2, None);
    timed = __unpk2[0].clone();
    last_end = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &py_str("time")), "finditer", args))(&__args) });
    for __item in __iter3 {
    match_ = __item.clone();
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &last_end.clone())) {
    continue;
    }
    last_end = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentence.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IN_A_RELATIVE(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_AFTER(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_period_of_extent(args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_AFTER(), "match", args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&timed.clone(), "append", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&timed.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LENGTH_OF_THE_JOB(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdepends\\b")); __args.push(sentence.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&start_age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&timed.clone()) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_cut_off_not_a_length(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    one = geti(&timed.clone(), &Py::Int(0));
    start_age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_start_age(args))(&__args) };
    }
    clause = Py::None;
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:;|,\\s*(?:and|but|so)\\b)")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    one = __item.clone();
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_cut_off(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_a_time(args))(&__args) })) } }) {
    continue;
    }
    clause = one.clone();
    break;
    }
    if py_truthy(&Py::Bool(py_is(&clause.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(24)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone()))) {
    continue;
    }
    return op_add(&op_add(&op_add(&g_NO_SET_TIME(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&clause.clone(), &Py::Int(0)), "lower", args))(&__args) }), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    if py_truthy(&start_age.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("when".to_string(), start_age.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_A_STAGE(), &Py::Int(0)), "format", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _a_period_of_extent(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("Whether the time ending at `at` is introduced by a word of extent (\"for a month\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &Py::None, &at.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERIOD_OF_EXTENT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _says_a_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("Whether this clause says anything about time at all.\n\n    The fallback hands the person the half of the sentence about the time, so a half that only\n    lists signs (\"three signs, all at once\") is not it - that answered \"how long will that go on\n    for\" with a list of what to look for.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &py_str("time")), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:long|longer|slow|quick|soon|never|until|till|months?|weeks?|years?|days?|hours?|minutes?)\\b")); __args.push(clause.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _only_a_cut_off(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("Whether every figure in this clause is the age a rule starts at, and none is a length.\n\n    \"For a baby over six months\" is a clause about when the advice begins; \"feedings at night are\n    normal for much longer\" is a clause about the time. Only the second answers \"how long will\n    this last\", so the first is passed over rather than said as if it gave the length.\n    ");
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SIGNALS(), &py_str("time")), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&figures.clone()))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&figures.clone()) { let mut figure: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); __args.push(figure.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_cut_off_not_a_length(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_answer_said_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut whole: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut opening: Py = Py::None;
    let _ = py_str("The sentence an answer opens with, when the answer has more to it than that.\n\n    Only for a when ask the answer says no when about: the hedge that would take the turn instead\n    reads as the bot forgetting what it just said.\n    ");
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    opening = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&sentences.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opening.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&opening.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(30)) } } } }) {
    return Py::None;
    }
    return opening.clone();
    Py::None
}
pub fn _the_sentence_a_figure_hangs_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut when_ask: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when_ask") { v.clone() }
    else { Py::Bool(false) };
    let mut holder: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&when_ask.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WHEN_WORD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return Py::None;
    }
    holder = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&holder.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(holder.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&holder.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(25)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(holder.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, "strip", args))(&__args) }))) {
    return Py::None;
    }
    return holder.clone();
    Py::None
}
pub fn _unit_stem(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let _ = py_str("The thing a unit word counts: \"mins\", \"minute\" and \"minutes\" are all a minute.");
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }, "rstrip", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNITS(), "get", args))(&__args) };
    Py::None
}
pub fn _same_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut short: Py = Py::None;
    let _ = py_str("Whether two words are the same word or one is the other with an ending on it.");
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &other.clone()))) {
    return Py::Bool(true);
    }
    short = if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) { Py::Int(4) } else { Py::Int(3) };
    return Py::Bool(py_eq(&slice_or_raise(&one.clone(), &Py::None, &short.clone(), &Py::None), &slice_or_raise(&other.clone(), &Py::None, &short.clone(), &Py::None)));
    Py::None
}
pub fn _names_one_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut wanted: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&words.clone()) { let mut said: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same_word(args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _only_a_count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("True when the only rate words in the piece are counts with no period beside them.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY(), "finditer", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut match_: Py = __item.clone(); __out.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BARE_COUNT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("once"), py_str("twice"), py_str("thrice")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) })) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE_INTERVAL(), "search", args))(&__args) })) } };
    Py::None
}
pub fn _how_often(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut about: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut first_word: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut rates: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut the_doing: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause of the answer that says how often, when the line asks that.\n\n    \"i started going to the gym\" is answered with the movements and then \"two or three sessions\n    a week\", and \"how often should i go\" wants that clause. The clause has to be about the same\n    thing the line asks about: \"how often should i mow it\" after a lawn answer that only says\n    how often to water is left to the caller rather than answered with the watering.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FREQUENCY(), "search", args))(&__args) }))) {
    return Py::None;
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NAMING_IGNORE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FREQUENCY_IGNORE(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    rates = py_list(vec![]);
    counted = Py::None;
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    about = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_one_of(args))(&__args) } } };
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __kws.extend(vec![("weak".to_string(), { let __b = about.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wanted.clone())) } }), ("wanted".to_string(), if py_truthy(&about.clone()) { wanted.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _freq_clause(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&about.clone())) } }) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_count(args))(&__args) })) } }) {
    return said.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_count(args))(&__args) }) {
    counted = { let __b = counted.clone(); if py_truthy(&__b) { __b } else { said.clone() } };
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rates.clone(), "append", args))(&__args) };
    }
    let __iter2 = py_iter_or_raise(&rates.clone());
    for __item in __iter2 {
    said = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_count(args))(&__args) }) {
    counted = { let __b = counted.clone(); if py_truthy(&__b) { __b } else { said.clone() } };
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQ_LIMIT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQ_PORTION(), "search", args))(&__args) } } }))) {
    return said.clone();
    }
    }
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    if py_truthy(&rates.clone()) {
    return geti(&rates.clone(), &Py::Int(0));
    }
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TRIGGER(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bagain\\b")); __args.push(sentence.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return said.clone();
    }
    }
    }
    opening = if py_truthy(&sentences.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&sentences.clone(), &Py::Int(0)), "strip", args))(&__args) } } else { py_str("") };
    first_word = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&opening.clone(), "split", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&opening.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }, "lower", args))(&__args) } } else { py_str("") };
    the_doing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_HOW_OFTEN_THE_DOING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { { let __b = first_word.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); __args.push(py_list(vec![first_word.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_one_of(args))(&__args) } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ONLY_THE_INTERVAL_ASK(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { the_doing.clone() } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE_INTERVAL(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SET_OF_STEPS(), "search", args))(&__args) } } } } }) {
    return g_NO_INTERVAL_SAID();
    }
    return if py_truthy(&rates.clone()) { geti(&rates.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _counts_on_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let _ = py_str("Whether the clause counts something the answer already said rather than naming its rate.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the same|same count|that count|as above|as before)\\b")); __args.push(piece.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _freq_clause(args: &[Py]) -> Py {
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
    let mut weak: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "weak") { v.clone() }
    else { Py::Bool(false) };
    let mut wanted: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("The clause of one sentence that carries the frequency, or None.");
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQ_CLAUSE(), "split", args))(&__args) }) { let mut piece: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = weak.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE_FREQ(), "search", args))(&__args) } } } } }) {  __out.push(piece.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(30))) {
    return sentence.clone();
    }
    if py_truthy(&wanted.clone()) {
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_one_of(args))(&__args) }) {  __out.push(piece.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    }
    plain = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counts_on_the_answer(args))(&__args) }))) {  __out.push(piece.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { found.clone() } };
    piece = geti(&plain.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:then|and|also|plus|after that|afterwards)\\b")); __args.push(piece.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    piece = geti(&plain.clone(), &Py::Int(1));
    }
    if py_truthy(&{ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } }) {
    piece = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&found.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14))) {
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",\\s*")); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    at = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6875 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk6875[0].clone(); let mut part: Py = __unpk6875[1].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY(), "search", args))(&__args) }) {  __out.push(index.clone()); } } py_list(__out) })() };
    if py_truthy(&at.clone()) {
    last = geti(&at.clone(), &op_neg(&Py::Int(1)));
    piece = geti(&parts.clone(), &last.clone());
    if py_truthy(&{ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)); if !py_truthy(&__b) { __b } else { last.clone() } }) {
    piece = op_add(&op_add(&geti(&parts.clone(), &op_sub(&last.clone(), &Py::Int(1))), &py_str(", ")), &piece.clone());
    }
    }
    }
    return if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) { piece.clone() } else { Py::None };
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
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut score: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence of the answer that says when, picking between dates by the words used.\n\n    After an answer that holds both 43 AD and 410 AD, \"when did they leave\" is about the\n    second one, and the word the question uses (\"leave\") is the word that sentence uses.\n    ");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAME(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "update", args))(&__args) };
    }
    let __unpk2 = py_unpack(&py_tuple(vec![Py::None, py_tuple(vec![op_neg(&Py::Int(1)), op_neg(&Py::Int(1))])]), 2, None);
    best = __unpk2[0].clone();
    best_score = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| has_a_date(args))(&__args) }))) {
    continue;
    }
    score = py_tuple(vec![if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DATED_STRONG(), "search", args))(&__args) }) { Py::Int(1) } else { Py::Int(0) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "intersection", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![sentence.clone(), score.clone()]), 2, None);
    best = __unpk4[0].clone();
    best_score = __unpk4[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    Py::None
}
pub fn _the_part_they_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut shared: Py = Py::None;
    let _ = py_str("The sentence of the answer about the thing this advice ask names, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FOR_A_HAND(), "search", args))(&__args) }))) {
    return Py::None;
    }
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _by_words(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    return Py::None;
    }
    shared = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(shared.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_NAMES_A_PART())) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return said.clone();
    }
    return Py::None;
    Py::None
}
pub fn _not_a_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut reply: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut option: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The sentence that answers a step or a property question, when it is not a figure.\n\n    A yes/no about one step (\"do I need to parboil them first?\") is answered by the sentence\n    that starts with that step. A question about a property (\"how do I get them crisp?\") is\n    answered by the sentence that says the same thing in other words (\"...they go crunchy\").\n    Both return None when no sentence speaks to the line, so the caller passes instead of\n    repeating the whole answer.\n\n    `reply` is the answer before the caller kept only the sentences that name the thing asked\n    about, because a script quoted in the answer can be split across two of the original\n    sentences (see `_with_the_script`).\n    ");
    option = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _by_option(args))(&__args) };
    if py_truthy(&{ let __b = option.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); __args.push(sentences.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_script(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(option.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) } } };
    }
    step = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _by_step(args))(&__args) };
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); __args.push(sentences.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_script(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) } } };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _property_ask(args))(&__args) }) {
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _by_words(args))(&__args) };
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &whole.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tidy(args))(&__args) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _named_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut best: Py = Py::None;
    let mut best_count: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut i: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut roots: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause of the answer about a thing the line names, or None.\n\n    Only a thing that the answer says in exactly one clause counts: the loaf tin is named in\n    one, so the question about it has one place to look. A word that runs through every clause\n    (\"the oven\" in an oven method) names no one part, and is left to the readers below.\n\n    Two words that belong together name that thing and nothing else: a bundt tin is not the\n    loaf tin the answer talks about, however many tins both of them mention.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NAMING_IGNORE(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    pairs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut i: Py = __item.clone(); __out.push(py_tuple(vec![geti(&named.clone(), &i.clone()), geti(&named.clone(), &op_add(&i.clone(), &Py::Int(1)))])); } py_list(__out) })() };
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(py_str(";\\s*|(?<=[.!?])\\s+|,\\s+(?:and|but|so)\\s+|\\s+(?:and|but|so)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pieces_of(args))(&__args) }) { let mut piece: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    roots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _root(args))(&__args) }); } py_set(__out) })() };
    pieces = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(g_NAMED_BREAK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| clauses(args))(&__args) };
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0), Py::Int(0)]), 3, None);
    best = __unpk1[0].clone();
    best_score = __unpk1[1].clone();
    best_count = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&pieces.clone());
    for __item in __iter2 {
    piece = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    score = if py_truthy(&pairs.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pairs.clone()) { let mut pair: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pair.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says(args))(&__args) }) {  __out.push(pair.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { Py::Int(0) };
    if py_truthy(&Py::Bool(!py_truthy(&score.clone()))) {
    score = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(roots.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _root(args))(&__args) }); } py_set(__out) })() }, "intersection", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk3 = py_unpack(&py_tuple(vec![piece.clone(), score.clone(), Py::Int(1)]), 3, None);
    best = __unpk3[0].clone();
    best_score = __unpk3[1].clone();
    best_count = __unpk3[2].clone();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&score.clone(), &best_score.clone())); if !py_truthy(&__b) { __b } else { score.clone() } }) {
    let __aug4 = op_add(&best_count.clone(), &Py::Int(1));
    best_count = __aug4;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&best_count.clone(), &Py::Int(1))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_le(&roots.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _root(args))(&__args) }); } py_set(__out) })() })))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".!?,;: ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&best.clone(), "rstrip", args))(&__args) };
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
    let _ = py_str("The stem a word shares with its other endings (\"cleaning\", \"cleaned\" -> \"clean\").");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    break;
    }
    }
    return if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("e")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() };
    Py::None
}
pub fn _says(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether `text` says `phrase`, plurals included (\"loaf tin\" in \"loaf tins\").");
    pattern = op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("[a-z]*\\s+"), "join", args))(&__args) }), &py_str("[a-z]*\\b"));
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    Py::None
}
pub fn _informative(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let _ = py_str("Whether this clause answers on its own: a figure in it, or a step to take.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COOKS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _cuts_a_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("Whether this clause is one item of a list the answer carries on with.\n\n    \"For a warm week: light cotton tops\" is the first item of \"light cotton tops and shorts, one\n    long-sleeved layer and a light waterproof ... shoes you can walk in\", and said alone it reads\n    like an answer that stopped halfway. A clause the comma or the \"and\" in front of it belongs\n    to is not a piece of its own, so the sentence that holds it is the answer.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = piece.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&whole.clone(), &said.clone())) } }) {
    return Py::Bool(false);
    }
    rest = slice_or_raise(&whole.clone(), &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&whole.clone(), "index", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None, &Py::None);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:[,;]|\\band\\b)")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _with_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut text: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("A clause that ends on a bare range gets the unit the answer used before it.\n\n    \"A loaf tin needs 45 to 55\" is right in the middle of the answer and thin on its own; the\n    minutes are a word earlier, so they come along.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_RANGE(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &op_neg(&Py::Int(12)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_UNIT(), "search", args))(&__args) } } }) {
    return text.clone();
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "find", args))(&__args) };
    units = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) { before.clone() } else { slice_or_raise(&before.clone(), &Py::None, &at.clone(), &Py::None) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_UNIT(), "finditer", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&units.clone()))) {
    return text.clone();
    }
    return op_add(&op_add(&text.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&units.clone(), &op_neg(&Py::Int(1))), "group", args))(&__args) });
    Py::None
}
pub fn _property_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line asks how the thing turns out, in words the answer may say differently.\n\n    Only a \"how do I get it crisp?\" shape counts. A statement that happens to hold one of\n    those words (\"a big project slipped\") is not a question about the answer. \"Saving\" is a\n    cheapness word only as a property of the thing (\"how do I get it for less\"); the person's\n    own saving (\"how do I start saving money\", \"how do I save up for a car\") is a plan they\n    are asking about, not a part of the answer we just gave.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|can|should|would|could) (?:i|we|you)\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OWN_SAVING(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SAME(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _option_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clean: Py = Py::None;
    let _ = py_str("Whether the line is an elliptical ask about one named option (\"and for a hard one?\").");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPTION(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUALIFIER(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_OPTION(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8)) } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _stands_alone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut text: Py = Py::None;
    let _ = py_str("Whether a clause cut out of an answer is an answer on its own, not half a sentence.\n\n    A qualifier word can sit in a fragment: \"A hard\" out of \"A hard, fixed lump under the skin\n    is worth showing to a GP\", \"Gas.\" out of a sentence about magma. Said alone the fragment is\n    not a reply, but the sentence it was cut from says the same thing with what to do about it,\n    so that sentence is the answer. A figure stands on its own (\"200C.\"), and so does any\n    clause of two content words (\"Under the skin\", \"Not the top\").\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = piece.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2));
    Py::None
}
pub fn _by_option(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clause: Py = Py::None;
    let mut qualifier: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The clause of the sentence that names the option the line asks about.");
    qualifier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUALIFIER(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&qualifier.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&qualifier.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clause_with(args))(&__args) };
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) })) } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_qualifier(args))(&__args) }) {
    return sentence.clone();
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stands_alone(args))(&__args) }) { clause.clone() } else { sentence.clone() };
    }
    }
    return Py::None;
    Py::None
}
pub fn _only_a_qualifier(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a clause cut out of an answer names nothing but the qualifier the line asked for.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_QUALIFIER_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_QUALIFIER_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _by_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The clause that starts with the step a yes/no question names, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_YES_NO(), "search", args))(&__args) }))) {
    return Py::None;
    }
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    step = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| step_word(args))(&__args) };
    if py_truthy(&step.clone()) {
    wanted = op_bitor(&py_set(vec![step.clone()]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAME(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    raw = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD(), "findall", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&raw.clone()))) {
    continue;
    }
    first = geti(&raw.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "endswith", args))(&__args) })) } } } }) {
    first = slice_or_raise(&first.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&Py::Bool(py_contains(&wanted.clone(), &first.clone()))) {
    cut = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[:,]\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)), "strip", args))(&__args) }; if py_truthy(&__b) { __b } else { sentence.clone() } };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stands_alone(args))(&__args) }) { cut.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) } };
    }
    }
    return Py::None;
    Py::None
}
pub fn _with_the_script(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut sentences: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut at: Py = Py::None;
    let mut head: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut whole_answer: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A bare verdict on the saying, with the words the person would use, and nothing else.\n\n    The readers above cut a sentence at its comma or its colon, which is right for a step whose\n    tail is the reason it works (\"Turn the boiler off: the pump...\"). It is wrong for a verdict\n    on the saying, because the tail there is a script quoted in the answer, and a person cannot\n    write that themselves: asked \"should i say something\" after the cancelling answer, the turn\n    was \"Say it once.\" on its own.\n\n    What is handed back is the verdict *and the quotation*, not the whole sentence: the rest of\n    that sentence is why the script works, which the previous turn already said, and a reply that\n    reads the last answer out again is passed over as a repeat. So a sentence with no quotation\n    in it returns nothing here and keeps its plain cut. Only a *bare* verdict is treated this\n    way: \"Say the good news once and let theirs be theirs\" is an instruction a person can act on\n    already, and keeps its cut too.\n    ");
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".?! ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "strip", args))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_BAREST_VERDICT()); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str(" "))) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS_SOMETHING(), "match", args))(&__args) }))) {
    return Py::None;
    }
    whole_answer = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENTENCE(), "split", args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&whole_answer.clone()))) {
    whole_answer = sentences.clone();
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole_answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, "startswith", args))(&__args) }) {
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole_answer.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _to_the_end_of_the_script(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_quotation(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    return op_modulo(&py_str("%s, like this: %s"), &py_tuple(vec![head.clone(), words.clone()]));
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_quotation(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The words quoted in the sentence, marks and all, when it quotes any.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED_WORDS(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED_WORDS_SINGLE(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    return op_modulo(&py_str("'%s'"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) });
    }
    return Py::None;
    Py::None
}
pub fn _to_the_end_of_the_script(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut index: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The sentence, plus what the splitter took for the next one when it broke inside a quote.\n\n    The answer's script is a quotation, and a full stop inside it ends a sentence for the\n    splitter: handed back on its own, \"Say it once, about the pattern rather than the last time:\n    \"We've cancelled three times now and honestly it's starting to feel like I'm an afterthought\"\n    is the words cut off in the middle, which is worse than the verdict alone. So the quote is\n    counted and the following sentences are added until it closes.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&sentences.clone(), &at.clone()), "strip", args))(&__args) };
    index = at.clone();
    while py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentences.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { op_modulo(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\"")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "count", args))(&__args) }, &Py::Int(2)) } }) {
    let __aug1 = op_add(&index.clone(), &Py::Int(1));
    index = __aug1;
    text = op_add(&op_add(&text.clone(), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&sentences.clone(), &index.clone()), "strip", args))(&__args) });
    }
    return text.clone();
    Py::None
}
pub fn _by_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut best: Py = Py::None;
    let mut best_length: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut length: Py = Py::None;
    let mut score: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence that shares the most words with the question, when it shares any.");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAME(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "update", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0), Py::Int(0)]), 3, None);
    best = __unpk2[0].clone();
    best_score = __unpk2[1].clone();
    best_length = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    score = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "intersection", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    length = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = score.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&score.clone(), &best_score.clone()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&score.clone(), &best_score.clone())); if !py_truthy(&__b) { __b } else { op_lt(&length.clone(), &best_length.clone()) } } } } } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![sentence.clone(), score.clone(), length.clone()]), 3, None);
    best = __unpk4[0].clone();
    best_score = __unpk4[1].clone();
    best_length = __unpk4[2].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _as_a_length(args: &[Py]) -> Py {
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
    let mut say: Py = Py::None;
    let _ = py_str("`a 20-minute job` read as `20 minutes`: the answer to how long is the time itself.");
pub fn __nested_say_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut say: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut number: Py = Py::None;
    let mut unit: Py = Py::None;
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }]), 2, None);
    number = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("1"), py_str("1.0")]), &number.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "endswith", args))(&__args) })) } }) {
    let __aug2 = op_add(&unit.clone(), &py_str("s"));
    unit = __aug2;
    }
    return op_add(&op_add(&number.clone(), &py_str(" ")), &unit.clone());
    Py::None
}
    say = {{ let say = say.clone(); let text = text.clone(); py_func("say", move |args: &[Py]| { let mut full = vec![say.clone(), text.clone()]; full.extend_from_slice(args); __nested_say_0(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(say.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DURATION_ADJECTIVE(), "sub", args))(&__args) };
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
    let mut question: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line names something of its own beyond the property it asks for.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_ASKING_ONLY(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTS_BACK(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _measure(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The unit a figure is given in, or \"\" when it names none.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = span.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIGURE_MEASURE(), "search", args))(&__args) };
    return if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut left: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The whole figure a span found, thousands separator and all.\n\n    The span patterns read a figure by its unit (\"475 km\", \"500C\"), and a number written with\n    thousands separators starts a new word at every comma, so 3,475 km was read as 475 km and\n    5,500C as 500C. Asked again, the answer came back as the wrong figure by a factor of seven:\n    `how far is the moon from earth`, after the moon's size was answered, gave \"400 km.\" from\n    \"384,400 km\", and \"how many miles is that\" after a tyre's life gave \"000 miles.\" from\n    \"20,000 to 40,000 miles\". The span is widened back over the digits and commas in front of it\n    here, rather than in every pattern, so a figure comes out whole whatever its unit.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    start = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) };
    left = start.clone();
    while py_truthy(&{ let __b = op_gt(&left.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&text.clone(), &op_sub(&left.clone(), &Py::Int(1))), "isdigit", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str(",."), &geti(&text.clone(), &op_sub(&left.clone(), &Py::Int(1))))) } } } }) {
    let __aug1 = op_sub(&left.clone(), &Py::Int(1));
    left = __aug1;
    }
    if py_truthy(&Py::Bool(py_eq(&left.clone(), &start.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &left.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None), "lstrip", args))(&__args) };
    Py::None
}
pub fn _a_figure_each(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut figure: Py = Py::None;
    let mut halves: Py = Py::None;
    let mut one: Py = Py::None;
    let mut stated: Py = Py::None;
    let _ = py_str("The halves of a sentence that each state a figure for a thing, or None.\n\n    One sentence can hold two answers: \"Green tea wants water at about 80C for 2 to 3 minutes;\n    black tea wants it just off the boil for 3 to 5.\" Each half names a thing and gives the\n    figure that belongs to it, so the first figure on its own is not the answer - it is the time\n    for the first of the two with nothing saying which. Only a semicolon joins two answers like\n    that, and only when both halves carry a figure; a sentence of one answer that happens to use\n    a semicolon is left to the clause reader as before.\n    ");
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPANS(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&figure.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_str(";"))) } }) {
    return Py::None;
    }
    halves = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(halves.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    stated = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&halves.clone()) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&stated.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&halves.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "rstrip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_RANGE(), "search", args))(&__args) } } }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    return halves.clone();
    Py::None
}
pub fn _the_item_they_named(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut _u: Py = Py::None;
    let mut best: Py = Py::None;
    let mut body: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut index: Py = Py::None;
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut span: Py = Py::None;
    let mut split: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut stop: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut winners: Py = Py::None;
    let _ = py_str("The item of a one-figure-per-thing list that the line names, or None.\n\n    Each item runs from one figure to the next, so the list is cut at its figures rather than at\n    its commas: in \"10 to 20 for a joint of beef, lamb or pork, 5 to 10 for a steak\" the \"lamb or\n    pork\" belongs to the beef, and cutting at a comma would leave it with no figure and drop it.\n    Only a list of two or more items is read this way, and only when the line names one of them\n    and no other: a bare \"and how long\" matches none of them and is left to the readers below,\n    which is what it said before.\n    ");
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPANS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    body = if py_truthy(&Py::Bool(py_contains(&sentence.clone(), &py_str(":")))) { geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }, &Py::Int(1)) } else { sentence.clone() };
    starts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&gattr(&figure.clone(), "pattern"), &py_str("|")), &gattr(&g_A_BARE_RANGE(), "pattern"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&starts.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_frequency(args))(&__args) }))) {  __out.push(match_.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    items = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    match_ = __unpk2[1].clone();
    stop = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&found.clone(), &op_add(&index.clone(), &Py::Int(1))), "start", args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, slice_or_raise(&body.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &stop.clone(), &Py::None)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&items.clone(), "append", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let __unpk1641 = py_unpack(&__item.clone(), 2, None); let mut span: Py = __unpk1641[0].clone(); let mut piece: Py = __unpk1641[1].clone(); __out.push(Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(span.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FOR_A_THING(), "match", args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    scored = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let __unpk2745 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk2745[0].clone(); let mut piece: Py = __unpk2745[1].clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, piece.clone()])); } py_list(__out) })() };
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk3321 = py_unpack(&__item.clone(), 2, None); let mut score: Py = __unpk3321[0].clone(); let mut _u: Py = __unpk3321[1].clone(); __out.push(score.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    winners = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk3801 = py_unpack(&__item.clone(), 2, None); let mut score: Py = __unpk3801[0].clone(); let mut piece: Py = __unpk3801[1].clone(); if py_truthy(&Py::Bool(py_eq(&score.clone(), &best.clone()))) {  __out.push(piece.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&best.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(winners.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) } }) {
    return Py::None;
    }
    item = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,]*(?:and|or)?[\\s,]*$")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&winners.clone(), &Py::Int(0)), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_UNIT_WORD(), "search", args))(&__args) }))) {
    unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let __unpk6201 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk6201[0].clone(); let mut one: Py = __unpk6201[1].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_UNIT_WORD(), "search", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_UNIT_WORD(), "search", args))(&__args) }); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return Py::None;
    }
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BARE_RANGE(), "search", args))(&__args) };
    split = if py_truthy(&head.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "end", args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    item = op_add(&op_add(&op_add(&slice_or_raise(&item.clone(), &Py::None, &split.clone(), &Py::None), &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "group", args))(&__args) }, "lower", args))(&__args) }), &slice_or_raise(&item.clone(), &split.clone(), &Py::None, &Py::None));
    }
    return item.clone();
    Py::None
}
pub fn _the_half_they_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut halves: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "halves") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: halves"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut _u: Py = Py::None;
    let mut best: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut score: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut winners: Py = Py::None;
    let _ = py_str("The half of a two-answer sentence the line names, or None when it names neither.\n\n    \"how long for black tea\" names the black tea half and not the green one, so the words the\n    line shares with each half decide it. A line that names both halves equally (\"and how\n    long\") gets None, because then both figures are the answer and neither works alone.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    scored = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&halves.clone()) { let mut one: Py = __item.clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, one.clone()])); } py_list(__out) })() };
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk1098 = py_unpack(&__item.clone(), 2, None); let mut score: Py = __unpk1098[0].clone(); let mut _u: Py = __unpk1098[1].clone(); __out.push(score.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    winners = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk1578 = py_unpack(&__item.clone(), 2, None); let mut score: Py = __unpk1578[0].clone(); let mut one: Py = __unpk1578[1].clone(); if py_truthy(&Py::Bool(py_eq(&score.clone(), &best.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&best.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(winners.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) } }) {
    return Py::None;
    }
    return geti(&winners.clone(), &Py::Int(0));
    Py::None
}
pub fn _a_condition(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when the figure counts something other than the length of the whole job.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AT_A_TIME(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RELATIVE_AFTER(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    before = slice_or_raise(&sentence.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None);
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IN_A_CONDITION(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BOUNDED(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STATE_FOR(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WEATHER_WINDOW(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IN_A_RELATIVE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _part(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut question: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut lead_spans: Py = Py::None;
    let mut lengths: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut options: Py = Py::None;
    let mut qualifier: Py = Py::None;
    let mut rates: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut result: Py = Py::None;
    let mut same: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut stepped: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut wanted_unit: Py = Py::None;
    let _ = py_str("The tightest safe answer inside one sentence.");
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPANS(), "get", args))(&__args) };
    qualifier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUALIFIER(), "search", args))(&__args) };
    if py_truthy(&qualifier.clone()) {
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&qualifier.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clause_with(args))(&__args) };
    if py_truthy(&clause.clone()) {
    found = if py_truthy(&figure.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } py_list(__out) })() } } else { py_list(vec![]) };
    if py_truthy(&found.clone()) {
    return geti(&found.clone(), &op_neg(&Py::Int(1)));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return clause.clone();
    }
    }
    }
    spans = if py_truthy(&figure.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } py_list(__out) })() } } else { py_list(vec![]) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = spans.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LENGTH_OF_THE_JOB(), "search", args))(&__args) } } } } }) {
    lengths = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_frequency(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MOMENT(), "fullmatch", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COUNTDOWN(), "search", args))(&__args) })) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_condition(args))(&__args) })) } } } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&lengths.clone()))) {
    return g_NO_LENGTH();
    }
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE_LEAD(), "match", args))(&__args) };
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "search", args))(&__args) } } }) {
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&sentence.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    if py_truthy(&{ let __b = cut.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "search", args))(&__args) } } }) {
    sentence = cut.clone();
    lengths = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_frequency(args))(&__args) }))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } } py_list(__out) })() };
    }
    }
    spans = lengths.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = spans.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LENGTH_OF_THE_JOB(), "search", args))(&__args) })) } } } }) {
    rates = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_frequency(args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } } py_list(__out) })() };
    lengths = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_frequency(args))(&__args) }))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&{ let __b = rates.clone(); if !py_truthy(&__b) { __b } else { lengths.clone() } }) {
    return geti(&rates.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&spans.clone()))) {
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("clock")))) {
    return Py::None;
    }
    return if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(28))) { sentence.clone() } else { Py::None };
    }
    wanted_unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = question.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKED_UNIT(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = wanted_unit.clone(); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } } } }) {
    unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted_unit.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) };
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&span.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _unit_stem(args))(&__args) }, &unit.clone()))) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(same.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&same.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("amount")))) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clause(args))(&__args) }; if py_truthy(&__b) { __b } else { sentence.clone() } };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("money"), py_str("distance")]), &kind.clone()))) {
    return geti(&spans.clone(), &Py::Int(0));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATE(), "search", args))(&__args) }) {
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bper\\b|\\ba\\b")); __args.push(span.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:takes|so)\\b")); __args.push(sentence.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return sentence.clone();
    }
    return geti(&wanted.clone(), &Py::Int(0));
    }
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:[^,.;:]{0,18}?)\\bfor\\b")); __args.push(sentence.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BEFORE(), "search", args))(&__args) } } }) {
    result = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RESULT(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&result.clone()) {
    return geti(&result.clone(), &op_neg(&Py::Int(1)));
    }
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SEQUENCE(), "search", args))(&__args) })) } }) {
    lead = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[:,;]\\s+|\\s+-\\s+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    lead_spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SPANS(), "get", args))(&__args) }, "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); } py_list(__out) })() };
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spans.clone()) { let mut span: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&lead_spans.clone(), &span.clone()))) {  __out.push(span.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead_spans.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut span: Py = __item.clone(); __out.push(Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(span.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _measure(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&lead_spans.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _measure(args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return geti(&lead_spans.clone(), &Py::Int(0));
    }
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spans.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&spans.clone(), &Py::Int(0));
    }
    stepped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(question.clone()); __args.push(figure.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _step_time(args))(&__args) };
    if py_truthy(&stepped.clone()) {
    return stepped.clone();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SEQUENCE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) })) } }) {
    return geti(&spans.clone(), &op_neg(&Py::Int(1)));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_thing(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CARRIES_ON(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bper\\b")); __args.push(geti(&spans.clone(), &Py::Int(0))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EITHER_OR(), "search", args))(&__args) })) } } } } } }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(geti(&spans.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clause_holding(args))(&__args) }; if py_truthy(&__b) { __b } else { geti(&spans.clone(), &Py::Int(0)) } };
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clause(args))(&__args) }; if py_truthy(&__b) { __b } else { sentence.clone() } };
    Py::None
}
pub fn _canonical(args: &[Py]) -> Py {
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
    let _ = py_str("The step a kitchen word names, so the oven is the oven however the person says it.");
    return if py_truthy(&Py::Bool(py_contains(&g_OVEN(), &word.clone()))) { py_str("oven") } else { word.clone() };
    Py::None
}
pub fn _a_frequency(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut start: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { Py::None };
    let mut after: Py = Py::None;
    let _ = py_str("Whether the time that ends at `end` is how often to do something, not how long it takes.\n    \"Twenty minutes a day\", \"5 days a week\" and \"three times a week\" are all rates: asked how\n    long the job takes, the figure to hand back is the length in the sentence (\"two or three\n    months\"), and a rate handed back is a wrong answer a person plans around.\n    ");
    after = slice_or_raise(&sentence.clone(), &end.clone(), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&start.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&sentence.clone(), &Py::None, &start.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_BEFORE(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_AFTER(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_AFTER_TIMES(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREQUENCY_AFTER_AT_A_TIME(), "match", args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _same_step(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut long_: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut short: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the question and the clause name the same step.\n\n    \"parboil\" is the \"boil\" the clause is about: the person's word has the answer's word in it,\n    so a prefix or a suffix of at least two letters is the same step (\"parboil\" is \"boil\",\n    \"roasting\" is \"roast\"), and one extra letter is a different word (\"rice\" is not \"price\",\n    \"ice\" is not \"slice\"). Two words for one step count as one word: \"bake\" and \"in the oven\"\n    are the same step, and so are \"roast\" and \"in the oven\", which is how a question about\n    baking picks the oven time out of a sentence that also gives the hob time.\n    ");
    if py_truthy(&op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _canonical(args))(&__args) }); } py_set(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _canonical(args))(&__args) }); } py_set(__out) })() })) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&asked.clone());
    for __item in __iter1 {
    mine = __item.clone();
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    theirs = __item.clone();
    let __unpk3 = py_unpack(&if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(theirs.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { py_tuple(vec![mine.clone(), theirs.clone()]) } else { py_tuple(vec![theirs.clone(), mine.clone()]) }, 2, None);
    short = __unpk3[0].clone();
    long_ = __unpk3[1].clone();
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(short.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(long_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(short.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&long_.clone(), &short.clone())) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _step_time(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut figure: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut asked: Py = Py::None;
    let mut cooking: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut places: Py = Py::None;
    let mut waiting: Py = Py::None;
    let mut waiting_here: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The time in the clause about the step the question names, or None.\n\n    \"12 minutes on a low simmer, then 10 minutes off the heat\" gives the time for cooking and\n    the time for standing, and asking how long to cook it must not come back with the standing\n    time. The verb the question uses picks the clause. A question that names no step still never\n    wants the standing time: what it asked is how long the job takes, and waiting afterwards is\n    not the job.\n    ");
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) }); } } py_set(__out) })() };
    pieces = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STEP(), "split", args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "strip", args))(&__args) }); } } py_list(__out) })() });
    for __item in __iter1 {
    piece = __item.clone();
    if py_truthy(&{ let __b = pieces.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_UNIT(), "match", args))(&__args) } } }) {
    let _ = py_setitem(&pieces.clone(), &op_neg(&Py::Int(1)), op_add(&op_add(&geti(&pieces.clone(), &op_neg(&Py::Int(1))), &py_str(" and ")), &piece.clone()));
    } else {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pieces.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SEQUENCE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER(), "search", args))(&__args) } } }))) {
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_WHERE(), &word.clone()))) {  __out.push(word.clone()); } } } py_set(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(places.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), py_list(vec![])]), 3, None);
    named = __unpk2[0].clone();
    cooking = __unpk2[1].clone();
    waiting = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&pieces.clone());
    for __item in __iter3 {
    piece = __item.clone();
    found = if py_truthy(&figure.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&figure.clone(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _figure(args))(&__args) }); } py_list(__out) })() } } else { py_list(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KEEPS(), "search", args))(&__args) }) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) }); } } py_set(__out) })() };
    waiting_here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&found.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&waiting_here.clone()) { waiting.clone() } else { cooking.clone() }, "append", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same_step(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = waiting_here.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) })) } })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&found.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SEQUENCE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AFTER(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { named.clone() } } } }))) {
    return Py::None;
    }
    if py_truthy(&asked.clone()) {
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&named.clone(), &Py::Int(0));
    }
    if py_truthy(&named.clone()) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WAIT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { waiting.clone() } }) {
    return geti(&waiting.clone(), &op_neg(&Py::Int(1)));
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cooking.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) {
    return Py::None;
    }
    if py_truthy(&cooking.clone()) {
    return geti(&cooking.clone(), &op_neg(&Py::Int(1)));
    }
    return if py_truthy(&waiting.clone()) { geti(&waiting.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    Py::None
}
pub fn _aside_held(args: &[Py]) -> Py {
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
    let mut hide: Py = Py::None;
    let mut kept: Py = Py::None;
    let _ = py_str("The text with each aside replaced by a placeholder, and the asides, to put back.");
    kept = py_list(vec![]);
pub fn __nested_hide_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut hide: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut kept: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    return op_modulo(&py_str(" \u{01}%d\u{01} "), &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)));
    Py::None
}
    hide = {{ let hide = hide.clone(); let kept = kept.clone(); let text = text.clone(); py_func("hide", move |args: &[Py]| { let mut full = vec![hide.clone(), kept.clone(), text.clone()]; full.extend_from_slice(args); __nested_hide_0(&full) }) }};
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(hide.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASIDE_ON(), "sub", args))(&__args) }, kept.clone()]);
    Py::None
}
pub fn _aside_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut kept: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let kept = kept.clone(); let piece = piece.clone(); py_func("lambda", move |args: &[Py]| { let kept = kept.clone(); let piece = piece.clone(); let match_: Py = __kwp(args, 0, "match").unwrap_or(Py::None); geti(&kept.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) }) }); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HELD(), "sub", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn clauses(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let _ = py_str("The pieces of `text` cut at the separators in `pattern` that are not inside an aside.\n\n    Every reader that narrows an answer to one of its clauses cuts the sentence at a comma, a\n    semicolon or a conjunction, and they all took a dash aside apart the same way. The cut is\n    shared so the aside is kept whole by all of them, in the words it was written in.\n    ");
    return geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(pattern.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| clauses_bare(args))(&__args) }, &Py::Int(0));
    Py::None
}
pub fn clauses_bare(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut bare: Py = Py::None;
    let mut dressed: Py = Py::None;
    let mut held: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let _ = py_str("The pieces of `clauses()`, and the same pieces with their asides left out.\n\n    The bare form is what a piece is *about*: a remark between two dashes says something about\n    the clause, it is not what the clause is about (\"a complete cat food - wet, or wet with a\n    little dry - and weigh the day's portion\" is about feeding wet, and the aside mentions dry\n    without giving an amount for it). Reading the words of a piece without its asides keeps a\n    question about the aside's own word (\"how much dry food?\") from settling on the piece that\n    only mentions it, and the answer falls back to the whole sentence instead.\n    ");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _aside_held(args))(&__args) }, 2, None);
    held = __unpk1[0].clone();
    kept = __unpk1[1].clone();
    let __unpk2 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    pieces = __unpk2[0].clone();
    bare = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(pattern.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pieces_of(args))(&__args) });
    for __item in __iter3 {
    piece = __item.clone();
    dressed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _aside_back(args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&dressed.clone()))) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(dressed.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pieces.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HELD(), "sub", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "append", args))(&__args) };
    }
    return py_tuple(vec![pieces.clone(), bare.clone()]);
    Py::None
}
pub fn _clause(args: &[Py]) -> Py {
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
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut bare: Py = Py::None;
    let mut best: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut scores: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The clause that shares the most words with the question, when that is a real narrowing.");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(question.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_CLAUSE_BREAK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| clauses_bare(args))(&__args) }, 2, None);
    pieces = __unpk1[0].clone();
    bare = __unpk1[1].clone();
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pieces.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::None;
    }
    scores = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&bare.clone()) { let mut piece: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&wanted.clone(), "intersection", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(scores.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = best.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "count", args))(&__args) }, &Py::Int(1))) } }) {
    piece = geti(&pieces.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(best.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "index", args))(&__args) });
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(piece.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEANING(), "match", args))(&__args) }) { Py::None } else { piece.clone() };
    }
    return Py::None;
    Py::None
}
pub fn _clause_with(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut hits: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let _ = py_str("The clause of `text` that says `word`, when it says it in exactly one.");
    pieces = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_CLAUSE_BREAK_COLON()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| clauses(args))(&__args) };
    hits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(piece.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(piece.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(hits.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) { geti(&hits.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _clause_holding(args: &[Py]) -> Py {
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
    let mut hits: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let _ = py_str("The one clause of the sentence that carries this figure, when one clause carries it alone.\n\n    \"Raw chicken keeps 1 to 2 days in the fridge, on the bottom shelf, and cooked chicken 3 days\"\n    gives a figure per food, so the figure on its own - \"1 to 2 days.\" - is read as the answer for\n    whichever food the person has. The clause keeps the subject with the number: \"Raw chicken\n    keeps 1 to 2 days in the fridge.\"\n    ");
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_CLAUSE_BREAK_COLON()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pieces_of(args))(&__args) }) { let mut piece: Py = __item.clone(); if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) } } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&piece.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    hits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&piece.clone(), &span.clone()))) {  __out.push(piece.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(hits.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) { geti(&hits.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn words_of(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The content words of `text`, without fillers or plurals.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD(), "findall", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EMPTY(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    }
    return found.clone();
    Py::None
}
pub fn flat(args: &[Py]) -> Py {
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn tidy(args: &[Py]) -> Py {
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
    let _ = py_str("One clean line: no trailing comma, no lowercase start, no double spaces.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&text.clone(), &Py::Int(0)), "islower", args))(&__args) }) {
    text = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&text.clone(), &Py::Int(0)), "upper", args))(&__args) }, &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("."), py_str("!"), py_str("?")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "endswith", args))(&__args) }))) {
    let __aug1 = op_add(&text.clone(), &py_str("."));
    text = __aug1;
    }
    return text.clone();
    Py::None
}
pub fn _NoLength____bool__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    return Py::Bool(false);
    Py::None
}
pub fn _NoLength____repr__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    return py_str("NO_LENGTH");
    Py::None
}
pub fn _NoLength(args: &[Py]) -> Py {
    ensure_init();
    let inst = Py::Instance(Rc::new(PyInstance { class_name: "_NoLength".to_string(), attrs: Rc::new(RefCell::new(IndexMap::new())) }));
    let mut full = vec![inst.clone()]; full.extend_from_slice(args);
    _NoLength____init__(&full);
    inst
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.narrow".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER" => Some(g_AFTER()),
            "AMOUNT_UNIT" => Some(g_AMOUNT_UNIT()),
            "ASIDE" => Some(g_ASIDE()),
            "ASIDE_ON" => Some(g_ASIDE_ON()),
            "ASKED_UNIT" => Some(g_ASKED_UNIT()),
            "ASKING_ONLY" => Some(g_ASKING_ONLY()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_ABOUT_IT" => Some(g_ASKS_ABOUT_IT()),
            "ASKS_FOR_A_HAND" => Some(g_ASKS_FOR_A_HAND()),
            "ASKS_FREQUENCY" => Some(g_ASKS_FREQUENCY()),
            "ASKS_THE_DATE_ITSELF" => Some(g_ASKS_THE_DATE_ITSELF()),
            "AT_A_TIME" => Some(g_AT_A_TIME()),
            "A_BARE_COUNT" => Some(g_A_BARE_COUNT()),
            "A_BARE_RANGE" => Some(g_A_BARE_RANGE()),
            "A_BOUNDED" => Some(g_A_BOUNDED()),
            "A_COUNTDOWN" => Some(g_A_COUNTDOWN()),
            "A_FOR_A_THING" => Some(g_A_FOR_A_THING()),
            "A_HOW_OFTEN_THE_DOING" => Some(g_A_HOW_OFTEN_THE_DOING()),
            "A_LENGTH" => Some(g_A_LENGTH()),
            "A_MOMENT" => Some(g_A_MOMENT()),
            "A_PERIOD_IN_A_LINE" => Some(g_A_PERIOD_IN_A_LINE()),
            "A_RATE_ON_A_FIGURE" => Some(g_A_RATE_ON_A_FIGURE()),
            "A_RELATIVE_AFTER" => Some(g_A_RELATIVE_AFTER()),
            "A_SET_OF_STEPS" => Some(g_A_SET_OF_STEPS()),
            "A_STAGE" => Some(g_A_STAGE()),
            "A_START_LEAD" => Some(g_A_START_LEAD()),
            "A_STATE_FOR" => Some(g_A_STATE_FOR()),
            "A_UNIT_WORD" => Some(g_A_UNIT_WORD()),
            "A_VALIDITY_ASK" => Some(g_A_VALIDITY_ASK()),
            "A_WEATHER_WINDOW" => Some(g_A_WEATHER_WINDOW()),
            "A_WHEN_WORD" => Some(g_A_WHEN_WORD()),
            "BAREST_VERDICT" => Some(g_BAREST_VERDICT()),
            "BARE_RANGE" => Some(g_BARE_RANGE()),
            "BARE_UNIT" => Some(g_BARE_UNIT()),
            "BEFORE" => Some(g_BEFORE()),
            "CARRIES_ON" => Some(g_CARRIES_ON()),
            "CAUSE" => Some(g_CAUSE()),
            "CLAUSE_BREAK" => Some(g_CLAUSE_BREAK()),
            "CLAUSE_BREAK_COLON" => Some(g_CLAUSE_BREAK_COLON()),
            "CLOCK" => Some(g_CLOCK()),
            "COOKS" => Some(g_COOKS()),
            "DATED_STRONG" => Some(g_DATED_STRONG()),
            "DAY_OR_YEAR" => Some(g_DAY_OR_YEAR()),
            "DONENESS" => Some(g_DONENESS()),
            "DURATION_ADJECTIVE" => Some(g_DURATION_ADJECTIVE()),
            "EITHER_OR" => Some(g_EITHER_OR()),
            "EMPTY" => Some(g_EMPTY()),
            "FIGURE_MEASURE" => Some(g_FIGURE_MEASURE()),
            "FOLLOW_UP" => Some(g_FOLLOW_UP()),
            "FREQUENCY" => Some(g_FREQUENCY()),
            "FREQUENCY_AFTER" => Some(g_FREQUENCY_AFTER()),
            "FREQUENCY_AFTER_AT_A_TIME" => Some(g_FREQUENCY_AFTER_AT_A_TIME()),
            "FREQUENCY_AFTER_TIMES" => Some(g_FREQUENCY_AFTER_TIMES()),
            "FREQUENCY_BEFORE" => Some(g_FREQUENCY_BEFORE()),
            "FREQUENCY_IGNORE" => Some(g_FREQUENCY_IGNORE()),
            "FREQ_CLAUSE" => Some(g_FREQ_CLAUSE()),
            "FREQ_LIMIT" => Some(g_FREQ_LIMIT()),
            "FREQ_PORTION" => Some(g_FREQ_PORTION()),
            "FULL_LENGTH" => Some(g_FULL_LENGTH()),
            "HELD" => Some(g_HELD()),
            "IN_A_CONDITION" => Some(g_IN_A_CONDITION()),
            "IN_A_RELATIVE" => Some(g_IN_A_RELATIVE()),
            "JOINS" => Some(g_JOINS()),
            "KEEPS" => Some(g_KEEPS()),
            "LEANING" => Some(g_LEANING()),
            "LENGTH_FILLER" => Some(g_LENGTH_FILLER()),
            "LENGTH_NAMED_DAYS" => Some(g_LENGTH_NAMED_DAYS()),
            "LENGTH_OF_THE_JOB" => Some(g_LENGTH_OF_THE_JOB()),
            "MONTHS_LONG" => Some(g_MONTHS_LONG()),
            "NAMED_BREAK" => Some(g_NAMED_BREAK()),
            "NAMES_A_PART" => Some(g_NAMES_A_PART()),
            "NAMING_IGNORE" => Some(g_NAMING_IGNORE()),
            "NOT_OPTION" => Some(g_NOT_OPTION()),
            "NOT_RELATIVE" => Some(g_NOT_RELATIVE()),
            "NO_INTERVAL_SAID" => Some(g_NO_INTERVAL_SAID()),
            "NO_LENGTH" => Some(g_NO_LENGTH()),
            "NO_SET_TIME" => Some(g_NO_SET_TIME()),
            "NUM_RANGE" => Some(g_NUM_RANGE()),
            "ONLY_THE_INTERVAL_ASK" => Some(g_ONLY_THE_INTERVAL_ASK()),
            "OPTION" => Some(g_OPTION()),
            "OUTCOME_ASKS" => Some(g_OUTCOME_ASKS()),
            "OVEN" => Some(g_OVEN()),
            "OWN_SAVING" => Some(g_OWN_SAVING()),
            "PAPERWORK" => Some(g_PAPERWORK()),
            "PLACE" => Some(g_PLACE()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "QUALIFIER" => Some(g_QUALIFIER()),
            "QUALIFIER_FILLER" => Some(g_QUALIFIER_FILLER()),
            "QUALIFIER_WORDS" => Some(g_QUALIFIER_WORDS()),
            "QUOTED_WORDS" => Some(g_QUOTED_WORDS()),
            "QUOTED_WORDS_SINGLE" => Some(g_QUOTED_WORDS_SINGLE()),
            "RANGE_JOIN" => Some(g_RANGE_JOIN()),
            "RATE" => Some(g_RATE()),
            "RATE_FREQ" => Some(g_RATE_FREQ()),
            "RATE_INTERVAL" => Some(g_RATE_INTERVAL()),
            "RATE_LEAD" => Some(g_RATE_LEAD()),
            "REMEDY" => Some(g_REMEDY()),
            "REMEDY_CUT" => Some(g_REMEDY_CUT()),
            "REMEDY_JOINER" => Some(g_REMEDY_JOINER()),
            "RESULT" => Some(g_RESULT()),
            "SAME" => Some(g_SAME()),
            "SAYS_SOMETHING" => Some(g_SAYS_SOMETHING()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SEQUENCE" => Some(g_SEQUENCE()),
            "SIGNALS" => Some(g_SIGNALS()),
            "SPANS" => Some(g_SPANS()),
            "SPAN_FILLER" => Some(g_SPAN_FILLER()),
            "STEP" => Some(g_STEP()),
            "STEP_VERB" => Some(g_STEP_VERB()),
            "SYMPTOM" => Some(g_SYMPTOM()),
            "THOUSANDS" => Some(g_THOUSANDS()),
            "THOUSANDS_JOIN" => Some(g_THOUSANDS_JOIN()),
            "TRIGGER" => Some(g_TRIGGER()),
            "TRIGGER_PATTERN" => Some(g_TRIGGER_PATTERN()),
            "UNIT" => Some(g_UNIT()),
            "URGENT" => Some(g_URGENT()),
            "WAIT" => Some(g_WAIT()),
            "WEEKDAY_OR_CLOCK" => Some(g_WEEKDAY_OR_CLOCK()),
            "WHERE" => Some(g_WHERE()),
            "WORD" => Some(g_WORD()),
            "YES_NO" => Some(g_YES_NO()),
            "_A_BARE_FIGURE" => Some(g__A_BARE_FIGURE()),
            "_A_CUT_OFF_AFTER" => Some(g__A_CUT_OFF_AFTER()),
            "_A_CUT_OFF_BEFORE" => Some(g__A_CUT_OFF_BEFORE()),
            "_A_DOING" => Some(g__A_DOING()),
            "_A_DURATION_VERB" => Some(g__A_DURATION_VERB()),
            "_A_PERIOD_OF_EXTENT" => Some(g__A_PERIOD_OF_EXTENT()),
            "_A_RANGE" => Some(g__A_RANGE()),
            "_COUNTED_UNIT" => Some(g__COUNTED_UNIT()),
            "_IN_MONTHS_OR_YEARS" => Some(g__IN_MONTHS_OR_YEARS()),
            "_NoLength" => Some(py_func("_NoLength", |args: &[Py]| _NoLength(args))),
            "_STATE_LENGTH_ASKS" => Some(g__STATE_LENGTH_ASKS()),
            "_UNITS" => Some(g__UNITS()),
            "_a_clause_before" => Some(py_func("_a_clause_before", |args: &[Py]| _a_clause_before(args))),
            "_a_condition" => Some(py_func("_a_condition", |args: &[Py]| _a_condition(args))),
            "_a_cut_off_not_a_length" => Some(py_func("_a_cut_off_not_a_length", |args: &[Py]| _a_cut_off_not_a_length(args))),
            "_a_figure_each" => Some(py_func("_a_figure_each", |args: &[Py]| _a_figure_each(args))),
            "_a_frequency" => Some(py_func("_a_frequency", |args: &[Py]| _a_frequency(args))),
            "_a_length_of_a_doing" => Some(py_func("_a_length_of_a_doing", |args: &[Py]| _a_length_of_a_doing(args))),
            "_a_period_of_extent" => Some(py_func("_a_period_of_extent", |args: &[Py]| _a_period_of_extent(args))),
            "_a_rate_cut_from_its_figure" => Some(py_func("_a_rate_cut_from_its_figure", |args: &[Py]| _a_rate_cut_from_its_figure(args))),
            "_as_a_length" => Some(py_func("_as_a_length", |args: &[Py]| _as_a_length(args))),
            "_aside_back" => Some(py_func("_aside_back", |args: &[Py]| _aside_back(args))),
            "_aside_held" => Some(py_func("_aside_held", |args: &[Py]| _aside_held(args))),
            "_at_another_place" => Some(py_func("_at_another_place", |args: &[Py]| _at_another_place(args))),
            "_by_option" => Some(py_func("_by_option", |args: &[Py]| _by_option(args))),
            "_by_step" => Some(py_func("_by_step", |args: &[Py]| _by_step(args))),
            "_by_words" => Some(py_func("_by_words", |args: &[Py]| _by_words(args))),
            "_canonical" => Some(py_func("_canonical", |args: &[Py]| _canonical(args))),
            "_clause" => Some(py_func("_clause", |args: &[Py]| _clause(args))),
            "_clause_holding" => Some(py_func("_clause_holding", |args: &[Py]| _clause_holding(args))),
            "_clause_with" => Some(py_func("_clause_with", |args: &[Py]| _clause_with(args))),
            "_counts_on_the_answer" => Some(py_func("_counts_on_the_answer", |args: &[Py]| _counts_on_the_answer(args))),
            "_cuts_a_list" => Some(py_func("_cuts_a_list", |args: &[Py]| _cuts_a_list(args))),
            "_dated" => Some(py_func("_dated", |args: &[Py]| _dated(args))),
            "_figure" => Some(py_func("_figure", |args: &[Py]| _figure(args))),
            "_freq_clause" => Some(py_func("_freq_clause", |args: &[Py]| _freq_clause(args))),
            "_gives_a_figure_in" => Some(py_func("_gives_a_figure_in", |args: &[Py]| _gives_a_figure_in(args))),
            "_gives_another_places_figure" => Some(py_func("_gives_another_places_figure", |args: &[Py]| _gives_another_places_figure(args))),
            "_how_often" => Some(py_func("_how_often", |args: &[Py]| _how_often(args))),
            "_informative" => Some(py_func("_informative", |args: &[Py]| _informative(args))),
            "_measure" => Some(py_func("_measure", |args: &[Py]| _measure(args))),
            "_named_clause" => Some(py_func("_named_clause", |args: &[Py]| _named_clause(args))),
            "_named_follow_up" => Some(py_func("_named_follow_up", |args: &[Py]| _named_follow_up(args))),
            "_names_a_thing" => Some(py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args))),
            "_names_one_of" => Some(py_func("_names_one_of", |args: &[Py]| _names_one_of(args))),
            "_narrow" => Some(py_func("_narrow", |args: &[Py]| _narrow(args))),
            "_not_a_figure" => Some(py_func("_not_a_figure", |args: &[Py]| _not_a_figure(args))),
            "_only_a_count" => Some(py_func("_only_a_count", |args: &[Py]| _only_a_count(args))),
            "_only_a_cut_off" => Some(py_func("_only_a_cut_off", |args: &[Py]| _only_a_cut_off(args))),
            "_only_a_qualifier" => Some(py_func("_only_a_qualifier", |args: &[Py]| _only_a_qualifier(args))),
            "_option_ask" => Some(py_func("_option_ask", |args: &[Py]| _option_ask(args))),
            "_part" => Some(py_func("_part", |args: &[Py]| _part(args))),
            "_property_ask" => Some(py_func("_property_ask", |args: &[Py]| _property_ask(args))),
            "_root" => Some(py_func("_root", |args: &[Py]| _root(args))),
            "_same" => Some(py_func("_same", |args: &[Py]| _same(args))),
            "_same_step" => Some(py_func("_same_step", |args: &[Py]| _same_step(args))),
            "_same_word" => Some(py_func("_same_word", |args: &[Py]| _same_word(args))),
            "_says" => Some(py_func("_says", |args: &[Py]| _says(args))),
            "_says_a_time" => Some(py_func("_says_a_time", |args: &[Py]| _says_a_time(args))),
            "_stands_alone" => Some(py_func("_stands_alone", |args: &[Py]| _stands_alone(args))),
            "_states_a_length" => Some(py_func("_states_a_length", |args: &[Py]| _states_a_length(args))),
            "_step_time" => Some(py_func("_step_time", |args: &[Py]| _step_time(args))),
            "_the_answer_said_again" => Some(py_func("_the_answer_said_again", |args: &[Py]| _the_answer_said_again(args))),
            "_the_clause_around" => Some(py_func("_the_clause_around", |args: &[Py]| _the_clause_around(args))),
            "_the_clause_in_the_unit" => Some(py_func("_the_clause_in_the_unit", |args: &[Py]| _the_clause_in_the_unit(args))),
            "_the_half_they_named" => Some(py_func("_the_half_they_named", |args: &[Py]| _the_half_they_named(args))),
            "_the_item_they_named" => Some(py_func("_the_item_they_named", |args: &[Py]| _the_item_they_named(args))),
            "_the_part_they_named" => Some(py_func("_the_part_they_named", |args: &[Py]| _the_part_they_named(args))),
            "_the_quotation" => Some(py_func("_the_quotation", |args: &[Py]| _the_quotation(args))),
            "_the_remedy_in" => Some(py_func("_the_remedy_in", |args: &[Py]| _the_remedy_in(args))),
            "_the_sentence_a_figure_hangs_on" => Some(py_func("_the_sentence_a_figure_hangs_on", |args: &[Py]| _the_sentence_a_figure_hangs_on(args))),
            "_the_sentence_in_the_unit" => Some(py_func("_the_sentence_in_the_unit", |args: &[Py]| _the_sentence_in_the_unit(args))),
            "_the_start_age" => Some(py_func("_the_start_age", |args: &[Py]| _the_start_age(args))),
            "_the_time_it_does_keep" => Some(py_func("_the_time_it_does_keep", |args: &[Py]| _the_time_it_does_keep(args))),
            "_to_the_end_of_the_script" => Some(py_func("_to_the_end_of_the_script", |args: &[Py]| _to_the_end_of_the_script(args))),
            "_unit_stem" => Some(py_func("_unit_stem", |args: &[Py]| _unit_stem(args))),
            "_with_the_script" => Some(py_func("_with_the_script", |args: &[Py]| _with_the_script(args))),
            "_with_unit" => Some(py_func("_with_unit", |args: &[Py]| _with_unit(args))),
            "asked_unit" => Some(py_func("asked_unit", |args: &[Py]| asked_unit(args))),
            "asks_a_part" => Some(py_func("asks_a_part", |args: &[Py]| asks_a_part(args))),
            "carries" => Some(py_func("carries", |args: &[Py]| carries(args))),
            "clauses" => Some(py_func("clauses", |args: &[Py]| clauses(args))),
            "clauses_bare" => Some(py_func("clauses_bare", |args: &[Py]| clauses_bare(args))),
            "flat" => Some(py_func("flat", |args: &[Py]| flat(args))),
            "has_a_date" => Some(py_func("has_a_date", |args: &[Py]| has_a_date(args))),
            "kind_of" => Some(py_func("kind_of", |args: &[Py]| kind_of(args))),
            "narrow" => Some(py_func("narrow", |args: &[Py]| narrow(args))),
            "only_a_length" => Some(py_func("only_a_length", |args: &[Py]| only_a_length(args))),
            "part_in_the_unit" => Some(py_func("part_in_the_unit", |args: &[Py]| part_in_the_unit(args))),
            "pieces_of" => Some(py_func("pieces_of", |args: &[Py]| pieces_of(args))),
            "step_word" => Some(py_func("step_word", |args: &[Py]| step_word(args))),
            "the_doings_length" => Some(py_func("the_doings_length", |args: &[Py]| the_doings_length(args))),
            "the_outcome_asked" => Some(py_func("the_outcome_asked", |args: &[Py]| the_outcome_asked(args))),
            "the_part_asked" => Some(py_func("the_part_asked", |args: &[Py]| the_part_asked(args))),
            "tidy" => Some(py_func("tidy", |args: &[Py]| tidy(args))),
            "words_of" => Some(py_func("words_of", |args: &[Py]| words_of(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFTER" => s_AFTER(v),
            "AMOUNT_UNIT" => s_AMOUNT_UNIT(v),
            "ASIDE" => s_ASIDE(v),
            "ASIDE_ON" => s_ASIDE_ON(v),
            "ASKED_UNIT" => s_ASKED_UNIT(v),
            "ASKING_ONLY" => s_ASKING_ONLY(v),
            "ASKS" => s_ASKS(v),
            "ASKS_ABOUT_IT" => s_ASKS_ABOUT_IT(v),
            "ASKS_FOR_A_HAND" => s_ASKS_FOR_A_HAND(v),
            "ASKS_FREQUENCY" => s_ASKS_FREQUENCY(v),
            "ASKS_THE_DATE_ITSELF" => s_ASKS_THE_DATE_ITSELF(v),
            "AT_A_TIME" => s_AT_A_TIME(v),
            "A_BARE_COUNT" => s_A_BARE_COUNT(v),
            "A_BARE_RANGE" => s_A_BARE_RANGE(v),
            "A_BOUNDED" => s_A_BOUNDED(v),
            "A_COUNTDOWN" => s_A_COUNTDOWN(v),
            "A_FOR_A_THING" => s_A_FOR_A_THING(v),
            "A_HOW_OFTEN_THE_DOING" => s_A_HOW_OFTEN_THE_DOING(v),
            "A_LENGTH" => s_A_LENGTH(v),
            "A_MOMENT" => s_A_MOMENT(v),
            "A_PERIOD_IN_A_LINE" => s_A_PERIOD_IN_A_LINE(v),
            "A_RATE_ON_A_FIGURE" => s_A_RATE_ON_A_FIGURE(v),
            "A_RELATIVE_AFTER" => s_A_RELATIVE_AFTER(v),
            "A_SET_OF_STEPS" => s_A_SET_OF_STEPS(v),
            "A_STAGE" => s_A_STAGE(v),
            "A_START_LEAD" => s_A_START_LEAD(v),
            "A_STATE_FOR" => s_A_STATE_FOR(v),
            "A_UNIT_WORD" => s_A_UNIT_WORD(v),
            "A_VALIDITY_ASK" => s_A_VALIDITY_ASK(v),
            "A_WEATHER_WINDOW" => s_A_WEATHER_WINDOW(v),
            "A_WHEN_WORD" => s_A_WHEN_WORD(v),
            "BAREST_VERDICT" => s_BAREST_VERDICT(v),
            "BARE_RANGE" => s_BARE_RANGE(v),
            "BARE_UNIT" => s_BARE_UNIT(v),
            "BEFORE" => s_BEFORE(v),
            "CARRIES_ON" => s_CARRIES_ON(v),
            "CAUSE" => s_CAUSE(v),
            "CLAUSE_BREAK" => s_CLAUSE_BREAK(v),
            "CLAUSE_BREAK_COLON" => s_CLAUSE_BREAK_COLON(v),
            "CLOCK" => s_CLOCK(v),
            "COOKS" => s_COOKS(v),
            "DATED_STRONG" => s_DATED_STRONG(v),
            "DAY_OR_YEAR" => s_DAY_OR_YEAR(v),
            "DONENESS" => s_DONENESS(v),
            "DURATION_ADJECTIVE" => s_DURATION_ADJECTIVE(v),
            "EITHER_OR" => s_EITHER_OR(v),
            "EMPTY" => s_EMPTY(v),
            "FIGURE_MEASURE" => s_FIGURE_MEASURE(v),
            "FOLLOW_UP" => s_FOLLOW_UP(v),
            "FREQUENCY" => s_FREQUENCY(v),
            "FREQUENCY_AFTER" => s_FREQUENCY_AFTER(v),
            "FREQUENCY_AFTER_AT_A_TIME" => s_FREQUENCY_AFTER_AT_A_TIME(v),
            "FREQUENCY_AFTER_TIMES" => s_FREQUENCY_AFTER_TIMES(v),
            "FREQUENCY_BEFORE" => s_FREQUENCY_BEFORE(v),
            "FREQUENCY_IGNORE" => s_FREQUENCY_IGNORE(v),
            "FREQ_CLAUSE" => s_FREQ_CLAUSE(v),
            "FREQ_LIMIT" => s_FREQ_LIMIT(v),
            "FREQ_PORTION" => s_FREQ_PORTION(v),
            "FULL_LENGTH" => s_FULL_LENGTH(v),
            "HELD" => s_HELD(v),
            "IN_A_CONDITION" => s_IN_A_CONDITION(v),
            "IN_A_RELATIVE" => s_IN_A_RELATIVE(v),
            "JOINS" => s_JOINS(v),
            "KEEPS" => s_KEEPS(v),
            "LEANING" => s_LEANING(v),
            "LENGTH_FILLER" => s_LENGTH_FILLER(v),
            "LENGTH_NAMED_DAYS" => s_LENGTH_NAMED_DAYS(v),
            "LENGTH_OF_THE_JOB" => s_LENGTH_OF_THE_JOB(v),
            "MONTHS_LONG" => s_MONTHS_LONG(v),
            "NAMED_BREAK" => s_NAMED_BREAK(v),
            "NAMES_A_PART" => s_NAMES_A_PART(v),
            "NAMING_IGNORE" => s_NAMING_IGNORE(v),
            "NOT_OPTION" => s_NOT_OPTION(v),
            "NOT_RELATIVE" => s_NOT_RELATIVE(v),
            "NO_INTERVAL_SAID" => s_NO_INTERVAL_SAID(v),
            "NO_LENGTH" => s_NO_LENGTH(v),
            "NO_SET_TIME" => s_NO_SET_TIME(v),
            "NUM_RANGE" => s_NUM_RANGE(v),
            "ONLY_THE_INTERVAL_ASK" => s_ONLY_THE_INTERVAL_ASK(v),
            "OPTION" => s_OPTION(v),
            "OUTCOME_ASKS" => s_OUTCOME_ASKS(v),
            "OVEN" => s_OVEN(v),
            "OWN_SAVING" => s_OWN_SAVING(v),
            "PAPERWORK" => s_PAPERWORK(v),
            "PLACE" => s_PLACE(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "QUALIFIER" => s_QUALIFIER(v),
            "QUALIFIER_FILLER" => s_QUALIFIER_FILLER(v),
            "QUALIFIER_WORDS" => s_QUALIFIER_WORDS(v),
            "QUOTED_WORDS" => s_QUOTED_WORDS(v),
            "QUOTED_WORDS_SINGLE" => s_QUOTED_WORDS_SINGLE(v),
            "RANGE_JOIN" => s_RANGE_JOIN(v),
            "RATE" => s_RATE(v),
            "RATE_FREQ" => s_RATE_FREQ(v),
            "RATE_INTERVAL" => s_RATE_INTERVAL(v),
            "RATE_LEAD" => s_RATE_LEAD(v),
            "REMEDY" => s_REMEDY(v),
            "REMEDY_CUT" => s_REMEDY_CUT(v),
            "REMEDY_JOINER" => s_REMEDY_JOINER(v),
            "RESULT" => s_RESULT(v),
            "SAME" => s_SAME(v),
            "SAYS_SOMETHING" => s_SAYS_SOMETHING(v),
            "SENTENCE" => s_SENTENCE(v),
            "SEQUENCE" => s_SEQUENCE(v),
            "SIGNALS" => s_SIGNALS(v),
            "SPANS" => s_SPANS(v),
            "SPAN_FILLER" => s_SPAN_FILLER(v),
            "STEP" => s_STEP(v),
            "STEP_VERB" => s_STEP_VERB(v),
            "SYMPTOM" => s_SYMPTOM(v),
            "THOUSANDS" => s_THOUSANDS(v),
            "THOUSANDS_JOIN" => s_THOUSANDS_JOIN(v),
            "TRIGGER" => s_TRIGGER(v),
            "TRIGGER_PATTERN" => s_TRIGGER_PATTERN(v),
            "UNIT" => s_UNIT(v),
            "URGENT" => s_URGENT(v),
            "WAIT" => s_WAIT(v),
            "WEEKDAY_OR_CLOCK" => s_WEEKDAY_OR_CLOCK(v),
            "WHERE" => s_WHERE(v),
            "WORD" => s_WORD(v),
            "YES_NO" => s_YES_NO(v),
            "_A_BARE_FIGURE" => s__A_BARE_FIGURE(v),
            "_A_CUT_OFF_AFTER" => s__A_CUT_OFF_AFTER(v),
            "_A_CUT_OFF_BEFORE" => s__A_CUT_OFF_BEFORE(v),
            "_A_DOING" => s__A_DOING(v),
            "_A_DURATION_VERB" => s__A_DURATION_VERB(v),
            "_A_PERIOD_OF_EXTENT" => s__A_PERIOD_OF_EXTENT(v),
            "_A_RANGE" => s__A_RANGE(v),
            "_COUNTED_UNIT" => s__COUNTED_UNIT(v),
            "_IN_MONTHS_OR_YEARS" => s__IN_MONTHS_OR_YEARS(v),
            "_STATE_LENGTH_ASKS" => s__STATE_LENGTH_ASKS(v),
            "_UNITS" => s__UNITS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.narrow", module_obj);
    pyrt::xrefs::reg_fn("memory.narrow", "only_a_length", py_func("only_a_length", |args: &[Py]| only_a_length(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_remedy_in", py_func("_the_remedy_in", |args: &[Py]| _the_remedy_in(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "carries", py_func("carries", |args: &[Py]| carries(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "has_a_date", py_func("has_a_date", |args: &[Py]| has_a_date(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "pieces_of", py_func("pieces_of", |args: &[Py]| pieces_of(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_same", py_func("_same", |args: &[Py]| _same(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "step_word", py_func("step_word", |args: &[Py]| step_word(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "kind_of", py_func("kind_of", |args: &[Py]| kind_of(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "asks_a_part", py_func("asks_a_part", |args: &[Py]| asks_a_part(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_named_follow_up", py_func("_named_follow_up", |args: &[Py]| _named_follow_up(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_gives_a_figure_in", py_func("_gives_a_figure_in", |args: &[Py]| _gives_a_figure_in(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_clause_in_the_unit", py_func("_the_clause_in_the_unit", |args: &[Py]| _the_clause_in_the_unit(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_sentence_in_the_unit", py_func("_the_sentence_in_the_unit", |args: &[Py]| _the_sentence_in_the_unit(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "part_in_the_unit", py_func("part_in_the_unit", |args: &[Py]| part_in_the_unit(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "asked_unit", py_func("asked_unit", |args: &[Py]| asked_unit(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_states_a_length", py_func("_states_a_length", |args: &[Py]| _states_a_length(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "the_outcome_asked", py_func("the_outcome_asked", |args: &[Py]| the_outcome_asked(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "the_part_asked", py_func("the_part_asked", |args: &[Py]| the_part_asked(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_rate_cut_from_its_figure", py_func("_a_rate_cut_from_its_figure", |args: &[Py]| _a_rate_cut_from_its_figure(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "narrow", py_func("narrow", |args: &[Py]| narrow(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_length_of_a_doing", py_func("_a_length_of_a_doing", |args: &[Py]| _a_length_of_a_doing(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_cut_off_not_a_length", py_func("_a_cut_off_not_a_length", |args: &[Py]| _a_cut_off_not_a_length(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_clause_before", py_func("_a_clause_before", |args: &[Py]| _a_clause_before(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "the_doings_length", py_func("the_doings_length", |args: &[Py]| the_doings_length(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_clause_around", py_func("_the_clause_around", |args: &[Py]| _the_clause_around(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_at_another_place", py_func("_at_another_place", |args: &[Py]| _at_another_place(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_gives_another_places_figure", py_func("_gives_another_places_figure", |args: &[Py]| _gives_another_places_figure(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_narrow", py_func("_narrow", |args: &[Py]| _narrow(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_start_age", py_func("_the_start_age", |args: &[Py]| _the_start_age(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_time_it_does_keep", py_func("_the_time_it_does_keep", |args: &[Py]| _the_time_it_does_keep(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_period_of_extent", py_func("_a_period_of_extent", |args: &[Py]| _a_period_of_extent(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_says_a_time", py_func("_says_a_time", |args: &[Py]| _says_a_time(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_only_a_cut_off", py_func("_only_a_cut_off", |args: &[Py]| _only_a_cut_off(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_answer_said_again", py_func("_the_answer_said_again", |args: &[Py]| _the_answer_said_again(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_sentence_a_figure_hangs_on", py_func("_the_sentence_a_figure_hangs_on", |args: &[Py]| _the_sentence_a_figure_hangs_on(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_unit_stem", py_func("_unit_stem", |args: &[Py]| _unit_stem(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_same_word", py_func("_same_word", |args: &[Py]| _same_word(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_names_one_of", py_func("_names_one_of", |args: &[Py]| _names_one_of(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_only_a_count", py_func("_only_a_count", |args: &[Py]| _only_a_count(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_how_often", py_func("_how_often", |args: &[Py]| _how_often(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_counts_on_the_answer", py_func("_counts_on_the_answer", |args: &[Py]| _counts_on_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_freq_clause", py_func("_freq_clause", |args: &[Py]| _freq_clause(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_dated", py_func("_dated", |args: &[Py]| _dated(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_part_they_named", py_func("_the_part_they_named", |args: &[Py]| _the_part_they_named(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_not_a_figure", py_func("_not_a_figure", |args: &[Py]| _not_a_figure(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_named_clause", py_func("_named_clause", |args: &[Py]| _named_clause(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_root", py_func("_root", |args: &[Py]| _root(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_says", py_func("_says", |args: &[Py]| _says(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_informative", py_func("_informative", |args: &[Py]| _informative(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_cuts_a_list", py_func("_cuts_a_list", |args: &[Py]| _cuts_a_list(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_with_unit", py_func("_with_unit", |args: &[Py]| _with_unit(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_property_ask", py_func("_property_ask", |args: &[Py]| _property_ask(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_option_ask", py_func("_option_ask", |args: &[Py]| _option_ask(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_stands_alone", py_func("_stands_alone", |args: &[Py]| _stands_alone(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_by_option", py_func("_by_option", |args: &[Py]| _by_option(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_only_a_qualifier", py_func("_only_a_qualifier", |args: &[Py]| _only_a_qualifier(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_by_step", py_func("_by_step", |args: &[Py]| _by_step(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_with_the_script", py_func("_with_the_script", |args: &[Py]| _with_the_script(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_quotation", py_func("_the_quotation", |args: &[Py]| _the_quotation(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_to_the_end_of_the_script", py_func("_to_the_end_of_the_script", |args: &[Py]| _to_the_end_of_the_script(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_by_words", py_func("_by_words", |args: &[Py]| _by_words(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_as_a_length", py_func("_as_a_length", |args: &[Py]| _as_a_length(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_names_a_thing", py_func("_names_a_thing", |args: &[Py]| _names_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_measure", py_func("_measure", |args: &[Py]| _measure(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_figure", py_func("_figure", |args: &[Py]| _figure(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_figure_each", py_func("_a_figure_each", |args: &[Py]| _a_figure_each(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_item_they_named", py_func("_the_item_they_named", |args: &[Py]| _the_item_they_named(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_the_half_they_named", py_func("_the_half_they_named", |args: &[Py]| _the_half_they_named(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_condition", py_func("_a_condition", |args: &[Py]| _a_condition(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_part", py_func("_part", |args: &[Py]| _part(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_canonical", py_func("_canonical", |args: &[Py]| _canonical(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_a_frequency", py_func("_a_frequency", |args: &[Py]| _a_frequency(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_same_step", py_func("_same_step", |args: &[Py]| _same_step(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_step_time", py_func("_step_time", |args: &[Py]| _step_time(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_aside_held", py_func("_aside_held", |args: &[Py]| _aside_held(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_aside_back", py_func("_aside_back", |args: &[Py]| _aside_back(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "clauses", py_func("clauses", |args: &[Py]| clauses(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "clauses_bare", py_func("clauses_bare", |args: &[Py]| clauses_bare(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_clause", py_func("_clause", |args: &[Py]| _clause(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_clause_with", py_func("_clause_with", |args: &[Py]| _clause_with(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_clause_holding", py_func("_clause_holding", |args: &[Py]| _clause_holding(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "words_of", py_func("words_of", |args: &[Py]| words_of(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "flat", py_func("flat", |args: &[Py]| flat(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "tidy", py_func("tidy", |args: &[Py]| tidy(args)));
    pyrt::xrefs::reg_fn("memory.narrow", "_NoLength", py_func("_NoLength", |args: &[Py]| _NoLength(args)));
    pyrt::xrefs::reg_access("memory.narrow", "AFTER", g_AFTER, s_AFTER);
    pyrt::xrefs::reg_access("memory.narrow", "AMOUNT_UNIT", g_AMOUNT_UNIT, s_AMOUNT_UNIT);
    pyrt::xrefs::reg_access("memory.narrow", "ASIDE", g_ASIDE, s_ASIDE);
    pyrt::xrefs::reg_access("memory.narrow", "ASIDE_ON", g_ASIDE_ON, s_ASIDE_ON);
    pyrt::xrefs::reg_access("memory.narrow", "ASKED_UNIT", g_ASKED_UNIT, s_ASKED_UNIT);
    pyrt::xrefs::reg_access("memory.narrow", "ASKING_ONLY", g_ASKING_ONLY, s_ASKING_ONLY);
    pyrt::xrefs::reg_access("memory.narrow", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.narrow", "ASKS_ABOUT_IT", g_ASKS_ABOUT_IT, s_ASKS_ABOUT_IT);
    pyrt::xrefs::reg_access("memory.narrow", "ASKS_FOR_A_HAND", g_ASKS_FOR_A_HAND, s_ASKS_FOR_A_HAND);
    pyrt::xrefs::reg_access("memory.narrow", "ASKS_FREQUENCY", g_ASKS_FREQUENCY, s_ASKS_FREQUENCY);
    pyrt::xrefs::reg_access("memory.narrow", "ASKS_THE_DATE_ITSELF", g_ASKS_THE_DATE_ITSELF, s_ASKS_THE_DATE_ITSELF);
    pyrt::xrefs::reg_access("memory.narrow", "AT_A_TIME", g_AT_A_TIME, s_AT_A_TIME);
    pyrt::xrefs::reg_access("memory.narrow", "A_BARE_COUNT", g_A_BARE_COUNT, s_A_BARE_COUNT);
    pyrt::xrefs::reg_access("memory.narrow", "A_BARE_RANGE", g_A_BARE_RANGE, s_A_BARE_RANGE);
    pyrt::xrefs::reg_access("memory.narrow", "A_BOUNDED", g_A_BOUNDED, s_A_BOUNDED);
    pyrt::xrefs::reg_access("memory.narrow", "A_COUNTDOWN", g_A_COUNTDOWN, s_A_COUNTDOWN);
    pyrt::xrefs::reg_access("memory.narrow", "A_FOR_A_THING", g_A_FOR_A_THING, s_A_FOR_A_THING);
    pyrt::xrefs::reg_access("memory.narrow", "A_HOW_OFTEN_THE_DOING", g_A_HOW_OFTEN_THE_DOING, s_A_HOW_OFTEN_THE_DOING);
    pyrt::xrefs::reg_access("memory.narrow", "A_LENGTH", g_A_LENGTH, s_A_LENGTH);
    pyrt::xrefs::reg_access("memory.narrow", "A_MOMENT", g_A_MOMENT, s_A_MOMENT);
    pyrt::xrefs::reg_access("memory.narrow", "A_PERIOD_IN_A_LINE", g_A_PERIOD_IN_A_LINE, s_A_PERIOD_IN_A_LINE);
    pyrt::xrefs::reg_access("memory.narrow", "A_RATE_ON_A_FIGURE", g_A_RATE_ON_A_FIGURE, s_A_RATE_ON_A_FIGURE);
    pyrt::xrefs::reg_access("memory.narrow", "A_RELATIVE_AFTER", g_A_RELATIVE_AFTER, s_A_RELATIVE_AFTER);
    pyrt::xrefs::reg_access("memory.narrow", "A_SET_OF_STEPS", g_A_SET_OF_STEPS, s_A_SET_OF_STEPS);
    pyrt::xrefs::reg_access("memory.narrow", "A_STAGE", g_A_STAGE, s_A_STAGE);
    pyrt::xrefs::reg_access("memory.narrow", "A_START_LEAD", g_A_START_LEAD, s_A_START_LEAD);
    pyrt::xrefs::reg_access("memory.narrow", "A_STATE_FOR", g_A_STATE_FOR, s_A_STATE_FOR);
    pyrt::xrefs::reg_access("memory.narrow", "A_UNIT_WORD", g_A_UNIT_WORD, s_A_UNIT_WORD);
    pyrt::xrefs::reg_access("memory.narrow", "A_VALIDITY_ASK", g_A_VALIDITY_ASK, s_A_VALIDITY_ASK);
    pyrt::xrefs::reg_access("memory.narrow", "A_WEATHER_WINDOW", g_A_WEATHER_WINDOW, s_A_WEATHER_WINDOW);
    pyrt::xrefs::reg_access("memory.narrow", "A_WHEN_WORD", g_A_WHEN_WORD, s_A_WHEN_WORD);
    pyrt::xrefs::reg_access("memory.narrow", "BAREST_VERDICT", g_BAREST_VERDICT, s_BAREST_VERDICT);
    pyrt::xrefs::reg_access("memory.narrow", "BARE_RANGE", g_BARE_RANGE, s_BARE_RANGE);
    pyrt::xrefs::reg_access("memory.narrow", "BARE_UNIT", g_BARE_UNIT, s_BARE_UNIT);
    pyrt::xrefs::reg_access("memory.narrow", "BEFORE", g_BEFORE, s_BEFORE);
    pyrt::xrefs::reg_access("memory.narrow", "CARRIES_ON", g_CARRIES_ON, s_CARRIES_ON);
    pyrt::xrefs::reg_access("memory.narrow", "CAUSE", g_CAUSE, s_CAUSE);
    pyrt::xrefs::reg_access("memory.narrow", "CLAUSE_BREAK", g_CLAUSE_BREAK, s_CLAUSE_BREAK);
    pyrt::xrefs::reg_access("memory.narrow", "CLAUSE_BREAK_COLON", g_CLAUSE_BREAK_COLON, s_CLAUSE_BREAK_COLON);
    pyrt::xrefs::reg_access("memory.narrow", "CLOCK", g_CLOCK, s_CLOCK);
    pyrt::xrefs::reg_access("memory.narrow", "COOKS", g_COOKS, s_COOKS);
    pyrt::xrefs::reg_access("memory.narrow", "DATED_STRONG", g_DATED_STRONG, s_DATED_STRONG);
    pyrt::xrefs::reg_access("memory.narrow", "DAY_OR_YEAR", g_DAY_OR_YEAR, s_DAY_OR_YEAR);
    pyrt::xrefs::reg_access("memory.narrow", "DONENESS", g_DONENESS, s_DONENESS);
    pyrt::xrefs::reg_access("memory.narrow", "DURATION_ADJECTIVE", g_DURATION_ADJECTIVE, s_DURATION_ADJECTIVE);
    pyrt::xrefs::reg_access("memory.narrow", "EITHER_OR", g_EITHER_OR, s_EITHER_OR);
    pyrt::xrefs::reg_access("memory.narrow", "EMPTY", g_EMPTY, s_EMPTY);
    pyrt::xrefs::reg_access("memory.narrow", "FIGURE_MEASURE", g_FIGURE_MEASURE, s_FIGURE_MEASURE);
    pyrt::xrefs::reg_access("memory.narrow", "FOLLOW_UP", g_FOLLOW_UP, s_FOLLOW_UP);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY", g_FREQUENCY, s_FREQUENCY);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY_AFTER", g_FREQUENCY_AFTER, s_FREQUENCY_AFTER);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY_AFTER_AT_A_TIME", g_FREQUENCY_AFTER_AT_A_TIME, s_FREQUENCY_AFTER_AT_A_TIME);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY_AFTER_TIMES", g_FREQUENCY_AFTER_TIMES, s_FREQUENCY_AFTER_TIMES);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY_BEFORE", g_FREQUENCY_BEFORE, s_FREQUENCY_BEFORE);
    pyrt::xrefs::reg_access("memory.narrow", "FREQUENCY_IGNORE", g_FREQUENCY_IGNORE, s_FREQUENCY_IGNORE);
    pyrt::xrefs::reg_access("memory.narrow", "FREQ_CLAUSE", g_FREQ_CLAUSE, s_FREQ_CLAUSE);
    pyrt::xrefs::reg_access("memory.narrow", "FREQ_LIMIT", g_FREQ_LIMIT, s_FREQ_LIMIT);
    pyrt::xrefs::reg_access("memory.narrow", "FREQ_PORTION", g_FREQ_PORTION, s_FREQ_PORTION);
    pyrt::xrefs::reg_access("memory.narrow", "FULL_LENGTH", g_FULL_LENGTH, s_FULL_LENGTH);
    pyrt::xrefs::reg_access("memory.narrow", "HELD", g_HELD, s_HELD);
    pyrt::xrefs::reg_access("memory.narrow", "IN_A_CONDITION", g_IN_A_CONDITION, s_IN_A_CONDITION);
    pyrt::xrefs::reg_access("memory.narrow", "IN_A_RELATIVE", g_IN_A_RELATIVE, s_IN_A_RELATIVE);
    pyrt::xrefs::reg_access("memory.narrow", "JOINS", g_JOINS, s_JOINS);
    pyrt::xrefs::reg_access("memory.narrow", "KEEPS", g_KEEPS, s_KEEPS);
    pyrt::xrefs::reg_access("memory.narrow", "LEANING", g_LEANING, s_LEANING);
    pyrt::xrefs::reg_access("memory.narrow", "LENGTH_FILLER", g_LENGTH_FILLER, s_LENGTH_FILLER);
    pyrt::xrefs::reg_access("memory.narrow", "LENGTH_NAMED_DAYS", g_LENGTH_NAMED_DAYS, s_LENGTH_NAMED_DAYS);
    pyrt::xrefs::reg_access("memory.narrow", "LENGTH_OF_THE_JOB", g_LENGTH_OF_THE_JOB, s_LENGTH_OF_THE_JOB);
    pyrt::xrefs::reg_access("memory.narrow", "MONTHS_LONG", g_MONTHS_LONG, s_MONTHS_LONG);
    pyrt::xrefs::reg_access("memory.narrow", "NAMED_BREAK", g_NAMED_BREAK, s_NAMED_BREAK);
    pyrt::xrefs::reg_access("memory.narrow", "NAMES_A_PART", g_NAMES_A_PART, s_NAMES_A_PART);
    pyrt::xrefs::reg_access("memory.narrow", "NAMING_IGNORE", g_NAMING_IGNORE, s_NAMING_IGNORE);
    pyrt::xrefs::reg_access("memory.narrow", "NOT_OPTION", g_NOT_OPTION, s_NOT_OPTION);
    pyrt::xrefs::reg_access("memory.narrow", "NOT_RELATIVE", g_NOT_RELATIVE, s_NOT_RELATIVE);
    pyrt::xrefs::reg_access("memory.narrow", "NO_INTERVAL_SAID", g_NO_INTERVAL_SAID, s_NO_INTERVAL_SAID);
    pyrt::xrefs::reg_access("memory.narrow", "NO_LENGTH", g_NO_LENGTH, s_NO_LENGTH);
    pyrt::xrefs::reg_access("memory.narrow", "NO_SET_TIME", g_NO_SET_TIME, s_NO_SET_TIME);
    pyrt::xrefs::reg_access("memory.narrow", "NUM_RANGE", g_NUM_RANGE, s_NUM_RANGE);
    pyrt::xrefs::reg_access("memory.narrow", "ONLY_THE_INTERVAL_ASK", g_ONLY_THE_INTERVAL_ASK, s_ONLY_THE_INTERVAL_ASK);
    pyrt::xrefs::reg_access("memory.narrow", "OPTION", g_OPTION, s_OPTION);
    pyrt::xrefs::reg_access("memory.narrow", "OUTCOME_ASKS", g_OUTCOME_ASKS, s_OUTCOME_ASKS);
    pyrt::xrefs::reg_access("memory.narrow", "OVEN", g_OVEN, s_OVEN);
    pyrt::xrefs::reg_access("memory.narrow", "OWN_SAVING", g_OWN_SAVING, s_OWN_SAVING);
    pyrt::xrefs::reg_access("memory.narrow", "PAPERWORK", g_PAPERWORK, s_PAPERWORK);
    pyrt::xrefs::reg_access("memory.narrow", "PLACE", g_PLACE, s_PLACE);
    pyrt::xrefs::reg_access("memory.narrow", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.narrow", "QUALIFIER", g_QUALIFIER, s_QUALIFIER);
    pyrt::xrefs::reg_access("memory.narrow", "QUALIFIER_FILLER", g_QUALIFIER_FILLER, s_QUALIFIER_FILLER);
    pyrt::xrefs::reg_access("memory.narrow", "QUALIFIER_WORDS", g_QUALIFIER_WORDS, s_QUALIFIER_WORDS);
    pyrt::xrefs::reg_access("memory.narrow", "QUOTED_WORDS", g_QUOTED_WORDS, s_QUOTED_WORDS);
    pyrt::xrefs::reg_access("memory.narrow", "QUOTED_WORDS_SINGLE", g_QUOTED_WORDS_SINGLE, s_QUOTED_WORDS_SINGLE);
    pyrt::xrefs::reg_access("memory.narrow", "RANGE_JOIN", g_RANGE_JOIN, s_RANGE_JOIN);
    pyrt::xrefs::reg_access("memory.narrow", "RATE", g_RATE, s_RATE);
    pyrt::xrefs::reg_access("memory.narrow", "RATE_FREQ", g_RATE_FREQ, s_RATE_FREQ);
    pyrt::xrefs::reg_access("memory.narrow", "RATE_INTERVAL", g_RATE_INTERVAL, s_RATE_INTERVAL);
    pyrt::xrefs::reg_access("memory.narrow", "RATE_LEAD", g_RATE_LEAD, s_RATE_LEAD);
    pyrt::xrefs::reg_access("memory.narrow", "REMEDY", g_REMEDY, s_REMEDY);
    pyrt::xrefs::reg_access("memory.narrow", "REMEDY_CUT", g_REMEDY_CUT, s_REMEDY_CUT);
    pyrt::xrefs::reg_access("memory.narrow", "REMEDY_JOINER", g_REMEDY_JOINER, s_REMEDY_JOINER);
    pyrt::xrefs::reg_access("memory.narrow", "RESULT", g_RESULT, s_RESULT);
    pyrt::xrefs::reg_access("memory.narrow", "SAME", g_SAME, s_SAME);
    pyrt::xrefs::reg_access("memory.narrow", "SAYS_SOMETHING", g_SAYS_SOMETHING, s_SAYS_SOMETHING);
    pyrt::xrefs::reg_access("memory.narrow", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.narrow", "SEQUENCE", g_SEQUENCE, s_SEQUENCE);
    pyrt::xrefs::reg_access("memory.narrow", "SIGNALS", g_SIGNALS, s_SIGNALS);
    pyrt::xrefs::reg_access("memory.narrow", "SPANS", g_SPANS, s_SPANS);
    pyrt::xrefs::reg_access("memory.narrow", "SPAN_FILLER", g_SPAN_FILLER, s_SPAN_FILLER);
    pyrt::xrefs::reg_access("memory.narrow", "STEP", g_STEP, s_STEP);
    pyrt::xrefs::reg_access("memory.narrow", "STEP_VERB", g_STEP_VERB, s_STEP_VERB);
    pyrt::xrefs::reg_access("memory.narrow", "SYMPTOM", g_SYMPTOM, s_SYMPTOM);
    pyrt::xrefs::reg_access("memory.narrow", "THOUSANDS", g_THOUSANDS, s_THOUSANDS);
    pyrt::xrefs::reg_access("memory.narrow", "THOUSANDS_JOIN", g_THOUSANDS_JOIN, s_THOUSANDS_JOIN);
    pyrt::xrefs::reg_access("memory.narrow", "TRIGGER", g_TRIGGER, s_TRIGGER);
    pyrt::xrefs::reg_access("memory.narrow", "TRIGGER_PATTERN", g_TRIGGER_PATTERN, s_TRIGGER_PATTERN);
    pyrt::xrefs::reg_access("memory.narrow", "UNIT", g_UNIT, s_UNIT);
    pyrt::xrefs::reg_access("memory.narrow", "URGENT", g_URGENT, s_URGENT);
    pyrt::xrefs::reg_access("memory.narrow", "WAIT", g_WAIT, s_WAIT);
    pyrt::xrefs::reg_access("memory.narrow", "WEEKDAY_OR_CLOCK", g_WEEKDAY_OR_CLOCK, s_WEEKDAY_OR_CLOCK);
    pyrt::xrefs::reg_access("memory.narrow", "WHERE", g_WHERE, s_WHERE);
    pyrt::xrefs::reg_access("memory.narrow", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.narrow", "YES_NO", g_YES_NO, s_YES_NO);
    pyrt::xrefs::reg_access("memory.narrow", "_A_BARE_FIGURE", g__A_BARE_FIGURE, s__A_BARE_FIGURE);
    pyrt::xrefs::reg_access("memory.narrow", "_A_CUT_OFF_AFTER", g__A_CUT_OFF_AFTER, s__A_CUT_OFF_AFTER);
    pyrt::xrefs::reg_access("memory.narrow", "_A_CUT_OFF_BEFORE", g__A_CUT_OFF_BEFORE, s__A_CUT_OFF_BEFORE);
    pyrt::xrefs::reg_access("memory.narrow", "_A_DOING", g__A_DOING, s__A_DOING);
    pyrt::xrefs::reg_access("memory.narrow", "_A_DURATION_VERB", g__A_DURATION_VERB, s__A_DURATION_VERB);
    pyrt::xrefs::reg_access("memory.narrow", "_A_PERIOD_OF_EXTENT", g__A_PERIOD_OF_EXTENT, s__A_PERIOD_OF_EXTENT);
    pyrt::xrefs::reg_access("memory.narrow", "_A_RANGE", g__A_RANGE, s__A_RANGE);
    pyrt::xrefs::reg_access("memory.narrow", "_COUNTED_UNIT", g__COUNTED_UNIT, s__COUNTED_UNIT);
    pyrt::xrefs::reg_access("memory.narrow", "_IN_MONTHS_OR_YEARS", g__IN_MONTHS_OR_YEARS, s__IN_MONTHS_OR_YEARS);
    pyrt::xrefs::reg_access("memory.narrow", "_STATE_LENGTH_ASKS", g__STATE_LENGTH_ASKS, s__STATE_LENGTH_ASKS);
    pyrt::xrefs::reg_access("memory.narrow", "_UNITS", g__UNITS, s__UNITS);
}

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


thread_local! { static G_ABOUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_FOR_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_FOR_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_RATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_FOR_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEADLINE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEBT_CLEARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HORIZON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEADLINE_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEADLINE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EACH_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HORIZON_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HORIZON_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MUCH_EACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_A_TOTAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_K_THOUSANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MATHS_DEFINITION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_GOAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_REPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIODS_IN_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIME_DEFINITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIME_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUNDING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUND_IS_NOT_THE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVING_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVING_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPENDING_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOWARDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_HALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANNUAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AREA_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AREA_TARGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AVERAGE_EXPLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AVERAGE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DISTANCE_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LENGTH_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PACE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_YEAR_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BANK_HOLIDAY_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BANK_HOLIDAY_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BANK_HOLIDAY_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BANK_HOLIDAY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BANK_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BILL_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIN_OPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOUGHT_BY_THE_BAG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_COUNTDOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_FIXED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_TODAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHRISTMAS_WEEKDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_GAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_SUBJECT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUP_IN_ML: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUP_IN_TBSP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DATE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEFINITION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISTANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EASTER_OFFSETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FACTOR_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTION_OF_A_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTION_OF_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTION_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FREE_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOURS_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOURS_WORKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_FAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IS_THAT_TOO_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__K_THOUSANDS_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LENGTH_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MATHS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEAN_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDIAN_EXPLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__METRES_SQUARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MODE_EXPLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_BESIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS_OR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_TITLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMED_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEAREST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_PAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_THOUSANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_STATISTICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERCENT_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERCENT_ASK_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERCENT_ASK_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERCENT_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERCENT_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERIMETER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_AT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POLITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUADRATIC_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REST_OF_A_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REST_OF_A_PERIOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROUND_IS_AROUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAYS_WEEKDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SEASONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIDE_BY_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SINCE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SINCE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SINGULAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SMALLER_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SMALL_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SQUARE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATISTIC_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SURFACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TENS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_TURN_OF_THE_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TODAY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TO_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNARY_OPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAGE_CONTEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAGE_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAGE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKDAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKDAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_THOUSANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORKING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORKING_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORKING_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOU_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOU_LEFT_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_rate_put_right: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_in_a_safety_thread: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_money_already_held: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_pot_said_alone: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.maths: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_ASK() -> Py { ensure_init(); G_ABOUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_ASK(v: Py) { ensure_init(); G_ABOUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_FOR_FIGURE() -> Py { ensure_init(); G_ASKED_FOR_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_FOR_FIGURE(v: Py) { ensure_init(); G_ASKED_FOR_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_FOR_RATE() -> Py { ensure_init(); G_ASKED_FOR_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_FOR_RATE(v: Py) { ensure_init(); G_ASKED_FOR_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_RATE_ASK() -> Py { ensure_init(); G_A_BARE_RATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_RATE_ASK(v: Py) { ensure_init(); G_A_BARE_RATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_FOR_THE_THING() -> Py { ensure_init(); G_A_DATE_FOR_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_FOR_THE_THING(v: Py) { ensure_init(); G_A_DATE_FOR_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEADLINE_DAY() -> Py { ensure_init(); G_A_DEADLINE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEADLINE_DAY(v: Py) { ensure_init(); G_A_DEADLINE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEBT_CLEARED() -> Py { ensure_init(); G_A_DEBT_CLEARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEBT_CLEARED(v: Py) { ensure_init(); G_A_DEBT_CLEARED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HORIZON() -> Py { ensure_init(); G_A_HORIZON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HORIZON(v: Py) { ensure_init(); G_A_HORIZON.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_SPLIT() -> Py { ensure_init(); G_CLAUSE_SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_SPLIT(v: Py) { ensure_init(); G_CLAUSE_SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK() -> Py { ensure_init(); G_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK(v: Py) { ensure_init(); G_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_DEADLINE_DAYS() -> Py { ensure_init(); G_DEADLINE_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEADLINE_DAYS(v: Py) { ensure_init(); G_DEADLINE_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEADLINE_NAMED() -> Py { ensure_init(); G_DEADLINE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEADLINE_NAMED(v: Py) { ensure_init(); G_DEADLINE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_EACH_PERIOD() -> Py { ensure_init(); G_EACH_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EACH_PERIOD(v: Py) { ensure_init(); G_EACH_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FACTS() -> Py { ensure_init(); G_FACTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACTS(v: Py) { ensure_init(); G_FACTS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOAL() -> Py { ensure_init(); G_GOAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOAL(v: Py) { ensure_init(); G_GOAL.with(|c| *c.borrow_mut() = v); }
pub fn g_HORIZON_COUNTS() -> Py { ensure_init(); G_HORIZON_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HORIZON_COUNTS(v: Py) { ensure_init(); G_HORIZON_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HORIZON_SAID() -> Py { ensure_init(); G_HORIZON_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HORIZON_SAID(v: Py) { ensure_init(); G_HORIZON_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_HEAVY() -> Py { ensure_init(); G_HOW_HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_HEAVY(v: Py) { ensure_init(); G_HOW_HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MUCH_EACH() -> Py { ensure_init(); G_HOW_MUCH_EACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MUCH_EACH(v: Py) { ensure_init(); G_HOW_MUCH_EACH.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_A_TOTAL() -> Py { ensure_init(); G_JUST_A_TOTAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_A_TOTAL(v: Py) { ensure_init(); G_JUST_A_TOTAL.with(|c| *c.borrow_mut() = v); }
pub fn g_K_THOUSANDS() -> Py { ensure_init(); G_K_THOUSANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_K_THOUSANDS(v: Py) { ensure_init(); G_K_THOUSANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MATHS_DEFINITION_ASK() -> Py { ensure_init(); G_MATHS_DEFINITION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MATHS_DEFINITION_ASK(v: Py) { ensure_init(); G_MATHS_DEFINITION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_GOAL() -> Py { ensure_init(); G_MONEY_GOAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_GOAL(v: Py) { ensure_init(); G_MONEY_GOAL.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_NUMBERS() -> Py { ensure_init(); G_MONTH_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_NUMBERS(v: Py) { ensure_init(); G_MONTH_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWED() -> Py { ensure_init(); G_OWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWED(v: Py) { ensure_init(); G_OWED.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_REPORT() -> Py { ensure_init(); G_PAST_REPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_REPORT(v: Py) { ensure_init(); G_PAST_REPORT.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIODS() -> Py { ensure_init(); G_PERIODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIODS(v: Py) { ensure_init(); G_PERIODS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIODS_IN_A_YEAR() -> Py { ensure_init(); G_PERIODS_IN_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIODS_IN_A_YEAR(v: Py) { ensure_init(); G_PERIODS_IN_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIME_DEFINITION() -> Py { ensure_init(); G_PRIME_DEFINITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIME_DEFINITION(v: Py) { ensure_init(); G_PRIME_DEFINITION.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIME_TEST() -> Py { ensure_init(); G_PRIME_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIME_TEST(v: Py) { ensure_init(); G_PRIME_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE() -> Py { ensure_init(); G_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE(v: Py) { ensure_init(); G_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUNDING_NAMED() -> Py { ensure_init(); G_ROUNDING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUNDING_NAMED(v: Py) { ensure_init(); G_ROUNDING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUND_IS_NOT_THE_VERB() -> Py { ensure_init(); G_ROUND_IS_NOT_THE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUND_IS_NOT_THE_VERB(v: Py) { ensure_init(); G_ROUND_IS_NOT_THE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_A_RATE() -> Py { ensure_init(); G_SAID_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_A_RATE(v: Py) { ensure_init(); G_SAID_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVE_VERB() -> Py { ensure_init(); G_SAVE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVE_VERB(v: Py) { ensure_init(); G_SAVE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVING_CLAUSE() -> Py { ensure_init(); G_SAVING_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVING_CLAUSE(v: Py) { ensure_init(); G_SAVING_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVING_CUE() -> Py { ensure_init(); G_SAVING_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVING_CUE(v: Py) { ensure_init(); G_SAVING_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_SPENDING_CLAUSE() -> Py { ensure_init(); G_SPENDING_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPENDING_CLAUSE(v: Py) { ensure_init(); G_SPENDING_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES() -> Py { ensure_init(); G_TAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES(v: Py) { ensure_init(); G_TAKES.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ASKED() -> Py { ensure_init(); G_TIME_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ASKED(v: Py) { ensure_init(); G_TIME_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_ELSEWHERE() -> Py { ensure_init(); G_TIME_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_ELSEWHERE(v: Py) { ensure_init(); G_TIME_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_TOWARDS() -> Py { ensure_init(); G_TOWARDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOWARDS(v: Py) { ensure_init(); G_TOWARDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IF() -> Py { ensure_init(); G_WHAT_IF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IF(v: Py) { ensure_init(); G_WHAT_IF.with(|c| *c.borrow_mut() = v); }
pub fn g_WORKED() -> Py { ensure_init(); G_WORKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORKED(v: Py) { ensure_init(); G_WORKED.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_HALF() -> Py { ensure_init(); G__AFTER_HALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_HALF(v: Py) { ensure_init(); G__AFTER_HALF.with(|c| *c.borrow_mut() = v); }
pub fn g__AMOUNT() -> Py { ensure_init(); G__AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMOUNT(v: Py) { ensure_init(); G__AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__ANNUAL() -> Py { ensure_init(); G__ANNUAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANNUAL(v: Py) { ensure_init(); G__ANNUAL.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_HOUR() -> Py { ensure_init(); G__AN_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_HOUR(v: Py) { ensure_init(); G__AN_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g__AREA_ASK() -> Py { ensure_init(); G__AREA_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AREA_ASK(v: Py) { ensure_init(); G__AREA_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__AREA_TARGET() -> Py { ensure_init(); G__AREA_TARGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AREA_TARGET(v: Py) { ensure_init(); G__AREA_TARGET.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_DATE() -> Py { ensure_init(); G__ASK_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_DATE(v: Py) { ensure_init(); G__ASK_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g__AVERAGE_EXPLAIN() -> Py { ensure_init(); G__AVERAGE_EXPLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AVERAGE_EXPLAIN(v: Py) { ensure_init(); G__AVERAGE_EXPLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__AVERAGE_FILLER() -> Py { ensure_init(); G__AVERAGE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AVERAGE_FILLER(v: Py) { ensure_init(); G__AVERAGE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNT() -> Py { ensure_init(); G__A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); G__A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DISTANCE_STATED() -> Py { ensure_init(); G__A_DISTANCE_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DISTANCE_STATED(v: Py) { ensure_init(); G__A_DISTANCE_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LENGTH_OF_TIME() -> Py { ensure_init(); G__A_LENGTH_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH_OF_TIME(v: Py) { ensure_init(); G__A_LENGTH_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NOTE() -> Py { ensure_init(); G__A_NOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NOTE(v: Py) { ensure_init(); G__A_NOTE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PACE_WORD() -> Py { ensure_init(); G__A_PACE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PACE_WORD(v: Py) { ensure_init(); G__A_PACE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PAY_WORD() -> Py { ensure_init(); G__A_PAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PAY_WORD(v: Py) { ensure_init(); G__A_PAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_YEAR_IN_THE_LINE() -> Py { ensure_init(); G__A_YEAR_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_YEAR_IN_THE_LINE(v: Py) { ensure_init(); G__A_YEAR_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__BANK_HOLIDAY_COUNT() -> Py { ensure_init(); G__BANK_HOLIDAY_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BANK_HOLIDAY_COUNT(v: Py) { ensure_init(); G__BANK_HOLIDAY_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__BANK_HOLIDAY_LIST() -> Py { ensure_init(); G__BANK_HOLIDAY_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BANK_HOLIDAY_LIST(v: Py) { ensure_init(); G__BANK_HOLIDAY_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__BANK_HOLIDAY_NEXT() -> Py { ensure_init(); G__BANK_HOLIDAY_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BANK_HOLIDAY_NEXT(v: Py) { ensure_init(); G__BANK_HOLIDAY_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__BANK_HOLIDAY_TAIL() -> Py { ensure_init(); G__BANK_HOLIDAY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BANK_HOLIDAY_TAIL(v: Py) { ensure_init(); G__BANK_HOLIDAY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__BANK_WHERE() -> Py { ensure_init(); G__BANK_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BANK_WHERE(v: Py) { ensure_init(); G__BANK_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__BILL_WORD() -> Py { ensure_init(); G__BILL_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BILL_WORD(v: Py) { ensure_init(); G__BILL_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__BIN_OPS() -> Py { ensure_init(); G__BIN_OPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIN_OPS(v: Py) { ensure_init(); G__BIN_OPS.with(|c| *c.borrow_mut() = v); }
pub fn g__BOUGHT_BY_THE_BAG() -> Py { ensure_init(); G__BOUGHT_BY_THE_BAG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOUGHT_BY_THE_BAG(v: Py) { ensure_init(); G__BOUGHT_BY_THE_BAG.with(|c| *c.borrow_mut() = v); }
pub fn g__BUYING() -> Py { ensure_init(); G__BUYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUYING(v: Py) { ensure_init(); G__BUYING.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_COUNTDOWN() -> Py { ensure_init(); G__CHRISTMAS_COUNTDOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_COUNTDOWN(v: Py) { ensure_init(); G__CHRISTMAS_COUNTDOWN.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_DATE() -> Py { ensure_init(); G__CHRISTMAS_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_DATE(v: Py) { ensure_init(); G__CHRISTMAS_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_DAY() -> Py { ensure_init(); G__CHRISTMAS_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_DAY(v: Py) { ensure_init(); G__CHRISTMAS_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_FIXED() -> Py { ensure_init(); G__CHRISTMAS_FIXED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_FIXED(v: Py) { ensure_init(); G__CHRISTMAS_FIXED.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_TAIL() -> Py { ensure_init(); G__CHRISTMAS_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_TAIL(v: Py) { ensure_init(); G__CHRISTMAS_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_TODAY() -> Py { ensure_init(); G__CHRISTMAS_TODAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_TODAY(v: Py) { ensure_init(); G__CHRISTMAS_TODAY.with(|c| *c.borrow_mut() = v); }
pub fn g__CHRISTMAS_WEEKDAY() -> Py { ensure_init(); G__CHRISTMAS_WEEKDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHRISTMAS_WEEKDAY(v: Py) { ensure_init(); G__CHRISTMAS_WEEKDAY.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT() -> Py { ensure_init(); G__COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT(v: Py) { ensure_init(); G__COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED() -> Py { ensure_init(); G__COUNTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED(v: Py) { ensure_init(); G__COUNTED.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_EVENT() -> Py { ensure_init(); G__COUNTED_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_EVENT(v: Py) { ensure_init(); G__COUNTED_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_GAP() -> Py { ensure_init(); G__COUNTED_GAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_GAP(v: Py) { ensure_init(); G__COUNTED_GAP.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_SUBJECT_TAIL() -> Py { ensure_init(); G__COUNTED_SUBJECT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_SUBJECT_TAIL(v: Py) { ensure_init(); G__COUNTED_SUBJECT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTS() -> Py { ensure_init(); G__COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTS(v: Py) { ensure_init(); G__COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_ASK() -> Py { ensure_init(); G__COUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_ASK(v: Py) { ensure_init(); G__COUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_VALUE() -> Py { ensure_init(); G__COUNT_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_VALUE(v: Py) { ensure_init(); G__COUNT_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__CUP_IN_ML() -> Py { ensure_init(); G__CUP_IN_ML.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUP_IN_ML(v: Py) { ensure_init(); G__CUP_IN_ML.with(|c| *c.borrow_mut() = v); }
pub fn g__CUP_IN_TBSP() -> Py { ensure_init(); G__CUP_IN_TBSP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUP_IN_TBSP(v: Py) { ensure_init(); G__CUP_IN_TBSP.with(|c| *c.borrow_mut() = v); }
pub fn g__DATE_TAIL() -> Py { ensure_init(); G__DATE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DATE_TAIL(v: Py) { ensure_init(); G__DATE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_MONTH() -> Py { ensure_init(); G__DAY_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_MONTH(v: Py) { ensure_init(); G__DAY_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_ONLY() -> Py { ensure_init(); G__DAY_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_ONLY(v: Py) { ensure_init(); G__DAY_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__DEFINITION_ASK() -> Py { ensure_init(); G__DEFINITION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_ASK(v: Py) { ensure_init(); G__DEFINITION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__DISTANCE() -> Py { ensure_init(); G__DISTANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISTANCE(v: Py) { ensure_init(); G__DISTANCE.with(|c| *c.borrow_mut() = v); }
pub fn g__EASTER_OFFSETS() -> Py { ensure_init(); G__EASTER_OFFSETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EASTER_OFFSETS(v: Py) { ensure_init(); G__EASTER_OFFSETS.with(|c| *c.borrow_mut() = v); }
pub fn g__FACTOR_ASK() -> Py { ensure_init(); G__FACTOR_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FACTOR_ASK(v: Py) { ensure_init(); G__FACTOR_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__FILLER() -> Py { ensure_init(); G__FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FILLER(v: Py) { ensure_init(); G__FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTION() -> Py { ensure_init(); G__FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTION(v: Py) { ensure_init(); G__FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTIONS() -> Py { ensure_init(); G__FRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTIONS(v: Py) { ensure_init(); G__FRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTION_OF_A_FRACTION() -> Py { ensure_init(); G__FRACTION_OF_A_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTION_OF_A_FRACTION(v: Py) { ensure_init(); G__FRACTION_OF_A_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTION_OF_FRACTION() -> Py { ensure_init(); G__FRACTION_OF_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTION_OF_FRACTION(v: Py) { ensure_init(); G__FRACTION_OF_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTION_UNITS() -> Py { ensure_init(); G__FRACTION_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTION_UNITS(v: Py) { ensure_init(); G__FRACTION_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g__FREE_TIME_ASK() -> Py { ensure_init(); G__FREE_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FREE_TIME_ASK(v: Py) { ensure_init(); G__FREE_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAVY() -> Py { ensure_init(); G__HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAVY(v: Py) { ensure_init(); G__HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g__HOURS_WEEK() -> Py { ensure_init(); G__HOURS_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOURS_WEEK(v: Py) { ensure_init(); G__HOURS_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g__HOURS_WORKED() -> Py { ensure_init(); G__HOURS_WORKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOURS_WORKED(v: Py) { ensure_init(); G__HOURS_WORKED.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_FAR() -> Py { ensure_init(); G__HOW_FAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_FAR(v: Py) { ensure_init(); G__HOW_FAR.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_MUCH() -> Py { ensure_init(); G__HOW_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_MUCH(v: Py) { ensure_init(); G__HOW_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g__IS_THAT_TOO_MUCH() -> Py { ensure_init(); G__IS_THAT_TOO_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IS_THAT_TOO_MUCH(v: Py) { ensure_init(); G__IS_THAT_TOO_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g__K_THOUSANDS_HERE() -> Py { ensure_init(); G__K_THOUSANDS_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__K_THOUSANDS_HERE(v: Py) { ensure_init(); G__K_THOUSANDS_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g__LENGTH() -> Py { ensure_init(); G__LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LENGTH(v: Py) { ensure_init(); G__LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__LENGTH_UNIT() -> Py { ensure_init(); G__LENGTH_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LENGTH_UNIT(v: Py) { ensure_init(); G__LENGTH_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__MATHS_WORDS() -> Py { ensure_init(); G__MATHS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MATHS_WORDS(v: Py) { ensure_init(); G__MATHS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__MEAN_VERB() -> Py { ensure_init(); G__MEAN_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEAN_VERB(v: Py) { ensure_init(); G__MEAN_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDIAN_EXPLAIN() -> Py { ensure_init(); G__MEDIAN_EXPLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDIAN_EXPLAIN(v: Py) { ensure_init(); G__MEDIAN_EXPLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__METRES_SQUARED() -> Py { ensure_init(); G__METRES_SQUARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__METRES_SQUARED(v: Py) { ensure_init(); G__METRES_SQUARED.with(|c| *c.borrow_mut() = v); }
pub fn g__MODE_EXPLAIN() -> Py { ensure_init(); G__MODE_EXPLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MODE_EXPLAIN(v: Py) { ensure_init(); G__MODE_EXPLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_BESIDE() -> Py { ensure_init(); G__MONEY_BESIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_BESIDE(v: Py) { ensure_init(); G__MONEY_BESIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_HERE() -> Py { ensure_init(); G__MONEY_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_HERE(v: Py) { ensure_init(); G__MONEY_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_WORD() -> Py { ensure_init(); G__MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_WORD(v: Py) { ensure_init(); G__MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS_OR() -> Py { ensure_init(); G__MONTHS_OR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS_OR(v: Py) { ensure_init(); G__MONTHS_OR.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS_SAID() -> Py { ensure_init(); G__MONTHS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS_SAID(v: Py) { ensure_init(); G__MONTHS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_NAMES() -> Py { ensure_init(); G__MONTH_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_NAMES(v: Py) { ensure_init(); G__MONTH_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_SHORT() -> Py { ensure_init(); G__MONTH_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_SHORT(v: Py) { ensure_init(); G__MONTH_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_TAIL() -> Py { ensure_init(); G__MONTH_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_TAIL(v: Py) { ensure_init(); G__MONTH_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_TITLE() -> Py { ensure_init(); G__MONTH_TITLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_TITLE(v: Py) { ensure_init(); G__MONTH_TITLE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_WORD() -> Py { ensure_init(); G__MONTH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_WORD(v: Py) { ensure_init(); G__MONTH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMED_DAYS() -> Py { ensure_init(); G__NAMED_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMED_DAYS(v: Py) { ensure_init(); G__NAMED_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g__NEAREST() -> Py { ensure_init(); G__NEAREST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEAREST(v: Py) { ensure_init(); G__NEAREST.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_PAY() -> Py { ensure_init(); G__NOT_PAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_PAY(v: Py) { ensure_init(); G__NOT_PAY.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER() -> Py { ensure_init(); G__NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER(v: Py) { ensure_init(); G__NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_THOUSANDS() -> Py { ensure_init(); G__NUMBER_THOUSANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_THOUSANDS(v: Py) { ensure_init(); G__NUMBER_THOUSANDS.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_STATISTICS() -> Py { ensure_init(); G__OTHER_STATISTICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_STATISTICS(v: Py) { ensure_init(); G__OTHER_STATISTICS.with(|c| *c.borrow_mut() = v); }
pub fn g__PACES() -> Py { ensure_init(); G__PACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PACES(v: Py) { ensure_init(); G__PACES.with(|c| *c.borrow_mut() = v); }
pub fn g__PART() -> Py { ensure_init(); G__PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PART(v: Py) { ensure_init(); G__PART.with(|c| *c.borrow_mut() = v); }
pub fn g__PAY_WORDS() -> Py { ensure_init(); G__PAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAY_WORDS(v: Py) { ensure_init(); G__PAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PERCENT_ADVICE() -> Py { ensure_init(); G__PERCENT_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERCENT_ADVICE(v: Py) { ensure_init(); G__PERCENT_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__PERCENT_ASK_TAIL() -> Py { ensure_init(); G__PERCENT_ASK_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERCENT_ASK_TAIL(v: Py) { ensure_init(); G__PERCENT_ASK_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__PERCENT_ASK_WORD() -> Py { ensure_init(); G__PERCENT_ASK_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERCENT_ASK_WORD(v: Py) { ensure_init(); G__PERCENT_ASK_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PERCENT_DOWN() -> Py { ensure_init(); G__PERCENT_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERCENT_DOWN(v: Py) { ensure_init(); G__PERCENT_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g__PERCENT_UP() -> Py { ensure_init(); G__PERCENT_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERCENT_UP(v: Py) { ensure_init(); G__PERCENT_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__PERIMETER_ASK() -> Py { ensure_init(); G__PERIMETER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERIMETER_ASK(v: Py) { ensure_init(); G__PERIMETER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACES() -> Py { ensure_init(); G__PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACES(v: Py) { ensure_init(); G__PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_AT_A_THING() -> Py { ensure_init(); G__POINTS_AT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_A_THING(v: Py) { ensure_init(); G__POINTS_AT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__POLITE() -> Py { ensure_init(); G__POLITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POLITE(v: Py) { ensure_init(); G__POLITE.with(|c| *c.borrow_mut() = v); }
pub fn g__QUADRATIC_ASK() -> Py { ensure_init(); G__QUADRATIC_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUADRATIC_ASK(v: Py) { ensure_init(); G__QUADRATIC_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__RATE() -> Py { ensure_init(); G__RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RATE(v: Py) { ensure_init(); G__RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__REST_OF_A_PERIOD() -> Py { ensure_init(); G__REST_OF_A_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REST_OF_A_PERIOD(v: Py) { ensure_init(); G__REST_OF_A_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g__REST_OF_A_PERIOD_WORDS() -> Py { ensure_init(); G__REST_OF_A_PERIOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REST_OF_A_PERIOD_WORDS(v: Py) { ensure_init(); G__REST_OF_A_PERIOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__ROUND_IS_AROUND() -> Py { ensure_init(); G__ROUND_IS_AROUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROUND_IS_AROUND(v: Py) { ensure_init(); G__ROUND_IS_AROUND.with(|c| *c.borrow_mut() = v); }
pub fn g__SAYS_WEEKDAY() -> Py { ensure_init(); G__SAYS_WEEKDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAYS_WEEKDAY(v: Py) { ensure_init(); G__SAYS_WEEKDAY.with(|c| *c.borrow_mut() = v); }
pub fn g__SEASONS() -> Py { ensure_init(); G__SEASONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SEASONS(v: Py) { ensure_init(); G__SEASONS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHAPE() -> Py { ensure_init(); G__SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHAPE(v: Py) { ensure_init(); G__SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__SIDE() -> Py { ensure_init(); G__SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIDE(v: Py) { ensure_init(); G__SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__SIDE_BY_SIDE() -> Py { ensure_init(); G__SIDE_BY_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIDE_BY_SIDE(v: Py) { ensure_init(); G__SIDE_BY_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__SINCE_ASK() -> Py { ensure_init(); G__SINCE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SINCE_ASK(v: Py) { ensure_init(); G__SINCE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SINCE_FILLER() -> Py { ensure_init(); G__SINCE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SINCE_FILLER(v: Py) { ensure_init(); G__SINCE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__SINGULAR() -> Py { ensure_init(); G__SINGULAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SINGULAR(v: Py) { ensure_init(); G__SINGULAR.with(|c| *c.borrow_mut() = v); }
pub fn g__SMALLER_PERIOD() -> Py { ensure_init(); G__SMALLER_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SMALLER_PERIOD(v: Py) { ensure_init(); G__SMALLER_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g__SMALL_NUMBER() -> Py { ensure_init(); G__SMALL_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SMALL_NUMBER(v: Py) { ensure_init(); G__SMALL_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__SQUARE_WORD() -> Py { ensure_init(); G__SQUARE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SQUARE_WORD(v: Py) { ensure_init(); G__SQUARE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__STATISTIC_WORDS() -> Py { ensure_init(); G__STATISTIC_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATISTIC_WORDS(v: Py) { ensure_init(); G__STATISTIC_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SURFACES() -> Py { ensure_init(); G__SURFACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SURFACES(v: Py) { ensure_init(); G__SURFACES.with(|c| *c.borrow_mut() = v); }
pub fn g__TAILS() -> Py { ensure_init(); G__TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAILS(v: Py) { ensure_init(); G__TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g__TENS_WORDS() -> Py { ensure_init(); G__TENS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TENS_WORDS(v: Py) { ensure_init(); G__TENS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_TURN_OF_THE_YEAR() -> Py { ensure_init(); G__THE_TURN_OF_THE_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_TURN_OF_THE_YEAR(v: Py) { ensure_init(); G__THE_TURN_OF_THE_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__TODAY_ASK() -> Py { ensure_init(); G__TODAY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TODAY_ASK(v: Py) { ensure_init(); G__TODAY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__TO_WORK() -> Py { ensure_init(); G__TO_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TO_WORK(v: Py) { ensure_init(); G__TO_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g__UNARY_OPS() -> Py { ensure_init(); G__UNARY_OPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNARY_OPS(v: Py) { ensure_init(); G__UNARY_OPS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT() -> Py { ensure_init(); G__UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT(v: Py) { ensure_init(); G__UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__UNITS() -> Py { ensure_init(); G__UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNITS(v: Py) { ensure_init(); G__UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g__WAGE() -> Py { ensure_init(); G__WAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAGE(v: Py) { ensure_init(); G__WAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__WAGE_CONTEXT() -> Py { ensure_init(); G__WAGE_CONTEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAGE_CONTEXT(v: Py) { ensure_init(); G__WAGE_CONTEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__WAGE_CUE() -> Py { ensure_init(); G__WAGE_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAGE_CUE(v: Py) { ensure_init(); G__WAGE_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g__WAGE_UNIT() -> Py { ensure_init(); G__WAGE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAGE_UNIT(v: Py) { ensure_init(); G__WAGE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKDAYS() -> Py { ensure_init(); G__WEEKDAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKDAYS(v: Py) { ensure_init(); G__WEEKDAYS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKDAY_WORDS() -> Py { ensure_init(); G__WEEKDAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKDAY_WORDS(v: Py) { ensure_init(); G__WEEKDAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_THOUSANDS() -> Py { ensure_init(); G__WORD_THOUSANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_THOUSANDS(v: Py) { ensure_init(); G__WORD_THOUSANDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WORKING_ASK() -> Py { ensure_init(); G__WORKING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORKING_ASK(v: Py) { ensure_init(); G__WORKING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WORKING_LEFT() -> Py { ensure_init(); G__WORKING_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORKING_LEFT(v: Py) { ensure_init(); G__WORKING_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g__WORKING_SHAPE() -> Py { ensure_init(); G__WORKING_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORKING_SHAPE(v: Py) { ensure_init(); G__WORKING_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__YOU_LEFT() -> Py { ensure_init(); G__YOU_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOU_LEFT(v: Py) { ensure_init(); G__YOU_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g__YOU_LEFT_ANYWHERE() -> Py { ensure_init(); G__YOU_LEFT_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOU_LEFT_ANYWHERE(v: Py) { ensure_init(); G__YOU_LEFT_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_a_rate_put_right() -> Py { ensure_init(); G_a_rate_put_right.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_rate_put_right(v: Py) { ensure_init(); G_a_rate_put_right.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_in_a_safety_thread() -> Py { ensure_init(); G_in_a_safety_thread.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_in_a_safety_thread(v: Py) { ensure_init(); G_in_a_safety_thread.with(|c| *c.borrow_mut() = v); }
pub fn g_the_money_already_held() -> Py { ensure_init(); G_the_money_already_held.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_money_already_held(v: Py) { ensure_init(); G_the_money_already_held.with(|c| *c.borrow_mut() = v); }
pub fn g_the_pot_said_alone() -> Py { ensure_init(); G_the_pot_said_alone.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_pot_said_alone(v: Py) { ensure_init(); G_the_pot_said_alone.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Maths and numbers: fixed facts as entries, and computed answers for real questions.\n\nMost maths questions carry their own numbers (\"what's 15% of 80\", \"120 miles at 60 miles an\nhour\", \"how many days in February\"), so a canned entry cannot answer them. This domain has\ntwo halves:\n\n- ENTRIES for facts that never change (weeks in a year, a prime number, pi).\n- solve(text, understood, memory) for questions that need arithmetic. It only answers when\n  the line clearly contains a calculation, and returns None otherwise, so ordinary lines with\n  a number in them are left to the other domains.\n\nbot/modules/answers.py calls solve() before looking for a static entry.\n");
    s_a_rate_put_right(py_func("a_rate_put_right", |args: &[Py]| pyrt::xrefs::xcall("memory.saving", "a_rate_put_right", args)));
    s_the_money_already_held(py_func("the_money_already_held", |args: &[Py]| pyrt::xrefs::xcall("memory.saving", "the_money_already_held", args)));
    s_the_pot_said_alone(py_func("the_pot_said_alone", |args: &[Py]| pyrt::xrefs::xcall("memory.saving", "the_pot_said_alone", args)));
    s_in_a_safety_thread(py_func("in_a_safety_thread", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "in_a_safety_thread", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("maths-and-numbers")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.MONTHS"); }
    s_MONTHS(py_dict(vec![(py_str("january"), Py::Int(31)), (py_str("february"), Py::Int(28)), (py_str("march"), Py::Int(31)), (py_str("april"), Py::Int(30)), (py_str("may"), Py::Int(31)), (py_str("june"), Py::Int(30)), (py_str("july"), Py::Int(31)), (py_str("august"), Py::Int(31)), (py_str("september"), Py::Int(30)), (py_str("october"), Py::Int(31)), (py_str("november"), Py::Int(30)), (py_str("december"), Py::Int(31))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._NUMBER"); }
    s__NUMBER(py_str("-?\\d+(?:[.,]\\d+)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BIN_OPS"); }
    s__BIN_OPS(py_dict(vec![(stdlib_attr("ast", "Add").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "add").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "Sub").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "sub").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "Mult").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "mul").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "Div").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "truediv").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "Pow").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "pow").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "Mod").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "mod").unwrap_or_else(|e| exc::raise(e)))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._UNARY_OPS"); }
    s__UNARY_OPS(py_dict(vec![(stdlib_attr("ast", "UAdd").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "pos").unwrap_or_else(|e| exc::raise(e))), (stdlib_attr("ast", "USub").unwrap_or_else(|e| exc::raise(e)), stdlib_attr("operator", "neg").unwrap_or_else(|e| exc::raise(e)))]));
    fn __data1() -> Py { py_dict(vec![(py_str("half"), py_tuple(vec![Py::Int(1), Py::Int(2)])), (py_str("halves"), py_tuple(vec![Py::Int(1), Py::Int(2)])), (py_str("third"), py_tuple(vec![Py::Int(1), Py::Int(3)])), (py_str("thirds"), py_tuple(vec![Py::Int(1), Py::Int(3)])), (py_str("quarter"), py_tuple(vec![Py::Int(1), Py::Int(4)])), (py_str("quarters"), py_tuple(vec![Py::Int(1), Py::Int(4)])), (py_str("fifth"), py_tuple(vec![Py::Int(1), Py::Int(5)])), (py_str("fifths"), py_tuple(vec![Py::Int(1), Py::Int(5)])), (py_str("sixth"), py_tuple(vec![Py::Int(1), Py::Int(6)])), (py_str("sixths"), py_tuple(vec![Py::Int(1), Py::Int(6)])), (py_str("eighth"), py_tuple(vec![Py::Int(1), Py::Int(8)])), (py_str("eighths"), py_tuple(vec![Py::Int(1), Py::Int(8)])), (py_str("tenth"), py_tuple(vec![Py::Int(1), Py::Int(10)])), (py_str("tenths"), py_tuple(vec![Py::Int(1), Py::Int(10)]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FRACTIONS"); }
    s__FRACTIONS(__data1());
    fn __data2() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNTS"); }
    s__COUNTS(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNT"); }
    s__COUNT(py_str("(a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PART"); }
    s__PART(py_str("(half|halves|third|thirds|quarter|quarters|fifth|fifths|sixth|sixths|eighth|eighths|tenth|tenths|\\d+\\s*/\\s*\\d+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AMOUNT"); }
    s__AMOUNT(py_str("(\\d+(?:\\s+\\d+)?\\s*/\\s*\\d+|\\d+(?:\\.\\d+)?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FRACTION_UNITS"); }
    s__FRACTION_UNITS(py_str("(cups?|tbsp|tablespoons?|tsp|teaspoons?|grams?|g|kg|ml|litres?|liters?|ounces?|oz|pounds?|pints?|sticks?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SINGULAR"); }
    s__SINGULAR(py_dict(vec![(py_str("cups"), py_str("cup")), (py_str("tablespoons"), py_str("tablespoon")), (py_str("teaspoons"), py_str("teaspoon")), (py_str("grams"), py_str("gram")), (py_str("litres"), py_str("litre")), (py_str("liters"), py_str("liter")), (py_str("ounces"), py_str("ounce")), (py_str("pounds"), py_str("pound")), (py_str("pints"), py_str("pint")), (py_str("sticks"), py_str("stick"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CUP_IN_TBSP"); }
    s__CUP_IN_TBSP(Py::Int(16));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CUP_IN_ML"); }
    s__CUP_IN_ML(Py::Int(240));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FRACTION_OF_FRACTION"); }
    s__FRACTION_OF_FRACTION(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:"), &g__COUNT()), &py_str(")\\s+)?")), &g__PART()), &py_str("\\s+of\\s+(?:a\\s+|an\\s+)?")), &g__AMOUNT()), &py_str("(?:\\s+of)?\\s+(?:a\\s+|an\\s+)?")), &g__FRACTION_UNITS()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FRACTION_OF_A_FRACTION"); }
    s__FRACTION_OF_A_FRACTION(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:"), &g__COUNT()), &py_str(")\\s+)?")), &g__PART()), &py_str("\\s+of\\s+\\d+\\s*/\\s*\\d+\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_COUNT"); }
    s__A_COUNT(py_str("(?:\\d+|two|three|four|five|six|seven|eight|nine|ten)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNT_VALUE"); }
    s__COUNT_VALUE(py_dict(vec![(py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FRACTION"); }
    s__FRACTION(py_str("(\\d+)\\s*(?:/|over|upon)\\s*(\\d+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PLACES"); }
    s__PLACES(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("1"), Py::Int(1)), (py_str("2"), Py::Int(2)), (py_str("3"), Py::Int(3)), (py_str("a"), Py::Int(1))]));
    fn __data3() -> Py { py_dict(vec![(py_str("whole"), Py::Int(0)), (py_str("unit"), Py::Int(0)), (py_str("units"), Py::Int(0)), (py_str("one"), Py::Int(0)), (py_str("ten"), op_neg(&Py::Int(1))), (py_str("tens"), op_neg(&Py::Int(1))), (py_str("10"), op_neg(&Py::Int(1))), (py_str("hundred"), op_neg(&Py::Int(2))), (py_str("hundreds"), op_neg(&Py::Int(2))), (py_str("100"), op_neg(&Py::Int(2))), (py_str("thousand"), op_neg(&Py::Int(3))), (py_str("thousands"), op_neg(&Py::Int(3))), (py_str("1000"), op_neg(&Py::Int(3)))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._NEAREST"); }
    s__NEAREST(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MATHS_WORDS"); }
    s__MATHS_WORDS(py_tuple(vec![py_tuple(vec![py_str("(?<![a-z])x\\s*squared\\b|\\bxsquared\\b"), py_str("x^2")]), py_tuple(vec![py_str("\\bplus\\b"), py_str("+")]), py_tuple(vec![py_str("\\bminus\\b"), py_str("-")]), py_tuple(vec![py_str("\\bequals\\b|\\bequal\\s+to\\b"), py_str("=")]), py_tuple(vec![py_str("\\bzero\\b"), py_str("0")]), py_tuple(vec![py_str("\\bnegative\\s+"), py_str("-")]), py_tuple(vec![py_str("\\b(?:find|work\\s+out|solve\\s+for)\\s+x\\b"), py_str("solve")])]));
    fn __data4() -> Py { py_tuple(vec![py_str("solve"), py_str("solves"), py_str("root"), py_str("roots"), py_str("zero"), py_str("zeros"), py_str("factorise"), py_str("factorize"), py_str("factor"), py_str("factors"), py_str("find"), py_str("work"), py_str("value")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._QUADRATIC_ASK"); }
    s__QUADRATIC_ASK(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FACTOR_ASK"); }
    s__FACTOR_ASK(py_tuple(vec![py_str("factorise"), py_str("factorize"), py_str("factor"), py_str("factors"), py_str("brackets")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERCENT_UP"); }
    s__PERCENT_UP(py_str("(?:going|goes|went|gone|is|are)\\s+up|up\\s+by|rise|rises|rising|rose|increase|increases|increased|more|higher|extra|hike|hikes|hiked|jump|jumps|jumped|on\\s+top|raise|raises|raised"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERCENT_DOWN"); }
    s__PERCENT_DOWN(py_str("(?:going|goes|went|gone|is|are)\\s+down|down\\s+by|drop|drops|dropped|fall|falls|fell|cut|cuts|slashed|reduce|reduced|less|cheaper|lower"));
    fn __data5() -> Py { py_tuple(vec![py_str("should i"), py_str("fair"), py_str("legal"), py_str("allowed"), py_str("normal"), py_str("refuse"), py_str("worth it"), py_str("can they"), py_str("can my landlord"), py_str("too much"), py_str("right"), py_str("negotiate"), py_str("accept")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERCENT_ADVICE"); }
    s__PERCENT_ADVICE(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONEY_WORD"); }
    s__MONEY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:rent|bill|price|salary|wage|wages|pay|cost|quote|fare|mortgage|charge|rate|total|fee|premium|budget|balance|deposit|pension|tax|income|earnings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERCENT_ASK_TAIL"); }
    s__PERCENT_ASK_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:of|off|on)\\s+(?:that|it|this|those|them|the\\s+(?:rent|bill|price|salary|total|figure|number|amount|money))\\b|\\b(?:be|come\\s+to|work\\s+out\\s+to)\\b[\\s?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERCENT_ASK_WORD"); }
    s__PERCENT_ASK_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|how\\s+much|how\\s+many|would|will|could|should|does|is|was)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._YOU_LEFT"); }
    s__YOU_LEFT(stdlib_call("re", "compile", &[py_str("(?:left|leave|leaving|gave|give|put down|tipped|tip)\\s+(?:£|\\$|€)?\\s*(?P<left>\\d[\\d,]*(?:\\.\\d+)?|a tenner|a fiver|a twenty)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s*(?:on|for|towards)\\s+(?:a|an|the|our|my)?\\s*(?:£|\\$|€)?\\s*(?P<bill>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s*(?P<what>bill|check|tab|meal|dinner|lunch|round)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._YOU_LEFT_ANYWHERE"); }
    s__YOU_LEFT_ANYWHERE(stdlib_call("re", "compile", &[py_str("(?:left|leave|leaving|gave|put down|tipped)\\s+(?:£|\\$|€)?\\s*(\\d[\\d,]*(?:\\.\\d+)?|a tenner|a fiver|a twenty)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s+(?:on|for|towards)\\s+(?:a|an|the|our|my)?\\s*(?:£|\\$|€)?\\s*(\\d[\\d,]*(?:\\.\\d+)?)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BILL_WORD"); }
    s__BILL_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:bill|tip|meal|dinner|lunch|check|tab|round|service)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_NOTE"); }
    s__A_NOTE(py_dict(vec![(py_str("a tenner"), Py::Int(10)), (py_str("a fiver"), Py::Int(5)), (py_str("a twenty"), Py::Int(20))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PAY_WORDS"); }
    s__PAY_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:earn|earns|earning|wage|wages|salary|salaried|paid|pay|income|come to|comes to|coming to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._NOT_PAY"); }
    s__NOT_PAY(stdlib_call("re", "compile", &[py_str("\\b(?:after tax|take home|take-home|national insurance|pension|student loan|miles? an hour|mph)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._RATE"); }
    s__RATE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)?\\s*(\\d+(?:\\.\\d+)?)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s*(?:an hour|a hour|per hour|hourly|/\\s*hours?|/\\s*hr|p/h)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._HOURS_WEEK"); }
    s__HOURS_WEEK(stdlib_call("re", "compile", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:hours?|hrs?)\\s*(?:a|per|each|/)\\s*(?:week|wk)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._ANNUAL"); }
    s__ANNUAL(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)?\\s*(\\d+(?:[.,]\\d+)?)\\s*(k)?\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s*(?:a|per|/)?\\s*(?:year|annum|yr)\\b(?!\\s+olds?\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONEY_BESIDE"); }
    s__MONEY_BESIDE(stdlib_call("re", "compile", &[py_str("£|\\$|€|\\b(?:pounds?|quid|euros?|dollars?|bucks?|salary|salaried|wage|wages|income|earn|earns|earning|paid|pay|annum|per annum|pa)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PACES"); }
    s__PACES(py_tuple(vec![py_tuple(vec![py_str("\\bwalk(?:s|ed|ing)?\\b|\\bon foot\\b|\\bby foot\\b|\\bpedestrian\\b"), Py::Float(3.0), py_str("at walking pace"), py_str("20 minutes a mile"), py_str("on foot"), py_str("Allow a bit more for hills and a stop."), py_str("A brisk one gets a little more, and hills and a bag take it back.")]), py_tuple(vec![py_str("\\bcycl(?:e|es|ed|ing)\\b|\\bbikes?\\b|\\bbicycles?\\b|\\bpushbike\\b|\\bby bike\\b"), Py::Float(12.0), py_str("on a bike"), py_str("five minutes a mile"), py_str("on a bike"), py_str("Allow a bit more for hills and a stop."), py_str("A good day with the wind behind you gets a little more.")]), py_tuple(vec![py_str("\\brun(?:s|ning)?\\b|\\bjog(?:s|ging)?\\b"), Py::Float(6.0), py_str("at running pace"), py_str("10 minutes a mile"), py_str("running"), py_str("Allow a bit more for hills and a stop."), py_str("A brisk one gets a little more, and hills take it back.")]), py_tuple(vec![py_str("\\bdriv(?:e|es|ing)\\b|\\bby car\\b|\\bin the car\\b|\\bcar journey\\b"), Py::Float(50.0), py_str("in the car"), py_str("a bit over a minute a mile"), py_str("in the car"), py_str("Motorway miles are quicker, A roads and a town slower, so treat it as the middle."), py_str("Motorway miles get you further, a town and its traffic get you less.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_PACE_WORD"); }
    s__A_PACE_WORD(stdlib_call("re", "compile", &[mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PACES()) { let __unpk1 = py_unpack(&__item.clone(), 2, Some(1)); let mut pattern: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); __out.push(pattern.clone()); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._DISTANCE"); }
    s__DISTANCE(stdlib_call("re", "compile", &[py_str("(?P<num>\\d+(?:\\.\\d+)?|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|half)\\s*(?P<unit>miles?|mile|km|kms|kilometres?|kilometers?|metres?|meters?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AFTER_HALF"); }
    s__AFTER_HALF(stdlib_call("re", "compile", &[py_str("\\bhalf (?:a|an|one)\\s*(?:mile|km|kilometre|kilometer)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._TO_WORK"); }
    s__TO_WORK(stdlib_call("re", "compile", &[py_str("\\b(?:to work|to the office|to my office|to the job|commute|commuting|into work|to the workplace)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_DISTANCE_STATED"); }
    s__A_DISTANCE_STATED(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|well|ok|okay|its|it'?s|it is|thats|that is|about|roughly|around|approximately|maybe|probably|just|only)\\s+){0,5}(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|half|\\d+(?:\\.\\d+)?)(?:\\s+and a half)?\\s*(?:miles?|km|kms|kilometres?|kilometers?)\\b[^.?!]{0,26}$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._IS_THAT_TOO_MUCH"); }
    s__IS_THAT_TOO_MUCH(stdlib_call("re", "compile", &[py_str("\\bis (?:that|this|it) (?:too far|too much|too long|a lot|alright|ok|okay|doable)\\b|\\b(?:is|would) that be (?:too far|too much|a lot)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.TIME_ASKED"); }
    s_TIME_ASKED(stdlib_call("re", "compile", &[py_str("\\bhow long\\b|\\bhow many (?:months|weeks|years|fortnights)\\b|\\bhow soon\\b|\\bwhen (?:will|would|should|do|can|am) (?:i|we)\\b|\\bwhen do i\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.WHAT_IF"); }
    s_WHAT_IF(stdlib_call("re", "compile", &[py_str("\\bwhat if (?:i|we)\\b|\\band if (?:i|we)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ABOUT_ASK"); }
    s_ABOUT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|how) about\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.CLOCK"); }
    s_CLOCK(stdlib_call("re", "compile", &[py_str("\\bo'?\\s?clock\\b|\\b\\d{1,2}\\s*(?:am|pm)\\b|\\b(?:morning|afternoon|evening|tonight|tuesday|wednesday|thursday|friday|saturday|sunday|monday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.TAKES"); }
    s_TAKES(stdlib_call("re", "compile", &[py_str("takes (?:about |roughly |around )?(?P<amount>[\\d,.]+) (?P<period>days?|weeks?|fortnights?|months?|years?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.A_HORIZON"); }
    s_A_HORIZON(stdlib_call("re", "compile", &[py_str("\\b(?:in|within|by|after)\\s+(?:\\d+|an?|[a-z-]+)\\s+(?:days?|weeks?|fortnights?|months?|years?)\\b|\\b\\d+\\s*(?:months?|years?|weeks?)\\s+(?:from now|time)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.TIME_ELSEWHERE"); }
    s_TIME_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:cook|roast|boil|bake|fry|grill|simmer|rest|leave|marinate|drive|walk|fly|cycle|ride|run|swim|wait|queue|charge|download|install)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.RATE"); }
    s_RATE(stdlib_call("re", "compile", &[py_str("(?P<amount>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?:pounds?|quid|euros?|dollars?|grand|k\\b)?\\s*(?:(?:by|aside|away|back|to (?:one|the) side)\\s+)?(?:a|an|per|each|every|/)\\s*(?P<period>day|week|fortnight|month|year)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.MONEY"); }
    s_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|spend|spends|spending|cost|costs|afford|budget|need|needs|want|wants|goal|target|total|price|deposit|pay|paying|put away|putting away|put by|putting by|earn|earning|owe|owing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.TOWARDS"); }
    s_TOWARDS(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|put away|putting away|put by|putting by|set aside|setting aside|towards|toward|pay off|paying off|paid off|pay towards|paying towards|pay down|paying down|pay back|paying back|afford|spare|extra|repay|repaying|return)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.CLAUSE_SPLIT"); }
    s_CLAUSE_SPLIT(stdlib_call("re", "compile", &[py_str("[,;.]|\\band\\b|\\bbut\\b|\\bthen\\b|\\bso\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.SAVING_CLAUSE"); }
    s_SAVING_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|savings|put|puts|putting|put away|putting away|put by|putting aside|set aside|setting aside|stash|stashing|squirrel|towards|toward|into|pay off|paying off|paid off|pay towards|paying towards|paid towards|pay down|paying down|pay back|paying back|paid back|overpay|overpaying|give|gives|giving|gave|send|sends|sending|transfer|transfers|transferred|repay|repays|repaying|return|returning)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.SPENDING_CLAUSE"); }
    s_SPENDING_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:spend|spends|spending|spent|cost|costs|costing|rent|bills?|charges?|wage|wages|salary|earn|earns|earning|lose|losing|lost|burn|burns|burning|waste|wasting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.OWED"); }
    s_OWED(stdlib_call("re", "compile", &[py_str("\\b(?:owe|owes|owed|owing|debt|debts|loan|loans|overdraft|mortgage|finance|financed|borrow|borrowed|borrowing|credit\\s*card|left\\s+on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.A_DEBT_CLEARED"); }
    s_A_DEBT_CLEARED(stdlib_call("re", "compile", &[py_str("\\b(?:pay(?:ing)?\\s+(?:it\\s+|them\\s+)?off|clear(?:ing)?|repay(?:ing)?|gone|off|rid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.PERIODS"); }
    s_PERIODS(py_dict(vec![(py_str("day"), op_truediv(&Py::Int(1), &Py::Float(30.44))), (py_str("week"), op_truediv(&Py::Int(1), &Py::Float(4.345))), (py_str("fortnight"), op_truediv(&Py::Int(1), &Py::Float(2.17))), (py_str("month"), Py::Int(1)), (py_str("year"), Py::Int(12))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.SAID_A_RATE"); }
    s_SAID_A_RATE(stdlib_call("re", "compile", &[py_str("\\b\\d[\\d,]*(?:\\.\\d+)?\\s*(?:k|grand|thousand)?\\s*(?:pounds?\\s+)?(?:a|per|each|every|/)\\s*(?:month|week|fortnight|year|day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.WORKED"); }
    s_WORKED(stdlib_call("re", "compile", &[py_str("£([\\d,]+)\\s+at\\s+£?[\\d,.]+\\s*(?:a|per|each)\\s+(?:month|week|fortnight|year)s?\\s+(?:is|takes|gives|comes to)\\s+(?:about\\s+|around\\s+|roughly\\s+)?\\d|\\bat\\s+£?[\\d,.]+\\s*(?:a|per|each)\\s+(?:month|week|fortnight|year)s?\\s*,\\s*(?:about\\s+)?£?([\\d,]+)\\s+takes\\s+(?:about\\s+|around\\s+|roughly\\s+)?\\d"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._TENS_WORDS"); }
    s__TENS_WORDS(py_dict(vec![(py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WORD_THOUSANDS"); }
    s__WORD_THOUSANDS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &mcall(&py_str("|"), "join", &[g__TENS_WORDS()])), &py_str(")\\s+(?:grand|thousand)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._NUMBER_THOUSANDS"); }
    s__NUMBER_THOUSANDS(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d+(?:\\.\\d+)?)\\s*(?:grand|thousand)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._K_THOUSANDS_HERE"); }
    s__K_THOUSANDS_HERE(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d+(?:\\.\\d+)?)\\s*k\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONEY_HERE"); }
    s__MONEY_HERE(stdlib_call("re", "compile", &[py_str("£|\\bpounds?\\b|\\bquid\\b|\\beuros?\\b|\\bdollars?\\b|\\bsav(?:e|ing|ings)\\b|\\bcost(?:s|ing)?\\b|\\bpay(?:ing|s|day)?\\b|\\bmoney\\b|\\bbudget\\b|\\bprice\\b|\\bafford\\b|\\bdeposit\\b|\\bspen[dt]\\b|\\bowe[sd]?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SMALLER_PERIOD"); }
    s__SMALLER_PERIOD(py_dict(vec![(py_str("year"), py_tuple(vec![py_str("month"), Py::Int(12)])), (py_str("month"), py_tuple(vec![py_str("week"), Py::Float(4.345)])), (py_str("fortnight"), py_tuple(vec![py_str("week"), Py::Int(2)])), (py_str("week"), py_tuple(vec![py_str("day"), Py::Int(7)])), (py_str("day"), py_tuple(vec![py_str("hour"), Py::Int(24)]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.MONEY_GOAL"); }
    s_MONEY_GOAL(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|put by|put aside|set aside|put away|deposit|afford|owe|owes|owed|debt|debts|repay|repaying|pay off|paying off|towards|toward)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ASKED_FOR_RATE"); }
    s_ASKED_FOR_RATE(py_str("how much could you set aside each month?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ASKED_FOR_FIGURE"); }
    s_ASKED_FOR_FIGURE(py_str("the figure you are saving towards"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.JUST_A_TOTAL"); }
    s_JUST_A_TOTAL(stdlib_call("re", "compile", &[py_str("^\\s*(?:no[, ]+|actually[, ]+|okay[, ]+|ok[, ]+|well[, ]+)?(?:say|lets say|let's say|make it|make that|call it|change it to|put it at)\\s+(?:i|we)?\\s*(?:need|needs|want|wants)?\\s*(?:about\\s+|around\\s+)?£?([\\d,.]+)\\s*(k|thousand|pounds?|quid)?\\s*[.!]*\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.HOW_MUCH_EACH"); }
    s_HOW_MUCH_EACH(stdlib_call("re", "compile", &[py_str("how much (?:should|do|would|will|can|need|have) (?:i|we)\\b[^?]*\\b(?:day|week|month)\\b|how much\\b[^?]*\\b(?:put away|put by|set aside|put aside|save each|save per)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.A_BARE_RATE_ASK"); }
    s_A_BARE_RATE_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|so|ok|okay|right|but|then|well)\\b[\\s,]*)*(?:how\\s+much|what)\\b(?:\\s+(?:would|will|do|should|could|does|did)\\b(?:[\\s,]+(?:that|it|i|we|you)\\b)?(?:[\\s,]+(?:need|have|be|take|come\\s+to)\\b)?)?\\s+(?:a|per|each|every|once\\s+a)\\s+(?P<period>day|week|fortnight|month)\\b(?:\\s+(?:would|will|could|should)\\s+(?:that|it|this)\\s+be\\b)?[\\s?.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.EACH_PERIOD"); }
    s_EACH_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:each|a|per|every|once a)\\s+(day|week|fortnight|month)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.PERIODS_IN_A_YEAR"); }
    s_PERIODS_IN_A_YEAR(py_dict(vec![(py_str("day"), Py::Int(365)), (py_str("week"), Py::Int(52)), (py_str("fortnight"), Py::Int(26)), (py_str("month"), Py::Int(12))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.DEADLINE_NAMED"); }
    s_DEADLINE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:by|before|until|till|in time for|ready for)\\s+(?:(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)(?:\\s+of)?\\s+)?(?:christmas|easter|summer|the summer|winter|the winter|next (?:week|month|year|summer|christmas|easter)|the end of (?:the |next |this )?(?:month|year|week|january|february|march|april|may|june|july|august|september|october|november|december)|january|february|march|april|may|june|july|august|september|october|november|december)\\b|\\bfor (?:christmas|the summer|summer|the holiday|our holiday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.GOAL"); }
    s_GOAL(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|deposit|need|needs|want|wants|afford|cost|costs|price|budget|goal|target|towards|toward|for a|for the|owe|owes|owing|owed|debt|debts|repay|repaying|pay off|paying off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.HORIZON_SAID"); }
    s_HORIZON_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:in|within|inside|over|after)\\s+(?P<count>\\d+|an?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|couple\\s+of|few)\\s*(?P<unit>days?|weeks?|fortnights?|months?|years?)\\b(?:\\s*(?:from now|time))?")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("couple of"), Py::Int(2)), (py_str("few"), Py::Int(3))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.HORIZON_COUNTS"); }
    s_HORIZON_COUNTS(__data6());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.PAST_REPORT"); }
    s_PAST_REPORT(stdlib_call("re", "compile", &[py_str("\\b(?:saved|spent|paid|lent|borrowed|bought|earned|received|got|took|have had|ago)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.DEADLINE_DAYS"); }
    s_DEADLINE_DAYS(py_dict(vec![(py_str("christmas"), py_tuple(vec![Py::Int(12), Py::Int(25)])), (py_str("the end of the year"), py_tuple(vec![Py::Int(12), Py::Int(31)])), (py_str("the end of the month"), Py::None), (py_str("the end of the week"), Py::None), (py_str("next year"), py_tuple(vec![Py::Int(1), Py::Int(1)])), (py_str("next month"), Py::None), (py_str("next week"), Py::None), (py_str("easter"), Py::None), (py_str("summer"), py_tuple(vec![Py::Int(7), Py::Int(1)])), (py_str("the summer"), py_tuple(vec![Py::Int(7), Py::Int(1)])), (py_str("winter"), py_tuple(vec![Py::Int(12), Py::Int(1)])), (py_str("the winter"), py_tuple(vec![Py::Int(12), Py::Int(1)]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.MONTH_NUMBERS"); }
    s_MONTH_NUMBERS(py_dict(vec![(py_str("january"), Py::Int(1)), (py_str("february"), Py::Int(2)), (py_str("march"), Py::Int(3)), (py_str("april"), Py::Int(4)), (py_str("may"), Py::Int(5)), (py_str("june"), Py::Int(6)), (py_str("july"), Py::Int(7)), (py_str("august"), Py::Int(8)), (py_str("september"), Py::Int(9)), (py_str("october"), Py::Int(10)), (py_str("november"), Py::Int(11)), (py_str("december"), Py::Int(12))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.K_THOUSANDS"); }
    s_K_THOUSANDS(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d+(?:\\.\\d+)?)\\s*k\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.A_DATE_FOR_THE_THING"); }
    s_A_DATE_FOR_THE_THING(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|ok|okay|right|well|then)\\b[\\s,]*)*(?:i|we)\\s+(?:want|need|would\\s+like|have\\s+to\\s+have|must\\s+have|'d\\s+like)\\s+(?:it|that|them|this)\\b[^0-9]*\\b(?:by|before|in\\s+time\\s+for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.SAVE_VERB"); }
    s_SAVE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|put by|put aside|set aside|put away|pay in|paying in)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.SAVING_CUE"); }
    s_SAVING_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|put by|put aside|set aside|put away|need|want|afford|goal|target|towards|toward|budget)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.A_DEADLINE_DAY"); }
    s_A_DEADLINE_DAY(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})(?:st|nd|rd|th)\\s+(?:of\\s+)?([a-z]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_dict(vec![(py_str("m"), py_str("m")), (py_str("metre"), py_str("m")), (py_str("metres"), py_str("m")), (py_str("meter"), py_str("m")), (py_str("meters"), py_str("m")), (py_str("cm"), py_str("cm")), (py_str("centimetre"), py_str("cm")), (py_str("centimetres"), py_str("cm")), (py_str("centimeter"), py_str("cm")), (py_str("centimeters"), py_str("cm")), (py_str("mm"), py_str("mm")), (py_str("millimetre"), py_str("mm")), (py_str("millimetres"), py_str("mm")), (py_str("millimeter"), py_str("mm")), (py_str("millimeters"), py_str("mm")), (py_str("ft"), py_str("ft")), (py_str("foot"), py_str("ft")), (py_str("feet"), py_str("ft")), (py_str("in"), py_str("in")), (py_str("inch"), py_str("in")), (py_str("inches"), py_str("in")), (py_str("yd"), py_str("yd")), (py_str("yard"), py_str("yd")), (py_str("yards"), py_str("yd")), (py_str("km"), py_str("km")), (py_str("kilometre"), py_str("km")), (py_str("kilometres"), py_str("km")), (py_str("kilometer"), py_str("km")), (py_str("kilometers"), py_str("km"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._LENGTH_UNIT"); }
    s__LENGTH_UNIT(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._LENGTH"); }
    s__LENGTH(py_str("(?:m|cm|mm|km|ft|feet|foot|in|inch|inches|yd|yard|yards|metres?|meters?|centimetres?|centimeters?|millimetres?|millimeters?|kilometres?|kilometers?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SIDE"); }
    s__SIDE({ let mut __s = String::new(); __s.push_str("(\\d+(?:\\.\\d+)?)\\s*("); __s.push_str(&fstr(&g__LENGTH(), -1, "")); __s.push_str(")?"); py_string(__s) });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SIDE_BY_SIDE"); }
    s__SIDE_BY_SIDE(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str(&fstr(&g__SIDE(), -1, "")); __s.push_str("\\s*(?:by|x|×|\\*)\\s*(\\d+(?:\\.\\d+)?)\\s*("); __s.push_str(&fstr(&g__LENGTH(), -1, "")); __s.push_str(")?(?:\\s*(?:by|x|×|\\*)\\s*(\\d+(?:\\.\\d+)?)\\s*("); __s.push_str(&fstr(&g__LENGTH(), -1, "")); __s.push_str(")?)?"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SQUARE_WORD"); }
    s__SQUARE_WORD(py_dict(vec![(py_str("m"), py_str("square metres")), (py_str("cm"), py_str("square centimetres")), (py_str("mm"), py_str("square millimetres")), (py_str("ft"), py_str("square feet")), (py_str("in"), py_str("square inches")), (py_str("yd"), py_str("square yards")), (py_str("km"), py_str("square kilometres"))]));
    fn __data8() -> Py { py_tuple(vec![py_str("area"), py_str("how big"), py_str("how much space"), py_str("square metres"), py_str("square meters"), py_str("square feet"), py_str("square foot"), py_str("m2"), py_str("sqm"), py_str("floor area"), py_str("carpet"), py_str("flooring"), py_str("laminate"), py_str("lino"), py_str("tiles"), py_str("tiling"), py_str("turf"), py_str("lawn"), py_str("patio"), py_str("decking"), py_str("shed base"), py_str("gravel"), py_str("bark"), py_str("chippings")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AREA_ASK"); }
    s__AREA_ASK(__data8());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._PERIMETER_ASK"); }
    s__PERIMETER_ASK(py_tuple(vec![py_str("perimeter"), py_str("skirting"), py_str("edging"), py_str("border"), py_str("how much fence"), py_str("fencing"), py_str("how much beading"), py_str("trim for the edge"), py_str("goes all the way round")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SHAPE"); }
    s__SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:rectangle|rectangular|oblong|triangle|triangular|circle|circular|square)\\b(?!\\s*(?:metres?|meters?|m|cm|mm|km|ft|feet|foot|inches?|yards?|yd)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_tuple(vec![py_tuple(vec![py_str("square feet"), py_str("ft")]), py_tuple(vec![py_str("square foot"), py_str("ft")]), py_tuple(vec![py_str("sq ft"), py_str("ft")]), py_tuple(vec![py_str("square metres"), py_str("m")]), py_tuple(vec![py_str("square meters"), py_str("m")]), py_tuple(vec![py_str("square metre"), py_str("m")]), py_tuple(vec![py_str("square meter"), py_str("m")]), py_tuple(vec![py_str("m2"), py_str("m")]), py_tuple(vec![py_str("sqm"), py_str("m")]), py_tuple(vec![py_str("square yards"), py_str("yd")]), py_tuple(vec![py_str("square inches"), py_str("in")]), py_tuple(vec![py_str("square centimetres"), py_str("cm")]), py_tuple(vec![py_str("square centimeters"), py_str("cm")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AREA_TARGET"); }
    s__AREA_TARGET(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._METRES_SQUARED"); }
    s__METRES_SQUARED(py_dict(vec![(py_str("m"), Py::Float(1.0)), (py_str("cm"), Py::Float(0.0001)), (py_str("mm"), Py::Float(1e-06)), (py_str("km"), Py::Float(1000000.0)), (py_str("ft"), Py::Float(0.09290304)), (py_str("in"), Py::Float(0.00064516)), (py_str("yd"), Py::Float(0.83612736))]));
    fn __data10() -> Py { py_tuple(vec![py_tuple(vec![py_str("lawn"), py_str("lawn")]), py_tuple(vec![py_str("garden"), py_str("garden")]), py_tuple(vec![py_str("patio"), py_str("patio")]), py_tuple(vec![py_str("decking"), py_str("deck")]), py_tuple(vec![py_str("driveway"), py_str("driveway")]), py_tuple(vec![py_str("wall"), py_str("wall")]), py_tuple(vec![py_str("ceiling"), py_str("ceiling")]), py_tuple(vec![py_str("roof"), py_str("roof")]), py_tuple(vec![py_str("floor"), py_str("floor")]), py_tuple(vec![py_str("room"), py_str("room")]), py_tuple(vec![py_str("table"), py_str("table")]), py_tuple(vec![py_str("field"), py_str("field")]), py_tuple(vec![py_str("greenhouse"), py_str("greenhouse")]), py_tuple(vec![py_str("shed"), py_str("shed")]), py_tuple(vec![py_str("pond"), py_str("pond")]), py_tuple(vec![py_str("border"), py_str("border")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SURFACES"); }
    s__SURFACES(__data10());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNTED_GAP"); }
    s__COUNTED_GAP(py_str("(?:(?!\\b(?:i|we|you|they|he|she|it|to|do|does|did|should|could|would|can|will|may|might|must|shall|is|are|was|were|be|am|have|has|had|not)\\b)\\w+\\s+){0,3}"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNTED"); }
    s__COUNTED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bhow many\\s+"), &g__COUNTED_GAP()), &py_str("(?:tiles?|rolls?|packs?|bags?|cans?|tins?|boards?|planks?|squares|lengths?|coats?|slabs?|bricks?|blocks?|panels?|posts?|sheets?)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data11() -> Py { py_tuple(vec![py_str("gravel"), py_str("shingle"), py_str("chippings"), py_str("bark"), py_str("mulch"), py_str("woodchip"), py_str("topsoil"), py_str("soil"), py_str("compost"), py_str("manure"), py_str("sand"), py_str("slabs"), py_str("paving"), py_str("turf"), py_str("underlay"), py_str("grout"), py_str("tile adhesive"), py_str("adhesive"), py_str("concrete"), py_str("postcrete"), py_str("post mix"), py_str("ballast"), py_str("cement"), py_str("mortar")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BOUGHT_BY_THE_BAG"); }
    s__BOUGHT_BY_THE_BAG(__data11());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BUYING"); }
    s__BUYING(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTH_TAIL"); }
    s__MONTH_TAIL(py_str("(?:\\s+(?:this year|next year|this time|again|now|already|exactly|in total|left|to go|please|though|then|anyway)|\\s+(?:19|20)\\d\\d|\\s*,\\s*[^.?!]{0,60}|\\s+(?:i|we)\\s+(?:always\\s+|never\\s+|keep\\s+)?(?:forget(?:ting)?|mix(?:ing)? (?:it|them) up|get\\s+(?:it|them)\\s+mixed up))*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_TAIL"); }
    s__CHRISTMAS_TAIL(py_str("(?:\\s+(?:this year|next year|this christmas|next christmas|now|again|yet|already|exactly|left|to go|from now|from today|at the moment|please|then))*(?:\\s+(?:in|as) (?P<unit2>days?|weeks?|sleeps?))?(?:\\s+(?P<year>\\d{4}))?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_DAY"); }
    s__CHRISTMAS_DAY(py_str("(?:christmas(?: day| eve)?|xmas(?: day| eve)?|crimbo)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_COUNTDOWN"); }
    s__CHRISTMAS_COUNTDOWN(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:(?:how many |how much longer |how long )?(?:shopping )?(?:more |still )?(?P<unit>days?|weeks?|months?|sleeps?) (?:is it |are there |are left |left |to go |still |have i got left |have i got )?(?:(?:are|is|there|left|to go|still|do we have|have we got|is it|was it|are they)\\s+){0,3}(?:until|'?till?|to|before) "), &g__CHRISTMAS_DAY()), &py_str("|(?:days|weeks) (?:until|'?till?|to|before) ")), &g__CHRISTMAS_DAY()), &py_str("|how long (?:is it |have i got |has it been )?(?:until|'?till?|to|before) ")), &g__CHRISTMAS_DAY()), &py_str("|how long (?:is it )?to go until ")), &g__CHRISTMAS_DAY()), &py_str("|countdown to ")), &g__CHRISTMAS_DAY()), &py_str("|(?:how many )?shopping days (?:are )?(?:left|to go)")), &py_str("|is it (?:almost|nearly|close to) ")), &g__CHRISTMAS_DAY()), &py_str("(?: yet)?|is it ")), &g__CHRISTMAS_DAY()), &py_str("(?: (?:today|yet|this year))?|(?:the )?")), &g__CHRISTMAS_DAY()), &py_str(" countdown)")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_DATE"); }
    s__CHRISTMAS_DATE(py_str("(?:whens|when(?:'s| is| will it be)|what date is|what day is|which day is|whats the date of)(?: christmas| xmas| christmas day|xmas day| christmas eve|xmas eve| christmas this year)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WEEKDAYS"); }
    s__WEEKDAYS(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_WEEKDAY"); }
    s__CHRISTMAS_WEEKDAY(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:what day of the week (?:is|does) |what weekday (?:is|does) |which day of the week (?:is|does) )?"), &g__CHRISTMAS_DAY()), &py_str("(?: fall| falls| land| lands)?(?: on)?(?: this year| this christmas)?|is ")), &g__CHRISTMAS_DAY()), &py_str("(?: this year| this christmas)? on a (?P<guess>")), &mcall(&py_str("|"), "join", &[g__WEEKDAYS()])), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_TODAY"); }
    s__CHRISTMAS_TODAY(py_str("(?:is it|is) (?:christmas|xmas)(?: day)?(?: today| yet| this year)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._CHRISTMAS_FIXED"); }
    s__CHRISTMAS_FIXED(py_str("(?:is|was|is it|was it) (?:christmas|xmas)(?: day)?(?: this year| next year| this christmas)?\\s+(?:(?:early|late)\\s+or\\s+(?:early|late)|early|late)(?: this year| next year| this christmas)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._EASTER_OFFSETS"); }
    s__EASTER_OFFSETS(py_tuple(vec![py_tuple(vec![py_str("easter(?:\\s+(?:sunday|day))?"), Py::Int(0), py_str("Easter Sunday")]), py_tuple(vec![py_str("good friday"), op_neg(&Py::Int(2)), py_str("Good Friday")]), py_tuple(vec![py_str("easter monday"), Py::Int(1), py_str("Easter Monday")]), py_tuple(vec![py_str("palm sunday"), op_neg(&Py::Int(7)), py_str("Palm Sunday")]), py_tuple(vec![py_str("maundy thursday"), op_neg(&Py::Int(3)), py_str("Maundy Thursday")]), py_tuple(vec![py_str("(?:ash wednesday|lent)"), op_neg(&Py::Int(46)), py_str("Ash Wednesday")]), py_tuple(vec![py_str("(?:shrove tuesday|pancake day)"), op_neg(&Py::Int(47)), py_str("Shrove Tuesday")]), py_tuple(vec![py_str("ascension day"), Py::Int(39), py_str("Ascension Day")]), py_tuple(vec![py_str("(?:whit sunday|pentecost)"), Py::Int(49), py_str("Pentecost")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTH_NAMES"); }
    s__MONTH_NAMES(py_tuple(vec![py_str("January"), py_str("February"), py_str("March"), py_str("April"), py_str("May"), py_str("June"), py_str("July"), py_str("August"), py_str("September"), py_str("October"), py_str("November"), py_str("December")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._ASK_DATE"); }
    s__ASK_DATE(py_str("(?:when(?:'s| is| will it be| does)|what (?:date|day|days) (?:is|are)(?: it)?|which (?:date|day) (?:is|are)|whats the date of)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._DATE_TAIL"); }
    s__DATE_TAIL(py_str("(?:(?:\\s+(?:in\\s+)?20\\d{2})|\\s+(?:this year|next year|this easter|next easter|now|again|yet|exactly|left|to go))*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNT_ASK"); }
    s__COUNT_ASK(py_str("(?:how many|how much longer|how long)?\\s*(?:more\\s+|still\\s+)?(?P<unit>days?|weeks?|months?|sleeps?|nights?)\\s+(?:is it\\s+|are there\\s+|are left\\s+|left\\s+|to go\\s+|have i got left\\s+)?(?:until|'?till?|to|before)\\s+(?P<rest>.+)|(?:how long(?:\\s+(?:is it|have i got))?|countdown)\\s+(?:to go\\s+)?(?:until|'?till?|to|before)\\s+(?P<rest2>.+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNTED_EVENT"); }
    s__COUNTED_EVENT(py_str("(?:(?:my|his|her|their|our|the)\\s+)?(?:little\\s+|big\\s+)?(?:birthday|holiday|holidays|wedding|anniversary|exam|exams|test|results day|due date|operation|op|flight|trip|cruise|driving test|interview|retirement|graduation|move|house move|scan|appointment|payday|pay day)(?:\\s+(?:on|in|to|for)\\s+(?P<when>.+))?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTHS_SAID"); }
    s__MONTHS_SAID(py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTH_SHORT"); }
    s__MONTH_SHORT(py_dict(vec![(py_str("jan"), Py::Int(1)), (py_str("feb"), Py::Int(2)), (py_str("mar"), Py::Int(3)), (py_str("apr"), Py::Int(4)), (py_str("jun"), Py::Int(6)), (py_str("jul"), Py::Int(7)), (py_str("aug"), Py::Int(8)), (py_str("sep"), Py::Int(9)), (py_str("sept"), Py::Int(9)), (py_str("oct"), Py::Int(10)), (py_str("nov"), Py::Int(11)), (py_str("dec"), Py::Int(12))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTH_WORD"); }
    s__MONTH_WORD(py_str("(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._DAY_MONTH"); }
    s__DAY_MONTH(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?P<day>\\d{1,2})(?:st|nd|rd|th)?\\s+(?:of\\s+)?(?P<month>"), &g__MONTH_WORD()), &py_str(")\\b|\\b(?P<month2>")), &g__MONTH_WORD()), &py_str(")\\s+(?P<day2>\\d{1,2})(?:st|nd|rd|th)?\\b")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._DAY_ONLY"); }
    s__DAY_ONLY(stdlib_call("re", "compile", &[py_str("(?:^|\\bthe\\s+|\\bof\\s+)(?P<day>\\d{1,2})(?:st|nd|rd|th)\\b(?:\\s+(?:now|please|thanks|again|yet))*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data12() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("couple of"), Py::Int(2)), (py_str("a couple of"), Py::Int(2)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SMALL_NUMBER"); }
    s__SMALL_NUMBER(__data12());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._HOW_FAR"); }
    s__HOW_FAR(stdlib_call("re", "compile", &[py_str("(?:in\\s+)?(?P<num>\\d{1,3}|a couple of|couple of|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?P<unit>days?|weeks?|months?|years?)(?:\\s*'?s?\\s*time|\\s+from\\s+(?:now|today|tomorrow))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._REST_OF_A_PERIOD_WORDS"); }
    s__REST_OF_A_PERIOD_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nhow many much long day days week weeks are is left to go remaining remain in of until till the\nthis end month year tax now already exactly please today january february march april may june july\naugust september october november december jan feb mar apr jun jul aug sep sept oct nov dec\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._REST_OF_A_PERIOD"); }
    s__REST_OF_A_PERIOD(stdlib_call("re", "compile", &[py_str("\\b(?:month|year|tax year|january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SINCE_ASK"); }
    s__SINCE_ASK(stdlib_call("re", "compile", &[py_str("(?:how many\\s+(?P<unit>days?|weeks?)\\s+(?:has it been\\s+|have passed\\s+)?(?:since|ago\\s+was)|how long\\s+(?:has it been\\s+)?(?:since|ago\\s+was))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SINCE_FILLER"); }
    s__SINCE_FILLER(stdlib_call("re", "compile", &[py_str("\\b(?:was|is|it|the|on|of|now|already|today|date|day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_YEAR_IN_THE_LINE"); }
    s__A_YEAR_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(1[89]\\d{2}|20\\d{2})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WORKING_ASK"); }
    s__WORKING_ASK(stdlib_call("re", "compile", &[py_str("(?P<count>how many\\s+(?:(?:working|work|business)\\s+days?|weekends?))\\s+(?:are there\\s+|are left\\s+|is it\\s+|do i have\\s+|left\\s+|to go\\s+)?(?:until|till|to|before)\\s+(?P<rest>.+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WORKING_LEFT"); }
    s__WORKING_LEFT(stdlib_call("re", "compile", &[py_str("(?P<count>how many\\s+(?:(?:working|work|business)\\s+days?|weekends?))\\s+(?:are\\s+|is\\s+|have i got\\s+)?(?:left|to go)\\s+(?:in\\s+|of\\s+)?(?:the\\s+|this\\s+)?(?P<period>year|month)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WORKING_SHAPE"); }
    s__WORKING_SHAPE(stdlib_call("re", "compile", &[py_str("(?:how many|what is|whats|what's|what are) (?:the )?(?P<which>working|work) (?P<unit>hours?|days?)(?: in| per| to| into| a| of)?(?: a| the| one)? (?P<week>week|day|month)(?: is (?:full|normal) time)?(?: of work)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._NAMED_DAYS"); }
    s__NAMED_DAYS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("valentines day"), py_str("valentine's day"), py_str("valentines")]), py_tuple(vec![py_str("fixed"), Py::Int(2), Py::Int(14)]), py_str("Valentine's Day"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("halloween"), py_str("hallowe'en")]), py_tuple(vec![py_str("fixed"), Py::Int(10), Py::Int(31)]), py_str("Halloween"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("bonfire night"), py_str("guy fawkes night"), py_str("guy fawkes")]), py_tuple(vec![py_str("fixed"), Py::Int(11), Py::Int(5)]), py_str("Bonfire Night"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("christmas eve"), py_str("xmas eve"), py_str("day before christmas")]), py_tuple(vec![py_str("fixed"), Py::Int(12), Py::Int(24)]), py_str("Christmas Eve"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("boxing day"), py_str("day after christmas")]), py_tuple(vec![py_str("fixed"), Py::Int(12), Py::Int(26)]), py_str("Boxing Day"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("new years eve"), py_str("new year's eve"), py_str("hogmanay"), py_str("day before new year"), py_str("day before new years day"), py_str("day before new year's day")]), py_tuple(vec![py_str("fixed"), Py::Int(12), Py::Int(31)]), py_str("New Year's Eve"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("new years day"), py_str("new year's day")]), py_tuple(vec![py_str("fixed"), Py::Int(1), Py::Int(1)]), py_str("New Year's Day"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("mothering sunday"), py_str("mothers day"), py_str("mother's day")]), py_tuple(vec![py_str("easter"), op_neg(&Py::Int(21))]), py_str("Mothering Sunday"), py_str(" That is the UK date; the American Mother's Day is a Sunday in May.")]), py_tuple(vec![py_tuple(vec![py_str("fathers day"), py_str("father's day")]), py_tuple(vec![py_str("nth"), Py::Int(6), Py::Int(6), Py::Int(3)]), py_str("Father's Day"), py_str("")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SEASONS"); }
    s__SEASONS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("spring")]), py_tuple(vec![Py::Int(3), Py::Int(1)]), Py::Int(5)]), py_tuple(vec![py_tuple(vec![py_str("summer")]), py_tuple(vec![Py::Int(6), Py::Int(1)]), Py::Int(8)]), py_tuple(vec![py_tuple(vec![py_str("autumn"), py_str("fall")]), py_tuple(vec![Py::Int(9), Py::Int(1)]), Py::Int(11)]), py_tuple(vec![py_tuple(vec![py_str("winter")]), py_tuple(vec![Py::Int(12), Py::Int(1)]), Py::Int(2)])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._COUNTED_SUBJECT_TAIL"); }
    s__COUNTED_SUBJECT_TAIL(py_str("(?:\\s+(?:now|again|yet|please|roughly|this year|next year|20\\d{2}))*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._THE_TURN_OF_THE_YEAR"); }
    s__THE_TURN_OF_THE_YEAR(py_tuple(vec![py_str("new year"), py_str("new years"), py_str("new year's"), py_str("the new year"), py_str("new year's day"), py_str("new years day"), py_str("new year's eve"), py_str("new years eve"), py_str("hogmanay")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BANK_HOLIDAY_TAIL"); }
    s__BANK_HOLIDAY_TAIL(py_str("(?:\\s+(?:in\\s+)?(?:england|wales|scotland|scottish|northern ireland|england and wales|the uk|the united kingdom|britain|edinburgh|glasgow|belfast|here|then|now|please|left|to go|coming up|off|in a row|weekend|in\\s+20\\d\\d|this year|next year))*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BANK_HOLIDAY_NEXT"); }
    s__BANK_HOLIDAY_NEXT(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("(?:when|what|whats|what's|which)(?:'s| is|s| are| will it be)?\\s+(?:the\\s+)?(?:next|coming|nearest|first)\\s+(?:bank|public)\\s+holiday(?:s)?(?:\\s+(?:after this one|after that|after|next))?"); __s.push_str(&fstr(&g__BANK_HOLIDAY_TAIL(), -1, "")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BANK_HOLIDAY_COUNT"); }
    s__BANK_HOLIDAY_COUNT(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("(?:how many|how long)\\s+(?:days?|weeks?|sleeps?)\\s+(?:is it |are there |left |to go )?(?:until|till|to|before)\\s+(?:the\\s+)?(?:next\\s+)?(?:bank|public)\\s+holiday"); __s.push_str(&fstr(&g__BANK_HOLIDAY_TAIL(), -1, "")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BANK_HOLIDAY_LIST"); }
    s__BANK_HOLIDAY_LIST(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("(?:when|what)(?:'s| are| is)?\\s+(?:the\\s+|all\\s+the\\s+)?(?:bank|public)\\s+holidays?(?:\\s+(?:this year|next year|in\\s+20\\d\\d|left|left this year|this coming year|which are left|are left|coming up))?"); __s.push_str(&fstr(&g__BANK_HOLIDAY_TAIL(), -1, "")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._SAYS_WEEKDAY"); }
    s__SAYS_WEEKDAY(stdlib_call("re", "compile", &[py_str("\\b(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|tue|weds|wed|thurs|thur|thu|fri|sat|sun)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTHS_OR"); }
    s__MONTHS_OR(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&call_builtin("list", &[g__MONTHS_SAID()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[g__MONTH_SHORT()]).unwrap_or_else(|e| exc::raise(e)))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._BANK_WHERE"); }
    s__BANK_WHERE(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bscotland\\b|\\bscottish\\b|\\bedinburgh\\b|\\bglasgow\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("scotland"), py_str("in Scotland")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bnorthern ireland\\b|\\bbelfast\\b|\\bderry\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("northern ireland"), py_str("in Northern Ireland")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WEEKDAYS"); }
    s__WEEKDAYS(py_tuple(vec![py_str("Monday"), py_str("Tuesday"), py_str("Wednesday"), py_str("Thursday"), py_str("Friday"), py_str("Saturday"), py_str("Sunday")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._POLITE"); }
    s__POLITE(py_str("(?:ok|okay|so|and|but|right|hi|hey|please|sorry|quick one|quick question|can you tell me|do you know|tell me|just wondering|i was wondering|wondering|out of interest)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FILLER"); }
    s__FILLER(py_str("(?:please|for me|thanks|thank you|exactly|roughly|about|approximately|again|then|altogether|in total|by the way|actually)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WEEKDAY_WORDS"); }
    s__WEEKDAY_WORDS(py_tuple(vec![py_tuple(vec![py_str("monday|mon"), Py::Int(0), py_str("Monday")]), py_tuple(vec![py_str("tuesday|tues|tue"), Py::Int(1), py_str("Tuesday")]), py_tuple(vec![py_str("wednesday|weds|wed"), Py::Int(2), py_str("Wednesday")]), py_tuple(vec![py_str("thursday|thurs|thur|thu"), Py::Int(3), py_str("Thursday")]), py_tuple(vec![py_str("friday|fri"), Py::Int(4), py_str("Friday")]), py_tuple(vec![py_str("saturday|sat"), Py::Int(5), py_str("Saturday")]), py_tuple(vec![py_str("sunday|sun"), Py::Int(6), py_str("Sunday")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MONTH_TITLE"); }
    s__MONTH_TITLE(py_tuple(vec![py_str("January"), py_str("February"), py_str("March"), py_str("April"), py_str("May"), py_str("June"), py_str("July"), py_str("August"), py_str("September"), py_str("October"), py_str("November"), py_str("December")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._TODAY_ASK"); }
    s__TODAY_ASK(py_str("(?:what|whats|what's|which)\\s+(?:is\\s+)?(?:the\\s+)?(?:day|date)(?:\\s+of\\s+the\\s+week)?(?:\\s+(?:is\\s+)?(?:it|today|now))?|what\\s+day\\s+(?:is\\s+it|are\\s+we)(?:\\s+on)?(?:\\s+today)?|(?:what|whats|what's)?\\s*(?:is\\s+)?today[’']?s\\s+(?:date|day)|(?:what|whats|what's)\\s+is\\s+today\\b(?:\\s+(?:the\\s+)?(?:date|day))?|the\\s+(?:date|day)(?:\\s+today)?|any\\s+idea\\s+what\\s+day\\s+it\\s+is"));
    fn __data13() -> Py { py_dict(vec![(py_str("second"), py_tuple(vec![py_str("second"), py_str("seconds"), py_str("sec"), py_str("secs")])), (py_str("minute"), py_tuple(vec![py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins")])), (py_str("hour"), py_tuple(vec![py_str("hour"), py_str("hours"), py_str("hr"), py_str("hrs")])), (py_str("day"), py_tuple(vec![py_str("day"), py_str("days")])), (py_str("week"), py_tuple(vec![py_str("week"), py_str("weeks")])), (py_str("month"), py_tuple(vec![py_str("month"), py_str("months")])), (py_str("year"), py_tuple(vec![py_str("year"), py_str("years")])), (py_str("ounce"), py_tuple(vec![py_str("ounce"), py_str("ounces"), py_str("oz")])), (py_str("pound"), py_tuple(vec![py_str("pound"), py_str("pounds"), py_str("lb"), py_str("lbs")])), (py_str("stone"), py_tuple(vec![py_str("stone"), py_str("stones")])), (py_str("gram"), py_tuple(vec![py_str("gram"), py_str("grams"), py_str("g"), py_str("gr")])), (py_str("kilogram"), py_tuple(vec![py_str("kilogram"), py_str("kilograms"), py_str("kilo"), py_str("kilos"), py_str("kg")])), (py_str("centimetre"), py_tuple(vec![py_str("centimetre"), py_str("centimetres"), py_str("centimeter"), py_str("centimeters"), py_str("cm")])), (py_str("inch"), py_tuple(vec![py_str("inch"), py_str("inches")])), (py_str("metre"), py_tuple(vec![py_str("metre"), py_str("metres"), py_str("meter"), py_str("meters")])), (py_str("foot"), py_tuple(vec![py_str("foot"), py_str("feet"), py_str("ft")])), (py_str("mile"), py_tuple(vec![py_str("mile"), py_str("miles")])), (py_str("pint"), py_tuple(vec![py_str("pint"), py_str("pints"), py_str("pt")])), (py_str("millilitre"), py_tuple(vec![py_str("millilitre"), py_str("millilitres"), py_str("milliliter"), py_str("milliliters"), py_str("ml")])), (py_str("litre"), py_tuple(vec![py_str("litre"), py_str("litres"), py_str("liter"), py_str("liters"), py_str("l")])), (py_str("cup"), py_tuple(vec![py_str("cup"), py_str("cups")])), (py_str("tablespoon"), py_tuple(vec![py_str("tablespoon"), py_str("tablespoons"), py_str("tbsp")])), (py_str("teaspoon"), py_tuple(vec![py_str("teaspoon"), py_str("teaspoons"), py_str("tsp")]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._UNITS"); }
    s__UNITS(__data13());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._UNIT"); }
    s__UNIT({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__UNITS(), "items", &[])) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk1[0].clone(); let mut aliases: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&aliases.clone()) { let mut alias: Py = __item.clone(); __out.push((alias.clone(), name.clone())); } } py_dict(__out) })() });
    fn __data14() -> Py { py_dict(vec![(py_tuple(vec![py_str("second"), py_str("minute")]), py_str("There are 60 seconds in a minute.")), (py_tuple(vec![py_str("minute"), py_str("hour")]), py_str("There are 60 minutes in an hour.")), (py_tuple(vec![py_str("hour"), py_str("day")]), py_str("There are 24 hours in a day.")), (py_tuple(vec![py_str("day"), py_str("week")]), py_str("There are 7 days in a week, so a fortnight is 14.")), (py_tuple(vec![py_str("hour"), py_str("week")]), py_str("There are 168 hours in a week: 7 days × 24 hours.")), (py_tuple(vec![py_str("minute"), py_str("day")]), py_str("There are 1,440 minutes in a day: 24 hours × 60 minutes.")), (py_tuple(vec![py_str("second"), py_str("day")]), py_str("There are 86,400 seconds in a day: 24 × 60 × 60.")), (py_tuple(vec![py_str("second"), py_str("year")]), py_str("There are 31,536,000 seconds in an ordinary year (365 × 24 × 60 × 60), and 31,622,400 in a leap year.")), (py_tuple(vec![py_str("minute"), py_str("year")]), py_str("There are 525,600 minutes in an ordinary year, and 527,040 in a leap year.")), (py_tuple(vec![py_str("hour"), py_str("year")]), py_str("There are 8,760 hours in an ordinary year, and 8,784 in a leap year.")), (py_tuple(vec![py_str("week"), py_str("year")]), py_str("There are 52 weeks and 1 day in an ordinary year (52 weeks and 2 days in a leap year), or about 52.14 weeks if you divide 365 by 7.")), (py_tuple(vec![py_str("day"), py_str("year")]), py_str("An ordinary year has 365 days; a leap year has 366, with 29 February. Leap years come every four years, except most century years.")), (py_tuple(vec![py_str("month"), py_str("year")]), py_str("There are 12 months in a year, from January to December.")), (py_tuple(vec![py_str("ounce"), py_str("pound")]), py_str("There are 16 ounces in a pound, and 14 pounds in a stone.")), (py_tuple(vec![py_str("pound"), py_str("stone")]), py_str("There are 14 pounds in a stone.")), (py_tuple(vec![py_str("gram"), py_str("kilogram")]), py_str("There are 1,000 grams in a kilogram, and 1,000 kilograms in a tonne.")), (py_tuple(vec![py_str("gram"), py_str("ounce")]), py_str("There are about 28.35 grams in an ounce, so 100g is about 3.5oz.")), (py_tuple(vec![py_str("pound"), py_str("kilogram")]), py_str("There are about 2.2 pounds in a kilogram, so 70kg is about 154lb and 1lb is about 0.45kg.")), (py_tuple(vec![py_str("centimetre"), py_str("metre")]), py_str("There are 100 centimetres in a metre, and 1,000 metres in a kilometre.")), (py_tuple(vec![py_str("centimetre"), py_str("inch")]), py_str("There are 2.54 centimetres in an inch, so 30cm is just under 12 inches.")), (py_tuple(vec![py_str("inch"), py_str("foot")]), py_str("There are 12 inches in a foot, and 3 feet in a yard.")), (py_tuple(vec![py_str("foot"), py_str("mile")]), py_str("There are 5,280 feet in a mile, which is 1,760 yards.")), (py_tuple(vec![py_str("millilitre"), py_str("litre")]), py_str("There are 1,000 millilitres in a litre.")), (py_tuple(vec![py_str("millilitre"), py_str("cup")]), py_str("A US cup is about 240ml and a UK cup is 250ml, so half a cup is around 120ml.")), (py_tuple(vec![py_str("gram"), py_str("cup")]), py_str("It depends what you're measuring: a cup is about 120g of flour, 200g of sugar, 225g of butter or 185g of rice.")), (py_tuple(vec![py_str("millilitre"), py_str("tablespoon")]), py_str("A tablespoon is 15ml and a teaspoon is 5ml, so three teaspoons make a tablespoon.")), (py_tuple(vec![py_str("millilitre"), py_str("pint")]), py_str("A UK pint is 568ml and a US pint is 473ml, so a UK pint is just over half a litre.")), (py_tuple(vec![py_str("pint"), py_str("litre")]), py_str("There are about 1.76 UK pints in a litre, or 2.1 US pints.")), (py_tuple(vec![py_str("teaspoon"), py_str("tablespoon")]), py_str("Three teaspoons make a tablespoon, and a tablespoon is 15ml against a teaspoon's 5ml."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.FACTS"); }
    s_FACTS(__data14());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AMOUNT"); }
    s__AMOUNT(py_str("(?:how many|number of)\\s+([a-z']+)\\s+(.*)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._TAILS"); }
    s__TAILS(py_tuple(vec![py_str("(?:are there |is there |are |is )?(?:in|to|per) (?:a |an |one |1 )?([a-z']+)(?: of .*)?"), py_str("(?:does|do) (?:a |an |one )?([a-z']+)(?: have| contain| make up)?"), py_str("(?:is|are) (?:a |an |one |1 )?([a-z']+)(?: of .*)?"), py_str("([a-z']+) (?:has|have)(?: how many)?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._HOW_MUCH"); }
    s__HOW_MUCH(py_str("how much (?:is |are )?(?:a |an |one |1 )?([a-z']+)(?: of [a-z]+)? (?:in|to) (?:a |an |one |1 )?([a-z']+)(?: of .*)?"));
    fn __data15() -> Py { py_dict(vec![(py_str("stone"), py_str("A stone is 14 pounds, which is about 6.35kg: 10 stone is 63.5kg and 12 stone is about 76kg.")), (py_str("stones"), py_str("A stone is 14 pounds, which is about 6.35kg: 10 stone is 63.5kg and 12 stone is about 76kg.")), (py_str("pound"), py_str("A pound is 16 ounces, about 454g, and 14 pounds make a stone.")), (py_str("pounds"), py_str("A pound is 16 ounces, about 454g, and 14 pounds make a stone.")), (py_str("lb"), py_str("A pound is 16 ounces, about 454g, and 14 pounds make a stone.")), (py_str("lbs"), py_str("A pound is 16 ounces, about 454g, and 14 pounds make a stone.")), (py_str("ounce"), py_str("An ounce is about 28.35g, and there are 16 ounces in a pound.")), (py_str("ounces"), py_str("An ounce is about 28.35g, and there are 16 ounces in a pound.")), (py_str("oz"), py_str("An ounce is about 28.35g, and there are 16 ounces in a pound.")), (py_str("kilo"), py_str("A kilo is 1,000 grams, which is about 2.2 pounds: 70kg is about 11 stone.")), (py_str("kilos"), py_str("A kilo is 1,000 grams, which is about 2.2 pounds: 70kg is about 11 stone.")), (py_str("kilogram"), py_str("A kilo is 1,000 grams, which is about 2.2 pounds: 70kg is about 11 stone.")), (py_str("kilograms"), py_str("A kilo is 1,000 grams, which is about 2.2 pounds: 70kg is about 11 stone.")), (py_str("kg"), py_str("A kilo is 1,000 grams, which is about 2.2 pounds: 70kg is about 11 stone.")), (py_str("gram"), py_str("A gram is a thousandth of a kilo. A paperclip or a sachet of sugar is about a gram.")), (py_str("grams"), py_str("A gram is a thousandth of a kilo. A paperclip or a sachet of sugar is about a gram.")), (py_str("g"), py_str("A gram is a thousandth of a kilo. A paperclip or a sachet of sugar is about a gram.")), (py_str("tonne"), py_str("A tonne is 1,000kg, which is about 2,205 pounds.")), (py_str("tonnes"), py_str("A tonne is 1,000kg, which is about 2,205 pounds.")), (py_str("ton"), py_str("A ton is 1,016kg in the UK and 907kg in the US; a tonne, spelled with the e, is 1,000kg.")), (py_str("tons"), py_str("A ton is 1,016kg in the UK and 907kg in the US; a tonne, spelled with the e, is 1,000kg."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.HOW_HEAVY"); }
    s_HOW_HEAVY(__data15());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._HEAVY"); }
    s__HEAVY(stdlib_call("re", "compile", &[py_str("(?:how (?:heavy|much) (?:is|does|do|are)|what (?:does|is)|how much does)\\s+(?:a |an |one |1 )?([a-z']+?)\\s*(?:weigh(?:s)?|weigh in at|in weight)?\\s*(?:in kilos|in kg|in pounds|in pounds and ounces|in lbs)?")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WAGE"); }
    s__WAGE(py_str("£?\\s*(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WAGE_UNIT"); }
    s__WAGE_UNIT(stdlib_call("re", "compile", &[py_str("\\s*(?:a|per|each|every|/)\\s*(?:month|week|fortnight|year|day|hour)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AN_HOUR"); }
    s__AN_HOUR(py_tuple(vec![py_str("an hour"), py_str("per hour"), py_str("/hour"), py_str("/hr"), py_str("/h "), py_str("a hour"), py_str("hourly rate of")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._HOURS_WORKED"); }
    s__HOURS_WORKED(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2}(?:\\.\\d+)?)\\s*(?:-|to)?\\s*(?:\\d{1,2}(?:\\.\\d+)?\\s*)?(?:hours?|hrs?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WAGE_CUE"); }
    s__WAGE_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:hourly|hour rate|rate an hour|per hour|an hour)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._WAGE_CONTEXT"); }
    s__WAGE_CONTEXT(stdlib_call("re", "compile", &[py_str("\\b(?:salary|wage|wages|pay|paid|earn|earns|earning|get|gets|worth|job|offer|contract|work|year|annual|month)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data16() -> Py { py_set(vec![py_str("what"), py_str("whats"), py_str("is"), py_str("are"), py_str("an"), py_str("a"), py_str("the"), py_str("how"), py_str("do"), py_str("does"), py_str("i"), py_str("you"), py_str("we"), py_str("work"), py_str("worked"), py_str("out"), py_str("working"), py_str("calculate"), py_str("find"), py_str("workout"), py_str("mean"), py_str("means"), py_str("average"), py_str("averages"), py_str("median"), py_str("mode"), py_str("difference"), py_str("between"), py_str("and"), py_str("define"), py_str("explain"), py_str("statistics"), py_str("vs"), py_str("in"), py_str("maths"), py_str("math"), py_str("to"), py_str("of"), py_str("that"), py_str("this"), py_str("for"), py_str("get"), py_str("tell"), py_str("me"), py_str("know"), py_str("it"), py_str("called"), py_str("please"), py_str("explanation"), py_str("definition"), py_str("set"), py_str("numbers"), py_str("number"), py_str("data"), py_str("list"), py_str("values"), py_str("value"), py_str("group"), py_str("series"), py_str("sequence"), py_str("sample"), py_str("figures"), py_str("bits")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AVERAGE_FILLER"); }
    s__AVERAGE_FILLER(__data16());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._AVERAGE_EXPLAIN"); }
    s__AVERAGE_EXPLAIN(py_str("The mean, which is what most people mean by an average, is the total shared out evenly: 4, 6 and 11 add to 21, and 21 ÷ 3 = 7, so the mean is 7. The median is the middle one once they are in order (6 above), and the mode is the value that appears most often."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MEDIAN_EXPLAIN"); }
    s__MEDIAN_EXPLAIN(py_str("The median is the middle one once the numbers are in order: 1, 4 and 11 is 4, and 1, 4, 6 and 11 is halfway between 4 and 6, so 5. It is the one to use when a few big numbers would drag a mean up, like house prices or wages. The mean is the total shared out evenly, and the mode is the value that appears most often."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MODE_EXPLAIN"); }
    s__MODE_EXPLAIN(py_str("The mode is the value that appears most often: in 2, 3, 3 and 7 it is 3, and if nothing repeats there is no mode at all. It is the one to use for the commonest answer rather than a middle one, like the most usual shoe size. The mean is the total shared out evenly, and the median is the middle one once the numbers are in order."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._MEAN_VERB"); }
    s__MEAN_VERB(py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._STATISTIC_WORDS"); }
    s__STATISTIC_WORDS(py_tuple(vec![py_str("average"), py_str("averages"), py_str("mean"), py_str("means"), py_str("median"), py_str("mode"), py_str("statistics")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._OTHER_STATISTICS"); }
    s__OTHER_STATISTICS(py_tuple(vec![py_str("average"), py_str("averages"), py_str("median"), py_str("mode")]));
    fn __data17() -> Py { py_tuple(vec![py_str("that"), py_str("this"), py_str("it"), py_str("these"), py_str("those"), py_str("you"), py_str("me"), py_str("we"), py_str("they"), py_str("he"), py_str("she"), py_str("them"), py_str("what"), py_str("whats"), py_str("how"), py_str("which")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._POINTS_AT_A_THING"); }
    s__POINTS_AT_A_THING(__data17());
    fn __data18() -> Py { py_tuple(vec![py_str("come round"), py_str("came round"), py_str("coming over round"), py_str("go round"), py_str("went round"), py_str("walk round"), py_str("run round"), py_str("ran round"), py_str("drive round"), py_str("drove round"), py_str("crawl round"), py_str("crawling round"), py_str("show round"), py_str("look round"), py_str("turn round"), py_str("round the corner"), py_str("round the house"), py_str("round the garden"), py_str("round the shops"), py_str("round the floor"), py_str("round the back"), py_str("round the world"), py_str("round here"), py_str("year round"), py_str("all year round"), py_str("round for dinner"), py_str("round for tea"), py_str("round for lunch"), py_str("round to see"), py_str("round trip"), py_str("round of drinks"), py_str("round of golf"), py_str("round of applause"), py_str("rounds of"), py_str("boxing"), py_str("boxer"), py_str("boxers"), py_str("boxercise"), py_str("round of boxing"), py_str("rounds of boxing"), py_str("a fight"), py_str("fights"), py_str("sparring"), py_str("round the bath"), py_str("round the bath and"), py_str("round a bath"), py_str("round the shower"), py_str("round the sink"), py_str("round the basin"), py_str("round the window"), py_str("round the window frame"), py_str("round the door"), py_str("round the worktop"), py_str("round the edges"), py_str("round the tiles"), py_str("round the skirting"), py_str("round the tiles and"), py_str("round the bottom"), py_str("round the toilet"), py_str("round the outside")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ROUND_IS_NOT_THE_VERB"); }
    s_ROUND_IS_NOT_THE_VERB(__data18());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._ROUND_IS_AROUND"); }
    s__ROUND_IS_AROUND(stdlib_call("re", "compile", &[py_str("\\brounds?\\s+(?:the|a|an|my|your|his|her|its|our|their|this|that)\\s+(?!(?:number|numbers|digit|digits|decimal|decimals|answer|answers|figure|figures|value|values|sum|total|result|nearest|whole|next)\\b)\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ROUNDING_NAMED"); }
    s_ROUNDING_NAMED(stdlib_call("re", "compile", &[py_str("\\bround(?:s|ed|ing)?\\s+(?:a\\s+|the\\s+|off\\s+|up\\s+|down\\s+)?(?:number|numbers|decimal|decimals|figure|figures|digit|digits|whole|thousand|thousands|hundred|hundreds|ten|tens)\\b|\\b(?:number|numbers|decimal|decimals|figure|figures|digit|digits)\\s+(?:is\\s+|are\\s+)?round(?:s|ed|ing)?\\b|\\bround(?:ed|ing)?\\s+(?:up|down)\\s+(?:the\\s+|a\\s+|your\\s+)?(?:price|prices|cost|bill|total|number|figure|amount|sum|pence|penny|pennies|pounds|pound|decimal|decimals|digit|digits)\\b|\\brounding\\b|\\bdecimal places?\\b|\\bsignificant figures?\\b|\\bnearest\\s+(?:whole|ten|tens|hundred|hundreds|thousand|thousands|million|penny|pence|pound|pounds|decimal|digit|digits|number|numbers|integer|unit|one|\\d)\\b|\\bround(?:s|ed|ing)?\\s+(?:it\\s+)?(?:to|at)\\s+(?:the\\s+)?nearest\\b|\\bmaths?\\b|\\bmathematics\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._FREE_TIME_ASK"); }
    s__FREE_TIME_ASK(py_tuple(vec![py_str("what can i do"), py_str("what should i do"), py_str("what do i do"), py_str("what to do"), py_str("shall i do"), py_str("should i do"), py_str("anything to do"), py_str("something to do"), py_str("things to do"), py_str("to kill"), py_str("kill an hour"), py_str("keep busy")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_LENGTH_OF_TIME"); }
    s__A_LENGTH_OF_TIME(stdlib_call("re", "compile", &[py_str("\\bhours?\\b|\\bminutes?\\b|\\bseconds?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._A_PAY_WORD"); }
    s__A_PAY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:pay|paid|pays|paying|salary|salaries|wage|wages|earn|earns|earning|rate|rates|hourly|take home|contract|overtime|per hour|a year|annual|annually|yearly|a month|monthly)\\b|\\d+(?:\\.\\d+)?\\s*(?:pounds?|quid|an hour|per hour)|\\bhow\\s+much\\s+(?:do|does|would|will|did)\\s+(?:i|you|we|they|it|that|he|she)\\s+(?:make|makes|get|take)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.PRIME_DEFINITION"); }
    s_PRIME_DEFINITION(py_str("A prime number is a whole number greater than 1 that divides only by 1 and itself: 2, 3, 5, 7, 11 and so on. 1 is not prime, and 2 is the only even prime."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.PRIME_TEST"); }
    s_PRIME_TEST(py_str(" To test one, try dividing it by 2, 3, 5, 7, 11 and the rest of the primes up to its square root: if none of them goes in exactly, it is prime. So 97 needs only a check against 2, 3, 5 and 7, since 11 squared is past it. Anything that ends in 0, 2, 4, 5, 6 or 8, or whose digits add up to a multiple of 3, is not prime."));
    fn __data20() -> Py { py_tuple(vec![py_str("wall"), py_str("walls"), py_str("wood"), py_str("woodwork"), py_str("paint"), py_str("painting"), py_str("painted"), py_str("primer"), py_str("undercoat"), py_str("metal"), py_str("gate"), py_str("gates"), py_str("door"), py_str("doors"), py_str("skirting"), py_str("ceiling"), py_str("plaster"), py_str("render"), py_str("mdf"), py_str("bare wood"), py_str("filler"), py_str("roller"), py_str("brush"), py_str("minis"), py_str("miniature"), py_str("prime minister"), py_str("amazon prime"), py_str("prime video"), py_str("prime day")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("solve"), py_str("solving"), py_str("formula"), py_str("method"), py_str("roots"), py_str("root"), py_str("factorise"), py_str("factorize"), py_str("factor"), py_str("how"), py_str("what"), py_str("work out"), py_str("explain"), py_str("equation"), py_str("equations")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("turf"), py_str("grass"), py_str("lawn"), py_str("carpet"), py_str("carpets"), py_str("carpeting"), py_str("flooring"), py_str("floor"), py_str("tiles"), py_str("tiling"), py_str("gravel"), py_str("soil"), py_str("decking"), py_str("paint"), py_str("wallpaper"), py_str("border"), py_str("grout"), py_str("adhesive"), py_str("plasterboard")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("why"), py_str("reason"), py_str("exist"), py_str("needed"), py_str("needs"), py_str("happen"), py_str("happens"), py_str("extra"), py_str("added"), py_str("invented"), py_str("what is"), py_str("whats"), py_str("mean")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("you mean"), py_str("i mean"), py_str("does that mean"), py_str("what do you"), py_str("that mean"), py_str("it mean"), py_str("this mean"), py_str("mean by"), py_str("portrait"), py_str("portrait mode"), py_str("bokeh"), py_str("selfie"), py_str("camera"), py_str("photo"), py_str("photos"), py_str("without being mean"), py_str("not being mean"), py_str("be mean"), py_str("being mean"), py_str("sound mean"), py_str("mean to"), py_str("mean about"), py_str("turn him down"), py_str("turn her down"), py_str("turn them down"), py_str("let him down"), py_str("let her down"), py_str("let them down")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("make with"), py_str("made with"), py_str("make using"), py_str("make from"), py_str("make out of"), py_str("what can i make"), py_str("what can i cook"), py_str("what should i make"), py_str("cook with"), py_str("cooking with"), py_str("fraction"), py_str("of an hour"), py_str("of a day"), py_str("hour and a half"), py_str("minutes"), py_str("seconds"), py_str("how long"), py_str("how many minutes"), py_str("electricity"), py_str("energy"), py_str("gas"), py_str("water"), py_str("parking"), py_str("charge"), py_str("charging"), py_str("battery"), py_str("kilowatt"), py_str("kwh"), py_str("boiler"), py_str("car"), py_str("van"), py_str("hire"), py_str("rent"), py_str("babysit"), py_str("childcare"), py_str("cleaner"), py_str("tradesman"), py_str("plumber"), py_str("electrician"), py_str("oven"), py_str("bake"), py_str("baked"), py_str("baking"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("temperature"), py_str("gas mark"), py_str("fan oven"), py_str("simmer"), py_str("simmering"), py_str("boil"), py_str("boiling"), py_str("marinate"), py_str("marinade"), py_str("prove"), py_str("proving"), py_str("knead"), py_str("chill"), py_str("chilling"), py_str("fridge"), py_str("freezer"), py_str("recipe"), py_str("casserole"), py_str("cake"), py_str("bread"), py_str("biscuits"), py_str("pastry"), py_str("pasties"), py_str("batter"), py_str("dough"), py_str("garage"), py_str("garages"), py_str("mechanic"), py_str("mechanics"), py_str("labour"), py_str("clutch"), py_str("cambelt"), py_str("mot"), py_str("servicing"), py_str("bodywork")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("no interest"), py_str("not interested"), py_str("hobby"), py_str("hobbies"), py_str("common interest"), py_str("in common"), py_str("of interest"), py_str("good rate"), py_str("good interest"), py_str("best rate"), py_str("best interest"), py_str("decent rate"), py_str("decent interest"), py_str("what rate"), py_str("which rate"), py_str("highest rate"), py_str("savings")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("marathon"), py_str("5k"), py_str("10k"), py_str("race"), py_str("run"), py_str("running"), py_str("ran"), py_str("training"), py_str("train"), py_str("distance"), py_str("miles"), py_str("km"), py_str("kilometres"), py_str("kilometers"), py_str("an hour"), py_str("hour"), py_str("past"), py_str("term"), py_str("price"), py_str("half price"), py_str("a day"), py_str("a week"), py_str("pint"), py_str("recipe"), py_str("cooking"), py_str("bottle"), py_str("glass"), py_str("cup"), py_str("use up"), py_str("use it up"), py_str("using up"), py_str("used up"), py_str("leftover"), py_str("left over"), py_str("leftovers"), py_str("tin"), py_str("tins"), py_str("tin of"), py_str("can"), py_str("cans"), py_str("can of"), py_str("jar"), py_str("jars"), py_str("packet"), py_str("packets"), py_str("packet of"), py_str("carton"), py_str("carton of"), py_str("tub"), py_str("pots"), py_str("pot"), py_str("punnet"), py_str("bag"), py_str("bags"), py_str("bunch"), py_str("tray"), py_str("half a tin"), py_str("half a can"), py_str("half a jar"), py_str("half a packet"), py_str("dose"), py_str("doses"), py_str("dosage"), py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("pills"), py_str("capsule"), py_str("capsules"), py_str("medication"), py_str("medicine"), py_str("medicines"), py_str("insulin"), py_str("steroid"), py_str("steroids"), py_str("inhaler"), py_str("inhalers"), py_str("antibiotic"), py_str("antibiotics"), py_str("statins"), py_str("painkillers"), py_str("antidepressants"), py_str("prescription")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("cold"), py_str("colds"), py_str("flu"), py_str("virus"), py_str("illness"), py_str("office"), py_str("park"), py_str("dog"), py_str("dogs"), py_str("paper round"), py_str("paper rounds"), py_str("newspaper round"), py_str("paperboy"), py_str("papergirl"), py_str("paper route"), py_str("deliver papers"), py_str("round of drinks"), py_str("round of golf"), py_str("in a round")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("android"), py_str("mobile"), py_str("tv"), py_str("television"), py_str("speaker"), py_str("speakers"), py_str("headphones"), py_str("airpods"), py_str("sound"), py_str("louder"), py_str("quieter"), py_str("quiet"), py_str("hear"), py_str("music"), py_str("podcast"), py_str("calls"), py_str("ringtone"), py_str("laptop"), py_str("computer"), py_str("telly"), py_str("soundbar"), py_str("sound bar"), py_str("radio"), py_str("earbuds"), py_str("turn up"), py_str("turn down"), py_str("mute"), py_str("volume up"), py_str("volume down"), py_str("the volume on")]) }
    fn __data19() -> Py { py_list(vec![pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("pi"), py_tuple(vec![py_str("pi"), py_tuple(vec![py_str("what"), py_str("value"), py_str("number"), py_str("3.14"), py_str("3.14159")])]), py_str("Pi (π) is about 3.14159: the circumference of any circle divided by its diameter. For most everyday sums, 3.14 is close enough.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("prime-number")); __args.push(py_tuple(vec![py_tuple(vec![py_str("prime"), py_str("prime number"), py_str("prime numbers")]), py_tuple(vec![py_str("what"), py_str("number"), py_str("definition"), py_str("mean"), py_str("work out"), py_str("how"), py_str("check"), py_str("test"), py_str("tell"), py_str("know")])])); __args.push(py_func("_prime_definition", |args: &[Py]| _prime_definition(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("define"), py_str("meaning"), py_str("explain"), py_str("how"), py_str("work out"), py_str("check"), py_str("tell")])), ("avoid".to_string(), __data20())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("percent-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("percentage"), py_str("percent"), py_str("percentages")]), py_tuple(vec![py_str("work"), py_str("calculate"), py_str("out"), py_str("how"), py_str("formula"), py_str("explain")])])); __args.push(py_str("To find a percentage of something, divide the amount by 100 and multiply by the percentage: 15% of 80 is 80 ÷ 100 × 15 = 12. To turn a fraction into a percentage, divide the top by the bottom and multiply by 100.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("explain"), py_str("formula"), py_str("calculate"), py_str("work out")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quadratic-formula")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quadratic"), py_str("quadratics"), py_str("quadratic equation"), py_str("quadratic equations"), py_str("quadratic formula")]), __data21()])); __args.push(py_str("Arrange it as ax² + bx + c = 0, then put the numbers into x = (-b ± √(b² - 4ac)) / 2a. If b² - 4ac is positive there are two answers, zero means one answer, and negative means none without imaginary numbers. Try factorising first when the numbers are small: x² + 5x + 6 is (x + 2)(x + 3), so x is -2 or -3.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("solve"), py_str("method"), py_str("explain"), py_str("roots"), py_str("formula")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bodmas")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bodmas"), py_str("bidmas"), py_str("order of operations")]), py_tuple(vec![py_str("what"), py_str("maths"), py_str("mean"), py_str("rules")])])); __args.push(py_str("Work in this order: brackets, then powers or roots, then multiplication and division left to right, then addition and subtraction left to right. So 2 + 3 × 4 is 14, not 20.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("how"), py_str("order"), py_str("mean")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("area-formula")); __args.push(py_tuple(vec![py_tuple(vec![py_str("area"), py_str("square metres"), py_str("square meters"), py_str("square footage"), py_str("square foot"), py_str("square feet"), py_str("m2"), py_str("sqm")]), py_tuple(vec![py_str("formula"), py_str("work out"), py_str("calculate"), py_str("how"), py_str("what"), py_str("many")])])); __args.push(py_str("For a rectangle, area = length × width; for a triangle, ½ × base × height; for a circle, π × radius². Keep all the measurements in the same unit. So a room 4m by 3m is 12 square metres, and an L-shaped floor is two rectangles added together.")); __kws.extend(vec![("avoid".to_string(), __data22()), ("ask".to_string(), py_tuple(vec![py_str("how"), py_str("formula"), py_str("calculate"), py_str("work")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leap-year-why")); __args.push(py_tuple(vec![py_tuple(vec![py_str("leap year"), py_str("leap years"), py_str("leap day"), py_str("february 29"), py_str("29 february"), py_str("29th of february")]), __data23()])); __args.push(py_str("A leap year is a year with 366 days and a 29th of February, because a year is really about 365 and a quarter: without the extra day the calendar would slip a day every four years. One correction keeps it exact, since the year is not quite a quarter longer - a century is only a leap year if it divides by 400, which is why 1900 was not one and 2000 was.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how many days"), py_str("how long"), py_str("how many weeks"), py_str("days in february")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fraction-decimal")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fraction")]), py_tuple(vec![py_str("decimal"), py_str("convert"), py_str("change")])])); __args.push(py_str("To turn a fraction into a decimal, divide the top number by the bottom: 3/4 is 3 ÷ 4 = 0.75. Some fractions repeat for ever, like 1/3 = 0.333…")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("convert"), py_str("change"), py_str("decimal")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("average-mean")); __args.push(py_tuple(vec![py_tuple(vec![py_str("average"), py_str("averages"), py_str("mean"), py_str("median"), py_str("mode"), py_str("statistics"), py_str("the mean"), py_str("the median"), py_str("the mode")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("work out"), py_str("calculate"), py_str("find"), py_str("difference"), py_str("define"), py_str("explain"), py_str("vs")])])); __args.push(py_func("_averages", |args: &[Py]| _averages(args))); __kws.extend(vec![("avoid".to_string(), __data24())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("vat-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("vat"), py_str("value added tax")]), py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("add"), py_str("take off"), py_str("remove"), py_str("price"), py_str("included"), py_str("charge"), py_str("deduct"), py_str("minus")])])); __args.push(py_str("VAT in the UK is 20%: add it by multiplying the price by 1.2, so £80 becomes £96, and take it off a VAT-inclusive price by dividing by 1.2, so £96 is £80 plus £16 of VAT. The VAT on a net price is simply a fifth of it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("add"), py_str("take off"), py_str("remove"), py_str("deduct")])), ("avoid".to_string(), py_tuple(vec![py_str("number"), py_str("registration"), py_str("register"), py_str("refund"), py_str("return"), py_str("rebate"), py_str("what is vat"), py_str("what's vat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hourly-rate")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hourly rate"), py_str("hourly pay"), py_str("rate an hour"), py_str("rate per hour"), py_str("per hour"), py_str("an hour")]), py_tuple(vec![py_str("work out"), py_str("calculate"), py_str("from my salary"), py_str("salary"), py_str("annual"), py_str("yearly"), py_str("take home"), py_str("contract"), py_str("hours")])])); __args.push(py_str("Divide the yearly pay by the hours you are actually paid for: £30,000 over 37.5 hours a week is £30,000 ÷ (37.5 × 52) = about £15.40 an hour, and only the paid weeks count, so unpaid breaks or a term-time contract change the figure. Take-home hourly pay uses the figure after tax and pension, not the salary.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("what")])), ("guard".to_string(), py_func("_not_a_spare_hour", |args: &[Py]| _not_a_spare_hour(args))), ("avoid".to_string(), __data25())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("compound-interest")); __args.push(py_tuple(vec![py_tuple(vec![py_str("compound interest"), py_str("interest rate"), py_str("interest rates"), py_str("compound")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("work out"), py_str("calculate"), py_str("means"), py_str("works"), py_str("worth"), py_str("better")])])); __args.push(py_str("Compound interest pays interest on the interest as well as on what you put in, so £1,000 at 5% is £1,050 after a year and £1,102.50 after two, because the second year pays 5% of £1,050. The rule of 72 is the shortcut: divide 72 by the rate to get the years it takes to double, so 5% doubles in about 14 years.")); __kws.extend(vec![("avoid".to_string(), __data26())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("half-of-a-number")); __args.push(py_tuple(vec![py_tuple(vec![py_str("half"), py_str("halve"), py_str("halves"), py_str("half of"), py_str("half of a number")]), py_tuple(vec![py_str("number"), py_str("numbers"), py_str("amount"), py_str("figure"), py_str("sum"), py_str("of a number"), py_str("work out"), py_str("find"), py_str("calculate"), py_str("how")])])); __args.push(py_str("Divide it by 2: half of 46 is 23, and half of 7 is 3.5. For a quarter, halve it twice; for 10 per cent divide by 10, and for 5 per cent halve that.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("find"), py_str("work out"), py_str("calculate"), py_str("of a number")])), ("avoid".to_string(), __data27())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ratio-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ratio"), py_str("ratios"), py_str("proportion"), py_str("proportions"), py_str("share in a ratio")]), py_tuple(vec![py_str("how"), py_str("work"), py_str("out"), py_str("share"), py_str("divide"), py_str("simplify"), py_str("what"), py_str("explain"), py_str("split"), py_str("mean"), py_str("means")])])); __args.push(py_str("Add the parts of the ratio first, then divide the amount by that: sharing £50 in the ratio 2:3 is 5 parts, so each part is £10 and the shares are £20 and £30. To simplify a ratio, divide both numbers by the same thing, so 10:15 becomes 2:3.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("explain"), py_str("mean"), py_str("work"), py_str("share"), py_str("simplify")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("standard-form")); __args.push(py_tuple(vec![py_tuple(vec![py_str("standard form"), py_str("scientific notation"), py_str("powers of ten"), py_str("powers of 10")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("write"), py_str("maths"), py_str("explain"), py_str("mean"), py_str("convert"), py_str("big"), py_str("small"), py_str("number"), py_str("numbers")])])); __args.push(py_str("Standard form writes a number as something between 1 and 10 times a power of ten: 4,500 is 4.5 x 10³, and 0.0002 is 2 x 10⁻⁴. Count how many places the decimal point moves, and make it negative when the number is below 1. To multiply, multiply the front numbers and add the powers.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("how"), py_str("write"), py_str("explain"), py_str("convert"), py_str("mean")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("simultaneous-equations")); __args.push(py_tuple(vec![py_tuple(vec![py_str("simultaneous equations"), py_str("simultaneous"), py_str("two equations"), py_str("equations with two")]), py_tuple(vec![py_str("solve"), py_str("how"), py_str("what"), py_str("method"), py_str("explain"), py_str("work"), py_str("find")])])); __args.push(py_str("Write one equation under the other and add or subtract them to lose a letter: 2x + y = 7 and x + y = 4 gives x = 3 straight away, then y = 1 from either one. If nothing cancels, multiply one equation until it does. Put both numbers back into both equations to check.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("solve"), py_str("explain"), py_str("method")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("factors-and-multiples")); __args.push(py_tuple(vec![py_tuple(vec![py_str("factor"), py_str("factors"), py_str("multiple"), py_str("multiples"), py_str("prime factor"), py_str("prime factors"), py_str("hcf"), py_str("lcm"), py_str("highest common factor"), py_str("lowest common multiple")]), py_tuple(vec![py_str("what"), py_str("difference"), py_str("how"), py_str("mean"), py_str("explain"), py_str("find"), py_str("vs"), py_str("between")])])); __args.push(py_str("A factor divides into a number exactly - 1, 2, 3, 4, 6 and 12 for 12 - and a multiple is in its times table: 12, 24, 36. A prime factor is a factor that is also prime, so 12 is 2 x 2 x 3. The highest common factor of two numbers is the biggest one they share; the lowest common multiple is the smallest one they both divide into.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("how"), py_str("difference"), py_str("mean"), py_str("explain"), py_str("find")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("negative-numbers")); __args.push(py_tuple(vec![py_tuple(vec![py_str("negative number"), py_str("negative numbers"), py_str("minus number"), py_str("minus numbers")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("add"), py_str("subtract"), py_str("multiply"), py_str("divide"), py_str("explain"), py_str("mean"), py_str("work")])])); __args.push(py_str("Use a number line: adding a negative moves you left, subtracting a negative moves you right, so 5 - (-3) is 8. Multiplying or dividing two negatives gives a positive, and one negative gives a negative: -2 x -3 is 6, and -6 / 2 is -3.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("how"), py_str("explain"), py_str("mean"), py_str("add"), py_str("subtract"), py_str("multiply"), py_str("work")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rounding-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rounding"), py_str("round"), py_str("decimal places"), py_str("significant figures"), py_str("nearest")]), py_tuple(vec![py_str("how"), py_str("what"), py_str("rules"), py_str("explain"), py_str("mean"), py_str("number"), py_str("work")])])); __args.push(py_str("Look at the digit just after the place you are keeping: 5 or more rounds the last kept digit up, 4 or less leaves it. So 3.456 to two decimal places is 3.46 and to the nearest whole number it is 3. Round once, at the end of a sum, not at every step.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_ROUND_IS_NOT_THE_VERB(), &__data28())), ("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("rules"), py_str("explain"), py_str("mean"), py_str("work")])), ("guard".to_string(), py_func("_rounding_is_the_ask", |args: &[Py]| _rounding_is_the_ask(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("triangle-angles")); __args.push(py_tuple(vec![py_tuple(vec![py_str("angle"), py_str("angles"), py_str("degree"), py_str("degrees")]), py_tuple(vec![py_str("triangle"), py_str("triangles"), py_str("add up"), py_str("add up to"), py_str("inside"), py_str("corner"), py_str("corners"), py_str("total"), py_str("sum"), py_str("180")])])); __args.push(py_str("The three angles inside any triangle add up to 180 degrees, so if you know two of them, take their total away from 180 for the third: 60 and 70 leaves 50. An equilateral triangle is 60 degrees each, and a right angle is 90. In a quadrilateral (a four-sided shape) the four angles add up to 360.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("add up"), py_str("add up to"), py_str("total"), py_str("sum"), py_str("what"), py_str("how many"), py_str("inside")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reading-a-graph")); __args.push(py_tuple(vec![py_tuple(vec![py_str("graph"), py_str("graphs"), py_str("line graph"), py_str("bar chart"), py_str("bar graph"), py_str("axis"), py_str("axes")]), py_tuple(vec![py_str("read"), py_str("reading"), py_str("how"), py_str("what"), py_str("plot"), py_str("draw"), py_str("explain"), py_str("mean")])])); __args.push(py_str("Read what each axis is measured in first, including the units and the step between the numbers. To read a value, go up from the number you know to the line, then across to the other axis. A straight line means the same change each time; a curve means it changes faster or slower.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("read"), py_str("explain"), py_str("mean")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fractions-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("add fractions"), py_str("adding fractions"), py_str("subtract fractions"), py_str("subtracting fractions"), py_str("common denominator"), py_str("fractions with different")]), py_tuple(vec![py_str("how"), py_str("add"), py_str("subtract"), py_str("different"), py_str("bottom"), py_str("denominator"), py_str("common"), py_str("explain")])])); __args.push(py_str("Give them the same bottom number first, then add or subtract the tops: 1/2 + 1/3 is 3/6 + 2/6, which is 5/6. Use the lowest bottom both numbers divide into, which is 6 here and 4 for 1/2 + 1/4, then cancel the answer down.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("add"), py_str("subtract"), py_str("explain"), py_str("different")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("volume-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("volume"), py_str("cubic")]), py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("formula"), py_str("box"), py_str("cube"), py_str("cylinder"), py_str("what")])])); __args.push(py_str("For a box it is length x width x height, so 10cm by 5cm by 2cm is 100cm³. A cylinder is pi x radius² x height, and a sphere is 4/3 x pi x radius³. Put every measurement in the same unit before multiplying.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("calculate"), py_str("formula"), py_str("work")])), ("avoid".to_string(), __data29())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("circumference-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("circumference")]), py_tuple(vec![py_str("how"), py_str("work out"), py_str("calculate"), py_str("formula"), py_str("what"), py_str("circle"), py_str("mean")])])); __args.push(py_str("The circumference is pi times the diameter, or 2 x pi x radius: a circle with a radius of 5 has one of about 31.4. Pi is 3.14 to two decimal places. The area of the same circle is pi x radius², which is a different sum.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("calculate"), py_str("formula"), py_str("work"), py_str("mean")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("simplify-fraction-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("simplify a fraction"), py_str("simplifying fractions"), py_str("simplify fractions"), py_str("simplest form"), py_str("lowest terms"), py_str("cancel down"), py_str("cancel a fraction")]), py_tuple(vec![py_str("how"), py_str("what"), py_str("explain"), py_str("mean"), py_str("simplify"), py_str("fraction"), py_str("fractions")])])); __args.push(py_str("Divide the top and the bottom by the same number until nothing goes into both: 6/8 divides by 2 to give 3/4, and that is as far as it goes. The number to start with is the highest common factor of the two.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("explain"), py_str("mean"), py_str("simplify")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("decimals-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("decimal"), py_str("decimals"), py_str("decimal point"), py_str("decimal places")]), py_tuple(vec![py_str("multiply"), py_str("multiplying"), py_str("divide"), py_str("dividing"), py_str("how"), py_str("work"), py_str("explain"), py_str("times")])])); __args.push(py_str("Multiply as if the points were not there, then put the point back with as many decimal places as the two numbers had between them: 0.3 x 0.4 is 0.12. To divide, move both points the same number of places until the number you are dividing by is whole: 1.2 / 0.4 is 12 / 4, which is 3.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("explain"), py_str("work"), py_str("multiply"), py_str("divide"), py_str("times")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.ENTRIES"); }
    s_ENTRIES(__data19());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths._DEFINITION_ASK"); }
    s__DEFINITION_ASK(py_tuple(vec![py_str("what is"), py_str("what's"), py_str("whats"), py_str("what are"), py_str("what does"), py_str("what do"), py_str("how does"), py_str("define"), py_str("definition"), py_str("meaning"), py_str("mean"), py_str("explain")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("decimal-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("decimal"), py_str("decimals")]), py_tuple(vec![py_str("what"), py_str("whats"), py_str("is a"), py_str("are"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("number")])])); __args.push(py_str("A decimal is a way of writing a number using place values for tenths, hundredths and so on, with a point after the units: 0.75 means seven tenths and five hundredths, which is the same as 3/4. The digits after the point tell you the place value, so 0.5 is five tenths and 0.05 is five hundredths.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("betting"), py_str("odds"), py_str("bookmaker"), py_str("currency"), py_str("exchange rate"), py_str("binary"), py_str("hexadecimal"), py_str("hex"), py_str("binary code"), py_str("timesheet"), py_str("invoice")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data30() -> Py { py_tuple(vec![py_str("of"), py_str("increase"), py_str("decrease"), py_str("change"), py_str("off"), py_str("discount"), py_str("tip"), py_str("vat"), py_str("tax"), py_str("interest"), py_str("raise"), py_str("calculator"), py_str("how much")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("percentage-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("percentage"), py_str("percentages"), py_str("percent")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain")])])); __args.push(py_str("A percentage is a fraction of 100: 25% means 25 out of every 100, or a quarter, and 100% is the whole thing. To turn a fraction into a percentage, divide the top by the bottom and multiply by 100, so 3/5 is 60%.")); __kws.extend(vec![("avoid".to_string(), __data30()), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data31() -> Py { py_tuple(vec![py_str("of"), py_str("live"), py_str("living"), py_str("move"), py_str("moving"), py_str("neighbourhood"), py_str("neighborhood"), py_str("postcode"), py_str("safe"), py_str("rough"), py_str("nice"), py_str("local area"), py_str("catchment"), py_str("deprived"), py_str("crime"), py_str("in my area"), py_str("my area"), py_str("in our area"), py_str("our area"), py_str("in the area"), py_str("this area"), py_str("near me"), py_str("near us"), py_str("nearby"), py_str("nearest"), py_str("around here"), py_str("round here"), py_str("where i live"), py_str("gyms"), py_str("gym"), py_str("code"), py_str("codes"), py_str("dialling"), py_str("dialing"), py_str("zip"), py_str("51"), py_str("52"), py_str("area 51"), py_str("area 52"), py_str("place"), py_str("place name")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("area-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("area")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("measured"), py_str("measure")])])); __args.push(py_str("Area is the amount of surface inside a shape, measured in square units: a room 4 m by 3 m has an area of 12 square metres. For a rectangle it is length times width, for a triangle half base times height, and for a circle pi times the radius squared.")); __kws.extend(vec![("avoid".to_string(), __data31()), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("perimeter-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("perimeter")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("measured"), py_str("measure")])])); __args.push(py_str("The perimeter is the distance all the way round the outside of a shape, found by adding the lengths of its sides: a room 4 m by 3 m has a perimeter of 14 m. Area is the surface inside; the perimeter is the edge round it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("of"), py_str("fence"), py_str("fencing"), py_str("security"), py_str("alarm"), py_str("guard"), py_str("patrol"), py_str("wall"), py_str("boundary"), py_str("camera"), py_str("sensor"), py_str("dog")])), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data32() -> Py { py_tuple(vec![py_str("weather"), py_str("speed"), py_str("wind"), py_str("temperature"), py_str("rate"), py_str("mortgage"), py_str("interest"), py_str("tariff"), py_str("energy"), py_str("salary"), py_str("income"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("mood"), py_str("quality"), py_str("signal"), py_str("power")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("variable-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("variable"), py_str("variables")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("maths"), py_str("math"), py_str("algebra")])])); __args.push(py_str("In maths a variable is a letter standing for a number you do not know yet, or one that can change, such as x in 3x + 4 = 19. You work with it as if it were a number, then solve to find what it must be.")); __kws.extend(vec![("avoid".to_string(), __data32()), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data33() -> Py { py_tuple(vec![py_str("of"), py_str("rain"), py_str("weather"), py_str("forecast"), py_str("rolling"), py_str("roll"), py_str("dice"), py_str("die"), py_str("coin"), py_str("cards"), py_str("lottery"), py_str("odds"), py_str("chance of")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("probability-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("probability")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain")])])); __args.push(py_str("Probability is how likely something is, written as a number from 0 (impossible) to 1 (certain): a fair coin landing heads has a probability of 1/2, or 50%. You find it by dividing the number of ways the thing can happen by the number of equally likely outcomes.")); __kws.extend(vec![("avoid".to_string(), __data33()), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("square-number-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("square number"), py_str("square numbers"), py_str("perfect square"), py_str("perfect squares")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain")])])); __args.push(py_str("A square number is what you get when a whole number is multiplied by itself: 1, 4, 9, 16, 25 and so on, because 4 × 4 = 16. It is called square because that many dots make a square grid, and it is always a whole number.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("metre"), py_str("meter"), py_str("foot"), py_str("feet"), py_str("mile"), py_str("room"), py_str("floor"), py_str("roof"), py_str("garden"), py_str("tile"), py_str("tiles"), py_str("root")])), ("ask".to_string(), g__DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numerator-denominator-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("numerator"), py_str("numerators"), py_str("denominator"), py_str("denominators")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("fraction"), py_str("top"), py_str("bottom")])])); __args.push(py_str("In a fraction the numerator is the top number, the one being counted, and the denominator is the bottom number, the one that says how many equal pieces the whole is cut into. In 3/4 the numerator is 3 and the denominator is 4, so you have three of four equal pieces.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("lowest common denominator"), py_str("common denominator"), py_str("idiom"), py_str("politics"), py_str("tv"), py_str("television"), py_str("film"), py_str("music"), py_str("audience")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data34() -> Py { py_tuple(vec![py_str("what is"), py_str("what's"), py_str("what are"), py_str("what does"), py_str("what do"), py_str("how does"), py_str("how do"), py_str("how is"), py_str("how are"), py_str("why is"), py_str("why are"), py_str("why does"), py_str("explain"), py_str("tell me about"), py_str("define")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.maths.MATHS_DEFINITION_ASK"); }
    s_MATHS_DEFINITION_ASK(__data34());
    fn __data35() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("number"), py_str("degrees"), py_str("degrees"), py_str("measure")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("camera"), py_str("photo"), py_str("photos"), py_str("photography"), py_str("shoot"), py_str("shooting"), py_str("best"), py_str("wrong"), py_str("fishing"), py_str("fish"), py_str("park"), py_str("parking"), py_str("at an angle"), py_str("an angle on"), py_str("of a roof"), py_str("roof"), py_str("roofs"), py_str("roof pitch"), py_str("of a wall")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("angle-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("angle"), py_str("angles")]), __data35()])); __args.push(py_str("An angle is the amount of turn between two lines that meet at a point, measured in degrees: a quarter turn is 90 degrees, a half turn 180 and a full turn 360. A right angle is exactly 90, an acute angle is less than that and an obtuse one is between 90 and 180.")); __kws.extend(vec![("avoid".to_string(), __data36()), ("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("integer-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("integer"), py_str("integers")]), py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("number"), py_str("numbers"), py_str("whole")])])); __args.push(py_str("An integer is a whole number - positive, negative or zero - with no fraction or decimal part. So 4, -7 and 0 are integers, while 2.5 and 1/3 are not; the integers are the counting numbers, their negatives, and zero.")); __kws.extend(vec![("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data37() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("number"), py_str("numbers"), py_str("squared"), py_str("index")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("electricity"), py_str("electric"), py_str("electrical"), py_str("solar"), py_str("nuclear"), py_str("wind"), py_str("battery"), py_str("outage"), py_str("cut"), py_str("cuts"), py_str("station"), py_str("plant"), py_str("political"), py_str("government"), py_str("prime"), py_str("minister"), py_str("president"), py_str("abuse"), py_str("corrupt"), py_str("money"), py_str("bank"), py_str("save"), py_str("saving")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("power-maths-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("power"), py_str("powers"), py_str("power of"), py_str("exponent"), py_str("exponents")]), __data37()])); __args.push(py_str("In maths a power tells you how many times to multiply a number by itself: 3 to the power 4 means 3 x 3 x 3 x 3, which is 81. The small raised number is the exponent and the number being multiplied is the base; a square is a power of 2 and a cube is a power of 3.")); __kws.extend(vec![("avoid".to_string(), __data38()), ("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data39() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("algebra"), py_str("number"), py_str("numbers"), py_str("equation")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("facial"), py_str("face"), py_str("faces"), py_str("art"), py_str("artistic"), py_str("creative"), py_str("freedom"), py_str("gene"), py_str("genes"), py_str("protein"), py_str("protein expression"), py_str("emotional"), py_str("feelings"), py_str("yourself"), py_str("myself"), py_str("of interest"), py_str("expression of interest"), py_str("of concern"), py_str("letter")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("expression-maths-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("expression"), py_str("expressions"), py_str("algebraic expression")]), __data39()])); __args.push(py_str("In maths an expression is a group of numbers, letters and operations with no equals sign, such as 3x + 4 or 2(a - b). It can be simplified or evaluated, but it is not a statement that two things are equal - that is an equation.")); __kws.extend(vec![("avoid".to_string(), __data40()), ("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data41() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("algebra"), py_str("number"), py_str("numbers"), py_str("letter"), py_str("variable")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coefficient-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coefficient"), py_str("coefficients")]), __data41()])); __args.push(py_str("A coefficient is the number multiplying a letter in an expression: in 3x the coefficient is 3, and in -5y squared it is -5. If a letter stands alone the coefficient is 1, because x means 1x.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("drag"), py_str("lift"), py_str("friction"), py_str("correlation"), py_str("alpha"), py_str("beta"), py_str("regression")])), ("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data42() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("algebra"), py_str("expression"), py_str("equation"), py_str("number"), py_str("numbers"), py_str("add")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("school"), py_str("school term"), py_str("term dates"), py_str("dates"), py_str("contract"), py_str("lease"), py_str("insurance"), py_str("loan"), py_str("mortgage"), py_str("pregnancy"), py_str("pregnant"), py_str("fixed term"), py_str("long term"), py_str("short term"), py_str("terms and conditions"), py_str("conditions"), py_str("half term"), py_str("autumn"), py_str("spring"), py_str("summer"), py_str("of endearment"), py_str("endearment"), py_str("of office"), py_str("of address"), py_str("term of")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("term-maths-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("term"), py_str("terms"), py_str("like terms")]), __data42()])); __args.push(py_str("In maths a term is one part of an expression, separated from the others by plus or minus signs: in 3x + 4y - 7 there are three terms, 3x, 4y and -7. Terms with the same letter part are like terms and can be added together, so 2x + 3x becomes 5x.")); __kws.extend(vec![("avoid".to_string(), __data43()), ("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data44() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("shape"), py_str("shapes"), py_str("line"), py_str("mirror"), py_str("turn")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("symmetry-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("symmetry"), py_str("symmetrical"), py_str("line of symmetry"), py_str("lines of symmetry")]), __data44()])); __args.push(py_str("Symmetry is when a shape or object looks the same after a fold, turn or flip. A line of symmetry divides a shape into two halves that are mirror images - a square has four, a circle has infinitely many - and rotational symmetry is how many times it matches in a full turn.")); __kws.extend(vec![("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data45() -> Py { py_tuple(vec![py_str("what"), py_str("define"), py_str("definition"), py_str("mean"), py_str("meaning"), py_str("explain"), py_str("called"), py_str("maths"), py_str("mathematics"), py_str("shape"), py_str("shapes"), py_str("sides"), py_str("closed")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("polygon-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("polygon"), py_str("polygons")]), __data45()])); __args.push(py_str("A polygon is a closed flat shape made only of straight sides that do not cross. A triangle has 3 sides, a quadrilateral 4, a pentagon 5 and a hexagon 6; a circle is not a polygon because its edge is curved.")); __kws.extend(vec![("ask".to_string(), g_MATHS_DEFINITION_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("equation-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("equation"), py_str("equations")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("difference"), py_str("expression")])])); __args.push(py_str("An equation says two things are equal, and the equals sign is the point: 3x + 4 = 19 is an equation. An expression such as 3x + 4 has no equals sign, so it can be simplified but not solved. Solving an equation means finding the number that makes both sides balance.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("solve"), py_str("solving"), py_str("work out"), py_str("find x"), py_str("value of x"), py_str("simultaneous"), py_str("quadratic"), py_str("rearrange"), py_str("balance"), py_str("check"), py_str("steps")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data46() -> Py { py_tuple(vec![py_str("solve"), py_str("solving"), py_str("simplify"), py_str("factorise"), py_str("factorize"), py_str("expand"), py_str("rearrange"), py_str("brackets"), py_str("simultaneous"), py_str("quadratic"), py_str("worksheet"), py_str("homework"), py_str("test"), py_str("exam")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("algebra-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("algebra"), py_str("algebraic")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("used"), py_str("use"), py_str("basic"), py_str("basics")])])); __args.push(py_str("Algebra is arithmetic with letters standing in for numbers, so a rule works for every number at once: if a notebook costs n pounds, three cost 3n. The letters are called variables, and the usual job is to rearrange an equation until the letter is alone.")); __kws.extend(vec![("avoid".to_string(), __data46()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data47() -> Py { py_tuple(vec![py_str("how"), py_str("work out"), py_str("share"), py_str("sharing"), py_str("split"), py_str("divide"), py_str("simplify"), py_str("calculate"), py_str("smoothie"), py_str("coffee"), py_str("paint"), py_str("mortar"), py_str("concrete"), py_str("cement"), py_str("dough"), py_str("bread"), py_str("rice"), py_str("porridge")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ratio-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ratio"), py_str("ratios")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("difference"), py_str("proportion")])])); __args.push(py_str("A ratio compares two amounts by division, so a ratio of 2:3 means the second amount is one and a half times the first. A recipe using 2 cups of flour to 3 of milk is in the ratio 2:3, and scaling both numbers by the same amount keeps the ratio the same.")); __kws.extend(vec![("avoid".to_string(), __data47()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("parallelogram-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("parallelogram"), py_str("parallelograms")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("area"), py_str("shape")])])); __args.push(py_str("A parallelogram is a four-sided shape with both pairs of opposite sides parallel and equal, so its opposite angles are equal too. A rectangle is a special parallelogram with right angles. Its area is the base times the perpendicular height, not the slanted side.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("parking"), py_str("park"), py_str("car"), py_str("road"), py_str("drive"), py_str("space"), py_str("space-saving"), py_str("shelf")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rhombus-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rhombus"), py_str("rhombuses"), py_str("rhombi")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("area"), py_str("shape"), py_str("diamond")])])); __args.push(py_str("A rhombus is a four-sided shape with all four sides the same length and both pairs of opposite sides parallel; a square is a rhombus with right angles, and a diamond shape is one too. Its area is half the product of its two diagonals.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("ring"), py_str("engagement"), py_str("wedding"), py_str("jewellery"), py_str("jewelry"), py_str("diamond"), py_str("carat"), py_str("brilliant"), py_str("mine"), py_str("stone"), py_str("ring size")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("trapezium-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("trapezium"), py_str("trapeziums"), py_str("trapezoid"), py_str("trapezoids")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("area"), py_str("shape")])])); __args.push(py_str("A trapezium is a four-sided shape with exactly one pair of parallel sides; in the US the same shape is usually called a trapezoid. Its area is half the sum of the two parallel sides times the height between them.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("bone"), py_str("wrist"), py_str("hand"), py_str("thumb"), py_str("carpal"), py_str("table"), py_str("desk"), py_str("garden"), py_str("land")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data48() -> Py { py_tuple(vec![py_str("ice"), py_str("ice cube"), py_str("stock"), py_str("stock cube"), py_str("bouillon"), py_str("sugar cube"), py_str("rubik"), py_str("rubik's"), py_str("puzzle"), py_str("cube steak"), py_str("cubicle"), py_str("van"), py_str("van hire"), py_str("storage"), py_str("packing"), py_str("freezer"), py_str("whisky"), py_str("whiskey"), py_str("drink"), py_str("number"), py_str("numbers")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cube-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cube"), py_str("cubes")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A cube is a solid with six square faces of the same size, twelve edges and eight corners: a sugar lump and a die are cubes. Its volume is the side multiplied by itself three times, so a 2 cm cube holds 8 cm³, and its surface area is six times one face.")); __kws.extend(vec![("avoid".to_string(), __data48()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cuboid-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cuboid"), py_str("cuboids")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A cuboid is a box shape with six rectangular faces, like a brick or a cereal box. Its volume is length times width times height, and a cube is the special cuboid where all three are equal.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("bone"), py_str("foot"), py_str("feet"), py_str("ankle"), py_str("toe"), py_str("wrist"), py_str("fracture"), py_str("tarsal")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data49() -> Py { py_tuple(vec![py_str("public"), py_str("social"), py_str("political"), py_str("private"), py_str("influence"), py_str("sphere of"), py_str("work"), py_str("home"), py_str("life"), py_str("music"), py_str("fashion"), py_str("media"), py_str("online"), py_str("dyson")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sphere-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sphere"), py_str("spheres")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A sphere is the solid round shape where every point on the surface is the same distance from the centre, like a ball or a marble. Its volume is 4/3 times pi times the radius cubed, and its surface area is 4 times pi times the radius squared.")); __kws.extend(vec![("avoid".to_string(), __data49()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data50() -> Py { py_tuple(vec![py_str("gas"), py_str("oxygen"), py_str("gas bottle"), py_str("bottle"), py_str("lock"), py_str("brake"), py_str("hydraulic"), py_str("clutch"), py_str("engine"), py_str("boiler"), py_str("hot water"), py_str("mower"), py_str("car"), py_str("van"), py_str("propane"), py_str("butane"), py_str("co2"), py_str("compressed")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cylinder-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cylinder"), py_str("cylinders")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A cylinder has two flat circular ends and one curved side, like a tin or a tube. Its volume is pi times the radius squared times the height, and the curved surface is 2 times pi times the radius times the height.")); __kws.extend(vec![("avoid".to_string(), __data50()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cone-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cone"), py_str("cones")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A cone has one circular base and a curved side that comes to a point, like a party hat or a road cone. Its volume is one third of the cylinder with the same base and height: 1/3 times pi times the radius squared times the height.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("ice cream"), py_str("pine"), py_str("waffle"), py_str("nose"), py_str("hedge"), py_str("plant"), py_str("flower"), py_str("evergreen"), py_str("fir"), py_str("eye"), py_str("retina")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data51() -> Py { py_tuple(vec![py_str("political"), py_str("politics"), py_str("politic"), py_str("media"), py_str("public"), py_str("social"), py_str("through the prism"), py_str("prism of"), py_str("optics"), py_str("rainbow"), py_str("light"), py_str("camera"), py_str("lens"), py_str("glasses")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("prism-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("prism"), py_str("prisms")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A prism is a solid with the same cross-section all the way along, like a triangular prism or a long box. Its volume is the area of that cross-section times the length. A cube and a cuboid are prisms too.")); __kws.extend(vec![("avoid".to_string(), __data51()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data52() -> Py { py_tuple(vec![py_str("scheme"), py_str("schemes"), py_str("scam"), py_str("selling"), py_str("food"), py_str("diet"), py_str("egypt"), py_str("giza"), py_str("tomb"), py_str("pharaoh"), py_str("saqqara"), py_str("maya"), py_str("aztec"), py_str("chichen"), py_str("tour"), py_str("ticket"), py_str("hotel"), py_str("great pyramid"), py_str("pyramid of giza")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pyramid-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pyramid"), py_str("pyramids")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("volume"), py_str("surface"), py_str("shape")])])); __args.push(py_str("A pyramid has a flat base and triangular faces that meet at a point above it; the base can be any polygon, and a square-based pyramid is the one people picture. Its volume is one third of the area of the base times the height.")); __kws.extend(vec![("avoid".to_string(), __data52()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data53() -> Py { py_tuple(vec![py_str("shader"), py_str("graphics"), py_str("blender"), py_str("3d"), py_str("game"), py_str("engine"), py_str("computer"), py_str("software"), py_str("pharma"), py_str("pharmaceutical"), py_str("stock"), py_str("share"), py_str("nasdaq")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("vertex-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("vertex"), py_str("vertices")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("corner"), py_str("edge"), py_str("edges")])])); __args.push(py_str("A vertex is a corner where two or more edges meet: a cube has eight vertices and a triangle has three. The plural is vertices, and in a graph a vertex is a point with lines, called edges, joining it to other points.")); __kws.extend(vec![("avoid".to_string(), __data53()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cube-number-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cube number"), py_str("cube numbers"), py_str("cubed number"), py_str("cubed numbers")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain")])])); __args.push(py_str("A cube number is what you get when a whole number is multiplied by itself three times: 1, 8, 27, 64, because 4 × 4 × 4 = 64. It is written 4³ and read as four cubed; a square number is the same idea with two multiplications.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("solid"), py_str("shape"), py_str("sugar"), py_str("ice"), py_str("stock"), py_str("dice"), py_str("die"), py_str("rubik"), py_str("box"), py_str("volume"), py_str("faces"), py_str("root")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data54() -> Py { py_tuple(vec![py_str("cooker"), py_str("oven"), py_str("hob"), py_str("car"), py_str("electric"), py_str("driving"), py_str("mile"), py_str("miles"), py_str("mountain"), py_str("golf"), py_str("shooting"), py_str("rifle"), py_str("archery"), py_str("price"), py_str("product"), py_str("salary"), py_str("age"), py_str("temperature"), py_str("tide"), py_str("wave"), py_str("frequency"), py_str("radio"), py_str("wifi"), py_str("signal"), py_str("exercise"), py_str("motion"), py_str("kitchen"), py_str("range hood"), py_str("free range"), py_str("range rover"), py_str("long range")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("range-maths-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("range")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain"), py_str("maths"), py_str("math"), py_str("statistics"), py_str("data"), py_str("numbers")])])); __args.push(py_str("In maths the range is the gap between the smallest and largest number: for 3, 7, 9 and 15 it is 15 - 3 = 12. Put the numbers in order first, then subtract the smallest from the largest.")); __kws.extend(vec![("avoid".to_string(), __data54()), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("place-value-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("place value"), py_str("place values")]), py_tuple(vec![py_str("what"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("explain")])])); __args.push(py_str("Place value is what a digit is worth because of where it sits: in 4,582 the 4 is four thousands, the 5 is five hundreds, the 8 is eight tens and the 2 is two ones. The same digit means different things in different columns, which is why 0.5 is five tenths and 0.05 is five hundredths.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("parking"), py_str("park"), py_str("house"), py_str("market"), py_str("asset"), py_str("valuation"), py_str("property"), py_str("invest"), py_str("estate"), py_str("neighbourhood"), py_str("neighborhood"), py_str("area")])), ("ask".to_string(), g__DEFINITION_ASK()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
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
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut places: Py = Py::None;
    let mut shown: Py = Py::None;
    let _ = py_str("A number the way a person writes it: 12, 2.5, 1,081.");
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), -1, &",")); py_string(__s) };
    }
    shown = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".2f")); py_string(__s) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(2), Py::Int(3), Py::Int(4)]));
    for __item in __iter1 {
    places = __item.clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&call_builtin("float", &[{ let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".{}f")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)), &value.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    shown = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".{}f")); py_string(__s) };
    break;
    }
    }
    return mcall(&mcall(&shown.clone(), "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    Py::None
}
pub fn _parse(args: &[Py]) -> Py {
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
    let mut raw: Py = Py::None;
    let _ = py_str("The numbers in a line, in order.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("\\d+(?:\\.\\d+)?"), mcall(&text.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    raw = __item.clone();
    let __try2 = exc::catch(|| {
    let _ = mcall(&found.clone(), "append", &[call_builtin("float", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return found.clone();
    Py::None
}
pub fn _eval(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut node: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "node") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: node"))) };
    let mut left: Py = Py::None;
    let mut right: Py = Py::None;
    if py_truthy(&call_builtin("isinstance", &[node.clone(), stdlib_attr("ast", "Expression").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return _eval(&[gattr(&node.clone(), "body")]);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[node.clone(), stdlib_attr("ast", "Constant").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[gattr(&node.clone(), "value"), py_tuple(vec![py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e))), py_func("float", |args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return gattr(&node.clone(), "value");
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[node.clone(), stdlib_attr("ast", "BinOp").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BIN_OPS(), &call_builtin("type", &[gattr(&node.clone(), "op")]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![_eval(&[gattr(&node.clone(), "left")]), _eval(&[gattr(&node.clone(), "right")])]), 2, None);
    left = __unpk1[0].clone();
    right = __unpk1[1].clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[gattr(&node.clone(), "op"), stdlib_attr("ast", "Pow").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("abs", &[right.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(64)) } }) {
    match call_builtin("ValueError", &[py_str("exponent too large")]).unwrap_or_else(|e| exc::raise(e)) {
        Py::Exc(x) => exc::raise((*x).clone()),
        other => exc::raise(exc::PyExc::new("RuntimeError", &py_str_of(&other))),
    }
    }
    return py_call(&geti(&g__BIN_OPS(), &call_builtin("type", &[gattr(&node.clone(), "op")]).unwrap_or_else(|e| exc::raise(e))), &[left.clone(), right.clone()]);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[node.clone(), stdlib_attr("ast", "UnaryOp").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__UNARY_OPS(), &call_builtin("type", &[gattr(&node.clone(), "op")]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_call(&geti(&g__UNARY_OPS(), &call_builtin("type", &[gattr(&node.clone(), "op")]).unwrap_or_else(|e| exc::raise(e))), &[_eval(&[gattr(&node.clone(), "operand")])]);
    }
    match call_builtin("ValueError", &[py_str("not arithmetic")]).unwrap_or_else(|e| exc::raise(e)) {
        Py::Exc(x) => exc::raise((*x).clone()),
        other => exc::raise(exc::PyExc::new("RuntimeError", &py_str_of(&other))),
    }
    Py::None
}
pub fn _expression(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The arithmetic expression in a line, or None. Only digits and operators survive.");
    text = clean.clone();
    text = stdlib_call("re", "sub", &[py_str("\\b(?:what's|whats|what is|calculate|work out|how much is|how much does|please|tell me|the answer to)\\b"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\b(?:how do i|how do you|how can i|how would i|how to|can you|could you|whats the answer to|what is the answer to)\\b"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&text.clone(), "replace", &[py_str(","), py_str("")]);
    text = stdlib_call("re", "sub", &[py_str("\\bmultiplied by\\b|\\btimes\\b|\\bx\\b"), py_str("*"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bdivide\\b\\s+(\\d[\\d.]*)\\s+by\\s+(\\d[\\d.]*)"), py_str("\\1/\\2"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bdivided by\\b|\\bover\\b"), py_str("/"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bplus\\b|\\band\\b"), py_str("+"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bminus\\b|\\btake away\\b|\\bsubtract\\b"), py_str("-"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bsquared\\b"), py_str("**2"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\bcubed\\b"), py_str("**3"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[py_str("\\b(?:to the power of|to the power|raised to the power of|raised to the|raised to)\\b"), py_str("**"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&text.clone(), "replace", &[py_str("^"), py_str("**")]);
    text = mcall(&mcall(&text.clone(), "replace", &[py_str("?"), py_str(" ")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[py_str("[0-9+\\-*/().\\s]+"), { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[+\\-*/]"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(80))) {
    return Py::None;
    }
    return text.clone();
    Py::None
}
pub fn _calc(args: &[Py]) -> Py {
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
    let mut expression: Py = Py::None;
    let mut value: Py = Py::None;
    expression = _expression(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&expression.clone(), &Py::None))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    value = _eval(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(expression.clone()); __kws.extend(vec![("mode".to_string(), py_str("eval"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("ast", "parse", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["SyntaxError", "ValueError", "ZeroDivisionError", "OverflowError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e))), py_func("float", |args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&value.clone(), &value.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("abs", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("float", &[py_str("inf")]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    return { let mut __s = String::new(); __s.push_str("That's "); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut match_: Py = Py::None;
    let mut root: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A square root: \"what's the square root of 144\", \"sqrt 81\".");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("cube root"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("fourth root"))) } }) {
    return Py::None;
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:square root(?: of)?|sqrt|root(?: of)?)\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    value = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&value.clone(), &Py::Int(0))) {
    return py_str("A negative number has no real square root.");
    }
    root = op_pow(&value.clone(), &Py::Float(0.5));
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&root.clone(), &call_builtin("round", &[root.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    return { let mut __s = String::new(); __s.push_str("The square root of "); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[root.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("The square root of "); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_number(&[root.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _roots(args: &[Py]) -> Py {
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
    let mut exact: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut root: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A root asked for in words: \"the square root of 144\", \"cube root of 27\".");
    match_ = stdlib_call("re", "search", &[py_str("(square root|cube root)\\s*(?:of|is)?\\s*(-?[\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    value = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str("cube root")))) {
    root = op_mul(&if py_truthy(&op_lt(&value.clone(), &Py::Int(0))) { op_neg(&Py::Int(1)) } else { Py::Int(1) }, &op_pow(&call_builtin("abs", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_truediv(&Py::Int(1), &Py::Int(3))));
    exact = Py::Bool(py_eq(&op_pow(&call_builtin("round", &[root.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)), &value.clone()));
    name = py_str("cube root");
    } else {
    if py_truthy(&op_lt(&value.clone(), &Py::Int(0))) {
    return Py::None;
    }
    root = op_pow(&value.clone(), &Py::Float(0.5));
    exact = Py::Bool(py_eq(&op_pow(&call_builtin("round", &[root.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &value.clone()));
    name = py_str("square root");
    }
    if py_truthy(&exact.clone()) {
    return { let mut __s = String::new(); __s.push_str("The "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[root.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("The "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[root.clone(), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _prime(args: &[Py]) -> Py {
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
    let mut d: Py = Py::None;
    let mut divisor: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("\"Is 91 a prime number?\": a yes or no answer with the reason, since 91 looks prime.");
    match_ = stdlib_call("re", "search", &[py_str("is\\s+(\\d+)\\s+(?:a\\s+)?prime"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    number = call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&number.clone(), &Py::Int(2))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is not prime: primes start at 2."); py_string(__s) };
    }
    divisor = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(2), op_add(&call_builtin("int", &[op_pow(&number.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut d: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&op_modulo(&number.clone(), &d.clone()), &Py::Int(0)))) {  __out.push(d.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&divisor.clone()) {
    return { let mut __s = String::new(); __s.push_str("No: "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&divisor.clone(), -1, "")); __s.push_str(" × "); __s.push_str(&fstr(&op_floordiv(&number.clone(), &divisor.clone()), -1, "")); __s.push_str(", so it is not prime."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("Yes: nothing but 1 and "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" divides "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(", so it is prime."); py_string(__s) };
    Py::None
}
pub fn _as_fraction(args: &[Py]) -> Py {
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
    let mut bottom: Py = Py::None;
    let mut part: Py = Py::None;
    let mut top: Py = Py::None;
    let mut value: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("'3/4', '1 1/2' or '1.5' as a Fraction, or None.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    let __unpk1 = py_unpack(&mcall(&text.clone(), "rpartition", &[py_str(" ")]), 3, None);
    whole = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    part = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&part.clone(), &py_str("/")))) {
    let __unpk2 = py_unpack(&mcall(&part.clone(), "partition", &[py_str("/")]), 3, None);
    top = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    bottom = __unpk2[2].clone();
    let __try3 = exc::catch(|| {
    value = stdlib_call("fractions", "Fraction", &[call_builtin("int", &[top.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[bottom.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["ValueError", "ZeroDivisionError"]) {
                __handled7 = true;
    return Py::None;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&mcall(&whole.clone(), "strip", &[])) {
    let __try8 = exc::catch(|| {
    let __aug13 = op_add(&value.clone(), &call_builtin("int", &[whole.clone()]).unwrap_or_else(|e| exc::raise(e)));
    value = __aug13;
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["ValueError"]) {
                __handled12 = true;
    return Py::None;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    }
    return value.clone();
    }
    let __try14 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), stdlib_call("fractions", "Fraction", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret15: Py = Py::Bool(false);
    let mut __val16: Py = Py::None;
    let mut __unhandled17: Option<exc::PyExc> = None;
    match __try14 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret15 = __p[0].clone(); __val16 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled18 = false;
            if !__handled18 && exc::matches_any(&__e, &["ValueError", "ZeroDivisionError"]) {
                __handled18 = true;
    return Py::None;
            }
            if !__handled18 { __unhandled17 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled17 { exc::raise(__e); }
    if py_truthy(&__ret15) { return __val16; }
    Py::None
}
pub fn _fraction_text(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A Fraction the way a recipe reads it: 3/8, 1 1/2, 2.");
    value = mcall(&stdlib_call("fractions", "Fraction", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "limit_denominator", &[Py::Int(64)]);
    if py_truthy(&Py::Bool(py_eq(&gattr(&value.clone(), "denominator"), &Py::Int(1)))) {
    return call_builtin("str", &[gattr(&value.clone(), "numerator")]).unwrap_or_else(|e| exc::raise(e));
    }
    let __unpk1 = py_unpack(&call_builtin("divmod", &[gattr(&value.clone(), "numerator"), gattr(&value.clone(), "denominator")]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    whole = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    if py_truthy(&whole.clone()) {
    return op_modulo(&py_str("%d %d/%d"), &py_tuple(vec![whole.clone(), rest.clone(), gattr(&value.clone(), "denominator")]));
    }
    return op_modulo(&py_str("%d/%d"), &py_tuple(vec![rest.clone(), gattr(&value.clone(), "denominator")]));
    Py::None
}
pub fn _fraction_of_a_fraction(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut amount_text: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut count: Py = Py::None;
    let mut count_word: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut millilitres: Py = Py::None;
    let mut part: Py = Py::None;
    let mut part_word: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_amount: Py = Py::None;
    let mut spoon_text: Py = Py::None;
    let mut spoons: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A part of a fraction, or of a mixed number: \"half of 3/4 cup\", \"2/3 of 3/8\".");
    match_ = mcall(&g__FRACTION_OF_FRACTION(), "search", &[clean.clone()]);
    bare = Py::Bool(!py_truthy(&match_.clone()));
    if py_truthy(&bare.clone()) {
    match_ = mcall(&g__FRACTION_OF_A_FRACTION(), "search", &[clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])]), 2, None);
    count_word = __unpk1[0].clone();
    part_word = __unpk1[1].clone();
    part = if py_truthy(&Py::Bool(py_contains(&g__FRACTIONS(), &part_word.clone()))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&geti(&g__FRACTIONS(), &part_word.clone()))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("fractions", "Fraction", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { _as_fraction(&[part_word.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return Py::None;
    }
    if py_truthy(&count_word.clone()) {
    count = if py_truthy(&mcall(&count_word.clone(), "isdigit", &[])) { call_builtin("int", &[count_word.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__COUNTS(), "get", &[count_word.clone(), Py::Int(1)]) };
    part = op_mul(&part.clone(), &count.clone());
    }
    amount_text = if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) { mcall(&match_.clone(), "group", &[Py::Int(3)]) } else { geti(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[py_str(" of ")]), &op_neg(&Py::Int(1))) };
    amount = _as_fraction(&[amount_text.clone()]);
    if py_truthy(&Py::Bool(py_is(&amount.clone(), &Py::None))) {
    return Py::None;
    }
    value = op_mul(&part.clone(), &amount.clone());
    unit = if py_truthy(&bare.clone()) { py_str("") } else { mcall(&match_.clone(), "group", &[Py::Int(4)]) };
    if py_truthy(&op_le(&value.clone(), &Py::Int(1))) {
    unit = mcall(&g__SINGULAR(), "get", &[unit.clone(), unit.clone()]);
    }
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![count_word.clone(), part_word.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    said_amount = op_add(&_fraction_text(&[amount.clone()]), &if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) { op_add(&py_str(" "), &mcall(&match_.clone(), "group", &[Py::Int(4)])) } else { py_str("") });
    answer = op_modulo(&py_str("%s of %s is %s%s"), &py_tuple(vec![said.clone(), said_amount.clone(), _fraction_text(&[value.clone()]), if py_truthy(&unit.clone()) { op_add(&py_str(" "), &unit.clone()) } else { py_str("") }]));
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("cup")])) {
    spoons = op_mul(&value.clone(), &g__CUP_IN_TBSP());
    millilitres = op_mul(&value.clone(), &g__CUP_IN_ML());
    spoon_text = if py_truthy(&Py::Bool(py_eq(&gattr(&spoons.clone(), "denominator"), &Py::Int(1)))) { _fraction_text(&[spoons.clone()]) } else { op_modulo(&py_str("about %d"), &call_builtin("round", &[call_builtin("float", &[spoons.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) };
    let __aug2 = op_add(&answer.clone(), &op_modulo(&py_str(", which is %s tablespoons (%d ml)"), &py_tuple(vec![spoon_text.clone(), call_builtin("round", &[millilitres.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    answer = __aug2;
    }
    if py_truthy(&{ let __b = bare.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&amount_text.clone(), &py_str("/"))) } }) {
    if py_truthy(&Py::Bool(py_eq(&part.clone(), &stdlib_call("fractions", "Fraction", &[Py::Int(1), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))))) {
    let __aug3 = op_add(&answer.clone(), &py_str(": halve the top number and keep the bottom one."));
    answer = __aug3;
    } else {
    let __aug4 = op_add(&answer.clone(), &py_str(": multiply the tops and the bottoms."));
    answer = __aug4;
    }
    } else {
    let __aug5 = op_add(&answer.clone(), &py_str("."));
    answer = __aug5;
    }
    return answer.clone();
    Py::None
}
pub fn _fraction_of(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut bottom: Py = Py::None;
    let mut count: Py = Py::None;
    let mut kitchen: Py = Py::None;
    let mut label: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut top: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A simple fraction of an amount: \"half of 96\", \"three quarters of 200\", \"2/5 of 60\".");
    kitchen = _fraction_of_a_fraction(&[clean.clone()]);
    if py_truthy(&kitchen.clone()) {
    return kitchen.clone();
    }
    match_ = stdlib_call("re", "search", &[py_str("\\b(\\d+)\\s*/\\s*(\\d+)\\s+of\\s+([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top = __unpk1[0].clone();
    bottom = __unpk1[1].clone();
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    label = { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[top.clone()]), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&_number(&[bottom.clone()]), -1, "")); py_string(__s) };
    } else {
    match_ = stdlib_call("re", "search", &[py_str("\\b(a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+)?\\s*(half|halves|third|thirds|quarter|quarters|fifth|fifths|sixth|sixths|eighth|eighths|tenth|tenths)\\s+of\\s+([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    said = { let __b = mcall(&match_.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("a") } };
    count = if py_truthy(&mcall(&said.clone(), "isdigit", &[])) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__COUNTS(), "get", &[said.clone(), Py::Int(1)]) };
    if py_truthy(&mcall(&said.clone(), "isdigit", &[])) {
    said = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__COUNTS(), "items", &[])) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk1[0].clone(); let mut number: Py = __unpk1[1].clone(); if py_truthy(&Py::Bool(py_eq(&number.clone(), &count.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    let __unpk2 = py_unpack(&geti(&g__FRACTIONS(), &mcall(&match_.clone(), "group", &[Py::Int(2)])), 2, None);
    part = __unpk2[0].clone();
    whole = __unpk2[1].clone();
    let __unpk3 = py_unpack(&py_tuple(vec![op_mul(&part.clone(), &count.clone()), whole.clone()]), 2, None);
    top = __unpk3[0].clone();
    bottom = __unpk3[1].clone();
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    label = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&match_.clone(), "group", &[Py::Int(1), Py::Int(2)])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = mcall(&said.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("") } }, "isdigit", &[]) } }) {
    label = { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(2)]), -1, "")); py_string(__s) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bottom.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&top.clone())) } }) {
    return Py::None;
    }
    value = op_truediv(&op_mul(&amount.clone(), &top.clone()), &bottom.clone());
    unit = stdlib_call("re", "match", &[py_str("\\s*(?:g|grams?|kg|kilograms?|ml|litres?|liters?|pounds?|oz|ounces?|minutes?|hours?|miles?|km|metres?|meters?)\\b"), slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&unit.clone()) { { let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&mcall(&mcall(&unit.clone(), "group", &[Py::Int(0)]), "strip", &[]), -1, "")); py_string(__s) } } else { py_str("") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&op_add(&mcall(&geti(&label.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&label.clone(), &Py::Int(1), &Py::None, &Py::None)), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[value.clone(), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _share_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut total: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "total") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: total"))) };
    let mut people: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "people") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: people"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut money: Py = Py::None;
    let mut share: Py = Py::None;
    let mut sign: Py = Py::None;
    if py_truthy(&op_lt(&people.clone(), &Py::Int(2))) {
    return Py::None;
    }
    money = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("£"), py_str("$"), py_str("€")])) { let mut sign: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&clean.clone(), &sign.clone()))) {  __out.push(sign.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    share = op_truediv(&total.clone(), &people.clone());
    return { let mut __s = String::new(); __s.push_str("That's "); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_number(&[call_builtin("round", &[share.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" each: "); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" split "); __s.push_str(&fstr(&people.clone(), -1, "")); __s.push_str(" ways."); py_string(__s) };
    Py::None
}
pub fn _count_value(args: &[Py]) -> Py {
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
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    return if py_truthy(&mcall(&text.clone(), "isdigit", &[])) { call_builtin("int", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__COUNT_VALUE(), "get", &[text.clone()]) };
    Py::None
}
pub fn _share(args: &[Py]) -> Py {
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
    let mut between: Py = Py::None;
    let mut left: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut money_word: Py = Py::None;
    let mut n: Py = Py::None;
    let mut people: Py = Py::None;
    let mut sign_sum: Py = Py::None;
    let mut total: Py = Py::None;
    let mut total_text: Py = Py::None;
    let mut ways: Py = Py::None;
    let _ = py_str("\"Split 84 pounds between 6 people\": a sum divided among a number of people.\n\n    The ways round people say it differ only in where the count is (\"between 6 people\",\n    \"4 ways\", \"5 of us\"), so the sum and the count are found separately rather than with\n    one pattern per phrasing.\n    ");
    match_ = stdlib_call("re", "search", &[op_add(&op_add(&py_str("[£$€]?\\s*([\\d,.]+)\\s*(?:pounds?|quid|euros?|dollars?)?\\s*(?:split|shared|divided|share)\\s+(?:it\\s+)?(?:equally\\s+)?(?:between|among|across)\\s+(?:the\\s+)?("), &g__A_COUNT()), &py_str(")\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[py_str("(?:split|share|divide)\\s+(?:the bill of\\s+|the\\s+|it\\s+)?[£$€]?\\s*([\\d,.]+)\\s*(?:pounds?|quid|euros?|dollars?)?\\s*(?:equally\\s+)?(?:between|among|across)\\s+(\\d+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    between = stdlib_call("re", "search", &[op_add(&op_add(&py_str("[£$€]?\\s*([\\d,.]+)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)?\\s+between\\s+(?:the\\s+)?("), &g__A_COUNT()), &py_str(")\\s+(?:of\\s+us|of\\s+them|people|friends|adults|ways)\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&between.clone()) {
    total = call_builtin("float", &[mcall(&mcall(&between.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    people = _count_value(&[mcall(&between.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&{ let __b = people.clone(); if !py_truthy(&__b) { __b } else { op_ge(&people.clone(), &Py::Int(2)) } }) {
    return _share_reply(&[total.clone(), people.clone(), clean.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:split|splitting|share|sharing|divide|dividing)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    ways = stdlib_call("re", "search", &[op_add(&op_add(&py_str("("), &g__A_COUNT()), &py_str(")\\s*(?:ways?|people|of us|friends|adults)")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&ways.clone()))) {
    return Py::None;
    }
    money_word = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&money_word.clone()) {
    total_text = mcall(&money_word.clone(), "group", &[Py::Int(1)]);
    } else {
    sign_sum = stdlib_call("re", "search", &[py_str("[£$€]\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&sign_sum.clone()) {
    total_text = mcall(&sign_sum.clone(), "group", &[Py::Int(1)]);
    } else {
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("\\d+(?:\\.\\d+)?"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut n: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&n.clone(), &mcall(&ways.clone(), "group", &[Py::Int(1)])))) {  __out.push(n.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&left.clone()))) {
    return Py::None;
    }
    total_text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); __kws.extend(vec![("key".to_string(), py_func("float", |args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    match_ = Py::None;
    total = call_builtin("float", &[mcall(&total_text.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    people = _count_value(&[mcall(&ways.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(!py_truthy(&people.clone()))) {
    return Py::None;
    }
    return _share_reply(&[total.clone(), people.clone(), clean.clone()]);
    }
    total = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    people = _count_value(&[mcall(&match_.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&people.clone())); if py_truthy(&__b) { __b } else { op_lt(&people.clone(), &Py::Int(2)) } }) {
    return Py::None;
    }
    return _share_reply(&[total.clone(), people.clone(), clean.clone()]);
    Py::None
}
pub fn _dec(args: &[Py]) -> Py {
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
    let mut places: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "places") { v.clone() }
    else { Py::Int(6) };
    let mut text: Py = Py::None;
    let _ = py_str("A decimal the way a person writes it: 0.125, 0.75, 12.");
    text = mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".{}f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    return { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("0") } };
    Py::None
}
pub fn _fraction_decimal(args: &[Py]) -> Py {
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
    let mut bottom: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut part: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut top: Py = Py::None;
    let mut value: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("Both directions: \"3/4 as a decimal\" and \"0.25 as a fraction\".\n\n    The number in the answer is the one the person wrote, so \"0.375 as a fraction\" says\n    0.375 and not a rounded 0.38, and \"1/8 as a decimal\" says 0.125 rather than 0.12.\n\n    A share said as \"1 in 10\" is one part in ten, so it is 10% and not 1000%: the number before\n    \"in\" is the top of the fraction and the one after it is the bottom. Without this the line\n    was read as \"10 as a percentage\" and the answer was wrong by a factor of a hundred. A\n    percentage asked as a fraction or a decimal is the same sum the other way round, so\n    \"25% as a fraction\" is 1/4 and \"25% as a decimal\" is 0.25 rather than a lecture on fractions.\n    ");
    match_ = stdlib_call("re", "search", &[py_str("(\\d+)\\s*in\\s*(\\d+)\\s*(?:as|to|into)\\s*(?:a\\s+)?(fraction|decimal|percentage|percent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)), mcall(&match_.clone(), "group", &[Py::Int(3)])]), 3, None);
    top = __unpk1[0].clone();
    bottom = __unpk1[1].clone();
    kind = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&bottom.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&kind.clone(), "startswith", &[py_str("percent")])) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_dec(&[op_mul(&op_truediv(&top.clone(), &bottom.clone()), &Py::Int(100)), Py::Int(2)]), -1, "")); __s.push_str("%."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("decimal")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" as a decimal is "); __s.push_str(&fstr(&_exact_decimal(&[top.clone(), bottom.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    part = mcall(&stdlib_call("fractions", "Fraction", &[top.clone(), bottom.clone()]).unwrap_or_else(|e| exc::raise(e)), "limit_denominator", &[Py::Int(1000)]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" as a fraction is "); __s.push_str(&fstr(&gattr(&part.clone(), "numerator"), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&gattr(&part.clone(), "denominator"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(%|percent|per cent)\\s*(?:as|to|into)\\s*(?:a\\s+)?(fraction|decimal)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    written = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "rstrip", &[py_str(".")]);
    sign = if py_truthy(&Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(2)]), &py_str("%")))) { py_str("%") } else { py_str(" percent") };
    value = call_builtin("float", &[mcall(&written.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(3)]), &py_str("decimal")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(" as a decimal is "); __s.push_str(&fstr(&_dec(&[op_truediv(&value.clone(), &Py::Int(100)), Py::Int(6)]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    part = mcall(&stdlib_call("fractions", "Fraction", &[op_truediv(&value.clone(), &Py::Int(100))]).unwrap_or_else(|e| exc::raise(e)), "limit_denominator", &[Py::Int(1000)]);
    if py_truthy(&Py::Bool(py_eq(&gattr(&part.clone(), "denominator"), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(" as a fraction is "); __s.push_str(&fstr(&gattr(&part.clone(), "numerator"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(" as a fraction is "); __s.push_str(&fstr(&gattr(&part.clone(), "numerator"), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&gattr(&part.clone(), "denominator"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(\\d+)\\s*/\\s*(\\d+)\\s*(?:as|in|to)\\s*(?:a\\s+)?(decimal|percentage|percent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top = __unpk2[0].clone();
    bottom = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&bottom.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "startswith", &[py_str("percent")])) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_dec(&[op_mul(&op_truediv(&top.clone(), &bottom.clone()), &Py::Int(100)), Py::Int(2)]), -1, "")); __s.push_str("%."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" as a decimal is "); __s.push_str(&fstr(&_exact_decimal(&[top.clone(), bottom.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:as|in|to)\\s*(?:a\\s+)?(fraction|percentage|percent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s+of\\s+[\\d,.]+"), slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    written = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "rstrip", &[py_str(".")]);
    value = call_builtin("float", &[mcall(&written.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(2)]), &py_str("fraction")))) {
    part = mcall(&stdlib_call("fractions", "Fraction", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "limit_denominator", &[Py::Int(1000)]);
    if py_truthy(&Py::Bool(py_eq(&gattr(&part.clone(), "denominator"), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is a whole number: "); __s.push_str(&fstr(&gattr(&part.clone(), "numerator"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&gattr(&part.clone(), "numerator"), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&gattr(&part.clone(), "denominator"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" as a percentage is "); __s.push_str(&fstr(&_dec(&[op_mul(&value.clone(), &Py::Int(100)), Py::Int(2)]), -1, "")); __s.push_str("%."); py_string(__s) };
    Py::None
}
pub fn _exact_decimal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut top: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "top") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: top"))) };
    let mut bottom: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bottom") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bottom"))) };
    let mut left: Py = Py::None;
    let mut prime: Py = Py::None;
    let _ = py_str("A fraction as a decimal: 3/8 is 0.375, not 0.38, and 1/3 is 0.33 recurring.\n\n    A decimal that stops is written out in full, because \"as a decimal\" asks for the number\n    itself; one that never stops is rounded to two places and marked as recurring, so it does\n    not read as exact.\n    ");
    left = bottom.clone();
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(2), Py::Int(5)]));
    for __item in __iter1 {
    prime = __item.clone();
    while py_truthy(&Py::Bool(py_eq(&op_modulo(&left.clone(), &prime.clone()), &Py::Int(0)))) {
    let __aug2 = op_floordiv(&left.clone(), &prime.clone());
    left = __aug2;
    }
    }
    if py_truthy(&Py::Bool(py_eq(&left.clone(), &Py::Int(1)))) {
    return mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&top.clone(), &bottom.clone()), -1, &".6f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[op_truediv(&top.clone(), &bottom.clone())]), -1, "")); __s.push_str(" recurring"); py_string(__s) };
    Py::None
}
pub fn _fraction(args: &[Py]) -> Py {
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
    let mut bottom: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut of: Py = Py::None;
    let mut top: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A fraction as a decimal or percentage, or a fraction of a number.");
    match_ = stdlib_call("re", "search", &[py_str("(-?\\d+)\\s*/\\s*(\\d+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[py_str("(-?\\d+)\\s+(?:over|upon)\\s+(\\d+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top = __unpk1[0].clone();
    bottom = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&bottom.clone(), &Py::Int(0)))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("decimal")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" as a decimal is "); __s.push_str(&fstr(&_exact_decimal(&[top.clone(), bottom.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("percent")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[op_mul(&op_truediv(&top.clone(), &bottom.clone()), &Py::Int(100))]), -1, "")); __s.push_str("%."); py_string(__s) };
    }
    of = stdlib_call("re", "search", &[py_str("/\\s*\\d+\\s*of\\s*([\\d,.]+)(?!\\s*/\\s*\\d)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&of.clone()) {
    total = call_builtin("float", &[mcall(&mcall(&of.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[op_mul(&op_truediv(&top.clone(), &bottom.clone()), &total.clone())]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _reduced(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut top: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "top") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: top"))) };
    let mut bottom: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bottom") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bottom"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let _ = py_str("The same fraction with the common factors divided out, and what was divided out.");
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("abs", &[top.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("abs", &[bottom.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    a = __unpk1[0].clone();
    b = __unpk1[1].clone();
    while py_truthy(&b.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![b.clone(), op_modulo(&a.clone(), &b.clone())]), 2, None);
    a = __unpk2[0].clone();
    b = __unpk2[1].clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&a.clone()))) {
    return py_tuple(vec![top.clone(), bottom.clone(), Py::Int(1)]);
    }
    return py_tuple(vec![op_floordiv(&top.clone(), &a.clone()), op_floordiv(&bottom.clone(), &a.clone()), a.clone()]);
    Py::None
}
pub fn _simplify(args: &[Py]) -> Py {
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
    let mut bottom: Py = Py::None;
    let mut by: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut simple_bottom: Py = Py::None;
    let mut simple_top: Py = Py::None;
    let mut top: Py = Py::None;
    let _ = py_str("\"simplify 6 over 8\": the same fraction with no common factor left.");
    match_ = stdlib_call("re", "search", &[op_add(&py_str("simplif(?:y|ies|ied)\\s+(?:the\\s+)?(?:fraction\\s+)?"), &g__FRACTION()), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[op_add(&g__FRACTION(), &py_str("\\s*(?:in|to)\\s+(?:its\\s+)?(?:simplest form|lowest terms)")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top = __unpk1[0].clone();
    bottom = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bottom.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&top.clone())) } }) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_reduced(&[top.clone(), bottom.clone()]), 3, None);
    simple_top = __unpk2[0].clone();
    simple_bottom = __unpk2[1].clone();
    by = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&by.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" is already as simple as it goes: nothing divides both numbers."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" simplifies to "); __s.push_str(&fstr(&simple_top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&simple_bottom.clone(), -1, "")); __s.push_str(": divide the top and the bottom by "); __s.push_str(&fstr(&by.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _add_fractions(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut bottom: Py = Py::None;
    let mut bottom_a: Py = Py::None;
    let mut bottom_b: Py = Py::None;
    let mut left: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut other: Py = Py::None;
    let mut right: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut simple_bottom: Py = Py::None;
    let mut simple_top: Py = Py::None;
    let mut top: Py = Py::None;
    let mut top_a: Py = Py::None;
    let mut top_b: Py = Py::None;
    let mut way: Py = Py::None;
    let _ = py_str("\"add 1/2 and 1/3\": a common bottom, then the tops.");
    match_ = stdlib_call("re", "search", &[op_add(&op_add(&g__FRACTION(), &py_str("\\s*(\\+|plus|and|minus|-)\\s*")), &g__FRACTION()), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    sentence = op_add(&op_add(&op_add(&py_str("(?:add|subtract|take away)\\s+(?:the fractions\\s+)?"), &g__FRACTION()), &py_str("\\s+(?:and|from|to)\\s+")), &g__FRACTION());
    other = stdlib_call("re", "search", &[sentence.clone(), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top_a = __unpk1[0].clone();
    bottom_a = __unpk1[1].clone();
    sign = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("-"), py_str("minus")]), &mcall(&match_.clone(), "group", &[Py::Int(3)])))) { py_str("-") } else { py_str("+") };
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(4)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(5)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top_b = __unpk2[0].clone();
    bottom_b = __unpk2[1].clone();
    } else {
    if py_truthy(&other.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&other.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&other.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top_a = __unpk3[0].clone();
    bottom_a = __unpk3[1].clone();
    let __unpk4 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&other.clone(), "group", &[Py::Int(3)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&other.clone(), "group", &[Py::Int(4)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top_b = __unpk4[0].clone();
    bottom_b = __unpk4[1].clone();
    sign = if py_truthy(&{ let __b = mcall(&clean.clone(), "startswith", &[py_str("subtract")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("take away"))) } }) { py_str("-") } else { py_str("+") };
    } else {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = bottom_a.clone(); if !py_truthy(&__b) { __b } else { bottom_b.clone() } }))) {
    return Py::None;
    }
    let __unpk5 = py_unpack(&_reduced(&[bottom_a.clone(), bottom_b.clone()]), 3, None);
    _u = __unpk5[0].clone();
    _u = __unpk5[1].clone();
    shared = __unpk5[2].clone();
    bottom = op_floordiv(&op_mul(&bottom_a.clone(), &bottom_b.clone()), &{ let __b = shared.clone(); if py_truthy(&__b) { __b } else { Py::Int(1) } });
    let __unpk6 = py_unpack(&py_tuple(vec![op_floordiv(&bottom.clone(), &bottom_a.clone()), op_floordiv(&bottom.clone(), &bottom_b.clone())]), 2, None);
    left = __unpk6[0].clone();
    right = __unpk6[1].clone();
    top = op_add(&op_mul(&top_a.clone(), &left.clone()), &if py_truthy(&Py::Bool(py_eq(&sign.clone(), &py_str("+")))) { op_mul(&top_b.clone(), &right.clone()) } else { op_mul(&op_neg(&top_b.clone()), &right.clone()) });
    let __unpk7 = py_unpack(&_reduced(&[top.clone(), bottom.clone()]), 3, None);
    simple_top = __unpk7[0].clone();
    simple_bottom = __unpk7[1].clone();
    _u = __unpk7[2].clone();
    way = if py_truthy(&Py::Bool(py_eq(&sign.clone(), &py_str("+")))) { py_str("add the tops") } else { py_str("subtract the tops") };
    if py_truthy(&Py::Bool(py_eq(&bottom_a.clone(), &bottom_b.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&top_a.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom_a.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&top_b.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom_b.clone(), -1, "")); __s.push_str(" = "); __s.push_str(&fstr(&simple_top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&simple_bottom.clone(), -1, "")); __s.push_str(": the bottoms are the same, so just "); __s.push_str(&fstr(&way.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&top_a.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom_a.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&top_b.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom_b.clone(), -1, "")); __s.push_str(" = "); __s.push_str(&fstr(&simple_top.clone(), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&simple_bottom.clone(), -1, "")); __s.push_str(". Put both over "); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" ("); __s.push_str(&fstr(&op_mul(&top_a.clone(), &left.clone()), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&op_mul(&top_b.clone(), &right.clone()), -1, "")); __s.push_str("/"); __s.push_str(&fstr(&bottom.clone(), -1, "")); __s.push_str("), "); __s.push_str(&fstr(&way.clone(), -1, "")); __s.push_str(", then cancel down."); py_string(__s) };
    Py::None
}
pub fn _factors_in_list(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The whole-number factors of one number, smallest first.");
    out = py_list(vec![]);
    step = Py::Int(1);
    while py_truthy(&op_le(&op_mul(&step.clone(), &step.clone()), &words.clone())) {
    if py_truthy(&Py::Bool(py_eq(&op_modulo(&words.clone(), &step.clone()), &Py::Int(0)))) {
    let _ = mcall(&out.clone(), "append", &[step.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&step.clone(), &op_floordiv(&words.clone(), &step.clone())))) {
    let _ = mcall(&out.clone(), "append", &[op_floordiv(&words.clone(), &step.clone())]);
    }
    }
    let __aug1 = op_add(&step.clone(), &Py::Int(1));
    step = __aug1;
    }
    return call_builtin("sorted", &[out.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut numbers: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "numbers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: numbers"))) };
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("1, 2, 3 and 6 - the way a list of numbers is written out loud.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&numbers.clone()) { let mut number: Py = __item.clone(); __out.push(call_builtin("str", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&op_le(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return mcall(&py_str(""), "join", &[said.clone()]);
    }
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn _the_factors(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut base: Py = Py::None;
    let mut factors: Py = Py::None;
    let mut found: Py = Py::None;
    let mut left: Py = Py::None;
    let mut number: Py = Py::None;
    let mut pairs: Py = Py::None;
    let mut said: Py = Py::None;
    let mut small: Py = Py::None;
    let mut times: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("\"the factors of 12\", \"what are the factors of 36\", \"list the factors of 24\".\n\n    A factor is a number that divides in with nothing left over, and people ask for the list far\n    more often than they ask what the word means: \"the factors of 12\" reached nothing at all and\n    \"what are the factors of 36\" was answered with the definition of a factor. The definition is\n    kept as the second sentence, where it belongs.\n    ");
    if py_truthy(&stdlib_call("re", "search", &[py_str("common factor|common multiple|prime factor|factorise|factorize|factor tree|factor the|factor out|scale factor|conversion factor|factor of safety"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    found = stdlib_call("re", "search", &[py_str("\\bis\\s+(\\d+)\\s*(?:,|\\bor\\b|\\band\\b|/)?\\s*(?:and\\s+)?(\\d+)?\\s+a\\s+factor\\s+of\\s+(\\d+)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    asked = py_list(vec![call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&mcall(&found.clone(), "group", &[Py::Int(2)])) {
    let _ = mcall(&asked.clone(), "append", &[call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    whole = call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(3)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_lt(&whole.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("max", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1000000)) } }) {
    return Py::None;
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&asked.clone());
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&op_gt(&number.clone(), &whole.clone())) {
    let _ = mcall(&said.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is bigger than "); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(", so no: it cannot divide into it even once."); py_string(__s) }]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&op_modulo(&whole.clone(), &number.clone()), &Py::Int(0)))) {
    let _ = mcall(&said.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is a factor of "); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" divided by "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&op_floordiv(&whole.clone(), &number.clone()), -1, "")); __s.push_str("."); py_string(__s) }]);
    } else {
    times = op_floordiv(&whole.clone(), &number.clone());
    left = op_sub(&whole.clone(), &op_mul(&times.clone(), &number.clone()));
    let _ = mcall(&said.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is not a factor of "); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" goes in "); __s.push_str(&fstr(&times.clone(), -1, "")); __s.push_str(" time"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(!py_eq(&times.clone(), &Py::Int(1)))) { py_str("s") } else { py_str("") }, -1, "")); __s.push_str(" with "); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(" left over."); py_string(__s) }]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&said.clone(), &Py::Int(0));
    }
    return mcall(&py_str(" "), "join", &[said.clone()]);
    }
    found = stdlib_call("re", "search", &[py_str("\\bis\\s+(\\d+)\\s+a\\s+multiple\\s+of\\s+(\\d+)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    number = __unpk2[0].clone();
    base = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&base.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("max", &[number.clone(), base.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1000000)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&op_modulo(&number.clone(), &base.clone()), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str("Yes: "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&base.clone(), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&op_floordiv(&number.clone(), &base.clone()), -1, "")); __s.push_str(", so it is in the "); __s.push_str(&fstr(&base.clone(), -1, "")); __s.push_str(" times table."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("No: "); __s.push_str(&fstr(&base.clone(), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&op_floordiv(&number.clone(), &base.clone()), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&op_mul(&base.clone(), &op_floordiv(&number.clone(), &base.clone())), -1, "")); __s.push_str(", and "); __s.push_str(&fstr(&base.clone(), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&op_add(&op_floordiv(&number.clone(), &base.clone()), &Py::Int(1)), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&op_mul(&base.clone(), &op_add(&op_floordiv(&number.clone(), &base.clone()), &Py::Int(1))), -1, "")); __s.push_str(", so "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" is not in the "); __s.push_str(&fstr(&base.clone(), -1, "")); __s.push_str(" times table."); py_string(__s) };
    }
    found = stdlib_call("re", "search", &[py_str("\\bfactors? of\\s+(\\d+)\\b(?!\\s*(?:and|&)\\b)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    number = call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_lt(&number.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&number.clone(), &Py::Int(100000)) } }) {
    return Py::None;
    }
    factors = _factors_in_list(&[number.clone()]);
    pairs = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&factors.clone());
    for __item in __iter3 {
    small = __item.clone();
    if py_truthy(&op_gt(&op_mul(&small.clone(), &small.clone()), &number.clone())) {
    break;
    }
    let _ = mcall(&pairs.clone(), "append", &[{ let mut __s = String::new(); __s.push_str(&fstr(&small.clone(), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&op_floordiv(&number.clone(), &small.clone()), -1, "")); py_string(__s) }]);
    }
    said = { let mut __s = String::new(); __s.push_str("The factors of "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" are "); __s.push_str(&fstr(&_said_list(&[factors.clone()]), -1, "")); __s.push_str(": those are the numbers that divide into "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" with nothing left over, counting 1 and "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" itself."); py_string(__s) };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[factors.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" That makes "); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(" a prime number."); py_string(__s) });
    }
    let __aug4 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" You only have to try up to "); __s.push_str(&fstr(&call_builtin("int", &[op_pow(&number.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" - the square root - because "); py_string(__s) });
    said = __aug4;
    let __aug5 = op_add(&said.clone(), &op_add(&op_add(&py_str("the pairs do the rest: "), &mcall(&py_str(", "), "join", &[pairs.clone()])), &py_str(".")));
    said = __aug5;
    return said.clone();
    }
    return Py::None;
    Py::None
}
pub fn _common_factors(args: &[Py]) -> Py {
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
    let mut a: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut b: Py = Py::None;
    let mut first: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("\"highest common factor of 12 and 18\", \"lowest common multiple of 4 and 6\".");
    asked = Py::None;
    if py_truthy(&stdlib_call("re", "search", &[py_str("highest common factor|greatest common (?:factor|divisor)|\\bhcf\\b|\\bgcd\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    asked = py_str("highest common factor");
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("lowest common multiple|least common multiple|\\blcm\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    asked = py_str("lowest common multiple");
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    numbers = stdlib_call("re", "findall", &[py_str("\\b(\\d+)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[geti(&numbers.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[geti(&numbers.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&second.clone())) } }) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![first.clone(), second.clone()]), 2, None);
    a = __unpk2[0].clone();
    b = __unpk2[1].clone();
    while py_truthy(&b.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![b.clone(), op_modulo(&a.clone(), &b.clone())]), 2, None);
    a = __unpk3[0].clone();
    b = __unpk3[1].clone();
    }
    if py_truthy(&Py::Bool(py_eq(&asked.clone(), &py_str("highest common factor")))) {
    return { let mut __s = String::new(); __s.push_str("The highest common factor of "); __s.push_str(&fstr(&first.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&second.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&a.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("The lowest common multiple of "); __s.push_str(&fstr(&first.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&second.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&op_mul(&op_floordiv(&first.clone(), &a.clone()), &second.clone()), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _round_to(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut below: Py = Py::None;
    let mut digits: Py = Py::None;
    let mut floor: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut log10: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut near: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut rounded: Py = Py::None;
    let mut stepped: Py = Py::None;
    let mut value: Py = Py::None;
    let mut what: Py = Py::None;
    let mut why: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("\"round 3.456 to two decimal places\", \"round 2.5 to the nearest whole number\".");
    match_ = stdlib_call("re", "search", &[py_str("round\\s+(-?\\d+(?:\\.\\d+)?)\\s*(?:to|at)\\s*(?:the\\s+)?(\\w+)\\s*(decimal places?|places?|dp|significant figures?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    near = stdlib_call("re", "search", &[py_str("round\\s+(-?\\d+(?:\\.\\d+)?)\\s*(?:up|down|off)?\\s*(?:to|at)\\s*(?:the\\s+)?nearest\\s+(\\w+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&near.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NEAREST(), &mcall(&near.clone(), "group", &[Py::Int(2)]))) } }) {
    bare = stdlib_call("re", "search", &[py_str("\\bround\\s+(-?\\d+\\.\\d+)\\s*$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    written = mcall(&bare.clone(), "group", &[Py::Int(1)]);
    kept = _places(&[written.clone(), Py::Int(0)]);
    value = stdlib_call("decimal", "Decimal", &[call_builtin("str", &[written.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    why = if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_call("decimal", "Decimal", &[Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("rounding".to_string(), stdlib_attr("decimal", "ROUND_HALF_UP").unwrap_or(Py::None))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "quantize", args))(&__args) }, &value.clone())) { py_str("up") } else { py_str("down") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&kept.clone(), -1, "")); __s.push_str(" to the nearest whole number: the digit after the point decides it, so the whole number goes "); __s.push_str(&fstr(&why.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&near.clone(), "group", &[Py::Int(1)]), mcall(&near.clone(), "group", &[Py::Int(2)])]), 2, None);
    written = __unpk1[0].clone();
    word = __unpk1[1].clone();
    below = geti(&g__NEAREST(), &word.clone());
    if py_truthy(&below.clone()) {
    stepped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&stdlib_call("decimal", "Decimal", &[Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), "scaleb", &[op_neg(&below.clone())])); __kws.extend(vec![("rounding".to_string(), stdlib_attr("decimal", "ROUND_HALF_UP").unwrap_or(Py::None))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&stdlib_call("decimal", "Decimal", &[call_builtin("str", &[written.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "quantize", args))(&__args) };
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&stepped.clone(), -1, &"f")); __s.push_str(" to the nearest "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    what = if py_truthy(&Py::Bool(py_eq(&geti(&g__NEAREST(), &word.clone()), &Py::Int(0)))) { py_str("whole number") } else { word.clone() };
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_places(&[written.clone(), Py::Int(0)]), -1, "")); __s.push_str(" to the nearest "); __s.push_str(&fstr(&what.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    written = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    value = call_builtin("float", &[written.clone()]).unwrap_or_else(|e| exc::raise(e));
    places = mcall(&g__PLACES(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&Py::Bool(py_is(&places.clone(), &Py::None))) {
    return Py::None;
    }
    plural = if py_truthy(&Py::Bool(!py_eq(&places.clone(), &Py::Int(1)))) { py_str("s") } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&mcall(&match_.clone(), "group", &[Py::Int(3)]), &py_str("figure")))) {
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return Py::None;
    }
    floor = stdlib_attr("math", "floor").unwrap_or(Py::None);
    log10 = stdlib_attr("math", "log10").unwrap_or(Py::None);
    digits = op_sub(&op_sub(&places.clone(), &call_builtin("int", &[py_call(&floor, &[py_call(&log10, &[call_builtin("abs", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))])])]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1));
    rounded = call_builtin("round", &[value.clone(), digits.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[rounded.clone()]), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(2)]), -1, "")); __s.push_str(" significant figure"); __s.push_str(&fstr(&plural.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&written.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_places(&[written.clone(), places.clone()]), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(2)]), -1, "")); __s.push_str(" decimal place"); __s.push_str(&fstr(&plural.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _places(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut written: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "written") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: written"))) };
    let mut places: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "places") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: places"))) };
    let mut kept: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A decimal rounded by the hand rule, so 2.675 is 2.68.\n\n    `round(2.675, 2)` gives 2.67, because 2.675 is stored just under itself in binary and the\n    half never arrives; a school answer has to round a half up, which is what Decimal does on\n    the digits as written.\n    ");
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&stdlib_call("decimal", "Decimal", &[Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), "scaleb", &[op_neg(&places.clone())])); __kws.extend(vec![("rounding".to_string(), stdlib_attr("decimal", "ROUND_HALF_UP").unwrap_or(Py::None))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&stdlib_call("decimal", "Decimal", &[call_builtin("str", &[written.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), "quantize", args))(&__args) };
    text = { let mut __s = String::new(); __s.push_str(&fstr(&kept.clone(), -1, &"f")); py_string(__s) };
    if py_truthy(&Py::Bool(!py_contains(&text.clone(), &py_str(".")))) {
    return text.clone();
    }
    return { let __b = mcall(&mcall(&text.clone(), "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]); if py_truthy(&__b) { __b } else { py_str("0") } };
    Py::None
}
pub fn _circle_measure(args: &[Py]) -> Py {
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
    let mut around: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut radius: Py = Py::None;
    let _ = py_str("The circumference of a circle, from its radius or its diameter.");
    if py_truthy(&Py::Bool(!py_contains(&clean.clone(), &py_str("circumference")))) {
    return Py::None;
    }
    match_ = stdlib_call("re", "search", &[py_str("radius\\s*(?:of\\s*)?(\\d+(?:\\.\\d+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    radius = call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    } else {
    match_ = stdlib_call("re", "search", &[py_str("diameter\\s*(?:of\\s*)?(\\d+(?:\\.\\d+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    radius = op_truediv(&call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    }
    around = op_mul(&op_mul(&Py::Int(2), &Py::Float(3.141592653589793)), &radius.clone());
    return { let mut __s = String::new(); __s.push_str("The circumference is 2 x pi x radius, so about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[around.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" (3.14 x "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[op_mul(&Py::Int(2), &radius.clone()), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(")."); py_string(__s) };
    Py::None
}
pub fn _box_volume(args: &[Py]) -> Py {
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
    let mut height: Py = Py::None;
    let mut length: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut size: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut volume: Py = Py::None;
    let mut width: Py = Py::None;
    let _ = py_str("\"volume of a box 10cm by 5cm by 2cm\": length x width x height.");
    if py_truthy(&Py::Bool(!py_contains(&clean.clone(), &py_str("volume")))) {
    return Py::None;
    }
    numbers = stdlib_call("re", "findall", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:cm|m|mm|metres?|centimetres?|inches?|in\\b)?\\s*(?:by|x|\\*)\\s*(\\d+(?:\\.\\d+)?)\\s*(?:cm|m|mm)?\\s*(?:by|x|\\*)\\s*(\\d+(?:\\.\\d+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&numbers.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&numbers.clone(), &Py::Int(0))) { let mut size: Py = __item.clone(); __out.push(call_builtin("float", &[size.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }, 3, None);
    length = __unpk1[0].clone();
    width = __unpk1[1].clone();
    height = __unpk1[2].clone();
    volume = op_mul(&op_mul(&length.clone(), &width.clone()), &height.clone());
    unit = if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("cm")))) { py_str("cm") } else { if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmetres?\\b|\\bm\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("m") } else { py_str("") } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[length.clone()]), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&_number(&[width.clone()]), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&_number(&[height.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[volume.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(&fstr(&if py_truthy(&unit.clone()) { op_add(&op_add(&py_str(" "), &unit.clone()), &py_str("3")) } else { py_str("") }, -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _factorised(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut a: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a"))) };
    let mut b: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "b") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: b"))) };
    let mut c: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "c") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: c"))) };
    let mut common: Py = Py::None;
    let mut number: Py = Py::None;
    let mut p: Py = Py::None;
    let mut q: Py = Py::None;
    let mut r: Py = Py::None;
    let mut reach: Py = Py::None;
    let mut s: Py = Py::None;
    let mut top: Py = Py::None;
    let _ = py_str("(m, (p, q, r, s)) for m(px + q)(rx + s) in whole numbers, or None.\n\n    The common factor comes out first: 2x squared minus 8 is 2(x - 2)(x + 2), and searching\n    the brackets alone found (x - 2)(2x + 4), which is right but not the way it is written.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&a.clone(), &call_builtin("int", &[a.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&b.clone(), &call_builtin("int", &[b.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&c.clone(), &call_builtin("int", &[c.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&a.clone(), &Py::Int(1)) } } } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[a.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[b.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))]), 3, None);
    a = __unpk1[0].clone();
    b = __unpk1[1].clone();
    c = __unpk1[2].clone();
    common = Py::Int(0);
    let __iter2 = py_iter_or_raise(&py_tuple(vec![a.clone(), b.clone(), c.clone()]));
    for __item in __iter2 {
    number = __item.clone();
    common = _gcd(&[common.clone(), number.clone()]);
    }
    if py_truthy(&op_lt(&common.clone(), &Py::Int(1))) {
    common = Py::Int(1);
    }
    let __unpk3 = py_unpack(&py_tuple(vec![op_floordiv(&a.clone(), &common.clone()), op_floordiv(&b.clone(), &common.clone()), op_floordiv(&c.clone(), &common.clone())]), 3, None);
    a = __unpk3[0].clone();
    b = __unpk3[1].clone();
    c = __unpk3[2].clone();
    reach = op_add(&op_add(&call_builtin("abs", &[b.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("abs", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1));
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_add(&a.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    p = __item.clone();
    if py_truthy(&op_modulo(&a.clone(), &p.clone())) {
    continue;
    }
    r = op_floordiv(&a.clone(), &p.clone());
    let __iter5 = py_iter_or_raise(&call_builtin("range", &[op_neg(&reach.clone()), op_add(&reach.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    q = __item.clone();
    top = op_sub(&b.clone(), &op_mul(&r.clone(), &q.clone()));
    if py_truthy(&op_modulo(&top.clone(), &p.clone())) {
    continue;
    }
    s = op_floordiv(&top.clone(), &p.clone());
    if py_truthy(&Py::Bool(py_eq(&op_mul(&q.clone(), &s.clone()), &c.clone()))) {
    return py_tuple(vec![common.clone(), py_tuple(vec![p.clone(), q.clone(), r.clone(), s.clone()])]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _gcd(args: &[Py]) -> Py {
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
    while py_truthy(&other.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![other.clone(), op_modulo(&one.clone(), &other.clone())]), 2, None);
    one = __unpk1[0].clone();
    other = __unpk1[1].clone();
    }
    return call_builtin("abs", &[one.clone()]).unwrap_or_else(|e| exc::raise(e));
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
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut bracket: Py = Py::None;
    let mut common: Py = Py::None;
    let mut first: Py = Py::None;
    let mut p: Py = Py::None;
    let mut q: Py = Py::None;
    let mut r: Py = Py::None;
    let mut s: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("(1, 3, 1, 4) said as (x + 3)(x + 4), and twice the same bracket as (x + 1)^2.");
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    common = __unpk1[0].clone();
    let __unpk2 = py_unpack(&__unpk1[1].clone(), 4, None);
    p = __unpk2[0].clone();
    q = __unpk2[1].clone();
    r = __unpk2[2].clone();
    s = __unpk2[3].clone();
pub fn __nested_bracket_2(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut bracket: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut common: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut first: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut found: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut p: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut q: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut r: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut s: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut second: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut lead: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "lead").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut tail: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "tail").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut left: Py = Py::None;
    left = if py_truthy(&Py::Bool(py_eq(&lead.clone(), &Py::Int(1)))) { py_str("x") } else { { let mut __s = String::new(); __s.push_str(&fstr(&lead.clone(), -1, "")); __s.push_str("x"); py_string(__s) } };
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    return { let mut __s = String::new(); __s.push_str("("); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(")"); py_string(__s) };
    }
    return if py_truthy(&op_gt(&tail.clone(), &Py::Int(0))) { { let mut __s = String::new(); __s.push_str("("); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(" + "); __s.push_str(&fstr(&tail.clone(), -1, "")); __s.push_str(")"); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str("("); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(" - "); __s.push_str(&fstr(&call_builtin("abs", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(")"); py_string(__s) } };
    Py::None
}
    bracket = {{ let bracket = bracket.clone(); let common = common.clone(); let first = first.clone(); let found = found.clone(); let p = p.clone(); let q = q.clone(); let r = r.clone(); let s = s.clone(); let said = said.clone(); let second = second.clone(); py_func("bracket", move |args: &[Py]| { let mut full = vec![bracket.clone(), common.clone(), first.clone(), found.clone(), p.clone(), q.clone(), r.clone(), s.clone(), said.clone(), second.clone()]; full.extend_from_slice(args); __nested_bracket_2(&full) }) }};
    let __unpk4 = py_unpack(&py_tuple(vec![py_call(&bracket, &[p.clone(), q.clone()]), py_call(&bracket, &[r.clone(), s.clone()])]), 2, None);
    first = __unpk4[0].clone();
    second = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&common.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&first.clone(), &py_str("(x)"))) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&common.clone(), -1, "")); __s.push_str("x"); __s.push_str(&fstr(&second.clone(), -1, "")); py_string(__s) };
    }
    said = if py_truthy(&Py::Bool(py_eq(&first.clone(), &second.clone()))) { { let mut __s = String::new(); __s.push_str(&fstr(&first.clone(), -1, "")); __s.push_str("^2"); py_string(__s) } } else { op_add(&first.clone(), &second.clone()) };
    return if py_truthy(&Py::Bool(py_eq(&common.clone(), &Py::Int(1)))) { said.clone() } else { { let mut __s = String::new(); __s.push_str(&fstr(&common.clone(), -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); py_string(__s) } };
    Py::None
}
pub fn _quadratic(args: &[Py]) -> Py {
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
    let mut a: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut b: Py = Py::None;
    let mut c: Py = Py::None;
    let mut constant: Py = Py::None;
    let mut discriminant: Py = Py::None;
    let mut exact: Py = Py::None;
    let mut factorised: Py = Py::None;
    let mut high: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut left: Py = Py::None;
    let mut linear: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut right: Py = Py::None;
    let mut root: Py = Py::None;
    let mut roots: Py = Py::None;
    let mut said: Py = Py::None;
    let mut side: Py = Py::None;
    let mut size: Py = Py::None;
    let mut swap: Py = Py::None;
    let mut text: Py = Py::None;
    let mut wants_brackets: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A quadratic equation with x squared in it, worked out: \"x squared + 5x + 6 = 0\".");
    text = clean.clone();
    let __iter1 = py_iter_or_raise(&g__MATHS_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    swap = __unpk2[1].clone();
    text = stdlib_call("re", "sub", &[pattern.clone(), swap.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    text = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    lead = stdlib_call("re", "search", &[py_str("(?:^|\\s)([+-]?\\d*\\.?\\d*)\\s*\\*?\\s*x\\s*\\^\\s*2"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    return Py::None;
    }
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    b = Py::Float(0.0);
    linear = stdlib_call("re", "match", &[py_str("([+-])\\s*(\\d*\\.?\\d*)\\s*\\*?\\s*x(?![\\w^])"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&linear.clone()) {
    size = if py_truthy(&mcall(&linear.clone(), "group", &[Py::Int(2)])) { call_builtin("float", &[mcall(&linear.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Float(1.0) };
    b = if py_truthy(&Py::Bool(py_eq(&mcall(&linear.clone(), "group", &[Py::Int(1)]), &py_str("+")))) { size.clone() } else { op_neg(&size.clone()) };
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&linear.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    c = Py::Float(0.0);
    constant = stdlib_call("re", "match", &[py_str("([+-])\\s*(\\d*\\.?\\d*)"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&constant.clone()) {
    size = if py_truthy(&mcall(&constant.clone(), "group", &[Py::Int(2)])) { call_builtin("float", &[mcall(&constant.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Float(1.0) };
    c = if py_truthy(&Py::Bool(py_eq(&mcall(&constant.clone(), "group", &[Py::Int(1)]), &py_str("+")))) { size.clone() } else { op_neg(&size.clone()) };
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&constant.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    right = Py::Float(0.0);
    if py_truthy(&mcall(&rest.clone(), "startswith", &[py_str("=")])) {
    side = mcall(&slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[py_str("[+-]?\\d+\\.?\\d*"), side.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    right = call_builtin("float", &[side.clone()]).unwrap_or_else(|e| exc::raise(e));
    rest = py_str("");
    }
    if py_truthy(&rest.clone()) {
    return Py::None;
    }
    raw = mcall(&lead.clone(), "group", &[Py::Int(1)]);
    a = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str(""), py_str("+")]), &raw.clone()))) { Py::Float(1.0) } else { if py_truthy(&Py::Bool(py_eq(&raw.clone(), &py_str("-")))) { op_neg(&Py::Float(1.0)) } else { call_builtin("float", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&a.clone()))) {
    return Py::None;
    }
    asked = mcall(&mcall(&clean.clone(), "replace", &[py_str("^2"), py_str(" ")]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&right.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUADRATIC_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&asked.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    let __aug3 = op_sub(&c.clone(), &right.clone());
    c = __aug3;
    discriminant = op_sub(&op_mul(&b.clone(), &b.clone()), &op_mul(&op_mul(&Py::Int(4), &a.clone()), &c.clone()));
    factorised = _factorised(&[a.clone(), b.clone(), c.clone()]);
    wants_brackets = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FACTOR_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&asked.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&discriminant.clone(), &op_neg(&Py::Float(1e-09)))) {
    if py_truthy(&wants_brackets.clone()) {
    return { let mut __s = String::new(); __s.push_str("It does not factorise with whole numbers, and no real number solves it either: the square of a number is never negative, so the formula stops at the square root of a negative number."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("No real number solves that: the square of a number is never negative, so there is no real answer. (The formula gives the square root of a negative number here.)"); py_string(__s) };
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[discriminant.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    root = op_truediv(&op_neg(&b.clone()), &op_mul(&Py::Int(2), &a.clone()));
    if py_truthy(&{ let __b = wants_brackets.clone(); if !py_truthy(&__b) { __b } else { factorised.clone() } }) {
    return { let mut __s = String::new(); __s.push_str("It factorises as "); __s.push_str(&fstr(&_said(&[factorised.clone()]), -1, "")); __s.push_str(", so x = "); __s.push_str(&fstr(&_number(&[root.clone()]), -1, "")); __s.push_str(" is the only answer: both brackets are the same, which is why there is one root, not two."); py_string(__s) };
    }
    said = { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&_number(&[root.clone()]), -1, "")); __s.push_str(". There is one answer, not two, because both roots of the formula are the same."); py_string(__s) };
    if py_truthy(&factorised.clone()) {
    let __aug4 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" The factorised form is "); __s.push_str(&fstr(&_said(&[factorised.clone()]), -1, "")); __s.push_str("."); py_string(__s) });
    said = __aug4;
    }
    return said.clone();
    }
    roots = call_builtin("sorted", &[py_tuple(vec![op_truediv(&op_add(&op_neg(&b.clone()), &op_pow(&discriminant.clone(), &Py::Float(0.5))), &op_mul(&Py::Int(2), &a.clone())), op_truediv(&op_sub(&op_neg(&b.clone()), &op_pow(&discriminant.clone(), &Py::Float(0.5))), &op_mul(&Py::Int(2), &a.clone()))])]).unwrap_or_else(|e| exc::raise(e));
    whole = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&roots.clone()) { let mut root: Py = __item.clone(); __out.push(op_lt(&call_builtin("abs", &[op_sub(&root.clone(), &call_builtin("round", &[root.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&wants_brackets.clone()) {
    if py_truthy(&factorised.clone()) {
    said = { let mut __s = String::new(); __s.push_str("It factorises as "); __s.push_str(&fstr(&_said(&[factorised.clone()]), -1, "")); __s.push_str(", so x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(0))]), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(1))]), -1, "")); __s.push_str("."); py_string(__s) };
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&geti(&roots.clone(), &Py::Int(0)), &geti(&roots.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    return op_add(&said.clone(), &py_str(" Multiply the brackets out to check you get the equation back."));
    }
    return said.clone();
    }
    return { let mut __s = String::new(); __s.push_str("It does not factorise with whole numbers, so put a = "); __s.push_str(&fstr(&_number(&[a.clone()]), -1, "")); __s.push_str(", b = "); __s.push_str(&fstr(&_number(&[b.clone()]), -1, "")); __s.push_str(" and c = "); __s.push_str(&fstr(&_number(&[c.clone()]), -1, "")); __s.push_str(" into x = (-b +/- the square root of b squared - 4ac) / 2a: that gives x = "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[geti(&roots.clone(), &Py::Int(0)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[geti(&roots.clone(), &Py::Int(1)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&whole.clone()) {
    let __unpk5 = py_unpack(&py_tuple(vec![call_builtin("int", &[call_builtin("round", &[geti(&roots.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[call_builtin("round", &[geti(&roots.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    left = __unpk5[0].clone();
    high = __unpk5[1].clone();
    said = { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str(". Put either one back in and the equation comes to zero, which is the check."); py_string(__s) };
    if py_truthy(&factorised.clone()) {
    let __aug6 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" The factorised form is "); __s.push_str(&fstr(&_said(&[factorised.clone()]), -1, "")); __s.push_str("."); py_string(__s) });
    said = __aug6;
    }
    return said.clone();
    }
    exact = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&roots.clone()) { let mut root: Py = __item.clone(); __out.push(op_lt(&call_builtin("abs", &[op_sub(&root.clone(), &call_builtin("round", &[root.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&factorised.clone()) {
    return { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(0))]), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(1))]), -1, "")); __s.push_str(", and both are exact: the factorised form is "); __s.push_str(&fstr(&_said(&[factorised.clone()]), -1, "")); __s.push_str(". Put either one back in and the equation comes to zero."); py_string(__s) };
    }
    if py_truthy(&exact.clone()) {
    return { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(0))]), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&_number(&[geti(&roots.clone(), &Py::Int(1))]), -1, "")); __s.push_str(". Put either one back in and the equation comes to zero, which is the check."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[geti(&roots.clone(), &Py::Int(0)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" or x = "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[geti(&roots.clone(), &Py::Int(1)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(", to two decimal places: it does not factor with whole numbers, so use the formula x = (-b +/- the square root of b squared - 4ac) / 2a."); py_string(__s) };
    Py::None
}
pub fn _equation(args: &[Py]) -> Py {
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
    let mut add: Py = Py::None;
    let mut coefficient: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut right: Py = Py::None;
    let mut word: Py = Py::None;
    let mut x: Py = Py::None;
    let _ = py_str("A one-variable equation with x: \"solve 2x + 3 = 11\".");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&clean.clone(), &py_str("="))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])x(?![a-z])"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("solve"), py_str("find x"), py_str("value of x"), py_str("what is x")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "fullmatch", &[py_str("[\\s0-9x+\\-=*.]*"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    match_ = stdlib_call("re", "search", &[py_str("([+-]?\\d*)\\*?x(?![a-z])\\s*([+-]\\s*\\d+(?:\\.\\d+)?)?\\s*=\\s*([+-]?\\d+(?:\\.\\d+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    coefficient = if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str(""), py_str("+"), py_str("-")]), &mcall(&match_.clone(), "group", &[Py::Int(1)])))) { call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Float(1.0) };
    if py_truthy(&Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str("-")))) {
    coefficient = op_neg(&Py::Float(1.0));
    }
    add = if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) { call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "replace", &[py_str(" "), py_str("")])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Float(0.0) };
    right = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(" "), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&coefficient.clone()))) {
    return Py::None;
    }
    x = op_truediv(&op_sub(&right.clone(), &add.clone()), &coefficient.clone());
    return { let mut __s = String::new(); __s.push_str("x = "); __s.push_str(&fstr(&_number(&[x.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _percent(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut part: Py = Py::None;
    let mut total: Py = Py::None;
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?:%|percent|per cent)\\s*(?:tip|gratuity)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    after = stdlib_call("re", "search", &[py_str("(?:costs?|priced at|is|are|was)\\s+(?:£|\\$)?([\\d,.]+)\\D.*?(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*off"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&after.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![_amount(&[mcall(&after.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&after.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk1[0].clone();
    part = __unpk1[1].clone();
    amount = op_truediv(&op_mul(&total.clone(), &part.clone()), &Py::Int(100));
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% off "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" leaves "); __s.push_str(&fstr(&_number(&[op_sub(&total.clone(), &amount.clone())]), -1, "")); __s.push_str(" (you save "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(")."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*(?:service(?:\\s+charge)?|tax|charge|vat)?\\s*(of|off|on(?:\\s+top\\s+of)?)\\s*(?:(?:my|our|the|a|an)\\s+)?(?:(?:bill|total|check|tab)\\s+of\\s+)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[py_str("what(?:'s| is)\\s*(\\d+(?:\\.\\d+)?)\\s*(of|off)\\s*([\\d,.]+)\\s*%"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), mcall(&match_.clone(), "group", &[Py::Int(2)]), call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e))]), 3, None);
    part = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    total = __unpk2[2].clone();
    } else {
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), mcall(&match_.clone(), "group", &[Py::Int(2)]), call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e))]), 3, None);
    part = __unpk3[0].clone();
    kind = __unpk3[1].clone();
    total = __unpk3[2].clone();
    }
    amount = op_truediv(&op_mul(&total.clone(), &part.clone()), &Py::Int(100));
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("off")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% off "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" leaves "); __s.push_str(&fstr(&_number(&[op_sub(&total.clone(), &amount.clone())]), -1, "")); __s.push_str(" (you save "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(")."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _percentage_change(args: &[Py]) -> Py {
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
    let mut change: Py = Py::None;
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    match_ = stdlib_call("re", "search", &[py_str("(?:percentage|percent)\\s+(?:increase|change|difference)\\s+from\\s+([\\d,.]+)\\s+to\\s+([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    start = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    end = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&start.clone()))) {
    return Py::None;
    }
    change = op_mul(&op_truediv(&op_sub(&end.clone(), &start.clone()), &start.clone()), &Py::Int(100));
    word = if py_truthy(&op_ge(&change.clone(), &Py::Int(0))) { py_str("increase") } else { py_str("decrease") };
    return { let mut __s = String::new(); __s.push_str("That's a "); __s.push_str(&fstr(&_number(&[call_builtin("abs", &[change.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("% "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _amount(args: &[Py]) -> Py {
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
    let _ = py_str("The number a short phrase gives, as a float.");
    return call_builtin("float", &[mcall(&text.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _what_percent(args: &[Py]) -> Py {
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
    let mut exact: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("One number as a percentage of another: \"what percentage is 15 of 80\".");
    part = Py::None;
    total = Py::None;
    match_ = stdlib_call("re", "search", &[py_str("what (?:percentage|percent) of ([\\d,.]+) is ([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk1[0].clone();
    part = __unpk1[1].clone();
    }
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    match_ = stdlib_call("re", "search", &[py_str("what (?:percentage|percent) (?:is )?([\\d,.]+) (?:of|out of) ([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk2[0].clone();
    total = __unpk2[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s+(?:is\\s+)?what\\s+(?:percentage|percent)\\s+of\\s+([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk3[0].clone();
    total = __unpk3[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+) (?:out of|of) ([\\d,.]+) (?:as a |in )?(?:percentage|percent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk4 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk4[0].clone();
    total = __unpk4[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&part.clone(), &Py::None))) {
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+) as a (?:percentage|percent) of ([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk5 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk5[0].clone();
    total = __unpk5[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&part.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&total.clone())) } }) {
    return Py::None;
    }
    share = op_mul(&op_truediv(&part.clone(), &total.clone()), &Py::Int(100));
    said = _number(&[call_builtin("round", &[share.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]);
    exact = op_lt(&call_builtin("abs", &[op_sub(&share.clone(), &call_builtin("round", &[share.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09));
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&if py_truthy(&exact.clone()) { py_str("exactly ") } else { py_str("about ") }, -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _a_share_of_a_group(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut bottom: Py = Py::None;
    let mut count: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut size: Py = Py::None;
    let mut top: Py = Py::None;
    let _ = py_str("A share scaled to a bigger group: \"1 in 5 of 200\" is 40 people.\n\n    \"if 1 in 5 people do it how many in 200\" is the commonest way this turns up, and nothing\n    answered it: the line was only noted back to the person. The share can be said in words\n    (\"one in five people\") and the group can come before the question (\"out of 200 people how\n    many if 1 in 5\"), so the two numbers are read separately: first the share, then the size of\n    the group, from whatever wording carries it.\n    ");
    bare = mcall(&g__COUNT(), "strip", &[py_str("()")]);
    share = stdlib_call("re", "search", &[op_modulo(&py_str("\\b(%s)\\s+in\\s+(%s)\\b"), &py_tuple(vec![bare.clone(), bare.clone()])), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&share.clone()))) {
    return Py::None;
    }
    top = mcall(&g__COUNTS(), "get", &[mcall(&share.clone(), "group", &[Py::Int(1)])]);
    bottom = mcall(&g__COUNTS(), "get", &[mcall(&share.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&top.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&bottom.clone(), &Py::None)) } }) {
    let __try1 = exc::catch(|| {
    let __unpk6 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&share.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&share.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    top = __unpk6[0].clone();
    bottom = __unpk6[1].clone();
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
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bottom.clone())); if py_truthy(&__b) { __b } else { op_gt(&top.clone(), &bottom.clone()) } }) {
    return Py::None;
    }
    rest = op_add(&op_add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&share.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clean.clone(), &mcall(&share.clone(), "end", &[]), &Py::None, &Py::None));
    size = Py::None;
    let __iter7 = py_iter_or_raise(&py_tuple(vec![py_str("how many (?:people |of them |would that be )?(?:are |is )?(?:in|of|out of|from|out of a)\\s+(\\d+)"), py_str("\\b(?:in|of|out of|from)\\s+(\\d+)\\s+(?:people|persons|students|adults|children|kids|men|women|workers|drivers|homes|households)"), py_str("\\b(?:out of|of|from)\\s+(\\d+)\\b")]));
    for __item in __iter7 {
    pattern = __item.clone();
    match_ = stdlib_call("re", "search", &[pattern.clone(), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    size = call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&size.clone())); if py_truthy(&__b) { __b } else { op_lt(&size.clone(), &Py::Int(2)) } }) {
    return Py::None;
    }
    count = op_truediv(&op_mul(&top.clone(), &size.clone()), &bottom.clone());
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    answer = _number(&[call_builtin("int", &[call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    answer = op_add(&py_str("about "), &_number(&[call_builtin("round", &[count.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]));
    }
    said = op_modulo(&py_str("%s in %s"), &py_tuple(vec![_number(&[top.clone()]), _number(&[bottom.clone()])]));
    if py_truthy(&Py::Bool(py_eq(&top.clone(), &Py::Int(1)))) {
    said = op_modulo(&py_str("1 in %s"), &_number(&[bottom.clone()]));
    }
    return op_add(&{ let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&_number(&[size.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&answer.clone(), -1, "")); __s.push_str(". That is the share times the group: "); __s.push_str(&fstr(&_number(&[size.clone()]), -1, "")); __s.push_str(" divided by "); __s.push_str(&fstr(&_number(&[bottom.clone()]), -1, "")); py_string(__s) }, &if py_truthy(&op_gt(&top.clone(), &Py::Int(1))) { { let mut __s = String::new(); __s.push_str(", times "); __s.push_str(&fstr(&_number(&[top.clone()]), -1, "")); __s.push_str("."); py_string(__s) } } else { py_str(".") });
    Py::None
}
pub fn _percent_on(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut going_down: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut part: Py = Py::None;
    let mut total: Py = Py::None;
    let mut way: Py = Py::None;
    let _ = py_str("A percentage added to an amount: \"add 20% to 40\", \"40 plus VAT\", \"add VAT to 240\".");
    match_ = stdlib_call("re", "search", &[py_str("\\b(increase|decrease|raise|reduce|lower|go up|go down|goes up|goes down)\\s+(?:of\\s+)?(?:£|\\$)?([\\d,.]+)\\s+by\\s+(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(3)])])]), 2, None);
    total = __unpk1[0].clone();
    part = __unpk1[1].clone();
    going_down = Py::Bool(py_contains(&py_tuple(vec![py_str("decrease"), py_str("reduce"), py_str("lower"), py_str("go down"), py_str("goes down")]), &mcall(&match_.clone(), "group", &[Py::Int(1)])));
    changed = op_mul(&total.clone(), &if py_truthy(&going_down.clone()) { op_sub(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))) } else { op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))) });
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&if py_truthy(&going_down.clone()) { py_str("decreased") } else { py_str("increased") }, -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[changed.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s+(?:on(?:\\s+top\\s+of)?|onto|added\\s+to)\\s+(?:£|\\$)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk2[0].clone();
    total = __unpk2[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s+(more|less|fewer|higher|lower)\\s+than\\s+(?:£|\\$|€)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), mcall(&match_.clone(), "group", &[Py::Int(2)]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(3)])])]), 3, None);
    part = __unpk3[0].clone();
    way = __unpk3[1].clone();
    total = __unpk3[2].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("more"), py_str("higher")]), &way.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" less "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_sub(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?:costs?|priced at|is|are|was)\\s+(?:£|\\$|€)?\\s*[\\d,.]+\\D.*?(?:\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*off"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:£|\\$|€)?\\s*([\\d,.]+)\\s*(?:pounds?|quid|euros?|dollars?|bucks?)\\s+(?:[a-z]+\\s+){0,3}?(?:with|at|minus|less)\\s+(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*off\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk4 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk4[0].clone();
    part = __unpk4[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" less "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_sub(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:add|increase)\\s+(?:a\\s+)?(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*(?:tip|service|tax|charge)?\\s*(?:on|to|onto)\\s+(?:£|\\$)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk5 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk5[0].clone();
    total = __unpk5[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\d+(?:\\.\\d+)?\\s*(?:%|percent|per cent)\\s*off\\s+(?:£|\\$)?[\\d,.]+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:subtract|deduct|minus|take|taking|remove|knock|strip)\\s+(?:a\\s+|off\\s+)?(?:the\\s+)?(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent|per cent)\\s*(?:off|from|of)\\s+(?:£|\\$)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk6 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk6[0].clone();
    total = __unpk6[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" less "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_sub(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:minus|less|without)\\s*(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk7 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk7[0].clone();
    part = __unpk7[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" less "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_sub(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:plus|and)\\s*(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk8 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk8[0].clone();
    part = __unpk8[1].clone();
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:plus|with|including|before)\\s+vat"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    total = _amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus VAT at 20% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &Py::Float(1.2))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:add|plus)\\s+(?:(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s+)?(?:the\\s+)?vat\\s+(?:on|to|onto)\\s*(?:£|\\$)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    part = if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(1)])) { _amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]) } else { Py::Float(20.0) };
    total = _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" plus VAT at "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% is "); __s.push_str(&fstr(&_number(&[op_mul(&total.clone(), &op_add(&Py::Int(1), &op_truediv(&part.clone(), &Py::Int(100))))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = stdlib_call("re", "search", &[py_str("(?:take|takes|taking|remove|removing|strip|knock)\\s+(?:the\\s+)?vat\\s+(?:off|from|out of)\\s*(?:£|\\$)?([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(?:minus|less|without|excluding|ex)\\s+vat"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&match_.clone()) {
    total = _amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" with VAT taken off is "); __s.push_str(&fstr(&_number(&[op_truediv(&total.clone(), &Py::Float(1.2))]), -1, "")); __s.push_str(", and the VAT in it is "); __s.push_str(&fstr(&_number(&[op_sub(&total.clone(), &op_truediv(&total.clone(), &Py::Float(1.2)))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _percent_rise(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut change: Py = Py::None;
    let mut down: Py = Py::None;
    let mut money_word: Py = Py::None;
    let mut number: Py = Py::None;
    let mut part: Py = Py::None;
    let mut percents: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut up: Py = Py::None;
    let _ = py_str("The money a percentage adds to or takes off: \"rent 800 going up 5 percent\".");
    percents = stdlib_call("re", "findall", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[percents.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    part = call_builtin("float", &[geti(&percents.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = part.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(100)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    money_word = call_builtin("bool", &[mcall(&g__MONEY_WORD(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    amount = Py::None;
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("(?:£|\\$|€)?\\s*(\\d[\\d,]*(?:\\.\\d+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    raw = __item.clone();
    number = _amount(&[mcall(&raw.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&number.clone(), &part.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("£$€"), &slice_or_raise(&mcall(&mcall(&raw.clone(), "group", &[Py::Int(0)]), "strip", &[]), &Py::None, &Py::Int(1), &Py::None))); if py_truthy(&__b) { __b } else { { let __b = op_ge(&number.clone(), &Py::Int(100)); if py_truthy(&__b) { __b } else { money_word.clone() } } } }) {
    amount = number.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&amount.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PERCENT_ADVICE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    up = stdlib_call("re", "search", &[op_modulo(&py_str("\\b(?:%s)\\b"), &g__PERCENT_UP()), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    down = stdlib_call("re", "search", &[op_modulo(&py_str("\\b(?:%s)\\b"), &g__PERCENT_DOWN()), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("bool", &[up.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("bool", &[down.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    change = op_truediv(&op_mul(&amount.clone(), &part.clone()), &Py::Int(100));
    if py_truthy(&up.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[change.clone()]), -1, "")); __s.push_str(", so it goes up to "); __s.push_str(&fstr(&_number(&[op_add(&amount.clone(), &change.clone())]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[change.clone()]), -1, "")); __s.push_str(", so it comes down to "); __s.push_str(&fstr(&_number(&[op_sub(&amount.clone(), &change.clone())]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _amount_in_the_chat(args: &[Py]) -> Py {
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
    let mut numbers: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The last amount the person said, skipping the percentages in what they said.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    said = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = stdlib_call("re", "sub", &[py_str("\\d+(?:\\.\\d+)?\\s*(?:%|percent|per cent)"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    numbers = stdlib_call("re", "findall", &[py_str("\\d[\\d,]*(?:\\.\\d+)?"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&numbers.clone()))) {
    continue;
    }
    value = _amount(&[geti(&numbers.clone(), &op_neg(&Py::Int(1)))]);
    if py_truthy(&op_ge(&value.clone(), &Py::Int(10))) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _percent_of_the_figure_in_the_chat(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut change: Py = Py::None;
    let mut off_reference: Py = Py::None;
    let mut part: Py = Py::None;
    let mut percents: Py = Py::None;
    let mut reference: Py = Py::None;
    let _ = py_str("A percentage of the amount the chat already holds: \"and 20 percent of that\".");
    percents = stdlib_call("re", "findall", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[percents.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), stdlib_call("re", "sub", &[py_str("\\d+(?:\\.\\d+)?\\s*(?:%|percent|per cent)"), py_str(" "), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    reference = stdlib_call("re", "search", &[py_str("\\b(?:of|on)\\s+(?:that|it|this|those|them|the\\s+(?:rent|bill|price|salary|total|figure|number|amount|money))\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    off_reference = stdlib_call("re", "search", &[py_str("\\b(?:off)\\s+(?:that|it|this|those|them)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    asked = { let __b = mcall(&g__PERCENT_ASK_WORD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:be|come\\s+to|work\\s+out\\s+to)\\b[\\s?.!]*$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = reference.clone(); if py_truthy(&__b) { __b } else { { let __b = off_reference.clone(); if py_truthy(&__b) { __b } else { asked.clone() } } } }))) {
    return Py::None;
    }
    amount = _amount_in_the_chat(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&amount.clone(), &Py::None))) {
    return Py::None;
    }
    part = call_builtin("float", &[geti(&percents.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = part.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(100)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    change = op_truediv(&op_mul(&amount.clone(), &part.clone()), &Py::Int(100));
    if py_truthy(&off_reference.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% off "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(" leaves "); __s.push_str(&fstr(&_number(&[op_sub(&amount.clone(), &change.clone())]), -1, "")); __s.push_str(" (you save "); __s.push_str(&fstr(&_number(&[change.clone()]), -1, "")); __s.push_str(")."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[change.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _tip(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut part: Py = Py::None;
    let mut tip: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A tip on a bill: \"15% tip on 45\", \"tip on 80 at 15 percent\".");
    match_ = stdlib_call("re", "search", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*(?:tip|service|gratuity)\\s*(?:on(?:\\s+top\\s+of)?|for|of)\\s*(?:(?:a|an|the|my|our)\\s+)?(?:(?:bill|total|check|tab)\\s+of\\s+)?(?:£|\\$|€)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk1[0].clone();
    total = __unpk1[1].clone();
    } else {
    match_ = stdlib_call("re", "search", &[py_str("(?:tip|service|gratuity)\\s*(?:of\\s*)?(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)\\s*(?:on|for|of)\\s*(?:£|\\$|€)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    part = __unpk2[0].clone();
    total = __unpk2[1].clone();
    } else {
    match_ = stdlib_call("re", "search", &[py_str("(?:tip|service|gratuity)\\s*(?:on|for)\\s*(?:£|\\$|€)?\\s*([\\d,.]+)\\s*(?:at|of)\\s*(\\d+(?:\\.\\d+)?)\\s*(?:%|percent|per cent)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![_amount(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]), _amount(&[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    total = __unpk3[0].clone();
    part = __unpk3[1].clone();
    }
    }
    tip = op_truediv(&op_mul(&total.clone(), &part.clone()), &Py::Int(100));
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&_number(&[part.clone()]), -1, "")); __s.push_str("% tip on "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[tip.clone()]), -1, "")); __s.push_str(", so "); __s.push_str(&fstr(&_number(&[op_add(&total.clone(), &tip.clone())]), -1, "")); __s.push_str(" in total."); py_string(__s) };
    Py::None
}
pub fn _two_places(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Money in a card's shape: 4.80 rather than 4.8, and 6 rather than 6.00.");
    text = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".2f")); py_string(__s) };
    return if py_truthy(&mcall(&text.clone(), "endswith", &[py_str(".00")])) { slice_or_raise(&text.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None) } else { text.clone() };
    Py::None
}
pub fn _left_on_the_bill(args: &[Py]) -> Py {
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
    let mut bill: Py = Py::None;
    let mut bill_text: Py = Py::None;
    let mut left: Py = Py::None;
    let mut left_text: Py = Py::None;
    let mut loose: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut money: Py = Py::None;
    let mut percent: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut usual: Py = Py::None;
    let _ = py_str("The percentage somebody left on a bill, and the usual tip on that bill.");
    match_ = mcall(&g__YOU_LEFT(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[py_str("left")]), mcall(&match_.clone(), "group", &[py_str("bill")])]), 2, None);
    left_text = __unpk1[0].clone();
    bill_text = __unpk1[1].clone();
    } else {
    loose = mcall(&g__YOU_LEFT_ANYWHERE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = loose.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__BILL_WORD(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&loose.clone(), "group", &[Py::Int(1)]), mcall(&loose.clone(), "group", &[Py::Int(2)])]), 2, None);
    left_text = __unpk2[0].clone();
    bill_text = __unpk2[1].clone();
    }
    left = { let __b = mcall(&g__A_NOTE(), "get", &[left_text.clone()]); if py_truthy(&__b) { __b } else { _amount(&[left_text.clone()]) } };
    bill = _amount(&[bill_text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bill.clone())); if py_truthy(&__b) { __b } else { { let __b = op_le(&left.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_le(&bill.clone(), &Py::Int(0)) } } } } } }) {
    return Py::None;
    }
    percent = op_mul(&op_truediv(&left.clone(), &bill.clone()), &Py::Int(100));
    money = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("£"), py_str("$"), py_str("€")])) { let mut sign: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&clean.clone(), &sign.clone()))) {  __out.push(sign.clone()); } } py_list(__out) })() }, py_str("£")]).unwrap_or_else(|e| exc::raise(e));
    usual = { let mut __s = String::new(); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_two_places(&[call_builtin("round", &[op_mul(&bill.clone(), &Py::Float(0.1)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_two_places(&[call_builtin("round", &[op_mul(&bill.clone(), &Py::Float(0.125)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); py_string(__s) };
    said = _number(&[call_builtin("round", &[percent.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&op_ge(&percent.clone(), &Py::Int(10))) {
    return { let mut __s = String::new(); __s.push_str("That's "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("% of the "); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_number(&[bill.clone()]), -1, "")); __s.push_str(" - the usual tip is ten to twelve and a half ("); __s.push_str(&fstr(&usual.clone(), -1, "")); __s.push_str("), so it's enough."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("That's "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("% of the "); __s.push_str(&fstr(&money.clone(), -1, "")); __s.push_str(&fstr(&_number(&[bill.clone()]), -1, "")); __s.push_str(", under the usual ten to twelve and a half ("); __s.push_str(&fstr(&usual.clone(), -1, "")); __s.push_str("). Check the bill for a service charge first, since that is the tip already on it."); py_string(__s) };
    Py::None
}
pub fn _save_time(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut goal: Py = Py::None;
    let mut length: Py = Py::None;
    let mut per: Py = Py::None;
    let mut total: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("How long a regular amount takes to reach a goal: \"save 50 a month for 1000\".");
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("how long|how many (?:months|weeks|years)|when will"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    per = stdlib_call("re", "search", &[py_str("(?:save|saving|put away|putting away|set aside|setting aside)\\s+(?:£|\\$|€)?\\s*([\\d,.]+)\\s*(?:a|per|each)\\s*(month|week|year)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    goal = stdlib_call("re", "search", &[py_str("(?:to save|save up|reach|get to|hit|for)\\s+(?:£|\\$|€)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&per.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&goal.clone())) } }) {
    return Py::None;
    }
    amount = _amount(&[mcall(&per.clone(), "group", &[Py::Int(1)])]);
    total = _amount(&[mcall(&goal.clone(), "group", &[Py::Int(1)])]);
    unit = mcall(&per.clone(), "group", &[Py::Int(2)]);
    if py_truthy(&{ let __b = op_le(&amount.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_le(&total.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    count = op_truediv(&total.clone(), &amount.clone());
    whole = call_builtin("int", &[count.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &whole.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    length = { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); py_string(__s) };
    } else {
    length = { let mut __s = String::new(); __s.push_str("about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[count.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str("s"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("Saving "); __s.push_str(&fstr(&_number(&[amount.clone()]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(", you get to "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&length.clone(), -1, "")); __s.push_str("."); py_string(__s) };
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut amount: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "amount") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: amount"))) };
    let mut sign: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The amount with the currency the person used, so a dollar question stays in dollars.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("€"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("euro"))) } }) {
    sign = py_str("€");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("$"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("dollar"))) } }) {
    sign = py_str("$");
    } else {
    sign = py_str("£");
    }
    }
    text = { let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, &",.2f")); py_string(__s) };
    if py_truthy(&mcall(&text.clone(), "endswith", &[py_str(".00")])) {
    text = slice_or_raise(&text.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None);
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&sign.clone(), -1, "")); __s.push_str(&fstr(&text.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _annual_pay(args: &[Py]) -> Py {
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
    let mut digits: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The yearly figure in the line, or None when the number is an age or a length of time.\n\n    Every part of _ANNUAL is optional, which is what lets \"30000 a year\" and \"30k a year\" both\n    read as a salary; the price is that a bare \"3 year\" reads as one too. So a figure with no\n    currency, no unit, no \"k\" and fewer than four digits has to be carried by the language of pay\n    somewhere in the line (\"i earn 40 a year\" is odd but it is a wage question; \"my 3 year old\"\n    is not).\n    ");
    match_ = mcall(&g__ANNUAL(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) {
    return match_.clone();
    }
    digits = stdlib_call("re", "sub", &[py_str("[.,]"), py_str(""), mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_ge(&call_builtin("len", &[digits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return match_.clone();
    }
    if py_truthy(&mcall(&g__MONEY_BESIDE(), "search", &[clean.clone()])) {
    return match_.clone();
    }
    return Py::None;
    Py::None
}
pub fn _wage(args: &[Py]) -> Py {
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
    let mut annual: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asks_hour: Py = Py::None;
    let mut asks_month: Py = Py::None;
    let mut asks_week: Py = Py::None;
    let mut asks_year: Py = Py::None;
    let mut hourly: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut hours_match: Py = Py::None;
    let mut per_hour: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut value: Py = Py::None;
    let mut weekly: Py = Py::None;
    let mut yearly: Py = Py::None;
    let _ = py_str("Pay worked out from an hourly rate, or an hourly rate from a yearly salary.\n\n    The line has to be about pay: \"120 miles at 60 miles an hour\" is travel, not a wage.\n    Hours default to 40 a week because that is the assumption most people mean, and it is\n    said in the reply. Tax is deliberately left out: the answer is the gross figure, and a\n    question that asks for take-home is passed on rather than answered with a guess.\n    ");
    if py_truthy(&mcall(&g__NOT_PAY(), "search", &[clean.clone()])) {
    return Py::None;
    }
    hours_match = mcall(&g__HOURS_WEEK(), "search", &[clean.clone()]);
    hours = if py_truthy(&hours_match.clone()) { call_builtin("float", &[mcall(&hours_match.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Float(40.0) };
    if py_truthy(&{ let __b = op_le(&hours.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_gt(&hours.clone(), &Py::Int(100)) } }) {
    return Py::None;
    }
    rate = mcall(&g__RATE(), "search", &[clean.clone()]);
    annual = _annual_pay(&[clean.clone()]);
    asked = mcall(&g__HOURS_WEEK(), "sub", &[py_str(" "), clean.clone()]);
    asks_year = stdlib_call("re", "search", &[py_str("\\b(?:a|per|each|/)\\s*(?:year|annum)|\\bannual(?:ly)?\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e));
    asks_month = stdlib_call("re", "search", &[py_str("\\b(?:a|per|each|/)\\s*month|\\bmonthly\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e));
    asks_week = stdlib_call("re", "search", &[py_str("\\b(?:a|per|each|/)\\s*week|\\bweekly\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e));
    asks_hour = stdlib_call("re", "search", &[py_str("\\b(?:an|a|per|each|/)\\s*hour|\\bhourly\\b"), asked.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__PAY_WORDS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = rate.clone(); if !py_truthy(&__b) { __b } else { { let __b = hours_match.clone(); if py_truthy(&__b) { __b } else { { let __b = asks_year.clone(); if py_truthy(&__b) { __b } else { { let __b = asks_month.clone(); if py_truthy(&__b) { __b } else { asks_week.clone() } } } } } } } }; if py_truthy(&__b) { __b } else { { let __b = annual.clone(); if !py_truthy(&__b) { __b } else { asks_hour.clone() } } } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&annual.clone())) } }) {
    hourly = call_builtin("float", &[mcall(&rate.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    weekly = op_mul(&hourly.clone(), &hours.clone());
    yearly = op_mul(&weekly.clone(), &Py::Int(52));
    if py_truthy(&{ let __b = asks_month.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_year.clone())) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), hourly.clone()]), -1, "")); __s.push_str(" an hour at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is about "); __s.push_str(&fstr(&_money(&[clean.clone(), op_truediv(&yearly.clone(), &Py::Int(12))]), -1, "")); __s.push_str(" a month before tax."); py_string(__s) };
    }
    if py_truthy(&{ let __b = asks_week.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = asks_year.clone(); if py_truthy(&__b) { __b } else { asks_month.clone() } })) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), hourly.clone()]), -1, "")); __s.push_str(" an hour at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is "); __s.push_str(&fstr(&_money(&[clean.clone(), weekly.clone()]), -1, "")); __s.push_str(" a week."); py_string(__s) };
    }
    if py_truthy(&asks_year.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), hourly.clone()]), -1, "")); __s.push_str(" an hour at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is about "); __s.push_str(&fstr(&_money(&[clean.clone(), yearly.clone()]), -1, "")); __s.push_str(" a year before tax."); py_string(__s) };
    }
    if py_truthy(&mcall(&g__PAY_WORDS(), "search", &[clean.clone()])) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), hourly.clone()]), -1, "")); __s.push_str(" an hour at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is "); __s.push_str(&fstr(&_money(&[clean.clone(), weekly.clone()]), -1, "")); __s.push_str(" a week, about "); __s.push_str(&fstr(&_money(&[clean.clone(), yearly.clone()]), -1, "")); __s.push_str(" a year before tax."); py_string(__s) };
    }
    if py_truthy(&hours_match.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), hourly.clone()]), -1, "")); __s.push_str(" an hour at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is "); __s.push_str(&fstr(&_money(&[clean.clone(), weekly.clone()]), -1, "")); __s.push_str(" a week, about "); __s.push_str(&fstr(&_money(&[clean.clone(), yearly.clone()]), -1, "")); __s.push_str(" a year before tax."); py_string(__s) };
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = annual.clone(); if !py_truthy(&__b) { __b } else { { let __b = asks_hour.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("hourly rate"))) } } } }) {
    value = call_builtin("float", &[mcall(&mcall(&annual.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&annual.clone(), "group", &[Py::Int(2)])) {
    let __aug1 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug1;
    }
    per_hour = op_truediv(&value.clone(), &op_mul(&hours.clone(), &Py::Int(52)));
    return { let mut __s = String::new(); __s.push_str(&fstr(&_money(&[clean.clone(), value.clone()]), -1, "")); __s.push_str(" a year at "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is about "); __s.push_str(&fstr(&_money(&[clean.clone(), per_hour.clone()]), -1, "")); __s.push_str(" an hour before tax."); py_string(__s) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("hourly rate"))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("salary"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("annual"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_parse(&[clean.clone()]))) } } } }) {
    return py_str("Divide your yearly salary by the hours you work in a year. At 40 hours a week that is 2,080 hours, so £30,000 is about £14.42 an hour.");
    }
    return Py::None;
    Py::None
}
pub fn _travel(args: &[Py]) -> Py {
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
    let mut distance: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut speed: Py = Py::None;
    match_ = stdlib_call("re", "search", &[py_str("([\\d,.]+)\\s*(miles?|km|kilometres?|kilometers?)\\b.*?(?:at|of)\\s*([\\d,.]+)\\s*(?:miles?|km|kilometres?|kilometers?)?\\s*(?:an hour|per hour|/hour|mph|kmh|km/h)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    distance = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    speed = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(3)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&speed.clone()))) {
    return Py::None;
    }
    hours = op_truediv(&distance.clone(), &speed.clone());
    if py_truthy(&op_lt(&hours.clone(), &Py::Int(1))) {
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_number(&[op_mul(&hours.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" minutes at "); __s.push_str(&fstr(&_number(&[speed.clone()]), -1, "")); __s.push_str(" miles an hour."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_number(&[hours.clone()]), -1, "")); __s.push_str(" hours at "); __s.push_str(&fstr(&_number(&[speed.clone()]), -1, "")); __s.push_str(" miles an hour."); py_string(__s) };
    Py::None
}
pub fn _distance_in_miles(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut units: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The distance in the line as miles. \"half a mile\" and \"a mile and a half\" are both said.");
    word = mcall(&match_.clone(), "group", &[py_str("num")]);
    units = mcall(&match_.clone(), "group", &[py_str("unit")]);
    if py_truthy(&mcall(&g__AFTER_HALF(), "search", &[clean.clone()])) {
    value = Py::Float(0.5);
    } else {
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("half")))) {
    value = Py::Float(0.5);
    } else {
    value = if py_truthy(&mcall(&geti(&word.clone(), &Py::Int(0)), "isdigit", &[])) { call_builtin("float", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("float", &[mcall(&g__COUNTS(), "get", &[word.clone(), Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e)) };
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\band a half\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&value.clone(), &Py::Float(0.5));
    value = __aug1;
    }
    if py_truthy(&mcall(&units.clone(), "startswith", &[py_tuple(vec![py_str("km"), py_str("kilomet")])])) {
    return op_mul(&value.clone(), &Py::Float(0.621371));
    }
    if py_truthy(&mcall(&units.clone(), "startswith", &[py_str("met")])) {
    return op_truediv(&value.clone(), &Py::Float(1609.34));
    }
    return value.clone();
    Py::None
}
pub fn _duration(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut hours: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "hours") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: hours"))) };
    let mut hour_word: Py = Py::None;
    let mut hours_: Py = Py::None;
    let mut mins: Py = Py::None;
    let mut minutes: Py = Py::None;
    let _ = py_str("An hour and a bit the way a person says it: 75 minutes, or two hours 15 minutes.");
    minutes = call_builtin("int", &[call_builtin("round", &[op_mul(&hours.clone(), &Py::Int(60))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&minutes.clone(), &Py::Int(60))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&minutes.clone(), -1, "")); __s.push_str(" minutes"); py_string(__s) };
    }
    let __unpk1 = py_unpack(&call_builtin("divmod", &[minutes.clone(), Py::Int(60)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    hours_ = __unpk1[0].clone();
    mins = __unpk1[1].clone();
    if py_truthy(&op_le(&mins.clone(), &Py::Int(2))) {
    return if py_truthy(&Py::Bool(py_eq(&hours_.clone(), &Py::Int(1)))) { py_str("an hour") } else { { let mut __s = String::new(); __s.push_str(&fstr(&hours_.clone(), -1, "")); __s.push_str(" hours"); py_string(__s) } };
    }
    if py_truthy(&op_ge(&mins.clone(), &Py::Int(58))) {
    let __aug2 = op_add(&hours_.clone(), &Py::Int(1));
    hours_ = __aug2;
    return if py_truthy(&Py::Bool(py_eq(&hours_.clone(), &Py::Int(1)))) { py_str("an hour") } else { { let mut __s = String::new(); __s.push_str(&fstr(&hours_.clone(), -1, "")); __s.push_str(" hours"); py_string(__s) } };
    }
    hour_word = if py_truthy(&Py::Bool(py_eq(&hours_.clone(), &Py::Int(1)))) { py_str("an hour") } else { { let mut __s = String::new(); __s.push_str(&fstr(&hours_.clone(), -1, "")); __s.push_str(" hours"); py_string(__s) } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&hour_word.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mins.clone(), -1, "")); __s.push_str(" minutes"); py_string(__s) };
    Py::None
}
pub fn _the_size_of_the_trip(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut minutes_there_and_back: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes_there_and_back") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes_there_and_back"))) };
    let _ = py_str("What a friend says about a trip of that length - the verdict, not the arithmetic.");
    if py_truthy(&op_le(&minutes_there_and_back.clone(), &Py::Int(45))) {
    return py_str("That is a short trip - easy to do every day.");
    }
    if py_truthy(&op_le(&minutes_there_and_back.clone(), &Py::Int(100))) {
    return py_str("That is a real part of the day, but plenty of people do it.");
    }
    return py_str("That is a long way to do every day - fine now and then, hard as a routine.");
    Py::None
}
pub fn _the_way_it_is_done(args: &[Py]) -> Py {
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
    else { Py::Int(4) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the person said on the last few turns, newest first: the way they mean to travel.\n\n    Only the person's own lines count. The bot's answer names every way it can think of - the\n    running plan says \"one minute of easy jogging, two minutes of walking\" - and a distance said\n    after it would be timed at the bot's walking pace rather than the running the person asked\n    about. Newest first, because the last way named is the one the chat is on.\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&back.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return said.clone();
    Py::None
}
pub fn _at_a_pace(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut a_verdict: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_verdict") { v.clone() }
    else { Py::Bool(false) };
    let mut asked_far: Py = Py::None;
    let mut asked_time: Py = Py::None;
    let mut bike: Py = Py::None;
    let mut car: Py = Py::None;
    let mut each_way: Py = Py::None;
    let mut far_said: Py = Py::None;
    let mut far_tail: Py = Py::None;
    let mut hour: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut miles: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut pace: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut per_mile: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut speed: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut tail_said: Py = Py::None;
    let _ = py_str("How long a walk, a ride or a run takes when the line gives the distance, not the speed.");
    asked_time = stdlib_call("re", "search", &[py_str("\\bhow long\\b|\\bhow many (?:minutes|hours)\\b|\\bhow much longer\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    asked_far = stdlib_call("re", "search", &[py_str("\\bhow far\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    stated = { let __b = call_builtin("bool", &[mcall(&g__A_DISTANCE_STATED(), "match", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = stdlib_call("re", "search", &[g__A_PACE_WORD(), op_add(&op_add(&clean.clone(), &py_str(" ")), &mcall(&py_str(" "), "join", &[_the_way_it_is_done(&[memory.clone()])]))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g__TO_WORK(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = asked_time.clone(); if py_truthy(&__b) { __b } else { { let __b = asked_far.clone(); if py_truthy(&__b) { __b } else { stated.clone() } } } }))) {
    if py_truthy(&mcall(&g__IS_THAT_TOO_MUCH(), "search", &[clean.clone()])) {
    said_before = _the_way_it_is_done(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&said_before.clone());
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&mcall(&g__A_DISTANCE_STATED(), "match", &[mcall(&line.clone(), "strip", &[])])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&line.clone(), "strip", &[])); __args.push(memory.clone()); __kws.extend(vec![("a_verdict".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _at_a_pace(args))(&__args) };
    }
    }
    }
    return Py::None;
    }
    pace = Py::None;
    let __iter2 = py_iter_or_raise(&g__PACES());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 7, None);
    pattern = __unpk3[0].clone();
    speed = __unpk3[1].clone();
    said = __unpk3[2].clone();
    per_mile = __unpk3[3].clone();
    far_said = __unpk3[4].clone();
    tail = __unpk3[5].clone();
    far_tail = __unpk3[6].clone();
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    pace = py_tuple(vec![speed.clone(), said.clone(), per_mile.clone(), far_said.clone(), tail.clone(), far_tail.clone()]);
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pace.clone())); if !py_truthy(&__b) { __b } else { { let __b = stated.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__TO_WORK(), "search", &[clean.clone()]) } } } }) {
    match_ = mcall(&g__DISTANCE(), "search", &[clean.clone()]);
    miles = if py_truthy(&match_.clone()) { _distance_in_miles(&[match_.clone(), clean.clone()]) } else { Py::Int(0) };
    if py_truthy(&{ let __b = miles.clone(); if !py_truthy(&__b) { __b } else { op_gt(&miles.clone(), &Py::Int(0)) } }) {
    each_way = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\beach way\\b|\\bboth ways\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    car = op_mul(&op_truediv(&miles.clone(), &Py::Float(50.0)), &Py::Int(60));
    bike = op_mul(&op_truediv(&miles.clone(), &Py::Float(12.0)), &Py::Int(60));
    if py_truthy(&each_way.clone()) {
    return op_add(&{ let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_duration(&[op_truediv(&car.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" in the car and "); __s.push_str(&fstr(&_duration(&[op_truediv(&bike.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" on a bike, each way - so around "); __s.push_str(&fstr(&_duration(&[op_truediv(&op_mul(&Py::Int(2), &car.clone()), &Py::Int(60))]), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&_duration(&[op_truediv(&op_mul(&Py::Int(2), &bike.clone()), &Py::Int(60))]), -1, "")); __s.push_str(" there and back. "); py_string(__s) }, &if py_truthy(&a_verdict.clone()) { _the_size_of_the_trip(&[op_mul(&Py::Int(2), &car.clone())]) } else { py_str("A ride that short is well within reach, and it is the cheaper of the two.") });
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_duration(&[op_truediv(&car.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" in the car and "); __s.push_str(&fstr(&_duration(&[op_truediv(&bike.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" on a bike - the ride is the one that saves the money at that distance."); py_string(__s) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pace.clone())); if !py_truthy(&__b) { __b } else { stated.clone() } }) {
    let __iter4 = py_iter_or_raise(&_the_way_it_is_done(&[memory.clone()]));
    for __item in __iter4 {
    line = __item.clone();
    let __iter5 = py_iter_or_raise(&g__PACES());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 7, None);
    pattern = __unpk6[0].clone();
    speed = __unpk6[1].clone();
    said = __unpk6[2].clone();
    per_mile = __unpk6[3].clone();
    far_said = __unpk6[4].clone();
    tail = __unpk6[5].clone();
    far_tail = __unpk6[6].clone();
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    pace = py_tuple(vec![speed.clone(), said.clone(), per_mile.clone(), far_said.clone(), tail.clone(), far_tail.clone()]);
    break;
    }
    }
    if py_truthy(&pace.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&pace.clone()))) {
    return Py::None;
    }
    let __unpk7 = py_unpack(&pace.clone(), 6, None);
    speed = __unpk7[0].clone();
    said = __unpk7[1].clone();
    per_mile = __unpk7[2].clone();
    far_said = __unpk7[3].clone();
    tail = __unpk7[4].clone();
    far_tail = __unpk7[5].clone();
    match_ = mcall(&g__DISTANCE(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    miles = _distance_in_miles(&[match_.clone(), clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&miles.clone())); if py_truthy(&__b) { __b } else { op_le(&miles.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    minutes = op_mul(&op_truediv(&miles.clone(), &speed.clone()), &Py::Int(60));
    if py_truthy(&{ let __b = asked_far.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asked_time.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&stated.clone()) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\beach way\\b|\\bboth ways\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    tail_said = if py_truthy(&a_verdict.clone()) { op_add(&py_str(" "), &_the_size_of_the_trip(&[op_mul(&Py::Int(2), &minutes.clone())])) } else { py_str("") };
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_duration(&[op_truediv(&minutes.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", so roughly "); __s.push_str(&fstr(&_duration(&[op_truediv(&op_mul(&Py::Int(2), &minutes.clone()), &Py::Int(60))]), -1, "")); __s.push_str(" there and back - "); __s.push_str(&fstr(&speed.clone(), -1, &"g")); __s.push_str(" miles an hour, so "); __s.push_str(&fstr(&per_mile.clone(), -1, "")); __s.push_str("."); __s.push_str(&fstr(&tail_said.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_duration(&[op_truediv(&minutes.clone(), &Py::Int(60))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" - "); __s.push_str(&fstr(&speed.clone(), -1, &"g")); __s.push_str(" miles an hour, so "); __s.push_str(&fstr(&per_mile.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_duration(&[op_truediv(&miles.clone(), &speed.clone())]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" - "); __s.push_str(&fstr(&speed.clone(), -1, &"g")); __s.push_str(" miles an hour, so "); __s.push_str(&fstr(&per_mile.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) };
    }
    hour = stdlib_call("re", "search", &[py_str("\\b(?:in|within|for)\\s+(?:an?|one|1)\\s+hour\\b|\\ban hour\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = hour.clone(); if !py_truthy(&__b) { __b } else { asked_far.clone() } }) {
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&speed.clone(), -1, &"g")); __s.push_str(" miles in an hour "); __s.push_str(&fstr(&far_said.clone(), -1, "")); __s.push_str(" - "); __s.push_str(&fstr(&per_mile.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&far_tail.clone(), -1, "")); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _convert(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut fact: Py = Py::None;
    let mut table: Py = Py::None;
    let _ = py_str("A unit conversion, from the one table of units (bot/knowledge/units.py).\n\n    A plain pair of units may already have a hand-written fact in _unit_fact() with a\n    second useful measure in it (\"16 ounces in a pound, and 14 pounds in a stone\"), and\n    that wording is kept. Everything else - a named food's cups and grams, an oven heat,\n    every pair nobody wrote a fact for - comes from the table.\n    ");
    fact = _unit_fact(&[clean.clone()]);
    table = pyrt::xrefs::xcall("knowledge.units", "convert", &[clean.clone(), memory.clone()]);
    if py_truthy(&{ let __b = fact.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.units", "answers_more_than_a_pair", &[clean.clone()]))) } }) {
    return fact.clone();
    }
    return { let __b = table.clone(); if py_truthy(&__b) { __b } else { fact.clone() } };
    Py::None
}
pub fn _rate_clause(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut end: Py = Py::None;
    let mut part: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The part of the line the rate sits in, so its direction can be read on its own.");
    start = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE_SPLIT(), "split", &[line.clone()]));
    for __item in __iter1 {
    part = __item.clone();
    end = op_add(&start.clone(), &call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&{ let __b = op_le(&start.clone(), &mcall(&match_.clone(), "start", &[])); if !py_truthy(&__b) { __b } else { op_le(&mcall(&match_.clone(), "end", &[]), &op_add(&end.clone(), &Py::Int(1))) } }) {
    return part.clone();
    }
    start = op_add(&end.clone(), &Py::Int(1));
    }
    return line.clone();
    Py::None
}
pub fn _user_lines(args: &[Py]) -> Py {
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
    else { Py::Int(8) };
    let mut lines: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the user said on the last few turns, oldest first.");
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&turn.clone(), "get", &[py_str("user")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&text.clone(), "strip", &[]) } }) {
    let _ = mcall(&lines.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])]);
    }
    }
    return lines.clone();
    Py::None
}
pub fn _a_rate_was_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut clause: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("True when one of these lines puts money by at a rate (\"put away 250 a month\").");
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_RATE(), "finditer", &[line.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    clause = _rate_clause(&[line.clone(), match_.clone()]);
    if py_truthy(&{ let __b = mcall(&g_SAVING_CLAUSE(), "search", &[clause.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SPENDING_CLAUSE(), "search", &[clause.clone()]))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _rate_in_chat(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut put_right: Py = Py::None;
    let _ = py_str("The most recent rate per period in the chat, when the money is going towards something.\n\n    A rate put right counts as the most recent one. \"i can put away 250 a month\" and then\n    \"actually i can do 300 now ive paid the car off\" is one rate written twice, and the second\n    line has no \"a month\" on it because the month never changed - so it matched nothing here, and\n    the length of the saving was worked out at the rate the person had already left behind.\n    ");
    lines = _user_lines(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    line = geti(&lines.clone(), &index.clone());
    put_right = pyrt::xrefs::xcall("memory.saving", "a_rate_put_right", &[line.clone(), if py_truthy(&index.clone()) { geti(&lines.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") }]);
    if py_truthy(&{ let __b = put_right.clone(); if !py_truthy(&__b) { __b } else { _a_rate_was_said(&[slice_or_raise(&lines.clone(), &Py::None, &index.clone(), &Py::None)]) } }) {
    return py_tuple(vec![call_builtin("float", &[put_right.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("month")]);
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_RATE(), "finditer", &[line.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    clause = _rate_clause(&[line.clone(), match_.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_SAVING_CLAUSE(), "search", &[clause.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_SPENDING_CLAUSE(), "search", &[clause.clone()]) } }) {
    continue;
    }
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[py_str("amount")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&amount.clone(), &Py::Int(0))) {
    return py_tuple(vec![amount.clone(), mcall(&match_.clone(), "group", &[py_str("period")])]);
    }
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _worked_in_chat(args: &[Py]) -> Py {
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
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut part: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut text: Py = Py::None;
    let mut total: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The total this chat has already worked a time for, read from the bot's own answer.\n\n    \"we can put away 400 a month\" is answered \"£20,000 at £400 a month is 50 months\", so the\n    \"how long would that take\" that follows is a question about the 20,000 the chat itself\n    divided - and a total named beside a rate is the plan's own figure, not an example or an\n    aim. Only a reply that did the division counts: the one before it says what to aim for and\n    no rate, so its figure is never taken as the plan.\n    ");
    total = Py::None;
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&turn.clone(), "get", &[py_str("bot")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_WORKED(), "findall", &[mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])])]));
    for __item in __iter2 {
    found = __item.clone();
    raw = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    let __try3 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&raw.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { op_lt(&value.clone(), &op_mul(&rate.clone(), &Py::Int(2))) } }) {
    continue;
    }
    if py_truthy(&op_ge(&value.clone(), &Py::Int(50))) {
    total = value.clone();
    }
    }
    }
    return total.clone();
    Py::None
}
pub fn _sums(args: &[Py]) -> Py {
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
    let _ = py_str("The sums in a line, with a scale word read as thousands: \"20 grand\" is 20,000.");
    text = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&g__WORD_THOUSANDS(), "sub", &[{ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); call_builtin("str", &[op_mul(&geti(&g__TENS_WORDS(), &mcall(&mcall(&m.clone(), "group", &[Py::Int(1)]), "lower", &[])), &Py::Int(1000))]).unwrap_or_else(|e| exc::raise(e)) }) }, text.clone()]);
    text = mcall(&g__NUMBER_THOUSANDS(), "sub", &[{ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); call_builtin("str", &[call_builtin("int", &[op_mul(&call_builtin("float", &[mcall(&m.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1000))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) }) }, text.clone()]);
    if py_truthy(&mcall(&g__MONEY_HERE(), "search", &[text.clone()])) {
    text = mcall(&g__K_THOUSANDS_HERE(), "sub", &[{ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); call_builtin("str", &[call_builtin("int", &[op_mul(&call_builtin("float", &[mcall(&m.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1000))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) }) }, text.clone()]);
    }
    return _parse(&[text.clone()]);
    Py::None
}
pub fn _goal_in_chat(args: &[Py]) -> Py {
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
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { Py::None };
    let mut a_goal_is_named: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut owed: Py = Py::None;
    let mut total: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The biggest figure earlier in the chat that is a goal rather than a bill or a wage.");
    lines = _user_lines(&[memory.clone()]);
    total = Py::None;
    a_goal_is_named = { let __b = Py::Bool(!py_is(&rate.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&g_GOAL(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    owed = Py::None;
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_GOAL(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_goal_is_named.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_GOAL(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { mcall(&g_SPENDING_CLAUSE(), "search", &[line.clone()]) } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.saving", "the_pot_said_alone", &[line.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_OWED(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DEBT_CLEARED(), "search", &[line.clone()]))) } }) {
    let __iter2 = py_iter_or_raise(&_sums(&[mcall(&g_SAID_A_RATE(), "sub", &[py_str(" "), line.clone()])]));
    for __item in __iter2 {
    value = __item.clone();
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { op_lt(&value.clone(), &op_mul(&rate.clone(), &Py::Int(2))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_ge(&value.clone(), &Py::Int(50)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&owed.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&value.clone(), &owed.clone()) } } } }) {
    owed = value.clone();
    }
    }
    continue;
    }
    let __iter3 = py_iter_or_raise(&_sums(&[mcall(&g_SAID_A_RATE(), "sub", &[py_str(" "), line.clone()])]));
    for __item in __iter3 {
    value = __item.clone();
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { op_lt(&value.clone(), &op_mul(&rate.clone(), &Py::Int(2))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_ge(&value.clone(), &Py::Int(50)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&total.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&value.clone(), &total.clone()) } } } }) {
    total = value.clone();
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&total.clone(), &Py::None))) {
    total = _worked_in_chat(&[memory.clone(), rate.clone()]);
    }
    return if py_truthy(&Py::Bool(!py_is(&total.clone(), &Py::None))) { total.clone() } else { owed.clone() };
    Py::None
}
pub fn _rate_and_total(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A rate per period and a total to reach, both stated in the chat, or (None, None, None).");
    let __unpk1 = py_unpack(&_rate_in_chat(&[memory.clone()]), 2, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&rate.clone(), &Py::None))) {
    return py_tuple(vec![Py::None, Py::None, Py::None]);
    }
    total = _goal_in_chat(&[memory.clone(), rate.clone()]);
    held = pyrt::xrefs::xcall("memory.saving", "the_money_already_held", &[memory.clone()]);
    if py_truthy(&{ let __b = total.clone(); if !py_truthy(&__b) { __b } else { { let __cmp21: Py = held.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp21)) { op_lt(&__cmp21, &total.clone()) } else { Py::Bool(false) } } } }) {
    total = op_sub(&total.clone(), &held.clone());
    }
    return py_tuple(vec![rate.clone(), period.clone(), total.clone()]);
    Py::None
}
pub fn _in_months(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut period: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let _ = py_str("A count of one period said in months, for the parenthesis in the reply.");
    return op_mul(&count.clone(), &geti(&g_PERIODS(), &period.clone()));
    Py::None
}
pub fn _duration_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut period: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut total: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "total") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: total"))) };
    let mut about: Py = Py::None;
    let mut count: Py = Py::None;
    let mut loose: Py = Py::None;
    let mut months: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nearest: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_count: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("The sentence for \"X a month for Y\": the count, and how long that is in years.");
    count = op_truediv(&total.clone(), &rate.clone());
    if py_truthy(&{ let __b = op_lt(&count.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&count.clone(), &Py::Int(1200)) } }) {
    return Py::None;
    }
    nearest = call_builtin("int", &[op_add(&count.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &nearest.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.05))) {
    let __unpk1 = py_unpack(&py_tuple(vec![_number(&[nearest.clone()]), py_str("")]), 2, None);
    said_count = __unpk1[0].clone();
    about = __unpk1[1].clone();
    } else {
    let __unpk2 = py_unpack(&py_tuple(vec![_number(&[if py_truthy(&op_lt(&count.clone(), &Py::Int(3))) { call_builtin("round", &[count.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)) } else { nearest.clone() }]), py_str("about ")]), 2, None);
    said_count = __unpk2[0].clone();
    about = __unpk2[1].clone();
    }
    name = if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { period.clone() } else { op_add(&period.clone(), &py_str("s")) };
    reply = { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&_number(&[rate.clone()]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&period.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" takes "); __s.push_str(&fstr(&about.clone(), -1, "")); __s.push_str(&fstr(&said_count.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) };
    months = _in_months(&[count.clone(), period.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&period.clone(), &py_str("year"))); if !py_truthy(&__b) { __b } else { op_ge(&months.clone(), &Py::Int(12)) } }) {
    let __unpk3 = py_unpack(&call_builtin("divmod", &[call_builtin("int", &[call_builtin("round", &[months.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), Py::Int(12)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    years = __unpk3[0].clone();
    rest = __unpk3[1].clone();
    said = op_add(&{ let mut __s = String::new(); __s.push_str(&fstr(&years.clone(), -1, "")); __s.push_str(" year"); py_string(__s) }, &if py_truthy(&Py::Bool(py_eq(&years.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") });
    if py_truthy(&rest.clone()) {
    let __aug4 = op_add(&said.clone(), &op_add(&{ let mut __s = String::new(); __s.push_str(" and "); __s.push_str(&fstr(&rest.clone(), -1, "")); __s.push_str(" month"); py_string(__s) }, &if py_truthy(&Py::Bool(py_eq(&rest.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }));
    said = __aug4;
    }
    loose = if py_truthy(&{ let __b = about.clone(); if py_truthy(&__b) { __b } else { op_ge(&call_builtin("abs", &[op_sub(&months.clone(), &call_builtin("round", &[months.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.05)) } }) { py_str("about ") } else { py_str("") };
    let __aug5 = op_add(&reply.clone(), &{ let mut __s = String::new(); __s.push_str(" ("); __s.push_str(&fstr(&loose.clone(), -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(")"); py_string(__s) });
    reply = __aug5;
    }
    return op_add(&reply.clone(), &py_str("."));
    Py::None
}
pub fn _time_to_reach(args: &[Py]) -> Py {
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
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("How long it takes to reach a total at a rate, when the chat said both and the line asks.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_TIME_ASKED(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_parse(&[clean.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_rate_and_total(&[memory.clone()]), 3, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    total = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&total.clone())) } }) {
    return Py::None;
    }
    return _duration_reply(&[rate.clone(), period.clone(), total.clone()]);
    Py::None
}
pub fn _asks_a_new_sum(args: &[Py]) -> Py {
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
    let _ = py_str("A line that proposes its own numbers for the sum the chat already holds.");
    if py_truthy(&mcall(&g_WHAT_IF(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_ABOUT_ASK(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CLOCK(), "search", &[clean.clone()]))) } };
    Py::None
}
pub fn _last_takes(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The rate and period the bot worked out on the turn before, when it worked one out.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    found = mcall(&g_TAKES(), "search", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[py_str("amount")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), mcall(&mcall(&found.clone(), "group", &[py_str("period")]), "rstrip", &[py_str("s")])]);
    Py::None
}
pub fn _time_at_a_new_rate(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A rate the line itself proposes for the same total: \"what if we do 700?\"\n\n    The question is implied, the way people ask it straight after an answer: the chat holds the\n    total, the bot has just said how long it takes, and this line changes the rate.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_asks_a_new_sum(&[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    before = _last_takes(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&before.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_rate_and_total(&[memory.clone()]), 3, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    total = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return Py::None;
    }
    match_ = mcall(&g_RATE(), "search", &[clean.clone()]);
    numbers = _parse(&[clean.clone()]);
    if py_truthy(&match_.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[py_str("amount")]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), mcall(&match_.clone(), "group", &[py_str("period")])]), 2, None);
    rate = __unpk2[0].clone();
    period = __unpk2[1].clone();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { op_lt(&geti(&numbers.clone(), &Py::Int(0)), &total.clone()) } }) {
    if py_truthy(&mcall(&g_A_HORIZON(), "search", &[clean.clone()])) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&numbers.clone(), &Py::Int(0)), geti(&before.clone(), &Py::Int(1))]), 2, None);
    rate = __unpk3[0].clone();
    period = __unpk3[1].clone();
    } else {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&period.clone())) } }) {
    return Py::None;
    }
    return _duration_reply(&[rate.clone(), period.clone(), total.clone()]);
    Py::None
}
pub fn _time_at_a_new_total(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut count: Py = Py::None;
    let mut dearer: Py = Py::None;
    let mut extra: Py = Py::None;
    let mut more: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nearest: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut old: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A bigger price for the same saving rate: \"and what if i find one for 6000, is that daft?\"\n\n    Same shape as the rate change, the other way round: the person keeps the money they can put\n    away and asks about a dearer thing, which is a question about how much longer it takes.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_asks_a_new_sum(&[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    before = _last_takes(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&before.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_rate_and_total(&[memory.clone()]), 3, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    total = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&total.clone())) } }) {
    return Py::None;
    }
    numbers = _parse(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { op_le(&geti(&numbers.clone(), &Py::Int(0)), &total.clone()) } }) {
    return Py::None;
    }
    dearer = geti(&numbers.clone(), &Py::Int(0));
    count = op_truediv(&dearer.clone(), &rate.clone());
    old = op_truediv(&total.clone(), &rate.clone());
    nearest = call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = _number(&[if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &nearest.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.05))) { nearest.clone() } else { call_builtin("round", &[count.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)) }]);
    name = if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { period.clone() } else { op_add(&period.clone(), &py_str("s")) };
    extra = call_builtin("round", &[op_sub(&count.clone(), &old.clone())]).unwrap_or_else(|e| exc::raise(e));
    more = if py_truthy(&op_ge(&extra.clone(), &Py::Int(1))) { { let mut __s = String::new(); __s.push_str(", so "); __s.push_str(&fstr(&_number(&[extra.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" more than "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); py_string(__s) } } else { py_str("") };
    return { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&_number(&[rate.clone()]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&period.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&_number(&[dearer.clone()]), -1, "")); __s.push_str(" takes "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(&fstr(&more.clone(), -1, "")); __s.push_str(". Only you know if the dearer one is worth the wait."); py_string(__s) };
    Py::None
}
pub fn _time_without_a_goal(args: &[Py]) -> Py {
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
    let mut each_hundred: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut yearly: Py = Py::None;
    let _ = py_str("A rate without a figure to reach yet: \"how long would that take?\"\n\n    There is nothing to divide, so the answer says what the rate comes to in a year, what one\n    hundred of the price costs in weeks at that rate, and asks for the one number that is missing.\n    Better than a shrug, and it is the question they will ask next: \"i put 20 by every week\n    towards a bike\" then \"so how many weeks until i can buy it\" was a search hedge, when 20 a\n    week is five weeks for every 100 of the bike.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_TIME_ASKED(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _parse(&[clean.clone()]) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_rate_in_chat(&[memory.clone()]), 2, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&rate.clone()))) {
    return Py::None;
    }
    if py_truthy(&_goal_in_chat(&[memory.clone(), rate.clone()])) {
    return Py::None;
    }
    yearly = op_truediv(&op_mul(&rate.clone(), &Py::Int(12)), &geti(&g_PERIODS(), &period.clone()));
    said = { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&_number(&[rate.clone()]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&period.clone(), -1, "")); __s.push_str(" that's about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[yearly.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" a year"); py_string(__s) };
    each_hundred = _each_hundred(&[rate.clone(), period.clone()]);
    if py_truthy(&each_hundred.clone()) {
    let __aug2 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(", so every 100 of the price takes "); __s.push_str(&fstr(&each_hundred.clone(), -1, "")); py_string(__s) });
    said = __aug2;
    }
    return op_add(&said.clone(), &py_str(". Tell me the figure you are saving towards and I will work out how long it takes."));
    Py::None
}
pub fn _each_hundred(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rate: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut period: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut count: Py = Py::None;
    let mut many: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("How long 100 of the price takes at a rate: \"five weeks\", \"eight days\", \"2.5 months\".\n\n    The figure that makes an empty sum useful without inventing a price: it is the rate read the\n    other way round, and a price can then be done in the head in steps of one hundred. The unit\n    follows the count down, so a rate of 300 a month answers in days rather than in \"a third of a\n    month\", and the count is rounded to the half that a person would say.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { op_le(&rate.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    count = op_truediv(&Py::Float(100.0), &rate.clone());
    while py_truthy(&{ let __b = op_lt(&count.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__SMALLER_PERIOD(), &period.clone())) } }) {
    let __unpk1 = py_unpack(&geti(&g__SMALLER_PERIOD(), &period.clone()), 2, None);
    period = __unpk1[0].clone();
    many = __unpk1[1].clone();
    let __aug2 = op_mul(&count.clone(), &many.clone());
    count = __aug2;
    }
    if py_truthy(&op_lt(&count.clone(), &Py::Int(1))) {
    return Py::None;
    }
    while py_truthy(&{ let __b = op_ge(&count.clone(), &Py::Int(13)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("day"), py_str("week"), py_str("fortnight")]), &period.clone())) } }) {
    let __unpk3 = py_unpack(&geti(&g__SMALLER_PERIOD(), &period.clone()), 2, None);
    period = __unpk3[0].clone();
    many = __unpk3[1].clone();
    let __aug4 = op_truediv(&count.clone(), &many.clone());
    count = __aug4;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&period.clone(), &py_str("hour"))); if !py_truthy(&__b) { __b } else { op_ge(&count.clone(), &Py::Int(12)) } }) {
    return Py::None;
    }
    if py_truthy(&op_lt(&count.clone(), &Py::Int(3))) {
    value = op_truediv(&call_builtin("round", &[op_mul(&count.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    said = _number(&[if py_truthy(&Py::Bool(py_eq(&value.clone(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))))) { call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { value.clone() }]);
    } else {
    value = call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = _number(&[call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&op_ge(&call_builtin("abs", &[op_sub(&count.clone(), &value.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.5))) {
    said = { let mut __s = String::new(); __s.push_str("about "); __s.push_str(&fstr(&said.clone(), -1, "")); py_string(__s) };
    }
    }
    name = if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.05))) { period.clone() } else { op_add(&period.clone(), &py_str("s")) };
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _a_saving_goal_in_chat(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut line: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The biggest figure on a line that says money is being put by.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&_user_lines(&[memory.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_MONEY_GOAL(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_SPENDING_CLAUSE(), "search", &[line.clone()]) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&_sums(&[mcall(&g_SAID_A_RATE(), "sub", &[py_str(" "), line.clone()])]));
    for __item in __iter2 {
    value = __item.clone();
    if py_truthy(&{ let __b = op_ge(&value.clone(), &Py::Int(50)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&value.clone(), &best.clone()) } } } }) {
    best = value.clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _the_plan_figure(args: &[Py]) -> Py {
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
    let mut newest: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut sums: Py = Py::None;
    let mut user: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The figure the **newest** plan names, if that plan is about money.\n\n    \"im trying to save 2000 for a used car\" then \"actually its 3000\" leaves the plan reading\n    \"Save 3000 for a used car\" (bot/memory/topics.py writes the correction), while the old sum is\n    still on the first line of the chat. Reading the lines alone answered a later ask with 2,000.\n\n    Only the newest plan, because a later plan is a new subject: \"i want to run 100 miles\" after the\n    car leaves the store holding both, and the goal in hand is the run. A plan with no money word in\n    it is not money at all, so this is also the fence that keeps 100 miles from being read as £100.\n    ");
    user = { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("list", &[{ let __b = mcall(&user.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) { let mut plan: Py = __item.clone(); if py_truthy(&plan.clone()) {  __out.push(plan.clone()); } } py_list(__out) })() };
    newest = if py_truthy(&plans.clone()) { geti(&plans.clone(), &op_neg(&Py::Int(1))) } else { mcall(&user.clone(), "get", &[py_str("plan")]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&newest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MONEY_GOAL(), "search", &[mcall(&call_builtin("str", &[newest.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]))) } }) {
    return Py::None;
    }
    sums = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_sums(&[call_builtin("str", &[newest.clone()]).unwrap_or_else(|e| exc::raise(e))])) { let mut value: Py = __item.clone(); if py_truthy(&op_ge(&value.clone(), &Py::Int(50))) {  __out.push(value.clone()); } } py_list(__out) })() };
    return if py_truthy(&sums.clone()) { call_builtin("max", &[sums.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    Py::None
}
pub fn _a_goal_without_a_rate(args: &[Py]) -> Py {
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
    let mut _period: Py = Py::None;
    let mut _time_to_save: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut total: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("A figure to reach but no rate to reach it at: \"im trying to save 2000 for a used car\" then\n    \"how long will it take\".\n\n    _time_without_a_goal is the same sum the other way round: with a rate and no figure, the answer\n    says what the rate comes to in a year and asks for the figure. With a figure and no rate there\n    is nothing to divide either, and the missing half is the one a friend asks for. Nothing said it,\n    so the question was answered from the card for the thing being saved for - \"ask how long they've\n    owned it and why they're selling\" - which is a question to put to a stranger about the car,\n    not an answer about time.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_TIME_ASKED(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _parse(&[clean.clone()]) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_rate_in_chat(&[memory.clone()]), 2, None);
    rate = __unpk1[0].clone();
    _period = __unpk1[1].clone();
    if py_truthy(&rate.clone()) {
    return Py::None;
    }
    total = _the_plan_figure(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&total.clone(), &Py::None))) {
    user = { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&user.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { mcall(&user.clone(), "get", &[py_str("plans")]) } } } }) {
    return Py::None;
    }
    total = _a_saving_goal_in_chat(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return Py::None;
    }
    _time_to_save = py_func("_time_to_save", |args: &[Py]| pyrt::xrefs::xcall("knowledge.money", "_time_to_save", args));
    return { let __b = py_call(&_time_to_save, &[memory.clone(), py_dict(vec![(py_str("clean"), clean.clone()), (py_str("text"), clean.clone())])]); if py_truthy(&__b) { __b } else { { let mut __s = String::new(); __s.push_str("Getting to "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" depends on what you can put by - "); __s.push_str(&fstr(&g_ASKED_FOR_RATE(), -1, "")); py_string(__s) } } };
    Py::None
}
pub fn _a_new_total_only(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line is a swap of the total: \"say i need 12000\".");
    return call_builtin("bool", &[mcall(&g_JUST_A_TOTAL(), "search", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _figure_just_given(args: &[Py]) -> Py {
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
    let mut numbers: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The total, given on the next line: \"say i need 12000\".\n\n    _time_without_a_goal asked for one number, and when it arrives the question it stood in for\n    (\"how long will that take\") is answered now that both numbers are in the chat. A chat that\n    worked a time for its own total (\"£20,000 at £200 a month is 100 months, about 8 years and\n    4 months\") is the same case without the asking: \"say i need 12000\" is that question again\n    about 12,000, and asking for a figure the chat is already holding would be the worse answer.\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    said = { let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &g_ASKED_FOR_FIGURE())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _a_new_total_only(&[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_WORKED(), "search", &[mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])])]) } })) } }) {
    return Py::None;
    }
    numbers = _parse(&[clean.clone()]);
    let __unpk1 = py_unpack(&_rate_in_chat(&[memory.clone()]), 2, None);
    rate = __unpk1[0].clone();
    period = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { op_lt(&geti(&numbers.clone(), &Py::Int(0)), &op_mul(&rate.clone(), &Py::Int(2))) } } } }) {
    return Py::None;
    }
    return _duration_reply(&[rate.clone(), period.clone(), geti(&numbers.clone(), &Py::Int(0))]);
    Py::None
}
pub fn _each_month_for_a_goal(args: &[Py]) -> Py {
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
    let mut dated: Py = Py::None;
    let mut per_year: Py = Py::None;
    let mut period: Py = Py::None;
    let mut total: Py = Py::None;
    let mut two: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("A figure to reach but no date: how much to put by each month.");
    bare = mcall(&g_A_BARE_RATE_ASK(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_HOW_MUCH_EACH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { bare.clone() } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _parse(&[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    total = _goal_in_chat(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_DEADLINE_NAMED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _a_deadline_in_chat(&[memory.clone()]) } }) {
    dated = _a_month_for_the_deadline_in_chat(&[total.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&dated.clone()))) {
    return Py::None;
    }
    return dated.clone();
    }
    period = _asked_period(&[clean.clone()]);
    per_year = geti(&g_PERIODS_IN_A_YEAR(), &period.clone());
    let __unpk1 = py_unpack(&py_tuple(vec![op_truediv(&total.clone(), &per_year.clone()), op_truediv(&total.clone(), &op_mul(&per_year.clone(), &Py::Int(2)))]), 2, None);
    year = __unpk1[0].clone();
    two = __unpk1[1].clone();
    return { let mut __s = String::new(); __s.push_str("Saving "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" over a year is about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[year.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&period.clone(), -1, "")); __s.push_str("; over two years, about "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[two.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&period.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _k_as_money(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The figure in a line that writes 10k, read as 10,000 when the line is about money.\n\n    Only used by the goal arithmetic below, so a race stays a race: \"how far is a 10k\" has no\n    goal word in it and is never read here.\n    ");
    found = mcall(&g_K_THOUSANDS(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SAVING_CUE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    amount = op_mul(&call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1000));
    return if py_truthy(&op_ge(&amount.clone(), &Py::Int(100))) { amount.clone() } else { Py::None };
    Py::None
}
pub fn _easter_day(args: &[Py]) -> Py {
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
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let mut c: Py = Py::None;
    let mut d: Py = Py::None;
    let mut day: Py = Py::None;
    let mut e: Py = Py::None;
    let mut f: Py = Py::None;
    let mut g: Py = Py::None;
    let mut h: Py = Py::None;
    let mut i: Py = Py::None;
    let mut k: Py = Py::None;
    let mut l: Py = Py::None;
    let mut m: Py = Py::None;
    let mut month: Py = Py::None;
    let _ = py_str("The date of Easter Sunday, so \"save 3000 by easter\" has a date to count back from.\n\n    The Gregorian rule in eight lines rather than a table: Easter is the first Sunday after the\n    first full moon on or after 21 March, and the arithmetic below is the standard way to get\n    that without a calendar lookup.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![op_modulo(&year.clone(), &Py::Int(19)), op_floordiv(&year.clone(), &Py::Int(100)), op_modulo(&year.clone(), &Py::Int(100))]), 3, None);
    a = __unpk1[0].clone();
    b = __unpk1[1].clone();
    c = __unpk1[2].clone();
    let __unpk2 = py_unpack(&py_tuple(vec![op_floordiv(&b.clone(), &Py::Int(4)), op_modulo(&b.clone(), &Py::Int(4))]), 2, None);
    d = __unpk2[0].clone();
    e = __unpk2[1].clone();
    f = op_floordiv(&op_add(&b.clone(), &Py::Int(8)), &Py::Int(25));
    g = op_floordiv(&op_add(&op_sub(&b.clone(), &f.clone()), &Py::Int(1)), &Py::Int(3));
    h = op_modulo(&op_add(&op_sub(&op_sub(&op_add(&op_mul(&Py::Int(19), &a.clone()), &b.clone()), &d.clone()), &g.clone()), &Py::Int(15)), &Py::Int(30));
    let __unpk3 = py_unpack(&py_tuple(vec![op_floordiv(&c.clone(), &Py::Int(4)), op_modulo(&c.clone(), &Py::Int(4))]), 2, None);
    i = __unpk3[0].clone();
    k = __unpk3[1].clone();
    l = op_modulo(&op_sub(&op_sub(&op_add(&op_add(&Py::Int(32), &op_mul(&Py::Int(2), &e.clone())), &op_mul(&Py::Int(2), &i.clone())), &h.clone()), &k.clone()), &Py::Int(7));
    m = op_floordiv(&op_add(&op_add(&a.clone(), &op_mul(&Py::Int(11), &h.clone())), &op_mul(&Py::Int(22), &l.clone())), &Py::Int(451));
    month = op_floordiv(&op_add(&op_sub(&op_add(&h.clone(), &l.clone()), &op_mul(&Py::Int(7), &m.clone())), &Py::Int(114)), &Py::Int(31));
    day = op_add(&op_modulo(&op_add(&op_sub(&op_add(&h.clone(), &l.clone()), &op_mul(&Py::Int(7), &m.clone())), &Py::Int(114)), &Py::Int(31)), &Py::Int(1));
    return py_tuple(vec![month.clone(), day.clone()]);
    Py::None
}
pub fn _the_last_day_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut month: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut calendar: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The day the month ends on: 31 for December, 30 for June, 28 or 29 for February.");
    calendar = stdlib_module("calendar");
    year = op_add(&gattr(&today.clone(), "year"), &if py_truthy(&op_lt(&month.clone(), &gattr(&today.clone(), "month"))) { Py::Int(1) } else { Py::Int(0) });
    return geti(&mcall(&calendar.clone(), "monthrange", &[year.clone(), month.clone()]), &Py::Int(1));
    Py::None
}
pub fn _a_named_deadline(args: &[Py]) -> Py {
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
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { Py::None };
    let mut a_day: Py = Py::None;
    let mut at_the_end: Py = Py::None;
    let mut calendar: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut when: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The date a line names for the money already in it: (\"christmas\", date) or None.\n\n    The months and days left are what a per-month figure is for, so a date that can be read\n    exactly is taken; a movable feast like Easter is left alone rather than guessed at.\n    ");
    datetime = stdlib_module("datetime");
    today = { let __b = today.clone(); if py_truthy(&__b) { __b } else { mcall(&gattr(&datetime.clone(), "date"), "today", &[]) } };
    found = mcall(&g_DEADLINE_NAMED(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    said = mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    name = geti(&mcall(&said.clone(), "split", &[]), &op_neg(&Py::Int(1)));
    a_day = mcall(&g_A_DEADLINE_DAY(), "search", &[said.clone()]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("the end of next month"), py_str("the end of this month"), py_str("the end of next week"), py_str("the end of this week"), py_str("the end of next year"), py_str("the end of this year"), py_str("the end of the year"), py_str("the end of the month"), py_str("the end of the week"), py_str("next summer"), py_str("next christmas"), py_str("next year"), py_str("next month"), py_str("next week"), py_str("the summer"), py_str("the winter")]));
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &phrase.clone()))) {
    name = phrase.clone();
    break;
    }
    }
    if py_truthy(&{ let __b = a_day.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MONTH_NUMBERS(), &mcall(&a_day.clone(), "group", &[Py::Int(2)]))) } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&g_MONTH_NUMBERS(), &mcall(&a_day.clone(), "group", &[Py::Int(2)])), call_builtin("int", &[mcall(&a_day.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    month = __unpk2[0].clone();
    day = __unpk2[1].clone();
    name = op_add(&py_str("the "), &mcall(&mcall(&a_day.clone(), "group", &[Py::Int(0)]), "lower", &[]));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp31: Py = day.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp31)) { op_le(&__cmp31, &Py::Int(31)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_MONTH_NUMBERS(), &name.clone()))) {
    month = geti(&g_MONTH_NUMBERS(), &name.clone());
    at_the_end = Py::Bool(py_contains(&op_add(&op_add(&said.clone(), &py_str(" ")), &slice_or_raise(&clean.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&found.clone(), "start", &[]), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&found.clone(), "start", &[]), &Py::None)), &py_str("end of")));
    day = if py_truthy(&at_the_end.clone()) { _the_last_day_of(&[month.clone(), today.clone()]) } else { Py::Int(1) };
    name = if py_truthy(&at_the_end.clone()) { op_add(&py_str("the end of "), &name.clone()) } else { name.clone() };
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("next summer"), py_str("next christmas")]), &name.clone()))) {
    let __unpk4 = py_unpack(&{ let __b = geti(&g_DEADLINE_DAYS(), &mcall(&name.clone(), "replace", &[py_str("next "), py_str("")])); if py_truthy(&__b) { __b } else { py_tuple(vec![Py::Int(7), Py::Int(1)]) } }, 2, None);
    month = __unpk4[0].clone();
    day = __unpk4[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("easter")))) {
    let __unpk5 = py_unpack(&_easter_day(&[gattr(&today.clone(), "year")]), 2, None);
    month = __unpk5[0].clone();
    day = __unpk5[1].clone();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DEADLINE_DAYS(), &name.clone())); if !py_truthy(&__b) { __b } else { geti(&g_DEADLINE_DAYS(), &name.clone()) } }) {
    let __unpk6 = py_unpack(&geti(&g_DEADLINE_DAYS(), &name.clone()), 2, None);
    month = __unpk6[0].clone();
    day = __unpk6[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("next month")))) {
    let __unpk7 = py_unpack(&py_tuple(vec![op_add(&gattr(&today.clone(), "year"), &Py::Bool(py_eq(&gattr(&today.clone(), "month"), &Py::Int(12)))), op_add(&op_modulo(&gattr(&today.clone(), "month"), &Py::Int(12)), &Py::Int(1))]), 2, None);
    year = __unpk7[0].clone();
    month = __unpk7[1].clone();
    return py_tuple(vec![name.clone(), mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), Py::Int(1)])]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("next week")))) {
    return py_tuple(vec![name.clone(), op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(7))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the end of the month")]), &name.clone()))) {
    return py_tuple(vec![name.clone(), mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), gattr(&today.clone(), "month"), _the_last_day_of(&[gattr(&today.clone(), "month"), today.clone()])])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the end of the week")]), &name.clone()))) {
    return py_tuple(vec![name.clone(), op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_sub(&Py::Int(7), &mcall(&today.clone(), "weekday", &[])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the end of next month"), py_str("the end of this month")]), &name.clone()))) {
    calendar = stdlib_module("calendar");
    step = if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str("next")))) { Py::Int(1) } else { Py::Int(0) };
    month = op_add(&gattr(&today.clone(), "month"), &step.clone());
    let __unpk8 = py_unpack(&py_tuple(vec![op_add(&gattr(&today.clone(), "year"), &op_floordiv(&op_sub(&month.clone(), &Py::Int(1)), &Py::Int(12))), op_add(&op_modulo(&op_sub(&month.clone(), &Py::Int(1)), &Py::Int(12)), &Py::Int(1))]), 2, None);
    year = __unpk8[0].clone();
    month = __unpk8[1].clone();
    return py_tuple(vec![name.clone(), mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), geti(&mcall(&calendar.clone(), "monthrange", &[year.clone(), month.clone()]), &Py::Int(1))])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the end of next year"), py_str("the end of this year"), py_str("the end of the year")]), &name.clone()))) {
    return py_tuple(vec![name.clone(), mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str("next")))) { Py::Int(1) } else { Py::Int(0) }), Py::Int(12), Py::Int(31)])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the end of next week"), py_str("the end of this week")]), &name.clone()))) {
    return py_tuple(vec![name.clone(), op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_add(&op_sub(&Py::Int(7), &mcall(&today.clone(), "weekday", &[])), &if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str("next")))) { Py::Int(7) } else { Py::Int(0) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })]);
    } else {
    return Py::None;
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
    }
    let __try9 = exc::catch(|| {
    when = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), month.clone(), day.clone()]);
    if py_truthy(&op_le(&when.clone(), &today.clone())) {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), day.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret10: Py = Py::Bool(false);
    let mut __val11: Py = Py::None;
    let mut __unhandled12: Option<exc::PyExc> = None;
    match __try9 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret10 = __p[0].clone(); __val11 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled13 = false;
            if !__handled13 && exc::matches_any(&__e, &["ValueError"]) {
                __handled13 = true;
    return Py::None;
            }
            if !__handled13 { __unhandled12 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled12 { exc::raise(__e); }
    if py_truthy(&__ret10) { return __val11; }
    return py_tuple(vec![name.clone(), when.clone()]);
    Py::None
}
pub fn _save_by_a_date(args: &[Py]) -> Py {
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
    let mut dated: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut name: Py = Py::None;
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut today: Py = Py::None;
    let mut total: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A goal and a named date: what has to go by each month, and each week when it is close.");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    dated = _a_named_deadline(&[clean.clone(), today.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&dated.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&dated.clone(), 2, None);
    name = __unpk1[0].clone();
    when = __unpk1[1].clone();
    if py_truthy(&mcall(&g_PAST_REPORT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    total = _k_as_money(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    numbers = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_parse(&[clean.clone()])) { let mut number: Py = __item.clone(); if py_truthy(&op_ge(&number.clone(), &Py::Int(50))) {  __out.push(number.clone()); } } py_list(__out) })() };
    total = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&numbers.clone(), &Py::Int(0)) } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_SAVE_VERB(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DATE_FOR_THE_THING(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    total = _goal_in_chat(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return Py::None;
    }
    return _a_total_by_a_date(&[total.clone(), name.clone(), when.clone(), today.clone()]);
    Py::None
}
pub fn _a_total_by_a_date(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut total: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "total") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: total"))) };
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut when: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut today: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { Py::None };
    let mut a_week: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut months: Py = Py::None;
    let mut said: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("A total and a date: the figure that comes out of each pay packet, or None.");
    datetime = stdlib_module("datetime");
    today = { let __b = today.clone(); if py_truthy(&__b) { __b } else { mcall(&gattr(&datetime.clone(), "date"), "today", &[]) } };
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    if py_truthy(&op_lt(&days.clone(), &Py::Int(1))) {
    return Py::None;
    }
    months = op_truediv(&days.clone(), &Py::Float(30.44));
    if py_truthy(&op_lt(&months.clone(), &Py::Float(1.2))) {
    said = { let mut __s = String::new(); __s.push_str("£"); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away, so it is one pay packet: put the £"); __s.push_str(&fstr(&_number(&[call_builtin("round", &[total.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" aside on payday before anything else"); py_string(__s) };
    } else {
    said = { let mut __s = String::new(); __s.push_str("£"); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away, so about £"); __s.push_str(&fstr(&_number(&[call_builtin("round", &[op_truediv(&total.clone(), &months.clone())]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" a month"); py_string(__s) };
    if py_truthy(&op_le(&days.clone(), &Py::Int(120))) {
    weeks = op_neg(&op_floordiv(&op_neg(&days.clone()), &Py::Int(7)));
    a_week = op_neg(&op_floordiv(&op_neg(&call_builtin("int", &[call_builtin("round", &[total.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &weeks.clone()));
    let __aug1 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" - about £"); __s.push_str(&fstr(&_number(&[a_week.clone()]), -1, "")); __s.push_str(" a week over the "); __s.push_str(&fstr(&weeks.clone(), -1, "")); __s.push_str(" weeks left, which is the number that actually comes out of a pay packet"); py_string(__s) });
    said = __aug1;
    }
    }
    return op_add(&said.clone(), &py_str(". Take it off the top on payday rather than saving what is left at the end."));
    Py::None
}
pub fn _a_month_for_the_deadline_in_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut total: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "total") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: total"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut dated: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The figure to the date the chat named: \"how much a month\" after a goal with a deadline.\n\n    \"im saving for a car\" / \"i need about 4000\" / \"i want it by next summer\" / \"how much a\n    month\" is one sum, and every number of it is already in the chat. The date is worked out the\n    same way as when the person says it on the same line, so the two routes cannot disagree.\n    ");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    dated = _a_named_deadline(&[line.clone(), today.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&dated.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_PAST_REPORT(), "search", &[line.clone()])) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&dated.clone(), 2, None);
    name = __unpk2[0].clone();
    when = __unpk2[1].clone();
    return _a_total_by_a_date(&[total.clone(), name.clone(), when.clone(), today.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _deadline_for_a_goal(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut months: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut per: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut short_hand: Py = Py::None;
    let mut total: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A date on its own for a figure already in the chat: \"i want it in a year\".\n\n    When both the goal and the time to reach it are known, the one number left is what it\n    takes each month: 5,000 in a year is 417 a month. Without this the follow-up after \"im\n    saving for a car hoping to spend about 5000\" was answered as small talk, and the person\n    still had to do the division themselves. The line has to carry the date and nothing else:\n    a line with its own figure or rate is a different sum, and the forms above answer it.\n    ");
    said = mcall(&g_HORIZON_SAID(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TIME_ELSEWHERE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DEADLINE_NAMED(), "search", &[clean.clone()]) } } } }) {
    return if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_DEADLINE_NAMED(), "search", &[clean.clone()])))) { Py::None } else { _save_by_a_date(&[clean.clone(), memory.clone()]) };
    }
    raw = mcall(&said.clone(), "group", &[py_str("count")]);
    count = if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("float", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g_HORIZON_COUNTS(), "get", &[mcall(&py_str(" "), "join", &[mcall(&raw.clone(), "split", &[])])]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if py_truthy(&__b) { __b } else { mcall(&g_RATE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    rest = mcall(&op_add(&op_add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&said.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clean.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    numbers = _parse(&[rest.clone()]);
    short_hand = _k_as_money(&[clean.clone()]);
    if py_truthy(&short_hand.clone()) {
    numbers = py_list(vec![short_hand.clone()]);
    }
    if py_truthy(&numbers.clone()) {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GOAL(), "search", &[rest.clone()]))); if py_truthy(&__b) { __b } else { op_lt(&geti(&numbers.clone(), &Py::Int(0)), &Py::Int(50)) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_PAST_REPORT(), "search", &[rest.clone()])) {
    return Py::None;
    }
    total = geti(&numbers.clone(), &Py::Int(0));
    } else {
    total = _goal_in_chat(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&total.clone()))) {
    return Py::None;
    }
    unit = mcall(&mcall(&said.clone(), "group", &[py_str("unit")]), "rstrip", &[py_str("s")]);
    months = op_mul(&count.clone(), &geti(&g_PERIODS(), &unit.clone()));
    if py_truthy(&{ let __b = op_lt(&months.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&months.clone(), &Py::Int(600)) } }) {
    return Py::None;
    }
    per = op_truediv(&total.clone(), &months.clone());
    if py_truthy(&op_lt(&per.clone(), &Py::Int(1))) {
    return Py::None;
    }
    when = if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { op_add(&py_str("a "), &unit.clone()) } else { { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[count.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str("s"); py_string(__s) } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&when.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[per.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" a month. Set the transfer for the day after payday so the money goes before you can spend it."); py_string(__s) };
    Py::None
}
pub fn _asked_period(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let _ = py_str("The period the line asks about, month when it names none.");
    asked = mcall(&g_EACH_PERIOD(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return if py_truthy(&asked.clone()) { mcall(&asked.clone(), "group", &[Py::Int(1)]) } else { py_str("month") };
    Py::None
}
pub fn _a_deadline_in_chat(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the chat named a deadline (\"by december\") that a per-month figure would ignore.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&g_DEADLINE_NAMED(), "search", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _saving(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut per: Py = Py::None;
    let mut total: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A monthly or weekly amount from \"save 480 pounds in 6 months\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("need"), py_str("want"), py_str("should"), py_str("how much"), py_str("each"), py_str("per"), py_str("plan")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    amount = stdlib_call("re", "search", &[py_str("(?:save|saving|saved)\\s+(?:up\\s+)?[£$€]?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    when = stdlib_call("re", "search", &[py_str("(?:in|over|within)\\s+(\\d+)\\s*(month|week|year)s?"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&amount.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&when.clone())) } }) {
    return Py::None;
    }
    total = call_builtin("float", &[mcall(&mcall(&amount.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    count = call_builtin("int", &[mcall(&when.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    unit = mcall(&when.clone(), "group", &[Py::Int(2)]);
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("year")))) {
    let __unpk1 = py_unpack(&py_tuple(vec![op_mul(&count.clone(), &Py::Int(12)), py_str("month")]), 2, None);
    count = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    }
    if py_truthy(&op_lt(&count.clone(), &Py::Int(1))) {
    return Py::None;
    }
    per = op_truediv(&total.clone(), &count.clone());
    return { let mut __s = String::new(); __s.push_str("Saving "); __s.push_str(&fstr(&_number(&[total.clone()]), -1, "")); __s.push_str(" over "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str("s is "); __s.push_str(&fstr(&_number(&[per.clone()]), -1, "")); __s.push_str(" a "); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _average(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut mean: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut word: Py = Py::None;
    match_ = stdlib_call("re", "search", &[py_str("average(?: of)?\\s+(.+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    numbers = _parse(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("speed"), py_str("month"), py_str("year")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    mean = op_truediv(&call_builtin("sum", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return { let mut __s = String::new(); __s.push_str("The average is "); __s.push_str(&fstr(&_number(&[mean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _sides(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut sizes: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut unit_at: Py = Py::None;
    let mut value: Py = Py::None;
    let mut value_at: Py = Py::None;
    let _ = py_str("The side lengths named in a line as (value, unit, unit_from_the_line), or None.\n\n    \"a 4m by 3m room\" gives 4m and 3m; \"12ft x 10ft\" gives 12ft and 10ft; \"a 4 by 3\"\n    gives two bare numbers and no unit at all, which is still a rectangle somebody drew.\n    ");
    found = mcall(&g__SIDE_BY_SIDE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    sizes = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![Py::Int(1), Py::Int(2)]), py_tuple(vec![Py::Int(3), Py::Int(4)]), py_tuple(vec![Py::Int(5), Py::Int(6)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    value_at = __unpk2[0].clone();
    unit_at = __unpk2[1].clone();
    raw = mcall(&found.clone(), "group", &[value_at.clone()]);
    if py_truthy(&Py::Bool(py_is(&raw.clone(), &Py::None))) {
    continue;
    }
    unit = mcall(&g__LENGTH_UNIT(), "get", &[{ let __b = mcall(&found.clone(), "group", &[unit_at.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }, Py::None]);
    let _ = mcall(&sizes.clone(), "append", &[py_tuple(vec![call_builtin("float", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)), unit.clone()])]);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[sizes.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    named = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sizes.clone()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk1[0].clone(); let mut unit: Py = __unpk1[1].clone(); if py_truthy(&unit.clone()) {  __out.push(unit.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return py_tuple(vec![sizes.clone(), Py::None]);
    }
    return py_tuple(vec![{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sizes.clone()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut value: Py = __unpk1[0].clone(); let mut unit: Py = __unpk1[1].clone(); __out.push(py_tuple(vec![value.clone(), { let __b = unit.clone(); if py_truthy(&__b) { __b } else { named.clone() } }])); } py_list(__out) })() }, named.clone()]);
    Py::None
}
pub fn _surface(args: &[Py]) -> Py {
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
    let mut label: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The word for the thing whose size was asked, or \"floor\" when the line names none.");
    let __iter1 = py_iter_or_raise(&g__SURFACES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    label = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return label.clone();
    }
    }
    return py_str("floor");
    Py::None
}
pub fn _square_text(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    word = mcall(&g__SQUARE_WORD(), "get", &[unit.clone(), py_str("square units")]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&_number(&[call_builtin("round", &[value.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _floor_area(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut area: Py = Py::None;
    let mut found: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut sides: Py = Py::None;
    let mut sizes: Py = Py::None;
    let mut target: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut to: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"a 4m by 3m room\": the two sides of a surface, multiplied.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AREA_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__BUYING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BOUGHT_BY_THE_BAG()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__COUNTED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__SHAPE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    found = _sides(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    sizes = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    area = op_mul(&geti(&geti(&sizes.clone(), &Py::Int(0)), &Py::Int(0)), &geti(&geti(&sizes.clone(), &Py::Int(1)), &Py::Int(0)));
    thing = _surface(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&unit.clone(), &Py::None))) {
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(0)), &Py::Int(0))]), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(1)), &Py::Int(0))]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[area.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" square metres if those are metres, "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[area.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" square feet if they are feet."); py_string(__s) };
    }
    sides = mcall(&py_str(" by "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&sizes.clone(), &Py::None, &Py::Int(2), &Py::None)) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut value: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&_number(&[value.clone()]), -1, "")); __s.push_str(&fstr(&unit.clone(), -1, "")); py_string(__s) }); } py_list(__out) })() }]);
    target = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AREA_TARGET()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk1[0].clone(); let mut to: Py = __unpk1[1].clone(); if py_truthy(&Py::Bool(py_contains(&clean.clone(), &word.clone()))) {  __out.push(to.clone()); } } py_list(__out) })() }, unit.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&target.clone(), &unit.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__METRES_SQUARED(), &target.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__METRES_SQUARED(), &unit.clone())) } } } }) {
    wanted = call_builtin("round", &[op_truediv(&op_mul(&area.clone(), &geti(&g__METRES_SQUARED(), &unit.clone())), &geti(&g__METRES_SQUARED(), &target.clone())), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&sides.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_square_text(&[wanted.clone(), target.clone()]), -1, "")); __s.push_str(", which is "); __s.push_str(&fstr(&_square_text(&[area.clone(), unit.clone()]), -1, "")); __s.push_str(". That is the two sides multiplied: "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(0)), &Py::Int(0))]), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(1)), &Py::Int(0))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&sides.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_square_text(&[area.clone(), unit.clone()]), -1, "")); __s.push_str(". That is the two sides multiplied: "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(0)), &Py::Int(0))]), -1, "")); __s.push_str(" x "); __s.push_str(&fstr(&_number(&[geti(&geti(&sizes.clone(), &Py::Int(1)), &Py::Int(0))]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _perimeter_of_a_room(args: &[Py]) -> Py {
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
    let mut around: Py = Py::None;
    let mut found: Py = Py::None;
    let mut length: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut sizes: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut width: Py = Py::None;
    let _ = py_str("\"the skirting for a 4m by 3m room\": the two sides added, then doubled.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PERIMETER_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    found = _sides(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    sizes = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&geti(&sizes.clone(), &Py::Int(0)), &Py::Int(0)), geti(&geti(&sizes.clone(), &Py::Int(1)), &Py::Int(0))]), 2, None);
    length = __unpk2[0].clone();
    width = __unpk2[1].clone();
    if py_truthy(&mcall(&g__SHAPE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:how many|how much)\\s+(?:\\w+\\s+){0,2}skirting\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    around = op_mul(&Py::Int(2), &op_add(&length.clone(), &width.clone()));
    thing = _surface(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&unit.clone(), &Py::None))) {
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&_number(&[length.clone()]), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_number(&[width.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" has a perimeter of "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[around.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" metres if those are metres: twice ("); __s.push_str(&fstr(&_number(&[length.clone()]), -1, "")); __s.push_str(" + "); __s.push_str(&fstr(&_number(&[width.clone()]), -1, "")); __s.push_str(")."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&_number(&[length.clone()]), -1, "")); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_number(&[width.clone()]), -1, "")); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" has a perimeter of "); __s.push_str(&fstr(&_number(&[call_builtin("round", &[around.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(&fstr(&unit.clone(), -1, "")); __s.push_str(": twice ("); __s.push_str(&fstr(&_number(&[length.clone()]), -1, "")); __s.push_str(" + "); __s.push_str(&fstr(&_number(&[width.clone()]), -1, "")); __s.push_str("). That is how much skirting, edging or border it takes."); py_string(__s) };
    Py::None
}
pub fn _area(args: &[Py]) -> Py {
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
    let mut diameter: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut r: Py = Py::None;
    let mut radius: Py = Py::None;
    let mut sized: Py = Py::None;
    sized = _floor_area(&[clean.clone()]);
    if py_truthy(&sized.clone()) {
    return sized.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("area"), py_str("how big"), py_str("how much space")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("circle")))) {
    radius = stdlib_call("re", "search", &[py_str("radius(?: of| is)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&radius.clone()) {
    r = call_builtin("float", &[mcall(&mcall(&radius.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str("A circle with radius "); __s.push_str(&fstr(&_number(&[r.clone()]), -1, "")); __s.push_str(" has an area of "); __s.push_str(&fstr(&_number(&[op_mul(&op_mul(&Py::Float(3.14159265), &r.clone()), &r.clone())]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    diameter = stdlib_call("re", "search", &[py_str("diameter(?: of| is)?\\s*([\\d,.]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&diameter.clone()) {
    r = op_truediv(&call_builtin("float", &[mcall(&mcall(&diameter.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    return { let mut __s = String::new(); __s.push_str("A circle with diameter "); __s.push_str(&fstr(&_number(&[op_mul(&r.clone(), &Py::Int(2))]), -1, "")); __s.push_str(" has an area of "); __s.push_str(&fstr(&_number(&[op_mul(&op_mul(&Py::Float(3.14159265), &r.clone()), &r.clone())]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return py_str("The area of a circle is π × radius²: square the radius, then multiply by 3.14.");
    }
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("triangle")))) {
    numbers = _parse(&[clean.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return { let mut __s = String::new(); __s.push_str("That triangle's area is "); __s.push_str(&fstr(&_number(&[op_truediv(&op_mul(&geti(&numbers.clone(), &Py::Int(0)), &geti(&numbers.clone(), &Py::Int(1))), &Py::Int(2))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return py_str("The area of a triangle is ½ × base × height.");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("rectangle"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("square"))) } }) {
    numbers = _parse(&[clean.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[numbers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return { let mut __s = String::new(); __s.push_str("That rectangle's area is "); __s.push_str(&fstr(&_number(&[op_mul(&geti(&numbers.clone(), &Py::Int(0)), &geti(&numbers.clone(), &Py::Int(1)))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return py_str("The area of a rectangle is length × width.");
    }
    return Py::None;
    Py::None
}
pub fn _month_year(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut found: Py = Py::None;
    let mut today: Py = Py::None;
    let _ = py_str("The year a month question names, if it names one (\"this year\" is the current one).");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bnext year\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&gattr(&today.clone(), "year"), &Py::Int(1));
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:this year|this time)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return gattr(&today.clone(), "year");
    }
    found = stdlib_call("re", "search", &[py_str("\\b(?:in |of )?((?:19|20)\\d\\d)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    return call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::None;
    Py::None
}
pub fn _leap(args: &[Py]) -> Py {
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
    let _ = py_str("Whether February has 29 days in that year (the rule, not a table).");
    return { let __b = Py::Bool(py_eq(&op_modulo(&year.clone(), &Py::Int(4)), &Py::Int(0))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&op_modulo(&year.clone(), &Py::Int(100)), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_modulo(&year.clone(), &Py::Int(400)), &Py::Int(0))) } } } };
    Py::None
}
pub fn _days_in_month(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut month: Py = Py::None;
    let mut text: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("How long one month is, when that is the whole question.\n\n    The month has to be the thing asked about: \"how many days in February\" and \"how\n    many days does August have\" are questions about a month, while \"how many days\n    should I take off in August\" is a question about leave that happens to name a\n    month, and \"August has 31 days\" would be a wrong answer to it.\n    ");
    text = _plain(&[clean.clone()]);
    let __iter1 = py_iter_or_raise(&mcall(&g_MONTHS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    month = __unpk2[0].clone();
    days = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "fullmatch", &[op_add(&{ let mut __s = String::new(); __s.push_str("(?:how many days (?:are there )?in (?:the month of )?"); __s.push_str(&fstr(&month.clone(), -1, "")); __s.push_str("|how many days (?:does|do) "); __s.push_str(&fstr(&month.clone(), -1, "")); __s.push_str(" have|how many days (?:do i have|have i got|are left) in "); __s.push_str(&fstr(&month.clone(), -1, "")); __s.push_str("|"); __s.push_str(&fstr(&month.clone(), -1, "")); __s.push_str(" has how many days)"); py_string(__s) }, &g__MONTH_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    year = _month_year(&[text.clone()]);
    if py_truthy(&Py::Bool(py_eq(&month.clone(), &py_str("february")))) {
    if py_truthy(&Py::Bool(!py_is(&year.clone(), &Py::None))) {
    longer = _leap(&[year.clone()]);
    return { let mut __s = String::new(); __s.push_str("February "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" has "); __s.push_str(&fstr(&if py_truthy(&longer.clone()) { Py::Int(29) } else { Py::Int(28) }, -1, "")); __s.push_str(" days: "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&if py_truthy(&longer.clone()) { py_str("a leap year") } else { py_str("not a leap year") }, -1, "")); __s.push_str("."); py_string(__s) };
    }
    return py_str("February has 28 days in an ordinary year and 29 in a leap year. Leap years are divisible by 4, except centuries unless they divide by 400.");
    }
    if py_truthy(&Py::Bool(!py_is(&year.clone(), &Py::None))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&month.clone(), "capitalize", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" has "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&month.clone(), "capitalize", &[]), -1, "")); __s.push_str(" has "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days."); py_string(__s) };
    }
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[{ let mut __s = String::new(); __s.push_str("(?:how many |how long )(?:days? )?(?:are there |does |is |has )?(?:in )?(?:a |one |the )?leap year(?: have| last| long| then| again)?"); __s.push_str(&fstr(&g__MONTH_TAIL(), -1, "")); py_string(__s) }, text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A leap year has 366 days: 29 February, and the rest of the year as usual. The rule is every four years, except centuries unless they divide by 400.");
    }
    return Py::None;
    Py::None
}
pub fn _christmas_name(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The day the line asked about: \"Christmas Eve\" when it said eve, else Christmas.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:christmas|xmas) eve\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("Christmas Eve") } else { py_str("Christmas") };
    Py::None
}
pub fn _christmas_date_of(args: &[Py]) -> Py {
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
    let mut year: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "year") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: year"))) };
    let mut datetime: Py = Py::None;
    let _ = py_str("Christmas Day or Christmas Eve in `year`, whichever the line named.");
    datetime = stdlib_module("datetime");
    return mcall(&datetime.clone(), "date", &[year.clone(), Py::Int(12), if py_truthy(&Py::Bool(py_eq(&_christmas_name(&[text.clone()]), &py_str("Christmas Eve")))) { Py::Int(24) } else { Py::Int(25) }]);
    Py::None
}
pub fn _christmas_day(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut christmas: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut said: Py = Py::None;
    let mut today: Py = Py::None;
    let _ = py_str("The Christmas Day the line means, and how many days away it is (negative once past).\n\n    A word that names the year decides which Christmas is meant: \"until christmas next year\"\n    is twelve months after the one \"until christmas\" counts, so answering it with this year's\n    number is a year out and said with confidence. \"this year\" in late December has already\n    gone, and the countdown says so instead of quietly counting next year's.\n    ");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bnext (?:year|christmas)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    christmas = _christmas_date_of(&[said.clone(), op_add(&gattr(&today.clone(), "year"), &Py::Int(1))]);
    } else {
    christmas = _christmas_date_of(&[said.clone(), gattr(&today.clone(), "year")]);
    if py_truthy(&{ let __b = op_lt(&christmas.clone(), &today.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bthis (?:year|christmas)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    christmas = _christmas_date_of(&[said.clone(), op_add(&gattr(&today.clone(), "year"), &Py::Int(1))]);
    }
    }
    return py_tuple(vec![christmas.clone(), gattr(&op_sub(&christmas.clone(), &today.clone()), "days")]);
    Py::None
}
pub fn _wait_text(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { py_str("Christmas") };
    let mut ago: Py = Py::None;
    let _ = py_str("How the wait reads: ahead of us, today, or already gone this year.");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    ago = op_neg(&days.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" this year was "); __s.push_str(&fstr(&ago.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&ago.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" ago."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _months_between(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut start: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut when: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut months: Py = Py::None;
    let _ = py_str("Whole months from one day to another on the calendar, and the days left over.\n\n    A month is a calendar month, not thirty days: from 9 November to 25 December is one month\n    and sixteen days, and thirty-day arithmetic would call it one month and two weeks. The\n    month is taken with the day clamped (`_month_on`), so 31 January plus a month is the end of\n    February rather than something that does not exist.\n    ");
    months = op_add(&op_mul(&op_sub(&gattr(&when.clone(), "year"), &gattr(&start.clone(), "year")), &Py::Int(12)), &op_sub(&gattr(&when.clone(), "month"), &gattr(&start.clone(), "month")));
    if py_truthy(&{ let __b = op_gt(&months.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { op_gt(&_month_on(&[start.clone(), months.clone()]), &when.clone()) } }) {
    let __aug1 = op_sub(&months.clone(), &Py::Int(1));
    months = __aug1;
    }
    return py_tuple(vec![months.clone(), gattr(&op_sub(&when.clone(), &_month_on(&[start.clone(), months.clone()])), "days")]);
    Py::None
}
pub fn _count_in(args: &[Py]) -> Py {
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
    let mut days: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "days") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: days"))) };
    let mut when: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { Py::None };
    let mut datetime: Py = Py::None;
    let mut months: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut start: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("The count in the unit the line asked for, so \"in weeks\" comes back in weeks.\n\n    `when` is the date being counted to, when the line asked for months: that unit is counted\n    on the calendar, so the answer says how many months and days away it is rather than\n    dividing by thirty.\n    ");
    datetime = stdlib_module("datetime");
    unit = mcall(&{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("sleep")])) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" sleeps"); py_string(__s) };
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) {
    let __unpk1 = py_unpack(&call_builtin("divmod", &[days.clone(), Py::Int(7)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    weeks = __unpk1[0].clone();
    spare = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&spare.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&weeks.clone(), -1, "")); __s.push_str(" weeks"); py_string(__s) };
    }
    return op_add(&{ let mut __s = String::new(); __s.push_str(&fstr(&weeks.clone(), -1, "")); __s.push_str(" weeks and "); __s.push_str(&fstr(&spare.clone(), -1, "")); __s.push_str(" day"); py_string(__s) }, &if py_truthy(&Py::Bool(py_eq(&spare.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") });
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&when.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&days.clone(), &Py::Int(0)) } }) {
    start = op_sub(&when.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), days.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    let __unpk2 = py_unpack(&_months_between(&[start.clone(), when.clone()]), 2, None);
    months = __unpk2[0].clone();
    spare = __unpk2[1].clone();
    } else {
    let __unpk3 = py_unpack(&call_builtin("divmod", &[days.clone(), Py::Int(30)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    months = __unpk3[0].clone();
    spare = __unpk3[1].clone();
    }
    if py_truthy(&{ let __b = months.clone(); if !py_truthy(&__b) { __b } else { spare.clone() } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&months.clone(), -1, "")); __s.push_str(" month"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&months.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&spare.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&spare.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); py_string(__s) };
    }
    if py_truthy(&months.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&months.clone(), -1, "")); __s.push_str(" month"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&months.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&spare.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&spare.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days"); py_string(__s) };
    Py::None
}
pub fn _shopping_days(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut step: Py = Py::None;
    let mut sundays: Py = Py::None;
    let mut today: Py = Py::None;
    let _ = py_str("How many of the days before Christmas are days the shops are open.\n\n    Sundays do not count: the big shops in Britain are shut or on short hours, so the number\n    people mean by \"shopping days\" is the count without them. It is always given with the\n    plain number of days, because the two are different and both are useful.\n    ");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    sundays = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), days.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), "weekday", &[]), &Py::Int(6)))) {
    let __aug2 = op_add(&sundays.clone(), &Py::Int(1));
    sundays = __aug2;
    }
    }
    return op_sub(&days.clone(), &sundays.clone());
    Py::None
}
pub fn _countdown_text(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut ago: Py = Py::None;
    let mut christmas: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut name: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The countdown in the unit that was asked for, and for the year that was named.");
    datetime = stdlib_module("datetime");
    groups = mcall(&found.clone(), "groupdict", &[]);
    unit = { let __b = mcall(&groups.clone(), "get", &[py_str("unit2")]); if py_truthy(&__b) { __b } else { mcall(&groups.clone(), "get", &[py_str("unit")]) } };
    year = mcall(&groups.clone(), "get", &[py_str("year")]);
    name = _christmas_name(&[text.clone()]);
    let __unpk1 = py_unpack(&_christmas_day(&[text.clone()]), 2, None);
    christmas = __unpk1[0].clone();
    days = __unpk1[1].clone();
    if py_truthy(&year.clone()) {
    christmas = _christmas_date_of(&[text.clone(), call_builtin("int", &[year.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    days = gattr(&op_sub(&christmas.clone(), &mcall(&gattr(&datetime.clone(), "date"), "today", &[])), "days");
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    ago = op_neg(&days.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" was "); __s.push_str(&fstr(&ago.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&ago.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" ago."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" is today."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), christmas.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    return _wait_text(&[days.clone(), name.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today."); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("is it\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __s = String::new(); __s.push_str("Not yet - "); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), christmas.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("shopping day")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" - "); __s.push_str(&fstr(&_shopping_days(&[days.clone()]), -1, "")); __s.push_str(" of them shopping days."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), christmas.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _days_until(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut found: Py = Py::None;
    let mut guessing: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut text: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A countdown to Christmas, only when the line asks for one.\n\n    \"How many days until Christmas\" is a countdown; \"what should I buy my sister for\n    Christmas\" is a shopping question that must not be answered with a number. The whole\n    cleaned line has to be that question, so a mention in passing never counts, and the ask\n    is spelled out in the pattern rather than guessed at with a second list of phrases.\n    ");
    text = _plain(&[clean.clone()]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[g__CHRISTMAS_TODAY(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk1 = py_unpack(&_christmas_day(&[py_str("")]), 2, None);
    _u = __unpk1[0].clone();
    days = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return py_str("Yes - it's Christmas Day today.");
    }
    return { let mut __s = String::new(); __s.push_str("Not yet - "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days until Christmas."); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[g__CHRISTMAS_FIXED(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    name = _christmas_name(&[text.clone()]);
    let __unpk2 = py_unpack(&_christmas_day(&[text.clone()]), 2, None);
    when = __unpk2[0].clone();
    days = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today, 25 December, the same date every year."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is always 25 December, so it is neither early nor late: it was on "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&when.clone(), "year"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is always 25 December, so it is neither early nor late: it is "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away."); py_string(__s) };
    }
    found = stdlib_call("re", "fullmatch", &[op_add(&g__CHRISTMAS_COUNTDOWN(), &g__CHRISTMAS_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    return _countdown_text(&[found.clone(), text.clone()]);
    }
    guessing = stdlib_call("re", "fullmatch", &[op_add(&g__CHRISTMAS_WEEKDAY(), &g__CHRISTMAS_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&guessing.clone()) {
    return _weekday_text(&[text.clone(), mcall(&mcall(&guessing.clone(), "groupdict", &[]), "get", &[py_str("guess")])]);
    }
    found = stdlib_call("re", "fullmatch", &[op_add(&g__CHRISTMAS_DATE(), &g__CHRISTMAS_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    let __unpk3 = py_unpack(&_christmas_day(&[text.clone()]), 2, None);
    when = __unpk3[0].clone();
    days = __unpk3[1].clone();
    name = _christmas_name(&[text.clone()]);
    named = if py_truthy(&Py::Bool(py_eq(&name.clone(), &py_str("Christmas Eve")))) { name.clone() } else { py_str("Christmas Day") };
    if py_truthy(&mcall(&found.clone(), "group", &[py_str("year")])) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&named.clone(), -1, "")); __s.push_str(" is on "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" December "); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[py_str("year")]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    return _wait_text(&[days.clone(), named.clone()]);
    }
    day = if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) { py_str("today") } else { { let mut __s = String::new(); __s.push_str("on "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" December, "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away"); py_string(__s) } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&named.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&day.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _weekday_text(args: &[Py]) -> Py {
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
    let mut guess: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "guess") { v.clone() }
    else { Py::None };
    let mut date: Py = Py::None;
    let mut days: Py = Py::None;
    let mut name: Py = Py::None;
    let mut weekday: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("\"Is christmas on a friday this year\", \"what day of the week is christmas\", answered\n    from the calendar. A guess in the line is answered yes or no rather than repeated.");
    let __unpk1 = py_unpack(&_christmas_day(&[text.clone()]), 2, None);
    when = __unpk1[0].clone();
    days = __unpk1[1].clone();
    name = _christmas_name(&[text.clone()]);
    weekday = mcall(&when.clone(), "strftime", &[py_str("%A")]);
    date = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" December "); __s.push_str(&fstr(&gattr(&when.clone(), "year"), -1, "")); py_string(__s) };
    if py_truthy(&guess.clone()) {
    if py_truthy(&Py::Bool(py_eq(&mcall(&guess.clone(), "lower", &[]), &mcall(&weekday.clone(), "lower", &[])))) {
    return { let mut __s = String::new(); __s.push_str("Yes - "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on a "); __s.push_str(&fstr(&weekday.clone(), -1, "")); __s.push_str(" this year, "); __s.push_str(&fstr(&date.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("No - "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on a "); __s.push_str(&fstr(&weekday.clone(), -1, "")); __s.push_str(" this year, "); __s.push_str(&fstr(&date.clone(), -1, "")); __s.push_str(", not a "); __s.push_str(&fstr(&mcall(&guess.clone(), "capitalize", &[]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on a "); __s.push_str(&fstr(&weekday.clone(), -1, "")); __s.push_str(" this year, "); __s.push_str(&fstr(&date.clone(), -1, "")); __s.push_str(". That is "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away."); py_string(__s) };
    Py::None
}
pub fn _easter_sunday(args: &[Py]) -> Py {
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
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let mut c: Py = Py::None;
    let mut d: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut e: Py = Py::None;
    let mut f: Py = Py::None;
    let mut g: Py = Py::None;
    let mut h: Py = Py::None;
    let mut i: Py = Py::None;
    let mut k: Py = Py::None;
    let mut l: Py = Py::None;
    let mut m: Py = Py::None;
    let mut month: Py = Py::None;
    let _ = py_str("Easter Sunday in the Gregorian calendar: the computus, in ten lines, no table.");
    datetime = stdlib_module("datetime");
    a = op_modulo(&year.clone(), &Py::Int(19));
    let __unpk1 = py_unpack(&call_builtin("divmod", &[year.clone(), Py::Int(100)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    b = __unpk1[0].clone();
    c = __unpk1[1].clone();
    let __unpk2 = py_unpack(&call_builtin("divmod", &[b.clone(), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    d = __unpk2[0].clone();
    e = __unpk2[1].clone();
    f = op_floordiv(&op_add(&b.clone(), &Py::Int(8)), &Py::Int(25));
    g = op_floordiv(&op_add(&op_sub(&b.clone(), &f.clone()), &Py::Int(1)), &Py::Int(3));
    h = op_modulo(&op_add(&op_sub(&op_sub(&op_add(&op_mul(&Py::Int(19), &a.clone()), &b.clone()), &d.clone()), &g.clone()), &Py::Int(15)), &Py::Int(30));
    let __unpk3 = py_unpack(&call_builtin("divmod", &[c.clone(), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    i = __unpk3[0].clone();
    k = __unpk3[1].clone();
    l = op_modulo(&op_sub(&op_sub(&op_add(&op_add(&Py::Int(32), &op_mul(&Py::Int(2), &e.clone())), &op_mul(&Py::Int(2), &i.clone())), &h.clone()), &k.clone()), &Py::Int(7));
    m = op_floordiv(&op_add(&op_add(&a.clone(), &op_mul(&Py::Int(11), &h.clone())), &op_mul(&Py::Int(22), &l.clone())), &Py::Int(451));
    let __unpk4 = py_unpack(&call_builtin("divmod", &[op_add(&op_sub(&op_add(&h.clone(), &l.clone()), &op_mul(&Py::Int(7), &m.clone())), &Py::Int(114)), Py::Int(31)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    month = __unpk4[0].clone();
    day = __unpk4[1].clone();
    return mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), op_add(&day.clone(), &Py::Int(1))]);
    Py::None
}
pub fn _feast_year(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut four_digits: Py = Py::None;
    let mut today: Py = Py::None;
    let _ = py_str("The year the line means, and whether it named one itself.");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    four_digits = stdlib_call("re", "search", &[py_str("\\b(20\\d{2})\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = four_digits.clone(); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = call_builtin("int", &[mcall(&four_digits.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2000), &__cmp11)) { op_le(&__cmp11, &Py::Int(2100)) } else { Py::Bool(false) } } } }) {
    return py_tuple(vec![call_builtin("int", &[mcall(&four_digits.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(true), today.clone()]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bnext (?:year|easter)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Bool(false), today.clone()]);
    }
    return py_tuple(vec![gattr(&today.clone(), "year"), Py::Bool(false), today.clone()]);
    Py::None
}
pub fn _month_number(args: &[Py]) -> Py {
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
    let _ = py_str("The month a name or short spelling stands for, or None.");
    word = mcall(&mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__MONTHS_SAID(), &word.clone()))) {
    return op_add(&mcall(&g__MONTHS_SAID(), "index", &[word.clone()]), &Py::Int(1));
    }
    return mcall(&g__MONTH_SHORT(), "get", &[word.clone()]);
    Py::None
}
pub fn _day_in_a_month(args: &[Py]) -> Py {
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
    let mut month_number: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month_number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month_number"))) };
    let mut today: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut datetime: Py = Py::None;
    let mut when: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("That day in that month, the next time it comes round.\n\n    Trying the years in turn is what makes \"how many days until 29 february\" countable: the date\n    is not on the calendar this year or next, and answering \"I do not know\" for a date that\n    exists every fourth year is the wrong kind of honesty.\n    ");
    datetime = stdlib_module("datetime");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[gattr(&today.clone(), "year"), op_add(&gattr(&today.clone(), "year"), &Py::Int(9))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    year = __item.clone();
    let __try2 = exc::catch(|| {
    when = mcall(&datetime.clone(), "date", &[year.clone(), month_number.clone(), day.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&op_ge(&when.clone(), &today.clone())) {
    return when.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _date_the_line_gave(args: &[Py]) -> Py {
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
    let mut bare_day: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut month: Py = Py::None;
    let mut month_index: Py = Py::None;
    let mut step: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The day and month the line names, as the next date it falls, and how to say it.");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    found = stdlib_call("re", "search", &[g__DAY_MONTH(), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    day = { let __b = mcall(&found.clone(), "group", &[py_str("day")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("day2")]) } };
    month = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("month")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("month2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    let __try1 = exc::catch(|| {
    when = _day_in_a_month(&[call_builtin("int", &[day.clone()]).unwrap_or_else(|e| exc::raise(e)), _month_number(&[month.clone()]), today.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError", "TypeError"]) {
                __handled5 = true;
    return py_tuple(vec![Py::None, Py::None]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&when.clone()) { py_tuple(vec![when.clone(), { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&when.clone(), "strftime", &[py_str("%B")]), -1, "")); py_string(__s) }]) } else { py_tuple(vec![Py::None, Py::None]) };
    }
    bare_day = mcall(&g__DAY_ONLY(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bare_day.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    day = call_builtin("int", &[mcall(&bare_day.clone(), "group", &[py_str("day")])]).unwrap_or_else(|e| exc::raise(e));
    let __iter6 = py_iter_or_raise(&call_builtin("range", &[Py::Int(13)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    step = __item.clone();
    month_index = op_add(&op_sub(&gattr(&today.clone(), "month"), &Py::Int(1)), &step.clone());
    let __try7 = exc::catch(|| {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &op_floordiv(&month_index.clone(), &Py::Int(12))), op_add(&op_modulo(&month_index.clone(), &Py::Int(12)), &Py::Int(1)), day.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["ValueError"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    if py_truthy(&op_ge(&when.clone(), &today.clone())) {
    return py_tuple(vec![when.clone(), { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&when.clone(), "strftime", &[py_str("%B")]), -1, "")); py_string(__s) }]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _month_on(args: &[Py]) -> Py {
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
    let mut months_ahead: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months_ahead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: months_ahead"))) };
    let mut calendar: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut last: Py = Py::None;
    let mut month: Py = Py::None;
    let mut month_index: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The same day that many months on, in the month that has it.");
    calendar = stdlib_module("calendar");
    datetime = stdlib_module("datetime");
    month_index = op_add(&op_sub(&gattr(&day.clone(), "month"), &Py::Int(1)), &months_ahead.clone());
    year = op_add(&gattr(&day.clone(), "year"), &op_floordiv(&month_index.clone(), &Py::Int(12)));
    month = op_add(&op_modulo(&month_index.clone(), &Py::Int(12)), &Py::Int(1));
    last = geti(&mcall(&calendar.clone(), "monthrange", &[year.clone(), month.clone()]), &Py::Int(1));
    return mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), call_builtin("min", &[gattr(&day.clone(), "day"), last.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _first_of_the_month(args: &[Py]) -> Py {
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
    let mut months_ahead: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "months_ahead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: months_ahead"))) };
    let mut datetime: Py = Py::None;
    let mut month: Py = Py::None;
    let mut month_index: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The 1st of the month that many months on: the turn of the month, not a day inside it.");
    datetime = stdlib_module("datetime");
    month_index = op_add(&op_sub(&gattr(&day.clone(), "month"), &Py::Int(1)), &months_ahead.clone());
    year = op_add(&gattr(&day.clone(), "year"), &op_floordiv(&month_index.clone(), &Py::Int(12)));
    month = op_add(&op_modulo(&month_index.clone(), &Py::Int(12)), &Py::Int(1));
    return mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), Py::Int(1)]);
    Py::None
}
pub fn _relative_day(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut found: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The date a phrase counted from today points at, and how to say it.\n\n    The phrase is turned into a date rather than into a rule about adding days, so every way of\n    saying the same day lands on one date: \"tomorrow\", \"in a day\" and \"a day from now\" are the\n    same count, and a month is a month on the calendar rather than thirty days.\n    ");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("(?:today|tonight|this day)"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![today.clone(), py_str("today")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("tomorrow"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), py_str("tomorrow")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("(?:the )?day after tomorrow"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), py_str("the day after tomorrow")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("(?:the|this) weekend"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_modulo(&op_sub(&Py::Int(5), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), py_str("the weekend")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("next weekend"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_add(&op_modulo(&op_sub(&Py::Int(5), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7)), &Py::Int(7)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), py_str("next weekend")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("next week"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), { let __b = op_modulo(&op_sub(&Py::Int(7), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7)); if py_truthy(&__b) { __b } else { Py::Int(7) } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), py_str("next week")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("next month"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![_first_of_the_month(&[today.clone(), Py::Int(1)]), py_str("next month")]);
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("next year"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(1), Py::Int(1)]), py_str("next year")]);
    }
    found = mcall(&g__HOW_FAR(), "fullmatch", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    raw = mcall(&mcall(&found.clone(), "group", &[py_str("num")]), "lower", &[]);
    count = if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__SMALL_NUMBER(), "get", &[raw.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if py_truthy(&__b) { __b } else { op_gt(&count.clone(), &Py::Int(999)) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    unit = mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]);
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("day")])) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), count.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), Py::None]);
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) {
    return py_tuple(vec![op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_mul(&Py::Int(7), &count.clone()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), Py::None]);
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) {
    return py_tuple(vec![_month_on(&[today.clone(), count.clone()]), Py::None]);
    }
    return py_tuple(vec![_month_on(&[today.clone(), op_mul(&Py::Int(12), &count.clone())]), Py::None]);
    Py::None
}
pub fn _relative_count_text(args: &[Py]) -> Py {
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
    let mut days: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "days") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: days"))) };
    let mut counted: Py = Py::None;
    let _ = py_str("The count in the unit the line asked for, in the singular when it is one.");
    unit = mcall(&{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    counted = if py_truthy(&mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("sleep"), py_str("night")])])) { py_str("sleeps") } else { unit.clone() };
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    return if py_truthy(&mcall(&counted.clone(), "endswith", &[py_str("s")])) { slice_or_raise(&counted.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { counted.clone() };
    }
    return counted.clone();
    Py::None
}
pub fn _days_until_a_relative_day(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut found: Py = Py::None;
    let mut one: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A countdown to a day counted from today: \"how many sleeps until the weekend\".");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    found = stdlib_call("re", "fullmatch", &[g__COUNT_ASK(), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    rest = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_relative_day(&[rest.clone()]), 2, None);
    when = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    return Py::None;
    }
    days = gattr(&op_sub(&when.clone(), &mcall(&gattr(&datetime.clone(), "date"), "today", &[])), "days");
    unit = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    one = _relative_count_text(&[unit.clone(), days.clone()]);
    if py_truthy(&op_le(&days.clone(), &Py::Int(0))) {
    return { let mut __s = String::new(); __s.push_str("That is today - no "); __s.push_str(&fstr(&_relative_count_text(&[unit.clone(), Py::Int(2)]), -1, "")); __s.push_str(" left to wait."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    if py_truthy(&Py::Bool(py_eq(&said.clone(), &py_str("tomorrow")))) {
    return { let mut __s = String::new(); __s.push_str("Tomorrow, so one "); __s.push_str(&fstr(&one.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&one.clone(), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), when.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { _short_date(&[when.clone()]) } }, -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _date_counted_from_today(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("\"what date is it in two weeks\": the calendar arithmetic, said as a date.");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    rest = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("what(?:'s|s| is)? the (?:date|day) (?:(?:in|of) )?(?P<rest>.+)"), py_str("what date (?:is it|is|will it be) (?P<rest>.+)"), py_str("what day (?:is it|will it be) (?P<rest>.+)"), py_str("what date is it on (?P<rest>.+)")]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = stdlib_call("re", "fullmatch", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    rest = mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_relative_day(&[rest.clone()]), 2, None);
    when = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&when.clone(), &mcall(&gattr(&datetime.clone(), "date"), "today", &[])))) {
    return { let mut __s = String::new(); __s.push_str("That is today, "); __s.push_str(&fstr(&_short_date(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("That is "); __s.push_str(&fstr(&_day_text(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _days_until_a_named_day(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut event: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A countdown to an event whose date the bot cannot know: ask for the day, then count.\n\n    \"How many sleeps until my birthday\" used to reach the sleep-needs entry and come back with\n    seven to nine hours a night (#1519), which is a different question about a different thing.\n    \"How many sleeps till christmas\" works because Christmas is a known date; a birthday, a\n    holiday or a wedding is not, so the honest answer is to ask for it and say how the count\n    works. A date said in the line is counted rather than asked for again.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[op_add(&g__CHRISTMAS_DAY(), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\beaster\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    found = stdlib_call("re", "fullmatch", &[g__COUNT_ASK(), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    rest = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:i|we)\\s+(?:go|am going|are going|'m going|'re going|leave)\\s+(?:on\\s+)?")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    unit = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    counted = if py_truthy(&{ let __b = mcall(&unit.clone(), "startswith", &[py_str("sleep")]); if py_truthy(&__b) { __b } else { mcall(&unit.clone(), "startswith", &[py_str("night")]) } }) { py_str("sleeps") } else { unit.clone() };
    event = stdlib_call("re", "fullmatch", &[op_add(&g__COUNTED_EVENT(), &py_str("(?:\\s+(?:please|thanks|now|yet))?")), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    named = call_builtin("bool", &[event.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&_date_the_line_gave(&[rest.clone()]), &Py::Int(0)))) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_date_the_line_gave(&[rest.clone()]), 2, None);
    when = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_is(&when.clone(), &Py::None))) {
    days = gattr(&op_sub(&when.clone(), &mcall(&gattr(&datetime.clone(), "date"), "today", &[])), "days");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str("That is today - no "); __s.push_str(&fstr(&counted.clone(), -1, "")); __s.push_str(" left to wait."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), when.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bpay ?day\\b"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __s = String::new(); __s.push_str("I can't see your payday from here, so tell me the date it lands on and I'll count the "); __s.push_str(&fstr(&counted.clone(), -1, "")); __s.push_str(" to it."); py_string(__s) };
    }
    asked = { let mut __s = String::new(); __s.push_str("I can't see a calendar from here, so tell me the date - the day and the month - and I'll count it: how many "); __s.push_str(&fstr(&counted.clone(), -1, "")); __s.push_str(" from today."); py_string(__s) };
    if py_truthy(&mcall(&counted.clone(), "startswith", &[py_str("month")])) {
    return asked.clone();
    }
    return op_add(&asked.clone(), &py_str(" If it is a child waiting, a paper chain or a row of crosses on the calendar is the same count, and they can do one a night, which does more for the waiting than the number does."));
    Py::None
}
pub fn _days_left_in_the_month_or_the_year(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut end: Py = Py::None;
    let mut extra: Py = Py::None;
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut waiting: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("How much of the month or the year is still to come, counted from today.\n\n    The count starts tomorrow, because that is what \"left\" means to someone looking at a\n    calendar, and the answer says both numbers: \"21 days left in November after today, 22\n    counting today\". Saying only the first would be short by one for anyone counting today in.\n    A month that has not begun yet is counted to its last day instead, because \"how many days\n    left in December\" said in November is a wait, not the length of a month that is not here.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__REST_OF_A_PERIOD(), "search", &[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&text.clone(), "split", &[]), &py_str("how"))) } }) {
    return Py::None;
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9]+"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g__REST_OF_A_PERIOD_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&py_set(vec![py_str("left"), py_str("go"), py_str("remaining"), py_str("remain"), py_str("end")]), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))))) {
    return Py::None;
    }
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    unit = if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("weeks"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("week"))) } }) { py_str("weeks") } else { py_str("days") };
    month = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__MONTH_NAMES()); __kws.extend(vec![("start".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    number = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &mcall(&name.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(3), &Py::None), "lower", &[]))) } }) {
    month = number.clone();
    }
    }
    waiting = Py::Bool(false);
    extra = py_str("");
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("tax")))) {
    end = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(4), Py::Int(5)]);
    if py_truthy(&op_lt(&end.clone(), &today.clone())) {
    end = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(4), Py::Int(5)]);
    }
    said = py_str("the tax year");
    extra = { let mut __s = String::new(); __s.push_str(" It ends on "); __s.push_str(&fstr(&_said_with_a_weekday(&[end.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    } else {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("year")))) {
    end = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(12), Py::Int(31)]);
    said = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&today.clone(), "year"), -1, "")); py_string(__s) };
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&month.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&month.clone(), &gattr(&today.clone(), "month"))) } }) {
    year = op_add(&gattr(&today.clone(), "year"), &if py_truthy(&op_lt(&month.clone(), &gattr(&today.clone(), "month"))) { Py::Int(1) } else { Py::Int(0) });
    end = op_sub(&mcall(&datetime.clone(), "date", &[op_add(&year.clone(), &op_floordiv(&month.clone(), &Py::Int(12))), op_add(&op_modulo(&month.clone(), &Py::Int(12)), &Py::Int(1)), Py::Int(1)]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    said = { let mut __s = String::new(); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&end.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(": the last day of it is "); __s.push_str(&fstr(&_said_with_a_weekday(&[end.clone()]), -1, "")); py_string(__s) };
    waiting = Py::Bool(true);
    } else {
    end = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &op_floordiv(&gattr(&today.clone(), "month"), &Py::Int(12))), op_add(&op_modulo(&gattr(&today.clone(), "month"), &Py::Int(12)), &Py::Int(1)), Py::Int(1)]);
    let __aug3 = op_sub(&end.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    end = __aug3;
    said = { let mut __s = String::new(); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&end.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    }
    }
    }
    days = gattr(&op_sub(&end.clone(), &today.clone()), "days");
    if py_truthy(&op_le(&days.clone(), &Py::Int(0))) {
    return { let mut __s = String::new(); __s.push_str("That is today - nothing left of "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" after today, and "); __s.push_str(&fstr(&_count_in(&[unit.clone(), Py::Int(1)]), -1, "")); __s.push_str(" in all if you count it."); py_string(__s) };
    }
    if py_truthy(&waiting.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone()]), -1, "")); __s.push_str(" until the end of "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(". That is "); __s.push_str(&fstr(&_count_in(&[unit.clone(), op_add(&days.clone(), &Py::Int(1))]), -1, "")); __s.push_str(" counting today."); py_string(__s) };
    }
    return op_add(&{ let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone()]), -1, "")); __s.push_str(" left in "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" after today, "); __s.push_str(&fstr(&_count_in(&[unit.clone(), op_add(&days.clone(), &Py::Int(1))]), -1, "")); __s.push_str(" counting today."); py_string(__s) }, &extra.clone());
    Py::None
}
pub fn _year_the_line_gave(args: &[Py]) -> Py {
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
    let _ = py_str("\"How long ago was 3 March 2020\" names a year, and the year is half the question.\n\n    Without it the count runs back to the last time the day came round, which for a date years\n    back is a confident number about the wrong year - worse than saying nothing. So a year in\n    the line is read, used, and put in the answer.\n    ");
    found = mcall(&g__A_YEAR_IN_THE_LINE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return if py_truthy(&found.clone()) { call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut count: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The days between a date the line gives and today, counting backwards.\n\n    \"How long ago was 3 March\" is the same arithmetic as a countdown with the sign turned\n    round, and the answer says which weekday it was, because that is how people place a date\n    they half remember.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    found = mcall(&g__SINCE_ASK(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    rest = mcall(&op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    rest = mcall(&py_str(" "), "join", &[mcall(&mcall(&g__SINCE_FILLER(), "sub", &[py_str(" "), rest.clone()]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    year = _year_the_line_gave(&[rest.clone()]);
    let __unpk1 = py_unpack(&_date_the_line_gave(&[rest.clone()]), 2, None);
    when = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    let __unpk2 = py_unpack(&_a_known_day_that_has_gone(&[rest.clone(), year.clone()]), 2, None);
    when = __unpk2[0].clone();
    said = __unpk2[1].clone();
    } else {
    if py_truthy(&Py::Bool(!py_is(&year.clone(), &Py::None))) {
    let __try3 = exc::catch(|| {
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("year".to_string(), year.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&when.clone(), "replace", args))(&__args) };
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
    said = { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); py_string(__s) };
    }
    }
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    return Py::None;
    }
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    if py_truthy(&op_gt(&when.clone(), &today.clone())) {
    if py_truthy(&Py::Bool(!py_is(&year.clone(), &Py::None))) {
    return Py::None;
    }
    let __try8 = exc::catch(|| {
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("year".to_string(), op_sub(&gattr(&when.clone(), "year"), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&when.clone(), "replace", args))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret9: Py = Py::Bool(false);
    let mut __val10: Py = Py::None;
    let mut __unhandled11: Option<exc::PyExc> = None;
    match __try8 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret9 = __p[0].clone(); __val10 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled12 = false;
            if !__handled12 && exc::matches_any(&__e, &["ValueError"]) {
                __handled12 = true;
    return Py::None;
            }
            if !__handled12 { __unhandled11 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled11 { exc::raise(__e); }
    if py_truthy(&__ret9) { return __val10; }
    }
    days = gattr(&op_sub(&today.clone(), &when.clone()), "days");
    unit = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str("That was today - "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is today, so no days ago at all."); py_string(__s) };
    }
    count = _count_in(&[unit.clone(), days.clone()]);
    if py_truthy(&{ let __b = op_ge(&days.clone(), &Py::Int(1000)); if !py_truthy(&__b) { __b } else { mcall(&unit.clone(), "startswith", &[py_str("day")]) } }) {
    count = { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, &",")); __s.push_str(" days"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" since "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" - that was a "); __s.push_str(&fstr(&geti(&g__WEEKDAYS(), &mcall(&when.clone(), "weekday", &[])), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _a_known_day_the_line_names_next(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    let mut eve: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The next time the day the line names comes round, as a date and how to say it.");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    if py_truthy(&stdlib_call("re", "search", &[op_add(&g__CHRISTMAS_DAY(), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    eve = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\beve\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    when = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(12), if py_truthy(&eve.clone()) { Py::Int(24) } else { Py::Int(25) }]);
    if py_truthy(&op_lt(&when.clone(), &today.clone())) {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(12), if py_truthy(&eve.clone()) { Py::Int(24) } else { Py::Int(25) }]);
    }
    return py_tuple(vec![when.clone(), if py_truthy(&eve.clone()) { py_str("Christmas Eve") } else { py_str("Christmas") }]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bnew year(?:'s|s')? (?:day|eve)\\b|\\bnew year\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    when = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(1), Py::Int(1)]);
    if py_truthy(&op_lt(&when.clone(), &today.clone())) {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(1), Py::Int(1)]);
    }
    return py_tuple(vec![when.clone(), py_str("New Year's Day")]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\beaster\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    when = _easter_sunday(&[gattr(&today.clone(), "year")]);
    if py_truthy(&op_lt(&when.clone(), &today.clone())) {
    when = _easter_sunday(&[op_add(&gattr(&today.clone(), "year"), &Py::Int(1))]);
    }
    return py_tuple(vec![when.clone(), py_str("Easter Sunday")]);
    }
    return _date_the_line_gave(&[text.clone()]);
    Py::None
}
pub fn _a_working_week(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut unit: Py = Py::None;
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]), "strip", &[py_str("?.!")]), "split", &[])]);
    found = mcall(&g__WORKING_SHAPE(), "fullmatch", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[py_str("unit")]), mcall(&found.clone(), "group", &[py_str("week")])]), 2, None);
    unit = __unpk1[0].clone();
    span = __unpk1[1].clone();
    if py_truthy(&{ let __b = mcall(&unit.clone(), "startswith", &[py_str("hour")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&span.clone(), &py_str("week"))) } }) {
    return py_str("A full-time week is usually 37.5 to 40 hours - 7.5 or 8 hours a day over five days. The law caps the average at 48 hours a week over 17 weeks unless the person signs to opt out, and part-time hours are usually written as a share of that full-time week.");
    }
    if py_truthy(&{ let __b = mcall(&unit.clone(), "startswith", &[py_str("hour")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&span.clone(), &py_str("day"))) } }) {
    return py_str("A full-time day is usually 7.5 or 8 hours, and a break of 20 minutes after six hours is the legal minimum in the UK. Contracted hours are the ones that count for pay, so check the contract rather than the rota.");
    }
    if py_truthy(&{ let __b = mcall(&unit.clone(), "startswith", &[py_str("day")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&span.clone(), &py_str("week"))) } }) {
    return py_str("Five, for a full-time job - Monday to Friday - which is why a week off is five days of leave, not seven. Shift work and four-day weeks are the usual exceptions, and the contract says which one you are on.");
    }
    return Py::None;
    Py::None
}
pub fn _working_days_or_weekends_until(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut _name: Py = Py::None;
    let mut count: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut end: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut holidays: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut step: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut weekends: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The weekdays, or the whole weekends, between tomorrow and the day the line names.\n\n    A bank holiday is not a working day, so the England and Wales calendar comes out of it (the\n    line can say Scotland or Northern Ireland, and then that list is used). Weekends are counted\n    whole: a Saturday whose Sunday is the day itself is not a weekend off before it.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("how many (?:working|work|business) days? (?:in|per) a year"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("About 253 working days in a year: 261 weekdays, less the 8 bank holidays in England and Wales. Leap years add one.");
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("how many (?:working|work) hours? (?:in|per|to) (?:a|the) week"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A full-time week is usually 37.5 to 40 hours - 7.5 or 8 hours a day, five days. The law caps the average at 48 hours a week over 17 weeks unless the person signs to opt out, and part-time hours are usually written as a share of the full-time week.");
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("how many weekends (?:in|per) a year"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("104 weekend days in a year - 52 Saturdays and 52 Sundays - plus a day or two in most years, because 365 is not a whole number of weeks.");
    }
    found = { let __b = mcall(&g__WORKING_ASK(), "fullmatch", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__WORKING_LEFT(), "fullmatch", &[text.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    weekends = Py::Bool(py_contains(&mcall(&mcall(&found.clone(), "group", &[py_str("count")]), "lower", &[]), &py_str("weekend")));
    rest = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    period = { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("period")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&rest.clone()) {
    let __unpk1 = py_unpack(&_a_known_day_the_line_names_next(&[rest.clone()]), 2, None);
    end = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_is(&end.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:the\\s+|this\\s+)?(year|month)\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    period = if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonth\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { py_str("month") } else { py_str("year") };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&end.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { period.clone() } } } }) {
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("month")))) {
    end = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &op_floordiv(&gattr(&today.clone(), "month"), &Py::Int(12))), op_add(&op_modulo(&gattr(&today.clone(), "month"), &Py::Int(12)), &Py::Int(1)), Py::Int(1)]);
    let __aug2 = op_sub(&end.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    end = __aug2;
    said = { let mut __s = String::new(); __s.push_str("the end of "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&end.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    } else {
    end = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(12), Py::Int(31)]);
    said = { let mut __s = String::new(); __s.push_str("the end of "); __s.push_str(&fstr(&gattr(&today.clone(), "year"), -1, "")); py_string(__s) };
    }
    }
    days = gattr(&op_sub(&end.clone(), &today.clone()), "days");
    if py_truthy(&op_le(&days.clone(), &Py::Int(0))) {
    return Py::None;
    }
    span = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_add(&days.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut step: Py = __item.clone(); __out.push(op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })); } py_list(__out) })() };
    if py_truthy(&weekends.clone()) {
    count = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&span.clone()) { let mut day: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&day.clone(), "weekday", &[]), &Py::Int(5))); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&day.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }), &end.clone()) } }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return { let mut __s = String::new(); __s.push_str("There is no whole weekend between today and "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" - the next Saturday falls on "); __s.push_str(&fstr(&_said_with_a_weekday(&[end.clone()]), -1, "")); __s.push_str(" or later."); py_string(__s) };
    }
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&span.clone()) { let mut day: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&day.clone(), "weekday", &[]), &Py::Int(5)))) {  __out.push(day.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" weekend"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", the first starting on "); __s.push_str(&fstr(&_said_with_a_weekday(&[first.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    let __unpk3 = py_unpack(&_bank_where(&[{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]), 2, None);
    where_ = __unpk3[0].clone();
    _u = __unpk3[1].clone();
    holidays = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter4 = py_iter_or_raise(&op_bitor(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&span.clone()) { let mut day: Py = __item.clone(); __out.push(gattr(&day.clone(), "year")); } py_set(__out) })() }, &py_set(vec![gattr(&end.clone(), "year")])));
    for __item in __iter4 {
    year = __item.clone();
    let __aug5 = op_bitor(&holidays.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_bank_holidays(&[year.clone(), where_.clone()])) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut _name: Py = __unpk1[0].clone(); let mut day: Py = __unpk1[1].clone(); __out.push(day.clone()); } py_set(__out) })() });
    holidays = __aug5;
    }
    count = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&span.clone()) { let mut day: Py = __item.clone(); if py_truthy(&{ let __b = op_lt(&mcall(&day.clone(), "weekday", &[]), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&holidays.clone(), &day.clone())) } }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" working days until "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", out of "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days in all."); py_string(__s) };
    Py::None
}
pub fn _a_known_day_that_has_gone(args: &[Py]) -> Py {
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
    let mut year: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "year") { v.clone() }
    else { Py::None };
    let mut datetime: Py = Py::None;
    let mut named: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The last time a day the bot knows came round, for a count backwards.\n\n    \"How long ago was Christmas\" names a fixed date, so it can be counted the same way as a date\n    the line gives. Easter moves, so it is worked out from its own rule. A year in the line\n    (\"Christmas 2019\") is that year's Christmas, not the last one.\n    ");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    tail = if py_truthy(&year.clone()) { { let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); py_string(__s) } } else { py_str("") };
    named = stdlib_call("re", "search", &[op_add(&g__CHRISTMAS_DAY(), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&named.clone()) {
    when = mcall(&datetime.clone(), "date", &[{ let __b = year.clone(); if py_truthy(&__b) { __b } else { gattr(&today.clone(), "year") } }, Py::Int(12), Py::Int(25)]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&year.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&when.clone(), &today.clone()) } }) {
    when = mcall(&datetime.clone(), "date", &[op_sub(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(12), Py::Int(25)]);
    }
    return py_tuple(vec![when.clone(), { let mut __s = String::new(); __s.push_str("Christmas Day"); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) }]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bnew year(?:'s|s')? (?:day|eve)\\b|\\bnew year\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    when = mcall(&datetime.clone(), "date", &[{ let __b = year.clone(); if py_truthy(&__b) { __b } else { gattr(&today.clone(), "year") } }, Py::Int(1), Py::Int(1)]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&year.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&when.clone(), &today.clone()) } }) {
    when = mcall(&datetime.clone(), "date", &[op_sub(&gattr(&today.clone(), "year"), &Py::Int(1)), Py::Int(1), Py::Int(1)]);
    }
    return py_tuple(vec![when.clone(), { let mut __s = String::new(); __s.push_str("New Year's Day"); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) }]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\beaster (?:sunday|day)\\b|\\beaster\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    when = _easter_sunday(&[{ let __b = year.clone(); if py_truthy(&__b) { __b } else { gattr(&today.clone(), "year") } }]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&year.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&when.clone(), &today.clone()) } }) {
    when = _easter_sunday(&[op_sub(&gattr(&today.clone(), "year"), &Py::Int(1))]);
    }
    return py_tuple(vec![when.clone(), { let mut __s = String::new(); __s.push_str("Easter Sunday"); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) }]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _days_until_easter(args: &[Py]) -> Py {
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
    let mut ago: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut counting: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut first: Py = Py::None;
    let mut judging: Py = Py::None;
    let mut last: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut offset: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sense: Py = Py::None;
    let mut span: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut tense: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut waiting: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("When Easter and the days around it fall, only when the line asks for the date.\n\n    \"When is Easter\" is a date question; \"what should I cook for Easter\" is a cooking one.\n    \"Is Easter late this year\" asks where the date falls rather than for the day itself, so it\n    gets the day and the judgement together: Easter can fall anywhere from 22 March to 25 April,\n    and \"late\" means nothing without that range. The whole cleaned line has to be the date\n    question, so a mention in passing never counts. Because the day moves, it is worked out from\n    the rule each time rather than typed in.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    let __iter1 = py_iter_or_raise(&g__EASTER_OFFSETS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    subject = __unpk2[0].clone();
    offset = __unpk2[1].clone();
    name = __unpk2[2].clone();
    asked = { let mut __s = String::new(); __s.push_str("(?:(?:"); __s.push_str(&fstr(&g__ASK_DATE(), -1, "")); __s.push_str(")\\s+(?:the\\s+)?(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")(?:\\s+(?:start|starts|begin|begins))?|(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")\\s+(?:date|dates|20\\d{2}))"); py_string(__s) };
    waiting = { let mut __s = String::new(); __s.push_str("(?:how many |how long |how much longer )?(?:days?|weeks?|sleeps?) (?:is it |are there |left |to go |still )?(?:until|till|to|before) (?:the\\s+)?(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")"); py_string(__s) };
    judging = { let mut __s = String::new(); __s.push_str("(?:(?:is|was|will)|(?:"); __s.push_str(&fstr(&g__ASK_DATE(), -1, "")); __s.push_str("))\\s*(?:the\\s+)?(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")\\s+(?:going to be\\s+)?(?:(?:next year|this year|next easter|this easter|(?:in )?20\\d{2})\\s+)?(?:(?:early|late)\\s+or\\s+(?:early|late)|early|late|on the early side|on the late side)"); py_string(__s) };
    counting = call_builtin("bool", &[stdlib_call("re", "fullmatch", &[op_add(&waiting.clone(), &g__DATE_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    where_ = call_builtin("bool", &[stdlib_call("re", "fullmatch", &[op_add(&judging.clone(), &g__DATE_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&counting.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&where_.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[op_add(&asked.clone(), &g__DATE_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    continue;
    }
    let __unpk3 = py_unpack(&_feast_year(&[text.clone()]), 3, None);
    year = __unpk3[0].clone();
    named = __unpk3[1].clone();
    today = __unpk3[2].clone();
    day = op_add(&_easter_sunday(&[year.clone()]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), offset.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    if py_truthy(&{ let __b = op_lt(&day.clone(), &today.clone()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bthis (?:year|easter)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    day = op_add(&_easter_sunday(&[op_add(&year.clone(), &Py::Int(1))]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), offset.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    }
    said = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&day.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&day.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    if py_truthy(&where_.clone()) {
    first = op_add(&mcall(&datetime.clone(), "date", &[gattr(&day.clone(), "year"), Py::Int(3), Py::Int(22)]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), offset.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    last = op_add(&mcall(&datetime.clone(), "date", &[gattr(&day.clone(), "year"), Py::Int(4), Py::Int(25)]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), offset.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    span = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&first.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&first.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&gattr(&last.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&last.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    tense = if py_truthy(&op_lt(&day.clone(), &today.clone())) { py_str("was") } else { py_str("is") };
    if py_truthy(&op_le(&day.clone(), &op_add(&first.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(9))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }))) {
    sense = py_str("early");
    } else {
    if py_truthy(&op_ge(&day.clone(), &op_sub(&last.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(9))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) }))) {
    sense = py_str("late");
    } else {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&tense.clone(), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", which is about the middle of the range it can fall in ("); __s.push_str(&fstr(&span.clone(), -1, "")); __s.push_str(") - neither early nor late."); py_string(__s) };
    }
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&tense.clone(), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", which is "); __s.push_str(&fstr(&sense.clone(), -1, "")); __s.push_str(": it can fall anywhere from "); __s.push_str(&fstr(&span.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&{ let __b = counting.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { op_ge(&day.clone(), &today.clone()) } } } }) {
    days = gattr(&op_sub(&day.clone(), &today.clone()), "days");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&named.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&day.clone(), "year"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    days = gattr(&op_sub(&day.clone(), &today.clone()), "days");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    ago = op_neg(&days.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" this year was on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&ago.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&ago.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" ago."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _nth_weekday(args: &[Py]) -> Py {
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
    let mut weekday: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "weekday") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: weekday"))) };
    let mut nth: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "nth") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: nth"))) };
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let _ = py_str("The nth weekday of a month: Father's Day is the third Sunday in June.");
    datetime = stdlib_module("datetime");
    day = mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), Py::Int(1)]);
    let __aug1 = op_add(&day.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_modulo(&op_sub(&weekday.clone(), &mcall(&day.clone(), "weekday", &[])), &Py::Int(7)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    day = __aug1;
    return op_add(&day.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_mul(&Py::Int(7), &op_sub(&nth.clone(), &Py::Int(1))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    Py::None
}
pub fn _named_day_date(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut counting: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut days: Py = Py::None;
    let mut extra: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named_year: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut waiting: Py = Py::None;
    let mut when: Py = Py::None;
    let mut words: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("When one of the days above falls: \"when is mothers day\", \"how many days until halloween\".\n\n    The whole cleaned line has to be the date question for that day, so \"what do i write in a\n    mothers day card\" and \"do bank holidays count towards my holiday\" never land here. A year in\n    the line is answered with that year's date; without one, a day that has already gone this year\n    is answered as next year's, the way the Christmas countdown does it.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    let __iter1 = py_iter_or_raise(&g__NAMED_DAYS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    words = __unpk2[0].clone();
    rule = __unpk2[1].clone();
    name = __unpk2[2].clone();
    extra = __unpk2[3].clone();
    subject = mcall(&py_str("|"), "join", &[words.clone()]);
    asked = { let mut __s = String::new(); __s.push_str("(?:"); __s.push_str(&fstr(&g__ASK_DATE(), -1, "")); __s.push_str(")\\s+(?:the\\s+)?(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")"); py_string(__s) };
    waiting = { let mut __s = String::new(); __s.push_str("(?:how many |how long )?(?:days?|weeks?|sleeps?) (?:is it |are there |left |to go )?(?:until|till|to|before) (?:the\\s+)?(?:"); __s.push_str(&fstr(&subject.clone(), -1, "")); __s.push_str(")"); py_string(__s) };
    counting = call_builtin("bool", &[stdlib_call("re", "fullmatch", &[op_add(&waiting.clone(), &g__DATE_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&counting.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "fullmatch", &[op_add(&asked.clone(), &g__DATE_TAIL()), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    let __unpk3 = py_unpack(&_feast_year(&[text.clone()]), 3, None);
    year = __unpk3[0].clone();
    named_year = __unpk3[1].clone();
    today = __unpk3[2].clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&rule.clone(), &Py::Int(0)), &py_str("fixed")))) {
    when = mcall(&datetime.clone(), "date", &[year.clone(), geti(&rule.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(2))]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&geti(&rule.clone(), &Py::Int(0)), &py_str("easter")))) {
    when = op_add(&_easter_sunday(&[year.clone()]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), geti(&rule.clone(), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    } else {
    when = _nth_weekday(&[year.clone(), geti(&rule.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(2)), geti(&rule.clone(), &Py::Int(3))]);
    }
    }
    if py_truthy(&{ let __b = op_lt(&when.clone(), &today.clone()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named_year.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&text.clone(), &py_str("this year"))) } } } }) {
    if py_truthy(&Py::Bool(py_eq(&geti(&rule.clone(), &Py::Int(0)), &py_str("fixed")))) {
    when = mcall(&datetime.clone(), "date", &[op_add(&year.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(2))]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&geti(&rule.clone(), &Py::Int(0)), &py_str("easter")))) {
    when = op_add(&_easter_sunday(&[op_add(&year.clone(), &Py::Int(1))]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), geti(&rule.clone(), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    } else {
    when = _nth_weekday(&[op_add(&year.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(1)), geti(&rule.clone(), &Py::Int(2)), geti(&rule.clone(), &Py::Int(3))]);
    }
    }
    }
    said = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    if py_truthy(&named_year.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&when.clone(), "year"), -1, "")); __s.push_str("."); __s.push_str(&fstr(&extra.clone(), -1, "")); py_string(__s) };
    }
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today."); __s.push_str(&fstr(&extra.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&counting.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" this year was on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&op_neg(&days.clone()), -1, "")); __s.push_str(" days ago."); __s.push_str(&fstr(&extra.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is on "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away."); __s.push_str(&fstr(&extra.clone(), -1, "")); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _days_until_a_month_or_season(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut end: Py = Py::None;
    let mut eve: Py = Py::None;
    let mut found: Py = Py::None;
    let mut label: Py = Py::None;
    let mut last_month: Py = Py::None;
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut next_year: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut when: Py = Py::None;
    let mut words: Py = Py::None;
    let mut year: Py = Py::None;
    let mut year_said: Py = Py::None;
    let _ = py_str("The countdown to a bare month, a season, or New Year, or None.\n\n    The count is to the first day of what was named - December starts on 1 December, summer on\n    1 June - and it is said in the unit the line asked for, the same way the Christmas countdown\n    replies in weeks when it is asked in weeks. A month or season that has already started this\n    year counts to the next turn of it, because \"how many days until december\" asked on the 4th\n    of December is about the next December and not about a date three days behind.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    found = stdlib_call("re", "fullmatch", &[g__COUNT_ASK(), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    rest = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("rest2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the|my|our)\\s+")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:please|then|now|exactly|roughly|again|yet)$")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:start|starts|begin|begins|come|comes|arrive|arrives|get here)$")); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:holiday|holidays|break|term|half term|exam|exams|payday|pay day|deadline|due|start|starts|begin|begins|end|ends)\\b"), rest.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    unit = mcall(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("days") } }, "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__THE_TURN_OF_THE_YEAR(), &rest.clone()))) {
    eve = Py::Bool(py_contains(&rest.clone(), &py_str("eve")));
    when = if py_truthy(&eve.clone()) { mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(12), Py::Int(31)]) } else { mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), Py::Int(1), Py::Int(1)]) };
    if py_truthy(&op_lt(&when.clone(), &today.clone())) {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&when.clone(), "year"), &Py::Int(1)), gattr(&when.clone(), "month"), gattr(&when.clone(), "day")]);
    }
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    name = if py_truthy(&eve.clone()) { py_str("New Year's Eve") } else { py_str("New Year's Day") };
    said = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today, "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    after = if py_truthy(&Py::Bool(!py_truthy(&eve.clone()))) { py_str(" New Year's Eve is the night before it.") } else { py_str("") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), when.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" - it is "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); __s.push_str(&fstr(&after.clone(), -1, "")); py_string(__s) };
    }
    let __iter1 = py_iter_or_raise(&g__SEASONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    words = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    month = __unpk3[0].clone();
    day = __unpk3[1].clone();
    last_month = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &rest.clone()))) {
    continue;
    }
    start = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), month.clone(), day.clone()]);
    end = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), last_month.clone(), geti(&g_MONTHS(), &mcall(&geti(&g__MONTH_NAMES(), &op_sub(&last_month.clone(), &Py::Int(1))), "lower", &[]))]);
    if py_truthy(&op_lt(&end.clone(), &start.clone())) {
    end = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), last_month.clone(), geti(&g_MONTHS(), &mcall(&geti(&g__MONTH_NAMES(), &op_sub(&last_month.clone(), &Py::Int(1))), "lower", &[]))]);
    }
    name = mcall(&rest.clone(), "capitalize", &[]);
    if py_truthy(&{ let __cmp41: Py = today.clone(); if py_truthy(&op_le(&start.clone(), &__cmp41)) { op_le(&__cmp41, &end.clone()) } else { Py::Bool(false) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is here now - it runs to the end of "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&end.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(". The next one starts on "); __s.push_str(&fstr(&gattr(&start.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&start.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(" next year."); py_string(__s) };
    }
    if py_truthy(&op_lt(&start.clone(), &today.clone())) {
    start = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), day.clone()]);
    }
    days = gattr(&op_sub(&start.clone(), &today.clone()), "days");
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), start.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" - it starts on "); __s.push_str(&fstr(&gattr(&start.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&start.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str("."); py_string(__s) };
    }
    year_said = stdlib_call("re", "fullmatch", &[py_str("(?:(?:the\\s+)?year\\s+)?(20\\d{2}|21\\d{2})"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&year_said.clone()) {
    year = call_builtin("int", &[mcall(&year_said.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    when = mcall(&datetime.clone(), "date", &[year.clone(), Py::Int(1), Py::Int(1)]);
    if py_truthy(&op_le(&when.clone(), &today.clone())) {
    return Py::None;
    }
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), when.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str(" - it starts on "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&year.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    month = _month_number(&[rest.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&month.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MONTHS_SAID()) { let mut name: Py = __item.clone(); __out.push(slice_or_raise(&name.clone(), &Py::None, &Py::Int(3), &Py::None)); } py_list(__out) })() }, &rest.clone())) } }) {
    month = op_add(&mcall(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MONTHS_SAID()) { let mut name: Py = __item.clone(); __out.push(slice_or_raise(&name.clone(), &Py::None, &Py::Int(3), &Py::None)); } py_list(__out) })() }, "index", &[rest.clone()]), &Py::Int(1));
    }
    if py_truthy(&Py::Bool(py_is(&month.clone(), &Py::None))) {
    return Py::None;
    }
    when = mcall(&datetime.clone(), "date", &[gattr(&today.clone(), "year"), month.clone(), Py::Int(1)]);
    next_year = Py::Bool(false);
    if py_truthy(&op_lt(&when.clone(), &today.clone())) {
    when = mcall(&datetime.clone(), "date", &[op_add(&gattr(&today.clone(), "year"), &Py::Int(1)), month.clone(), Py::Int(1)]);
    next_year = Py::Bool(true);
    }
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    label = op_add(&geti(&g__MONTH_NAMES(), &op_sub(&month.clone(), &Py::Int(1))), &if py_truthy(&next_year.clone()) { py_str(" next year") } else { py_str("") });
    return { let mut __s = String::new(); __s.push_str(&fstr(&_count_in(&[unit.clone(), days.clone(), when.clone()]), -1, "")); __s.push_str(" until "); __s.push_str(&fstr(&label.clone(), -1, "")); __s.push_str(" - it starts on "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); __s.push_str(&fstr(&if py_truthy(&next_year.clone()) { op_add(&py_str(" "), &call_builtin("str", &[gattr(&when.clone(), "year")]).unwrap_or_else(|e| exc::raise(e))) } else { py_str("") }, -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _ordinal(args: &[Py]) -> Py {
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
    let _ = py_str("24 -> \"24th\", 1 -> \"1st\", 22 -> \"22nd\".");
    if py_truthy(&{ let __cmp11: Py = op_modulo(&number.clone(), &Py::Int(100)); if py_truthy(&op_le(&Py::Int(11), &__cmp11)) { op_le(&__cmp11, &Py::Int(13)) } else { Py::Bool(false) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str("th"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&number.clone(), -1, "")); __s.push_str(&fstr(&if py_truthy(&op_lt(&op_modulo(&number.clone(), &Py::Int(10)), &Py::Int(4))) { geti(&py_tuple(vec![py_str("th"), py_str("st"), py_str("nd"), py_str("rd")]), &op_modulo(&call_builtin("min", &[op_modulo(&number.clone(), &Py::Int(10)), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) } else { py_str("th") }, -1, "")); py_string(__s) };
    Py::None
}
pub fn _the_weekday_of_a_date(args: &[Py]) -> Py {
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
    let mut ago: Py = Py::None;
    let mut backwards: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut first: Py = Py::None;
    let mut following: Py = Py::None;
    let mut found: Py = Py::None;
    let mut guess: Py = Py::None;
    let mut month: Py = Py::None;
    let mut month_said: Py = Py::None;
    let mut next_month: Py = Py::None;
    let mut next_year: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_next: Py = Py::None;
    let mut second: Py = Py::None;
    let mut shift: Py = Py::None;
    let mut step: Py = Py::None;
    let mut text: Py = Py::None;
    let mut this_month: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let mut year: Py = Py::None;
    let mut year_found: Py = Py::None;
    let mut year_said: Py = Py::None;
    let mut years_back: Py = Py::None;
    let _ = py_str("The weekday a date falls on: \"what day is the 24th of december\".\n\n    Only the whole line counts, so \"i get paid on the 24th\" stays a plan. A line that names a\n    weekday as well (\"is the 24th a friday\") is a yes or no question and is left alone, as is a\n    line asking how many days it is until a date, which the countdown answers.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    said = stdlib_call("re", "match", &[py_str("^(?:what|which)\\s+(?:day|date|weekday)\\s+(?:of\\s+the\\s+week\\s+)?(?:is|was|will)\\s+(?:it\\s+)?(?:be\\s+)?(?:on\\s+)?(?:the\\s+)?"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = stdlib_call("re", "match", &[py_str("^what\\s+day\\s+does\\s+(?:the\\s+)?"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    backwards = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bwas\\b"), mcall(&said.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    rest = stdlib_call("re", "sub", &[py_str("\\s+(?:fall|falls|land|lands)\\s+on$"), py_str(""), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    year_said = Py::None;
    year_found = stdlib_call("re", "search", &[py_str("\\s+(?:in\\s+)?(20\\d{2})$"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&year_found.clone()) {
    year_said = call_builtin("int", &[mcall(&year_found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    rest = mcall(&slice_or_raise(&rest.clone(), &Py::None, &mcall(&year_found.clone(), "start", &[]), &Py::None), "strip", &[]);
    }
    rest = stdlib_call("re", "sub", &[py_str("\\s+(?:this|next)\\s+year$"), py_str(""), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g__SAYS_WEEKDAY(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    found = { let __b = stdlib_call("re", "fullmatch", &[{ let mut __s = String::new(); __s.push_str("(\\d{1,2})(?:st|nd|rd|th)?\\s+(?:of\\s+)?("); __s.push_str(&fstr(&g__MONTHS_OR(), -1, "")); __s.push_str(")"); py_string(__s) }, rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "fullmatch", &[{ let mut __s = String::new(); __s.push_str("("); __s.push_str(&fstr(&g__MONTHS_OR(), -1, "")); __s.push_str(")\\s+(\\d{1,2})(?:st|nd|rd|th)?"); py_string(__s) }, rest.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&found.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)])]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    let __unpk2 = py_unpack(&if py_truthy(&mcall(&first.clone(), "isdigit", &[])) { py_tuple(vec![call_builtin("int", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), second.clone()]) } else { py_tuple(vec![call_builtin("int", &[second.clone()]).unwrap_or_else(|e| exc::raise(e)), first.clone()]) }, 2, None);
    day = __unpk2[0].clone();
    month = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_is(&_month_number(&[month.clone()]), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&year_said.clone()) {
    let __try3 = exc::catch(|| {
    when = mcall(&datetime.clone(), "date", &[year_said.clone(), _month_number(&[month.clone()]), day.clone()]);
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
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&when.clone(), -1, &"%B")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&when.clone(), "year"), -1, "")); __s.push_str(" is a "); __s.push_str(&fstr(&when.clone(), -1, &"%A")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&backwards.clone()) {
    when = Py::None;
    let __iter8 = py_iter_or_raise(&call_builtin("range", &[Py::Int(2)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter8 {
    years_back = __item.clone();
    let __try9 = exc::catch(|| {
    guess = mcall(&datetime.clone(), "date", &[op_sub(&gattr(&today.clone(), "year"), &years_back.clone()), _month_number(&[month.clone()]), day.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret10: Py = Py::Bool(false);
    let mut __val11: Py = Py::None;
    let mut __unhandled12: Option<exc::PyExc> = None;
    match __try9 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret10 = __p[0].clone(); __val11 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled13 = false;
            if !__handled13 && exc::matches_any(&__e, &["ValueError"]) {
                __handled13 = true;
    continue;
            }
            if !__handled13 { __unhandled12 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled12 { exc::raise(__e); }
    if py_truthy(&__ret10) { return __val11; }
    if py_truthy(&op_le(&guess.clone(), &today.clone())) {
    when = guess.clone();
    break;
    }
    }
    } else {
    when = _day_in_a_month(&[day.clone(), _month_number(&[month.clone()]), today.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&when.clone(), &Py::None))) {
    return Py::None;
    }
    days = gattr(&op_sub(&when.clone(), &today.clone()), "days");
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    ago = op_neg(&days.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&when.clone(), -1, &"%B")); __s.push_str(" was a "); __s.push_str(&fstr(&when.clone(), -1, &"%A")); __s.push_str(", "); __s.push_str(&fstr(&ago.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&ago.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" ago."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&when.clone(), -1, &"%B")); __s.push_str(" is today, a "); __s.push_str(&fstr(&when.clone(), -1, &"%A")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&when.clone(), -1, &"%B")); __s.push_str(" is tomorrow, a "); __s.push_str(&fstr(&when.clone(), -1, &"%A")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&when.clone(), -1, &"%B")); __s.push_str(" is a "); __s.push_str(&fstr(&when.clone(), -1, &"%A")); __s.push_str(", "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away."); py_string(__s) };
    }
    bare = stdlib_call("re", "fullmatch", &[{ let mut __s = String::new(); __s.push_str("(\\d{1,2})(?:st|nd|rd|th)?"); py_string(__s) }, rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    day = call_builtin("int", &[mcall(&bare.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    this_month = Py::None;
    let __iter14 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter14 {
    step = __item.clone();
    shift = if py_truthy(&backwards.clone()) { op_neg(&step.clone()) } else { step.clone() };
    month = op_add(&op_modulo(&op_add(&op_sub(&gattr(&today.clone(), "month"), &Py::Int(1)), &shift.clone()), &Py::Int(12)), &Py::Int(1));
    year = op_add(&gattr(&today.clone(), "year"), &op_floordiv(&op_add(&op_sub(&gattr(&today.clone(), "month"), &Py::Int(1)), &shift.clone()), &Py::Int(12)));
    let __try15 = exc::catch(|| {
    guess = mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), day.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret16: Py = Py::Bool(false);
    let mut __val17: Py = Py::None;
    let mut __unhandled18: Option<exc::PyExc> = None;
    match __try15 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret16 = __p[0].clone(); __val17 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled19 = false;
            if !__handled19 && exc::matches_any(&__e, &["ValueError"]) {
                __handled19 = true;
    continue;
            }
            if !__handled19 { __unhandled18 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled18 { exc::raise(__e); }
    if py_truthy(&__ret16) { return __val17; }
    if py_truthy(&{ let __b = backwards.clone(); if !py_truthy(&__b) { __b } else { op_gt(&guess.clone(), &today.clone()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&backwards.clone())); if !py_truthy(&__b) { __b } else { op_lt(&guess.clone(), &today.clone()) } }) {
    continue;
    }
    this_month = guess.clone();
    break;
    }
    if py_truthy(&Py::Bool(py_is(&this_month.clone(), &Py::None))) {
    return Py::None;
    }
    next_month = op_add(&op_modulo(&gattr(&today.clone(), "month"), &Py::Int(12)), &Py::Int(1));
    next_year = op_add(&gattr(&today.clone(), "year"), &if py_truthy(&Py::Bool(py_eq(&gattr(&today.clone(), "month"), &Py::Int(12)))) { Py::Int(1) } else { Py::Int(0) });
    let __try20 = exc::catch(|| {
    following = mcall(&datetime.clone(), "date", &[next_year.clone(), next_month.clone(), day.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret21: Py = Py::Bool(false);
    let mut __val22: Py = Py::None;
    let mut __unhandled23: Option<exc::PyExc> = None;
    match __try20 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret21 = __p[0].clone(); __val22 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled24 = false;
            if !__handled24 && exc::matches_any(&__e, &["ValueError"]) {
                __handled24 = true;
    following = Py::None;
            }
            if !__handled24 { __unhandled23 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled23 { exc::raise(__e); }
    if py_truthy(&__ret21) { return __val22; }
    if py_truthy(&{ let __b = backwards.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&following.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&gattr(&following.clone(), "month"), &gattr(&this_month.clone(), "month"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&gattr(&following.clone(), "year"), &gattr(&this_month.clone(), "year"))) } } } } } }) {
    said_next = py_str("");
    } else {
    said_next = { let mut __s = String::new(); __s.push_str(", and the "); __s.push_str(&fstr(&_ordinal(&[day.clone()]), -1, "")); __s.push_str(" of next month is a "); __s.push_str(&fstr(&following.clone(), -1, &"%A")); py_string(__s) };
    }
    days = gattr(&op_sub(&this_month.clone(), &today.clone()), "days");
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str("Today is the "); __s.push_str(&fstr(&_ordinal(&[day.clone()]), -1, "")); __s.push_str(", a "); __s.push_str(&fstr(&this_month.clone(), -1, &"%A")); __s.push_str(&fstr(&said_next.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    month_said = if py_truthy(&Py::Bool(py_eq(&gattr(&this_month.clone(), "month"), &gattr(&today.clone(), "month")))) { py_str("this month") } else { mcall(&this_month.clone(), "strftime", &[py_str("%B")]) };
    if py_truthy(&op_lt(&days.clone(), &Py::Int(0))) {
    ago = op_neg(&days.clone());
    return { let mut __s = String::new(); __s.push_str("The "); __s.push_str(&fstr(&_ordinal(&[day.clone()]), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&month_said.clone(), -1, "")); __s.push_str(" was a "); __s.push_str(&fstr(&this_month.clone(), -1, &"%A")); __s.push_str(", "); __s.push_str(&fstr(&ago.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&ago.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" ago."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("The "); __s.push_str(&fstr(&_ordinal(&[day.clone()]), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&month_said.clone(), -1, "")); __s.push_str(" is a "); __s.push_str(&fstr(&this_month.clone(), -1, &"%A")); __s.push_str(", "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" days away"); __s.push_str(&fstr(&said_next.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _the_wider_day(args: &[Py]) -> Py {
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
    let mut taken: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "taken") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: taken"))) };
    let mut datetime: Py = Py::None;
    let _ = py_str("A fixed bank holiday that lands at the weekend is taken on the next free weekday.");
    datetime = stdlib_module("datetime");
    while py_truthy(&{ let __b = op_gt(&mcall(&day.clone(), "weekday", &[]), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&taken.clone(), &day.clone())) } }) {
    let __aug1 = op_add(&day.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    day = __aug1;
    }
    return day.clone();
    Py::None
}
pub fn _last_weekday(args: &[Py]) -> Py {
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
    let mut weekday: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "weekday") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: weekday"))) };
    let mut datetime: Py = Py::None;
    let mut last: Py = Py::None;
    let _ = py_str("The last weekday of a month: the summer bank holiday is the last Monday in August.");
    datetime = stdlib_module("datetime");
    if py_truthy(&Py::Bool(py_eq(&month.clone(), &Py::Int(12)))) {
    last = mcall(&datetime.clone(), "date", &[year.clone(), Py::Int(12), Py::Int(31)]);
    } else {
    last = op_sub(&mcall(&datetime.clone(), "date", &[year.clone(), op_add(&month.clone(), &Py::Int(1)), Py::Int(1)]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    }
    return op_sub(&last.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_modulo(&op_sub(&mcall(&last.clone(), "weekday", &[]), &weekday.clone()), &Py::Int(7)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    Py::None
}
pub fn _bank_holidays(args: &[Py]) -> Py {
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
    let mut where_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "where") { v.clone() }
    else { py_str("england") };
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut easter: Py = Py::None;
    let mut fixed_days: Py = Py::None;
    let mut month: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut name: Py = Py::None;
    let mut rows: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("Every bank holiday in one year, as (name, date), in order.\n\n    The three calendars are not the same list: Scotland has 2 January, St Andrew's Day and the\n    first Monday of August rather than the last, and Northern Ireland has St Patrick's Day and the\n    Battle of the Boyne. All of them are computable from the same pieces: the fixed dates (moved to\n    the next free weekday when they land at a weekend), the two that hang off Easter, and the\n    Mondays of a named week of a month.\n    ");
    datetime = stdlib_module("datetime");
    easter = _easter_sunday(&[year.clone()]);
    if py_truthy(&Py::Bool(py_eq(&where_.clone(), &py_str("scotland")))) {
    fixed_days = py_tuple(vec![py_tuple(vec![py_str("New Year's Day"), Py::Int(1), Py::Int(1)]), py_tuple(vec![py_str("2 January"), Py::Int(1), Py::Int(2)]), py_tuple(vec![py_str("St Andrew's Day"), Py::Int(11), Py::Int(30)]), py_tuple(vec![py_str("Christmas Day"), Py::Int(12), Py::Int(25)]), py_tuple(vec![py_str("Boxing Day"), Py::Int(12), Py::Int(26)])]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&where_.clone(), &py_str("northern ireland")))) {
    fixed_days = py_tuple(vec![py_tuple(vec![py_str("New Year's Day"), Py::Int(1), Py::Int(1)]), py_tuple(vec![py_str("St Patrick's Day"), Py::Int(3), Py::Int(17)]), py_tuple(vec![py_str("the Battle of the Boyne"), Py::Int(7), Py::Int(12)]), py_tuple(vec![py_str("Christmas Day"), Py::Int(12), Py::Int(25)]), py_tuple(vec![py_str("Boxing Day"), Py::Int(12), Py::Int(26)])]);
    } else {
    fixed_days = py_tuple(vec![py_tuple(vec![py_str("New Year's Day"), Py::Int(1), Py::Int(1)]), py_tuple(vec![py_str("Christmas Day"), Py::Int(12), Py::Int(25)]), py_tuple(vec![py_str("Boxing Day"), Py::Int(12), Py::Int(26)])]);
    }
    }
    taken = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    rows = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&fixed_days.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    month = __unpk2[1].clone();
    day = __unpk2[2].clone();
    moved = _the_wider_day(&[mcall(&datetime.clone(), "date", &[year.clone(), month.clone(), day.clone()]), taken.clone()]);
    let _ = mcall(&taken.clone(), "add", &[moved.clone()]);
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![name.clone(), moved.clone()])]);
    }
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("Good Friday"), op_sub(&easter.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })])]);
    if py_truthy(&Py::Bool(!py_eq(&where_.clone(), &py_str("scotland")))) {
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("Easter Monday"), op_add(&easter.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) })])]);
    }
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("the early May bank holiday"), _nth_weekday(&[year.clone(), Py::Int(5), Py::Int(0), Py::Int(1)])])]);
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("the spring bank holiday"), _last_weekday(&[year.clone(), Py::Int(5), Py::Int(0)])])]);
    if py_truthy(&Py::Bool(py_eq(&where_.clone(), &py_str("scotland")))) {
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("the summer bank holiday"), _nth_weekday(&[year.clone(), Py::Int(8), Py::Int(0), Py::Int(1)])])]);
    } else {
    let _ = mcall(&rows.clone(), "append", &[py_tuple(vec![py_str("the summer bank holiday"), _last_weekday(&[year.clone(), Py::Int(8), Py::Int(0)])])]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rows.clone()); __kws.extend(vec![("key".to_string(), { let datetime = datetime.clone(); let day = day.clone(); let easter = easter.clone(); let fixed_days = fixed_days.clone(); let month = month.clone(); let moved = moved.clone(); let name = name.clone(); let rows = rows.clone(); let taken = taken.clone(); let where_ = where_.clone(); let year = year.clone(); py_func("lambda", move |args: &[Py]| { let datetime = datetime.clone(); let day = day.clone(); let easter = easter.clone(); let fixed_days = fixed_days.clone(); let month = month.clone(); let moved = moved.clone(); let name = name.clone(); let rows = rows.clone(); let taken = taken.clone(); let where_ = where_.clone(); let year = year.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); geti(&row.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _bank_where(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut where_: Py = Py::None;
    let __iter1 = py_iter_or_raise(&g__BANK_WHERE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    pattern = __unpk2[0].clone();
    where_ = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])) {
    return py_tuple(vec![where_.clone(), said.clone()]);
    }
    }
    return py_tuple(vec![py_str("england"), py_str("in England and Wales")]);
    Py::None
}
pub fn _said_with_a_weekday(args: &[Py]) -> Py {
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
    let mut datetime: Py = Py::None;
    datetime = stdlib_module("datetime");
    return { let mut __s = String::new(); __s.push_str(&fstr(&geti(&g__WEEKDAYS(), &mcall(&when.clone(), "weekday", &[])), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&when.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_NAMES(), &op_sub(&gattr(&when.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    Py::None
}
pub fn _next_bank_holiday(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut ahead: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut counting: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut day: Py = Py::None;
    let mut days: Py = Py::None;
    let mut left: Py = Py::None;
    let mut listing: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named_year: Py = Py::None;
    let mut row: Py = Py::None;
    let mut rows: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut told_where: Py = Py::None;
    let mut when: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The next bank holiday, its countdown, or the year's list: \"when is the next bank holiday\".");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    asking = mcall(&g__BANK_HOLIDAY_NEXT(), "fullmatch", &[text.clone()]);
    counting = mcall(&g__BANK_HOLIDAY_COUNT(), "fullmatch", &[text.clone()]);
    listing = mcall(&g__BANK_HOLIDAY_LIST(), "fullmatch", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = asking.clone(); if py_truthy(&__b) { __b } else { { let __b = counting.clone(); if py_truthy(&__b) { __b } else { listing.clone() } } } }))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_feast_year(&[text.clone()]), 3, None);
    year = __unpk1[0].clone();
    named_year = __unpk1[1].clone();
    today = __unpk1[2].clone();
    let __unpk2 = py_unpack(&_bank_where(&[text.clone()]), 2, None);
    where_ = __unpk2[0].clone();
    told_where = __unpk2[1].clone();
    rows = _bank_holidays(&[year.clone(), where_.clone()]);
    ahead = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rows.clone()) { let mut row: Py = __item.clone(); if py_truthy(&op_ge(&geti(&row.clone(), &Py::Int(1)), &today.clone())) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&ahead.clone()))) {
    let __aug3 = op_add(&year.clone(), &Py::Int(1));
    year = __aug3;
    rows = _bank_holidays(&[year.clone(), where_.clone()]);
    ahead = rows.clone();
    }
    when = geti(&ahead.clone(), &Py::Int(0));
    after = if py_truthy(&op_gt(&call_builtin("len", &[ahead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&ahead.clone(), &Py::Int(1)) } else { geti(&_bank_holidays(&[op_add(&year.clone(), &Py::Int(1)), where_.clone()]), &Py::Int(0)) };
    days = gattr(&op_sub(&geti(&when.clone(), &Py::Int(1)), &today.clone()), "days");
    if py_truthy(&{ let __b = listing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&counting.clone())) } } } }) {
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rows.clone()) { let mut row: Py = __item.clone(); if py_truthy(&op_ge(&geti(&row.clone(), &Py::Int(1)), &today.clone())) {  __out.push(row.clone()); } } py_list(__out) })() };
    said = mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk1[0].clone(); let mut day: Py = __unpk1[1].clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&_said_with_a_weekday(&[day.clone()]), -1, "")); py_string(__s) }); } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_eq(&where_.clone(), &py_str("england")))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&slice_or_raise(&told_where.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&told_where.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" the bank holidays left this year are "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("In England and Wales the bank holidays left this year are "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(". Scotland and Northern Ireland keep a few of their own as well."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str("Today is a bank holiday "); __s.push_str(&fstr(&told_where.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&mcall(&slice_or_raise(&geti(&when.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&geti(&when.clone(), &Py::Int(0)), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&_said_with_a_weekday(&[geti(&when.clone(), &Py::Int(1))]), -1, "")); __s.push_str(". The next one after that is "); __s.push_str(&fstr(&geti(&after.clone(), &Py::Int(0)), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&_said_with_a_weekday(&[geti(&after.clone(), &Py::Int(1))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&counting.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" until the next bank holiday "); __s.push_str(&fstr(&told_where.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&geti(&when.clone(), &Py::Int(0)), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&_said_with_a_weekday(&[geti(&when.clone(), &Py::Int(1))]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("The next bank holiday "); __s.push_str(&fstr(&told_where.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&geti(&when.clone(), &Py::Int(0)), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&_said_with_a_weekday(&[geti(&when.clone(), &Py::Int(1))]), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&days.clone(), -1, "")); __s.push_str(" day"); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, -1, "")); __s.push_str(" away. After that it is "); __s.push_str(&fstr(&geti(&after.clone(), &Py::Int(0)), -1, "")); __s.push_str(" on "); __s.push_str(&fstr(&_said_with_a_weekday(&[geti(&after.clone(), &Py::Int(1))]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _day_text(args: &[Py]) -> Py {
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
    let _ = py_str("A date the way a friend says it: the weekday first, then the day and the month.");
    return { let mut __s = String::new(); __s.push_str(&fstr(&day.clone(), -1, &"%A")); __s.push_str(", "); __s.push_str(&fstr(&gattr(&day.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_TITLE(), &op_sub(&gattr(&day.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    Py::None
}
pub fn _short_date(args: &[Py]) -> Py {
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
    let _ = py_str("Just the day and the month, for when the weekday is already in the sentence.");
    return { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&day.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g__MONTH_TITLE(), &op_sub(&gattr(&day.clone(), "month"), &Py::Int(1))), -1, "")); py_string(__s) };
    Py::None
}
pub fn _weekday_of(args: &[Py]) -> Py {
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
    let mut tail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The weekday the line names, its index, and whether it said \"next\".");
    let __iter1 = py_iter_or_raise(&g__WEEKDAY_WORDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    words = __unpk2[0].clone();
    index = __unpk2[1].clone();
    name = __unpk2[2].clone();
    if py_truthy(&stdlib_call("re", "fullmatch", &[{ let mut __s = String::new(); __s.push_str(&fstr(&tail.clone(), -1, "")); __s.push_str("(?:(?:the|this|next|coming|on|a week on)\\s+)*(?:"); __s.push_str(&fstr(&words.clone(), -1, "")); __s.push_str(")(?:\\s+(?:today|now|already|yet))*"); py_string(__s) }, text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![name.clone(), index.clone(), call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bnext\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return py_tuple(vec![Py::None, Py::None, Py::Bool(false)]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut _u: Py = Py::None;
    let mut ahead: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut datetime: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut next_one: Py = Py::None;
    let mut text: Py = Py::None;
    let mut today: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("Today's date, and when the next Monday is.\n\n    \"What day is it today\" is arithmetic on the clock, not knowledge, so it is worked out\n    rather than guessed: the bot already counts days to Christmas the same way. The whole\n    cleaned line has to be the question, so \"what day is christmas\" stays a date question\n    about Christmas and \"what day suits you\" stays a plan.\n    ");
    datetime = stdlib_module("datetime");
    text = _plain(&[clean.clone()]);
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[g__TODAY_ASK(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __s = String::new(); __s.push_str("It's "); __s.push_str(&fstr(&_day_text(&[today.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    let __unpk1 = py_unpack(&_weekday_of(&[text.clone(), py_str("is (?:it|today) (?:still )?")]), 3, None);
    name = __unpk1[0].clone();
    index = __unpk1[1].clone();
    _u = __unpk1[2].clone();
    if py_truthy(&name.clone()) {
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &mcall(&today.clone(), "weekday", &[])))) {
    return { let mut __s = String::new(); __s.push_str("Yes, it's "); __s.push_str(&fstr(&_day_text(&[today.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("No, it's "); __s.push_str(&fstr(&_day_text(&[today.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    ask = py_str("(?:how many (?:days?|sleeps?) (?:is it |are there |do i have )?(?:until|till|til|to go before|before|to) |how long (?:until|till|til|to) |what date is |what date is it on |what day is |what day is it on |which date is |what day does |what date does |what(?:'s|s| is) the date(?: of| on| next| this)? |what(?:'s|s| is) the (?:day|date) of |when(?:'s|s) |when is |when does )");
    let __unpk2 = py_unpack(&_weekday_of(&[text.clone(), ask.clone()]), 3, None);
    name = __unpk2[0].clone();
    index = __unpk2[1].clone();
    next_one = __unpk2[2].clone();
    if py_truthy(&name.clone()) {
    ahead = op_modulo(&op_sub(&index.clone(), &mcall(&today.clone(), "weekday", &[])), &Py::Int(7));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&ahead.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { next_one.clone() } }) {
    ahead = Py::Int(7);
    }
    when = op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), ahead.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&datetime.clone(), "timedelta", args))(&__args) });
    if py_truthy(&Py::Bool(py_eq(&ahead.clone(), &Py::Int(0)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is today, "); __s.push_str(&fstr(&_short_date(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&ahead.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is tomorrow, "); __s.push_str(&fstr(&_short_date(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&ahead.clone(), &Py::Int(7)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is a week away, on "); __s.push_str(&fstr(&_short_date(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&ahead.clone(), -1, "")); __s.push_str(" days away, on "); __s.push_str(&fstr(&_short_date(&[when.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut text: Py = Py::None;
    let _ = py_str("The line without politeness and filler, so a question can be matched whole.");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "strip", &[py_str("?.!")]);
    text = stdlib_call("re", "sub", &[py_str("\\s*[,;]\\s*"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[{ let mut __s = String::new(); __s.push_str("^(?:"); __s.push_str(&fstr(&g__POLITE(), -1, "")); __s.push_str("\\b[\\s]+)+"); py_string(__s) }, py_str(""), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    text = stdlib_call("re", "sub", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g__FILLER(), -1, "")); __s.push_str("\\b"); py_string(__s) }, py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    Py::None
}
pub fn _unit_pair(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut counted: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "counted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: counted"))) };
    let mut of_unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "of_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: of_unit"))) };
    let _ = py_str("The fact for this pair, whichever way round the question asked it.");
    return { let __b = mcall(&g_FACTS(), "get", &[py_tuple(vec![counted.clone(), of_unit.clone()])]); if py_truthy(&__b) { __b } else { mcall(&g_FACTS(), "get", &[py_tuple(vec![of_unit.clone(), counted.clone()])]) } };
    Py::None
}
pub fn _unit_fact(args: &[Py]) -> Py {
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
    let mut counted: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut of_unit: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("How many of one unit make another, when that is the question asked.\n\n    The question has to be the whole line: \"how many hours in a week\" is the fact, and\n    \"how many hours a week should I work out\" is a training plan that must not be\n    answered with 168. This is why these facts are computed here instead of being\n    entries: an entry matches words anywhere in a line, and words like \"hours\" and\n    \"week\" turn up in questions that are not about arithmetic at all.\n    ");
    text = _plain(&[clean.clone()]);
    match_ = stdlib_call("re", "fullmatch", &[g__HEAVY(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    return mcall(&g_HOW_HEAVY(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    }
    match_ = stdlib_call("re", "fullmatch", &[g__HOW_MUCH(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&g__UNIT(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]), mcall(&g__UNIT(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(2)])])]), 2, None);
    counted = __unpk1[0].clone();
    of_unit = __unpk1[1].clone();
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { of_unit.clone() } }) {
    return _unit_pair(&[counted.clone(), of_unit.clone()]);
    }
    return Py::None;
    }
    match_ = stdlib_call("re", "fullmatch", &[g__AMOUNT(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    counted = mcall(&g__UNIT(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(!py_truthy(&counted.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&g__TAILS());
    for __item in __iter2 {
    pattern = __item.clone();
    tail = stdlib_call("re", "fullmatch", &[pattern.clone(), mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&tail.clone()) {
    of_unit = mcall(&g__UNIT(), "get", &[mcall(&tail.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&of_unit.clone()) {
    return _unit_pair(&[counted.clone(), of_unit.clone()]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _hours_a_week(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The hours the line says they work: \"37.5 hours a week\", \"40 hours\". 0 when it says none.");
    let __iter1 = py_iter_or_raise(&mcall(&g__HOURS_WORKED(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    let __try2 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&{ let __cmp71: Py = value.clone(); if py_truthy(&op_le(&Py::Int(5), &__cmp71)) { op_le(&__cmp71, &Py::Int(70)) } else { Py::Bool(false) } }) {
    return value.clone();
    }
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _hourly_named(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The rate the line states, in pounds and pence: \"12.50 an hour\" is not £12.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&g__WAGE(), &py_str("\\s*([^.;!?]{0,18})")), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    tail = op_add(&py_str(" "), &mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AN_HOUR()) { let mut unit: Py = __item.clone(); __out.push(Py::Bool(py_contains(&tail.clone(), &unit.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __try2 = exc::catch(|| {
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    return if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) { op_mul(&amount.clone(), &Py::Int(1000)) } else { amount.clone() };
    }
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _yearly_named(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("A bare salary: \"on 25k\", \"i earn 45000\". A figure with a rate unit after it is not one.");
    best = Py::Float(0.0);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[g__WAGE(), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    let __try2 = exc::catch(|| {
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) {
    let __aug7 = op_mul(&amount.clone(), &Py::Int(1000));
    amount = __aug7;
    }
    if py_truthy(&mcall(&g__WAGE_UNIT(), "match", &[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(16)), &Py::None)])) {
    continue;
    }
    if py_truthy(&{ let __b = op_ge(&amount.clone(), &Py::Int(1000)); if !py_truthy(&__b) { __b } else { op_gt(&amount.clone(), &best.clone()) } }) {
    best = amount.clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _pounds(args: &[Py]) -> Py {
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
    return { let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), -1, &",")); py_string(__s) };
    Py::None
}
pub fn _rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut hours: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "hours") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: hours"))) };
    let _ = py_str("37.5 stays 37.5 and 12.50 stays 12.50: pence in a rate are not noise.");
    return if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&hours.clone(), &call_builtin("round", &[hours.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) { { let mut __s = String::new(); __s.push_str(&fstr(&hours.clone(), -1, &".2f")); py_string(__s) } } else { call_builtin("str", &[call_builtin("int", &[call_builtin("round", &[hours.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) };
    Py::None
}
pub fn _a_yearline(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut scale: Py = Py::None;
    let _ = py_str("The yearly figure the line names, from \"30000 a year\", \"2200 a month\" or a bare \"25k\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![op_add(&g__WAGE(), &py_str("\\s*(?:a|per|each|every)\\s*year")), Py::Int(1)]), py_tuple(vec![op_add(&g__WAGE(), &py_str("\\s*(?:a|per|each|every)\\s*month")), Py::Int(12)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    scale = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "finditer", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    match_ = __item.clone();
    let __try4 = exc::catch(|| {
    amount = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret5: Py = Py::Bool(false);
    let mut __val6: Py = Py::None;
    let mut __unhandled7: Option<exc::PyExc> = None;
    match __try4 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret5 = __p[0].clone(); __val6 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled8 = false;
            if !__handled8 && exc::matches_any(&__e, &["ValueError"]) {
                __handled8 = true;
    continue;
            }
            if !__handled8 { __unhandled7 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled7 { exc::raise(__e); }
    if py_truthy(&__ret5) { return __val6; }
    if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(2)])) {
    let __aug9 = op_mul(&amount.clone(), &Py::Int(1000));
    amount = __aug9;
    }
    return op_mul(&amount.clone(), &scale.clone());
    }
    }
    return Py::Float(0.0);
    Py::None
}
pub fn _wage_rate(args: &[Py]) -> Py {
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
    let mut asks_a_year: Py = Py::None;
    let mut hourly: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut year: Py = Py::None;
    let mut yearly: Py = Py::None;
    let _ = py_str("A salary in hours, or an hourly rate in a year. None when the line is not that sum.");
    hours = { let __b = _hours_a_week(&[text.clone()]); if py_truthy(&__b) { __b } else { Py::Float(37.5) } };
    rate = _hourly_named(&[text.clone()]);
    yearly = { let __b = _a_yearline(&[text.clone()]); if py_truthy(&__b) { __b } else { _yearly_named(&[text.clone()]) } };
    asks_a_year = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:a year|per year|annual|annually|yearly|salary)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&yearly.clone())); if !py_truthy(&__b) { __b } else { op_le(&rate.clone(), &Py::Int(500)) } } } }) {
    year = op_mul(&op_mul(&rate.clone(), &hours.clone()), &Py::Int(52));
    return { let mut __s = String::new(); __s.push_str("£"); __s.push_str(&fstr(&_rate(&[rate.clone()]), -1, "")); __s.push_str(" an hour over "); __s.push_str(&fstr(&_rate(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is about £"); __s.push_str(&fstr(&_pounds(&[year.clone()]), -1, "")); __s.push_str(" a year before tax, £"); __s.push_str(&fstr(&_pounds(&[op_truediv(&year.clone(), &Py::Int(12))]), -1, "")); __s.push_str(" a month. Over 40 hours a week the same rate is £"); __s.push_str(&fstr(&_pounds(&[op_mul(&op_mul(&rate.clone(), &Py::Int(40)), &Py::Int(52))]), -1, "")); __s.push_str(", and paid holiday is already in the 52 weeks."); py_string(__s) };
    }
    if py_truthy(&op_ge(&yearly.clone(), &Py::Int(1000))) {
    hourly = op_truediv(&yearly.clone(), &op_mul(&hours.clone(), &Py::Int(52)));
    said = { let mut __s = String::new(); __s.push_str("£"); __s.push_str(&fstr(&_pounds(&[yearly.clone()]), -1, "")); __s.push_str(" a year over "); __s.push_str(&fstr(&_rate(&[hours.clone()]), -1, "")); __s.push_str(" hours a week is about £"); __s.push_str(&fstr(&_number(&[call_builtin("round", &[hourly.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" an hour before tax, £"); __s.push_str(&fstr(&_pounds(&[op_truediv(&yearly.clone(), &Py::Int(12))]), -1, "")); __s.push_str(" a month, £"); __s.push_str(&fstr(&_pounds(&[op_truediv(&yearly.clone(), &Py::Int(52))]), -1, "")); __s.push_str(" a week."); py_string(__s) };
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&hours.clone(), &Py::Int(40))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.01))) {
    let __aug1 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" Over 40 hours a week it is £"); __s.push_str(&fstr(&_number(&[call_builtin("round", &[op_truediv(&yearly.clone(), &op_mul(&Py::Int(40), &Py::Int(52))), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str("."); py_string(__s) });
    said = __aug1;
    }
    return op_add(&said.clone(), &py_str(" Take off any unpaid lunch break first: you are only paid for the hours you work."));
    }
    if py_truthy(&{ let __b = asks_a_year.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:hourly|hour rate|an hour|per hour)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("Take the yearly salary and divide it by the hours you actually work: £30,000 over 37.5 hours a week for 52 weeks is £15.38 an hour, and over 40 hours it is £14.42. Do it before tax, drop any unpaid lunch break, and divide by 12 for the month.");
    }
    return Py::None;
    Py::None
}
pub fn solve(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answer: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A computed maths reply, or None when the line is not a calculation.");
    clean = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]) } };
    if py_truthy(&pyrt::xrefs::xcall("memory.topics", "in_a_safety_thread", &[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__WAGE_CUE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__WAGE_CONTEXT(), "search", &[clean.clone()]) } }) {
    said = { let __b = _wage(&[clean.clone()]); if py_truthy(&__b) { __b } else { _wage_rate(&[clean.clone()]) } };
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![_quadratic(&[clean.clone()]), _simplify(&[clean.clone()]), _add_fractions(&[clean.clone()]), _the_factors(&[clean.clone()]), _common_factors(&[clean.clone()]), _round_to(&[clean.clone()]), _circle_measure(&[clean.clone()]), _box_volume(&[clean.clone()]), _equation(&[clean.clone()]), _root(&[clean.clone()]), _roots(&[clean.clone()]), _prime(&[clean.clone()]), _what_percent(&[clean.clone()]), _percent_on(&[clean.clone()]), _a_share_of_a_group(&[clean.clone()]), _fraction_decimal(&[clean.clone()]), _fraction(&[clean.clone()]), _fraction_of(&[clean.clone()]), _share(&[clean.clone()]), _percent(&[clean.clone()]), _percentage_change(&[clean.clone()]), _percent_rise(&[clean.clone()]), _percent_of_the_figure_in_the_chat(&[clean.clone(), memory.clone()]), _tip(&[clean.clone()]), _left_on_the_bill(&[clean.clone()]), _wage(&[clean.clone()]), _travel(&[clean.clone()]), _at_a_pace(&[clean.clone(), memory.clone()]), _save_by_a_date(&[clean.clone(), memory.clone()]), _save_time(&[clean.clone()]), _saving(&[clean.clone()]), _time_to_reach(&[clean.clone(), memory.clone()]), _a_goal_without_a_rate(&[clean.clone(), memory.clone()]), _time_at_a_new_rate(&[clean.clone(), memory.clone()]), _time_at_a_new_total(&[clean.clone(), memory.clone()]), _time_without_a_goal(&[clean.clone(), memory.clone()]), _figure_just_given(&[clean.clone(), memory.clone()]), _deadline_for_a_goal(&[clean.clone(), memory.clone()]), _save_by_a_date(&[clean.clone(), memory.clone()]), _each_month_for_a_goal(&[clean.clone(), memory.clone()]), _a_working_week(&[clean.clone()]), _convert(&[clean.clone(), memory.clone()]), _unit_fact(&[clean.clone()]), _average(&[clean.clone()]), _area(&[clean.clone()]), _perimeter_of_a_room(&[clean.clone()]), _days_in_month(&[clean.clone()]), _today(&[clean.clone()]), _days_until(&[clean.clone()]), _date_counted_from_today(&[clean.clone()]), _days_until_easter(&[clean.clone()]), _named_day_date(&[clean.clone()]), _the_weekday_of_a_date(&[clean.clone()]), _next_bank_holiday(&[clean.clone()]), _days_until_a_relative_day(&[clean.clone()]), _days_until_a_named_day(&[clean.clone()]), _days_until_a_month_or_season(&[clean.clone()]), _days_left_in_the_month_or_the_year(&[clean.clone()]), _working_days_or_weekends_until(&[clean.clone()]), _how_long_ago(&[clean.clone()])]));
    for __item in __iter1 {
    answer = __item.clone();
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    return _calc(&[clean.clone()]);
    Py::None
}
pub fn _figures(args: &[Py]) -> Py {
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
    let mut number: Py = Py::None;
    let _ = py_str("The numbers the line types out, in order: \"the median of 3, 5 and 10\" is 3, 5 and 10.");
    found = stdlib_call("re", "findall", &[py_str("(?<![\\w.])\\d+(?:\\.\\d+)?"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut number: Py = __item.clone(); __out.push(stdlib_call("fractions", "Fraction", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    Py::None
}
pub fn _short(args: &[Py]) -> Py {
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
    if py_truthy(&Py::Bool(py_eq(&gattr(&value.clone(), "denominator"), &Py::Int(1)))) {
    return call_builtin("str", &[gattr(&value.clone(), "numerator")]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("float", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, &".2f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    Py::None
}
pub fn _names_a_statistic(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line names the statistic it asks about, rather than pointing at a thing.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&asked.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("?.!,")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g__STATISTIC_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g__MEAN_VERB(), &word.clone()))) {
    before = if py_truthy(&index.clone()) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g__POINTS_AT_A_THING(), &before.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__OTHER_STATISTICS(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _about_words(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&asked.clone(), "split", &[]);
    i = op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    while py_truthy(&{ let __b = op_gt(&i.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__AVERAGE_FILLER(), &mcall(&geti(&words.clone(), &i.clone()), "strip", &[py_str("?.!,")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__MEAN_VERB(), &mcall(&geti(&words.clone(), &i.clone()), "strip", &[py_str("?.!,")]))) } } } }) {
    let __aug1 = op_sub(&i.clone(), &Py::Int(1));
    i = __aug1;
    }
    if py_truthy(&{ let __b = op_gt(&i.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__MEAN_VERB(), &mcall(&geti(&words.clone(), &i.clone()), "strip", &[py_str("?.!,")]))) } }) {
    words = op_add(&slice_or_raise(&words.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&words.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None));
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _averages(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut count: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut mean: Py = Py::None;
    let mut median: Py = Py::None;
    let mut middle: Py = Py::None;
    let mut mode: Py = Py::None;
    let mut ordered: Py = Py::None;
    let mut top: Py = Py::None;
    let mut total: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    clean = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    figures = _figures(&[clean.clone()]);
    asked = mcall(&clean.clone(), "lower", &[]);
    about = _about_words(&[asked.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), asked.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_list(__out) })() };
    if py_truthy(&{ let __b = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__AVERAGE_FILLER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _names_a_statistic(&[asked.clone()]) } }) {
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&asked.clone(), &py_str("median"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&about.clone(), &py_str("mean"))) } }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&asked.clone(), &py_str("difference"))) } }) {
    return g__AVERAGE_EXPLAIN();
    }
    if py_truthy(&Py::Bool(py_contains(&asked.clone(), &py_str("median")))) {
    return g__MEDIAN_EXPLAIN();
    }
    if py_truthy(&Py::Bool(py_contains(&asked.clone(), &py_str("mode")))) {
    return g__MODE_EXPLAIN();
    }
    return g__AVERAGE_EXPLAIN();
    }
    return Py::None;
    }
    ordered = call_builtin("sorted", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e));
    listed = mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ordered.clone()) { let mut figure: Py = __item.clone(); __out.push(_short(&[figure.clone()])); } py_list(__out) })() }]);
    mean = op_truediv(&call_builtin("sum", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)));
    middle = op_floordiv(&call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    if py_truthy(&op_modulo(&call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    median = geti(&ordered.clone(), &middle.clone());
    } else {
    median = op_truediv(&op_add(&geti(&ordered.clone(), &op_sub(&middle.clone(), &Py::Int(1))), &geti(&ordered.clone(), &middle.clone())), &Py::Int(2));
    }
    counts = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&ordered.clone());
    for __item in __iter1 {
    figure = __item.clone();
    let _ = py_setitem(&counts.clone(), &figure.clone(), op_add(&mcall(&counts.clone(), "get", &[figure.clone(), Py::Int(0)]), &Py::Int(1)));
    }
    top = call_builtin("max", &[mcall(&counts.clone(), "values", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&asked.clone(), &py_str("median")))) {
    return { let mut __s = String::new(); __s.push_str("The median of "); __s.push_str(&fstr(&listed.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_short(&[median.clone()]), -1, "")); __s.push_str(": in order, that is the middle one."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&asked.clone(), &py_str("mode")))) {
    if py_truthy(&Py::Bool(py_eq(&top.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str("No number in "); __s.push_str(&fstr(&listed.clone(), -1, "")); __s.push_str(" repeats, so there is no mode. The mean is "); __s.push_str(&fstr(&_short(&[mean.clone()]), -1, "")); __s.push_str(" and the median is "); __s.push_str(&fstr(&_short(&[median.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    mode = geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&counts.clone(), "items", &[])) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut figure: Py = __unpk1[0].clone(); let mut count: Py = __unpk1[1].clone(); if py_truthy(&Py::Bool(py_eq(&count.clone(), &top.clone()))) {  __out.push(figure.clone()); } } py_list(__out) })() }, &Py::Int(0));
    return { let mut __s = String::new(); __s.push_str("The mode of "); __s.push_str(&fstr(&listed.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_short(&[mode.clone()]), -1, "")); __s.push_str(", the number that appears most often."); py_string(__s) };
    }
    total = call_builtin("sum", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str("The mean of "); __s.push_str(&fstr(&listed.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_short(&[mean.clone()]), -1, "")); __s.push_str(": they add to "); __s.push_str(&fstr(&_short(&[total.clone()]), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&_short(&[total.clone()]), -1, "")); __s.push_str(" ÷ "); __s.push_str(&fstr(&call_builtin("len", &[ordered.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" = "); __s.push_str(&fstr(&_short(&[mean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _a_rounding_question(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line is about rounding a number, not about working round something.\n\n    A digit anywhere in the line is the plain sign of it - \"round 3.456 to two decimal places\",\n    \"what is 2.5 rounded to the nearest whole number\", \"round 47 to the nearest ten\". Without one\n    the line has to name the thing: \"how do you round numbers\", \"what are the rules for rounding\".\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&mcall(&g_ROUNDING_NAMED(), "search", &[line.clone()]), &Py::None));
    Py::None
}
pub fn _rounding_is_the_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The card needs the verb and needs a number or the name: \"round work\" is neither.");
    return { let __b = _round_is_the_verb(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _a_rounding_question(&[understood.clone()]) } };
    Py::None
}
pub fn _round_is_the_verb(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when \"round\" in the line is the maths verb rather than a way of saying \"around\".");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("text")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&g__ROUND_IS_AROUND(), "search", &[said.clone()])));
    Py::None
}
pub fn _not_a_spare_hour(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("False when the line asks what to do with the hour rather than what it pays.");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FREE_TIME_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__A_LENGTH_OF_TIME(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_PAY_WORD(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _prime_definition(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The definition, with the test of a number added only when the line asks how to test one.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b\\d+\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(how|work out|check|test|tell|know)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&g_PRIME_DEFINITION(), &g_PRIME_TEST());
    }
    return g_PRIME_DEFINITION();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.maths".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_ASK" => Some(g_ABOUT_ASK()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKED_FOR_FIGURE" => Some(g_ASKED_FOR_FIGURE()),
            "ASKED_FOR_RATE" => Some(g_ASKED_FOR_RATE()),
            "A_BARE_RATE_ASK" => Some(g_A_BARE_RATE_ASK()),
            "A_DATE_FOR_THE_THING" => Some(g_A_DATE_FOR_THE_THING()),
            "A_DEADLINE_DAY" => Some(g_A_DEADLINE_DAY()),
            "A_DEBT_CLEARED" => Some(g_A_DEBT_CLEARED()),
            "A_HORIZON" => Some(g_A_HORIZON()),
            "CLAUSE_SPLIT" => Some(g_CLAUSE_SPLIT()),
            "CLOCK" => Some(g_CLOCK()),
            "DEADLINE_DAYS" => Some(g_DEADLINE_DAYS()),
            "DEADLINE_NAMED" => Some(g_DEADLINE_NAMED()),
            "EACH_PERIOD" => Some(g_EACH_PERIOD()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FACTS" => Some(g_FACTS()),
            "GOAL" => Some(g_GOAL()),
            "HORIZON_COUNTS" => Some(g_HORIZON_COUNTS()),
            "HORIZON_SAID" => Some(g_HORIZON_SAID()),
            "HOW_HEAVY" => Some(g_HOW_HEAVY()),
            "HOW_MUCH_EACH" => Some(g_HOW_MUCH_EACH()),
            "JUST_A_TOTAL" => Some(g_JUST_A_TOTAL()),
            "K_THOUSANDS" => Some(g_K_THOUSANDS()),
            "MATHS_DEFINITION_ASK" => Some(g_MATHS_DEFINITION_ASK()),
            "MONEY" => Some(g_MONEY()),
            "MONEY_GOAL" => Some(g_MONEY_GOAL()),
            "MONTHS" => Some(g_MONTHS()),
            "MONTH_NUMBERS" => Some(g_MONTH_NUMBERS()),
            "OWED" => Some(g_OWED()),
            "PAST_REPORT" => Some(g_PAST_REPORT()),
            "PERIODS" => Some(g_PERIODS()),
            "PERIODS_IN_A_YEAR" => Some(g_PERIODS_IN_A_YEAR()),
            "PRIME_DEFINITION" => Some(g_PRIME_DEFINITION()),
            "PRIME_TEST" => Some(g_PRIME_TEST()),
            "RATE" => Some(g_RATE()),
            "ROUNDING_NAMED" => Some(g_ROUNDING_NAMED()),
            "ROUND_IS_NOT_THE_VERB" => Some(g_ROUND_IS_NOT_THE_VERB()),
            "SAID_A_RATE" => Some(g_SAID_A_RATE()),
            "SAVE_VERB" => Some(g_SAVE_VERB()),
            "SAVING_CLAUSE" => Some(g_SAVING_CLAUSE()),
            "SAVING_CUE" => Some(g_SAVING_CUE()),
            "SPENDING_CLAUSE" => Some(g_SPENDING_CLAUSE()),
            "TAKES" => Some(g_TAKES()),
            "TIME_ASKED" => Some(g_TIME_ASKED()),
            "TIME_ELSEWHERE" => Some(g_TIME_ELSEWHERE()),
            "TOWARDS" => Some(g_TOWARDS()),
            "WHAT_IF" => Some(g_WHAT_IF()),
            "WORKED" => Some(g_WORKED()),
            "_AFTER_HALF" => Some(g__AFTER_HALF()),
            "_AMOUNT" => Some(g__AMOUNT()),
            "_ANNUAL" => Some(g__ANNUAL()),
            "_AN_HOUR" => Some(g__AN_HOUR()),
            "_AREA_ASK" => Some(g__AREA_ASK()),
            "_AREA_TARGET" => Some(g__AREA_TARGET()),
            "_ASK_DATE" => Some(g__ASK_DATE()),
            "_AVERAGE_EXPLAIN" => Some(g__AVERAGE_EXPLAIN()),
            "_AVERAGE_FILLER" => Some(g__AVERAGE_FILLER()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_A_DISTANCE_STATED" => Some(g__A_DISTANCE_STATED()),
            "_A_LENGTH_OF_TIME" => Some(g__A_LENGTH_OF_TIME()),
            "_A_NOTE" => Some(g__A_NOTE()),
            "_A_PACE_WORD" => Some(g__A_PACE_WORD()),
            "_A_PAY_WORD" => Some(g__A_PAY_WORD()),
            "_A_YEAR_IN_THE_LINE" => Some(g__A_YEAR_IN_THE_LINE()),
            "_BANK_HOLIDAY_COUNT" => Some(g__BANK_HOLIDAY_COUNT()),
            "_BANK_HOLIDAY_LIST" => Some(g__BANK_HOLIDAY_LIST()),
            "_BANK_HOLIDAY_NEXT" => Some(g__BANK_HOLIDAY_NEXT()),
            "_BANK_HOLIDAY_TAIL" => Some(g__BANK_HOLIDAY_TAIL()),
            "_BANK_WHERE" => Some(g__BANK_WHERE()),
            "_BILL_WORD" => Some(g__BILL_WORD()),
            "_BIN_OPS" => Some(g__BIN_OPS()),
            "_BOUGHT_BY_THE_BAG" => Some(g__BOUGHT_BY_THE_BAG()),
            "_BUYING" => Some(g__BUYING()),
            "_CHRISTMAS_COUNTDOWN" => Some(g__CHRISTMAS_COUNTDOWN()),
            "_CHRISTMAS_DATE" => Some(g__CHRISTMAS_DATE()),
            "_CHRISTMAS_DAY" => Some(g__CHRISTMAS_DAY()),
            "_CHRISTMAS_FIXED" => Some(g__CHRISTMAS_FIXED()),
            "_CHRISTMAS_TAIL" => Some(g__CHRISTMAS_TAIL()),
            "_CHRISTMAS_TODAY" => Some(g__CHRISTMAS_TODAY()),
            "_CHRISTMAS_WEEKDAY" => Some(g__CHRISTMAS_WEEKDAY()),
            "_COUNT" => Some(g__COUNT()),
            "_COUNTED" => Some(g__COUNTED()),
            "_COUNTED_EVENT" => Some(g__COUNTED_EVENT()),
            "_COUNTED_GAP" => Some(g__COUNTED_GAP()),
            "_COUNTED_SUBJECT_TAIL" => Some(g__COUNTED_SUBJECT_TAIL()),
            "_COUNTS" => Some(g__COUNTS()),
            "_COUNT_ASK" => Some(g__COUNT_ASK()),
            "_COUNT_VALUE" => Some(g__COUNT_VALUE()),
            "_CUP_IN_ML" => Some(g__CUP_IN_ML()),
            "_CUP_IN_TBSP" => Some(g__CUP_IN_TBSP()),
            "_DATE_TAIL" => Some(g__DATE_TAIL()),
            "_DAY_MONTH" => Some(g__DAY_MONTH()),
            "_DAY_ONLY" => Some(g__DAY_ONLY()),
            "_DEFINITION_ASK" => Some(g__DEFINITION_ASK()),
            "_DISTANCE" => Some(g__DISTANCE()),
            "_EASTER_OFFSETS" => Some(g__EASTER_OFFSETS()),
            "_FACTOR_ASK" => Some(g__FACTOR_ASK()),
            "_FILLER" => Some(g__FILLER()),
            "_FRACTION" => Some(g__FRACTION()),
            "_FRACTIONS" => Some(g__FRACTIONS()),
            "_FRACTION_OF_A_FRACTION" => Some(g__FRACTION_OF_A_FRACTION()),
            "_FRACTION_OF_FRACTION" => Some(g__FRACTION_OF_FRACTION()),
            "_FRACTION_UNITS" => Some(g__FRACTION_UNITS()),
            "_FREE_TIME_ASK" => Some(g__FREE_TIME_ASK()),
            "_HEAVY" => Some(g__HEAVY()),
            "_HOURS_WEEK" => Some(g__HOURS_WEEK()),
            "_HOURS_WORKED" => Some(g__HOURS_WORKED()),
            "_HOW_FAR" => Some(g__HOW_FAR()),
            "_HOW_MUCH" => Some(g__HOW_MUCH()),
            "_IS_THAT_TOO_MUCH" => Some(g__IS_THAT_TOO_MUCH()),
            "_K_THOUSANDS_HERE" => Some(g__K_THOUSANDS_HERE()),
            "_LENGTH" => Some(g__LENGTH()),
            "_LENGTH_UNIT" => Some(g__LENGTH_UNIT()),
            "_MATHS_WORDS" => Some(g__MATHS_WORDS()),
            "_MEAN_VERB" => Some(g__MEAN_VERB()),
            "_MEDIAN_EXPLAIN" => Some(g__MEDIAN_EXPLAIN()),
            "_METRES_SQUARED" => Some(g__METRES_SQUARED()),
            "_MODE_EXPLAIN" => Some(g__MODE_EXPLAIN()),
            "_MONEY_BESIDE" => Some(g__MONEY_BESIDE()),
            "_MONEY_HERE" => Some(g__MONEY_HERE()),
            "_MONEY_WORD" => Some(g__MONEY_WORD()),
            "_MONTHS_OR" => Some(g__MONTHS_OR()),
            "_MONTHS_SAID" => Some(g__MONTHS_SAID()),
            "_MONTH_NAMES" => Some(g__MONTH_NAMES()),
            "_MONTH_SHORT" => Some(g__MONTH_SHORT()),
            "_MONTH_TAIL" => Some(g__MONTH_TAIL()),
            "_MONTH_TITLE" => Some(g__MONTH_TITLE()),
            "_MONTH_WORD" => Some(g__MONTH_WORD()),
            "_NAMED_DAYS" => Some(g__NAMED_DAYS()),
            "_NEAREST" => Some(g__NEAREST()),
            "_NOT_PAY" => Some(g__NOT_PAY()),
            "_NUMBER" => Some(g__NUMBER()),
            "_NUMBER_THOUSANDS" => Some(g__NUMBER_THOUSANDS()),
            "_OTHER_STATISTICS" => Some(g__OTHER_STATISTICS()),
            "_PACES" => Some(g__PACES()),
            "_PART" => Some(g__PART()),
            "_PAY_WORDS" => Some(g__PAY_WORDS()),
            "_PERCENT_ADVICE" => Some(g__PERCENT_ADVICE()),
            "_PERCENT_ASK_TAIL" => Some(g__PERCENT_ASK_TAIL()),
            "_PERCENT_ASK_WORD" => Some(g__PERCENT_ASK_WORD()),
            "_PERCENT_DOWN" => Some(g__PERCENT_DOWN()),
            "_PERCENT_UP" => Some(g__PERCENT_UP()),
            "_PERIMETER_ASK" => Some(g__PERIMETER_ASK()),
            "_PLACES" => Some(g__PLACES()),
            "_POINTS_AT_A_THING" => Some(g__POINTS_AT_A_THING()),
            "_POLITE" => Some(g__POLITE()),
            "_QUADRATIC_ASK" => Some(g__QUADRATIC_ASK()),
            "_RATE" => Some(g__RATE()),
            "_REST_OF_A_PERIOD" => Some(g__REST_OF_A_PERIOD()),
            "_REST_OF_A_PERIOD_WORDS" => Some(g__REST_OF_A_PERIOD_WORDS()),
            "_ROUND_IS_AROUND" => Some(g__ROUND_IS_AROUND()),
            "_SAYS_WEEKDAY" => Some(g__SAYS_WEEKDAY()),
            "_SEASONS" => Some(g__SEASONS()),
            "_SHAPE" => Some(g__SHAPE()),
            "_SIDE" => Some(g__SIDE()),
            "_SIDE_BY_SIDE" => Some(g__SIDE_BY_SIDE()),
            "_SINCE_ASK" => Some(g__SINCE_ASK()),
            "_SINCE_FILLER" => Some(g__SINCE_FILLER()),
            "_SINGULAR" => Some(g__SINGULAR()),
            "_SMALLER_PERIOD" => Some(g__SMALLER_PERIOD()),
            "_SMALL_NUMBER" => Some(g__SMALL_NUMBER()),
            "_SQUARE_WORD" => Some(g__SQUARE_WORD()),
            "_STATISTIC_WORDS" => Some(g__STATISTIC_WORDS()),
            "_SURFACES" => Some(g__SURFACES()),
            "_TAILS" => Some(g__TAILS()),
            "_TENS_WORDS" => Some(g__TENS_WORDS()),
            "_THE_TURN_OF_THE_YEAR" => Some(g__THE_TURN_OF_THE_YEAR()),
            "_TODAY_ASK" => Some(g__TODAY_ASK()),
            "_TO_WORK" => Some(g__TO_WORK()),
            "_UNARY_OPS" => Some(g__UNARY_OPS()),
            "_UNIT" => Some(g__UNIT()),
            "_UNITS" => Some(g__UNITS()),
            "_WAGE" => Some(g__WAGE()),
            "_WAGE_CONTEXT" => Some(g__WAGE_CONTEXT()),
            "_WAGE_CUE" => Some(g__WAGE_CUE()),
            "_WAGE_UNIT" => Some(g__WAGE_UNIT()),
            "_WEEKDAYS" => Some(g__WEEKDAYS()),
            "_WEEKDAY_WORDS" => Some(g__WEEKDAY_WORDS()),
            "_WORD_THOUSANDS" => Some(g__WORD_THOUSANDS()),
            "_WORKING_ASK" => Some(g__WORKING_ASK()),
            "_WORKING_LEFT" => Some(g__WORKING_LEFT()),
            "_WORKING_SHAPE" => Some(g__WORKING_SHAPE()),
            "_YOU_LEFT" => Some(g__YOU_LEFT()),
            "_YOU_LEFT_ANYWHERE" => Some(g__YOU_LEFT_ANYWHERE()),
            "_a_deadline_in_chat" => Some(py_func("_a_deadline_in_chat", |args: &[Py]| _a_deadline_in_chat(args))),
            "_a_goal_without_a_rate" => Some(py_func("_a_goal_without_a_rate", |args: &[Py]| _a_goal_without_a_rate(args))),
            "_a_known_day_that_has_gone" => Some(py_func("_a_known_day_that_has_gone", |args: &[Py]| _a_known_day_that_has_gone(args))),
            "_a_known_day_the_line_names_next" => Some(py_func("_a_known_day_the_line_names_next", |args: &[Py]| _a_known_day_the_line_names_next(args))),
            "_a_month_for_the_deadline_in_chat" => Some(py_func("_a_month_for_the_deadline_in_chat", |args: &[Py]| _a_month_for_the_deadline_in_chat(args))),
            "_a_named_deadline" => Some(py_func("_a_named_deadline", |args: &[Py]| _a_named_deadline(args))),
            "_a_new_total_only" => Some(py_func("_a_new_total_only", |args: &[Py]| _a_new_total_only(args))),
            "_a_rate_was_said" => Some(py_func("_a_rate_was_said", |args: &[Py]| _a_rate_was_said(args))),
            "_a_rounding_question" => Some(py_func("_a_rounding_question", |args: &[Py]| _a_rounding_question(args))),
            "_a_saving_goal_in_chat" => Some(py_func("_a_saving_goal_in_chat", |args: &[Py]| _a_saving_goal_in_chat(args))),
            "_a_share_of_a_group" => Some(py_func("_a_share_of_a_group", |args: &[Py]| _a_share_of_a_group(args))),
            "_a_total_by_a_date" => Some(py_func("_a_total_by_a_date", |args: &[Py]| _a_total_by_a_date(args))),
            "_a_working_week" => Some(py_func("_a_working_week", |args: &[Py]| _a_working_week(args))),
            "_a_yearline" => Some(py_func("_a_yearline", |args: &[Py]| _a_yearline(args))),
            "_about_words" => Some(py_func("_about_words", |args: &[Py]| _about_words(args))),
            "_add_fractions" => Some(py_func("_add_fractions", |args: &[Py]| _add_fractions(args))),
            "_amount" => Some(py_func("_amount", |args: &[Py]| _amount(args))),
            "_amount_in_the_chat" => Some(py_func("_amount_in_the_chat", |args: &[Py]| _amount_in_the_chat(args))),
            "_annual_pay" => Some(py_func("_annual_pay", |args: &[Py]| _annual_pay(args))),
            "_area" => Some(py_func("_area", |args: &[Py]| _area(args))),
            "_as_fraction" => Some(py_func("_as_fraction", |args: &[Py]| _as_fraction(args))),
            "_asked_period" => Some(py_func("_asked_period", |args: &[Py]| _asked_period(args))),
            "_asks_a_new_sum" => Some(py_func("_asks_a_new_sum", |args: &[Py]| _asks_a_new_sum(args))),
            "_at_a_pace" => Some(py_func("_at_a_pace", |args: &[Py]| _at_a_pace(args))),
            "_average" => Some(py_func("_average", |args: &[Py]| _average(args))),
            "_averages" => Some(py_func("_averages", |args: &[Py]| _averages(args))),
            "_bank_holidays" => Some(py_func("_bank_holidays", |args: &[Py]| _bank_holidays(args))),
            "_bank_where" => Some(py_func("_bank_where", |args: &[Py]| _bank_where(args))),
            "_box_volume" => Some(py_func("_box_volume", |args: &[Py]| _box_volume(args))),
            "_calc" => Some(py_func("_calc", |args: &[Py]| _calc(args))),
            "_christmas_date_of" => Some(py_func("_christmas_date_of", |args: &[Py]| _christmas_date_of(args))),
            "_christmas_day" => Some(py_func("_christmas_day", |args: &[Py]| _christmas_day(args))),
            "_christmas_name" => Some(py_func("_christmas_name", |args: &[Py]| _christmas_name(args))),
            "_circle_measure" => Some(py_func("_circle_measure", |args: &[Py]| _circle_measure(args))),
            "_common_factors" => Some(py_func("_common_factors", |args: &[Py]| _common_factors(args))),
            "_convert" => Some(py_func("_convert", |args: &[Py]| _convert(args))),
            "_count_in" => Some(py_func("_count_in", |args: &[Py]| _count_in(args))),
            "_count_value" => Some(py_func("_count_value", |args: &[Py]| _count_value(args))),
            "_countdown_text" => Some(py_func("_countdown_text", |args: &[Py]| _countdown_text(args))),
            "_date_counted_from_today" => Some(py_func("_date_counted_from_today", |args: &[Py]| _date_counted_from_today(args))),
            "_date_the_line_gave" => Some(py_func("_date_the_line_gave", |args: &[Py]| _date_the_line_gave(args))),
            "_day_in_a_month" => Some(py_func("_day_in_a_month", |args: &[Py]| _day_in_a_month(args))),
            "_day_text" => Some(py_func("_day_text", |args: &[Py]| _day_text(args))),
            "_days_in_month" => Some(py_func("_days_in_month", |args: &[Py]| _days_in_month(args))),
            "_days_left_in_the_month_or_the_year" => Some(py_func("_days_left_in_the_month_or_the_year", |args: &[Py]| _days_left_in_the_month_or_the_year(args))),
            "_days_until" => Some(py_func("_days_until", |args: &[Py]| _days_until(args))),
            "_days_until_a_month_or_season" => Some(py_func("_days_until_a_month_or_season", |args: &[Py]| _days_until_a_month_or_season(args))),
            "_days_until_a_named_day" => Some(py_func("_days_until_a_named_day", |args: &[Py]| _days_until_a_named_day(args))),
            "_days_until_a_relative_day" => Some(py_func("_days_until_a_relative_day", |args: &[Py]| _days_until_a_relative_day(args))),
            "_days_until_easter" => Some(py_func("_days_until_easter", |args: &[Py]| _days_until_easter(args))),
            "_deadline_for_a_goal" => Some(py_func("_deadline_for_a_goal", |args: &[Py]| _deadline_for_a_goal(args))),
            "_dec" => Some(py_func("_dec", |args: &[Py]| _dec(args))),
            "_distance_in_miles" => Some(py_func("_distance_in_miles", |args: &[Py]| _distance_in_miles(args))),
            "_duration" => Some(py_func("_duration", |args: &[Py]| _duration(args))),
            "_duration_reply" => Some(py_func("_duration_reply", |args: &[Py]| _duration_reply(args))),
            "_each_hundred" => Some(py_func("_each_hundred", |args: &[Py]| _each_hundred(args))),
            "_each_month_for_a_goal" => Some(py_func("_each_month_for_a_goal", |args: &[Py]| _each_month_for_a_goal(args))),
            "_easter_day" => Some(py_func("_easter_day", |args: &[Py]| _easter_day(args))),
            "_easter_sunday" => Some(py_func("_easter_sunday", |args: &[Py]| _easter_sunday(args))),
            "_equation" => Some(py_func("_equation", |args: &[Py]| _equation(args))),
            "_eval" => Some(py_func("_eval", |args: &[Py]| _eval(args))),
            "_exact_decimal" => Some(py_func("_exact_decimal", |args: &[Py]| _exact_decimal(args))),
            "_expression" => Some(py_func("_expression", |args: &[Py]| _expression(args))),
            "_factorised" => Some(py_func("_factorised", |args: &[Py]| _factorised(args))),
            "_factors_in_list" => Some(py_func("_factors_in_list", |args: &[Py]| _factors_in_list(args))),
            "_feast_year" => Some(py_func("_feast_year", |args: &[Py]| _feast_year(args))),
            "_figure_just_given" => Some(py_func("_figure_just_given", |args: &[Py]| _figure_just_given(args))),
            "_figures" => Some(py_func("_figures", |args: &[Py]| _figures(args))),
            "_first_of_the_month" => Some(py_func("_first_of_the_month", |args: &[Py]| _first_of_the_month(args))),
            "_floor_area" => Some(py_func("_floor_area", |args: &[Py]| _floor_area(args))),
            "_fraction" => Some(py_func("_fraction", |args: &[Py]| _fraction(args))),
            "_fraction_decimal" => Some(py_func("_fraction_decimal", |args: &[Py]| _fraction_decimal(args))),
            "_fraction_of" => Some(py_func("_fraction_of", |args: &[Py]| _fraction_of(args))),
            "_fraction_of_a_fraction" => Some(py_func("_fraction_of_a_fraction", |args: &[Py]| _fraction_of_a_fraction(args))),
            "_fraction_text" => Some(py_func("_fraction_text", |args: &[Py]| _fraction_text(args))),
            "_gcd" => Some(py_func("_gcd", |args: &[Py]| _gcd(args))),
            "_goal_in_chat" => Some(py_func("_goal_in_chat", |args: &[Py]| _goal_in_chat(args))),
            "_hourly_named" => Some(py_func("_hourly_named", |args: &[Py]| _hourly_named(args))),
            "_hours_a_week" => Some(py_func("_hours_a_week", |args: &[Py]| _hours_a_week(args))),
            "_how_long_ago" => Some(py_func("_how_long_ago", |args: &[Py]| _how_long_ago(args))),
            "_in_months" => Some(py_func("_in_months", |args: &[Py]| _in_months(args))),
            "_k_as_money" => Some(py_func("_k_as_money", |args: &[Py]| _k_as_money(args))),
            "_last_takes" => Some(py_func("_last_takes", |args: &[Py]| _last_takes(args))),
            "_last_weekday" => Some(py_func("_last_weekday", |args: &[Py]| _last_weekday(args))),
            "_leap" => Some(py_func("_leap", |args: &[Py]| _leap(args))),
            "_left_on_the_bill" => Some(py_func("_left_on_the_bill", |args: &[Py]| _left_on_the_bill(args))),
            "_money" => Some(py_func("_money", |args: &[Py]| _money(args))),
            "_month_number" => Some(py_func("_month_number", |args: &[Py]| _month_number(args))),
            "_month_on" => Some(py_func("_month_on", |args: &[Py]| _month_on(args))),
            "_month_year" => Some(py_func("_month_year", |args: &[Py]| _month_year(args))),
            "_months_between" => Some(py_func("_months_between", |args: &[Py]| _months_between(args))),
            "_named_day_date" => Some(py_func("_named_day_date", |args: &[Py]| _named_day_date(args))),
            "_names_a_statistic" => Some(py_func("_names_a_statistic", |args: &[Py]| _names_a_statistic(args))),
            "_next_bank_holiday" => Some(py_func("_next_bank_holiday", |args: &[Py]| _next_bank_holiday(args))),
            "_not_a_spare_hour" => Some(py_func("_not_a_spare_hour", |args: &[Py]| _not_a_spare_hour(args))),
            "_nth_weekday" => Some(py_func("_nth_weekday", |args: &[Py]| _nth_weekday(args))),
            "_number" => Some(py_func("_number", |args: &[Py]| _number(args))),
            "_ordinal" => Some(py_func("_ordinal", |args: &[Py]| _ordinal(args))),
            "_parse" => Some(py_func("_parse", |args: &[Py]| _parse(args))),
            "_percent" => Some(py_func("_percent", |args: &[Py]| _percent(args))),
            "_percent_of_the_figure_in_the_chat" => Some(py_func("_percent_of_the_figure_in_the_chat", |args: &[Py]| _percent_of_the_figure_in_the_chat(args))),
            "_percent_on" => Some(py_func("_percent_on", |args: &[Py]| _percent_on(args))),
            "_percent_rise" => Some(py_func("_percent_rise", |args: &[Py]| _percent_rise(args))),
            "_percentage_change" => Some(py_func("_percentage_change", |args: &[Py]| _percentage_change(args))),
            "_perimeter_of_a_room" => Some(py_func("_perimeter_of_a_room", |args: &[Py]| _perimeter_of_a_room(args))),
            "_places" => Some(py_func("_places", |args: &[Py]| _places(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_pounds" => Some(py_func("_pounds", |args: &[Py]| _pounds(args))),
            "_prime" => Some(py_func("_prime", |args: &[Py]| _prime(args))),
            "_prime_definition" => Some(py_func("_prime_definition", |args: &[Py]| _prime_definition(args))),
            "_quadratic" => Some(py_func("_quadratic", |args: &[Py]| _quadratic(args))),
            "_rate" => Some(py_func("_rate", |args: &[Py]| _rate(args))),
            "_rate_and_total" => Some(py_func("_rate_and_total", |args: &[Py]| _rate_and_total(args))),
            "_rate_clause" => Some(py_func("_rate_clause", |args: &[Py]| _rate_clause(args))),
            "_rate_in_chat" => Some(py_func("_rate_in_chat", |args: &[Py]| _rate_in_chat(args))),
            "_reduced" => Some(py_func("_reduced", |args: &[Py]| _reduced(args))),
            "_relative_count_text" => Some(py_func("_relative_count_text", |args: &[Py]| _relative_count_text(args))),
            "_relative_day" => Some(py_func("_relative_day", |args: &[Py]| _relative_day(args))),
            "_root" => Some(py_func("_root", |args: &[Py]| _root(args))),
            "_roots" => Some(py_func("_roots", |args: &[Py]| _roots(args))),
            "_round_is_the_verb" => Some(py_func("_round_is_the_verb", |args: &[Py]| _round_is_the_verb(args))),
            "_round_to" => Some(py_func("_round_to", |args: &[Py]| _round_to(args))),
            "_rounding_is_the_ask" => Some(py_func("_rounding_is_the_ask", |args: &[Py]| _rounding_is_the_ask(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_said_list" => Some(py_func("_said_list", |args: &[Py]| _said_list(args))),
            "_said_with_a_weekday" => Some(py_func("_said_with_a_weekday", |args: &[Py]| _said_with_a_weekday(args))),
            "_save_by_a_date" => Some(py_func("_save_by_a_date", |args: &[Py]| _save_by_a_date(args))),
            "_save_time" => Some(py_func("_save_time", |args: &[Py]| _save_time(args))),
            "_saving" => Some(py_func("_saving", |args: &[Py]| _saving(args))),
            "_share" => Some(py_func("_share", |args: &[Py]| _share(args))),
            "_share_reply" => Some(py_func("_share_reply", |args: &[Py]| _share_reply(args))),
            "_shopping_days" => Some(py_func("_shopping_days", |args: &[Py]| _shopping_days(args))),
            "_short" => Some(py_func("_short", |args: &[Py]| _short(args))),
            "_short_date" => Some(py_func("_short_date", |args: &[Py]| _short_date(args))),
            "_sides" => Some(py_func("_sides", |args: &[Py]| _sides(args))),
            "_simplify" => Some(py_func("_simplify", |args: &[Py]| _simplify(args))),
            "_square_text" => Some(py_func("_square_text", |args: &[Py]| _square_text(args))),
            "_sums" => Some(py_func("_sums", |args: &[Py]| _sums(args))),
            "_surface" => Some(py_func("_surface", |args: &[Py]| _surface(args))),
            "_the_factors" => Some(py_func("_the_factors", |args: &[Py]| _the_factors(args))),
            "_the_last_day_of" => Some(py_func("_the_last_day_of", |args: &[Py]| _the_last_day_of(args))),
            "_the_plan_figure" => Some(py_func("_the_plan_figure", |args: &[Py]| _the_plan_figure(args))),
            "_the_size_of_the_trip" => Some(py_func("_the_size_of_the_trip", |args: &[Py]| _the_size_of_the_trip(args))),
            "_the_way_it_is_done" => Some(py_func("_the_way_it_is_done", |args: &[Py]| _the_way_it_is_done(args))),
            "_the_weekday_of_a_date" => Some(py_func("_the_weekday_of_a_date", |args: &[Py]| _the_weekday_of_a_date(args))),
            "_the_wider_day" => Some(py_func("_the_wider_day", |args: &[Py]| _the_wider_day(args))),
            "_time_at_a_new_rate" => Some(py_func("_time_at_a_new_rate", |args: &[Py]| _time_at_a_new_rate(args))),
            "_time_at_a_new_total" => Some(py_func("_time_at_a_new_total", |args: &[Py]| _time_at_a_new_total(args))),
            "_time_to_reach" => Some(py_func("_time_to_reach", |args: &[Py]| _time_to_reach(args))),
            "_time_without_a_goal" => Some(py_func("_time_without_a_goal", |args: &[Py]| _time_without_a_goal(args))),
            "_tip" => Some(py_func("_tip", |args: &[Py]| _tip(args))),
            "_today" => Some(py_func("_today", |args: &[Py]| _today(args))),
            "_travel" => Some(py_func("_travel", |args: &[Py]| _travel(args))),
            "_two_places" => Some(py_func("_two_places", |args: &[Py]| _two_places(args))),
            "_unit_fact" => Some(py_func("_unit_fact", |args: &[Py]| _unit_fact(args))),
            "_unit_pair" => Some(py_func("_unit_pair", |args: &[Py]| _unit_pair(args))),
            "_user_lines" => Some(py_func("_user_lines", |args: &[Py]| _user_lines(args))),
            "_wage" => Some(py_func("_wage", |args: &[Py]| _wage(args))),
            "_wage_rate" => Some(py_func("_wage_rate", |args: &[Py]| _wage_rate(args))),
            "_wait_text" => Some(py_func("_wait_text", |args: &[Py]| _wait_text(args))),
            "_weekday_of" => Some(py_func("_weekday_of", |args: &[Py]| _weekday_of(args))),
            "_weekday_text" => Some(py_func("_weekday_text", |args: &[Py]| _weekday_text(args))),
            "_what_percent" => Some(py_func("_what_percent", |args: &[Py]| _what_percent(args))),
            "_worked_in_chat" => Some(py_func("_worked_in_chat", |args: &[Py]| _worked_in_chat(args))),
            "_working_days_or_weekends_until" => Some(py_func("_working_days_or_weekends_until", |args: &[Py]| _working_days_or_weekends_until(args))),
            "_year_the_line_gave" => Some(py_func("_year_the_line_gave", |args: &[Py]| _year_the_line_gave(args))),
            "_yearly_named" => Some(py_func("_yearly_named", |args: &[Py]| _yearly_named(args))),
            "a_rate_put_right" => Some(g_a_rate_put_right()),
            "entry" => Some(g_entry()),
            "in_a_safety_thread" => Some(g_in_a_safety_thread()),
            "solve" => Some(py_func("solve", |args: &[Py]| solve(args))),
            "the_money_already_held" => Some(g_the_money_already_held()),
            "the_pot_said_alone" => Some(g_the_pot_said_alone()),
            "units" => Some(pyrt::xrefs::xmodule("knowledge.units")),
            "ast" => Some(stdlib_module("ast")),
            "operator" => Some(stdlib_module("operator")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_ASK" => s_ABOUT_ASK(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASKED_FOR_FIGURE" => s_ASKED_FOR_FIGURE(v),
            "ASKED_FOR_RATE" => s_ASKED_FOR_RATE(v),
            "A_BARE_RATE_ASK" => s_A_BARE_RATE_ASK(v),
            "A_DATE_FOR_THE_THING" => s_A_DATE_FOR_THE_THING(v),
            "A_DEADLINE_DAY" => s_A_DEADLINE_DAY(v),
            "A_DEBT_CLEARED" => s_A_DEBT_CLEARED(v),
            "A_HORIZON" => s_A_HORIZON(v),
            "CLAUSE_SPLIT" => s_CLAUSE_SPLIT(v),
            "CLOCK" => s_CLOCK(v),
            "DEADLINE_DAYS" => s_DEADLINE_DAYS(v),
            "DEADLINE_NAMED" => s_DEADLINE_NAMED(v),
            "EACH_PERIOD" => s_EACH_PERIOD(v),
            "ENTRIES" => s_ENTRIES(v),
            "FACTS" => s_FACTS(v),
            "GOAL" => s_GOAL(v),
            "HORIZON_COUNTS" => s_HORIZON_COUNTS(v),
            "HORIZON_SAID" => s_HORIZON_SAID(v),
            "HOW_HEAVY" => s_HOW_HEAVY(v),
            "HOW_MUCH_EACH" => s_HOW_MUCH_EACH(v),
            "JUST_A_TOTAL" => s_JUST_A_TOTAL(v),
            "K_THOUSANDS" => s_K_THOUSANDS(v),
            "MATHS_DEFINITION_ASK" => s_MATHS_DEFINITION_ASK(v),
            "MONEY" => s_MONEY(v),
            "MONEY_GOAL" => s_MONEY_GOAL(v),
            "MONTHS" => s_MONTHS(v),
            "MONTH_NUMBERS" => s_MONTH_NUMBERS(v),
            "OWED" => s_OWED(v),
            "PAST_REPORT" => s_PAST_REPORT(v),
            "PERIODS" => s_PERIODS(v),
            "PERIODS_IN_A_YEAR" => s_PERIODS_IN_A_YEAR(v),
            "PRIME_DEFINITION" => s_PRIME_DEFINITION(v),
            "PRIME_TEST" => s_PRIME_TEST(v),
            "RATE" => s_RATE(v),
            "ROUNDING_NAMED" => s_ROUNDING_NAMED(v),
            "ROUND_IS_NOT_THE_VERB" => s_ROUND_IS_NOT_THE_VERB(v),
            "SAID_A_RATE" => s_SAID_A_RATE(v),
            "SAVE_VERB" => s_SAVE_VERB(v),
            "SAVING_CLAUSE" => s_SAVING_CLAUSE(v),
            "SAVING_CUE" => s_SAVING_CUE(v),
            "SPENDING_CLAUSE" => s_SPENDING_CLAUSE(v),
            "TAKES" => s_TAKES(v),
            "TIME_ASKED" => s_TIME_ASKED(v),
            "TIME_ELSEWHERE" => s_TIME_ELSEWHERE(v),
            "TOWARDS" => s_TOWARDS(v),
            "WHAT_IF" => s_WHAT_IF(v),
            "WORKED" => s_WORKED(v),
            "_AFTER_HALF" => s__AFTER_HALF(v),
            "_AMOUNT" => s__AMOUNT(v),
            "_ANNUAL" => s__ANNUAL(v),
            "_AN_HOUR" => s__AN_HOUR(v),
            "_AREA_ASK" => s__AREA_ASK(v),
            "_AREA_TARGET" => s__AREA_TARGET(v),
            "_ASK_DATE" => s__ASK_DATE(v),
            "_AVERAGE_EXPLAIN" => s__AVERAGE_EXPLAIN(v),
            "_AVERAGE_FILLER" => s__AVERAGE_FILLER(v),
            "_A_COUNT" => s__A_COUNT(v),
            "_A_DISTANCE_STATED" => s__A_DISTANCE_STATED(v),
            "_A_LENGTH_OF_TIME" => s__A_LENGTH_OF_TIME(v),
            "_A_NOTE" => s__A_NOTE(v),
            "_A_PACE_WORD" => s__A_PACE_WORD(v),
            "_A_PAY_WORD" => s__A_PAY_WORD(v),
            "_A_YEAR_IN_THE_LINE" => s__A_YEAR_IN_THE_LINE(v),
            "_BANK_HOLIDAY_COUNT" => s__BANK_HOLIDAY_COUNT(v),
            "_BANK_HOLIDAY_LIST" => s__BANK_HOLIDAY_LIST(v),
            "_BANK_HOLIDAY_NEXT" => s__BANK_HOLIDAY_NEXT(v),
            "_BANK_HOLIDAY_TAIL" => s__BANK_HOLIDAY_TAIL(v),
            "_BANK_WHERE" => s__BANK_WHERE(v),
            "_BILL_WORD" => s__BILL_WORD(v),
            "_BIN_OPS" => s__BIN_OPS(v),
            "_BOUGHT_BY_THE_BAG" => s__BOUGHT_BY_THE_BAG(v),
            "_BUYING" => s__BUYING(v),
            "_CHRISTMAS_COUNTDOWN" => s__CHRISTMAS_COUNTDOWN(v),
            "_CHRISTMAS_DATE" => s__CHRISTMAS_DATE(v),
            "_CHRISTMAS_DAY" => s__CHRISTMAS_DAY(v),
            "_CHRISTMAS_FIXED" => s__CHRISTMAS_FIXED(v),
            "_CHRISTMAS_TAIL" => s__CHRISTMAS_TAIL(v),
            "_CHRISTMAS_TODAY" => s__CHRISTMAS_TODAY(v),
            "_CHRISTMAS_WEEKDAY" => s__CHRISTMAS_WEEKDAY(v),
            "_COUNT" => s__COUNT(v),
            "_COUNTED" => s__COUNTED(v),
            "_COUNTED_EVENT" => s__COUNTED_EVENT(v),
            "_COUNTED_GAP" => s__COUNTED_GAP(v),
            "_COUNTED_SUBJECT_TAIL" => s__COUNTED_SUBJECT_TAIL(v),
            "_COUNTS" => s__COUNTS(v),
            "_COUNT_ASK" => s__COUNT_ASK(v),
            "_COUNT_VALUE" => s__COUNT_VALUE(v),
            "_CUP_IN_ML" => s__CUP_IN_ML(v),
            "_CUP_IN_TBSP" => s__CUP_IN_TBSP(v),
            "_DATE_TAIL" => s__DATE_TAIL(v),
            "_DAY_MONTH" => s__DAY_MONTH(v),
            "_DAY_ONLY" => s__DAY_ONLY(v),
            "_DEFINITION_ASK" => s__DEFINITION_ASK(v),
            "_DISTANCE" => s__DISTANCE(v),
            "_EASTER_OFFSETS" => s__EASTER_OFFSETS(v),
            "_FACTOR_ASK" => s__FACTOR_ASK(v),
            "_FILLER" => s__FILLER(v),
            "_FRACTION" => s__FRACTION(v),
            "_FRACTIONS" => s__FRACTIONS(v),
            "_FRACTION_OF_A_FRACTION" => s__FRACTION_OF_A_FRACTION(v),
            "_FRACTION_OF_FRACTION" => s__FRACTION_OF_FRACTION(v),
            "_FRACTION_UNITS" => s__FRACTION_UNITS(v),
            "_FREE_TIME_ASK" => s__FREE_TIME_ASK(v),
            "_HEAVY" => s__HEAVY(v),
            "_HOURS_WEEK" => s__HOURS_WEEK(v),
            "_HOURS_WORKED" => s__HOURS_WORKED(v),
            "_HOW_FAR" => s__HOW_FAR(v),
            "_HOW_MUCH" => s__HOW_MUCH(v),
            "_IS_THAT_TOO_MUCH" => s__IS_THAT_TOO_MUCH(v),
            "_K_THOUSANDS_HERE" => s__K_THOUSANDS_HERE(v),
            "_LENGTH" => s__LENGTH(v),
            "_LENGTH_UNIT" => s__LENGTH_UNIT(v),
            "_MATHS_WORDS" => s__MATHS_WORDS(v),
            "_MEAN_VERB" => s__MEAN_VERB(v),
            "_MEDIAN_EXPLAIN" => s__MEDIAN_EXPLAIN(v),
            "_METRES_SQUARED" => s__METRES_SQUARED(v),
            "_MODE_EXPLAIN" => s__MODE_EXPLAIN(v),
            "_MONEY_BESIDE" => s__MONEY_BESIDE(v),
            "_MONEY_HERE" => s__MONEY_HERE(v),
            "_MONEY_WORD" => s__MONEY_WORD(v),
            "_MONTHS_OR" => s__MONTHS_OR(v),
            "_MONTHS_SAID" => s__MONTHS_SAID(v),
            "_MONTH_NAMES" => s__MONTH_NAMES(v),
            "_MONTH_SHORT" => s__MONTH_SHORT(v),
            "_MONTH_TAIL" => s__MONTH_TAIL(v),
            "_MONTH_TITLE" => s__MONTH_TITLE(v),
            "_MONTH_WORD" => s__MONTH_WORD(v),
            "_NAMED_DAYS" => s__NAMED_DAYS(v),
            "_NEAREST" => s__NEAREST(v),
            "_NOT_PAY" => s__NOT_PAY(v),
            "_NUMBER" => s__NUMBER(v),
            "_NUMBER_THOUSANDS" => s__NUMBER_THOUSANDS(v),
            "_OTHER_STATISTICS" => s__OTHER_STATISTICS(v),
            "_PACES" => s__PACES(v),
            "_PART" => s__PART(v),
            "_PAY_WORDS" => s__PAY_WORDS(v),
            "_PERCENT_ADVICE" => s__PERCENT_ADVICE(v),
            "_PERCENT_ASK_TAIL" => s__PERCENT_ASK_TAIL(v),
            "_PERCENT_ASK_WORD" => s__PERCENT_ASK_WORD(v),
            "_PERCENT_DOWN" => s__PERCENT_DOWN(v),
            "_PERCENT_UP" => s__PERCENT_UP(v),
            "_PERIMETER_ASK" => s__PERIMETER_ASK(v),
            "_PLACES" => s__PLACES(v),
            "_POINTS_AT_A_THING" => s__POINTS_AT_A_THING(v),
            "_POLITE" => s__POLITE(v),
            "_QUADRATIC_ASK" => s__QUADRATIC_ASK(v),
            "_RATE" => s__RATE(v),
            "_REST_OF_A_PERIOD" => s__REST_OF_A_PERIOD(v),
            "_REST_OF_A_PERIOD_WORDS" => s__REST_OF_A_PERIOD_WORDS(v),
            "_ROUND_IS_AROUND" => s__ROUND_IS_AROUND(v),
            "_SAYS_WEEKDAY" => s__SAYS_WEEKDAY(v),
            "_SEASONS" => s__SEASONS(v),
            "_SHAPE" => s__SHAPE(v),
            "_SIDE" => s__SIDE(v),
            "_SIDE_BY_SIDE" => s__SIDE_BY_SIDE(v),
            "_SINCE_ASK" => s__SINCE_ASK(v),
            "_SINCE_FILLER" => s__SINCE_FILLER(v),
            "_SINGULAR" => s__SINGULAR(v),
            "_SMALLER_PERIOD" => s__SMALLER_PERIOD(v),
            "_SMALL_NUMBER" => s__SMALL_NUMBER(v),
            "_SQUARE_WORD" => s__SQUARE_WORD(v),
            "_STATISTIC_WORDS" => s__STATISTIC_WORDS(v),
            "_SURFACES" => s__SURFACES(v),
            "_TAILS" => s__TAILS(v),
            "_TENS_WORDS" => s__TENS_WORDS(v),
            "_THE_TURN_OF_THE_YEAR" => s__THE_TURN_OF_THE_YEAR(v),
            "_TODAY_ASK" => s__TODAY_ASK(v),
            "_TO_WORK" => s__TO_WORK(v),
            "_UNARY_OPS" => s__UNARY_OPS(v),
            "_UNIT" => s__UNIT(v),
            "_UNITS" => s__UNITS(v),
            "_WAGE" => s__WAGE(v),
            "_WAGE_CONTEXT" => s__WAGE_CONTEXT(v),
            "_WAGE_CUE" => s__WAGE_CUE(v),
            "_WAGE_UNIT" => s__WAGE_UNIT(v),
            "_WEEKDAYS" => s__WEEKDAYS(v),
            "_WEEKDAY_WORDS" => s__WEEKDAY_WORDS(v),
            "_WORD_THOUSANDS" => s__WORD_THOUSANDS(v),
            "_WORKING_ASK" => s__WORKING_ASK(v),
            "_WORKING_LEFT" => s__WORKING_LEFT(v),
            "_WORKING_SHAPE" => s__WORKING_SHAPE(v),
            "_YOU_LEFT" => s__YOU_LEFT(v),
            "_YOU_LEFT_ANYWHERE" => s__YOU_LEFT_ANYWHERE(v),
            "a_rate_put_right" => s_a_rate_put_right(v),
            "entry" => s_entry(v),
            "in_a_safety_thread" => s_in_a_safety_thread(v),
            "the_money_already_held" => s_the_money_already_held(v),
            "the_pot_said_alone" => s_the_pot_said_alone(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.maths", module_obj);
    pyrt::xrefs::reg_fn("knowledge.maths", "_number", py_func("_number", |args: &[Py]| _number(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_parse", py_func("_parse", |args: &[Py]| _parse(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_eval", py_func("_eval", |args: &[Py]| _eval(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_expression", py_func("_expression", |args: &[Py]| _expression(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_calc", py_func("_calc", |args: &[Py]| _calc(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_root", py_func("_root", |args: &[Py]| _root(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_roots", py_func("_roots", |args: &[Py]| _roots(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_prime", py_func("_prime", |args: &[Py]| _prime(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_as_fraction", py_func("_as_fraction", |args: &[Py]| _as_fraction(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_fraction_text", py_func("_fraction_text", |args: &[Py]| _fraction_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_fraction_of_a_fraction", py_func("_fraction_of_a_fraction", |args: &[Py]| _fraction_of_a_fraction(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_fraction_of", py_func("_fraction_of", |args: &[Py]| _fraction_of(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_share_reply", py_func("_share_reply", |args: &[Py]| _share_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_count_value", py_func("_count_value", |args: &[Py]| _count_value(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_share", py_func("_share", |args: &[Py]| _share(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_dec", py_func("_dec", |args: &[Py]| _dec(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_fraction_decimal", py_func("_fraction_decimal", |args: &[Py]| _fraction_decimal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_exact_decimal", py_func("_exact_decimal", |args: &[Py]| _exact_decimal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_fraction", py_func("_fraction", |args: &[Py]| _fraction(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_reduced", py_func("_reduced", |args: &[Py]| _reduced(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_simplify", py_func("_simplify", |args: &[Py]| _simplify(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_add_fractions", py_func("_add_fractions", |args: &[Py]| _add_fractions(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_factors_in_list", py_func("_factors_in_list", |args: &[Py]| _factors_in_list(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_said_list", py_func("_said_list", |args: &[Py]| _said_list(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_factors", py_func("_the_factors", |args: &[Py]| _the_factors(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_common_factors", py_func("_common_factors", |args: &[Py]| _common_factors(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_round_to", py_func("_round_to", |args: &[Py]| _round_to(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_places", py_func("_places", |args: &[Py]| _places(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_circle_measure", py_func("_circle_measure", |args: &[Py]| _circle_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_box_volume", py_func("_box_volume", |args: &[Py]| _box_volume(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_factorised", py_func("_factorised", |args: &[Py]| _factorised(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_gcd", py_func("_gcd", |args: &[Py]| _gcd(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_quadratic", py_func("_quadratic", |args: &[Py]| _quadratic(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_equation", py_func("_equation", |args: &[Py]| _equation(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_percent", py_func("_percent", |args: &[Py]| _percent(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_percentage_change", py_func("_percentage_change", |args: &[Py]| _percentage_change(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_amount", py_func("_amount", |args: &[Py]| _amount(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_what_percent", py_func("_what_percent", |args: &[Py]| _what_percent(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_share_of_a_group", py_func("_a_share_of_a_group", |args: &[Py]| _a_share_of_a_group(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_percent_on", py_func("_percent_on", |args: &[Py]| _percent_on(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_percent_rise", py_func("_percent_rise", |args: &[Py]| _percent_rise(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_amount_in_the_chat", py_func("_amount_in_the_chat", |args: &[Py]| _amount_in_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_percent_of_the_figure_in_the_chat", py_func("_percent_of_the_figure_in_the_chat", |args: &[Py]| _percent_of_the_figure_in_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_tip", py_func("_tip", |args: &[Py]| _tip(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_two_places", py_func("_two_places", |args: &[Py]| _two_places(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_left_on_the_bill", py_func("_left_on_the_bill", |args: &[Py]| _left_on_the_bill(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_save_time", py_func("_save_time", |args: &[Py]| _save_time(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_money", py_func("_money", |args: &[Py]| _money(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_annual_pay", py_func("_annual_pay", |args: &[Py]| _annual_pay(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_wage", py_func("_wage", |args: &[Py]| _wage(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_travel", py_func("_travel", |args: &[Py]| _travel(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_distance_in_miles", py_func("_distance_in_miles", |args: &[Py]| _distance_in_miles(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_duration", py_func("_duration", |args: &[Py]| _duration(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_size_of_the_trip", py_func("_the_size_of_the_trip", |args: &[Py]| _the_size_of_the_trip(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_way_it_is_done", py_func("_the_way_it_is_done", |args: &[Py]| _the_way_it_is_done(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_at_a_pace", py_func("_at_a_pace", |args: &[Py]| _at_a_pace(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_convert", py_func("_convert", |args: &[Py]| _convert(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_rate_clause", py_func("_rate_clause", |args: &[Py]| _rate_clause(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_user_lines", py_func("_user_lines", |args: &[Py]| _user_lines(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_rate_was_said", py_func("_a_rate_was_said", |args: &[Py]| _a_rate_was_said(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_rate_in_chat", py_func("_rate_in_chat", |args: &[Py]| _rate_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_worked_in_chat", py_func("_worked_in_chat", |args: &[Py]| _worked_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_sums", py_func("_sums", |args: &[Py]| _sums(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_goal_in_chat", py_func("_goal_in_chat", |args: &[Py]| _goal_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_rate_and_total", py_func("_rate_and_total", |args: &[Py]| _rate_and_total(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_in_months", py_func("_in_months", |args: &[Py]| _in_months(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_duration_reply", py_func("_duration_reply", |args: &[Py]| _duration_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_time_to_reach", py_func("_time_to_reach", |args: &[Py]| _time_to_reach(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_asks_a_new_sum", py_func("_asks_a_new_sum", |args: &[Py]| _asks_a_new_sum(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_last_takes", py_func("_last_takes", |args: &[Py]| _last_takes(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_time_at_a_new_rate", py_func("_time_at_a_new_rate", |args: &[Py]| _time_at_a_new_rate(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_time_at_a_new_total", py_func("_time_at_a_new_total", |args: &[Py]| _time_at_a_new_total(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_time_without_a_goal", py_func("_time_without_a_goal", |args: &[Py]| _time_without_a_goal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_each_hundred", py_func("_each_hundred", |args: &[Py]| _each_hundred(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_saving_goal_in_chat", py_func("_a_saving_goal_in_chat", |args: &[Py]| _a_saving_goal_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_plan_figure", py_func("_the_plan_figure", |args: &[Py]| _the_plan_figure(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_goal_without_a_rate", py_func("_a_goal_without_a_rate", |args: &[Py]| _a_goal_without_a_rate(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_new_total_only", py_func("_a_new_total_only", |args: &[Py]| _a_new_total_only(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_figure_just_given", py_func("_figure_just_given", |args: &[Py]| _figure_just_given(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_each_month_for_a_goal", py_func("_each_month_for_a_goal", |args: &[Py]| _each_month_for_a_goal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_k_as_money", py_func("_k_as_money", |args: &[Py]| _k_as_money(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_easter_day", py_func("_easter_day", |args: &[Py]| _easter_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_last_day_of", py_func("_the_last_day_of", |args: &[Py]| _the_last_day_of(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_named_deadline", py_func("_a_named_deadline", |args: &[Py]| _a_named_deadline(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_save_by_a_date", py_func("_save_by_a_date", |args: &[Py]| _save_by_a_date(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_total_by_a_date", py_func("_a_total_by_a_date", |args: &[Py]| _a_total_by_a_date(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_month_for_the_deadline_in_chat", py_func("_a_month_for_the_deadline_in_chat", |args: &[Py]| _a_month_for_the_deadline_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_deadline_for_a_goal", py_func("_deadline_for_a_goal", |args: &[Py]| _deadline_for_a_goal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_asked_period", py_func("_asked_period", |args: &[Py]| _asked_period(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_deadline_in_chat", py_func("_a_deadline_in_chat", |args: &[Py]| _a_deadline_in_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_saving", py_func("_saving", |args: &[Py]| _saving(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_average", py_func("_average", |args: &[Py]| _average(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_sides", py_func("_sides", |args: &[Py]| _sides(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_surface", py_func("_surface", |args: &[Py]| _surface(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_square_text", py_func("_square_text", |args: &[Py]| _square_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_floor_area", py_func("_floor_area", |args: &[Py]| _floor_area(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_perimeter_of_a_room", py_func("_perimeter_of_a_room", |args: &[Py]| _perimeter_of_a_room(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_area", py_func("_area", |args: &[Py]| _area(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_month_year", py_func("_month_year", |args: &[Py]| _month_year(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_leap", py_func("_leap", |args: &[Py]| _leap(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_in_month", py_func("_days_in_month", |args: &[Py]| _days_in_month(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_christmas_name", py_func("_christmas_name", |args: &[Py]| _christmas_name(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_christmas_date_of", py_func("_christmas_date_of", |args: &[Py]| _christmas_date_of(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_christmas_day", py_func("_christmas_day", |args: &[Py]| _christmas_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_wait_text", py_func("_wait_text", |args: &[Py]| _wait_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_months_between", py_func("_months_between", |args: &[Py]| _months_between(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_count_in", py_func("_count_in", |args: &[Py]| _count_in(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_shopping_days", py_func("_shopping_days", |args: &[Py]| _shopping_days(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_countdown_text", py_func("_countdown_text", |args: &[Py]| _countdown_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_until", py_func("_days_until", |args: &[Py]| _days_until(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_weekday_text", py_func("_weekday_text", |args: &[Py]| _weekday_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_easter_sunday", py_func("_easter_sunday", |args: &[Py]| _easter_sunday(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_feast_year", py_func("_feast_year", |args: &[Py]| _feast_year(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_month_number", py_func("_month_number", |args: &[Py]| _month_number(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_day_in_a_month", py_func("_day_in_a_month", |args: &[Py]| _day_in_a_month(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_date_the_line_gave", py_func("_date_the_line_gave", |args: &[Py]| _date_the_line_gave(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_month_on", py_func("_month_on", |args: &[Py]| _month_on(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_first_of_the_month", py_func("_first_of_the_month", |args: &[Py]| _first_of_the_month(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_relative_day", py_func("_relative_day", |args: &[Py]| _relative_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_relative_count_text", py_func("_relative_count_text", |args: &[Py]| _relative_count_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_until_a_relative_day", py_func("_days_until_a_relative_day", |args: &[Py]| _days_until_a_relative_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_date_counted_from_today", py_func("_date_counted_from_today", |args: &[Py]| _date_counted_from_today(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_until_a_named_day", py_func("_days_until_a_named_day", |args: &[Py]| _days_until_a_named_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_left_in_the_month_or_the_year", py_func("_days_left_in_the_month_or_the_year", |args: &[Py]| _days_left_in_the_month_or_the_year(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_year_the_line_gave", py_func("_year_the_line_gave", |args: &[Py]| _year_the_line_gave(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_how_long_ago", py_func("_how_long_ago", |args: &[Py]| _how_long_ago(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_known_day_the_line_names_next", py_func("_a_known_day_the_line_names_next", |args: &[Py]| _a_known_day_the_line_names_next(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_working_week", py_func("_a_working_week", |args: &[Py]| _a_working_week(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_working_days_or_weekends_until", py_func("_working_days_or_weekends_until", |args: &[Py]| _working_days_or_weekends_until(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_known_day_that_has_gone", py_func("_a_known_day_that_has_gone", |args: &[Py]| _a_known_day_that_has_gone(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_until_easter", py_func("_days_until_easter", |args: &[Py]| _days_until_easter(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_nth_weekday", py_func("_nth_weekday", |args: &[Py]| _nth_weekday(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_named_day_date", py_func("_named_day_date", |args: &[Py]| _named_day_date(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_days_until_a_month_or_season", py_func("_days_until_a_month_or_season", |args: &[Py]| _days_until_a_month_or_season(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_ordinal", py_func("_ordinal", |args: &[Py]| _ordinal(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_weekday_of_a_date", py_func("_the_weekday_of_a_date", |args: &[Py]| _the_weekday_of_a_date(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_the_wider_day", py_func("_the_wider_day", |args: &[Py]| _the_wider_day(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_last_weekday", py_func("_last_weekday", |args: &[Py]| _last_weekday(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_bank_holidays", py_func("_bank_holidays", |args: &[Py]| _bank_holidays(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_bank_where", py_func("_bank_where", |args: &[Py]| _bank_where(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_said_with_a_weekday", py_func("_said_with_a_weekday", |args: &[Py]| _said_with_a_weekday(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_next_bank_holiday", py_func("_next_bank_holiday", |args: &[Py]| _next_bank_holiday(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_day_text", py_func("_day_text", |args: &[Py]| _day_text(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_short_date", py_func("_short_date", |args: &[Py]| _short_date(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_weekday_of", py_func("_weekday_of", |args: &[Py]| _weekday_of(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_today", py_func("_today", |args: &[Py]| _today(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_unit_pair", py_func("_unit_pair", |args: &[Py]| _unit_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_unit_fact", py_func("_unit_fact", |args: &[Py]| _unit_fact(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_hours_a_week", py_func("_hours_a_week", |args: &[Py]| _hours_a_week(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_hourly_named", py_func("_hourly_named", |args: &[Py]| _hourly_named(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_yearly_named", py_func("_yearly_named", |args: &[Py]| _yearly_named(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_pounds", py_func("_pounds", |args: &[Py]| _pounds(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_rate", py_func("_rate", |args: &[Py]| _rate(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_yearline", py_func("_a_yearline", |args: &[Py]| _a_yearline(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_wage_rate", py_func("_wage_rate", |args: &[Py]| _wage_rate(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "solve", py_func("solve", |args: &[Py]| solve(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_figures", py_func("_figures", |args: &[Py]| _figures(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_short", py_func("_short", |args: &[Py]| _short(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_names_a_statistic", py_func("_names_a_statistic", |args: &[Py]| _names_a_statistic(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_about_words", py_func("_about_words", |args: &[Py]| _about_words(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_averages", py_func("_averages", |args: &[Py]| _averages(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_a_rounding_question", py_func("_a_rounding_question", |args: &[Py]| _a_rounding_question(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_rounding_is_the_ask", py_func("_rounding_is_the_ask", |args: &[Py]| _rounding_is_the_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_round_is_the_verb", py_func("_round_is_the_verb", |args: &[Py]| _round_is_the_verb(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_not_a_spare_hour", py_func("_not_a_spare_hour", |args: &[Py]| _not_a_spare_hour(args)));
    pyrt::xrefs::reg_fn("knowledge.maths", "_prime_definition", py_func("_prime_definition", |args: &[Py]| _prime_definition(args)));
    pyrt::xrefs::reg_access("knowledge.maths", "ABOUT_ASK", g_ABOUT_ASK, s_ABOUT_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.maths", "ASKED_FOR_FIGURE", g_ASKED_FOR_FIGURE, s_ASKED_FOR_FIGURE);
    pyrt::xrefs::reg_access("knowledge.maths", "ASKED_FOR_RATE", g_ASKED_FOR_RATE, s_ASKED_FOR_RATE);
    pyrt::xrefs::reg_access("knowledge.maths", "A_BARE_RATE_ASK", g_A_BARE_RATE_ASK, s_A_BARE_RATE_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "A_DATE_FOR_THE_THING", g_A_DATE_FOR_THE_THING, s_A_DATE_FOR_THE_THING);
    pyrt::xrefs::reg_access("knowledge.maths", "A_DEADLINE_DAY", g_A_DEADLINE_DAY, s_A_DEADLINE_DAY);
    pyrt::xrefs::reg_access("knowledge.maths", "A_DEBT_CLEARED", g_A_DEBT_CLEARED, s_A_DEBT_CLEARED);
    pyrt::xrefs::reg_access("knowledge.maths", "A_HORIZON", g_A_HORIZON, s_A_HORIZON);
    pyrt::xrefs::reg_access("knowledge.maths", "CLAUSE_SPLIT", g_CLAUSE_SPLIT, s_CLAUSE_SPLIT);
    pyrt::xrefs::reg_access("knowledge.maths", "CLOCK", g_CLOCK, s_CLOCK);
    pyrt::xrefs::reg_access("knowledge.maths", "DEADLINE_DAYS", g_DEADLINE_DAYS, s_DEADLINE_DAYS);
    pyrt::xrefs::reg_access("knowledge.maths", "DEADLINE_NAMED", g_DEADLINE_NAMED, s_DEADLINE_NAMED);
    pyrt::xrefs::reg_access("knowledge.maths", "EACH_PERIOD", g_EACH_PERIOD, s_EACH_PERIOD);
    pyrt::xrefs::reg_access("knowledge.maths", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.maths", "FACTS", g_FACTS, s_FACTS);
    pyrt::xrefs::reg_access("knowledge.maths", "GOAL", g_GOAL, s_GOAL);
    pyrt::xrefs::reg_access("knowledge.maths", "HORIZON_COUNTS", g_HORIZON_COUNTS, s_HORIZON_COUNTS);
    pyrt::xrefs::reg_access("knowledge.maths", "HORIZON_SAID", g_HORIZON_SAID, s_HORIZON_SAID);
    pyrt::xrefs::reg_access("knowledge.maths", "HOW_HEAVY", g_HOW_HEAVY, s_HOW_HEAVY);
    pyrt::xrefs::reg_access("knowledge.maths", "HOW_MUCH_EACH", g_HOW_MUCH_EACH, s_HOW_MUCH_EACH);
    pyrt::xrefs::reg_access("knowledge.maths", "JUST_A_TOTAL", g_JUST_A_TOTAL, s_JUST_A_TOTAL);
    pyrt::xrefs::reg_access("knowledge.maths", "K_THOUSANDS", g_K_THOUSANDS, s_K_THOUSANDS);
    pyrt::xrefs::reg_access("knowledge.maths", "MATHS_DEFINITION_ASK", g_MATHS_DEFINITION_ASK, s_MATHS_DEFINITION_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("knowledge.maths", "MONEY_GOAL", g_MONEY_GOAL, s_MONEY_GOAL);
    pyrt::xrefs::reg_access("knowledge.maths", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("knowledge.maths", "MONTH_NUMBERS", g_MONTH_NUMBERS, s_MONTH_NUMBERS);
    pyrt::xrefs::reg_access("knowledge.maths", "OWED", g_OWED, s_OWED);
    pyrt::xrefs::reg_access("knowledge.maths", "PAST_REPORT", g_PAST_REPORT, s_PAST_REPORT);
    pyrt::xrefs::reg_access("knowledge.maths", "PERIODS", g_PERIODS, s_PERIODS);
    pyrt::xrefs::reg_access("knowledge.maths", "PERIODS_IN_A_YEAR", g_PERIODS_IN_A_YEAR, s_PERIODS_IN_A_YEAR);
    pyrt::xrefs::reg_access("knowledge.maths", "PRIME_DEFINITION", g_PRIME_DEFINITION, s_PRIME_DEFINITION);
    pyrt::xrefs::reg_access("knowledge.maths", "PRIME_TEST", g_PRIME_TEST, s_PRIME_TEST);
    pyrt::xrefs::reg_access("knowledge.maths", "RATE", g_RATE, s_RATE);
    pyrt::xrefs::reg_access("knowledge.maths", "ROUNDING_NAMED", g_ROUNDING_NAMED, s_ROUNDING_NAMED);
    pyrt::xrefs::reg_access("knowledge.maths", "ROUND_IS_NOT_THE_VERB", g_ROUND_IS_NOT_THE_VERB, s_ROUND_IS_NOT_THE_VERB);
    pyrt::xrefs::reg_access("knowledge.maths", "SAID_A_RATE", g_SAID_A_RATE, s_SAID_A_RATE);
    pyrt::xrefs::reg_access("knowledge.maths", "SAVE_VERB", g_SAVE_VERB, s_SAVE_VERB);
    pyrt::xrefs::reg_access("knowledge.maths", "SAVING_CLAUSE", g_SAVING_CLAUSE, s_SAVING_CLAUSE);
    pyrt::xrefs::reg_access("knowledge.maths", "SAVING_CUE", g_SAVING_CUE, s_SAVING_CUE);
    pyrt::xrefs::reg_access("knowledge.maths", "SPENDING_CLAUSE", g_SPENDING_CLAUSE, s_SPENDING_CLAUSE);
    pyrt::xrefs::reg_access("knowledge.maths", "TAKES", g_TAKES, s_TAKES);
    pyrt::xrefs::reg_access("knowledge.maths", "TIME_ASKED", g_TIME_ASKED, s_TIME_ASKED);
    pyrt::xrefs::reg_access("knowledge.maths", "TIME_ELSEWHERE", g_TIME_ELSEWHERE, s_TIME_ELSEWHERE);
    pyrt::xrefs::reg_access("knowledge.maths", "TOWARDS", g_TOWARDS, s_TOWARDS);
    pyrt::xrefs::reg_access("knowledge.maths", "WHAT_IF", g_WHAT_IF, s_WHAT_IF);
    pyrt::xrefs::reg_access("knowledge.maths", "WORKED", g_WORKED, s_WORKED);
    pyrt::xrefs::reg_access("knowledge.maths", "_AFTER_HALF", g__AFTER_HALF, s__AFTER_HALF);
    pyrt::xrefs::reg_access("knowledge.maths", "_AMOUNT", g__AMOUNT, s__AMOUNT);
    pyrt::xrefs::reg_access("knowledge.maths", "_ANNUAL", g__ANNUAL, s__ANNUAL);
    pyrt::xrefs::reg_access("knowledge.maths", "_AN_HOUR", g__AN_HOUR, s__AN_HOUR);
    pyrt::xrefs::reg_access("knowledge.maths", "_AREA_ASK", g__AREA_ASK, s__AREA_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_AREA_TARGET", g__AREA_TARGET, s__AREA_TARGET);
    pyrt::xrefs::reg_access("knowledge.maths", "_ASK_DATE", g__ASK_DATE, s__ASK_DATE);
    pyrt::xrefs::reg_access("knowledge.maths", "_AVERAGE_EXPLAIN", g__AVERAGE_EXPLAIN, s__AVERAGE_EXPLAIN);
    pyrt::xrefs::reg_access("knowledge.maths", "_AVERAGE_FILLER", g__AVERAGE_FILLER, s__AVERAGE_FILLER);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_COUNT", g__A_COUNT, s__A_COUNT);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_DISTANCE_STATED", g__A_DISTANCE_STATED, s__A_DISTANCE_STATED);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_LENGTH_OF_TIME", g__A_LENGTH_OF_TIME, s__A_LENGTH_OF_TIME);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_NOTE", g__A_NOTE, s__A_NOTE);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_PACE_WORD", g__A_PACE_WORD, s__A_PACE_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_PAY_WORD", g__A_PAY_WORD, s__A_PAY_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_A_YEAR_IN_THE_LINE", g__A_YEAR_IN_THE_LINE, s__A_YEAR_IN_THE_LINE);
    pyrt::xrefs::reg_access("knowledge.maths", "_BANK_HOLIDAY_COUNT", g__BANK_HOLIDAY_COUNT, s__BANK_HOLIDAY_COUNT);
    pyrt::xrefs::reg_access("knowledge.maths", "_BANK_HOLIDAY_LIST", g__BANK_HOLIDAY_LIST, s__BANK_HOLIDAY_LIST);
    pyrt::xrefs::reg_access("knowledge.maths", "_BANK_HOLIDAY_NEXT", g__BANK_HOLIDAY_NEXT, s__BANK_HOLIDAY_NEXT);
    pyrt::xrefs::reg_access("knowledge.maths", "_BANK_HOLIDAY_TAIL", g__BANK_HOLIDAY_TAIL, s__BANK_HOLIDAY_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_BANK_WHERE", g__BANK_WHERE, s__BANK_WHERE);
    pyrt::xrefs::reg_access("knowledge.maths", "_BILL_WORD", g__BILL_WORD, s__BILL_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_BIN_OPS", g__BIN_OPS, s__BIN_OPS);
    pyrt::xrefs::reg_access("knowledge.maths", "_BOUGHT_BY_THE_BAG", g__BOUGHT_BY_THE_BAG, s__BOUGHT_BY_THE_BAG);
    pyrt::xrefs::reg_access("knowledge.maths", "_BUYING", g__BUYING, s__BUYING);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_COUNTDOWN", g__CHRISTMAS_COUNTDOWN, s__CHRISTMAS_COUNTDOWN);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_DATE", g__CHRISTMAS_DATE, s__CHRISTMAS_DATE);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_DAY", g__CHRISTMAS_DAY, s__CHRISTMAS_DAY);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_FIXED", g__CHRISTMAS_FIXED, s__CHRISTMAS_FIXED);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_TAIL", g__CHRISTMAS_TAIL, s__CHRISTMAS_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_TODAY", g__CHRISTMAS_TODAY, s__CHRISTMAS_TODAY);
    pyrt::xrefs::reg_access("knowledge.maths", "_CHRISTMAS_WEEKDAY", g__CHRISTMAS_WEEKDAY, s__CHRISTMAS_WEEKDAY);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNT", g__COUNT, s__COUNT);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNTED", g__COUNTED, s__COUNTED);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNTED_EVENT", g__COUNTED_EVENT, s__COUNTED_EVENT);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNTED_GAP", g__COUNTED_GAP, s__COUNTED_GAP);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNTED_SUBJECT_TAIL", g__COUNTED_SUBJECT_TAIL, s__COUNTED_SUBJECT_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNTS", g__COUNTS, s__COUNTS);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNT_ASK", g__COUNT_ASK, s__COUNT_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_COUNT_VALUE", g__COUNT_VALUE, s__COUNT_VALUE);
    pyrt::xrefs::reg_access("knowledge.maths", "_CUP_IN_ML", g__CUP_IN_ML, s__CUP_IN_ML);
    pyrt::xrefs::reg_access("knowledge.maths", "_CUP_IN_TBSP", g__CUP_IN_TBSP, s__CUP_IN_TBSP);
    pyrt::xrefs::reg_access("knowledge.maths", "_DATE_TAIL", g__DATE_TAIL, s__DATE_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_DAY_MONTH", g__DAY_MONTH, s__DAY_MONTH);
    pyrt::xrefs::reg_access("knowledge.maths", "_DAY_ONLY", g__DAY_ONLY, s__DAY_ONLY);
    pyrt::xrefs::reg_access("knowledge.maths", "_DEFINITION_ASK", g__DEFINITION_ASK, s__DEFINITION_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_DISTANCE", g__DISTANCE, s__DISTANCE);
    pyrt::xrefs::reg_access("knowledge.maths", "_EASTER_OFFSETS", g__EASTER_OFFSETS, s__EASTER_OFFSETS);
    pyrt::xrefs::reg_access("knowledge.maths", "_FACTOR_ASK", g__FACTOR_ASK, s__FACTOR_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_FILLER", g__FILLER, s__FILLER);
    pyrt::xrefs::reg_access("knowledge.maths", "_FRACTION", g__FRACTION, s__FRACTION);
    pyrt::xrefs::reg_access("knowledge.maths", "_FRACTIONS", g__FRACTIONS, s__FRACTIONS);
    pyrt::xrefs::reg_access("knowledge.maths", "_FRACTION_OF_A_FRACTION", g__FRACTION_OF_A_FRACTION, s__FRACTION_OF_A_FRACTION);
    pyrt::xrefs::reg_access("knowledge.maths", "_FRACTION_OF_FRACTION", g__FRACTION_OF_FRACTION, s__FRACTION_OF_FRACTION);
    pyrt::xrefs::reg_access("knowledge.maths", "_FRACTION_UNITS", g__FRACTION_UNITS, s__FRACTION_UNITS);
    pyrt::xrefs::reg_access("knowledge.maths", "_FREE_TIME_ASK", g__FREE_TIME_ASK, s__FREE_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_HEAVY", g__HEAVY, s__HEAVY);
    pyrt::xrefs::reg_access("knowledge.maths", "_HOURS_WEEK", g__HOURS_WEEK, s__HOURS_WEEK);
    pyrt::xrefs::reg_access("knowledge.maths", "_HOURS_WORKED", g__HOURS_WORKED, s__HOURS_WORKED);
    pyrt::xrefs::reg_access("knowledge.maths", "_HOW_FAR", g__HOW_FAR, s__HOW_FAR);
    pyrt::xrefs::reg_access("knowledge.maths", "_HOW_MUCH", g__HOW_MUCH, s__HOW_MUCH);
    pyrt::xrefs::reg_access("knowledge.maths", "_IS_THAT_TOO_MUCH", g__IS_THAT_TOO_MUCH, s__IS_THAT_TOO_MUCH);
    pyrt::xrefs::reg_access("knowledge.maths", "_K_THOUSANDS_HERE", g__K_THOUSANDS_HERE, s__K_THOUSANDS_HERE);
    pyrt::xrefs::reg_access("knowledge.maths", "_LENGTH", g__LENGTH, s__LENGTH);
    pyrt::xrefs::reg_access("knowledge.maths", "_LENGTH_UNIT", g__LENGTH_UNIT, s__LENGTH_UNIT);
    pyrt::xrefs::reg_access("knowledge.maths", "_MATHS_WORDS", g__MATHS_WORDS, s__MATHS_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_MEAN_VERB", g__MEAN_VERB, s__MEAN_VERB);
    pyrt::xrefs::reg_access("knowledge.maths", "_MEDIAN_EXPLAIN", g__MEDIAN_EXPLAIN, s__MEDIAN_EXPLAIN);
    pyrt::xrefs::reg_access("knowledge.maths", "_METRES_SQUARED", g__METRES_SQUARED, s__METRES_SQUARED);
    pyrt::xrefs::reg_access("knowledge.maths", "_MODE_EXPLAIN", g__MODE_EXPLAIN, s__MODE_EXPLAIN);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONEY_BESIDE", g__MONEY_BESIDE, s__MONEY_BESIDE);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONEY_HERE", g__MONEY_HERE, s__MONEY_HERE);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONEY_WORD", g__MONEY_WORD, s__MONEY_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTHS_OR", g__MONTHS_OR, s__MONTHS_OR);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTHS_SAID", g__MONTHS_SAID, s__MONTHS_SAID);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTH_NAMES", g__MONTH_NAMES, s__MONTH_NAMES);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTH_SHORT", g__MONTH_SHORT, s__MONTH_SHORT);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTH_TAIL", g__MONTH_TAIL, s__MONTH_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTH_TITLE", g__MONTH_TITLE, s__MONTH_TITLE);
    pyrt::xrefs::reg_access("knowledge.maths", "_MONTH_WORD", g__MONTH_WORD, s__MONTH_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_NAMED_DAYS", g__NAMED_DAYS, s__NAMED_DAYS);
    pyrt::xrefs::reg_access("knowledge.maths", "_NEAREST", g__NEAREST, s__NEAREST);
    pyrt::xrefs::reg_access("knowledge.maths", "_NOT_PAY", g__NOT_PAY, s__NOT_PAY);
    pyrt::xrefs::reg_access("knowledge.maths", "_NUMBER", g__NUMBER, s__NUMBER);
    pyrt::xrefs::reg_access("knowledge.maths", "_NUMBER_THOUSANDS", g__NUMBER_THOUSANDS, s__NUMBER_THOUSANDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_OTHER_STATISTICS", g__OTHER_STATISTICS, s__OTHER_STATISTICS);
    pyrt::xrefs::reg_access("knowledge.maths", "_PACES", g__PACES, s__PACES);
    pyrt::xrefs::reg_access("knowledge.maths", "_PART", g__PART, s__PART);
    pyrt::xrefs::reg_access("knowledge.maths", "_PAY_WORDS", g__PAY_WORDS, s__PAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERCENT_ADVICE", g__PERCENT_ADVICE, s__PERCENT_ADVICE);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERCENT_ASK_TAIL", g__PERCENT_ASK_TAIL, s__PERCENT_ASK_TAIL);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERCENT_ASK_WORD", g__PERCENT_ASK_WORD, s__PERCENT_ASK_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERCENT_DOWN", g__PERCENT_DOWN, s__PERCENT_DOWN);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERCENT_UP", g__PERCENT_UP, s__PERCENT_UP);
    pyrt::xrefs::reg_access("knowledge.maths", "_PERIMETER_ASK", g__PERIMETER_ASK, s__PERIMETER_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_PLACES", g__PLACES, s__PLACES);
    pyrt::xrefs::reg_access("knowledge.maths", "_POINTS_AT_A_THING", g__POINTS_AT_A_THING, s__POINTS_AT_A_THING);
    pyrt::xrefs::reg_access("knowledge.maths", "_POLITE", g__POLITE, s__POLITE);
    pyrt::xrefs::reg_access("knowledge.maths", "_QUADRATIC_ASK", g__QUADRATIC_ASK, s__QUADRATIC_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_RATE", g__RATE, s__RATE);
    pyrt::xrefs::reg_access("knowledge.maths", "_REST_OF_A_PERIOD", g__REST_OF_A_PERIOD, s__REST_OF_A_PERIOD);
    pyrt::xrefs::reg_access("knowledge.maths", "_REST_OF_A_PERIOD_WORDS", g__REST_OF_A_PERIOD_WORDS, s__REST_OF_A_PERIOD_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_ROUND_IS_AROUND", g__ROUND_IS_AROUND, s__ROUND_IS_AROUND);
    pyrt::xrefs::reg_access("knowledge.maths", "_SAYS_WEEKDAY", g__SAYS_WEEKDAY, s__SAYS_WEEKDAY);
    pyrt::xrefs::reg_access("knowledge.maths", "_SEASONS", g__SEASONS, s__SEASONS);
    pyrt::xrefs::reg_access("knowledge.maths", "_SHAPE", g__SHAPE, s__SHAPE);
    pyrt::xrefs::reg_access("knowledge.maths", "_SIDE", g__SIDE, s__SIDE);
    pyrt::xrefs::reg_access("knowledge.maths", "_SIDE_BY_SIDE", g__SIDE_BY_SIDE, s__SIDE_BY_SIDE);
    pyrt::xrefs::reg_access("knowledge.maths", "_SINCE_ASK", g__SINCE_ASK, s__SINCE_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_SINCE_FILLER", g__SINCE_FILLER, s__SINCE_FILLER);
    pyrt::xrefs::reg_access("knowledge.maths", "_SINGULAR", g__SINGULAR, s__SINGULAR);
    pyrt::xrefs::reg_access("knowledge.maths", "_SMALLER_PERIOD", g__SMALLER_PERIOD, s__SMALLER_PERIOD);
    pyrt::xrefs::reg_access("knowledge.maths", "_SMALL_NUMBER", g__SMALL_NUMBER, s__SMALL_NUMBER);
    pyrt::xrefs::reg_access("knowledge.maths", "_SQUARE_WORD", g__SQUARE_WORD, s__SQUARE_WORD);
    pyrt::xrefs::reg_access("knowledge.maths", "_STATISTIC_WORDS", g__STATISTIC_WORDS, s__STATISTIC_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_SURFACES", g__SURFACES, s__SURFACES);
    pyrt::xrefs::reg_access("knowledge.maths", "_TAILS", g__TAILS, s__TAILS);
    pyrt::xrefs::reg_access("knowledge.maths", "_TENS_WORDS", g__TENS_WORDS, s__TENS_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_THE_TURN_OF_THE_YEAR", g__THE_TURN_OF_THE_YEAR, s__THE_TURN_OF_THE_YEAR);
    pyrt::xrefs::reg_access("knowledge.maths", "_TODAY_ASK", g__TODAY_ASK, s__TODAY_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_TO_WORK", g__TO_WORK, s__TO_WORK);
    pyrt::xrefs::reg_access("knowledge.maths", "_UNARY_OPS", g__UNARY_OPS, s__UNARY_OPS);
    pyrt::xrefs::reg_access("knowledge.maths", "_UNIT", g__UNIT, s__UNIT);
    pyrt::xrefs::reg_access("knowledge.maths", "_UNITS", g__UNITS, s__UNITS);
    pyrt::xrefs::reg_access("knowledge.maths", "_WAGE", g__WAGE, s__WAGE);
    pyrt::xrefs::reg_access("knowledge.maths", "_WAGE_CONTEXT", g__WAGE_CONTEXT, s__WAGE_CONTEXT);
    pyrt::xrefs::reg_access("knowledge.maths", "_WAGE_CUE", g__WAGE_CUE, s__WAGE_CUE);
    pyrt::xrefs::reg_access("knowledge.maths", "_WAGE_UNIT", g__WAGE_UNIT, s__WAGE_UNIT);
    pyrt::xrefs::reg_access("knowledge.maths", "_WEEKDAYS", g__WEEKDAYS, s__WEEKDAYS);
    pyrt::xrefs::reg_access("knowledge.maths", "_WEEKDAY_WORDS", g__WEEKDAY_WORDS, s__WEEKDAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_WORD_THOUSANDS", g__WORD_THOUSANDS, s__WORD_THOUSANDS);
    pyrt::xrefs::reg_access("knowledge.maths", "_WORKING_ASK", g__WORKING_ASK, s__WORKING_ASK);
    pyrt::xrefs::reg_access("knowledge.maths", "_WORKING_LEFT", g__WORKING_LEFT, s__WORKING_LEFT);
    pyrt::xrefs::reg_access("knowledge.maths", "_WORKING_SHAPE", g__WORKING_SHAPE, s__WORKING_SHAPE);
    pyrt::xrefs::reg_access("knowledge.maths", "_YOU_LEFT", g__YOU_LEFT, s__YOU_LEFT);
    pyrt::xrefs::reg_access("knowledge.maths", "_YOU_LEFT_ANYWHERE", g__YOU_LEFT_ANYWHERE, s__YOU_LEFT_ANYWHERE);
    pyrt::xrefs::reg_access("knowledge.maths", "a_rate_put_right", g_a_rate_put_right, s_a_rate_put_right);
    pyrt::xrefs::reg_access("knowledge.maths", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.maths", "in_a_safety_thread", g_in_a_safety_thread, s_in_a_safety_thread);
    pyrt::xrefs::reg_access("knowledge.maths", "the_money_already_held", g_the_money_already_held, s_the_money_already_held);
    pyrt::xrefs::reg_access("knowledge.maths", "the_pot_said_alone", g_the_pot_said_alone, s_the_pot_said_alone);
}

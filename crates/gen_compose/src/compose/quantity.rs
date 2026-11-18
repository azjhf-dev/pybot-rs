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


thread_local! { static G_ABOUT_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AREA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUDGET_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BUDGET_THEY_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHANGE_IS_THIS_SMALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHANGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_ASKED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIMIT_ON_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_ABOUT_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RANGE_WITH_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REMAINDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SINGLE_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_EITHER_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STRETCH_THAT_IS_NOT_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAY_OF_PAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHAT_IS_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOUNDED_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTANCE_DAY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTANCE_FAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTANCE_FAR_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTANCE_MID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTANCE_NEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_ON_THE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ABOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_BELOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_INSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_WEIGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_UNIT_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_TABLE_DISTANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_TABLE_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PER_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVINGS_AFTER_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMETHING_WEIGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_WEEKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM_IN_A_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEN_TIMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_HAVE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_HAVE_THE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HOUSEHOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_BUDGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_ON_THE_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_A_FIGURE() -> Py { ensure_init(); G_ABOUT_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_A_FIGURE(v: Py) { ensure_init(); G_ABOUT_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AGE() -> Py { ensure_init(); G_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE(v: Py) { ensure_init(); G_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_AREA() -> Py { ensure_init(); G_AREA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AREA(v: Py) { ensure_init(); G_AREA.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOUND() -> Py { ensure_init(); G_A_BOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOUND(v: Py) { ensure_init(); G_A_BOUND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUDGET_QUESTION() -> Py { ensure_init(); G_A_BUDGET_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUDGET_QUESTION(v: Py) { ensure_init(); G_A_BUDGET_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BUDGET_THEY_SET() -> Py { ensure_init(); G_A_BUDGET_THEY_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BUDGET_THEY_SET(v: Py) { ensure_init(); G_A_BUDGET_THEY_SET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHANGE_IS_THIS_SMALL() -> Py { ensure_init(); G_A_CHANGE_IS_THIS_SMALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHANGE_IS_THIS_SMALL(v: Py) { ensure_init(); G_A_CHANGE_IS_THIS_SMALL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHANGE_SAID() -> Py { ensure_init(); G_A_CHANGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHANGE_SAID(v: Py) { ensure_init(); G_A_CHANGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_ASKED_FOR() -> Py { ensure_init(); G_A_COUNT_ASKED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_ASKED_FOR(v: Py) { ensure_init(); G_A_COUNT_ASKED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_ALONE() -> Py { ensure_init(); G_A_FIGURE_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_ALONE(v: Py) { ensure_init(); G_A_FIGURE_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIMIT_ON_A_LENGTH() -> Py { ensure_init(); G_A_LIMIT_ON_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT_ON_A_LENGTH(v: Py) { ensure_init(); G_A_LIMIT_ON_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN() -> Py { ensure_init(); G_A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN(v: Py) { ensure_init(); G_A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_ABOUT_A_LENGTH() -> Py { ensure_init(); G_A_QUESTION_ABOUT_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ABOUT_A_LENGTH(v: Py) { ensure_init(); G_A_QUESTION_ABOUT_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RANGE_WITH_A_UNIT() -> Py { ensure_init(); G_A_RANGE_WITH_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RANGE_WITH_A_UNIT(v: Py) { ensure_init(); G_A_RANGE_WITH_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REMAINDER() -> Py { ensure_init(); G_A_REMAINDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REMAINDER(v: Py) { ensure_init(); G_A_REMAINDER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SINGLE_RULE() -> Py { ensure_init(); G_A_SINGLE_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SINGLE_RULE(v: Py) { ensure_init(); G_A_SINGLE_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_EITHER_WAY() -> Py { ensure_init(); G_A_STEP_EITHER_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_EITHER_WAY(v: Py) { ensure_init(); G_A_STEP_EITHER_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STRETCH_THAT_IS_NOT_SHORT() -> Py { ensure_init(); G_A_STRETCH_THAT_IS_NOT_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STRETCH_THAT_IS_NOT_SHORT(v: Py) { ensure_init(); G_A_STRETCH_THAT_IS_NOT_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAY_OF_PAYING() -> Py { ensure_init(); G_A_WAY_OF_PAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAY_OF_PAYING(v: Py) { ensure_init(); G_A_WAY_OF_PAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHAT_IS_LEFT() -> Py { ensure_init(); G_A_WHAT_IS_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_IS_LEFT(v: Py) { ensure_init(); G_A_WHAT_IS_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_BOUNDED_BY() -> Py { ensure_init(); G_BOUNDED_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOUNDED_BY(v: Py) { ensure_init(); G_BOUNDED_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRENCY() -> Py { ensure_init(); G_CURRENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRENCY(v: Py) { ensure_init(); G_CURRENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTANCE_DAY_OUT() -> Py { ensure_init(); G_DISTANCE_DAY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_DAY_OUT(v: Py) { ensure_init(); G_DISTANCE_DAY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTANCE_FAR() -> Py { ensure_init(); G_DISTANCE_FAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_FAR(v: Py) { ensure_init(); G_DISTANCE_FAR.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTANCE_FAR_TRIP() -> Py { ensure_init(); G_DISTANCE_FAR_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_FAR_TRIP(v: Py) { ensure_init(); G_DISTANCE_FAR_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTANCE_MID() -> Py { ensure_init(); G_DISTANCE_MID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_MID(v: Py) { ensure_init(); G_DISTANCE_MID.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTANCE_NEAR() -> Py { ensure_init(); G_DISTANCE_NEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_NEAR(v: Py) { ensure_init(); G_DISTANCE_NEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_ON_THE_FIGURE() -> Py { ensure_init(); G_DOUBT_ON_THE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_ON_THE_FIGURE(v: Py) { ensure_init(); G_DOUBT_ON_THE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ABOVE() -> Py { ensure_init(); G_FIGURE_ABOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ABOVE(v: Py) { ensure_init(); G_FIGURE_ABOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_BELOW() -> Py { ensure_init(); G_FIGURE_BELOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_BELOW(v: Py) { ensure_init(); G_FIGURE_BELOW.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_INSIDE() -> Py { ensure_init(); G_FIGURE_INSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_INSIDE(v: Py) { ensure_init(); G_FIGURE_INSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGES() -> Py { ensure_init(); G_HEDGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGES(v: Py) { ensure_init(); G_HEDGES.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME() -> Py { ensure_init(); G_INCOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME(v: Py) { ensure_init(); G_INCOME.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME_MONTH() -> Py { ensure_init(); G_INCOME_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME_MONTH(v: Py) { ensure_init(); G_INCOME_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME_PLAIN() -> Py { ensure_init(); G_INCOME_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME_PLAIN(v: Py) { ensure_init(); G_INCOME_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME_WEEK() -> Py { ensure_init(); G_INCOME_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME_WEEK(v: Py) { ensure_init(); G_INCOME_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME_YEAR() -> Py { ensure_init(); G_INCOME_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME_YEAR(v: Py) { ensure_init(); G_INCOME_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_I_WEIGH() -> Py { ensure_init(); G_I_WEIGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_WEIGH(v: Py) { ensure_init(); G_I_WEIGH.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_DAY() -> Py { ensure_init(); G_MONEY_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_DAY(v: Py) { ensure_init(); G_MONEY_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_MONTH() -> Py { ensure_init(); G_MONEY_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_MONTH(v: Py) { ensure_init(); G_MONEY_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_NOUN() -> Py { ensure_init(); G_MONEY_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_NOUN(v: Py) { ensure_init(); G_MONEY_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_PLAIN() -> Py { ensure_init(); G_MONEY_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_PLAIN(v: Py) { ensure_init(); G_MONEY_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_WEEK() -> Py { ensure_init(); G_MONEY_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_WEEK(v: Py) { ensure_init(); G_MONEY_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_YEAR() -> Py { ensure_init(); G_MONEY_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_YEAR(v: Py) { ensure_init(); G_MONEY_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_UNIT_ITSELF() -> Py { ensure_init(); G_NOT_A_UNIT_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_UNIT_ITSELF(v: Py) { ensure_init(); G_NOT_A_UNIT_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBERS() -> Py { ensure_init(); G_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBERS(v: Py) { ensure_init(); G_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_TABLE_DISTANCE() -> Py { ensure_init(); G_ON_THE_TABLE_DISTANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_DISTANCE(v: Py) { ensure_init(); G_ON_THE_TABLE_DISTANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_TABLE_MONEY() -> Py { ensure_init(); G_ON_THE_TABLE_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_MONEY(v: Py) { ensure_init(); G_ON_THE_TABLE_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIODS() -> Py { ensure_init(); G_PERIODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIODS(v: Py) { ensure_init(); G_PERIODS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD_NAMED() -> Py { ensure_init(); G_PERIOD_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD_NAMED(v: Py) { ensure_init(); G_PERIOD_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_PER_DAY() -> Py { ensure_init(); G_PER_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PER_DAY(v: Py) { ensure_init(); G_PER_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_BACK() -> Py { ensure_init(); G_POINTING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_BACK(v: Py) { ensure_init(); G_POINTING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVINGS() -> Py { ensure_init(); G_SAVINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVINGS(v: Py) { ensure_init(); G_SAVINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVINGS_AFTER_A_RATE() -> Py { ensure_init(); G_SAVINGS_AFTER_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVINGS_AFTER_A_RATE(v: Py) { ensure_init(); G_SAVINGS_AFTER_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMETHING_WEIGHED() -> Py { ensure_init(); G_SOMETHING_WEIGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_WEIGHED(v: Py) { ensure_init(); G_SOMETHING_WEIGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_DAYS() -> Py { ensure_init(); G_SPAN_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_DAYS(v: Py) { ensure_init(); G_SPAN_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_FRAME() -> Py { ensure_init(); G_SPAN_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_FRAME(v: Py) { ensure_init(); G_SPAN_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_MONTHS() -> Py { ensure_init(); G_SPAN_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_MONTHS(v: Py) { ensure_init(); G_SPAN_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_WEEKS() -> Py { ensure_init(); G_SPAN_WEEKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_WEEKS(v: Py) { ensure_init(); G_SPAN_WEEKS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEPS() -> Py { ensure_init(); G_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEPS(v: Py) { ensure_init(); G_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT() -> Py { ensure_init(); G_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT(v: Py) { ensure_init(); G_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM_IN_A_WORD() -> Py { ensure_init(); G_SUM_IN_A_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM_IN_A_WORD(v: Py) { ensure_init(); G_SUM_IN_A_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKE() -> Py { ensure_init(); G_TAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKE(v: Py) { ensure_init(); G_TAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_TENS() -> Py { ensure_init(); G_TENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TENS(v: Py) { ensure_init(); G_TENS.with(|c| *c.borrow_mut() = v); }
pub fn g_TEN_TIMES() -> Py { ensure_init(); G_TEN_TIMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEN_TIMES(v: Py) { ensure_init(); G_TEN_TIMES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_THING() -> Py { ensure_init(); G_THEIR_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_THING(v: Py) { ensure_init(); G_THEIR_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_HAVE_IT() -> Py { ensure_init(); G_THEY_HAVE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_HAVE_IT(v: Py) { ensure_init(); G_THEY_HAVE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_HAVE_THE_TIME() -> Py { ensure_init(); G_THEY_HAVE_THE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_HAVE_THE_TIME(v: Py) { ensure_init(); G_THEY_HAVE_THE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HOUSEHOLD() -> Py { ensure_init(); G_THE_HOUSEHOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HOUSEHOLD(v: Py) { ensure_init(); G_THE_HOUSEHOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_A_DAY() -> Py { ensure_init(); G_TIME_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_A_DAY(v: Py) { ensure_init(); G_TIME_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_BUDGET() -> Py { ensure_init(); G_TIME_BUDGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_BUDGET(v: Py) { ensure_init(); G_TIME_BUDGET.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_LONG() -> Py { ensure_init(); G_TIME_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_LONG(v: Py) { ensure_init(); G_TIME_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_TRIP() -> Py { ensure_init(); G_TIME_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_TRIP(v: Py) { ensure_init(); G_TIME_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WEEK() -> Py { ensure_init(); G_TIME_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WEEK(v: Py) { ensure_init(); G_TIME_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP() -> Py { ensure_init(); G_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP(v: Py) { ensure_init(); G_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS() -> Py { ensure_init(); G_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); G_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_IS() -> Py { ensure_init(); G_UNIT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_IS(v: Py) { ensure_init(); G_UNIT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_SAID() -> Py { ensure_init(); G_UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_SAID(v: Py) { ensure_init(); G_UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_GOING_ON() -> Py { ensure_init(); G_WEIGHT_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_GOING_ON(v: Py) { ensure_init(); G_WEIGHT_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_LOSS() -> Py { ensure_init(); G_WEIGHT_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_LOSS(v: Py) { ensure_init(); G_WEIGHT_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_OF_A_THING() -> Py { ensure_init(); G_WEIGHT_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_OF_A_THING(v: Py) { ensure_init(); G_WEIGHT_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_ON_THE_TABLE() -> Py { ensure_init(); G_WEIGHT_ON_THE_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_ON_THE_TABLE(v: Py) { ensure_init(); G_WEIGHT_ON_THE_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_PLAIN() -> Py { ensure_init(); G_WEIGHT_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_PLAIN(v: Py) { ensure_init(); G_WEIGHT_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_THEIR_OWN() -> Py { ensure_init(); G_WEIGHT_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_THEIR_OWN(v: Py) { ensure_init(); G_WEIGHT_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reply to a line that is nothing but a number.\n\nPeople plan out loud in numbers. \"20 mins\", \"45 a month\", \"its about 5 miles\", \"40 minutes each\nway\", \"20 pounds a week\". The number is the constraint the whole plan turns on, and it is usually\nthe answer to the question the bot asked one turn earlier: \"what can you spend?\", \"how long have\nyou got?\", \"how far is it?\".\n\nWhat the bot says to such a line today is nothing. It is not a question, so no module answers it;\nit carries no feeling, so no pool fits; the memory store can file it (\"Got it - you have 40\npounds left for food this week\") or it lands in one of the stock lines, and both are the failure\nSTYLE.md names. Measured on main, one process a line:\n\n    its 45 a month              -> I'm listening - say more if it helps.\n    20 mins                     -> Got it. Is there anything you'd like a hand with?\n    its about 5 miles           -> I'm listening - say more if it helps.\n    forty minutes each way      -> Thanks for telling me - I'm here if you want to dig in.\n    we have 40 pounds left ...  -> Got it - you have 40 pounds left for food this week.\n\nA number on its own is half of a sum, and the sum is the reply. Three sums, all of them\narithmetic the bot can do without knowing anything:\n\n- a rate across time: \"45 a month\" is 540 a year, \"20 a week\" is 87 a month. The yearly figure is\n  what a person actually weighs a thing against, and nobody does that sum in their head.\n- a habit across time: \"40 minutes each way\" is 1 hour 20 a day and 6 hours 40 a week. The cost of\n  a commute is invisible until it is added up.\n- a distance as a time: 5 miles is about 100 minutes walking and 12 in the car, and which of those\n  matters depends on whether they have the car.\n\nWhere the number has no second number to work with (\"20 mins\" of what?), the reply is the same\narithmetic one step cheaper: the shortest version rather than the best one, one thing done\nproperly and not two done badly. That is what a friend says when they hear \"I have twenty\nminutes\".\n\nThe read is deliberately narrow. The number has to be the whole line, wrapped in the lead-ins\npeople wrap numbers in (\"its\", \"about\", \"only\", \"just\", \"i have\", \"and\") and followed only by the\nwords that say what the number is about (\"a month\", \"each way\", \"on the bus\", \"left\", \"now\"). A\nline with a subject of its own - \"i have 2000 saved and no idea what to do with it\" - is not a\nbare number and keeps the reply it has today. A number that is not time, distance or money is\nleft alone: \"95 kilos\" is a body, and the health answers are not this file's to guess at.\n");
    s_CURRENCY(py_tuple(vec![py_str("£"), py_str("$"), py_str("€")]));
    s_LEAD(call_builtin("frozenset", &[py_tuple(vec![py_str("its"), py_str("it's"), py_str("thats"), py_str("that's"), py_str("the"), py_str("about"), py_str("around"), py_str("roughly"), py_str("probably"), py_str("maybe"), py_str("only"), py_str("just"), py_str("like"), py_str("so"), py_str("and"), py_str("but"), py_str("plus"), py_str("well"), py_str("currently"), py_str("still"), py_str("now"), py_str("i"), py_str("have"), py_str("we"), py_str("got"), py_str("ive"), py_str("i've"), py_str("we've"), py_str("weve"), py_str("there"), py_str("is"), py_str("there's"), py_str("theres"), py_str("it"), py_str("of"), py_str("us"), py_str("them"), py_str("both"), py_str("looks"), py_str("works"), py_str("out"), py_str("at"), py_str("keeps"), py_str("costs"), py_str("cost"), py_str("pays"), py_str("paid"), py_str("uses"), py_str("takes"), py_str("spend"), py_str("spent"), py_str("save"), py_str("saved"), py_str("earn"), py_str("earns"), py_str("makes"), py_str("make"), py_str("means"), py_str("that"), py_str("this"), py_str("they"), py_str("he"), py_str("she"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("was"), py_str("were"), py_str("will"), py_str("would"), py_str("be"), py_str("a"), py_str("an"), py_str("half"), py_str("for"), py_str("on"), py_str("over"), py_str("under"), py_str("being"), py_str("weigh"), py_str("weighs"), py_str("weighed"), py_str("weighing"), py_str("bring"), py_str("brings"), py_str("bringing"), py_str("brought"), py_str("take"), py_str("takes"), py_str("taking"), py_str("took"), py_str("home"), py_str("clears"), py_str("net"), py_str("nett"), py_str("put"), py_str("puts"), py_str("putting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNITS(py_dict(vec![(py_str("minute"), py_tuple(vec![py_str("time"), Py::Float(1.0)])), (py_str("minutes"), py_tuple(vec![py_str("time"), Py::Float(1.0)])), (py_str("min"), py_tuple(vec![py_str("time"), Py::Float(1.0)])), (py_str("mins"), py_tuple(vec![py_str("time"), Py::Float(1.0)])), (py_str("hour"), py_tuple(vec![py_str("time"), Py::Float(60.0)])), (py_str("hours"), py_tuple(vec![py_str("time"), Py::Float(60.0)])), (py_str("hr"), py_tuple(vec![py_str("time"), Py::Float(60.0)])), (py_str("hrs"), py_tuple(vec![py_str("time"), Py::Float(60.0)])), (py_str("day"), py_tuple(vec![py_str("span"), Py::Float(1.0)])), (py_str("days"), py_tuple(vec![py_str("span"), Py::Float(1.0)])), (py_str("week"), py_tuple(vec![py_str("span"), Py::Float(7.0)])), (py_str("weeks"), py_tuple(vec![py_str("span"), Py::Float(7.0)])), (py_str("month"), py_tuple(vec![py_str("span"), Py::Float(30.0)])), (py_str("months"), py_tuple(vec![py_str("span"), Py::Float(30.0)])), (py_str("year"), py_tuple(vec![py_str("span"), Py::Float(365.0)])), (py_str("years"), py_tuple(vec![py_str("span"), Py::Float(365.0)])), (py_str("mile"), py_tuple(vec![py_str("distance"), Py::Float(1.0)])), (py_str("miles"), py_tuple(vec![py_str("distance"), Py::Float(1.0)])), (py_str("km"), py_tuple(vec![py_str("distance"), Py::Float(0.6214)])), (py_str("kilometre"), py_tuple(vec![py_str("distance"), Py::Float(0.6214)])), (py_str("kilometres"), py_tuple(vec![py_str("distance"), Py::Float(0.6214)])), (py_str("kilometers"), py_tuple(vec![py_str("distance"), Py::Float(0.6214)])), (py_str("pound"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("pounds"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("quid"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("euro"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("euros"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("dollar"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("dollars"), py_tuple(vec![py_str("money"), Py::Float(1.0)])), (py_str("grand"), py_tuple(vec![py_str("money"), Py::Float(1000.0)])), (py_str("k"), py_tuple(vec![py_str("money"), Py::Float(1000.0)])), (py_str("metre"), py_tuple(vec![py_str("size"), Py::Float(1.0)])), (py_str("metres"), py_tuple(vec![py_str("size"), Py::Float(1.0)])), (py_str("meter"), py_tuple(vec![py_str("size"), Py::Float(1.0)])), (py_str("meters"), py_tuple(vec![py_str("size"), Py::Float(1.0)])), (py_str("foot"), py_tuple(vec![py_str("size"), Py::Float(0.3048)])), (py_str("feet"), py_tuple(vec![py_str("size"), Py::Float(0.3048)])), (py_str("inch"), py_tuple(vec![py_str("size"), Py::Float(0.0254)])), (py_str("inches"), py_tuple(vec![py_str("size"), Py::Float(0.0254)])), (py_str("cm"), py_tuple(vec![py_str("size"), Py::Float(0.01)])), (py_str("stone"), py_tuple(vec![py_str("weight"), Py::Float(6.35029)])), (py_str("stones"), py_tuple(vec![py_str("weight"), Py::Float(6.35029)])), (py_str("kilo"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("kilos"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("kilogram"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("kilograms"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("kg"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("kgs"), py_tuple(vec![py_str("weight"), Py::Float(1.0)])), (py_str("lb"), py_tuple(vec![py_str("weight"), Py::Float(0.4536)])), (py_str("lbs"), py_tuple(vec![py_str("weight"), Py::Float(0.4536)]))]));
    s_NUMBERS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("hundred"), Py::Int(100)), (py_str("thousand"), Py::Int(1000)), (py_str("dozen"), Py::Int(12)), (py_str("couple"), Py::Int(2)), (py_str("few"), Py::Int(3)), (py_str("half"), Py::Float(0.5)), (py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("several"), Py::Int(4))]));
    s_TENS(py_dict(vec![(py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90))]));
    s_PERIODS(py_tuple(vec![py_tuple(vec![py_str("a month"), py_str("month")]), py_tuple(vec![py_str("per month"), py_str("month")]), py_tuple(vec![py_str("each month"), py_str("month")]), py_tuple(vec![py_str("monthly"), py_str("month")]), py_tuple(vec![py_str("a week"), py_str("week")]), py_tuple(vec![py_str("per week"), py_str("week")]), py_tuple(vec![py_str("each week"), py_str("week")]), py_tuple(vec![py_str("weekly"), py_str("week")]), py_tuple(vec![py_str("a year"), py_str("year")]), py_tuple(vec![py_str("per year"), py_str("year")]), py_tuple(vec![py_str("each year"), py_str("year")]), py_tuple(vec![py_str("yearly"), py_str("year")]), py_tuple(vec![py_str("a day"), py_str("day")]), py_tuple(vec![py_str("per day"), py_str("day")]), py_tuple(vec![py_str("each day"), py_str("day")]), py_tuple(vec![py_str("daily"), py_str("day")]), py_tuple(vec![py_str("a fortnight"), py_str("fortnight")]), py_tuple(vec![py_str("per fortnight"), py_str("fortnight")])]));
    s_POINTING_BACK(call_builtin("frozenset", &[py_tuple(vec![py_str("for"), py_str("on"), py_str("about"), py_str("of"), py_str("at"), py_str("with"), py_str("in"), py_str("into"), py_str("to"), py_str("towards"), py_str("toward"), py_str("from")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("those"), py_str("these"), py_str("him"), py_str("her"), py_str("everything"), py_str("anything"), py_str("something")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEDGES(call_builtin("frozenset", &[py_tuple(vec![py_str("think"), py_str("thought"), py_str("reckon"), py_str("guess"), py_str("suppose"), py_str("believe"), py_str("hope"), py_str("say"), py_str("imagine"), py_str("assume")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIP(stdlib_call("re", "compile", &[py_str("\\b(each\\s+way|each-way|one\\s+way|round\\s+trip|there\\s+and\\s+back|on\\s+the\\s+(?:bus|train|tube|tram|bike|bicycle)|in\\s+the\\s+car|by\\s+(?:car|bus|train|bike)|commut\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PER_DAY(stdlib_call("re", "compile", &[py_str("\\b(a\\s+day|per\\s+day|each\\s+day|every\\s+day|a\\s+weekday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str("each"), py_str("way"), py_str("one"), py_str("round"), py_str("trip"), py_str("there"), py_str("and"), py_str("back"), py_str("a"), py_str("an"), py_str("per"), py_str("every"), py_str("day"), py_str("days"), py_str("week"), py_str("weeks"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("fortnight"), py_str("hour"), py_str("hours"), py_str("minute"), py_str("minutes"), py_str("weekday"), py_str("left"), py_str("now"), py_str("though"), py_str("today"), py_str("already"), py_str("more"), py_str("total"), py_str("altogether"), py_str("on"), py_str("in"), py_str("the"), py_str("bus"), py_str("train"), py_str("tube"), py_str("tram"), py_str("bike"), py_str("bicycle"), py_str("car"), py_str("to"), py_str("work"), py_str("at"), py_str("moment"), py_str("right"), py_str("so"), py_str("far"), py_str("max"), py_str("maximum"), py_str("ish"), py_str("or"), py_str("about"), py_str("around"), py_str("extra"), py_str("over"), py_str("under"), py_str("spare"), py_str("saved"), py_str("saving"), py_str("savings"), py_str("coming"), py_str("out"), py_str("roughly"), py_str("odd"), py_str("monthly"), py_str("weekly"), py_str("yearly"), py_str("daily"), py_str("currently"), py_str("between"), py_str("probably"), py_str("maybe"), py_str("only"), py_str("just"), py_str("like"), py_str("well"), py_str("still"), py_str(":"), py_str("is"), py_str("all"), py_str("what"), py_str("have"), py_str("has"), py_str("to"), py_str("play"), py_str("with"), py_str("spend"), py_str("by"), py_str("of"), py_str("it"), py_str("that"), py_str("i"), py_str("we"), py_str("they"), py_str("you"), py_str("off"), py_str("away"), py_str("wide"), py_str("long"), py_str("high"), py_str("tall"), py_str("deep"), py_str("square"), py_str("weeknight"), py_str("weeknights"), py_str("weekday"), py_str("weekdays"), py_str("morning"), py_str("mornings"), py_str("evening"), py_str("evenings"), py_str("night"), py_str("nights"), py_str("shift"), py_str("shifts"), py_str("tops"), py_str("plus"), py_str("minus"), py_str("if"), py_str("when")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DOUBT_ON_THE_FIGURE(stdlib_call("re", "compile", &[py_str("^(?:[a-z',\\u2019]+\\s+){0,3}(?:is|isnt|isn'?t|was|wasnt|wasn'?t|too|thats|that'?s)\\b\\s*(?:that|this|it|they)?\\s*(?:a\\s+lot|too\\s+much|too\\s+dear|expensive|cheap|dear|pricey|steep|normal|about\\s+right|fair|reasonable|good\\s+value|worth\\s+it|okay?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAVINGS_AFTER_A_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:savings?|save|saves|saving|saved)\\b|\\b(?:put|puts|putting|set|setting|pay|paying)\\b[^.!?]{0,14}\\b(?:away|aside|by|in|into)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAVINGS(py_tuple(vec![py_str("{q} {period} is {year} a year. That is the figure to plan with, not the weekly one."), py_str("{q} {period} is {year} a year. Is it steady, or does it move with the month?"), py_str("{q} {period} is {year} a year - the yearly figure is the one that adds up to something.")]));
    s_MONEY_MONTH(py_tuple(vec![py_str("{q} a month is {year} a year. Put as a year it is easier to weigh against a one-off cost."), py_str("{q} a month is {year} a year - the number that matters if it runs for a while."), py_str("{q} a month is {year} a year. Would you pay {year} up front for the same thing?"), py_str("{q} a month is {year} a year, which is the figure to hold against the yearly price.")]));
    s_MONEY_WEEK(py_tuple(vec![py_str("{q} a week is {month} a month, about {year} a year. Weekly adds up faster than it feels."), py_str("{q} a week is {month} a month and {year} a year. Is that steady or a bad patch?"), py_str("{q} a week - that is {year} a year, or {month} a month. Which one is it really?")]));
    s_MONEY_YEAR(py_tuple(vec![py_str("{q} a year is {month} a month, about {week} a week. That is how it lands."), py_str("Put as a month, {q} a year is {month} - and a month is the number people plan on."), py_str("{q} a year is {month} a month. Is the money coming in monthly or in lumps?")]));
    s_MONEY_DAY(py_tuple(vec![py_str("{q} a day is {month} a month. It is the small daily ones that get away from people."), py_str("{q} a day is {year} a year. Would you notice it going out once a year?")]));
    s_INCOME_MONTH(py_tuple(vec![py_str("{q} a month is {year} a year. Is that what you were expecting, or are you working out where it goes?"), py_str("{q} a month is {year} a year before anything comes off it. What are you weighing it against?"), py_str("{q} a month comes to {year} a year, which is the figure to compare one job with another.")]));
    s_INCOME_WEEK(py_tuple(vec![py_str("{q} a week is {month} a month, about {year} a year. Is the work steady or up and down?"), py_str("{q} a week is {year} a year - that is what it comes to over the good and the quiet weeks.")]));
    s_INCOME_YEAR(py_tuple(vec![py_str("{q} a year is {month} a month, about {week} a week. Is that what it was before, or a change?"), py_str("{q} a year is {month} a month. Put as a month it is easier to see where it goes.")]));
    s_INCOME_PLAIN(py_tuple(vec![py_str("{q} - is that what you take home, or before anything comes off it?"), py_str("{q}. Does that have to cover everything, or is some of it spoken for already?")]));
    s_INCOME(stdlib_call("re", "compile", &[py_str("\\b(?:earn(?:s|ed|ing)?|salary|salaries|wages?|income|pension|take[- ]home|get(?:s|ting)? paid|paid (?:me|us)|pays? me|my pay|a pay ?rise|pay ?rise|bring(?:s|ing)? (?:it )?home|brought (?:it )?home)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_PLAIN(py_tuple(vec![py_str("{q} - is that the whole budget or what is left of a bigger one?"), py_str("{q} goes further as a plan than as a number. What does it need to cover?"), py_str("{q}. What does it have to stretch to?")]));
    s_TIME_BUDGET(py_tuple(vec![py_str("{q} is enough for one thing done properly and not two done badly."), py_str("In {q} take the shortest version of it rather than the best one. What is it for?"), py_str("{q} goes faster than it looks, so decide before you start rather than halfway through."), py_str("{q} is tight - one thing only. What are you fitting into it?"), py_str("{q}: get everything out before the clock starts and the time is all work, not looking.")]));
    s_TIME_TRIP(py_tuple(vec![py_str("{q} each way is {day} a day, {week} a week if it is five days. Worth a number first."), py_str("{q} twice a day is {day}, and five days of it is {week}. Do you get to sit down for it?"), py_str("That is {day} of your day, {week} a week. It is the part of the job nobody pays you for.")]));
    s_TIME_A_DAY(py_tuple(vec![py_str("{q} a day is {week} a week - small enough to keep up, which is the point of it."), py_str("{q} a day adds up to {week} a week. Enough to see a change if it is the right thing."), py_str("{q} a day, so {week} a week. The trick is that it has to fit a day you actually have.")]));
    s_TIME_LONG(py_tuple(vec![py_str("{q} is a real chunk of a week. What has to fit inside it?"), py_str("{q} - is that what you have, or what you think it needs?"), py_str("{q} is enough to get somewhere with it. What is the first thing in there?")]));
    s_TIME_WEEK(py_tuple(vec![py_str("{q} a week is about {day} a day. That is what it has to fit into."), py_str("{q} a week comes to {day} a day - doable, if nothing else takes that slot.")]));
    s_AREA(py_tuple(vec![py_str("{q} by {b} is {area} square metres - have the number before you buy anything."), py_str("That is {area} square metres. What is going on it?"), py_str("{q} by {b} is {area} square metres. A tin or a roll never covers what the label claims.")]));
    s_DISTANCE_NEAR(py_tuple(vec![py_str("{q} is about {walk} minutes walking, or {bike} on a bike."), py_str("{q} - {walk} minutes on foot, and no time at all on a bike ({bike} minutes)."), py_str("{q} is close enough to walk in {walk} minutes, or ride in {bike}.")]));
    s_DISTANCE_MID(py_tuple(vec![py_str("{q} is about {bike} minutes on a bike, or {drive} in the car."), py_str("{q} - a ride of about {bike} minutes, or {drive} driving."), py_str("{q} is close enough to cycle ({bike} minutes) and quick in the car ({drive}).")]));
    s_DISTANCE_FAR(py_tuple(vec![py_str("{q} is about {drive} of driving."), py_str("{q} is {drive} behind the wheel."), py_str("{q} - {drive} in the car, and a lot longer any other way.")]));
    s_DISTANCE_DAY_OUT(py_tuple(vec![py_str("{q} is about {drive} of driving - a day out rather than a hop."), py_str("{q} is {drive} behind the wheel; a day out, with a stop on the way.")]));
    s_DISTANCE_FAR_TRIP(py_tuple(vec![py_str("{q} each way is {drive} in the car. Doable in a day if you leave early."), py_str("{q} each way - {drive} behind the wheel, and that is before you turn round.")]));
    s_SPAN_DAYS(py_tuple(vec![py_str("{q} is long enough to know whether it is working and short enough to change course."), py_str("{q} - what has it done in that time? Better, worse or the same?"), py_str("{q} is a while. Is it going the way you want?")]));
    s_SPAN_WEEKS(py_tuple(vec![py_str("{q} is usually where a thing either settles or needs a proper look. Which is it?"), py_str("{q} - what would have to change for you to be happy with it?"), py_str("{q} is enough time for something to show. What has shown?")]));
    s_SPAN_MONTHS(py_tuple(vec![py_str("{q} is long enough that the small bits add up. What is the biggest part of it?"), py_str("{q} - is that how long it has been, or how long it needs to last?")]));
    s_WEIGHT_LOSS(py_tuple(vec![py_str("{q} is {other} - at a pound or two a week that is {slow} to {fast} weeks, and the slow way is the one that stays off."), py_str("{q} - {other}. Think in months rather than weeks: a pound or two a week is {slow} to {fast} weeks, and that is the pace that holds."), py_str("{q} is {other}, so it is a few months of small changes rather than a push. What would you change first?")]));
    s_WEIGHT_PLAIN(py_tuple(vec![py_str("{q} is {other}. Is that you, or something you are carrying?"), py_str("{q} - {other}. What is it that weighs that?")]));
    s_WEIGHT_OF_A_THING(py_tuple(vec![py_str("{q} is {other}."), py_str("{q} - {other}. Is that what you expected?")]));
    s_WEIGHT_THEIR_OWN(py_tuple(vec![py_str("{q} is {other}. Is there a number you are aiming for?"), py_str("{q} - {other}. What would you like to do about it?")]));
    s_WEIGHT_ON_THE_TABLE(stdlib_call("re", "compile", &[py_str("\\b(?:weight|weigh|weighing|weighed|lose|losing|lost|slim|slimming|diet|diets|dieting|stone|stones|kilos?|kilograms?|kg|lbs?|bmi|calories|waist|dress size)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WEIGHT_GOING_ON(stdlib_call("re", "compile", &[py_str("\\bput (?:on|it on|a bit on|a few on|a little on|a couple on|some on|another on|weight on|the weight on|a stone on|half a stone on)\\b|\\b(?:piling|piled|piling it) on\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECT(stdlib_call("re", "compile", &[py_str("\\A(?:the|my|our|their|his|her|its|this|that|it|there)\\s+\\w+\\s+(?:is|are|was|were|would be|will be)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_NOUN(py_dict(vec![(py_str("tenner"), Py::Int(10)), (py_str("fiver"), Py::Int(5))]));
    s_SUM_IN_A_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:a|an)\\s+(tenner|fiver)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_TABLE_DISTANCE(stdlib_call("re", "compile", &[py_str("\\b(?:far|further|distance|miles?|km|kilometres?|kilometers?|kms|run|runs|running|ran|walk|walked|walking|walks|cycle|cycled|cycling|drive|drove|driving|commute|commuting|hike|hiked|hiking|swim|swam|swimming|journey|trip)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_TABLE_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:budget|spend|spending|spent|cost|costs|afford|save|saving|savings|price|prices|buy|buying|bought|paid|pay|rent|bill|bills|deposit|money|cash|quid|salary|wage|wages|income|pension|£|\\$|€)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BUDGET_THEY_SET(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:budget|limit|cap|ceiling|spending money)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SPAN_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:been|since|ago|started|start|had|having|took|taking|on it|so far|now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION_ABOUT_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|far|much|many|big|deep|hot)\\b|\\bhow soon\\b|\\bwhen will\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BUDGET_QUESTION(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|much time|often)\\s+(?:should|shall|do|can|could|might|would) (?:i|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BOUND(stdlib_call("re", "compile", &[py_str("\\b(?:only|just|barely|no more than|at most|less than)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BOUNDED_BY(call_builtin("frozenset", &[py_tuple(vec![py_str("before"), py_str("until"), py_str("till"), py_str("til"), py_str("to"), py_str("for"), py_str("free"), py_str("spare"), py_str("off"), py_str("left"), py_str("after"), py_str("between"), py_str("while"), py_str("then"), py_str("so"), py_str("if"), py_str("and"), py_str("but"), py_str("plus"), py_str("today"), py_str("tonight"), py_str("now")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_HAVE_THE_TIME(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:i|we|ive|i've|we've|weve)\\s+(?:only\\s+|just\\s+)?(?:have|had|got)\\b|(?:only\\s+|just\\s+)?got\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REMAINDER(stdlib_call("re", "compile", &[py_str("\\b(?:left|to go|remaining|to run)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_HAVE_IT(stdlib_call("re", "compile", &[py_str("\\b(?:have|has|had|got|left|spare|need|needs|gives? me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIMIT_ON_A_LENGTH(py_tuple(vec![py_str("{q} is short. What is on the other side of it?"), py_str("{q} is not much time to sit through. What would make it easier?"), py_str("{q} is nothing in the scheme of a day. What are you least looking forward to?"), py_str("{q} is short enough that it will be over before it has properly started. What do you want out of it?")]));
    s_A_WHAT_IS_LEFT(py_tuple(vec![py_str("{q} left - short enough now that what goes in it matters. What has to happen in it?"), py_str("{q} left - that is the room there is. What is the one thing that has to get done?"), py_str("{q} left, so it is time to decide what matters. What would you put first?")]));
    s_A_CHANGE_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:lose|losing|lost|shed|shedding|drop|dropping|shift|shifting|put on|putting on|gained|gaining|gain|gained|lose off|off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHANGE_IS_THIS_SMALL(Py::Float(20.0));
    s_I_WEIGH(stdlib_call("re", "compile", &[py_str("\\bi (?:weigh|weighed|weighs)\\b|\\bive (?:weighed|put)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING_WEIGHED(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|it|they|we|you|the|my|our|his|her|their|this|that)\\b[^.?!]{0,24}\\bweighs?\\b|\\bweighs?\\b[^.?!]{0,24}\\b(?:the|my|our|his|her|their|this|that)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_ASKED_FOR(stdlib_call("re", "compile", &[py_str("\\bhow many\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WAY_OF_PAYING(stdlib_call("re", "compile", &[py_str("\\b(?:money|pounds?|quid|euros?|dollars?|bucks?|pence|cost|costs|spend|spent|spending|pay|paid|paying|price|priced|budget|rent|bill|bills|salary|wage|wages|income|save|saving|savings|afford|cheap|expensive|earn|earned|overdraft|loan|subscription)\\b|[\\u00a3$\\u20ac]")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+([a-z][a-z'-]{2,})")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nmonth months week weeks year years day days fortnight quarter hour hours minute minutes time\ntimes lot lots one ones thing things rest whole same other other's own money cash budget\nfirst last next same top bottom bill bills price prices cost costs total monthly weekly yearly\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_HOUSEHOLD(py_tuple(vec![py_tuple(vec![py_str("between"), py_str("us")]), py_tuple(vec![py_str("between"), py_str("the"), py_str("two"), py_str("of"), py_str("us")]), py_tuple(vec![py_str("for"), py_str("the"), py_str("two"), py_str("of"), py_str("us")]), py_tuple(vec![py_str("for"), py_str("both"), py_str("of"), py_str("us")]), py_tuple(vec![py_str("between"), py_str("the"), py_str("two"), py_str("of"), py_str("them")]), py_tuple(vec![py_str("together")]), py_tuple(vec![py_str("combined")]), py_tuple(vec![py_str("in"), py_str("total")]), py_tuple(vec![py_str("altogether")]), py_tuple(vec![py_str("in"), py_str("all")])]));
    s_TAKE(py_tuple(vec![py_str("take"), py_str("takes"), py_str("taking"), py_str("took")]));
    s_A_STRETCH_THAT_IS_NOT_SHORT(Py::Int(120));
    s_A_RANGE_WITH_A_UNIT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?P<lo>\\d{1,4}(?:\\.\\d+)?|"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NUMBERS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s*(?:to|and|-|\\u2013|or)\\s*(?P<hi>\\d{1,4}(?:\\.\\d+)?|")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NUMBERS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s*(?P<unit>glasses?|litres?|liters?|hours?|minutes?|steps|portions?|servings?|sessions?|workouts?|units?)\\b(?:\\s*(?:a|per|each)\\s*(?P<period>day|week|month))?")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNIT_SAID(py_dict(vec![(py_str("glasses"), py_str("glasses")), (py_str("glass"), py_str("glasses")), (py_str("litres"), py_str("litres")), (py_str("litre"), py_str("litres")), (py_str("liters"), py_str("litres")), (py_str("liter"), py_str("litres")), (py_str("hours"), py_str("hours")), (py_str("hour"), py_str("hours")), (py_str("minutes"), py_str("minutes")), (py_str("minute"), py_str("minutes")), (py_str("steps"), py_str("steps")), (py_str("step"), py_str("steps")), (py_str("portions"), py_str("portions")), (py_str("portion"), py_str("portions")), (py_str("servings"), py_str("portions")), (py_str("serving"), py_str("portions")), (py_str("sessions"), py_str("sessions")), (py_str("session"), py_str("sessions")), (py_str("workouts"), py_str("sessions")), (py_str("workout"), py_str("sessions")), (py_str("units"), py_str("units")), (py_str("unit"), py_str("units"))]));
    s_UNIT_IS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("glass|litre|liter"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("water")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("hour"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("sleep")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("minute|step|session|workout"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("move")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("portion|serving"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("portion")])]));
    s_A_SINGLE_RULE(stdlib_call("re", "compile", &[py_str("\\b(?:about|around|roughly|at least|aim for|aim to|up to|no more than|no less than)\\s+(?P<num>\\d{1,4}(?:\\.\\d+)?)\\s*(?P<unit>glasses?|litres?|liters?|hours?|minutes?|steps|portions?|servings?|sessions?|workouts?|units?)\\b(?:\\s*(?:a|per|each)\\s*(?P<period>day|week|month))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_ALONE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:[a-z']+\\s+){0,5}?(?P<num>\\d{1,3}(?:\\.\\d+)?|"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NUMBERS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s*(?P<what>[a-z']*)\\s*\\.?$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:i (?:drink|get|sleep|do|walk|run|eat|have|take|manage|average|only get|usually get)|about|around|roughly|maybe|only|just|probably|usually|average|like)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AGE(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|im|i am)\\b|\\byears? old\\b|\\bage\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_UNIT_ITSELF(stdlib_call("re", "compile", &[py_str("[£$€]|\\b(?:k|kg|g|mg|ml|cm|mm|miles?|km|inches|feet|foot)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_BELOW(py_tuple(vec![py_str("{figure} {unit} is short of the {range} the answer named. {step}"), py_str("{figure} {unit} - that is under the {range} that answer gave, so the gap is the bit worth closing. {step}"), py_str("You are at {figure} {unit} against {range}. {step}")]));
    s_FIGURE_INSIDE(py_tuple(vec![py_str("{figure} {unit} is inside the {range} the answer named - that is the part that matters, so keep it there rather than adding to it."), py_str("{figure} {unit} sits in the {range} the answer named. Steady is the whole of it at that level.")]));
    s_FIGURE_ABOVE(py_tuple(vec![py_str("{figure} {unit} is past the {range} the answer named. {step}"), py_str("You are already over the {range} the answer named at {figure} {unit}. {step}")]));
    s_STEPS(py_dict(vec![(py_str("water"), py_dict(vec![(py_str("below"), py_str("Put a glass by the kettle and have one with each meal, which is three before you count anything else.")), (py_str("above"), py_str("Past that, more is not better: pale straw is the check, and you are already there."))])), (py_str("sleep"), py_dict(vec![(py_str("below"), py_str("Half an hour earlier to bed, with the same wake-up time even after a bad night, is what closes it.")), (py_str("above"), py_str("Sleeping past it is not a gain - the steady wake-up time is what keeps the good nights coming."))])), (py_str("move"), py_dict(vec![(py_str("below"), py_str("Two ten-minute walks beat one long one, because they are easier to keep up.")), (py_str("above"), py_str("Past it, the gain is in keeping it going rather than adding more."))])), (py_str("portion"), py_dict(vec![(py_str("below"), py_str("One at breakfast and one at each main meal gets you to it without a plan.")), (py_str("above"), py_str("Past it, the thing worth holding is the mix rather than the count."))]))]));
    s_A_STEP_EITHER_WAY(py_dict(vec![(py_str("below"), py_str("Start at what you can keep up and add a little when it feels easy.")), (py_str("above"), py_str("Being past it is not what needs fixing; keeping it steady is."))]));
    s_PERIOD_NAMED(py_dict(vec![(py_str("day"), py_str("a day")), (py_str("week"), py_str("a week")), (py_str("month"), py_str("a month"))]));
    s_TEN_TIMES(Py::Int(10));
}
pub fn _number_at(args: &[Py]) -> Py {
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
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut first: Py = Py::None;
    let mut nxt: Py = Py::None;
    let _ = py_str("(value, how many words it took) for the number starting at `words[at]`, or (None, 0).\n\n    People write a number in more than one word (\"twenty five minutes\", \"three hundred a\n    year\"), so a tens word followed by a units word is added up and a number followed by\n    \"hundred\" is multiplied, the way it is said.\n    ");
    first = _amount(&[geti(&words.clone(), &at.clone())]);
    if py_truthy(&Py::Bool(py_is(&first.clone(), &Py::None))) {
    return py_tuple(vec![Py::None, Py::Int(0)]);
    }
    if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    nxt = mcall(&mcall(&geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1))), "lower", &[]), "replace", &[py_str("-"), py_str("")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&nxt.clone(), &py_str("hundred"))); if !py_truthy(&__b) { __b } else { op_lt(&first.clone(), &Py::Int(100)) } }) {
    return py_tuple(vec![op_mul(&first.clone(), &Py::Int(100)), Py::Int(2)]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TENS(), &mcall(&geti(&words.clone(), &at.clone()), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine")]), &nxt.clone())) } }) {
    return py_tuple(vec![op_add(&first.clone(), &geti(&g_NUMBERS(), &nxt.clone())), Py::Int(2)]);
    }
    }
    return py_tuple(vec![first.clone(), Py::Int(1)]);
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
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut base: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut ten: Py = Py::None;
    let _ = py_str("The number a word gives, or None: \"45\", \"£250\", \"twenty\", \"forty\", \"twentyfive\".");
    plain = mcall(&word.clone(), "replace", &[py_str(","), py_str("")]);
    let __iter1 = py_iter_or_raise(&g_CURRENCY());
    for __item in __iter1 {
    mark = __item.clone();
    plain = mcall(&plain.clone(), "replace", &[mark.clone(), py_str("")]);
    }
    plain = mcall(&plain.clone(), "replace", &[py_str("-"), py_str("")]);
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d+(?:\\.\\d+)?"), plain.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("float", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_contains(&g_NUMBERS(), &plain.clone()))) {
    return call_builtin("float", &[geti(&g_NUMBERS(), &plain.clone())]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_TENS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    ten = __unpk3[0].clone();
    base = __unpk3[1].clone();
    if py_truthy(&{ let __b = mcall(&plain.clone(), "startswith", &[ten.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NUMBERS(), &slice_or_raise(&plain.clone(), &call_builtin("len", &[ten.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } }) {
    return call_builtin("float", &[op_add(&base.clone(), &geti(&g_NUMBERS(), &slice_or_raise(&plain.clone(), &call_builtin("len", &[ten.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::None;
    Py::None
}
pub fn _fmt(args: &[Py]) -> Py {
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
    let _ = py_str("A number as a person writes it: whole numbers whole, halves with one decimal place.");
    if py_truthy(&mcall(&call_builtin("float", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "is_integer", &[])) {
    return mcall(&py_str("{:,}"), "format", &[call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return mcall(&mcall(&mcall(&py_str("{:.1f}"), "format", &[value.clone()]), "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    Py::None
}
pub fn _minutes(args: &[Py]) -> Py {
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
    let mut hours: Py = Py::None;
    let mut minutes: Py = Py::None;
    let _ = py_str("A length in minutes said the way people say it: \"1 hour 20\", \"6 hours 40\".");
    total = op_mul(&call_builtin("int", &[call_builtin("round", &[op_truediv(&total.clone(), &Py::Float(5.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5));
    if py_truthy(&op_lt(&total.clone(), &Py::Int(60))) {
    return op_modulo(&py_str("%d minutes"), &total.clone());
    }
    let __unpk1 = py_unpack(&call_builtin("divmod", &[total.clone(), Py::Int(60)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    hours = __unpk1[0].clone();
    minutes = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&minutes.clone()))) {
    return op_modulo(&py_str("%d hour%s"), &py_tuple(vec![hours.clone(), if py_truthy(&Py::Bool(py_eq(&hours.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }]));
    }
    return op_modulo(&py_str("%d hour%s %d"), &py_tuple(vec![hours.clone(), if py_truthy(&Py::Bool(py_eq(&hours.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }, minutes.clone()]));
    Py::None
}
pub fn _the_sum_inside_the_word(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("(\"a tenner\", \"... 10 ...\") when the line names its sum with a word, else (\"\", text).");
    found = mcall(&g_SUM_IN_A_WORD(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![py_str(""), text.clone()]);
    }
    said = op_modulo(&py_str("a %s"), &mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]));
    return py_tuple(vec![said.clone(), op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &op_modulo(&py_str("%d"), &geti(&g_MONEY_NOUN(), &mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[])))), &slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None))]);
    Py::None
}
pub fn _the_money_words(args: &[Py]) -> Py {
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
    return call_builtin("bool", &[mcall(&g_ON_THE_TABLE_MONEY(), "search", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_a_distance(args: &[Py]) -> Py {
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
    let mut elsewhere: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the thing just said, or the last turn or two, is a distance and not a sum.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    elsewhere = py_list(vec![said.clone()]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(2)), &Py::None, &Py::None) } else { py_list(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    let __aug2 = op_add(&elsewhere.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("bot")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() });
    elsewhere = __aug2;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&elsewhere.clone()) { let mut text: Py = __item.clone(); __out.push(_the_money_words(&[text.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&elsewhere.clone()) { let mut text: Py = __item.clone(); __out.push(mcall(&g_ON_THE_TABLE_DISTANCE(), "search", &[mcall(&text.clone(), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _read(args: &[Py]) -> Py {
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
    let mut a_budget: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut at: Py = Py::None;
    let mut body: Py = Py::None;
    let mut body_tail: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut family: Py = Py::None;
    let mut half_more: Py = Py::None;
    let mut head: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut period: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sum_word: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut took: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(value, family, unit, period, phrase, tail) for a bare number line, or six Nones.\n\n    `phrase` is the number as the person said it (\"45\", \"£250\", \"half an hour\") so the reply can\n    quote it back, and `tail` is what the words after it said the number is about.\n    ");
    text = mcall(&mcall(&clean.clone(), "strip", &[]), "strip", &[py_str(".!,")]);
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=\\d)(kilometres|kilometers|kilograms|kilogram|miles|mile|stones|kilos|kilo|stone|kms|km|mi|kgs|kg|lbs|lb)\\b")); __args.push(py_str(" \\1")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    named = call_builtin("bool", &[mcall(&g_SUBJECT(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    a_budget = call_builtin("bool", &[mcall(&g_A_BUDGET_THEY_SET(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SUBJECT(), "sub", args))(&__args) };
    let __unpk1 = py_unpack(&_the_sum_inside_the_word(&[text.clone()]), 2, None);
    sum_word = __unpk1[0].clone();
    text = __unpk1[1].clone();
    words = mcall(&text.clone(), "split", &[]);
    at = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&_amount(&[geti(&words.clone(), &at.clone())]), &Py::None)) } }) {
    if py_truthy(&Py::Bool(!py_contains(&g_LEAD(), &mcall(&geti(&words.clone(), &at.clone()), "lower", &[])))) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    let __aug2 = op_add(&at.clone(), &Py::Int(1));
    at = __aug2;
    }
    if py_truthy(&op_ge(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    head = geti(&words.clone(), &at.clone());
    let __unpk3 = py_unpack(&_number_at(&[words.clone(), at.clone()]), 2, None);
    amount = __unpk3[0].clone();
    took = __unpk3[1].clone();
    body = slice_or_raise(&words.clone(), &op_add(&at.clone(), &took.clone()), &Py::None, &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&head.clone(), "lower", &[]), &py_str("half"))); if !py_truthy(&__b) { __b } else { { let __b = slice_or_raise(&body.clone(), &Py::None, &Py::Int(1), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[]))) } } } }) {
    body = slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    unit = if py_truthy(&{ let __b = body.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UNITS(), &mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[]))) } }) { mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[]) } else { py_str("") };
    if py_truthy(&unit.clone()) {
    body = slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    half_more = Py::Bool(py_eq(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&body.clone(), &Py::None, &Py::Int(3), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() }, &py_list(vec![py_str("and"), py_str("a"), py_str("half")])));
    if py_truthy(&half_more.clone()) {
    let __aug4 = op_mul(&amount.clone(), &Py::Float(1.5));
    amount = __aug4;
    body = slice_or_raise(&body.clone(), &Py::Int(3), &Py::None, &Py::None);
    }
    body = _without_a_remark(&[body.clone()]);
    body = _without_the_household(&[body.clone()]);
    tail = mcall(&mcall(&py_str(" "), "join", &[body.clone()]), "lower", &[]);
    period = Py::None;
    let __iter5 = py_iter_or_raise(&g_PERIODS());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    said = __unpk6[0].clone();
    name = __unpk6[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&tail.clone(), &said.clone())); if py_truthy(&__b) { __b } else { mcall(&tail.clone(), "startswith", &[said.clone()]) } }) {
    period = name.clone();
    body = slice_or_raise(&body.clone(), &call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    if py_truthy(&{ let __b = period.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DOUBT_ON_THE_FIGURE(), "match", &[mcall(&mcall(&py_str(" "), "join", &[body.clone()]), "strip", &[py_str(" .,!?")])]) } }) {
    body = py_list(vec![]);
    }
    let __iter7 = py_iter_or_raise(&body.clone());
    for __item in __iter7 {
    word = __item.clone();
    plain = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TAIL(), &plain.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_UNITS(), &plain.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_amount(&[word.clone()]), &Py::None)) } } } }) {
    continue;
    }
    if py_truthy(&_a_length_they_have_bounded(&[clean.clone(), unit.clone(), body.clone(), period.clone()])) {
    break;
    }
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    phrase = if py_truthy(&{ let __b = mcall(&slice_or_raise(&head.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CURRENCY(), &slice_or_raise(&head.clone(), &Py::None, &Py::Int(1), &Py::None))) } }) { head.clone() } else { _fmt(&[amount.clone()]) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand")]), &unit.clone()))) {
    phrase = op_modulo(&py_str("%s %s"), &py_tuple(vec![phrase.clone(), unit.clone()]));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&head.clone(), "lower", &[]), &py_str("half"))); if !py_truthy(&__b) { __b } else { unit.clone() } }) {
    phrase = op_modulo(&py_str("%s %s"), &py_tuple(vec![if py_truthy(&Py::Bool(py_contains(&py_str("aeiouh"), &slice_or_raise(&unit.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("half an") } else { py_str("half a") }, unit.clone()]));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&head.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { unit.clone() } }) {
    phrase = op_modulo(&py_str("%s %s"), &py_tuple(vec![if py_truthy(&Py::Bool(py_contains(&py_str("aeiouh"), &slice_or_raise(&unit.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an") } else { py_str("a") }, unit.clone()]));
    }
    if py_truthy(&half_more.clone()) {
    phrase = op_modulo(&py_str("%s and a half"), &phrase.clone());
    }
    if py_truthy(&unit.clone()) {
    let __unpk8 = py_unpack(&geti(&g_UNITS(), &unit.clone()), 2, None);
    family = __unpk8[0].clone();
    factor = __unpk8[1].clone();
    body_tail = mcall(&mcall(&py_str(" "), "join", &[body.clone()]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("k"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&period.clone())); if !py_truthy(&__b) { __b } else { _about_a_distance(&[clean.clone(), memory.clone()]) } } } }) {
    return py_tuple(vec![op_mul(&amount.clone(), &geti(&geti(&g_UNITS(), &py_str("km")), &Py::Int(1))), py_str("distance"), py_str("km"), Py::None, if py_truthy(&mcall(&phrase.clone(), "endswith", &[py_str(" k")])) { mcall(&slice_or_raise(&phrase.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), "strip", &[]) } else { phrase.clone() }, body_tail.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("money"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("pound"), py_str("pounds"), py_str("quid"), py_str("euro"), py_str("euros"), py_str("dollar"), py_str("dollars")]), &unit.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_TRIP(), "search", &[op_add(&op_add(&body_tail.clone(), &py_str(" ")), &tail.clone())]) } } } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("money"))); if !py_truthy(&__b) { __b } else { a_budget.clone() } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("span"))); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("weight"))); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    return py_tuple(vec![op_mul(&amount.clone(), &factor.clone()), family.clone(), unit.clone(), period.clone(), phrase.clone(), body_tail.clone()]);
    }
    if py_truthy(&{ let __b = _a_take_without_home(&[slice_or_raise(&words.clone(), &Py::None, &at.clone(), &Py::None)]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sum_word.clone())) } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    if py_truthy(&{ let __b = a_budget.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sum_word.clone())) } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    if py_truthy(&period.clone()) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sum_word.clone())); if !py_truthy(&__b) { __b } else { _a_count_the_question_asked_for(&[clean.clone(), memory.clone()]) } }) {
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    }
    phrase = { let __b = sum_word.clone(); if py_truthy(&__b) { __b } else { phrase.clone() } };
    return py_tuple(vec![amount.clone(), py_str("money"), py_str("money"), period.clone(), phrase.clone(), mcall(&mcall(&py_str(" "), "join", &[body.clone()]), "lower", &[])]);
    }
    if py_truthy(&sum_word.clone()) {
    return py_tuple(vec![amount.clone(), py_str("money"), py_str("money"), Py::None, sum_word.clone(), mcall(&mcall(&py_str(" "), "join", &[body.clone()]), "lower", &[])]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_CURRENCY(), &slice_or_raise(&head.clone(), &Py::None, &Py::Int(1), &Py::None)))) {
    return py_tuple(vec![amount.clone(), py_str("money"), py_str("money"), Py::None, phrase.clone(), py_str("")]);
    }
    return op_mul(&py_tuple(vec![Py::None]), &Py::Int(6));
    Py::None
}
pub fn _q(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("The number with its unit, unless the unit is the period the sentence already carries.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("money"), py_str("k"), py_str("grand")]), &unit.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&phrase.clone(), &unit.clone())) } }) {
    return phrase.clone();
    }
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![phrase.clone(), unit.clone()]));
    Py::None
}
pub fn _a_length_they_have_bounded(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut body: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: body"))) };
    let mut period: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut family: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a length the person has is being bounded or explained, not made into a subject.\n\n    Read only for a unit of time, only when the line says the time is theirs (\"i have\", \"ive\n    got\", \"got\"), and only when the words after the unit open on what the stretch is bounded by\n    (\"before i have to leave\", \"to get to the station\", \"free\", \"between meetings\"). A number\n    that is not in that frame is the length of something (\"a 20 minute walk\", \"the 20 minute\n    drive\"), and the words after it say what it is the length of, so nothing is loosened there.\n    ");
    family = geti(&mcall(&g_UNITS(), "get", &[unit.clone(), py_tuple(vec![Py::None, Py::None])]), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&family.clone(), &py_str("time"))); if py_truthy(&__b) { __b } else { period.clone() } }) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THEY_HAVE_THE_TIME(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&body.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_BOUNDED_BY(), &mcall(&mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(".,!?:")])));
    Py::None
}
pub fn _a_bound_not_a_budget(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the length is a limit the person put on it rather than time they have to spend.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_A_BOUND(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THEY_HAVE_IT(), "search", &[said.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_turn_above(args: &[Py]) -> Py {
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
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    Py::None
}
pub fn _the_change_is_the_line(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this line itself says the figure is weight coming off or going on.");
    return call_builtin("bool", &[mcall(&g_A_CHANGE_SAID(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_weight_they_mean_themselves(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether this line, or a turn or two back, says the figure is about a body.");
    if py_truthy(&mcall(&g_I_WEIGH(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    return Py::Bool(true);
    }
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(2)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("bot")]));
    for __item in __iter2 {
    key = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&g_WEIGHT_ON_THE_TABLE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WEIGHT_GOING_ON(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_thing_was_named(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut above: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether this line or the turn above named a thing this figure could be the weight of.\n\n    \"its about 3 kilos\" after \"i bought a whole salmon by mistake\" is the salmon's weight, and\n    asking \"is that you, or something you are carrying\" is the bot not reading its own\n    conversation. The names come from the understanding engine's own list, so this is the same\n    set of things the rest of the bot recognises.\n    ");
    if py_truthy(&mcall(&g_SOMETHING_WEIGHED(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    return Py::Bool(true);
    }
    above = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("user")]);
    if py_truthy(&Py::Bool(!py_truthy(&above.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut THINGS: Py = gen_understanding::understanding::topics::g_THINGS();
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
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&call_builtin("str", &[above.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&THINGS.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_span_they_are_in(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line, or the turn above it, says this stretch has already been lived.");
    said = py_list(vec![clean.clone()]);
    above = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("user")]);
    if py_truthy(&above.clone()) {
    let _ = mcall(&said.clone(), "append", &[above.clone()]);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&g_SPAN_FRAME(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_count_the_question_asked_for(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the turn above asked \"how many <thing>\" and nothing on the table is money.\n\n    A rate with no currency word in it (\"2000 a day\") is read as money, and that is right for\n    \"45 a month\" out of nowhere. It is wrong when the bot has just asked how many of something:\n    the person is answering that question, so the number counts the thing asked about.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    above = if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    said = py_list(vec![mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&above.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&above.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(2), &Py::None)) { let mut one: Py = __item.clone(); __out.push(mcall(&g_A_COUNT_ASKED_FOR(), "search", &[one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&g_A_WAY_OF_PAYING(), "search", &[one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_fact_about_a_length(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let _ = py_str("Whether the turn above asked how long something lasts, is, or takes.");
    above = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = above.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_QUESTION_ABOUT_A_LENGTH(), "search", &[above.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_BUDGET_QUESTION(), "search", &[above.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut family: Py = Py::None;
    let mut left: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut period: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A reply to a line that is nothing but a number, or None to leave it to the caller.");
    let __unpk1 = py_unpack(&_read(&[clean.clone(), memory.clone()]), 6, None);
    value = __unpk1[0].clone();
    family = __unpk1[1].clone();
    unit = __unpk1[2].clone();
    period = __unpk1[3].clone();
    phrase = __unpk1[4].clone();
    tail = __unpk1[5].clone();
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("span"))); if !py_truthy(&__b) { __b } else { { let __b = _the_turn_above(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_span_they_are_in(&[clean.clone(), memory.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&period.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_le(&value.clone(), &Py::Int(180)); if !py_truthy(&__b) { __b } else { _a_fact_about_a_length(&[memory.clone()]) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("span")]), &family.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&period.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_REMAINDER(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]) } } } }) {
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), _q(&[phrase.clone(), unit.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_A_WHAT_IS_LEFT(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&left.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&family.clone(), &py_str("time"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&period.clone())); if !py_truthy(&__b) { __b } else { _a_bound_not_a_budget(&[clean.clone()]) } } } }) {
    limit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), _q(&[phrase.clone(), unit.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_A_LIMIT_ON_A_LENGTH(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&limit.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&limit.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    reply = _say(&[family.clone(), _q(&[phrase.clone(), unit.clone()]), value.clone(), period.clone(), tail.clone(), memory.clone(), clean.clone(), unit.clone()]);
    return if py_truthy(&reply.clone()) { op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { Py::None };
    Py::None
}
pub fn the_thing_they_pay_for(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the rate in the line is for, or None when the line only gives the number.\n\n    \"my mortgage is 900 a month, is that normal\" names the thing, so the question can be answered\n    the way a person answers it - by comparing that thing with the same thing elsewhere, which is\n    what the price pool does. \"its 45 a month, is that a lot\" names nothing: there is no \"it\" in\n    the line to compare with anything, and the only answer about what they said is the sum it\n    comes to over a year. Same reading, one word apart, so the caller asks this before choosing.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&mcall(&g_THEIR_THING(), "findall", &[said.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone()))) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_rate_they_asked_about(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut period: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_period: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The arithmetic on a rate the line itself carries and judges, or None.\n\n    \"its 45 a month, is that a lot\" is a question about that rate. The comparison advice the price\n    pool gives - two or three places, the same thing - is for a price with no figure in the line,\n    because there is nothing to work out; here there is, and the sum it comes to over a year is\n    the one answer that is about what the person said. A line that does name the thing (\"my\n    mortgage is 900 a month\") is the other way round - see `the_thing_they_pay_for`.\n    ");
    let __unpk1 = py_unpack(&_read(&[clean.clone(), memory.clone()]), 6, None);
    value = __unpk1[0].clone();
    family = __unpk1[1].clone();
    unit = __unpk1[2].clone();
    period = __unpk1[3].clone();
    phrase = __unpk1[4].clone();
    tail = __unpk1[5].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&family.clone(), &py_str("money"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&period.clone())) } }) {
    return Py::None;
    }
    let mut PRICE_JUDGEMENT: Py = crate::compose::prices::g_PRICE_JUDGEMENT();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&PRICE_JUDGEMENT.clone(), "search", &[said.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SAVINGS_AFTER_A_RATE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("month"), py_str("week"), py_str("year")]), &period.clone())) } }) {
    year = op_mul(&value.clone(), &geti(&py_dict(vec![(py_str("month"), Py::Int(12)), (py_str("week"), Py::Int(52)), (py_str("year"), Py::Int(1))]), &period.clone()));
    said_period = geti(&py_dict(vec![(py_str("month"), py_str("a month")), (py_str("week"), py_str("a week")), (py_str("year"), py_str("a year"))]), &period.clone());
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), _q(&[phrase.clone(), unit.clone()])), ("period".to_string(), said_period.clone()), ("year".to_string(), _fmt(&[year.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_SAVINGS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    return op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    reply = _say(&[family.clone(), _q(&[phrase.clone(), unit.clone()]), value.clone(), period.clone(), tail.clone(), memory.clone(), clean.clone(), unit.clone()]);
    return if py_truthy(&reply.clone()) { op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { Py::None };
    Py::None
}
pub fn _without_a_pointing_tail(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words with a trailing preposition and pronoun taken off (\"a day for it\").\n\n    The pronoun says the subject is the one already on the table, so it adds nothing the number\n    does not already have. It is not a new subject like the noun in \"20 minutes for the roast\".\n    ");
    if py_truthy(&op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:;")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_POINTING_BACK(), &geti(&tail.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PRONOUN(), &geti(&tail.clone(), &Py::Int(1)))) } }) {
    return slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    }
    return words.clone();
    Py::None
}
pub fn _without_a_hedge(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words with the speaker's own doubt taken off the end (\"... a day, i think\").\n\n    \"i think\", \"i reckon\", \"i guess\" after a number say how sure the person is of it, not what it\n    is about, and the number is still the whole of the line. Only the speaker's doubt: \"at least\"\n    and \"at most\" change what the figure means and are left where they are.\n    ");
    if py_truthy(&op_ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:;")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("id"), py_str("i'd")]), &geti(&tail.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HEDGES(), &geti(&tail.clone(), &Py::Int(1)))) } }) {
    return slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    }
    return words.clone();
    Py::None
}
pub fn _without_the_household(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words with the household taken off, if that is what they are.\n\n    The phrase sits in front of the period (\"3000 between us a month\") or behind it (\"3000 a\n    month between us\"), and either way it is the people, not the thing the number is about.\n    ");
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:;")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&g_THE_HOUSEHOLD());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&call_builtin("tuple", &[slice_or_raise(&plain.clone(), &Py::None, &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &phrase.clone()))) {
    return slice_or_raise(&words.clone(), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("tuple", &[slice_or_raise(&plain.clone(), &op_neg(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &phrase.clone())) } }) {
    return slice_or_raise(&words.clone(), &Py::None, &op_neg(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return words.clone();
    Py::None
}
pub fn _a_take_without_home(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lead: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lead"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line led with \"take\" and never said \"home\": a count, not a wage.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lead.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:;")])); } py_list(__out) })() };
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TAKE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str("home"))) } };
    Py::None
}
pub fn _without_a_remark(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let _ = py_str("A number with the person's own tail on it: \"a day for it, i think\".");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    trimmed = _without_a_pointing_tail(&[_without_a_hedge(&[words.clone()])]);
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &words.clone()))) {
    break;
    }
    words = trimmed.clone();
    }
    return words.clone();
    Py::None
}
pub fn _in_a_conversation(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the bot has already been told what the number is about.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&turn.clone(), "get", &[py_str("user")])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _not_short(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The pool with the lines that tell the person the time is short taken out, if it is not.");
    if py_truthy(&op_lt(&value.clone(), &g_A_STRETCH_THAT_IS_NOT_SHORT())) {
    return pool.clone();
    }
    kept = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&line.clone(), &py_str("is tight"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&line.clone(), &py_str("faster than it looks"))) } }) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = kept.clone(); if py_truthy(&__b) { __b } else { pool.clone() } };
    Py::None
}
pub fn _without_a_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut line: Py = Py::None;
    let _ = py_str("\"5 minutes\" after \"how long shall i boil an egg\" is not an opening: asking what it is for\n    again, one turn after being told, is the bot not listening. Lines that ask are for a number\n    that arrives out of nowhere.");
    return { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("?")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { pool.clone() } };
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
    let mut family: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut q: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "q") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: q"))) };
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut period: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut tail: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut memory: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut unit: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { Py::None };
    let mut bike: Py = Py::None;
    let mut body: Py = Py::None;
    let mut drive: Py = Py::None;
    let mut fast: Py = Py::None;
    let mut kilos: Py = Py::None;
    let mut other: Py = Py::None;
    let mut per_day: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pounds: Py = Py::None;
    let mut said_kilos: Py = Py::None;
    let mut second: Py = Py::None;
    let mut slow: Py = Py::None;
    let mut walk: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("The reply for one family of number, or None.");
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("money")))) {
    if py_truthy(&mcall(&g_INCOME(), "search", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) {
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("month")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("year".to_string(), _fmt(&[op_mul(&value.clone(), &Py::Int(12))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_INCOME_MONTH(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("week"), py_str("fortnight")]), &period.clone()))) {
    weeks = if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("week")))) { value.clone() } else { op_truediv(&value.clone(), &Py::Float(2.0)) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("month".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&op_mul(&weeks.clone(), &Py::Int(52)), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e))])), ("year".to_string(), _fmt(&[op_mul(&weeks.clone(), &Py::Int(52))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_INCOME_WEEK(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("year")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("month".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&value.clone(), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e))])), ("week".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&value.clone(), &Py::Float(52.0))]).unwrap_or_else(|e| exc::raise(e))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_INCOME_YEAR(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_INCOME_PLAIN(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("month")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("year".to_string(), _fmt(&[op_mul(&value.clone(), &Py::Int(12))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_MONEY_MONTH(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("week")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("month".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&op_mul(&value.clone(), &Py::Int(52)), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e))])), ("year".to_string(), _fmt(&[op_mul(&value.clone(), &Py::Int(52))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_MONEY_WEEK(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("year")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("month".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&value.clone(), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e))])), ("week".to_string(), _fmt(&[call_builtin("round", &[op_truediv(&value.clone(), &Py::Float(52.0))]).unwrap_or_else(|e| exc::raise(e))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_MONEY_YEAR(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("day"), py_str("fortnight")]), &period.clone()))) {
    per_day = if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("fortnight")))) { op_truediv(&value.clone(), &Py::Float(14.0)) } else { value.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("month".to_string(), _fmt(&[call_builtin("round", &[op_mul(&per_day.clone(), &Py::Int(30))]).unwrap_or_else(|e| exc::raise(e))])), ("year".to_string(), _fmt(&[call_builtin("round", &[op_mul(&per_day.clone(), &Py::Int(365))]).unwrap_or_else(|e| exc::raise(e))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_MONEY_DAY(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_MONEY_PLAIN(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("time")))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&period.clone(), &py_str("day"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TRIP(), "search", &[call_builtin("str", &[{ let __b = tail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("week".to_string(), _minutes(&[op_mul(&value.clone(), &Py::Int(7))])), ("month".to_string(), _minutes(&[op_mul(&value.clone(), &Py::Int(30))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_TIME_A_DAY(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&mcall(&g_TRIP(), "search", &[tail.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("day".to_string(), _minutes(&[op_mul(&value.clone(), &Py::Int(2))])), ("week".to_string(), _minutes(&[op_mul(&value.clone(), &Py::Int(10))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_TIME_TRIP(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("week")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("day".to_string(), _minutes(&[op_truediv(&value.clone(), &Py::Float(7.0))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_TIME_WEEK(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&op_gt(&value.clone(), &Py::Int(180))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_TIME_LONG(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    pool = if py_truthy(&Py::Bool(!py_truthy(&_in_a_conversation(&[memory.clone()])))) { g_TIME_BUDGET() } else { _without_a_question(&[g_TIME_BUDGET()]) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[_not_short(&[pool.clone(), value.clone()]), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("distance")))) {
    if py_truthy(&op_le(&value.clone(), &Py::Int(0))) {
    return Py::None;
    }
    walk = call_builtin("int", &[call_builtin("round", &[op_mul(&value.clone(), &Py::Int(20))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    bike = call_builtin("int", &[call_builtin("round", &[op_mul(&value.clone(), &Py::Int(5))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    drive = call_builtin("int", &[call_builtin("round", &[if py_truthy(&op_gt(&value.clone(), &Py::Int(10))) { op_mul(&value.clone(), &Py::Float(1.33)) } else { op_mul(&value.clone(), &Py::Float(2.4)) }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_le(&value.clone(), &Py::Int(3))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("walk".to_string(), walk.clone()), ("bike".to_string(), bike.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_DISTANCE_NEAR(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&op_le(&value.clone(), &Py::Int(10))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("bike".to_string(), bike.clone()), ("drive".to_string(), _minutes(&[drive.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_DISTANCE_MID(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&mcall(&g_TRIP(), "search", &[call_builtin("str", &[{ let __b = tail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    pool = g_DISTANCE_FAR_TRIP();
    } else {
    if py_truthy(&op_gt(&value.clone(), &Py::Int(60))) {
    pool = g_DISTANCE_DAY_OUT();
    } else {
    pool = g_DISTANCE_FAR();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("drive".to_string(), _minutes(&[drive.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("size")))) {
    other = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bby\\s+(\\d+(?:\\.\\d+)?|"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_NUMBERS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b")), tail.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&other.clone(), &Py::None))) {
    return Py::None;
    }
    second = _amount(&[mcall(&other.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&second.clone())); if py_truthy(&__b) { __b } else { op_le(&second.clone(), &Py::Int(0)) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("b".to_string(), _fmt(&[second.clone()])), ("area".to_string(), _fmt(&[op_mul(&value.clone(), &second.clone())]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_AREA(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("span")))) {
    if py_truthy(&op_le(&value.clone(), &Py::Int(3))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_SPAN_DAYS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&op_le(&value.clone(), &Py::Int(21))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_SPAN_WEEKS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&op_le(&value.clone(), &Py::Int(400))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_SPAN_MONTHS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("weight")))) {
    if py_truthy(&op_le(&value.clone(), &Py::Float(0.2))) {
    return Py::None;
    }
    kilos = op_modulo(&py_str("%s kilo%s"), &py_tuple(vec![_fmt(&[call_builtin("round", &[value.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]), if py_truthy(&Py::Bool(py_eq(&call_builtin("round", &[value.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { py_str("") } else { py_str("s") }]));
    pounds = op_truediv(&value.clone(), &Py::Float(0.4536));
    said_kilos = Py::Bool(py_contains(&py_tuple(vec![py_str("kilo"), py_str("kilos"), py_str("kilogram"), py_str("kilograms"), py_str("kg"), py_str("kgs")]), &{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }));
    other = if py_truthy(&said_kilos.clone()) { op_modulo(&py_str("%s pound%s"), &py_tuple(vec![_fmt(&[call_builtin("round", &[pounds.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e))]), if py_truthy(&Py::Bool(py_eq(&call_builtin("round", &[pounds.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { py_str("") } else { py_str("s") }])) } else { kilos.clone() };
    slow = call_builtin("max", &[Py::Int(1), call_builtin("int", &[call_builtin("round", &[op_truediv(&pounds.clone(), &Py::Float(2.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    fast = call_builtin("max", &[Py::Int(1), call_builtin("int", &[call_builtin("round", &[pounds.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    body = _a_weight_they_mean_themselves(&[memory.clone(), clean.clone()]);
    if py_truthy(&{ let __b = body.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&fast.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = op_le(&value.clone(), &g_A_CHANGE_IS_THIS_SMALL()); if py_truthy(&__b) { __b } else { _the_change_is_the_line(&[clean.clone()]) } } } } } }) {
    pool = g_WEIGHT_LOSS();
    } else {
    if py_truthy(&mcall(&g_I_WEIGH(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    pool = g_WEIGHT_THEIR_OWN();
    } else {
    if py_truthy(&_a_thing_was_named(&[memory.clone(), clean.clone()])) {
    pool = g_WEIGHT_OF_A_THING();
    } else {
    pool = g_WEIGHT_PLAIN();
    }
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), q.clone()), ("other".to_string(), other.clone()), ("slow".to_string(), slow.clone()), ("fast".to_string(), fast.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _the_unit_of_one(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The unit as it is written after a figure of one: \"1 glass\", not \"1 glasses\".");
    word = call_builtin("str", &[{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("sses")])) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ies")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _the_answer_range(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut above: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "above") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: above"))) };
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut period: Py = Py::None;
    let mut rules: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(low, high, unit, said) for the rule with a unit in the answer, or None.\n\n    A range is the usual shape (\"six to eight glasses a day\"); a single aiming figure is the other\n    (\"aim for about 150 minutes a week\"), and both are a rule the person's own figure sits against.\n    ");
    rules = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_RANGE_WITH_A_UNIT(), "finditer", &[above.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    low = _amount(&[mcall(&mcall(&found.clone(), "group", &[py_str("lo")]), "lower", &[])]);
    high = _amount(&[mcall(&mcall(&found.clone(), "group", &[py_str("hi")]), "lower", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&high.clone())); if py_truthy(&__b) { __b } else { { let __b = op_le(&low.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_le(&high.clone(), &Py::Int(0)) } } } } } }) {
    continue;
    }
    if py_truthy(&op_lt(&high.clone(), &low.clone())) {
    let __unpk2 = py_unpack(&py_tuple(vec![high.clone(), low.clone()]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    }
    let _ = mcall(&rules.clone(), "append", &[py_tuple(vec![mcall(&found.clone(), "start", &[]), low.clone(), high.clone(), mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]), mcall(&found.clone(), "group", &[py_str("period")])])]);
    }
    let __iter3 = py_iter_or_raise(&mcall(&g_A_SINGLE_RULE(), "finditer", &[above.clone()]));
    for __item in __iter3 {
    found = __item.clone();
    value = _amount(&[mcall(&mcall(&found.clone(), "group", &[py_str("num")]), "lower", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { op_le(&value.clone(), &Py::Int(0)) } }) {
    continue;
    }
    let _ = mcall(&rules.clone(), "append", &[py_tuple(vec![mcall(&found.clone(), "start", &[]), value.clone(), value.clone(), mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]), mcall(&found.clone(), "group", &[py_str("period")])])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&rules.clone()))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rules.clone()); __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let above = above.clone(); let found = found.clone(); let high = high.clone(); let low = low.clone(); let period = period.clone(); let rules = rules.clone(); let unit = unit.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let above = above.clone(); let found = found.clone(); let high = high.clone(); let low = low.clone(); let period = period.clone(); let rules = rules.clone(); let unit = unit.clone(); let value = value.clone(); let rule: Py = __kwp(args, 0, "rule").unwrap_or(Py::None); geti(&rule.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 5, None);
    _u = __unpk4[0].clone();
    low = __unpk4[1].clone();
    high = __unpk4[2].clone();
    unit = __unpk4[3].clone();
    period = __unpk4[4].clone();
    return py_tuple(vec![low.clone(), high.clone(), unit.clone(), _the_rule_said(&[low.clone(), if py_truthy(&Py::Bool(py_eq(&low.clone(), &high.clone()))) { Py::None } else { high.clone() }, unit.clone(), period.clone()])]);
    Py::None
}
pub fn _the_rule_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut high: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "high") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: high"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut period: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut figure: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The rule as the reply says it back: \"6 to 8 glasses a day\", \"150 minutes a week\".");
    figure = if py_truthy(&Py::Bool(!py_is(&high.clone(), &Py::None))) { op_modulo(&py_str("%s to %s"), &py_tuple(vec![_fmt(&[low.clone()]), _fmt(&[high.clone()])])) } else { _fmt(&[low.clone()]) };
    said = op_modulo(&py_str("%s %s"), &py_tuple(vec![figure.clone(), mcall(&g_UNIT_SAID(), "get", &[mcall(&call_builtin("str", &[unit.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), mcall(&call_builtin("str", &[unit.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]));
    when = mcall(&g_PERIOD_NAMED(), "get", &[mcall(&call_builtin("str", &[{ let __b = period.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    return op_add(&said.clone(), &if py_truthy(&when.clone()) { op_add(&py_str(" "), &when.clone()) } else { py_str("") });
    Py::None
}
pub fn _the_family(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut pattern: Py = Py::None;
    let __iter1 = py_iter_or_raise(&g_UNIT_IS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    family = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[unit.clone()])) {
    return family.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn the_figure_against_the_answer(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut alone: Py = Py::None;
    let mut family: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut figure_said: Py = Py::None;
    let mut high: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_unit: Py = Py::None;
    let mut step: Py = Py::None;
    let mut their_unit: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The figure the person gives, read in the unit of the rule the answer just named, or None.\n\n    Only a line that is a figure and little else is read, and only when the bot's last answer\n    named a rule with a unit. A money amount, a measurement or an age is left to the entries that\n    read those, and so is a line whose own unit is not the answer's.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_A_UNIT_ITSELF(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_AN_AGE(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ABOUT_A_FIGURE(), "search", &[line.clone()])))) {
    return Py::None;
    }
    alone = mcall(&g_A_FIGURE_ALONE(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(py_is(&alone.clone(), &Py::None))) {
    return Py::None;
    }
    figure = _amount(&[mcall(&alone.clone(), "group", &[py_str("num")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&figure.clone())); if py_truthy(&__b) { __b } else { { let __b = op_le(&figure.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_gt(&figure.clone(), &Py::Int(500)) } } } }) {
    return Py::None;
    }
    reading = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let __b = mcall(&reading.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("maths"))) } }) {
    return Py::None;
    }
    above = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = gen_memory::memory::last_bot(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&above.clone()))) {
    return Py::None;
    }
    named = _the_answer_range(&[above.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&named.clone(), 4, None);
    low = __unpk1[0].clone();
    high = __unpk1[1].clone();
    unit = __unpk1[2].clone();
    said = __unpk1[3].clone();
    family = _the_family(&[unit.clone()]);
    said_unit = mcall(&g_UNIT_SAID(), "get", &[unit.clone(), unit.clone()]);
    their_unit = mcall(&{ let __b = mcall(&alone.clone(), "group", &[py_str("what")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = their_unit.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&_the_family(&[their_unit.clone()]), &family.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&family.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_lt(&op_mul(&figure.clone(), &g_TEN_TIMES()), &low.clone()); if py_truthy(&__b) { __b } else { op_gt(&figure.clone(), &op_mul(&high.clone(), &g_TEN_TIMES())) } }) {
    return Py::None;
    }
    figure_said = _fmt(&[figure.clone()]);
    said_unit = if py_truthy(&Py::Bool(py_eq(&figure.clone(), &Py::Int(1)))) { _the_unit_of_one(&[said_unit.clone()]) } else { said_unit.clone() };
    if py_truthy(&op_lt(&figure.clone(), &low.clone())) {
    step = geti(&mcall(&g_STEPS(), "get", &[family.clone(), g_A_STEP_EITHER_WAY()]), &py_str("below"));
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure_said.clone()), ("unit".to_string(), said_unit.clone()), ("range".to_string(), said.clone()), ("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_FIGURE_BELOW(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    } else {
    if py_truthy(&op_gt(&figure.clone(), &high.clone())) {
    step = geti(&mcall(&g_STEPS(), "get", &[family.clone(), g_A_STEP_EITHER_WAY()]), &py_str("above"));
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure_said.clone()), ("unit".to_string(), said_unit.clone()), ("range".to_string(), said.clone()), ("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_FIGURE_ABOVE(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    } else {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure_said.clone()), ("unit".to_string(), said_unit.clone()), ("range".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::pick(&[g_FIGURE_INSIDE(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    }
    return op_add(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.quantity".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_A_FIGURE" => Some(g_ABOUT_A_FIGURE()),
            "AN_AGE" => Some(g_AN_AGE()),
            "AREA" => Some(g_AREA()),
            "A_BOUND" => Some(g_A_BOUND()),
            "A_BUDGET_QUESTION" => Some(g_A_BUDGET_QUESTION()),
            "A_BUDGET_THEY_SET" => Some(g_A_BUDGET_THEY_SET()),
            "A_CHANGE_IS_THIS_SMALL" => Some(g_A_CHANGE_IS_THIS_SMALL()),
            "A_CHANGE_SAID" => Some(g_A_CHANGE_SAID()),
            "A_COUNT_ASKED_FOR" => Some(g_A_COUNT_ASKED_FOR()),
            "A_FIGURE_ALONE" => Some(g_A_FIGURE_ALONE()),
            "A_LIMIT_ON_A_LENGTH" => Some(g_A_LIMIT_ON_A_LENGTH()),
            "A_PRONOUN" => Some(g_A_PRONOUN()),
            "A_QUESTION_ABOUT_A_LENGTH" => Some(g_A_QUESTION_ABOUT_A_LENGTH()),
            "A_RANGE_WITH_A_UNIT" => Some(g_A_RANGE_WITH_A_UNIT()),
            "A_REMAINDER" => Some(g_A_REMAINDER()),
            "A_SINGLE_RULE" => Some(g_A_SINGLE_RULE()),
            "A_STEP_EITHER_WAY" => Some(g_A_STEP_EITHER_WAY()),
            "A_STRETCH_THAT_IS_NOT_SHORT" => Some(g_A_STRETCH_THAT_IS_NOT_SHORT()),
            "A_WAY_OF_PAYING" => Some(g_A_WAY_OF_PAYING()),
            "A_WHAT_IS_LEFT" => Some(g_A_WHAT_IS_LEFT()),
            "BOUNDED_BY" => Some(g_BOUNDED_BY()),
            "CURRENCY" => Some(g_CURRENCY()),
            "DISTANCE_DAY_OUT" => Some(g_DISTANCE_DAY_OUT()),
            "DISTANCE_FAR" => Some(g_DISTANCE_FAR()),
            "DISTANCE_FAR_TRIP" => Some(g_DISTANCE_FAR_TRIP()),
            "DISTANCE_MID" => Some(g_DISTANCE_MID()),
            "DISTANCE_NEAR" => Some(g_DISTANCE_NEAR()),
            "DOUBT_ON_THE_FIGURE" => Some(g_DOUBT_ON_THE_FIGURE()),
            "FIGURE_ABOVE" => Some(g_FIGURE_ABOVE()),
            "FIGURE_BELOW" => Some(g_FIGURE_BELOW()),
            "FIGURE_INSIDE" => Some(g_FIGURE_INSIDE()),
            "HEDGES" => Some(g_HEDGES()),
            "INCOME" => Some(g_INCOME()),
            "INCOME_MONTH" => Some(g_INCOME_MONTH()),
            "INCOME_PLAIN" => Some(g_INCOME_PLAIN()),
            "INCOME_WEEK" => Some(g_INCOME_WEEK()),
            "INCOME_YEAR" => Some(g_INCOME_YEAR()),
            "I_WEIGH" => Some(g_I_WEIGH()),
            "LEAD" => Some(g_LEAD()),
            "MONEY_DAY" => Some(g_MONEY_DAY()),
            "MONEY_MONTH" => Some(g_MONEY_MONTH()),
            "MONEY_NOUN" => Some(g_MONEY_NOUN()),
            "MONEY_PLAIN" => Some(g_MONEY_PLAIN()),
            "MONEY_WEEK" => Some(g_MONEY_WEEK()),
            "MONEY_YEAR" => Some(g_MONEY_YEAR()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_UNIT_ITSELF" => Some(g_NOT_A_UNIT_ITSELF()),
            "NUMBERS" => Some(g_NUMBERS()),
            "ON_THE_TABLE_DISTANCE" => Some(g_ON_THE_TABLE_DISTANCE()),
            "ON_THE_TABLE_MONEY" => Some(g_ON_THE_TABLE_MONEY()),
            "PERIODS" => Some(g_PERIODS()),
            "PERIOD_NAMED" => Some(g_PERIOD_NAMED()),
            "PER_DAY" => Some(g_PER_DAY()),
            "POINTING_BACK" => Some(g_POINTING_BACK()),
            "SAVINGS" => Some(g_SAVINGS()),
            "SAVINGS_AFTER_A_RATE" => Some(g_SAVINGS_AFTER_A_RATE()),
            "SOMETHING_WEIGHED" => Some(g_SOMETHING_WEIGHED()),
            "SPAN_DAYS" => Some(g_SPAN_DAYS()),
            "SPAN_FRAME" => Some(g_SPAN_FRAME()),
            "SPAN_MONTHS" => Some(g_SPAN_MONTHS()),
            "SPAN_WEEKS" => Some(g_SPAN_WEEKS()),
            "STEPS" => Some(g_STEPS()),
            "SUBJECT" => Some(g_SUBJECT()),
            "SUM_IN_A_WORD" => Some(g_SUM_IN_A_WORD()),
            "TAIL" => Some(g_TAIL()),
            "TAKE" => Some(g_TAKE()),
            "TENS" => Some(g_TENS()),
            "TEN_TIMES" => Some(g_TEN_TIMES()),
            "THEIR_THING" => Some(g_THEIR_THING()),
            "THEY_HAVE_IT" => Some(g_THEY_HAVE_IT()),
            "THEY_HAVE_THE_TIME" => Some(g_THEY_HAVE_THE_TIME()),
            "THE_HOUSEHOLD" => Some(g_THE_HOUSEHOLD()),
            "TIME_A_DAY" => Some(g_TIME_A_DAY()),
            "TIME_BUDGET" => Some(g_TIME_BUDGET()),
            "TIME_LONG" => Some(g_TIME_LONG()),
            "TIME_TRIP" => Some(g_TIME_TRIP()),
            "TIME_WEEK" => Some(g_TIME_WEEK()),
            "TRIP" => Some(g_TRIP()),
            "UNITS" => Some(g_UNITS()),
            "UNIT_IS" => Some(g_UNIT_IS()),
            "UNIT_SAID" => Some(g_UNIT_SAID()),
            "WEIGHT_GOING_ON" => Some(g_WEIGHT_GOING_ON()),
            "WEIGHT_LOSS" => Some(g_WEIGHT_LOSS()),
            "WEIGHT_OF_A_THING" => Some(g_WEIGHT_OF_A_THING()),
            "WEIGHT_ON_THE_TABLE" => Some(g_WEIGHT_ON_THE_TABLE()),
            "WEIGHT_PLAIN" => Some(g_WEIGHT_PLAIN()),
            "WEIGHT_THEIR_OWN" => Some(g_WEIGHT_THEIR_OWN()),
            "_a_bound_not_a_budget" => Some(py_func("_a_bound_not_a_budget", |args: &[Py]| _a_bound_not_a_budget(args))),
            "_a_count_the_question_asked_for" => Some(py_func("_a_count_the_question_asked_for", |args: &[Py]| _a_count_the_question_asked_for(args))),
            "_a_fact_about_a_length" => Some(py_func("_a_fact_about_a_length", |args: &[Py]| _a_fact_about_a_length(args))),
            "_a_length_they_have_bounded" => Some(py_func("_a_length_they_have_bounded", |args: &[Py]| _a_length_they_have_bounded(args))),
            "_a_span_they_are_in" => Some(py_func("_a_span_they_are_in", |args: &[Py]| _a_span_they_are_in(args))),
            "_a_take_without_home" => Some(py_func("_a_take_without_home", |args: &[Py]| _a_take_without_home(args))),
            "_a_thing_was_named" => Some(py_func("_a_thing_was_named", |args: &[Py]| _a_thing_was_named(args))),
            "_a_weight_they_mean_themselves" => Some(py_func("_a_weight_they_mean_themselves", |args: &[Py]| _a_weight_they_mean_themselves(args))),
            "_about_a_distance" => Some(py_func("_about_a_distance", |args: &[Py]| _about_a_distance(args))),
            "_amount" => Some(py_func("_amount", |args: &[Py]| _amount(args))),
            "_fmt" => Some(py_func("_fmt", |args: &[Py]| _fmt(args))),
            "_in_a_conversation" => Some(py_func("_in_a_conversation", |args: &[Py]| _in_a_conversation(args))),
            "_minutes" => Some(py_func("_minutes", |args: &[Py]| _minutes(args))),
            "_not_short" => Some(py_func("_not_short", |args: &[Py]| _not_short(args))),
            "_number_at" => Some(py_func("_number_at", |args: &[Py]| _number_at(args))),
            "_q" => Some(py_func("_q", |args: &[Py]| _q(args))),
            "_read" => Some(py_func("_read", |args: &[Py]| _read(args))),
            "_say" => Some(py_func("_say", |args: &[Py]| _say(args))),
            "_the_answer_range" => Some(py_func("_the_answer_range", |args: &[Py]| _the_answer_range(args))),
            "_the_change_is_the_line" => Some(py_func("_the_change_is_the_line", |args: &[Py]| _the_change_is_the_line(args))),
            "_the_family" => Some(py_func("_the_family", |args: &[Py]| _the_family(args))),
            "_the_money_words" => Some(py_func("_the_money_words", |args: &[Py]| _the_money_words(args))),
            "_the_rule_said" => Some(py_func("_the_rule_said", |args: &[Py]| _the_rule_said(args))),
            "_the_sum_inside_the_word" => Some(py_func("_the_sum_inside_the_word", |args: &[Py]| _the_sum_inside_the_word(args))),
            "_the_turn_above" => Some(py_func("_the_turn_above", |args: &[Py]| _the_turn_above(args))),
            "_the_unit_of_one" => Some(py_func("_the_unit_of_one", |args: &[Py]| _the_unit_of_one(args))),
            "_without_a_hedge" => Some(py_func("_without_a_hedge", |args: &[Py]| _without_a_hedge(args))),
            "_without_a_pointing_tail" => Some(py_func("_without_a_pointing_tail", |args: &[Py]| _without_a_pointing_tail(args))),
            "_without_a_question" => Some(py_func("_without_a_question", |args: &[Py]| _without_a_question(args))),
            "_without_a_remark" => Some(py_func("_without_a_remark", |args: &[Py]| _without_a_remark(args))),
            "_without_the_household" => Some(py_func("_without_the_household", |args: &[Py]| _without_the_household(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_figure_against_the_answer" => Some(py_func("the_figure_against_the_answer", |args: &[Py]| the_figure_against_the_answer(args))),
            "the_rate_they_asked_about" => Some(py_func("the_rate_they_asked_about", |args: &[Py]| the_rate_they_asked_about(args))),
            "the_thing_they_pay_for" => Some(py_func("the_thing_they_pay_for", |args: &[Py]| the_thing_they_pay_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_A_FIGURE" => s_ABOUT_A_FIGURE(v),
            "AN_AGE" => s_AN_AGE(v),
            "AREA" => s_AREA(v),
            "A_BOUND" => s_A_BOUND(v),
            "A_BUDGET_QUESTION" => s_A_BUDGET_QUESTION(v),
            "A_BUDGET_THEY_SET" => s_A_BUDGET_THEY_SET(v),
            "A_CHANGE_IS_THIS_SMALL" => s_A_CHANGE_IS_THIS_SMALL(v),
            "A_CHANGE_SAID" => s_A_CHANGE_SAID(v),
            "A_COUNT_ASKED_FOR" => s_A_COUNT_ASKED_FOR(v),
            "A_FIGURE_ALONE" => s_A_FIGURE_ALONE(v),
            "A_LIMIT_ON_A_LENGTH" => s_A_LIMIT_ON_A_LENGTH(v),
            "A_PRONOUN" => s_A_PRONOUN(v),
            "A_QUESTION_ABOUT_A_LENGTH" => s_A_QUESTION_ABOUT_A_LENGTH(v),
            "A_RANGE_WITH_A_UNIT" => s_A_RANGE_WITH_A_UNIT(v),
            "A_REMAINDER" => s_A_REMAINDER(v),
            "A_SINGLE_RULE" => s_A_SINGLE_RULE(v),
            "A_STEP_EITHER_WAY" => s_A_STEP_EITHER_WAY(v),
            "A_STRETCH_THAT_IS_NOT_SHORT" => s_A_STRETCH_THAT_IS_NOT_SHORT(v),
            "A_WAY_OF_PAYING" => s_A_WAY_OF_PAYING(v),
            "A_WHAT_IS_LEFT" => s_A_WHAT_IS_LEFT(v),
            "BOUNDED_BY" => s_BOUNDED_BY(v),
            "CURRENCY" => s_CURRENCY(v),
            "DISTANCE_DAY_OUT" => s_DISTANCE_DAY_OUT(v),
            "DISTANCE_FAR" => s_DISTANCE_FAR(v),
            "DISTANCE_FAR_TRIP" => s_DISTANCE_FAR_TRIP(v),
            "DISTANCE_MID" => s_DISTANCE_MID(v),
            "DISTANCE_NEAR" => s_DISTANCE_NEAR(v),
            "DOUBT_ON_THE_FIGURE" => s_DOUBT_ON_THE_FIGURE(v),
            "FIGURE_ABOVE" => s_FIGURE_ABOVE(v),
            "FIGURE_BELOW" => s_FIGURE_BELOW(v),
            "FIGURE_INSIDE" => s_FIGURE_INSIDE(v),
            "HEDGES" => s_HEDGES(v),
            "INCOME" => s_INCOME(v),
            "INCOME_MONTH" => s_INCOME_MONTH(v),
            "INCOME_PLAIN" => s_INCOME_PLAIN(v),
            "INCOME_WEEK" => s_INCOME_WEEK(v),
            "INCOME_YEAR" => s_INCOME_YEAR(v),
            "I_WEIGH" => s_I_WEIGH(v),
            "LEAD" => s_LEAD(v),
            "MONEY_DAY" => s_MONEY_DAY(v),
            "MONEY_MONTH" => s_MONEY_MONTH(v),
            "MONEY_NOUN" => s_MONEY_NOUN(v),
            "MONEY_PLAIN" => s_MONEY_PLAIN(v),
            "MONEY_WEEK" => s_MONEY_WEEK(v),
            "MONEY_YEAR" => s_MONEY_YEAR(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_UNIT_ITSELF" => s_NOT_A_UNIT_ITSELF(v),
            "NUMBERS" => s_NUMBERS(v),
            "ON_THE_TABLE_DISTANCE" => s_ON_THE_TABLE_DISTANCE(v),
            "ON_THE_TABLE_MONEY" => s_ON_THE_TABLE_MONEY(v),
            "PERIODS" => s_PERIODS(v),
            "PERIOD_NAMED" => s_PERIOD_NAMED(v),
            "PER_DAY" => s_PER_DAY(v),
            "POINTING_BACK" => s_POINTING_BACK(v),
            "SAVINGS" => s_SAVINGS(v),
            "SAVINGS_AFTER_A_RATE" => s_SAVINGS_AFTER_A_RATE(v),
            "SOMETHING_WEIGHED" => s_SOMETHING_WEIGHED(v),
            "SPAN_DAYS" => s_SPAN_DAYS(v),
            "SPAN_FRAME" => s_SPAN_FRAME(v),
            "SPAN_MONTHS" => s_SPAN_MONTHS(v),
            "SPAN_WEEKS" => s_SPAN_WEEKS(v),
            "STEPS" => s_STEPS(v),
            "SUBJECT" => s_SUBJECT(v),
            "SUM_IN_A_WORD" => s_SUM_IN_A_WORD(v),
            "TAIL" => s_TAIL(v),
            "TAKE" => s_TAKE(v),
            "TENS" => s_TENS(v),
            "TEN_TIMES" => s_TEN_TIMES(v),
            "THEIR_THING" => s_THEIR_THING(v),
            "THEY_HAVE_IT" => s_THEY_HAVE_IT(v),
            "THEY_HAVE_THE_TIME" => s_THEY_HAVE_THE_TIME(v),
            "THE_HOUSEHOLD" => s_THE_HOUSEHOLD(v),
            "TIME_A_DAY" => s_TIME_A_DAY(v),
            "TIME_BUDGET" => s_TIME_BUDGET(v),
            "TIME_LONG" => s_TIME_LONG(v),
            "TIME_TRIP" => s_TIME_TRIP(v),
            "TIME_WEEK" => s_TIME_WEEK(v),
            "TRIP" => s_TRIP(v),
            "UNITS" => s_UNITS(v),
            "UNIT_IS" => s_UNIT_IS(v),
            "UNIT_SAID" => s_UNIT_SAID(v),
            "WEIGHT_GOING_ON" => s_WEIGHT_GOING_ON(v),
            "WEIGHT_LOSS" => s_WEIGHT_LOSS(v),
            "WEIGHT_OF_A_THING" => s_WEIGHT_OF_A_THING(v),
            "WEIGHT_ON_THE_TABLE" => s_WEIGHT_ON_THE_TABLE(v),
            "WEIGHT_PLAIN" => s_WEIGHT_PLAIN(v),
            "WEIGHT_THEIR_OWN" => s_WEIGHT_THEIR_OWN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.quantity", module_obj);
    pyrt::xrefs::reg_fn("compose.quantity", "_number_at", py_func("_number_at", |args: &[Py]| _number_at(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_amount", py_func("_amount", |args: &[Py]| _amount(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_fmt", py_func("_fmt", |args: &[Py]| _fmt(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_minutes", py_func("_minutes", |args: &[Py]| _minutes(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_sum_inside_the_word", py_func("_the_sum_inside_the_word", |args: &[Py]| _the_sum_inside_the_word(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_money_words", py_func("_the_money_words", |args: &[Py]| _the_money_words(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_about_a_distance", py_func("_about_a_distance", |args: &[Py]| _about_a_distance(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_read", py_func("_read", |args: &[Py]| _read(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_q", py_func("_q", |args: &[Py]| _q(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_length_they_have_bounded", py_func("_a_length_they_have_bounded", |args: &[Py]| _a_length_they_have_bounded(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_bound_not_a_budget", py_func("_a_bound_not_a_budget", |args: &[Py]| _a_bound_not_a_budget(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_turn_above", py_func("_the_turn_above", |args: &[Py]| _the_turn_above(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_change_is_the_line", py_func("_the_change_is_the_line", |args: &[Py]| _the_change_is_the_line(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_weight_they_mean_themselves", py_func("_a_weight_they_mean_themselves", |args: &[Py]| _a_weight_they_mean_themselves(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_thing_was_named", py_func("_a_thing_was_named", |args: &[Py]| _a_thing_was_named(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_span_they_are_in", py_func("_a_span_they_are_in", |args: &[Py]| _a_span_they_are_in(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_count_the_question_asked_for", py_func("_a_count_the_question_asked_for", |args: &[Py]| _a_count_the_question_asked_for(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_fact_about_a_length", py_func("_a_fact_about_a_length", |args: &[Py]| _a_fact_about_a_length(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "the_thing_they_pay_for", py_func("the_thing_they_pay_for", |args: &[Py]| the_thing_they_pay_for(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "the_rate_they_asked_about", py_func("the_rate_they_asked_about", |args: &[Py]| the_rate_they_asked_about(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_without_a_pointing_tail", py_func("_without_a_pointing_tail", |args: &[Py]| _without_a_pointing_tail(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_without_a_hedge", py_func("_without_a_hedge", |args: &[Py]| _without_a_hedge(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_without_the_household", py_func("_without_the_household", |args: &[Py]| _without_the_household(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_a_take_without_home", py_func("_a_take_without_home", |args: &[Py]| _a_take_without_home(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_without_a_remark", py_func("_without_a_remark", |args: &[Py]| _without_a_remark(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_in_a_conversation", py_func("_in_a_conversation", |args: &[Py]| _in_a_conversation(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_not_short", py_func("_not_short", |args: &[Py]| _not_short(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_without_a_question", py_func("_without_a_question", |args: &[Py]| _without_a_question(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_say", py_func("_say", |args: &[Py]| _say(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_unit_of_one", py_func("_the_unit_of_one", |args: &[Py]| _the_unit_of_one(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_answer_range", py_func("_the_answer_range", |args: &[Py]| _the_answer_range(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_rule_said", py_func("_the_rule_said", |args: &[Py]| _the_rule_said(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "_the_family", py_func("_the_family", |args: &[Py]| _the_family(args)));
    pyrt::xrefs::reg_fn("compose.quantity", "the_figure_against_the_answer", py_func("the_figure_against_the_answer", |args: &[Py]| the_figure_against_the_answer(args)));
    pyrt::xrefs::reg_access("compose.quantity", "ABOUT_A_FIGURE", g_ABOUT_A_FIGURE, s_ABOUT_A_FIGURE);
    pyrt::xrefs::reg_access("compose.quantity", "AN_AGE", g_AN_AGE, s_AN_AGE);
    pyrt::xrefs::reg_access("compose.quantity", "AREA", g_AREA, s_AREA);
    pyrt::xrefs::reg_access("compose.quantity", "A_BOUND", g_A_BOUND, s_A_BOUND);
    pyrt::xrefs::reg_access("compose.quantity", "A_BUDGET_QUESTION", g_A_BUDGET_QUESTION, s_A_BUDGET_QUESTION);
    pyrt::xrefs::reg_access("compose.quantity", "A_BUDGET_THEY_SET", g_A_BUDGET_THEY_SET, s_A_BUDGET_THEY_SET);
    pyrt::xrefs::reg_access("compose.quantity", "A_CHANGE_IS_THIS_SMALL", g_A_CHANGE_IS_THIS_SMALL, s_A_CHANGE_IS_THIS_SMALL);
    pyrt::xrefs::reg_access("compose.quantity", "A_CHANGE_SAID", g_A_CHANGE_SAID, s_A_CHANGE_SAID);
    pyrt::xrefs::reg_access("compose.quantity", "A_COUNT_ASKED_FOR", g_A_COUNT_ASKED_FOR, s_A_COUNT_ASKED_FOR);
    pyrt::xrefs::reg_access("compose.quantity", "A_FIGURE_ALONE", g_A_FIGURE_ALONE, s_A_FIGURE_ALONE);
    pyrt::xrefs::reg_access("compose.quantity", "A_LIMIT_ON_A_LENGTH", g_A_LIMIT_ON_A_LENGTH, s_A_LIMIT_ON_A_LENGTH);
    pyrt::xrefs::reg_access("compose.quantity", "A_PRONOUN", g_A_PRONOUN, s_A_PRONOUN);
    pyrt::xrefs::reg_access("compose.quantity", "A_QUESTION_ABOUT_A_LENGTH", g_A_QUESTION_ABOUT_A_LENGTH, s_A_QUESTION_ABOUT_A_LENGTH);
    pyrt::xrefs::reg_access("compose.quantity", "A_RANGE_WITH_A_UNIT", g_A_RANGE_WITH_A_UNIT, s_A_RANGE_WITH_A_UNIT);
    pyrt::xrefs::reg_access("compose.quantity", "A_REMAINDER", g_A_REMAINDER, s_A_REMAINDER);
    pyrt::xrefs::reg_access("compose.quantity", "A_SINGLE_RULE", g_A_SINGLE_RULE, s_A_SINGLE_RULE);
    pyrt::xrefs::reg_access("compose.quantity", "A_STEP_EITHER_WAY", g_A_STEP_EITHER_WAY, s_A_STEP_EITHER_WAY);
    pyrt::xrefs::reg_access("compose.quantity", "A_STRETCH_THAT_IS_NOT_SHORT", g_A_STRETCH_THAT_IS_NOT_SHORT, s_A_STRETCH_THAT_IS_NOT_SHORT);
    pyrt::xrefs::reg_access("compose.quantity", "A_WAY_OF_PAYING", g_A_WAY_OF_PAYING, s_A_WAY_OF_PAYING);
    pyrt::xrefs::reg_access("compose.quantity", "A_WHAT_IS_LEFT", g_A_WHAT_IS_LEFT, s_A_WHAT_IS_LEFT);
    pyrt::xrefs::reg_access("compose.quantity", "BOUNDED_BY", g_BOUNDED_BY, s_BOUNDED_BY);
    pyrt::xrefs::reg_access("compose.quantity", "CURRENCY", g_CURRENCY, s_CURRENCY);
    pyrt::xrefs::reg_access("compose.quantity", "DISTANCE_DAY_OUT", g_DISTANCE_DAY_OUT, s_DISTANCE_DAY_OUT);
    pyrt::xrefs::reg_access("compose.quantity", "DISTANCE_FAR", g_DISTANCE_FAR, s_DISTANCE_FAR);
    pyrt::xrefs::reg_access("compose.quantity", "DISTANCE_FAR_TRIP", g_DISTANCE_FAR_TRIP, s_DISTANCE_FAR_TRIP);
    pyrt::xrefs::reg_access("compose.quantity", "DISTANCE_MID", g_DISTANCE_MID, s_DISTANCE_MID);
    pyrt::xrefs::reg_access("compose.quantity", "DISTANCE_NEAR", g_DISTANCE_NEAR, s_DISTANCE_NEAR);
    pyrt::xrefs::reg_access("compose.quantity", "DOUBT_ON_THE_FIGURE", g_DOUBT_ON_THE_FIGURE, s_DOUBT_ON_THE_FIGURE);
    pyrt::xrefs::reg_access("compose.quantity", "FIGURE_ABOVE", g_FIGURE_ABOVE, s_FIGURE_ABOVE);
    pyrt::xrefs::reg_access("compose.quantity", "FIGURE_BELOW", g_FIGURE_BELOW, s_FIGURE_BELOW);
    pyrt::xrefs::reg_access("compose.quantity", "FIGURE_INSIDE", g_FIGURE_INSIDE, s_FIGURE_INSIDE);
    pyrt::xrefs::reg_access("compose.quantity", "HEDGES", g_HEDGES, s_HEDGES);
    pyrt::xrefs::reg_access("compose.quantity", "INCOME", g_INCOME, s_INCOME);
    pyrt::xrefs::reg_access("compose.quantity", "INCOME_MONTH", g_INCOME_MONTH, s_INCOME_MONTH);
    pyrt::xrefs::reg_access("compose.quantity", "INCOME_PLAIN", g_INCOME_PLAIN, s_INCOME_PLAIN);
    pyrt::xrefs::reg_access("compose.quantity", "INCOME_WEEK", g_INCOME_WEEK, s_INCOME_WEEK);
    pyrt::xrefs::reg_access("compose.quantity", "INCOME_YEAR", g_INCOME_YEAR, s_INCOME_YEAR);
    pyrt::xrefs::reg_access("compose.quantity", "I_WEIGH", g_I_WEIGH, s_I_WEIGH);
    pyrt::xrefs::reg_access("compose.quantity", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_DAY", g_MONEY_DAY, s_MONEY_DAY);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_MONTH", g_MONEY_MONTH, s_MONEY_MONTH);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_NOUN", g_MONEY_NOUN, s_MONEY_NOUN);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_PLAIN", g_MONEY_PLAIN, s_MONEY_PLAIN);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_WEEK", g_MONEY_WEEK, s_MONEY_WEEK);
    pyrt::xrefs::reg_access("compose.quantity", "MONEY_YEAR", g_MONEY_YEAR, s_MONEY_YEAR);
    pyrt::xrefs::reg_access("compose.quantity", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.quantity", "NOT_A_UNIT_ITSELF", g_NOT_A_UNIT_ITSELF, s_NOT_A_UNIT_ITSELF);
    pyrt::xrefs::reg_access("compose.quantity", "NUMBERS", g_NUMBERS, s_NUMBERS);
    pyrt::xrefs::reg_access("compose.quantity", "ON_THE_TABLE_DISTANCE", g_ON_THE_TABLE_DISTANCE, s_ON_THE_TABLE_DISTANCE);
    pyrt::xrefs::reg_access("compose.quantity", "ON_THE_TABLE_MONEY", g_ON_THE_TABLE_MONEY, s_ON_THE_TABLE_MONEY);
    pyrt::xrefs::reg_access("compose.quantity", "PERIODS", g_PERIODS, s_PERIODS);
    pyrt::xrefs::reg_access("compose.quantity", "PERIOD_NAMED", g_PERIOD_NAMED, s_PERIOD_NAMED);
    pyrt::xrefs::reg_access("compose.quantity", "PER_DAY", g_PER_DAY, s_PER_DAY);
    pyrt::xrefs::reg_access("compose.quantity", "POINTING_BACK", g_POINTING_BACK, s_POINTING_BACK);
    pyrt::xrefs::reg_access("compose.quantity", "SAVINGS", g_SAVINGS, s_SAVINGS);
    pyrt::xrefs::reg_access("compose.quantity", "SAVINGS_AFTER_A_RATE", g_SAVINGS_AFTER_A_RATE, s_SAVINGS_AFTER_A_RATE);
    pyrt::xrefs::reg_access("compose.quantity", "SOMETHING_WEIGHED", g_SOMETHING_WEIGHED, s_SOMETHING_WEIGHED);
    pyrt::xrefs::reg_access("compose.quantity", "SPAN_DAYS", g_SPAN_DAYS, s_SPAN_DAYS);
    pyrt::xrefs::reg_access("compose.quantity", "SPAN_FRAME", g_SPAN_FRAME, s_SPAN_FRAME);
    pyrt::xrefs::reg_access("compose.quantity", "SPAN_MONTHS", g_SPAN_MONTHS, s_SPAN_MONTHS);
    pyrt::xrefs::reg_access("compose.quantity", "SPAN_WEEKS", g_SPAN_WEEKS, s_SPAN_WEEKS);
    pyrt::xrefs::reg_access("compose.quantity", "STEPS", g_STEPS, s_STEPS);
    pyrt::xrefs::reg_access("compose.quantity", "SUBJECT", g_SUBJECT, s_SUBJECT);
    pyrt::xrefs::reg_access("compose.quantity", "SUM_IN_A_WORD", g_SUM_IN_A_WORD, s_SUM_IN_A_WORD);
    pyrt::xrefs::reg_access("compose.quantity", "TAIL", g_TAIL, s_TAIL);
    pyrt::xrefs::reg_access("compose.quantity", "TAKE", g_TAKE, s_TAKE);
    pyrt::xrefs::reg_access("compose.quantity", "TENS", g_TENS, s_TENS);
    pyrt::xrefs::reg_access("compose.quantity", "TEN_TIMES", g_TEN_TIMES, s_TEN_TIMES);
    pyrt::xrefs::reg_access("compose.quantity", "THEIR_THING", g_THEIR_THING, s_THEIR_THING);
    pyrt::xrefs::reg_access("compose.quantity", "THEY_HAVE_IT", g_THEY_HAVE_IT, s_THEY_HAVE_IT);
    pyrt::xrefs::reg_access("compose.quantity", "THEY_HAVE_THE_TIME", g_THEY_HAVE_THE_TIME, s_THEY_HAVE_THE_TIME);
    pyrt::xrefs::reg_access("compose.quantity", "THE_HOUSEHOLD", g_THE_HOUSEHOLD, s_THE_HOUSEHOLD);
    pyrt::xrefs::reg_access("compose.quantity", "TIME_A_DAY", g_TIME_A_DAY, s_TIME_A_DAY);
    pyrt::xrefs::reg_access("compose.quantity", "TIME_BUDGET", g_TIME_BUDGET, s_TIME_BUDGET);
    pyrt::xrefs::reg_access("compose.quantity", "TIME_LONG", g_TIME_LONG, s_TIME_LONG);
    pyrt::xrefs::reg_access("compose.quantity", "TIME_TRIP", g_TIME_TRIP, s_TIME_TRIP);
    pyrt::xrefs::reg_access("compose.quantity", "TIME_WEEK", g_TIME_WEEK, s_TIME_WEEK);
    pyrt::xrefs::reg_access("compose.quantity", "TRIP", g_TRIP, s_TRIP);
    pyrt::xrefs::reg_access("compose.quantity", "UNITS", g_UNITS, s_UNITS);
    pyrt::xrefs::reg_access("compose.quantity", "UNIT_IS", g_UNIT_IS, s_UNIT_IS);
    pyrt::xrefs::reg_access("compose.quantity", "UNIT_SAID", g_UNIT_SAID, s_UNIT_SAID);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_GOING_ON", g_WEIGHT_GOING_ON, s_WEIGHT_GOING_ON);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_LOSS", g_WEIGHT_LOSS, s_WEIGHT_LOSS);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_OF_A_THING", g_WEIGHT_OF_A_THING, s_WEIGHT_OF_A_THING);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_ON_THE_TABLE", g_WEIGHT_ON_THE_TABLE, s_WEIGHT_ON_THE_TABLE);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_PLAIN", g_WEIGHT_PLAIN, s_WEIGHT_PLAIN);
    pyrt::xrefs::reg_access("compose.quantity", "WEIGHT_THEIR_OWN", g_WEIGHT_THEIR_OWN, s_WEIGHT_THEIR_OWN);
}

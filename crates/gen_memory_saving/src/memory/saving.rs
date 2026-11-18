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


thread_local! { static G_ADDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIM_AT_A_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_A_LENGTH_FOR_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_HOW_MUCH_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_IF_ENOUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CAN_CARRY_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CORRECTION_AND_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOAL_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOAL_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_WAS_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LONE_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONTH_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MORE_LIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_THEN_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_ANY_OTHER_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RATE_WITH_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RESTATED_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPENT_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUM_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRAILING_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEEK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEEK_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_END_OF_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY_A_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_EDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_END_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_END_OF_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FURTHER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELD_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_FAR_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_LONG_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_LONG_UNTIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS_IN_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEED_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_POT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_POT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTGOING_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_SAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POT_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POT_REPORTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVING_CLOSERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THING_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOGETHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_AT_THE_GOAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KNOWN_TO_THE_MONEY_ENTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_FIGURE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_END_OF_A_SEASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.saving: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ADDED() -> Py { ensure_init(); G_ADDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADDED(v: Py) { ensure_init(); G_ADDED.with(|c| *c.borrow_mut() = v); }
pub fn g_AIM_AT_A_SUM() -> Py { ensure_init(); G_AIM_AT_A_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIM_AT_A_SUM(v: Py) { ensure_init(); G_AIM_AT_A_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY() -> Py { ensure_init(); G_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY(v: Py) { ensure_init(); G_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_AFTER() -> Py { ensure_init(); G_ALREADY_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_AFTER(v: Py) { ensure_init(); G_ALREADY_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_HELD() -> Py { ensure_init(); G_ALREADY_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_HELD(v: Py) { ensure_init(); G_ALREADY_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_SUM() -> Py { ensure_init(); G_ANOTHER_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_SUM(v: Py) { ensure_init(); G_ANOTHER_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_LENGTH() -> Py { ensure_init(); G_ASKS_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_LENGTH(v: Py) { ensure_init(); G_ASKS_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_LENGTH_FOR_A_THING() -> Py { ensure_init(); G_ASKS_A_LENGTH_FOR_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_LENGTH_FOR_A_THING(v: Py) { ensure_init(); G_ASKS_A_LENGTH_FOR_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_HOW_MUCH_MORE() -> Py { ensure_init(); G_ASKS_HOW_MUCH_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_HOW_MUCH_MORE(v: Py) { ensure_init(); G_ASKS_HOW_MUCH_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_IF_ENOUGH() -> Py { ensure_init(); G_ASKS_IF_ENOUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_IF_ENOUGH(v: Py) { ensure_init(); G_ASKS_IF_ENOUGH.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_A_FIGURE() -> Py { ensure_init(); G_AT_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_A_FIGURE(v: Py) { ensure_init(); G_AT_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CAN_CARRY_IT() -> Py { ensure_init(); G_A_CAN_CARRY_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CAN_CARRY_IT(v: Py) { ensure_init(); G_A_CAN_CARRY_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CORRECTION_AND_A_FIGURE() -> Py { ensure_init(); G_A_CORRECTION_AND_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION_AND_A_FIGURE(v: Py) { ensure_init(); G_A_CORRECTION_AND_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOAL_STATED() -> Py { ensure_init(); G_A_GOAL_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOAL_STATED(v: Py) { ensure_init(); G_A_GOAL_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOAL_SUM() -> Py { ensure_init(); G_A_GOAL_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOAL_SUM(v: Py) { ensure_init(); G_A_GOAL_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_SAID() -> Py { ensure_init(); G_A_LENGTH_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_SAID(v: Py) { ensure_init(); G_A_LENGTH_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_WAS_ASKED() -> Py { ensure_init(); G_A_LENGTH_WAS_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_WAS_ASKED(v: Py) { ensure_init(); G_A_LENGTH_WAS_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LONE_SUM() -> Py { ensure_init(); G_A_LONE_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LONE_SUM(v: Py) { ensure_init(); G_A_LONE_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONTH_FIGURE() -> Py { ensure_init(); G_A_MONTH_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONTH_FIGURE(v: Py) { ensure_init(); G_A_MONTH_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MORE_LIKE() -> Py { ensure_init(); G_A_MORE_LIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MORE_LIKE(v: Py) { ensure_init(); G_A_MORE_LIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_RATE() -> Py { ensure_init(); G_A_NEW_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_RATE(v: Py) { ensure_init(); G_A_NEW_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_THEN_A_RATE() -> Py { ensure_init(); G_A_NO_THEN_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_THEN_A_RATE(v: Py) { ensure_init(); G_A_NO_THEN_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE() -> Py { ensure_init(); G_A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE(v: Py) { ensure_init(); G_A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_ANY_OTHER_PERIOD() -> Py { ensure_init(); G_A_RATE_ANY_OTHER_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_ANY_OTHER_PERIOD(v: Py) { ensure_init(); G_A_RATE_ANY_OTHER_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_PUT_RIGHT() -> Py { ensure_init(); G_A_RATE_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_PUT_RIGHT(v: Py) { ensure_init(); G_A_RATE_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RATE_WITH_A_VERB() -> Py { ensure_init(); G_A_RATE_WITH_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RATE_WITH_A_VERB(v: Py) { ensure_init(); G_A_RATE_WITH_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RESTATED_RATE() -> Py { ensure_init(); G_A_RESTATED_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RESTATED_RATE(v: Py) { ensure_init(); G_A_RESTATED_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPENT_ON() -> Py { ensure_init(); G_A_SPENT_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPENT_ON(v: Py) { ensure_init(); G_A_SPENT_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUM_A_MONTH() -> Py { ensure_init(); G_A_SUM_A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUM_A_MONTH(v: Py) { ensure_init(); G_A_SUM_A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED() -> Py { ensure_init(); G_A_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED(v: Py) { ensure_init(); G_A_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRAILING_SUM() -> Py { ensure_init(); G_A_TRAILING_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRAILING_SUM(v: Py) { ensure_init(); G_A_TRAILING_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEEK_ASK() -> Py { ensure_init(); G_A_WEEK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEEK_ASK(v: Py) { ensure_init(); G_A_WEEK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEEK_FIGURE() -> Py { ensure_init(); G_A_WEEK_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEEK_FIGURE(v: Py) { ensure_init(); G_A_WEEK_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DATE() -> Py { ensure_init(); G_BARE_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DATE(v: Py) { ensure_init(); G_BARE_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_END_OF_RELATIVE() -> Py { ensure_init(); G_BARE_END_OF_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_END_OF_RELATIVE(v: Py) { ensure_init(); G_BARE_END_OF_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FIGURE() -> Py { ensure_init(); G_BARE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FIGURE(v: Py) { ensure_init(); G_BARE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_RATE() -> Py { ensure_init(); G_BARE_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_RATE(v: Py) { ensure_init(); G_BARE_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BY_A_DATE() -> Py { ensure_init(); G_BY_A_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY_A_DATE(v: Py) { ensure_init(); G_BY_A_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_EDGE() -> Py { ensure_init(); G_CLAUSE_EDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_EDGE(v: Py) { ensure_init(); G_CLAUSE_EDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONAL() -> Py { ensure_init(); G_CONDITIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL(v: Py) { ensure_init(); G_CONDITIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING() -> Py { ensure_init(); G_CORRECTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); G_CORRECTING.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE() -> Py { ensure_init(); G_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE(v: Py) { ensure_init(); G_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_END_OF() -> Py { ensure_init(); G_END_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_END_OF(v: Py) { ensure_init(); G_END_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_END_OF_RELATIVE() -> Py { ensure_init(); G_END_OF_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_END_OF_RELATIVE(v: Py) { ensure_init(); G_END_OF_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER_LEAD() -> Py { ensure_init(); G_FILLER_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER_LEAD(v: Py) { ensure_init(); G_FILLER_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_FURTHER_SAID() -> Py { ensure_init(); G_FURTHER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FURTHER_SAID(v: Py) { ensure_init(); G_FURTHER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD() -> Py { ensure_init(); G_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD(v: Py) { ensure_init(); G_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_HELD_SAID() -> Py { ensure_init(); G_HELD_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELD_SAID(v: Py) { ensure_init(); G_HELD_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_FAR_OFF() -> Py { ensure_init(); G_HOW_FAR_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_FAR_OFF(v: Py) { ensure_init(); G_HOW_FAR_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_LONG_ASK() -> Py { ensure_init(); G_HOW_LONG_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_ASK(v: Py) { ensure_init(); G_HOW_LONG_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_LONG_UNTIL() -> Py { ensure_init(); G_HOW_LONG_UNTIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_UNTIL(v: Py) { ensure_init(); G_HOW_LONG_UNTIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MUCH() -> Py { ensure_init(); G_HOW_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MUCH(v: Py) { ensure_init(); G_HOW_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME_FRAME() -> Py { ensure_init(); G_INCOME_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME_FRAME(v: Py) { ensure_init(); G_INCOME_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_INTENT() -> Py { ensure_init(); G_INTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTENT(v: Py) { ensure_init(); G_INTENT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_OUT() -> Py { ensure_init(); G_MONEY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_OUT(v: Py) { ensure_init(); G_MONEY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS_IN_A() -> Py { ensure_init(); G_MONTHS_IN_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS_IN_A(v: Py) { ensure_init(); G_MONTHS_IN_A.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_WHEN() -> Py { ensure_init(); G_NAMED_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_WHEN(v: Py) { ensure_init(); G_NAMED_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NEED_A_FIGURE() -> Py { ensure_init(); G_NEED_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEED_A_FIGURE(v: Py) { ensure_init(); G_NEED_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_POT() -> Py { ensure_init(); G_NOT_A_POT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_POT(v: Py) { ensure_init(); G_NOT_A_POT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PRICE() -> Py { ensure_init(); G_NOT_A_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PRICE(v: Py) { ensure_init(); G_NOT_A_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_POT() -> Py { ensure_init(); G_OTHER_POT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_POT(v: Py) { ensure_init(); G_OTHER_POT.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTGOING_FRAME() -> Py { ensure_init(); G_OUTGOING_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTGOING_FRAME(v: Py) { ensure_init(); G_OUTGOING_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_SAVING() -> Py { ensure_init(); G_PAST_SAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_SAVING(v: Py) { ensure_init(); G_PAST_SAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_POT_OPENER() -> Py { ensure_init(); G_POT_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POT_OPENER(v: Py) { ensure_init(); G_POT_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_POT_REPORTED() -> Py { ensure_init(); G_POT_REPORTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POT_REPORTED(v: Py) { ensure_init(); G_POT_REPORTED.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_WORDS() -> Py { ensure_init(); G_PRICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_WORDS(v: Py) { ensure_init(); G_PRICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_BY() -> Py { ensure_init(); G_PUT_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_BY(v: Py) { ensure_init(); G_PUT_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_GIVEN() -> Py { ensure_init(); G_RATE_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_GIVEN(v: Py) { ensure_init(); G_RATE_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_RATE_VERB() -> Py { ensure_init(); G_RATE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATE_VERB(v: Py) { ensure_init(); G_RATE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVE_VERB() -> Py { ensure_init(); G_SAVE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVE_VERB(v: Py) { ensure_init(); G_SAVE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVING() -> Py { ensure_init(); G_SAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVING(v: Py) { ensure_init(); G_SAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVING_CLOSERS() -> Py { ensure_init(); G_SAVING_CLOSERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVING_CLOSERS(v: Py) { ensure_init(); G_SAVING_CLOSERS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM() -> Py { ensure_init(); G_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM(v: Py) { ensure_init(); G_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THING_WANTED() -> Py { ensure_init(); G_THE_THING_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THING_WANTED(v: Py) { ensure_init(); G_THE_THING_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g_TOGETHER() -> Py { ensure_init(); G_TOGETHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOGETHER(v: Py) { ensure_init(); G_TOGETHER.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_AT_THE_GOAL() -> Py { ensure_init(); G_WHEN_AT_THE_GOAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_AT_THE_GOAL(v: Py) { ensure_init(); G_WHEN_AT_THE_GOAL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); G_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); G_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__KNOWN_TO_THE_MONEY_ENTRY() -> Py { ensure_init(); G__KNOWN_TO_THE_MONEY_ENTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KNOWN_TO_THE_MONEY_ENTRY(v: Py) { ensure_init(); G__KNOWN_TO_THE_MONEY_ENTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_RATE() -> Py { ensure_init(); G__NOT_THE_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_RATE(v: Py) { ensure_init(); G__NOT_THE_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_FIGURE_WORDS() -> Py { ensure_init(); G__OTHER_FIGURE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_FIGURE_WORDS(v: Py) { ensure_init(); G__OTHER_FIGURE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_END_OF_A_SEASON() -> Py { ensure_init(); G__THE_END_OF_A_SEASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_END_OF_A_SEASON(v: Py) { ensure_init(); G__THE_END_OF_A_SEASON.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A saving goal carries its own arithmetic, and the sum is a follow-up waiting to happen.\n\nPeople say the goal before they say the date: \"i want to save 2000 pounds\" and then \"by june\".\nBoth lines on their own name nothing to work out - a goal is a statement, and a date is a date -\nso neither reached the money entry, and the chat module answered the goal with \"start with the\nsmallest version of it, not the whole thing\", which is advice about a habit and says nothing\nabout money; the date after it got \"got it - is there anything you'd like a hand with\".\n\nThe two lines are one question, and the knowledge base already knows how to answer it: the sum,\nthe date and the weeks left give the weekly figure. This module reads that question:\n\n* a saving goal with a sum in it is that question, with the date if the line carries one;\n* a line that is only a date (\"by june\", \"actually make that by april\") is the same question\n  about the sum the person named a turn or two ago;\n* nothing else is a saving question, so every other line passes.\n\nWhat it hands back is a plain question in the person's own terms, which `bot/modules/answers.py`\nanswers from `bot/knowledge/money.py`.\n");
    s_SAVING(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|savings|saving up|putting by|putting away|putting\\b[^.?]{0,18}?\\b(?:by|away|aside)|put\\b[^.?]{0,18}?\\b(?:by|away|aside)|setting aside|stashing|put by|put away|set aside|stash|pot|fund)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUM(stdlib_call("re", "compile", &[py_str("(?:£|\\$|\\u20ac)\\s*(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?|\\b(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand|pounds|quid|euros|dollars|bucks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_POT(stdlib_call("re", "compile", &[py_str("\\s*(?:on|off|from|for)\\s+(?:my|the|our|a|an|each|every|it)\\b|\\s*a\\s+(?:month|week|year)\\s+(?:on|off|from)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(\\d{3,7})\\b(?!\\s*(?:k|grand|thousand|pounds|quid|euros|dollars|bucks)\\b)(?!\\s*(?:a|per|each)\\s+(?:month|week|year|fortnight))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS(py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")]));
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![py_str("christmas"), py_str("december")]), py_tuple(vec![py_str("xmas"), py_str("december")]), py_tuple(vec![py_str("new year"), py_str("december")]), py_tuple(vec![py_str("hogmanay"), py_str("december")]), py_tuple(vec![py_str("halloween"), py_str("october")]), py_tuple(vec![py_str("valentine"), py_str("february")]), py_tuple(vec![py_str("easter"), py_str("april")]), py_tuple(vec![py_str("black friday"), py_str("november")]), py_tuple(vec![py_str("spring"), py_str("march")]), py_tuple(vec![py_str("summer"), py_str("june")]), py_tuple(vec![py_str("autumn"), py_str("september")]), py_tuple(vec![py_str("fall"), py_str("october")]), py_tuple(vec![py_str("winter"), py_str("december")]), py_tuple(vec![py_str("the year"), py_str("december")])]) }
    s_NAMED_WHEN(__data1());
    s_WHEN_WORDS(op_add(&op_add(&mcall(&py_str("|"), "join", &[g_MONTHS()]), &py_str("|")), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NAMED_WHEN()) { let __unpk5840 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk5840[0].clone(); let mut _month: Py = __unpk5840[1].clone(); __out.push(word.clone()); } py_list(__out) })() }])));
    s_END_OF(py_str("(?:(?:at\\s+)?the\\s+end\\s+of\\s+|end\\s+of\\s+)?"));
    s_DATE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:by|before|until|till|for|in|at the end of|end of)\\s+"), &g_END_OF()), &py_str("(?:next\\s+|this\\s+|early\\s+|late\\s+)?(")), &g_WHEN_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTING(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:actually|no|sorry|nope|rather|instead|oh)\\b[\\s,]*(?:(?:its|it's|it\\s+is|it\\s+was|thats|that's|that\\s+is)\\b[\\s,]*)?|(?:make|change)\\s+(?:that|it)\\s+(?:to\\s+)?|i\\s+mean\\b[\\s,]*)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_END_OF_RELATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:by|before|until|till|in time for|for|in|at the end of|end of)\\s+(?:at\\s+)?the\\s+end\\s+of\\s+(next\\s+month|this\\s+month|next\\s+week|this\\s+week|next\\s+year|this\\s+year|(?:the\\s+)?(?:summer|winter|spring|autumn|fall))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_END_OF_RELATIVE(stdlib_call("re", "compile", &[py_str("^\\s*(?:by|before|until|till|for|in)?\\s*(?:at\\s+)?the\\s+end\\s+of\\s+(next\\s+month|this\\s+month|next\\s+week|this\\s+week|next\\s+year|this\\s+year)\\s*[.!,]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DATE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:by|before|until|till|for|in)?\\s*(?:at\\s+|the\\s+)?(?:end\\s+of\\s+)?(?:next\\s+|this\\s+|early\\s+|late\\s+)?("), &g_WHEN_WORDS()), &py_str(")\\s*[.!,]?\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY(stdlib_call("re", "compile", &[py_str("\\b(?:already|so far|to date|up to now|last (?:month|year|week)|this (?:month|year|week))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INTENT(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanted|need|needs|needed|hope|hoping|plan|planning|aim|aiming|going to|gonna|would like|like to|love to|trying to|goal|target)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_SAVING(stdlib_call("re", "compile", &[py_str("\\b(?:saved|have\\s+saved|had\\s+saved|managed\\s+to\\s+save)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELD(stdlib_call("re", "compile", &[py_str("\\b(?:in|into)\\s+(?:(?:my|the|our|a|an)\\s+)?(?:savings?|pot|fund|account)\\b|\\bin\\s+it\\b|\\b(?:savings?|pot|fund)\\b[^0-9]{0,14}\\b(?:is|are|was|were|has|have|holds|sits\\s+at|stood\\s+at|sat\\s+at|of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TOGETHER(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?\\s*(?:pounds?|quid)?\\s*(?:on\\s+top\\s+of|plus|alongside|as\\s+well\\s+as|and\\s+another|together\\s+with)\\s*(?:the\\s+|my\\s+|our\\s+)?(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELD_SAID(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?\\s*(?:pounds?|quid)?[^.!?]{0,40}?\\b(?:saved|put\\s+by|set\\s+aside|put\\s+aside|in\\s+savings|in\\s+the\\s+pot)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FURTHER_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:another|a\\s+further|an\\s+extra|extra|more)\\b\\s*(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POT_REPORTED(stdlib_call("re", "compile", &[py_str("\\b(?:this\\s+(?:month|year|week)|last\\s+(?:month|year|week)|so\\s+far|to\\s+date)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_HELD(stdlib_call("re", "compile", &[py_str("\\b(?:already|so far)\\b[^.!?]{0,24}?\\b(?:have|has|got|hold|put)\\b\\s*(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_AFTER(stdlib_call("re", "compile", &[py_str("\\b(?:have|has|got|hold)\\b\\s*(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?\\s*(?:pounds?|quid)?\\s+(?:already|so far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AIM_AT_A_SUM(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanting|need|needs|hope|hoping|goal|aim|aiming|plan|planning|trying|like)\\b[^.!?]{0,30}?\\b(?:have|save|saved|set\\s+aside|put\\s+by|reach|get\\s+to|afford)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BY_A_DATE(stdlib_call("re", "compile", &[py_str("\\bby\\s+(?:next\\b|the\\s+end\\b|\\d|january|february|march|april|may|june|july|august|september|october|november|december|christmas|spring|summer|autumn|winter|xmas|easter|payday|then\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:im|i'?m|i\\s+am|we'?re|we\\s+are|ive|i'?ve|i\\s+have|we'?ve|we\\s+have)\\s+(?:now\\s+|only\\s+|just\\s+|already\\s+)?(?:on|at|up\\s+to|got(?:\\s+to)?|onto|standing\\s+at)\\s+(?:about\\s+|around\\s+|nearly\\s+|just\\s+)?(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:a|per|each|every)\\s+(?:month|week|year|fortnight|pay\\s*day|payday|payslip)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_FAR_OFF(stdlib_call("re", "compile", &[py_str("\\bhow\\s+far\\s+(?:off|away)\\b|\\bhow\\s+much\\s+(?:more\\s+)?(?:do\\s+i\\s+need|have\\s+i\\s+(?:got\\s+)?left|is\\s+left)\\b|\\bhow\\s+much\\s+(?:more|left)\\s+to\\s+go\\b|\\bwhat'?s\\s+left\\s+(?:to\\s+go|on\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEED_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|need\\s+to\\s+(?:get\\s+to|reach|have|save|find)|have\\s+to\\s+(?:find|reach|save)|should\\s+have|aiming\\s+for|target\\s+is)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POT_OPENER(stdlib_call("re", "compile", &[py_str("^For\\s+£\\s*[\\d,]+:\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_LONG_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|then|oh|right|yeah)\\W+)*(?:how\\s+long(?:\\s+(?:will|would|does|do|did|is|to)\\s+(?:that|it|this)(?:\\s+(?:take|be))?(?:\\s+to\\s+(?:save|get)(?:\\s+(?:that|it|there|the\\s+money))?)?)?|how\\s+many\\s+(?:months|years|weeks)(?:\\s+is\\s+(?:that|it|this))?|how\\s+much\\s+time(?:\\s+(?:will|would|does)\\s+(?:that|it|this)\\s+take)?)\\s*[?!.]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_LONG_UNTIL(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|then|oh|right|yeah)\\W+)*how\\s+long\\s+(?:until|till|til|before)\\s+(?:i|we|you|it|its|it's|that|this|there)\\b|^\\W*(?:(?:and|but|so|ok|okay|then|oh|right|yeah)\\W+)*how\\s+long\\s+to\\s+(?:save|have|get|reach|afford|be there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_AT_THE_GOAL(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|then|oh|right|yeah)\\W+)*when\\s+(?:will|would|can|could|should|might)\\s+(?:i|we|you|it|its|it's|that|this)\\s+(?:have|get|be|reach|afford|save|hit|make|manage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|costs?|cost|about|around|at|for|goes for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRAILING_SUM(stdlib_call("re", "compile", &[py_str(",\\s*£?\\s*\\d")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:earn\\w*|salary|wages?|income|payslip|paid|pension|before tax|after tax|take home|i'?m\\s+\\d+|i am\\s+\\d+|how old)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOAL_STATED(stdlib_call("re", "compile", &[py_str("(?:\\bfor\\s+)?£\\s*(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k)?\\s*(?::|\\bat\\s+£|\\bis\\s+(?:about\\s+)?\\d)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_WAS_ASKED(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b|\\bhow\\s+many\\s+(?:months|years|weeks)\\b|\\bhow\\s+much\\s+time\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:in|within|over|inside)\\s+(?:the\\s+next\\s+|the\\s+coming\\s+)?(?:\\d[\\d,]*|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|eighteen|twenty[-\\s]?four|a\\s+couple\\s+of|a\\s+few)\\s*(?:months?|years?|weeks?)\\b|\\b(?:the\\s+)?end\\s+of\\s+(?:the\\s+)?(?:month|year|week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_SUM(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay)\\s+)?(?:(?:what|how)\\s+about|and)\\s+(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand|pounds?|quid)?\\s*[?!.]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOAL_SUM(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay)\\b[\\s,]*)*(?:i(?:'d|'ll)?\\s+)?(?:will\\s+|just\\s+|only\\s+|need\\s+to\\s+|want\\s+to\\s+)?sav(?:e|es|ing)\\s+(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand|pounds?|quid)?\\s*[?!.]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?\\s+)?(?:[a-z]+\\s+){0,2}?(?:a|per|each)\\s+month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_ANY_OTHER_PERIOD(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?\\s+)?(?:[a-z]+\\s+){0,2}?(?:a|per|each)\\s+(week|fortnight|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS_IN_A(py_dict(vec![(py_str("week"), op_truediv(&Py::Int(12), &Py::Float(52.0))), (py_str("fortnight"), op_truediv(&Py::Int(12), &Py::Float(26.0))), (py_str("month"), Py::Float(1.0)), (py_str("year"), Py::Float(12.0))]));
    s_INCOME_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:earn|earns|earning|earned|paid|salary|wages?|income|take[- ]home|brings?\\s+in|pension|benefits?|allowance)\\b|\\b(?:i|we)\\s+(?:get|make|bring)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OUTGOING_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:rent|mortgage|bills?|childcare|nurser(?:y|ies)|loan|debt|overdraft|council\\s+tax|energy|electric(?:ity)?|gas|heating|insurance|commut(?:e|ing)|transport|travel|groceries|food\\s+shop|spend(?:ing)?|goes\\s+on|paying\\s+out|outgoings|repayments?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_EDGE(stdlib_call("re", "compile", &[py_str("[,;.]|\\band\\b|\\bbut\\b|\\bso\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_PUT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:actually|sorry|i meant|meant|make it|make that|no wait|rather than|instead|not)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CAN_CARRY_IT(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|able|afford|manage|put|save|saving|spare|stretch)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUM_A_MONTH(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?\\s+)?(?:a|per|each)\\s+month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OTHER_FIGURE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|needed|owe|owes|owed|debt|debts|cost|costs|spend|spent|spending|bill|bills|rent|goal|target|total|altogether|towards|toward|week|weeks|day|days|year|years)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LONE_SUM(stdlib_call("re", "compile", &[py_str("(?<![\\d.,£$\\u20ac])(\\d[\\d,]*)(?![\\d.,])")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION_AND_A_FIGURE(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:actually|sorry|no|nope|wait|oh|well|ok|okay|right|hang\\s+on)\\b\\W*){0,3}(?:(?:it|its|it's|that|thats|that's|make\\s+it|make\\s+that|i\\s+meant|meant)\\b\\W*){0,2}(?:£\\s*)?\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:on|spent|spending|cost|costs|bill|bills|balance|owe|owes|owed|debt|debts|rent|price|paid|pay|total|goal|target|week|weeks|day|days|year|years|times?|hours?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NO_THEN_A_RATE(stdlib_call("re", "compile", &[py_str("^\\s*(?:no|nope|nah)\\b[\\s,.!\\u2014-]*(?:i\\s+(?:can|could)\\s+(?:do|manage|put|save|spare)\\s+)?(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?\\s+)?(?:a|per|each)\\s+month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPENT_ON(stdlib_call("re", "compile", &[py_str("\\b(?:on|spent|spending|costs?|bills?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MORE_LIKE(stdlib_call("re", "compile", &[py_str("\\b(?:more\\s+like|closer\\s+to|call\\s+it)\\s*(?:£\\s*)?\\d|\\b(?:it'?s|its|that'?s)\\s+(?:(?:more\\s+like|about|only)\\s+)?(?:£\\s*)?\\d[\\d,]*\\s+now\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MUCH(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many)\\b[^?.!]{0,24}?\\b(?:a|per|each)\\s+(?:month|week)\\b|\\bhow\\s+much\\s+(?:should|do|would)\\s+i\\s+(?:put|save|need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WEEK_FIGURE(stdlib_call("re", "compile", &[py_str("£\\s*(\\d[\\d,]*)\\s*a\\s+week"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_IF_ENOUGH(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|ok|okay|right|well)\\b[\\s,]*)*(?:is|will|would|does|do|am\\s+i|are\\s+we)\\s+(?:that|this|it|they|we|i|£?\\s*\\d[\\d,]*\\s*(?:a|per|each)\\s+month|(?:putting\\s+by|saving|saved)\\s+£?\\s*\\d[\\d,]*\\s*(?:a|per|each)\\s+month)\\s+(?:be\\s+)?(?:enough|do(?:\\s+it)?|get\\s+(?:me|us)\\s+there|get\\s+there|cover(?:\\s+it)?|be\\s+enough)(?:\\s+(?:then|though|really|now|do\\s+you\\s+think|i\\s+wonder))?\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_HOW_MUCH_MORE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|ok|okay|right|well)\\b[\\s,]*)*(?:how\\s+much\\s+(?:more|extra)\\s+(?:a|per|each)\\s+month|how\\s+much\\s+(?:more|extra)\\s+(?:would|do|will)\\s+(?:i|we)\\s+(?:need|have\\s+to))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEW_RATE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay|right)\\b[\\s,]*)*(?:(?:what|how)\\s+about\\s+|at\\s+|make\\s+it\\s+)?(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k)?\\s*(?:pounds?\\s+)?(?:a|per|each)\\s+month\\s*(?:then|now)?\\s*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_WITH_A_VERB(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay|right)\\b[\\s,]*)*(?:(?:what|how)\\s+about\\s+|what\\s+if\\s+(?:i\\s+)?|(?:and\\s+)?if\\s+i\\s+)?(?:only\\s+)?(?:put|putting|set|setting|pay|paying)\\s+(?:(?:away|by|aside|in|up)\\s*)?(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k)?\\s*(?:pounds?|quid)?\\s*(?:(?:instead|then|now)\\b\\s*)?[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b|\\bhow\\s+many\\s+(?:months|years|weeks)\\b|\\bwhen\\s+(?:will|would|can|do|should)\\s+(?:i|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__KNOWN_TO_THE_MONEY_ENTRY(py_tuple(vec![py_str("christmas"), py_str("xmas")]));
    s__THE_END_OF_A_SEASON(py_dict(vec![(py_str("spring"), py_str("may")), (py_str("summer"), py_str("august")), (py_str("autumn"), py_str("november")), (py_str("fall"), py_str("november")), (py_str("winter"), py_str("february"))]));
    s_OTHER_POT(stdlib_call("re", "compile", &[py_str("\\s*(?:into|in|towards|toward|for)\\s+(?:a|an|my|our|the|his|her|their)\\s+[a-z]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_GIVEN(stdlib_call("re", "compile", &[py_str("\\b(?:putting by|putting away|put by|put away|setting aside|set aside|saving|saved|save|stashing|stash)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:putting|put|setting|set|stashing|stash|socking|sock|saving|save)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|also|ok|okay|well|if|maybe|what\\s+if|how\\s+about|i|we|i'd|we'd|could|can|would|will|shall|should|might|to|just|try|trying)\\b[\\s,]+)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_RATE(stdlib_call("re", "compile", &[py_str("(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?|quid|aside|away|by|back)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAVE_VERB(stdlib_call("re", "compile", &[py_str("^\\s*sav(?:e|es|ing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL(stdlib_call("re", "compile", &[py_str("\\b(?:if|what\\s+if|could|can|would|might)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ADDED(stdlib_call("re", "compile", &[py_str("\\b(?:another|extra|plus|more|on\\s+top)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RESTATED_RATE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|then|ok|okay|right)\\b[\\s,]*)*(?:£\\s*)?(\\d[\\d,]*)\\s*(?:pounds?\\s+)?(?:a|per|each)\\s+month\\s*(?:then|now)?\\s*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAVING_CLOSERS(py_tuple(vec![py_str("A standing order the day after payday does it without you deciding again, and any interest or pay rise only shortens it."), py_str("Set it up as a standing order for the day after payday, so the month cannot quietly spend what the plan needs."), py_str("The day after payday is the day to move it, before the rest of the month gets a look at it.")]));
    s_A_WEEK_ASK(py_str("how much should i put by each week"));
    s_A_MONTH_FIGURE(stdlib_call("re", "compile", &[py_str("£\\s*(\\d[\\d,]*)\\s*a\\s+month"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_LENGTH_FOR_A_THING(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:many|long)\\b[^?.!,]{0,32}?\\b(?:weeks?|months?|years?|days?)\\b|\\bhow\\s+long\\b[^?.!,]{0,32}?\\b(?:until|till|before|to\\s+buy|to\\s+afford|to\\s+get)\\b|\\bwhen\\s+(?:can|will)\\s+i\\s+(?:afford|buy|get)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_THING_WANTED(stdlib_call("re", "compile", &[py_str("\\b(?:afford|buy|pay\\s+for|save\\s+up\\s+for|have\\s+enough\\s+for)\\b|\\bget\\s+(?:it|one|ones|them|that|this|another)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:afford|buy|get|pay\\s+for)\\s+(?:a|an|the|my|our|another|some)?\\s*([a-z][a-z'-]{2,})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("it"), py_str("one"), py_str("ones"), py_str("them"), py_str("that"), py_str("this"), py_str("thing"), py_str("stuff"), py_str("something"), py_str("anything"), py_str("enough"), py_str("there"), py_str("what"), py_str("which")]) }
    s_NOT_A_THING(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_BY(stdlib_call("re", "compile", &[py_str("\\bsav(?:e|es|ed|ing|ings)\\b|\\bput(?:ting)?\\b[^.]{0,24}?\\b(?:aside|by|away)\\b|\\bset(?:ting)?\\b[^.]{0,24}?\\baside\\b|\\bspar(?:e|es|ed|ing)\\b|\\bsock(?:ing)?\\b[^.]{0,12}?\\baway\\b|\\btuck(?:ing)?\\b[^.]{0,12}?\\baway\\b|\\bsave\\s+up\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_OUT(stdlib_call("re", "compile", &[py_str("\\brent\\b|\\bmortgage\\b|\\bchildcare\\b|\\bnursery\\b|\\bwages?\\b|\\bsalary\\b|\\bpay\\b|\\bpaid\\b|\\bbills?\\b|\\bloan\\b|\\binsurance\\b|\\bcouncil\\s+tax\\b|\\belectricity\\b|\\bheating\\b|\\bwater\\s+bill\\b|\\bsubscriptions?\\b|\\bpayment\\b|\\bpayments\\b|\\bpaying\\b|\\bcost(?:s|ing)?\\b|\\bfees?\\b|\\bfare\\b|\\bfares?\\b|\\btax\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_sum(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The sum of money in the line, as a plain number, or None.");
    line = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    let __iter1 = py_iter_or_raise(&mcall(&g_SUM(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    figure = { let __b = mcall(&match_.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    unit = mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[Py::Int(4)]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_POT(), "match", &[slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    let __try2 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&figure.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand"), py_str("thousand")]), &unit.clone()))) {
    let __aug7 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug7;
    }
    if py_truthy(&op_lt(&value.clone(), &Py::Int(20))) {
    continue;
    }
    return call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter8 = py_iter_or_raise(&mcall(&g_BARE_FIGURE(), "finditer", &[line.clone()]));
    for __item in __iter8 {
    match_ = __item.clone();
    let __try9 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&{ let __b = op_lt(&value.clone(), &Py::Int(100)); if py_truthy(&__b) { __b } else { { let __cmp141: Py = value.clone(); if py_truthy(&op_le(&Py::Int(1990), &__cmp141)) { op_le(&__cmp141, &Py::Int(2100)) } else { Py::Bool(false) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_POT(), "match", &[slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    return value.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_date(args: &[Py]) -> Py {
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
    let mut relative: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The date the line puts the goal in, in the person's own words (\"june\", \"christmas\"), or \"\".\n\n    The word is kept as they said it so the answer can use it back - \"£500 by Christmas\" reads\n    better than \"£500 by December\" - and `_month_of` turns it into a month to count with.\n    \"The end of\" is kept with it: it names the last day of the month rather than the first, and\n    the two are a month apart.\n    ");
    relative = mcall(&g_END_OF_RELATIVE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&relative.clone()) {
    return op_add(&py_str("the end of "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&relative.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])]));
    }
    match_ = mcall(&g_DATE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    word = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    said = mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("end of")))) {
    return op_add(&py_str("the end of "), &word.clone());
    }
    return word.clone();
    Py::None
}
pub fn _month_of(args: &[Py]) -> Py {
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
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut step: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The month a date falls in, so a named one can be counted: \"christmas\" is December.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    if py_truthy(&mcall(&word.clone(), "startswith", &[py_str("the end of ")])) {
    word = mcall(&slice_or_raise(&word.clone(), &call_builtin("len", &[py_str("the end of ")]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("next month"), py_str("this month")]), &word.clone()))) {
    datetime = stdlib_module("datetime");
    step = if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("this month")))) { Py::Int(1) } else { Py::Int(2) };
    return geti(&g_MONTHS(), &op_modulo(&op_sub(&op_add(&gattr(&mcall(&gattr(&datetime.clone(), "date"), "today", &[]), "month"), &step.clone()), &Py::Int(2)), &Py::Int(12)));
    }
    if py_truthy(&Py::Bool(py_contains(&g_MONTHS(), &word.clone()))) {
    return word.clone();
    }
    let __iter1 = py_iter_or_raise(&g_NAMED_WHEN());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    month = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &name.clone()))) {
    return month.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _two_sums_added(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut total: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The two sums the line adds together, or 0 when it names only one.");
    found = mcall(&g_TOGETHER(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Int(0);
    }
    total = Py::Int(0);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)])]), py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(3)]), mcall(&found.clone(), "group", &[Py::Int(4)])])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    figure = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    let __try3 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&figure.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled7 = true;
    return Py::Int(0);
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand"), py_str("thousand")]), &mcall(&{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    let __aug8 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug8;
    }
    let __aug9 = op_add(&total.clone(), &call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    total = __aug9;
    }
    return if py_truthy(&op_ge(&total.clone(), &Py::Int(20))) { total.clone() } else { Py::Int(0) };
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
    let mut figure: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut value: Py = Py::None;
    let _ = py_str("A figure written as people write it (\"2\", \"2k\", \"2 grand\") as a number, or 0.");
    let __try1 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&call_builtin("str", &[{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::Int(0);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand"), py_str("thousand")]), &mcall(&{ let __b = unit.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    let __aug6 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug6;
    }
    return if py_truthy(&op_ge(&value.clone(), &Py::Int(20))) { call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    Py::None
}
pub fn the_pot_and_the_rest(args: &[Py]) -> Py {
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
    let mut further: Py = Py::None;
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("(money already held, money still to find) when the line names both, else (0, 0).\n\n    The two sums are the pot and the rest of it: \"i have 1000 saved already and want another\n    2000\", \"theres 500 put by and i want another 1500\". Only a line with both is this shape - a\n    report of money saved with nothing further named is not a goal at all, and a further sum with\n    no pot behind it is an ordinary goal.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[])))) {
    return py_tuple(vec![Py::Int(0), Py::Int(0)]);
    }
    further = mcall(&g_FURTHER_SAID(), "search", &[line.clone()]);
    held = mcall(&g_HELD_SAID(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&further.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return py_tuple(vec![Py::Int(0), Py::Int(0)]);
    }
    if py_truthy(&op_lt(&mcall(&further.clone(), "start", &[]), &mcall(&held.clone(), "start", &[]))) {
    return py_tuple(vec![Py::Int(0), Py::Int(0)]);
    }
    pot = _pounds(&[mcall(&held.clone(), "group", &[Py::Int(1)]), mcall(&held.clone(), "group", &[Py::Int(2)])]);
    rest = _pounds(&[mcall(&further.clone(), "group", &[Py::Int(1)]), mcall(&further.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pot.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rest.clone())) } }) {
    return py_tuple(vec![Py::Int(0), Py::Int(0)]);
    }
    return py_tuple(vec![pot.clone(), rest.clone()]);
    Py::None
}
pub fn the_pot_said_alone(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pot: Py = Py::None;
    let _ = py_str("The money the line says is already saved, when the line is only that, else 0.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&g_FURTHER_SAID(), "search", &[line.clone()]) } }) {
    return Py::Int(0);
    }
    if py_truthy(&{ let __b = mcall(&g_AIM_AT_A_SUM(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BY_A_DATE(), "search", &[line.clone()]) } }) {
    return Py::Int(0);
    }
    held = mcall(&g_HELD_SAID(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_POT_REPORTED(), "search", &[line.clone()]))) } }) {
    if py_truthy(&{ let __b = mcall(&g_INTENT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:have|has|got|got\\s+\\w+)\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Int(0);
    }
    return _pounds(&[mcall(&held.clone(), "group", &[Py::Int(1)]), mcall(&held.clone(), "group", &[Py::Int(2)])]);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_ALREADY_HELD(), g_ALREADY_AFTER()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    pot = _pounds(&[mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&op_ge(&pot.clone(), &Py::Int(50))) {
    return pot.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn the_pot_they_are_on(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The money the line says the person is on their way to, when that is what the line says.\n\n    \"im on 4 thousand\" against a goal of 20 thousand is £4,000 of the way there. Only a line that\n    says nothing but where they are counts: a line naming what they earn (\"im 29 and on about\n    30k\") is a salary, not a pot, and a line naming a goal (\"im on 4 thousand a month\") is a rate.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_A_PRICE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { _a_rate(&[line.clone()]) } } } }) {
    return Py::Int(0);
    }
    match_ = mcall(&g_AT_A_FIGURE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Int(0);
    }
    if py_truthy(&mcall(&g_NOT_A_POT(), "match", &[slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return Py::Int(0);
    }
    return _pounds(&[mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])]);
    Py::None
}
pub fn _a_rate(args: &[Py]) -> Py {
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
    let _ = py_str("True when the figure in the line is a rate rather than a sum.");
    return call_builtin("bool", &[mcall(&g_A_RATE(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_pot_they_are_on(args: &[Py]) -> Py {
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
    let mut pot: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last position the person said they were on towards the goal, or 0.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    pot = the_pot_they_are_on(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&pot.clone()) {
    return pot.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_goal_they_need(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The sum the chat says they need, when the chat has already said what they save for.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let __b = mcall(&g_SAVING(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_sum(&[line.clone()]))); if py_truthy(&__b) { __b } else { _a_rate(&[line.clone()]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = the_pot_they_are_on(&[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = the_pot_said_alone(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_rate(&[line.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_NEED_A_FIGURE(), "search", &[line.clone()])) {
    wanted = _a_sum(&[line.clone()]);
    if py_truthy(&wanted.clone()) {
    return wanted.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn asks_how_far_off(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how far off the goal the person is.");
    return call_builtin("bool", &[mcall(&g_HOW_FAR_OFF(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_gap_left(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The money between the pot and the goal, said back, or \"\".\n\n    The two figures are the chat's own: the goal it named and the position it last heard. Only\n    when both are held and the pot is short of the goal - a gap of nothing is not a gap, and a\n    pot past the goal is a different answer (\"you're there\").\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks_how_far_off(&[line.clone()]))); if py_truthy(&__b) { __b } else { { let __b = _a_sum(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_rate(&[line.clone()]) } } } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&the_goal_and_the_pot(&[memory.clone()]), 2, None);
    goal = __unpk1[0].clone();
    pot = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&goal.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pot.clone())); if py_truthy(&__b) { __b } else { op_ge(&pot.clone(), &goal.clone()) } }) {
    return py_str("");
    }
    said = _the_gap_said(&[goal.clone(), pot.clone()]);
    if py_truthy(&_the_gap_was_just_said(&[said.clone(), memory.clone()])) {
    return _the_gap_again(&[goal.clone(), pot.clone(), memory.clone()]);
    }
    return said.clone();
    Py::None
}
pub fn _the_gap_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut goal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "goal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: goal"))) };
    let mut pot: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pot") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pot"))) };
    let mut left: Py = Py::None;
    let mut share: Py = Py::None;
    let _ = py_str("The two figures side by side: what is left, and how much of it is already there.");
    left = op_sub(&goal.clone(), &pot.clone());
    share = call_builtin("int", &[call_builtin("round", &[op_truediv(&op_mul(&Py::Float(100.0), &pot.clone()), &goal.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return op_modulo(&py_str("£%s to go of £%s - you're £%s in, %d%% of the way there."), &py_tuple(vec![call_builtin("format", &[left.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[pot.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), share.clone()]));
    Py::None
}
pub fn _the_gap_was_just_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's own last line was this same sentence.\n\n    The position line and the question after it both want the gap, so asking the question straight\n    after the position would hand the person the same sentence twice in a row - the one thing a\n    reply must never do.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), &mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])])));
    Py::None
}
pub fn _the_gap_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut goal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "goal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: goal"))) };
    let mut pot: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pot") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pot"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The same gap said another way: the length at the rate they settled on, else a restatement.\n\n    A rate the chat has already settled on (\"i can put by 300 a month\") is the thing that turns the\n    gap into a date, which is the next thing a saver wants and not a number they have just read.\n    ");
    rate = _the_rate_in_the_chat(&[memory.clone()]);
    if py_truthy(&rate.clone()) {
    said = _the_length_at_that_rate(&[op_sub(&goal.clone(), &pot.clone()), rate.clone(), py_str("month"), pot.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return op_modulo(&py_str("Still £%s to go - you've £%s of the £%s in the pot."), &py_tuple(vec![call_builtin("format", &[op_sub(&goal.clone(), &pot.clone()), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[pot.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    Py::None
}
pub fn the_goal_and_the_pot(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut pot: Py = Py::None;
    let _ = py_str("(goal, pot) as the chat holds them, the goal from whichever reader knows it. (0, 0) else.");
    goal = { let __b = _the_goal_in_the_chat(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_goal_said_as_a_price(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_goal_they_need(&[memory.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&goal.clone()))) {
    return py_tuple(vec![Py::Int(0), Py::Int(0)]);
    }
    pot = { let __b = _the_money_already_held(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_pot_they_are_on(&[memory.clone()]) } };
    return py_tuple(vec![goal.clone(), pot.clone()]);
    Py::None
}
pub fn the_pot_position(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut pot: Py = Py::None;
    let _ = py_str("The gap said back when the line reports the position itself, or \"\".\n\n    \"i need 20 thousand\" / \"im on 4 thousand\" is the same pair of figures the gap question asks\n    about, said without a question: on its own the line was answered with an echo (\"On 4\n    thousand, then.\") which told the person nothing they had not just typed.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&the_pot_they_are_on(&[text.clone()]))); if py_truthy(&__b) { __b } else { { let __b = asks_how_far_off(&[text.clone()]); if py_truthy(&__b) { __b } else { _a_sum_that_is_a_goal(&[text.clone()]) } } } }) {
    return py_str("");
    }
    if py_truthy(&the_pot_said_alone(&[text.clone()])) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&the_goal_and_the_pot(&[memory.clone()]), 2, None);
    goal = __unpk1[0].clone();
    pot = __unpk1[1].clone();
    pot = { let __b = the_pot_they_are_on(&[text.clone()]); if py_truthy(&__b) { __b } else { pot.clone() } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&pot.clone())); if py_truthy(&__b) { __b } else { op_ge(&pot.clone(), &goal.clone()) } } } }) {
    return py_str("");
    }
    return _the_gap_said(&[goal.clone(), pot.clone()]);
    Py::None
}
pub fn _a_sum_that_is_a_goal(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line carries a goal of its own, so it is not only a position.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_a_rate(&[line.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_NEED_A_FIGURE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _a_sum(&[line.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn rate_question_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut amount: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "amount") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: amount"))) };
    let mut when: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { py_str("") };
    let _ = py_str("The money question about a sum the module works out itself, or \"\" - the money still to\n    find is not a figure the line names, so the question has to be written for it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&amount.clone())); if py_truthy(&__b) { __b } else { op_lt(&amount.clone(), &Py::Int(20)) } }) {
    return py_str("");
    }
    if py_truthy(&when.clone()) {
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds by %s?"), &py_tuple(vec![call_builtin("format", &[amount.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _in_the_money_readers_words(&[when.clone()])]));
    }
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds?"), &call_builtin("format", &[amount.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn the_money_already_held(args: &[Py]) -> Py {
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
    let _ = py_str("Public name for the pot, for the arithmetic outside this module that reads it too.");
    return _the_money_already_held(&[memory.clone()]);
    Py::None
}
pub fn _the_money_already_held(args: &[Py]) -> Py {
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
    let mut _rest: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The money the person said is already in the pot, from the last few lines, or 0.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __unpk2 = py_unpack(&the_pot_and_the_rest(&[line.clone()]), 2, None);
    pot = __unpk2[0].clone();
    _rest = __unpk2[1].clone();
    if py_truthy(&pot.clone()) {
    return pot.clone();
    }
    pot = the_pot_said_alone(&[line.clone()]);
    if py_truthy(&pot.clone()) {
    return pot.clone();
    }
    pot = the_pot_they_are_on(&[line.clone()]);
    if py_truthy(&pot.clone()) {
    return pot.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn without_the_pot_opener(args: &[Py]) -> Py {
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
    let _ = py_str("The answer with its \"For £2,000: \" opener taken off, so a lead-in can say the pot once.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POT_OPENER(), "sub", args))(&__args) };
    Py::None
}
pub fn _names_a_goal(args: &[Py]) -> Py {
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
    let mut added: Py = Py::None;
    let mut aim: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The sum when the line is aiming at money rather than reporting money already put by.\n\n    A line with an aim in it (\"i want to save 2000\") is always a goal, whatever else it says.\n    Without one it is a report of what is in the pot, which is not a goal.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    added = _two_sums_added(&[line.clone()]);
    if py_truthy(&{ let __b = added.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_SAVING(), "search", &[line.clone()]) } }) {
    return added.clone();
    }
    let __unpk1 = py_unpack(&the_pot_and_the_rest(&[line.clone()]), 2, None);
    pot = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    if py_truthy(&{ let __b = pot.clone(); if !py_truthy(&__b) { __b } else { { let __b = rest.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_SAVING(), "search", &[line.clone()]) } } } }) {
    return op_add(&pot.clone(), &rest.clone());
    }
    if py_truthy(&{ let __b = mcall(&g_INTENT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SAVING(), "search", &[line.clone()]) } }) {
    aim = _a_sum(&[line.clone()]);
    if py_truthy(&aim.clone()) {
    return aim.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SAVING(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _a_sum(&[line.clone()]) } }))) {
    return Py::Int(0);
    }
    if py_truthy(&mcall(&g_INTENT(), "search", &[line.clone()])) {
    return _a_sum(&[line.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_ALREADY(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PAST_SAVING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HELD(), "search", &[line.clone()]) } } } }) {
    return Py::Int(0);
    }
    return _a_sum(&[line.clone()]);
    Py::None
}
pub fn asks_how_long(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how long the goal takes and names no figure itself.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), line.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_HOW_LONG_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_LONG_UNTIL(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHEN_AT_THE_GOAL(), "match", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn how_long_it_takes(args: &[Py]) -> Py {
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
    let mut fast: Py = Py::None;
    let mut from_the_answer: Py = Py::None;
    let mut goal: Py = Py::None;
    let mut income: Py = Py::None;
    let mut last: Py = Py::None;
    let mut last_bot: Py = Py::None;
    let mut months: Py = Py::None;
    let mut period: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut priced: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_goal: Py = Py::None;
    let mut said_turns: Py = Py::None;
    let mut slow: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns_said: Py = Py::None;
    let _ = py_str("The goal said as a length of time, at the gentle rate and one faster, or \"\".\n\n    The goal is read from the answer above when the line named a purpose instead of a sum\n    (\"i want to save 2000 pounds for a holiday\"), so the length is about the money that answer\n    offered rates for.\n\n    With no rate and no date in the chat yet, \"how long will that take\" has an answer that is\n    already known: the goal at a rate the pay can carry. Handed the options again the line said\n    nothing new; handed nothing it got a search hedge.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return py_str("");
    }
    previous = _the_line_before(&[memory.clone()]);
    goal = _the_goal_in_the_chat(&[memory.clone()]);
    priced = if py_truthy(&goal.clone()) { Py::Int(0) } else { _the_goal_said_as_a_price(&[memory.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&goal.clone()))) {
    goal = priced.clone();
    }
    from_the_answer = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&goal.clone()))) {
    goal = _the_goal_in_the_answer(&[memory.clone()]);
    turns_said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    last_bot = if py_truthy(&turns_said.clone()) { call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns_said.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    from_the_answer = { let __b = call_builtin("bool", &[goal.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&last_bot.clone(), "lower", &[]), &py_str("still to find"))) } };
    }
    pot = if py_truthy(&Py::Bool(!py_truthy(&from_the_answer.clone()))) { _the_money_already_held(&[memory.clone()]) } else { Py::Int(0) };
    if py_truthy(&{ let __b = goal.clone(); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = pot.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_lt(&__cmp11, &goal.clone()) } else { Py::Bool(false) } } } }) {
    goal = op_sub(&goal.clone(), &pot.clone());
    } else {
    pot = Py::Int(0);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { op_lt(&goal.clone(), &Py::Int(100)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = previous.clone(); if !py_truthy(&__b) { __b } else { _a_length_was_asked(&[previous.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _the_rate_in_the_chat(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_date_in_the_chat(&[memory.clone()]) } }) {
    rate = _the_rate_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { { let __b = goal.clone(); if !py_truthy(&__b) { __b } else { { let __b = priced.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_date_in_the_chat(&[memory.clone()]))) } } } } } }) {
    return _the_length_at_that_rate(&[goal.clone(), rate.clone(), py_str("month"), pot.clone()]);
    }
    if py_truthy(&{ let __b = rate.clone(); if !py_truthy(&__b) { __b } else { op_lt(&rate.clone(), &goal.clone()) } }) {
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    said_turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    last = if py_truthy(&said_turns.clone()) { call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&said_turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_modulo(&py_str("\\b%d\\s+months?\\b"), &months.clone()), last.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return without_what_was_just_said(&[_a_rate_against_the_goal(&[goal.clone(), rate.clone(), memory.clone()]), memory.clone()]);
    }
    }
    return py_str("");
    }
    let __unpk2 = py_unpack(&_a_rate_in_another_period(&[memory.clone()]), 2, None);
    rate = __unpk2[0].clone();
    period = __unpk2[1].clone();
    if py_truthy(&rate.clone()) {
    return _the_length_at_that_rate(&[goal.clone(), rate.clone(), period.clone(), pot.clone()]);
    }
    slow = op_neg(&op_floordiv(&op_neg(&goal.clone()), &Py::Int(100)));
    fast = op_neg(&op_floordiv(&op_neg(&goal.clone()), &Py::Int(250)));
    income = _the_income_in_the_chat(&[memory.clone()]);
    if py_truthy(&income.clone()) {
    said_goal = if py_truthy(&pot.clone()) { op_modulo(&py_str("the £%s still to find"), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) } else { op_modulo(&py_str("£%s"), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) };
    return op_modulo(&py_str("How much of the £%s can you put by each month? At £100 a month %s takes %s."), &py_tuple(vec![call_builtin("format", &[income.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), said_goal.clone(), _months(&[slow.clone()])]));
    }
    said = if py_truthy(&pot.clone()) { op_modulo(&py_str("the £%s still to find takes "), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) } else { op_modulo(&py_str("£%s takes "), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) };
    if py_truthy(&Py::Bool(py_eq(&fast.clone(), &slow.clone()))) {
    return op_add(&op_add(&py_str("At £100 a month, "), &said.clone()), &op_modulo(&py_str("%s. Pick the rate your pay can carry and set the standing order for the day after payday."), &_months(&[slow.clone()])));
    }
    return op_add(&op_add(&py_str("At £100 a month, "), &said.clone()), &op_modulo(&py_str("%s - or %s at £250 a month. Pick the one your pay can carry."), &py_tuple(vec![_months(&[slow.clone()]), _months(&[fast.clone()])])));
    Py::None
}
pub fn _only_a_date(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut relative: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The month when the line is nothing but a date, or \"\".");
    bare = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    bare = mcall(&mcall(&g_CORRECTING(), "sub", &[py_str(""), bare.clone()]), "strip", &[]);
    relative = mcall(&g_BARE_END_OF_RELATIVE(), "match", &[bare.clone()]);
    if py_truthy(&relative.clone()) {
    return op_add(&py_str("the end of "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&relative.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])]));
    }
    match_ = mcall(&g_BARE_DATE(), "match", &[bare.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    word = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]), &py_str("end of")))) {
    return op_add(&py_str("the end of "), &word.clone());
    }
    return word.clone();
    Py::None
}
pub fn is_only_a_date(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is nothing but a date (\"by june\", \"by christmas\").\n\n    A date on its own is a follow-up, never a subject of its own: \"by christmas\" after a saving\n    goal used to be read as Christmas shopping and answered with a gift budget, and the money\n    question it really is never got asked.\n    ");
    return call_builtin("bool", &[_only_a_date(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_a_sum(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The sum when the line corrects a figure with nothing but a figure, or 0.\n\n    A correction carries no saving word (\"actually make that 1000\", \"sorry, 1500\"), so a chat that\n    corrects its goal used to keep answering the sum it had before: the store said \"Save 1000 pounds\n    - got it\" and the date still worked on 500. The line has to be nothing but the figure, or \"no,\n    1000 a month\" would be read as a new pot instead of a rate.\n    ");
    bare = mcall(&mcall(&g_CORRECTING(), "sub", &[py_str(""), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&bare.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::Int(0);
    }
    aim = _a_sum(&[bare.clone()]);
    rest = mcall(&stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&mcall(&g_SUM(), "sub", &[py_str(""), bare.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&rest.clone()) {
    return Py::Int(0);
    }
    if py_truthy(&Py::Bool(!py_truthy(&aim.clone()))) {
    figure = stdlib_call("re", "sub", &[py_str("[^0-9]"), py_str(""), bare.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("int", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return call_builtin("int", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Int(0);
    }
    return aim.clone();
    Py::None
}
pub fn asks_about_saving(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line states a saving goal with a sum in it.\n\n    The chat module asks this before it offers a plan: \"i want to save 2000 pounds by april\" is a\n    money question, and \"start with the smallest version of it\" answers nothing about it.\n    ");
    return call_builtin("bool", &[_names_a_goal(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_money_still_to_find(args: &[Py]) -> Py {
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
    let mut whole: Py = Py::None;
    let _ = py_str("The goal less the money already in the pot: what is still to be found.\n\n    \"i have 1000 saved already and want another 2000\" is a £3,000 goal with £1,000 already there,\n    and every figure the person wants next - the monthly rate, the weeks left, the date - is about\n    the £2,000 still to find. The whole pot is the goal; the arithmetic is on the rest.\n    ");
    whole = _the_goal_in_the_chat(&[memory.clone()]);
    held = _the_money_already_held(&[memory.clone()]);
    if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = held.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_lt(&__cmp11, &whole.clone()) } else { Py::Bool(false) } } } }) {
    return op_sub(&whole.clone(), &held.clone());
    }
    return whole.clone();
    Py::None
}
pub fn _the_goal_in_the_chat(args: &[Py]) -> Py {
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
    let _ = py_str("The sum the person said they were saving, from the last few lines, or 0.");
    return _the_goal_named_in_a_line(&[memory.clone()]);
    Py::None
}
pub fn _the_goal_named_in_a_line(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The sum a line puts beside a saving word (\"im saving 3000 for a car\"), or 0.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    corrected = _only_a_sum(&[line.clone()]);
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    if py_truthy(&the_pot_said_alone(&[line.clone()])) {
    continue;
    }
    aim = _names_a_goal(&[line.clone()]);
    if py_truthy(&aim.clone()) {
    return aim.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_goal_said_as_a_price(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut line: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The sum said of the thing they are saving for, when no line named a goal, or 0.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    recent = slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent.clone()) { let mut t: Py = __item.clone(); __out.push({ let __b = mcall(&g_SAVING(), "search", &[call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_sum(&[call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))); if py_truthy(&__b) { __b } else { _a_rate(&[call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[recent.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { the_pot_said_alone(&[line.clone()]) } }) {
    continue;
    }
    if py_truthy(&the_pot_they_are_on(&[line.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_A_RATE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_RATE_ANY_OTHER_PERIOD(), "search", &[line.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PRICE_WORDS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_TRAILING_SUM(), "search", &[line.clone()]) } }))) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_PRICE(), "search", &[line.clone()])) {
    continue;
    }
    aim = _a_sum(&[line.clone()]);
    if py_truthy(&{ let __b = aim.clone(); if !py_truthy(&__b) { __b } else { op_ge(&aim.clone(), &Py::Int(100)) } }) {
    return aim.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn a_length_was_said(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names the time it has: \"save 10k in 6 months\", \"5000 in a year\".\n\n    A goal and a length are a whole question - the figure to put by each month - and the money\n    entry works it out from the line itself. Read as a goal on its own the same line got the\n    three rates nobody asked for (\"For £10,000: £100 a month is 100 months, ...\").\n    ");
    return call_builtin("bool", &[mcall(&g_A_LENGTH_SAID(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_length_was_asked(args: &[Py]) -> Py {
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
    let _ = py_str("True when a line asks how long something takes, however much else it says.");
    return call_builtin("bool", &[mcall(&g_A_LENGTH_WAS_ASKED(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's line before this one, or \"\" - the ask a follow-up is repeating.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_goal_in_the_answer(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The pot stated in the bot's last few answers, or 0.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_A_GOAL_STATED(), "finditer", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter2 {
    found = __item.clone();
    let __try3 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), &py_str("k")))) {
    let __aug8 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug8;
    }
    value = call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_ge(&value.clone(), &Py::Int(100))) {
    return value.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn another_sum_asked(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut talked_money: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line read as a saving goal on a new sum, or \"\".\n\n    Only when the line is that and nothing else, and only when a goal is already in the chat: a\n    sum with no question around it after one is the same arithmetic on a different figure.\n    ");
    found = { let __b = mcall(&g_ANOTHER_SUM(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_A_GOAL_SUM(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    talked_money = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = _only_a_sum(&[line.clone()]); if py_truthy(&__b) { __b } else { _names_a_goal(&[line.clone()]) } }) {
    talked_money = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&talked_money.clone()))) {
    return py_str("");
    }
    figure = _a_sum(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&figure.clone())); if py_truthy(&__b) { __b } else { op_lt(&figure.clone(), &Py::Int(100)) } }) {
    return py_str("");
    }
    return op_modulo(&py_str("save £%s"), &call_builtin("format", &[figure.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _a_rate_is_settled(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when a rate the person gave is already in the chat.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&g_A_RATE(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_length_at_that_rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut goal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "goal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: goal"))) };
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut period: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "period") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: period"))) };
    let mut pot: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pot") { v.clone() }
    else { Py::Int(0) };
    let mut count: Py = Py::None;
    let mut line: Py = Py::None;
    let mut months: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The goal at a rate given in weeks, fortnights or years, in that same unit.\n\n    A part payment is another one, so a goal that does not divide exactly rounds up: £1,200 at £50\n    a week is 24 weeks, and £1,250 at the same rate is 25 payments.\n    ");
    count = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    said = if py_truthy(&pot.clone()) { op_modulo(&py_str("the £%s still to find takes "), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) } else { op_modulo(&py_str("£%s takes "), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))) };
    line = op_modulo(&py_str("At £%s a %s, %s%s %s"), &py_tuple(vec![call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), period.clone(), said.clone(), count.clone(), if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) { period.clone() } else { op_add(&period.clone(), &py_str("s")) }]));
    if py_truthy(&Py::Bool(py_eq(&period.clone(), &py_str("year")))) {
    return op_add(&line.clone(), &py_str("."));
    }
    months = call_builtin("int", &[call_builtin("round", &[op_mul(&count.clone(), &geti(&g_MONTHS_IN_A(), &period.clone()))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&period.clone(), &py_str("month"))); if !py_truthy(&__b) { __b } else { op_ge(&months.clone(), &Py::Int(2)) } }) {
    let __aug1 = op_add(&line.clone(), &op_modulo(&py_str(" (about %s)"), &_months(&[months.clone()])));
    line = __aug1;
    }
    return op_add(&line.clone(), &py_str("."));
    Py::None
}
pub fn _a_rate_in_another_period(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The rate and the period when the person gave one in weeks, fortnights or years.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    found = mcall(&g_A_RATE_ANY_OTHER_PERIOD(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&op_ge(&value.clone(), &Py::Int(5))) {
    return py_tuple(vec![value.clone(), mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[])]);
    }
    }
    return py_tuple(vec![Py::Int(0), py_str("")]);
    Py::None
}
pub fn _is_an_income(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when the figure in `found` is what the person is paid, not what they put by.");
    before = geti(&mcall(&g_CLAUSE_EDGE(), "split", &[slice_or_raise(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)]), &op_neg(&Py::Int(1)));
    return call_builtin("bool", &[mcall(&g_INCOME_FRAME(), "search", &[before.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_an_outgoing(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut text: Py = Py::None;
    let _ = py_str("True when the figure in `found` is money that goes out each month, not money put by.");
    text = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[mcall(&g_OUTGOING_FRAME(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&found.clone(), "start", &[]), &Py::Int(40))]).unwrap_or_else(|e| exc::raise(e)), &op_add(&mcall(&found.clone(), "end", &[]), &Py::Int(40)), &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_not_a_rate(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("True when this figure a month is neither what they put by nor a sum towards a goal.");
    return { let __b = _is_an_income(&[line.clone(), found.clone()]); if py_truthy(&__b) { __b } else { _is_an_outgoing(&[line.clone(), found.clone()]) } };
    Py::None
}
pub fn _the_income_in_the_chat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The month's money the person says they are paid, or 0.\n\n    Kept apart from the rate because the two answer different questions: the pay is what the\n    length question has to ask about (\"how much of that can you put by\"), and the rate is what it\n    can already do the sum with.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&g_A_RATE(), "finditer", &[line.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_is_an_income(&[line.clone(), found.clone()])))) {
    continue;
    }
    value = _a_sum_named(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&value.clone()) {
    return value.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _is_a_rate_line(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("True when the line puts money by at a rate, whatever else it says.");
    let __iter1 = py_iter_or_raise(&mcall(&g_A_RATE(), "finditer", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_is_not_a_rate(&[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), found.clone()])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_rate_put_right(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut after_a_no: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut with_the_month: Py = Py::None;
    let _ = py_str("The monthly rate a line puts right, or 0.\n\n    \"i can put by 250 a month\" / \"actually i can do 300 now ive paid the car off\" is one rate\n    written twice, and the second one is the rate: the car is paid off, so the 250 is old news.\n    The sum in the second line often has no \"a month\" after it, because the month was never in\n    question, so `A_RATE` cannot see it - and every sum the chat works out afterwards (the\n    length, the figure a week, the date it lands) was worked out at the rate the person had\n    already left behind.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MORE_LIKE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_RATE_PUT_RIGHT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _is_a_rate_line(&[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__OTHER_FIGURE_WORDS(), "search", &[line.clone()]))) } } } } } } } }) {
    figures = mcall(&g_A_LONE_SUM(), "findall", &[line.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return _a_sum_named(&[geti(&figures.clone(), &Py::Int(0))]);
    }
    }
    after_a_no = mcall(&g_A_NO_THEN_A_RATE(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = after_a_no.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_SPENT_ON(), "search", &[slice_or_raise(&line.clone(), &mcall(&after_a_no.clone(), "end", &[]), &Py::None, &Py::None)]))) } }) {
    return _a_sum_named(&[mcall(&after_a_no.clone(), "group", &[Py::Int(1)])]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_RATE_PUT_RIGHT(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__OTHER_FIGURE_WORDS(), "search", &[line.clone()]) } }) {
    return Py::Int(0);
    }
    with_the_month = mcall(&g_A_SUM_A_MONTH(), "search", &[line.clone()]);
    if py_truthy(&with_the_month.clone()) {
    return _a_sum_named(&[mcall(&with_the_month.clone(), "group", &[Py::Int(1)])]);
    }
    figures = mcall(&g_A_LONE_SUM(), "findall", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::Int(0);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_CAN_CARRY_IT(), "search", &[line.clone()])))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_CORRECTION_AND_A_FIGURE(), "match", &[mcall(&line.clone(), "strip", &[])]))); if py_truthy(&__b) { __b } else { mcall(&g__NOT_THE_RATE(), "search", &[line.clone()]) } }) {
    return Py::Int(0);
    }
    }
    return _a_sum_named(&[geti(&figures.clone(), &Py::Int(0))]);
    Py::None
}
pub fn _a_sum_named(args: &[Py]) -> Py {
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
    let mut value: Py = Py::None;
    let _ = py_str("One sum as a number, or 0 when it is not a rate anybody could put by.");
    let __try1 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&call_builtin("str", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    return Py::Int(0);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return if py_truthy(&op_ge(&value.clone(), &Py::Int(10))) { value.clone() } else { Py::Int(0) };
    Py::None
}
pub fn _the_rate_in_the_chat(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The sum a month the person has settled on, reading the last few lines in order, or 0.\n\n    Read forwards, so the newest word on it wins: a rate put right after a rate is the rate, and\n    everything the chat works out from here - the length, the figure a week, the date it lands -\n    is worked out on what the person can put by now.\n    ");
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(6)), &Py::None, &Py::None);
    settled = Py::Int(0);
    before = py_str("");
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&g_A_RATE(), "finditer", &[line.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&_is_not_a_rate(&[line.clone(), found.clone()])) {
    continue;
    }
    value = _a_sum_named(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&value.clone()) {
    settled = value.clone();
    }
    continue;
    }
    if py_truthy(&settled.clone()) {
    put_right = a_rate_put_right(&[line.clone(), before.clone()]);
    if py_truthy(&put_right.clone()) {
    settled = put_right.clone();
    }
    }
    before = line.clone();
    }
    return settled.clone();
    Py::None
}
pub fn _the_date_in_the_chat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The month the goal was put in, from the last few lines, or \"\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    when = { let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } };
    if py_truthy(&when.clone()) {
    return when.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _months_left(args: &[Py]) -> Py {
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
    let mut month: Py = Py::None;
    let mut number: Py = Py::None;
    let mut today: Py = Py::None;
    let mut word: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The months from this one to the month the person named: \"by december\" said in November is\n    one month off, whichever December that is.");
    datetime = stdlib_module("datetime");
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    word = mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&word.clone(), &py_str("this month"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&word.clone(), &py_str("this week"))) } }) {
    return Py::Int(0);
    }
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str("next month")))) {
    return Py::Int(1);
    }
    month = _month_of(&[when.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&month.clone()))) {
    return Py::Int(0);
    }
    number = op_add(&mcall(&g_MONTHS(), "index", &[month.clone()]), &Py::Int(1));
    year = op_add(&gattr(&today.clone(), "year"), &if py_truthy(&op_le(&number.clone(), &gattr(&today.clone(), "month"))) { Py::Int(1) } else { Py::Int(0) });
    return call_builtin("max", &[op_add(&op_mul(&op_sub(&year.clone(), &gattr(&today.clone(), "year")), &Py::Int(12)), &op_sub(&number.clone(), &gattr(&today.clone(), "month"))), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _months(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut n: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: n"))) };
    let mut months: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("The length as months, with the years in it once it is over a year.\n\n    \"£3,000 at £250 a month is 12 months\" makes the person do the counting again; the money entry\n    already says \"32 months, about 2 years and 8 months\", and a length is only useful if it is\n    the length they can feel.\n    ");
    said = op_modulo(&py_str("%s month%s"), &py_tuple(vec![call_builtin("format", &[n.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), if py_truthy(&Py::Bool(py_eq(&n.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }]));
    if py_truthy(&op_lt(&n.clone(), &Py::Int(12))) {
    return said.clone();
    }
    let __unpk1 = py_unpack(&call_builtin("divmod", &[n.clone(), Py::Int(12)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    years = __unpk1[0].clone();
    months = __unpk1[1].clone();
    parts = py_list(vec![]);
    if py_truthy(&years.clone()) {
    let _ = mcall(&parts.clone(), "append", &[op_modulo(&py_str("%s year%s"), &py_tuple(vec![call_builtin("format", &[years.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), if py_truthy(&Py::Bool(py_eq(&years.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }]))]);
    }
    if py_truthy(&months.clone()) {
    let _ = mcall(&parts.clone(), "append", &[op_modulo(&py_str("%s month%s"), &py_tuple(vec![months.clone(), if py_truthy(&Py::Bool(py_eq(&months.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }]))]);
    }
    return op_add(&op_add(&said.clone(), &py_str(", about ")), &mcall(&py_str(" and "), "join", &[parts.clone()]));
    Py::None
}
pub fn a_date_on_a_settled_rate(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The answer to a bare date asked after the rate was already chosen, or \"\".\n\n    Once someone has said what they can put by (\"if we put by 150 a month\"), a date on its own is\n    not a request for a new weekly figure - the money entry would answer it with \"£3,000 by\n    December is about £700 a week\", which argues with the rate they just gave. It asks whether\n    that rate gets there by then, and the honest answer puts the two figures side by side: how\n    long the rate takes, how far off the date is, and what it would take instead.\n    ");
    when = _only_a_date(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&when.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_rate_is_settled(&[memory.clone()]))) } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![_the_rate_in_the_chat(&[memory.clone()]), _the_money_still_to_find(&[memory.clone()])]), 2, None);
    rate = __unpk1[0].clone();
    goal = __unpk1[1].clone();
    return _a_date_on_a_rate(&[goal.clone(), rate.clone(), when.clone()]);
    Py::None
}
pub fn _a_date_on_a_rate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut goal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "goal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: goal"))) };
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut when: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut goal_text: Py = Py::None;
    let mut left: Py = Py::None;
    let mut months: Py = Py::None;
    let mut name: Py = Py::None;
    let mut need: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rate_text: Py = Py::None;
    let _ = py_str("The rate against the month it has to reach: whether it gets there, and what would.");
    left = _months_left(&[when.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { { let __b = op_le(&goal.clone(), &rate.clone()); if py_truthy(&__b) { __b } else { op_lt(&left.clone(), &Py::Int(1)) } } } } } }) {
    return py_str("");
    }
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    name = if py_truthy(&mcall(&when.clone(), "startswith", &[py_str("the end of ")])) { when.clone() } else { mcall(&when.clone(), "capitalize", &[]) };
    goal_text = op_modulo(&py_str("£%s"), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    rate_text = op_modulo(&py_str("£%s a month"), &call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&op_le(&months.clone(), &left.clone())) {
    return op_modulo(&py_str("%s at %s takes %s and %s is %s away, so that rate gets you there. Set the standing order for the day after payday and it looks after itself — a bonus or a refund just brings the date forward."), &py_tuple(vec![goal_text.clone(), rate_text.clone(), _months(&[months.clone()]), name.clone(), _months(&[left.clone()])]));
    }
    need = op_neg(&op_floordiv(&op_neg(&goal.clone()), &left.clone()));
    need = call_builtin("int", &[op_mul(&op_neg(&op_floordiv(&op_neg(&need.clone()), &Py::Int(10))), &Py::Int(10))]).unwrap_or_else(|e| exc::raise(e));
    part = call_builtin("min", &[op_mul(&rate.clone(), &left.clone()), goal.clone()]).unwrap_or_else(|e| exc::raise(e));
    return op_modulo(&py_str("%s at %s takes %s and %s is %s away, so it does not get you there — that would take about £%s a month. At %s you have £%s of it by then; a bonus, a bill that drops or a later date closes the gap."), &py_tuple(vec![goal_text.clone(), rate_text.clone(), _months(&[months.clone()]), name.clone(), _months(&[left.clone()]), call_builtin("format", &[need.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), rate_text.clone(), call_builtin("format", &[part.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    Py::None
}
pub fn asks_if_the_rate_is_enough(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks whether the rate already in the chat does the job.");
    return call_builtin("bool", &[mcall(&g_ASKS_IF_ENOUGH(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_yes_no_about_the_rate(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut goal_text: Py = Py::None;
    let mut left: Py = Py::None;
    let mut months: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut rate_only: Py = Py::None;
    let mut rate_text: Py = Py::None;
    let mut room: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The verdict on the rate the chat holds, in its short form, or \"\".\n\n    \"i want to save 3000 pounds by christmas\" / \"i can put by 200 a month\" / \"is that enough\"\n    ended in \"Sounds good.\" - the opposite of what the answer a turn earlier had just worked out\n    (that the rate does not get there). The sum is already in `_a_date_on_a_rate`; what this line\n    wants is the verdict first and the figures behind it, not the same long sentence twice, so\n    the date is answered in one line and the figures are only the ones the verdict turns on.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks_if_the_rate_is_enough(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![_the_money_still_to_find(&[memory.clone()]), _the_rate_in_the_chat(&[memory.clone()])]), 2, None);
    goal = __unpk1[0].clone();
    rate = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rate.clone())) } }) {
    return py_str("");
    }
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    when = _the_date_in_the_chat(&[memory.clone()]);
    left = if py_truthy(&when.clone()) { _months_left(&[when.clone()]) } else { Py::Int(0) };
    goal_text = op_modulo(&py_str("£%s"), &call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    rate_only = op_modulo(&py_str("£%s a month"), &call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    rate_text = op_modulo(&py_str("at %s"), &rate_only.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&when.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&left.clone())) } }) {
    return op_modulo(&py_str("%s %s takes %s. Name the month you want it by and I will tell you whether that is soon enough."), &py_tuple(vec![goal_text.clone(), rate_text.clone(), _months(&[months.clone()])]));
    }
    if py_truthy(&op_le(&months.clone(), &left.clone())) {
    spare = op_sub(&left.clone(), &months.clone());
    room = if py_truthy(&spare.clone()) { op_modulo(&py_str("%s month%s to spare"), &py_tuple(vec![spare.clone(), if py_truthy(&Py::Bool(py_eq(&spare.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }])) } else { py_str("nothing to spare") };
    return op_modulo(&py_str("Yes — %s gets you to %s by %s, %s. Leave the standing order where it is."), &py_tuple(vec![rate_only.clone(), goal_text.clone(), mcall(&call_builtin("str", &[when.clone()]).unwrap_or_else(|e| exc::raise(e)), "capitalize", &[]), room.clone()]));
    }
    part = call_builtin("min", &[op_mul(&rate.clone(), &left.clone()), goal.clone()]).unwrap_or_else(|e| exc::raise(e));
    return op_modulo(&py_str("No — %s you have £%s of %s by %s. The whole %s takes %s at that rate, so the rest wants more a month or a later date."), &py_tuple(vec![rate_text.clone(), call_builtin("format", &[part.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), goal_text.clone(), mcall(&call_builtin("str", &[when.clone()]).unwrap_or_else(|e| exc::raise(e)), "capitalize", &[]), goal_text.clone(), _months(&[months.clone()])]));
    Py::None
}
pub fn asks_how_much_more_a_month(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what extra a month the goal and the date need.");
    return call_builtin("bool", &[mcall(&g_ASKS_HOW_MUCH_MORE(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_extra_a_month_asked(args: &[Py]) -> Py {
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
    let mut goal: Py = Py::None;
    let mut left: Py = Py::None;
    let mut left_text: Py = Py::None;
    let mut months: Py = Py::None;
    let mut more: Py = Py::None;
    let mut need: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The extra a month the date needs, or \"\" - the question the verdict above invites.\n\n    \"so the rest wants more a month or a later date\" is an invitation, and \"and how much more a\n    month would do it\" is the natural next line. Unread, it went to the goal's own entry and\n    printed the first answer again, word for word, three turns later.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks_how_much_more_a_month(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![_the_money_still_to_find(&[memory.clone()]), _the_rate_in_the_chat(&[memory.clone()])]), 2, None);
    goal = __unpk1[0].clone();
    rate = __unpk1[1].clone();
    when = _the_date_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&when.clone())) } } } }) {
    return py_str("");
    }
    left = _months_left(&[when.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&left.clone()))) {
    return py_str("");
    }
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    if py_truthy(&op_le(&months.clone(), &left.clone())) {
    spare = op_sub(&left.clone(), &months.clone());
    return op_modulo(&py_str("Nothing more — %s already gets you to £%s by %s with %s month%s to spare."), &py_tuple(vec![op_modulo(&py_str("£%s a month"), &call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))), call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), mcall(&call_builtin("str", &[when.clone()]).unwrap_or_else(|e| exc::raise(e)), "capitalize", &[]), spare.clone(), if py_truthy(&Py::Bool(py_eq(&spare.clone(), &Py::Int(1)))) { py_str("") } else { py_str("s") }]));
    }
    need = op_neg(&op_floordiv(&op_neg(&goal.clone()), &left.clone()));
    need = call_builtin("int", &[op_mul(&op_neg(&op_floordiv(&op_neg(&need.clone()), &Py::Int(10))), &Py::Int(10))]).unwrap_or_else(|e| exc::raise(e));
    more = call_builtin("max", &[op_sub(&need.clone(), &rate.clone()), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e));
    left_text = if py_truthy(&Py::Bool(py_eq(&left.clone(), &Py::Int(1)))) { py_str("the month that is left") } else { op_modulo(&py_str("the %s left"), &_months(&[left.clone()])) };
    return op_modulo(&py_str("About £%s a month more: £%s a month for %s, rather than the £%s you have. Or give it longer — at £%s a month the whole £%s takes %s."), &py_tuple(vec![call_builtin("format", &[more.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[need.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), left_text.clone(), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _months(&[months.clone()])]));
    Py::None
}
pub fn the_same_figure_a_month(args: &[Py]) -> Py {
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
    let mut a_month: Py = Py::None;
    let mut a_week: Py = Py::None;
    let mut found: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The rate asked for in months, said in months, when the chat already gave it in weeks, or \"\".\n\n    \"how much a month?\" after \"£2,000 by June is about £67 a week\" asks for the same figure in\n    another unit, and working it out again from the goal gives the same sentence a second time.\n    Said in months from the figure already on the screen, the line gets an answer of its own.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HOW_MUCH(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    found = mcall(&g_A_WEEK_FIGURE(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    a_week = call_builtin("int", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    return py_str("");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&op_lt(&a_week.clone(), &Py::Int(5))) {
    return py_str("");
    }
    a_month = op_mul(&call_builtin("int", &[call_builtin("round", &[op_truediv(&op_truediv(&op_mul(&a_week.clone(), &Py::Int(52)), &Py::Float(12.0)), &Py::Float(10.0))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10));
    return op_modulo(&py_str("£%s a week is about £%s a month. Set the standing order for the day after payday and it runs itself — a bonus or a refund only shortens the time."), &py_tuple(vec![call_builtin("format", &[a_week.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[a_month.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    }
    return py_str("");
    Py::None
}
pub fn a_rate_restated(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The goal worked out at the rate the line puts in place of the old one, or \"\".\n\n    \"i need to save 3000\" / \"how long at 200 a month\" / \"and at 300 a month\" is the same question\n    with one figure changed, and the rate on its own names neither the goal nor the money word: it\n    reached the knowledge entry as a line about nothing and came back as the 200 answer again, the\n    closing sentence and all.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&g_A_NEW_RATE(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_PAST_SAVING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OTHER_POT(), "search", &[line.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } }) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    rate = call_builtin("float", &[mcall(&mcall(&said.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&mcall(&said.clone(), "group", &[Py::Int(2)])) {
    let __aug6 = op_mul(&rate.clone(), &Py::Int(1000));
    rate = __aug6;
    }
    rate = call_builtin("int", &[call_builtin("round", &[rate.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&rate.clone(), &Py::Int(20))) {
    return py_str("");
    }
    aim = _the_goal_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&aim.clone())); if py_truthy(&__b) { __b } else { op_ge(&rate.clone(), &aim.clone()) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&rate.clone(), &_the_rate_in_the_chat(&[memory.clone()])))) {
    return py_str("");
    }
    return without_what_was_just_said(&[_a_rate_against_the_goal(&[aim.clone(), rate.clone(), memory.clone()]), memory.clone()]);
    Py::None
}
pub fn a_rate_put_in_with_a_verb(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The goal at a new rate when the figure comes in without its unit, or \"\".\n\n    \"i've got 200 a month to put away\" / \"how long until i can afford 5000\" / \"what if i put away\n    300\" is one goal and three rates. The third line names no unit, so on its own it is a sum,\n    and the money entry read it as the goal - a goal of 300, worked out at rates nobody asked\n    about. A rate already in the chat is what makes the bare figure a rate.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_RATE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_NEW_RATE(), "match", &[line.clone()]) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } }) {
    return py_str("");
    }
    said = mcall(&g_A_RATE_WITH_A_VERB(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_PAST_SAVING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OTHER_POT(), "search", &[line.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_rate_is_settled(&[memory.clone()])))) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    rate = call_builtin("float", &[mcall(&mcall(&said.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&mcall(&said.clone(), "group", &[Py::Int(2)])) {
    let __aug6 = op_mul(&rate.clone(), &Py::Int(1000));
    rate = __aug6;
    }
    rate = call_builtin("int", &[call_builtin("round", &[rate.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&rate.clone(), &Py::Int(20))) {
    return py_str("");
    }
    aim = { let __b = _the_goal_in_the_chat(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_goal_in_the_answer(&[memory.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&aim.clone())); if py_truthy(&__b) { __b } else { op_ge(&rate.clone(), &aim.clone()) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&rate.clone(), &_the_rate_in_the_chat(&[memory.clone()])))) {
    return py_str("");
    }
    return without_what_was_just_said(&[_a_rate_against_the_goal(&[aim.clone(), rate.clone(), memory.clone()]), memory.clone()]);
    Py::None
}
pub fn a_length_at_the_rate_in_the_chat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The money question about the goal in the line at the rate already in the chat, or \"\".\n\n    Written as the question the money entry answers in full (\"how long will it take to save 5000\n    at 200 a month\"), because the sum is the entry's - this reads only that the rate is the\n    chat's own and the line did not name another one.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_A_LENGTH(), "search", &[line.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_RATE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_NEW_RATE(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_RATE_WITH_A_VERB(), "match", &[line.clone()]) } } } }) {
    return py_str("");
    }
    if py_truthy(&_the_date_in_the_chat(&[memory.clone()])) {
    return py_str("");
    }
    said = _a_sum(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    rate = _the_rate_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { op_ge(&rate.clone(), &said.clone()) } }) {
    return py_str("");
    }
    return op_modulo(&py_str("How long will it take to save %s at %s a month?"), &py_tuple(vec![call_builtin("format", &[said.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    Py::None
}
pub fn a_rate_with_no_date(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The rate asked for when the goal is in the chat but no date is, or \"\".\n\n    \"how much a month?\" with no month named is a question no entry can work out: the rate is the\n    sum over however many months they have in mind. Asking for that is the one honest answer, and\n    it is better than the figure for the goal alone, which the chat has already had.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_HOW_MUCH(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { { let __b = _a_sum(&[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } } } } } }) {
    return py_str("");
    }
    aim = _the_goal_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&aim.clone())); if py_truthy(&__b) { __b } else { _the_date_in_the_chat(&[memory.clone()]) } }) {
    return py_str("");
    }
    return op_modulo(&py_str("£%s is a different rate over six months than over a year, so name the month you want it by and I will work out what to put by each month — and whether a standing order gets you there."), &call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn asks_the_rate(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line asks for the rate and names neither the goal nor the date.\n\n    The money reading of a line has to go before the general one for this shape: \"how much a\n    month?\" is about the goal said two turns ago, and read on its own it is about nothing.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_HOW_MUCH(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { _a_sum(&[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[_the_goal_in_the_chat(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _in_the_money_readers_words(args: &[Py]) -> Py {
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
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut season: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The date as the knowledge base can count it: a named day becomes the month it falls in.\n\n    \"The end of\" is carried through, because it is the whole of the difference between a goal\n    said \"by december\" and one said \"by the end of december\", and dropping it asked the money\n    entry for a fortnight's saving when there were seven weeks: \"i need to save 3000 by the end\n    of the year\" was answered \"£3,000 by december is 20 days away\" from the first of the month.\n    A season's end is the last day of its last month, which is a date, so \"the end of autumn\" is\n    the end of November rather than the September an autumn starts in.\n    ");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__KNOWN_TO_THE_MONEY_ENTRY(), &word.clone())) } } } }) {
    return word.clone();
    }
    if py_truthy(&mcall(&word.clone(), "startswith", &[py_str("the end of ")])) {
    rest = mcall(&slice_or_raise(&word.clone(), &call_builtin("len", &[py_str("the end of ")]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("next year"), py_str("this year"), py_str("the year")]), &rest.clone()))) {
    return word.clone();
    }
    season = if py_truthy(&mcall(&rest.clone(), "startswith", &[py_str("the ")])) { slice_or_raise(&rest.clone(), &Py::Int(4), &Py::None, &Py::None) } else { rest.clone() };
    month = { let __b = mcall(&g__THE_END_OF_A_SEASON(), "get", &[season.clone()]); if py_truthy(&__b) { __b } else { _in_the_money_readers_words(&[rest.clone()]) } };
    return if py_truthy(&month.clone()) { op_add(&py_str("the end of "), &month.clone()) } else { word.clone() };
    }
    let __iter1 = py_iter_or_raise(&g_NAMED_WHEN());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    month = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &name.clone()))) {
    return month.clone();
    }
    }
    return word.clone();
    Py::None
}
pub fn rate_question(args: &[Py]) -> Py {
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
    let mut aim: Py = Py::None;
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The money question this line really is, or \"\" when it is not one.\n\n    A goal with a sum is the question, and a date on its own is the same question about the sum\n    said a line or two ago - which is the follow-up people actually make: \"i want to save 2000\n    pounds\" / \"by june\".\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[])))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_HOW_MUCH(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_sum(&[line.clone()]))) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![_the_goal_in_the_chat(&[memory.clone()]), _the_date_in_the_chat(&[memory.clone()])]), 2, None);
    aim = __unpk1[0].clone();
    when = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&aim.clone()))) {
    return py_str("");
    }
    if py_truthy(&when.clone()) {
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds by %s?"), &py_tuple(vec![call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _in_the_money_readers_words(&[when.clone()])]));
    }
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds?"), &call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    }
    if py_truthy(&_names_a_goal(&[line.clone()])) {
    let __unpk2 = py_unpack(&py_tuple(vec![_names_a_goal(&[line.clone()]), _a_date(&[line.clone()])]), 2, None);
    aim = __unpk2[0].clone();
    when = __unpk2[1].clone();
    } else {
    when = _only_a_date(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&when.clone())); if py_truthy(&__b) { __b } else { _a_rate_is_settled(&[memory.clone()]) } }) {
    return py_str("");
    }
    aim = _the_goal_in_the_chat(&[memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&aim.clone()))) {
    return py_str("");
    }
    held = { let __b = geti(&the_pot_and_the_rest(&[line.clone()]), &Py::Int(0)); if py_truthy(&__b) { __b } else { _the_money_already_held(&[memory.clone()]) } };
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { { let __cmp31: Py = held.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp31)) { op_lt(&__cmp31, &aim.clone()) } else { Py::Bool(false) } } } }) {
    aim = op_sub(&aim.clone(), &held.clone());
    }
    if py_truthy(&when.clone()) {
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds by %s?"), &py_tuple(vec![call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _in_the_money_readers_words(&[when.clone()])]));
    }
    return op_modulo(&py_str("How much do i need to put by a week to have %s pounds?"), &call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _a_rate_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The sum a month the person says they will put by, and whether they named the period.\n\n    \"im putting by 500 a month\" names it; \"and if i put away 300\" right after the goal does not,\n    and in a chat about a saving goal the period is a month - nobody puts by 300 a year. Read as a\n    goal the line names a different pot (\"For £300: £100 a month is 3 months\") and works out a\n    number about the wrong figure, which is worse than saying nothing.\n    ");
    found = mcall(&g_A_RATE(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&found.clone()) {
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_tuple(vec![call_builtin("int", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(true)])]);
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
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    if py_truthy(&{ let __b = mcall(&g_INTENT(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("£|\\bpounds?\\b|\\bquid\\b"), { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
    }
    rest = mcall(&mcall(&g_FILLER_LEAD(), "sub", &[py_str(""), call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), "strip", &[py_str(" ,.")]);
    verb = { let __b = mcall(&g_RATE_GIVEN(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RATE_VERB(), "match", &[rest.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&verb.clone()))) {
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
    }
    if py_truthy(&{ let __b = mcall(&g_SAVE_VERB(), "match", &[rest.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CONDITIONAL(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } }) {
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
    }
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&verb.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.")]);
    match_ = mcall(&g_BARE_RATE(), "fullmatch", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
    }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_tuple(vec![call_builtin("int", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(false)])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["ValueError"]) {
                __handled10 = true;
    return py_tuple(vec![Py::Int(0), Py::Bool(false)]);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn a_rate_for_the_goal(args: &[Py]) -> Py {
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
    let mut _period: Py = Py::None;
    let mut against: Py = Py::None;
    let mut closing: Py = Py::None;
    let mut found: Py = Py::None;
    let mut goal: Py = Py::None;
    let mut line: Py = Py::None;
    let mut months: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The arithmetic for a rate the person states, when the goal is already in the chat, or \"\".\n\n    \"i want to save 2000 pounds\" / \"im putting by 500 a month\" is one question: at that rate the\n    goal takes four months. The line names no date, so it is not asked - but the answer is\n    arithmetic the person cannot do in their head, and it is the one thing they wanted.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_RATE_GIVEN(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_RATE_VERB(), "search", &[line.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _only_a_sum(&[line.clone()]); if py_truthy(&__b) { __b } else { _only_a_date(&[line.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bfor\\s+(?:\\d|a|an|two|three|four|five|six)"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&_a_rate_said(&[line.clone()]), 2, None);
    rate = __unpk1[0].clone();
    _period = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&rate.clone()))) {
    return py_str("");
    }
    found = mcall(&g_A_RATE(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    tail = slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&{ let __b = mcall(&g_NOT_A_POT(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OTHER_POT(), "match", &[tail.clone()]) } }) {
    return py_str("");
    }
    }
    goal = _the_money_still_to_find(&[memory.clone()]);
    if py_truthy(&op_le(&goal.clone(), &rate.clone())) {
    return py_str("");
    }
    when = _the_date_in_the_chat(&[memory.clone()]);
    if py_truthy(&when.clone()) {
    against = _a_date_on_a_rate(&[goal.clone(), rate.clone(), when.clone()]);
    if py_truthy(&against.clone()) {
    return against.clone();
    }
    }
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    closing = _a_closer(&[memory.clone()]);
    said = op_modulo(&py_str("£%s at £%s a month is %s."), &py_tuple(vec![call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _months(&[months.clone()])]));
    return op_add(&said.clone(), &if py_truthy(&closing.clone()) { op_add(&py_str(" "), &closing.clone()) } else { py_str("") });
    Py::None
}
pub fn _the_added_sum(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The sum the line adds to the rate, or 0 when it names none.");
    found = stdlib_call("re", "search", &[py_str("(?:£\\s*)?(\\d[\\d,]*(?:\\.\\d+)?)\\s*(k|grand|thousand)?"), { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Int(0);
    }
    let __try1 = exc::catch(|| {
    value = call_builtin("float", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand"), py_str("thousand")]), &mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    let __aug6 = op_mul(&value.clone(), &Py::Int(1000));
    value = __aug6;
    }
    value = call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_lt(&value.clone(), &Py::Int(5)); if py_truthy(&__b) { __b } else { op_gt(&value.clone(), &Py::Int(5000)) } }) {
    return Py::Int(0);
    }
    return value.clone();
    Py::None
}
pub fn a_rate_raised(args: &[Py]) -> Py {
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
    let mut added: Py = Py::None;
    let mut goal: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut raised: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut restated: Py = Py::None;
    let _ = py_str("The goal worked out again at the rate the line raises it to, or \"\".\n\n    \"i can put by 400 a month\" / \"and if i cut the gym thats another 40\" is the same question at\n    £440, and it is the arithmetic the person wants: the months, not the £40 on its own.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))) } }) {
    return py_str("");
    }
    goal = { let __b = _the_money_still_to_find(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_goal_in_the_answer(&[memory.clone()]) } };
    rate = _the_rate_in_the_chat(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&goal.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { op_le(&goal.clone(), &rate.clone()) } } } }) {
    return py_str("");
    }
    restated = mcall(&g_A_RESTATED_RATE(), "match", &[line.clone()]);
    if py_truthy(&restated.clone()) {
    let __try1 = exc::catch(|| {
    named = call_builtin("int", &[mcall(&mcall(&restated.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = op_le(&named.clone(), &rate.clone()); if py_truthy(&__b) { __b } else { op_ge(&named.clone(), &goal.clone()) } }) {
    return py_str("");
    }
    if py_truthy(&_said_a_rate_at(&[named.clone(), memory.clone()])) {
    return op_modulo(&py_str("That is the rate: put the standing order in for £%s the day after payday and the £%s takes care of itself from there."), &py_tuple(vec![call_builtin("format", &[named.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    }
    return _a_rate_against_the_goal(&[goal.clone(), named.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ADDED(), "search", &[line.clone()])))) {
    return py_str("");
    }
    if py_truthy(&_names_a_goal(&[line.clone()])) {
    return py_str("");
    }
    added = _the_added_sum(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&added.clone()))) {
    return py_str("");
    }
    raised = op_add(&rate.clone(), &added.clone());
    if py_truthy(&op_ge(&raised.clone(), &goal.clone())) {
    return py_str("");
    }
    return _a_rate_against_the_goal(&[goal.clone(), raised.clone(), memory.clone()]);
    Py::None
}
pub fn _said_a_rate_at(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last answer already worked the goal out at this rate.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_modulo(&py_str("£\\s*%s\\s*a\\s+month"), &mcall(&call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str(","), py_str(",?")])), last.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_rate_against_the_goal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut goal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "goal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: goal"))) };
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut closing: Py = Py::None;
    let mut months: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The months the goal takes at this rate, and a closing line the reply above did not use.");
    months = op_neg(&op_floordiv(&op_neg(&goal.clone()), &rate.clone()));
    said = op_modulo(&py_str("£%s at £%s a month is %s."), &py_tuple(vec![call_builtin("format", &[goal.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _months(&[months.clone()])]));
    closing = _a_closer(&[memory.clone()]);
    return op_add(&said.clone(), &if py_truthy(&closing.clone()) { op_add(&py_str(" "), &closing.clone()) } else { py_str("") });
    Py::None
}
pub fn _a_closer(args: &[Py]) -> Py {
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
    let mut closing: Py = Py::None;
    let mut here: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A closing line the reply just above did not already use, or \"\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    last = if py_truthy(&turns.clone()) { mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    said = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), last.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_SAVING_CLOSERS());
    for __item in __iter1 {
    closing = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&closing.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    here = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&here.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    return closing.clone();
    }
    return py_str("");
    Py::None
}
pub fn _a_said_sentence(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut here: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when most of the words of this sentence are in the reply above.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    here = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return op_ge(&op_mul(&here.clone(), &Py::Int(4)), &op_mul(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)));
    Py::None
}
pub fn said_already(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut first: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the first sentence of `reply` is one the answer above already said.\n\n    \"we want to save 3000 pounds\" / \"by december\" / \"how much should i put by a month\" asks the\n    same question a second time, and the entry answers it with the same sentence - it is one pay\n    packet either way. `without_what_was_just_said` can only take the tail off, because its first\n    sentence is normally the part that is new, so the repeat itself has to be recognised here and\n    answered in the unit the person asked for.\n    ");
    first = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    said = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_said_sentence(&[first.clone(), said.clone()]) } };
    Py::None
}
pub fn _weeks_left(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut month: Py = Py::None;
    let mut number: Py = Py::None;
    let mut today: Py = Py::None;
    let mut year: Py = Py::None;
    let _ = py_str("The weeks from this one to the month the person named, rounded up: a week left is a week.");
    datetime = stdlib_module("datetime");
    month = _month_of(&[when.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&month.clone()))) {
    return Py::Int(0);
    }
    today = mcall(&gattr(&datetime.clone(), "date"), "today", &[]);
    number = op_add(&mcall(&g_MONTHS(), "index", &[month.clone()]), &Py::Int(1));
    year = op_add(&gattr(&today.clone(), "year"), &if py_truthy(&op_le(&number.clone(), &gattr(&today.clone(), "month"))) { Py::Int(1) } else { Py::Int(0) });
    days = gattr(&op_sub(&mcall(&datetime.clone(), "date", &[year.clone(), number.clone(), Py::Int(1)]), &today.clone()), "days");
    return call_builtin("max", &[op_neg(&op_floordiv(&op_neg(&days.clone()), &Py::Int(7))), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_figure_each_week(args: &[Py]) -> Py {
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
    let mut a_week: Py = Py::None;
    let mut aim: Py = Py::None;
    let mut closing: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut weeks: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("The goal divided by the weeks left, said as one figure, or \"\".\n\n    The last resort for \"how much a month\" when the answer above already said the goal in months:\n    the same requirement said in the unit that comes out of a pay packet is a figure they did not\n    have yet, where the sentence they just read again is nothing at all.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![_the_money_still_to_find(&[memory.clone()]), _the_date_in_the_chat(&[memory.clone()])]), 2, None);
    aim = __unpk1[0].clone();
    when = __unpk1[1].clone();
    weeks = _weeks_left(&[when.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&aim.clone())); if py_truthy(&__b) { __b } else { op_lt(&weeks.clone(), &Py::Int(2)) } }) {
    return py_str("");
    }
    a_week = op_neg(&op_floordiv(&op_neg(&aim.clone()), &weeks.clone()));
    said = Py::None;
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    found = if py_truthy(&turns.clone()) { mcall(&g_A_MONTH_FIGURE(), "search", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } else { Py::None };
    if py_truthy(&found.clone()) {
    said = op_modulo(&py_str("£%s a month is"), &mcall(&found.clone(), "group", &[Py::Int(1)]));
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = op_modulo(&py_str("£%s by %s is"), &py_tuple(vec![call_builtin("format", &[aim.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), mcall(&call_builtin("str", &[when.clone()]).unwrap_or_else(|e| exc::raise(e)), "capitalize", &[])]));
    }
    let __aug2 = op_add(&said.clone(), &op_modulo(&py_str(" about £%s a week over the %s weeks left"), &py_tuple(vec![call_builtin("format", &[a_week.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[weeks.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))])));
    said = __aug2;
    closing = _a_closer(&[memory.clone()]);
    if py_truthy(&closing.clone()) {
    return op_add(&op_add(&op_add(&said.clone(), &py_str(" — ")), &mcall(&geti(&closing.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&closing.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return op_add(&said.clone(), &py_str("."));
    Py::None
}
pub fn the_figure_a_week(args: &[Py]) -> Py {
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
    let _ = py_str("The question that gets the figure each week, or \"\" when there is nothing to divide.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_the_goal_in_the_chat(&[memory.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_date_in_the_chat(&[memory.clone()]))) } }) {
    return py_str("");
    }
    return g_A_WEEK_ASK();
    Py::None
}
pub fn without_what_was_just_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("`reply` with any sentence of it the reply above already said taken off.\n\n    Only whole sentences go, and never the first: the answer itself is new - the rate is another\n    rate - so what is left is the figure with the same closing on it, and the closing is dropped.\n    A reply that is one sentence, or that says something wholly new, comes back untouched.\n    ");
    reply = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&reply.clone(), &py_str("\n")))) {
    return reply.clone();
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return reply.clone();
    }
    said = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return reply.clone();
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&reply.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(part.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return reply.clone();
    }
    kept = op_add(&py_list(vec![geti(&parts.clone(), &Py::Int(0))]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut part: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_a_said_sentence(&[part.clone(), said.clone()])))) {  __out.push(part.clone()); } } py_list(__out) })() });
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn _the_rate_put_by(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let mut window: Py = Py::None;
    let _ = py_str("The sum a month the person put by, from the last few lines, or 0.\n\n    `_the_rate_in_the_chat` reads any monthly figure, because it is only ever reached from a chat\n    already about saving. This reader is reached from an ordinary chat - \"how long until i can buy\n    a bike\" is not about saving - so the rate has to be recognisable as money put by, and a cost or\n    a wage has to count for nothing.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    window = slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&window.clone()) { let mut turn: Py = __item.clone(); __out.push(mcall(&g_PUT_BY(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[window.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    found = mcall(&g_A_RATE(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { mcall(&g_MONEY_OUT(), "search", &[line.clone()]) } }) {
    continue;
    }
    let __try2 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&op_ge(&value.clone(), &Py::Int(10))) {
    return value.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn a_length_with_no_price(args: &[Py]) -> Py {
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
    let mut a_year: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut said: Py = Py::None;
    let mut six_months: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The rate said as the money it becomes, and the price asked for, or \"\".\n\n    Only when the length is asked and the price is missing: a line with a figure in it is the\n    money entry's (\"how long until i can afford 600\"), and a chat with a goal in it is the\n    arithmetic on the goal (`a_length_at_the_rate_in_the_chat`).\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&line.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_A_LENGTH_FOR_A_THING(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_THING_WANTED(), "search", &[line.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    rate = _the_rate_put_by(&[memory.clone()]);
    if py_truthy(&op_lt(&rate.clone(), &Py::Int(10))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _the_goal_in_the_chat(&[memory.clone()]); if py_truthy(&__b) { __b } else { _the_goal_in_the_answer(&[memory.clone()]) } }) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![op_mul(&rate.clone(), &Py::Int(6)), op_mul(&rate.clone(), &Py::Int(12))]), 2, None);
    six_months = __unpk1[0].clone();
    a_year = __unpk1[1].clone();
    said = op_modulo(&py_str("£%s a month is £%s every six months, £%s in a year."), &py_tuple(vec![call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[six_months.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[a_year.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e))]));
    named = mcall(&g_A_THING_NAMED(), "search", &[line.clone()]);
    thing = py_str("");
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING(), &mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    thing = op_add(&py_str(" "), &mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]));
    }
    asked = if py_truthy(&thing.clone()) { op_modulo(&py_str("What's the%s going for?"), &thing.clone()) } else { py_str("What's the price on it?") };
    if py_truthy(&_already_asked_for_the_price(&[memory.clone()])) {
    return op_add(&said.clone(), &py_str(" Tell me the price and I'll do the weeks."));
    }
    return op_add(&op_add(&said.clone(), &py_str(" ")), &asked.clone());
    Py::None
}
pub fn _already_asked_for_the_price(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the answer above already asked what the thing costs.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    above = mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bwhat(?:'s| is) (?:the |it |that )?(?:.* )?going for\\b|\\bwhat'?s the price\\b|\\bwhat does it cost\\b|\\bhow much is it\\b"), above.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_price_in_an_answer(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut months: Py = Py::None;
    let mut price: Py = Py::None;
    let mut rate: Py = Py::None;
    let _ = py_str("The arithmetic for a price given in reply to the question \"what's it going for?\", or \"\".\n\n    A price on its own line is a sum and nothing else, so read as a goal it would be answered as\n    \"For £200: £100 a month is 2 months...\" - a fresh set of rates for someone who has already\n    said what they can put by. It follows a question about the price, so the answer is the sum on\n    the rate already in the chat: £200 at £50 a month is four months.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } } } }) {
    return py_str("");
    }
    price = _a_sum(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&price.clone())); if py_truthy(&__b) { __b } else { { let __b = _only_a_date(&[line.clone()]); if py_truthy(&__b) { __b } else { _a_date(&[line.clone()]) } } } }) {
    return py_str("");
    }
    rate = _the_rate_put_by(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rate.clone())); if py_truthy(&__b) { __b } else { op_ge(&rate.clone(), &price.clone()) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_already_asked_for_the_price(&[memory.clone()])))) {
    return py_str("");
    }
    months = op_neg(&op_floordiv(&op_neg(&price.clone()), &rate.clone()));
    return op_modulo(&py_str("£%s at £%s a month is %s."), &py_tuple(vec![call_builtin("format", &[price.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), call_builtin("format", &[rate.clone(), py_str(",")]).unwrap_or_else(|e| exc::raise(e)), _months(&[months.clone()])]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.saving".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADDED" => Some(g_ADDED()),
            "AIM_AT_A_SUM" => Some(g_AIM_AT_A_SUM()),
            "ALREADY" => Some(g_ALREADY()),
            "ALREADY_AFTER" => Some(g_ALREADY_AFTER()),
            "ALREADY_HELD" => Some(g_ALREADY_HELD()),
            "ANOTHER_SUM" => Some(g_ANOTHER_SUM()),
            "ASKS_A_LENGTH" => Some(g_ASKS_A_LENGTH()),
            "ASKS_A_LENGTH_FOR_A_THING" => Some(g_ASKS_A_LENGTH_FOR_A_THING()),
            "ASKS_HOW_MUCH_MORE" => Some(g_ASKS_HOW_MUCH_MORE()),
            "ASKS_IF_ENOUGH" => Some(g_ASKS_IF_ENOUGH()),
            "AT_A_FIGURE" => Some(g_AT_A_FIGURE()),
            "A_CAN_CARRY_IT" => Some(g_A_CAN_CARRY_IT()),
            "A_CORRECTION_AND_A_FIGURE" => Some(g_A_CORRECTION_AND_A_FIGURE()),
            "A_GOAL_STATED" => Some(g_A_GOAL_STATED()),
            "A_GOAL_SUM" => Some(g_A_GOAL_SUM()),
            "A_LENGTH_SAID" => Some(g_A_LENGTH_SAID()),
            "A_LENGTH_WAS_ASKED" => Some(g_A_LENGTH_WAS_ASKED()),
            "A_LONE_SUM" => Some(g_A_LONE_SUM()),
            "A_MONTH_FIGURE" => Some(g_A_MONTH_FIGURE()),
            "A_MORE_LIKE" => Some(g_A_MORE_LIKE()),
            "A_NEW_RATE" => Some(g_A_NEW_RATE()),
            "A_NO_THEN_A_RATE" => Some(g_A_NO_THEN_A_RATE()),
            "A_RATE" => Some(g_A_RATE()),
            "A_RATE_ANY_OTHER_PERIOD" => Some(g_A_RATE_ANY_OTHER_PERIOD()),
            "A_RATE_PUT_RIGHT" => Some(g_A_RATE_PUT_RIGHT()),
            "A_RATE_WITH_A_VERB" => Some(g_A_RATE_WITH_A_VERB()),
            "A_RESTATED_RATE" => Some(g_A_RESTATED_RATE()),
            "A_SPENT_ON" => Some(g_A_SPENT_ON()),
            "A_SUM_A_MONTH" => Some(g_A_SUM_A_MONTH()),
            "A_THING_NAMED" => Some(g_A_THING_NAMED()),
            "A_TRAILING_SUM" => Some(g_A_TRAILING_SUM()),
            "A_WEEK_ASK" => Some(g_A_WEEK_ASK()),
            "A_WEEK_FIGURE" => Some(g_A_WEEK_FIGURE()),
            "BARE_DATE" => Some(g_BARE_DATE()),
            "BARE_END_OF_RELATIVE" => Some(g_BARE_END_OF_RELATIVE()),
            "BARE_FIGURE" => Some(g_BARE_FIGURE()),
            "BARE_RATE" => Some(g_BARE_RATE()),
            "BY_A_DATE" => Some(g_BY_A_DATE()),
            "CLAUSE_EDGE" => Some(g_CLAUSE_EDGE()),
            "CONDITIONAL" => Some(g_CONDITIONAL()),
            "CORRECTING" => Some(g_CORRECTING()),
            "DATE" => Some(g_DATE()),
            "END_OF" => Some(g_END_OF()),
            "END_OF_RELATIVE" => Some(g_END_OF_RELATIVE()),
            "FILLER_LEAD" => Some(g_FILLER_LEAD()),
            "FURTHER_SAID" => Some(g_FURTHER_SAID()),
            "HELD" => Some(g_HELD()),
            "HELD_SAID" => Some(g_HELD_SAID()),
            "HOW_FAR_OFF" => Some(g_HOW_FAR_OFF()),
            "HOW_LONG_ASK" => Some(g_HOW_LONG_ASK()),
            "HOW_LONG_UNTIL" => Some(g_HOW_LONG_UNTIL()),
            "HOW_MUCH" => Some(g_HOW_MUCH()),
            "INCOME_FRAME" => Some(g_INCOME_FRAME()),
            "INTENT" => Some(g_INTENT()),
            "MONEY_OUT" => Some(g_MONEY_OUT()),
            "MONTHS" => Some(g_MONTHS()),
            "MONTHS_IN_A" => Some(g_MONTHS_IN_A()),
            "NAMED_WHEN" => Some(g_NAMED_WHEN()),
            "NEED_A_FIGURE" => Some(g_NEED_A_FIGURE()),
            "NOT_A_POT" => Some(g_NOT_A_POT()),
            "NOT_A_PRICE" => Some(g_NOT_A_PRICE()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "OTHER_POT" => Some(g_OTHER_POT()),
            "OUTGOING_FRAME" => Some(g_OUTGOING_FRAME()),
            "PAST_SAVING" => Some(g_PAST_SAVING()),
            "POT_OPENER" => Some(g_POT_OPENER()),
            "POT_REPORTED" => Some(g_POT_REPORTED()),
            "PRICE_WORDS" => Some(g_PRICE_WORDS()),
            "PUT_BY" => Some(g_PUT_BY()),
            "RATE_GIVEN" => Some(g_RATE_GIVEN()),
            "RATE_VERB" => Some(g_RATE_VERB()),
            "SAVE_VERB" => Some(g_SAVE_VERB()),
            "SAVING" => Some(g_SAVING()),
            "SAVING_CLOSERS" => Some(g_SAVING_CLOSERS()),
            "SUM" => Some(g_SUM()),
            "THE_THING_WANTED" => Some(g_THE_THING_WANTED()),
            "TOGETHER" => Some(g_TOGETHER()),
            "WHEN_AT_THE_GOAL" => Some(g_WHEN_AT_THE_GOAL()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "_KNOWN_TO_THE_MONEY_ENTRY" => Some(g__KNOWN_TO_THE_MONEY_ENTRY()),
            "_NOT_THE_RATE" => Some(g__NOT_THE_RATE()),
            "_OTHER_FIGURE_WORDS" => Some(g__OTHER_FIGURE_WORDS()),
            "_THE_END_OF_A_SEASON" => Some(g__THE_END_OF_A_SEASON()),
            "_a_closer" => Some(py_func("_a_closer", |args: &[Py]| _a_closer(args))),
            "_a_date" => Some(py_func("_a_date", |args: &[Py]| _a_date(args))),
            "_a_date_on_a_rate" => Some(py_func("_a_date_on_a_rate", |args: &[Py]| _a_date_on_a_rate(args))),
            "_a_length_was_asked" => Some(py_func("_a_length_was_asked", |args: &[Py]| _a_length_was_asked(args))),
            "_a_rate" => Some(py_func("_a_rate", |args: &[Py]| _a_rate(args))),
            "_a_rate_against_the_goal" => Some(py_func("_a_rate_against_the_goal", |args: &[Py]| _a_rate_against_the_goal(args))),
            "_a_rate_in_another_period" => Some(py_func("_a_rate_in_another_period", |args: &[Py]| _a_rate_in_another_period(args))),
            "_a_rate_is_settled" => Some(py_func("_a_rate_is_settled", |args: &[Py]| _a_rate_is_settled(args))),
            "_a_rate_said" => Some(py_func("_a_rate_said", |args: &[Py]| _a_rate_said(args))),
            "_a_said_sentence" => Some(py_func("_a_said_sentence", |args: &[Py]| _a_said_sentence(args))),
            "_a_sum" => Some(py_func("_a_sum", |args: &[Py]| _a_sum(args))),
            "_a_sum_named" => Some(py_func("_a_sum_named", |args: &[Py]| _a_sum_named(args))),
            "_a_sum_that_is_a_goal" => Some(py_func("_a_sum_that_is_a_goal", |args: &[Py]| _a_sum_that_is_a_goal(args))),
            "_already_asked_for_the_price" => Some(py_func("_already_asked_for_the_price", |args: &[Py]| _already_asked_for_the_price(args))),
            "_in_the_money_readers_words" => Some(py_func("_in_the_money_readers_words", |args: &[Py]| _in_the_money_readers_words(args))),
            "_is_a_rate_line" => Some(py_func("_is_a_rate_line", |args: &[Py]| _is_a_rate_line(args))),
            "_is_an_income" => Some(py_func("_is_an_income", |args: &[Py]| _is_an_income(args))),
            "_is_an_outgoing" => Some(py_func("_is_an_outgoing", |args: &[Py]| _is_an_outgoing(args))),
            "_is_not_a_rate" => Some(py_func("_is_not_a_rate", |args: &[Py]| _is_not_a_rate(args))),
            "_month_of" => Some(py_func("_month_of", |args: &[Py]| _month_of(args))),
            "_months" => Some(py_func("_months", |args: &[Py]| _months(args))),
            "_months_left" => Some(py_func("_months_left", |args: &[Py]| _months_left(args))),
            "_names_a_goal" => Some(py_func("_names_a_goal", |args: &[Py]| _names_a_goal(args))),
            "_only_a_date" => Some(py_func("_only_a_date", |args: &[Py]| _only_a_date(args))),
            "_only_a_sum" => Some(py_func("_only_a_sum", |args: &[Py]| _only_a_sum(args))),
            "_pounds" => Some(py_func("_pounds", |args: &[Py]| _pounds(args))),
            "_said_a_rate_at" => Some(py_func("_said_a_rate_at", |args: &[Py]| _said_a_rate_at(args))),
            "_the_added_sum" => Some(py_func("_the_added_sum", |args: &[Py]| _the_added_sum(args))),
            "_the_date_in_the_chat" => Some(py_func("_the_date_in_the_chat", |args: &[Py]| _the_date_in_the_chat(args))),
            "_the_gap_again" => Some(py_func("_the_gap_again", |args: &[Py]| _the_gap_again(args))),
            "_the_gap_said" => Some(py_func("_the_gap_said", |args: &[Py]| _the_gap_said(args))),
            "_the_gap_was_just_said" => Some(py_func("_the_gap_was_just_said", |args: &[Py]| _the_gap_was_just_said(args))),
            "_the_goal_in_the_answer" => Some(py_func("_the_goal_in_the_answer", |args: &[Py]| _the_goal_in_the_answer(args))),
            "_the_goal_in_the_chat" => Some(py_func("_the_goal_in_the_chat", |args: &[Py]| _the_goal_in_the_chat(args))),
            "_the_goal_named_in_a_line" => Some(py_func("_the_goal_named_in_a_line", |args: &[Py]| _the_goal_named_in_a_line(args))),
            "_the_goal_said_as_a_price" => Some(py_func("_the_goal_said_as_a_price", |args: &[Py]| _the_goal_said_as_a_price(args))),
            "_the_goal_they_need" => Some(py_func("_the_goal_they_need", |args: &[Py]| _the_goal_they_need(args))),
            "_the_income_in_the_chat" => Some(py_func("_the_income_in_the_chat", |args: &[Py]| _the_income_in_the_chat(args))),
            "_the_length_at_that_rate" => Some(py_func("_the_length_at_that_rate", |args: &[Py]| _the_length_at_that_rate(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_money_already_held" => Some(py_func("_the_money_already_held", |args: &[Py]| _the_money_already_held(args))),
            "_the_money_still_to_find" => Some(py_func("_the_money_still_to_find", |args: &[Py]| _the_money_still_to_find(args))),
            "_the_pot_they_are_on" => Some(py_func("_the_pot_they_are_on", |args: &[Py]| _the_pot_they_are_on(args))),
            "_the_rate_in_the_chat" => Some(py_func("_the_rate_in_the_chat", |args: &[Py]| _the_rate_in_the_chat(args))),
            "_the_rate_put_by" => Some(py_func("_the_rate_put_by", |args: &[Py]| _the_rate_put_by(args))),
            "_two_sums_added" => Some(py_func("_two_sums_added", |args: &[Py]| _two_sums_added(args))),
            "_weeks_left" => Some(py_func("_weeks_left", |args: &[Py]| _weeks_left(args))),
            "a_date_on_a_settled_rate" => Some(py_func("a_date_on_a_settled_rate", |args: &[Py]| a_date_on_a_settled_rate(args))),
            "a_figure_each_week" => Some(py_func("a_figure_each_week", |args: &[Py]| a_figure_each_week(args))),
            "a_length_at_the_rate_in_the_chat" => Some(py_func("a_length_at_the_rate_in_the_chat", |args: &[Py]| a_length_at_the_rate_in_the_chat(args))),
            "a_length_was_said" => Some(py_func("a_length_was_said", |args: &[Py]| a_length_was_said(args))),
            "a_length_with_no_price" => Some(py_func("a_length_with_no_price", |args: &[Py]| a_length_with_no_price(args))),
            "a_price_in_an_answer" => Some(py_func("a_price_in_an_answer", |args: &[Py]| a_price_in_an_answer(args))),
            "a_rate_for_the_goal" => Some(py_func("a_rate_for_the_goal", |args: &[Py]| a_rate_for_the_goal(args))),
            "a_rate_put_in_with_a_verb" => Some(py_func("a_rate_put_in_with_a_verb", |args: &[Py]| a_rate_put_in_with_a_verb(args))),
            "a_rate_put_right" => Some(py_func("a_rate_put_right", |args: &[Py]| a_rate_put_right(args))),
            "a_rate_raised" => Some(py_func("a_rate_raised", |args: &[Py]| a_rate_raised(args))),
            "a_rate_restated" => Some(py_func("a_rate_restated", |args: &[Py]| a_rate_restated(args))),
            "a_rate_with_no_date" => Some(py_func("a_rate_with_no_date", |args: &[Py]| a_rate_with_no_date(args))),
            "a_yes_no_about_the_rate" => Some(py_func("a_yes_no_about_the_rate", |args: &[Py]| a_yes_no_about_the_rate(args))),
            "another_sum_asked" => Some(py_func("another_sum_asked", |args: &[Py]| another_sum_asked(args))),
            "asks_about_saving" => Some(py_func("asks_about_saving", |args: &[Py]| asks_about_saving(args))),
            "asks_how_far_off" => Some(py_func("asks_how_far_off", |args: &[Py]| asks_how_far_off(args))),
            "asks_how_long" => Some(py_func("asks_how_long", |args: &[Py]| asks_how_long(args))),
            "asks_how_much_more_a_month" => Some(py_func("asks_how_much_more_a_month", |args: &[Py]| asks_how_much_more_a_month(args))),
            "asks_if_the_rate_is_enough" => Some(py_func("asks_if_the_rate_is_enough", |args: &[Py]| asks_if_the_rate_is_enough(args))),
            "asks_the_rate" => Some(py_func("asks_the_rate", |args: &[Py]| asks_the_rate(args))),
            "how_long_it_takes" => Some(py_func("how_long_it_takes", |args: &[Py]| how_long_it_takes(args))),
            "is_only_a_date" => Some(py_func("is_only_a_date", |args: &[Py]| is_only_a_date(args))),
            "rate_question" => Some(py_func("rate_question", |args: &[Py]| rate_question(args))),
            "rate_question_for" => Some(py_func("rate_question_for", |args: &[Py]| rate_question_for(args))),
            "said_already" => Some(py_func("said_already", |args: &[Py]| said_already(args))),
            "the_extra_a_month_asked" => Some(py_func("the_extra_a_month_asked", |args: &[Py]| the_extra_a_month_asked(args))),
            "the_figure_a_week" => Some(py_func("the_figure_a_week", |args: &[Py]| the_figure_a_week(args))),
            "the_gap_left" => Some(py_func("the_gap_left", |args: &[Py]| the_gap_left(args))),
            "the_goal_and_the_pot" => Some(py_func("the_goal_and_the_pot", |args: &[Py]| the_goal_and_the_pot(args))),
            "the_money_already_held" => Some(py_func("the_money_already_held", |args: &[Py]| the_money_already_held(args))),
            "the_pot_and_the_rest" => Some(py_func("the_pot_and_the_rest", |args: &[Py]| the_pot_and_the_rest(args))),
            "the_pot_position" => Some(py_func("the_pot_position", |args: &[Py]| the_pot_position(args))),
            "the_pot_said_alone" => Some(py_func("the_pot_said_alone", |args: &[Py]| the_pot_said_alone(args))),
            "the_pot_they_are_on" => Some(py_func("the_pot_they_are_on", |args: &[Py]| the_pot_they_are_on(args))),
            "the_same_figure_a_month" => Some(py_func("the_same_figure_a_month", |args: &[Py]| the_same_figure_a_month(args))),
            "without_the_pot_opener" => Some(py_func("without_the_pot_opener", |args: &[Py]| without_the_pot_opener(args))),
            "without_what_was_just_said" => Some(py_func("without_what_was_just_said", |args: &[Py]| without_what_was_just_said(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADDED" => s_ADDED(v),
            "AIM_AT_A_SUM" => s_AIM_AT_A_SUM(v),
            "ALREADY" => s_ALREADY(v),
            "ALREADY_AFTER" => s_ALREADY_AFTER(v),
            "ALREADY_HELD" => s_ALREADY_HELD(v),
            "ANOTHER_SUM" => s_ANOTHER_SUM(v),
            "ASKS_A_LENGTH" => s_ASKS_A_LENGTH(v),
            "ASKS_A_LENGTH_FOR_A_THING" => s_ASKS_A_LENGTH_FOR_A_THING(v),
            "ASKS_HOW_MUCH_MORE" => s_ASKS_HOW_MUCH_MORE(v),
            "ASKS_IF_ENOUGH" => s_ASKS_IF_ENOUGH(v),
            "AT_A_FIGURE" => s_AT_A_FIGURE(v),
            "A_CAN_CARRY_IT" => s_A_CAN_CARRY_IT(v),
            "A_CORRECTION_AND_A_FIGURE" => s_A_CORRECTION_AND_A_FIGURE(v),
            "A_GOAL_STATED" => s_A_GOAL_STATED(v),
            "A_GOAL_SUM" => s_A_GOAL_SUM(v),
            "A_LENGTH_SAID" => s_A_LENGTH_SAID(v),
            "A_LENGTH_WAS_ASKED" => s_A_LENGTH_WAS_ASKED(v),
            "A_LONE_SUM" => s_A_LONE_SUM(v),
            "A_MONTH_FIGURE" => s_A_MONTH_FIGURE(v),
            "A_MORE_LIKE" => s_A_MORE_LIKE(v),
            "A_NEW_RATE" => s_A_NEW_RATE(v),
            "A_NO_THEN_A_RATE" => s_A_NO_THEN_A_RATE(v),
            "A_RATE" => s_A_RATE(v),
            "A_RATE_ANY_OTHER_PERIOD" => s_A_RATE_ANY_OTHER_PERIOD(v),
            "A_RATE_PUT_RIGHT" => s_A_RATE_PUT_RIGHT(v),
            "A_RATE_WITH_A_VERB" => s_A_RATE_WITH_A_VERB(v),
            "A_RESTATED_RATE" => s_A_RESTATED_RATE(v),
            "A_SPENT_ON" => s_A_SPENT_ON(v),
            "A_SUM_A_MONTH" => s_A_SUM_A_MONTH(v),
            "A_THING_NAMED" => s_A_THING_NAMED(v),
            "A_TRAILING_SUM" => s_A_TRAILING_SUM(v),
            "A_WEEK_ASK" => s_A_WEEK_ASK(v),
            "A_WEEK_FIGURE" => s_A_WEEK_FIGURE(v),
            "BARE_DATE" => s_BARE_DATE(v),
            "BARE_END_OF_RELATIVE" => s_BARE_END_OF_RELATIVE(v),
            "BARE_FIGURE" => s_BARE_FIGURE(v),
            "BARE_RATE" => s_BARE_RATE(v),
            "BY_A_DATE" => s_BY_A_DATE(v),
            "CLAUSE_EDGE" => s_CLAUSE_EDGE(v),
            "CONDITIONAL" => s_CONDITIONAL(v),
            "CORRECTING" => s_CORRECTING(v),
            "DATE" => s_DATE(v),
            "END_OF" => s_END_OF(v),
            "END_OF_RELATIVE" => s_END_OF_RELATIVE(v),
            "FILLER_LEAD" => s_FILLER_LEAD(v),
            "FURTHER_SAID" => s_FURTHER_SAID(v),
            "HELD" => s_HELD(v),
            "HELD_SAID" => s_HELD_SAID(v),
            "HOW_FAR_OFF" => s_HOW_FAR_OFF(v),
            "HOW_LONG_ASK" => s_HOW_LONG_ASK(v),
            "HOW_LONG_UNTIL" => s_HOW_LONG_UNTIL(v),
            "HOW_MUCH" => s_HOW_MUCH(v),
            "INCOME_FRAME" => s_INCOME_FRAME(v),
            "INTENT" => s_INTENT(v),
            "MONEY_OUT" => s_MONEY_OUT(v),
            "MONTHS" => s_MONTHS(v),
            "MONTHS_IN_A" => s_MONTHS_IN_A(v),
            "NAMED_WHEN" => s_NAMED_WHEN(v),
            "NEED_A_FIGURE" => s_NEED_A_FIGURE(v),
            "NOT_A_POT" => s_NOT_A_POT(v),
            "NOT_A_PRICE" => s_NOT_A_PRICE(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "OTHER_POT" => s_OTHER_POT(v),
            "OUTGOING_FRAME" => s_OUTGOING_FRAME(v),
            "PAST_SAVING" => s_PAST_SAVING(v),
            "POT_OPENER" => s_POT_OPENER(v),
            "POT_REPORTED" => s_POT_REPORTED(v),
            "PRICE_WORDS" => s_PRICE_WORDS(v),
            "PUT_BY" => s_PUT_BY(v),
            "RATE_GIVEN" => s_RATE_GIVEN(v),
            "RATE_VERB" => s_RATE_VERB(v),
            "SAVE_VERB" => s_SAVE_VERB(v),
            "SAVING" => s_SAVING(v),
            "SAVING_CLOSERS" => s_SAVING_CLOSERS(v),
            "SUM" => s_SUM(v),
            "THE_THING_WANTED" => s_THE_THING_WANTED(v),
            "TOGETHER" => s_TOGETHER(v),
            "WHEN_AT_THE_GOAL" => s_WHEN_AT_THE_GOAL(v),
            "WHEN_WORDS" => s_WHEN_WORDS(v),
            "_KNOWN_TO_THE_MONEY_ENTRY" => s__KNOWN_TO_THE_MONEY_ENTRY(v),
            "_NOT_THE_RATE" => s__NOT_THE_RATE(v),
            "_OTHER_FIGURE_WORDS" => s__OTHER_FIGURE_WORDS(v),
            "_THE_END_OF_A_SEASON" => s__THE_END_OF_A_SEASON(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.saving", module_obj);
    pyrt::xrefs::reg_fn("memory.saving", "_a_sum", py_func("_a_sum", |args: &[Py]| _a_sum(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_date", py_func("_a_date", |args: &[Py]| _a_date(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_month_of", py_func("_month_of", |args: &[Py]| _month_of(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_two_sums_added", py_func("_two_sums_added", |args: &[Py]| _two_sums_added(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_pounds", py_func("_pounds", |args: &[Py]| _pounds(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_pot_and_the_rest", py_func("the_pot_and_the_rest", |args: &[Py]| the_pot_and_the_rest(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_pot_said_alone", py_func("the_pot_said_alone", |args: &[Py]| the_pot_said_alone(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_pot_they_are_on", py_func("the_pot_they_are_on", |args: &[Py]| the_pot_they_are_on(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_rate", py_func("_a_rate", |args: &[Py]| _a_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_pot_they_are_on", py_func("_the_pot_they_are_on", |args: &[Py]| _the_pot_they_are_on(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_goal_they_need", py_func("_the_goal_they_need", |args: &[Py]| _the_goal_they_need(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_how_far_off", py_func("asks_how_far_off", |args: &[Py]| asks_how_far_off(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_gap_left", py_func("the_gap_left", |args: &[Py]| the_gap_left(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_gap_said", py_func("_the_gap_said", |args: &[Py]| _the_gap_said(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_gap_was_just_said", py_func("_the_gap_was_just_said", |args: &[Py]| _the_gap_was_just_said(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_gap_again", py_func("_the_gap_again", |args: &[Py]| _the_gap_again(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_goal_and_the_pot", py_func("the_goal_and_the_pot", |args: &[Py]| the_goal_and_the_pot(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_pot_position", py_func("the_pot_position", |args: &[Py]| the_pot_position(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_sum_that_is_a_goal", py_func("_a_sum_that_is_a_goal", |args: &[Py]| _a_sum_that_is_a_goal(args)));
    pyrt::xrefs::reg_fn("memory.saving", "rate_question_for", py_func("rate_question_for", |args: &[Py]| rate_question_for(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_money_already_held", py_func("the_money_already_held", |args: &[Py]| the_money_already_held(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_money_already_held", py_func("_the_money_already_held", |args: &[Py]| _the_money_already_held(args)));
    pyrt::xrefs::reg_fn("memory.saving", "without_the_pot_opener", py_func("without_the_pot_opener", |args: &[Py]| without_the_pot_opener(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_names_a_goal", py_func("_names_a_goal", |args: &[Py]| _names_a_goal(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_how_long", py_func("asks_how_long", |args: &[Py]| asks_how_long(args)));
    pyrt::xrefs::reg_fn("memory.saving", "how_long_it_takes", py_func("how_long_it_takes", |args: &[Py]| how_long_it_takes(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_only_a_date", py_func("_only_a_date", |args: &[Py]| _only_a_date(args)));
    pyrt::xrefs::reg_fn("memory.saving", "is_only_a_date", py_func("is_only_a_date", |args: &[Py]| is_only_a_date(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_only_a_sum", py_func("_only_a_sum", |args: &[Py]| _only_a_sum(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_about_saving", py_func("asks_about_saving", |args: &[Py]| asks_about_saving(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_money_still_to_find", py_func("_the_money_still_to_find", |args: &[Py]| _the_money_still_to_find(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_goal_in_the_chat", py_func("_the_goal_in_the_chat", |args: &[Py]| _the_goal_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_goal_named_in_a_line", py_func("_the_goal_named_in_a_line", |args: &[Py]| _the_goal_named_in_a_line(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_goal_said_as_a_price", py_func("_the_goal_said_as_a_price", |args: &[Py]| _the_goal_said_as_a_price(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_length_was_said", py_func("a_length_was_said", |args: &[Py]| a_length_was_said(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_length_was_asked", py_func("_a_length_was_asked", |args: &[Py]| _a_length_was_asked(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_goal_in_the_answer", py_func("_the_goal_in_the_answer", |args: &[Py]| _the_goal_in_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.saving", "another_sum_asked", py_func("another_sum_asked", |args: &[Py]| another_sum_asked(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_rate_is_settled", py_func("_a_rate_is_settled", |args: &[Py]| _a_rate_is_settled(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_length_at_that_rate", py_func("_the_length_at_that_rate", |args: &[Py]| _the_length_at_that_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_rate_in_another_period", py_func("_a_rate_in_another_period", |args: &[Py]| _a_rate_in_another_period(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_is_an_income", py_func("_is_an_income", |args: &[Py]| _is_an_income(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_is_an_outgoing", py_func("_is_an_outgoing", |args: &[Py]| _is_an_outgoing(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_is_not_a_rate", py_func("_is_not_a_rate", |args: &[Py]| _is_not_a_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_income_in_the_chat", py_func("_the_income_in_the_chat", |args: &[Py]| _the_income_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_is_a_rate_line", py_func("_is_a_rate_line", |args: &[Py]| _is_a_rate_line(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_put_right", py_func("a_rate_put_right", |args: &[Py]| a_rate_put_right(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_sum_named", py_func("_a_sum_named", |args: &[Py]| _a_sum_named(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_rate_in_the_chat", py_func("_the_rate_in_the_chat", |args: &[Py]| _the_rate_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_date_in_the_chat", py_func("_the_date_in_the_chat", |args: &[Py]| _the_date_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_months_left", py_func("_months_left", |args: &[Py]| _months_left(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_months", py_func("_months", |args: &[Py]| _months(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_date_on_a_settled_rate", py_func("a_date_on_a_settled_rate", |args: &[Py]| a_date_on_a_settled_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_date_on_a_rate", py_func("_a_date_on_a_rate", |args: &[Py]| _a_date_on_a_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_if_the_rate_is_enough", py_func("asks_if_the_rate_is_enough", |args: &[Py]| asks_if_the_rate_is_enough(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_yes_no_about_the_rate", py_func("a_yes_no_about_the_rate", |args: &[Py]| a_yes_no_about_the_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_how_much_more_a_month", py_func("asks_how_much_more_a_month", |args: &[Py]| asks_how_much_more_a_month(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_extra_a_month_asked", py_func("the_extra_a_month_asked", |args: &[Py]| the_extra_a_month_asked(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_same_figure_a_month", py_func("the_same_figure_a_month", |args: &[Py]| the_same_figure_a_month(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_restated", py_func("a_rate_restated", |args: &[Py]| a_rate_restated(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_put_in_with_a_verb", py_func("a_rate_put_in_with_a_verb", |args: &[Py]| a_rate_put_in_with_a_verb(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_length_at_the_rate_in_the_chat", py_func("a_length_at_the_rate_in_the_chat", |args: &[Py]| a_length_at_the_rate_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_with_no_date", py_func("a_rate_with_no_date", |args: &[Py]| a_rate_with_no_date(args)));
    pyrt::xrefs::reg_fn("memory.saving", "asks_the_rate", py_func("asks_the_rate", |args: &[Py]| asks_the_rate(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_in_the_money_readers_words", py_func("_in_the_money_readers_words", |args: &[Py]| _in_the_money_readers_words(args)));
    pyrt::xrefs::reg_fn("memory.saving", "rate_question", py_func("rate_question", |args: &[Py]| rate_question(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_rate_said", py_func("_a_rate_said", |args: &[Py]| _a_rate_said(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_for_the_goal", py_func("a_rate_for_the_goal", |args: &[Py]| a_rate_for_the_goal(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_added_sum", py_func("_the_added_sum", |args: &[Py]| _the_added_sum(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_rate_raised", py_func("a_rate_raised", |args: &[Py]| a_rate_raised(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_said_a_rate_at", py_func("_said_a_rate_at", |args: &[Py]| _said_a_rate_at(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_rate_against_the_goal", py_func("_a_rate_against_the_goal", |args: &[Py]| _a_rate_against_the_goal(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_closer", py_func("_a_closer", |args: &[Py]| _a_closer(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_a_said_sentence", py_func("_a_said_sentence", |args: &[Py]| _a_said_sentence(args)));
    pyrt::xrefs::reg_fn("memory.saving", "said_already", py_func("said_already", |args: &[Py]| said_already(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_weeks_left", py_func("_weeks_left", |args: &[Py]| _weeks_left(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_figure_each_week", py_func("a_figure_each_week", |args: &[Py]| a_figure_each_week(args)));
    pyrt::xrefs::reg_fn("memory.saving", "the_figure_a_week", py_func("the_figure_a_week", |args: &[Py]| the_figure_a_week(args)));
    pyrt::xrefs::reg_fn("memory.saving", "without_what_was_just_said", py_func("without_what_was_just_said", |args: &[Py]| without_what_was_just_said(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_the_rate_put_by", py_func("_the_rate_put_by", |args: &[Py]| _the_rate_put_by(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_length_with_no_price", py_func("a_length_with_no_price", |args: &[Py]| a_length_with_no_price(args)));
    pyrt::xrefs::reg_fn("memory.saving", "_already_asked_for_the_price", py_func("_already_asked_for_the_price", |args: &[Py]| _already_asked_for_the_price(args)));
    pyrt::xrefs::reg_fn("memory.saving", "a_price_in_an_answer", py_func("a_price_in_an_answer", |args: &[Py]| a_price_in_an_answer(args)));
    pyrt::xrefs::reg_access("memory.saving", "ADDED", g_ADDED, s_ADDED);
    pyrt::xrefs::reg_access("memory.saving", "AIM_AT_A_SUM", g_AIM_AT_A_SUM, s_AIM_AT_A_SUM);
    pyrt::xrefs::reg_access("memory.saving", "ALREADY", g_ALREADY, s_ALREADY);
    pyrt::xrefs::reg_access("memory.saving", "ALREADY_AFTER", g_ALREADY_AFTER, s_ALREADY_AFTER);
    pyrt::xrefs::reg_access("memory.saving", "ALREADY_HELD", g_ALREADY_HELD, s_ALREADY_HELD);
    pyrt::xrefs::reg_access("memory.saving", "ANOTHER_SUM", g_ANOTHER_SUM, s_ANOTHER_SUM);
    pyrt::xrefs::reg_access("memory.saving", "ASKS_A_LENGTH", g_ASKS_A_LENGTH, s_ASKS_A_LENGTH);
    pyrt::xrefs::reg_access("memory.saving", "ASKS_A_LENGTH_FOR_A_THING", g_ASKS_A_LENGTH_FOR_A_THING, s_ASKS_A_LENGTH_FOR_A_THING);
    pyrt::xrefs::reg_access("memory.saving", "ASKS_HOW_MUCH_MORE", g_ASKS_HOW_MUCH_MORE, s_ASKS_HOW_MUCH_MORE);
    pyrt::xrefs::reg_access("memory.saving", "ASKS_IF_ENOUGH", g_ASKS_IF_ENOUGH, s_ASKS_IF_ENOUGH);
    pyrt::xrefs::reg_access("memory.saving", "AT_A_FIGURE", g_AT_A_FIGURE, s_AT_A_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "A_CAN_CARRY_IT", g_A_CAN_CARRY_IT, s_A_CAN_CARRY_IT);
    pyrt::xrefs::reg_access("memory.saving", "A_CORRECTION_AND_A_FIGURE", g_A_CORRECTION_AND_A_FIGURE, s_A_CORRECTION_AND_A_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "A_GOAL_STATED", g_A_GOAL_STATED, s_A_GOAL_STATED);
    pyrt::xrefs::reg_access("memory.saving", "A_GOAL_SUM", g_A_GOAL_SUM, s_A_GOAL_SUM);
    pyrt::xrefs::reg_access("memory.saving", "A_LENGTH_SAID", g_A_LENGTH_SAID, s_A_LENGTH_SAID);
    pyrt::xrefs::reg_access("memory.saving", "A_LENGTH_WAS_ASKED", g_A_LENGTH_WAS_ASKED, s_A_LENGTH_WAS_ASKED);
    pyrt::xrefs::reg_access("memory.saving", "A_LONE_SUM", g_A_LONE_SUM, s_A_LONE_SUM);
    pyrt::xrefs::reg_access("memory.saving", "A_MONTH_FIGURE", g_A_MONTH_FIGURE, s_A_MONTH_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "A_MORE_LIKE", g_A_MORE_LIKE, s_A_MORE_LIKE);
    pyrt::xrefs::reg_access("memory.saving", "A_NEW_RATE", g_A_NEW_RATE, s_A_NEW_RATE);
    pyrt::xrefs::reg_access("memory.saving", "A_NO_THEN_A_RATE", g_A_NO_THEN_A_RATE, s_A_NO_THEN_A_RATE);
    pyrt::xrefs::reg_access("memory.saving", "A_RATE", g_A_RATE, s_A_RATE);
    pyrt::xrefs::reg_access("memory.saving", "A_RATE_ANY_OTHER_PERIOD", g_A_RATE_ANY_OTHER_PERIOD, s_A_RATE_ANY_OTHER_PERIOD);
    pyrt::xrefs::reg_access("memory.saving", "A_RATE_PUT_RIGHT", g_A_RATE_PUT_RIGHT, s_A_RATE_PUT_RIGHT);
    pyrt::xrefs::reg_access("memory.saving", "A_RATE_WITH_A_VERB", g_A_RATE_WITH_A_VERB, s_A_RATE_WITH_A_VERB);
    pyrt::xrefs::reg_access("memory.saving", "A_RESTATED_RATE", g_A_RESTATED_RATE, s_A_RESTATED_RATE);
    pyrt::xrefs::reg_access("memory.saving", "A_SPENT_ON", g_A_SPENT_ON, s_A_SPENT_ON);
    pyrt::xrefs::reg_access("memory.saving", "A_SUM_A_MONTH", g_A_SUM_A_MONTH, s_A_SUM_A_MONTH);
    pyrt::xrefs::reg_access("memory.saving", "A_THING_NAMED", g_A_THING_NAMED, s_A_THING_NAMED);
    pyrt::xrefs::reg_access("memory.saving", "A_TRAILING_SUM", g_A_TRAILING_SUM, s_A_TRAILING_SUM);
    pyrt::xrefs::reg_access("memory.saving", "A_WEEK_ASK", g_A_WEEK_ASK, s_A_WEEK_ASK);
    pyrt::xrefs::reg_access("memory.saving", "A_WEEK_FIGURE", g_A_WEEK_FIGURE, s_A_WEEK_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "BARE_DATE", g_BARE_DATE, s_BARE_DATE);
    pyrt::xrefs::reg_access("memory.saving", "BARE_END_OF_RELATIVE", g_BARE_END_OF_RELATIVE, s_BARE_END_OF_RELATIVE);
    pyrt::xrefs::reg_access("memory.saving", "BARE_FIGURE", g_BARE_FIGURE, s_BARE_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "BARE_RATE", g_BARE_RATE, s_BARE_RATE);
    pyrt::xrefs::reg_access("memory.saving", "BY_A_DATE", g_BY_A_DATE, s_BY_A_DATE);
    pyrt::xrefs::reg_access("memory.saving", "CLAUSE_EDGE", g_CLAUSE_EDGE, s_CLAUSE_EDGE);
    pyrt::xrefs::reg_access("memory.saving", "CONDITIONAL", g_CONDITIONAL, s_CONDITIONAL);
    pyrt::xrefs::reg_access("memory.saving", "CORRECTING", g_CORRECTING, s_CORRECTING);
    pyrt::xrefs::reg_access("memory.saving", "DATE", g_DATE, s_DATE);
    pyrt::xrefs::reg_access("memory.saving", "END_OF", g_END_OF, s_END_OF);
    pyrt::xrefs::reg_access("memory.saving", "END_OF_RELATIVE", g_END_OF_RELATIVE, s_END_OF_RELATIVE);
    pyrt::xrefs::reg_access("memory.saving", "FILLER_LEAD", g_FILLER_LEAD, s_FILLER_LEAD);
    pyrt::xrefs::reg_access("memory.saving", "FURTHER_SAID", g_FURTHER_SAID, s_FURTHER_SAID);
    pyrt::xrefs::reg_access("memory.saving", "HELD", g_HELD, s_HELD);
    pyrt::xrefs::reg_access("memory.saving", "HELD_SAID", g_HELD_SAID, s_HELD_SAID);
    pyrt::xrefs::reg_access("memory.saving", "HOW_FAR_OFF", g_HOW_FAR_OFF, s_HOW_FAR_OFF);
    pyrt::xrefs::reg_access("memory.saving", "HOW_LONG_ASK", g_HOW_LONG_ASK, s_HOW_LONG_ASK);
    pyrt::xrefs::reg_access("memory.saving", "HOW_LONG_UNTIL", g_HOW_LONG_UNTIL, s_HOW_LONG_UNTIL);
    pyrt::xrefs::reg_access("memory.saving", "HOW_MUCH", g_HOW_MUCH, s_HOW_MUCH);
    pyrt::xrefs::reg_access("memory.saving", "INCOME_FRAME", g_INCOME_FRAME, s_INCOME_FRAME);
    pyrt::xrefs::reg_access("memory.saving", "INTENT", g_INTENT, s_INTENT);
    pyrt::xrefs::reg_access("memory.saving", "MONEY_OUT", g_MONEY_OUT, s_MONEY_OUT);
    pyrt::xrefs::reg_access("memory.saving", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.saving", "MONTHS_IN_A", g_MONTHS_IN_A, s_MONTHS_IN_A);
    pyrt::xrefs::reg_access("memory.saving", "NAMED_WHEN", g_NAMED_WHEN, s_NAMED_WHEN);
    pyrt::xrefs::reg_access("memory.saving", "NEED_A_FIGURE", g_NEED_A_FIGURE, s_NEED_A_FIGURE);
    pyrt::xrefs::reg_access("memory.saving", "NOT_A_POT", g_NOT_A_POT, s_NOT_A_POT);
    pyrt::xrefs::reg_access("memory.saving", "NOT_A_PRICE", g_NOT_A_PRICE, s_NOT_A_PRICE);
    pyrt::xrefs::reg_access("memory.saving", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.saving", "OTHER_POT", g_OTHER_POT, s_OTHER_POT);
    pyrt::xrefs::reg_access("memory.saving", "OUTGOING_FRAME", g_OUTGOING_FRAME, s_OUTGOING_FRAME);
    pyrt::xrefs::reg_access("memory.saving", "PAST_SAVING", g_PAST_SAVING, s_PAST_SAVING);
    pyrt::xrefs::reg_access("memory.saving", "POT_OPENER", g_POT_OPENER, s_POT_OPENER);
    pyrt::xrefs::reg_access("memory.saving", "POT_REPORTED", g_POT_REPORTED, s_POT_REPORTED);
    pyrt::xrefs::reg_access("memory.saving", "PRICE_WORDS", g_PRICE_WORDS, s_PRICE_WORDS);
    pyrt::xrefs::reg_access("memory.saving", "PUT_BY", g_PUT_BY, s_PUT_BY);
    pyrt::xrefs::reg_access("memory.saving", "RATE_GIVEN", g_RATE_GIVEN, s_RATE_GIVEN);
    pyrt::xrefs::reg_access("memory.saving", "RATE_VERB", g_RATE_VERB, s_RATE_VERB);
    pyrt::xrefs::reg_access("memory.saving", "SAVE_VERB", g_SAVE_VERB, s_SAVE_VERB);
    pyrt::xrefs::reg_access("memory.saving", "SAVING", g_SAVING, s_SAVING);
    pyrt::xrefs::reg_access("memory.saving", "SAVING_CLOSERS", g_SAVING_CLOSERS, s_SAVING_CLOSERS);
    pyrt::xrefs::reg_access("memory.saving", "SUM", g_SUM, s_SUM);
    pyrt::xrefs::reg_access("memory.saving", "THE_THING_WANTED", g_THE_THING_WANTED, s_THE_THING_WANTED);
    pyrt::xrefs::reg_access("memory.saving", "TOGETHER", g_TOGETHER, s_TOGETHER);
    pyrt::xrefs::reg_access("memory.saving", "WHEN_AT_THE_GOAL", g_WHEN_AT_THE_GOAL, s_WHEN_AT_THE_GOAL);
    pyrt::xrefs::reg_access("memory.saving", "WHEN_WORDS", g_WHEN_WORDS, s_WHEN_WORDS);
    pyrt::xrefs::reg_access("memory.saving", "_KNOWN_TO_THE_MONEY_ENTRY", g__KNOWN_TO_THE_MONEY_ENTRY, s__KNOWN_TO_THE_MONEY_ENTRY);
    pyrt::xrefs::reg_access("memory.saving", "_NOT_THE_RATE", g__NOT_THE_RATE, s__NOT_THE_RATE);
    pyrt::xrefs::reg_access("memory.saving", "_OTHER_FIGURE_WORDS", g__OTHER_FIGURE_WORDS, s__OTHER_FIGURE_WORDS);
    pyrt::xrefs::reg_access("memory.saving", "_THE_END_OF_A_SEASON", g__THE_END_OF_A_SEASON, s__THE_END_OF_A_SEASON);
}

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


thread_local! { static G_ABROAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABROAD_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHAT_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_DAY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHECK_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEAL_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PART_OF_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SKY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_IN_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRIP_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECKS_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_OF_SOMETHING_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNCIL_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNCIL_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_OUT_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORM_FROM_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORM_TO_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GET_A_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GET_FROM_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_COUNTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_NATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOURNEY_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_TIME_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_HOME_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVING_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AVAILABILITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_CHECK_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DAY_OUT_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DESTINATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOURNEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SHORTLIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_HOME_COUNTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_PAPERWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARKING_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_OF_DAY_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTE_ENDS_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEND_A_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICES_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOP_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORTLIST_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THINGS_TO_DO_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_DAY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_SHORTLIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_THING_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_THE_RULES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEATHER_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COMPARISON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NAMED_COUNTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NAME_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_QUALIFIER_AFTER_THE_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_LABEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_MODE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ENDS_REVERSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABROAD() -> Py { ensure_init(); G_ABROAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABROAD(v: Py) { ensure_init(); G_ABROAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ABROAD_POOL() -> Py { ensure_init(); G_ABROAD_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABROAD_POOL(v: Py) { ensure_init(); G_ABROAD_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ON_ASK() -> Py { ensure_init(); G_AN_ON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ON_ASK(v: Py) { ensure_init(); G_AN_ON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHAT_TO_DO() -> Py { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_TO_DO(v: Py) { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_DAY_OUT() -> Py { ensure_init(); G_A_BARE_DAY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_DAY_OUT(v: Py) { ensure_init(); G_A_BARE_DAY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHECK_SENTENCE() -> Py { ensure_init(); G_A_CHECK_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHECK_SENTENCE(v: Py) { ensure_init(); G_A_CHECK_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDITION() -> Py { ensure_init(); G_A_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION(v: Py) { ensure_init(); G_A_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEW_WORDS() -> Py { ensure_init(); G_A_FEW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEW_WORDS(v: Py) { ensure_init(); G_A_FEW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEAL_TIME() -> Py { ensure_init(); G_A_MEAL_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEAL_TIME(v: Py) { ensure_init(); G_A_MEAL_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOT_A_PLACE() -> Py { ensure_init(); G_A_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOT_A_PLACE(v: Py) { ensure_init(); G_A_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PART_OF_DAY() -> Py { ensure_init(); G_A_PART_OF_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PART_OF_DAY(v: Py) { ensure_init(); G_A_PART_OF_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_THERE() -> Py { ensure_init(); G_A_PLACE_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_THERE(v: Py) { ensure_init(); G_A_PLACE_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_OF_ITS_OWN() -> Py { ensure_init(); G_A_QUESTION_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_OF_ITS_OWN(v: Py) { ensure_init(); G_A_QUESTION_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SETTING() -> Py { ensure_init(); G_A_SETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SETTING(v: Py) { ensure_init(); G_A_SETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SKY_ASK() -> Py { ensure_init(); G_A_SKY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SKY_ASK(v: Py) { ensure_init(); G_A_SKY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_IN_HAND() -> Py { ensure_init(); G_A_THING_IN_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_IN_HAND(v: Py) { ensure_init(); G_A_THING_IN_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NOT_A_PLACE() -> Py { ensure_init(); G_A_THING_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NOT_A_PLACE(v: Py) { ensure_init(); G_A_THING_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_AFTER() -> Py { ensure_init(); G_A_TIME_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_AFTER(v: Py) { ensure_init(); G_A_TIME_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_NOT_A_PLACE() -> Py { ensure_init(); G_A_TIME_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_NOT_A_PLACE(v: Py) { ensure_init(); G_A_TIME_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRIP_BEFORE() -> Py { ensure_init(); G_A_TRIP_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRIP_BEFORE(v: Py) { ensure_init(); G_A_TRIP_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECKS_POOL() -> Py { ensure_init(); G_CHECKS_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECKS_POOL(v: Py) { ensure_init(); G_CHECKS_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_OF_SOMETHING_ELSE() -> Py { ensure_init(); G_CHECK_OF_SOMETHING_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_OF_SOMETHING_ELSE(v: Py) { ensure_init(); G_CHECK_OF_SOMETHING_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_POOL() -> Py { ensure_init(); G_CHOICE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_POOL(v: Py) { ensure_init(); G_CHOICE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNCIL_POOL() -> Py { ensure_init(); G_COUNCIL_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNCIL_POOL(v: Py) { ensure_init(); G_COUNCIL_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNCIL_WORD() -> Py { ensure_init(); G_COUNCIL_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNCIL_WORD(v: Py) { ensure_init(); G_COUNCIL_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTRIES() -> Py { ensure_init(); G_COUNTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRIES(v: Py) { ensure_init(); G_COUNTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_OUT_NOT() -> Py { ensure_init(); G_DAY_OUT_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_OUT_NOT(v: Py) { ensure_init(); G_DAY_OUT_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_FORM_FROM_POOL() -> Py { ensure_init(); G_FORM_FROM_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORM_FROM_POOL(v: Py) { ensure_init(); G_FORM_FROM_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_FORM_TO_POOL() -> Py { ensure_init(); G_FORM_TO_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORM_TO_POOL(v: Py) { ensure_init(); G_FORM_TO_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_GET_A_FORM() -> Py { ensure_init(); G_GET_A_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GET_A_FORM(v: Py) { ensure_init(); G_GET_A_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_GET_FROM_TO() -> Py { ensure_init(); G_GET_FROM_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GET_FROM_TO(v: Py) { ensure_init(); G_GET_FROM_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_AWAY() -> Py { ensure_init(); G_GOING_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_AWAY(v: Py) { ensure_init(); G_GOING_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_A_VERB() -> Py { ensure_init(); G_HAS_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_A_VERB(v: Py) { ensure_init(); G_HAS_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_COUNTRIES() -> Py { ensure_init(); G_HOME_COUNTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_COUNTRIES(v: Py) { ensure_init(); G_HOME_COUNTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_NATIONS() -> Py { ensure_init(); G_HOME_NATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_NATIONS(v: Py) { ensure_init(); G_HOME_NATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOURNEY_POOL() -> Py { ensure_init(); G_JOURNEY_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOURNEY_POOL(v: Py) { ensure_init(); G_JOURNEY_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_TIME_POOL() -> Py { ensure_init(); G_MEAL_TIME_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_TIME_POOL(v: Py) { ensure_init(); G_MEAL_TIME_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_HOME_NAMES() -> Py { ensure_init(); G_MOVE_HOME_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_HOME_NAMES(v: Py) { ensure_init(); G_MOVE_HOME_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVING() -> Py { ensure_init(); G_MOVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVING(v: Py) { ensure_init(); G_MOVING.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVING_HOUSE() -> Py { ensure_init(); G_MOVING_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVING_HOUSE(v: Py) { ensure_init(); G_MOVING_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AVAILABILITY() -> Py { ensure_init(); G_NOT_AVAILABILITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AVAILABILITY(v: Py) { ensure_init(); G_NOT_AVAILABILITY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_CHECK_SENTENCE() -> Py { ensure_init(); G_NOT_A_CHECK_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CHECK_SENTENCE(v: Py) { ensure_init(); G_NOT_A_CHECK_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_CHOICE() -> Py { ensure_init(); G_NOT_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CHOICE(v: Py) { ensure_init(); G_NOT_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DAY_OUT_PLACE() -> Py { ensure_init(); G_NOT_A_DAY_OUT_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DAY_OUT_PLACE(v: Py) { ensure_init(); G_NOT_A_DAY_OUT_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DESTINATION() -> Py { ensure_init(); G_NOT_A_DESTINATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DESTINATION(v: Py) { ensure_init(); G_NOT_A_DESTINATION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FORM() -> Py { ensure_init(); G_NOT_A_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FORM(v: Py) { ensure_init(); G_NOT_A_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOURNEY() -> Py { ensure_init(); G_NOT_A_JOURNEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOURNEY(v: Py) { ensure_init(); G_NOT_A_JOURNEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SHORTLIST() -> Py { ensure_init(); G_NOT_A_SHORTLIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SHORTLIST(v: Py) { ensure_init(); G_NOT_A_SHORTLIST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_HOME_COUNTRIES() -> Py { ensure_init(); G_NOT_HOME_COUNTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_HOME_COUNTRIES(v: Py) { ensure_init(); G_NOT_HOME_COUNTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_PAPERWORK() -> Py { ensure_init(); G_NOT_THE_PAPERWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_PAPERWORK(v: Py) { ensure_init(); G_NOT_THE_PAPERWORK.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_SUBJECT() -> Py { ensure_init(); G_ONLY_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_SUBJECT(v: Py) { ensure_init(); G_ONLY_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_PARKING_RULE() -> Py { ensure_init(); G_PARKING_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARKING_RULE(v: Py) { ensure_init(); G_PARKING_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_OF_DAY_POOL() -> Py { ensure_init(); G_PART_OF_DAY_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_OF_DAY_POOL(v: Py) { ensure_init(); G_PART_OF_DAY_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PICKS() -> Py { ensure_init(); G_PICKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICKS(v: Py) { ensure_init(); G_PICKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_NOT() -> Py { ensure_init(); G_PICK_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_NOT(v: Py) { ensure_init(); G_PICK_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_POOL() -> Py { ensure_init(); G_PICK_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_POOL(v: Py) { ensure_init(); G_PICK_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_WORDS() -> Py { ensure_init(); G_POINTING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_WORDS(v: Py) { ensure_init(); G_POINTING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTE_ENDS_POOL() -> Py { ensure_init(); G_ROUTE_ENDS_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTE_ENDS_POOL(v: Py) { ensure_init(); G_ROUTE_ENDS_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTE_POOL() -> Py { ensure_init(); G_ROUTE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTE_POOL(v: Py) { ensure_init(); G_ROUTE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SEND_A_FORM() -> Py { ensure_init(); G_SEND_A_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEND_A_FORM(v: Py) { ensure_init(); G_SEND_A_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICES() -> Py { ensure_init(); G_SERVICES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICES(v: Py) { ensure_init(); G_SERVICES.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICES_POOL() -> Py { ensure_init(); G_SERVICES_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICES_POOL(v: Py) { ensure_init(); G_SERVICES_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOPS() -> Py { ensure_init(); G_SHOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOPS(v: Py) { ensure_init(); G_SHOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOP_ASK() -> Py { ensure_init(); G_SHOP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOP_ASK(v: Py) { ensure_init(); G_SHOP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOP_POOL() -> Py { ensure_init(); G_SHOP_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOP_POOL(v: Py) { ensure_init(); G_SHOP_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORTLIST_POOL() -> Py { ensure_init(); G_SHORTLIST_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORTLIST_POOL(v: Py) { ensure_init(); G_SHORTLIST_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_KINDS() -> Py { ensure_init(); G_SUBJECT_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_KINDS(v: Py) { ensure_init(); G_SUBJECT_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THINGS_TO_DO_POOL() -> Py { ensure_init(); G_THINGS_TO_DO_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THINGS_TO_DO_POOL(v: Py) { ensure_init(); G_THINGS_TO_DO_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL() -> Py { ensure_init(); G_TRAVEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL(v: Py) { ensure_init(); G_TRAVEL.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_CHECK() -> Py { ensure_init(); G_WANTS_A_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_CHECK(v: Py) { ensure_init(); G_WANTS_A_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_CHOICE() -> Py { ensure_init(); G_WANTS_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_CHOICE(v: Py) { ensure_init(); G_WANTS_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_DAY_OUT() -> Py { ensure_init(); G_WANTS_A_DAY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_DAY_OUT(v: Py) { ensure_init(); G_WANTS_A_DAY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_PICK() -> Py { ensure_init(); G_WANTS_A_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_PICK(v: Py) { ensure_init(); G_WANTS_A_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_PLAN() -> Py { ensure_init(); G_WANTS_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_PLAN(v: Py) { ensure_init(); G_WANTS_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_SHORTLIST() -> Py { ensure_init(); G_WANTS_A_SHORTLIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_SHORTLIST(v: Py) { ensure_init(); G_WANTS_A_SHORTLIST.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_THING_DONE() -> Py { ensure_init(); G_WANTS_A_THING_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_THING_DONE(v: Py) { ensure_init(); G_WANTS_A_THING_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_TIME() -> Py { ensure_init(); G_WANTS_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_TIME(v: Py) { ensure_init(); G_WANTS_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_THE_RULES() -> Py { ensure_init(); G_WANTS_THE_RULES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_THE_RULES(v: Py) { ensure_init(); G_WANTS_THE_RULES.with(|c| *c.borrow_mut() = v); }
pub fn g_WEATHER_POOL() -> Py { ensure_init(); G_WEATHER_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEATHER_POOL(v: Py) { ensure_init(); G_WEATHER_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COMPARISON() -> Py { ensure_init(); G__A_COMPARISON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COMPARISON(v: Py) { ensure_init(); G__A_COMPARISON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NAMED_COUNTRY() -> Py { ensure_init(); G__A_NAMED_COUNTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NAMED_COUNTRY(v: Py) { ensure_init(); G__A_NAMED_COUNTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NAME_PART() -> Py { ensure_init(); G__A_NAME_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NAME_PART(v: Py) { ensure_init(); G__A_NAME_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__A_QUALIFIER_AFTER_THE_PLACE() -> Py { ensure_init(); G__A_QUALIFIER_AFTER_THE_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_QUALIFIER_AFTER_THE_PLACE(v: Py) { ensure_init(); G__A_QUALIFIER_AFTER_THE_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_PLACE() -> Py { ensure_init(); G__NOT_THE_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_PLACE(v: Py) { ensure_init(); G__NOT_THE_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_LABEL() -> Py { ensure_init(); G__SUBJECT_LABEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_LABEL(v: Py) { ensure_init(); G__SUBJECT_LABEL.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_MODE() -> Py { ensure_init(); G__SUBJECT_MODE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_MODE(v: Py) { ensure_init(); G__SUBJECT_MODE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ENDS() -> Py { ensure_init(); G__THE_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ENDS(v: Py) { ensure_init(); G__THE_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ENDS_REVERSED() -> Py { ensure_init(); G__THE_ENDS_REVERSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ENDS_REVERSED(v: Py) { ensure_init(); G__THE_ENDS_REVERSED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The things the bot cannot see, and the one place that does see them.\n\nThe floor of this bot is one reply for everything nobody knows, and on main that reply is the\nhonest shrug: \"I'm not sure about that one, sorry - a quick search online will get you a solid\nanswer\". It is honest, and it is useless to somebody who asked a question with a *place* behind\nit. STYLE.md draws the line this module sits on:\n\n    What time does the post office on Hill Road close today?\n    I can't check live opening hours, sorry. Search for the branch online: its listing on a map\n    app usually shows today's hours, holidays included.\n\nThree things make that reply good, and none of them is in the shrug: it says what the bot cannot\ndo (check live hours), it names the one place that can (the branch's listing on a map app), and it\ndoes not guess. The pool cannot do that for every line - it does not know what the line is about -\nso this module reads the line for the *kind* of thing being asked about and gives the limit, the\nplace to look and, where it helps, one short question.\n\n    is it on iplayer                      \"I can't see what's on iPlayer from here, sorry...\"\n    what time is the last train to leeds  \"...no live train times from me - a journey planner...\"\n    whats the best beach in cornwall      \"...I don't know the beaches round there well enough...\"\n    how about moving to spain             \"...it depends on your passport - gov.uk's living\n                                           abroad pages and the country's own official site...\"\n    will it frost tonight                 \"...no live weather from me - a forecast app has\n                                           today's, hour by hour...\"\n\nSix kinds are here and no more, because each one has either a *named* place that knows or a list\nthe bot has already given, and that is what makes the reply worth more than a search: a service's\nown catalogue, a journey planner, the place's own listing, the government that writes the rules,\nthe sky over the person's own postcode, and - the sixth - the checks the bot itself named a turn or\ntwo ago, handed back when the person asks what to check before they commit.\nAnything else stands down (returns None) and the floor answers as it did before.\n\nTwo more were added after the operator's note that a pointer on its own is not an answer: a choice\nbetween two named things, and an ask for a shortlist. Neither names a subject - that is exactly\nwhat the line does not have - so each one says the part that is true of every question of its\nshape, what to look for and the usual first step, and then asks the one thing only the person\nknows. That is STYLE.md's \"what to look for\" half of \"Not knowing, honestly\", and it comes before\nany mention of where the rest lives.\n\nThis is a last resort and it is written to be one: `bot/modules/greeting.py` asks it alongside the\ndetail of an answer and the composer, and the composer is asked first, so a line any entry, any\nmodule or the composer can answer never reaches this file. What it can lose is a line the shrug\nwould have got; what it cannot do is take a turn away from a better answer, or invent a fact to\nfill the gap. Nothing here is a fact about the world: every reply says only what the bot cannot\ncheck, what decides a thing of that kind, and where a person can find the rest.\n");
    s_COUNTRIES(gen_memory::memory::facts::g_COUNTRIES());
    s_HOME_NATIONS(call_builtin("frozenset", &[mcall(&py_str("england scotland wales britain uk ireland northern ireland"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HOME_COUNTRIES(call_builtin("frozenset", &[py_tuple(vec![py_str("united kingdom"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("northern ireland"), py_str("great britain"), py_str("britain"), py_str("uk"), py_str("ireland")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_HOME_COUNTRIES(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gen_memory::memory::facts::g_COUNTRIES()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_HOME_COUNTRIES(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_HOME_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("united kingdom"), py_str("england"), py_str("scotland"), py_str("wales"), py_str("northern ireland"), py_str("great britain"), py_str("britain"), py_str("uk")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NAMED_COUNTRY(Py::None);
    s_SERVICES(py_tuple(vec![py_tuple(vec![py_str("iplayer"), py_str("iPlayer")]), py_tuple(vec![py_str("bbc iplayer"), py_str("iPlayer")]), py_tuple(vec![py_str("itv hub"), py_str("ITV Hub")]), py_tuple(vec![py_str("itvx"), py_str("ITVX")]), py_tuple(vec![py_str("itv"), py_str("ITV")]), py_tuple(vec![py_str("channel 4"), py_str("Channel 4")]), py_tuple(vec![py_str("all 4"), py_str("All 4")]), py_tuple(vec![py_str("4od"), py_str("All 4")]), py_tuple(vec![py_str("my5"), py_str("My5")]), py_tuple(vec![py_str("netflix"), py_str("Netflix")]), py_tuple(vec![py_str("prime video"), py_str("Prime Video")]), py_tuple(vec![py_str("amazon prime"), py_str("Prime Video")]), py_tuple(vec![py_str("disney plus"), py_str("Disney+")]), py_tuple(vec![py_str("disney+"), py_str("Disney+")]), py_tuple(vec![py_str("now tv"), py_str("Now TV")]), py_tuple(vec![py_str("nowtv"), py_str("Now TV")]), py_tuple(vec![py_str("apple tv"), py_str("Apple TV")]), py_tuple(vec![py_str("paramount"), py_str("Paramount+")]), py_tuple(vec![py_str("britbox"), py_str("BritBox")]), py_tuple(vec![py_str("hayu"), py_str("Hayu")]), py_tuple(vec![py_str("sky go"), py_str("Sky Go")]), py_tuple(vec![py_str("sky q"), py_str("Sky Q")]), py_tuple(vec![py_str("sky tv"), py_str("Sky")]), py_tuple(vec![py_str("sky cinema"), py_str("Sky Cinema")]), py_tuple(vec![py_str("sky sports"), py_str("Sky Sports")]), py_tuple(vec![py_str("spotify"), py_str("Spotify")]), py_tuple(vec![py_str("audible"), py_str("Audible")]), py_tuple(vec![py_str("youtube"), py_str("YouTube")])]));
    s_NOT_AVAILABILITY(stdlib_call("re", "compile", &[py_str("\\b(?:stopping|stops|stopped|buffer\\w*|glitch\\w*|not working|keeps? (?:cutting|freezing|loading)|wont play|won't play|password|log ?in|sign ?up|account|cancel\\w*|subscri\\w*|price|prices|cost|costs|cheaper|worth it|share|sharing|binge|download\\w*|aeroplane|plane|offline|recommend|any good|what to watch|something to watch|worth watch\\w*|turn (?:off|on)|settings|autoplay|auto play|notification\\w*|episodes? (?:left|to go)|(?:anything|something|whats|what's)\\s+(?:\\w+\\s+){0,2}(?:funny|good|scary|sad|new|decent|chilled|violent|easy)\\b|what (?:can|could|should) (?:i|we) (?:watch|see|stream)|(?:anything|something) (?:good )?(?:on|to watch))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAVEL(py_dict(vec![(py_str("train"), py_tuple(vec![py_str("train"), py_str("trains")])), (py_str("trains"), py_tuple(vec![py_str("train"), py_str("trains")])), (py_str("bus"), py_tuple(vec![py_str("bus"), py_str("buses")])), (py_str("buses"), py_tuple(vec![py_str("bus"), py_str("buses")])), (py_str("coach"), py_tuple(vec![py_str("coach"), py_str("coaches")])), (py_str("tube"), py_tuple(vec![py_str("tube"), py_str("")])), (py_str("underground"), py_tuple(vec![py_str("tube"), py_str("")])), (py_str("metro"), py_tuple(vec![py_str("metro"), py_str("")])), (py_str("tram"), py_tuple(vec![py_str("tram"), py_str("trams")])), (py_str("ferry"), py_tuple(vec![py_str("ferry"), py_str("ferries")])), (py_str("ferries"), py_tuple(vec![py_str("ferry"), py_str("ferries")]))]));
    s__A_COMPARISON(py_str("(?:best|quickest|fastest|easiest|cheapest|simplest|shortest|safest|most direct|nicest|prettiest)"));
    s_GET_FROM_TO(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bhow (?:do|can|would|could) (?:i|we|you) (?:get|travel)\\s+(?:to|from)\\b[^?.]{0,30}\\bfrom\\b|\\b(?:"), &g__A_COMPARISON()), &py_str("\\s+)?(?:route|journey|drive|commute)\\s+to\\b[^?.]{0,30}\\bfrom\\b|\\b")), &g__A_COMPARISON()), &py_str("\\s+(?:way|route)\\s+(?:to|of)\\s+(?:get|getting|travel|travelling)\\s+(?:to|from)\\b[^?.]{0,30}\\bfrom\\b|\\b")), &g__A_COMPARISON()), &py_str("\\s+(?:way|route)\\s+from\\b[^?.]{0,30}\\bto\\b|\\b")), &g__A_COMPARISON()), &py_str("\\s+(?:way|route)\\s+(?:to|of)\\s+(?:get|getting|travel|travelling)\\s+from\\b[^?.]{0,30}\\bto\\b|\\bhow long (?:does it|will it|would it) take to (?:get|drive|travel|cycle|walk)\\s+to\\b[^?.]{0,30}\\bfrom\\b|\\bhow (?:do|can|would|could) (?:i|we|you) (?:get|travel)\\s+from\\b[^?.]{0,30}\\bto\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:what time|whatt?ime|how long|last|first|next|earliest|latest|timetable|doable|won'?t make|is there a|are there any|how often)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DESTINATION(stdlib_call("re", "compile", &[py_str("\\bget(?:ting)?\\s+(?:to\\s+)?(?:sleep|rid|grips|know|understand|talk|the\\s+(?:point|bottom)|terms|an?\\s+agreement|a\\s+decision|the\\s+root)\\b|\\bget(?:ting)?\\s+(?:over|through|around|on\\s+top\\s+of|out\\s+of|away\\s+with|the\\s+hang)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_JOURNEY(stdlib_call("re", "compile", &[py_str("\\b(?:delayed|delay|cancelled|cancel|refund|ticket price|how much is .{0,20}ticket|season ticket|railcard|strike|replacement bus|sleeper|i'?m on (?:the|a) (?:train|bus)|on the (?:train|bus)|lost property|left (?:my|her|his) \\w+ on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICKS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("beach"), py_str("beaches")]), py_str("beaches")]), py_tuple(vec![py_tuple(vec![py_str("pub"), py_str("pubs")]), py_str("pubs")]), py_tuple(vec![py_tuple(vec![py_str("restaurant"), py_str("restaurants"), py_str("place to eat"), py_str("places to eat"), py_str("eatery")]), py_str("restaurants")]), py_tuple(vec![py_tuple(vec![py_str("cafe"), py_str("cafes"), py_str("coffee shop"), py_str("coffee shops")]), py_str("cafes")]), py_tuple(vec![py_tuple(vec![py_str("walk"), py_str("walks"), py_str("hike"), py_str("hikes")]), py_str("walks")]), py_tuple(vec![py_tuple(vec![py_str("park"), py_str("parks")]), py_str("parks")]), py_tuple(vec![py_tuple(vec![py_str("market"), py_str("markets")]), py_str("markets")]), py_tuple(vec![py_tuple(vec![py_str("museum"), py_str("museums"), py_str("gallery"), py_str("galleries")]), py_str("museums and galleries")]), py_tuple(vec![py_tuple(vec![py_str("chippy"), py_str("chippies")]), py_str("chip shops")]), py_tuple(vec![py_tuple(vec![py_str("thing to do"), py_str("things to do"), py_str("day out"), py_str("day trip")]), py_str("things to do")])]));
    s_SHOPS(py_tuple(vec![py_tuple(vec![py_str("aldi"), py_str("Aldi")]), py_tuple(vec![py_str("lidl"), py_str("Lidl")]), py_tuple(vec![py_str("tesco"), py_str("Tesco")]), py_tuple(vec![py_str("asda"), py_str("Asda")]), py_tuple(vec![py_str("sainsbury's"), py_str("Sainsbury's")]), py_tuple(vec![py_str("sainsburys"), py_str("Sainsbury's")]), py_tuple(vec![py_str("morrisons"), py_str("Morrisons")]), py_tuple(vec![py_str("waitrose"), py_str("Waitrose")]), py_tuple(vec![py_str("iceland"), py_str("Iceland")]), py_tuple(vec![py_str("co-op"), py_str("the Co-op")]), py_tuple(vec![py_str("coop"), py_str("the Co-op")]), py_tuple(vec![py_str("boots"), py_str("Boots")]), py_tuple(vec![py_str("superdrug"), py_str("Superdrug")]), py_tuple(vec![py_str("argos"), py_str("Argos")]), py_tuple(vec![py_str("amazon"), py_str("Amazon")]), py_tuple(vec![py_str("ebay"), py_str("eBay")]), py_tuple(vec![py_str("ikea"), py_str("IKEA")]), py_tuple(vec![py_str("currys"), py_str("Currys")]), py_tuple(vec![py_str("screwfix"), py_str("Screwfix")]), py_tuple(vec![py_str("b&q"), py_str("B&Q")]), py_tuple(vec![py_str("b and q"), py_str("B&Q")]), py_tuple(vec![py_str("homebase"), py_str("Homebase")]), py_tuple(vec![py_str("wickes"), py_str("Wickes")]), py_tuple(vec![py_str("halfords"), py_str("Halfords")]), py_tuple(vec![py_str("decathlon"), py_str("Decathlon")]), py_tuple(vec![py_str("sports direct"), py_str("Sports Direct")]), py_tuple(vec![py_str("john lewis"), py_str("John Lewis")]), py_tuple(vec![py_str("marks and spencer"), py_str("Marks and Spencer")]), py_tuple(vec![py_str("m&s"), py_str("M&S")]), py_tuple(vec![py_str("poundland"), py_str("Poundland")]), py_tuple(vec![py_str("home bargains"), py_str("Home Bargains")]), py_tuple(vec![py_str("the range"), py_str("The Range")]), py_tuple(vec![py_str("dunelm"), py_str("Dunelm")]), py_tuple(vec![py_str("asos"), py_str("ASOS")]), py_tuple(vec![py_str("wickes"), py_str("Wickes")])]));
    s_SHOP_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how much|do they|do you|does it|can i|when|delivery|deliver|in stock|stock|return|returns|refund|click and collect|collect|charge|postage|warranty|guarantee|trade ?in|price|prices|cost)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_NOT(py_dict(vec![(py_str("beach"), stdlib_call("re", "compile", &[py_str("\\b(?:read|reads|book|books|novel|towel|body|volleyball|ball|hut|huts|wedding|holiday\\s+read)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("park"), stdlib_call("re", "compile", &[py_str("\\b(?:the car|my car|your car|the van|the bike|it there|carefully|properly|legally)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("walk"), stdlib_call("re", "compile", &[py_str("\\b(?:the dog|my dog|to work|to school|home|back|the kids to|take|taking|wear|wearing|bring|bringing|pack|packing|carry|boots|shoes|rain|wet|long|short|slow|fast|bit of a)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("market"), stdlib_call("re", "compile", &[py_str("\\b(?:the car|to buy|flog|sell)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("thing to do"), stdlib_call("re", "compile", &[py_str("\\b(?:with it|with them|with the|today|tonight|now|this evening|picnic|take|bring|pack)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("restaurant"), stdlib_call("re", "compile", &[py_str("\\b(?:wear|wearing|wore|dress|order|book a table|booking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("chippy"), stdlib_call("re", "compile", &[py_str("\\b(?:order|nice from|to have|whats nice|what is nice)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
    s_A_TIME_NOT_A_PLACE(py_str("(?:dinner|dinnertime|dinner\\s+time|lunch|lunchtime|lunch\\s+time|breakfast|breakfast\\s+time|tea|teatime|tea\\s+time|supper|brunch|snack|elevenses|bed|bedtime|bed\\s+time|nap|naptime|nap\\s+time|playtime|play\\s+time|bathtime|bath\\s+time|homework|pickup|pick\\s+up|school\\s+run|morning|mornings|afternoon|afternoons|evening|evenings|night|nights|tonight|midday|noon|time)"));
    s_A_NOT_A_PLACE(op_add(&op_add(&py_str("(?:(?:the|a|an)\\s+)?(?:eat|go|walk|do|take|get|see|find|pick|order|book|be|have|"), &g_A_TIME_NOT_A_PLACE()), &py_str(")\\b")));
    s_A_PLACE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:in|near|around|round|at|by|beside|from|to|outside|inside)\\s+(?!"), &g_A_NOT_A_PLACE()), &py_str(")[a-z]")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_PICK(stdlib_call("re", "compile", &[py_str("\\b(?:best|good|nice|decent|any good|recommend|worth|where should|what should|suggest|anywhere|favourite|favorite|top)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEAL_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:at|for|before|after|around|over)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:dinner|dinner\\s*time|lunch|lunch\\s*time|breakfast|breakfast\\s*time|tea|tea\\s*time|supper|supper\\s*time|brunch|tea ?time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PART_OF_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:at|in|during|over|before|after|this|tomorrow|next)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:morning|afternoon|evening|night|midday|noon|bedtime)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|any|how)\\b[^?]{0,32}\\b(?:should|shall|can|could|do)\\s+(?:i|we)\\s+do\\b|\\bwhat\\s+to\\s+do\\b|\\bhow\\s+(?:do|should)\\s+(?:i|we)\\s+(?:fill|pass|plan|spend)\\b|\\banything\\s+(?:to\\s+do|planned|on|happening)\\b|\\bwhat(?:'s|s|\\s+s|\\s+is|\\s+else\\s+is)?\\s+(?:on|happening|going\\s+on|good\\s+to\\s+do)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ON_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s|\\s+s|\\s+is|\\s+else\\s+is)?\\s+(?:on|happening|going\\s+on)\\b|\\banything\\s+(?:on|happening)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEAL_TIME_POOL(py_tuple(vec![py_str("Meals are the part worth pinning down: decide the one thing you are eating and keep the rest plain, and the time sorts itself out. What have you got in the fridge?"), py_str("At that time of day the trap is deciding it late. Pick the food first - something you can do with one pan is plenty - and let the rest of it be whatever it is. What do you fancy?"), py_str("One thing cooked and one thing cold is a meal, and it takes about twenty minutes. Tell me what is in the house and I will say which of them to use.")]));
    s_PART_OF_DAY_POOL(py_tuple(vec![py_str("The part of the day that works has one thing in it you actually want to do, and nothing else booked round it. What would that one thing be?"), py_str("Fill it from the other end: what would you be sorry to have missed by bedtime? That is the one thing, and the rest of the time can look after itself. What sounds good?"), py_str("Days like that are easier when the shape is set rather than the plan: one thing out of the house, one meal, and the rest left loose. What kind of thing do you enjoy doing?")]));
    s_THINGS_TO_DO_POOL(py_tuple(vec![py_str("A day that works most places is one big thing in the morning and nothing booked after it, though I don't know this place well enough to name the things: one worth the journey out, one long lunch, the rest loose. What do you usually enjoy doing?"), py_str("Naming places I have not been to would be guessing, so here is the shape of a day instead: one big thing in the morning, one long lunch, and nothing booked after it. Are you after coast and walking, or food and towns?"), py_str("Pick the one thing you would be sorry to miss and plan the day round that, though I can't recommend places I have not been to: it decides where you stay, when you eat and what you drop. What sort of thing do you like doing?")]));
    s_WANTS_A_THING_DONE(stdlib_call("re", "compile", &[py_str("\\b(?:what|anything|something|where)\\b[^?.!]{0,18}\\b(?:to do|to see|to visit)\\b|\\b(?:things|stuff) to do\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHAT_TO_DO(stdlib_call("re", "compile", &[py_str("\\bwhat (?:should|shall|can|could|do) (?:i|we|you) do\\b|\\bwhat (?:should|shall|can|could) (?:i|we|you) (?:see|visit)\\b|\\bwhats there to do\\b|\\bwhat is there to do\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_DAY_OUT(stdlib_call("re", "compile", &[op_add(&op_add(&gattr(&g_WANTS_A_THING_DONE(), "pattern"), &py_str("|")), &gattr(&g_ASKS_WHAT_TO_DO(), "pattern")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_OUT_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:about|to (?:sleep|stop|help|get|fix|make|keep)|for (?:my|the) )|\\b(?:in|at|round|around) the (?:morning|afternoon|evening|night|weekend|meantime|garden|kitchen|office|house|home|shed|garage|bedroom|bathroom|flat|car|shower|bath)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_IN_HAND(stdlib_call("re", "compile", &[py_str("\\bwith (?:it|them|him|her|this|that|an?|a)\\b|\\bwith (?:the|my) (?!kids|children|family|parents|boys|girls|toddler|baby|dog|grandparents|grandchildren|niece|nephew|in-laws)\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE_THERE(stdlib_call("re", "compile", &[py_str("\\b(?:there|here|nearby|round here|around here)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_DAY_OUT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:so|and|ok|okay|right|then|also|but)[\\s,]+)*(?:what|anything|something)\\b[^?.!]{0,18}\\b(?:to do|to see|to visit)\\b|^\\W*(?:(?:so|and|ok|okay|right|then|also|but)[\\s,]+)*(?:things|stuff) to do\\b|^\\W*(?:(?:so|and|ok|okay|right|then|also|but)[\\s,]+)*(?:what|where) (?:should|shall|can|could|do) (?:i|we|you) (?:do|see|visit)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRIP_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:going to|going away|off to|flying to|driving to|heading to|travelling to|traveling to|holiday|holidays|trip to|weekend in|week in|fortnight in|visiting|booked|days in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DAY_OUT_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|doctors?|dentist|gp|clinic|ward|surgery|a&e|court|work|school|nursery|college|uni|university|interview|funeral|wedding|christening|prison|jail|care home|vets?|physio|therapist|my mum|my dad|my sister|my brother|my son|my daughter|my kids|my gran|my nan|my grandma|my grandad|my husband|my wife)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_AFTER(py_str("(?!\\s*(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|few|a\\s+couple|next|this|last|the\\s+(?:next|new|coming|end)|january|february|march|april|may|june|july|august|september|october|november|december|spring|summer|autumn|winter)\\b)"));
    s_ABROAD(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("\\b(?:abroad|overseas|emigrat\\w*|expat\\w*|visa|residency|passport for|work permit|right to (?:live|work)|liv(?:e|ing) abroad|work(?:ing)? abroad)\\b|\\b(?:relocat|retir|buy(?:ing)? (?:a|an) (?:house|flat|place))\\w*\\b[^?.]{0,25}\\b(?:to|in|abroad)\\b"), &g_A_TIME_AFTER()), &py_str("|\\bmov\\w*\\b(?!\\s*out\\b)[^?.]{0,25}\\b(?:to|in|abroad)\\b")), &g_A_TIME_AFTER()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVING(stdlib_call("re", "compile", &[py_str("\\b(?:mov|relocat|retir|buy(?:ing)? (?:a|an) (?:house|flat|place))\\w*\\b[^?.]{0,25}\\b(?:to|in|abroad)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:abroad|overseas|emigrat\\w*|expat\\w*|planning to (?:move|go|leave)|going to (?:move|emigrate)|thinking of (?:moving|emigrating|leaving)|want(?:s|ed)? to (?:move|emigrate|live) (?:to|abroad|overseas))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NOT_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:phone|mobile|laptop|computer|pc|tablet|account|money|funds|savings|isa|pension|bin|box|boxes|photos?|files?|data|number|direct debit|standing order|everything|furniture|house|flat|home|office|job|team|city|town|area|school|bedroom|kitchen)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_PAPERWORK(stdlib_call("re", "compile", &[py_str("\\b(?:how do i (?:ask|raise|bring (?:it )?up|mention|broach)|who do i ask|ask (?:my|your|the) (?:boss|manager|employer)|talk to (?:my|your) (?:boss|manager)|send(?:ing)? (?:money|cash)|transfer(?:ring)? (?:money|cash)|ripped off|scam\\w*|how much (?:money|cash) (?:to|can)|mov(?:e|ing|ed) (?:in|on)\\b|(?:phone|mobile).{0,25}(?:charges?|roaming|data|contract|plan))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVING_HOUSE(stdlib_call("re", "compile", &[py_str("\\b(?:house|flat|home|office|job|team|city|town|area|neighbourhood|school)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNCIL_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:council|councils|council tax|local authority|permit|planning permission|byelaw\\w*|blue badge|housing benefit|bulky waste|licensing|premises licence|taxi licence)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PARKING_RULE(stdlib_call("re", "compile", &[py_str("\\b(?:park|parking)\\b[^?.]{0,30}\\b(?:allowed|legal|illegal|ok|okay|fine|ticket|permit|charge|charges|charged|free|sunday|sundays|resident\\w*|restriction\\w*|badge|disabled)\\b|\\b(?:allowed|legal|illegal|ok|okay|fine|against the rules)\\b[^?.]{0,25}\\bpark\\b|\\b(?:can|could|am i allowed to|are we allowed to|where can i|where should i|is it ok to|is it okay to)\\b[^?.]{0,20}\\bpark\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_THE_RULES(stdlib_call("re", "compile", &[py_str("\\b(?:can i|could i|do i|does it|how do i|how can i|what do i|what are the|whats the|what's the|am i allowed|is it (?:allowed|legal|ok|okay|possible)|are we allowed|will i|do we|who do i|where do i|how much is the|how about|what about|thoughts on|what do you think (?:of|about)|is it easy to|whats it like to|what's it like to|should we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICES_POOL(py_tuple(vec![py_str("A catalogue turns over every month with the rights, so I can't hold the list: {service}'s own search settles it in seconds, and says whether the thing is included or to pay for."), py_str("A catalogue turns over every month with the rights, so typing the title into {service}'s own search is what says whether it is there today - I can't see it from here, and it says whether that is a rental or part of the subscription."), py_str("What is on {service} changes with the rights, month to month, so I can't check it: its own search is the one that knows, and it says whether that is a rental or part of the subscription.")]));
    s_JOURNEY_POOL(py_tuple(vec![py_str("No live {mode} times from me, sorry: they are re-run all day and cancellations land late, which is why a journey planner has today's and I can't - where are you starting from, and when do you need to be there?"), py_str("No live {mode} times from me, sorry - they are re-run all morning and cancellations land later still, so a journey planner has the real ones and I can't: tell me the two ends and the time you have to be there and I'll help you plan around it.")]));
    s_ROUTE_ENDS_POOL(py_tuple(vec![py_str("On {start} to {end} the hour you set off decides it more than the miles do, and the bit at each end counts as much as the road between - I can't see the times or the traffic from here, sorry. A journey planner does it door to door, parking or platform included, and the one thing worth settling before you look is the time you have to be there."), py_str("What settles {start} to {end} is the hour more than the distance: the peak is where it goes wrong, and the queue at both ends counts as much as the road between them, so the same pair is two different journeys at eight and at eleven. I can't see the day's times or the traffic from here, so a planner is the one that gives both ends door to door - when do you need to be there?"), py_str("Where in each place you start and finish settles {start} to {end} more than the route between, so fix the door first and let a planner work back from the hour you have to be there. The times are the planner's, not mine, sorry.")]));
    s__THE_ENDS(stdlib_call("re", "compile", &[py_str("\\bto\\s+(?P<end>.+?)\\s+from\\s+(?P<start>.+?)\\s*[?.!,]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_ENDS_REVERSED(stdlib_call("re", "compile", &[py_str("\\bfrom\\s+(?P<start>.+?)\\s+to\\s+(?P<end>.+?)\\s*[?.!,]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_PLACE(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("on"), py_str("at"), py_str("in"), py_str("by"), py_str("for"), py_str("about"), py_str("and"), py_str("or"), py_str("but"), py_str("tomorrow"), py_str("today"), py_str("tonight"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("next"), py_str("this"), py_str("please"), py_str("thanks"), py_str("quickest"), py_str("fastest"), py_str("best"), py_str("cheapest"), py_str("easiest"), py_str("way"), py_str("route"), py_str("time"), py_str("times")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_QUALIFIER_AFTER_THE_PLACE(stdlib_call("re", "compile", &[py_str("\\s+(?P<word>by|via|with|in|on|upon|at|for|then|please|thanks|today|tomorrow|tonight|now|early|late|weekdays|this|next|around)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NAME_PART(call_builtin("frozenset", &[py_tuple(vec![py_str("trent"), py_str("thames"), py_str("tyne"), py_str("avon"), py_str("wye"), py_str("humber"), py_str("sea"), py_str("furness"), py_str("ashfield"), py_str("arden"), py_str("lyme"), py_str("wolds"), py_str("moss"), py_str("ribble"), py_str("derwent"), py_str("don"), py_str("aire"), py_str("clyde"), py_str("ness"), py_str("stour")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTE_POOL(py_tuple(vec![py_str("No timetables from me, sorry: they are re-cut every few months and the day's works change them, so I can't see today's - a journey planner has the options door to door: where from, and when do you need to be there?"), py_str("No timetables from me, sorry - they are re-cut every few months, so the planner is the one that knows the real times between two places, and I can't: tell me the two ends and the day and I'll help you pick.")]));
    s_PICK_POOL(py_tuple(vec![py_str("Reviews beat a guess from me on the {thing} near you, which I have never been to: a map app's, sorted by what you need, is the honest way - say what matters and I'll help you narrow it down."), py_str("Reviews from people who have been beat my guess on the {thing} near you, since I have never been to any of them: sort a map app's by what you need, and the count matters more than the score.")]));
    s_SHOP_POOL(py_tuple(vec![py_str("Stock, prices and delivery change by the hour and they are {shop}'s own to know, so I can't see them: its site has them worked out for your postcode, and the checkout tells you before you pay."), py_str("Stock, prices and delivery are {shop}'s own to know and they change by the hour, so there is no window on them from here: its site has the answer, and the checkout tells you before you pay.")]));
    s_ABROAD_POOL(py_tuple(vec![py_str("What paperwork you need depends on your passport and what you would be doing there, so I can't tell you which one applies: gov.uk's living abroad guidance and the country's own government site are the two that count."), py_str("It depends on your nationality and your plans, so the official pages are the ones to read: gov.uk's living abroad guidance, then the destination's own government site.")]));
    s_WEATHER_POOL(py_tuple(vec![py_str("A forecast is worked out for a postcode and re-run through the day, so I can't hold today's: a weather app has it for yours, hour by hour, and the week ahead when you are planning."), py_str("A forecast is re-run for a postcode through the day, so there is no live weather from me: a forecast app or the national weather site has what is actually coming where you are.")]));
    s_A_SKY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:will|would|is|are|does|do|should|might|shall|can|could)\\s+(?:it|we|they)\\s+(?:(?:going\\s+to|gonna|meant\\s+to|likely\\s+to|supposed\\s+to)\\s+)?(?:be\\s+|get\\s+|stay\\s+|turn\\s+)?(?:raining|rainy|rain|snowing|snowy|snow|frost|frosty|freezing|freeze|thaw\\w*|icy|ice|hail|hailing|sleet|foggy|fog|misty|mist|windy|gusty|gusts|thunder|stormy|storm|sunny|sunshine|cloudy|drizzle|drizzling|showers?|humid|muggy|warm|cold|hot|chilly|mild)\\b|\\b(?:is|will|are)\\s+there\\s+(?:a\\s+|an\\s+|any\\s+|much\\s+)?(?:frost|ice|snow|rain|hail|sleet|fog|mist|wind|showers?|sun|thunder|storm)\\b|\\b(?:is|are|will)\\s+there\\s+(?:going\\s+to\\s+be|gonna\\s+be|likely\\s+to\\s+be|be)\\s+(?:a\\s+|an\\s+|any\\s+|much\\s+)?(?:frost|ice|snow|rain|hail|sleet|fog|mist|wind|showers?|sun|thunder|storms?|heat|heatwaves?)\\b|\\b(?:any|much)\\s+(?:chance|risk|sign)\\s+of\\s+(?:a\\s+|any\\s+)?(?:frost|ice|snow|rain|sleet|hail|fog|wind|thunder|showers?)\\b|\\b(?:are|is|will)\\s+we\\s+(?:due|in\\s+for)\\s+(?:a\\s+|any\\s+|some\\s+)?(?:frost|ice|snow|rain|sleet|hail|fog|wind|thunder|showers?|storm|sun)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SETTING(stdlib_call("re", "compile", &[py_str("\\b(?:oven|grill|hob|fridge|freezer|microwave|kettle|boiler|radiator|cooker|washing machine|tumble dryer|dishwasher|iron|shower|bath|sauna|engine|car|van|room|bedroom|house|flat|dough|batter|sourdough)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNCIL_POOL(py_tuple(vec![py_str("The rules differ from one council to the next and change with the year, so I would not guess at yours: its own site has the rule and the form, and the phone line will tell you which applies to you."), py_str("It depends on where you live and no two councils do it the same, so I can't see your council's rules from here: its own site is the one that decides it, and it has the form.")]));
    s_CHOICE_POOL(py_tuple(vec![py_str("Decide it on the one thing it must do well for you and compare only on that; the rest of the difference rarely matters. What must it do well?"), py_str("What settles these is fit rather than quality, so name the job it has to do and the limit you cannot go past - money, space or time. Which of those is tighter?"), py_str("When two are close, what separates them is how they are in use rather than on paper. What will you use it for most?")]));
    s_SHORTLIST_POOL(py_tuple(vec![py_str("A shortlist needs the thing you are starting from: what you are choosing between, and the one limit you cannot go past. Give me those two and I will put three or four in order."), py_str("I can narrow it once I know what is on the table and what would make one of them useless to you. What is on the table?"), py_str("The useful part is what rules one out - money, timing, effort, or a line you will not cross. Tell me that and what you are choosing from.")]));
    s_FORM_FROM_POOL(py_tuple(vec![py_str("The form comes from the organisation that wants it, and their own site has it under the name of the thing you are applying for. Their contact page will post one if it is not there. Which organisation is it?"), py_str("Start from the letter or page that asked you: it names the office, and that office's site has the form to download or to fill in online. Their number can post one out. Do you still have it?")]));
    s_FORM_TO_POOL(py_tuple(vec![py_str("It goes to the address printed on the form itself - not to a general office - and a photo of it before it goes is what settles a dispute later. Which one is it?"), py_str("The letter that came with it names the office and the address to return it to; if it is missing, their contact page has one and a ring will confirm which. Do you have the letter?")]));
    s_ONLY_A_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:and|but|ok|okay|so|also)\\s+what\\s+about\\b|^what\\s+about\\b|^(?:and|but|also)\\s+(?:the|my|your|a|an|those|these|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAS_A_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|has|have|had|does|do|did|can|could|will|would|should|hurts?|feels?|seems?|goes|went|keeps?|works?|tastes?|smells?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION_OF_ITS_OWN(stdlib_call("re", "compile", &[py_str("[?,]\\s*(?:is|are|was|were|does|do|did|can|could|will|would|should|how|why|when|where|what|who|which)\\b|\\b(?:is|are|does|do|did|can|could|will|would|should)\\s+(?:it|that|they|this|i|we|you|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEW_WORDS(Py::Int(5));
    s_POINTING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("any"), py_str("some"), py_str("my"), py_str("your"), py_str("our"), py_str("their"), py_str("its"), py_str("his"), py_str("her"), py_str("these"), py_str("those"), py_str("that"), py_str("this"), py_str("near"), py_str("nearby"), py_str("here"), py_str("there"), py_str("around"), py_str("round"), py_str("about"), py_str("in"), py_str("on"), py_str("at"), py_str("to"), py_str("for"), py_str("like"), py_str("instead"), py_str("then"), py_str("too"), py_str("and"), py_str("or"), py_str("please"), py_str("though")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECT_KINDS(py_tuple(vec![py_tuple(vec![py_str("council"), stdlib_call("re", "compile", &[py_str("\\b(?:bins?|rubbish|recycling|refuse|potholes?|librar(?:y|ies)|leisure cent(?:re|er)|bulky waste|council tax|parking permits?|green waste|fly ?tipping|grass cutting|school places|school catchments?|catchment areas?|catchments?|admissions|schools?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("map"), stdlib_call("re", "compile", &[py_str("\\b(?:food|restaurants?|pubs?|cafes?|coffee shops?|takeaways?|take-aways?|beaches|walks|things to do|days? out|markets?|parks?|chippy|chip shops?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("journey"), stdlib_call("re", "compile", &[py_str("\\b(?:trains?|buses|coach|trams?|ferr(?:y|ies)|the drive|traffic)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]));
    s__SUBJECT_LABEL(py_dict(vec![(py_str("food"), py_str("restaurants")), (py_str("restaurant"), py_str("restaurants")), (py_str("restaurants"), py_str("restaurants")), (py_str("pub"), py_str("pubs")), (py_str("pubs"), py_str("pubs")), (py_str("cafe"), py_str("cafes")), (py_str("cafes"), py_str("cafes")), (py_str("coffee shop"), py_str("cafes")), (py_str("coffee shops"), py_str("cafes")), (py_str("takeaway"), py_str("takeaways")), (py_str("takeaways"), py_str("takeaways")), (py_str("take-away"), py_str("takeaways")), (py_str("take-aways"), py_str("takeaways")), (py_str("beach"), py_str("beaches")), (py_str("beaches"), py_str("beaches")), (py_str("walk"), py_str("walks")), (py_str("walks"), py_str("walks")), (py_str("thing to do"), py_str("things to do")), (py_str("things to do"), py_str("things to do")), (py_str("day out"), py_str("days out")), (py_str("days out"), py_str("days out")), (py_str("market"), py_str("markets")), (py_str("markets"), py_str("markets")), (py_str("park"), py_str("parks")), (py_str("parks"), py_str("parks")), (py_str("chippy"), py_str("chip shops")), (py_str("chip shop"), py_str("chip shops")), (py_str("chip shops"), py_str("chip shops"))]));
    s__SUBJECT_MODE(py_dict(vec![(py_str("trains"), py_str("train")), (py_str("buses"), py_str("bus")), (py_str("trams"), py_str("tram")), (py_str("ferries"), py_str("ferry")), (py_str("coach"), py_str("coach")), (py_str("the drive"), py_str("travel")), (py_str("traffic"), py_str("travel"))]));
    s_WANTS_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\b(?:which|what)\\s+(?:is|s|'s|would be|will be)\\s+(?:the\\s+)?(?:better|best|cheaper|stronger|safer|easier|quicker|healthier|nicer)\\b|\\bwhich (?:one )?(?:should|would|do) i (?:get|buy|use|go for|pick|choose|have)\\b|\\b(?:is|are)\\s+[a-z][a-z' -]{0,24}?\\s+or\\s+[a-z][a-z' -]{0,24}?\\s+(?:better|best|worth it|the one)\\b|\\bwhat would you (?:go for|pick|choose|get)\\b|\\bis it better to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\b(?:for a|for my|for the)\\s+(?:burn|cut|rash|cough|cold|headache|baby|child|dog|cat)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_SHORTLIST(stdlib_call("re", "compile", &[py_str("\\b(?:short ?list|a list of|(?:a few|some|a couple of|three|two) (?:options|ideas|suggestions|choices|picks)|options (?:for|on)|what are my options|what are the options|(?:give|gimme|send) me (?:a few|some|three|two|a couple)|(?:suggest|recommend) (?:a few|some|three|two))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SHORTLIST(stdlib_call("re", "compile", &[py_str("\\b(?:shopping list|ingredients list|list of ingredients|to do list|todo list|checklist|waiting list|price list|job list|shortlisted|list of names)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_A_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:get|getting|find|download|print|print off|obtain|ask for|request)\\b[^?.]{0,25}\\bforms?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SEND_A_FORM(stdlib_call("re", "compile", &[py_str("\\b(?:where|who|what address|which office|which address|how do i (?:send|return|post))\\b[^?.]{0,40}\\b(?:send|sending|post|return|returning|hand|hand in|submit|give)[^?.]{0,25}\\bforms?\\b|\\b(?:where|who)\\b[^?.]{0,40}\\bforms?\\b[^?.]{0,25}\\b(?:go|goes|going|back)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FORM(stdlib_call("re", "compile", &[py_str("\\bform of (?:words|government|address|money|id|identification)\\b|\\b(?:in|on) (?:good |top )?form\\b|\\bform an?\\b|\\bform (?:a|an)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_CHECK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|anything|something)\\b[^?.!]{0,26}\\b(?:should|shall|do|to|need to|must|can|could)\\b[^?.!]{0,24}\\b(?:check|checking|look for|look at|look over|look out for|watch out for|watch for|ask|be careful|be wary)\\b|\\bwhat to (?:check|look for|look at|ask)\\b|\\banything\\b[^?.!]{0,26}\\b(?:be careful|be wary|to watch out|to look out)\\b|\\b(?:what|anything)\\b[^?.!]{0,24}\\bto be careful (?:of|about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECK_OF_SOMETHING_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:on|about|for|in|with)\\s+(?:a|an|the|my|your|his|her|their|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHECK_SENTENCE(stdlib_call("re", "compile", &[py_str("\\bfine if\\b|\\bworth (?:checking|looking|a)\\b|\\bmatters?\\b|\\bwhat (?:settles|decides)\\b|\\bchecks?\\b|\\bchecking\\b|\\blook (?:for|at)\\b|\\bwatch (?:out )?for\\b|\\bsigns? of\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CHECK_SENTENCE(stdlib_call("re", "compile", &[py_str("\\?\\s*$|\\b(?:search online|look it up|not sure about that|tell me more)\\b|\\b(?:999|112|111|a&e|ambulance|hospital|nhs|gp|doctor|pharmacist|midwife|vet|urgent|emergency|straight away|right away)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION(stdlib_call("re", "compile", &[py_str("\\b(?:fine )?if (?:the|it|there|they|you|your|its)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECKS_POOL(py_tuple(vec![py_str("{checks} That is the list to go through, and I can't see it from here - which of them can you try yourself?"), py_str("{checks} Those are still the things that decide it, so work through them first and tell me what you find; I can only go on what you say about it."), py_str("{checks} Nothing to add to that: those are what goes wrong, in the order I would check them. Which of them are you least sure about?")]));
}
pub fn _a_named_country_in(args: &[Py]) -> Py {
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
    let mut names: Py = Py::None;
    let _ = py_str("True when the line names a country other than this one.");
    if py_truthy(&Py::Bool(py_is(&g__A_NAMED_COUNTRY(), &Py::None))) {
    let mut all_countries: Py = gen_knowledge::knowledge::geography::g_COUNTRIES();
    let mut _COUNTRY_ALIASES: Py = gen_knowledge::knowledge::geography::g__COUNTRY_ALIASES();
    names = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&op_bitor(&call_builtin("set", &[all_countries.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[_COUNTRY_ALIASES.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_MOVE_HOME_NAMES())); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    s__A_NAMED_COUNTRY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    }
    return call_builtin("bool", &[mcall(&g__A_NAMED_COUNTRY(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_time_of_day(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut meal: Py = Py::None;
    let mut meal_pool: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The reply for a plan asked about a meal or a part of the day, or None.\n\n    Only a line that names a time and asks what to do with it. A place named as well as the time\n    (\"what should we do in the evening in rome\") is the day-out reading's, a person or a ward is\n    not a plan at all, and a line that asks what to cook has a card of its own.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    meal = mcall(&g_A_MEAL_TIME(), "search", &[said.clone()]);
    part = mcall(&g_A_PART_OF_DAY(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = meal.clone(); if py_truthy(&__b) { __b } else { part.clone() } })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WANTS_A_PLAN(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_DAY_OUT_PLACE(), "search", &[said.clone()])) {
    return Py::None;
    }
    rest = said.clone();
    let __iter1 = py_iter_or_raise(&py_tuple(vec![meal.clone(), part.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&match_.clone()) {
    rest = mcall(&rest.clone(), "replace", &[mcall(&match_.clone(), "group", &[Py::Int(0)]), py_str(" ")]);
    }
    }
    if py_truthy(&{ let __b = mcall(&g_A_PLACE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PLACE_THERE(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    meal_pool = { let __b = meal.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AN_ON_ASK(), "search", &[said.clone()]))) } };
    return crate::compose::pick(&[if py_truthy(&meal_pool.clone()) { g_MEAL_TIME_POOL() } else { g_PART_OF_DAY_POOL() }, py_dict(vec![]), said.clone()]);
    Py::None
}
pub fn _the_route_ends(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The two places in a route ask, as the person wrote them, or None.\n\n    The line is a route question already, so the ends are the last two place-like runs of words\n    around \"from\" and \"to\". Anything longer than four words is not a place name a person typed,\n    and a line whose ends are only \"the\"/\"my\" has none to say back.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__THE_ENDS(), g__THE_ENDS_REVERSED()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    end = _a_place_name(&[mcall(&found.clone(), "group", &[py_str("end")])]);
    start = _a_place_name(&[mcall(&found.clone(), "group", &[py_str("start")])]);
    if py_truthy(&{ let __b = end.clone(); if !py_truthy(&__b) { __b } else { { let __b = start.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&end.clone(), "lower", &[]), &mcall(&start.clone(), "lower", &[]))) } } } }) {
    return py_tuple(vec![start.clone(), end.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _without_a_qualifier(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut run: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "run") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: run"))) };
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The place alone, with anything said after it about how or when cut off.");
    text = mcall(&call_builtin("str", &[{ let __b = run.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .?,'")]);
    let __iter1 = py_iter_or_raise(&mcall(&g__A_QUALIFIER_AFTER_THE_PLACE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("in"), py_str("on"), py_str("upon")]), &mcall(&mcall(&found.clone(), "group", &[py_str("word")]), "lower", &[])))) {
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_NAME_PART(), &mcall(&mcall(&geti(&rest.clone(), &Py::Int(0)), "strip", &[py_str(".,")]), "lower", &[]))) } }) {
    continue;
    }
    }
    return slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    }
    return text.clone();
    Py::None
}
pub fn _a_place_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut run: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "run") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: run"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&_without_a_qualifier(&[run.clone()]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_THE_PLACE(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("to"), py_str("from"), py_str("get"), py_str("my"), py_str("our")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("here"), py_str("there")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the person said in the last few turns, or \"\" on the first turn.\n\n    A follow-up names what is being asked about and not where: \"we're thinking of moving to spain\"\n    then \"what paperwork do i need\", and \"we're going to cornwall in august\" ... then \"any good\n    beaches for the kids\" are one question in two turns, and the second turn alone has no country\n    and no place in it. Only the kind is read from the earlier lines, never the answer, and the\n    last three are enough for a person to have changed the subject.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn _the_answers_before(args: &[Py]) -> Py {
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
    let mut count: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { Py::Int(4) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's own last few answers, newest first, or [] on the first turn.\n\n    Newest first because a person asking what to check is asking about the thing that is still on\n    the table: \"what should i check\" three turns after the price came up is about the price, and an\n    answer from before that is about something they have moved on from.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&count.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } py_list(__out) })() };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(line.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _the_checks_in(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of an answer that names the conditions, or \"\".\n\n    Taken whole, and not cut at a comma: the conditions *are* the sentence, and \"A second-hand one\n    is fine if the frame is straight\" without the brakes and the fabric is half the list the person\n    asked for. A long sentence is handed back as it is for the same reason; the only answers\n    skipped are the ones that are a question or a shrug, which name nothing.\n    ");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&mcall(&mcall(&sentence.clone(), "strip", &[]), "strip", &[py_str("\"'“”")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(320)) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_CHECK_SENTENCE(), "search", &[sentence.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&g_A_CHECK_SENTENCE(), "search", &[sentence.clone()])) {
    let _ = mcall(&found.clone(), "append", &[sentence.clone()]);
    }
    }
    let __iter2 = py_iter_or_raise(&found.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    if py_truthy(&mcall(&g_A_CONDITION(), "search", &[sentence.clone()])) {
    return sentence.clone();
    }
    }
    return if py_truthy(&found.clone()) { geti(&found.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _about_a_check(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answer: Py = Py::None;
    let mut checks: Py = Py::None;
    let _ = py_str("The checks the bot has named, handed back, or None.\n\n    The reply is the bot's own sentence with one line about what to do with it and, in two of the\n    three, a short question. Nothing is added to the list: a check this file made up would be a\n    fact about a pram or a boiler that the knowledge base has not got, which is the one thing a\n    last resort may not do. So an ask with no such sentence behind it stands down and the floor\n    answers as it did before.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&mcall(&g_WANTS_A_CHECK(), "search", &[said.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { mcall(&g_CHECK_OF_SOMETHING_ELSE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_the_answers_before(&[memory.clone()]));
    for __item in __iter1 {
    answer = __item.clone();
    checks = _the_checks_in(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&checks.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_str(".!?"), &geti(&checks.clone(), &op_neg(&Py::Int(1)))))) {
    let __aug2 = op_add(&checks.clone(), &py_str("."));
    checks = __aug2;
    }
    return mcall(&crate::compose::pick(&[g_CHECKS_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]), "replace", &[py_str("{checks}"), checks.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _named_service(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The service a line names, as (the word in the line, the name on screen), or (\"\", \"\").\n\n    The longest name wins, because \"bbc iplayer\" and \"iplayer\" are the same service and \"sky go\"\n    is a narrower thing than \"sky\".\n    ");
    best = py_str("");
    let __iter1 = py_iter_or_raise(&g_SERVICES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    shown = __unpk2[1].clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![word.clone(), shown.clone()]), 2, None);
    best = __unpk3[0].clone();
    chosen = __unpk3[1].clone();
    }
    }
    return if py_truthy(&best.clone()) { py_tuple(vec![best.clone(), chosen.clone()]) } else { py_tuple(vec![py_str(""), py_str("")]) };
    Py::None
}
pub fn _about_a_service(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut platform: Py = Py::None;
    let mut service: Py = Py::None;
    let mut word: Py = Py::None;
    let __unpk1 = py_unpack(&_named_service(&[said.clone()]), 2, None);
    word = __unpk1[0].clone();
    service = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_AVAILABILITY(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    platform = stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:on|from|at|via|in)\\s+(?:the\\s+)?"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b|\\b")), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:'s|s')\\b|\\b")), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\s+(?:has|have|got|shows|show|streams|stream|adds)\\b|\\b(?:does|do|did|is|are|has|have)\\s+(?:the\\s+)?")), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&platform.clone(), "search", &[said.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:is|are|was|were|isnt|arent|any|anything|do|does|did|can|could|where|how)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return mcall(&crate::compose::pick(&[g_SERVICES_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]), "replace", &[py_str("{service}"), service.clone()]);
    Py::None
}
pub fn _about_a_shop(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The shop a line asks about, in the form the reply uses, or \"\".\n\n    The name alone is not the shop: \"my boots creak when i walk\" is about footwear and \"boots\"\n    there is a pair of them, so the name has to be in a frame that makes it a shop - at, in, from,\n    to, \"does\", or its own possessive.\n    ");
    best = py_str("");
    let __iter1 = py_iter_or_raise(&g_SHOPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    shown = __unpk2[1].clone();
    frame = stdlib_call("re", "search", &[op_add(&op_add(&op_add(&op_add(&py_str("(?:\\b(?:at|in|from|to|does|do|did|is|are|asda's|with)\\s+(?:the\\s+)?"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b)|(?:\\b")), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:'s|s')\\b)")), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = frame.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![word.clone(), shown.clone()]), 2, None);
    best = __unpk3[0].clone();
    chosen = __unpk3[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SHOP_ASK(), "search", &[said.clone()]))) } }) {
    return py_str("");
    }
    return chosen.clone();
    Py::None
}
pub fn _the_mode_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mode: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mode") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mode"))) };
    let mut single: Py = Py::None;
    let _ = py_str("The service noun as the pools need it: a modifier before \"times\", so always singular.\n\n    The line can name the service in the plural - \"how often do the buses run\", \"how often do the\n    trains run on sunday\" - and the slot it fills is \"No live {mode} times\", where the noun is\n    doing the work of an adjective. So a plural is put in the singular there; \"no live buses\n    times\" is not a sentence anybody says. Which plural belongs to which singular is already the\n    table above, so nothing is guessed from the ending of the word.\n    ");
    single = geti(&{ let __b = mcall(&g_TRAVEL(), "get", &[mcall(&{ let __b = mode.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]); if py_truthy(&__b) { __b } else { py_tuple(vec![py_str(""), py_str("")]) } }, &Py::Int(0));
    return { let __b = single.clone(); if py_truthy(&__b) { __b } else { mode.clone() } };
    Py::None
}
pub fn _about_a_journey(args: &[Py]) -> Py {
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
    let mut mode: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut single: Py = Py::None;
    let mut word: Py = Py::None;
    if py_truthy(&{ let __b = mcall(&g_GET_FROM_TO(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_DESTINATION(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TRAVEL()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return py_str("route");
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_TRAVEL(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    single = __unpk3[0].clone();
    plural = __unpk3[1].clone();
    noun = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:the|a|an|my|your|our|his|her|their|this|that|these|those|last|next|first|any|every|no|by|take the|catch the|get the)\\s+"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&noun.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_JOURNEY(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WANTS_A_TIME(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("tube"), py_str("metro")]), &word.clone()))) {
    mode = py_str("Tube");
    } else {
    mode = single.clone();
    }
    return mode.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_country_not_home(args: &[Py]) -> Py {
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
    let _ = py_str("The first country in the line that is not this one, or \"\".\n\n    Read from the knowledge base's own country list rather than a short list of holiday names,\n    because the thread can leave the UK in any direction and a council is then the wrong body.\n    ");
    let __iter1 = py_iter_or_raise(&g_NOT_HOME_COUNTRIES());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _about_the_subject_named(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut kind: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut left: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut words_in_line: Py = Py::None;
    let _ = py_str("The limit and the place that knows, for a follow-up that names only a subject.\n\n    \"what about the schools\" after a house move is a question about admissions, and it went to\n    the search shrug because there is no verb to read. The noun is the whole of it, and it names\n    a kind with a place behind it, so the answer here is the same one the long version of the\n    line would have got. Anything else - a subject nobody keeps (\"what about the parsnips\",\n    \"what about a stainless steel one\") - is left to the floor, which is where it was.\n    ");
    lead = mcall(&g_ONLY_A_SUBJECT(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&lead.clone()))) {
    return Py::None;
    }
    rest = mcall(&slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_FEW_WORDS()); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION_OF_ITS_OWN(), "search", &[said.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&slice_or_raise(&said.clone(), &mcall(&lead.clone(), "start", &[]), &mcall(&lead.clone(), "end", &[]), &Py::None), "strip", &[]), "startswith", &[py_tuple(vec![py_str("and"), py_str("but"), py_str("also")])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("what about"), slice_or_raise(&said.clone(), &Py::None, &mcall(&lead.clone(), "end", &[]), &Py::None), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    if py_truthy(&mcall(&g_HAS_A_VERB(), "search", &[said.clone()])) {
    return Py::None;
    }
    }
    words_in_line = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("?,.!'")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&g_SUBJECT_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    words = __unpk2[1].clone();
    named = mcall(&words.clone(), "search", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    continue;
    }
    left = call_builtin("list", &[words_in_line.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter3 = py_iter_or_raise(&mcall(&mcall(&mcall(&named.clone(), "group", &[Py::Int(0)]), "lower", &[]), "split", &[]));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&left.clone(), &word.clone()))) {
    let _ = mcall(&left.clone(), "remove", &[word.clone()]);
    }
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTING_WORDS(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("council")))) {
    if py_truthy(&{ let __b = mcall(&g_ABROAD(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_country_not_home(&[before.clone()]); if py_truthy(&__b) { __b } else { _a_country_not_home(&[said.clone()]) } } } }) {
    return Py::None;
    }
    return crate::compose::pick(&[g_COUNCIL_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("journey")))) {
    word = mcall(&mcall(&named.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    return mcall(&crate::compose::pick(&[g_JOURNEY_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]), "replace", &[py_str("{mode}"), _the_mode_named(&[mcall(&g__SUBJECT_MODE(), "get", &[word.clone(), word.clone()])])]);
    }
    thing = mcall(&g__SUBJECT_LABEL(), "get", &[mcall(&mcall(&named.clone(), "group", &[Py::Int(0)]), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_PLACE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\b(?:there|here|nearby|local)\\b"), rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PLACE(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:there|here|nearby|local)\\b"), before.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }))) {
    return Py::None;
    }
    return mcall(&crate::compose::pick(&[g_PICK_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]), "replace", &[py_str("{thing}"), thing.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _about_a_pick(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut blocked: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The kind of place a line asks to be told about, or None.\n\n    Two things have to hold, and both of them are about the word being the thing asked *for*\n    rather than a place named on the way: an ask word (\"best\", \"any good\", \"recommend\") has to come\n    just before it, and it must not be the object of a preposition of place, so \"anywhere nice to\n    eat near the beach\" is a question about eating and not about beaches.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_PLACE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PLACE(), "search", &[before.clone()]))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_PICKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    thing = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    let __iter4 = py_iter_or_raise(&stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    match_ = __item.clone();
    blocked = mcall(&g_PICK_NOT(), "get", &[geti(&words.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&blocked.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&blocked.clone(), "search", &[said.clone()]) } }) {
    break;
    }
    before = slice_or_raise(&said.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(26))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:in|near|around|round|at|by|beside|on|from|to)\\s+(?:the\\s+)?$"), before.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WANTS_A_PICK(), "search", &[before.clone()])))) {
    continue;
    }
    return thing.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_home_town(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The town the person's own words already put them in, lowercased, or ().\n\n    The understanding engine files a move already made as a home (\"i just moved to bristol\" is\n    `home: true`, `city: Bristol`), and a move being planned as a place or a plan instead\n    (\"we're thinking of moving to spain\" is a place with no home), so this reads the one and\n    not the other.\n\n    A country is not a town, and the engine files \"i moved to france last year\" the same way it\n    files the move to Bristol - `home: true`, `city: France` - so the guard read that home as the\n    town they are in and stood the paperwork down for the person who needs it most: \"what\n    paperwork do i need to stay\" after moving to France was answered with the search hedge. The\n    guard is about a town the move brought them to, so a country named as the home is left out -\n    except the four nations of the UK, where a move is a move at home and \"what classes are on\"\n    after \"i moved to scotland\" is the class question, not the paperwork.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![]);
    }
    user = mcall(&memory.clone(), "get", &[py_str("user")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[user.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&user.clone(), "get", &[py_str("home")]))) } }) {
    return py_tuple(vec![]);
    }
    plans = mcall(&user.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    plans = py_tuple(vec![plans.clone()]);
    }
    if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut plan: Py = __item.clone(); __out.push({ let __b = mcall(&g_GOING_AWAY(), "search", &[call_builtin("str", &[plan.clone()]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_MOVING(), "search", &[call_builtin("str", &[plan.clone()]).unwrap_or_else(|e| exc::raise(e))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![]);
    }
    }
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("city"), py_str("town"), py_str("village"), py_str("hometown"), py_str("home_city")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&user.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[mcall(&mcall(&value.clone(), "strip", &[]), "lower", &[])]);
    }
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HOME_NATIONS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&gen_memory::memory::facts::g_COUNTRIES(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_lines_of_this_question(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last few turns and this one, oldest first, as the person said them.\n\n    `_the_line_before` glues the earlier lines into one string, which is enough to say that\n    something was said about moving and not enough to say which came last. A move already made\n    and a move still to come are told apart by which of them is the newer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![said.clone()]);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    earlier = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    return op_add(&earlier.clone(), &py_list(vec![said.clone()]));
    Py::None
}
pub fn _the_move_was_to_their_own_town(args: &[Py]) -> Py {
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
    let mut destination: Py = Py::None;
    let mut line: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let _ = py_str("True when the newest thing said about moving is the move that brought them where they are.\n\n    \"i just moved to bristol\" is a statement about the town the person lives in now, and it was\n    still being read three turns later: after \"whats a good way to meet people here\" and \"i dont\n    really drink\", the line \"ok what about classes or clubs\" was answered with the paperwork for\n    living abroad, because \"moved to\" is a move and nothing checked the destination. A place\n    already in their own words as where they are is not a somewhere-else they are planning.\n\n    Only the newest line that talks about moving counts. \"i just moved to bristol\" and then\n    \"we are planning to emigrate\" is a person in Bristol who is still going, and the paperwork\n    answer is theirs; the home move is only the last word on the subject when nothing after it\n    says the move is still to come.\n    ");
    places = _the_home_town(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_the_lines_of_this_question(&[said.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    move_ = mcall(&g_MOVING(), "search", &[line.clone()]);
    if py_truthy(&move_.clone()) {
    destination = slice_or_raise(&line.clone(), &mcall(&move_.clone(), "end", &[]), &op_add(&mcall(&move_.clone(), "end", &[]), &Py::Int(30)), &Py::None);
    let __iter2 = py_iter_or_raise(&places.clone());
    for __item in __iter2 {
    place = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?:^|[^a-z])"), &stdlib_call("re", "escape", &[place.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:[^a-z]|$)")), destination.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_GOING_AWAY(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_day_out(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let _ = py_str("True when the line asks what to do or see in the place the thread is already about.\n\n    The place has to be somewhere real: named on this line (\"what should we see in rome\"), or\n    only pointed at (\"what should i do there\", \"anything good to do while we are there\") with a\n    trip in the last few lines, which is how a follow-up is asked. A line with an object of its\n    own (\"what should i do about my knee\", \"what should i do with it\") is not this reading,\n    neither is a day at home (\"what should i do in the garden\" is the garden's), and neither is\n    a visit to a ward or to see somebody.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_WANTS_A_DAY_OUT(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_DAY_OUT_NOT(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_ASKS_WHAT_TO_DO(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_THING_IN_HAND(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_PLACE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_DAY_OUT_PLACE(), "search", &[said.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_PLACE_THERE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_BARE_DAY_OUT(), "search", &[said.clone()]) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_TRIP_BEFORE(), "search", &[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_DAY_OUT_PLACE(), "search", &[before.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_the_rules(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut both: Py = Py::None;
    let mut strong: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WANTS_THE_RULES(), "search", &[said.clone()])))) {
    return Py::None;
    }
    both = op_add(&op_add(&said.clone(), &py_str(" ")), &before.clone());
    strong = stdlib_call("re", "search", &[py_str("\\b(?:abroad|overseas|emigrat\\w*|expat\\w*|visa|residency|passport for|work permit|right to (?:live|work)|country)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_ABROAD(), "search", &[both.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = strong.clone(); if py_truthy(&__b) { __b } else { _a_named_country_in(&[both.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_the_move_was_to_their_own_town(&[said.clone(), memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_THE_PAPERWORK(), "search", &[said.clone()]))) } } } } } }) {
    if py_truthy(&{ let __b = { let __b = strong.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_THING_NOT_A_PLACE(), "search", &[said.clone()]))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_MOVING_HOUSE(), "search", &[both.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:abroad|overseas|country)\\b"), both.clone()]).unwrap_or_else(|e| exc::raise(e)))) } })) } }) {
    return crate::compose::pick(&[g_ABROAD_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    }
    }
    if py_truthy(&{ let __b = mcall(&g_COUNCIL_WORD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PARKING_RULE(), "search", &[said.clone()]) } }) {
    return crate::compose::pick(&[g_COUNCIL_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _a_form(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("Where a form comes from, or where it goes back to, or None.");
    if py_truthy(&mcall(&g_NOT_A_FORM(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_SEND_A_FORM(), "search", &[said.clone()])) {
    return crate::compose::pick(&[g_FORM_TO_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    }
    if py_truthy(&mcall(&g_GET_A_FORM(), "search", &[said.clone()])) {
    return crate::compose::pick(&[g_FORM_FROM_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _a_choice(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("The deciding factor for a choice between two things, or None.\n\n    \"Which is better, water or oil based\" names no subject the bot can look up, and a search is\n    a poor second to saying the one thing that is true of every choice of that shape: what it is\n    used for and how hard it is worked decides it. The question at the end is the way in.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_WANTS_A_CHOICE(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_CHOICE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    return crate::compose::pick(&[g_CHOICE_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    Py::None
}
pub fn _a_shortlist(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("What a shortlist has to come from, or None: the bot cannot draw one out of nothing.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_WANTS_A_SHORTLIST(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_SHORTLIST(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    return crate::compose::pick(&[g_SHORTLIST_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    Py::None
}
pub fn _about_the_sky(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("The weather a line asks about, as a reply naming where it is read, or None.\n\n    The composer has its own weather line for the asks it recognises (\"whats the weather like\",\n    \"is it going to rain on saturday\"), and this reading only ever runs on a line the composer\n    left at the shrug: a sky word in an asking frame that it did not have words for, which is\n    every frost, ice, hail, sleet, fog, mist, gust, thunder, drizzle, muggy or freeze question.\n    A reading of a thing rather than the sky - the oven, the fridge, the car - stands down, so\n    \"will it be hot in the oven\" keeps the roast answer it has.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_SKY_ASK(), "search", &[said.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_SETTING(), "search", &[said.clone()])) {
    return Py::None;
    }
    return crate::compose::pick(&[g_WEATHER_POOL(), { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]);
    Py::None
}
pub fn the_checks_already_named(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut candidate: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The checks the bot named earlier, handed back, or None.\n\n    The same reading `answer_for` makes, under a name `bot/modules/greeting.py` can call for the\n    one kind of line that does not always reach it as a hedge: \"anything i should be careful of\"\n    after the price of a second-hand pram is filed by the composer's neutral pool (\"Got it. Is\n    there anything you'd like a hand with?\"), which is a receipt rather than an answer, and the\n    receipt is what the reading replaces there.\n    ");
    said = _said(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::None;
    }
    read = py_dict(vec![]);
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    candidate = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&call_builtin("isinstance", &[candidate.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    read = candidate.clone();
    }
    }
    return _about_a_check(&[said.clone(), read.clone(), memory.clone()]);
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut before: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut checks: Py = Py::None;
    let mut ends: Py = Py::None;
    let mut mode: Py = Py::None;
    let mut read: Py = Py::None;
    let mut rules: Py = Py::None;
    let mut said: Py = Py::None;
    let mut service: Py = Py::None;
    let mut shop: Py = Py::None;
    let mut sky: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut time_of_day: Py = Py::None;
    let _ = py_str("The honest limit and the place that knows, or None when the line is not one of these.\n\n    Only six kinds are read, and each one names a place the person can go to today or hands back\n    the checks the bot gave earlier. A line that fits none of them is left to the floor, which is\n    where it was before this module existed.\n    ");
    said = _said(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::None;
    }
    read = py_dict(vec![]);
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    candidate = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&call_builtin("isinstance", &[candidate.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    read = candidate.clone();
    }
    }
    service = _about_a_service(&[said.clone(), read.clone()]);
    if py_truthy(&service.clone()) {
    return service.clone();
    }
    sky = _about_the_sky(&[said.clone(), read.clone()]);
    if py_truthy(&sky.clone()) {
    return sky.clone();
    }
    mode = _about_a_journey(&[said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&mode.clone(), &py_str("route")))) {
    ends = _the_route_ends(&[said.clone()]);
    if py_truthy(&ends.clone()) {
    return mcall(&mcall(&crate::compose::pick(&[g_ROUTE_ENDS_POOL(), read.clone(), said.clone()]), "replace", &[py_str("{start}"), geti(&ends.clone(), &Py::Int(0))]), "replace", &[py_str("{end}"), geti(&ends.clone(), &Py::Int(1))]);
    }
    return crate::compose::pick(&[g_ROUTE_POOL(), read.clone(), said.clone()]);
    }
    if py_truthy(&mode.clone()) {
    return mcall(&crate::compose::pick(&[g_JOURNEY_POOL(), read.clone(), said.clone()]), "replace", &[py_str("{mode}"), _the_mode_named(&[mode.clone()])]);
    }
    shop = _about_a_shop(&[said.clone()]);
    if py_truthy(&shop.clone()) {
    return mcall(&crate::compose::pick(&[g_SHOP_POOL(), read.clone(), said.clone()]), "replace", &[py_str("{shop}"), shop.clone()]);
    }
    before = _the_line_before(&[memory.clone()]);
    time_of_day = _a_time_of_day(&[said.clone()]);
    if py_truthy(&time_of_day.clone()) {
    return time_of_day.clone();
    }
    if py_truthy(&_a_day_out(&[said.clone(), before.clone()])) {
    return crate::compose::pick(&[g_THINGS_TO_DO_POOL(), read.clone(), said.clone()]);
    }
    thing = _about_a_pick(&[said.clone(), before.clone()]);
    if py_truthy(&thing.clone()) {
    return mcall(&crate::compose::pick(&[g_PICK_POOL(), read.clone(), said.clone()]), "replace", &[py_str("{thing}"), thing.clone()]);
    }
    subject = _about_the_subject_named(&[said.clone(), read.clone(), before.clone()]);
    if py_truthy(&subject.clone()) {
    return subject.clone();
    }
    checks = _about_a_check(&[said.clone(), read.clone(), memory.clone()]);
    if py_truthy(&checks.clone()) {
    return checks.clone();
    }
    rules = _about_the_rules(&[said.clone(), read.clone(), before.clone(), memory.clone()]);
    if py_truthy(&rules.clone()) {
    return rules.clone();
    }
    return { let __b = _a_choice(&[said.clone(), read.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_shortlist(&[said.clone(), read.clone()]); if py_truthy(&__b) { __b } else { _a_form(&[said.clone(), read.clone()]) } } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.checking".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABROAD" => Some(g_ABROAD()),
            "ABROAD_POOL" => Some(g_ABROAD_POOL()),
            "AN_ON_ASK" => Some(g_AN_ON_ASK()),
            "ASKS_WHAT_TO_DO" => Some(g_ASKS_WHAT_TO_DO()),
            "A_BARE_DAY_OUT" => Some(g_A_BARE_DAY_OUT()),
            "A_CHECK_SENTENCE" => Some(g_A_CHECK_SENTENCE()),
            "A_CONDITION" => Some(g_A_CONDITION()),
            "A_FEW_WORDS" => Some(g_A_FEW_WORDS()),
            "A_MEAL_TIME" => Some(g_A_MEAL_TIME()),
            "A_NOT_A_PLACE" => Some(g_A_NOT_A_PLACE()),
            "A_PART_OF_DAY" => Some(g_A_PART_OF_DAY()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_PLACE_THERE" => Some(g_A_PLACE_THERE()),
            "A_QUESTION_OF_ITS_OWN" => Some(g_A_QUESTION_OF_ITS_OWN()),
            "A_SETTING" => Some(g_A_SETTING()),
            "A_SKY_ASK" => Some(g_A_SKY_ASK()),
            "A_THING_IN_HAND" => Some(g_A_THING_IN_HAND()),
            "A_THING_NOT_A_PLACE" => Some(g_A_THING_NOT_A_PLACE()),
            "A_TIME_AFTER" => Some(g_A_TIME_AFTER()),
            "A_TIME_NOT_A_PLACE" => Some(g_A_TIME_NOT_A_PLACE()),
            "A_TRIP_BEFORE" => Some(g_A_TRIP_BEFORE()),
            "CHECKS_POOL" => Some(g_CHECKS_POOL()),
            "CHECK_OF_SOMETHING_ELSE" => Some(g_CHECK_OF_SOMETHING_ELSE()),
            "CHOICE_POOL" => Some(g_CHOICE_POOL()),
            "COUNCIL_POOL" => Some(g_COUNCIL_POOL()),
            "COUNCIL_WORD" => Some(g_COUNCIL_WORD()),
            "COUNTRIES" => Some(g_COUNTRIES()),
            "DAY_OUT_NOT" => Some(g_DAY_OUT_NOT()),
            "FORM_FROM_POOL" => Some(g_FORM_FROM_POOL()),
            "FORM_TO_POOL" => Some(g_FORM_TO_POOL()),
            "GET_A_FORM" => Some(g_GET_A_FORM()),
            "GET_FROM_TO" => Some(g_GET_FROM_TO()),
            "GOING_AWAY" => Some(g_GOING_AWAY()),
            "HAS_A_VERB" => Some(g_HAS_A_VERB()),
            "HOME_COUNTRIES" => Some(g_HOME_COUNTRIES()),
            "HOME_NATIONS" => Some(g_HOME_NATIONS()),
            "JOURNEY_POOL" => Some(g_JOURNEY_POOL()),
            "MEAL_TIME_POOL" => Some(g_MEAL_TIME_POOL()),
            "MOVE_HOME_NAMES" => Some(g_MOVE_HOME_NAMES()),
            "MOVING" => Some(g_MOVING()),
            "MOVING_HOUSE" => Some(g_MOVING_HOUSE()),
            "NOT_AVAILABILITY" => Some(g_NOT_AVAILABILITY()),
            "NOT_A_CHECK_SENTENCE" => Some(g_NOT_A_CHECK_SENTENCE()),
            "NOT_A_CHOICE" => Some(g_NOT_A_CHOICE()),
            "NOT_A_DAY_OUT_PLACE" => Some(g_NOT_A_DAY_OUT_PLACE()),
            "NOT_A_DESTINATION" => Some(g_NOT_A_DESTINATION()),
            "NOT_A_FORM" => Some(g_NOT_A_FORM()),
            "NOT_A_JOURNEY" => Some(g_NOT_A_JOURNEY()),
            "NOT_A_SHORTLIST" => Some(g_NOT_A_SHORTLIST()),
            "NOT_HOME_COUNTRIES" => Some(g_NOT_HOME_COUNTRIES()),
            "NOT_THE_PAPERWORK" => Some(g_NOT_THE_PAPERWORK()),
            "ONLY_A_SUBJECT" => Some(g_ONLY_A_SUBJECT()),
            "PARKING_RULE" => Some(g_PARKING_RULE()),
            "PART_OF_DAY_POOL" => Some(g_PART_OF_DAY_POOL()),
            "PICKS" => Some(g_PICKS()),
            "PICK_NOT" => Some(g_PICK_NOT()),
            "PICK_POOL" => Some(g_PICK_POOL()),
            "POINTING_WORDS" => Some(g_POINTING_WORDS()),
            "ROUTE_ENDS_POOL" => Some(g_ROUTE_ENDS_POOL()),
            "ROUTE_POOL" => Some(g_ROUTE_POOL()),
            "SEND_A_FORM" => Some(g_SEND_A_FORM()),
            "SERVICES" => Some(g_SERVICES()),
            "SERVICES_POOL" => Some(g_SERVICES_POOL()),
            "SHOPS" => Some(g_SHOPS()),
            "SHOP_ASK" => Some(g_SHOP_ASK()),
            "SHOP_POOL" => Some(g_SHOP_POOL()),
            "SHORTLIST_POOL" => Some(g_SHORTLIST_POOL()),
            "SUBJECT_KINDS" => Some(g_SUBJECT_KINDS()),
            "THINGS_TO_DO_POOL" => Some(g_THINGS_TO_DO_POOL()),
            "TRAVEL" => Some(g_TRAVEL()),
            "WANTS_A_CHECK" => Some(g_WANTS_A_CHECK()),
            "WANTS_A_CHOICE" => Some(g_WANTS_A_CHOICE()),
            "WANTS_A_DAY_OUT" => Some(g_WANTS_A_DAY_OUT()),
            "WANTS_A_PICK" => Some(g_WANTS_A_PICK()),
            "WANTS_A_PLAN" => Some(g_WANTS_A_PLAN()),
            "WANTS_A_SHORTLIST" => Some(g_WANTS_A_SHORTLIST()),
            "WANTS_A_THING_DONE" => Some(g_WANTS_A_THING_DONE()),
            "WANTS_A_TIME" => Some(g_WANTS_A_TIME()),
            "WANTS_THE_RULES" => Some(g_WANTS_THE_RULES()),
            "WEATHER_POOL" => Some(g_WEATHER_POOL()),
            "_A_COMPARISON" => Some(g__A_COMPARISON()),
            "_A_NAMED_COUNTRY" => Some(g__A_NAMED_COUNTRY()),
            "_A_NAME_PART" => Some(g__A_NAME_PART()),
            "_A_QUALIFIER_AFTER_THE_PLACE" => Some(g__A_QUALIFIER_AFTER_THE_PLACE()),
            "_NOT_THE_PLACE" => Some(g__NOT_THE_PLACE()),
            "_SUBJECT_LABEL" => Some(g__SUBJECT_LABEL()),
            "_SUBJECT_MODE" => Some(g__SUBJECT_MODE()),
            "_THE_ENDS" => Some(g__THE_ENDS()),
            "_THE_ENDS_REVERSED" => Some(g__THE_ENDS_REVERSED()),
            "_a_choice" => Some(py_func("_a_choice", |args: &[Py]| _a_choice(args))),
            "_a_country_not_home" => Some(py_func("_a_country_not_home", |args: &[Py]| _a_country_not_home(args))),
            "_a_day_out" => Some(py_func("_a_day_out", |args: &[Py]| _a_day_out(args))),
            "_a_form" => Some(py_func("_a_form", |args: &[Py]| _a_form(args))),
            "_a_named_country_in" => Some(py_func("_a_named_country_in", |args: &[Py]| _a_named_country_in(args))),
            "_a_place_name" => Some(py_func("_a_place_name", |args: &[Py]| _a_place_name(args))),
            "_a_shortlist" => Some(py_func("_a_shortlist", |args: &[Py]| _a_shortlist(args))),
            "_a_time_of_day" => Some(py_func("_a_time_of_day", |args: &[Py]| _a_time_of_day(args))),
            "_about_a_check" => Some(py_func("_about_a_check", |args: &[Py]| _about_a_check(args))),
            "_about_a_journey" => Some(py_func("_about_a_journey", |args: &[Py]| _about_a_journey(args))),
            "_about_a_pick" => Some(py_func("_about_a_pick", |args: &[Py]| _about_a_pick(args))),
            "_about_a_service" => Some(py_func("_about_a_service", |args: &[Py]| _about_a_service(args))),
            "_about_a_shop" => Some(py_func("_about_a_shop", |args: &[Py]| _about_a_shop(args))),
            "_about_the_rules" => Some(py_func("_about_the_rules", |args: &[Py]| _about_the_rules(args))),
            "_about_the_sky" => Some(py_func("_about_the_sky", |args: &[Py]| _about_the_sky(args))),
            "_about_the_subject_named" => Some(py_func("_about_the_subject_named", |args: &[Py]| _about_the_subject_named(args))),
            "_named_service" => Some(py_func("_named_service", |args: &[Py]| _named_service(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_the_answers_before" => Some(py_func("_the_answers_before", |args: &[Py]| _the_answers_before(args))),
            "_the_checks_in" => Some(py_func("_the_checks_in", |args: &[Py]| _the_checks_in(args))),
            "_the_home_town" => Some(py_func("_the_home_town", |args: &[Py]| _the_home_town(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_lines_of_this_question" => Some(py_func("_the_lines_of_this_question", |args: &[Py]| _the_lines_of_this_question(args))),
            "_the_mode_named" => Some(py_func("_the_mode_named", |args: &[Py]| _the_mode_named(args))),
            "_the_move_was_to_their_own_town" => Some(py_func("_the_move_was_to_their_own_town", |args: &[Py]| _the_move_was_to_their_own_town(args))),
            "_the_route_ends" => Some(py_func("_the_route_ends", |args: &[Py]| _the_route_ends(args))),
            "_without_a_qualifier" => Some(py_func("_without_a_qualifier", |args: &[Py]| _without_a_qualifier(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "the_checks_already_named" => Some(py_func("the_checks_already_named", |args: &[Py]| the_checks_already_named(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABROAD" => s_ABROAD(v),
            "ABROAD_POOL" => s_ABROAD_POOL(v),
            "AN_ON_ASK" => s_AN_ON_ASK(v),
            "ASKS_WHAT_TO_DO" => s_ASKS_WHAT_TO_DO(v),
            "A_BARE_DAY_OUT" => s_A_BARE_DAY_OUT(v),
            "A_CHECK_SENTENCE" => s_A_CHECK_SENTENCE(v),
            "A_CONDITION" => s_A_CONDITION(v),
            "A_FEW_WORDS" => s_A_FEW_WORDS(v),
            "A_MEAL_TIME" => s_A_MEAL_TIME(v),
            "A_NOT_A_PLACE" => s_A_NOT_A_PLACE(v),
            "A_PART_OF_DAY" => s_A_PART_OF_DAY(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_PLACE_THERE" => s_A_PLACE_THERE(v),
            "A_QUESTION_OF_ITS_OWN" => s_A_QUESTION_OF_ITS_OWN(v),
            "A_SETTING" => s_A_SETTING(v),
            "A_SKY_ASK" => s_A_SKY_ASK(v),
            "A_THING_IN_HAND" => s_A_THING_IN_HAND(v),
            "A_THING_NOT_A_PLACE" => s_A_THING_NOT_A_PLACE(v),
            "A_TIME_AFTER" => s_A_TIME_AFTER(v),
            "A_TIME_NOT_A_PLACE" => s_A_TIME_NOT_A_PLACE(v),
            "A_TRIP_BEFORE" => s_A_TRIP_BEFORE(v),
            "CHECKS_POOL" => s_CHECKS_POOL(v),
            "CHECK_OF_SOMETHING_ELSE" => s_CHECK_OF_SOMETHING_ELSE(v),
            "CHOICE_POOL" => s_CHOICE_POOL(v),
            "COUNCIL_POOL" => s_COUNCIL_POOL(v),
            "COUNCIL_WORD" => s_COUNCIL_WORD(v),
            "COUNTRIES" => s_COUNTRIES(v),
            "DAY_OUT_NOT" => s_DAY_OUT_NOT(v),
            "FORM_FROM_POOL" => s_FORM_FROM_POOL(v),
            "FORM_TO_POOL" => s_FORM_TO_POOL(v),
            "GET_A_FORM" => s_GET_A_FORM(v),
            "GET_FROM_TO" => s_GET_FROM_TO(v),
            "GOING_AWAY" => s_GOING_AWAY(v),
            "HAS_A_VERB" => s_HAS_A_VERB(v),
            "HOME_COUNTRIES" => s_HOME_COUNTRIES(v),
            "HOME_NATIONS" => s_HOME_NATIONS(v),
            "JOURNEY_POOL" => s_JOURNEY_POOL(v),
            "MEAL_TIME_POOL" => s_MEAL_TIME_POOL(v),
            "MOVE_HOME_NAMES" => s_MOVE_HOME_NAMES(v),
            "MOVING" => s_MOVING(v),
            "MOVING_HOUSE" => s_MOVING_HOUSE(v),
            "NOT_AVAILABILITY" => s_NOT_AVAILABILITY(v),
            "NOT_A_CHECK_SENTENCE" => s_NOT_A_CHECK_SENTENCE(v),
            "NOT_A_CHOICE" => s_NOT_A_CHOICE(v),
            "NOT_A_DAY_OUT_PLACE" => s_NOT_A_DAY_OUT_PLACE(v),
            "NOT_A_DESTINATION" => s_NOT_A_DESTINATION(v),
            "NOT_A_FORM" => s_NOT_A_FORM(v),
            "NOT_A_JOURNEY" => s_NOT_A_JOURNEY(v),
            "NOT_A_SHORTLIST" => s_NOT_A_SHORTLIST(v),
            "NOT_HOME_COUNTRIES" => s_NOT_HOME_COUNTRIES(v),
            "NOT_THE_PAPERWORK" => s_NOT_THE_PAPERWORK(v),
            "ONLY_A_SUBJECT" => s_ONLY_A_SUBJECT(v),
            "PARKING_RULE" => s_PARKING_RULE(v),
            "PART_OF_DAY_POOL" => s_PART_OF_DAY_POOL(v),
            "PICKS" => s_PICKS(v),
            "PICK_NOT" => s_PICK_NOT(v),
            "PICK_POOL" => s_PICK_POOL(v),
            "POINTING_WORDS" => s_POINTING_WORDS(v),
            "ROUTE_ENDS_POOL" => s_ROUTE_ENDS_POOL(v),
            "ROUTE_POOL" => s_ROUTE_POOL(v),
            "SEND_A_FORM" => s_SEND_A_FORM(v),
            "SERVICES" => s_SERVICES(v),
            "SERVICES_POOL" => s_SERVICES_POOL(v),
            "SHOPS" => s_SHOPS(v),
            "SHOP_ASK" => s_SHOP_ASK(v),
            "SHOP_POOL" => s_SHOP_POOL(v),
            "SHORTLIST_POOL" => s_SHORTLIST_POOL(v),
            "SUBJECT_KINDS" => s_SUBJECT_KINDS(v),
            "THINGS_TO_DO_POOL" => s_THINGS_TO_DO_POOL(v),
            "TRAVEL" => s_TRAVEL(v),
            "WANTS_A_CHECK" => s_WANTS_A_CHECK(v),
            "WANTS_A_CHOICE" => s_WANTS_A_CHOICE(v),
            "WANTS_A_DAY_OUT" => s_WANTS_A_DAY_OUT(v),
            "WANTS_A_PICK" => s_WANTS_A_PICK(v),
            "WANTS_A_PLAN" => s_WANTS_A_PLAN(v),
            "WANTS_A_SHORTLIST" => s_WANTS_A_SHORTLIST(v),
            "WANTS_A_THING_DONE" => s_WANTS_A_THING_DONE(v),
            "WANTS_A_TIME" => s_WANTS_A_TIME(v),
            "WANTS_THE_RULES" => s_WANTS_THE_RULES(v),
            "WEATHER_POOL" => s_WEATHER_POOL(v),
            "_A_COMPARISON" => s__A_COMPARISON(v),
            "_A_NAMED_COUNTRY" => s__A_NAMED_COUNTRY(v),
            "_A_NAME_PART" => s__A_NAME_PART(v),
            "_A_QUALIFIER_AFTER_THE_PLACE" => s__A_QUALIFIER_AFTER_THE_PLACE(v),
            "_NOT_THE_PLACE" => s__NOT_THE_PLACE(v),
            "_SUBJECT_LABEL" => s__SUBJECT_LABEL(v),
            "_SUBJECT_MODE" => s__SUBJECT_MODE(v),
            "_THE_ENDS" => s__THE_ENDS(v),
            "_THE_ENDS_REVERSED" => s__THE_ENDS_REVERSED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.checking", module_obj);
    pyrt::xrefs::reg_fn("compose.checking", "_a_named_country_in", py_func("_a_named_country_in", |args: &[Py]| _a_named_country_in(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_time_of_day", py_func("_a_time_of_day", |args: &[Py]| _a_time_of_day(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_route_ends", py_func("_the_route_ends", |args: &[Py]| _the_route_ends(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_without_a_qualifier", py_func("_without_a_qualifier", |args: &[Py]| _without_a_qualifier(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_place_name", py_func("_a_place_name", |args: &[Py]| _a_place_name(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_answers_before", py_func("_the_answers_before", |args: &[Py]| _the_answers_before(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_checks_in", py_func("_the_checks_in", |args: &[Py]| _the_checks_in(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_a_check", py_func("_about_a_check", |args: &[Py]| _about_a_check(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_named_service", py_func("_named_service", |args: &[Py]| _named_service(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_a_service", py_func("_about_a_service", |args: &[Py]| _about_a_service(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_a_shop", py_func("_about_a_shop", |args: &[Py]| _about_a_shop(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_mode_named", py_func("_the_mode_named", |args: &[Py]| _the_mode_named(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_a_journey", py_func("_about_a_journey", |args: &[Py]| _about_a_journey(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_country_not_home", py_func("_a_country_not_home", |args: &[Py]| _a_country_not_home(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_the_subject_named", py_func("_about_the_subject_named", |args: &[Py]| _about_the_subject_named(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_a_pick", py_func("_about_a_pick", |args: &[Py]| _about_a_pick(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_home_town", py_func("_the_home_town", |args: &[Py]| _the_home_town(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_lines_of_this_question", py_func("_the_lines_of_this_question", |args: &[Py]| _the_lines_of_this_question(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_the_move_was_to_their_own_town", py_func("_the_move_was_to_their_own_town", |args: &[Py]| _the_move_was_to_their_own_town(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_day_out", py_func("_a_day_out", |args: &[Py]| _a_day_out(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_the_rules", py_func("_about_the_rules", |args: &[Py]| _about_the_rules(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_form", py_func("_a_form", |args: &[Py]| _a_form(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_choice", py_func("_a_choice", |args: &[Py]| _a_choice(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_a_shortlist", py_func("_a_shortlist", |args: &[Py]| _a_shortlist(args)));
    pyrt::xrefs::reg_fn("compose.checking", "_about_the_sky", py_func("_about_the_sky", |args: &[Py]| _about_the_sky(args)));
    pyrt::xrefs::reg_fn("compose.checking", "the_checks_already_named", py_func("the_checks_already_named", |args: &[Py]| the_checks_already_named(args)));
    pyrt::xrefs::reg_fn("compose.checking", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.checking", "ABROAD", g_ABROAD, s_ABROAD);
    pyrt::xrefs::reg_access("compose.checking", "ABROAD_POOL", g_ABROAD_POOL, s_ABROAD_POOL);
    pyrt::xrefs::reg_access("compose.checking", "AN_ON_ASK", g_AN_ON_ASK, s_AN_ON_ASK);
    pyrt::xrefs::reg_access("compose.checking", "ASKS_WHAT_TO_DO", g_ASKS_WHAT_TO_DO, s_ASKS_WHAT_TO_DO);
    pyrt::xrefs::reg_access("compose.checking", "A_BARE_DAY_OUT", g_A_BARE_DAY_OUT, s_A_BARE_DAY_OUT);
    pyrt::xrefs::reg_access("compose.checking", "A_CHECK_SENTENCE", g_A_CHECK_SENTENCE, s_A_CHECK_SENTENCE);
    pyrt::xrefs::reg_access("compose.checking", "A_CONDITION", g_A_CONDITION, s_A_CONDITION);
    pyrt::xrefs::reg_access("compose.checking", "A_FEW_WORDS", g_A_FEW_WORDS, s_A_FEW_WORDS);
    pyrt::xrefs::reg_access("compose.checking", "A_MEAL_TIME", g_A_MEAL_TIME, s_A_MEAL_TIME);
    pyrt::xrefs::reg_access("compose.checking", "A_NOT_A_PLACE", g_A_NOT_A_PLACE, s_A_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "A_PART_OF_DAY", g_A_PART_OF_DAY, s_A_PART_OF_DAY);
    pyrt::xrefs::reg_access("compose.checking", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "A_PLACE_THERE", g_A_PLACE_THERE, s_A_PLACE_THERE);
    pyrt::xrefs::reg_access("compose.checking", "A_QUESTION_OF_ITS_OWN", g_A_QUESTION_OF_ITS_OWN, s_A_QUESTION_OF_ITS_OWN);
    pyrt::xrefs::reg_access("compose.checking", "A_SETTING", g_A_SETTING, s_A_SETTING);
    pyrt::xrefs::reg_access("compose.checking", "A_SKY_ASK", g_A_SKY_ASK, s_A_SKY_ASK);
    pyrt::xrefs::reg_access("compose.checking", "A_THING_IN_HAND", g_A_THING_IN_HAND, s_A_THING_IN_HAND);
    pyrt::xrefs::reg_access("compose.checking", "A_THING_NOT_A_PLACE", g_A_THING_NOT_A_PLACE, s_A_THING_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "A_TIME_AFTER", g_A_TIME_AFTER, s_A_TIME_AFTER);
    pyrt::xrefs::reg_access("compose.checking", "A_TIME_NOT_A_PLACE", g_A_TIME_NOT_A_PLACE, s_A_TIME_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "A_TRIP_BEFORE", g_A_TRIP_BEFORE, s_A_TRIP_BEFORE);
    pyrt::xrefs::reg_access("compose.checking", "CHECKS_POOL", g_CHECKS_POOL, s_CHECKS_POOL);
    pyrt::xrefs::reg_access("compose.checking", "CHECK_OF_SOMETHING_ELSE", g_CHECK_OF_SOMETHING_ELSE, s_CHECK_OF_SOMETHING_ELSE);
    pyrt::xrefs::reg_access("compose.checking", "CHOICE_POOL", g_CHOICE_POOL, s_CHOICE_POOL);
    pyrt::xrefs::reg_access("compose.checking", "COUNCIL_POOL", g_COUNCIL_POOL, s_COUNCIL_POOL);
    pyrt::xrefs::reg_access("compose.checking", "COUNCIL_WORD", g_COUNCIL_WORD, s_COUNCIL_WORD);
    pyrt::xrefs::reg_access("compose.checking", "COUNTRIES", g_COUNTRIES, s_COUNTRIES);
    pyrt::xrefs::reg_access("compose.checking", "DAY_OUT_NOT", g_DAY_OUT_NOT, s_DAY_OUT_NOT);
    pyrt::xrefs::reg_access("compose.checking", "FORM_FROM_POOL", g_FORM_FROM_POOL, s_FORM_FROM_POOL);
    pyrt::xrefs::reg_access("compose.checking", "FORM_TO_POOL", g_FORM_TO_POOL, s_FORM_TO_POOL);
    pyrt::xrefs::reg_access("compose.checking", "GET_A_FORM", g_GET_A_FORM, s_GET_A_FORM);
    pyrt::xrefs::reg_access("compose.checking", "GET_FROM_TO", g_GET_FROM_TO, s_GET_FROM_TO);
    pyrt::xrefs::reg_access("compose.checking", "GOING_AWAY", g_GOING_AWAY, s_GOING_AWAY);
    pyrt::xrefs::reg_access("compose.checking", "HAS_A_VERB", g_HAS_A_VERB, s_HAS_A_VERB);
    pyrt::xrefs::reg_access("compose.checking", "HOME_COUNTRIES", g_HOME_COUNTRIES, s_HOME_COUNTRIES);
    pyrt::xrefs::reg_access("compose.checking", "HOME_NATIONS", g_HOME_NATIONS, s_HOME_NATIONS);
    pyrt::xrefs::reg_access("compose.checking", "JOURNEY_POOL", g_JOURNEY_POOL, s_JOURNEY_POOL);
    pyrt::xrefs::reg_access("compose.checking", "MEAL_TIME_POOL", g_MEAL_TIME_POOL, s_MEAL_TIME_POOL);
    pyrt::xrefs::reg_access("compose.checking", "MOVE_HOME_NAMES", g_MOVE_HOME_NAMES, s_MOVE_HOME_NAMES);
    pyrt::xrefs::reg_access("compose.checking", "MOVING", g_MOVING, s_MOVING);
    pyrt::xrefs::reg_access("compose.checking", "MOVING_HOUSE", g_MOVING_HOUSE, s_MOVING_HOUSE);
    pyrt::xrefs::reg_access("compose.checking", "NOT_AVAILABILITY", g_NOT_AVAILABILITY, s_NOT_AVAILABILITY);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_CHECK_SENTENCE", g_NOT_A_CHECK_SENTENCE, s_NOT_A_CHECK_SENTENCE);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_CHOICE", g_NOT_A_CHOICE, s_NOT_A_CHOICE);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_DAY_OUT_PLACE", g_NOT_A_DAY_OUT_PLACE, s_NOT_A_DAY_OUT_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_DESTINATION", g_NOT_A_DESTINATION, s_NOT_A_DESTINATION);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_FORM", g_NOT_A_FORM, s_NOT_A_FORM);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_JOURNEY", g_NOT_A_JOURNEY, s_NOT_A_JOURNEY);
    pyrt::xrefs::reg_access("compose.checking", "NOT_A_SHORTLIST", g_NOT_A_SHORTLIST, s_NOT_A_SHORTLIST);
    pyrt::xrefs::reg_access("compose.checking", "NOT_HOME_COUNTRIES", g_NOT_HOME_COUNTRIES, s_NOT_HOME_COUNTRIES);
    pyrt::xrefs::reg_access("compose.checking", "NOT_THE_PAPERWORK", g_NOT_THE_PAPERWORK, s_NOT_THE_PAPERWORK);
    pyrt::xrefs::reg_access("compose.checking", "ONLY_A_SUBJECT", g_ONLY_A_SUBJECT, s_ONLY_A_SUBJECT);
    pyrt::xrefs::reg_access("compose.checking", "PARKING_RULE", g_PARKING_RULE, s_PARKING_RULE);
    pyrt::xrefs::reg_access("compose.checking", "PART_OF_DAY_POOL", g_PART_OF_DAY_POOL, s_PART_OF_DAY_POOL);
    pyrt::xrefs::reg_access("compose.checking", "PICKS", g_PICKS, s_PICKS);
    pyrt::xrefs::reg_access("compose.checking", "PICK_NOT", g_PICK_NOT, s_PICK_NOT);
    pyrt::xrefs::reg_access("compose.checking", "PICK_POOL", g_PICK_POOL, s_PICK_POOL);
    pyrt::xrefs::reg_access("compose.checking", "POINTING_WORDS", g_POINTING_WORDS, s_POINTING_WORDS);
    pyrt::xrefs::reg_access("compose.checking", "ROUTE_ENDS_POOL", g_ROUTE_ENDS_POOL, s_ROUTE_ENDS_POOL);
    pyrt::xrefs::reg_access("compose.checking", "ROUTE_POOL", g_ROUTE_POOL, s_ROUTE_POOL);
    pyrt::xrefs::reg_access("compose.checking", "SEND_A_FORM", g_SEND_A_FORM, s_SEND_A_FORM);
    pyrt::xrefs::reg_access("compose.checking", "SERVICES", g_SERVICES, s_SERVICES);
    pyrt::xrefs::reg_access("compose.checking", "SERVICES_POOL", g_SERVICES_POOL, s_SERVICES_POOL);
    pyrt::xrefs::reg_access("compose.checking", "SHOPS", g_SHOPS, s_SHOPS);
    pyrt::xrefs::reg_access("compose.checking", "SHOP_ASK", g_SHOP_ASK, s_SHOP_ASK);
    pyrt::xrefs::reg_access("compose.checking", "SHOP_POOL", g_SHOP_POOL, s_SHOP_POOL);
    pyrt::xrefs::reg_access("compose.checking", "SHORTLIST_POOL", g_SHORTLIST_POOL, s_SHORTLIST_POOL);
    pyrt::xrefs::reg_access("compose.checking", "SUBJECT_KINDS", g_SUBJECT_KINDS, s_SUBJECT_KINDS);
    pyrt::xrefs::reg_access("compose.checking", "THINGS_TO_DO_POOL", g_THINGS_TO_DO_POOL, s_THINGS_TO_DO_POOL);
    pyrt::xrefs::reg_access("compose.checking", "TRAVEL", g_TRAVEL, s_TRAVEL);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_CHECK", g_WANTS_A_CHECK, s_WANTS_A_CHECK);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_CHOICE", g_WANTS_A_CHOICE, s_WANTS_A_CHOICE);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_DAY_OUT", g_WANTS_A_DAY_OUT, s_WANTS_A_DAY_OUT);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_PICK", g_WANTS_A_PICK, s_WANTS_A_PICK);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_PLAN", g_WANTS_A_PLAN, s_WANTS_A_PLAN);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_SHORTLIST", g_WANTS_A_SHORTLIST, s_WANTS_A_SHORTLIST);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_THING_DONE", g_WANTS_A_THING_DONE, s_WANTS_A_THING_DONE);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_A_TIME", g_WANTS_A_TIME, s_WANTS_A_TIME);
    pyrt::xrefs::reg_access("compose.checking", "WANTS_THE_RULES", g_WANTS_THE_RULES, s_WANTS_THE_RULES);
    pyrt::xrefs::reg_access("compose.checking", "WEATHER_POOL", g_WEATHER_POOL, s_WEATHER_POOL);
    pyrt::xrefs::reg_access("compose.checking", "_A_COMPARISON", g__A_COMPARISON, s__A_COMPARISON);
    pyrt::xrefs::reg_access("compose.checking", "_A_NAMED_COUNTRY", g__A_NAMED_COUNTRY, s__A_NAMED_COUNTRY);
    pyrt::xrefs::reg_access("compose.checking", "_A_NAME_PART", g__A_NAME_PART, s__A_NAME_PART);
    pyrt::xrefs::reg_access("compose.checking", "_A_QUALIFIER_AFTER_THE_PLACE", g__A_QUALIFIER_AFTER_THE_PLACE, s__A_QUALIFIER_AFTER_THE_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "_NOT_THE_PLACE", g__NOT_THE_PLACE, s__NOT_THE_PLACE);
    pyrt::xrefs::reg_access("compose.checking", "_SUBJECT_LABEL", g__SUBJECT_LABEL, s__SUBJECT_LABEL);
    pyrt::xrefs::reg_access("compose.checking", "_SUBJECT_MODE", g__SUBJECT_MODE, s__SUBJECT_MODE);
    pyrt::xrefs::reg_access("compose.checking", "_THE_ENDS", g__THE_ENDS, s__THE_ENDS);
    pyrt::xrefs::reg_access("compose.checking", "_THE_ENDS_REVERSED", g__THE_ENDS_REVERSED, s__THE_ENDS_REVERSED);
}

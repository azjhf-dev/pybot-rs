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


thread_local! { static G_AN_EVENT_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WHEN_NO_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARING_PLACE_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAPPY_COURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_IS_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POSSESSION_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAY_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHEN_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_IN_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_IN_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DURATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_WITHOUT_A_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EAT_WITHOUT_A_TOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERYDAY_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_LONG_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INDOOR_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOURNEY_MODE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_A_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIDS_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIDS_NUMBERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIDS_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAND_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_THING_NEEDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPEN_AIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_OF_SEASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_NO_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_NO_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_NO_PLACE_NO_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING_WITH_KIDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEAK_BOOKED_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEAK_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUIET_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEASON_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEA_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEA_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEWHERE_TO_EAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEWHERE_TO_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY_WITHOUT_A_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY_WITHOUT_A_TOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DRIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FLIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_JOURNEY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LENGTH_NOT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LENGTH_NOT_SAID_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MODE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NIGHTS_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TICKETED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_A_FEW_DAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_A_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_A_WEEKEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_PACKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_PACKING_FOR_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISIT_PACKING_FOR_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKEND_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKEND_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK_NOTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK_NOTES_LAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WINTER_SUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_last_bot: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_plan_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.trips: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_EVENT_NOT_A_PLACE() -> Py { ensure_init(); G_AN_EVENT_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EVENT_NOT_A_PLACE(v: Py) { ensure_init(); G_AN_EVENT_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_VISIT() -> Py { ensure_init(); G_ASK_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_VISIT(v: Py) { ensure_init(); G_ASK_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WHAT() -> Py { ensure_init(); G_ASK_WHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WHAT(v: Py) { ensure_init(); G_ASK_WHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WHEN() -> Py { ensure_init(); G_ASK_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WHEN(v: Py) { ensure_init(); G_ASK_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WHEN_NO_QUESTION() -> Py { ensure_init(); G_ASK_WHEN_NO_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WHEN_NO_QUESTION(v: Py) { ensure_init(); G_ASK_WHEN_NO_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BABY() -> Py { ensure_init(); G_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BABY(v: Py) { ensure_init(); G_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARING_PLACE_HEAD() -> Py { ensure_init(); G_A_CARING_PLACE_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARING_PLACE_HEAD(v: Py) { ensure_init(); G_A_CARING_PLACE_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAPPY_COURT() -> Py { ensure_init(); G_A_HAPPY_COURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAPPY_COURT(v: Py) { ensure_init(); G_A_HAPPY_COURT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEAL() -> Py { ensure_init(); G_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEAL(v: Py) { ensure_init(); G_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_BABY() -> Py { ensure_init(); G_A_NEW_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_BABY(v: Py) { ensure_init(); G_A_NEW_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_IS_NOT_A_PLACE() -> Py { ensure_init(); G_A_PERSON_IS_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IS_NOT_A_PLACE(v: Py) { ensure_init(); G_A_PERSON_IS_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POSSESSION_NOT_A_PLACE() -> Py { ensure_init(); G_A_POSSESSION_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POSSESSION_NOT_A_PLACE(v: Py) { ensure_init(); G_A_POSSESSION_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAY_LENGTH() -> Py { ensure_init(); G_A_STAY_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAY_LENGTH(v: Py) { ensure_init(); G_A_STAY_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHEN_PHRASE() -> Py { ensure_init(); G_A_WHEN_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHEN_PHRASE(v: Py) { ensure_init(); G_A_WHEN_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DAY() -> Py { ensure_init(); G_BARE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DAY(v: Py) { ensure_init(); G_BARE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_WHO() -> Py { ensure_init(); G_BARE_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_WHO(v: Py) { ensure_init(); G_BARE_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED() -> Py { ensure_init(); G_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED(v: Py) { ensure_init(); G_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_DONE() -> Py { ensure_init(); G_BOOKED_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_DONE(v: Py) { ensure_init(); G_BOOKED_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_IN_PLACE() -> Py { ensure_init(); G_BOOKED_IN_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_IN_PLACE(v: Py) { ensure_init(); G_BOOKED_IN_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_IN_WHEN() -> Py { ensure_init(); G_BOOKED_IN_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_IN_WHEN(v: Py) { ensure_init(); G_BOOKED_IN_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_PLACE() -> Py { ensure_init(); G_BOOKED_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_PLACE(v: Py) { ensure_init(); G_BOOKED_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKED_THING() -> Py { ensure_init(); G_BOOKED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKED_THING(v: Py) { ensure_init(); G_BOOKED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY() -> Py { ensure_init(); G_BUSY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY(v: Py) { ensure_init(); G_BUSY.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY_NOTES() -> Py { ensure_init(); G_BUSY_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY_NOTES(v: Py) { ensure_init(); G_BUSY_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_BY() -> Py { ensure_init(); G_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY(v: Py) { ensure_init(); G_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_DAYS() -> Py { ensure_init(); G_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAYS(v: Py) { ensure_init(); G_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_NOTES() -> Py { ensure_init(); G_DAY_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_NOTES(v: Py) { ensure_init(); G_DAY_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_DURATION() -> Py { ensure_init(); G_DURATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DURATION(v: Py) { ensure_init(); G_DURATION.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_ADVICE() -> Py { ensure_init(); G_EAT_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_ADVICE(v: Py) { ensure_init(); G_EAT_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_WITHOUT_A_CARD() -> Py { ensure_init(); G_EAT_WITHOUT_A_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_WITHOUT_A_CARD(v: Py) { ensure_init(); G_EAT_WITHOUT_A_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_EAT_WITHOUT_A_TOWN() -> Py { ensure_init(); G_EAT_WITHOUT_A_TOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EAT_WITHOUT_A_TOWN(v: Py) { ensure_init(); G_EAT_WITHOUT_A_TOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERYDAY_TRIP() -> Py { ensure_init(); G_EVERYDAY_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERYDAY_TRIP(v: Py) { ensure_init(); G_EVERYDAY_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_HOW_LONG() -> Py { ensure_init(); G_FOR_HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_HOW_LONG(v: Py) { ensure_init(); G_FOR_HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING() -> Py { ensure_init(); G_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING(v: Py) { ensure_init(); G_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_BARE() -> Py { ensure_init(); G_GOING_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_BARE(v: Py) { ensure_init(); G_GOING_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_LONG_ASK() -> Py { ensure_init(); G_HOW_LONG_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_LONG_ASK(v: Py) { ensure_init(); G_HOW_LONG_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_INDOOR_NOTES() -> Py { ensure_init(); G_INDOOR_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INDOOR_NOTES(v: Py) { ensure_init(); G_INDOOR_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_PLACE() -> Py { ensure_init(); G_IN_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_PLACE(v: Py) { ensure_init(); G_IN_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOURNEY_MODE() -> Py { ensure_init(); G_JOURNEY_MODE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOURNEY_MODE(v: Py) { ensure_init(); G_JOURNEY_MODE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_A_DETAIL() -> Py { ensure_init(); G_JUST_A_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_A_DETAIL(v: Py) { ensure_init(); G_JUST_A_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_A_MONTH() -> Py { ensure_init(); G_JUST_A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_A_MONTH(v: Py) { ensure_init(); G_JUST_A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_HOW_LONG() -> Py { ensure_init(); G_JUST_HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_HOW_LONG(v: Py) { ensure_init(); G_JUST_HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_KIDS_NOTES() -> Py { ensure_init(); G_KIDS_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIDS_NOTES(v: Py) { ensure_init(); G_KIDS_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_KIDS_NUMBERED() -> Py { ensure_init(); G_KIDS_NUMBERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIDS_NUMBERED(v: Py) { ensure_init(); G_KIDS_NUMBERED.with(|c| *c.borrow_mut() = v); }
pub fn g_KIDS_ONE() -> Py { ensure_init(); G_KIDS_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIDS_ONE(v: Py) { ensure_init(); G_KIDS_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LAND_NOTES() -> Py { ensure_init(); G_LAND_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAND_NOTES(v: Py) { ensure_init(); G_LAND_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_NOTES() -> Py { ensure_init(); G_LONG_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_NOTES(v: Py) { ensure_init(); G_LONG_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_AMOUNT() -> Py { ensure_init(); G_NOT_AN_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_AMOUNT(v: Py) { ensure_init(); G_NOT_AN_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TRIP() -> Py { ensure_init(); G_NOT_A_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TRIP(v: Py) { ensure_init(); G_NOT_A_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PLACES() -> Py { ensure_init(); G_NOT_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PLACES(v: Py) { ensure_init(); G_NOT_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THING_NAMED() -> Py { ensure_init(); G_NO_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THING_NAMED(v: Py) { ensure_init(); G_NO_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORD() -> Py { ensure_init(); G_NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORD(v: Py) { ensure_init(); G_NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_THING_NEEDED() -> Py { ensure_init(); G_ONE_THING_NEEDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_THING_NEEDED(v: Py) { ensure_init(); G_ONE_THING_NEEDED.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_A() -> Py { ensure_init(); G_ON_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A(v: Py) { ensure_init(); G_ON_A.with(|c| *c.borrow_mut() = v); }
pub fn g_OPEN_AIR() -> Py { ensure_init(); G_OPEN_AIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPEN_AIR(v: Py) { ensure_init(); G_OPEN_AIR.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTING() -> Py { ensure_init(); G_OUTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTING(v: Py) { ensure_init(); G_OUTING.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_OF_SEASON() -> Py { ensure_init(); G_OUT_OF_SEASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_OF_SEASON(v: Py) { ensure_init(); G_OUT_OF_SEASON.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING() -> Py { ensure_init(); G_PACKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING(v: Py) { ensure_init(); G_PACKING.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_FOR() -> Py { ensure_init(); G_PACKING_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_FOR(v: Py) { ensure_init(); G_PACKING_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_NOTES() -> Py { ensure_init(); G_PACKING_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_NOTES(v: Py) { ensure_init(); G_PACKING_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_NO_MONTH() -> Py { ensure_init(); G_PACKING_NO_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_NO_MONTH(v: Py) { ensure_init(); G_PACKING_NO_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_NO_PLACE() -> Py { ensure_init(); G_PACKING_NO_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_NO_PLACE(v: Py) { ensure_init(); G_PACKING_NO_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_NO_PLACE_NO_MONTH() -> Py { ensure_init(); G_PACKING_NO_PLACE_NO_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_NO_PLACE_NO_MONTH(v: Py) { ensure_init(); G_PACKING_NO_PLACE_NO_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_SAID() -> Py { ensure_init(); G_PACKING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_SAID(v: Py) { ensure_init(); G_PACKING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING_WITH_KIDS() -> Py { ensure_init(); G_PACKING_WITH_KIDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING_WITH_KIDS(v: Py) { ensure_init(); G_PACKING_WITH_KIDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PEAK() -> Py { ensure_init(); G_PEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEAK(v: Py) { ensure_init(); G_PEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_PEAK_BOOKED_NOTES() -> Py { ensure_init(); G_PEAK_BOOKED_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEAK_BOOKED_NOTES(v: Py) { ensure_init(); G_PEAK_BOOKED_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_PEAK_NOTES() -> Py { ensure_init(); G_PEAK_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEAK_NOTES(v: Py) { ensure_init(); G_PEAK_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_VERBS() -> Py { ensure_init(); G_PLACE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_VERBS(v: Py) { ensure_init(); G_PLACE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_RELATIONS() -> Py { ensure_init(); G_PLURAL_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_RELATIONS(v: Py) { ensure_init(); G_PLURAL_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIET() -> Py { ensure_init(); G_QUIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIET(v: Py) { ensure_init(); G_QUIET.with(|c| *c.borrow_mut() = v); }
pub fn g_QUIET_NOTES() -> Py { ensure_init(); G_QUIET_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUIET_NOTES(v: Py) { ensure_init(); G_QUIET_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_SEA() -> Py { ensure_init(); G_SEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEA(v: Py) { ensure_init(); G_SEA.with(|c| *c.borrow_mut() = v); }
pub fn g_SEASON() -> Py { ensure_init(); G_SEASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEASON(v: Py) { ensure_init(); G_SEASON.with(|c| *c.borrow_mut() = v); }
pub fn g_SEASON_NOTES() -> Py { ensure_init(); G_SEASON_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEASON_NOTES(v: Py) { ensure_init(); G_SEASON_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_SEA_NOTES() -> Py { ensure_init(); G_SEA_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEA_NOTES(v: Py) { ensure_init(); G_SEA_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_SEA_PLACES() -> Py { ensure_init(); G_SEA_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEA_PLACES(v: Py) { ensure_init(); G_SEA_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL() -> Py { ensure_init(); G_SMALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL(v: Py) { ensure_init(); G_SMALL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEWHERE_TO_EAT() -> Py { ensure_init(); G_SOMEWHERE_TO_EAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEWHERE_TO_EAT(v: Py) { ensure_init(); G_SOMEWHERE_TO_EAT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEWHERE_TO_STAY() -> Py { ensure_init(); G_SOMEWHERE_TO_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEWHERE_TO_STAY(v: Py) { ensure_init(); G_SOMEWHERE_TO_STAY.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY_ADVICE() -> Py { ensure_init(); G_STAY_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY_ADVICE(v: Py) { ensure_init(); G_STAY_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY_THING() -> Py { ensure_init(); G_STAY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY_THING(v: Py) { ensure_init(); G_STAY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY_WITHOUT_A_CARD() -> Py { ensure_init(); G_STAY_WITHOUT_A_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY_WITHOUT_A_CARD(v: Py) { ensure_init(); G_STAY_WITHOUT_A_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY_WITHOUT_A_TOWN() -> Py { ensure_init(); G_STAY_WITHOUT_A_TOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY_WITHOUT_A_TOWN(v: Py) { ensure_init(); G_STAY_WITHOUT_A_TOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP_AFTER() -> Py { ensure_init(); G_STOP_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP_AFTER(v: Py) { ensure_init(); G_STOP_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DRIVE() -> Py { ensure_init(); G_THE_DRIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DRIVE(v: Py) { ensure_init(); G_THE_DRIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FLIGHT() -> Py { ensure_init(); G_THE_FLIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FLIGHT(v: Py) { ensure_init(); G_THE_FLIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_JOURNEY_ASK() -> Py { ensure_init(); G_THE_JOURNEY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_JOURNEY_ASK(v: Py) { ensure_init(); G_THE_JOURNEY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LENGTH_ASK() -> Py { ensure_init(); G_THE_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LENGTH_ASK(v: Py) { ensure_init(); G_THE_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LENGTH_NOT_SAID() -> Py { ensure_init(); G_THE_LENGTH_NOT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LENGTH_NOT_SAID(v: Py) { ensure_init(); G_THE_LENGTH_NOT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LENGTH_NOT_SAID_NAMED() -> Py { ensure_init(); G_THE_LENGTH_NOT_SAID_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LENGTH_NOT_SAID_NAMED(v: Py) { ensure_init(); G_THE_LENGTH_NOT_SAID_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MODE() -> Py { ensure_init(); G_THE_MODE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MODE(v: Py) { ensure_init(); G_THE_MODE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NIGHTS_FIRST() -> Py { ensure_init(); G_THE_NIGHTS_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NIGHTS_FIRST(v: Py) { ensure_init(); G_THE_NIGHTS_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_TICKETED() -> Py { ensure_init(); G_TICKETED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TICKETED(v: Py) { ensure_init(); G_TICKETED.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORD() -> Py { ensure_init(); G_TIME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORD(v: Py) { ensure_init(); G_TIME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_PLACE() -> Py { ensure_init(); G_TO_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_PLACE(v: Py) { ensure_init(); G_TO_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_A_FEW_DAYS() -> Py { ensure_init(); G_VISIT_A_FEW_DAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_A_FEW_DAYS(v: Py) { ensure_init(); G_VISIT_A_FEW_DAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_A_WEEK() -> Py { ensure_init(); G_VISIT_A_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_A_WEEK(v: Py) { ensure_init(); G_VISIT_A_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_A_WEEKEND() -> Py { ensure_init(); G_VISIT_A_WEEKEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_A_WEEKEND(v: Py) { ensure_init(); G_VISIT_A_WEEKEND.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_HOW_LONG() -> Py { ensure_init(); G_VISIT_HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_HOW_LONG(v: Py) { ensure_init(); G_VISIT_HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_PACKING() -> Py { ensure_init(); G_VISIT_PACKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_PACKING(v: Py) { ensure_init(); G_VISIT_PACKING.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_PACKING_FOR_A_BABY() -> Py { ensure_init(); G_VISIT_PACKING_FOR_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_PACKING_FOR_A_BABY(v: Py) { ensure_init(); G_VISIT_PACKING_FOR_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_VISIT_PACKING_FOR_A_MEAL() -> Py { ensure_init(); G_VISIT_PACKING_FOR_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISIT_PACKING_FOR_A_MEAL(v: Py) { ensure_init(); G_VISIT_PACKING_FOR_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_WARM() -> Py { ensure_init(); G_WARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARM(v: Py) { ensure_init(); G_WARM.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKEND_AWAY() -> Py { ensure_init(); G_WEEKEND_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKEND_AWAY(v: Py) { ensure_init(); G_WEEKEND_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKEND_NOTES() -> Py { ensure_init(); G_WEEKEND_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKEND_NOTES(v: Py) { ensure_init(); G_WEEKEND_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK_NOTES() -> Py { ensure_init(); G_WEEK_NOTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK_NOTES(v: Py) { ensure_init(); G_WEEK_NOTES.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK_NOTES_LAND() -> Py { ensure_init(); G_WEEK_NOTES_LAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK_NOTES_LAND(v: Py) { ensure_init(); G_WEEK_NOTES_LAND.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORD() -> Py { ensure_init(); G_WHEN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORD(v: Py) { ensure_init(); G_WHEN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WINTER_SUN() -> Py { ensure_init(); G_WINTER_SUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WINTER_SUN(v: Py) { ensure_init(); G_WINTER_SUN.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_WHO() -> Py { ensure_init(); G_WITH_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_WHO(v: Py) { ensure_init(); G_WITH_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT() -> Py { ensure_init(); G__COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT(v: Py) { ensure_init(); G__COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP() -> Py { ensure_init(); G__GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP(v: Py) { ensure_init(); G__GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING_OF_THEIRS() -> Py { ensure_init(); G__NOT_A_THING_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING_OF_THEIRS(v: Py) { ensure_init(); G__NOT_A_THING_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_PEOPLE() -> Py { ensure_init(); G__PLURAL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_PEOPLE(v: Py) { ensure_init(); G__PLURAL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_last_bot() -> Py { ensure_init(); G_last_bot.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_last_bot(v: Py) { ensure_init(); G_last_bot.with(|c| *c.borrow_mut() = v); }
pub fn g_plan_line() -> Py { ensure_init(); G_plan_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_plan_line(v: Py) { ensure_init(); G_plan_line.with(|c| *c.borrow_mut() = v); }
pub fn g_user() -> Py { ensure_init(); G_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_user(v: Py) { ensure_init(); G_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A trip the person is going on, said plainly, is answered about the trip.\n\nThe store files a trip in the words it was said in (\"Cornwall in August\"), so the parts of it are\nthere to answer with - where, when, and who is going. What was missing was the reply to the line\nitself. Main answered\n\n    we're going to cornwall in august   -> \"Noted: you're going to cornwall in august.\"\n    were going to spain in july         -> \"Thanks for telling me - I'm here if you want to dig\n                                            into any of that.\"\n    were off to spain in june           -> \"I'm listening - say more if it helps.\"\n    we booked cornwall for august       -> \"Good - that's one off the list.\"\n\nwhich is the line said back, a stock line, or a line about nothing in particular. A friend says\nsomething about the trip: what the time of year means for booking it, what changes with two kids\nin the car, and often one thing they would want to know themselves.\n\nThe second half is the follow-up. Once a trip is live, the short line that only adds a detail to\nit - \"with two kids\", \"for a week\", \"we're driving\" - is a detail of the trip and is answered\nabout the trip, where main echoed it back (\"Got it - the kids would love it\").\n\nBoth shapes are read here and nowhere else, and every other line is left alone: a question about\nwhere to go, a trip already taken (\"we went to cornwall in august\"), a plan that is not a trip\n(\"we're going to the gym tomorrow\"), and the store's own recall asks all keep their own answers.\n");
    s_last_bot(py_func("last_bot", |args: &[Py]| pyrt::xrefs::xcall("memory", "last_bot", args)));
    s_plan_line(py_func("plan_line", |args: &[Py]| pyrt::xrefs::xcall("memory", "plan_line", args)));
    s_user(py_func("user", |args: &[Py]| pyrt::xrefs::xcall("memory", "user", args)));
    s_DAYS(pyrt::xrefs::xget("memory.facts", "DAYS"));
    s_KIN(pyrt::xrefs::xget("memory.facts", "KIN"));
    s_MONTHS(pyrt::xrefs::xget("memory.facts", "MONTHS"));
    s_NOT_PLACES(pyrt::xrefs::xget("memory.facts", "NOT_PLACES"));
    s_PLACE_VERBS(pyrt::xrefs::xget("memory.facts", "PLACE_VERBS"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.GOING"); }
    s_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:we're|we\\s+are|were|we've|weve|we\\s+have|we|i'm|i\\s+am|im|i've|ive|i\\s+have|i|they're|theyre|they\\s+are|they've|theyve|they|he's|shes|she's|hes)\\b\\s*(?:(?:thinking\\s+of|planning|hoping\\s+to|looking\\s+at|off|away)\\s+)?(?:going|off|heading|flying|driving|sailing|travelling|traveling|booked|booking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.GOING_BARE"); }
    s_GOING_BARE(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|oh\\s+|right\\s+|ok(?:ay)?[ ,]+)*(?:thinking\\s+of\\s+|planning\\s+|hoping\\s+to\\s+|looking\\s+at\\s+)?(?:going|off|heading|flying|driving|sailing|travelling|traveling|booked|book|booking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.TO_PLACE"); }
    s_TO_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:to|into)\\s+((?:the\\s+)?[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.IN_PLACE"); }
    s_IN_PLACE(stdlib_call("re", "compile", &[py_str("\\bin\\s+((?:the\\s+)?[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED_PLACE"); }
    s_BOOKED_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:booked|book|booking)\\s+(?:us\\s+)?((?:the\\s+)?[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,3}?)\\s+(?:for|from)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.NOT_A_TRIP"); }
    s_NOT_A_TRIP(stdlib_call("re", "compile", &[py_str("^\\s*(?:a|an|the|my|our|his|her)\\s+(?:wedding|party|concert|gig|festival|funeral|match|game|show|play|hen|stag|birthday|christening|meeting|conference|interview|appointment|course|class|exam|test)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.NOT_AN_AMOUNT"); }
    s_NOT_AN_AMOUNT(call_builtin("frozenset", &[mcall(&py_str("half halves quarter quarters third thirds two three four five six ten\ntwenty hundred dozen bits pieces lumps chunks squares slices rounds"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.NOT_A_PLACE"); }
    s_NOT_A_PLACE(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("be get got go going do see try have check sort pick take make book\nlose losing lost sell selling sold lose win won keep quit leave leaving start stop manage afford\nask tell call look find put buy eat drink work bed sleep town home hospital doctor dentist\nprison jobcentre funeral crematorium coroner interview solicitor tribunal\ndrive fly stay visit meet leave arrive catch ride walk swim camp pitch collect drop fetch\nplay watch celebrate shop rest\n# A body failing is not a place: \"i'm going to pass out\", \"going to faint\", \"im going to be sick\"\n# are read as a trip to Pass Out and said back as \"{place}, lovely. When is it?\". The verbs are\n# here with the other things nobody travels to.\npass faint collapse vomit throw sick black die choke bleed fit seizure\n# Going round the same ground is not a journey: \"im going round in circles at work\" was read as\n# a trip to a place called Circles and answered \"Circles - nice. When are you going?\", which is the\n# last thing a person who has just said they are stuck wants to hear. The words a loop is said with\n# are nobody's destination.\ncircles circle loops loop\n# \"book recommendations for my dad\" is a person asking for a book to read, not a trip with their\n# father in it: \"book\" is the noun here, so the word behind it was read as the place and the reply\n# was \"Recommendations is booked, then. Get the big days out booked now too\". A word that asks for\n# a pick is never somewhere to go.\nrecommend recommends recommended recommendation recommendations suggest suggests suggested\nsuggestion suggestions ideas idea picks tips\n# A person going to harm themselves is not a trip to a place called Kill Themselves: with no\n# person named in the chat, \"they are going to kill themselves\" was answered \"Kill Themselves,\n# lovely. When is it?\".\nkill kills killing killed suicide overdosing overdose hang hanging harm hurt hurting starve\nmyself yourself himself herself ourselves themselves\nschool work church gym shop shops tesco asda aldi lidl mum mums dad dads nan gran grandparents\nkids children baby partner wife husband friend friends sister brother them him her you me us\nit that this there somewhere anywhere out in on up down back again now later today tomorrow\ntonight yesterday first last next"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_AN_AMOUNT()));
    fn __data1() -> Py { py_tuple(vec![py_str("kids"), py_str("children"), py_str("grandparents"), py_str("grandchildren"), py_str("family"), py_str("relatives"), py_str("folks"), py_str("twins"), py_str("mate"), py_str("in laws"), py_str("in-laws"), py_str("inlaws"), py_str("in law"), py_str("in-law"), py_str("inlaw")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_PERSON_IS_NOT_A_PLACE"); }
    s_A_PERSON_IS_NOT_A_PLACE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:my|our|his|her|their)\\s+(?P<person>(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g_KIN(), &__data1())); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str("))(?P<plural>s)?(?P<poss>['\\u2019]s?)?(?!\\w)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("in"), py_str("on"), py_str("at"), py_str("for"), py_str("with"), py_str("from"), py_str("to"), py_str("by"), py_str("during"), py_str("after"), py_str("before"), py_str("and"), py_str("or"), py_str("but"), py_str("next"), py_str("this"), py_str("last"), py_str("tomorrow"), py_str("today"), py_str("tonight"), py_str("soon"), py_str("later"), py_str("again"), py_str("now"), py_str("then"), py_str("there"), py_str("here"), py_str("every"), py_str("each"), py_str("all"), py_str("week"), py_str("weekend"), py_str("month"), py_str("year")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips._NOT_A_THING_OF_THEIRS"); }
    s__NOT_A_THING_OF_THEIRS(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_HAPPY_COURT"); }
    s_A_HAPPY_COURT(py_str("\\b(?:adopt\\w*|adoption)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.EVERYDAY_TRIP"); }
    s_EVERYDAY_TRIP(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:to the (?:gym|shops?|pub|supermarket|doctors?|dentist|hospital|school|office|bank)|off to work|going to work|to bed|to sleep|to the loo|to the tip|(?!.*"); __s.push_str(&fstr(&g_A_HAPPY_COURT(), -1, "")); __s.push_str(")to (?:the )?court|to (?:the )?prison|to (?:the )?job ?centre|to (?:a |the )?funeral|to (?:the )?crematorium|to (?:the )?coroner'?s?|to (?:a |the )?tribunal|to (?:an |the )?interview)\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ON_A"); }
    s_ON_A(stdlib_call("re", "compile", &[py_str("\\b(?:on|for)\\s+(?:a\\s+|the\\s+|our\\s+)?(holiday|holidays|trip|break|cruise|weekend away)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WITH_WHO"); }
    s_WITH_WHO(stdlib_call("re", "compile", &[py_str("\\bwith\\s+((?:the|my|our|his|her|both|all|a|an|two|three|four|\\d+)\\s+[a-z0-9](?:(?!\\b(?:for|on|at|next|this|during|over|until)\\b|\\bin\\s+(?!law\\b))[a-z0-9' -])*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.NUMBER_WORD"); }
    s_NUMBER_WORD(py_str("(?:\\d+|a\\s+few|a\\s+couple\\s+of|few|several|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.FOR_HOW_LONG"); }
    s_FOR_HOW_LONG(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bfor\\s+("), &g_NUMBER_WORD()), &py_str("\\s+(?:day|days|night|nights|week|weeks|fortnight|month|months)?s?|the weekend)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.JUST_HOW_LONG"); }
    s_JUST_HOW_LONG(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:(?:just|only|about|maybe|probably)\\s+)?("), &g_NUMBER_WORD()), &py_str("\\s+(?:day|days|night|nights|week|weeks|fortnight|month|months))$|^(?:(?:just|only|maybe)\\s+)((?:a|the)\\s+weekend)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.TIME_WORD"); }
    s_TIME_WORD(call_builtin("frozenset", &[mcall(&py_str("morning afternoon evening night today tomorrow yesterday\nminute minutes moment moments hour hours second seconds bit while sec secs min mins"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.DURATION"); }
    s_DURATION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:"), &g_NUMBER_WORD()), &py_str("\\s+)?(?:second|minute|moment|hour|bit|while|day|week|month|fortnight)s?$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING"); }
    s_PACKING(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|also\\s+|oh\\s+)?(?:what|anything)\\s+(?:should|shall|do|would|will|to)?\\s*(?:we|i|you)?\\s*(?:pack|bring|take)\\b(.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_SAID"); }
    s_PACKING_SAID(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|also\\s+|oh\\s+)?(?:should|shall|do|does|would|will|can|could)\\s+(?:we|i|you)\\s+(?:need\\s+to\\s+)?(?:pack|bring|take|need)\\b(.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ONE_THING_NEEDED"); }
    s_ONE_THING_NEEDED(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|also\\s+|oh\\s+)?(?:should|shall|do|does|would|will|can|could)\\s+(?:we|i|you)\\s+need\\s+(?!to\\b)(?:a\\s+|an\\s+|my\\s+|our\\s+|some\\s+|any\\s+)?(?P<thing>[a-z]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.NO_THING_NAMED"); }
    s_NO_THING_NAMED(call_builtin("frozenset", &[mcall(&py_str("anything something nothing much else more anymore any\nanother"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.OUT_OF_SEASON"); }
    s_OUT_OF_SEASON(call_builtin("frozenset", &[py_tuple(vec![py_str("october")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.AN_EVENT_NOT_A_PLACE"); }
    s_AN_EVENT_NOT_A_PLACE(call_builtin("frozenset", &[mcall(&py_str("picnic barbecue bbq party wedding funeral christening festival\nconcert gig show play match game race hen stag birthday dinner lunch barbecue"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_FOR"); }
    s_PACKING_FOR(stdlib_call("re", "compile", &[py_str("^(?:for\\s+(?:the\\s+)?(?:kids?|children|us|me|him|her|them|wife|husband|partner|trip|holiday|week|weekend|walk|beach|evenings?)|for\\s+(?:a|one|two|three|four|five|six|ten|\\d+)\\s+(?:day|days|night|nights|week|weeks|fortnight|months?)|this time of year)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_NOTES"); }
    s_PACKING_NOTES(py_tuple(vec![py_str("for {place} in {month}: layers for the evenings, one warm thing whatever the forecast says, and shoes you can walk in."), py_str("pack light for {place} in {month} - a waterproof, something warm for the evening, and half the clothes you think you need.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_NO_MONTH"); }
    s_PACKING_NO_MONTH(py_tuple(vec![py_str("for {place}: layers, one warm thing for the evening, and shoes you can walk in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_NO_PLACE"); }
    s_PACKING_NO_PLACE(py_tuple(vec![py_str("for {kind} in {month}: layers, one warm thing for the evening, a waterproof and shoes you can walk in - and half the clothes you think you need."), py_str("{kind} in {month} fits in one bag: a change of clothes for each day, something warm for the evening, a waterproof and shoes you can walk in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_NO_PLACE_NO_MONTH"); }
    s_PACKING_NO_PLACE_NO_MONTH(py_tuple(vec![py_str("for {kind}: layers, one warm thing for the evening, a waterproof and shoes you can walk in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PACKING_WITH_KIDS"); }
    s_PACKING_WITH_KIDS(py_str("with the kids, one change of clothes each per day and a bag for the wet things covers it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_PACKING"); }
    s_VISIT_PACKING(py_tuple(vec![py_str("for {place}, nothing much: something for the table - a bottle, a pudding or flowers - and a bag for the night if you are staying over."), py_str("{place} is the easy kind of visit to pack for: a bottle or a pudding for the table, and check whether they want you for the whole day or just the evening."), py_str("for {place}: what you would bring to a table, and a bag only if you are staying the night.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_PACKING_FOR_A_MEAL"); }
    s_VISIT_PACKING_FOR_A_MEAL(py_tuple(vec![py_str("take a bottle and something for after - a pudding, cheese and biscuits, or a box of chocolates - so you are not competing for their oven. Ask what they are cooking, then bring the thing that goes with it."), py_str("a bottle for the table and something for after, and ask what they are cooking so what you bring goes with it rather than fighting for the oven.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_PACKING_FOR_A_BABY"); }
    s_VISIT_PACKING_FOR_A_BABY(py_tuple(vec![py_str("nothing for the baby - they have more than they can use. Take food: a meal for the freezer or something for lunch, and keep the visit to an hour unless they ask you to stay."), py_str("what a house with a new baby wants is a cooked meal and short visits, so take dinner in a dish you do not want back, and ask before you hold the baby.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_STAY_LENGTH"); }
    s_A_STAY_LENGTH(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|many (?:nights|days)) (?:should|shall|do|would|will) (?:we|i) (?:stay|stop|be there|go for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_HOW_LONG"); }
    s_VISIT_HOW_LONG(py_tuple(vec![py_str("two nights is the sweet spot for family - long enough to settle in, short enough that you are missed."), py_str("one night the first time, two if it is a long drive. Leave while everyone is still enjoying it rather than when the conversation has run out."), py_str("a weekend is plenty unless there is something on: arrive Saturday, go home Monday, and say when you are leaving before you get there.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_A_WEEK"); }
    s_VISIT_A_WEEK(py_tuple(vec![py_str("a week with family is a long visit: agree the leaving day before you go, and keep an afternoon or two that are just yours."), py_str("a week is plenty to be in somebody else's house - say when you are leaving, and plan one day of it that is just yours.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_A_FEW_DAYS"); }
    s_VISIT_A_FEW_DAYS(py_tuple(vec![py_str("a night or two is the easy length for a visit: enough to catch up, short enough that nobody is hosting for too long."), py_str("one or two nights is right for family: bring something for the table, and leave while you are both still enjoying it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.VISIT_A_WEEKEND"); }
    s_VISIT_A_WEEKEND(py_tuple(vec![py_str("a weekend is the classic visit - one evening together, one day out, and home before the week starts."), py_str("the weekend is enough for most visits: Friday night to Sunday afternoon, and say on the Friday when you are heading back.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_MEAL"); }
    s_A_MEAL(stdlib_call("re", "compile", &[py_str("\\b(?:dinner|lunch|tea|supper|brunch|barbecue|bbq|roast)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_NEW_BABY"); }
    s_A_NEW_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:new\\s?born|new baby|just had a (?:baby|girl|boy)|had a baby|baby (?:is|was) (?:born|due)|\\dbaby)\\b|\\bnewborn\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_FLIGHT"); }
    s_THE_FLIGHT(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|far) (?:is|does) the flight\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.HOW_LONG_ASK"); }
    s_HOW_LONG_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|far)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_MODE"); }
    s_THE_MODE(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|are) we get(?:ting)? there\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_DRIVE"); }
    s_THE_DRIVE(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|far) (?:is|does) the (?:drive|journey|trip)\\b|\\bhow (?:long|far) (?:will|would) the (?:drive|journey|trip) (?:take|be)(?:\\b|$)|\\bhow do we get there\\b|\\bhow are we getting there\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_JOURNEY_ASK"); }
    s_THE_JOURNEY_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:long|far)\\s+(?:will|would|does|do|did|is|are|might)\\s+(?:it|that|this|the\\s+(?:drive|journey|trip|flight|train))?\\s*(?:take|be|takes)?\\s*(?:to\\s+)?(?P<mode>drive|get\\s+there|get\\s+to|travel|walk|sail|go)\\b|\\bhow\\s+(?:long|far)\\s+(?:it|is\\s+it)\\b[^?]{0,20}\\b(?P<mode2>driving|drive|flight|train)\\b|\\bhow\\s+long\\s+to\\s+(?P<mode3>get\\s+there|drive|fly|travel)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_LENGTH_ASK"); }
    s_THE_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:are|is|were|was)\\s+(?:we|you|i|it)\\s+(?:there|away|staying|going|going\\s+for|staying\\s+for|away\\s+for)\\b|\\bhow\\s+long\\s+(?:are|is|were)\\s+(?:the|our|my|your)\\s+(?:trip|holiday|break|stay)\\b|\\bhow\\s+many\\s+(?:nights|days)\\s+(?:are\\s+we|is\\s+it|will\\s+we|do\\s+we|have\\s+we|should\\s+we|would\\s+we)\\b|\\bhow\\s+many\\s+(?:nights|days)\\s*[?.]?\\s*$|\\bhow\\s+long\\s+for\\s*[?.]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_LENGTH_NOT_SAID"); }
    s_THE_LENGTH_NOT_SAID(py_tuple(vec![py_str("You haven't said how long - how many nights are you thinking?"), py_str("Nothing said about how long yet - how many nights are you thinking?"), py_str("How many nights are you thinking? The plan hasn't got the length on it yet.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_LENGTH_NOT_SAID_NAMED"); }
    s_THE_LENGTH_NOT_SAID_NAMED(py_tuple(vec![py_str("You haven't said how long - how many nights are you thinking for {named}?"), py_str("How many nights are you thinking for {named}? Nothing said about that yet.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.THE_NIGHTS_FIRST"); }
    s_THE_NIGHTS_FIRST(py_tuple(vec![py_str("The nights decide what you can book, so it's worth settling that first."), py_str("Once the nights are fixed the place to stay and the packing both follow."), py_str("Get the nights settled and the rest of it falls into place.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.JOURNEY_MODE"); }
    s_JOURNEY_MODE(stdlib_call("re", "compile", &[py_str("\\bdriv\\w*|\\bby\\s+car\\b|\\bin\\s+the\\s+car\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BY"); }
    s_BY(stdlib_call("re", "compile", &[py_str("\\b(?:we're|we are|were|i'm|im|i am)\\s+(driving|flying|getting the train|on the train|taking the ferry|sailing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PEAK"); }
    s_PEAK(py_tuple(vec![py_str("july"), py_str("august")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BUSY"); }
    s_BUSY(py_tuple(vec![py_str("april"), py_str("may"), py_str("june"), py_str("september"), py_str("october")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.QUIET"); }
    s_QUIET(py_tuple(vec![py_str("november"), py_str("december"), py_str("january"), py_str("february"), py_str("march")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PEAK_NOTES"); }
    s_PEAK_NOTES(py_tuple(vec![py_str("{month} is the busiest, priciest few weeks of the year, so book the place to stay now."), py_str("{month} takes the school holidays, so the prices only go one way - book soon."), py_str("that is the height of the season: book the place to stay now, and expect slow roads on the way down.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PEAK_BOOKED_NOTES"); }
    s_PEAK_BOOKED_NOTES(py_tuple(vec![py_str("at the height of the season the popular places fill up, so book the big days out now too."), py_str("that is the busiest few weeks of the year, so get the days out booked now as well.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BUSY_NOTES"); }
    s_BUSY_NOTES(py_tuple(vec![py_str("it misses the worst of it: cheaper and calmer than August, and the weather is usually decent."), py_str("you have timed it well - {month} is quieter and cheaper than the school holidays.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.QUIET_NOTES"); }
    s_QUIET_NOTES(py_tuple(vec![py_str("that is out of season, so it will be quiet and cheap, and dark by half four."), py_str("out of season: quiet, cheap, and short days.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SEASON_NOTES"); }
    s_SEASON_NOTES(py_tuple(vec![py_str("that is the season for it, so book soon - the weeks people want go first."), py_str("{month} is the good time there: warm days, cool evenings, and the prices know it."), py_str("{month} is when it is at its best, so the flights and the rooms are the first thing to sort.")]));
    fn __data3() -> Py { py_tuple(vec![py_str("canaries"), py_str("lanzarote"), py_str("tenerife"), py_str("gran canaria"), py_str("fuerteventura"), py_str("la palma"), py_str("cape verde"), py_str("madeira"), py_str("egypt"), py_str("red sea"), py_str("dubai"), py_str("abu dhabi"), py_str("marrakesh"), py_str("morocco"), py_str("cape town"), py_str("gambia"), py_str("thailand"), py_str("maldives"), py_str("sri lanka"), py_str("goa"), py_str("barbados"), py_str("jamaica"), py_str("caribbean"), py_str("cancun"), py_str("mexico"), py_str("florida"), py_str("orlando")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WINTER_SUN"); }
    s_WINTER_SUN(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.KIDS_NOTES"); }
    s_KIDS_NOTES(py_tuple(vec![py_str("with kids in tow, plan one thing a day and keep a wet-day backup up your sleeve."), py_str("with the kids, the beach early and something under cover for the afternoon works best."), py_str("kids on a trip like that want one big thing a day and somewhere to dry out afterwards.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.KIDS_NUMBERED"); }
    s_KIDS_NUMBERED(py_tuple(vec![py_str("{who}: one big thing a day is plenty, and line up something indoors for the wet afternoon."), py_str("with {who}, plan one thing a day and keep a wet-day backup up your sleeve.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.KIDS_ONE"); }
    s_KIDS_ONE(py_str("with kids in tow, plan one thing a day and keep a wet-day backup up your sleeve."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WEEK_NOTES"); }
    s_WEEK_NOTES(py_tuple(vec![py_str("a week gives you time for the beach days and a couple of trips out without rushing."), py_str("a week is about right: long enough to settle, short enough to want to go back.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WEEK_NOTES_LAND"); }
    s_WEEK_NOTES_LAND(py_tuple(vec![py_str("a week gives you room for two or three places rather than one, without a day lost to moving each time."), py_str("a week is about right there: enough for the big one and a quiet day either side.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SEASON"); }
    s_SEASON(call_builtin("frozenset", &[mcall(&py_str("spring springtime summer summertime autumn autumntime winter wintertime"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SEA_PLACES"); }
    s_SEA_PLACES(call_builtin("frozenset", &[mcall(&py_str("cornwall devon dorset sussex kent norfolk suffolk wales pembrokeshire\nanglesey skye iona jersey guernsey crete cyprus malta sicily sardinia mallorca majorca ibiza\ntenerife lanzarote fuerteventura gran canaria madeira algarve portugal spain greece turkey\ncroatia italy france"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.DAY_NOTES"); }
    s_DAY_NOTES(py_tuple(vec![py_str("that many days is room for a couple of big things and a slow morning in between."), py_str("with that many days, pick the two things that matter and let the rest happen.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WEEKEND_NOTES"); }
    s_WEEKEND_NOTES(py_tuple(vec![py_str("a weekend away gives you two nights, so pick one thing to do properly rather than three."), py_str("for a weekend, one big thing a day and a slow breakfast is the whole trick.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.LONG_NOTES"); }
    s_LONG_NOTES(py_tuple(vec![py_str("a trip that long means you can stop rushing: a couple of lazy days and one proper day out each."), py_str("with that many days you can plan the big day out for the middle, when everyone has settled.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ASK_WHEN"); }
    s_ASK_WHEN(py_tuple(vec![py_str("{place} - nice. When are you going?"), py_str("{place}, lovely. When is it?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ASK_WHEN_NO_QUESTION"); }
    s_ASK_WHEN_NO_QUESTION(py_tuple(vec![py_str("{place}, nice - tell me when and I can help you work out what to do with it."), py_str("{place}, lovely. Say when it is and I will help you plan the days.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.OUTING"); }
    s_OUTING(stdlib_call("re", "compile", &[py_str("^the (beach|coast|seaside|park|woods|forest|hills|countryside|lido|zoo|museum|aquarium)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SEA_NOTES"); }
    s_SEA_NOTES(py_tuple(vec![py_str("take more food and water than you think, and check the tide before you set off."), py_str("a change of clothes and a towel in the boot covers most of what goes wrong by the sea."), py_str("go early for the parking, and take a bag for the wet things on the way back.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.LAND_NOTES"); }
    s_LAND_NOTES(py_tuple(vec![py_str("take more food and water than you think, and go early if you are driving there."), py_str("sturdy shoes and a bag for the wet things covers most of what goes wrong."), py_str("check the opening times before you set off, and take a coat even on a warm morning.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.INDOOR_NOTES"); }
    s_INDOOR_NOTES(py_tuple(vec![py_str("check the opening times and book the timed slot if they have one - it saves the queue."), py_str("buy the tickets online the night before, and go at opening rather than the middle of the day.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SEA"); }
    s_SEA(py_tuple(vec![py_str("beach"), py_str("coast"), py_str("seaside"), py_str("lido")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.TICKETED"); }
    s_TICKETED(py_tuple(vec![py_str("zoo"), py_str("museum"), py_str("aquarium")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.OPEN_AIR"); }
    s_OPEN_AIR(py_tuple(vec![py_str("park"), py_str("woods"), py_str("forest"), py_str("hills"), py_str("countryside")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ASK_WHAT"); }
    s_ASK_WHAT(py_tuple(vec![py_str("what do you want to sort out first, where to stay or what to do while you are there?"), py_str("what are you hoping to do while you are there?"), py_str("have you sorted where you are staying, or is that still open?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.ASK_VISIT"); }
    s_ASK_VISIT(py_tuple(vec![py_str("what are you doing while you're there?"), py_str("what's the plan for the visit?"), py_str("is it a flying visit or are you staying a while?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.JUST_A_DETAIL"); }
    s_JUST_A_DETAIL(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|oh\\s+|also\\s+|plus\\s+)?(?:with\\b|for\\s+(?:a|one|two|three|four|\\d|the\\s+(?:week|weekend|month|fortnight))\\b|(?:a|one|two|three|four|five|six|seven|eight|nine|ten|\\d+)\\s+(?:day|days|night|nights|week|weeks|fortnight|months?)\\b|(?:we're|were|we are|i'm|im|i am)\\s+(?:driving|flying|getting the train|getting a\\s+(?:train|flight|ferry)|taking the (?:train|ferry|plane)|sailing|going by\\s+(?:car|train|ferry|plane)|on the (?:train|ferry))\\b|by\\s+(?:car|train|ferry|plane)\\b|(?:a|one|two|three|four|five|six|seven|eight|nine|ten|\\d+)\\s+(?:kids?|children|adults)\\b|what\\s+(?:should|shall|do|would)\\s+(?:we|i|you)\\s+(?:pack|bring|take)\\b|what\\s+to\\s+(?:pack|bring|take)\\b|how\\s+(?:long|far)\\s+(?:is|does|will|would)\\s+the\\s+(?:drive|journey|trip|flight)\\b|how\\s+(?:do|are)\\s+we\\s+get(?:ting)?\\s+there\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WARM"); }
    s_WARM(py_tuple(vec![py_str("lovely"), py_str("nice"), py_str("good one")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WORD"); }
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9'\\u2019]+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED"); }
    s_BOOKED(stdlib_call("re", "compile", &[py_str("\\bbook\\w*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED_DONE"); }
    s_BOOKED_DONE(stdlib_call("re", "compile", &[py_str("\\bbooked\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED_IN_PLACE"); }
    s_BOOKED_IN_PLACE(stdlib_call("re", "compile", &[py_str("\\bbook\\w*\\b[^.!?]{0,40}?\\bin\\s+((?:the\\s+)?[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WHEN_WORD"); }
    s_WHEN_WORD(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&op_bitor(&g_MONTHS(), &g_DAYS()), &g_SEASON())); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED_IN_WHEN"); }
    s_BOOKED_IN_WHEN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:booked|book|booking)\\s+(?:us\\s+)?((?:the\\s+)?[a-z][a-z'\\u2019-]*(?:\\s+[a-z][a-z'\\u2019-]*){0,3}?)\\s+in\\s+(?:"), &g_WHEN_WORD()), &py_str("|next|this|a\\s+(?:few|couple)|\\d)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.PLURAL_RELATIONS"); }
    s_PLURAL_RELATIONS(call_builtin("frozenset", &[mcall(&py_str("grandparents parents kids children siblings relatives inlaws\n    grandchildren"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_POSSESSION_NOT_A_PLACE"); }
    s_A_POSSESSION_NOT_A_PLACE(stdlib_call("re", "compile", &[py_str("^(?:my|our|his|her|their|the)\\s+([a-z]+(?:\\s+in\\s+law)?)(?:'s|\\u2019s|s)?$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_CARING_PLACE_HEAD"); }
    s_A_CARING_PLACE_HEAD(stdlib_call("re", "compile", &[py_str("^(?:the\\s+|a\\s+|an\\s+)?(?:hospital|hospice|care home|nursing home|residential home|clinic|infirmary|surgery|ward|a&e|ae|icu|intensive care|high dependency|coronary care)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BOOKED_THING"); }
    s_BOOKED_THING(call_builtin("frozenset", &[mcall(&py_str("table tables ticket tickets seat seats row taxi cab minicab\nhaircut appointment appointments slot court pitch room rooms booth class classes lesson\nlessons dinner lunch breakfast delivery takeaway table tennis\nphysio physiotherapist dentist dental doctor doctors gp optician hygienist chiropractor\nosteopath vet vets clinic scan xray x-ray bloods surgery consultation checkup\nmot service servicing boiler plumber electrician garage engineer gas"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.STAY_THING"); }
    s_STAY_THING(call_builtin("frozenset", &[mcall(&py_str("caravan cottage villa apartment flat hotel lodge cabin tent campervan\ncamper motorhome house boat ferry crossing tunnel shuttle flight flights cruise\napartment"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.STOP_AFTER"); }
    s_STOP_AFTER(call_builtin("frozenset", &[mcall(&py_str("on in at for from with and next this that the a an to by until\ntill near around over through tomorrow tonight today later soon\nagain anyway then first"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_WHEN_PHRASE"); }
    s_A_WHEN_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:next|this|at|over|for|in the|after)\\s+(?:month|week|weekend|summer|winter|spring|autumn|christmas|xmas|easter|half term|new year|bank holiday)\\b|\\b(?:tomorrow|tonight|the day after tomorrow|this afternoon|this morning)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BARE_DAY"); }
    s_BARE_DAY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&g_DAYS(), &py_set(vec![py_str("weekend")]))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips._GROUP"); }
    s__GROUP(py_str("(?:kids?|children|child|adults?|grown-?ups?|teenagers?|bab(?:y|ies)|toddlers?|boys?|girls?|year\\s+olds?|pensioners?|grandparents?|parents?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips._COUNT"); }
    s__COUNT(py_str("(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|\\d+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.BARE_WHO"); }
    s_BARE_WHO(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^\\s*(?:and\\s+|oh\\s+|also\\s+|plus\\s+)?((?:"), &g__COUNT()), &py_str("\\s+")), &g__GROUP()), &py_str("|\\d+\\s+year\\s+olds?)(?:\\s+(?:and|plus)\\s+(?:an?\\s+|our\\s+|my\\s+|their\\s+|the\\s+)?(?:")), &g__COUNT()), &py_str("\\s+)?")), &g__GROUP()), &py_str(")*)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SMALL"); }
    s_SMALL(call_builtin("frozenset", &[mcall(&py_str("of the and on in de du van der le la"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SOMEWHERE_TO_EAT"); }
    s_SOMEWHERE_TO_EAT(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|also|plus|then|oh|ok|okay)\\s+)?(?:somewhere|a place|anywhere)\\s+(?:good\\s+|nice\\s+|decent\\s+)?to\\s+(?:eat|have lunch|have dinner|get food|get something to eat|grab a bite)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.SOMEWHERE_TO_STAY"); }
    s_SOMEWHERE_TO_STAY(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|also|plus|then|oh|ok|okay)\\s+)?(?:somewhere|a place|anywhere)\\s+(?:good\\s+|nice\\s+|cheap\\s+)?to\\s+(?:stay|sleep|stop over|kip)\\b|^\\s*(?:(?:and|also|plus|then)\\s+)?(?:a|any)\\s+(?:good\\s+|decent\\s+|cheap\\s+)?(?:hotel|b&b|guesthouse|bed for the night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.EAT_WITHOUT_A_CARD"); }
    s_EAT_WITHOUT_A_CARD(py_tuple(vec![py_str("I cannot name a place in {place} without guessing, so narrow it yourself: set a budget a head and how far you will walk, then pick the shortest menu - a short one usually means the kitchen is cooking it fresh."), py_str("{place} is not a town I can name places in, so go by the street: the shortest menu, and the place with people in it at six rather than the one with photos of the food outside.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.STAY_WITHOUT_A_CARD"); }
    s_STAY_WITHOUT_A_CARD(py_tuple(vec![py_str("I cannot name a hotel in {place} without guessing, so go by the area rather than the stars: near the station if it is one night, near where you will spend the evening if it is two, and reviews from this year rather than the rating."), py_str("{place} is not a town I can name hotels in. Pick the area first - the station for one night, wherever you are eating for two - then check the recent reviews for noise.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.EAT_WITHOUT_A_TOWN"); }
    s_EAT_WITHOUT_A_TOWN(py_tuple(vec![py_str("Where are you off to? With a town I can point you at an end of it. Without one: set a budget a head and how far you will walk, then pick the shortest menu - a short one usually means the kitchen is cooking it fresh."), py_str("Tell me the town and I will narrow it down. Failing that: a short menu, and the place that is busy at six rather than the one with photos of the food outside.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.EAT_ADVICE"); }
    s_EAT_ADVICE(py_tuple(vec![py_str("Set a budget per head and how far you will walk, then pick the shortest menu - a short one usually means the kitchen is cooking it fresh, and the place with people in it at six beats the one with photos of the food outside.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.STAY_ADVICE"); }
    s_STAY_ADVICE(py_tuple(vec![py_str("Go by the area rather than the stars: near the station if it is one night, near where you will spend the evening if it is two, and read reviews from this year rather than the rating.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.STAY_WITHOUT_A_TOWN"); }
    s_STAY_WITHOUT_A_TOWN(py_tuple(vec![py_str("Where will you be? With a town I can say which area to look in. Without one: near the station if it is one night, near where you will eat if it is two, and reviews from this year rather than the star rating."), py_str("Tell me where and I will point at an end of it. Failing that: go by the area rather than the hotel - the station for one night, the evening's end of town for two.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.WEEKEND_AWAY"); }
    s_WEEKEND_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:weekend away|weekend break|long weekend|mini ?break|short break|few days away|going away|get away|(?:away|off) for (?:a|two|three|four|five|\\d+) (?:week|weeks|fortnight|days?|months?)|for (?:a fortnight|two weeks))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.A_BABY"); }
    s_A_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:bab(?:y|ies)|newborns?|toddlers?|infants?|napp(?:y|ies)|prams?|pushchair|changing bag|baby bag|nurser(?:y|ies)|breastfeed\\w*|formula|due date|expecting|pregnan\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips.JUST_A_MONTH"); }
    s_JUST_A_MONTH(stdlib_call("re", "compile", &[py_str("^\\s*(?:in\\s+|next\\s+|this\\s+|maybe\\s+|probably\\s+|its\\s+|it's\\s+|er,?\\s+)*([a-z]+)\\s*[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.trips._PLURAL_PEOPLE"); }
    s__PLURAL_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("parents"), py_str("grandparents"), py_str("children"), py_str("kids"), py_str("family"), py_str("relatives"), py_str("folks"), py_str("friends"), py_str("twins"), py_str("in laws"), py_str("in-laws"), py_str("inlaws")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _one_thing_they_need(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing a \"will i need ...\" asks about, or \"\" when it names nothing in particular.");
    found = mcall(&g_ONE_THING_NEEDED(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_NO_THING_NAMED(), &thing.clone()))) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _start(args: &[Py]) -> Py {
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
    let _ = py_str("A reply that begins with a place whose \"the\" was lowered mid-sentence still opens a line.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&said.clone()) { op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { said.clone() };
    Py::None
}
pub fn _low(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _a_relation_not_a_place(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the place the line names is somebody of theirs, not somewhere.");
    found = mcall(&g_A_POSSESSION_NOT_A_PLACE(), "match", &[mcall(&place.clone(), "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLURAL_RELATIONS(), &word.clone())) } } } };
    Py::None
}
pub fn _a_person_visited(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The person a line is going to see, with the words that belong to them, or \"\".\n\n    \"im off to my brothers tonight\" and \"we are going to my in laws\" name a person, and a person\n    is not a destination - but the visit is still something to answer, and the shrug (\"Worth\n    doing. What would you like help with first?\") is not it. The words after \"to\" are kept as the\n    trip's place so `_as_a_visit` can say them back as theirs, and a visit said as the purpose of\n    the journey (\"visiting my parents this weekend\", \"im going to see my sister on sunday\") names\n    no place at all, so it is read from the line itself.\n    ");
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.not_a_trip", "IN_A_CARING_PLACE"), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return py_str("");
    }
    who = pyrt::xrefs::xcall("memory.not_a_trip", "the_visit", &[line.clone()]);
    if py_truthy(&who.clone()) {
    return who.clone();
    }
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __kws.extend(vec![("a_visit".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _places(args))(&__args) };
    return if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { _a_relation_not_a_place(&[phrase.clone()]) } }) { phrase.clone() } else { py_str("") };
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut a_visit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_visit") { v.clone() }
    else { Py::Bool(false) };
    let mut bare: Py = Py::None;
    let mut bare_place: Py = Py::None;
    let mut booked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut place: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The place a line says it is going to, in the person's own casing, or \"\".\n\n    With a_visit the relation fence below stands aside: a person is not a place, and the visit\n    answer needs the words to say back (\"my brothers\"), not the trip card that names a town.\n    ");
    booked = mcall(&g_BOOKED(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = booked.clone(); if !py_truthy(&__b) { __b } else { _the_thing_booked(&[_low(&[slice_or_raise(&call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), &mcall(&booked.clone(), "end", &[]), &Py::None, &Py::None)])]) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_TO_PLACE(), g_BOOKED_IN_PLACE(), g_BOOKED_IN_WHEN(), g_BOOKED_PLACE(), g_IN_PLACE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    kept = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(",.;:!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &bare.clone())) } }) {
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_PLACE(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DAYS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SEASON(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STOP_AFTER(), &bare.clone())) } } } } } } } } } }) {
    break;
    }
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DAYS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SEASON(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_STOP_AFTER(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("with"), py_str("and"), py_str("for"), py_str("next"), py_str("this")]), &bare.clone())) } } } } } } } } } }) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    place = mcall(&mcall(&py_str(" "), "join", &[kept.clone()]), "strip", &[py_str(" ,.;:!?")]);
    low = mcall(&place.clone(), "lower", &[]);
    head = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&low.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TIME_WORD(), &geti(&head.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { mcall(&g_DURATION(), "match", &[low.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_NOT_A_TRIP(), "match", &[low.clone()])) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.not_a_trip", "not_a_destination", &[words.clone(), place.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_PLACE(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DAYS(), &low.clone())); if py_truthy(&__b) { __b } else { op_bitand(&g_SEASON(), &call_builtin("set", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } } } } } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_CARING_PLACE_HEAD(), "match", &[low.clone()])) {
    continue;
    }
    bare_place = stdlib_call("re", "sub", &[py_str("^(?:the|a|an)\\s+"), py_str(""), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_PLACE(), &bare_place.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &bare_place.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DAYS(), &bare_place.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_visit.clone())); if !py_truthy(&__b) { __b } else { _a_relation_not_a_place(&[low.clone()]) } }) {
    continue;
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &low.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bbook"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = _the_thing_booked(&[low.clone()]); if py_truthy(&__b) { __b } else { _a_thing_booked_for_when(&[low.clone()]) } } } }) {
    continue;
    }
    return place.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_thing_booked(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the words name the thing that was booked rather than somewhere to go.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&low.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BOOKED_THING(), &geti(&words.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _a_thing_booked_for_when(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the words name the thing booked, where a date and not a place follows it.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&low.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STAY_THING(), &geti(&words.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _month(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[_low(&[line.clone()])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_MONTHS(), &word.clone()))) {
    return word.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _when_phrase(args: &[Py]) -> Py {
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
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The date the line gives without naming a month (\"next month\"), or \"\".");
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    found = mcall(&g_A_WHEN_PHRASE(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), "split", &[])]);
    }
    day = mcall(&g_BARE_DAY(), "search", &[said.clone()]);
    return if py_truthy(&day.clone()) { mcall(&mcall(&day.clone(), "group", &[Py::Int(1)]), "lower", &[]) } else { py_str("") };
    Py::None
}
pub fn _who(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    found = mcall(&g_WITH_WHO(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,.;:!?")]);
    }
    bare = mcall(&g_BARE_WHO(), "match", &[line.clone()]);
    return if py_truthy(&bare.clone()) { mcall(&mcall(&bare.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,.;:!?")]) } else { py_str("") };
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    found = mcall(&g_FOR_HOW_LONG(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,.;:!?")]);
    }
    bare = mcall(&g_JUST_HOW_LONG(), "match", &[mcall(&line.clone(), "strip", &[py_str(" .,;:!?")])]);
    if py_truthy(&bare.clone()) {
    return mcall(&{ let __b = mcall(&bare.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { mcall(&bare.clone(), "group", &[Py::Int(2)]) } }, "strip", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _how_getting_there(args: &[Py]) -> Py {
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
    found = mcall(&g_BY(), "search", &[line.clone()]);
    return if py_truthy(&found.clone()) { mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]) } else { py_str("") };
    Py::None
}
pub fn _by_the_sea(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Whether a week there is a week of beach days: a coast, or a place famous for one.\n\n    A place nobody named on the list is given the general week note, which says nothing about\n    beaches or hills, so a guess about Scotland cannot promise sand.\n    ");
    words = call_builtin("set", &[mcall(&mcall(&_low(&[place.clone()]), "replace", &[py_str("-"), py_str(" ")]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[op_bitand(&words.clone(), &op_bitor(&g_SEA_PLACES(), &py_set(vec![py_str("beach"), py_str("beaches"), py_str("coast"), py_str("seaside"), py_str("sea"), py_str("shore")])))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_carded_place(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut other: Py = Py::None;
    let mut table: Py = Py::None;
    let mut travel: Py = Py::None;
    let _ = py_str("Whether the knowledge base has a card of its own for this place.\n\n    A card is the better answer to the trip itself - \"In Rome: the Colosseum with a timed ticket,\n    the Pantheon...\" names what to see, where a note about the month does not - so the trip reader\n    stands back for those places and lets the card answer. It walks in again on the next turn,\n    for the detail the card does not cover (how long, the drive, what to pack), because the store\n    keeps the plan. bot/knowledge/travel.py draws the line: the cities it has a card for, and the\n    countries it has one for, are the places a \"going to X\" line is already answered about.\n    ");
    let __try1 = exc::catch(|| {
    travel = pyrt::xrefs::xmodule("knowledge.travel");
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
    key = mcall(&_low(&[place.clone()]), "strip", &[]);
    if py_truthy(&mcall(&key.clone(), "startswith", &[py_str("the ")])) {
    key = slice_or_raise(&key.clone(), &Py::Int(4), &Py::None, &Py::None);
    }
    let __iter6 = py_iter_or_raise(&py_tuple(vec![py_str("CITIES"), py_str("BEST_TIME")]));
    for __item in __iter6 {
    name = __item.clone();
    table = { let __b = call_builtin("getattr", &[travel.clone(), name.clone(), Py::None]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&table.clone()) { let mut other: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() }, &key.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_moved_destination(args: &[Py]) -> Py {
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
    let mut live: Py = Py::None;
    let mut place: Py = Py::None;
    let _ = py_str("The trip the conversation is on, with its place put right, or None.\n\n    \"we moved it to florence\" is not a trip of its own: it is the trip already in the chat, moved.\n    The month, who and how long come from that trip, so the reply says Florence in June rather\n    than asking when it is, and the store keeps the new place (`facts` moves the plan). With no\n    trip in the chat the place is all there is, and the reply asks the one thing a friend would.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } } } }) {
    return Py::None;
    }
    place = pyrt::xrefs::xcall("memory.facts", "a_moved_place", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    if py_truthy(&_the_last_line_named_an_animal(&[memory.clone()])) {
    return Py::None;
    }
    live = _the_trip_so_far(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&live.clone()))) {
    return py_dict(vec![(py_str("place"), place.clone()), (py_str("month"), py_str("")), (py_str("when"), py_str("")), (py_str("who"), py_str("")), (py_str("how_long"), py_str("")), (py_str("by"), py_str(""))]);
    }
    if py_truthy(&Py::Bool(py_eq(&_low(&[_cap(&[{ let __b = mcall(&live.clone(), "get", &[py_str("place")]); if py_truthy(&__b) { __b } else { py_str("") } }])]), &_low(&[place.clone()])))) {
    return Py::None;
    }
    let _ = _keep_the_new_place(&[memory.clone(), place.clone(), { let __b = mcall(&live.clone(), "get", &[py_str("place")]); if py_truthy(&__b) { __b } else { py_str("") } }, line.clone()]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(live.clone()); __kws.extend(vec![("place".to_string(), place.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_last_line_named_an_animal(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line before this one gave an animal a name or asked about one.\n\n    Read through `facts.extract`, so every shape of naming an animal counts (\"my dog is called\n    rex\", \"we're getting a puppy called Biscuit\") and a line about anything else does not.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    said = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    words = mcall(&turn.clone(), "get", &[py_str("user")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[words.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&words.clone(), "strip", &[]) } }) {
    said = words.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    found = pyrt::xrefs::xcall("memory.facts", "extract", &[said.clone()]);
    return call_builtin("bool", &[{ let __b = mcall(&found.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "get", &[py_str("pets_gone")]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _keep_the_new_place(args: &[Py]) -> Py {
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
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut old: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { py_str("") };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut entry: Py = Py::None;
    let mut known: Py = Py::None;
    let mut log: Py = Py::None;
    let mut old_place: Py = Py::None;
    let _ = py_str("The store keeps the place the trip is going to now, in place of the one it left.\n\n    The trip is already in the chat, so the plan is the same plan with a new destination: \"we are\n    going to cornwall in august\" then \"no we changed it to devon\" is a week in Devon, and the next\n    line that names nothing (\"what should we do there\") has to be read against Devon.\n\n    Only the trip's own entries move. Everything else the chat has been told stays where it was -\n    \"my sister is coming in june\" is not a holiday, and the store still has to be able to say\n    \"you are going to Devon in August, and your sister is coming in June\".\n    ");
    known = pyrt::xrefs::xcall("memory", "user", &[memory.clone()]);
    old_place = mcall(&call_builtin("str", &[{ let __b = old.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let _ = py_setitem(&known.clone(), &py_str("places"), _the_place_put_right(&[mcall(&known.clone(), "get", &[py_str("places")]), old_place.clone(), place.clone()]));
    let _ = py_setitem(&known.clone(), &py_str("plans"), _the_place_put_right(&[mcall(&known.clone(), "get", &[py_str("plans")]), old_place.clone(), place.clone()]));
    let _ = py_setitem(&known.clone(), &py_str("plan"), call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&known.clone(), &py_str("plans"))) { let mut entry: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&_low(&[entry.clone()]), &_low(&[place.clone()])))) {  __out.push(entry.clone()); } } py_list(__out) })() }, place.clone()]).unwrap_or_else(|e| exc::raise(e)));
    log = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[log.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    let __assigned1 = py_list(vec![]);
    log = __assigned1.clone();
    let _ = py_setitem(&memory.clone(), &py_str("facts_log"), __assigned1.clone());
    }
    let _ = mcall(&log.clone(), "append", &[py_dict(vec![(py_str("key"), py_str("places")), (py_str("value"), place.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), said.clone())])]);
    let _ = mcall(&log.clone(), "append", &[py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), place.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), said.clone())])]);
    Py::None
}
pub fn _the_place_put_right(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut place: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut entry: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The chat's own list with the destination put right, and every other entry left alone.\n\n    The old place is replaced inside the entry that names it, so \"Cornwall in august\" becomes\n    \"Devon in august\" and keeps its month and its place in the list. Entries that name nothing\n    about the trip are copied as they are, and the new place is added only when no entry named the\n    old one.\n    ");
    kept = py_list(vec![]);
    put_right = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { entries.clone() } else { py_list(vec![]) });
    for __item in __iter1 {
    entry = __item.clone();
    text = mcall(&call_builtin("str", &[{ let __b = entry.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = old.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_low(&[text.clone()]), &_low(&[old.clone()]))) } }) {
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_call("re", "escape", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))); __args.push({ let entries = entries.clone(); let entry = entry.clone(); let kept = kept.clone(); let old = old.clone(); let place = place.clone(); let put_right = put_right.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let entries = entries.clone(); let entry = entry.clone(); let kept = kept.clone(); let old = old.clone(); let place = place.clone(); let put_right = put_right.clone(); let text = text.clone(); let _m: Py = __kwp(args, 0, "_m").unwrap_or(Py::None); place.clone() }) }); __args.push(text.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    put_right = Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&kept.clone(), &text.clone()))) {
    let _ = mcall(&kept.clone(), "append", &[text.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&put_right.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&kept.clone(), &place.clone())) } }) {
    let _ = mcall(&kept.clone(), "append", &[place.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn a_trip(args: &[Py]) -> Py {
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
    let mut cards: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cards") { v.clone() }
    else { Py::Bool(true) };
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut place: Py = Py::None;
    let mut visit: Py = Py::None;
    let _ = py_str("The trip this line states, as {place, month, who, how_long, by}, or None.\n\n    A trip is a going to a place that is not one of the everyday ones, in a line that is not a\n    question and not a trip already taken. Everything the line does not say is \"\".\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    kind = pyrt::xrefs::xcall("memory.ack", "reads_as_a_trip", &[line.clone()]);
    if py_truthy(&{ let __b = kind.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&kind.clone(), &py_str("trip"))) } }) {
    return Py::None;
    }
    low = _low(&[line.clone()]);
    if py_truthy(&{ let __b = mcall(&g_EVERYDAY_TRIP(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ON_A(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bwent\\b|\\bhave been\\b|\\bwas in\\b|\\blast year\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:when|where|how|what|why|should|shall|can|could|would|will|worth)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:we're|were|we are|i'm|im|i am)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    visit = _a_person_visited(&[line.clone()]);
    if py_truthy(&visit.clone()) {
    return py_dict(vec![(py_str("place"), visit.clone()), (py_str("month"), _month(&[low.clone()])), (py_str("when"), _when_phrase(&[low.clone()])), (py_str("who"), _who(&[low.clone()])), (py_str("how_long"), _how_long(&[low.clone()])), (py_str("by"), py_str("")), (py_str("visit"), Py::Bool(true)), (py_str("meal"), call_builtin("bool", &[mcall(&g_A_MEAL(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e))), (py_str("baby"), call_builtin("bool", &[mcall(&g_A_NEW_BABY(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_GOING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_GOING_BARE(), "search", &[low.clone()]) } }))) {
    return Py::None;
    }
    place = _places(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = cards.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_carded_place(&[place.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_BOOKED_DONE(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _month(&[low.clone()]); if py_truthy(&__b) { __b } else { _when_phrase(&[low.clone()]) } } } })) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&pyrt::xrefs::xcall("memory.facts", "extract", &[line.clone()]), "get", &[py_str("name")])) {
    return Py::None;
    }
    return py_dict(vec![(py_str("place"), place.clone()), (py_str("month"), _month(&[low.clone()])), (py_str("when"), _when_phrase(&[low.clone()])), (py_str("who"), _who(&[low.clone()])), (py_str("how_long"), _how_long(&[low.clone()])), (py_str("by"), _how_getting_there(&[low.clone()]))]);
    Py::None
}
pub fn the_trip_they_booked(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The (place, when) a line says it has booked, or (\"\", \"\").\n\n    \"we booked a cottage in cornwall for august\" is a trip to Cornwall in August, and the store\n    reads only the shape a trip is said in (\"im going to cornwall in august\"), so it kept nothing\n    and \"where are we going\" answered \"You haven't said - where are you going?\". A booking is the\n    same trip said as a thing already done.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_BOOKED(), "search", &[said.clone()])))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    place = _places(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    return py_tuple(vec![place.clone(), { let __b = _month(&[said.clone()]); if py_truthy(&__b) { __b } else { _when_phrase(&[said.clone()]) } }]);
    Py::None
}
pub fn _the_trip_so_far(args: &[Py]) -> Py {
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
    let mut facts: Py = Py::None;
    let mut latest: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trip: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The trip the conversation is on, from the plan the store keeps, or None.\n\n    The plan is kept as the phrase the person said it in (\"Cornwall in August\"), so the parts are\n    read from the line it came from - a plan can be a trip without the phrase saying so.\n    ");
    facts = pyrt::xrefs::xcall("memory", "user", &[memory.clone()]);
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = py_call(&pyrt::xrefs::xget("memory.facts", "get"), &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&p.clone(), "strip", &[]) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    latest = mcall(&facts.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&latest.clone(), "strip", &[]) } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&plans.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    said = { let __b = pyrt::xrefs::xcall("memory", "plan_line", &[memory.clone(), plan.clone()]); if py_truthy(&__b) { __b } else { plan.clone() } };
    trip = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); __kws.extend(vec![("cards".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_trip(args))(&__args) };
    if py_truthy(&trip.clone()) {
    let _ = py_setitem(&trip.clone(), &py_str("plan"), plan.clone());
    return trip.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    trip = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(memory.clone()); __kws.extend(vec![("cards".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_trip(args))(&__args) };
    if py_truthy(&trip.clone()) {
    let _ = py_setitem(&trip.clone(), &py_str("plan"), py_str(""));
    return trip.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _asked_last(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the bot's last reply ended on a question, so this one must not ask another.");
    return mcall(&mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory", "last_bot", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "endswith", &[py_str("?")]);
    Py::None
}
pub fn _told(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The replies already given this conversation, so the same line is not said twice.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_low(&[mcall(&t.clone(), "get", &[py_str("bot")])])); } } py_set(__out) })() };
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut c: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut o: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_inside: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One of options, never one the bot has just said. Same line and same turn, same pick.");
    said = _told(&[text.clone(), memory.clone()]);
    said_inside = mcall(&py_str(" "), "join", &[said.clone()]);
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut o: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &_low(&[o.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_gt(&call_builtin("len", &[o.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(15)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said_inside.clone(), &_low(&[o.clone()]))) } })) } }) {  __out.push(o.clone()); } } py_list(__out) })() };
    fresh = { let __b = fresh.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    seed = Py::Int(0);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    seed = op_add(&op_mul(&seed.clone(), &Py::Int(31)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    }
    }
    seed = op_add(&op_add(&op_mul(&seed.clone(), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&fresh.clone(), &op_modulo(&seed.clone(), &call_builtin("len", &[fresh.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _join(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut parts: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("Sentences joined as sentences: each one opens with a capital.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    part = mcall(&call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&part.clone()) {
    let _ = mcall(&out.clone(), "append", &[op_add(&mcall(&slice_or_raise(&part.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&part.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _with_them(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut ask: Py = Py::None;
    let _ = py_str("Somebody coming along who is not a child: the trip said with them, and the useful ask.");
    ask = _pick(&[g_ASK_WHAT(), memory.clone(), line.clone()]);
    return mcall(&py_str("{0} with {1}. {2}"), "format", &[_mid(&[named.clone()]), mcall(&_yours(&[who.clone()]), "lower", &[]), op_add(&mcall(&slice_or_raise(&ask.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&ask.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn _yours(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The people the person is going with, said back as theirs: \"with my mum\" -> \"with your mum\".\n\n    The bot is not the one going, so the person's own word for their family and friends is the\n    one word that has to turn round. A name is left exactly as they said it.\n    ");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("ours")]), &low.clone()))) {
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&Py::Bool(!py_eq(&low.clone(), &py_str("ours")))) { py_str("your") } else { py_str("yours") }]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&low.clone(), &py_str("mine")))) {
    let _ = mcall(&words.clone(), "append", &[py_str("yours")]);
    } else {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _mid(args: &[Py]) -> Py {
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
    let _ = py_str("A place as it reads inside a sentence: \"The Isle of Wight\" -> \"the Isle of Wight\".");
    named = call_builtin("str", &[{ let __b = named.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&mcall(&named.clone(), "startswith", &[py_str("The ")])) { op_add(&py_str("the "), &slice_or_raise(&named.clone(), &Py::Int(4), &Py::None, &Py::None)) } else { named.clone() };
    Py::None
}
pub fn _cap(args: &[Py]) -> Py {
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
    let mut position: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The place as it is written: \"cornwall\" -> \"Cornwall\", \"the isle of wight\" -> \"The Isle of\n    Wight\".");
    text = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return text.clone();
    }
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = position.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SMALL(), &mcall(&word.clone(), "lower", &[]))) } }) {
    let _ = mcall(&words.clone(), "append", &[mcall(&word.clone(), "lower", &[])]);
    } else {
    if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[])) {
    let _ = mcall(&words.clone(), "append", &[op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    } else {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _seasons_we_know(args: &[Py]) -> Py {
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
    let mut country: Py = Py::None;
    let mut key: Py = Py::None;
    let mut note: Py = Py::None;
    let mut travel: Py = Py::None;
    let _ = py_str("The knowledge base's own note on when to go there, or \"\".\n\n    Its travel data carries the seasons of the cities and countries people ask about, month by\n    month. Cornwall and Devon are not in it and keep the general season below; Thailand and Dubai\n    are, and there the general note is the wrong way round - January is their high season, not\n    their quiet one - so a place it knows is never given a guess about the weather or the prices.\n    ");
    let __try1 = exc::catch(|| {
    travel = pyrt::xrefs::xmodule("knowledge.travel");
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    key = mcall(&_low(&[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    if py_truthy(&mcall(&key.clone(), "startswith", &[py_str("the ")])) {
    key = slice_or_raise(&key.clone(), &Py::Int(4), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return py_str("");
    }
    note = mcall(&gattr(&travel.clone(), "CITY_BEST_TIME"), "get", &[key.clone()]);
    if py_truthy(&note.clone()) {
    return note.clone();
    }
    country = mcall(&gattr(&travel.clone(), "CITY_COUNTRY"), "get", &[key.clone(), key.clone()]);
    return { let __b = mcall(&gattr(&travel.clone(), "BEST_TIME"), "get", &[country.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gattr(&travel.clone(), "BEST_TIME"), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    Py::None
}
pub fn _their_month(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut note: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "note") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: note"))) };
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of that note which names their month, or \"\".");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), note.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &month.clone()), &py_str("\\b")), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    said = _mid(&[mcall(&sentence.clone(), "strip", &[])]);
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return py_str("");
    Py::None
}
pub fn _in_its_season(args: &[Py]) -> Py {
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
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut key: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut row: Py = Py::None;
    let mut travel: Py = Py::None;
    let _ = py_str("Whether the knowledge base's own month picks name this place as a place to go then.\n\n    It is the same data that answers \"where should I go in January\", read the other way round: a\n    month that names the place is a month worth going there, so \"lanzarote in january\" is not\n    told it is out of season.\n    ");
    let __try1 = exc::catch(|| {
    travel = pyrt::xrefs::xmodule("knowledge.travel");
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
    picks = { let __b = call_builtin("getattr", &[travel.clone(), py_str("_MONTH_PICKS"), Py::None]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    row = _low(&[{ let __b = mcall(&picks.clone(), "get", &[month.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&row.clone()))) {
    return Py::Bool(false);
    }
    key = mcall(&_low(&[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    if py_truthy(&mcall(&key.clone(), "startswith", &[py_str("the ")])) {
    key = slice_or_raise(&key.clone(), &Py::Int(4), &Py::None, &Py::None);
    }
    if py_truthy(&op_lt(&call_builtin("len", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[key.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), row.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_winter_place(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether this is somewhere the winter is the season, not the off-season.");
    key = mcall(&_low(&[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    if py_truthy(&mcall(&key.clone(), "startswith", &[py_str("the ")])) {
    key = slice_or_raise(&key.clone(), &Py::Int(4), &Py::None, &Py::None);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WINTER_SUN()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), key.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _when_note(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut booked: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "booked") { v.clone() }
    else { Py::Bool(false) };
    let mut place: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { py_str("") };
    let mut known: Py = Py::None;
    let mut n: Py = Py::None;
    let mut notes: Py = Py::None;
    let _ = py_str("The one useful sentence a friend adds about a trip in that month: the season itself.");
    known = _seasons_we_know(&[place.clone()]);
    if py_truthy(&known.clone()) {
    return _their_month(&[known.clone(), month.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_QUIET(), &month.clone())); if !py_truthy(&__b) { __b } else { { let __b = _a_winter_place(&[place.clone()]); if py_truthy(&__b) { __b } else { _in_its_season(&[place.clone(), month.clone()]) } } } }) {
    return _pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SEASON_NOTES()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("month".to_string(), _cap(&[month.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), text.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_PEAK(), &month.clone()))) {
    notes = if py_truthy(&booked.clone()) { g_PEAK_BOOKED_NOTES() } else { g_PEAK_NOTES() };
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_BUSY(), &month.clone()))) {
    notes = g_BUSY_NOTES();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_QUIET(), &month.clone()))) {
    notes = g_QUIET_NOTES();
    } else {
    return py_str("");
    }
    }
    }
    return _pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&notes.clone()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("month".to_string(), _cap(&[month.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), text.clone()]);
    Py::None
}
pub fn _the_visit_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut how_long: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how_long") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how_long"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut low: Py = Py::None;
    let _ = py_str("The sentence a visit gets for how long it lasts, or \"\" when the line gave no length.");
    if py_truthy(&Py::Bool(!py_truthy(&how_long.clone()))) {
    return py_str("");
    }
    low = _low(&[how_long.clone()]);
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &py_str("weekend")))) {
    return _pick(&[g_VISIT_A_WEEKEND(), memory.clone(), text.clone()]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("night|\\bday"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return _pick(&[g_VISIT_A_FEW_DAYS(), memory.clone(), text.clone()]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("week|fortnight|month"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return _pick(&[g_VISIT_A_WEEK(), memory.clone(), text.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _who_note(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut how_long: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how_long") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how_long"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut place: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { Py::None };
    let mut n: Py = Py::None;
    let mut notes: Py = Py::None;
    let _ = py_str("The sentence about who is going and for how long, if the line said either.");
    notes = py_list(vec![]);
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bkids?\\b|\\bchild|\\bchildren\\b|\\bbaby\\b|\\btoddler"), who.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:two|three|four|five|six|\\d+)\\b"), who.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&notes.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KIDS_NUMBERED()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() }]);
    } else {
    let _ = mcall(&notes.clone(), "extend", &[g_KIDS_NOTES()]);
    }
    }
    if py_truthy(&{ let __b = how_long.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&how_long.clone(), &py_str("weekend"))) } }) {
    let _ = mcall(&notes.clone(), "extend", &[g_WEEKEND_NOTES()]);
    } else {
    if py_truthy(&{ let __b = how_long.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("night|\\bday"), how_long.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&notes.clone(), "extend", &[g_DAY_NOTES()]);
    } else {
    if py_truthy(&{ let __b = how_long.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("week|fortnight|month"), how_long.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fortnight|\\d"), how_long.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&notes.clone(), "extend", &[g_LONG_NOTES()]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if py_truthy(&__b) { __b } else { _by_the_sea(&[place.clone()]) } }) {
    let _ = mcall(&notes.clone(), "extend", &[g_WEEK_NOTES()]);
    } else {
    let _ = mcall(&notes.clone(), "extend", &[g_WEEK_NOTES_LAND()]);
    }
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&notes.clone()))) {
    return py_str("");
    }
    return _pick(&[notes.clone(), memory.clone(), text.clone()]);
    Py::None
}
pub fn _a_note(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut how_long: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how_long") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how_long"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut booked: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "booked") { v.clone() }
    else { Py::Bool(false) };
    let mut place: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { py_str("") };
    let mut n: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("Both notes, when the month and the people are both worth a sentence.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![_when_note(&[month.clone(), memory.clone(), text.clone(), booked.clone(), place.clone()]), _who_note(&[who.clone(), how_long.clone(), memory.clone(), text.clone(), place.clone()])])) { let mut n: Py = __item.clone(); if py_truthy(&n.clone()) {  __out.push(n.clone()); } } py_list(__out) })() };
    return _join(&[parts.clone()]);
    Py::None
}
pub fn _a_known_city(args: &[Py]) -> Py {
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
    let mut CITIES: Py = Py::None;
    let _ = py_str("Whether the knowledge base has a card for this place, so its own answer keeps the turn.");
    place = mcall(&_low(&[place.clone()]), "strip", &[py_str(" .,;:!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    CITIES = pyrt::xrefs::xget("knowledge.travel", "CITIES");
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
    return { let __b = Py::Bool(py_contains(&CITIES.clone(), &place.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&CITIES.clone(), &geti(&mcall(&place.clone(), "split", &[]), &Py::Int(0)))) } };
    Py::None
}
pub fn _the_areas_of(args: &[Py]) -> Py {
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
    let mut CITY_AREA: Py = Py::None;
    let _ = py_str("The parts of the town the knowledge base says to stay in, or \"\" when it says none.");
    let __try1 = exc::catch(|| {
    CITY_AREA = pyrt::xrefs::xget("knowledge.travel", "CITY_AREA");
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return call_builtin("str", &[{ let __b = mcall(&CITY_AREA.clone(), "get", &[mcall(&_low(&[place.clone()]), "strip", &[py_str(" .,;:!?")])]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somewhere_for(args: &[Py]) -> Py {
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
    let mut eat: Py = Py::None;
    let mut found: Py = Py::None;
    let mut p: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stay: Py = Py::None;
    let mut t: Py = Py::None;
    let mut trip: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The reply to \"and somewhere to eat\" / \"somewhere to stay\", or None for anything else.\n\n    Only the two needs the chat can answer: a place to eat and a place for the night. A town the\n    knowledge base has a card for is left to it, so this never takes a good answer away.\n    ");
    eat = call_builtin("bool", &[mcall(&g_SOMEWHERE_TO_EAT(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    stay = call_builtin("bool", &[mcall(&g_SOMEWHERE_TO_STAY(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = eat.clone(); if py_truthy(&__b) { __b } else { stay.clone() } }))) {
    return Py::None;
    }
    trip = { let __b = _the_trip_so_far(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    place = _cap(&[_as_a_visit(&[call_builtin("str", &[{ let __b = mcall(&trip.clone(), "get", &[py_str("place")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    ref_ = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("ref")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&ref_.clone(), "get", &[py_str("there")]) } }) {
    place = _cap(&[call_builtin("str", &[geti(&ref_.clone(), &py_str("there"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&p.clone()) {  __out.push(p.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    place = _cap(&[call_builtin("str", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    said = _places(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&said.clone()) {
    place = _cap(&[said.clone()]);
    break;
    }
    }
    }
    if py_truthy(&{ let __b = place.clone(); if !py_truthy(&__b) { __b } else { { let __b = _a_known_city(&[place.clone()]); if !py_truthy(&__b) { __b } else { { let __b = eat.clone(); if py_truthy(&__b) { __b } else { _the_areas_of(&[place.clone()]) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&place.clone()) {
    pool = if py_truthy(&eat.clone()) { g_EAT_WITHOUT_A_CARD() } else { g_STAY_WITHOUT_A_CARD() };
    return _pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), _mid(&[place.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), line.clone()]);
    }
    if py_truthy(&_asked_last(&[memory.clone()])) {
    return _pick(&[call_builtin("list", &[if py_truthy(&eat.clone()) { g_EAT_ADVICE() } else { g_STAY_ADVICE() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), line.clone()]);
    }
    return _pick(&[call_builtin("list", &[if py_truthy(&eat.clone()) { g_EAT_WITHOUT_A_TOWN() } else { g_STAY_WITHOUT_A_TOWN() }]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), line.clone()]);
    Py::None
}
pub fn _the_kind_of_trip(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut how_long: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how_long") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how_long"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The trip said as a kind rather than a place: \"a weekend away\", \"a week away\".");
    said = _low(&[how_long.clone()]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("weekend")))) {
    return py_str("a weekend away");
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("fortnight")))) {
    return py_str("a fortnight away");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bweek"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("a week away");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonth"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("a month away");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bday"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("a few days away");
    }
    return py_str("a weekend away");
    Py::None
}
pub fn _the_warm_week_is_out_of_season(args: &[Py]) -> Py {
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
    let _ = py_str("True for the month the knowledge base's warm-week list gets wrong for a cold place.");
    return Py::Bool(py_contains(&g_OUT_OF_SEASON(), &mcall(&_low(&[{ let __b = month.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[])));
    Py::None
}
pub fn _an_event_not_a_place(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the \"place\" the trip reading found is a thing they are going to, not a town.");
    bare = mcall(&_low(&[{ let __b = place.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("a "), py_str("an "), py_str("the "), py_str("my "), py_str("our ")]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&mcall(&bare.clone(), "startswith", &[word.clone()])) {
    bare = slice_or_raise(&bare.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    return Py::Bool(py_contains(&g_AN_EVENT_NOT_A_PLACE(), &bare.clone()));
    Py::None
}
pub fn a_packing_question_about_the_trip(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut live: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The trip a packing question belongs to, or None.\n\n    Read by `answers` before it takes the line, so a bag written down for a baby does not answer\n    a question about a weekend away. The line itself has to be a packing question, the chat has\n    to be on a trip, and nothing in the recent lines may mention a baby: when one does, the\n    question is about that baby and every module is free to answer it.\n\n    The reading and the reply have to agree about which packing questions are the trip's, or the\n    answer module stands down for a line nothing else will take: `what should i pack for the\n    flight` after a trip to Rome lost the boarding-pass answer to a search hedge, because this\n    reading said \"the trip\" and `_a_detail` then refused the tail. The tail is read here with the\n    same test, so a question about something else to pack for keeps its own card.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = _low(&[text.clone()]);
    found = if py_truthy(&line.clone()) { mcall(&g_PACKING(), "search", &[line.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_BABY(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_A_BABY(), "search", &[_low(&[mcall(&turn.clone(), "get", &[py_str("user")])])]) } }) {
    return Py::None;
    }
    }
    live = _the_trip_so_far(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&live.clone()))) {
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    said = _low(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = mcall(&g_WEEKEND_AWAY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&said.clone(), "strip", &[]), "endswith", &[py_str("?")]))) } }) {
    live = py_dict(vec![(py_str("place"), py_str("")), (py_str("month"), _month(&[said.clone()])), (py_str("when"), py_str("")), (py_str("who"), _who(&[said.clone()])), (py_str("how_long"), { let __b = _how_long(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("a weekend") } }), (py_str("by"), py_str("")), (py_str("plan"), py_str(""))]);
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&live.clone()))) {
    return Py::None;
    }
    tail = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ?.!,")]);
    place = _low(&[mcall(&live.clone(), "get", &[py_str("place")])]);
    if py_truthy(&_an_event_not_a_place(&[place.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_PACKING_FOR(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { { let __b = place.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&tail.clone(), &op_add(&py_str("for "), &place.clone()))) } } } })) } }) {
    return Py::None;
    }
    return live.clone();
    Py::None
}
pub fn the_bag_beats_the_list(args: &[Py]) -> Py {
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
    let mut live: Py = Py::None;
    let _ = py_str("True when the bag for the place and the month is the better answer than the list.\n\n    The knowledge base's packing lists are written for a month, so they are the better answer\n    most of the year; in October the warm-week list answers a cold place with cotton tops, shorts\n    and swimwear, and there the bag the store keeps wins. The answer module stands down for the\n    line on this reading and nothing else, so every other month keeps the answer it had.\n    ");
    live = a_packing_question_about_the_trip(&[text.clone(), memory.clone()]);
    return { let __b = call_builtin("bool", &[live.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _the_warm_week_is_out_of_season(&[mcall(&live.clone(), "get", &[py_str("month")])]) } };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut cards: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cards") { v.clone() }
    else { Py::Bool(true) };
    let mut about_the_trip: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut line: Py = Py::None;
    let mut live: Py = Py::None;
    let mut month: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut somewhere: Py = Py::None;
    let mut trip: Py = Py::None;
    let _ = py_str("The reply to a trip said out loud, or a detail added to one, or None.\n\n    Only two shapes: the line that states a trip, and the short line that adds a detail to a trip\n    the conversation is already on. Everything else is another module's.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    moved = a_moved_destination(&[line.clone(), memory.clone()]);
    if py_truthy(&moved.clone()) {
    return _start(&[_a_new_trip(&[line.clone(), moved.clone(), memory.clone()])]);
    }
    trip = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(memory.clone()); __kws.extend(vec![("cards".to_string(), cards.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_trip(args))(&__args) };
    if py_truthy(&trip.clone()) {
    return _start(&[_a_new_trip(&[line.clone(), trip.clone(), memory.clone()])]);
    }
    if py_truthy(&mcall(&g_A_STAY_LENGTH(), "search", &[line.clone()])) {
    live = _the_trip_so_far(&[memory.clone()]);
    if py_truthy(&{ let __b = live.clone(); if !py_truthy(&__b) { __b } else { mcall(&live.clone(), "get", &[py_str("visit")]) } }) {
    return _start(&[_pick(&[g_VISIT_HOW_LONG(), memory.clone(), line.clone()])]);
    }
    }
    somewhere = _somewhere_for(&[line.clone(), memory.clone()]);
    if py_truthy(&somewhere.clone()) {
    return _start(&[somewhere.clone()]);
    }
    about_the_trip = { let __b = mcall(&g_PACKING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PACKING_SAID(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_DRIVE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_FLIGHT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_JOURNEY_ASK(), "search", &[line.clone()]) } } } } } } } };
    if py_truthy(&{ let __b = { let __b = mcall(&g_JUST_A_DETAIL(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = _only_a_length(&[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PACKING_SAID(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_JOURNEY_ASK(), "search", &[line.clone()]) } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "endswith", &[py_str("?")]))); if py_truthy(&__b) { __b } else { about_the_trip.clone() } } } }) {
    live = { let __b = _the_trip_so_far(&[memory.clone()]); if py_truthy(&__b) { __b } else { a_packing_question_about_the_trip(&[line.clone(), memory.clone()]) } };
    if py_truthy(&live.clone()) {
    detail = _a_detail(&[line.clone(), live.clone(), memory.clone()]);
    if py_truthy(&detail.clone()) {
    return _start(&[detail.clone()]);
    }
    }
    }
    if py_truthy(&{ let __b = mcall(&g_THE_LENGTH_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_THE_FLIGHT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_DRIVE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_JOURNEY_ASK(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_MODE(), "search", &[line.clone()]) } } } } } })) } }) {
    live = _the_trip_so_far(&[memory.clone()]);
    if py_truthy(&{ let __b = live.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&live.clone(), "get", &[py_str("how_long")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_length_said(&[memory.clone()]))) } } } }) {
    return _start(&[_the_length_asked(&[live.clone(), memory.clone(), line.clone()])]);
    }
    }
    month = _the_month_answered(&[line.clone(), memory.clone()]);
    if py_truthy(&month.clone()) {
    live = _the_trip_so_far(&[memory.clone()]);
    if py_truthy(&live.clone()) {
    live = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(live.clone()); __kws.extend(vec![("month".to_string(), month.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return _start(&[_a_new_trip(&[line.clone(), live.clone(), memory.clone()])]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _only_a_length(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let _ = py_str("Whether the line is the length of the stay and nothing else: \"for ten days\", \"two weeks\".\n\n    A length is how people answer \"how long are you going for\", so it is a detail of the trip the\n    chat is on. The words are the only thing on the line, so nothing else is being said.\n    ");
    bare = mcall(&_low(&[line.clone()]), "strip", &[py_str(" .,;:!?")]);
    bare = mcall(&stdlib_call("re", "sub", &[py_str("^(?:its|it's|thats|that's|were|we're|we are|there|were there|its for|its about|we're there)\\s+"), py_str(""), bare.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return call_builtin("bool", &[{ let __b = mcall(&g_FOR_HOW_LONG(), "fullmatch", &[bare.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_JUST_HOW_LONG(), "match", &[bare.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_month_answered(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The month in a line that is only a month, when the bot's last line asked a trip's when.");
    asked = _low(&[pyrt::xrefs::xcall("memory", "last_bot", &[memory.clone()])]);
    if py_truthy(&Py::Bool(!py_contains(&asked.clone(), &py_str("when")))) {
    return py_str("");
    }
    found = mcall(&g_JUST_A_MONTH(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    return if py_truthy(&Py::Bool(py_contains(&g_MONTHS(), &word.clone()))) { word.clone() } else { py_str("") };
    Py::None
}
pub fn _as_a_visit(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut own: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut person: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("A relative or friend someone is visiting, said back as a place they belong to.\n\n    \"im going to my sisters for the weekend\" is not a trip to a place called My Sisters. The\n    person is named with a possessive, so the reply says it the way a friend would: \"your\n    sister's\", \"your parents'\". A thing after the person stays after it: \"my parents house\" is\n    \"your parents' house\", not \"Your Parents House's\". A word that is already a place stays as\n    it was.\n    ");
    match_ = mcall(&g_A_PERSON_IS_NOT_A_PLACE(), "match", &[place.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return place.clone();
    }
    person = mcall(&match_.clone(), "group", &[py_str("person")]);
    low = mcall(&person.clone(), "lower", &[]);
    own = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("poss")]); if py_truthy(&__b) { __b } else { py_str("") } }, "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&own.clone()) {
    owner = op_add(&person.clone(), &own.clone());
    } else {
    if py_truthy(&Py::Bool(py_contains(&g__PLURAL_PEOPLE(), &low.clone()))) {
    owner = op_add(&person.clone(), &if py_truthy(&mcall(&low.clone(), "endswith", &[py_str("s")])) { py_str("'") } else { py_str("'s") });
    } else {
    owner = op_add(&person.clone(), &py_str("'s"));
    }
    }
    rest = mcall(&slice_or_raise(&place.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,;:!?")]);
    first = if py_truthy(&rest.clone()) { mcall(&geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_THING_OF_THEIRS(), &first.clone())) } }) {
    return op_add(&op_add(&op_add(&py_str("your "), &owner.clone()), &py_str(" ")), &rest.clone());
    }
    return op_add(&py_str("your "), &owner.clone());
    Py::None
}
pub fn _a_new_trip(args: &[Py]) -> Py {
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
    let mut trip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trip"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut ask: Py = Py::None;
    let mut booked: Py = Py::None;
    let mut day: Py = Py::None;
    let mut month: Py = Py::None;
    let mut named: Py = Py::None;
    let mut note: Py = Py::None;
    let mut notes: Py = Py::None;
    let mut outing: Py = Py::None;
    let mut part: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sea: Py = Py::None;
    let mut visit: Py = Py::None;
    let mut who: Py = Py::None;
    let mut who_note: Py = Py::None;
    let mut word: Py = Py::None;
    visit = _as_a_visit(&[geti(&trip.clone(), &py_str("place"))]);
    let __unpk1 = py_unpack(&py_tuple(vec![_cap(&[visit.clone()]), geti(&trip.clone(), &py_str("who"))]), 2, None);
    place = __unpk1[0].clone();
    who = __unpk1[1].clone();
    month = geti(&trip.clone(), &py_str("month"));
    outing = mcall(&g_OUTING(), "match", &[_low(&[geti(&trip.clone(), &py_str("place"))])]);
    if py_truthy(&outing.clone()) {
    word = mcall(&mcall(&outing.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    sea = Py::Bool(py_contains(&g_SEA(), &word.clone()));
    said = mcall(&py_str("A day {0} the {1}, {2}. "), "format", &[if py_truthy(&Py::Bool(py_contains(&g_OPEN_AIR(), &word.clone()))) { py_str("in") } else { py_str("at") }, word.clone(), _pick(&[g_WARM(), memory.clone(), line.clone()])]);
    who_note = py_str("");
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { sea.clone() } }) {
    who_note = _who_note(&[who.clone(), geti(&trip.clone(), &py_str("how_long")), memory.clone(), line.clone()]);
    }
    if py_truthy(&who_note.clone()) {
    return op_add(&said.clone(), &_join(&[py_list(vec![who_note.clone()])]));
    }
    notes = if py_truthy(&sea.clone()) { g_SEA_NOTES() } else { if py_truthy(&Py::Bool(py_contains(&g_TICKETED(), &word.clone()))) { g_INDOOR_NOTES() } else { g_LAND_NOTES() } };
    return op_add(&said.clone(), &_join(&[py_list(vec![_pick(&[notes.clone(), memory.clone(), line.clone()])])]));
    }
    day = { let __b = mcall(&trip.clone(), "get", &[py_str("when")]); if py_truthy(&__b) { __b } else { py_str("") } };
    day = if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DAYS(), &day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&day.clone(), &py_str("weekend"))) } }) { day.clone() } else { py_str("") };
    named = op_add(&place.clone(), &if py_truthy(&month.clone()) { op_add(&py_str(" in "), &_cap(&[month.clone()])) } else { if py_truthy(&day.clone()) { op_add(&py_str(" on "), &_cap(&[day.clone()])) } else { py_str("") } });
    if py_truthy(&who.clone()) {
    let __aug2 = op_add(&named.clone(), &op_add(&py_str(" with "), &_yours(&[who.clone()])));
    named = __aug2;
    }
    booked = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bbook"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&booked.clone()) { mcall(&py_str("{0} is booked, then. "), "format", &[named.clone()]) } else { mcall(&py_str("{0}, {1}. "), "format", &[named.clone(), _pick(&[g_WARM(), memory.clone(), line.clone()])]) };
    if py_truthy(&Py::Bool(!py_eq(&visit.clone(), &geti(&trip.clone(), &py_str("place"))))) {
    note = _join(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![_who_note(&[who.clone(), py_str(""), memory.clone(), line.clone()]), _the_visit_length(&[geti(&trip.clone(), &py_str("how_long")), memory.clone(), line.clone()])])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]);
    if py_truthy(&note.clone()) {
    return _join(&[py_list(vec![said.clone(), note.clone()])]);
    }
    ask = _pick(&[g_ASK_VISIT(), memory.clone(), line.clone()]);
    return _join(&[py_list(vec![said.clone(), op_add(&mcall(&slice_or_raise(&ask.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&ask.clone(), &Py::Int(1), &Py::None, &Py::None))])]);
    }
    note = _a_note(&[month.clone(), who.clone(), geti(&trip.clone(), &py_str("how_long")), memory.clone(), line.clone(), booked.clone(), geti(&trip.clone(), &py_str("place"))]);
    if py_truthy(&note.clone()) {
    return op_add(&said.clone(), &note.clone());
    }
    if py_truthy(&booked.clone()) {
    return op_add(&said.clone(), &py_str("Get the big days out booked now too, and you can take the rest as it comes."));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&month.clone())); if !py_truthy(&__b) { __b } else { day.clone() } }) {
    return mcall(&said.clone(), "strip", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&month.clone())); if !py_truthy(&__b) { __b } else { mcall(&trip.clone(), "get", &[py_str("when")]) } }) {
    return op_add(&op_add(&said.clone(), &mcall(&slice_or_raise(&_pick(&[g_ASK_WHAT(), memory.clone(), line.clone()]), &Py::None, &Py::Int(1), &Py::None), "upper", &[])), &slice_or_raise(&_pick(&[g_ASK_WHAT(), memory.clone(), line.clone()]), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(!py_truthy(&month.clone()))) {
    if py_truthy(&_asked_last(&[memory.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), place.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_pick(&[g_ASK_WHEN_NO_QUESTION(), memory.clone(), line.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), place.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_pick(&[g_ASK_WHEN(), memory.clone(), line.clone()]), "format", args))(&__args) };
    }
    return op_add(&op_add(&said.clone(), &mcall(&slice_or_raise(&_pick(&[g_ASK_WHAT(), memory.clone(), line.clone()]), &Py::None, &Py::Int(1), &Py::None), "upper", &[])), &slice_or_raise(&_pick(&[g_ASK_WHAT(), memory.clone(), line.clone()]), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _a_length_said(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The length the chat has already given for the trip, in their own words, or None.\n\n    \"we are going to spain in august\" / \"its for two weeks\" / \"how long are we there for\": the\n    length is on the table even though no slot holds it - nothing files a line that only says a\n    length - and the ask is answered by the line they said it on (\"You said two weeks.\"). Asking\n    the question back there told the person the chat had not heard what they had just said, which\n    is what the ask branch did before this guard.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { mcall(&mcall(&text.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    continue;
    }
    found = mcall(&g_FOR_HOW_LONG(), "search", &[text.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&found.clone(), "group", &[Py::Int(1)]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_length_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut trip: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trip"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut month: Py = Py::None;
    let mut n: Py = Py::None;
    let mut named: Py = Py::None;
    let mut options: Py = Py::None;
    let mut place: Py = Py::None;
    let _ = py_str("The length of the stay asked while the chat has not said one: the question back.");
    place = _cap(&[_as_a_visit(&[{ let __b = mcall(&trip.clone(), "get", &[py_str("place")]); if py_truthy(&__b) { __b } else { py_str("") } }])]);
    month = { let __b = mcall(&trip.clone(), "get", &[py_str("month")]); if py_truthy(&__b) { __b } else { py_str("") } };
    options = g_THE_LENGTH_NOT_SAID();
    if py_truthy(&place.clone()) {
    named = _mid(&[op_add(&place.clone(), &if py_truthy(&month.clone()) { op_add(&py_str(" in "), &_cap(&[month.clone()])) } else { py_str("") })]);
    options = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_LENGTH_NOT_SAID_NAMED()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("named".to_string(), named.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return _join(&[py_list(vec![_pick(&[options.clone(), memory.clone(), line.clone()]), _pick(&[g_THE_NIGHTS_FIRST(), memory.clone(), line.clone()])])]);
    Py::None
}
pub fn _a_detail(args: &[Py]) -> Py {
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
    let mut trip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trip"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut by: Py = Py::None;
    let mut how_long: Py = Py::None;
    let mut kids: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut month: Py = Py::None;
    let mut n: Py = Py::None;
    let mut named: Py = Py::None;
    let mut note: Py = Py::None;
    let mut notes: Py = Py::None;
    let mut packing: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said_as_a_need: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut told: Py = Py::None;
    let mut who: Py = Py::None;
    low = _low(&[line.clone()]);
    let __unpk1 = py_unpack(&py_tuple(vec![_cap(&[_as_a_visit(&[geti(&trip.clone(), &py_str("place"))])]), geti(&trip.clone(), &py_str("month"))]), 2, None);
    place = __unpk1[0].clone();
    month = __unpk1[1].clone();
    named = op_add(&place.clone(), &if py_truthy(&month.clone()) { op_add(&py_str(" in "), &_cap(&[month.clone()])) } else { py_str("") });
    who = { let __b = _who(&[low.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    how_long = _how_long(&[low.clone()]);
    by = if py_truthy(&Py::Bool(!py_contains(&low.clone(), &_low(&[place.clone()])))) { _how_getting_there(&[low.clone()]) } else { py_str("") };
    told = py_list(vec![]);
    named = _mid(&[named.clone()]);
    kids = stdlib_call("re", "search", &[py_str("\\bkids?\\b|\\bchild|\\bchildren\\b|\\bbaby\\b|\\btoddler"), { let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    packing = { let __b = mcall(&g_PACKING(), "match", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PACKING_SAID(), "match", &[low.clone()]) } };
    if py_truthy(&packing.clone()) {
    tail = mcall(&mcall(&packing.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ?.!,")]);
    said_as_a_need = call_builtin("bool", &[mcall(&g_PACKING_SAID(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    kind = _the_kind_of_trip(&[{ let __b = mcall(&trip.clone(), "get", &[py_str("how_long")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&month.clone()) {
    pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PACKING_NO_PLACE()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("kind".to_string(), kind.clone()), ("month".to_string(), _cap(&[month.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    } else {
    pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PACKING_NO_PLACE_NO_MONTH()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("kind".to_string(), kind.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    }
    return _join(&[py_list(vec![_pick(&[pool.clone(), memory.clone(), line.clone()])])]);
    }
    if py_truthy(&mcall(&trip.clone(), "get", &[py_str("visit")])) {
    if py_truthy(&{ let __b = said_as_a_need.clone(); if !py_truthy(&__b) { __b } else { _one_thing_they_need(&[low.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&trip.clone(), "get", &[py_str("baby")])) {
    return _pick(&[g_VISIT_PACKING_FOR_A_BABY(), memory.clone(), line.clone()]);
    }
    if py_truthy(&mcall(&trip.clone(), "get", &[py_str("meal")])) {
    return _pick(&[g_VISIT_PACKING_FOR_A_MEAL(), memory.clone(), line.clone()]);
    }
    return _pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VISIT_PACKING()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), _mid(&[place.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), line.clone()]);
    }
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said_as_a_need.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_PACKING_FOR(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_low(&[tail.clone()]), &op_add(&py_str("for "), &_low(&[place.clone()])))) } })) } } } }) {
    return Py::None;
    }
    if py_truthy(&_an_event_not_a_place(&[place.clone()])) {
    return Py::None;
    }
    if py_truthy(&month.clone()) {
    notes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PACKING_NOTES()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), _mid(&[place.clone()])), ("month".to_string(), _cap(&[month.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    } else {
    notes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PACKING_NO_MONTH()) { let mut n: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), _mid(&[place.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&n.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    }
    if py_truthy(&kids.clone()) {
    let _ = mcall(&notes.clone(), "append", &[g_PACKING_WITH_KIDS()]);
    }
    return _join(&[py_list(vec![_pick(&[notes.clone(), memory.clone(), line.clone()])])]);
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_THE_FLIGHT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_DRIVE(), "search", &[low.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_LONG_ASK(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { _a_carded_place(&[place.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&by.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_MODE(), "search", &[low.clone()]))) } }) {
    if py_truthy(&mcall(&g_THE_FLIGHT(), "search", &[low.clone()])) {
    by = py_str("flying");
    } else {
    if py_truthy(&mcall(&g_THE_DRIVE(), "search", &[low.clone()])) {
    by = py_str("driving");
    } else {
    if py_truthy(&mcall(&g_THE_JOURNEY_ASK(), "search", &[low.clone()])) {
    by = if py_truthy(&mcall(&g_JOURNEY_MODE(), "search", &[low.clone()])) { py_str("driving") } else { py_str("either") };
    }
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&by.clone(), &py_str("either"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&by.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_THE_MODE(), "search", &[low.clone()]) } } } }) {
    let _ = mcall(&told.clone(), "append", &[mcall(&py_str("To {0}: the car or the train if it is a few hours, a flight if it is not - the train is the one you can leave late on."), "format", &[named.clone()])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("driving"), py_str("by car")]), &by.clone()))) {
    let _ = mcall(&told.clone(), "append", &[mcall(&py_str("Driving to {0}, so leave early and take the break halfway - the last hour is the one that drags."), "format", &[named.clone()])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("flying"), py_str("by plane")]), &by.clone()))) {
    let _ = mcall(&told.clone(), "append", &[mcall(&py_str("Flying to {0} - take the early flight, because the day you lose waiting around is the first day of the trip."), "format", &[named.clone()])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("getting the train"), py_str("on the train"), py_str("by train"), py_str("getting a train")]), &by.clone()))) {
    let _ = mcall(&told.clone(), "append", &[mcall(&py_str("Taking the train to {0} is the easy way in - book it now, the cheap seats go first."), "format", &[named.clone()])]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("taking the ferry"), py_str("getting a ferry"), py_str("sailing"), py_str("by ferry")]), &by.clone()))) {
    let _ = mcall(&told.clone(), "append", &[mcall(&py_str("On the ferry to {0}, so book it now - the summer crossings fill up."), "format", &[named.clone()])]);
    }
    }
    }
    }
    }
    if py_truthy(&told.clone()) {
    if py_truthy(&who.clone()) {
    let _ = mcall(&told.clone(), "append", &[{ let __b = _who_note(&[who.clone(), how_long.clone(), memory.clone(), line.clone(), place.clone()]); if py_truthy(&__b) { __b } else { _with_them(&[named.clone(), who.clone(), memory.clone(), line.clone()]) } }]);
    }
    return _join(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&told.clone()) { let mut t: Py = __item.clone(); if py_truthy(&t.clone()) {  __out.push(t.clone()); } } py_list(__out) })() }]);
    }
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { kids.clone() } }) {
    return _join(&[py_list(vec![_who_note(&[who.clone(), how_long.clone(), memory.clone(), line.clone(), place.clone()])])]);
    }
    if py_truthy(&who.clone()) {
    return _with_them(&[named.clone(), who.clone(), memory.clone(), line.clone()]);
    }
    note = _who_note(&[who.clone(), how_long.clone(), memory.clone(), line.clone(), place.clone()]);
    if py_truthy(&note.clone()) {
    return _join(&[py_list(vec![note.clone()])]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.trips".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_EVENT_NOT_A_PLACE" => Some(g_AN_EVENT_NOT_A_PLACE()),
            "ASK_VISIT" => Some(g_ASK_VISIT()),
            "ASK_WHAT" => Some(g_ASK_WHAT()),
            "ASK_WHEN" => Some(g_ASK_WHEN()),
            "ASK_WHEN_NO_QUESTION" => Some(g_ASK_WHEN_NO_QUESTION()),
            "A_BABY" => Some(g_A_BABY()),
            "A_CARING_PLACE_HEAD" => Some(g_A_CARING_PLACE_HEAD()),
            "A_HAPPY_COURT" => Some(g_A_HAPPY_COURT()),
            "A_MEAL" => Some(g_A_MEAL()),
            "A_NEW_BABY" => Some(g_A_NEW_BABY()),
            "A_PERSON_IS_NOT_A_PLACE" => Some(g_A_PERSON_IS_NOT_A_PLACE()),
            "A_POSSESSION_NOT_A_PLACE" => Some(g_A_POSSESSION_NOT_A_PLACE()),
            "A_STAY_LENGTH" => Some(g_A_STAY_LENGTH()),
            "A_WHEN_PHRASE" => Some(g_A_WHEN_PHRASE()),
            "BARE_DAY" => Some(g_BARE_DAY()),
            "BARE_WHO" => Some(g_BARE_WHO()),
            "BOOKED" => Some(g_BOOKED()),
            "BOOKED_DONE" => Some(g_BOOKED_DONE()),
            "BOOKED_IN_PLACE" => Some(g_BOOKED_IN_PLACE()),
            "BOOKED_IN_WHEN" => Some(g_BOOKED_IN_WHEN()),
            "BOOKED_PLACE" => Some(g_BOOKED_PLACE()),
            "BOOKED_THING" => Some(g_BOOKED_THING()),
            "BUSY" => Some(g_BUSY()),
            "BUSY_NOTES" => Some(g_BUSY_NOTES()),
            "BY" => Some(g_BY()),
            "DAYS" => Some(g_DAYS()),
            "DAY_NOTES" => Some(g_DAY_NOTES()),
            "DURATION" => Some(g_DURATION()),
            "EAT_ADVICE" => Some(g_EAT_ADVICE()),
            "EAT_WITHOUT_A_CARD" => Some(g_EAT_WITHOUT_A_CARD()),
            "EAT_WITHOUT_A_TOWN" => Some(g_EAT_WITHOUT_A_TOWN()),
            "EVERYDAY_TRIP" => Some(g_EVERYDAY_TRIP()),
            "FOR_HOW_LONG" => Some(g_FOR_HOW_LONG()),
            "GOING" => Some(g_GOING()),
            "GOING_BARE" => Some(g_GOING_BARE()),
            "HOW_LONG_ASK" => Some(g_HOW_LONG_ASK()),
            "INDOOR_NOTES" => Some(g_INDOOR_NOTES()),
            "IN_PLACE" => Some(g_IN_PLACE()),
            "JOURNEY_MODE" => Some(g_JOURNEY_MODE()),
            "JUST_A_DETAIL" => Some(g_JUST_A_DETAIL()),
            "JUST_A_MONTH" => Some(g_JUST_A_MONTH()),
            "JUST_HOW_LONG" => Some(g_JUST_HOW_LONG()),
            "KIDS_NOTES" => Some(g_KIDS_NOTES()),
            "KIDS_NUMBERED" => Some(g_KIDS_NUMBERED()),
            "KIDS_ONE" => Some(g_KIDS_ONE()),
            "KIN" => Some(g_KIN()),
            "LAND_NOTES" => Some(g_LAND_NOTES()),
            "LONG_NOTES" => Some(g_LONG_NOTES()),
            "MONTHS" => Some(g_MONTHS()),
            "NOT_AN_AMOUNT" => Some(g_NOT_AN_AMOUNT()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NOT_A_TRIP" => Some(g_NOT_A_TRIP()),
            "NOT_PLACES" => Some(g_NOT_PLACES()),
            "NO_THING_NAMED" => Some(g_NO_THING_NAMED()),
            "NUMBER_WORD" => Some(g_NUMBER_WORD()),
            "ONE_THING_NEEDED" => Some(g_ONE_THING_NEEDED()),
            "ON_A" => Some(g_ON_A()),
            "OPEN_AIR" => Some(g_OPEN_AIR()),
            "OUTING" => Some(g_OUTING()),
            "OUT_OF_SEASON" => Some(g_OUT_OF_SEASON()),
            "PACKING" => Some(g_PACKING()),
            "PACKING_FOR" => Some(g_PACKING_FOR()),
            "PACKING_NOTES" => Some(g_PACKING_NOTES()),
            "PACKING_NO_MONTH" => Some(g_PACKING_NO_MONTH()),
            "PACKING_NO_PLACE" => Some(g_PACKING_NO_PLACE()),
            "PACKING_NO_PLACE_NO_MONTH" => Some(g_PACKING_NO_PLACE_NO_MONTH()),
            "PACKING_SAID" => Some(g_PACKING_SAID()),
            "PACKING_WITH_KIDS" => Some(g_PACKING_WITH_KIDS()),
            "PEAK" => Some(g_PEAK()),
            "PEAK_BOOKED_NOTES" => Some(g_PEAK_BOOKED_NOTES()),
            "PEAK_NOTES" => Some(g_PEAK_NOTES()),
            "PLACE_VERBS" => Some(g_PLACE_VERBS()),
            "PLURAL_RELATIONS" => Some(g_PLURAL_RELATIONS()),
            "QUIET" => Some(g_QUIET()),
            "QUIET_NOTES" => Some(g_QUIET_NOTES()),
            "SEA" => Some(g_SEA()),
            "SEASON" => Some(g_SEASON()),
            "SEASON_NOTES" => Some(g_SEASON_NOTES()),
            "SEA_NOTES" => Some(g_SEA_NOTES()),
            "SEA_PLACES" => Some(g_SEA_PLACES()),
            "SMALL" => Some(g_SMALL()),
            "SOMEWHERE_TO_EAT" => Some(g_SOMEWHERE_TO_EAT()),
            "SOMEWHERE_TO_STAY" => Some(g_SOMEWHERE_TO_STAY()),
            "STAY_ADVICE" => Some(g_STAY_ADVICE()),
            "STAY_THING" => Some(g_STAY_THING()),
            "STAY_WITHOUT_A_CARD" => Some(g_STAY_WITHOUT_A_CARD()),
            "STAY_WITHOUT_A_TOWN" => Some(g_STAY_WITHOUT_A_TOWN()),
            "STOP_AFTER" => Some(g_STOP_AFTER()),
            "THE_DRIVE" => Some(g_THE_DRIVE()),
            "THE_FLIGHT" => Some(g_THE_FLIGHT()),
            "THE_JOURNEY_ASK" => Some(g_THE_JOURNEY_ASK()),
            "THE_LENGTH_ASK" => Some(g_THE_LENGTH_ASK()),
            "THE_LENGTH_NOT_SAID" => Some(g_THE_LENGTH_NOT_SAID()),
            "THE_LENGTH_NOT_SAID_NAMED" => Some(g_THE_LENGTH_NOT_SAID_NAMED()),
            "THE_MODE" => Some(g_THE_MODE()),
            "THE_NIGHTS_FIRST" => Some(g_THE_NIGHTS_FIRST()),
            "TICKETED" => Some(g_TICKETED()),
            "TIME_WORD" => Some(g_TIME_WORD()),
            "TO_PLACE" => Some(g_TO_PLACE()),
            "VISIT_A_FEW_DAYS" => Some(g_VISIT_A_FEW_DAYS()),
            "VISIT_A_WEEK" => Some(g_VISIT_A_WEEK()),
            "VISIT_A_WEEKEND" => Some(g_VISIT_A_WEEKEND()),
            "VISIT_HOW_LONG" => Some(g_VISIT_HOW_LONG()),
            "VISIT_PACKING" => Some(g_VISIT_PACKING()),
            "VISIT_PACKING_FOR_A_BABY" => Some(g_VISIT_PACKING_FOR_A_BABY()),
            "VISIT_PACKING_FOR_A_MEAL" => Some(g_VISIT_PACKING_FOR_A_MEAL()),
            "WARM" => Some(g_WARM()),
            "WEEKEND_AWAY" => Some(g_WEEKEND_AWAY()),
            "WEEKEND_NOTES" => Some(g_WEEKEND_NOTES()),
            "WEEK_NOTES" => Some(g_WEEK_NOTES()),
            "WEEK_NOTES_LAND" => Some(g_WEEK_NOTES_LAND()),
            "WHEN_WORD" => Some(g_WHEN_WORD()),
            "WINTER_SUN" => Some(g_WINTER_SUN()),
            "WITH_WHO" => Some(g_WITH_WHO()),
            "WORD" => Some(g_WORD()),
            "_COUNT" => Some(g__COUNT()),
            "_GROUP" => Some(g__GROUP()),
            "_NOT_A_THING_OF_THEIRS" => Some(g__NOT_A_THING_OF_THEIRS()),
            "_PLURAL_PEOPLE" => Some(g__PLURAL_PEOPLE()),
            "_a_carded_place" => Some(py_func("_a_carded_place", |args: &[Py]| _a_carded_place(args))),
            "_a_detail" => Some(py_func("_a_detail", |args: &[Py]| _a_detail(args))),
            "_a_known_city" => Some(py_func("_a_known_city", |args: &[Py]| _a_known_city(args))),
            "_a_length_said" => Some(py_func("_a_length_said", |args: &[Py]| _a_length_said(args))),
            "_a_new_trip" => Some(py_func("_a_new_trip", |args: &[Py]| _a_new_trip(args))),
            "_a_note" => Some(py_func("_a_note", |args: &[Py]| _a_note(args))),
            "_a_person_visited" => Some(py_func("_a_person_visited", |args: &[Py]| _a_person_visited(args))),
            "_a_relation_not_a_place" => Some(py_func("_a_relation_not_a_place", |args: &[Py]| _a_relation_not_a_place(args))),
            "_a_thing_booked_for_when" => Some(py_func("_a_thing_booked_for_when", |args: &[Py]| _a_thing_booked_for_when(args))),
            "_a_winter_place" => Some(py_func("_a_winter_place", |args: &[Py]| _a_winter_place(args))),
            "_an_event_not_a_place" => Some(py_func("_an_event_not_a_place", |args: &[Py]| _an_event_not_a_place(args))),
            "_as_a_visit" => Some(py_func("_as_a_visit", |args: &[Py]| _as_a_visit(args))),
            "_asked_last" => Some(py_func("_asked_last", |args: &[Py]| _asked_last(args))),
            "_by_the_sea" => Some(py_func("_by_the_sea", |args: &[Py]| _by_the_sea(args))),
            "_cap" => Some(py_func("_cap", |args: &[Py]| _cap(args))),
            "_how_getting_there" => Some(py_func("_how_getting_there", |args: &[Py]| _how_getting_there(args))),
            "_how_long" => Some(py_func("_how_long", |args: &[Py]| _how_long(args))),
            "_in_its_season" => Some(py_func("_in_its_season", |args: &[Py]| _in_its_season(args))),
            "_join" => Some(py_func("_join", |args: &[Py]| _join(args))),
            "_keep_the_new_place" => Some(py_func("_keep_the_new_place", |args: &[Py]| _keep_the_new_place(args))),
            "_low" => Some(py_func("_low", |args: &[Py]| _low(args))),
            "_mid" => Some(py_func("_mid", |args: &[Py]| _mid(args))),
            "_month" => Some(py_func("_month", |args: &[Py]| _month(args))),
            "_one_thing_they_need" => Some(py_func("_one_thing_they_need", |args: &[Py]| _one_thing_they_need(args))),
            "_only_a_length" => Some(py_func("_only_a_length", |args: &[Py]| _only_a_length(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_places" => Some(py_func("_places", |args: &[Py]| _places(args))),
            "_seasons_we_know" => Some(py_func("_seasons_we_know", |args: &[Py]| _seasons_we_know(args))),
            "_somewhere_for" => Some(py_func("_somewhere_for", |args: &[Py]| _somewhere_for(args))),
            "_start" => Some(py_func("_start", |args: &[Py]| _start(args))),
            "_the_areas_of" => Some(py_func("_the_areas_of", |args: &[Py]| _the_areas_of(args))),
            "_the_kind_of_trip" => Some(py_func("_the_kind_of_trip", |args: &[Py]| _the_kind_of_trip(args))),
            "_the_last_line_named_an_animal" => Some(py_func("_the_last_line_named_an_animal", |args: &[Py]| _the_last_line_named_an_animal(args))),
            "_the_length_asked" => Some(py_func("_the_length_asked", |args: &[Py]| _the_length_asked(args))),
            "_the_month_answered" => Some(py_func("_the_month_answered", |args: &[Py]| _the_month_answered(args))),
            "_the_place_put_right" => Some(py_func("_the_place_put_right", |args: &[Py]| _the_place_put_right(args))),
            "_the_thing_booked" => Some(py_func("_the_thing_booked", |args: &[Py]| _the_thing_booked(args))),
            "_the_trip_so_far" => Some(py_func("_the_trip_so_far", |args: &[Py]| _the_trip_so_far(args))),
            "_the_visit_length" => Some(py_func("_the_visit_length", |args: &[Py]| _the_visit_length(args))),
            "_the_warm_week_is_out_of_season" => Some(py_func("_the_warm_week_is_out_of_season", |args: &[Py]| _the_warm_week_is_out_of_season(args))),
            "_their_month" => Some(py_func("_their_month", |args: &[Py]| _their_month(args))),
            "_told" => Some(py_func("_told", |args: &[Py]| _told(args))),
            "_when_note" => Some(py_func("_when_note", |args: &[Py]| _when_note(args))),
            "_when_phrase" => Some(py_func("_when_phrase", |args: &[Py]| _when_phrase(args))),
            "_who" => Some(py_func("_who", |args: &[Py]| _who(args))),
            "_who_note" => Some(py_func("_who_note", |args: &[Py]| _who_note(args))),
            "_with_them" => Some(py_func("_with_them", |args: &[Py]| _with_them(args))),
            "_yours" => Some(py_func("_yours", |args: &[Py]| _yours(args))),
            "a_moved_destination" => Some(py_func("a_moved_destination", |args: &[Py]| a_moved_destination(args))),
            "a_packing_question_about_the_trip" => Some(py_func("a_packing_question_about_the_trip", |args: &[Py]| a_packing_question_about_the_trip(args))),
            "a_trip" => Some(py_func("a_trip", |args: &[Py]| a_trip(args))),
            "last_bot" => Some(g_last_bot()),
            "plan_line" => Some(g_plan_line()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_bag_beats_the_list" => Some(py_func("the_bag_beats_the_list", |args: &[Py]| the_bag_beats_the_list(args))),
            "the_trip_they_booked" => Some(py_func("the_trip_they_booked", |args: &[Py]| the_trip_they_booked(args))),
            "user" => Some(g_user()),
            "ack" => Some(pyrt::xrefs::xmodule("memory.ack")),
            "facts" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "not_a_trip" => Some(pyrt::xrefs::xmodule("memory.not_a_trip")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_EVENT_NOT_A_PLACE" => s_AN_EVENT_NOT_A_PLACE(v),
            "ASK_VISIT" => s_ASK_VISIT(v),
            "ASK_WHAT" => s_ASK_WHAT(v),
            "ASK_WHEN" => s_ASK_WHEN(v),
            "ASK_WHEN_NO_QUESTION" => s_ASK_WHEN_NO_QUESTION(v),
            "A_BABY" => s_A_BABY(v),
            "A_CARING_PLACE_HEAD" => s_A_CARING_PLACE_HEAD(v),
            "A_HAPPY_COURT" => s_A_HAPPY_COURT(v),
            "A_MEAL" => s_A_MEAL(v),
            "A_NEW_BABY" => s_A_NEW_BABY(v),
            "A_PERSON_IS_NOT_A_PLACE" => s_A_PERSON_IS_NOT_A_PLACE(v),
            "A_POSSESSION_NOT_A_PLACE" => s_A_POSSESSION_NOT_A_PLACE(v),
            "A_STAY_LENGTH" => s_A_STAY_LENGTH(v),
            "A_WHEN_PHRASE" => s_A_WHEN_PHRASE(v),
            "BARE_DAY" => s_BARE_DAY(v),
            "BARE_WHO" => s_BARE_WHO(v),
            "BOOKED" => s_BOOKED(v),
            "BOOKED_DONE" => s_BOOKED_DONE(v),
            "BOOKED_IN_PLACE" => s_BOOKED_IN_PLACE(v),
            "BOOKED_IN_WHEN" => s_BOOKED_IN_WHEN(v),
            "BOOKED_PLACE" => s_BOOKED_PLACE(v),
            "BOOKED_THING" => s_BOOKED_THING(v),
            "BUSY" => s_BUSY(v),
            "BUSY_NOTES" => s_BUSY_NOTES(v),
            "BY" => s_BY(v),
            "DAYS" => s_DAYS(v),
            "DAY_NOTES" => s_DAY_NOTES(v),
            "DURATION" => s_DURATION(v),
            "EAT_ADVICE" => s_EAT_ADVICE(v),
            "EAT_WITHOUT_A_CARD" => s_EAT_WITHOUT_A_CARD(v),
            "EAT_WITHOUT_A_TOWN" => s_EAT_WITHOUT_A_TOWN(v),
            "EVERYDAY_TRIP" => s_EVERYDAY_TRIP(v),
            "FOR_HOW_LONG" => s_FOR_HOW_LONG(v),
            "GOING" => s_GOING(v),
            "GOING_BARE" => s_GOING_BARE(v),
            "HOW_LONG_ASK" => s_HOW_LONG_ASK(v),
            "INDOOR_NOTES" => s_INDOOR_NOTES(v),
            "IN_PLACE" => s_IN_PLACE(v),
            "JOURNEY_MODE" => s_JOURNEY_MODE(v),
            "JUST_A_DETAIL" => s_JUST_A_DETAIL(v),
            "JUST_A_MONTH" => s_JUST_A_MONTH(v),
            "JUST_HOW_LONG" => s_JUST_HOW_LONG(v),
            "KIDS_NOTES" => s_KIDS_NOTES(v),
            "KIDS_NUMBERED" => s_KIDS_NUMBERED(v),
            "KIDS_ONE" => s_KIDS_ONE(v),
            "KIN" => s_KIN(v),
            "LAND_NOTES" => s_LAND_NOTES(v),
            "LONG_NOTES" => s_LONG_NOTES(v),
            "MONTHS" => s_MONTHS(v),
            "NOT_AN_AMOUNT" => s_NOT_AN_AMOUNT(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NOT_A_TRIP" => s_NOT_A_TRIP(v),
            "NOT_PLACES" => s_NOT_PLACES(v),
            "NO_THING_NAMED" => s_NO_THING_NAMED(v),
            "NUMBER_WORD" => s_NUMBER_WORD(v),
            "ONE_THING_NEEDED" => s_ONE_THING_NEEDED(v),
            "ON_A" => s_ON_A(v),
            "OPEN_AIR" => s_OPEN_AIR(v),
            "OUTING" => s_OUTING(v),
            "OUT_OF_SEASON" => s_OUT_OF_SEASON(v),
            "PACKING" => s_PACKING(v),
            "PACKING_FOR" => s_PACKING_FOR(v),
            "PACKING_NOTES" => s_PACKING_NOTES(v),
            "PACKING_NO_MONTH" => s_PACKING_NO_MONTH(v),
            "PACKING_NO_PLACE" => s_PACKING_NO_PLACE(v),
            "PACKING_NO_PLACE_NO_MONTH" => s_PACKING_NO_PLACE_NO_MONTH(v),
            "PACKING_SAID" => s_PACKING_SAID(v),
            "PACKING_WITH_KIDS" => s_PACKING_WITH_KIDS(v),
            "PEAK" => s_PEAK(v),
            "PEAK_BOOKED_NOTES" => s_PEAK_BOOKED_NOTES(v),
            "PEAK_NOTES" => s_PEAK_NOTES(v),
            "PLACE_VERBS" => s_PLACE_VERBS(v),
            "PLURAL_RELATIONS" => s_PLURAL_RELATIONS(v),
            "QUIET" => s_QUIET(v),
            "QUIET_NOTES" => s_QUIET_NOTES(v),
            "SEA" => s_SEA(v),
            "SEASON" => s_SEASON(v),
            "SEASON_NOTES" => s_SEASON_NOTES(v),
            "SEA_NOTES" => s_SEA_NOTES(v),
            "SEA_PLACES" => s_SEA_PLACES(v),
            "SMALL" => s_SMALL(v),
            "SOMEWHERE_TO_EAT" => s_SOMEWHERE_TO_EAT(v),
            "SOMEWHERE_TO_STAY" => s_SOMEWHERE_TO_STAY(v),
            "STAY_ADVICE" => s_STAY_ADVICE(v),
            "STAY_THING" => s_STAY_THING(v),
            "STAY_WITHOUT_A_CARD" => s_STAY_WITHOUT_A_CARD(v),
            "STAY_WITHOUT_A_TOWN" => s_STAY_WITHOUT_A_TOWN(v),
            "STOP_AFTER" => s_STOP_AFTER(v),
            "THE_DRIVE" => s_THE_DRIVE(v),
            "THE_FLIGHT" => s_THE_FLIGHT(v),
            "THE_JOURNEY_ASK" => s_THE_JOURNEY_ASK(v),
            "THE_LENGTH_ASK" => s_THE_LENGTH_ASK(v),
            "THE_LENGTH_NOT_SAID" => s_THE_LENGTH_NOT_SAID(v),
            "THE_LENGTH_NOT_SAID_NAMED" => s_THE_LENGTH_NOT_SAID_NAMED(v),
            "THE_MODE" => s_THE_MODE(v),
            "THE_NIGHTS_FIRST" => s_THE_NIGHTS_FIRST(v),
            "TICKETED" => s_TICKETED(v),
            "TIME_WORD" => s_TIME_WORD(v),
            "TO_PLACE" => s_TO_PLACE(v),
            "VISIT_A_FEW_DAYS" => s_VISIT_A_FEW_DAYS(v),
            "VISIT_A_WEEK" => s_VISIT_A_WEEK(v),
            "VISIT_A_WEEKEND" => s_VISIT_A_WEEKEND(v),
            "VISIT_HOW_LONG" => s_VISIT_HOW_LONG(v),
            "VISIT_PACKING" => s_VISIT_PACKING(v),
            "VISIT_PACKING_FOR_A_BABY" => s_VISIT_PACKING_FOR_A_BABY(v),
            "VISIT_PACKING_FOR_A_MEAL" => s_VISIT_PACKING_FOR_A_MEAL(v),
            "WARM" => s_WARM(v),
            "WEEKEND_AWAY" => s_WEEKEND_AWAY(v),
            "WEEKEND_NOTES" => s_WEEKEND_NOTES(v),
            "WEEK_NOTES" => s_WEEK_NOTES(v),
            "WEEK_NOTES_LAND" => s_WEEK_NOTES_LAND(v),
            "WHEN_WORD" => s_WHEN_WORD(v),
            "WINTER_SUN" => s_WINTER_SUN(v),
            "WITH_WHO" => s_WITH_WHO(v),
            "WORD" => s_WORD(v),
            "_COUNT" => s__COUNT(v),
            "_GROUP" => s__GROUP(v),
            "_NOT_A_THING_OF_THEIRS" => s__NOT_A_THING_OF_THEIRS(v),
            "_PLURAL_PEOPLE" => s__PLURAL_PEOPLE(v),
            "last_bot" => s_last_bot(v),
            "plan_line" => s_plan_line(v),
            "user" => s_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.trips", module_obj);
    pyrt::xrefs::reg_fn("memory.trips", "_one_thing_they_need", py_func("_one_thing_they_need", |args: &[Py]| _one_thing_they_need(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_start", py_func("_start", |args: &[Py]| _start(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_low", py_func("_low", |args: &[Py]| _low(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_relation_not_a_place", py_func("_a_relation_not_a_place", |args: &[Py]| _a_relation_not_a_place(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_person_visited", py_func("_a_person_visited", |args: &[Py]| _a_person_visited(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_places", py_func("_places", |args: &[Py]| _places(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_thing_booked", py_func("_the_thing_booked", |args: &[Py]| _the_thing_booked(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_thing_booked_for_when", py_func("_a_thing_booked_for_when", |args: &[Py]| _a_thing_booked_for_when(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_month", py_func("_month", |args: &[Py]| _month(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_when_phrase", py_func("_when_phrase", |args: &[Py]| _when_phrase(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_who", py_func("_who", |args: &[Py]| _who(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_how_long", py_func("_how_long", |args: &[Py]| _how_long(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_how_getting_there", py_func("_how_getting_there", |args: &[Py]| _how_getting_there(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_by_the_sea", py_func("_by_the_sea", |args: &[Py]| _by_the_sea(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_carded_place", py_func("_a_carded_place", |args: &[Py]| _a_carded_place(args)));
    pyrt::xrefs::reg_fn("memory.trips", "a_moved_destination", py_func("a_moved_destination", |args: &[Py]| a_moved_destination(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_last_line_named_an_animal", py_func("_the_last_line_named_an_animal", |args: &[Py]| _the_last_line_named_an_animal(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_keep_the_new_place", py_func("_keep_the_new_place", |args: &[Py]| _keep_the_new_place(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_place_put_right", py_func("_the_place_put_right", |args: &[Py]| _the_place_put_right(args)));
    pyrt::xrefs::reg_fn("memory.trips", "a_trip", py_func("a_trip", |args: &[Py]| a_trip(args)));
    pyrt::xrefs::reg_fn("memory.trips", "the_trip_they_booked", py_func("the_trip_they_booked", |args: &[Py]| the_trip_they_booked(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_trip_so_far", py_func("_the_trip_so_far", |args: &[Py]| _the_trip_so_far(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_asked_last", py_func("_asked_last", |args: &[Py]| _asked_last(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_told", py_func("_told", |args: &[Py]| _told(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_join", py_func("_join", |args: &[Py]| _join(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_with_them", py_func("_with_them", |args: &[Py]| _with_them(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_yours", py_func("_yours", |args: &[Py]| _yours(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_mid", py_func("_mid", |args: &[Py]| _mid(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_cap", py_func("_cap", |args: &[Py]| _cap(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_seasons_we_know", py_func("_seasons_we_know", |args: &[Py]| _seasons_we_know(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_their_month", py_func("_their_month", |args: &[Py]| _their_month(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_in_its_season", py_func("_in_its_season", |args: &[Py]| _in_its_season(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_winter_place", py_func("_a_winter_place", |args: &[Py]| _a_winter_place(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_when_note", py_func("_when_note", |args: &[Py]| _when_note(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_visit_length", py_func("_the_visit_length", |args: &[Py]| _the_visit_length(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_who_note", py_func("_who_note", |args: &[Py]| _who_note(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_note", py_func("_a_note", |args: &[Py]| _a_note(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_known_city", py_func("_a_known_city", |args: &[Py]| _a_known_city(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_areas_of", py_func("_the_areas_of", |args: &[Py]| _the_areas_of(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_somewhere_for", py_func("_somewhere_for", |args: &[Py]| _somewhere_for(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_kind_of_trip", py_func("_the_kind_of_trip", |args: &[Py]| _the_kind_of_trip(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_warm_week_is_out_of_season", py_func("_the_warm_week_is_out_of_season", |args: &[Py]| _the_warm_week_is_out_of_season(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_an_event_not_a_place", py_func("_an_event_not_a_place", |args: &[Py]| _an_event_not_a_place(args)));
    pyrt::xrefs::reg_fn("memory.trips", "a_packing_question_about_the_trip", py_func("a_packing_question_about_the_trip", |args: &[Py]| a_packing_question_about_the_trip(args)));
    pyrt::xrefs::reg_fn("memory.trips", "the_bag_beats_the_list", py_func("the_bag_beats_the_list", |args: &[Py]| the_bag_beats_the_list(args)));
    pyrt::xrefs::reg_fn("memory.trips", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_only_a_length", py_func("_only_a_length", |args: &[Py]| _only_a_length(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_month_answered", py_func("_the_month_answered", |args: &[Py]| _the_month_answered(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_as_a_visit", py_func("_as_a_visit", |args: &[Py]| _as_a_visit(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_new_trip", py_func("_a_new_trip", |args: &[Py]| _a_new_trip(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_length_said", py_func("_a_length_said", |args: &[Py]| _a_length_said(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_the_length_asked", py_func("_the_length_asked", |args: &[Py]| _the_length_asked(args)));
    pyrt::xrefs::reg_fn("memory.trips", "_a_detail", py_func("_a_detail", |args: &[Py]| _a_detail(args)));
    pyrt::xrefs::reg_access("memory.trips", "AN_EVENT_NOT_A_PLACE", g_AN_EVENT_NOT_A_PLACE, s_AN_EVENT_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "ASK_VISIT", g_ASK_VISIT, s_ASK_VISIT);
    pyrt::xrefs::reg_access("memory.trips", "ASK_WHAT", g_ASK_WHAT, s_ASK_WHAT);
    pyrt::xrefs::reg_access("memory.trips", "ASK_WHEN", g_ASK_WHEN, s_ASK_WHEN);
    pyrt::xrefs::reg_access("memory.trips", "ASK_WHEN_NO_QUESTION", g_ASK_WHEN_NO_QUESTION, s_ASK_WHEN_NO_QUESTION);
    pyrt::xrefs::reg_access("memory.trips", "A_BABY", g_A_BABY, s_A_BABY);
    pyrt::xrefs::reg_access("memory.trips", "A_CARING_PLACE_HEAD", g_A_CARING_PLACE_HEAD, s_A_CARING_PLACE_HEAD);
    pyrt::xrefs::reg_access("memory.trips", "A_HAPPY_COURT", g_A_HAPPY_COURT, s_A_HAPPY_COURT);
    pyrt::xrefs::reg_access("memory.trips", "A_MEAL", g_A_MEAL, s_A_MEAL);
    pyrt::xrefs::reg_access("memory.trips", "A_NEW_BABY", g_A_NEW_BABY, s_A_NEW_BABY);
    pyrt::xrefs::reg_access("memory.trips", "A_PERSON_IS_NOT_A_PLACE", g_A_PERSON_IS_NOT_A_PLACE, s_A_PERSON_IS_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "A_POSSESSION_NOT_A_PLACE", g_A_POSSESSION_NOT_A_PLACE, s_A_POSSESSION_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "A_STAY_LENGTH", g_A_STAY_LENGTH, s_A_STAY_LENGTH);
    pyrt::xrefs::reg_access("memory.trips", "A_WHEN_PHRASE", g_A_WHEN_PHRASE, s_A_WHEN_PHRASE);
    pyrt::xrefs::reg_access("memory.trips", "BARE_DAY", g_BARE_DAY, s_BARE_DAY);
    pyrt::xrefs::reg_access("memory.trips", "BARE_WHO", g_BARE_WHO, s_BARE_WHO);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED", g_BOOKED, s_BOOKED);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED_DONE", g_BOOKED_DONE, s_BOOKED_DONE);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED_IN_PLACE", g_BOOKED_IN_PLACE, s_BOOKED_IN_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED_IN_WHEN", g_BOOKED_IN_WHEN, s_BOOKED_IN_WHEN);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED_PLACE", g_BOOKED_PLACE, s_BOOKED_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "BOOKED_THING", g_BOOKED_THING, s_BOOKED_THING);
    pyrt::xrefs::reg_access("memory.trips", "BUSY", g_BUSY, s_BUSY);
    pyrt::xrefs::reg_access("memory.trips", "BUSY_NOTES", g_BUSY_NOTES, s_BUSY_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "BY", g_BY, s_BY);
    pyrt::xrefs::reg_access("memory.trips", "DAYS", g_DAYS, s_DAYS);
    pyrt::xrefs::reg_access("memory.trips", "DAY_NOTES", g_DAY_NOTES, s_DAY_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "DURATION", g_DURATION, s_DURATION);
    pyrt::xrefs::reg_access("memory.trips", "EAT_ADVICE", g_EAT_ADVICE, s_EAT_ADVICE);
    pyrt::xrefs::reg_access("memory.trips", "EAT_WITHOUT_A_CARD", g_EAT_WITHOUT_A_CARD, s_EAT_WITHOUT_A_CARD);
    pyrt::xrefs::reg_access("memory.trips", "EAT_WITHOUT_A_TOWN", g_EAT_WITHOUT_A_TOWN, s_EAT_WITHOUT_A_TOWN);
    pyrt::xrefs::reg_access("memory.trips", "EVERYDAY_TRIP", g_EVERYDAY_TRIP, s_EVERYDAY_TRIP);
    pyrt::xrefs::reg_access("memory.trips", "FOR_HOW_LONG", g_FOR_HOW_LONG, s_FOR_HOW_LONG);
    pyrt::xrefs::reg_access("memory.trips", "GOING", g_GOING, s_GOING);
    pyrt::xrefs::reg_access("memory.trips", "GOING_BARE", g_GOING_BARE, s_GOING_BARE);
    pyrt::xrefs::reg_access("memory.trips", "HOW_LONG_ASK", g_HOW_LONG_ASK, s_HOW_LONG_ASK);
    pyrt::xrefs::reg_access("memory.trips", "INDOOR_NOTES", g_INDOOR_NOTES, s_INDOOR_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "IN_PLACE", g_IN_PLACE, s_IN_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "JOURNEY_MODE", g_JOURNEY_MODE, s_JOURNEY_MODE);
    pyrt::xrefs::reg_access("memory.trips", "JUST_A_DETAIL", g_JUST_A_DETAIL, s_JUST_A_DETAIL);
    pyrt::xrefs::reg_access("memory.trips", "JUST_A_MONTH", g_JUST_A_MONTH, s_JUST_A_MONTH);
    pyrt::xrefs::reg_access("memory.trips", "JUST_HOW_LONG", g_JUST_HOW_LONG, s_JUST_HOW_LONG);
    pyrt::xrefs::reg_access("memory.trips", "KIDS_NOTES", g_KIDS_NOTES, s_KIDS_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "KIDS_NUMBERED", g_KIDS_NUMBERED, s_KIDS_NUMBERED);
    pyrt::xrefs::reg_access("memory.trips", "KIDS_ONE", g_KIDS_ONE, s_KIDS_ONE);
    pyrt::xrefs::reg_access("memory.trips", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("memory.trips", "LAND_NOTES", g_LAND_NOTES, s_LAND_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "LONG_NOTES", g_LONG_NOTES, s_LONG_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.trips", "NOT_AN_AMOUNT", g_NOT_AN_AMOUNT, s_NOT_AN_AMOUNT);
    pyrt::xrefs::reg_access("memory.trips", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "NOT_A_TRIP", g_NOT_A_TRIP, s_NOT_A_TRIP);
    pyrt::xrefs::reg_access("memory.trips", "NOT_PLACES", g_NOT_PLACES, s_NOT_PLACES);
    pyrt::xrefs::reg_access("memory.trips", "NO_THING_NAMED", g_NO_THING_NAMED, s_NO_THING_NAMED);
    pyrt::xrefs::reg_access("memory.trips", "NUMBER_WORD", g_NUMBER_WORD, s_NUMBER_WORD);
    pyrt::xrefs::reg_access("memory.trips", "ONE_THING_NEEDED", g_ONE_THING_NEEDED, s_ONE_THING_NEEDED);
    pyrt::xrefs::reg_access("memory.trips", "ON_A", g_ON_A, s_ON_A);
    pyrt::xrefs::reg_access("memory.trips", "OPEN_AIR", g_OPEN_AIR, s_OPEN_AIR);
    pyrt::xrefs::reg_access("memory.trips", "OUTING", g_OUTING, s_OUTING);
    pyrt::xrefs::reg_access("memory.trips", "OUT_OF_SEASON", g_OUT_OF_SEASON, s_OUT_OF_SEASON);
    pyrt::xrefs::reg_access("memory.trips", "PACKING", g_PACKING, s_PACKING);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_FOR", g_PACKING_FOR, s_PACKING_FOR);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_NOTES", g_PACKING_NOTES, s_PACKING_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_NO_MONTH", g_PACKING_NO_MONTH, s_PACKING_NO_MONTH);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_NO_PLACE", g_PACKING_NO_PLACE, s_PACKING_NO_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_NO_PLACE_NO_MONTH", g_PACKING_NO_PLACE_NO_MONTH, s_PACKING_NO_PLACE_NO_MONTH);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_SAID", g_PACKING_SAID, s_PACKING_SAID);
    pyrt::xrefs::reg_access("memory.trips", "PACKING_WITH_KIDS", g_PACKING_WITH_KIDS, s_PACKING_WITH_KIDS);
    pyrt::xrefs::reg_access("memory.trips", "PEAK", g_PEAK, s_PEAK);
    pyrt::xrefs::reg_access("memory.trips", "PEAK_BOOKED_NOTES", g_PEAK_BOOKED_NOTES, s_PEAK_BOOKED_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "PEAK_NOTES", g_PEAK_NOTES, s_PEAK_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "PLACE_VERBS", g_PLACE_VERBS, s_PLACE_VERBS);
    pyrt::xrefs::reg_access("memory.trips", "PLURAL_RELATIONS", g_PLURAL_RELATIONS, s_PLURAL_RELATIONS);
    pyrt::xrefs::reg_access("memory.trips", "QUIET", g_QUIET, s_QUIET);
    pyrt::xrefs::reg_access("memory.trips", "QUIET_NOTES", g_QUIET_NOTES, s_QUIET_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "SEA", g_SEA, s_SEA);
    pyrt::xrefs::reg_access("memory.trips", "SEASON", g_SEASON, s_SEASON);
    pyrt::xrefs::reg_access("memory.trips", "SEASON_NOTES", g_SEASON_NOTES, s_SEASON_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "SEA_NOTES", g_SEA_NOTES, s_SEA_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "SEA_PLACES", g_SEA_PLACES, s_SEA_PLACES);
    pyrt::xrefs::reg_access("memory.trips", "SMALL", g_SMALL, s_SMALL);
    pyrt::xrefs::reg_access("memory.trips", "SOMEWHERE_TO_EAT", g_SOMEWHERE_TO_EAT, s_SOMEWHERE_TO_EAT);
    pyrt::xrefs::reg_access("memory.trips", "SOMEWHERE_TO_STAY", g_SOMEWHERE_TO_STAY, s_SOMEWHERE_TO_STAY);
    pyrt::xrefs::reg_access("memory.trips", "STAY_ADVICE", g_STAY_ADVICE, s_STAY_ADVICE);
    pyrt::xrefs::reg_access("memory.trips", "STAY_THING", g_STAY_THING, s_STAY_THING);
    pyrt::xrefs::reg_access("memory.trips", "STAY_WITHOUT_A_CARD", g_STAY_WITHOUT_A_CARD, s_STAY_WITHOUT_A_CARD);
    pyrt::xrefs::reg_access("memory.trips", "STAY_WITHOUT_A_TOWN", g_STAY_WITHOUT_A_TOWN, s_STAY_WITHOUT_A_TOWN);
    pyrt::xrefs::reg_access("memory.trips", "STOP_AFTER", g_STOP_AFTER, s_STOP_AFTER);
    pyrt::xrefs::reg_access("memory.trips", "THE_DRIVE", g_THE_DRIVE, s_THE_DRIVE);
    pyrt::xrefs::reg_access("memory.trips", "THE_FLIGHT", g_THE_FLIGHT, s_THE_FLIGHT);
    pyrt::xrefs::reg_access("memory.trips", "THE_JOURNEY_ASK", g_THE_JOURNEY_ASK, s_THE_JOURNEY_ASK);
    pyrt::xrefs::reg_access("memory.trips", "THE_LENGTH_ASK", g_THE_LENGTH_ASK, s_THE_LENGTH_ASK);
    pyrt::xrefs::reg_access("memory.trips", "THE_LENGTH_NOT_SAID", g_THE_LENGTH_NOT_SAID, s_THE_LENGTH_NOT_SAID);
    pyrt::xrefs::reg_access("memory.trips", "THE_LENGTH_NOT_SAID_NAMED", g_THE_LENGTH_NOT_SAID_NAMED, s_THE_LENGTH_NOT_SAID_NAMED);
    pyrt::xrefs::reg_access("memory.trips", "THE_MODE", g_THE_MODE, s_THE_MODE);
    pyrt::xrefs::reg_access("memory.trips", "THE_NIGHTS_FIRST", g_THE_NIGHTS_FIRST, s_THE_NIGHTS_FIRST);
    pyrt::xrefs::reg_access("memory.trips", "TICKETED", g_TICKETED, s_TICKETED);
    pyrt::xrefs::reg_access("memory.trips", "TIME_WORD", g_TIME_WORD, s_TIME_WORD);
    pyrt::xrefs::reg_access("memory.trips", "TO_PLACE", g_TO_PLACE, s_TO_PLACE);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_A_FEW_DAYS", g_VISIT_A_FEW_DAYS, s_VISIT_A_FEW_DAYS);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_A_WEEK", g_VISIT_A_WEEK, s_VISIT_A_WEEK);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_A_WEEKEND", g_VISIT_A_WEEKEND, s_VISIT_A_WEEKEND);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_HOW_LONG", g_VISIT_HOW_LONG, s_VISIT_HOW_LONG);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_PACKING", g_VISIT_PACKING, s_VISIT_PACKING);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_PACKING_FOR_A_BABY", g_VISIT_PACKING_FOR_A_BABY, s_VISIT_PACKING_FOR_A_BABY);
    pyrt::xrefs::reg_access("memory.trips", "VISIT_PACKING_FOR_A_MEAL", g_VISIT_PACKING_FOR_A_MEAL, s_VISIT_PACKING_FOR_A_MEAL);
    pyrt::xrefs::reg_access("memory.trips", "WARM", g_WARM, s_WARM);
    pyrt::xrefs::reg_access("memory.trips", "WEEKEND_AWAY", g_WEEKEND_AWAY, s_WEEKEND_AWAY);
    pyrt::xrefs::reg_access("memory.trips", "WEEKEND_NOTES", g_WEEKEND_NOTES, s_WEEKEND_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "WEEK_NOTES", g_WEEK_NOTES, s_WEEK_NOTES);
    pyrt::xrefs::reg_access("memory.trips", "WEEK_NOTES_LAND", g_WEEK_NOTES_LAND, s_WEEK_NOTES_LAND);
    pyrt::xrefs::reg_access("memory.trips", "WHEN_WORD", g_WHEN_WORD, s_WHEN_WORD);
    pyrt::xrefs::reg_access("memory.trips", "WINTER_SUN", g_WINTER_SUN, s_WINTER_SUN);
    pyrt::xrefs::reg_access("memory.trips", "WITH_WHO", g_WITH_WHO, s_WITH_WHO);
    pyrt::xrefs::reg_access("memory.trips", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.trips", "_COUNT", g__COUNT, s__COUNT);
    pyrt::xrefs::reg_access("memory.trips", "_GROUP", g__GROUP, s__GROUP);
    pyrt::xrefs::reg_access("memory.trips", "_NOT_A_THING_OF_THEIRS", g__NOT_A_THING_OF_THEIRS, s__NOT_A_THING_OF_THEIRS);
    pyrt::xrefs::reg_access("memory.trips", "_PLURAL_PEOPLE", g__PLURAL_PEOPLE, s__PLURAL_PEOPLE);
    pyrt::xrefs::reg_access("memory.trips", "last_bot", g_last_bot, s_last_bot);
    pyrt::xrefs::reg_access("memory.trips", "plan_line", g_plan_line, s_plan_line);
    pyrt::xrefs::reg_access("memory.trips", "user", g_user, s_user);
}

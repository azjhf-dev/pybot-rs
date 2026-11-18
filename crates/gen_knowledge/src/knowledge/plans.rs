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


thread_local! { static G_ANNOUNCED_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOURS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEATHER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_OFF_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVENING_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_WEEKEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLIDAY_PREFIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEDICINE_NEEDS_A_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWBORN_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_FEMALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_MALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMMER_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRY_ONCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRY_ONCE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRY_ONCE_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VAGUE_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKEND_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK_PLAN_SKINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WINTER_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ADULT_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_AT_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMBIGUOUS_CITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_IDEA_ASKED_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_A_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_THE_WORLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AT_THE_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILD_AT_A_MILESTONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MILESTONE_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MILESTONE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MILESTONE_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MOVE_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MOVE_TO_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PLANNED_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PRESENT_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PRESENT_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_THE_PERSON_HAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY_GEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY_GEAR_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY_GEAR_MEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_RID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BATHROOM_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BLOCK_QUALIFIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BRING_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_AT_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COLD_WEATHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECLUTTER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECLUTTER_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECLUTTER_STUFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EVENT_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EVENT_EVENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOR_A_RAINY_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOR_THE_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FREE_BLOCK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIFT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOME_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDEAS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDEAS_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDEA_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDEA_NONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIDS_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KITCHEN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDIUM_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILESTONE_AGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILESTONE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_WITH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOTE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_ACTIVITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PURPOSE_AFTER_RUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REQUESTING_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHORT_RUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SWALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SWALLOWED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_EVENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_WORLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TODAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOO_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIP_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNWELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UTILITY_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_GUESTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_LEAD_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_PAPERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEDDING_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKEND_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_HAPPENED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHERE_GO_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORK_BLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANNOUNCED_DISH() -> Py { ensure_init(); G_ANNOUNCED_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANNOUNCED_DISH(v: Py) { ensure_init(); G_ANNOUNCED_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_HOME() -> Py { ensure_init(); G_AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_HOME(v: Py) { ensure_init(); G_AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_CONDITION() -> Py { ensure_init(); G_A_DAY_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_CONDITION(v: Py) { ensure_init(); G_A_DAY_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOURS_ASK() -> Py { ensure_init(); G_A_HOURS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOURS_ASK(v: Py) { ensure_init(); G_A_HOURS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEATHER_ASK() -> Py { ensure_init(); G_A_WEATHER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEATHER_ASK(v: Py) { ensure_init(); G_A_WEATHER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_OFF_PLAN() -> Py { ensure_init(); G_DAY_OFF_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_OFF_PLAN(v: Py) { ensure_init(); G_DAY_OFF_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_PLAN() -> Py { ensure_init(); G_DAY_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_PLAN(v: Py) { ensure_init(); G_DAY_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EVENING_PLAN() -> Py { ensure_init(); G_EVENING_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVENING_PLAN(v: Py) { ensure_init(); G_EVENING_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME() -> Py { ensure_init(); G_FREE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME(v: Py) { ensure_init(); G_FREE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME_ASK() -> Py { ensure_init(); G_FREE_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME_ASK(v: Py) { ensure_init(); G_FREE_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME_CUE() -> Py { ensure_init(); G_FREE_TIME_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME_CUE(v: Py) { ensure_init(); G_FREE_TIME_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME_TIME() -> Py { ensure_init(); G_FREE_TIME_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME_TIME(v: Py) { ensure_init(); G_FREE_TIME_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME_WHO() -> Py { ensure_init(); G_FREE_TIME_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME_WHO(v: Py) { ensure_init(); G_FREE_TIME_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_TIME_WORDS() -> Py { ensure_init(); G_FREE_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_TIME_WORDS(v: Py) { ensure_init(); G_FREE_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_WEEKEND() -> Py { ensure_init(); G_FREE_WEEKEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_WEEKEND(v: Py) { ensure_init(); G_FREE_WEEKEND.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLIDAY_PREFIX() -> Py { ensure_init(); G_HOLIDAY_PREFIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLIDAY_PREFIX(v: Py) { ensure_init(); G_HOLIDAY_PREFIX.with(|c| *c.borrow_mut() = v); }
pub fn g_MEDICINE_NEEDS_A_REASON() -> Py { ensure_init(); G_MEDICINE_NEEDS_A_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEDICINE_NEEDS_A_REASON(v: Py) { ensure_init(); G_MEDICINE_NEEDS_A_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_WORDS() -> Py { ensure_init(); G_MONEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORDS(v: Py) { ensure_init(); G_MONEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWBORN_WEEK() -> Py { ensure_init(); G_NEWBORN_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWBORN_WEEK(v: Py) { ensure_init(); G_NEWBORN_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MONEY() -> Py { ensure_init(); G_NO_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MONEY(v: Py) { ensure_init(); G_NO_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_FEMALE() -> Py { ensure_init(); G_PEOPLE_FEMALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_FEMALE(v: Py) { ensure_init(); G_PEOPLE_FEMALE.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_MALE() -> Py { ensure_init(); G_PEOPLE_MALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_MALE(v: Py) { ensure_init(); G_PEOPLE_MALE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_ASK() -> Py { ensure_init(); G_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASK(v: Py) { ensure_init(); G_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMMER_WEEK() -> Py { ensure_init(); G_SUMMER_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMMER_WEEK(v: Py) { ensure_init(); G_SUMMER_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_TRY_ONCE() -> Py { ensure_init(); G_TRY_ONCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRY_ONCE(v: Py) { ensure_init(); G_TRY_ONCE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRY_ONCE_ASK() -> Py { ensure_init(); G_TRY_ONCE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRY_ONCE_ASK(v: Py) { ensure_init(); G_TRY_ONCE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TRY_ONCE_MORE() -> Py { ensure_init(); G_TRY_ONCE_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRY_ONCE_MORE(v: Py) { ensure_init(); G_TRY_ONCE_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_VAGUE_TOPICS() -> Py { ensure_init(); G_VAGUE_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VAGUE_TOPICS(v: Py) { ensure_init(); G_VAGUE_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKEND_PLAN() -> Py { ensure_init(); G_WEEKEND_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKEND_PLAN(v: Py) { ensure_init(); G_WEEKEND_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK_OFF() -> Py { ensure_init(); G_WEEK_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK_OFF(v: Py) { ensure_init(); G_WEEK_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK_PLAN() -> Py { ensure_init(); G_WEEK_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK_PLAN(v: Py) { ensure_init(); G_WEEK_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK_PLAN_SKINT() -> Py { ensure_init(); G_WEEK_PLAN_SKINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK_PLAN_SKINT(v: Py) { ensure_init(); G_WEEK_PLAN_SKINT.with(|c| *c.borrow_mut() = v); }
pub fn g_WINTER_WEEK() -> Py { ensure_init(); G_WINTER_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WINTER_WEEK(v: Py) { ensure_init(); G_WINTER_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g__ADULT_PARTY() -> Py { ensure_init(); G__ADULT_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ADULT_PARTY(v: Py) { ensure_init(); G__ADULT_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_AT_PARTY() -> Py { ensure_init(); G__AGE_AT_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_AT_PARTY(v: Py) { ensure_init(); G__AGE_AT_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g__AMBIGUOUS_CITY_WORDS() -> Py { ensure_init(); G__AMBIGUOUS_CITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMBIGUOUS_CITY_WORDS(v: Py) { ensure_init(); G__AMBIGUOUS_CITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_IDEA_ASKED_FOR() -> Py { ensure_init(); G__AN_IDEA_ASKED_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_IDEA_ASKED_FOR(v: Py) { ensure_init(); G__AN_IDEA_ASKED_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_ABOUT_IT() -> Py { ensure_init(); G__ASKS_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_ABOUT_IT(v: Py) { ensure_init(); G__ASKS_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_A_CLOCK() -> Py { ensure_init(); G__ASKS_A_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_A_CLOCK(v: Py) { ensure_init(); G__ASKS_A_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_THE_WORLD() -> Py { ensure_init(); G__ASKS_THE_WORLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_THE_WORLD(v: Py) { ensure_init(); G__ASKS_THE_WORLD.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__AT_HOME() -> Py { ensure_init(); G__AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AT_HOME(v: Py) { ensure_init(); G__AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__AT_THE_PLACE_WORDS() -> Py { ensure_init(); G__AT_THE_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AT_THE_PLACE_WORDS(v: Py) { ensure_init(); G__AT_THE_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILD_AT_A_MILESTONE() -> Py { ensure_init(); G__A_CHILD_AT_A_MILESTONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILD_AT_A_MILESTONE(v: Py) { ensure_init(); G__A_CHILD_AT_A_MILESTONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DATE_ASK() -> Py { ensure_init(); G__A_DATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DATE_ASK(v: Py) { ensure_init(); G__A_DATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DO() -> Py { ensure_init(); G__A_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DO(v: Py) { ensure_init(); G__A_DO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MILESTONE_AGE() -> Py { ensure_init(); G__A_MILESTONE_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MILESTONE_AGE(v: Py) { ensure_init(); G__A_MILESTONE_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MILESTONE_PERSON() -> Py { ensure_init(); G__A_MILESTONE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MILESTONE_PERSON(v: Py) { ensure_init(); G__A_MILESTONE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MILESTONE_SOON() -> Py { ensure_init(); G__A_MILESTONE_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MILESTONE_SOON(v: Py) { ensure_init(); G__A_MILESTONE_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MOVE_ELSEWHERE() -> Py { ensure_init(); G__A_MOVE_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MOVE_ELSEWHERE(v: Py) { ensure_init(); G__A_MOVE_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MOVE_TO_PLAN() -> Py { ensure_init(); G__A_MOVE_TO_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MOVE_TO_PLAN(v: Py) { ensure_init(); G__A_MOVE_TO_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PLANNED_ROOM() -> Py { ensure_init(); G__A_PLANNED_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PLANNED_ROOM(v: Py) { ensure_init(); G__A_PLANNED_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PRESENT_ASKED() -> Py { ensure_init(); G__A_PRESENT_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PRESENT_ASKED(v: Py) { ensure_init(); G__A_PRESENT_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PRESENT_FOR() -> Py { ensure_init(); G__A_PRESENT_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PRESENT_FOR(v: Py) { ensure_init(); G__A_PRESENT_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_THE_PERSON_HAS() -> Py { ensure_init(); G__A_THING_THE_PERSON_HAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_THE_PERSON_HAS(v: Py) { ensure_init(); G__A_THING_THE_PERSON_HAS.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY_GEAR() -> Py { ensure_init(); G__BABY_GEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY_GEAR(v: Py) { ensure_init(); G__BABY_GEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY_GEAR_FAULT() -> Py { ensure_init(); G__BABY_GEAR_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY_GEAR_FAULT(v: Py) { ensure_init(); G__BABY_GEAR_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY_GEAR_MEND() -> Py { ensure_init(); G__BABY_GEAR_MEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY_GEAR_MEND(v: Py) { ensure_init(); G__BABY_GEAR_MEND.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_RID() -> Py { ensure_init(); G__BARE_RID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_RID(v: Py) { ensure_init(); G__BARE_RID.with(|c| *c.borrow_mut() = v); }
pub fn g__BATHROOM_PLAN() -> Py { ensure_init(); G__BATHROOM_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BATHROOM_PLAN(v: Py) { ensure_init(); G__BATHROOM_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__BLOCK_QUALIFIERS() -> Py { ensure_init(); G__BLOCK_QUALIFIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BLOCK_QUALIFIERS(v: Py) { ensure_init(); G__BLOCK_QUALIFIERS.with(|c| *c.borrow_mut() = v); }
pub fn g__BRING_IT() -> Py { ensure_init(); G__BRING_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BRING_IT(v: Py) { ensure_init(); G__BRING_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_AT_PARTY() -> Py { ensure_init(); G__CHILD_AT_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_AT_PARTY(v: Py) { ensure_init(); G__CHILD_AT_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g__COLD() -> Py { ensure_init(); G__COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COLD(v: Py) { ensure_init(); G__COLD.with(|c| *c.borrow_mut() = v); }
pub fn g__COLD_WEATHER() -> Py { ensure_init(); G__COLD_WEATHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COLD_WEATHER(v: Py) { ensure_init(); G__COLD_WEATHER.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DECLUTTER_ASK() -> Py { ensure_init(); G__DECLUTTER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECLUTTER_ASK(v: Py) { ensure_init(); G__DECLUTTER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__DECLUTTER_ROOM() -> Py { ensure_init(); G__DECLUTTER_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECLUTTER_ROOM(v: Py) { ensure_init(); G__DECLUTTER_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g__DECLUTTER_STUFF() -> Py { ensure_init(); G__DECLUTTER_STUFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECLUTTER_STUFF(v: Py) { ensure_init(); G__DECLUTTER_STUFF.with(|c| *c.borrow_mut() = v); }
pub fn g__EVENT_AWAY() -> Py { ensure_init(); G__EVENT_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EVENT_AWAY(v: Py) { ensure_init(); G__EVENT_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__EVENT_EVENING() -> Py { ensure_init(); G__EVENT_EVENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EVENT_EVENING(v: Py) { ensure_init(); G__EVENT_EVENING.with(|c| *c.borrow_mut() = v); }
pub fn g__FOR_A_RAINY_DAY() -> Py { ensure_init(); G__FOR_A_RAINY_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOR_A_RAINY_DAY(v: Py) { ensure_init(); G__FOR_A_RAINY_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__FOR_THE_PLACE_WORDS() -> Py { ensure_init(); G__FOR_THE_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOR_THE_PLACE_WORDS(v: Py) { ensure_init(); G__FOR_THE_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FREE_BLOCK_WORDS() -> Py { ensure_init(); G__FREE_BLOCK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FREE_BLOCK_WORDS(v: Py) { ensure_init(); G__FREE_BLOCK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__GIFT_ASK() -> Py { ensure_init(); G__GIFT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIFT_ASK(v: Py) { ensure_init(); G__GIFT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__HOME_IN_THE_LINE() -> Py { ensure_init(); G__HOME_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOME_IN_THE_LINE(v: Py) { ensure_init(); G__HOME_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__IDEAS_ASK() -> Py { ensure_init(); G__IDEAS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDEAS_ASK(v: Py) { ensure_init(); G__IDEAS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__IDEAS_FOR() -> Py { ensure_init(); G__IDEAS_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDEAS_FOR(v: Py) { ensure_init(); G__IDEAS_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__IDEA_FILLER() -> Py { ensure_init(); G__IDEA_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDEA_FILLER(v: Py) { ensure_init(); G__IDEA_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__IDEA_NONE() -> Py { ensure_init(); G__IDEA_NONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDEA_NONE(v: Py) { ensure_init(); G__IDEA_NONE.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB_NAMED() -> Py { ensure_init(); G__JOB_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB_NAMED(v: Py) { ensure_init(); G__JOB_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB_WORDS() -> Py { ensure_init(); G__JOB_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB_WORDS(v: Py) { ensure_init(); G__JOB_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__KIDS_PARTY() -> Py { ensure_init(); G__KIDS_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIDS_PARTY(v: Py) { ensure_init(); G__KIDS_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g__KITCHEN_PLAN() -> Py { ensure_init(); G__KITCHEN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KITCHEN_PLAN(v: Py) { ensure_init(); G__KITCHEN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDIUM_NAMED() -> Py { ensure_init(); G__MEDIUM_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDIUM_NAMED(v: Py) { ensure_init(); G__MEDIUM_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__MILESTONE_AGES() -> Py { ensure_init(); G__MILESTONE_AGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILESTONE_AGES(v: Py) { ensure_init(); G__MILESTONE_AGES.with(|c| *c.borrow_mut() = v); }
pub fn g__MILESTONE_SAID() -> Py { ensure_init(); G__MILESTONE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILESTONE_SAID(v: Py) { ensure_init(); G__MILESTONE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_ONLY() -> Py { ensure_init(); G__MONEY_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_ONLY(v: Py) { ensure_init(); G__MONEY_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_WITH_IT() -> Py { ensure_init(); G__MONEY_WITH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_WITH_IT(v: Py) { ensure_init(); G__MONEY_WITH_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS() -> Py { ensure_init(); G__MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS(v: Py) { ensure_init(); G__MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSING() -> Py { ensure_init(); G__MUSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSING(v: Py) { ensure_init(); G__MUSING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOTE_ASK() -> Py { ensure_init(); G__NOTE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOTE_ASK(v: Py) { ensure_init(); G__NOTE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_THING() -> Py { ensure_init(); G__NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_THING(v: Py) { ensure_init(); G__NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_SUBJECT() -> Py { ensure_init(); G__NO_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_SUBJECT(v: Py) { ensure_init(); G__NO_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_ACTIVITY() -> Py { ensure_init(); G__OWN_ACTIVITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_ACTIVITY(v: Py) { ensure_init(); G__OWN_ACTIVITY.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_COUNT() -> Py { ensure_init(); G__OWN_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_COUNT(v: Py) { ensure_init(); G__OWN_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE() -> Py { ensure_init(); G__PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE(v: Py) { ensure_init(); G__PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PURPOSE_AFTER_RUN() -> Py { ensure_init(); G__PURPOSE_AFTER_RUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PURPOSE_AFTER_RUN(v: Py) { ensure_init(); G__PURPOSE_AFTER_RUN.with(|c| *c.borrow_mut() = v); }
pub fn g__REQUESTING_TIME() -> Py { ensure_init(); G__REQUESTING_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REQUESTING_TIME(v: Py) { ensure_init(); G__REQUESTING_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__SHORT_RUN() -> Py { ensure_init(); G__SHORT_RUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHORT_RUN(v: Py) { ensure_init(); G__SHORT_RUN.with(|c| *c.borrow_mut() = v); }
pub fn g__SWALLOWED() -> Py { ensure_init(); G__SWALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SWALLOWED(v: Py) { ensure_init(); G__SWALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__SWALLOWED_THING() -> Py { ensure_init(); G__SWALLOWED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SWALLOWED_THING(v: Py) { ensure_init(); G__SWALLOWED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_EVENING() -> Py { ensure_init(); G__THE_EVENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_EVENING(v: Py) { ensure_init(); G__THE_EVENING.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_OWNER() -> Py { ensure_init(); G__THE_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_OWNER(v: Py) { ensure_init(); G__THE_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_WORLD() -> Py { ensure_init(); G__THE_WORLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_WORLD(v: Py) { ensure_init(); G__THE_WORLD.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_WORDS() -> Py { ensure_init(); G__TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_WORDS(v: Py) { ensure_init(); G__TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TODAY_WORDS() -> Py { ensure_init(); G__TODAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TODAY_WORDS(v: Py) { ensure_init(); G__TODAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TOO_MUCH() -> Py { ensure_init(); G__TOO_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOO_MUCH(v: Py) { ensure_init(); G__TOO_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIP_PHRASES() -> Py { ensure_init(); G__TRIP_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIP_PHRASES(v: Py) { ensure_init(); G__TRIP_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__UNWELL() -> Py { ensure_init(); G__UNWELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNWELL(v: Py) { ensure_init(); G__UNWELL.with(|c| *c.borrow_mut() = v); }
pub fn g__UTILITY_PLAN() -> Py { ensure_init(); G__UTILITY_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UTILITY_PLAN(v: Py) { ensure_init(); G__UTILITY_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_ASK() -> Py { ensure_init(); G__WEDDING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_ASK(v: Py) { ensure_init(); G__WEDDING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_COUNT() -> Py { ensure_init(); G__WEDDING_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_COUNT(v: Py) { ensure_init(); G__WEDDING_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_GUESTS() -> Py { ensure_init(); G__WEDDING_GUESTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_GUESTS(v: Py) { ensure_init(); G__WEDDING_GUESTS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_LEAD_TIME() -> Py { ensure_init(); G__WEDDING_LEAD_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_LEAD_TIME(v: Py) { ensure_init(); G__WEDDING_LEAD_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_MONEY() -> Py { ensure_init(); G__WEDDING_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_MONEY(v: Py) { ensure_init(); G__WEDDING_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_PAPERS() -> Py { ensure_init(); G__WEDDING_PAPERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_PAPERS(v: Py) { ensure_init(); G__WEDDING_PAPERS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEDDING_WHEN() -> Py { ensure_init(); G__WEDDING_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEDDING_WHEN(v: Py) { ensure_init(); G__WEDDING_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKEND_WORDS() -> Py { ensure_init(); G__WEEKEND_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKEND_WORDS(v: Py) { ensure_init(); G__WEEKEND_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_HAPPENED() -> Py { ensure_init(); G__WHAT_HAPPENED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_HAPPENED(v: Py) { ensure_init(); G__WHAT_HAPPENED.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN_AWAY() -> Py { ensure_init(); G__WHEN_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN_AWAY(v: Py) { ensure_init(); G__WHEN_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__WHERE_GO_PHRASES() -> Py { ensure_init(); G__WHERE_GO_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHERE_GO_PHRASES(v: Py) { ensure_init(); G__WHERE_GO_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g__WORK_BLOCK() -> Py { ensure_init(); G__WORK_BLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORK_BLOCK(v: Py) { ensure_init(); G__WORK_BLOCK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Plans: the jobs with a date on them, and the order to do them in.\n\nThe answers lane owns this domain. People don't ask about \"project management\", they ask\n\"where do I even start\" about something with a deadline: a house move, a party, a wedding,\na kitchen, a baby, a holiday. The useful answer is always the same shape — what to settle\nfirst, what to do how many weeks out, and the one thing that catches everyone out — so the\nentries here are keyed by the event (moving house, a kids' party, a wedding, a renovation,\na new baby, a holiday, a clear-out, a dinner party, Christmas, a road trip) and the asking\npart of the line (plan, start, organise, how much, what first).\n\nNothing here repeats a domain's own facts: the cooking stays in cooking.py, the packing\nlist in travel.py, and the moving-day van in home.py. This domain only holds the order and\nthe timing, which is what nobody has when they ask.\n");
    s_ANNOUNCED_DISH(crate::knowledge::cooking::g_ANNOUNCED_DISH());
    s_A_HOURS_ASK(crate::knowledge::lookup::g_A_HOURS_ASK());
    s_A_WEATHER_ASK(crate::knowledge::lookup::g_A_WEATHER_ASK());
    s_PEOPLE_FEMALE(crate::knowledge::lookup::g_PEOPLE_FEMALE());
    s_PEOPLE_MALE(crate::knowledge::lookup::g_PEOPLE_MALE());
    s_ANSWERS(py_tuple(vec![py_str("home-and-diy"), py_str("family-and-parenting"), py_str("geography-and-travel")]));
    s_PLAN_ASK(py_tuple(vec![py_str("how"), py_str("what"), py_str("whats"), py_str("what's"), py_str("where"), py_str("when"), py_str("any"), py_str("help"), py_str("should"), py_str("could"), py_str("would"), py_str("can i"), py_str("can you"), py_str("much"), py_str("idea"), py_str("ideas"), py_str("tips"), py_str("advice"), py_str("start"), py_str("best way"), py_str("planning"), py_str("plan"), py_str("plans"), py_str("organising"), py_str("organizing"), py_str("arranging"), py_str("throwing"), py_str("thrown"), py_str("hosting"), py_str("sorting out"), py_str("putting together"), py_str("getting ready for")]));
    s__OWN_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many (?:\\w+ ){0,3}(?:do|does|did) (?:i|we) (?:have|got)\\b|\\bhow many (?:\\w+ ){0,3}have (?:i|we) got\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DATE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|what date|which date|what day)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:party|parties|invitations?|invites?|guests?|venue|plan|plans|planning|organis\\w*|organiz\\w*|menu|buffet|food|catering|decorations?|cake|games|themes?|book|booking)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WHAT_HAPPENED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:did|has|have)\\s+(?:anything|something|much|any good)\\b|\\bwhat(?:'s|s| is)?\\s+happened\\b|\\banything (?:good |nice |interesting )?happen\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS_A_CLOCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat time (?:is|are|was|were|do|does|did|will|would)\\b|\\bwhen (?:is|are|was|were|do|does|did|will) (?!i\\b|we\\b|you\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&crate::knowledge::lookup::g_PEOPLE_MALE(), &crate::knowledge::lookup::g_PEOPLE_FEMALE())) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("he"), py_str("him"), py_str("his"), py_str("she"), py_str("her"), py_str("hers"), py_str("his")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PRESENT_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("buy"), py_str("buys"), py_str("buying"), py_str("bought"), py_str("give"), py_str("gives"), py_str("giving"), py_str("gave")])) { let mut verb: Py = __item.clone(); for __item in py_iter_or_raise(&g__PEOPLE()) { let mut who: Py = __item.clone(); __out.push(op_modulo(&py_str("%s my %s"), &py_tuple(vec![verb.clone(), who.clone()]))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DECLUTTER_ASK(py_tuple(vec![py_str("clear out"), py_str("clearing out"), py_str("clearout"), py_str("clear the clutter"), py_str("clear clutter"), py_str("declutter"), py_str("too much stuff"), py_str("too much clutter"), py_str("fed up with"), py_str("sick of"), py_str("rid of everything"), py_str("rid of it all"), py_str("rid of the lot"), py_str("rid of all of it")]));
    s__DECLUTTER_ROOM(py_tuple(vec![py_str("room"), py_str("rooms"), py_str("house"), py_str("home"), py_str("flat"), py_str("place"), py_str("garage"), py_str("loft"), py_str("shed"), py_str("attic"), py_str("wardrobe"), py_str("wardrobes"), py_str("cupboard"), py_str("cupboards"), py_str("closet"), py_str("kitchen"), py_str("bedroom"), py_str("bedrooms"), py_str("cellar"), py_str("spare room"), py_str("drawer"), py_str("drawers"), py_str("shelves"), py_str("hall"), py_str("hallway"), py_str("landing"), py_str("under the stairs"), py_str("cabinets"), py_str("units"), py_str("car boot"), py_str("box room"), py_str("study"), py_str("desk"), py_str("filing")]));
    s__DECLUTTER_STUFF(py_tuple(vec![py_str("stuff"), py_str("clutter"), py_str("junk"), py_str("rubbish"), py_str("things"), py_str("mess"), py_str("gear"), py_str("bits"), py_str("crap"), py_str("tat"), py_str("odds and ends"), py_str("it all"), py_str("everything")]));
    s__BARE_RID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bget rid of\\s*(?:$|[.?!])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__JOB_NAMED(py_tuple(vec![py_str("clear out"), py_str("clearing out"), py_str("clearout"), py_str("declutter"), py_str("clear the clutter"), py_str("clear clutter"), py_str("big tidy"), py_str("tidy up the house"), py_str("tidy my house"), py_str("tidy up my house"), py_str("sort the house out"), py_str("sort the house")]));
    s__TOO_MUCH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\btoo much (?:stuff|clutter|junk|rubbish|things|mess|gear|crap|tat)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WEDDING_PAPERS(py_tuple(vec![py_str("paperwork"), py_str("notice"), py_str("registrar"), py_str("register office"), py_str("legal"), py_str("admin"), py_str("papers"), py_str("documents"), py_str("bring"), py_str("how do i get married"), py_str("how do we get married"), py_str("how to get married"), py_str("how do you get married"), py_str("how do i marry"), py_str("how do we marry"), py_str("how do i arrange"), py_str("how does it work"), py_str("licence"), py_str("license"), py_str("certificate"), py_str("witness"), py_str("witnesses"), py_str("need to do"), py_str("what do i need"), py_str("what do we need"), py_str("whats needed"), py_str("what's needed"), py_str("need before"), py_str("before we get married"), py_str("getting married what"), py_str("give notice"), py_str("notice to marry"), py_str("give notice to marry"), py_str("to marry")]));
    s__WEDDING_ASK(py_tuple(vec![py_str("notice"), py_str("paperwork"), py_str("registrar"), py_str("register office"), py_str("need to"), py_str("to marry"), py_str("witness"), py_str("witnesses")]));
    s__WEDDING_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how many (?:guests|people|to invite|should we (?:invite|have))|number of guests|how big|guest count|size of the wedding")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WEDDING_WHEN(py_tuple(vec![py_str("far ahead"), py_str("how early"), py_str("how long before"), py_str("lead time"), py_str("how soon"), py_str("when should we book"), py_str("when do we book"), py_str("when to book"), py_str("when should i book"), py_str("book the venue"), py_str("book a venue"), py_str("book the photographer"), py_str("book a photographer"), py_str("book the registrar"), py_str("book the caterer"), py_str("book the church")]));
    s__WEDDING_GUESTS(py_str("There is no right number, only a price: each guest costs roughly £100 to £150 at a catered wedding, so 100 guests is a £10,000 to £15,000 decision. Write two lists - the people you would be sad not to see, then everyone else - and cut from the second until the money works. The room is the other hard limit, because a seated meal needs about 1.2 square metres a head, and children count towards both."));
    s__WEDDING_LEAD_TIME(py_str("Venue first: a year to 18 months ahead for a Saturday in summer, and six months is enough for a winter Friday or a village hall. The registrar or celebrant as soon as the date is fixed, the photographer nine to twelve months out, the caterer six to nine, the cake and the outfit four to six. Ask the venue for its own notice period before you fall for a date, because a barn that has to give notice to the council can block a summer Saturday."));
    s__WEDDING_MONEY(py_tuple(vec![py_str("cost"), py_str("costs"), py_str("much"), py_str("budget"), py_str("cheap"), py_str("cheapest"), py_str("afford"), py_str("price"), py_str("save"), py_str("saving"), py_str("spend"), py_str("quotes"), py_str("quote"), py_str("£")]));
    s_VAGUE_TOPICS(py_tuple(vec![Py::None, py_str(""), py_str("small-talk"), py_str("opinions"), py_str("creative")]));
    s_TRY_ONCE_ASK(py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("should"), py_str("suggest"), py_str("recommend"), py_str("tell"), py_str("give"), py_str("idea"), py_str("ideas"), py_str("anything"), py_str("something"), py_str("one thing"), py_str("surprise"), py_str("want"), py_str("need"), py_str("any"), py_str("name")]));
    s_TRY_ONCE(py_tuple(vec![py_str("Learn to make one loaf of bread by hand: flour, water, yeast, salt, an hour of rising, and the house does the rest. The first one is always a bit wonky and nobody minds."), py_str("Swim outside once - a lake, a lido, the sea in October - with a friend and a towel waiting. Twenty seconds is enough to know you did it."), py_str("See a band you have never heard of in a small room on a week night. A cheap ticket, no expectations, and it is the best way to find music you keep."), py_str("Take one night away on your own: a train, a cheap room, a book. It is a different kind of trip from a holiday with people, and easier than it sounds."), py_str("Watch the sunrise on purpose once: check the time, set an alarm, take a flask. One early start buys you a morning you will talk about for years."), py_str("Learn to change a tyre and check your oil before you have to. Ten minutes of practice on your own drive beats an hour on a hard shoulder in the rain."), py_str("Eat in a restaurant by yourself, with a book, once. Nobody is looking, and it is the most peaceful hour you can buy for the price of a lunch."), py_str("Post a real letter to somebody who will not expect it - a grandparent, an old teacher, a friend who moved away. It lands differently from a message."), py_str("Plant something you can eat and keep it alive to the plate. Herbs on a windowsill count, and one tomato plant teaches you more than a book does."), py_str("Go to a match of a sport you know nothing about and sit with the crowd. Someone will explain the rules before half time, and the noise is the point."), py_str("Give a morning to something local - a food bank, a park clean-up, a repair cafe. One morning with no commitment afterwards, and you will meet people you would never otherwise meet."), py_str("Cook the same dish once a week until you can do it without the recipe. Everyone should own one dinner, and six goes is usually enough."), py_str("Draw something badly on purpose, and keep it. Being a beginner at something for an afternoon is good for you."), py_str("Catch a train to a town an hour away with no plan for the day: walk it, have lunch, come home. The having-no-plan is the good bit."), py_str("Take a class where you make something with your hands - pottery, woodwork, knitting, welding. A thing that stays made is a rare sort of satisfaction."), py_str("Sleep outside once: a tent in a friend's garden, a campsite, a bothy. You will remember the night, not how well you slept."), py_str("Learn ten words of a language before you go somewhere that speaks it. Shopkeepers forgive a lot for one honest attempt."), py_str("Ask an older relative to tell you about their childhood and record it on your phone. It is the cheapest history you will ever collect.")]));
    s_TRY_ONCE_MORE(py_tuple(vec![py_tuple(vec![py_str("another one"), py_str("one more"), py_str("another idea"), py_str("one more idea"), py_str("another thing"), py_str("something else to try"), py_str("anything else to try"), py_str("a different one"), py_str("more ideas"), py_str("give me more"), py_str("another suggestion")]), py_tuple(vec![py_str("give"), py_str("one"), py_str("more"), py_str("another"), py_str("try"), py_str("idea"), py_str("ideas"), py_str("else"), py_str("anything"), py_str("something"), py_str("different")])]));
    s_FREE_TIME(py_tuple(vec![py_str("weekend"), py_str("weekends"), py_str("this weekend"), py_str("next weekend"), py_str("long weekend"), py_str("saturday"), py_str("sundays"), py_str("on saturday"), py_str("on sunday"), py_str("day off"), py_str("bank holiday"), py_str("tonight"), py_str("this evening"), py_str("free evening"), py_str("after work"), py_str("free afternoon"), py_str("home from work"), py_str("get home from work"), py_str("back from work"), py_str("from work"), py_str("when i get home"), py_str("when i get in"), py_str("finish work"), py_str("finished work"), py_str("leave work"), py_str("left work"), py_str("got home from work"), py_str("this afternoon"), py_str("afternoon off"), py_str("free day"), py_str("nothing planned"), py_str("evenings"), py_str("my evenings"), py_str("weekday evenings"), py_str("week off"), py_str("weeks off"), py_str("week off work"), py_str("days off"), py_str("few days off"), py_str("time off"), py_str("time to myself"), py_str("days to myself"), py_str("off next week"), py_str("off for a week"), py_str("off all week"), py_str("whole week off"), py_str("week free"), py_str("a week free"), py_str("week at home"), py_str("week to myself"), py_str("off work for a week"), py_str("off work next week"), py_str("next week off"), py_str("next week off work"), py_str("week off next month"), py_str("a few days off"), py_str("few days off"), py_str("days off next month"), py_str("off for a few days")]));
    s_FREE_TIME_ASK(py_tuple(vec![py_str("what should i do"), py_str("what can i do"), py_str("what do i do"), py_str("what to do"), py_str("any ideas"), py_str("ideas for"), py_str("any suggestions"), py_str("suggestions for"), py_str("something to do"), py_str("anything to do"), py_str("what shall i do"), py_str("where should i go"), py_str("where can i go"), py_str("whats on"), py_str("what's on"), py_str("big plans"), py_str("nothing planned"), py_str("nothing on"), py_str("any tips"), py_str("tips"), py_str("advice"), py_str("suggestions"), py_str("what do you suggest"), py_str("plan it"), py_str("help me plan"), py_str("a few days off"), py_str("few days off"), py_str("days off next month"), py_str("week off next month"), py_str("i have the day off"), py_str("i have a day off"), py_str("have the day off"), py_str("day off on"), py_str("got the day off"), py_str("got a day off"), py_str("day off tomorrow"), py_str("i have a long weekend"), py_str("long weekend free"), py_str("days off next week"), py_str("days off next month"), py_str("days off this week"), py_str("days off at the end of"), py_str("i have days off"), py_str("have days off"), py_str("got days off"), py_str("days holiday"), py_str("days free next week"), py_str("days free next month"), py_str("weekend free"), py_str("i have days off"), py_str("i have the weekend"), py_str("the weekend to myself"), py_str("in the summer"), py_str("in summer"), py_str("in the winter"), py_str("in winter"), py_str("in the spring"), py_str("in spring"), py_str("this summer"), py_str("this winter"), py_str("no plans"), py_str("nothing to do with it"), py_str("use a week off"), py_str("use my week off"), py_str("week off well"), py_str("make the most of"), py_str("plan a week off"), py_str("plan my week off"), py_str("plan the week off"), py_str("spend a week off"), py_str("spend my week off"), py_str("spend the week off"), py_str("how do i use"), py_str("how do i plan"), py_str("how should i spend"), py_str("sort my"), py_str("sort out")]));
    s_DAY_PLAN(py_str("Put two things in the afternoon and give each a time: one that gets you out (a walk you have not done, a park, the library) and one you keep putting off that takes an hour. A free afternoon with no time on it is what dissolves into the phone."));
    s_DAY_OFF_PLAN(py_str("Treat it as a small Saturday: one thing out of the house (a walk you have not done, a market, a museum or gallery - most are free), one job you keep putting off that takes an hour, and one person to see. Put times on the first two the night before; a free day goes in small pieces when nothing is decided."));
    s__THE_EVENING(py_tuple(vec![py_str("tonight"), py_str("this evening"), py_str("free evening"), py_str("after work"), py_str("evening"), py_str("evenings"), py_str("home from work"), py_str("get home from work"), py_str("back from work"), py_str("from work"), py_str("when i get home"), py_str("when i get in"), py_str("finish work"), py_str("finished work"), py_str("leave work"), py_str("left work"), py_str("got home from work")]));
    s_EVENING_PLAN(py_str("Give the evening a shape rather than waiting for an idea: one thing you will actually do at 8pm (a film you have been meaning to watch, a walk before it gets dark, a call to someone you like), tea that takes a bit of effort, and the phone in another room for the first hour. A free evening feels long when nothing is decided and short when one thing is."));
    s_WEEKEND_PLAN(py_str("Three things, one each: something out of the house (a walk you have not done, a park, the library, a free museum), one job you keep putting off but that takes an hour (the drawer, the bike, the photos), and one thing with a person - a call, a walk with someone, a coffee. Put the outdoor one on the better day and keep Sunday evening free, or the weekend ends in a rush and feels like it never happened."));
    s_WEEK_OFF(py_tuple(vec![py_str("week off"), py_str("weeks off"), py_str("week off work"), py_str("time off"), py_str("time to myself"), py_str("days to myself"), py_str("off next week"), py_str("off for a week"), py_str("off all week"), py_str("whole week off"), py_str("week free"), py_str("a week free"), py_str("week at home"), py_str("week to myself"), py_str("off work for a week"), py_str("off for the week"), py_str("off work next week")]));
    s_WEEK_PLAN(py_str("Put a couple of nights away in the middle of the week and keep the first and last days free: somewhere two or three hours away by train is enough, and going Tuesday to Thursday costs less than the weekend. Fill the other days a day at a time - one thing out of the house, one job you keep putting off - and a week off stops slipping past unnoticed."));
    s_SUMMER_WEEK(py_str("In summer, put the outdoors before ten or after five and keep the middle of the day for the job you keep putting off, so the heat sets the day instead of you."));
    s_WINTER_WEEK(py_str("In winter, plan round the dark: the walk or the market at midday, and the cinema, the museum and the long cook for the evenings."));
    s_WEEK_PLAN_SKINT(py_str("Start with the days that cost nothing: the walk you have not done, the library, a market, a park with a flask and a sandwich. Put the one paid thing in the middle of the week, because a cheap train day trip on a Tuesday off-peak is the best value going, and give each day one job out of the house so the week does not go by in small pieces."));
    s_FREE_WEEKEND(py_str("Pick two things and put them in the diary by Friday: one that gets you out of the house - a walk with a destination, a market on Saturday morning, a museum or gallery you keep meaning to visit - and one person to see or one thing waiting for you at home. The free ones that work are the walk, the park, a canal, a hill with a view, the library and a market on a Saturday morning. Two plans makes a good weekend; five makes a list you abandon by Saturday lunchtime. An evening is the same rule: one thing out and one thing in. Tell me the town and who is with you and I can suggest something specific."));
    s_MEDICINE_NEEDS_A_REASON(py_str("What is it for? That decides it, and the same tablet is right for one thing and wrong for the next: paracetamol or ibuprofen at the dose on the pack covers most aches, and a pharmacist will check anything for a stomach ulcer, a kidney problem or another medicine you take. Tell me the symptom and I will say what you can take tonight and what to keep off."));
    s__AN_IDEA_ASKED_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ideas?|suggestions?|things to do|something to do|nothing to do|what can i do|what can we do|what should i do|what should we do|what shall i do|what shall we do|what to do|what else|recommend|recommendations|any good)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SWALLOWED_THING(py_tuple(vec![py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("pills"), py_str("medicine"), py_str("medicines"), py_str("painkiller"), py_str("painkillers"), py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("antibiotic"), py_str("antibiotics"), py_str("antihistamine"), py_str("antihistamines"), py_str("omeprazole"), py_str("dose"), py_str("supplement"), py_str("vitamin"), py_str("vitamins"), py_str("swallow"), py_str("swallows"), py_str("swallowed"), py_str("swallowing")]));
    s__SWALLOWED(py_tuple(vec![py_str("take"), py_str("taking"), py_str("took"), py_str("give"), py_str("giving"), py_str("swallow"), py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("pills"), py_str("medicine"), py_str("medicines"), py_str("painkiller"), py_str("painkillers"), py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("antibiotic"), py_str("antibiotics"), py_str("antihistamine"), py_str("antihistamines"), py_str("omeprazole"), py_str("dose"), py_str("supplement"), py_str("vitamin"), py_str("vitamins")]));
    s__BRING_IT(py_tuple(vec![py_str("picnic"), py_str("picnics"), py_str("bbq"), py_str("bbqs"), py_str("barbecue"), py_str("barbecues"), py_str("barbie"), py_str("party"), py_str("parties"), py_str("dinner party"), py_str("housewarming"), py_str("host"), py_str("hosts"), py_str("hostess"), py_str("birthday"), py_str("wedding"), py_str("buffet"), py_str("round"), py_str("potluck"), py_str("gathering")]));
    s__OWN_ACTIVITY(py_tuple(vec![py_str("weekend"), py_str("weekends"), py_str("saturday"), py_str("sunday"), py_str("day off"), py_str("days off"), py_str("walk"), py_str("walks"), py_str("film"), py_str("films"), py_str("book"), py_str("books"), py_str("cook"), py_str("cooking"), py_str("garden"), py_str("gardening"), py_str("friend"), py_str("friends"), py_str("market"), py_str("museum"), py_str("gallery"), py_str("park"), py_str("library"), py_str("gym"), py_str("game"), py_str("games"), py_str("shop"), py_str("shopping"), py_str("trip"), py_str("away"), py_str("pub"), py_str("coffee"), py_str("music"), py_str("cinema"), py_str("tv"), py_str("series"), py_str("run"), py_str("swim")]));
    s__THE_WORLD(py_tuple(vec![py_str("in the world"), py_str("around the world"), py_str("across the world"), py_str("world news"), py_str("the news"), py_str("the headlines"), py_str("headlines"), py_str("current events"), py_str("in the country")]));
    s__ASKS_THE_WORLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|whats|what's|hows|how's|any|latest|tell|give|show)\\b[^.]{0,24}\\b(?:going on|happening|happened|news|headlines|new)\\b|\\b(?:any|the|todays|today's)\\s+(?:world\\s+)?news\\b|\\bheadlines\\b|\\bcurrent events\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MOVE_TO_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:plan|planning|planned|organise|organize|organising|organizing|prep|prepare|preparing|arrange|arranging|checklist|tips?|advice|start|started|starting|before|ahead|get ready|getting ready|ready)\\b.{0,24}\\b(?:a move|the move|the house move|my move|our move|move out|move in)\\b|\\b(?:a move|the move|my move|our move)\\b.{0,24}\\b(?:plan|planning|organise|organize|checklist|tips?|advice|prepare|prep|arrange)\\b|\\bwhen\\s+(?:i|we|you)\\s+move\\b|\\bwhen\\s+moving\\b|\\bbefore\\s+(?:i|we|you)\\s+move\\b|\\bwe\\s*(?:'|\\s)?re\\s+moving\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MOVE_ELSEWHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:abroad|overseas|emigrat\\w*|visa|another country|different country|new country|australia|canada|new zealand|america|usa|spain|france|portugal|germany|ireland|dubai|team|teams|job|jobs|career|office|house share|houseshare|chess|piece|checkmate|the goalposts|goalposts|in together|moving in with|move in with)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HOME_IN_THE_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:house|houses|home|homes|flat|flats|apartment|apartments)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TODAY_WORDS(py_tuple(vec![py_str("today"), py_str("tonight"), py_str("this afternoon"), py_str("this morning"), py_str("this evening"), py_str("tomorrow")]));
    s__WEEKEND_WORDS(py_tuple(vec![py_str("weekend"), py_str("weekends"), py_str("saturday"), py_str("sunday"), py_str("day off"), py_str("days off"), py_str("week off"), py_str("long weekend")]));
    s_FREE_TIME_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    weekend weekends saturday saturdays sunday sundays friday fridays monday mondays tuesday\n    tuesdays wednesday wednesdays thursday thursdays week weeks weekday weeknight evening\n    evenings afternoon afternoons morning mornings night nights tonight tomorrow today\n    holiday holidays day days off hour hours while time break\n    spring summer autumn winter terms half-term\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREE_TIME_WORDS(op_bitor(&g_FREE_TIME_TIME(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    cheap cheaper cheapest cheaply budget budgets free money cost costs spend spending spent\n    afford much less little broke skint quid pound pounds\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_FREE_TIME_WHO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    i me my we us our you your they them he she him her kid kids child children family families\n    everyone anybody somebody friend friends partner husband wife mum dad son daughter baby\n    toddler\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOLIDAY_PREFIX({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("bank public school"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AT_HOME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at home|at ours|indoors|inside|outdoors|outside|out and about)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREE_TIME_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bnothing (?:much )?planned\\b|\\b(?:no|not much|nothing much) plans?\\b|\\bnothing (?:at all )?(?:to do|going on|on)\\b|\\bnot a thing planned\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DAY_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    cold colder hot hotter warm warmer wet damp raining rainy rain sunny freezing frozen snow\n    snowy icy windy foggy grey gray dark weather forecast degrees bored boring bored stiff\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FOR_A_RAINY_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor a rainy day\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MONEY_WITH_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:save|saves|saving|saved|savings|put aside|aside|fund|funds|money|cash|pounds?|quid|income|salary|afford|afforded|budget|emergency|invest|investing|interest)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_MONEY(py_str("None of that has to cost anything: the walk and the library are free either way, and a flask beats a cafe."));
    s_MONEY_WORDS(py_tuple(vec![py_str("no money"), py_str("not much money"), py_str("dont have money"), py_str("don't have money"), py_str("broke"), py_str("skint"), py_str("no cash"), py_str("on a budget"), py_str("tight budget"), py_str("money is tight"), py_str("cant afford"), py_str("can't afford"), py_str("cheap things"), py_str("free things"), py_str("as little as possible")]));
    s__COLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("((with|have|has|had|got|caught|a|the|my|his|her|their)\\s+cold\\b|\\bcold\\s+(sore|virus|coming on|and a cough|at the moment))")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COLD_WEATHER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("((is it|it is|it's|its|too|so|very|really|freezing|bit|outside|weather|getting|turning|pretty)\\s+(\\w+\\s+)?cold\\b|\\bcold\\s+(today|tomorrow|outside|weather|enough|out)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__UNWELL(py_tuple(vec![py_str("sick"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("flu"), py_str("covid"), py_str("migraine"), py_str("surgery"), py_str("operation"), py_str("hospital"), py_str("injury"), py_str("injured"), py_str("recovering"), py_str("recovery"), py_str("pain"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("ache"), py_str("aching"), py_str("back"), py_str("knee"), py_str("shoulder"), py_str("ankle"), py_str("stomach"), py_str("chest"), py_str("fever"), py_str("temperature"), py_str("dizzy"), py_str("exhausted"), py_str("burnt out")]));
    s_NEWBORN_WEEK(py_str("With a newborn the week plans itself, so plan the small things instead: take the nights in shifts so one of you gets a four-hour stretch, get out once a day even if it is only round the block, and say yes to anyone who offers to hold the baby while you sleep. Register the birth inside 42 days, keep the health visitor's number to hand, and book nothing that has a start time - a week off now is recovery, not a holiday."));
    s__NOTE_ASK(py_tuple(vec![py_str("need a note"), py_str("need the note"), py_str("fit note"), py_str("sick note"), py_str("doctors note"), py_str("doctor's note"), py_str("medical note"), py_str("medical certificate"), py_str("self certificate"), py_str("self-certificate"), py_str("self cert"), py_str("self-certify"), py_str("signed off"), py_str("sign me off"), py_str("getting a note"), py_str("get a note"), py_str("note from the doctor")]));
    s__EVENT_AWAY(py_tuple(vec![py_str("wedding"), py_str("funeral"), py_str("christening"), py_str("baptism"), py_str("moving house"), py_str("moving home"), py_str("house move"), py_str("moving day"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("court"), py_str("interview")]));
    s__WHERE_GO_PHRASES(py_tuple(vec![py_str("where should i go"), py_str("where can i go"), py_str("where to go"), py_str("where should we go"), py_str("where can we go"), py_str("where would you go"), py_str("where could i go"), py_str("where to"), py_str("where would we go"), py_str("where could we go"), py_str("which country"), py_str("what place"), py_str("somewhere warm"), py_str("somewhere hot"), py_str("somewhere near"), py_str("where shall we go"), py_str("where shall i go")]));
    s__AMBIGUOUS_CITY_WORDS(py_tuple(vec![py_str("bath")]));
    s__TRIP_PHRASES(py_tuple(vec![py_str("plan a trip"), py_str("planning a trip"), py_str("planning a holiday"), py_str("book a trip"), py_str("going away"), py_str("go away"), py_str("get away for"), py_str("away for a"), py_str("a trip"), py_str("trip with"), py_str("trip for"), py_str("holiday with"), py_str("travel with"), py_str("a break away"), py_str("book a holiday"), py_str("go somewhere"), py_str("going somewhere"), py_str("somewhere away"), py_str("stay at home"), py_str("staying at home"), py_str("at home or away"), py_str("away or at home"), py_str("weekend away"), py_str("weekends away"), py_str("away for the weekend"), py_str("away for a weekend")]));
    s__WHEN_AWAY(py_tuple(vec![py_str("in january"), py_str("in february"), py_str("in march"), py_str("in april"), py_str("in may"), py_str("in june"), py_str("in july"), py_str("in august"), py_str("in september"), py_str("in october"), py_str("in november"), py_str("in december"), py_str("in the summer"), py_str("in summer"), py_str("in the winter"), py_str("in winter"), py_str("in spring"), py_str("in autumn"), py_str("at christmas"), py_str("over christmas"), py_str("next summer"), py_str("half term"), py_str("bank holiday week"), py_str("shoulder season"), py_str("in the sun"), py_str("for some sun")]));
    s__MONEY_ONLY(py_tuple(vec![py_str("quid"), py_str("pounds"), py_str("euros"), py_str("dollars"), py_str("budget"), py_str("cheap"), py_str("afford")]));
    s__GIFT_ASK(py_tuple(vec![py_str("what should i get"), py_str("what do i get"), py_str("what can i get"), py_str("what to get"), py_str("what should i buy"), py_str("what can i buy"), py_str("what to buy"), py_str("should i get"), py_str("ideas for a present"), py_str("ideas for a gift"), py_str("present ideas"), py_str("gift ideas"), py_str("ideas for presents"), py_str("ideas for the present"), py_str("ideas for her"), py_str("ideas for him"), py_str("present for"), py_str("presents for"), py_str("gift for"), py_str("gifts for"), py_str("buy for"), py_str("get her for"), py_str("get him for"), py_str("get them for"), py_str("get my mum"), py_str("get my dad"), py_str("get my mam"), py_str("get my mom"), py_str("get my mother"), py_str("get my father"), py_str("get my sister"), py_str("get my brother"), py_str("get my nan"), py_str("get my gran"), py_str("get my partner"), py_str("get my wife"), py_str("get my husband"), py_str("get my friend")]));
    s__EVENT_EVENING(py_tuple(vec![py_str("parents evening"), py_str("parent evening"), py_str("school evening"), py_str("open evening"), py_str("induction evening"), py_str("college evening"), py_str("back to school evening"), py_str("parents nights"), py_str("parents night"), py_str("open night")]));
    s__MEDIUM_NAMED(py_tuple(vec![py_str("netflix"), py_str("iplayer"), py_str("prime video"), py_str("disney"), py_str("now tv"), py_str("nowtv"), py_str("britbox"), py_str("paramount"), py_str("all 4"), py_str("apple tv"), py_str("hayu"), py_str("youtube"), py_str("streaming"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("series"), py_str("show"), py_str("shows"), py_str("boxset"), py_str("box set"), py_str("episode"), py_str("episodes"), py_str("documentary"), py_str("documentaries"), py_str("sitcom"), py_str("sitcoms"), py_str("drama series"), py_str("podcast"), py_str("podcasts"), py_str("audiobook"), py_str("audiobooks"), py_str("album"), py_str("playlist"), py_str("novel"), py_str("novels"), py_str("something to read"), py_str("something to watch")]));
    s__MONTHS(py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")]));
    s__WORK_BLOCK(py_tuple(vec![py_str("off work"), py_str("off next week"), py_str("next week off"), py_str("week off work"), py_str("off all week"), py_str("off for a week"), py_str("taking next week off"), py_str("taking the week off"), py_str("the week off"), py_str("off from monday"), py_str("off work for"), py_str("week at home")]));
    s__TIME_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tonight today tomorrow evening evenings afternoon afternoons morning mornings night nights weekend weekends week weeks weekday weekdays day days month months year years saturday sunday monday tuesday wednesday thursday friday summer winter spring autumn christmas easter holiday holidays time break when once whenever after before until till while"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NO_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("things thing something anything stuff more other else"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IDEA_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the any some more other no not few couple another new better good great best and or of on for to up out off my your our their his her its i we you they have has had got need needs want wants running stuck fresh different"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IDEAS_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bideas?\\s+(?:for|about|on)\\s+(.+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IDEAS_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bideas?\\s+(?:what|which|who|where|how|when)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IDEA_NONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(no|not|never|without|dont|don't|doesnt|doesn't|havent|haven't)\\s+(?:idea|ideas)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_THING_THE_PERSON_HAS(py_tuple(vec![py_str("health"), py_str("fitness"), py_str("sleep")]));
    s__ASKS_ABOUT_IT(py_tuple(vec![py_str("what can i do"), py_str("what should i do"), py_str("what do i do"), py_str("what to do"), py_str("what shall i do"), py_str("what can we do"), py_str("what should we do"), py_str("how do i"), py_str("how can i"), py_str("what helps"), py_str("what would help"), py_str("what can i take"), py_str("can i take"), py_str("anything i can"), py_str("anything to take"), py_str("anything i can take")]));
    s__ASKS(py_tuple(vec![py_str("what should i do"), py_str("what can i do"), py_str("what do i do"), py_str("what to do"), py_str("what shall i do"), py_str("any ideas"), py_str("ideas"), py_str("idea"), py_str("anything to do"), py_str("something to do"), py_str("suggestions"), py_str("suggestion"), py_str("tips"), py_str("tip"), py_str("advice"), py_str("nothing planned"), py_str("nothing on"), py_str("no idea"), py_str("bored"), py_str("plan it"), py_str("help me plan"), py_str("plan something"), py_str("whats on"), py_str("what's on"), py_str("big plans"), py_str("recommend"), py_str("how do i fill"), py_str("fill the time"), py_str("keep busy"), py_str("keep me busy")]));
    s__SHORT_RUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+|one|two|three|four|five|six)\\s+(?:whole\\s+|full\\s+|more\\s+)?days?\\s+(?:off|free|holiday|to myself|at home)\\b|\\b(?:a\\s+)?(?:few|couple\\s+of|handful\\s+of)\\s+days?\\s+(?:off|free|to myself)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__PURPOSE_AFTER_RUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:to\\s+[a-z]+|for\\s+(?:the|a|an|my|our|his|her|their|some)\\b|so\\s+(?:i|we)\\s+can\\b|because\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__REQUESTING_TIME(py_tuple(vec![py_str("book a day off"), py_str("book a week off"), py_str("book time off"), py_str("book some time off"), py_str("book the day off"), py_str("book it off"), py_str("take a day off"), py_str("take a week off"), py_str("take time off"), py_str("take some time off"), py_str("take the day off"), py_str("take it off"), py_str("request a day off"), py_str("request a week off"), py_str("request time off"), py_str("ask for a day off"), py_str("ask for a week off"), py_str("ask for time off"), py_str("ask my boss"), py_str("ask the boss"), py_str("ask my manager"), py_str("put in for"), py_str("want to book"), py_str("want to take a day"), py_str("want to take a week"), py_str("can i book"), py_str("can i take a day"), py_str("can i take a week"), py_str("could i take"), py_str("could i book"), py_str("am i allowed to take")]));
    s__FREE_BLOCK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("weekend"), py_str("weekends"), py_str("saturday"), py_str("sundays"), py_str("sunday"), py_str("day"), py_str("days"), py_str("evening"), py_str("evenings"), py_str("night"), py_str("nights"), py_str("tonight"), py_str("afternoon"), py_str("morning"), py_str("holiday"), py_str("holidays"), py_str("week"), py_str("weeks"), py_str("time"), py_str("leave"), py_str("rest"), py_str("break"), py_str("free"), py_str("off"), py_str("plans"), py_str("plan"), py_str("work"), py_str("tomorrow"), py_str("today"), py_str("now"), py_str("later"), py_str("nothing"), py_str("home"), py_str("myself"), py_str("ourselves"), py_str("me"), py_str("us"), py_str("family"), py_str("friends"), py_str("monday"), py_str("mondays"), py_str("tuesday"), py_str("tuesdays"), py_str("wednesday"), py_str("wednesdays"), py_str("thursday"), py_str("thursdays"), py_str("friday"), py_str("fridays"), py_str("saturday"), py_str("weekday"), py_str("weekdays"), py_str("weeknight"), py_str("rainy"), py_str("wet"), py_str("cold"), py_str("indoors"), py_str("outside"), py_str("sunny"), py_str("summer"), py_str("winter"), py_str("autumn"), py_str("spring"), py_str("hot")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("which"), py_str("where"), py_str("when"), py_str("how"), py_str("why"), py_str("who"), py_str("should"), py_str("shall"), py_str("can"), py_str("could"), py_str("would"), py_str("will"), py_str("do"), py_str("does"), py_str("did"), py_str("doing"), py_str("i"), py_str("we"), py_str("you"), py_str("id"), py_str("im"), py_str("ive"), py_str("us"), py_str("me"), py_str("my"), py_str("our"), py_str("your"), py_str("the"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("next"), py_str("last"), py_str("coming"), py_str("some"), py_str("any"), py_str("something"), py_str("anything"), py_str("nothing"), py_str("thing"), py_str("things"), py_str("stuff"), py_str("ideas"), py_str("idea"), py_str("suggestions"), py_str("suggestion"), py_str("suggest"), py_str("recommend"), py_str("tips"), py_str("tip"), py_str("advice"), py_str("advise"), py_str("good"), py_str("nice"), py_str("fun"), py_str("funny"), py_str("best"), py_str("better"), py_str("on"), py_str("in"), py_str("at"), py_str("for"), py_str("to"), py_str("of"), py_str("and"), py_str("or"), py_str("but"), py_str("with"), py_str("without"), py_str("around"), py_str("about"), py_str("out"), py_str("up"), py_str("there"), py_str("here"), py_str("together"), py_str("nearby"), py_str("local"), py_str("all"), py_str("day"), py_str("whole"), py_str("long"), py_str("few"), py_str("couple"), py_str("off"), py_str("free"), py_str("bored"), py_str("nothing"), py_str("fill"), py_str("busy"), py_str("keep"), py_str("make"), py_str("most"), py_str("use"), py_str("spend"), py_str("sort"), py_str("go"), py_str("going"), py_str("somewhere"), py_str("anywhere"), py_str("nowhere"), py_str("else"), py_str("instead"), py_str("please"), py_str("help"), py_str("planned"), py_str("plans"), py_str("plan"), py_str("properly"), py_str("proper"), py_str("decent"), py_str("little"), py_str("bit"), py_str("much"), py_str("spend"), py_str("spending"), py_str("spent"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("hi"), py_str("hiya"), py_str("hey"), py_str("hello"), py_str("yo"), py_str("ok"), py_str("okay"), py_str("so"), py_str("well"), py_str("just"), py_str("really"), py_str("actually"), py_str("still")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__JOB_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("sort"), py_str("sorted"), py_str("sorting"), py_str("tidy"), py_str("tidying"), py_str("clear"), py_str("clearing"), py_str("clean"), py_str("cleaning"), py_str("declutter"), py_str("decluttering"), py_str("pack"), py_str("packing"), py_str("unpack"), py_str("unpacking"), py_str("paint"), py_str("painting"), py_str("fix"), py_str("fixing"), py_str("mend"), py_str("mending"), py_str("repair"), py_str("repairing"), py_str("decorate"), py_str("decorating"), py_str("redecorate"), py_str("reorganise"), py_str("reorganising"), py_str("organise"), py_str("organising"), py_str("rearrange"), py_str("hoover"), py_str("weed"), py_str("weeding"), py_str("tackle"), py_str("tackling"), py_str("file"), py_str("filing"), py_str("paperwork")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COUNT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    one two three four five six seven eight nine ten eleven twelve dozen couple few several\n    half single both\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("not"), py_str("isnt"), py_str("isn't"), py_str("arent"), py_str("aren't"), py_str("wasnt"), py_str("wasn't"), py_str("no"), py_str("without"), py_str("non"), py_str("except"), py_str("besides"), py_str("bar"), py_str("minus"), py_str("other"), py_str("than"), py_str("rather"), py_str("instead"), py_str("nor"), py_str("never")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BLOCK_QUALIFIERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("cheap"), py_str("free"), py_str("budget"), py_str("afford"), py_str("money"), py_str("quid"), py_str("pounds"), py_str("tenner"), py_str("fiver"), py_str("spare"), py_str("skint"), py_str("pennies"), py_str("cash"), py_str("nice"), py_str("quiet"), py_str("lazy"), py_str("relaxing"), py_str("broke"), py_str("broken"), py_str("poor"), py_str("brassic"), py_str("skint"), py_str("tight"), py_str("moneyless"), py_str("bust")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AT_HOME(py_tuple(vec![py_str("kitchen"), py_str("house"), py_str("flat"), py_str("home"), py_str("loft"), py_str("attic"), py_str("cellar"), py_str("living room"), py_str("dining room"), py_str("bedroom"), py_str("bathroom"), py_str("spare room"), py_str("conservatory"), py_str("study"), py_str("front room"), py_str("back room"), py_str("box room"), py_str("utility room"), py_str("hall"), py_str("hallway"), py_str("landing"), py_str("porch")]));
    s__AT_THE_PLACE_WORDS(py_tuple(vec![py_str("in"), py_str("at"), py_str("on"), py_str("into"), py_str("inside"), py_str("around"), py_str("round")]));
    s__FOR_THE_PLACE_WORDS(py_tuple(vec![py_str("for")]));
    s__THE_OWNER(py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their")]));
    s__MUSING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:suppose|guess|reckon|expect|think)\\b|\\b(?:maybe|perhaps|probably)\\b[^.?!]*\\b(?:i|we)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BABY_GEAR(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("pram"), py_str("prams"), py_str("pushchair"), py_str("pushchairs"), py_str("buggy"), py_str("buggies"), py_str("stroller"), py_str("strollers"), py_str("travel system")]), py_str("pram"), py_str("A pram is £150 to £400 new and £60 to £150 second-hand, and what matters is that it folds into the car and lies flat for a newborn. A second-hand one is fine if the frame is straight, the brakes hold and the fabric washes.")]), py_tuple(vec![py_tuple(vec![py_str("carrier"), py_str("carriers"), py_str("sling"), py_str("slings"), py_str("baby carrier"), py_str("baby sling")]), py_str("carrier"), py_str("A carrier or sling is £30 to £90, and it is the easy one on a bus, on stairs and in the first weeks; a pram carries the shopping and the sleeping baby. Most people end up with both, a sling first and a pram from about three months.")]), py_tuple(vec![py_tuple(vec![py_str("cot"), py_str("cots"), py_str("cot bed"), py_str("cotbed"), py_str("crib"), py_str("cribs"), py_str("moses basket"), py_str("travel cot")]), py_str("cot"), py_str("A cot is £100 to £250 new and £30 to £80 second-hand. The cot itself can be second-hand, but the mattress has to be new and firm and fit with no gaps round the edge, 120 by 60cm for a cot and 140 by 70cm for a cot bed. A Moses basket is for the first months only.")]), py_tuple(vec![py_tuple(vec![py_str("car seat"), py_str("car seats"), py_str("carseat"), py_str("carseats"), py_str("i-size"), py_str("isofix"), py_str("infant carrier")]), py_str("car seat"), py_str("A car seat is £80 to £250 new and it is the one piece not to buy second-hand unless you know its history: a seat that has been in a crash may look fine and still be spent. i-Size (R129) is the newer standard, and how it fits your car matters more than the brand.")]), py_tuple(vec![py_tuple(vec![py_str("high chair"), py_str("highchair"), py_str("high chairs")]), py_str("high chair"), py_str("A high chair is £25 to £150 and it can wait until six months, so it is the one piece you leave: a £25 one that wipes clean and has a footrest does the job.")]), py_tuple(vec![py_tuple(vec![py_str("baby monitor"), py_str("monitor"), py_str("monitors")]), py_str("baby monitor"), py_str("A baby monitor is £30 to £120 and it is a comfort rather than a must: it does not stop anything happening, so a cheap audio one beside the cot is enough.")])]));
    s__BABY_GEAR_FAULT(py_tuple(vec![py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("squeaky"), py_str("sticking"), py_str("stuck"), py_str("jammed"), py_str("jam"), py_str("wobbly"), py_str("loose"), py_str("broken"), py_str("broke"), py_str("snapped"), py_str("wont fold"), py_str("won't fold"), py_str("wont push"), py_str("won't push"), py_str("dragging"), py_str("rusty"), py_str("rust"), py_str("cracked"), py_str("torn"), py_str("rips"), py_str("ripped"), py_str("fraying"), py_str("buckle"), py_str("buckles"), py_str("harness"), py_str("brake"), py_str("brakes"), py_str("wheel"), py_str("wheels"), py_str("axle"), py_str("axles"), py_str("handle")]));
    s__BABY_GEAR_MEND(py_str("Squeaking or sticking wheels are the axle: a drop of silicone spray or a dry PTFE lubricant on each axle and the grit wiped off the wheels, and a brake that will not hold wants its little spring or cable adjusting. A wobbly handle is a bolt inside the frame to tighten, a torn hood or seat is a replacement cover from the maker, and a bent frame is the one fault that means a new pram rather than a mending job."));
    s__ADULT_PARTY(py_str("Count the people first, because it decides the room and the food: 8 to 12 is easy to cook for at home, 30 to 40 is about as many as a house takes, and a pub's back room or a restaurant takes 20 to 60 for a similar price to a hall. Pick the day and send one message with the date, time, place and whether it is a meal or drinks, because \"let me know if you can make it\" gets half-answers. Keep the food to things made the day before - a pie or a chilli kept warm, a baked ham, bread, two salads and a bought cake - and give two people one job each, drinks and cards and the speech, so you are not doing all of it. Send the invitations three to four weeks ahead, six for a big do or a date near Christmas, and ask for replies by a date a week before. Budget £10 to £15 a head for a buffet at home and £25 to £40 in a restaurant's back room, drink on top."));
    s__KIDS_PARTY(py_str("Two hours and about age-plus-one guests is the sweet spot: one venue or one big activity, food that can be made the day before, and a bag for the way home. At home it costs around £5 a head but you need a wet-weather plan and one adult per four children; a hall or soft-play party at £10 to £15 a head buys you both."));
    s__CHILD_AT_PARTY(py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("childs"), py_str("toddler"), py_str("toddlers"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("grandson"), py_str("granddaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("godson"), py_str("goddaughter"), py_str("school"), py_str("nursery"), py_str("soft play"), py_str("playcentre"), py_str("reception class"), py_str("year one")]));
    s__AGE_AT_PARTY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\d{1,2})(?:st|nd|rd|th)?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MILESTONE_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mum|mums|mom|moms|mam|mams|dad|dads|father|mother|nan|nans|nana|nan's|gran|grans|granny|grandma|grandad|grandpa|grandmother|grandfather|aunt|auntie|uncle|sister|brother|wife|husband|partner|mother in law|father in law|she|he|son|sons|daughter|daughters|grandson|granddaughter|grandchild|godson|goddaughter|nephew|niece|kid|kids|child|children)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CHILD_AT_A_MILESTONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:son|sons|daughter|daughters|grandson|granddaughter|grandchild|godson|goddaughter|nephew|niece|kid|kids|child|children|boy|girl|toddler)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MILESTONE_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|turns|turning|will be|going to be|hits|becomes)\\s+(?:the age of\\s+)?(\\d{1,2})\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MILESTONE_SOON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:next (?:week|month|weekend|saturday|sunday|friday|year)|this (?:week|weekend)|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|soon|coming up|in \\w+ (?:weeks|days))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PRESENT_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:present|presents|gift|gifts)\\b|\\bwhat\\s+(?:should|shall|can|could|do|would)\\s+(?:i|we)\\s+(?:get|buy|give|make|send)\\b|\\b(?:get|buy|give|make|send)\\s+(?:her|him|them|my\\s+(?:mum|mom|mother|dad|father|gran|grandma|grandad|granny|nan|nana|wife|husband|partner|son|daughter|sister|brother|friend|parents?))\\b|\\bideas?\\s+(?:for|of)\\s+(?:a\\s+)?(?:present|gift)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MILESTONE_AGES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); __args.push(Py::Int(100)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MILESTONE_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MILESTONE_AGES()) { let mut years: Py = __item.clone(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("is"), py_str("turns"), py_str("turning"), py_str("will be")])) { let mut verb: Py = __item.clone(); __out.push(op_modulo(&py_str("%s %d"), &py_tuple(vec![verb.clone(), years.clone()]))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__KITCHEN_PLAN(py_str("Plan a kitchen round the three points that get used most - the sink, the hob and the fridge: any two of them want 1.2 to 2.7 metres between them, the sink wants a clear 600mm of worktop on each side, and the hob wants space to put a pan down on the right and nothing flammable beside it. Then list what you need before what you want: deep drawers beat a bottom cupboard, a socket every metre of worktop, a tall cupboard for the ironing board and the hoover. Mark where the pipes, the soil stack and the door swing are before you fall in love with a layout, because moving a drain is most of a kitchen's cost. Tape the outline on the floor and walk it for a week before you order."));
    s__BATHROOM_PLAN(py_str("Plan a bathroom round the soil pipe: the toilet has to stay where the drain already is unless you raise the floor for a new one, so put the toilet and the bath first and the basin where there is wall for it. A bath needs 1.7m and a shower 900mm square, and leave 700mm clear in front of the basin and 600mm beside the toilet. Decide the shower before the tiles, because a wet room floor has to fall to a drain and a screen cannot be added later, and set the shower valve about 1.1m off the floor so nobody has to reach. A bathroom also needs an extractor that moves about 15 litres a second, or the paint and the grout will go."));
    s__UTILITY_PLAN(py_str("Plan a utility room round the machines and the water: the washing machine and the dryer want an outside wall for the vent and the plumbing, a sink close to the drain, and a worktop over the appliances at about 900mm so there is a surface to fold on. Leave 600mm clear in front of every door you have to open, put the boiler behind a cupboard you can still get into, and keep a shelf at eye height for powder, pegs and the iron - it is the walk-through clutter, not the size of the room, that makes a utility room useless."));
    s__A_PLANNED_ROOM(py_tuple(vec![py_str("kitchen"), py_str("kitchens"), py_str("bathroom"), py_str("bathrooms"), py_str("bath"), py_str("baths"), py_str("shower"), py_str("showers"), py_str("toilet"), py_str("toilets"), py_str("loo"), py_str("loos"), py_str("ensuite"), py_str("en suite"), py_str("wet room"), py_str("utility"), py_str("utility room"), py_str("laundry"), py_str("laundry room"), py_str("boot room"), py_str("small room"), py_str("spare room"), py_str("dining room")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("free-time-plan")); __args.push(py_tuple(vec![g_FREE_TIME(), py_tuple(vec![py_str("do"), py_str("doing"), py_str("go"), py_str("going"), py_str("ideas"), py_str("idea"), py_str("suggestions"), py_str("something"), py_str("anything"), py_str("plan"), py_str("plans"), py_str("bored"), py_str("nothing"), py_str("saturday"), py_str("sunday"), py_str("tips"), py_str("tip"), py_str("advice"), py_str("suggest"), py_str("days off"), py_str("day off"), py_str("time off"), py_str("off work"), py_str("weekend free"), py_str("long weekend"), py_str("weekend to myself"), py_str("summer"), py_str("winter"), py_str("spring"), py_str("autumn"), py_str("no plans"), py_str("well"), py_str("use"), py_str("spend"), py_str("sort")])])); __args.push(g__free_time()); __kws.extend(vec![("ask".to_string(), g_FREE_TIME_ASK()), ("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__a_stretch_of_time_to_fill()), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("dinner"), py_str("tea"), py_str("lunch"), py_str("breakfast"), py_str("bake"), py_str("shop"), py_str("shopping"), py_str("supermarket"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("car"), py_str("mot"), py_str("van"), py_str("garage"), py_str("boiler"), py_str("washing machine"), py_str("essay"), py_str("coursework"), py_str("assignment"), py_str("deadline"), py_str("tax return"), py_str("invoice"), py_str("homework"), py_str("revise"), py_str("revision"), py_str("exam"), py_str("at work"), py_str("office"), py_str("shift"), py_str("overtime"), py_str("rota"), py_str("weather"), py_str("rain"), py_str("forecast"), py_str("train"), py_str("bus"), py_str("flight"), py_str("traffic"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("pain"), py_str("toothache"), py_str("tooth"), py_str("hurt"), py_str("hurts"), py_str("ill"), py_str("sick"), py_str("fever"), py_str("temperature"), py_str("rash"), py_str("dizzy"), py_str("dentist"), py_str("doctor"), py_str("pharmacy"), py_str("hospital"), py_str("symptom"), py_str("symptoms"), py_str("headache"), py_str("left"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("use up"), py_str("using up"), py_str("with it"), py_str("with them"), py_str("with these"), py_str("goes off"), py_str("go off"), py_str("out of date"), py_str("there"), py_str("trip"), py_str("break away"), py_str("visit"), py_str("visiting"), py_str("hotel"), py_str("city"), py_str("abroad"), py_str("travel"), py_str("flights"), py_str("in rome"), py_str("in paris"), py_str("in london"), py_str("in new york"), py_str("weekend away"), py_str("weekend break"), py_str("city break"), py_str("city breaks"), py_str("night away"), py_str("nights away"), py_str("mini break"), py_str("short break"), py_str("getaway"), py_str("days away"), py_str("couple of nights"), py_str("few nights away"), py_str("two nights away"), py_str("weekend in"), py_str("feel low"), py_str("feeling low"), py_str("feel really low"), py_str("feeling really low"), py_str("feel down"), py_str("feeling down"), py_str("feel really down"), py_str("feeling really down"), py_str("feel sad"), py_str("feeling sad"), py_str("feel depressed"), py_str("feeling depressed"), py_str("depressed"), py_str("feel anxious"), py_str("feeling anxious"), py_str("anxious"), py_str("feel lonely"), py_str("feeling lonely"), py_str("lonely"), py_str("feel miserable"), py_str("feeling miserable"), py_str("miserable"), py_str("feel rubbish"), py_str("feeling rubbish"), py_str("feel awful"), py_str("feeling awful"), py_str("feel terrible"), py_str("feeling terrible"), py_str("feel like crying"), py_str("cant be bothered"), py_str("no energy"), py_str("hate everything"), py_str("can i have"), py_str("can i take"), py_str("can i book"), py_str("can i get"), py_str("could i have"), py_str("could i take"), py_str("may i have"), py_str("may i take"), py_str("am i allowed"), py_str("is it ok if i"), py_str("is it okay if i"), py_str("do i have to book"), py_str("my boss"), py_str("my manager"), py_str("ask for it off"), py_str("take a day off"), py_str("take a week off"), py_str("take time off"), py_str("take some time off"), py_str("book a day off"), py_str("book a week off"), py_str("book time off"), py_str("want to take"), py_str("want to book"), py_str("request a day off"), py_str("request a week off"), py_str("request time off"), py_str("put in for"), py_str("hoping to take"), py_str("hoping to book")]), &crate::knowledge::lookup::g_A_HOURS_ASK()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("move-house-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("moving house"), py_str("moving home"), py_str("moving flat"), py_str("moving out"), py_str("house move"), py_str("move house"), py_str("move flat"), py_str("moving next month"), py_str("packing up the house"), py_str("move to a new house"), py_str("move to a new flat"), py_str("move into a new house"), py_str("move into a new flat"), py_str("move into my first"), py_str("how do i move house"), py_str("how do i move home"), py_str("how do i move flat"), py_str("how do i move out"), py_str("how to move house"), py_str("how to move home"), py_str("how do i move to a new house"), py_str("how do i move to a new flat"), py_str("what do i need to move house"), py_str("what do i need to move home"), py_str("when moving"), py_str("when we move"), py_str("when i move"), py_str("when you move"), py_str("if we move"), py_str("plan a move"), py_str("planning a move"), py_str("plan the move"), py_str("planning the move"), py_str("organise a move"), py_str("organising a move"), py_str("organize a move"), py_str("organizing a move"), py_str("a move to plan"), py_str("before a move"), py_str("before the move"), py_str("the move go"), py_str("prepare for a move"), py_str("prepare to move"), py_str("get ready for a move"), py_str("get ready to move"), py_str("ready for a move"), py_str("start with a move"), py_str("start on a move"), py_str("start with moving"), py_str("sort out a move"), py_str("moving out of my"), py_str("moving out next")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("start"), py_str("organise"), py_str("organize"), py_str("checklist"), py_str("prepare"), py_str("sorted"), py_str("first"), py_str("ahead"), py_str("organise"), py_str("where do i"), py_str("tips"), py_str("advice"), py_str("move"), py_str("moving"), py_str("moved")])])); __args.push(py_str("Work back from the date: book a van or removals six to eight weeks out, and tell the council, energy, water and broadband a month before, because the bills follow you. Box the things you don't use three weeks out, label every box with its room, and pack one bag with a kettle, mugs, loo roll, bedding and chargers for the first night.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("guard".to_string(), g__a_household_move_or_a_named_move()), ("avoid".to_string(), py_tuple(vec![py_str("council"), py_str("doctor"), py_str("gp"), py_str("surgery"), py_str("dentist"), py_str("school"), py_str("register"), py_str("council tax"), py_str("notify"), py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("cheaply"), py_str("cost"), py_str("costs"), py_str("price"), py_str("how much"), py_str("van"), py_str("removals"), py_str("man with a van"), py_str("hoping"), py_str("hopin"), py_str("hope to move"), py_str("hopin to move"), py_str("hopes to"), py_str("thinking of moving"), py_str("thinking about moving"), py_str("dream of moving"), py_str("dreaming of moving"), py_str("want to move out"), py_str("cant wait to move"), py_str("tell my landlord"), py_str("tell the landlord"), py_str("landlord"), py_str("landlady"), py_str("letting agent"), py_str("notice period"), py_str("give notice"), py_str("can't wait to move"), py_str("one day"), py_str("my back"), py_str("back is killing"), py_str("back hurts"), py_str("bad back"), py_str("sore back"), py_str("pulled my back"), py_str("hurt my back"), py_str("done my back"), py_str("pack"), py_str("packing"), py_str("packed"), py_str("boxes"), py_str("box up"), py_str("label"), py_str("labels"), py_str("bubble wrap"), py_str("what to pack"), py_str("declutter"), py_str("unpack"), py_str("how do i tell"), py_str("how to tell"), py_str("how do i explain"), py_str("what do i say"), py_str("tell the kids"), py_str("tell my kids"), py_str("tell the children"), py_str("tell my children"), py_str("tell my son"), py_str("tell my daughter"), py_str("break the news"), py_str("explain to my"), py_str("how do i break"), py_str("with kids"), py_str("with children"), py_str("with a baby"), py_str("with a toddler"), py_str("with a child"), py_str("my kids"), py_str("my children"), py_str("my son"), py_str("my daughter"), py_str("pregnant"), py_str("my baby"), py_str("toddler"), py_str("toddlers"), py_str("kids"), py_str("children"), py_str("with my partner"), py_str("with a partner"), py_str("move in with"), py_str("moving in with"), py_str("moved in with"), py_str("move into his"), py_str("move into her"), py_str("my girlfriend"), py_str("my boyfriend"), py_str("my partner")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("birthday-date")); __args.push(py_tuple(vec![py_tuple(vec![py_str("when is"), py_str("when's"), py_str("whens"), py_str("what date"), py_str("which date"), py_str("what day")]), py_tuple(vec![py_str("birthday"), py_str("birthdays")])])); __args.push(py_str("I can't look a birthday up, and guessing one is worse than asking: a text now, and the answer goes in your phone with a reminder a week before. People rarely mind being asked.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("party"), py_str("parties"), py_str("invite"), py_str("invites"), py_str("invitations"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("buy"), py_str("plan"), py_str("planning"), py_str("many"), py_str("venue"), py_str("cake"), py_str("food"), py_str("guest"), py_str("guests"), py_str("surprise"), py_str("ideas"), py_str("organise")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("party-milestone")); __args.push(py_tuple(vec![g__MILESTONE_SAID(), py_tuple(vec![py_str("do"), py_str("doing"), py_str("what"), py_str("how"), py_str("should"), py_str("could"), py_str("can we"), py_str("ideas"), py_str("any"), py_str("anything"), py_str("something"), py_str("plan"), py_str("planning"), py_str("organise"), py_str("help"), py_str("sort"), py_str("get"), py_str("make"), py_str("give")])])); __args.push(g__party_plan()); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("guard".to_string(), g__a_milestone_said_as_an_age()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("years old"), py_str("year old"), py_str("miles"), py_str("years"), py_str("kg"), py_str("degrees")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("party-grownup")); __args.push(py_tuple(vec![py_tuple(vec![py_str("birthday party"), py_str("birthday for"), py_str("party for my"), py_str("party for her"), py_str("party for him"), py_str("surprise party"), py_str("anniversary party"), py_str("retirement party"), py_str("retirement do"), py_str("big birthday"), py_str("40th"), py_str("50th"), py_str("60th"), py_str("65th"), py_str("70th"), py_str("75th"), py_str("80th"), py_str("90th"), py_str("my mums birthday"), py_str("my dads birthday"), py_str("my mums 60th"), py_str("mums 70th"), py_str("a party"), py_str("the party"), py_str("house party"), py_str("party at home"), py_str("birthday"), py_str("birthdays"), py_str("invite to a party"), py_str("people to invite"), py_str("how many to invite"), py_str("guest list"), py_str("invitations"), py_str("the invitations"), py_str("send invitations"), py_str("send the invitations"), py_str("my nans birthday"), py_str("my nans 80th"), py_str("my grans birthday"), py_str("nans 80th"), py_str("grans 80th"), py_str("her 80th"), py_str("her 70th"), py_str("her 90th"), py_str("his 70th"), py_str("his 80th"), py_str("for her birthday"), py_str("for his birthday"), py_str("what can we do for"), py_str("leaving do"), py_str("leaving party"), py_str("retirement do"), py_str("work do"), py_str("christmas do")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("start"), py_str("first"), py_str("organise"), py_str("organized"), py_str("invite"), py_str("how many"), py_str("how much"), py_str("food"), py_str("venue"), py_str("where"), py_str("ideas"), py_str("tips"), py_str("advice"), py_str("sorted"), py_str("guests"), py_str("guest list"), py_str("party"), py_str("do"), py_str("doing"), py_str("what"), py_str("how"), py_str("should"), py_str("want"), py_str("wanted"), py_str("throwing"), py_str("throw"), py_str("hosting"), py_str("host"), py_str("send"), py_str("sent"), py_str("when"), py_str("what can we do"), py_str("we do")])])); __args.push(g__party_plan()); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("annual leave"), py_str("days leave"), py_str("day leave"), py_str("much leave"), py_str("leave days"), py_str("leave left"), py_str("leave balance"), py_str("leave allowance"), py_str("leave entitlement"), py_str("holiday entitlement"), py_str("days off"), py_str("annual leave entitlement"), py_str("end a chat"), py_str("end the chat"), py_str("end a conversation"), py_str("end the conversation"), py_str("chat at a party"), py_str("conversation at a party"), py_str("small talk"), py_str("stop talking"), py_str("get out of a conversation"), py_str("nattering"), py_str("get out of"), py_str("dont want to go"), py_str("don't want to go"), py_str("do i have to go"), py_str("cant come"), py_str("can't come"), py_str("make my excuses"), py_str("would rather not go"), py_str("decline"), py_str("turn down"), py_str("say no"), py_str("back out of"), py_str("not going"), py_str("kids party"), py_str("children's party"), py_str("childs party"), py_str("toddler party"), py_str("soft play"), py_str("conversation"), py_str("conversations"), py_str("chat"), py_str("excuse myself"), py_str("cornered"), py_str("mingle"), py_str("mingling"), py_str("remember"), py_str("remembering"), py_str("forget"), py_str("forgetting"), py_str("forgot"), py_str("keep track"), py_str("track of"), py_str("reminder"), py_str("reminders"), py_str("calendar"), py_str("diary"), py_str("list of birthdays"), py_str("playcentre"), py_str("kids"), py_str("kid"), py_str("child"), py_str("children"), py_str("year old"), py_str("years old"), py_str("5th"), py_str("6th"), py_str("7th"), py_str("8th"), py_str("9th"), py_str("10th"), py_str("11th"), py_str("12th"), py_str("toddler"), py_str("toddlers"), py_str("nursery"), py_str("school"), py_str("dinner party"), py_str("buffet for"), py_str("people coming over"), py_str("christmas party"), py_str("office party"), py_str("political"), py_str("party wall"), py_str("hen do"), py_str("stag do"), py_str("wedding"), py_str("baby shower"), py_str("christening"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("get her"), py_str("get him"), py_str("get my"), py_str("get our"), py_str("card"), py_str("cards"), py_str("what to write"), py_str("wear"), py_str("wearing"), py_str("put on"), py_str("putting on"), py_str("dress"), py_str("dress code"), py_str("outfit"), py_str("outfits"), py_str("shoes"), py_str("what to get"), g__A_PRESENT_FOR(), py_str("what should i write"), py_str("what do i write"), py_str("write in"), py_str("writing in"), py_str("message"), py_str("messages"), py_str("verse"), py_str("verses"), py_str("poem"), py_str("poems"), py_str("wishes"), py_str("what to say"), py_str("what do i say"), py_str("say goodbye"), py_str("say goodbyes"), py_str("goodbye"), py_str("goodbyes"), py_str("say bye"), py_str("leave the party"), py_str("leave a party"), py_str("leaving the party"), py_str("leaving a party"), py_str("leave early"), py_str("leaving early"), py_str("slip away"), py_str("say my goodbyes"), py_str("head off"), py_str("make my excuses"), py_str("when to leave"), py_str("how do i leave"), py_str("what should i cook"), py_str("what to cook"), py_str("what should i make"), py_str("what food should i"), py_str("what can i cook"), py_str("what can i make"), py_str("what should i serve"), py_str("what to make"), py_str("what shall i cook"), py_str("what do i cook"), py_str("card"), py_str("cards"), py_str("write in"), py_str("write on"), py_str("what to write"), py_str("what should i write"), py_str("message in"), py_str("note in"), py_str("sign the"), py_str("buy my"), py_str("get a"), py_str("buy a"), py_str("get for"), py_str("buy for"), py_str("what should i get"), py_str("what do i get"), py_str("what can i get"), py_str("what shall i get"), py_str("what could i get"), py_str("what would i get"), py_str("get them"), py_str("give them"), py_str("something for"), py_str("present for"), py_str("gift for"), py_str("game"), py_str("games"), py_str("board game"), py_str("board games"), py_str("party game"), py_str("party games"), py_str("what should we play"), py_str("something to play"), py_str("quiz"), py_str("games night")])), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let u: Py = __kwp(args, 0, "u").unwrap_or(Py::None); { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(u.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_a_gift_or_a_date(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(u.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_a_deed_of_their_own(args))(&__args) } } } }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("party-kids")); __args.push(py_tuple(vec![py_tuple(vec![py_str("kids party"), py_str("children's party"), py_str("childs party"), py_str("birthday party"), py_str("birthday for"), py_str("party for a"), py_str("5th birthday"), py_str("6th birthday"), py_str("7th birthday"), py_str("8th birthday"), py_str("toddler party"), py_str("how many kids"), py_str("how many children"), py_str("invite to a birthday")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("start"), py_str("idea"), py_str("ideas"), py_str("much"), py_str("activities"), py_str("food"), py_str("venue"), py_str("where"), py_str("organise"), py_str("invite"), py_str("cost"), py_str("decorate"), py_str("theme"), py_str("games"), py_str("cake"), py_str("guests"), py_str("party"), py_str("birthday"), py_str("toddler"), py_str("activities")])])); __args.push(g__KIDS_PARTY()); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("guard".to_string(), g__not_a_count_of_their_own()), ("avoid".to_string(), py_tuple(vec![py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my mother"), py_str("my father"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("grandma"), py_str("grandad"), py_str("nana"), py_str("nan"), py_str("granny"), py_str("40th"), py_str("50th"), py_str("60th"), py_str("65th"), py_str("70th"), py_str("75th"), py_str("80th"), py_str("90th"), py_str("retirement"), py_str("anniversary"), py_str("grown up"), py_str("for adults"), py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("wishes"), py_str("what to write"), py_str("what should i write"), py_str("what do i write"), py_str("write in"), py_str("message in"), py_str("what food should i bring"), py_str("what food should i take"), py_str("what food can i bring"), py_str("what food can i take"), py_str("what food do i bring"), py_str("what food to bring"), py_str("bring a dish"), py_str("dish to bring"), py_str("food to bring"), py_str("what should i bring"), py_str("what can i bring"), py_str("anything i can bring")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wedding-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("marriage"), py_str("civil partnership"), py_str("getting married"), py_str("marry"), py_str("marrying"), py_str("engagement party"), py_str("hen do"), py_str("stag do"), py_str("get married"), py_str("married"), py_str("we are getting married"), py_str("give notice"), py_str("notice of marriage"), py_str("notice to marry"), py_str("wedding day"), py_str("book the venue"), py_str("book a venue"), py_str("the wedding venue"), py_str("book the photographer"), py_str("book a photographer")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("budget"), py_str("guest list"), py_str("guests"), py_str("afford"), py_str("cheap"), py_str("start"), py_str("organise"), py_str("venue"), py_str("save"), py_str("first"), py_str("sort"), py_str("sort first"), py_str("sort out first"), py_str("pick"), py_str("pick first"), py_str("order"), py_str("prioritise"), py_str("priorities"), py_str("before what"), py_str("what do i need"), py_str("need to do"), py_str("whats needed"), py_str("what's needed"), py_str("paperwork"), py_str("papers"), py_str("document"), py_str("documents"), py_str("bring"), py_str("what to bring"), py_str("notice"), py_str("notice of marriage"), py_str("registrar"), py_str("register office"), py_str("legal"), py_str("admin"), py_str("witness"), py_str("witnesses"), py_str("arrange"), py_str("how do i get married"), py_str("how do we get married"), py_str("how to get married"), py_str("how do you get married"), py_str("how do i marry"), py_str("how do we marry"), py_str("how do i arrange"), py_str("how do we arrange"), py_str("what do i do"), py_str("what do we do"), py_str("what do i need to do"), py_str("where do i start"), py_str("how does it work"), py_str("how many guests"), py_str("how many people"), py_str("how many should we invite"), py_str("how many to invite"), py_str("number of guests"), py_str("how big"), py_str("size of the wedding"), py_str("far ahead"), py_str("how early"), py_str("how long before"), py_str("lead time"), py_str("how soon"), py_str("when should we book"), py_str("when to book"), py_str("book the venue"), py_str("book a venue"), py_str("book the photographer"), py_str("book a photographer"), py_str("book the registrar"), py_str("book the venue"), py_str("book a venue"), py_str("the wedding venue"), py_str("the photographer"), py_str("a photographer")])])); __args.push(g__wedding_plan()); __kws.extend(vec![("ask".to_string(), op_add(&op_add(&g_PLAN_ASK(), &g__WEDDING_ASK()), &py_tuple(vec![py_str("how many"), py_str("how far"), py_str("how long"), py_str("how soon"), py_str("how big"), py_str("when"), py_str("count")]))), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("costs"), py_str("how much"), py_str("price"), py_str("prices"), py_str("spend"), py_str("£"), py_str("wear"), py_str("wearing"), py_str("outfit"), py_str("outfits"), py_str("dress"), py_str("dresses"), py_str("jumpsuit"), py_str("jumpsuits"), py_str("hat"), py_str("hats"), py_str("fascinator"), py_str("shoes"), py_str("heels"), py_str("tie"), py_str("colour"), py_str("color"), py_str("dress code"), py_str("black tie"), py_str("landlord"), py_str("landlords"), py_str("landlady"), py_str("letting agent"), py_str("letting agents"), py_str("tenancy"), py_str("tenancies"), py_str("tenants"), py_str("my flat"), py_str("my lease"), py_str("the flat"), py_str("rent"), py_str("notice period"), py_str("evict"), py_str("deposit"), py_str("already booked"), py_str("havnt booked"), py_str("havent booked"), py_str("got the venue"), py_str("venue booked")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wedding-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("wedding breakfast"), py_str("wedding reception"), py_str("reception"), py_str("getting married"), py_str("we are getting married"), py_str("marrying"), py_str("barn wedding"), py_str("wedding day"), py_str("big day"), py_str("the wedding"), py_str("our wedding"), py_str("my wedding"), py_str("for a wedding"), py_str("a wedding")]), py_tuple(vec![py_str("food"), py_str("menu"), py_str("menus"), py_str("cater"), py_str("caters"), py_str("catering"), py_str("caterer"), py_str("caterers"), py_str("buffet"), py_str("meal"), py_str("meals"), py_str("eat"), py_str("eating"), py_str("feed"), py_str("feeding"), py_str("supper"), py_str("dinner"), py_str("breakfast"), py_str("hog roast"), py_str("barbecue"), py_str("bbq"), py_str("canapes"), py_str("finger food"), py_str("serve"), py_str("serving"), py_str("cook"), py_str("cooking"), py_str("drinks"), py_str("what should we eat"), py_str("what do we eat"), py_str("sit down"), py_str("sit-down"), py_str("plated")])])); __args.push(py_str("Decide the shape before the menu: a sit-down meal is about £35 to £60 a head catered and a buffet or hog roast about £20 to £35, while doing it yourselves lands near £10 to £15 a head and costs you the day. Work back from the number of guests - 60 people is a caterer's job unless you have a big kitchen and willing help - and ask the venue first, because many barns and halls either insist on their own list or charge corkage and a kitchen fee. Book six to nine months ahead for a summer Saturday, put dietary needs on the reply card, and plan evening food if the day runs past nine: something hot and cheap at 9pm saves a long hungry evening.")); __kws.extend(vec![("ask".to_string(), op_add(&g_PLAN_ASK(), &py_tuple(vec![py_str("food"), py_str("menu"), py_str("cater"), py_str("caterer"), py_str("caterers"), py_str("catering"), py_str("buffet"), py_str("eat"), py_str("feeds"), py_str("feed"), py_str("serve"), py_str("serving"), py_str("need"), py_str("needs"), py_str("what do we"), py_str("what should we"), py_str("guests")]))), ("avoid".to_string(), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("notice"), py_str("registrar"), py_str("register office"), py_str("witness"), py_str("witnesses"), py_str("wear"), py_str("wearing"), py_str("outfit"), py_str("dress code"), py_str("speech"), py_str("photographer"), py_str("flowers"), py_str("music"), py_str("playlist"), py_str("band"), py_str("dj"), py_str("first dance"), py_str("honeymoon"), py_str("thank you card")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wedding-timings")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("wedding day"), py_str("ceremony"), py_str("reception"), py_str("wedding breakfast"), py_str("getting married"), py_str("we are getting married"), py_str("big day"), py_str("marrying"), py_str("the invite"), py_str("the invitation"), py_str("on the invite"), py_str("on the invitation"), py_str("what time on the invite"), py_str("order of the day")]), py_tuple(vec![py_str("what time"), py_str("when should"), py_str("what time should"), py_str("order of the day"), py_str("timetable"), py_str("timeline"), py_str("schedule"), py_str("run order"), py_str("running order"), py_str("how early"), py_str("how long does the day"), py_str("how long does it last"), py_str("how long is the ceremony"), py_str("how long do we need"), py_str("how long should the day"), py_str("long does the day"), py_str("how long is the day"), py_str("day last"), py_str("whole day"), py_str("day go on"), py_str("how long do guests"), py_str("how long are guests"), py_str("long does a wedding"), py_str("long is a wedding"), py_str("wedding last"), py_str("how long the day"), py_str("what time do people"), py_str("what time do you"), py_str("what time is usual"), py_str("usual time"), py_str("start time"), py_str("what hour")])])); __args.push(py_str("The time on the invite is the ceremony time, not the time to arrive: guests should be there 20 to 30 minutes before, so say the ceremony time and let the venue tell people when to turn up. A ceremony runs about 30 minutes, then drinks and photographs for an hour to ninety minutes while the room is turned round, then the meal - which takes about two hours with speeches after it. Evening guests usually come from 7pm, and the first dance follows the evening food. Work backwards from the meal, because caterers want the room by a set time, and leave 20 minutes of slack: a late registrar cannot be made up.")); __kws.extend(vec![("ask".to_string(), op_add(&g_PLAN_ASK(), &py_tuple(vec![py_str("what time"), py_str("when"), py_str("order"), py_str("timetable"), py_str("timeline"), py_str("schedule"), py_str("run order"), py_str("running order"), py_str("usual")]))), ("avoid".to_string(), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("notice"), py_str("registrar"), py_str("register office"), py_str("witness"), py_str("witnesses"), py_str("wear"), py_str("wearing"), py_str("outfit"), py_str("dress code"), py_str("speech"), py_str("honeymoon"), py_str("food"), py_str("menu"), py_str("catering"), py_str("buffet"), py_str("venue"), py_str("cost"), py_str("how much"), py_str("ago"), py_str("how long ago"), py_str("how old"), py_str("already been"), py_str("hotel"), py_str("hotels"), py_str("guesthouse"), py_str("guest house"), py_str("b&b"), py_str("a room"), py_str("rooms"), py_str("flight"), py_str("flights"), py_str("travel"), py_str("parking"), py_str("taxi")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("renovation-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("renovation"), py_str("renovating"), py_str("extension"), py_str("building work"), py_str("new kitchen"), py_str("new bathroom"), py_str("rewire"), py_str("refurb"), py_str("doing up"), py_str("do up the house"), py_str("knock through"), py_str("kitchen"), py_str("kitchens"), py_str("bathroom"), py_str("bathrooms")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("start"), py_str("quote"), py_str("quotes"), py_str("architect"), py_str("order"), py_str("first"), py_str("organise"), py_str("cost"), py_str("timeline"), py_str("how long"), py_str("permission")])])); __args.push(py_str("Get three quotes on the same written scope, and don't start until you have the money for the whole job plus 15% for what the walls hide. Order the kitchen or bathroom six to eight weeks before you need it, because that is the delay everyone hits, and put the start date and the payment stages in a contract rather than agreeing them by text.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("clean"), py_str("cleans"), py_str("cleaning"), py_str("cleaned"), py_str("tidy"), py_str("tidies"), py_str("tidying"), py_str("scrub"), py_str("scrubbing"), py_str("hoover"), py_str("hoovering"), py_str("vacuum"), py_str("vacuuming"), py_str("mop"), py_str("mopping"), py_str("dust"), py_str("dusting"), py_str("wipe"), py_str("wiping"), py_str("wash"), py_str("washing up"), py_str("bleach"), py_str("disinfect"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("mould"), py_str("mold"), py_str("damp")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("room-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("kitchen"), py_str("kitchens"), py_str("new kitchen"), py_str("bathroom"), py_str("bathrooms"), py_str("new bathroom"), py_str("utility room"), py_str("laundry room"), py_str("refurb"), py_str("refit"), py_str("redo")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("planned"), py_str("lay out"), py_str("layout"), py_str("design"), py_str("designing"), py_str("redesign"), py_str("work triangle"), py_str("where do things go"), py_str("where should things go"), py_str("how should i lay"), py_str("how do i arrange"), py_str("rearrange"), py_str("sizes"), py_str("measurements"), py_str("what size"), py_str("get it right")])])); __args.push(g__a_room_plan()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cost"), py_str("costs"), py_str("how much"), py_str("price"), py_str("prices"), py_str("quote"), py_str("quotes"), py_str("cheap"), py_str("cheapest"), py_str("budget"), py_str("afford"), py_str("builder"), py_str("builders"), py_str("architect"), py_str("permission"), py_str("planning permission"), py_str("how long"), py_str("timeline"), py_str("contractor"), py_str("contractors"), py_str("clean"), py_str("cleaning"), py_str("tidy"), py_str("mess"), py_str("messy"), py_str("hoover"), py_str("scrub")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baby-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baby"), py_str("pregnant"), py_str("pregnancy"), py_str("newborn"), py_str("due date"), py_str("nursery")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("prepare"), py_str("preparing"), py_str("need"), py_str("needs"), py_str("buy"), py_str("buying"), py_str("hospital bag"), py_str("nesting"), py_str("ready for the baby"), py_str("before the baby"), py_str("shopping list"), py_str("organise"), py_str("cot"), py_str("pram"), py_str("car seat")])])); __args.push(py_str("Buy very little in advance: a car seat, a cot or Moses basket with a firm new mattress, a pram, six babygros in two sizes, nappies and whatever you're feeding with. Put the cot in your room, empty, with no bumpers or pillows, and pack the hospital bag at 36 weeks with snacks and a long phone charger — everyone forgets the charger.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("sleep"), py_str("sleeping"), py_str("hurt"), py_str("hurts"), py_str("pain"), py_str("aching"), py_str("feed"), py_str("feeding"), py_str("formula"), py_str("temperature"), py_str("fever"), py_str("cold"), py_str("cough"), py_str("sick"), py_str("ill"), py_str("health visitor"), py_str("jar"), py_str("jars"), py_str("pouch"), py_str("pouches"), py_str("puree"), py_str("purees"), py_str("baby food"), py_str("food"), py_str("eat"), py_str("eating"), py_str("eats"), py_str("meal"), py_str("meals"), py_str("weaning"), py_str("breakfast"), py_str("lunch"), py_str("dinner"), py_str("snack"), py_str("snacks"), py_str("milk")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baby-gear")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pram"), py_str("prams"), py_str("pushchair"), py_str("pushchairs"), py_str("buggy"), py_str("buggies"), py_str("cot"), py_str("cots"), py_str("moses basket"), py_str("car seat"), py_str("car seats"), py_str("bouncy chair"), py_str("high chair"), py_str("highchair"), py_str("travel cot"), py_str("crib"), py_str("cotbed"), py_str("cot bed"), py_str("nursery furniture"), py_str("baby monitor")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("get"), py_str("getting"), py_str("need"), py_str("needs"), py_str("which"), py_str("what about"), py_str("best"), py_str("cheap"), py_str("cheapest"), py_str("second hand"), py_str("secondhand"), py_str("second-hand"), py_str("new"), py_str("safe"), py_str("worth"), py_str("shop"), py_str("shopping"), py_str("borrow"), py_str("hand me down"), py_str("choose"), py_str("recommend"), py_str("compare"), py_str("price"), py_str("cost"), py_str("much"), py_str("how much"), py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("squeaky"), py_str("sticking"), py_str("stuck"), py_str("jammed"), py_str("wobbly"), py_str("broken"), py_str("broke"), py_str("snapped"), py_str("wont fold"), py_str("won't fold"), py_str("wont push"), py_str("won't push"), py_str("brake"), py_str("brakes"), py_str("wheel"), py_str("wheels"), py_str("fix"), py_str("mend"), py_str("repair")])])); __args.push(g__a_piece_of_baby_gear()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("sleep"), py_str("sleeping"), py_str("wont sleep"), py_str("hurt"), py_str("hurts"), py_str("pain"), py_str("aching"), py_str("feed"), py_str("feeding"), py_str("formula"), py_str("temperature"), py_str("fever"), py_str("cold"), py_str("cough"), py_str("sick"), py_str("ill"), py_str("reflux"), py_str("colic"), py_str("teething"), py_str("rash"), py_str("health visitor"), py_str("cat"), py_str("cats"), py_str("dog"), py_str("dogs"), py_str("pregnant"), py_str("pregnancy"), py_str("labour"), py_str("hospital bag"), py_str("clean"), py_str("cleaning"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("spill"), py_str("spills"), py_str("spilt"), py_str("spilled"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("stinks"), py_str("vomit"), py_str("wash"), py_str("washing"), py_str("dirty"), py_str("mould"), py_str("mold"), py_str("wipe"), py_str("scrub"), py_str("dry"), py_str("drying"), py_str("out of"), py_str("out the"), py_str("off a"), py_str("off my"), py_str("off the"), py_str("off our"), py_str("off his"), py_str("off her"), py_str("got it on"), py_str("got on the"), py_str("soaked"), py_str("spilled on"), py_str("smoke"), py_str("smoky"), py_str("cigarette"), py_str("cigarettes"), py_str("odour"), py_str("valet"), py_str("get rid"), py_str("sick in"), py_str("coat"), py_str("coats"), py_str("jacket"), py_str("jackets"), py_str("afford"), py_str("affords"), py_str("affording"), py_str("can i afford"), py_str("save up"), py_str("saving up"), py_str("savings"), py_str("budget"), py_str("budgeting"), py_str("pay for it"), py_str("installments"), py_str("instalments"), py_str("on finance"), py_str("payment plan")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pram-buy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pram"), py_str("prams"), py_str("pushchair"), py_str("pushchairs"), py_str("buggy"), py_str("buggies"), py_str("travel system"), py_str("travel systems"), py_str("stroller"), py_str("strollers")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("bought"), py_str("which"), py_str("what"), py_str("best"), py_str("choose"), py_str("choosing"), py_str("look for"), py_str("look out for"), py_str("worth"), py_str("recommend"), py_str("new"), py_str("second hand"), py_str("secondhand"), py_str("second-hand"), py_str("cheap"), py_str("price"), py_str("cost"), py_str("get"), py_str("need"), py_str("better"), py_str("worse"), py_str("bigger"), py_str("smaller")])])); __args.push(py_str("Lay it flat first: a newborn needs a carrycot or a seat that goes properly flat, and after that the fold and the boot decide it - fold it one-handed in the shop and lift it into your own boot before you pay. Check the brake holds on a slope with the child in it, that the handle suits your height, and that the wheels are big enough for the pavement you actually walk on. A new one is £150 to £400 and a good second-hand pram is £60 to £150, which is the better buy; the car seat is the one piece worth buying new, or with a history you know.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("fold"), py_str("folding"), py_str("folded"), py_str("stuck"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("mould"), py_str("mold"), py_str("repair"), py_str("fix"), py_str("bus"), py_str("plane"), py_str("airport"), py_str("safe to"), py_str("car seat"), py_str("isofix"), py_str("wont sleep"), py_str("sleep"), py_str("reflux"), py_str("colic"), py_str("carrier"), py_str("carriers"), py_str("sling"), py_str("slings"), py_str("or a carrier"), py_str("or a sling"), py_str("where can i"), py_str("where to buy"), py_str("where do i buy"), py_str("which shop"), py_str("cheapest place"), py_str("shop")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("holiday-countdown")); __args.push(py_tuple(vec![py_tuple(vec![py_str("holiday"), py_str("trip"), py_str("flight"), py_str("flights"), py_str("fly to"), py_str("flying to"), py_str("flying out"), py_str("going abroad"), py_str("abroad"), py_str("vacation"), py_str("travel insurance"), py_str("passport"), py_str("airport")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("prepare"), py_str("countdown"), py_str("organise"), py_str("checklist"), py_str("need to do"), py_str("what do i need"), py_str("sort out"), py_str("still to do"), py_str("book first"), py_str("first"), py_str("where do i start"), py_str("where to start"), py_str("how do i start"), py_str("start with")])])); __args.push(py_str("Two months out: check the passport has six months left on it, book what sells out, and take the insurance out the day you book, so a cancellation is covered. Two weeks out: repeat prescriptions, any jabs the country wants, and a card that works abroad; a week out, send copies of the passport and insurance to someone at home.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("coach trip"), py_str("coach journey"), py_str("train journey"), py_str("train ride"), py_str("train trip"), py_str("bus trip"), py_str("bus journey"), py_str("on a train"), py_str("on the train"), py_str("on a coach"), py_str("for hours"), py_str("long journey")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("declutter-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("declutter"), py_str("clear out"), py_str("clearout"), py_str("big tidy"), py_str("tidy up the house"), py_str("loft"), py_str("garage"), py_str("spare room"), py_str("wardrobe clear"), py_str("hire a skip"), py_str("skip hire"), py_str("a skip"), py_str("skip for"), py_str("full of junk"), py_str("junk"), py_str("clutter"), py_str("clear the clutter"), py_str("clear clutter"), py_str("messy house"), py_str("tidy my house"), py_str("tidy up my house"), py_str("clear my desk"), py_str("clear a room"), py_str("clear the room"), py_str("tidy my flat"), py_str("tidy the flat"), py_str("tidy my home"), py_str("tidy the house"), py_str("tidy my whole house"), py_str("tidy the whole house"), py_str("too much stuff"), py_str("clear the house"), py_str("rid of"), py_str("the mess"), py_str("mess everywhere"), py_str("clear the mess"), py_str("too much clutter"), py_str("sort the house out"), py_str("sort the house")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("start"), py_str("where"), py_str("first"), py_str("organise"), py_str("how"), py_str("much time"), py_str("weekend"), py_str("sorted"), py_str("hire"), py_str("should"), py_str("suppose"), py_str("then"), py_str("worth"), py_str("need"), py_str("full"), py_str("rubbish"), py_str("junk"), py_str("clear out"), py_str("declutter"), py_str("rid of"), py_str("stuff"), py_str("mess"), py_str("too much"), py_str("sort")])])); __args.push(py_str("One room at a time, three bags on the floor — bin, donate, keep — and the donate bags go to the car the same day or they drift back. Ten minutes a day beats a lost weekend, and start with the room you walk through most, because clearing that one is what makes the rest of the house feel possible.")); __kws.extend(vec![("ask".to_string(), op_add(&g_PLAN_ASK(), &g__DECLUTTER_ASK())), ("guard".to_string(), g__declutter_fits()), ("avoid".to_string(), py_tuple(vec![py_str("warm"), py_str("warmer"), py_str("warmth"), py_str("heat"), py_str("heated"), py_str("heating"), py_str("radiator"), py_str("insulate"), py_str("insulation"), py_str("lagging"), py_str("mineral wool"), py_str("loft insulation"), py_str("board out"), py_str("boarding"), py_str("floor the loft"), py_str("paint"), py_str("painting"), py_str("painted"), py_str("repaint"), py_str("charge"), py_str("charges"), py_str("charge for"), py_str("labour"), py_str("hourly"), py_str("mot"), py_str("service"), py_str("servicing"), py_str("serviced"), py_str("full service"), py_str("spend money"), py_str("spending money"), py_str("spent money"), py_str("save money"), py_str("saving money"), py_str("waste money"), py_str("wasting money"), py_str("money on"), py_str("junk food"), py_str("smoke"), py_str("smoking"), py_str("smoky"), py_str("fumes"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("condensation"), py_str("air out"), py_str("air it out"), py_str("stink"), py_str("stinks"), py_str("odour"), py_str("odor"), py_str("reek"), py_str("musty"), py_str("radiator"), py_str("radiators"), py_str("rad"), py_str("rads"), py_str("towel rail"), py_str("heating"), py_str("boiler"), py_str("thermostat"), py_str("thermostatic"), py_str("valve"), py_str("trv"), py_str("frost setting"), py_str("bleed"), py_str("fix"), py_str("fixed"), py_str("fixing"), py_str("repair"), py_str("repairs"), py_str("repaired"), py_str("broken"), py_str("not working"), py_str("doesnt work"), py_str("doesn't work"), py_str("wont close"), py_str("won't close"), py_str("wont open"), py_str("won't open"), py_str("stuck"), py_str("leak"), py_str("leaking"), py_str("replace"), py_str("replacing"), py_str("roller"), py_str("spring"), py_str("lock"), py_str("handle"), py_str("hinge"), py_str("jammed"), py_str("car"), py_str("cars"), py_str("van"), py_str("bike"), py_str("bicycle"), py_str("motorbike"), py_str("phone"), py_str("laptop"), py_str("washing machine"), py_str("fridge"), py_str("freezer"), py_str("dishwasher"), py_str("tumble dryer"), py_str("sell"), py_str("selling"), py_str("sold"), py_str("scrap"), py_str("part exchange"), py_str("trade in"), py_str("get rid of my"), py_str("mattress"), py_str("mattresses"), py_str("sofa"), py_str("settee"), py_str("couch"), py_str("armchair"), py_str("bed"), py_str("beds"), py_str("cot"), py_str("carpet"), py_str("carpets"), py_str("rug"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("battery"), py_str("rubble"), py_str("hardcore"), py_str("garden waste"), py_str("tree"), py_str("branches"), py_str("asbestos"), py_str("paddling pool"), py_str("trampoline"), py_str("moth"), py_str("moths"), py_str("fly"), py_str("flies"), py_str("ant"), py_str("ants"), py_str("mouse"), py_str("mice"), py_str("rat"), py_str("rats"), py_str("slug"), py_str("slugs"), py_str("snail"), py_str("snails"), py_str("weed"), py_str("weeds"), py_str("moss"), py_str("aphid"), py_str("aphids"), py_str("bedbug"), py_str("bedbugs"), py_str("bed bugs"), py_str("flea"), py_str("fleas"), py_str("wasp"), py_str("wasps"), py_str("cockroach"), py_str("cockroaches"), py_str("woodlice"), py_str("spider"), py_str("spiders"), py_str("pigeon"), py_str("pigeons"), py_str("fox"), py_str("foxes"), py_str("stain"), py_str("stains"), py_str("weevil"), py_str("weevils"), py_str("mite"), py_str("mites"), py_str("flour mite"), py_str("flour mites"), py_str("larder"), py_str("silverfish"), py_str("booklice"), py_str("woodworm"), py_str("carpet beetle"), py_str("clothes moth"), py_str("fruit fly"), py_str("fruit flies"), py_str("drain fly"), py_str("drain flies"), py_str("gnat"), py_str("gnats"), py_str("midge"), py_str("midges"), py_str("wasp nest"), py_str("ants nest"), py_str("flatmate"), py_str("flatmates"), py_str("housemate"), py_str("housemates"), py_str("roommate"), py_str("roommates"), py_str("house share"), py_str("houseshare"), py_str("lodger"), py_str("dust"), py_str("dusty"), py_str("dusting"), py_str("cobweb"), py_str("cobwebs"), py_str("hoover"), py_str("hoovering"), py_str("vacuum"), py_str("vacuuming"), py_str("wipe"), py_str("wiping"), py_str("mop"), py_str("mopping"), py_str("polish"), py_str("cleaning"), py_str("clean the"), py_str("scrub"), py_str("scrubbing"), py_str("limescale"), py_str("to move"), py_str("moving house"), py_str("moving home"), py_str("moving flat"), py_str("moving out"), py_str("moved house"), py_str("house move"), py_str("removal"), py_str("removals"), py_str("pack up"), py_str("packing"), py_str("care home"), py_str("residential home"), py_str("nursing home"), py_str("email"), py_str("emails"), py_str("inbox"), py_str("spam"), py_str("junk mail"), py_str("newsletter"), py_str("newsletters"), py_str("unsubscribe")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("weekend-ideas")); __args.push(py_tuple(vec![py_tuple(vec![py_str("weekend"), py_str("weekends"), py_str("day off"), py_str("days off"), py_str("saturday"), py_str("sunday"), py_str("free day"), py_str("bank holiday weekend"), py_str("bank holiday"), py_str("next weekend"), py_str("this weekend"), py_str("this evening"), py_str("tonight"), py_str("today"), py_str("around here"), py_str("near here"), py_str("near me"), py_str("in town"), py_str("in the area"), py_str("tomorrow"), py_str("tomorrow morning"), py_str("tomorrow afternoon"), py_str("tomorrow evening"), py_str("tomorrow night")]), py_tuple(vec![py_str("what"), py_str("anything"), py_str("ideas"), py_str("idea"), py_str("things to do"), py_str("something to do"), py_str("do this"), py_str("suggest"), py_str("recommend"), py_str("bored"), py_str("plan"), py_str("plans"), py_str("shall i"), py_str("can i do"), py_str("what can i do"), py_str("what should i do"), py_str("what can we do"), py_str("what shall we do"), py_str("nothing to do"), py_str("what is there to do"), py_str("any ideas"), py_str("do with")])])); __args.push(g__a_free_weekend()); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_free_weekend_fits_and_is_asked(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stretch_of_time_to_fill(args))(&__args) } } } })), ("avoid".to_string(), op_add(&op_add(&py_tuple(vec![py_str("clear out"), py_str("clear the"), py_str("tidy"), py_str("chores"), py_str("garage"), py_str("loft"), py_str("cupboard"), py_str("what helps"), py_str("what would help"), py_str("helps it"), py_str("help it"), py_str("help the"), py_str("ease it"), py_str("relief"), py_str("get rid of"), py_str("make it better"), py_str("stop it"), py_str("tablet"), py_str("tablets"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("painkiller"), py_str("painkillers"), py_str("ibuprofen"), py_str("paracetamol"), py_str("antihistamine"), py_str("antibiotic"), py_str("dose"), py_str("pharmacy"), py_str("chemist"), py_str("prescription"), py_str("symptom"), py_str("symptoms"), py_str("toothache"), py_str("headache"), py_str("migraine"), py_str("sore throat"), py_str("temperature"), py_str("fever"), py_str("nausea"), py_str("cramps"), py_str("stomach ache"), py_str("dizzy"), py_str("dentist"), py_str("gp"), py_str("doctor"), py_str("nurse"), py_str("111"), py_str("999"), py_str("side effect"), py_str("side effects"), py_str("decorate"), py_str("painting"), py_str("diy"), py_str("away"), py_str("hotel"), py_str("flight"), py_str("abroad"), py_str("match"), py_str("race"), py_str("half marathon"), py_str("long run"), py_str("work"), py_str("shift"), py_str("wear"), py_str("wearing"), py_str("wore"), py_str("put on"), py_str("dress"), py_str("dressing"), py_str("what should i take"), py_str("what should i bring"), py_str("what to take"), py_str("what to bring"), py_str("take with"), py_str("bring with"), py_str("take to a"), py_str("bring to"), py_str("need to bring"), py_str("need to take"), py_str("need to pack"), py_str("must bring"), py_str("to bring tomorrow"), py_str("pack"), py_str("packs"), py_str("packing"), py_str("packed"), py_str("suitcase"), py_str("suitcases"), py_str("luggage"), py_str("hand luggage"), py_str("carry on"), py_str("toiletries"), py_str("wash bag"), py_str("rucksack"), py_str("watch"), py_str("watching"), py_str("film"), py_str("films"), py_str("series"), py_str("netflix"), py_str("tv"), py_str("read"), py_str("reading"), py_str("book"), py_str("books"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("telly"), py_str("television"), py_str("bbc"), py_str("itv"), py_str("channel 4"), py_str("channel 5"), py_str("channel four"), py_str("channel five"), py_str("listings"), py_str("on telly"), py_str("on tv"), py_str("dinner"), py_str("lunch"), py_str("eat"), py_str("food"), py_str("bake"), py_str("baking"), py_str("shop"), py_str("buy"), py_str("train"), py_str("training"), py_str("gym"), py_str("run"), py_str("running"), py_str("football"), py_str("park"), py_str("kids"), py_str("children"), py_str("child"), py_str("baby"), py_str("cheap"), py_str("free"), py_str("money"), py_str("cost"), py_str("costs"), py_str("budget"), py_str("pay"), py_str("price"), py_str("any plans"), py_str("got plans"), py_str("got any plans"), py_str("up to tonight"), py_str("up to later")]), &crate::knowledge::lookup::g_A_WEATHER_ASK()), &crate::knowledge::lookup::g_A_HOURS_ASK()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hosting-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dinner party"), py_str("having people over"), py_str("people coming over"), py_str("guests"), py_str("buffet"), py_str("barbecue"), py_str("bbq"), py_str("roast for"), py_str("cooking for"), py_str("family over")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("much"), py_str("how many"), py_str("ahead"), py_str("timing"), py_str("timings"), py_str("first"), py_str("menu"), py_str("quantities"), py_str("12 people"), py_str("10 people"), py_str("8 people")])])); __args.push(py_str("Choose a menu you can cook twice before: one oven dish, something cold on the side, bread, and a pudding made the day before, so you're out of the kitchen when they arrive. For a barbecue, allow 250g of meat a head plus a vegetarian option; for a sit-down dinner, eight people is the most a normal oven does well.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("late-guests")); __args.push(py_tuple(vec![py_tuple(vec![py_str("guests"), py_str("guest"), py_str("guests coming"), py_str("guest coming"), py_str("guests are coming"), py_str("people coming"), py_str("people are coming"), py_str("people over"), py_str("people round"), py_str("people arriving"), py_str("people turning up"), py_str("friends coming"), py_str("friends are coming"), py_str("friends over"), py_str("friends round"), py_str("friends arriving"), py_str("friends turning up"), py_str("family coming"), py_str("family over"), py_str("company coming"), py_str("company over"), py_str("visitors"), py_str("visitors coming"), py_str("someone coming over"), py_str("people for dinner"), py_str("guests for dinner"), py_str("people coming for dinner"), py_str("hosting dinner"), py_str("having people over"), py_str("having friends over"), py_str("having guests"), py_str("party tonight"), py_str("people here at"), py_str("people are here"), py_str("guests are here"), py_str("theyre here"), py_str("they are here"), py_str("people are at the door"), py_str("someone at the door"), py_str("somebody at the door"), py_str("friend dropping"), py_str("friend popping"), py_str("dropping round"), py_str("popping round"), py_str("dropping by"), py_str("popping in"), py_str("coming round"), py_str("coming over"), py_str("someone"), py_str("somebody"), py_str("in laws"), py_str("in-laws"), py_str("mother in law"), py_str("father in law"), py_str("parents"), py_str("mum"), py_str("dad"), py_str("sister"), py_str("brother"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("grandparents"), py_str("neighbours")]), py_tuple(vec![py_str("in an hour"), py_str("in half an hour"), py_str("in a hour"), py_str("in 20 minutes"), py_str("in twenty minutes"), py_str("in 30 minutes"), py_str("in thirty minutes"), py_str("in 10 minutes"), py_str("in ten minutes"), py_str("in 15 minutes"), py_str("in fifteen minutes"), py_str("in 45 minutes"), py_str("in two hours"), py_str("in a couple of hours"), py_str("in a bit"), py_str("in 40 minutes"), py_str("this evening"), py_str("tonight"), py_str("nothing is ready"), py_str("nothing ready"), py_str("nothing prepared"), py_str("nothing made"), py_str("nothing in"), py_str("not ready"), py_str("im not ready"), py_str("i'm not ready"), py_str("not prepared"), py_str("havent started"), py_str("haven't started"), py_str("havent even started"), py_str("havent cooked"), py_str("last minute"), py_str("at short notice"), py_str("short notice"), py_str("unexpected"), py_str("unannounced"), py_str("turned up"), py_str("turning up"), py_str("arrived early"), py_str("arriving early"), py_str("early"), py_str("just told me"), py_str("just rang"), py_str("just called"), py_str("just texted"), py_str("surprise visit"), py_str("on their way"), py_str("on his way"), py_str("on her way"), py_str("on the way"), py_str("minutes away"), py_str("20 minutes away"), py_str("ten minutes away"), py_str("five minutes away"), py_str("at 6"), py_str("at 7"), py_str("at 8"), py_str("at 6pm"), py_str("at 7pm"), py_str("at 8pm"), py_str("at six"), py_str("at seven"), py_str("at eight"), py_str("at the door"), py_str("at my door"), py_str("already here"), py_str("later"), py_str("this afternoon"), py_str("only have"), py_str("only got"), py_str("nothing in"), py_str("nothing much in"), py_str("no food in")])])); __args.push(py_str("Do the three things they will actually notice, in this order: clear the floor into one basket and shut the doors on the rest, put out glasses and something to nibble, and cook one thing from what you already have - pasta, an oven tray of whatever vegetables and sausages are in, or bread, cheese and something cold from the fridge. Buy only the missing bit on the way home, and don't start a clean nobody will see: a drink in their hand and food within half an hour is the whole of it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("what should i cook"), py_str("what can i cook"), py_str("what to cook"), py_str("what can i make"), py_str("what to make"), py_str("recipe"), py_str("recipes"), py_str("what should i wear"), py_str("what to wear"), py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("menu"), py_str("roast"), py_str("for dinner"), py_str("dinner party"), py_str("dinner tonight"), py_str("meal"), py_str("meals"), py_str("food for"), py_str("feed"), py_str("what do i cook"), py_str("mess"), py_str("messy"), py_str("clean"), py_str("cleaning"), py_str("tidy"), py_str("tidy up"), py_str("hoover"), py_str("vacuum"), py_str("dust"), py_str("dishes"), py_str("washing up"), py_str("disaster"), py_str("shambles"), py_str("dirty"), py_str("filthy"), py_str("stay with us"), py_str("staying with us"), py_str("for the weekend"), py_str("for a week"), py_str("next week"), py_str("next month"), py_str("in a few weeks"), py_str("in two weeks"), py_str("tomorrow"), py_str("keeps turning up"), py_str("keeps coming over"), py_str("keeps coming round"), py_str("keeps dropping in"), py_str("keeps dropping by"), py_str("keeps popping in"), py_str("turning up unannounced"), py_str("unannounced"), py_str("uninvited"), py_str("without asking"), py_str("every weekend"), py_str("all the time"), py_str("every day"), py_str("again and again"), py_str("boundary"), py_str("boundaries"), py_str("how do i tell"), py_str("how do i ask"), py_str("without warning"), py_str("without telling me"), py_str("out of the blue"), py_str("watch"), py_str("watching"), py_str("watch with"), py_str("what to watch"), py_str("something to watch"), py_str("watch tonight"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("box set"), py_str("box sets"), py_str("series"), py_str("telly"), py_str("netflix"), py_str("prime"), py_str("disney"), py_str("iplayer"), py_str("streaming"), py_str("episode"), py_str("episodes"), py_str("listen to"), py_str("listen with"), py_str("play with"), py_str("a game"), py_str("board game"), py_str("board games"), py_str("cards"), py_str("whenever she likes"), py_str("whenever he likes"), py_str("whenever they like"), py_str("turns up without"), py_str("turned up without")])), ("bias".to_string(), Py::Int(10))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("christmas-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("christmas"), py_str("xmas"), py_str("christmas day"), py_str("christmas dinner"), py_str("boxing day")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("organise"), py_str("ahead"), py_str("timing"), py_str("timings"), py_str("hosting"), py_str("much"), py_str("start"), py_str("list"), py_str("prepare"), py_str("shop")])])); __args.push(py_str("Write every cooking time on one sheet and work back from when you want to eat: the bird rests an hour, so it comes out an hour before, and the potatoes go in as it comes out. Shop for the cold stuff two days before rather than on the day, because fridge space runs out, and hand out the pudding, the drinks and the ice to other people.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("road-trip-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("road trip"), py_str("long drive"), py_str("driving down"), py_str("driving to"), py_str("driving holiday"), py_str("driving abroad")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("stops"), py_str("route"), py_str("how long"), py_str("overnight"), py_str("break"), py_str("rest"), py_str("hours"), py_str("first"), py_str("check")])])); __args.push(py_str("Plan the driving days rather than the hours: put the stops in first, every two hours and one proper stop a day, and don't book more than six hours of driving in a day. Book the first and last night's bed before you go, check the spare tyre, the oil and the breakdown cover, and keep the overnight bag on the back seat.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("how long is the drive"), py_str("how long would the drive"), py_str("how long does the drive"), py_str("how far is"), py_str("how many miles"), py_str("how long does it take to drive"), py_str("warning light"), py_str("warning lights"), py_str("engine light"), py_str("dashboard"), py_str("mot"), py_str("car service"), py_str("full service"), py_str("needs a service"), py_str("due a service"), py_str("due for a service"), py_str("service the car"), py_str("serviced"), py_str("servicing"), py_str("breakdown"), py_str("battery light"), py_str("oil light")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("week-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("my week"), py_str("the week ahead"), py_str("plan my week"), py_str("my time"), py_str("everything at once"), py_str("everything feels urgent"), py_str("to do list"), py_str("never enough time")]), py_tuple(vec![py_str("plan"), py_str("planning"), py_str("organise"), py_str("prioritise"), py_str("prioritize"), py_str("urgent"), py_str("start"), py_str("manage"), py_str("sorted")])])); __args.push(py_str("Write down what is actually due this week, then pick three things a day and put them in the diary as blocks; the rest waits, and that is fine. Real deadlines go in first, the urgent-but-not-important jobs get one slot a day, and the way to protect the rest is to say no to one thing out loud.")); __kws.extend(vec![("ask".to_string(), g_PLAN_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-shop-waste")); __args.push(py_tuple(vec![py_tuple(vec![py_str("food"), py_str("food shopping"), py_str("groceries"), py_str("shopping"), py_str("supermarket"), py_str("fridge"), py_str("veg"), py_str("vegetables"), py_str("leftovers"), py_str("left over")]), py_tuple(vec![py_str("waste"), py_str("wasting"), py_str("wasted"), py_str("throw"), py_str("throwing"), py_str("threw"), py_str("thrown"), py_str("throw away"), py_str("throwing away"), py_str("thrown away"), py_str("throws away"), py_str("toss out"), py_str("bin"), py_str("binned"), py_str("binning"), py_str("in the bin"), py_str("rubbish"), py_str("goes off"), py_str("going off"), py_str("gone off"), py_str("rot"), py_str("rotting"), py_str("never use it"), py_str("never use them"), py_str("never get through"), py_str("buy too much"), py_str("buying too much"), py_str("buy less"), py_str("shop better")])])); __args.push(py_str("Write the week's four meals down before you shop and buy for those, not for the week you imagine. Keep a box in the fridge for what must go first, and put tonight's dinner in it when you unpack. Cook double twice a week and freeze the spare that evening, because leftovers in the fridge are what gets binned, and buy fresh less often: a top-up on the way home beats a full trolley on a Sunday.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("use by"), py_str("best before"), py_str("how long"), py_str("safe to eat"), py_str("still ok"), py_str("cat food"), py_str("dog food"), py_str("food bank"), py_str("school dinner"), py_str("packed lunch"), py_str("declutter"), py_str("moving house"), py_str("kitchen bin"), py_str("left out"), py_str("all night"), py_str("overnight"), py_str("out of the fridge"), py_str("is it ok"), py_str("two hours"), py_str("bin it"), py_str("gone bad"), py_str("goes off first"), py_str("go off first"), py_str("eat first"), py_str("use first"), py_str("use up first"), py_str("which shelf"), py_str("top shelf"), py_str("bottom shelf"), py_str("middle shelf"), py_str("salad drawer"), py_str("crisper"), py_str("fridge order"), py_str("where in the fridge"), py_str("what goes where"), py_str("off her food"), py_str("off his food"), py_str("off their food"), py_str("off our food"), py_str("off my food"), py_str("off your food"), py_str("gone off food"), py_str("gone off eating"), py_str("appetite")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("saving-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("save money"), py_str("saving money"), py_str("start saving"), py_str("savings"), py_str("save each month"), py_str("save every month"), py_str("put money aside"), py_str("money aside"), py_str("save some money"), py_str("save a bit"), py_str("i need to save"), py_str("need to save up"), py_str("for a rainy day"), py_str("save up")]), py_tuple(vec![py_str("how"), py_str("what"), py_str("whats"), py_str("what's"), py_str("where"), py_str("any"), py_str("help"), py_str("should"), py_str("could"), py_str("best"), py_str("idea"), py_str("ideas"), py_str("tips"), py_str("advice"), py_str("start"), py_str("better at"), py_str("get better"), py_str("want"), py_str("wanted"), py_str("trying"), py_str("need"), py_str("intend"), py_str("never have anything left"), py_str("keep"), py_str("manage"), py_str("much"), py_str("each month"), py_str("every month")])])); __args.push(py_str("Track two weeks of what actually leaves the account before you change anything: the subscriptions you forgot are usually where it went. Then pick an amount you will not miss, 10 to 20 per cent of what you take home, and move it on pay day into a separate account you carry no card for, so saving happens before the spending. Build towards three to six months of essential bills as the buffer, and raise the standing order by 10 pounds whenever a bill or a habit goes.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("whats"), py_str("what's"), py_str("any"), py_str("help"), py_str("should"), py_str("could"), py_str("best"), py_str("ideas"), py_str("tips"), py_str("advice"), py_str("start"), py_str("better"), py_str("want"), py_str("wanted"), py_str("trying"), py_str("need"), py_str("intend"), py_str("help me")])), ("avoid".to_string(), py_tuple(vec![py_str("daylight saving"), py_str("daylight savings"), py_str("summer time"), py_str("the clocks"), py_str("how long"), py_str("per month for"), py_str("interest"), py_str("isa"), py_str("pension"), py_str("mortgage"), py_str("deal"), py_str("cheaper"), py_str("coupon"), py_str("voucher"), py_str("insurance"), py_str("contract"), py_str("flight"), py_str("holiday"), py_str("christmas"), py_str("student loan"), py_str("overdraft fee"), py_str("moving"), py_str("move"), py_str("removal"), py_str("van"), py_str("sofa"), py_str("flat"), py_str("house"), py_str("heating"), py_str("boiler"), py_str("electricity"), py_str("energy"), py_str("gas"), py_str("water"), py_str("petrol"), py_str("fuel"), py_str("bill"), py_str("bills"), py_str("broadband"), py_str("phone"), py_str("groceries"), py_str("takeaway"), py_str("subscription"), py_str("account"), py_str("accounts"), py_str("rate"), py_str("rates"), py_str("bank"), py_str("gone"), py_str("lost"), py_str("losing"), py_str("things to do"), py_str("something to do"), py_str("bored"), py_str("kids are"), py_str("keeping the kids"), py_str("rainy day out"), py_str("ideas for a rainy day"), py_str("rainy day ideas"), py_str("ideas for rainy days"), py_str("what can i do for a rainy day"), py_str("what can we do for a rainy day"), py_str("what can i do on a rainy day"), py_str("what can we do on a rainy day"), py_str("what should i do on a rainy day"), py_str("what shall we do on a rainy day"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("watch"), py_str("watching"), py_str("tv"), py_str("telly"), py_str("series"), py_str("box set"), py_str("netflix"), py_str("read"), py_str("reading"), py_str("book"), py_str("books"), py_str("listen"), py_str("listening"), py_str("podcast"), py_str("game"), py_str("games"), py_str("music"), py_str("song"), py_str("songs"), py_str("album"), py_str("albums"), py_str("playlist"), py_str("playlists"), py_str("spotify"), py_str("tunes"), py_str("band"), py_str("bands"), py_str("draw"), py_str("drawing"), py_str("drawings"), py_str("doodle"), py_str("doodling"), py_str("sketch"), py_str("sketching"), py_str("crayons"), py_str("colouring"), py_str("coloring"), py_str("paint"), py_str("painting"), py_str("where should i keep"), py_str("where do i keep"), py_str("where to keep"), py_str("keep my savings"), py_str("credit card"), py_str("card"), py_str("pay off"), py_str("paying off"), py_str("debt"), py_str("debts"), py_str("overdraft"), py_str("loan"), py_str("what did i say"), py_str("what did i tell"), py_str("did i say"), py_str("did i tell"), py_str("remind me"), py_str("you said"), py_str("again")])), ("guard".to_string(), g__a_rainy_day_is_money())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("out-of-a-rut")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rut"), py_str("stuck"), py_str("same thing every day"), py_str("groundhog"), py_str("bored of my life"), py_str("bored of everything"), py_str("going nowhere"), py_str("in a slump"), py_str("slump"), py_str("plateau")]), py_tuple(vec![py_str("in"), py_str("get out"), py_str("getting out"), py_str("escape"), py_str("break"), py_str("how"), py_str("feel"), py_str("feeling"), py_str("again"), py_str("fix"), py_str("change")])])); __args.push(py_str("Change one thing you do on autopilot - a different walk, a new lunch, saying yes to the first invitation - because a rut is a loop of the same cues. Put one thing in the diary this week that you are looking forward to: the lift comes from having something ahead more than from willpower.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("depressed"), py_str("depression"), py_str("weeks now"), py_str("months now"), py_str("can't get out of bed"), py_str("no pleasure"), py_str("anxious"), py_str("anxiety"), py_str("job"), py_str("career"), py_str("relationship"), py_str("marriage"), py_str("running"), py_str("training"), py_str("weight"), py_str("gym"), py_str("traffic"), py_str("jam"), py_str("queue"), py_str("lift"), py_str("snow"), py_str("mud"), py_str("airport"), py_str("stuck on"), py_str("stuck at"), py_str("stuck in a meeting"), py_str("stuck in the house"), py_str("stuck indoors"), py_str("stuck with"), py_str("wheel"), py_str("drawer"), py_str("jar"), py_str("contract"), py_str("contracts"), py_str("contractor"), py_str("cancellation"), py_str("minimum term"), py_str("cake"), py_str("cakes"), py_str("loaf"), py_str("sponge"), py_str("bun"), py_str("buns"), py_str("muffin"), py_str("muffins"), py_str("bread"), py_str("tin"), py_str("tins"), py_str("baking"), py_str("pan"), py_str("tray"), py_str("mould"), py_str("pastry"), py_str("cash machine"), py_str("cashpoint"), py_str("atm"), py_str("card reader"), py_str("chip and pin"), py_str("ticket machine"), py_str("parking machine"), py_str("throat"), py_str("lump"), py_str("swallow"), py_str("swallowing"), py_str("swallowed"), py_str("something stuck"), py_str("i feel stuck"), py_str("feel stuck"), py_str("stuck in life"), py_str("feels stuck"), py_str("gear"), py_str("gears"), py_str("gearbox"), py_str("derailleur"), py_str("chain"), py_str("chains"), py_str("chainring"), py_str("pedal"), py_str("pedals"), py_str("cranks"), py_str("brakes"), py_str("brake"), py_str("clutch"), py_str("engine"), py_str("wheel"), py_str("wheels"), py_str("handlebars"), py_str("saddle"), py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("motorbike"), py_str("scooter"), py_str("mower"), py_str("lawnmower"), py_str("printer"), py_str("scanner"), py_str("laptop"), py_str("computer"), py_str("tablet"), py_str("keyboard"), py_str("mouse"), py_str("phone"), py_str("washing machine"), py_str("tumble dryer"), py_str("dishwasher"), py_str("vacuum"), py_str("fridge"), py_str("freezer"), py_str("boiler"), py_str("radiator"), py_str("oven"), py_str("microwave"), py_str("toilet"), py_str("tap"), py_str("shower"), py_str("door"), py_str("doors"), py_str("window"), py_str("windows"), py_str("gate"), py_str("zip"), py_str("zipper"), py_str("button"), py_str("screw"), py_str("screws"), py_str("bolt"), py_str("bolts"), py_str("lock"), py_str("key"), py_str("keys"), py_str("stuck in gear"), py_str("stuck in a gear"), py_str("stuck in first"), py_str("stuck in second"), py_str("stuck in third"), py_str("stuck in top gear"), py_str("stuck on the same gear"), py_str("gears are stuck"), py_str("gears wont"), py_str("gears won't"), py_str("wont go into gear"), py_str("won't go into gear"), py_str("wont shift"), py_str("won't shift")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("new-year-resolution")); __args.push(py_tuple(vec![py_tuple(vec![py_str("new year"), py_str("new years"), py_str("resolution"), py_str("resolutions"), py_str("new year's")]), py_tuple(vec![py_str("keep"), py_str("keeping"), py_str("stick"), py_str("sticking"), py_str("goal"), py_str("goals"), py_str("promise"), py_str("start"), py_str("starting"), py_str("how"), py_str("make"), py_str("best"), py_str("good"), py_str("succeed")])])); __args.push(py_str("Make it one resolution, small enough to do on your worst day - ten minutes, not an hour - and tie it to something you already do, like a walk after your morning coffee. Mark the days you did it rather than the streak you broke, and if you miss once, the rule is never to miss twice.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("when is"), py_str("date"), py_str("eve"), py_str("party"), py_str("fireworks"), py_str("open"), py_str("closed"), py_str("traffic"), py_str("dinner"), py_str("meal"), py_str("gym membership"), py_str("join"), py_str("died"), py_str("dies"), py_str("dying"), py_str("death"), py_str("dead"), py_str("passed away"), py_str("funeral"), py_str("killed"), py_str("grief"), py_str("grieving"), py_str("loss"), py_str("lost him"), py_str("lost her"), py_str("lost them"), py_str("miss him"), py_str("miss her"), py_str("miss them"), py_str("keep going over"), py_str("going over"), py_str("over the last"), py_str("that night"), py_str("cant stop thinking"), py_str("can't stop thinking")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-to-set-a-goal")); __args.push(py_tuple(vec![py_tuple(vec![py_str("goal"), py_str("goals"), py_str("goal setting"), py_str("set a goal"), py_str("set goals"), py_str("plan for the year"), py_str("yearly plan"), py_str("five year plan"), py_str("5 year plan"), py_str("3 year plan"), py_str("life plan"), py_str("plan my year"), py_str("plan my life"), py_str("what i want to achieve"), py_str("achieve"), py_str("ambition")]), py_tuple(vec![py_str("how"), py_str("set"), py_str("setting"), py_str("make"), py_str("making"), py_str("write"), py_str("start"), py_str("stick"), py_str("keep to"), py_str("better"), py_str("worth"), py_str("should i"), py_str("realistic"), py_str("realistically"), py_str("stuck"), py_str("no idea")])])); __args.push(py_str("Pick one theme for the year rather than ten goals, then turn it into three actions with dates: \"run 5k by June - book a race in January, run twice a week\" does more than \"get fit\". Make each one small enough to start in ten minutes on a bad day, put the first step in the diary this week, and look at the list once a month rather than every day. Two things that raise the odds: write them down where you will see them, and tell one person.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("new year"), py_str("resolution"), py_str("resolutions"), py_str("save"), py_str("saving"), py_str("savings"), py_str("deposit"), py_str("pension"), py_str("budget"), py_str("calories"), py_str("weight"), py_str("marathon"), py_str("training"), py_str("revision"), py_str("exam"), py_str("deadline"), py_str("workout"), py_str("gym")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-to-do-with-my-life")); __args.push(py_tuple(vec![py_tuple(vec![py_str("what should i do with my life"), py_str("what to do with my life"), py_str("what do i do with my life"), py_str("dont know what to do with my life"), py_str("don't know what to do with my life"), py_str("no idea what to do with my life"), py_str("what next in life"), py_str("direction in life"), py_str("purpose in life"), py_str("my purpose"), py_str("where am i going"), py_str("what am i doing with my life"), py_str("find my purpose")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("should"), py_str("find"), py_str("figure"), py_str("work out"), py_str("decide"), py_str("lost"), py_str("stuck"), py_str("confused"), py_str("help"), py_str("advice"), py_str("next")])])); __args.push(py_str("Don't try to answer the whole of it at once. Write three short lists: what you are good at (the things other people ask you for, not what you would like to be good at), what you would do for free, and what you need money for. Then pick one small experiment that touches two of the three - a course, a volunteering shift, a paid side project - and give it three months, because direction comes from doing rather than deciding. Keep whatever you would happily do again.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("depressed"), py_str("depression"), py_str("anxious"), py_str("suicidal"), py_str("weeks now"), py_str("months now"), py_str("cant get out of bed"), py_str("career change"), py_str("apply"), py_str("ucsas"), py_str("ucas"), py_str("course fees"), py_str("my job"), py_str("my boss"), py_str("my partner"), py_str("my husband"), py_str("my wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("divorce"), py_str("break up"), py_str("grief"), py_str("died"), py_str("funeral")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("free-day-out")); __args.push(py_tuple(vec![py_tuple(vec![py_str("free"), py_str("cheap"), py_str("cheaper"), py_str("no money"), py_str("not much money"), py_str("skint"), py_str("broke"), py_str("dont cost"), py_str("doesnt cost"), py_str("thank mustnt"), py_str("tight budget"), py_str("on a budget"), py_str("low cost")]), py_tuple(vec![py_str("day out"), py_str("days out"), py_str("things to do"), py_str("something to do"), py_str("something nice"), py_str("something special"), py_str("what can i do"), py_str("what can we do"), py_str("where can we go"), py_str("where can i go"), py_str("near me"), py_str("this weekend"), py_str("at the weekend"), py_str("with the kids"), py_str("take the kids"), py_str("with children"), py_str("family")])])); __args.push(py_str("Free ones that work: a walk with a purpose (a park, a canal, the woods, a hill with a view - flask in the bag), a museum or gallery with free entry, the library, or a Saturday market. With kids, add a list of five things to find and a ball. Take snacks and a drink, and the day costs you parking and a coffee.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("meal"), py_str("meals"), py_str("food"), py_str("eat"), py_str("eating"), py_str("dinner"), py_str("lunch"), py_str("tea"), py_str("breakfast"), py_str("takeaway"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("bake"), py_str("baking"), py_str("shop"), py_str("shopping"), py_str("grocery"), py_str("groceries"), py_str("supermarket"), py_str("film"), py_str("films"), py_str("movie"), py_str("watch"), py_str("book"), py_str("books"), py_str("read"), py_str("reading"), py_str("freezer"), py_str("freeze"), py_str("sugar free"), py_str("gluten free"), py_str("dairy free"), py_str("free range"), py_str("car"), py_str("cars"), py_str("van"), py_str("insurance"), py_str("rent"), py_str("bill"), py_str("bills"), py_str("delivery"), py_str("postage"), py_str("sim"), py_str("tariff"), py_str("subscription"), py_str("mobile"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("secret santa"), py_str("stocking"), py_str("stocking filler")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheap-things-to-do")); __args.push(py_tuple(vec![py_tuple(vec![py_str("things to do"), py_str("something to do"), py_str("what can i do"), py_str("what should i do"), py_str("what to do"), py_str("what can we do"), py_str("what should we do"), py_str("anything to do"), py_str("ideas"), py_str("activities"), py_str("activity"), py_str("day out"), py_str("days out"), py_str("afternoon out")]), py_tuple(vec![py_str("weekend"), py_str("saturday"), py_str("sunday"), py_str("free"), py_str("cheap"), py_str("cheaply"), py_str("budget"), py_str("family"), py_str("everyone"), py_str("at home"), py_str("indoors"), py_str("outside")])])); __args.push(py_str("Pick one shape and put a time on it, because a free day with no plan turns into the sofa. The ones that cost nothing: a walk with a destination (a park, a canal, a hill, a shop that sells something you like), a free museum or gallery - most have a permanent collection that costs nothing - and the library, which has events, papers and warmth. At home: a film nobody has seen, a bake with whatever is in the cupboard, a swap of clothes or books with friends, or a board game with a rule that the phones stay in another room. For a family, one outside thing in the morning and one quiet thing after lunch is plenty: children remember the day that had a walk and a picnic in it, not the one that cost forty pounds.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("recipe"), py_str("cook"), py_str("cooking"), py_str("dinner"), py_str("lunch"), py_str("tea"), py_str("eat"), py_str("meal"), py_str("back"), py_str("breakfast"), py_str("breakfasts"), py_str("brunch"), py_str("snack"), py_str("snacks"), py_str("packed lunch"), py_str("pain"), py_str("knee"), py_str("doctor"), py_str("gp"), py_str("work"), py_str("job"), py_str("boss"), py_str("office"), py_str("school run"), py_str("hotel"), py_str("flight"), py_str("flights"), py_str("abroad"), py_str("holiday"), py_str("leave my"), py_str("quit"), py_str("city break"), py_str("city breaks"), py_str("weekend away"), py_str("weekend break"), py_str("break away"), py_str("night away"), py_str("couple of nights"), py_str("few nights"), py_str("weekend in"), py_str("rent"), py_str("bills"), py_str("benefits"), py_str("save money"), py_str("savings"), py_str("invest"), py_str("loan"), py_str("debt"), py_str("kids"), py_str("children"), py_str("child"), py_str("toddler"), py_str("baby"), py_str("year old"), py_str("entertain"), py_str("keep busy"), py_str("stuck inside"), py_str("bored"), py_str("boring"), py_str("half term"), py_str("thunderstorm"), py_str("thunderstorms"), py_str("lightning"), py_str("thunder"), py_str("flood"), py_str("floods"), py_str("flooding"), py_str("flooded"), py_str("dairy free"), py_str("gluten free"), py_str("gluten-free"), py_str("sugar free"), py_str("sugar-free"), py_str("wheat free"), py_str("egg free"), py_str("nut free"), py_str("meat free"), py_str("fat free"), py_str("lactose free"), py_str("caffeine free"), py_str("alcohol free"), py_str("free from"), py_str("free range"), py_str("freezer"), py_str("freezers"), py_str("frozen"), py_str("duty free"), py_str("hands free"), py_str("hands-free"), py_str("free trial"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("secret santa"), py_str("stocking"), py_str("stocking filler")])), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_day_out_is_the_question(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_stretch_of_time_to_fill(args))(&__args) } } } }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dish-for-tonight")); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::knowledge::cooking::g_RECIPES()) { let __unpk648 = py_unpack(&__item.clone(), 4, None); let mut aliases: Py = __unpk648[0].clone(); let mut _keys: Py = __unpk648[1].clone(); let mut _body: Py = __unpk648[2].clone(); let mut _avoid: Py = __unpk648[3].clone(); for __item in py_iter_or_raise(&aliases.clone()) { let mut alias: Py = __item.clone(); __out.push(alias.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_tuple(vec![py_str("tonight"), py_str("this evening"), py_str("this afternoon"), py_str("for tea"), py_str("for dinner"), py_str("for supper"), py_str("for lunch"), py_str("later"), py_str("this weekend"), py_str("tomorrow night")]), py_tuple(vec![py_str("planning"), py_str("planning on"), py_str("plan"), py_str("plans"), py_str("planned"), py_str("fancy"), py_str("fancied"), py_str("i might"), py_str("shall i"), py_str("should i"), py_str("decided to")])])); __args.push(g__dish_for_tonight()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(3))), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("went wrong"), py_str("too thin"), py_str("too thick"), py_str("too salty"), py_str("bland"), py_str("burnt"), py_str("what should i cook"), py_str("what can i cook"), py_str("what shall i cook"), py_str("what to cook"), py_str("what should i make"), py_str("what can i make"), py_str("what should i eat"), py_str("what can i eat"), py_str("whats for"), py_str("what's for"), py_str("what to eat"), py_str("meal ideas"), py_str("dinner ideas"), py_str("watery"), py_str("dry"), py_str("recipe"), py_str("how long"), py_str("how do i"), py_str("what goes with"), py_str("vegan"), py_str("vegetarian"), py_str("veggie"), py_str("dairy"), py_str("gluten"), py_str("wheat"), py_str("allergic"), py_str("allergy"), py_str("coeliac"), py_str("celiac"), py_str("intolerant"), py_str("calories"), py_str("healthy"), py_str("keep"), py_str("keeps"), py_str("store"), py_str("freeze"), py_str("leftover"), py_str("left over"), py_str("reheat"), py_str("restaurant"), py_str("takeaway"), py_str("shop"), py_str("buy"), py_str("supermarket"), py_str("tesco"), py_str("aldi"), py_str("make"), py_str("makes"), py_str("making"), py_str("made"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("cooked")]), &crate::knowledge::cooking::g_ANNOUNCED_DISH()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("try-once")); __args.push(py_tuple(vec![py_tuple(vec![py_str("try once"), py_str("tried once"), py_str("at least once"), py_str("once in my life"), py_str("once in a lifetime"), py_str("never done"), py_str("never tried"), py_str("never have i"), py_str("ive never"), py_str("i have never"), py_str("something new"), py_str("something different"), py_str("new thing"), py_str("worth trying"), py_str("something to try"), py_str("one thing to try"), py_str("bucket list"), py_str("everyone should try"), py_str("everyone should do"), py_str("everyone should have"), py_str("everyone has to do"), py_str("wouldnt think of"), py_str("try next"), py_str("something ive not"), py_str("before i die"), py_str("missing out")]), g_TRY_ONCE_ASK()])); __args.push(g__try_once()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("try"), py_str("tried"), py_str("trying"), py_str("should"), py_str("suggest"), py_str("recommend"), py_str("tell"), py_str("give"), py_str("idea"), py_str("ideas"), py_str("anything"), py_str("something"), py_str("one thing"), py_str("surprise"), py_str("once"), py_str("want"), py_str("need")])), ("bias".to_string(), Py::Int(6)), ("avoid".to_string(), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("dinner"), py_str("lunch"), py_str("breakfast"), py_str("tea"), py_str("bake"), py_str("baking"), py_str("food"), py_str("restaurant"), py_str("meal"), py_str("meals"), py_str("gym"), py_str("workout"), py_str("training"), py_str("running"), py_str("run"), py_str("diet"), py_str("exercise"), py_str("hair"), py_str("skin"), py_str("garden"), py_str("plant"), py_str("plants"), py_str("decorate"), py_str("paint"), py_str("job"), py_str("jobs"), py_str("career"), py_str("interview"), py_str("course"), py_str("holiday"), py_str("trip"), py_str("city"), py_str("town"), py_str("hotel"), py_str("shop"), py_str("shopping"), py_str("buy"), py_str("money"), py_str("song"), py_str("album"), py_str("band"), py_str("film"), py_str("films"), py_str("book"), py_str("books"), py_str("series"), py_str("game"), py_str("games"), py_str("music"), py_str("podcast"), py_str("tv"), py_str("netflix"), py_str("sport"), py_str("booked"), py_str("already"), py_str("tried it"), py_str("tried that"), py_str("didnt like"), py_str("not for me"), py_str("went wrong"), py_str("every"), py_str("each"), py_str("daily"), py_str("everyday"), py_str("weekly"), py_str("monthly"), py_str("routine"), py_str("habit"), py_str("habits"), py_str("regularly"), py_str("stick to"), py_str("keep it up")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("try-once-more")); __args.push(g_TRY_ONCE_MORE()); __args.push(g__try_once_more()); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("avoid".to_string(), py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("dinner"), py_str("food"), py_str("paint"), py_str("garden")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _not_a_count_of_their_own(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the line counts what the person has rather than asking how many to invite.\n\n    Read on the line as typed, not on the words the memory of the chat lends it: \"how many\n    children do we have\" after a party question was read with the party's words added and reached\n    the party plan again.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(w.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OWN_COUNT(), "search", args))(&__args) }));
    Py::None
}
pub fn _not_a_date_question(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the line asks for the date of a birthday rather than for a party.\n\n    Read on the line as typed, for the same reason _not_a_count_of_their_own is: the memory of a\n    party chat lends the row its own words back.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(w.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    line = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }), &py_str(" "));
    return Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DATE_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DO(), "search", args))(&__args) })) } }));
    Py::None
}
pub fn _asks_for_ideas(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when the line asks what happened rather than for something to do.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(w.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WHAT_HAPPENED(), "search", args))(&__args) }));
    Py::None
}
pub fn _the_free_weekend_fits_and_is_asked(args: &[Py]) -> Py {
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
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _free_weekend_fits(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_for_ideas(args))(&__args) } } };
    Py::None
}
pub fn _not_a_deed_of_their_own(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("False when the line reports something the person did to somebody: that is a repair.\n\n    \"i forgot my mums birthday, what do i do\" asks how to put it right, and the party plan -\n    guest count, buffet budget, invitations three weeks out - is a plan for a day that has gone.\n    \"i didnt get my mum anything for her birthday\" is the same turn, and it was reading as a\n    birthday still to plan as well (bot/compose/repair.py). An ask to put it right names no deed\n    of its own (\"how do i make it up to my mum\"), and in a chat that has just said the birthday\n    was missed it is the same turn again: the plan for the party took it.\n    ");
    return Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.repair", "owns_the_deed", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.repair", "line_of", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.repair", "amends_to", args))(&__args) } } }));
    Py::None
}
pub fn _declutter_fits(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut job: Py = Py::None;
    let mut names_pile: Py = Py::None;
    let mut names_room: Py = Py::None;
    let mut pile: Py = Py::None;
    let mut room: Py = Py::None;
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when the line names a room or a pile to clear, or asks the job with nothing named.\n\n    The row keys the plainest ways of saying a clear-out, and most of them (\"rid of\", \"mess\",\n    \"stuff\", \"too much\") are also how a wart, a limescale stain or an exclamation is said.\n    ");
    text = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut w: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(w.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    clean = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BARE_RID(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__JOB_NAMED()) { let mut job: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &job.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TOO_MUCH(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    names_room = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DECLUTTER_ROOM()) { let mut room: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &room.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    names_pile = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DECLUTTER_STUFF()) { let mut pile: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &pile.clone()), &py_str(" "))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = names_room.clone(); if py_truthy(&__b) { __b } else { names_pile.clone() } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _try_once(args: &[Py]) -> Py {
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
    let _ = py_str("One thing worth trying once, or None when the line is about a subject of its own.\n\n    The pool holds no facts about anything in particular, so it can only answer a line that\n    asks for a suggestion and nothing more. A line that names a subject - \"something new to try\n    with chicken\", \"something new to try at the gym\" - is a question about that subject, and\n    the module that knows about it should take the turn rather than this pool.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_VAGUE_TOPICS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TRY_ONCE()); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn _try_once_more(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut one: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Another thing to try, when the last reply was one - None for every other line.\n\n    \"Give me another one\" is the commonest thing said to this pool, and it names nothing at\n    all: the only way to read it is from what the bot just said. So the row takes the turn only\n    when the last reply was one of these, and the rest of the time it steps back and lets the\n    line be whatever else it was about.\n    ");
    turns = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turns.clone(), &op_neg(&Py::Int(1)))); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TRY_ONCE()) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "said_again", args))(&__args) }); } py_set(__out) })() }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "said_again", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TRY_ONCE()); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn _wedding_plan(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    line = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WEDDING_WHEN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g__WEDDING_LEAD_TIME();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WEDDING_PAPERS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WEDDING_MONEY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_str("Give notice first: you each do it at your local register office at least 29 days before the wedding (in Scotland a marriage notice to the registrar), with a passport or birth certificate and proof of address. Book the venue and the registrar's slot, and find two witnesses. The 29 days is a legal minimum, not a booking window, so do not leave it late.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WEDDING_COUNT(), "search", args))(&__args) }) {
    return g__WEDDING_GUESTS();
    }
    return py_str("Guest list first, because it sets the cost of everything else: the venue, the food and the drink are about 60% of a wedding, and halving the guest list roughly halves the bill. Book the venue and the registrar next (a year ahead for a Saturday), then the photographer, and keep 10% back for what you forgot.");
    Py::None
}
pub fn _asks_about_the_world(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is asking what is going on in the world rather than filling a day.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__THE_WORLD()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_THE_WORLD(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _said_words(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The line as one lowered string, from the clean text as well as the words.");
    parts = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }]);
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&parts.clone(), "extend", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    Py::None
}
pub fn _said_now(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("This line alone, without the turn before that a short follow-up arrives with.\n\n    `clean` and `text` carry the turn before when the line leans on it, so a chat about\n    \"taking my mum to york\" put \"taking\" in the context of the next line, and \"what should we\n    see\" was asked what the tablets were for. `clauses` is where the two are kept apart: the\n    last one is this line, and one clause means the line stood on its own.\n    ");
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clauses")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut clause: Py = __item.clone(); if py_truthy(&clause.clone()) {  __out.push(clause.clone()); } } py_list(__out) })() };
    if py_truthy(&clauses.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&clauses.clone(), &op_neg(&Py::Int(1)))) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) };
    Py::None
}
pub fn _a_household_move(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks for the plan of a move of house, spelled without the word house.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MOVE_TO_PLAN(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MOVE_ELSEWHERE(), "search", args))(&__args) })) } };
    Py::None
}
pub fn _a_household_move_or_a_named_move(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names the home it is leaving, or asks for a move's plan without one.\n\n    The row's own key is written with the house in it (\"moving house\", \"how do i move flat\"),\n    so a line that says one of those keeps its answer. The guard only decides the newer guess -\n    \"plan a move\" with no house in it - which would otherwise take a move abroad or a move at\n    work.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOME_IN_THE_LINE(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_household_move(args))(&__args) };
    Py::None
}
pub fn _a_free_weekend(args: &[Py]) -> Py {
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
    let _ = py_str("The free-weekend plan, or None when the line is asking about something you swallow.\n\n    \"can i take anything tonight\" and \"can i take anything for it tonight\" got the plan for a\n    weekend (\"Pick two things and put them in the diary by Friday\"): \"tonight\" is in this row's\n    key and \"anything\" is the rest of it, and neither says what the person is asking about. A\n    line that names a medicine, or asks whether to take one, is never a thing to do on a\n    Saturday. When it leans on the turn before (\"what can i take for it\", after a toothache) the\n    row stands down and the health rows answer it from the line it leans on; when it names\n    nothing at all, the honest answer is to ask what it is for.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_medicine_line(args))(&__args) }) {
    return g_MEDICINE_NEEDS_A_REASON();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__TODAY_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__WEEKEND_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) })) } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_tuple(vec![py_str("tonight"), py_str("this evening")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }) {
    return g_EVENING_PLAN();
    }
    return g_DAY_OFF_PLAN();
    }
    return g_FREE_WEEKEND();
    Py::None
}
pub fn _holds(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when one of `words` stands in `said` as a whole word or phrase.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_medicine_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is about something you swallow, not about a free day.\n\n    The verb on its own is too little: \"give\" and \"take\" are how people ask for a suggestion\n    (\"give me some ideas for a rainy weekend\", \"take a look at my weekend\"), and the medicine\n    branch answered the first of those with a question about a tablet. A swallowed thing named\n    by name keeps any line a medicine line; the verb only counts when the line is not asking\n    for ideas.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__SWALLOWED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_IDEA_ASKED_FOR(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__SWALLOWED_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_stretch_of_time_to_fill(args: &[Py]) -> Py {
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
    let mut covered: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here_now: Py = Py::None;
    let mut line: Py = Py::None;
    let mut line_said: Py = Py::None;
    let mut named: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words_named: Py = Py::None;
    let _ = py_str("Whether the line asks what to do with time it has, rather than about a thing it names.\n\n    The reading already says what the line is about: `subject` names the one thing it is about\n    and `entities` names the rest. A subject made only of time, money and company words is the\n    free day itself (\"something to do on saturday without spending much\" reads as `spending`,\n    \"i have a week off in summer any ideas\" as `summer`), and anything else is the subject the\n    line is asking about instead.\n    ");
    named = py_list(vec![]);
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&subject.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("entities")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&found.clone(), &py_str("text"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    }
    line_said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREE_TIME_CUE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_FREE_TIME_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FREE_TIME_WHO(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_DAY_CONDITION(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FREE_TIME_TIME(), &word.clone())) } } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    words_named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AT_HOME(), "sub", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_named.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FREE_TIME_TIME(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    words_named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_named.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_HOLIDAY_PREFIX(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    }
    covered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_said.clone()); __args.push(g_FREE_TIME()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _covered_words(args))(&__args) };
    here_now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line_said.clone()); __kws.extend(vec![("block".to_string(), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(covered.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(line_said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FREE_TIME_TIME())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_place_the_block_is_at(args))(&__args) };
    let __iter2 = py_iter_or_raise(&words_named.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FREE_TIME_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FREE_TIME_WHO(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&covered.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&here_now.clone(), &word.clone())) } }) {
    continue;
    }
    return Py::Bool(false);
    }
    line = line_said.clone();
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FREE_TIME_TIME())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AT_HOME(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&covered.clone()) {
    return Py::Bool(true);
    }
    topic = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic_own")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    return { let __b = Py::Bool(!py_truthy(&topic.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&topic.clone(), &py_str("small-talk"))) } };
    Py::None
}
pub fn _free_weekend_fits(args: &[Py]) -> Py {
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
    let mut follows: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("False when the line is not a question about the weekend at all.\n\n    Three lines reached this row only because \"tonight\" is in its key: an ask for a medicine\n    that names no medicine (\"can i take anything for it tonight\", after a toothache), an ask\n    for something to take to a place (\"anything i can take to a picnic this weekend\"), and a\n    plan ask the turn after an illness (\"i have a cold\" then \"anything i can do tonight\"). The\n    row stands down on all three, so the row that knows the subject answers instead. A line\n    that names its own day or its own thing to do keeps the plan.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_a_thing_the_person_has(args))(&__args) }) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_A_CLOCK(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_about_the_world(args))(&__args) }) {
    return Py::Bool(false);
    }
    leans = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    follows = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leans.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__BRING_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(take|taking|took|bring|bringing|give|giving|make)\\b[^.]*\\bto\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__SWALLOWED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_now(args))(&__args) }); __args.push(g__SWALLOWED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_now(args))(&__args) }); __args.push(g__OWN_ACTIVITY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_medicine_line(args))(&__args) }))) {
    if py_truthy(&{ let __b = follows.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans.clone(), "get", args))(&__args) }, &py_str("health"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g__OWN_ACTIVITY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) })) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&follows.clone()));
    Py::None
}
pub fn _a_rainy_day_is_money(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut said: Py = Py::None;
    let _ = py_str("False when \"for a rainy day\" is the weather rather than the savings idiom.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FOR_A_RAINY_DAY(), "search", args))(&__args) }))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MONEY_WITH_IT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_city_is_named(args: &[Py]) -> Py {
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
    let mut city: Py = Py::None;
    let _ = py_str("True when the line names a city a guide answers for.");
    let mut CITIES: Py = crate::knowledge::travel::g_CITIES();
    let __iter1 = py_iter_or_raise(&CITIES.clone());
    for __item in __iter1 {
    city = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__AMBIGUOUS_CITY_WORDS(), &city.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("(?<![a-z])%s(?![a-z])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(city.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push(op_add(&op_add(&py_str(" "), &line.clone()), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _ideas_about_a_thing(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut covered: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for ideas about a named thing rather than about a free block.\n\n    \"any ideas for a film tonight\" and \"cheap christmas present ideas\" are the film row's and\n    the present row's question; \"any ideas for the weekend\" and \"any ideas for tonight\" are\n    this domain's. Read from the clean line, because it is the words as typed that say what the\n    ideas are for, and the reading drops the preposition the subject hangs on.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__IDEA_NONE(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__IDEAS_ASK(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__IDEAS_FOR(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__IDEA_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("away"), py_str("somewhere"), py_str("abroad")]), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    covered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push(op_add(&g_FREE_TIME(), &g_WEEK_OFF())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _covered_words(args))(&__args) };
    here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __kws.extend(vec![("block".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_place_the_block_is_at(args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&covered.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&here.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_contains(&g__TIME_WORDS(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NO_SUBJECT(), &geti(&words.clone(), &Py::Int(0)))) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _about_a_thing_the_person_has(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("True when the line asks what to do about an ache or a complaint of its own.\n\n    Three things have to hold, and each of them keeps a different line out. The subject has to\n    be named: \"i have toothache at the weekend, what can i do\" has one and \"ideas for a weekend\n    when you are ill\" does not. The topic has to be the body: \"ive got half a cabbage left from\n    sunday, what can i do with it\" is food, not a complaint. And the line has to ask what to do\n    about it: \"i have nothing planned this weekend and it is cold\" names a subject (the reading\n    calls the weather a cold) but asks about the weekend, and the plan is what it came for.\n    ");
    reading = { let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&g__A_THING_THE_PERSON_HAS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic_own")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_words(args))(&__args) }); __args.push(g__ASKS_ABOUT_IT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _holds(args))(&__args) };
    Py::None
}
pub fn _a_day_out_is_the_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("False when the line asks what to do about a thing of its own rather than about a free day.\n\n    \"any ideas for a film tonight\" names a film, \"i have toothache at the weekend, what can i do\"\n    names the toothache and \"ive got half a cabbage left from sunday, what can i do with it\" names\n    the cabbage: both rows here are for a line whose subject is the time or the money.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ideas_about_a_thing(args))(&__args) }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_a_thing_the_person_has(args))(&__args) }));
    Py::None
}
pub fn _short_run(args: &[Py]) -> Py {
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
    let _ = py_str("The counted run of days in the line, unless the line says what the time is for.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SHORT_RUN(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PURPOSE_AFTER_RUN(), "search", args))(&__args) }) {
    return Py::None;
    }
    return found.clone();
    Py::None
}
pub fn _the_place_the_block_is_at(args: &[Py]) -> Py {
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
    let mut block: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "block") { v.clone() }
    else { Py::Bool(false) };
    let mut here: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut preps: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the room or corner the free block is spent in, if the line names one.\n\n    \"in the garden\", \"at home\", \"on the balcony\", \"in the back garden\": the time is spent\n    there, and what to do with it is still the question, so the plan is the answer. Read word\n    by word rather than by regex, because a place is often two words, the first of them alone\n    (\"back\", \"front\", \"living\") is not the place, and one match must not eat the next.\n\n    `block` says the line holds a free block, which is what makes \"for the garden\" a place the\n    time is spent in rather than the thing it is about.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    preps = op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__AT_THE_PLACE_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &if py_truthy(&block.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__FOR_THE_PLACE_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } });
    here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&preps.clone(), &word.clone()))) {
    continue;
    }
    j = op_add(&i.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = op_lt(&j.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__THE_OWNER(), &geti(&words.clone(), &j.clone()))) } }) {
    let __aug3 = op_add(&j.clone(), &Py::Int(1));
    j = __aug3;
    }
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &j.clone(), &op_add(&j.clone(), &Py::Int(2)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g__AT_HOME(), &pair.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pair.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&here.clone(), "update", args))(&__args) };
    } else {
    if py_truthy(&{ let __b = op_lt(&j.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__AT_HOME(), &geti(&words.clone(), &j.clone()))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &j.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&here.clone(), "add", args))(&__args) };
    }
    }
    }
    return here.clone();
    Py::None
}
pub fn _names_the_free_block(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line's subject is the free block itself rather than a thing of its own.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = named.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__FREE_BLOCK_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__ASK_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__BLOCK_QUALIFIERS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__COUNT_WORDS(), &word.clone())) } } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _covered_words(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut covered: Py = Py::None;
    let mut left: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of the phrases of `phrases` this line holds, counted once each.\n\n    A free block said as a phrase (\"after work\", \"week off work\", \"day off tomorrow\") brings\n    words of its own that name no thing: the plan is still the answer to \"what should i do\n    after work\". They come off the line before what is left is read as a thing.\n    ");
    covered = py_dict(vec![]);
    left = op_modulo(&py_str(" %s "), &text.clone());
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrases.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    pattern = op_modulo(&py_str("(?<![a-z0-9])%s(?![a-z0-9])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    while py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(py_str(" ")); __args.push(left.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    let _ = py_setitem(&covered.clone(), &word.clone(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&covered.clone(), "get", args))(&__args) }, &Py::Int(1)));
    }
    }
    }
    return covered.clone();
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut covered: Py = Py::None;
    let mut here: Py = Py::None;
    let mut holds_block: Py = Py::None;
    let mut left: Py = Py::None;
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line names a thing the plan is not about.\n\n    A free block plus an ask is this card's question (\"what should i do this weekend\", \"any\n    ideas for a rainy day\"), and every other word of such a line either says when it is or\n    what it is for. A line that carries a word outside that vocabulary is asking about that\n    word: \"any ideas for a film tonight\" is a film and \"give me three ideas for a cheap date\"\n    is a date - both were answered with a plan for the evening or for a free day instead.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keywords")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "replace", args))(&__args) }); } } py_set(__out) })() };
    if py_truthy(&op_bitand(&words.clone(), &g__JOB_WORDS())) {
    return Py::Bool(false);
    }
    said = op_modulo(&py_str(" %s "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) });
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MUSING(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    holds_block = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(op_add(&op_add(&g_FREE_TIME(), &g_WEEK_OFF()), &g__WORK_BLOCK())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _covered_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FREE_TIME_TIME())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("block".to_string(), holds_block.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_place_the_block_is_at(args))(&__args) };
    shown = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subject")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(shown.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { shown.clone() } }) {
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shown.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&here.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = left.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(left.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_the_free_block(args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    }
    covered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(op_add(&op_add(&op_add(&op_add(&g_FREE_TIME(), &g_WEEK_OFF()), &g_FREE_TIME_ASK()), &g__ASKS()), &g__WORK_BLOCK())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _covered_words(args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keywords")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__FREE_BLOCK_WORDS(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__ASK_WORDS(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__BLOCK_QUALIFIERS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__COUNT_WORDS(), &low.clone())) } } } } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "isdigit", args))(&__args) }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&here.clone(), &low.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&covered.clone(), "get", args))(&__args) }) {
    let __aug2 = op_sub(&geti(&covered.clone(), &low.clone()), &Py::Int(1));
    let _ = py_setitem(&covered.clone(), &low.clone(), __aug2);
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_as_the_thing_it_does_not_want(args))(&__args) }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_as_the_thing_it_does_not_want(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("Whether this word comes into the line as the thing the person wants to get away from.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("(?<![a-z0-9])%s(?![a-z0-9])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "start", args))(&__args) }, &Py::None), "lower", args))(&__args) }, "split", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = slice_or_raise(&before.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&before.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__NOT_THE_THING()) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _free_time(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    line = op_add(&op_add(&said.clone(), &py_str(" ")), &clean.clone());
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::dishes::a_dish_is_already_the_plan(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__UNWELL()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z0-9])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z0-9])"))); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COLD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COLD_WEATHER(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WHERE_GO_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g_MONEY_WORDS(), &g__MONEY_ONLY()), &g__WHEN_AWAY())) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__TRIP_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MONTHS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_city_is_named(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _dish_for_tonight(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOTE_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EVENT_AWAY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EVENT_EVENING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MEDIUM_NAMED()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("newborn"), py_str("new baby"), py_str("new-born"), py_str("just had a baby"), py_str("had a baby"), py_str("baby is due"), py_str("baby due"), py_str("paternity"), py_str("maternity leave"), py_str("on maternity")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_NEWBORN_WEEK();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__REQUESTING_TIME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WORK_BLOCK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _short_run(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ideas_about_a_thing(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _about_a_thing_the_person_has(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_thing_of_its_own(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GIFT_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("this afternoon"), py_str("free afternoon"), py_str("afternoon off")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = g_DAY_PLAN();
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("day off"), py_str("days off"), py_str("free day")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = g_DAY_OFF_PLAN();
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__THE_EVENING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = g_EVENING_PLAN();
    } else {
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _short_run(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("few days"), py_str("couple of days")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("week off"), py_str("a week off"), py_str("weeks off"), py_str("whole week"), py_str("week free")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    plan = g_DAY_PLAN();
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WEEK_OFF()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = g_WEEK_PLAN();
    } else {
    plan = g_WEEKEND_PLAN();
    }
    }
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&plan.clone(), &g_WEEK_PLAN())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_MONEY_WORDS(), &g__MONEY_ONLY())) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    plan = g_WEEK_PLAN_SKINT();
    }
    if py_truthy(&Py::Bool(py_is(&plan.clone(), &g_WEEK_PLAN()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("summer"), py_str("hot"), py_str("heat"), py_str("sunny"), py_str("sunshine")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = op_add(&op_add(&plan.clone(), &py_str(" ")), &g_SUMMER_WEEK());
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("winter"), py_str("autumn"), py_str("frost"), py_str("snow"), py_str("dark")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = op_add(&op_add(&plan.clone(), &py_str(" ")), &g_WINTER_WEEK());
    }
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MONEY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plan = op_add(&op_add(&plan.clone(), &py_str(" ")), &g_NO_MONEY());
    }
    return plan.clone();
    Py::None
}
pub fn _gear_named(args: &[Py]) -> Py {
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
    let mut _said: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The pieces of gear the line names, in the order they are in the table.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BABY_GEAR()) { let __unpk6765 = py_unpack(&__item.clone(), 3, None); let mut keys: Py = __unpk6765[0].clone(); let mut name: Py = __unpk6765[1].clone(); let mut _said: Py = __unpk6765[2].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(name.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _has_word(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z0-9])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z0-9])"))); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    Py::None
}
pub fn _a_piece_of_baby_gear(args: &[Py]) -> Py {
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
    let mut _keys: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut fault: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The one piece the line is about, answered first, or the whole first shop.\n\n    Every line that reached this row used to get the entire list, so the price of a pram opened\n    on a cot mattress and a pick between a pram and a carrier was not answered at all.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words_of(args))(&__args) };
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gear_named(args))(&__args) };
    fault = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BABY_GEAR_FAULT()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has_word(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = fault.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named.clone())) } }) {
    carried = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    topic = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&carried.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gear_named(args))(&__args) };
    }
    if py_truthy(&{ let __b = fault.clone(); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    return g__BABY_GEAR_MEND();
    }
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("which"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("better"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&op_modulo(&py_str(" %s "), &words.clone()), &py_str(" or "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("versus"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("vs"))) } } } } } } } } } }) {
    said = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g__BABY_GEAR()) { let __unpk2853 = py_unpack(&__item.clone(), 3, None); let mut _keys: Py = __unpk2853[0].clone(); let mut name: Py = __unpk2853[1].clone(); let mut sentence: Py = __unpk2853[2].clone(); __out.push((name.clone(), sentence.clone())); } py_dict(__out) })() };
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&named.clone(), &Py::None, &Py::Int(2), &Py::None)) { let mut name: Py = __item.clone(); __out.push(geti(&said.clone(), &name.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&py_str("Whichever you use most is the one to spend on. "), &pair.clone()), "strip", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    let __iter1 = py_iter_or_raise(&g__BABY_GEAR());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    keys = __unpk2[0].clone();
    name = __unpk2[1].clone();
    sentence = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &geti(&named.clone(), &Py::Int(0))))) {
    return sentence.clone();
    }
    }
    }
    return py_str("Buy second-hand for all of it except the mattress: a cot (mattress 120 by 60cm) or cot bed (140 by 70cm) with a new firm mattress that fits with no gaps, a car seat that has not been in a crash and whose history you know - i-Size (R129) is the newer standard, and how it fits your car matters more than the brand - and a pram that folds small enough for your car and lies flat for a newborn. Prices: a new pram £150 to £400, second-hand £60 to £150, a cot £100 to £250 new. You need little else at the start (six babygros in two sizes, nappies, muslins); a monitor is a comfort rather than a must, and a high chair can wait until six months.");
    Py::None
}
pub fn _words_of(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The line, and the words it was read as, in one lowered string.");
    parts = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }]);
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&parts.clone(), "extend", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    Py::None
}
pub fn _dish_for_tonight(args: &[Py]) -> Py {
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
    let mut alias: Py = Py::None;
    let mut aliases: Py = Py::None;
    let mut avoid: Py = Py::None;
    let mut body: Py = Py::None;
    let mut found: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The recipe for the dish the line names for an evening, or None.\n\n    Only the names cooking.RECIPES already holds, so this row answers with the same words the\n    kitchen table does: \"curry\" is the 40-minute curry card, and a dish nobody has written a\n    card for is left to the module that might still know it.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words_of(args))(&__args) };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::knowledge::cooking::g_RECIPES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    aliases = __unpk2[0].clone();
    keys = __unpk2[1].clone();
    body = __unpk2[2].clone();
    avoid = __unpk2[3].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&aliases.clone()) { let mut alias: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alias.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&avoid.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&aliases.clone()) { let mut alias: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alias.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(alias.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, body.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __kws.extend(vec![("key".to_string(), { let alias = alias.clone(); let aliases = aliases.clone(); let avoid = avoid.clone(); let body = body.clone(); let found = found.clone(); let keys = keys.clone(); let memory = memory.clone(); let said = said.clone(); let understood = understood.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let alias = alias.clone(); let aliases = aliases.clone(); let avoid = avoid.clone(); let body = body.clone(); let found = found.clone(); let keys = keys.clone(); let memory = memory.clone(); let said = said.clone(); let understood = understood.clone(); let word = word.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); geti(&pair.clone(), &Py::Int(0)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1));
    Py::None
}
pub fn _a_milestone_said_as_an_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("True when the line is somebody's birthday, said as an age and a day.\n\n    The age is a round one and a grown-up one, a day has to be near, and the person has to be\n    named: an age said of a boiler or a car is not a do, and a child's birthday has its own plan.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MILESTONE_PERSON(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MILESTONE_AGE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&age.clone()))) {
    return Py::Bool(false);
    }
    years = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&age.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_lt(&years.clone(), &Py::Int(18)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CHILD_AT_A_MILESTONE(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PRESENT_ASKED(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MILESTONE_SOON(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _party_plan(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut ages: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The grown-up party plan, or the children's one when the line names a child.\n\n    \"how do i plan a birthday party for my mum\" and \"...for my son\" reach the same entry, and\n    an age-plus-one answer is wrong for one of them. The age in the line settles it when it is\n    there (\"my sons 40th\" is a grown-up do), and a child word settles it when it is not.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    line = op_add(&op_add(&clean.clone(), &py_str(" ")), &words.clone());
    ages = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AGE_AT_PARTY(), "findall", args))(&__args) }) { let mut found: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(99)) } else { Py::Bool(false) } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&ages.clone()) {
    return if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(13))) { g__KIDS_PARTY() } else { g__ADULT_PARTY() };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CHILD_AT_PARTY()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b"))); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g__KIDS_PARTY();
    }
    return g__ADULT_PARTY();
    Py::None
}
pub fn _not_a_gift_ask(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is asking for a thing to give rather than for the do itself.\n\n    \"what should i get my mum for her birthday\" holds \"birthday\" and an ask, which is enough\n    for the party row, and the gift and the guest list are different answers. \"how many people\n    should i invite to my mums 60th\" names the do and not a present, so it keeps the plan.\n    ");
    line = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    let __aug1 = op_add(&line.clone(), &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &py_str(" ")));
    line = __aug1;
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GIFT_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _not_a_gift_or_a_date(args: &[Py]) -> Py {
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
    let _ = py_str("Both of the party row's stand-downs, so the row keeps neither a present nor a date.\n\n    The date question and the gift question arrive in the same words (\"what should i do for my\n    mum's birthday this weekend\" is neither), so the row asks both before it answers.\n    ");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_a_gift_ask(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_a_date_question(args))(&__args) } } };
    Py::None
}
pub fn _a_room_plan(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The room the line names, planned in its own terms, or None for a line that names none.\n\n    \"how do i plan a new bathroom\" was answered with the kitchen's work triangle, because one\n    card served both rooms and a kitchen's numbers are not a bathroom's.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z0-9])(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_PLANNED_ROOM()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")(?![a-z0-9])"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("utility"), py_str("laundry"), py_str("boot room")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g__UTILITY_PLAN();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("bath"), py_str("bathroom"), py_str("shower"), py_str("toilet"), py_str("loo"), py_str("ensuite"), py_str("wet room")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g__BATHROOM_PLAN();
    }
    return g__KITCHEN_PLAN();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.plans".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANNOUNCED_DISH" => Some(g_ANNOUNCED_DISH()),
            "ANSWERS" => Some(g_ANSWERS()),
            "AT_HOME" => Some(g_AT_HOME()),
            "A_DAY_CONDITION" => Some(g_A_DAY_CONDITION()),
            "A_HOURS_ASK" => Some(g_A_HOURS_ASK()),
            "A_WEATHER_ASK" => Some(g_A_WEATHER_ASK()),
            "DAY_OFF_PLAN" => Some(g_DAY_OFF_PLAN()),
            "DAY_PLAN" => Some(g_DAY_PLAN()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EVENING_PLAN" => Some(g_EVENING_PLAN()),
            "FREE_TIME" => Some(g_FREE_TIME()),
            "FREE_TIME_ASK" => Some(g_FREE_TIME_ASK()),
            "FREE_TIME_CUE" => Some(g_FREE_TIME_CUE()),
            "FREE_TIME_TIME" => Some(g_FREE_TIME_TIME()),
            "FREE_TIME_WHO" => Some(g_FREE_TIME_WHO()),
            "FREE_TIME_WORDS" => Some(g_FREE_TIME_WORDS()),
            "FREE_WEEKEND" => Some(g_FREE_WEEKEND()),
            "HOLIDAY_PREFIX" => Some(g_HOLIDAY_PREFIX()),
            "MEDICINE_NEEDS_A_REASON" => Some(g_MEDICINE_NEEDS_A_REASON()),
            "MONEY_WORDS" => Some(g_MONEY_WORDS()),
            "NEWBORN_WEEK" => Some(g_NEWBORN_WEEK()),
            "NO_MONEY" => Some(g_NO_MONEY()),
            "PEOPLE_FEMALE" => Some(g_PEOPLE_FEMALE()),
            "PEOPLE_MALE" => Some(g_PEOPLE_MALE()),
            "PLAN_ASK" => Some(g_PLAN_ASK()),
            "SUMMER_WEEK" => Some(g_SUMMER_WEEK()),
            "TRY_ONCE" => Some(g_TRY_ONCE()),
            "TRY_ONCE_ASK" => Some(g_TRY_ONCE_ASK()),
            "TRY_ONCE_MORE" => Some(g_TRY_ONCE_MORE()),
            "VAGUE_TOPICS" => Some(g_VAGUE_TOPICS()),
            "WEEKEND_PLAN" => Some(g_WEEKEND_PLAN()),
            "WEEK_OFF" => Some(g_WEEK_OFF()),
            "WEEK_PLAN" => Some(g_WEEK_PLAN()),
            "WEEK_PLAN_SKINT" => Some(g_WEEK_PLAN_SKINT()),
            "WINTER_WEEK" => Some(g_WINTER_WEEK()),
            "_ADULT_PARTY" => Some(g__ADULT_PARTY()),
            "_AGE_AT_PARTY" => Some(g__AGE_AT_PARTY()),
            "_AMBIGUOUS_CITY_WORDS" => Some(g__AMBIGUOUS_CITY_WORDS()),
            "_AN_IDEA_ASKED_FOR" => Some(g__AN_IDEA_ASKED_FOR()),
            "_ASKS" => Some(g__ASKS()),
            "_ASKS_ABOUT_IT" => Some(g__ASKS_ABOUT_IT()),
            "_ASKS_A_CLOCK" => Some(g__ASKS_A_CLOCK()),
            "_ASKS_THE_WORLD" => Some(g__ASKS_THE_WORLD()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_AT_HOME" => Some(g__AT_HOME()),
            "_AT_THE_PLACE_WORDS" => Some(g__AT_THE_PLACE_WORDS()),
            "_A_CHILD_AT_A_MILESTONE" => Some(g__A_CHILD_AT_A_MILESTONE()),
            "_A_DATE_ASK" => Some(g__A_DATE_ASK()),
            "_A_DO" => Some(g__A_DO()),
            "_A_MILESTONE_AGE" => Some(g__A_MILESTONE_AGE()),
            "_A_MILESTONE_PERSON" => Some(g__A_MILESTONE_PERSON()),
            "_A_MILESTONE_SOON" => Some(g__A_MILESTONE_SOON()),
            "_A_MOVE_ELSEWHERE" => Some(g__A_MOVE_ELSEWHERE()),
            "_A_MOVE_TO_PLAN" => Some(g__A_MOVE_TO_PLAN()),
            "_A_PLANNED_ROOM" => Some(g__A_PLANNED_ROOM()),
            "_A_PRESENT_ASKED" => Some(g__A_PRESENT_ASKED()),
            "_A_PRESENT_FOR" => Some(g__A_PRESENT_FOR()),
            "_A_THING_THE_PERSON_HAS" => Some(g__A_THING_THE_PERSON_HAS()),
            "_BABY_GEAR" => Some(g__BABY_GEAR()),
            "_BABY_GEAR_FAULT" => Some(g__BABY_GEAR_FAULT()),
            "_BABY_GEAR_MEND" => Some(g__BABY_GEAR_MEND()),
            "_BARE_RID" => Some(g__BARE_RID()),
            "_BATHROOM_PLAN" => Some(g__BATHROOM_PLAN()),
            "_BLOCK_QUALIFIERS" => Some(g__BLOCK_QUALIFIERS()),
            "_BRING_IT" => Some(g__BRING_IT()),
            "_CHILD_AT_PARTY" => Some(g__CHILD_AT_PARTY()),
            "_COLD" => Some(g__COLD()),
            "_COLD_WEATHER" => Some(g__COLD_WEATHER()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_DECLUTTER_ASK" => Some(g__DECLUTTER_ASK()),
            "_DECLUTTER_ROOM" => Some(g__DECLUTTER_ROOM()),
            "_DECLUTTER_STUFF" => Some(g__DECLUTTER_STUFF()),
            "_EVENT_AWAY" => Some(g__EVENT_AWAY()),
            "_EVENT_EVENING" => Some(g__EVENT_EVENING()),
            "_FOR_A_RAINY_DAY" => Some(g__FOR_A_RAINY_DAY()),
            "_FOR_THE_PLACE_WORDS" => Some(g__FOR_THE_PLACE_WORDS()),
            "_FREE_BLOCK_WORDS" => Some(g__FREE_BLOCK_WORDS()),
            "_GIFT_ASK" => Some(g__GIFT_ASK()),
            "_HOME_IN_THE_LINE" => Some(g__HOME_IN_THE_LINE()),
            "_IDEAS_ASK" => Some(g__IDEAS_ASK()),
            "_IDEAS_FOR" => Some(g__IDEAS_FOR()),
            "_IDEA_FILLER" => Some(g__IDEA_FILLER()),
            "_IDEA_NONE" => Some(g__IDEA_NONE()),
            "_JOB_NAMED" => Some(g__JOB_NAMED()),
            "_JOB_WORDS" => Some(g__JOB_WORDS()),
            "_KIDS_PARTY" => Some(g__KIDS_PARTY()),
            "_KITCHEN_PLAN" => Some(g__KITCHEN_PLAN()),
            "_MEDIUM_NAMED" => Some(g__MEDIUM_NAMED()),
            "_MILESTONE_AGES" => Some(g__MILESTONE_AGES()),
            "_MILESTONE_SAID" => Some(g__MILESTONE_SAID()),
            "_MONEY_ONLY" => Some(g__MONEY_ONLY()),
            "_MONEY_WITH_IT" => Some(g__MONEY_WITH_IT()),
            "_MONTHS" => Some(g__MONTHS()),
            "_MUSING" => Some(g__MUSING()),
            "_NOTE_ASK" => Some(g__NOTE_ASK()),
            "_NOT_THE_THING" => Some(g__NOT_THE_THING()),
            "_NO_SUBJECT" => Some(g__NO_SUBJECT()),
            "_OWN_ACTIVITY" => Some(g__OWN_ACTIVITY()),
            "_OWN_COUNT" => Some(g__OWN_COUNT()),
            "_PEOPLE" => Some(g__PEOPLE()),
            "_PURPOSE_AFTER_RUN" => Some(g__PURPOSE_AFTER_RUN()),
            "_REQUESTING_TIME" => Some(g__REQUESTING_TIME()),
            "_SHORT_RUN" => Some(g__SHORT_RUN()),
            "_SWALLOWED" => Some(g__SWALLOWED()),
            "_SWALLOWED_THING" => Some(g__SWALLOWED_THING()),
            "_THE_EVENING" => Some(g__THE_EVENING()),
            "_THE_OWNER" => Some(g__THE_OWNER()),
            "_THE_WORLD" => Some(g__THE_WORLD()),
            "_TIME_WORDS" => Some(g__TIME_WORDS()),
            "_TODAY_WORDS" => Some(g__TODAY_WORDS()),
            "_TOO_MUCH" => Some(g__TOO_MUCH()),
            "_TRIP_PHRASES" => Some(g__TRIP_PHRASES()),
            "_UNWELL" => Some(g__UNWELL()),
            "_UTILITY_PLAN" => Some(g__UTILITY_PLAN()),
            "_WEDDING_ASK" => Some(g__WEDDING_ASK()),
            "_WEDDING_COUNT" => Some(g__WEDDING_COUNT()),
            "_WEDDING_GUESTS" => Some(g__WEDDING_GUESTS()),
            "_WEDDING_LEAD_TIME" => Some(g__WEDDING_LEAD_TIME()),
            "_WEDDING_MONEY" => Some(g__WEDDING_MONEY()),
            "_WEDDING_PAPERS" => Some(g__WEDDING_PAPERS()),
            "_WEDDING_WHEN" => Some(g__WEDDING_WHEN()),
            "_WEEKEND_WORDS" => Some(g__WEEKEND_WORDS()),
            "_WHAT_HAPPENED" => Some(g__WHAT_HAPPENED()),
            "_WHEN_AWAY" => Some(g__WHEN_AWAY()),
            "_WHERE_GO_PHRASES" => Some(g__WHERE_GO_PHRASES()),
            "_WORK_BLOCK" => Some(g__WORK_BLOCK()),
            "_a_city_is_named" => Some(py_func("_a_city_is_named", |args: &[Py]| _a_city_is_named(args))),
            "_a_day_out_is_the_question" => Some(py_func("_a_day_out_is_the_question", |args: &[Py]| _a_day_out_is_the_question(args))),
            "_a_free_weekend" => Some(py_func("_a_free_weekend", |args: &[Py]| _a_free_weekend(args))),
            "_a_household_move" => Some(py_func("_a_household_move", |args: &[Py]| _a_household_move(args))),
            "_a_household_move_or_a_named_move" => Some(py_func("_a_household_move_or_a_named_move", |args: &[Py]| _a_household_move_or_a_named_move(args))),
            "_a_medicine_line" => Some(py_func("_a_medicine_line", |args: &[Py]| _a_medicine_line(args))),
            "_a_milestone_said_as_an_age" => Some(py_func("_a_milestone_said_as_an_age", |args: &[Py]| _a_milestone_said_as_an_age(args))),
            "_a_piece_of_baby_gear" => Some(py_func("_a_piece_of_baby_gear", |args: &[Py]| _a_piece_of_baby_gear(args))),
            "_a_rainy_day_is_money" => Some(py_func("_a_rainy_day_is_money", |args: &[Py]| _a_rainy_day_is_money(args))),
            "_a_room_plan" => Some(py_func("_a_room_plan", |args: &[Py]| _a_room_plan(args))),
            "_a_stretch_of_time_to_fill" => Some(py_func("_a_stretch_of_time_to_fill", |args: &[Py]| _a_stretch_of_time_to_fill(args))),
            "_about_a_thing_the_person_has" => Some(py_func("_about_a_thing_the_person_has", |args: &[Py]| _about_a_thing_the_person_has(args))),
            "_asks_about_the_world" => Some(py_func("_asks_about_the_world", |args: &[Py]| _asks_about_the_world(args))),
            "_asks_for_ideas" => Some(py_func("_asks_for_ideas", |args: &[Py]| _asks_for_ideas(args))),
            "_covered_words" => Some(py_func("_covered_words", |args: &[Py]| _covered_words(args))),
            "_declutter_fits" => Some(py_func("_declutter_fits", |args: &[Py]| _declutter_fits(args))),
            "_dish_for_tonight" => Some(py_func("_dish_for_tonight", |args: &[Py]| _dish_for_tonight(args))),
            "_free_time" => Some(py_func("_free_time", |args: &[Py]| _free_time(args))),
            "_free_weekend_fits" => Some(py_func("_free_weekend_fits", |args: &[Py]| _free_weekend_fits(args))),
            "_gear_named" => Some(py_func("_gear_named", |args: &[Py]| _gear_named(args))),
            "_has_word" => Some(py_func("_has_word", |args: &[Py]| _has_word(args))),
            "_holds" => Some(py_func("_holds", |args: &[Py]| _holds(args))),
            "_ideas_about_a_thing" => Some(py_func("_ideas_about_a_thing", |args: &[Py]| _ideas_about_a_thing(args))),
            "_names_a_thing_of_its_own" => Some(py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args))),
            "_names_the_free_block" => Some(py_func("_names_the_free_block", |args: &[Py]| _names_the_free_block(args))),
            "_not_a_count_of_their_own" => Some(py_func("_not_a_count_of_their_own", |args: &[Py]| _not_a_count_of_their_own(args))),
            "_not_a_date_question" => Some(py_func("_not_a_date_question", |args: &[Py]| _not_a_date_question(args))),
            "_not_a_deed_of_their_own" => Some(py_func("_not_a_deed_of_their_own", |args: &[Py]| _not_a_deed_of_their_own(args))),
            "_not_a_gift_ask" => Some(py_func("_not_a_gift_ask", |args: &[Py]| _not_a_gift_ask(args))),
            "_not_a_gift_or_a_date" => Some(py_func("_not_a_gift_or_a_date", |args: &[Py]| _not_a_gift_or_a_date(args))),
            "_party_plan" => Some(py_func("_party_plan", |args: &[Py]| _party_plan(args))),
            "_said_as_the_thing_it_does_not_want" => Some(py_func("_said_as_the_thing_it_does_not_want", |args: &[Py]| _said_as_the_thing_it_does_not_want(args))),
            "_said_now" => Some(py_func("_said_now", |args: &[Py]| _said_now(args))),
            "_said_words" => Some(py_func("_said_words", |args: &[Py]| _said_words(args))),
            "_short_run" => Some(py_func("_short_run", |args: &[Py]| _short_run(args))),
            "_the_free_weekend_fits_and_is_asked" => Some(py_func("_the_free_weekend_fits_and_is_asked", |args: &[Py]| _the_free_weekend_fits_and_is_asked(args))),
            "_the_place_the_block_is_at" => Some(py_func("_the_place_the_block_is_at", |args: &[Py]| _the_place_the_block_is_at(args))),
            "_try_once" => Some(py_func("_try_once", |args: &[Py]| _try_once(args))),
            "_try_once_more" => Some(py_func("_try_once_more", |args: &[Py]| _try_once_more(args))),
            "_wedding_plan" => Some(py_func("_wedding_plan", |args: &[Py]| _wedding_plan(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANNOUNCED_DISH" => s_ANNOUNCED_DISH(v),
            "ANSWERS" => s_ANSWERS(v),
            "AT_HOME" => s_AT_HOME(v),
            "A_DAY_CONDITION" => s_A_DAY_CONDITION(v),
            "A_HOURS_ASK" => s_A_HOURS_ASK(v),
            "A_WEATHER_ASK" => s_A_WEATHER_ASK(v),
            "DAY_OFF_PLAN" => s_DAY_OFF_PLAN(v),
            "DAY_PLAN" => s_DAY_PLAN(v),
            "ENTRIES" => s_ENTRIES(v),
            "EVENING_PLAN" => s_EVENING_PLAN(v),
            "FREE_TIME" => s_FREE_TIME(v),
            "FREE_TIME_ASK" => s_FREE_TIME_ASK(v),
            "FREE_TIME_CUE" => s_FREE_TIME_CUE(v),
            "FREE_TIME_TIME" => s_FREE_TIME_TIME(v),
            "FREE_TIME_WHO" => s_FREE_TIME_WHO(v),
            "FREE_TIME_WORDS" => s_FREE_TIME_WORDS(v),
            "FREE_WEEKEND" => s_FREE_WEEKEND(v),
            "HOLIDAY_PREFIX" => s_HOLIDAY_PREFIX(v),
            "MEDICINE_NEEDS_A_REASON" => s_MEDICINE_NEEDS_A_REASON(v),
            "MONEY_WORDS" => s_MONEY_WORDS(v),
            "NEWBORN_WEEK" => s_NEWBORN_WEEK(v),
            "NO_MONEY" => s_NO_MONEY(v),
            "PEOPLE_FEMALE" => s_PEOPLE_FEMALE(v),
            "PEOPLE_MALE" => s_PEOPLE_MALE(v),
            "PLAN_ASK" => s_PLAN_ASK(v),
            "SUMMER_WEEK" => s_SUMMER_WEEK(v),
            "TRY_ONCE" => s_TRY_ONCE(v),
            "TRY_ONCE_ASK" => s_TRY_ONCE_ASK(v),
            "TRY_ONCE_MORE" => s_TRY_ONCE_MORE(v),
            "VAGUE_TOPICS" => s_VAGUE_TOPICS(v),
            "WEEKEND_PLAN" => s_WEEKEND_PLAN(v),
            "WEEK_OFF" => s_WEEK_OFF(v),
            "WEEK_PLAN" => s_WEEK_PLAN(v),
            "WEEK_PLAN_SKINT" => s_WEEK_PLAN_SKINT(v),
            "WINTER_WEEK" => s_WINTER_WEEK(v),
            "_ADULT_PARTY" => s__ADULT_PARTY(v),
            "_AGE_AT_PARTY" => s__AGE_AT_PARTY(v),
            "_AMBIGUOUS_CITY_WORDS" => s__AMBIGUOUS_CITY_WORDS(v),
            "_AN_IDEA_ASKED_FOR" => s__AN_IDEA_ASKED_FOR(v),
            "_ASKS" => s__ASKS(v),
            "_ASKS_ABOUT_IT" => s__ASKS_ABOUT_IT(v),
            "_ASKS_A_CLOCK" => s__ASKS_A_CLOCK(v),
            "_ASKS_THE_WORLD" => s__ASKS_THE_WORLD(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_AT_HOME" => s__AT_HOME(v),
            "_AT_THE_PLACE_WORDS" => s__AT_THE_PLACE_WORDS(v),
            "_A_CHILD_AT_A_MILESTONE" => s__A_CHILD_AT_A_MILESTONE(v),
            "_A_DATE_ASK" => s__A_DATE_ASK(v),
            "_A_DO" => s__A_DO(v),
            "_A_MILESTONE_AGE" => s__A_MILESTONE_AGE(v),
            "_A_MILESTONE_PERSON" => s__A_MILESTONE_PERSON(v),
            "_A_MILESTONE_SOON" => s__A_MILESTONE_SOON(v),
            "_A_MOVE_ELSEWHERE" => s__A_MOVE_ELSEWHERE(v),
            "_A_MOVE_TO_PLAN" => s__A_MOVE_TO_PLAN(v),
            "_A_PLANNED_ROOM" => s__A_PLANNED_ROOM(v),
            "_A_PRESENT_ASKED" => s__A_PRESENT_ASKED(v),
            "_A_PRESENT_FOR" => s__A_PRESENT_FOR(v),
            "_A_THING_THE_PERSON_HAS" => s__A_THING_THE_PERSON_HAS(v),
            "_BABY_GEAR" => s__BABY_GEAR(v),
            "_BABY_GEAR_FAULT" => s__BABY_GEAR_FAULT(v),
            "_BABY_GEAR_MEND" => s__BABY_GEAR_MEND(v),
            "_BARE_RID" => s__BARE_RID(v),
            "_BATHROOM_PLAN" => s__BATHROOM_PLAN(v),
            "_BLOCK_QUALIFIERS" => s__BLOCK_QUALIFIERS(v),
            "_BRING_IT" => s__BRING_IT(v),
            "_CHILD_AT_PARTY" => s__CHILD_AT_PARTY(v),
            "_COLD" => s__COLD(v),
            "_COLD_WEATHER" => s__COLD_WEATHER(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_DECLUTTER_ASK" => s__DECLUTTER_ASK(v),
            "_DECLUTTER_ROOM" => s__DECLUTTER_ROOM(v),
            "_DECLUTTER_STUFF" => s__DECLUTTER_STUFF(v),
            "_EVENT_AWAY" => s__EVENT_AWAY(v),
            "_EVENT_EVENING" => s__EVENT_EVENING(v),
            "_FOR_A_RAINY_DAY" => s__FOR_A_RAINY_DAY(v),
            "_FOR_THE_PLACE_WORDS" => s__FOR_THE_PLACE_WORDS(v),
            "_FREE_BLOCK_WORDS" => s__FREE_BLOCK_WORDS(v),
            "_GIFT_ASK" => s__GIFT_ASK(v),
            "_HOME_IN_THE_LINE" => s__HOME_IN_THE_LINE(v),
            "_IDEAS_ASK" => s__IDEAS_ASK(v),
            "_IDEAS_FOR" => s__IDEAS_FOR(v),
            "_IDEA_FILLER" => s__IDEA_FILLER(v),
            "_IDEA_NONE" => s__IDEA_NONE(v),
            "_JOB_NAMED" => s__JOB_NAMED(v),
            "_JOB_WORDS" => s__JOB_WORDS(v),
            "_KIDS_PARTY" => s__KIDS_PARTY(v),
            "_KITCHEN_PLAN" => s__KITCHEN_PLAN(v),
            "_MEDIUM_NAMED" => s__MEDIUM_NAMED(v),
            "_MILESTONE_AGES" => s__MILESTONE_AGES(v),
            "_MILESTONE_SAID" => s__MILESTONE_SAID(v),
            "_MONEY_ONLY" => s__MONEY_ONLY(v),
            "_MONEY_WITH_IT" => s__MONEY_WITH_IT(v),
            "_MONTHS" => s__MONTHS(v),
            "_MUSING" => s__MUSING(v),
            "_NOTE_ASK" => s__NOTE_ASK(v),
            "_NOT_THE_THING" => s__NOT_THE_THING(v),
            "_NO_SUBJECT" => s__NO_SUBJECT(v),
            "_OWN_ACTIVITY" => s__OWN_ACTIVITY(v),
            "_OWN_COUNT" => s__OWN_COUNT(v),
            "_PEOPLE" => s__PEOPLE(v),
            "_PURPOSE_AFTER_RUN" => s__PURPOSE_AFTER_RUN(v),
            "_REQUESTING_TIME" => s__REQUESTING_TIME(v),
            "_SHORT_RUN" => s__SHORT_RUN(v),
            "_SWALLOWED" => s__SWALLOWED(v),
            "_SWALLOWED_THING" => s__SWALLOWED_THING(v),
            "_THE_EVENING" => s__THE_EVENING(v),
            "_THE_OWNER" => s__THE_OWNER(v),
            "_THE_WORLD" => s__THE_WORLD(v),
            "_TIME_WORDS" => s__TIME_WORDS(v),
            "_TODAY_WORDS" => s__TODAY_WORDS(v),
            "_TOO_MUCH" => s__TOO_MUCH(v),
            "_TRIP_PHRASES" => s__TRIP_PHRASES(v),
            "_UNWELL" => s__UNWELL(v),
            "_UTILITY_PLAN" => s__UTILITY_PLAN(v),
            "_WEDDING_ASK" => s__WEDDING_ASK(v),
            "_WEDDING_COUNT" => s__WEDDING_COUNT(v),
            "_WEDDING_GUESTS" => s__WEDDING_GUESTS(v),
            "_WEDDING_LEAD_TIME" => s__WEDDING_LEAD_TIME(v),
            "_WEDDING_MONEY" => s__WEDDING_MONEY(v),
            "_WEDDING_PAPERS" => s__WEDDING_PAPERS(v),
            "_WEDDING_WHEN" => s__WEDDING_WHEN(v),
            "_WEEKEND_WORDS" => s__WEEKEND_WORDS(v),
            "_WHAT_HAPPENED" => s__WHAT_HAPPENED(v),
            "_WHEN_AWAY" => s__WHEN_AWAY(v),
            "_WHERE_GO_PHRASES" => s__WHERE_GO_PHRASES(v),
            "_WORK_BLOCK" => s__WORK_BLOCK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.plans", module_obj);
    pyrt::xrefs::reg_fn("knowledge.plans", "_not_a_count_of_their_own", py_func("_not_a_count_of_their_own", |args: &[Py]| _not_a_count_of_their_own(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_not_a_date_question", py_func("_not_a_date_question", |args: &[Py]| _not_a_date_question(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_asks_for_ideas", py_func("_asks_for_ideas", |args: &[Py]| _asks_for_ideas(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_the_free_weekend_fits_and_is_asked", py_func("_the_free_weekend_fits_and_is_asked", |args: &[Py]| _the_free_weekend_fits_and_is_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_not_a_deed_of_their_own", py_func("_not_a_deed_of_their_own", |args: &[Py]| _not_a_deed_of_their_own(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_declutter_fits", py_func("_declutter_fits", |args: &[Py]| _declutter_fits(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_try_once", py_func("_try_once", |args: &[Py]| _try_once(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_try_once_more", py_func("_try_once_more", |args: &[Py]| _try_once_more(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_wedding_plan", py_func("_wedding_plan", |args: &[Py]| _wedding_plan(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_asks_about_the_world", py_func("_asks_about_the_world", |args: &[Py]| _asks_about_the_world(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_said_words", py_func("_said_words", |args: &[Py]| _said_words(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_said_now", py_func("_said_now", |args: &[Py]| _said_now(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_household_move", py_func("_a_household_move", |args: &[Py]| _a_household_move(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_household_move_or_a_named_move", py_func("_a_household_move_or_a_named_move", |args: &[Py]| _a_household_move_or_a_named_move(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_free_weekend", py_func("_a_free_weekend", |args: &[Py]| _a_free_weekend(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_holds", py_func("_holds", |args: &[Py]| _holds(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_medicine_line", py_func("_a_medicine_line", |args: &[Py]| _a_medicine_line(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_stretch_of_time_to_fill", py_func("_a_stretch_of_time_to_fill", |args: &[Py]| _a_stretch_of_time_to_fill(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_free_weekend_fits", py_func("_free_weekend_fits", |args: &[Py]| _free_weekend_fits(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_rainy_day_is_money", py_func("_a_rainy_day_is_money", |args: &[Py]| _a_rainy_day_is_money(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_city_is_named", py_func("_a_city_is_named", |args: &[Py]| _a_city_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_ideas_about_a_thing", py_func("_ideas_about_a_thing", |args: &[Py]| _ideas_about_a_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_about_a_thing_the_person_has", py_func("_about_a_thing_the_person_has", |args: &[Py]| _about_a_thing_the_person_has(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_day_out_is_the_question", py_func("_a_day_out_is_the_question", |args: &[Py]| _a_day_out_is_the_question(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_short_run", py_func("_short_run", |args: &[Py]| _short_run(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_the_place_the_block_is_at", py_func("_the_place_the_block_is_at", |args: &[Py]| _the_place_the_block_is_at(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_names_the_free_block", py_func("_names_the_free_block", |args: &[Py]| _names_the_free_block(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_covered_words", py_func("_covered_words", |args: &[Py]| _covered_words(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_names_a_thing_of_its_own", py_func("_names_a_thing_of_its_own", |args: &[Py]| _names_a_thing_of_its_own(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_said_as_the_thing_it_does_not_want", py_func("_said_as_the_thing_it_does_not_want", |args: &[Py]| _said_as_the_thing_it_does_not_want(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_free_time", py_func("_free_time", |args: &[Py]| _free_time(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_gear_named", py_func("_gear_named", |args: &[Py]| _gear_named(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_has_word", py_func("_has_word", |args: &[Py]| _has_word(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_piece_of_baby_gear", py_func("_a_piece_of_baby_gear", |args: &[Py]| _a_piece_of_baby_gear(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_dish_for_tonight", py_func("_dish_for_tonight", |args: &[Py]| _dish_for_tonight(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_milestone_said_as_an_age", py_func("_a_milestone_said_as_an_age", |args: &[Py]| _a_milestone_said_as_an_age(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_party_plan", py_func("_party_plan", |args: &[Py]| _party_plan(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_not_a_gift_ask", py_func("_not_a_gift_ask", |args: &[Py]| _not_a_gift_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_not_a_gift_or_a_date", py_func("_not_a_gift_or_a_date", |args: &[Py]| _not_a_gift_or_a_date(args)));
    pyrt::xrefs::reg_fn("knowledge.plans", "_a_room_plan", py_func("_a_room_plan", |args: &[Py]| _a_room_plan(args)));
    pyrt::xrefs::reg_access("knowledge.plans", "ANNOUNCED_DISH", g_ANNOUNCED_DISH, s_ANNOUNCED_DISH);
    pyrt::xrefs::reg_access("knowledge.plans", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.plans", "AT_HOME", g_AT_HOME, s_AT_HOME);
    pyrt::xrefs::reg_access("knowledge.plans", "A_DAY_CONDITION", g_A_DAY_CONDITION, s_A_DAY_CONDITION);
    pyrt::xrefs::reg_access("knowledge.plans", "A_HOURS_ASK", g_A_HOURS_ASK, s_A_HOURS_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "A_WEATHER_ASK", g_A_WEATHER_ASK, s_A_WEATHER_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "DAY_OFF_PLAN", g_DAY_OFF_PLAN, s_DAY_OFF_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "DAY_PLAN", g_DAY_PLAN, s_DAY_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.plans", "EVENING_PLAN", g_EVENING_PLAN, s_EVENING_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME", g_FREE_TIME, s_FREE_TIME);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME_ASK", g_FREE_TIME_ASK, s_FREE_TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME_CUE", g_FREE_TIME_CUE, s_FREE_TIME_CUE);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME_TIME", g_FREE_TIME_TIME, s_FREE_TIME_TIME);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME_WHO", g_FREE_TIME_WHO, s_FREE_TIME_WHO);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_TIME_WORDS", g_FREE_TIME_WORDS, s_FREE_TIME_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "FREE_WEEKEND", g_FREE_WEEKEND, s_FREE_WEEKEND);
    pyrt::xrefs::reg_access("knowledge.plans", "HOLIDAY_PREFIX", g_HOLIDAY_PREFIX, s_HOLIDAY_PREFIX);
    pyrt::xrefs::reg_access("knowledge.plans", "MEDICINE_NEEDS_A_REASON", g_MEDICINE_NEEDS_A_REASON, s_MEDICINE_NEEDS_A_REASON);
    pyrt::xrefs::reg_access("knowledge.plans", "MONEY_WORDS", g_MONEY_WORDS, s_MONEY_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "NEWBORN_WEEK", g_NEWBORN_WEEK, s_NEWBORN_WEEK);
    pyrt::xrefs::reg_access("knowledge.plans", "NO_MONEY", g_NO_MONEY, s_NO_MONEY);
    pyrt::xrefs::reg_access("knowledge.plans", "PEOPLE_FEMALE", g_PEOPLE_FEMALE, s_PEOPLE_FEMALE);
    pyrt::xrefs::reg_access("knowledge.plans", "PEOPLE_MALE", g_PEOPLE_MALE, s_PEOPLE_MALE);
    pyrt::xrefs::reg_access("knowledge.plans", "PLAN_ASK", g_PLAN_ASK, s_PLAN_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "SUMMER_WEEK", g_SUMMER_WEEK, s_SUMMER_WEEK);
    pyrt::xrefs::reg_access("knowledge.plans", "TRY_ONCE", g_TRY_ONCE, s_TRY_ONCE);
    pyrt::xrefs::reg_access("knowledge.plans", "TRY_ONCE_ASK", g_TRY_ONCE_ASK, s_TRY_ONCE_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "TRY_ONCE_MORE", g_TRY_ONCE_MORE, s_TRY_ONCE_MORE);
    pyrt::xrefs::reg_access("knowledge.plans", "VAGUE_TOPICS", g_VAGUE_TOPICS, s_VAGUE_TOPICS);
    pyrt::xrefs::reg_access("knowledge.plans", "WEEKEND_PLAN", g_WEEKEND_PLAN, s_WEEKEND_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "WEEK_OFF", g_WEEK_OFF, s_WEEK_OFF);
    pyrt::xrefs::reg_access("knowledge.plans", "WEEK_PLAN", g_WEEK_PLAN, s_WEEK_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "WEEK_PLAN_SKINT", g_WEEK_PLAN_SKINT, s_WEEK_PLAN_SKINT);
    pyrt::xrefs::reg_access("knowledge.plans", "WINTER_WEEK", g_WINTER_WEEK, s_WINTER_WEEK);
    pyrt::xrefs::reg_access("knowledge.plans", "_ADULT_PARTY", g__ADULT_PARTY, s__ADULT_PARTY);
    pyrt::xrefs::reg_access("knowledge.plans", "_AGE_AT_PARTY", g__AGE_AT_PARTY, s__AGE_AT_PARTY);
    pyrt::xrefs::reg_access("knowledge.plans", "_AMBIGUOUS_CITY_WORDS", g__AMBIGUOUS_CITY_WORDS, s__AMBIGUOUS_CITY_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_AN_IDEA_ASKED_FOR", g__AN_IDEA_ASKED_FOR, s__AN_IDEA_ASKED_FOR);
    pyrt::xrefs::reg_access("knowledge.plans", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("knowledge.plans", "_ASKS_ABOUT_IT", g__ASKS_ABOUT_IT, s__ASKS_ABOUT_IT);
    pyrt::xrefs::reg_access("knowledge.plans", "_ASKS_A_CLOCK", g__ASKS_A_CLOCK, s__ASKS_A_CLOCK);
    pyrt::xrefs::reg_access("knowledge.plans", "_ASKS_THE_WORLD", g__ASKS_THE_WORLD, s__ASKS_THE_WORLD);
    pyrt::xrefs::reg_access("knowledge.plans", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_AT_HOME", g__AT_HOME, s__AT_HOME);
    pyrt::xrefs::reg_access("knowledge.plans", "_AT_THE_PLACE_WORDS", g__AT_THE_PLACE_WORDS, s__AT_THE_PLACE_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_CHILD_AT_A_MILESTONE", g__A_CHILD_AT_A_MILESTONE, s__A_CHILD_AT_A_MILESTONE);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_DATE_ASK", g__A_DATE_ASK, s__A_DATE_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_DO", g__A_DO, s__A_DO);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_MILESTONE_AGE", g__A_MILESTONE_AGE, s__A_MILESTONE_AGE);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_MILESTONE_PERSON", g__A_MILESTONE_PERSON, s__A_MILESTONE_PERSON);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_MILESTONE_SOON", g__A_MILESTONE_SOON, s__A_MILESTONE_SOON);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_MOVE_ELSEWHERE", g__A_MOVE_ELSEWHERE, s__A_MOVE_ELSEWHERE);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_MOVE_TO_PLAN", g__A_MOVE_TO_PLAN, s__A_MOVE_TO_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_PLANNED_ROOM", g__A_PLANNED_ROOM, s__A_PLANNED_ROOM);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_PRESENT_ASKED", g__A_PRESENT_ASKED, s__A_PRESENT_ASKED);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_PRESENT_FOR", g__A_PRESENT_FOR, s__A_PRESENT_FOR);
    pyrt::xrefs::reg_access("knowledge.plans", "_A_THING_THE_PERSON_HAS", g__A_THING_THE_PERSON_HAS, s__A_THING_THE_PERSON_HAS);
    pyrt::xrefs::reg_access("knowledge.plans", "_BABY_GEAR", g__BABY_GEAR, s__BABY_GEAR);
    pyrt::xrefs::reg_access("knowledge.plans", "_BABY_GEAR_FAULT", g__BABY_GEAR_FAULT, s__BABY_GEAR_FAULT);
    pyrt::xrefs::reg_access("knowledge.plans", "_BABY_GEAR_MEND", g__BABY_GEAR_MEND, s__BABY_GEAR_MEND);
    pyrt::xrefs::reg_access("knowledge.plans", "_BARE_RID", g__BARE_RID, s__BARE_RID);
    pyrt::xrefs::reg_access("knowledge.plans", "_BATHROOM_PLAN", g__BATHROOM_PLAN, s__BATHROOM_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "_BLOCK_QUALIFIERS", g__BLOCK_QUALIFIERS, s__BLOCK_QUALIFIERS);
    pyrt::xrefs::reg_access("knowledge.plans", "_BRING_IT", g__BRING_IT, s__BRING_IT);
    pyrt::xrefs::reg_access("knowledge.plans", "_CHILD_AT_PARTY", g__CHILD_AT_PARTY, s__CHILD_AT_PARTY);
    pyrt::xrefs::reg_access("knowledge.plans", "_COLD", g__COLD, s__COLD);
    pyrt::xrefs::reg_access("knowledge.plans", "_COLD_WEATHER", g__COLD_WEATHER, s__COLD_WEATHER);
    pyrt::xrefs::reg_access("knowledge.plans", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_DECLUTTER_ASK", g__DECLUTTER_ASK, s__DECLUTTER_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_DECLUTTER_ROOM", g__DECLUTTER_ROOM, s__DECLUTTER_ROOM);
    pyrt::xrefs::reg_access("knowledge.plans", "_DECLUTTER_STUFF", g__DECLUTTER_STUFF, s__DECLUTTER_STUFF);
    pyrt::xrefs::reg_access("knowledge.plans", "_EVENT_AWAY", g__EVENT_AWAY, s__EVENT_AWAY);
    pyrt::xrefs::reg_access("knowledge.plans", "_EVENT_EVENING", g__EVENT_EVENING, s__EVENT_EVENING);
    pyrt::xrefs::reg_access("knowledge.plans", "_FOR_A_RAINY_DAY", g__FOR_A_RAINY_DAY, s__FOR_A_RAINY_DAY);
    pyrt::xrefs::reg_access("knowledge.plans", "_FOR_THE_PLACE_WORDS", g__FOR_THE_PLACE_WORDS, s__FOR_THE_PLACE_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_FREE_BLOCK_WORDS", g__FREE_BLOCK_WORDS, s__FREE_BLOCK_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_GIFT_ASK", g__GIFT_ASK, s__GIFT_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_HOME_IN_THE_LINE", g__HOME_IN_THE_LINE, s__HOME_IN_THE_LINE);
    pyrt::xrefs::reg_access("knowledge.plans", "_IDEAS_ASK", g__IDEAS_ASK, s__IDEAS_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_IDEAS_FOR", g__IDEAS_FOR, s__IDEAS_FOR);
    pyrt::xrefs::reg_access("knowledge.plans", "_IDEA_FILLER", g__IDEA_FILLER, s__IDEA_FILLER);
    pyrt::xrefs::reg_access("knowledge.plans", "_IDEA_NONE", g__IDEA_NONE, s__IDEA_NONE);
    pyrt::xrefs::reg_access("knowledge.plans", "_JOB_NAMED", g__JOB_NAMED, s__JOB_NAMED);
    pyrt::xrefs::reg_access("knowledge.plans", "_JOB_WORDS", g__JOB_WORDS, s__JOB_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_KIDS_PARTY", g__KIDS_PARTY, s__KIDS_PARTY);
    pyrt::xrefs::reg_access("knowledge.plans", "_KITCHEN_PLAN", g__KITCHEN_PLAN, s__KITCHEN_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "_MEDIUM_NAMED", g__MEDIUM_NAMED, s__MEDIUM_NAMED);
    pyrt::xrefs::reg_access("knowledge.plans", "_MILESTONE_AGES", g__MILESTONE_AGES, s__MILESTONE_AGES);
    pyrt::xrefs::reg_access("knowledge.plans", "_MILESTONE_SAID", g__MILESTONE_SAID, s__MILESTONE_SAID);
    pyrt::xrefs::reg_access("knowledge.plans", "_MONEY_ONLY", g__MONEY_ONLY, s__MONEY_ONLY);
    pyrt::xrefs::reg_access("knowledge.plans", "_MONEY_WITH_IT", g__MONEY_WITH_IT, s__MONEY_WITH_IT);
    pyrt::xrefs::reg_access("knowledge.plans", "_MONTHS", g__MONTHS, s__MONTHS);
    pyrt::xrefs::reg_access("knowledge.plans", "_MUSING", g__MUSING, s__MUSING);
    pyrt::xrefs::reg_access("knowledge.plans", "_NOTE_ASK", g__NOTE_ASK, s__NOTE_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_NOT_THE_THING", g__NOT_THE_THING, s__NOT_THE_THING);
    pyrt::xrefs::reg_access("knowledge.plans", "_NO_SUBJECT", g__NO_SUBJECT, s__NO_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.plans", "_OWN_ACTIVITY", g__OWN_ACTIVITY, s__OWN_ACTIVITY);
    pyrt::xrefs::reg_access("knowledge.plans", "_OWN_COUNT", g__OWN_COUNT, s__OWN_COUNT);
    pyrt::xrefs::reg_access("knowledge.plans", "_PEOPLE", g__PEOPLE, s__PEOPLE);
    pyrt::xrefs::reg_access("knowledge.plans", "_PURPOSE_AFTER_RUN", g__PURPOSE_AFTER_RUN, s__PURPOSE_AFTER_RUN);
    pyrt::xrefs::reg_access("knowledge.plans", "_REQUESTING_TIME", g__REQUESTING_TIME, s__REQUESTING_TIME);
    pyrt::xrefs::reg_access("knowledge.plans", "_SHORT_RUN", g__SHORT_RUN, s__SHORT_RUN);
    pyrt::xrefs::reg_access("knowledge.plans", "_SWALLOWED", g__SWALLOWED, s__SWALLOWED);
    pyrt::xrefs::reg_access("knowledge.plans", "_SWALLOWED_THING", g__SWALLOWED_THING, s__SWALLOWED_THING);
    pyrt::xrefs::reg_access("knowledge.plans", "_THE_EVENING", g__THE_EVENING, s__THE_EVENING);
    pyrt::xrefs::reg_access("knowledge.plans", "_THE_OWNER", g__THE_OWNER, s__THE_OWNER);
    pyrt::xrefs::reg_access("knowledge.plans", "_THE_WORLD", g__THE_WORLD, s__THE_WORLD);
    pyrt::xrefs::reg_access("knowledge.plans", "_TIME_WORDS", g__TIME_WORDS, s__TIME_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_TODAY_WORDS", g__TODAY_WORDS, s__TODAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_TOO_MUCH", g__TOO_MUCH, s__TOO_MUCH);
    pyrt::xrefs::reg_access("knowledge.plans", "_TRIP_PHRASES", g__TRIP_PHRASES, s__TRIP_PHRASES);
    pyrt::xrefs::reg_access("knowledge.plans", "_UNWELL", g__UNWELL, s__UNWELL);
    pyrt::xrefs::reg_access("knowledge.plans", "_UTILITY_PLAN", g__UTILITY_PLAN, s__UTILITY_PLAN);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_ASK", g__WEDDING_ASK, s__WEDDING_ASK);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_COUNT", g__WEDDING_COUNT, s__WEDDING_COUNT);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_GUESTS", g__WEDDING_GUESTS, s__WEDDING_GUESTS);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_LEAD_TIME", g__WEDDING_LEAD_TIME, s__WEDDING_LEAD_TIME);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_MONEY", g__WEDDING_MONEY, s__WEDDING_MONEY);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_PAPERS", g__WEDDING_PAPERS, s__WEDDING_PAPERS);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEDDING_WHEN", g__WEDDING_WHEN, s__WEDDING_WHEN);
    pyrt::xrefs::reg_access("knowledge.plans", "_WEEKEND_WORDS", g__WEEKEND_WORDS, s__WEEKEND_WORDS);
    pyrt::xrefs::reg_access("knowledge.plans", "_WHAT_HAPPENED", g__WHAT_HAPPENED, s__WHAT_HAPPENED);
    pyrt::xrefs::reg_access("knowledge.plans", "_WHEN_AWAY", g__WHEN_AWAY, s__WHEN_AWAY);
    pyrt::xrefs::reg_access("knowledge.plans", "_WHERE_GO_PHRASES", g__WHERE_GO_PHRASES, s__WHERE_GO_PHRASES);
    pyrt::xrefs::reg_access("knowledge.plans", "_WORK_BLOCK", g__WORK_BLOCK, s__WORK_BLOCK);
}

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


thread_local! { static G_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGE_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAFE_: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAFE_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCHOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCREEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_CHILDREN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLASS_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLASS_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAMILY_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAMILY_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOME_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOME_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JUDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PLACE_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAISING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE_OBJECT_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE_OBJECT_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAFE_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCREEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCREEN_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCREEN_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGHT_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGHT_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAY_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAY_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRAVEL_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIP_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIP_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WALK_CRITERIA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WALK_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WITH_CHILDREN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.a_place_nobody_can_see: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CLASS() -> Py { ensure_init(); G_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS(v: Py) { ensure_init(); G_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD() -> Py { ensure_init(); G_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD(v: Py) { ensure_init(); G_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGE_OPENERS() -> Py { ensure_init(); G_HEDGE_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGE_OPENERS(v: Py) { ensure_init(); G_HEDGE_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME() -> Py { ensure_init(); G_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME(v: Py) { ensure_init(); G_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB() -> Py { ensure_init(); G_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB(v: Py) { ensure_init(); G_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JUDGEMENT() -> Py { ensure_init(); G_NOT_A_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JUDGEMENT(v: Py) { ensure_init(); G_NOT_A_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER() -> Py { ensure_init(); G_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER(v: Py) { ensure_init(); G_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_SAFE_() -> Py { ensure_init(); G_SAFE_.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAFE_(v: Py) { ensure_init(); G_SAFE_.with(|c| *c.borrow_mut() = v); }
pub fn g_SAFE_OBJECT() -> Py { ensure_init(); G_SAFE_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAFE_OBJECT(v: Py) { ensure_init(); G_SAFE_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_SCHOOL() -> Py { ensure_init(); G_SCHOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCHOOL(v: Py) { ensure_init(); G_SCHOOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SCREEN() -> Py { ensure_init(); G_SCREEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCREEN(v: Py) { ensure_init(); G_SCREEN.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGHT() -> Py { ensure_init(); G_SIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGHT(v: Py) { ensure_init(); G_SIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY() -> Py { ensure_init(); G_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY(v: Py) { ensure_init(); G_STAY.with(|c| *c.borrow_mut() = v); }
pub fn g_TRIP() -> Py { ensure_init(); G_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRIP(v: Py) { ensure_init(); G_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_ASK() -> Py { ensure_init(); G_VERDICT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_ASK(v: Py) { ensure_init(); G_VERDICT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WALK() -> Py { ensure_init(); G_WALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WALK(v: Py) { ensure_init(); G_WALK.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_CHILDREN() -> Py { ensure_init(); G_WITH_CHILDREN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_CHILDREN(v: Py) { ensure_init(); G_WITH_CHILDREN.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_BOT() -> Py { ensure_init(); G__ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_BOT(v: Py) { ensure_init(); G__ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_NOUN() -> Py { ensure_init(); G__BARE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_NOUN(v: Py) { ensure_init(); G__BARE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_CRITERIA() -> Py { ensure_init(); G__CHILD_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_CRITERIA(v: Py) { ensure_init(); G__CHILD_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_WHERE() -> Py { ensure_init(); G__CHILD_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_WHERE(v: Py) { ensure_init(); G__CHILD_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__CLASS() -> Py { ensure_init(); G__CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLASS(v: Py) { ensure_init(); G__CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g__CLASS_CRITERIA() -> Py { ensure_init(); G__CLASS_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLASS_CRITERIA(v: Py) { ensure_init(); G__CLASS_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__CLASS_WHERE() -> Py { ensure_init(); G__CLASS_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLASS_WHERE(v: Py) { ensure_init(); G__CLASS_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__FAMILY_CRITERIA() -> Py { ensure_init(); G__FAMILY_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAMILY_CRITERIA(v: Py) { ensure_init(); G__FAMILY_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__FAMILY_WHERE() -> Py { ensure_init(); G__FAMILY_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAMILY_WHERE(v: Py) { ensure_init(); G__FAMILY_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELING() -> Py { ensure_init(); G__FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELING(v: Py) { ensure_init(); G__FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD() -> Py { ensure_init(); G__FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD(v: Py) { ensure_init(); G__FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD_CRITERIA() -> Py { ensure_init(); G__FOOD_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD_CRITERIA(v: Py) { ensure_init(); G__FOOD_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD_WHERE() -> Py { ensure_init(); G__FOOD_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD_WHERE(v: Py) { ensure_init(); G__FOOD_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__HOME() -> Py { ensure_init(); G__HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOME(v: Py) { ensure_init(); G__HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__HOME_CRITERIA() -> Py { ensure_init(); G__HOME_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOME_CRITERIA(v: Py) { ensure_init(); G__HOME_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__HOME_WHERE() -> Py { ensure_init(); G__HOME_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOME_WHERE(v: Py) { ensure_init(); G__HOME_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB() -> Py { ensure_init(); G__JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB(v: Py) { ensure_init(); G__JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB_CRITERIA() -> Py { ensure_init(); G__JOB_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB_CRITERIA(v: Py) { ensure_init(); G__JOB_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB_WHERE() -> Py { ensure_init(); G__JOB_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB_WHERE(v: Py) { ensure_init(); G__JOB_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__JUDGE() -> Py { ensure_init(); G__JUDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JUDGE(v: Py) { ensure_init(); G__JUDGE.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMED() -> Py { ensure_init(); G__NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMED(v: Py) { ensure_init(); G__NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PLACE_TOPICS() -> Py { ensure_init(); G__NOT_A_PLACE_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PLACE_TOPICS(v: Py) { ensure_init(); G__NOT_A_PLACE_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING() -> Py { ensure_init(); G__NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); G__NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_ASK() -> Py { ensure_init(); G__PAST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_ASK(v: Py) { ensure_init(); G__PAST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE() -> Py { ensure_init(); G__PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE(v: Py) { ensure_init(); G__PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__RAISING() -> Py { ensure_init(); G__RAISING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAISING(v: Py) { ensure_init(); G__RAISING.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE() -> Py { ensure_init(); G__SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE(v: Py) { ensure_init(); G__SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE_CRITERIA() -> Py { ensure_init(); G__SAFE_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE_CRITERIA(v: Py) { ensure_init(); G__SAFE_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE_OBJECT() -> Py { ensure_init(); G__SAFE_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE_OBJECT(v: Py) { ensure_init(); G__SAFE_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE_OBJECT_CRITERIA() -> Py { ensure_init(); G__SAFE_OBJECT_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE_OBJECT_CRITERIA(v: Py) { ensure_init(); G__SAFE_OBJECT_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE_OBJECT_WHERE() -> Py { ensure_init(); G__SAFE_OBJECT_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE_OBJECT_WHERE(v: Py) { ensure_init(); G__SAFE_OBJECT_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAFE_WHERE() -> Py { ensure_init(); G__SAFE_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAFE_WHERE(v: Py) { ensure_init(); G__SAFE_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL() -> Py { ensure_init(); G__SCHOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL(v: Py) { ensure_init(); G__SCHOOL.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_CRITERIA() -> Py { ensure_init(); G__SCHOOL_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_CRITERIA(v: Py) { ensure_init(); G__SCHOOL_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_WHERE() -> Py { ensure_init(); G__SCHOOL_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_WHERE(v: Py) { ensure_init(); G__SCHOOL_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__SCREEN() -> Py { ensure_init(); G__SCREEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCREEN(v: Py) { ensure_init(); G__SCREEN.with(|c| *c.borrow_mut() = v); }
pub fn g__SCREEN_CRITERIA() -> Py { ensure_init(); G__SCREEN_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCREEN_CRITERIA(v: Py) { ensure_init(); G__SCREEN_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__SCREEN_WHERE() -> Py { ensure_init(); G__SCREEN_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCREEN_WHERE(v: Py) { ensure_init(); G__SCREEN_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGHT() -> Py { ensure_init(); G__SIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGHT(v: Py) { ensure_init(); G__SIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGHT_CRITERIA() -> Py { ensure_init(); G__SIGHT_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGHT_CRITERIA(v: Py) { ensure_init(); G__SIGHT_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGHT_WHERE() -> Py { ensure_init(); G__SIGHT_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGHT_WHERE(v: Py) { ensure_init(); G__SIGHT_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__STAY() -> Py { ensure_init(); G__STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAY(v: Py) { ensure_init(); G__STAY.with(|c| *c.borrow_mut() = v); }
pub fn g__STAY_CRITERIA() -> Py { ensure_init(); G__STAY_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAY_CRITERIA(v: Py) { ensure_init(); G__STAY_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__STAY_WHERE() -> Py { ensure_init(); G__STAY_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAY_WHERE(v: Py) { ensure_init(); G__STAY_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__STOP() -> Py { ensure_init(); G__STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STOP(v: Py) { ensure_init(); G__STOP.with(|c| *c.borrow_mut() = v); }
pub fn g__THING() -> Py { ensure_init(); G__THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THING(v: Py) { ensure_init(); G__THING.with(|c| *c.borrow_mut() = v); }
pub fn g__TRAVEL_TOPICS() -> Py { ensure_init(); G__TRAVEL_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRAVEL_TOPICS(v: Py) { ensure_init(); G__TRAVEL_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIP() -> Py { ensure_init(); G__TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIP(v: Py) { ensure_init(); G__TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIP_CRITERIA() -> Py { ensure_init(); G__TRIP_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIP_CRITERIA(v: Py) { ensure_init(); G__TRIP_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIP_WHERE() -> Py { ensure_init(); G__TRIP_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIP_WHERE(v: Py) { ensure_init(); G__TRIP_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__WALK() -> Py { ensure_init(); G__WALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WALK(v: Py) { ensure_init(); G__WALK.with(|c| *c.borrow_mut() = v); }
pub fn g__WALK_CRITERIA() -> Py { ensure_init(); G__WALK_CRITERIA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WALK_CRITERIA(v: Py) { ensure_init(); G__WALK_CRITERIA.with(|c| *c.borrow_mut() = v); }
pub fn g__WALK_WHERE() -> Py { ensure_init(); G__WALK_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WALK_WHERE(v: Py) { ensure_init(); G__WALK_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__WITH_CHILDREN() -> Py { ensure_init(); G__WITH_CHILDREN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WITH_CHILDREN(v: Py) { ensure_init(); G__WITH_CHILDREN.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A verdict asked for on a place or a thing the bot has never seen: the criteria, not a guess.\n\nPeople ask the bot to judge things it cannot see more often than they ask it for a fact about\nthem: \"is benidorm ok with a toddler\", \"is the old town doable\", \"is that restaurant good\", \"is\nthis area safe\". No knowledge base holds a judgement about a place, so every one of those lines\nreached the search hedge:\n\n    is benidorm ok with a toddler\n    That's beyond what I know well, sorry. Searching for it will turn up better sources than I\n    can offer; tell me what you find and I'll help you weigh it up.\n\nNothing there is wrong and nothing in it helps. STYLE.md's \"Not knowing, honestly\" has the shape\nthat does: say what the bot cannot see in a few words, give the part it does know, and stay with\nthe person for the rest. The part it knows here is not a fact about Benidorm - it is what actually\ndecides a trip with a toddler, which is true of every trip with a toddler. So the reply names the\nthing back, says plainly that there is no first-hand knowledge of it, gives the two or three things\nthat settle it, and names where the live detail is (recent reviews, a map app, the council's crime\nmap). Nobody is told what to think about a place the bot has never been to.\n\nThis is a last resort: `fallback.unknown_pool` asks it only on the branch where nothing else has\nanswered, and only for a line that is a judgement ask. Any entry, any module, and the composer's\nown decision or live answer all come first, so the file can only replace a search hedge.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.HEDGE_OPENERS"); }
    s_HEDGE_OPENERS(py_tuple(vec![py_str("i'm not sure about that one"), py_str("that's beyond what i know well"), py_str("i don't know enough to answer that honestly")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._JUDGE"); }
    s__JUDGE(py_str("(?:ok|okay|alright|all right|good|nice|safe|doable|suitable|any good|any cop|any use|decent|handy|worth it|worth going|worth seeing)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.VERDICT_ASK"); }
    s_VERDICT_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^\\s*(?:so|but|and|ok|okay)?\\s*(?:do you think\\s+)?(?:(?:is|are|was|were)\\s+[a-z' ]{2,40}?\\s+"), &g__JUDGE()), &py_str("|(?:would|will)\\s+[a-z' ]{2,40}?\\s+(?:suit|work for|be ok for|be alright for|be all right for|be good for)\\b|(?:is|was|would)\\s+[a-z' ]{0,40}?\\s*worth\\s+(?:it|a (?:look|visit|go|trip|day out)|going|the (?:trip|money|hype))\\b)|\\b(?:is|would)\\s+it\\s+")), &g__JUDGE()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.NOT_A_JUDGEMENT"); }
    s_NOT_A_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:eat|eaten|ate|eating|cook|cooked|cooking|cooker|freeze|frozen|defrost|fridge|left out|left over|leftover|reheat|next day|day after|still ok|still good|gone off|use by|sell by|best before|chicken|meat|fish|rice|milk|eggs|yoghurt|yogurt|dose|dosage|mg|ml|tablet|tablets|pill|pills|medicine|paracetamol|ibuprofen|antibiotic|pregnan\\w*|symptom\\w*|rash|fever|temperature|covid|flu|old enough|how old|legal|illegal|allowed|the law|licen\\w+|rules|cost|costs|price|prices|how much|cheaper|afford|salary|wage|wages|tax|refund|deposit|visa|passport|insur\\w*|contract|tenancy|notice period|worth buying|worth fixing|worth repairing|worth selling|repair|fix|broken)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._THING"); }
    s__THING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:so|but|and|ok|okay)?\\s*(?:do you think\\s+)?(?:is|are|was|were|would|will)\\s+(?P<thing>[a-z' ]{2,40}?)\\s+(?:"), &g__JUDGE()), &py_str("|suit|work for|be ok for|be alright for|be all right for|be good for)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._BARE_NOUN"); }
    s__BARE_NOUN(call_builtin("frozenset", &[mcall(&py_str("area place street road estate flat house home hotel restaurant cafe\n    pub school nursery gym club class course town city village beach park market walk route\n    holiday trip"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._NOT_A_THING"); }
    s__NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("food menu price cost drink beer wine coffee"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("suit"), py_str("work"), py_str("be"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("good"), py_str("nice"), py_str("safe"), py_str("doable"), py_str("worth"), py_str("any"), py_str("decent"), py_str("handy"), py_str("all")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._STOP"); }
    s__STOP(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._TRIP_CRITERIA"); }
    s__TRIP_CRITERIA(py_str("how far the flight or the drive is, how hot it gets, and how much walking there is once you are there"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._TRIP_WHERE"); }
    s__TRIP_WHERE(py_str("Reviews from the last few months say what a place is like now rather than five years ago."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._CHILD_CRITERIA"); }
    s__CHILD_CRITERIA(py_str("the flight or the drive, the heat, and how far the beach, the food and the shops are from the room"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._CHILD_WHERE"); }
    s__CHILD_WHERE(py_str("The hotel's own page and the last few months of reviews cover the rest, and a map app shows the walk to the sand."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE_CRITERIA"); }
    s__SAFE_CRITERIA(py_str("the time of day you would be out, how busy the street is then, and whether you would be walking it alone"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE_WHERE"); }
    s__SAFE_WHERE(py_str("The police's own crime map for the postcode says more than any general view of a place does."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE_OBJECT_CRITERIA"); }
    s__SAFE_OBJECT_CRITERIA(py_str("the moving parts, the fixings and the seals, and anything cracked, loose, or smelling of gas or hot plastic"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE_OBJECT_WHERE"); }
    s__SAFE_OBJECT_WHERE(py_str("Whoever services them will look it over for very little, and a gas or electrical job wants a registered engineer rather than a guess."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCHOOL_CRITERIA"); }
    s__SCHOOL_CRITERIA(py_str("the walk there, what the mornings feel like, and how easy it is to get a straight answer out of the staff"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCHOOL_WHERE"); }
    s__SCHOOL_WHERE(py_str("Their own site has the numbers and the last report, and other parents tell you the rest."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FOOD_CRITERIA"); }
    s__FOOD_CRITERIA(py_str("the last few months of reviews, the menu, and whether the price suits the evening you have in mind"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FOOD_WHERE"); }
    s__FOOD_WHERE(py_str("Their own page and the recent reviews have all three, and one visit settles it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._STAY_CRITERIA"); }
    s__STAY_CRITERIA(py_str("the last few months of reviews, how far it is from where you want to be, and what the noise is like at night"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._STAY_WHERE"); }
    s__STAY_WHERE(py_str("Their own page and recent reviews cover all three, and the map listing shows the walk into town."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._CLASS_CRITERIA"); }
    s__CLASS_CRITERIA(py_str("the times you can actually get to, who takes the session, and whether the first one is free"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._CLASS_WHERE"); }
    s__CLASS_WHERE(py_str("Their timetable and a trial session answer that better than any review."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._WALK_CRITERIA"); }
    s__WALK_CRITERIA(py_str("the distance, how much of it is up or down, and the ground underfoot"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._WALK_WHERE"); }
    s__WALK_WHERE(py_str("A map app shows the distance and the climb in a minute, and street view shows you the path."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._HOME_CRITERIA"); }
    s__HOME_CRITERIA(py_str("the walk from the station or the bus, the light in the rooms, the noise at the time you would be home, and what the neighbours are like"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._HOME_WHERE"); }
    s__HOME_WHERE(py_str("A map app shows the walk and the streets, and one visit on a weekday evening tells you the noise."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.TRIP"); }
    s_TRIP(py_tuple(vec![py_str("{lead}never been, so no verdict from me. What decides a trip there is {criteria}. {where}"), py_str("{lead}no first-hand knowledge of the place, so no verdict from me. The things that settle it are {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.WITH_CHILDREN"); }
    s_WITH_CHILDREN(py_tuple(vec![py_str("{lead}not been with a little one, so I can't call it. What decides that trip is {criteria}. {where}"), py_str("{lead}can't call it from here. With a child in tow it comes down to {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.SAFE_"); }
    s_SAFE_(py_tuple(vec![py_str("{lead}can't tell you whether a place is safe from here, and I won't guess at it. What changes the answer is {criteria}. {where}"), py_str("{lead}no way for me to know about crime on a street I can't see, so no verdict from me. It turns on {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.SAFE_OBJECT"); }
    s_SAFE_OBJECT(py_tuple(vec![py_str("{lead}I can't look at it from here, so no verdict from me on whether it is safe. What a check comes down to is {criteria}. {where}"), py_str("{lead}no way for me to see the state of it, and safety is not something I'll guess at. It comes down to {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.SCHOOL"); }
    s_SCHOOL(py_tuple(vec![py_str("{lead}can't judge a school from here, and guessing would be worse than saying so. What tells you most is {criteria}. {where}"), py_str("{lead}no verdict from me on that one - I can't see inside a school from here. What tells you most is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.FOOD"); }
    s_FOOD(py_tuple(vec![py_str("{lead}not eaten there, so no verdict from me. What decides it is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.STAY"); }
    s_STAY(py_tuple(vec![py_str("{lead}not stayed there, so no verdict from me. What decides it is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.CLASS"); }
    s_CLASS(py_tuple(vec![py_str("{lead}can't judge one from here. It comes down to {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.WALK"); }
    s_WALK(py_tuple(vec![py_str("{lead}can't see the ground from here, so I won't guess at how hard it is. What decides it is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.SIGHT"); }
    s_SIGHT(py_tuple(vec![py_str("{lead}no first-hand knowledge of the place, so no verdict from me. The things that settle it are {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.FAMILY"); }
    s_FAMILY(py_tuple(vec![py_str("{lead}can't judge a place to bring children up from here. What decides it is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.HOME"); }
    s_HOME(py_tuple(vec![py_str("{lead}no way of seeing a place I haven't been to, so no verdict from me. What decides it is {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.OTHER"); }
    s_OTHER(py_tuple(vec![py_str("No first-hand knowledge of that one, so no verdict from me - and guessing would be worse than saying so. What settles most of these is what you want out of it, what you would do with it if it turned out to be no good, and what people who wanted the same thing say about it once they have had it a few months. Reviews from people who wanted the same thing say more than the page selling it."), py_str("No first-hand knowledge of that one here, so no verdict from me. What it comes down to is how it works for the thing you actually want it for, how it holds up after a few months, and what it costs you if it is wrong. Recent reviews from people who wanted the same thing are the shortest way to all three.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCREEN_CRITERIA"); }
    s__SCREEN_CRITERIA(py_str("who made it and what you thought of theirs before, how long it takes to get going, and whether you want to think or switch off"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCREEN_WHERE"); }
    s__SCREEN_WHERE(py_str("A trailer or the first chapter tells you more in ten minutes than any review, and the recent reviews say whether it holds up after the opening."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._JOB_CRITERIA"); }
    s__JOB_CRITERIA(py_str("the pay, the hours, and what the day actually looks like rather than what the job is called"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._JOB_WHERE"); }
    s__JOB_WHERE(py_str("The pay scales are published, and asking two people who do it now about a normal week settles more than any article."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.SCREEN"); }
    s_SCREEN(py_tuple(vec![py_str("{lead}no first-hand knowledge of that one, so no verdict from me. What decides it is {criteria}. {where}"), py_str("{lead}can't judge it from here. It comes down to {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see.JOB"); }
    s_JOB(py_tuple(vec![py_str("{lead}wouldn't want to call that from here. What decides it is {criteria}. {where}"), py_str("{lead}no first-hand knowledge of that one. The honest answer turns on {criteria}. {where}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._ABOUT_THE_BOT"); }
    s__ABOUT_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:are|is|was|were)\\s+(?:you|u|ya|the bot|this bot|this thing|the chatbot)\\b|\\b(?:ok|okay|alright|all right|fine|good|bad)\\s+(?:with|to|for)\\s+(?:you|u|ya)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._PLACE"); }
    s__PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:area|areas|street|streets|road|roads|neighbourhood|neighborhood|estate|town|city|village|centre|center|park|beach|seafront|suburb|postcode|place|location|block|here|there|abroad|hotel|hostel|flat|house|home|station|walkway|alley|underpass|path|route|car park|sea|lake|river|coast|seaside|harbour|harbor)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE"); }
    s__SAFE(stdlib_call("re", "compile", &[py_str("\\b(?:safe|safety|rough|dodgy|crime|mugged|sketchy|dangers?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FEELING"); }
    s__FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:feel|feels|feeling|felt)\\b|\\blike this\\b|\\babout myself\\b|\\bfor me to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCREEN"); }
    s__SCREEN(stdlib_call("re", "compile", &[py_str("\\b(?:films?|movies?|series|show|shows|book|books|novel|album|game|games|documentar(?:y|ies)|podcasts?|sitcom|box set)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._JOB"); }
    s__JOB(stdlib_call("re", "compile", &[py_str("\\b(?:jobs?|careers?|trades?|profession|line of work|working as|worth (?:doing|studying|training))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SAFE_OBJECT"); }
    s__SAFE_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:bikes?|cars?|vans?|mopeds?|scooters?|motorcycles?|ladders?|boilers?|heaters?|wood ?burners?|wiring|electrics|roof|chainsaws?|trampolines?|swing sets?|woodburner)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SCHOOL"); }
    s__SCHOOL(stdlib_call("re", "compile", &[py_str("\\b(?:school|schools|nursery|nurseries|preschool|pre-school|childminder|creche|cr[e\\u00e8]che|kindergarten|college|university|uni)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._STAY"); }
    s__STAY(stdlib_call("re", "compile", &[py_str("\\b(?:hotel|hotels|b&b|b and b|hostel|guest house|guesthouse|apartment|villa|cottage|campsite)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FOOD"); }
    s__FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:restaurant|restaurants|caf[e\\u00e9]|pub|takeaway|bar|canteen|chippy|kebab shop|menu|food)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._CLASS"); }
    s__CLASS(stdlib_call("re", "compile", &[py_str("\\b(?:gym|gyms|class|classes|club|clubs|course|courses|lessons|swimming pool|pool|leisure centre)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._WITH_CHILDREN"); }
    s__WITH_CHILDREN(stdlib_call("re", "compile", &[py_str("\\b(?:toddlers?|bab(?:y|ies)|kids?|children|child|pram|buggy|pushchair|teenagers?|families)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._WALK"); }
    s__WALK(stdlib_call("re", "compile", &[py_str("\\b(?:walk|walks|route|hike|climb|stairs|hill)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._RAISING"); }
    s__RAISING(stdlib_call("re", "compile", &[py_str("\\b(?:bring (?:up|ing up)|raise|raising|grow(?:ing)? up|settle down|to live|live in|family life)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SIGHT"); }
    s__SIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:old town|town cent(?:re|er)|city cent(?:re|er)|market|attractions?|museum|gallery|castle|cathedral|zoo|theme park|water park)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._TRIP"); }
    s__TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:holiday|holidays|trip|resort|resorts|beach|town|city|village|island|abroad|seaside|centre|center|market|attraction|museum|zoo|park|cruise|cruises|ferry|flight|flights|long haul|day out|days out|weekend away)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._NAMED"); }
    s__NAMED(stdlib_call("re", "compile", &[py_str("\\S\\s+(?:the\\s+|a\\s+|an\\s+)?[A-Z][a-z]{2,}")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._TRAVEL_TOPICS"); }
    s__TRAVEL_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("geography-and-travel"), py_str("travel"), py_str("holidays"), py_str("places")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("sport"), py_str("fitness"), py_str("health"), py_str("body"), py_str("eating"), py_str("food-and-drink"), py_str("money"), py_str("work"), py_str("technology"), py_str("pets"), py_str("law"), py_str("driving"), py_str("school-and-study"), py_str("hobbies"), py_str("home-and-garden"), py_str("beauty"), py_str("clothes")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._NOT_A_PLACE_TOPICS"); }
    s__NOT_A_PLACE_TOPICS(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SIGHT_CRITERIA"); }
    s__SIGHT_CRITERIA(py_str("how much walking there is, whether it goes up or down, and how busy it gets at the time you would go"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._SIGHT_WHERE"); }
    s__SIGHT_WHERE(py_str("The place's own page has the access and the opening times, and a map app shows the walk and the climb"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FAMILY_CRITERIA"); }
    s__FAMILY_CRITERIA(py_str("the schools and how you get into them, how much green space is within walking distance, and what there is for children to do"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._FAMILY_WHERE"); }
    s__FAMILY_WHERE(py_str("The council's own site lists the schools and the areas they take, and an afternoon walk round on a school day tells you the rest"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._HOME"); }
    s__HOME(stdlib_call("re", "compile", &[py_str("\\b(?:flat|house|home|area|neighbourhood|neighborhood|street|road|estate|place|location|block)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.a_place_nobody_can_see._PAST_ASK"); }
    s__PAST_ASK(stdlib_call("re", "compile", &[py_str("^(?:and\\s+)?(?:was|were|did|how was|how were|how did)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let mut owning: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line names, capitalised and ready to say back, or \"\" when it names none.");
    found = mcall(&g__THING(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    words = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__STOP(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("there"), py_str("they"), py_str("these"), py_str("those")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    owning = Py::Bool(false);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    owning = Py::Bool(true);
    }
    if py_truthy(&{ let __b = owning.clone(); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    return op_add(&mcall(&slice_or_raise(&op_add(&py_str("that "), &mcall(&py_str(" "), "join", &[words.clone()])), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&op_add(&py_str("that "), &mcall(&py_str(" "), "join", &[words.clone()])), &Py::Int(1), &Py::None, &Py::None));
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("one"), py_str("career"), py_str("idea"), py_str("option"), py_str("choice")]), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("one")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_A_THING(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    return py_str("");
    }
    thing = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__BARE_NOUN(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    thing = op_add(&py_str("that "), &geti(&words.clone(), &Py::Int(0)));
    }
    return op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _pool(args: &[Py]) -> Py {
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
    let mut named: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { Py::Bool(false) };
    let mut other_subject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other_subject") { v.clone() }
    else { Py::Bool(false) };
    let mut travel: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "travel") { v.clone() }
    else { Py::Bool(false) };
    let mut somewhere: Py = Py::None;
    let _ = py_str("The pool for the kind of thing being judged, with the criteria and place that go with it.\n\n    `(None, None, None)` when the line names a kind nothing here can speak to, so the hedge it had\n    is kept rather than replaced with criteria about the wrong thing.\n\n    Order is the reading: the most specific kind wins, and a class in the line (\"for kids\", \"with a\n    toddler\", \"on a budget\") beats the verb it is asked with, so \"is benidorm safe for kids\" gets\n    the child's criteria and not the crime map (@spare-leopard, #10238). `named` says the line has\n    a capitalised name in it, which is how a place is named when no place word is used, and\n    `other_subject` says the understanding engine read the line as being about something that is\n    not a place at all, which no number of children turns into a trip.\n    ");
    if py_truthy(&{ let __b = mcall(&g__SAFE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__SAFE_OBJECT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__HOME(), "search", &[said.clone()]))) } } } }) {
    return py_tuple(vec![g_SAFE_OBJECT(), g__SAFE_OBJECT_CRITERIA(), g__SAFE_OBJECT_WHERE()]);
    }
    if py_truthy(&mcall(&g__SCHOOL(), "search", &[said.clone()])) {
    return py_tuple(vec![g_SCHOOL(), g__SCHOOL_CRITERIA(), g__SCHOOL_WHERE()]);
    }
    if py_truthy(&mcall(&g__STAY(), "search", &[said.clone()])) {
    return py_tuple(vec![g_STAY(), g__STAY_CRITERIA(), g__STAY_WHERE()]);
    }
    if py_truthy(&mcall(&g__FOOD(), "search", &[said.clone()])) {
    return py_tuple(vec![g_FOOD(), g__FOOD_CRITERIA(), g__FOOD_WHERE()]);
    }
    if py_truthy(&mcall(&g__CLASS(), "search", &[said.clone()])) {
    return py_tuple(vec![g_CLASS(), g__CLASS_CRITERIA(), g__CLASS_WHERE()]);
    }
    if py_truthy(&mcall(&g__SIGHT(), "search", &[said.clone()])) {
    return py_tuple(vec![g_SIGHT(), g__SIGHT_CRITERIA(), g__SIGHT_WHERE()]);
    }
    if py_truthy(&{ let __b = mcall(&g__RAISING(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__WITH_CHILDREN(), "search", &[said.clone()]) } }) {
    return py_tuple(vec![g_FAMILY(), g__FAMILY_CRITERIA(), g__FAMILY_WHERE()]);
    }
    somewhere = call_builtin("bool", &[{ let __b = mcall(&g__PLACE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__TRIP(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = named.clone(); if py_truthy(&__b) { __b } else { travel.clone() } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g__WITH_CHILDREN(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = somewhere.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SAFE(), "search", &[said.clone()]))) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = other_subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&somewhere.clone())) } })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = mcall(&g__SAFE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _the_thing(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&somewhere.clone())) } } } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__HOME(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__TRIP(), "search", &[said.clone()]))) } })) } } } } } } } }) {
    return py_tuple(vec![g_WITH_CHILDREN(), g__CHILD_CRITERIA(), g__CHILD_WHERE()]);
    }
    if py_truthy(&{ let __b = mcall(&g__SAFE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PLACE(), "search", &[said.clone()]))) } }) {
    return py_tuple(vec![Py::None, Py::None, Py::None]);
    }
    if py_truthy(&mcall(&g__SAFE(), "search", &[said.clone()])) {
    return py_tuple(vec![g_SAFE_(), g__SAFE_CRITERIA(), g__SAFE_WHERE()]);
    }
    if py_truthy(&mcall(&g__WALK(), "search", &[said.clone()])) {
    return py_tuple(vec![g_WALK(), g__WALK_CRITERIA(), g__WALK_WHERE()]);
    }
    if py_truthy(&mcall(&g__TRIP(), "search", &[said.clone()])) {
    return py_tuple(vec![g_TRIP(), g__TRIP_CRITERIA(), g__TRIP_WHERE()]);
    }
    if py_truthy(&mcall(&g__HOME(), "search", &[said.clone()])) {
    return py_tuple(vec![g_HOME(), g__HOME_CRITERIA(), g__HOME_WHERE()]);
    }
    if py_truthy(&mcall(&g__SCREEN(), "search", &[said.clone()])) {
    return py_tuple(vec![g_SCREEN(), g__SCREEN_CRITERIA(), g__SCREEN_WHERE()]);
    }
    if py_truthy(&mcall(&g__JOB(), "search", &[said.clone()])) {
    return py_tuple(vec![g_JOB(), g__JOB_CRITERIA(), g__JOB_WHERE()]);
    }
    return py_tuple(vec![g_OTHER(), py_str(""), py_str("")]);
    Py::None
}
pub fn reads_as_a_verdict_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the bot to judge a thing it cannot see.");
    said = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_VERDICT_ASK(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__ABOUT_THE_BOT(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__FEELING(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__PAST_ASK(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), py_str("it"), py_str("that"), py_str("this"), py_str("they")]), &mcall(&_the_thing(&[said.clone()]), "lower", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NOT_A_JUDGEMENT(), "search", &[said.clone()])));
    Py::None
}
pub fn reads_as_a_hedge(args: &[Py]) -> Py {
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
    let mut opener: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply is the search hedge this file replaces.");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEDGE_OPENERS()) { let mut opener: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[opener.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_topics(args: &[Py]) -> Py {
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
    let mut topics: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The topics the understanding engine read the line as being about.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    topics = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("topic")])) {
    let _ = mcall(&topics.clone(), "add", &[geti(&understood.clone(), &py_str("topic"))]);
    }
    return topics.clone();
    Py::None
}
pub fn pool_for(args: &[Py]) -> Py {
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
    let mut criteria: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_lines: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The pool of replies for a judgement ask on a thing the bot cannot see, or None.\n\n    The caller picks from the pool with the memory in hand (`pick` needs the turns), so this\n    returns every phrasing for the kind of thing the line names.\n    ");
    raw = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    said = mcall(&raw.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_verdict_ask(&[said.clone()])))) {
    return Py::None;
    }
    topics = _the_topics(&[memory.clone()]);
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("named".to_string(), call_builtin("bool", &[mcall(&g__NAMED(), "search", &[raw.clone()])]).unwrap_or_else(|e| exc::raise(e))), ("other_subject".to_string(), call_builtin("bool", &[op_bitand(&topics.clone(), &g__NOT_A_PLACE_TOPICS())]).unwrap_or_else(|e| exc::raise(e))), ("travel".to_string(), call_builtin("bool", &[op_bitand(&topics.clone(), &g__TRAVEL_TOPICS())]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pool(args))(&__args) }, 3, None);
    pool = __unpk1[0].clone();
    criteria = __unpk1[1].clone();
    where_ = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &g_OTHER())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_thing(&[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &g_OTHER()))) {
    return pool.clone();
    }
    thing = _the_thing(&[said.clone()]);
    lead = if py_truthy(&thing.clone()) { op_add(&thing.clone(), &py_str(" — ")) } else { py_str("") };
    said_lines = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&pool.clone());
    for __item in __iter2 {
    line = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("lead".to_string(), lead.clone()), ("criteria".to_string(), criteria.clone()), ("where".to_string(), where_.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    let _ = mcall(&said_lines.clone(), "append", &[if py_truthy(&lead.clone()) { line.clone() } else { op_add(&mcall(&slice_or_raise(&line.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&line.clone(), &Py::Int(1), &Py::None, &Py::None)) }]);
    }
    return call_builtin("tuple", &[said_lines.clone()]).unwrap_or_else(|e| exc::raise(e));
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { py_str("") };
    let mut pool: Py = Py::None;
    let _ = py_str("A verdict ask the bot can help with, said honestly, or None.\n\n    `reply` is what the bot was going to say. Only the search hedge is replaced, so any entry, any\n    module and the composer's own answer all keep the turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_hedge(&[reply.clone()])))) {
    return Py::None;
    }
    pool = pool_for(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_place_nobody_can_see".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CLASS" => Some(g_CLASS()),
            "FAMILY" => Some(g_FAMILY()),
            "FOOD" => Some(g_FOOD()),
            "HEDGE_OPENERS" => Some(g_HEDGE_OPENERS()),
            "HOME" => Some(g_HOME()),
            "JOB" => Some(g_JOB()),
            "NOT_A_JUDGEMENT" => Some(g_NOT_A_JUDGEMENT()),
            "OTHER" => Some(g_OTHER()),
            "SAFE_" => Some(g_SAFE_()),
            "SAFE_OBJECT" => Some(g_SAFE_OBJECT()),
            "SCHOOL" => Some(g_SCHOOL()),
            "SCREEN" => Some(g_SCREEN()),
            "SIGHT" => Some(g_SIGHT()),
            "STAY" => Some(g_STAY()),
            "TRIP" => Some(g_TRIP()),
            "VERDICT_ASK" => Some(g_VERDICT_ASK()),
            "WALK" => Some(g_WALK()),
            "WITH_CHILDREN" => Some(g_WITH_CHILDREN()),
            "_ABOUT_THE_BOT" => Some(g__ABOUT_THE_BOT()),
            "_BARE_NOUN" => Some(g__BARE_NOUN()),
            "_CHILD_CRITERIA" => Some(g__CHILD_CRITERIA()),
            "_CHILD_WHERE" => Some(g__CHILD_WHERE()),
            "_CLASS" => Some(g__CLASS()),
            "_CLASS_CRITERIA" => Some(g__CLASS_CRITERIA()),
            "_CLASS_WHERE" => Some(g__CLASS_WHERE()),
            "_FAMILY_CRITERIA" => Some(g__FAMILY_CRITERIA()),
            "_FAMILY_WHERE" => Some(g__FAMILY_WHERE()),
            "_FEELING" => Some(g__FEELING()),
            "_FOOD" => Some(g__FOOD()),
            "_FOOD_CRITERIA" => Some(g__FOOD_CRITERIA()),
            "_FOOD_WHERE" => Some(g__FOOD_WHERE()),
            "_HOME" => Some(g__HOME()),
            "_HOME_CRITERIA" => Some(g__HOME_CRITERIA()),
            "_HOME_WHERE" => Some(g__HOME_WHERE()),
            "_JOB" => Some(g__JOB()),
            "_JOB_CRITERIA" => Some(g__JOB_CRITERIA()),
            "_JOB_WHERE" => Some(g__JOB_WHERE()),
            "_JUDGE" => Some(g__JUDGE()),
            "_NAMED" => Some(g__NAMED()),
            "_NOT_A_PLACE_TOPICS" => Some(g__NOT_A_PLACE_TOPICS()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_PAST_ASK" => Some(g__PAST_ASK()),
            "_PLACE" => Some(g__PLACE()),
            "_RAISING" => Some(g__RAISING()),
            "_SAFE" => Some(g__SAFE()),
            "_SAFE_CRITERIA" => Some(g__SAFE_CRITERIA()),
            "_SAFE_OBJECT" => Some(g__SAFE_OBJECT()),
            "_SAFE_OBJECT_CRITERIA" => Some(g__SAFE_OBJECT_CRITERIA()),
            "_SAFE_OBJECT_WHERE" => Some(g__SAFE_OBJECT_WHERE()),
            "_SAFE_WHERE" => Some(g__SAFE_WHERE()),
            "_SCHOOL" => Some(g__SCHOOL()),
            "_SCHOOL_CRITERIA" => Some(g__SCHOOL_CRITERIA()),
            "_SCHOOL_WHERE" => Some(g__SCHOOL_WHERE()),
            "_SCREEN" => Some(g__SCREEN()),
            "_SCREEN_CRITERIA" => Some(g__SCREEN_CRITERIA()),
            "_SCREEN_WHERE" => Some(g__SCREEN_WHERE()),
            "_SIGHT" => Some(g__SIGHT()),
            "_SIGHT_CRITERIA" => Some(g__SIGHT_CRITERIA()),
            "_SIGHT_WHERE" => Some(g__SIGHT_WHERE()),
            "_STAY" => Some(g__STAY()),
            "_STAY_CRITERIA" => Some(g__STAY_CRITERIA()),
            "_STAY_WHERE" => Some(g__STAY_WHERE()),
            "_STOP" => Some(g__STOP()),
            "_THING" => Some(g__THING()),
            "_TRAVEL_TOPICS" => Some(g__TRAVEL_TOPICS()),
            "_TRIP" => Some(g__TRIP()),
            "_TRIP_CRITERIA" => Some(g__TRIP_CRITERIA()),
            "_TRIP_WHERE" => Some(g__TRIP_WHERE()),
            "_WALK" => Some(g__WALK()),
            "_WALK_CRITERIA" => Some(g__WALK_CRITERIA()),
            "_WALK_WHERE" => Some(g__WALK_WHERE()),
            "_WITH_CHILDREN" => Some(g__WITH_CHILDREN()),
            "_pool" => Some(py_func("_pool", |args: &[Py]| _pool(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_topics" => Some(py_func("_the_topics", |args: &[Py]| _the_topics(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            "reads_as_a_hedge" => Some(py_func("reads_as_a_hedge", |args: &[Py]| reads_as_a_hedge(args))),
            "reads_as_a_verdict_ask" => Some(py_func("reads_as_a_verdict_ask", |args: &[Py]| reads_as_a_verdict_ask(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CLASS" => s_CLASS(v),
            "FAMILY" => s_FAMILY(v),
            "FOOD" => s_FOOD(v),
            "HEDGE_OPENERS" => s_HEDGE_OPENERS(v),
            "HOME" => s_HOME(v),
            "JOB" => s_JOB(v),
            "NOT_A_JUDGEMENT" => s_NOT_A_JUDGEMENT(v),
            "OTHER" => s_OTHER(v),
            "SAFE_" => s_SAFE_(v),
            "SAFE_OBJECT" => s_SAFE_OBJECT(v),
            "SCHOOL" => s_SCHOOL(v),
            "SCREEN" => s_SCREEN(v),
            "SIGHT" => s_SIGHT(v),
            "STAY" => s_STAY(v),
            "TRIP" => s_TRIP(v),
            "VERDICT_ASK" => s_VERDICT_ASK(v),
            "WALK" => s_WALK(v),
            "WITH_CHILDREN" => s_WITH_CHILDREN(v),
            "_ABOUT_THE_BOT" => s__ABOUT_THE_BOT(v),
            "_BARE_NOUN" => s__BARE_NOUN(v),
            "_CHILD_CRITERIA" => s__CHILD_CRITERIA(v),
            "_CHILD_WHERE" => s__CHILD_WHERE(v),
            "_CLASS" => s__CLASS(v),
            "_CLASS_CRITERIA" => s__CLASS_CRITERIA(v),
            "_CLASS_WHERE" => s__CLASS_WHERE(v),
            "_FAMILY_CRITERIA" => s__FAMILY_CRITERIA(v),
            "_FAMILY_WHERE" => s__FAMILY_WHERE(v),
            "_FEELING" => s__FEELING(v),
            "_FOOD" => s__FOOD(v),
            "_FOOD_CRITERIA" => s__FOOD_CRITERIA(v),
            "_FOOD_WHERE" => s__FOOD_WHERE(v),
            "_HOME" => s__HOME(v),
            "_HOME_CRITERIA" => s__HOME_CRITERIA(v),
            "_HOME_WHERE" => s__HOME_WHERE(v),
            "_JOB" => s__JOB(v),
            "_JOB_CRITERIA" => s__JOB_CRITERIA(v),
            "_JOB_WHERE" => s__JOB_WHERE(v),
            "_JUDGE" => s__JUDGE(v),
            "_NAMED" => s__NAMED(v),
            "_NOT_A_PLACE_TOPICS" => s__NOT_A_PLACE_TOPICS(v),
            "_NOT_A_THING" => s__NOT_A_THING(v),
            "_PAST_ASK" => s__PAST_ASK(v),
            "_PLACE" => s__PLACE(v),
            "_RAISING" => s__RAISING(v),
            "_SAFE" => s__SAFE(v),
            "_SAFE_CRITERIA" => s__SAFE_CRITERIA(v),
            "_SAFE_OBJECT" => s__SAFE_OBJECT(v),
            "_SAFE_OBJECT_CRITERIA" => s__SAFE_OBJECT_CRITERIA(v),
            "_SAFE_OBJECT_WHERE" => s__SAFE_OBJECT_WHERE(v),
            "_SAFE_WHERE" => s__SAFE_WHERE(v),
            "_SCHOOL" => s__SCHOOL(v),
            "_SCHOOL_CRITERIA" => s__SCHOOL_CRITERIA(v),
            "_SCHOOL_WHERE" => s__SCHOOL_WHERE(v),
            "_SCREEN" => s__SCREEN(v),
            "_SCREEN_CRITERIA" => s__SCREEN_CRITERIA(v),
            "_SCREEN_WHERE" => s__SCREEN_WHERE(v),
            "_SIGHT" => s__SIGHT(v),
            "_SIGHT_CRITERIA" => s__SIGHT_CRITERIA(v),
            "_SIGHT_WHERE" => s__SIGHT_WHERE(v),
            "_STAY" => s__STAY(v),
            "_STAY_CRITERIA" => s__STAY_CRITERIA(v),
            "_STAY_WHERE" => s__STAY_WHERE(v),
            "_STOP" => s__STOP(v),
            "_THING" => s__THING(v),
            "_TRAVEL_TOPICS" => s__TRAVEL_TOPICS(v),
            "_TRIP" => s__TRIP(v),
            "_TRIP_CRITERIA" => s__TRIP_CRITERIA(v),
            "_TRIP_WHERE" => s__TRIP_WHERE(v),
            "_WALK" => s__WALK(v),
            "_WALK_CRITERIA" => s__WALK_CRITERIA(v),
            "_WALK_WHERE" => s__WALK_WHERE(v),
            "_WITH_CHILDREN" => s__WITH_CHILDREN(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_place_nobody_can_see", module_obj);
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "_pool", py_func("_pool", |args: &[Py]| _pool(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "reads_as_a_verdict_ask", py_func("reads_as_a_verdict_ask", |args: &[Py]| reads_as_a_verdict_ask(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "reads_as_a_hedge", py_func("reads_as_a_hedge", |args: &[Py]| reads_as_a_hedge(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "_the_topics", py_func("_the_topics", |args: &[Py]| _the_topics(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "pool_for", py_func("pool_for", |args: &[Py]| pool_for(args)));
    pyrt::xrefs::reg_fn("compose.a_place_nobody_can_see", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "CLASS", g_CLASS, s_CLASS);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "FOOD", g_FOOD, s_FOOD);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "HEDGE_OPENERS", g_HEDGE_OPENERS, s_HEDGE_OPENERS);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "HOME", g_HOME, s_HOME);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "JOB", g_JOB, s_JOB);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "NOT_A_JUDGEMENT", g_NOT_A_JUDGEMENT, s_NOT_A_JUDGEMENT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "OTHER", g_OTHER, s_OTHER);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "SAFE_", g_SAFE_, s_SAFE_);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "SAFE_OBJECT", g_SAFE_OBJECT, s_SAFE_OBJECT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "SCHOOL", g_SCHOOL, s_SCHOOL);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "SCREEN", g_SCREEN, s_SCREEN);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "SIGHT", g_SIGHT, s_SIGHT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "STAY", g_STAY, s_STAY);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "TRIP", g_TRIP, s_TRIP);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "VERDICT_ASK", g_VERDICT_ASK, s_VERDICT_ASK);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "WALK", g_WALK, s_WALK);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "WITH_CHILDREN", g_WITH_CHILDREN, s_WITH_CHILDREN);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_ABOUT_THE_BOT", g__ABOUT_THE_BOT, s__ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_BARE_NOUN", g__BARE_NOUN, s__BARE_NOUN);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_CHILD_CRITERIA", g__CHILD_CRITERIA, s__CHILD_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_CHILD_WHERE", g__CHILD_WHERE, s__CHILD_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_CLASS", g__CLASS, s__CLASS);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_CLASS_CRITERIA", g__CLASS_CRITERIA, s__CLASS_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_CLASS_WHERE", g__CLASS_WHERE, s__CLASS_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FAMILY_CRITERIA", g__FAMILY_CRITERIA, s__FAMILY_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FAMILY_WHERE", g__FAMILY_WHERE, s__FAMILY_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FEELING", g__FEELING, s__FEELING);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FOOD", g__FOOD, s__FOOD);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FOOD_CRITERIA", g__FOOD_CRITERIA, s__FOOD_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_FOOD_WHERE", g__FOOD_WHERE, s__FOOD_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_HOME", g__HOME, s__HOME);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_HOME_CRITERIA", g__HOME_CRITERIA, s__HOME_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_HOME_WHERE", g__HOME_WHERE, s__HOME_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_JOB", g__JOB, s__JOB);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_JOB_CRITERIA", g__JOB_CRITERIA, s__JOB_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_JOB_WHERE", g__JOB_WHERE, s__JOB_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_JUDGE", g__JUDGE, s__JUDGE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_NAMED", g__NAMED, s__NAMED);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_NOT_A_PLACE_TOPICS", g__NOT_A_PLACE_TOPICS, s__NOT_A_PLACE_TOPICS);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_NOT_A_THING", g__NOT_A_THING, s__NOT_A_THING);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_PAST_ASK", g__PAST_ASK, s__PAST_ASK);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_PLACE", g__PLACE, s__PLACE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_RAISING", g__RAISING, s__RAISING);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE", g__SAFE, s__SAFE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE_CRITERIA", g__SAFE_CRITERIA, s__SAFE_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE_OBJECT", g__SAFE_OBJECT, s__SAFE_OBJECT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE_OBJECT_CRITERIA", g__SAFE_OBJECT_CRITERIA, s__SAFE_OBJECT_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE_OBJECT_WHERE", g__SAFE_OBJECT_WHERE, s__SAFE_OBJECT_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SAFE_WHERE", g__SAFE_WHERE, s__SAFE_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCHOOL", g__SCHOOL, s__SCHOOL);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCHOOL_CRITERIA", g__SCHOOL_CRITERIA, s__SCHOOL_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCHOOL_WHERE", g__SCHOOL_WHERE, s__SCHOOL_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCREEN", g__SCREEN, s__SCREEN);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCREEN_CRITERIA", g__SCREEN_CRITERIA, s__SCREEN_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SCREEN_WHERE", g__SCREEN_WHERE, s__SCREEN_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SIGHT", g__SIGHT, s__SIGHT);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SIGHT_CRITERIA", g__SIGHT_CRITERIA, s__SIGHT_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_SIGHT_WHERE", g__SIGHT_WHERE, s__SIGHT_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_STAY", g__STAY, s__STAY);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_STAY_CRITERIA", g__STAY_CRITERIA, s__STAY_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_STAY_WHERE", g__STAY_WHERE, s__STAY_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_STOP", g__STOP, s__STOP);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_THING", g__THING, s__THING);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_TRAVEL_TOPICS", g__TRAVEL_TOPICS, s__TRAVEL_TOPICS);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_TRIP", g__TRIP, s__TRIP);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_TRIP_CRITERIA", g__TRIP_CRITERIA, s__TRIP_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_TRIP_WHERE", g__TRIP_WHERE, s__TRIP_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_WALK", g__WALK, s__WALK);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_WALK_CRITERIA", g__WALK_CRITERIA, s__WALK_CRITERIA);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_WALK_WHERE", g__WALK_WHERE, s__WALK_WHERE);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "_WITH_CHILDREN", g__WITH_CHILDREN, s__WITH_CHILDREN);
    pyrt::xrefs::reg_access("compose.a_place_nobody_can_see", "pick", g_pick, s_pick);
}

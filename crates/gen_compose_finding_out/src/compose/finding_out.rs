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


thread_local! { static G_ABOUT_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_TO_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_NOT_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_NOT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_TAIL_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_THING_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEMISTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTACT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLING_IT_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GET_TO_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOES_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIRING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_THE_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MATERIALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MATERIAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_METHOD_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_HIRE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOURNEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_LOOKUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_WRECKED_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_A_SCREEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPEN_CHEMIST_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPEN_CHEMIST_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PROFESSIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICKED_IN_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_ANYWHERE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ROUTE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLICY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROFESSIONAL_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTING_TRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_READERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECORD_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECORD_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECORD_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAVING_FAILED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_JOB_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_NOT_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTING_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKE_TO_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BARE_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORK_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIDE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIDE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIDE_WHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORTH_VISITING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRECKED_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THE_BOT() -> Py { ensure_init(); G_ABOUT_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT(v: Py) { ensure_init(); G_ABOUT_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_TO_START() -> Py { ensure_init(); G_BACK_TO_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_TO_START(v: Py) { ensure_init(); G_BACK_TO_START.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_ASK() -> Py { ensure_init(); G_BUY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_ASK(v: Py) { ensure_init(); G_BUY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_NOT() -> Py { ensure_init(); G_BUY_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_NOT(v: Py) { ensure_init(); G_BUY_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_NOT_PEOPLE() -> Py { ensure_init(); G_BUY_NOT_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_NOT_PEOPLE(v: Py) { ensure_init(); G_BUY_NOT_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_NOT_THE_THING() -> Py { ensure_init(); G_BUY_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_NOT_THE_THING(v: Py) { ensure_init(); G_BUY_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_NOT_THINGS() -> Py { ensure_init(); G_BUY_NOT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_NOT_THINGS(v: Py) { ensure_init(); G_BUY_NOT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_POOL() -> Py { ensure_init(); G_BUY_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_POOL(v: Py) { ensure_init(); G_BUY_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_TAIL() -> Py { ensure_init(); G_BUY_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_TAIL(v: Py) { ensure_init(); G_BUY_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_TAIL_PHRASES() -> Py { ensure_init(); G_BUY_TAIL_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_TAIL_PHRASES(v: Py) { ensure_init(); G_BUY_TAIL_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_THING() -> Py { ensure_init(); G_BUY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_THING(v: Py) { ensure_init(); G_BUY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_THING_STOPS() -> Py { ensure_init(); G_BUY_THING_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_THING_STOPS(v: Py) { ensure_init(); G_BUY_THING_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEMISTS() -> Py { ensure_init(); G_CHEMISTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEMISTS(v: Py) { ensure_init(); G_CHEMISTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTACT_ASKS() -> Py { ensure_init(); G_CONTACT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTACT_ASKS(v: Py) { ensure_init(); G_CONTACT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_ASKS() -> Py { ensure_init(); G_FIGURE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ASKS(v: Py) { ensure_init(); G_FIGURE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_POOL() -> Py { ensure_init(); G_FIGURE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_POOL(v: Py) { ensure_init(); G_FIGURE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLING_IT_IN() -> Py { ensure_init(); G_FILLING_IT_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLING_IT_IN(v: Py) { ensure_init(); G_FILLING_IT_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_GET_TO_ASKS() -> Py { ensure_init(); G_GET_TO_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GET_TO_ASKS(v: Py) { ensure_init(); G_GET_TO_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_GOES_TO() -> Py { ensure_init(); G_GOES_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOES_TO(v: Py) { ensure_init(); G_GOES_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_HIRING() -> Py { ensure_init(); G_HIRING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIRING(v: Py) { ensure_init(); G_HIRING.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY_POOL() -> Py { ensure_init(); G_HOW_MANY_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_POOL(v: Py) { ensure_init(); G_HOW_MANY_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_PLACE() -> Py { ensure_init(); G_IN_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_PLACE(v: Py) { ensure_init(); G_IN_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_THE_WORK() -> Py { ensure_init(); G_LOST_THE_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_THE_WORK(v: Py) { ensure_init(); G_LOST_THE_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_MATERIALS() -> Py { ensure_init(); G_MATERIALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MATERIALS(v: Py) { ensure_init(); G_MATERIALS.with(|c| *c.borrow_mut() = v); }
pub fn g_MATERIAL_ASK() -> Py { ensure_init(); G_MATERIAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MATERIAL_ASK(v: Py) { ensure_init(); G_MATERIAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_METHOD_ASKS() -> Py { ensure_init(); G_METHOD_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_METHOD_ASKS(v: Py) { ensure_init(); G_METHOD_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAR_ME() -> Py { ensure_init(); G_NEAR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAR_ME(v: Py) { ensure_init(); G_NEAR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_NAMED() -> Py { ensure_init(); G_NOBODY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_NAMED(v: Py) { ensure_init(); G_NOBODY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_HIRE() -> Py { ensure_init(); G_NOT_A_HIRE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_HIRE(v: Py) { ensure_init(); G_NOT_A_HIRE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOURNEY() -> Py { ensure_init(); G_NOT_A_JOURNEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOURNEY(v: Py) { ensure_init(); G_NOT_A_JOURNEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_LOOKUP() -> Py { ensure_init(); G_NOT_A_LOOKUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LOOKUP(v: Py) { ensure_init(); G_NOT_A_LOOKUP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PICK() -> Py { ensure_init(); G_NOT_A_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PICK(v: Py) { ensure_init(); G_NOT_A_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TRADE() -> Py { ensure_init(); G_NOT_A_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TRADE(v: Py) { ensure_init(); G_NOT_A_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_WRECKED_FORM() -> Py { ensure_init(); G_NOT_A_WRECKED_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_WRECKED_FORM(v: Py) { ensure_init(); G_NOT_A_WRECKED_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_A_SCREEN() -> Py { ensure_init(); G_ON_A_SCREEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_A_SCREEN(v: Py) { ensure_init(); G_ON_A_SCREEN.with(|c| *c.borrow_mut() = v); }
pub fn g_OPEN_CHEMIST_HOURS() -> Py { ensure_init(); G_OPEN_CHEMIST_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPEN_CHEMIST_HOURS(v: Py) { ensure_init(); G_OPEN_CHEMIST_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPEN_CHEMIST_POOL() -> Py { ensure_init(); G_OPEN_CHEMIST_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPEN_CHEMIST_POOL(v: Py) { ensure_init(); G_OPEN_CHEMIST_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PERSON() -> Py { ensure_init(); G_OWN_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PERSON(v: Py) { ensure_init(); G_OWN_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PROFESSIONAL() -> Py { ensure_init(); G_OWN_PROFESSIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PROFESSIONAL(v: Py) { ensure_init(); G_OWN_PROFESSIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PICKED_IN_A_PLACE() -> Py { ensure_init(); G_PICKED_IN_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICKED_IN_A_PLACE(v: Py) { ensure_init(); G_PICKED_IN_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_ANYWHERE_POOL() -> Py { ensure_init(); G_PICK_ANYWHERE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_ANYWHERE_POOL(v: Py) { ensure_init(); G_PICK_ANYWHERE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_ASKS() -> Py { ensure_init(); G_PICK_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_ASKS(v: Py) { ensure_init(); G_PICK_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_POOL() -> Py { ensure_init(); G_PICK_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_POOL(v: Py) { ensure_init(); G_PICK_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_NOUNS() -> Py { ensure_init(); G_PLACE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_NOUNS(v: Py) { ensure_init(); G_PLACE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_POOL() -> Py { ensure_init(); G_PLACE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_POOL(v: Py) { ensure_init(); G_PLACE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ROUTE_POOL() -> Py { ensure_init(); G_PLACE_ROUTE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ROUTE_POOL(v: Py) { ensure_init(); G_PLACE_ROUTE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_POLICY_WORDS() -> Py { ensure_init(); G_POLICY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLICY_WORDS(v: Py) { ensure_init(); G_POLICY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PROFESSIONAL_POOL() -> Py { ensure_init(); G_PROFESSIONAL_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROFESSIONAL_POOL(v: Py) { ensure_init(); G_PROFESSIONAL_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTING_TRADES() -> Py { ensure_init(); G_QUOTING_TRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTING_TRADES(v: Py) { ensure_init(); G_QUOTING_TRADES.with(|c| *c.borrow_mut() = v); }
pub fn g_READERS() -> Py { ensure_init(); G_READERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_READERS(v: Py) { ensure_init(); G_READERS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECORD_ASKED() -> Py { ensure_init(); G_RECORD_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECORD_ASKED(v: Py) { ensure_init(); G_RECORD_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_RECORD_ASKS() -> Py { ensure_init(); G_RECORD_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECORD_ASKS(v: Py) { ensure_init(); G_RECORD_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECORD_POOL() -> Py { ensure_init(); G_RECORD_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECORD_POOL(v: Py) { ensure_init(); G_RECORD_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTE_ASKS() -> Py { ensure_init(); G_ROUTE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTE_ASKS(v: Py) { ensure_init(); G_ROUTE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAVING_FAILED() -> Py { ensure_init(); G_SAVING_FAILED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAVING_FAILED(v: Py) { ensure_init(); G_SAVING_FAILED.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_ASK() -> Py { ensure_init(); G_SERVICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_ASK(v: Py) { ensure_init(); G_SERVICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_JOB_NAMES() -> Py { ensure_init(); G_SERVICE_JOB_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_JOB_NAMES(v: Py) { ensure_init(); G_SERVICE_JOB_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_NOT_A_THING() -> Py { ensure_init(); G_SERVICE_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_NOT_A_THING(v: Py) { ensure_init(); G_SERVICE_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_NOT_HEALTH() -> Py { ensure_init(); G_SERVICE_NOT_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_NOT_HEALTH(v: Py) { ensure_init(); G_SERVICE_NOT_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_NOUNS() -> Py { ensure_init(); G_SERVICE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_NOUNS(v: Py) { ensure_init(); G_SERVICE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_PHRASES() -> Py { ensure_init(); G_SERVICE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_PHRASES(v: Py) { ensure_init(); G_SERVICE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_POOL() -> Py { ensure_init(); G_SERVICE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_POOL(v: Py) { ensure_init(); G_SERVICE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_STOPS() -> Py { ensure_init(); G_SERVICE_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_STOPS(v: Py) { ensure_init(); G_SERVICE_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_THING() -> Py { ensure_init(); G_SERVICE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_THING(v: Py) { ensure_init(); G_SERVICE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE_WORK() -> Py { ensure_init(); G_SERVICE_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE_WORK(v: Py) { ensure_init(); G_SERVICE_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTING_ASKS() -> Py { ensure_init(); G_SETTING_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTING_ASKS(v: Py) { ensure_init(); G_SETTING_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKE_TO_ASKS() -> Py { ensure_init(); G_TAKE_TO_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKE_TO_ASKS(v: Py) { ensure_init(); G_TAKE_TO_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BARE_FORMS() -> Py { ensure_init(); G_THE_BARE_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BARE_FORMS(v: Py) { ensure_init(); G_THE_BARE_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORK_ITSELF() -> Py { ensure_init(); G_THE_WORK_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORK_ITSELF(v: Py) { ensure_init(); G_THE_WORK_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_TIDE_ASK() -> Py { ensure_init(); G_TIDE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIDE_ASK(v: Py) { ensure_init(); G_TIDE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TIDE_POOL() -> Py { ensure_init(); G_TIDE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIDE_POOL(v: Py) { ensure_init(); G_TIDE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_TIDE_WHY() -> Py { ensure_init(); G_TIDE_WHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIDE_WHY(v: Py) { ensure_init(); G_TIDE_WHY.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADES() -> Py { ensure_init(); G_TRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADES(v: Py) { ensure_init(); G_TRADES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADE_POOL() -> Py { ensure_init(); G_TRADE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADE_POOL(v: Py) { ensure_init(); G_TRADE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_TOPICS() -> Py { ensure_init(); G_TRAVEL_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_TOPICS(v: Py) { ensure_init(); G_TRAVEL_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_WORDS() -> Py { ensure_init(); G_TRAVEL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_WORDS(v: Py) { ensure_init(); G_TRAVEL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASKS() -> Py { ensure_init(); G_WHERE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASKS(v: Py) { ensure_init(); G_WHERE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_FACT() -> Py { ensure_init(); G_WHO_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_FACT(v: Py) { ensure_init(); G_WHO_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_POOL() -> Py { ensure_init(); G_WHO_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_POOL(v: Py) { ensure_init(); G_WHO_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_WORTH_VISITING() -> Py { ensure_init(); G_WORTH_VISITING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORTH_VISITING(v: Py) { ensure_init(); G_WORTH_VISITING.with(|c| *c.borrow_mut() = v); }
pub fn g_WRECKED_POOL() -> Py { ensure_init(); G_WRECKED_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRECKED_POOL(v: Py) { ensure_init(); G_WRECKED_POOL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The answer the bot does not hold, and the one place that has it.\n\nNothing in the bot knew \"whats the postcode for buckingham palace\", \"find me a plumber near me\"\nor \"who plays in the beatles\". The composer's honest pool answered all three with the same\nsentence - \"a quick search online will get you a solid answer\" - which is true, and useless: it\nnames neither the thing that was asked about nor the place that holds it, so a person is told to\ngo and look without being told where. The decks are full of turns like these, and the bot has one\njob in all of them: say in a few words what it cannot see (a branch's hours, who works near you,\na figure nobody has told it) and name the one thing that can, then stay for the part that is\nstill the bot's.\n\nSo this reads the shape of the ask and the thing named in it, and answers with the route a\nfriend would give:\n\n* a place, an address, a postcode or a phone number -> a map app, which has all four at once;\n* somebody to do a job -> a directory sorted by reviews, and two quotes before booking;\n* something to do or stay in somewhere -> the reviews and the visitor site, in that order;\n* a fact about a named thing, a band or a person -> the page that keeps it, with its sources;\n* a figure about the world -> the same page, because a guessed number is the one thing worse\n  than no number;\n* a journey -> a map app or a journey planner, which knows the roads it has never seen;\n* \"how many do I need\" for a material -> the two measurements, and the bot does the sum itself;\n* where to buy a thing -> a map app search, and ring before setting off.\n\nEvery reader returns None for what it cannot route better than the hedge, and the hedge is what\na line with no shape here keeps. Nothing is ever filled in with an invented fact: the whole point\nof the route is that the bot says what it does not know and hands over the one place that does.\n");
    s_OWN_PERSON(call_builtin("frozenset", &[mcall(&py_str("\nmy our his her their i me we us myself himself herself yourself husband wife partner boyfriend\ngirlfriend fiance fiancee mum mom mother dad father son daughter kid kids child children baby\nnan nana grandma grandad granny sister brother friend friends boss manager landlord landlady\nneighbour neighbor colleague carer stepmum stepdad stepson stepdaughter nephew niece\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOBODY_NAMED(call_builtin("frozenset", &[mcall(&py_str("\nit that this these those he she they him her them you your me us i we someone somebody anyone\nanybody everyone everybody something anything nothing\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_BOT(py_tuple(vec![py_str("made you"), py_str("created you"), py_str("built you"), py_str("who are you"), py_str("what are you"), py_str("who made u"), py_str("who created u"), py_str("made u"), py_str("your name"), py_str("were you made"), py_str("invented you"), py_str("designed you")]));
    fn __data1() -> Py { py_tuple(vec![py_str("postcode"), py_str("postal code"), py_str("phone number"), py_str("telephone number"), py_str("telephone"), py_str("contact number"), py_str("contact details"), py_str("their number"), py_str("the number for"), py_str("number for the"), py_str("whats the number"), py_str("what's the number"), py_str("what is the number"), py_str("the address"), py_str("address of"), py_str("address for")]) }
    s_CONTACT_ASKS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("where is the"), py_str("where's the"), py_str("wheres the"), py_str("weres the"), py_str("where is a"), py_str("where is an"), py_str("where can i find"), py_str("where do i find"), py_str("nearest branch"), py_str("nearest one"), py_str("nearest shop"), py_str("nearest store"), py_str("nearest office"), py_str("closest branch"), py_str("nearest chemist"), py_str("nearest pharmacy")]) }
    s_WHERE_ASKS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("station"), py_str("airport"), py_str("office"), py_str("shop"), py_str("store"), py_str("supermarket"), py_str("bank"), py_str("post office"), py_str("tip"), py_str("recycling centre"), py_str("dump"), py_str("library"), py_str("pool"), py_str("swimming pool"), py_str("leisure centre"), py_str("park"), py_str("car park"), py_str("car park"), py_str("surgery"), py_str("pharmacy"), py_str("chemist"), py_str("hospital"), py_str("museum"), py_str("hotel"), py_str("restaurant"), py_str("pub"), py_str("cafe"), py_str("church"), py_str("cathedral"), py_str("school"), py_str("college"), py_str("university"), py_str("market"), py_str("garage"), py_str("petrol station"), py_str("bus stop"), py_str("train station"), py_str("coach station"), py_str("toilets"), py_str("loo"), py_str("customer service"), py_str("reception"), py_str("depot"), py_str("showroom"), py_str("river"), py_str("lake"), py_str("mountain"), py_str("hill"), py_str("beach"), py_str("coast"), py_str("forest"), py_str("wood"), py_str("castle"), py_str("palace"), py_str("bridge"), py_str("square"), py_str("street"), py_str("road"), py_str("lane"), py_str("avenue"), py_str("village"), py_str("town"), py_str("city"), py_str("centre"), py_str("center")]) }
    s_PLACE_NOUNS(__data3());
    s_NOT_A_PLACE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:parcel|order|delivery|refund|application|claim|appointment|interview|flight|connection|luggage|suitcase|passport|keys|wallet|phone|remote|charger|filter|point of|point in|sense of|point to|best place to put|best place for it)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_LOOKUP(stdlib_call("re", "compile", &[py_str("^\\s*who\\b|(?<![a-z])(?:how do you say|how do i say|how would you say|how to say|what does [a-z' ]{1,20} mean|translate|in french|in spanish|in german|in italian|in welsh)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_POOL(py_tuple(vec![py_str("A postcode, a phone number and opening hours all change hands, so I can't hold them: a map app turns the name into the address, the postcode and the number, and the place's own page has today's hours."), py_str("The place's own page is the one with today's hours, and a map app has the address and the number, so tell me what they say and I'll help with the rest - I can't look either up from here.")]));
    fn __data4() -> Py { py_tuple(vec![py_str("train"), py_str("trains"), py_str("station"), py_str("airport"), py_str("coach"), py_str("bus"), py_str("buses"), py_str("flight"), py_str("flights"), py_str("plane"), py_str("ferry"), py_str("tram"), py_str("tube"), py_str("underground"), py_str("motorway"), py_str("road"), py_str("drive"), py_str("driving"), py_str("journey"), py_str("commute"), py_str("cycle"), py_str("walk"), py_str("route"), py_str("miles"), py_str("distance"), py_str("terminals"), py_str("platform"), py_str("coast"), py_str("beach"), py_str("island"), py_str("seaside"), py_str("harbour"), py_str("harbor"), py_str("port"), py_str("stadium"), py_str("venue"), py_str("town centre"), py_str("city centre")]) }
    s_TRAVEL_WORDS(__data4());
    s_GET_TO_ASKS(py_tuple(vec![py_str("how do i get to"), py_str("how do you get to"), py_str("how do i get from"), py_str("how to get to"), py_str("how to get from"), py_str("directions to"), py_str("the way to"), py_str("get to"), py_str("get from"), py_str("way to get to"), py_str("way to get from"), py_str("way into town")]));
    s_ROUTE_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how far (?:is|are) (?:it |that )?(?:from|away from)|how many miles (?:is it |are there )?from|how long does it take to (?:get|travel|drive|walk|cycle|go)\\b|how far is it\\b|best route from|distance (?:from|between))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("in advance"), py_str("get over"), py_str("get fit"), py_str("get better"), py_str("get decent"), py_str("get going"), py_str("get there with"), py_str("how do i get a"), py_str("how do i get my"), py_str("how do i get more"), py_str("how do i get rid"), py_str("how do i get out of"), py_str("get toned"), py_str("get crispy"), py_str("get to sleep"), py_str("get up"), py_str("get through"), py_str("get started"), py_str("get going"), py_str("get a passport"), py_str("get an appointment"), py_str("get a visa"), py_str("get my passport"), py_str("get a driving test"), py_str("get my licence"), py_str("get a refund"), py_str("get pregnant")]) }
    s_NOT_A_JOURNEY(__data5());
    s_TAKE_TO_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])how long does it take to(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("airport"), py_str("station"), py_str("town"), py_str("city"), py_str("coast"), py_str("beach"), py_str("work"), py_str("school"), py_str("office"), py_str("home"), py_str("town centre"), py_str("city centre"), py_str("train station"), py_str("bus stop")]) }
    s_GOES_TO(op_add(&call_builtin("tuple", &[g_PLACE_NOUNS()]).unwrap_or_else(|e| exc::raise(e)), &__data6()));
    s_PLACE_ROUTE_POOL(py_tuple(vec![py_str("Miles and minutes depend on the roads on the day, which is why a journey planner is the one to ask - I can't measure one from here: put the two places in and it gives the distance and the time in one go."), py_str("The roads decide it rather than the map, so I can't give you the miles and the time myself: put the two places into a map app and it'll have both; say what you're planning and I'll help with the rest of it.")]));
    fn __data7() -> Py { py_tuple(vec![py_str("plumber"), py_str("electrician"), py_str("builder"), py_str("builders"), py_str("roofer"), py_str("painter"), py_str("decorator"), py_str("joiner"), py_str("carpenter"), py_str("plasterer"), py_str("tiler"), py_str("gardener"), py_str("cleaner"), py_str("locksmith"), py_str("boiler engineer"), py_str("gas engineer"), py_str("heating engineer"), py_str("accountant"), py_str("bookkeeper"), py_str("solicitor"), py_str("lawyer"), py_str("conveyancer"), py_str("dentist"), py_str("optician"), py_str("vet"), py_str("mechanic"), py_str("tutor"), py_str("physio"), py_str("physiotherapist"), py_str("chiropodist"), py_str("hairdresser"), py_str("barber"), py_str("mover"), py_str("movers"), py_str("removals"), py_str("surveyor"), py_str("architect"), py_str("tree surgeon"), py_str("pest controller"), py_str("chimney sweep"), py_str("appliance repair"), py_str("man with a van")]) }
    s_TRADES(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("find"), py_str("recommend"), py_str("any good"), py_str("know a"), py_str("know any"), py_str("know of"), py_str("looking for"), py_str("need a"), py_str("need an"), py_str("need someone"), py_str("someone who"), py_str("near me"), py_str("nearby"), py_str("local"), py_str("in my area"), py_str("can anyone"), py_str("suggest"), py_str("anybody"), py_str("anyone"), py_str("ne1"), py_str("some1"), py_str("any1")]) }
    s_HIRING(__data8());
    s_NOT_A_TRADE(py_tuple(vec![py_str("vacuum cleaner"), py_str("steam cleaner"), py_str("carpet cleaner"), py_str("oven cleaner"), py_str("drain cleaner"), py_str("glass cleaner"), py_str("mascara"), py_str("lawyered")]));
    fn __data9() -> Py { py_tuple(vec![py_str("do i need a"), py_str("do i need an"), py_str("will i need a"), py_str("will i need an"), py_str("should i get a"), py_str("should i get an"), py_str("myself"), py_str("died"), py_str("death"), py_str("bereave"), py_str("funeral"), py_str("complaint"), py_str("complain"), py_str("how do i"), py_str("how to"), py_str("why did"), py_str("my own"), py_str("sack"), py_str("dismiss"), py_str("tribunal"), py_str("quote from"), py_str("already got"), py_str("says i need"), py_str("told me i need"), py_str("said i need"), py_str("joining"), py_str("joined"), py_str("registered with"), py_str("agreeing a price with"), py_str("already have"), py_str("already seeing")]) }
    s_NOT_A_HIRE(__data9());
    s_OWN_PROFESSIONAL(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z])my\\s+(?:[a-z]+\\s+){0,2}(?:"), &mcall(&py_str("|"), "join", &[g_TRADES()])), &py_str(")(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data10() -> Py { py_tuple(vec![py_str("plumber"), py_str("electrician"), py_str("builder"), py_str("builders"), py_str("roofer"), py_str("painter"), py_str("decorator"), py_str("joiner"), py_str("carpenter"), py_str("plasterer"), py_str("tiler"), py_str("gardener"), py_str("locksmith"), py_str("boiler engineer"), py_str("gas engineer"), py_str("heating engineer"), py_str("mover"), py_str("movers"), py_str("removals"), py_str("tree surgeon"), py_str("pest controller"), py_str("chimney sweep"), py_str("appliance repair"), py_str("man with a van"), py_str("cleaner"), py_str("mechanic")]) }
    s_QUOTING_TRADES(__data10());
    s_TRADE_POOL(py_tuple(vec![py_str("Who is any good and who is free changes month to month, so I can't vouch for one: a directory or map app search for {trade} near you sorted by the recent reviews, then two written quotes before you book anyone."), py_str("The recent reviews are what tells you and I can't see them from here: a map app search for {trade} near you sorted by reviews is the quickest way in; ask for two quotes, and check they're registered before you pay a deposit.")]));
    s_PROFESSIONAL_POOL(py_tuple(vec![py_str("Whether they're taking new people and how long the wait is are the two things that decide it, and neither is something I can see: a map app or the profession's own finder lists them by area - ring two and ask about the wait and the fee before you go."), py_str("The wait and the fee are the parts that decide it and I can't see either from here: a map app or the official finder for {trade} near you shows who is taking on new people, so read the recent reviews and ring before you set off.")]));
    fn __data11() -> Py { py_tuple(vec![py_str("place"), py_str("places"), py_str("area"), py_str("part"), py_str("parts"), py_str("bit"), py_str("side"), py_str("neighbourhood"), py_str("neighborhood"), py_str("quarter"), py_str("hotel"), py_str("hotels"), py_str("restaurant"), py_str("restaurants"), py_str("pub"), py_str("pubs"), py_str("bar"), py_str("bars"), py_str("cafe"), py_str("cafes"), py_str("café"), py_str("beach"), py_str("beaches"), py_str("town"), py_str("city"), py_str("village"), py_str("park"), py_str("museum"), py_str("museums"), py_str("gallery"), py_str("market"), py_str("markets"), py_str("spot"), py_str("spots"), py_str("stay"), py_str("campsite"), py_str("campsites"), py_str("site"), py_str("sites"), py_str("walk"), py_str("walks"), py_str("swim"), py_str("swimming"), py_str("view"), py_str("coffee"), py_str("breakfast"), py_str("brunch"), py_str("chippy"), py_str("takeaway"), py_str("shop"), py_str("shops"), py_str("things to do"), py_str("thing to do"), py_str("things to see"), py_str("day out"), py_str("day trip"), py_str("beer garden")]) }
    s_PICKED_IN_A_PLACE(__data11());
    s_PICK_ASKS(py_tuple(vec![py_str("best"), py_str("top"), py_str("good"), py_str("nice"), py_str("decent"), py_str("recommend"), py_str("worth"), py_str("which")]));
    s_NOT_A_PICK(py_tuple(vec![py_str("best time"), py_str("good time"), py_str("right time"), py_str("busiest time"), py_str("cheapest time"), py_str("worth it if"), py_str("worth it when"), py_str("worth buying"), py_str("worth the money")]));
    s_IN_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:in|near|around|round|outside)\\s+(?:the\\s+|this\\s+|that\\s+)?(?:here|there|[a-z][a-z'\\-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEAR_ME(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:near|around|round|by) (?:me|us|here)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_PICK_POOL(py_tuple(vec![py_str("Reviews beat anything I could say about {thing}, which I have never been to: a map app or the town's visitor site will name the ones people rate - tell me what you want from it and I'll help you choose."), py_str("Reviews beat a guess from me about {thing}, since I have never been to any of them: a map app or the local visitor site names the ones people rate - say what matters and I'll help you narrow it down.")]));
    s_PICK_ANYWHERE_POOL(py_tuple(vec![py_str("Reviews from people who have been beat any guess of mine about a place, and a map app or the town's visitor site names the ones people rate: tell me what you want from the trip and I'll help you choose."), py_str("Other people's reviews beat anything I could say about a place, and I have never been to it: a map app or the local visitor site sorts it by what you want from the trip - say what matters and I'll help you narrow it down.")]));
    s_WORTH_VISITING(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:is|are|will)\\b[^.!?]{0,40}\\bworth (?:visiting|going to|a visit|seeing)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_FACT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:who|whos|who's)\\s+(?:is|are|was|were|owns|runs|makes|made|plays|played|wrote|writes|invented|founded|created|discovered|sang|sings|composed|directed|built|designed|painted|won)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_POOL(py_tuple(vec![py_str("The record is the only place it is written down, and a name I got half right would stick, so I can't check one: Wikipedia has it with its sources and the official site will confirm it - tell me what you find and I'll help you make sense of it."), py_str("The Wikipedia page for it answers that in a line and cites where the line comes from, and I can't look it up from here: paste what it says and I'll help you weigh the rest.")]));
    s_FIGURE_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:population|how many people (?:live|are there)|how tall|how deep|how high|how old (?:is|are)|how heavy|how big (?:is|are|was)|how large|how long is the|how wide|how many miles away|how far away|how far is|square miles|square metres does)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTING_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:should|shall|do i|can i|need to|would i|is it best|is best|ought)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_METHOD_ASKS(stdlib_call("re", "compile", &[py_str("^\\W*(?:how (?:do|would|can|might) (?:you|i|we|one)|ways? to|any (?:way|tips))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_POLICY_WORDS(py_tuple(vec![py_str("return window"), py_str("refund policy"), py_str("returns policy"), py_str("warranty"), py_str("guarantee"), py_str("terms and conditions"), py_str("the policy"), py_str("their policy"), py_str("rules")]));
    s_FIGURE_POOL(py_tuple(vec![py_str("The date on a figure matters as much as the figure, and I can't check one I wasn't told: Wikipedia has that number with its source - tell me what it says and I'll help you make sense of it."), py_str("A number I invented would be worse than none, and the year is half of it, so I would not guess: the Wikipedia page or the official site will have it, with the source; paste what it says and I'll help you weigh it up.")]));
    fn __data12() -> Py { py_tuple(vec![py_str("slab"), py_str("slabs"), py_str("tile"), py_str("tiles"), py_str("brick"), py_str("bricks"), py_str("paint"), py_str("wallpaper"), py_str("turf"), py_str("gravel"), py_str("sand"), py_str("cement"), py_str("mortar"), py_str("decking"), py_str("boards"), py_str("panel"), py_str("panels"), py_str("pots"), py_str("seeds"), py_str("bulbs"), py_str("compost"), py_str("bark"), py_str("chippings"), py_str("insulation"), py_str("plasterboard"), py_str("skirting"), py_str("beading"), py_str("underlay"), py_str("carpet"), py_str("lino"), py_str("edging"), py_str("fence posts"), py_str("screws"), py_str("hooks"), py_str("plants"), py_str("shrubs"), py_str("trees")]) }
    s_MATERIALS(__data12());
    s_MATERIAL_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how many|how much)\\b[^.!?]{0,40}?(?<![a-z])(?:do i need|will i need|should i (?:get|buy)|would i need|do we need|to cover|for a|for an|for my|needed)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_POOL(py_tuple(vec![py_str("I can do that sum if you give me the numbers: the length and width of what you're covering, and how much one slab, pot, roll or bag covers. Tell me those and I'll work out how many to buy, with a little extra for cuts."), py_str("Give me the measurements - the area's length and width, and the size of one of them - and I'll do the arithmetic, waste and cuts included. That's the part I'm good for; the size of your patio is the part I can't see.")]));
    s_BUY_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:where can i|where do i|where could i|where should i|where to|who sells|where sells)\\s+(?:buy|get|find|pick up|order)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_BUY_THING(stdlib_call("re", "compile", &[py_str("\\b(?:buy|get|find|pick up|order|sells|selling)\\s+(?:me\\s+)?((?:a|an|some|any|the|one|my|our)?\\s*[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,3})")]).unwrap_or_else(|e| exc::raise(e)));
    s_BUY_TAIL(call_builtin("frozenset", &[mcall(&py_str("\n    today tonight tomorrow now online cheap cheaply near nearby locally please then round\n    second hand secondhand from at in on shop shops store stores\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data13() -> Py { py_tuple(vec![py_str(" today"), py_str(" tonight"), py_str(" tomorrow"), py_str(" now"), py_str(" online"), py_str(" cheap"), py_str(" cheaply"), py_str(" near me"), py_str(" near you"), py_str(" nearby"), py_str(" locally"), py_str(" round here"), py_str(" in town"), py_str(" second hand"), py_str(" secondhand"), py_str(" please"), py_str(" asap"), py_str(" this week")]) }
    s_BUY_TAIL_PHRASES(__data13());
    s_BUY_THING_STOPS(stdlib_call("re", "compile", &[py_str("\\s(?:for|to|as|with|in|on|at|from|near|under|around|that|which|because|so|and|or|but)\\s")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data14() -> Py { py_tuple(vec![py_str("advice"), py_str("help"), py_str("support"), py_str("information"), py_str("info"), py_str("a loan"), py_str("a mortgage"), py_str("a job"), py_str("apprenticeship"), py_str("work"), py_str("jab"), py_str("jabs"), py_str("vaccine"), py_str("vaccination"), py_str("prescription"), py_str("treatment"), py_str("test"), py_str("checked"), py_str("tested"), py_str("treated"), py_str("seen"), py_str("blocked list"), py_str("settings"), py_str("setting"), py_str("menu"), py_str("my money back")]) }
    s_BUY_NOT(__data14());
    s_BUY_NOT_THINGS(call_builtin("frozenset", &[mcall(&py_str("\nrate rates price prices deal deals offer offers quote quotes times time hours opening\navailability stock information info news number address date dates list lists\nloan loans fixed repaired serviced mended certificate certificates replacement\npayslip payslips document documents form forms passport licence license visa\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BUY_NOT_PEOPLE(py_tuple(vec![py_str("one"), py_str("ones"), py_str("someone"), py_str("somebody"), py_str("anyone"), py_str("anybody"), py_str("person"), py_str("people")]));
    fn __data15() -> Py { py_tuple(vec![py_str("matched"), py_str("made"), py_str("done"), py_str("fixed"), py_str("printed"), py_str("framed"), py_str("engraved"), py_str("repaired"), py_str("serviced"), py_str("mended"), py_str("altered"), py_str("adjusted"), py_str("measured"), py_str("fitted"), py_str("installed"), py_str("drilled"), py_str("cleaned"), py_str("mounted"), py_str("trimmed"), py_str("sharpened"), py_str("copied"), py_str("printed off")]) }
    s_BUY_NOT_THE_THING(__data15());
    fn __data16() -> Py { py_dict(vec![(py_str("fixed"), py_str("repair")), (py_str("fix"), py_str("repair")), (py_str("mended"), py_str("repair")), (py_str("mend"), py_str("repair")), (py_str("repaired"), py_str("repair")), (py_str("repair"), py_str("repair")), (py_str("serviced"), py_str("service")), (py_str("service"), py_str("service")), (py_str("cleaned"), py_str("cleaning")), (py_str("clean"), py_str("cleaning")), (py_str("cut"), py_str("cut")), (py_str("trimmed"), py_str("trimming")), (py_str("trim"), py_str("trimming")), (py_str("altered"), py_str("alterations")), (py_str("alter"), py_str("alterations")), (py_str("adjusted"), py_str("adjustment")), (py_str("adjust"), py_str("adjustment")), (py_str("framed"), py_str("framing")), (py_str("frame"), py_str("framing")), (py_str("engraved"), py_str("engraving")), (py_str("engrave"), py_str("engraving")), (py_str("sharpened"), py_str("sharpening")), (py_str("sharpen"), py_str("sharpening")), (py_str("printed"), py_str("printing")), (py_str("print"), py_str("printing")), (py_str("fitted"), py_str("fitting")), (py_str("fit"), py_str("fitting")), (py_str("installed"), py_str("installation")), (py_str("install"), py_str("installation")), (py_str("drilled"), py_str("drilling")), (py_str("drill"), py_str("drilling")), (py_str("mounted"), py_str("mounting")), (py_str("mount"), py_str("mounting")), (py_str("copied"), py_str("copying")), (py_str("copy"), py_str("copying")), (py_str("restored"), py_str("restoration")), (py_str("restore"), py_str("restoration")), (py_str("valeted"), py_str("valet")), (py_str("valet"), py_str("valet")), (py_str("resoled"), py_str("resoling")), (py_str("resole"), py_str("resoling")), (py_str("replaced"), py_str("replacement")), (py_str("replace"), py_str("replacement")), (py_str("resprayed"), py_str("respray")), (py_str("respray"), py_str("respray")), (py_str("washed"), py_str("washing")), (py_str("wash"), py_str("washing"))]) }
    s_SERVICE_WORK(__data16());
    s_SERVICE_PHRASES(py_tuple(vec![py_tuple(vec![py_str("taken in"), py_str("alterations")]), py_tuple(vec![py_str("take in"), py_str("alterations")]), py_tuple(vec![py_str("taking in"), py_str("alterations")]), py_tuple(vec![py_str("let out"), py_str("alterations")]), py_tuple(vec![py_str("taken up"), py_str("alterations")]), py_tuple(vec![py_str("taken in a bit"), py_str("alterations")])]));
    s_SERVICE_NOUNS(py_tuple(vec![py_tuple(vec![py_str("haircut"), py_tuple(vec![py_str("hair cut"), py_str("")])]), py_tuple(vec![py_str("hair cut"), py_tuple(vec![py_str("hair cut"), py_str("")])]), py_tuple(vec![py_str("dry cleaning"), py_tuple(vec![py_str("dry cleaning"), py_str("")])]), py_tuple(vec![py_str("mot"), py_tuple(vec![py_str("mot"), py_str("car")])])]));
    s_SERVICE_JOB_NAMES(py_dict(vec![(py_tuple(vec![py_str("hair"), py_str("cut")]), py_str("haircuts")), (py_tuple(vec![py_str(""), py_str("hair cut")]), py_str("haircuts")), (py_tuple(vec![py_str("key"), py_str("cut")]), py_str("key cutting")), (py_tuple(vec![py_str("car"), py_str("mot")]), py_str("an MOT")), (py_tuple(vec![py_str("car"), py_str("washing")]), py_str("a car wash")), (py_tuple(vec![py_str("shoes"), py_str("resoling")]), py_str("shoe repairs")), (py_tuple(vec![py_str("shoe"), py_str("resoling")]), py_str("shoe repairs"))]));
    s_SERVICE_POOL(py_tuple(vec![py_str("The last few reviews and two prices are what decide a {job}, not a list I could keep: search a map app or a directory near you, read the recent ones, and ring two for a price before you book."), py_str("Prices for the same {job} swing wide, so the reviews and two quotes decide it: a map app or a directory near you, sorted by the recent ones, then two prices before you book.")]));
    s_SERVICE_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:where can i|where do i|where could i|where should i|where to|who can|who could|who should i|who does|who would|anywhere i can)\\s+(?:get|have|take|find|send|put)\\b|(?<![a-z])(?:who can|who could|who does|who would)\\b|(?<![a-z])(?:can|could|should|would|will)\\s+i\\s+(?:get|have)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:get|have|take|send|put|fix|repair|mend|service|clean|cut|alter|resole|reheel|replace)\\s+(?:me\\s+)?((?:a|an|the|my|our|some)?\\s*[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,2})")]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nit that this them one ones me him her us you yourself myself anything something nothing\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE_NOT_HEALTH(call_builtin("frozenset", &[mcall(&py_str("\nteeth tooth dental blood bloods jab injection vaccine vaccination wound stitches cast mole skin\neye eyes ear ears smear scan xray procedure\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE_STOPS(call_builtin("frozenset", &[mcall(&py_str("\ncheap cheaply properly professionally done somewhere someone today tonight tomorrow please again\nback off out about that which because so and or for taken take taking let\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BUY_POOL(py_tuple(vec![py_str("Stock moves by the hour and a phone call is the only thing that checks it, so I can't see who has {thing} near you: a map app search shows what's nearest, and ring before you set off."), py_str("Stock moves by the hour, so a list from me would be wrong by the time you got there: a map app or the chain's own site will say who has {thing} near you, and a ring first saves the trip.")]));
    s_OPEN_CHEMIST_HOURS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:open|opens|opening|closed|closing|closes|shut|shuts|late|late-night|latenight|tonight|now|today|sunday|weekend|bank holiday|out of hours|24 ?hour|all night)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHEMISTS(py_tuple(vec![py_str("pharmacy"), py_str("pharmacies"), py_str("chemist"), py_str("chemists"), py_str("dispensary")]));
    s_OPEN_CHEMIST_POOL(py_tuple(vec![py_str("Most towns have one open late: a big chain chemist usually runs to 10 or 11, a supermarket one shuts by 8 and earlier on a Sunday, and a bank holiday moves it again. Which of the ones near you is open tonight I can't see - the NHS pharmacy finder lists them with the closing time and the distance, and 111 says the same on the phone."), py_str("There is nearly always a late chemist somewhere in a town - the chains stay open to about 10, and the ones inside supermarkets shut by 8, earlier on a Sunday - but the hours near you are not something I can look up. The NHS pharmacy finder shows what is open now and until when; 111 will find you one if the phone is easier, and an out-of-hours prescription can go there.")]));
    s_RECORD_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:biggest|largest|tallest|longest|oldest|youngest|smallest|deepest|shallowest|highest|lowest|heaviest|lightest|fastest|slowest|widest|narrowest|thinnest|strongest|richest|poorest|wealthiest|busiest|coldest|hottest|wettest|driest|sunniest)(?![a-z])|\\bmost\\s+(?:populous|expensive|common|popular|visited|deadly|abundant|spoken)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECORD_ASKED(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what|whats|what's|which|whichs|which's|who|whos|who's|where|wheres|where's|name|tell me)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECORD_POOL(py_tuple(vec![py_str("I can't check a record from here, sorry - the list moves when somebody breaks it. Wikipedia keeps it with the date each one was set; paste what it says and I'll help you weigh it up."), py_str("No record book here, sorry, and a half-remembered number would be wrong without saying so. The Wikipedia page for it keeps the list and dates it - tell me what it says and we can go from there.")]));
    s_TRAVEL_TOPICS(py_tuple(vec![py_str("geography-and-travel"), py_str("travel"), py_str("holidays"), py_str("days-out"), py_str("outdoors")]));
    s_TIDE_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what|which|whats|what's)\\s+time\\s+(?:is|are|does|do|will|would)[^.!?]{0,24}?(?<![a-z])(?:high|low|the)\\s*tides?(?![a-z])|(?<![a-z])when(?:'s|s|'re|re| is| are| does| do| will| can| should)?\\s+(?:the\\s+)?(?:high|low)\\s+tide(?![a-z])|(?<![a-z])when\\s+(?:is|does|will)\\s+the\\s+tide(?![a-z])|(?<![a-z])is\\s+the\\s+tide\\s+(?:in|out|coming|going|high|low|up|down)(?![a-z])|(?<![a-z])tide\\s+(?:is\\s+)?(?:in|out)\\s+or\\s+(?:in|out)(?![a-z])|(?<![a-z])tide\\s+(?:times|table|timetable)(?![a-z])|(?<![a-z])what(?:'s|s|\\s+is)\\s+the\\s+tide\\s+(?:doing|like)(?![a-z])|(?<![a-z])how\\s+(?:high|low|far\\s+out)\\s+(?:is|will|does)\\s+the\\s+tide(?![a-z])|(?<![a-z])when\\s+(?:can|should)\\s+(?:i|we)\\s+(?:swim|walk|launch|get)\\s+(?:out|across|on\\s+the\\s+water|to\\s+the\\s+boat)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIDE_WHY(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:why|how\\s+(?:do|does|come)|what\\s+causes?|explain|explained|what\\s+are)\\b[^.!?]{0,20}(?<![a-z])tides?(?![a-z])|(?<![a-z])tides?\\b[^.!?]{0,24}(?:work|works|happen|caused|spring|neap|moon)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIDE_POOL(py_tuple(vec![py_str("High and low water move by up to an hour a day, and by minutes between two beaches a few miles apart, so a tide time is the one thing I would not guess: a tide-table site for that beach, or the harbour or pier's own page, has today and tomorrow, and the local paper prints them too."), py_str("Two beaches a few miles apart turn minutes apart and the same one is an hour later tomorrow, so I can't hold today's times: the harbour's page or a tide table for that beach has them, with the height as well as the clock time - the height is the bit that matters if you are walking a causeway or launching a boat.")]));
    s_FILLING_IT_IN(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:form|forms|application|answers|questionnaire|survey|booking|checkout|basket|page|webpage|website|site|screen|portal|online|typed|filled\\s+it\\s+in|filled\\s+the\\s+form\\s+in)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAVING_FAILED(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:didnt|didn't|did\\s+not|doesnt|doesn't|does\\s+not|wouldnt|wouldn't|would\\s+not|wont|won't|will\\s+not|refused\\s+to)\\s+(?:save|keep|remember)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_THE_WORK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:didnt|didn't|did\\s+not|doesnt|doesn't|does\\s+not|wouldnt|wouldn't|would\\s+not|wont|won't|will\\s+not|refused\\s+to)\\s+(?:save|keep|remember)(?![a-z])|(?<![a-z])(?:went|took\\s+me|jumped)\\s+back\\s+to\\s+the\\s+start(?![a-z])|(?<![a-z])(?:start|do)\\b[^.!?]{0,22}?(?<![a-z])(?:again|from\\s+scratch)(?![a-z])|(?<![a-z])(?:all\\s+over\\s+again)(?![a-z])|(?<![a-z])(?:lost|wiped|cleared|erased|emptied)\\s+(?:my\\s+|the\\s+|all\\s+|the\\s+whole\\s+)?(?:answers|work|everything|it\\s+all|the\\s+lot|basket|cart|form|application)(?![a-z])|(?<![a-z])(?:disappeared|vanished|never\\s+went\\s+through)(?![a-z])|(?<![a-z])(?:timed\\s+out|crashed)(?![a-z])|(?<![a-z])(?:signed\\s+me\\s+out|logged\\s+me\\s+out|kicked\\s+me\\s+out|threw\\s+me\\s+out)(?![a-z])|(?<![a-z])(?:cant|can't|couldnt|couldn't|wont|won't)\\s+(?:let\\s+me\\s+|allow\\s+me\\s+)?(?:submit|send\\s+it)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_A_SCREEN(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:online|website|web\\s?site|site|webpage|web\\s?page|page|screen|portal|checkout|basket|cart|typed|typing|type|submitted|submit|link|app|browser|internet|form\\s+online|filling\\s+in|click\\w*|tap\\w*|refresh\\w*|reload\\w*|scroll\\w*|timed\\s+out|session|the\\s+back\\s+button|save\\s+button)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WORK_ITSELF(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:whole|entire|all\\s+of\\s+(?:it|my|the)|every\\s+(?:answer|page|line|word)|all\\s+(?:of\\s+)?(?:my|the)\\s+(?:answers|work))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_BACK_TO_START(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:went|took\\s+me|jumped|sent\\s+me)\\s+back\\s+to\\s+the\\s+start(?![a-z])|(?<![a-z])(?:start|do)\\b[^.!?]{0,22}?(?<![a-z])(?:again|from\\s+scratch)(?![a-z])|(?<![a-z])(?:all\\s+over\\s+again)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BARE_FORMS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:form|forms|application|applications|answers|questionnaire|survey)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data17() -> Py { py_tuple(vec![py_str("in the post"), py_str("by post"), py_str("posted"), py_str("paper"), py_str("printed"), py_str("print it"), py_str("in the letter"), py_str("through the door"), py_str("on paper"), py_str("claim"), py_str("compensation"), py_str("lost my bag"), py_str("lost bag")]) }
    s_NOT_A_WRECKED_FORM(__data17());
    s_WRECKED_POOL(py_tuple(vec![py_str("I can't see their system, but this order gets a lost form back more often than not: the back button before a reload, then a saved draft or a 'continue where you left off' link, then the service itself - ask them to check the half-finished application before you type it all again, because a duplicate is harder to undo than a gap."), py_str("Their system isn't visible from here, so the order is what matters: back one page rather than reloading, then the drafts or 'continue your application' link, then the service itself, quoting any reference you were given. When you do type it again, keep the long answers in a document and paste them in, so a second timeout costs you nothing.")]));
    s_READERS(py_tuple(vec![py_func("_trade_route", |args: &[Py]| _trade_route(args)), py_func("_service_route", |args: &[Py]| _service_route(args)), py_func("_buy_route", |args: &[Py]| _buy_route(args)), py_func("_pick_route", |args: &[Py]| _pick_route(args)), py_func("_how_many_route", |args: &[Py]| _how_many_route(args)), py_func("_place_route", |args: &[Py]| _place_route(args)), py_func("_route_route", |args: &[Py]| _route_route(args)), py_func("_who_route", |args: &[Py]| _who_route(args)), py_func("_tide_route", |args: &[Py]| _tide_route(args)), py_func("_figure_route", |args: &[Py]| _figure_route(args)), py_func("_record_route", |args: &[Py]| _record_route(args)), py_func("_open_chemist_route", |args: &[Py]| _open_chemist_route(args))]));
}
pub fn _has(args: &[Py]) -> Py {
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
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("True when one of the phrases is in the line as a whole word or phrase.\n\n    Whole words, because \"stop\" holds \"top\" and \"goodbye\" holds \"good\": a request for help with a\n    cramp is not a request for the best of anything.\n    ");
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    return call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names one of the things, as a whole word or a whole phrase.\n\n    Splitting every phrase into single words is how \"things to do\" matched the \"thing\" and the\n    \"to\" of \"how do i make a good impression\", and it is why this takes each phrase as it stands.\n    ");
    words = _words(&[line.clone()]);
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    parts = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &geti(&parts.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &mcall(&py_str("\\s+"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str("(?![a-z0-9])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_trade(args: &[Py]) -> Py {
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
    let mut trade: Py = Py::None;
    let _ = py_str("The trade named in the line, or \"\" when it names none.");
    let __iter1 = py_iter_or_raise(&g_TRADES());
    for __item in __iter1 {
    trade = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[trade.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return trade.clone();
    }
    }
    return py_str("");
    Py::None
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing being bought, with its article, or \"\" when the line does not yield one.");
    found = mcall(&g_BUY_THING(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    text = op_add(&py_str(" "), &mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]));
    let __iter1 = py_iter_or_raise(&g_BUY_TAIL_PHRASES());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&mcall(&text.clone(), "endswith", &[phrase.clone()])) {
    text = slice_or_raise(&text.clone(), &Py::None, &op_neg(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    cut = mcall(&g_BUY_THING_STOPS(), "search", &[text.clone()]);
    if py_truthy(&cut.clone()) {
    text = slice_or_raise(&text.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_BUY_TAIL(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("any"), py_str("my"), py_str("our"), py_str("of"), py_str("for")]), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(4), &Py::None)]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("any"), py_str("it"), py_str("one"), py_str("them"), py_str("me")]), &thing.clone())) } }) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _trade_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut pool: Py = Py::None;
    let mut trade: Py = Py::None;
    trade = _the_trade(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&trade.clone())); if py_truthy(&__b) { __b } else { { let __b = _has(&[line.clone(), g_NOT_A_TRADE()]); if py_truthy(&__b) { __b } else { _has(&[line.clone(), g_NOT_A_HIRE()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_OWN_PROFESSIONAL(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[line.clone(), g_HIRING()])))) {
    return Py::None;
    }
    pool = if py_truthy(&Py::Bool(py_contains(&g_QUOTING_TRADES(), &trade.clone()))) { g_TRADE_POOL() } else { g_PROFESSIONAL_POOL() };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("trade".to_string(), trade.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _buy_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_BUY_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { _has(&[line.clone(), g_BUY_NOT()]) } }) {
    return Py::None;
    }
    thing = _the_thing(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BUY_NOT_THINGS(), &geti(&mcall(&thing.clone(), "split", &[]), &op_neg(&Py::Int(1))))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &mcall(&geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)), "strip", &[])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has(&[op_add(&op_add(&py_str(" "), &thing.clone()), &py_str(" ")), g_BUY_NOT_THE_THING()]); if py_truthy(&__b) { __b } else { _has(&[op_add(&op_add(&py_str(" "), &thing.clone()), &py_str(" ")), g_BUY_NOT_PEOPLE()]) } }) {
    return Py::None;
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BUY_POOL()) { let mut pool: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pool.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_service(args: &[Py]) -> Py {
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
    let mut pointed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pointed") { v.clone() }
    else { py_str("") };
    let mut found: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut work: Py = Py::None;
    let _ = py_str("(the work, the thing) for a line that puts a thing in for work, or None.\n\n    `pointed` is what a pronoun in the line points at, which the understanding engine has already\n    worked out: after \"the jeans dont fit\", \"can i get them taken in\" is the jeans, and a reader\n    that only looked for a noun in the line itself would miss it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SERVICE_ASK(), "search", &[line.clone()])))) {
    return Py::None;
    }
    work = py_str("");
    let __iter1 = py_iter_or_raise(&g_SERVICE_PHRASES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    work = word.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&work.clone()))) {
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mcall(&g_SERVICE_WORK(), "items", &[])); __kws.extend(vec![("key".to_string(), { let found = found.clone(); let line = line.clone(); let noun = noun.clone(); let phrase = phrase.clone(); let pointed = pointed.clone(); let thing = thing.clone(); let verb = verb.clone(); let word = word.clone(); let words = words.clone(); let work = work.clone(); py_func("lambda", move |args: &[Py]| { let found = found.clone(); let line = line.clone(); let noun = noun.clone(); let phrase = phrase.clone(); let pointed = pointed.clone(); let thing = thing.clone(); let verb = verb.clone(); let word = word.clone(); let words = words.clone(); let work = work.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); op_neg(&call_builtin("len", &[geti(&pair.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    verb = __unpk4[0].clone();
    word = __unpk4[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    work = word.clone();
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&work.clone()))) {
    let __iter5 = py_iter_or_raise(&g_SERVICE_NOUNS());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    noun = __unpk6[0].clone();
    let __unpk7 = py_unpack(&__unpk6[1].clone(), 2, None);
    word = __unpk7[0].clone();
    thing = __unpk7[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[noun.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![word.clone(), thing.clone()]);
    }
    }
    return Py::None;
    }
    found = mcall(&g_SERVICE_THING(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    words = py_list(vec![]);
    let __iter8 = py_iter_or_raise(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "split", &[]));
    for __item in __iter8 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("some"), py_str("of"), py_str("for"), py_str("to"), py_str("in"), py_str("on")]), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BUY_TAIL(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SERVICE_WORK(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SERVICE_STOPS(), &word.clone())) } } } }) {
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SERVICE_NOT_A_THING(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = pointed.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&stdlib_call("re", "search", &[py_str("[a-z]"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SERVICE_NOT_A_THING(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SERVICE_NOT_HEALTH(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return py_tuple(vec![work.clone(), mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None)])]);
    Py::None
}
pub fn _service_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut found: Py = Py::None;
    let mut job: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut work: Py = Py::None;
    found = _the_service(&[line.clone(), if py_truthy(&call_builtin("isinstance", &[ctx.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&ctx.clone(), "get", &[py_str("pointed")]) } else { py_str("") }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    work = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    job = { let __b = mcall(&g_SERVICE_JOB_NAMES(), "get", &[py_tuple(vec![thing.clone(), work.clone()])]); if py_truthy(&__b) { __b } else { mcall(&op_add(&op_add(&thing.clone(), &py_str(" ")), &work.clone()), "strip", &[]) } };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SERVICE_POOL()) { let mut pool: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("job".to_string(), job.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pool.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_a_place(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let _ = py_str("True when the line is about somewhere out in the world, not \"in food\" or \"in a new job\".\n\n    A named place is the surest sign, and the understanding engine already says when a line\n    carries one; a travel line without a name (\"the best beach near me\") is the other way in, and\n    \"near me\" is the one phrase that makes a line local on its own.\n    ");
    if py_truthy(&mcall(&ctx.clone(), "get", &[py_str("places")])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&ctx.clone(), "get", &[py_str("travel")])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_WORTH_VISITING(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_NEAR_ME(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_picked_thing(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("The phrase the person picked out of the place: \"beaches\" from \"the best beach in cornwall\".\n\n    PICK_POOL names it back to them (\"Reviews beat anything I could say about beaches\"), so the\n    pool has a {thing} slot and every route that returns it has to fill that slot - a line said\n    with \"{thing}\" still in it is worse than the shrug it replaced.\n    ");
    found = py_str("");
    let __iter1 = py_iter_or_raise(&g_PICKED_IN_A_PLACE());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&op_le(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    continue;
    }
    if py_truthy(&_names_one_of(&[line.clone(), py_tuple(vec![phrase.clone()])])) {
    found = phrase.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&found.clone(), &py_str("thing to do"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&found.clone(), &py_str("things to do"))) } }) {
    return py_str("things to do");
    }
    return found.clone();
    Py::None
}
pub fn _pick_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("Something to do, eat or stay in somewhere: the picks are the reviews' job, not the bot's.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_has(&[line.clone(), g_PICK_ASKS()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WORTH_VISITING(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _has(&[line.clone(), g_NOT_A_PICK()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_a_place(&[line.clone(), ctx.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_WORTH_VISITING(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_IN_A_PLACE(), "search", &[line.clone()]) } }) {
    thing = _the_picked_thing(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return g_PICK_ANYWHERE_POOL();
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PICK_POOL()) { let mut pool: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pool.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    where_ = mcall(&g_IN_A_PLACE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&where_.clone()))) {
    return Py::None;
    }
    thing = _the_picked_thing(&[slice_or_raise(&line.clone(), &Py::None, &mcall(&where_.clone(), "start", &[]), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PICK_POOL()) { let mut pool: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pool.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_many_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_MATERIAL_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _names_one_of(&[line.clone(), g_MATERIALS()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])how (?:many|much)(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return g_HOW_MANY_POOL();
    Py::None
}
pub fn _place_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    if py_truthy(&{ let __b = mcall(&g_NOT_A_PLACE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_LOOKUP(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_has(&[line.clone(), g_CONTACT_ASKS()])) {
    return g_PLACE_POOL();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[line.clone(), g_WHERE_ASKS()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_one_of(&[line.clone(), g_PLACE_NOUNS()])))) {
    return Py::None;
    }
    return g_PLACE_POOL();
    Py::None
}
pub fn _route_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut travel: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How to get somewhere, and how far it is: the journey is the map app's to measure.");
    if py_truthy(&_has(&[line.clone(), g_NOT_A_JOURNEY()])) {
    return Py::None;
    }
    words = _words(&[line.clone()]);
    travel = call_builtin("bool", &[op_bitand(&words.clone(), &call_builtin("set", &[g_TRAVEL_WORDS()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_ROUTE_ASKS(), "search", &[line.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = travel.clone(); if py_truthy(&__b) { __b } else { mcall(&ctx.clone(), "get", &[py_str("places")]) } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_TAKE_TO_ASKS(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&ctx.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { _names_one_of(&[line.clone(), g_GOES_TO()]) } })) } }) {
    return Py::None;
    }
    return g_PLACE_ROUTE_POOL();
    }
    if py_truthy(&{ let __b = _has(&[line.clone(), g_GET_TO_ASKS()]); if !py_truthy(&__b) { __b } else { { let __b = travel.clone(); if py_truthy(&__b) { __b } else { mcall(&ctx.clone(), "get", &[py_str("places")]) } } } }) {
    return g_PLACE_ROUTE_POOL();
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("(?<![a-z])from\\b[^.!?]{1,40}\\bto(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = travel.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("how (?:do|can|should|would) (?:i|you|we)"), line.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return g_PLACE_ROUTE_POOL();
    }
    return Py::None;
    Py::None
}
pub fn _open_chemist_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let _ = py_str("A chemist that has to be open now, or near, or on a day the usual one shuts.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_has(&[line.clone(), g_CHEMISTS()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OPEN_CHEMIST_HOURS(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("(?<![a-z])(?:what|anything|something|buy|get|recommend)(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])(?:open|closes|close|shut|late|tonight)(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return g_OPEN_CHEMIST_POOL();
    Py::None
}
pub fn _who_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let mut words: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_WHO_FACT(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { _has(&[line.clone(), g_ABOUT_THE_BOT()]) } }) {
    return Py::None;
    }
    words = _words(&[line.clone()]);
    if py_truthy(&op_bitand(&words.clone(), &g_OWN_PERSON())) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])(?:who|whos|who's)\\s+(?:is|was|were|are)\\s+(?:it|that|this|he|she|they|them|you|thou)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return g_WHO_POOL();
    Py::None
}
pub fn _tide_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let _ = py_str("When the sea turns, which is a table for one beach and one day.\n\n    High and low water move by up to an hour a day and by minutes between two beaches a few miles\n    apart, so nothing here can hold the clock time, and a guess would send somebody onto a\n    causeway or off a slipway at the wrong moment. The route names the table that has it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_TIDE_ASK(), "search", &[line.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_TIDE_WHY(), "search", &[line.clone()])) {
    return Py::None;
    }
    return g_TIDE_POOL();
    Py::None
}
pub fn _figure_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_FIGURE_ASKS(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_SETTING_ASKS(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_METHOD_ASKS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { _has(&[op_add(&op_add(&py_str(" "), &line.clone()), &py_str(" ")), g_POLICY_WORDS()]) } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&_words(&[line.clone()]), &g_OWN_PERSON())) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])how (?:old|tall|big|heavy|deep|far) (?:is|are) (?:he|she|they|it|you|we)(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return g_FIGURE_POOL();
    Py::None
}
pub fn _record_route(args: &[Py]) -> Py {
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
    let mut ctx: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ctx") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ctx"))) };
    let _ = py_str("A record the bot does not hold: the page that keeps the list, not today's news.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&ctx.clone(), "get", &[py_str("settled")])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_RECORD_ASKS(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_RECORD_ASKED(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&_words(&[line.clone()]), &g_OWN_PERSON()); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SETTING_ASKS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_METHOD_ASKS(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])(?:the best|my best|your best|best way|worst way|best time)(?![a-z])"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&_has(&[op_add(&op_add(&py_str(" "), &line.clone()), &py_str(" ")), py_tuple(vec![py_str("size"), py_str("sizes"), py_str("pack"), py_str("packs"), py_str("box"), py_str("boxes"), py_str("bottle"), py_str("bottles"), py_str("tin"), py_str("tins"), py_str("bag"), py_str("bags")])])) {
    return Py::None;
    }
    if py_truthy(&_has(&[op_add(&op_add(&py_str(" "), &line.clone()), &py_str(" ")), py_tuple(vec![py_str("number"), py_str("numbers"), py_str("word"), py_str("words"), py_str("letter"), py_str("letters"), py_str("digit"), py_str("digits"), py_str("prime"), py_str("sum"), py_str("total")])])) {
    return Py::None;
    }
    return g_RECORD_POOL();
    Py::None
}
pub fn _the_reading(args: &[Py]) -> Py {
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
    let mut entity: Py = Py::None;
    let mut key: Py = Py::None;
    let mut places: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut travel: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The parts of the reading the readers use, from what the understanding engine says.");
    if py_truthy(&Py::Bool(!py_truthy(&understood.clone()))) {
    return py_dict(vec![(py_str("places"), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))), (py_str("travel"), Py::Bool(false)), (py_str("pointed"), py_str("")), (py_str("settled"), Py::Bool(false))]);
    }
    places = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entity = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("place"))); if !py_truthy(&__b) { __b } else { mcall(&entity.clone(), "get", &[py_str("text")]) } } } }) {
    let _ = mcall(&places.clone(), "add", &[geti(&entity.clone(), &py_str("text"))]);
    }
    }
    pointed = py_str("");
    ref_ = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("it"), py_str("that"), py_str("they"), py_str("them")]));
    for __item in __iter2 {
    key = __item.clone();
    value = mcall(&ref_.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    pointed = mcall(&value.clone(), "strip", &[]);
    break;
    }
    }
    }
    topics = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&understood.clone(), "get", &[py_str("topic")])) {
    let _ = mcall(&topics.clone(), "add", &[geti(&understood.clone(), &py_str("topic"))]);
    }
    travel = call_builtin("bool", &[op_bitand(&topics.clone(), &call_builtin("set", &[g_TRAVEL_TOPICS()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    settled = Py::Bool(py_is(&mcall(&understood.clone(), "get", &[py_str("over_time")]), &Py::Bool(false)));
    return py_dict(vec![(py_str("places"), places.clone()), (py_str("travel"), travel.clone()), (py_str("pointed"), pointed.clone()), (py_str("settled"), settled.clone())]);
    Py::None
}
pub fn the_wrecked_form_reply(args: &[Py]) -> Py {
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
    let _ = py_str("The pool for a form or a page that threw the person's work away, or None.\n\n    Public because a line like this is a *statement* - \"my online application didnt save\" has no\n    question in it - so it never reaches `for_the_ask`: the chain has already offered the lost-ask\n    and sympathy pools by then. The composer asks this one question first, and it answers for\n    every wording of a form that ate the work, not just this one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { _has(&[line.clone(), g_NOT_A_WRECKED_FORM()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_FILLING_IT_IN(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_LOST_THE_WORK(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_THE_BARE_FORMS(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_ON_A_SCREEN(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SAVING_FAILED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BACK_TO_START(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_WORK_ITSELF(), "search", &[line.clone()]) } } } } } })) } }) {
    return Py::None;
    }
    return g_WRECKED_POOL();
    Py::None
}
pub fn for_the_ask(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { py_str("") };
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut ctx: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reader: Py = Py::None;
    let _ = py_str("The pool of honest routes for this ask, or None when none of them beats the hedge.\n\n    `clean` is the line as typed and `repaired` the same line with its chat-speak put right, the\n    convention every reader in the composer follows: the typed line is read first, and the\n    repaired one only catches what a slip hid.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    ctx = _the_reading(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), { let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&g_READERS());
    for __item in __iter2 {
    reader = __item.clone();
    found = py_call(&reader, &[line.clone(), ctx.clone()]);
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.finding_out".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT" => Some(g_ABOUT_THE_BOT()),
            "BACK_TO_START" => Some(g_BACK_TO_START()),
            "BUY_ASK" => Some(g_BUY_ASK()),
            "BUY_NOT" => Some(g_BUY_NOT()),
            "BUY_NOT_PEOPLE" => Some(g_BUY_NOT_PEOPLE()),
            "BUY_NOT_THE_THING" => Some(g_BUY_NOT_THE_THING()),
            "BUY_NOT_THINGS" => Some(g_BUY_NOT_THINGS()),
            "BUY_POOL" => Some(g_BUY_POOL()),
            "BUY_TAIL" => Some(g_BUY_TAIL()),
            "BUY_TAIL_PHRASES" => Some(g_BUY_TAIL_PHRASES()),
            "BUY_THING" => Some(g_BUY_THING()),
            "BUY_THING_STOPS" => Some(g_BUY_THING_STOPS()),
            "CHEMISTS" => Some(g_CHEMISTS()),
            "CONTACT_ASKS" => Some(g_CONTACT_ASKS()),
            "FIGURE_ASKS" => Some(g_FIGURE_ASKS()),
            "FIGURE_POOL" => Some(g_FIGURE_POOL()),
            "FILLING_IT_IN" => Some(g_FILLING_IT_IN()),
            "GET_TO_ASKS" => Some(g_GET_TO_ASKS()),
            "GOES_TO" => Some(g_GOES_TO()),
            "HIRING" => Some(g_HIRING()),
            "HOW_MANY_POOL" => Some(g_HOW_MANY_POOL()),
            "IN_A_PLACE" => Some(g_IN_A_PLACE()),
            "LOST_THE_WORK" => Some(g_LOST_THE_WORK()),
            "MATERIALS" => Some(g_MATERIALS()),
            "MATERIAL_ASK" => Some(g_MATERIAL_ASK()),
            "METHOD_ASKS" => Some(g_METHOD_ASKS()),
            "NEAR_ME" => Some(g_NEAR_ME()),
            "NOBODY_NAMED" => Some(g_NOBODY_NAMED()),
            "NOT_A_HIRE" => Some(g_NOT_A_HIRE()),
            "NOT_A_JOURNEY" => Some(g_NOT_A_JOURNEY()),
            "NOT_A_LOOKUP" => Some(g_NOT_A_LOOKUP()),
            "NOT_A_PICK" => Some(g_NOT_A_PICK()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NOT_A_TRADE" => Some(g_NOT_A_TRADE()),
            "NOT_A_WRECKED_FORM" => Some(g_NOT_A_WRECKED_FORM()),
            "ON_A_SCREEN" => Some(g_ON_A_SCREEN()),
            "OPEN_CHEMIST_HOURS" => Some(g_OPEN_CHEMIST_HOURS()),
            "OPEN_CHEMIST_POOL" => Some(g_OPEN_CHEMIST_POOL()),
            "OWN_PERSON" => Some(g_OWN_PERSON()),
            "OWN_PROFESSIONAL" => Some(g_OWN_PROFESSIONAL()),
            "PICKED_IN_A_PLACE" => Some(g_PICKED_IN_A_PLACE()),
            "PICK_ANYWHERE_POOL" => Some(g_PICK_ANYWHERE_POOL()),
            "PICK_ASKS" => Some(g_PICK_ASKS()),
            "PICK_POOL" => Some(g_PICK_POOL()),
            "PLACE_NOUNS" => Some(g_PLACE_NOUNS()),
            "PLACE_POOL" => Some(g_PLACE_POOL()),
            "PLACE_ROUTE_POOL" => Some(g_PLACE_ROUTE_POOL()),
            "POLICY_WORDS" => Some(g_POLICY_WORDS()),
            "PROFESSIONAL_POOL" => Some(g_PROFESSIONAL_POOL()),
            "QUOTING_TRADES" => Some(g_QUOTING_TRADES()),
            "READERS" => Some(g_READERS()),
            "RECORD_ASKED" => Some(g_RECORD_ASKED()),
            "RECORD_ASKS" => Some(g_RECORD_ASKS()),
            "RECORD_POOL" => Some(g_RECORD_POOL()),
            "ROUTE_ASKS" => Some(g_ROUTE_ASKS()),
            "SAVING_FAILED" => Some(g_SAVING_FAILED()),
            "SERVICE_ASK" => Some(g_SERVICE_ASK()),
            "SERVICE_JOB_NAMES" => Some(g_SERVICE_JOB_NAMES()),
            "SERVICE_NOT_A_THING" => Some(g_SERVICE_NOT_A_THING()),
            "SERVICE_NOT_HEALTH" => Some(g_SERVICE_NOT_HEALTH()),
            "SERVICE_NOUNS" => Some(g_SERVICE_NOUNS()),
            "SERVICE_PHRASES" => Some(g_SERVICE_PHRASES()),
            "SERVICE_POOL" => Some(g_SERVICE_POOL()),
            "SERVICE_STOPS" => Some(g_SERVICE_STOPS()),
            "SERVICE_THING" => Some(g_SERVICE_THING()),
            "SERVICE_WORK" => Some(g_SERVICE_WORK()),
            "SETTING_ASKS" => Some(g_SETTING_ASKS()),
            "TAKE_TO_ASKS" => Some(g_TAKE_TO_ASKS()),
            "THE_BARE_FORMS" => Some(g_THE_BARE_FORMS()),
            "THE_WORK_ITSELF" => Some(g_THE_WORK_ITSELF()),
            "TIDE_ASK" => Some(g_TIDE_ASK()),
            "TIDE_POOL" => Some(g_TIDE_POOL()),
            "TIDE_WHY" => Some(g_TIDE_WHY()),
            "TRADES" => Some(g_TRADES()),
            "TRADE_POOL" => Some(g_TRADE_POOL()),
            "TRAVEL_TOPICS" => Some(g_TRAVEL_TOPICS()),
            "TRAVEL_WORDS" => Some(g_TRAVEL_WORDS()),
            "WHERE_ASKS" => Some(g_WHERE_ASKS()),
            "WHO_FACT" => Some(g_WHO_FACT()),
            "WHO_POOL" => Some(g_WHO_POOL()),
            "WORTH_VISITING" => Some(g_WORTH_VISITING()),
            "WRECKED_POOL" => Some(g_WRECKED_POOL()),
            "_about_a_place" => Some(py_func("_about_a_place", |args: &[Py]| _about_a_place(args))),
            "_buy_route" => Some(py_func("_buy_route", |args: &[Py]| _buy_route(args))),
            "_figure_route" => Some(py_func("_figure_route", |args: &[Py]| _figure_route(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_how_many_route" => Some(py_func("_how_many_route", |args: &[Py]| _how_many_route(args))),
            "_names_one_of" => Some(py_func("_names_one_of", |args: &[Py]| _names_one_of(args))),
            "_open_chemist_route" => Some(py_func("_open_chemist_route", |args: &[Py]| _open_chemist_route(args))),
            "_pick_route" => Some(py_func("_pick_route", |args: &[Py]| _pick_route(args))),
            "_place_route" => Some(py_func("_place_route", |args: &[Py]| _place_route(args))),
            "_record_route" => Some(py_func("_record_route", |args: &[Py]| _record_route(args))),
            "_route_route" => Some(py_func("_route_route", |args: &[Py]| _route_route(args))),
            "_service_route" => Some(py_func("_service_route", |args: &[Py]| _service_route(args))),
            "_the_picked_thing" => Some(py_func("_the_picked_thing", |args: &[Py]| _the_picked_thing(args))),
            "_the_reading" => Some(py_func("_the_reading", |args: &[Py]| _the_reading(args))),
            "_the_service" => Some(py_func("_the_service", |args: &[Py]| _the_service(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_trade" => Some(py_func("_the_trade", |args: &[Py]| _the_trade(args))),
            "_tide_route" => Some(py_func("_tide_route", |args: &[Py]| _tide_route(args))),
            "_trade_route" => Some(py_func("_trade_route", |args: &[Py]| _trade_route(args))),
            "_who_route" => Some(py_func("_who_route", |args: &[Py]| _who_route(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "for_the_ask" => Some(py_func("for_the_ask", |args: &[Py]| for_the_ask(args))),
            "the_wrecked_form_reply" => Some(py_func("the_wrecked_form_reply", |args: &[Py]| the_wrecked_form_reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_BOT" => s_ABOUT_THE_BOT(v),
            "BACK_TO_START" => s_BACK_TO_START(v),
            "BUY_ASK" => s_BUY_ASK(v),
            "BUY_NOT" => s_BUY_NOT(v),
            "BUY_NOT_PEOPLE" => s_BUY_NOT_PEOPLE(v),
            "BUY_NOT_THE_THING" => s_BUY_NOT_THE_THING(v),
            "BUY_NOT_THINGS" => s_BUY_NOT_THINGS(v),
            "BUY_POOL" => s_BUY_POOL(v),
            "BUY_TAIL" => s_BUY_TAIL(v),
            "BUY_TAIL_PHRASES" => s_BUY_TAIL_PHRASES(v),
            "BUY_THING" => s_BUY_THING(v),
            "BUY_THING_STOPS" => s_BUY_THING_STOPS(v),
            "CHEMISTS" => s_CHEMISTS(v),
            "CONTACT_ASKS" => s_CONTACT_ASKS(v),
            "FIGURE_ASKS" => s_FIGURE_ASKS(v),
            "FIGURE_POOL" => s_FIGURE_POOL(v),
            "FILLING_IT_IN" => s_FILLING_IT_IN(v),
            "GET_TO_ASKS" => s_GET_TO_ASKS(v),
            "GOES_TO" => s_GOES_TO(v),
            "HIRING" => s_HIRING(v),
            "HOW_MANY_POOL" => s_HOW_MANY_POOL(v),
            "IN_A_PLACE" => s_IN_A_PLACE(v),
            "LOST_THE_WORK" => s_LOST_THE_WORK(v),
            "MATERIALS" => s_MATERIALS(v),
            "MATERIAL_ASK" => s_MATERIAL_ASK(v),
            "METHOD_ASKS" => s_METHOD_ASKS(v),
            "NEAR_ME" => s_NEAR_ME(v),
            "NOBODY_NAMED" => s_NOBODY_NAMED(v),
            "NOT_A_HIRE" => s_NOT_A_HIRE(v),
            "NOT_A_JOURNEY" => s_NOT_A_JOURNEY(v),
            "NOT_A_LOOKUP" => s_NOT_A_LOOKUP(v),
            "NOT_A_PICK" => s_NOT_A_PICK(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NOT_A_TRADE" => s_NOT_A_TRADE(v),
            "NOT_A_WRECKED_FORM" => s_NOT_A_WRECKED_FORM(v),
            "ON_A_SCREEN" => s_ON_A_SCREEN(v),
            "OPEN_CHEMIST_HOURS" => s_OPEN_CHEMIST_HOURS(v),
            "OPEN_CHEMIST_POOL" => s_OPEN_CHEMIST_POOL(v),
            "OWN_PERSON" => s_OWN_PERSON(v),
            "OWN_PROFESSIONAL" => s_OWN_PROFESSIONAL(v),
            "PICKED_IN_A_PLACE" => s_PICKED_IN_A_PLACE(v),
            "PICK_ANYWHERE_POOL" => s_PICK_ANYWHERE_POOL(v),
            "PICK_ASKS" => s_PICK_ASKS(v),
            "PICK_POOL" => s_PICK_POOL(v),
            "PLACE_NOUNS" => s_PLACE_NOUNS(v),
            "PLACE_POOL" => s_PLACE_POOL(v),
            "PLACE_ROUTE_POOL" => s_PLACE_ROUTE_POOL(v),
            "POLICY_WORDS" => s_POLICY_WORDS(v),
            "PROFESSIONAL_POOL" => s_PROFESSIONAL_POOL(v),
            "QUOTING_TRADES" => s_QUOTING_TRADES(v),
            "READERS" => s_READERS(v),
            "RECORD_ASKED" => s_RECORD_ASKED(v),
            "RECORD_ASKS" => s_RECORD_ASKS(v),
            "RECORD_POOL" => s_RECORD_POOL(v),
            "ROUTE_ASKS" => s_ROUTE_ASKS(v),
            "SAVING_FAILED" => s_SAVING_FAILED(v),
            "SERVICE_ASK" => s_SERVICE_ASK(v),
            "SERVICE_JOB_NAMES" => s_SERVICE_JOB_NAMES(v),
            "SERVICE_NOT_A_THING" => s_SERVICE_NOT_A_THING(v),
            "SERVICE_NOT_HEALTH" => s_SERVICE_NOT_HEALTH(v),
            "SERVICE_NOUNS" => s_SERVICE_NOUNS(v),
            "SERVICE_PHRASES" => s_SERVICE_PHRASES(v),
            "SERVICE_POOL" => s_SERVICE_POOL(v),
            "SERVICE_STOPS" => s_SERVICE_STOPS(v),
            "SERVICE_THING" => s_SERVICE_THING(v),
            "SERVICE_WORK" => s_SERVICE_WORK(v),
            "SETTING_ASKS" => s_SETTING_ASKS(v),
            "TAKE_TO_ASKS" => s_TAKE_TO_ASKS(v),
            "THE_BARE_FORMS" => s_THE_BARE_FORMS(v),
            "THE_WORK_ITSELF" => s_THE_WORK_ITSELF(v),
            "TIDE_ASK" => s_TIDE_ASK(v),
            "TIDE_POOL" => s_TIDE_POOL(v),
            "TIDE_WHY" => s_TIDE_WHY(v),
            "TRADES" => s_TRADES(v),
            "TRADE_POOL" => s_TRADE_POOL(v),
            "TRAVEL_TOPICS" => s_TRAVEL_TOPICS(v),
            "TRAVEL_WORDS" => s_TRAVEL_WORDS(v),
            "WHERE_ASKS" => s_WHERE_ASKS(v),
            "WHO_FACT" => s_WHO_FACT(v),
            "WHO_POOL" => s_WHO_POOL(v),
            "WORTH_VISITING" => s_WORTH_VISITING(v),
            "WRECKED_POOL" => s_WRECKED_POOL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.finding_out", module_obj);
    pyrt::xrefs::reg_fn("compose.finding_out", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_names_one_of", py_func("_names_one_of", |args: &[Py]| _names_one_of(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_the_trade", py_func("_the_trade", |args: &[Py]| _the_trade(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_trade_route", py_func("_trade_route", |args: &[Py]| _trade_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_buy_route", py_func("_buy_route", |args: &[Py]| _buy_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_the_service", py_func("_the_service", |args: &[Py]| _the_service(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_service_route", py_func("_service_route", |args: &[Py]| _service_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_about_a_place", py_func("_about_a_place", |args: &[Py]| _about_a_place(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_the_picked_thing", py_func("_the_picked_thing", |args: &[Py]| _the_picked_thing(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_pick_route", py_func("_pick_route", |args: &[Py]| _pick_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_how_many_route", py_func("_how_many_route", |args: &[Py]| _how_many_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_place_route", py_func("_place_route", |args: &[Py]| _place_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_route_route", py_func("_route_route", |args: &[Py]| _route_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_open_chemist_route", py_func("_open_chemist_route", |args: &[Py]| _open_chemist_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_who_route", py_func("_who_route", |args: &[Py]| _who_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_tide_route", py_func("_tide_route", |args: &[Py]| _tide_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_figure_route", py_func("_figure_route", |args: &[Py]| _figure_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_record_route", py_func("_record_route", |args: &[Py]| _record_route(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "_the_reading", py_func("_the_reading", |args: &[Py]| _the_reading(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "the_wrecked_form_reply", py_func("the_wrecked_form_reply", |args: &[Py]| the_wrecked_form_reply(args)));
    pyrt::xrefs::reg_fn("compose.finding_out", "for_the_ask", py_func("for_the_ask", |args: &[Py]| for_the_ask(args)));
    pyrt::xrefs::reg_access("compose.finding_out", "ABOUT_THE_BOT", g_ABOUT_THE_BOT, s_ABOUT_THE_BOT);
    pyrt::xrefs::reg_access("compose.finding_out", "BACK_TO_START", g_BACK_TO_START, s_BACK_TO_START);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_ASK", g_BUY_ASK, s_BUY_ASK);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_NOT", g_BUY_NOT, s_BUY_NOT);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_NOT_PEOPLE", g_BUY_NOT_PEOPLE, s_BUY_NOT_PEOPLE);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_NOT_THE_THING", g_BUY_NOT_THE_THING, s_BUY_NOT_THE_THING);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_NOT_THINGS", g_BUY_NOT_THINGS, s_BUY_NOT_THINGS);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_POOL", g_BUY_POOL, s_BUY_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_TAIL", g_BUY_TAIL, s_BUY_TAIL);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_TAIL_PHRASES", g_BUY_TAIL_PHRASES, s_BUY_TAIL_PHRASES);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_THING", g_BUY_THING, s_BUY_THING);
    pyrt::xrefs::reg_access("compose.finding_out", "BUY_THING_STOPS", g_BUY_THING_STOPS, s_BUY_THING_STOPS);
    pyrt::xrefs::reg_access("compose.finding_out", "CHEMISTS", g_CHEMISTS, s_CHEMISTS);
    pyrt::xrefs::reg_access("compose.finding_out", "CONTACT_ASKS", g_CONTACT_ASKS, s_CONTACT_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "FIGURE_ASKS", g_FIGURE_ASKS, s_FIGURE_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "FIGURE_POOL", g_FIGURE_POOL, s_FIGURE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "FILLING_IT_IN", g_FILLING_IT_IN, s_FILLING_IT_IN);
    pyrt::xrefs::reg_access("compose.finding_out", "GET_TO_ASKS", g_GET_TO_ASKS, s_GET_TO_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "GOES_TO", g_GOES_TO, s_GOES_TO);
    pyrt::xrefs::reg_access("compose.finding_out", "HIRING", g_HIRING, s_HIRING);
    pyrt::xrefs::reg_access("compose.finding_out", "HOW_MANY_POOL", g_HOW_MANY_POOL, s_HOW_MANY_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "IN_A_PLACE", g_IN_A_PLACE, s_IN_A_PLACE);
    pyrt::xrefs::reg_access("compose.finding_out", "LOST_THE_WORK", g_LOST_THE_WORK, s_LOST_THE_WORK);
    pyrt::xrefs::reg_access("compose.finding_out", "MATERIALS", g_MATERIALS, s_MATERIALS);
    pyrt::xrefs::reg_access("compose.finding_out", "MATERIAL_ASK", g_MATERIAL_ASK, s_MATERIAL_ASK);
    pyrt::xrefs::reg_access("compose.finding_out", "METHOD_ASKS", g_METHOD_ASKS, s_METHOD_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "NEAR_ME", g_NEAR_ME, s_NEAR_ME);
    pyrt::xrefs::reg_access("compose.finding_out", "NOBODY_NAMED", g_NOBODY_NAMED, s_NOBODY_NAMED);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_HIRE", g_NOT_A_HIRE, s_NOT_A_HIRE);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_JOURNEY", g_NOT_A_JOURNEY, s_NOT_A_JOURNEY);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_LOOKUP", g_NOT_A_LOOKUP, s_NOT_A_LOOKUP);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_PICK", g_NOT_A_PICK, s_NOT_A_PICK);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_TRADE", g_NOT_A_TRADE, s_NOT_A_TRADE);
    pyrt::xrefs::reg_access("compose.finding_out", "NOT_A_WRECKED_FORM", g_NOT_A_WRECKED_FORM, s_NOT_A_WRECKED_FORM);
    pyrt::xrefs::reg_access("compose.finding_out", "ON_A_SCREEN", g_ON_A_SCREEN, s_ON_A_SCREEN);
    pyrt::xrefs::reg_access("compose.finding_out", "OPEN_CHEMIST_HOURS", g_OPEN_CHEMIST_HOURS, s_OPEN_CHEMIST_HOURS);
    pyrt::xrefs::reg_access("compose.finding_out", "OPEN_CHEMIST_POOL", g_OPEN_CHEMIST_POOL, s_OPEN_CHEMIST_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "OWN_PERSON", g_OWN_PERSON, s_OWN_PERSON);
    pyrt::xrefs::reg_access("compose.finding_out", "OWN_PROFESSIONAL", g_OWN_PROFESSIONAL, s_OWN_PROFESSIONAL);
    pyrt::xrefs::reg_access("compose.finding_out", "PICKED_IN_A_PLACE", g_PICKED_IN_A_PLACE, s_PICKED_IN_A_PLACE);
    pyrt::xrefs::reg_access("compose.finding_out", "PICK_ANYWHERE_POOL", g_PICK_ANYWHERE_POOL, s_PICK_ANYWHERE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "PICK_ASKS", g_PICK_ASKS, s_PICK_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "PICK_POOL", g_PICK_POOL, s_PICK_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "PLACE_NOUNS", g_PLACE_NOUNS, s_PLACE_NOUNS);
    pyrt::xrefs::reg_access("compose.finding_out", "PLACE_POOL", g_PLACE_POOL, s_PLACE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "PLACE_ROUTE_POOL", g_PLACE_ROUTE_POOL, s_PLACE_ROUTE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "POLICY_WORDS", g_POLICY_WORDS, s_POLICY_WORDS);
    pyrt::xrefs::reg_access("compose.finding_out", "PROFESSIONAL_POOL", g_PROFESSIONAL_POOL, s_PROFESSIONAL_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "QUOTING_TRADES", g_QUOTING_TRADES, s_QUOTING_TRADES);
    pyrt::xrefs::reg_access("compose.finding_out", "READERS", g_READERS, s_READERS);
    pyrt::xrefs::reg_access("compose.finding_out", "RECORD_ASKED", g_RECORD_ASKED, s_RECORD_ASKED);
    pyrt::xrefs::reg_access("compose.finding_out", "RECORD_ASKS", g_RECORD_ASKS, s_RECORD_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "RECORD_POOL", g_RECORD_POOL, s_RECORD_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "ROUTE_ASKS", g_ROUTE_ASKS, s_ROUTE_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "SAVING_FAILED", g_SAVING_FAILED, s_SAVING_FAILED);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_ASK", g_SERVICE_ASK, s_SERVICE_ASK);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_JOB_NAMES", g_SERVICE_JOB_NAMES, s_SERVICE_JOB_NAMES);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_NOT_A_THING", g_SERVICE_NOT_A_THING, s_SERVICE_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_NOT_HEALTH", g_SERVICE_NOT_HEALTH, s_SERVICE_NOT_HEALTH);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_NOUNS", g_SERVICE_NOUNS, s_SERVICE_NOUNS);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_PHRASES", g_SERVICE_PHRASES, s_SERVICE_PHRASES);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_POOL", g_SERVICE_POOL, s_SERVICE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_STOPS", g_SERVICE_STOPS, s_SERVICE_STOPS);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_THING", g_SERVICE_THING, s_SERVICE_THING);
    pyrt::xrefs::reg_access("compose.finding_out", "SERVICE_WORK", g_SERVICE_WORK, s_SERVICE_WORK);
    pyrt::xrefs::reg_access("compose.finding_out", "SETTING_ASKS", g_SETTING_ASKS, s_SETTING_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "TAKE_TO_ASKS", g_TAKE_TO_ASKS, s_TAKE_TO_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "THE_BARE_FORMS", g_THE_BARE_FORMS, s_THE_BARE_FORMS);
    pyrt::xrefs::reg_access("compose.finding_out", "THE_WORK_ITSELF", g_THE_WORK_ITSELF, s_THE_WORK_ITSELF);
    pyrt::xrefs::reg_access("compose.finding_out", "TIDE_ASK", g_TIDE_ASK, s_TIDE_ASK);
    pyrt::xrefs::reg_access("compose.finding_out", "TIDE_POOL", g_TIDE_POOL, s_TIDE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "TIDE_WHY", g_TIDE_WHY, s_TIDE_WHY);
    pyrt::xrefs::reg_access("compose.finding_out", "TRADES", g_TRADES, s_TRADES);
    pyrt::xrefs::reg_access("compose.finding_out", "TRADE_POOL", g_TRADE_POOL, s_TRADE_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "TRAVEL_TOPICS", g_TRAVEL_TOPICS, s_TRAVEL_TOPICS);
    pyrt::xrefs::reg_access("compose.finding_out", "TRAVEL_WORDS", g_TRAVEL_WORDS, s_TRAVEL_WORDS);
    pyrt::xrefs::reg_access("compose.finding_out", "WHERE_ASKS", g_WHERE_ASKS, s_WHERE_ASKS);
    pyrt::xrefs::reg_access("compose.finding_out", "WHO_FACT", g_WHO_FACT, s_WHO_FACT);
    pyrt::xrefs::reg_access("compose.finding_out", "WHO_POOL", g_WHO_POOL, s_WHO_POOL);
    pyrt::xrefs::reg_access("compose.finding_out", "WORTH_VISITING", g_WORTH_VISITING, s_WORTH_VISITING);
    pyrt::xrefs::reg_access("compose.finding_out", "WRECKED_POOL", g_WRECKED_POOL, s_WRECKED_POOL);
}

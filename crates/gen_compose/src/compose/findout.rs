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


thread_local! { static G_AN_APPLIANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_LOCAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_BE_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_COVERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_EXACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_GET_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_SEND_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_SIZE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_TOOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AVAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AVAIL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_AND_E: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BACK_REFERENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMMITMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COOKING_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DECISION_ABOUT_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIXTURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_ON_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JUDGEMENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_METHOD_OR_ADVICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOW_OR_NEVER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_IN_A_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_READING_JUST_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECEIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATUS_SPENT_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY_NO_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUSY_NO_PLACE_AND_CONTEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DELIVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DELIVER_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_COMPOUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLDS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOURS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KICK_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KICK_OFF_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCAL_AREA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOCAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARBY_AREA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARBY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARBY_SERVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAREST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_THE_CONVERSATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_THE_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_LOOKUP_BUT_A_REQUEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_VENUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ON_THE_PLATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THEIR_STATUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICIAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICIAL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICIAL_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENER_TERMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULES_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULES_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATUS_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TERM_DATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TERM_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_COPY_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_ONLINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_CLAIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHATS_ON_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_IS_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_APPLIANCE() -> Py { ensure_init(); G_AN_APPLIANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_APPLIANCE(v: Py) { ensure_init(); G_AN_APPLIANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_LOCAL() -> Py { ensure_init(); G_ASKS_LOCAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_LOCAL(v: Py) { ensure_init(); G_ASKS_LOCAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_BE_IN() -> Py { ensure_init(); G_ASK_BE_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_BE_IN(v: Py) { ensure_init(); G_ASK_BE_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_COVERED() -> Py { ensure_init(); G_ASK_COVERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_COVERED(v: Py) { ensure_init(); G_ASK_COVERED.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_EXACT() -> Py { ensure_init(); G_ASK_EXACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_EXACT(v: Py) { ensure_init(); G_ASK_EXACT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_GET_OUT() -> Py { ensure_init(); G_ASK_GET_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_GET_OUT(v: Py) { ensure_init(); G_ASK_GET_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_HOW_LONG() -> Py { ensure_init(); G_ASK_HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_HOW_LONG(v: Py) { ensure_init(); G_ASK_HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_SEND_BACK() -> Py { ensure_init(); G_ASK_SEND_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_SEND_BACK(v: Py) { ensure_init(); G_ASK_SEND_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_SIZE() -> Py { ensure_init(); G_ASK_SIZE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_SIZE(v: Py) { ensure_init(); G_ASK_SIZE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_STOP() -> Py { ensure_init(); G_ASK_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_STOP(v: Py) { ensure_init(); G_ASK_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_TOOLS() -> Py { ensure_init(); G_ASK_TOOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_TOOLS(v: Py) { ensure_init(); G_ASK_TOOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_AVAIL() -> Py { ensure_init(); G_AVAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AVAIL(v: Py) { ensure_init(); G_AVAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_AVAIL_LINES() -> Py { ensure_init(); G_AVAIL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AVAIL_LINES(v: Py) { ensure_init(); G_AVAIL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_AND_E() -> Py { ensure_init(); G_A_AND_E.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AND_E(v: Py) { ensure_init(); G_A_AND_E.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BACK_REFERENCE() -> Py { ensure_init(); G_A_BACK_REFERENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BACK_REFERENCE(v: Py) { ensure_init(); G_A_BACK_REFERENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMMITMENT() -> Py { ensure_init(); G_A_COMMITMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMMITMENT(v: Py) { ensure_init(); G_A_COMMITMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COOKING_LINE() -> Py { ensure_init(); G_A_COOKING_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COOKING_LINE(v: Py) { ensure_init(); G_A_COOKING_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DECISION_ABOUT_GOING_ON() -> Py { ensure_init(); G_A_DECISION_ABOUT_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DECISION_ABOUT_GOING_ON(v: Py) { ensure_init(); G_A_DECISION_ABOUT_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIXTURE() -> Py { ensure_init(); G_A_FIXTURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIXTURE(v: Py) { ensure_init(); G_A_FIXTURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FOLLOW_UP() -> Py { ensure_init(); G_A_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FOLLOW_UP(v: Py) { ensure_init(); G_A_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_ON_IT() -> Py { ensure_init(); G_A_JOB_ON_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_ON_IT(v: Py) { ensure_init(); G_A_JOB_ON_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JUDGEMENT_ASK() -> Py { ensure_init(); G_A_JUDGEMENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JUDGEMENT_ASK(v: Py) { ensure_init(); G_A_JUDGEMENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KIT() -> Py { ensure_init(); G_A_KIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KIT(v: Py) { ensure_init(); G_A_KIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_METHOD_OR_ADVICE_ASK() -> Py { ensure_init(); G_A_METHOD_OR_ADVICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_METHOD_OR_ADVICE_ASK(v: Py) { ensure_init(); G_A_METHOD_OR_ADVICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOW_OR_NEVER_ASK() -> Py { ensure_init(); G_A_NOW_OR_NEVER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOW_OR_NEVER_ASK(v: Py) { ensure_init(); G_A_NOW_OR_NEVER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_SUBJECT() -> Py { ensure_init(); G_A_PERSON_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_SUBJECT(v: Py) { ensure_init(); G_A_PERSON_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_IN_A_TURN() -> Py { ensure_init(); G_A_PLACE_IN_A_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_IN_A_TURN(v: Py) { ensure_init(); G_A_PLACE_IN_A_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_NAMED() -> Py { ensure_init(); G_A_PLACE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_NAMED(v: Py) { ensure_init(); G_A_PLACE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_ASK() -> Py { ensure_init(); G_A_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_ASK(v: Py) { ensure_init(); G_A_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_READING_JUST_GIVEN() -> Py { ensure_init(); G_A_READING_JUST_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_READING_JUST_GIVEN(v: Py) { ensure_init(); G_A_READING_JUST_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECEIPT() -> Py { ensure_init(); G_A_RECEIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECEIPT(v: Py) { ensure_init(); G_A_RECEIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPORT() -> Py { ensure_init(); G_A_REPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPORT(v: Py) { ensure_init(); G_A_REPORT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATUS_SPENT_ON() -> Py { ensure_init(); G_A_STATUS_SPENT_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATUS_SPENT_ON(v: Py) { ensure_init(); G_A_STATUS_SPENT_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT_NOUN() -> Py { ensure_init(); G_A_WAIT_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_NOUN(v: Py) { ensure_init(); G_A_WAIT_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHY_ASK() -> Py { ensure_init(); G_A_WHY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHY_ASK(v: Py) { ensure_init(); G_A_WHY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY() -> Py { ensure_init(); G_BUSY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY(v: Py) { ensure_init(); G_BUSY.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY_ASKS() -> Py { ensure_init(); G_BUSY_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY_ASKS(v: Py) { ensure_init(); G_BUSY_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY_LINES() -> Py { ensure_init(); G_BUSY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY_LINES(v: Py) { ensure_init(); G_BUSY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY_NO_PLACE() -> Py { ensure_init(); G_BUSY_NO_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY_NO_PLACE(v: Py) { ensure_init(); G_BUSY_NO_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_BUSY_NO_PLACE_AND_CONTEXT() -> Py { ensure_init(); G_BUSY_NO_PLACE_AND_CONTEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUSY_NO_PLACE_AND_CONTEXT(v: Py) { ensure_init(); G_BUSY_NO_PLACE_AND_CONTEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRIERS() -> Py { ensure_init(); G_CARRIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRIERS(v: Py) { ensure_init(); G_CARRIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD() -> Py { ensure_init(); G_CROWD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD(v: Py) { ensure_init(); G_CROWD.with(|c| *c.borrow_mut() = v); }
pub fn g_DELIVER() -> Py { ensure_init(); G_DELIVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DELIVER(v: Py) { ensure_init(); G_DELIVER.with(|c| *c.borrow_mut() = v); }
pub fn g_DELIVER_LINES() -> Py { ensure_init(); G_DELIVER_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DELIVER_LINES(v: Py) { ensure_init(); G_DELIVER_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_DIED() -> Py { ensure_init(); G_DIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIED(v: Py) { ensure_init(); G_DIED.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_ASK() -> Py { ensure_init(); G_FREE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_ASK(v: Py) { ensure_init(); G_FREE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_COMPOUND() -> Py { ensure_init(); G_FREE_COMPOUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_COMPOUND(v: Py) { ensure_init(); G_FREE_COMPOUND.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME() -> Py { ensure_init(); G_GAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME(v: Py) { ensure_init(); G_GAME.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLDS_IT() -> Py { ensure_init(); G_HOLDS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLDS_IT(v: Py) { ensure_init(); G_HOLDS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOURS() -> Py { ensure_init(); G_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOURS(v: Py) { ensure_init(); G_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOURS_ASK() -> Py { ensure_init(); G_HOURS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOURS_ASK(v: Py) { ensure_init(); G_HOURS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO_OPEN() -> Py { ensure_init(); G_HOW_TO_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_OPEN(v: Py) { ensure_init(); G_HOW_TO_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_KICK_OFF() -> Py { ensure_init(); G_KICK_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KICK_OFF(v: Py) { ensure_init(); G_KICK_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_KICK_OFF_LINES() -> Py { ensure_init(); G_KICK_OFF_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KICK_OFF_LINES(v: Py) { ensure_init(); G_KICK_OFF_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS() -> Py { ensure_init(); G_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); G_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_ASK() -> Py { ensure_init(); G_LATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_ASK(v: Py) { ensure_init(); G_LATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCAL_AREA() -> Py { ensure_init(); G_LOCAL_AREA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCAL_AREA(v: Py) { ensure_init(); G_LOCAL_AREA.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCAL_ASK() -> Py { ensure_init(); G_LOCAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCAL_ASK(v: Py) { ensure_init(); G_LOCAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LOCAL_LINES() -> Py { ensure_init(); G_LOCAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOCAL_LINES(v: Py) { ensure_init(); G_LOCAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARBY_AREA() -> Py { ensure_init(); G_NEARBY_AREA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARBY_AREA(v: Py) { ensure_init(); G_NEARBY_AREA.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARBY_LINES() -> Py { ensure_init(); G_NEARBY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARBY_LINES(v: Py) { ensure_init(); G_NEARBY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARBY_SERVICE() -> Py { ensure_init(); G_NEARBY_SERVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARBY_SERVICE(v: Py) { ensure_init(); G_NEARBY_SERVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAREST() -> Py { ensure_init(); G_NEAREST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAREST(v: Py) { ensure_init(); G_NEAREST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_THE_CONVERSATION() -> Py { ensure_init(); G_NOT_ABOUT_THE_CONVERSATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_THE_CONVERSATION(v: Py) { ensure_init(); G_NOT_ABOUT_THE_CONVERSATION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_THE_VISIT() -> Py { ensure_init(); G_NOT_ABOUT_THE_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_THE_VISIT(v: Py) { ensure_init(); G_NOT_ABOUT_THE_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_LOOKUP_BUT_A_REQUEST() -> Py { ensure_init(); G_NOT_A_LOOKUP_BUT_A_REQUEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LOOKUP_BUT_A_REQUEST(v: Py) { ensure_init(); G_NOT_A_LOOKUP_BUT_A_REQUEST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_VENUE() -> Py { ensure_init(); G_NOT_A_VENUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VENUE(v: Py) { ensure_init(); G_NOT_A_VENUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_HERE() -> Py { ensure_init(); G_NOT_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_HERE(v: Py) { ensure_init(); G_NOT_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ON_THE_PLATE() -> Py { ensure_init(); G_NOT_ON_THE_PLATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ON_THE_PLATE(v: Py) { ensure_init(); G_NOT_ON_THE_PLATE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THEIR_STATUS() -> Py { ensure_init(); G_NOT_THEIR_STATUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIR_STATUS(v: Py) { ensure_init(); G_NOT_THEIR_STATUS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICIAL_ASK() -> Py { ensure_init(); G_OFFICIAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICIAL_ASK(v: Py) { ensure_init(); G_OFFICIAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICIAL_LINES() -> Py { ensure_init(); G_OFFICIAL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICIAL_LINES(v: Py) { ensure_init(); G_OFFICIAL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICIAL_THING() -> Py { ensure_init(); G_OFFICIAL_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICIAL_THING(v: Py) { ensure_init(); G_OFFICIAL_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_ASK() -> Py { ensure_init(); G_ON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_ASK(v: Py) { ensure_init(); G_ON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_EVENT() -> Py { ensure_init(); G_ON_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_EVENT(v: Py) { ensure_init(); G_ON_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_WORDS() -> Py { ensure_init(); G_ON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_WORDS(v: Py) { ensure_init(); G_ON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENER() -> Py { ensure_init(); G_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENER(v: Py) { ensure_init(); G_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENER_TERMS() -> Py { ensure_init(); G_OPENER_TERMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENER_TERMS(v: Py) { ensure_init(); G_OPENER_TERMS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS_ASKING() -> Py { ensure_init(); G_OPENS_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS_ASKING(v: Py) { ensure_init(); G_OPENS_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK() -> Py { ensure_init(); G_PAPERWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK(v: Py) { ensure_init(); G_PAPERWORK.with(|c| *c.borrow_mut() = v); }
pub fn g_PET() -> Py { ensure_init(); G_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET(v: Py) { ensure_init(); G_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_ASK() -> Py { ensure_init(); G_PRICE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); G_PRICE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_LINES() -> Py { ensure_init(); G_PRICE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_LINES(v: Py) { ensure_init(); G_PRICE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_THING() -> Py { ensure_init(); G_PRICE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_THING(v: Py) { ensure_init(); G_PRICE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION() -> Py { ensure_init(); G_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); G_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_KIND() -> Py { ensure_init(); G_QUESTION_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_KIND(v: Py) { ensure_init(); G_QUESTION_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_ROAD() -> Py { ensure_init(); G_ROAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROAD(v: Py) { ensure_init(); G_ROAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RULES_ASK() -> Py { ensure_init(); G_RULES_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULES_ASK(v: Py) { ensure_init(); G_RULES_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_RULES_LINES() -> Py { ensure_init(); G_RULES_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULES_LINES(v: Py) { ensure_init(); G_RULES_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_STATUS() -> Py { ensure_init(); G_STATUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATUS(v: Py) { ensure_init(); G_STATUS.with(|c| *c.borrow_mut() = v); }
pub fn g_STATUS_PHRASES() -> Py { ensure_init(); G_STATUS_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATUS_PHRASES(v: Py) { ensure_init(); G_STATUS_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_STORY() -> Py { ensure_init(); G_STORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STORY(v: Py) { ensure_init(); G_STORY.with(|c| *c.borrow_mut() = v); }
pub fn g_TERM_DATES() -> Py { ensure_init(); G_TERM_DATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TERM_DATES(v: Py) { ensure_init(); G_TERM_DATES.with(|c| *c.borrow_mut() = v); }
pub fn g_TERM_LINES() -> Py { ensure_init(); G_TERM_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TERM_LINES(v: Py) { ensure_init(); G_TERM_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_COPY_ASKS() -> Py { ensure_init(); G_THEIR_COPY_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_COPY_ASKS(v: Py) { ensure_init(); G_THEIR_COPY_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN() -> Py { ensure_init(); G_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); G_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_ONLINE() -> Py { ensure_init(); G_THEIR_OWN_ONLINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_ONLINE(v: Py) { ensure_init(); G_THEIR_OWN_ONLINE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PEOPLE() -> Py { ensure_init(); G_THE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PEOPLE(v: Py) { ensure_init(); G_THE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL() -> Py { ensure_init(); G_TRAVEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL(v: Py) { ensure_init(); G_TRAVEL.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_CLAIM() -> Py { ensure_init(); G_TRAVEL_CLAIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_CLAIM(v: Py) { ensure_init(); G_TRAVEL_CLAIM.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_LINES() -> Py { ensure_init(); G_TRAVEL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_LINES(v: Py) { ensure_init(); G_TRAVEL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHATS_ON_LINES() -> Py { ensure_init(); G_WHATS_ON_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHATS_ON_LINES(v: Py) { ensure_init(); G_WHATS_ON_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_IS_GOOD() -> Py { ensure_init(); G_WHAT_IS_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_IS_GOOD(v: Py) { ensure_init(); G_WHAT_IS_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__DETERMINER() -> Py { ensure_init(); G__DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DETERMINER(v: Py) { ensure_init(); G__DETERMINER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Where the answer lives, when it is not in this bot.\n\nThe last thing the bot says to a question nothing else answered is an honest hedge: \"I'd\nrather not guess, a quick search online will get you a solid answer\". It is true, and it fits\nnothing in particular - a person asking what time the tip shuts gets no more out of it than a\nperson asking who won the darts, and STYLE.md's \"Not knowing, honestly\" asks for the place\nthe answer lives instead:\n\n    **User:** What time does the post office on Hill Road close today?\n    **Bot:** I can't check live opening hours, sorry. Search for the branch online: its\n    listing on a map app usually shows today's hours, holidays included.\n\nNearly every such line has the same shape underneath: the answer exists, and it lives in one\nnamed place - the venue's own page, the operator's app, a map app, the office that\nissues the thing. This module reads which of those the ask needs and says it,\nwith what the person will find there. Nothing here is a fact about the world, so nothing can\nbe invented and nothing goes stale.\n\nKinds are read from the ask rather than from a list of subjects, so a line nobody has written\ndown (\"when does the bottle bank shut\", \"are there delays on the a38\") lands in the same place\nas the ones in the deck. A line that is not one of these gets None and keeps whatever the\ncomposer had.\n");
    s_HOURS(py_tuple(vec![py_str("Opening hours are set by each branch rather than by the chain, and they shift at bank holidays and at short notice. The branch's own page or its map listing has today's - that's the bit that catches people out."), py_str("A branch keeps its own hours and they move for holidays, a delivery day or a refit, so nothing said from here would be right today. A map app's listing for that branch is the quickest check, and a phone call settles it."), py_str("Hours like that are local and they change: bank holidays, stock takes, a summer timetable. Ring the place or look at its map listing - both are updated when the hours move.")]));
    s_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    shop shops store stores supermarket tesco asda sainsburys sainsbury aldi lidl morrisons\n    coop co-op iceland waitrose branch bank banks building society library libraries tip dump\n    recycling pool lido leisure gym market stall cafe caff cafe restaurant chippy chippies\n    takeaway takeaway kebab chip pizza pub pubs bar pharmacy chemist chemists surgery gp\n    doctors dentist dental hospital garage showroom museum gallery gallery bowling cinema\n    theatre softplay church chapel club hairdresser hairdressers barber salon vets vet\n    merchants warehouse centre center office reception\n    beach beaches seafront promenade park parks playground zoo farm station airport stadium\n    ground services mall sauna spa rink bowling karting\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_VENUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    window windows jar tin box boxes door doors envelope packet bottle can letter parcel\n    present gift browser app file folder email shift work school run day week\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    son daughter kid kids child children baby wife husband partner mum mom mother dad father\n    boyfriend girlfriend nan grandma grandad grandson granddaughter friend boss manager\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOURS_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|what\\s+time[s]?)\\s+(?:does|do|is|are|will|would|can|should)\\b[^?]{0,44}?\\b(?:open|opens|opening|close|closes|closing|shut|shuts|shutting)\\b|\\b(?:is|are)\\b[^?]{0,32}?\\bopen\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LATE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+late\\b|\\bopen\\s+late\\b|\\bopen\\s+early\\b|\\bopening\\s+(?:hours|times?)\\b|\\bclosing\\s+time\\b|\\btill\\s+(?:when|what\\s+time)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_TO_OPEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|to|can|should)\\s+(?:i\\s+)?open\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRAVEL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    bus buses coach coaches train trains tram trams tube underground flight flights plane\n    planes ferry ferries motorway motorways a-road by-pass bypass line lines route routes\n    service services replacement bus traffic junction\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STATUS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    running delayed delay delays cancelled canceled cancellation cancel strike strikes closed\n    shut diverted diversion disruption disruptions roadworks traffic\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRAVEL_CLAIM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    claim claims claimed refund refunds compensation compensate money owed reimburse reclaim\n    pays insurance insurer hotel rebook rebooking\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|will|any|does|do)\\b|^\\s*(?:any\\s+)?(?:delays?|disruptions?|traffic|roadworks?|road\\s+works|cancellations?|strikes?|engineering\\s+works)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPENS_ASKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:is|are|will|any|does|do|what|when|how)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STORY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:was|were|got|had|did|have|has|theyve|weve|ive|again|yesterday|this\\s+morning|last\\s+night)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STATUS_PHRASES(py_tuple(vec![py_str("engineering works"), py_str("called off"), py_str("on time"), py_str("bus replacement"), py_str("replacement bus"), py_str("road works")]));
    s_ROAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z0-9])[am]\\s?\\d{1,3}(?![a-z0-9])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRAVEL_LINES(py_tuple(vec![py_str("Disruption can clear or spread within the hour, so only today's version is worth acting on: the operator's app or a journey planner has the next service, and for a road the traffic service names the stretch."), py_str("Whether it is running today is on the operator's own system, and delays move. Their app or a journey planner shows the next one and any disruption - check it just before you set off.")]));
    s_ON_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    cinema theatre theater pictures gig gigs match game show concert festival listings listing\n    play screening movie movies film films odeon vue cineworld multiplex venue\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ON_EVENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    market markets festival fete fair gig gigs match game show concert cinema theatre play\n    screening event events boot sale\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ON_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:whats?|what\\s+is)\\s+on\\b|\\blistings?\\b|\\bwhats\\s+on\\s+at\\b|\\bis\\s+(?:it|that|the\\s+[a-z]+)\\s+on\\b|\\bstill\\s+on\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KICK_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat\\s+time\\s+(?:is|does|do|will)\\b[^?]{0,24}?\\bkick\\s*-?\\s*off\\b|\\bwhen\\s+(?:is|does)\\b[^?]{0,24}?\\bkick\\s*-?\\s*off\\b|\\bwhat\\s+time\\s+(?:is|does)\\s+the\\s+(?:match|game|final)\\b|\\bis\\s+the\\s+(?:match|game)\\s+on\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHAT_IS_GOOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:whats?|what\\s+is|anything)\\s+(?:good|worth\\s+(?:seeing|watching|a\\s+look))\\b|\\bwhats?\\s+(?:the\\s+)?(?:new|latest)\\s+(?:film|movie|play|show)\\b|\\bwhat\\s+(?:film|films|movie|movies|play|plays)\\s+(?:is|are)\\s+(?:on|showing|worth)\\b|\\bwhats?\\s+worth\\s+(?:seeing|watching)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHATS_ON_LINES(py_tuple(vec![py_str("Listings turn over every week and seats sell out quietly, so the venue's own page or a listings app is the only schedule that is right today: times, prices and what is left."), py_str("That is this week's timetable, and it changes more often than a general answer could keep up with. The venue's own page or a listings app has the times, plus the number if you want to check seats.")]));
    s_KICK_OFF_LINES(py_tuple(vec![py_str("Kick-off moves for television and it can move twice, so the club's own page or a sports app is the one to trust - anything said from here could be a week out of date."), py_str("The fixture list is the club's own, and televised games get moved late. Their page or a sports app has the time and says if it has been moved.")]));
    s_BUSY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+busy\\b|\\bis\\s+(?:it|the\\s+[a-z]+)\\s+busy\\b|\\bhow\\s+long\\s+(?:is|are)\\s+(?:the\\s+)?(?:wait|queue|line)\\b|\\bwhats\\s+the\\s+(?:wait|queue|line)\\b|\\bwaiting\\s+times?\\b|\\bis\\s+there\\s+(?:a\\s+|any\\s+)?(?:queue|wait|line)\\b|\\bwill\\s+there\\s+be\\s+(?:a\\s+)?(?:queue|wait|line)\\b|\\bhow\\s+(?:packed|crowded|heaving|rammed)\\b|\\bwhats\\s+the\\s+(?:waiting\\s+time|wait\\s+time)\\b|\\bis\\s+the\\s+(?:queue|wait|line)\\s+(?:long|bad|big|short|quick|slow)\\b|\\bwhen\\s+(?:is|are)\\s+(?:\\w+\\s+){0,4}(?:quiet|emptiest|quietest|least\\s+busy)\\b|\\b(?:is|are|was|were|will|would)\\s+(?:\\w+\\s+){1,4}(?:busy|packed|heaving|rammed|crowded|manic)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CROWD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:busy|packed|heaving|rammed|crowded|manic|chocka|jammed|mobbed)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|will|would|does|do|did)\\s+(?:my\\s+|our\\s+|your\\s+|the\\s+)?(?:he|she|they|him|her|them|you|we|i|mum|mom|mother|mam|dad|father|da|boss|manager|sister|brother|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|mate|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|kid|kids|child|children|baby|lad|lass|boy|girl|colleague|neighbour|neighbor|landlord|tenant|staff|team|people|everyone|anyone|someone|nobody|somebody|bot|assistant|robot)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WAIT_NOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wait|waiting|queue|queues|queuing|queueing|waiting\\s+times?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BUSY_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:is|are|was|were|will|would|do|does|did|can|could|should|any|how|what|whats|when|where|which|who)\\b|\\?\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BUSY_LINES(py_tuple(vec![py_str("A queue changes hour to hour: the weekend and school pick-up are the busy half of the week and a weekday morning the quiet one, and a wet Saturday pushes everybody indoors at once. Nobody publishes it everywhere, so one call settles today faster than anything I could guess, and their own page has a live wait if they keep one."), py_str("It will be worse at the weekend and around school pick-up, and better first thing, but today's wait is only on their system. Ring them before you set off - the place itself is the only one that knows."), py_str("The busy hours are much the same everywhere - late morning to mid afternoon at the weekend, and the hour after school lets out - and the weather moves the whole thing. Today's version is at the place: their own page keeps a live count where it keeps one, and one call settles the rest.")]));
    s_BUSY_NO_PLACE(py_tuple(vec![py_str("A wait is live and local, so the only figure that is right is the one at that place today: their own page keeps it where they keep one, and one call settles the rest. Waits are usually shortest first thing and longest from mid morning on. Which place is it?"), py_str("That changes hour to hour and place to place, so I can't put a number on it - their own page or phone has today's, and a map app's popular times shows the busy hours if you are picking when to go. First thing is usually the quiet end. Which place did you have in mind?")]));
    s_BUSY_NO_PLACE_AND_CONTEXT(py_tuple(vec![py_str("A wait is live and local, so today's figure is only at the place itself: their own page keeps it where they keep one, one call settles the rest, and first thing is usually the quiet end of the day."), py_str("That one moves hour to hour and place to place, so I can't put a number on it: their own page or phone has today's, and a map app's popular times shows the busy hours if you are choosing when to go.")]));
    s_A_PLACE_IN_A_TURN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:beach|seafront|promenade|gym|pool|leisure\\s+cent\\w+|tip|dump|hospital|casualty|clinic|surgery|pharmacy|chemist|supermarket|shop|store|market|pub|restaurant|cafe|takeaway|park|playground|museum|gallery|zoo|library|station|airport|car\\s+park|high\\s+street|town\\s+cent\\w+|mall|ground|stadium|theatre|cinema|venue|club|centre|sauna|spa|dentist|bank|services|attraction|soft\\s*play|ice\\s+rink|bowling)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_AND_E({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a\\s*&\\s*e|a\\s+and\\s+e|a\\s*and\\s*e|casualty|emergency\\s+department|emergency\\s+room|urgent\\s+care|walk[-\\s]?in\\s+cent\\w+|minor\\s+injuries)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRICE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\s+(?:is|are|does|do|will)\\b|\\bwhats?\\s+the\\s+(?:price|charge|cost|fee|rate)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis\\s+(?:the\\s+|a\\s+)?[a-z' -]{2,24}\\s+free\\b|\\bis\\s+(?:entry|admission|parking)\\s+free\\b|\\bis\\s+there\\s+a\\s+charge\\b|\\bpay\\s+to\\s+(?:park|get\\s+in|go\\s+in|enter|visit)\\b|\\bdo\\s+(?:i|we)\\s+have\\s+to\\s+pay\\s+(?:to|for\\s+(?:a|an|the|any|my|our))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FREE_COMPOUND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    gluten dairy sugar fat nut nuts wheat lactose caffeine msg toll duty tax hands care cruelty\n    smoke contact peanut egg soya soya soy\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRICE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    parking park bus buses train trains tube taxi cab admission entry entrance ticket tickets\n    delivery postage post fee fees fare fares hire rental charge\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRICE_LINES(py_tuple(vec![py_str("Rates like that are set by whoever runs the place and they change with the season: their own site or one quick call has the figure they actually charge, and a map listing carries the number if there is no site."), py_str("Admission and parking prices are set locally and move with the season, so the current one is on their website or through one call, and a map listing has the number.")]));
    s_AVAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tickets?|seats?|tables?|rooms?|spaces?|places?|slots?|pitches?|appointments?)\\s+left\\b|\\b(?:are|is)\\s+there\\s+(?:any\\s+|a\\s+)?(?:tickets?|seats?|tables?|rooms?|spaces?|pitches?)\\b|\\b(?:is\\s+it|are\\s+they)\\s+(?:fully\\s+)?(?:booked\\s+up|full|sold\\s+out)\\b|\\b(?:is|are|has|have|did|does|will)\\b[^?]{0,20}?\\bsold\\s+out\\b|\\bcan\\s+(?:i|we)\\s+(?:still\\s+)?(?:get|book)\\s+(?:a|any)\\s+(?:table|seat|ticket|room|space|pitch|slot)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AVAIL_LINES(py_tuple(vec![py_str("What is left is on their booking system rather than anywhere I can see, and it changes by the hour. Their booking page shows what is still free, and the box office settles a sell-out in a second."), py_str("Anything still free is in their own system, sorry - it moves too fast for a general answer. Their booking page shows it, and ringing is quicker if you are on your way.")]));
    s_LOCAL_AREA({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:town|village|area|county|streets?|roads?|ward|district|neighbourhood|postcode|council|city)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOCAL_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+many\\b|\\bpopulation\\b|\\bwho\\s+is\\b|\\bwhich\\s+band\\b|\\bhow\\s+do\\s+i\\s+find\\b|\\bwhat\\s+is\\s+the\\s+population\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOCAL_LINES(py_tuple(vec![py_str("Figures about your own patch come from your council, and they publish them: their site has it - the box at the top takes your postcode - and one call to the main number works if the site buries it."), py_str("Your council holds that and keeps it up to date. Their site has it, or one call to the main number answers it - the postcode does most of the work.")]));
    s_TERM_DATES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bterm\\s+dates?\\b|\\binset\\s+days?\\b|\\bacademic\\s+year\\b|\\bwhen\\s+do\\s+(?:the\\s+)?schools?\\s+(?:break|finish|start)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TERM_LINES(py_tuple(vec![py_str("Term dates are the school's own calendar, and the inset days are the bit a general answer always gets wrong. The school's site has both, and the council lists every school's in one place if you need to compare."), py_str("The school sets its own term dates and its own inset days. Their page has both, and the council's site collects them all if you are looking at more than one school.")]));
    s_NEARBY_AREA({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bnear\\s?(?:me|us|here)\\b|\\bround\\s+here\\b|\\bnear\\sby\\b|\\bnearby\\b|\\bin\\s+my\\s+(?:area|town|village|city|street|road|postcode|neighbourhood)\\b|\\bmy\\s+(?:postcode|area|town|village)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_LOCAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis\\s+there\\b|\\bare\\s+there\\b|\\bwhere\\b|\\bwheres\\b|\\bwhich\\b|\\bany\\b|\\banywhere\\b|\\bcan\\s+(?:i|we|you)\\b|\\bcould\\s+(?:i|we|you)\\b|\\bdo\\s+you\\s+know\\b|\\b(?:i|we)\\s+(?:need|want|are\\s+looking|am\\s+looking)\\b|\\blooking\\s+for\\b|\\brecommend\\b|\\bfind\\b|\\bwhats\\s+good\\b|\\bgood\\s+(?:place|places|spot|spots)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEAREST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:nearest|closest)\\s+(?!to\\b)[a-z]+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEARBY_SERVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    pharmacy chemist cashpoint atm supermarket shop store garage launderette laundrette dentist\n    doctors surgery hospital petrol gym pool park school nursery vet takeaway chippy pub cafe\n    restaurant bank tip\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DELIVER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdeliver(?:y|s|ed)?\\s+(?:to|in)\\b|\\bdeliver\\s+to\\s+my\\b|\\bdo\\s+(?:they|you)\\s+deliver\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DELIVER_LINES(py_tuple(vec![py_str("Delivery depends on the postcode and the day, and only the shop's own checker knows it: put your postcode in on their site and it says yes or no in a second, with the earliest slot."), py_str("That depends on the postcode, which their own delivery checker holds - it takes ten seconds with your postcode in, and it shows the earliest date too.")]));
    s_NEARBY_LINES(py_tuple(vec![py_str("What is near you depends on your postcode, so a map app is the one that knows: it lists the closest in order of distance, with today's hours and a phone number for each one."), py_str("A map app puts them in order of distance and carries the phone number, which is faster than any list I could give you from here.")]));
    s_HOW_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|can|should|would)\\s+(?:i|we)\\b|\\bhow\\s+to\\b|\\bwhat\\s+do\\s+i\\s+need\\s+to\\b|\\bwhats\\s+the\\s+best\\s+way\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OFFICIAL_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:whats|what\\s+is|what\\s+are)\\s+the\\s+(?:law|rules?|legal\\s+position|regulations?)\\b|\\bis\\s+it\\s+(?:legal|allowed|illegal)\\b|\\bam\\s+i\\s+(?:allowed|legally)\\b|\\bdo\\s+i\\s+need\\s+(?:a|an|any|to)\\b|\\bhow\\s+do\\s+i\\s+(?:get|apply|renew|replace|register|order|book)\\b|\\bhow\\s+do\\s+i\\s+(?:check|find\\s+out)\\s+(?:if|whether|who|what)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OFFICIAL_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    licence license permit permission registration certificate copy record records badge card\n    cscs dbs crb passport visa form application skip planning consent exemption grant benefit\n    number registered register renewal certificate blue disabled\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULES_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:law|legal|allowed|rules?|regulations?|illegal)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    uno chess poker monopoly scrabble darts snooker dominoes cluedo risk rummy bingo\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_STOP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    what whats is are the a an it i we you do does to of on for in my our and if can could\n    there be this that rule rules law legal allowed illegal regulations regulation about\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OFFICIAL_LINES(py_tuple(vec![py_str("Anything with a form behind it is set by the office that issues it, and the fee and the paperwork change with the year. Their official page - GOV.UK for the national ones, your council for anything local - names what to send, what it costs and how long it takes."), py_str("The form, the fee and the waiting time all come from the office that issues it, and they change. Their own page is the one to act on, and it says who to ring if your case does not fit the standard one."), py_str("The rules differ by council and by country, so the version that counts is on the issuing office's own page: GOV.UK has the national forms, and the page names who to ring about the rest.")]));
    s_RULES_LINES(py_tuple(vec![py_str("Rules like that are set locally and the byelaws differ from one council to the next: your council's site has them, plus the permit if one is needed, and their phone line answers the odd case."), py_str("The rules differ by council and by country, so the local version is the only one worth acting on: the council's site has it, and GOV.UK has the national half.")]));
    s_PET({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    cat cats dog dogs kitten puppy pet pets hamster rabbit budgie parrot\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DIED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    died dead death passing euthanised put down cremated\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_REPORT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:every\\s+(?:single\\s+)?(?:day|week|morning|evening|night|time)|all\\s+(?:week|weeks|month|months|year|day|days|morning|night|the\\s+time)|for\\s+(?:weeks|months|days|ages|hours)|since\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|last|monday|tuesday|wednesday|thursday|friday|saturday|sunday|easter|christmas|then)|each\\s+(?:day|week|morning)|the\\s+whole\\s+(?:week|month|day)|ruined|ruins|wrecked|wrecks|chaos|nightmare|fed\\s+up|sick\\s+of|so\\s+tired|held\\s+me\\s+up|made\\s+me\\s+late|cost\\s+me|missed\\s+my)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_STATUS_SPENT_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:any\\s+|the\\s+)?(?:delays?|disruptions?|traffic|roadworks?|road\\s+works|cancellations?|strikes?|engineering\\s+works)\\s+(?:on|at|near|around|by|into|outside|through)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_OWN_ONLINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:site|website|web\\s*page|page|shop|store|storefront|listing|listings|blog|channel|profile|etsy|ebay|account|sales)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_THEIR_STATUS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|order|parcel|delivery|claim|refund)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FOLLOW_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:is|are|will|would|how\\s+(?:long|much|bad|far)|what\\s+about|and\\b|does|do)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BACK_REFERENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|they|them|those|this|the\\s+same|still)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLACE_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:map app|maps|google maps|the council|council'?s?|gov\\.uk|\\.gov\\b|government|operator'?s?|journey planner|traffic service|road agency|roads agency|national rail|pharmac(?:y|ist)|chemist|\\bgp\\b|doctor'?s|surgery|the ward|nurse|vet'?s?|veterinary|their (?:own )?(?:site|website|web page|page|app|account|office|desk|number)|the (?:official )?(?:site|website) of|official (?:page|site)|box office|ticket office|the office that|issuing office|passport office|dvla|registry office|landlord|letting agent|estate agent|employer|\\bhr\\b|payroll|your bank|the bank|insurer|insurance company|the airline|the hotel|the venue|the airline'?s?|one call|give them a ring|ring them|call them|their line)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOLDS_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:has|have|shows|lists|carries|publishes|updates)\\b[^.]{0,30}\\b(?:number|form|fee|price|prices|hours|wait|times|schedule|status|page|listing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_KIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:shed|wardrobe|bookcase|bookcases|shelves|shelving|shelf|flat\\s*pack|flatpack|kit|units?|desk|bed|bunk|trampoline|greenhouse|climbing\\s+frame|table|chair|chairs|cabinet|bbq|barbecue|swing\\s*set|playhouse|sideboard|tiles?|flooring|laminate|decking|fence|fencing|gate|sheds|curtain\\s+pole|blinds?|radiator\\s+cover|sofa|settee|couch|mattress|sofa\\s+bed|armchair|dining\\s+table)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIXTURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tap|taps|sink|sinks|shower|showers|toilet|loo|cistern|ballcock|bath|bathroom\\s+suite|stopcock|soil\\s+pipe|waste\\s+pipe)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AN_APPLIANCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:boiler|washing\\s+machine|washer|dishwasher|fridge|freezer|oven|cooker|hob|microwave|kettle|toaster|hoover|vacuum|dryer|tumble\\s+dryer|radiator|thermostat|heater|dehumidifier|mower|lawnmower|strimmer|hedge\\s+trimmer|drill|printer|laptop|phone|tablet|console|telly|tv|television|router|doorbell|camera|battery|charger|remote)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PAPERWORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:letter|form|invoice|bill|statement|contract|policy|insurance|quote|receipt|reference|application|claim|tenancy|lease|mortgage|payslip|p60|p45|council\\s+tax|licence|license|passport|visa|permit|booking|confirmation|delivery|order|parcel|refund|warranty|guarantee|tracking)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_COMMITMENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gym\\s+membership|phone\\s+contract|mobile\\s+contract|care\\s+home|home\\s+care|lease\\s+car|car\\s+finance|direct\\s+debit|times\\s+share|gym|membership|contract|subscription|tariff|tenancy|lease|timeshare|nursery|course|classes|insurance|policy|broadband|loan|finance|hp)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_GET_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bget\\s+out\\s+of\\b|\\bhow\\s+(?:do|can|should)\\s+(?:i|we)\\s+(?:cancel|stop|end|leave|quit|exit)\\b|\\bcancel\\s+(?:it|my|the|this|that|our)\\b|\\bstop\\s+paying\\b|\\bend\\s+(?:the|my|this|our)\\b|\\bgive\\s+notice\\b|\\bopt\\s+out\\b|\\bcome\\s+out\\s+of\\b|\\bbreak\\s+(?:the|my)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ABOUT_THE_CONVERSATION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which)\\s+(?:was|were|did|have|had)\\s+(?:i|we|you)\\b|\\bwhat\\s+am\\s+i\\b|\\bdo\\s+you\\s+remember\\b|\\bwhat\\s+did\\s+(?:i|we|you)\\s+(?:say|ask|tell)\\b|\\bwhat\\s+(?:i|we)\\s+(?:said|asked|told)\\b|\\bremind\\s+me\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_LOOKUP_BUT_A_REQUEST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:help\\s+me|can\\s+you|could\\s+you|would\\s+you|will\\s+you|shall\\s+i)\\s+(?:write|draft|word|put|phrase|text|reply|answer|compose|translate|summari[sz]e|rewrite|shorten|tidy|check|read)\\b|\\bwrit(?:e|ing)\\s+(?:it|that|this|him|her|them)\\b|\\bdraft\\s+(?:it|that|this|something)\\b|\\b(?:write|reply|text|email|message|answer)\\s+(?:to\\s+)?(?:him|her|them|the\\s+\\w+)\\b|\\bhow\\s+(?:do|should)\\s+i\\s+(?:write|word|phrase|put)\\b|\\b(?:re)?word\\s+(?:it|that|this)\\b|^make\\s+(?:it|this|that|them)\\b|\\bmake\\s+it\\s+(?:more|less|a\\s+bit|much)\\b|\\b(?:add|drop|cut|keep)\\s+(?:a|the)\\s+(?:line|bit|sentence|word)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ABOUT_THE_VISIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwill\\s+(?:they|he|she|it)\\s+need\\b|\\bneed\\s+to\\s+come\\s+(?:in|inside|out|back|round)\\b|\\bcome\\s+(?:in|inside|out|back|round)\\b|\\bneed\\s+access\\b|\\bget\\s+to\\s+it\\b|\\bhow\\s+long\\s+will\\s+(?:they|he|she|someone|the\\s+(?:engineer|plumber|fitter|installer|technician|electrician|man))\\b|\\b(?:they|he|she)\\s+(?:turn\\s+up|arrive)\\b|\\b(?:have|get)\\s+(?:it\\s+|everything\\s+|them\\s+)?ready\\b|\\bready\\s+(?:for|when)\\s+(?:them|him|her|someone|the\\s+\\w+)\\b|\\bwhat\\s+(?:do|should)\\s+i\\s+(?:tell|say\\s+to|show|give|ask|bring)\\b|\\bwait\\s+in\\b|\\bbe\\s+(?:in|there)\\s+(?:for|all)\\s+(?:the\\s+)?(?:day|morning|afternoon)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLAN_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat\\s+time\\s+(?:should|do|shall|would)\\s+(?:i|we)\\b|\\bwhen\\s+should\\s+(?:i|we)\\b|\\bshould\\s+(?:i|we)\\s+(?:start|begin|leave|set\\s+off|get\\s+up)\\b|\\bhow\\s+long\\s+(?:before|ahead)\\b|^\\W*shall\\s+i\\b|\\bshall\\s+i\\s+(?:also|do|just|be)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_COOKING_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cook|cooked|cooking|roast|bake|baking|fry|frying|boil|grill|simmer|steam|poach|marinate|season|seasoning|baste|skin|thighs?|breasts?|fillet|mince|pasta|rice|sauce|dough|pastry|chicken|beef|pork|lamb|turkey|fish|prawn|veg|vegetables|potatoes|onion|garlic)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_READING_JUST_GIVEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was|are)\\s+(?:that|this|it|those)\\s+(?:too|about\\s+right|ok\\b|okay\\b|normal\\b|enough\\b|much\\b|low\\b|high\\b|fine\\b)|\\bis\\s+\\d[\\d.,]*\\s*\\w*\\s+(?:enough|too|ok\\b|fine\\b|normal\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WHY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis\\s+that\\s+why\\b|\\bwhy\\s+(?:does|do|is|are|would|did|has|have|am\\s+i)\\b|\\bwhat\\s+causes?\\b|\\bcould\\s+that\\s+be\\s+(?:why|it)\\b|\\bis\\s+that\\s+(?:what|the\\s+reason)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_NOW_OR_NEVER_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:now|today|tonight|tomorrow|this\\s+evening|the\\s+day\\s+before|the\\s+night\\s+before|last\\s+night|overnight)\\b[^.?!]{0,40}\\b(?:can|could|should|shall|will)\\s+(?:i|we)\\b|\\b(?:can|could|should|shall)\\s+(?:i|we)\\b[^.?!]{0,40}\\b(?:now|today|tonight|tomorrow|overnight|the\\s+day\\s+before)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DECISION_ABOUT_GOING_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bshould\\s+(?:i|we)\\s+(?:stop|keep|carry\\s+on|carry|bother|risk|go\\s+ahead|wait|use|still|replace|fix|put\\s+up\\s+with)\\b|\\b(?:is|was)\\s+(?:it|that)\\s+(?:safe|ok\\b|okay\\b|fine\\b|alright)\\s+to\\s+(?:keep|carry|use|go\\s+on|carry\\s+on)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_METHOD_OR_ADVICE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|can|should|would)\\s+(?:i|we|you)\\s+(?:take|get|put|fit|remove|undo|open|change|replace|fix|clean|unblock|unblock|bleed|reset|wire|connect|assemble|dismantle|strip|hang|mount|seal|glue|screw|drill|paint|cut|adjust|tighten|loosen)\\b|\\bwhat\\s+would\\s+you\\s+do\\b|\\bwhat\\s+should\\s+(?:i|we)\\s+do\\b|\\bwhat\\s+do\\s+you\\s+(?:think|reckon|suggest|recommend)\\b|\\bany\\s+(?:advice|ideas|thoughts)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_JUDGEMENT_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bworth\\b|\\bvalue\\b|\\bany\\s+good\\b|\\bgood\\s+(?:idea|buy)\\b|\\bhow\\s+much\\b|\\bcheaper\\b|\\bbetter\\s+buy\\b|\\bshould\\s+(?:i|we)\\s+(?:buy|get)\\b|\\bwaste\\s+of\\s+money\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ON_THE_PLATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bservic(?:e[sd]?|ing)\\b|\\bhow\\s+often\\b|\\brinse\\s+aid\\b|\\bsalt\\b|\\bdescal\\w*|\\badditive\\b|\\bsoftener\\b|\\b(?:which|what)\\s+filter\\b|\\bfilter\\s+(?:does|should)\\b|\\btablets?\\b|\\bpowder\\b|\\bdetergent\\b|\\bhow\\s+(?:do|should)\\s+(?:i|we)\\s+(?:use|load|set|run|fill)\\b|\\bshould\\s+(?:i|we)\\s+(?:use|put|add|fill|run)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_HERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pain|hurts?|ache|aching|sore|swollen|swelling|rash|lump|bleeding|blood|dose|dosage|tablets?|pills?|medicines?|medication|prescription|symptoms?|sick|vomiting|dizzy|breath|chest|stomach|mum|mom|dad|son|daughter|wife|husband|partner|baby|child|kids?|gran|nan)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_TOOLS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what\\s+tools?|what\\s+(?:do|will|would|might)\\s+(?:i|we|you)\\s+need|tools?\\s+(?:do|will)|what\\s+do\\s+i\\s+need\\s+(?:to|for)|what\\s+will\\s+i\\s+need)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_HOW_LONG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+long\\s+(?:does|do|will|would|should|to|is|takes?)\\b|\\bhow\\s+many\\s+hours\\b|\\bhow\\s+long\\s+(?:a|an|one|it|they|that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_SIZE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat\\s+size\\b|\\bwill\\s+it\\s+fit\\b|\\bhow\\s+big\\b|\\bwhat\\s+(?:kind|sort)\\s+of\\s+\\w+\\s+(?:do|will)\\s+(?:i|we)\\b|\\bwhich\\s+(?:one|size|model)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_BE_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbe\\s+in\\s+for\\s+it\\b|\\bneed\\s+(?:to\\s+)?be\\s+(?:in|here|home)\\b|\\bsign\\s+for\\s+it\\b|\\b(?:does|will)\\s+(?:it|someone|they)\\s+need\\s+to\\s+be\\s+in\\b|\\bwaiting\\s+in\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_COVERED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:covered|cover\\s+(?:it|this|that|me)|tell\\s+the\\s+insurance|insurance\\s+(?:about|know)|claim\\s+(?:on|for)|on\\s+the\\s+policy|included|comes?\\s+with\\s+it)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_EXACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhich\\s+(?:model|part|spare|size|one|type|version)\\b|\\bwhat\\s+(?:model|number)\\b|\\bpart\\s+number\\b|\\bwhich\\s+one\\s+(?:do|should|will)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_SEND_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsend\\s+(?:it|them|that|this|the\\s+old\\s+one)\\s+(?:back|off|in)\\b|\\bsend\\s+(?:it\\s+|them\\s+)?(?:back|off)\\b|\\b(?:do|must|should|need\\s+to|have\\s+to)\\s+(?:i\\s+)?(?:return|post|send)\\b|\\breturn\\s+(?:it|them|the\\s+old\\s+one)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARRIERS(py_dict(vec![(py_tuple(vec![py_str("tools"), py_str("kit")]), py_tuple(vec![py_str("The instruction sheet in the box lists the tools and the fixings, and says which of them you can do without"), py_str("The parts list that came with it names the tools it wants, and the fixings are usually in the box with them")])), (py_tuple(vec![py_str("tools"), py_str("appliance")]), py_tuple(vec![py_str("The installation page for your model lists what it needs, and the plate on the machine says which model yours is"), py_str("The maker's own page for that model names the tools and whether the fixings come with it")])), (py_tuple(vec![py_str("how_long"), py_str("kit")]), py_tuple(vec![py_str("The instruction sheet gives the number of hours for two people, and the maker's page usually repeats it with the same figure"), py_str("The maker's page for it has the build time, and the instructions break that down into what has to be done in order")])), (py_tuple(vec![py_str("how_long"), py_str("appliance")]), py_tuple(vec![py_str("The maker's page for your model says how long the job should take, and the plate on it tells you the model"), py_str("Whoever services that make will give you a time on the phone from the model number alone, which is on the plate")])), (py_tuple(vec![py_str("how_long"), py_str("paperwork")]), py_tuple(vec![py_str("The reference on the paperwork is what a straight answer is given against, and the office named on it is the one that holds the times"), py_str("Whoever issued it can say where yours has got to from the reference - and the letter or the confirmation says who that is")])), (py_tuple(vec![py_str("be_in"), py_str("kit")]), py_tuple(vec![py_str("The order confirmation and the tracking page say whether it needs a signature, and they are what count on the day"), py_str("Whoever is delivering it puts the answer on the tracking page, and the confirmation says what happens if nobody is in")])), (py_tuple(vec![py_str("size"), py_str("kit")]), py_tuple(vec![py_str("The maker's page for each version gives the footprint and the clearance it needs, and the size chart is the part that settles which one fits"), py_str("The maker's page has the dimensions for each version, so the sheet and the page together settle which one fits your space")])), (py_tuple(vec![py_str("size"), py_str("appliance")]), py_tuple(vec![py_str("The maker's page for your model has the dimensions, and the plate on it tells you the model"), py_str("The plate on it gives the model number, and the maker's page has the size for that exact one")])), (py_tuple(vec![py_str("be_in"), py_str("paperwork")]), py_tuple(vec![py_str("The order confirmation or the tracking page says whether it needs a signature, and that is the one that counts on the day"), py_str("Whoever is delivering it puts the answer on the tracking page, and the confirmation says what happens if nobody is in")])), (py_tuple(vec![py_str("covered"), py_str("paperwork")]), py_tuple(vec![py_str("The policy or the reference says: what is covered, what the excess is, and what they ask you to tell them, all in the same document"), py_str("The terms that came with it settle it, and a phone call quoting the reference gets a yes or no from the people who apply them")])), (py_tuple(vec![py_str("send_back"), py_str("paperwork")]), py_tuple(vec![py_str("What it wants back, in what state and by when is on the page it came with, and the reference on it is how you check they have got it"), py_str("Whatever came with it sets out what to return and what to keep; the office named on the paper will confirm against the reference")])), (py_tuple(vec![py_str("get_out"), py_str("commitment")]), py_tuple(vec![py_str("The way out is written in the terms you were given: the notice period, whether it has to be in writing, and the date it renews on - and a written notice sent to the address on that paperwork is what they have to answer"), py_str("It comes down to the wording you signed: how much notice it wants, whether it renews itself, and who the notice has to go to. Notice in writing, sent to the address on the paperwork, is the version that gets an answer")])), (py_tuple(vec![py_str("exact"), py_str("appliance")]), py_tuple(vec![py_str("The plate on it - inside the door or on the back - gives the model and the part number, and that is what the maker's page is searched by"), py_str("The model and serial on the plate are what any parts page wants, and they make the answer exact rather than near")])), (py_tuple(vec![py_str("first_step"), py_str("kit")]), py_tuple(vec![py_str("The instruction sheet in the box and the maker's page for it between them cover what it needs and what to do when it will not - and the sheet is the one that came with your copy"), py_str("Start with what came with it: the sheet and the parts list say what it should do and what is missing, and the maker's page for that exact one says the rest")])), (py_tuple(vec![py_str("first_step"), py_str("appliance")]), py_tuple(vec![py_str("The plate on it - inside the door or on the back - gives the model and the serial, and the maker's page for that model has the fault codes, the parts and who services it"), py_str("Whatever the question is, the model number on the plate is the way in: the maker's page for it, or the engineer on the phone, will take that and work from there")])), (py_tuple(vec![py_str("first_step"), py_str("fixture")]), py_tuple(vec![py_str("The part that goes on those is the cheap one - a washer, a cartridge, a seal or a valve, a few pounds rather than a new unit - and the make settles which: it is stamped on the body of it or under the cap, and the maker's page for that exact one has the part number and the video for the job"), py_str("Most of what goes wrong on one is a seal, a washer or a valve, and none of them is dear - take the make and model off it, on the body or under the handle, and the maker's page for that model shows the part and how the head comes off")])), (py_tuple(vec![py_str("tools"), py_str("fixture")]), py_tuple(vec![py_str("It is a spanner job rather than a kit: the maker's page for the model names the sizes, and a plumber's merchant has the washer or the cartridge"), py_str("The fittings come with the unit, and the maker's page for that model says which sizes you need for it")])), (py_tuple(vec![py_str("how_long"), py_str("fixture")]), py_tuple(vec![py_str("With the part in hand it is a short job - the maker's page for the model gives the time, and it is usually the inside of an hour for a washer or a cartridge"), py_str("The maker's page for that model says how long it takes, and most of that is the water off and the old part out")]))]));
    s_KINDS(py_tuple(vec![py_tuple(vec![g_A_FIXTURE(), py_str("fixture")]), py_tuple(vec![g_A_KIT(), py_str("kit")]), py_tuple(vec![g_AN_APPLIANCE(), py_str("appliance")]), py_tuple(vec![g_PAPERWORK(), py_str("paperwork")]), py_tuple(vec![g_A_COMMITMENT(), py_str("commitment")])]));
    s_ASKS(py_tuple(vec![py_tuple(vec![g_ASK_GET_OUT(), py_str("get_out")]), py_tuple(vec![g_ASK_TOOLS(), py_str("tools")]), py_tuple(vec![g_ASK_HOW_LONG(), py_str("how_long")]), py_tuple(vec![g_ASK_SIZE(), py_str("size")]), py_tuple(vec![g_ASK_BE_IN(), py_str("be_in")]), py_tuple(vec![g_ASK_COVERED(), py_str("covered")]), py_tuple(vec![g_ASK_SEND_BACK(), py_str("send_back")]), py_tuple(vec![g_ASK_EXACT(), py_str("exact")])]));
    s_OPENER(py_tuple(vec![py_str("I can't see {S} from here, sorry"), py_str("I haven't got {S} in front of me, sorry"), py_str("There is no way for me to check {S} from here, sorry")]));
    s_OPENER_TERMS(py_tuple(vec![py_str("I can't see the terms on {S} from here, sorry"), py_str("I haven't got the small print for {S}, sorry"), py_str("I can't check what {S} says about getting out, sorry")]));
    s_QUESTION(py_dict(vec![(py_str("tools"), py_tuple(vec![py_str("What does your sheet list?"), py_str("What came in the box with it?")])), (py_str("how_long"), py_tuple(vec![py_str("What does it say on the sheet?"), py_str("Does the sheet give a time for the job?")])), (py_str("size"), py_tuple(vec![py_str("What have you got to fit it in?"), py_str("What do the dimensions say on yours?")])), (py_str("be_in"), py_tuple(vec![py_str("What does the tracking say?"), py_str("Did they give you a time slot?")])), (py_str("covered"), py_tuple(vec![py_str("What does the paperwork say it covers?"), py_str("Do you have the reference to hand?")])), (py_str("send_back"), py_tuple(vec![py_str("What does it ask you to send?"), py_str("Have you got the reference on it?")])), (py_str("get_out"), py_tuple(vec![py_str("How much notice does it ask for?"), py_str("Do you still have the terms it came with?")])), (py_str("exact"), py_tuple(vec![py_str("What is the model number on it?"), py_str("Can you read the plate on it?")])), (py_str("first_step"), py_tuple(vec![py_str("What does the plate say on yours?"), py_str("Have you got the sheet that came with it?")]))]));
    s_QUESTION_KIND(py_dict(vec![(py_tuple(vec![py_str("first_step"), py_str("kit")]), py_tuple(vec![py_str("Have you got the sheet that came with it?"), py_str("Is the sheet it came with still about?")])), (py_tuple(vec![py_str("tools"), py_str("appliance")]), py_tuple(vec![py_str("What does the plate say your model is?"), py_str("Do the fixings come with it or are they extra?")])), (py_tuple(vec![py_str("first_step"), py_str("fixture")]), py_tuple(vec![py_str("Do you know the make of it?"), py_str("Can you see the make on it?")])), (py_tuple(vec![py_str("tools"), py_str("fixture")]), py_tuple(vec![py_str("Do you know the make of it?"), py_str("Is the make on it anywhere?")]))]));
    s_THEIR_OWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("the"), py_str("this"), py_str("that"), py_str("these"), py_str("those")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_RECEIPT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:got|bought|ordered|received|arrived|came|delivered|picked\\s+up|sent\\s+(?:me|us)|turned\\s+up|was\\s+delivered|was\\s+sent|it's\\s+here|its\\s+here|came\\s+with)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_JOB_ON_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bput(?:ting)?\\s+(?:\\w+\\s+){0,4}(?:up|together)\\b|\\bassembl\\w*|\\binstall\\w*|\\bbuild\\w*|\\berecting\\b|\\bfitting\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_COPY_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("be_in"), py_str("send_back"), py_str("exact")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DETERMINER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(my|the|our|your|this|that|a|an|his|her|their)\\s+$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn reads_as_a_report(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is telling the bot about something rather than asking about it.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_REPORT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("The line as whole words, for the tests that only need to know one is present.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9'-]+")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_crowd(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how busy somewhere is - the crowd half of `reads_as_busy`.\n\n    Kept apart from the bare \"how long is the wait\", because a wait is also the word for a\n    service's own delay (\"how long is the wait for a hip replacement\") and a card answers that.\n    A crowd adjective is never that: \"is the tip busy today\" is about today's queue and nothing\n    else.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CROWD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUSY_ASKS(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    Py::None
}
pub fn reads_as_busy(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how busy somewhere is, or how long the wait is.\n\n    The ask keeps its kind even where the bot's own line already names a page: a page tells the\n    person where today's figure is, and what they asked about was the next hour. So a hedge that\n    says \"Wikipedia has it\" still gives way to the busier-or-not reading.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUSY(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUSY_ASKS(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    Py::None
}
pub fn _a_place_a_line_back(args: &[Py]) -> Py {
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
    let mut side: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The place named in the last thing either side said, if the thread has one in it.\n\n    \"is the beach heaving\" then \"how long is the wait\" is one question with the place already in\n    it, so the second line gets the place's answer and is not asked which place it means.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("bot")]));
    for __item in __iter2 {
    side = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(side.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLACE_IN_A_TURN(), "search", args))(&__args) } } }) {
    return said.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The lines for a question whose answer lives somewhere else, or None.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_report(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUSY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BUSY_ASKS(), "search", args))(&__args) } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_SUBJECT(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&said.clone(), &g_A_PLACE()); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLACE_IN_A_TURN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_AND_E(), "search", args))(&__args) } } } } }) {
    return g_BUSY_LINES();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_place_a_line_back(args))(&__args) }) {
    return g_BUSY_LINES();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WAIT_NOUN(), "search", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }) {
    return g_BUSY_NO_PLACE_AND_CONTEXT();
    }
    return g_BUSY_NO_PLACE();
    }
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOURS_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&said.clone(), &g_A_PLACE()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("it"))) } } } }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LATE_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_bitand(&said.clone(), &g_A_PLACE()) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_TO_OPEN(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_NOT_A_VENUE()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_THE_PEOPLE()))) } } } }) {
    return g_HOURS();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KICK_OFF(), "search", args))(&__args) }) {
    return g_KICK_OFF_LINES();
    }
    if py_truthy(&{ let __b = op_bitand(&said.clone(), &op_bitor(&g_ON_WORDS(), &g_ON_EVENT())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ON_ASK(), "search", args))(&__args) } } }) {
    return g_WHATS_ON_LINES();
    }
    if py_truthy(&{ let __b = op_bitand(&said.clone(), &g_ON_WORDS()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_IS_GOOD(), "search", args))(&__args) } } }) {
    return g_WHATS_ON_LINES();
    }
    if py_truthy(&{ let __b = { let __b = op_bitand(&said.clone(), &g_TRAVEL()); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ROAD(), "search", args))(&__args) } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_TO(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_TRAVEL_CLAIM()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_OWN_ONLINE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STORY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPENS_ASKING(), "search", args))(&__args) })) } })); if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&said.clone(), &g_STATUS()); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STATUS_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } } } } } } } } } }) {
    return g_TRAVEL_LINES();
    }
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRICE_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_bitand(&said.clone(), &g_PRICE_THING()) } }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FREE_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_FREE_COMPOUND()))) } } } }) {
    return g_PRICE_LINES();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AVAIL(), "search", args))(&__args) }) {
    return g_AVAIL_LINES();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LOCAL_AREA(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LOCAL_ASK(), "search", args))(&__args) } } }) {
    return g_LOCAL_LINES();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DELIVER(), "search", args))(&__args) }) {
    return g_DELIVER_LINES();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TERM_DATES(), "search", args))(&__args) }) {
    return g_TERM_LINES();
    }
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEARBY_AREA(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_LOCAL(), "search", args))(&__args) } } }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEAREST(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_bitand(&said.clone(), &g_NEARBY_SERVICE()) } } } }) {
    return g_NEARBY_LINES();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OFFICIAL_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_bitand(&said.clone(), &g_PET()); if !py_truthy(&__b) { __b } else { op_bitand(&said.clone(), &g_DIED()) } })) } }) {
    if py_truthy(&op_bitand(&said.clone(), &g_OFFICIAL_THING())) {
    return g_OFFICIAL_LINES();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULES_ASK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&said.clone(), &g_GAME()))); if !py_truthy(&__b) { __b } else { op_sub(&said.clone(), &g_ASK_STOP()) } } } }) {
    return g_RULES_LINES();
    }
    }
    return Py::None;
    Py::None
}
pub fn pool_over_a_receipt(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The lines for a line the composer would otherwise meet with only the flat receipt.\n\n    Kept apart from `pool` because the reading is deliberately looser: the only evidence of a\n    question is a status word spent on a named road or line. It is consulted only where the reply\n    is already the neutral receipt, so every pool above keeps its turn.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_STATUS_SPENT_ON(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_report(args))(&__args) }) {
    return Py::None;
    }
    bare = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "startswith", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::Int(4), &Py::None, &Py::None), "strip", args))(&__args) } } else { text.clone() };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pool(args))(&__args) };
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_THEIR_STATUS(), "search", args))(&__args) }) {
    return Py::None;
    }
    return g_TRAVEL_LINES();
    Py::None
}
pub fn more_about_the_journey(args: &[Py]) -> Py {
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
    let mut _kind: Py = Py::None;
    let mut _phrases: Py = Py::None;
    let mut last: Py = Py::None;
    let mut options: Py = Py::None;
    let mut t: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The travel answer again, for a follow-up to a turn findout just answered.\n\n    Only the journey: hours, prices and the rest of the kinds have a follow-up that is a new\n    question about the same place, and the pool for the previous kind would answer the wrong\n    one. The bot's own last line is the test - if it came from somewhere else, this stands down.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FOLLOW_UP(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BACK_REFERENCE(), "search", args))(&__args) })) } } } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(py_contains(&g_TRAVEL_LINES(), &last.clone()))) {
    return g_TRAVEL_LINES();
    }
    let mut fallback_data: Py = crate::compose::fallback_data::module_obj();
    let __iter1 = py_iter_or_raise(&gattr(&fallback_data.clone(), "LIVE_KINDS"));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _kind = __unpk2[0].clone();
    _phrases = __unpk2[1].clone();
    options = __unpk2[2].clone();
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&options.clone(), &last.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return Py::None;
    Py::None
}
pub fn says_where_to_look(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply already names a place that holds what the person asked for.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLACE_NAMED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOLDS_IT(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _they_have_it(args: &[Py]) -> Py {
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
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut determiner: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "determiner") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: determiner"))) };
    let _ = py_str("True when the line says the thing is theirs to look at, or the ask is about their own copy.");
    if py_truthy(&Py::Bool(py_contains(&g_THEIR_COPY_ASKS(), &ask.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_THEIR_OWN(), &determiner.clone()))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RECEIPT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_JOB_ON_IT(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _the_thing_named(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut best: Py = Py::None;
    let mut determiner: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut papers: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said_as: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(how the person said the thing, its kind) or None.\n\n    Scans for a whole word from one of the kind lists rather than for a noun phrase, so a line\n    with no such thing in it gets nothing at all. The longest match wins, and the determiner in\n    front of it is kept so it reads back as \"the shed\" or \"a van\" rather than as \"shed\".\n    ");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter3 {
    match_ = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&best.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    best = py_tuple(vec![word.clone(), kind.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_is(&best.clone(), &Py::None))) {
    papers = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PAPERWORK(), "finditer", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&papers.clone()) {
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&papers.clone(), &Py::Int(0)), "group", args))(&__args) };
    best = py_tuple(vec![word.clone(), py_str("paperwork"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&papers.clone(), &Py::Int(0)), "start", args))(&__args) }]);
    }
    }
    if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&best.clone(), 3, None);
    word = __unpk4[0].clone();
    kind = __unpk4[1].clone();
    start = __unpk4[2].clone();
    before = slice_or_raise(&said.clone(), &Py::None, &start.clone(), &Py::None);
    said_as = op_modulo(&py_str("the %s"), &word.clone());
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DETERMINER(), "search", args))(&__args) };
    determiner = if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) } } else { Py::None };
    if py_truthy(&found.clone()) {
    said_as = op_modulo(&py_str("%s %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your"))]), "get", args))(&__args) }, word.clone()]));
    }
    return py_tuple(vec![said_as.clone(), kind.clone(), determiner.clone()]);
    Py::None
}
pub fn _the_thing_in_the_thread(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the last few lines were about, when this line names none of its own.\n\n    \"we ordered a shed, it was meant to come on the 14th\" / \"do i need to be in for it\": the\n    second line has no thing in it, and the answer is still on the shed's confirmation. Only the\n    person's own lines are read, the nearest first, and only for a thing of a known kind - a line\n    about a delivery two turns back is not a licence to answer a different question with a label.\n    ");
    turns = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) } } else { py_str("") }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_named(args))(&__args) };
    if py_truthy(&found.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&found.clone(), &Py::Int(0)), geti(&found.clone(), &Py::Int(1))]), 2, None);
    word = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:my|our|your|his|her|their|the|a|an)\\s+")); __args.push(py_str("")); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return py_tuple(vec![op_add(&py_str("the "), &word.clone()), kind.clone(), py_str("the")]);
    }
    let __aug3 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug3;
    if py_truthy(&op_ge(&seen.clone(), &Py::Int(2))) {
    return Py::None;
    }
    }
    return Py::None;
    Py::None
}
pub fn on_the_thing(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut ask: Py = Py::None;
    let mut carrier: Py = Py::None;
    let mut determiner: Py = Py::None;
    let mut from_the_thread: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut opener_pool: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut question: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A partial answer that names where the rest lives, or None to keep the hedge.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_HERE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THE_CONVERSATION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_LOOKUP_BUT_A_REQUEST(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_JUDGEMENT_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLAN_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COOKING_LINE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_READING_JUST_GIVEN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WHY_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_METHOD_OR_ADVICE_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DECISION_ABOUT_GOING_ON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_NOW_OR_NEVER_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ABOUT_THE_VISIT(), "search", args))(&__args) } } } } } } } } } } } } } } } } } } } } } } } } }) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_named(args))(&__args) };
    from_the_thread = Py::Bool(py_is(&thing.clone(), &Py::None));
    thing = { let __b = thing.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_in_the_thread(args))(&__args) } } };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&thing.clone(), 3, None);
    word = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    determiner = __unpk1[2].clone();
    ask = Py::None;
    let __iter2 = py_iter_or_raise(&g_ASKS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk3[0].clone();
    name = __unpk3[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    ask = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&ask.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&from_the_thread.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ON_THE_PLATE(), "search", args))(&__args) }) {
    return Py::None;
    }
    ask = py_str("first_step");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&from_the_thread.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(ask.clone()); __args.push(determiner.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _they_have_it(args))(&__args) })) } }) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![ask.clone(), kind.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CARRIERS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    opener_pool = if py_truthy(&Py::Bool(py_eq(&ask.clone(), &py_str("get_out")))) { g_OPENER_TERMS() } else { g_OPENER() };
    opener = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("S".to_string(), word.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opener_pool.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }, "format", args))(&__args) };
    carrier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    question = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![ask.clone(), kind.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUESTION_KIND(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { geti(&g_QUESTION(), &ask.clone()) } }); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    return op_modulo(&py_str("%s. %s. %s"), &py_tuple(vec![opener.clone(), carrier.clone(), question.clone()]));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.findout".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_APPLIANCE" => Some(g_AN_APPLIANCE()),
            "ASKED" => Some(g_ASKED()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_LOCAL" => Some(g_ASKS_LOCAL()),
            "ASK_BE_IN" => Some(g_ASK_BE_IN()),
            "ASK_COVERED" => Some(g_ASK_COVERED()),
            "ASK_EXACT" => Some(g_ASK_EXACT()),
            "ASK_GET_OUT" => Some(g_ASK_GET_OUT()),
            "ASK_HOW_LONG" => Some(g_ASK_HOW_LONG()),
            "ASK_SEND_BACK" => Some(g_ASK_SEND_BACK()),
            "ASK_SIZE" => Some(g_ASK_SIZE()),
            "ASK_STOP" => Some(g_ASK_STOP()),
            "ASK_TOOLS" => Some(g_ASK_TOOLS()),
            "AVAIL" => Some(g_AVAIL()),
            "AVAIL_LINES" => Some(g_AVAIL_LINES()),
            "A_AND_E" => Some(g_A_AND_E()),
            "A_BACK_REFERENCE" => Some(g_A_BACK_REFERENCE()),
            "A_COMMITMENT" => Some(g_A_COMMITMENT()),
            "A_COOKING_LINE" => Some(g_A_COOKING_LINE()),
            "A_DECISION_ABOUT_GOING_ON" => Some(g_A_DECISION_ABOUT_GOING_ON()),
            "A_FIXTURE" => Some(g_A_FIXTURE()),
            "A_FOLLOW_UP" => Some(g_A_FOLLOW_UP()),
            "A_JOB_ON_IT" => Some(g_A_JOB_ON_IT()),
            "A_JUDGEMENT_ASK" => Some(g_A_JUDGEMENT_ASK()),
            "A_KIT" => Some(g_A_KIT()),
            "A_METHOD_OR_ADVICE_ASK" => Some(g_A_METHOD_OR_ADVICE_ASK()),
            "A_NOW_OR_NEVER_ASK" => Some(g_A_NOW_OR_NEVER_ASK()),
            "A_PERSON_SUBJECT" => Some(g_A_PERSON_SUBJECT()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_PLACE_IN_A_TURN" => Some(g_A_PLACE_IN_A_TURN()),
            "A_PLACE_NAMED" => Some(g_A_PLACE_NAMED()),
            "A_PLAN_ASK" => Some(g_A_PLAN_ASK()),
            "A_READING_JUST_GIVEN" => Some(g_A_READING_JUST_GIVEN()),
            "A_RECEIPT" => Some(g_A_RECEIPT()),
            "A_REPORT" => Some(g_A_REPORT()),
            "A_STATUS_SPENT_ON" => Some(g_A_STATUS_SPENT_ON()),
            "A_WAIT_NOUN" => Some(g_A_WAIT_NOUN()),
            "A_WHY_ASK" => Some(g_A_WHY_ASK()),
            "BUSY" => Some(g_BUSY()),
            "BUSY_ASKS" => Some(g_BUSY_ASKS()),
            "BUSY_LINES" => Some(g_BUSY_LINES()),
            "BUSY_NO_PLACE" => Some(g_BUSY_NO_PLACE()),
            "BUSY_NO_PLACE_AND_CONTEXT" => Some(g_BUSY_NO_PLACE_AND_CONTEXT()),
            "CARRIERS" => Some(g_CARRIERS()),
            "CROWD" => Some(g_CROWD()),
            "DELIVER" => Some(g_DELIVER()),
            "DELIVER_LINES" => Some(g_DELIVER_LINES()),
            "DIED" => Some(g_DIED()),
            "FREE_ASK" => Some(g_FREE_ASK()),
            "FREE_COMPOUND" => Some(g_FREE_COMPOUND()),
            "GAME" => Some(g_GAME()),
            "HOLDS_IT" => Some(g_HOLDS_IT()),
            "HOURS" => Some(g_HOURS()),
            "HOURS_ASK" => Some(g_HOURS_ASK()),
            "HOW_TO" => Some(g_HOW_TO()),
            "HOW_TO_OPEN" => Some(g_HOW_TO_OPEN()),
            "KICK_OFF" => Some(g_KICK_OFF()),
            "KICK_OFF_LINES" => Some(g_KICK_OFF_LINES()),
            "KINDS" => Some(g_KINDS()),
            "LATE_ASK" => Some(g_LATE_ASK()),
            "LOCAL_AREA" => Some(g_LOCAL_AREA()),
            "LOCAL_ASK" => Some(g_LOCAL_ASK()),
            "LOCAL_LINES" => Some(g_LOCAL_LINES()),
            "NEARBY_AREA" => Some(g_NEARBY_AREA()),
            "NEARBY_LINES" => Some(g_NEARBY_LINES()),
            "NEARBY_SERVICE" => Some(g_NEARBY_SERVICE()),
            "NEAREST" => Some(g_NEAREST()),
            "NOT_ABOUT_THE_CONVERSATION" => Some(g_NOT_ABOUT_THE_CONVERSATION()),
            "NOT_ABOUT_THE_VISIT" => Some(g_NOT_ABOUT_THE_VISIT()),
            "NOT_A_LOOKUP_BUT_A_REQUEST" => Some(g_NOT_A_LOOKUP_BUT_A_REQUEST()),
            "NOT_A_VENUE" => Some(g_NOT_A_VENUE()),
            "NOT_HERE" => Some(g_NOT_HERE()),
            "NOT_ON_THE_PLATE" => Some(g_NOT_ON_THE_PLATE()),
            "NOT_THEIR_STATUS" => Some(g_NOT_THEIR_STATUS()),
            "OFFICIAL_ASK" => Some(g_OFFICIAL_ASK()),
            "OFFICIAL_LINES" => Some(g_OFFICIAL_LINES()),
            "OFFICIAL_THING" => Some(g_OFFICIAL_THING()),
            "ON_ASK" => Some(g_ON_ASK()),
            "ON_EVENT" => Some(g_ON_EVENT()),
            "ON_WORDS" => Some(g_ON_WORDS()),
            "OPENER" => Some(g_OPENER()),
            "OPENER_TERMS" => Some(g_OPENER_TERMS()),
            "OPENS_ASKING" => Some(g_OPENS_ASKING()),
            "PAPERWORK" => Some(g_PAPERWORK()),
            "PET" => Some(g_PET()),
            "PRICE_ASK" => Some(g_PRICE_ASK()),
            "PRICE_LINES" => Some(g_PRICE_LINES()),
            "PRICE_THING" => Some(g_PRICE_THING()),
            "QUESTION" => Some(g_QUESTION()),
            "QUESTION_KIND" => Some(g_QUESTION_KIND()),
            "ROAD" => Some(g_ROAD()),
            "RULES_ASK" => Some(g_RULES_ASK()),
            "RULES_LINES" => Some(g_RULES_LINES()),
            "STATUS" => Some(g_STATUS()),
            "STATUS_PHRASES" => Some(g_STATUS_PHRASES()),
            "STORY" => Some(g_STORY()),
            "TERM_DATES" => Some(g_TERM_DATES()),
            "TERM_LINES" => Some(g_TERM_LINES()),
            "THEIR_COPY_ASKS" => Some(g_THEIR_COPY_ASKS()),
            "THEIR_OWN" => Some(g_THEIR_OWN()),
            "THEIR_OWN_ONLINE" => Some(g_THEIR_OWN_ONLINE()),
            "THE_PEOPLE" => Some(g_THE_PEOPLE()),
            "TRAVEL" => Some(g_TRAVEL()),
            "TRAVEL_CLAIM" => Some(g_TRAVEL_CLAIM()),
            "TRAVEL_LINES" => Some(g_TRAVEL_LINES()),
            "WHATS_ON_LINES" => Some(g_WHATS_ON_LINES()),
            "WHAT_IS_GOOD" => Some(g_WHAT_IS_GOOD()),
            "_DETERMINER" => Some(g__DETERMINER()),
            "_a_place_a_line_back" => Some(py_func("_a_place_a_line_back", |args: &[Py]| _a_place_a_line_back(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_the_thing_in_the_thread" => Some(py_func("_the_thing_in_the_thread", |args: &[Py]| _the_thing_in_the_thread(args))),
            "_the_thing_named" => Some(py_func("_the_thing_named", |args: &[Py]| _the_thing_named(args))),
            "_they_have_it" => Some(py_func("_they_have_it", |args: &[Py]| _they_have_it(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "more_about_the_journey" => Some(py_func("more_about_the_journey", |args: &[Py]| more_about_the_journey(args))),
            "on_the_thing" => Some(py_func("on_the_thing", |args: &[Py]| on_the_thing(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "pool_over_a_receipt" => Some(py_func("pool_over_a_receipt", |args: &[Py]| pool_over_a_receipt(args))),
            "reads_as_a_report" => Some(py_func("reads_as_a_report", |args: &[Py]| reads_as_a_report(args))),
            "reads_as_busy" => Some(py_func("reads_as_busy", |args: &[Py]| reads_as_busy(args))),
            "reads_as_crowd" => Some(py_func("reads_as_crowd", |args: &[Py]| reads_as_crowd(args))),
            "says_where_to_look" => Some(py_func("says_where_to_look", |args: &[Py]| says_where_to_look(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_APPLIANCE" => s_AN_APPLIANCE(v),
            "ASKED" => s_ASKED(v),
            "ASKS" => s_ASKS(v),
            "ASKS_LOCAL" => s_ASKS_LOCAL(v),
            "ASK_BE_IN" => s_ASK_BE_IN(v),
            "ASK_COVERED" => s_ASK_COVERED(v),
            "ASK_EXACT" => s_ASK_EXACT(v),
            "ASK_GET_OUT" => s_ASK_GET_OUT(v),
            "ASK_HOW_LONG" => s_ASK_HOW_LONG(v),
            "ASK_SEND_BACK" => s_ASK_SEND_BACK(v),
            "ASK_SIZE" => s_ASK_SIZE(v),
            "ASK_STOP" => s_ASK_STOP(v),
            "ASK_TOOLS" => s_ASK_TOOLS(v),
            "AVAIL" => s_AVAIL(v),
            "AVAIL_LINES" => s_AVAIL_LINES(v),
            "A_AND_E" => s_A_AND_E(v),
            "A_BACK_REFERENCE" => s_A_BACK_REFERENCE(v),
            "A_COMMITMENT" => s_A_COMMITMENT(v),
            "A_COOKING_LINE" => s_A_COOKING_LINE(v),
            "A_DECISION_ABOUT_GOING_ON" => s_A_DECISION_ABOUT_GOING_ON(v),
            "A_FIXTURE" => s_A_FIXTURE(v),
            "A_FOLLOW_UP" => s_A_FOLLOW_UP(v),
            "A_JOB_ON_IT" => s_A_JOB_ON_IT(v),
            "A_JUDGEMENT_ASK" => s_A_JUDGEMENT_ASK(v),
            "A_KIT" => s_A_KIT(v),
            "A_METHOD_OR_ADVICE_ASK" => s_A_METHOD_OR_ADVICE_ASK(v),
            "A_NOW_OR_NEVER_ASK" => s_A_NOW_OR_NEVER_ASK(v),
            "A_PERSON_SUBJECT" => s_A_PERSON_SUBJECT(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_PLACE_IN_A_TURN" => s_A_PLACE_IN_A_TURN(v),
            "A_PLACE_NAMED" => s_A_PLACE_NAMED(v),
            "A_PLAN_ASK" => s_A_PLAN_ASK(v),
            "A_READING_JUST_GIVEN" => s_A_READING_JUST_GIVEN(v),
            "A_RECEIPT" => s_A_RECEIPT(v),
            "A_REPORT" => s_A_REPORT(v),
            "A_STATUS_SPENT_ON" => s_A_STATUS_SPENT_ON(v),
            "A_WAIT_NOUN" => s_A_WAIT_NOUN(v),
            "A_WHY_ASK" => s_A_WHY_ASK(v),
            "BUSY" => s_BUSY(v),
            "BUSY_ASKS" => s_BUSY_ASKS(v),
            "BUSY_LINES" => s_BUSY_LINES(v),
            "BUSY_NO_PLACE" => s_BUSY_NO_PLACE(v),
            "BUSY_NO_PLACE_AND_CONTEXT" => s_BUSY_NO_PLACE_AND_CONTEXT(v),
            "CARRIERS" => s_CARRIERS(v),
            "CROWD" => s_CROWD(v),
            "DELIVER" => s_DELIVER(v),
            "DELIVER_LINES" => s_DELIVER_LINES(v),
            "DIED" => s_DIED(v),
            "FREE_ASK" => s_FREE_ASK(v),
            "FREE_COMPOUND" => s_FREE_COMPOUND(v),
            "GAME" => s_GAME(v),
            "HOLDS_IT" => s_HOLDS_IT(v),
            "HOURS" => s_HOURS(v),
            "HOURS_ASK" => s_HOURS_ASK(v),
            "HOW_TO" => s_HOW_TO(v),
            "HOW_TO_OPEN" => s_HOW_TO_OPEN(v),
            "KICK_OFF" => s_KICK_OFF(v),
            "KICK_OFF_LINES" => s_KICK_OFF_LINES(v),
            "KINDS" => s_KINDS(v),
            "LATE_ASK" => s_LATE_ASK(v),
            "LOCAL_AREA" => s_LOCAL_AREA(v),
            "LOCAL_ASK" => s_LOCAL_ASK(v),
            "LOCAL_LINES" => s_LOCAL_LINES(v),
            "NEARBY_AREA" => s_NEARBY_AREA(v),
            "NEARBY_LINES" => s_NEARBY_LINES(v),
            "NEARBY_SERVICE" => s_NEARBY_SERVICE(v),
            "NEAREST" => s_NEAREST(v),
            "NOT_ABOUT_THE_CONVERSATION" => s_NOT_ABOUT_THE_CONVERSATION(v),
            "NOT_ABOUT_THE_VISIT" => s_NOT_ABOUT_THE_VISIT(v),
            "NOT_A_LOOKUP_BUT_A_REQUEST" => s_NOT_A_LOOKUP_BUT_A_REQUEST(v),
            "NOT_A_VENUE" => s_NOT_A_VENUE(v),
            "NOT_HERE" => s_NOT_HERE(v),
            "NOT_ON_THE_PLATE" => s_NOT_ON_THE_PLATE(v),
            "NOT_THEIR_STATUS" => s_NOT_THEIR_STATUS(v),
            "OFFICIAL_ASK" => s_OFFICIAL_ASK(v),
            "OFFICIAL_LINES" => s_OFFICIAL_LINES(v),
            "OFFICIAL_THING" => s_OFFICIAL_THING(v),
            "ON_ASK" => s_ON_ASK(v),
            "ON_EVENT" => s_ON_EVENT(v),
            "ON_WORDS" => s_ON_WORDS(v),
            "OPENER" => s_OPENER(v),
            "OPENER_TERMS" => s_OPENER_TERMS(v),
            "OPENS_ASKING" => s_OPENS_ASKING(v),
            "PAPERWORK" => s_PAPERWORK(v),
            "PET" => s_PET(v),
            "PRICE_ASK" => s_PRICE_ASK(v),
            "PRICE_LINES" => s_PRICE_LINES(v),
            "PRICE_THING" => s_PRICE_THING(v),
            "QUESTION" => s_QUESTION(v),
            "QUESTION_KIND" => s_QUESTION_KIND(v),
            "ROAD" => s_ROAD(v),
            "RULES_ASK" => s_RULES_ASK(v),
            "RULES_LINES" => s_RULES_LINES(v),
            "STATUS" => s_STATUS(v),
            "STATUS_PHRASES" => s_STATUS_PHRASES(v),
            "STORY" => s_STORY(v),
            "TERM_DATES" => s_TERM_DATES(v),
            "TERM_LINES" => s_TERM_LINES(v),
            "THEIR_COPY_ASKS" => s_THEIR_COPY_ASKS(v),
            "THEIR_OWN" => s_THEIR_OWN(v),
            "THEIR_OWN_ONLINE" => s_THEIR_OWN_ONLINE(v),
            "THE_PEOPLE" => s_THE_PEOPLE(v),
            "TRAVEL" => s_TRAVEL(v),
            "TRAVEL_CLAIM" => s_TRAVEL_CLAIM(v),
            "TRAVEL_LINES" => s_TRAVEL_LINES(v),
            "WHATS_ON_LINES" => s_WHATS_ON_LINES(v),
            "WHAT_IS_GOOD" => s_WHAT_IS_GOOD(v),
            "_DETERMINER" => s__DETERMINER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.findout", module_obj);
    pyrt::xrefs::reg_fn("compose.findout", "reads_as_a_report", py_func("reads_as_a_report", |args: &[Py]| reads_as_a_report(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.findout", "reads_as_crowd", py_func("reads_as_crowd", |args: &[Py]| reads_as_crowd(args)));
    pyrt::xrefs::reg_fn("compose.findout", "reads_as_busy", py_func("reads_as_busy", |args: &[Py]| reads_as_busy(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_a_place_a_line_back", py_func("_a_place_a_line_back", |args: &[Py]| _a_place_a_line_back(args)));
    pyrt::xrefs::reg_fn("compose.findout", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_fn("compose.findout", "pool_over_a_receipt", py_func("pool_over_a_receipt", |args: &[Py]| pool_over_a_receipt(args)));
    pyrt::xrefs::reg_fn("compose.findout", "more_about_the_journey", py_func("more_about_the_journey", |args: &[Py]| more_about_the_journey(args)));
    pyrt::xrefs::reg_fn("compose.findout", "says_where_to_look", py_func("says_where_to_look", |args: &[Py]| says_where_to_look(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_they_have_it", py_func("_they_have_it", |args: &[Py]| _they_have_it(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_the_thing_named", py_func("_the_thing_named", |args: &[Py]| _the_thing_named(args)));
    pyrt::xrefs::reg_fn("compose.findout", "_the_thing_in_the_thread", py_func("_the_thing_in_the_thread", |args: &[Py]| _the_thing_in_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.findout", "on_the_thing", py_func("on_the_thing", |args: &[Py]| on_the_thing(args)));
    pyrt::xrefs::reg_access("compose.findout", "AN_APPLIANCE", g_AN_APPLIANCE, s_AN_APPLIANCE);
    pyrt::xrefs::reg_access("compose.findout", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("compose.findout", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.findout", "ASKS_LOCAL", g_ASKS_LOCAL, s_ASKS_LOCAL);
    pyrt::xrefs::reg_access("compose.findout", "ASK_BE_IN", g_ASK_BE_IN, s_ASK_BE_IN);
    pyrt::xrefs::reg_access("compose.findout", "ASK_COVERED", g_ASK_COVERED, s_ASK_COVERED);
    pyrt::xrefs::reg_access("compose.findout", "ASK_EXACT", g_ASK_EXACT, s_ASK_EXACT);
    pyrt::xrefs::reg_access("compose.findout", "ASK_GET_OUT", g_ASK_GET_OUT, s_ASK_GET_OUT);
    pyrt::xrefs::reg_access("compose.findout", "ASK_HOW_LONG", g_ASK_HOW_LONG, s_ASK_HOW_LONG);
    pyrt::xrefs::reg_access("compose.findout", "ASK_SEND_BACK", g_ASK_SEND_BACK, s_ASK_SEND_BACK);
    pyrt::xrefs::reg_access("compose.findout", "ASK_SIZE", g_ASK_SIZE, s_ASK_SIZE);
    pyrt::xrefs::reg_access("compose.findout", "ASK_STOP", g_ASK_STOP, s_ASK_STOP);
    pyrt::xrefs::reg_access("compose.findout", "ASK_TOOLS", g_ASK_TOOLS, s_ASK_TOOLS);
    pyrt::xrefs::reg_access("compose.findout", "AVAIL", g_AVAIL, s_AVAIL);
    pyrt::xrefs::reg_access("compose.findout", "AVAIL_LINES", g_AVAIL_LINES, s_AVAIL_LINES);
    pyrt::xrefs::reg_access("compose.findout", "A_AND_E", g_A_AND_E, s_A_AND_E);
    pyrt::xrefs::reg_access("compose.findout", "A_BACK_REFERENCE", g_A_BACK_REFERENCE, s_A_BACK_REFERENCE);
    pyrt::xrefs::reg_access("compose.findout", "A_COMMITMENT", g_A_COMMITMENT, s_A_COMMITMENT);
    pyrt::xrefs::reg_access("compose.findout", "A_COOKING_LINE", g_A_COOKING_LINE, s_A_COOKING_LINE);
    pyrt::xrefs::reg_access("compose.findout", "A_DECISION_ABOUT_GOING_ON", g_A_DECISION_ABOUT_GOING_ON, s_A_DECISION_ABOUT_GOING_ON);
    pyrt::xrefs::reg_access("compose.findout", "A_FIXTURE", g_A_FIXTURE, s_A_FIXTURE);
    pyrt::xrefs::reg_access("compose.findout", "A_FOLLOW_UP", g_A_FOLLOW_UP, s_A_FOLLOW_UP);
    pyrt::xrefs::reg_access("compose.findout", "A_JOB_ON_IT", g_A_JOB_ON_IT, s_A_JOB_ON_IT);
    pyrt::xrefs::reg_access("compose.findout", "A_JUDGEMENT_ASK", g_A_JUDGEMENT_ASK, s_A_JUDGEMENT_ASK);
    pyrt::xrefs::reg_access("compose.findout", "A_KIT", g_A_KIT, s_A_KIT);
    pyrt::xrefs::reg_access("compose.findout", "A_METHOD_OR_ADVICE_ASK", g_A_METHOD_OR_ADVICE_ASK, s_A_METHOD_OR_ADVICE_ASK);
    pyrt::xrefs::reg_access("compose.findout", "A_NOW_OR_NEVER_ASK", g_A_NOW_OR_NEVER_ASK, s_A_NOW_OR_NEVER_ASK);
    pyrt::xrefs::reg_access("compose.findout", "A_PERSON_SUBJECT", g_A_PERSON_SUBJECT, s_A_PERSON_SUBJECT);
    pyrt::xrefs::reg_access("compose.findout", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("compose.findout", "A_PLACE_IN_A_TURN", g_A_PLACE_IN_A_TURN, s_A_PLACE_IN_A_TURN);
    pyrt::xrefs::reg_access("compose.findout", "A_PLACE_NAMED", g_A_PLACE_NAMED, s_A_PLACE_NAMED);
    pyrt::xrefs::reg_access("compose.findout", "A_PLAN_ASK", g_A_PLAN_ASK, s_A_PLAN_ASK);
    pyrt::xrefs::reg_access("compose.findout", "A_READING_JUST_GIVEN", g_A_READING_JUST_GIVEN, s_A_READING_JUST_GIVEN);
    pyrt::xrefs::reg_access("compose.findout", "A_RECEIPT", g_A_RECEIPT, s_A_RECEIPT);
    pyrt::xrefs::reg_access("compose.findout", "A_REPORT", g_A_REPORT, s_A_REPORT);
    pyrt::xrefs::reg_access("compose.findout", "A_STATUS_SPENT_ON", g_A_STATUS_SPENT_ON, s_A_STATUS_SPENT_ON);
    pyrt::xrefs::reg_access("compose.findout", "A_WAIT_NOUN", g_A_WAIT_NOUN, s_A_WAIT_NOUN);
    pyrt::xrefs::reg_access("compose.findout", "A_WHY_ASK", g_A_WHY_ASK, s_A_WHY_ASK);
    pyrt::xrefs::reg_access("compose.findout", "BUSY", g_BUSY, s_BUSY);
    pyrt::xrefs::reg_access("compose.findout", "BUSY_ASKS", g_BUSY_ASKS, s_BUSY_ASKS);
    pyrt::xrefs::reg_access("compose.findout", "BUSY_LINES", g_BUSY_LINES, s_BUSY_LINES);
    pyrt::xrefs::reg_access("compose.findout", "BUSY_NO_PLACE", g_BUSY_NO_PLACE, s_BUSY_NO_PLACE);
    pyrt::xrefs::reg_access("compose.findout", "BUSY_NO_PLACE_AND_CONTEXT", g_BUSY_NO_PLACE_AND_CONTEXT, s_BUSY_NO_PLACE_AND_CONTEXT);
    pyrt::xrefs::reg_access("compose.findout", "CARRIERS", g_CARRIERS, s_CARRIERS);
    pyrt::xrefs::reg_access("compose.findout", "CROWD", g_CROWD, s_CROWD);
    pyrt::xrefs::reg_access("compose.findout", "DELIVER", g_DELIVER, s_DELIVER);
    pyrt::xrefs::reg_access("compose.findout", "DELIVER_LINES", g_DELIVER_LINES, s_DELIVER_LINES);
    pyrt::xrefs::reg_access("compose.findout", "DIED", g_DIED, s_DIED);
    pyrt::xrefs::reg_access("compose.findout", "FREE_ASK", g_FREE_ASK, s_FREE_ASK);
    pyrt::xrefs::reg_access("compose.findout", "FREE_COMPOUND", g_FREE_COMPOUND, s_FREE_COMPOUND);
    pyrt::xrefs::reg_access("compose.findout", "GAME", g_GAME, s_GAME);
    pyrt::xrefs::reg_access("compose.findout", "HOLDS_IT", g_HOLDS_IT, s_HOLDS_IT);
    pyrt::xrefs::reg_access("compose.findout", "HOURS", g_HOURS, s_HOURS);
    pyrt::xrefs::reg_access("compose.findout", "HOURS_ASK", g_HOURS_ASK, s_HOURS_ASK);
    pyrt::xrefs::reg_access("compose.findout", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("compose.findout", "HOW_TO_OPEN", g_HOW_TO_OPEN, s_HOW_TO_OPEN);
    pyrt::xrefs::reg_access("compose.findout", "KICK_OFF", g_KICK_OFF, s_KICK_OFF);
    pyrt::xrefs::reg_access("compose.findout", "KICK_OFF_LINES", g_KICK_OFF_LINES, s_KICK_OFF_LINES);
    pyrt::xrefs::reg_access("compose.findout", "KINDS", g_KINDS, s_KINDS);
    pyrt::xrefs::reg_access("compose.findout", "LATE_ASK", g_LATE_ASK, s_LATE_ASK);
    pyrt::xrefs::reg_access("compose.findout", "LOCAL_AREA", g_LOCAL_AREA, s_LOCAL_AREA);
    pyrt::xrefs::reg_access("compose.findout", "LOCAL_ASK", g_LOCAL_ASK, s_LOCAL_ASK);
    pyrt::xrefs::reg_access("compose.findout", "LOCAL_LINES", g_LOCAL_LINES, s_LOCAL_LINES);
    pyrt::xrefs::reg_access("compose.findout", "NEARBY_AREA", g_NEARBY_AREA, s_NEARBY_AREA);
    pyrt::xrefs::reg_access("compose.findout", "NEARBY_LINES", g_NEARBY_LINES, s_NEARBY_LINES);
    pyrt::xrefs::reg_access("compose.findout", "NEARBY_SERVICE", g_NEARBY_SERVICE, s_NEARBY_SERVICE);
    pyrt::xrefs::reg_access("compose.findout", "NEAREST", g_NEAREST, s_NEAREST);
    pyrt::xrefs::reg_access("compose.findout", "NOT_ABOUT_THE_CONVERSATION", g_NOT_ABOUT_THE_CONVERSATION, s_NOT_ABOUT_THE_CONVERSATION);
    pyrt::xrefs::reg_access("compose.findout", "NOT_ABOUT_THE_VISIT", g_NOT_ABOUT_THE_VISIT, s_NOT_ABOUT_THE_VISIT);
    pyrt::xrefs::reg_access("compose.findout", "NOT_A_LOOKUP_BUT_A_REQUEST", g_NOT_A_LOOKUP_BUT_A_REQUEST, s_NOT_A_LOOKUP_BUT_A_REQUEST);
    pyrt::xrefs::reg_access("compose.findout", "NOT_A_VENUE", g_NOT_A_VENUE, s_NOT_A_VENUE);
    pyrt::xrefs::reg_access("compose.findout", "NOT_HERE", g_NOT_HERE, s_NOT_HERE);
    pyrt::xrefs::reg_access("compose.findout", "NOT_ON_THE_PLATE", g_NOT_ON_THE_PLATE, s_NOT_ON_THE_PLATE);
    pyrt::xrefs::reg_access("compose.findout", "NOT_THEIR_STATUS", g_NOT_THEIR_STATUS, s_NOT_THEIR_STATUS);
    pyrt::xrefs::reg_access("compose.findout", "OFFICIAL_ASK", g_OFFICIAL_ASK, s_OFFICIAL_ASK);
    pyrt::xrefs::reg_access("compose.findout", "OFFICIAL_LINES", g_OFFICIAL_LINES, s_OFFICIAL_LINES);
    pyrt::xrefs::reg_access("compose.findout", "OFFICIAL_THING", g_OFFICIAL_THING, s_OFFICIAL_THING);
    pyrt::xrefs::reg_access("compose.findout", "ON_ASK", g_ON_ASK, s_ON_ASK);
    pyrt::xrefs::reg_access("compose.findout", "ON_EVENT", g_ON_EVENT, s_ON_EVENT);
    pyrt::xrefs::reg_access("compose.findout", "ON_WORDS", g_ON_WORDS, s_ON_WORDS);
    pyrt::xrefs::reg_access("compose.findout", "OPENER", g_OPENER, s_OPENER);
    pyrt::xrefs::reg_access("compose.findout", "OPENER_TERMS", g_OPENER_TERMS, s_OPENER_TERMS);
    pyrt::xrefs::reg_access("compose.findout", "OPENS_ASKING", g_OPENS_ASKING, s_OPENS_ASKING);
    pyrt::xrefs::reg_access("compose.findout", "PAPERWORK", g_PAPERWORK, s_PAPERWORK);
    pyrt::xrefs::reg_access("compose.findout", "PET", g_PET, s_PET);
    pyrt::xrefs::reg_access("compose.findout", "PRICE_ASK", g_PRICE_ASK, s_PRICE_ASK);
    pyrt::xrefs::reg_access("compose.findout", "PRICE_LINES", g_PRICE_LINES, s_PRICE_LINES);
    pyrt::xrefs::reg_access("compose.findout", "PRICE_THING", g_PRICE_THING, s_PRICE_THING);
    pyrt::xrefs::reg_access("compose.findout", "QUESTION", g_QUESTION, s_QUESTION);
    pyrt::xrefs::reg_access("compose.findout", "QUESTION_KIND", g_QUESTION_KIND, s_QUESTION_KIND);
    pyrt::xrefs::reg_access("compose.findout", "ROAD", g_ROAD, s_ROAD);
    pyrt::xrefs::reg_access("compose.findout", "RULES_ASK", g_RULES_ASK, s_RULES_ASK);
    pyrt::xrefs::reg_access("compose.findout", "RULES_LINES", g_RULES_LINES, s_RULES_LINES);
    pyrt::xrefs::reg_access("compose.findout", "STATUS", g_STATUS, s_STATUS);
    pyrt::xrefs::reg_access("compose.findout", "STATUS_PHRASES", g_STATUS_PHRASES, s_STATUS_PHRASES);
    pyrt::xrefs::reg_access("compose.findout", "STORY", g_STORY, s_STORY);
    pyrt::xrefs::reg_access("compose.findout", "TERM_DATES", g_TERM_DATES, s_TERM_DATES);
    pyrt::xrefs::reg_access("compose.findout", "TERM_LINES", g_TERM_LINES, s_TERM_LINES);
    pyrt::xrefs::reg_access("compose.findout", "THEIR_COPY_ASKS", g_THEIR_COPY_ASKS, s_THEIR_COPY_ASKS);
    pyrt::xrefs::reg_access("compose.findout", "THEIR_OWN", g_THEIR_OWN, s_THEIR_OWN);
    pyrt::xrefs::reg_access("compose.findout", "THEIR_OWN_ONLINE", g_THEIR_OWN_ONLINE, s_THEIR_OWN_ONLINE);
    pyrt::xrefs::reg_access("compose.findout", "THE_PEOPLE", g_THE_PEOPLE, s_THE_PEOPLE);
    pyrt::xrefs::reg_access("compose.findout", "TRAVEL", g_TRAVEL, s_TRAVEL);
    pyrt::xrefs::reg_access("compose.findout", "TRAVEL_CLAIM", g_TRAVEL_CLAIM, s_TRAVEL_CLAIM);
    pyrt::xrefs::reg_access("compose.findout", "TRAVEL_LINES", g_TRAVEL_LINES, s_TRAVEL_LINES);
    pyrt::xrefs::reg_access("compose.findout", "WHATS_ON_LINES", g_WHATS_ON_LINES, s_WHATS_ON_LINES);
    pyrt::xrefs::reg_access("compose.findout", "WHAT_IS_GOOD", g_WHAT_IS_GOOD, s_WHAT_IS_GOOD);
    pyrt::xrefs::reg_access("compose.findout", "_DETERMINER", g__DETERMINER, s__DETERMINER);
}

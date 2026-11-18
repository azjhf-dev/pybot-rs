#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static AGREEMENT_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALLOWANCE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALLOWED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALLOWED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FACILITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLACE_HAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_ROOM_FACILITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CANCEL_TERMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARRYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMPLAINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRACT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_AS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COURT_ARM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COVERED_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COVERED_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COVER_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_OUT_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARM_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INCLUDED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INCLUDED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSURANCE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSURER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ISSUER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAW_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVE_EARLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCAL_SERVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCAL_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVING_THE_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEEDED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEEDED_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_AN_AGREEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_HAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_SERVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_OLD_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPERATOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_ALLOWED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SERVICE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHIFT_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONES_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKE_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKE_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERMS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERMS_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERM_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TERM_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_OLD_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_OLD_ONE_POOL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TICKET_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TO_INSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VENUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VENUE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISITOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORKING_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AGREEMENT_WORD() -> Py { ensure_init(); AGREEMENT_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGREEMENT_WORD(v: Py) { ensure_init(); if let Ok(mut w) = AGREEMENT_WORD.write() { *w = v; } }
pub fn g_ALLOWANCE_WORD() -> Py { ensure_init(); ALLOWANCE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALLOWANCE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = ALLOWANCE_WORD.write() { *w = v; } }
pub fn g_ALLOWED() -> Py { ensure_init(); ALLOWED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALLOWED(v: Py) { ensure_init(); if let Ok(mut w) = ALLOWED.write() { *w = v; } }
pub fn g_ALLOWED_ASK() -> Py { ensure_init(); ALLOWED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALLOWED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ALLOWED_ASK.write() { *w = v; } }
pub fn g_ASKING_SHAPE() -> Py { ensure_init(); ASKING_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_SHAPE.write() { *w = v; } }
pub fn g_A_FACILITY() -> Py { ensure_init(); A_FACILITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FACILITY(v: Py) { ensure_init(); if let Ok(mut w) = A_FACILITY.write() { *w = v; } }
pub fn g_A_HAS_ASK() -> Py { ensure_init(); A_HAS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_HAS_ASK.write() { *w = v; } }
pub fn g_A_PLACE() -> Py { ensure_init(); A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = A_PLACE.write() { *w = v; } }
pub fn g_A_PLACE_HAS() -> Py { ensure_init(); A_PLACE_HAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_HAS(v: Py) { ensure_init(); if let Ok(mut w) = A_PLACE_HAS.write() { *w = v; } }
pub fn g_A_ROOM_FACILITY() -> Py { ensure_init(); A_ROOM_FACILITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_ROOM_FACILITY(v: Py) { ensure_init(); if let Ok(mut w) = A_ROOM_FACILITY.write() { *w = v; } }
pub fn g_BODY_STATE() -> Py { ensure_init(); BODY_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE(v: Py) { ensure_init(); if let Ok(mut w) = BODY_STATE.write() { *w = v; } }
pub fn g_CANCEL_TERMS() -> Py { ensure_init(); CANCEL_TERMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CANCEL_TERMS(v: Py) { ensure_init(); if let Ok(mut w) = CANCEL_TERMS.write() { *w = v; } }
pub fn g_CARRYING() -> Py { ensure_init(); CARRYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARRYING(v: Py) { ensure_init(); if let Ok(mut w) = CARRYING.write() { *w = v; } }
pub fn g_COMPLAINT() -> Py { ensure_init(); COMPLAINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT(v: Py) { ensure_init(); if let Ok(mut w) = COMPLAINT.write() { *w = v; } }
pub fn g_CONTRACT() -> Py { ensure_init(); CONTRACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRACT(v: Py) { ensure_init(); if let Ok(mut w) = CONTRACT.write() { *w = v; } }
pub fn g_CONTRACT_WORDS() -> Py { ensure_init(); CONTRACT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRACT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CONTRACT_WORDS.write() { *w = v; } }
pub fn g_COUNT_AS() -> Py { ensure_init(); COUNT_AS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_AS(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_AS.write() { *w = v; } }
pub fn g_COURT_ARM() -> Py { ensure_init(); COURT_ARM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COURT_ARM(v: Py) { ensure_init(); if let Ok(mut w) = COURT_ARM.write() { *w = v; } }
pub fn g_COVERED_IN() -> Py { ensure_init(); COVERED_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COVERED_IN(v: Py) { ensure_init(); if let Ok(mut w) = COVERED_IN.write() { *w = v; } }
pub fn g_COVERED_PHRASE() -> Py { ensure_init(); COVERED_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COVERED_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = COVERED_PHRASE.write() { *w = v; } }
pub fn g_COVER_IT() -> Py { ensure_init(); COVER_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COVER_IT(v: Py) { ensure_init(); if let Ok(mut w) = COVER_IT.write() { *w = v; } }
pub fn g_GET_OUT_OF() -> Py { ensure_init(); GET_OUT_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_OUT_OF(v: Py) { ensure_init(); if let Ok(mut w) = GET_OUT_OF.write() { *w = v; } }
pub fn g_HARM_NOUN() -> Py { ensure_init(); HARM_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARM_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = HARM_NOUN.write() { *w = v; } }
pub fn g_INCLUDED() -> Py { ensure_init(); INCLUDED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INCLUDED(v: Py) { ensure_init(); if let Ok(mut w) = INCLUDED.write() { *w = v; } }
pub fn g_INCLUDED_ASK() -> Py { ensure_init(); INCLUDED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INCLUDED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = INCLUDED_ASK.write() { *w = v; } }
pub fn g_INSURANCE_WORD() -> Py { ensure_init(); INSURANCE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSURANCE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = INSURANCE_WORD.write() { *w = v; } }
pub fn g_INSURER() -> Py { ensure_init(); INSURER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSURER(v: Py) { ensure_init(); if let Ok(mut w) = INSURER.write() { *w = v; } }
pub fn g_ISSUER() -> Py { ensure_init(); ISSUER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ISSUER(v: Py) { ensure_init(); if let Ok(mut w) = ISSUER.write() { *w = v; } }
pub fn g_LAW_NAMED() -> Py { ensure_init(); LAW_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAW_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = LAW_NAMED.write() { *w = v; } }
pub fn g_LEAVE_EARLY() -> Py { ensure_init(); LEAVE_EARLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVE_EARLY(v: Py) { ensure_init(); if let Ok(mut w) = LEAVE_EARLY.write() { *w = v; } }
pub fn g_LEAVING_ASK() -> Py { ensure_init(); LEAVING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = LEAVING_ASK.write() { *w = v; } }
pub fn g_LOCAL() -> Py { ensure_init(); LOCAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCAL(v: Py) { ensure_init(); if let Ok(mut w) = LOCAL.write() { *w = v; } }
pub fn g_LOCAL_SERVICE() -> Py { ensure_init(); LOCAL_SERVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCAL_SERVICE(v: Py) { ensure_init(); if let Ok(mut w) = LOCAL_SERVICE.write() { *w = v; } }
pub fn g_LOCAL_WORDS() -> Py { ensure_init(); LOCAL_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCAL_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LOCAL_WORDS.write() { *w = v; } }
pub fn g_MOVING_THE_OLD() -> Py { ensure_init(); MOVING_THE_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVING_THE_OLD(v: Py) { ensure_init(); if let Ok(mut w) = MOVING_THE_OLD.write() { *w = v; } }
pub fn g_NAMED_OBJECT() -> Py { ensure_init(); NAMED_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_OBJECT.write() { *w = v; } }
pub fn g_NEEDED_ASK() -> Py { ensure_init(); NEEDED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEEDED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NEEDED_ASK.write() { *w = v; } }
pub fn g_NEEDED_THING() -> Py { ensure_init(); NEEDED_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEEDED_THING(v: Py) { ensure_init(); if let Ok(mut w) = NEEDED_THING.write() { *w = v; } }
pub fn g_NOISE() -> Py { ensure_init(); NOISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOISE(v: Py) { ensure_init(); if let Ok(mut w) = NOISE.write() { *w = v; } }
pub fn g_NOT_AN_AGREEMENT() -> Py { ensure_init(); NOT_AN_AGREEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_AGREEMENT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_AN_AGREEMENT.write() { *w = v; } }
pub fn g_NOT_A_HAS() -> Py { ensure_init(); NOT_A_HAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_HAS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_HAS.write() { *w = v; } }
pub fn g_NOT_A_SERVICE() -> Py { ensure_init(); NOT_A_SERVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SERVICE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_SERVICE.write() { *w = v; } }
pub fn g_NOT_THE_OLD_ONE() -> Py { ensure_init(); NOT_THE_OLD_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_OLD_ONE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_OLD_ONE.write() { *w = v; } }
pub fn g_OPERATOR() -> Py { ensure_init(); OPERATOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPERATOR(v: Py) { ensure_init(); if let Ok(mut w) = OPERATOR.write() { *w = v; } }
pub fn g_PERSON_ALLOWED() -> Py { ensure_init(); PERSON_ALLOWED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_ALLOWED(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_ALLOWED.write() { *w = v; } }
pub fn g_PRICE_ASK() -> Py { ensure_init(); PRICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_ASK.write() { *w = v; } }
pub fn g_PRICE_THING() -> Py { ensure_init(); PRICE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_THING(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_THING.write() { *w = v; } }
pub fn g_QUESTION() -> Py { ensure_init(); QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION.write() { *w = v; } }
pub fn g_SERVICE_WORDS() -> Py { ensure_init(); SERVICE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SERVICE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SERVICE_WORDS.write() { *w = v; } }
pub fn g_SHIFT_WORD() -> Py { ensure_init(); SHIFT_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHIFT_WORD(v: Py) { ensure_init(); if let Ok(mut w) = SHIFT_WORD.write() { *w = v; } }
pub fn g_SOMEONES_PERSON() -> Py { ensure_init(); SOMEONES_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONES_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONES_PERSON.write() { *w = v; } }
pub fn g_TAKE_ON() -> Py { ensure_init(); TAKE_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKE_ON(v: Py) { ensure_init(); if let Ok(mut w) = TAKE_ON.write() { *w = v; } }
pub fn g_TAKE_TO() -> Py { ensure_init(); TAKE_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKE_TO(v: Py) { ensure_init(); if let Ok(mut w) = TAKE_TO.write() { *w = v; } }
pub fn g_TERMS() -> Py { ensure_init(); TERMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERMS(v: Py) { ensure_init(); if let Ok(mut w) = TERMS.write() { *w = v; } }
pub fn g_TERMS_ASK() -> Py { ensure_init(); TERMS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERMS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TERMS_ASK.write() { *w = v; } }
pub fn g_TERMS_THING() -> Py { ensure_init(); TERMS_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERMS_THING(v: Py) { ensure_init(); if let Ok(mut w) = TERMS_THING.write() { *w = v; } }
pub fn g_TERM_SHAPE() -> Py { ensure_init(); TERM_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERM_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = TERM_SHAPE.write() { *w = v; } }
pub fn g_TERM_WORD() -> Py { ensure_init(); TERM_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TERM_WORD(v: Py) { ensure_init(); if let Ok(mut w) = TERM_WORD.write() { *w = v; } }
pub fn g_THE_OLD_ONE() -> Py { ensure_init(); THE_OLD_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_OLD_ONE(v: Py) { ensure_init(); if let Ok(mut w) = THE_OLD_ONE.write() { *w = v; } }
pub fn g_THE_OLD_ONE_POOL() -> Py { ensure_init(); THE_OLD_ONE_POOL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_OLD_ONE_POOL(v: Py) { ensure_init(); if let Ok(mut w) = THE_OLD_ONE_POOL.write() { *w = v; } }
pub fn g_TICKET_WORD() -> Py { ensure_init(); TICKET_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TICKET_WORD(v: Py) { ensure_init(); if let Ok(mut w) = TICKET_WORD.write() { *w = v; } }
pub fn g_TO_INSIDE() -> Py { ensure_init(); TO_INSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TO_INSIDE(v: Py) { ensure_init(); if let Ok(mut w) = TO_INSIDE.write() { *w = v; } }
pub fn g_VENUE() -> Py { ensure_init(); VENUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VENUE(v: Py) { ensure_init(); if let Ok(mut w) = VENUE.write() { *w = v; } }
pub fn g_VENUE_WORDS() -> Py { ensure_init(); VENUE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VENUE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = VENUE_WORDS.write() { *w = v; } }
pub fn g_VISITOR() -> Py { ensure_init(); VISITOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISITOR(v: Py) { ensure_init(); if let Ok(mut w) = VISITOR.write() { *w = v; } }
pub fn g_WARD() -> Py { ensure_init(); WARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WARD(v: Py) { ensure_init(); if let Ok(mut w) = WARD.write() { *w = v; } }
pub fn g_WORKING_OUT() -> Py { ensure_init(); WORKING_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORKING_OUT(v: Py) { ensure_init(); if let Ok(mut w) = WORKING_OUT.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Where a person would find out what the bot cannot know.\n\nThe bot has no network and no files, so it cannot read the small print of an insurance policy or a\ntravel pass, and it cannot say what somebody else's rules allow. Those are ordinary questions that\npeople bring here often, and they used to end in the honest hedge (\"a search should sort you out\nquickly\"), which is true and no help at all.\n\nThe live table in fallback.py already does the right thing for the weather, opening hours, live\nprices and stock of a named thing, and `bot/compose/prices.py` reads a price asked as a shape.\nThis module is the same move for the two kinds those do not carry: **the terms a thing is sold\nunder** (what the policy, the pass or the warranty covers) and **whose rules allow something**\n(is that legal, can I bring this in, what can my landlord ask for).\n\n`options(clean)` returns a pool of replies, or None when the line is not one of those kinds. The\ncaller is `bot/modules/where_it_answers.py`, a module of its own at PRIORITY -999: it asks the\ncomposer first and stands down unless that reply is a hedge, so this can only ever take the place\nof a hedge, never a card's turn or a reply of the composer's own. It is a module rather than a hook\nin `bot/compose/fallback.py` because that file is over the world's 1,048,576-byte cap on main.\n\nNothing here invents a fact. Each reply says, in a few words, what the bot cannot see and then\nnames the one place the answer actually lives: the terms page for what is included, the insurer\nfor a policy, the operator for what you can bring on, the council for its own rules, and Citizens\nAdvice or ACAS for a tenancy or a job. \"Not knowing, honestly\" in STYLE.md is the bar: a short\nhonest route beats a confident guess, and a confident guess here is somebody travelling on a\nmade-up rule or claiming on a policy that never covered them.\n");
    s_INSURANCE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:insurance|insurer|insured|insure|insures|policy|policies|excess|excesses|claim|claims|cover note)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TERMS_THING(stdlib_call("re", "compile", &[py_str("\\b(?:insurance|policy|cover note|pass|railcard|ticket|tickets|warranty|guarantee|membership|subscription|tariff|contract|lease|tenancy|terms|small print|conditions|excess|excesses|deposit|exclusions|breakdown|breakdown cover|recovery|roadside|contents cover|gadget cover|extended warranty)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:price|prices|rate|rates|cost|costs|booking|room|rooms|hotel|holiday|package|deal|hire|fee|fees|quote|breakfast|meals?|parking|wifi|transfer|transfers|extras?|tax|taxes|vat|service charge|delivery|postage and packing|p&p)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_INCLUDED_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|does|do)\\b[^?]{0,35}\\b(?:include|includes|included|extra|free with|come with|comes with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COVERED_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:am i|are we|are they|is it|is that|is this|is my|is our|is the|is that phone|are my)\\b[^?]{0,30}\\bcovered\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TERMS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:cover|covers|covered|include|includes|included|excluded|excludes|apply|applies|count|counts|valid|honour|honoured|pay out|payout|work|works|worked|accepted|accept|any use|use it|use on)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORKING_OUT(stdlib_call("re", "compile", &[py_str("\\bwork(?:s|ed|ing)?\\s+out\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ALLOWED_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|am i|are we|can i|can we|could i|may i)\\b[^?]{0,30}\\b(?:allowed|permitted|legal|against the rules)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKE_ON(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|may|am i allowed to|are we allowed to)\\s+(?:i\\s+|we\\s+)?(?:take|bring|carry|pack|board with)\\b[^?]{0,60}\\b(?:on|onto|into|in|aboard)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKE_TO(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|may)\\s+(?:i|we)\\s+(?:bring|take|carry)\\b[^?]{0,60}\\bto\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEEDED_THING(stdlib_call("re", "compile", &[py_str("\\b(?:licen[cs]e|licen[cs]es|licen[cs]ing|licen[cs]ed|permit|permits|visa|visas|register|registration|registered|certificate|certification|medical|inspection|qualification|qualifications|background check|dbs check|crhb|health and safety|passport|passports)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEEDED_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|needed|require|requires|required|have to|has to|must|renew|renewing|speed up|speed it up|expedite|fast track|pay extra|pay more)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGREEMENT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:tenancy|tenant|lease|landlord|letting agent|contract|membership|gym|subscription|phone|mobile|broadband|internet|insurance|policy|booking|holiday|rental|hire|course)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_AGREEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:card|meeting|dinner|lunch|reservation|restaurant|appointment|appointments|plans?|party|birthday|barbecue|bbq)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAVING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:cancel(?:ling)?\\s+early|leave\\s+early|break the contract|end the contract|charge\\w*\\s+me\\s+to\\s+(?:leave|cancel|go)|how much to cancel|notice period|early termination)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_OUT_OF(stdlib_call("re", "compile", &[py_str("\\bget(?:ting)?\\s+out\\s+of\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHIFT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:rota|shift|shifts|boss|manager|supervisor)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAVE_EARLY(stdlib_call("re", "compile", &[py_str("\\bleav(?:e|ing)\\s+early\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CANCEL_TERMS(stdlib_call("re", "compile", &[py_str("\\b(?:cancellation (?:fee|fees|charge|charges|policy)|how much to cancel|fee to cancel|cost to cancel|charge to cancel|refund policy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HARM_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:damage|damaged|flood|flooding|flooded|storm|storms|wind|water|fire|theft|thief|thieves|stolen|stole|accidental|accident|liquid|injury|injuries|illness|subsidence|burst|vermin)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TERM_SHAPE(stdlib_call("re", "compile", &[py_str("\\bhow long\\b[^?]{0,30}\\b(?:warranty|guarantee|guaranteed|cover|policy|contract|lease|tenancy|membership|subscription|protection)\\b|\\b(?:cancellation|refund|returns?|claims?|exchange) polic(?:y|ies)\\b|\\binsur(?:e|ed|es|ing|ance)\\b|\\b(?:what|which|whos|who) (?:is |are )?(?:not )?cover(?:ed|s)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRICE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how much|how many|cheapest|cheaper|quote|quotes?|priced?|prices?|cost|costs|costly|roughly|budget)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TERM_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:policy|policies|excess|excesses|claim|claims|terms|small print|conditions|exclusions|warranty|guarantee|deposit|premium|premiums)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_AS(stdlib_call("re", "compile", &[py_str("\\bcount(?:s|ed|ing)?\\s+(?:towards|toward|as)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ALLOWANCE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:luggage|baggage|allowance|limit|weight|fare|fares|ticket|tickets|quota|carry on|cabin bag|hand luggage|bag|bags|pushchair|buggy|pram)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:take|bring|carry|pack|board with)\\s+(?:it|this|that|them|those|one)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VENUE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:ferry|plane|flight|train|bus|coach|tube|tram|boat|cruise|hotel|restaurant|cafe|pub|camping|campsite|camp site|festival|stadium|ground|museum|gallery|cinema|pool|park|beach|venue|abroad|airport|airline|airlines|on board|carry on|cabin bag|hand luggage|luggage|baggage|suitcase|arena|theatre|gig|club|conference|showground|racecourse)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRACT_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:landlord|letting agent|tenancy|tenant|deposit|notice|contract|employer|employee|boss|redundancy|dismissed|sacked|refund|fined|fine|penalty|course of action|small claims|rent|evict|evicted|eviction|housing|mortgage|arrears|work|working|job|jobs|shift|shifts|annual leave|time off|day off|holiday ?pay|sick pay|maternity|paternity|overtime|rota)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ISSUER(py_tuple(vec![py_str("A licence, a permit or a visa is granted on conditions the issuing body writes, and most publish them as a short list: what you need, what it costs, how long it takes. The council grants licences and permits, and the government's site has passports, visas and driving licences. Their page first, their phone line if your own case is unusual."), py_str("Conditions come from whoever grants the permission, and they are on its own site rather than in the form. The same body can tell you whether your own case needs one at all, and a phone call about that is usually quicker than filling anything in.")]));
    s_LOCAL(py_tuple(vec![py_str("Council rules are written for a street or a postcode, which is exactly why nobody else can answer them for you: a parking permit, the bin day, what the tip takes and planning are all on your own council's site. Their phone line gets you a straight answer about your address."), py_str("Parking, bins, a permit, a hedge, planning: the council writes those rules and enforces them, and they are on its site under those headings. A ring with your address sorts out which one applies to you.")]));
    s_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:is|are|was|were|does|do|did|will|would|can|could|should|shall|may|am|have|has|what|whats|which)\\b|\\b(?:is it|is that|is this|is she|is he|is my|is the|are they|are we|does it|does my|does the|do they|do i|will it|will my|would it|can i|can we|should i)\\b|\\bhow (?:much|long|do i|can i|does|would)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WARD(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|ward|icu|hdu|itu|nicu|scbu|intensive care|high dependency|patient|patients|ward sister|nurse in charge)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VISITOR(stdlib_call("re", "compile", &[py_str("\\b(?:visitors?|visiting|visit)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONES_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|our|their)\\s+(?:mum|mom|mam|dad|mother|father|nan|nana|gran|granny|son|daughter|sister|brother|wife|husband|partner|friend|aunt|uncle|cousin|neighbour|neighbor|grandad|grandpa|grandma)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAW_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:the law|in law|against the law|legal|illegal|legally|unlawful|statute|statutory|consumer rights|gov\\.?uk|offence|offense|criminal|solicitor|lawyers?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CARRYING(stdlib_call("re", "compile", &[py_str("\\b(?:take|taking|bring|bringing|carry|carrying|come in|walk in|get in)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TO_INSIDE(stdlib_call("re", "compile", &[py_str("\\b(?:into|inside|on board|aboard|through security|in the cabin)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_ALLOWED(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|my son|my daughter|my kid|my kids|my child|my boy|my girl|the kids)\\b[^?]{0,20}\\ballowed\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COURT_ARM(stdlib_call("re", "compile", &[py_str("\\b(?:summons|summoned|magistrate|court date|court hearing|tribunal|prosecut\\w*|penalty charge notice|pcn|fixed penalty|fine notice|appeal the fine|contest the fine)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOCAL_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:parking|parked|park on|permit|pavement|kerb|dropped kerb|bins?|bin day|fly tip|fly tipping|recycling|planning permission|noise|neighbour's hedge|footpath|right of way|council|tip|skip permit|dog fouling|street light)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:bins?|wheelie|recycling|bulky waste|household waste|garden waste|the tip(?! of)|recycling cent(?:re|er)|street lights?|street lamps?|potholes?|kerbs?|gritting|fly ?tipping|(?:parking|resident|visitor) permits?|planning permission|planning application|allotments?|council tax)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOISE(stdlib_call("re", "compile", &[py_str("\\bnoise\\b|\\bnoisy\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COMPLAINT(stdlib_call("re", "compile", &[py_str("\\b(?:neighbours?|next door|loud music|nuisance|complain\\w*|report\\w*|environmental health|council)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SERVICE(stdlib_call("re", "compile", &[py_str("\\bbin(?:ned|ning)?\\s+(?:it|that|this|them|those|these)\\b|\\b(?:should|shall|do|can)\\s+(?:i|we)\\s+bin\\b|\\b(?:bread|kitchen|pedal|nappy|compost|diaper|sensor|swing) bins?\\b|\\brecycle bin\\b|\\brecycling bin\\b|\\bwindows?\\b|\\blaptop\\b|\\bdesktop\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:what|how|where|who|when|why|which|can|could|should|do|does|did|is|are|will|would)\\b|\\?")]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:on an empty stomach|empty stomach|with food|with water|before food|after food|before eating|after eating|on antibiotics|with alcohol|on a full stomach)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COVER_IT(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|do|can)\\s+(?:i|we)\\s+(?:cover|covering)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COVERED_IN(stdlib_call("re", "compile", &[py_str("\\bcovered\\s+(?:in|with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TERMS(py_tuple(vec![py_str("Cover sits in a section headed what is included and what is excluded, and the exclusions are often the longer list. That section is the one that counts; if the page is vague, the letter or email they sent you repeats the same words, and asking them in writing leaves you with an answer you can point to."), py_str("This turns on one clause: what the policy calls the thing, and whether your loss is on its list of exclusions. Find that section before you rely on it, and ask them to confirm it in writing, because a written answer is the one you can hold them to."), py_str("What applies is what the wording says, and it is written in the same three places: the section on what is included, the exclusions, and the letter they sent when you took it out. A call to whoever sold it you settles the last doubt, and their answer in writing settles it for good.")]));
    s_INCLUDED(py_tuple(vec![py_str("What a price covers is written where it is sold: the booking page lists what comes with it, and anything it does not list is charged on top, often at the desk rather than before. Ask for the total in writing before you pay, and keep that email for the day itself."), py_str("Included and not included are two lists, and the second is the one that costs you: it is on the page you book on, then at the desk, where the extras get added. Read the first list before you book and ask for the total by email, because that is the version they have to honour."), py_str("A room, a package or a rate names what comes with it in its own terms, and whatever it does not name is charged separately - parking, breakfast and the wifi are the usual three. Check that list before you book, and ring to confirm what applies to your own booking.")]));
    s_LOCAL_SERVICE(py_tuple(vec![py_str("Bins, the tip, permits, a street light, a complaint about noise: each one is your local authority's own service, decided for a whole street at a time and published by the council or town hall that runs it. Their site takes your postcode and gives the page for your address, and their phone line with the address answers faster than a search will. Tell me what they say and I'll help with the next step."), py_str("That one is settled per address rather than in general, so I can't see your street from here: the council or town hall for your address decides it for its own postcodes, and publishes what it wants from you - the address, the date, and usually one photo. Their page first, then their phone line if your case does not fit it. What is it you are trying to get done?"), py_str("Your council or town hall runs that service and writes the rule for it street by street, so its site with your postcode in it is the only place that is right for your address. Ring them if the page does not cover your case - they answer for one address faster than a search will. What have you got so far?")]));
    s_TICKET_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:railcard|rail pass|ticket|tickets|season ticket|oyster|travelcard|travel card|boarding pass|day pass|bus pass|fare|fares|rover|interrail)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OPERATOR(py_tuple(vec![py_str("A ticket usually names on its face the route, the operator and the times it is good for, and the full conditions live on the operator's page rather than at the station. Check the ticket, then their page on what the fare covers; the guard on board checks the one you have."), py_str("What a pass is good on is set by the operator whose buses or trains it is for, and their page on what is included lists it by name: the routes, the times and the extras. A call to them settles your own ticket in a few minutes."), py_str("Fares and passes come with conditions written by the company that runs the service, and they are published with the timetable rather than with the ticket. Their page on what the fare covers is the one that counts, and the guard on the day has the last word.")]));
    s_INSURER(py_tuple(vec![py_str("A policy is decided by its own wording and its list of exclusions, not by the kind of claim it sounds like. Read the exclusions first, then ring the insurer to confirm your own case before you claim or pay for anything: they have to tell you, and that answer is the one that counts."), py_str("Insurers decide this from the wording in your schedule, which is written when the policy is taken out and does not change with the claim. Their claims line will tell you straight away, and it is worth asking for it by email so you have it in writing."), py_str("Whether a claim is met is the policy's own test: what is included, then what is excluded, then the excess. The insurer is the only one who can apply that to your case, and asking before you spend anything is the cheap way round.")]));
    s_ALLOWED(py_tuple(vec![py_str("Places and companies write these rules down, usually under a heading like what you can bring or what is not allowed, and the person on the door has the last word on the day. Find that page, then ask them by email so you have the answer to show."), py_str("This is somebody else's rule to make, and they are the only ones who can apply it to your case. An email asking whether it is allowed is worth more than a phone call, because you can point to it later."), py_str("Rules like this are set by whoever runs the thing rather than by the law or by me, and they are written where you book. If the page is vague, ring before you set off: being turned away at the door is the expensive way to find out.")]));
    s_VENUE(py_tuple(vec![py_str("A ferry, an arena, a campsite: whoever runs the place writes what you can bring on, and it is on their own page about what is allowed in, not in the booking email. Worth a call before you travel if the page is vague."), py_str("What you can bring on with you is decided by whoever runs the place, and it is written for travellers rather than for lawyers - on their page about what is allowed in. A quick call settles it before you pack."), py_str("Their rule, written where you book: most places publish what is allowed in and what has to go in the hold or be left at the door. The page first, then a ring to confirm your own case.")]));
    s_CONTRACT(py_tuple(vec![py_str("That is set by what you agreed to: the section on notice and fees says what it costs to end it early, and it is usually a fixed sum or the rest of the term. If it is a tenancy or a job, Citizens Advice, or ACAS for work, will read it beside you for free."), py_str("The notice and the fee are on the paper you signed, or in the account page if you joined online, and it is usually one short section headed ending the agreement. Ask them for the figure in writing, because that is the one you can hold them to."), py_str("Leaving early usually costs a fixed sum or the rest of the term, and the section that says which is in the terms you agreed. If the wording is not clear, Citizens Advice will go through a tenancy or an employment contract with you for free.")]));
    s_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:hotel|guest ?house|b\\s?and\\s?b|hostel|campsite|caravan park|holiday park|cottage|villa|apartment|airport|station|venue|concert hall|leisure cent(?:re|er)|swimming pool|gym|restaurant|pub|club|theatre|cinema|hospital|surgery|office|rooms?|bedrooms?|the flat|the house|the cottage|the caravan|flat we(?:ve| have) booked|the building|building|the premises|the shop|the salon|the clinic|the garage)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FACILITY(stdlib_call("re", "compile", &[py_str("\\b(?:parking|car park|wifi|wi-?fi|internet|pool|gym|spa|sauna|breakfast|air ?con(?:ditioning)?|kitchen(?:ette)?|lift|elevator|shower|bath|towels?|hairdryer|hair dryer|kettle|microwave|fridge|freezer|washing machine|iron|tv|television|safe|balcony|room service|charger|plug sockets?|toaster|crockery|cutlery|pans|a cot|cot bed|high ?chair|towels|bedding|linen|accessible|wheelchair|dog friendly|disabled access)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_ROOM_FACILITY(stdlib_call("re", "compile", &[py_str("\\b(?:parking|car park|wifi|wi-?fi|internet|air ?con(?:ditioning)?|kitchen(?:ette)?|lift|elevator|shower|bath|towels?|hairdryer|hair dryer|kettle|microwave|fridge|washing machine|iron|tv|television|a safe|the safe|balcony|toaster|a cot|cot bed|high ?chair|plug sockets?|room service)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is there|are there|is there any|do they have|does (?:it|this|that|the [a-z]+) have|have (?:they|you) got|does it come with|will there be|is it got|got a|do (?:the|these|those|your) [a-z]+ have)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_HAS(stdlib_call("re", "compile", &[py_str("\\b(?:open|opens|opening|closed?|closing|hours|what time|how much|price|cost|charge|fee|parking permit|bus|train|tube|tram|roadworks|traffic)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_OLD_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:the|my|our|that|this|these|those) old (?:one|ones|unit|units|boiler|fridge|freezer|fridge ?freezer|washing machine|dishwasher|tumble dryer|cooker|oven|hob|bath|shower|toilet|radiators?|windows?|doors?|carpets?|sofa|settee|bed|mattress|tyres?|batteries|cupboard|wardrobe|kitchen|bathroom|fire|fireplace|water tank|tank)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVING_THE_OLD(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeping|take\\b[^,.;]{0,30}\\baway|getting? rid|get rid|dispose|disposal|recycl\\w*|scrap\\w*|trade ?in|remove|removed|removal|throw (?:it|them) out|skip|tip|leave (?:it|them)|what happens to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_OLD_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:rice|potatoes?|bread|milk|chicken|meat|curry|soup|leftovers?|stock|pasta|veg|cat|dog|horse|rabbit|hamster|fish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE_HAS(py_tuple(vec![py_str("Whether a place has a thing is decided by that place and not by its name: the same chain differs from one branch to the next, and a station, an office and a hotel each answer for themselves. Its own site or the page you book on lists what comes as standard and what costs extra, and one call to the place confirms what the page leaves out - ask for it in writing if it matters on the day."), py_str("Facilities belong to whoever runs the place, so the current list is on their own page rather than anywhere general, and anything charged for usually sits on a line of its own, so look for both before you decide. If the page is silent, the place itself is the one that knows."), py_str("That turns on the building rather than the name above the door, and the people there know it exactly. Ring or email the place itself and ask them to confirm it for the day you are going, so you have it in writing if it turns out not to be there.")]));
    s_THE_OLD_ONE_POOL(py_tuple(vec![py_str("The old one is usually the fitter's to take: whoever installs the new one carries the old away for a small fee if you ask when the job is quoted, and a shop that only delivers will not touch it. Ask about it with the quote and it is one less thing on the day."), py_str("Whether you can keep it turns on who owns it now: bought outright it is yours, and rented or on finance it is not - that is in the agreement, not on any page. If it is yours and nobody wants it, the council's own site lists the bulky collection, what they take and what it costs."), py_str("Two things settle it: who owns it after the swap, and who will move it. The installer answers the first when they quote, and a scrap merchant or the council's bulky waste service answers the second - both are worth asking before the day rather than on it.")]));
}
pub fn options(args: &[Py]) -> Py {
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
    let mut asking: Py = Py::None;
    let mut early_exit: Py = Py::None;
    let mut leaving: Py = Py::None;
    let mut taking: Py = Py::None;
    let mut terms: Py = Py::None;
    let _ = py_str("A pool of \"here is where to find out\" replies for this line, or None.\n\n    `clean` is the matchable line the rest of the composer reads: lower case, no apostrophes,\n    single spaces. Terms are read first, because \"is my phone covered if it gets stolen at the\n    festival\" is a question about a policy, not about the festival's rules.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_COVER_IT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_COVERED_IN(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_PERSON_ALLOWED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WORKING_OUT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BODY_STATE(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_VISITOR(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_WARD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SOMEONES_PERSON(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    asking = call_builtin("bool", &[mcall(&g_QUESTION(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*(?:was|were)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_THE_OLD_ONE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_MOVING_THE_OLD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_THE_OLD_ONE(), "search", &[clean.clone()]))) } } } } } }) {
    return g_THE_OLD_ONE_POOL();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_A_PLACE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_ROOM_FACILITY(), "search", &[clean.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_FACILITY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HAS_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_HAS(), "search", &[clean.clone()]))) } } } } } } } }) {
    return g_A_PLACE_HAS();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_CANCEL_TERMS(), "search", &[clean.clone()]) } }) {
    return g_TERMS();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_HARM_NOUN(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bcover(?:ed|s)?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return g_INSURER();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_TERM_SHAPE(), "search", &[clean.clone()]) } }) {
    if py_truthy(&{ let __b = mcall(&g_PRICE_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TERM_WORD(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_INSURANCE_WORD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HARM_NOUN(), "search", &[clean.clone()]) } }) {
    return g_INSURER();
    }
    if py_truthy(&mcall(&g_TICKET_WORD(), "search", &[clean.clone()])) {
    return g_OPERATOR();
    }
    if py_truthy(&mcall(&g_CONTRACT_WORDS(), "search", &[clean.clone()])) {
    return g_CONTRACT();
    }
    return g_TERMS();
    }
    terms = { let __b = mcall(&g_TERMS_THING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_COVERED_PHRASE(), "search", &[clean.clone()]) } };
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&terms.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_INCLUDED_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PRICE_THING(), "search", &[clean.clone()]) } } } } } }) {
    return g_INCLUDED();
    }
    if py_truthy(&mcall(&g_COURT_ARM(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = terms.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_TERMS_ASK(), "search", &[clean.clone()]) } } } }) {
    if py_truthy(&mcall(&g_INSURANCE_WORD(), "search", &[clean.clone()])) {
    return g_INSURER();
    }
    if py_truthy(&mcall(&g_TICKET_WORD(), "search", &[clean.clone()])) {
    return g_OPERATOR();
    }
    return g_TERMS();
    }
    early_exit = { let __b = stdlib_call("re", "search", &[py_str("\\bearly\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_AGREEMENT_WORD(), "search", &[clean.clone()]) } };
    leaving = { let __b = mcall(&g_LEAVING_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = early_exit.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GET_OUT_OF(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_AGREEMENT_WORD(), "search", &[clean.clone()]) } } } } } };
    if py_truthy(&{ let __b = leaving.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_LEAVE_EARLY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SHIFT_WORD(), "search", &[clean.clone()]) } } } }) {
    leaving = Py::Bool(false);
    }
    if py_truthy(&{ let __b = leaving.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_AN_AGREEMENT(), "search", &[clean.clone()]))) } }) {
    return g_CONTRACT();
    }
    if py_truthy(&{ let __b = mcall(&g_COUNT_AS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_ALLOWANCE_WORD(), "search", &[clean.clone()]) } }) {
    return g_VENUE();
    }
    if py_truthy(&{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NEEDED_THING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_NEEDED_ASK(), "search", &[clean.clone()]) } } } }) {
    return g_ISSUER();
    }
    taking = { let __b = Py::Bool(!py_truthy(&mcall(&g_NAMED_OBJECT(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAKE_ON(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAKE_TO(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_VENUE_WORDS(), "search", &[clean.clone()]) } } } } } };
    if py_truthy(&{ let __b = mcall(&g_ALLOWED_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { taking.clone() } }) {
    if py_truthy(&mcall(&g_LOCAL_WORDS(), "search", &[clean.clone()])) {
    return g_LOCAL();
    }
    if py_truthy(&{ let __b = mcall(&g_LAW_NAMED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_CARRYING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_TO_INSIDE(), "search", &[clean.clone()]) } })) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_VENUE_WORDS(), "search", &[clean.clone()])) {
    return g_VENUE();
    }
    if py_truthy(&mcall(&g_CONTRACT_WORDS(), "search", &[clean.clone()])) {
    return g_CONTRACT();
    }
    return g_ALLOWED();
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING_SHAPE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_A_SERVICE(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_SERVICE_WORDS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOISE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_COMPLAINT(), "search", &[clean.clone()]) } } } } } } } }) {
    return g_LOCAL_SERVICE();
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.routes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGREEMENT_WORD" => Some(g_AGREEMENT_WORD()),
            "ALLOWANCE_WORD" => Some(g_ALLOWANCE_WORD()),
            "ALLOWED" => Some(g_ALLOWED()),
            "ALLOWED_ASK" => Some(g_ALLOWED_ASK()),
            "ASKING_SHAPE" => Some(g_ASKING_SHAPE()),
            "A_FACILITY" => Some(g_A_FACILITY()),
            "A_HAS_ASK" => Some(g_A_HAS_ASK()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_PLACE_HAS" => Some(g_A_PLACE_HAS()),
            "A_ROOM_FACILITY" => Some(g_A_ROOM_FACILITY()),
            "BODY_STATE" => Some(g_BODY_STATE()),
            "CANCEL_TERMS" => Some(g_CANCEL_TERMS()),
            "CARRYING" => Some(g_CARRYING()),
            "COMPLAINT" => Some(g_COMPLAINT()),
            "CONTRACT" => Some(g_CONTRACT()),
            "CONTRACT_WORDS" => Some(g_CONTRACT_WORDS()),
            "COUNT_AS" => Some(g_COUNT_AS()),
            "COURT_ARM" => Some(g_COURT_ARM()),
            "COVERED_IN" => Some(g_COVERED_IN()),
            "COVERED_PHRASE" => Some(g_COVERED_PHRASE()),
            "COVER_IT" => Some(g_COVER_IT()),
            "GET_OUT_OF" => Some(g_GET_OUT_OF()),
            "HARM_NOUN" => Some(g_HARM_NOUN()),
            "INCLUDED" => Some(g_INCLUDED()),
            "INCLUDED_ASK" => Some(g_INCLUDED_ASK()),
            "INSURANCE_WORD" => Some(g_INSURANCE_WORD()),
            "INSURER" => Some(g_INSURER()),
            "ISSUER" => Some(g_ISSUER()),
            "LAW_NAMED" => Some(g_LAW_NAMED()),
            "LEAVE_EARLY" => Some(g_LEAVE_EARLY()),
            "LEAVING_ASK" => Some(g_LEAVING_ASK()),
            "LOCAL" => Some(g_LOCAL()),
            "LOCAL_SERVICE" => Some(g_LOCAL_SERVICE()),
            "LOCAL_WORDS" => Some(g_LOCAL_WORDS()),
            "MOVING_THE_OLD" => Some(g_MOVING_THE_OLD()),
            "NAMED_OBJECT" => Some(g_NAMED_OBJECT()),
            "NEEDED_ASK" => Some(g_NEEDED_ASK()),
            "NEEDED_THING" => Some(g_NEEDED_THING()),
            "NOISE" => Some(g_NOISE()),
            "NOT_AN_AGREEMENT" => Some(g_NOT_AN_AGREEMENT()),
            "NOT_A_HAS" => Some(g_NOT_A_HAS()),
            "NOT_A_SERVICE" => Some(g_NOT_A_SERVICE()),
            "NOT_THE_OLD_ONE" => Some(g_NOT_THE_OLD_ONE()),
            "OPERATOR" => Some(g_OPERATOR()),
            "PERSON_ALLOWED" => Some(g_PERSON_ALLOWED()),
            "PRICE_ASK" => Some(g_PRICE_ASK()),
            "PRICE_THING" => Some(g_PRICE_THING()),
            "QUESTION" => Some(g_QUESTION()),
            "SERVICE_WORDS" => Some(g_SERVICE_WORDS()),
            "SHIFT_WORD" => Some(g_SHIFT_WORD()),
            "SOMEONES_PERSON" => Some(g_SOMEONES_PERSON()),
            "TAKE_ON" => Some(g_TAKE_ON()),
            "TAKE_TO" => Some(g_TAKE_TO()),
            "TERMS" => Some(g_TERMS()),
            "TERMS_ASK" => Some(g_TERMS_ASK()),
            "TERMS_THING" => Some(g_TERMS_THING()),
            "TERM_SHAPE" => Some(g_TERM_SHAPE()),
            "TERM_WORD" => Some(g_TERM_WORD()),
            "THE_OLD_ONE" => Some(g_THE_OLD_ONE()),
            "THE_OLD_ONE_POOL" => Some(g_THE_OLD_ONE_POOL()),
            "TICKET_WORD" => Some(g_TICKET_WORD()),
            "TO_INSIDE" => Some(g_TO_INSIDE()),
            "VENUE" => Some(g_VENUE()),
            "VENUE_WORDS" => Some(g_VENUE_WORDS()),
            "VISITOR" => Some(g_VISITOR()),
            "WARD" => Some(g_WARD()),
            "WORKING_OUT" => Some(g_WORKING_OUT()),
            "options" => Some(py_func("options", |args: &[Py]| options(args))),
            _ => None,
        }),
        set: None,
    }))
}

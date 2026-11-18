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


pub static ADVICE_AGAIN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE_COOL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE_TIME_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE_TIME_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVICE_WORSE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ANSWER_ABOUT_A_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_IN_A_CRISIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VIEW_ASKED_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_BLISTER_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_ON_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURN_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_SEEN_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CRISIS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DONE_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELS_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELS_NOT_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_HELPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_QUOTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HYPOTHETICAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_HOSPITAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVING_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEDICAL_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVE_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_VISIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POP_A_BARE_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRIVACY_NOT_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFERS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED_FAMILY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED_NO_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSED_PARTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_BARE_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_DECISION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_TARIFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_TRADE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_WHAT_IT_IS_WORTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPAIR_WHETHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_GOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIED_THE_LOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRUST_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISIT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_FOR_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_FOR_IT_BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_FOR_IT_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_FOR_IT_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_FOR_IT_SAID_VOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_NAMED_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_TO_TALK_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_TO_TALK_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHO_TO_TALK_TO_A_BAD_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ADVICE_AGAIN_ASK() -> Py { ensure_init(); ADVICE_AGAIN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_AGAIN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_AGAIN_ASK.write() { *w = v; } }
pub fn g_ADVICE_COOL_ASK() -> Py { ensure_init(); ADVICE_COOL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_COOL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_COOL_ASK.write() { *w = v; } }
pub fn g_ADVICE_TIME_ASK() -> Py { ensure_init(); ADVICE_TIME_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_TIME_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_TIME_ASK.write() { *w = v; } }
pub fn g_ADVICE_TIME_WORDS() -> Py { ensure_init(); ADVICE_TIME_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_TIME_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_TIME_WORDS.write() { *w = v; } }
pub fn g_ADVICE_WORSE_ASK() -> Py { ensure_init(); ADVICE_WORSE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVICE_WORSE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ADVICE_WORSE_ASK.write() { *w = v; } }
pub fn g_AN_ANSWER_ABOUT_A_NO() -> Py { ensure_init(); AN_ANSWER_ABOUT_A_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ANSWER_ABOUT_A_NO(v: Py) { ensure_init(); if let Ok(mut w) = AN_ANSWER_ABOUT_A_NO.write() { *w = v; } }
pub fn g_ASKS_WHY() -> Py { ensure_init(); ASKS_WHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHY(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHY.write() { *w = v; } }
pub fn g_A_PERSON_IN_A_CRISIS() -> Py { ensure_init(); A_PERSON_IN_A_CRISIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IN_A_CRISIS(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_IN_A_CRISIS.write() { *w = v; } }
pub fn g_A_VIEW_ASKED_FOR() -> Py { ensure_init(); A_VIEW_ASKED_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VIEW_ASKED_FOR(v: Py) { ensure_init(); if let Ok(mut w) = A_VIEW_ASKED_FOR.write() { *w = v; } }
pub fn g_BURN_ASK() -> Py { ensure_init(); BURN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = BURN_ASK.write() { *w = v; } }
pub fn g_BURN_BLISTER_LINE() -> Py { ensure_init(); BURN_BLISTER_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_BLISTER_LINE(v: Py) { ensure_init(); if let Ok(mut w) = BURN_BLISTER_LINE.write() { *w = v; } }
pub fn g_BURN_ON_IT() -> Py { ensure_init(); BURN_ON_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_ON_IT(v: Py) { ensure_init(); if let Ok(mut w) = BURN_ON_IT.write() { *w = v; } }
pub fn g_BURN_THING() -> Py { ensure_init(); BURN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_THING(v: Py) { ensure_init(); if let Ok(mut w) = BURN_THING.write() { *w = v; } }
pub fn g_BURN_WORD() -> Py { ensure_init(); BURN_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURN_WORD(v: Py) { ensure_init(); if let Ok(mut w) = BURN_WORD.write() { *w = v; } }
pub fn g_CARE_SEEN_ASKS() -> Py { ensure_init(); CARE_SEEN_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_SEEN_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = CARE_SEEN_ASKS.write() { *w = v; } }
pub fn g_CRISIS_ASK() -> Py { ensure_init(); CRISIS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CRISIS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = CRISIS_ASK.write() { *w = v; } }
pub fn g_DONE_IT() -> Py { ensure_init(); DONE_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DONE_IT(v: Py) { ensure_init(); if let Ok(mut w) = DONE_IT.write() { *w = v; } }
pub fn g_FEELS_BETTER() -> Py { ensure_init(); FEELS_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELS_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = FEELS_BETTER.write() { *w = v; } }
pub fn g_FEELS_NOT_BETTER() -> Py { ensure_init(); FEELS_NOT_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELS_NOT_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = FEELS_NOT_BETTER.write() { *w = v; } }
pub fn g_HUMAN_ASK() -> Py { ensure_init(); HUMAN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_ASK.write() { *w = v; } }
pub fn g_HUMAN_HELPS() -> Py { ensure_init(); HUMAN_HELPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_HELPS(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_HELPS.write() { *w = v; } }
pub fn g_HUMAN_LEAD() -> Py { ensure_init(); HUMAN_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_LEAD.write() { *w = v; } }
pub fn g_HUMAN_QUOTED() -> Py { ensure_init(); HUMAN_QUOTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_QUOTED(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_QUOTED.write() { *w = v; } }
pub fn g_HUMAN_TAIL() -> Py { ensure_init(); HUMAN_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_TAIL.write() { *w = v; } }
pub fn g_HYPOTHETICAL() -> Py { ensure_init(); HYPOTHETICAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HYPOTHETICAL(v: Py) { ensure_init(); if let Ok(mut w) = HYPOTHETICAL.write() { *w = v; } }
pub fn g_IN_HOSPITAL() -> Py { ensure_init(); IN_HOSPITAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_HOSPITAL(v: Py) { ensure_init(); if let Ok(mut w) = IN_HOSPITAL.write() { *w = v; } }
pub fn g_LEAVING_IN_THE_LINE() -> Py { ensure_init(); LEAVING_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVING_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = LEAVING_IN_THE_LINE.write() { *w = v; } }
pub fn g_MEDICAL_PLACE() -> Py { ensure_init(); MEDICAL_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDICAL_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = MEDICAL_PLACE.write() { *w = v; } }
pub fn g_MOVE_ON() -> Py { ensure_init(); MOVE_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVE_ON(v: Py) { ensure_init(); if let Ok(mut w) = MOVE_ON.write() { *w = v; } }
pub fn g_NAMES_THE_BOT() -> Py { ensure_init(); NAMES_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_THE_BOT.write() { *w = v; } }
pub fn g_NOT_A_VISIT() -> Py { ensure_init(); NOT_A_VISIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_VISIT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_VISIT.write() { *w = v; } }
pub fn g_OWN_NO() -> Py { ensure_init(); OWN_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_NO(v: Py) { ensure_init(); if let Ok(mut w) = OWN_NO.write() { *w = v; } }
pub fn g_POP_A_BARE_IT() -> Py { ensure_init(); POP_A_BARE_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POP_A_BARE_IT(v: Py) { ensure_init(); if let Ok(mut w) = POP_A_BARE_IT.write() { *w = v; } }
pub fn g_PRIVACY_NOT_THIS() -> Py { ensure_init(); PRIVACY_NOT_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRIVACY_NOT_THIS(v: Py) { ensure_init(); if let Ok(mut w) = PRIVACY_NOT_THIS.write() { *w = v; } }
pub fn g_REFERS_BACK() -> Py { ensure_init(); REFERS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFERS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = REFERS_BACK.write() { *w = v; } }
pub fn g_REFUSED() -> Py { ensure_init(); REFUSED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED.write() { *w = v; } }
pub fn g_REFUSED_FAMILY() -> Py { ensure_init(); REFUSED_FAMILY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED_FAMILY(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED_FAMILY.write() { *w = v; } }
pub fn g_REFUSED_NOT() -> Py { ensure_init(); REFUSED_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED_NOT(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED_NOT.write() { *w = v; } }
pub fn g_REFUSED_NO_OBJECT() -> Py { ensure_init(); REFUSED_NO_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED_NO_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED_NO_OBJECT.write() { *w = v; } }
pub fn g_REFUSED_PARTY() -> Py { ensure_init(); REFUSED_PARTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSED_PARTY(v: Py) { ensure_init(); if let Ok(mut w) = REFUSED_PARTY.write() { *w = v; } }
pub fn g_REPAIR_BARE_IT() -> Py { ensure_init(); REPAIR_BARE_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_BARE_IT(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_BARE_IT.write() { *w = v; } }
pub fn g_REPAIR_DECISION() -> Py { ensure_init(); REPAIR_DECISION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_DECISION(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_DECISION.write() { *w = v; } }
pub fn g_REPAIR_PERSON() -> Py { ensure_init(); REPAIR_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_PERSON.write() { *w = v; } }
pub fn g_REPAIR_TARIFF() -> Py { ensure_init(); REPAIR_TARIFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_TARIFF(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_TARIFF.write() { *w = v; } }
pub fn g_REPAIR_TRADE() -> Py { ensure_init(); REPAIR_TRADE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_TRADE(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_TRADE.write() { *w = v; } }
pub fn g_REPAIR_WHAT_IT_IS_WORTH() -> Py { ensure_init(); REPAIR_WHAT_IT_IS_WORTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHAT_IT_IS_WORTH(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_WHAT_IT_IS_WORTH.write() { *w = v; } }
pub fn g_REPAIR_WHETHER() -> Py { ensure_init(); REPAIR_WHETHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WHETHER(v: Py) { ensure_init(); if let Ok(mut w) = REPAIR_WHETHER.write() { *w = v; } }
pub fn g_SELF_ASK() -> Py { ensure_init(); SELF_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SELF_ASK.write() { *w = v; } }
pub fn g_STILL_GOING() -> Py { ensure_init(); STILL_GOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_GOING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_GOING.write() { *w = v; } }
pub fn g_TRIED_THE_LOT() -> Py { ensure_init(); TRIED_THE_LOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIED_THE_LOT(v: Py) { ensure_init(); if let Ok(mut w) = TRIED_THE_LOT.write() { *w = v; } }
pub fn g_TRUST_ASK() -> Py { ensure_init(); TRUST_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRUST_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TRUST_ASK.write() { *w = v; } }
pub fn g_VISIT_ASK() -> Py { ensure_init(); VISIT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISIT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = VISIT_ASK.write() { *w = v; } }
pub fn g_WHO_FOR_IT() -> Py { ensure_init(); WHO_FOR_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT(v: Py) { ensure_init(); if let Ok(mut w) = WHO_FOR_IT.write() { *w = v; } }
pub fn g_WHO_FOR_IT_BODY() -> Py { ensure_init(); WHO_FOR_IT_BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT_BODY(v: Py) { ensure_init(); if let Ok(mut w) = WHO_FOR_IT_BODY.write() { *w = v; } }
pub fn g_WHO_FOR_IT_ROUTES() -> Py { ensure_init(); WHO_FOR_IT_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = WHO_FOR_IT_ROUTES.write() { *w = v; } }
pub fn g_WHO_FOR_IT_SAID() -> Py { ensure_init(); WHO_FOR_IT_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT_SAID(v: Py) { ensure_init(); if let Ok(mut w) = WHO_FOR_IT_SAID.write() { *w = v; } }
pub fn g_WHO_FOR_IT_SAID_VOICE() -> Py { ensure_init(); WHO_FOR_IT_SAID_VOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_FOR_IT_SAID_VOICE(v: Py) { ensure_init(); if let Ok(mut w) = WHO_FOR_IT_SAID_VOICE.write() { *w = v; } }
pub fn g_WHO_NAMED_BEFORE() -> Py { ensure_init(); WHO_NAMED_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_NAMED_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = WHO_NAMED_BEFORE.write() { *w = v; } }
pub fn g_WHO_TO_TALK_ASK() -> Py { ensure_init(); WHO_TO_TALK_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_TALK_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHO_TO_TALK_ASK.write() { *w = v; } }
pub fn g_WHO_TO_TALK_TO() -> Py { ensure_init(); WHO_TO_TALK_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_TALK_TO(v: Py) { ensure_init(); if let Ok(mut w) = WHO_TO_TALK_TO.write() { *w = v; } }
pub fn g_WHO_TO_TALK_TO_A_BAD_STRETCH() -> Py { ensure_init(); WHO_TO_TALK_TO_A_BAD_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_TALK_TO_A_BAD_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = WHO_TO_TALK_TO_A_BAD_STRETCH.write() { *w = v; } }
fn module_body() {
    let _ = py_str("These are patterns from `bot/compose/fallback.py`, kept in their own file.\n\nThe world refuses a PR whose clone holds a file over 1 MB, and this file reached more than\nthat on main: from then on every change to it was refused before a reviewer saw it.\nEvery name here is a literal or a `re.compile` of literals, and none reads anything the\nfile defines, so each block moves as one piece with no change to a character of it. They\nare imported back where they used to be defined, so every pattern built from them below\nand every name a test reads off the module still resolves.\n");
    s_WHO_FOR_IT(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|ok|okay|right|but|then|now|hi|hey|hello|please)\\s+)*who\\s+(?:do|should|can|could|would|must|will)\\s+(?:i|we)\\s+(?:call|ring|phone|contact|tell|ask|chase|see|use|get|speak\\s+to|talk\\s+to|go\\s+to|report(?:\\s+it)?\\s+to)(?:\\s+(?:out|in|around|round))?(?:\\s+(?:to\\s+)?(?:look\\s+at|fix|check|sort|deal\\s+with))?(?:\\s+(?:it|that|this|them))?(?:\\s+about\\s+(?:it|that|this|them))?(?:\\s+(?:first|next|now|then|please|again))?\\s*[.?!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_CRISIS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:who|someone|somebody|anyone|anybody)\\b[^.!?]{0,30}?\\b(?:talk|speak|chat|text|listen)\\b|\\b(?:need|want|like)\\b[^.!?]{0,12}?\\b(?:someone|somebody|anyone|a\\s+person)\\b[^.!?]{0,20}?\\b(?:talk|speak|listen|there)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_IN_A_CRISIS(py_tuple(vec![py_str("Ring 116 123 - Samaritans, free, any hour, and they will stay on the line as long as you need. If you are in danger right now, 999. I am still here too, for as long as you want to type.")]));
    s_WHO_FOR_IT_SAID(py_tuple(vec![py_str("{who}. Ring them about what you were just saying, and give the one line: what it is, since when, and what you want done."), py_str("{who}. Same call as a moment ago, so ring them with the details ready - what it is, when it started, any reference - and tell me what they say.")]));
    s_WHO_FOR_IT_SAID_VOICE(py_tuple(vec![py_str("{who}. Talk to them about what you were just saying in your own words and ask what they would do - there is nothing to prepare."), py_str("{who}. Same person as a moment ago, so talk to them about it: what it is, since when, and how much it is on your mind.")]));
    s_WHO_NAMED_BEFORE(py_tuple(vec![py_str("{who} - the one the last answer pointed at. Ring them, say the one line, and ask what happens next."), py_str("{who}. You have the rest of it from a moment ago, so that is the call to make.")]));
    s_WHO_FOR_IT_BODY(py_tuple(vec![py_str("A body is not something I can look at, so the person to ring is your GP - and 111 tonight if it is getting worse. Give them the one line: what it is, when it started, and what you are afraid it might be."), py_str("Your GP is the one who can look at it, or 111 if it cannot wait until morning. Write down when it started first - that is the first thing they will ask.")]));
    s_WHO_FOR_IT_ROUTES(py_tuple(vec![py_str("It depends what it is, so give me the one line and I will point you at the right person: money, housing or a company that will not play ball is Citizens Advice; a job is a union rep or ACAS; a body or a medicine is a pharmacist or the GP; a leak, gas, the electrics or a broken appliance is a plumber, a gas engineer or an electrician."), py_str("There is no one answer to that, so tell me what it is about - money, a repair, a body, a pet, a form - and I will name whose job it is.")]));
    s_WHO_TO_TALK_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:who|anyone|anybody|someone|somebody)\\b[^.!?]{0,40}?\\b(?:talk|speak|chat)\\b|\\b(?:need|want)\\b[^.!?]{0,15}?\\b(?:someone|somebody|anyone|a\\s+person)\\b[^.!?]{0,20}?\\b(?:talk|speak|listen)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_TO_TALK_TO(py_tuple(vec![py_str("Who you can talk to is the one thing I have no record of, so say what it is about - a class, a job, a repair, a body - and I will name the kind of person worth asking, then help you word the first message."), py_str("I cannot pick the person for you, because I only have what you type. Tell me what it is about and I will say which kind of person knows, and what to say to them first.")]));
    s_WHO_TO_TALK_TO_A_BAD_STRETCH(py_tuple(vec![py_str("Who you can talk to is the one thing I have no record of, so it has to be your people: whoever you would ring at eleven at night, and if that is nobody this week, the GP is not only for a body - say it is too much and they can get you talking to someone properly. I can help you word the first message if you tell me who it is."), py_str("I cannot pick the person for you, because I only have what you type. The ordinary way in is one person who already knows you, and one sentence about how things actually are; if it is bigger than a bad week, your GP can put you in front of somebody trained for it. Say who you have in mind and I will help you work out what to say.")]));
    s_REPAIR_DECISION(stdlib_call("re", "compile", &[py_str("\\b(?:worth|bother)\\s+(?:fixing|repairing|mending|replacing|getting\\s+(?:it|that|this)\\s+(?:fixed|repaired|mended))\\b|\\bfix(?:ing)?\\s+(?:it\\s+)?or\\s+(?:replace|replacing|replacement|live|bin|chuck|buy\\s+(?:a\\s+)?new|buying\\s+(?:a\\s+)?new)\\b|\\brepair\\s+or\\s+replace\\b|\\b(?:fix|repair|mend)\\s+(?:it|that|this)\\s+or\\s+(?:live\\s+with\\s+it|replace\\s+it|leave\\s+it|buy\\s+(?:a\\s+)?new)\\b|\\bshall\\s+i\\s+(?:live\\s+with\\s+it|just\\s+(?:fix|replace)\\s+it)\\b|\\b(?:is|would)\\s+it\\s+be\\s+(?:better|cheaper)\\s+to\\s+(?:fix|repair|replace)\\b|\\b(?:worth|bother)\\s+(?:servicing|getting\\s+(?:it|that|this|them)\\s+serviced|a\\s+service|paying\\s+(?:for\\s+)?(?:a\\s+)?(?:service|repair|fix))\\b|\\b(?:worth|bother)\\s+getting\\s+(?:a\\s+)?new\\s+one\\b|\\b(?:get|getting)\\s+(?:it|that|this)\\s+serviced\\s+or\\s+(?:replace|replacing|bin|leave|keep)\\b|\\bservice\\s+(?:it|that|this)\\s+or\\s+(?:replace|bin|scrap|chuck)\\b|\\bkeep\\s+(?:it|that|this|them)\\s+or\\s+(?:replace|replacing|bin|chuck|get\\s+rid|buy\\s+(?:a\\s+)?new|trade\\s+(?:it\\s+)?in)\\b|\\b(?:write|writing)\\s+(?:it|that|this)\\s+off\\s+or\\s+(?:fix|repair|mend|keep)\\b|\\bor\\s+should\\s+i\\s+just\\s+(?:replace|bin|chuck)\\s+(?:it|that|this)\\b|\\b(?:should|shall)\\s+i\\s+(?:just\\s+)?(?:bin|chuck|scrap)\\s+(?:it|that|this)\\b|\\b(?:should|shall|do)\\s+i\\s+just\\s+write\\s+(?:it|that|this)\\s+off\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_WHETHER(stdlib_call("re", "compile", &[py_str("\\bcan\\s+(?:it|they|this|that|these|those)\\s+(?:be\\s+)?(?:get\\s+)?(?:fixed|repaired|mended|restored|salvaged)\\b|\\bcan\\s+i\\s+get\\s+(?:it|them|this|that|these|those|mine)\\s+(?:fixed|repaired|mended|restored)\\b|\\bis\\s+(?:it|this|that|my|our|the)\\s+(?:[a-z']+\\s+){0,2}(?:fixable|repairable|mendable|salvageable)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_WHAT_IT_IS_WORTH(stdlib_call("re", "compile", &[py_str("\\b(?:worth|bother|cheaper)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_BARE_IT(stdlib_call("re", "compile", &[py_str("\\b(?:bin|chuck|scrap|ditch|replace|write|throw)\\s+(?:it|that|this|them)\\b|\\bkeep\\s+(?:it|that|this|them)\\s+or\\b|\\bor\\s+should\\s+i\\s+just\\s+(?:replace|bin|chuck|scrap)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:plumber|electrician|engineer|handyman|someone|somebody|tradesman|professional|garage)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:marriage|relationship|friendship|ex|husband|wife|partner|boyfriend|girlfriend|son|daughter|kids?|children|mum|mom|mother|dad|father|brother|sister|nan|gran|grandma|grandad|friend|friends|mate|mates|parents?|family|hair|skin|teeth|credit)\\b|\\bthings?\\s+(?:with|between)\\s+(?:my|his|her|their|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAVING_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\bor\\s+(?:shall|should)\\s+i\\s+(?:just\\s+)?(?:leave|walk\\s+away|split\\s+up|end\\s+it|call\\s+it\\s+a\\s+day|move\\s+out|divorce)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_TARIFF(stdlib_call("re", "compile", &[py_str("\\b(?:supplier|tariffs?|energy|electricity|gas and electric|price cap|standing charge|mortgage|remortgag\\w*|contract|deal|policy|premium|insurance|price|prices|rate|rates|fix(?:ed|ing)?\\s+(?:it\\s+)?for\\s+(?:one|two|three|four|five|six|a|an|\\d+)\\s*(?:year|years|month|months))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRUST_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|would)\\s+i\\s+trust\\s+(?:you|what\\s+you|your\\s+\\w+|the\\s+(?:answers?|advice|things?)\\s+you)\\b|\\btrust\\s+(?:what\\s+you\\s+(?:say|tell)|your\\s+answers?|you\\s+with)\\b|\\bare\\s+you\\s+(?:safe|reliable|trustworthy|honest)\\b|\\bis\\s+(?:it|this)\\s+safe\\s+to\\s+(?:talk|chat|speak)\\s+(?:to\\s+)?you\\b|\\bhow\\s+do\\s+i\\s+know\\s+(?:you|that\\s+you|what\\s+you\\s+(?:say|tell))\\b[^?!.]{0,30}?\\b(?:right|true|correct|honest|telling|lying|making)\\b|\\bcan\\s+you\\s+be\\s+(?:wrong|trusted)\\b|\\bare\\s+you\\s+making\\s+(?:this|that|it)\\s+up\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRIVACY_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:eat|eaten|eating|cook|cooked|cooking|freezer|fridge|left\\s+out|defrost|reheat|mouldy|moldy|smell|shelf|use\\s+by|best\\s+before|baby|child|kid|toddler|pregnan\\w*|drink|water|ibuprofen|paracetamol|medicine|dose|tablet|drive|driving|swim)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFERS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do|does|were|was|had|have)\\s+(?:you|u)\\s+(?:say|said|tell|told|mention|mean)\\b|\\byou\\s+(?:said|says|told|mentioned)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:ai|a\\.i\\.|bot|robot|chatbot|machine|computer|program|programme|algorithm|model|human|person|people|alive|sentient|conscious|real)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VIEW_ASKED_FOR(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|so|but|ok|okay|well|honestly|then|right|also)\\s+)*(?:do|does|did|would|will)\\s+(?:you|u)\\s+(?:think|reckon|believe|say)\\b(.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|do|does|did|are|is|was|were|will|would|have|has|why|where|when|how\\s+long)\\s+(?:you|u)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUMAN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:am i|are we|is anyone|is somebody) (?:talking|speaking|chatting|typing) (?:to|with) (?:a |an |a real |some )*(?:human|person|real person|bot|robot|machine|computer|ai|program)\\b|\\bis (?:this|it) (?:a |an |really )*(?:real )?(?:human|person|bot|robot|machine|ai|program|computer)\\b|\\b(?:real person|actual person|human being) (?:on|at) the (?:other|end)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUMAN_TAIL(stdlib_call("re", "compile", &[py_str("^(?:\\s*(?:or|and|not|just|really|actually|so|then|please|pls|though|right|then)\\b)*\\s*(?:a |an |the )?(?:human|person|bot|robot|machine|computer|ai|program|real person|program|chatbot|chat bot|thing|it)?\\s*(?:or (?:a |an )?(?:human|person|bot|robot|machine|computer|ai|program|chatbot|chat bot|thing))?\\s*(?:on here|in here|in this chat|on this|here|end|line|side|thanks|thank you|cheers|ok|okay)?\\s*[.!?]*\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUMAN_HELPS(stdlib_call("re", "compile", &[py_str("^\\s*(?:who|that|which)?\\s*(?:can|could|will|would|is able to|to)\\s+(?:help|answer|sort|deal with|look at|explain|tell me|check)\\b|^\\s*(?:i can (?:talk|speak|ask) to|to (?:talk|speak) (?:to|with))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUMAN_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:hi|hiya|hey|hello|ok|okay|so|but|and|also|sorry|wait|right|just|quick|question|one|more|thing|to|check|first|then|is|are|am|was|were|do|does|did|there|any|anyone|anybody|somebody|this|that|a|an|the|real|actually|really)\\b[ ,]*)*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUMAN_QUOTED(stdlib_call("re", "compile", &[py_str("\\b(?:says|said|asks|asked|wants to know|is asking|was asking|keeps asking|texted)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIED_THE_LOT(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|alright|right|so|well|yeah|yep|yes|cool|fine|and|but|then|i|we|ive|i've|weve|we've)\\s+)*(?:(?:have|had|did)\\s+)?(?:tried|been trying|done|did)\\s+(?:them all|it all|everything|the lot|all of (?:it|them|those|these)|both)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_IT(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|alright|right|so|well|yeah|yep|yes|cool|fine|and|but|then|i|we|ive|i've|weve|we've)\\s+)*(?:(?:have|had)\\s+)?(?:done|did)\\s+(?:it|that|this|them)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|saying|told\\s+me)\\s+no\\b|\\b(?:turned|turns)\\s+(?:me|it|us|him|her|them)\\s+down\\b|\\b(?:knocked|knocks)\\s+(?:me|it|us|them)\\s+back\\b|\\bno\\s+(?:for\\s+an\\s+answer|was\\s+the\\s+answer)\\b|\\b(?:he|she|they|we|the\\s+\\w+|my\\s+\\w+)\\s+refused\\b|\\b(?:wouldnt|wouldn't|wont|won't)\\s+(?:budge|bend|agree|have\\s+it|do\\s+it|come\\s+round)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:doctor|gp|nurse|consultant|midwife|surgeon|physio\\w*|pharmacist|chemist|dentist|hospital|surgery|clinic|111|a&e|ae|paramedic\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_NO(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|ive|i've|weve|we've)\\b[^.!?]{0,12}?\\b(?:said|says|told|tells?|turned|refused|knocked)\\b[^.!?]{0,12}?\\bno\\b|\\b(?:i|we)\\b[^.!?]{0,12}?\\b(?:refused|turned\\s+\\w+\\s+down)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED_FAMILY(stdlib_call("re", "compile", &[py_str("\\b(?:husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|ex|exes|mum|mom|mother|dad|father|son|daughter|sister|brother|mate|friend|flatmate|housemate|roommate|stepdad|stepmum|stepmother|stepfather|gran|granny|nan|nana|grandad|grandpa|grandma|uncle|aunt|auntie|cousin|in-law|in\\s+laws)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED_PARTY(stdlib_call("re", "compile", &[py_str("\\b(?:boss|manager|managers|employer|employers|hr|personnel|bank|banks|building\\s+society|council|councils|landlord|landlady|letting\\s+agent|jobcentre|job\\s+centre|dwp|panel|committee|board|union|university|uni|college|school|nursery|shop|store|retailer|seller|supplier|insurer|insurance|agent|agency|recruiter|firm|company|office|department|court|tribunal|ombudsman|clinic|hospital|charity|club|gym|council\\s+tax|mortgage\\s+company|housing\\s+association|benefits\\s+office)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSED_NO_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|saying|told\\s+me)\\s+no\\s+(?!(?:way|chance|deal|thanks|thank|worries|problem|dice|objection|to|about|on|again|twice|either|yet|at|for\\s+now|so|and|but|then|though|after|before|last|this|it|because|when|until|unless)\\b)[a-z]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ANSWER_ABOUT_A_NO(stdlib_call("re", "compile", &[py_str("\\b(?:dispute[ds]?|disputing|appeal|complain\\w*|complaint\\w*|ombudsman|tribunal|small\\s+claims|letter\\s+before\\s+action|trading\\s+standards|protection\\s+scheme|escalate|raise\\s+it\\s+with\\s+the\\s+council)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHY(stdlib_call("re", "compile", &[py_str("^(?:why|how|what|when|where|who|did|does|do|is|are|was|were|can|could|should|would|will|any\\s+idea)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HYPOTHETICAL(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+if|if|suppose|supposing|supposed|in\\s+case|unless|should|were\\s+to|may|might|would|what\\s+happens\\s+(?:if|when)|when)\\b(?:\\s+\\w+){0,5}\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELS_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'?m|we|we'?re|feeling|feel|doing|getting|its|it'?s)\\b[^.?!]{0,24}\\b(?:better|calmer|calm|alright|okay|ok|fine|easier|lighter|less\\s+(?:awful|bad))\\b|\\b(?:better|calmer)\\s+(?:now|today|tonight|thanks)\\b|\\b(?:(?:just\\s+)?a\\s+(?:bit|little|touch|lot)|lots|much|slightly|way|far)\\s+(?:better|calmer|easier|lighter)\\b|\\b(?:its|it'?s|it is)\\s+easing\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELS_NOT_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:don'?t|doesn'?t|not|never|still|worse|no better|not any)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_ON(stdlib_call("re", "compile", &[py_str("\\b(?:something else|something different|change the subject|move on|different topic|another topic|talk about anything|talk about something|nothing else)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:still|keeps?|kept|stopped|stops|worse|worsening|again|hasn't|has not|haven't|have not|won't)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_SEEN_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|do|does|need|needs|have to|got to) (?:i|we|it|this)\\b[^.?!]*\\b(?:go|a&e|stitches|checked|seen|hospital|doctor|walk-in|later|tonight)\\b|\\bdo (?:i|we) need\\b|\\bam i being (?:silly|daft)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_HOSPITAL(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|ward|casualty|admitted|admission|intensive care|icu|hdu|hospice|a\\s*&\\s*e|a and e|aande|ambulance|in for tests|in for an operation|having an operation|on a drip|taken in)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VISIT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:go|come|get) (?:up|over|round|across|down)\\b|\\bgo (?:and )?see\\b|\\bvisit(?:ing)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:go|come|get|walk|climb|run)\\s+(?:up|down)\\s+(?:and\\s+(?:down|up)\\s+)?(?:the\\s+|a\\s+|my\\s+|these\\s+|those\\s+)?(?:stairs?|staircase|steps?|ladder|hill|slope|ramp|stile|loft|scaffold)\\b|\\b(?:go|get|come)\\s+up\\s+(?:to\\s+)?(?:bed|my room|the bedroom)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEDICAL_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|casualty|ward|clinic|surgery|doctor|doctors|gp|walk[\\s-]?in|out of hours|a\\s*&\\s*e|a and e|aande|ae|999|111)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_ON_IT(stdlib_call("re", "compile", &[py_str("\\b(?:put|rub|smear|spread|dab)\\b[^.?!]{0,30}\\b(?:on|over) (?:it|them|that|there)\\b|\\b(?:cover|wrap|dress|bandage)\\b[^.?!]{0,20}\\b(?:it|them|that|the burn)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_THING(stdlib_call("re", "compile", &[py_str("\\b(?:cream|ointment|antiseptic|butter|toothpaste|ice|ice ?pack|plaster|dressing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|can|could|do|would|will|is|are)\\s+(?:i|we|it|they)\\b|\\?")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_BLISTER_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:pop|piercing|pierce|burst|drain|squeeze|lance)\\b[^.?!]{0,20}\\b(?:blister|bubble)\\b|\\bblister\\b[^.?!]{0,20}\\b(?:pop|burst|drain|squeeze)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURN_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:burns?|burnt|burned|burning|scald(?:s|ed|ing)?|sunburn(?:s|ed|t)?|sunburnt)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POP_A_BARE_IT(stdlib_call("re", "compile", &[py_str("\\b(?:pop|pierce|burst|drain|lance|squeeze)\\b[^.?!]{0,12}\\b(?:it|that|them|this)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_TIME_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was)\\s+(?:\\d+|one|two|three|four|five|ten|fifteen|twenty|thirty|forty)\\s*(?:min|mins|minutes?|secs?|seconds?)\\s+(?:enough|too\\s+(?:long|short|much|little)|ok|okay|fine)\\b|\\bis\\s+(?:that|this|it)\\s+long\\s+enough\\b|\\bhow\\s+long\\s+(?:should|shall|do|can|must|need)\\s+(?:i|we)\\s+(?:hold|keep|leave|stay|run|do|press|put)\\b|\\bhow\\s+long\\s+does\\s+it\\s+(?:need|take|have\\s+to\\s+be)\\b|\\bdoes\\s+it\\s+have\\s+to\\s+be\\s+that\\s+long\\b|\\b(?:can|could|should)\\s+i\\s+stop\\s+(?:early|now|sooner)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_COOL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:lukewarm|tepid)\\b|\\b(?:cold|cool|warm|hot)\\s+water\\b|\\bdoes\\s+it\\s+have\\s+to\\s+be\\s+(?:cold|cool|warm)\\b|\\bis\\s+(?:cold|cool|warm)\\s+(?:ok|okay|fine|enough)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_TIME_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|five|ten|fifteen|twenty|thirty)\\s*(?:minutes?|mins?|seconds?|secs?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_AGAIN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|shall|do)\\s+i\\s+(?:do|have|start|try|run|hold|press)\\s+(?:it|that|this)\\s+(?:again|twice)\\b|\\bdo\\s+i\\s+need\\s+to\\s+(?:do|start|repeat)\\s+(?:it|that)\\s+again\\b|\\b(?:can|could|should)\\s+i\\s+(?:do|have|run)\\s+(?:another|a\\s+second)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE_WORSE_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+if\\s+(?:it|that|this|the\\s+bleeding|the\\s+pain)\\s+(?:doesnt|does\\s+not|didnt|wont|won't|still\\s+doesnt)\\s+(?:stop|work|settle|ease|help|go\\s+away|slow)\\b|\\bwhat\\s+if\\s+(?:it|that)\\s+keeps?\\s+(?:going|bleeding|hurting|burning)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.fallback_words".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE_AGAIN_ASK" => Some(g_ADVICE_AGAIN_ASK()),
            "ADVICE_COOL_ASK" => Some(g_ADVICE_COOL_ASK()),
            "ADVICE_TIME_ASK" => Some(g_ADVICE_TIME_ASK()),
            "ADVICE_TIME_WORDS" => Some(g_ADVICE_TIME_WORDS()),
            "ADVICE_WORSE_ASK" => Some(g_ADVICE_WORSE_ASK()),
            "AN_ANSWER_ABOUT_A_NO" => Some(g_AN_ANSWER_ABOUT_A_NO()),
            "ASKS_WHY" => Some(g_ASKS_WHY()),
            "A_PERSON_IN_A_CRISIS" => Some(g_A_PERSON_IN_A_CRISIS()),
            "A_VIEW_ASKED_FOR" => Some(g_A_VIEW_ASKED_FOR()),
            "BURN_ASK" => Some(g_BURN_ASK()),
            "BURN_BLISTER_LINE" => Some(g_BURN_BLISTER_LINE()),
            "BURN_ON_IT" => Some(g_BURN_ON_IT()),
            "BURN_THING" => Some(g_BURN_THING()),
            "BURN_WORD" => Some(g_BURN_WORD()),
            "CARE_SEEN_ASKS" => Some(g_CARE_SEEN_ASKS()),
            "CRISIS_ASK" => Some(g_CRISIS_ASK()),
            "DONE_IT" => Some(g_DONE_IT()),
            "FEELS_BETTER" => Some(g_FEELS_BETTER()),
            "FEELS_NOT_BETTER" => Some(g_FEELS_NOT_BETTER()),
            "HUMAN_ASK" => Some(g_HUMAN_ASK()),
            "HUMAN_HELPS" => Some(g_HUMAN_HELPS()),
            "HUMAN_LEAD" => Some(g_HUMAN_LEAD()),
            "HUMAN_QUOTED" => Some(g_HUMAN_QUOTED()),
            "HUMAN_TAIL" => Some(g_HUMAN_TAIL()),
            "HYPOTHETICAL" => Some(g_HYPOTHETICAL()),
            "IN_HOSPITAL" => Some(g_IN_HOSPITAL()),
            "LEAVING_IN_THE_LINE" => Some(g_LEAVING_IN_THE_LINE()),
            "MEDICAL_PLACE" => Some(g_MEDICAL_PLACE()),
            "MOVE_ON" => Some(g_MOVE_ON()),
            "NAMES_THE_BOT" => Some(g_NAMES_THE_BOT()),
            "NOT_A_VISIT" => Some(g_NOT_A_VISIT()),
            "OWN_NO" => Some(g_OWN_NO()),
            "POP_A_BARE_IT" => Some(g_POP_A_BARE_IT()),
            "PRIVACY_NOT_THIS" => Some(g_PRIVACY_NOT_THIS()),
            "REFERS_BACK" => Some(g_REFERS_BACK()),
            "REFUSED" => Some(g_REFUSED()),
            "REFUSED_FAMILY" => Some(g_REFUSED_FAMILY()),
            "REFUSED_NOT" => Some(g_REFUSED_NOT()),
            "REFUSED_NO_OBJECT" => Some(g_REFUSED_NO_OBJECT()),
            "REFUSED_PARTY" => Some(g_REFUSED_PARTY()),
            "REPAIR_BARE_IT" => Some(g_REPAIR_BARE_IT()),
            "REPAIR_DECISION" => Some(g_REPAIR_DECISION()),
            "REPAIR_PERSON" => Some(g_REPAIR_PERSON()),
            "REPAIR_TARIFF" => Some(g_REPAIR_TARIFF()),
            "REPAIR_TRADE" => Some(g_REPAIR_TRADE()),
            "REPAIR_WHAT_IT_IS_WORTH" => Some(g_REPAIR_WHAT_IT_IS_WORTH()),
            "REPAIR_WHETHER" => Some(g_REPAIR_WHETHER()),
            "SELF_ASK" => Some(g_SELF_ASK()),
            "STILL_GOING" => Some(g_STILL_GOING()),
            "TRIED_THE_LOT" => Some(g_TRIED_THE_LOT()),
            "TRUST_ASK" => Some(g_TRUST_ASK()),
            "VISIT_ASK" => Some(g_VISIT_ASK()),
            "WHO_FOR_IT" => Some(g_WHO_FOR_IT()),
            "WHO_FOR_IT_BODY" => Some(g_WHO_FOR_IT_BODY()),
            "WHO_FOR_IT_ROUTES" => Some(g_WHO_FOR_IT_ROUTES()),
            "WHO_FOR_IT_SAID" => Some(g_WHO_FOR_IT_SAID()),
            "WHO_FOR_IT_SAID_VOICE" => Some(g_WHO_FOR_IT_SAID_VOICE()),
            "WHO_NAMED_BEFORE" => Some(g_WHO_NAMED_BEFORE()),
            "WHO_TO_TALK_ASK" => Some(g_WHO_TO_TALK_ASK()),
            "WHO_TO_TALK_TO" => Some(g_WHO_TO_TALK_TO()),
            "WHO_TO_TALK_TO_A_BAD_STRETCH" => Some(g_WHO_TO_TALK_TO_A_BAD_STRETCH()),
            _ => None,
        }),
        set: None,
    }))
}

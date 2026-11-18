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


pub static ABUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_DISCLOSURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_DISCLOSURE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_HEAD_INTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_HIT_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_MEMORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_PURSUIT_HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL_LABEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL_MAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERBAL_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABUSE_VERB_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_AFTER_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLOW_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BY_NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_AGGRESSOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_HIT_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAME_AFTER_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAME_NOT_A_BLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAME_PLAYED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAME_WORD_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIP_GAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIP_GAME_FENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIP_NOT_AN_ASSAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIP_NOT_AN_ASSAULT_STR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAS_ME_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIT_AS_A_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIT_BY_A_VEHICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIT_THING_NOT_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static I_DID_NOT_HIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static I_HIT_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MY_GROWN_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_FEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTNER_CLOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAY_FACE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAY_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_FEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_LOUD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_OUTSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_PLACE_WORK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PURSUIT_SUBJECT_EX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMORSE_FOR_THE_ACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_HARM_NEEDS_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_OWN_DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_FACE_PUSH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_FACE_STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_HIT_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFT_HIT_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_HOLDS_THE_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_BLOW_ON_A_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THREAT_COERCIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THREAT_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THROWN_AT_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_THREAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _REFUSED_EXIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_BLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_HAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_ONE_HIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static __all__: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABUSE() -> Py { ensure_init(); ABUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE.write() { *w = v; } }
pub fn g_ABUSER() -> Py { ensure_init(); ABUSER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSER(v: Py) { ensure_init(); if let Ok(mut w) = ABUSER.write() { *w = v; } }
pub fn g_ABUSE_DISCLOSURE() -> Py { ensure_init(); ABUSE_DISCLOSURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_DISCLOSURE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_DISCLOSURE.write() { *w = v; } }
pub fn g_ABUSE_DISCLOSURE_FINE() -> Py { ensure_init(); ABUSE_DISCLOSURE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_DISCLOSURE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_DISCLOSURE_FINE.write() { *w = v; } }
pub fn g_ABUSE_FINE() -> Py { ensure_init(); ABUSE_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_FINE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_FINE.write() { *w = v; } }
pub fn g_ABUSE_HEAD_INTO() -> Py { ensure_init(); ABUSE_HEAD_INTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_HEAD_INTO(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_HEAD_INTO.write() { *w = v; } }
pub fn g_ABUSE_HIT_ME() -> Py { ensure_init(); ABUSE_HIT_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_HIT_ME(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_HIT_ME.write() { *w = v; } }
pub fn g_ABUSE_MEMORY() -> Py { ensure_init(); ABUSE_MEMORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_MEMORY(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_MEMORY.write() { *w = v; } }
pub fn g_ABUSE_NOW() -> Py { ensure_init(); ABUSE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_NOW.write() { *w = v; } }
pub fn g_ABUSE_PURSUIT_HARD() -> Py { ensure_init(); ABUSE_PURSUIT_HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PURSUIT_HARD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_PURSUIT_HARD.write() { *w = v; } }
pub fn g_ABUSE_VERB() -> Py { ensure_init(); ABUSE_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERB(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERB.write() { *w = v; } }
pub fn g_ABUSE_VERBAL() -> Py { ensure_init(); ABUSE_VERBAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL.write() { *w = v; } }
pub fn g_ABUSE_VERBAL_FINE() -> Py { ensure_init(); ABUSE_VERBAL_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_FINE(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL_FINE.write() { *w = v; } }
pub fn g_ABUSE_VERBAL_LABEL() -> Py { ensure_init(); ABUSE_VERBAL_LABEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_LABEL(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL_LABEL.write() { *w = v; } }
pub fn g_ABUSE_VERBAL_MAD() -> Py { ensure_init(); ABUSE_VERBAL_MAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_MAD(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL_MAD.write() { *w = v; } }
pub fn g_ABUSE_VERBAL_PERSON() -> Py { ensure_init(); ABUSE_VERBAL_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL_PERSON.write() { *w = v; } }
pub fn g_ABUSE_VERBAL_SUBJECT() -> Py { ensure_init(); ABUSE_VERBAL_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERBAL_SUBJECT.write() { *w = v; } }
pub fn g_ABUSE_VERB_PART() -> Py { ensure_init(); ABUSE_VERB_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERB_PART(v: Py) { ensure_init(); if let Ok(mut w) = ABUSE_VERB_PART.write() { *w = v; } }
pub fn g_ASK_AFTER_ME() -> Py { ensure_init(); ASK_AFTER_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_AFTER_ME(v: Py) { ensure_init(); if let Ok(mut w) = ASK_AFTER_ME.write() { *w = v; } }
pub fn g_BLOW_THINGS() -> Py { ensure_init(); BLOW_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLOW_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = BLOW_THINGS.write() { *w = v; } }
pub fn g_BY_NOT_A_THING() -> Py { ensure_init(); BY_NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BY_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = BY_NOT_A_THING.write() { *w = v; } }
pub fn g_CHILD_AGGRESSOR() -> Py { ensure_init(); CHILD_AGGRESSOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_AGGRESSOR(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_AGGRESSOR.write() { *w = v; } }
pub fn g_CHILD_HIT_NOW() -> Py { ensure_init(); CHILD_HIT_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_HIT_NOW(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_HIT_NOW.write() { *w = v; } }
pub fn g_CLOSE() -> Py { ensure_init(); CLOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOSE(v: Py) { ensure_init(); if let Ok(mut w) = CLOSE.write() { *w = v; } }
pub fn g_DYING_OTHER() -> Py { ensure_init(); DYING_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = DYING_OTHER.write() { *w = v; } }
pub fn g_GAME_AFTER_ME() -> Py { ensure_init(); GAME_AFTER_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAME_AFTER_ME(v: Py) { ensure_init(); if let Ok(mut w) = GAME_AFTER_ME.write() { *w = v; } }
pub fn g_GAME_NOT_A_BLOW() -> Py { ensure_init(); GAME_NOT_A_BLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAME_NOT_A_BLOW(v: Py) { ensure_init(); if let Ok(mut w) = GAME_NOT_A_BLOW.write() { *w = v; } }
pub fn g_GAME_PLAYED() -> Py { ensure_init(); GAME_PLAYED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAME_PLAYED(v: Py) { ensure_init(); if let Ok(mut w) = GAME_PLAYED.write() { *w = v; } }
pub fn g_GAME_WORD_STOP() -> Py { ensure_init(); GAME_WORD_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAME_WORD_STOP(v: Py) { ensure_init(); if let Ok(mut w) = GAME_WORD_STOP.write() { *w = v; } }
pub fn g_GRIP_GAME() -> Py { ensure_init(); GRIP_GAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIP_GAME(v: Py) { ensure_init(); if let Ok(mut w) = GRIP_GAME.write() { *w = v; } }
pub fn g_GRIP_GAME_FENCE() -> Py { ensure_init(); GRIP_GAME_FENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIP_GAME_FENCE(v: Py) { ensure_init(); if let Ok(mut w) = GRIP_GAME_FENCE.write() { *w = v; } }
pub fn g_GRIP_NOT_AN_ASSAULT() -> Py { ensure_init(); GRIP_NOT_AN_ASSAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIP_NOT_AN_ASSAULT(v: Py) { ensure_init(); if let Ok(mut w) = GRIP_NOT_AN_ASSAULT.write() { *w = v; } }
pub fn g_GRIP_NOT_AN_ASSAULT_STR() -> Py { ensure_init(); GRIP_NOT_AN_ASSAULT_STR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIP_NOT_AN_ASSAULT_STR(v: Py) { ensure_init(); if let Ok(mut w) = GRIP_NOT_AN_ASSAULT_STR.write() { *w = v; } }
pub fn g_HAS_ME_SUBJECT() -> Py { ensure_init(); HAS_ME_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAS_ME_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = HAS_ME_SUBJECT.write() { *w = v; } }
pub fn g_HIT_AS_A_CHILD() -> Py { ensure_init(); HIT_AS_A_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIT_AS_A_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = HIT_AS_A_CHILD.write() { *w = v; } }
pub fn g_HIT_BY_A_VEHICLE() -> Py { ensure_init(); HIT_BY_A_VEHICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIT_BY_A_VEHICLE(v: Py) { ensure_init(); if let Ok(mut w) = HIT_BY_A_VEHICLE.write() { *w = v; } }
pub fn g_HIT_THING_NOT_PERSON() -> Py { ensure_init(); HIT_THING_NOT_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIT_THING_NOT_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = HIT_THING_NOT_PERSON.write() { *w = v; } }
pub fn g_I_DID_NOT_HIT() -> Py { ensure_init(); I_DID_NOT_HIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_I_DID_NOT_HIT(v: Py) { ensure_init(); if let Ok(mut w) = I_DID_NOT_HIT.write() { *w = v; } }
pub fn g_I_HIT_SOMEONE() -> Py { ensure_init(); I_HIT_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_I_HIT_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = I_HIT_SOMEONE.write() { *w = v; } }
pub fn g_MONEY_NAMED() -> Py { ensure_init(); MONEY_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_NAMED.write() { *w = v; } }
pub fn g_MY_GROWN_UP() -> Py { ensure_init(); MY_GROWN_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MY_GROWN_UP(v: Py) { ensure_init(); if let Ok(mut w) = MY_GROWN_UP.write() { *w = v; } }
pub fn g_NOT_A_FEAR() -> Py { ensure_init(); NOT_A_FEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FEAR(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_FEAR.write() { *w = v; } }
pub fn g_OTHERS() -> Py { ensure_init(); OTHERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHERS(v: Py) { ensure_init(); if let Ok(mut w) = OTHERS.write() { *w = v; } }
pub fn g_PARTNER_CLOSE() -> Py { ensure_init(); PARTNER_CLOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTNER_CLOSE(v: Py) { ensure_init(); if let Ok(mut w) = PARTNER_CLOSE.write() { *w = v; } }
pub fn g_PEOPLE() -> Py { ensure_init(); PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE.write() { *w = v; } }
pub fn g_PERSON() -> Py { ensure_init(); PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = PERSON.write() { *w = v; } }
pub fn g_PLAY_FACE_REPLIES() -> Py { ensure_init(); PLAY_FACE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAY_FACE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = PLAY_FACE_REPLIES.write() { *w = v; } }
pub fn g_PLAY_NAMED() -> Py { ensure_init(); PLAY_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAY_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = PLAY_NAMED.write() { *w = v; } }
pub fn g_PURSUIT() -> Py { ensure_init(); PURSUIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT.write() { *w = v; } }
pub fn g_PURSUIT_FEAR() -> Py { ensure_init(); PURSUIT_FEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_FEAR(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_FEAR.write() { *w = v; } }
pub fn g_PURSUIT_LOUD() -> Py { ensure_init(); PURSUIT_LOUD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_LOUD(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_LOUD.write() { *w = v; } }
pub fn g_PURSUIT_OUTSIDE() -> Py { ensure_init(); PURSUIT_OUTSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_OUTSIDE(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_OUTSIDE.write() { *w = v; } }
pub fn g_PURSUIT_PLACE_WORK() -> Py { ensure_init(); PURSUIT_PLACE_WORK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_PLACE_WORK(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_PLACE_WORK.write() { *w = v; } }
pub fn g_PURSUIT_SUBJECT() -> Py { ensure_init(); PURSUIT_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_SUBJECT.write() { *w = v; } }
pub fn g_PURSUIT_SUBJECT_EX() -> Py { ensure_init(); PURSUIT_SUBJECT_EX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_SUBJECT_EX(v: Py) { ensure_init(); if let Ok(mut w) = PURSUIT_SUBJECT_EX.write() { *w = v; } }
pub fn g_REMORSE_FOR_THE_ACT() -> Py { ensure_init(); REMORSE_FOR_THE_ACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMORSE_FOR_THE_ACT(v: Py) { ensure_init(); if let Ok(mut w) = REMORSE_FOR_THE_ACT.write() { *w = v; } }
pub fn g_SELF_HARM_NEEDS_PERSON() -> Py { ensure_init(); SELF_HARM_NEEDS_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_NEEDS_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = SELF_HARM_NEEDS_PERSON.write() { *w = v; } }
pub fn g_SELF_OWN_DYING() -> Py { ensure_init(); SELF_OWN_DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_OWN_DYING(v: Py) { ensure_init(); if let Ok(mut w) = SELF_OWN_DYING.write() { *w = v; } }
pub fn g_SOFT_FACE_PUSH() -> Py { ensure_init(); SOFT_FACE_PUSH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_FACE_PUSH(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_FACE_PUSH.write() { *w = v; } }
pub fn g_SOFT_FACE_STOP() -> Py { ensure_init(); SOFT_FACE_STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_FACE_STOP(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_FACE_STOP.write() { *w = v; } }
pub fn g_SOFT_HIT_THING() -> Py { ensure_init(); SOFT_HIT_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_HIT_THING(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_HIT_THING.write() { *w = v; } }
pub fn g_SOFT_HIT_VERB() -> Py { ensure_init(); SOFT_HIT_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFT_HIT_VERB(v: Py) { ensure_init(); if let Ok(mut w) = SOFT_HIT_VERB.write() { *w = v; } }
pub fn g_SOMEONE_HOLDS_THE_MONEY() -> Py { ensure_init(); SOMEONE_HOLDS_THE_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_HOLDS_THE_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_HOLDS_THE_MONEY.write() { *w = v; } }
pub fn g_THE_BLOW_ON_A_CHILD() -> Py { ensure_init(); THE_BLOW_ON_A_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_BLOW_ON_A_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = THE_BLOW_ON_A_CHILD.write() { *w = v; } }
pub fn g_THREAT_COERCIVE() -> Py { ensure_init(); THREAT_COERCIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THREAT_COERCIVE(v: Py) { ensure_init(); if let Ok(mut w) = THREAT_COERCIVE.write() { *w = v; } }
pub fn g_THREAT_SUBJECT() -> Py { ensure_init(); THREAT_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THREAT_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = THREAT_SUBJECT.write() { *w = v; } }
pub fn g_THROWN_AT_ME() -> Py { ensure_init(); THROWN_AT_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THROWN_AT_ME(v: Py) { ensure_init(); if let Ok(mut w) = THROWN_AT_ME.write() { *w = v; } }
pub fn g__NOT_A_THREAT() -> Py { ensure_init(); _NOT_A_THREAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THREAT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_THREAT.write() { *w = v; } }
pub fn g__REFUSED_EXIT() -> Py { ensure_init(); _REFUSED_EXIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__REFUSED_EXIT(v: Py) { ensure_init(); if let Ok(mut w) = _REFUSED_EXIT.write() { *w = v; } }
pub fn g__THE_BLOW() -> Py { ensure_init(); _THE_BLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_BLOW(v: Py) { ensure_init(); if let Ok(mut w) = _THE_BLOW.write() { *w = v; } }
pub fn g__THE_HAND() -> Py { ensure_init(); _THE_HAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_HAND(v: Py) { ensure_init(); if let Ok(mut w) = _THE_HAND.write() { *w = v; } }
pub fn g__THE_ONE_HIT() -> Py { ensure_init(); _THE_ONE_HIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_ONE_HIT(v: Py) { ensure_init(); if let Ok(mut w) = _THE_ONE_HIT.write() { *w = v; } }
pub fn g___all__() -> Py { ensure_init(); __all__.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); if let Ok(mut w) = __all__.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Phrase tables for `safety.py`, kept in their own file.\n\n`bot/modules/safety.py` was 56 KB over the world's 1 MiB per-file cap on main `64514606`, and\n`open_pr` refuses every PR that touches a file over it - so the safety lane could not open a\nchange at all. These are the same tables, moved whole and imported back at the same point in\nthe same order, so the replies people get are identical. Nothing was deleted or rewritten.\n");
    s___all__(py_list(vec![py_str("ABUSE"), py_str("ABUSER"), py_str("BY_NOT_A_THING"), py_str("ABUSE_DISCLOSURE"), py_str("ABUSE_DISCLOSURE_FINE"), py_str("ABUSE_FINE"), py_str("ABUSE_HEAD_INTO"), py_str("ABUSE_HIT_ME"), py_str("ABUSE_MEMORY"), py_str("ABUSE_NOW"), py_str("ABUSE_PURSUIT_HARD"), py_str("ABUSE_VERB"), py_str("ABUSE_VERBAL"), py_str("ABUSE_VERBAL_FINE"), py_str("ABUSE_VERBAL_LABEL"), py_str("ABUSE_VERBAL_MAD"), py_str("ABUSE_VERBAL_PERSON"), py_str("ABUSE_VERBAL_SUBJECT"), py_str("ABUSE_VERB_PART"), py_str("ASK_AFTER_ME"), py_str("BLOW_THINGS"), py_str("CHILD_AGGRESSOR"), py_str("CHILD_HIT_NOW"), py_str("CLOSE"), py_str("DYING_OTHER"), py_str("GAME_AFTER_ME"), py_str("GAME_PLAYED"), py_str("GAME_WORD_STOP"), py_str("GRIP_GAME"), py_str("GRIP_GAME_FENCE"), py_str("GRIP_NOT_AN_ASSAULT"), py_str("GRIP_NOT_AN_ASSAULT_STR"), py_str("HAS_ME_SUBJECT"), py_str("HIT_AS_A_CHILD"), py_str("HIT_BY_A_VEHICLE"), py_str("HIT_THING_NOT_PERSON"), py_str("I_DID_NOT_HIT"), py_str("I_HIT_SOMEONE"), py_str("MONEY_NAMED"), py_str("NOT_A_FEAR"), py_str("OTHERS"), py_str("PARTNER_CLOSE"), py_str("PEOPLE"), py_str("PERSON"), py_str("PLAY_FACE_REPLIES"), py_str("PURSUIT"), py_str("PURSUIT_FEAR"), py_str("PURSUIT_LOUD"), py_str("PURSUIT_OUTSIDE"), py_str("PURSUIT_PLACE_WORK"), py_str("PURSUIT_SUBJECT"), py_str("PURSUIT_SUBJECT_EX"), py_str("REMORSE_FOR_THE_ACT"), py_str("SELF_HARM_NEEDS_PERSON"), py_str("SELF_OWN_DYING"), py_str("SOFT_FACE_PUSH"), py_str("SOFT_FACE_STOP"), py_str("SOMEONE_HOLDS_THE_MONEY"), py_str("THREAT_COERCIVE"), py_str("THREAT_SUBJECT"), py_str("_NOT_A_THREAT"), py_str("_REFUSED_EXIT"), py_str("_THE_BLOW"), py_str("_THE_HAND"), py_str("_THE_ONE_HIT"), py_str("_both_here")]));
    s_MONEY_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:pay\\s?day|paid|pay|wages?|salary|pension|benefits?|universal\\s+credit|money|cash|quid|pounds?|dollars?|euros?|fiver|tenner|skint|paycheck|overdraft|rent|bills?|\\d+\\s*(?:pounds?|quid|dollars?|euros?|bucks?))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_HOLDS_THE_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|someone|somebody|my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|mum|mom|dad|father|mother|son|daughter|brother|sister|flatmate|roommate|housemate|landlord))\\b[^.?!]{0,40}?\\b(?:took|takes|taking|emptied|empties|cleared\\s+out|clears\\s+out|controls?|controlled|keeps?|kept|hides?|hid|hidden|spends?|spent|gambled|stole|steals?)\\b[^.?!]{0,30}\\b(?:money|wages?|salary|pay|paycheck|account|benefits?|universal\\s+credit|pension|savings?|card)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_HARM_NEEDS_PERSON(Py::Bool(true));
    s_DYING_OTHER(py_tuple(vec![py_str("\\b(?:my|our|his|her|their|the)\\s+(?:nan|gran|nanny|nana|granny|grandma|grandmother|grandad|granddad|grandpa|grandfather|mum|mom|mother|dad|father|wife|husband|partner|son|daughter|child|kid|baby|brother|sister|aunt|uncle|cousin|friend|mate|dog|cat|puppy|kitten|pet|horse)\\b[^.?!]{0,20}\\b(?:is|are|'s|s)\\s+(?:going\\s+to|gonna|about\\s+to)\\s+die\\b")]));
    s_SELF_OWN_DYING(py_tuple(vec![py_str("\\b(?:im|i'm|i\\s+am|i\\s+think\\s+im|i\\s+think\\s+i'm|i\\s+feel|i\\s+keep\\s+thinking)\\b[^.?!]{0,20}\\b(?:going\\s+to|gonna|want\\s+to|wanna|ready\\s+to)\\s+die\\b"), py_str("\\bwant\\s+to\\s+die\\b"), py_str("\\bkill\\s+my\\s*self\\b"), py_str("\\bbetter\\s+off\\s+dead\\b"), py_str("\\bsuicid\\w*\\b")]));
    s_PERSON(py_str("(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|flatmate|roommate|baby|child|kid|kids|children|grand(?:child|children|son|daughter|kid)s?|step(?:child|children|son|daughter)s?|boy|girl|man|woman|lady|guy|bloke|neighbou?r|colleague|boss|gran|granny|nan|grandad|grandma|grandpa|mate|carers?|care\\s+workers?|support\\s+workers?|social\\s+workers?|personal\\s+assistants?|nurses?|midwives?|midwife|doctors?|gps?|physios?|therapists?|patient|pupil|student|teacher|driver|passenger|singer|player|runner|swimmer)"));
    s_PEOPLE(add(&add(&add(&add(&add(&add(&py_str("(?:i|i'm|i've|he|he's|she|she's|they|they're|we|you|someone|somebody|anyone|everyone|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")")));
    s_OTHERS(add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("(?:he|he's|she|she's|they|they're|him|her|them|someone|somebody|anyone|everyone|my\\s+"), &g_PERSON()), &py_str("|his\\s+")), &g_PERSON()), &py_str("|her\\s+")), &g_PERSON()), &py_str("|their\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")")));
    s_ABUSER(add(&add(&py_str("(?:"), &g_PERSON()), &py_str("|step(?:mum|mom|dad|mother|father|brother|sister|son|daughter|parent|child|children)|ex|landlord|landlady|housemate|lodger|aunt|auntie|uncle|cousin|nan|nana|grandparent|in-?laws?|father-?in-?law|mother-?in-?law|son-?in-?law|daughter-?in-?law|guardian|foster\\s+parent|boy|friend|manager|supervisor|nieces?|nephews?|grandmothers?|grandfathers?|grandparents?)")));
    s__THE_ONE_HIT(py_str("(?:my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|fianc(?:e|ée)|mum|mom|mother|dad|father|son|daughter|child|kid|kids|children|baby|stepson|stepdaughter|stepdad|stepmum|brother|sister|nan|gran|grandma|grandad|carer)(?!'s)|(?:my|the)\\s+(?:kids|children|child|baby|son|daughter)s?(?!'s)|her|him|them)\\b"));
    s__THE_BLOW(py_str("(?:hit|hitting|slap(?:ped|ping)?|punch(?:ed|ing)?|kick(?:ed|ing)?|beat|beating|chok(?:ed|ing)?|strangl(?:ed|ing)?|drag(?:ged|ging)?|threw|thrown|throwing|scratch(?:ed|ing)|claw(?:ed|ing))"));
    s__THE_HAND(py_str("(?:push(?:ed|ing)?|shov(?:e|ed|es|ing)|grab(?:bed|bing)?|pin(?:ned|ning)?|held|hold|pulled|yank(?:ed|ing)?|twist(?:ed|ing)?)"));
    s_I_HIT_SOMEONE(py_tuple(vec![add(&add(&add(&py_str("\\bi\\s+(?:keep\\s+|kept\\s+|just\\s+|nearly\\s+|almost\\s+|have\\s+|had\\s+)?"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT()), add(&add(&add(&add(&py_str("\\bi\\s+(?:keep\\s+|kept\\s+)?"), &g__THE_HAND()), &py_str("\\s+")), &g__THE_ONE_HIT()), &py_str("(?:[^.?!]{0,30}\\b(?:throat|neck|wall|floor|stairs|ground|door|hard|down|against|into\\s+the\\s+(?:road|traffic|kitchen|hall|bathroom|fire|car|bushes))\\b|[^.?!]{0,40}\\b(?:during|in|after|before)\\s+(?:an?\\s+)?(?:argument|row|fight|disagreement|falling\\s+out)\\b)")), add(&add(&add(&py_str("\\b(?:i\\s+)?(?:lost|lose|be\\s+losing)\\s+my\\s+temper\\b[^.?!]{0,30}\\b"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT()), add(&add(&add(&py_str("\\bi\\s+(?:was\\s+|got\\s+|get\\s+|am\\s+|'m\\s+|im\\s+)?(?:so\\s+)?(?:angry|mad|raging|drunk|pissed|upset|frustrated|stressed|exhausted|wound\\s+up|beside\\s+myself)\\b[^.?!]{0,25}\\b"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT())]));
    s_HIT_THING_NOT_PERSON(py_tuple(vec![py_str("\\b(?:partner|husband|wife|boyfriend|girlfriend|ex|fiance|fiancee|son|daughter|child|kid|kids|children|baby|mum|mom|mother|dad|father|brother|sister|nan|gran|grandma|grandad|carer|parents)'s\\b"), py_str("\\b(?:partner|husband|wife|boyfriend|girlfriend|ex|son|daughter|child|kid|kids|children|baby|mum|mom|mother|dad|father|brother|sister|carer)(?:s)?\\s+(?:\\w+\\s+){0,2}(?:cake|car|bike|toys?|ball|phone|laptop|window|door|plate|glass|cup|mug|tv|telly|scooter|bike|present|cards?|homework|drawing|picture)\\b")]));
    s_GAME_NOT_A_BLOW(py_tuple(vec![py_str("\\b(?:mess(?:ing)?\\s+about|mucking\\s+about|play(?:ing)?\\s*(?:fighting|fight)|sparring|boxing|wrestl(?:ing|e)|judo|karate|rugby|football|practice|training|drill|a\\s+game|snowball|water\\s+fight|paintball|nerf|tickl(?:e|ing)|play)\\b"), py_str("\\b(?:at|in|during|on)\\s+(?:the\\s+)?(?:chess|cards?|scrabble|monopoly|backgammon|draughts|checkers|dominoes|poker|darts|snooker|billiards|pool|bowling|tennis|badminton|squash|golf|cricket|football|rugby|rounders|netball|basketball|hockey|table\\s+tennis|video\\s+games?|playstation|xbox|wii|board\\s+game)\\b"), py_str("\\b(?:in|at|during)\\s+(?:[\\w-]+\\s+)?(?:final|match|game|tournament|league|race|marathon|competition|championship|quiz)\\b(?!\\s+(?:argument|row|fight|bust-?up|falling\\s+out))")]));
    s_I_DID_NOT_HIT(add(&py_tuple(vec![py_str("\\b(?:never|didn'?t|didnt|don'?t|dont|wouldn'?t|wouldnt|not|have\\s+never)\\s+(?:ever\\s+)?(?:hit|hitting|hurt|hurting|touch|touched|lay\\s+a\\s+hand|laid\\s+a\\s+hand)\\b"), add(&add(&py_str("\\bhit\\s+(?:her|him|them|my\\s+"), &g_ABUSER()), &py_str(")\\s+back\\b")), add(&add(&py_str("\\b(?:lose|losing|lost)\\s+my\\s+temper\\s+(?:with|at)\\s+(?:her|him|them|the\\s+kids|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:shout|shouted|shouting|yell|yelled|swore|swear|snapped|said|slam|slammed|walked|rowing|row|argue|argued|arguing)\\b")), py_str("\\bwhen\\s+we\\s+were\\s+(?:kids|children|little|lads|girls|boys)\\b"), py_str("\\bwe\\s+used\\s+to\\s+(?:fight|hit|scrap|wrestle)\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:\\d+|a\\s+kid|a\\s+child|a\\s+lad|a\\s+boy|a\\s+girl|young|little|small|tiny|at\\s+school|in\\s+school)\\b"), py_str("\\b(?:as|like)\\s+(?:a\\s+)?(?:kid|child|kids|children|lads|boys|girls)\\b"), py_str("\\bgrowing\\s+up\\b"), py_str("\\b(?:at|in)\\s+(?:primary|secondary|junior|middle|comprehensive)\\s+school\\b"), py_str("\\bi\\s+used\\s+to\\s+(?:\\w+\\s+){0,3}(?:hit|fight|scrap|wrestle)\\b[^.?!]{0,25}\\b(?:brother|sister|cousin|mate|friend|kids|kids|boy|girl)s?\\b"), py_str("\\b(?:accident|accidentally|by\\s+mistake|didn'?t\\s+mean|didnt\\s+mean|in\\s+my\\s+sleep|sleepwalking|fidgeting)\\b"), py_str("\\b(?:awake|out\\s+of\\s+bed|to\\s+wake\\s+(?:me|him|her|them|the\\s+kids))\\b"), py_str("\\b(?:in\\s+a\\s+dream|dream(?:t|ed)\\b|had\\s+a\\s+dream|nightmare)\\b")]), &g_GAME_NOT_A_BLOW()));
    s_MY_GROWN_UP(py_str("(?:dad|father|stepdad|mum|mom|mother|stepmum|parents|uncle|aunt|auntie|brother|sister|cousin|carer|nan|nana|gran|grandma|grandad|grandpa|grandparents|foster\\s+\\w+)"));
    s_THE_BLOW_ON_A_CHILD(add(&g__THE_BLOW(), &py_str("|beat|belt(?:ed)?|strap(?:ped)?|cane(?:d)?|smack(?:s|ed|ing)?|lock(?:ed)?|starve(?:d)?|smother(?:ed)?|throttle(?:d)?|abuse(?:d|s)?|molest(?:ed|s)?|touch(?:ed|es)?")));
    s_THROWN_AT_ME(py_str("(?:(?:throw|threw|thrown|throwing|hurl(?:ed|ing)?|chuck(?:ed|ing)?)[^.?!]{0,20}\\bat\\s+(?:me|us)\\b|(?:threw|thrown|throwing|throw|hurled)[^.?!]{0,15}\\b(?:against|into|down)\\s+(?:the\\s+)?(?:wall|door|floor|stairs|ground)\\b)"));
    s_HIT_AS_A_CHILD(py_tuple(vec![add(&add(&add(&add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,40}\\b(?:")), &g__THE_BLOW()), &py_str("|burn(?:ed|t)|scalded|whipped|belt(?:ed)?|strap(?:ped)?|cane(?:d)?|smack(?:s|ed|ing)?|abused|molested|touched\\s+me|locked\\s+me|starved|smothered|throttled)\\b[^.?!]{0,40}\\b(?:when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small|teenager|teen|at\\s+school|in\\s+school|\\d{1,2})|as\\s+a\\s+(?:kid|child|little\\s+\\w+|teenager)|growing\\s+up|back\\s+then|years\\s+ago|decades\\s+ago|my\\s+whole\\s+childhood|all\\s+my\\s+childhood|(?:used|would)\\s+to\\s+\\w+\\s+me)\\b")), py_str("\\bi\\s+(?:was|got)\\s+(?:hit|beaten|abused|molested|whipped|slapped|kicked|punched)\\s+(?:as\\s+a\\s+(?:kid|child|little\\s+\\w+|teenager)|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small|teenager|at\\s+school)|growing\\s+up|my\\s+whole\\s+childhood)\\b"), add(&add(&add(&add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,25}\\b(?:used\\s+to|would)\\s+[^.?!]{0,20}\\b(?:")), &g_THE_BLOW_ON_A_CHILD()), &py_str(")\\b[^.?!]{0,15}\\b(?:me|us)\\b")), add(&add(&add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,25}\\b(?:used\\s+to|would)\\s+[^.?!]{0,20}")), &g_THROWN_AT_ME()), add(&add(&py_str("\\bi\\s+(?:used|would)\\s+to\\s+(?:get|be)\\s+(?:hit|beaten|beat|slapped|punched|kicked|whipped|belted|abused|smacked)\\b[^.?!]{0,25}\\b(?:at\\s+home|by\\s+my\\s+"), &g_MY_GROWN_UP()), &py_str("|as\\s+a\\s+(?:kid|child)|when\\s+i\\s+was)\\b"))]));
    s_CHILD_HIT_NOW(py_tuple(vec![py_str("\\b(?:still|now|again|these\\s+days|nowadays|every\\s+(?:day|night|week|time))\\s+(?:\\w+\\s+){0,2}(?:hits?|beats?|does|doing|comes?|turns?\\s+up)\\b")]));
    s_REMORSE_FOR_THE_ACT(py_tuple(vec![py_str("\\b(?:hate|hating|disgusted\\s+(?:with|by)|ashamed\\s+of|sickened\\s+by|disappointed\\s+in|furious\\s+(?:with|at)|angry\\s+at)\\s+my\\s*self\\b"), py_str("\\bi\\s*(?:'?m|am|feel|felt)\\s+(?:like\\s+)?(?:such\\s+)?(?:an?\\s+)?(?:monster|beast|awful\\s+person|terrible\\s+person|horrible\\s+person|bad\\s+person)\\b"), py_str("\\bwhat\\s+have\\s+i\\s+done\\b"), py_str("\\bcan'?t\\s+believe\\s+i\\s+(?:did|done)\\s+(?:that|it)\\b")]));
    s_ABUSE_VERB(py_str("(?:hits?|hitting|beat(?:s|ing)?|punch(?:es|ed|ing)?|kick(?:s|ed|ing)?|chok(?:es|ed|ing)|slapp?(?:s|ed|ing)?|grab(?:s|bed|bing)?|drag(?:s|ged|ging)?|burn(?:s|ed|ing)?|strangl(?:e|es|ed|ing)|push(?:es|ed|ing)?|shov(?:e|es|ed|ing)|bit\\b|bite|bites|biting|spat\\s+at|spit\\s+at|throw(?:s|ing)?|threw|hurl(?:s|ed|ing)?|yank(?:s|ed|ing)?|twist(?:s|ed|ing)?|wrench(?:es|ed|ing)?|swung\\s+at|swing\\s+at|attack(?:s|ed|ing)?|savaged|mauled|lash(?:es|ed|ing)?\\s+out\\s+at|lunged\\s+at|held\\s+me\\s+down|pinned\\s+me|scratch(?:es|ed|ing)?|claw(?:s|ed|ing)?)"));
    s_HIT_BY_A_VEHICLE(py_str("(?!\\s+(?:with|by)\\s+(?:a\\s+|an\\s+|the\\s+|his\\s+|her\\s+|their\\s+|our\\s+|my\\s+)?(?:car|cars|van|bike|bikes|bicycle|motorbike|motorcycle|scooter|lorry|truck|bus|train|tram|taxi|car\\s+door|door|doors|vehicle|wheelchair|buggy|pram|cart|trolley|skateboard|scooter)\\b)"));
    s__NOT_A_THREAT(py_str("(?!\\s+(?:with|over|about)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:written|warning|fine|fines|eviction|legal|lawsuit|court|police|report|complaint|solicitor|letter|bill|bill?s|charge|charges|prosecution|dismissal|sack|sacking|redundancy|redundant|fired|disciplinary|hearing|notice|tenancy|deposit)\\b)"));
    s_BY_NOT_A_THING(add(&add(&py_str("(?!\\s+by\\s+(?!(?:(?:him|her|them|his|their|someone|somebody|some\\s+men|men|guys|blokes|lads|youths|people|strangers)\\b|(?:a|an|the|two|three|four|five|several|some|another|that|this|one)\\s+(?:\\w+\\s+){0,2}(?:men|women|man|bloke|blokes|stranger|strangers|guy|guys|woman|lady|ladies|person|people|group|youth|youths|teenager|teenagers|teen|teens|kid|kids|boy|boys|girl|girls|fella|fellas|lad|lads|neighbour|neighbours|driver|carer|partner|husband|wife|boyfriend|girlfriend|ex|dad|mum|father|mother|stepdad|stepmum|brother|sister|uncle|aunt|cousin|son|daughter|landlord|landlady|housemate|colleague|boss|manager)\\b)|my\\s+"), &g_ABUSER()), &py_str("\\b))")));
    s_CLOSE(py_str("(?:ex|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|stepdad|stepmum|stepmom|stepfather|stepmother|dad|mum|mom|father|mother|son|daughter|brother|sister|carer|uncle|aunt|auntie|nan|grandad|grandpa|grandma)"));
    s_THREAT_SUBJECT(add(&add(&py_str("(?:he'?s|she'?s|they'?re|he|she|they|my\\s+"), &g_CLOSE()), &py_str(")")));
    s_THREAT_COERCIVE(py_tuple(vec![add(&add(&py_str("\\b"), &g_THREAT_SUBJECT()), &py_str("\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b[^.?!]{0,25}\\b(?:police|cops?|solicitor|lawyer|court|custody|social\\s+services)\\b")), add(&add(&py_str("\\b"), &g_THREAT_SUBJECT()), &py_str("\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\s+to\\s+(?:call|ring|report|tell|contact|go\\s+to)\\b[^.?!]{0,20}\\b(?:police|cops?|solicitor|lawyer|court|custody|social\\s+services)\\b")), add(&add(&add(&add(&py_str("\\bmy\\s+"), &g_CLOSE()), &py_str("(?:'?s|\\s+is|\\s+keeps?|\\s+has\\s+been)\\s+threatening\\s+my\\s+")), &g_CLOSE()), &py_str("\\b"))]));
    s__REFUSED_EXIT(py_str("(?:past|through|out|out\\s+of\\s+the\\s+(?:house|flat|room|car)|leave|go|go\\s+out|go\\s+anywhere|see\\s+(?:(?:my|her|his|the|any|our)\\s+)?(?:own\\s+)?(?:friends|family|mum|mom|dad|mother|father|parents|sister|brother|kids|children|mates|pals|buddies|colleagues|coworkers|anyone|people)|speak\\s+(?:to|with)\\s+(?:(?:my|her|his|the|any)\\s+)?(?:friends|family|mum|mom|dad|mother|father|parents|sister|brother|kids|children|anyone|people)|have\\s+(?:any\\s+|my\\s+own\\s+|the\\s+)?(?:money|wages|bank\\s+card|bank\\s+account|debit\\s+card|keys|a\\s+job|my\\s+own\\s+money|friends|a\\s+life|people\\s+over|visitors|company|a\\s+social\\s+life)|work(?!\\s+(?:from|at|in|late|early|nights?|weekends?|evenings?|overtime|flexibly|remotely|hybrid|part[\\s-]?time|full[\\s-]?time|with|on|for))|study|sleep|get\\s+a\\s+job|look\\s+for\\s+work|drive\\s+(?:the\\s+car|my\\s+car|his\\s+car|her\\s+car|our\\s+car|to\\s+work|to\\s+the\\s+shops|anywhere|myself)|visit\\s+(?:(?:my|her|his|any)\\s+)?(?:friends|family|mum|mom|dad|mother|father))"));
    s_ABUSE_VERB_PART(py_str("(?:hits?|hitting|beat(?:s|ing)?|punch(?:es|ed|ing)?|kick(?:es|ed|ing)?|chok(?:es|ed|ing)|slapp?(?:s|ed|ing)?|smack(?:s|ed|ing)?|drag(?:s|ged|ging)?|strangl(?:e|es|ed|ing)|shov(?:e|es|ed|ing)|spat\\s+at|spit\\s+at|hurl(?:s|ed|ing)?|attack(?:s|ed|ing)?|savaged|mauled|thump(?:s|ed|ing)?|push(?:es|ed|ing)?|whack(?:s|ed|ing)?|head-?butt(?:s|ed|ing)?)"));
    s_ABUSE_HIT_ME(py_tuple(vec![add(&add(&add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b")), &g_ABUSE_VERB()), &py_str("\\s+me\\b")), add(&add(&add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b")), &g_ABUSE_VERB_PART()), &py_str("\\s+(?:my|me\\s+in\\s+my)\\s+(?:face|head|back|arm|arms|leg|legs|ribs?|stomach|belly|jaw|eye|eyes|ear|ears|neck|throat|shoulders?|chest|side|hip|hips|thigh|mouth|nose|teeth|wrist|knee)\\b(?![^.?!]{0,40}\\b(?:cake|cakes|cream|icing|jelly|bun|buns|pudding|pie|pies|flour|bunting|snow|snowdrift|leaves|foam|bubbles|mud|sand|dough|pillow\\s+fight)\\b(?![^.?!]{0,60}\\b(?:hit|hits|punch(?:ed|es)?|kick(?:ed|es)?|slap(?:ped|s)?|beat(?:s)?|chok(?:ed|es)?|shov(?:ed|es)?|push(?:ed|es)?)\\s+me\\b(?!\\s+(?:on|round|in)\\s+the\\s+back\\b))(?![^.?!]{0,60}\\b(?:breath|breathe|breathing|breathless|blacked\\s+out|passed\\s+out|unconscious|choking|couldnt\\s+breathe|could\\s+not\\s+breathe|no\\s+air)\\b))")), add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:scratch(?:es|ed|ing)?|claw(?:s|ed|ing)?)\\s+(?:my|me\\s+in\\s+my)\\s+(?:face|head|arm|arms|leg|legs|ribs?|stomach|belly|jaw|eye|eyes|ear|ears|neck|throat|shoulders?|chest|side|hip|hips|thigh|mouth|nose|teeth|wrist|knee)\\b(?![^.?!]{0,30}\\b(?:on|against|through|by)\\s+(?:the|a|an|some|my|his|her|their|our)?\\s*\\w+)(?![^.?!]{0,40}\\b(?:bleed|bleeds|bled|bleeding|blood|bruise|bruises|bruised|marks|scarred|again|police|scared|afraid|frightened|threaten\\w*|strangl\\w*|choke\\w*|hit|hits|punch\\w*|kick\\w*|beat\\w*)\\b)"))]));
    s_HAS_ME_SUBJECT(add(&add(&py_str("\\b(?:he|she|they|he'?s|she'?s|they'?re|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b")));
    s_GRIP_GAME(py_str("\\b(?:squash|tennis|judo|karate|mma|taekwondo|rugby|football|netball|wrestl\\w*|sparring|dojo|scrum|scrums|matches?)\\b"));
    s_GRIP_GAME_FENCE(add(&add(&py_str("(?![^.?!]{0,30}"), &g_GRIP_GAME()), &py_str(")")));
    s_GRIP_NOT_AN_ASSAULT(py_tuple(vec![g_GRIP_GAME_FENCE(), py_str("(?![^.?!]{0,30}\\b(?:play\\w*|tickl\\w*|laugh\\w*|giggl\\w*|joking|for\\s+fun)\\b)(?![^.?!]{0,30}\\b(?:deadline|deadlines|budget|bills?|rent|mortgage|money|hours|rota|shift|shifts|clock|targets?|emails?|paperwork)\\b)(?![^.?!]{0,30}\\b(?:haircut|while\\s+(?:he|she|they|it)\\s+cut)\\b)")]));
    s_GRIP_NOT_AN_ASSAULT_STR(mcall(&py_str(""), "join", &[g_GRIP_NOT_AN_ASSAULT()]));
    s_GAME_WORD_STOP(py_str("(?!the\\b|a\\b|an\\b|home\\b|house\\b|work\\b|school\\b|night\\b|nights\\b|bed\\b|morning\\b|afternoon\\b|evening\\b|weekend\\b|weekends\\b|front\\b|public\\b|private\\b|kitchen\\b|bathroom\\b|bedroom\\b|hall\\b|stairs\\b|car\\b|street\\b|garden\\b|pub\\b|court\\b|office\\b|hospital\\b|head\\b|face\\b|ribs\\b|back\\b|stomach\\b|chest\\b|arm\\b|arms\\b|leg\\b|legs\\b|neck\\b|throat\\b|gut\\b|kidney\\b|kidneys\\b|hands?\\b|fists?\\b|feet\\b|foot\\b|shoulders?\\b|knees?\\b|elbows?\\b)[\\w'-]+"));
    s_GAME_PLAYED(add(&py_str("\\bbeat(?:s|en|ing)?\\s+(?:me|us)\\s+(?:at|in)\\s+(?:the\\s+|a\\s+)?"), &g_GAME_WORD_STOP()));
    s_GAME_AFTER_ME(add(&add(&py_str("(?!\\s+(?:at|in)\\s+(?:the\\s+|a\\s+)?"), &g_GAME_WORD_STOP()), &py_str(")")));
    s_ASK_AFTER_ME(py_str("(?!\\s+up\\s+(?:for|to)\\b)"));
    s_ABUSE(add(&g_THREAT_COERCIVE(), &py_tuple(vec![add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:holds?|keeps?|pins?|grips?|presses?)\\s+(?:me|us)\\s+(?:up\\s+)?(?:against|into|onto)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:wall|floor|door|bed|ground|fridge|cooker|cupboard|sofa|table)\\b")), g_ABUSE_HIT_ME(), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b(?:thr(?:ew|ows|owing)|hurl(?:ed|s)|lobbed|shov(?:e|es|ed)|push(?:es|ed)|swung|swing|lunged)\\b.{0,25}\\b(?:at|against)\\s+(?:me|my\\s+(?:head|face|arm|back|leg|son|daughter|kids?|child|mum|dad))\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,25}\\b(?:hands?|fingers?|arm)\\s+(?:round|around|on|across)\\s+(?:my\\s+)?(?:throat|neck)\\b")), py_str("\\bput\\s+(?:his|her|their)\\s+hands?\\s+(?:round|around|on)\\s+(?:my\\s+)?(?:throat|neck)\\b"), add(&py_str("\\b(?:held|pinned|pressed)\\s+me\\s+(?:down|against\\s+the\\s+(?:wall|floor|door|bed)|on\\s+the\\s+(?:floor|ground|bed)|in\\s+the\\s+(?:kitchen|bathroom|bedroom|hall|hallway))\\b"), &g_GRIP_NOT_AN_ASSAULT_STR()), add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,25}\\b(?:held|pinned|pressed|pushed|shoved|backed|slammed|threw)\\s+me\\s+(?:up\\s+)?(?:against|onto|into|to)\\s+the\\s+(?:wall|floor|door|bed|fridge|cooker|counter|cupboard|sofa|ground)\\b")), &g_GRIP_GAME_FENCE()), add(&g_HAS_ME_SUBJECT(), &py_str(".{0,20}\\b(?:has|had|have|got|gets|keeps?|kept|holds?|held|leaves?|left|pins?|pinned)\\s+me\\s+(?:pinned|trapped|cornered|locked\\s+in|pressed|held|shoved|pushed|backed|slammed)\\b(?!\\s+(?:to|on|onto|against|into|in|by)\\s+(?:the\\s+|a\\s+|my\\s+|his\\s+|her\\s+|their\\s+|our\\s+)?(?:rota|rotas|shift|shifts|roster|rosters|list|lists|board|boards|schedule|schedules|duty|duties|plan|plans|app|apps|desk|desks|office|meeting\\s+room|conference\\s+room|trolley|till|screen|monitor|keyboard|in-?tray|inbox|deadline|deadlines|shop\\s+floor|checkout|call\\s+list)\\b)")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:corner(?:ed|s)?|trapped|backed|blocked|pinned)\\s+me\\s+(?:in|into)\\s+(?:the\\s+|my\\s+|a\\s+)?(?:kitchen|bedroom|bathroom|bath|hall|hallway|corridor|lift|stairwell|stairs|garage|flat|house|room|doorway|balcony|landing|porch|study|shed)\\b")), add(&add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:wouldn?'?t|won'?t|would|will|refused?|refuses?)\\s*(?:not\\s+)?(?:to\\s+)?(?:let|allow)\\s+me\\s+")), &g__REFUSED_EXIT()), &py_str("\\b")), add(&add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:doesn'?t|don'?t|didn'?t|(?:does|do|did)\\s*not)\\s*let\\s+me\\s+")), &g__REFUSED_EXIT()), &py_str("\\b")), add(&add(&py_str("\\b(?:locked|shut|trapped|stuck)\\s+(?:in|inside)\\s+(?:the\\s+|my\\s+|a\\s+|this\\s+)?(?:bathroom|bedroom|toilet|kitchen|room|flat|house|hall|hallway|car|cellar|basement|loft|attic)\\b[^.?!]{0,40}\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|hammer(?:ing|s)?|kick(?:ing|s)?|smash(?:ing|es)?|breaking|shout(?:ing|s)?|scream(?:ing|s)?|rattl(?:ing|es))\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|hammer(?:ing|s)?|kick(?:ing|s)?|shout(?:ing|s)?|scream(?:ing|s)?)\\b[^.?!]{0,40}\\b(?:locked|shut|trapped|stuck)\\s+(?:in|inside)\\b")), add(&add(&g_HAS_ME_SUBJECT(), &py_str("[^.?!]{0,15}\\b(?:has|have|had|got|keeps?|kept|holds?|held|grabs?|grabbed|drags?|dragged|pulls?|pulled|takes?|took)\\s+me\\s+by\\s+(?:the|my|his|her|their)\\s+(?:throat|neck|hair|scalp|collar|scruff|jaw)s?\\b")), &g_GRIP_NOT_AN_ASSAULT_STR()), add(&add(&g_HAS_ME_SUBJECT(), &py_str("[^.?!]{0,15}\\b(?:has|have|had|got|keeps?|kept|holds?|held|pins?|pinned|presses?|pressed|shoves?|shoved|pushes?|pushed)\\s+me\\s+(?:up\\s+)?(?:against|onto|into|to)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:wall|floor|door|bed|ground|fridge|cooker|cupboard|sofa|counter|table|car)\\b")), &g_GRIP_NOT_AN_ASSAULT_STR()), py_str("\\b(?:shook|shak(?:es|ing))\\s+me\\s+(?:hard|violently|roughly|until|so\\s+hard)\\b"), py_str("\\b(?:grab(?:s|bed|bing)?|yank(?:s|ed|ing)?|twist(?:s|ed|ing)?|wrench(?:es|ed|ing)?|pull(?:s|ed|ing)?)\\s+(?:me\\s+)?by\\s+the\\s+(?:hair|throat|neck|arm|wrist|collar|scruff)\\b"), py_str("\\b(?:grab(?:s|bed|bing)?|yank(?:s|ed|ing)?)\\s+my\\s+(?:arm|wrist|hair|throat|neck|collar)\\b"), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:hits?|hitting|beat|beats|beating|slapp?(?:s|ed)?|punch(?:es|ed)?|kick(?:s|ed)?|struck|thump(?:s|ed)?|whack(?:s|ed)?|smack(?:s|ed)?|belt(?:s|ed)?|cuff(?:s|ed)?)\\b\\s+(?:me\\s+(?:in|on|across|round|about)\\s+)?(?:the|my)\\s+(?:face|head|arm|ribs|stomach|back|throat|neck|jaw|cheek|nose|eye|eyes|leg|hip|shoulder|wrist|chest|side|belly|ear|mouth|thigh|knee)s?\\b")), add(&add(&add(&py_str("\\b(?:he|she|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\bthreaten(?:s|ed|ing)?\\s+me\\b")), &g__NOT_A_THREAT()), add(&add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\bthreaten(?:s|ed|ing)?\\s+(?:me\\b|us\\b|to\\s+(?:kill|hurt|hit|stab)\\s+me)")), &g__NOT_A_THREAT()), add(&add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b(?:said|says|told\\s+me)?\\s*(?:he|she|they)?\\s*(?:would|will|is\\s+going\\s+to|was\\s+going\\s+to)\\s+(?:kill|hurt|stab|rape|strangle|shoot)\\s+me\\b")), add(&add(&py_str("\\b(?:he|she|they)\\s+(?:(?:won'?t|wont|wouldn'?t|wouldnt|didn'?t|doesn'?t|refus(?:es?|ed)\\s+to)\\s+(?:let|allow)|never\\s+lets?)\\s+me\\s+"), &g__REFUSED_EXIT()), &py_str("\\b")), py_str("\\b(?:abusing|abused|abuses)\\s+me\\b"), py_str("\\b(?:sexual(?:ly)?\\s+assault(?:ed)?|raped|raping)\\b"), py_str("\\bhits?\\s+me\\s+(?:when|if|again)\\b"), py_str("\\bafraid\\s+(?:of|to\\s+go)\\s+(?:him|her|them|home|back\\s+home)\\b"), py_str("\\bscared\\s+(?:of|to\\s+go)\\s+(?:him|her|home|back\\s+home|my\\s+(?:husband|partner|dad|mum|boyfriend|girlfriend|carer))\\b"), py_str("\\b(?:locks?|locked)\\s+me\\s+(?:in|out)\\b"), add(&add(&py_str("\\b(?:"), &g_THREAT_SUBJECT()), &py_str("|the\\s+(?:man|woman|lady|guy|bloke))\\b[^.?!]{0,20}\\b(?:trapped|pinned|wedged|locked|shut)\\s+(?:me|us)\\s+(?:in|inside|under|into)\\b")), add(&add(&py_str("\\b(?:he|she|they|my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|dad|mum|mom|father|mother|son|daughter|brother|sister|step(?:dad|mum|mom|father|mother|son|daughter|brother|sister)|carer|boy|girl|nan|nana|gran|granny|uncle|aunt|auntie|cousin|in-?laws?))\\s+(?:won'?t|wont|wouldn'?t|wouldnt|refus(?:es?|ed)\\s+to|refuse\\s+to|never\\s+lets?\\s+me|didn'?t\\s+let|doesn'?t\\s+let|did\\s+not\\s+let)\\s+let\\s+me\\s+"), &g__REFUSED_EXIT()), &py_str("\\b")), py_str("\\bthreatened?\\s+(?:to\\s+kill|me\\s+with)\\b"), add(&add(&add(&py_str("\\b(?:(?:he|she|they|someone|somebody|my\\s+"), &g_CLOSE()), &py_str(")'?s|they'?re|is|are|keeps?|has\\s+been|have\\s+been)\\s+threatening\\s+(?:me|us)\\b")), &g__NOT_A_THREAT()), py_str("\\b(?:i'm|i\\s+am)\\s+(?:being\\s+)?(?:controlled|coerced)\\b"), add(&add(&py_str("\\b(?:i'm|i\\s+am|im|i\\s+was|i\\s+have\\s+been|i've\\s+been|ive\\s+been|i\\s+keep\\s+being|i\\s+keep\\s+getting)\\s+(?:being\\s+|getting\\s+)?(?:abused|threatened|stalked|followed|watched|harassed|hit|attacked|assaulted|raped|molested|groomed|beaten\\s+up)\\b"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT()), add(&add(&py_str("\\b(?:i'm|i\\s+am|im)\\s+(?:being|getting)\\s+(?:choked|strangled)\\b"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT()), add(&add(&py_str("\\b(?:i\\s+was|i\\s+have\\s+been|i've\\s+been|ive\\s+been|i\\s+keep\\s+(?:being|getting))\\s+(?:being\\s+|getting\\s+)?(?:choked|strangled)\\b(?!\\s+(?:about|over)\\b)"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT()), add(&py_str("\\b(?:he|she|they|my\\s+\\w+)'?(?:s|ve)?\\s+(?:been\\s+)?(?:hurting|hitting|strangling)\\s+me\\b"), &g_ASK_AFTER_ME()), add(&add(&py_str("\\b(?:he|she|they|my\\s+\\w+)'?(?:s|ve)?\\s+(?:been\\s+)?beating\\s+me\\b"), &g_GAME_AFTER_ME()), &g_ASK_AFTER_ME()), py_str("\\bmy\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer)\\s+is\\s+(?:being\\s+)?(?:abusive|violent)\\b|\\bmy\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer)\\s+is\\s+(?:controlling|coercive)\\b"), py_str("\\b(?:he|she|they)'?s?\\s+(?:being|is|keeps?\\s+being)\\s+(?:abusive|violent)\\b"), py_str("\\b(?:he|she|they)\\s+is\\s+being\\s+(?:abusive|violent)\\b"), py_str("\\bmy\\s+(?:violent|abusive|controlling|coercive|dangerous|aggressive|threatening)\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer|son|daughter|brother|sister|stepdad|stepdad|stepmum|stepfather|stepmother)\\b"), add(&add(&add(&py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person)|the\\s+(?:man|woman|person))\\b[^.?!]{0,25}\\b"), &g_ABUSE_VERB()), &py_str("\\s+me\\b")), &g_HIT_BY_A_VEHICLE()), add(&py_str("\\b(?:someone|somebody)\\b[^.?!]{0,25}\\bhurt(?:s|ing)?\\s+me\\b"), &g_HIT_BY_A_VEHICLE()), py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person|partner)|the\\s+(?:man|woman|person|partner))\\b[^.?!]{0,25}\\b(?:is\\s+|was\\s+|being\\s+|been\\s+)?(?:abusive|violent|controlling|coercive|dangerous|aggressive)\\b"), py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person)|the\\s+(?:man|woman|person))\\b[^.?!]{0,25}\\b(?:scares|frightens|terrifies|intimidates)\\s+me\\b"), py_str("\\b(?:i|we|he|she|they)\\s+(?:was|were|got|just\\s+got)\\s+(?:attacked|assaulted|mugged|beaten\\s+up)\\b"), py_str("\\b(?:i'm|i\\s+am)\\s+(?:scared|terrified)\\s+of\\s+(?:going\\s+home|him|her)\\b"), py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad)|(?:the|our|my)\\s+neighbours?|next\\s+door)\\b[^.?!]{0,30}\\bthreaten(?:s|ed|ing)?\\s+(?:me\\b|to\\s+(?:kill|hurt|harm|hit|stab|beat)\\s+me)")])));
    s_PARTNER_CLOSE(py_str("(?:ex|exes|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|baby\\s*daddy|baby\\s*dad|baby\\s*mother|baby\\s*mum)"));
    s_PURSUIT_SUBJECT(py_tuple(vec![add(&add(&py_str("\\b(?:my\\s+"), &g_PARTNER_CLOSE()), &py_str("|my\\s+ex'?s?)\\b"))]));
    s_NOT_A_FEAR(py_str("(?!\\s+(?:that\\s+)?(?:hes|shes|theyre|its|it'?s|im|i'?m|ill|i'?ll|hell|well|going\\s+to\\s+be|we\\s+will|we'?ll)\\s*(?:going\\s+to\\s+be\\s+)?(?:late|late\\s+home|in\\s+trouble|on\\s+time))"));
    s_PURSUIT_FEAR(py_tuple(vec![add(&py_str("\\b(?:i'?m|i\\s+am|im)\\s+(?:(?:really|so|very|that|proper|properly|absolutely|dead|scared\\s+witless)\\s+)*(?:scared|frightened|terrified|afraid|panicking|shaking|worried)\\b"), &g_NOT_A_FEAR()), py_str("\\b(?:scared|frightened|terrified|afraid)\\s+(?:of|to\\s+go|to\\s+leave|to\\s+open)\\b")]));
    s_PURSUIT_SUBJECT_EX(py_tuple(vec![py_str("\\b(?:my\\s+(?:ex|exes|baby\\s*daddy|baby\\s*dad|baby\\s*mother|baby\\s*mum)|my\\s+ex'?s?)\\b")]));
    s_PURSUIT_OUTSIDE(py_tuple(vec![py_str("\\b(?:waiting|waits|sat|sitting|stood|standing|parked|lurking|hiding|hovering|hanging\\s+around)\\s+(?:right\\s+|still\\s+|now\\s+)?(?:outside|out\\s+front|in\\s+the\\s+car\\s+outside|round\\s+the\\s+back)\\b"), py_str("\\bwait(?:ing|s|ed)?\\s+for\\s+(?:me|us)\\b")]));
    s_PURSUIT_PLACE_WORK(py_tuple(vec![py_str("\\b(?:outside|in\\s+front\\s+of|at\\s+the\\s+end\\s+of|round\\s+the\\s+back\\s+of)\\s+(?:my|our|the)\\s+(?:work|workplace|office|school|nursery|college|uni|university|gates)\\b"), py_str("\\b(?:waiting|waits|sat|sitting|stood|standing|parked|lurking|hiding|hovering|hanging\\s+around)\\s+(?:right\\s+|still\\s+|now\\s+)?(?:outside|near)\\s+(?:my|our|the)\\s+(?:work|workplace|office|school|nursery|college|uni|university|gates)\\b")]));
    s_PURSUIT_LOUD(py_tuple(vec![py_str("\\b(?:shouting|yelling|screaming)\\s+at\\s+(?:the\\s+)?(?:football|telly|tv|match|game|radio|news|kids|dog|cat|ref|referee)\\b(?!.*\\b(?:i'?m|i\\s+am|im)\\s+(?:scared|terrified))"), py_str("\\b(?:shouting|yelling|screaming|drunk|hammering|banging|pounding)\\b"), py_str("\\b(?:won'?t|wont|refuses?\\s+to)\\s+(?:leave|go|move)\\b")]));
    s_PURSUIT(py_tuple(vec![_both_here(&[g_PURSUIT_SUBJECT(), crate::gen::modules::safety_cards::g_PURSUIT_PLACE_HOME()]), _both_here(&[g_PURSUIT_SUBJECT(), py_tuple(vec![py_str("\\b(?:is|was|keeps?|still)\\s+(?:right\\s+)?(?:outside|here|back)\\b")]), add(&g_PURSUIT_FEAR(), &g_PURSUIT_LOUD())]), _both_here(&[g_PURSUIT_SUBJECT_EX(), g_PURSUIT_PLACE_WORK()]), _both_here(&[g_PURSUIT_SUBJECT_EX(), g_PURSUIT_OUTSIDE()]), py_str("\\b(?:he|she|they|hes|shes|theyre)'?s?\\b[^.?!]{0,12}\\b(?:follow(?:s|ing|ed)?|stalk(?:s|ing|ed)?)\\s+(?:me|us)\\b")]));
    s_ABUSE_VERBAL_PERSON(py_str("(?:he|she|they|hubby|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|ex|mum|mom|mam|mother|dad|father|stepdad|stepmum|stepmom|stepfather|stepmother|nan|nana|gran|granny|grandma|grandad|grandpa|carer)"));
    s_ABUSE_VERBAL_MAD(py_str("(?:mental|crazy|mad)(?!\\s+(?:(?:about|over)\\s+(?!everything|anything|every\\s+thing|all\\s+(?:of|the)|whatever|how\\s+i|everyone|every\\s+time|being|doing|saying|wanting)|at\\s+(?!the\\s+time|first|least)|in\\s+(?:love|bed|a\\s+good\\s+way|the\\s+good\\s+way|a\\s+kind\\s+way)))"));
    s_ABUSE_VERBAL_LABEL(add(&add(&py_str("(?:names|stupid|thick|dumb|useless|worthless|lazy|fat|ugly|pathetic|hopeless|a\\s+failure|a\\s+loser|a\\s+waste\\s+of\\s+space|a\\s+waste|nothing|"), &g_ABUSE_VERBAL_MAD()), &py_str("|psycho|disgusting|a\\s+slag|a\\s+slut|a\\s+bitch|a\\s+cow|rubbish|a\\s+disappointment|a\\s+mistake|an\\s+idiot|an\\s+imbecile|stupid\\s+cow|fat\\s+cow|silly\\s+cow|cretin|a\\s+joke|unloveable|unlovable|a\\s+burden|a\\s+drain|a\\s+piece\\s+of\\s+shit|a\\s+piece\\s+of\\s+crap|shit)")));
    s_CHILD_AGGRESSOR(slice_or_raise(&py_tuple(vec![py_str("\\b(?:(?:my|our|his|her|their|the)\\s+(?:son|sons|daughter|daughters|child|children|kid|kids|toddler|baby|teen|teenager|teenagers|boy|girl|boys|girls|little\\s+one)|my\\s+(?:eldest|oldest|youngest))\\b(?!\\s+[^.?!]{0,12}\\b(?:when|while|as)\\s+(?:he|she|they)\\b)[^.?!]{0,12}\\b(?:pushes?|pushed|pushing|hits?|hit|hitting|kicks?|kicked|kicking|slaps?|slapped|slapping|bites?|bit|biting|punches?|punched|punching|calls?\\s+me|calling\\s+me|called\\s+me|swears?\\s+at\\s+me|shouts?\\s+at\\s+me|yells?\\s+at\\s+me)\\b")]), &Py::None, &Py::Int(1), &Py::None));
    s_ABUSE_VERBAL_SUBJECT(add(&add(&py_str("\\b(?:he|she|they|he'?s|she'?s|they'?re|my\\s+"), &g_ABUSE_VERBAL_PERSON()), &py_str(")\\b")));
    s_ABUSE_VERBAL(py_tuple(vec![add(&add(&add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,25}\\b(?:calls?|calling|called|keeps\\s+calling)\\s+me\\s+(?:(?:a|an)\\s+)?")), &g_ABUSE_VERBAL_LABEL()), &py_str("\\b")), add(&add(&add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,25}\\b(?:tells?|telling|told|says?|saying|said)\\s+(?:me\\s+)?i(?:'?m|\\s+am)\\s+(?:(?:a|an)\\s+)?")), &g_ABUSE_VERBAL_LABEL()), &py_str("\\b")), add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,30}\\b(?:keeps?|kept|always|constantly|regularly|never\\s+stops|every\\s+(?:day|night|week))\\b[^.?!]{0,20}\\b(?:putting\\s+me\\s+down|put\\s+me\\s+down|running\\s+me\\s+down|having\\s+a\\s+go\\s+at\\s+me)\\b")), add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,30}\\b(?:puts?\\s+me\\s+down|putting\\s+me\\s+down|runs?\\s+me\\s+down|running\\s+me\\s+down|has\\s+a\\s+go\\s+at\\s+me|having\\s+a\\s+go\\s+at\\s+me)\\b[^.?!]{0,25}\\b(?:all\\s+the\\s+time|all\\s+day|constantly|always|every\\s+(?:day|night|week|time)|non[\\s-]?stop)\\b"))]));
    s_ABUSE_VERBAL_FINE(py_tuple(vec![py_str("\\b(?:call|calls|calling|called)\\s+each\\s+other\\b"), py_str("\\b(?:we|we'?re|were)\\s+(?:both\\s+)?(?:call|calls|calling|tease|teasing|wind)\\b"), py_str("\\b(?:me\\s+and|between\\s+me\\s+and|my\\s+(?:mates?|friends?|lads|colleagues?|team|classmates?|schoolmates?|flatmates?|housemates?))\\b[^.?!]{0,20}\\b(?:call|calls|calling|tease|teasing|rip\\s+into)\\b"), py_str("\\b(?:nickname|nicknames|term\\s+of\\s+endearment|pet\\s+name|affectionately|fondly)\\b"), py_str("\\b(?:in\\s+fun|for\\s+a\\s+laugh|in\\s+good\\s+fun|in\\s+jest|in\\s+play|at\\s+five-a-side|on\\s+the\\s+pitch|at\\s+rugby|at\\s+football|in\\s+training)\\b")]));
    s_ABUSE_HEAD_INTO(add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)|slam(?:med|s)?|bang(?:ed|s|ing)?|smash(?:ed|es|ing)?|knock(?:ed|s|ing)?|ram(?:med|s|ming)?|thr(?:ew|ows|owing)|yank(?:ed|s|ing)?|drag(?:ged|s|ging)?|smack(?:ed|s|ing)?|press(?:ed|es|ing)?)\\b[^.?!]{0,15}\\bmy\\s+(?:head|face)\\b[^.?!]{0,15}\\b(?:into|onto|against|off|through)\\b\\s*(?:the\\s+|a\\s+|his\\s+|her\\s+)?(?:wall|walls|door|doors|door\\s?frame|doorframe|frame|floor|ground|table|table\\s?top|desk|worktop|counter|cupboard|cabinet|fridge|freezer|radiator|banister|stairs|stair\\s?gate|step|kerb|curb|pavement|window|windowsill|sill|sink|bath|bathroom|toilet|headboard|bed\\s?frame|wardrobe|shelf|shelves|cooker|oven|dashboard|car|van|kitchen\\s?unit|brick\\s?wall|concrete)\\b")));
    s_SOFT_FACE_PUSH(add(&add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)|slam(?:med|s)?|bang(?:ed|s|ing)?|smash(?:ed|es|ing)?|ram(?:med|s|ming)?|thr(?:ew|ows|owing)|dunk(?:ed|s|ing)?)\\b[^.?!]{0,15}\\bmy\\s+(?:face|head)\\b[^.?!]{0,15}\\b(?:into|onto|in|through)\\b\\s*(?:(?:the|a|his|her|my|their)\\s+)?(?:\\w+\\s+){0,2}(?:cake|cakes|snow|mud|sand|dough|flour|icing|bubbles|foam|paint|slime)\\b")));
    s_SOFT_FACE_STOP(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|punch(?:ed|es|ing)?|kick(?:ed|es|ing)?|slap(?:ped|s|ping)?|beat(?:s|en)?|chok(?:ed|es|ing)?|shov(?:ed|es)?\\s+me|push(?:ed|es)?\\s+me|held\\s+me|hold(?:ing)?\\s+me|kept\\s+me|held\\s+it|pinned)\\b"), py_str("\\b(?:can'?t|couldn'?t|could\\s+not|cannot|unable\\s+to|struggl\\w*\\s+to)\\s+breath\\w*\\b"), py_str("\\b(?:gasping|choking|unconscious|blue\\s+in\\s+the\\s+face)\\b")]));
    s_PLAY_FACE_REPLIES(py_tuple(vec![py_str("A proper mess. Were you both laughing, or did it land badly?"), py_str("Ha - somebody got you. Was it a laugh, or did that land badly?"), py_str("That's the classic. Were you both laughing, or did it not feel funny to you?")]));
    s_ABUSE(add(&add(&add(&g_ABUSE(), &g_PURSUIT()), &g_ABUSE_VERBAL()), &py_tuple(vec![g_ABUSE_HEAD_INTO()])));
    s_ABUSE_PURSUIT_HARD(add(&add(&py_tuple(vec![add(&add(&py_str("\\b"), &g_ABUSE_VERB()), &py_str("\\s+(?:me|us)\\b"))]), &g_PURSUIT_FEAR()), &py_tuple(vec![py_str("\\b(?:drunk|threatening|hit|hitting|kicked|pushed|grabbed)\\b"), py_str("\\b(?:won'?t|wont|refuses?)\\s+(?:leave|go|let)\\b")])));
    s_ABUSE_DISCLOSURE(py_tuple(vec![py_str("\\b(?:i|we)\\s+(?:was|were|have\\s+been|'ve\\s+been|had\\s+been)\\s+(?:being\\s+|getting\\s+)?(?:abused|assaulted|raped|molested|groomed|stalked|harassed|threatened|beaten|hit|attacked|choked|strangled)\\b"), py_str("\\b(?:i|we)\\s+(?:was|were)\\s+in\\s+an?\\s+(?:abusive|violent)\\s+(?:relationship|marriage|home)\\b"), py_str("\\b(?:i|we)\\s+used\\s+to\\s+be\\s+(?:beaten|hit|abused|assaulted)\\b")]));
    s_ABUSE_DISCLOSURE_FINE(py_tuple(vec![py_str("\\bthreatened\\s+(?:with|over|about)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:being\\s+)?(?:written\\s+)?(?:warning|fine|fines|eviction|legal|lawsuit|court|police|report|complaint|solicitor|letter|bills?|charges?|prosecution|dismissal|sack|sack(?:ing|ed)|redundancy|redundant|fired|firing|disciplinary|hearing|notice|tenancy|deposit)\\b")]));
    s_ABUSE_MEMORY(py_tuple(vec![py_str("\\bas\\s+a\\s+(?:child|kid|baby|boy|girl|teenager|teen|little\\s+(?:boy|girl|kid|one))\\b"), py_str("\\bgrowing\\s+up\\b"), py_str("\\bin\\s+my\\s+(?:childhood|teens|twenties|thirties)\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:a\\s+(?:child|kid|baby|teenager|little\\s+\\w+)|young|little|small|at\\s+school|in\\s+school|at\\s+college|at\\s+university)\\b"), py_str("\\bat\\s+(?:school|primary|secondary|college|university)\\b"), py_str("\\bat\\s+my\\s+(?:old|previous|last|former)\\s+(?:job|work|school|house|flat)\\b"), py_str("\\byears\\s+and\\s+years\\s+ago\\b"), py_str("\\byears\\s+ago\\b"), py_str("\\bages\\s+ago\\b"), py_str("\\bdecades?\\s+ago\\b"), py_str("\\ba\\s+long\\s+(?:time|while)\\s+ago\\b"), py_str("\\bback\\s+(?:then|in\\s+(?:the\\s+)?(?:\\d0s|\\d{4}))\\b"), py_str("\\bin\\s+the\\s+\\d0s\\b"), py_str("\\bused\\s+to\\b"), py_str("\\bmy\\s+(?:childhood|past)\\b")]));
    s_ABUSE_NOW(py_tuple(vec![py_str("\\b(?:still|now|again)\\s+(?:\\w+\\s+){0,2}(?:does|doing|hits?|hurts?\\s+me|beats?|threatens?|follows?|watches?|comes?|turns?\\s+up|shows?\\s+up|calls?|texts?|happens?|happening|at\\s+it)\\b"), py_str("\\b(?:keeps?\\s+(?:doing|on|at)|hasn'?t\\s+stopped|never\\s+stopped|carries\\s+on)\\b"), py_str("\\b(?:it|this|he|she)\\s*(?:'s|s|\\s+is)?\\s*(?:happening|going\\s+on|happened)\\s+again\\b"), py_str("\\bat\\s+it\\s+again\\b"), py_str("\\b(?:every|each)\\s+(?:day|night|morning|week|evening)\\b"), py_str("\\b(?:nowadays|these\\s+days)\\b"), py_str("\\b(?:tonight|yesterday|last\\s+night|this\\s+(?:morning|week))\\b")]));
    s_BLOW_THINGS(py_str("(?:bottles?|glasses?|pans?|frying\\s+pans?|saucepans?|bats?|baseball\\s+bats?|belts?|belt\\s+buckles?|chairs?|stools?|cups?|mugs?|plates?|bowls?|knife|knives|sticks?|canes?|hammers?|bricks?|stones?|rocks?|remotes?|remote\\s+controls?|shoes?|boots?|slippers?|trainers?|books?|newspapers?|candlesticks?|spanners?|wrenches?|wires?|cables?|cords?|pipes?|rods?|bars?|iron\\s+bars?|rolling\\s+pins?|ladles?|spoons?|wooden\\s+spoons?|forks?|pokers?|torches?|phones?|lamps?|vases?|ashtrays?|tools?|hands?|fists?|feet|foot|elbows?|knees?|shoulders?)"));
    s_SOFT_HIT_THING(py_str("(?:pillow|cushion|duvet|blanket|quilt|snowball(?:s)?|ball(?:s)?|beach\\s+ball|plastic\\s+ball|tennis\\s+ball|frisbee|sponge|flannel|tea\\s+towel|towel|newspaper|magazine|paper|roll?ed[\\s-]?up\\s+(?:paper|newspaper|magazine|towel)|water\\s?bomb|water\\s?balloon|water\\s+pistol|sock|slipper|plush\\s+toy|teddy|toy|stickle\\s?brick|lego|balloon|cushion\\s+case)"));
    s_PLAY_NAMED(py_str("(?:pillow\\s+fight|snowball\\s+fight|water\\s+fight|food\\s+fight|play\\s?fight\\w*|nerf\\s+(?:gun|war|fight)|paintball|tickl\\w+|muck(?:ing)?\\s+(?:about|around)|mess(?:ing)?\\s+(?:about|around)|fool(?:ing)?\\s+(?:about|around)|lark(?:ing)?\\s+(?:about|around)|jok(?:ing|e)\\s+(?:about|around)|banter|we\\s+(?:were|was|are)\\s+(?:only\\s+|just\\s+)?(?:playing|mucking|messing|fooling|larking|joking|having\\s+a\\s+laugh)|we\\s+both\\s+(?:laughed|were\\s+laughing|laugh)|we\\s+were\\s+laughing|playing\\s+(?:catch|football|rounders|netball|basketball|tag|a\\s+game|games|together))"));
    s_SOFT_HIT_VERB(py_str("(?:hit|hits|hitting|slapped|slaps|smacked|whacked|pelted|threw|throws|thrown|sprayed|squirted|flicked|dunked|dumped|doused)"));
    s_ABUSE_FINE(py_tuple(vec![py_str("\\bbeat(?:s|en|ing)?\\s+(?:me|us)\\s+(?:at|in)\\s+(?:chess|cards|poker|pool|snooker|billiards|darts|tennis|badminton|table\\s+tennis|ping\\s?pong|squash|golf|football|rugby|cricket|basketball|netball|scrabble|monopoly|cluedo|checkers|draughts|dominoes|mario\\s?kart|video\\s+games?|the\\s+quiz|the\\s+game|the\\s+final|the\\s+match|the\\s+league|arm\\s?wrestl\\w*)\\b"), g_GAME_PLAYED(), py_str("\\bhit(?:s|ting)?\\s+(?:me|us)\\s+up\\s+(?:for|to)\\b"), py_str("\\b(?:had|has|have|got|gets|kept|keeps)\\s+me\\s+cornered\\s+(?:at|during|after)\\s+(?:the\\s+|a\\s+|his\\s+|her\\s+)?(?:party|pub|wedding|conference|meeting|office|work|do|dinner|reception|barbecue|bbq|gig|match|networking\\s+event)\\b"), py_str("\\bcornered?\\s+me\\s+(?:to\\s+(?:talk|ask|discuss|pitch|sell|persuade|convince|apologise|apologize)|at\\s+(?:the\\s+|a\\s+)?(?:party|pub|wedding|conference|meeting|office|work|do))\\b"), py_str("\\bmy\\s+(?:mum|mom|mam|dad|mother|father|parents?|nan|nana|grandad|grandma)\\b[^.?!]{0,25}\\b(?:let|allow)\\s+me\\s+(?:go|out|stay|leave)\\b"), py_str("\\b(?:landlord|landlady|letting\\s+agent|agent|council|employer|boss|manager|company|firm|university|college|school|gym)\\b[^.?!]{0,30}\\b(?:let|allow)\\s+me\\s+(?:leave|go|out|past)\\b"), py_str("\\b(?:debt\\s+collector|debt\\s+collectors|bailiffs?|bank|mortgage|lender|utility|utilities|energy|water\\s+company|broadband|phone\\s+company|insurer|insurance|hmrc|tax\\s+office|dvla)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\s+(?:me|us)\\b"), py_str("\\b(?:landlord|letting\\s+agent|agent|council|housing|bank|mortgage|utility|utilities|energy|water\\s+company|broadband|phone\\s+company|mobile\\s+network|insurer|insurance|employer|boss|manager|company|firm|agency|payroll|hr\\b|hmrc|tax\\s+office|dvla|university|college|school|gym|debt\\s+collector|solicitor)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b[^.?!]{0,25}\\b(?:over|about|with)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:money|pay|wages|salary|rent|hours|shifts?|rota|arrears|fees?|charges?|debt|bills?)\\b"), py_str("\\bhit\\s+(?:on|with)\\b"), py_str("\\bfeel(?:s|ing)?\\s+like\\s+(?:i\\s*(?:'m|am)|im)\\s+being\\s+watched\\b"), py_str("\\bharass(?:ed|es|ing|ment)\\b[^.?!]{0,30}\\b(?:at|from)\\s+(?:work|the\\s+office|my\\s+job)\\b"), py_str("\\b(?:thr(?:ew|ows)|push(?:ed|es)|shov(?:ed|es)|kick(?:ed|s)?|hit|hits|slapped)\\b(?!\\s+(?:my|his|her|their)\\b)(?!\\s+me\\s+(?:in|on|round|across)\\b)[^.?!]{0,20}\\b(?:ball|snowball|frisbee|water\\s+balloon|pillow|toy|party|surprise|birthday|early\\s+night|hint|look)\\b"), py_str("\\b(?:push(?:ed|es)|shov(?:ed|es)|thr(?:ew|ows))\\s+me\\s+(?:into|in)\\s+the\\s+(?:pool|water|sea|lake|river|snow|leaves)\\b"), py_str("\\b(?:push(?:ed|es)|shov(?:ed|es))\\s+me\\s+(?:in|on)\\s+the\\s+(?:buggy|pushchair|wheelchair|swing|sledge|sled|trolley|bike|boat)\\b"), py_str("\\b(?:my|his|her)\\s+(?:sister|brother|friend|mate|son|daughter|kids?)\\s+(?:hit|hits|punched|pushed|kicked|slapped)\\s+me\\s+(?:at|during|in)\\s+(?:rugby|football|the\\s+match|training|practice|a\\s+game|the\\s+game|playtime|the\\s+pool)\\b"), py_str("\\bmy\\s+(?:son|daughter|child|children|kid|kids|baby|babies|toddler|nephew|niece|grandson|granddaughter)\\b[^.?!]{0,40}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|kick(?:ing|s)?|hammer(?:ing|s)?|shout(?:ing|s)?)\\b"), py_str("\\bmy\\s+(?:son|daughter|niece|nephew|godson|goddaughter)\\b[^.?!]{0,25}\\b(?:(?:has|have|had|got|keeps?|kept)\\s+me\\s+pinned|pinned\\s+me|wrestl(?:ed|es|ing)\\s+me)\\b"), py_str("\\bmy\\s+(?:kid|kids|child|children|toddler|baby|babies|newborn)\\b[^.?!]{0,25}\\b(?:(?:has|have|had|got|keeps?|kept)\\s+me\\s+(?:pinned|trapped|stuck)|(?:pinned|trapped|wrestl(?:ed|es|ing))\\s+me)\\b"), py_str("\\b(?:grab(?:bed|s)?|held|took)\\b[^.?!]{0,25}\\bto\\s+(?:hug|kiss|say|shake|stop|help|save|pull\\s+me\\s+up|hold|steady|catch|guide|lead|thank|greet|calm)\\b"), py_str("\\b(?:as\\s+(?:i|he|she)\\s+(?:fell|slipped))\\b"), add(&add(&py_str("\\bhit(?:s|ting)?\\s+me\\s+with\\s+(?!(?:(?:the|a|an|my|his|her|their|this|that)\\s+)?(?:washing|clothes|clothesline|fishing|tow|washing-up|party|garden|climbing)\\s*-?\\s*line\\b)(?:(?:the|a|an|this|that|his|her|their)\\s+)?(?:[\\w'-]+\\s+){0,2}(?:next\\s+)?(?:deadline|deadlines|bill|bills|news|invoice|invoices|fine|tax|taxes|work|project|projects|question|questions|idea|ideas|surprise|party|joke|jokes|pun|punchline|line|lines|quote|quotes|fact|facts|truth|hint|warning|newsflash|update|updates|headline|headlines|statistic|statistics|stats|figure|figures|number|numbers|story|stories|riddle|suggestion|suggestions|advice|compliment|compliments|argument|excuse|excuses|ultimatum|guilt\\s+trip|lecture|earful|mouthful|reality\\s+check|curveball|revelation|rumour|rumor|gossip|opinion|promise|invite|invitation|letter|email|text|phone\\s+call)\\b(?!\\s+of\\s+(?:punches|blows|kicks|hits|slaps|thumps|digs|jabs|beatings|whacks|wallops|shots|smacks|strikes|knocks)\\b)(?!\\s+(?:opener|book|books|plate|plates|sheet|sheets|skate|skates|prop|props|board|boards|rack|stand|clip|gun|punch|hole|weight|pin|block|cutter|knife|bat|pad|bag|cover|holder|rest|frame|case|box|bag|handle|head|piece|booklet|boards?)\\b)(?!(?:[^.?!]{0,40})\\b(?:and|then|but|so)\\b[^.?!]{0,30}(?:hit|hits|hitting|punched|punching|slapped|slapping|kicked|kicking|shoved|shoving|pushed|pushing|grabbed|grabbing|choked|choking|strangled|beat|beating|thumped|whacked|headbutted|pinned|dragged|smashed)\\s+(?:me|my|us)\\b(?!\\s+(?:with|by)\\b))(?![^.?!]{0,40}\\b(?:keeps?|kept)\\s+(?:on\\s+)?(?:hitting|punching|slapping|kicking|shoving|pushing|grabbing|choking|beating|thumping|whacking)\\s+(?:me|my|us)\\b)(?![^.?!]{0,30}\\b(?:and|then|or|but)\\b[\\s,]+(?:(?:then|also|with|by)\\s+)?(?:(?:a|an|the|his|her|their|my|our)\\s+)?"), &g_BLOW_THINGS()), &py_str("\\b(?!\\s+(?:of|on|round|around|to)\\b))")), py_str("\\b(?:bailiffs?|debt\\s+collectors?)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b"), py_str("\\b(?:playfully|jokingly|in\\s+play|for\\s+fun|messing\\s+about|playfight\\w*|larking\\s+about|tickl\\w+)\\b"), py_str("\\b(?:muck(?:ing)?\\s+(?:about|around)|mess(?:ing)?\\s+(?:about|around)|fool(?:ing)?\\s+(?:about|around)|lark(?:ing)?\\s+(?:about|around)|jok(?:ing|e)\\s+(?:about|around)|muck(?:ing)?\\s+around)\\b"), add(&add(&add(&add(&py_str("\\b"), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,24}\\b(?:my|the|his|her|your)\\s+(?:head|face)\\b[^.?!]{0,24}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b")), add(&add(&add(&add(&add(&add(&py_str("\\b"), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,40}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b[^.?!]{0,60}")), &g_PLAY_NAMED()), &py_str("\\b")), add(&add(&add(&add(&add(&add(&py_str("\\b"), &g_PLAY_NAMED()), &py_str("\\b[^.?!]{0,60}\\b")), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,40}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b")), add(&add(&add(&add(&py_str("\\b"), &g_PLAY_NAMED()), &py_str("\\b[^.?!]{0,60}\\b(?:threw|throw|thrown|throws|pelted|sprayed|squirted)\\b[^.?!]{0,20}\\b")), &g_SOFT_HIT_THING()), &py_str("\\s+at\\s+(?:me|us)\\b")), py_str("\\bmy\\s+(?:son|daughter|child|kid|baby|toddler|boy|girl|children|stepson|stepdaughter|grandson|granddaughter|nephew|niece|little\\s+one)\\b[^.?!]{0,30}\\b(?:bit|bite|bites|biting)\\s+me\\b"), py_str("\\bmy\\s+(?:baby|toddler|child|kid|little\\s+one|newborn|infant|two\\s+year\\s+old|three\\s+year\\s+old)\\b[^.?!]{0,30}\\b(?:hit|hits|hitting|kicked|kicks|kicking|slapped|slaps|slapping|pinched|pinches|pinching|scratched|scratches|scratching|pushed|pushes|shoved|shoves|headbutted|headbutts)\\s+me\\b"), py_str("\\b(?:they|someone)\\s+(?:are|is|was|were)?\\s*(?:threatening|threatened)\\s+me\\b[^.?!]{0,30}\\b(?:debt|debts|money|loan|loans|repayment|repayments|bill|bills)\\b"), py_str("\\b(?:boss|manager|colleague|coworker|co-worker|supervisor)\\b[^.?!]{0,30}\\bthreaten(?![^.?!]{0,60}\\b(?:kill|kills|killing|murder|hurt|hurts|harm|attack|stab|shoot|strangle|choke|rape|beat|punch|hit|slap|grab|shove|push|throat|weapon|knife|gun)\\b)"), py_str("\\bthreaten(?![^.?!]{0,60}\\b(?:kill|kills|killing|murder|hurt|hurts|harm|attack|stab|shoot|strangle|choke|rape|beat|punch|hit|slap|grab|shove|push|throat|weapon|knife|gun)\\b)[^.?!]{0,30}\\b(?:boss|manager|colleague|coworker|co-worker|supervisor)\\b"), py_str("\\b(?:held|pulled|pushed|shoved)\\s+(?:me|us)\\s+(?:against|to|back\\s+against)\\s+(?:the\\s+)?(?:wall|fence|barrier|side|back)\\b[^.?!]{0,25}\\b(?:to\\s+let|so\\s+they|so\\s+the|to\\s+make\\s+room|to\\s+get\\s+out\\s+of\\s+the\\s+way)\\b")]));
}
pub fn _both_here(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut groups: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut group: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("One thing from each group, all in the line, in whichever order.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&groups.clone()) { let mut group: Py = __item.clone(); __out.push(add(&add(&py_str("(?=.*(?:"), &mcall(&py_str("|"), "join", &[group.clone()])), &py_str("))"))); } py_list(__out) })() };
    return mcall(&py_str(""), "join", &[parts.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.safety_phrases".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABUSE" => Some(g_ABUSE()),
            "ABUSER" => Some(g_ABUSER()),
            "ABUSE_DISCLOSURE" => Some(g_ABUSE_DISCLOSURE()),
            "ABUSE_DISCLOSURE_FINE" => Some(g_ABUSE_DISCLOSURE_FINE()),
            "ABUSE_FINE" => Some(g_ABUSE_FINE()),
            "ABUSE_HEAD_INTO" => Some(g_ABUSE_HEAD_INTO()),
            "ABUSE_HIT_ME" => Some(g_ABUSE_HIT_ME()),
            "ABUSE_MEMORY" => Some(g_ABUSE_MEMORY()),
            "ABUSE_NOW" => Some(g_ABUSE_NOW()),
            "ABUSE_PURSUIT_HARD" => Some(g_ABUSE_PURSUIT_HARD()),
            "ABUSE_VERB" => Some(g_ABUSE_VERB()),
            "ABUSE_VERBAL" => Some(g_ABUSE_VERBAL()),
            "ABUSE_VERBAL_FINE" => Some(g_ABUSE_VERBAL_FINE()),
            "ABUSE_VERBAL_LABEL" => Some(g_ABUSE_VERBAL_LABEL()),
            "ABUSE_VERBAL_MAD" => Some(g_ABUSE_VERBAL_MAD()),
            "ABUSE_VERBAL_PERSON" => Some(g_ABUSE_VERBAL_PERSON()),
            "ABUSE_VERBAL_SUBJECT" => Some(g_ABUSE_VERBAL_SUBJECT()),
            "ABUSE_VERB_PART" => Some(g_ABUSE_VERB_PART()),
            "ASK_AFTER_ME" => Some(g_ASK_AFTER_ME()),
            "BLOW_THINGS" => Some(g_BLOW_THINGS()),
            "BY_NOT_A_THING" => Some(g_BY_NOT_A_THING()),
            "CHILD_AGGRESSOR" => Some(g_CHILD_AGGRESSOR()),
            "CHILD_HIT_NOW" => Some(g_CHILD_HIT_NOW()),
            "CLOSE" => Some(g_CLOSE()),
            "DYING_OTHER" => Some(g_DYING_OTHER()),
            "GAME_AFTER_ME" => Some(g_GAME_AFTER_ME()),
            "GAME_NOT_A_BLOW" => Some(g_GAME_NOT_A_BLOW()),
            "GAME_PLAYED" => Some(g_GAME_PLAYED()),
            "GAME_WORD_STOP" => Some(g_GAME_WORD_STOP()),
            "GRIP_GAME" => Some(g_GRIP_GAME()),
            "GRIP_GAME_FENCE" => Some(g_GRIP_GAME_FENCE()),
            "GRIP_NOT_AN_ASSAULT" => Some(g_GRIP_NOT_AN_ASSAULT()),
            "GRIP_NOT_AN_ASSAULT_STR" => Some(g_GRIP_NOT_AN_ASSAULT_STR()),
            "HAS_ME_SUBJECT" => Some(g_HAS_ME_SUBJECT()),
            "HIT_AS_A_CHILD" => Some(g_HIT_AS_A_CHILD()),
            "HIT_BY_A_VEHICLE" => Some(g_HIT_BY_A_VEHICLE()),
            "HIT_THING_NOT_PERSON" => Some(g_HIT_THING_NOT_PERSON()),
            "I_DID_NOT_HIT" => Some(g_I_DID_NOT_HIT()),
            "I_HIT_SOMEONE" => Some(g_I_HIT_SOMEONE()),
            "MONEY_NAMED" => Some(g_MONEY_NAMED()),
            "MY_GROWN_UP" => Some(g_MY_GROWN_UP()),
            "NOT_A_FEAR" => Some(g_NOT_A_FEAR()),
            "OTHERS" => Some(g_OTHERS()),
            "PARTNER_CLOSE" => Some(g_PARTNER_CLOSE()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PERSON" => Some(g_PERSON()),
            "PLAY_FACE_REPLIES" => Some(g_PLAY_FACE_REPLIES()),
            "PLAY_NAMED" => Some(g_PLAY_NAMED()),
            "PURSUIT" => Some(g_PURSUIT()),
            "PURSUIT_FEAR" => Some(g_PURSUIT_FEAR()),
            "PURSUIT_LOUD" => Some(g_PURSUIT_LOUD()),
            "PURSUIT_OUTSIDE" => Some(g_PURSUIT_OUTSIDE()),
            "PURSUIT_PLACE_WORK" => Some(g_PURSUIT_PLACE_WORK()),
            "PURSUIT_SUBJECT" => Some(g_PURSUIT_SUBJECT()),
            "PURSUIT_SUBJECT_EX" => Some(g_PURSUIT_SUBJECT_EX()),
            "REMORSE_FOR_THE_ACT" => Some(g_REMORSE_FOR_THE_ACT()),
            "SELF_HARM_NEEDS_PERSON" => Some(g_SELF_HARM_NEEDS_PERSON()),
            "SELF_OWN_DYING" => Some(g_SELF_OWN_DYING()),
            "SOFT_FACE_PUSH" => Some(g_SOFT_FACE_PUSH()),
            "SOFT_FACE_STOP" => Some(g_SOFT_FACE_STOP()),
            "SOFT_HIT_THING" => Some(g_SOFT_HIT_THING()),
            "SOFT_HIT_VERB" => Some(g_SOFT_HIT_VERB()),
            "SOMEONE_HOLDS_THE_MONEY" => Some(g_SOMEONE_HOLDS_THE_MONEY()),
            "THE_BLOW_ON_A_CHILD" => Some(g_THE_BLOW_ON_A_CHILD()),
            "THREAT_COERCIVE" => Some(g_THREAT_COERCIVE()),
            "THREAT_SUBJECT" => Some(g_THREAT_SUBJECT()),
            "THROWN_AT_ME" => Some(g_THROWN_AT_ME()),
            "_NOT_A_THREAT" => Some(g__NOT_A_THREAT()),
            "_REFUSED_EXIT" => Some(g__REFUSED_EXIT()),
            "_THE_BLOW" => Some(g__THE_BLOW()),
            "_THE_HAND" => Some(g__THE_HAND()),
            "_THE_ONE_HIT" => Some(g__THE_ONE_HIT()),
            "__all__" => Some(g___all__()),
            "_both_here" => Some(py_func("_both_here", |args: &[Py]| _both_here(args))),
            _ => None,
        }),
        set: None,
    }))
}

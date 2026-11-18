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


thread_local! { static G_ABUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_DISCLOSURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_DISCLOSURE_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_HEAD_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_HIT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_MEMORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_PURSUIT_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL_LABEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL_MAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERBAL_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_VERB_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_AFTER_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLOW_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BY_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_AGGRESSOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_HIT_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DYING_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_AFTER_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_NOT_A_BLOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_PLAYED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_WORD_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIP_GAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIP_GAME_FENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIP_NOT_AN_ASSAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRIP_NOT_AN_ASSAULT_STR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_ME_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIT_AS_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIT_BY_A_VEHICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIT_THING_NOT_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_DID_NOT_HIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_HIT_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_GROWN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER_CLOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAY_FACE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_FEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_LOUD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_PLACE_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_PLACE_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PURSUIT_SUBJECT_EX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMORSE_FOR_THE_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_HARM_NEEDS_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_OWN_DYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_FACE_PUSH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_FACE_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_HIT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_HIT_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_HOLDS_THE_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BLOW_ON_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAT_COERCIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAT_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THROWN_AT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THREAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REFUSED_EXIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_BLOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ONE_HIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G___all__: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.safety_phrases: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABUSE() -> Py { ensure_init(); G_ABUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE(v: Py) { ensure_init(); G_ABUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSER() -> Py { ensure_init(); G_ABUSER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSER(v: Py) { ensure_init(); G_ABUSER.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_DISCLOSURE() -> Py { ensure_init(); G_ABUSE_DISCLOSURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_DISCLOSURE(v: Py) { ensure_init(); G_ABUSE_DISCLOSURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_DISCLOSURE_FINE() -> Py { ensure_init(); G_ABUSE_DISCLOSURE_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_DISCLOSURE_FINE(v: Py) { ensure_init(); G_ABUSE_DISCLOSURE_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_FINE() -> Py { ensure_init(); G_ABUSE_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_FINE(v: Py) { ensure_init(); G_ABUSE_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_HEAD_INTO() -> Py { ensure_init(); G_ABUSE_HEAD_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_HEAD_INTO(v: Py) { ensure_init(); G_ABUSE_HEAD_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_HIT_ME() -> Py { ensure_init(); G_ABUSE_HIT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_HIT_ME(v: Py) { ensure_init(); G_ABUSE_HIT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_MEMORY() -> Py { ensure_init(); G_ABUSE_MEMORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_MEMORY(v: Py) { ensure_init(); G_ABUSE_MEMORY.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_NOW() -> Py { ensure_init(); G_ABUSE_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_NOW(v: Py) { ensure_init(); G_ABUSE_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_PURSUIT_HARD() -> Py { ensure_init(); G_ABUSE_PURSUIT_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_PURSUIT_HARD(v: Py) { ensure_init(); G_ABUSE_PURSUIT_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERB() -> Py { ensure_init(); G_ABUSE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERB(v: Py) { ensure_init(); G_ABUSE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL() -> Py { ensure_init(); G_ABUSE_VERBAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL(v: Py) { ensure_init(); G_ABUSE_VERBAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL_FINE() -> Py { ensure_init(); G_ABUSE_VERBAL_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_FINE(v: Py) { ensure_init(); G_ABUSE_VERBAL_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL_LABEL() -> Py { ensure_init(); G_ABUSE_VERBAL_LABEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_LABEL(v: Py) { ensure_init(); G_ABUSE_VERBAL_LABEL.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL_MAD() -> Py { ensure_init(); G_ABUSE_VERBAL_MAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_MAD(v: Py) { ensure_init(); G_ABUSE_VERBAL_MAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL_PERSON() -> Py { ensure_init(); G_ABUSE_VERBAL_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_PERSON(v: Py) { ensure_init(); G_ABUSE_VERBAL_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERBAL_SUBJECT() -> Py { ensure_init(); G_ABUSE_VERBAL_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERBAL_SUBJECT(v: Py) { ensure_init(); G_ABUSE_VERBAL_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_VERB_PART() -> Py { ensure_init(); G_ABUSE_VERB_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_VERB_PART(v: Py) { ensure_init(); G_ABUSE_VERB_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_AFTER_ME() -> Py { ensure_init(); G_ASK_AFTER_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_AFTER_ME(v: Py) { ensure_init(); G_ASK_AFTER_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_BLOW_THINGS() -> Py { ensure_init(); G_BLOW_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLOW_THINGS(v: Py) { ensure_init(); G_BLOW_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_BY_NOT_A_THING() -> Py { ensure_init(); G_BY_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BY_NOT_A_THING(v: Py) { ensure_init(); G_BY_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_AGGRESSOR() -> Py { ensure_init(); G_CHILD_AGGRESSOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_AGGRESSOR(v: Py) { ensure_init(); G_CHILD_AGGRESSOR.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_HIT_NOW() -> Py { ensure_init(); G_CHILD_HIT_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_HIT_NOW(v: Py) { ensure_init(); G_CHILD_HIT_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSE() -> Py { ensure_init(); G_CLOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSE(v: Py) { ensure_init(); G_CLOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_DYING_OTHER() -> Py { ensure_init(); G_DYING_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DYING_OTHER(v: Py) { ensure_init(); G_DYING_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_AFTER_ME() -> Py { ensure_init(); G_GAME_AFTER_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_AFTER_ME(v: Py) { ensure_init(); G_GAME_AFTER_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_NOT_A_BLOW() -> Py { ensure_init(); G_GAME_NOT_A_BLOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_NOT_A_BLOW(v: Py) { ensure_init(); G_GAME_NOT_A_BLOW.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_PLAYED() -> Py { ensure_init(); G_GAME_PLAYED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_PLAYED(v: Py) { ensure_init(); G_GAME_PLAYED.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_WORD_STOP() -> Py { ensure_init(); G_GAME_WORD_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_WORD_STOP(v: Py) { ensure_init(); G_GAME_WORD_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIP_GAME() -> Py { ensure_init(); G_GRIP_GAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIP_GAME(v: Py) { ensure_init(); G_GRIP_GAME.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIP_GAME_FENCE() -> Py { ensure_init(); G_GRIP_GAME_FENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIP_GAME_FENCE(v: Py) { ensure_init(); G_GRIP_GAME_FENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIP_NOT_AN_ASSAULT() -> Py { ensure_init(); G_GRIP_NOT_AN_ASSAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIP_NOT_AN_ASSAULT(v: Py) { ensure_init(); G_GRIP_NOT_AN_ASSAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_GRIP_NOT_AN_ASSAULT_STR() -> Py { ensure_init(); G_GRIP_NOT_AN_ASSAULT_STR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRIP_NOT_AN_ASSAULT_STR(v: Py) { ensure_init(); G_GRIP_NOT_AN_ASSAULT_STR.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_ME_SUBJECT() -> Py { ensure_init(); G_HAS_ME_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_ME_SUBJECT(v: Py) { ensure_init(); G_HAS_ME_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_HIT_AS_A_CHILD() -> Py { ensure_init(); G_HIT_AS_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIT_AS_A_CHILD(v: Py) { ensure_init(); G_HIT_AS_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_HIT_BY_A_VEHICLE() -> Py { ensure_init(); G_HIT_BY_A_VEHICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIT_BY_A_VEHICLE(v: Py) { ensure_init(); G_HIT_BY_A_VEHICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_HIT_THING_NOT_PERSON() -> Py { ensure_init(); G_HIT_THING_NOT_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIT_THING_NOT_PERSON(v: Py) { ensure_init(); G_HIT_THING_NOT_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_I_DID_NOT_HIT() -> Py { ensure_init(); G_I_DID_NOT_HIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_DID_NOT_HIT(v: Py) { ensure_init(); G_I_DID_NOT_HIT.with(|c| *c.borrow_mut() = v); }
pub fn g_I_HIT_SOMEONE() -> Py { ensure_init(); G_I_HIT_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_HIT_SOMEONE(v: Py) { ensure_init(); G_I_HIT_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_NAMED() -> Py { ensure_init(); G_MONEY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_NAMED(v: Py) { ensure_init(); G_MONEY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_GROWN_UP() -> Py { ensure_init(); G_MY_GROWN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_GROWN_UP(v: Py) { ensure_init(); G_MY_GROWN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FEAR() -> Py { ensure_init(); G_NOT_A_FEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FEAR(v: Py) { ensure_init(); G_NOT_A_FEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHERS() -> Py { ensure_init(); G_OTHERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHERS(v: Py) { ensure_init(); G_OTHERS.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER_CLOSE() -> Py { ensure_init(); G_PARTNER_CLOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER_CLOSE(v: Py) { ensure_init(); G_PARTNER_CLOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE() -> Py { ensure_init(); G_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); G_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON() -> Py { ensure_init(); G_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); G_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAY_FACE_REPLIES() -> Py { ensure_init(); G_PLAY_FACE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAY_FACE_REPLIES(v: Py) { ensure_init(); G_PLAY_FACE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAY_NAMED() -> Py { ensure_init(); G_PLAY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAY_NAMED(v: Py) { ensure_init(); G_PLAY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT() -> Py { ensure_init(); G_PURSUIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT(v: Py) { ensure_init(); G_PURSUIT.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_FEAR() -> Py { ensure_init(); G_PURSUIT_FEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_FEAR(v: Py) { ensure_init(); G_PURSUIT_FEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_LOUD() -> Py { ensure_init(); G_PURSUIT_LOUD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_LOUD(v: Py) { ensure_init(); G_PURSUIT_LOUD.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_OUTSIDE() -> Py { ensure_init(); G_PURSUIT_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_OUTSIDE(v: Py) { ensure_init(); G_PURSUIT_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_PLACE_HOME() -> Py { ensure_init(); G_PURSUIT_PLACE_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_PLACE_HOME(v: Py) { ensure_init(); G_PURSUIT_PLACE_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_PLACE_WORK() -> Py { ensure_init(); G_PURSUIT_PLACE_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_PLACE_WORK(v: Py) { ensure_init(); G_PURSUIT_PLACE_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_SUBJECT() -> Py { ensure_init(); G_PURSUIT_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_SUBJECT(v: Py) { ensure_init(); G_PURSUIT_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_PURSUIT_SUBJECT_EX() -> Py { ensure_init(); G_PURSUIT_SUBJECT_EX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PURSUIT_SUBJECT_EX(v: Py) { ensure_init(); G_PURSUIT_SUBJECT_EX.with(|c| *c.borrow_mut() = v); }
pub fn g_REMORSE_FOR_THE_ACT() -> Py { ensure_init(); G_REMORSE_FOR_THE_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMORSE_FOR_THE_ACT(v: Py) { ensure_init(); G_REMORSE_FOR_THE_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_HARM_NEEDS_PERSON() -> Py { ensure_init(); G_SELF_HARM_NEEDS_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_HARM_NEEDS_PERSON(v: Py) { ensure_init(); G_SELF_HARM_NEEDS_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_OWN_DYING() -> Py { ensure_init(); G_SELF_OWN_DYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_OWN_DYING(v: Py) { ensure_init(); G_SELF_OWN_DYING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_FACE_PUSH() -> Py { ensure_init(); G_SOFT_FACE_PUSH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_FACE_PUSH(v: Py) { ensure_init(); G_SOFT_FACE_PUSH.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_FACE_STOP() -> Py { ensure_init(); G_SOFT_FACE_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_FACE_STOP(v: Py) { ensure_init(); G_SOFT_FACE_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_HIT_THING() -> Py { ensure_init(); G_SOFT_HIT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_HIT_THING(v: Py) { ensure_init(); G_SOFT_HIT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_HIT_VERB() -> Py { ensure_init(); G_SOFT_HIT_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_HIT_VERB(v: Py) { ensure_init(); G_SOFT_HIT_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_HOLDS_THE_MONEY() -> Py { ensure_init(); G_SOMEONE_HOLDS_THE_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_HOLDS_THE_MONEY(v: Py) { ensure_init(); G_SOMEONE_HOLDS_THE_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BLOW_ON_A_CHILD() -> Py { ensure_init(); G_THE_BLOW_ON_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BLOW_ON_A_CHILD(v: Py) { ensure_init(); G_THE_BLOW_ON_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAT_COERCIVE() -> Py { ensure_init(); G_THREAT_COERCIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAT_COERCIVE(v: Py) { ensure_init(); G_THREAT_COERCIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAT_SUBJECT() -> Py { ensure_init(); G_THREAT_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAT_SUBJECT(v: Py) { ensure_init(); G_THREAT_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_THROWN_AT_ME() -> Py { ensure_init(); G_THROWN_AT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THROWN_AT_ME(v: Py) { ensure_init(); G_THROWN_AT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THREAT() -> Py { ensure_init(); G__NOT_A_THREAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THREAT(v: Py) { ensure_init(); G__NOT_A_THREAT.with(|c| *c.borrow_mut() = v); }
pub fn g__REFUSED_EXIT() -> Py { ensure_init(); G__REFUSED_EXIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REFUSED_EXIT(v: Py) { ensure_init(); G__REFUSED_EXIT.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_BLOW() -> Py { ensure_init(); G__THE_BLOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_BLOW(v: Py) { ensure_init(); G__THE_BLOW.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_HAND() -> Py { ensure_init(); G__THE_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_HAND(v: Py) { ensure_init(); G__THE_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ONE_HIT() -> Py { ensure_init(); G__THE_ONE_HIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ONE_HIT(v: Py) { ensure_init(); G__THE_ONE_HIT.with(|c| *c.borrow_mut() = v); }
pub fn g___all__() -> Py { ensure_init(); G___all__.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); G___all__.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Phrase tables for `safety.py`, kept in their own file.\n\n`bot/modules/safety.py` was 56 KB over the world's 1 MiB per-file cap on main `64514606`, and\n`open_pr` refuses every PR that touches a file over it - so the safety lane could not open a\nchange at all. These are the same tables, moved whole and imported back at the same point in\nthe same order, so the replies people get are identical. Nothing was deleted or rewritten.\n");
    s_PURSUIT_PLACE_HOME(pyrt::xrefs::xget("modules.safety_cards", "PURSUIT_PLACE_HOME"));
    fn __data1() -> Py { py_list(vec![py_str("ABUSE"), py_str("ABUSER"), py_str("BY_NOT_A_THING"), py_str("ABUSE_DISCLOSURE"), py_str("ABUSE_DISCLOSURE_FINE"), py_str("ABUSE_FINE"), py_str("ABUSE_HEAD_INTO"), py_str("ABUSE_HIT_ME"), py_str("ABUSE_MEMORY"), py_str("ABUSE_NOW"), py_str("ABUSE_PURSUIT_HARD"), py_str("ABUSE_VERB"), py_str("ABUSE_VERBAL"), py_str("ABUSE_VERBAL_FINE"), py_str("ABUSE_VERBAL_LABEL"), py_str("ABUSE_VERBAL_MAD"), py_str("ABUSE_VERBAL_PERSON"), py_str("ABUSE_VERBAL_SUBJECT"), py_str("ABUSE_VERB_PART"), py_str("ASK_AFTER_ME"), py_str("BLOW_THINGS"), py_str("CHILD_AGGRESSOR"), py_str("CHILD_HIT_NOW"), py_str("CLOSE"), py_str("DYING_OTHER"), py_str("GAME_AFTER_ME"), py_str("GAME_PLAYED"), py_str("GAME_WORD_STOP"), py_str("GRIP_GAME"), py_str("GRIP_GAME_FENCE"), py_str("GRIP_NOT_AN_ASSAULT"), py_str("GRIP_NOT_AN_ASSAULT_STR"), py_str("HAS_ME_SUBJECT"), py_str("HIT_AS_A_CHILD"), py_str("HIT_BY_A_VEHICLE"), py_str("HIT_THING_NOT_PERSON"), py_str("I_DID_NOT_HIT"), py_str("I_HIT_SOMEONE"), py_str("MONEY_NAMED"), py_str("NOT_A_FEAR"), py_str("OTHERS"), py_str("PARTNER_CLOSE"), py_str("PEOPLE"), py_str("PERSON"), py_str("PLAY_FACE_REPLIES"), py_str("PURSUIT"), py_str("PURSUIT_FEAR"), py_str("PURSUIT_LOUD"), py_str("PURSUIT_OUTSIDE"), py_str("PURSUIT_PLACE_WORK"), py_str("PURSUIT_SUBJECT"), py_str("PURSUIT_SUBJECT_EX"), py_str("REMORSE_FOR_THE_ACT"), py_str("SELF_HARM_NEEDS_PERSON"), py_str("SELF_OWN_DYING"), py_str("SOFT_FACE_PUSH"), py_str("SOFT_FACE_STOP"), py_str("SOMEONE_HOLDS_THE_MONEY"), py_str("THREAT_COERCIVE"), py_str("THREAT_SUBJECT"), py_str("_NOT_A_THREAT"), py_str("_REFUSED_EXIT"), py_str("_THE_BLOW"), py_str("_THE_HAND"), py_str("_THE_ONE_HIT"), py_str("_both_here")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.__all__"); }
    s___all__(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.MONEY_NAMED"); }
    s_MONEY_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:pay\\s?day|paid|pay|wages?|salary|pension|benefits?|universal\\s+credit|money|cash|quid|pounds?|dollars?|euros?|fiver|tenner|skint|paycheck|overdraft|rent|bills?|\\d+\\s*(?:pounds?|quid|dollars?|euros?|bucks?))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SOMEONE_HOLDS_THE_MONEY"); }
    s_SOMEONE_HOLDS_THE_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|someone|somebody|my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|mum|mom|dad|father|mother|son|daughter|brother|sister|flatmate|roommate|housemate|landlord))\\b[^.?!]{0,40}?\\b(?:took|takes|taking|emptied|empties|cleared\\s+out|clears\\s+out|controls?|controlled|keeps?|kept|hides?|hid|hidden|spends?|spent|gambled|stole|steals?)\\b[^.?!]{0,30}\\b(?:money|wages?|salary|pay|paycheck|account|benefits?|universal\\s+credit|pension|savings?|card)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SELF_HARM_NEEDS_PERSON"); }
    s_SELF_HARM_NEEDS_PERSON(Py::Bool(true));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.DYING_OTHER"); }
    s_DYING_OTHER(py_tuple(vec![py_str("\\b(?:my|our|his|her|their|the)\\s+(?:nan|gran|nanny|nana|granny|grandma|grandmother|grandad|granddad|grandpa|grandfather|mum|mom|mother|dad|father|wife|husband|partner|son|daughter|child|kid|baby|brother|sister|aunt|uncle|cousin|friend|mate|dog|cat|puppy|kitten|pet|horse)\\b[^.?!]{0,20}\\b(?:is|are|'s|s)\\s+(?:going\\s+to|gonna|about\\s+to)\\s+die\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SELF_OWN_DYING"); }
    s_SELF_OWN_DYING(py_tuple(vec![py_str("\\b(?:im|i'm|i\\s+am|i\\s+think\\s+im|i\\s+think\\s+i'm|i\\s+feel|i\\s+keep\\s+thinking)\\b[^.?!]{0,20}\\b(?:going\\s+to|gonna|want\\s+to|wanna|ready\\s+to)\\s+die\\b"), py_str("\\bwant\\s+to\\s+die\\b"), py_str("\\bkill\\s+my\\s*self\\b"), py_str("\\bbetter\\s+off\\s+dead\\b"), py_str("\\bsuicid\\w*\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PERSON"); }
    s_PERSON(py_str("(?:mum|mom|mother|dad|father|son|daughter|husband|wife|partner|boyfriend|girlfriend|brother|sister|friend|flatmate|roommate|baby|child|kid|kids|children|grand(?:child|children|son|daughter|kid)s?|step(?:child|children|son|daughter)s?|boy|girl|man|woman|lady|guy|bloke|neighbou?r|colleague|boss|gran|granny|nan|grandad|grandma|grandpa|mate|carers?|care\\s+workers?|support\\s+workers?|social\\s+workers?|personal\\s+assistants?|nurses?|midwives?|midwife|doctors?|gps?|physios?|therapists?|patient|pupil|student|teacher|driver|passenger|singer|player|runner|swimmer)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PEOPLE"); }
    s_PEOPLE(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:i|i'm|i've|he|he's|she|she's|they|they're|we|you|someone|somebody|anyone|everyone|my\\s+"), &g_PERSON()), &py_str("|the\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.OTHERS"); }
    s_OTHERS(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:he|he's|she|she's|they|they're|him|her|them|someone|somebody|anyone|everyone|my\\s+"), &g_PERSON()), &py_str("|his\\s+")), &g_PERSON()), &py_str("|her\\s+")), &g_PERSON()), &py_str("|their\\s+")), &g_PERSON()), &py_str("|")), &g_PERSON()), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSER"); }
    s_ABUSER(op_add(&op_add(&py_str("(?:"), &g_PERSON()), &py_str("|step(?:mum|mom|dad|mother|father|brother|sister|son|daughter|parent|child|children)|ex|landlord|landlady|housemate|lodger|aunt|auntie|uncle|cousin|nan|nana|grandparent|in-?laws?|father-?in-?law|mother-?in-?law|son-?in-?law|daughter-?in-?law|guardian|foster\\s+parent|boy|friend|manager|supervisor|nieces?|nephews?|grandmothers?|grandfathers?|grandparents?)")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases._THE_ONE_HIT"); }
    s__THE_ONE_HIT(py_str("(?:my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|fianc(?:e|ée)|mum|mom|mother|dad|father|son|daughter|child|kid|kids|children|baby|stepson|stepdaughter|stepdad|stepmum|brother|sister|nan|gran|grandma|grandad|carer)(?!'s)|(?:my|the)\\s+(?:kids|children|child|baby|son|daughter)s?(?!'s)|her|him|them)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases._THE_BLOW"); }
    s__THE_BLOW(py_str("(?:hit|hitting|slap(?:ped|ping)?|punch(?:ed|ing)?|kick(?:ed|ing)?|beat|beating|chok(?:ed|ing)?|strangl(?:ed|ing)?|drag(?:ged|ging)?|threw|thrown|throwing|scratch(?:ed|ing)|claw(?:ed|ing))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases._THE_HAND"); }
    s__THE_HAND(py_str("(?:push(?:ed|ing)?|shov(?:e|ed|es|ing)|grab(?:bed|bing)?|pin(?:ned|ning)?|held|hold|pulled|yank(?:ed|ing)?|twist(?:ed|ing)?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.I_HIT_SOMEONE"); }
    s_I_HIT_SOMEONE(py_tuple(vec![op_add(&op_add(&op_add(&py_str("\\bi\\s+(?:keep\\s+|kept\\s+|just\\s+|nearly\\s+|almost\\s+|have\\s+|had\\s+)?"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT()), op_add(&op_add(&op_add(&op_add(&py_str("\\bi\\s+(?:keep\\s+|kept\\s+)?"), &g__THE_HAND()), &py_str("\\s+")), &g__THE_ONE_HIT()), &py_str("(?:[^.?!]{0,30}\\b(?:throat|neck|wall|floor|stairs|ground|door|hard|down|against|into\\s+the\\s+(?:road|traffic|kitchen|hall|bathroom|fire|car|bushes))\\b|[^.?!]{0,40}\\b(?:during|in|after|before)\\s+(?:an?\\s+)?(?:argument|row|fight|disagreement|falling\\s+out)\\b)")), op_add(&op_add(&op_add(&py_str("\\b(?:i\\s+)?(?:lost|lose|be\\s+losing)\\s+my\\s+temper\\b[^.?!]{0,30}\\b"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT()), op_add(&op_add(&op_add(&py_str("\\bi\\s+(?:was\\s+|got\\s+|get\\s+|am\\s+|'m\\s+|im\\s+)?(?:so\\s+)?(?:angry|mad|raging|drunk|pissed|upset|frustrated|stressed|exhausted|wound\\s+up|beside\\s+myself)\\b[^.?!]{0,25}\\b"), &g__THE_BLOW()), &py_str("\\s+")), &g__THE_ONE_HIT())]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.HIT_THING_NOT_PERSON"); }
    s_HIT_THING_NOT_PERSON(py_tuple(vec![py_str("\\b(?:partner|husband|wife|boyfriend|girlfriend|ex|fiance|fiancee|son|daughter|child|kid|kids|children|baby|mum|mom|mother|dad|father|brother|sister|nan|gran|grandma|grandad|carer|parents)'s\\b"), py_str("\\b(?:partner|husband|wife|boyfriend|girlfriend|ex|son|daughter|child|kid|kids|children|baby|mum|mom|mother|dad|father|brother|sister|carer)(?:s)?\\s+(?:\\w+\\s+){0,2}(?:cake|car|bike|toys?|ball|phone|laptop|window|door|plate|glass|cup|mug|tv|telly|scooter|bike|present|cards?|homework|drawing|picture)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GAME_NOT_A_BLOW"); }
    s_GAME_NOT_A_BLOW(py_tuple(vec![py_str("\\b(?:mess(?:ing)?\\s+about|mucking\\s+about|play(?:ing)?\\s*(?:fighting|fight)|sparring|boxing|wrestl(?:ing|e)|judo|karate|rugby|football|practice|training|drill|a\\s+game|snowball|water\\s+fight|paintball|nerf|tickl(?:e|ing)|play)\\b"), py_str("\\b(?:at|in|during|on)\\s+(?:the\\s+)?(?:chess|cards?|scrabble|monopoly|backgammon|draughts|checkers|dominoes|poker|darts|snooker|billiards|pool|bowling|tennis|badminton|squash|golf|cricket|football|rugby|rounders|netball|basketball|hockey|table\\s+tennis|video\\s+games?|playstation|xbox|wii|board\\s+game)\\b"), py_str("\\b(?:in|at|during)\\s+(?:[\\w-]+\\s+)?(?:final|match|game|tournament|league|race|marathon|competition|championship|quiz)\\b(?!\\s+(?:argument|row|fight|bust-?up|falling\\s+out))")]));
    fn __data2() -> Py { py_tuple(vec![py_str("\\b(?:never|didn'?t|didnt|don'?t|dont|wouldn'?t|wouldnt|not|have\\s+never)\\s+(?:ever\\s+)?(?:hit|hitting|hurt|hurting|touch|touched|lay\\s+a\\s+hand|laid\\s+a\\s+hand)\\b"), op_add(&op_add(&py_str("\\bhit\\s+(?:her|him|them|my\\s+"), &g_ABUSER()), &py_str(")\\s+back\\b")), op_add(&op_add(&py_str("\\b(?:lose|losing|lost)\\s+my\\s+temper\\s+(?:with|at)\\s+(?:her|him|them|the\\s+kids|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:shout|shouted|shouting|yell|yelled|swore|swear|snapped|said|slam|slammed|walked|rowing|row|argue|argued|arguing)\\b")), py_str("\\bwhen\\s+we\\s+were\\s+(?:kids|children|little|lads|girls|boys)\\b"), py_str("\\bwe\\s+used\\s+to\\s+(?:fight|hit|scrap|wrestle)\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:\\d+|a\\s+kid|a\\s+child|a\\s+lad|a\\s+boy|a\\s+girl|young|little|small|tiny|at\\s+school|in\\s+school)\\b"), py_str("\\b(?:as|like)\\s+(?:a\\s+)?(?:kid|child|kids|children|lads|boys|girls)\\b"), py_str("\\bgrowing\\s+up\\b"), py_str("\\b(?:at|in)\\s+(?:primary|secondary|junior|middle|comprehensive)\\s+school\\b"), py_str("\\bi\\s+used\\s+to\\s+(?:\\w+\\s+){0,3}(?:hit|fight|scrap|wrestle)\\b[^.?!]{0,25}\\b(?:brother|sister|cousin|mate|friend|kids|kids|boy|girl)s?\\b"), py_str("\\b(?:accident|accidentally|by\\s+mistake|didn'?t\\s+mean|didnt\\s+mean|in\\s+my\\s+sleep|sleepwalking|fidgeting)\\b"), py_str("\\b(?:awake|out\\s+of\\s+bed|to\\s+wake\\s+(?:me|him|her|them|the\\s+kids))\\b"), py_str("\\b(?:in\\s+a\\s+dream|dream(?:t|ed)\\b|had\\s+a\\s+dream|nightmare)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.I_DID_NOT_HIT"); }
    s_I_DID_NOT_HIT(op_add(&__data2(), &g_GAME_NOT_A_BLOW()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.MY_GROWN_UP"); }
    s_MY_GROWN_UP(py_str("(?:dad|father|stepdad|mum|mom|mother|stepmum|parents|uncle|aunt|auntie|brother|sister|cousin|carer|nan|nana|gran|grandma|grandad|grandpa|grandparents|foster\\s+\\w+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.THE_BLOW_ON_A_CHILD"); }
    s_THE_BLOW_ON_A_CHILD(op_add(&g__THE_BLOW(), &py_str("|beat|belt(?:ed)?|strap(?:ped)?|cane(?:d)?|smack(?:s|ed|ing)?|lock(?:ed)?|starve(?:d)?|smother(?:ed)?|throttle(?:d)?|abuse(?:d|s)?|molest(?:ed|s)?|touch(?:ed|es)?")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.THROWN_AT_ME"); }
    s_THROWN_AT_ME(py_str("(?:(?:throw|threw|thrown|throwing|hurl(?:ed|ing)?|chuck(?:ed|ing)?)[^.?!]{0,20}\\bat\\s+(?:me|us)\\b|(?:threw|thrown|throwing|throw|hurled)[^.?!]{0,15}\\b(?:against|into|down)\\s+(?:the\\s+)?(?:wall|door|floor|stairs|ground)\\b)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.HIT_AS_A_CHILD"); }
    s_HIT_AS_A_CHILD(py_tuple(vec![op_add(&op_add(&op_add(&op_add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,40}\\b(?:")), &g__THE_BLOW()), &py_str("|burn(?:ed|t)|scalded|whipped|belt(?:ed)?|strap(?:ped)?|cane(?:d)?|smack(?:s|ed|ing)?|abused|molested|touched\\s+me|locked\\s+me|starved|smothered|throttled)\\b[^.?!]{0,40}\\b(?:when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small|teenager|teen|at\\s+school|in\\s+school|\\d{1,2})|as\\s+a\\s+(?:kid|child|little\\s+\\w+|teenager)|growing\\s+up|back\\s+then|years\\s+ago|decades\\s+ago|my\\s+whole\\s+childhood|all\\s+my\\s+childhood|(?:used|would)\\s+to\\s+\\w+\\s+me)\\b")), py_str("\\bi\\s+(?:was|got)\\s+(?:hit|beaten|abused|molested|whipped|slapped|kicked|punched)\\s+(?:as\\s+a\\s+(?:kid|child|little\\s+\\w+|teenager)|when\\s+i\\s+was\\s+(?:a\\s+)?(?:kid|child|little|young|small|teenager|at\\s+school)|growing\\s+up|my\\s+whole\\s+childhood)\\b"), op_add(&op_add(&op_add(&op_add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,25}\\b(?:used\\s+to|would)\\s+[^.?!]{0,20}\\b(?:")), &g_THE_BLOW_ON_A_CHILD()), &py_str(")\\b[^.?!]{0,15}\\b(?:me|us)\\b")), op_add(&op_add(&op_add(&py_str("\\bmy\\s+"), &g_MY_GROWN_UP()), &py_str("\\b[^.?!]{0,25}\\b(?:used\\s+to|would)\\s+[^.?!]{0,20}")), &g_THROWN_AT_ME()), op_add(&op_add(&py_str("\\bi\\s+(?:used|would)\\s+to\\s+(?:get|be)\\s+(?:hit|beaten|beat|slapped|punched|kicked|whipped|belted|abused|smacked)\\b[^.?!]{0,25}\\b(?:at\\s+home|by\\s+my\\s+"), &g_MY_GROWN_UP()), &py_str("|as\\s+a\\s+(?:kid|child)|when\\s+i\\s+was)\\b"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.CHILD_HIT_NOW"); }
    s_CHILD_HIT_NOW(py_tuple(vec![py_str("\\b(?:still|now|again|these\\s+days|nowadays|every\\s+(?:day|night|week|time))\\s+(?:\\w+\\s+){0,2}(?:hits?|beats?|does|doing|comes?|turns?\\s+up)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.REMORSE_FOR_THE_ACT"); }
    s_REMORSE_FOR_THE_ACT(py_tuple(vec![py_str("\\b(?:hate|hating|disgusted\\s+(?:with|by)|ashamed\\s+of|sickened\\s+by|disappointed\\s+in|furious\\s+(?:with|at)|angry\\s+at)\\s+my\\s*self\\b"), py_str("\\bi\\s*(?:'?m|am|feel|felt)\\s+(?:like\\s+)?(?:such\\s+)?(?:an?\\s+)?(?:monster|beast|awful\\s+person|terrible\\s+person|horrible\\s+person|bad\\s+person)\\b"), py_str("\\bwhat\\s+have\\s+i\\s+done\\b"), py_str("\\bcan'?t\\s+believe\\s+i\\s+(?:did|done)\\s+(?:that|it)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERB"); }
    s_ABUSE_VERB(py_str("(?:hits?|hitting|beat(?:s|ing)?|punch(?:es|ed|ing)?|kick(?:s|ed|ing)?|chok(?:es|ed|ing)|slapp?(?:s|ed|ing)?|grab(?:s|bed|bing)?|drag(?:s|ged|ging)?|burn(?:s|ed|ing)?|strangl(?:e|es|ed|ing)|push(?:es|ed|ing)?|shov(?:e|es|ed|ing)|bit\\b|bite|bites|biting|spat\\s+at|spit\\s+at|throw(?:s|ing)?|threw|hurl(?:s|ed|ing)?|yank(?:s|ed|ing)?|twist(?:s|ed|ing)?|wrench(?:es|ed|ing)?|swung\\s+at|swing\\s+at|attack(?:s|ed|ing)?|savaged|mauled|lash(?:es|ed|ing)?\\s+out\\s+at|lunged\\s+at|held\\s+me\\s+down|pinned\\s+me|scratch(?:es|ed|ing)?|claw(?:s|ed|ing)?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.HIT_BY_A_VEHICLE"); }
    s_HIT_BY_A_VEHICLE(py_str("(?!\\s+(?:with|by)\\s+(?:a\\s+|an\\s+|the\\s+|his\\s+|her\\s+|their\\s+|our\\s+|my\\s+)?(?:car|cars|van|bike|bikes|bicycle|motorbike|motorcycle|scooter|lorry|truck|bus|train|tram|taxi|car\\s+door|door|doors|vehicle|wheelchair|buggy|pram|cart|trolley|skateboard|scooter)\\b)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases._NOT_A_THREAT"); }
    s__NOT_A_THREAT(py_str("(?!\\s+(?:with|over|about)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:written|warning|fine|fines|eviction|legal|lawsuit|court|police|report|complaint|solicitor|letter|bill|bill?s|charge|charges|prosecution|dismissal|sack|sacking|redundancy|redundant|fired|disciplinary|hearing|notice|tenancy|deposit)\\b)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.BY_NOT_A_THING"); }
    s_BY_NOT_A_THING(op_add(&op_add(&py_str("(?!\\s+by\\s+(?!(?:(?:him|her|them|his|their|someone|somebody|some\\s+men|men|guys|blokes|lads|youths|people|strangers)\\b|(?:a|an|the|two|three|four|five|several|some|another|that|this|one)\\s+(?:\\w+\\s+){0,2}(?:men|women|man|bloke|blokes|stranger|strangers|guy|guys|woman|lady|ladies|person|people|group|youth|youths|teenager|teenagers|teen|teens|kid|kids|boy|boys|girl|girls|fella|fellas|lad|lads|neighbour|neighbours|driver|carer|partner|husband|wife|boyfriend|girlfriend|ex|dad|mum|father|mother|stepdad|stepmum|brother|sister|uncle|aunt|cousin|son|daughter|landlord|landlady|housemate|colleague|boss|manager)\\b)|my\\s+"), &g_ABUSER()), &py_str("\\b))")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.CLOSE"); }
    s_CLOSE(py_str("(?:ex|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|stepdad|stepmum|stepmom|stepfather|stepmother|dad|mum|mom|father|mother|son|daughter|brother|sister|carer|uncle|aunt|auntie|nan|grandad|grandpa|grandma)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.THREAT_SUBJECT"); }
    s_THREAT_SUBJECT(op_add(&op_add(&py_str("(?:he'?s|she'?s|they'?re|he|she|they|my\\s+"), &g_CLOSE()), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.THREAT_COERCIVE"); }
    s_THREAT_COERCIVE(py_tuple(vec![op_add(&op_add(&py_str("\\b"), &g_THREAT_SUBJECT()), &py_str("\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b[^.?!]{0,25}\\b(?:police|cops?|solicitor|lawyer|court|custody|social\\s+services)\\b")), op_add(&op_add(&py_str("\\b"), &g_THREAT_SUBJECT()), &py_str("\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\s+to\\s+(?:call|ring|report|tell|contact|go\\s+to)\\b[^.?!]{0,20}\\b(?:police|cops?|solicitor|lawyer|court|custody|social\\s+services)\\b")), op_add(&op_add(&op_add(&op_add(&py_str("\\bmy\\s+"), &g_CLOSE()), &py_str("(?:'?s|\\s+is|\\s+keeps?|\\s+has\\s+been)\\s+threatening\\s+my\\s+")), &g_CLOSE()), &py_str("\\b"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases._REFUSED_EXIT"); }
    s__REFUSED_EXIT(py_str("(?:past|through|out|out\\s+of\\s+the\\s+(?:house|flat|room|car)|leave|go|go\\s+out|go\\s+anywhere|see\\s+(?:(?:my|her|his|the|any|our)\\s+)?(?:own\\s+)?(?:friends|family|mum|mom|dad|mother|father|parents|sister|brother|kids|children|mates|pals|buddies|colleagues|coworkers|anyone|people)|speak\\s+(?:to|with)\\s+(?:(?:my|her|his|the|any)\\s+)?(?:friends|family|mum|mom|dad|mother|father|parents|sister|brother|kids|children|anyone|people)|have\\s+(?:any\\s+|my\\s+own\\s+|the\\s+)?(?:money|wages|bank\\s+card|bank\\s+account|debit\\s+card|keys|a\\s+job|my\\s+own\\s+money|friends|a\\s+life|people\\s+over|visitors|company|a\\s+social\\s+life)|work(?!\\s+(?:from|at|in|late|early|nights?|weekends?|evenings?|overtime|flexibly|remotely|hybrid|part[\\s-]?time|full[\\s-]?time|with|on|for))|study|sleep|get\\s+a\\s+job|look\\s+for\\s+work|drive\\s+(?:the\\s+car|my\\s+car|his\\s+car|her\\s+car|our\\s+car|to\\s+work|to\\s+the\\s+shops|anywhere|myself)|visit\\s+(?:(?:my|her|his|any)\\s+)?(?:friends|family|mum|mom|dad|mother|father))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERB_PART"); }
    s_ABUSE_VERB_PART(py_str("(?:hits?|hitting|beat(?:s|ing)?|punch(?:es|ed|ing)?|kick(?:es|ed|ing)?|chok(?:es|ed|ing)|slapp?(?:s|ed|ing)?|smack(?:s|ed|ing)?|drag(?:s|ged|ging)?|strangl(?:e|es|ed|ing)|shov(?:e|es|ed|ing)|spat\\s+at|spit\\s+at|hurl(?:s|ed|ing)?|attack(?:s|ed|ing)?|savaged|mauled|thump(?:s|ed|ing)?|push(?:es|ed|ing)?|whack(?:s|ed|ing)?|head-?butt(?:s|ed|ing)?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_HIT_ME"); }
    s_ABUSE_HIT_ME(py_tuple(vec![op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b")), &g_ABUSE_VERB()), &py_str("\\s+me\\b")), op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b")), &g_ABUSE_VERB_PART()), &py_str("\\s+(?:my|me\\s+in\\s+my)\\s+(?:face|head|back|arm|arms|leg|legs|ribs?|stomach|belly|jaw|eye|eyes|ear|ears|neck|throat|shoulders?|chest|side|hip|hips|thigh|mouth|nose|teeth|wrist|knee)\\b(?![^.?!]{0,40}\\b(?:cake|cakes|cream|icing|jelly|bun|buns|pudding|pie|pies|flour|bunting|snow|snowdrift|leaves|foam|bubbles|mud|sand|dough|pillow\\s+fight)\\b(?![^.?!]{0,60}\\b(?:hit|hits|punch(?:ed|es)?|kick(?:ed|es)?|slap(?:ped|s)?|beat(?:s)?|chok(?:ed|es)?|shov(?:ed|es)?|push(?:ed|es)?)\\s+me\\b(?!\\s+(?:on|round|in)\\s+the\\s+back\\b))(?![^.?!]{0,60}\\b(?:breath|breathe|breathing|breathless|blacked\\s+out|passed\\s+out|unconscious|choking|couldnt\\s+breathe|could\\s+not\\s+breathe|no\\s+air)\\b))")), op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:scratch(?:es|ed|ing)?|claw(?:s|ed|ing)?)\\s+(?:my|me\\s+in\\s+my)\\s+(?:face|head|arm|arms|leg|legs|ribs?|stomach|belly|jaw|eye|eyes|ear|ears|neck|throat|shoulders?|chest|side|hip|hips|thigh|mouth|nose|teeth|wrist|knee)\\b(?![^.?!]{0,30}\\b(?:on|against|through|by)\\s+(?:the|a|an|some|my|his|her|their|our)?\\s*\\w+)(?![^.?!]{0,40}\\b(?:bleed|bleeds|bled|bleeding|blood|bruise|bruises|bruised|marks|scarred|again|police|scared|afraid|frightened|threaten\\w*|strangl\\w*|choke\\w*|hit|hits|punch\\w*|kick\\w*|beat\\w*)\\b)"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.HAS_ME_SUBJECT"); }
    s_HAS_ME_SUBJECT(op_add(&op_add(&py_str("\\b(?:he|she|they|he'?s|she'?s|they'?re|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GRIP_GAME"); }
    s_GRIP_GAME(py_str("\\b(?:squash|tennis|judo|karate|mma|taekwondo|rugby|football|netball|wrestl\\w*|sparring|dojo|scrum|scrums|matches?)\\b"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GRIP_GAME_FENCE"); }
    s_GRIP_GAME_FENCE(op_add(&op_add(&py_str("(?![^.?!]{0,30}"), &g_GRIP_GAME()), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GRIP_NOT_AN_ASSAULT"); }
    s_GRIP_NOT_AN_ASSAULT(py_tuple(vec![g_GRIP_GAME_FENCE(), py_str("(?![^.?!]{0,30}\\b(?:play\\w*|tickl\\w*|laugh\\w*|giggl\\w*|joking|for\\s+fun)\\b)(?![^.?!]{0,30}\\b(?:deadline|deadlines|budget|bills?|rent|mortgage|money|hours|rota|shift|shifts|clock|targets?|emails?|paperwork)\\b)(?![^.?!]{0,30}\\b(?:haircut|while\\s+(?:he|she|they|it)\\s+cut)\\b)")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GRIP_NOT_AN_ASSAULT_STR"); }
    s_GRIP_NOT_AN_ASSAULT_STR(mcall(&py_str(""), "join", &[g_GRIP_NOT_AN_ASSAULT()]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GAME_WORD_STOP"); }
    s_GAME_WORD_STOP(py_str("(?!the\\b|a\\b|an\\b|home\\b|house\\b|work\\b|school\\b|night\\b|nights\\b|bed\\b|morning\\b|afternoon\\b|evening\\b|weekend\\b|weekends\\b|front\\b|public\\b|private\\b|kitchen\\b|bathroom\\b|bedroom\\b|hall\\b|stairs\\b|car\\b|street\\b|garden\\b|pub\\b|court\\b|office\\b|hospital\\b|head\\b|face\\b|ribs\\b|back\\b|stomach\\b|chest\\b|arm\\b|arms\\b|leg\\b|legs\\b|neck\\b|throat\\b|gut\\b|kidney\\b|kidneys\\b|hands?\\b|fists?\\b|feet\\b|foot\\b|shoulders?\\b|knees?\\b|elbows?\\b)[\\w'-]+"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GAME_PLAYED"); }
    s_GAME_PLAYED(op_add(&py_str("\\bbeat(?:s|en|ing)?\\s+(?:me|us)\\s+(?:at|in)\\s+(?:the\\s+|a\\s+)?"), &g_GAME_WORD_STOP()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.GAME_AFTER_ME"); }
    s_GAME_AFTER_ME(op_add(&op_add(&py_str("(?!\\s+(?:at|in)\\s+(?:the\\s+|a\\s+)?"), &g_GAME_WORD_STOP()), &py_str(")")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ASK_AFTER_ME"); }
    s_ASK_AFTER_ME(py_str("(?!\\s+up\\s+(?:for|to)\\b)"));
    fn __data3() -> Py { { let mut __v: Vec<Py> = Vec::new(); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:holds?|keeps?|pins?|grips?|presses?)\\s+(?:me|us)\\s+(?:up\\s+)?(?:against|into|onto)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:wall|floor|door|bed|ground|fridge|cooker|cupboard|sofa|table)\\b"))); __v.extend(py_iter_or_raise(&g_ABUSE_HIT_ME())); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b(?:thr(?:ew|ows|owing)|hurl(?:ed|s)|lobbed|shov(?:e|es|ed)|push(?:es|ed)|swung|swing|lunged)\\b.{0,25}\\b(?:at|against)\\s+(?:me|my\\s+(?:head|face|arm|back|leg|son|daughter|kids?|child|mum|dad))\\b"))); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,25}\\b(?:hands?|fingers?|arm)\\s+(?:round|around|on|across)\\s+(?:my\\s+)?(?:throat|neck)\\b"))); __v.push(py_str("\\bput\\s+(?:his|her|their)\\s+hands?\\s+(?:round|around|on)\\s+(?:my\\s+)?(?:throat|neck)\\b")); __v.push(op_add(&py_str("\\b(?:held|pinned|pressed)\\s+me\\s+(?:down|against\\s+the\\s+(?:wall|floor|door|bed)|on\\s+the\\s+(?:floor|ground|bed)|in\\s+the\\s+(?:kitchen|bathroom|bedroom|hall|hallway))\\b"), &g_GRIP_NOT_AN_ASSAULT_STR())); __v.push(op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,25}\\b(?:held|pinned|pressed|pushed|shoved|backed|slammed|threw)\\s+me\\s+(?:up\\s+)?(?:against|onto|into|to)\\s+the\\s+(?:wall|floor|door|bed|fridge|cooker|counter|cupboard|sofa|ground)\\b")), &g_GRIP_GAME_FENCE())); __v.push(op_add(&g_HAS_ME_SUBJECT(), &py_str(".{0,20}\\b(?:has|had|have|got|gets|keeps?|kept|holds?|held|leaves?|left|pins?|pinned)\\s+me\\s+(?:pinned|trapped|cornered|locked\\s+in|pressed|held|shoved|pushed|backed|slammed)\\b(?!\\s+(?:to|on|onto|against|into|in|by)\\s+(?:the\\s+|a\\s+|my\\s+|his\\s+|her\\s+|their\\s+|our\\s+)?(?:rota|rotas|shift|shifts|roster|rosters|list|lists|board|boards|schedule|schedules|duty|duties|plan|plans|app|apps|desk|desks|office|meeting\\s+room|conference\\s+room|trolley|till|screen|monitor|keyboard|in-?tray|inbox|deadline|deadlines|shop\\s+floor|checkout|call\\s+list)\\b)"))); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:corner(?:ed|s)?|trapped|backed|blocked|pinned)\\s+me\\s+(?:in|into)\\s+(?:the\\s+|my\\s+|a\\s+)?(?:kitchen|bedroom|bathroom|bath|hall|hallway|corridor|lift|stairwell|stairs|garage|flat|house|room|doorway|balcony|landing|porch|study|shed)\\b"))); __v.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:wouldn?'?t|won'?t|would|will|refused?|refuses?)\\s*(?:not\\s+)?(?:to\\s+)?(?:let|allow)\\s+me\\s+")), &g__REFUSED_EXIT()), &py_str("\\b"))); __v.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,25}\\b(?:doesn'?t|don'?t|didn'?t|(?:does|do|did)\\s*not)\\s*let\\s+me\\s+")), &g__REFUSED_EXIT()), &py_str("\\b"))); __v.push(op_add(&op_add(&py_str("\\b(?:locked|shut|trapped|stuck)\\s+(?:in|inside)\\s+(?:the\\s+|my\\s+|a\\s+|this\\s+)?(?:bathroom|bedroom|toilet|kitchen|room|flat|house|hall|hallway|car|cellar|basement|loft|attic)\\b[^.?!]{0,40}\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|hammer(?:ing|s)?|kick(?:ing|s)?|smash(?:ing|es)?|breaking|shout(?:ing|s)?|scream(?:ing|s)?|rattl(?:ing|es))\\b"))); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|hammer(?:ing|s)?|kick(?:ing|s)?|shout(?:ing|s)?|scream(?:ing|s)?)\\b[^.?!]{0,40}\\b(?:locked|shut|trapped|stuck)\\s+(?:in|inside)\\b"))); __v.push(op_add(&op_add(&g_HAS_ME_SUBJECT(), &py_str("[^.?!]{0,15}\\b(?:has|have|had|got|keeps?|kept|holds?|held|grabs?|grabbed|drags?|dragged|pulls?|pulled|takes?|took)\\s+me\\s+by\\s+(?:the|my|his|her|their)\\s+(?:throat|neck|hair|scalp|collar|scruff|jaw)s?\\b")), &g_GRIP_NOT_AN_ASSAULT_STR())); __v.push(op_add(&op_add(&g_HAS_ME_SUBJECT(), &py_str("[^.?!]{0,15}\\b(?:has|have|had|got|keeps?|kept|holds?|held|pins?|pinned|presses?|pressed|shoves?|shoved|pushes?|pushed)\\s+me\\s+(?:up\\s+)?(?:against|onto|into|to)\\s+(?:the\\s+|a\\s+|my\\s+)?(?:wall|floor|door|bed|ground|fridge|cooker|cupboard|sofa|counter|table|car)\\b")), &g_GRIP_NOT_AN_ASSAULT_STR())); __v.push(py_str("\\b(?:shook|shak(?:es|ing))\\s+me\\s+(?:hard|violently|roughly|until|so\\s+hard)\\b")); __v.push(py_str("\\b(?:grab(?:s|bed|bing)?|yank(?:s|ed|ing)?|twist(?:s|ed|ing)?|wrench(?:es|ed|ing)?|pull(?:s|ed|ing)?)\\s+(?:me\\s+)?by\\s+the\\s+(?:hair|throat|neck|arm|wrist|collar|scruff)\\b")); __v.push(py_str("\\b(?:grab(?:s|bed|bing)?|yank(?:s|ed|ing)?)\\s+my\\s+(?:arm|wrist|hair|throat|neck|collar)\\b")); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,30}\\b(?:hits?|hitting|beat|beats|beating|slapp?(?:s|ed)?|punch(?:es|ed)?|kick(?:s|ed)?|struck|thump(?:s|ed)?|whack(?:s|ed)?|smack(?:s|ed)?|belt(?:s|ed)?|cuff(?:s|ed)?)\\b\\s+(?:me\\s+(?:in|on|across|round|about)\\s+)?(?:the|my)\\s+(?:face|head|arm|ribs|stomach|back|throat|neck|jaw|cheek|nose|eye|eyes|leg|hip|shoulder|wrist|chest|side|belly|ear|mouth|thigh|knee)s?\\b"))); __v.push(op_add(&op_add(&op_add(&py_str("\\b(?:he|she|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\bthreaten(?:s|ed|ing)?\\s+me\\b")), &g__NOT_A_THREAT())); __v.push(op_add(&op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\bthreaten(?:s|ed|ing)?\\s+(?:me\\b|us\\b|to\\s+(?:kill|hurt|hit|stab)\\s+me)")), &g__NOT_A_THREAT())); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+"), &g_ABUSER()), &py_str(")\\b.{0,30}\\b(?:said|says|told\\s+me)?\\s*(?:he|she|they)?\\s*(?:would|will|is\\s+going\\s+to|was\\s+going\\s+to)\\s+(?:kill|hurt|stab|rape|strangle|shoot)\\s+me\\b"))); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they)\\s+(?:(?:won'?t|wont|wouldn'?t|wouldnt|didn'?t|doesn'?t|refus(?:es?|ed)\\s+to)\\s+(?:let|allow)|never\\s+lets?)\\s+me\\s+"), &g__REFUSED_EXIT()), &py_str("\\b"))); __v.push(py_str("\\b(?:abusing|abused|abuses)\\s+me\\b")); __v.push(py_str("\\b(?:sexual(?:ly)?\\s+assault(?:ed)?|raped|raping)\\b")); __v.push(py_str("\\bhits?\\s+me\\s+(?:when|if|again)\\b")); __v.push(py_str("\\bafraid\\s+(?:of|to\\s+go)\\s+(?:him|her|them|home|back\\s+home)\\b")); __v.push(py_str("\\bscared\\s+(?:of|to\\s+go)\\s+(?:him|her|home|back\\s+home|my\\s+(?:husband|partner|dad|mum|boyfriend|girlfriend|carer))\\b")); __v.push(py_str("\\b(?:locks?|locked)\\s+me\\s+(?:in|out)\\b")); __v.push(op_add(&op_add(&py_str("\\b(?:"), &g_THREAT_SUBJECT()), &py_str("|the\\s+(?:man|woman|lady|guy|bloke))\\b[^.?!]{0,20}\\b(?:trapped|pinned|wedged|locked|shut)\\s+(?:me|us)\\s+(?:in|inside|under|into)\\b"))); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|dad|mum|mom|father|mother|son|daughter|brother|sister|step(?:dad|mum|mom|father|mother|son|daughter|brother|sister)|carer|boy|girl|nan|nana|gran|granny|uncle|aunt|auntie|cousin|in-?laws?))\\s+(?:won'?t|wont|wouldn'?t|wouldnt|refus(?:es?|ed)\\s+to|refuse\\s+to|never\\s+lets?\\s+me|didn'?t\\s+let|doesn'?t\\s+let|did\\s+not\\s+let)\\s+let\\s+me\\s+"), &g__REFUSED_EXIT()), &py_str("\\b"))); __v.push(py_str("\\bthreatened?\\s+(?:to\\s+kill|me\\s+with)\\b")); __v.push(op_add(&op_add(&op_add(&py_str("\\b(?:(?:he|she|they|someone|somebody|my\\s+"), &g_CLOSE()), &py_str(")'?s|they'?re|is|are|keeps?|has\\s+been|have\\s+been)\\s+threatening\\s+(?:me|us)\\b")), &g__NOT_A_THREAT())); __v.push(py_str("\\b(?:i'm|i\\s+am)\\s+(?:being\\s+)?(?:controlled|coerced)\\b")); __v.push(op_add(&op_add(&py_str("\\b(?:i'm|i\\s+am|im|i\\s+was|i\\s+have\\s+been|i've\\s+been|ive\\s+been|i\\s+keep\\s+being|i\\s+keep\\s+getting)\\s+(?:being\\s+|getting\\s+)?(?:abused|threatened|stalked|followed|watched|harassed|hit|attacked|assaulted|raped|molested|groomed|beaten\\s+up)\\b"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT())); __v.push(op_add(&op_add(&py_str("\\b(?:i'm|i\\s+am|im)\\s+(?:being|getting)\\s+(?:choked|strangled)\\b"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT())); __v.push(op_add(&op_add(&py_str("\\b(?:i\\s+was|i\\s+have\\s+been|i've\\s+been|ive\\s+been|i\\s+keep\\s+(?:being|getting))\\s+(?:being\\s+|getting\\s+)?(?:choked|strangled)\\b(?!\\s+(?:about|over)\\b)"), &g_BY_NOT_A_THING()), &g__NOT_A_THREAT())); __v.push(op_add(&py_str("\\b(?:he|she|they|my\\s+\\w+)'?(?:s|ve)?\\s+(?:been\\s+)?(?:hurting|hitting|strangling)\\s+me\\b"), &g_ASK_AFTER_ME())); __v.push(op_add(&op_add(&py_str("\\b(?:he|she|they|my\\s+\\w+)'?(?:s|ve)?\\s+(?:been\\s+)?beating\\s+me\\b"), &g_GAME_AFTER_ME()), &g_ASK_AFTER_ME())); __v.push(py_str("\\bmy\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer)\\s+is\\s+(?:being\\s+)?(?:abusive|violent)\\b|\\bmy\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer)\\s+is\\s+(?:controlling|coercive)\\b")); __v.push(py_str("\\b(?:he|she|they)'?s?\\s+(?:being|is|keeps?\\s+being)\\s+(?:abusive|violent)\\b")); __v.push(py_str("\\b(?:he|she|they)\\s+is\\s+being\\s+(?:abusive|violent)\\b")); __v.push(py_str("\\bmy\\s+(?:violent|abusive|controlling|coercive|dangerous|aggressive|threatening)\\s+(?:partner|husband|wife|boyfriend|girlfriend|dad|mum|father|mother|ex|carer|son|daughter|brother|sister|stepdad|stepdad|stepmum|stepfather|stepmother)\\b")); __v.push(op_add(&op_add(&op_add(&py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person)|the\\s+(?:man|woman|person))\\b[^.?!]{0,25}\\b"), &g_ABUSE_VERB()), &py_str("\\s+me\\b")), &g_HIT_BY_A_VEHICLE())); __v.push(op_add(&py_str("\\b(?:someone|somebody)\\b[^.?!]{0,25}\\bhurt(?:s|ing)?\\s+me\\b"), &g_HIT_BY_A_VEHICLE())); __v.push(py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person|partner)|the\\s+(?:man|woman|person|partner))\\b[^.?!]{0,25}\\b(?:is\\s+|was\\s+|being\\s+|been\\s+)?(?:abusive|violent|controlling|coercive|dangerous|aggressive)\\b")); __v.push(py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad|person)|the\\s+(?:man|woman|person))\\b[^.?!]{0,25}\\b(?:scares|frightens|terrifies|intimidates)\\s+me\\b")); __v.push(py_str("\\b(?:i|we|he|she|they)\\s+(?:was|were|got|just\\s+got)\\s+(?:attacked|assaulted|mugged|beaten\\s+up)\\b")); __v.push(py_str("\\b(?:i'm|i\\s+am)\\s+(?:scared|terrified)\\s+of\\s+(?:going\\s+home|him|her)\\b")); __v.push(py_str("\\b(?:someone|somebody|a\\s+(?:man|bloke|guy|woman|lad)|(?:the|our|my)\\s+neighbours?|next\\s+door)\\b[^.?!]{0,30}\\bthreaten(?:s|ed|ing)?\\s+(?:me\\b|to\\s+(?:kill|hurt|harm|hit|stab|beat)\\s+me)")); py_tuple(__v) } }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE"); }
    s_ABUSE(op_add(&g_THREAT_COERCIVE(), &__data3()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PARTNER_CLOSE"); }
    s_PARTNER_CLOSE(py_str("(?:ex|exes|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|baby\\s*daddy|baby\\s*dad|baby\\s*mother|baby\\s*mum)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_SUBJECT"); }
    s_PURSUIT_SUBJECT(py_tuple(vec![op_add(&op_add(&py_str("\\b(?:my\\s+"), &g_PARTNER_CLOSE()), &py_str("|my\\s+ex'?s?)\\b"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.NOT_A_FEAR"); }
    s_NOT_A_FEAR(py_str("(?!\\s+(?:that\\s+)?(?:hes|shes|theyre|its|it'?s|im|i'?m|ill|i'?ll|hell|well|going\\s+to\\s+be|we\\s+will|we'?ll)\\s*(?:going\\s+to\\s+be\\s+)?(?:late|late\\s+home|in\\s+trouble|on\\s+time))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_FEAR"); }
    s_PURSUIT_FEAR(py_tuple(vec![op_add(&py_str("\\b(?:i'?m|i\\s+am|im)\\s+(?:(?:really|so|very|that|proper|properly|absolutely|dead|scared\\s+witless)\\s+)*(?:scared|frightened|terrified|afraid|panicking|shaking|worried)\\b"), &g_NOT_A_FEAR()), py_str("\\b(?:scared|frightened|terrified|afraid)\\s+(?:of|to\\s+go|to\\s+leave|to\\s+open)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_SUBJECT_EX"); }
    s_PURSUIT_SUBJECT_EX(py_tuple(vec![py_str("\\b(?:my\\s+(?:ex|exes|baby\\s*daddy|baby\\s*dad|baby\\s*mother|baby\\s*mum)|my\\s+ex'?s?)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_OUTSIDE"); }
    s_PURSUIT_OUTSIDE(py_tuple(vec![py_str("\\b(?:waiting|waits|sat|sitting|stood|standing|parked|lurking|hiding|hovering|hanging\\s+around)\\s+(?:right\\s+|still\\s+|now\\s+)?(?:outside|out\\s+front|in\\s+the\\s+car\\s+outside|round\\s+the\\s+back)\\b"), py_str("\\bwait(?:ing|s|ed)?\\s+for\\s+(?:me|us)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_PLACE_WORK"); }
    s_PURSUIT_PLACE_WORK(py_tuple(vec![py_str("\\b(?:outside|in\\s+front\\s+of|at\\s+the\\s+end\\s+of|round\\s+the\\s+back\\s+of)\\s+(?:my|our|the)\\s+(?:work|workplace|office|school|nursery|college|uni|university|gates)\\b"), py_str("\\b(?:waiting|waits|sat|sitting|stood|standing|parked|lurking|hiding|hovering|hanging\\s+around)\\s+(?:right\\s+|still\\s+|now\\s+)?(?:outside|near)\\s+(?:my|our|the)\\s+(?:work|workplace|office|school|nursery|college|uni|university|gates)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT_LOUD"); }
    s_PURSUIT_LOUD(py_tuple(vec![py_str("\\b(?:shouting|yelling|screaming)\\s+at\\s+(?:the\\s+)?(?:football|telly|tv|match|game|radio|news|kids|dog|cat|ref|referee)\\b(?!.*\\b(?:i'?m|i\\s+am|im)\\s+(?:scared|terrified))"), py_str("\\b(?:shouting|yelling|screaming|drunk|hammering|banging|pounding)\\b"), py_str("\\b(?:won'?t|wont|refuses?\\s+to)\\s+(?:leave|go|move)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PURSUIT"); }
    s_PURSUIT(py_tuple(vec![_both_here(&[g_PURSUIT_SUBJECT(), g_PURSUIT_PLACE_HOME()]), _both_here(&[g_PURSUIT_SUBJECT(), py_tuple(vec![py_str("\\b(?:is|was|keeps?|still)\\s+(?:right\\s+)?(?:outside|here|back)\\b")]), op_add(&g_PURSUIT_FEAR(), &g_PURSUIT_LOUD())]), _both_here(&[g_PURSUIT_SUBJECT_EX(), g_PURSUIT_PLACE_WORK()]), _both_here(&[g_PURSUIT_SUBJECT_EX(), g_PURSUIT_OUTSIDE()]), py_str("\\b(?:he|she|they|hes|shes|theyre)'?s?\\b[^.?!]{0,12}\\b(?:follow(?:s|ing|ed)?|stalk(?:s|ing|ed)?)\\s+(?:me|us)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL_PERSON"); }
    s_ABUSE_VERBAL_PERSON(py_str("(?:he|she|they|hubby|husband|wife|partner|boyfriend|girlfriend|fiance|fiancee|ex|mum|mom|mam|mother|dad|father|stepdad|stepmum|stepmom|stepfather|stepmother|nan|nana|gran|granny|grandma|grandad|grandpa|carer)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL_MAD"); }
    s_ABUSE_VERBAL_MAD(py_str("(?:mental|crazy|mad)(?!\\s+(?:(?:about|over)\\s+(?!everything|anything|every\\s+thing|all\\s+(?:of|the)|whatever|how\\s+i|everyone|every\\s+time|being|doing|saying|wanting)|at\\s+(?!the\\s+time|first|least)|in\\s+(?:love|bed|a\\s+good\\s+way|the\\s+good\\s+way|a\\s+kind\\s+way)))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL_LABEL"); }
    s_ABUSE_VERBAL_LABEL(op_add(&op_add(&py_str("(?:names|stupid|thick|dumb|useless|worthless|lazy|fat|ugly|pathetic|hopeless|a\\s+failure|a\\s+loser|a\\s+waste\\s+of\\s+space|a\\s+waste|nothing|"), &g_ABUSE_VERBAL_MAD()), &py_str("|psycho|disgusting|a\\s+slag|a\\s+slut|a\\s+bitch|a\\s+cow|rubbish|a\\s+disappointment|a\\s+mistake|an\\s+idiot|an\\s+imbecile|stupid\\s+cow|fat\\s+cow|silly\\s+cow|cretin|a\\s+joke|unloveable|unlovable|a\\s+burden|a\\s+drain|a\\s+piece\\s+of\\s+shit|a\\s+piece\\s+of\\s+crap|shit)")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.CHILD_AGGRESSOR"); }
    s_CHILD_AGGRESSOR(slice_or_raise(&py_tuple(vec![py_str("\\b(?:(?:my|our|his|her|their|the)\\s+(?:son|sons|daughter|daughters|child|children|kid|kids|toddler|baby|teen|teenager|teenagers|boy|girl|boys|girls|little\\s+one)|my\\s+(?:eldest|oldest|youngest))\\b(?!\\s+[^.?!]{0,12}\\b(?:when|while|as)\\s+(?:he|she|they)\\b)[^.?!]{0,12}\\b(?:pushes?|pushed|pushing|hits?|hit|hitting|kicks?|kicked|kicking|slaps?|slapped|slapping|bites?|bit|biting|punches?|punched|punching|calls?\\s+me|calling\\s+me|called\\s+me|swears?\\s+at\\s+me|shouts?\\s+at\\s+me|yells?\\s+at\\s+me)\\b")]), &Py::None, &Py::Int(1), &Py::None));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL_SUBJECT"); }
    s_ABUSE_VERBAL_SUBJECT(op_add(&op_add(&py_str("\\b(?:he|she|they|he'?s|she'?s|they'?re|my\\s+"), &g_ABUSE_VERBAL_PERSON()), &py_str(")\\b")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL"); }
    s_ABUSE_VERBAL(py_tuple(vec![op_add(&op_add(&op_add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,25}\\b(?:calls?|calling|called|keeps\\s+calling)\\s+me\\s+(?:(?:a|an)\\s+)?")), &g_ABUSE_VERBAL_LABEL()), &py_str("\\b")), op_add(&op_add(&op_add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,25}\\b(?:tells?|telling|told|says?|saying|said)\\s+(?:me\\s+)?i(?:'?m|\\s+am)\\s+(?:(?:a|an)\\s+)?")), &g_ABUSE_VERBAL_LABEL()), &py_str("\\b")), op_add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,30}\\b(?:keeps?|kept|always|constantly|regularly|never\\s+stops|every\\s+(?:day|night|week))\\b[^.?!]{0,20}\\b(?:putting\\s+me\\s+down|put\\s+me\\s+down|running\\s+me\\s+down|having\\s+a\\s+go\\s+at\\s+me)\\b")), op_add(&g_ABUSE_VERBAL_SUBJECT(), &py_str("[^.?!]{0,30}\\b(?:puts?\\s+me\\s+down|putting\\s+me\\s+down|runs?\\s+me\\s+down|running\\s+me\\s+down|has\\s+a\\s+go\\s+at\\s+me|having\\s+a\\s+go\\s+at\\s+me)\\b[^.?!]{0,25}\\b(?:all\\s+the\\s+time|all\\s+day|constantly|always|every\\s+(?:day|night|week|time)|non[\\s-]?stop)\\b"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_VERBAL_FINE"); }
    s_ABUSE_VERBAL_FINE(py_tuple(vec![py_str("\\b(?:call|calls|calling|called)\\s+each\\s+other\\b"), py_str("\\b(?:we|we'?re|were)\\s+(?:both\\s+)?(?:call|calls|calling|tease|teasing|wind)\\b"), py_str("\\b(?:me\\s+and|between\\s+me\\s+and|my\\s+(?:mates?|friends?|lads|colleagues?|team|classmates?|schoolmates?|flatmates?|housemates?))\\b[^.?!]{0,20}\\b(?:call|calls|calling|tease|teasing|rip\\s+into)\\b"), py_str("\\b(?:nickname|nicknames|term\\s+of\\s+endearment|pet\\s+name|affectionately|fondly)\\b"), py_str("\\b(?:in\\s+fun|for\\s+a\\s+laugh|in\\s+good\\s+fun|in\\s+jest|in\\s+play|at\\s+five-a-side|on\\s+the\\s+pitch|at\\s+rugby|at\\s+football|in\\s+training)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_HEAD_INTO"); }
    s_ABUSE_HEAD_INTO(op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)|slam(?:med|s)?|bang(?:ed|s|ing)?|smash(?:ed|es|ing)?|knock(?:ed|s|ing)?|ram(?:med|s|ming)?|thr(?:ew|ows|owing)|yank(?:ed|s|ing)?|drag(?:ged|s|ging)?|smack(?:ed|s|ing)?|press(?:ed|es|ing)?)\\b[^.?!]{0,15}\\bmy\\s+(?:head|face)\\b[^.?!]{0,15}\\b(?:into|onto|against|off|through)\\b\\s*(?:the\\s+|a\\s+|his\\s+|her\\s+)?(?:wall|walls|door|doors|door\\s?frame|doorframe|frame|floor|ground|table|table\\s?top|desk|worktop|counter|cupboard|cabinet|fridge|freezer|radiator|banister|stairs|stair\\s?gate|step|kerb|curb|pavement|window|windowsill|sill|sink|bath|bathroom|toilet|headboard|bed\\s?frame|wardrobe|shelf|shelves|cooker|oven|dashboard|car|van|kitchen\\s?unit|brick\\s?wall|concrete)\\b")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SOFT_FACE_PUSH"); }
    s_SOFT_FACE_PUSH(op_add(&op_add(&py_str("\\b(?:he|she|they|hubby|my\\s+"), &g_ABUSER()), &py_str(")\\b[^.?!]{0,20}\\b(?:push(?:ed|es|ing)?|shov(?:ed|es|ing)|slam(?:med|s)?|bang(?:ed|s|ing)?|smash(?:ed|es|ing)?|ram(?:med|s|ming)?|thr(?:ew|ows|owing)|dunk(?:ed|s|ing)?)\\b[^.?!]{0,15}\\bmy\\s+(?:face|head)\\b[^.?!]{0,15}\\b(?:into|onto|in|through)\\b\\s*(?:(?:the|a|his|her|my|their)\\s+)?(?:\\w+\\s+){0,2}(?:cake|cakes|snow|mud|sand|dough|flour|icing|bubbles|foam|paint|slime)\\b")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SOFT_FACE_STOP"); }
    s_SOFT_FACE_STOP(py_tuple(vec![py_str("\\b(?:hit|hits|hitting|punch(?:ed|es|ing)?|kick(?:ed|es|ing)?|slap(?:ped|s|ping)?|beat(?:s|en)?|chok(?:ed|es|ing)?|shov(?:ed|es)?\\s+me|push(?:ed|es)?\\s+me|held\\s+me|hold(?:ing)?\\s+me|kept\\s+me|held\\s+it|pinned)\\b"), py_str("\\b(?:can'?t|couldn'?t|could\\s+not|cannot|unable\\s+to|struggl\\w*\\s+to)\\s+breath\\w*\\b"), py_str("\\b(?:gasping|choking|unconscious|blue\\s+in\\s+the\\s+face)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PLAY_FACE_REPLIES"); }
    s_PLAY_FACE_REPLIES(py_tuple(vec![py_str("A proper mess. Were you both laughing, or did it land badly?"), py_str("Ha - somebody got you. Was it a laugh, or did that land badly?"), py_str("That's the classic. Were you both laughing, or did it not feel funny to you?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE"); }
    s_ABUSE(op_add(&op_add(&op_add(&g_ABUSE(), &g_PURSUIT()), &g_ABUSE_VERBAL()), &py_tuple(vec![g_ABUSE_HEAD_INTO()])));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_PURSUIT_HARD"); }
    s_ABUSE_PURSUIT_HARD(op_add(&op_add(&py_tuple(vec![op_add(&op_add(&py_str("\\b"), &g_ABUSE_VERB()), &py_str("\\s+(?:me|us)\\b"))]), &g_PURSUIT_FEAR()), &py_tuple(vec![py_str("\\b(?:drunk|threatening|hit|hitting|kicked|pushed|grabbed)\\b"), py_str("\\b(?:won'?t|wont|refuses?)\\s+(?:leave|go|let)\\b")])));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_DISCLOSURE"); }
    s_ABUSE_DISCLOSURE(py_tuple(vec![py_str("\\b(?:i|we)\\s+(?:was|were|have\\s+been|'ve\\s+been|had\\s+been)\\s+(?:being\\s+|getting\\s+)?(?:abused|assaulted|raped|molested|groomed|stalked|harassed|threatened|beaten|hit|attacked|choked|strangled)\\b"), py_str("\\b(?:i|we)\\s+(?:was|were)\\s+in\\s+an?\\s+(?:abusive|violent)\\s+(?:relationship|marriage|home)\\b"), py_str("\\b(?:i|we)\\s+used\\s+to\\s+be\\s+(?:beaten|hit|abused|assaulted)\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_DISCLOSURE_FINE"); }
    s_ABUSE_DISCLOSURE_FINE(py_tuple(vec![py_str("\\bthreatened\\s+(?:with|over|about)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:being\\s+)?(?:written\\s+)?(?:warning|fine|fines|eviction|legal|lawsuit|court|police|report|complaint|solicitor|letter|bills?|charges?|prosecution|dismissal|sack|sack(?:ing|ed)|redundancy|redundant|fired|firing|disciplinary|hearing|notice|tenancy|deposit)\\b")]));
    fn __data4() -> Py { py_tuple(vec![py_str("\\bas\\s+a\\s+(?:child|kid|baby|boy|girl|teenager|teen|little\\s+(?:boy|girl|kid|one))\\b"), py_str("\\bgrowing\\s+up\\b"), py_str("\\bin\\s+my\\s+(?:childhood|teens|twenties|thirties)\\b"), py_str("\\bwhen\\s+i\\s+was\\s+(?:a\\s+(?:child|kid|baby|teenager|little\\s+\\w+)|young|little|small|at\\s+school|in\\s+school|at\\s+college|at\\s+university)\\b"), py_str("\\bat\\s+(?:school|primary|secondary|college|university)\\b"), py_str("\\bat\\s+my\\s+(?:old|previous|last|former)\\s+(?:job|work|school|house|flat)\\b"), py_str("\\byears\\s+and\\s+years\\s+ago\\b"), py_str("\\byears\\s+ago\\b"), py_str("\\bages\\s+ago\\b"), py_str("\\bdecades?\\s+ago\\b"), py_str("\\ba\\s+long\\s+(?:time|while)\\s+ago\\b"), py_str("\\bback\\s+(?:then|in\\s+(?:the\\s+)?(?:\\d0s|\\d{4}))\\b"), py_str("\\bin\\s+the\\s+\\d0s\\b"), py_str("\\bused\\s+to\\b"), py_str("\\bmy\\s+(?:childhood|past)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_MEMORY"); }
    s_ABUSE_MEMORY(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_NOW"); }
    s_ABUSE_NOW(py_tuple(vec![py_str("\\b(?:still|now|again)\\s+(?:\\w+\\s+){0,2}(?:does|doing|hits?|hurts?\\s+me|beats?|threatens?|follows?|watches?|comes?|turns?\\s+up|shows?\\s+up|calls?|texts?|happens?|happening|at\\s+it)\\b"), py_str("\\b(?:keeps?\\s+(?:doing|on|at)|hasn'?t\\s+stopped|never\\s+stopped|carries\\s+on)\\b"), py_str("\\b(?:it|this|he|she)\\s*(?:'s|s|\\s+is)?\\s*(?:happening|going\\s+on|happened)\\s+again\\b"), py_str("\\bat\\s+it\\s+again\\b"), py_str("\\b(?:every|each)\\s+(?:day|night|morning|week|evening)\\b"), py_str("\\b(?:nowadays|these\\s+days)\\b"), py_str("\\b(?:tonight|yesterday|last\\s+night|this\\s+(?:morning|week))\\b")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.BLOW_THINGS"); }
    s_BLOW_THINGS(py_str("(?:bottles?|glasses?|pans?|frying\\s+pans?|saucepans?|bats?|baseball\\s+bats?|belts?|belt\\s+buckles?|chairs?|stools?|cups?|mugs?|plates?|bowls?|knife|knives|sticks?|canes?|hammers?|bricks?|stones?|rocks?|remotes?|remote\\s+controls?|shoes?|boots?|slippers?|trainers?|books?|newspapers?|candlesticks?|spanners?|wrenches?|wires?|cables?|cords?|pipes?|rods?|bars?|iron\\s+bars?|rolling\\s+pins?|ladles?|spoons?|wooden\\s+spoons?|forks?|pokers?|torches?|phones?|lamps?|vases?|ashtrays?|tools?|hands?|fists?|feet|foot|elbows?|knees?|shoulders?)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SOFT_HIT_THING"); }
    s_SOFT_HIT_THING(py_str("(?:pillow|cushion|duvet|blanket|quilt|snowball(?:s)?|ball(?:s)?|beach\\s+ball|plastic\\s+ball|tennis\\s+ball|frisbee|sponge|flannel|tea\\s+towel|towel|newspaper|magazine|paper|roll?ed[\\s-]?up\\s+(?:paper|newspaper|magazine|towel)|water\\s?bomb|water\\s?balloon|water\\s+pistol|sock|slipper|plush\\s+toy|teddy|toy|stickle\\s?brick|lego|balloon|cushion\\s+case)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.PLAY_NAMED"); }
    s_PLAY_NAMED(py_str("(?:pillow\\s+fight|snowball\\s+fight|water\\s+fight|food\\s+fight|play\\s?fight\\w*|nerf\\s+(?:gun|war|fight)|paintball|tickl\\w+|muck(?:ing)?\\s+(?:about|around)|mess(?:ing)?\\s+(?:about|around)|fool(?:ing)?\\s+(?:about|around)|lark(?:ing)?\\s+(?:about|around)|jok(?:ing|e)\\s+(?:about|around)|banter|we\\s+(?:were|was|are)\\s+(?:only\\s+|just\\s+)?(?:playing|mucking|messing|fooling|larking|joking|having\\s+a\\s+laugh)|we\\s+both\\s+(?:laughed|were\\s+laughing|laugh)|we\\s+were\\s+laughing|playing\\s+(?:catch|football|rounders|netball|basketball|tag|a\\s+game|games|together))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.SOFT_HIT_VERB"); }
    s_SOFT_HIT_VERB(py_str("(?:hit|hits|hitting|slapped|slaps|smacked|whacked|pelted|threw|throws|thrown|sprayed|squirted|flicked|dunked|dumped|doused)"));
    fn __data5() -> Py { py_tuple(vec![py_str("\\bbeat(?:s|en|ing)?\\s+(?:me|us)\\s+(?:at|in)\\s+(?:chess|cards|poker|pool|snooker|billiards|darts|tennis|badminton|table\\s+tennis|ping\\s?pong|squash|golf|football|rugby|cricket|basketball|netball|scrabble|monopoly|cluedo|checkers|draughts|dominoes|mario\\s?kart|video\\s+games?|the\\s+quiz|the\\s+game|the\\s+final|the\\s+match|the\\s+league|arm\\s?wrestl\\w*)\\b"), g_GAME_PLAYED(), py_str("\\bhit(?:s|ting)?\\s+(?:me|us)\\s+up\\s+(?:for|to)\\b"), py_str("\\b(?:had|has|have|got|gets|kept|keeps)\\s+me\\s+cornered\\s+(?:at|during|after)\\s+(?:the\\s+|a\\s+|his\\s+|her\\s+)?(?:party|pub|wedding|conference|meeting|office|work|do|dinner|reception|barbecue|bbq|gig|match|networking\\s+event)\\b"), py_str("\\bcornered?\\s+me\\s+(?:to\\s+(?:talk|ask|discuss|pitch|sell|persuade|convince|apologise|apologize)|at\\s+(?:the\\s+|a\\s+)?(?:party|pub|wedding|conference|meeting|office|work|do))\\b"), py_str("\\bmy\\s+(?:mum|mom|mam|dad|mother|father|parents?|nan|nana|grandad|grandma)\\b[^.?!]{0,25}\\b(?:let|allow)\\s+me\\s+(?:go|out|stay|leave)\\b"), py_str("\\b(?:landlord|landlady|letting\\s+agent|agent|council|employer|boss|manager|company|firm|university|college|school|gym)\\b[^.?!]{0,30}\\b(?:let|allow)\\s+me\\s+(?:leave|go|out|past)\\b"), py_str("\\b(?:debt\\s+collector|debt\\s+collectors|bailiffs?|bank|mortgage|lender|utility|utilities|energy|water\\s+company|broadband|phone\\s+company|insurer|insurance|hmrc|tax\\s+office|dvla)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\s+(?:me|us)\\b"), py_str("\\b(?:landlord|letting\\s+agent|agent|council|housing|bank|mortgage|utility|utilities|energy|water\\s+company|broadband|phone\\s+company|mobile\\s+network|insurer|insurance|employer|boss|manager|company|firm|agency|payroll|hr\\b|hmrc|tax\\s+office|dvla|university|college|school|gym|debt\\s+collector|solicitor)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b[^.?!]{0,25}\\b(?:over|about|with)\\s+(?:a\\s+|the\\s+|his\\s+|her\\s+|their\\s+|my\\s+|our\\s+)?(?:money|pay|wages|salary|rent|hours|shifts?|rota|arrears|fees?|charges?|debt|bills?)\\b"), py_str("\\bhit\\s+(?:on|with)\\b"), py_str("\\bfeel(?:s|ing)?\\s+like\\s+(?:i\\s*(?:'m|am)|im)\\s+being\\s+watched\\b"), py_str("\\bharass(?:ed|es|ing|ment)\\b[^.?!]{0,30}\\b(?:at|from)\\s+(?:work|the\\s+office|my\\s+job)\\b"), py_str("\\b(?:thr(?:ew|ows)|push(?:ed|es)|shov(?:ed|es)|kick(?:ed|s)?|hit|hits|slapped)\\b(?!\\s+(?:my|his|her|their)\\b)(?!\\s+me\\s+(?:in|on|round|across)\\b)[^.?!]{0,20}\\b(?:ball|snowball|frisbee|water\\s+balloon|pillow|toy|party|surprise|birthday|early\\s+night|hint|look)\\b"), py_str("\\b(?:push(?:ed|es)|shov(?:ed|es)|thr(?:ew|ows))\\s+me\\s+(?:into|in)\\s+the\\s+(?:pool|water|sea|lake|river|snow|leaves)\\b"), py_str("\\b(?:push(?:ed|es)|shov(?:ed|es))\\s+me\\s+(?:in|on)\\s+the\\s+(?:buggy|pushchair|wheelchair|swing|sledge|sled|trolley|bike|boat)\\b"), py_str("\\b(?:my|his|her)\\s+(?:sister|brother|friend|mate|son|daughter|kids?)\\s+(?:hit|hits|punched|pushed|kicked|slapped)\\s+me\\s+(?:at|during|in)\\s+(?:rugby|football|the\\s+match|training|practice|a\\s+game|the\\s+game|playtime|the\\s+pool)\\b"), py_str("\\bmy\\s+(?:son|daughter|child|children|kid|kids|baby|babies|toddler|nephew|niece|grandson|granddaughter)\\b[^.?!]{0,40}\\b(?:bang(?:ing|s)?|pound(?:ing|s)?|kick(?:ing|s)?|hammer(?:ing|s)?|shout(?:ing|s)?)\\b"), py_str("\\bmy\\s+(?:son|daughter|niece|nephew|godson|goddaughter)\\b[^.?!]{0,25}\\b(?:(?:has|have|had|got|keeps?|kept)\\s+me\\s+pinned|pinned\\s+me|wrestl(?:ed|es|ing)\\s+me)\\b"), py_str("\\bmy\\s+(?:kid|kids|child|children|toddler|baby|babies|newborn)\\b[^.?!]{0,25}\\b(?:(?:has|have|had|got|keeps?|kept)\\s+me\\s+(?:pinned|trapped|stuck)|(?:pinned|trapped|wrestl(?:ed|es|ing))\\s+me)\\b"), py_str("\\b(?:grab(?:bed|s)?|held|took)\\b[^.?!]{0,25}\\bto\\s+(?:hug|kiss|say|shake|stop|help|save|pull\\s+me\\s+up|hold|steady|catch|guide|lead|thank|greet|calm)\\b"), py_str("\\b(?:as\\s+(?:i|he|she)\\s+(?:fell|slipped))\\b"), op_add(&op_add(&py_str("\\bhit(?:s|ting)?\\s+me\\s+with\\s+(?!(?:(?:the|a|an|my|his|her|their|this|that)\\s+)?(?:washing|clothes|clothesline|fishing|tow|washing-up|party|garden|climbing)\\s*-?\\s*line\\b)(?:(?:the|a|an|this|that|his|her|their)\\s+)?(?:[\\w'-]+\\s+){0,2}(?:next\\s+)?(?:deadline|deadlines|bill|bills|news|invoice|invoices|fine|tax|taxes|work|project|projects|question|questions|idea|ideas|surprise|party|joke|jokes|pun|punchline|line|lines|quote|quotes|fact|facts|truth|hint|warning|newsflash|update|updates|headline|headlines|statistic|statistics|stats|figure|figures|number|numbers|story|stories|riddle|suggestion|suggestions|advice|compliment|compliments|argument|excuse|excuses|ultimatum|guilt\\s+trip|lecture|earful|mouthful|reality\\s+check|curveball|revelation|rumour|rumor|gossip|opinion|promise|invite|invitation|letter|email|text|phone\\s+call)\\b(?!\\s+of\\s+(?:punches|blows|kicks|hits|slaps|thumps|digs|jabs|beatings|whacks|wallops|shots|smacks|strikes|knocks)\\b)(?!\\s+(?:opener|book|books|plate|plates|sheet|sheets|skate|skates|prop|props|board|boards|rack|stand|clip|gun|punch|hole|weight|pin|block|cutter|knife|bat|pad|bag|cover|holder|rest|frame|case|box|bag|handle|head|piece|booklet|boards?)\\b)(?!(?:[^.?!]{0,40})\\b(?:and|then|but|so)\\b[^.?!]{0,30}(?:hit|hits|hitting|punched|punching|slapped|slapping|kicked|kicking|shoved|shoving|pushed|pushing|grabbed|grabbing|choked|choking|strangled|beat|beating|thumped|whacked|headbutted|pinned|dragged|smashed)\\s+(?:me|my|us)\\b(?!\\s+(?:with|by)\\b))(?![^.?!]{0,40}\\b(?:keeps?|kept)\\s+(?:on\\s+)?(?:hitting|punching|slapping|kicking|shoving|pushing|grabbing|choking|beating|thumping|whacking)\\s+(?:me|my|us)\\b)(?![^.?!]{0,30}\\b(?:and|then|or|but)\\b[\\s,]+(?:(?:then|also|with|by)\\s+)?(?:(?:a|an|the|his|her|their|my|our)\\s+)?"), &g_BLOW_THINGS()), &py_str("\\b(?!\\s+(?:of|on|round|around|to)\\b))")), py_str("\\b(?:bailiffs?|debt\\s+collectors?)\\b[^.?!]{0,40}\\bthreaten(?:s|ed|ing)?\\b"), py_str("\\b(?:playfully|jokingly|in\\s+play|for\\s+fun|messing\\s+about|playfight\\w*|larking\\s+about|tickl\\w+)\\b"), py_str("\\b(?:muck(?:ing)?\\s+(?:about|around)|mess(?:ing)?\\s+(?:about|around)|fool(?:ing)?\\s+(?:about|around)|lark(?:ing)?\\s+(?:about|around)|jok(?:ing|e)\\s+(?:about|around)|muck(?:ing)?\\s+around)\\b"), op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,24}\\b(?:my|the|his|her|your)\\s+(?:head|face)\\b[^.?!]{0,24}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b")), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,40}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b[^.?!]{0,60}")), &g_PLAY_NAMED()), &py_str("\\b")), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_PLAY_NAMED()), &py_str("\\b[^.?!]{0,60}\\b")), &g_SOFT_HIT_VERB()), &py_str("\\b[^.?!]{0,40}\\bwith\\s+(?:a\\s+|an\\s+|the\\s+|some\\s+)?")), &g_SOFT_HIT_THING()), &py_str("\\b")), op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g_PLAY_NAMED()), &py_str("\\b[^.?!]{0,60}\\b(?:threw|throw|thrown|throws|pelted|sprayed|squirted)\\b[^.?!]{0,20}\\b")), &g_SOFT_HIT_THING()), &py_str("\\s+at\\s+(?:me|us)\\b")), py_str("\\bmy\\s+(?:son|daughter|child|kid|baby|toddler|boy|girl|children|stepson|stepdaughter|grandson|granddaughter|nephew|niece|little\\s+one)\\b[^.?!]{0,30}\\b(?:bit|bite|bites|biting)\\s+me\\b"), py_str("\\bmy\\s+(?:baby|toddler|child|kid|little\\s+one|newborn|infant|two\\s+year\\s+old|three\\s+year\\s+old)\\b[^.?!]{0,30}\\b(?:hit|hits|hitting|kicked|kicks|kicking|slapped|slaps|slapping|pinched|pinches|pinching|scratched|scratches|scratching|pushed|pushes|shoved|shoves|headbutted|headbutts)\\s+me\\b"), py_str("\\b(?:they|someone)\\s+(?:are|is|was|were)?\\s*(?:threatening|threatened)\\s+me\\b[^.?!]{0,30}\\b(?:debt|debts|money|loan|loans|repayment|repayments|bill|bills)\\b"), py_str("\\b(?:boss|manager|colleague|coworker|co-worker|supervisor)\\b[^.?!]{0,30}\\bthreaten(?![^.?!]{0,60}\\b(?:kill|kills|killing|murder|hurt|hurts|harm|attack|stab|shoot|strangle|choke|rape|beat|punch|hit|slap|grab|shove|push|throat|weapon|knife|gun)\\b)"), py_str("\\bthreaten(?![^.?!]{0,60}\\b(?:kill|kills|killing|murder|hurt|hurts|harm|attack|stab|shoot|strangle|choke|rape|beat|punch|hit|slap|grab|shove|push|throat|weapon|knife|gun)\\b)[^.?!]{0,30}\\b(?:boss|manager|colleague|coworker|co-worker|supervisor)\\b"), py_str("\\b(?:held|pulled|pushed|shoved)\\s+(?:me|us)\\s+(?:against|to|back\\s+against)\\s+(?:the\\s+)?(?:wall|fence|barrier|side|back)\\b[^.?!]{0,25}\\b(?:to\\s+let|so\\s+they|so\\s+the|to\\s+make\\s+room|to\\s+get\\s+out\\s+of\\s+the\\s+way)\\b")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.safety_phrases.ABUSE_FINE"); }
    s_ABUSE_FINE(__data5());
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
    let mut parts: Py = Py::None;
    let _ = py_str("One thing from each group, all in the line, in whichever order.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&groups.clone()) { let mut group: Py = __item.clone(); __out.push(op_add(&op_add(&py_str("(?=.*(?:"), &mcall(&py_str("|"), "join", &[group.clone()])), &py_str("))"))); } py_list(__out) })() };
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
            "PURSUIT_PLACE_HOME" => Some(g_PURSUIT_PLACE_HOME()),
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
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABUSE" => s_ABUSE(v),
            "ABUSER" => s_ABUSER(v),
            "ABUSE_DISCLOSURE" => s_ABUSE_DISCLOSURE(v),
            "ABUSE_DISCLOSURE_FINE" => s_ABUSE_DISCLOSURE_FINE(v),
            "ABUSE_FINE" => s_ABUSE_FINE(v),
            "ABUSE_HEAD_INTO" => s_ABUSE_HEAD_INTO(v),
            "ABUSE_HIT_ME" => s_ABUSE_HIT_ME(v),
            "ABUSE_MEMORY" => s_ABUSE_MEMORY(v),
            "ABUSE_NOW" => s_ABUSE_NOW(v),
            "ABUSE_PURSUIT_HARD" => s_ABUSE_PURSUIT_HARD(v),
            "ABUSE_VERB" => s_ABUSE_VERB(v),
            "ABUSE_VERBAL" => s_ABUSE_VERBAL(v),
            "ABUSE_VERBAL_FINE" => s_ABUSE_VERBAL_FINE(v),
            "ABUSE_VERBAL_LABEL" => s_ABUSE_VERBAL_LABEL(v),
            "ABUSE_VERBAL_MAD" => s_ABUSE_VERBAL_MAD(v),
            "ABUSE_VERBAL_PERSON" => s_ABUSE_VERBAL_PERSON(v),
            "ABUSE_VERBAL_SUBJECT" => s_ABUSE_VERBAL_SUBJECT(v),
            "ABUSE_VERB_PART" => s_ABUSE_VERB_PART(v),
            "ASK_AFTER_ME" => s_ASK_AFTER_ME(v),
            "BLOW_THINGS" => s_BLOW_THINGS(v),
            "BY_NOT_A_THING" => s_BY_NOT_A_THING(v),
            "CHILD_AGGRESSOR" => s_CHILD_AGGRESSOR(v),
            "CHILD_HIT_NOW" => s_CHILD_HIT_NOW(v),
            "CLOSE" => s_CLOSE(v),
            "DYING_OTHER" => s_DYING_OTHER(v),
            "GAME_AFTER_ME" => s_GAME_AFTER_ME(v),
            "GAME_NOT_A_BLOW" => s_GAME_NOT_A_BLOW(v),
            "GAME_PLAYED" => s_GAME_PLAYED(v),
            "GAME_WORD_STOP" => s_GAME_WORD_STOP(v),
            "GRIP_GAME" => s_GRIP_GAME(v),
            "GRIP_GAME_FENCE" => s_GRIP_GAME_FENCE(v),
            "GRIP_NOT_AN_ASSAULT" => s_GRIP_NOT_AN_ASSAULT(v),
            "GRIP_NOT_AN_ASSAULT_STR" => s_GRIP_NOT_AN_ASSAULT_STR(v),
            "HAS_ME_SUBJECT" => s_HAS_ME_SUBJECT(v),
            "HIT_AS_A_CHILD" => s_HIT_AS_A_CHILD(v),
            "HIT_BY_A_VEHICLE" => s_HIT_BY_A_VEHICLE(v),
            "HIT_THING_NOT_PERSON" => s_HIT_THING_NOT_PERSON(v),
            "I_DID_NOT_HIT" => s_I_DID_NOT_HIT(v),
            "I_HIT_SOMEONE" => s_I_HIT_SOMEONE(v),
            "MONEY_NAMED" => s_MONEY_NAMED(v),
            "MY_GROWN_UP" => s_MY_GROWN_UP(v),
            "NOT_A_FEAR" => s_NOT_A_FEAR(v),
            "OTHERS" => s_OTHERS(v),
            "PARTNER_CLOSE" => s_PARTNER_CLOSE(v),
            "PEOPLE" => s_PEOPLE(v),
            "PERSON" => s_PERSON(v),
            "PLAY_FACE_REPLIES" => s_PLAY_FACE_REPLIES(v),
            "PLAY_NAMED" => s_PLAY_NAMED(v),
            "PURSUIT" => s_PURSUIT(v),
            "PURSUIT_FEAR" => s_PURSUIT_FEAR(v),
            "PURSUIT_LOUD" => s_PURSUIT_LOUD(v),
            "PURSUIT_OUTSIDE" => s_PURSUIT_OUTSIDE(v),
            "PURSUIT_PLACE_HOME" => s_PURSUIT_PLACE_HOME(v),
            "PURSUIT_PLACE_WORK" => s_PURSUIT_PLACE_WORK(v),
            "PURSUIT_SUBJECT" => s_PURSUIT_SUBJECT(v),
            "PURSUIT_SUBJECT_EX" => s_PURSUIT_SUBJECT_EX(v),
            "REMORSE_FOR_THE_ACT" => s_REMORSE_FOR_THE_ACT(v),
            "SELF_HARM_NEEDS_PERSON" => s_SELF_HARM_NEEDS_PERSON(v),
            "SELF_OWN_DYING" => s_SELF_OWN_DYING(v),
            "SOFT_FACE_PUSH" => s_SOFT_FACE_PUSH(v),
            "SOFT_FACE_STOP" => s_SOFT_FACE_STOP(v),
            "SOFT_HIT_THING" => s_SOFT_HIT_THING(v),
            "SOFT_HIT_VERB" => s_SOFT_HIT_VERB(v),
            "SOMEONE_HOLDS_THE_MONEY" => s_SOMEONE_HOLDS_THE_MONEY(v),
            "THE_BLOW_ON_A_CHILD" => s_THE_BLOW_ON_A_CHILD(v),
            "THREAT_COERCIVE" => s_THREAT_COERCIVE(v),
            "THREAT_SUBJECT" => s_THREAT_SUBJECT(v),
            "THROWN_AT_ME" => s_THROWN_AT_ME(v),
            "_NOT_A_THREAT" => s__NOT_A_THREAT(v),
            "_REFUSED_EXIT" => s__REFUSED_EXIT(v),
            "_THE_BLOW" => s__THE_BLOW(v),
            "_THE_HAND" => s__THE_HAND(v),
            "_THE_ONE_HIT" => s__THE_ONE_HIT(v),
            "__all__" => s___all__(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.safety_phrases", module_obj);
    pyrt::xrefs::reg_fn("modules.safety_phrases", "_both_here", py_func("_both_here", |args: &[Py]| _both_here(args)));
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE", g_ABUSE, s_ABUSE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSER", g_ABUSER, s_ABUSER);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_DISCLOSURE", g_ABUSE_DISCLOSURE, s_ABUSE_DISCLOSURE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_DISCLOSURE_FINE", g_ABUSE_DISCLOSURE_FINE, s_ABUSE_DISCLOSURE_FINE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_FINE", g_ABUSE_FINE, s_ABUSE_FINE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_HEAD_INTO", g_ABUSE_HEAD_INTO, s_ABUSE_HEAD_INTO);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_HIT_ME", g_ABUSE_HIT_ME, s_ABUSE_HIT_ME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_MEMORY", g_ABUSE_MEMORY, s_ABUSE_MEMORY);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_NOW", g_ABUSE_NOW, s_ABUSE_NOW);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_PURSUIT_HARD", g_ABUSE_PURSUIT_HARD, s_ABUSE_PURSUIT_HARD);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERB", g_ABUSE_VERB, s_ABUSE_VERB);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL", g_ABUSE_VERBAL, s_ABUSE_VERBAL);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL_FINE", g_ABUSE_VERBAL_FINE, s_ABUSE_VERBAL_FINE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL_LABEL", g_ABUSE_VERBAL_LABEL, s_ABUSE_VERBAL_LABEL);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL_MAD", g_ABUSE_VERBAL_MAD, s_ABUSE_VERBAL_MAD);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL_PERSON", g_ABUSE_VERBAL_PERSON, s_ABUSE_VERBAL_PERSON);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERBAL_SUBJECT", g_ABUSE_VERBAL_SUBJECT, s_ABUSE_VERBAL_SUBJECT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ABUSE_VERB_PART", g_ABUSE_VERB_PART, s_ABUSE_VERB_PART);
    pyrt::xrefs::reg_access("modules.safety_phrases", "ASK_AFTER_ME", g_ASK_AFTER_ME, s_ASK_AFTER_ME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "BLOW_THINGS", g_BLOW_THINGS, s_BLOW_THINGS);
    pyrt::xrefs::reg_access("modules.safety_phrases", "BY_NOT_A_THING", g_BY_NOT_A_THING, s_BY_NOT_A_THING);
    pyrt::xrefs::reg_access("modules.safety_phrases", "CHILD_AGGRESSOR", g_CHILD_AGGRESSOR, s_CHILD_AGGRESSOR);
    pyrt::xrefs::reg_access("modules.safety_phrases", "CHILD_HIT_NOW", g_CHILD_HIT_NOW, s_CHILD_HIT_NOW);
    pyrt::xrefs::reg_access("modules.safety_phrases", "CLOSE", g_CLOSE, s_CLOSE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "DYING_OTHER", g_DYING_OTHER, s_DYING_OTHER);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GAME_AFTER_ME", g_GAME_AFTER_ME, s_GAME_AFTER_ME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GAME_NOT_A_BLOW", g_GAME_NOT_A_BLOW, s_GAME_NOT_A_BLOW);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GAME_PLAYED", g_GAME_PLAYED, s_GAME_PLAYED);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GAME_WORD_STOP", g_GAME_WORD_STOP, s_GAME_WORD_STOP);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GRIP_GAME", g_GRIP_GAME, s_GRIP_GAME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GRIP_GAME_FENCE", g_GRIP_GAME_FENCE, s_GRIP_GAME_FENCE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GRIP_NOT_AN_ASSAULT", g_GRIP_NOT_AN_ASSAULT, s_GRIP_NOT_AN_ASSAULT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "GRIP_NOT_AN_ASSAULT_STR", g_GRIP_NOT_AN_ASSAULT_STR, s_GRIP_NOT_AN_ASSAULT_STR);
    pyrt::xrefs::reg_access("modules.safety_phrases", "HAS_ME_SUBJECT", g_HAS_ME_SUBJECT, s_HAS_ME_SUBJECT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "HIT_AS_A_CHILD", g_HIT_AS_A_CHILD, s_HIT_AS_A_CHILD);
    pyrt::xrefs::reg_access("modules.safety_phrases", "HIT_BY_A_VEHICLE", g_HIT_BY_A_VEHICLE, s_HIT_BY_A_VEHICLE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "HIT_THING_NOT_PERSON", g_HIT_THING_NOT_PERSON, s_HIT_THING_NOT_PERSON);
    pyrt::xrefs::reg_access("modules.safety_phrases", "I_DID_NOT_HIT", g_I_DID_NOT_HIT, s_I_DID_NOT_HIT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "I_HIT_SOMEONE", g_I_HIT_SOMEONE, s_I_HIT_SOMEONE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "MONEY_NAMED", g_MONEY_NAMED, s_MONEY_NAMED);
    pyrt::xrefs::reg_access("modules.safety_phrases", "MY_GROWN_UP", g_MY_GROWN_UP, s_MY_GROWN_UP);
    pyrt::xrefs::reg_access("modules.safety_phrases", "NOT_A_FEAR", g_NOT_A_FEAR, s_NOT_A_FEAR);
    pyrt::xrefs::reg_access("modules.safety_phrases", "OTHERS", g_OTHERS, s_OTHERS);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PARTNER_CLOSE", g_PARTNER_CLOSE, s_PARTNER_CLOSE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PEOPLE", g_PEOPLE, s_PEOPLE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PERSON", g_PERSON, s_PERSON);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PLAY_FACE_REPLIES", g_PLAY_FACE_REPLIES, s_PLAY_FACE_REPLIES);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PLAY_NAMED", g_PLAY_NAMED, s_PLAY_NAMED);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT", g_PURSUIT, s_PURSUIT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_FEAR", g_PURSUIT_FEAR, s_PURSUIT_FEAR);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_LOUD", g_PURSUIT_LOUD, s_PURSUIT_LOUD);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_OUTSIDE", g_PURSUIT_OUTSIDE, s_PURSUIT_OUTSIDE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_PLACE_HOME", g_PURSUIT_PLACE_HOME, s_PURSUIT_PLACE_HOME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_PLACE_WORK", g_PURSUIT_PLACE_WORK, s_PURSUIT_PLACE_WORK);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_SUBJECT", g_PURSUIT_SUBJECT, s_PURSUIT_SUBJECT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "PURSUIT_SUBJECT_EX", g_PURSUIT_SUBJECT_EX, s_PURSUIT_SUBJECT_EX);
    pyrt::xrefs::reg_access("modules.safety_phrases", "REMORSE_FOR_THE_ACT", g_REMORSE_FOR_THE_ACT, s_REMORSE_FOR_THE_ACT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SELF_HARM_NEEDS_PERSON", g_SELF_HARM_NEEDS_PERSON, s_SELF_HARM_NEEDS_PERSON);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SELF_OWN_DYING", g_SELF_OWN_DYING, s_SELF_OWN_DYING);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SOFT_FACE_PUSH", g_SOFT_FACE_PUSH, s_SOFT_FACE_PUSH);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SOFT_FACE_STOP", g_SOFT_FACE_STOP, s_SOFT_FACE_STOP);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SOFT_HIT_THING", g_SOFT_HIT_THING, s_SOFT_HIT_THING);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SOFT_HIT_VERB", g_SOFT_HIT_VERB, s_SOFT_HIT_VERB);
    pyrt::xrefs::reg_access("modules.safety_phrases", "SOMEONE_HOLDS_THE_MONEY", g_SOMEONE_HOLDS_THE_MONEY, s_SOMEONE_HOLDS_THE_MONEY);
    pyrt::xrefs::reg_access("modules.safety_phrases", "THE_BLOW_ON_A_CHILD", g_THE_BLOW_ON_A_CHILD, s_THE_BLOW_ON_A_CHILD);
    pyrt::xrefs::reg_access("modules.safety_phrases", "THREAT_COERCIVE", g_THREAT_COERCIVE, s_THREAT_COERCIVE);
    pyrt::xrefs::reg_access("modules.safety_phrases", "THREAT_SUBJECT", g_THREAT_SUBJECT, s_THREAT_SUBJECT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "THROWN_AT_ME", g_THROWN_AT_ME, s_THROWN_AT_ME);
    pyrt::xrefs::reg_access("modules.safety_phrases", "_NOT_A_THREAT", g__NOT_A_THREAT, s__NOT_A_THREAT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "_REFUSED_EXIT", g__REFUSED_EXIT, s__REFUSED_EXIT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "_THE_BLOW", g__THE_BLOW, s__THE_BLOW);
    pyrt::xrefs::reg_access("modules.safety_phrases", "_THE_HAND", g__THE_HAND, s__THE_HAND);
    pyrt::xrefs::reg_access("modules.safety_phrases", "_THE_ONE_HIT", g__THE_ONE_HIT, s__THE_ONE_HIT);
    pyrt::xrefs::reg_access("modules.safety_phrases", "__all__", g___all__, s___all__);
}

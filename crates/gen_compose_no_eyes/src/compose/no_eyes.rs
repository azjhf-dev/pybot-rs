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


thread_local! { static G_ABOUT_WHAT_IT_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_EAR_HEARS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_EYE_READS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_THE_BOT_TO_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_AN_EMAIL_ADDRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ASKING_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_CONTACT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_HANDOVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_PHONE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BEEP_OR_AN_ALARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BEEP_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DESCRIPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FORWARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAND_DOES_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIGHT_OUT_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIGHT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LISTEN_OFFER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOOK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOOK_ASK_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MACHINE_OR_A_NEIGHBOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MARK_ON_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MORE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_ADDRESSEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOISE_ABOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOISE_IN_THE_PIPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOISE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOISE_OUTSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOISE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POWER_CUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECORDING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SMELL_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SOUNDS_AGREEING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THE_STREET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_IN_THE_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_URL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WITNESSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BACK_AS_A_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_HEAR_A_BEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_SEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_SEE_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_SMELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESCRIBED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESCRIBED_CYCLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESCRIBED_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_TO_PLAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDED_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDED_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLDS_THE_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHTS_ARE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESSAGE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_HAND_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_READS_HANDED_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEEING_FAMILIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THAT_OPENS_A_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_MESSAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLACEHOLDERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRITES_A_MESSAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DESCRIBED_GOES_QUIET_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOISE_OR_SMELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROOM_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.no_eyes: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_WHAT_IT_KEEPS() -> Py { ensure_init(); G_ABOUT_WHAT_IT_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_WHAT_IT_KEEPS(v: Py) { ensure_init(); G_ABOUT_WHAT_IT_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_EAR_HEARS_IT() -> Py { ensure_init(); G_AN_EAR_HEARS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EAR_HEARS_IT(v: Py) { ensure_init(); G_AN_EAR_HEARS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_EYE_READS_IT() -> Py { ensure_init(); G_AN_EYE_READS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EYE_READS_IT(v: Py) { ensure_init(); G_AN_EYE_READS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_THE_BOT_TO_ACT() -> Py { ensure_init(); G_ASKS_THE_BOT_TO_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_BOT_TO_ACT(v: Py) { ensure_init(); G_ASKS_THE_BOT_TO_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_AN_EMAIL_ADDRESS() -> Py { ensure_init(); G_A_AN_EMAIL_ADDRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AN_EMAIL_ADDRESS(v: Py) { ensure_init(); G_A_AN_EMAIL_ADDRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ASKING_FOR_IT() -> Py { ensure_init(); G_A_ASKING_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ASKING_FOR_IT(v: Py) { ensure_init(); G_A_ASKING_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_CONTACT_WORD() -> Py { ensure_init(); G_A_A_CONTACT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_CONTACT_WORD(v: Py) { ensure_init(); G_A_A_CONTACT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_HANDOVER() -> Py { ensure_init(); G_A_A_HANDOVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_HANDOVER(v: Py) { ensure_init(); G_A_A_HANDOVER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_LENGTH() -> Py { ensure_init(); G_A_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_LENGTH(v: Py) { ensure_init(); G_A_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_PHONE_NUMBER() -> Py { ensure_init(); G_A_A_PHONE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_PHONE_NUMBER(v: Py) { ensure_init(); G_A_A_PHONE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BEEP_OR_AN_ALARM() -> Py { ensure_init(); G_A_BEEP_OR_AN_ALARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BEEP_OR_AN_ALARM(v: Py) { ensure_init(); G_A_BEEP_OR_AN_ALARM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BEEP_WORD() -> Py { ensure_init(); G_A_BEEP_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BEEP_WORD(v: Py) { ensure_init(); G_A_BEEP_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DESCRIPTION() -> Py { ensure_init(); G_A_DESCRIPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DESCRIPTION(v: Py) { ensure_init(); G_A_DESCRIPTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FORWARD() -> Py { ensure_init(); G_A_FORWARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FORWARD(v: Py) { ensure_init(); G_A_FORWARD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAND_DOES_IT() -> Py { ensure_init(); G_A_HAND_DOES_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAND_DOES_IT(v: Py) { ensure_init(); G_A_HAND_DOES_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIGHT_OUT_THERE() -> Py { ensure_init(); G_A_LIGHT_OUT_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIGHT_OUT_THERE(v: Py) { ensure_init(); G_A_LIGHT_OUT_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIGHT_WORD() -> Py { ensure_init(); G_A_LIGHT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIGHT_WORD(v: Py) { ensure_init(); G_A_LIGHT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LISTEN_OFFER() -> Py { ensure_init(); G_A_LISTEN_OFFER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LISTEN_OFFER(v: Py) { ensure_init(); G_A_LISTEN_OFFER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOOK_ASK() -> Py { ensure_init(); G_A_LOOK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOOK_ASK(v: Py) { ensure_init(); G_A_LOOK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOOK_ASK_IT() -> Py { ensure_init(); G_A_LOOK_ASK_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOOK_ASK_IT(v: Py) { ensure_init(); G_A_LOOK_ASK_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MACHINE_OR_A_NEIGHBOUR() -> Py { ensure_init(); G_A_MACHINE_OR_A_NEIGHBOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_OR_A_NEIGHBOUR(v: Py) { ensure_init(); G_A_MACHINE_OR_A_NEIGHBOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MARK_ON_THEM() -> Py { ensure_init(); G_A_MARK_ON_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MARK_ON_THEM(v: Py) { ensure_init(); G_A_MARK_ON_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MORE_BODY() -> Py { ensure_init(); G_A_MORE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MORE_BODY(v: Py) { ensure_init(); G_A_MORE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_ADDRESSEE() -> Py { ensure_init(); G_A_NAMED_ADDRESSEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_ADDRESSEE(v: Py) { ensure_init(); G_A_NAMED_ADDRESSEE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_ASK() -> Py { ensure_init(); G_A_NAME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_ASK(v: Py) { ensure_init(); G_A_NAME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOISE_ABOVE() -> Py { ensure_init(); G_A_NOISE_ABOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOISE_ABOVE(v: Py) { ensure_init(); G_A_NOISE_ABOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOISE_IN_THE_PIPES() -> Py { ensure_init(); G_A_NOISE_IN_THE_PIPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOISE_IN_THE_PIPES(v: Py) { ensure_init(); G_A_NOISE_IN_THE_PIPES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOISE_NAMED() -> Py { ensure_init(); G_A_NOISE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOISE_NAMED(v: Py) { ensure_init(); G_A_NOISE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOISE_OUTSIDE() -> Py { ensure_init(); G_A_NOISE_OUTSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOISE_OUTSIDE(v: Py) { ensure_init(); G_A_NOISE_OUTSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOISE_VERB() -> Py { ensure_init(); G_A_NOISE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOISE_VERB(v: Py) { ensure_init(); G_A_NOISE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POWER_CUT() -> Py { ensure_init(); G_A_POWER_CUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POWER_CUT(v: Py) { ensure_init(); G_A_POWER_CUT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECORDING_WORD() -> Py { ensure_init(); G_A_RECORDING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECORDING_WORD(v: Py) { ensure_init(); G_A_RECORDING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SMELL_WORD() -> Py { ensure_init(); G_A_SMELL_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SMELL_WORD(v: Py) { ensure_init(); G_A_SMELL_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SOUNDS_AGREEING() -> Py { ensure_init(); G_A_SOUNDS_AGREEING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SOUNDS_AGREEING(v: Py) { ensure_init(); G_A_SOUNDS_AGREEING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THE_STREET() -> Py { ensure_init(); G_A_THE_STREET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THE_STREET(v: Py) { ensure_init(); G_A_THE_STREET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_IN_THE_ROOM() -> Py { ensure_init(); G_A_THING_IN_THE_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_IN_THE_ROOM(v: Py) { ensure_init(); G_A_THING_IN_THE_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED() -> Py { ensure_init(); G_A_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED(v: Py) { ensure_init(); G_A_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_URL() -> Py { ensure_init(); G_A_URL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_URL(v: Py) { ensure_init(); G_A_URL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WITNESSED() -> Py { ensure_init(); G_A_WITNESSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WITNESSED(v: Py) { ensure_init(); G_A_WITNESSED.with(|c| *c.borrow_mut() = v); }
pub fn g_BACK_AS_A_BODY() -> Py { ensure_init(); G_BACK_AS_A_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BACK_AS_A_BODY(v: Py) { ensure_init(); G_BACK_AS_A_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_HEAR_A_BEEP() -> Py { ensure_init(); G_CANNOT_HEAR_A_BEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_HEAR_A_BEEP(v: Py) { ensure_init(); G_CANNOT_HEAR_A_BEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_SEE() -> Py { ensure_init(); G_CANNOT_SEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_SEE(v: Py) { ensure_init(); G_CANNOT_SEE.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_SEE_YOU() -> Py { ensure_init(); G_CANNOT_SEE_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_SEE_YOU(v: Py) { ensure_init(); G_CANNOT_SEE_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_SMELL() -> Py { ensure_init(); G_CANNOT_SMELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_SMELL(v: Py) { ensure_init(); G_CANNOT_SMELL.with(|c| *c.borrow_mut() = v); }
pub fn g_DESCRIBED() -> Py { ensure_init(); G_DESCRIBED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESCRIBED(v: Py) { ensure_init(); G_DESCRIBED.with(|c| *c.borrow_mut() = v); }
pub fn g_DESCRIBED_CYCLE() -> Py { ensure_init(); G_DESCRIBED_CYCLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESCRIBED_CYCLE(v: Py) { ensure_init(); G_DESCRIBED_CYCLE.with(|c| *c.borrow_mut() = v); }
pub fn g_DESCRIBED_ROOM() -> Py { ensure_init(); G_DESCRIBED_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESCRIBED_ROOM(v: Py) { ensure_init(); G_DESCRIBED_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_TO_PLAY() -> Py { ensure_init(); G_GAME_TO_PLAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_TO_PLAY(v: Py) { ensure_init(); G_GAME_TO_PLAY.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_BACK() -> Py { ensure_init(); G_GOING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_BACK(v: Py) { ensure_init(); G_GOING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDED_FAMILIES() -> Py { ensure_init(); G_HANDED_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDED_FAMILIES(v: Py) { ensure_init(); G_HANDED_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDED_OVER() -> Py { ensure_init(); G_HANDED_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDED_OVER(v: Py) { ensure_init(); G_HANDED_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLDS_THE_TEXT() -> Py { ensure_init(); G_HOLDS_THE_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLDS_THE_TEXT(v: Py) { ensure_init(); G_HOLDS_THE_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHTS_ARE_OUT() -> Py { ensure_init(); G_LIGHTS_ARE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHTS_ARE_OUT(v: Py) { ensure_init(); G_LIGHTS_ARE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_MESSAGE_WORD() -> Py { ensure_init(); G_MESSAGE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESSAGE_WORD(v: Py) { ensure_init(); G_MESSAGE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_BACK() -> Py { ensure_init(); G_MY_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_BACK(v: Py) { ensure_init(); G_MY_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_HAND_VERB() -> Py { ensure_init(); G_NO_HAND_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_HAND_VERB(v: Py) { ensure_init(); G_NO_HAND_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_READS_HANDED_TEXT() -> Py { ensure_init(); G_READS_HANDED_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_READS_HANDED_TEXT(v: Py) { ensure_init(); G_READS_HANDED_TEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_SEEING_FAMILIES() -> Py { ensure_init(); G_SEEING_FAMILIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEEING_FAMILIES(v: Py) { ensure_init(); G_SEEING_FAMILIES.with(|c| *c.borrow_mut() = v); }
pub fn g_THAT_OPENS_A_SENTENCE() -> Py { ensure_init(); G_THAT_OPENS_A_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THAT_OPENS_A_SENTENCE(v: Py) { ensure_init(); G_THAT_OPENS_A_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_MESSAGE() -> Py { ensure_init(); G_THEIR_OWN_MESSAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_MESSAGE(v: Py) { ensure_init(); G_THEIR_OWN_MESSAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLACEHOLDERS() -> Py { ensure_init(); G_THE_PLACEHOLDERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLACEHOLDERS(v: Py) { ensure_init(); G_THE_PLACEHOLDERS.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_A_PERSON() -> Py { ensure_init(); G_TO_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_A_PERSON(v: Py) { ensure_init(); G_TO_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_WRITES_A_MESSAGE() -> Py { ensure_init(); G_WRITES_A_MESSAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRITES_A_MESSAGE(v: Py) { ensure_init(); G_WRITES_A_MESSAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__DESCRIBED_GOES_QUIET_LINES() -> Py { ensure_init(); G__DESCRIBED_GOES_QUIET_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DESCRIBED_GOES_QUIET_LINES(v: Py) { ensure_init(); G__DESCRIBED_GOES_QUIET_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOISE_OR_SMELL() -> Py { ensure_init(); G__NOISE_OR_SMELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOISE_OR_SMELL(v: Py) { ensure_init(); G__NOISE_OR_SMELL.with(|c| *c.borrow_mut() = v); }
pub fn g__ROOM_ASKS() -> Py { ensure_init(); G__ROOM_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROOM_ASKS(v: Py) { ensure_init(); G__ROOM_ASKS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reader for a thing of the person's own that is on their side of the screen.\n\nTwo shapes, one reading. A line can ask the bot to open, read, look at, fill in, print or send\nsomething of theirs, or it can simply hand the thing over (\"here's a screenshot\", \"the link is\nwww...\"). Either way nothing here can see it: nothing comes through an attachment, nothing can be\nopened, and there are no hands to click with. On main the first shape got the search hedge, \"a\nquick search online will get you a solid answer\", which sends somebody to go and google their own\ninbox, and the second got \"Got it. Is there anything you'd like a hand with?\" as though the\npicture had arrived.\n\nThe pool that answers is read from the object, never invented: a link is pasted, a photo or a\nscreen is described, an inbox is copied from, a form is read out, a click or a print is turned\ninto steps, and a bare \"this\" is the text on their screen. Nothing here is a fact about the world.\n\nBoth readings are narrow. The ask needs the bot's own act (\"can you ...\", or an imperative with\n\"for me\") and a verb only an eye or a hand can do; the handed line needs a handing word and the\nfamily named, or a bare link. Both stand down for what the bot can do (\"can you write my cv for\nme\", \"can you type up my notes\"), for a line that already holds the text (\"i pasted the text\nbelow\"), for what the bot keeps of the person (\"can you see my chats with you\"), for an address or\na number they are giving the bot, for a message or a text (this chat, not their mail), and for any\nbody part, symptom or doctor's word: a rash is not a file, and the pharmacist line is the answer.\n\nThis lives in its own file because `bot/compose/fallback.py` is at the world's size limit; the\nfallback imports the two readers and keeps the choosing of the line.\n");
    fn __data1() -> Py { py_tuple(vec![py_str("link"), py_str("links"), py_str("url"), py_str("urls"), py_str("website"), py_str("websites"), py_str("site"), py_str("sites"), py_str("webpage"), py_str("web page"), py_str("webpages"), py_str("page"), py_str("pages"), py_str("article"), py_str("articles"), py_str("tab"), py_str("tabs"), py_str("browser")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("screen"), py_str("screens"), py_str("camera"), py_str("webcam"), py_str("photo"), py_str("photos"), py_str("picture"), py_str("pictures"), py_str("image"), py_str("images"), py_str("screenshot"), py_str("screenshots"), py_str("video"), py_str("videos"), py_str("clip"), py_str("clips"), py_str("notification"), py_str("notifications")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("voice note"), py_str("voice notes"), py_str("voice memo"), py_str("voice memos"), py_str("voice message"), py_str("voice messages"), py_str("voice recording"), py_str("voice recordings"), py_str("voice clip"), py_str("recording"), py_str("recordings"), py_str("audio"), py_str("audio clip"), py_str("audio file"), py_str("audio message"), py_str("sound file"), py_str("sound clip"), py_str("memo"), py_str("memos"), py_str("voicemail"), py_str("voicemails"), py_str("podcast"), py_str("podcasts")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("email"), py_str("emails"), py_str("e-mail"), py_str("e-mails"), py_str("mail"), py_str("inbox"), py_str("message"), py_str("messages"), py_str("text"), py_str("texts"), py_str("voicemail"), py_str("voicemails"), py_str("letter"), py_str("letters"), py_str("post"), py_str("folder"), py_str("folders"), py_str("spam")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("file"), py_str("files"), py_str("document"), py_str("documents"), py_str("doc"), py_str("docs"), py_str("pdf"), py_str("pdfs"), py_str("spreadsheet"), py_str("spreadsheets"), py_str("essay"), py_str("essays"), py_str("cv"), py_str("homework"), py_str("statement"), py_str("invoice"), py_str("receipt"), py_str("contract"), py_str("attachment"), py_str("attachments")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.SEEING_FAMILIES"); }
    s_SEEING_FAMILIES(py_dict(vec![(py_str("link"), __data1()), (py_str("photo"), __data2()), (py_str("voice"), __data3()), (py_str("inbox"), __data4()), (py_str("form"), py_tuple(vec![py_str("form"), py_str("forms"), py_str("paperwork"), py_str("questionnaire"), py_str("tax return"), py_str("tax returns"), py_str("return form")])), (py_str("file"), __data5())]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_HAND_DOES_IT"); }
    s_A_HAND_DOES_IT(stdlib_call("re", "compile", &[py_str("\\b(?:click|clicking|tap|tapping|press|pressing|scroll|scrolling|print|printing|scan|scanning|download|downloading|upload|uploading|send|sending|forward|forwarding|save|saving|delete|deleting|install|installing|access|accessing|submit|submitting|log\\s+in|sign\\s+in|screenshot|screenshotting|take\\s+(?:a\\s+)?(?:photo|picture|screenshot)|type\\s+(?:in|into)|sign\\s+(?:it|this|that|these|those|them|for\\s+(?:me|us))|copy\\s+(?:the|it|that|this|out))")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.AN_EYE_READS_IT"); }
    s_AN_EYE_READS_IT(stdlib_call("re", "compile", &[py_str("\\b(?:open|opening|read|reading|see|seeing|view|viewing|watch|watching|find|finding|look\\s+(?:at|through|over|in|into)|check|checking|fill(?:ing)?\\s+(?:in|out)|fill|filling|complete|completing|translate|translating|read\\s+(?:out|it\\s+out)?\\s+what\\s+(?:it|that|this)\\s+says|what\\s+(?:my|the)\\s+\\S+\\s+says|update|updating|go\\s+through|get\\s+into)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.AN_EAR_HEARS_IT"); }
    s_AN_EAR_HEARS_IT(stdlib_call("re", "compile", &[py_str("\\b(?:listen|listening|hear|hearing|play|playing|playback|transcribe|transcribing|decipher|make\\s+out)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.NO_HAND_VERB"); }
    s_NO_HAND_VERB(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&gattr(&g_A_HAND_DOES_IT(), "pattern"), &py_str("|")), &gattr(&g_AN_EYE_READS_IT(), "pattern")), &py_str("|")), &gattr(&g_AN_EAR_HEARS_IT(), "pattern"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.READS_HANDED_TEXT"); }
    s_READS_HANDED_TEXT(stdlib_call("re", "compile", &[py_str("\\b(?:open|opening|read|reading|see|seeing|view|viewing|watch|watching|look\\s+(?:at|through|over)|translate|translating|what\\s+(?:it|this|that)\\s+says)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.ASKS_THE_BOT_TO_ACT"); }
    s_ASKS_THE_BOT_TO_ACT(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will|are|do)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*|\\bhelp\\s+(?:me|us)\\b\\s+(?:\\S+\\s+){0,3}\\S|\\b(?:have\\s+a\\s+(?:listen|look|read|quick\\s+look)|take\\s+a\\s+(?:listen|look|read))\\b|^\\s*(?:please\\s+|just\\s+)?(?:open|print|read|see|check|look\\s+at|fill\\s+in|fill\\s+out|scan|download|upload|click|tap|press|scroll)\\b[^?]*\\b(?:for\\s+me|please)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.HOLDS_THE_TEXT"); }
    s_HOLDS_THE_TEXT(stdlib_call("re", "compile", &[py_str(":\\s*\\S+(?:\\s+\\S+){5,}")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_dict(vec![(py_str("link"), py_tuple(vec![py_str("I can't open links, sorry - nothing on my side can browse. Tell me what it says, or what you want from it, and I'll help you with that."), py_str("No link opening from here, sorry. Paste the bit that matters into the chat and I'll work through it with you."), py_str("I can't follow a link, sorry. Copy the words you need and send them here - I'll take it from there.")])), (py_str("photo"), py_tuple(vec![py_str("I can't see your screen or your photos, sorry - I only get the words you type. Describe what's there, or type out the part that matters, and I'll help."), py_str("Nothing on my side can look at a photo or a screen, sorry. Tell me what it shows and what you need to know about it."), py_str("I can't see pictures from here - no camera, no screen. Say what is in it and I'll tell you what I make of it.")])), (py_str("inbox"), py_tuple(vec![py_str("I can't get into your inbox or your messages, sorry - they're on your side. Copy the words that matter in here and I'll help you answer them."), py_str("Your email is not something I can see. Paste the part you need to deal with and I'll draft the reply."), py_str("I can't read your mail from here. Send me the gist of it and I'll help you work out what to say back.")])), (py_str("form"), py_tuple(vec![py_str("I can't fill in a form I can't see, sorry. Tell me what it asks, one line at a time, and I'll give you an answer for each."), py_str("I can't write on a form from here - no hands on my side. Read me the questions and I'll give you the words; you put them in.")])), (py_str("file"), py_tuple(vec![py_str("I can't open files, sorry - nothing here can see or send them. Paste the text in and I'll go through it with you."), py_str("Nothing here can open a file, sorry. Type in the part you want help with and I'll take it from there."), py_str("I can't see a file from here, sorry - I only get what you type. Paste a bit of it in and we'll work on that.")])), (py_str("text"), py_tuple(vec![py_str("I can't see the page you're looking at, sorry - I only get what you type. Paste the bit you want help with and I'll go through it with you."), py_str("Nothing on my side can see the text on your screen, sorry. Type in the part that matters and I'll take it from there."), py_str("I only see the words you send me, sorry - not what is on your screen. Paste it in here and I'll work on that.")])), (py_str("voice"), py_tuple(vec![py_str("I can't hear a voice note or a recording, sorry - no ears on this end. Type the bit that matters in here and I'll work from that."), py_str("Nothing on my side can play or listen to audio. Write down what was said, even roughly, and I'll take it from there."), py_str("I can't listen to a recording from here - I only get what you type. Put the words in the chat and I'll help with them.")])), (py_str("sound"), py_tuple(vec![py_str("I can't hear anything from here, sorry - no ears and no speaker on this side. Tell me what it was like and where it came from - a knock, a bang, a hum, water, the pipes - and I'll say what usually makes that one."), py_str("Nothing on my side can listen or play, sorry. Say what you noticed: how loud, how often, which room. Most house sounds have two or three usual causes and the answer narrows it."), py_str("There is no microphone or speaker on this end, so I can't hear it. Describe it - a thud, a rattle, a drip, a hum - and where it comes from, and we can work it out from that.")])), (py_str("noise_roof"), py_tuple(vec![py_str("Overhead is usually one of three: an animal in the loft - birds, squirrels, sometimes a rat, and it scratches and moves fast; the cold water tank filling, which is a slow gurgle; or the pipes under it, which bang once when the heating starts. I can't hear anything from here - which of those is it closest to?"), py_str("A noise above the ceiling most often comes from the loft: an animal moving about, the tank refilling, or pipes expanding when the heating comes on. I have no ears on this side of the chat, so the sound itself is yours to place - scratchy and quick, a gurgle, or a single bang?")])), (py_str("noise_pipes"), py_tuple(vec![py_str("Banging or knocking from the pipes is usually air, or water hammer when a tap shuts quickly; a hum that comes with the heating is often the pump. None of that needs a plumber today - bleeding the radiators is the first thing to try. I can't hear it from here: does it happen when the heating starts, or when a tap is turned off?"), py_str("A rattle or a knock in the pipework is usually air in the system or a loose clip, and a hum that follows the heating is the pump. There is nothing on this end that can listen, so tell me when it starts - with the heating, or after a tap?")])), (py_str("noise_outside"), py_tuple(vec![py_str("From outside the house it is usually roadworks, a boiler flue or a fan next door, or an animal - and if it keeps you awake, the council's noise team wants the times and days written down, which is what gets anything done. I can't hear any of it from here: which side of the house is it on?"), py_str("Sound that comes in from outside is most often work on the road, a neighbour's extractor or boiler, or an animal - and a log of times and days is what the council acts on. Nothing here can listen, so where in the house do you hear it clearest?")])), (py_str("beep"), py_tuple(vec![py_str("A chirp every minute or so is nearly always a smoke alarm asking for its battery - twist it off its base and change it today rather than waiting. A carbon monoxide alarm is a different, louder note and means out of the house and onto 0800 111 999."), py_str("One beep at regular intervals is usually a smoke alarm's battery. If it is the carbon monoxide alarm instead - it says CO on it, and the sound is louder and harder to ignore - get everyone out and ring the gas emergency line on 0800 111 999.")])), (py_str("hands"), py_tuple(vec![py_str("I can't click, print or type on your device, sorry - I'm only words in the chat. Tell me what's on the screen and what you want to do and I'll walk you through the steps."), py_str("No hands on my side, sorry: nothing here can press, print or send anything. Say what you are trying to do and I'll give you the clicks in order.")])), (py_str("light"), py_tuple(vec![py_str("I can't see it from here, sorry - only the words you type reach me. A light that blinks on a machine is nearly always a code: count the flashes and note what it does before repeating, and the manual or the maker's app names that pattern. On a router, red usually means no internet at all, so check the provider's status page first."), py_str("Nothing on this end has eyes, sorry. Tell me what the light is on - a boiler, a router, a charger, the dashboard - and whether it flashes or stays on. A steady red on a boiler is usually a lockout and the manual's relight steps come first; a flashing pattern is a code the manual lists."), py_str("I can't see the room you're in. If a bulb went with a bang or a pop it is the bulb - switch it off at the wall before you change it, and if a new one does the same, the fitting or the driver is the fault. A light on a smoke alarm that blinks about once a minute is its battery, and a street light flickering or out is the council's to fix.")])), (py_str("lights_out"), py_tuple(vec![py_str("If several lights went at once, it is a tripped breaker or RCD in the consumer unit rather than the bulbs: switch the one that is down back on, and if it trips again leave it off and get an electrician rather than holding it up."), py_str("Several lights going together is a circuit, not a bulb - look in the consumer unit for the switch that is down and reset it once. If it goes again straight away, leave it off and ring an electrician; that is a fault they need to find.")])), (py_str("power_cut"), py_tuple(vec![py_str("If the whole street is dark it is the network operator's cut, not a fault in your house: check a neighbour's lights, then your operator's live outage map - 105 in the UK - which says what is off and when it is due back. Leave the breakers alone while it is theirs."), py_str("Nothing on this end can see the outage, sorry. Neighbours dark too means the network: ring 105 or open the operator's outage page and it will say how long. If you are the only one without power, then it is yours - start at the consumer unit.")])), (py_str("smell"), py_tuple(vec![py_str("I can't smell anything from here, sorry - only the words you type reach me. If it could be gas, get everyone out and ring the gas emergency line on 0800 111 999 before anything else; burning is usually the oven, a plug or a pan."), py_str("Nothing on this end has a nose, sorry. Tell me what it is like - gas, burning, damp, drains, or something dead under the floor - and I'll say what that one usually is and who deals with it."), py_str("I can't smell it from here. Gas gets everybody out of the house first and then the gas emergency line on 0800 111 999; anything else, say where it is strongest and whether it is damp, drains or burning, and I'll tell you if it is a landlord job.")]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.CANNOT_SEE"); }
    s_CANNOT_SEE(__data6());
    let _ = py_setitem(&g_CANNOT_SEE(), &py_str("contact"), py_tuple(vec![py_str("Nothing here can send, dial or save a contact, so an address or a number stays in this chat with me. Tell me what it is for - a message, a form, a complaint - and I'll write that out for you."), py_str("I can read it, but I can't email, ring or store it: nothing leaves this chat. Say who it is for and what you want said, and I'll put the words together."), py_str("No sending or calling from this end, sorry. If you say what you want done with that address or number, I'll help with the wording and you send it yourself.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.CANNOT_SMELL"); }
    s_CANNOT_SMELL(geti(&g_CANNOT_SEE(), &py_str("smell")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.CANNOT_HEAR_A_BEEP"); }
    s_CANNOT_HEAR_A_BEEP(geti(&g_CANNOT_SEE(), &py_str("beep")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.ABOUT_WHAT_IT_KEEPS"); }
    s_ABOUT_WHAT_IT_KEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:with|to)\\s+you\\b|\\bour\\s+(?:chat|chats|talk|talks|conversation|conversations)\\b|\\bremember\\s+(?:me|my|what|anything|things)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_URL"); }
    s_A_URL(stdlib_call("re", "compile", &[py_str("\\b(?:https?://|www\\.)\\S+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.HANDED_FAMILIES"); }
    s_HANDED_FAMILIES(py_dict(vec![(py_str("inbox"), py_tuple(vec![py_str("email"), py_str("emails"), py_str("mail"), py_str("inbox"), py_str("letter"), py_str("letters"), py_str("attachment"), py_str("attachments")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.HANDED_OVER"); }
    s_HANDED_OVER(stdlib_call("re", "compile", &[py_str("\\b(?:here'?s|here\\s+(?:is|are)|attached|attaching|see\\s+attached|sent\\s+you|sending\\s+you|shall\\s+send\\s+you|screenshot\\s+below|photo\\s+below|picture\\s+below|recorded\\b[^.]{0,30}\\bfor\\s+you|this\\s+is\\s+(?:a|my)\\s+(?:video|recording|voice\\s+note|voice\\s+memo|photo|picture|screenshot)\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_RECORDING_WORD"); }
    s_A_RECORDING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:record(?:ed|ing|s)?|voice\\s+note|voice\\s+memo|voice\\s+message|audio|podcast|memo|taped|tape)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_LISTEN_OFFER"); }
    s_A_LISTEN_OFFER(stdlib_call("re", "compile", &[py_str("\\b(?:want\\s+(?:me\\s+)?to\\s+(?:hear|listen)|(?:have|give)\\s+(?:it|this|that)\\s+a\\s+listen|(?:have|give)\\s+a\\s+listen|can\\s+you\\s+(?:listen|hear)\\s+(?:to\\s+)?(?:it|this|that)|listen\\s+to\\s+it|hear\\s+it|listen\\s+in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_A_LENGTH"); }
    s_A_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:about|roughly|around|like)\\s+\\d+\\s*(?:secs?|seconds?|mins?|minutes?)\\b|\\b\\d+\\s*(?:secs?|seconds?|mins?|minutes?)\\s+(?:long|of\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_AN_EMAIL_ADDRESS"); }
    s_A_AN_EMAIL_ADDRESS(stdlib_call("re", "compile", &[py_str("\\S+@\\S+\\.\\w+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_A_PHONE_NUMBER"); }
    s_A_A_PHONE_NUMBER(stdlib_call("re", "compile", &[py_str("(?:\\+44|\\(?0)\\d[\\d\\s\\-]{8,}\\d")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_A_CONTACT_WORD"); }
    s_A_A_CONTACT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:email|e-mail|phone|mobile|number|contact\\s+details?|postcode|address)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_A_HANDOVER"); }
    s_A_A_HANDOVER(stdlib_call("re", "compile", &[py_str("^\\s*(?:o+k?|okay|so|right|anyway|and|also)?[ ,]*(?:here'?s|heres|here\\s+(?:is|are)|this\\s+is|that'?s|my|our)\\b|\\bmy\\s+(?:email|e-?mail|phone|mobile|number|contact\\s+details?|postcode|address)\\s+is\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_ASKING_FOR_IT"); }
    s_A_ASKING_FOR_IT(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|whats|what's|how|where|which|who|when|why|is|are|do|does|did|can|could|should|would|any)\\b|\\?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.WRITES_A_MESSAGE"); }
    s_WRITES_A_MESSAGE(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|also\\s+)*(?:\\w+\\s+){0,3}(?:send|write|draft|type\\s+out|put|reply|forward|answer)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.MESSAGE_WORD"); }
    s_MESSAGE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:email|emails|e-mail|e-mails|text|texts|message|messages|note|notes|letter|letters|reply|replies|memo)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.TO_A_PERSON"); }
    s_TO_A_PERSON(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:me|us)\\b|\\bon\\s+my\\s+behalf\\b|\\bto\\s+(?:my|our|the|his|her|their|a)\\s+[a-z]+|\\b(?:someone|somebody|anyone)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_FORWARD"); }
    s_A_FORWARD(stdlib_call("re", "compile", &[py_str("\\bforward(?:ing|ed)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.THEIR_OWN_MESSAGE"); }
    s_THEIR_OWN_MESSAGE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+(?:email|e-mail|text|message|note|letter|reply|memo)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NAMED_ADDRESSEE"); }
    s_A_NAMED_ADDRESSEE(stdlib_call("re", "compile", &[py_str("\\b(?:to|for|with)\\s+(?:my|our|his|her|their|the|a)\\s+[a-z]+|\\b(?:to|for)\\s+(?:someone|somebody)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.BACK_AS_A_BODY"); }
    s_BACK_AS_A_BODY(stdlib_call("re", "compile", &[py_str("\\b(?:my|your|his|her|our|their|the|lower|upper|sore|bad)\\s+back\\b|\\bback\\s+(?:pain|ache|aches|hurts?|spasms?|injur\\w*|problem|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.GAME_TO_PLAY"); }
    s_GAME_TO_PLAY(stdlib_call("re", "compile", &[py_str("\\b(?:game|games|chess|match|quiz|level|levels|round|rounds|move|moves|puzzle|puzzles|cards|darts|board|boards|innings|hole|holes|shot|shots|score|scores|character|save|savegame)\\b|\\bplay\\s+[^.!?]{0,30}\\bwith\\s+me\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.CANNOT_SEE_YOU"); }
    s_CANNOT_SEE_YOU(py_tuple(vec![py_str("I can't see it from here, sorry - only what you type reaches me. Tell me where it is, how big it is and how long it has been there, and I'll say what to watch for. A pharmacist can take a look today if you want eyes on it."), py_str("Nothing on my side can look, sorry. Describe it - where it is, what it is like, whether it has changed - and I'll help you work out what to do next. A chemist will look at it for free, and today."), py_str("I can't see or feel anything from here, so I can't judge it for you. Say where it is and what it is like, and I'll tell you what I would do about it; if you want it looked at today, a pharmacist is the quickest way in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_LOOK_ASK"); }
    s_A_LOOK_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would)\\s+(?:you|u)\\s+(?:please\\s+)?(?:look\\s+(?:at|over)|(?:have|take)\\s+a\\s+look(?:\\s+at)?|see|check|examine|tell\\s+me\\s+(?:if|whether))\\b|^\\s*(?:does|do|is|are)\\s+(?:this|these|it|that)\\b|^\\s*tell\\s+me\\s+(?:if|whether)\\b|\\bwhat\\s+does\\s+(?:this|it|my)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_LOOK_ASK_IT"); }
    s_A_LOOK_ASK_IT(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would)\\s+(?:you|u)\\s+(?:please\\s+)?(?:look\\s+at\\s+it|check\\s+it|see\\s+it|have\\s+a\\s+look|take\\s+a\\s+look)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_str("tongue"), py_str("lip"), py_str("lips"), py_str("cheek"), py_str("cheeks"), py_str("chin"), py_str("forehead"), py_str("thumb"), py_str("thumbs"), py_str("thigh"), py_str("thighs"), py_str("shin"), py_str("shins"), py_str("calf"), py_str("calves"), py_str("heel"), py_str("heels"), py_str("palm"), py_str("palms"), py_str("scalp"), py_str("armpit"), py_str("armpits"), py_str("groin"), py_str("bum"), py_str("bottom"), py_str("eyelid"), py_str("eyelids"), py_str("nostril"), py_str("navel"), py_str("kidney"), py_str("kidneys"), py_str("lung"), py_str("lungs"), py_str("breast"), py_str("breasts"), py_str("testicle"), py_str("testicles"), py_str("penis")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_MORE_BODY"); }
    s_A_MORE_BODY(call_builtin("frozenset", &[__data7()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_MARK_ON_THEM"); }
    s_A_MARK_ON_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:rash|rashes|lump|lumps|mole|moles|spot|spots|cut|cuts|bruise|bruises|wound|wounds|blister|blisters|sore|sores|scar|scars|swelling|swollen|bite|bites|burn|burns|infection|infected|healing|stitches|wart|warts|abscess|ulcer|numb)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.GOING_BACK"); }
    s_GOING_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:go|goes|going|went|get|gets|getting|got|come|comes|coming|came|move|moves|moving|moved|be|been|being|bring|brings|bringing|brought|give|gives|giving|gave|pay|pays|paying|paid|put|puts|putting|hold|holds|holding|held|hand|hands|handing|send|sends|sending|sent|cut|cutting|run|running|ran|turn|turns|turning|turned|look|looks|looking|looked|walk|walks|walking|walked|drive|drives|driving|drove|fly|flies|flying|flew|sit|sits|sitting|sat|step|steps|stepping|stepped|write|writes|writing|wrote|ring|rings|ringing|rang|call|calls|calling|called|text|texts|texting|texted|reply|replies|replying|replied)\\s+(?:straight\\s+|right\\s+|all\\s+the\\s+way\\s+)?back\\b|\\bback\\s+(?:to|from|at|in|on|into|with|home)\\b|\\b(?:give|gives|giving|gave|pay|pays|paying|paid|put|puts|putting|hold|holds|holding|hand|hands|handing|send|sends|sending|sent|cut|cutting)\\s+(?:it|them|him|her|that|this|me|us|the\\s+\\w+)\\s+back\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.MY_BACK"); }
    s_MY_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:my|lower|upper|sore|aching|achy|stiff|bad|your|his|her|their)\\s+back\\b|\\bback\\s+(?:pain|ache|aches|spasm|injury|problem|problems|hurts|is\\s+sore)\\b|\\bon\\s+(?:my|his|her|their|your)\\s+back\\b|\\bback\\s+(?:of\\s+my|of\\s+his|of\\s+her)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes._NOISE_OR_SMELL"); }
    s__NOISE_OR_SMELL(py_str("(?:noise|sound|bang|crash|thud|thump|clatter|rattl|screech|whistl|hum|buzz|ring|beep|bleep|chirp|scream|shout|creak|groan|gurgl|drip|tick|knock|tap|pop|crack|dron|whirr|hiss|rumbl|racket|alarm|clank|clang|siren|whine|smell|stink|pong|whiff|stench|reek|odour|odor|fume)[a-z]*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_THING_IN_THE_ROOM"); }
    s_A_THING_IN_THE_ROOM(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^\\s*what(?:'s|s)?\\s+(?:was|is|were)\\s+(?:that|this)\\s+(?:\\w+\\s+){0,2}"), &g__NOISE_OR_SMELL()), &py_str("\\b|\\bwhat(?:'s|s)?\\s+(?:that|this)\\s+(?:\\w+\\s+){0,2}")), &g__NOISE_OR_SMELL()), &py_str("\\b|\\bwhat\\s+is\\s+(?:that|this)\\s+(?:\\w+\\s+){0,2}")), &g__NOISE_OR_SMELL()), &py_str("\\b|\\bwhat(?:'s|s)?\\s+(?:is\\s+|was\\s+)?(?:beeping|ringing|humming|buzzing|rattling|chirping|whistling|smelling|banging|clanking)\\b|\\bdid\\s+you\\s+(?:hear|smell)\\s+that\\b|\\b(?:is|was)\\s+that\\s+a\\s+\\w+\\s+(?:i|we)\\s+can\\s+hear\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_THING_NAMED"); }
    s_A_THING_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:banging|bang(?:ed|s)?|crash(?:ing|ed)?|thud\\w*|thump\\w*|clatter\\w*|rattl(?:e|es|ing|ed)|hum(?:ming|s)?|whirr\\w*|buzz(?:ing|es)?|scrap(?:ing|es)?|scratch(?:ing|es)?|squeal(?:ing|s)?|whistl(?:e|es|ing)|dripp(?:ing|s)?|clank(?:ing|s)?|creak(?:ing|s)?|click(?:ing|s)?|beep(?:ing|s)?|bleep(?:ing|s)?|chirp(?:ing|s)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NOISE_NAMED"); }
    s_A_NOISE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|this|that|some|strange|funny|odd|weird|loud|quiet|faint|low|high|humming|scraping|banging|rattling|dripping)\\s+(?:\\w+\\s+){0,2}(?:noise|noises|sound|sounds)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_SOUNDS_AGREEING"); }
    s_A_SOUNDS_AGREEING(stdlib_call("re", "compile", &[py_str("\\bsounds?\\s+(?:like|good|great|fine|ok|okay|lovely|nice|fun|perfect|ideal|brilliant|amazing|delicious|to\\s+me|about\\s+right)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_WITNESSED"); }
    s_A_WITNESSED(stdlib_call("re", "compile", &[py_str("\\b(?:there(?:'s|\\s+is|\\s+was|\\s+are|\\s+were)|i\\s+(?:can\\s+)?hea(?:r|rd)|we\\s+(?:can\\s+)?hea(?:r|rd)|can\\s+you\\s+hear|listen(?:ing)?|coming\\s+from|from\\s+the|from\\s+my|in\\s+the|in\\s+my|on\\s+the|through\\s+the|under\\s+the|behind\\s+the|above|below|upstairs|downstairs|outside|next\\s+door|in\\s+here|in\\s+there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NOISE_ABOVE"); }
    s_A_NOISE_ABOVE(stdlib_call("re", "compile", &[py_str("\\b(?:loft|attic|roof|roofspace|rafters?|ceiling|in\\s+the\\s+eaves)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NOISE_IN_THE_PIPES"); }
    s_A_NOISE_IN_THE_PIPES(stdlib_call("re", "compile", &[py_str("\\b(?:pipes?|pipework|radiators?|taps?|cistern|water\\s+tank|immersion|plumbing|stop\\s?cock|drains?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NOISE_OUTSIDE"); }
    s_A_NOISE_OUTSIDE(stdlib_call("re", "compile", &[py_str("\\b(?:garden|street|road|outside|alley|yard|next\\s+door(?:'s)?\\s+(?:garden|house)|drive|cul-de-sac|estate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NOISE_VERB"); }
    s_A_NOISE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:knock|bang|clunk|tick|creak|groan|hum|buzz|rattl|whistl|drip|gurgl|scratch|scuttl|rustl|scrap|rumbl|thump|hammer|whin|whirr|click|pop|crackl|squeak|squeal|drum|judder|shudder|vibrat|mov(?:e|ing)|bang(?:ing)?)\\w*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_MACHINE_OR_A_NEIGHBOUR"); }
    s_A_MACHINE_OR_A_NEIGHBOUR(stdlib_call("re", "compile", &[py_str("\\b(?:car|van|engine|boiler|washing\\s+machine|washer|dishwasher|dryer|tumble|fridge|freezer|motorbike|mot|bike|neighbou?rs?|next\\s+door|dog|cat|baby|child|children|kids?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_NAME_ASK"); }
    s_A_NAME_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:called|named|name\\s+for|word\\s+for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.THAT_OPENS_A_SENTENCE"); }
    s_THAT_OPENS_A_SENTENCE(stdlib_call("re", "compile", &[py_str("\\bthat\\s+(?:the|a|an|he|she|they|we|you|i|it|there|my|his|her|our|their)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_BEEP_WORD"); }
    s_A_BEEP_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:beep|beeps|beeping|chirp|chirps|chirping|bleep|bleeping)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_LIGHT_WORD"); }
    s_A_LIGHT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:flash|flashes|flashed|flashing|flicker|flickers|flickered|flickering|blink|blinks|blinked|blinking)\\b|\\b(?:red|green|orange|amber|blue|white)\\s+(?:light|led|lamp)\\b|\\b(?:light|lamp|bulb|bulbs|lights|led|screen)\\b[^.]{0,24}\\b(?:on|off|out|gone|went|flicker\\w*|flash\\w*|blink\\w*|bang|popped|blew)\\b|\\b(?:lights|light)\\s+(?:have\\s+|has\\s+)?(?:gone|went)\\s+off\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_LIGHT_OUT_THERE"); }
    s_A_LIGHT_OUT_THERE(stdlib_call("re", "compile", &[py_str("\\b(?:sky|star|stars|moon|plane|meteor|shooting|satellite|aurora|rainbow|sunset|sunrise|sun|laser|traffic|helicopter|drone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.LIGHTS_ARE_OUT"); }
    s_LIGHTS_ARE_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:lights?|power|electric(?:ity)?)\\b[^.]{0,20}\\b(?:went\\s+(?:off|out)|gone\\s+off|all\\s+off|tripped)\\b|\\b(?:power\\s+cut|blackout|fuse\\s+box|consumer\\s+unit)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_POWER_CUT"); }
    s_A_POWER_CUT(stdlib_call("re", "compile", &[py_str("\\b(?:power\\s+cut|power\\s+outage|outage|blackout|no\\s+power|power'?s?\\s+(?:gone|off|out)|power\\s+went\\s+(?:off|out)|electric(?:ity)?\\s+(?:is\\s+)?(?:off|gone|out|down)|supply\\s+(?:is\\s+)?(?:off|down))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_THE_STREET"); }
    s_A_THE_STREET(stdlib_call("re", "compile", &[py_str("\\b(?:street|road|avenue|estate|block|area|neighbou?rs?|next\\s+door|everyone|whole\\s+street)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_SMELL_WORD"); }
    s_A_SMELL_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:smell|smells|smelling|smelly|stink|stinks|stinking|pong|whiff|stench|reek|reeks|reeking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.THE_PLACEHOLDERS"); }
    s_THE_PLACEHOLDERS(py_tuple(vec![py_str("got it. is there anything you'd like a hand with"), py_str("i'm listening"), py_str("thanks for telling me"), py_str("noted:"), py_str("thanks for letting me know"), py_str("ok - noted")]));
    fn __data8() -> Py { py_tuple(vec![py_str("what it was like and where it came from"), py_str("what usually makes that one"), py_str("two or three usual causes"), py_str("where it comes from, and we can work it out"), py_str("what it is like - gas"), py_str("where it is strongest"), py_str("nothing on this end has a nose"), py_str("has no nose, sorry"), py_str("i can't smell anything from here"), py_str("i can't smell it from here"), py_str("only the words you type reach me"), py_str("no microphone or speaker on this end"), py_str("i can't hear anything from here"), py_str("is nearly always a smoke alarm"), py_str("usually a smoke alarm's battery"), py_str("usually a smoke alarm asking for its battery"), py_str("a light that blinks on a machine"), py_str("what the light is on"), py_str("the number of flashes"), py_str("several lights went at once"), py_str("several lights going together"), py_str("i can't see the room you're in"), py_str("count the flashes"), py_str("the provider's status page"), py_str("whether it flashes or stays on"), py_str("a light on a smoke alarm"), py_str("if a bulb went with a bang"), py_str("is the council's to fix"), py_str("tell me which room and how often"), py_str("if it is in one spot and repeats")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes._ROOM_ASKS"); }
    s__ROOM_ASKS(__data8());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes._DESCRIBED_GOES_QUIET_LINES"); }
    s__DESCRIBED_GOES_QUIET_LINES(py_tuple(vec![py_str("Something that stops for a bit and comes back is usually a warning, not a fault - most often a battery on its way out, which is the cheap thing to change first. If it comes back at the same interval as before, get it sorted today rather than sleeping through it."), py_str("That pattern is what a battery does on its way out: it chirps, pauses, then chirps again. Change it now, press the test button, and if it still does it, the unit itself is at the end of its ten years and wants replacing.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_BEEP_OR_AN_ALARM"); }
    s_A_BEEP_OR_AN_ALARM(stdlib_call("re", "compile", &[py_str("\\b(?:beep|beeps|beeping|bleep|bleeping|chirp|chirps|chirping|alarm|alarms|smoke|carbon\\s+monoxide|detector|battery|timer|clock|watch)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.DESCRIBED_CYCLE"); }
    s_DESCRIBED_CYCLE(py_tuple(vec![py_str("A noise that comes and goes on its own is more often a machine doing its job than a fault: the boiler firing and resting, a fridge, a pump, a fan. Note what is happening either side of the quiet - the heating clicking on, water running, the fridge door - and that names the appliance."), py_str("Something that starts and stops by itself is usually a cycle rather than a problem: heating and hot water run on one, and so does a fridge or a pump. Tell me what it does just before it starts and I will point at the one it is.")]));
    fn __data10() -> Py { py_tuple(vec![py_str("stops"), py_str("stop"), py_str("goes quiet"), py_str("comes back"), py_str("comes and goes"), py_str("every so often"), py_str("now and then"), py_str("intermittent"), py_str("not all the time"), py_str("on and off"), py_str("once in a while"), py_str("after a while"), py_str("then starts"), py_str("every minute"), py_str("every hour"), py_str("every few"), py_str("at intervals"), py_str("repeats"), py_str("repeating"), py_str("regular")]) }
    fn __data9() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("flash"), py_str("flashes"), py_str("flashed"), py_str("flashing"), py_str("blink"), py_str("blinks"), py_str("blinking"), py_str("a pattern"), py_str("in a pattern"), py_str("timed"), py_str("count")]), py_tuple(vec![py_str("A light that flashes in a pattern is a code, and the count is the code: the number of flashes before the pause is what the manual or the maker's app looks up. Write it down before it stops, and if the thing is under warranty ring the maker rather than opening it."), py_str("Then the flashes are the thing to count - two then a pause is a different fault from five. The manual's fault table or the maker's app takes that number, and a boiler that has locked out usually wants its relight steps first.")])]), py_tuple(vec![py_tuple(vec![py_str("stays on"), py_str("stayed on"), py_str("steady"), py_str("constant"), py_str("solid"), py_str("it is on"), py_str("its on"), py_str("always on")]), py_tuple(vec![py_str("A light that stays on rather than flashing is usually a warning rather than a fault code: the thing is running with something it does not like - a filter, a low level, a blocked drain - and the manual's first page of warnings names it."), py_str("Steady on means it has noticed something and is still working. The manual's warning list is the place to look, and if it is a boiler showing a lockout light, the relight steps in the manual come before anything else.")])]), py_tuple(vec![py_tuple(vec![py_str("the whole house"), py_str("whole house"), py_str("every room"), py_str("all of them"), py_str("all of the lights"), py_str("everywhere"), py_str("the lot went"), py_str("downstairs and upstairs")]), py_tuple(vec![py_str("Several lights going together is a circuit rather than a bulb: look in the consumer unit for the switch that is down and reset it once, and if it trips again leave it off and get an electrician - a fault that blows twice is one they need to find, not one to keep resetting."), py_str("Then it is the circuit, not the lamps: one reset in the consumer unit is fine, a second trip means something on that circuit is faulty and it stays off until an electrician has seen it.")])]), py_tuple(vec![py_tuple(vec![py_str("the router"), py_str("router"), py_str("wifi"), py_str("wi-fi"), py_str("hub")]), py_tuple(vec![py_str("A red or amber light on the router is about the connection, not the machine: check the provider's status page for an outage in your area first, then reseat the cable from the wall, then power it off for a minute. Red on its own usually means it cannot see the internet at all."), py_str("On a router, the light follows the line: the provider's outage page settles whether it is them, and a cable that has been kicked out settles whether it is you.")])]), py_tuple(vec![__data10(), g__DESCRIBED_GOES_QUIET_LINES()]), py_tuple(vec![py_tuple(vec![py_str("under the sink"), py_str("drain"), py_str("drains"), py_str("plughole"), py_str("plug hole"), py_str("trap"), py_str("toilet"), py_str("bath"), py_str("shower"), py_str("waste"), py_str("sewer"), py_str("sink")]), py_tuple(vec![py_str("A smell from a drain is nearly always the trap under the sink or the bath: it holds water as a seal, and when it dries out the sewer comes back up. Pour a jug of water down it every week and the smell goes; a slow drain wants a kettle of hot water and a spoon of soda crystals, not bleach."), py_str("That is the drain, not the room: the U-bend under the sink traps water to keep the sewer smell out, and a trap that has dried out lets it back. Run the taps in every sink, bath and shower once a week, and if it is still there the trap unscrews by hand with a bucket under it.")])]), py_tuple(vec![py_tuple(vec![py_str("burning"), py_str("burn"), py_str("smoke"), py_str("smoky"), py_str("plastic"), py_str("electric"), py_str("wiring"), py_str("wire"), py_str("scorch")]), py_tuple(vec![py_str("Burning plastic or hot wiring is the one to act on now: switch the appliance off at the wall, unplug it if you can reach the plug safely and don't use it again until somebody has looked at it. Smoke anywhere means out of the house and 999."), py_str("A burning smell with nothing on fire is usually a plug, a socket or a cable getting hot. Switch it off at the wall and leave it off; an electrician will find which one in minutes, and a landlord has to send one.")])]), py_tuple(vec![py_tuple(vec![py_str("damp"), py_str("musty"), py_str("musty"), py_str("mould"), py_str("mold"), py_str("wet"), py_str("dank"), py_str("cellar"), py_str("basement")]), py_tuple(vec![py_str("Damp that stays is a leak or a room with no air moving through it, and painting over it brings it back within a season. Find the wall it starts at, open a window across the room for ten minutes a day, and tell the landlord in writing - damp and mould are their job under the housing health and safety rating."), py_str("A musty smell is damp somewhere, often behind a unit or under a floor. Look for the tidemark - the highest line the damp has reached - because that is where the water is coming in from.")])]), py_tuple(vec![py_tuple(vec![py_str("dead"), py_str("mouse"), py_str("mice"), py_str("rat"), py_str("rats"), py_str("rodent"), py_str("rotting"), py_str("rotten"), py_str("decay")]), py_tuple(vec![py_str("A smell that arrives suddenly and gets worse for a week is usually a dead rodent behind a unit or under the floor. It fades on its own in two or three weeks; if it is in a wall or the floor, the landlord or a pest controller should open it up rather than you."), py_str("That one is usually a mouse or a rat that died where it could not be reached. Air the room, and if it is in a cavity or under boards, ask the landlord or a pest controller to get to it - the smell does go on its own, but slowly.")])]), py_tuple(vec![py_tuple(vec![py_str("loft"), py_str("roof"), py_str("attic"), py_str("ceiling"), py_str("chimney"), py_str("eaves"), py_str("gutter")]), py_tuple(vec![py_str("A noise in the loft or on the roof is usually an animal: birds and squirrels start at first light, mice and rats work at night. A bird on the roof at 5am in summer is doing what birds do; scratching that goes on after dark is the one to have looked at."), py_str("From the loft, the usual two are an animal (birds and squirrels at dawn, mice and rats after dark) and the pipework expanding when the heating comes on. A trap, or a landlord's pest contract, settles the first; clipping the pipes settles the second.")])]), py_tuple(vec![py_tuple(vec![py_str("radiator"), py_str("pipe"), py_str("pipes"), py_str("heating"), py_str("boiler"), py_str("hot water"), py_str("central heating"), py_str("clanking"), py_str("banging pipes"), py_str("knocking")]), py_tuple(vec![py_str("Banging pipes and radiators are usually air in the system or a pipe rubbing where it passes through a floor or a wall. Bleed the radiators with the heating off and cold, and if it is a single knock when the boiler fires, that is a valve shutting quickly."), py_str("That is usually air in the heating or a loose pipe moving as it warms. Bleed the radiators - a radiator key, heating off, until water not air comes out - and if it carries on, a heating engineer clips the run so it cannot knock.")])]), py_tuple(vec![py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("washing machine"), py_str("dishwasher"), py_str("tumble"), py_str("dryer"), py_str("oven"), py_str("fan"), py_str("extractor"), py_str("machine")]), py_tuple(vec![py_str("An appliance that hums, rattles or buzzes is usually not level, or has something touching it - a pipe, a cupboard, the floor. Push it an inch away from the wall, check the feet, and if it is still there, the noise is the pump or the fan and it wants an engineer."), py_str("Appliances mostly rattle because they are not level or because something is touching them when they spin. Move it clear of the wall and the cupboard, check the feet are flat on the floor, and if it stays, it is a bearing or a pump.")])]), py_tuple(vec![py_tuple(vec![py_str("door"), py_str("doors"), py_str("gate"), py_str("window"), py_str("windows"), py_str("draught"), py_str("draft"), py_str("latch"), py_str("cupboard")]), py_tuple(vec![py_str("A door or a window that knocks is usually a draught moving it, or a latch that has worked loose - hold each one still on a windy night and the one that stops is the one to fix. A cupboard door that does it is a hinge or a magnetic catch."), py_str("That is usually wind on a door, a gate or a window, or a latch that no longer holds. Hold each one still as it happens; the one that stops is the one to adjust, and a rubber wedge does until then.")])]), py_tuple(vec![py_tuple(vec![py_str("outside"), py_str("next door"), py_str("neighbour"), py_str("neighbors"), py_str("street"), py_str("garden"), py_str("road"), py_str("alley")]), py_tuple(vec![py_str("Coming from outside, it is usually something in the street rather than in your walls: a fan, a fridge unit on a shop, a door, traffic. Note the time and how long it lasts - if it is after 11pm on a regular night, the council's noise team is who acts on it."), py_str("From outside, the useful thing is a note of when it happens and how long it lasts, since that is what the council's noise team acts on. Record a minute of it on your phone if you can - a recording is what turns a complaint into a case.")])]), py_tuple(vec![py_tuple(vec![py_str("every morning"), py_str("at night"), py_str("at 3am"), py_str("at 5am"), py_str("5am"), py_str("3am"), py_str("night"), py_str("dawn"), py_str("morning"), py_str("when it rains"), py_str("when it is windy"), py_str("when the heating comes on")]), py_tuple(vec![py_str("The time it happens narrows it more than the sound does. Dawn is birds and the heating coming on; after dark is mice, a boiler, or a neighbour's appliance; when it rains it is a gutter or a downpipe; in wind it is a fence, a gate or a loose flashing."), py_str("The time is the clue: first light is birds and the heating, after dark is animals or an appliance on a timer, rain is a gutter or downpipe, and wind is a fence or a flashing. Say which of those it is and I can take it further.")])])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.DESCRIBED"); }
    s_DESCRIBED(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.DESCRIBED_ROOM"); }
    s_DESCRIBED_ROOM(py_tuple(vec![py_str("That points at the room rather than the sound - a knock in one place at one time is usually something fixed moving, and a sound that moves about is usually air or water. Tell me which room and how often, and I'll narrow it to the one thing."), py_str("Thanks - that helps. If it is in one spot and repeats, it is usually something fixed: a pipe, a board, a fitting. If it wanders round the house, it is air or water. Which is it?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.no_eyes.A_DESCRIPTION"); }
    s_A_DESCRIPTION(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|the noise|the sound|the smell|its|it's)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn a_recording_they_offer(args: &[Py]) -> Py {
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
    let mut length: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The honest line for a recording the person is offering the bot, or None.");
    line = mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    return Py::None;
    }
    length = mcall(&g_A_A_LENGTH(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_LISTEN_OFFER(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&length.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_RECORDING_WORD(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { length.clone() } }) {
    return geti(&g_CANNOT_SEE(), &py_str("voice"));
    }
    return Py::None;
    Py::None
}
pub fn a_contact_they_handed_over(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The honest line for an address or a number handed over, or None.");
    line = mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    return Py::None;
    }
    held = { let __b = mcall(&g_A_AN_EMAIL_ADDRESS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_A_PHONE_NUMBER(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_A_CONTACT_WORD(), "search", &[line.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_A_HANDOVER(), "search", &[line.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_ASKING_FOR_IT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_AN_EMAIL_ADDRESS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_A_PHONE_NUMBER(), "search", &[line.clone()]) } })) } }) {
    return Py::None;
    }
    return geti(&g_CANNOT_SEE(), &py_str("contact"));
    Py::None
}
pub fn handed_you_something(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let _ = py_str("The honest line for a thing the person believes they have just handed over, or None.\n\n    \"here's a screenshot\", \"i sent you a photo\", \"here is the link\", \"the link is www.example.com/\n    offer\" are not asks at all, so nothing in the bot answers them and the person gets \"Got it. Is\n    there anything you would like a hand with?\" - which reads as though the picture had arrived.\n    Nothing comes through an attachment and nothing can be opened, so the line has to say that and\n    ask for the words instead. A line that does hold the text (\"i pasted the text below\") is left\n    alone: that one did come through.\n    ");
    if py_truthy(&mcall(&g_HOLDS_THE_TEXT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\S+@\\S+\\.\\w+|\\baddress\\b|\\bnumber\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_URL(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+(?:do|can|should)\\s+i\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("link"));
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HANDED_OVER(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("voice"), py_str("photo"), py_str("link"), py_str("file"), py_str("inbox"), py_str("form")]));
    for __item in __iter1 {
    family = __item.clone();
    things = { let __b = mcall(&g_HANDED_FAMILIES(), "get", &[family.clone()]); if py_truthy(&__b) { __b } else { geti(&g_SEEING_FAMILIES(), &family.clone()) } };
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut thing: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g_CANNOT_SEE(), &family.clone());
    }
    }
    if py_truthy(&mcall(&g_ASKS_THE_BOT_TO_ACT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bsee\\s+attached\\b|\\battached\\s+(?:is|are)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g_CANNOT_SEE(), &py_str("file"));
    }
    return Py::None;
    Py::None
}
pub fn names_the_addressee(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line already says who the message is for.");
    return call_builtin("bool", &[mcall(&g_A_NAMED_ADDRESSEE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn writes_the_message(args: &[Py]) -> Py {
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
    let _ = py_str("True when the ask is for the words of a message to somebody: write it, not send it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_MESSAGE_WORD(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WRITES_A_MESSAGE(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_FORWARD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN_MESSAGE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_TO_A_PERSON(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_body_or_a_doctor(args: &[Py]) -> Py {
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
    let mut BODY_PARTS: Py = Py::None;
    let mut HEALTH_WORDS: Py = Py::None;
    let mut SYMPTOM_WORDS: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("True when the line names a body, a symptom or a doctor - a rash is not a file.\n\n    \"can you look at this rash\" and \"can you look at this thing on my leg\" were answered \"I can't\n    open files, sorry\", which takes the health card off the table; the pharmacist line is the one\n    that helps. The word tables are the fallback's own, read here so the two cannot drift.\n    ");
    BODY_PARTS = pyrt::xrefs::xget("compose.fallback", "BODY_PARTS");
    HEALTH_WORDS = pyrt::xrefs::xget("compose.fallback", "HEALTH_WORDS");
    SYMPTOM_WORDS = pyrt::xrefs::xget("compose.fallback", "SYMPTOM_WORDS");
    plain = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&plain.clone(), &py_str("back"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BACK_AS_A_BODY(), "search", &[clean.clone()]))) } }) {
    let _ = mcall(&plain.clone(), "discard", &[py_str("back")]);
    }
    return call_builtin("bool", &[{ let __b = op_bitand(&plain.clone(), &BODY_PARTS.clone()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&plain.clone(), &SYMPTOM_WORDS.clone()); if py_truthy(&__b) { __b } else { op_bitand(&plain.clone(), &HEALTH_WORDS.clone()) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_voice_is_named(args: &[Py]) -> Py {
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
    let mut thing: Py = Py::None;
    let _ = py_str("A recording, a memo, a podcast: something with words in it to be written down.\n\n    A bare \"can you hear that\" names no recording, so it is owed the smaller line - nothing here\n    hears it - rather than \"type the bit that matters\", which assumes a recording that the person\n    is not holding. #7519's first head sent \"can you hear that noise\" to the voice-note pool.\n    ");
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&g_SEEING_FAMILIES(), &py_str("voice"))) { let mut thing: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_game_to_play(args: &[Py]) -> Py {
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
    let _ = py_str("The line asks the bot to join in a game, or names a turn of one, so nothing is to be heard.");
    return call_builtin("bool", &[mcall(&g_GAME_TO_PLAY(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn cannot_see_your_thing(args: &[Py]) -> Py {
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
    let mut hit: Py = Py::None;
    let mut matched: Py = Py::None;
    let mut name: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let _ = py_str("The honest line for a thing of theirs the bot cannot open, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_THE_BOT_TO_ACT(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NO_HAND_VERB(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_HOLDS_THE_TEXT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ABOUT_WHAT_IT_KEEPS(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25))) {
    return Py::None;
    }
    if py_truthy(&_a_body_or_a_doctor(&[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&writes_the_message(&[clean.clone()])) {
    return Py::None;
    }
    matched = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_SEEING_FAMILIES(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    things = __unpk2[1].clone();
    hit = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut thing: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&hit.clone()) {
    let _ = mcall(&matched.clone(), "append", &[py_tuple(vec![call_builtin("len", &[mcall(&hit.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e)), name.clone()])]);
    }
    }
    if py_truthy(&matched.clone()) {
    return geti(&g_CANNOT_SEE(), &geti(&call_builtin("max", &[matched.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bfill(?:ing)?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:this|that)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("form"));
    }
    if py_truthy(&mcall(&g_A_HAND_DOES_IT(), "search", &[clean.clone()])) {
    return geti(&g_CANNOT_SEE(), &py_str("hands"));
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:this|that|these|those)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_AN_EAR_HEARS_IT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_game_to_play(&[clean.clone()]))) } } } }) {
    if py_truthy(&_a_voice_is_named(&[clean.clone()])) {
    return geti(&g_CANNOT_SEE(), &py_str("voice"));
    }
    return geti(&g_CANNOT_SEE(), &py_str("sound"));
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:this|that|these|those)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_AN_EYE_READS_IT(), "search", &[clean.clone()]) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("text"));
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bit\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_READS_HANDED_TEXT(), "search", &[clean.clone()]) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("text"));
    }
    return Py::None;
    Py::None
}
pub fn _asks_me_to_look_at_them(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut BODY_PARTS: Py = Py::None;
    let mut SYMPTOM_WORDS: Py = Py::None;
    let mut _thread_is_about_the_body: Py = Py::None;
    let mut body: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("A line asking the bot to look at their body, or a mark on it.\n\n    \"can you look at my knee\", \"can you tell me if this mole is ok\", \"does this look infected\" and\n    \"what does this rash look like\" all reached the search hedge, and a search engine cannot see\n    their leg either. The honest line is the one the health pool already gives: nothing here can\n    examine anything, and the person who can is a pharmacist, a GP or 111. A line about something\n    of theirs that is not their body (\"can you look at my cv\") names no body and no mark, so it is\n    left alone; a red flag never reaches this reader, because the safety pool answers it first.\n    ");
    BODY_PARTS = pyrt::xrefs::xget("compose.fallback", "BODY_PARTS");
    SYMPTOM_WORDS = pyrt::xrefs::xget("compose.fallback", "SYMPTOM_WORDS");
    _thread_is_about_the_body = py_func("_thread_is_about_the_body", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback", "_thread_is_about_the_body", args));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), { let __b = repaired.clone(); if py_truthy(&__b) { __b } else { clean.clone() } }]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LOOK_ASK(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&g_A_MARK_ON_THEM(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    body = op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &op_bitor(&op_bitor(&BODY_PARTS.clone(), &g_A_MORE_BODY()), &SYMPTOM_WORDS.clone()));
    if py_truthy(&body.clone()) {
    if py_truthy(&{ let __b = op_sub(&body.clone(), &py_set(vec![py_str("back")])); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GOING_BACK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_MY_BACK(), "search", &[line.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = mcall(&g_A_LOOK_ASK_IT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { py_call(&_thread_is_about_the_body, &[memory.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn look_at_a_body_lines(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The pool to say for a line asking the bot to look at the person's body, or None.");
    if py_truthy(&_asks_me_to_look_at_them(&[clean.clone(), repaired.clone(), memory.clone()])) {
    return g_CANNOT_SEE_YOU();
    }
    return Py::None;
    Py::None
}
pub fn a_thing_in_the_room(args: &[Py]) -> Py {
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
    let mut a_place_at_it: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The pool for a noise, a smell or a light in the room with them, or None.");
    line = mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = mcall(&g_A_POWER_CUT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_LIGHT_OUT_THERE(), "search", &[line.clone()]))) } }) {
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20))) {
    return geti(&g_CANNOT_SEE(), &py_str("power_cut"));
    }
    }
    if py_truthy(&{ let __b = mcall(&g_LIGHTS_ARE_OUT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_THE_STREET(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_LIGHT_OUT_THERE(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } } } } } }) {
    return geti(&g_CANNOT_SEE(), &py_str("power_cut"));
    }
    if py_truthy(&{ let __b = mcall(&g_LIGHTS_ARE_OUT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_LIGHT_OUT_THERE(), "search", &[line.clone()]))) } }) {
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_NAME_ASK(), "search", &[line.clone()]))) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("lights_out"));
    }
    }
    a_place_at_it = call_builtin("bool", &[{ let __b = { let __b = mcall(&g_A_NOISE_ABOVE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_NOISE_IN_THE_PIPES(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_NOISE_OUTSIDE(), "search", &[line.clone()]) } } } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_NOISE_VERB(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THAT_OPENS_A_SENTENCE(), "search", &[line.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_THING_IN_THE_ROOM(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_LIGHT_WORD(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_THING_NAMED(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_NOISE_NAMED(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_place_at_it.clone())) } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_SOUNDS_AGREEING(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NAME_ASK(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("^\\s*did\\s+you\\s+(?:hear|smell)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_THAT_OPENS_A_SENTENCE(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_SMELL_WORD(), "search", &[line.clone()])) {
    return g_CANNOT_SMELL();
    }
    if py_truthy(&mcall(&g_A_BEEP_WORD(), "search", &[line.clone()])) {
    return g_CANNOT_HEAR_A_BEEP();
    }
    if py_truthy(&{ let __b = mcall(&g_A_LIGHT_WORD(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_LIGHT_OUT_THERE(), "search", &[line.clone()]))) } }) {
    return geti(&g_CANNOT_SEE(), &py_str("light"));
    }
    if py_truthy(&mcall(&g_A_MACHINE_OR_A_NEIGHBOUR(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NOISE_ABOVE(), "search", &[line.clone()])) {
    return geti(&g_CANNOT_SEE(), &py_str("noise_roof"));
    }
    if py_truthy(&mcall(&g_A_NOISE_IN_THE_PIPES(), "search", &[line.clone()])) {
    return geti(&g_CANNOT_SEE(), &py_str("noise_pipes"));
    }
    if py_truthy(&mcall(&g_A_NOISE_OUTSIDE(), "search", &[line.clone()])) {
    return geti(&g_CANNOT_SEE(), &py_str("noise_outside"));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_A_THING_IN_THE_ROOM(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_WITNESSED(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    return geti(&g_CANNOT_SEE(), &py_str("sound"));
    Py::None
}
pub fn _is_a_placeholder(args: &[Py]) -> Py {
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
    let mut stem: Py = Py::None;
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("—"), py_str("-")]), "replace", &[py_str("–"), py_str("-")]), "split", &[])]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_PLACEHOLDERS()) { let mut stem: Py = __item.clone(); __out.push({ let __b = mcall(&said.clone(), "startswith", &[stem.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(60), &Py::None), &stem.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_light_in_the_room_over_a_placeholder(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut pool: Py = Py::None;
    let _ = py_str("The light pool for a light in the room that was answered with a placeholder, else None.");
    if py_truthy(&Py::Bool(!py_truthy(&_is_a_placeholder(&[reply.clone()])))) {
    return Py::None;
    }
    pool = a_thing_in_the_room(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pool.clone(), &geti(&g_CANNOT_SEE(), &py_str("light")))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&pool.clone(), &geti(&g_CANNOT_SEE(), &py_str("lights_out")))) } }) {
    return Py::None;
    }
    return pool.clone();
    Py::None
}
pub fn a_thing_in_the_room_over_a_placeholder(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("The room pool for a noise, a smell or a light answered with a placeholder, else None.\n\n    The light hook alone left the same hole for a sound and a smell: the statement form of the\n    line (\"there is a strange noise coming from the loft\", \"theres a beeping coming from the\n    hall\") is answered earlier, by the chain's own momentum pools, so the room reader in\n    `fallback_pools._unknown` never runs and the person gets \"Thanks for telling me - I'm here if\n    you want to dig into any of that.\" Anything the room reader has a pool for is worth more\n    than a placeholder, and only a placeholder is ever replaced.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_is_a_placeholder(&[reply.clone()])))) {
    return Py::None;
    }
    return a_thing_in_the_room(&[clean.clone()]);
    Py::None
}
pub fn what_they_described(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut before: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The other half of a room question: what the thing they named usually is, or None.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    before = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = { let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&said.clone()) {
    before = mcall(&said.clone(), "lower", &[]);
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ROOM_ASKS()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&before.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    line = mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&g_DESCRIBED());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    words = __unpk3[0].clone();
    pool = __unpk3[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&pool.clone(), &g__DESCRIBED_GOES_QUIET_LINES())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_BEEP_OR_AN_ALARM(), "search", &[before.clone()]))) } }) {
    return g_DESCRIBED_CYCLE();
    }
    return pool.clone();
    }
    }
    if py_truthy(&mcall(&g_A_DESCRIPTION(), "search", &[line.clone()])) {
    return g_DESCRIBED_ROOM();
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.no_eyes".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_WHAT_IT_KEEPS" => Some(g_ABOUT_WHAT_IT_KEEPS()),
            "AN_EAR_HEARS_IT" => Some(g_AN_EAR_HEARS_IT()),
            "AN_EYE_READS_IT" => Some(g_AN_EYE_READS_IT()),
            "ASKS_THE_BOT_TO_ACT" => Some(g_ASKS_THE_BOT_TO_ACT()),
            "A_AN_EMAIL_ADDRESS" => Some(g_A_AN_EMAIL_ADDRESS()),
            "A_ASKING_FOR_IT" => Some(g_A_ASKING_FOR_IT()),
            "A_A_CONTACT_WORD" => Some(g_A_A_CONTACT_WORD()),
            "A_A_HANDOVER" => Some(g_A_A_HANDOVER()),
            "A_A_LENGTH" => Some(g_A_A_LENGTH()),
            "A_A_PHONE_NUMBER" => Some(g_A_A_PHONE_NUMBER()),
            "A_BEEP_OR_AN_ALARM" => Some(g_A_BEEP_OR_AN_ALARM()),
            "A_BEEP_WORD" => Some(g_A_BEEP_WORD()),
            "A_DESCRIPTION" => Some(g_A_DESCRIPTION()),
            "A_FORWARD" => Some(g_A_FORWARD()),
            "A_HAND_DOES_IT" => Some(g_A_HAND_DOES_IT()),
            "A_LIGHT_OUT_THERE" => Some(g_A_LIGHT_OUT_THERE()),
            "A_LIGHT_WORD" => Some(g_A_LIGHT_WORD()),
            "A_LISTEN_OFFER" => Some(g_A_LISTEN_OFFER()),
            "A_LOOK_ASK" => Some(g_A_LOOK_ASK()),
            "A_LOOK_ASK_IT" => Some(g_A_LOOK_ASK_IT()),
            "A_MACHINE_OR_A_NEIGHBOUR" => Some(g_A_MACHINE_OR_A_NEIGHBOUR()),
            "A_MARK_ON_THEM" => Some(g_A_MARK_ON_THEM()),
            "A_MORE_BODY" => Some(g_A_MORE_BODY()),
            "A_NAMED_ADDRESSEE" => Some(g_A_NAMED_ADDRESSEE()),
            "A_NAME_ASK" => Some(g_A_NAME_ASK()),
            "A_NOISE_ABOVE" => Some(g_A_NOISE_ABOVE()),
            "A_NOISE_IN_THE_PIPES" => Some(g_A_NOISE_IN_THE_PIPES()),
            "A_NOISE_NAMED" => Some(g_A_NOISE_NAMED()),
            "A_NOISE_OUTSIDE" => Some(g_A_NOISE_OUTSIDE()),
            "A_NOISE_VERB" => Some(g_A_NOISE_VERB()),
            "A_POWER_CUT" => Some(g_A_POWER_CUT()),
            "A_RECORDING_WORD" => Some(g_A_RECORDING_WORD()),
            "A_SMELL_WORD" => Some(g_A_SMELL_WORD()),
            "A_SOUNDS_AGREEING" => Some(g_A_SOUNDS_AGREEING()),
            "A_THE_STREET" => Some(g_A_THE_STREET()),
            "A_THING_IN_THE_ROOM" => Some(g_A_THING_IN_THE_ROOM()),
            "A_THING_NAMED" => Some(g_A_THING_NAMED()),
            "A_URL" => Some(g_A_URL()),
            "A_WITNESSED" => Some(g_A_WITNESSED()),
            "BACK_AS_A_BODY" => Some(g_BACK_AS_A_BODY()),
            "CANNOT_HEAR_A_BEEP" => Some(g_CANNOT_HEAR_A_BEEP()),
            "CANNOT_SEE" => Some(g_CANNOT_SEE()),
            "CANNOT_SEE_YOU" => Some(g_CANNOT_SEE_YOU()),
            "CANNOT_SMELL" => Some(g_CANNOT_SMELL()),
            "DESCRIBED" => Some(g_DESCRIBED()),
            "DESCRIBED_CYCLE" => Some(g_DESCRIBED_CYCLE()),
            "DESCRIBED_ROOM" => Some(g_DESCRIBED_ROOM()),
            "GAME_TO_PLAY" => Some(g_GAME_TO_PLAY()),
            "GOING_BACK" => Some(g_GOING_BACK()),
            "HANDED_FAMILIES" => Some(g_HANDED_FAMILIES()),
            "HANDED_OVER" => Some(g_HANDED_OVER()),
            "HOLDS_THE_TEXT" => Some(g_HOLDS_THE_TEXT()),
            "LIGHTS_ARE_OUT" => Some(g_LIGHTS_ARE_OUT()),
            "MESSAGE_WORD" => Some(g_MESSAGE_WORD()),
            "MY_BACK" => Some(g_MY_BACK()),
            "NO_HAND_VERB" => Some(g_NO_HAND_VERB()),
            "READS_HANDED_TEXT" => Some(g_READS_HANDED_TEXT()),
            "SEEING_FAMILIES" => Some(g_SEEING_FAMILIES()),
            "THAT_OPENS_A_SENTENCE" => Some(g_THAT_OPENS_A_SENTENCE()),
            "THEIR_OWN_MESSAGE" => Some(g_THEIR_OWN_MESSAGE()),
            "THE_PLACEHOLDERS" => Some(g_THE_PLACEHOLDERS()),
            "TO_A_PERSON" => Some(g_TO_A_PERSON()),
            "WRITES_A_MESSAGE" => Some(g_WRITES_A_MESSAGE()),
            "_DESCRIBED_GOES_QUIET_LINES" => Some(g__DESCRIBED_GOES_QUIET_LINES()),
            "_NOISE_OR_SMELL" => Some(g__NOISE_OR_SMELL()),
            "_ROOM_ASKS" => Some(g__ROOM_ASKS()),
            "_a_body_or_a_doctor" => Some(py_func("_a_body_or_a_doctor", |args: &[Py]| _a_body_or_a_doctor(args))),
            "_a_game_to_play" => Some(py_func("_a_game_to_play", |args: &[Py]| _a_game_to_play(args))),
            "_a_voice_is_named" => Some(py_func("_a_voice_is_named", |args: &[Py]| _a_voice_is_named(args))),
            "_asks_me_to_look_at_them" => Some(py_func("_asks_me_to_look_at_them", |args: &[Py]| _asks_me_to_look_at_them(args))),
            "_is_a_placeholder" => Some(py_func("_is_a_placeholder", |args: &[Py]| _is_a_placeholder(args))),
            "a_contact_they_handed_over" => Some(py_func("a_contact_they_handed_over", |args: &[Py]| a_contact_they_handed_over(args))),
            "a_light_in_the_room_over_a_placeholder" => Some(py_func("a_light_in_the_room_over_a_placeholder", |args: &[Py]| a_light_in_the_room_over_a_placeholder(args))),
            "a_recording_they_offer" => Some(py_func("a_recording_they_offer", |args: &[Py]| a_recording_they_offer(args))),
            "a_thing_in_the_room" => Some(py_func("a_thing_in_the_room", |args: &[Py]| a_thing_in_the_room(args))),
            "a_thing_in_the_room_over_a_placeholder" => Some(py_func("a_thing_in_the_room_over_a_placeholder", |args: &[Py]| a_thing_in_the_room_over_a_placeholder(args))),
            "cannot_see_your_thing" => Some(py_func("cannot_see_your_thing", |args: &[Py]| cannot_see_your_thing(args))),
            "handed_you_something" => Some(py_func("handed_you_something", |args: &[Py]| handed_you_something(args))),
            "look_at_a_body_lines" => Some(py_func("look_at_a_body_lines", |args: &[Py]| look_at_a_body_lines(args))),
            "names_the_addressee" => Some(py_func("names_the_addressee", |args: &[Py]| names_the_addressee(args))),
            "what_they_described" => Some(py_func("what_they_described", |args: &[Py]| what_they_described(args))),
            "writes_the_message" => Some(py_func("writes_the_message", |args: &[Py]| writes_the_message(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_WHAT_IT_KEEPS" => s_ABOUT_WHAT_IT_KEEPS(v),
            "AN_EAR_HEARS_IT" => s_AN_EAR_HEARS_IT(v),
            "AN_EYE_READS_IT" => s_AN_EYE_READS_IT(v),
            "ASKS_THE_BOT_TO_ACT" => s_ASKS_THE_BOT_TO_ACT(v),
            "A_AN_EMAIL_ADDRESS" => s_A_AN_EMAIL_ADDRESS(v),
            "A_ASKING_FOR_IT" => s_A_ASKING_FOR_IT(v),
            "A_A_CONTACT_WORD" => s_A_A_CONTACT_WORD(v),
            "A_A_HANDOVER" => s_A_A_HANDOVER(v),
            "A_A_LENGTH" => s_A_A_LENGTH(v),
            "A_A_PHONE_NUMBER" => s_A_A_PHONE_NUMBER(v),
            "A_BEEP_OR_AN_ALARM" => s_A_BEEP_OR_AN_ALARM(v),
            "A_BEEP_WORD" => s_A_BEEP_WORD(v),
            "A_DESCRIPTION" => s_A_DESCRIPTION(v),
            "A_FORWARD" => s_A_FORWARD(v),
            "A_HAND_DOES_IT" => s_A_HAND_DOES_IT(v),
            "A_LIGHT_OUT_THERE" => s_A_LIGHT_OUT_THERE(v),
            "A_LIGHT_WORD" => s_A_LIGHT_WORD(v),
            "A_LISTEN_OFFER" => s_A_LISTEN_OFFER(v),
            "A_LOOK_ASK" => s_A_LOOK_ASK(v),
            "A_LOOK_ASK_IT" => s_A_LOOK_ASK_IT(v),
            "A_MACHINE_OR_A_NEIGHBOUR" => s_A_MACHINE_OR_A_NEIGHBOUR(v),
            "A_MARK_ON_THEM" => s_A_MARK_ON_THEM(v),
            "A_MORE_BODY" => s_A_MORE_BODY(v),
            "A_NAMED_ADDRESSEE" => s_A_NAMED_ADDRESSEE(v),
            "A_NAME_ASK" => s_A_NAME_ASK(v),
            "A_NOISE_ABOVE" => s_A_NOISE_ABOVE(v),
            "A_NOISE_IN_THE_PIPES" => s_A_NOISE_IN_THE_PIPES(v),
            "A_NOISE_NAMED" => s_A_NOISE_NAMED(v),
            "A_NOISE_OUTSIDE" => s_A_NOISE_OUTSIDE(v),
            "A_NOISE_VERB" => s_A_NOISE_VERB(v),
            "A_POWER_CUT" => s_A_POWER_CUT(v),
            "A_RECORDING_WORD" => s_A_RECORDING_WORD(v),
            "A_SMELL_WORD" => s_A_SMELL_WORD(v),
            "A_SOUNDS_AGREEING" => s_A_SOUNDS_AGREEING(v),
            "A_THE_STREET" => s_A_THE_STREET(v),
            "A_THING_IN_THE_ROOM" => s_A_THING_IN_THE_ROOM(v),
            "A_THING_NAMED" => s_A_THING_NAMED(v),
            "A_URL" => s_A_URL(v),
            "A_WITNESSED" => s_A_WITNESSED(v),
            "BACK_AS_A_BODY" => s_BACK_AS_A_BODY(v),
            "CANNOT_HEAR_A_BEEP" => s_CANNOT_HEAR_A_BEEP(v),
            "CANNOT_SEE" => s_CANNOT_SEE(v),
            "CANNOT_SEE_YOU" => s_CANNOT_SEE_YOU(v),
            "CANNOT_SMELL" => s_CANNOT_SMELL(v),
            "DESCRIBED" => s_DESCRIBED(v),
            "DESCRIBED_CYCLE" => s_DESCRIBED_CYCLE(v),
            "DESCRIBED_ROOM" => s_DESCRIBED_ROOM(v),
            "GAME_TO_PLAY" => s_GAME_TO_PLAY(v),
            "GOING_BACK" => s_GOING_BACK(v),
            "HANDED_FAMILIES" => s_HANDED_FAMILIES(v),
            "HANDED_OVER" => s_HANDED_OVER(v),
            "HOLDS_THE_TEXT" => s_HOLDS_THE_TEXT(v),
            "LIGHTS_ARE_OUT" => s_LIGHTS_ARE_OUT(v),
            "MESSAGE_WORD" => s_MESSAGE_WORD(v),
            "MY_BACK" => s_MY_BACK(v),
            "NO_HAND_VERB" => s_NO_HAND_VERB(v),
            "READS_HANDED_TEXT" => s_READS_HANDED_TEXT(v),
            "SEEING_FAMILIES" => s_SEEING_FAMILIES(v),
            "THAT_OPENS_A_SENTENCE" => s_THAT_OPENS_A_SENTENCE(v),
            "THEIR_OWN_MESSAGE" => s_THEIR_OWN_MESSAGE(v),
            "THE_PLACEHOLDERS" => s_THE_PLACEHOLDERS(v),
            "TO_A_PERSON" => s_TO_A_PERSON(v),
            "WRITES_A_MESSAGE" => s_WRITES_A_MESSAGE(v),
            "_DESCRIBED_GOES_QUIET_LINES" => s__DESCRIBED_GOES_QUIET_LINES(v),
            "_NOISE_OR_SMELL" => s__NOISE_OR_SMELL(v),
            "_ROOM_ASKS" => s__ROOM_ASKS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.no_eyes", module_obj);
    pyrt::xrefs::reg_fn("compose.no_eyes", "a_recording_they_offer", py_func("a_recording_they_offer", |args: &[Py]| a_recording_they_offer(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "a_contact_they_handed_over", py_func("a_contact_they_handed_over", |args: &[Py]| a_contact_they_handed_over(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "handed_you_something", py_func("handed_you_something", |args: &[Py]| handed_you_something(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "names_the_addressee", py_func("names_the_addressee", |args: &[Py]| names_the_addressee(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "writes_the_message", py_func("writes_the_message", |args: &[Py]| writes_the_message(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "_a_body_or_a_doctor", py_func("_a_body_or_a_doctor", |args: &[Py]| _a_body_or_a_doctor(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "_a_voice_is_named", py_func("_a_voice_is_named", |args: &[Py]| _a_voice_is_named(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "_a_game_to_play", py_func("_a_game_to_play", |args: &[Py]| _a_game_to_play(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "cannot_see_your_thing", py_func("cannot_see_your_thing", |args: &[Py]| cannot_see_your_thing(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "_asks_me_to_look_at_them", py_func("_asks_me_to_look_at_them", |args: &[Py]| _asks_me_to_look_at_them(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "look_at_a_body_lines", py_func("look_at_a_body_lines", |args: &[Py]| look_at_a_body_lines(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "a_thing_in_the_room", py_func("a_thing_in_the_room", |args: &[Py]| a_thing_in_the_room(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "_is_a_placeholder", py_func("_is_a_placeholder", |args: &[Py]| _is_a_placeholder(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "a_light_in_the_room_over_a_placeholder", py_func("a_light_in_the_room_over_a_placeholder", |args: &[Py]| a_light_in_the_room_over_a_placeholder(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "a_thing_in_the_room_over_a_placeholder", py_func("a_thing_in_the_room_over_a_placeholder", |args: &[Py]| a_thing_in_the_room_over_a_placeholder(args)));
    pyrt::xrefs::reg_fn("compose.no_eyes", "what_they_described", py_func("what_they_described", |args: &[Py]| what_they_described(args)));
    pyrt::xrefs::reg_access("compose.no_eyes", "ABOUT_WHAT_IT_KEEPS", g_ABOUT_WHAT_IT_KEEPS, s_ABOUT_WHAT_IT_KEEPS);
    pyrt::xrefs::reg_access("compose.no_eyes", "AN_EAR_HEARS_IT", g_AN_EAR_HEARS_IT, s_AN_EAR_HEARS_IT);
    pyrt::xrefs::reg_access("compose.no_eyes", "AN_EYE_READS_IT", g_AN_EYE_READS_IT, s_AN_EYE_READS_IT);
    pyrt::xrefs::reg_access("compose.no_eyes", "ASKS_THE_BOT_TO_ACT", g_ASKS_THE_BOT_TO_ACT, s_ASKS_THE_BOT_TO_ACT);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_AN_EMAIL_ADDRESS", g_A_AN_EMAIL_ADDRESS, s_A_AN_EMAIL_ADDRESS);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_ASKING_FOR_IT", g_A_ASKING_FOR_IT, s_A_ASKING_FOR_IT);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_A_CONTACT_WORD", g_A_A_CONTACT_WORD, s_A_A_CONTACT_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_A_HANDOVER", g_A_A_HANDOVER, s_A_A_HANDOVER);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_A_LENGTH", g_A_A_LENGTH, s_A_A_LENGTH);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_A_PHONE_NUMBER", g_A_A_PHONE_NUMBER, s_A_A_PHONE_NUMBER);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_BEEP_OR_AN_ALARM", g_A_BEEP_OR_AN_ALARM, s_A_BEEP_OR_AN_ALARM);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_BEEP_WORD", g_A_BEEP_WORD, s_A_BEEP_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_DESCRIPTION", g_A_DESCRIPTION, s_A_DESCRIPTION);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_FORWARD", g_A_FORWARD, s_A_FORWARD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_HAND_DOES_IT", g_A_HAND_DOES_IT, s_A_HAND_DOES_IT);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_LIGHT_OUT_THERE", g_A_LIGHT_OUT_THERE, s_A_LIGHT_OUT_THERE);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_LIGHT_WORD", g_A_LIGHT_WORD, s_A_LIGHT_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_LISTEN_OFFER", g_A_LISTEN_OFFER, s_A_LISTEN_OFFER);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_LOOK_ASK", g_A_LOOK_ASK, s_A_LOOK_ASK);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_LOOK_ASK_IT", g_A_LOOK_ASK_IT, s_A_LOOK_ASK_IT);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_MACHINE_OR_A_NEIGHBOUR", g_A_MACHINE_OR_A_NEIGHBOUR, s_A_MACHINE_OR_A_NEIGHBOUR);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_MARK_ON_THEM", g_A_MARK_ON_THEM, s_A_MARK_ON_THEM);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_MORE_BODY", g_A_MORE_BODY, s_A_MORE_BODY);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NAMED_ADDRESSEE", g_A_NAMED_ADDRESSEE, s_A_NAMED_ADDRESSEE);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NAME_ASK", g_A_NAME_ASK, s_A_NAME_ASK);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NOISE_ABOVE", g_A_NOISE_ABOVE, s_A_NOISE_ABOVE);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NOISE_IN_THE_PIPES", g_A_NOISE_IN_THE_PIPES, s_A_NOISE_IN_THE_PIPES);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NOISE_NAMED", g_A_NOISE_NAMED, s_A_NOISE_NAMED);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NOISE_OUTSIDE", g_A_NOISE_OUTSIDE, s_A_NOISE_OUTSIDE);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_NOISE_VERB", g_A_NOISE_VERB, s_A_NOISE_VERB);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_POWER_CUT", g_A_POWER_CUT, s_A_POWER_CUT);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_RECORDING_WORD", g_A_RECORDING_WORD, s_A_RECORDING_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_SMELL_WORD", g_A_SMELL_WORD, s_A_SMELL_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_SOUNDS_AGREEING", g_A_SOUNDS_AGREEING, s_A_SOUNDS_AGREEING);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_THE_STREET", g_A_THE_STREET, s_A_THE_STREET);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_THING_IN_THE_ROOM", g_A_THING_IN_THE_ROOM, s_A_THING_IN_THE_ROOM);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_THING_NAMED", g_A_THING_NAMED, s_A_THING_NAMED);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_URL", g_A_URL, s_A_URL);
    pyrt::xrefs::reg_access("compose.no_eyes", "A_WITNESSED", g_A_WITNESSED, s_A_WITNESSED);
    pyrt::xrefs::reg_access("compose.no_eyes", "BACK_AS_A_BODY", g_BACK_AS_A_BODY, s_BACK_AS_A_BODY);
    pyrt::xrefs::reg_access("compose.no_eyes", "CANNOT_HEAR_A_BEEP", g_CANNOT_HEAR_A_BEEP, s_CANNOT_HEAR_A_BEEP);
    pyrt::xrefs::reg_access("compose.no_eyes", "CANNOT_SEE", g_CANNOT_SEE, s_CANNOT_SEE);
    pyrt::xrefs::reg_access("compose.no_eyes", "CANNOT_SEE_YOU", g_CANNOT_SEE_YOU, s_CANNOT_SEE_YOU);
    pyrt::xrefs::reg_access("compose.no_eyes", "CANNOT_SMELL", g_CANNOT_SMELL, s_CANNOT_SMELL);
    pyrt::xrefs::reg_access("compose.no_eyes", "DESCRIBED", g_DESCRIBED, s_DESCRIBED);
    pyrt::xrefs::reg_access("compose.no_eyes", "DESCRIBED_CYCLE", g_DESCRIBED_CYCLE, s_DESCRIBED_CYCLE);
    pyrt::xrefs::reg_access("compose.no_eyes", "DESCRIBED_ROOM", g_DESCRIBED_ROOM, s_DESCRIBED_ROOM);
    pyrt::xrefs::reg_access("compose.no_eyes", "GAME_TO_PLAY", g_GAME_TO_PLAY, s_GAME_TO_PLAY);
    pyrt::xrefs::reg_access("compose.no_eyes", "GOING_BACK", g_GOING_BACK, s_GOING_BACK);
    pyrt::xrefs::reg_access("compose.no_eyes", "HANDED_FAMILIES", g_HANDED_FAMILIES, s_HANDED_FAMILIES);
    pyrt::xrefs::reg_access("compose.no_eyes", "HANDED_OVER", g_HANDED_OVER, s_HANDED_OVER);
    pyrt::xrefs::reg_access("compose.no_eyes", "HOLDS_THE_TEXT", g_HOLDS_THE_TEXT, s_HOLDS_THE_TEXT);
    pyrt::xrefs::reg_access("compose.no_eyes", "LIGHTS_ARE_OUT", g_LIGHTS_ARE_OUT, s_LIGHTS_ARE_OUT);
    pyrt::xrefs::reg_access("compose.no_eyes", "MESSAGE_WORD", g_MESSAGE_WORD, s_MESSAGE_WORD);
    pyrt::xrefs::reg_access("compose.no_eyes", "MY_BACK", g_MY_BACK, s_MY_BACK);
    pyrt::xrefs::reg_access("compose.no_eyes", "NO_HAND_VERB", g_NO_HAND_VERB, s_NO_HAND_VERB);
    pyrt::xrefs::reg_access("compose.no_eyes", "READS_HANDED_TEXT", g_READS_HANDED_TEXT, s_READS_HANDED_TEXT);
    pyrt::xrefs::reg_access("compose.no_eyes", "SEEING_FAMILIES", g_SEEING_FAMILIES, s_SEEING_FAMILIES);
    pyrt::xrefs::reg_access("compose.no_eyes", "THAT_OPENS_A_SENTENCE", g_THAT_OPENS_A_SENTENCE, s_THAT_OPENS_A_SENTENCE);
    pyrt::xrefs::reg_access("compose.no_eyes", "THEIR_OWN_MESSAGE", g_THEIR_OWN_MESSAGE, s_THEIR_OWN_MESSAGE);
    pyrt::xrefs::reg_access("compose.no_eyes", "THE_PLACEHOLDERS", g_THE_PLACEHOLDERS, s_THE_PLACEHOLDERS);
    pyrt::xrefs::reg_access("compose.no_eyes", "TO_A_PERSON", g_TO_A_PERSON, s_TO_A_PERSON);
    pyrt::xrefs::reg_access("compose.no_eyes", "WRITES_A_MESSAGE", g_WRITES_A_MESSAGE, s_WRITES_A_MESSAGE);
    pyrt::xrefs::reg_access("compose.no_eyes", "_DESCRIBED_GOES_QUIET_LINES", g__DESCRIBED_GOES_QUIET_LINES, s__DESCRIBED_GOES_QUIET_LINES);
    pyrt::xrefs::reg_access("compose.no_eyes", "_NOISE_OR_SMELL", g__NOISE_OR_SMELL, s__NOISE_OR_SMELL);
    pyrt::xrefs::reg_access("compose.no_eyes", "_ROOM_ASKS", g__ROOM_ASKS, s__ROOM_ASKS);
}
